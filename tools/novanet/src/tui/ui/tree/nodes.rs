//! Node Classes section of the tree panel.
//!
//! Builds tree lines for the Realm > Layer > Class > Instance hierarchy.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::helpers::{branch_char, cont_char, expand_icon, format_health_badge, make_line};
use super::highlight::highlight_matches_with_bg;
use super::{render_power_bar, COLOR_ENTITY_SLUG, COLOR_ENTITY_TEXT};
use super::super::{
    COLOR_ACTIVE_CLASS_BG, COLOR_HIGHLIGHT_BG, COLOR_INSTANCE, COLOR_MUTED_TEXT,
};
use crate::tui::app::App;
use crate::tui::data::locale_to_flag;
use crate::tui::palette;
use crate::tui::theme::hex_to_color;
use crate::tui::unicode::display_width;

/// Build all tree lines for the "Node Classes" section.
///
/// Returns the next `idx` value after all node class lines have been added.
pub(super) fn build_node_class_lines<'a>(
    all_lines: &mut Vec<Line<'a>>,
    idx: &mut usize,
    app: &App,
    area_width: u16,
    focused: bool,
) {
    let branch = branch_char;
    let cont = cont_char;

    // === KINDS HEADER ===
    let classes_collapsed = app.tree.is_collapsed("classes");
    let classes_icon = expand_icon(classes_collapsed);
    let classes_count: usize = app
        .tree
        .realms
        .iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.classes.len())
        .sum();
    all_lines.push(make_line(
        *idx,
        app.tree_cursor,
        focused,
        "",
        classes_icon,
        format!("Node Classes ({})", classes_count),
        Color::Magenta,
        Color::Magenta,
        app.search.matches.get(idx).map(|v| v.as_slice()),
        None,
    ));
    *idx += 1;

    let has_arcs = !app.tree.arc_families.is_empty();

    if classes_collapsed {
        return;
    }

    // === REALMS ===
    let visible_realms: Vec<_> = app.tree.realms.iter().collect();
    let realm_count = visible_realms.len();

    for (ri, realm) in visible_realms.iter().enumerate() {
        let realm_is_last = ri == realm_count - 1 && !has_arcs;
        let realm_key = format!("realm:{}", realm.key);
        let realm_collapsed = app.tree.is_collapsed(&realm_key);
        let realm_icon = expand_icon(realm_collapsed);

        let realm_color = hex_to_color(&realm.color);

        let is_cursor = *idx == app.tree_cursor;
        let cursor_char = if is_cursor { ">" } else { " " };
        let layers_count = realm.layers.len();
        let classes_count = realm.total_classes();

        let left_content = format!(
            "{}{}{}  {}",
            cursor_char,
            branch(realm_is_last),
            realm_icon,
            realm.display_name
        );

        let health_str = if let Some((percent, issues)) = realm.health_rollup() {
            format_health_badge(Some(percent), Some(issues))
        } else {
            String::new()
        };
        let stats_str = format!("▦{} ◇{}{}", layers_count, classes_count, health_str);

        let tree_width = area_width.saturating_sub(5) as usize;
        let left_width = display_width(&left_content);
        let stats_width = display_width(&stats_str);

        let total_content = left_width + stats_width + 2;
        let padding = tree_width.saturating_sub(total_content);

        if is_cursor && focused {
            let full_line = format!("{} {}{}", left_content, stats_str, " ".repeat(padding));
            all_lines.push(Line::from(Span::styled(
                full_line,
                Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White),
            )));
        } else {
            let mut spans: Vec<Span> = vec![
                Span::styled(cursor_char, Style::default()),
                Span::styled(
                    branch(realm_is_last).to_string(),
                    Style::default().fg(Color::Magenta),
                ),
                Span::styled(
                    format!("{}  ", realm_icon),
                    Style::default().fg(realm_color),
                ),
            ];
            spans.extend(highlight_matches_with_bg(
                &realm.display_name,
                app.search.matches.get(idx).map(|v| v.as_slice()),
                realm_color,
                None,
            ));
            spans.push(Span::styled(" ", Style::default()));
            spans.push(Span::styled(
                stats_str,
                Style::default().fg(COLOR_MUTED_TEXT),
            ));

            all_lines.push(Line::from(spans));
        }
        *idx += 1;

        if realm_collapsed {
            continue;
        }

        // === LAYERS ===
        let is_data_mode = app.is_graph_mode();
        let hide_empty = app.hide_empty && is_data_mode;

        let visible_layers: Vec<_> = realm
            .layers
            .iter()
            .filter(|l| {
                if hide_empty {
                    l.classes.iter().map(|k| k.instance_count).sum::<i64>() > 0
                } else {
                    true
                }
            })
            .collect();
        let layer_count = visible_layers.len();

        for (li, layer) in visible_layers.iter().enumerate() {
            let layer_is_last = li == layer_count - 1;
            let layer_key = format!("layer:{}:{}", realm.key, layer.key);
            let layer_collapsed = app.tree.is_collapsed(&layer_key);

            let layer_instance_count: i64 =
                layer.classes.iter().map(|k| k.instance_count).sum();

            let layer_icon = expand_icon(layer_collapsed);

            let layer_color = hex_to_color(&layer.color);
            let text_color = layer_color;

            let is_cursor = *idx == app.tree_cursor;
            let cursor_char = if is_cursor { ">" } else { " " };
            let prefix = format!("{}{}", cont(realm_is_last), branch(layer_is_last));
            let classes_in_layer = layer.classes.len();

            let display_name = if is_data_mode {
                format!("{} ({})", layer.display_name, layer_instance_count)
            } else {
                layer.display_name.clone()
            };

            let left_content =
                format!("{}{}{}  {}", cursor_char, prefix, layer_icon, display_name);

            let health_str = if let Some((percent, issues)) = layer.health_rollup() {
                format_health_badge(Some(percent), Some(issues))
            } else {
                String::new()
            };
            let stats_str = format!("◇{}{}", classes_in_layer, health_str);

            let tree_width = area_width.saturating_sub(5) as usize;
            let left_width = display_width(&left_content);
            let stats_width = display_width(&stats_str);
            let right_side = "│";
            let right_width = 1;

            let total_content = left_width + stats_width + right_width + 2;
            let total_padding = tree_width.saturating_sub(total_content);
            let padding_before_stats = total_padding / 2;
            let padding_after_stats = total_padding - padding_before_stats;

            if is_cursor && focused {
                let full_line = format!(
                    "{}{}{}{}{}",
                    left_content,
                    " ".repeat(padding_before_stats + 1),
                    stats_str,
                    " ".repeat(padding_after_stats + 1),
                    right_side
                );
                all_lines.push(Line::from(Span::styled(
                    full_line,
                    Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White),
                )));
            } else {
                let base_style = Style::default();
                let mut spans: Vec<Span> = vec![
                    Span::styled(cursor_char, base_style),
                    Span::styled(prefix.clone(), base_style.fg(realm_color)),
                    Span::styled(format!("{}  ", layer_icon), base_style.fg(text_color)),
                ];
                spans.extend(highlight_matches_with_bg(
                    &display_name,
                    app.search.matches.get(idx).map(|v| v.as_slice()),
                    text_color,
                    None,
                ));
                spans.push(Span::styled(
                    " ".repeat(padding_before_stats + 1),
                    base_style,
                ));
                spans.push(Span::styled(
                    stats_str.clone(),
                    base_style.fg(COLOR_MUTED_TEXT),
                ));
                spans.push(Span::styled(
                    " ".repeat(padding_after_stats + 1),
                    base_style,
                ));
                spans.push(Span::styled("│", base_style.fg(layer_color)));

                all_lines.push(Line::from(spans));
            }
            *idx += 1;

            if layer_collapsed {
                continue;
            }

            // === CLASSES ===
            let visible_classes: Vec<_> = layer
                .classes
                .iter()
                .filter(|k| {
                    if hide_empty {
                        k.instance_count > 0
                    } else {
                        true
                    }
                })
                .collect();
            let class_count = visible_classes.len();

            for (ki, class_info) in visible_classes.iter().enumerate() {
                let class_is_last = ki == class_count - 1;
                let class_key_str = format!("class:{}", class_info.key);
                let class_collapsed = app.tree.is_collapsed(&class_key_str);

                let class_icon = if is_data_mode && class_info.instance_count > 0 {
                    let instances_loaded = if class_info.key == "Entity" {
                        app.tree.has_entity_instances()
                    } else if class_info.key == "EntityNative" {
                        !app.tree.entity_native_groups.is_empty()
                    } else {
                        app.tree.get_instances(&class_info.key).is_some()
                    };
                    if instances_loaded {
                        expand_icon(class_collapsed)
                    } else {
                        expand_icon(true)
                    }
                } else {
                    " "
                };

                let class_is_empty = class_info.instance_count == 0;
                let instance_count = class_info.instance_count;

                let populated_icon = if class_is_empty { "○" } else { "●" };

                let (display_text, class_text_color, count_str, count_color) =
                    if is_data_mode {
                        let cnt_str = if class_is_empty {
                            String::new()
                        } else {
                            format!(" ({})", instance_count)
                        };

                        let text = class_info.display_name.clone();

                        let text_color = if class_is_empty {
                            palette::BRIGHT_DIM
                        } else {
                            Color::White
                        };

                        let cnt_color = if instance_count >= 1000 {
                            Color::Yellow
                        } else if instance_count >= 100 {
                            Color::Cyan
                        } else if instance_count > 0 {
                            Color::Green
                        } else {
                            palette::DIM_110
                        };

                        (text, text_color, cnt_str, cnt_color)
                    } else {
                        (
                            class_info.display_name.clone(),
                            Color::White,
                            String::new(),
                            Color::White,
                        )
                    };

                let prefix = format!(
                    "{}{}{}",
                    cont(realm_is_last),
                    cont(layer_is_last),
                    branch(class_is_last)
                );
                let has_loaded_instances = if class_info.key == "Entity" {
                    app.tree.has_entity_instances()
                } else if class_info.key == "EntityNative" {
                    !app.tree.entity_native_groups.is_empty()
                } else {
                    app.tree
                        .get_instances(&class_info.key)
                        .is_some_and(|i| !i.is_empty())
                };
                let class_has_expanded_instances =
                    is_data_mode && !class_collapsed && has_loaded_instances;
                let class_bg = if class_has_expanded_instances {
                    Some(COLOR_ACTIVE_CLASS_BG)
                } else {
                    None
                };

                let is_cursor = *idx == app.tree_cursor;
                let cursor_char = if is_cursor { ">" } else { " " };

                let left_content = format!(
                    "{}{}{} {} {} {}",
                    cursor_char,
                    prefix,
                    class_icon,
                    populated_icon,
                    count_str,
                    display_text
                );

                let tree_width = area_width.saturating_sub(5) as usize;
                let left_width = display_width(&left_content);
                let right_side = "│";
                let right_width = 1;
                let padding_width = tree_width.saturating_sub(left_width + right_width);

                if is_cursor && focused {
                    let full_line = format!(
                        "{}{}{}",
                        left_content,
                        " ".repeat(padding_width),
                        right_side
                    );
                    all_lines.push(Line::from(Span::styled(
                        full_line,
                        Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White),
                    )));
                } else {
                    let base_style = if let Some(bg) = class_bg {
                        Style::default().bg(bg)
                    } else {
                        Style::default()
                    };

                    let icon_color = if class_is_empty {
                        palette::DIM_110
                    } else {
                        Color::Green
                    };

                    let mut spans: Vec<Span> = vec![
                        Span::styled(cursor_char, base_style),
                        Span::styled(prefix.clone(), base_style.fg(layer_color)),
                        Span::styled(
                            format!("{} ", class_icon),
                            base_style.fg(class_text_color),
                        ),
                        Span::styled(
                            format!("{} ", populated_icon),
                            base_style.fg(icon_color),
                        ),
                        Span::styled(
                            format!("{} ", count_str),
                            base_style.fg(count_color),
                        ),
                    ];

                    spans.extend(highlight_matches_with_bg(
                        &display_text,
                        app.search.matches.get(idx).map(|v| v.as_slice()),
                        class_text_color,
                        class_bg,
                    ));

                    spans.push(Span::styled(" ".repeat(padding_width), base_style));
                    spans.push(Span::styled("│", base_style.fg(layer_color)));

                    all_lines.push(Line::from(spans));
                }
                *idx += 1;

                // === INSTANCES (Data mode only) ===
                if is_data_mode && !class_collapsed {
                    build_instance_lines(
                        all_lines,
                        idx,
                        app,
                        class_info,
                        area_width,
                        focused,
                        realm_is_last,
                        layer_is_last,
                        class_is_last,
                        layer_color,
                    );
                }
            }
        }
    }
}

