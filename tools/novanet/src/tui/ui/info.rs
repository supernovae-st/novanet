//! Info panel rendering for TUI.
//!
//! This module contains all functions related to rendering the Info panel,
//! which displays details about the currently selected tree item.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use crate::tui::app::{App, Focus, InfoBox};
use crate::tui::colors;
use crate::tui::data::{ArcDirection, InstanceInfo, TreeItem};
use crate::tui::schema::ValidationStatus;
use crate::tui::theme::{ColorMode, hex_to_color};
use crate::tui::unicode::truncate_to_width;

use serde_json::Value as JsonValue;

use super::{
    STYLE_ACCENT,
    STYLE_DIM,
    STYLE_HIGHLIGHT,
    STYLE_INFO,
    STYLE_MUTED,
    STYLE_PRIMARY,
    STYLE_SUCCESS,
    arc_family_badge_icon,
};

// =============================================================================
// YAML-STYLE COLORS FOR PROPERTIES
// =============================================================================

/// YAML key style (cyan) - matches SOURCE panel styling.
const STYLE_PROP_KEY: Style = Style::new().fg(Color::Rgb(139, 233, 253)); // Cyan

/// YAML colon style.
const STYLE_PROP_COLON: Style = Style::new().fg(Color::Cyan);

/// JSON value colors - match yaml_panel.rs json_value_color().
const COLOR_VALUE_NULL: Color = Color::DarkGray;
const COLOR_VALUE_BOOL: Color = Color::Rgb(189, 147, 249); // Purple
const COLOR_VALUE_NUMBER: Color = Color::Rgb(249, 226, 175); // Yellow
const COLOR_VALUE_STRING: Color = Color::Rgb(166, 227, 161); // Green
const COLOR_VALUE_ARRAY: Color = Color::Rgb(137, 180, 250); // Blue
const COLOR_VALUE_OBJECT: Color = Color::Rgb(245, 194, 231); // Pink

// =============================================================================
// SECTION HEADER COLORS
// =============================================================================

/// STANDARD section header - teal (same as shared realm color #2aa198).
/// Standard properties are common/boring - stable teal conveys "foundational".
const COLOR_HEADER_STANDARD: Color = Color::Rgb(42, 161, 152);

/// SPECIFIC section header - orange (same as semantic layer color #f97316).
/// Specific properties are unique/interesting - vibrant orange conveys "differentiation".
const COLOR_HEADER_SPECIFIC: Color = Color::Rgb(249, 115, 22);

/// PROVENANCE section header - violet (ADR-042 provenance tracking).
/// Provenance shows data origin and lifecycle - violet conveys "authority/trust".
const COLOR_HEADER_PROVENANCE: Color = Color::Rgb(139, 92, 246); // Violet-500

/// Focused property background - subtle highlight for j/k navigation.
/// Dark blue background that works well with all text colors.
const COLOR_PROPERTY_FOCUSED_BG: Color = Color::Rgb(30, 50, 80);

// =============================================================================
// PROPERTY INDICATOR COLORS (Solarized palette)
// =============================================================================

/// Green checkmark (✓) for properties that have values - Solarized Green #859900
const COLOR_STATUS_OK: Color = Color::Rgb(133, 153, 0);

/// Red asterisk (*) for required properties - Solarized Red #dc322f
const COLOR_REQUIRED_MARKER: Color = Color::Rgb(220, 50, 47);

