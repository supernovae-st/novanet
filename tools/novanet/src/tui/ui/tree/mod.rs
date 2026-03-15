//! Tree panel rendering for TUI v2.
//!
//! Renders the taxonomy hierarchy with:
//! - Box-drawing characters for visual structure
//! - Collapse/expand state management
//! - Fuzzy search match highlighting
//! - Data mode instance display
//! - Filtered instances view

mod arcs_section;
mod breadcrumb;
mod filtered;
mod helpers;
mod highlight;
mod minimap;
mod nodes;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};

use self::arcs_section::build_arc_lines;
use self::breadcrumb::render_breadcrumb;
use self::filtered::render_filtered_instances;
use self::minimap::{build_minimap_info, render_minimap};
use self::nodes::build_node_class_lines;
use super::{COLOR_UNFOCUSED_BORDER, EmptyStateClass, STYLE_DIM, render_empty_state};
use crate::tui::app::{App, Focus};
use crate::tui::palette;
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

/// Pre-computed power bar strings (zero-allocation optimization)
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

    // === BUILD ALL VISIBLE TREE LINES ===
    let mut all_lines: Vec<Line> = Vec::new();
    let mut idx = 0;

    // Node Classes section (Realm > Layer > Class > Instance)
    build_node_class_lines(&mut all_lines, &mut idx, app, area.width, focused);

    // Arcs section (ArcFamily > ArcClass)
    build_arc_lines(&mut all_lines, &mut idx, app, area.width, focused);

    // === SCROLL + LAYOUT ===

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
            Style::default().fg(palette::EMPTY_SLOT),
        )));
        f.render_widget(separator, separator_area);
    }

    // Render tree content below breadcrumb (with padding)
    let tree_area = Rect::new(content_x, tree_y, content_width, tree_height);

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, tree_area);

    // Render mini-map on right side
    let sep_x = inner_area.x + inner_area.width - minimap_width;
    let minimap_area = Rect::new(sep_x + 1, inner_area.y, 2, inner_area.height);
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
