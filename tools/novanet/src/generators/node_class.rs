//! Generate Class meta-nodes + schema_hint, context_budget + facet relations.
//!
//! Reads all 57 YAML node definitions (v0.17.3 / ADR-036) and produces idempotent MERGE statements
//! for Class nodes with auto-computed properties, plus hierarchy and facet rels.
//!
//! v11.8 (ADR-023): Kind → Class, :Meta:Kind → :Schema:Class, [:HAS_KIND] → [:HAS_CLASS]
//! v0.17.3 (ADR-036): NodeTrait removed, EXHIBITS facet removed
//!
//! Output target: `packages/db/seed/01-classes.cypher`

use super::cypher_utils::{cypher_list_ref, cypher_str, write_section_header_counted};
use crate::parsers::yaml_node::{self, ParsedNode};
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

/// Determine context_budget based on layer.
///
/// v0.17.3 (ADR-036): Refactored to use layer instead of trait.
/// Provenance (who created) is now tracked per-instance, not per-class.
///
/// Rules:
/// - `output` layer → `low` (LLM-generated content, self-referential)
/// - `knowledge`, `instruction`, `config`, `locale`, `geography` → `medium`
/// - `foundation`, `structure`, `semantic` → `high`
fn context_budget(node: &ParsedNode) -> &'static str {
    match node.layer.as_str() {
        // output layer: generated content (PageNative, BlockNative)
        "output" => "low",
        // Config-like layers: knowledge atoms, instructions, settings
        "knowledge" | "instruction" | "config" | "locale" | "geography" => "medium",
        // Core layers: foundation, structure, semantic → high priority
        _ => "high",
    }
}

/// Derive visibility from realm, layer, and class name.
///
/// Visibility levels:
/// - `internal`: Not exposed (config, instruction, locale knowledge)
/// - `fragment`: Building blocks, not publishable alone (foundation, structure)
/// - `publishable`: Can be published to end users (semantic, output)
///
/// v11.2: Added as derived property, not classification axis.
fn derive_visibility(realm: &str, layer: &str, class_name: &str) -> &'static str {
    // Class-name overrides (priority 1)
    match class_name {
        // Page/Block types and templates are fragments
        // v0.12.5: PageStructure deleted (ADR-028)
        "Page" | "Block" | "BlockType" => return "fragment",
        // Generated and content nodes are publishable
        // v0.13.0 ADR-029: PageGenerated→PageNative, BlockGenerated→BlockNative, EntityContent→EntityNative
        "PageNative" | "BlockNative" => return "publishable",
        "Entity" | "EntityNative" => return "publishable",
        // SEO/GEO data is publishable (v11.4: GEOMetrics removed)
        "GEOQuery" | "GEOAnswer" | "SEOKeyword" | "SEOKeywordMetrics" => return "publishable",
        _ => {},
    }

    // Layer rules (priority 2)
    match (realm, layer) {
        // SHARED realm — always internal (locale knowledge, config)
        ("shared", _) => "internal",

        // ORG publishable layers (v11.4: seo/geo moved to shared)
        ("org", "semantic") | ("org", "output") => "publishable",

        // ORG fragment layers
        ("org", "foundation") | ("org", "structure") => "fragment",

        // ORG internal layers
        ("org", "config") | ("org", "instruction") => "internal",

        // Fallback
        _ => "internal",
    }
}

/// Compute the YAML path relative to models directory.
fn yaml_path(node: &ParsedNode) -> String {
    // Extract relative path after "node-classes/" from source_path.
    // This preserves subdirectories like atoms/ (e.g. shared/knowledge/atoms/culture-ref.yaml).
    let path_str = node.source_path.to_string_lossy();
    if let Some(idx) = path_str.find("node-classes/") {
        return path_str[idx..].to_string();
    }
    // Fallback: construct from realm/layer + filename (loses subdirectories)
    let filename = node
        .source_path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    format!("node-classes/{}/{}/{}", node.realm, node.layer, filename)
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
/// - `"EntityNative"` → `"entity-native"`
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
            // Safe: to_lowercase always returns at least one char for valid Unicode
            result.push(c.to_lowercase().next().unwrap_or(c));
        } else {
            result.push(c);
        }
    }
    result
}

