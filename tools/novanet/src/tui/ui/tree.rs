//! Tree panel rendering for TUI v2.
//!
//! Renders the taxonomy hierarchy with:
//! - Box-drawing characters for visual structure
//! - Collapse/expand state management
//! - Fuzzy search match highlighting
//! - Data mode instance display
//! - Filtered instances view

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use rustc_hash::FxHashSet;

use super::{
    COLOR_ACTIVE_CLASS_BG, COLOR_ARC_FAMILY, COLOR_DESC_TEXT, COLOR_HIGHLIGHT_BG, COLOR_INSTANCE,
    COLOR_MUTED_TEXT, COLOR_UNFOCUSED_BORDER, EmptyStateClass, STYLE_DIM, STYLE_HIGHLIGHT,
    STYLE_PRIMARY, STYLE_UNFOCUSED, cardinality_abbrev, layer_badge_icon, realm_badge_icon,
    render_empty_state, spinner, trait_icon,
};
use crate::tui::app::{App, Focus};
use crate::tui::data::ArcDirection;
use crate::tui::theme::hex_to_color;
use crate::tui::unicode::display_width;

/// Create styled spans with fuzzy match highlighting and optional background.
/// Matched character positions are shown with a yellow highlight.
/// Optional background color is applied to non-matched text segments.
fn highlight_matches_with_bg(
    text: &str,
    matches: Option<&[u32]>,
    base_color: Color,
    bg_color: Option<Color>,
) -> Vec<Span<'static>> {
    let base_style = if let Some(bg) = bg_color {
        Style::default().fg(base_color).bg(bg)
    } else {
        Style::default().fg(base_color)
    };

    let Some(positions) = matches else {
        return vec![Span::styled(text.to_string(), base_style)];
    };

    if positions.is_empty() {
        return vec![Span::styled(text.to_string(), base_style)];
    }

    let match_set: FxHashSet<usize> = positions.iter().map(|&p| p as usize).collect();
    let mut spans = Vec::with_capacity(positions.len() * 2 + 1);
    let mut current_text = String::new();
    let mut in_match = false;

    for (i, c) in text.chars().enumerate() {
        let is_match = match_set.contains(&i);

        if is_match != in_match {
            if !current_text.is_empty() {
                let style = if in_match {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    base_style
                };
                spans.push(Span::styled(std::mem::take(&mut current_text), style));
            }
            in_match = is_match;
        }
        current_text.push(c);
    }

    if !current_text.is_empty() {
        let style = if in_match {
            Style::default().fg(Color::Black).bg(Color::Yellow)
        } else {
            base_style
        };
        spans.push(Span::styled(current_text, style));
    }

    spans
}

// =============================================================================
// CONSTANTS
// =============================================================================

/// Horizontal padding inside tree panel (left side only, right has minimap).
const TREE_PADDING_LEFT: u16 = 1;

// =============================================================================
// BREADCRUMB RENDERING (v11.6)
// =============================================================================

/// A single level in the breadcrumb path.
struct BreadcrumbLevel {
    icon: &'static str,
    label: String,
    color: Color,
}

/// Build breadcrumb path from current selection.
/// Returns a vector of levels from root to current item.
fn build_breadcrumb_path(app: &App) -> Vec<BreadcrumbLevel> {
    use crate::tui::data::TreeItem;

    let mut path = Vec::new();

    match app.current_item() {
        Some(TreeItem::Realm(r)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
        }
        Some(TreeItem::Layer(r, l)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
        }
        Some(TreeItem::Class(r, l, k)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            let class_label = if app.is_graph_mode() && k.instance_count > 0 {
                format!("{} ({})", k.display_name, k.instance_count)
            } else {
                k.display_name.clone()
            };
            path.push(BreadcrumbLevel {
                icon: trait_icon(&k.trait_name),
                label: class_label,
                color: app.theme.trait_color(&k.trait_name),
            });
        }
        Some(TreeItem::EntityCategory(r, l, k, cat)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: trait_icon(&k.trait_name),
                label: k.display_name.clone(),
                color: app.theme.trait_color(&k.trait_name),
            });
            path.push(BreadcrumbLevel {
                icon: "◫",
                label: cat.display_name.clone(),
                color: Color::Gray,
            });
        }
        Some(TreeItem::Instance(r, l, k, inst)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: trait_icon(&k.trait_name),
                label: k.display_name.clone(),
                color: app.theme.trait_color(&k.trait_name),
            });
            path.push(BreadcrumbLevel {
                icon: "►",
                label: inst.display_name.clone(),
                color: COLOR_INSTANCE,
            });
        }
        Some(TreeItem::ArcFamily(f)) => {
            path.push(BreadcrumbLevel {
                icon: "⊶",
                label: "Arcs".to_string(),
                color: Color::Magenta,
            });
            path.push(BreadcrumbLevel {
                icon: "◇",
                label: f.display_name.clone(),
                color: Color::Magenta,
            });
        }
        Some(TreeItem::ArcClass(f, ak)) => {
            path.push(BreadcrumbLevel {
                icon: "⊶",
                label: "Arcs".to_string(),
                color: Color::Magenta,
            });
            path.push(BreadcrumbLevel {
                icon: "◇",
                label: f.display_name.clone(),
                color: Color::Magenta,
            });
            path.push(BreadcrumbLevel {
                icon: "→",
                label: ak.display_name.clone(),
                color: Color::White,
            });
        }
        Some(TreeItem::ClassesSection) => {
            path.push(BreadcrumbLevel {
                icon: "◈",
                label: "Node Classes".to_string(),
                color: Color::Cyan,
            });
        }
        Some(TreeItem::ArcsSection) => {
            path.push(BreadcrumbLevel {
                icon: "⊶",
                label: "Arcs".to_string(),
                color: Color::Magenta,
            });
        }
        None => {}
    }

    path
}

/// Render sticky breadcrumb at top of tree panel.
/// Returns the height used (always 1 line for consistent layout).
fn render_breadcrumb(f: &mut Frame, area: Rect, app: &App) -> u16 {
    let path = build_breadcrumb_path(app);

    // Always render 1 line for consistent header height
    let breadcrumb_area = Rect::new(area.x, area.y, area.width, 1);

    if path.is_empty() {
        // Empty breadcrumb: show subtle placeholder
        let line = Line::from(Span::styled(
            " ◇ Select an item",
            Style::default().fg(Color::Rgb(80, 80, 100)),
        ));
        let paragraph = Paragraph::new(line).style(Style::default().bg(Color::Rgb(25, 25, 35)));
        f.render_widget(paragraph, breadcrumb_area);
        return 1;
    }

    // Build horizontal breadcrumb: ◎ Org → ⚙ Config → ■ Class
    let mut spans: Vec<Span> = Vec::with_capacity(path.len() * 3);
    spans.push(Span::raw(" ")); // Left padding

    for (i, level) in path.iter().enumerate() {
        if i > 0 {
            // Arrow separator with subtle color
            spans.push(Span::styled(
                " → ",
                Style::default().fg(Color::Rgb(100, 100, 120)),
            ));
        }
        // Icon
        spans.push(Span::styled(
            format!("{} ", level.icon),
            Style::default().fg(level.color),
        ));
        // Label (bold for last item)
        let label_style = if i == path.len() - 1 {
            Style::default()
                .fg(level.color)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(level.color)
        };
        spans.push(Span::styled(level.label.clone(), label_style));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(Color::Rgb(25, 25, 35)));
    f.render_widget(paragraph, breadcrumb_area);

    1 // Always 1 line
}

