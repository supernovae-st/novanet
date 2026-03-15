//! Arc relationships panel rendering for TUI.
//!
//! v0.13: Consolidated ARCS panel (merged Graph + Arcs boxes).
//! Displays Neo4j arc relationships for the selected Class or Instance,
//! realm/layer statistics, and arc details.

mod helpers;
mod stats;
mod views;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};

use super::super::app::{App, Focus};
use super::super::data::TreeItem;
use super::{BOX_BORDER_SELECTED, BOX_BORDER_UNFOCUSED, STYLE_DIM, scroll_indicator};
use crate::tui::widgets::bordered_block;

// =============================================================================
// ARC DISPLAY CONSTANTS
// =============================================================================

/// Separator line for arc sections (44 dashes with 2-space indent).
const ARC_SEPARATOR: &str = "  ────────────────────────────────────────────";

// =============================================================================
// SCROLL HELPER
// =============================================================================

/// Render lines with scroll support and scrollbar.
/// Returns the total number of lines for scroll calculation.
fn render_with_scroll(f: &mut Frame, area: Rect, lines: Vec<Line>, scroll_offset: usize) -> usize {
    let total_lines = lines.len();
    let visible_height = area.height as usize;
    let max_scroll = total_lines.saturating_sub(visible_height);
    let scroll = scroll_offset.min(max_scroll);

    let paragraph = Paragraph::new(lines).scroll((scroll as u16, 0));
    f.render_widget(paragraph, area);

    // Render scrollbar if content exceeds visible area
    if total_lines > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state = ScrollbarState::new(max_scroll).position(scroll);

        let scrollbar_area = Rect {
            x: area.x + area.width.saturating_sub(1),
            y: area.y,
            width: 1,
            height: area.height,
        };

        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    total_lines
}

// =============================================================================
// GRAPH PANEL (slim dispatcher)
// =============================================================================

/// Graph panel: Displays Neo4j relationships for the selected Class or Instance.
///
/// Shows real arc data from Neo4j when a Class is selected,
/// instance arcs in Data mode, or contextual messages for other selections.
///
/// Arcs panel [4] using Focus::Arcs for panel selection.
pub fn render_graph_panel(f: &mut Frame, area: Rect, app: &mut App) {
    // Use Focus for panel focus
    let selected = app.focus == Focus::Arcs;
    let border_color = if selected {
        BOX_BORDER_SELECTED
    } else {
        BOX_BORDER_UNFOCUSED
    };

    // Calculate arc counts for title (separate in/out)
    let (incoming_count, outgoing_count, arcs_loading) =
        if let Some(ref arcs) = app.details.class_arcs {
            (arcs.incoming.len(), arcs.outgoing.len(), false)
        } else if let Some(TreeItem::Instance(_, _, _, inst)) = app.current_item() {
            (
                inst.incoming_arcs.len(),
                inst.outgoing_arcs.len(),
                inst.arcs_loading,
            )
        } else {
            (0, 0, false)
        };

    // Build enhanced title with selection indicator
    // Order: Out first (→), then In (←) - more logical flow direction
    let title_spans = if selected {
        let mut spans = vec![
            Span::styled(
                " \u{25B6} ", // ▶
                Style::default()
                    .fg(BOX_BORDER_SELECTED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "ARCS ",
                Style::default()
                    .fg(BOX_BORDER_SELECTED)
                    .add_modifier(Modifier::BOLD),
            ),
        ];
        if arcs_loading {
            spans.push(Span::styled("[...] ", Style::default().fg(Color::DarkGray)));
        } else if incoming_count > 0 || outgoing_count > 0 {
            spans.push(Span::styled("[", Style::default().fg(Color::DarkGray)));
            spans.push(Span::styled(
                format!("{}→", outgoing_count),
                Style::default().fg(Color::Cyan),
            ));
            spans.push(Span::styled(" ", Style::default().fg(Color::DarkGray)));
            spans.push(Span::styled(
                format!("←{}", incoming_count),
                Style::default().fg(Color::Magenta),
            ));
            spans.push(Span::styled("] ", Style::default().fg(Color::DarkGray)));
        }
        spans
    } else {
        let mut spans = vec![Span::styled(" ARCS ", Style::default().fg(Color::DarkGray))];
        if arcs_loading {
            spans.push(Span::styled("[...] ", Style::default().fg(Color::DarkGray)));
        } else if incoming_count > 0 || outgoing_count > 0 {
            spans.push(Span::styled("[", Style::default().fg(Color::DarkGray)));
            spans.push(Span::styled(
                format!("{}→ ←{}", outgoing_count, incoming_count),
                Style::default().fg(Color::DarkGray),
            ));
            spans.push(Span::styled("] ", Style::default().fg(Color::DarkGray)));
        }
        spans
    };

    // Add scroll indicator using cached line count from previous frame
    let visible_height = area.height.saturating_sub(2) as usize; // -2 for borders
    let scroll_hint = scroll_indicator(app.arcs_scroll, app.arcs_line_count, visible_height);

    let block = bordered_block(Line::from(title_spans), border_color)
        .title_bottom(Span::styled(scroll_hint, STYLE_DIM));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // === DISPATCH TO VIEW FUNCTIONS ===

    // 1. Loading indicator (any pending async operation)
    if views::render_loading(f, inner, app) {
        return;
    }

    // 2. Realm details view
    if app.details.realm.is_some() {
        views::render_realm_details(f, inner, app);
        return;
    }

    // 3. Layer details view
    if app.details.layer.is_some() {
        views::render_layer_details(f, inner, app);
        return;
    }

    // 4. Instance arcs view (Data mode)
    if matches!(app.current_item(), Some(TreeItem::Instance(..))) {
        views::render_instance_arcs(f, inner, app);
        return;
    }

    // 5. Class arcs view (from Neo4j)
    if app.details.class_arcs.is_some() {
        views::render_class_arcs(f, inner, app);
        return;
    }

    // 6. Arc class details view
    if app.details.arc_class.is_some() {
        views::render_arc_class_details(f, inner, app);
        return;
    }

    // 7. Fallback: distribution stats + contextual hint
    views::render_fallback(f, inner, app);
}
