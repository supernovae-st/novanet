//! Generate Mermaid diagrams for individual view definitions.
//!
//! Walks view include rules on the schema (no Neo4j needed) to determine
//! reachable node types and edges, then renders a filtered Mermaid diagram.
//!
//! Output target: `packages/core/models/docs/<view-id>.md` (one per view)

use crate::generators::mermaid::{self, ExpandedEdge};
use crate::parsers::arcs;
use crate::parsers::arcs::ArcDef;
use crate::parsers::organizing::{self, OrganizingDoc};
use crate::parsers::views::{Direction, IncludeRule, ViewDef, ViewLayer};
use crate::parsers::yaml_node::{self, ParsedNode};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// View graph resolution
// ─────────────────────────────────────────────────────────────────────────────

/// Result of resolving a view's include rules against the schema.
pub struct ViewGraph {
    pub reachable: HashSet<String>,
    pub edges: Vec<ExpandedEdge>,
}

/// Walk view include rules on the schema to find reachable types and edges.
pub fn resolve_view_graph(view: &ViewDef, relations: &[ArcDef]) -> ViewGraph {
    let mut graph = ViewGraph {
        reachable: HashSet::from([view.root.node_type.clone()]),
        edges: Vec::new(),
    };

    let frontier = HashSet::from([view.root.node_type.clone()]);
    walk_rules(&frontier, &view.include, relations, &mut graph);

    graph.edges.sort();
    graph.edges.dedup();
    graph
}

