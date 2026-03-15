//! View rendering functions for the graph panel.
//!
//! Each function renders a specific view state (realm details, layer details,
//! instance arcs, class arcs, arc class details, or fallback stats).

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::super::super::app::App;
use super::super::super::data::TreeItem;
use super::super::{
    STYLE_ACCENT, STYLE_BRIGHT_DIM, STYLE_DIM, STYLE_HIGHLIGHT, STYLE_INFO, STYLE_PRIMARY,
    STYLE_SUCCESS, spinner, wrap_text,
};
use super::helpers::render_arcs_by_direction;
use super::render_with_scroll;
use super::stats::build_graph_distribution_stats;
use super::ARC_SEPARATOR;
use crate::tui::palette;

/// Render the loading indicator when async data is pending.
/// Returns true if a loading state was rendered.
pub(super) fn render_loading(
    f: &mut Frame,
    inner: Rect,
    app: &App,
) -> bool {
    let loading_msg = if app.pending.arcs.is_some() {
        Some("Loading arc relationships...")
    } else if app.pending.arc_class.is_some() {
        Some("Loading arc class details...")
    } else if app.pending.realm.is_some() {
        Some("Loading realm statistics...")
    } else if app.pending.layer.is_some() {
        Some("Loading layer statistics...")
    } else {
        None
    };

    if let Some(msg) = loading_msg {
        let lines = vec![Line::from(Span::styled(
            format!("  {} {}", spinner(app.tick), msg),
            STYLE_HIGHLIGHT,
        ))];
        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        true
    } else {
        false
    }
}

/// Render realm details view (layers, stats, outgoing HAS_LAYER arcs).
pub(super) fn render_realm_details(
    f: &mut Frame,
    inner: Rect,
    app: &mut App,
) {
    let theme = &app.theme;
    let details = app.details.realm.as_ref().unwrap();
    let dim = Style::default().fg(palette::DIM);
    let mut lines: Vec<Line> = Vec::new();

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
            Style::default().fg(palette::COUNT_TEXT),
        )));
        lines.push(Line::from(Span::raw("")));
    }

    // Stats summary
    lines.push(Line::from(vec![
        Span::styled("  \u{25aa}", dim),
        Span::styled(format!("{} Layers", details.layers.len()), STYLE_INFO),
        Span::styled(" \u{b7} ", dim),
        Span::styled(
            format!("{} Node Classes", details.total_classes),
            STYLE_SUCCESS,
        ),
        Span::styled(" \u{b7} ", dim),
        Span::styled(
            format!("{} Instances", details.total_instances),
            STYLE_HIGHLIGHT,
        ),
    ]));
    lines.push(Line::from(Span::raw("")));

    // === OUTGOING ARCS (HAS_LAYER) ===
    let ownership_color = theme.arc_family_color("ownership");
    if !details.layers.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  ━▶ OUTGOING ({}) ", details.layers.len()),
            Style::default()
                .fg(ownership_color)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(ARC_SEPARATOR, dim)));

        for layer in &details.layers {
            let layer_color = theme.layer_color(&layer.key);
            lines.push(Line::from(vec![
                Span::styled(
                    "    → ",
                    Style::default()
                        .fg(ownership_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("HAS_LAYER", Style::default().fg(ownership_color)),
                Span::styled(" → ", dim),
                Span::styled(&layer.display_name, Style::default().fg(layer_color)),
                Span::styled(format!(" ({} classes)", layer.class_count), STYLE_BRIGHT_DIM),
                Span::styled(" [own]", dim),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "  No outgoing arcs — this Realm has no layers",
            STYLE_DIM,
        )));
    }

    app.arcs_line_count = render_with_scroll(f, inner, lines, app.arcs_scroll);
}

/// Render layer details view (classes, incoming/outgoing arcs).
pub(super) fn render_layer_details(
    f: &mut Frame,
    inner: Rect,
    app: &mut App,
) {
    let theme = &app.theme;
    let details = app.details.layer.as_ref().unwrap();
    let dim = Style::default().fg(palette::DIM);
    let bright_dim = STYLE_BRIGHT_DIM;
    let mut lines: Vec<Line> = Vec::new();

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
            Style::default().fg(palette::COUNT_TEXT),
        )));
        lines.push(Line::from(Span::raw("")));
    }

    // Stats summary
    lines.push(Line::from(vec![
        Span::styled("  \u{25aa}", dim),
        Span::styled(
            format!("{} Node Classes", details.total_classes),
            STYLE_SUCCESS,
        ),
        Span::styled(" \u{b7} ", dim),
        Span::styled(
            format!("{} Instances", details.total_instances),
            STYLE_HIGHLIGHT,
        ),
    ]));
    lines.push(Line::from(Span::raw("")));

    // === INCOMING ARCS (HAS_LAYER from Realm) ===
    let incoming_count = 1;
    let ownership_color = theme.arc_family_color("ownership");
    lines.push(Line::from(Span::styled(
        format!("  ◀━ INCOMING ({}) ", incoming_count),
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(ARC_SEPARATOR, dim)));
    lines.push(Line::from(vec![
        Span::styled(
            "    ← ",
            Style::default()
                .fg(ownership_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            &details.realm,
            Style::default().fg(theme.realm_color(&details.realm)),
        ),
        Span::styled(" ← ", dim),
        Span::styled("HAS_LAYER", Style::default().fg(ownership_color)),
        Span::styled(" [own]", dim),
    ]));
    lines.push(Line::from(Span::raw("")));

    // === OUTGOING ARCS (Classes in this layer) ===
    let total_classes = details.class_names.len();

    if total_classes > 0 {
        lines.push(Line::from(Span::styled(
            format!("  ━▶ OUTGOING ({}) ", total_classes),
            Style::default()
                .fg(ownership_color)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(ARC_SEPARATOR, dim)));

        for class_name in &details.class_names {
            lines.push(Line::from(vec![
                Span::styled(
                    "    → ",
                    Style::default()
                        .fg(ownership_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("HAS_CLASS", Style::default().fg(ownership_color)),
                Span::styled(" → ", dim),
                Span::styled(
                    class_name,
                    Style::default().fg(theme.layer_color(&details.key)),
                ),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "  No outgoing arcs — this Layer has no classes",
            STYLE_DIM,
        )));
    }

    app.arcs_line_count = render_with_scroll(f, inner, lines, app.arcs_scroll);
}

/// Render instance arcs view (Data mode - outgoing/incoming arcs for an instance).
pub(super) fn render_instance_arcs(
    f: &mut Frame,
    inner: Rect,
    app: &mut App,
) {
    let theme = &app.theme;
    let dim = Style::default().fg(palette::DIM);
    let bright_dim = STYLE_BRIGHT_DIM;
    let mut lines: Vec<Line> = Vec::new();

    // Re-extract the instance data (we know it matches from the caller)
    let (realm, layer, class_info, instance) = match app.current_item() {
        Some(TreeItem::Instance(r, l, c, i)) => (r, l, c, i),
        _ => return,
    };

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
        Span::styled(class_info.display_name.clone(), STYLE_SUCCESS),
        Span::styled(" \u{2192} ", bright_dim),
        Span::styled(
            instance_key.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(Span::raw("")));

    // Check loading state first
    if instance.arcs_loading {
        lines.push(Line::from(Span::styled(
            "  Loading arc connections...",
            Style::default().fg(Color::Yellow),
        )));
    } else {
        let total = instance.outgoing_arcs.len() + instance.incoming_arcs.len();

        if total == 0 {
            lines.push(Line::from(Span::styled(
                "  No arc connections for this instance",
                STYLE_DIM,
            )));
        } else {
            // Outgoing arcs
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
                        Span::styled(format!(" ({})", arc.target_class), STYLE_DIM),
                    ]));
                }
                lines.push(Line::from(Span::raw("")));
            }

            // Incoming arcs
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
                        Span::styled(format!(" ({})", arc.target_class), STYLE_DIM),
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
    }

    app.arcs_line_count = render_with_scroll(f, inner, lines, app.arcs_scroll);
}

/// Render class arcs view (schema arcs for a NodeClass from Neo4j).
pub(super) fn render_class_arcs(
    f: &mut Frame,
    inner: Rect,
    app: &mut App,
) {
    let theme = &app.theme;
    let arcs = app.details.class_arcs.as_ref().unwrap();
    let dim = Style::default().fg(palette::DIM);
    let bright_dim = STYLE_BRIGHT_DIM;
    let mut lines: Vec<Line> = Vec::new();

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
            &arcs.class_label,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" (Node Class)", STYLE_DIM),
    ]));
    lines.push(Line::from(Span::raw("")));

    // Group arcs by direction with classification badges
    render_arcs_by_direction(&mut lines, arcs, app, theme, &dim);

    app.arcs_line_count = render_with_scroll(f, inner, lines, app.arcs_scroll);
}

