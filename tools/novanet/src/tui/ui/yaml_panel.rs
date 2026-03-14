//! YAML panel rendering for TUI.
//!
//! This module handles the YAML preview panel with:
//! - Syntax highlighting for keys, values, comments, and punctuation
//! - Scrollbar for long content
//! - v0.13.0: Split into SOURCE and DIAGRAM boxes with visual states
//! - v0.13.1: Simplified - no collapse/peek (PROPERTIES panel shows instance data)
//! - v0.17.3: Context-aware content via ContentPanelMode (Phase 3)
//! - v0.17.3: Unified panel - shows YAML for Schema, Neo4j properties for Instance

use std::collections::BTreeMap;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use serde_json::Value as JsonValue;

use super::{COLOR_MUTED_TEXT, STYLE_DIM, scroll_indicator};
use crate::tui::app::{App, ContentPanelMode};

// =============================================================================
// BOX VISUAL STATES v0.13 (enhanced palette)
// =============================================================================

/// Unfocused: Nord Polar Night (dim) - box is NOT selected
const BOX_BORDER_UNFOCUSED: Color = Color::Rgb(59, 66, 82); // #3B4252

/// Selected: Solarized Cyan (bright, active) - this specific box is Tab-selected
const BOX_BORDER_SELECTED: Color = Color::Rgb(42, 161, 152); // #2AA198

// =============================================================================
// v0.19.0 STANDARD PROPERTIES (ADR-044)
// =============================================================================
//
// ALL nodes (DATA and SCHEMA) have THE SAME 8 standard properties:
//
// 1. key            - Unique identifier
// 2. display_name   - Human-readable label
// 3. node_class     - Type discriminator (PascalCase=DATA, lowercase=SCHEMA)
// 4. content        - What this node IS (1-3 sentences)
// 5. triggers       - Keyword triggers for search/spreading activation
// 6. provenance     - Data origin {source, version}
// 7. created_at     - Creation timestamp
// 8. updated_at     - Last modification timestamp
//
// CASE CONVENTION for node_class:
//   • lowercase = SCHEMA/META node (realm, layer, class, arc_class)
//   • PascalCase = DATA node (Entity, Page, Block, etc.)
//
// =============================================================================

/// Standard properties for ALL nodes (both DATA and SCHEMA).
/// These 8 properties appear at the top of every node panel.
/// node_class case indicates type: PascalCase=DATA, lowercase=SCHEMA
const STANDARD_PROPERTIES: &[&str] = &[
    "key",
    "display_name",
    "node_class", // v0.19.0: PascalCase = DATA, lowercase = SCHEMA
    "content",    // v0.19.0: unified name for all nodes
    "triggers",
    "provenance", // v0.19.0: {source, version} object
    "created_at",
    "updated_at",
];

/// Timestamp property names for human-readable formatting.
const TIMESTAMP_PROPERTIES: &[&str] = &["created_at", "updated_at"];

/// Format a Unix timestamp as a human-readable date string.
/// Returns "YYYY-MM-DD HH:MM" format or the original number if not a valid timestamp.
fn format_timestamp(timestamp: i64) -> String {
    // Neo4j timestamps can be in seconds or milliseconds
    // Heuristic: if > 10_000_000_000, it's milliseconds
    let secs = if timestamp > 10_000_000_000 {
        timestamp / 1000
    } else {
        timestamp
    };

    // Only format positive timestamps (valid Unix time)
    if secs < 0 {
        return timestamp.to_string();
    }

    let total_secs = secs as u64;
    let days = total_secs / 86400;
    let remaining = total_secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;

    // Simple year/month/day calculation (approximate, good enough for display)
    // Starting from 1970-01-01
    let mut year = 1970u32;
    let mut remaining_days = days;

    loop {
        let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            366
        } else {
            365
        };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = [
        31,
        if is_leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let mut month = 1u32;
    for &d in &days_in_month {
        if remaining_days < d {
            break;
        }
        remaining_days -= d;
        month += 1;
    }
    let day = remaining_days + 1;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        year, month, day, hours, minutes
    )
}

// =============================================================================
// v0.17.3 INSTANCE PANEL COLORS (rich YAML-style)
// =============================================================================

/// YAML key color (cyan)
const COLOR_YAML_KEY: Color = Color::Rgb(86, 182, 194); // #56B6C2 Cyan
/// YAML string color (yellow/gold)
const COLOR_YAML_STRING: Color = Color::Rgb(229, 192, 123); // #E5C07B Yellow
/// YAML number color (orange)
const COLOR_YAML_NUMBER: Color = Color::Rgb(209, 154, 102); // #D19A66 Orange
/// YAML boolean/null color (violet)
const COLOR_YAML_BOOL: Color = Color::Rgb(198, 120, 221); // #C678DD Violet
/// Section header color (muted)
const COLOR_SECTION_HEADER: Color = Color::Rgb(92, 99, 112); // #5C6370 Muted

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

/// Render the Content panel [2] (SOURCE box only).
/// v0.17.3: Renamed from render_yaml_panel to reflect context-aware content.
///
/// Visual states:
/// - Selected (cyan): This panel is focused (Focus::Content)
/// - Unfocused (dim): This panel is NOT focused
pub fn render_content_panel(f: &mut Frame, area: Rect, app: &App) {
    // v0.17.3: Use Focus::Content for panel focus
    use crate::tui::app::Focus;
    let source_selected = app.focus == Focus::Content;
    render_source_box(f, area, app, source_selected);
}

