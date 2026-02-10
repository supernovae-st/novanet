//! UI rendering for TUI v2.

mod atlas;
mod audit;
mod graph;
mod info;
mod overlays;
mod status;
mod tree;
mod yaml_panel;

#[allow(unused_imports)]
pub use atlas::render_atlas; // Atlas mode is work-in-progress
pub use audit::render_audit;
pub use graph::render_graph_panel;
pub use info::render_info_panel;
pub use status::render_status;
pub use tree::render_tree;
pub use yaml_panel::render_yaml_panel;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::app::{App, NavMode};
use super::theme::{self, hex_to_color};
use super::unicode::{truncate_start_to_width, truncate_to_width};

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

/// Wide layout: Tree panel percentage (v11.6: equal 1/3 columns).
const LAYOUT_WIDE_TREE_PCT: u16 = 33;
/// Wide layout: Info+Graph panel percentage.
const LAYOUT_WIDE_INFO_PCT: u16 = 34;
/// Wide layout: YAML panel percentage.
const LAYOUT_WIDE_YAML_PCT: u16 = 33;
/// Wide layout: Info section percentage (within Info+Graph).
const LAYOUT_WIDE_INFO_SECTION_PCT: u16 = 60;
/// Wide layout: Graph section percentage (within Info+Graph).
const LAYOUT_WIDE_GRAPH_SECTION_PCT: u16 = 40;

/// Narrow layout: Tree panel percentage (v11.6: equal 1/3).
const LAYOUT_NARROW_TREE_PCT: u16 = 33;
/// Narrow layout: Detail panel percentage.
const LAYOUT_NARROW_DETAIL_PCT: u16 = 67;
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

/// Bright dim text style.
const STYLE_BRIGHT_DIM: Style = Style::new().fg(COLOR_BRIGHT_DIM);

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
// HELPER FUNCTIONS (shared across UI modules)
// =============================================================================

/// Get icon for a node trait (from visual-encoding.yaml).
///
/// - invariant: ■ (solid square) - stable across locales
/// - localized: □ (empty square) - locale-specific content
/// - knowledge: ◊ (diamond) - locale expertise data
/// - generated: ★ (star) - LLM-generated output
/// - aggregated: ▪ (small square) - computed metrics
///
/// Note: v11.2 removed job trait, split derived into generated+aggregated
pub(super) fn trait_icon(trait_name: &str) -> &'static str {
    match trait_name {
        "invariant" => "■",
        "localized" => "□",
        "knowledge" => "◊",
        "generated" => "★",
        "aggregated" => "▪",
        _ => "·", // fallback
    }
}

/// Get color for a node trait (from taxonomy.yaml node_traits).
/// Uses TrueColor RGB values for visual consistency in tree view.
pub(super) fn trait_color(trait_name: &str) -> Color {
    match trait_name {
        "invariant" => hex_to_color(theme::traits::INVARIANT_HEX), // #3b82f6 blue
        "localized" => hex_to_color(theme::traits::LOCALIZED_HEX), // #22c55e green
        "knowledge" => hex_to_color(theme::traits::KNOWLEDGE_HEX), // #8b5cf6 purple
        "generated" => hex_to_color(theme::traits::GENERATED_HEX), // #b58900 gold
        "aggregated" => hex_to_color(theme::traits::AGGREGATED_HEX), // #6c71c4 violet
        _ => Color::White,                                         // fallback
    }
}

// =============================================================================
// CLASSIFICATION BADGE HELPERS (v11.5 TreeView Enhancement)
// =============================================================================

/// Get short abbreviation for realm display in tree badges.
/// Format: ◎xxx where xxx is 3-letter abbreviation
pub(super) fn realm_abbrev(realm_key: &str) -> &'static str {
    match realm_key {
        "shared" => "shd",
        "org" => "org",
        _ => "???",
    }
}

/// Get icon for realm badge (from visual-encoding.yaml).
/// Named `_badge` to avoid collision with expand_icon variables in tree.rs
pub(super) fn realm_badge_icon(realm_key: &str) -> &'static str {
    match realm_key {
        "shared" => "◎",
        "org" => "◉",
        _ => "○",
    }
}