/// Build instance lines for a specific class.
///
/// Dispatches to Entity, EntityNative, or generic instance rendering.
#[allow(clippy::too_many_arguments)]
fn build_instance_lines<'a>(
    all_lines: &mut Vec<Line<'a>>,
    idx: &mut usize,
    app: &App,
    class_info: &crate::tui::data::ClassInfo,
    area_width: u16,
    focused: bool,
    realm_is_last: bool,
    layer_is_last: bool,
    class_is_last: bool,
    layer_color: Color,
) {
    let cont = cont_char;
    let branch = branch_char;

    if class_info.key == "Entity" {
        // Entity instances (flat list with pillar indicator)
        let all_entities: Vec<_> = app.tree.entity_instances_flat().collect();
        let inst_count = all_entities.len();

        for (ii, instance) in all_entities.iter().enumerate() {
            let inst_is_last = ii == inst_count - 1;
            let is_cursor = *idx == app.tree_cursor;

            let is_pillar = instance
                .properties
                .get("is_pillar")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let icon = if is_pillar { "★" } else { "○" };

            let style = if is_cursor && focused {
                Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
            } else {
                Style::default().fg(COLOR_INSTANCE)
            };

            let cursor_char = if is_cursor { ">" } else { " " };
            let tree_prefix = format!(
                "{}{}{}{}",
                cont(realm_is_last),
                cont(layer_is_last),
                cont(class_is_last),
                branch(inst_is_last)
            );

            if is_cursor && focused {
                all_lines.push(Line::from(Span::styled(
                    format!(
                        "{}{}{} {}",
                        cursor_char, tree_prefix, icon, instance.display_name,
                    ),
                    style,
                )));
            } else {
                let spans = vec![
                    Span::styled(cursor_char, Style::default()),
                    Span::styled(tree_prefix, Style::default().fg(layer_color)),
                    Span::styled(
                        format!("{} {}", icon, instance.display_name),
                        style,
                    ),
                ];
                all_lines.push(Line::from(spans));
            }
            *idx += 1;
        }
    } else if class_info.key == "EntityNative" && !app.tree.entity_native_groups.is_empty() {
        // EntityNative grouped by entity with power bars
        build_entity_native_lines(
            all_lines,
            idx,
            app,
            area_width,
            focused,
            realm_is_last,
            layer_is_last,
            class_is_last,
            layer_color,
        );
    } else if let Some(instances) = app.tree.get_instances(&class_info.key) {
        // Generic instances (Locale, Page, Block, etc.)
        let inst_count = instances.len();
        for (ii, instance) in instances.iter().enumerate() {
            let inst_is_last = ii == inst_count - 1;
            let is_cursor = *idx == app.tree_cursor;

            let is_primary = instance
                .properties
                .get("is_primary")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let fallback_count = if is_primary {
                instance
                    .incoming_arcs
                    .iter()
                    .filter(|a| a.arc_type == "FALLBACK_TO")
                    .count()
            } else {
                0
            };

            let icon = if is_primary { "●" } else { "○" };
            let base_color = COLOR_INSTANCE;

            let style = if is_cursor && focused {
                Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
            } else {
                Style::default().fg(base_color)
            };

            let cursor_char = if is_cursor { ">" } else { " " };
            let suffix = if is_primary && fallback_count > 0 {
                format!(" [{}↓]", fallback_count)
            } else {
                String::new()
            };

            let tree_prefix = format!(
                "{}{}{}{}",
                cont(realm_is_last),
                cont(layer_is_last),
                cont(class_is_last),
                branch(inst_is_last)
            );

            if is_cursor && focused {
                all_lines.push(Line::from(Span::styled(
                    format!(
                        "{}{}{} {}{}",
                        cursor_char, tree_prefix, icon, instance.display_name, suffix,
                    ),
                    style,
                )));
            } else {
                let spans = vec![
                    Span::styled(cursor_char, Style::default()),
                    Span::styled(tree_prefix, Style::default().fg(layer_color)),
                    Span::styled(
                        format!("{} {}{}", icon, instance.display_name, suffix),
                        style,
                    ),
                ];
                all_lines.push(Line::from(spans));
            }
            *idx += 1;
        }
    }
}

