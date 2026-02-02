//! Kind detail: fetch rich profile from Neo4j + format for display.
//!
//! When a Kind node is selected in the tree, the runtime spawns a
//! background task that fetches its full profile (realm, layer, trait,
//! edges) and sends the result back to update the detail pane.

use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::db::Db;
use crate::tui::theme;

/// Full profile of a Kind node, fetched from Neo4j.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Used by Phase 7B detail pane rendering
pub struct KindDetail {
    pub label: String,
    pub display_name: String,
    pub realm: Option<String>,
    pub layer: Option<String>,
    pub trait_key: Option<String>,
    pub context_budget: Option<String>,
    pub schema_hint: Option<String>,
    pub edges_in: Vec<EdgeInfo>,
    pub edges_out: Vec<EdgeInfo>,
}

/// An edge relationship to/from a Kind.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Used by Phase 7B detail pane rendering
pub struct EdgeInfo {
    pub key: String,
    pub family: String,
}

/// Fetch a Kind's full profile from Neo4j (metadata + edges).
///
/// Uses two separate queries to avoid cartesian products:
/// 1. Metadata: realm, layer, trait, context_budget, schema_hint
/// 2. Edges: FROM_KIND (outgoing) and TO_KIND (incoming) with families
#[allow(dead_code)] // Used by Phase 7B detail pane rendering
pub async fn fetch_kind_detail(db: &Db, label: &str) -> crate::Result<KindDetail> {
    // Query 1: Kind metadata
    let meta_cypher = "\
MATCH (k:Kind {label: $label})
OPTIONAL MATCH (k)-[:IN_REALM]->(r:Realm)
OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer)
OPTIONAL MATCH (k)-[:HAS_TRAIT]->(t:Trait)
RETURN k.label AS label, coalesce(k.display_name, k.label) AS display_name,
       r.key AS realm, l.key AS layer, t.key AS trait_key,
       k.context_budget AS context_budget, k.schema_hint AS schema_hint";

    let meta_rows = db
        .execute_with_params(meta_cypher, [("label", label.to_string())])
        .await?;

    let (display_name, realm, layer, trait_key, context_budget, schema_hint) =
        if let Some(row) = meta_rows.first() {
            (
                row.get::<String>("display_name")
                    .unwrap_or_else(|_| label.to_string()),
                row.get::<String>("realm").ok(),
                row.get::<String>("layer").ok(),
                row.get::<String>("trait_key").ok(),
                row.get::<String>("context_budget").ok(),
                row.get::<String>("schema_hint").ok(),
            )
        } else {
            (label.to_string(), None, None, None, None, None)
        };

    // Query 2: edges (FROM_KIND = outgoing, TO_KIND = incoming)
    let edge_cypher = "\
MATCH (ek:EdgeKind)-[:FROM_KIND]->(k:Kind {label: $label})
OPTIONAL MATCH (ek)-[:IN_FAMILY]->(ef:EdgeFamily)
RETURN ek.key AS edge_key, coalesce(ef.key, 'unknown') AS family, 'out' AS direction
UNION ALL
MATCH (ek:EdgeKind)-[:TO_KIND]->(k:Kind {label: $label})
OPTIONAL MATCH (ek)-[:IN_FAMILY]->(ef:EdgeFamily)
RETURN ek.key AS edge_key, coalesce(ef.key, 'unknown') AS family, 'in' AS direction";

    let edge_rows = db
        .execute_with_params(edge_cypher, [("label", label.to_string())])
        .await?;

    let mut edges_in = Vec::new();
    let mut edges_out = Vec::new();

    for row in &edge_rows {
        let edge_key: String = row.get("edge_key").unwrap_or_default();
        let family: String = row.get("family").unwrap_or_default();
        let direction: String = row.get("direction").unwrap_or_default();

        let info = EdgeInfo {
            key: edge_key,
            family,
        };
        match direction.as_str() {
            "in" => edges_in.push(info),
            "out" => edges_out.push(info),
            _ => {}
        }
    }

    Ok(KindDetail {
        label: label.to_string(),
        display_name,
        realm,
        layer,
        trait_key,
        context_budget,
        schema_hint,
        edges_in,
        edges_out,
    })
}

