//! Schema-level content builders: ClassesSection, ArcsSection, Realm, Layer.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::tui::app::App;
use crate::tui::widgets::ProgressBar;
use crate::tui::theme::hex_to_color;

use super::{
    COLOR_HEADER_STANDARD,
    PropType, UnifiedContent,
    render_property_line,
};
use super::super::{
    STYLE_ACCENT, STYLE_DIM, STYLE_HIGHLIGHT, STYLE_MUTED, STYLE_PRIMARY, STYLE_SUCCESS,
};

/// Build content for ClassesSection.
pub(super) fn build_classes_section_content(app: &App) -> UnifiedContent<'static> {
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
        for realm in &app.tree.realms {
            let realm_classes: usize = realm.layers.iter().map(|l| l.classes.len()).sum();
            let percent = (realm_classes as f64 / class_count as f64 * 100.0).round() as u8;
            let realm_color = app.theme.realm_color(&realm.key);
            let (bar, empty) = ProgressBar::new(realm_classes, class_count, 16)
                .filled_style(Style::default().fg(realm_color))
                .empty_style(STYLE_DIM)
                .to_spans();

            content.coverage.add_line(Line::from(vec![
                Span::styled(
                    format!("{:8} ", realm.display_name),
                    Style::default().fg(realm_color),
                ),
                bar,
                empty,
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
pub(super) fn build_arcs_section_content(app: &App) -> UnifiedContent<'static> {
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
pub(super) fn build_realm_content(
    app: &App,
    realm: &crate::tui::data::RealmInfo,
) -> UnifiedContent<'static> {
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
        for layer in &realm.layers {
            let count = layer.classes.len();
            if count == 0 {
                continue;
            }
            let percent = (count as f64 / class_count as f64 * 100.0).round() as u8;
            let layer_color = theme.layer_color(&layer.key);
            let (bar, empty) = ProgressBar::new(count, class_count, 12)
                .filled_style(Style::default().fg(layer_color))
                .empty_style(STYLE_DIM)
                .to_spans();

            content.coverage.add_line(Line::from(vec![
                Span::styled(
                    format!("{:12} ", layer.display_name),
                    Style::default().fg(layer_color),
                ),
                bar,
                empty,
                Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
            ]));
        }
    } else {
        content.coverage.add_empty();
    }

    // PROPERTIES - v0.19.0: Show 8 standard properties (ADR-044)
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
pub(super) fn build_layer_content(
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

    // RELATIONSHIPS - v0.17: show incoming HAS_LAYER + outgoing HAS_CLASS
    let class_count = layer.classes.len();

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