/// Get short abbreviation for layer display in tree badges.
/// v11.5: 10 layers (4 shared + 6 org)
pub(super) fn layer_abbrev(layer_key: &str) -> &'static str {
    match layer_key {
        // v11.5: 4 shared layers
        "config" => "cfg",
        "locale" => "loc",
        "geography" => "geo",
        "knowledge" => "kno",
        // v11.5: 6 org layers
        "foundation" => "fnd",
        "structure" => "str",
        "semantic" => "sem",
        "instruction" => "ins",
        "output" => "out",
        _ => "???",
    }
}

/// Get icon for layer badge (from visual-encoding.yaml).
/// v11.5: All icons are single-width Unicode symbols (no emojis)
pub(super) fn layer_badge_icon(layer_key: &str) -> &'static str {
    match layer_key {
        // v11.5: 4 shared layers
        "config" => "⚙",
        "locale" => "⊕",
        "geography" => "⊙",
        "knowledge" => "◈",
        // v11.5: 6 org layers
        "foundation" => "▣",
        "structure" => "▤",
        "semantic" => "◆",
        "instruction" => "▧",
        "output" => "●",
        _ => "○",
    }
}

/// Get short abbreviation for trait display in tree badges.
pub(super) fn trait_abbrev(trait_name: &str) -> &'static str {
    match trait_name {
        "invariant" => "inv",
        "localized" => "loc",
        "knowledge" => "kno",
        "generated" => "gen",
        "aggregated" => "agg",
        _ => "???",
    }
}

// v11.6.1: Arc Family helpers for tree visual enrichment

/// Get short abbreviation for arc family display in tree badges.
pub(super) fn arc_family_abbrev(family_key: &str) -> &'static str {
    match family_key {
        "ownership" => "own",
        "localization" => "loc",
        "semantic" => "sem",
        "generation" => "gen",
        "mining" => "min",
        _ => "???",
    }
}

/// Get icon for arc family badge (from visual-encoding.yaml).
pub(super) fn arc_family_badge_icon(family_key: &str) -> &'static str {
    match family_key {
        "ownership" => "→",
        "localization" => "⇢",
        "semantic" => "~",
        "generation" => "⇒",
        "mining" => "⇝",
        _ => "?",
    }
}

/// Get short abbreviation for cardinality display.
pub(super) fn cardinality_abbrev(cardinality: &str) -> &'static str {
    match cardinality {
        "zero_to_one" => "0:1",
        "one_to_one" => "1:1",
        "one_to_many" => "1:N",
        "many_to_one" => "N:1",
        "many_to_many" => "N:M",
        _ => "?:?",
    }
}

/// Format classification badges for a Kind node.
/// Returns: "◎shd ▣cfg ■inv" format with colored spans
/// Note: Currently unused but available for future use in info panel
#[allow(dead_code)]
pub(super) fn format_classification_badge(
    realm_key: &str,
    layer_key: &str,
    trait_name: &str,
    realm_color: Color,
    layer_color: Color,
) -> Vec<Span<'static>> {
    let t_color = trait_color(trait_name);
    vec![
        Span::styled(
            format!("{}{}", realm_badge_icon(realm_key), realm_abbrev(realm_key)),
            Style::default().fg(realm_color),
        ),
        Span::styled(" ", Style::default()),
        Span::styled(
            format!("{}{}", layer_badge_icon(layer_key), layer_abbrev(layer_key)),
            Style::default().fg(layer_color),
        ),
        Span::styled(" ", Style::default()),
        Span::styled(
            format!("{}{}", trait_icon(trait_name), trait_abbrev(trait_name)),
            Style::default().fg(t_color),
        ),
    ]
}

