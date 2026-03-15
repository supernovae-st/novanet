//! Identity & Data Management panel (top center).
//!
//! v0.21.0: Complete redesign — focuses exclusively on data management and graph location.
//! - NO repeated node info (key, name, description) — that's in Properties panel
//! - Dynamic title: "Schema Structure" (schema nodes) vs "Data Management" (data nodes)
//! - Category-colored rounded borders with DataCategory-aware styling
//! - Pipeline visualization showing how node was created (YAML → Cypher → Neo4j)
//! - Lifecycle badges (reseed, backup, editable) with inline explanations
//! - Completeness gauge using Unicode block characters
//! - Scrollable with Scrollbar when focused
//! - Visual distinction between schema and data nodes

mod data;
mod helpers;
mod schema;

use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};

use crate::tui::app::{App, Focus};
use crate::tui::data::TreeItem;
use crate::tui::palette;
use crate::tui::theme;
use crate::tui::widgets::{bordered_block, clamp_scroll};

// DataCategory and ProvenanceMeta re-exported for submodules via super::super::info

// =============================================================================
// COLORS — sourced from crate::tui::palette
// =============================================================================

const COLOR_DIM: Color = palette::DIM_110;
const COLOR_MUTED: Color = palette::MUTED_130;
const COLOR_SCHEMA_STRUCTURE: Color = palette::SLATE_500;
const COLOR_FLOW: Color = palette::HINT_TEXT;
const COLOR_FILE: Color = palette::FILE_TEXT;
const COLOR_CMD: Color = palette::PROP_KEY;
const COLOR_GAUGE_FILLED: Color = palette::GREEN_500;

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Identity + Data Management panel.
pub fn render_identity_panel(f: &mut Frame, area: Rect, app: &mut App) {
  let is_focused = app.focus == Focus::Identity;

  // Build content lines
  let (lines, panel_title, border_color) = build_panel_content(app);

  // Track total line count for scrollbar
  let total_lines = lines.len();
  app.identity_line_count = total_lines;

  // Clamp scroll position
  let visible_height = area.height.saturating_sub(2) as usize; // minus borders
  clamp_scroll(&mut app.identity_scroll, total_lines, visible_height);

  // Build block with rounded borders and dynamic title
  let focus_border = if is_focused {
    theme::ui::ACCENT
  } else {
    border_color.unwrap_or(palette::BORDER_UNFOCUSED)
  };

  let mut block = bordered_block(format!(" {} ", panel_title), focus_border);

  // Bottom title: scroll hint when focused and content overflows
  if is_focused && total_lines > visible_height {
    block = block.title_bottom(Line::from(vec![
      Span::styled(" j", Style::default().fg(COLOR_CMD)),
      Span::styled("/", Style::default().fg(COLOR_DIM)),
      Span::styled("k", Style::default().fg(COLOR_CMD)),
      Span::styled(" scroll ", Style::default().fg(COLOR_DIM)),
    ]));
  }

  let paragraph = Paragraph::new(lines)
    .block(block)
    .scroll((app.identity_scroll as u16, 0));

  f.render_widget(paragraph, area);

  // Render scrollbar when focused and content overflows
  if is_focused && total_lines > visible_height {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
      .thumb_style(Style::default().fg(focus_border));
    let mut scrollbar_state = ScrollbarState::new(total_lines)
      .position(app.identity_scroll);
    f.render_stateful_widget(
      scrollbar,
      area.inner(Margin { vertical: 1, horizontal: 0 }),
      &mut scrollbar_state,
    );
  }
}

// =============================================================================
// CONTENT DISPATCHER
// =============================================================================

/// Build panel content, title, and border color based on current selection.
/// Returns (lines, title, optional border color for category).
fn build_panel_content(app: &App) -> (Vec<Line<'static>>, String, Option<Color>) {
  let item = app.tree.item_at(app.tree_cursor);

  match item {
    // Schema nodes
    Some(TreeItem::Realm(realm)) => schema::build_realm(realm),
    Some(TreeItem::Layer(realm, layer)) => schema::build_layer(realm, layer),
    Some(TreeItem::Class(realm, layer, class_info)) => {
      schema::build_class(realm, layer, class_info)
    }
    Some(TreeItem::ArcFamily(family)) => schema::build_arc_family(family),
    Some(TreeItem::ArcClass(family, arc_class)) => schema::build_arc_class(family, arc_class),
    Some(TreeItem::ClassesSection) => schema::build_classes_section(),
    Some(TreeItem::ArcsSection) => schema::build_arcs_section(),

    // Data nodes
    Some(TreeItem::Instance(realm, layer, class_info, instance)) => {
      data::build_instance(realm, layer, class_info, instance)
    }
    Some(TreeItem::EntityNativeItem(realm, layer, class_info, native)) => {
      data::build_entity_native(realm, layer, class_info, native)
    }
    Some(TreeItem::EntityCategory(realm, layer, class_info, category_item)) => {
      data::build_entity_category(realm, layer, class_info, category_item)
    }
    Some(TreeItem::EntityGroup(realm, layer, class_info, group)) => {
      data::build_entity_group(realm, layer, class_info, group)
    }

    // Nothing selected
    _ => {
      let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
          "  Select a node to see data management info",
          Style::default().fg(COLOR_DIM),
        )),
      ];
      (lines, "Identity".into(), None)
    }
  }
}
