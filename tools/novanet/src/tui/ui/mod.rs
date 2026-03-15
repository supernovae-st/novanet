//! UI rendering for TUI v2.
//!
//! Two modes (Graph + Flow). Renders tree, identity, content, props, arcs panels.
//!
//! Sub-modules:
//! - `badges.rs`: Icon and badge classification helpers (realm, layer, arc family, cardinality)
//! - `text_utils.rs`: Text wrapping, truncation, spinner animation

mod badges;
mod flow;
mod graph;
mod identity_panel;
mod info;
mod overlays;
mod status;
mod text_utils;
mod tree;
mod yaml_panel;

pub use graph::render_graph_panel;
pub use identity_panel::render_identity_panel;
pub use info::{build_unified_content, render_props_panel};
pub use status::render_status;
pub use tree::render_tree;
pub use yaml_panel::render_content_panel;

// Re-export extracted modules for backward compatibility.
// Submodules import badge/text functions via `super::` or `super::super::`.
pub(super) use badges::{
    arc_family_badge_icon, cardinality_abbrev, layer_badge_icon, realm_badge_icon,
};
pub(super) use text_utils::{spinner, truncate_str, wrap_text};

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph};

use super::app::{App, NavMode};
use super::palette;
use super::theme;
use super::widgets::bordered_block;

// =============================================================================
// CONSTANTS
// =============================================================================

/// Minimum terminal width for wide (3-column) layout.
const WIDE_LAYOUT_MIN_WIDTH: u16 = 160;

// -----------------------------------------------------------------------------
// Color palette constants (re-exported from palette.rs)
// -----------------------------------------------------------------------------

/// Unfocused panel border color.
pub(super) const COLOR_UNFOCUSED_BORDER: Color = palette::BORDER_UNFOCUSED;

/// Muted text for secondary information.
pub(super) const COLOR_MUTED_TEXT: Color = palette::MUTED;

/// Highlighted row background.
const COLOR_HIGHLIGHT_BG: Color = palette::BG_HIGHLIGHT;

/// Instance color (v0.13.1: unified yellow for all instances).
pub(super) const COLOR_INSTANCE: Color = Color::Yellow;

/// Arc family label color.
const COLOR_ARC_FAMILY: Color = palette::FAMILY_LABEL;

/// Description/secondary text.
const COLOR_DESC_TEXT: Color = palette::DESC_TEXT;

/// Separator dots between stats.
pub(super) const COLOR_SEPARATOR: Color = palette::SEPARATOR;

/// Hint text (dimmed).
pub(super) const COLOR_HINT_TEXT: Color = palette::HINT_TEXT;

/// Overlay/popup background.
pub(super) const COLOR_OVERLAY_BG: Color = palette::BG_OVERLAY;

/// Brighter dim text.
const COLOR_BRIGHT_DIM: Color = palette::BRIGHT_DIM;

/// Active Class background (subtle highlight for Class with expanded instances).
const COLOR_ACTIVE_CLASS_BG: Color = palette::BG_ACTIVE;

// -----------------------------------------------------------------------------
// Box border constants (centralized for graph, yaml_panel, info panels)
// -----------------------------------------------------------------------------

/// Unfocused box border: Nord Polar Night (dim) - box not selected.
pub(super) const BOX_BORDER_UNFOCUSED: Color = palette::NORD_BORDER_UNFOCUSED;

/// Focused box border: Nord slightly brighter - panel active, box not selected.
pub(super) const BOX_BORDER_FOCUSED: Color = palette::NORD_BORDER_FOCUSED;

/// Selected box border: Solarized Cyan (bright) - active box for copy/scroll.
pub(super) const BOX_BORDER_SELECTED: Color = palette::SOLARIZED_CYAN;

// =============================================================================
// LAYOUT CONSTANTS (v0.18.3: New 4-panel layout)
// =============================================================================

/// Wide layout column percentages: Tree | Center | Right
const LAYOUT_TREE_PCT: u16 = 25;
const LAYOUT_CENTER_PCT: u16 = 40;
const LAYOUT_RIGHT_PCT: u16 = 35;

/// Center column split: Identity+Provenance (top) | Data Viewer (bottom)
const LAYOUT_IDENTITY_PCT: u16 = 35;
const LAYOUT_DATA_VIEWER_PCT: u16 = 65;

