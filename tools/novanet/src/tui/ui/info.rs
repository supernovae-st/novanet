//! Info panel rendering for TUI.
//!
//! This module contains all functions related to rendering the Info panel,
//! which displays details about the currently selected tree item.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use crate::tui::app::{App, Focus};
use crate::tui::data::{ArcDirection, InstanceInfo, TreeItem};
use crate::tui::schema::ValidationStatus;
use crate::tui::theme::hex_to_color;
use crate::tui::unicode::truncate_to_width;

use serde_json::Value as JsonValue;

use super::{
    COLOR_UNFOCUSED_BORDER, STYLE_ACCENT, STYLE_DESC, STYLE_DIM, STYLE_HIGHLIGHT, STYLE_INFO,
    STYLE_MUTED, STYLE_PRIMARY, STYLE_SUCCESS, STYLE_WARNING, trait_icon, wrap_text,
};

// =============================================================================
// FIXED SECTION HEIGHTS
// =============================================================================

/// Fixed height for top row (IDENTITY | LOCATION side-by-side)
const SECTION_HEIGHT_TOP: u16 = 6;
/// Fixed height for METRICS section
const SECTION_HEIGHT_METRICS: u16 = 4;
/// Fixed height for COVERAGE section
const SECTION_HEIGHT_COVERAGE: u16 = 4;
/// Fixed height for RELATIONSHIPS section
const SECTION_HEIGHT_RELATIONSHIPS: u16 = 8;

// =============================================================================
// UNIFIED SECTION TYPES
// =============================================================================

/// Content for a single info section.
/// Each section has a title and content lines.
/// Empty sections display "—" as content.
#[derive(Default)]
struct SectionContent<'a> {
    lines: Vec<Line<'a>>,
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

    fn add_empty(&mut self) {
        self.lines.push(Line::from(Span::styled("—", STYLE_DIM)));
    }
}

/// Unified info content with 6 fixed sections.
/// All sections are always present; empty sections show "—".
#[derive(Default)]
struct UnifiedContent<'a> {
    /// IDENTITY: type, category, key, class
    identity: SectionContent<'a>,
    /// LOCATION: realm, layer, trait
    location: SectionContent<'a>,
    /// METRICS: counts, totals, budgets
    metrics: SectionContent<'a>,
    /// COVERAGE: property fill rates, health bars
    coverage: SectionContent<'a>,
    /// PROPERTIES: property list with values/schema
    properties: SectionContent<'a>,
    /// RELATIONSHIPS: arcs, pipeline context
    relationships: SectionContent<'a>,
}

// =============================================================================
// UNIFIED CONTENT BUILDERS
// =============================================================================