/// Render the SOURCE box with context-aware content.
/// v0.17.3: Phase 3 - uses ContentPanelMode to determine what to show.
fn render_source_box(f: &mut Frame, area: Rect, app: &App, selected: bool) {
    // Determine border color: selected = cyan, otherwise = dim
    let border_color = if selected {
        BOX_BORDER_SELECTED
    } else {
        BOX_BORDER_UNFOCUSED
    };

    // Get the content mode based on current tree selection
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

/// Abbreviate a YAML path to show only the last 3 segments.
/// Example: "packages/core/models/node-classes/org/semantic/entity-native.yaml"
///       -> "org/semantic/entity-native.yaml"
fn abbreviate_yaml_path(path: &str) -> String {
    let segments: Vec<&str> = path.split('/').collect();
    if segments.len() <= 3 {
        path.to_string()
    } else {
        segments[segments.len() - 3..].join("/")
    }
}

// =============================================================================
// CONTENT PANEL RENDER FUNCTIONS (v0.17.3 Phase 3)
// =============================================================================

/// Render SCHEMA content - shows YAML with syntax highlighting.
/// v0.17.3: Used when a Class or ArcClass is selected in the tree.
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

    // Build title: ` ▶ SCHEMA ⊞N │ path/file.yaml `
    let title = build_schema_title(selected, line_count, yaml_path, class_name);

    render_yaml_content_in_box(f, area, app, visible_height, border_color, title);
}

/// Build the SCHEMA panel title with YAML badge.
/// v0.17.3: Added 📄 YAML badge for symmetry with 🔷 NEO4J badge on instances.
/// Format: ` 📄 SCHEMA ⊞N │ path/file.yaml ` (when selected)
fn build_schema_title(
    selected: bool,
    line_count: usize,
    yaml_path: &str,
    _class_name: &str,
) -> Line<'static> {
    let mut spans = Vec::new();

    // v0.17.3: Add 📄 YAML badge for data source indicator
    spans.push(Span::styled(" ", Style::default()));
    spans.push(Span::styled("📄", Style::default())); // YAML badge
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
        Style::default().fg(Color::Rgb(136, 192, 208)) // Nord Frost
    } else {
        Style::default().fg(Color::DarkGray)
    };
    spans.push(Span::styled(format!("⊞{}", line_count), badge_style));

    // YAML path (abbreviated)
    if !yaml_path.is_empty() {
        let short_path = abbreviate_yaml_path(yaml_path);
        let path_style = if selected {
            Style::default().fg(Color::Rgb(100, 120, 140))
        } else {
            Style::default().fg(Color::Rgb(70, 70, 70))
        };
        spans.push(Span::styled(
            " │ ",
            Style::default().fg(Color::Rgb(60, 60, 60)),
        ));
        spans.push(Span::styled(short_path, path_style));
    }

    spans.push(Span::styled(" ", Style::default()));

    Line::from(spans)
}

/// Render content for instances - shows properties in YAML-like format.
/// v0.17.3: Unified panel - symmetric with Schema (shows Neo4j data instead of YAML file).
/// v0.17.3: Enhanced display with STANDARD/SPECIFIC grouping, word-wrap, multi-line YAML.
/// v0.17.3: Added collapse/expand support for sections (Enter to toggle).
#[allow(clippy::too_many_arguments)]
fn render_instance_info(
    f: &mut Frame,
    area: Rect,
    selected: bool,
    border_color: Color,
    instance_key: &str,
    class_name: &str,
    realm: &str,
    layer: &str,
    properties: &BTreeMap<String, JsonValue>,
    standard_collapsed: bool,
    specific_collapsed: bool,
) {
    let mut lines: Vec<Line> = Vec::new();
    let panel_width = area.width.saturating_sub(4) as usize; // Account for borders + padding

    // YAML-like header with class info
    lines.push(Line::from(vec![
        Span::styled("# ", Style::default().fg(Color::Rgb(108, 113, 196))),
        Span::styled(
            format!("{} instance", class_name),
            Style::default().fg(Color::Rgb(108, 113, 196)),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("# ", Style::default().fg(Color::Rgb(108, 113, 196))),
        Span::styled(
            format!("realm: {} | layer: {}", realm, layer),
            Style::default().fg(Color::Rgb(108, 113, 196)),
        ),
    ]));
    lines.push(Line::from(""));

    // Separate standard and specific properties
    let mut standard_props: Vec<(&String, &JsonValue)> = Vec::new();
    let mut specific_props: Vec<(&String, &JsonValue)> = Vec::new();

    for (key, value) in properties.iter() {
        if STANDARD_PROPERTIES.contains(&key.as_str()) {
            standard_props.push((key, value));
        } else {
            specific_props.push((key, value));
        }
    }

    // Sort standard props in the order defined in STANDARD_PROPERTIES
    standard_props.sort_by_key(|(k, _)| {
        STANDARD_PROPERTIES
            .iter()
            .position(|p| p == k)
            .unwrap_or(usize::MAX)
    });

    // Count properties for headers
    let standard_count = standard_props.len();
    let specific_count = specific_props.len();

    // ─────────── STANDARD (N) ▼/▶ ───────────
    lines.push(render_section_separator_with_toggle(
        "STANDARD",
        standard_count,
        panel_width,
        standard_collapsed,
    ));

    if !standard_collapsed {
        lines.push(Line::from(""));
        // Render standard properties with spacing between each
        let standard_len = standard_props.len();
        for (idx, (key, value)) in standard_props.iter().enumerate() {
            render_property_lines_with_timestamp(&mut lines, key, value, panel_width, 0);
            // Add blank line between properties (but not after the last one)
            if idx < standard_len - 1 {
                lines.push(Line::from(""));
            }
        }
    }

    // Add empty line before specific if we have specific properties
    if specific_count > 0 {
        lines.push(Line::from(""));
        // ─────────── SPECIFIC (N) ▼/▶ ───────────
        lines.push(render_section_separator_with_toggle(
            "SPECIFIC",
            specific_count,
            panel_width,
            specific_collapsed,
        ));

        if !specific_collapsed {
            lines.push(Line::from(""));
            // Render specific properties with spacing between each
            let specific_len = specific_props.len();
            for (idx, (key, value)) in specific_props.iter().enumerate() {
                render_property_lines_with_timestamp(&mut lines, key, value, panel_width, 0);
                // Add blank line between properties (but not after the last one)
                if idx < specific_len - 1 {
                    lines.push(Line::from(""));
                }
            }
        }
    }

    // If no properties at all, show a hint
    if properties.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "# No properties loaded",
            Style::default().fg(Color::Rgb(108, 113, 196)),
        )));
    }

    // Build title with NEO4J badge
    let title = build_neo4j_title(selected, instance_key);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Render a section separator line with collapse/expand toggle indicator.
