//! UI rendering for TUI v2.

mod atlas;
mod audit;
mod graph;
mod overlays;
mod yaml_panel;

pub use atlas::render_atlas;
pub use audit::render_audit;
pub use graph::render_graph_panel;
pub use yaml_panel::render_yaml_panel;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Bar, BarChart, BarGroup, Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Sparkline,
};

use rustc_hash::FxHashSet;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

use super::app::{App, Focus, NavMode};
use super::data::{ArcDirection, TreeItem};
use super::schema::{PropertyStatus, ValidationStatus};
use super::theme::{self, hex_to_color};
use super::unicode::{display_width, truncate_start_to_width, truncate_to_width};

// =============================================================================
// CONSTANTS
// =============================================================================

/// Minimum terminal width for wide (3-column) layout.
const WIDE_LAYOUT_MIN_WIDTH: u16 = 160;

/// Spinner animation speed divisor (higher = slower animation).
const SPINNER_SPEED_DIVISOR: usize = 2;

// -----------------------------------------------------------------------------
// Color palette constants (avoid repeated Color::Rgb constructions)
// -----------------------------------------------------------------------------

/// Unfocused panel border color.
pub(super) const COLOR_UNFOCUSED_BORDER: Color = Color::Rgb(60, 60, 70);

/// Muted text for secondary information.
pub(super) const COLOR_MUTED_TEXT: Color = Color::Rgb(100, 100, 120);

/// Highlighted row background.
const COLOR_HIGHLIGHT_BG: Color = Color::Rgb(30, 40, 50);

/// Connected/active status indicator.
const COLOR_CONNECTED: Color = Color::Rgb(100, 180, 100);

/// Arc family label color.
const COLOR_ARC_FAMILY: Color = Color::Rgb(180, 140, 80);

/// Description/secondary text.
const COLOR_DESC_TEXT: Color = Color::Rgb(150, 150, 150);

/// Separator dots between stats.
const COLOR_SEPARATOR: Color = Color::Rgb(70, 70, 80);

/// Hint text (dimmed).
pub(super) const COLOR_HINT_TEXT: Color = Color::Rgb(80, 80, 100);

/// Overlay/popup background.
pub(super) const COLOR_OVERLAY_BG: Color = Color::Rgb(20, 20, 30);

/// Brighter dim text.
const COLOR_BRIGHT_DIM: Color = Color::Rgb(140, 140, 140);

/// Active Kind background (subtle highlight for Kind with expanded instances).
const COLOR_ACTIVE_KIND_BG: Color = Color::Rgb(25, 35, 45);

// -----------------------------------------------------------------------------
// Layout constants (percentages and sizes)
// -----------------------------------------------------------------------------

/// Wide layout: Tree panel percentage.
const LAYOUT_WIDE_TREE_PCT: u16 = 30;
/// Wide layout: Info+Graph panel percentage.
const LAYOUT_WIDE_INFO_PCT: u16 = 35;
/// Wide layout: YAML panel percentage.
const LAYOUT_WIDE_YAML_PCT: u16 = 35;
/// Wide layout: Info section percentage (within Info+Graph).
const LAYOUT_WIDE_INFO_SECTION_PCT: u16 = 60;
/// Wide layout: Graph section percentage (within Info+Graph).
const LAYOUT_WIDE_GRAPH_SECTION_PCT: u16 = 40;

/// Narrow layout: Tree panel percentage.
const LAYOUT_NARROW_TREE_PCT: u16 = 30;
/// Narrow layout: Detail panel percentage.
const LAYOUT_NARROW_DETAIL_PCT: u16 = 70;
/// Narrow layout: Info section percentage.
const LAYOUT_NARROW_INFO_PCT: u16 = 35;
/// Narrow layout: Graph section percentage.
const LAYOUT_NARROW_GRAPH_PCT: u16 = 30;
/// Narrow layout: YAML section percentage.
const LAYOUT_NARROW_YAML_PCT: u16 = 35;

/// Popup/overlay box dimensions.
const POPUP_BOX_WIDTH: u16 = 50;
const POPUP_BOX_HEIGHT: u16 = 12;
const POPUP_BOX_MIN_WIDTH: u16 = 20;
const POPUP_BOX_MIN_HEIGHT: u16 = 6;

// -----------------------------------------------------------------------------
// Trait icons for visual node classification
// -----------------------------------------------------------------------------

/// Get icon for a node trait.
/// - invariant: ◆ (solid diamond) - core structural nodes
/// - localized: ◇ (empty diamond) - locale-specific content
/// - knowledge: ● (solid circle) - knowledge atoms
/// - job: ○ (empty circle) - async processing nodes
/// - derived: ◈ (fancy diamond) - computed/derived nodes
fn trait_icon(trait_name: &str) -> &'static str {
    match trait_name {
        "invariant" => "◆",
        "localized" => "◇",
        "knowledge" => "●",
        "job" => "○",
        "derived" => "◈",
        _ => "·", // fallback
    }
}

/// Convert property type to short badge for schema overlay.
/// All badges are exactly 4 characters for consistent column alignment.
fn type_badge(prop_type: &str) -> &'static str {
    match prop_type.to_lowercase().as_str() {
        "string" => "str ",
        "json" => "json",
        "enum" => "enum",
        "datetime" => "dt  ",
        "int" | "integer" => "int ",
        "float" | "number" => "num ",
        "bool" | "boolean" => "bool",
        "array" | "list" => "arr ",
        "object" | "map" => "obj ",
        "url" | "uri" => "url ",
        "?" => "?   ", // unknown type from validation
        _ => "··· ",   // fallback for unknown types
    }
}

// -----------------------------------------------------------------------------
// Text wrapping helper (avoids Vec<char> allocation per frame)
// -----------------------------------------------------------------------------

/// Wrap text to lines of max `width` characters, returning owned Strings.
/// Uses char indices instead of collecting to Vec<char> for efficiency.
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = text.char_indices().peekable();
    while chars.peek().is_some() {
        let start = chars.peek().map(|(i, _)| *i).unwrap_or(0);
        let mut end = start;
        let mut count = 0;
        while let Some((idx, c)) = chars.peek() {
            if count >= width {
                break;
            }
            end = *idx + c.len_utf8();
            count += 1;
            chars.next();
        }
        if start < end {
            result.push(text[start..end].to_string());
        }
    }
    result
}

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

/// Error indicators.
const STYLE_ERROR: Style = Style::new().fg(Color::Red);

/// Warning indicators.
const STYLE_WARNING: Style = Style::new().fg(Color::Yellow);

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

/// Arc family label style.
const STYLE_ARC_FAMILY: Style = Style::new().fg(COLOR_ARC_FAMILY);

/// Bright dim text style.
const STYLE_BRIGHT_DIM: Style = Style::new().fg(COLOR_BRIGHT_DIM);

// =============================================================================
// SECTION SEPARATORS (Tier System)
// =============================================================================

/// Major section separator (double line) - for PROPERTY COVERAGE, LAYER BREAKDOWN, etc.
const SEPARATOR_MAJOR: &str = "══════════════════════════";

// =============================================================================
// SCROLL INDICATOR HELPERS
// =============================================================================

/// Build a scroll indicator with directional arrows.
///
/// Returns:
/// - `""` if content fits in visible area (no scrolling needed)
/// - `" ↓ [1/N] "` if at top (can scroll down)
/// - `" ↑ [N/N] "` if at bottom (can scroll up)
/// - `" ↕ [M/N] "` if in middle (can scroll both ways)
pub(super) fn scroll_indicator(
    scroll_pos: usize,
    total_lines: usize,
    visible_height: usize,
) -> String {
    if total_lines <= visible_height {
        return String::new();
    }

    let max_scroll = total_lines.saturating_sub(visible_height);
    let current_page = scroll_pos + 1;
    let total_pages = max_scroll + 1;

    let arrow = if scroll_pos == 0 {
        "↓" // at top, can scroll down
    } else if scroll_pos >= max_scroll {
        "↑" // at bottom, can scroll up
    } else {
        "↕" // in middle, can scroll both
    };

    format!(" {} [{}/{}] ", arrow, current_page, total_pages)
}

// =============================================================================
// EMPTY STATE RENDERING
// =============================================================================

/// Types of empty states that can be displayed.
/// Some variants are defined for future use in error handling paths.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // Variants used incrementally as error paths are implemented
pub enum EmptyStateKind {
    /// Neo4j connection failed
    NoConnection,
    /// Database has no node kinds
    NoKinds,
    /// Query returned no results
    NoResults,
    /// Kind has no instances
    NoInstances,
    /// Loading data (with animation)
    Loading,
}

impl EmptyStateKind {
    /// Get the icon for this empty state.
    fn icon(&self) -> &'static str {
        match self {
            EmptyStateKind::NoConnection => "⚠",
            EmptyStateKind::NoKinds => "∅",
            EmptyStateKind::NoResults => "◌",
            EmptyStateKind::NoInstances => "□",
            EmptyStateKind::Loading => "◐",
        }
    }

    /// Get the title for this empty state.
    fn title(&self) -> &'static str {
        match self {
            EmptyStateKind::NoConnection => "Neo4j Not Connected",
            EmptyStateKind::NoKinds => "No Node Kinds Found",
            EmptyStateKind::NoResults => "No Results",
            EmptyStateKind::NoInstances => "No Instances",
            EmptyStateKind::Loading => "Loading…",
        }
    }

    /// Get the description lines for this empty state (zero allocation).
    fn description(&self) -> &'static [&'static str] {
        match self {
            EmptyStateKind::NoConnection => &[
                "Unable to connect to Neo4j",
                "",
                "Try:",
                "  • pnpm infra:up",
                "  • Check NEO4J_URI environment variable",
            ],
            EmptyStateKind::NoKinds => &[
                "The taxonomy tree is empty.",
                "",
                "Run:",
                "  • cargo run -- schema generate",
                "  • cargo run -- db seed",
            ],
            EmptyStateKind::NoResults => &[
                "No nodes match your current filter.",
                "",
                "Try:",
                "  • Remove filters with 'c'",
                "  • Switch modes with 1-4",
            ],
            EmptyStateKind::NoInstances => &[
                "This Kind has no data instances yet.",
                "",
                "Create one with:",
                "  cargo run -- node create --kind=<Kind>",
            ],
            EmptyStateKind::Loading => &["Fetching data from Neo4j…"],
        }
    }

    /// Get the hint text for this empty state.
    fn hint(&self) -> &'static str {
        match self {
            EmptyStateKind::NoConnection => "Press 'r' to retry",
            EmptyStateKind::NoKinds => "Press 'q' to quit",
            EmptyStateKind::NoResults => "Press 'c' to clear filters",
            EmptyStateKind::NoInstances => "Press Esc to go back",
            EmptyStateKind::Loading => "",
        }
    }
}

/// Render an empty state message in a centered box.
fn render_empty_state(f: &mut Frame, area: Rect, kind: EmptyStateKind, tick: u16) {
    // Calculate centered box dimensions
    let box_width = POPUP_BOX_WIDTH.min(area.width.saturating_sub(4));
    let box_height = POPUP_BOX_HEIGHT.min(area.height.saturating_sub(2));

    if box_width < POPUP_BOX_MIN_WIDTH || box_height < POPUP_BOX_MIN_HEIGHT {
        // Area too small for empty state
        return;
    }

    let x = (area.width.saturating_sub(box_width)) / 2 + area.x;
    let y = (area.height.saturating_sub(box_height)) / 2 + area.y;
    let box_area = Rect::new(x, y, box_width, box_height);

    // Build content lines
    let mut lines: Vec<Line> = Vec::new();

    // Title with icon
    let title_icon = kind.icon();
    let title_text = kind.title();

    // Loading spinner animation
    let display_icon = if matches!(kind, EmptyStateKind::Loading) {
        const BRAILLE: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        BRAILLE[(tick / SPINNER_SPEED_DIVISOR as u16) as usize % BRAILLE.len()]
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

    // Description lines
    for desc_line in kind.description() {
        lines.push(Line::from(Span::styled(
            format!("  {}", desc_line),
            STYLE_DESC,
        )));
    }

    // Hint (if any)
    let hint = kind.hint();
    if !hint.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(format!("  {}", hint), STYLE_INFO)));
    }

    // Render block with border
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER))
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);

    // Clear background and render
    f.render_widget(Clear, box_area);
    f.render_widget(paragraph, box_area);
}

