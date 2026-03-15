//! Instance content builder.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::tui::app::{App, Focus};
use crate::tui::colors;
use crate::tui::palette;
use crate::tui::widgets::{Badge, ProgressBar};
use crate::tui::data::InstanceInfo;
use crate::tui::color_mode::ColorMode;

use super::{
    STYLE_PROP_KEY, STYLE_PROP_COLON,
    COLOR_HEADER_STANDARD, COLOR_HEADER_SPECIFIC,
    COLOR_PROPERTY_FOCUSED_BG, COLOR_VALUE_NULL,
    UnifiedContent,
    build_provenance_section, is_standard_property,
    type_badge, type_color, truncate_str, wrap_json_value,
    json_value_to_display, json_value_color,
};
use super::super::{
    STYLE_DIM, STYLE_HIGHLIGHT, STYLE_INFO, STYLE_MUTED, STYLE_PRIMARY,
};

/// Build content for Instance.
pub(super) fn build_instance_content(
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
        // Required bar - shows filled/total required
        let req_percent = if required_props > 0 {
            (filled_required as f64 / required_props as f64 * 100.0).round() as u8
        } else {
            0
        };
        let (req_bar, req_empty) = ProgressBar::new(filled_required, required_props, 12)
            .filled_style(Style::default().fg(Color::Yellow))
            .empty_style(STYLE_DIM)
            .to_spans();

        content.coverage.add_line(Line::from(vec![
            Span::styled("* ", Style::default().fg(Color::Red)),
            Span::styled("required   ", Style::default().fg(Color::Yellow)),
            req_bar,
            req_empty,
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
        let (opt_bar, opt_empty) = ProgressBar::new(filled_optional, optional_props, 12)
            .filled_style(Style::default().fg(Color::White))
            .empty_style(STYLE_DIM)
            .to_spans();

        content.coverage.add_line(Line::from(vec![
            Span::styled("  ", STYLE_DIM),
            Span::styled("optional   ", Style::default().fg(Color::White)),
            opt_bar,
            opt_empty,
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
    let provenance = instance.properties.get("provenance");
    content.provenance = build_provenance_section(provenance);

    // PROPERTIES - v0.13.1: Display ALL properties with STANDARD/SPECIFIC sections
    let all_schema_keys: Vec<&String> = class
        .properties
        .iter()
        .filter(|k| !k.starts_with('_'))
        .collect();

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

    // Collect extra instance props not in schema
    let extra_keys: Vec<&String> = instance
        .properties
        .keys()
        .filter(|k| !k.starts_with('_') && !all_schema_keys.contains(k))
        .collect();

    if !all_schema_keys.is_empty() || !extra_keys.is_empty() {
        let max_key_len = all_schema_keys
            .iter()
            .chain(extra_keys.iter())
            .map(|k| k.len())
            .max()
            .unwrap_or(0)
            .min(18);

        let is_props_focused = app.focus == Focus::Props;
        let mut property_idx: usize = 0;

        // STANDARD section (teal header)
        if !standard_keys.is_empty() {
            content.properties.add_line(Line::from(vec![Span::styled(
                format!("── STANDARD ({}) ──", standard_keys.len()),
                Style::default().fg(COLOR_HEADER_STANDARD),
            )]));
            for key in &standard_keys {
                render_instance_property(
                    &mut content,
                    app,
                    instance,
                    class,
                    key,
                    max_key_len,
                    is_props_focused,
                    property_idx,
                );
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
                render_instance_property(
                    &mut content,
                    app,
                    instance,
                    class,
                    key,
                    max_key_len,
                    is_props_focused,
                    property_idx,
                );
                property_idx += 1;
            }
        }

        // Extra instance props not in schema (marked with ?)
        for key in &extra_keys {
            if let Some(value) = instance.properties.get(*key) {
                let value_str = json_value_to_display(value);
                let value_color = json_value_color(value);

                let is_focused = is_props_focused && property_idx == app.focused_property_idx;
                let bg_style = if is_focused {
                    Style::default().bg(COLOR_PROPERTY_FOCUSED_BG)
                } else {
                    Style::default()
                };

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

/// Render a single instance property line with status, badge, value, and focus highlight.
#[allow(clippy::too_many_arguments)]
fn render_instance_property(
    content: &mut UnifiedContent<'static>,
    app: &App,
    instance: &InstanceInfo,
    class: &crate::tui::data::ClassInfo,
    key: &str,
    max_key_len: usize,
    is_props_focused: bool,
    property_idx: usize,
) {
    let is_required = class.required_properties.iter().any(|k| k == key);
    let has_value = instance
        .properties
        .get(key)
        .map(|v| !v.is_null())
        .unwrap_or(false);

    let (status_icon, status_style) = if has_value {
        ("✓", Style::default().fg(palette::SOLARIZED_GREEN))
    } else if is_required {
        ("⚠", Style::default().fg(palette::SOLARIZED_ORANGE))
    } else {
        (" ", STYLE_DIM)
    };
    let required_marker = if is_required { "*" } else { " " };

    let (value_str, value_color) = if let Some(value) = instance.properties.get(key) {
        (json_value_to_display(value), json_value_color(value))
    } else {
        ("~".to_string(), COLOR_VALUE_NULL)
    };

    let prop_type = app
        .schema_overlay
        .validated_class_properties
        .as_ref()
        .and_then(|props| props.iter().find(|p| p.name.as_str() == key))
        .map(|p| p.prop_type.as_str())
        .unwrap_or("???");
    let badge = type_badge(prop_type);
    let badge_color = type_color(prop_type);

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
                Style::default()
                    .fg(palette::SOLARIZED_RED)
                    .patch(bg_style),
            ),
            Span::styled(
                format!("{:width$}", key, width = max_key_len),
                STYLE_PROP_KEY.patch(bg_style),
            ),
            Span::styled(": ", STYLE_PROP_COLON.patch(bg_style)),
            Badge::new(badge.trim())
                .style(Style::default().fg(badge_color).patch(bg_style))
                .spaced()
                .to_span(),
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
                Style::default()
                    .fg(palette::SOLARIZED_RED)
                    .patch(bg_style),
            ),
            Span::styled(
                format!("{:width$}", key, width = max_key_len),
                STYLE_PROP_KEY.patch(bg_style),
            ),
            Span::styled(": ", STYLE_PROP_COLON.patch(bg_style)),
            Badge::new(badge.trim())
                .style(Style::default().fg(badge_color).patch(bg_style))
                .spaced()
                .to_span(),
            Span::styled(
                display_value,
                Style::default().fg(value_color).patch(bg_style),
            ),
        ]));
    }
}
