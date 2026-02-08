//! Generate ArcKind meta-nodes + cypher_pattern + FROM/TO_KIND + IN_FAMILY relations.
//!
//! Reads individual arc-kind YAML files from `packages/core/models/arc-kinds/{family}/*.yaml`,
//! filters out inverse relations, and produces idempotent MERGE statements for ArcKind nodes
//! plus hierarchy and arc-schema relationships.
//!
//! v10.7: Migrated from deprecated `relations.yaml` to individual arc-kind YAML files.
//!
//! Output target: `packages/db/seed/02-arc-kinds.cypher`

use super::cypher_utils::{cypher_list_owned, cypher_str};
use crate::parsers::arcs;
use crate::parsers::arcs::{ArcDef, ArcsDocument, Cardinality};
use crate::parsers::yaml_node;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Convert SCREAMING_SNAKE_CASE to Title Case.
/// e.g. "HAS_PAGE" -> "Has Page", "SEMANTIC_LINK" -> "Semantic Link"
fn display_name(rel_type: &str) -> String {
    rel_type
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Build a Cypher-ready traversal pattern from source/target labels.
/// e.g. "(Project)-[:HAS_PAGE]->(Page)" or "(Page, Block)-[:HAS_GENERATED]->(PageGenerated, BlockGenerated)"
fn cypher_pattern(rel: &ArcDef) -> String {
    let sources = rel.source.labels().join(", ");
    let targets = rel.target.labels().join(", ");
    format!(
        "({sources})-[:{rel_type}]->({targets})",
        rel_type = rel.arc_type
    )
}

/// Map Cardinality enum to snake_case string for Cypher property.
fn cardinality_key(c: Cardinality) -> &'static str {
    match c {
        Cardinality::OneToOne => "one_to_one",
        Cardinality::OneToMany => "one_to_many",
        Cardinality::ManyToOne => "many_to_one",
        Cardinality::ManyToMany => "many_to_many",
    }
}

/// Compute arc scope based on source/target kind realms.
/// Returns "intra_realm" if all sources and targets are in the same realm,
/// "cross_realm" if they span different realms, or None if realms can't be determined.
fn compute_scope(rel: &ArcDef, kind_realms: &HashMap<String, String>) -> Option<&'static str> {
    let source_realms: Vec<&String> = rel
        .source
        .labels()
        .iter()
        .filter_map(|l| kind_realms.get(*l))
        .collect();
    let target_realms: Vec<&String> = rel
        .target
        .labels()
        .iter()
        .filter_map(|l| kind_realms.get(*l))
        .collect();

    // If we couldn't find realms for all source/target kinds, return None
    if source_realms.len() != rel.source.len() || target_realms.len() != rel.target.len() {
        return None;
    }

    // Check if all realms are the same
    let all_realms: Vec<&String> = source_realms
        .iter()
        .chain(target_realms.iter())
        .copied()
        .collect();
    if all_realms.is_empty() {
        return None;
    }

    let first = all_realms[0];
    if all_realms.iter().all(|r| *r == first) {
        Some("intra_realm")
    } else {
        Some("cross_realm")
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Generator
// ─────────────────────────────────────────────────────────────────────────────

pub struct ArcKindGenerator;

impl super::Generator for ArcKindGenerator {
    fn name(&self) -> &'static str {
        "arc_schema"
    }

    fn generate(&self, root: &Path) -> crate::Result<String> {
        // v10.7: Load from individual arc-kind YAML files (replaces deprecated relations.yaml)
        let doc = arcs::load_arc_kinds_from_files(root)?;
        // Load temperature thresholds separately (for backward compatibility)
        let temps = arcs::load_arc_temperatures(root)?;
        // Load all nodes to build Kind -> realm map for scope computation
        let nodes = yaml_node::load_all_nodes(root)?;
        let kind_realms: HashMap<String, String> = nodes
            .into_iter()
            .map(|n| (n.def.name.clone(), n.realm))
            .collect();
        generate_arc_schema(&doc, &temps, &kind_realms)
    }
}