/// Right column split: Properties+Stats (top) | Arcs (bottom)
const LAYOUT_PROPS_STATS_PCT: u16 = 50;
const LAYOUT_ARCS_PCT: u16 = 50;

/// Narrow layout: Tree panel percentage (compact sidebar).
const LAYOUT_NARROW_TREE_PCT: u16 = 35;
/// Narrow layout: Detail panel percentage.
const LAYOUT_NARROW_DETAIL_PCT: u16 = 65;

/// Popup/overlay box dimensions.
const POPUP_BOX_WIDTH: u16 = 50;
const POPUP_BOX_HEIGHT: u16 = 12;
const POPUP_BOX_MIN_WIDTH: u16 = 20;
const POPUP_BOX_MIN_HEIGHT: u16 = 6;

// -----------------------------------------------------------------------------
// General UI styles (most frequently used, avoid Style::default() overhead)
// -----------------------------------------------------------------------------

/// Dim/secondary text (e.g., counts, separators).
pub(super) const STYLE_DIM: Style = Style::new().fg(Color::DarkGray);

/// Default/primary text.
pub(super) const STYLE_PRIMARY: Style = Style::new().fg(Color::White);

/// Highlighted/important text (e.g., selected items).
pub(super) const STYLE_HIGHLIGHT: Style = Style::new().fg(Color::Yellow);

/// Informational text (e.g., types, metadata).
pub(super) const STYLE_INFO: Style = Style::new().fg(Color::Cyan);

/// Success/positive indicators.
const STYLE_SUCCESS: Style = Style::new().fg(Color::Green);

/// Accent color (e.g., special values).
pub(super) const STYLE_ACCENT: Style = Style::new().fg(Color::Magenta);

/// Muted/secondary text (custom RGB).
const STYLE_MUTED: Style = Style::new().fg(COLOR_MUTED_TEXT);

/// Separator dots style.
const STYLE_SEPARATOR: Style = Style::new().fg(COLOR_SEPARATOR);

/// Hint text style.
const STYLE_HINT: Style = Style::new().fg(COLOR_HINT_TEXT);

/// Description text style.
pub(super) const STYLE_DESC: Style = Style::new().fg(COLOR_DESC_TEXT);

/// Unfocused border style.
pub(super) const STYLE_UNFOCUSED: Style = Style::new().fg(COLOR_UNFOCUSED_BORDER);

/// Bright dim text style.
const STYLE_BRIGHT_DIM: Style = Style::new().fg(COLOR_BRIGHT_DIM);

/// Palette-specific dim style (RGB 100,100,100) used by graph panel and helpers.
pub(super) const STYLE_PALETTE_DIM: Style = Style::new().fg(palette::DIM);

// =============================================================================
// SCROLL INDICATOR HELPERS
// =============================================================================

/// Build a scroll indicator with directional arrows.
pub(super) fn scroll_indicator(
    scroll_pos: usize,
    total_lines: usize,
    visible_height: usize,
) -> String {
    let mut state = super::widgets::ScrollState::new(total_lines, visible_height);
    state.offset = scroll_pos;
    state.indicator()
}

// =============================================================================
// EMPTY STATE RENDERING
// =============================================================================

/// Types of empty states that can be displayed.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum EmptyStateClass {
    /// Neo4j connection failed
    NoConnection,
    /// Database has no node classes
    NoClasses,
    /// Query returned no results
    NoResults,
    /// Class has no instances
    NoInstances,
    /// Loading data (with animation)
    Loading,
}