/// Safely truncate a UTF-8 string to N terminal columns (not chars).
/// Appends "…" if truncated. Handles CJK, emoji, and combining characters.
fn truncate_str(s: &str, max_width: usize) -> String {
    truncate_to_width(s, max_width)
}

/// Safely truncate a UTF-8 string from the START, keeping last N columns.
/// Prepends "…" if truncated. Used for breadcrumbs where the end is most relevant.
fn truncate_start(s: &str, max_width: usize) -> String {
    truncate_start_to_width(s, max_width)
}

/// Animated spinner for loading states.
/// Cycles through braille patterns for smooth animation.
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Get the current spinner frame based on tick counter.
fn spinner(tick: u16) -> &'static str {
    SPINNER_FRAMES[(tick as usize / SPINNER_SPEED_DIVISOR) % SPINNER_FRAMES.len()]
}

/// Main render function.
pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_main(f, chunks[1], app);
    render_status(f, chunks[2], app);

    // Overlays on top (order matters: last = topmost)
    if app.search.active {
        overlays::render_search(f, app);
    }
    if app.help_active {
        overlays::render_help(f, app);
    }
    if app.legend_active {
        overlays::render_legend(f, app);
    }
    if app.recent_items_active {
        render_recent_items_overlay(f, app);
    }
}

/// Header: Logo + Mode tabs.
/// Shows: [1]Meta, [2]Data, [3]Atlas, [4]Audit, [5]Guide
/// (Overlay and Query modes removed - not useful for now)
fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let tabs: Vec<Span> = [
        NavMode::Meta,
        NavMode::Data,
        NavMode::Atlas,
        NavMode::Audit,
        NavMode::Guide,
    ]
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

    // Show hide_empty indicator when active in Data mode
    if app.hide_empty && app.mode == NavMode::Data {
        header.push(Span::styled(
            " [∅ hidden]",
            Style::default().fg(Color::Yellow),
        ));
    }

    // Context-aware shortcuts
    let right_side = if app.mode == NavMode::Atlas {
        vec![Span::styled(
            "  a-r:views  d:demo  l:locale  ?:help  q:quit",
            theme::ui::muted_style(),
        )]
    } else if app.mode == NavMode::Guide {
        vec![Span::styled(
            "  1-4:tabs  jk:nav  Enter:drill  Esc:back  ?:help  q:quit",
            theme::ui::muted_style(),
        )]
    } else if app.mode == NavMode::Data {
        vec![Span::styled(
            "  h/l:toggle  jk:scroll  0:hide  Tab:panel  /:find  ?:help  q:quit",
            theme::ui::muted_style(),
        )]
    } else {
        vec![Span::styled(
            "  h/l:toggle  jk:scroll  Tab:panel  /:find  ?:help  q:quit",
            theme::ui::muted_style(),
        )]
    };

    let mut full_header: Vec<Span<'static>> = header;
    // Calculate padding to right-align
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

/// Layout mode based on terminal width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayoutMode {
    Wide,   // 3 columns: Tree | Info | YAML
    Narrow, // 2 columns: Tree | (Info / YAML stacked)
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

/// Main content: responsive layout based on terminal width.
fn render_main(f: &mut Frame, area: Rect, app: &mut App) {
    // Atlas mode has its own rendering
    if app.mode == NavMode::Atlas {
        render_atlas(f, area, app);
        return;
    }

    // Audit mode has its own rendering (Feature 6)
    if app.mode == NavMode::Audit {
        render_audit(f, area, app);
        return;
    }

    // Guide mode has its own rendering (Batch 3+)
    if app.mode == NavMode::Guide {
        super::guide::render_guide(f, area, app);
        return;
    }

    let layout_mode = LayoutMode::detect(area.width);

    match layout_mode {
        LayoutMode::Wide => render_main_wide(f, area, app),
        LayoutMode::Narrow => render_main_narrow(f, area, app),
    }
}

/// Wide layout: Tree | Info+Graph | YAML.
fn render_main_wide(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(LAYOUT_WIDE_TREE_PCT),
            Constraint::Percentage(LAYOUT_WIDE_INFO_PCT),
            Constraint::Percentage(LAYOUT_WIDE_YAML_PCT),
        ])
        .split(area);

    render_tree(f, chunks[0], app);

    // Stack Info and Graph vertically in the middle panel
    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(LAYOUT_WIDE_INFO_SECTION_PCT),
            Constraint::Percentage(LAYOUT_WIDE_GRAPH_SECTION_PCT),
        ])
        .split(chunks[1]);

    render_info_panel(f, middle_chunks[0], app);
    render_graph_panel(f, middle_chunks[1], app);

    render_yaml_panel(f, chunks[2], app);
}

/// Narrow layout: Tree | Info+Graph+YAML stacked.
fn render_main_narrow(f: &mut Frame, area: Rect, app: &mut App) {
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(LAYOUT_NARROW_TREE_PCT),
            Constraint::Percentage(LAYOUT_NARROW_DETAIL_PCT),
        ])
        .split(area);

    render_tree(f, h_chunks[0], app);

    // Stack Info, Graph, YAML vertically
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(LAYOUT_NARROW_INFO_PCT),
            Constraint::Percentage(LAYOUT_NARROW_GRAPH_PCT),
            Constraint::Percentage(LAYOUT_NARROW_YAML_PCT),
        ])
        .split(h_chunks[1]);

    render_info_panel(f, v_chunks[0], app);
    render_graph_panel(f, v_chunks[1], app);
    render_yaml_panel(f, v_chunks[2], app);
}

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