fn walk_rules(
    frontier: &HashSet<String>,
    rules: &[IncludeRule],
    relations: &[ArcDef],
    graph: &mut ViewGraph,
) {
    for rule in rules {
        let mut new_targets: HashSet<String> = HashSet::new();

        for rel in relations.iter().filter(|r| r.arc_type == rule.relation) {
            // Outgoing: edges from frontier types
            if matches!(rule.direction, Direction::Outgoing | Direction::Both) {
                for src in frontier {
                    if rel.source.labels().contains(&src.as_str()) {
                        for tgt in rel.target.labels() {
                            graph.edges.push(ExpandedEdge {
                                from: src.clone(),
                                arc_type: rule.relation.clone(),
                                to: tgt.to_string(),
                                family: rel.family,
                            });
                            new_targets.insert(tgt.to_string());
                        }
                    }
                }
            }

            // Incoming: edges to frontier types
            if matches!(rule.direction, Direction::Incoming | Direction::Both) {
                for tgt in frontier {
                    if rel.target.labels().contains(&tgt.as_str()) {
                        for src in rel.source.labels() {
                            graph.edges.push(ExpandedEdge {
                                from: src.to_string(),
                                arc_type: rule.relation.clone(),
                                to: tgt.clone(),
                                family: rel.family,
                            });
                            new_targets.insert(src.to_string());
                        }
                    }
                }
            }
        }

        graph.reachable.extend(new_targets.iter().cloned());

        // Recurse for nested includes using newly discovered types as frontier
        if let Some(nested) = &rule.include {
            if !new_targets.is_empty() {
                walk_rules(&new_targets, nested, relations, graph);
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Mermaid rendering
// ─────────────────────────────────────────────────────────────────────────────

/// Generate a complete Mermaid-in-Markdown document for a single view.
pub fn generate_view(root: &Path, view: &ViewDef) -> crate::Result<String> {
    let nodes = yaml_node::load_all_nodes(root)?;
    let rels_doc = arcs::load_arcs(root)?;
    let org_doc = organizing::load_organizing(root)?;

    let view_graph = resolve_view_graph(view, &rels_doc.arcs);

    let view_nodes: Vec<&ParsedNode> = nodes
        .iter()
        .filter(|n| view_graph.reachable.contains(&n.def.name))
        .collect();

    let mermaid_code = render_view_mermaid(&view_nodes, &view_graph.edges, &org_doc, view);

    let title = view
        .docs
        .as_ref()
        .and_then(|d| d.title.as_deref())
        .unwrap_or(&view.name);
    let description = view
        .docs
        .as_ref()
        .and_then(|d| d.description.as_deref())
        .unwrap_or(&view.description);
    let empty: Vec<String> = Vec::new();
    let notes = view
        .docs
        .as_ref()
        .and_then(|d| d.notes.as_ref())
        .unwrap_or(&empty);

    Ok(wrap_view_markdown(
        title,
        description,
        &mermaid_code,
        notes,
        &view.id,
    ))
}

fn render_view_mermaid(
    nodes: &[&ParsedNode],
    edges: &[ExpandedEdge],
    org_doc: &OrganizingDoc,
    view: &ViewDef,
) -> String {
    let mut out = String::with_capacity(4096);

    writeln!(out, "flowchart TB").unwrap();
    writeln!(out, "  %% View: {} ({})", view.name, view.id).unwrap();
    writeln!(out, "  %% {} nodes, {} edges", nodes.len(), edges.len()).unwrap();
    writeln!(out).unwrap();

    mermaid::write_classdefs(&mut out);

    // Choose subgraph grouping strategy
    let has_custom_layers = view
        .docs
        .as_ref()
        .and_then(|d| d.layers.as_ref())
        .is_some_and(|l| !l.is_empty());

    if has_custom_layers {
        let layers = view.docs.as_ref().unwrap().layers.as_ref().unwrap();
        write_custom_layers(&mut out, nodes, layers);
    } else {
        write_realm_layers(&mut out, nodes, org_doc);
    }

    mermaid::write_edges_and_styles(&mut out, edges);
    mermaid::write_class_assignments(&mut out, nodes);

    out
}

/// Render nodes grouped by the view's custom `docs.layers`.
fn write_custom_layers(out: &mut String, nodes: &[&ParsedNode], layers: &[ViewLayer]) {
    let node_map: HashMap<&str, &&ParsedNode> =
        nodes.iter().map(|n| (n.def.name.as_str(), n)).collect();

    let mut assigned: HashSet<&str> = HashSet::new();

    for layer in layers {
        let layer_id = layer.name.replace([' ', '/', '&'], "_").to_uppercase();
        writeln!(out, "  subgraph {layer_id}[\"{}\"]", layer.name).unwrap();

        for node_name in &layer.nodes {
            if let Some(node) = node_map.get(node_name.as_str()) {
                let behavior = node.def.node_trait.to_string();
                let emoji = mermaid::trait_emoji(&behavior);
                writeln!(out, "    {node_name}[\"{emoji} {node_name}\"]").unwrap();
                assigned.insert(node_name.as_str());
            }
        }

        writeln!(out, "  end").unwrap();
        writeln!(out).unwrap();
    }

    // Unassigned reachable nodes outside subgraphs
    let mut unassigned: Vec<&&ParsedNode> = nodes
        .iter()
        .filter(|n| !assigned.contains(n.def.name.as_str()))
        .collect();
    unassigned.sort_by_key(|n| &n.def.name);

    if !unassigned.is_empty() {
        writeln!(out, "  %% Additional reachable nodes").unwrap();
        for node in unassigned {
            let behavior = node.def.node_trait.to_string();
            let emoji = mermaid::trait_emoji(&behavior);
            writeln!(out, "  {}[\"{} {}\"]", node.def.name, emoji, node.def.name).unwrap();
        }
        writeln!(out).unwrap();
    }
}

/// Render nodes grouped by realm → layer (fallback when no docs.layers).
fn write_realm_layers(out: &mut String, nodes: &[&ParsedNode], org_doc: &OrganizingDoc) {
    let mut realm_layer_nodes: BTreeMap<String, BTreeMap<String, Vec<&&ParsedNode>>> =
        BTreeMap::new();
    for node in nodes {
        realm_layer_nodes
            .entry(node.realm.clone())
            .or_default()
            .entry(node.layer.clone())
            .or_default()
            .push(node);
    }
    for layers in realm_layer_nodes.values_mut() {
        for node_list in layers.values_mut() {
            node_list.sort_by_key(|n| &n.def.name);
        }
    }

    for realm_def in &org_doc.realms {
        let Some(layer_map) = realm_layer_nodes.get(&realm_def.key) else {
            continue;
        };
        let emoji = mermaid::realm_emoji(&realm_def.key, org_doc);
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
            let display = mermaid::layer_display_name(&layer_def.key, org_doc);
            let layer_id = format!("{}_{}", realm_def.key.to_uppercase(), layer_def.key);
            writeln!(out, "    subgraph {layer_id}[\"{display}\"]").unwrap();
            for node in node_list {
                let behavior = node.def.node_trait.to_string();
                let emoji = mermaid::trait_emoji(&behavior);
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
}

/// Wrap Mermaid code in a view-specific Markdown document.
fn wrap_view_markdown(
    title: &str,
    description: &str,
    mermaid_code: &str,
    notes: &[String],
    view_id: &str,
) -> String {
    let mut out = String::with_capacity(mermaid_code.len() + 1024);

    writeln!(out, "# {title}").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "> Auto-generated by novanet v9.0.0. Do not edit manually."
    )
    .unwrap();
    writeln!(out).unwrap();

    if !description.is_empty() {
        writeln!(out, "## Overview").unwrap();
        writeln!(out).unwrap();
        write!(out, "{description}").unwrap();
        if !description.ends_with('\n') {
            writeln!(out).unwrap();
        }
        writeln!(out).unwrap();
    }

    writeln!(out, "### Legend").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "| Color | Trait | Description |").unwrap();
    writeln!(out, "|-------|-------|-------------|").unwrap();
    writeln!(
        out,
        "| \u{1F535} Blue | Invariant | Nodes that don't change between locales |"
    )
    .unwrap();
    writeln!(
        out,
        "| \u{1F7E2} Green | Localized | Nodes with locale-specific content |"
    )
    .unwrap();
    writeln!(
        out,
        "| \u{1F7E3} Purple | Knowledge | Cultural/linguistic knowledge per locale |"
    )
    .unwrap();
    writeln!(
        out,
        "| \u{26AA} Gray | Derived | Computed/aggregated data |"
    )
    .unwrap();
    writeln!(
        out,
        "| \u{2699}\u{FE0F} Gray | Job | Background processing tasks |"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "## Graph Diagram").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "```mermaid").unwrap();
    write!(out, "{mermaid_code}").unwrap();
    writeln!(out, "```").unwrap();

    if !notes.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "## Notes").unwrap();
        writeln!(out).unwrap();
        for note in notes {
            writeln!(out, "- {note}").unwrap();
        }
    }

    writeln!(out).unwrap();
    writeln!(out, "---").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "*Generated by novanet ViewMermaidGenerator — view: {view_id}*"
    )
    .unwrap();

    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::arcs::{ArcDef, ArcFamily, Cardinality, NodeRef};
    use crate::parsers::views::{Direction, IncludeRule, RootDef, ViewDef};

    fn make_rel(rel_type: &str, family: ArcFamily, source: &str, target: &str) -> ArcDef {
        ArcDef {
            arc_type: rel_type.to_string(),
            family,
            source: NodeRef::Single(source.to_string()),
            target: NodeRef::Single(target.to_string()),
            cardinality: Cardinality::OneToMany,
            llm_context: format!("{rel_type} context"),
            properties: None,
            is_self_referential: None,
            inverse_of: None,
        }
    }

    fn make_view(root_type: &str, rules: Vec<IncludeRule>) -> ViewDef {
        ViewDef {
            id: "test".to_string(),
            name: "Test View".to_string(),
            description: "test".to_string(),
            version: None,
            root: RootDef {
                node_type: root_type.to_string(),
            },
            include: rules,
            filters: None,
            docs: None,
        }
    }

    fn rule(relation: &str, direction: Direction) -> IncludeRule {
        IncludeRule {
            relation: relation.to_string(),
            direction,
            depth: None,
            target_types: None,
            include: None,
        }
    }

    fn rule_with_nested(
        relation: &str,
        direction: Direction,
        nested: Vec<IncludeRule>,
    ) -> IncludeRule {
        IncludeRule {
            relation: relation.to_string(),
            direction,
            depth: None,
            target_types: None,
            include: Some(nested),
        }
    }

    #[test]
    fn resolve_simple_outgoing() {
        let rels = vec![make_rel(
            "HAS_PAGE",
            ArcFamily::Ownership,
            "Project",
            "Page",
        )];
        let view = make_view("Project", vec![rule("HAS_PAGE", Direction::Outgoing)]);
        let graph = resolve_view_graph(&view, &rels);

        assert!(graph.reachable.contains("Project"));
        assert!(graph.reachable.contains("Page"));
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0].from, "Project");
        assert_eq!(graph.edges[0].to, "Page");
    }

    #[test]
    fn resolve_incoming() {
        let rels = vec![make_rel(
            "HAS_PAGE",
            ArcFamily::Ownership,
            "Project",
            "Page",
        )];
        let view = make_view("Page", vec![rule("HAS_PAGE", Direction::Incoming)]);
        let graph = resolve_view_graph(&view, &rels);

        assert!(graph.reachable.contains("Page"));
        assert!(graph.reachable.contains("Project"));
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0].from, "Project");
        assert_eq!(graph.edges[0].to, "Page");
    }

    #[test]
    fn resolve_both_direction() {
        let rels = vec![make_rel(
            "SEMANTIC_LINK",
            ArcFamily::Semantic,
            "Concept",
            "Concept",
        )];
        let view = make_view("Concept", vec![rule("SEMANTIC_LINK", Direction::Both)]);
        let graph = resolve_view_graph(&view, &rels);

        assert!(graph.reachable.contains("Concept"));
        assert!(!graph.edges.is_empty());
    }

    #[test]
    fn resolve_nested_includes() {
        let rels = vec![
            make_rel("HAS_PAGE", ArcFamily::Ownership, "Project", "Page"),
            make_rel("HAS_BLOCK", ArcFamily::Ownership, "Page", "Block"),
        ];
        let view = make_view(
            "Project",
            vec![rule_with_nested(
                "HAS_PAGE",
                Direction::Outgoing,
                vec![rule("HAS_BLOCK", Direction::Outgoing)],
            )],
        );
        let graph = resolve_view_graph(&view, &rels);

        assert!(graph.reachable.contains("Project"));
        assert!(graph.reachable.contains("Page"));
        assert!(graph.reachable.contains("Block"));
        assert_eq!(graph.edges.len(), 2);
    }

    #[test]
    fn resolve_no_matching_relation() {
        let rels = vec![make_rel(
            "HAS_PAGE",
            ArcFamily::Ownership,
            "Project",
            "Page",
        )];
        let view = make_view("Project", vec![rule("NONEXISTENT", Direction::Outgoing)]);
        let graph = resolve_view_graph(&view, &rels);

        assert_eq!(graph.reachable.len(), 1);
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn resolve_multi_source_target() {
        let rel = ArcDef {
            arc_type: "OF_TYPE".to_string(),
            family: ArcFamily::Ownership,
            source: NodeRef::Multiple(vec!["Page".to_string(), "Block".to_string()]),
            target: NodeRef::Multiple(vec!["PageType".to_string(), "BlockType".to_string()]),
            cardinality: Cardinality::ManyToOne,
            llm_context: "type".to_string(),
            properties: None,
            is_self_referential: None,
            inverse_of: None,
        };
        let view = make_view("Block", vec![rule("OF_TYPE", Direction::Outgoing)]);
        let graph = resolve_view_graph(&view, &[rel]);

        assert!(graph.reachable.contains("Block"));
        assert!(graph.reachable.contains("PageType"));
        assert!(graph.reachable.contains("BlockType"));
        assert_eq!(graph.edges.len(), 2);
    }

    #[test]
    fn edges_are_deduped() {
        let rels = vec![make_rel(
            "SEMANTIC_LINK",
            ArcFamily::Semantic,
            "Concept",
            "Concept",
        )];
        let view = make_view("Concept", vec![rule("SEMANTIC_LINK", Direction::Both)]);
        let graph = resolve_view_graph(&view, &rels);

        // Both outgoing and incoming produce the same edge; should be deduped
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn wrap_view_markdown_structure() {
        let md = wrap_view_markdown(
            "My View",
            "Description here.\n",
            "flowchart TB\n  A --> B\n",
            &["Note one".to_string(), "Note two".to_string()],
            "my-view",
        );
        assert!(md.contains("# My View"));
        assert!(md.contains("Auto-generated by novanet v9.0.0"));
        assert!(md.contains("## Overview"));
        assert!(md.contains("Description here."));
        assert!(md.contains("```mermaid"));
        assert!(md.contains("flowchart TB"));
        assert!(md.contains("## Notes"));
        assert!(md.contains("- Note one"));
        assert!(md.contains("- Note two"));
        assert!(md.contains("view: my-view"));
    }

    fn test_root() -> Option<std::path::PathBuf> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());
        let root = root?;
        if !root.join("pnpm-workspace.yaml").exists() {
            return None;
        }
        Some(root.to_path_buf())
    }

    #[test]
    fn generate_block_generation_view_integration() {
        let Some(root) = test_root() else { return };
        let view = crate::parsers::views::load_view(&root, "block-generation")
            .expect("should load block-generation");
        let output = generate_view(&root, &view).expect("should generate view");

        assert!(output.contains("# Block Generation Context"));
        assert!(output.contains("```mermaid"));
        assert!(output.contains("flowchart TB"));
        assert!(output.contains("Block["));
        assert!(output.contains("view: block-generation"));
    }

    #[test]
    fn generate_locale_knowledge_view_integration() {
        let Some(root) = test_root() else { return };
        let view = crate::parsers::views::load_view(&root, "locale-full-knowledge")
            .expect("should load locale-full-knowledge");
        let output = generate_view(&root, &view).expect("should generate view");

        assert!(output.contains("Locale["));
        assert!(output.contains("flowchart TB"));
    }

    #[test]
    fn generate_all_views_integration() {
        let Some(root) = test_root() else { return };
        let views = crate::parsers::views::load_all_views(&root).expect("should load views");

        for view in &views {
            let result = generate_view(&root, view);
            assert!(
                result.is_ok(),
                "failed to generate view '{}': {:?}",
                view.id,
                result.err()
            );
            let output = result.unwrap();
            assert!(
                output.contains("```mermaid"),
                "view '{}' missing mermaid block",
                view.id
            );
        }
    }
}
