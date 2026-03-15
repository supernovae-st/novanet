//! Class (NodeClass) content builder.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::tui::app::App;
use crate::tui::colors;
use crate::tui::palette;
use crate::tui::widgets::{Badge, ProgressBar};
use crate::tui::data::ArcDirection;
use crate::tui::schema::ValidationStatus;
use crate::tui::color_mode::ColorMode;

use super::{
    STYLE_PROP_KEY, STYLE_PROP_COLON,
    COLOR_HEADER_STANDARD, COLOR_HEADER_SPECIFIC,
    UnifiedContent,
    is_standard_property,
    type_badge, type_color,
};
use super::super::{
    STYLE_DIM, STYLE_HIGHLIGHT, STYLE_INFO, STYLE_MUTED, STYLE_PRIMARY,
};

/// Build content for Class (NodeClass).
pub(super) fn build_class_content(
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
        // Required bar
        let req_percent = (required_props as f64 / total_props as f64 * 100.0).round() as u8;
        let (req_bar, req_empty) = ProgressBar::new(required_props, total_props, 12)
            .filled_style(Style::default().fg(Color::Yellow))
            .empty_style(STYLE_DIM)
            .to_spans();

        content.coverage.add_line(Line::from(vec![
            Span::styled("* ", Style::default().fg(Color::Red)),
            Span::styled("required   ", Style::default().fg(Color::Yellow)),
            req_bar,
            req_empty,
            Span::styled(format!(" {:>3}%", req_percent), STYLE_MUTED),
            Span::styled(format!("  {}", required_props), STYLE_DIM),
        ]));

        // Optional bar
        let opt_percent = (optional_props as f64 / total_props as f64 * 100.0).round() as u8;
        let (opt_bar, opt_empty) = ProgressBar::new(optional_props, total_props, 12)
            .filled_style(Style::default().fg(Color::White))
            .empty_style(STYLE_DIM)
            .to_spans();

        content.coverage.add_line(Line::from(vec![
            Span::styled("  ", STYLE_DIM),
            Span::styled("optional   ", Style::default().fg(Color::White)),
            opt_bar,
            opt_empty,
            Span::styled(format!(" {:>3}%", opt_percent), STYLE_MUTED),
            Span::styled(format!("  {}", optional_props), STYLE_DIM),
        ]));
    } else {
        content.coverage.add_empty();
    }

    // PROPERTIES - v0.13.1: YAML-style with cyan keys, type badges, section headers
    if let Some(validated) = &app.schema_overlay.validated_class_properties {
        let standard_props: Vec<_> = validated
            .iter()
            .filter(|p| is_standard_property(&p.name))
            .collect();
        let specific_props: Vec<_> = validated
            .iter()
            .filter(|p| !is_standard_property(&p.name))
            .collect();

        let max_name_len = validated
            .iter()
            .map(|p| p.name.len())
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
                render_validated_property(&mut content, prop, max_name_len);
            }
        }

        // SPECIFIC section header (orange)
        if !specific_props.is_empty() {
            content.properties.add_line(Line::from(vec![Span::styled(
                format!("── SPECIFIC ({}) ──", specific_props.len()),
                Style::default().fg(COLOR_HEADER_SPECIFIC),
            )]));

            for prop in &specific_props {
                render_validated_property(&mut content, prop, max_name_len);
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
                render_fallback_property(&mut content, prop, max_name_len, class);
            }
        }

        // SPECIFIC section header (orange)
        if !specific_props.is_empty() {
            content.properties.add_line(Line::from(vec![Span::styled(
                format!("── SPECIFIC ({}) ──", specific_props.len()),
                Style::default().fg(COLOR_HEADER_SPECIFIC),
            )]));

            for prop in &specific_props {
                render_fallback_property(&mut content, prop, max_name_len, class);
            }
        }
    } else {
        content.properties.add_empty();
    }

    // RELATIONSHIPS - v0.13: with arc family colors from Neo4j data
    if let Some(arcs_data) = &app.details.class_arcs {
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

/// Render a validated property line (with status icon, badge, etc.).
fn render_validated_property(
    content: &mut UnifiedContent<'static>,
    prop: &crate::tui::schema::ValidatedProperty,
    max_name_len: usize,
) {
    let (status_icon, status_style) = match prop.status {
        ValidationStatus::Sync => ("✓", Style::default().fg(palette::SOLARIZED_GREEN)),
        ValidationStatus::Missing => ("⚠", Style::default().fg(palette::SOLARIZED_ORANGE)),
        ValidationStatus::Extra => ("?", STYLE_DIM),
    };
    let required_marker = if prop.required { "*" } else { " " };
    let badge = type_badge(&prop.prop_type);
    let badge_color = type_color(&prop.prop_type);

    content.properties.add_line(Line::from(vec![
        Span::styled(status_icon, status_style),
        Span::styled(
            required_marker,
            Style::default().fg(palette::SOLARIZED_RED),
        ),
        Span::styled(
            format!("{:width$}", prop.name, width = max_name_len),
            STYLE_PROP_KEY,
        ),
        Span::styled(": ", STYLE_PROP_COLON),
        Badge::new(badge.trim())
            .style(Style::default().fg(badge_color))
            .to_span(),
    ]));
}

/// Render a fallback property line (no validation data, just schema info).
fn render_fallback_property(
    content: &mut UnifiedContent<'static>,
    prop: &str,
    max_name_len: usize,
    class: &crate::tui::data::ClassInfo,
) {
    let is_required = class.required_properties.iter().any(|k| k == prop);
    let marker = if is_required { "*" } else { " " };

    content.properties.add_line(Line::from(vec![
        Span::styled("  ", STYLE_DIM),
        Span::styled(marker, Style::default().fg(palette::SOLARIZED_RED)),
        Span::styled(
            format!("{:width$}", prop, width = max_name_len),
            STYLE_PROP_KEY,
        ),
        Span::styled(": ", STYLE_PROP_COLON),
        Span::styled(
            if is_required { "[req]" } else { "[opt]" },
            if is_required {
                Style::default().fg(palette::SOLARIZED_GOLD)
            } else {
                STYLE_DIM
            },
        ),
    ]));
}