/// Tree panel: taxonomy hierarchy with scroll and collapse.
/// Uses box-drawing characters for visual hierarchy.
fn render_tree(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.focus == Focus::Tree;
    let border_color = if focused {
        Color::Cyan
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Calculate visible height (area minus borders)
    let visible_height = area.height.saturating_sub(2) as usize;
    app.tree_height = visible_height;

    // === EMPTY STATE: No node kinds loaded ===
    let total_kinds: usize = app
        .tree
        .realms
        .iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.kinds.len())
        .sum();

    if total_kinds == 0 {
        // Render empty tree panel with border
        let block = Block::default()
            .title(" Taxonomy ")
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
        render_empty_state(f, inner_area, EmptyStateKind::NoKinds, app.tick);
        return;
    }

    // === FILTERED DATA MODE: Show only instances of selected Kind ===
    if let Some(kind_key) = app.get_filter_kind() {
        render_filtered_instances(
            f,
            area,
            app,
            kind_key,
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
    // bg_color: optional background color for the line (e.g., active Kind highlight)
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
            // When focused/selected, use white on highlight bg for entire line
            let style = Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White);
            Line::from(Span::styled(
                format!(
                    "{}{}{}{}{}",
                    cursor_char, tree_prefix, icon, icon_space, text
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
            let mut spans = Vec::with_capacity(6);
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

    // Box-drawing helpers
    let branch = |is_last: bool| if is_last { "└─" } else { "├─" };
    let cont = |parent_is_last: bool| if parent_is_last { "  " } else { "│ " };

    // === KINDS SECTION ===
    let kinds_collapsed = app.tree.is_collapsed("kinds");
    let kinds_icon = if kinds_collapsed { "▶" } else { "▼" };
    let kinds_count: usize = app
        .tree
        .realms
        .iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.kinds.len())
        .sum();
    all_lines.push(make_line(
        idx,
        app.tree_cursor,
        focused,
        "",
        kinds_icon,
        format!("Node Kinds ({})", kinds_count),
        Color::Magenta, // line_color (not used - no prefix)
        Color::Magenta, // text_color
        app.search.matches.get(&idx).map(|v| v.as_slice()),
        None, // bg_color
    ));
    idx += 1;

    let has_arcs = !app.tree.arc_families.is_empty();

    if !kinds_collapsed {
        let realm_count = app.tree.realms.len();
        for (ri, realm) in app.tree.realms.iter().enumerate() {
            let realm_is_last = ri == realm_count - 1 && !has_arcs;
            let realm_key = format!("realm:{}", realm.key);
            let realm_collapsed = app.tree.is_collapsed(&realm_key);
            let realm_icon = if realm_collapsed { "▶" } else { "▼" };

            let realm_color = hex_to_color(&realm.color);
            all_lines.push(make_line(
                idx,
                app.tree_cursor,
                focused,
                branch(realm_is_last),
                realm_icon,
                format!("{} {}", realm.icon, realm.display_name),
                Color::Magenta, // line_color: parent section color
                realm_color,    // text_color
                app.search.matches.get(&idx).map(|v| v.as_slice()),
                None, // bg_color
            ));
            idx += 1;

            if !realm_collapsed {
                let is_data_mode = app.is_data_mode();
                let hide_empty = app.hide_empty && is_data_mode;

                // Filter visible layers (hide empty if hide_empty is true)
                let visible_layers: Vec<_> = realm
                    .layers
                    .iter()
                    .filter(|l| {
                        if hide_empty {
                            l.kinds.iter().map(|k| k.instance_count).sum::<i64>() > 0
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
                        layer.kinds.iter().map(|k| k.instance_count).sum();
                    let layer_is_empty = layer_instance_count == 0;

                    // Show expand icon only if layer has content
                    let layer_icon = if layer_collapsed { "▶" } else { "▼" };

                    // In Data mode: gray out empty layers, show instance count
                    let layer_color = hex_to_color(&layer.color);
                    let (display_name, text_color) = if is_data_mode {
                        let name = format!("{} ({})", layer.display_name, layer_instance_count);
                        let color = if layer_is_empty {
                            COLOR_MUTED_TEXT // Gray for empty layers
                        } else {
                            layer_color
                        };
                        (name, color)
                    } else {
                        (layer.display_name.clone(), layer_color)
                    };

                    let prefix = format!("{}{}", cont(realm_is_last), branch(layer_is_last));
                    all_lines.push(make_line(
                        idx,
                        app.tree_cursor,
                        focused,
                        &prefix,
                        layer_icon,
                        display_name,
                        realm_color, // line_color: parent realm color
                        text_color,  // text_color (grayed if empty in Data mode)
                        app.search.matches.get(&idx).map(|v| v.as_slice()),
                        None, // bg_color
                    ));
                    idx += 1;

                    if !layer_collapsed {
                        // Filter visible kinds (hide empty if hide_empty is true)
                        let visible_kinds: Vec<_> = layer
                            .kinds
                            .iter()
                            .filter(|k| {
                                if hide_empty {
                                    k.instance_count > 0
                                } else {
                                    true
                                }
                            })
                            .collect();
                        let kind_count = visible_kinds.len();

                        for (ki, kind) in visible_kinds.iter().enumerate() {
                            let kind_is_last = ki == kind_count - 1;
                            let kind_key_str = format!("kind:{}", kind.key);
                            let kind_collapsed = app.tree.is_collapsed(&kind_key_str);

                            // Show collapse icon in Data mode if instances exist
                            let kind_icon = if is_data_mode {
                                if let Some(instances) = app.tree.get_instances(&kind.key) {
                                    if !instances.is_empty() {
                                        if kind_collapsed { "▶" } else { "▼" }
                                    } else {
                                        ""
                                    }
                                } else {
                                    ""
                                }
                            } else {
                                ""
                            };

                            // v10.1: Show instance count (always in Data mode)
                            // v10.6: Add trait icon prefix
                            // QW7: Show arc count in Meta mode
                            // Feature 2: Health badges in Data mode
                            let kind_is_empty = kind.instance_count == 0;
                            let icon = trait_icon(&kind.trait_name);
                            let arc_count = kind.arcs.len();
                            let (display_text, kind_text_color) = if is_data_mode {
                                // Build health badge if data present
                                let health_badge = if let Some(percent) = kind.health_percent {
                                    let filled = percent / 10;
                                    let empty = 10 - filled;
                                    let issues = kind.issues_count.unwrap_or(0);
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
                                } else {
                                    String::new()
                                };
                                let text = format!(
                                    "{} {} ({}){}",
                                    icon, kind.display_name, kind.instance_count, health_badge
                                );
                                let color = if kind_is_empty {
                                    COLOR_MUTED_TEXT // Gray for empty kinds
                                } else {
                                    Color::White
                                };
                                (text, color)
                            } else {
                                // Meta mode: show arc count inline
                                let text = if arc_count > 0 {
                                    format!("{} {} ↔{}", icon, kind.display_name, arc_count)
                                } else {
                                    format!("{} {}", icon, kind.display_name)
                                };
                                (text, Color::White)
                            };

                            let prefix = format!(
                                "{}{}{}",
                                cont(realm_is_last),
                                cont(layer_is_last),
                                branch(kind_is_last)
                            );
                            // Highlight Kind if it has expanded instances (active focus)
                            let kind_has_expanded_instances = is_data_mode
                                && !kind_collapsed
                                && app
                                    .tree
                                    .get_instances(&kind.key)
                                    .is_some_and(|i| !i.is_empty());
                            let kind_bg = if kind_has_expanded_instances {
                                Some(COLOR_ACTIVE_KIND_BG)
                            } else {
                                None
                            };

                            all_lines.push(make_line(
                                idx,
                                app.tree_cursor,
                                focused,
                                &prefix,
                                kind_icon,
                                display_text,
                                layer_color,     // line_color: parent layer color
                                kind_text_color, // text_color (grayed if empty)
                                app.search.matches.get(&idx).map(|v| v.as_slice()),
                                kind_bg, // bg_color: highlight if instances expanded
                            ));
                            idx += 1;

                            // In Data mode, show instances under Kind (if not collapsed)
                            if is_data_mode && !kind_collapsed {
                                if let Some(instances) = app.tree.get_instances(&kind.key) {
                                    let inst_count = instances.len();
                                    for (ii, instance) in instances.iter().enumerate() {
                                        let inst_is_last = ii == inst_count - 1;
                                        let is_cursor = idx == app.tree_cursor;

                                        // Check if primary (for Locale kind)
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

                                        let (icon, base_color) = if is_primary {
                                            ("●", Color::Yellow)
                                        } else {
                                            ("○", COLOR_CONNECTED)
                                        };

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
                                        let missing_badge = if instance.missing_required_count > 0 {
                                            format!(" (✗{}!)", instance.missing_required_count)
                                        } else {
                                            String::new()
                                        };

                                        // Arc count badge: [->N <-M] (only if has arcs)
                                        let out_count = instance.outgoing_arcs.len();
                                        let in_count = instance.incoming_arcs.len();
                                        let arc_badge = if out_count > 0 || in_count > 0 {
                                            format!(" [->{}|<-{}]", out_count, in_count)
                                        } else {
                                            String::new()
                                        };

                                        // Completeness bar: [==--] only shown if incomplete
                                        let completeness_badge = if instance.total_properties > 0 {
                                            let filled = instance.filled_properties;
                                            let total = instance.total_properties;
                                            if filled >= total {
                                                // 100% complete - hide badge
                                                String::new()
                                            } else {
                                                let ratio = filled as f32 / total as f32;
                                                let filled_chars = (ratio * 4.0).round() as usize;
                                                let empty_chars = 4 - filled_chars;
                                                format!(
                                                    " [{}{}]",
                                                    "=".repeat(filled_chars),
                                                    "-".repeat(empty_chars)
                                                )
                                            }
                                        } else {
                                            String::new()
                                        };

                                        let tree_prefix = format!(
                                            "{}{}{}{}",
                                            cont(realm_is_last),
                                            cont(layer_is_last),
                                            cont(kind_is_last),
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
    let arcs_icon = if arcs_collapsed { "▶" } else { "▼" };
    let arcs_count: usize = app
        .tree
        .arc_families
        .iter()
        .map(|f| f.arc_kinds.len())
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
    ));
    idx += 1;

    if !arcs_collapsed {
        let family_count = app.tree.arc_families.len();
        for (fi, family) in app.tree.arc_families.iter().enumerate() {
            let family_is_last = fi == family_count - 1;
            let family_key = format!("family:{}", family.key);
            let family_collapsed = app.tree.is_collapsed(&family_key);
            let family_icon = if family_collapsed { "▶" } else { "▼" };

            all_lines.push(make_line(
                idx,
                app.tree_cursor,
                focused,
                branch(family_is_last),
                family_icon,
                format!("{} ({})", family.display_name, family.arc_kinds.len()),
                Color::Yellow,    // line_color: parent section color
                COLOR_ARC_FAMILY, // text_color
                app.search.matches.get(&idx).map(|v| v.as_slice()),
                None, // bg_color
            ));
            idx += 1;

            if !family_collapsed {
                let arc_count = family.arc_kinds.len();
                for (ai, arc_kind) in family.arc_kinds.iter().enumerate() {
                    let arc_is_last = ai == arc_count - 1;
                    let prefix = format!("{}{}", cont(family_is_last), branch(arc_is_last));
                    all_lines.push(make_line(
                        idx,
                        app.tree_cursor,
                        focused,
                        &prefix,
                        "",
                        arc_kind.display_name.clone(),
                        COLOR_ARC_FAMILY, // line_color: parent family color
                        COLOR_DESC_TEXT,  // text_color
                        app.search.matches.get(&idx).map(|v| v.as_slice()),
                        None, // bg_color
                    ));
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

    // Show scroll indicator in title (use mode-aware count for Data view)
    let total = app.current_item_count();
    let title = if total > visible_height {
        format!(
            " Taxonomy [{}-{}/{}] ",
            app.tree_scroll + 1,
            (app.tree_scroll + visible_height).min(total),
            total
        )
    } else {
        " Taxonomy ".to_string()
    };

    // Breadcrumb as bottom title (truncate if too long, UTF-8 safe)
    let breadcrumb = app.current_breadcrumb();
    let max_breadcrumb_len = area.width.saturating_sub(4) as usize;
    let breadcrumb_display = truncate_start(&breadcrumb, max_breadcrumb_len);

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .title_bottom(Line::from(Span::styled(
            format!(" {} ", breadcrumb_display),
            Style::default().fg(COLOR_HINT_TEXT),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);

    // Add scrollbar if content exceeds visible area
    if total > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state =
            ScrollbarState::new(total.saturating_sub(visible_height)).position(app.tree_scroll);

        let scrollbar_area = Rect {
            x: area.x + area.width.saturating_sub(2),
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

/// Render filtered Data mode: only instances of a specific Kind with breadcrumb.
fn render_filtered_instances(
    f: &mut Frame,
    area: Rect,
    app: &App,
    kind_key: &str,
    visible_height: usize,
    focused: bool,
    border_color: Color,
) {
    // Get Kind info for display with full hierarchy
    let kind_info = app.tree.find_kind(kind_key);
    let (realm_display, realm_color, layer_display, layer_color, kind_display) = kind_info
        .map(|(realm, layer, kind)| {
            (
                realm.display_name.clone(),
                hex_to_color(&realm.color),
                layer.display_name.clone(),
                hex_to_color(&layer.color),
                kind.display_name.clone(),
            )
        })
        .unwrap_or_else(|| {
            (
                "Unknown".to_string(),
                Color::White,
                "Unknown".to_string(),
                Color::White,
                kind_key.to_string(),
            )
        });

    // Build lines: breadcrumb + instances
    let mut all_lines: Vec<Line> = Vec::new();

    // Breadcrumb header with full hierarchy: Realm → Layer → Kind
    all_lines.push(Line::from(vec![
        Span::styled("← ", STYLE_DIM),
        Span::styled("Esc", STYLE_HIGHLIGHT),
        Span::styled(" │ ", STYLE_DIM),
        Span::styled(&realm_display, Style::default().fg(realm_color)),
        Span::styled(" → ", STYLE_DIM),
        Span::styled(&layer_display, Style::default().fg(layer_color)),
        Span::styled(" → ", STYLE_DIM),
        Span::styled(&kind_display, STYLE_PRIMARY),
    ]));
    all_lines.push(Line::from(Span::styled(
        "─".repeat(area.width.saturating_sub(2) as usize),
        STYLE_UNFOCUSED,
    )));

    // Get instances and total count
    let instances = app.tree.get_instances(kind_key);
    let instance_count = instances.map(|i| i.len()).unwrap_or(0);
    let total_count = app
        .tree
        .get_instance_total(kind_key)
        .unwrap_or(instance_count);
    let is_truncated = total_count > instance_count;
    let is_loading = app
        .pending_instance_load
        .as_ref()
        .is_some_and(|k| k == kind_key);

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
                "  No instances exist for this Kind",
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

            // Primary locales: filled circle, yellow
            // Secondary locales: empty circle, green
            let (icon, base_color) = if is_primary {
                ("●", Color::Yellow)
            } else {
                ("○", COLOR_CONNECTED)
            };

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

    // Title with Kind name and count + position indicator
    // Format: "Locale (3/203)" when all loaded, "Locale (3/500 of 847)" when truncated
    let title = if instance_count > 0 {
        if is_truncated {
            format!(
                " {} ({}/{} of {}) ",
                kind_display,
                app.tree_cursor + 1,
                instance_count,
                total_count
            )
        } else {
            format!(
                " {} ({}/{}) ",
                kind_display,
                app.tree_cursor + 1,
                instance_count
            )
        }
    } else {
        format!(" {} (0) ", kind_display)
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(layer_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Info panel: displays metadata for selected item with independent scroll.
/// Shows a BarChart when on a Realm item.
fn render_info_panel(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.focus == Focus::Info;
    let border_color = if focused {
        Color::Cyan
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Check if we should show a chart (Realm, Layer, or Kind item)
    let show_bar_chart = matches!(app.current_item(), Some(TreeItem::Realm(_)));
    let show_sparkline = matches!(app.current_item(), Some(TreeItem::Layer(_, _)));
    let show_arc_chart = matches!(app.current_item(), Some(TreeItem::Kind(..)));

    if show_bar_chart && area.height > 12 {
        // Split area: top for text, bottom for bar chart
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Length(8)])
            .split(area);

        // Render text info in top chunk
        render_info_text(f, chunks[0], app, focused, border_color);

        // Render bar chart in bottom chunk
        render_realm_bar_chart(f, chunks[1], app);
    } else if show_sparkline && area.height > 10 {
        // Split area: top for text, bottom for sparkline
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Length(5)])
            .split(area);

        // Render text info in top chunk
        render_info_text(f, chunks[0], app, focused, border_color);

        // Render sparkline in bottom chunk
        render_layer_sparkline(f, chunks[1], app);
    } else if show_arc_chart && area.height > 10 {
        // Split area: top for text, bottom for arc distribution chart
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Length(5)])
            .split(area);

        // Render text info in top chunk
        render_info_text(f, chunks[0], app, focused, border_color);

        // Render arc distribution chart in bottom chunk
        render_kind_arc_chart(f, chunks[1], app);
    } else {
        // Normal text-only info panel
        render_info_text(f, area, app, focused, border_color);
    }
}

/// Render the text portion of the info panel.
fn render_info_text(f: &mut Frame, area: Rect, app: &mut App, focused: bool, border_color: Color) {
    // Build info lines
    let all_lines = build_info_lines(app);

    // Update line count for scroll bounds
    app.info_line_count = all_lines.len();

    // Apply scroll
    let visible_height = area.height.saturating_sub(2) as usize; // Account for borders
    let lines: Vec<Line> = all_lines
        .into_iter()
        .skip(app.info_scroll)
        .take(visible_height)
        .collect();

    // Get title from current item
    let title = get_detail_title(app);

    // Build scroll indicator with directional arrows
    let scroll_hint = scroll_indicator(app.info_scroll, app.info_line_count, visible_height);

    let block = Block::default()
        .title(Span::styled(format!(" {} ", title), STYLE_PRIMARY))
        .title_bottom(Span::styled(scroll_hint, STYLE_DIM))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if focused { Color::Cyan } else { border_color }));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);

    // Add scrollbar if content exceeds visible area
    if app.info_line_count > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state =
            ScrollbarState::new(app.info_line_count.saturating_sub(visible_height))
                .position(app.info_scroll);

        let scrollbar_area = Rect {
            x: area.x + area.width.saturating_sub(2),
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

/// Render a bar chart showing kinds per layer for the selected Realm.
fn render_realm_bar_chart(f: &mut Frame, area: Rect, app: &App) {
    let Some(TreeItem::Realm(realm)) = app.current_item() else {
        return;
    };

    // Build bar data from layers
    let bars: Vec<Bar> = realm
        .layers
        .iter()
        .map(|layer| {
            let count = layer.kinds.len() as u64;
            // Use first 4 chars of layer name as label (Unicode-safe)
            let label: String = layer.display_name.chars().take(4).collect();
            Bar::default()
                .value(count)
                .label(Line::from(label))
                .style(Style::default().fg(hex_to_color(&layer.color)))
        })
        .collect();

    if bars.is_empty() {
        return;
    }

    let bar_group = BarGroup::default().bars(&bars);

    let chart = BarChart::default()
        .block(
            Block::default()
                .title(Span::styled(" Kinds/Layer ", STYLE_DIM))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
        )
        .data(bar_group)
        .bar_width(5)
        .bar_gap(1)
        .value_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::Gray));

    f.render_widget(chart, area);
}

/// Render a sparkline showing instance counts per kind for the selected Layer.
fn render_layer_sparkline(f: &mut Frame, area: Rect, app: &App) {
    let Some(TreeItem::Layer(_, layer)) = app.current_item() else {
        return;
    };

    // Collect instance counts from kinds
    let data: Vec<u64> = layer
        .kinds
        .iter()
        .map(|k| k.instance_count.max(0) as u64)
        .collect();

    if data.is_empty() {
        return;
    }

    // Calculate max for label
    let max_val = *data.iter().max().unwrap_or(&0);
    let total: u64 = data.iter().sum();

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(Span::styled(
                    format!(" Instances ({} total, max {}) ", total, max_val),
                    STYLE_DIM,
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
        )
        .data(&data)
        .style(Style::default().fg(hex_to_color(&layer.color)));

    f.render_widget(sparkline, area);
}

/// Render a bar chart showing incoming vs outgoing arc distribution for the selected Kind.
fn render_kind_arc_chart(f: &mut Frame, area: Rect, app: &App) {
    let Some(TreeItem::Kind(_, _, kind)) = app.current_item() else {
        return;
    };

    // Count incoming and outgoing arcs from kind definition
    use super::data::ArcDirection;
    let incoming: usize = kind
        .arcs
        .iter()
        .filter(|a| a.direction == ArcDirection::Incoming)
        .count();
    let outgoing: usize = kind
        .arcs
        .iter()
        .filter(|a| a.direction == ArcDirection::Outgoing)
        .count();

    if incoming == 0 && outgoing == 0 {
        // No arcs, show placeholder
        let block = Block::default()
            .title(Span::styled(" Arc Distribution ", STYLE_DIM))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));
        let paragraph = Paragraph::new(Span::styled("  No arcs defined", STYLE_MUTED)).block(block);
        f.render_widget(paragraph, area);
        return;
    }

    // Build bar data
    let bars = vec![
        Bar::default()
            .value(incoming as u64)
            .label(Line::from("← In"))
            .style(Style::default().fg(Color::Green)),
        Bar::default()
            .value(outgoing as u64)
            .label(Line::from("Out →"))
            .style(Style::default().fg(Color::Cyan)),
    ];

    let chart = BarChart::default()
        .block(
            Block::default()
                .title(Span::styled(
                    format!(" Arc Distribution ({} total) ", incoming + outgoing),
                    STYLE_DIM,
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
        )
        .data(BarGroup::default().bars(&bars))
        .bar_width(8)
        .bar_gap(2)
        .direction(Direction::Vertical);

    f.render_widget(chart, area);
}

/// Get title for detail panel based on current selection.
/// Uses [K] badge for Kind and [I] badge for Instance for instant recognition.
fn get_detail_title(app: &App) -> String {
    match app.current_item() {
        Some(TreeItem::KindsSection) => "Node Kinds".to_string(),
        Some(TreeItem::ArcsSection) => "Arcs".to_string(),
        Some(TreeItem::Realm(r)) => format!("{} {}", r.icon, r.display_name),
        Some(TreeItem::Layer(_, l)) => l.display_name.clone(),
        Some(TreeItem::Kind(_, _, k)) => {
            // [K] badge for Kind - instant recognition
            if k.icon.is_empty() {
                format!("[K] {}", k.display_name)
            } else {
                format!("[K] {} {}", k.icon, k.display_name)
            }
        }
        Some(TreeItem::ArcFamily(f)) => f.display_name.clone(),
        Some(TreeItem::ArcKind(_, ek)) => ek.display_name.clone(),
        Some(TreeItem::Instance(_, _, _, inst)) => {
            // [I] badge for Instance - instant recognition
            format!("[I] {} ({})", inst.key, inst.kind_key)
        }
        None => "Detail".to_string(),
    }
}

/// Colorize path inline for title.
pub(super) fn colorize_path_inline(path: &str) -> Vec<Span<'static>> {
    let parts: Vec<&str> = path.split('/').collect();
    let mut spans: Vec<Span<'static>> = Vec::new();

    for (i, part) in parts.iter().enumerate() {
        let color = match i {
            0..=2 => Color::Rgb(80, 80, 90), // packages/core/models
            3 => Color::Magenta,             // nodes
            4 => match *part {
                // realm (v10.6: 2 realms - global + tenant)
                "global" => Color::Green,
                "tenant" => Color::Yellow,
                _ => Color::White,
            },
            5 => COLOR_CONNECTED, // layer
            _ => Color::White,    // filename
        };
        spans.push(Span::styled(part.to_string(), Style::default().fg(color)));
        if i < parts.len() - 1 {
            spans.push(Span::styled(
                "/",
                Style::default().fg(Color::Rgb(50, 50, 60)),
            ));
        }
    }
    spans
}

/// Build info lines for detail panel.
fn build_info_lines(app: &App) -> Vec<Line<'static>> {
    // Use mode-aware item lookup (shows instances in Data mode)
    match app.current_item() {
        Some(TreeItem::KindsSection) => {
            let theme = &app.theme;
            let kind_count: usize = app
                .tree
                .realms
                .iter()
                .flat_map(|r| r.layers.iter())
                .map(|l| l.kinds.len())
                .sum();

            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Section", STYLE_ACCENT),
                ]),
                Line::from(vec![
                    Span::styled("realms    ", STYLE_DIM),
                    Span::styled(app.tree.realms.len().to_string(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", STYLE_DIM),
                    Span::styled(kind_count.to_string(), STYLE_PRIMARY),
                ]),
                Line::from(""),
            ];

            // Add realm distribution breakdown
            if kind_count > 0 {
                lines.push(Line::from(Span::styled(
                    "REALM DISTRIBUTION",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                let bar_width = 16usize;
                for realm in &app.tree.realms {
                    let realm_kinds: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
                    let percent = (realm_kinds as f64 / kind_count as f64 * 100.0).round() as u8;
                    let filled = (realm_kinds * bar_width) / kind_count.max(1);
                    let bar = "█".repeat(filled.max(1));
                    let empty = "░".repeat(bar_width.saturating_sub(filled));

                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{:8} ", realm.display_name),
                            Style::default().fg(theme.realm_color(&realm.key)),
                        ),
                        Span::styled(bar, Style::default().fg(theme.realm_color(&realm.key))),
                        Span::styled(empty, STYLE_DIM),
                        Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
                        Span::styled(format!("  {} Kinds", realm_kinds), STYLE_DIM),
                    ]));
                }
                lines.push(Line::from(""));
            }

            lines.push(Line::from(Span::styled(
                "h/l to collapse/expand",
                STYLE_DIM,
            )));
            lines
        }
        Some(TreeItem::ArcsSection) => {
            let arc_count: usize = app
                .tree
                .arc_families
                .iter()
                .map(|f| f.arc_kinds.len())
                .sum();
            vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Section", STYLE_HIGHLIGHT),
                ]),
                Line::from(vec![
                    Span::styled("families  ", STYLE_DIM),
                    Span::styled(app.tree.arc_families.len().to_string(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("arcs      ", STYLE_DIM),
                    Span::styled(arc_count.to_string(), STYLE_PRIMARY),
                ]),
                Line::from(""),
                Line::from(Span::styled("h/l to collapse/expand", STYLE_DIM)),
            ]
        }
        Some(TreeItem::Realm(realm)) => {
            let theme = &app.theme;
            let kind_count: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Realm", STYLE_ACCENT),
                ]),
                Line::from(vec![
                    Span::styled("key       ", STYLE_DIM),
                    Span::styled(realm.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("layers    ", STYLE_DIM),
                    Span::styled(realm.layers.len().to_string(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", STYLE_DIM),
                    Span::styled(kind_count.to_string(), STYLE_PRIMARY),
                ]),
            ];

            // Add layer breakdown if there are layers with kinds
            if kind_count > 0 {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "LAYER BREAKDOWN",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                let bar_width = 12usize;
                for layer in &realm.layers {
                    let count = layer.kinds.len();
                    if count == 0 {
                        continue;
                    }
                    let percent = (count as f64 / kind_count as f64 * 100.0).round() as u8;
                    let filled = (count * bar_width) / kind_count.max(1);
                    let bar = "█".repeat(filled.max(1));
                    let empty = "░".repeat(bar_width.saturating_sub(filled));
                    let layer_color = theme.layer_color(&layer.key);

                    lines.push(Line::from(vec![
                        Span::styled("  ", Style::default().fg(layer_color)),
                        Span::styled(
                            format!("{:16} ", layer.display_name),
                            Style::default().fg(layer_color),
                        ),
                        Span::styled(bar, Style::default().fg(layer_color)),
                        Span::styled(empty, STYLE_DIM),
                        Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
                        Span::styled(format!("  {}", count), STYLE_DIM),
                    ]));
                }
            }

            lines
        }
        Some(TreeItem::Layer(realm, layer)) => {
            let theme = &app.theme;
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Layer", STYLE_SUCCESS),
                ]),
                Line::from(vec![
                    Span::styled("key       ", STYLE_DIM),
                    Span::styled(layer.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("realm     ", STYLE_DIM),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(hex_to_color(&realm.color)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", STYLE_DIM),
                    Span::styled(layer.kinds.len().to_string(), STYLE_PRIMARY),
                ]),
            ];

            // Add trait breakdown if there are kinds
            if !layer.kinds.is_empty() {
                // Count kinds by trait
                let mut trait_counts: std::collections::BTreeMap<String, usize> =
                    std::collections::BTreeMap::new();
                for kind in &layer.kinds {
                    *trait_counts.entry(kind.trait_name.clone()).or_insert(0) += 1;
                }

                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "TRAIT BREAKDOWN",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                let total = layer.kinds.len();
                let bar_width = 12usize;
                for (trait_name, count) in &trait_counts {
                    let percent = (*count as f64 / total as f64 * 100.0).round() as u8;
                    let filled = (*count * bar_width) / total.max(1);
                    let bar = "█".repeat(filled.max(1));
                    let empty = "░".repeat(bar_width.saturating_sub(filled));
                    let icon = trait_icon(trait_name);

                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{} ", icon),
                            Style::default().fg(theme.trait_color(trait_name)),
                        ),
                        Span::styled(
                            format!("{:12} ", trait_name),
                            Style::default().fg(theme.trait_color(trait_name)),
                        ),
                        Span::styled(bar, Style::default().fg(theme.trait_color(trait_name))),
                        Span::styled(empty, STYLE_DIM),
                        Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
                        Span::styled(format!("  {}", count), STYLE_DIM),
                    ]));
                }
            }

            lines
        }
        Some(TreeItem::Kind(realm, layer, kind)) => {
            let theme = &app.theme;

            // Unified header: type, key, kind, realm, layer, trait (12-char labels)
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type        ", STYLE_DIM),
                    Span::styled("Node Kind", STYLE_INFO),
                ]),
                Line::from(vec![
                    Span::styled("key         ", STYLE_DIM),
                    Span::styled(kind.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("kind        ", STYLE_DIM),
                    Span::styled("—", STYLE_DIM),
                ]),
                Line::from(vec![
                    Span::styled("realm       ", STYLE_DIM),
                    Span::styled(format!("{} ", realm.icon), STYLE_DIM),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(hex_to_color(&realm.color)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("layer       ", STYLE_DIM),
                    Span::styled(format!("{} ", theme.icons.layer(&layer.key)), STYLE_DIM),
                    Span::styled(
                        layer.display_name.clone(),
                        Style::default().fg(hex_to_color(&layer.color)),
                    ),
                ]),
            ];

            // Trait with icon (if present)
            if !kind.trait_name.is_empty() {
                let trait_icon = theme.icons.trait_icon(&kind.trait_name);
                lines.push(Line::from(vec![
                    Span::styled("trait       ", STYLE_DIM),
                    Span::styled(format!("{} ", trait_icon), STYLE_DIM),
                    Span::styled(
                        kind.trait_name.clone(),
                        Style::default().fg(theme.trait_color(&kind.trait_name)),
                    ),
                ]));
            }

            // v10.1: knowledge_tier removed from display (node type is sufficient)

            // Instances count (aligned with Instance view)
            let instance_count = kind.instance_count;
            lines.push(Line::from(vec![
                Span::styled("instances   ", STYLE_DIM),
                Span::styled(format!("{} total", instance_count), STYLE_MUTED),
            ]));

            // Blank line before stats section
            lines.push(Line::from(""));

            // Properties line (aligned with Instance view)
            let total_props = kind.properties.len();

            // Format: "properties  8 defined ████░░░░"
            let bar_width = 10usize;
            let log_val = if instance_count > 0 {
                (instance_count as f64).log10().max(0.0)
            } else {
                0.0
            };
            let filled = ((log_val / 4.0) * bar_width as f64).round() as usize;
            let filled = filled.clamp(if instance_count > 0 { 1 } else { 0 }, bar_width);
            let bar = "━".repeat(filled);
            let empty = "░".repeat(bar_width.saturating_sub(filled));

            lines.push(Line::from(vec![
                Span::styled("properties  ", STYLE_DIM),
                Span::styled(format!("{} defined", total_props), STYLE_INFO),
                Span::styled(" ", STYLE_DIM),
                Span::styled(bar, STYLE_SUCCESS),
                Span::styled(empty, STYLE_DIM),
            ]));

            // Context budget (if present)
            if !kind.context_budget.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("budget      ", STYLE_DIM),
                    Span::styled(kind.context_budget.clone(), STYLE_INFO),
                ]));
            }

            // Property coverage summary
            let total_props = kind.properties.len();
            let required_props = kind.required_properties.len();
            let optional_props = total_props.saturating_sub(required_props);

            if total_props > 0 {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "PROPERTY COVERAGE",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                let bar_width = 12usize;
                // Required bar
                let req_percent =
                    (required_props as f64 / total_props as f64 * 100.0).round() as u8;
                let req_filled = (required_props * bar_width) / total_props.max(1);
                let req_bar = "█".repeat(req_filled.max(if required_props > 0 { 1 } else { 0 }));
                let req_empty = "░".repeat(bar_width.saturating_sub(req_filled));

                lines.push(Line::from(vec![
                    Span::styled("* ", Style::default().fg(Color::Red)),
                    Span::styled("required     ", Style::default().fg(Color::Yellow)),
                    Span::styled(req_bar, Style::default().fg(Color::Yellow)),
                    Span::styled(req_empty, STYLE_DIM),
                    Span::styled(format!(" {:>3}%", req_percent), STYLE_MUTED),
                    Span::styled(format!("  {}", required_props), STYLE_DIM),
                ]));

                // Optional bar
                let opt_percent =
                    (optional_props as f64 / total_props as f64 * 100.0).round() as u8;
                let opt_filled = (optional_props * bar_width) / total_props.max(1);
                let opt_bar = "█".repeat(opt_filled.max(if optional_props > 0 { 1 } else { 0 }));
                let opt_empty = "░".repeat(bar_width.saturating_sub(opt_filled));

                lines.push(Line::from(vec![
                    Span::styled("  ", STYLE_DIM),
                    Span::styled("optional     ", Style::default().fg(Color::White)),
                    Span::styled(opt_bar, Style::default().fg(Color::White)),
                    Span::styled(opt_empty, STYLE_DIM),
                    Span::styled(format!(" {:>3}%", opt_percent), STYLE_MUTED),
                    Span::styled(format!("  {}", optional_props), STYLE_DIM),
                ]));
            }

            // Properties section with validation (Neo4j ↔ YAML)
            // If validated properties available, show with validation status
            // Otherwise fall back to simple property list
            if let Some(validated) = &app.validated_kind_properties {
                lines.push(Line::from(""));

                // Header with validation stats
                if let Some(stats) = &app.validation_stats {
                    lines.push(Line::from(vec![
                        Span::styled(format!("Properties ({}) ", validated.len()), STYLE_MUTED),
                        Span::styled(format!("✓{}", stats.sync_count), STYLE_SUCCESS),
                        Span::styled(" ", STYLE_DIM),
                        if stats.missing_count > 0 {
                            Span::styled(format!("⚠{}", stats.missing_count), STYLE_WARNING)
                        } else {
                            Span::styled("", STYLE_DIM)
                        },
                        Span::styled(" ", STYLE_DIM),
                        if stats.extra_count > 0 {
                            Span::styled(format!("?{}", stats.extra_count), STYLE_DIM)
                        } else {
                            Span::styled("", STYLE_DIM)
                        },
                    ]));
                } else {
                    lines.push(Line::from(Span::styled(
                        format!("Properties ({})", validated.len()),
                        STYLE_MUTED,
                    )));
                }

                // Render each validated property
                for prop in validated {
                    let (status_icon, status_style) = match prop.status {
                        ValidationStatus::Sync => ("✓", STYLE_SUCCESS),
                        ValidationStatus::Missing => ("⚠", STYLE_WARNING),
                        ValidationStatus::Extra => ("?", STYLE_DIM),
                    };

                    let required_marker = if prop.required { "*" } else { " " };
                    let type_badge = type_badge(&prop.prop_type);

                    // Example value (if available)
                    let example_str = prop
                        .example
                        .as_ref()
                        .map(|e| format!("→ {}", truncate_str(e, 25)))
                        .unwrap_or_default();

                    lines.push(Line::from(vec![
                        Span::styled(status_icon, status_style),
                        Span::styled(
                            required_marker,
                            Style::default().fg(Color::Rgb(255, 100, 100)),
                        ),
                        Span::styled(format!("[{:4}] ", type_badge), STYLE_DIM),
                        Span::styled(format!("{:<15}", prop.name), STYLE_INFO),
                        Span::styled(example_str, STYLE_MUTED),
                    ]));
                }

                // Legend
                lines.push(Line::from(vec![
                    Span::styled("  ✓=sync ⚠=missing ?=extra  ", STYLE_DIM),
                    Span::styled("*=required", STYLE_DIM),
                ]));
            } else if !kind.properties.is_empty() {
                // Fallback: simple property list (no YAML loaded)
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Properties ({})", kind.properties.len()),
                    STYLE_MUTED,
                )));

                for prop in &kind.properties {
                    let is_required = kind.required_properties.contains(prop);
                    let marker = if is_required { "*" } else { " " };
                    let prop_color = if is_required {
                        Color::Yellow
                    } else {
                        Color::White
                    };

                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("  {}", marker),
                            Style::default().fg(Color::Rgb(255, 100, 100)),
                        ),
                        Span::styled(prop.clone(), Style::default().fg(prop_color)),
                    ]));
                }

                // Legend
                lines.push(Line::from(Span::styled("  * = required", STYLE_DIM)));
            }

            // Arcs section
            if !kind.arcs.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Arcs ({})", kind.arcs.len()),
                    STYLE_MUTED,
                )));

                for arc in &kind.arcs {
                    let (arrow, arrow_color) = match arc.direction {
                        ArcDirection::Outgoing => ("→", Color::Cyan),
                        ArcDirection::Incoming => ("←", Color::Magenta),
                    };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", arrow), Style::default().fg(arrow_color)),
                        Span::styled(arc.rel_type.clone(), Style::default().fg(arrow_color)),
                        Span::styled(" → ", STYLE_DIM),
                        Span::styled(arc.target_kind.clone(), STYLE_HIGHLIGHT),
                    ]));
                }
            }

            // Description
            if !kind.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("Description", STYLE_MUTED)));
                // Wrap description to multiple lines (no Vec<char> allocation)
                for line in wrap_text(&kind.description, 60) {
                    lines.push(Line::from(Span::styled(format!("  {}", line), STYLE_DESC)));
                }
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Cypher", STYLE_MUTED)));
            lines.push(Line::from(Span::styled(
                format!("  MATCH (n:{}) RETURN n LIMIT 100", kind.key),
                STYLE_HINT,
            )));

            lines
        }
        Some(TreeItem::ArcFamily(family)) => {
            vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("ArcFamily", STYLE_ARC_FAMILY),
                ]),
                Line::from(vec![
                    Span::styled("key       ", STYLE_DIM),
                    Span::styled(family.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("arcs      ", STYLE_DIM),
                    Span::styled(family.arc_kinds.len().to_string(), STYLE_PRIMARY),
                ]),
                Line::from(""),
                Line::from(Span::styled("h/l to collapse/expand", STYLE_DIM)),
            ]
        }
        Some(TreeItem::ArcKind(family, arc_kind)) => {
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("ArcKind", STYLE_HIGHLIGHT),
                ]),
                Line::from(vec![
                    Span::styled("key       ", STYLE_DIM),
                    Span::styled(arc_kind.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("family    ", STYLE_DIM),
                    Span::styled(family.display_name.clone(), STYLE_ARC_FAMILY),
                ]),
                Line::from(vec![
                    Span::styled("from      ", STYLE_DIM),
                    Span::styled(arc_kind.from_kind.clone(), STYLE_INFO),
                ]),
                Line::from(vec![
                    Span::styled("to        ", STYLE_DIM),
                    Span::styled(arc_kind.to_kind.clone(), STYLE_INFO),
                ]),
            ];

            // Cardinality (if present)
            if !arc_kind.cardinality.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("cardin.   ", STYLE_DIM),
                    Span::styled(arc_kind.cardinality.clone(), STYLE_ACCENT),
                ]));
            }

            // Description (if present)
            if !arc_kind.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("Description", STYLE_MUTED)));
                lines.push(Line::from(Span::styled(
                    format!("  {}", &arc_kind.description),
                    STYLE_DESC,
                )));
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Cypher", STYLE_MUTED)));
            lines.push(Line::from(Span::styled(
                format!("  MATCH ()-[r:{}]->() RETURN r LIMIT 100", arc_kind.key),
                STYLE_HINT,
            )));

            lines
        }
        Some(TreeItem::Instance(realm, layer, kind, instance)) => {
            // Instance info for Data view
            // Unified header: type, key, kind, realm, layer, trait (12-char labels + icons)
            let theme = &app.theme;

            // Header - matches Kind view structure for easy comparison
            let mut lines: Vec<Line<'static>> = vec![
                Line::from(vec![
                    Span::styled("type        ", STYLE_DIM),
                    Span::styled("Instance", STYLE_SUCCESS),
                ]),
                Line::from(vec![
                    Span::styled("key         ", STYLE_DIM),
                    Span::styled(instance.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("kind        ", STYLE_DIM),
                    Span::styled(kind.display_name.clone(), STYLE_INFO),
                ]),
                Line::from(vec![
                    Span::styled("realm       ", STYLE_DIM),
                    Span::styled(format!("{} ", realm.icon), STYLE_DIM),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(hex_to_color(&realm.color)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("layer       ", STYLE_DIM),
                    Span::styled(format!("{} ", theme.icons.layer(&layer.key)), STYLE_DIM),
                    Span::styled(
                        layer.display_name.clone(),
                        Style::default().fg(hex_to_color(&layer.color)),
                    ),
                ]),
            ];
            // Trait with icon
            if !kind.trait_name.is_empty() {
                let trait_icon = theme.icons.trait_icon(&kind.trait_name);
                lines.push(Line::from(vec![
                    Span::styled("trait       ", STYLE_DIM),
                    Span::styled(format!("{} ", trait_icon), STYLE_DIM),
                    Span::styled(
                        kind.trait_name.clone(),
                        Style::default().fg(theme.trait_color(&kind.trait_name)),
                    ),
                ]));
            }

            // Instances count (aligned with Kind's "properties" line context)
            if kind.instance_count > 0 {
                lines.push(Line::from(vec![
                    Span::styled("instances   ", STYLE_DIM),
                    Span::styled(format!("{} total", kind.instance_count), STYLE_MUTED),
                ]));
            }

            // Properties with optional Schema Overlay
            // If schema overlay is enabled and we have matched properties, show schema view
            // Otherwise, fall back to simple property list
            if app.schema_overlay_enabled {
                if let Some(matched) = &app.matched_properties {
                    // Schema overlay: show all schema properties with status
                    let stats = app.coverage_stats.as_ref();
                    let (filled, total) = stats.map(|s| (s.filled, s.total)).unwrap_or((
                        matched
                            .iter()
                            .filter(|p| p.status == PropertyStatus::Filled)
                            .count(),
                        matched.len(),
                    ));
                    let percent = if total > 0 {
                        (filled * 100) / total
                    } else {
                        100
                    };

                    lines.push(Line::from(""));

                    // Properties header (aligned with Kind view)
                    // Format: "properties  14/14 filled ━━━━━━━━━━ 100%"
                    let bar_width = 10usize;
                    let progress_filled = (percent * bar_width) / 100;
                    let progress_empty = bar_width.saturating_sub(progress_filled);
                    lines.push(Line::from(vec![
                        Span::styled("properties  ", STYLE_DIM),
                        Span::styled(format!("{}/{} filled", filled, total), STYLE_INFO),
                        Span::styled(" ", STYLE_DIM),
                        Span::styled("━".repeat(progress_filled), STYLE_SUCCESS),
                        Span::styled("░".repeat(progress_empty), STYLE_DIM),
                        Span::styled(format!(" {}%", percent), STYLE_MUTED),
                    ]));

                    // Status line (aligned with Kind's "budget" line)
                    let missing_required = matched
                        .iter()
                        .filter(|p| p.schema.required && p.status != PropertyStatus::Filled)
                        .count();
                    let (status_text, status_style) = if missing_required > 0 {
                        (
                            format!("missing {} required", missing_required),
                            STYLE_ERROR,
                        )
                    } else if percent == 100 {
                        ("complete".to_string(), STYLE_SUCCESS)
                    } else {
                        ("partial".to_string(), STYLE_INFO)
                    };
                    lines.push(Line::from(vec![
                        Span::styled("status      ", STYLE_DIM),
                        Span::styled(status_text, status_style),
                    ]));

                    // PROPERTY COVERAGE section (aligned with Kind view)
                    let required_count = matched.iter().filter(|p| p.schema.required).count();
                    let optional_count = matched.len().saturating_sub(required_count);
                    let required_filled = matched
                        .iter()
                        .filter(|p| p.schema.required && p.status == PropertyStatus::Filled)
                        .count();
                    let optional_filled = matched
                        .iter()
                        .filter(|p| !p.schema.required && p.status == PropertyStatus::Filled)
                        .count();

                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        "PROPERTY COVERAGE",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )));
                    lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                    // Required bar
                    let req_percent = if required_count > 0 {
                        (required_filled * 100) / required_count
                    } else {
                        100
                    };
                    let req_bar_filled = (req_percent * bar_width) / 100;
                    let req_bar_filled =
                        req_bar_filled.max(if required_filled > 0 { 1 } else { 0 });
                    lines.push(Line::from(vec![
                        Span::styled("* ", Style::default().fg(Color::Red)),
                        Span::styled("required     ", Style::default().fg(Color::Yellow)),
                        Span::styled(
                            "█".repeat(req_bar_filled),
                            Style::default().fg(Color::Yellow),
                        ),
                        Span::styled(
                            "░".repeat(bar_width.saturating_sub(req_bar_filled)),
                            STYLE_DIM,
                        ),
                        Span::styled(format!(" {:>3}%", req_percent), STYLE_MUTED),
                        Span::styled(
                            format!("  {}/{}", required_filled, required_count),
                            STYLE_DIM,
                        ),
                    ]));

                    // Optional bar
                    let opt_percent = if optional_count > 0 {
                        (optional_filled * 100) / optional_count
                    } else {
                        100
                    };
                    let opt_bar_filled = (opt_percent * bar_width) / 100;
                    let opt_bar_filled =
                        opt_bar_filled.max(if optional_filled > 0 { 1 } else { 0 });
                    lines.push(Line::from(vec![
                        Span::styled("  ", Style::default()),
                        Span::styled("optional     ", Style::default().fg(Color::Gray)),
                        Span::styled("█".repeat(opt_bar_filled), Style::default().fg(Color::Gray)),
                        Span::styled(
                            "░".repeat(bar_width.saturating_sub(opt_bar_filled)),
                            STYLE_DIM,
                        ),
                        Span::styled(format!(" {:>3}%", opt_percent), STYLE_MUTED),
                        Span::styled(
                            format!("  {}/{}", optional_filled, optional_count),
                            STYLE_DIM,
                        ),
                    ]));

                    // Properties list header
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        format!("Properties ({}) ✓{}", matched.len(), filled),
                        STYLE_MUTED,
                    )));

                    // Show each property with status
                    // Feature 3: Track focused property index for intelligent truncation
                    // Feature 6: Type badges [str], [json], [enum], etc.
                    for (prop_idx, prop) in matched.iter().enumerate() {
                        let is_required = prop.schema.required;
                        let prefix = if is_required { "*" } else { " " };
                        let badge = type_badge(&prop.schema.prop_type);
                        // Feature 3: Show full value when property is focused
                        let is_focused = prop_idx == app.focused_property_idx;
                        let truncate_limit = if is_focused { 200 } else { 40 };

                        match prop.status {
                            PropertyStatus::Filled => {
                                // Has value: show normally with type badge
                                let value_str = prop
                                    .value
                                    .as_ref()
                                    .map(|v| {
                                        if app.json_pretty
                                            && (v.starts_with('{') || v.starts_with('['))
                                        {
                                            // Pretty-print JSON
                                            serde_json::from_str::<serde_json::Value>(v)
                                                .ok()
                                                .and_then(|j| serde_json::to_string_pretty(&j).ok())
                                                .unwrap_or_else(|| v.clone())
                                        } else {
                                            v.clone()
                                        }
                                    })
                                    .unwrap_or_default();
                                // Feature 3: Highlight focused property row
                                let name_style = if is_focused {
                                    STYLE_HIGHLIGHT
                                } else {
                                    STYLE_INFO
                                };

                                // Feature 3b: Expand text with Enter toggle
                                if is_focused && app.expanded_property {
                                    // Expanded: show full value with word-wrap
                                    // First line with property name and expand indicator
                                    lines.push(Line::from(vec![
                                        Span::styled(
                                            format!("{}[{:4}] ", prefix, badge),
                                            STYLE_DIM,
                                        ),
                                        Span::styled(
                                            format!("{:<15}", prop.schema.name),
                                            name_style,
                                        ),
                                        Span::styled("▼ ", STYLE_HINT), // Expanded indicator
                                    ]));
                                    // Wrap value text to multiple lines (no Vec<char> allocation)
                                    let full_value = format!("\"{}\"", value_str);
                                    for line in wrap_text(&full_value, 50) {
                                        lines.push(Line::from(vec![
                                            Span::styled("                        ", STYLE_DIM), // Indent
                                            Span::styled(line, STYLE_SUCCESS),
                                        ]));
                                    }
                                } else {
                                    // Collapsed: truncate as before
                                    let truncated =
                                        truncate_str(&format!("\"{}\"", value_str), truncate_limit);
                                    let indicator = if is_focused { "▶ " } else { "" };
                                    lines.push(Line::from(vec![
                                        Span::styled(
                                            format!("{}[{:4}] ", prefix, badge),
                                            STYLE_DIM,
                                        ),
                                        Span::styled(
                                            format!("{:<15}", prop.schema.name),
                                            name_style,
                                        ),
                                        Span::styled(indicator, STYLE_HINT),
                                        Span::styled(truncated, STYLE_SUCCESS),
                                    ]));
                                }
                            }
                            PropertyStatus::EmptyOptional => {
                                // Optional, empty: dim with type badge + example
                                let hint = format!(
                                    "— e.g. {}",
                                    prop.schema.example.as_deref().unwrap_or("...")
                                );
                                lines.push(Line::from(vec![
                                    Span::styled(format!("{}[{:4}] ", prefix, badge), STYLE_DIM),
                                    Span::styled(format!("{:<15}", prop.schema.name), STYLE_DIM),
                                    Span::styled(truncate_str(&hint, 40), STYLE_DIM),
                                ]));
                            }
                            PropertyStatus::MissingRequired => {
                                // Required, missing: red warning with type badge + example
                                let hint = format!(
                                    "⚠ e.g. {}",
                                    prop.schema.example.as_deref().unwrap_or("...")
                                );
                                lines.push(Line::from(vec![
                                    Span::styled(format!("{}[{:4}] ", prefix, badge), STYLE_ERROR),
                                    Span::styled(format!("{:<15}", prop.schema.name), STYLE_ERROR),
                                    Span::styled(truncate_str(&hint, 40), STYLE_ERROR),
                                ]));
                            }
                        }
                    }
                } else {
                    // Schema overlay enabled but no matched properties loaded yet
                    // Fall back to simple display
                    render_simple_properties(&mut lines, &instance.properties);
                }
            } else {
                // Schema overlay disabled: simple property list with fill rate header
                let total_schema_props = kind.properties.len();
                let filled_props = instance.properties.len();

                if total_schema_props > 0 && filled_props > 0 {
                    let fill_percent = ((filled_props as f64 / total_schema_props as f64) * 100.0)
                        .round()
                        .min(100.0) as usize;
                    let bar_width = 10usize;
                    let filled = (fill_percent * bar_width) / 100;
                    let bar = "━".repeat(filled.max(1));
                    let empty = "░".repeat(bar_width.saturating_sub(filled));

                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("Properties ({}/{}) ", filled_props, total_schema_props),
                            STYLE_MUTED,
                        ),
                        Span::styled(bar, STYLE_SUCCESS),
                        Span::styled(empty, STYLE_DIM),
                        Span::styled(format!(" {}%", fill_percent), STYLE_MUTED),
                    ]));

                    // Show properties in YAML definition order (kind.properties preserves order)
                    for prop_name in &kind.properties {
                        if prop_name.starts_with('_')
                            || prop_name == "key"
                            || prop_name == "display_name"
                        {
                            continue;
                        }
                        if let Some(value) = instance.properties.get(prop_name) {
                            let value_str = json_value_to_display(value);
                            let truncated = truncate_str(&value_str, 45);
                            lines.push(Line::from(vec![
                                Span::styled(format!("{:<20}", prop_name), STYLE_INFO),
                                Span::styled(truncated, STYLE_PRIMARY),
                            ]));
                        }
                    }
                } else {
                    render_simple_properties(&mut lines, &instance.properties);
                }
            }

            // Arc comparison diagram: schema arcs vs actual arcs
            // Shows existing (══) and missing (╌╌) connections
            if !kind.arcs.is_empty() {
                let comparisons = instance.compare_arcs(&kind.arcs);
                let existing_count = comparisons.iter().filter(|c| c.exists).count();
                let missing_count = comparisons.len() - existing_count;

                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!(
                        "Arc Diagram ({} exist, {} missing)",
                        existing_count, missing_count
                    ),
                    STYLE_MUTED,
                )));

                // Box drawing for instance node (use display width for CJK/emoji alignment)
                let key_width = display_width(&instance.key);
                lines.push(Line::from(Span::styled(
                    format!("  ┌{}┐", "─".repeat(key_width + 2)),
                    STYLE_INFO,
                )));
                lines.push(Line::from(Span::styled(
                    format!("  │ {} │", instance.key),
                    STYLE_INFO,
                )));
                lines.push(Line::from(Span::styled(
                    format!("  └{}┘", "─".repeat(key_width + 2)),
                    STYLE_INFO,
                )));

                // Arcs with status
                for cmp in &comparisons {
                    if cmp.exists {
                        // Existing arc: solid double line (══)
                        let target_display = cmp
                            .target_key
                            .clone()
                            .unwrap_or_else(|| cmp.target_kind.clone());
                        lines.push(Line::from(vec![
                            Span::styled("    ══", STYLE_SUCCESS),
                            Span::styled(format!("[{}]", cmp.arc_type), STYLE_HIGHLIGHT),
                            Span::styled("══> ", STYLE_SUCCESS),
                            Span::styled(target_display, STYLE_PRIMARY),
                            Span::styled(" ✓", STYLE_SUCCESS),
                        ]));
                    } else {
                        // Missing arc: dashed line (╌╌)
                        lines.push(Line::from(vec![
                            Span::styled("    ╌╌", STYLE_ERROR),
                            Span::styled(format!("[{}]", cmp.arc_type), STYLE_DIM),
                            Span::styled("╌╌> ", STYLE_ERROR),
                            Span::styled(
                                format!("({} - not connected)", cmp.target_kind),
                                STYLE_DIM,
                            ),
                            Span::styled(" ✗", STYLE_ERROR),
                        ]));
                    }
                }
            }

            lines
        }
        None => {
            vec![Line::from(Span::styled("Select an item", STYLE_DIM))]
        }
    }
}