/// Render arc class details view (endpoints, properties, description).
pub(super) fn render_arc_class_details(
    f: &mut Frame,
    inner: Rect,
    app: &mut App,
) {
    let theme = &app.theme;
    let details = app.details.arc_class.as_ref().unwrap();
    let dim = Style::default().fg(palette::DIM);
    let mut lines: Vec<Line> = Vec::new();

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
                &from.class_label,
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
                &to.class_label,
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
        for line in wrap_text(&details.description, 45) {
            lines.push(Line::from(Span::styled(format!("    {}", line), STYLE_DIM)));
        }
    }

    app.arcs_line_count = render_with_scroll(f, inner, lines, app.arcs_scroll);
}

/// Render fallback view (distribution stats + contextual hint).
pub(super) fn render_fallback(
    f: &mut Frame,
    inner: Rect,
    app: &App,
) {
    let mut lines: Vec<Line> = Vec::new();

    lines.extend(build_graph_distribution_stats(app));

    // Add contextual hint at the bottom
    lines.push(Line::from(Span::raw("")));
    let hint = match app.current_item() {
        Some(TreeItem::ClassesSection) | Some(TreeItem::ArcsSection) => {
            "\u{25b8} Expand a section to explore"
        },
        Some(TreeItem::ArcFamily(_)) => "\u{25b8} Select an Arc to see endpoints",
        Some(TreeItem::ArcClass(_, _)) => "\u{25b8} Loading arc details...",
        Some(TreeItem::Realm(_)) | Some(TreeItem::Layer(_, _)) => {
            "\u{25b8} Select a Node Class to see arc relationships"
        },
        _ => "\u{25b8} Select a Node Class or Arc to see details",
    };
    lines.push(Line::from(Span::styled(hint, STYLE_DIM)));

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}
