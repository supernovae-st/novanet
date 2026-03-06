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
    STYLE_ACCENT, STYLE_DESC, STYLE_DIM, STYLE_HIGHLIGHT, STYLE_INFO, STYLE_MUTED, STYLE_PRIMARY,
    STYLE_SUCCESS, arc_family_badge_icon, trait_icon, wrap_text,
};

// =============================================================================
// v0.13.1 YAML-STYLE COLORS FOR PROPERTIES
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
// v0.13.1 SECTION HEADER COLORS
// =============================================================================

/// STANDARD section header - teal (same as shared realm color #2aa198).
/// Standard properties are common/boring - stable teal conveys "foundational".
const COLOR_HEADER_STANDARD: Color = Color::Rgb(42, 161, 152);

/// SPECIFIC section header - orange (same as semantic layer color #f97316).
/// Specific properties are unique/interesting - vibrant orange conveys "differentiation".
const COLOR_HEADER_SPECIFIC: Color = Color::Rgb(249, 115, 22);

/// Focused property background - subtle highlight for j/k navigation.
/// Dark blue background that works well with all text colors.
const COLOR_PROPERTY_FOCUSED_BG: Color = Color::Rgb(30, 50, 80);

// =============================================================================
// v0.13.1 STANDARD PROPERTIES (schema-standard.md)
// =============================================================================

/// Standard properties that ALL nodes have (from standard_properties in YAML).
/// Order: key → *_key (denormalized) → display_name → description → created_at → updated_at
const STANDARD_PROPERTY_NAMES: &[&str] = &[
    "key",
    "entity_key",
    "page_key",
    "block_key",
    "locale_key",
    "display_name",
    "description",
    "created_at",
    "updated_at",
];