/// Shows ▶ when collapsed, ▼ when expanded.
/// Format: ─────── ▼ LABEL (N) ───────
fn render_section_separator_with_toggle(
    label: &str,
    count: usize,
    width: usize,
    collapsed: bool,
) -> Line<'static> {
    let toggle_icon = if collapsed { "▶" } else { "▼" };
    let label_text = format!(" {} {} ({}) ", toggle_icon, label, count);
    let label_len = label_text.chars().count();
    let remaining = width.saturating_sub(label_len);
    let left_dashes = remaining / 2;
    let right_dashes = remaining - left_dashes;

    let left = "─".repeat(left_dashes);
    let right = "─".repeat(right_dashes);

    Line::from(vec![
        Span::styled(left, Style::default().fg(COLOR_SECTION_HEADER)),
        Span::styled(label_text, Style::default().fg(COLOR_SECTION_HEADER)),
        Span::styled(right, Style::default().fg(COLOR_SECTION_HEADER)),
    ])
}

/// Render a property as one or more lines, handling word-wrap and multi-line YAML.
fn render_property_lines(
    lines: &mut Vec<Line<'static>>,
    key: &str,
    value: &JsonValue,
    width: usize,
    indent: usize,
) {
    let indent_str = "  ".repeat(indent);

    match value {
        JsonValue::Null => {
            lines.push(Line::from(vec![
                Span::styled(indent_str, Style::default()),
                Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                Span::styled(": ", Style::default().fg(Color::White)),
                Span::styled("null", Style::default().fg(COLOR_YAML_BOOL)),
            ]));
        },
        JsonValue::Bool(b) => {
            lines.push(Line::from(vec![
                Span::styled(indent_str, Style::default()),
                Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                Span::styled(": ", Style::default().fg(Color::White)),
                Span::styled(b.to_string(), Style::default().fg(COLOR_YAML_BOOL)),
            ]));
        },
        JsonValue::Number(n) => {
            lines.push(Line::from(vec![
                Span::styled(indent_str, Style::default()),
                Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                Span::styled(": ", Style::default().fg(Color::White)),
                Span::styled(n.to_string(), Style::default().fg(COLOR_YAML_NUMBER)),
            ]));
        },
        JsonValue::String(s) => {
            // v0.17.3: Check if string contains embedded JSON (common for denomination_forms etc.)
            let trimmed = s.trim();
            if (trimmed.starts_with('[') && trimmed.ends_with(']'))
                || (trimmed.starts_with('{') && trimmed.ends_with('}'))
            {
                // Try to parse as JSON and render as proper YAML
                if let Ok(parsed) = serde_json::from_str::<JsonValue>(trimmed) {
                    // Render the parsed JSON recursively
                    render_property_lines(lines, key, &parsed, width, indent);
                    return;
                }
            }

            // Calculate available width for the string value
            let key_prefix_len = indent * 2 + key.chars().count() + 2; // indent + key + ": "
            let available_width = width.saturating_sub(key_prefix_len);

            if s.chars().count() <= available_width && !s.contains('\n') {
                // Single line string
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(": ", Style::default().fg(Color::White)),
                    Span::styled(format!("\"{}\"", s), Style::default().fg(COLOR_YAML_STRING)),
                ]));
            } else {
                // Multi-line string with word-wrap
                lines.push(Line::from(vec![
                    Span::styled(indent_str.clone(), Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(": |", Style::default().fg(Color::White)),
                ]));
                // Word-wrap the string content
                let wrapped = word_wrap(s, width.saturating_sub((indent + 1) * 2));
                for line in wrapped {
                    lines.push(Line::from(vec![
                        Span::styled("  ".repeat(indent + 1), Style::default()),
                        Span::styled(line, Style::default().fg(COLOR_YAML_STRING)),
                    ]));
                }
            }
        },
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(": ", Style::default().fg(Color::White)),
                    Span::styled("[]", Style::default().fg(Color::Rgb(97, 175, 239))),
                ]));
            } else {
                // Array header
                lines.push(Line::from(vec![
                    Span::styled(indent_str.clone(), Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(":", Style::default().fg(Color::White)),
                ]));
                // Array items
                for item in arr {
                    render_array_item_lines(lines, item, width, indent + 1);
                }
            }
        },
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(": ", Style::default().fg(Color::White)),
                    Span::styled("{}", Style::default().fg(Color::Rgb(97, 175, 239))),
                ]));
            } else {
                // Object header
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(":", Style::default().fg(Color::White)),
                ]));
                // Object properties
                for (k, v) in obj.iter() {
                    render_property_lines(lines, k, v, width, indent + 1);
                }
            }
        },
    }
}