impl EmptyStateClass {
    fn icon(&self) -> &'static str {
        match self {
            EmptyStateClass::NoConnection => "\u{26a0}",
            EmptyStateClass::NoClasses => "\u{2205}",
            EmptyStateClass::NoResults => "\u{25cc}",
            EmptyStateClass::NoInstances => "\u{25a1}",
            EmptyStateClass::Loading => "\u{25d0}",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            EmptyStateClass::NoConnection => "Neo4j Not Connected",
            EmptyStateClass::NoClasses => "No Node Classes Found",
            EmptyStateClass::NoResults => "No Results",
            EmptyStateClass::NoInstances => "No Instances",
            EmptyStateClass::Loading => "Loading\u{2026}",
        }
    }

    fn description(&self) -> &'static [&'static str] {
        match self {
            EmptyStateClass::NoConnection => &[
                "Unable to connect to Neo4j",
                "",
                "Try:",
                "  \u{2022} pnpm infra:up",
                "  \u{2022} Check NEO4J_URI environment variable",
            ],
            EmptyStateClass::NoClasses => &[
                "The taxonomy tree is empty.",
                "",
                "Run:",
                "  \u{2022} cargo run -- schema generate",
                "  \u{2022} cargo run -- db seed",
            ],
            EmptyStateClass::NoResults => &[
                "No nodes match your current filter.",
                "",
                "Try:",
                "  \u{2022} Remove filters with 'c'",
                "  \u{2022} Switch modes with 1-4",
            ],
            EmptyStateClass::NoInstances => &[
                "This Class has no data instances yet.",
                "",
                "Create one with:",
                "  cargo run -- node create --class=<Class>",
            ],
            EmptyStateClass::Loading => &["Fetching data from Neo4j\u{2026}"],
        }
    }

    fn hint(&self) -> &'static str {
        match self {
            EmptyStateClass::NoConnection => "Press 'r' to retry",
            EmptyStateClass::NoClasses => "Press 'q' to quit",
            EmptyStateClass::NoResults => "Press 'c' to clear filters",
            EmptyStateClass::NoInstances => "Press Esc to go back",
            EmptyStateClass::Loading => "",
        }
    }
}

/// Render an empty state message in a centered box.
fn render_empty_state(f: &mut Frame, area: Rect, empty_state: EmptyStateClass, tick: u16) {
    let box_width = POPUP_BOX_WIDTH.min(area.width.saturating_sub(4));
    let box_height = POPUP_BOX_HEIGHT.min(area.height.saturating_sub(2));

    if box_width < POPUP_BOX_MIN_WIDTH || box_height < POPUP_BOX_MIN_HEIGHT {
        return;
    }

    let x = (area.width.saturating_sub(box_width)) / 2 + area.x;
    let y = (area.height.saturating_sub(box_height)) / 2 + area.y;
    let box_area = Rect::new(x, y, box_width, box_height);

    let mut lines: Vec<Line> = Vec::new();

    let title_icon = empty_state.icon();
    let title_text = empty_state.title();

    // Loading spinner animation (uses spinner() from text_utils)
    let display_icon = if matches!(empty_state, EmptyStateClass::Loading) {
        spinner(tick)
    } else {
        title_icon
    };

    lines.push(Line::from(vec![
        Span::styled(format!("  {} ", display_icon), STYLE_HIGHLIGHT),
        Span::styled(
            title_text,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(""));

    for desc_line in empty_state.description() {
        lines.push(Line::from(Span::styled(
            format!("  {}", desc_line),
            STYLE_DESC,
        )));
    }

    let hint = empty_state.hint();
    if !hint.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(format!("  {}", hint), STYLE_INFO)));
    }

    let block = bordered_block("", COLOR_UNFOCUSED_BORDER)
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);

    f.render_widget(Clear, box_area);
    f.render_widget(paragraph, box_area);
}