/// Build a mini realm distribution bar using Unicode blocks with percentages.
/// Returns spans showing proportion of kinds per realm with realm colors.
/// Example: "▓▓█████ G:30% T:70%"
fn build_realm_mini_bar(app: &App, bar_width: usize) -> Vec<Span<'static>> {
    let mut spans = Vec::with_capacity(8);

    // Calculate total kinds from all realms
    let mut realm_counts: Vec<(&str, usize, Color)> = Vec::with_capacity(app.tree.realms.len());
    let mut total_kinds: usize = 0;

    for realm in &app.tree.realms {
        let count: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
        let color = hex_to_color(&realm.color);
        realm_counts.push((&realm.key, count, color));
        total_kinds += count;
    }

    if total_kinds == 0 {
        spans.push(Span::styled("░".repeat(bar_width), STYLE_DIM));
        return spans;
    }

    // Calculate proportional widths and build the bar
    let mut used_width = 0;
    let mut percentages: Vec<(&str, u8, Color)> = Vec::with_capacity(realm_counts.len());

    for (i, (key, count, color)) in realm_counts.iter().enumerate() {
        let proportion = *count as f64 / total_kinds as f64;
        let percent = (proportion * 100.0).round() as u8;
        percentages.push((key, percent, *color));

        let width = if i == realm_counts.len() - 1 {
            // Last realm gets remaining width to avoid rounding errors
            bar_width.saturating_sub(used_width)
        } else {
            (proportion * bar_width as f64).round() as usize
        };

        if width > 0 {
            // Use different block characters for distinction
            let block = match *key {
                "global" => "▓", // Lighter block for global (reference data)
                _ => "█",        // Solid block for tenant (business data)
            };
            spans.push(Span::styled(
                block.repeat(width),
                Style::default().fg(*color),
            ));
        }
        used_width += width;
    }

    // Add percentages after the bar: " Global:30% Tenant:70%"
    for (key, percent, color) in percentages {
        let label = match key {
            "global" => "Global",
            "tenant" => "Tenant",
            _ => key,
        };
        spans.push(Span::styled(format!(" {}:", label), STYLE_DIM));
        spans.push(Span::styled(
            format!("{}%", percent),
            Style::default().fg(color),
        ));
    }

    spans
}

