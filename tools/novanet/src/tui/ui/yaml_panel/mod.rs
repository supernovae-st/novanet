//! YAML panel rendering for TUI.
//!
//! This module handles the content panel with:
//! - Schema YAML preview with syntax highlighting
//! - Instance properties in YAML-like format
//! - Section info for navigation nodes
//! - Empty state placeholder
//!
//! Submodules:
//! - `colors`: Semantic color mapping (realm, layer, arc family, scope, cardinality)
//! - `properties`: Instance property rendering with STANDARD/SPECIFIC grouping
//! - `syntax`: YAML syntax highlighting and arc badge rendering
//! - `utils`: Text formatting helpers (word wrap, timestamp, path abbreviation)

mod colors;
mod properties;
mod syntax;
mod utils;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::{BOX_BORDER_SELECTED, BOX_BORDER_UNFOCUSED, COLOR_MUTED_TEXT, STYLE_PALETTE_DIM};
use crate::tui::app::{App, ContentPanelMode};
use crate::tui::palette;
use crate::tui::widgets::bordered_block;

use self::properties::render_instance_info;
use self::syntax::render_yaml_content_in_box;
use self::utils::abbreviate_yaml_path;

// =============================================================================
// PUBLIC API
// =============================================================================

/// Render the Content panel [2] (SOURCE box only).
/// Renamed from render_yaml_panel to reflect context-aware content.
///
/// Visual states:
/// - Selected (cyan): This panel is focused (Focus::Content)
/// - Unfocused (dim): This panel is NOT focused
pub fn render_content_panel(f: &mut Frame, area: Rect, app: &App) {
    use crate::tui::app::Focus;
    let source_selected = app.focus == Focus::Content;
    render_source_box(f, area, app, source_selected);
}

/// Render the SOURCE box with context-aware content.
/// Uses ContentPanelMode to determine what to show.
fn render_source_box(f: &mut Frame, area: Rect, app: &App, selected: bool) {
    let border_color = if selected {
        BOX_BORDER_SELECTED
    } else {
        BOX_BORDER_UNFOCUSED
    };

    let mode = app.content_panel_mode();

    match mode {
        ContentPanelMode::Schema { path, name } => {
            render_schema_content(f, area, app, selected, border_color, &path, &name);
        },
        ContentPanelMode::InstanceInfo {
            instance_key,
            class_name,
            realm,
            layer,
            ref properties,
        } => {
            render_instance_info(
                f,
                area,
                selected,
                border_color,
                &instance_key,
                &class_name,
                &realm,
                &layer,
                properties,
                app.instance_standard_collapsed,
                app.instance_specific_collapsed,
            );
        },
        ContentPanelMode::SectionInfo { name, description } => {
            render_section_info(f, area, selected, border_color, &name, &description);
        },
        ContentPanelMode::Empty => {
            render_empty_content(f, area, selected, border_color);
        },
    }
}

// =============================================================================
// CONTENT PANEL RENDER FUNCTIONS
// =============================================================================

/// Render SCHEMA content - shows YAML with syntax highlighting.
/// Used when a Class or ArcClass is selected in the tree.
fn render_schema_content(
    f: &mut Frame,
    area: Rect,
    app: &App,
    selected: bool,
    border_color: Color,
    yaml_path: &str,
    class_name: &str,
) {
    let visible_height = area.height.saturating_sub(2) as usize;
    let line_count = app.yaml.content.lines().count();

    let title = build_schema_title(selected, line_count, yaml_path, class_name);

    render_yaml_content_in_box(f, area, app, visible_height, border_color, title);
}

