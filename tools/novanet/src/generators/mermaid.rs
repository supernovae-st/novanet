//! Generate Mermaid flowchart with Realm/Layer/Trait coloring.
//!
//! Reads all node YAMLs, arc-classes/, and `taxonomy.yaml` (v10.9)
//! to produce a complete graph diagram with:
//! - Subgraphs grouped by Realm → Layer
//! - Node styling by node_trait (Trait)
//! - Arc styling by ArcFamily (arrow style + color)
//!
//! Output target: `packages/core/models/docs/complete-graph.md` (Markdown wrapper)

use crate::parsers::arcs;
use crate::parsers::arcs::{ArcDef, ArcFamily};
use crate::parsers::organizing::{self, OrganizingDoc};
use crate::parsers::yaml_node::{self, ParsedNode};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

/// Layer → Mermaid classDef fill + stroke + text color.
/// v0.17.3 (ADR-036): Replaced TRAIT_STYLES - provenance is per-instance, styling is by layer.
pub const LAYER_STYLES: &[(&str, &str, &str)] = &[
    ("config", "#64748b", "#475569"),      // slate - configuration
    ("locale", "#06b6d4", "#0891b2"),      // cyan - locale settings
    ("geography", "#10b981", "#059669"),   // emerald - geography
    ("knowledge", "#8b5cf6", "#7c3aed"),   // violet - knowledge atoms
    ("foundation", "#3b82f6", "#1d4ed8"),  // blue - org foundation
    ("structure", "#f59e0b", "#d97706"),   // amber - page/block structure
    ("semantic", "#ec4899", "#db2777"),    // pink - semantic entities
    ("instruction", "#84cc16", "#65a30d"), // lime - LLM instructions
    ("output", "#22c55e", "#16a34a"),      // green - generated output
];

/// Arc family → Mermaid arrow syntax.
/// Sourced from `taxonomy.yaml` arc_families section.
pub const FAMILY_ARROWS: &[(&str, &str)] = &[
    ("ownership", "-->"),
    ("localization", "-.->"),
    ("semantic", "-.->"),
    ("generation", "==>"),
    ("mining", "--o"),
];

/// Arc family → stroke color for linkStyle.
/// Sourced from `taxonomy.yaml` arc_families section.
pub const FAMILY_COLORS: &[(&str, &str)] = &[
    ("ownership", "#3b82f6"),
    ("localization", "#22c55e"),
    ("semantic", "#f97316"),
    ("generation", "#8b5cf6"),
    ("mining", "#ec4899"),
];

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Returns emoji for node_trait trait (O(1) match instead of O(n) search).
/// v0.12.0: ADR-024 Data Origin rename.
pub fn trait_emoji(behavior: &str) -> &'static str {
    match behavior {
        "defined" => "\u{1F535}",   // 🔵
        "authored" => "\u{1F7E2}",  // 🟢
        "imported" => "\u{1F7E3}",  // 🟣
        "generated" => "\u{1F31F}", // 🌟
        "retrieved" => "\u{26AA}",  // ⚪
        _ => "\u{26AA}",            // fallback: white circle
    }
}

/// Returns Mermaid arrow syntax for arc family (O(1) match).
pub fn family_arrow(family: ArcFamily) -> &'static str {
    match family {
        ArcFamily::Ownership => "-->",
        ArcFamily::Localization => "-.->",
        ArcFamily::Semantic => "-.->",
        ArcFamily::Generation => "==>",
        ArcFamily::Mining => "--o",
        ArcFamily::Schema => "..->",
    }
}

/// Returns stroke color for arc family (O(1) match).
pub fn family_color(family: ArcFamily) -> &'static str {
    match family {
        ArcFamily::Ownership => "#3b82f6",
        ArcFamily::Localization => "#22c55e",
        ArcFamily::Semantic => "#f97316",
        ArcFamily::Generation => "#8b5cf6",
        ArcFamily::Mining => "#ec4899",
        ArcFamily::Schema => "#6366f1",
    }
}

/// Realm key → emoji (from taxonomy.yaml).
pub fn realm_emoji(key: &str, doc: &OrganizingDoc) -> String {
    for realm in &doc.realms {
        if realm.key == key {
            return realm.emoji.clone();
        }
    }
    "\u{1F4E6}".to_string() // fallback: 📦
}

