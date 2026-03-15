//! Property rendering for instance nodes.
//!
//! Renders Neo4j node properties in YAML-like format with:
//! - Type-aware coloring (strings, numbers, booleans, null)
//! - Recursive object/array rendering
//! - Word-wrapping for long strings
//! - Embedded JSON detection and expansion
//! - Timestamp formatting for created_at/updated_at

use std::collections::BTreeMap;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use serde_json::Value as JsonValue;

use super::colors::{
    COLOR_SECTION_HEADER, COLOR_YAML_BOOL, COLOR_YAML_KEY, COLOR_YAML_NUMBER, COLOR_YAML_STRING,
};
use super::utils::{format_timestamp, word_wrap};
use crate::tui::palette;
use crate::tui::widgets::bordered_block;

use ratatui::style::Color;

// =============================================================================
// STANDARD PROPERTIES (ADR-044)
// =============================================================================

/// Standard properties for ALL nodes (both DATA and SCHEMA).
/// These 8 properties appear at the top of every node panel.
/// node_class case indicates type: PascalCase=DATA, lowercase=SCHEMA
const STANDARD_PROPERTIES: &[&str] = &[
    "key",
    "display_name",
    "node_class",
    "content",
    "triggers",
    "provenance",
    "created_at",
    "updated_at",
];

/// Timestamp property names for human-readable formatting.
const TIMESTAMP_PROPERTIES: &[&str] = &["created_at", "updated_at"];

// =============================================================================
// INSTANCE INFO RENDERING
// =============================================================================

/// Render content for instances - shows properties in YAML-like format.
/// Enhanced display with STANDARD/SPECIFIC grouping, word-wrap, multi-line YAML.
#[allow(clippy::too_many_arguments)]
pub(super) fn render_instance_info(
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
    let panel_width = area.width.saturating_sub(4) as usize;

    // YAML-like header with class info
    lines.push(Line::from(vec![
        Span::styled("# ", Style::default().fg(palette::SOLARIZED_VIOLET)),
        Span::styled(
            format!("{} instance", class_name),
            Style::default().fg(palette::SOLARIZED_VIOLET),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("# ", Style::default().fg(palette::SOLARIZED_VIOLET)),
        Span::styled(
            format!("realm: {} | layer: {}", realm, layer),
            Style::default().fg(palette::SOLARIZED_VIOLET),
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
        let standard_len = standard_props.len();
        for (idx, (key, value)) in standard_props.iter().enumerate() {
            render_property_lines_with_timestamp(&mut lines, key, value, panel_width, 0);
            if idx < standard_len - 1 {
                lines.push(Line::from(""));
            }
        }
    }

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
            let specific_len = specific_props.len();
            for (idx, (key, value)) in specific_props.iter().enumerate() {
                render_property_lines_with_timestamp(&mut lines, key, value, panel_width, 0);
                if idx < specific_len - 1 {
                    lines.push(Line::from(""));
                }
            }
        }
    }

    if properties.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "# No properties loaded",
            Style::default().fg(palette::SOLARIZED_VIOLET),
        )));
    }

    let title = super::build_neo4j_title(selected, instance_key);
    let block = bordered_block(title, border_color);
    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

// =============================================================================
// SECTION SEPARATOR
// =============================================================================

/// Render a section separator line with collapse/expand toggle indicator.
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

// =============================================================================
// PROPERTY RENDERING
// =============================================================================

/// Render a property as one or more lines, handling word-wrap and multi-line YAML.
pub(super) fn render_property_lines(
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
            // Check if string contains embedded JSON
            let trimmed = s.trim();
            if (trimmed.starts_with('[') && trimmed.ends_with(']'))
                || (trimmed.starts_with('{') && trimmed.ends_with('}'))
            {
                if let Ok(parsed) = serde_json::from_str::<JsonValue>(trimmed) {
                    render_property_lines(lines, key, &parsed, width, indent);
                    return;
                }
            }

            let key_prefix_len = indent * 2 + key.chars().count() + 2;
            let available_width = width.saturating_sub(key_prefix_len);

            if s.chars().count() <= available_width && !s.contains('\n') {
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(": ", Style::default().fg(Color::White)),
                    Span::styled(format!("\"{}\"", s), Style::default().fg(COLOR_YAML_STRING)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(indent_str.clone(), Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(": |", Style::default().fg(Color::White)),
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
            if arr.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(": ", Style::default().fg(Color::White)),
                    Span::styled("[]", Style::default().fg(palette::YAML_BRACKET)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(indent_str.clone(), Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(":", Style::default().fg(Color::White)),
                ]));
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
                    Span::styled("{}", Style::default().fg(palette::YAML_BRACKET)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(":", Style::default().fg(Color::White)),
                ]));
                for (k, v) in obj.iter() {
                    render_property_lines(lines, k, v, width, indent + 1);
                }
            }
        },
    }
}

