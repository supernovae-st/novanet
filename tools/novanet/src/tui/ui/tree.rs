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
    render_empty_state, spinner,
    // v0.17.3 (ADR-036): trait_icon removed - traits no longer in schema
};
use crate::tui::app::{App, Focus};
use crate::tui::data::locale_to_flag;
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
const SCROLLBAR_WIDTH: u16 = 1;

// =============================================================================
// POWER BAR RENDERING (Entity relationship visualization)
// =============================================================================

/// Power bar width (10 filled/empty characters)
const POWER_BAR_WIDTH: usize = 10;

/// Power bar color thresholds (Tailwind colors)
const COLOR_POWER_HIGH: Color = Color::Rgb(34, 197, 94);   // green-500 (≥80%)
const COLOR_POWER_MED: Color = Color::Rgb(249, 115, 22);   // orange-500 (50-79%)
const COLOR_POWER_LOW: Color = Color::Rgb(239, 68, 68);    // red-500 (<50%)

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
/// v0.17.3: Zero-allocation using lookup table instead of format!
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
const COLOR_ENTITY_SLUG: Color = Color::Rgb(148, 163, 184);

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
                icon: layer_badge_icon(&l.key), // v0.17.3: use layer icon (trait removed)
                label: class_label,
                color: hex_to_color(&l.color), // v0.17.3: use layer color (trait removed)
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
                icon: layer_badge_icon(&l.key), // v0.17.3: use layer icon (trait removed)
                label: k.display_name.clone(),
                color: hex_to_color(&l.color), // v0.17.3: use layer color (trait removed)
            });
            path.push(BreadcrumbLevel {
                icon: "◫",
                label: cat.display_name.clone(),
                color: Color::Gray,
            });
        }
        Some(TreeItem::LocaleGroup(r, l, k, group)) => {
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
                icon: layer_badge_icon(&l.key), // v0.17.3: use layer icon (trait removed)
                label: k.display_name.clone(),
                color: hex_to_color(&l.color), // v0.17.3: use layer color (trait removed)
            });
            path.push(BreadcrumbLevel {
                icon: "🌐",
                label: format!("{} {} ({})", group.flag, group.locale_code, group.locale_name),
                color: Color::Cyan,
            });
        }
        // v0.17.3: EntityGroup breadcrumb (entity-grouped EntityNatives)
        Some(TreeItem::EntityGroup(r, l, k, group)) => {
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
                icon: layer_badge_icon(&l.key), // v0.17.3: use layer icon (trait removed)
                label: k.display_name.clone(),
                color: hex_to_color(&l.color), // v0.17.3: use layer color (trait removed)
            });
            path.push(BreadcrumbLevel {
                icon: "◈",
                label: group.entity_display_name.clone(),
                color: Color::Yellow,
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
                icon: layer_badge_icon(&l.key), // v0.17.3: use layer icon (trait removed)
                label: k.display_name.clone(),
                color: hex_to_color(&l.color), // v0.17.3: use layer color (trait removed)
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
        Some(TreeItem::EntityNativeItem(r, l, k, native)) => {
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
                icon: layer_badge_icon(&l.key), // v0.17.3: use layer icon (trait removed)
                label: k.display_name.clone(),
                color: hex_to_color(&l.color), // v0.17.3: use layer color (trait removed)
            });
            path.push(BreadcrumbLevel {
                icon: "◆",
                label: native.display_name.clone(),
                color: COLOR_INSTANCE,
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
        Some(crate::tui::data::TreeItem::LocaleGroup(r, _, _, _)) => hex_to_color(&r.color),
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

    // v0.17.3 (ADR-036): trait filter removed - show all realms
    if !classes_collapsed {
        let visible_realms: Vec<_> = app.tree.realms.iter().collect();
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
            let tree_width = area.width.saturating_sub(5) as usize;
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

                // v0.17.3 (ADR-036): trait filter removed
                // Filter visible layers (hide empty if hide_empty)
                let visible_layers: Vec<_> = realm
                    .layers
                    .iter()
                    .filter(|l| {
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
                    let tree_width = area.width.saturating_sub(5) as usize;
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
                        // v0.17.3 (ADR-036): trait filter removed
                        // Filter visible classes (hide empty if hide_empty is true)
                        let visible_classes: Vec<_> = layer
                            .classes
                            .iter()
                            .filter(|k| {
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
                                // v0.17.3: Use helpers for Entity/EntityNative dual storage
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
                            let populated_icon = if class_is_empty { "○" } else { "●" };

                            // v0.17.3: Simplified format - just Name (count) - removed arc/prop noise
                            let (display_text, class_text_color, count_str, count_color) =
                                if is_data_mode {
                                    // Data mode: Name (count)
                                    let cnt_str = if class_is_empty {
                                        String::new()
                                    } else {
                                        format!(" ({})", instance_count)
                                    };

                                    let text = class_info.display_name.clone();

                                    // Dim text for empty classes, white for populated
                                    let text_color = if class_is_empty {
                                        Color::Rgb(140, 140, 150)
                                    } else {
                                        Color::White
                                    };

                                    // Count color based on quantity
                                    let cnt_color = if instance_count >= 1000 {
                                        Color::Yellow
                                    } else if instance_count >= 100 {
                                        Color::Cyan
                                    } else if instance_count > 0 {
                                        Color::Green
                                    } else {
                                        Color::Rgb(100, 100, 110)
                                    };

                                    (text, text_color, cnt_str, cnt_color)
                                } else {
                                    // Meta mode: just name
                                    (class_info.display_name.clone(), Color::White, String::new(), Color::White)
                                };

                            let prefix = format!(
                                "{}{}{}",
                                cont(realm_is_last),
                                cont(layer_is_last),
                                branch(class_is_last)
                            );
                            // Highlight Class if it has expanded instances (active focus)
                            // v0.17.3: Use helpers for Entity/EntityNative dual storage
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

                            // v0.16.4: New Class line format
                            // Format: [cursor] [prefix] [icon] [●/○] [count] [name] [arcs] [props] │
                            let is_cursor = idx == app.tree_cursor;
                            let cursor_char = if is_cursor { ">" } else { " " };

                            // Build left side content: ● 200 Name →out ←in ⊞req/tot
                            let left_content = format!(
                                "{}{}{} {} {} {}",
                                cursor_char,
                                prefix,
                                class_icon,
                                populated_icon,
                                count_str,
                                display_text
                            );

                            // v0.13.1: Simple color bar only (no repeated text badges)
                            // Just a colored │ at the right edge, matching layer color

                            // Calculate padding for right-alignment
                            let tree_width = area.width.saturating_sub(5) as usize;
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
                                // v0.17.3: Entity shows flat alphabetical list (no category grouping)
                                if class_info.key == "Entity" {
                                    // v0.17.3: Entity shows simple flat list (same format as regular instances)
                                    // EntityNatives are shown under the EntityNative class instead
                                    let all_entities: Vec<_> =
                                        app.tree.entity_instances_flat().collect();
                                    let inst_count = all_entities.len();

                                    for (ii, instance) in all_entities.iter().enumerate() {
                                        let inst_is_last = ii == inst_count - 1;
                                        let is_cursor = idx == app.tree_cursor;

                                        // Pillar entities get ★ icon, others get ○
                                        let is_pillar = instance.properties.get("is_pillar")
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
                                } else if class_info.key == "EntityNative" && !app.tree.entity_native_groups.is_empty() {
                                    // EntityNative class: group by parent Entity
                                    // v0.17.3: Show entity groups with power bar and expandable natives
                                    use unicode_width::UnicodeWidthStr;

                                    let group_count = app.tree.entity_native_groups.len();
                                    for (gi, entity_group) in app.tree.entity_native_groups.iter().enumerate() {
                                        let group_is_last = gi == group_count - 1;
                                        let is_cursor = idx == app.tree_cursor;

                                        // Expand/collapse state for this entity group
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

                                        // Format: ▼ qr-code (5)      ▰▰▰▰▰▰▱▱ (power bar right-aligned)
                                        let (power_bar, power_color) = render_power_bar(entity_group.relationship_power);

                                        // Calculate widths for right-alignment
                                        let tree_width = area.width.saturating_sub(5) as usize;
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
                                                Span::styled(
                                                    tree_prefix,
                                                    Style::default().fg(layer_color),
                                                ),
                                                Span::styled(
                                                    format!("{} {} ({})", expand_icon, entity_group.entity_key, entity_group.native_count),
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
                                            if let Some(natives) = app.tree.entity_native_by_entity.get(&entity_group.entity_key) {
                                                // v0.17.3: Format with locale flags: "🇫🇷 fr-FR - Display Name    /slug"
                                                // Pre-compute left parts and find max width for slug alignment
                                                let native_parts: Vec<_> = natives.iter().map(|native| {
                                                    // Use locale_code for flag
                                                    let flag = locale_to_flag(&native.locale_code);
                                                    let left = format!(
                                                        "{} {} - {}",
                                                        flag,
                                                        native.locale_code,
                                                        native.display_name
                                                    );
                                                    (left, native.slug.as_deref())
                                                }).collect();

                                                let native_count_inner = native_parts.len();
                                                for (ni, (left_part, slug_opt)) in native_parts.iter().enumerate() {
                                                    let native_is_last = ni == native_count_inner - 1;
                                                    let is_native_cursor = idx == app.tree_cursor;

                                                    let native_style = if is_native_cursor && focused {
                                                        Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
                                                    } else {
                                                        Style::default().fg(COLOR_ENTITY_TEXT)
                                                    };

                                                    let slug_style = if is_native_cursor && focused {
                                                        Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
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

                                                    // Calculate slug display with right-alignment
                                                    let slug_display = match slug_opt {
                                                        Some(slug) => format!("/{}", slug),
                                                        None => String::new(),
                                                    };

                                                    // Calculate padding for right-alignment of slug
                                                    let prefix_len = 1 + UnicodeWidthStr::width(native_tree_prefix.as_str()); // cursor + prefix
                                                    let left_width = UnicodeWidthStr::width(left_part.as_str());
                                                    let slug_width = UnicodeWidthStr::width(slug_display.as_str());
                                                    let total_content = prefix_len + left_width + slug_width;
                                                    let padding_width = tree_width.saturating_sub(total_content + 1);

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
                                            // Not selected: split into spans for colored prefix
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
                    let tree_width = area.width.saturating_sub(5) as usize;
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

    // v0.17.3 (ADR-036): trait filter indicator removed

    let hierarchy = app
        .tree
        .hierarchy_position(app.tree_cursor, app.is_graph_mode(), app.hide_empty);
    let hierarchy_str = hierarchy.to_compact_string();
    let title = if hierarchy_str.is_empty() {
        format!(" {} ", mode_prefix)
    } else {
        format!(" {} │ {} ", mode_prefix, hierarchy_str)
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
        .saturating_sub(minimap_width + TREE_PADDING_LEFT + SCROLLBAR_WIDTH);

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

        // Place scrollbar in reserved space between tree content and mini-map separator
        let scrollbar_area = Rect {
            x: content_x + content_width,
            y: tree_y,
            width: SCROLLBAR_WIDTH,
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
    // Locale to flag tests
    // =============================================================================

    #[test]
    fn test_locale_to_flag_france() {
        assert_eq!(locale_to_flag("fr-FR"), "🇫🇷");
    }

    #[test]
    fn test_locale_to_flag_mexico() {
        assert_eq!(locale_to_flag("es-MX"), "🇲🇽");
    }

    #[test]
    fn test_locale_to_flag_usa() {
        assert_eq!(locale_to_flag("en-US"), "🇺🇸");
    }

    #[test]
    fn test_locale_to_flag_germany() {
        assert_eq!(locale_to_flag("de-DE"), "🇩🇪");
    }

    #[test]
    fn test_locale_to_flag_japan() {
        assert_eq!(locale_to_flag("ja-JP"), "🇯🇵");
    }

    #[test]
    fn test_locale_to_flag_fallback_invalid() {
        // Invalid format should return white flag
        assert_eq!(locale_to_flag("invalid"), "🏳️");
    }

    #[test]
    fn test_locale_to_flag_single_part() {
        // Just language code, no country - uses the language as country
        // "fr" → treats as country code → "FR" → 🇫🇷
        assert_eq!(locale_to_flag("FR"), "🇫🇷");
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
