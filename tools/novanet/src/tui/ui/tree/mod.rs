//! Tree panel rendering for TUI v2.
//!
//! Renders the taxonomy hierarchy with:
//! - Box-drawing characters for visual structure
//! - Collapse/expand state management
//! - Fuzzy search match highlighting
//! - Data mode instance display
//! - Filtered instances view

mod breadcrumb;
mod filtered;
mod helpers;
mod highlight;
mod minimap;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};

use self::breadcrumb::render_breadcrumb;
use self::filtered::render_filtered_instances;
use self::helpers::{branch_char, cont_char, expand_icon, format_health_badge};
use self::highlight::highlight_matches_with_bg;
use self::minimap::{build_minimap_info, render_minimap};
use super::{
    COLOR_ACTIVE_CLASS_BG, COLOR_ARC_FAMILY, COLOR_DESC_TEXT, COLOR_HIGHLIGHT_BG, COLOR_INSTANCE,
    COLOR_MUTED_TEXT, COLOR_UNFOCUSED_BORDER, EmptyStateClass, STYLE_DIM, cardinality_abbrev,
    render_empty_state,
};
use crate::tui::app::{App, Focus};
use crate::tui::data::locale_to_flag;
use crate::tui::palette;
use crate::tui::theme::hex_to_color;
use crate::tui::unicode::display_width;
use crate::tui::widgets::bordered_block;

// =============================================================================
// CONSTANTS
// =============================================================================

/// Horizontal padding inside tree panel (left side only, right has minimap).
const TREE_PADDING_LEFT: u16 = 1;
const SCROLLBAR_WIDTH: u16 = 1;

// =============================================================================
// POWER BAR RENDERING (Entity relationship visualization)
// =============================================================================

/// Power bar width (10 filled/empty characters)
const POWER_BAR_WIDTH: usize = 10;

/// Power bar color thresholds (Tailwind colors via palette)
const COLOR_POWER_HIGH: Color = palette::GREEN_500;
const COLOR_POWER_MED: Color = palette::ORANGE_500;
const COLOR_POWER_LOW: Color = palette::RED_500;

/// Pre-computed power bar strings (v0.17.3: zero-allocation optimization)
/// Index 0 = 0% filled, Index 10 = 100% filled
const POWER_BARS: [&str; 11] = [
    "▱▱▱▱▱▱▱▱▱▱", // 0/10
    "▰▱▱▱▱▱▱▱▱▱", // 1/10
    "▰▰▱▱▱▱▱▱▱▱", // 2/10
    "▰▰▰▱▱▱▱▱▱▱", // 3/10
    "▰▰▰▰▱▱▱▱▱▱", // 4/10
    "▰▰▰▰▰▱▱▱▱▱", // 5/10
    "▰▰▰▰▰▰▱▱▱▱", // 6/10
    "▰▰▰▰▰▰▰▱▱▱", // 7/10
    "▰▰▰▰▰▰▰▰▱▱", // 8/10
    "▰▰▰▰▰▰▰▰▰▱", // 9/10
    "▰▰▰▰▰▰▰▰▰▰", // 10/10
];

/// Render power bar with color based on percentage.
/// Returns (&'static bar_string, color) where bar looks like: ▰▰▰▰▰▰▰▰▱▱
/// Zero-allocation using lookup table instead of format!
#[inline]
fn render_power_bar(power: u8) -> (&'static str, Color) {
    let filled = (power as usize * POWER_BAR_WIDTH / 100).min(POWER_BAR_WIDTH);

    let color = if power >= 80 {
        COLOR_POWER_HIGH
    } else if power >= 50 {
        COLOR_POWER_MED
    } else {
        COLOR_POWER_LOW
    };

    (POWER_BARS[filled], color)
}

/// Entity/EntityNative text color (white, not yellow)
const COLOR_ENTITY_TEXT: Color = Color::White;

/// Entity slug color (slate-400, same as EntityNative slug)
const COLOR_ENTITY_SLUG: Color = palette::ENTITY_SLUG;