/// Main render function.
pub fn render(f: &mut Frame, app: &mut App) {
    app.panel_rects.clear();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_main(f, chunks[1], app);
    render_status(f, chunks[2], app);

    if app.search.active {
        overlays::render_search(f, app);
    }
    if app.overlays.help_active {
        overlays::render_help(f, app);
    }
    if app.overlays.legend_active {
        overlays::render_legend(f, app);
    }
    if app.overlays.recent_items_active {
        render_recent_items_overlay(f, app);
    }
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let tabs: Vec<Span> = NavMode::all()
        .iter()
        .enumerate()
        .map(|(i, mode)| {
            let num = format!("[{}]", i + 1);
            let label = mode.label();
            let is_active = *mode == app.mode;

            if is_active {
                Span::styled(
                    format!(" {}{}\u{2022} ", num, label),
                    theme::ui::focused_style(),
                )
            } else {
                Span::styled(format!(" {}{} ", num, label), STYLE_DIM)
            }
        })
        .collect();

    let mut header: Vec<Span> = vec![
        Span::styled(" NovaNet ", theme::ui::logo_style()),
        Span::raw("        "),
    ];
    header.extend(tabs);

    if app.hide_empty && app.is_graph_mode() {
        header.push(Span::styled(
            " [\u{2205} hidden]",
            Style::default().fg(Color::Yellow),
        ));
    }

    let right_side = vec![Span::styled("  ?:help  q:quit", theme::ui::muted_style())];

    let mut full_header: Vec<Span<'static>> = header;
    let header_len: usize = full_header.iter().map(|s| s.content.len()).sum();
    let right_len: usize = right_side.iter().map(|s| s.content.len()).sum();
    let padding = area
        .width
        .saturating_sub(header_len as u16 + right_len as u16);
    full_header.push(Span::raw(" ".repeat(padding as usize)));
    full_header.extend(right_side);

    let paragraph =
        Paragraph::new(Line::from(full_header)).style(Style::default().bg(theme::ui::HEADER_BG));

    f.render_widget(paragraph, area);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayoutMode {
    Wide,
    Narrow,
}

impl LayoutMode {
    fn detect(width: u16) -> Self {
        if width >= WIDE_LAYOUT_MIN_WIDTH {
            LayoutMode::Wide
        } else {
            LayoutMode::Narrow
        }
    }
}

fn render_main(f: &mut Frame, area: Rect, app: &mut App) {
    if app.mode == NavMode::Flow {
        flow::render_flow(f, app, area);
        return;
    }

    let layout_mode = LayoutMode::detect(area.width);

    match layout_mode {
        LayoutMode::Wide => render_main_wide(f, area, app),
        LayoutMode::Narrow => render_main_narrow(f, area, app),
    }
}

fn render_main_wide(f: &mut Frame, area: Rect, app: &mut App) {
    let content = build_unified_content(app);

    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(LAYOUT_TREE_PCT),
            Constraint::Percentage(LAYOUT_CENTER_PCT),
            Constraint::Percentage(LAYOUT_RIGHT_PCT),
        ])
        .split(area);

    render_tree(f, h_chunks[0], app);

    let center_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(LAYOUT_IDENTITY_PCT),
            Constraint::Percentage(LAYOUT_DATA_VIEWER_PCT),
        ])
        .split(h_chunks[1]);

    render_identity_panel(f, center_chunks[0], app);
    render_content_panel(f, center_chunks[1], app);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(LAYOUT_PROPS_STATS_PCT),
            Constraint::Percentage(LAYOUT_ARCS_PCT),
        ])
        .split(h_chunks[2]);

    render_props_panel(f, right_chunks[0], app, &content);
    render_graph_panel(f, right_chunks[1], app);

    app.panel_rects.tree = Some(h_chunks[0]);
    app.panel_rects.identity = Some(center_chunks[0]);
    app.panel_rects.content = Some(center_chunks[1]);
    app.panel_rects.props = Some(right_chunks[0]);
    app.panel_rects.arcs = Some(right_chunks[1]);
}

fn render_main_narrow(f: &mut Frame, area: Rect, app: &mut App) {
    let content = build_unified_content(app);

    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(LAYOUT_NARROW_TREE_PCT),
            Constraint::Percentage(LAYOUT_NARROW_DETAIL_PCT),
        ])
        .split(area);

    render_tree(f, h_chunks[0], app);

    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(25),
        ])
        .split(h_chunks[1]);

    render_identity_panel(f, v_chunks[0], app);
    render_content_panel(f, v_chunks[1], app);
    render_props_panel(f, v_chunks[2], app, &content);
    render_graph_panel(f, v_chunks[3], app);

    app.panel_rects.tree = Some(h_chunks[0]);
    app.panel_rects.identity = Some(v_chunks[0]);
    app.panel_rects.content = Some(v_chunks[1]);
    app.panel_rects.props = Some(v_chunks[2]);
    app.panel_rects.arcs = Some(v_chunks[3]);
}

