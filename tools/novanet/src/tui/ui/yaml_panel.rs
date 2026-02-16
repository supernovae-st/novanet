//! YAML panel rendering for TUI.
//!
//! This module handles the YAML preview panel with:
//! - Syntax highlighting for keys, values, comments, and punctuation
//! - Contextual view with Class/Instance sections
//! - Peek mode to preview hidden sections
//! - Scrollbar for long content
//! - v0.13.0: Split into SOURCE and DIAGRAM boxes with visual states

use ratatui::Frame;
use ratatui::layout::{Layout, Constraint, Direction, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use super::{
    COLOR_HINT_TEXT, COLOR_MUTED_TEXT, STYLE_DIM, STYLE_UNFOCUSED, colorize_path_inline,
    scroll_indicator,
};
use crate::tui::app::{App, InfoBox};
use crate::tui::yaml::YamlViewSection;

// =============================================================================
// BOX VISUAL STATES v0.13 (enhanced palette)
// =============================================================================

/// Unfocused: Nord Polar Night (dim) - box is NOT selected
const BOX_BORDER_UNFOCUSED: Color = Color::Rgb(59, 66, 82); // #3B4252

/// Selected: Solarized Cyan (bright, active) - this specific box is Tab-selected
const BOX_BORDER_SELECTED: Color = Color::Rgb(42, 161, 152); // #2AA198

// =============================================================================
// v0.13 SEMANTIC COLORS
// =============================================================================

/// Realm colors
const COLOR_REALM_SHARED: Color = Color::Rgb(42, 161, 152); // #2AA198 Solarized Cyan
const COLOR_REALM_ORG: Color = Color::Rgb(108, 113, 196); // #6C71C4 Solarized Violet

/// Layer colors (subset)
const COLOR_LAYER_SEMANTIC: Color = Color::Rgb(249, 115, 22); // #F97316 Orange
const COLOR_LAYER_OUTPUT: Color = Color::Rgb(34, 197, 94); // #22C55E Green
const COLOR_LAYER_KNOWLEDGE: Color = Color::Rgb(139, 92, 246); // #8B5CF6 Violet

/// Trait colors (ADR-024 Data Origin)
const COLOR_TRAIT_DEFINED: Color = Color::Rgb(59, 130, 246); // #3B82F6 Blue
const COLOR_TRAIT_AUTHORED: Color = Color::Rgb(34, 197, 94); // #22C55E Green
const COLOR_TRAIT_IMPORTED: Color = Color::Rgb(139, 92, 246); // #8B5CF6 Violet
const COLOR_TRAIT_GENERATED: Color = Color::Rgb(181, 137, 0); // #B58900 Gold
const COLOR_TRAIT_RETRIEVED: Color = Color::Rgb(108, 113, 196); // #6C71C4 Violet

/// Arc family colors
const COLOR_FAMILY_OWNERSHIP: Color = Color::Rgb(59, 130, 246); // Blue
const COLOR_FAMILY_SEMANTIC: Color = Color::Rgb(249, 115, 22); // Orange
const COLOR_FAMILY_GENERATION: Color = Color::Rgb(181, 137, 0); // Gold
const COLOR_FAMILY_LOCALIZATION: Color = Color::Rgb(34, 197, 94); // Green
const COLOR_FAMILY_MINING: Color = Color::Rgb(139, 92, 246); // Violet

// =============================================================================
// YAML SYNTAX HIGHLIGHTING STYLES
// =============================================================================

/// YAML comment style.
const STYLE_YAML_COMMENT: Style = Style::new().fg(Color::DarkGray);

/// YAML key style.
const STYLE_YAML_KEY: Style = Style::new().fg(Color::Yellow);

/// YAML colon/dash style.
const STYLE_YAML_PUNCT: Style = Style::new().fg(Color::Cyan);

/// YAML string value style.
const STYLE_YAML_STRING: Style = Style::new().fg(Color::Green);

/// YAML boolean/null style.
const STYLE_YAML_LITERAL: Style = Style::new().fg(Color::Magenta);

/// YAML number style.
const STYLE_YAML_NUMBER: Style = Style::new().fg(Color::Cyan);

/// YAML plain text style.
const STYLE_YAML_TEXT: Style = Style::new().fg(Color::White);

// =============================================================================
// PUBLIC API
// =============================================================================

/// Render the YAML panel split into SOURCE and DIAGRAM boxes.
/// v0.13.0: Two boxes with independent selected states using InfoBox.
///
/// Visual states:
/// - Selected (cyan): This specific box is active (Tab target)
/// - Unfocused (dim): This box is NOT selected
pub fn render_yaml_panel(f: &mut Frame, area: Rect, app: &App) {
    // Split into SOURCE (60%) and DIAGRAM (40%)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Render SOURCE box (YAML content)
    let source_selected = app.selected_box == InfoBox::Source;
    render_source_box(f, chunks[0], app, source_selected);

    // Render DIAGRAM box (ASCII hierarchy)
    let diagram_selected = app.selected_box == InfoBox::Diagram;
    render_diagram_box(f, chunks[1], app, diagram_selected);
}

/// Render the SOURCE box with YAML content.
fn render_source_box(f: &mut Frame, area: Rect, app: &App, selected: bool) {
    let visible_height = area.height.saturating_sub(2) as usize;

    // Determine border color: selected = cyan, otherwise = dim
    let border_color = if selected {
        BOX_BORDER_SELECTED
    } else {
        BOX_BORDER_UNFOCUSED
    };

    // Build enhanced title with line count badge
    let line_count = app.yaml_content.lines().count();
    let title = if selected {
        Line::from(vec![
            Span::styled(" ▶ ", Style::default().fg(BOX_BORDER_SELECTED).add_modifier(Modifier::BOLD)),
            Span::styled("SOURCE ", Style::default().fg(BOX_BORDER_SELECTED).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("⊞{} ", line_count),
                Style::default().fg(Color::Rgb(136, 192, 208)), // Nord Frost
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(" SOURCE ", Style::default().fg(COLOR_MUTED_TEXT)),
            Span::styled(
                format!("⊞{} ", line_count),
                Style::default().fg(Color::DarkGray),
            ),
        ])
    };

    render_yaml_content_in_box(f, area, app, visible_height, border_color, title);
}

/// Render the DIAGRAM box with ASCII hierarchy and arc graph.
fn render_diagram_box(f: &mut Frame, area: Rect, app: &App, selected: bool) {
    // Determine border color: selected = cyan, otherwise = dim
    let border_color = if selected {
        BOX_BORDER_SELECTED
    } else {
        BOX_BORDER_UNFOCUSED
    };

    // Get diagram type label
    let diagram_type = get_diagram_type(app);

    // Build enhanced title with diagram type badge
    let title = if selected {
        Line::from(vec![
            Span::styled(" ▶ ", Style::default().fg(BOX_BORDER_SELECTED).add_modifier(Modifier::BOLD)),
            Span::styled("DIAGRAM ", Style::default().fg(BOX_BORDER_SELECTED).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("◇{} ", diagram_type),
                Style::default().fg(Color::Rgb(180, 142, 173)), // Nord Aurora Purple
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(" DIAGRAM ", Style::default().fg(COLOR_MUTED_TEXT)),
            Span::styled(
                format!("◇{} ", diagram_type),
                Style::default().fg(Color::DarkGray),
            ),
        ])
    };

    // Generate diagram content based on current item
    let diagram_lines = generate_diagram_content(app);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(diagram_lines).block(block);
    f.render_widget(paragraph, area);
}

/// Get diagram type label for the title.
fn get_diagram_type(app: &App) -> &'static str {
    use crate::tui::data::TreeItem;
    match app.current_item() {
        Some(TreeItem::Class(..)) => "ER",
        Some(TreeItem::Realm(_)) => "HIER",
        Some(TreeItem::Layer(..)) => "HIER",
        Some(TreeItem::Instance(..)) => "INST",
        Some(TreeItem::ArcFamily(_)) => "FAM",
        Some(TreeItem::ArcClass(..)) => "ARC",
        Some(TreeItem::EntityCategory(..)) => "CAT",
        _ => "—",
    }
}

/// Generate rich ASCII diagram content for the current item.
/// v0.13: Enhanced with colored badges, arc graphs, and trait indicators.
fn generate_diagram_content(app: &App) -> Vec<Line<'static>> {
    use crate::tui::data::TreeItem;

    let mut lines = Vec::new();

    match app.current_item() {
        Some(TreeItem::Class(realm, layer, class)) => {
            // Header with realm/layer badges
            let rc = realm_color(&realm.key);
            let tc = trait_color(&class.trait_name);

            lines.push(Line::from(vec![
                Span::styled("┌─ ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("●{}", realm.key.to_uppercase()), Style::default().fg(rc).add_modifier(Modifier::BOLD)),
                Span::styled(" / ", Style::default().fg(Color::DarkGray)),
                Span::styled(layer.key.clone(), Style::default().fg(COLOR_LAYER_SEMANTIC)),
                Span::styled(" ─┐", Style::default().fg(Color::DarkGray)),
            ]));

            // Class box with trait badge
            lines.push(Line::from(vec![
                Span::styled("│  ", Style::default().fg(Color::DarkGray)),
                Span::styled(trait_icon(&class.trait_name), Style::default().fg(tc)),
                Span::styled(" ", Style::default()),
                Span::styled(class.display_name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));

            // Properties summary
            let req_count = class.required_properties.len();
            let total_count = class.properties.len();
            lines.push(Line::from(vec![
                Span::styled("│  ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("⊞{}/{} props", req_count, total_count), Style::default().fg(Color::Rgb(136, 192, 208))),
            ]));

            // Arc graph if available
            if let Some(ref arcs) = app.class_arcs {
                lines.push(Line::from(Span::styled("├──────────────────", Style::default().fg(Color::DarkGray))));

                // Outgoing arcs (max 3)
                for arc in arcs.outgoing.iter().take(3) {
                    let fc = arc_family_color(&arc.family);
                    lines.push(Line::from(vec![
                        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
                        Span::styled("→ ", Style::default().fg(fc).add_modifier(Modifier::BOLD)),
                        Span::styled(arc.arc_key.clone(), Style::default().fg(fc)),
                        Span::styled(" → ", Style::default().fg(Color::DarkGray)),
                        Span::styled(arc.other_class.clone(), Style::default().fg(Color::White)),
                    ]));
                }
                if arcs.outgoing.len() > 3 {
                    lines.push(Line::from(vec![
                        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(format!("  +{} more", arcs.outgoing.len() - 3), Style::default().fg(Color::DarkGray)),
                    ]));
                }

                // Incoming arcs (max 3)
                for arc in arcs.incoming.iter().take(3) {
                    let fc = arc_family_color(&arc.family);
                    lines.push(Line::from(vec![
                        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
                        Span::styled("← ", Style::default().fg(fc).add_modifier(Modifier::BOLD)),
                        Span::styled(arc.other_class.clone(), Style::default().fg(Color::White)),
                        Span::styled(" ← ", Style::default().fg(Color::DarkGray)),
                        Span::styled(arc.arc_key.clone(), Style::default().fg(fc)),
                    ]));
                }
                if arcs.incoming.len() > 3 {
                    lines.push(Line::from(vec![
                        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(format!("  +{} more", arcs.incoming.len() - 3), Style::default().fg(Color::DarkGray)),
                    ]));
                }
            }

            lines.push(Line::from(Span::styled("└──────────────────", Style::default().fg(Color::DarkGray))));
        }

        Some(TreeItem::Realm(realm)) => {
            let rc = realm_color(&realm.key);

            lines.push(Line::from(vec![
                Span::styled("◉ ", Style::default().fg(rc).add_modifier(Modifier::BOLD)),
                Span::styled(realm.display_name.clone(), Style::default().fg(rc).add_modifier(Modifier::BOLD)),
            ]));

            // Show layers with class counts
            if let Some(realm_info) = app.tree.realms.iter().find(|r| r.key == realm.key) {
                for (i, layer) in realm_info.layers.iter().enumerate() {
                    let is_last = i == realm_info.layers.len() - 1;
                    let prefix = if is_last { "└── " } else { "├── " };

                    lines.push(Line::from(vec![
                        Span::styled(prefix, Style::default().fg(Color::DarkGray)),
                        Span::styled(layer.display_name.clone(), Style::default().fg(Color::Yellow)),
                        Span::styled(format!(" ({})", layer.classes.len()), Style::default().fg(Color::DarkGray)),
                    ]));
                }
            }
        }

        Some(TreeItem::Layer(realm, layer)) => {
            let rc = realm_color(&realm.key);

            lines.push(Line::from(vec![
                Span::styled("◎ ", Style::default().fg(rc)),
                Span::styled(realm.display_name.clone(), Style::default().fg(rc)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("└── ", Style::default().fg(Color::DarkGray)),
                Span::styled("◆ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(layer.display_name.clone(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]));

            // Show class summary by trait
            let mut trait_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            for class in &layer.classes {
                *trait_counts.entry(class.trait_name.clone()).or_insert(0) += 1;
            }

            lines.push(Line::from(Span::styled("    ┌────────────────", Style::default().fg(Color::DarkGray))));
            for (trait_name, count) in trait_counts.iter() {
                let tc = trait_color(trait_name);
                lines.push(Line::from(vec![
                    Span::styled("    │ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(trait_icon(trait_name), Style::default().fg(tc)),
                    Span::styled(format!(" {} ×{}", trait_name, count), Style::default().fg(tc)),
                ]));
            }
            lines.push(Line::from(Span::styled("    └────────────────", Style::default().fg(Color::DarkGray))));
        }

        Some(TreeItem::Instance(realm, layer, class, instance)) => {
            let rc = realm_color(&realm.key);
            let tc = trait_color(&class.trait_name);

            // Path breadcrumb
            lines.push(Line::from(vec![
                Span::styled("◎ ", Style::default().fg(rc)),
                Span::styled(realm.key.clone(), Style::default().fg(rc)),
                Span::styled(" / ", Style::default().fg(Color::DarkGray)),
                Span::styled(layer.key.clone(), Style::default().fg(Color::Yellow)),
                Span::styled(" / ", Style::default().fg(Color::DarkGray)),
                Span::styled(class.key.clone(), Style::default().fg(tc)),
            ]));

            // Instance box
            lines.push(Line::from(vec![
                Span::styled("└── ", Style::default().fg(Color::DarkGray)),
                Span::styled("● ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(instance.key.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));

            // Property count
            let prop_count = instance.properties.len();
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(format!("⊞{} properties", prop_count), Style::default().fg(Color::Rgb(136, 192, 208))),
            ]));
        }

        Some(TreeItem::ArcFamily(family)) => {
            let fc = arc_family_color(&family.key);

            lines.push(Line::from(vec![
                Span::styled("◈ ", Style::default().fg(fc).add_modifier(Modifier::BOLD)),
                Span::styled(family.display_name.clone(), Style::default().fg(fc).add_modifier(Modifier::BOLD)),
            ]));

            // Show arc classes with cardinality (limit to 5 to avoid overflow)
            for (i, arc) in family.arc_classes.iter().take(5).enumerate() {
                let is_last = i == family.arc_classes.len().min(5) - 1;
                let prefix = if is_last { "└── " } else { "├── " };

                lines.push(Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::DarkGray)),
                    Span::styled(arc.key.clone(), Style::default().fg(Color::Cyan)),
                ]));

                let sub_prefix = if is_last { "    " } else { "│   " };
                lines.push(Line::from(vec![
                    Span::styled(sub_prefix, Style::default().fg(Color::DarkGray)),
                    Span::styled(arc.from_class.clone(), Style::default().fg(Color::White)),
                    Span::styled(" → ", Style::default().fg(fc)),
                    Span::styled(arc.to_class.clone(), Style::default().fg(Color::White)),
                ]));
            }
            if family.arc_classes.len() > 5 {
                lines.push(Line::from(vec![
                    Span::styled("    ", Style::default()),
                    Span::styled(format!("+{} more arcs", family.arc_classes.len() - 5), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }

        Some(TreeItem::ArcClass(family, arc)) => {
            let fc = arc_family_color(&family.key);

            // Arc diagram box
            lines.push(Line::from(Span::styled("┌─────────────────────────┐", Style::default().fg(Color::DarkGray))));
            lines.push(Line::from(vec![
                Span::styled("│ ", Style::default().fg(Color::DarkGray)),
                Span::styled(arc.from_class.clone(), Style::default().fg(COLOR_LAYER_SEMANTIC).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" ─[{}]─▶ ", arc.key), Style::default().fg(fc).add_modifier(Modifier::BOLD)),
                Span::styled(arc.to_class.clone(), Style::default().fg(COLOR_LAYER_OUTPUT).add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from(Span::styled("└─────────────────────────┘", Style::default().fg(Color::DarkGray))));

            // Metadata
            lines.push(Line::from(vec![
                Span::styled("  family: ", Style::default().fg(Color::DarkGray)),
                Span::styled(family.key.clone(), Style::default().fg(fc)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("  cardinality: ", Style::default().fg(Color::DarkGray)),
                Span::styled(arc.cardinality.clone(), Style::default().fg(Color::Cyan)),
            ]));
        }

        Some(TreeItem::EntityCategory(realm, _layer, _class, cat)) => {
            let rc = realm_color(&realm.key);

            lines.push(Line::from(vec![
                Span::styled("◎ ", Style::default().fg(rc)),
                Span::styled("EntityCategory", Style::default().fg(Color::Yellow)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("└── ", Style::default().fg(Color::DarkGray)),
                Span::styled("▣ ", Style::default().fg(COLOR_LAYER_KNOWLEDGE)),
                Span::styled(cat.key.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));
        }

        _ => {
            lines.push(Line::from(vec![
                Span::styled("◇ ", Style::default().fg(Color::DarkGray)),
                Span::styled("Select a node", Style::default().fg(COLOR_MUTED_TEXT)),
            ]));
            lines.push(Line::from(Span::styled(
                "  to see its diagram",
                Style::default().fg(COLOR_MUTED_TEXT),
            )));
        }
    }

    lines
}

/// Get realm color from key.
fn realm_color(key: &str) -> Color {
    match key {
        "shared" => COLOR_REALM_SHARED,
        "org" => COLOR_REALM_ORG,
        _ => Color::White,
    }
}

/// Get trait color from name (ADR-024 Data Origin).
fn trait_color(trait_name: &str) -> Color {
    match trait_name {
        "defined" => COLOR_TRAIT_DEFINED,
        "authored" => COLOR_TRAIT_AUTHORED,
        "imported" => COLOR_TRAIT_IMPORTED,
        "generated" => COLOR_TRAIT_GENERATED,
        "retrieved" => COLOR_TRAIT_RETRIEVED,
        _ => Color::White,
    }
}

/// Get trait icon from name.
fn trait_icon(trait_name: &str) -> &'static str {
    match trait_name {
        "defined" => "■",
        "authored" => "□",
        "imported" => "◊",
        "generated" => "✦",
        "retrieved" => "⋆",
        _ => "○",
    }
}

/// Get arc family color from key.
fn arc_family_color(family: &str) -> Color {
    match family {
        "ownership" => COLOR_FAMILY_OWNERSHIP,
        "semantic" => COLOR_FAMILY_SEMANTIC,
        "generation" => COLOR_FAMILY_GENERATION,
        "localization" => COLOR_FAMILY_LOCALIZATION,
        "mining" => COLOR_FAMILY_MINING,
        _ => Color::White,
    }
}

// =============================================================================
// INTERNAL FUNCTIONS
// =============================================================================

/// Render YAML content in a box with given border color and title.
fn render_yaml_content_in_box(
    f: &mut Frame,
    area: Rect,
    app: &App,
    visible_height: usize,
    border_color: Color,
    title: Line<'static>,
) {
    // Check if we have valid parsed sections for contextual view
    let sections_opt = app.yaml_sections.as_ref().filter(|s| s.is_valid());
    let active_section = app.yaml_active_section();

    // Build YAML lines with syntax highlighting
    let mut lines: Vec<Line> = Vec::new();

    if let Some(sections) = sections_opt {
        // Contextual view: show active section with ellipsis for hidden section

        match active_section {
            YamlViewSection::Class => {
                // Show Class section
                for yaml_line in sections
                    .class_lines_iter()
                    .skip(app.yaml_scroll)
                    .take(visible_height.saturating_sub(1))
                {
                    lines.push(highlight_yaml_line(yaml_line));
                }
                // Add ellipsis for hidden Instance section (if not in peek mode)
                if !app.yaml_peek && lines.len() < visible_height {
                    let hint = "[Enter: peek]";
                    lines.push(Line::from(vec![
                        Span::styled("... ", Style::default().fg(COLOR_MUTED_TEXT)),
                        Span::styled(
                            "standard_properties",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::DIM),
                        ),
                        Span::styled(
                            format!(" ({} lines) ", sections.instance_line_count()),
                            Style::default().fg(COLOR_MUTED_TEXT),
                        ),
                        Span::styled(hint, Style::default().fg(COLOR_HINT_TEXT)),
                        Span::styled(" ...", Style::default().fg(COLOR_MUTED_TEXT)),
                    ]));
                }
                // Show peeked content (dim)
                if app.yaml_peek {
                    lines.push(Line::from(Span::styled(
                        "................................................",
                        Style::default().fg(COLOR_MUTED_TEXT),
                    )));
                    let remaining = visible_height.saturating_sub(lines.len()).saturating_sub(1);
                    for yaml_line in sections.instance_lines_iter().take(remaining) {
                        lines.push(highlight_yaml_line_dim(yaml_line));
                    }
                    let hint = "[Enter: collapse]";
                    lines.push(Line::from(vec![
                        Span::styled("............ ", Style::default().fg(COLOR_MUTED_TEXT)),
                        Span::styled(hint, Style::default().fg(COLOR_HINT_TEXT)),
                        Span::styled(" ............", Style::default().fg(COLOR_MUTED_TEXT)),
                    ]));
                }
            }
            YamlViewSection::Instance => {
                // Add ellipsis for hidden Class section (if not in peek mode)
                if !app.yaml_peek {
                    let hint = "[Enter: peek]";
                    lines.push(Line::from(vec![
                        Span::styled("... ", Style::default().fg(COLOR_MUTED_TEXT)),
                        Span::styled(
                            "node metadata",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::DIM),
                        ),
                        Span::styled(
                            format!(" ({} lines) ", sections.class_line_count()),
                            Style::default().fg(COLOR_MUTED_TEXT),
                        ),
                        Span::styled(hint, Style::default().fg(COLOR_HINT_TEXT)),
                        Span::styled(" ...", Style::default().fg(COLOR_MUTED_TEXT)),
                    ]));
                }
                // Show peeked Class content (dim) at the top
                if app.yaml_peek {
                    let hint = "[Enter: collapse]";
                    lines.push(Line::from(vec![
                        Span::styled("............ ", Style::default().fg(COLOR_MUTED_TEXT)),
                        Span::styled(hint, Style::default().fg(COLOR_HINT_TEXT)),
                        Span::styled(" ............", Style::default().fg(COLOR_MUTED_TEXT)),
                    ]));
                    let peek_lines = visible_height / 3; // Show ~1/3 of the hidden section
                    for yaml_line in sections.class_lines_iter().take(peek_lines) {
                        lines.push(highlight_yaml_line_dim(yaml_line));
                    }
                    lines.push(Line::from(Span::styled(
                        "................................................",
                        Style::default().fg(COLOR_MUTED_TEXT),
                    )));
                }
                // Show Instance section
                let remaining = visible_height.saturating_sub(lines.len());
                for yaml_line in sections
                    .instance_lines_iter()
                    .skip(app.yaml_scroll)
                    .take(remaining)
                {
                    lines.push(highlight_yaml_line(yaml_line));
                }
            }
        }
    } else if !app.yaml_content.is_empty() {
        // Fallback: show full YAML (non-NodeClass files)
        for yaml_line in app
            .yaml_content
            .lines()
            .skip(app.yaml_scroll)
            .take(visible_height)
        {
            lines.push(highlight_yaml_line(yaml_line));
        }
    } else {
        lines.push(Line::from(Span::styled("No YAML file", STYLE_DIM)));
    }

    // Compute total lines for scroll indicator
    let total_lines = match sections_opt {
        Some(sections) => match active_section {
            YamlViewSection::Class => sections.class_line_count(),
            YamlViewSection::Instance => sections.instance_line_count(),
        },
        None => app.yaml_content.lines().count(),
    };

    // Build scroll indicator with directional arrows
    let scroll_hint = scroll_indicator(app.yaml_scroll, total_lines, visible_height);

    let block = Block::default()
        .title(title)
        .title_bottom(Span::styled(scroll_hint, STYLE_DIM))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);

    // Add scrollbar if content exceeds visible area
    if total_lines > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("^"))
            .end_symbol(Some("v"))
            .track_symbol(Some("|"))
            .thumb_symbol("#");

        let mut scrollbar_state = ScrollbarState::new(total_lines.saturating_sub(visible_height))
            .position(app.yaml_scroll);

        // Render scrollbar in the inner area (inside border)
        let scrollbar_area = Rect {
            x: area.x + area.width.saturating_sub(2),
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

/// Highlight a YAML line with dimmed colors (for peeked content).
fn highlight_yaml_line_dim(line: &str) -> Line<'static> {
    let dim_style = Style::default().fg(Color::DarkGray);
    Line::from(Span::styled(line.to_string(), dim_style))
}

/// Build YAML panel title with section tabs.
#[allow(dead_code)] // Used in tests
fn build_yaml_title_with_tabs(
    path: &str,
    active: YamlViewSection,
    has_sections: bool,
) -> Vec<Span<'static>> {
    let mut spans = vec![Span::styled(" ", Style::default())];

    // Add tabs if we have sections
    if has_sections {
        let (class_style, instance_style) = match active {
            YamlViewSection::Class => (
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(COLOR_MUTED_TEXT),
            ),
            YamlViewSection::Instance => (
                Style::default().fg(COLOR_MUTED_TEXT),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        };

        let class_indicator = if active == YamlViewSection::Class {
            "*"
        } else {
            "o"
        };
        let instance_indicator = if active == YamlViewSection::Instance {
            "*"
        } else {
            "o"
        };

        spans.push(Span::styled("[Class ", class_style));
        spans.push(Span::styled(class_indicator, class_style));
        spans.push(Span::styled("]", class_style));
        spans.push(Span::styled(" ", Style::default()));
        spans.push(Span::styled("[Instance ", instance_style));
        spans.push(Span::styled(instance_indicator, instance_style));
        spans.push(Span::styled("]", instance_style));
        spans.push(Span::styled("  ", Style::default()));
    }

    // Add path
    if !path.is_empty() {
        spans.extend(colorize_path_inline(path));
    } else {
        spans.push(Span::styled("YAML", STYLE_UNFOCUSED));
    }

    spans.push(Span::styled(" ", Style::default()));
    spans
}

/// Highlight a YAML line with syntax coloring.
fn highlight_yaml_line(line: &str) -> Line<'static> {
    // Comment line
    if line.trim_start().starts_with('#') {
        return Line::from(Span::styled(line.to_string(), STYLE_YAML_COMMENT));
    }

    // Empty line
    if line.trim().is_empty() {
        return Line::from(Span::raw(line.to_string()));
    }

    // Key-value or list item (most lines have 2-4 spans)
    let mut spans: Vec<Span<'static>> = Vec::with_capacity(4);

    // Find leading whitespace
    let indent_len = line.len() - line.trim_start().len();
    let indent = &line[..indent_len];
    let rest = &line[indent_len..];

    spans.push(Span::raw(indent.to_string()));

    // Check for list item
    if rest.starts_with("- ") {
        spans.push(Span::styled("-", STYLE_YAML_PUNCT));
        let after_dash = &rest[1..];

        // Check if it's a key-value after dash
        if let Some(colon_pos) = after_dash.find(':') {
            let key = &after_dash[..colon_pos + 1];
            let value = &after_dash[colon_pos + 1..];
            spans.push(Span::styled(key.to_string(), STYLE_YAML_KEY));
            spans.push(highlight_yaml_value(value));
        } else {
            spans.push(highlight_yaml_value(after_dash));
        }
    } else if let Some(colon_pos) = rest.find(':') {
        // Key-value pair
        let key = &rest[..colon_pos];
        let colon_and_rest = &rest[colon_pos..];

        spans.push(Span::styled(key.to_string(), STYLE_YAML_KEY));

        if colon_and_rest.len() > 1 {
            spans.push(Span::styled(":", STYLE_YAML_TEXT));
            let value = &colon_and_rest[1..];
            spans.push(highlight_yaml_value(value));
        } else {
            spans.push(Span::styled(":", STYLE_YAML_TEXT));
        }
    } else {
        // Plain text
        spans.push(Span::styled(rest.to_string(), STYLE_YAML_TEXT));
    }

    Line::from(spans)
}

/// Highlight a YAML value with appropriate color.
/// Uses const STYLE_YAML_* for efficiency.
fn highlight_yaml_value(value: &str) -> Span<'static> {
    let trimmed = value.trim();

    // Boolean
    if trimmed == "true" || trimmed == "false" {
        return Span::styled(value.to_string(), STYLE_YAML_LITERAL);
    }

    // Null
    if trimmed == "null" || trimmed == "~" {
        return Span::styled(value.to_string(), STYLE_YAML_LITERAL);
    }

    // Number
    if trimmed.parse::<f64>().is_ok() {
        return Span::styled(value.to_string(), STYLE_YAML_NUMBER);
    }

    // String (quoted or unquoted)
    Span::styled(value.to_string(), STYLE_YAML_STRING)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // highlight_yaml_value tests
    // =========================================================================

    #[test]
    fn test_highlight_yaml_value_boolean_true() {
        let span = highlight_yaml_value(" true");
        assert_eq!(span.content, " true");
        assert_eq!(span.style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_value_boolean_false() {
        let span = highlight_yaml_value(" false");
        assert_eq!(span.content, " false");
        assert_eq!(span.style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_value_null() {
        let span = highlight_yaml_value(" null");
        assert_eq!(span.content, " null");
        assert_eq!(span.style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_value_tilde_null() {
        let span = highlight_yaml_value(" ~");
        assert_eq!(span.content, " ~");
        assert_eq!(span.style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_value_integer() {
        let span = highlight_yaml_value(" 42");
        assert_eq!(span.content, " 42");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_negative_integer() {
        let span = highlight_yaml_value(" -17");
        assert_eq!(span.content, " -17");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_float() {
        let span = highlight_yaml_value(" 3.14");
        assert_eq!(span.content, " 3.14");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_string() {
        let span = highlight_yaml_value(" hello world");
        assert_eq!(span.content, " hello world");
        assert_eq!(span.style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_value_quoted_string() {
        let span = highlight_yaml_value(" \"quoted\"");
        assert_eq!(span.content, " \"quoted\"");
        assert_eq!(span.style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_value_empty() {
        let span = highlight_yaml_value("");
        assert_eq!(span.content, "");
        assert_eq!(span.style, STYLE_YAML_STRING);
    }

    // =========================================================================
    // highlight_yaml_line tests
    // =========================================================================

    #[test]
    fn test_highlight_yaml_line_comment() {
        let line = highlight_yaml_line("# This is a comment");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "# This is a comment");
        assert_eq!(line.spans[0].style, STYLE_YAML_COMMENT);
    }

    #[test]
    fn test_highlight_yaml_line_comment_with_indent() {
        let line = highlight_yaml_line("  # Indented comment");
        // Indented comments are still treated as full comment lines
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "  # Indented comment");
        assert_eq!(line.spans[0].style, STYLE_YAML_COMMENT);
    }

    #[test]
    fn test_highlight_yaml_line_empty() {
        let line = highlight_yaml_line("");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "");
    }

    #[test]
    fn test_highlight_yaml_line_whitespace_only() {
        let line = highlight_yaml_line("   ");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "   ");
    }

    #[test]
    fn test_highlight_yaml_line_key_value() {
        let line = highlight_yaml_line("name: Page");
        assert_eq!(line.spans.len(), 4);
        // spans[0] = "" (empty indent)
        // spans[1] = "name" (key)
        // spans[2] = ":" (colon)
        // spans[3] = " Page" (value)
        assert_eq!(line.spans[1].content, "name");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
        assert_eq!(line.spans[2].content, ":");
        assert_eq!(line.spans[3].content, " Page");
        assert_eq!(line.spans[3].style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_line_key_value_indented() {
        let line = highlight_yaml_line("  realm: shared");
        assert_eq!(line.spans.len(), 4);
        assert_eq!(line.spans[0].content, "  "); // indent
        assert_eq!(line.spans[1].content, "realm"); // key
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
        assert_eq!(line.spans[2].content, ":"); // colon
        assert_eq!(line.spans[3].content, " shared"); // value
    }

    #[test]
    fn test_highlight_yaml_line_key_with_boolean_value() {
        let line = highlight_yaml_line("enabled: true");
        assert_eq!(line.spans[3].content, " true");
        assert_eq!(line.spans[3].style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_line_key_with_number_value() {
        let line = highlight_yaml_line("count: 42");
        assert_eq!(line.spans[3].content, " 42");
        assert_eq!(line.spans[3].style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_line_key_no_value() {
        let line = highlight_yaml_line("properties:");
        assert_eq!(line.spans.len(), 3);
        assert_eq!(line.spans[1].content, "properties");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
        assert_eq!(line.spans[2].content, ":");
    }

    #[test]
    fn test_highlight_yaml_line_list_item() {
        let line = highlight_yaml_line("- item");
        assert_eq!(line.spans.len(), 3);
        assert_eq!(line.spans[0].content, ""); // empty indent
        assert_eq!(line.spans[1].content, "-");
        assert_eq!(line.spans[1].style, STYLE_YAML_PUNCT);
        assert_eq!(line.spans[2].content, " item");
        assert_eq!(line.spans[2].style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_line_list_item_indented() {
        let line = highlight_yaml_line("  - indented item");
        assert_eq!(line.spans[0].content, "  "); // indent
        assert_eq!(line.spans[1].content, "-");
        assert_eq!(line.spans[1].style, STYLE_YAML_PUNCT);
    }

    #[test]
    fn test_highlight_yaml_line_list_item_with_key_value() {
        let line = highlight_yaml_line("- name: value");
        assert_eq!(line.spans.len(), 4);
        assert_eq!(line.spans[1].content, "-");
        assert_eq!(line.spans[1].style, STYLE_YAML_PUNCT);
        assert_eq!(line.spans[2].content, " name:");
        assert_eq!(line.spans[2].style, STYLE_YAML_KEY);
        assert_eq!(line.spans[3].content, " value");
        assert_eq!(line.spans[3].style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_line_plain_text() {
        let line = highlight_yaml_line("just plain text without colon");
        assert_eq!(line.spans.len(), 2);
        assert_eq!(line.spans[0].content, ""); // empty indent
        assert_eq!(line.spans[1].content, "just plain text without colon");
        assert_eq!(line.spans[1].style, STYLE_YAML_TEXT);
    }

    // =========================================================================
    // highlight_yaml_line_dim tests
    // =========================================================================

    #[test]
    fn test_highlight_yaml_line_dim() {
        let line = highlight_yaml_line_dim("name: value");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "name: value");
        assert_eq!(line.spans[0].style.fg, Some(Color::DarkGray));
    }

    #[test]
    fn test_highlight_yaml_line_dim_empty() {
        let line = highlight_yaml_line_dim("");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "");
    }

    // =========================================================================
    // build_yaml_title_with_tabs tests
    // =========================================================================

    #[test]
    fn test_build_yaml_title_no_sections() {
        let spans = build_yaml_title_with_tabs("path/to/file.yaml", YamlViewSection::Class, false);
        // Without sections, should just show the path
        assert!(!spans.is_empty());
        // Should not have [Class] or [Instance] tabs
        let full_text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(!full_text.contains("[Class"));
        assert!(!full_text.contains("[Instance"));
        assert!(full_text.contains("file.yaml"));
    }

    #[test]
    fn test_build_yaml_title_with_sections_class_active() {
        let spans = build_yaml_title_with_tabs("file.yaml", YamlViewSection::Class, true);
        let full_text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        // Should have tabs
        assert!(full_text.contains("[Class"));
        assert!(full_text.contains("[Instance"));
        // Class should be active (*)
        assert!(full_text.contains("*]")); // active indicator
    }

    #[test]
    fn test_build_yaml_title_with_sections_instance_active() {
        let spans = build_yaml_title_with_tabs("file.yaml", YamlViewSection::Instance, true);
        let full_text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        // Should have tabs
        assert!(full_text.contains("[Class"));
        assert!(full_text.contains("[Instance"));
    }

    #[test]
    fn test_build_yaml_title_empty_path() {
        let spans = build_yaml_title_with_tabs("", YamlViewSection::Class, false);
        let full_text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        // Should show "YAML" fallback
        assert!(full_text.contains("YAML"));
    }

    #[test]
    fn test_build_yaml_title_active_indicators() {
        // When Class is active
        let class_spans = build_yaml_title_with_tabs("f.yaml", YamlViewSection::Class, true);
        let kind_text: String = class_spans.iter().map(|s| s.content.as_ref()).collect();

        // When Instance is active
        let instance_spans = build_yaml_title_with_tabs("f.yaml", YamlViewSection::Instance, true);
        let instance_text: String = instance_spans.iter().map(|s| s.content.as_ref()).collect();

        // Both should contain the active indicator somewhere
        assert!(kind_text.contains("*"));
        assert!(instance_text.contains("*"));
    }

    // =========================================================================
    // Style constant tests
    // =========================================================================

    #[test]
    fn test_style_yaml_comment_is_dark_gray() {
        assert_eq!(STYLE_YAML_COMMENT.fg, Some(Color::DarkGray));
    }

    #[test]
    fn test_style_yaml_key_is_yellow() {
        assert_eq!(STYLE_YAML_KEY.fg, Some(Color::Yellow));
    }

    #[test]
    fn test_style_yaml_punct_is_cyan() {
        assert_eq!(STYLE_YAML_PUNCT.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_style_yaml_string_is_green() {
        assert_eq!(STYLE_YAML_STRING.fg, Some(Color::Green));
    }

    #[test]
    fn test_style_yaml_literal_is_magenta() {
        assert_eq!(STYLE_YAML_LITERAL.fg, Some(Color::Magenta));
    }

    #[test]
    fn test_style_yaml_number_is_cyan() {
        assert_eq!(STYLE_YAML_NUMBER.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_style_yaml_text_is_white() {
        assert_eq!(STYLE_YAML_TEXT.fg, Some(Color::White));
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_highlight_yaml_line_colon_in_value() {
        // Value containing a colon (URL, time, etc.)
        let line = highlight_yaml_line("url: https://example.com");
        assert_eq!(line.spans[1].content, "url");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
        // The value should include the URL with colons
        assert_eq!(line.spans[3].content, " https://example.com");
    }

    #[test]
    fn test_highlight_yaml_line_multiword_key() {
        let line = highlight_yaml_line("display_name: My Page");
        assert_eq!(line.spans[1].content, "display_name");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
    }

    #[test]
    fn test_highlight_yaml_line_deeply_indented() {
        let line = highlight_yaml_line("        nested: value");
        assert_eq!(line.spans[0].content, "        "); // 8 spaces
        assert_eq!(line.spans[1].content, "nested");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
    }

    #[test]
    fn test_highlight_yaml_value_scientific_notation() {
        let span = highlight_yaml_value(" 1.5e10");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_zero() {
        let span = highlight_yaml_value(" 0");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_negative_float() {
        let span = highlight_yaml_value(" -0.5");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }
}