// =============================================================================
// MINI-MAP RENDERING (v11.6)
// =============================================================================

/// Information needed to render the mini-map.
struct MiniMapInfo {
    /// Total number of items in the tree.
    total_items: usize,
    /// Current cursor position (0-indexed).
    cursor_pos: usize,
    /// First visible item index.
    scroll_offset: usize,
    /// Number of visible items in viewport.
    visible_count: usize,
    /// Current realm color (for theming).
    realm_color: Color,
}

/// Render mini-map on the right side of tree panel.
/// Returns the width used (2 chars).
fn render_minimap(f: &mut Frame, area: Rect, info: &MiniMapInfo) {
    if area.height == 0 || area.width < 2 || info.total_items == 0 {
        return;
    }

    let height = area.height as usize;
    let mut lines: Vec<Line> = Vec::with_capacity(height);

    // Calculate proportions
    let total = info.total_items;
    let viewport_start = info.scroll_offset;
    let viewport_end = (viewport_start + info.visible_count).min(total);
    let cursor = info.cursor_pos;

    for row in 0..height {
        // Map this row to a position in the full tree
        let tree_start = (row * total) / height;
        let tree_end = ((row + 1) * total) / height;

        // Determine what's visible in this row's range
        let cursor_in_range = cursor >= tree_start && cursor < tree_end.max(tree_start + 1);
        let viewport_overlaps = tree_end > viewport_start && tree_start < viewport_end;

        let (symbol, color) = if cursor_in_range {
            // Cursor position: solid block with realm color
            ("██", info.realm_color)
        } else if viewport_overlaps {
            // Visible viewport: light shade
            ("░░", COLOR_MUTED_TEXT)
        } else {
            // Outside viewport: medium shade
            ("▒▒", Color::Rgb(40, 40, 50))
        };

        lines.push(Line::from(Span::styled(symbol, Style::default().fg(color))));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

/// Build mini-map info from current app state.
fn build_minimap_info(app: &App, visible_height: usize) -> MiniMapInfo {
    // Get current realm color from selection
    let realm_color = match app.current_item() {
        Some(crate::tui::data::TreeItem::Realm(r)) => hex_to_color(&r.color),
        Some(crate::tui::data::TreeItem::Layer(r, _)) => hex_to_color(&r.color),
        Some(crate::tui::data::TreeItem::Class(r, _, _)) => hex_to_color(&r.color),
        Some(crate::tui::data::TreeItem::EntityCategory(r, _, _, _)) => hex_to_color(&r.color),
        Some(crate::tui::data::TreeItem::Instance(r, _, _, _)) => hex_to_color(&r.color),
        _ => Color::Cyan, // Default for arc sections
    };

    MiniMapInfo {
        total_items: app.current_item_count(),
        cursor_pos: app.tree_cursor,
        scroll_offset: app.tree_scroll,
        visible_count: visible_height,
        realm_color,
    }
}

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
        // Render empty tree panel with border
        // v11.6: Show mode in empty state too
        let empty_title = if app.is_graph_mode() {
            " ● Data "
        } else {
            " ◆ Schema "
        };
        let block = Block::default()
            .title(empty_title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));
        f.render_widget(block, area);

        // Overlay empty state
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
    // line_color: color for tree prefix (│├└ characters)
    // text_color: color for icon and text
    // match_positions: optional fuzzy match positions for highlighting
    // bg_color: optional background color for the line (e.g., active Class highlight)
    // trait_icon_opt: optional (trait_icon, trait_color) for colored trait icons
    let make_line = |idx: usize,
                     cursor: usize,
                     focused: bool,
                     tree_prefix: &str,
                     icon: &str,
                     text: String,
                     line_color: Color,
                     text_color: Color,
                     match_positions: Option<&[u32]>,
                     bg_color: Option<Color>,
                     trait_icon_opt: Option<(&str, Color)>|
     -> Line {
        let is_cursor = idx == cursor;
        let cursor_char = if is_cursor { ">" } else { " " };
        let icon_space = if icon.is_empty() { "" } else { " " };

        // Build trait icon string if provided
        let trait_str = trait_icon_opt
            .map(|(ti, _)| format!("{} ", ti))
            .unwrap_or_default();

        if is_cursor && focused {
            // When focused/selected, use white on highlight bg for entire line
            let style = Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White);
            Line::from(Span::styled(
                format!(
                    "{}{}{}{}{}{}",
                    cursor_char, tree_prefix, icon, icon_space, trait_str, text
                ),
                style,
            ))
        } else {
            // Split into spans: tree_prefix colored, text colored differently
            // Apply optional background color to all spans
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
            // Add colored trait icon if provided
            if let Some((ti, tc)) = trait_icon_opt {
                spans.push(Span::styled(format!("{} ", ti), base_style.fg(tc)));
            }
            // Apply fuzzy match highlighting to text if positions provided
            spans.extend(highlight_matches_with_bg(
                &text,
                match_positions,
                text_color,
                bg_color,
            ));
            Line::from(spans)
        }
    };

    // Box-drawing helpers (using extracted pure functions)
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
        Color::Magenta, // line_color (not used - no prefix)
        Color::Magenta, // text_color
        app.search.matches.get(&idx).map(|v| v.as_slice()),
        None, // bg_color
        None, // trait_icon_opt
    ));
    idx += 1;

    let has_arcs = !app.tree.arc_families.is_empty();

    // Trait filter: only show realms/layers/classes matching the filter
    let trait_filter = app.trait_filter.as_deref();

    if !classes_collapsed {
        // Filter visible realms (skip realms with no matching classes when trait filter active)
        let visible_realms: Vec<_> = app
            .tree
            .realms
            .iter()
            .filter(|r| {
                if let Some(filter) = trait_filter {
                    r.layers
                        .iter()
                        .any(|l| l.classes.iter().any(|k| k.trait_name == filter))
                } else {
                    true
                }
            })
            .collect();
        let realm_count = visible_realms.len();

        for (ri, realm) in visible_realms.iter().enumerate() {
            let realm_is_last = ri == realm_count - 1 && !has_arcs;
            let realm_key = format!("realm:{}", realm.key);
            let realm_collapsed = app.tree.is_collapsed(&realm_key);
            let realm_icon = expand_icon(realm_collapsed);

            let realm_color = hex_to_color(&realm.color);

            // v11.6.1: Custom Realm line with counts and right-aligned badge
            // Format: [cursor][prefix][chevron] [icon] [name]  [▦layers ◇classes]  │ [badge] │R│
            let is_cursor = idx == app.tree_cursor;
            let cursor_char = if is_cursor { ">" } else { " " };
            let layers_count = realm.layers.len();
            let classes_count = realm.total_classes();

            // Build left side content
            let left_content = format!(
                "{}{}{}  {}",
                cursor_char,
                branch(realm_is_last),
                realm_icon,
                realm.display_name
            );

            // Build center stats: ▦6 ◇21 + health rollup
            let health_str = if let Some((percent, issues)) = realm.health_rollup() {
                format_health_badge(Some(percent), Some(issues))
            } else {
                String::new()
            };
            let stats_str = format!("▦{} ◇{}{}", layers_count, classes_count, health_str);

            // v0.13.1: No right badge for Realm (bar starts at Layer level)
            // Calculate padding for alignment (using display_width for Unicode support)
            let tree_width = area.width.saturating_sub(4) as usize;
            let left_width = display_width(&left_content);
            let stats_width = display_width(&stats_str);

            // Simple padding: left content + space + stats (no right badge)
            let total_content = left_width + stats_width + 2; // +2 for space around stats
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
                // Apply fuzzy match highlighting to display_name
                spans.extend(highlight_matches_with_bg(
                    &realm.display_name,
                    app.search.matches.get(&idx).map(|v| v.as_slice()),
                    realm_color,
                    None,
                ));
                // Padding + stats (v0.13.1: no right badge - bar starts at Layer)
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

                // Filter visible layers (hide empty if hide_empty, trait filter if active)
                let visible_layers: Vec<_> = realm
                    .layers
                    .iter()
                    .filter(|l| {
                        // Trait filter: skip layers with no matching classes
                        if let Some(filter) = trait_filter {
                            if !l.classes.iter().any(|k| k.trait_name == filter) {
                                return false;
                            }
                        }
                        // Hide empty filter (Data mode only)
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

                    // Calculate total instances in this layer
                    let layer_instance_count: i64 =
                        layer.classes.iter().map(|k| k.instance_count).sum();

                    // Show expand icon only if layer has content
                    let layer_icon = expand_icon(layer_collapsed);

                    // v0.16.4: All layers visible (no dimming for empty layers)
                    let layer_color = hex_to_color(&layer.color);
                    let text_color = layer_color;

                    // v11.6.1: Custom Layer line with counts and right-aligned badge
                    // Format: [cursor][prefix][chevron] [icon] [name]  [◇classes]  │ [badge] │L│
                    let is_cursor = idx == app.tree_cursor;
                    let cursor_char = if is_cursor { ">" } else { " " };
                    let prefix = format!("{}{}", cont(realm_is_last), branch(layer_is_last));
                    let classes_in_layer = layer.classes.len();

                    // Display name with instance count in Data mode
                    let display_name = if is_data_mode {
                        format!("{} ({})", layer.display_name, layer_instance_count)
                    } else {
                        layer.display_name.clone()
                    };

                    // Build left side content
                    let left_content =
                        format!("{}{}{}  {}", cursor_char, prefix, layer_icon, display_name);

                    // Build center stats: ◇4 + health rollup
                    let health_str = if let Some((percent, issues)) = layer.health_rollup() {
                        format_health_badge(Some(percent), Some(issues))
                    } else {
                        String::new()
                    };
                    let stats_str = format!("◇{}{}", classes_in_layer, health_str);

                    // v0.13.1: Simple color bar (layer color) - starts at Layer level
                    // Calculate padding for alignment
                    let tree_width = area.width.saturating_sub(4) as usize;
                    let left_width = display_width(&left_content);
                    let stats_width = display_width(&stats_str);
                    let right_side = "│"; // Simple color bar
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
                        // Apply fuzzy match highlighting to display_name
                        spans.extend(highlight_matches_with_bg(
                            &display_name,
                            app.search.matches.get(&idx).map(|v| v.as_slice()),
                            text_color,
                            None,
                        ));
                        // Padding + stats + color bar
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
                        // v0.13.1: Simple color bar (layer color)
                        spans.push(Span::styled("│", base_style.fg(layer_color)));

                        all_lines.push(Line::from(spans));
                    }
                    idx += 1;

                    if !layer_collapsed {
                        // Filter visible classes (hide empty if hide_empty is true)
                        let visible_classes: Vec<_> = layer
                            .classes
                            .iter()
                            .filter(|k| {
                                // Trait filter: skip classes that don't match
                                if let Some(filter) = trait_filter {
                                    if k.trait_name != filter {
                                        return false;
                                    }
                                }
                                // Hide empty filter (Data mode only)
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

                            // Show collapse icon based on mode:
                            // - Data mode: show chevron if instances exist
                            // - Schema mode: Classes are leaf nodes (no children to expand)
                            let class_icon = if is_data_mode && class_info.instance_count > 0 {
                                // Show expanded (▼) only if instances are actually loaded
                                // Otherwise show collapsed (▶) even if state says "expanded"
                                let instances_loaded =
                                    app.tree.get_instances(&class_info.key).is_some();
                                if instances_loaded {
                                    expand_icon(class_collapsed)
                                } else {
                                    expand_icon(true) // ▶ - not loaded yet
                                }
                            } else {
                                // Meta mode or no instances: leaf node
                                " "
                            };

                            // v10.1: Show instance count (always in Data mode)
                            // v10.6: Add trait icon prefix
                            // v11.3: Colored trait icons (from visual-encoding.yaml + traits/*.yaml)
                            // v11.5: Enhanced display with all useful metrics
                            // v0.16.3: Populated icon (● vs ○) for visual clarity
                            // Format: Name (instances) →out←in req/tot
                            let class_is_empty = class_info.instance_count == 0;
                            let instance_count = class_info.instance_count;

                            // v0.16.3: Use populated icon (● filled = has data, ○ empty = no data)
                            // This replaces trait icon for better "what has data?" visibility
                            let populated_icon = if class_is_empty { "○" } else { "●" };
                            let _t_color = app.theme.trait_color(&class_info.trait_name);

                            // Count arcs by direction
                            let outgoing = class_info
                                .arcs
                                .iter()
                                .filter(|a| a.direction == ArcDirection::Outgoing)
                                .count();
                            let incoming = class_info
                                .arcs
                                .iter()
                                .filter(|a| a.direction == ArcDirection::Incoming)
                                .count();

                            // v11.6.1: Build arc direction suffix with spaces: →2 ←1
                            let arc_suffix = match (outgoing, incoming) {
                                (0, 0) => String::new(),
                                (o, 0) => format!(" →{}", o),
                                (0, i) => format!(" ←{}", i),
                                (o, i) => format!(" →{} ←{}", o, i),
                            };

                            // v11.6.1: Properties count with ⊞ icon: ⊞6/9
                            let props_suffix = if !class_info.properties.is_empty() {
                                format!(
                                    " ⊞{}/{}",
                                    class_info.required_properties.len(),
                                    class_info.properties.len()
                                )
                            } else {
                                String::new()
                            };

                            // Build display text with all metrics
                            // v0.16.4: New format: ● 200 Name →out ←in ⊞req/tot
                            // Count on left, no (def) abbreviation
                            let (display_text, class_text_color, count_str, count_color) = if is_data_mode {
                                // Data mode: instances + arcs + props + health
                                let health_badge = format_health_badge(
                                    class_info.health_percent,
                                    class_info.issues_count,
                                );

                                // v0.16.4: Count string (right-aligned 5 chars): "  200" or "   - "
                                let cnt_str = if class_is_empty {
                                    "  - ".to_string()
                                } else {
                                    format!("{:>4}", instance_count)
                                };

                                let text = format!(
                                    "{}{}{}{}",
                                    class_info.display_name,
                                    arc_suffix,
                                    props_suffix,
                                    health_badge
                                );

                                // v0.16.4: Dim text for empty classes, white for populated
                                let text_color = if class_is_empty {
                                    Color::Rgb(140, 140, 150) // Slightly dimmed
                                } else {
                                    Color::White
                                };

                                // v0.16.4: Count color based on quantity
                                // Green for 1-99, Cyan for 100+, Yellow for 1000+
                                let cnt_color = if instance_count >= 1000 {
                                    Color::Yellow
                                } else if instance_count >= 100 {
                                    Color::Cyan
                                } else if instance_count > 0 {
                                    Color::Green
                                } else {
                                    Color::Rgb(100, 100, 110) // Dim gray for empty
                                };

                                (text, text_color, cnt_str, cnt_color)
                            } else {
                                // Meta mode: arcs + props (no instance count)
                                let text = format!(
                                    "{}{}{}",
                                    class_info.display_name, arc_suffix, props_suffix
                                );
                                (text, Color::White, "    ".to_string(), Color::White)
                            };

                            let prefix = format!(
                                "{}{}{}",
                                cont(realm_is_last),
                                cont(layer_is_last),
                                branch(class_is_last)
                            );
                            // Highlight Class if it has expanded instances (active focus)
                            let class_has_expanded_instances = is_data_mode
                                && !class_collapsed
                                && app
                                    .tree
                                    .get_instances(&class_info.key)
                                    .is_some_and(|i| !i.is_empty());
                            let class_bg = if class_has_expanded_instances {
                                Some(COLOR_ACTIVE_CLASS_BG)
                            } else {
                                None
                            };

                            // v0.16.4: New Class line format
                            // Format: [cursor] [prefix] [icon] [●/○] [count] [name] [arcs] [props] │
                            let is_cursor = idx == app.tree_cursor;
                            let cursor_char = if is_cursor { ">" } else { " " };

                            // Build left side content: ● 200 Name →out ←in ⊞req/tot
                            let left_content = format!(
                                "{}{}{} {} {} {}",
                                cursor_char, prefix, class_icon, populated_icon, count_str, display_text
                            );

                            // v0.13.1: Simple color bar only (no repeated text badges)
                            // Just a colored │ at the right edge, matching layer color

                            // Calculate padding for right-alignment
                            let tree_width = area.width.saturating_sub(4) as usize;
                            let left_width = display_width(&left_content);
                            let right_side = "│"; // Simple color bar
                            let right_width = 1;
                            let padding_width = tree_width.saturating_sub(left_width + right_width);

                            if is_cursor && focused {
                                // Highlighted cursor line - single span with full highlight
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
                                // Build multi-span line with colors
                                let base_style = if let Some(bg) = class_bg {
                                    Style::default().bg(bg)
                                } else {
                                    Style::default()
                                };

                                // v0.16.4: New format: ● 200 Name (no trait abbrev)
                                let icon_color = if class_is_empty {
                                    Color::Rgb(100, 100, 110) // Dim gray
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
                                    // v0.16.4: populated icon (● vs ○)
                                    Span::styled(
                                        format!("{} ", populated_icon),
                                        base_style.fg(icon_color),
                                    ),
                                    // v0.16.4: count with color coding
                                    Span::styled(
                                        format!("{} ", count_str),
                                        base_style.fg(count_color),
                                    ),
                                ];

                                // Apply fuzzy match highlighting to display_text
                                spans.extend(highlight_matches_with_bg(
                                    &display_text,
                                    app.search.matches.get(&idx).map(|v| v.as_slice()),
                                    class_text_color,
                                    class_bg,
                                ));

                                // v0.13.1: Simple color bar (layer color) - no text badges
                                spans.push(Span::styled(" ".repeat(padding_width), base_style));
                                spans.push(Span::styled("│", base_style.fg(layer_color)));

                                all_lines.push(Line::from(spans));
                            }
                            idx += 1;

                            // In Data mode, show instances under Class (if not collapsed)
                            if is_data_mode && !class_collapsed {
                                // v0.16.4: Entity shows flat instances with category suffix + native count
                                // v0.16.5: Fallback to instances["Entity"] if entity_category_instances empty
                                let has_category_instances = !app.tree.entity_category_instances.is_empty();

                                if class_info.key == "Entity"
                                    && !app.tree.entity_categories.is_empty()
                                    && has_category_instances
                                {
                                    // Build reverse lookup: entity_key -> category_key
                                    let mut entity_to_category: rustc_hash::FxHashMap<&str, &str> =
                                        rustc_hash::FxHashMap::default();
                                    for category in &app.tree.entity_categories {
                                        if let Some(instances) =
                                            app.tree.entity_category_instances.get(&category.key)
                                        {
                                            for inst in instances {
                                                entity_to_category.insert(&inst.key, &category.key);
                                            }
                                        }
                                    }

                                    // Collect all Entity instances flat
                                    let all_entities: Vec<_> = app
                                        .tree
                                        .entity_categories
                                        .iter()
                                        .flat_map(|cat| {
                                            app.tree
                                                .entity_category_instances
                                                .get(&cat.key)
                                                .map(|v| v.iter())
                                                .into_iter()
                                                .flatten()
                                        })
                                        .collect();

                                    let inst_count = all_entities.len();
                                    for (ii, instance) in all_entities.iter().enumerate() {
                                        let inst_is_last = ii == inst_count - 1;
                                        let is_cursor = idx == app.tree_cursor;

                                        // Get category for this entity
                                        let category_suffix = entity_to_category
                                            .get(instance.key.as_str())
                                            .map(|c| c.to_lowercase())
                                            .unwrap_or_default();

                                        // Count EntityNative (HAS_NATIVE outgoing arcs)
                                        let native_count = instance
                                            .outgoing_arcs
                                            .iter()
                                            .filter(|a| a.arc_type == "HAS_NATIVE")
                                            .count();
                                        let native_icon = if native_count > 0 { "◆" } else { "◇" };
                                        let native_color = if native_count > 0 {
                                            Color::Green
                                        } else {
                                            Color::Rgb(100, 100, 110)
                                        };

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
                                                    "{}{}○ {}  {}{}  {}",
                                                    cursor_char,
                                                    tree_prefix,
                                                    instance.display_name,
                                                    native_icon,
                                                    native_count,
                                                    category_suffix
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
                                                    format!("○ {}  ", instance.display_name),
                                                    style,
                                                ),
                                                // Native count
                                                Span::styled(
                                                    format!("{}{}", native_icon, native_count),
                                                    Style::default().fg(native_color),
                                                ),
                                                // Category suffix (dim)
                                                Span::styled(
                                                    format!("  {}", category_suffix),
                                                    Style::default().fg(COLOR_MUTED_TEXT),
                                                ),
                                            ];
                                            all_lines.push(Line::from(spans));
                                        }
                                        idx += 1;
                                    }
                                } else if let Some(instances) =
                                    app.tree.get_instances(&class_info.key)
                                {
                                    // Regular classes: show instances directly
                                    let inst_count = instances.len();
                                    for (ii, instance) in instances.iter().enumerate() {
                                        let inst_is_last = ii == inst_count - 1;
                                        let is_cursor = idx == app.tree_cursor;

                                        // Check if primary (for Locale Class)
                                        let is_primary = instance
                                            .properties
                                            .get("is_primary")
                                            .and_then(|v| v.as_bool())
                                            .unwrap_or(false);

                                        // Count incoming FALLBACK_TO
                                        let fallback_count = if is_primary {
                                            instance
                                                .incoming_arcs
                                                .iter()
                                                .filter(|a| a.arc_type == "FALLBACK_TO")
                                                .count()
                                        } else {
                                            0
                                        };

                                        // v0.13.1: Unified instance color (yellow)
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

                                        // Badge for missing required properties: (✗N!)
                                        let missing_badge =
                                            format_missing_badge(instance.missing_required_count);

                                        // Arc count badge: [->N <-M] (only if has arcs, "..." while loading)
                                        let arc_badge = if instance.arcs_loading {
                                            " [...]".to_string()
                                        } else {
                                            format_arc_badge(
                                                instance.outgoing_arcs.len(),
                                                instance.incoming_arcs.len(),
                                            )
                                        };

                                        // Completeness bar: [==--] only shown if incomplete
                                        let completeness_badge = format_completeness_badge(
                                            instance.filled_properties,
                                            instance.total_properties,
                                        );

                                        let tree_prefix = format!(
                                            "{}{}{}{}",
                                            cont(realm_is_last),
                                            cont(layer_is_last),
                                            cont(class_is_last),
                                            branch(inst_is_last)
                                        );

                                        if is_cursor && focused {
                                            // Selected: single span with highlight bg
                                            all_lines.push(Line::from(Span::styled(
                                                format!(
                                                    "{}{}{} {}{}{}{}{}",
                                                    cursor_char,
                                                    tree_prefix,
                                                    icon,
                                                    instance.display_name,
                                                    suffix,
                                                    completeness_badge,
                                                    arc_badge,
                                                    missing_badge
                                                ),
                                                style,
                                            )));
                                        } else {
                                            // Not selected: split into spans for colored badges
                                            let mut spans = vec![
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
                                            // Completeness bar (green gradient based on fill)
                                            if !completeness_badge.is_empty() {
                                                let color = if instance.filled_properties
                                                    == instance.total_properties
                                                {
                                                    Color::Green
                                                } else if instance.filled_properties
                                                    > instance.total_properties / 2
                                                {
                                                    Color::Yellow
                                                } else {
                                                    Color::Red
                                                };
                                                spans.push(Span::styled(
                                                    completeness_badge,
                                                    Style::default().fg(color),
                                                ));
                                            }
                                            // Arc count (cyan)
                                            if !arc_badge.is_empty() {
                                                spans.push(Span::styled(
                                                    arc_badge,
                                                    Style::default().fg(Color::Cyan),
                                                ));
                                            }
                                            // Missing required (red)
                                            if !missing_badge.is_empty() {
                                                spans.push(Span::styled(
                                                    missing_badge,
                                                    Style::default().fg(Color::Red),
                                                ));
                                            }
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
        Color::Yellow, // line_color (not used - no prefix)
        Color::Yellow, // text_color
        app.search.matches.get(&idx).map(|v| v.as_slice()),
        None, // bg_color
        None, // trait_icon_opt
    ));
    idx += 1;

    if !arcs_collapsed {
        let family_count = app.tree.arc_families.len();
        for (fi, family) in app.tree.arc_families.iter().enumerate() {
            let family_is_last = fi == family_count - 1;
            let family_key = format!("family:{}", family.key);
            let family_collapsed = app.tree.is_collapsed(&family_key);
            let family_icon = expand_icon(family_collapsed);

            // v0.13.1: Simplified ArcFamily line - no right badge (like Realm)
            // Format: [cursor][prefix][chevron] [icon] [name]  [◇arcs]
            let is_cursor = idx == app.tree_cursor;
            let cursor_char = if is_cursor { ">" } else { " " };
            let arcs_in_family = family.arc_classes.len();

            // Build left side content
            let left_content = format!(
                "{}{}{}  {}",
                cursor_char,
                branch(family_is_last),
                family_icon,
                family.display_name
            );

            // Build center stats: ◇43
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
                // Apply fuzzy match highlighting to display_name
                spans.extend(highlight_matches_with_bg(
                    &family.display_name,
                    app.search.matches.get(&idx).map(|v| v.as_slice()),
                    COLOR_ARC_FAMILY,
                    None,
                ));
                // v0.13.1: stats only, no right badge
                spans.push(Span::styled(" ", base_style));
                spans.push(Span::styled(stats_str, base_style.fg(COLOR_MUTED_TEXT)));

                all_lines.push(Line::from(spans));
            }
            idx += 1;

            if !family_collapsed {
                let arc_count = family.arc_classes.len();
                for (ai, arc_class) in family.arc_classes.iter().enumerate() {
                    let arc_is_last = ai == arc_count - 1;

                    // v0.13.1: Simplified ArcClass line with color bar
                    // Format: [cursor][prefix] [name]  [From→To]  [card] │
                    let is_cursor = idx == app.tree_cursor;
                    let cursor_char = if is_cursor { ">" } else { " " };
                    let prefix = format!("{}{}", cont(family_is_last), branch(arc_is_last));

                    // Build left side content: arc name
                    let left_content =
                        format!("{}{}  {}", cursor_char, prefix, arc_class.display_name);

                    // Build center: From→To (abbreviated class names)
                    let from_abbrev = arc_class.from_class.chars().take(8).collect::<String>();
                    let to_abbrev = arc_class.to_class.chars().take(8).collect::<String>();
                    let flow_str = format!("{}→{}", from_abbrev, to_abbrev);

                    // Cardinality (useful info, keep it)
                    let card_str = cardinality_abbrev(&arc_class.cardinality);

                    // Calculate padding for alignment (using display_width for Unicode support)
                    let tree_width = area.width.saturating_sub(4) as usize;
                    let left_width = display_width(&left_content);
                    let flow_width = display_width(&flow_str);
                    let card_width = display_width(card_str);
                    let right_side = "│"; // v0.13.1: Simple color bar
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
                        // Apply fuzzy match highlighting to display_name
                        spans.extend(highlight_matches_with_bg(
                            &arc_class.display_name,
                            app.search.matches.get(&idx).map(|v| v.as_slice()),
                            COLOR_DESC_TEXT,
                            None,
                        ));
                        // Flow + cardinality
                        spans.push(Span::styled(" ", base_style));
                        spans.push(Span::styled(flow_str, base_style.fg(COLOR_MUTED_TEXT)));
                        spans.push(Span::styled(" ", base_style));
                        spans.push(Span::styled(card_str, base_style.fg(Color::Cyan)));
                        // v0.13.1: Simple color bar (arc family color)
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

    // Show hierarchical position in title: R:1/2 L:2/4 K:3/7 I:42/300
    // v11.6: Show mode with icon + hierarchy position
    let total = app.current_item_count(); // Used for scrollbar
    let mode_prefix = if app.is_graph_mode() {
        "● Data" // Filled circle = instances/data
    } else {
        "◆ Schema" // Diamond = structure/schema
    };

    // v11.8: Renamed per ADR-024 Data Origin semantics
    let filter_indicator = match app.trait_filter.as_deref() {
        Some("defined") => " │ ■ defined",
        Some("authored") => " │ □ authored",
        Some("imported") => " │ ◊ imported",
        Some("generated") => " │ ★ generated",
        Some("retrieved") => " │ ▪ retrieved",
        _ => "",
    };

    let hierarchy = app
        .tree
        .hierarchy_position(app.tree_cursor, app.is_graph_mode());
    let hierarchy_str = hierarchy.to_compact_string();
    let title = if hierarchy_str.is_empty() {
        format!(" {}{} ", mode_prefix, filter_indicator)
    } else {
        format!(" {} │ {}{} ", mode_prefix, hierarchy_str, filter_indicator)
    };

    // Render block with title
    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // v11.6: Reserve space for mini-map (2 chars + 1 separator) and add left padding
    let minimap_width: u16 = 3;
    let content_x = inner_area.x + TREE_PADDING_LEFT;
    let content_width = inner_area
        .width
        .saturating_sub(minimap_width + TREE_PADDING_LEFT);

    // v11.6: Render sticky breadcrumb at top of content area (with padding)
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

    // v11.6: Render mini-map on right side (positioned from right edge)
    let sep_x = inner_area.x + inner_area.width - minimap_width;
    let minimap_area = Rect::new(
        sep_x + 1, // After separator
        inner_area.y,
        2, // Mini-map is 2 chars wide
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
                Style::default().fg(Color::Rgb(50, 50, 60)),
            )));
        }
        let sep_paragraph = Paragraph::new(sep_lines);
        f.render_widget(sep_paragraph, sep_area);
    }

    // Add scrollbar if content exceeds visible area (adjust for breadcrumb)
    // Position scrollbar at the left edge of the mini-map separator
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

        // Place scrollbar between tree content and mini-map separator
        let scrollbar_area = Rect {
            x: inner_area.x + content_width.saturating_sub(1),
            y: tree_y,
            width: 1,
            height: tree_height,
        };
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

/// Render filtered Data mode: only instances of a specific Class with breadcrumb.
fn render_filtered_instances(
    f: &mut Frame,
    area: Rect,
    app: &App,
    class_key: &str,
    visible_height: usize,
    focused: bool,
    border_color: Color,
) {
    // Get Class info for display with full hierarchy
    let class_info = app.tree.find_class(class_key);
    let (realm_display, realm_color, layer_display, layer_color, class_display) = class_info
        .map(|(realm, layer, class)| {
            (
                realm.display_name.clone(),
                hex_to_color(&realm.color),
                layer.display_name.clone(),
                hex_to_color(&layer.color),
                class.display_name.clone(),
            )
        })
        .unwrap_or_else(|| {
            (
                "Unknown".to_string(),
                Color::White,
                "Unknown".to_string(),
                Color::White,
                class_key.to_string(),
            )
        });

    // Build lines: breadcrumb + instances
    let mut all_lines: Vec<Line> = Vec::new();

    // Breadcrumb header with full hierarchy: Realm → Layer → Class
    all_lines.push(Line::from(vec![
        Span::styled("← ", STYLE_DIM),
        Span::styled("Esc", STYLE_HIGHLIGHT),
        Span::styled(" │ ", STYLE_DIM),
        Span::styled(&realm_display, Style::default().fg(realm_color)),
        Span::styled(" → ", STYLE_DIM),
        Span::styled(&layer_display, Style::default().fg(layer_color)),
        Span::styled(" → ", STYLE_DIM),
        Span::styled(&class_display, STYLE_PRIMARY),
    ]));
    all_lines.push(Line::from(Span::styled(
        "─".repeat(area.width.saturating_sub(2) as usize),
        STYLE_UNFOCUSED,
    )));

    // Get instances and total count
    let instances = app.tree.get_instances(class_key);
    let instance_count = instances.map(|i| i.len()).unwrap_or(0);
    let total_count = app
        .tree
        .get_instance_total(class_key)
        .unwrap_or(instance_count);
    let is_truncated = total_count > instance_count;
    let is_loading = app
        .pending
        .instance
        .as_ref()
        .is_some_and(|k| k == class_key);

    if instance_count == 0 {
        if is_loading {
            // Still loading from Neo4j (animated spinner)
            all_lines.push(Line::from(Span::styled(
                format!("  {} Loading instances...", spinner(app.tick)),
                STYLE_HIGHLIGHT,
            )));
        } else {
            // Loaded but empty
            all_lines.push(Line::from(Span::styled(
                "  No instances exist for this Class",
                STYLE_DIM,
            )));
        }
    } else if let Some(instances) = instances {
        for (idx, instance) in instances.iter().enumerate() {
            let is_cursor = idx == app.tree_cursor;

            // Check if this is a primary locale (is_primary: true)
            let is_primary = instance
                .properties
                .get("is_primary")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            // Count incoming FALLBACK_TO arcs for primary locales
            let fallback_count = if is_primary {
                instance
                    .incoming_arcs
                    .iter()
                    .filter(|arc| arc.arc_type == "FALLBACK_TO")
                    .count()
            } else {
                0
            };

            // v0.13.1: Unified instance color (yellow)
            // Primary locales: filled circle ●, secondary: empty circle ○
            let icon = if is_primary { "●" } else { "○" };
            let base_color = COLOR_INSTANCE;

            let style = if is_cursor && focused {
                Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
            } else {
                Style::default().fg(base_color)
            };

            let prefix = if is_cursor { "› " } else { "  " };

            // Format: "● Arabic (Saudi Arabia) [13↓]" for primary
            // Format: "○ Arabic (Algeria)" for secondary
            let display = if is_primary && fallback_count > 0 {
                format!(
                    "{}{} {} [{}↓]",
                    prefix, icon, instance.display_name, fallback_count
                )
            } else {
                format!("{}{} {}", prefix, icon, instance.display_name)
            };

            all_lines.push(Line::from(Span::styled(display, style)));
        }
    }

    // Apply scroll
    let lines: Vec<Line> = all_lines
        .into_iter()
        .skip(app.tree_scroll)
        .take(visible_height)
        .collect();

    // Title with Class name and count + position indicator
    // Format: "Locale (3/203)" when all loaded, "Locale (3/500 of 847)" when truncated
    let title = if instance_count > 0 {
        if is_truncated {
            format!(
                " {} ({}/{} of {}) ",
                class_display,
                app.tree_cursor + 1,
                instance_count,
                total_count
            )
        } else {
            format!(
                " {} ({}/{}) ",
                class_display,
                app.tree_cursor + 1,
                instance_count
            )
        }
    } else {
        format!(" {} (0) ", class_display)
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(layer_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

// =============================================================================
// PURE HELPER FUNCTIONS (extracted for testability)
// =============================================================================

/// Get the branch character for tree drawing.
/// - `└─` for last item (no more siblings)
/// - `├─` for non-last item (more siblings below)
#[inline]
pub(super) fn branch_char(is_last: bool) -> &'static str {
    if is_last { "└─" } else { "├─" }
}

/// Get the continuation character for tree drawing.
/// - `  ` (two spaces) if parent was last (no vertical line needed)
/// - `│ ` if parent was not last (vertical line continues)
#[inline]
pub(super) fn cont_char(parent_is_last: bool) -> &'static str {
    if parent_is_last { "  " } else { "│ " }
}

/// Get the expand/collapse icon for a tree node.
/// - `▶` when collapsed (pointing right, can expand)
/// - `▼` when expanded (pointing down, can collapse)
#[inline]
pub(super) fn expand_icon(is_collapsed: bool) -> &'static str {
    if is_collapsed { "▶" } else { "▼" }
}

/// Build a tree prefix string for a given depth and position.
///
/// # Arguments
/// * `parent_is_last` - Slice of booleans indicating whether each ancestor was the last child
/// * `is_last` - Whether this node is the last child at its level
///
/// # Returns
/// A string like "│ │ └─" for drawing tree structure
///
/// NOTE: Currently used only in tests to verify tree prefix logic.
/// Future refactoring could use this in render_tree to replace inline format!() calls.
#[allow(dead_code)]
pub(super) fn build_tree_prefix(parent_is_last: &[bool], is_last: bool) -> String {
    let mut prefix = String::with_capacity(parent_is_last.len() * 3 + 3);
    for &was_last in parent_is_last {
        prefix.push_str(cont_char(was_last));
    }
    prefix.push_str(branch_char(is_last));
    prefix
}

/// Format a health badge for a Class node.
/// Returns empty string if no health data, or a bar like " ━━━░░░░░░░50%"
pub(super) fn format_health_badge(
    health_percent: Option<u8>,
    issues_count: Option<usize>,
) -> String {
    let Some(percent) = health_percent else {
        return String::new();
    };
    let filled = percent / 10;
    let empty = 10 - filled;
    let issues = issues_count.unwrap_or(0);
    if issues > 0 {
        format!(
            " {}{}{}% ⚠{}",
            "━".repeat(filled as usize),
            "░".repeat(empty as usize),
            percent,
            issues
        )
    } else {
        format!(
            " {}{}{}%",
            "━".repeat(filled as usize),
            "░".repeat(empty as usize),
            percent
        )
    }
}

/// Format a completeness badge for an instance.
/// Returns empty string if 100% complete, or "[==--]" style bar if incomplete.
pub(super) fn format_completeness_badge(filled: usize, total: usize) -> String {
    if total == 0 {
        return String::new();
    }
    if filled >= total {
        // 100% complete - hide badge
        return String::new();
    }
    let ratio = filled as f32 / total as f32;
    // v0.16.5: Clamp to 0..=4 to prevent overflow from floating point rounding
    let filled_chars = (ratio * 4.0).round().min(4.0) as usize;
    let empty_chars = 4 - filled_chars;
    format!(" [{}{}]", "=".repeat(filled_chars), "-".repeat(empty_chars))
}

/// Format an arc count badge for an instance.
/// Returns empty string if no arcs, or "[->N|<-M]" if has arcs.
pub(super) fn format_arc_badge(outgoing: usize, incoming: usize) -> String {
    if outgoing == 0 && incoming == 0 {
        String::new()
    } else {
        format!(" [->{}|<-{}]", outgoing, incoming)
    }
}

/// Format a missing required properties badge.
/// Returns empty string if none missing, or " (✗N!)" if missing.
pub(super) fn format_missing_badge(missing_count: usize) -> String {
    if missing_count > 0 {
        format!(" (✗{}!)", missing_count)
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =============================================================================
    // Tree structure helpers tests
    // =============================================================================

    #[test]
    fn test_branch_char_last() {
        assert_eq!(branch_char(true), "└─");
    }

    #[test]
    fn test_branch_char_not_last() {
        assert_eq!(branch_char(false), "├─");
    }

    #[test]
    fn test_cont_char_parent_was_last() {
        assert_eq!(cont_char(true), "  ");
    }

    #[test]
    fn test_cont_char_parent_was_not_last() {
        assert_eq!(cont_char(false), "│ ");
    }

    #[test]
    fn test_expand_icon_collapsed() {
        assert_eq!(expand_icon(true), "▶");
    }

    #[test]
    fn test_expand_icon_expanded() {
        assert_eq!(expand_icon(false), "▼");
    }

    // =============================================================================
    // Tree prefix building tests
    // =============================================================================

    #[test]
    fn test_build_tree_prefix_root_level() {
        // First level item (no parents), last child
        assert_eq!(build_tree_prefix(&[], true), "└─");
        // First level item, not last
        assert_eq!(build_tree_prefix(&[], false), "├─");
    }

    #[test]
    fn test_build_tree_prefix_second_level() {
        // Second level, parent was not last, this is last
        assert_eq!(build_tree_prefix(&[false], true), "│ └─");
        // Second level, parent was not last, this is not last
        assert_eq!(build_tree_prefix(&[false], false), "│ ├─");
        // Second level, parent was last, this is last
        assert_eq!(build_tree_prefix(&[true], true), "  └─");
        // Second level, parent was last, this is not last
        assert_eq!(build_tree_prefix(&[true], false), "  ├─");
    }

    #[test]
    fn test_build_tree_prefix_third_level() {
        // Third level: grandparent not last, parent not last, this is last
        assert_eq!(build_tree_prefix(&[false, false], true), "│ │ └─");
        // Third level: grandparent last, parent not last, this is not last
        assert_eq!(build_tree_prefix(&[true, false], false), "  │ ├─");
        // Third level: all were last
        assert_eq!(build_tree_prefix(&[true, true], true), "    └─");
    }

    #[test]
    fn test_build_tree_prefix_deep_nesting() {
        // Deep nesting with mixed last states
        let prefix = build_tree_prefix(&[false, true, false, true], false);
        assert_eq!(prefix, "│   │   ├─");
    }

    // =============================================================================
    // Health badge tests
    // =============================================================================

    #[test]
    fn test_format_health_badge_none() {
        assert_eq!(format_health_badge(None, None), "");
    }

    #[test]
    fn test_format_health_badge_zero_percent() {
        let badge = format_health_badge(Some(0), None);
        assert!(badge.contains("0%"));
        assert!(badge.contains("░░░░░░░░░░")); // 10 empty chars
    }

    #[test]
    fn test_format_health_badge_fifty_percent() {
        let badge = format_health_badge(Some(50), None);
        assert!(badge.contains("50%"));
        assert!(badge.contains("━━━━━")); // 5 filled chars
        assert!(badge.contains("░░░░░")); // 5 empty chars
    }

    #[test]
    fn test_format_health_badge_hundred_percent() {
        let badge = format_health_badge(Some(100), None);
        assert!(badge.contains("100%"));
        assert!(badge.contains("━━━━━━━━━━")); // 10 filled chars
    }

    #[test]
    fn test_format_health_badge_with_issues() {
        let badge = format_health_badge(Some(70), Some(3));
        assert!(badge.contains("70%"));
        assert!(badge.contains("⚠3"));
    }

    #[test]
    fn test_format_health_badge_with_zero_issues() {
        let badge = format_health_badge(Some(80), Some(0));
        assert!(badge.contains("80%"));
        assert!(!badge.contains("⚠"));
    }

    // =============================================================================
    // Completeness badge tests
    // =============================================================================

    #[test]
    fn test_format_completeness_badge_empty() {
        assert_eq!(format_completeness_badge(0, 0), "");
    }

    #[test]
    fn test_format_completeness_badge_complete() {
        // 100% complete - should hide badge
        assert_eq!(format_completeness_badge(10, 10), "");
        assert_eq!(format_completeness_badge(5, 5), "");
    }

    #[test]
    fn test_format_completeness_badge_zero_filled() {
        let badge = format_completeness_badge(0, 8);
        assert_eq!(badge, " [----]"); // 0/8 = 0%, rounds to 0 filled
    }

    #[test]
    fn test_format_completeness_badge_half_filled() {
        let badge = format_completeness_badge(4, 8);
        assert_eq!(badge, " [==--]"); // 50% = 2 filled chars
    }

    #[test]
    fn test_format_completeness_badge_three_quarters() {
        let badge = format_completeness_badge(6, 8);
        // 6/8 = 75% -> 0.75 * 4 = 3 filled chars
        assert_eq!(badge, " [===-]");
    }

    #[test]
    fn test_format_completeness_badge_almost_complete() {
        // 7/8 = 87.5% rounds to 4 filled chars (but badge shown since not 100%)
        let badge = format_completeness_badge(7, 8);
        assert_eq!(badge, " [====]");

        // 9/10 = 90% also rounds to 4 filled chars
        let badge2 = format_completeness_badge(9, 10);
        assert_eq!(badge2, " [====]");
    }

    #[test]
    fn test_format_completeness_badge_low_fill() {
        // 1/8 = 12.5% -> 0.5 rounds to 1
        let badge = format_completeness_badge(1, 8);
        assert_eq!(badge, " [=---]");

        // 2/8 = 25% -> 1 filled char
        let badge2 = format_completeness_badge(2, 8);
        assert_eq!(badge2, " [=---]");
    }

    // =============================================================================
    // Arc badge tests
    // =============================================================================

    #[test]
    fn test_format_arc_badge_no_arcs() {
        assert_eq!(format_arc_badge(0, 0), "");
    }

    #[test]
    fn test_format_arc_badge_outgoing_only() {
        assert_eq!(format_arc_badge(5, 0), " [->5|<-0]");
    }

    #[test]
    fn test_format_arc_badge_incoming_only() {
        assert_eq!(format_arc_badge(0, 3), " [->0|<-3]");
    }

    #[test]
    fn test_format_arc_badge_both() {
        assert_eq!(format_arc_badge(2, 7), " [->2|<-7]");
    }

    // =============================================================================
    // Missing badge tests
    // =============================================================================

    #[test]
    fn test_format_missing_badge_none() {
        assert_eq!(format_missing_badge(0), "");
    }

    #[test]
    fn test_format_missing_badge_one() {
        assert_eq!(format_missing_badge(1), " (✗1!)");
    }

    #[test]
    fn test_format_missing_badge_many() {
        assert_eq!(format_missing_badge(5), " (✗5!)");
    }

    // =============================================================================
    // Highlight matches tests (fuzzy search highlighting)
    // =============================================================================

    #[test]
    fn test_highlight_matches_no_positions() {
        let spans = highlight_matches_with_bg("hello", None, Color::White, None);
        assert_eq!(spans.len(), 1);
        // Check that the text is correct
        let span = &spans[0];
        assert_eq!(span.content, "hello");
    }

    #[test]
    fn test_highlight_matches_empty_positions() {
        let spans = highlight_matches_with_bg("hello", Some(&[]), Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "hello");
    }

    #[test]
    fn test_highlight_matches_single_char() {
        // Match positions: 0 (first char 'h')
        let spans = highlight_matches_with_bg("hello", Some(&[0]), Color::White, None);
        // Should be: [highlighted 'h'], [normal 'ello']
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].content, "h");
        assert_eq!(spans[1].content, "ello");
    }

    #[test]
    fn test_highlight_matches_consecutive() {
        // Match positions: 0, 1, 2 ('hel')
        let spans = highlight_matches_with_bg("hello", Some(&[0, 1, 2]), Color::White, None);
        // Should be: [highlighted 'hel'], [normal 'lo']
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].content, "hel");
        assert_eq!(spans[1].content, "lo");
    }

    #[test]
    fn test_highlight_matches_scattered() {
        // Match positions: 0, 2, 4 ('h', 'l', 'o')
        let spans = highlight_matches_with_bg("hello", Some(&[0, 2, 4]), Color::White, None);
        // Should produce alternating highlighted/normal spans
        // [h], [e], [l], [l], [o] with h, l, o highlighted
        // Merged: [h(hl)], [e(norm)], [l(hl)], [l(norm)], [o(hl)]
        assert!(spans.len() >= 3);
    }

    #[test]
    fn test_highlight_matches_full_match() {
        // All chars match
        let spans = highlight_matches_with_bg("hi", Some(&[0, 1]), Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "hi");
    }

    // Task 5.1: Additional explicit tests for highlight_matches
    #[test]
    fn test_highlight_matches_no_match() {
        // When matches is None, return single span with base style
        let spans = highlight_matches_with_bg("hello", None, Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "hello");
        // Verify base color is applied
        assert_eq!(spans[0].style.fg, Some(Color::White));
        assert_eq!(spans[0].style.bg, None);
    }

    #[test]
    fn test_highlight_matches_with_positions() {
        // Match positions 0 and 2: 'h' and 'l' in "hello"
        let spans = highlight_matches_with_bg("hello", Some(&[0, 2]), Color::White, None);
        // Expected: [h(yellow)], [e(white)], [l(yellow)], [lo(white)]
        assert!(
            spans.len() >= 3,
            "Expected at least 3 spans, got {}",
            spans.len()
        );

        // First span should be 'h' highlighted (yellow bg)
        assert_eq!(spans[0].content, "h");
        assert_eq!(spans[0].style.bg, Some(Color::Yellow));
        assert_eq!(spans[0].style.fg, Some(Color::Black));

        // Second span should be 'e' with base color
        assert_eq!(spans[1].content, "e");
        assert_eq!(spans[1].style.fg, Some(Color::White));

        // Third span should be 'l' highlighted
        assert_eq!(spans[2].content, "l");
        assert_eq!(spans[2].style.bg, Some(Color::Yellow));
        assert_eq!(spans[2].style.fg, Some(Color::Black));

        // Fourth span should be 'lo' with base color
        assert_eq!(spans[3].content, "lo");
        assert_eq!(spans[3].style.fg, Some(Color::White));
    }
}