/// Status bar: enriched with mode indicator, breadcrumb, shortcuts, spinner.
fn render_status(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.theme;

    // Mode indicator with icon and color
    let mode_label = app.mode.label();
    let mode_icon = theme.nav_mode_icon(mode_label);
    let mode_style = theme.nav_mode_style(mode_label);

    // Breadcrumb (truncated from start if too long - uses UTF-8 safe truncation)
    let breadcrumb = app.current_breadcrumb();
    let max_breadcrumb_len = (area.width as usize).saturating_sub(60).min(40);
    let breadcrumb_display = truncate_start(&breadcrumb, max_breadcrumb_len);

    // Contextual shortcuts based on mode, focus, and selection
    let shortcuts = match app.mode {
        NavMode::Atlas => "j/k:nav  1-4:modes  ?:help",
        NavMode::Audit => "j/k:nav  1-4:modes  ?:help",
        NavMode::Guide => "j/k:nav  Enter:select  q:back  ?:help",
        NavMode::Data => {
            // Check if on an Instance (can navigate to Kind with '1')
            if matches!(
                app.current_item(),
                Some(crate::tui::data::TreeItem::Instance(..))
            ) {
                "j/k:nav  y/Y:copy  1:Kind  ?:help"
            } else {
                "j/k:nav  y:copy  h/l:toggle  0:hide  ?:help"
            }
        }
        NavMode::Query => "j/k:nav  f:filter  ?:help",
        NavMode::Meta | NavMode::Overlay => match app.focus {
            Focus::Tree => {
                // Check if on a Kind (can drill into instances with '2')
                if matches!(
                    app.current_item(),
                    Some(crate::tui::data::TreeItem::Kind(..))
                ) {
                    "j/k:nav  y:copy  2:Data  h/l:toggle  ?:help"
                } else {
                    "j/k:nav  y:copy  h/l:toggle  ?:help"
                }
            }
            Focus::Yaml | Focus::Info => "j/k:scroll  d/u:page  g/G:jump",
            Focus::Graph => "Tab:panel  1-4:modes",
        },
    };

    // Filter indicator (show when filter is active)
    let filter_indicator = if app.is_filtered_data_mode() {
        if let Some(kind_key) = app.get_filter_kind() {
            format!(" [{}]", kind_key)
        } else {
            String::new()
        }
    } else if app.hide_empty {
        " [hide-empty]".to_string()
    } else {
        String::new()
    };

    // Build status line spans
    let mut spans = vec![
        // Mode indicator: [◈ META]
        Span::raw(" "),
        Span::styled(
            format!("{} {}", mode_icon, mode_label.to_uppercase()),
            mode_style,
        ),
    ];

    // Add filter indicator if active
    if !filter_indicator.is_empty() {
        spans.push(Span::styled(
            filter_indicator,
            Style::default().fg(Color::Yellow),
        ));
    }

    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    // Breadcrumb
    spans.push(Span::styled(breadcrumb_display, STYLE_HINT));

    // Loading spinner (if pending load)
    if app.has_pending_load() {
        spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
        spans.push(Span::styled(
            format!("{} Loading…", app.spinner_frame()),
            Style::default().fg(Color::Yellow),
        ));
    }

    // Status message (temporary, e.g., "Copied: key")
    if let Some((msg, _)) = &app.status_message {
        spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
        spans.push(Span::styled(msg.clone(), Style::default().fg(Color::Green)));
    }

    // Spacer to push shortcuts to the right
    spans.push(Span::raw("  "));

    // Stats (full words: nodes·arcs │ kinds·arc-kinds)
    let stats = &app.tree.stats;
    spans.push(Span::styled(
        format!(
            "{} nodes·{} arcs │ {} Kinds·{} ArcKinds",
            stats.node_count, stats.arc_count, stats.kind_count, stats.arc_kind_count
        ),
        STYLE_MUTED,
    ));

    // Mini realm distribution bar (8 char width) - shows proportion of kinds per realm
    spans.push(Span::styled(" ", STYLE_SEPARATOR));
    spans.extend(build_realm_mini_bar(app, 8));

    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));

    // Contextual shortcuts
    spans.push(Span::styled(shortcuts, STYLE_DIM));

    spans.push(Span::raw(" "));

    let status = Line::from(spans);
    let paragraph = Paragraph::new(status).style(Style::default().bg(Color::Rgb(15, 15, 20)));

    f.render_widget(paragraph, area);
}