// ─────────────────────────────────────────────────────────────────────────────
// Generator
// ─────────────────────────────────────────────────────────────────────────────

pub struct NodeClassGenerator;

impl super::Generator for NodeClassGenerator {
    fn name(&self) -> &'static str {
        "classes"
    }

    fn generate(&self, root: &Path) -> crate::Result<String> {
        let nodes = yaml_node::load_all_nodes(root)?;
        generate_class_cypher(&nodes)
    }
}

fn generate_class_cypher(nodes: &[ParsedNode]) -> crate::Result<String> {
    let mut out = String::with_capacity(32 * 1024);

    // Header
    writeln!(
        out,
        "// ═══════════════════════════════════════════════════════════════════════════════"
    )
    .unwrap();
    writeln!(out, "// Class Schema-Nodes — v0.17.3 (ADR-029, ADR-036)").unwrap();
    writeln!(out, "// AUTO-GENERATED by novanet — DO NOT EDIT").unwrap();
    writeln!(
        out,
        "// {count} Class nodes + hierarchy + facet relations",
        count = nodes.len()
    )
    .unwrap();
    writeln!(out, "//").unwrap();
    writeln!(out, "// ADR References:").unwrap();
    writeln!(
        out,
        "//   ADR-029: *Native Pattern (EntityNative, PageNative, HAS_NATIVE)"
    )
    .unwrap();
    writeln!(
        out,
        "//   ADR-030: Slug Ownership (Page owns URL, Entity owns semantics)"
    )
    .unwrap();
    writeln!(
        out,
        "//   ADR-036: Trait Removal (provenance is per-instance, not per-class)"
    )
    .unwrap();
    writeln!(
        out,
        "// ═══════════════════════════════════════════════════════════════════════════════"
    )
    .unwrap();
    writeln!(out).unwrap();

    // ── Section 1: Class nodes ────────────────────────────────────────────────
    write_section_header_counted(&mut out, "Class Nodes", nodes.len());
    writeln!(out).unwrap();

    for node in nodes {
        let label = &node.def.name;
        let var = format!("c_{label}");
        let display = cypher_str(&node.def.name);
        let content_val = cypher_str(&node.def.description);
        let ypath = cypher_str(&yaml_path(node));
        let props = all_properties(node);
        let req_props = required_properties(node);
        let hint = cypher_str(&build_schema_hint(node));
        let budget = context_budget(node);
        let visibility = derive_visibility(&node.realm, &node.layer, label);

        let key = to_kebab_case(label);
        let realm = &node.realm;
        let layer = &node.layer;
        // v0.17.3 (ADR-036): trait property removed, provenance is per-instance

        writeln!(out, "MERGE ({var}:Schema:Class {{label: '{label}'}})").unwrap();
        writeln!(out, "ON CREATE SET").unwrap();
        writeln!(out, "  {var}.key = '{key}',").unwrap();
        writeln!(out, "  {var}.name = '{label}',").unwrap();
        writeln!(out, "  {var}.realm = '{realm}',").unwrap();
        writeln!(out, "  {var}.layer = '{layer}',").unwrap();
        writeln!(out, "  {var}.display_name = '{display}',").unwrap();
        writeln!(out, "  {var}.content = '{content_val}',").unwrap();
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
        writeln!(out, "  {var}.visibility = '{visibility}',").unwrap();
        // v10: knowledge_tier for knowledge layer nodes only
        if let Some(tier) = &node.def.knowledge_tier {
            writeln!(out, "  {var}.knowledge_tier = '{tier}',").unwrap();
        }
        writeln!(out, "  {var}.generation_count = 0,").unwrap();
        // v0.19.0 (ADR-037): node_class discriminator (lowercase = SCHEMA node)
        writeln!(out, "  {var}.node_class = 'class',").unwrap();
        // v0.17.3 (ADR-036): Add provenance tracking
        writeln!(out, "  {var}.created_by = 'seed:schema',").unwrap();
        writeln!(out, "  {var}.created_at = datetime()").unwrap();
        writeln!(out, "ON MATCH SET").unwrap();
        writeln!(out, "  {var}.key = '{key}',").unwrap();
        writeln!(out, "  {var}.name = '{label}',").unwrap();
        writeln!(out, "  {var}.realm = '{realm}',").unwrap();
        writeln!(out, "  {var}.layer = '{layer}',").unwrap();
        writeln!(out, "  {var}.display_name = '{display}',").unwrap();
        writeln!(out, "  {var}.content = '{content_val}',").unwrap();
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
        writeln!(out, "  {var}.visibility = '{visibility}',").unwrap();
        // v10: knowledge_tier for knowledge layer nodes only
        if let Some(tier) = &node.def.knowledge_tier {
            writeln!(out, "  {var}.knowledge_tier = '{tier}',").unwrap();
        }
        writeln!(out, "  {var}.generation_count = 0,").unwrap();
        // v0.19.0 (ADR-037): Always set node_class on match too
        writeln!(out, "  {var}.node_class = 'class',").unwrap();
        writeln!(out, "  {var}.updated_at = datetime();").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 2: HAS_CLASS (Layer → Class) ──────────────────────────────────
    write_section_header_counted(
        &mut out,
        "Hierarchy: Layer -[:HAS_CLASS]-> Class",
        nodes.len(),
    );
    writeln!(out).unwrap();

    for node in nodes {
        writeln!(
            out,
            "MATCH (l:Layer {{key: '{layer}'}}), (c:Class {{label: '{label}'}})",
            layer = node.layer,
            label = node.def.name
        )
        .unwrap();
        writeln!(out, "MERGE (l)-[:HAS_CLASS]->(c);").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 3: IN_REALM (Class → Realm) ──────────────────────────────────
    write_section_header_counted(&mut out, "Facet: Class -[:IN_REALM]-> Realm", nodes.len());
    writeln!(out).unwrap();

    for node in nodes {
        writeln!(
            out,
            "MATCH (c:Class {{label: '{label}'}}), (r:Realm {{key: '{realm}'}})",
            label = node.def.name,
            realm = node.realm
        )
        .unwrap();
        writeln!(out, "MERGE (c)-[:IN_REALM]->(r);").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 4: IN_LAYER (Class → Layer) ──────────────────────────────────
    write_section_header_counted(&mut out, "Facet: Class -[:IN_LAYER]-> Layer", nodes.len());
    writeln!(out).unwrap();

    for node in nodes {
        writeln!(
            out,
            "MATCH (c:Class {{label: '{label}'}}), (l:Layer {{key: '{layer}'}})",
            label = node.def.name,
            layer = node.layer
        )
        .unwrap();
        writeln!(out, "MERGE (c)-[:IN_LAYER]->(l);").unwrap();
        writeln!(out).unwrap();
    }

    // v0.17.3 (ADR-036): Section 5 EXHIBITS (Class → Trait) removed
    // Provenance is now tracked per-instance, not per-class

    Ok(out)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::Generator;
    use crate::generators::test_utils::{make_node, make_node_with_props};
    use serial_test::serial;

    /// Clean up any test files left by other tests to avoid pollution.
    fn cleanup_test_files(root: &std::path::Path) {
        use walkdir::WalkDir;

        let node_classes_dir = crate::config::node_classes_dir(root);
        if !node_classes_dir.exists() {
            return;
        }

        for entry in WalkDir::new(node_classes_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                if name.starts_with("test-")
                    || name.starts_with("_tmp-")
                    || name.starts_with("__test__")
                    || name.contains("-test")
                {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
    }

    #[test]
    fn schema_hint_no_properties() {
        // v0.17.3: NodeTrait removed (ADR-036)
        let node = make_node("Test", "org", "foundation");
        assert_eq!(build_schema_hint(&node), "");
    }

    #[test]
    fn schema_hint_with_properties() {
        // v0.17.3: NodeTrait removed (ADR-036)
        let node = make_node_with_props(
            "Page",
            "org",
            "structure",
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
    fn context_budget_by_layer() {
        // v0.17.3 (ADR-036): context_budget now based purely on layer
        // output layer → low
        assert_eq!(
            context_budget(&make_node("PageNative", "org", "output")),
            "low"
        );
        assert_eq!(
            context_budget(&make_node("BlockNative", "org", "output")),
            "low"
        );

        // knowledge layer → medium
        assert_eq!(
            context_budget(&make_node("Expression", "shared", "knowledge")),
            "medium"
        );

        // instruction layer → medium
        assert_eq!(
            context_budget(&make_node("BlockType", "org", "instruction")),
            "medium"
        );

        // config layer → medium
        assert_eq!(
            context_budget(&make_node("Locale", "shared", "config")),
            "medium"
        );

        // locale layer → medium
        assert_eq!(
            context_budget(&make_node("LocaleVoice", "shared", "locale")),
            "medium"
        );

        // geography layer → medium
        assert_eq!(
            context_budget(&make_node("Country", "shared", "geography")),
            "medium"
        );

        // structure layer → high
        assert_eq!(
            context_budget(&make_node("Page", "org", "structure")),
            "high"
        );

        // foundation layer → high
        assert_eq!(
            context_budget(&make_node("Project", "org", "foundation")),
            "high"
        );

        // semantic layer → high
        assert_eq!(
            context_budget(&make_node("Entity", "org", "semantic")),
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
    fn to_kebab_case_native() {
        // v0.13.0 ADR-029: *Content/*Generated → *Native
        assert_eq!(to_kebab_case("EntityNative"), "entity-native");
        assert_eq!(to_kebab_case("PageNative"), "page-native");
        assert_eq!(to_kebab_case("BlockNative"), "block-native");
        assert_eq!(to_kebab_case("ProjectNative"), "project-native");
    }

    #[test]
    fn generate_small_class_cypher() {
        // v0.17.3: NodeTrait removed (ADR-036)
        let nodes = vec![
            make_node_with_props(
                "Page",
                "org",
                "structure",
                vec![("key", "string", true), ("display_name", "string", true)],
            ),
            make_node("Locale", "shared", "config"),
        ];

        let cypher = generate_class_cypher(&nodes).unwrap();

        // Header (v0.17.3: ADR-036 trait removal)
        assert!(cypher.contains("v0.17.3"));
        assert!(cypher.contains("AUTO-GENERATED"));
        assert!(cypher.contains("2 Class nodes"));

        // Class node for Page (v11.8: :Schema:Class, c_ prefix)
        assert!(cypher.contains("MERGE (c_Page:Schema:Class {label: 'Page'})"));
        assert!(cypher.contains("c_Page.display_name = 'Page'"));
        assert!(cypher.contains("c_Page.context_budget = 'high'"));
        assert!(cypher.contains("c_Page.generation_count = 0"));
        // Properties now preserve YAML definition order (key first, then display_name)
        assert!(cypher.contains("c_Page.schema_hint = 'display_name (req), key (req)'"));
        assert!(cypher.contains("c_Page.properties = ['key', 'display_name']"));
        assert!(cypher.contains("c_Page.required_properties = ['key', 'display_name']"));
        assert!(cypher.contains("c_Page.yaml_path = 'node-classes/org/structure/page.yaml'"));

        // v0.17.3: No trait property
        assert!(!cypher.contains(".trait ="));

        // Class node for Locale
        assert!(cypher.contains("MERGE (c_Locale:Schema:Class {label: 'Locale'})"));
        assert!(cypher.contains("c_Locale.context_budget = 'medium'"));

        // HAS_CLASS hierarchy (v11.8: :Class label, [:HAS_CLASS] arc)
        assert!(cypher.contains("(l:Layer {key: 'structure'}), (c:Class {label: 'Page'})"));
        assert!(cypher.contains("MERGE (l)-[:HAS_CLASS]->(c)"));

        // IN_REALM facet
        assert!(cypher.contains("(c:Class {label: 'Page'}), (r:Realm {key: 'org'})"));
        assert!(cypher.contains("MERGE (c)-[:IN_REALM]->(r)"));

        // IN_LAYER facet
        assert!(cypher.contains("(c:Class {label: 'Page'}), (l:Layer {key: 'structure'})"));
        assert!(cypher.contains("MERGE (c)-[:IN_LAYER]->(l)"));

        // v0.17.3: No EXHIBITS facet (trait removed)
        assert!(!cypher.contains("[:EXHIBITS]"));
        assert!(!cypher.contains(":Schema:Trait")); // No Trait nodes in schema

        // Section headers (v11.8: Class terminology)
        assert!(cypher.contains("Hierarchy: Layer -[:HAS_CLASS]-> Class"));
        assert!(cypher.contains("Facet: Class -[:IN_REALM]-> Realm"));
        assert!(cypher.contains("Facet: Class -[:IN_LAYER]-> Layer"));

        // Timestamps + provenance (v0.17.3 ADR-036)
        assert!(cypher.contains("created_by = 'seed:schema'"));
        assert!(cypher.contains("created_at = datetime()"));
        assert!(cypher.contains("updated_at = datetime()"));
    }

    #[test]
    #[serial]
    fn generate_class_integration() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        // Clean up any test files from parallel tests
        cleanup_test_files(root);

        let generator = NodeClassGenerator;
        let cypher = generator
            .generate(root)
            .expect("should generate kind cypher");

        // v0.20.0: 59 nodes (36 shared + 23 org)
        let class_merges = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains(":Schema:Class"))
            .count();
        assert_eq!(
            class_merges, 59,
            "expected 59 Class MERGE statements (v0.20.0: 36 shared + 23 org)"
        );

        // 59 HAS_CLASS relationships (v0.20.0)
        let has_class = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:HAS_CLASS]"))
            .count();
        assert_eq!(has_class, 59, "expected 59 HAS_CLASS relationships");

        // 59 IN_REALM relationships
        let in_realm = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:IN_REALM]"))
            .count();
        assert_eq!(in_realm, 59, "expected 59 IN_REALM relationships");

        // 59 IN_LAYER relationships
        let in_layer = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:IN_LAYER]"))
            .count();
        assert_eq!(in_layer, 59, "expected 59 IN_LAYER relationships");

        // v0.17.3: No EXHIBITS relationships (trait removed)
        let exhibits = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:EXHIBITS]"))
            .count();
        assert_eq!(
            exhibits, 0,
            "expected 0 EXHIBITS relationships (v0.17.3: trait removed)"
        );

        // Spot checks — specific Classes (v11.8: c_ prefix, :Schema:Class)
        assert!(cypher.contains("c_Project:Schema:Class {label: 'Project'}"));
        assert!(cypher.contains("c_Page:Schema:Class {label: 'Page'}"));
        assert!(cypher.contains("c_Style:Schema:Class {label: 'Style'}"));
        assert!(cypher.contains("c_SEOKeyword:Schema:Class {label: 'SEOKeyword'}"));

        // Spot check — context_budget assignments
        assert!(cypher.contains("c_Page.context_budget = 'high'"));
        assert!(cypher.contains("c_BlockType.context_budget = 'medium'"));
        assert!(cypher.contains("c_Style.context_budget = 'medium'"));
        assert!(cypher.contains("c_Locale.context_budget = 'medium'"));

        // Spot check — facet wiring (v11.8: :Class label)
        assert!(cypher.contains("(c:Class {label: 'Page'}), (r:Realm {key: 'org'})"));

        // v0.17.3: No trait references
        assert!(
            !cypher.contains(".trait ="),
            "No trait property (v0.17.3: removed)"
        );

        // All context_budget values are valid (v0.17.3: no 'minimal' since that was for retrieved trait)
        for line in cypher.lines() {
            if line.contains("context_budget") && line.contains('=') {
                assert!(
                    line.contains("'high'") || line.contains("'medium'") || line.contains("'low'"),
                    "invalid context_budget: {line}"
                );
            }
        }

        // v0.20.0: Header mentions 59 Class nodes
        assert!(cypher.contains("59 Class nodes"));

        // v10.1: knowledge_tier removed from all YAMLs (node type is sufficient)
        assert!(
            !cypher.contains(".knowledge_tier"),
            "No nodes should have knowledge_tier (removed in v10.1)"
        );
    }

    /// Snapshot test for a minimal Kind generator output.
    /// Run `cargo insta review` to accept changes.
    #[test]
    fn snapshot_minimal_kinds() {
        // v0.17.3: NodeTrait removed (ADR-036)
        let nodes = vec![
            make_node("Project", "org", "foundation"),
            make_node_with_props(
                "Page",
                "org",
                "structure",
                vec![
                    ("key", "string", true),
                    ("display_name", "string", true),
                    ("slug", "string", false),
                ],
            ),
            make_node("PageNative", "org", "output"),
            make_node("Concept", "org", "semantic"),
        ];

        let cypher = generate_class_cypher(&nodes).unwrap();
        insta::assert_snapshot!(cypher);
    }

    // =========================================================================
    // Visibility derivation tests (v11.2)
    // =========================================================================

    #[test]
    fn visibility_class_overrides() {
        // Kind-name overrides take precedence
        assert_eq!(derive_visibility("org", "structure", "Page"), "fragment");
        assert_eq!(derive_visibility("org", "structure", "Block"), "fragment");
        // v0.13.0 ADR-029: PageGenerated→PageNative, BlockGenerated→BlockNative
        assert_eq!(
            derive_visibility("org", "output", "PageNative"),
            "publishable"
        );
        assert_eq!(
            derive_visibility("org", "output", "BlockNative"),
            "publishable"
        );
        assert_eq!(
            derive_visibility("org", "semantic", "Entity"),
            "publishable"
        );
        // v0.13.0 ADR-029: EntityContent→EntityNative
        assert_eq!(
            derive_visibility("org", "semantic", "EntityNative"),
            "publishable"
        );
        // v11.4: SEO/GEO nodes in shared/knowledge but still publishable (kind-name override)
        assert_eq!(
            derive_visibility("shared", "knowledge", "SEOKeyword"),
            "publishable"
        );
        assert_eq!(
            derive_visibility("shared", "knowledge", "GEOAnswer"),
            "publishable"
        );
    }

    #[test]
    fn visibility_layer_rules() {
        // Shared realm → internal (regardless of layer)
        // v11.4: 4 shared layers (config, locale, geography, knowledge)
        assert_eq!(derive_visibility("shared", "locale", "Locale"), "internal");
        assert_eq!(derive_visibility("shared", "knowledge", "Term"), "internal");
        assert_eq!(
            derive_visibility("shared", "geography", "Continent"),
            "internal"
        );
        assert_eq!(
            derive_visibility("shared", "config", "EntityCategory"),
            "internal"
        );

        // Org publishable layers (v11.4: seo/geo removed from org)
        assert_eq!(
            derive_visibility("org", "semantic", "Unknown"),
            "publishable"
        );
        assert_eq!(derive_visibility("org", "output", "Unknown"), "publishable");

        // Org fragment layers
        assert_eq!(
            derive_visibility("org", "foundation", "Project"),
            "fragment"
        );
        assert_eq!(derive_visibility("org", "structure", "Unknown"), "fragment");

        // Org internal layers
        assert_eq!(derive_visibility("org", "config", "Org"), "internal");
        // v0.12.5: PageInstruction deleted, use BlockInstruction
        assert_eq!(
            derive_visibility("org", "instruction", "BlockInstruction"),
            "internal"
        );
    }

    #[test]
    fn visibility_fallback() {
        // Unknown realm/layer → internal
        assert_eq!(derive_visibility("unknown", "unknown", "Test"), "internal");
    }
}
