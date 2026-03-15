//! Mini-map rendering for tree panel (v11.6).
//!
//! Shows a proportional overview of the tree with cursor and viewport indicators.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::tui::app::App;
use crate::tui::palette;
use crate::tui::theme::hex_to_color;
use crate::tui::ui::COLOR_MUTED_TEXT;

/// Information needed to render the mini-map.
pub struct MiniMapInfo {
    /// Total number of items in the tree.
    pub total_items: usize,
    /// Current cursor position (0-indexed).
    pub cursor_pos: usize,
    /// First visible item index.
    pub scroll_offset: usize,
    /// Number of visible items in viewport.
    pub visible_count: usize,
    /// Current realm color (for theming).
    pub realm_color: Color,
}

/// Render mini-map on the right side of tree panel.
pub fn render_minimap(f: &mut Frame, area: Rect, info: &MiniMapInfo) {
    if area.height == 0 || area.width < 2 || info.total_items == 0 {
        return;
    }

    let height = area.height as usize;
    let mut lines: Vec<Line> = Vec::with_capacity(height);

    let total = info.total_items;
    let viewport_start = info.scroll_offset;
    let viewport_end = (viewport_start + info.visible_count).min(total);
    let cursor = info.cursor_pos;

    for row in 0..height {
        let tree_start = (row * total) / height;
        let tree_end = ((row + 1) * total) / height;

        let cursor_in_range = cursor >= tree_start && cursor < tree_end.max(tree_start + 1);
        let viewport_overlaps = tree_end > viewport_start && tree_start < viewport_end;

        let (symbol, color) = if cursor_in_range {
            ("██", info.realm_color)
        } else if viewport_overlaps {
            ("░░", COLOR_MUTED_TEXT)
        } else {
            ("▒▒", palette::EMPTY_SLOT)
        };

        lines.push(Line::from(Span::styled(symbol, Style::default().fg(color))));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

/// Build mini-map info from current app state.
pub fn build_minimap_info(app: &App, visible_height: usize) -> MiniMapInfo {
    let realm_color = match app.current_item() {
        Some(crate::tui::data::TreeItem::Realm(r)) => hex_to_color(&r.color),
        Some(crate::tui::data::TreeItem::Layer(r, _)) => hex_to_color(&r.color),
        Some(crate::tui::data::TreeItem::Class(r, _, _)) => hex_to_color(&r.color),
        Some(crate::tui::data::TreeItem::EntityCategory(r, _, _, _)) => hex_to_color(&r.color),
        Some(crate::tui::data::TreeItem::Instance(r, _, _, _)) => hex_to_color(&r.color),
        _ => Color::Cyan,
    };

    MiniMapInfo {
        total_items: app.current_item_count(),
        cursor_pos: app.tree_cursor,
        scroll_offset: app.tree_scroll,
        visible_count: visible_height,
        realm_color,
    }
}