// =============================================================================
// HELPER: Simple Property Rendering
// =============================================================================

/// Render instance properties in simple mode (no schema overlay).
/// Shows each property with key-value format, truncating long values.
fn render_simple_properties(lines: &mut Vec<Line<'_>>, properties: &BTreeMap<String, JsonValue>) {
    if properties.is_empty() {
        return;
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("Properties", STYLE_MUTED)));

    for (key, value) in properties {
        // Skip internal properties (starting with underscore or known meta)
        if key.starts_with('_') || key == "key" || key == "display_name" {
            continue;
        }

        let value_str = json_value_to_display(value);
        let truncated = truncate_str(&value_str, 45);

        lines.push(Line::from(vec![
            Span::styled(format!("{:<20}", key), STYLE_INFO),
            Span::styled(truncated, STYLE_PRIMARY),
        ]));
    }
}

/// Convert a JSON value to a display string.
fn json_value_to_display(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => format!("\"{}\"", s),
        JsonValue::Array(arr) => serde_json::to_string(arr).unwrap_or_else(|_| "[]".to_string()),
        JsonValue::Object(obj) => serde_json::to_string(obj).unwrap_or_else(|_| "{}".to_string()),
    }
}

/// Render the recent items popup overlay.
fn render_recent_items_overlay(f: &mut Frame, app: &App) {
    use ratatui::widgets::Clear;

    // Center the popup
    let area = f.area();
    let width = POPUP_BOX_WIDTH.min(area.width.saturating_sub(4));
    let height = 14.min(area.height.saturating_sub(4)); // Taller for list items
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 3;

    let popup_area = Rect::new(x, y, width, height);

    // Clear the area behind the overlay
    f.render_widget(Clear, popup_area);

    // Build content
    let mut lines: Vec<Line> = Vec::new();

    // Title
    lines.push(Line::from(vec![
        Span::styled(" Recent Items ", STYLE_INFO),
        Span::styled("(j/k Enter Esc)", STYLE_DIM),
    ]));
    lines.push(Line::from(""));

    // Show history items (newest first, limit to 10)
    let max_items = 10.min(app.nav_history.len());
    let visible_height = height.saturating_sub(4) as usize;

    // Calculate scroll window
    let start = if max_items <= visible_height || app.recent_items_cursor < visible_height / 2 {
        0
    } else if app.recent_items_cursor > max_items - visible_height / 2 {
        max_items.saturating_sub(visible_height)
    } else {
        app.recent_items_cursor.saturating_sub(visible_height / 2)
    };

    for display_idx in start..start + visible_height.min(max_items - start) {
        // History is oldest→newest, we show newest first
        let history_idx = app.nav_history.len().saturating_sub(1 + display_idx);
        let is_selected = display_idx == app.recent_items_cursor;

        if let Some(&(mode, cursor)) = app.nav_history.get(history_idx) {
            // Get item name at that cursor position
            let item = app.tree.item_at(cursor);
            let (icon, name) = match item {
                Some(crate::tui::data::TreeItem::KindsSection) => ("≡", "Node Kinds".to_string()),
                Some(crate::tui::data::TreeItem::ArcsSection) => ("⇄", "Arcs".to_string()),
                Some(crate::tui::data::TreeItem::Realm(r)) => (r.icon, r.display_name.clone()),
                Some(crate::tui::data::TreeItem::Layer(_, l)) => ("▸", l.display_name.clone()),
                Some(crate::tui::data::TreeItem::Kind(_, _, k)) => ("◆", k.display_name.clone()),
                Some(crate::tui::data::TreeItem::Instance(_, _, _, i)) => {
                    ("•", i.display_name.clone())
                }
                Some(crate::tui::data::TreeItem::ArcFamily(f)) => ("↔", f.display_name.clone()),
                Some(crate::tui::data::TreeItem::ArcKind(_, ak)) => ("→", ak.display_name.clone()),
                None => ("?", format!("(cursor {})", cursor)),
            };

            let mode_badge = match mode {
                crate::tui::app::NavMode::Meta => "[M]",
                crate::tui::app::NavMode::Data => "[D]",
                crate::tui::app::NavMode::Overlay => "[O]",
                crate::tui::app::NavMode::Query => "[Q]",
                crate::tui::app::NavMode::Atlas => "[A]",
                crate::tui::app::NavMode::Audit => "[!]",
                crate::tui::app::NavMode::Guide => "[G]",
            };

            let prefix = if is_selected { "› " } else { "  " };
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

    // Footer hint
    if max_items == 0 {
        lines.push(Line::from(Span::styled(
            "  No history yet. Navigate around!",
            STYLE_DIM,
        )));
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(COLOR_OVERLAY_BG)),
    );

    f.render_widget(paragraph, popup_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_str_ascii_under_limit() {
        assert_eq!(truncate_str("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_str_ascii_at_limit() {
        assert_eq!(truncate_str("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_str_ascii_over_limit() {
        // Width-based: "hell" (4 cols) + "…" (1 col) = 5 cols
        assert_eq!(truncate_str("hello world", 5), "hell…");
    }

    #[test]
    fn test_truncate_str_utf8_bengali() {
        // Bengali: "বাংলা (বাংলাদেশ)" - this caused the original panic
        let bengali = "বাংলা (বাংলাদেশ)";
        // Should not panic even when truncating in the middle of multi-byte chars
        let result = truncate_str(bengali, 5);
        // Width-based truncation with "…" suffix
        assert!(result.ends_with('…'));
    }

    #[test]
    fn test_truncate_str_utf8_emoji() {
        let emoji = "Hello 👋🏻 World 🌍";
        let result = truncate_str(emoji, 8);
        // Width-based truncation uses "…" (single char ellipsis)
        assert!(result.ends_with('…'));
    }

    #[test]
    fn test_truncate_str_chinese() {
        // Chinese chars are 2 columns each: 你(2) + …(1) = 3 cols fits in 4
        // But 你(2) + 好(2) = 4, which equals max, so we can fit "你好" if exact
        // Actually: max_width=4, 你好 = 4 cols, fits exactly without truncation
        // Let's use max_width=3: 你(2) + …(1) = 3 cols
        let chinese = "你好世界这是中文测试";
        let result = truncate_str(chinese, 3);
        assert_eq!(result, "你…"); // 你(2) + …(1) = 3 cols
    }

    #[test]
    fn test_truncate_str_empty() {
        assert_eq!(truncate_str("", 10), "");
    }

    // =============================================================================
    // truncate_start tests (UTF-8 safe start truncation for breadcrumbs)
    // =============================================================================

    #[test]
    fn test_truncate_start_under_limit() {
        assert_eq!(truncate_start("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_start_at_limit() {
        assert_eq!(truncate_start("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_start_over_limit() {
        // Width-based: "…" (1 col) + "orld" (4 cols) = 5 cols
        assert_eq!(truncate_start("hello world", 5), "…orld");
    }

    #[test]
    fn test_truncate_start_utf8_arrows() {
        // This is the actual bug case: "Global → Tenant" with → being 3 bytes
        let s = "Global → Tenant Configuration → Slugification";
        let result = truncate_start(s, 20);
        // Should keep last 20 chars without panicking
        assert!(result.starts_with('…'));
        assert!(result.chars().count() <= 21); // 20 + ellipsis
    }

    #[test]
    fn test_truncate_start_utf8_emoji() {
        // "🎉 Hello 🎉 World" - emojis are 4 bytes each
        let s = "🎉 Hello 🎉 World";
        let result = truncate_start(s, 8);
        assert!(result.starts_with('…'));
        // Should not panic on multi-byte boundaries
    }

    #[test]
    fn test_truncate_start_empty() {
        assert_eq!(truncate_start("", 10), "");
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

    // =============================================================================
    // Spinner tests
    // =============================================================================

    #[test]
    fn test_spinner_cycles_through_frames() {
        // Spinner should return different chars for different ticks
        let frames: Vec<&str> = (0..20).map(spinner).collect();
        // Check that we get braille characters
        assert!(frames.iter().all(|f| f.chars().all(|c| !c.is_ascii())));
    }

    // =============================================================================
    // EmptyStateKind tests (Phase 6.2 TDD)
    // =============================================================================

    #[test]
    fn test_empty_state_kind_icon_no_connection() {
        assert_eq!(EmptyStateKind::NoConnection.icon(), "⚠");
    }

    #[test]
    fn test_empty_state_kind_icon_no_kinds() {
        assert_eq!(EmptyStateKind::NoKinds.icon(), "∅");
    }

    #[test]
    fn test_empty_state_kind_icon_no_results() {
        assert_eq!(EmptyStateKind::NoResults.icon(), "◌");
    }

    #[test]
    fn test_empty_state_kind_icon_no_instances() {
        assert_eq!(EmptyStateKind::NoInstances.icon(), "□");
    }

    #[test]
    fn test_empty_state_kind_icon_loading() {
        assert_eq!(EmptyStateKind::Loading.icon(), "◐");
    }

    #[test]
    fn test_empty_state_kind_title_no_connection() {
        assert_eq!(EmptyStateKind::NoConnection.title(), "Neo4j Not Connected");
    }

    #[test]
    fn test_empty_state_kind_title_no_kinds() {
        assert_eq!(EmptyStateKind::NoKinds.title(), "No Node Kinds Found");
    }

    #[test]
    fn test_empty_state_kind_title_no_results() {
        assert_eq!(EmptyStateKind::NoResults.title(), "No Results");
    }

    #[test]
    fn test_empty_state_kind_title_no_instances() {
        assert_eq!(EmptyStateKind::NoInstances.title(), "No Instances");
    }

    #[test]
    fn test_empty_state_kind_title_loading() {
        assert_eq!(EmptyStateKind::Loading.title(), "Loading…");
    }

    #[test]
    fn test_empty_state_kind_description_no_connection() {
        let desc = EmptyStateKind::NoConnection.description();
        assert!(!desc.is_empty(), "description should not be empty");
        assert!(
            desc.iter().any(|s| s.contains("Neo4j")),
            "should mention Neo4j"
        );
        assert!(
            desc.iter().any(|s| s.contains("infra:up")),
            "should suggest pnpm infra:up"
        );
    }

    #[test]
    fn test_empty_state_kind_description_no_kinds() {
        let desc = EmptyStateKind::NoKinds.description();
        assert!(!desc.is_empty());
        assert!(
            desc.iter().any(|s| s.contains("schema generate")),
            "should suggest schema generate"
        );
        assert!(
            desc.iter().any(|s| s.contains("db seed")),
            "should suggest db seed"
        );
    }

    #[test]
    fn test_empty_state_kind_description_no_results() {
        let desc = EmptyStateKind::NoResults.description();
        assert!(!desc.is_empty());
        assert!(
            desc.iter().any(|s| s.contains("filter")),
            "should mention filters"
        );
    }

    #[test]
    fn test_empty_state_kind_description_no_instances() {
        let desc = EmptyStateKind::NoInstances.description();
        assert!(!desc.is_empty());
        assert!(
            desc.iter().any(|s| s.contains("node create")),
            "should suggest node create command"
        );
    }

    #[test]
    fn test_empty_state_kind_description_loading() {
        let desc = EmptyStateKind::Loading.description();
        assert!(!desc.is_empty());
        assert!(
            desc.iter().any(|s| s.contains("Neo4j")),
            "should mention Neo4j"
        );
    }

    #[test]
    fn test_empty_state_kind_hint_no_connection() {
        let hint = EmptyStateKind::NoConnection.hint();
        assert!(hint.contains("r"), "hint should suggest retry with 'r'");
    }

    #[test]
    fn test_empty_state_kind_hint_no_kinds() {
        let hint = EmptyStateKind::NoKinds.hint();
        assert!(hint.contains("q"), "hint should suggest quit with 'q'");
    }

    #[test]
    fn test_empty_state_kind_hint_no_results() {
        let hint = EmptyStateKind::NoResults.hint();
        assert!(
            hint.contains("c"),
            "hint should suggest clearing filters with 'c'"
        );
    }

    #[test]
    fn test_empty_state_kind_hint_no_instances() {
        let hint = EmptyStateKind::NoInstances.hint();
        assert!(hint.contains("Esc"), "hint should suggest go back");
    }

    #[test]
    fn test_empty_state_kind_hint_loading() {
        // Loading has no hint - it's a transient state
        let hint = EmptyStateKind::Loading.hint();
        // Just verify it doesn't panic and returns something
        assert!(hint.is_empty() || !hint.is_empty());
    }

    #[test]
    fn test_empty_state_kind_is_copy() {
        // Verify EmptyStateKind is Copy (can be assigned without move)
        let kind = EmptyStateKind::NoConnection;
        let kind2 = kind; // Copy
        let _kind3 = kind; // Still valid - proves Copy
        assert_eq!(kind2.title(), "Neo4j Not Connected");
    }

    #[test]
    fn test_empty_state_kind_debug_trait() {
        // Verify Debug is implemented
        let kind = EmptyStateKind::Loading;
        let debug_str = format!("{:?}", kind);
        assert!(
            debug_str.contains("Loading"),
            "Debug should contain variant name"
        );
    }

    #[test]
    fn test_all_empty_state_kinds_have_non_empty_icon() {
        let kinds = [
            EmptyStateKind::NoConnection,
            EmptyStateKind::NoKinds,
            EmptyStateKind::NoResults,
            EmptyStateKind::NoInstances,
            EmptyStateKind::Loading,
        ];
        for kind in kinds {
            assert!(
                !kind.icon().is_empty(),
                "{:?} icon should not be empty",
                kind
            );
        }
    }

    #[test]
    fn test_all_empty_state_kinds_have_non_empty_title() {
        let kinds = [
            EmptyStateKind::NoConnection,
            EmptyStateKind::NoKinds,
            EmptyStateKind::NoResults,
            EmptyStateKind::NoInstances,
            EmptyStateKind::Loading,
        ];
        for kind in kinds {
            assert!(
                !kind.title().is_empty(),
                "{:?} title should not be empty",
                kind
            );
        }
    }
}
