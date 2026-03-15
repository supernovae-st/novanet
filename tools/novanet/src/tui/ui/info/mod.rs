//! Info panel rendering for TUI.
//!
//! v0.20.0: Split into submodules:
//! - `mod.rs`: Types, constants, helpers, render functions
//! - `builders.rs`: Routing dispatcher for tree item → builder
//! - `build_schema.rs`: ClassesSection, ArcsSection, Realm, Layer
//! - `build_class.rs`: Class (NodeClass) content builder
//! - `build_arcs.rs`: ArcFamily, ArcClass content builders
//! - `build_instance.rs`: Instance content builder
//! - `build_groups.rs`: EntityCategory, EntityGroup, empty state

mod builders;
mod build_arcs;
mod build_class;
mod build_groups;
mod build_instance;
mod build_schema;

pub use builders::build_unified_content;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use crate::tui::app::{App, Focus};
use crate::tui::palette;
use crate::tui::unicode::truncate_to_width;

use serde_json::Value as JsonValue;

use super::{STYLE_DIM, STYLE_MUTED};

// =============================================================================
// YAML-STYLE COLORS FOR PROPERTIES
// =============================================================================

/// YAML key style (cyan) - matches SOURCE panel styling.
pub(super) const STYLE_PROP_KEY: Style = Style::new().fg(palette::PROP_KEY);

/// YAML colon style.
pub(super) const STYLE_PROP_COLON: Style = Style::new().fg(Color::Cyan);

/// JSON value colors - match yaml_panel.rs json_value_color().
pub(super) const COLOR_VALUE_NULL: Color = Color::DarkGray;
const COLOR_VALUE_BOOL: Color = palette::VALUE_BOOL;
const COLOR_VALUE_NUMBER: Color = palette::VALUE_NUMBER;
const COLOR_VALUE_STRING: Color = palette::VALUE_STRING;
const COLOR_VALUE_ARRAY: Color = palette::VALUE_ARRAY;
const COLOR_VALUE_OBJECT: Color = palette::VALUE_OBJECT;

// =============================================================================
// SECTION HEADER COLORS
// =============================================================================

/// STANDARD section header - teal (same as shared realm color #2aa198).
/// Standard properties are common/boring - stable teal conveys "foundational".
pub(super) const COLOR_HEADER_STANDARD: Color = palette::SOLARIZED_CYAN;

/// SPECIFIC section header - orange (same as semantic layer color #f97316).
/// Specific properties are unique/interesting - vibrant orange conveys "differentiation".
pub(super) const COLOR_HEADER_SPECIFIC: Color = palette::ORANGE_500;

/// PROVENANCE section header - violet (ADR-042 provenance tracking).
/// Provenance shows data origin and lifecycle - violet conveys "authority/trust".
pub(super) const COLOR_HEADER_PROVENANCE: Color = palette::VIOLET_500;

/// Focused property background - subtle highlight for j/k navigation.
/// Dark blue background that works well with all text colors.
pub(super) const COLOR_PROPERTY_FOCUSED_BG: Color = palette::BG_PROPERTY_FOCUSED;

// =============================================================================
// PROPERTY INDICATOR COLORS (Solarized palette)
// =============================================================================

/// Green checkmark (✓) for properties that have values - Solarized Green #859900
pub(super) const COLOR_STATUS_OK: Color = palette::SOLARIZED_GREEN;

/// Red asterisk (*) for required properties - Solarized Red #dc322f
pub(super) const COLOR_REQUIRED_MARKER: Color = palette::SOLARIZED_RED;

