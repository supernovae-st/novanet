//! Content builders for the info panel.
//!
//! Each `build_*_content` function creates a `UnifiedContent` for a specific
//! tree item type (Class, Instance, Realm, etc.).

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::tui::app::{App, Focus};
use crate::tui::colors;
use crate::tui::data::{ArcDirection, InstanceInfo, TreeItem};
use crate::tui::schema::ValidationStatus;
use crate::tui::theme::{ColorMode, hex_to_color};

// Parent module re-exports
use super::{
    STYLE_PROP_KEY, STYLE_PROP_COLON,
    COLOR_HEADER_STANDARD, COLOR_HEADER_SPECIFIC,
    COLOR_PROPERTY_FOCUSED_BG, COLOR_VALUE_NULL,
    PropType, UnifiedContent,
    build_provenance_section, is_standard_property, render_property_line,
    type_badge, type_color, truncate_str, format_json_value, wrap_json_value,
    json_value_to_display, json_value_color,
};

// UI style re-exports from grandparent (ui/mod.rs)
use super::super::{
    STYLE_ACCENT, STYLE_DIM, STYLE_HIGHLIGHT, STYLE_INFO, STYLE_MUTED, STYLE_PRIMARY,
    STYLE_SUCCESS, arc_family_badge_icon,
};

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
                outgoing_arcs: vec![], // Arcs loaded on-demand via detail panel, not during construction
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
        let is_props_focused = app.focus == Focus::Props;
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

