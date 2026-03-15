//! JSON value formatting helpers for info panel rendering.
//!
//! Functions for displaying, wrapping, and coloring JSON values
//! in the PROPERTIES panel.

use ratatui::style::Color;

use serde_json::Value as JsonValue;

use super::{
    COLOR_VALUE_NULL, COLOR_VALUE_BOOL, COLOR_VALUE_NUMBER,
    COLOR_VALUE_STRING, COLOR_VALUE_ARRAY, COLOR_VALUE_OBJECT,
};

/// Format a JSON value for display in PROPERTIES panel.
/// Returns a concise string representation.
pub(crate) fn format_json_value(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => s.clone(),
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else {
                format!("[...{} items]", arr.len())
            }
        },
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                format!("{{...{} keys}}", obj.len())
            }
        },
    }
}

/// Wrap a JSON value string across multiple lines.
/// For expanded property display.
/// Returns lines with configurable indent for continuation.
pub(crate) fn wrap_json_value(s: &str, max_width: usize, indent: usize) -> Vec<String> {
    // Early return for empty or short strings (use char count, not byte count)
    let char_count = s.chars().count();
    if char_count <= max_width {
        return vec![s.to_string()];
    }

    // Guard: if max_width is 0, return single line
    if max_width == 0 {
        return vec![s.to_string()];
    }

    let mut lines = Vec::new();
    let indent_str = " ".repeat(indent);
    let mut chars = s.chars().peekable();

    // First line: take up to max_width characters
    let first_line: String = chars.by_ref().take(max_width).collect();
    lines.push(first_line);

    // Subsequent lines: width reduced by indent
    let cont_width = max_width.saturating_sub(indent);
    if cont_width == 0 {
        // Edge case: indent >= max_width, dump remaining as one line
        let remaining: String = chars.collect();
        if !remaining.is_empty() {
            lines.push(format!("{}{}", indent_str, remaining));
        }
        return lines;
    }

    // Process remaining characters in chunks of cont_width
    loop {
        let chunk: String = chars.by_ref().take(cont_width).collect();
        if chunk.is_empty() {
            break;
        }
        lines.push(format!("{}{}", indent_str, chunk));
    }

    lines
}

/// Convert JSON value to display string.
pub(crate) fn json_value_to_display(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => format!("\"{}\"", s),
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else if arr.len() <= 3 {
                format!("[{} items]", arr.len())
            } else {
                format!("[{} items...]", arr.len())
            }
        },
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                format!("{{...{} keys}}", obj.len())
            }
        },
    }
}

/// Get color for JSON value type.
/// Matches yaml_panel.rs json_value_color() for consistency.
pub(crate) fn json_value_color(value: &JsonValue) -> Color {
    match value {
        JsonValue::Null => COLOR_VALUE_NULL,
        JsonValue::Bool(_) => COLOR_VALUE_BOOL,
        JsonValue::Number(_) => COLOR_VALUE_NUMBER,
        JsonValue::String(_) => COLOR_VALUE_STRING,
        JsonValue::Array(_) => COLOR_VALUE_ARRAY,
        JsonValue::Object(_) => COLOR_VALUE_OBJECT,
    }
}
