//! Generate ArcClass schema-nodes + cypher_pattern + FROM/TO_CLASS + IN_FAMILY relations.
//!
//! Reads individual arc-kind YAML files from `packages/core/models/arc-classes/{family}/*.yaml`,
//! filters out inverse relations, and produces idempotent MERGE statements for ArcClass nodes
//! plus hierarchy and arc-schema relationships.
//!
//! v10.7: Migrated from deprecated `relations.yaml` to individual arc-kind YAML files.
//! v11.8 (ADR-023): ArcKind → ArcClass, :Meta:ArcKind → :Schema:ArcClass, FROM/TO_KIND → FROM/TO_CLASS
//!
//! Output target: `packages/db/seed/02-arc-classes.cypher`

use super::cypher_utils::{cypher_list_owned, cypher_str, write_section_header_counted};
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
/// e.g. "(Project)-[:HAS_PAGE]->(Page)" or "(Page, Block)-[:HAS_NATIVE]->(PageNative, BlockNative)"
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

/// Compute arc scope based on source/target class realms.
/// Returns "intra_realm" if all sources and targets are in the same realm,
/// "cross_realm" if they span different realms, or None if realms can't be determined.
fn compute_scope(rel: &ArcDef, class_realms: &HashMap<String, String>) -> Option<&'static str> {
    let source_realms: Vec<&String> = rel
        .source
        .labels()
        .iter()
        .filter_map(|l| class_realms.get(*l))
        .collect();
    let target_realms: Vec<&String> = rel
        .target
        .labels()
        .iter()
        .filter_map(|l| class_realms.get(*l))
        .collect();

    // If we couldn't find realms for all source/target classes, return None
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

pub struct ArcClassGenerator;

impl super::Generator for ArcClassGenerator {
    fn name(&self) -> &'static str {
        "arc_schema"
    }

    fn generate(&self, root: &Path) -> crate::Result<String> {
        // Load arc definitions from individual YAML files
        let doc = arcs::load_arc_classes_from_files(root)?;
        // Load temperature thresholds separately
        let temps = arcs::load_arc_temperatures(root)?;
        // Load all nodes to build Class -> realm map for scope computation
        let nodes = yaml_node::load_all_nodes(root)?;
        let class_realms: HashMap<String, String> = nodes
            .into_iter()
            .map(|n| (n.def.name.clone(), n.realm))
            .collect();
        generate_arc_schema(&doc, &temps, &class_realms)
    }
}