/// Blue type badge [str] - Solarized Blue #268bd2
pub(super) const COLOR_TYPE_STRING: Color = palette::SOLARIZED_BLUE;

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
pub(super) const STANDARD_PROPERTY_NAMES: &[&str] = &[
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
pub(super) fn is_standard_property(name: &str) -> bool {
    STANDARD_PROPERTY_NAMES.contains(&name)
}

// =============================================================================
// PROVENANCE HELPERS (ADR-035)
// =============================================================================

/// Data category derived from provenance.source field.
/// 6 sources mapped to 6 categories with distinct lifecycle properties.
///
///   Source           │ Reseed │ Backup │ Editable │ Color (TUI)
///   ─────────────────┼────────┼────────┼──────────┼────────────
///   seed:schema      │  ✓     │  No    │  No      │ Slate-500
///   seed:immutable   │  ✓     │  No    │  No      │ Green-500
///   seed:locale      │  ✓     │  No    │  No      │ Cyan-500
///   seed:content     │  ✓     │  Yes   │  Yes     │ Blue-500
///   runtime:nika     │  No    │  Yes   │  Yes     │ Orange-500
///   runtime:mcp      │  No    │  Yes   │  Yes     │ Purple-500
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DataCategory {
    Schema,    // seed:schema - regenerable from YAML models
    Immutable, // seed:immutable - static reference data (geography, culture)
    Locale,    // seed:locale - locale knowledge atoms (expressions, patterns)
    Content,   // seed:content - bootstrap content examples (entities, pages)
    Nika,      // runtime:nika - Nika workflow output
    Mcp,       // runtime:mcp - MCP API mutation
}

impl DataCategory {
    /// Parse category from provenance source value.
    /// 6 sources with distinct lifecycle properties.
    pub(crate) fn from_source(source: &str) -> Self {
        match source {
            "seed:schema" => DataCategory::Schema,
            "seed:immutable" => DataCategory::Immutable,
            "seed:locale" => DataCategory::Locale,
            "seed:content" => DataCategory::Content,
            "runtime:nika" => DataCategory::Nika,
            "runtime:mcp" => DataCategory::Mcp,
            // Legacy fallbacks
            s if s.starts_with("seed:schema") => DataCategory::Schema,
            s if s.starts_with("seed:immutable") => DataCategory::Immutable,
            s if s.starts_with("seed:locale") => DataCategory::Locale,
            s if s.starts_with("seed:content") || s.starts_with("content:") => {
                DataCategory::Content
            }
            s if s.starts_with("runtime:nika") || s.starts_with("nika:") => DataCategory::Nika,
            s if s.starts_with("runtime:mcp") || s.starts_with("mcp:") => DataCategory::Mcp,
            _ => DataCategory::Mcp, // Unknown runtime sources default to MCP
        }
    }

    /// Human-readable category name (concise).
    pub(crate) fn label(&self) -> &'static str {
        match self {
            DataCategory::Schema => "Schema",
            DataCategory::Immutable => "Immutable",
            DataCategory::Locale => "Locale",
            DataCategory::Content => "Content",
            DataCategory::Nika => "Nika",
            DataCategory::Mcp => "MCP",
        }
    }

    /// Short description for the category (shown after label).
    pub(crate) fn description(&self) -> &'static str {
        match self {
            DataCategory::Schema => "regenerable from YAML",
            DataCategory::Immutable => "static reference data",
            DataCategory::Locale => "knowledge atoms",
            DataCategory::Content => "bootstrap examples",
            DataCategory::Nika => "workflow output",
            DataCategory::Mcp => "API mutation",
        }
    }

    /// Terminal icon for the category (Unicode).
    /// Consistent with visual-encoding.yaml dual-icon pattern.
    pub(crate) fn icon(&self) -> &'static str {
        match self {
            DataCategory::Schema => "◆",    // filled diamond — structured definitions
            DataCategory::Immutable => "◇",  // outline diamond — fixed reference
            DataCategory::Locale => "◈",     // diamond with dot — locale-specific
            DataCategory::Content => "●",    // filled circle — content data
            DataCategory::Nika => "▶",       // play — workflow execution
            DataCategory::Mcp => "⚡",       // lightning — API mutation
        }
    }

    /// Color for the category badge.
    pub(crate) fn color(&self) -> Color {
        match self {
            DataCategory::Schema => palette::SLATE_500,
            DataCategory::Immutable => palette::GREEN_500,
            DataCategory::Locale => palette::CYAN_500,
            DataCategory::Content => palette::BLUE_500,
            DataCategory::Nika => palette::ORANGE_500,
            DataCategory::Mcp => palette::PURPLE_500,
        }
    }

    /// Whether this data survives reseed (seed sources are regenerable).
    pub(crate) fn reseed_safe(&self) -> bool {
        matches!(
            self,
            DataCategory::Schema
                | DataCategory::Immutable
                | DataCategory::Locale
                | DataCategory::Content
        )
    }

    /// Whether this data needs backup (content + runtime data is unique).
    pub(crate) fn needs_backup(&self) -> bool {
        matches!(
            self,
            DataCategory::Content | DataCategory::Nika | DataCategory::Mcp
        )
    }

    /// Whether this data is editable by users.
    pub(crate) fn is_editable(&self) -> bool {
        matches!(
            self,
            DataCategory::Content | DataCategory::Nika | DataCategory::Mcp
        )
    }
}