/// Layer key → display_name (from taxonomy.yaml).
pub fn layer_display_name(key: &str, doc: &OrganizingDoc) -> String {
    for realm in &doc.realms {
        for layer in &realm.layers {
            if layer.key == key {
                return layer.display_name.clone();
            }
        }
    }
    key.to_string()
}

/// Layer key → emoji (from taxonomy.yaml).
/// v0.17.3 (ADR-036): Added to replace trait_emoji for node labels.
pub fn layer_emoji(key: &str) -> &'static str {
    match key {
        "config" => "\u{2699}\u{fe0f}",      // ⚙️
        "locale" => "\u{1F310}",             // 🌐
        "geography" => "\u{1F5FA}\u{fe0f}",  // 🗺️
        "knowledge" => "\u{1F4DA}",          // 📚
        "foundation" => "\u{1F3DB}\u{fe0f}", // 🏛️
        "structure" => "\u{1F4C4}",          // 📄
        "semantic" => "\u{1F4A1}",           // 💡
        "instruction" => "\u{1F4DD}",        // 📝
        "output" => "\u{2728}",              // ✨
        _ => "\u{1F4E6}",                    // 📦 fallback
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Expanded arc (after multi-source/target expansion) — v10.4: renamed from edge
// ─────────────────────────────────────────────────────────────────────────────

/// A single concrete from→to arc (after expanding multi-source/target).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpandedArc {
    pub from: String,
    pub arc_type: String,
    pub to: String,
    pub family: ArcFamily,
}

/// Expand multi-source/target relations into concrete arcs, filtering wildcards.
pub fn expand_arcs(relations: &[ArcDef]) -> Vec<ExpandedArc> {
    let mut arcs = Vec::new();
    for rel in relations {
        let sources = rel.source.labels();
        let targets = rel.target.labels();
        for &src in &sources {
            if src == "*" {
                continue;
            }
            for &tgt in &targets {
                if tgt == "*" {
                    continue;
                }
                arcs.push(ExpandedArc {
                    from: src.to_string(),
                    arc_type: rel.arc_type.clone(),
                    to: tgt.to_string(),
                    family: rel.family,
                });
            }
        }
    }
    arcs.sort();
    arcs
}

// ─────────────────────────────────────────────────────────────────────────────
// Shared write helpers (used by both MermaidGenerator and ViewMermaidGenerator)
// ─────────────────────────────────────────────────────────────────────────────

/// Write Mermaid `classDef` lines for all layers.
/// v0.17.3 (ADR-036): Changed from trait-based to layer-based styling.
pub fn write_classdefs(out: &mut String) {
    writeln!(out, "  %% Layer styling (v0.17.3 ADR-036)").unwrap();
    for &(layer, fill, stroke) in LAYER_STYLES {
        writeln!(
            out,
            "  classDef {layer} fill:{fill},stroke:{stroke},color:#fff"
        )
        .unwrap();
    }
    writeln!(out).unwrap();
}

/// Write arcs with family-based arrow styles + `linkStyle` coloring.
pub fn write_arcs_and_styles(out: &mut String, arcs: &[ExpandedArc]) {
    writeln!(out, "  %% Relationships (styled by arc family)").unwrap();
    let mut arc_indices_by_family: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, arc) in arcs.iter().enumerate() {
        let arrow = family_arrow(arc.family);
        writeln!(out, "  {} {}|{}| {}", arc.from, arrow, arc.arc_type, arc.to).unwrap();
        arc_indices_by_family
            .entry(arc.family.to_string())
            .or_default()
            .push(i);
    }
    writeln!(out).unwrap();

    writeln!(out, "  %% Arc colors by family").unwrap();
    for (family_str, indices) in &arc_indices_by_family {
        let family = match family_str.as_str() {
            "ownership" => ArcFamily::Ownership,
            "localization" => ArcFamily::Localization,
            "semantic" => ArcFamily::Semantic,
            "generation" => ArcFamily::Generation,
            "mining" => ArcFamily::Mining,
            _ => continue,
        };
        let color = family_color(family);
        let idx_str: Vec<String> = indices.iter().map(|i| i.to_string()).collect();
        writeln!(
            out,
            "  linkStyle {} stroke:{color},stroke-width:2px",
            idx_str.join(",")
        )
        .unwrap();
    }
    writeln!(out).unwrap();
}