/// Check if a property name is a standard property.
fn is_standard_property(name: &str) -> bool {
    STANDARD_PROPERTY_NAMES.contains(&name)
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
/// v0.16.4: Made public for render optimization
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

/// Unified info content with 6 fixed sections.
/// All sections are always present; empty sections show "—".
/// v0.16.4: Made public for render optimization
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
/// v0.16.4: Made public for single-build optimization in mod.rs
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
        }
        Some(TreeItem::EntityCategory(_, _, _, cat)) => build_category_content(cat),
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

    // PROPERTIES - v0.17: Show realm schema properties in formatted list
    // Realm has structural properties: key, display_name, color, icon, llm_context
    content.properties.add_line(Line::from(vec![Span::styled(
        format!("── STANDARD ({}) ──", 4),
        Style::default().fg(COLOR_HEADER_STANDARD),
    )]));

    // key property
    content.properties.add_line(Line::from(vec![
        Span::styled("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
        Span::styled("*", Style::default().fg(Color::Rgb(220, 50, 47))),
        Span::styled(format!("{:14}", "key"), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled("[str]", Style::default().fg(Color::Rgb(38, 139, 210))),
    ]));

    // display_name property
    content.properties.add_line(Line::from(vec![
        Span::styled("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
        Span::styled("*", Style::default().fg(Color::Rgb(220, 50, 47))),
        Span::styled(format!("{:14}", "display_name"), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled("[str]", Style::default().fg(Color::Rgb(38, 139, 210))),
    ]));

    // color property
    content.properties.add_line(Line::from(vec![
        Span::styled("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
        Span::styled(" ", STYLE_DIM),
        Span::styled(format!("{:14}", "color"), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled("[str]", Style::default().fg(Color::Rgb(38, 139, 210))),
    ]));

    // icon property
    content.properties.add_line(Line::from(vec![
        Span::styled("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
        Span::styled(" ", STYLE_DIM),
        Span::styled(format!("{:14}", "icon"), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled("[obj]", Style::default().fg(Color::Rgb(181, 137, 0))),
    ]));

    // SPECIFIC section
    content.properties.add_line(Line::from(vec![Span::styled(
        format!("── SPECIFIC ({}) ──", 1),
        Style::default().fg(COLOR_HEADER_SPECIFIC),
    )]));

    // llm_context property
    content.properties.add_line(Line::from(vec![
        Span::styled("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
        Span::styled(" ", STYLE_DIM),
        Span::styled(format!("{:14}", "llm_context"), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled("[str]", Style::default().fg(Color::Rgb(38, 139, 210))),
    ]));

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

    // COVERAGE - trait breakdown
    if !layer.classes.is_empty() {
        let mut trait_counts: std::collections::BTreeMap<String, usize> =
            std::collections::BTreeMap::new();
        for class_info in &layer.classes {
            *trait_counts
                .entry(class_info.trait_name.clone())
                .or_insert(0) += 1;
        }

        let total = layer.classes.len();
        let bar_width = 12usize;
        for (trait_name, count) in &trait_counts {
            let percent = (*count as f64 / total as f64 * 100.0).round() as u8;
            let filled = (*count * bar_width) / total.max(1);
            let bar = "█".repeat(filled.max(1));
            let empty = "░".repeat(bar_width.saturating_sub(filled));
            let icon = trait_icon(trait_name);

            content.coverage.add_line(Line::from(vec![
                Span::styled(
                    format!("{} ", icon),
                    Style::default().fg(theme.trait_color(trait_name)),
                ),
                Span::styled(
                    format!("{:10} ", trait_name),
                    Style::default().fg(theme.trait_color(trait_name)),
                ),
                Span::styled(bar, Style::default().fg(theme.trait_color(trait_name))),
                Span::styled(empty, STYLE_DIM),
                Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
            ]));
        }
    } else {
        content.coverage.add_empty();
    }

    // PROPERTIES - v0.17: Show layer schema properties in formatted list
    // Layer has structural properties: key, display_name, color, llm_context
    content.properties.add_line(Line::from(vec![Span::styled(
        format!("── STANDARD ({}) ──", 3),
        Style::default().fg(COLOR_HEADER_STANDARD),
    )]));

    // key property
    content.properties.add_line(Line::from(vec![
        Span::styled("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
        Span::styled("*", Style::default().fg(Color::Rgb(220, 50, 47))),
        Span::styled(format!("{:14}", "key"), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled("[str]", Style::default().fg(Color::Rgb(38, 139, 210))),
    ]));

    // display_name property
    content.properties.add_line(Line::from(vec![
        Span::styled("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
        Span::styled("*", Style::default().fg(Color::Rgb(220, 50, 47))),
        Span::styled(format!("{:14}", "display_name"), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled("[str]", Style::default().fg(Color::Rgb(38, 139, 210))),
    ]));

    // color property
    content.properties.add_line(Line::from(vec![
        Span::styled("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
        Span::styled(" ", STYLE_DIM),
        Span::styled(format!("{:14}", "color"), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled("[str]", Style::default().fg(Color::Rgb(38, 139, 210))),
    ]));

    // SPECIFIC section
    content.properties.add_line(Line::from(vec![Span::styled(
        format!("── SPECIFIC ({}) ──", 1),
        Style::default().fg(COLOR_HEADER_SPECIFIC),
    )]));

    // llm_context property
    content.properties.add_line(Line::from(vec![
        Span::styled("✓", Style::default().fg(Color::Rgb(133, 153, 0))),
        Span::styled(" ", STYLE_DIM),
        Span::styled(format!("{:14}", "llm_context"), STYLE_PROP_KEY),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled("[str]", Style::default().fg(Color::Rgb(38, 139, 210))),
    ]));

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
        let trait_color = theme.trait_color(&class_info.trait_name);
        content.relationships.add_line(Line::from(vec![
            Span::styled(
                "  → ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("HAS_CLASS", Style::default().fg(Color::Cyan)),
            Span::styled(" → ", STYLE_DIM),
            Span::styled(class_info.display_name.clone(), Style::default().fg(trait_color)),
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
    let mode = ColorMode::TrueColor; // v0.13: Use TrueColor for semantic colors

    // v0.13: Get semantic colors from colors.generated.rs
    let realm_color = colors::realm::color(&realm.key, mode);
    let layer_color = colors::layer::color(&layer.key, mode);
    let trait_color = colors::traits::color(&class.trait_name, mode);

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

    // LOCATION (Classification) - explicit key:value format for all 3 axes
    content
        .location
        .add_classification("realm", realm.icon, &realm.key, realm_color);
    content.location.add_classification(
        "layer",
        theme.icons.layer(&layer.key),
        &layer.key,
        layer_color,
    );
    if !class.trait_name.is_empty() {
        content.location.add_classification(
            "trait",
            trait_icon(&class.trait_name),
            &class.trait_name,
            trait_color,
        );
    }

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
                    }
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
                    }
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
        // v0.13: Use Neo4j arc data with family-based colors
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

        // v0.13: Show outgoing arcs with family colors
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

        // v0.13: Show incoming arcs with family colors
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
    let mode = ColorMode::TrueColor; // v0.13: semantic colors

    // v0.13: Get family-specific color
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

    // PROPERTIES - LLM context
    if !family.llm_context.is_empty() {
        for wrapped_line in wrap_text(&family.llm_context, 38) {
            content
                .properties
                .add_line(Line::from(Span::styled(wrapped_line, STYLE_DESC)));
        }
    } else {
        content.properties.add_empty();
    }

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
    let mode = ColorMode::TrueColor; // v0.13: semantic colors

    // v0.13: Get family-specific color
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

    // PROPERTIES - description
    if !arc_class.description.is_empty() {
        for wrapped_line in wrap_text(&arc_class.description, 38) {
            content
                .properties
                .add_line(Line::from(Span::styled(wrapped_line, STYLE_DESC)));
        }
    } else {
        content.properties.add_empty();
    }

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
    let mode = ColorMode::TrueColor; // v0.13: semantic colors

    // v0.13: Get semantic colors from colors.generated.rs
    let realm_color = colors::realm::color(&realm.key, mode);
    let layer_color = colors::layer::color(&layer.key, mode);
    let trait_color = colors::traits::color(&class.trait_name, mode);

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

    // LOCATION (Classification) - explicit key:value format for all 3 axes
    content
        .location
        .add_classification("realm", realm.icon, &realm.key, realm_color);
    content.location.add_classification(
        "layer",
        theme.icons.layer(&layer.key),
        &layer.key,
        layer_color,
    );
    if !class.trait_name.is_empty() {
        content.location.add_classification(
            "trait",
            trait_icon(&class.trait_name),
            &class.trait_name,
            trait_color,
        );
    }

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

                // v0.16.4: Wrap long values when expanded
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

                // v0.16.4: Wrap long values when expanded
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

                // v0.16.4: Wrap long values when expanded
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
    if !cat.llm_context.is_empty() {
        for line in cat.llm_context.lines() {
            content
                .properties
                .add_line(Line::from(Span::styled(format!("  {}", line), STYLE_DIM)));
        }
    }

    // RELATIONSHIPS - not applicable
    content.relationships.add_empty();

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
// CONSTANTS
// =============================================================================

// v0.13: STYLE_ARC_FAMILY removed - now using colors::arc_family::color() from colors.generated.rs

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

/// v0.13: Return semantic color for property type.
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

/// Wrap a JSON value string across multiple lines.
/// v0.16.4: For expanded property display.
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
        }
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                format!("{{...{} keys}}", obj.len())
            }
        }
    }
}

/// Get color for JSON value type.
/// v0.13.1: Matches yaml_panel.rs json_value_color() for consistency.
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

/// Get detail panel title for current item.
/// v0.13: Not currently used (outer panel removed), kept for future use.
#[allow(dead_code)]
fn get_detail_title(app: &App) -> String {
    match app.current_item() {
        Some(TreeItem::ClassesSection) => "Node Classes".to_string(),
        Some(TreeItem::ArcsSection) => "Arcs".to_string(),
        Some(TreeItem::Realm(r)) => format!("{} {}", r.icon, r.display_name),
        Some(TreeItem::Layer(_, l)) => l.display_name.clone(),
        Some(TreeItem::Class(_, _, k)) => {
            // [C] badge for Class - instant recognition
            if k.icon.is_empty() {
                format!("[C] {}", k.display_name)
            } else {
                format!("[C] {} {}", k.icon, k.display_name)
            }
        }
        Some(TreeItem::ArcFamily(f)) => f.display_name.clone(),
        Some(TreeItem::ArcClass(_, ek)) => ek.display_name.clone(),
        Some(TreeItem::Instance(_, _, _, inst)) => {
            // [I] badge for Instance - instant recognition
            format!("[I] {} ({})", inst.key, inst.class_key)
        }
        Some(TreeItem::EntityCategory(_, _, _, cat)) => {
            // [C] badge for Category
            format!("[C] {}", cat.display_name)
        }
        None => "Detail".to_string(),
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

// =============================================================================
// v0.16.4 WOW EFFECT COLORS - GALAXY THEME
// =============================================================================

/// Nova cyan/teal - primary accent for selection
const COLOR_NOVA_CYAN: Color = Color::Rgb(34, 211, 238); // #22d3ee

/// Nova purple/violet - secondary accent
const COLOR_NOVA_PURPLE: Color = Color::Rgb(168, 85, 247); // #a855f7

/// Nova gold/amber - metrics accent
const COLOR_NOVA_GOLD: Color = Color::Rgb(251, 191, 36); // #fbbf24

/// Nova pink - coverage accent
const COLOR_NOVA_PINK: Color = Color::Rgb(236, 72, 153); // #ec4899

/// Border dim color
const COLOR_BORDER_DIM: Color = Color::Rgb(71, 85, 105); // slate-600

// =============================================================================
// TYPE STYLING
// =============================================================================

/// Get icon, label and accent color for node type
fn get_type_style(type_name: &str) -> (&'static str, &'static str, Color) {
    match type_name.to_lowercase().as_str() {
        "realm" => ("◉", "REALM", COLOR_NOVA_CYAN),
        "layer" => ("≡", "LAYER", COLOR_NOVA_PURPLE),
        "class" => ("◆", "CLASS", COLOR_NOVA_PINK),
        "instance" => ("●", "INSTANCE", COLOR_NOVA_GOLD),
        "arcfamily" => ("⟿", "ARC FAMILY", Color::Rgb(249, 115, 22)),
        "arcclass" => ("→", "ARC", Color::Rgb(249, 115, 22)),
        "section" => ("★", "SECTION", Color::Rgb(226, 232, 240)),
        "entitycategory" => ("◈", "CATEGORY", COLOR_NOVA_CYAN),
        _ => ("◇", "NODE", Color::Gray),
    }
}

// =============================================================================
// ASCII ART BUILDERS - RESPONSIVE
// =============================================================================

/// Build the title banner row: ╔═══════ ◆ CLASS ◆ ═══════╗
fn build_title_row(label: &str, icon: &str, width: usize, color: Color) -> Line<'static> {
    if width < 10 {
        return Line::from(Span::styled(
            format!("{} {}", icon, label),
            Style::default().fg(color),
        ));
    }

    let title = format!(" {} {} {} ", icon, label, icon);
    let title_len = title.chars().count();
    let remaining = width.saturating_sub(title_len).saturating_sub(2); // -2 for ╔╗
    let left_bars = remaining / 2;
    let right_bars = remaining - left_bars;

    Line::from(vec![
        Span::styled("╔", Style::default().fg(color)),
        Span::styled("═".repeat(left_bars), Style::default().fg(color)),
        Span::styled(
            title,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("═".repeat(right_bars), Style::default().fg(color)),
        Span::styled("╗", Style::default().fg(color)),
    ])
}

/// Build a separator row: ╠═══════════════════════════════╣
fn build_separator_row(width: usize, color: Color) -> Line<'static> {
    if width < 4 {
        return Line::from(Span::styled("═".repeat(width), Style::default().fg(color)));
    }
    let inner = width.saturating_sub(2);
    Line::from(vec![
        Span::styled("╠", Style::default().fg(color)),
        Span::styled("═".repeat(inner), Style::default().fg(color)),
        Span::styled("╣", Style::default().fg(color)),
    ])
}

/// Build the bottom row: ╚═══════════════════════════════╝
fn build_bottom_row(width: usize, color: Color) -> Line<'static> {
    if width < 4 {
        return Line::from(Span::styled("═".repeat(width), Style::default().fg(color)));
    }
    let inner = width.saturating_sub(2);
    Line::from(vec![
        Span::styled("╚", Style::default().fg(color)),
        Span::styled("═".repeat(inner), Style::default().fg(color)),
        Span::styled("╝", Style::default().fg(color)),
    ])
}

/// Build a content row with side borders: ║  content  ║
fn build_content_row(
    spans: Vec<Span<'static>>,
    width: usize,
    border_color: Color,
) -> Line<'static> {
    // Calculate content width
    let content_width: usize = spans.iter().map(|s| s.content.chars().count()).sum();
    let inner_width = width.saturating_sub(4); // -2 for ║ on each side, -2 for padding
    let padding = inner_width.saturating_sub(content_width);

    let mut all_spans = vec![Span::styled("║ ", Style::default().fg(border_color))];
    all_spans.extend(spans);
    all_spans.push(Span::styled(
        format!("{} ║", " ".repeat(padding)),
        Style::default().fg(border_color),
    ));

    Line::from(all_spans)
}

/// Build location badges row: │ ◎ org │ ◎ config │ ■ defined │
fn build_location_badges(
    realm: Option<(&str, &str, Color)>, // (icon, name, color)
    layer: Option<(&str, &str, Color)>,
    trait_info: Option<(&str, &str, Color)>,
    width: usize,
    border_color: Color,
) -> Vec<Line<'static>> {
    let mut badges: Vec<(String, Color)> = Vec::new();

    if let Some((icon, name, color)) = realm {
        badges.push((format!("{} {}", icon, name), color));
    }
    if let Some((icon, name, color)) = layer {
        badges.push((format!("{} {}", icon, name), color));
    }
    if let Some((icon, name, color)) = trait_info {
        badges.push((format!("{} {}", icon, name), color));
    }

    if badges.is_empty() {
        return vec![];
    }

    // Build the badge boxes
    let inner_width = width.saturating_sub(4);
    let badge_count = badges.len();
    let badge_width = if badge_count > 0 {
        inner_width / badge_count
    } else {
        0
    };

    // Top of badges: ┌───────────┬───────────┬───────────┐
    let mut top_parts = vec![Span::styled("║ ┌", Style::default().fg(border_color))];
    for i in 0..badge_count {
        let w = if i == badge_count - 1 {
            inner_width - (badge_width * (badge_count - 1)) - 2
        } else {
            badge_width.saturating_sub(1)
        };
        top_parts.push(Span::styled(
            "─".repeat(w),
            Style::default().fg(COLOR_BORDER_DIM),
        ));
        if i < badge_count - 1 {
            top_parts.push(Span::styled("┬", Style::default().fg(COLOR_BORDER_DIM)));
        }
    }
    top_parts.push(Span::styled("┐ ║", Style::default().fg(border_color)));

    // Middle with content: │ ◎ org   │ ◎ config │ ■ defined │
    let mut mid_parts = vec![Span::styled("║ │", Style::default().fg(border_color))];
    for (i, (text, color)) in badges.iter().enumerate() {
        let w = if i == badge_count - 1 {
            inner_width - (badge_width * (badge_count - 1)) - 2
        } else {
            badge_width.saturating_sub(1)
        };
        let text_len = text.chars().count();
        let pad = w.saturating_sub(text_len);
        mid_parts.push(Span::styled(
            format!(" {}", text),
            Style::default().fg(*color),
        ));
        mid_parts.push(Span::styled(
            " ".repeat(pad.saturating_sub(1)),
            Style::default(),
        ));
        mid_parts.push(Span::styled("│", Style::default().fg(COLOR_BORDER_DIM)));
    }
    mid_parts.push(Span::styled(" ║", Style::default().fg(border_color)));

    // Bottom of badges: └───────────┴───────────┴───────────┘
    let mut bot_parts = vec![Span::styled("║ └", Style::default().fg(border_color))];
    for i in 0..badge_count {
        let w = if i == badge_count - 1 {
            inner_width - (badge_width * (badge_count - 1)) - 2
        } else {
            badge_width.saturating_sub(1)
        };
        bot_parts.push(Span::styled(
            "─".repeat(w),
            Style::default().fg(COLOR_BORDER_DIM),
        ));
        if i < badge_count - 1 {
            bot_parts.push(Span::styled("┴", Style::default().fg(COLOR_BORDER_DIM)));
        }
    }
    bot_parts.push(Span::styled("┘ ║", Style::default().fg(border_color)));

    vec![
        Line::from(top_parts),
        Line::from(mid_parts),
        Line::from(bot_parts),
    ]
}

/// Build metrics cards row: ┌────────┐ ┌────────┐ ┌────────┐
fn build_metric_cards(
    metrics: Vec<(&str, String, Color)>, // (label, value, color)
    width: usize,
    border_color: Color,
) -> Vec<Line<'static>> {
    if metrics.is_empty() {
        return vec![];
    }

    let inner_width = width.saturating_sub(4);
    let card_count = metrics.len();
    let card_width = if card_count > 0 {
        (inner_width / card_count).min(14) // Max 14 chars per card
    } else {
        0
    };

    // Top: ┌────────┐  ┌────────┐  ┌────────┐
    let mut top_spans = vec![Span::styled("║ ", Style::default().fg(border_color))];
    for i in 0..card_count {
        top_spans.push(Span::styled("┌", Style::default().fg(COLOR_BORDER_DIM)));
        top_spans.push(Span::styled(
            "─".repeat(card_width.saturating_sub(2)),
            Style::default().fg(COLOR_BORDER_DIM),
        ));
        top_spans.push(Span::styled("┐", Style::default().fg(COLOR_BORDER_DIM)));
        if i < card_count - 1 {
            top_spans.push(Span::styled(" ", Style::default()));
        }
    }
    // Padding to fill width
    // v0.16.5: Use saturating_sub to prevent underflow if card_count somehow becomes 0
    let used: usize = 2 + card_count * card_width + card_count.saturating_sub(1);
    let remaining = width.saturating_sub(used).saturating_sub(2);
    top_spans.push(Span::styled(
        format!("{} ║", " ".repeat(remaining)),
        Style::default().fg(border_color),
    ));

    // Middle: │ 5 inst │  │ 9 prop │  │ 500tok │
    let mut mid_spans = vec![Span::styled("║ ", Style::default().fg(border_color))];
    for (i, (label, value, color)) in metrics.iter().enumerate() {
        let content = format!("{}{}", value, label);
        let content_len = content.chars().count();
        let inner = card_width.saturating_sub(2);
        let pad = inner.saturating_sub(content_len);
        mid_spans.push(Span::styled("│", Style::default().fg(COLOR_BORDER_DIM)));
        mid_spans.push(Span::styled(
            value.to_string(),
            Style::default().fg(*color).add_modifier(Modifier::BOLD),
        ));
        mid_spans.push(Span::styled(
            label.to_string(),
            Style::default().fg(Color::Gray),
        ));
        mid_spans.push(Span::styled(" ".repeat(pad), Style::default()));
        mid_spans.push(Span::styled("│", Style::default().fg(COLOR_BORDER_DIM)));
        if i < card_count - 1 {
            mid_spans.push(Span::styled(" ", Style::default()));
        }
    }
    mid_spans.push(Span::styled(
        format!("{} ║", " ".repeat(remaining)),
        Style::default().fg(border_color),
    ));

    // Bottom: └────────┘  └────────┘  └────────┘
    let mut bot_spans = vec![Span::styled("║ ", Style::default().fg(border_color))];
    for i in 0..card_count {
        bot_spans.push(Span::styled("└", Style::default().fg(COLOR_BORDER_DIM)));
        bot_spans.push(Span::styled(
            "─".repeat(card_width.saturating_sub(2)),
            Style::default().fg(COLOR_BORDER_DIM),
        ));
        bot_spans.push(Span::styled("┘", Style::default().fg(COLOR_BORDER_DIM)));
        if i < card_count - 1 {
            bot_spans.push(Span::styled(" ", Style::default()));
        }
    }
    bot_spans.push(Span::styled(
        format!("{} ║", " ".repeat(remaining)),
        Style::default().fg(border_color),
    ));

    vec![
        Line::from(top_spans),
        Line::from(mid_spans),
        Line::from(bot_spans),
    ]
}

/// Render the consolidated HEADER box with WOW horizontal layout.
/// v0.16.4: Mix of style A (big title) + C (card metrics)
fn render_header_box(f: &mut Frame, area: Rect, content: &UnifiedContent, state: BoxVisualState) {
    let (border_color, _title_style) = box_styles(state);

    // Extract node type from identity for styling
    let node_type = content
        .identity
        .lines
        .first()
        .and_then(|l| l.spans.get(1))
        .map(|s| s.content.trim().to_lowercase())
        .unwrap_or_default();

    // Get icon, label and accent color for this type
    let (type_icon, type_label, accent_color) = get_type_style(&node_type);

    // Border style based on state with accent color when selected
    let main_border_color = match state {
        BoxVisualState::Selected => accent_color,
        BoxVisualState::Focused => COLOR_NOVA_PURPLE,
        BoxVisualState::Unfocused => border_color,
    };

    // We render WITHOUT the Block wrapper - we draw our own ASCII box
    let available_width = area.width as usize;
    let mut lines: Vec<Line> = Vec::new();

    // ═══════════════════════════════════════════════════════════
    // ROW 1: Title banner ╔═══════ ◆ CLASS ◆ ═══════╗
    // ═══════════════════════════════════════════════════════════
    lines.push(build_title_row(
        type_label,
        type_icon,
        available_width,
        accent_color,
    ));

    // ═══════════════════════════════════════════════════════════
    // ROW 2: Node name/key
    // ═══════════════════════════════════════════════════════════
    // v0.16.5: Extract key and display from identity - handle different structures
    // Most types: [type, key, display?]
    // EntityCategory: [type, category, key, name]
    // Section types: [type, name]
    let (key_idx, display_idx) = if node_type == "entitycategory" {
        (2, Some(3)) // key at line 2, name at line 3
    } else if node_type == "section" {
        (1, None) // only name at line 1
    } else {
        (1, Some(2)) // key at line 1, display at line 2
    };

    let key = content
        .identity
        .lines
        .get(key_idx)
        .and_then(|l| l.spans.get(1))
        .map(|s| s.content.trim().to_string())
        .unwrap_or_default();
    let display = display_idx.and_then(|idx| {
        content
            .identity
            .lines
            .get(idx)
            .and_then(|l| l.spans.get(1))
            .map(|s| s.content.trim().to_string())
    });

    let name_spans = if let Some(d) = display {
        if d != key {
            vec![
                Span::styled(
                    key,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" · ", Style::default().fg(Color::DarkGray)),
                Span::styled(d, Style::default().fg(Color::Gray)),
            ]
        } else {
            vec![Span::styled(
                key,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )]
        }
    } else {
        vec![Span::styled(
            key,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]
    };
    lines.push(build_content_row(
        name_spans,
        available_width,
        main_border_color,
    ));

    // ═══════════════════════════════════════════════════════════
    // ROW 3-5: Location badges (if present)
    // ═══════════════════════════════════════════════════════════
    if !content.location.is_empty() {
        // Parse location from content
        let mut realm_info: Option<(&str, &str, Color)> = None;
        let mut layer_info: Option<(&str, &str, Color)> = None;
        let mut trait_info: Option<(&str, &str, Color)> = None;

        for line in &content.location.lines {
            if line.spans.len() >= 3 {
                let label = line.spans[0].content.trim().trim_end_matches(':');
                let icon = line.spans[1].content.trim();
                let value = line.spans[2].content.trim();
                let color = line.spans[1].style.fg.unwrap_or(Color::White);

                match label {
                    "realm" => realm_info = Some((icon, value, color)),
                    "layer" => layer_info = Some((icon, value, color)),
                    "trait" => trait_info = Some((icon, value, color)),
                    _ => {}
                }
            }
        }

        // Convert to owned strings for the badges
        let realm_owned: Option<(String, String, Color)> =
            realm_info.map(|(i, n, c)| (i.to_string(), n.to_string(), c));
        let layer_owned: Option<(String, String, Color)> =
            layer_info.map(|(i, n, c)| (i.to_string(), n.to_string(), c));
        let trait_owned: Option<(String, String, Color)> =
            trait_info.map(|(i, n, c)| (i.to_string(), n.to_string(), c));

        let badge_lines = build_location_badges(
            realm_owned
                .as_ref()
                .map(|(i, n, c)| (i.as_str(), n.as_str(), *c)),
            layer_owned
                .as_ref()
                .map(|(i, n, c)| (i.as_str(), n.as_str(), *c)),
            trait_owned
                .as_ref()
                .map(|(i, n, c)| (i.as_str(), n.as_str(), *c)),
            available_width,
            main_border_color,
        );
        lines.extend(badge_lines);
    }

    // ═══════════════════════════════════════════════════════════
    // ROW 6: Separator before metrics
    // ═══════════════════════════════════════════════════════════
    if !content.metrics.is_empty() || !content.coverage.is_empty() {
        lines.push(build_separator_row(available_width, main_border_color));
    }

    // ═══════════════════════════════════════════════════════════
    // ROW 7-9: Metrics cards
    // ═══════════════════════════════════════════════════════════
    if !content.metrics.is_empty() {
        let mut metrics: Vec<(&str, String, Color)> = Vec::new();

        for line in &content.metrics.lines {
            if line.spans.len() >= 2 {
                let label = line.spans[0].content.trim();
                let value = line.spans[1].content.trim();

                // Shorten labels for cards
                let short_label = match label {
                    "instances" => " inst",
                    "properties" => " prop",
                    "budget" => " tok",
                    "realms" => " rlm",
                    "classes" => " cls",
                    "layers" => " lyr",
                    "families" => " fam",
                    "arcs" => " arc",
                    "entities" => " ent",
                    "cardin." => "",
                    _ => "",
                };

                // Extract just the number
                let num: String = value.chars().take_while(|c| c.is_ascii_digit()).collect();
                if !num.is_empty() {
                    metrics.push((short_label, num, COLOR_NOVA_GOLD));
                }
            }
        }

        if !metrics.is_empty() {
            let card_lines = build_metric_cards(metrics, available_width, main_border_color);
            lines.extend(card_lines);
        }
    }

    // ═══════════════════════════════════════════════════════════
    // ROW 10: Coverage bar (if present)
    // ═══════════════════════════════════════════════════════════
    if !content.coverage.is_empty() {
        // Build a compact coverage line - must create owned spans for 'static lifetime
        let mut coverage_spans: Vec<Span<'static>> = Vec::new();
        for (i, line) in content.coverage.lines.iter().enumerate() {
            if i > 0 {
                coverage_spans.push(Span::styled("  ", Style::default()));
            }
            for span in &line.spans {
                // Convert to owned String to satisfy 'static lifetime
                let owned_content: String = span.content.to_string();
                coverage_spans.push(Span::styled(owned_content, span.style));
            }
        }
        if !coverage_spans.is_empty() {
            lines.push(build_content_row(
                coverage_spans,
                available_width,
                main_border_color,
            ));
        }
    }

    // ═══════════════════════════════════════════════════════════
    // BOTTOM ROW: ╚═══════════════════════════════════════════╝
    // ═══════════════════════════════════════════════════════════
    lines.push(build_bottom_row(available_width, accent_color));

    // Render
    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

/// Compute visual state for a box in the Detail panel.
/// Info panel contains: HEADER, PROPERTIES (v0.13: ARCS moved to dedicated panel).
fn detail_box_state(
    panel_focused: bool,
    selected_box: InfoBox,
    this_box: InfoBox,
) -> BoxVisualState {
    if !panel_focused {
        BoxVisualState::Unfocused
    } else if selected_box == this_box {
        BoxVisualState::Selected
    } else {
        BoxVisualState::Focused
    }
}

/// Render the unified info panel showing node metadata header.
///
/// v0.16.3: This now only renders the HEADER section (Identity, Location, Metrics, Coverage).
/// Properties are rendered separately by `render_props_panel`.
/// v0.16.4: Accepts pre-built content to avoid double-building.
pub fn render_unified_info_panel(f: &mut Frame, area: Rect, app: &App, content: &UnifiedContent) {
    // v0.16.3: YAML panel is [2], this is just the header info
    let panel_focused = app.focus == Focus::Yaml;
    let selected_box = app.selected_box;

    // Check minimum height - fall back to simple layout if too small
    if area.height < 6 {
        let all_lines: Vec<Line> = content
            .identity
            .lines
            .iter()
            .cloned()
            .chain(content.location.lines.iter().cloned())
            .chain(content.metrics.lines.iter().cloned())
            .chain(content.coverage.lines.iter().cloned())
            .collect();

        let paragraph = Paragraph::new(all_lines);
        f.render_widget(paragraph, area);
        return;
    }

    // v0.16.3: Render only the HEADER box (consolidated Identity, Location, Metrics, Coverage)
    let header_state = detail_box_state(panel_focused, selected_box, InfoBox::Header);
    render_header_box(f, area, content, header_state);
}

/// Render the properties panel [3] in the right column.
/// v0.16.3: New function for the separated Properties panel.
/// v0.16.4: Accepts pre-built content to avoid double-building.
pub fn render_props_panel(f: &mut Frame, area: Rect, app: &mut App, content: &UnifiedContent) {
    // v0.16.3: Props panel focused when Focus::Props
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