/// Build the SCHEMA panel title with YAML badge.
/// Format: ` 📄 SCHEMA ⊞N │ path/file.yaml `
fn build_schema_title(
    selected: bool,
    line_count: usize,
    yaml_path: &str,
    _class_name: &str,
) -> Line<'static> {
    let mut spans = Vec::new();

    spans.push(Span::styled(" ", Style::default()));
    spans.push(Span::styled("📄", Style::default()));
    spans.push(Span::styled(" ", Style::default()));

    if selected {
        spans.push(Span::styled(
            "SCHEMA ",
            Style::default()
                .fg(BOX_BORDER_SELECTED)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        spans.push(Span::styled(
            "SCHEMA ",
            Style::default().fg(COLOR_MUTED_TEXT),
        ));
    }

    // Line count badge
    let badge_style = if selected {
        Style::default().fg(palette::NORD_FROST)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    spans.push(Span::styled(format!("⊞{}", line_count), badge_style));

    // YAML path (abbreviated)
    if !yaml_path.is_empty() {
        let short_path = abbreviate_yaml_path(yaml_path);
        let path_style = if selected {
            Style::default().fg(palette::SLATE_500)
        } else {
            Style::default().fg(palette::SEPARATOR)
        };
        spans.push(Span::styled(
            " │ ",
            Style::default().fg(palette::BORDER_UNFOCUSED),
        ));
        spans.push(Span::styled(short_path, path_style));
    }

    spans.push(Span::styled(" ", Style::default()));

    Line::from(spans)
}

/// Build title with NEO4J badge for instance panel.
pub(super) fn build_neo4j_title(selected: bool, instance_key: &str) -> Line<'static> {
    let border_color = if selected {
        BOX_BORDER_SELECTED
    } else {
        BOX_BORDER_UNFOCUSED
    };

    // Truncate instance key if too long (UTF-8 safe using char boundaries)
    let display_key = if instance_key.chars().count() > 30 {
        let truncated: String = instance_key.chars().take(27).collect();
        format!("{}...", truncated)
    } else {
        instance_key.to_string()
    };

    Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled("🔷", Style::default()),
        Span::styled(" ", Style::default()),
        Span::styled(
            "INSTANCE",
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(display_key, Style::default().fg(palette::NORD_FROST)),
        Span::styled(" ", Style::default()),
    ])
}

/// Render INFO content for sections (Realm, Layer, ArcFamily).
/// Used when a navigation section is selected.
fn render_section_info(
    f: &mut Frame,
    area: Rect,
    selected: bool,
    border_color: Color,
    name: &str,
    description: &str,
) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("   ", Style::default()),
        Span::styled(
            name.to_string(),
            Style::default()
                .fg(palette::NORD_FROST)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("   {}", description),
        Style::default().fg(Color::DarkGray),
    )));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "   Select a Class to view its YAML schema.",
        STYLE_PALETTE_DIM,
    )));

    let title = build_info_title(selected, name);
    let block = bordered_block(title, border_color);
    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Render empty content state.
/// Used when nothing is selected.
fn render_empty_content(f: &mut Frame, area: Rect, selected: bool, border_color: Color) {
    let lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            "   No selection",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "   Navigate to a node to view its content.",
            STYLE_PALETTE_DIM,
        )),
    ];

    let title = build_info_title(selected, "INFO");
    let block = bordered_block(title, border_color);
    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Build the INFO panel title.
/// Format: ` ◇ INSTANCE │ name `
fn build_info_title(selected: bool, name: &str) -> Line<'static> {
    let mut spans = Vec::new();

    if selected {
        spans.push(Span::styled(
            " ◇ ",
            Style::default()
                .fg(BOX_BORDER_SELECTED)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            "INSTANCE ",
            Style::default()
                .fg(BOX_BORDER_SELECTED)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        spans.push(Span::styled(
            " INSTANCE ",
            Style::default().fg(COLOR_MUTED_TEXT),
        ));
    }

    // Name badge
    let name_style = if selected {
        Style::default().fg(palette::NORD_FROST)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    spans.push(Span::styled(
        "│ ",
        Style::default().fg(palette::BORDER_UNFOCUSED),
    ));
    spans.push(Span::styled(name.to_string(), name_style));
    spans.push(Span::styled(" ", Style::default()));

    Line::from(spans)
}
