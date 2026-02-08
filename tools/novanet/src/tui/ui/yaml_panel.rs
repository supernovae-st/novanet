//! YAML panel rendering for TUI.
//!
//! This module handles the YAML preview panel with:
//! - Syntax highlighting for keys, values, comments, and punctuation
//! - Contextual view with Kind/Instance sections
//! - Peek mode to preview hidden sections
//! - Scrollbar for long content

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use super::{
    COLOR_HINT_TEXT, COLOR_MUTED_TEXT, COLOR_UNFOCUSED_BORDER, STYLE_DIM, STYLE_UNFOCUSED,
    colorize_path_inline, scroll_indicator,
};
use crate::tui::app::{App, Focus};
use crate::tui::yaml::YamlViewSection;

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

/// Render the YAML panel with syntax highlighting and contextual view.
pub fn render_yaml_panel(f: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Yaml;
    let visible_height = area.height.saturating_sub(2) as usize;

    // Always show YAML with contextual view (Kind or Instance section)
    render_yaml_content(f, area, app, focused, visible_height);
}

// =============================================================================
// INTERNAL FUNCTIONS
// =============================================================================

/// Render YAML panel with contextual view (Kind vs Instance sections).
fn render_yaml_content(f: &mut Frame, area: Rect, app: &App, focused: bool, visible_height: usize) {
    let border_color = if focused {
        Color::Cyan // Consistent with Tree/Info panels
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Check if we have valid parsed sections for contextual view
    let sections_opt = app.yaml_sections.as_ref().filter(|s| s.is_valid());
    let active_section = app.yaml_active_section();

    // Build YAML lines with syntax highlighting
    let mut lines: Vec<Line> = Vec::new();

    if let Some(sections) = sections_opt {
        // Contextual view: show active section with ellipsis for hidden section

        match active_section {
            YamlViewSection::Kind => {
                // Show Kind section
                for yaml_line in sections
                    .kind_lines_iter()
                    .skip(app.yaml_scroll)
                    .take(visible_height.saturating_sub(1))
                {
                    lines.push(highlight_yaml_line(yaml_line));
                }
                // Add ellipsis for hidden Instance section (if not in peek mode)
                if !app.yaml_peek && lines.len() < visible_height {
                    let hint = if focused { "[Enter: peek]" } else { "" };
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
                    let hint = if focused { "[Enter: collapse]" } else { "" };
                    lines.push(Line::from(vec![
                        Span::styled("............ ", Style::default().fg(COLOR_MUTED_TEXT)),
                        Span::styled(hint, Style::default().fg(COLOR_HINT_TEXT)),
                        Span::styled(" ............", Style::default().fg(COLOR_MUTED_TEXT)),
                    ]));
                }
            }
            YamlViewSection::Instance => {
                // Add ellipsis for hidden Kind section (if not in peek mode)
                if !app.yaml_peek {
                    let hint = if focused { "[Enter: peek]" } else { "" };
                    lines.push(Line::from(vec![
                        Span::styled("... ", Style::default().fg(COLOR_MUTED_TEXT)),
                        Span::styled(
                            "node metadata",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::DIM),
                        ),
                        Span::styled(
                            format!(" ({} lines) ", sections.kind_line_count()),
                            Style::default().fg(COLOR_MUTED_TEXT),
                        ),
                        Span::styled(hint, Style::default().fg(COLOR_HINT_TEXT)),
                        Span::styled(" ...", Style::default().fg(COLOR_MUTED_TEXT)),
                    ]));
                }
                // Show peeked Kind content (dim) at the top
                if app.yaml_peek {
                    let hint = if focused { "[Enter: collapse]" } else { "" };
                    lines.push(Line::from(vec![
                        Span::styled("............ ", Style::default().fg(COLOR_MUTED_TEXT)),
                        Span::styled(hint, Style::default().fg(COLOR_HINT_TEXT)),
                        Span::styled(" ............", Style::default().fg(COLOR_MUTED_TEXT)),
                    ]));
                    let peek_lines = visible_height / 3; // Show ~1/3 of the hidden section
                    for yaml_line in sections.kind_lines_iter().take(peek_lines) {
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
        // Fallback: show full YAML (non-NodeKind files)
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

    // Build title with tabs and path
    let has_sections = sections_opt.is_some();
    let title_spans = build_yaml_title_with_tabs(&app.yaml_path, active_section, has_sections);

    // Compute total lines for scroll indicator
    let total_lines = match sections_opt {
        Some(sections) => match active_section {
            YamlViewSection::Kind => sections.kind_line_count(),
            YamlViewSection::Instance => sections.instance_line_count(),
        },
        None => app.yaml_content.lines().count(),
    };

    // Build scroll indicator with directional arrows
    let scroll_hint = scroll_indicator(app.yaml_scroll, total_lines, visible_height);

    let block = Block::default()
        .title(Line::from(title_spans))
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
fn build_yaml_title_with_tabs(
    path: &str,
    active: YamlViewSection,
    has_sections: bool,
) -> Vec<Span<'static>> {
    let mut spans = vec![Span::styled(" ", Style::default())];

    // Add tabs if we have sections
    if has_sections {
        let (kind_style, instance_style) = match active {
            YamlViewSection::Kind => (
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

        let kind_indicator = if active == YamlViewSection::Kind {
            "*"
        } else {
            "o"
        };
        let instance_indicator = if active == YamlViewSection::Instance {
            "*"
        } else {
            "o"
        };

        spans.push(Span::styled("[Kind ", kind_style));
        spans.push(Span::styled(kind_indicator, kind_style));
        spans.push(Span::styled("]", kind_style));
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
