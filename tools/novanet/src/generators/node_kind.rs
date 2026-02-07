//! Generate Kind meta-nodes + schema_hint, context_budget + facet relations.
//!
//! Reads all 44 YAML node definitions and produces idempotent MERGE statements
//! for Kind nodes with auto-computed properties, plus hierarchy and facet rels.
//!
//! Output target: `packages/db/seed/01-kinds.cypher`

use super::cypher_utils::{cypher_list_ref, cypher_str};
use crate::parsers::yaml_node::{self, LocaleBehavior, ParsedNode};
use std::fmt::Write;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Build schema_hint from a ParsedNode's properties.
///
/// Format: `"key, display_name, instructions (req), node_trait"`
/// Required properties get the "(req)" suffix.
fn build_schema_hint(node: &ParsedNode) -> String {
    let mut parts: Vec<String> = Vec::new();

    // Collect standard properties
    if let Some(ref sp) = node.def.standard_properties {
        for (name, def) in sp {
            if def.required == Some(true) {
                parts.push(format!("{name} (req)"));
            } else {
                parts.push(name.clone());
            }
        }
    }

    // Collect business properties
    if let Some(ref p) = node.def.properties {
        for (name, def) in p {
            if def.required == Some(true) {
                parts.push(format!("{name} (req)"));
            } else {
                parts.push(name.clone());
            }
        }
    }

    parts.sort();
    parts.join(", ")
}

/// Determine context_budget based on node_trait trait and layer.
///
/// Rules:
/// - `job` trait → `minimal`
/// - `derived` trait → `low`
/// - `knowledge` trait → `medium`
/// - `instruction` or `config` layer → `medium`
/// - Everything else → `high`
fn context_budget(node: &ParsedNode) -> &'static str {
    match node.def.node_trait {
        LocaleBehavior::Job => "minimal",
        LocaleBehavior::Derived => "low",
        LocaleBehavior::Knowledge => "medium",
        LocaleBehavior::Invariant | LocaleBehavior::Localized => match node.layer.as_str() {
            "instruction" | "config" => "medium",
            _ => "high",
        },
    }
}

/// Compute the YAML path relative to models directory.
fn yaml_path(node: &ParsedNode) -> String {
    let filename = node
        .source_path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    format!("node-kinds/{}/{}/{}", node.realm, node.layer, filename)
}

/// Collect all property names (standard + business) in YAML definition order.
fn all_properties(node: &ParsedNode) -> Vec<&str> {
    node.all_property_names().to_vec()
}

/// Collect required property names in YAML definition order.
fn required_properties(node: &ParsedNode) -> Vec<&str> {
    let mut names: Vec<&str> = Vec::new();
    if let Some(ref sp) = node.def.standard_properties {
        for (name, def) in sp {
            if def.required == Some(true) {
                names.push(name.as_str());
            }
        }
    }
    if let Some(ref p) = node.def.properties {
        for (name, def) in p {
            if def.required == Some(true) {
                names.push(name.as_str());
            }
        }
    }
    // No sorting — preserve YAML definition order
    names
}

/// Convert PascalCase to kebab-case.
///
/// Examples:
/// - `"LocaleVoice"` → `"locale-voice"`
/// - `"SEOKeyword"` → `"seo-keyword"`
/// - `"EntityL10n"` → `"entity-l10n"`
fn to_kebab_case(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::with_capacity(s.len() + 4);

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                let prev = chars[i - 1];
                let next = chars.get(i + 1);

                // Add hyphen if:
                // 1. Previous was lowercase (normal transition: "Voice" in "LocaleVoice")
                // 2. Previous was uppercase AND next is lowercase (acronym boundary: "K" in "SEOKeyword")
                let prev_was_lower = prev.is_lowercase();
                let at_acronym_boundary =
                    prev.is_uppercase() && next.is_some_and(|n| n.is_lowercase());

                if prev_was_lower || at_acronym_boundary {
                    result.push('-');
                }
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

// ─────────────────────────────────────────────────────────────────────────────
// Generator
// ─────────────────────────────────────────────────────────────────────────────

pub struct NodeKindGenerator;

impl super::Generator for NodeKindGenerator {
    fn name(&self) -> &'static str {
        "kinds"
    }

    fn generate(&self, root: &Path) -> crate::Result<String> {
        let nodes = yaml_node::load_all_nodes(root)?;
        generate_kind_cypher(&nodes)
    }
}