/// Format KindDetail as plain text lines (fallback for detail_lines).
#[allow(dead_code)] // Used by Phase 7B detail pane rendering
pub fn format_detail_lines(detail: &KindDetail) -> Vec<String> {
    let mut lines = vec![format!("{} (Kind)", detail.display_name), String::new()];

    if let Some(ref realm) = detail.realm {
        lines.push(format!("Realm: {realm}"));
    }
    if let Some(ref layer) = detail.layer {
        lines.push(format!("Layer: {layer}"));
    }
    if let Some(ref t) = detail.trait_key {
        lines.push(format!("Trait: {t}"));
    }
    if let Some(ref budget) = detail.context_budget {
        lines.push(format!("Budget: {budget}"));
    }
    if let Some(ref hint) = detail.schema_hint {
        lines.push(format!("Schema: {hint}"));
    }

    if !detail.edges_in.is_empty() {
        lines.push(String::new());
        lines.push(format!("EDGES IN ({})", detail.edges_in.len()));
        for e in &detail.edges_in {
            lines.push(format!("  {} ({})", e.key, e.family));
        }
    }
    if !detail.edges_out.is_empty() {
        lines.push(String::new());
        lines.push(format!("EDGES OUT ({})", detail.edges_out.len()));
        for e in &detail.edges_out {
            lines.push(format!("  {} ({})", e.key, e.family));
        }
    }
    if detail.edges_in.is_empty() && detail.edges_out.is_empty() {
        lines.push(String::new());
        lines.push("No edges defined.".to_string());
    }

    lines
}

/// Render KindDetail as styled ratatui Lines with Galaxy theme colors.
#[allow(dead_code)] // Used by Phase 7B detail pane rendering
pub fn styled_lines(detail: &KindDetail) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = vec![
        Line::from(Span::styled(
            format!("{} (Kind)", detail.display_name),
            theme::accent_bold(theme::CYBER_CYAN),
        )),
        Line::from(""),
    ];

    // Facet chips on one line
    let mut chips: Vec<Span<'static>> = Vec::new();
    if let Some(ref realm) = detail.realm {
        chips.push(Span::styled("Realm: ", theme::dim_style()));
        chips.push(Span::styled(
            realm.clone(),
            Style::default().fg(theme::realm_color(realm)),
        ));
        chips.push(Span::raw("  "));
    }
    if let Some(ref layer) = detail.layer {
        chips.push(Span::styled("Layer: ", theme::dim_style()));
        chips.push(Span::styled(
            layer.clone(),
            Style::default().fg(theme::layer_color(layer)),
        ));
        chips.push(Span::raw("  "));
    }
    if let Some(ref t) = detail.trait_key {
        chips.push(Span::styled("Trait: ", theme::dim_style()));
        chips.push(Span::styled(t.clone(), theme::trait_style(t)));
    }
    if !chips.is_empty() {
        lines.push(Line::from(chips));
    }

    if let Some(ref budget) = detail.context_budget {
        lines.push(Line::from(vec![
            Span::styled("Budget: ", theme::dim_style()),
            Span::styled(budget.clone(), Style::default().fg(theme::NOVA_WHITE)),
        ]));
    }
    if let Some(ref hint) = detail.schema_hint {
        lines.push(Line::from(vec![
            Span::styled("Schema: ", theme::dim_style()),
            Span::styled(hint.clone(), Style::default().fg(theme::NOVA_WHITE)),
        ]));
    }

    // Edges In
    if !detail.edges_in.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("EDGES IN ({})", detail.edges_in.len()),
            theme::accent_bold(theme::NOVA_WHITE),
        )));
        for e in &detail.edges_in {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(e.key.clone(), Style::default().fg(theme::NOVA_WHITE)),
                Span::styled(
                    format!(" ({})", e.family),
                    Style::default().fg(theme::family_color(&e.family)),
                ),
            ]));
        }
    }

    // Edges Out
    if !detail.edges_out.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("EDGES OUT ({})", detail.edges_out.len()),
            theme::accent_bold(theme::NOVA_WHITE),
        )));
        for e in &detail.edges_out {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(e.key.clone(), Style::default().fg(theme::NOVA_WHITE)),
                Span::styled(
                    format!(" ({})", e.family),
                    Style::default().fg(theme::family_color(&e.family)),
                ),
            ]));
        }
    }

    if detail.edges_in.is_empty() && detail.edges_out.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "No edges defined.".to_string(),
            theme::dim_style(),
        )));
    }

    lines
}

/// Total number of edges (in + out) for edge explorer bounds.
pub fn edge_count(detail: &KindDetail) -> usize {
    detail.edges_in.len() + detail.edges_out.len()
}

/// Get the edge at the given explorer index (in edges first, then out).
/// Returns (edge, direction, index_within_direction).
pub fn edge_at(detail: &KindDetail, idx: usize) -> Option<(&EdgeInfo, &str)> {
    if idx < detail.edges_in.len() {
        Some((&detail.edges_in[idx], "in"))
    } else {
        let out_idx = idx - detail.edges_in.len();
        detail.edges_out.get(out_idx).map(|e| (e, "out"))
    }
}