fn generate_arc_schema(
    doc: &ArcsDocument,
    temps: &HashMap<String, f32>,
    kind_realms: &HashMap<String, String>,
) -> crate::Result<String> {
    // Separate forward (non-inverse) from inverse relations
    let forward: Vec<&ArcDef> = doc.arcs.iter().filter(|r| !r.is_inverse()).collect();

    // Build inverse name lookup: forward_type -> inverse_type
    let inverse_names: HashMap<&str, &str> = doc
        .arcs
        .iter()
        .filter(|r| r.is_inverse())
        .filter_map(|r| {
            r.inverse_of
                .as_deref()
                .map(|fwd| (fwd, r.arc_type.as_str()))
        })
        .collect();

    let mut out = String::with_capacity(32 * 1024);

    // Header
    writeln!(
        out,
        "// ═══════════════════════════════════════════════════════════════════════════════"
    )
    .unwrap();
    writeln!(out, "// ArcKind Meta-Nodes — v11.0.0").unwrap();
    writeln!(out, "// AUTO-GENERATED by novanet — DO NOT EDIT").unwrap();
    writeln!(
        out,
        "// {count} ArcKind nodes + hierarchy + arc schema relations",
        count = forward.len()
    )
    .unwrap();
    writeln!(
        out,
        "// ═══════════════════════════════════════════════════════════════════════════════"
    )
    .unwrap();
    writeln!(out).unwrap();

    // ── Section 1: ArcKind nodes ─────────────────────────────────────────────
    write_section_header(&mut out, "ArcKind Nodes", forward.len());
    writeln!(out).unwrap();

    for rel in &forward {
        let key = &rel.arc_type;
        let var = format!("ak_{key}");
        let dn = cypher_str(&display_name(key));
        let llm = cypher_str(&rel.llm_context);
        let family = &rel.family.to_string();
        let scope = compute_scope(rel, kind_realms);
        let card = cardinality_key(rel.cardinality);
        let self_ref = rel.is_self_referential.unwrap_or(false);
        // Prefer the direct inverse_name field (from arc-kind YAML), fall back to map (relations.yaml)
        let inv: Option<&str> = rel
            .inverse_name
            .as_deref()
            .or_else(|| inverse_names.get(key.as_str()).copied());
        let props: Vec<String> = rel.properties.clone().unwrap_or_default();
        let pattern = cypher_str(&cypher_pattern(rel));

        writeln!(out, "MERGE ({var}:Meta:ArcKind {{key: '{key}'}})").unwrap();
        writeln!(out, "ON CREATE SET").unwrap();
        writeln!(out, "  {var}.display_name = '{dn}',").unwrap();
        writeln!(out, "  {var}.llm_context = '{llm}',").unwrap();
        writeln!(out, "  {var}.family = '{family}',").unwrap();
        if let Some(s) = scope {
            writeln!(out, "  {var}.scope = '{s}',").unwrap();
        } else {
            writeln!(out, "  {var}.scope = null,").unwrap();
        }
        writeln!(out, "  {var}.cardinality = '{card}',").unwrap();
        writeln!(out, "  {var}.is_self_referential = {self_ref},").unwrap();
        if let Some(inv_name) = inv {
            writeln!(out, "  {var}.inverse_name = '{inv_name}',").unwrap();
        } else {
            writeln!(out, "  {var}.inverse_name = null,").unwrap();
        }
        writeln!(
            out,
            "  {var}.arc_properties = {props},",
            props = cypher_list_owned(&props)
        )
        .unwrap();
        writeln!(out, "  {var}.cypher_pattern = '{pattern}',").unwrap();
        // Use actual temperature_threshold from arc-kind YAML if available
        if let Some(temp) = temps.get(key) {
            writeln!(out, "  {var}.temperature_threshold = {temp},").unwrap();
        } else {
            writeln!(out, "  {var}.temperature_threshold = null,").unwrap();
        }
        writeln!(out, "  {var}.created_at = datetime()").unwrap();
        writeln!(out, "ON MATCH SET").unwrap();
        writeln!(out, "  {var}.display_name = '{dn}',").unwrap();
        writeln!(out, "  {var}.llm_context = '{llm}',").unwrap();
        writeln!(out, "  {var}.family = '{family}',").unwrap();
        if let Some(s) = scope {
            writeln!(out, "  {var}.scope = '{s}',").unwrap();
        } else {
            writeln!(out, "  {var}.scope = null,").unwrap();
        }
        writeln!(out, "  {var}.cardinality = '{card}',").unwrap();
        writeln!(out, "  {var}.is_self_referential = {self_ref},").unwrap();
        if let Some(inv_name) = inv {
            writeln!(out, "  {var}.inverse_name = '{inv_name}',").unwrap();
        } else {
            writeln!(out, "  {var}.inverse_name = null,").unwrap();
        }
        writeln!(
            out,
            "  {var}.arc_properties = {props},",
            props = cypher_list_owned(&props)
        )
        .unwrap();
        writeln!(out, "  {var}.cypher_pattern = '{pattern}',").unwrap();
        // Use actual temperature_threshold from arc-kind YAML if available
        if let Some(temp) = temps.get(key) {
            writeln!(out, "  {var}.temperature_threshold = {temp},").unwrap();
        } else {
            writeln!(out, "  {var}.temperature_threshold = null,").unwrap();
        }
        writeln!(out, "  {var}.updated_at = datetime();").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 2: HAS_ARC_KIND (ArcFamily → ArcKind) ─────────────────────────
    write_section_header(
        &mut out,
        "Hierarchy: ArcFamily -[:HAS_ARC_KIND]-> ArcKind",
        forward.len(),
    );
    writeln!(out).unwrap();

    for rel in &forward {
        writeln!(
            out,
            "MATCH (af:ArcFamily {{key: '{family}'}}), (ak:ArcKind {{key: '{key}'}})",
            family = rel.family,
            key = rel.arc_type
        )
        .unwrap();
        writeln!(out, "MERGE (af)-[:HAS_ARC_KIND]->(ak);").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 3: IN_FAMILY (ArcKind → ArcFamily) ───────────────────────────
    write_section_header(
        &mut out,
        "Facet: ArcKind -[:IN_FAMILY]-> ArcFamily",
        forward.len(),
    );
    writeln!(out).unwrap();

    for rel in &forward {
        writeln!(
            out,
            "MATCH (ak:ArcKind {{key: '{key}'}}), (af:ArcFamily {{key: '{family}'}})",
            key = rel.arc_type,
            family = rel.family
        )
        .unwrap();
        writeln!(out, "MERGE (ak)-[:IN_FAMILY]->(af);").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 4: FROM_KIND (ArcKind → Kind source labels) ─────────────────
    let from_count: usize = forward.iter().map(|r| r.source.len()).sum();
    write_section_header(
        &mut out,
        "Arc Schema: ArcKind -[:FROM_KIND]-> Kind",
        from_count,
    );
    writeln!(out).unwrap();

    for rel in &forward {
        for source in rel.source.labels() {
            writeln!(
                out,
                "MATCH (ak:ArcKind {{key: '{key}'}}), (k:Kind {{label: '{source}'}})",
                key = rel.arc_type
            )
            .unwrap();
            writeln!(out, "MERGE (ak)-[:FROM_KIND]->(k);").unwrap();
            writeln!(out).unwrap();
        }
    }

    // ── Section 5: TO_KIND (ArcKind → Kind target labels) ───────────────────
    let to_count: usize = forward.iter().map(|r| r.target.len()).sum();
    write_section_header(&mut out, "Arc Schema: ArcKind -[:TO_KIND]-> Kind", to_count);
    writeln!(out).unwrap();

    for rel in &forward {
        for target in rel.target.labels() {
            writeln!(
                out,
                "MATCH (ak:ArcKind {{key: '{key}'}}), (k:Kind {{label: '{target}'}})",
                key = rel.arc_type
            )
            .unwrap();
            writeln!(out, "MERGE (ak)-[:TO_KIND]->(k);").unwrap();
            writeln!(out).unwrap();
        }
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
    use crate::parsers::arcs::{ArcFamily, NodeRef};

    fn make_rel(
        rel_type: &str,
        family: ArcFamily,
        source: NodeRef,
        target: NodeRef,
        cardinality: Cardinality,
    ) -> ArcDef {
        ArcDef {
            arc_type: rel_type.to_string(),
            family,
            source,
            target,
            cardinality,
            llm_context: format!("{rel_type} context."),
            properties: None,
            is_self_referential: None,
            inverse_of: None,
            inverse_name: None,
        }
    }

    fn make_inverse(rel_type: &str, family: ArcFamily, inverse_of: &str) -> ArcDef {
        ArcDef {
            arc_type: rel_type.to_string(),
            family,
            source: NodeRef::Single("B".to_string()),
            target: NodeRef::Single("A".to_string()),
            cardinality: Cardinality::ManyToOne,
            llm_context: "Inverse.".to_string(),
            properties: None,
            is_self_referential: None,
            inverse_of: Some(inverse_of.to_string()),
            inverse_name: None,
        }
    }

    /// Build a mock kind_realms map for tests.
    fn mock_kind_realms() -> HashMap<String, String> {
        let mut m = HashMap::new();
        // Common test kinds - all in tenant realm for simplicity
        m.insert("Project".to_string(), "tenant".to_string());
        m.insert("Page".to_string(), "tenant".to_string());
        m.insert("Block".to_string(), "tenant".to_string());
        m.insert("Entity".to_string(), "tenant".to_string());
        m.insert("PageGenerated".to_string(), "tenant".to_string());
        m.insert("BlockGenerated".to_string(), "tenant".to_string());
        // Locale is in global realm (cross_realm test)
        m.insert("Locale".to_string(), "global".to_string());
        m
    }

    #[test]
    fn display_name_simple() {
        assert_eq!(display_name("HAS_PAGE"), "Has Page");
    }

    #[test]
    fn display_name_three_words() {
        assert_eq!(display_name("SEMANTIC_LINK"), "Semantic Link");
    }

    #[test]
    fn display_name_single_word() {
        assert_eq!(display_name("GENERATES"), "Generates");
    }

    #[test]
    fn cypher_pattern_simple() {
        let rel = make_rel(
            "HAS_PAGE",
            ArcFamily::Ownership,
            NodeRef::Single("Project".to_string()),
            NodeRef::Single("Page".to_string()),
            Cardinality::OneToMany,
        );
        assert_eq!(cypher_pattern(&rel), "(Project)-[:HAS_PAGE]->(Page)");
    }

    #[test]
    fn cypher_pattern_multi_source_target() {
        let rel = make_rel(
            "HAS_GENERATED",
            ArcFamily::Localization,
            NodeRef::Multiple(vec!["Page".to_string(), "Block".to_string()]),
            NodeRef::Multiple(vec!["PageGenerated".to_string(), "BlockGenerated".to_string()]),
            Cardinality::OneToMany,
        );
        assert_eq!(
            cypher_pattern(&rel),
            "(Page, Block)-[:HAS_GENERATED]->(PageGenerated, BlockGenerated)"
        );
    }

    #[test]
    fn cardinality_key_values() {
        assert_eq!(cardinality_key(Cardinality::OneToOne), "one_to_one");
        assert_eq!(cardinality_key(Cardinality::OneToMany), "one_to_many");
        assert_eq!(cardinality_key(Cardinality::ManyToOne), "many_to_one");
        assert_eq!(cardinality_key(Cardinality::ManyToMany), "many_to_many");
    }

    #[test]
    fn generate_small_arc_schema() {
        let doc = ArcsDocument {
            arcs: vec![
                make_rel(
                    "HAS_PAGE",
                    ArcFamily::Ownership,
                    NodeRef::Single("Project".to_string()),
                    NodeRef::Single("Page".to_string()),
                    Cardinality::OneToMany,
                ),
                {
                    let mut rel = make_rel(
                        "HAS_BLOCK",
                        ArcFamily::Ownership,
                        NodeRef::Single("Page".to_string()),
                        NodeRef::Single("Block".to_string()),
                        Cardinality::OneToMany,
                    );
                    rel.properties = Some(vec!["position".to_string()]);
                    rel
                },
                make_inverse("BLOCK_OF", ArcFamily::Ownership, "HAS_BLOCK"),
            ],
            semantic_link_types: None,
            examples: None,
        };

        let cypher = generate_arc_schema(&doc, &HashMap::new(), &mock_kind_realms()).unwrap();

        // Header
        assert!(cypher.contains("2 ArcKind nodes"));

        // ArcKind MERGE statements (only forward, not inverse)
        let ak_merges = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains(":Meta:ArcKind"))
            .count();
        assert_eq!(ak_merges, 2, "should have 2 ArcKind merges (no inverse)");

        // No BLOCK_OF ArcKind node
        assert!(
            !cypher.contains("ak_BLOCK_OF:Meta:ArcKind"),
            "inverse should not create ArcKind"
        );

        // Properties
        assert!(cypher.contains("ak_HAS_PAGE.cardinality = 'one_to_many'"));
        assert!(cypher.contains("ak_HAS_PAGE.display_name = 'Has Page'"));
        assert!(cypher.contains("ak_HAS_PAGE.cypher_pattern = '(Project)-[:HAS_PAGE]->(Page)'"));
        assert!(cypher.contains("ak_HAS_PAGE.inverse_name = null"));

        // HAS_BLOCK has inverse_name and properties
        assert!(cypher.contains("ak_HAS_BLOCK.inverse_name = 'BLOCK_OF'"));
        assert!(cypher.contains("ak_HAS_BLOCK.arc_properties = ['position']"));

        // HAS_PAGE has no properties
        assert!(cypher.contains("ak_HAS_PAGE.arc_properties = []"));

        // temperature_threshold always null
        assert!(cypher.contains("ak_HAS_PAGE.temperature_threshold = null"));

        // HAS_ARC_KIND relationships
        let has_ak = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:HAS_ARC_KIND]"))
            .count();
        assert_eq!(has_ak, 2);

        // IN_FAMILY relationships
        let in_family = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:IN_FAMILY]"))
            .count();
        assert_eq!(in_family, 2);

        // FROM_KIND relationships (1 source each = 2 total)
        let from_kind = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:FROM_KIND]"))
            .count();
        assert_eq!(from_kind, 2);

        // TO_KIND relationships (1 target each = 2 total)
        let to_kind = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:TO_KIND]"))
            .count();
        assert_eq!(to_kind, 2);

        // Spot check FROM_KIND wiring
        assert!(cypher.contains("(ak:ArcKind {key: 'HAS_PAGE'}), (k:Kind {label: 'Project'})"));
        assert!(cypher.contains("(ak:ArcKind {key: 'HAS_BLOCK'}), (k:Kind {label: 'Page'})"));

        // Spot check TO_KIND wiring
        assert!(cypher.contains("(ak:ArcKind {key: 'HAS_PAGE'}), (k:Kind {label: 'Page'})"));
        assert!(cypher.contains("(ak:ArcKind {key: 'HAS_BLOCK'}), (k:Kind {label: 'Block'})"));

        // Timestamps
        assert!(cypher.contains("created_at = datetime()"));
        assert!(cypher.contains("updated_at = datetime()"));
    }

    #[test]
    fn generate_multi_source_target_from_to() {
        let doc = ArcsDocument {
            arcs: vec![make_rel(
                "HAS_GENERATED",
                ArcFamily::Localization,
                NodeRef::Multiple(vec!["Page".to_string(), "Block".to_string()]),
                NodeRef::Multiple(vec!["PageGenerated".to_string(), "BlockGenerated".to_string()]),
                Cardinality::OneToMany,
            )],
            semantic_link_types: None,
            examples: None,
        };

        let cypher = generate_arc_schema(&doc, &HashMap::new(), &mock_kind_realms()).unwrap();

        // 2 FROM_KIND (Page, Block)
        let from_kind = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:FROM_KIND]"))
            .count();
        assert_eq!(from_kind, 2, "should have 2 FROM_KIND for multi-source");

        // 2 TO_KIND (PageGenerated, BlockGenerated)
        let to_kind = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:TO_KIND]"))
            .count();
        assert_eq!(to_kind, 2, "should have 2 TO_KIND for multi-target");

        // cypher_pattern includes all sources and targets
        assert!(cypher.contains("(Page, Block)-[:HAS_GENERATED]->(PageGenerated, BlockGenerated)"));
    }

    #[test]
    fn generate_self_referential() {
        let mut rel = make_rel(
            "FALLBACK_TO",
            ArcFamily::Localization,
            NodeRef::Single("Locale".to_string()),
            NodeRef::Single("Locale".to_string()),
            Cardinality::ManyToOne,
        );
        rel.is_self_referential = Some(true);

        let doc = ArcsDocument {
            arcs: vec![rel],
            semantic_link_types: None,
            examples: None,
        };

        let cypher = generate_arc_schema(&doc, &HashMap::new(), &mock_kind_realms()).unwrap();
        assert!(cypher.contains("ak_FALLBACK_TO.is_self_referential = true"));
    }

    #[test]
    fn generate_arc_schema_integration() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let generator = ArcKindGenerator;
        let cypher = generator
            .generate(root)
            .expect("should generate arc schema cypher");

        // v11.0: Count arc kinds (123 total: +3 inverse arcs READ_BY, ENHANCED_BY, OPERATED_BY)
        let ak_merges = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains(":Meta:ArcKind"))
            .count();
        assert_eq!(
            ak_merges, 123,
            "expected 123 ArcKind MERGE statements (v11.0 +3 inverse arcs)"
        );

        // HAS_ARC_KIND relationships match ArcKind count
        let has_ak = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:HAS_ARC_KIND]"))
            .count();
        assert_eq!(has_ak, 123, "expected 123 HAS_ARC_KIND relationships");

        // IN_FAMILY relationships match ArcKind count
        let in_family = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:IN_FAMILY]"))
            .count();
        assert_eq!(in_family, 123, "expected 123 IN_FAMILY relationships");

        // Family distribution (non-inverse counts)
        // Section 2 MATCH lines have ArcFamily first: "MATCH (af:ArcFamily ..."
        let count_family = |family: &str| {
            cypher
                .lines()
                .filter(|l: &&str| {
                    l.contains("MATCH (af:ArcFamily")
                        && l.contains(&format!("ArcFamily {{key: '{family}'}}"))
                })
                .count()
        };

        let ownership = count_family("ownership");
        let localization = count_family("localization");
        let semantic = count_family("semantic");
        let generation = count_family("generation");
        let mining = count_family("mining");

        // v11.0: Total arcs = 123 (ownership=47, semantic=41 +3 inverse, localization=15, generation=17, mining=3)
        assert!(
            ownership + localization + semantic + generation + mining == 123,
            "family counts should sum to 123: o={ownership} l={localization} s={semantic} g={generation} m={mining}"
        );

        // Spot checks — specific ArcKinds
        assert!(cypher.contains("ak_HAS_PAGE:Meta:ArcKind {key: 'HAS_PAGE'}"));
        assert!(cypher.contains("ak_HAS_BLOCK:Meta:ArcKind {key: 'HAS_BLOCK'}"));
        assert!(cypher.contains("ak_FOR_LOCALE:Meta:ArcKind {key: 'FOR_LOCALE'}"));

        // Spot check — inverse_name populated
        assert!(
            cypher.contains("ak_HAS_BLOCK.inverse_name = 'BLOCK_OF'"),
            "HAS_BLOCK should have inverse_name BLOCK_OF"
        );

        // Spot check — FROM_KIND/TO_KIND for HAS_PAGE
        assert!(cypher.contains("(ak:ArcKind {key: 'HAS_PAGE'}), (k:Kind {label: 'Project'})"));

        // All cardinality values are valid
        for line in cypher.lines() {
            if line.contains(".cardinality = '") {
                assert!(
                    line.contains("'one_to_one'")
                        || line.contains("'one_to_many'")
                        || line.contains("'many_to_one'")
                        || line.contains("'many_to_many'"),
                    "invalid cardinality: {line}"
                );
            }
        }

        // v11.0: Header reflects count (123 total, +3 inverse arcs)
        assert!(cypher.contains("123 ArcKind nodes"));
    }

    /// Snapshot test for a minimal ArcSchema generator output.
    /// Run `cargo insta review` to accept changes.
    #[test]
    fn snapshot_minimal_arcs() {
        use crate::parsers::arcs::ArcsDocument;

        let doc = ArcsDocument {
            arcs: vec![
                make_rel(
                    "HAS_PAGE",
                    ArcFamily::Ownership,
                    NodeRef::Single("Project".to_string()),
                    NodeRef::Single("Page".to_string()),
                    Cardinality::OneToMany,
                ),
                make_inverse("PAGE_OF", ArcFamily::Ownership, "HAS_PAGE"),
                make_rel(
                    "USES_ENTITY",
                    ArcFamily::Semantic,
                    NodeRef::Single("Page".to_string()),
                    NodeRef::Single("Entity".to_string()),
                    Cardinality::ManyToMany,
                ),
                make_rel(
                    "FOR_LOCALE",
                    ArcFamily::Localization,
                    NodeRef::Single("PageGenerated".to_string()),
                    NodeRef::Single("Locale".to_string()),
                    Cardinality::ManyToOne,
                ),
            ],
            semantic_link_types: None,
            examples: None,
        };

        let cypher = generate_arc_schema(&doc, &HashMap::new(), &mock_kind_realms()).unwrap();
        insta::assert_snapshot!(cypher);
    }
}