fn generate_kind_cypher(nodes: &[ParsedNode]) -> crate::Result<String> {
    let mut out = String::with_capacity(32 * 1024);

    // Header
    writeln!(
        out,
        "// ═══════════════════════════════════════════════════════════════════════════════"
    )
    .unwrap();
    writeln!(out, "// Kind Meta-Nodes — v10.6.0").unwrap();
    writeln!(out, "// AUTO-GENERATED by novanet — DO NOT EDIT").unwrap();
    writeln!(
        out,
        "// {count} Kind nodes + hierarchy + facet relations",
        count = nodes.len()
    )
    .unwrap();
    writeln!(
        out,
        "// ═══════════════════════════════════════════════════════════════════════════════"
    )
    .unwrap();
    writeln!(out).unwrap();

    // ── Section 1: Kind nodes ────────────────────────────────────────────────
    write_section_header(&mut out, "Kind Nodes", nodes.len());
    writeln!(out).unwrap();

    for node in nodes {
        let label = &node.def.name;
        let var = format!("k_{label}");
        let display = cypher_str(&node.def.name);
        let llm = cypher_str(&node.def.description);
        let ypath = cypher_str(&yaml_path(node));
        let props = all_properties(node);
        let req_props = required_properties(node);
        let hint = cypher_str(&build_schema_hint(node));
        let budget = context_budget(node);

        let key = to_kebab_case(label);
        let realm = &node.realm;
        let layer = &node.layer;
        let trait_key = &node.def.node_trait;

        writeln!(out, "MERGE ({var}:Meta:Kind {{label: '{label}'}})").unwrap();
        writeln!(out, "ON CREATE SET").unwrap();
        writeln!(out, "  {var}.key = '{key}',").unwrap();
        writeln!(out, "  {var}.realm = '{realm}',").unwrap();
        writeln!(out, "  {var}.layer = '{layer}',").unwrap();
        writeln!(out, "  {var}.trait = '{trait_key}',").unwrap();
        writeln!(out, "  {var}.display_name = '{display}',").unwrap();
        writeln!(out, "  {var}.llm_context = '{llm}',").unwrap();
        writeln!(out, "  {var}.yaml_path = '{ypath}',").unwrap();
        writeln!(
            out,
            "  {var}.properties = {props},",
            props = cypher_list_ref(&props)
        )
        .unwrap();
        writeln!(
            out,
            "  {var}.required_properties = {req_props},",
            req_props = cypher_list_ref(&req_props)
        )
        .unwrap();
        writeln!(out, "  {var}.schema_hint = '{hint}',").unwrap();
        writeln!(out, "  {var}.context_budget = '{budget}',").unwrap();
        // v10: knowledge_tier for knowledge trait nodes only
        if let Some(tier) = &node.def.knowledge_tier {
            writeln!(out, "  {var}.knowledge_tier = '{tier}',").unwrap();
        }
        writeln!(out, "  {var}.generation_count = 0,").unwrap();
        writeln!(out, "  {var}.created_at = datetime()").unwrap();
        writeln!(out, "ON MATCH SET").unwrap();
        writeln!(out, "  {var}.key = '{key}',").unwrap();
        writeln!(out, "  {var}.realm = '{realm}',").unwrap();
        writeln!(out, "  {var}.layer = '{layer}',").unwrap();
        writeln!(out, "  {var}.trait = '{trait_key}',").unwrap();
        writeln!(out, "  {var}.display_name = '{display}',").unwrap();
        writeln!(out, "  {var}.llm_context = '{llm}',").unwrap();
        writeln!(out, "  {var}.yaml_path = '{ypath}',").unwrap();
        writeln!(
            out,
            "  {var}.properties = {props},",
            props = cypher_list_ref(&props)
        )
        .unwrap();
        writeln!(
            out,
            "  {var}.required_properties = {req_props},",
            req_props = cypher_list_ref(&req_props)
        )
        .unwrap();
        writeln!(out, "  {var}.schema_hint = '{hint}',").unwrap();
        writeln!(out, "  {var}.context_budget = '{budget}',").unwrap();
        // v10: knowledge_tier for knowledge trait nodes only
        if let Some(tier) = &node.def.knowledge_tier {
            writeln!(out, "  {var}.knowledge_tier = '{tier}',").unwrap();
        }
        writeln!(out, "  {var}.generation_count = 0,").unwrap();
        writeln!(out, "  {var}.updated_at = datetime();").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 2: HAS_KIND (Layer → Kind) ──────────────────────────────────
    write_section_header(
        &mut out,
        "Hierarchy: Layer -[:HAS_KIND]-> Kind",
        nodes.len(),
    );
    writeln!(out).unwrap();

    for node in nodes {
        writeln!(
            out,
            "MATCH (l:Layer {{key: '{layer}'}}), (k:Kind {{label: '{label}'}})",
            layer = node.layer,
            label = node.def.name
        )
        .unwrap();
        writeln!(out, "MERGE (l)-[:HAS_KIND]->(k);").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 3: IN_REALM (Kind → Realm) ──────────────────────────────────
    write_section_header(&mut out, "Facet: Kind -[:IN_REALM]-> Realm", nodes.len());
    writeln!(out).unwrap();

    for node in nodes {
        writeln!(
            out,
            "MATCH (k:Kind {{label: '{label}'}}), (r:Realm {{key: '{realm}'}})",
            label = node.def.name,
            realm = node.realm
        )
        .unwrap();
        writeln!(out, "MERGE (k)-[:IN_REALM]->(r);").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 4: IN_LAYER (Kind → Layer) ──────────────────────────────────
    write_section_header(&mut out, "Facet: Kind -[:IN_LAYER]-> Layer", nodes.len());
    writeln!(out).unwrap();

    for node in nodes {
        writeln!(
            out,
            "MATCH (k:Kind {{label: '{label}'}}), (l:Layer {{key: '{layer}'}})",
            label = node.def.name,
            layer = node.layer
        )
        .unwrap();
        writeln!(out, "MERGE (k)-[:IN_LAYER]->(l);").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 5: EXHIBITS (Kind → Trait) ──────────────────────────────────
    write_section_header(&mut out, "Facet: Kind -[:EXHIBITS]-> Trait", nodes.len());
    writeln!(out).unwrap();

    for node in nodes {
        writeln!(
            out,
            "MATCH (k:Kind {{label: '{label}'}}), (t:Trait {{key: '{trait_key}'}})",
            label = node.def.name,
            trait_key = node.def.node_trait
        )
        .unwrap();
        writeln!(out, "MERGE (k)-[:EXHIBITS]->(t);").unwrap();
        writeln!(out).unwrap();
    }

    Ok(out)
}

/// Write a visual section header comment.
fn write_section_header(out: &mut String, title: &str, count: usize) {
    let bar = "// ═══════════════════════════════════════════════════════════════════════════════";
    writeln!(out, "{bar}").unwrap();
    writeln!(out, "// {title} ({count})").unwrap();
    writeln!(out, "{bar}").unwrap();
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::Generator;
    use crate::parsers::yaml_node::{NodeDef, PropertyDef};
    use indexmap::IndexMap;
    use std::collections::BTreeMap;

    /// Build a minimal ParsedNode for testing.
    fn make_node(name: &str, realm: &str, layer: &str, behavior: LocaleBehavior) -> ParsedNode {
        ParsedNode {
            def: NodeDef {
                name: name.to_string(),
                realm: realm.to_string(),
                layer: layer.to_string(),
                node_trait: behavior,
                knowledge_tier: None,
                icon: None,
                description: format!("{name} description."),
                standard_properties: None,
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: realm.to_string(),
            layer: layer.to_string(),
            source_path: std::path::PathBuf::from(format!(
                "models/node-kinds/{realm}/{layer}/{}.yaml",
                name.to_lowercase()
            )),
        }
    }

    fn make_node_with_props(
        name: &str,
        realm: &str,
        layer: &str,
        behavior: LocaleBehavior,
        props: Vec<(&str, &str, bool)>, // (name, type, required)
    ) -> ParsedNode {
        // Use IndexMap to preserve YAML definition order
        let mut properties = IndexMap::new();
        for (pname, ptype, req) in props {
            properties.insert(
                pname.to_string(),
                PropertyDef {
                    prop_type: ptype.to_string(),
                    required: Some(req),
                    description: None,
                    extra: BTreeMap::new(),
                },
            );
        }

        let mut node = make_node(name, realm, layer, behavior);
        node.def.properties = Some(properties);
        node
    }

    #[test]
    fn schema_hint_no_properties() {
        let node = make_node("Test", "project", "foundation", LocaleBehavior::Invariant);
        assert_eq!(build_schema_hint(&node), "");
    }

    #[test]
    fn schema_hint_with_properties() {
        let node = make_node_with_props(
            "Page",
            "project",
            "structure",
            LocaleBehavior::Invariant,
            vec![
                ("key", "string", true),
                ("display_name", "string", true),
                ("instructions", "string", false),
            ],
        );
        let hint = build_schema_hint(&node);
        assert_eq!(hint, "display_name (req), instructions, key (req)");
    }

    #[test]
    fn context_budget_by_trait() {
        // v10.4: SEO layer is in global realm
        assert_eq!(
            context_budget(&make_node("Run", "global", "seo", LocaleBehavior::Job)),
            "minimal"
        );
        assert_eq!(
            context_budget(&make_node(
                "Metrics",
                "global",
                "seo",
                LocaleBehavior::Derived
            )),
            "low"
        );
        assert_eq!(
            context_budget(&make_node(
                "Voice",
                "global",
                "knowledge",
                LocaleBehavior::Knowledge
            )),
            "medium"
        );
    }

    #[test]
    fn context_budget_by_layer() {
        // instruction layer → medium (even with invariant trait)
        assert_eq!(
            context_budget(&make_node(
                "PageType",
                "project",
                "instruction",
                LocaleBehavior::Invariant
            )),
            "medium"
        );
        // config layer → medium
        assert_eq!(
            context_budget(&make_node(
                "Locale",
                "global",
                "config",
                LocaleBehavior::Invariant
            )),
            "medium"
        );
        // structure layer + invariant → high
        assert_eq!(
            context_budget(&make_node(
                "Page",
                "project",
                "structure",
                LocaleBehavior::Invariant
            )),
            "high"
        );
        // output layer + localized → high
        assert_eq!(
            context_budget(&make_node(
                "PageL10n",
                "project",
                "output",
                LocaleBehavior::Localized
            )),
            "high"
        );
    }

    #[test]
    fn to_kebab_case_simple() {
        assert_eq!(to_kebab_case("Page"), "page");
        assert_eq!(to_kebab_case("Project"), "project");
    }

    #[test]
    fn to_kebab_case_pascal() {
        assert_eq!(to_kebab_case("LocaleVoice"), "locale-voice");
        assert_eq!(to_kebab_case("BlockType"), "block-type");
        assert_eq!(to_kebab_case("GenerationJob"), "generation-job");
    }

    #[test]
    fn to_kebab_case_acronyms() {
        assert_eq!(to_kebab_case("SEOKeyword"), "seo-keyword");
        assert_eq!(to_kebab_case("SEOMiningRun"), "seo-mining-run");
    }

    #[test]
    fn to_kebab_case_l10n() {
        assert_eq!(to_kebab_case("EntityL10n"), "entity-l10n");
        assert_eq!(to_kebab_case("PageL10n"), "page-l10n");
        assert_eq!(to_kebab_case("BlockL10n"), "block-l10n");
    }

    #[test]
    fn generate_small_kind_cypher() {
        let nodes = vec![
            make_node_with_props(
                "Page",
                "project",
                "structure",
                LocaleBehavior::Invariant,
                vec![("key", "string", true), ("display_name", "string", true)],
            ),
            make_node("Locale", "global", "config", LocaleBehavior::Invariant),
        ];

        let cypher = generate_kind_cypher(&nodes).unwrap();

        // Header
        assert!(cypher.contains("v10.6.0"));
        assert!(cypher.contains("AUTO-GENERATED"));
        assert!(cypher.contains("2 Kind nodes"));

        // Kind node for Page
        assert!(cypher.contains("MERGE (k_Page:Meta:Kind {label: 'Page'})"));
        assert!(cypher.contains("k_Page.display_name = 'Page'"));
        assert!(cypher.contains("k_Page.context_budget = 'high'"));
        assert!(cypher.contains("k_Page.generation_count = 0"));
        // Properties now preserve YAML definition order (key first, then display_name)
        assert!(cypher.contains("k_Page.schema_hint = 'display_name (req), key (req)'"));
        assert!(cypher.contains("k_Page.properties = ['key', 'display_name']"));
        assert!(cypher.contains("k_Page.required_properties = ['key', 'display_name']"));
        assert!(cypher.contains("k_Page.yaml_path = 'node-kinds/project/structure/page.yaml'"));

        // Kind node for Locale
        assert!(cypher.contains("MERGE (k_Locale:Meta:Kind {label: 'Locale'})"));
        assert!(cypher.contains("k_Locale.context_budget = 'medium'"));

        // HAS_KIND hierarchy
        assert!(cypher.contains("(l:Layer {key: 'structure'}), (k:Kind {label: 'Page'})"));
        assert!(cypher.contains("MERGE (l)-[:HAS_KIND]->(k)"));

        // IN_REALM facet
        assert!(cypher.contains("(k:Kind {label: 'Page'}), (r:Realm {key: 'project'})"));
        assert!(cypher.contains("MERGE (k)-[:IN_REALM]->(r)"));

        // IN_LAYER facet
        assert!(cypher.contains("(k:Kind {label: 'Page'}), (l:Layer {key: 'structure'})"));
        assert!(cypher.contains("MERGE (k)-[:IN_LAYER]->(l)"));

        // EXHIBITS facet
        assert!(cypher.contains("(k:Kind {label: 'Page'}), (t:Trait {key: 'invariant'})"));
        assert!(cypher.contains("MERGE (k)-[:EXHIBITS]->(t)"));
        assert!(cypher.contains("(k:Kind {label: 'Locale'}), (t:Trait {key: 'invariant'})"));

        // Section headers
        assert!(cypher.contains("Hierarchy: Layer -[:HAS_KIND]-> Kind"));
        assert!(cypher.contains("Facet: Kind -[:IN_REALM]-> Realm"));
        assert!(cypher.contains("Facet: Kind -[:IN_LAYER]-> Layer"));
        assert!(cypher.contains("Facet: Kind -[:EXHIBITS]-> Trait"));

        // Timestamps
        assert!(cypher.contains("created_at = datetime()"));
        assert!(cypher.contains("updated_at = datetime()"));
    }

    #[test]
    fn generate_kind_integration() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let generator = NodeKindGenerator;
        let cypher = generator
            .generate(root)
            .expect("should generate kind cypher");

        // v10.6: 48 Kind MERGE statements (2 realms: global + tenant)
        let kind_merges = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains(":Meta:Kind"))
            .count();
        assert_eq!(
            kind_merges, 60,
            "expected 60 Kind MERGE statements (v10.8: 2 realms, 37+23 nodes)"
        );

        // 60 HAS_KIND relationships
        let has_kind = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:HAS_KIND]"))
            .count();
        assert_eq!(has_kind, 60, "expected 60 HAS_KIND relationships");

        // 60 IN_REALM relationships
        let in_realm = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:IN_REALM]"))
            .count();
        assert_eq!(in_realm, 60, "expected 60 IN_REALM relationships");

        // 60 IN_LAYER relationships
        let in_layer = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:IN_LAYER]"))
            .count();
        assert_eq!(in_layer, 60, "expected 60 IN_LAYER relationships");

        // 60 EXHIBITS relationships
        let exhibits = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:EXHIBITS]"))
            .count();
        assert_eq!(exhibits, 60, "expected 60 EXHIBITS relationships");

        // Spot checks — specific Kinds
        assert!(cypher.contains("k_Project:Meta:Kind {label: 'Project'}"));
        assert!(cypher.contains("k_Page:Meta:Kind {label: 'Page'}"));
        assert!(cypher.contains("k_Style:Meta:Kind {label: 'Style'}")); // v10: LocaleVoice → Style
        assert!(cypher.contains("k_SEOKeyword:Meta:Kind {label: 'SEOKeyword'}"));

        // Spot check — context_budget assignments
        assert!(cypher.contains("k_Page.context_budget = 'high'"));
        assert!(cypher.contains("k_BlockType.context_budget = 'medium'"));
        assert!(cypher.contains("k_Style.context_budget = 'medium'")); // v10: LocaleVoice → Style
        assert!(cypher.contains("k_Locale.context_budget = 'medium'"));

        // Spot check — facet wiring (v10.6: project → tenant)
        assert!(cypher.contains("(k:Kind {label: 'Page'}), (r:Realm {key: 'tenant'})"));
        assert!(cypher.contains("(k:Kind {label: 'Style'}), (t:Trait {key: 'knowledge'})")); // v10: LocaleVoice → Style
        assert!(cypher.contains("(k:Kind {label: 'SEOMiningRun'}), (t:Trait {key: 'job'})"));

        // All context_budget values are valid
        for line in cypher.lines() {
            if line.contains("context_budget") && line.contains('=') {
                assert!(
                    line.contains("'high'")
                        || line.contains("'medium'")
                        || line.contains("'low'")
                        || line.contains("'minimal'"),
                    "invalid context_budget: {line}"
                );
            }
        }

        // v10.8: Header mentions 60 Kind nodes
        assert!(cypher.contains("60 Kind nodes"));

        // v10.1: knowledge_tier removed from all YAMLs (node type is sufficient)
        // Verify no knowledge_tier properties are present in output
        assert!(
            !cypher.contains(".knowledge_tier"),
            "No nodes should have knowledge_tier (removed in v10.1)"
        );
    }

    /// Snapshot test for a minimal Kind generator output.
    /// Run `cargo insta review` to accept changes.
    #[test]
    fn snapshot_minimal_kinds() {
        let nodes = vec![
            make_node("Project", "tenant", "foundation", LocaleBehavior::Invariant),
            make_node_with_props(
                "Page",
                "tenant",
                "structure",
                LocaleBehavior::Invariant,
                vec![
                    ("key", "string", true),
                    ("display_name", "string", true),
                    ("slug", "string", false),
                ],
            ),
            make_node("PageL10n", "tenant", "structure", LocaleBehavior::Localized),
            make_node("Concept", "tenant", "semantic", LocaleBehavior::Invariant),
            make_node("GenerationJob", "tenant", "output", LocaleBehavior::Job),
        ];

        let cypher = generate_kind_cypher(&nodes).unwrap();
        insta::assert_snapshot!(cypher);
    }
}