/// Build unified content for the current tree item.
/// Returns all 6 sections populated with appropriate content.
fn build_unified_content(app: &App) -> UnifiedContent<'static> {
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
    content.identity.add_kv(
        "category",
        Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
    );
    content
        .identity
        .add_kv("key", Span::styled(realm.key.clone(), STYLE_PRIMARY));

    // LOCATION - realm is top-level
    content.location.add_line(Line::from(vec![
        Span::styled(
            format!("{} ", realm.icon),
            Style::default().fg(hex_to_color(&realm.color)),
        ),
        Span::styled(
            realm.display_name.clone(),
            Style::default().fg(hex_to_color(&realm.color)),
        ),
    ]));

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

    // PROPERTIES - LLM context
    if !realm.llm_context.is_empty() {
        for wrapped_line in wrap_text(&realm.llm_context, 38) {
            content
                .properties
                .add_line(Line::from(Span::styled(wrapped_line, STYLE_DESC)));
        }
    } else {
        content.properties.add_empty();
    }

    // RELATIONSHIPS - not applicable for realm
    content.relationships.add_empty();

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
    content.identity.add_kv(
        "category",
        Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
    );
    content
        .identity
        .add_kv("key", Span::styled(layer.key.clone(), STYLE_PRIMARY));

    // LOCATION
    content.location.add_line(Line::from(vec![
        Span::styled(format!("{} ", realm.icon), STYLE_DIM),
        Span::styled(
            realm.display_name.clone(),
            Style::default().fg(hex_to_color(&realm.color)),
        ),
    ]));
    content.location.add_line(Line::from(vec![
        Span::styled(
            format!("{} ", theme.icons.layer(&layer.key)),
            Style::default().fg(hex_to_color(&layer.color)),
        ),
        Span::styled(
            layer.display_name.clone(),
            Style::default().fg(hex_to_color(&layer.color)),
        ),
    ]));

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
            *trait_counts.entry(class_info.trait_name.clone()).or_insert(0) += 1;
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

    // PROPERTIES - LLM context
    if !layer.llm_context.is_empty() {
        for wrapped_line in wrap_text(&layer.llm_context, 38) {
            content
                .properties
                .add_line(Line::from(Span::styled(wrapped_line, STYLE_DESC)));
        }
    } else {
        content.properties.add_empty();
    }

    // RELATIONSHIPS - not applicable for layer
    content.relationships.add_empty();

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

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("Node Class", STYLE_INFO));
    content.identity.add_kv(
        "category",
        Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
    );
    content
        .identity
        .add_kv("key", Span::styled(class.key.clone(), STYLE_PRIMARY));

    // LOCATION
    content.location.add_line(Line::from(vec![
        Span::styled(format!("{} ", realm.icon), STYLE_DIM),
        Span::styled(
            realm.display_name.clone(),
            Style::default().fg(hex_to_color(&realm.color)),
        ),
    ]));
    content.location.add_line(Line::from(vec![
        Span::styled(format!("{} ", theme.icons.layer(&layer.key)), STYLE_DIM),
        Span::styled(
            layer.display_name.clone(),
            Style::default().fg(hex_to_color(&layer.color)),
        ),
    ]));
    if !class.trait_name.is_empty() {
        let trait_icon = theme.icons.trait_icon(&class.trait_name);
        content.location.add_line(Line::from(vec![
            Span::styled(format!("{} ", trait_icon), STYLE_DIM),
            Span::styled(
                class.trait_name.clone(),
                Style::default().fg(theme.trait_color(&class.trait_name)),
            ),
        ]));
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

    // PROPERTIES - validated or simple list
    if let Some(validated) = &app.validated_class_properties {
        for prop in validated {
            let (status_icon, status_style) = match prop.status {
                ValidationStatus::Sync => ("✓", STYLE_SUCCESS),
                ValidationStatus::Missing => ("⚠", STYLE_WARNING),
                ValidationStatus::Extra => ("?", STYLE_DIM),
            };
            let required_marker = if prop.required { "*" } else { " " };
            let badge = type_badge(&prop.prop_type);

            content.properties.add_line(Line::from(vec![
                Span::styled(status_icon, status_style),
                Span::styled(
                    required_marker,
                    Style::default().fg(Color::Rgb(255, 100, 100)),
                ),
                Span::styled(format!("[{:4}] ", badge), STYLE_DIM),
                Span::styled(format!("{:<15}", prop.name), STYLE_INFO),
            ]));
        }
    } else if !class.properties.is_empty() {
        for prop in &class.properties {
            let is_required = class.required_properties.contains(prop);
            let marker = if is_required { "*" } else { " " };
            let prop_color = if is_required {
                Color::Yellow
            } else {
                Color::White
            };

            content.properties.add_line(Line::from(vec![
                Span::styled(
                    format!("  {}", marker),
                    Style::default().fg(Color::Rgb(255, 100, 100)),
                ),
                Span::styled(prop.clone(), Style::default().fg(prop_color)),
            ]));
        }
    } else {
        content.properties.add_empty();
    }

    // RELATIONSHIPS - related arcs
    if !class.arcs.is_empty() {
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

        // Show first few arcs
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

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("ArcFamily", STYLE_ARC_FAMILY));
    content.identity.add_kv(
        "category",
        Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
    );
    content
        .identity
        .add_kv("key", Span::styled(family.key.clone(), STYLE_PRIMARY));

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

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("ArcClass", STYLE_HIGHLIGHT));
    content.identity.add_kv(
        "category",
        Span::styled("◈ Schema", Style::default().fg(Color::Cyan)),
    );
    content
        .identity
        .add_kv("key", Span::styled(arc_class.key.clone(), STYLE_PRIMARY));
    content.identity.add_kv(
        "family",
        Span::styled(family.display_name.clone(), STYLE_ARC_FAMILY),
    );

    // LOCATION - not applicable for arcs
    content.location.add_empty();

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

    // RELATIONSHIPS - from/to
    content.relationships.add_line(Line::from(vec![
        Span::styled("from ", STYLE_DIM),
        Span::styled(arc_class.from_class.clone(), STYLE_INFO),
    ]));
    content.relationships.add_line(Line::from(vec![
        Span::styled("to   ", STYLE_DIM),
        Span::styled(arc_class.to_class.clone(), STYLE_INFO),
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

    // IDENTITY
    content
        .identity
        .add_kv("type", Span::styled("Instance", STYLE_SUCCESS));
    content.identity.add_kv(
        "category",
        Span::styled("◆ Data", Style::default().fg(Color::Yellow)),
    );
    content
        .identity
        .add_kv("key", Span::styled(instance.key.clone(), STYLE_PRIMARY));
    content.identity.add_kv(
        "class",
        Span::styled(class.display_name.clone(), STYLE_INFO),
    );

    // LOCATION
    content.location.add_line(Line::from(vec![
        Span::styled(format!("{} ", realm.icon), STYLE_DIM),
        Span::styled(
            realm.display_name.clone(),
            Style::default().fg(hex_to_color(&realm.color)),
        ),
    ]));
    content.location.add_line(Line::from(vec![
        Span::styled(format!("{} ", theme.icons.layer(&layer.key)), STYLE_DIM),
        Span::styled(
            layer.display_name.clone(),
            Style::default().fg(hex_to_color(&layer.color)),
        ),
    ]));
    if !class.trait_name.is_empty() {
        let trait_icon = theme.icons.trait_icon(&class.trait_name);
        content.location.add_line(Line::from(vec![
            Span::styled(format!("{} ", trait_icon), STYLE_DIM),
            Span::styled(
                class.trait_name.clone(),
                Style::default().fg(theme.trait_color(&class.trait_name)),
            ),
        ]));
    }

    // METRICS
    if class.instance_count > 0 {
        content.metrics.add_kv(
            "siblings",
            Span::styled(format!("{} total", class.instance_count), STYLE_MUTED),
        );
    }
    content.metrics.add_kv(
        "props",
        Span::styled(instance.properties.len().to_string(), STYLE_INFO),
    );

    // COVERAGE - property fill rate
    let total_schema_props = class.properties.len();
    let filled_props = instance.properties.len();
    if total_schema_props > 0 && filled_props > 0 {
        let fill_percent = ((filled_props as f64 / total_schema_props as f64) * 100.0)
            .round()
            .min(100.0) as usize;
        let bar_width = 12usize;
        let filled = (fill_percent * bar_width) / 100;
        let bar = "━".repeat(filled.max(1));
        let empty = "░".repeat(bar_width.saturating_sub(filled));

        content.coverage.add_line(Line::from(vec![
            Span::styled(
                format!("{}/{} ", filled_props, total_schema_props),
                STYLE_INFO,
            ),
            Span::styled(bar, STYLE_SUCCESS),
            Span::styled(empty, STYLE_DIM),
            Span::styled(format!(" {}%", fill_percent), STYLE_MUTED),
        ]));
    } else {
        content.coverage.add_empty();
    }

    // PROPERTIES - simple property list
    if !instance.properties.is_empty() {
        for (key, value) in &instance.properties {
            if key.starts_with('_') || key == "key" || key == "display_name" {
                continue;
            }
            let value_str = json_value_to_display(value);
            let truncated = truncate_str(&value_str, 30);
            content.properties.add_line(Line::from(vec![
                Span::styled(format!("{:<14}", key), STYLE_INFO),
                Span::styled(truncated, STYLE_PRIMARY),
            ]));
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

/// Arc family label style.
const STYLE_ARC_FAMILY: Style = Style::new().fg(Color::Rgb(180, 140, 80));

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

/// Safely truncate a UTF-8 string to N terminal columns (not chars).
/// Appends "..." if truncated. Handles CJK, emoji, and combining characters.
fn truncate_str(s: &str, max_width: usize) -> String {
    truncate_to_width(s, max_width)
}

/// Convert JSON value to display string.
fn json_value_to_display(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => format!("\"{}\"", s),
        JsonValue::Array(arr) => serde_json::to_string(arr).unwrap_or_else(|_| "[]".to_string()),
        JsonValue::Object(obj) => serde_json::to_string(obj).unwrap_or_else(|_| "{}".to_string()),
    }
}

/// Get detail panel title for current item.
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

/// Render a section box with title and content (non-scrollable).
fn render_section_box(
    f: &mut Frame,
    area: Rect,
    title: &str,
    content: &SectionContent,
    border_color: Color,
) {
    let lines: Vec<Line> = if content.is_empty() {
        vec![Line::from(Span::styled("—", STYLE_DIM))]
    } else {
        content.lines.clone()
    };

    let block = Block::default()
        .title(Span::styled(format!(" {} ", title), STYLE_MUTED))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Render a scrollable section box with scroll indicator.
fn render_scrollable_section_box(
    f: &mut Frame,
    area: Rect,
    title: &str,
    content: &SectionContent,
    border_color: Color,
    scroll_offset: usize,
    focused: bool,
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

    // Show scroll indicator in title if scrollable
    let title_text = if total_lines > visible_height {
        format!(" {} [{}/{}] ", title, clamped_scroll + 1, total_lines)
    } else {
        format!(" {} ", title)
    };

    let title_style = if focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        STYLE_MUTED
    };

    let block = Block::default()
        .title(Span::styled(title_text, title_style))
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

/// Unified info panel: 6 fixed sections with consistent layout.
/// - Row 1: IDENTITY | LOCATION (side-by-side, fixed height)
/// - Row 2: METRICS (fixed height)
/// - Row 3: COVERAGE (fixed height)
/// - Row 4: PROPERTIES (fills remaining, scrollable)
/// - Row 5: RELATIONSHIPS (fixed height)
pub fn render_unified_info_panel(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.focus == Focus::Info;
    let border_color = if focused {
        Color::Cyan
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Build unified content
    let content = build_unified_content(app);

    // Get title from current item
    let title = get_detail_title(app);

    // Outer block for the entire panel
    let outer_block = Block::default()
        .title(Span::styled(format!(" {} ", title), STYLE_PRIMARY))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = outer_block.inner(area);
    f.render_widget(outer_block, area);

    // Check minimum height
    if inner.height < 15 {
        // Too small for boxes, fall back to simple layout
        let all_lines: Vec<Line> = content
            .identity
            .lines
            .into_iter()
            .chain(content.location.lines)
            .chain(content.metrics.lines)
            .chain(content.coverage.lines)
            .chain(content.properties.lines)
            .chain(content.relationships.lines)
            .collect();

        let paragraph = Paragraph::new(all_lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // Fixed section heights - PROPERTIES fills remaining space
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(SECTION_HEIGHT_TOP), // IDENTITY | LOCATION
            Constraint::Length(SECTION_HEIGHT_METRICS), // METRICS
            Constraint::Length(SECTION_HEIGHT_COVERAGE), // COVERAGE
            Constraint::Min(6),                     // PROPERTIES (fills remaining, scrollable)
            Constraint::Length(SECTION_HEIGHT_RELATIONSHIPS), // RELATIONSHIPS
        ])
        .split(inner);

    // Row 1: IDENTITY | LOCATION side-by-side
    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[0]);

    render_section_box(f, top_row[0], "IDENTITY", &content.identity, border_color);
    render_section_box(f, top_row[1], "LOCATION", &content.location, border_color);

    // Row 2: METRICS
    render_section_box(f, main_chunks[1], "METRICS", &content.metrics, border_color);

    // Row 3: COVERAGE
    render_section_box(
        f,
        main_chunks[2],
        "COVERAGE",
        &content.coverage,
        border_color,
    );

    // Row 4: PROPERTIES (scrollable with app.info_scroll)
    let total_lines = render_scrollable_section_box(
        f,
        main_chunks[3],
        "PROPERTIES",
        &content.properties,
        border_color,
        app.info_scroll,
        focused,
    );
    // Update line count for scroll bounds
    app.info_line_count = total_lines;

    // Row 5: RELATIONSHIPS
    render_section_box(
        f,
        main_chunks[4],
        "RELATIONSHIPS",
        &content.relationships,
        border_color,
    );
}