/// Parse provenance JSON for display in the TUI info panel.
/// Unified provenance object with tagged union on `source`.
///
/// Structure depends on source:
///   seed:*       → { source, version, file? }
///   runtime:nika → { source, version, workflow_id?, task_id?, provider?, model?, generated_at? }
///   runtime:mcp  → { source, version, tool?, user? }
#[derive(Default)]
pub(crate) struct ProvenanceMeta {
    pub(crate) source: Option<String>,
    pub(crate) version: Option<String>,
    // Seed fields
    pub(crate) file: Option<String>,
    // Nika fields
    pub(crate) workflow_id: Option<String>,
    pub(crate) task_id: Option<String>,
    pub(crate) provider: Option<String>,
    pub(crate) model: Option<String>,
    pub(crate) generated_at: Option<String>,
    // MCP fields
    pub(crate) tool: Option<String>,
    pub(crate) user: Option<String>,
}

impl ProvenanceMeta {
    /// Parse from provenance JSON value.
    pub(crate) fn from_json(value: &JsonValue) -> Self {
        let obj = value.as_object();
        let get_str = |key: &str| -> Option<String> {
            obj.and_then(|o| o.get(key))
                .and_then(|v| v.as_str())
                .map(String::from)
        };
        Self {
            source: get_str("source"),
            version: get_str("version"),
            file: get_str("file"),
            workflow_id: get_str("workflow_id"),
            task_id: get_str("task_id"),
            provider: get_str("provider"),
            model: get_str("model"),
            generated_at: get_str("generated_at"),
            tool: get_str("tool"),
            user: get_str("user"),
        }
    }

    /// Check if this contains Nika generation details.
    fn is_nika_generated(&self) -> bool {
        self.workflow_id.is_some() || self.task_id.is_some()
    }

    /// Check if this is an MCP mutation with extra details.
    fn is_mcp_mutation(&self) -> bool {
        self.tool.is_some() || self.user.is_some()
    }
}