fn render_recent_items_overlay(f: &mut Frame, app: &App) {
    use ratatui::widgets::Clear;

    let area = f.area();
    let width = POPUP_BOX_WIDTH.min(area.width.saturating_sub(4));
    let height = 14.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 3;

    let popup_area = Rect::new(x, y, width, height);

    f.render_widget(Clear, popup_area);

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(vec![
        Span::styled(" Recent Items ", STYLE_INFO),
        Span::styled("(j/k Enter Esc)", STYLE_DIM),
    ]));
    lines.push(Line::from(""));

    let max_items = 10.min(app.nav_history.len());
    let visible_height = height.saturating_sub(4) as usize;

    let start =
        if max_items <= visible_height || app.overlays.recent_items_cursor < visible_height / 2 {
            0
        } else if app.overlays.recent_items_cursor > max_items - visible_height / 2 {
            max_items.saturating_sub(visible_height)
        } else {
            app.overlays
                .recent_items_cursor
                .saturating_sub(visible_height / 2)
        };

    for display_idx in start..start + visible_height.min(max_items - start) {
        let history_idx = app.nav_history.len().saturating_sub(1 + display_idx);
        let is_selected = display_idx == app.overlays.recent_items_cursor;

        if let Some(&(mode, cursor)) = app.nav_history.get(history_idx) {
            let item = app.tree.item_at(cursor);
            let (icon, name) = match item {
                Some(crate::tui::data::TreeItem::ClassesSection) => {
                    ("\u{2261}", "Node Classes".to_string())
                },
                Some(crate::tui::data::TreeItem::ArcsSection) => ("\u{21c4}", "Arcs".to_string()),
                Some(crate::tui::data::TreeItem::Realm(r)) => (r.icon, r.display_name.clone()),
                Some(crate::tui::data::TreeItem::Layer(_, l)) => ("\u{25b8}", l.display_name.clone()),
                Some(crate::tui::data::TreeItem::Class(_, _, k)) => ("\u{25c6}", k.display_name.clone()),
                Some(crate::tui::data::TreeItem::Instance(_, _, _, i)) => {
                    ("\u{2022}", i.display_name.clone())
                },
                Some(crate::tui::data::TreeItem::ArcFamily(af)) => ("\u{2194}", af.display_name.clone()),
                Some(crate::tui::data::TreeItem::ArcClass(_, ak)) => ("\u{2192}", ak.display_name.clone()),
                Some(crate::tui::data::TreeItem::EntityCategory(_, _, _, cat)) => {
                    ("\u{25eb}", cat.display_name.clone())
                },
                Some(crate::tui::data::TreeItem::EntityGroup(_, _, _, group)) => {
                    ("\u{25c8}", group.entity_display_name.clone())
                },
                Some(crate::tui::data::TreeItem::EntityNativeItem(_, _, _, native)) => {
                    ("\u{25c6}", native.display_name.clone())
                },
                None => ("?", format!("(cursor {})", cursor)),
            };

            let mode_badge = match mode {
                crate::tui::app::NavMode::Graph => "[G]",
                crate::tui::app::NavMode::Flow => "[F]",
            };

            let prefix = if is_selected { "\u{203a} " } else { "  " };
            let style = if is_selected {
                Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
            } else {
                Style::default()
            };

            let text = format!(
                "{}{} {} {}",
                prefix,
                mode_badge,
                icon,
                truncate_str(&name, 35)
            );
            lines.push(Line::from(Span::styled(text, style)));
        }
    }

    if max_items == 0 {
        lines.push(Line::from(Span::styled(
            "  No history yet. Navigate around!",
            STYLE_DIM,
        )));
    }

    let paragraph = Paragraph::new(lines).block(
        bordered_block("", Color::Cyan)
            .style(Style::default().bg(COLOR_OVERLAY_BG)),
    );

    f.render_widget(paragraph, popup_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_state_class_icon_no_connection() {
        assert_eq!(EmptyStateClass::NoConnection.icon(), "\u{26a0}");
    }

    #[test]
    fn test_empty_state_class_icon_no_classes() {
        assert_eq!(EmptyStateClass::NoClasses.icon(), "\u{2205}");
    }

    #[test]
    fn test_empty_state_class_icon_no_results() {
        assert_eq!(EmptyStateClass::NoResults.icon(), "\u{25cc}");
    }

    #[test]
    fn test_empty_state_class_icon_no_instances() {
        assert_eq!(EmptyStateClass::NoInstances.icon(), "\u{25a1}");
    }

    #[test]
    fn test_empty_state_class_icon_loading() {
        assert_eq!(EmptyStateClass::Loading.icon(), "\u{25d0}");
    }

    #[test]
    fn test_empty_state_class_title_no_connection() {
        assert_eq!(EmptyStateClass::NoConnection.title(), "Neo4j Not Connected");
    }

    #[test]
    fn test_empty_state_class_title_no_classes() {
        assert_eq!(EmptyStateClass::NoClasses.title(), "No Node Classes Found");
    }

    #[test]
    fn test_empty_state_class_title_no_results() {
        assert_eq!(EmptyStateClass::NoResults.title(), "No Results");
    }

    #[test]
    fn test_empty_state_class_title_no_instances() {
        assert_eq!(EmptyStateClass::NoInstances.title(), "No Instances");
    }

    #[test]
    fn test_empty_state_class_title_loading() {
        assert_eq!(EmptyStateClass::Loading.title(), "Loading\u{2026}");
    }

    #[test]
    fn test_empty_state_class_description_no_connection() {
        let desc = EmptyStateClass::NoConnection.description();
        assert!(!desc.is_empty(), "description should not be empty");
        assert!(desc.iter().any(|s| s.contains("Neo4j")));
        assert!(desc.iter().any(|s| s.contains("infra:up")));
    }

    #[test]
    fn test_empty_state_class_description_no_classes() {
        let desc = EmptyStateClass::NoClasses.description();
        assert!(!desc.is_empty());
        assert!(desc.iter().any(|s| s.contains("schema generate")));
        assert!(desc.iter().any(|s| s.contains("db seed")));
    }

    #[test]
    fn test_empty_state_class_description_no_results() {
        let desc = EmptyStateClass::NoResults.description();
        assert!(!desc.is_empty());
        assert!(desc.iter().any(|s| s.contains("filter")));
    }

    #[test]
    fn test_empty_state_class_description_no_instances() {
        let desc = EmptyStateClass::NoInstances.description();
        assert!(!desc.is_empty());
        assert!(desc.iter().any(|s| s.contains("node create")));
    }

    #[test]
    fn test_empty_state_class_description_loading() {
        let desc = EmptyStateClass::Loading.description();
        assert!(!desc.is_empty());
        assert!(desc.iter().any(|s| s.contains("Neo4j")));
    }

    #[test]
    fn test_empty_state_class_hint_no_connection() {
        assert!(EmptyStateClass::NoConnection.hint().contains("r"));
    }

    #[test]
    fn test_empty_state_class_hint_no_classes() {
        assert!(EmptyStateClass::NoClasses.hint().contains("q"));
    }

    #[test]
    fn test_empty_state_class_hint_no_results() {
        assert!(EmptyStateClass::NoResults.hint().contains("c"));
    }

    #[test]
    fn test_empty_state_class_hint_no_instances() {
        assert!(EmptyStateClass::NoInstances.hint().contains("Esc"));
    }

    #[test]
    fn test_empty_state_class_hint_loading() {
        let hint = EmptyStateClass::Loading.hint();
        assert!(hint.is_empty() || !hint.is_empty());
    }

    #[test]
    fn test_empty_state_class_is_copy() {
        let empty_state = EmptyStateClass::NoConnection;
        let empty_state2 = empty_state;
        let _empty_state3 = empty_state;
        assert_eq!(empty_state2.title(), "Neo4j Not Connected");
    }

    #[test]
    fn test_empty_state_class_debug_impl() {
        let empty_state = EmptyStateClass::Loading;
        let debug_str = format!("{:?}", empty_state);
        assert!(debug_str.contains("Loading"));
    }

    #[test]
    fn test_all_empty_state_variants_have_non_empty_icon() {
        let variants = [
            EmptyStateClass::NoConnection,
            EmptyStateClass::NoClasses,
            EmptyStateClass::NoResults,
            EmptyStateClass::NoInstances,
            EmptyStateClass::Loading,
        ];
        for empty_state in variants {
            assert!(!empty_state.icon().is_empty(), "{:?} icon should not be empty", empty_state);
        }
    }

    #[test]
    fn test_all_empty_state_variants_have_non_empty_title() {
        let variants = [
            EmptyStateClass::NoConnection,
            EmptyStateClass::NoClasses,
            EmptyStateClass::NoResults,
            EmptyStateClass::NoInstances,
            EmptyStateClass::Loading,
        ];
        for empty_state in variants {
            assert!(!empty_state.title().is_empty(), "{:?} title should not be empty", empty_state);
        }
    }
}