/// Render a property with special handling for timestamps.
/// v0.17.3: Timestamps (created_at, updated_at) are formatted as human-readable dates.
fn render_property_lines_with_timestamp(
    lines: &mut Vec<Line<'static>>,
    key: &str,
    value: &JsonValue,
    width: usize,
    indent: usize,
) {
    // Check if this is a timestamp property that should be formatted
    if TIMESTAMP_PROPERTIES.contains(&key) {
        if let JsonValue::Number(n) = value {
            if let Some(ts) = n.as_i64() {
                let indent_str = "  ".repeat(indent);
                let formatted = format_timestamp(ts);
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(": ", Style::default().fg(Color::White)),
                    Span::styled(formatted, Style::default().fg(Color::Rgb(136, 192, 208))), // Nord Frost for timestamps
                ]));
                return;
            }
        }
    }

    // Fall back to normal rendering
    render_property_lines(lines, key, value, width, indent);
}

/// Render an array item as YAML list item (- value).
fn render_array_item_lines(
    lines: &mut Vec<Line<'static>>,
    value: &JsonValue,
    width: usize,
    indent: usize,
) {
    let indent_str = "  ".repeat(indent);

    match value {
        JsonValue::Null => {
            lines.push(Line::from(vec![
                Span::styled(indent_str, Style::default()),
                Span::styled("- ", Style::default().fg(Color::White)),
                Span::styled("null", Style::default().fg(COLOR_YAML_BOOL)),
            ]));
        },
        JsonValue::Bool(b) => {
            lines.push(Line::from(vec![
                Span::styled(indent_str, Style::default()),
                Span::styled("- ", Style::default().fg(Color::White)),
                Span::styled(b.to_string(), Style::default().fg(COLOR_YAML_BOOL)),
            ]));
        },
        JsonValue::Number(n) => {
            lines.push(Line::from(vec![
                Span::styled(indent_str, Style::default()),
                Span::styled("- ", Style::default().fg(Color::White)),
                Span::styled(n.to_string(), Style::default().fg(COLOR_YAML_NUMBER)),
            ]));
        },
        JsonValue::String(s) => {
            let prefix_len = indent * 2 + 2; // indent + "- "
            let available_width = width.saturating_sub(prefix_len);

            if s.chars().count() <= available_width && !s.contains('\n') {
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled("- ", Style::default().fg(Color::White)),
                    Span::styled(format!("\"{}\"", s), Style::default().fg(COLOR_YAML_STRING)),
                ]));
            } else {
                // Multi-line string in array
                lines.push(Line::from(vec![
                    Span::styled(indent_str.clone(), Style::default()),
                    Span::styled("- |", Style::default().fg(Color::White)),
                ]));
                let wrapped = word_wrap(s, width.saturating_sub((indent + 1) * 2));
                for line in wrapped {
                    lines.push(Line::from(vec![
                        Span::styled("  ".repeat(indent + 1), Style::default()),
                        Span::styled(line, Style::default().fg(COLOR_YAML_STRING)),
                    ]));
                }
            }
        },
        JsonValue::Array(arr) => {
            // Nested array
            lines.push(Line::from(vec![
                Span::styled(indent_str, Style::default()),
                Span::styled("-", Style::default().fg(Color::White)),
            ]));
            for item in arr {
                render_array_item_lines(lines, item, width, indent + 1);
            }
        },
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled("- ", Style::default().fg(Color::White)),
                    Span::styled("{}", Style::default().fg(Color::Rgb(97, 175, 239))),
                ]));
            } else {
                // Object in array - first key on same line as dash
                let mut first = true;
                for (k, v) in obj.iter() {
                    if first {
                        // First key-value on same line as -
                        match v {
                            JsonValue::String(s) if s.chars().count() < 30 && !s.contains('\n') => {
                                lines.push(Line::from(vec![
                                    Span::styled(indent_str.clone(), Style::default()),
                                    Span::styled("- ", Style::default().fg(Color::White)),
                                    Span::styled(
                                        k.to_string(),
                                        Style::default().fg(COLOR_YAML_KEY),
                                    ),
                                    Span::styled(": ", Style::default().fg(Color::White)),
                                    Span::styled(
                                        format!("\"{}\"", s),
                                        Style::default().fg(COLOR_YAML_STRING),
                                    ),
                                ]));
                            },
                            JsonValue::Number(n) => {
                                lines.push(Line::from(vec![
                                    Span::styled(indent_str.clone(), Style::default()),
                                    Span::styled("- ", Style::default().fg(Color::White)),
                                    Span::styled(
                                        k.to_string(),
                                        Style::default().fg(COLOR_YAML_KEY),
                                    ),
                                    Span::styled(": ", Style::default().fg(Color::White)),
                                    Span::styled(
                                        n.to_string(),
                                        Style::default().fg(COLOR_YAML_NUMBER),
                                    ),
                                ]));
                            },
                            JsonValue::Bool(b) => {
                                lines.push(Line::from(vec![
                                    Span::styled(indent_str.clone(), Style::default()),
                                    Span::styled("- ", Style::default().fg(Color::White)),
                                    Span::styled(
                                        k.to_string(),
                                        Style::default().fg(COLOR_YAML_KEY),
                                    ),
                                    Span::styled(": ", Style::default().fg(Color::White)),
                                    Span::styled(
                                        b.to_string(),
                                        Style::default().fg(COLOR_YAML_BOOL),
                                    ),
                                ]));
                            },
                            _ => {
                                // Complex first value - put on next line
                                lines.push(Line::from(vec![
                                    Span::styled(indent_str.clone(), Style::default()),
                                    Span::styled("-", Style::default().fg(Color::White)),
                                ]));
                                render_property_lines(lines, k, v, width, indent + 1);
                            },
                        }
                        first = false;
                    } else {
                        render_property_lines(lines, k, v, width, indent + 1);
                    }
                }
            }
        },
    }
}

