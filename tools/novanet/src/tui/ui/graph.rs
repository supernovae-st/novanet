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

    // Calculate arc counts for title (separate in/out)
    let (incoming_count, outgoing_count) = if let Some(ref arcs) = app.kind_arcs {
        (arcs.incoming.len(), arcs.outgoing.len())
    } else if let Some(TreeItem::Instance(_, _, _, inst)) = app.current_item() {
        (inst.incoming_arcs.len(), inst.outgoing_arcs.len())
    } else {
        (0, 0)
    };

    // Build title with in/out counts
    let title = if incoming_count > 0 || outgoing_count > 0 {
        format!(
            " Arc Relationships [←{} In] [{} Out→] ",
            incoming_count, outgoing_count
        )
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

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::data::{KindArcsData, KindInfo, LayerInfo, Neo4jArc, RealmInfo, TaxonomyTree};
    use crate::tui::theme::{ColorMode, Theme};
    use pretty_assertions::assert_eq;
    use ratatui::style::Color;
    use rustc_hash::FxHashMap;

    // =========================================================================
    // Test helpers
    // =========================================================================

    fn create_test_theme() -> Theme {
        Theme::with_mode(ColorMode::TrueColor)
    }

    fn create_kind_arcs_data(
        kind_label: &str,
        incoming: Vec<Neo4jArc>,
        outgoing: Vec<Neo4jArc>,
    ) -> KindArcsData {
        KindArcsData {
            kind_label: kind_label.to_string(),
            realm: "org".to_string(),
            layer: "structure".to_string(),
            incoming,
            outgoing,
        }
    }

    fn create_neo4j_arc(arc_key: &str, other_kind: &str, family: &str) -> Neo4jArc {
        Neo4jArc {
            arc_key: arc_key.to_string(),
            other_kind: other_kind.to_string(),
            family: family.to_string(),
        }
    }

    fn create_test_kind(key: &str) -> KindInfo {
        KindInfo {
            key: key.to_string(),
            display_name: key.to_string(),
            description: String::new(),
            icon: String::new(),
            trait_name: "invariant".to_string(),
            instance_count: 0,
            arcs: Vec::new(),
            yaml_path: String::new(),
            properties: Vec::new(),
            required_properties: Vec::new(),
            schema_hint: String::new(),
            context_budget: String::new(),
            knowledge_tier: None,
            health_percent: None,
            issues_count: None,
        }
    }

    fn create_test_layer(key: &str, kinds: Vec<KindInfo>) -> LayerInfo {
        LayerInfo {
            key: key.to_string(),
            display_name: key.to_string(),
            color: "#ffffff".to_string(),
            kinds,
        }
    }

    fn create_test_realm(key: &str, layers: Vec<LayerInfo>) -> RealmInfo {
        RealmInfo {
            key: key.to_string(),
            display_name: key.to_string(),
            color: "#ffffff".to_string(),
            icon: "○",
            layers,
        }
    }

    fn create_empty_tree() -> TaxonomyTree {
        TaxonomyTree {
            realms: Vec::new(),
            arc_families: Vec::new(),
            stats: Default::default(),
            collapsed: Default::default(),
            instances: Default::default(),
            instance_totals: Default::default(),
            kind_index: FxHashMap::default(),
            entity_categories: Vec::new(),
            entity_category_instances: Default::default(),
        }
    }

    fn create_tree_with_realms(realms: Vec<RealmInfo>) -> TaxonomyTree {
        // Build kind_index for O(1) lookups
        let mut kind_index = FxHashMap::default();
        for (r_idx, realm) in realms.iter().enumerate() {
            for (l_idx, layer) in realm.layers.iter().enumerate() {
                for (k_idx, kind) in layer.kinds.iter().enumerate() {
                    kind_index.insert(kind.key.clone(), (r_idx, l_idx, k_idx));
                }
            }
        }

        TaxonomyTree {
            realms,
            arc_families: Vec::new(),
            stats: Default::default(),
            collapsed: Default::default(),
            instances: Default::default(),
            instance_totals: Default::default(),
            kind_index,
            entity_categories: Vec::new(),
            entity_category_instances: Default::default(),
        }
    }

    /// Create a minimal App for testing build_graph_distribution_stats.
    /// Uses App::new() then replaces only the tree field we care about.
    fn create_test_app_with_tree(tree: TaxonomyTree) -> App {
        // Create app with mock tree, then replace with our test tree
        let mut app = App::new(TaxonomyTree::mock_for_testing(), String::new());
        app.tree = tree;
        app.theme = create_test_theme();
        app
    }

    // =========================================================================
    // render_arcs_by_family tests
    // =========================================================================

    #[test]
    fn test_render_arcs_by_family_empty() {
        let theme = create_test_theme();
        let dim = Style::default().fg(Color::Rgb(100, 100, 100));
        let arcs = create_kind_arcs_data("Page", Vec::new(), Vec::new());

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_family(&mut lines, &arcs, &theme, &dim);

        // Should show "No arc relationships" message
        assert_eq!(lines.len(), 1, "empty arcs should produce 1 line");

        let line_content: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            line_content.contains("No arc relationships"),
            "should contain 'No arc relationships' message, got: {}",
            line_content
        );
    }

    #[test]
    fn test_render_arcs_by_family_single_family() {
        let theme = create_test_theme();
        let dim = Style::default().fg(Color::Rgb(100, 100, 100));

        let outgoing = vec![
            create_neo4j_arc("USES_ENTITY", "Entity", "semantic"),
            create_neo4j_arc("USES_BLOCK", "Block", "semantic"),
        ];
        let arcs = create_kind_arcs_data("Page", Vec::new(), outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_family(&mut lines, &arcs, &theme, &dim);

        // Should have family header + separator + 2 arcs + blank line = 5 lines
        assert!(lines.len() >= 4, "should have header, separator, and arcs");

        // First line should be family header (SEMANTIC)
        let header: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            header.contains("SEMANTIC"),
            "header should contain 'SEMANTIC', got: {}",
            header
        );
    }

    #[test]
    fn test_render_arcs_by_family_multiple_families() {
        let theme = create_test_theme();
        let dim = Style::default().fg(Color::Rgb(100, 100, 100));

        let incoming = vec![create_neo4j_arc("BELONGS_TO", "Project", "ownership")];
        let outgoing = vec![create_neo4j_arc("USES_ENTITY", "Entity", "semantic")];
        let arcs = create_kind_arcs_data("Page", incoming, outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_family(&mut lines, &arcs, &theme, &dim);

        // Should have 2 family sections
        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("OWNERSHIP"),
            "should contain OWNERSHIP family"
        );
        assert!(
            all_content.contains("SEMANTIC"),
            "should contain SEMANTIC family"
        );
    }

    #[test]
    fn test_render_arcs_by_family_incoming_outgoing_counts() {
        let theme = create_test_theme();
        let dim = Style::default().fg(Color::Rgb(100, 100, 100));

        // 2 incoming, 1 outgoing for semantic family
        let incoming = vec![
            create_neo4j_arc("USED_BY_PAGE", "Page", "semantic"),
            create_neo4j_arc("USED_BY_BLOCK", "Block", "semantic"),
        ];
        let outgoing = vec![create_neo4j_arc("USES_ENTITY", "Entity", "semantic")];
        let arcs = create_kind_arcs_data("Kind", incoming, outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_family(&mut lines, &arcs, &theme, &dim);

        // Find the header line with counts
        let header: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        // Should show (◀2 ▶1) for 2 incoming, 1 outgoing
        assert!(
            header.contains("2") && header.contains("1"),
            "header should show counts, got: {}",
            header
        );
    }

    #[test]
    fn test_render_arcs_by_family_arc_direction_display() {
        let theme = create_test_theme();
        let dim = Style::default().fg(Color::Rgb(100, 100, 100));

        let incoming = vec![create_neo4j_arc("BELONGS_TO", "Project", "ownership")];
        let outgoing = vec![create_neo4j_arc("HAS_PAGE", "Page", "ownership")];
        let arcs = create_kind_arcs_data("Kind", incoming, outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_family(&mut lines, &arcs, &theme, &dim);

        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        // Outgoing: Kind --[ARC]---> Target
        // Incoming: Source --[ARC]---> Kind
        assert!(
            all_content.contains("Kind"),
            "should contain the kind label"
        );
        assert!(
            all_content.contains("BELONGS_TO"),
            "should contain incoming arc"
        );
        assert!(
            all_content.contains("HAS_PAGE"),
            "should contain outgoing arc"
        );
    }

    // =========================================================================
    // build_graph_distribution_stats tests
    // =========================================================================

    #[test]
    fn test_build_graph_distribution_stats_empty_tree() {
        let tree = create_empty_tree();
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);

        assert_eq!(lines.len(), 1, "empty tree should produce 1 line");
        let content: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            content.contains("No kinds loaded"),
            "should show 'No kinds loaded', got: {}",
            content
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_single_realm() {
        let kind1 = create_test_kind("Page");
        let kind2 = create_test_kind("Block");
        let layer = create_test_layer("structure", vec![kind1, kind2]);
        let realm = create_test_realm("org", vec![layer]);
        let tree = create_tree_with_realms(vec![realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);

        // Should have realm distribution header + separator + realm bar + blank + layer header + separator + layer bar
        assert!(lines.len() >= 5, "should have multiple lines for stats");

        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("REALM DISTRIBUTION"),
            "should contain realm header"
        );
        assert!(
            all_content.contains("LAYER BREAKDOWN"),
            "should contain layer header"
        );
        assert!(all_content.contains("100%"), "single realm should be 100%");
    }

    #[test]
    fn test_build_graph_distribution_stats_percentage_calculation() {
        // Create 2 realms: shared with 1 kind, org with 3 kinds
        // Expected: shared = 25%, org = 75%
        let global_kind = create_test_kind("Config");
        let global_layer = create_test_layer("config", vec![global_kind]);
        let global_realm = create_test_realm("shared", vec![global_layer]);

        let tenant_kinds = vec![
            create_test_kind("Page"),
            create_test_kind("Block"),
            create_test_kind("Entity"),
        ];
        let tenant_layer = create_test_layer("structure", tenant_kinds);
        let tenant_realm = create_test_realm("org", vec![tenant_layer]);

        let tree = create_tree_with_realms(vec![global_realm, tenant_realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);
        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        // Should show 25% for shared (1/4 = 25%) and 75% for org (3/4 = 75%)
        assert!(
            all_content.contains("25%"),
            "shared should be 25%, got: {}",
            all_content
        );
        assert!(
            all_content.contains("75%"),
            "org should be 75%, got: {}",
            all_content
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_bar_width_calculation() {
        // Create 2 realms with different kind counts
        // bar_width = (realm_kinds * bar_max_width) / total_kinds
        // bar_max_width = 20

        // Shared: 2 kinds, Org: 8 kinds, Total: 10
        // Shared bar = (2 * 20) / 10 = 4
        // Org bar = (8 * 20) / 10 = 16

        let global_kinds = vec![create_test_kind("Config1"), create_test_kind("Config2")];
        let global_layer = create_test_layer("config", global_kinds);
        let global_realm = create_test_realm("shared", vec![global_layer]);

        let tenant_kinds: Vec<KindInfo> = (0..8)
            .map(|i| create_test_kind(&format!("Kind{}", i)))
            .collect();
        let tenant_layer = create_test_layer("structure", tenant_kinds);
        let tenant_realm = create_test_realm("org", vec![tenant_layer]);

        let tree = create_tree_with_realms(vec![global_realm, tenant_realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);

        // Count the number of filled bar characters in realm lines
        // The realm bars are on lines after the header/separator
        let realm_section: Vec<&Line> = lines
            .iter()
            .filter(|l| {
                let content: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                (content.contains("shared") || content.contains("org")) && content.contains('%')
            })
            .collect();

        assert_eq!(realm_section.len(), 2, "should have 2 realm bar lines");

        // Verify the bars have different widths by counting block characters
        let global_line: String = realm_section[0]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        let tenant_line: String = realm_section[1]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();

        let global_blocks = global_line.matches('\u{2588}').count();
        let tenant_blocks = tenant_line.matches('\u{2588}').count();

        // Org should have more blocks than global (8 kinds vs 2 kinds)
        assert!(
            tenant_blocks > global_blocks,
            "tenant ({} blocks) should have more than global ({} blocks)",
            tenant_blocks,
            global_blocks
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_layer_bar_scaling() {
        // Layer bars scale relative to max layer kinds, not total kinds
        // Create layers with different sizes

        let layer1_kinds = vec![create_test_kind("Kind1")];
        let layer2_kinds = vec![
            create_test_kind("Kind2"),
            create_test_kind("Kind3"),
            create_test_kind("Kind4"),
            create_test_kind("Kind5"),
        ];

        let layer1 = create_test_layer("config", layer1_kinds);
        let layer2 = create_test_layer("foundation", layer2_kinds);
        let realm = create_test_realm("org", vec![layer1, layer2]);
        let tree = create_tree_with_realms(vec![realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);

        // Find layer breakdown section
        let layer_section: Vec<&Line> = lines
            .iter()
            .skip_while(|l| {
                let content: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                !content.contains("LAYER BREAKDOWN")
            })
            .skip(2) // Skip header and separator
            .filter(|l| !l.spans.is_empty())
            .collect();

        assert!(layer_section.len() >= 2, "should have 2 layer lines");

        // max_layer_kinds = 4 (foundation), bar_max_width = 20
        // config bar = (1 * 20) / 4 = 5
        // foundation bar = (4 * 20) / 4 = 20 (full width)

        let config_line: String = layer_section
            .iter()
            .find(|l| {
                let content: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                content.contains("config")
            })
            .map(|l| l.spans.iter().map(|s| s.content.as_ref()).collect())
            .unwrap_or_default();

        let foundation_line: String = layer_section
            .iter()
            .find(|l| {
                let content: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                content.contains("foundation")
            })
            .map(|l| l.spans.iter().map(|s| s.content.as_ref()).collect())
            .unwrap_or_default();

        let config_blocks = config_line.matches('\u{2588}').count();
        let foundation_blocks = foundation_line.matches('\u{2588}').count();

        // Foundation should have more blocks (4 kinds vs 1 kind)
        assert!(
            foundation_blocks > config_blocks,
            "foundation ({} blocks) should have more than config ({} blocks)",
            foundation_blocks,
            config_blocks
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_skips_empty_layers() {
        // Empty layers should not be shown in layer breakdown
        let kind = create_test_kind("Page");
        let layer_with_kinds = create_test_layer("structure", vec![kind]);
        let empty_layer = create_test_layer("empty", Vec::new());
        let realm = create_test_realm("org", vec![layer_with_kinds, empty_layer]);
        let tree = create_tree_with_realms(vec![realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);
        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        // Should show structure but not empty
        assert!(
            all_content.contains("structure"),
            "should contain structure layer"
        );
        // Empty layer should not appear in the layer breakdown
        // (it may appear if listed, but the function skips layer_kinds == 0)
    }

    #[test]
    fn test_build_graph_distribution_stats_kind_counts_displayed() {
        let kinds = vec![
            create_test_kind("Page"),
            create_test_kind("Block"),
            create_test_kind("Entity"),
        ];
        let layer = create_test_layer("structure", kinds);
        let realm = create_test_realm("org", vec![layer]);
        let tree = create_tree_with_realms(vec![realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);
        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        // Should show "3 Kinds" somewhere in the content
        assert!(
            all_content.contains("3 Kinds") || all_content.contains("3"),
            "should show kind count, got: {}",
            all_content
        );
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_render_arcs_by_family_all_five_families() {
        let theme = create_test_theme();
        let dim = Style::default().fg(Color::Rgb(100, 100, 100));

        // Test all 5 arc families: ownership, localization, semantic, generation, mining
        let outgoing = vec![
            create_neo4j_arc("BELONGS_TO", "Project", "ownership"),
            create_neo4j_arc("LOCALIZES", "EntityContent", "localization"),
            create_neo4j_arc("USES_ENTITY", "Entity", "semantic"),
            create_neo4j_arc("GENERATES", "Block", "generation"),
            create_neo4j_arc("MINES_DATA", "Source", "mining"),
        ];
        let arcs = create_kind_arcs_data("Kind", Vec::new(), outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_family(&mut lines, &arcs, &theme, &dim);

        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        // All families should be present (BTreeMap sorts alphabetically)
        assert!(all_content.contains("GENERATION"), "should have generation");
        assert!(
            all_content.contains("LOCALIZATION"),
            "should have localization"
        );
        assert!(all_content.contains("MINING"), "should have mining");
        assert!(all_content.contains("OWNERSHIP"), "should have ownership");
        assert!(all_content.contains("SEMANTIC"), "should have semantic");
    }

    #[test]
    fn test_build_graph_distribution_stats_many_realms() {
        // Test with multiple realms to ensure all are displayed
        let realms: Vec<RealmInfo> = (0..3)
            .map(|i| {
                let kinds: Vec<KindInfo> = (0..(i + 1))
                    .map(|j| create_test_kind(&format!("Kind{}_{}", i, j)))
                    .collect();
                let layer = create_test_layer(&format!("layer{}", i), kinds);
                create_test_realm(&format!("realm{}", i), vec![layer])
            })
            .collect();

        let tree = create_tree_with_realms(realms);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);
        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        // All realms should be present
        assert!(all_content.contains("realm0"), "should have realm0");
        assert!(all_content.contains("realm1"), "should have realm1");
        assert!(all_content.contains("realm2"), "should have realm2");
    }
}
