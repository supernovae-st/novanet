//! Arc-level content builders: ArcFamily, ArcClass.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::tui::colors;
use crate::tui::theme::ColorMode;

use super::{
    COLOR_HEADER_STANDARD, COLOR_HEADER_SPECIFIC,
    PropType, UnifiedContent,
    render_property_line,
};
use super::super::{
    STYLE_ACCENT, STYLE_DIM, STYLE_PRIMARY,
    arc_family_badge_icon,
};

/// Build content for ArcFamily.
pub(super) fn build_arc_family_content(
    family: &crate::tui::data::ArcFamilyInfo,
) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();
    let mode = ColorMode::TrueColor;

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
pub(super) fn build_arc_class_content(
    family: &crate::tui::data::ArcFamilyInfo,
    arc_class: &crate::tui::data::ArcClassInfo,
) -> UnifiedContent<'static> {
    let mut content = UnifiedContent::default();
    let mode = ColorMode::TrueColor;

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