/// Word-wrap a string at word boundaries.
fn word_wrap(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }

    let mut result = Vec::new();

    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            result.push(String::new());
            continue;
        }

        let mut current_line = String::new();

        for word in paragraph.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.chars().count() + 1 + word.chars().count() <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                result.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            result.push(current_line);
        }
    }

    if result.is_empty() {
        result.push(String::new());
    }

    result
}

/// Build title with NEO4J badge for instance panel.
fn build_neo4j_title(selected: bool, instance_key: &str) -> Line<'static> {
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
        Span::styled("🔷", Style::default()), // NEO4J badge
        Span::styled(" ", Style::default()),
        Span::styled(
            "INSTANCE",
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(display_key, Style::default().fg(Color::Rgb(136, 192, 208))),
        Span::styled(" ", Style::default()),
    ])
}

/// Render INFO content for sections (Realm, Layer, ArcFamily).
/// v0.17.3: Used when a navigation section is selected.
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
                .fg(Color::Rgb(136, 192, 208)) // Nord Frost
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
        Style::default().fg(Color::Rgb(100, 100, 100)),
    )));

    // Build title
    let title = build_info_title(selected, name);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Render empty content state.
/// v0.17.3: Used when nothing is selected.
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
            Style::default().fg(Color::Rgb(100, 100, 100)),
        )),
    ];

    // Build title
    let title = build_info_title(selected, "INFO");

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Build the INFO panel title.
/// Format: ` ▶ INFO │ name ` (when selected)
fn build_info_title(selected: bool, name: &str) -> Line<'static> {
    let mut spans = Vec::new();

    // v0.17.3: Replace generic "INFO" with "INSTANCE" for clarity
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
        Style::default().fg(Color::Rgb(136, 192, 208)) // Nord Frost
    } else {
        Style::default().fg(Color::DarkGray)
    };

    spans.push(Span::styled(
        "│ ",
        Style::default().fg(Color::Rgb(60, 60, 60)),
    ));
    spans.push(Span::styled(name.to_string(), name_style));
    spans.push(Span::styled(" ", Style::default()));

    Line::from(spans)
}

// v0.13.1: render_diagram_box, get_diagram_type, generate_diagram_content removed (panel simplification)