/// Build EntityNative group lines with power bars and expandable natives.
#[allow(clippy::too_many_arguments)]
fn build_entity_native_lines<'a>(
    all_lines: &mut Vec<Line<'a>>,
    idx: &mut usize,
    app: &App,
    area_width: u16,
    focused: bool,
    realm_is_last: bool,
    layer_is_last: bool,
    class_is_last: bool,
    layer_color: Color,
) {
    use unicode_width::UnicodeWidthStr;

    let cont = cont_char;
    let branch = branch_char;
    let tree_width = area_width.saturating_sub(5) as usize;

    let group_count = app.tree.entity_native_groups.len();
    for (gi, entity_group) in app.tree.entity_native_groups.iter().enumerate() {
        let group_is_last = gi == group_count - 1;
        let is_cursor = *idx == app.tree_cursor;

        let group_key = format!("entity_group:{}", entity_group.entity_key);
        let is_collapsed = app.tree.is_collapsed(&group_key);
        let expand_icon = if is_collapsed { "▶" } else { "▼" };

        let style = if is_cursor && focused {
            Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
        } else {
            Style::default().fg(COLOR_ENTITY_TEXT)
        };

        let cursor_char = if is_cursor { ">" } else { " " };
        let tree_prefix = format!(
            "{}{}{}{}",
            cont(realm_is_last),
            cont(layer_is_last),
            cont(class_is_last),
            branch(group_is_last)
        );

        let (power_bar, power_color) =
            render_power_bar(entity_group.relationship_power);

        let left_content = format!(
            "{}{}{} {} ({})",
            cursor_char,
            tree_prefix,
            expand_icon,
            entity_group.entity_key,
            entity_group.native_count,
        );
        let left_width = UnicodeWidthStr::width(left_content.as_str());
        let power_bar_width = UnicodeWidthStr::width(power_bar);
        let padding_width = tree_width.saturating_sub(left_width + power_bar_width + 1);

        if is_cursor && focused {
            all_lines.push(Line::from(Span::styled(
                format!(
                    "{}{}{}",
                    left_content,
                    " ".repeat(padding_width),
                    power_bar,
                ),
                style,
            )));
        } else {
            let spans = vec![
                Span::styled(cursor_char, Style::default()),
                Span::styled(tree_prefix, Style::default().fg(layer_color)),
                Span::styled(
                    format!(
                        "{} {} ({})",
                        expand_icon, entity_group.entity_key, entity_group.native_count
                    ),
                    style,
                ),
                Span::styled(" ".repeat(padding_width), Style::default()),
                Span::styled(power_bar, Style::default().fg(power_color)),
            ];
            all_lines.push(Line::from(spans));
        }
        *idx += 1;

        // Render EntityNatives for this entity when expanded
        if !is_collapsed {
            if let Some(natives) = app
                .tree
                .entity_native_by_entity
                .get(&entity_group.entity_key)
            {
                let native_parts: Vec<_> = natives
                    .iter()
                    .map(|native| {
                        let flag = locale_to_flag(&native.locale_code);
                        let left = format!(
                            "{} {} - {}",
                            flag, native.locale_code, native.display_name
                        );
                        (left, native.slug.as_deref())
                    })
                    .collect();

                let native_count_inner = native_parts.len();
                for (ni, (left_part, slug_opt)) in native_parts.iter().enumerate() {
                    let native_is_last = ni == native_count_inner - 1;
                    let is_native_cursor = *idx == app.tree_cursor;

                    let native_style = if is_native_cursor && focused {
                        Style::default()
                            .bg(COLOR_HIGHLIGHT_BG)
                            .fg(Color::White)
                    } else {
                        Style::default().fg(COLOR_ENTITY_TEXT)
                    };

                    let slug_style = if is_native_cursor && focused {
                        Style::default()
                            .bg(COLOR_HIGHLIGHT_BG)
                            .fg(Color::White)
                    } else {
                        Style::default().fg(COLOR_ENTITY_SLUG)
                    };

                    let native_cursor = if is_native_cursor { ">" } else { " " };
                    let native_tree_prefix = format!(
                        "{}{}{}{}{}",
                        cont(realm_is_last),
                        cont(layer_is_last),
                        cont(class_is_last),
                        cont(group_is_last),
                        branch(native_is_last)
                    );

                    let slug_display = match slug_opt {
                        Some(slug) => format!("/{}", slug),
                        None => String::new(),
                    };

                    let prefix_len =
                        1 + UnicodeWidthStr::width(native_tree_prefix.as_str());
                    let left_width = UnicodeWidthStr::width(left_part.as_str());
                    let slug_width =
                        UnicodeWidthStr::width(slug_display.as_str());
                    let total_content = prefix_len + left_width + slug_width;
                    let padding_width =
                        tree_width.saturating_sub(total_content + 1);

                    if is_native_cursor && focused {
                        all_lines.push(Line::from(Span::styled(
                            format!(
                                "{}{}{}{}{}",
                                native_cursor,
                                native_tree_prefix,
                                left_part,
                                " ".repeat(padding_width),
                                slug_display,
                            ),
                            native_style,
                        )));
                    } else {
                        let spans = vec![
                            Span::styled(native_cursor, Style::default()),
                            Span::styled(
                                native_tree_prefix.clone(),
                                Style::default().fg(layer_color),
                            ),
                            Span::styled(left_part.clone(), native_style),
                            Span::styled(
                                " ".repeat(padding_width),
                                Style::default(),
                            ),
                            Span::styled(slug_display.clone(), slug_style),
                        ];
                        all_lines.push(Line::from(spans));
                    }
                    *idx += 1;
                }
            }
        }
    }
}
