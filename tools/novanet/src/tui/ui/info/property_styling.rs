//! Property styling helpers for info panel rendering.
//!
//! Contains property type badges, colors, standard property detection,
//! and the `render_property_line()` function.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use crate::tui::palette;

use super::{
    COLOR_REQUIRED_MARKER, COLOR_STATUS_OK, COLOR_TYPE_STRING,
    STYLE_PROP_COLON, STYLE_PROP_KEY,
};
use super::super::STYLE_DIM;

// =============================================================================
// STANDARD PROPERTIES (ADR-044)
// =============================================================================

/// Standard properties that ALL nodes have (DATA and SCHEMA).
/// ALL nodes have THE SAME 8 standard properties.
///
/// Order: key -> display_name -> node_class -> content -> triggers -> provenance -> created_at -> updated_at
///
/// ALL NODES (8 props):
/// 1. key            - Unique identifier
/// 2. display_name   - Human-readable label
/// 3. node_class     - Type discriminator (PascalCase=DATA, lowercase=SCHEMA)
/// 4. content        - What this node IS (1-3 sentences)
/// 5. triggers       - Keyword triggers for search/spreading activation
/// 6. provenance     - Data origin {source, version}
/// 7. created_at     - Creation timestamp
/// 8. updated_at     - Last modification timestamp
///
/// Note: Composite keys (*_key) are handled separately via COMPOSITE_KEY_PROPERTIES.
pub(crate) const STANDARD_PROPERTY_NAMES: &[&str] = &[
    "key",
    "display_name",
    "node_class", // PascalCase = DATA node, lowercase = SCHEMA node
    "content",    // Unified name for all nodes
    "triggers",
    "provenance", // {source, version} object
    "created_at",
    "updated_at",
];

/// Check if a property name is a standard property.
/// Only the 8 ADR-044 properties are standard. Composite keys are SPECIFIC.
pub(crate) fn is_standard_property(name: &str) -> bool {
    STANDARD_PROPERTY_NAMES.contains(&name)
}

// =============================================================================
// PROPERTY TYPE RENDERING
// =============================================================================

/// Property type for type badge rendering.
#[derive(Clone, Copy)]
pub(crate) enum PropType {
    String,
    DateTime,
    List,
}

impl PropType {
    pub(crate) fn badge(&self) -> &'static str {
        match self {
            PropType::String => "[str]",
            PropType::DateTime => "[dt]",
            PropType::List => "[list]",
        }
    }

    pub(crate) fn color(&self) -> Color {
        match self {
            PropType::String => COLOR_TYPE_STRING,
            PropType::DateTime => COLOR_TYPE_STRING, // Same color as string
            PropType::List => COLOR_TYPE_STRING,
        }
    }
}

/// Render a property line with status indicator, required marker, name, and type.
///
/// Format: `✓*  name          : [type]`
/// - ✓ = green checkmark (property has value)
/// - * = red asterisk (required property) or space (optional)
pub(crate) fn render_property_line(name: &str, is_required: bool, prop_type: PropType) -> Line<'static> {
    let required_marker = if is_required {
        Span::styled("*", Style::default().fg(COLOR_REQUIRED_MARKER))
    } else {
        Span::styled(" ", STYLE_DIM)
    };

    Line::from(vec![
        Span::styled("✓", Style::default().fg(COLOR_STATUS_OK)),
        required_marker,
        Span::styled(format!("{:14}", name), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled(prop_type.badge(), Style::default().fg(prop_type.color())),
    ])
}

/// Convert property type to short badge for schema overlay.
/// All badges are exactly 4 characters for consistent column alignment.
pub(crate) fn type_badge(prop_type: &str) -> &'static str {
    match prop_type.to_lowercase().as_str() {
        "string" => "str ",
        "json" => "json",
        "enum" => "enum",
        "datetime" => "dt  ",
        "int" | "integer" => "int ",
        "float" | "number" => "num ",
        "bool" | "boolean" => "bool",
        "array" | "list" => "arr ",
        "object" | "map" => "obj ",
        "url" | "uri" => "url ",
        "?" => "?   ", // unknown type from validation
        _ => "··· ",   // fallback for unknown types
    }
}

/// Return semantic color for property type.
pub(crate) fn type_color(prop_type: &str) -> Color {
    match prop_type.to_lowercase().as_str() {
        "string" => palette::SOLARIZED_CYAN,      // cyan/teal - text
        "json" => palette::SOLARIZED_VIOLET,       // violet - complex
        "enum" => palette::SOLARIZED_GOLD,         // yellow - constrained
        "datetime" => palette::SOLARIZED_MAGENTA,  // magenta - temporal
        "int" | "integer" => palette::SOLARIZED_BLUE, // blue - numeric
        "float" | "number" => palette::SOLARIZED_BLUE, // blue - numeric
        "bool" | "boolean" => palette::SOLARIZED_GREEN, // green - binary
        "array" | "list" => palette::SOLARIZED_ORANGE,  // orange - collection
        "object" | "map" => palette::SOLARIZED_RED,     // red - complex
        "url" | "uri" => palette::SOLARIZED_CYAN,  // cyan - reference
        "?" => Color::DarkGray,                    // unknown
        _ => Color::Gray,                          // fallback
    }
}