fn generate_arc_schema(
    doc: &ArcsDocument,
    temps: &HashMap<String, f32>,
    class_realms: &HashMap<String, String>,
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
    writeln!(out, "// ArcClass Schema-Nodes — v0.13.0 (ADR-029)").unwrap();
    writeln!(out, "// AUTO-GENERATED by novanet — DO NOT EDIT").unwrap();
    writeln!(
        out,
        "// {count} ArcClass nodes + hierarchy + arc schema relations",
        count = forward.len()
    )
    .unwrap();
    writeln!(
        out,
        "// ═══════════════════════════════════════════════════════════════════════════════"
    )
    .unwrap();
    writeln!(out).unwrap();

    // ── Section 1: ArcClass nodes ─────────────────────────────────────────────
    write_section_header_counted(&mut out, "ArcClass Nodes", forward.len());
    writeln!(out).unwrap();

    for rel in &forward {
        let key = &rel.arc_type;
        let var = format!("ac_{key}");
        let dn = cypher_str(&display_name(key));
        let llm = cypher_str(&rel.llm_context);
        let family = &rel.family.to_string();
        let scope = compute_scope(rel, class_realms);
        let card = cardinality_key(rel.cardinality);
        let self_ref = rel.is_self_referential.unwrap_or(false);
        // Prefer the direct inverse_name field (from arc-kind YAML), fall back to map (relations.yaml)
        let inv: Option<&str> = rel
            .inverse_name
            .as_deref()
            .or_else(|| inverse_names.get(key.as_str()).copied());
        let props: Vec<String> = rel.properties.clone().unwrap_or_default();
        let pattern = cypher_str(&cypher_pattern(rel));

        writeln!(out, "MERGE ({var}:Schema:ArcClass {{key: '{key}'}})").unwrap();
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

    // ── Section 2: HAS_ARC_CLASS (ArcFamily → ArcClass) ─────────────────────────
    write_section_header_counted(
        &mut out,
        "Hierarchy: ArcFamily -[:HAS_ARC_CLASS]-> ArcClass",
        forward.len(),
    );
    writeln!(out).unwrap();

    for rel in &forward {
        writeln!(
            out,
            "MATCH (af:ArcFamily {{key: '{family}'}}), (ac:ArcClass {{key: '{key}'}})",
            family = rel.family,
            key = rel.arc_type
        )
        .unwrap();
        writeln!(out, "MERGE (af)-[:HAS_ARC_CLASS]->(ac);").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 3: IN_FAMILY (ArcClass → ArcFamily) ───────────────────────────
    write_section_header_counted(
        &mut out,
        "Facet: ArcClass -[:IN_FAMILY]-> ArcFamily",
        forward.len(),
    );
    writeln!(out).unwrap();

    for rel in &forward {
        writeln!(
            out,
            "MATCH (ac:ArcClass {{key: '{key}'}}), (af:ArcFamily {{key: '{family}'}})",
            key = rel.arc_type,
            family = rel.family
        )
        .unwrap();
        writeln!(out, "MERGE (ac)-[:IN_FAMILY]->(af);").unwrap();
        writeln!(out).unwrap();
    }

    // ── Section 4: FROM_CLASS (ArcClass → Class source labels) ─────────────────
    let from_count: usize = forward.iter().map(|r| r.source.len()).sum();
    write_section_header_counted(
        &mut out,
        "Arc Schema: ArcClass -[:FROM_CLASS]-> Class",
        from_count,
    );
    writeln!(out).unwrap();

    for rel in &forward {
        for source in rel.source.labels() {
            writeln!(
                out,
                "MATCH (ac:ArcClass {{key: '{key}'}}), (c:Class {{label: '{source}'}})",
                key = rel.arc_type
            )
            .unwrap();
            writeln!(out, "MERGE (ac)-[:FROM_CLASS]->(c);").unwrap();
            writeln!(out).unwrap();
        }
    }

    // ── Section 5: TO_CLASS (ArcClass → Class target labels) ───────────────────
    let to_count: usize = forward.iter().map(|r| r.target.len()).sum();
    write_section_header_counted(
        &mut out,
        "Arc Schema: ArcClass -[:TO_CLASS]-> Class",
        to_count,
    );
    writeln!(out).unwrap();

    for rel in &forward {
        for target in rel.target.labels() {
            writeln!(
                out,
                "MATCH (ac:ArcClass {{key: '{key}'}}), (c:Class {{label: '{target}'}})",
                key = rel.arc_type
            )
            .unwrap();
            writeln!(out, "MERGE (ac)-[:TO_CLASS]->(c);").unwrap();
            writeln!(out).unwrap();
        }
    }

    Ok(out)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::Generator;
    use crate::generators::test_utils::make_rel_full as make_rel;
    use crate::parsers::arcs::{ArcFamily, NodeRef};

    fn make_inverse(rel_type: &str, family: ArcFamily, inverse_of: &str) -> ArcDef {
        ArcDef {
            arc_type: rel_type.to_string(),
            family,
            scope: None,
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

    /// Build a mock class_realms map for tests.
    fn mock_class_realms() -> HashMap<String, String> {
        let mut m = HashMap::new();
        // Common test kinds - all in org realm for simplicity
        m.insert("Project".to_string(), "org".to_string());
        m.insert("Page".to_string(), "org".to_string());
        m.insert("Block".to_string(), "org".to_string());
        m.insert("Entity".to_string(), "org".to_string());
        m.insert("PageNative".to_string(), "org".to_string());
        m.insert("BlockNative".to_string(), "org".to_string());
        // Locale is in shared realm (cross_realm test)
        m.insert("Locale".to_string(), "shared".to_string());
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
            "HAS_NATIVE",
            ArcFamily::Localization,
            NodeRef::Multiple(vec!["Page".to_string(), "Block".to_string()]),
            NodeRef::Multiple(vec!["PageNative".to_string(), "BlockNative".to_string()]),
            Cardinality::OneToMany,
        );
        assert_eq!(
            cypher_pattern(&rel),
            "(Page, Block)-[:HAS_NATIVE]->(PageNative, BlockNative)"
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

        let cypher = generate_arc_schema(&doc, &HashMap::new(), &mock_class_realms()).unwrap();

        // Header (v11.8: ArcClass terminology)
        assert!(cypher.contains("2 ArcClass nodes"));

        // ArcClass MERGE statements (only forward, not inverse) - v11.8: :Schema:ArcClass
        let ac_merges = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains(":Schema:ArcClass"))
            .count();
        assert_eq!(ac_merges, 2, "should have 2 ArcClass merges (no inverse)");

        // No BLOCK_OF ArcClass node
        assert!(
            !cypher.contains("ac_BLOCK_OF:Schema:ArcClass"),
            "inverse should not create ArcClass"
        );

        // Properties (v11.8: ac_ prefix)
        assert!(cypher.contains("ac_HAS_PAGE.cardinality = 'one_to_many'"));
        assert!(cypher.contains("ac_HAS_PAGE.display_name = 'Has Page'"));
        assert!(cypher.contains("ac_HAS_PAGE.cypher_pattern = '(Project)-[:HAS_PAGE]->(Page)'"));
        assert!(cypher.contains("ac_HAS_PAGE.inverse_name = null"));

        // HAS_BLOCK has inverse_name and properties
        assert!(cypher.contains("ac_HAS_BLOCK.inverse_name = 'BLOCK_OF'"));
        assert!(cypher.contains("ac_HAS_BLOCK.arc_properties = ['position']"));

        // HAS_PAGE has no properties
        assert!(cypher.contains("ac_HAS_PAGE.arc_properties = []"));

        // temperature_threshold always null
        assert!(cypher.contains("ac_HAS_PAGE.temperature_threshold = null"));

        // HAS_ARC_CLASS relationships (v11.8: renamed from HAS_ARC_KIND)
        let has_ac = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:HAS_ARC_CLASS]"))
            .count();
        assert_eq!(has_ac, 2);

        // IN_FAMILY relationships
        let in_family = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:IN_FAMILY]"))
            .count();
        assert_eq!(in_family, 2);

        // FROM_CLASS relationships (v11.8: renamed from FROM_KIND)
        let from_class = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:FROM_CLASS]"))
            .count();
        assert_eq!(from_class, 2);

        // TO_CLASS relationships (v11.8: renamed from TO_KIND)
        let to_class = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:TO_CLASS]"))
            .count();
        assert_eq!(to_class, 2);

        // Spot check FROM_CLASS wiring (v11.8: :ArcClass, :Class)
        assert!(cypher.contains("(ac:ArcClass {key: 'HAS_PAGE'}), (c:Class {label: 'Project'})"));
        assert!(cypher.contains("(ac:ArcClass {key: 'HAS_BLOCK'}), (c:Class {label: 'Page'})"));

        // Spot check TO_CLASS wiring
        assert!(cypher.contains("(ac:ArcClass {key: 'HAS_PAGE'}), (c:Class {label: 'Page'})"));
        assert!(cypher.contains("(ac:ArcClass {key: 'HAS_BLOCK'}), (c:Class {label: 'Block'})"));

        // Timestamps
        assert!(cypher.contains("created_at = datetime()"));
        assert!(cypher.contains("updated_at = datetime()"));
    }

    #[test]
    fn generate_multi_source_target_from_to() {
        let doc = ArcsDocument {
            arcs: vec![make_rel(
                "HAS_NATIVE",
                ArcFamily::Localization,
                NodeRef::Multiple(vec!["Page".to_string(), "Block".to_string()]),
                NodeRef::Multiple(vec!["PageNative".to_string(), "BlockNative".to_string()]),
                Cardinality::OneToMany,
            )],
            semantic_link_types: None,
            examples: None,
        };

        let cypher = generate_arc_schema(&doc, &HashMap::new(), &mock_class_realms()).unwrap();

        // 2 FROM_CLASS (Page, Block) — v11.8: renamed from FROM_CLASS
        let from_class = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:FROM_CLASS]"))
            .count();
        assert_eq!(from_class, 2, "should have 2 FROM_CLASS for multi-source");

        // 2 TO_CLASS (PageNative, BlockNative) — v11.8: renamed from TO_CLASS
        let to_class = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:TO_CLASS]"))
            .count();
        assert_eq!(to_class, 2, "should have 2 TO_CLASS for multi-target");

        // cypher_pattern includes all sources and targets
        assert!(cypher.contains("(Page, Block)-[:HAS_NATIVE]->(PageNative, BlockNative)"));
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

        let cypher = generate_arc_schema(&doc, &HashMap::new(), &mock_class_realms()).unwrap();
        assert!(cypher.contains("ac_FALLBACK_TO.is_self_referential = true"));
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

        let generator = ArcClassGenerator;
        let cypher = generator
            .generate(root)
            .expect("should generate arc schema cypher");

        // v0.13 ADR-029: 169 ArcClass nodes (merged HAS_CONTENT/HAS_GENERATED → HAS_NATIVE)
        // Brand Architecture: +HAS_BRAND, HAS_DESIGN, HAS_PRINCIPLES, HAS_PROMPT_STYLE, FOR_MARKET, INSPIRED_BY_REGION
        // ADR-028: +REFERENCES, +HAS_KEYWORD, +MENTIONS, +REFERENCED_BY, +REPRESENTED_BY (inverse of REPRESENTS)
        // ADR-028 inverses: +BRAND_OF, +DESIGN_OF, +PRINCIPLES_OF, +PROMPT_STYLE_OF
        // ADR-026 geographic inverses (11): CLASSIFIES, POPULATION_OF, PRIMARY_FOR, HAS_LOCALE,
        //   HAS_LOCALE_VARIANT, SPOKEN_BY, HAS_BRANCH, HAS_SUBCLUSTER, HAS_REGION, HAS_SUBREGION, HAS_SUBREALM
        // ADR-031: +SEO_CLUSTER_OF
        // ADR-032: +DERIVED_SLUG_FROM
        // ADR-029: -HAS_CONTENT, -CONTENT_OF, -HAS_GENERATED, -GENERATED_FOR (merged into HAS_NATIVE, NATIVE_OF)
        let ac_merges = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains(":Schema:ArcClass"))
            .count();
        assert_eq!(
            ac_merges, 182,
            "expected 182 ArcClass MERGE statements (v0.13.1 ADR-030 +SLUGIFIED_BY +SLUGIFIES +3 schema arcs)"
        );

        // HAS_ARC_CLASS relationships match ArcClass count
        let has_ac = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:HAS_ARC_CLASS]"))
            .count();
        assert_eq!(
            has_ac, 182,
            "expected 182 HAS_ARC_CLASS relationships (v0.13.1 ADR-030 +SLUGIFIED_BY +SLUGIFIES +3 schema arcs)"
        );

        // IN_FAMILY relationships match ArcClass count
        let in_family = cypher
            .lines()
            .filter(|l: &&str| l.contains("MERGE") && l.contains("[:IN_FAMILY]"))
            .count();
        assert_eq!(
            in_family, 182,
            "expected 182 IN_FAMILY relationships (v0.13.1 ADR-030 +SLUGIFIED_BY +SLUGIFIES +3 schema arcs)"
        );

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

        // v0.13.1 ADR-030 + knowledge atom arcs: Total arcs = 179
        // ownership=79 (includes TIER 1 + TIER 2 inverses + HAS_NATIVE/NATIVE_OF)
        // localization=20
        // semantic=62 (60 + 2 new: SLUGIFIED_BY + SLUGIFIES from ADR-030)
        // generation=12 (PRODUCED, PRODUCED_BY, minus merged arcs)
        // mining=6 (SEO/GEO mining arcs)
        assert!(
            ownership + localization + semantic + generation + mining == 179,
            "family counts should sum to 179: o={ownership} l={localization} s={semantic} g={generation} m={mining}"
        );

        // Spot checks — specific ArcClass nodes (v11.8: renamed from ArcClass)
        assert!(cypher.contains("ac_HAS_PAGE:Schema:ArcClass {key: 'HAS_PAGE'}"));
        assert!(cypher.contains("ac_HAS_BLOCK:Schema:ArcClass {key: 'HAS_BLOCK'}"));
        assert!(cypher.contains("ac_FOR_LOCALE:Schema:ArcClass {key: 'FOR_LOCALE'}"));

        // Spot check — inverse_name populated (v11.8: ac_ prefix)
        assert!(
            cypher.contains("ac_HAS_BLOCK.inverse_name = 'BLOCK_OF'"),
            "HAS_BLOCK should have inverse_name BLOCK_OF"
        );

        // Spot check — FROM_CLASS/TO_CLASS for HAS_PAGE (v11.8: renamed from FROM_CLASS/TO_CLASS)
        assert!(cypher.contains("(ac:ArcClass {key: 'HAS_PAGE'}), (c:Class {label: 'Project'})"));

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

        // v0.13.1 ADR-030 + knowledge atom arcs + schema arcs: Header reflects count (182 total ArcClass nodes)
        assert!(cypher.contains("182 ArcClass nodes"));
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
                    NodeRef::Single("PageNative".to_string()),
                    NodeRef::Single("Locale".to_string()),
                    Cardinality::ManyToOne,
                ),
            ],
            semantic_link_types: None,
            examples: None,
        };

        let cypher = generate_arc_schema(&doc, &HashMap::new(), &mock_class_realms()).unwrap();
        insta::assert_snapshot!(cypher);
    }
}