/// Get realm color from key.
fn realm_color(key: &str) -> Color {
    match key {
        "shared" => COLOR_REALM_SHARED,
        "org" => COLOR_REALM_ORG,
        _ => Color::White,
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

/// Get layer color from key.
fn layer_color(layer: &str) -> Color {
    match layer {
        "config" => Color::Rgb(59, 130, 246),   // Blue
        "locale" => Color::Rgb(236, 72, 153),   // Pink
        "geography" => Color::Rgb(34, 197, 94), // Green
        "knowledge" => COLOR_LAYER_KNOWLEDGE,
        "foundation" => Color::Rgb(168, 85, 247), // Purple
        "structure" => Color::Rgb(59, 130, 246),  // Blue
        "semantic" => COLOR_LAYER_SEMANTIC,
        "instruction" => Color::Rgb(181, 137, 0), // Gold
        "output" => COLOR_LAYER_OUTPUT,
        _ => Color::White,
    }
}

/// Get arc scope color.
fn scope_color(scope: &str) -> Color {
    match scope {
        "intra_realm" => Color::Rgb(42, 161, 152), // Cyan
        "cross_realm" => Color::Rgb(249, 115, 22), // Orange
        _ => Color::White,
    }
}

/// Get cardinality color.
fn cardinality_color(cardinality: &str) -> Color {
    match cardinality {
        "one_to_one" | "1:1" => Color::Rgb(34, 197, 94), // Green
        "one_to_many" | "1:N" => Color::Rgb(59, 130, 246), // Blue
        "many_to_one" | "N:1" => Color::Rgb(168, 85, 247), // Purple
        "many_to_many" | "N:M" => Color::Rgb(249, 115, 22), // Orange
        _ => Color::White,
    }
}

/// Check if a YAML key should have semantic coloring for its value.
/// Returns Some(color_fn) if the key is semantic, None otherwise.
fn semantic_value_color(key: &str, value: &str) -> Option<Color> {
    let key_trimmed = key.trim().trim_end_matches(':');
    let value_trimmed = value.trim();

    match key_trimmed {
        "realm" => Some(realm_color(value_trimmed)),
        "layer" => Some(layer_color(value_trimmed)),
        "family" => Some(arc_family_color(value_trimmed)),
        "scope" => Some(scope_color(value_trimmed)),
        "cardinality" => Some(cardinality_color(value_trimmed)),
        _ => None,
    }
}

// =============================================================================
// INTERNAL FUNCTIONS
// =============================================================================

/// Generate arc badge lines for ArcClass items.
/// v0.13 Option C: Shows source→target relationship with colored badges.
/// Format: ┌ [Source] ──[:ARC_NAME]──► [Target] ┐
fn generate_arc_badge(app: &App) -> Vec<Line<'static>> {
    use crate::tui::data::TreeItem;

    let mut badge_lines = Vec::new();

    if let Some(TreeItem::ArcClass(family, arc)) = app.current_item() {
        let fc = arc_family_color(&family.key);

        // Get source/target class colors (use layer colors if we can resolve them)
        let source_color = Color::Rgb(136, 192, 208); // Nord Frost (default)
        let target_color = Color::Rgb(163, 190, 140); // Nord Aurora Green (default)

        // Line 1: Source ──[:ARC]──► Target
        badge_lines.push(Line::from(vec![
            Span::styled("┌ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("[{}]", arc.from_class),
                Style::default()
                    .fg(source_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ──[:", Style::default().fg(Color::DarkGray)),
            Span::styled(
                arc.key.clone(),
                Style::default().fg(fc).add_modifier(Modifier::BOLD),
            ),
            Span::styled("]──► ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("[{}]", arc.to_class),
                Style::default()
                    .fg(target_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ┐", Style::default().fg(Color::DarkGray)),
        ]));

        // Line 2: Family + Cardinality badges
        let card_color = cardinality_color(&arc.cardinality);
        badge_lines.push(Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("◇{}", family.key), Style::default().fg(fc)),
            Span::styled(" ", Style::default()),
            Span::styled(
                format!("⊞{}", arc.cardinality),
                Style::default().fg(card_color),
            ),
            Span::styled(" │", Style::default().fg(Color::DarkGray)),
        ]));

        // Line 3: Separator
        badge_lines.push(Line::from(Span::styled(
            "└────────────────────────────────────────┘",
            Style::default().fg(Color::DarkGray),
        )));
    }

    badge_lines
}

/// Render YAML content in a box with given border color and title.
/// v0.13.1: Simplified - shows full YAML with scroll, no collapse/peek.
/// PROPERTIES panel already shows instance properties, so no need for contextual sections.
fn render_yaml_content_in_box(
    f: &mut Frame,
    area: Rect,
    app: &App,
    visible_height: usize,
    border_color: Color,
    title: Line<'static>,
) {
    // Build YAML lines with syntax highlighting
    let mut lines: Vec<Line> = Vec::new();

    // v0.13 Option C: Add arc badge for ArcClass items
    let arc_badge = generate_arc_badge(app);
    let badge_height = arc_badge.len();
    lines.extend(arc_badge);

    // Adjust visible height for badge
    let content_visible_height = visible_height.saturating_sub(badge_height);

    // v0.17.3: Always show YAML content (Instance tab removed, data is in PROPERTIES panel)
    if !app.yaml.content.is_empty() {
        for yaml_line in app
            .yaml
            .content
            .lines()
            .skip(app.yaml.scroll)
            .take(content_visible_height)
        {
            lines.push(highlight_yaml_line(yaml_line));
        }
    } else {
        lines.push(Line::from(Span::styled("No YAML file", STYLE_DIM)));
    }

    // Total lines for scroll indicator
    let total_lines = app.yaml.content.lines().count();

    // Build scroll indicator with directional arrows
    let scroll_hint = scroll_indicator(app.yaml.scroll, total_lines, visible_height);

    let block = Block::default()
        .title(title)
        .title_bottom(Span::styled(scroll_hint, STYLE_DIM))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);

    // Add scrollbar if content exceeds visible area
    // v0.16.4: Use Unicode symbols for visual consistency with other panels
    if total_lines > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state = ScrollbarState::new(total_lines.saturating_sub(visible_height))
            .position(app.yaml.scroll);

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

/// Highlight a YAML line with syntax coloring.
/// v0.13: Enhanced with semantic coloring for realm, layer, trait, family, scope, cardinality.
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
            // v0.13: Semantic coloring for values
            spans.push(highlight_yaml_value_semantic(key, value));
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
            // v0.13: Semantic coloring for values
            spans.push(highlight_yaml_value_semantic(key, value));
        } else {
            spans.push(Span::styled(":", STYLE_YAML_TEXT));
        }
    } else {
        // Plain text
        spans.push(Span::styled(rest.to_string(), STYLE_YAML_TEXT));
    }

    Line::from(spans)
}