/// Render a property with special handling for timestamps.
fn render_property_lines_with_timestamp(
    lines: &mut Vec<Line<'static>>,
    key: &str,
    value: &JsonValue,
    width: usize,
    indent: usize,
) {
    if TIMESTAMP_PROPERTIES.contains(&key) {
        if let JsonValue::Number(n) = value {
            if let Some(ts) = n.as_i64() {
                let indent_str = "  ".repeat(indent);
                let formatted = format_timestamp(ts);
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled(key.to_string(), Style::default().fg(COLOR_YAML_KEY)),
                    Span::styled(": ", Style::default().fg(Color::White)),
                    Span::styled(formatted, Style::default().fg(palette::NORD_FROST)),
                ]));
                return;
            }
        }
    }

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
            let prefix_len = indent * 2 + 2;
            let available_width = width.saturating_sub(prefix_len);

            if s.chars().count() <= available_width && !s.contains('\n') {
                lines.push(Line::from(vec![
                    Span::styled(indent_str, Style::default()),
                    Span::styled("- ", Style::default().fg(Color::White)),
                    Span::styled(format!("\"{}\"", s), Style::default().fg(COLOR_YAML_STRING)),
                ]));
            } else {
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
                    Span::styled("{}", Style::default().fg(palette::YAML_BRACKET)),
                ]));
            } else {
                let mut first = true;
                for (k, v) in obj.iter() {
                    if first {
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(STANDARD_PROPERTIES.len(), 8);
    }

    #[test]
    fn test_timestamp_properties_list() {
        assert!(TIMESTAMP_PROPERTIES.contains(&"created_at"));
        assert!(TIMESTAMP_PROPERTIES.contains(&"updated_at"));
        assert_eq!(TIMESTAMP_PROPERTIES.len(), 2);
    }

    #[test]
    fn test_standard_properties_order() {
        let key_pos = STANDARD_PROPERTIES.iter().position(|p| *p == "key").unwrap();
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

        assert!(key_pos < created_at_pos);
        assert!(display_name_pos < created_at_pos);
        assert!(created_at_pos >= STANDARD_PROPERTIES.len() - 2);
        assert!(updated_at_pos >= STANDARD_PROPERTIES.len() - 2);
    }

    #[test]
    fn test_render_property_lines_embedded_json_array() {
        let mut lines = Vec::new();
        let json_string =
            r#"[{"type":"text","value":"code QR"},{"type":"title","value":"Code QR"}]"#;
        let value = JsonValue::String(json_string.to_string());
        render_property_lines(&mut lines, "denomination_forms", &value, 80, 0);
        assert!(lines.len() > 1, "Should render multiple lines for parsed JSON array");
    }

    #[test]
    fn test_render_property_lines_embedded_json_object() {
        let mut lines = Vec::new();
        let json_string = r#"{"name":"test","value":123}"#;
        let value = JsonValue::String(json_string.to_string());
        render_property_lines(&mut lines, "metadata", &value, 80, 0);
        assert!(lines.len() > 1, "Should render multiple lines for parsed JSON object");
    }

    #[test]
    fn test_render_property_lines_regular_string_not_parsed() {
        let mut lines = Vec::new();
        let regular_string = "This is just a regular string value";
        let value = JsonValue::String(regular_string.to_string());
        render_property_lines(&mut lines, "description", &value, 80, 0);
        assert_eq!(lines.len(), 1, "Regular string should render as single line");
    }

    #[test]
    fn test_render_property_lines_invalid_json_not_parsed() {
        let mut lines = Vec::new();
        let invalid_json = "[not valid json{";
        let value = JsonValue::String(invalid_json.to_string());
        render_property_lines(&mut lines, "data", &value, 80, 0);
        assert!(!lines.is_empty(), "Invalid JSON should still render as string");
    }
}