/// Write `class` assignments for all nodes.
/// v0.17.3 (ADR-036): traits removed, using layer for styling instead.
pub fn write_class_assignments(out: &mut String, nodes: &[&ParsedNode]) {
    writeln!(out, "  %% Class assignments (by layer)").unwrap();
    let mut sorted: Vec<&&ParsedNode> = nodes.iter().collect();
    sorted.sort_by_key(|n| &n.def.name);
    for node in sorted {
        writeln!(out, "  class {} {}", node.def.name, node.def.layer).unwrap();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Generator
// ─────────────────────────────────────────────────────────────────────────────

pub struct MermaidGenerator;

impl super::Generator for MermaidGenerator {
    fn name(&self) -> &'static str {
        "mermaid"
    }

    fn generate(&self, root: &Path) -> crate::Result<String> {
        let nodes = yaml_node::load_all_nodes(root)?;
        let rels_doc = arcs::load_arc_classes_from_files(root)?;
        let org_doc = organizing::load_organizing(root)?;
        render_mermaid(&nodes, &rels_doc.arcs, &org_doc)
    }
}

fn render_mermaid(
    nodes: &[ParsedNode],
    relations: &[ArcDef],
    org_doc: &OrganizingDoc,
) -> crate::Result<String> {
    let arcs = expand_arcs(relations);

    // Group nodes by realm → layer (using BTreeMap for deterministic order)
    let mut realm_layer_nodes: BTreeMap<String, BTreeMap<String, Vec<&ParsedNode>>> =
        BTreeMap::new();
    for node in nodes {
        realm_layer_nodes
            .entry(node.realm.clone())
            .or_default()
            .entry(node.layer.clone())
            .or_default()
            .push(node);
    }

    // Sort nodes within each layer by name
    for layers in realm_layer_nodes.values_mut() {
        for node_list in layers.values_mut() {
            node_list.sort_by_key(|n| &n.def.name);
        }
    }

    let arc_count = arcs.len();
    let node_count = nodes.len();

    let mut out = String::with_capacity(8192);

    // ── Header ────────────────────────────────────────────────────────────
    writeln!(out, "flowchart TB").unwrap();
    writeln!(out, "  %% NovaNet Graph v0.13.0").unwrap();
    writeln!(out, "  %% Generated: {node_count} nodes, {arc_count} arcs").unwrap();
    writeln!(
        out,
        "  %% Source: node-classes/ + arc-classes/ + taxonomy.yaml"
    )
    .unwrap();
    writeln!(out).unwrap();

    // ── classDef — Trait-based node styling ────────────────────────────────
    write_classdefs(&mut out);

    // ── Subgraphs — Realm > Layer > Nodes ─────────────────────────────────
    // Use ordering from taxonomy.yaml for realm/layer order
    for realm_def in &org_doc.realms {
        let Some(layer_map) = realm_layer_nodes.get(&realm_def.key) else {
            continue;
        };

        let emoji = realm_emoji(&realm_def.key, org_doc);
        let realm_id = format!("{}_REALM", realm_def.key.to_uppercase());
        writeln!(
            out,
            "  subgraph {realm_id}[\"{emoji} {}\"]",
            realm_def.key.to_uppercase()
        )
        .unwrap();
        writeln!(out, "    direction TB").unwrap();

        for layer_def in &realm_def.layers {
            let Some(node_list) = layer_map.get(&layer_def.key) else {
                continue;
            };
            if node_list.is_empty() {
                continue;
            }

            let display = layer_display_name(&layer_def.key, org_doc);
            let layer_id = format!("{}_{}", realm_def.key.to_uppercase(), layer_def.key);
            writeln!(out, "    subgraph {layer_id}[\"{display}\"]").unwrap();

            for node in node_list {
                // v0.17.3 (ADR-036): traits removed, using layer emoji instead
                let emoji = layer_emoji(&node.def.layer);
                writeln!(
                    out,
                    "      {}[\"{} {}\"]",
                    node.def.name, emoji, node.def.name
                )
                .unwrap();
            }
            writeln!(out, "    end").unwrap();
        }

        writeln!(out, "  end").unwrap();
        writeln!(out).unwrap();
    }

    // ── Arcs — styled by ArcFamily ──────────────────────────────────────
    write_arcs_and_styles(&mut out, &arcs);

    // ── class assignments ─────────────────────────────────────────────────
    let node_refs: Vec<&ParsedNode> = nodes.iter().collect();
    write_class_assignments(&mut out, &node_refs);

    Ok(out)
}

/// Wrap Mermaid code in a Markdown document.
pub fn wrap_in_markdown(mermaid_code: &str) -> String {
    let mut out = String::with_capacity(mermaid_code.len() + 1024);

    writeln!(out, "# NovaNet Complete Graph").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "> Auto-generated by novanet v0.13.0. Do not edit manually."
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Overview").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "This diagram shows the complete NovaNet graph schema with all 60 node types and their relationships."
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "### Legend (ADR-024)").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "| Color | Trait | Description |").unwrap();
    writeln!(out, "|-------|-------|-------------|").unwrap();
    writeln!(
        out,
        "| \u{1F535} Blue | Defined | Nodes that don't change between locales |"
    )
    .unwrap();
    writeln!(
        out,
        "| \u{1F7E2} Green | Authored | Nodes with locale-specific content |"
    )
    .unwrap();
    writeln!(
        out,
        "| \u{1F7E3} Purple | Imported | Cultural/linguistic knowledge per locale |"
    )
    .unwrap();
    writeln!(out, "| \u{1F31F} Gold | Generated | LLM-generated output |").unwrap();
    writeln!(
        out,
        "| \u{26AA} Gray | Retrieved | Computed/retrieved data |"
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "### Realms").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "- **\u{1F30D} SHARED** — Locale configuration and knowledge (shared across all projects)"
    )
    .unwrap();
    writeln!(
        out,
        "- **\u{1F4E6} ORG** — Organization-specific content structure and generation"
    )
    .unwrap();
    // v11.5: Architecture is 2 realms (shared + org), 10 layers
    writeln!(out).unwrap();
    writeln!(out, "## Graph Diagram").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "```mermaid").unwrap();
    write!(out, "{mermaid_code}").unwrap();
    writeln!(out, "```").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Arc Families").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "| Arrow | Family | Description |").unwrap();
    writeln!(out, "|-------|--------|-------------|").unwrap();
    writeln!(
        out,
        "| `-->` | Ownership | Parent-child structural relationships |"
    )
    .unwrap();
    writeln!(
        out,
        "| `.->` | Localization | Locale-specific content links |"
    )
    .unwrap();
    writeln!(
        out,
        "| `.->` | Semantic | Meaning and concept connections |"
    )
    .unwrap();
    writeln!(out, "| `==>` | Generation | LLM generation flow |").unwrap();
    writeln!(out, "| `--o` | Mining | SEO keyword mining |").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "---").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "*Generated by novanet MermaidGenerator*").unwrap();

    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::Generator;
    use crate::generators::test_utils::{make_node, make_rel};
    use crate::parsers::arcs::{Cardinality, NodeRef};
    // v0.17.3 (ADR-036): TraitDef removed, provenance is per-instance
    use crate::parsers::organizing::{ArcFamilyDef, LayerDef, RealmDef};
    use serial_test::serial;

    fn make_org_doc() -> OrganizingDoc {
        OrganizingDoc {
            version: "0.13.0".to_string(),
            realms: vec![
                RealmDef {
                    key: "shared".to_string(),
                    display_name: "Shared".to_string(),
                    emoji: "\u{1F30D}".to_string(),
                    color: "#2aa198".to_string(),
                    llm_context: "Shared context.".to_string(),
                    layers: vec![LayerDef {
                        key: "config".to_string(),
                        display_name: "Configuration".to_string(),
                        emoji: "\u{2699}\u{FE0F}".to_string(),
                        color: "#64748b".to_string(),
                        llm_context: "Config layer.".to_string(),
                    }],
                },
                RealmDef {
                    key: "org".to_string(),
                    display_name: "Org".to_string(),
                    emoji: "\u{1F4E6}".to_string(),
                    color: "#6c71c4".to_string(),
                    llm_context: "Org context.".to_string(),
                    layers: vec![LayerDef {
                        key: "foundation".to_string(),
                        display_name: "Foundation".to_string(),
                        emoji: "\u{1F3DB}\u{FE0F}".to_string(),
                        color: "#3b82f6".to_string(),
                        llm_context: "Foundation layer.".to_string(),
                    }],
                },
            ],
            // v0.17.3 (ADR-036): traits removed, provenance is per-instance
            arc_families: vec![ArcFamilyDef {
                key: "ownership".to_string(),
                display_name: "Ownership".to_string(),
                color: "#3b82f6".to_string(),
                arrow_style: "-->".to_string(),
                llm_context: "Ownership.".to_string(),
            }],
        }
    }

    #[test]
    fn expand_arcs_filters_wildcards() {
        let rels = vec![
            make_rel("HAS_PAGE", ArcFamily::Ownership, "Project", "Page"),
            make_rel("WILDCARD", ArcFamily::Semantic, "*", "Page"),
        ];
        let expanded = expand_arcs(&rels);
        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0].from, "Project");
        assert_eq!(expanded[0].to, "Page");
    }

    #[test]
    fn expand_arcs_multi_source_target() {
        let rel = ArcDef {
            arc_type: "HAS_NATIVE".to_string(),
            family: ArcFamily::Generation,
            scope: None,
            source: NodeRef::Multiple(vec!["Page".to_string(), "Block".to_string()]),
            target: NodeRef::Multiple(vec!["PageNative".to_string(), "BlockNative".to_string()]),
            cardinality: Cardinality::OneToMany,
            llm_context: "output".to_string(),
            properties: None,
            property_defs: None,
            is_self_referential: None,
            inverse_of: None,
            inverse_name: None,
        };
        let expanded = expand_arcs(&[rel]);
        assert_eq!(expanded.len(), 4); // 2 sources × 2 targets
    }

    #[test]
    fn render_small_mermaid() {
        // v0.13.0 ADR-029: ProjectContent → ProjectNative, HAS_CONTENT → HAS_NATIVE
        // v0.17.3 (ADR-036): NodeTrait removed, provenance is per-instance
        let nodes = vec![
            make_node("Locale", "shared", "config"),
            make_node("Project", "org", "foundation"),
            make_node("ProjectNative", "org", "foundation"),
        ];

        let rels = vec![
            make_rel(
                "HAS_NATIVE",
                ArcFamily::Localization,
                "Project",
                "ProjectNative",
            ),
            make_rel(
                "FOR_LOCALE",
                ArcFamily::Localization,
                "ProjectNative",
                "Locale",
            ),
        ];

        let org_doc = make_org_doc();
        let output = render_mermaid(&nodes, &rels, &org_doc).unwrap();

        // Header
        assert!(output.contains("flowchart TB"));
        assert!(output.contains("NovaNet Graph v0.13.0"));
        assert!(output.contains("3 nodes, 2 arcs"));

        // classDef (v0.17.3 ADR-036: layer-based styling)
        assert!(output.contains("classDef config fill:#64748b,stroke:#475569,color:#fff"));
        assert!(output.contains("classDef foundation fill:#3b82f6,stroke:#1d4ed8,color:#fff"));

        // Subgraphs — Realm order from organizing doc
        assert!(output.contains("SHARED_REALM"));
        assert!(output.contains("ORG_REALM"));
        assert!(output.contains("SHARED_config"));
        assert!(output.contains("ORG_foundation"));

        // Realm labels with emoji
        assert!(output.contains("\"\u{1F30D} SHARED\""));
        assert!(output.contains("\"\u{1F4E6} ORG\""));

        // Layer display names
        assert!(output.contains("\"Configuration\""));
        assert!(output.contains("\"Foundation\""));

        // Node labels with emoji (v0.17.3: layer emoji instead of trait)
        assert!(output.contains("Locale[\"\u{2699}\u{fe0f} Locale\"]"));
        assert!(output.contains("Project[\"\u{1F3DB}\u{fe0f} Project\"]"));
        assert!(output.contains("ProjectNative[\"\u{1F3DB}\u{fe0f} ProjectNative\"]"));

        // Edges
        assert!(output.contains("Project -.->|HAS_NATIVE| ProjectNative"));
        assert!(output.contains("ProjectNative -.->|FOR_LOCALE| Locale"));

        // linkStyle
        assert!(output.contains("stroke:#22c55e,stroke-width:2px"));

        // Class assignments (v0.12.0: defined, authored)
        // v0.17.3 (ADR-036): classes by layer instead of trait
        assert!(output.contains("class Locale config"));
        assert!(output.contains("class Project foundation"));
        assert!(output.contains("class ProjectNative foundation"));
    }

    #[test]
    fn render_edge_families_distinct_arrows() {
        // v0.17.3 (ADR-036): NodeTrait removed, provenance is per-instance
        let nodes = vec![
            make_node("A", "shared", "config"),
            make_node("B", "shared", "config"),
            make_node("C", "shared", "config"),
        ];

        let rels = vec![
            make_rel("OWN", ArcFamily::Ownership, "A", "B"),
            make_rel("GEN", ArcFamily::Generation, "A", "C"),
            make_rel("MINE", ArcFamily::Mining, "B", "C"),
        ];

        let org_doc = make_org_doc();
        let output = render_mermaid(&nodes, &rels, &org_doc).unwrap();

        // Different arrow styles
        assert!(output.contains("A -->|OWN| B"));
        assert!(output.contains("A ==>|GEN| C"));
        assert!(output.contains("B --o|MINE| C"));

        // Different linkStyle colors
        assert!(output.contains("stroke:#3b82f6")); // ownership
        assert!(output.contains("stroke:#8b5cf6")); // generation
        assert!(output.contains("stroke:#ec4899")); // mining
    }

    #[test]
    fn wrap_in_markdown_includes_structure() {
        let markdown = wrap_in_markdown("flowchart TB\n  A --> B\n");

        assert!(markdown.contains("# NovaNet Complete Graph"));
        assert!(markdown.contains("Auto-generated by novanet v0.13.0"));
        assert!(markdown.contains("```mermaid"));
        assert!(markdown.contains("flowchart TB"));
        assert!(markdown.contains("A --> B"));
        assert!(markdown.contains("```"));
        assert!(markdown.contains("## Arc Families"));
        assert!(markdown.contains("| `-->` | Ownership"));
        assert!(markdown.contains("| `==>` | Generation"));
        assert!(markdown.contains("| `--o` | Mining"));
    }

    #[test]
    #[serial]
    fn generate_mermaid_integration() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let generator = MermaidGenerator;
        let output = generator
            .generate(root)
            .expect("should generate Mermaid diagram");

        // Header
        assert!(output.contains("flowchart TB"));
        assert!(output.contains("NovaNet Graph v0.13.0"));

        // 2 realms (v11.3: shared + org)
        assert!(output.contains("SHARED_REALM"));
        assert!(output.contains("ORG_REALM"));

        // Sample layer subgraphs (v11.4: 4 shared + 6 org)
        assert!(output.contains("SHARED_config")); // v11.4: shared/config for classifications
        assert!(output.contains("SHARED_locale"));
        assert!(output.contains("SHARED_geography"));
        assert!(output.contains("SHARED_knowledge")); // v11.4: SEO/GEO nodes now here
        assert!(output.contains("ORG_config"));
        assert!(output.contains("ORG_foundation"));
        assert!(output.contains("ORG_structure"));
        assert!(output.contains("ORG_semantic"));
        assert!(output.contains("ORG_instruction"));
        assert!(output.contains("ORG_output"));
        // v11.4: seo/geo layers removed from org (nodes moved to shared/knowledge)

        // All 9 classDef layer styles (v0.17.3 ADR-036: layer-based styling)
        assert!(output.contains("classDef config"));
        assert!(output.contains("classDef locale"));
        assert!(output.contains("classDef geography"));
        assert!(output.contains("classDef knowledge"));
        assert!(output.contains("classDef foundation"));
        assert!(output.contains("classDef structure"));
        assert!(output.contains("classDef semantic"));
        assert!(output.contains("classDef instruction"));
        assert!(output.contains("classDef output"));

        // Spot checks — some known nodes
        assert!(output.contains("Locale["));
        assert!(output.contains("Project["));
        assert!(output.contains("Page["));
        assert!(output.contains("Block["));
        assert!(output.contains("Entity["));
        assert!(output.contains("EntityNative[")); // v0.13.0 ADR-029: renamed from EntityContent
        assert!(output.contains("PageNative[")); // v0.13.0 ADR-029: renamed from PageGenerated
        assert!(output.contains("BlockNative[")); // v0.13.0 ADR-029: renamed from BlockGenerated

        // Edges exist (at least some)
        assert!(output.contains("|HAS_PAGE|"));
        assert!(output.contains("|HAS_BLOCK|"));

        // linkStyle with arc family colors
        assert!(output.contains("stroke:#3b82f6")); // ownership
        assert!(output.contains("stroke:#22c55e")); // localization

        // Class assignments (v0.17.3 ADR-036: by layer instead of trait)
        assert!(output.contains("class Locale config"));
        assert!(output.contains("class Project foundation"));
        assert!(output.contains("class PageNative output")); // v0.13.0 ADR-029: renamed to PageNative

        // No v8 terms
        assert!(!output.contains("SCOPE_HIERARCHY"));
        assert!(!output.contains("Subcategory"));
    }
}