/// Highlight a YAML value with semantic coloring if applicable.
/// v0.13: Checks if the key is a semantic key (realm, layer, trait, family, scope, cardinality)
/// and applies the appropriate color from the taxonomy.
fn highlight_yaml_value_semantic(key: &str, value: &str) -> Span<'static> {
    // Check for semantic coloring first
    if let Some(color) = semantic_value_color(key, value) {
        return Span::styled(
            value.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        );
    }

    // Fall back to default value highlighting
    highlight_yaml_value(value)
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

    // =========================================================================
    // v0.13 semantic_value_color tests (Option B)
    // =========================================================================

    #[test]
    fn test_semantic_value_color_realm_shared() {
        let color = semantic_value_color("realm", " shared");
        assert_eq!(color, Some(COLOR_REALM_SHARED));
    }

    #[test]
    fn test_semantic_value_color_realm_org() {
        let color = semantic_value_color("realm", " org");
        assert_eq!(color, Some(COLOR_REALM_ORG));
    }

    #[test]
    fn test_semantic_value_color_layer_semantic() {
        let color = semantic_value_color("layer", " semantic");
        assert_eq!(color, Some(COLOR_LAYER_SEMANTIC));
    }

    #[test]
    fn test_semantic_value_color_layer_output() {
        let color = semantic_value_color("layer", " output");
        assert_eq!(color, Some(COLOR_LAYER_OUTPUT));
    }

    #[test]
    fn test_semantic_value_color_family_ownership() {
        let color = semantic_value_color("family", " ownership");
        assert_eq!(color, Some(COLOR_FAMILY_OWNERSHIP));
    }

    #[test]
    fn test_semantic_value_color_family_semantic() {
        let color = semantic_value_color("family", " semantic");
        assert_eq!(color, Some(COLOR_FAMILY_SEMANTIC));
    }

    #[test]
    fn test_semantic_value_color_scope_intra() {
        let color = semantic_value_color("scope", " intra_realm");
        assert!(color.is_some()); // Should have a color
    }

    #[test]
    fn test_semantic_value_color_scope_cross() {
        let color = semantic_value_color("scope", " cross_realm");
        assert!(color.is_some()); // Should have a color
    }

    #[test]
    fn test_semantic_value_color_cardinality() {
        let color = semantic_value_color("cardinality", " one_to_many");
        assert!(color.is_some()); // Should have a color
    }

    #[test]
    fn test_semantic_value_color_non_semantic_key() {
        let color = semantic_value_color("name", " Page");
        assert_eq!(color, None); // Not a semantic key
    }

    #[test]
    fn test_semantic_value_color_with_colon() {
        // Key might have trailing colon from parsing
        let color = semantic_value_color("realm:", " shared");
        assert_eq!(color, Some(COLOR_REALM_SHARED));
    }

    // =========================================================================
    // v0.13 highlight_yaml_value_semantic tests
    // =========================================================================

    #[test]
    fn test_highlight_yaml_value_semantic_realm() {
        let span = highlight_yaml_value_semantic("realm", " shared");
        assert_eq!(span.content, " shared");
        // Should be bold with COLOR_REALM_SHARED
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_highlight_yaml_value_semantic_fallback() {
        // Non-semantic key should fall back to default highlighting
        let span = highlight_yaml_value_semantic("name", " Page");
        assert_eq!(span.content, " Page");
        // Should be string style (green), not bold
        assert_eq!(span.style, STYLE_YAML_STRING);
    }

    // =========================================================================
    // v0.13 layer_color tests
    // =========================================================================

    #[test]
    fn test_layer_color_knowledge() {
        assert_eq!(layer_color("knowledge"), COLOR_LAYER_KNOWLEDGE);
    }

    #[test]
    fn test_layer_color_semantic() {
        assert_eq!(layer_color("semantic"), COLOR_LAYER_SEMANTIC);
    }

    #[test]
    fn test_layer_color_output() {
        assert_eq!(layer_color("output"), COLOR_LAYER_OUTPUT);
    }

    #[test]
    fn test_layer_color_unknown() {
        assert_eq!(layer_color("unknown"), Color::White);
    }

    // =========================================================================
    // v0.13 cardinality_color tests
    // =========================================================================

    #[test]
    fn test_cardinality_color_one_to_one() {
        let color = cardinality_color("one_to_one");
        assert_eq!(color, Color::Rgb(34, 197, 94)); // Green
    }

    #[test]
    fn test_cardinality_color_one_to_many() {
        let color = cardinality_color("one_to_many");
        assert_eq!(color, Color::Rgb(59, 130, 246)); // Blue
    }

    #[test]
    fn test_cardinality_color_many_to_many() {
        let color = cardinality_color("many_to_many");
        assert_eq!(color, Color::Rgb(249, 115, 22)); // Orange
    }

    // =========================================================================
    // v0.17.3 word_wrap tests
    // =========================================================================

    #[test]
    fn test_word_wrap_short_text() {
        let result = word_wrap("hello world", 50);
        assert_eq!(result, vec!["hello world"]);
    }

    #[test]
    fn test_word_wrap_long_text() {
        let result = word_wrap("the quick brown fox jumps over the lazy dog", 20);
        assert_eq!(
            result,
            vec!["the quick brown fox", "jumps over the lazy", "dog"]
        );
    }

    #[test]
    fn test_word_wrap_with_newlines() {
        let result = word_wrap("first line\nsecond line", 50);
        assert_eq!(result, vec!["first line", "second line"]);
    }

    #[test]
    fn test_word_wrap_empty_string() {
        let result = word_wrap("", 50);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_word_wrap_zero_width() {
        let result = word_wrap("hello", 0);
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_word_wrap_single_long_word() {
        let result = word_wrap("supercalifragilisticexpialidocious", 10);
        // Single word longer than width should not break
        assert_eq!(result, vec!["supercalifragilisticexpialidocious"]);
    }

    #[test]
    fn test_word_wrap_utf8() {
        let result = word_wrap("你好 世界 很高兴见到你", 10);
        // Should handle UTF-8 correctly
        assert!(!result.is_empty());
        // Chars count, not bytes
        assert!(result.iter().all(|line| line.chars().count() <= 12)); // Allow slight overrun for single word
    }

    // =========================================================================
    // v0.17.3 STANDARD_PROPERTIES tests
    // =========================================================================

    #[test]
    fn test_standard_properties_contains_key() {
        assert!(STANDARD_PROPERTIES.contains(&"key"));
    }

    #[test]
    fn test_standard_properties_contains_display_name() {
        assert!(STANDARD_PROPERTIES.contains(&"display_name"));
    }

    #[test]
    fn test_standard_properties_contains_timestamps() {
        assert!(STANDARD_PROPERTIES.contains(&"created_at"));
        assert!(STANDARD_PROPERTIES.contains(&"updated_at"));
    }

    #[test]
    fn test_standard_properties_count() {
        // v0.19.0: 8 standard properties for DATA nodes (ADR-035)
        // key, display_name, content, triggers, node_class, provenance, created_at, updated_at
        assert_eq!(STANDARD_PROPERTIES.len(), 8);
    }

    // =========================================================================
    // v0.17.3 format_timestamp tests
    // =========================================================================

    #[test]
    fn test_format_timestamp_unix_epoch() {
        // Unix epoch (1970-01-01 00:00)
        let result = format_timestamp(0);
        assert_eq!(result, "1970-01-01 00:00");
    }

    #[test]
    fn test_format_timestamp_known_date() {
        // 2024-01-15 12:36:40 UTC = 1705322200 seconds
        let result = format_timestamp(1705322200);
        assert_eq!(result, "2024-01-15 12:36");
    }

    #[test]
    fn test_format_timestamp_milliseconds() {
        // Same timestamp but in milliseconds (> 10_000_000_000)
        let result = format_timestamp(1705322200000);
        assert_eq!(result, "2024-01-15 12:36");
    }

    #[test]
    fn test_format_timestamp_negative() {
        // Negative timestamps should return as-is
        let result = format_timestamp(-1000);
        assert_eq!(result, "-1000");
    }

    #[test]
    fn test_timestamp_properties_list() {
        assert!(TIMESTAMP_PROPERTIES.contains(&"created_at"));
        assert!(TIMESTAMP_PROPERTIES.contains(&"updated_at"));
        assert_eq!(TIMESTAMP_PROPERTIES.len(), 2);
    }

    #[test]
    fn test_standard_properties_order() {
        // v0.17.3: key and display_name should be first, timestamps last
        let key_pos = STANDARD_PROPERTIES
            .iter()
            .position(|p| *p == "key")
            .unwrap();
        let display_name_pos = STANDARD_PROPERTIES
            .iter()
            .position(|p| *p == "display_name")
            .unwrap();
        let created_at_pos = STANDARD_PROPERTIES
            .iter()
            .position(|p| *p == "created_at")
            .unwrap();
        let updated_at_pos = STANDARD_PROPERTIES
            .iter()
            .position(|p| *p == "updated_at")
            .unwrap();

        // Key and display_name should come before timestamps
        assert!(key_pos < created_at_pos);
        assert!(display_name_pos < created_at_pos);
        // created_at and updated_at should be last two
        assert!(created_at_pos >= STANDARD_PROPERTIES.len() - 2);
        assert!(updated_at_pos >= STANDARD_PROPERTIES.len() - 2);
    }

    // =========================================================================
    // v0.17.3 embedded JSON parsing tests
    // =========================================================================

    #[test]
    fn test_render_property_lines_embedded_json_array() {
        let mut lines = Vec::new();
        let json_string =
            r#"[{"type":"text","value":"code QR"},{"type":"title","value":"Code QR"}]"#;
        let value = JsonValue::String(json_string.to_string());
        render_property_lines(&mut lines, "denomination_forms", &value, 80, 0);
        // Should parse and render as YAML array, not as a single string
        // First line should be "denomination_forms:" (the header)
        assert!(
            lines.len() > 1,
            "Should render multiple lines for parsed JSON array"
        );
    }

    #[test]
    fn test_render_property_lines_embedded_json_object() {
        let mut lines = Vec::new();
        let json_string = r#"{"name":"test","value":123}"#;
        let value = JsonValue::String(json_string.to_string());
        render_property_lines(&mut lines, "metadata", &value, 80, 0);
        // Should parse and render as YAML object
        assert!(
            lines.len() > 1,
            "Should render multiple lines for parsed JSON object"
        );
    }

    #[test]
    fn test_render_property_lines_regular_string_not_parsed() {
        let mut lines = Vec::new();
        let regular_string = "This is just a regular string value";
        let value = JsonValue::String(regular_string.to_string());
        render_property_lines(&mut lines, "description", &value, 80, 0);
        // Should render as single line (fits within width)
        assert_eq!(
            lines.len(),
            1,
            "Regular string should render as single line"
        );
    }

    #[test]
    fn test_render_property_lines_invalid_json_not_parsed() {
        let mut lines = Vec::new();
        let invalid_json = "[not valid json{";
        let value = JsonValue::String(invalid_json.to_string());
        render_property_lines(&mut lines, "data", &value, 80, 0);
        // Should fall back to string rendering (doesn't crash)
        assert!(
            !lines.is_empty(),
            "Invalid JSON should still render as string"
        );
    }
}
