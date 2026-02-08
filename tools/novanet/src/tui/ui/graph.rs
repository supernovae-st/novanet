//! Graph panel rendering for TUI.
//!
//! Displays Neo4j relationships for the selected Kind or Instance,
//! realm/layer statistics, and arc details.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use std::collections::BTreeMap;

use super::super::app::{App, Focus};
use super::super::data::TreeItem;
use super::super::theme;
use super::{
    COLOR_UNFOCUSED_BORDER, STYLE_ACCENT, STYLE_BRIGHT_DIM, STYLE_DIM, STYLE_HIGHLIGHT, STYLE_INFO,
    STYLE_MUTED, STYLE_PRIMARY, STYLE_SUCCESS, spinner, wrap_text,
};

// =============================================================================
// GRAPH PANEL
// =============================================================================

/// Graph panel: Displays Neo4j relationships for the selected Kind or Instance.
///
/// Shows real arc data from Neo4j when a Kind is selected,
/// instance arcs in Data mode, or contextual messages for other selections.
pub fn render_graph_panel(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.theme; // Use cached theme from App
    let focused = app.focus == Focus::Graph;
    let border_color = if focused {
        Color::Magenta
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Calculate arc count for title
    let arc_count = if let Some(ref arcs) = app.kind_arcs {
        arcs.incoming.len() + arcs.outgoing.len()
    } else if let Some(TreeItem::Instance(_, _, _, inst)) = app.current_item() {
        inst.outgoing_arcs.len() + inst.incoming_arcs.len()
    } else {
        0
    };

    // Build title with count
    let title = if arc_count > 0 {
        format!(" Arc Relationships ({}) ", arc_count)
    } else {
        " Arc Relationships ".to_string()
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();
    let dim = Style::default().fg(Color::Rgb(100, 100, 100));
    let bright_dim = STYLE_BRIGHT_DIM;

    // === LOADING INDICATOR (specific message based on what's loading) ===
    let loading_msg = if app.pending_arcs_load.is_some() {
        Some("Loading arc relationships...")
    } else if app.pending_arc_kind_load.is_some() {
        Some("Loading arc kind details...")
    } else if app.pending_realm_load.is_some() {
        Some("Loading realm statistics...")
    } else if app.pending_layer_load.is_some() {
        Some("Loading layer statistics...")
    } else {
        None
    };

    if let Some(msg) = loading_msg {
        lines.push(Line::from(Span::styled(
            format!("  {} {}", spinner(app.tick), msg),
            STYLE_HIGHLIGHT,
        )));
        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === REALM DETAILS VIEW ===
    if let Some(ref details) = app.realm_details {
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &details.display_name,
                Style::default()
                    .fg(theme.realm_color(&details.key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Realm)", STYLE_DIM),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Description (if any)
        if !details.description.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("  {}", details.description),
                Style::default().fg(Color::Rgb(180, 180, 180)),
            )));
            lines.push(Line::from(Span::raw("")));
        }

        // Stats summary
        lines.push(Line::from(vec![
            Span::styled("  \u{25aa}", dim),
            Span::styled(format!("{} Layers", details.layers.len()), STYLE_INFO),
            Span::styled(" \u{b7} ", dim),
            Span::styled(format!("{} Node Kinds", details.total_kinds), STYLE_SUCCESS),
            Span::styled(" \u{b7} ", dim),
            Span::styled(
                format!("{} Instances", details.total_instances),
                STYLE_HIGHLIGHT,
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Layers with kind counts (horizontal bar chart)
        lines.push(Line::from(Span::styled(
            "  LAYERS",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            dim,
        )));

        let max_kinds = details
            .layers
            .iter()
            .map(|l| l.kind_count)
            .max()
            .unwrap_or(1)
            .max(1);
        let bar_max_width = 20usize;

        for layer in &details.layers {
            let bar_width = (layer.kind_count * bar_max_width) / max_kinds;
            let bar = "\u{2588}".repeat(bar_width.max(1));

            lines.push(Line::from(vec![
                Span::styled("    ", dim),
                Span::styled(
                    format!("{:16}", layer.display_name),
                    Style::default().fg(theme.layer_color(&layer.key)),
                ),
                Span::styled(format!("{:>3} ", layer.kind_count), bright_dim),
                Span::styled(bar, Style::default().fg(theme.layer_color(&layer.key))),
            ]));
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === LAYER DETAILS VIEW ===
    if let Some(ref details) = app.layer_details {
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &details.realm,
                Style::default().fg(theme.realm_color(&details.realm)),
            ),
            Span::styled(" \u{2192} ", bright_dim),
            Span::styled(
                &details.display_name,
                Style::default()
                    .fg(theme.layer_color(&details.key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Layer)", STYLE_DIM),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Description (if any)
        if !details.description.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("  {}", details.description),
                Style::default().fg(Color::Rgb(180, 180, 180)),
            )));
            lines.push(Line::from(Span::raw("")));
        }

        // Stats summary
        lines.push(Line::from(vec![
            Span::styled("  \u{25aa}", dim),
            Span::styled(format!("{} Node Kinds", details.total_kinds), STYLE_SUCCESS),
            Span::styled(" \u{b7} ", dim),
            Span::styled(
                format!("{} Instances", details.total_instances),
                STYLE_HIGHLIGHT,
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Node Kinds grouped by trait
        lines.push(Line::from(Span::styled(
            "  NODE KINDS BY TRAIT",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            dim,
        )));

        for group in &details.kinds_by_trait {
            // Trait header with color
            let trait_color = theme.trait_color(&group.trait_key);
            lines.push(Line::from(vec![
                Span::styled("    ", dim),
                Span::styled(
                    format!("{} ({})", group.trait_key, group.kind_names.len()),
                    Style::default()
                        .fg(trait_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            // Kind names
            for kind_name in &group.kind_names {
                lines.push(Line::from(vec![
                    Span::styled("      \u{2022} ", dim),
                    Span::styled(
                        kind_name,
                        Style::default().fg(theme.layer_color(&details.key)),
                    ),
                ]));
            }
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === INSTANCE ARCS VIEW (Data mode) ===
    if let Some(TreeItem::Instance(realm, layer, kind, instance)) = app.current_item() {
        // Use references where possible, clone only when Span needs ownership
        let realm_key = &realm.key;
        let layer_key = &layer.key;
        let instance_key = &instance.key;

        // Breadcrumb for instance
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                realm_key.clone(),
                Style::default()
                    .fg(theme.realm_color(realm_key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" \u{2192} ", bright_dim),
            Span::styled(
                layer_key.clone(),
                Style::default()
                    .fg(theme.layer_color(layer_key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" \u{2192} ", bright_dim),
            Span::styled(kind.display_name.clone(), STYLE_SUCCESS),
            Span::styled(" \u{2192} ", bright_dim),
            Span::styled(
                instance_key.clone(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Use references to arc vectors (no clone needed)
        let total = instance.outgoing_arcs.len() + instance.incoming_arcs.len();

        if total == 0 {
            lines.push(Line::from(Span::styled(
                "  No arc connections for this instance",
                STYLE_DIM,
            )));
        } else {
            // Outgoing arcs (iterate over reference, no clone of Vec)
            if !instance.outgoing_arcs.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!(
                        "  \u{2500}\u{25b6} OUTGOING ({}) ",
                        instance.outgoing_arcs.len()
                    ),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(
                    "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
                    dim,
                )));

                for arc in &instance.outgoing_arcs {
                    let status_style = if arc.exists {
                        STYLE_SUCCESS
                    } else {
                        STYLE_HIGHLIGHT
                    };
                    let status_char = if arc.exists { "\u{2713}" } else { "\u{25cb}" };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", status_char), status_style),
                        Span::styled(instance_key.clone(), STYLE_PRIMARY),
                        Span::styled(" \u{2500}\u{2500}[", dim),
                        Span::styled(
                            arc.arc_type.clone(),
                            Style::default()
                                .fg(theme.arc_family_color("semantic"))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("]\u{2500}\u{2500}\u{25b6} ", dim),
                        Span::styled(arc.target_key.clone(), STYLE_SUCCESS),
                        Span::styled(format!(" ({})", arc.target_kind), STYLE_DIM),
                    ]));
                }
                lines.push(Line::from(Span::raw("")));
            }

            // Incoming arcs (iterate over reference, no clone of Vec)
            if !instance.incoming_arcs.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!(
                        "  \u{25c0}\u{2500} INCOMING ({}) ",
                        instance.incoming_arcs.len()
                    ),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(
                    "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
                    dim,
                )));

                for arc in &instance.incoming_arcs {
                    let status_style = if arc.exists {
                        STYLE_SUCCESS
                    } else {
                        STYLE_HIGHLIGHT
                    };
                    let status_char = if arc.exists { "\u{2713}" } else { "\u{25cb}" };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", status_char), status_style),
                        Span::styled(arc.target_key.clone(), STYLE_SUCCESS),
                        Span::styled(format!(" ({})", arc.target_kind), STYLE_DIM),
                        Span::styled(" \u{2500}\u{2500}[", dim),
                        Span::styled(
                            arc.arc_type.clone(),
                            Style::default()
                                .fg(theme.arc_family_color("semantic"))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("]\u{2500}\u{2500}\u{25b6} ", dim),
                        Span::styled(instance_key.clone(), STYLE_PRIMARY),
                    ]));
                }
            }
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === KIND ARCS VIEW (from Neo4j) ===
    if let Some(ref arcs) = app.kind_arcs {
        // Hierarchy breadcrumb with theme colors
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &arcs.realm,
                Style::default()
                    .fg(theme.realm_color(&arcs.realm))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Realm)", STYLE_DIM),
            Span::styled(" \u{2192} ", bright_dim),
            Span::styled(
                &arcs.layer,
                Style::default()
                    .fg(theme.layer_color(&arcs.layer))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Layer)", STYLE_DIM),
            Span::styled(" \u{2192} ", bright_dim),
            Span::styled(
                &arcs.kind_label,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Node Kind)", STYLE_DIM),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Group all arcs by family
        render_arcs_by_family(&mut lines, arcs, theme, &dim);

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === ARCKIND DETAILS VIEW ===
    if let Some(ref details) = app.arc_kind_details {
        let family_color = theme.arc_family_color(&details.family);

        // Arc name with family
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &details.display_name,
                Style::default()
                    .fg(family_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("  ({})", details.family), STYLE_DIM),
        ]));
        lines.push(Line::from(Span::raw("")));

        // === ENDPOINTS ===
        lines.push(Line::from(Span::styled(
            "  ENDPOINTS",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            dim,
        )));

        // FROM endpoint with theme colors
        if let Some(ref from) = details.from_endpoint {
            lines.push(Line::from(vec![
                Span::styled("    FROM: ", STYLE_ACCENT),
                Span::styled(
                    &from.kind_label,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  (", STYLE_DIM),
                Span::styled(
                    &from.realm,
                    Style::default().fg(theme.realm_color(&from.realm)),
                ),
                Span::styled("/", STYLE_DIM),
                Span::styled(
                    &from.layer,
                    Style::default().fg(theme.layer_color(&from.layer)),
                ),
                Span::styled(")", STYLE_DIM),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("    FROM: ", STYLE_ACCENT),
                Span::styled("(not defined)", STYLE_DIM),
            ]));
        }

        // TO endpoint with theme colors
        if let Some(ref to) = details.to_endpoint {
            lines.push(Line::from(vec![
                Span::styled("    TO:   ", STYLE_INFO),
                Span::styled(
                    &to.kind_label,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  (", STYLE_DIM),
                Span::styled(&to.realm, Style::default().fg(theme.realm_color(&to.realm))),
                Span::styled("/", STYLE_DIM),
                Span::styled(&to.layer, Style::default().fg(theme.layer_color(&to.layer))),
                Span::styled(")", STYLE_DIM),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("    TO:   ", STYLE_INFO),
                Span::styled("(not defined)", STYLE_DIM),
            ]));
        }
        lines.push(Line::from(Span::raw("")));

        // === PROPERTIES ===
        lines.push(Line::from(Span::styled(
            "  PROPERTIES",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            dim,
        )));

        // Cardinality
        if !details.cardinality.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("    Cardinality: ", dim),
                Span::styled(&details.cardinality, STYLE_HIGHLIGHT),
            ]));
        }

        // Cypher pattern
        if !details.cypher_pattern.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("    Pattern: ", dim),
                Span::styled(
                    &details.cypher_pattern,
                    Style::default().fg(theme.arc_family_color("generation")),
                ),
            ]));
        }

        // Description
        if !details.description.is_empty() {
            lines.push(Line::from(Span::raw("")));
            lines.push(Line::from(Span::styled(
                "  DESCRIPTION",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
                dim,
            )));
            // Wrap description if too long (no Vec<char> allocation)
            for line in wrap_text(&details.description, 45) {
                lines.push(Line::from(Span::styled(format!("    {}", line), STYLE_DIM)));
            }
        }
    } else {
        // No Neo4j data - show realm/layer distribution stats
        lines.extend(build_graph_distribution_stats(app));

        // Add contextual hint at the bottom
        lines.push(Line::from(Span::raw("")));
        let hint = match app.current_item() {
            Some(TreeItem::KindsSection) | Some(TreeItem::ArcsSection) => {
                "\u{25b8} Expand a section to explore"
            }
            Some(TreeItem::ArcFamily(_)) => "\u{25b8} Select an Arc to see endpoints",
            Some(TreeItem::ArcKind(_, _)) => "\u{25b8} Loading arc details...",
            Some(TreeItem::Realm(_)) | Some(TreeItem::Layer(_, _)) => {
                "\u{25b8} Select a Node Kind to see arc relationships"
            }
            _ => "\u{25b8} Select a Node Kind or Arc to see details",
        };
        lines.push(Line::from(Span::styled(hint, STYLE_DIM)));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

/// Render arcs grouped by family (instead of by direction).
/// Optimized to use references instead of cloning strings.
fn render_arcs_by_family(
    lines: &mut Vec<Line>,
    arcs: &super::super::data::KindArcsData,
    theme: &theme::Theme,
    dim: &Style,
) {
    // Collect all arcs grouped by family using references (no cloning)
    let mut by_family: BTreeMap<&str, Vec<(bool, &str, &str)>> = BTreeMap::new();

    for arc in &arcs.incoming {
        by_family
            .entry(&arc.family)
            .or_default()
            .push((false, &arc.arc_key, &arc.other_kind)); // false = incoming
    }
    for arc in &arcs.outgoing {
        by_family
            .entry(&arc.family)
            .or_default()
            .push((true, &arc.arc_key, &arc.other_kind)); // true = outgoing
    }

    if by_family.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No arc relationships defined for this Node Kind",
            STYLE_DIM,
        )));
        return;
    }

    let kind_label = &arcs.kind_label;

    // Render each family group
    for (family, family_arcs) in &by_family {
        let family_color = theme.arc_family_color(family);
        let incoming_count = family_arcs.iter().filter(|(is_out, _, _)| !is_out).count();
        let outgoing_count = family_arcs.iter().filter(|(is_out, _, _)| *is_out).count();

        // Family header with counts
        lines.push(Line::from(vec![
            Span::styled("  ", *dim),
            Span::styled(
                family.to_uppercase(),
                Style::default()
                    .fg(family_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  (\u{25c0}{} \u{25b6}{})", incoming_count, outgoing_count),
                STYLE_DIM,
            ),
        ]));
        lines.push(Line::from(Span::styled(
            "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            *dim,
        )));

        // Render arcs in this family (convert &str to owned String only for Span)
        for (is_outgoing, arc_key, other_kind) in family_arcs {
            if *is_outgoing {
                // Outgoing: Kind --[ARC]---> Target
                lines.push(Line::from(vec![
                    Span::styled("    ", *dim),
                    Span::styled(kind_label.to_string(), STYLE_PRIMARY),
                    Span::styled(" \u{2500}\u{2500}[", *dim),
                    Span::styled(
                        (*arc_key).to_string(),
                        Style::default()
                            .fg(family_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("]\u{2500}\u{2500}\u{25b6} ", *dim),
                    Span::styled((*other_kind).to_string(), STYLE_SUCCESS),
                ]));
            } else {
                // Incoming: Source --[ARC]---> Kind
                lines.push(Line::from(vec![
                    Span::styled("    ", *dim),
                    Span::styled((*other_kind).to_string(), STYLE_SUCCESS),
                    Span::styled(" \u{2500}\u{2500}[", *dim),
                    Span::styled(
                        (*arc_key).to_string(),
                        Style::default()
                            .fg(family_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("]\u{2500}\u{2500}\u{25b6} ", *dim),
                    Span::styled(kind_label.to_string(), STYLE_PRIMARY),
                ]));
            }
        }
        lines.push(Line::from(Span::raw("")));
    }
}

/// Build realm/layer distribution stats for the graph panel fallback view.
fn build_graph_distribution_stats(app: &App) -> Vec<Line<'static>> {
    let theme = &app.theme;
    let dim = Style::default().fg(Color::Rgb(100, 100, 100));
    let mut lines: Vec<Line<'static>> = Vec::with_capacity(20);

    // Calculate total kinds
    let mut total_kinds: usize = 0;
    for realm in &app.tree.realms {
        for layer in &realm.layers {
            total_kinds += layer.kinds.len();
        }
    }

    if total_kinds == 0 {
        lines.push(Line::from(Span::styled("  No kinds loaded", STYLE_DIM)));
        return lines;
    }

    // Header
    lines.push(Line::from(Span::styled(
        "  REALM DISTRIBUTION",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
        dim,
    )));

    let bar_max_width = 20usize;

    // Realm bars
    for realm in &app.tree.realms {
        let realm_kinds: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
        let percent = (realm_kinds as f64 / total_kinds as f64 * 100.0).round() as u8;
        let bar_width = (realm_kinds * bar_max_width) / total_kinds.max(1);
        let bar = "\u{2588}".repeat(bar_width.max(1));
        let empty = "\u{2591}".repeat(bar_max_width.saturating_sub(bar_width));

        lines.push(Line::from(vec![
            Span::styled("    ", dim),
            Span::styled(
                format!("{:8}", realm.display_name),
                Style::default()
                    .fg(theme.realm_color(&realm.key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", dim),
            Span::styled(bar, Style::default().fg(theme.realm_color(&realm.key))),
            Span::styled(empty, STYLE_DIM),
            Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
            Span::styled(format!("  {} Kinds", realm_kinds), STYLE_DIM),
        ]));
    }

    lines.push(Line::from(Span::raw("")));

    // Layer breakdown header
    lines.push(Line::from(Span::styled(
        "  LAYER BREAKDOWN",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
        dim,
    )));

    // Find max kinds per layer for scaling
    let max_layer_kinds = app
        .tree
        .realms
        .iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.kinds.len())
        .max()
        .unwrap_or(1)
        .max(1);

    // Layer bars (grouped by realm)
    for realm in &app.tree.realms {
        for layer in &realm.layers {
            let layer_kinds = layer.kinds.len();
            if layer_kinds == 0 {
                continue; // Skip empty layers
            }
            let bar_width = (layer_kinds * bar_max_width) / max_layer_kinds;
            let bar = "\u{2588}".repeat(bar_width.max(1));

            lines.push(Line::from(vec![
                Span::styled("    ", dim),
                Span::styled(
                    format!("{:16}", layer.display_name),
                    Style::default().fg(theme.layer_color(&layer.key)),
                ),
                Span::styled(format!("{:>3} ", layer_kinds), STYLE_MUTED),
                Span::styled(bar, Style::default().fg(theme.layer_color(&layer.key))),
            ]));
        }
    }

    lines
}