/// Render a focused edge explorer view as styled ratatui Lines.
pub fn edge_explorer_lines(detail: &KindDetail, cursor: usize) -> Vec<Line<'static>> {
    let total = edge_count(detail);
    if total == 0 {
        return vec![
            Line::from(Span::styled(
                format!("{} (Kind)", detail.display_name),
                theme::accent_bold(theme::CYBER_CYAN),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "No edges to explore.".to_string(),
                theme::dim_style(),
            )),
        ];
    }

    let (edge, direction) = edge_at(detail, cursor).unwrap();
    let dir_label = if direction == "in" { "IN" } else { "OUT" };
    let arrow = if direction == "in" {
        format!("? -> {}", detail.label)
    } else {
        format!("{} -> ?", detail.label)
    };

    let mut lines: Vec<Line<'static>> = vec![
        Line::from(Span::styled(
            format!("Edge Explorer ({}/{})", cursor + 1, total),
            theme::accent_bold(theme::NEBULA_PURPLE),
        )),
        Line::from(""),
        // Edge key
        Line::from(vec![
            Span::styled("Edge: ", theme::dim_style()),
            Span::styled(
                edge.key.clone(),
                Style::default()
                    .fg(theme::NOVA_WHITE)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        // Family
        Line::from(vec![
            Span::styled("Family: ", theme::dim_style()),
            Span::styled(
                edge.family.clone(),
                Style::default().fg(theme::family_color(&edge.family)),
            ),
        ]),
        // Direction
        Line::from(vec![
            Span::styled("Direction: ", theme::dim_style()),
            Span::styled(
                dir_label.to_string(),
                Style::default().fg(if direction == "in" {
                    theme::CYBER_CYAN
                } else {
                    theme::SOLAR_AMBER
                }),
            ),
        ]),
        // Arrow diagram
        Line::from(vec![
            Span::styled("Pattern: ", theme::dim_style()),
            Span::styled(arrow, Style::default().fg(theme::NOVA_WHITE)),
        ]),
        Line::from(""),
    ];

    // Cypher pattern
    let cypher = if direction == "out" {
        format!("(n:{})-[:{}]->(m)", detail.label, edge.key)
    } else {
        format!("(m)-[:{}]->(n:{})", edge.key, detail.label)
    };
    lines.push(Line::from(vec![
        Span::styled("Cypher: ", theme::dim_style()),
        Span::styled(cypher, Style::default().fg(theme::MATRIX_GREEN)),
    ]));

    // Navigation hint
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Up/Down navigate] [e close]".to_string(),
        theme::dim_style(),
    )));

    // Edge list with cursor indicator
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "All edges:".to_string(),
        theme::accent_bold(theme::NOVA_WHITE),
    )));

    let mut idx = 0;
    if !detail.edges_in.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  IN ({})", detail.edges_in.len()),
            Style::default().fg(theme::CYBER_CYAN),
        )));
        for e in &detail.edges_in {
            let marker = if idx == cursor { " > " } else { "   " };
            let style = if idx == cursor {
                Style::default()
                    .fg(theme::NOVA_WHITE)
                    .add_modifier(Modifier::BOLD)
            } else {
                theme::dim_style()
            };
            lines.push(Line::from(vec![
                Span::styled(marker.to_string(), style),
                Span::styled(e.key.clone(), style),
                Span::styled(
                    format!(" ({})", e.family),
                    Style::default().fg(theme::family_color(&e.family)),
                ),
            ]));
            idx += 1;
        }
    }
    if !detail.edges_out.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  OUT ({})", detail.edges_out.len()),
            Style::default().fg(theme::SOLAR_AMBER),
        )));
        for e in &detail.edges_out {
            let marker = if idx == cursor { " > " } else { "   " };
            let style = if idx == cursor {
                Style::default()
                    .fg(theme::NOVA_WHITE)
                    .add_modifier(Modifier::BOLD)
            } else {
                theme::dim_style()
            };
            lines.push(Line::from(vec![
                Span::styled(marker.to_string(), style),
                Span::styled(e.key.clone(), style),
                Span::styled(
                    format!(" ({})", e.family),
                    Style::default().fg(theme::family_color(&e.family)),
                ),
            ]));
            idx += 1;
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_detail() -> KindDetail {
        KindDetail {
            label: "Page".to_string(),
            display_name: "Page".to_string(),
            realm: Some("project".to_string()),
            layer: Some("structure".to_string()),
            trait_key: Some("invariant".to_string()),
            context_budget: Some("medium".to_string()),
            schema_hint: None,
            edges_in: vec![
                EdgeInfo {
                    key: "HAS_PAGE".to_string(),
                    family: "ownership".to_string(),
                },
                EdgeInfo {
                    key: "LINKS_TO".to_string(),
                    family: "semantic".to_string(),
                },
            ],
            edges_out: vec![EdgeInfo {
                key: "HAS_BLOCK".to_string(),
                family: "ownership".to_string(),
            }],
        }
    }

    #[test]
    fn format_includes_header() {
        let lines = format_detail_lines(&sample_detail());
        assert_eq!(lines[0], "Page (Kind)");
    }

    #[test]
    fn format_includes_facets() {
        let lines = format_detail_lines(&sample_detail());
        assert!(lines.iter().any(|l| l == "Realm: project"));
        assert!(lines.iter().any(|l| l == "Layer: structure"));
        assert!(lines.iter().any(|l| l == "Trait: invariant"));
        assert!(lines.iter().any(|l| l == "Budget: medium"));
    }

    #[test]
    fn format_includes_edges() {
        let lines = format_detail_lines(&sample_detail());
        assert!(lines.iter().any(|l| l.contains("EDGES IN (2)")));
        assert!(lines.iter().any(|l| l.contains("EDGES OUT (1)")));
        assert!(lines.iter().any(|l| l.contains("HAS_PAGE")));
        assert!(lines.iter().any(|l| l.contains("HAS_BLOCK")));
    }

    #[test]
    fn format_no_edges() {
        let detail = KindDetail {
            label: "Test".to_string(),
            display_name: "Test".to_string(),
            realm: None,
            layer: None,
            trait_key: None,
            context_budget: None,
            schema_hint: None,
            edges_in: vec![],
            edges_out: vec![],
        };
        let lines = format_detail_lines(&detail);
        assert!(lines.iter().any(|l| l.contains("No edges defined")));
    }

    #[test]
    fn format_optional_fields() {
        let detail = KindDetail {
            label: "Locale".to_string(),
            display_name: "Locale".to_string(),
            realm: Some("global".to_string()),
            layer: None,
            trait_key: None,
            context_budget: None,
            schema_hint: Some("locale_schema".to_string()),
            edges_in: vec![],
            edges_out: vec![],
        };
        let lines = format_detail_lines(&detail);
        assert!(lines.iter().any(|l| l == "Realm: global"));
        assert!(!lines.iter().any(|l| l.starts_with("Layer:")));
        assert!(lines.iter().any(|l| l == "Schema: locale_schema"));
    }

    #[test]
    fn styled_includes_header() {
        let lines = styled_lines(&sample_detail());
        assert!(!lines.is_empty());
        let content: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(content.contains("Page (Kind)"));
    }

    #[test]
    fn styled_includes_edges() {
        let lines = styled_lines(&sample_detail());
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(all_text.contains("EDGES IN"));
        assert!(all_text.contains("HAS_PAGE"));
        assert!(all_text.contains("EDGES OUT"));
        assert!(all_text.contains("HAS_BLOCK"));
    }

    #[test]
    fn styled_no_edges_shows_message() {
        let detail = KindDetail {
            label: "Empty".to_string(),
            display_name: "Empty".to_string(),
            realm: None,
            layer: None,
            trait_key: None,
            context_budget: None,
            schema_hint: None,
            edges_in: vec![],
            edges_out: vec![],
        };
        let lines = styled_lines(&detail);
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(all_text.contains("No edges defined"));
    }

    #[test]
    fn edge_count_sums_in_and_out() {
        let detail = sample_detail();
        assert_eq!(edge_count(&detail), 3); // 2 in + 1 out
    }

    #[test]
    fn edge_at_in_range() {
        let detail = sample_detail();
        let (edge, dir) = edge_at(&detail, 0).unwrap();
        assert_eq!(edge.key, "HAS_PAGE");
        assert_eq!(dir, "in");
        let (edge, dir) = edge_at(&detail, 2).unwrap();
        assert_eq!(edge.key, "HAS_BLOCK");
        assert_eq!(dir, "out");
    }

    #[test]
    fn edge_at_out_of_bounds() {
        let detail = sample_detail();
        assert!(edge_at(&detail, 5).is_none());
    }

    #[test]
    fn explorer_lines_shows_edge_details() {
        let detail = sample_detail();
        let lines = edge_explorer_lines(&detail, 0);
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(all_text.contains("Edge Explorer"));
        assert!(all_text.contains("HAS_PAGE"));
        assert!(all_text.contains("ownership"));
        assert!(all_text.contains("IN"));
        assert!(all_text.contains("Cypher:"));
    }

    #[test]
    fn explorer_lines_cursor_navigates() {
        let detail = sample_detail();
        // Cursor at out edge (idx 2)
        let lines = edge_explorer_lines(&detail, 2);
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(all_text.contains("Edge: "));
        assert!(all_text.contains("HAS_BLOCK"));
        assert!(all_text.contains("OUT"));
    }

    #[test]
    fn explorer_lines_no_edges() {
        let detail = KindDetail {
            label: "Empty".to_string(),
            display_name: "Empty".to_string(),
            realm: None,
            layer: None,
            trait_key: None,
            context_budget: None,
            schema_hint: None,
            edges_in: vec![],
            edges_out: vec![],
        };
        let lines = edge_explorer_lines(&detail, 0);
        let all_text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(all_text.contains("No edges to explore"));
    }
}