// =============================================================================
// TREE RENDERING
// =============================================================================

/// Tree panel: taxonomy hierarchy with scroll and collapse.
/// Uses box-drawing characters for visual hierarchy.
pub fn render_tree(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.focus == Focus::Tree;
    let border_color = if focused {
        Color::Cyan
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Calculate visible height (area minus borders)
    let visible_height = area.height.saturating_sub(2) as usize;
    app.tree_height = visible_height;

    // === EMPTY STATE: No node classes loaded ===
    let total_classes: usize = app
        .tree
        .realms
        .iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.classes.len())
        .sum();

    if total_classes == 0 {
        let empty_title = if app.is_graph_mode() {
            " ● Data "
        } else {
            " ◆ Schema "
        };
        let block = bordered_block(empty_title, border_color);
        f.render_widget(block, area);

        let inner_area = Rect::new(
            area.x + 1,
            area.y + 1,
            area.width.saturating_sub(2),
            area.height.saturating_sub(2),
        );
        render_empty_state(f, inner_area, EmptyStateClass::NoClasses, app.tick);
        return;
    }

    // === FILTERED DATA MODE: Show only instances of selected Class ===
    if let Some(class_key) = app.get_filter_class() {
        render_filtered_instances(
            f,
            area,
            app,
            class_key,
            visible_height,
            focused,
            border_color,
        );
        return;
    }

    // Build all visible tree lines
    let mut all_lines: Vec<Line> = Vec::new();
    let mut idx = 0;

    // Helper to create a tree line with box-drawing
    let make_line = |idx: usize,
                     cursor: usize,
                     focused: bool,
                     tree_prefix: &str,
                     icon: &str,
                     text: String,
                     line_color: Color,
                     text_color: Color,
                     match_positions: Option<&[u32]>,
                     bg_color: Option<Color>|
     -> Line {
        let is_cursor = idx == cursor;
        let cursor_char = if is_cursor { ">" } else { " " };
        let icon_space = if icon.is_empty() { "" } else { " " };

        if is_cursor && focused {
            let style = Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White);
            Line::from(Span::styled(
                format!(
                    "{}{}{}{}{}",
                    cursor_char, tree_prefix, icon, icon_space, text
                ),
                style,
            ))
        } else {
            let base_style = if let Some(bg) = bg_color {
                Style::default().bg(bg)
            } else {
                Style::default()
            };
            let mut spans = Vec::with_capacity(8);
            spans.push(Span::styled(cursor_char, base_style));
            if !tree_prefix.is_empty() {
                spans.push(Span::styled(
                    tree_prefix.to_string(),
                    base_style.fg(line_color),
                ));
            }
            spans.push(Span::styled(
                format!("{}{}", icon, icon_space),
                base_style.fg(text_color),
            ));
            spans.extend(highlight_matches_with_bg(
                &text,
                match_positions,
                text_color,
                bg_color,
            ));
            Line::from(spans)
        }
    };

    // Box-drawing helpers
    let branch = branch_char;
    let cont = cont_char;

    // === KINDS SECTION ===
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
        idx,
        app.tree_cursor,
        focused,
        "",
        classes_icon,
        format!("Node Classes ({})", classes_count),
        Color::Magenta,
        Color::Magenta,
        app.search.matches.get(&idx).map(|v| v.as_slice()),
        None,
    ));
    idx += 1;

    let has_arcs = !app.tree.arc_families.is_empty();

    // Show all realms
    if !classes_collapsed {
        let visible_realms: Vec<_> = app.tree.realms.iter().collect();
        let realm_count = visible_realms.len();

        for (ri, realm) in visible_realms.iter().enumerate() {
            let realm_is_last = ri == realm_count - 1 && !has_arcs;
            let realm_key = format!("realm:{}", realm.key);
            let realm_collapsed = app.tree.is_collapsed(&realm_key);
            let realm_icon = expand_icon(realm_collapsed);

            let realm_color = hex_to_color(&realm.color);

            let is_cursor = idx == app.tree_cursor;
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

            let tree_width = area.width.saturating_sub(5) as usize;
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
                    app.search.matches.get(&idx).map(|v| v.as_slice()),
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
            idx += 1;

            if !realm_collapsed {
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

                    let is_cursor = idx == app.tree_cursor;
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

                    let tree_width = area.width.saturating_sub(5) as usize;
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
                            app.search.matches.get(&idx).map(|v| v.as_slice()),
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
                    idx += 1;

                    if !layer_collapsed {
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

                            let is_cursor = idx == app.tree_cursor;
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

                            let tree_width = area.width.saturating_sub(5) as usize;
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
                                    app.search.matches.get(&idx).map(|v| v.as_slice()),
                                    class_text_color,
                                    class_bg,
                                ));

                                spans.push(Span::styled(" ".repeat(padding_width), base_style));
                                spans.push(Span::styled("│", base_style.fg(layer_color)));

                                all_lines.push(Line::from(spans));
                            }
                            idx += 1;

                            // In Data mode, show instances under Class (if not collapsed)
                            if is_data_mode && !class_collapsed {
                                if class_info.key == "Entity" {
                                    let all_entities: Vec<_> =
                                        app.tree.entity_instances_flat().collect();
                                    let inst_count = all_entities.len();

                                    for (ii, instance) in all_entities.iter().enumerate() {
                                        let inst_is_last = ii == inst_count - 1;
                                        let is_cursor = idx == app.tree_cursor;

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
                                                    cursor_char,
                                                    tree_prefix,
                                                    icon,
                                                    instance.display_name,
                                                ),
                                                style,
                                            )));
                                        } else {
                                            let spans = vec![
                                                Span::styled(cursor_char, Style::default()),
                                                Span::styled(
                                                    tree_prefix,
                                                    Style::default().fg(layer_color),
                                                ),
                                                Span::styled(
                                                    format!("{} {}", icon, instance.display_name),
                                                    style,
                                                ),
                                            ];
                                            all_lines.push(Line::from(spans));
                                        }
                                        idx += 1;
                                    }
                                } else if class_info.key == "EntityNative"
                                    && !app.tree.entity_native_groups.is_empty()
                                {
                                    use unicode_width::UnicodeWidthStr;

                                    let group_count = app.tree.entity_native_groups.len();
                                    for (gi, entity_group) in
                                        app.tree.entity_native_groups.iter().enumerate()
                                    {
                                        let group_is_last = gi == group_count - 1;
                                        let is_cursor = idx == app.tree_cursor;

                                        let group_key =
                                            format!("entity_group:{}", entity_group.entity_key);
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

                                        let tree_width = area.width.saturating_sub(5) as usize;
                                        let left_content = format!(
                                            "{}{}{} {} ({})",
                                            cursor_char,
                                            tree_prefix,
                                            expand_icon,
                                            entity_group.entity_key,
                                            entity_group.native_count,
                                        );
                                        let left_width =
                                            UnicodeWidthStr::width(left_content.as_str());
                                        let power_bar_width = UnicodeWidthStr::width(power_bar);
                                        let padding_width = tree_width
                                            .saturating_sub(left_width + power_bar_width + 1);

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
                                                Span::styled(
                                                    tree_prefix,
                                                    Style::default().fg(layer_color),
                                                ),
                                                Span::styled(
                                                    format!(
                                                        "{} {} ({})",
                                                        expand_icon,
                                                        entity_group.entity_key,
                                                        entity_group.native_count
                                                    ),
                                                    style,
                                                ),
                                                Span::styled(
                                                    " ".repeat(padding_width),
                                                    Style::default(),
                                                ),
                                                Span::styled(
                                                    power_bar,
                                                    Style::default().fg(power_color),
                                                ),
                                            ];
                                            all_lines.push(Line::from(spans));
                                        }
                                        idx += 1;

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
                                                        let flag =
                                                            locale_to_flag(&native.locale_code);
                                                        let left = format!(
                                                            "{} {} - {}",
                                                            flag,
                                                            native.locale_code,
                                                            native.display_name
                                                        );
                                                        (left, native.slug.as_deref())
                                                    })
                                                    .collect();

                                                let native_count_inner = native_parts.len();
                                                for (ni, (left_part, slug_opt)) in
                                                    native_parts.iter().enumerate()
                                                {
                                                    let native_is_last =
                                                        ni == native_count_inner - 1;
                                                    let is_native_cursor = idx == app.tree_cursor;

                                                    let native_style =
                                                        if is_native_cursor && focused {
                                                            Style::default()
                                                                .bg(COLOR_HIGHLIGHT_BG)
                                                                .fg(Color::White)
                                                        } else {
                                                            Style::default().fg(COLOR_ENTITY_TEXT)
                                                        };

                                                    let slug_style = if is_native_cursor && focused
                                                    {
                                                        Style::default()
                                                            .bg(COLOR_HIGHLIGHT_BG)
                                                            .fg(Color::White)
                                                    } else {
                                                        Style::default().fg(COLOR_ENTITY_SLUG)
                                                    };

                                                    let native_cursor =
                                                        if is_native_cursor { ">" } else { " " };
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

                                                    let prefix_len = 1 + UnicodeWidthStr::width(
                                                        native_tree_prefix.as_str(),
                                                    );
                                                    let left_width =
                                                        UnicodeWidthStr::width(left_part.as_str());
                                                    let slug_width = UnicodeWidthStr::width(
                                                        slug_display.as_str(),
                                                    );
                                                    let total_content =
                                                        prefix_len + left_width + slug_width;
                                                    let padding_width = tree_width
                                                        .saturating_sub(total_content + 1);

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
                                                            Span::styled(
                                                                native_cursor,
                                                                Style::default(),
                                                            ),
                                                            Span::styled(
                                                                native_tree_prefix.clone(),
                                                                Style::default().fg(layer_color),
                                                            ),
                                                            Span::styled(
                                                                left_part.clone(),
                                                                native_style,
                                                            ),
                                                            Span::styled(
                                                                " ".repeat(padding_width),
                                                                Style::default(),
                                                            ),
                                                            Span::styled(
                                                                slug_display.clone(),
                                                                slug_style,
                                                            ),
                                                        ];
                                                        all_lines.push(Line::from(spans));
                                                    }
                                                    idx += 1;
                                                }
                                            }
                                        }
                                    }
                                } else if let Some(instances) =
                                    app.tree.get_instances(&class_info.key)
                                {
                                    let inst_count = instances.len();
                                    for (ii, instance) in instances.iter().enumerate() {
                                        let inst_is_last = ii == inst_count - 1;
                                        let is_cursor = idx == app.tree_cursor;

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
                                                    cursor_char,
                                                    tree_prefix,
                                                    icon,
                                                    instance.display_name,
                                                    suffix,
                                                ),
                                                style,
                                            )));
                                        } else {
                                            let spans = vec![
                                                Span::styled(cursor_char, Style::default()),
                                                Span::styled(
                                                    tree_prefix,
                                                    Style::default().fg(layer_color),
                                                ),
                                                Span::styled(
                                                    format!(
                                                        "{} {}{}",
                                                        icon, instance.display_name, suffix
                                                    ),
                                                    style,
                                                ),
                                            ];
                                            all_lines.push(Line::from(spans));
                                        }
                                        idx += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // === RELATIONS SECTION ===
    let arcs_collapsed = app.tree.is_collapsed("arcs");
    let arcs_icon = expand_icon(arcs_collapsed);
    let arcs_count: usize = app
        .tree
        .arc_families
        .iter()
        .map(|f| f.arc_classes.len())
        .sum();
    all_lines.push(make_line(
        idx,
        app.tree_cursor,
        focused,
        "",
        arcs_icon,
        format!("Arcs ({})", arcs_count),
        Color::Yellow,
        Color::Yellow,
        app.search.matches.get(&idx).map(|v| v.as_slice()),
        None,
    ));
    idx += 1;

    if !arcs_collapsed {
        let family_count = app.tree.arc_families.len();
        for (fi, family) in app.tree.arc_families.iter().enumerate() {
            let family_is_last = fi == family_count - 1;
            let family_key = format!("family:{}", family.key);
            let family_collapsed = app.tree.is_collapsed(&family_key);
            let family_icon = expand_icon(family_collapsed);

            let is_cursor = idx == app.tree_cursor;
            let cursor_char = if is_cursor { ">" } else { " " };
            let arcs_in_family = family.arc_classes.len();

            let left_content = format!(
                "{}{}{}  {}",
                cursor_char,
                branch(family_is_last),
                family_icon,
                family.display_name
            );

            let stats_str = format!("◇{}", arcs_in_family);

            if is_cursor && focused {
                let full_line = format!("{} {}", left_content, stats_str);
                all_lines.push(Line::from(Span::styled(
                    full_line,
                    Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White),
                )));
            } else {
                let base_style = Style::default();
                let mut spans: Vec<Span> = vec![
                    Span::styled(cursor_char, base_style),
                    Span::styled(
                        branch(family_is_last).to_string(),
                        base_style.fg(Color::Yellow),
                    ),
                    Span::styled(
                        format!("{}  ", family_icon),
                        base_style.fg(COLOR_ARC_FAMILY),
                    ),
                ];
                spans.extend(highlight_matches_with_bg(
                    &family.display_name,
                    app.search.matches.get(&idx).map(|v| v.as_slice()),
                    COLOR_ARC_FAMILY,
                    None,
                ));
                spans.push(Span::styled(" ", base_style));
                spans.push(Span::styled(stats_str, base_style.fg(COLOR_MUTED_TEXT)));

                all_lines.push(Line::from(spans));
            }
            idx += 1;

            if !family_collapsed {
                let arc_count = family.arc_classes.len();
                for (ai, arc_class) in family.arc_classes.iter().enumerate() {
                    let arc_is_last = ai == arc_count - 1;

                    let is_cursor = idx == app.tree_cursor;
                    let cursor_char = if is_cursor { ">" } else { " " };
                    let prefix = format!("{}{}", cont(family_is_last), branch(arc_is_last));

                    let left_content =
                        format!("{}{}  {}", cursor_char, prefix, arc_class.display_name);

                    let from_abbrev = arc_class.from_class.chars().take(8).collect::<String>();
                    let to_abbrev = arc_class.to_class.chars().take(8).collect::<String>();
                    let flow_str = format!("{}→{}", from_abbrev, to_abbrev);

                    let card_str = cardinality_abbrev(&arc_class.cardinality);

                    let tree_width = area.width.saturating_sub(5) as usize;
                    let left_width = display_width(&left_content);
                    let flow_width = display_width(&flow_str);
                    let card_width = display_width(card_str);
                    let right_side = "│";
                    let right_width = 1;

                    let total_content = left_width + flow_width + card_width + right_width + 3;
                    let padding = tree_width.saturating_sub(total_content);

                    if is_cursor && focused {
                        let full_line =
                            format!("{} {} {} {}", left_content, flow_str, card_str, right_side);
                        all_lines.push(Line::from(Span::styled(
                            full_line,
                            Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White),
                        )));
                    } else {
                        let base_style = Style::default();
                        let mut spans: Vec<Span> = vec![
                            Span::styled(cursor_char, base_style),
                            Span::styled(prefix.clone(), base_style.fg(COLOR_ARC_FAMILY)),
                            Span::styled("  ", base_style),
                        ];
                        spans.extend(highlight_matches_with_bg(
                            &arc_class.display_name,
                            app.search.matches.get(&idx).map(|v| v.as_slice()),
                            COLOR_DESC_TEXT,
                            None,
                        ));
                        spans.push(Span::styled(" ", base_style));
                        spans.push(Span::styled(flow_str, base_style.fg(COLOR_MUTED_TEXT)));
                        spans.push(Span::styled(" ", base_style));
                        spans.push(Span::styled(card_str, base_style.fg(Color::Cyan)));
                        spans.push(Span::styled(" ".repeat(padding), base_style));
                        spans.push(Span::styled("│", base_style.fg(COLOR_ARC_FAMILY)));

                        all_lines.push(Line::from(spans));
                    }
                    idx += 1;
                }
            }
        }
    }

    // Apply scroll - only show visible lines
    let lines: Vec<Line> = all_lines
        .into_iter()
        .skip(app.tree_scroll)
        .take(visible_height)
        .collect();

    // Handle empty tree
    let lines = if lines.is_empty() {
        vec![Line::from(Span::styled("  No data loaded", STYLE_DIM))]
    } else {
        lines
    };

    // Show hierarchical position in title
    let total = app.current_item_count();
    let mode_prefix = if app.is_graph_mode() {
        "● Data"
    } else {
        "◆ Schema"
    };


    let hierarchy =
        app.tree
            .hierarchy_position(app.tree_cursor, app.is_graph_mode(), app.hide_empty);
    let hierarchy_str = hierarchy.to_compact_string();
    let title = if hierarchy_str.is_empty() {
        format!(" {} ", mode_prefix)
    } else {
        format!(" {} │ {} ", mode_prefix, hierarchy_str)
    };

    // Render block with title (square borders for main tree panel)
    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Reserve space for mini-map (2 chars + 1 separator) and add left padding
    let minimap_width: u16 = 3;
    let content_x = inner_area.x + TREE_PADDING_LEFT;
    let content_width = inner_area
        .width
        .saturating_sub(minimap_width + TREE_PADDING_LEFT + SCROLLBAR_WIDTH);

    // Render sticky breadcrumb at top of content area (with padding)
    let breadcrumb_area = Rect::new(content_x, inner_area.y, content_width, inner_area.height);
    let breadcrumb_height = render_breadcrumb(f, breadcrumb_area, app);

    // Calculate tree area below breadcrumb (with separator line)
    let separator_height = if breadcrumb_height > 0 { 1 } else { 0 };
    let tree_y = inner_area.y + breadcrumb_height + separator_height;
    let tree_height = inner_area
        .height
        .saturating_sub(breadcrumb_height + separator_height);

    // Render separator line if breadcrumb exists (with padding)
    if breadcrumb_height > 0 && content_width > 0 {
        let separator_area = Rect::new(
            content_x,
            inner_area.y + breadcrumb_height,
            content_width,
            1,
        );
        let separator = Paragraph::new(Line::from(Span::styled(
            "─".repeat(content_width as usize),
            Style::default().fg(COLOR_MUTED_TEXT),
        )));
        f.render_widget(separator, separator_area);
    }

    // Render tree content below breadcrumb (with padding)
    let tree_area = Rect::new(content_x, tree_y, content_width, tree_height);

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, tree_area);

    // Render mini-map on right side
    let sep_x = inner_area.x + inner_area.width - minimap_width;
    let minimap_area = Rect::new(
        sep_x + 1,
        inner_area.y,
        2,
        inner_area.height,
    );
    let minimap_info = build_minimap_info(app, tree_height as usize);
    render_minimap(f, minimap_area, &minimap_info);

    // Render vertical separator between tree and mini-map
    if inner_area.height > 0 {
        let sep_area = Rect::new(sep_x, inner_area.y, 1, inner_area.height);
        let mut sep_lines: Vec<Line> = Vec::with_capacity(inner_area.height as usize);
        for _ in 0..inner_area.height {
            sep_lines.push(Line::from(Span::styled(
                "│",
                Style::default().fg(palette::EMPTY_SLOT),
            )));
        }
        let sep_paragraph = Paragraph::new(sep_lines);
        f.render_widget(sep_paragraph, sep_area);
    }

    // Add scrollbar if content exceeds visible area
    let effective_visible_height = tree_height as usize;
    if total > effective_visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state =
            ScrollbarState::new(total.saturating_sub(effective_visible_height))
                .position(app.tree_scroll);

        let scrollbar_area = Rect {
            x: content_x + content_width,
            y: tree_y,
            width: SCROLLBAR_WIDTH,
            height: tree_height,
        };
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}