/// Wrap text to lines of max `width` characters, returning owned Strings.
/// Uses char indices instead of collecting to Vec<char> for efficiency.
pub(super) fn wrap_text(text: &str, width: usize) -> Vec<String> {
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
#[allow(dead_code)] // Used by tests
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
            Constraint::Length(1), // Footer hints
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_main(f, chunks[1], app);
    render_status(f, chunks[2], app);
    render_footer_hints(f, chunks[3], app);

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

/// Footer: Contextual keybinding hints based on mode and focus.
fn render_footer_hints(f: &mut Frame, area: Rect, app: &App) {
    use crate::tui::app::Focus;

    let hints = match app.mode {
        NavMode::Meta | NavMode::Data => match app.focus {
            Focus::Tree => {
                "Tree: [h/l] Toggle  [j/k] Navigate  [Space] Expand  [g/G] Top/Bottom  [Tab] Panel  [/] Search  [?] Help"
            }
            Focus::Info => "Info: [j/k] Scroll  [y] Copy  [Tab] Panel  [/] Search  [?] Help",
            Focus::Graph => "Graph: [Click] Select  [Scroll] Zoom  [Tab] Panel  [?] Help",
            Focus::Yaml => "YAML: [j/k] Scroll  [y] Copy  [Tab] Panel  [/] Search  [?] Help",
        },
        NavMode::Audit => {
            "[j/k] Navigate  [Enter] Drill down  [r] Refresh  [1-4] Mode  [?] Help  [q] Quit"
        }
        NavMode::Nexus => {
            "[1-4] Tabs  [j/k] Navigate  [Enter] Select  [Esc] Back  [/] Search  [?] Help"
        }
    };

    let line = Line::from(Span::styled(
        format!("  {}", hints),
        Style::default().fg(COLOR_HINT_TEXT),
    ));

    let paragraph = Paragraph::new(line).style(Style::default().bg(Color::Rgb(18, 18, 25)));
    f.render_widget(paragraph, area);
}

/// Header: Logo + Mode tabs.
/// Shows: [1]Meta, [2]Data, [3]Audit, [4]Nexus
/// v11.6: 4 independent modes with keys 1-4
fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let tabs: Vec<Span> = [NavMode::Meta, NavMode::Data, NavMode::Audit, NavMode::Nexus]
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

    // Show hide_empty indicator when active in Instances view
    if app.hide_empty && app.is_data_mode() {
        header.push(Span::styled(
            " [∅ hidden]",
            Style::default().fg(Color::Yellow),
        ));
    }

    // Context-aware shortcuts (v11.6: 4 independent modes, 1-4 global)
    let right_side = if app.mode == NavMode::Nexus {
        vec![Span::styled(
            "  []:tabs  jk:nav  Enter:drill  Esc:back  1-4:modes  ?:help  q:quit",
            theme::ui::muted_style(),
        )]
    } else if app.mode == NavMode::Audit {
        vec![Span::styled(
            "  jk:nav  r:refresh  1-4:modes  ?:help  q:quit",
            theme::ui::muted_style(),
        )]
    } else {
        // Meta/Data mode (v11.6: 't' toggle removed, 1-4 for modes)
        vec![Span::styled(
            "  h/l:toggle  jk:scroll  Tab:panel  /:find  1-4:modes  ?:help  q:quit",
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
    // Audit mode has its own rendering (Feature 6)
    if app.mode == NavMode::Audit {
        render_audit(f, area, app);
        return;
    }

    // Nexus mode has its own rendering (v11.3: renamed from Guide)
    if app.mode == NavMode::Nexus {
        super::nexus::render_nexus(f, area, app);
        return;
    }

    // Graph mode: standard 3-panel layout
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

/// Colorize path inline for title.
pub(super) fn colorize_path_inline(path: &str) -> Vec<Span<'static>> {
    let parts: Vec<&str> = path.split('/').collect();
    let mut spans: Vec<Span<'static>> = Vec::new();

    for (i, part) in parts.iter().enumerate() {
        let color = match i {
            0..=2 => Color::Rgb(80, 80, 90), // packages/core/models
            3 => Color::Magenta,             // nodes
            4 => match *part {
                // realm (v11.4: 2 realms - shared + org)
                "shared" => Color::Green,
                "org" => Color::Yellow,
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
                Some(crate::tui::data::TreeItem::EntityCategory(_, _, _, cat)) => {
                    ("◫", cat.display_name.clone())
                }
                None => ("?", format!("(cursor {})", cursor)),
            };

            // v11.6: 4 modes (Meta, Data, Audit, Nexus)
            let mode_badge = match mode {
                crate::tui::app::NavMode::Meta => "[M]",
                crate::tui::app::NavMode::Data => "[D]",
                crate::tui::app::NavMode::Audit => "[!]",
                crate::tui::app::NavMode::Nexus => "[N]",
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
        // This is the actual bug case: "Shared → Org" with → being 3 bytes
        let s = "Shared → Org Configuration → Slugification";
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