/// Blue type badge [str] - Solarized Blue #268bd2
const COLOR_TYPE_STRING: Color = Color::Rgb(38, 139, 210);

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
const STANDARD_PROPERTY_NAMES: &[&str] = &[
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
fn is_standard_property(name: &str) -> bool {
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
            DataCategory::Schema => Color::Rgb(100, 116, 139),  // Slate-500
            DataCategory::Immutable => Color::Rgb(34, 197, 94), // Green-500
            DataCategory::Locale => Color::Rgb(6, 182, 212),    // Cyan-500
            DataCategory::Content => Color::Rgb(59, 130, 246),  // Blue-500
            DataCategory::Nika => Color::Rgb(249, 115, 22),     // Orange-500
            DataCategory::Mcp => Color::Rgb(168, 85, 247),      // Purple-500
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
fn build_provenance_section(provenance: Option<&JsonValue>) -> SectionContent<'static> {
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
                        version: None,
                        file: None,
                        workflow_id: None,
                        task_id: None,
                        provider: None,
                        model: None,
                        generated_at: None,
                        tool: None,
                        user: None,
                    }
                }
            }
            // Null or other types
            _ => {
                section.add_line(Line::from(vec![
                    Span::styled("  Source       ", STYLE_DIM),
                    Span::styled(
                        "⚠ missing",
                        Style::default().fg(Color::Rgb(234, 179, 8)), // Yellow-500
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
                    Style::default().fg(Color::Rgb(234, 179, 8)), // Yellow-500
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
                    Style::default().fg(Color::Rgb(234, 179, 8)), // Yellow-500
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
        Span::styled("✓Reseed", Style::default().fg(Color::Rgb(34, 197, 94))) // Green
    } else {
        Span::styled("⚠Reseed", Style::default().fg(Color::Rgb(239, 68, 68))) // Red
    };

    let backup_badge = if category.needs_backup() {
        Span::styled("●Backup", Style::default().fg(Color::Rgb(249, 115, 22))) // Orange
    } else {
        Span::styled("○Backup", Style::default().fg(Color::DarkGray))
    };

    let edit_badge = if category.is_editable() {
        Span::styled("✎Edit", Style::default().fg(Color::Rgb(59, 130, 246))) // Blue
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
enum PropType {
    String,
    DateTime,
    List,
}

impl PropType {
    fn badge(&self) -> &'static str {
        match self {
            PropType::String => "[str]",
            PropType::DateTime => "[dt]",
            PropType::List => "[list]",
        }
    }

    fn color(&self) -> Color {
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
fn render_property_line(name: &str, is_required: bool, prop_type: PropType) -> Line<'static> {
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
const BOX_BORDER_UNFOCUSED: Color = Color::Rgb(59, 66, 82); // #3B4252

/// Border color for focused but not selected boxes (light gray - panel active, other box selected)
const BOX_BORDER_FOCUSED: Color = Color::Rgb(76, 86, 106); // #4C566A

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
    fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    fn add_line(&mut self, line: Line<'a>) {
        self.lines.push(line);
    }

    fn add_kv(&mut self, key: &str, value: Span<'a>) {
        self.lines.push(Line::from(vec![
            Span::styled(format!("{:<10} ", key), STYLE_DIM),
            value,
        ]));
    }

    /// Add a classification entry with explicit key:value format.
    /// Format: `key: icon value` (e.g., `realm: ◎ org`)
    /// Uses narrower 8-char width for compact CLASSIFICATION section.
    fn add_classification(&mut self, key: &str, icon: &str, value: &str, color: Color) {
        self.lines.push(Line::from(vec![
            Span::styled(format!("{:<8}", format!("{}:", key)), STYLE_DIM),
            Span::styled(format!("{} ", icon), Style::default().fg(color)),
            Span::styled(value.to_string(), Style::default().fg(color)),
        ]));
    }

    fn add_empty(&mut self) {
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
    /// LOCATION: realm, layer, trait
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
// UNIFIED CONTENT BUILDERS
// =============================================================================

/// Build unified content for the current tree item.
/// Returns all 6 sections populated with appropriate content.
/// Made public for single-build optimization in mod.rs.
pub fn build_unified_content(app: &App) -> UnifiedContent<'static> {
    match app.current_item() {
        Some(TreeItem::ClassesSection) => build_classes_section_content(app),
        Some(TreeItem::ArcsSection) => build_arcs_section_content(app),
        Some(TreeItem::Realm(realm)) => build_realm_content(app, realm),
        Some(TreeItem::Layer(realm, layer)) => build_layer_content(app, realm, layer),
        Some(TreeItem::Class(realm, layer, class)) => build_class_content(app, realm, layer, class),
        Some(TreeItem::ArcFamily(family)) => build_arc_family_content(family),
        Some(TreeItem::ArcClass(family, arc_class)) => build_arc_class_content(family, arc_class),
        Some(TreeItem::Instance(realm, layer, class, instance)) => {
            build_instance_content(app, realm, layer, class, instance)
        },
        Some(TreeItem::EntityCategory(_, _, _, cat)) => build_category_content(cat),
        Some(TreeItem::LocaleGroup(_, _, _, group)) => build_locale_group_content(group),
        // EntityGroup shows parent Entity as INSTANCE panel
        // Look up Entity instance and class to render with full INSTANCE layout
        Some(TreeItem::EntityGroup(_, _, _, group)) => {
            if let Some((entity_realm, entity_layer, entity_class)) = app.tree.find_class("Entity")
            {
                if let Some(instances) = app.tree.instances.get("Entity") {
                    if let Some(entity_instance) =
                        instances.iter().find(|i| i.key == group.entity_key)
                    {
                        return build_instance_content(
                            app,
                            entity_realm,
                            entity_layer,
                            entity_class,
                            entity_instance,
                        );
                    }
                }
            }
            // Fallback to custom content if lookup fails
            build_entity_group_content(app, group)
        },
        // EntityNativeItem shows as INSTANCE panel with full layout
        // Create InstanceInfo from EntityNativeInfo for consistent rendering
        Some(TreeItem::EntityNativeItem(realm, layer, class, native)) => {
            // Compute property stats for consistent INSTANCE panel display
            let filled = native.properties.len();
            let total = class.properties.len();
            let missing_required = class
                .required_properties
                .iter()
                .filter(|k| !native.properties.contains_key(*k))
                .count();

            // Create an InstanceInfo from EntityNativeInfo for consistent INSTANCE panel
            let instance = InstanceInfo {
                key: native.key.clone(),
                display_name: native.display_name.clone(),
                class_key: class.key.clone(),
                properties: native.properties.clone(),
                outgoing_arcs: vec![], // TODO: Load arcs from Neo4j
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: missing_required,
                filled_properties: filled,
                total_properties: total,
                entity_slug: None, // EntityNative doesn't have entity_slug
                relationship_power: 0,
            };
            build_instance_content(app, realm, layer, class, &instance)
        },
        None => build_empty_content(),
    }
}

/// Build content for ClassesSection.
fn build_classes_section_content(app: &App) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("Section", STYLE_ACCENT));
    content
        .identity
        .add_kv("name", Span::styled("Node Classes", STYLE_PRIMARY));

    // LOCATION - not applicable
    content.location.add_empty();

    // METRICS
    let class_count: usize = app
        .tree
        .realms
        .iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.classes.len())
        .sum();
    content.metrics.add_kv(
        "realms",
        Span::styled(app.tree.realms.len().to_string(), STYLE_PRIMARY),
    );
    content.metrics.add_kv(
        "classes",
        Span::styled(class_count.to_string(), STYLE_PRIMARY),
    );

    // COVERAGE - realm distribution
    if class_count > 0 {
        let bar_width = 16usize;
        for realm in &app.tree.realms {
            let realm_classes: usize = realm.layers.iter().map(|l| l.classes.len()).sum();
            let percent = (realm_classes as f64 / class_count as f64 * 100.0).round() as u8;
            let filled = (realm_classes * bar_width) / class_count.max(1);
            let bar = "█".repeat(filled.max(1));
            let empty = "░".repeat(bar_width.saturating_sub(filled));

            content.coverage.add_line(Line::from(vec![
                Span::styled(
                    format!("{:8} ", realm.display_name),
                    Style::default().fg(app.theme.realm_color(&realm.key)),
                ),
                Span::styled(bar, Style::default().fg(app.theme.realm_color(&realm.key))),
                Span::styled(empty, STYLE_DIM),
                Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
            ]));
        }
    } else {
        content.coverage.add_empty();
    }

    // PROPERTIES - not applicable
    content.properties.add_empty();

    // RELATIONSHIPS
    content.relationships.add_line(Line::from(Span::styled(
        "h/l to collapse/expand",
        STYLE_DIM,
    )));

    content
}

/// Build content for ArcsSection.
fn build_arcs_section_content(app: &App) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("Section", STYLE_HIGHLIGHT));
    content
        .identity
        .add_kv("name", Span::styled("Arcs", STYLE_PRIMARY));

    // LOCATION - not applicable
    content.location.add_empty();

    // METRICS
    let arc_count: usize = app
        .tree
        .arc_families
        .iter()
        .map(|f| f.arc_classes.len())
        .sum();
    content.metrics.add_kv(
        "families",
        Span::styled(app.tree.arc_families.len().to_string(), STYLE_PRIMARY),
    );
    content
        .metrics
        .add_kv("arcs", Span::styled(arc_count.to_string(), STYLE_PRIMARY));

    // COVERAGE - not applicable
    content.coverage.add_empty();

    // PROPERTIES - not applicable
    content.properties.add_empty();

    // RELATIONSHIPS
    content.relationships.add_line(Line::from(Span::styled(
        "h/l to collapse/expand",
        STYLE_DIM,
    )));

    content
}

/// Build content for Realm.
fn build_realm_content(app: &App, realm: &crate::tui::data::RealmInfo) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();
    let theme = &app.theme;
    let class_count: usize = realm.layers.iter().map(|l| l.classes.len()).sum();

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("Realm", STYLE_ACCENT));
    content
        .identity
        .add_kv("key", Span::styled(realm.key.clone(), STYLE_PRIMARY));
    content.identity.add_kv(
        "display",
        Span::styled(
            realm.display_name.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    );

    // LOCATION - explicit classification with key:value
    let realm_color = hex_to_color(&realm.color);
    content
        .location
        .add_classification("realm", realm.icon, &realm.key, realm_color);

    // METRICS
    content.metrics.add_kv(
        "layers",
        Span::styled(realm.layers.len().to_string(), STYLE_PRIMARY),
    );
    content.metrics.add_kv(
        "classes",
        Span::styled(class_count.to_string(), STYLE_PRIMARY),
    );

    // COVERAGE - layer breakdown
    if class_count > 0 {
        let bar_width = 12usize;
        for layer in &realm.layers {
            let count = layer.classes.len();
            if count == 0 {
                continue;
            }
            let percent = (count as f64 / class_count as f64 * 100.0).round() as u8;
            let filled = (count * bar_width) / class_count.max(1);
            let bar = "█".repeat(filled.max(1));
            let empty = "░".repeat(bar_width.saturating_sub(filled));
            let layer_color = theme.layer_color(&layer.key);

            content.coverage.add_line(Line::from(vec![
                Span::styled(
                    format!("{:12} ", layer.display_name),
                    Style::default().fg(layer_color),
                ),
                Span::styled(bar, Style::default().fg(layer_color)),
                Span::styled(empty, STYLE_DIM),
                Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
            ]));
        }
    } else {
        content.coverage.add_empty();
    }

    // PROPERTIES - v0.19.0: Show 8 standard properties (ADR-044)
    // Standard: key, display_name, node_class, content, triggers, provenance, created_at, updated_at
    content.properties.add_line(Line::from(vec![Span::styled(
        format!("── STANDARD ({}) ──", 8),
        Style::default().fg(COLOR_HEADER_STANDARD),
    )]));
    content
        .properties
        .add_line(render_property_line("key", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("display_name", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("node_class", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("content", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("triggers", true, PropType::List));
    content
        .properties
        .add_line(render_property_line("provenance", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("created_at", true, PropType::DateTime));
    content
        .properties
        .add_line(render_property_line("updated_at", true, PropType::DateTime));

    // Realm has NO specific properties per ADR-044
    // Only the 8 standard properties exist on Realm nodes

    // RELATIONSHIPS - v0.17: show HAS_LAYER arcs to layers
    if !realm.layers.is_empty() {
        let layer_count = realm.layers.len();
        content.relationships.add_line(Line::from(vec![
            Span::styled(
                "→ ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("{} outgoing", layer_count), STYLE_MUTED),
        ]));

        // Show HAS_LAYER arcs to each layer
        for layer in realm.layers.iter().take(6) {
            let layer_color = hex_to_color(&layer.color);
            content.relationships.add_line(Line::from(vec![
                Span::styled(
                    "  → ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("HAS_LAYER", Style::default().fg(Color::Cyan)),
                Span::styled(" → ", STYLE_DIM),
                Span::styled(layer.display_name.clone(), Style::default().fg(layer_color)),
                Span::styled(" [own]", STYLE_DIM),
            ]));
        }
        if layer_count > 6 {
            content.relationships.add_line(Line::from(vec![Span::styled(
                format!("     ... +{} more", layer_count - 6),
                STYLE_DIM,
            )]));
        }
    } else {
        content.relationships.add_empty();
    }

    content
}

/// Build content for Layer.
fn build_layer_content(
    app: &App,
    realm: &crate::tui::data::RealmInfo,
    layer: &crate::tui::data::LayerInfo,
) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();
    let theme = &app.theme;

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("Layer", STYLE_SUCCESS));
    content
        .identity
        .add_kv("key", Span::styled(layer.key.clone(), STYLE_PRIMARY));
    content.identity.add_kv(
        "display",
        Span::styled(
            layer.display_name.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    );

    // LOCATION (Classification)
    let realm_color = hex_to_color(&realm.color);
    let layer_color = hex_to_color(&layer.color);

    content
        .location
        .add_classification("realm", realm.icon, &realm.key, realm_color);
    content.location.add_classification(
        "layer",
        theme.icons.layer(&layer.key),
        &layer.key,
        layer_color,
    );

    // METRICS
    content.metrics.add_kv(
        "classes",
        Span::styled(layer.classes.len().to_string(), STYLE_PRIMARY),
    );

    if !layer.classes.is_empty() {
        content.coverage.add_line(Line::from(vec![
            Span::styled("◆ ", STYLE_PRIMARY),
            Span::styled(format!("{} classes", layer.classes.len()), STYLE_PRIMARY),
        ]));
    } else {
        content.coverage.add_empty();
    }

    // PROPERTIES - v0.19.0: Show 8 standard properties (ADR-044)
    // Standard: key, display_name, node_class, content, triggers, provenance, created_at, updated_at
    content.properties.add_line(Line::from(vec![Span::styled(
        format!("── STANDARD ({}) ──", 8),
        Style::default().fg(COLOR_HEADER_STANDARD),
    )]));
    content
        .properties
        .add_line(render_property_line("key", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("display_name", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("node_class", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("content", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("triggers", true, PropType::List));
    content
        .properties
        .add_line(render_property_line("provenance", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("created_at", true, PropType::DateTime));
    content
        .properties
        .add_line(render_property_line("updated_at", true, PropType::DateTime));

    // Layer has NO specific properties per ADR-044
    // Only the 8 standard properties exist on Layer nodes

    // RELATIONSHIPS - v0.17: show incoming HAS_LAYER + outgoing HAS_CLASS
    let class_count = layer.classes.len();

    // Summary line
    content.relationships.add_line(Line::from(vec![
        Span::styled(
            "← ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("1 in  ", STYLE_MUTED),
        Span::styled(
            "→ ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!("{} out", class_count), STYLE_MUTED),
    ]));

    // Incoming: HAS_LAYER from Realm
    let realm_color = hex_to_color(&realm.color);
    content.relationships.add_line(Line::from(vec![
        Span::styled(
            "  ← ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("HAS_LAYER", Style::default().fg(Color::Magenta)),
        Span::styled(" ← ", STYLE_DIM),
        Span::styled(realm.display_name.clone(), Style::default().fg(realm_color)),
        Span::styled(" [own]", STYLE_DIM),
    ]));

    // Outgoing: HAS_CLASS to each class
    for class_info in layer.classes.iter().take(4) {
        content.relationships.add_line(Line::from(vec![
            Span::styled(
                "  → ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("HAS_CLASS", Style::default().fg(Color::Cyan)),
            Span::styled(" → ", STYLE_DIM),
            Span::styled(
                class_info.display_name.clone(),
                Style::default().fg(layer_color),
            ),
            Span::styled(" [own]", STYLE_DIM),
        ]));
    }
    if class_count > 4 {
        content.relationships.add_line(Line::from(vec![Span::styled(
            format!("     ... +{} more classes", class_count - 4),
            STYLE_DIM,
        )]));
    }

    content
}

/// Build content for Class (NodeClass).
fn build_class_content(
    app: &App,
    realm: &crate::tui::data::RealmInfo,
    layer: &crate::tui::data::LayerInfo,
    class: &crate::tui::data::ClassInfo,
) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();
    let theme = &app.theme;
    let mode = ColorMode::TrueColor; // TrueColor for semantic colors

    // Get semantic colors from colors.generated.rs
    let realm_color = colors::realm::color(&realm.key, mode);
    let layer_color = colors::layer::color(&layer.key, mode);

    // IDENTITY - clean explicit key:value format (no inline badges)
    content
        .identity
        .add_kv("type", Span::styled("Class", STYLE_INFO));
    content
        .identity
        .add_kv("key", Span::styled(class.key.clone(), STYLE_PRIMARY));
    content.identity.add_kv(
        "display",
        Span::styled(
            class.display_name.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    );

    // LOCATION (Classification) - realm and layer only
    content
        .location
        .add_classification("realm", realm.icon, &realm.key, realm_color);
    content.location.add_classification(
        "layer",
        theme.icons.layer(&layer.key),
        &layer.key,
        layer_color,
    );

    // METRICS
    content.metrics.add_kv(
        "instances",
        Span::styled(format!("{} total", class.instance_count), STYLE_MUTED),
    );
    content.metrics.add_kv(
        "properties",
        Span::styled(format!("{} defined", class.properties.len()), STYLE_INFO),
    );
    if !class.context_budget.is_empty() {
        content.metrics.add_kv(
            "budget",
            Span::styled(class.context_budget.clone(), STYLE_INFO),
        );
    }

    // COVERAGE - property coverage
    let total_props = class.properties.len();
    let required_props = class.required_properties.len();
    let optional_props = total_props.saturating_sub(required_props);

    if total_props > 0 {
        let bar_width = 12usize;
        // Required bar
        let req_percent = (required_props as f64 / total_props as f64 * 100.0).round() as u8;
        let req_filled = (required_props * bar_width) / total_props.max(1);
        let req_bar = "█".repeat(req_filled.max(if required_props > 0 { 1 } else { 0 }));
        let req_empty = "░".repeat(bar_width.saturating_sub(req_filled));

        content.coverage.add_line(Line::from(vec![
            Span::styled("* ", Style::default().fg(Color::Red)),
            Span::styled("required   ", Style::default().fg(Color::Yellow)),
            Span::styled(req_bar, Style::default().fg(Color::Yellow)),
            Span::styled(req_empty, STYLE_DIM),
            Span::styled(format!(" {:>3}%", req_percent), STYLE_MUTED),
            Span::styled(format!("  {}", required_props), STYLE_DIM),
        ]));

        // Optional bar
        let opt_percent = (optional_props as f64 / total_props as f64 * 100.0).round() as u8;
        let opt_filled = (optional_props * bar_width) / total_props.max(1);
        let opt_bar = "█".repeat(opt_filled.max(if optional_props > 0 { 1 } else { 0 }));
        let opt_empty = "░".repeat(bar_width.saturating_sub(opt_filled));

        content.coverage.add_line(Line::from(vec![
            Span::styled("  ", STYLE_DIM),
            Span::styled("optional   ", Style::default().fg(Color::White)),
            Span::styled(opt_bar, Style::default().fg(Color::White)),
            Span::styled(opt_empty, STYLE_DIM),
            Span::styled(format!(" {:>3}%", opt_percent), STYLE_MUTED),
            Span::styled(format!("  {}", optional_props), STYLE_DIM),
        ]));
    } else {
        content.coverage.add_empty();
    }

    // PROPERTIES - v0.13.1: YAML-style with cyan keys, type badges, section headers
    // Split into STANDARD and SPECIFIC sections (same as Instance view)
    if let Some(validated) = &app.schema_overlay.validated_class_properties {
        // Split into standard vs specific
        let standard_props: Vec<_> = validated
            .iter()
            .filter(|p| is_standard_property(&p.name))
            .collect();
        let specific_props: Vec<_> = validated
            .iter()
            .filter(|p| !is_standard_property(&p.name))
            .collect();

        // Calculate max property name length for colon alignment
        let max_name_len = validated
            .iter()
            .map(|p| p.name.len())
            .max()
            .unwrap_or(0)
            .min(18); // Cap at 18 chars for readability

        // STANDARD section header (teal)
        if !standard_props.is_empty() {
            content.properties.add_line(Line::from(vec![Span::styled(
                format!("── STANDARD ({}) ──", standard_props.len()),
                Style::default().fg(COLOR_HEADER_STANDARD),
            )]));

            for prop in &standard_props {
                let (status_icon, status_style) = match prop.status {
                    ValidationStatus::Sync => ("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
                    ValidationStatus::Missing => {
                        ("⚠", Style::default().fg(Color::Rgb(203, 75, 22)))
                    },
                    ValidationStatus::Extra => ("?", STYLE_DIM),
                };
                let required_marker = if prop.required { "*" } else { " " };
                let badge = type_badge(&prop.prop_type);
                let badge_color = type_color(&prop.prop_type);

                content.properties.add_line(Line::from(vec![
                    Span::styled(status_icon, status_style),
                    Span::styled(
                        required_marker,
                        Style::default().fg(Color::Rgb(220, 50, 47)),
                    ),
                    Span::styled(
                        format!("{:width$}", prop.name, width = max_name_len),
                        STYLE_PROP_KEY,
                    ),
                    Span::styled(": ", STYLE_PROP_COLON),
                    Span::styled(
                        format!("[{}]", badge.trim()),
                        Style::default().fg(badge_color),
                    ),
                ]));
            }
        }

        // SPECIFIC section header (orange)
        if !specific_props.is_empty() {
            content.properties.add_line(Line::from(vec![Span::styled(
                format!("── SPECIFIC ({}) ──", specific_props.len()),
                Style::default().fg(COLOR_HEADER_SPECIFIC),
            )]));

            for prop in &specific_props {
                let (status_icon, status_style) = match prop.status {
                    ValidationStatus::Sync => ("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
                    ValidationStatus::Missing => {
                        ("⚠", Style::default().fg(Color::Rgb(203, 75, 22)))
                    },
                    ValidationStatus::Extra => ("?", STYLE_DIM),
                };
                let required_marker = if prop.required { "*" } else { " " };
                let badge = type_badge(&prop.prop_type);
                let badge_color = type_color(&prop.prop_type);

                content.properties.add_line(Line::from(vec![
                    Span::styled(status_icon, status_style),
                    Span::styled(
                        required_marker,
                        Style::default().fg(Color::Rgb(220, 50, 47)),
                    ),
                    Span::styled(
                        format!("{:width$}", prop.name, width = max_name_len),
                        STYLE_PROP_KEY,
                    ),
                    Span::styled(": ", STYLE_PROP_COLON),
                    Span::styled(
                        format!("[{}]", badge.trim()),
                        Style::default().fg(badge_color),
                    ),
                ]));
            }
        }
    } else if !class.properties.is_empty() {
        // Fallback: simple property list with section headers
        let standard_props: Vec<_> = class
            .properties
            .iter()
            .filter(|p| is_standard_property(p))
            .collect();
        let specific_props: Vec<_> = class
            .properties
            .iter()
            .filter(|p| !is_standard_property(p))
            .collect();

        let max_name_len = class
            .properties
            .iter()
            .map(|p| p.len())
            .max()
            .unwrap_or(0)
            .min(18);

        // STANDARD section header (teal)
        if !standard_props.is_empty() {
            content.properties.add_line(Line::from(vec![Span::styled(
                format!("── STANDARD ({}) ──", standard_props.len()),
                Style::default().fg(COLOR_HEADER_STANDARD),
            )]));

            for prop in &standard_props {
                let is_required = class.required_properties.contains(*prop);
                let marker = if is_required { "*" } else { " " };

                content.properties.add_line(Line::from(vec![
                    Span::styled("  ", STYLE_DIM),
                    Span::styled(marker, Style::default().fg(Color::Rgb(220, 50, 47))),
                    Span::styled(
                        format!("{:width$}", prop, width = max_name_len),
                        STYLE_PROP_KEY,
                    ),
                    Span::styled(": ", STYLE_PROP_COLON),
                    Span::styled(
                        if is_required { "[req]" } else { "[opt]" },
                        if is_required {
                            Style::default().fg(Color::Rgb(181, 137, 0))
                        } else {
                            STYLE_DIM
                        },
                    ),
                ]));
            }
        }

        // SPECIFIC section header (orange)
        if !specific_props.is_empty() {
            content.properties.add_line(Line::from(vec![Span::styled(
                format!("── SPECIFIC ({}) ──", specific_props.len()),
                Style::default().fg(COLOR_HEADER_SPECIFIC),
            )]));

            for prop in &specific_props {
                let is_required = class.required_properties.contains(*prop);
                let marker = if is_required { "*" } else { " " };

                content.properties.add_line(Line::from(vec![
                    Span::styled("  ", STYLE_DIM),
                    Span::styled(marker, Style::default().fg(Color::Rgb(220, 50, 47))),
                    Span::styled(
                        format!("{:width$}", prop, width = max_name_len),
                        STYLE_PROP_KEY,
                    ),
                    Span::styled(": ", STYLE_PROP_COLON),
                    Span::styled(
                        if is_required { "[req]" } else { "[opt]" },
                        if is_required {
                            Style::default().fg(Color::Rgb(181, 137, 0))
                        } else {
                            STYLE_DIM
                        },
                    ),
                ]));
            }
        }
    } else {
        content.properties.add_empty();
    }

    // RELATIONSHIPS - v0.13: with arc family colors from Neo4j data
    if let Some(arcs_data) = &app.details.class_arcs {
        // Use Neo4j arc data with family-based colors
        let outgoing_count = arcs_data.outgoing.len();
        let incoming_count = arcs_data.incoming.len();

        content.relationships.add_line(Line::from(vec![
            Span::styled(
                "→ ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("{} out  ", outgoing_count), STYLE_MUTED),
            Span::styled(
                "← ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("{} in", incoming_count), STYLE_MUTED),
        ]));

        // Show outgoing arcs with family colors
        for arc in arcs_data.outgoing.iter().take(4) {
            let family_color = colors::arc_family::color(&arc.family, mode);
            content.relationships.add_line(Line::from(vec![
                Span::styled(
                    "  → ",
                    Style::default()
                        .fg(family_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(arc.arc_key.clone(), Style::default().fg(family_color)),
                Span::styled(" → ", STYLE_DIM),
                Span::styled(arc.other_class.clone(), STYLE_HIGHLIGHT),
                Span::styled(
                    format!(" [{}]", arc.family),
                    Style::default()
                        .fg(family_color)
                        .add_modifier(Modifier::DIM),
                ),
            ]));
        }
        if outgoing_count > 4 {
            content.relationships.add_line(Line::from(vec![Span::styled(
                format!("     ... +{} more outgoing", outgoing_count - 4),
                STYLE_DIM,
            )]));
        }

        // Show incoming arcs with family colors
        for arc in arcs_data.incoming.iter().take(3) {
            let family_color = colors::arc_family::color(&arc.family, mode);
            content.relationships.add_line(Line::from(vec![
                Span::styled(
                    "  ← ",
                    Style::default()
                        .fg(family_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(arc.arc_key.clone(), Style::default().fg(family_color)),
                Span::styled(" ← ", STYLE_DIM),
                Span::styled(arc.other_class.clone(), STYLE_HIGHLIGHT),
                Span::styled(
                    format!(" [{}]", arc.family),
                    Style::default()
                        .fg(family_color)
                        .add_modifier(Modifier::DIM),
                ),
            ]));
        }
        if incoming_count > 3 {
            content.relationships.add_line(Line::from(vec![Span::styled(
                format!("     ... +{} more incoming", incoming_count - 3),
                STYLE_DIM,
            )]));
        }
    } else if !class.arcs.is_empty() {
        // Fallback: use schema arcs without family colors
        let outgoing_count = class
            .arcs
            .iter()
            .filter(|a| a.direction == ArcDirection::Outgoing)
            .count();
        let incoming_count = class
            .arcs
            .iter()
            .filter(|a| a.direction == ArcDirection::Incoming)
            .count();

        content.relationships.add_line(Line::from(vec![
            Span::styled("→ ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{} out  ", outgoing_count), STYLE_MUTED),
            Span::styled("← ", Style::default().fg(Color::Magenta)),
            Span::styled(format!("{} in", incoming_count), STYLE_MUTED),
        ]));

        for arc in class.arcs.iter().take(6) {
            let (icon, color) = if arc.direction == ArcDirection::Outgoing {
                ("→", Color::Cyan)
            } else {
                ("←", Color::Magenta)
            };
            content.relationships.add_line(Line::from(vec![
                Span::styled(format!("  {} ", icon), Style::default().fg(color)),
                Span::styled(arc.arc_type.clone(), Style::default().fg(color)),
                Span::styled(" → ", STYLE_DIM),
                Span::styled(arc.target_class.clone(), STYLE_HIGHLIGHT),
            ]));
        }
        if class.arcs.len() > 6 {
            content.relationships.add_line(Line::from(vec![Span::styled(
                format!("  ... +{} more", class.arcs.len() - 6),
                STYLE_DIM,
            )]));
        }
    } else {
        content.relationships.add_empty();
    }

    content
}

/// Build content for ArcFamily.
fn build_arc_family_content(family: &crate::tui::data::ArcFamilyInfo) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();
    let mode = ColorMode::TrueColor; // Semantic colors

    // Get family-specific color
    let family_color = colors::arc_family::color(&family.key, mode);

    // IDENTITY - explicit key:value format
    content.identity.add_kv(
        "type",
        Span::styled("ArcFamily", Style::default().fg(family_color)),
    );
    content.identity.add_kv(
        "key",
        Span::styled(
            family.key.clone(),
            Style::default()
                .fg(family_color)
                .add_modifier(Modifier::BOLD),
        ),
    );
    content.identity.add_kv(
        "display",
        Span::styled(
            family.display_name.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    );

    // LOCATION - not applicable
    content.location.add_empty();

    // METRICS
    content.metrics.add_kv(
        "arcs",
        Span::styled(family.arc_classes.len().to_string(), STYLE_PRIMARY),
    );

    // COVERAGE - not applicable
    content.coverage.add_empty();

    // PROPERTIES - v0.19.0: Show 8 standard properties (ADR-044)
    // Same pattern as Realm/Layer for display coherence
    content.properties.add_line(Line::from(vec![Span::styled(
        format!("── STANDARD ({}) ──", 8),
        Style::default().fg(COLOR_HEADER_STANDARD),
    )]));
    content
        .properties
        .add_line(render_property_line("key", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("display_name", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("node_class", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("content", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("triggers", true, PropType::List));
    content
        .properties
        .add_line(render_property_line("provenance", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("created_at", true, PropType::DateTime));
    content
        .properties
        .add_line(render_property_line("updated_at", true, PropType::DateTime));

    // RELATIONSHIPS
    content.relationships.add_line(Line::from(Span::styled(
        "h/l to collapse/expand",
        STYLE_DIM,
    )));

    content
}

/// Build content for ArcClass.
fn build_arc_class_content(
    family: &crate::tui::data::ArcFamilyInfo,
    arc_class: &crate::tui::data::ArcClassInfo,
) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();
    let mode = ColorMode::TrueColor; // Semantic colors

    // Get family-specific color
    let family_color = colors::arc_family::color(&family.key, mode);

    // IDENTITY - explicit key:value format
    content.identity.add_kv(
        "type",
        Span::styled("ArcClass", Style::default().fg(family_color)),
    );
    content.identity.add_kv(
        "key",
        Span::styled(
            arc_class.key.clone(),
            Style::default()
                .fg(family_color)
                .add_modifier(Modifier::BOLD),
        ),
    );
    content.identity.add_kv(
        "display",
        Span::styled(
            arc_class.display_name.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    );

    // LOCATION (Classification) - family as classification
    content.location.add_classification(
        "family",
        arc_family_badge_icon(&family.key),
        &family.key,
        family_color,
    );

    // METRICS - cardinality
    if !arc_class.cardinality.is_empty() {
        content.metrics.add_kv(
            "cardin.",
            Span::styled(arc_class.cardinality.clone(), STYLE_ACCENT),
        );
    } else {
        content.metrics.add_empty();
    }

    // COVERAGE - not applicable
    content.coverage.add_empty();

    // PROPERTIES - v0.19.0: Show 8 standard properties (ADR-044)
    // Same pattern as Realm/Layer/ArcFamily for display coherence
    content.properties.add_line(Line::from(vec![Span::styled(
        format!("── STANDARD ({}) ──", 8),
        Style::default().fg(COLOR_HEADER_STANDARD),
    )]));
    content
        .properties
        .add_line(render_property_line("key", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("display_name", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("node_class", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("content", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("triggers", true, PropType::List));
    content
        .properties
        .add_line(render_property_line("provenance", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("created_at", true, PropType::DateTime));
    content
        .properties
        .add_line(render_property_line("updated_at", true, PropType::DateTime));

    // SPECIFIC PROPERTIES - ArcClass-specific
    content.properties.add_line(Line::from(vec![Span::styled(
        "── SPECIFIC (3) ──",
        Style::default().fg(COLOR_HEADER_SPECIFIC),
    )]));
    content
        .properties
        .add_line(render_property_line("from_class", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("to_class", true, PropType::String));
    content
        .properties
        .add_line(render_property_line("cardinality", false, PropType::String));

    // RELATIONSHIPS - v0.13: from/to with family color
    content.relationships.add_line(Line::from(vec![
        Span::styled("● ", Style::default().fg(family_color)),
        Span::styled("from ", STYLE_DIM),
        Span::styled(
            arc_class.from_class.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    content.relationships.add_line(Line::from(vec![
        Span::styled(
            "→ ",
            Style::default()
                .fg(family_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(arc_class.key.clone(), Style::default().fg(family_color)),
    ]));
    content.relationships.add_line(Line::from(vec![
        Span::styled("○ ", Style::default().fg(family_color)),
        Span::styled("to   ", STYLE_DIM),
        Span::styled(
            arc_class.to_class.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    content
}

/// Build content for Instance.
fn build_instance_content(
    app: &App,
    realm: &crate::tui::data::RealmInfo,
    layer: &crate::tui::data::LayerInfo,
    class: &crate::tui::data::ClassInfo,
    instance: &InstanceInfo,
) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();
    let theme = &app.theme;
    let mode = ColorMode::TrueColor; // Semantic colors

    // Get semantic colors from colors.generated.rs
    let realm_color = colors::realm::color(&realm.key, mode);
    let layer_color = colors::layer::color(&layer.key, mode);

    // IDENTITY - clean explicit key:value format (no inline badges)
    content
        .identity
        .add_kv("type", Span::styled("Instance", STYLE_HIGHLIGHT));
    content.identity.add_kv(
        "key",
        Span::styled(
            instance.key.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    );
    content.identity.add_kv(
        "class",
        Span::styled(class.display_name.clone(), Style::default().fg(layer_color)),
    );

    // LOCATION (Classification) - realm and layer only
    content
        .location
        .add_classification("realm", realm.icon, &realm.key, realm_color);
    content.location.add_classification(
        "layer",
        theme.icons.layer(&layer.key),
        &layer.key,
        layer_color,
    );

    // METRICS - unified with Class view (same labels, same positions)
    content.metrics.add_kv(
        "instances",
        Span::styled(format!("{} total", class.instance_count), STYLE_MUTED),
    );
    content.metrics.add_kv(
        "properties",
        Span::styled(
            format!(
                "{}/{} filled",
                instance.properties.len(),
                class.properties.len()
            ),
            STYLE_INFO,
        ),
    );
    if !class.context_budget.is_empty() {
        content.metrics.add_kv(
            "budget",
            Span::styled(class.context_budget.clone(), STYLE_INFO),
        );
    }

    // COVERAGE - required/optional bars (unified with Class view)
    let total_props = class.properties.len();
    let required_props = class.required_properties.len();
    let optional_props = total_props.saturating_sub(required_props);

    // Count filled required and optional properties
    let filled_required = instance
        .properties
        .keys()
        .filter(|k| class.required_properties.contains(*k))
        .count();
    let filled_optional = instance
        .properties
        .keys()
        .filter(|k| !class.required_properties.contains(*k) && class.properties.contains(*k))
        .count();

    if total_props > 0 {
        let bar_width = 12usize;

        // Required bar - shows filled/total required
        let req_percent = if required_props > 0 {
            (filled_required as f64 / required_props as f64 * 100.0).round() as u8
        } else {
            0
        };
        let req_filled = if required_props > 0 {
            (filled_required * bar_width) / required_props.max(1)
        } else {
            0
        };
        let req_bar = "█".repeat(req_filled.max(if filled_required > 0 { 1 } else { 0 }));
        let req_empty = "░".repeat(bar_width.saturating_sub(req_filled));

        content.coverage.add_line(Line::from(vec![
            Span::styled("* ", Style::default().fg(Color::Red)),
            Span::styled("required   ", Style::default().fg(Color::Yellow)),
            Span::styled(req_bar, Style::default().fg(Color::Yellow)),
            Span::styled(req_empty, STYLE_DIM),
            Span::styled(format!(" {:>3}%", req_percent), STYLE_MUTED),
            Span::styled(
                format!("  {}/{}", filled_required, required_props),
                STYLE_DIM,
            ),
        ]));

        // Optional bar - shows filled/total optional
        let opt_percent = if optional_props > 0 {
            (filled_optional as f64 / optional_props as f64 * 100.0).round() as u8
        } else {
            0
        };
        let opt_filled = if optional_props > 0 {
            (filled_optional * bar_width) / optional_props.max(1)
        } else {
            0
        };
        let opt_bar = "█".repeat(opt_filled.max(if filled_optional > 0 { 1 } else { 0 }));
        let opt_empty = "░".repeat(bar_width.saturating_sub(opt_filled));

        content.coverage.add_line(Line::from(vec![
            Span::styled("  ", STYLE_DIM),
            Span::styled("optional   ", Style::default().fg(Color::White)),
            Span::styled(opt_bar, Style::default().fg(Color::White)),
            Span::styled(opt_empty, STYLE_DIM),
            Span::styled(format!(" {:>3}%", opt_percent), STYLE_MUTED),
            Span::styled(
                format!("  {}/{}", filled_optional, optional_props),
                STYLE_DIM,
            ),
        ]));
    } else {
        content.coverage.add_empty();
    }

    // PROVENANCE - v0.19.0: ADR-035 unified provenance property
    // Extract provenance JSON object from instance properties
    let provenance = instance.properties.get("provenance");
    content.provenance = build_provenance_section(provenance);

    // PROPERTIES - v0.13.1: Display ALL properties with STANDARD/SPECIFIC sections
    // Format: `✓* property_name: value` with section headers
    let all_schema_keys: Vec<&String> = class
        .properties
        .iter()
        .filter(|k| !k.starts_with('_'))
        .collect();

    // Split into standard vs specific properties
    let standard_keys: Vec<&String> = all_schema_keys
        .iter()
        .filter(|k| is_standard_property(k.as_str()))
        .copied()
        .collect();
    let specific_keys: Vec<&String> = all_schema_keys
        .iter()
        .filter(|k| !is_standard_property(k.as_str()))
        .copied()
        .collect();

    // Collect extra instance props not in schema (e.g., computed fields)
    let extra_keys: Vec<&String> = instance
        .properties
        .keys()
        .filter(|k| !k.starts_with('_') && !all_schema_keys.contains(k))
        .collect();

    if !all_schema_keys.is_empty() || !extra_keys.is_empty() {
        // Same max width as Class (18 chars)
        let max_key_len = all_schema_keys
            .iter()
            .chain(extra_keys.iter())
            .map(|k| k.len())
            .max()
            .unwrap_or(0)
            .min(18); // Same cap as Class for visual alignment

        // Check if properties box is focused for highlighting
        let is_props_focused = app.focus == Focus::Props && app.selected_box == InfoBox::Properties;
        let mut property_idx: usize = 0;

        // STANDARD section (teal header)
        if !standard_keys.is_empty() {
            content.properties.add_line(Line::from(vec![Span::styled(
                format!("── STANDARD ({}) ──", standard_keys.len()),
                Style::default().fg(COLOR_HEADER_STANDARD),
            )]));
            for key in &standard_keys {
                let is_required = class.required_properties.contains(*key);
                let has_value = instance
                    .properties
                    .get(*key)
                    .map(|v| !v.is_null())
                    .unwrap_or(false);

                let (status_icon, status_style) = if has_value {
                    ("✓", Style::default().fg(Color::Rgb(133, 153, 0)))
                } else if is_required {
                    ("⚠", Style::default().fg(Color::Rgb(203, 75, 22)))
                } else {
                    (" ", STYLE_DIM)
                };
                let required_marker = if is_required { "*" } else { " " };

                let (value_str, value_color) = if let Some(value) = instance.properties.get(*key) {
                    (json_value_to_display(value), json_value_color(value))
                } else {
                    ("~".to_string(), COLOR_VALUE_NULL)
                };

                // Standard properties now use same styling as specific for consistency
                // Look up type from validated_class_properties
                let prop_type = app
                    .schema_overlay
                    .validated_class_properties
                    .as_ref()
                    .and_then(|props| props.iter().find(|p| p.name.as_str() == *key))
                    .map(|p| p.prop_type.as_str())
                    .unwrap_or("???");
                let badge = type_badge(prop_type);
                let badge_color = type_color(prop_type);

                // Apply background highlight if this property is focused
                let is_focused = is_props_focused && property_idx == app.focused_property_idx;
                let bg_style = if is_focused {
                    Style::default().bg(COLOR_PROPERTY_FOCUSED_BG)
                } else {
                    Style::default()
                };

                // Wrap long values when expanded
                if is_focused && app.expanded_property && value_str.len() > 24 {
                    // Wrap value across multiple lines
                    let prefix_len = 2 + max_key_len + 8;
                    let wrap_width = 50;
                    let wrapped = wrap_json_value(&value_str, wrap_width, prefix_len);

                    // First line with full prefix
                    content.properties.add_line(Line::from(vec![
                        Span::styled(status_icon, status_style.patch(bg_style)),
                        Span::styled(
                            required_marker,
                            Style::default().fg(Color::Rgb(220, 50, 47)).patch(bg_style),
                        ),
                        Span::styled(
                            format!("{:width$}", key, width = max_key_len),
                            STYLE_PROP_KEY.patch(bg_style),
                        ),
                        Span::styled(": ", STYLE_PROP_COLON.patch(bg_style)),
                        Span::styled(
                            format!("[{}] ", badge.trim()),
                            Style::default().fg(badge_color).patch(bg_style),
                        ),
                        Span::styled(
                            wrapped.first().cloned().unwrap_or_default(),
                            Style::default().fg(value_color).patch(bg_style),
                        ),
                    ]));

                    // Continuation lines
                    for cont_line in wrapped.iter().skip(1) {
                        content.properties.add_line(Line::from(vec![Span::styled(
                            cont_line.clone(),
                            Style::default().fg(value_color).patch(bg_style),
                        )]));
                    }
                } else {
                    // Single line (truncated or short value)
                    let display_value = truncate_str(&value_str, 24);
                    content.properties.add_line(Line::from(vec![
                        Span::styled(status_icon, status_style.patch(bg_style)),
                        Span::styled(
                            required_marker,
                            Style::default().fg(Color::Rgb(220, 50, 47)).patch(bg_style),
                        ),
                        Span::styled(
                            format!("{:width$}", key, width = max_key_len),
                            STYLE_PROP_KEY.patch(bg_style),
                        ),
                        Span::styled(": ", STYLE_PROP_COLON.patch(bg_style)),
                        Span::styled(
                            format!("[{}] ", badge.trim()),
                            Style::default().fg(badge_color).patch(bg_style),
                        ),
                        Span::styled(
                            display_value,
                            Style::default().fg(value_color).patch(bg_style),
                        ),
                    ]));
                }
                property_idx += 1;
            }
        }

        // SPECIFIC section (gold/orange header)
        if !specific_keys.is_empty() {
            content.properties.add_line(Line::from(vec![Span::styled(
                format!("── SPECIFIC ({}) ──", specific_keys.len()),
                Style::default().fg(COLOR_HEADER_SPECIFIC),
            )]));
            for key in &specific_keys {
                let is_required = class.required_properties.contains(*key);
                let has_value = instance
                    .properties
                    .get(*key)
                    .map(|v| !v.is_null())
                    .unwrap_or(false);

                let (status_icon, status_style) = if has_value {
                    ("✓", Style::default().fg(Color::Rgb(133, 153, 0)))
                } else if is_required {
                    ("⚠", Style::default().fg(Color::Rgb(203, 75, 22)))
                } else {
                    (" ", STYLE_DIM)
                };
                let required_marker = if is_required { "*" } else { " " };

                let (value_str, value_color) = if let Some(value) = instance.properties.get(*key) {
                    (json_value_to_display(value), json_value_color(value))
                } else {
                    ("~".to_string(), COLOR_VALUE_NULL)
                };

                // Look up type from validated_class_properties
                let prop_type = app
                    .schema_overlay
                    .validated_class_properties
                    .as_ref()
                    .and_then(|props| props.iter().find(|p| p.name.as_str() == *key))
                    .map(|p| p.prop_type.as_str())
                    .unwrap_or("???");
                let badge = type_badge(prop_type);
                let badge_color = type_color(prop_type);

                // Apply background highlight if this property is focused
                let is_focused = is_props_focused && property_idx == app.focused_property_idx;
                let bg_style = if is_focused {
                    Style::default().bg(COLOR_PROPERTY_FOCUSED_BG)
                } else {
                    Style::default()
                };

                // Wrap long values when expanded
                if is_focused && app.expanded_property && value_str.len() > 24 {
                    let prefix_len = 2 + max_key_len + 8;
                    let wrap_width = 50;
                    let wrapped = wrap_json_value(&value_str, wrap_width, prefix_len);

                    content.properties.add_line(Line::from(vec![
                        Span::styled(status_icon, status_style.patch(bg_style)),
                        Span::styled(
                            required_marker,
                            Style::default().fg(Color::Rgb(220, 50, 47)).patch(bg_style),
                        ),
                        Span::styled(
                            format!("{:width$}", key, width = max_key_len),
                            STYLE_PROP_KEY.patch(bg_style),
                        ),
                        Span::styled(": ", STYLE_PROP_COLON.patch(bg_style)),
                        Span::styled(
                            format!("[{}] ", badge.trim()),
                            Style::default().fg(badge_color).patch(bg_style),
                        ),
                        Span::styled(
                            wrapped.first().cloned().unwrap_or_default(),
                            Style::default().fg(value_color).patch(bg_style),
                        ),
                    ]));

                    for cont_line in wrapped.iter().skip(1) {
                        content.properties.add_line(Line::from(vec![Span::styled(
                            cont_line.clone(),
                            Style::default().fg(value_color).patch(bg_style),
                        )]));
                    }
                } else {
                    let display_value = truncate_str(&value_str, 24);
                    content.properties.add_line(Line::from(vec![
                        Span::styled(status_icon, status_style.patch(bg_style)),
                        Span::styled(
                            required_marker,
                            Style::default().fg(Color::Rgb(220, 50, 47)).patch(bg_style),
                        ),
                        Span::styled(
                            format!("{:width$}", key, width = max_key_len),
                            STYLE_PROP_KEY.patch(bg_style),
                        ),
                        Span::styled(": ", STYLE_PROP_COLON.patch(bg_style)),
                        Span::styled(
                            format!("[{}] ", badge.trim()),
                            Style::default().fg(badge_color).patch(bg_style),
                        ),
                        Span::styled(
                            display_value,
                            Style::default().fg(value_color).patch(bg_style),
                        ),
                    ]));
                }
                property_idx += 1;
            }
        }

        // Extra instance props not in schema (marked with ?)
        for key in &extra_keys {
            if let Some(value) = instance.properties.get(*key) {
                let value_str = json_value_to_display(value);
                let value_color = json_value_color(value);

                // Apply background highlight if this property is focused
                let is_focused = is_props_focused && property_idx == app.focused_property_idx;
                let bg_style = if is_focused {
                    Style::default().bg(COLOR_PROPERTY_FOCUSED_BG)
                } else {
                    Style::default()
                };

                // Wrap long values when expanded
                if is_focused && app.expanded_property && value_str.len() > 24 {
                    let prefix_len = 2 + max_key_len + 2;
                    let wrap_width = 50;
                    let wrapped = wrap_json_value(&value_str, wrap_width, prefix_len);

                    content.properties.add_line(Line::from(vec![
                        Span::styled("?", STYLE_DIM.patch(bg_style)),
                        Span::styled(" ", STYLE_DIM.patch(bg_style)),
                        Span::styled(
                            format!("{:width$}", key, width = max_key_len),
                            STYLE_PROP_KEY.patch(bg_style),
                        ),
                        Span::styled(": ", STYLE_PROP_COLON.patch(bg_style)),
                        Span::styled(
                            wrapped.first().cloned().unwrap_or_default(),
                            Style::default().fg(value_color).patch(bg_style),
                        ),
                    ]));

                    for cont_line in wrapped.iter().skip(1) {
                        content.properties.add_line(Line::from(vec![Span::styled(
                            cont_line.clone(),
                            Style::default().fg(value_color).patch(bg_style),
                        )]));
                    }
                } else {
                    let display_value = truncate_str(&value_str, 24);
                    content.properties.add_line(Line::from(vec![
                        Span::styled("?", STYLE_DIM.patch(bg_style)),
                        Span::styled(" ", STYLE_DIM.patch(bg_style)),
                        Span::styled(
                            format!("{:width$}", key, width = max_key_len),
                            STYLE_PROP_KEY.patch(bg_style),
                        ),
                        Span::styled(": ", STYLE_PROP_COLON.patch(bg_style)),
                        Span::styled(
                            display_value,
                            Style::default().fg(value_color).patch(bg_style),
                        ),
                    ]));
                }
                property_idx += 1;
            }
        }
    } else {
        content.properties.add_empty();
    }

    // RELATIONSHIPS - arc diagram
    let arc_count = instance.outgoing_arcs.len() + instance.incoming_arcs.len();
    if arc_count > 0 {
        content.relationships.add_line(Line::from(vec![
            Span::styled("→ ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} out  ", instance.outgoing_arcs.len()),
                STYLE_MUTED,
            ),
            Span::styled("← ", Style::default().fg(Color::Magenta)),
            Span::styled(format!("{} in", instance.incoming_arcs.len()), STYLE_MUTED),
        ]));

        // Show first few arcs
        for arc in instance.outgoing_arcs.iter().take(4) {
            content.relationships.add_line(Line::from(vec![
                Span::styled("  → ", Style::default().fg(Color::Cyan)),
                Span::styled(arc.arc_type.clone(), STYLE_HIGHLIGHT),
                Span::styled(" → ", STYLE_DIM),
                Span::styled(arc.target_key.clone(), STYLE_PRIMARY),
            ]));
        }
        for arc in instance.incoming_arcs.iter().take(2) {
            content.relationships.add_line(Line::from(vec![
                Span::styled("  ← ", Style::default().fg(Color::Magenta)),
                Span::styled(arc.arc_type.clone(), STYLE_HIGHLIGHT),
                Span::styled(" ← ", STYLE_DIM),
                Span::styled(arc.target_key.clone(), STYLE_PRIMARY),
            ]));
        }
    } else {
        content.relationships.add_empty();
    }

    content
}

/// Build content for EntityCategory.
fn build_category_content(cat: &crate::tui::data::EntityCategory) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("EntityCategory", STYLE_ACCENT));
    content.identity.add_kv(
        "category",
        Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
    );
    content
        .identity
        .add_kv("key", Span::styled(cat.key.clone(), STYLE_PRIMARY));
    content.identity.add_kv(
        "name",
        Span::styled(cat.display_name.clone(), STYLE_PRIMARY),
    );

    // LOCATION - not applicable
    content.location.add_empty();

    // METRICS
    content.metrics.add_kv(
        "entities",
        Span::styled(cat.instance_count.to_string(), STYLE_PRIMARY),
    );

    // COVERAGE - not applicable
    content.coverage.add_empty();

    // PROPERTIES - question + context
    content.properties.add_line(Line::from(vec![
        Span::styled("question: ", STYLE_DIM),
        Span::styled(cat.question.clone(), STYLE_MUTED),
    ]));
    if !cat.content.is_empty() {
        for line in cat.content.lines() {
            content
                .properties
                .add_line(Line::from(Span::styled(format!("  {}", line), STYLE_DIM)));
        }
    }

    // RELATIONSHIPS - not applicable
    content.relationships.add_empty();

    content
}

/// Build content for LocaleGroup (EntityNative grouping by locale).
fn build_locale_group_content(group: &crate::tui::data::LocaleGroup) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("LocaleGroup", STYLE_ACCENT));
    content.identity.add_kv(
        "locale",
        Span::styled(
            format!("{} {}", group.flag, group.locale_code),
            Style::default().fg(Color::Cyan),
        ),
    );
    content.identity.add_kv(
        "name",
        Span::styled(group.locale_name.clone(), STYLE_PRIMARY),
    );

    // LOCATION - not applicable
    content.location.add_empty();

    // METRICS
    content.metrics.add_kv(
        "natives",
        Span::styled(group.instance_count.to_string(), STYLE_PRIMARY),
    );

    // COVERAGE - not applicable
    content.coverage.add_empty();

    // PROPERTIES - not applicable
    content.properties.add_empty();

    // RELATIONSHIPS - not applicable
    content.relationships.add_empty();

    content
}

/// Build content for EntityGroup (EntityNatives grouped by parent Entity).
/// Groups natives by entity instead of locale.
/// Shows the parent Entity's properties and relationships.
fn build_entity_group_content(
    app: &crate::tui::app::App,
    group: &crate::tui::data::EntityNativeGroup,
) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();

    // IDENTITY - show Entity info
    content
        .identity
        .add_kv("type", Span::styled("Entity", STYLE_ACCENT));
    content.identity.add_kv(
        "key",
        Span::styled(group.entity_key.clone(), Style::default().fg(Color::Cyan)),
    );
    content.identity.add_kv(
        "name",
        Span::styled(group.entity_display_name.clone(), STYLE_PRIMARY),
    );

    // Try to find the Entity class info for location
    if let Some((realm, layer, _class)) = app.tree.find_class("Entity") {
        // LOCATION
        content.location.add_kv(
            "realm",
            Span::styled(realm.display_name.clone(), STYLE_ACCENT),
        );
        content.location.add_kv(
            "layer",
            Span::styled(layer.display_name.clone(), STYLE_ACCENT),
        );
        content
            .location
            .add_kv("class", Span::styled("Entity", STYLE_ACCENT));
    } else {
        content.location.add_empty();
    }

    // METRICS
    content.metrics.add_kv(
        "natives",
        Span::styled(group.native_count.to_string(), STYLE_PRIMARY),
    );

    // COVERAGE - not applicable
    content.coverage.add_empty();

    // PROPERTIES - try to find the Entity instance and show its properties
    if let Some(instances) = app.tree.instances.get("Entity") {
        if let Some(entity_instance) = instances.iter().find(|i| i.key == group.entity_key) {
            // Show all properties from the Entity instance
            for (key, value) in &entity_instance.properties {
                let value_str = format_json_value(value);
                content.properties.add_line(Line::from(vec![
                    Span::styled(format!("{}: ", key), STYLE_DIM),
                    Span::styled(value_str, STYLE_MUTED),
                ]));
            }
            if entity_instance.properties.is_empty() {
                content
                    .properties
                    .add_line(Line::from(Span::styled("(no properties)", STYLE_DIM)));
            }
        } else {
            content.properties.add_line(Line::from(Span::styled(
                "(Entity instance not loaded)",
                STYLE_DIM,
            )));
        }
    } else {
        content.properties.add_line(Line::from(Span::styled(
            "(Entity instances not loaded)",
            STYLE_DIM,
        )));
    }

    // RELATIONSHIPS - show arcs to EntityNatives
    content.relationships.add_line(Line::from(vec![
        Span::styled("HAS_NATIVE → ", STYLE_DIM),
        Span::styled(
            format!("{} EntityNatives", group.native_count),
            STYLE_PRIMARY,
        ),
    ]));

    content
}

/// Build empty content for no selection.
fn build_empty_content() -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();
    content
        .identity
        .add_line(Line::from(Span::styled("Select an item", STYLE_DIM)));
    content.location.add_empty();
    content.metrics.add_empty();
    content.coverage.add_empty();
    content.properties.add_empty();
    content.relationships.add_empty();
    content
}

// =============================================================================
// HELPER FUNCTIONS (local to this module)
// =============================================================================

/// Convert property type to short badge for schema overlay.
/// All badges are exactly 4 characters for consistent column alignment.
fn type_badge(prop_type: &str) -> &'static str {
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
fn type_color(prop_type: &str) -> Color {
    match prop_type.to_lowercase().as_str() {
        "string" => Color::Rgb(42, 161, 152),   // cyan/teal - text
        "json" => Color::Rgb(108, 113, 196),    // violet - complex
        "enum" => Color::Rgb(181, 137, 0),      // yellow - constrained
        "datetime" => Color::Rgb(211, 54, 130), // magenta - temporal
        "int" | "integer" => Color::Rgb(38, 139, 210), // blue - numeric
        "float" | "number" => Color::Rgb(38, 139, 210), // blue - numeric
        "bool" | "boolean" => Color::Rgb(133, 153, 0), // green - binary
        "array" | "list" => Color::Rgb(203, 75, 22), // orange - collection
        "object" | "map" => Color::Rgb(220, 50, 47), // red - complex
        "url" | "uri" => Color::Rgb(42, 161, 152), // cyan - reference
        "?" => Color::DarkGray,                 // unknown
        _ => Color::Gray,                       // fallback
    }
}

/// Safely truncate a UTF-8 string to N terminal columns (not chars).
/// Appends "..." if truncated. Handles CJK, emoji, and combining characters.
fn truncate_str(s: &str, max_width: usize) -> String {
    truncate_to_width(s, max_width)
}

/// Format a JSON value for display in PROPERTIES panel.
/// Returns a concise string representation.
fn format_json_value(value: &JsonValue) -> String {
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
fn wrap_json_value(s: &str, max_width: usize, indent: usize) -> Vec<String> {
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
fn json_value_to_display(value: &JsonValue) -> String {
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
fn json_value_color(value: &JsonValue) -> Color {
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
/// Simplified to use only panel_focused (Focus enum is source of truth).
/// Old selected_box parameter is deprecated and ignored.
fn detail_box_state(
    panel_focused: bool,
    _selected_box: InfoBox,
    _this_box: InfoBox,
) -> BoxVisualState {
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
    let selected_box = app.selected_box;

    // Render the PROPERTIES section as a scrollable panel
    let props_state = detail_box_state(panel_focused, selected_box, InfoBox::Properties);
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
