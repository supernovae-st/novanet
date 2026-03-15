//! Group content builders: EntityCategory, LocaleGroup, EntityGroup, empty state.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::UnifiedContent;
use super::super::{
    STYLE_ACCENT, STYLE_DIM, STYLE_MUTED, STYLE_PRIMARY,
};
use super::format_json_value;

/// Build content for EntityCategory.
pub(super) fn build_category_content(
    cat: &crate::tui::data::EntityCategory,
) -> UnifiedContent<'static> {
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
pub(super) fn build_locale_group_content(
    group: &crate::tui::data::LocaleGroup,
) -> UnifiedContent<'static> {
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
pub(super) fn build_entity_group_content(
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
pub(super) fn build_empty_content() -> UnifiedContent<'static> {
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