/// Build provenance section content from unified provenance property.
/// Consolidated from `created_by` + `created_by_meta` into single `provenance` JSON object.
///
/// Display layout:
/// ```text
///   Source        ◆ seed:schema          (icon + source in category color)
///   Category      Schema — regenerable from YAML
///   Version       v0.19.0
///   Lifecycle     ✓Reseed  ○Backup  🔒Edit
///   File          48-page-block-qr-code.cypher
/// ```
pub(super) fn build_provenance_section(provenance: Option<&JsonValue>) -> SectionContent<'static> {
    let mut section = SectionContent::default();

    // Parse provenance — handle both JSON object and JSON-encoded string
    let meta = match provenance {
        Some(prov) => match prov {
            // Direct JSON object (normal case)
            JsonValue::Object(_) => ProvenanceMeta::from_json(prov),
            // JSON-encoded string (legacy fallback: provenance stored as string)
            JsonValue::String(s) => {
                if let Ok(parsed) = serde_json::from_str::<JsonValue>(s) {
                    ProvenanceMeta::from_json(&parsed)
                } else {
                    // Plain string, not JSON — treat as raw source
                    ProvenanceMeta {
                        source: Some(s.clone()),
                        ..Default::default()
                    }
                }
            }
            // Null or other types
            _ => {
                section.add_line(Line::from(vec![
                    Span::styled("  Source       ", STYLE_DIM),
                    Span::styled(
                        "⚠ missing",
                        Style::default().fg(palette::YELLOW_500),
                    ),
                ]));
                return section;
            }
        },
        None => {
            section.add_line(Line::from(vec![
                Span::styled("  Source       ", STYLE_DIM),
                Span::styled(
                    "⚠ missing",
                    Style::default().fg(palette::YELLOW_500),
                ),
            ]));
            return section;
        }
    };

    // Get source from provenance.source field
    let source = match &meta.source {
        Some(s) if !s.is_empty() => s.as_str(),
        _ => {
            section.add_line(Line::from(vec![
                Span::styled("  Source       ", STYLE_DIM),
                Span::styled(
                    "⚠ missing source",
                    Style::default().fg(palette::YELLOW_500),
                ),
            ]));
            return section;
        }
    };

    let category = DataCategory::from_source(source);

    // Source line: icon + source in category color
    section.add_line(Line::from(vec![
        Span::styled("  Source       ", STYLE_DIM),
        Span::styled(
            format!("{} ", category.icon()),
            Style::default().fg(category.color()),
        ),
        Span::styled(source.to_string(), Style::default().fg(category.color())),
    ]));

    // Category line: label + description
    section.add_line(Line::from(vec![
        Span::styled("  Category     ", STYLE_DIM),
        Span::styled(
            category.label(),
            Style::default()
                .fg(category.color())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" — {}", category.description()),
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    // Version line (if present)
    if let Some(ref version) = meta.version {
        section.add_line(Line::from(vec![
            Span::styled("  Version      ", STYLE_DIM),
            Span::styled(version.clone(), Style::default().fg(Color::DarkGray)),
        ]));
    }

    // Lifecycle badges line
    let reseed_badge = if category.reseed_safe() {
        Span::styled("✓Reseed", Style::default().fg(palette::GREEN_500))
    } else {
        Span::styled("⚠Reseed", Style::default().fg(palette::RED_500))
    };

    let backup_badge = if category.needs_backup() {
        Span::styled("●Backup", Style::default().fg(palette::ORANGE_500))
    } else {
        Span::styled("○Backup", Style::default().fg(Color::DarkGray))
    };

    let edit_badge = if category.is_editable() {
        Span::styled("✎Edit", Style::default().fg(palette::BLUE_500))
    } else {
        Span::styled("🔒Edit", Style::default().fg(Color::DarkGray))
    };

    section.add_line(Line::from(vec![
        Span::styled("  Lifecycle    ", STYLE_DIM),
        reseed_badge,
        Span::styled("  ", Style::default()),
        backup_badge,
        Span::styled("  ", Style::default()),
        edit_badge,
    ]));

    // Seed source file (if present)
    if let Some(ref file) = meta.file {
        section.add_line(Line::from(vec![
            Span::styled("  File         ", STYLE_DIM),
            Span::styled(file.clone(), Style::default().fg(Color::DarkGray)),
        ]));
    }

    // If Nika-generated, show generation details
    if meta.is_nika_generated() {
        section.add_line(Line::from(Span::styled(
            "  ─── Generation Details ───",
            Style::default()
                .fg(COLOR_HEADER_PROVENANCE)
                .add_modifier(Modifier::DIM),
        )));

        if let Some(ref wf) = meta.workflow_id {
            let task_str = meta.task_id.as_deref().unwrap_or("");
            section.add_line(Line::from(vec![
                Span::styled("  Workflow     ", STYLE_DIM),
                Span::styled(wf.clone(), Style::default().fg(Color::Cyan)),
                if !task_str.is_empty() {
                    Span::styled(
                        format!(" ({})", task_str),
                        Style::default().fg(Color::DarkGray),
                    )
                } else {
                    Span::styled("", Style::default())
                },
            ]));
        }

        if let (Some(provider), Some(model)) = (&meta.provider, &meta.model) {
            section.add_line(Line::from(vec![
                Span::styled("  Provider     ", STYLE_DIM),
                Span::styled(
                    format!("{}/{}", provider, model),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
        }

        if let Some(ref ts) = meta.generated_at {
            section.add_line(Line::from(vec![
                Span::styled("  Generated    ", STYLE_DIM),
                Span::styled(ts.clone(), Style::default().fg(Color::DarkGray)),
            ]));
        }
    }

    // If MCP mutation, show mutation details
    if meta.is_mcp_mutation() {
        section.add_line(Line::from(Span::styled(
            "  ─── MCP Details ───",
            Style::default()
                .fg(COLOR_HEADER_PROVENANCE)
                .add_modifier(Modifier::DIM),
        )));

        if let Some(ref tool) = meta.tool {
            section.add_line(Line::from(vec![
                Span::styled("  Tool         ", STYLE_DIM),
                Span::styled(tool.clone(), Style::default().fg(Color::Cyan)),
            ]));
        }

        if let Some(ref user) = meta.user {
            section.add_line(Line::from(vec![
                Span::styled("  User         ", STYLE_DIM),
                Span::styled(user.clone(), Style::default().fg(Color::Yellow)),
            ]));
        }
    }

    section
}

// =============================================================================
// PROPERTY RENDERING HELPERS
// =============================================================================

/// Property type for type badge rendering.
#[derive(Clone, Copy)]
pub(super) enum PropType {
    String,
    DateTime,
    List,
}

impl PropType {
    pub(super) fn badge(&self) -> &'static str {
        match self {
            PropType::String => "[str]",
            PropType::DateTime => "[dt]",
            PropType::List => "[list]",
        }
    }

    pub(super) fn color(&self) -> Color {
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
pub(super) fn render_property_line(name: &str, is_required: bool, prop_type: PropType) -> Line<'static> {
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

// =============================================================================
// VISUAL STATES FOR BOX NAVIGATION
// =============================================================================

/// Border color for unfocused boxes (dim gray - panel not active)
const BOX_BORDER_UNFOCUSED: Color = palette::NORD_BORDER_UNFOCUSED;

/// Border color for focused but not selected boxes (light gray - panel active, other box selected)
const BOX_BORDER_FOCUSED: Color = palette::NORD_BORDER_FOCUSED;

/// Border color for selected box (cyan bright - active box for copy/scroll)
const BOX_BORDER_SELECTED: Color = Color::Cyan;

// =============================================================================
// UNIFIED SECTION TYPES
// =============================================================================

/// Content for a single info section.
/// Each section has a title and content lines.
/// Empty sections display "—" as content.
/// Made public for render optimization.
#[derive(Default)]
pub struct SectionContent<'a> {
    pub lines: Vec<Line<'a>>,
}

impl<'a> SectionContent<'a> {
    pub(super) fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub(super) fn add_line(&mut self, line: Line<'a>) {
        self.lines.push(line);
    }

    pub(super) fn add_kv(&mut self, key: &str, value: Span<'a>) {
        self.lines.push(Line::from(vec![
            Span::styled(format!("{:<10} ", key), STYLE_DIM),
            value,
        ]));
    }

    /// Add a classification entry with explicit key:value format.
    /// Format: `key: icon value` (e.g., `realm: ◎ org`)
    /// Uses narrower 8-char width for compact CLASSIFICATION section.
    pub(super) fn add_classification(&mut self, key: &str, icon: &str, value: &str, color: Color) {
        self.lines.push(Line::from(vec![
            Span::styled(format!("{:<8}", format!("{}:", key)), STYLE_DIM),
            Span::styled(format!("{} ", icon), Style::default().fg(color)),
            Span::styled(value.to_string(), Style::default().fg(color)),
        ]));
    }

    pub(super) fn add_empty(&mut self) {
        self.lines.push(Line::from(Span::styled("—", STYLE_DIM)));
    }
}

/// Unified info content with 7 fixed sections.
/// All sections are always present; empty sections show "—".
/// Made public for render optimization.
/// Added PROVENANCE section (ADR-042).
#[derive(Default)]
pub struct UnifiedContent<'a> {
    /// IDENTITY: type, category, key, class
    pub identity: SectionContent<'a>,
    /// LOCATION: realm, layer
    pub location: SectionContent<'a>,
    /// METRICS: counts, totals, budgets
    pub metrics: SectionContent<'a>,
    /// COVERAGE: property fill rates, health bars
    pub coverage: SectionContent<'a>,
    /// PROVENANCE: data origin, category, lifecycle (ADR-042)
    pub provenance: SectionContent<'a>,
    /// PROPERTIES: property list with values/schema
    pub properties: SectionContent<'a>,
    /// RELATIONSHIPS: arcs, pipeline context
    pub relationships: SectionContent<'a>,
}

// =============================================================================
// HELPER FUNCTIONS (local to this module)
// =============================================================================

/// Convert property type to short badge for schema overlay.
/// All badges are exactly 4 characters for consistent column alignment.
pub(super) fn type_badge(prop_type: &str) -> &'static str {
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
pub(super) fn type_color(prop_type: &str) -> Color {
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

/// Safely truncate a UTF-8 string to N terminal columns (not chars).
/// Appends "..." if truncated. Handles CJK, emoji, and combining characters.
pub(super) fn truncate_str(s: &str, max_width: usize) -> String {
    truncate_to_width(s, max_width)
}

/// Format a JSON value for display in PROPERTIES panel.
/// Returns a concise string representation.
pub(super) fn format_json_value(value: &JsonValue) -> String {
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
pub(super) fn wrap_json_value(s: &str, max_width: usize, indent: usize) -> Vec<String> {
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
pub(super) fn json_value_to_display(value: &JsonValue) -> String {
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
pub(super) fn json_value_color(value: &JsonValue) -> Color {
    match value {
        JsonValue::Null => COLOR_VALUE_NULL,
        JsonValue::Bool(_) => COLOR_VALUE_BOOL,
        JsonValue::Number(_) => COLOR_VALUE_NUMBER,
        JsonValue::String(_) => COLOR_VALUE_STRING,
        JsonValue::Array(_) => COLOR_VALUE_ARRAY,
        JsonValue::Object(_) => COLOR_VALUE_OBJECT,
    }
}


// =============================================================================
// UNIFIED INFO PANEL RENDERING
// =============================================================================

/// Visual state for a box in the info panel.
#[derive(Clone, Copy, PartialEq, Eq)]
enum BoxVisualState {
    /// Panel not active
    Unfocused,
    /// Panel active, but this box is not selected
    /// NOTE: Reserved for future sub-panel focus tracking.
    #[allow(dead_code)]
    Focused,
    /// This box is selected (active for copy/scroll)
    Selected,
}

/// Get border color and title style based on visual state.
fn box_styles(state: BoxVisualState) -> (Color, Style) {
    match state {
        BoxVisualState::Unfocused => (BOX_BORDER_UNFOCUSED, STYLE_DIM),
        BoxVisualState::Focused => (BOX_BORDER_FOCUSED, STYLE_MUTED),
        BoxVisualState::Selected => (
            BOX_BORDER_SELECTED,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    }
}

/// Render a scrollable section box with scroll indicator.
fn render_scrollable_section_box(
    f: &mut Frame,
    area: Rect,
    title: &str,
    content: &SectionContent,
    state: BoxVisualState,
    scroll_offset: usize,
) -> usize {
    let lines: Vec<Line> = if content.is_empty() {
        vec![Line::from(Span::styled("—", STYLE_DIM))]
    } else {
        content.lines.clone()
    };

    let total_lines = lines.len();
    // Inner height = area height - 2 (for borders)
    let visible_height = area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_height);
    let clamped_scroll = scroll_offset.min(max_scroll);

    let (border_color, base_title_style) = box_styles(state);

    // Show scroll indicator in title if scrollable
    let scroll_info = if total_lines > visible_height {
        format!(" [{}/{}]", clamped_scroll + 1, total_lines)
    } else {
        String::new()
    };

    // Selected box gets a ▶ indicator
    let title_text = if state == BoxVisualState::Selected {
        format!(" ▶ {}{} ", title, scroll_info)
    } else {
        format!(" {}{} ", title, scroll_info)
    };

    let block = Block::default()
        .title(Span::styled(title_text, base_title_style))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((clamped_scroll as u16, 0));

    f.render_widget(paragraph, area);

    // Render scrollbar if content exceeds visible area
    if total_lines > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state = ScrollbarState::new(max_scroll).position(clamped_scroll);

        // Scrollbar area is inside the block, right edge
        let scrollbar_area = Rect {
            x: area.x + area.width - 2,
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };

        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    // Return total lines for scroll calculation in app
    total_lines
}

/// Compute visual state for a box in the Detail panel.
/// Focus enum is the source of truth for panel selection.
fn detail_box_state(panel_focused: bool) -> BoxVisualState {
    if panel_focused {
        BoxVisualState::Selected
    } else {
        BoxVisualState::Unfocused
    }
}

// Dead galaxy theme block deleted (v0 cleanup): build_bottom_row, build_content_row,
// build_location_badges, build_metric_cards, render_header_box, render_unified_info_panel
// ~550 lines removed — replaced by render_identity_panel in v0.18.3 4-panel refactor.
/// Render the properties panel [3] in the right column.
/// Render the separated Properties panel.
/// Accepts pre-built content to avoid double-building.
pub fn render_props_panel(f: &mut Frame, area: Rect, app: &mut App, content: &UnifiedContent) {
    // Props panel focused when Focus::Props
    let panel_focused = app.focus == Focus::Props;

    // Render the PROPERTIES section as a scrollable panel
    let props_state = detail_box_state(panel_focused);
    let total_lines = render_scrollable_section_box(
        f,
        area,
        "PROPERTIES [3]",
        &content.properties,
        props_state,
        app.props_scroll,
    );
    app.props_line_count = total_lines;
}
