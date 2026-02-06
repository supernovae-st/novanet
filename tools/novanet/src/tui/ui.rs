//! UI rendering for TUI v2.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
#[allow(unused_imports)] // Sparkline used in Batch 4.2
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, Borders, Clear, Paragraph, Sparkline, Wrap};

use super::app::{App, Focus, NavMode};
use super::data::{ArcDirection, TreeItem};
use super::theme::{self, hex_to_color};

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
const COLOR_UNFOCUSED_BORDER: Color = Color::Rgb(60, 60, 70);

/// Muted text for secondary information.
const COLOR_MUTED_TEXT: Color = Color::Rgb(100, 100, 120);

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
const COLOR_HINT_TEXT: Color = Color::Rgb(80, 80, 100);

/// Overlay/popup background.
const COLOR_OVERLAY_BG: Color = Color::Rgb(20, 20, 30);

/// Brighter dim text.
const COLOR_BRIGHT_DIM: Color = Color::Rgb(140, 140, 140);

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

// -----------------------------------------------------------------------------
// YAML syntax highlighting styles (const to avoid recreation per line)
// -----------------------------------------------------------------------------

/// YAML comment style.
const STYLE_YAML_COMMENT: Style = Style::new().fg(Color::DarkGray);

/// YAML key style.
const STYLE_YAML_KEY: Style = Style::new().fg(Color::Yellow);

/// YAML colon/dash style.
const STYLE_YAML_PUNCT: Style = Style::new().fg(Color::Cyan);

/// YAML string value style.
const STYLE_YAML_STRING: Style = Style::new().fg(Color::Green);

/// YAML boolean/null style.
const STYLE_YAML_LITERAL: Style = Style::new().fg(Color::Magenta);

/// YAML number style.
const STYLE_YAML_NUMBER: Style = Style::new().fg(Color::Cyan);

/// YAML plain text style.
const STYLE_YAML_TEXT: Style = Style::new().fg(Color::White);

// -----------------------------------------------------------------------------
// General UI styles (most frequently used, avoid Style::default() overhead)
// -----------------------------------------------------------------------------

/// Dim/secondary text (e.g., counts, separators).
const STYLE_DIM: Style = Style::new().fg(Color::DarkGray);

/// Default/primary text.
const STYLE_PRIMARY: Style = Style::new().fg(Color::White);

/// Highlighted/important text (e.g., selected items).
const STYLE_HIGHLIGHT: Style = Style::new().fg(Color::Yellow);

/// Informational text (e.g., types, metadata).
const STYLE_INFO: Style = Style::new().fg(Color::Cyan);

/// Success/positive indicators.
const STYLE_SUCCESS: Style = Style::new().fg(Color::Green);

/// Accent color (e.g., special values).
const STYLE_ACCENT: Style = Style::new().fg(Color::Magenta);

/// Error/warning indicators.
const STYLE_ERROR: Style = Style::new().fg(Color::Red);

/// Muted/secondary text (custom RGB).
const STYLE_MUTED: Style = Style::new().fg(COLOR_MUTED_TEXT);

/// Separator dots style.
const STYLE_SEPARATOR: Style = Style::new().fg(COLOR_SEPARATOR);

/// Hint text style.
const STYLE_HINT: Style = Style::new().fg(COLOR_HINT_TEXT);

/// Description text style.
const STYLE_DESC: Style = Style::new().fg(COLOR_DESC_TEXT);

/// Unfocused border style.
const STYLE_UNFOCUSED: Style = Style::new().fg(COLOR_UNFOCUSED_BORDER);

/// Arc family label style.
const STYLE_ARC_FAMILY: Style = Style::new().fg(COLOR_ARC_FAMILY);

/// Bright dim text style.
const STYLE_BRIGHT_DIM: Style = Style::new().fg(COLOR_BRIGHT_DIM);

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
            EmptyStateKind::NoKinds => "📭",
            EmptyStateKind::NoResults => "🔍",
            EmptyStateKind::NoInstances => "📋",
            EmptyStateKind::Loading => "⏳",
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

    /// Get the description lines for this empty state.
    fn description(&self) -> Vec<&'static str> {
        match self {
            EmptyStateKind::NoConnection => vec![
                "Unable to reach bolt://localhost:7687",
                "",
                "Try:",
                "  • pnpm infra:up",
                "  • Check Neo4j credentials",
            ],
            EmptyStateKind::NoKinds => vec![
                "The taxonomy tree is empty.",
                "",
                "Run:",
                "  • cargo run -- schema generate",
                "  • cargo run -- db seed",
            ],
            EmptyStateKind::NoResults => vec![
                "No nodes match your current filter.",
                "",
                "Try:",
                "  • Remove filters with 'c'",
                "  • Switch modes with 1-5",
            ],
            EmptyStateKind::NoInstances => vec![
                "This Kind has no data instances yet.",
                "",
                "Create one with:",
                "  cargo run -- node create --kind=<Kind>",
            ],
            EmptyStateKind::Loading => vec!["Fetching data from Neo4j…"],
        }
    }

    /// Get the hint text for this empty state.
    fn hint(&self) -> &'static str {
        match self {
            EmptyStateKind::NoConnection => "Press 'r' to retry",
            EmptyStateKind::NoKinds => "Press 'q' to quit",
            EmptyStateKind::NoResults => "Press '1' for Meta mode",
            EmptyStateKind::NoInstances => "Press Esc to go back",
            EmptyStateKind::Loading => "",
        }
    }
}

/// Render an empty state message in a centered box.
fn render_empty_state(f: &mut Frame, area: Rect, kind: EmptyStateKind, tick: u16) {
    // Calculate centered box dimensions
    let box_width = 50.min(area.width.saturating_sub(4));
    let box_height = 12.min(area.height.saturating_sub(2));

    if box_width < 20 || box_height < 6 {
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
        Span::styled(title_text, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
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
        lines.push(Line::from(Span::styled(
            format!("  {}", hint),
            STYLE_INFO,
        )));
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

/// Safely truncate a UTF-8 string to N characters (not bytes).
/// Appends "..." if truncated.
fn truncate_str(s: &str, max_chars: usize) -> String {
    let char_count = s.chars().count();
    if char_count > max_chars {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{}...", truncated)
    } else {
        s.to_string()
    }
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
    if app.search_active {
        render_search(f, app);
    }
    if app.help_active {
        render_help(f);
    }
}

/// Header: Logo + Mode tabs.
fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let tabs: Vec<Span> = [
        NavMode::Meta,
        NavMode::Data,
        NavMode::Overlay,
        NavMode::Query,
        NavMode::Atlas,
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
            Span::styled(
                format!(" {}{} ", num, label),
                STYLE_DIM,
            )
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
        header.push(Span::styled(" [∅ hidden]", Style::default().fg(Color::Yellow)));
    }

    // Context-aware shortcuts
    let right_side = if app.mode == NavMode::Atlas {
        vec![Span::styled(
            "  a-r:views  d:demo  l:locale  /:help  q:quit",
            theme::ui::muted_style(),
        )]
    } else if app.mode == NavMode::Data {
        vec![Span::styled(
            "  h/l:toggle  jk:scroll  0:hide  Tab:panel  f:find  /:help  q:quit",
            theme::ui::muted_style(),
        )]
    } else {
        vec![Span::styled(
            "  h/l:toggle  jk:scroll  Tab:panel  f:find  /:help  q:quit",
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

    let layout_mode = LayoutMode::detect(area.width);

    match layout_mode {
        LayoutMode::Wide => render_main_wide(f, area, app),
        LayoutMode::Narrow => render_main_narrow(f, area, app),
    }
}

/// Wide layout: Tree (20%) | Info+Graph (40%) | YAML (40%).
fn render_main_wide(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Tree
            Constraint::Percentage(40), // Info + Graph (stacked)
            Constraint::Percentage(40), // YAML
        ])
        .split(area);

    render_tree(f, chunks[0], app);

    // Stack Info (60%) and Graph (40%) vertically in the middle panel
    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // Info
            Constraint::Percentage(40), // Graph
        ])
        .split(chunks[1]);

    render_info_panel(f, middle_chunks[0], app);
    render_graph_panel(f, middle_chunks[1], app);

    render_yaml_panel(f, chunks[2], app);
}

/// Narrow layout: Tree (20%) | Info+Graph+YAML stacked (80%).
fn render_main_narrow(f: &mut Frame, area: Rect, app: &mut App) {
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Tree (narrower)
            Constraint::Percentage(80), // Detail (stacked)
        ])
        .split(area);

    render_tree(f, h_chunks[0], app);

    // Stack Info (35%), Graph (30%), YAML (35%) vertically
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35), // Info
            Constraint::Percentage(30), // Graph
            Constraint::Percentage(35), // YAML
        ])
        .split(h_chunks[1]);

    render_info_panel(f, v_chunks[0], app);
    render_graph_panel(f, v_chunks[1], app);
    render_yaml_panel(f, v_chunks[2], app);
}

/// Create styled spans with fuzzy match highlighting.
/// Matched character positions are shown with a highlight background.
fn highlight_matches(text: &str, matches: Option<&[u32]>, base_color: Color) -> Vec<Span<'static>> {
    let Some(positions) = matches else {
        return vec![Span::styled(text.to_string(), Style::default().fg(base_color))];
    };

    if positions.is_empty() {
        return vec![Span::styled(text.to_string(), Style::default().fg(base_color))];
    }

    // Build a set of matched positions for O(1) lookup
    let match_set: std::collections::HashSet<usize> = positions.iter().map(|&p| p as usize).collect();

    let mut spans = Vec::new();
    let mut current_text = String::new();
    let mut in_match = false;

    for (i, c) in text.chars().enumerate() {
        let is_match = match_set.contains(&i);

        if is_match != in_match {
            // Flush current segment
            if !current_text.is_empty() {
                let style = if in_match {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    Style::default().fg(base_color)
                };
                spans.push(Span::styled(std::mem::take(&mut current_text), style));
            }
            in_match = is_match;
        }
        current_text.push(c);
    }

    // Flush remaining
    if !current_text.is_empty() {
        let style = if in_match {
            Style::default().fg(Color::Black).bg(Color::Yellow)
        } else {
            Style::default().fg(base_color)
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
    let make_line = |idx: usize,
                     cursor: usize,
                     focused: bool,
                     tree_prefix: &str,
                     icon: &str,
                     text: String,
                     line_color: Color,
                     text_color: Color,
                     match_positions: Option<&[u32]>|
     -> Line {
        let is_cursor = idx == cursor;
        let cursor_char = if is_cursor { "›" } else { " " };
        let icon_space = if icon.is_empty() { "" } else { " " };

        if is_cursor && focused {
            // When focused/selected, use white on highlight bg for entire line
            let style = Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White);
            Line::from(Span::styled(
                format!("{}{}{}{}{}", cursor_char, tree_prefix, icon, icon_space, text),
                style,
            ))
        } else {
            // Split into spans: tree_prefix colored, text colored differently
            let mut spans = Vec::with_capacity(6);
            spans.push(Span::styled(cursor_char, Style::default()));
            if !tree_prefix.is_empty() {
                spans.push(Span::styled(tree_prefix.to_string(), Style::default().fg(line_color)));
            }
            spans.push(Span::styled(
                format!("{}{}", icon, icon_space),
                Style::default().fg(text_color),
            ));
            // Apply fuzzy match highlighting to text if positions provided
            spans.extend(highlight_matches(&text, match_positions, text_color));
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
        app.search_matches.get(&idx).map(|v| v.as_slice()),
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
                app.search_matches.get(&idx).map(|v| v.as_slice()),
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
                    let layer_key = format!("layer:{}", layer.key);
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
                        app.search_matches.get(&idx).map(|v| v.as_slice()),
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
                            let kind_is_empty = kind.instance_count == 0;
                            let icon = trait_icon(&kind.trait_name);
                            let (display_text, kind_text_color) = if is_data_mode {
                                let text = format!(
                                    "{} {} ({})",
                                    icon, kind.display_name, kind.instance_count
                                );
                                let color = if kind_is_empty {
                                    COLOR_MUTED_TEXT // Gray for empty kinds
                                } else {
                                    Color::White
                                };
                                (text, color)
                            } else {
                                (format!("{} {}", icon, kind.display_name), Color::White)
                            };

                            let prefix = format!(
                                "{}{}{}",
                                cont(realm_is_last),
                                cont(layer_is_last),
                                branch(kind_is_last)
                            );
                            all_lines.push(make_line(
                                idx,
                                app.tree_cursor,
                                focused,
                                &prefix,
                                kind_icon,
                                display_text,
                                layer_color,     // line_color: parent layer color
                                kind_text_color, // text_color (grayed if empty)
                                app.search_matches.get(&idx).map(|v| v.as_slice()),
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
                                            Style::default()
                                                .bg(COLOR_HIGHLIGHT_BG)
                                                .fg(Color::White)
                                        } else {
                                            Style::default().fg(base_color)
                                        };

                                        let cursor_char = if is_cursor { "›" } else { " " };
                                        let suffix = if is_primary && fallback_count > 0 {
                                            format!(" [{}↓]", fallback_count)
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
                                                    "{}{}{} {}{}",
                                                    cursor_char,
                                                    tree_prefix,
                                                    icon,
                                                    instance.display_name,
                                                    suffix
                                                ),
                                                style,
                                            )));
                                        } else {
                                            // Not selected: split into spans for colored tree lines
                                            all_lines.push(Line::from(vec![
                                                Span::styled(cursor_char, Style::default()),
                                                Span::styled(
                                                    tree_prefix,
                                                    Style::default().fg(layer_color),
                                                ),
                                                Span::styled(
                                                    format!("{} {}{}", icon, instance.display_name, suffix),
                                                    style,
                                                ),
                                            ]));
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
        app.search_matches.get(&idx).map(|v| v.as_slice()),
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
                app.search_matches.get(&idx).map(|v| v.as_slice()),
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
                        app.search_matches.get(&idx).map(|v| v.as_slice()),
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
        vec![Line::from(Span::styled(
            "  No data loaded",
            STYLE_DIM,
        ))]
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

    // Breadcrumb as bottom title (truncate if too long)
    let breadcrumb = app.current_breadcrumb();
    let max_breadcrumb_len = area.width.saturating_sub(4) as usize;
    let breadcrumb_display = if breadcrumb.len() > max_breadcrumb_len {
        format!("…{}", &breadcrumb[breadcrumb.len() - max_breadcrumb_len + 1..])
    } else {
        breadcrumb
    };

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

    // Get instances
    let instances = app.tree.get_instances(kind_key);
    let instance_count = instances.map(|i| i.len()).unwrap_or(0);
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
    let title = if instance_count > 0 {
        format!(
            " {} ({}/{}) [FILTERED] ",
            kind_display,
            app.tree_cursor + 1,
            instance_count
        )
    } else {
        format!(" {} (0) [FILTERED] ", kind_display)
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

    // Check if we should show a chart (Realm or Layer item)
    let show_bar_chart = matches!(app.current_item(), Some(TreeItem::Realm(_)));
    let show_sparkline = matches!(app.current_item(), Some(TreeItem::Layer(_, _)));

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

    // Show scroll indicator if scrollable
    let scroll_indicator = if app.info_line_count > visible_height {
        format!(
            " [{}/{}] ",
            app.info_scroll + 1,
            app.info_line_count.saturating_sub(visible_height) + 1
        )
    } else {
        String::new()
    };

    let block = Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            STYLE_PRIMARY,
        ))
        .title_bottom(Span::styled(
            scroll_indicator,
            STYLE_DIM,
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if focused {
            Color::Cyan
        } else {
            border_color
        }));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
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
            // Use first 3 chars of layer name as label
            let label = if layer.display_name.len() > 4 {
                layer.display_name[..4].to_string()
            } else {
                layer.display_name.clone()
            };
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
        .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .label_style(Style::default().fg(Color::Gray));

    f.render_widget(chart, area);
}

/// Graph panel: Displays Neo4j relationships for the selected Kind or Instance.
///
/// Shows real arc data from Neo4j when a Kind is selected,
/// instance arcs in Data mode, or contextual messages for other selections.
fn render_graph_panel(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.theme; // Use cached theme from App
    let focused = app.focus == Focus::Graph;
    let border_color = if focused {
        Color::Magenta
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Calculate arc count for title
    let arc_count = if let Some(ref arcs) = app.kind_arcs {
        arcs.incoming.len() + arcs.outgoing.len()
    } else if let Some(TreeItem::Instance(_, _, _, inst)) = app.current_item() {
        inst.outgoing_arcs.len() + inst.incoming_arcs.len()
    } else {
        0
    };

    // Build title with count
    let title = if arc_count > 0 {
        format!(" Arc Relationships ({}) ", arc_count)
    } else {
        " Arc Relationships ".to_string()
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();
    let dim = Style::default().fg(Color::Rgb(100, 100, 100));
    let bright_dim = STYLE_BRIGHT_DIM;

    // === LOADING INDICATOR (specific message based on what's loading) ===
    let loading_msg = if app.pending_arcs_load.is_some() {
        Some("Loading arc relationships...")
    } else if app.pending_arc_kind_load.is_some() {
        Some("Loading arc kind details...")
    } else if app.pending_realm_load.is_some() {
        Some("Loading realm statistics...")
    } else if app.pending_layer_load.is_some() {
        Some("Loading layer statistics...")
    } else {
        None
    };

    if let Some(msg) = loading_msg {
        lines.push(Line::from(Span::styled(
            format!("  {} {}", spinner(app.tick), msg),
            STYLE_HIGHLIGHT,
        )));
        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === REALM DETAILS VIEW ===
    if let Some(ref details) = app.realm_details {
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &details.display_name,
                Style::default()
                    .fg(theme.realm_color(&details.key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Realm)", STYLE_DIM),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Description (if any)
        if !details.description.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("  {}", details.description),
                Style::default().fg(Color::Rgb(180, 180, 180)),
            )));
            lines.push(Line::from(Span::raw("")));
        }

        // Stats summary
        lines.push(Line::from(vec![
            Span::styled("  ▪", dim),
            Span::styled(
                format!("{} Layers", details.layers.len()),
                STYLE_INFO,
            ),
            Span::styled(" · ", dim),
            Span::styled(
                format!("{} Node Kinds", details.total_kinds),
                STYLE_SUCCESS,
            ),
            Span::styled(" · ", dim),
            Span::styled(
                format!("{} Instances", details.total_instances),
                STYLE_HIGHLIGHT,
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Layers with kind counts (horizontal bar chart)
        lines.push(Line::from(Span::styled(
            "  LAYERS",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim,
        )));

        let max_kinds = details
            .layers
            .iter()
            .map(|l| l.kind_count)
            .max()
            .unwrap_or(1)
            .max(1);
        let bar_max_width = 20usize;

        for layer in &details.layers {
            let bar_width = (layer.kind_count * bar_max_width) / max_kinds;
            let bar = "█".repeat(bar_width.max(1));

            lines.push(Line::from(vec![
                Span::styled("    ", dim),
                Span::styled(
                    format!("{:16}", layer.display_name),
                    Style::default().fg(theme.layer_color(&layer.key)),
                ),
                Span::styled(format!("{:>3} ", layer.kind_count), bright_dim),
                Span::styled(bar, Style::default().fg(theme.layer_color(&layer.key))),
            ]));
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === LAYER DETAILS VIEW ===
    if let Some(ref details) = app.layer_details {
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &details.realm,
                Style::default().fg(theme.realm_color(&details.realm)),
            ),
            Span::styled(" → ", bright_dim),
            Span::styled(
                &details.display_name,
                Style::default()
                    .fg(theme.layer_color(&details.key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Layer)", STYLE_DIM),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Description (if any)
        if !details.description.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("  {}", details.description),
                Style::default().fg(Color::Rgb(180, 180, 180)),
            )));
            lines.push(Line::from(Span::raw("")));
        }

        // Stats summary
        lines.push(Line::from(vec![
            Span::styled("  ▪", dim),
            Span::styled(
                format!("{} Node Kinds", details.total_kinds),
                STYLE_SUCCESS,
            ),
            Span::styled(" · ", dim),
            Span::styled(
                format!("{} Instances", details.total_instances),
                STYLE_HIGHLIGHT,
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Node Kinds grouped by trait
        lines.push(Line::from(Span::styled(
            "  NODE KINDS BY TRAIT",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim,
        )));

        for group in &details.kinds_by_trait {
            // Trait header with color
            let trait_color = theme.trait_color(&group.trait_key);
            lines.push(Line::from(vec![
                Span::styled("    ", dim),
                Span::styled(
                    format!("{} ({})", group.trait_key, group.kind_names.len()),
                    Style::default()
                        .fg(trait_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            // Kind names
            for kind_name in &group.kind_names {
                lines.push(Line::from(vec![
                    Span::styled("      • ", dim),
                    Span::styled(
                        kind_name,
                        Style::default().fg(theme.layer_color(&details.key)),
                    ),
                ]));
            }
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === INSTANCE ARCS VIEW (Data mode) ===
    if let Some(TreeItem::Instance(realm, layer, kind, instance)) = app.current_item() {
        // Use references where possible, clone only when Span needs ownership
        let realm_key = &realm.key;
        let layer_key = &layer.key;
        let instance_key = &instance.key;

        // Breadcrumb for instance
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                realm_key.clone(),
                Style::default()
                    .fg(theme.realm_color(realm_key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" → ", bright_dim),
            Span::styled(
                layer_key.clone(),
                Style::default()
                    .fg(theme.layer_color(layer_key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" → ", bright_dim),
            Span::styled(kind.display_name.clone(), STYLE_SUCCESS),
            Span::styled(" → ", bright_dim),
            Span::styled(
                instance_key.clone(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Use references to arc vectors (no clone needed)
        let total = instance.outgoing_arcs.len() + instance.incoming_arcs.len();

        if total == 0 {
            lines.push(Line::from(Span::styled(
                "  No arc connections for this instance",
                STYLE_DIM,
            )));
        } else {
            // Outgoing arcs (iterate over reference, no clone of Vec)
            if !instance.outgoing_arcs.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("  ─▶ OUTGOING ({}) ", instance.outgoing_arcs.len()),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(
                    "  ─────────────────────────────────────────",
                    dim,
                )));

                for arc in &instance.outgoing_arcs {
                    let status_style = if arc.exists {
                        STYLE_SUCCESS
                    } else {
                        STYLE_HIGHLIGHT
                    };
                    let status_char = if arc.exists { "✓" } else { "○" };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", status_char), status_style),
                        Span::styled(instance_key.clone(), STYLE_PRIMARY),
                        Span::styled(" ──[", dim),
                        Span::styled(
                            arc.arc_type.clone(),
                            Style::default()
                                .fg(theme.arc_family_color("semantic"))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("]──▶ ", dim),
                        Span::styled(arc.target_key.clone(), STYLE_SUCCESS),
                        Span::styled(
                            format!(" ({})", arc.target_kind),
                            STYLE_DIM,
                        ),
                    ]));
                }
                lines.push(Line::from(Span::raw("")));
            }

            // Incoming arcs (iterate over reference, no clone of Vec)
            if !instance.incoming_arcs.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("  ◀─ INCOMING ({}) ", instance.incoming_arcs.len()),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(
                    "  ─────────────────────────────────────────",
                    dim,
                )));

                for arc in &instance.incoming_arcs {
                    let status_style = if arc.exists {
                        STYLE_SUCCESS
                    } else {
                        STYLE_HIGHLIGHT
                    };
                    let status_char = if arc.exists { "✓" } else { "○" };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", status_char), status_style),
                        Span::styled(arc.target_key.clone(), STYLE_SUCCESS),
                        Span::styled(
                            format!(" ({})", arc.target_kind),
                            STYLE_DIM,
                        ),
                        Span::styled(" ──[", dim),
                        Span::styled(
                            arc.arc_type.clone(),
                            Style::default()
                                .fg(theme.arc_family_color("semantic"))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("]──▶ ", dim),
                        Span::styled(instance_key.clone(), STYLE_PRIMARY),
                    ]));
                }
            }
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === KIND ARCS VIEW (from Neo4j) ===
    if let Some(ref arcs) = app.kind_arcs {
        // Hierarchy breadcrumb with theme colors
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &arcs.realm,
                Style::default()
                    .fg(theme.realm_color(&arcs.realm))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Realm)", STYLE_DIM),
            Span::styled(" → ", bright_dim),
            Span::styled(
                &arcs.layer,
                Style::default()
                    .fg(theme.layer_color(&arcs.layer))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Layer)", STYLE_DIM),
            Span::styled(" → ", bright_dim),
            Span::styled(
                &arcs.kind_label,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Node Kind)", STYLE_DIM),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Group all arcs by family
        render_arcs_by_family(&mut lines, arcs, theme, &dim);

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === ARCKIND DETAILS VIEW ===
    if let Some(ref details) = app.arc_kind_details {
        let family_color = theme.arc_family_color(&details.family);

        // Arc name with family
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &details.display_name,
                Style::default()
                    .fg(family_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  ({})", details.family),
                STYLE_DIM,
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // === ENDPOINTS ===
        lines.push(Line::from(Span::styled(
            "  ENDPOINTS",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim,
        )));

        // FROM endpoint with theme colors
        if let Some(ref from) = details.from_endpoint {
            lines.push(Line::from(vec![
                Span::styled("    FROM: ", STYLE_ACCENT),
                Span::styled(
                    &from.kind_label,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  (", STYLE_DIM),
                Span::styled(
                    &from.realm,
                    Style::default().fg(theme.realm_color(&from.realm)),
                ),
                Span::styled("/", STYLE_DIM),
                Span::styled(
                    &from.layer,
                    Style::default().fg(theme.layer_color(&from.layer)),
                ),
                Span::styled(")", STYLE_DIM),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("    FROM: ", STYLE_ACCENT),
                Span::styled("(not defined)", STYLE_DIM),
            ]));
        }

        // TO endpoint with theme colors
        if let Some(ref to) = details.to_endpoint {
            lines.push(Line::from(vec![
                Span::styled("    TO:   ", STYLE_INFO),
                Span::styled(
                    &to.kind_label,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  (", STYLE_DIM),
                Span::styled(&to.realm, Style::default().fg(theme.realm_color(&to.realm))),
                Span::styled("/", STYLE_DIM),
                Span::styled(&to.layer, Style::default().fg(theme.layer_color(&to.layer))),
                Span::styled(")", STYLE_DIM),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("    TO:   ", STYLE_INFO),
                Span::styled("(not defined)", STYLE_DIM),
            ]));
        }
        lines.push(Line::from(Span::raw("")));

        // === PROPERTIES ===
        lines.push(Line::from(Span::styled(
            "  PROPERTIES",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim,
        )));

        // Cardinality
        if !details.cardinality.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("    Cardinality: ", dim),
                Span::styled(&details.cardinality, STYLE_HIGHLIGHT),
            ]));
        }

        // Cypher pattern
        if !details.cypher_pattern.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("    Pattern: ", dim),
                Span::styled(
                    &details.cypher_pattern,
                    Style::default().fg(theme.arc_family_color("generation")),
                ),
            ]));
        }

        // Description
        if !details.description.is_empty() {
            lines.push(Line::from(Span::raw("")));
            lines.push(Line::from(Span::styled(
                "  DESCRIPTION",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                "  ─────────────────────────────────────────",
                dim,
            )));
            // Wrap description if too long
            for chunk in details.description.chars().collect::<Vec<_>>().chunks(45) {
                let line: String = chunk.iter().collect();
                lines.push(Line::from(Span::styled(
                    format!("    {}", line),
                    STYLE_DIM,
                )));
            }
        }
    } else {
        // No Neo4j data - show contextual message
        let msg = match app.current_item() {
            Some(TreeItem::KindsSection) | Some(TreeItem::ArcsSection) => {
                "▸ Expand a section to explore"
            }
            Some(TreeItem::ArcFamily(_)) => "▸ Select an Arc to see endpoints",
            Some(TreeItem::ArcKind(_, _)) => "▸ Loading arc details...",
            Some(TreeItem::Realm(_)) | Some(TreeItem::Layer(_, _)) => {
                "▸ Select a Node Kind to see arc relationships"
            }
            _ => "▸ Select a Node Kind or Arc to see details",
        };
        lines.push(Line::from(Span::styled(
            msg,
            STYLE_DIM,
        )));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

/// Render arcs grouped by family (instead of by direction).
fn render_arcs_by_family(
    lines: &mut Vec<Line>,
    arcs: &super::data::KindArcsData,
    theme: &theme::Theme,
    dim: &Style,
) {
    use std::collections::BTreeMap;

    // Collect all arcs grouped by family (clone data to avoid lifetime issues)
    let mut by_family: BTreeMap<String, Vec<(bool, String, String)>> = BTreeMap::new();

    for arc in &arcs.incoming {
        by_family.entry(arc.family.clone()).or_default().push((
            false,
            arc.arc_key.clone(),
            arc.other_kind.clone(),
        )); // false = incoming
    }
    for arc in &arcs.outgoing {
        by_family.entry(arc.family.clone()).or_default().push((
            true,
            arc.arc_key.clone(),
            arc.other_kind.clone(),
        )); // true = outgoing
    }

    if by_family.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No arc relationships defined for this Node Kind",
            STYLE_DIM,
        )));
        return;
    }

    let kind_label = arcs.kind_label.clone();

    // Render each family group
    for (family, family_arcs) in &by_family {
        let family_color = theme.arc_family_color(family);
        let incoming_count = family_arcs.iter().filter(|(is_out, _, _)| !is_out).count();
        let outgoing_count = family_arcs.iter().filter(|(is_out, _, _)| *is_out).count();

        // Family header with counts
        lines.push(Line::from(vec![
            Span::styled("  ", *dim),
            Span::styled(
                family.to_uppercase(),
                Style::default()
                    .fg(family_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  (◀{} ▶{})", incoming_count, outgoing_count),
                STYLE_DIM,
            ),
        ]));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            *dim,
        )));

        // Render arcs in this family
        for (is_outgoing, arc_key, other_kind) in family_arcs {
            if *is_outgoing {
                // Outgoing: Kind ──[ARC]──▶ Target
                lines.push(Line::from(vec![
                    Span::styled("    ", *dim),
                    Span::styled(kind_label.clone(), STYLE_PRIMARY),
                    Span::styled(" ──[", *dim),
                    Span::styled(
                        arc_key.clone(),
                        Style::default()
                            .fg(family_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("]──▶ ", *dim),
                    Span::styled(other_kind.clone(), STYLE_SUCCESS),
                ]));
            } else {
                // Incoming: Source ──[ARC]──▶ Kind
                lines.push(Line::from(vec![
                    Span::styled("    ", *dim),
                    Span::styled(other_kind.clone(), STYLE_SUCCESS),
                    Span::styled(" ──[", *dim),
                    Span::styled(
                        arc_key.clone(),
                        Style::default()
                            .fg(family_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("]──▶ ", *dim),
                    Span::styled(kind_label.clone(), STYLE_PRIMARY),
                ]));
            }
        }
        lines.push(Line::from(Span::raw("")));
    }
}


/// YAML panel: displays YAML source with independent scroll.
fn render_yaml_panel(f: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Yaml;
    let visible_height = area.height.saturating_sub(2) as usize;

    // Check if we're in Data mode on an Instance → show JSON instead of YAML
    let is_json_mode = app.mode == NavMode::Data
        && matches!(app.current_item(), Some(TreeItem::Instance(_, _, _, _)));

    if is_json_mode {
        render_json_panel(f, area, app, focused, visible_height);
    } else {
        render_yaml_content(f, area, app, focused, visible_height);
    }
}

/// Render JSON panel for Instance data in Data mode.
fn render_json_panel(f: &mut Frame, area: Rect, app: &App, focused: bool, visible_height: usize) {
    // Cyan border for JSON mode
    let border_color = if focused {
        Color::Cyan
    } else {
        Color::Rgb(60, 130, 130) // Dimmed cyan when unfocused
    };

    let (title, json_lines, total_lines) =
        if let Some(TreeItem::Instance(_, _, _, inst)) = app.current_item() {
            let all_lines = inst.to_colored_json();
            let total = all_lines.len();
            let visible: Vec<Line> = all_lines
                .into_iter()
                .skip(app.yaml_scroll)
                .take(visible_height)
                .collect();
            (format!(" {} ", inst.key), visible, total)
        } else {
            (
                " JSON ".to_string(),
                vec![Line::from(Span::styled(
                    "No instance data",
                    STYLE_DIM,
                ))],
                1,
            )
        };

    // Scroll indicator
    let scroll_indicator = if total_lines > visible_height {
        format!(
            " [{}/{}] ",
            app.yaml_scroll + 1,
            total_lines.saturating_sub(visible_height) + 1
        )
    } else {
        String::new()
    };

    let block = Block::default()
        .title(Span::styled(title, STYLE_INFO))
        .title_bottom(Span::styled(
            scroll_indicator,
            STYLE_DIM,
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(json_lines).block(block);
    f.render_widget(paragraph, area);
}

/// Render YAML panel (original behavior for Meta mode and Kind selection).
fn render_yaml_content(f: &mut Frame, area: Rect, app: &App, focused: bool, visible_height: usize) {
    let border_color = if focused {
        Color::Green
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Build YAML lines with syntax highlighting
    let mut lines: Vec<Line> = Vec::new();

    if !app.yaml_content.is_empty() {
        for yaml_line in app
            .yaml_content
            .lines()
            .skip(app.yaml_scroll)
            .take(visible_height)
        {
            lines.push(highlight_yaml_line(yaml_line));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "No YAML file",
            STYLE_DIM,
        )));
    }

    // Build title with colored path
    let title_spans = build_yaml_title(&app.yaml_path);

    // Show scroll indicator
    let total_lines = app.yaml_content.lines().count();
    let scroll_indicator = if total_lines > visible_height {
        format!(
            " [{}/{}] ",
            app.yaml_scroll + 1,
            total_lines.saturating_sub(visible_height) + 1
        )
    } else {
        String::new()
    };

    let block = Block::default()
        .title(Line::from(title_spans))
        .title_bottom(Span::styled(
            scroll_indicator,
            STYLE_DIM,
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Build YAML panel title with colored path segments.
fn build_yaml_title(path: &str) -> Vec<Span<'static>> {
    if path.is_empty() {
        return vec![Span::styled(
            " YAML ",
            STYLE_UNFOCUSED,
        )];
    }

    let mut spans = vec![Span::styled(" ", Style::default())];
    spans.extend(colorize_path_inline(path));
    spans.push(Span::styled(" ", Style::default()));
    spans
}

/// Get title for detail panel based on current selection.
fn get_detail_title(app: &App) -> String {
    match app.current_item() {
        Some(TreeItem::KindsSection) => "Node Kinds".to_string(),
        Some(TreeItem::ArcsSection) => "Arcs".to_string(),
        Some(TreeItem::Realm(r)) => format!("{} {}", r.icon, r.display_name),
        Some(TreeItem::Layer(_, l)) => l.display_name.clone(),
        Some(TreeItem::Kind(_, _, k)) => {
            if k.icon.is_empty() {
                k.display_name.clone()
            } else {
                format!("{} {}", k.icon, k.display_name)
            }
        }
        Some(TreeItem::ArcFamily(f)) => f.display_name.clone(),
        Some(TreeItem::ArcKind(_, ek)) => ek.display_name.clone(),
        Some(TreeItem::Instance(_, _, _, inst)) => {
            format!("{} ({})", inst.display_name, inst.kind_key)
        }
        None => "Detail".to_string(),
    }
}

/// Colorize path inline for title.
fn colorize_path_inline(path: &str) -> Vec<Span<'static>> {
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
            _ => Color::White,              // filename
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
            let kind_count: usize = app
                .tree
                .realms
                .iter()
                .flat_map(|r| r.layers.iter())
                .map(|l| l.kinds.len())
                .sum();
            vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Section", STYLE_ACCENT),
                ]),
                Line::from(vec![
                    Span::styled("realms    ", STYLE_DIM),
                    Span::styled(
                        app.tree.realms.len().to_string(),
                        STYLE_PRIMARY,
                    ),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", STYLE_DIM),
                    Span::styled(kind_count.to_string(), STYLE_PRIMARY),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "h/l to collapse/expand",
                    STYLE_DIM,
                )),
            ]
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
                    Span::styled(
                        app.tree.arc_families.len().to_string(),
                        STYLE_PRIMARY,
                    ),
                ]),
                Line::from(vec![
                    Span::styled("arcs ", STYLE_DIM),
                    Span::styled(arc_count.to_string(), STYLE_PRIMARY),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "h/l to collapse/expand",
                    STYLE_DIM,
                )),
            ]
        }
        Some(TreeItem::Realm(realm)) => {
            let kind_count: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
            vec![
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
                    Span::styled(
                        realm.layers.len().to_string(),
                        STYLE_PRIMARY,
                    ),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", STYLE_DIM),
                    Span::styled(kind_count.to_string(), STYLE_PRIMARY),
                ]),
            ]
        }
        Some(TreeItem::Layer(realm, layer)) => {
            vec![
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
                    Span::styled(
                        layer.kinds.len().to_string(),
                        STYLE_PRIMARY,
                    ),
                ]),
            ]
        }
        Some(TreeItem::Kind(realm, layer, kind)) => {
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Node Kind", STYLE_INFO),
                ]),
                Line::from(vec![
                    Span::styled("key       ", STYLE_DIM),
                    Span::styled(kind.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("realm     ", STYLE_DIM),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(hex_to_color(&realm.color)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("layer     ", STYLE_DIM),
                    Span::styled(
                        layer.display_name.clone(),
                        Style::default().fg(hex_to_color(&layer.color)),
                    ),
                ]),
            ];

            // Trait (if present)
            if !kind.trait_name.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("trait     ", STYLE_DIM),
                    Span::styled(kind.trait_name.clone(), STYLE_ACCENT),
                ]));
            }

            // v10.1: knowledge_tier removed from display (node type is sufficient)

            lines.push(Line::from(vec![
                Span::styled("instances ", STYLE_DIM),
                Span::styled(
                    kind.instance_count.to_string(),
                    STYLE_PRIMARY,
                ),
            ]));

            // Context budget (if present)
            if !kind.context_budget.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("budget    ", STYLE_DIM),
                    Span::styled(
                        kind.context_budget.clone(),
                        STYLE_INFO,
                    ),
                ]));
            }

            // Properties section (ALL properties with required markers)
            if !kind.properties.is_empty() {
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
                lines.push(Line::from(Span::styled(
                    "  * = required",
                    STYLE_DIM,
                )));
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
                lines.push(Line::from(Span::styled(
                    "Description",
                    STYLE_MUTED,
                )));
                // Wrap description to multiple lines if too long
                let desc = &kind.description;
                for chunk in desc.chars().collect::<Vec<_>>().chunks(60) {
                    let line: String = chunk.iter().collect();
                    lines.push(Line::from(Span::styled(
                        format!("  {}", line),
                        STYLE_DESC,
                    )));
                }
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Cypher",
                STYLE_MUTED,
            )));
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
                    Span::styled("arcs ", STYLE_DIM),
                    Span::styled(
                        family.arc_kinds.len().to_string(),
                        STYLE_PRIMARY,
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "h/l to collapse/expand",
                    STYLE_DIM,
                )),
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
                    Span::styled(
                        family.display_name.clone(),
                        STYLE_ARC_FAMILY,
                    ),
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
                    Span::styled(
                        arc_kind.cardinality.clone(),
                        STYLE_ACCENT,
                    ),
                ]));
            }

            // Description (if present)
            if !arc_kind.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "Description",
                    STYLE_MUTED,
                )));
                lines.push(Line::from(Span::styled(
                    format!("  {}", &arc_kind.description),
                    STYLE_DESC,
                )));
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Cypher",
                STYLE_MUTED,
            )));
            lines.push(Line::from(Span::styled(
                format!("  MATCH ()-[r:{}]->() RETURN r LIMIT 100", arc_kind.key),
                STYLE_HINT,
            )));

            lines
        }
        Some(TreeItem::Instance(_realm, _layer, kind, instance)) => {
            // Instance info for Data view
            let mut lines: Vec<Line<'static>> = Vec::new();

            // Header
            lines.push(Line::from(vec![
                Span::styled("type      ", STYLE_DIM),
                Span::styled("Instance", STYLE_SUCCESS),
            ]));
            lines.push(Line::from(vec![
                Span::styled("key       ", STYLE_DIM),
                Span::styled(instance.key.clone(), STYLE_PRIMARY),
            ]));
            lines.push(Line::from(vec![
                Span::styled("kind      ", STYLE_DIM),
                Span::styled(kind.display_name.clone(), STYLE_INFO),
            ]));

            // Properties
            if !instance.properties.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Properties ({})", instance.properties.len()),
                    STYLE_MUTED,
                )));
                for (key, value) in &instance.properties {
                    // Format JSON value for display
                    let value_str = match value {
                        serde_json::Value::String(s) => format!("\"{}\"", s),
                        serde_json::Value::Null => "null".to_string(),
                        _ => value.to_string(),
                    };
                    let truncated = truncate_str(&value_str, 40);
                    let color = match value {
                        serde_json::Value::String(_) => Color::Green,
                        serde_json::Value::Number(_) => Color::Yellow,
                        serde_json::Value::Bool(_) => Color::Yellow,
                        serde_json::Value::Null => Color::DarkGray,
                        _ => Color::White,
                    };
                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", key), STYLE_INFO),
                        Span::styled(truncated, Style::default().fg(color)),
                    ]));
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

                // Box drawing for instance node (use char count for proper alignment)
                let key_width = instance.key.chars().count();
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
                            Span::styled(
                                format!("[{}]", cmp.arc_type),
                                STYLE_HIGHLIGHT,
                            ),
                            Span::styled("══> ", STYLE_SUCCESS),
                            Span::styled(target_display, STYLE_PRIMARY),
                            Span::styled(" ✓", STYLE_SUCCESS),
                        ]));
                    } else {
                        // Missing arc: dashed line (╌╌)
                        lines.push(Line::from(vec![
                            Span::styled("    ╌╌", STYLE_ERROR),
                            Span::styled(
                                format!("[{}]", cmp.arc_type),
                                STYLE_DIM,
                            ),
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
            vec![Line::from(Span::styled(
                "Select an item",
                STYLE_DIM,
            ))]
        }
    }
}

/// Highlight a YAML line with syntax coloring.
/// Uses const STYLE_YAML_* for efficiency (avoids Style recreation per line).
fn highlight_yaml_line(line: &str) -> Line<'static> {
    // Comment line
    if line.trim_start().starts_with('#') {
        return Line::from(Span::styled(line.to_string(), STYLE_YAML_COMMENT));
    }

    // Empty line
    if line.trim().is_empty() {
        return Line::from(Span::raw(line.to_string()));
    }

    // Key-value or list item (most lines have 2-4 spans)
    let mut spans: Vec<Span<'static>> = Vec::with_capacity(4);

    // Find leading whitespace
    let indent_len = line.len() - line.trim_start().len();
    let indent = &line[..indent_len];
    let rest = &line[indent_len..];

    spans.push(Span::raw(indent.to_string()));

    // Check for list item
    if rest.starts_with("- ") {
        spans.push(Span::styled("-", STYLE_YAML_PUNCT));
        let after_dash = &rest[1..];

        // Check if it's a key-value after dash
        if let Some(colon_pos) = after_dash.find(':') {
            let key = &after_dash[..colon_pos + 1];
            let value = &after_dash[colon_pos + 1..];
            spans.push(Span::styled(key.to_string(), STYLE_YAML_KEY));
            spans.push(highlight_yaml_value(value));
        } else {
            spans.push(highlight_yaml_value(after_dash));
        }
    } else if let Some(colon_pos) = rest.find(':') {
        // Key-value pair
        let key = &rest[..colon_pos];
        let colon_and_rest = &rest[colon_pos..];

        spans.push(Span::styled(key.to_string(), STYLE_YAML_KEY));

        if colon_and_rest.len() > 1 {
            spans.push(Span::styled(":", STYLE_YAML_TEXT));
            let value = &colon_and_rest[1..];
            spans.push(highlight_yaml_value(value));
        } else {
            spans.push(Span::styled(":", STYLE_YAML_TEXT));
        }
    } else {
        // Plain text
        spans.push(Span::styled(rest.to_string(), STYLE_YAML_TEXT));
    }

    Line::from(spans)
}

/// Highlight a YAML value with appropriate color.
/// Uses const STYLE_YAML_* for efficiency.
fn highlight_yaml_value(value: &str) -> Span<'static> {
    let trimmed = value.trim();

    // Boolean
    if trimmed == "true" || trimmed == "false" {
        return Span::styled(value.to_string(), STYLE_YAML_LITERAL);
    }

    // Null
    if trimmed == "null" || trimmed == "~" {
        return Span::styled(value.to_string(), STYLE_YAML_LITERAL);
    }

    // Number
    if trimmed.parse::<f64>().is_ok() {
        return Span::styled(value.to_string(), STYLE_YAML_NUMBER);
    }

    // String (quoted or unquoted)
    Span::styled(value.to_string(), STYLE_YAML_STRING)
}

/// Status bar: enriched with mode indicator, breadcrumb, shortcuts, spinner.
fn render_status(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.theme;

    // Mode indicator with icon and color
    let mode_label = app.mode.label();
    let mode_icon = theme.nav_mode_icon(mode_label);
    let mode_style = theme.nav_mode_style(mode_label);

    // Breadcrumb (truncated if too long)
    let breadcrumb = app.current_breadcrumb();
    let max_breadcrumb_len = (area.width as usize).saturating_sub(60).min(40);
    let breadcrumb_display = if breadcrumb.len() > max_breadcrumb_len {
        format!("…{}", &breadcrumb[breadcrumb.len().saturating_sub(max_breadcrumb_len)..])
    } else {
        breadcrumb
    };

    // Contextual shortcuts based on mode and focus
    let shortcuts = match app.mode {
        NavMode::Atlas => "j/k:nav  1-4:modes  /:help",
        NavMode::Data => "j/k:nav  h/l:toggle  0:hide∅  /:help",
        NavMode::Query => "j/k:nav  f:filter  /:help",
        _ => match app.focus {
            Focus::Tree => "j/k:nav  h/l:toggle  H/L:all  /:help",
            Focus::Yaml | Focus::Info => "j/k:scroll  d/u:page  g/G:jump",
            Focus::Graph => "Tab:panel  1-5:modes",
        },
    };

    // Build status line spans
    let mut spans = vec![
        // Mode indicator: [◈ META]
        Span::raw(" "),
        Span::styled(format!("{} {}", mode_icon, mode_label.to_uppercase()), mode_style),
        Span::styled(" │ ", STYLE_SEPARATOR),
        // Breadcrumb
        Span::styled(breadcrumb_display, STYLE_HINT),
    ];

    // Loading spinner (if pending load)
    if app.has_pending_load() {
        spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
        spans.push(Span::styled(
            format!("{} Loading…", app.spinner_frame()),
            Style::default().fg(Color::Yellow),
        ));
    }

    // Spacer to push shortcuts to the right
    spans.push(Span::raw("  "));

    // Stats (condensed: nodes·arcs | kinds·arc-kinds)
    let stats = &app.tree.stats;
    spans.push(Span::styled(
        format!(
            "{}n·{}a │ {}K·{}A",
            stats.node_count, stats.arc_count, stats.kind_count, stats.arc_kind_count
        ),
        STYLE_MUTED,
    ));

    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));

    // Contextual shortcuts
    spans.push(Span::styled(shortcuts, STYLE_DIM));

    spans.push(Span::raw(" "));

    let status = Line::from(spans);
    let paragraph = Paragraph::new(status).style(Style::default().bg(Color::Rgb(15, 15, 20)));

    f.render_widget(paragraph, area);
}

/// Search overlay: input + results.
fn render_search(f: &mut Frame, app: &App) {
    // Center the search box
    let area = f.area();
    let width = 50.min(area.width.saturating_sub(4));
    let height = 12.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 3; // Slightly above center

    let search_area = Rect::new(x, y, width, height);

    // Clear the area behind the overlay
    f.render_widget(Clear, search_area);

    // Build content
    let mut lines: Vec<Line> = Vec::new();

    // Input line with cursor
    lines.push(Line::from(vec![
        Span::styled(" > ", STYLE_INFO),
        Span::styled(&app.search_query, STYLE_PRIMARY),
        Span::styled("█", STYLE_INFO), // Cursor
    ]));

    lines.push(Line::from(""));

    // Results count
    let count_text = if app.search_results.is_empty() {
        if app.search_query.is_empty() {
            "Type to search...".to_string()
        } else {
            "No results".to_string()
        }
    } else {
        format!("{} results", app.search_results.len())
    };
    lines.push(Line::from(Span::styled(
        count_text,
        STYLE_DIM,
    )));

    lines.push(Line::from(""));

    // Results list with scroll window around cursor
    let max_visible = 8;
    let total_results = app.search_results.len();

    // Calculate scroll window to keep cursor visible
    let start = if total_results <= max_visible || app.search_cursor < max_visible / 2 {
        0
    } else if app.search_cursor > total_results - max_visible / 2 {
        total_results.saturating_sub(max_visible)
    } else {
        app.search_cursor.saturating_sub(max_visible / 2)
    };

    let visible_results = app.search_results.iter().skip(start).take(max_visible);
    for (i, &idx) in visible_results.enumerate() {
        let actual_idx = start + i;
        let is_selected = actual_idx == app.search_cursor;
        let item = app.tree.item_at(idx);

        let (prefix, name, type_label) = match item {
            Some(TreeItem::KindsSection) => ("", "Node Kinds".to_string(), "Section"),
            Some(TreeItem::ArcsSection) => ("", "Arcs".to_string(), "Section"),
            Some(TreeItem::Realm(r)) => (r.icon, r.display_name.clone(), "Realm"),
            Some(TreeItem::Layer(_, l)) => ("  ", l.display_name.clone(), "Layer"),
            Some(TreeItem::Kind(_, _, k)) => ("    ", k.display_name.clone(), "Node Kind"),
            Some(TreeItem::ArcFamily(f)) => ("  ", f.display_name.clone(), "ArcFamily"),
            Some(TreeItem::ArcKind(_, ek)) => ("    ", ek.display_name.clone(), "Arc Kind"),
            Some(TreeItem::Instance(_, _, _, inst)) => {
                ("      ", inst.display_name.clone(), "Instance")
            }
            None => ("?", "Unknown".to_string(), ""),
        };

        let style = if is_selected {
            Style::default().bg(Color::Rgb(30, 50, 70)).fg(Color::White)
        } else {
            STYLE_DESC
        };

        let type_style = if is_selected {
            Style::default()
                .bg(Color::Rgb(30, 50, 70))
                .fg(Color::DarkGray)
        } else {
            STYLE_DIM
        };

        lines.push(Line::from(vec![
            Span::styled(format!(" {}{}", prefix, name), style),
            Span::styled(format!("  {}", type_label), type_style),
        ]));
    }

    let block = Block::default()
        .title(Span::styled(" Search ", STYLE_INFO))
        .borders(Borders::ALL)
        .border_style(STYLE_INFO)
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, search_area);
}

/// Help overlay: keyboard shortcuts.
fn render_help(f: &mut Frame) {
    let area = f.area();
    let width = 50.min(area.width.saturating_sub(4));
    let height = 32.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let help_area = Rect::new(x, y, width, height);
    f.render_widget(Clear, help_area);

    let lines = vec![
        Line::from(Span::styled(
            " NovaNet TUI — Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Navigation",
            STYLE_HIGHLIGHT,
        )]),
        Line::from(vec![
            Span::styled("    Tab      ", STYLE_PRIMARY),
            Span::styled(
                "Cycle: Tree→Info→Graph→YAML",
                STYLE_DIM,
            ),
        ]),
        Line::from(vec![
            Span::styled("    ←→       ", STYLE_PRIMARY),
            Span::styled("Quick panel switch", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    j/k ↑↓   ", STYLE_PRIMARY),
            Span::styled("Move cursor / scroll", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Tree (vim-style)",
            STYLE_HIGHLIGHT,
        )]),
        Line::from(vec![
            Span::styled("    h/l      ", STYLE_PRIMARY),
            Span::styled("Collapse/expand node", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    H/L      ", STYLE_PRIMARY),
            Span::styled("Collapse/expand all", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    g/G      ", STYLE_PRIMARY),
            Span::styled("Jump to first/last", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Graph panel",
            STYLE_ACCENT,
        )]),
        Line::from(vec![
            Span::styled("    j/k ↑↓   ", STYLE_PRIMARY),
            Span::styled("Select neighbor node", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    h/l ←→   ", STYLE_PRIMARY),
            Span::styled(
                "Navigate incoming/outgoing",
                STYLE_DIM,
            ),
        ]),
        Line::from(vec![
            Span::styled("    Enter    ", STYLE_PRIMARY),
            Span::styled(
                "Jump to selected node",
                STYLE_DIM,
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Scrolling",
            STYLE_HIGHLIGHT,
        )]),
        Line::from(vec![
            Span::styled("    d/u      ", STYLE_PRIMARY),
            Span::styled("Page down/up", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Modes",
            STYLE_HIGHLIGHT,
        )]),
        Line::from(vec![
            Span::styled("    1-4      ", STYLE_PRIMARY),
            Span::styled(
                "Meta/Data/Overlay/Query",
                STYLE_DIM,
            ),
        ]),
        Line::from(vec![
            Span::styled("    N        ", STYLE_PRIMARY),
            Span::styled("Cycle through modes", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Actions",
            STYLE_HIGHLIGHT,
        )]),
        Line::from(vec![
            Span::styled("    f        ", STYLE_PRIMARY),
            Span::styled("Find / search", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    /        ", STYLE_PRIMARY),
            Span::styled("Show this help", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    q        ", STYLE_PRIMARY),
            Span::styled("Quit", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Press any key to close",
            STYLE_DIM,
        )),
    ];

    let block = Block::default()
        .title(Span::styled(" Help ", STYLE_ACCENT))
        .borders(Borders::ALL)
        .border_style(STYLE_ACCENT)
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, help_area);
}

/// Atlas mode: interactive architecture visualizations.
fn render_atlas(f: &mut Frame, area: Rect, app: &mut App) {
    use super::atlas::AtlasView;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    // View selector tabs
    let view_tabs: Vec<Line> = AtlasView::all()
        .iter()
        .map(|v| {
            let is_selected = v == &app.atlas.current_view;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                STYLE_DIM
            };
            Line::from(vec![
                Span::styled(format!("[{}] ", v.shortcut()), style),
                Span::styled(v.label(), style),
            ])
        })
        .collect();

    let tabs_text = view_tabs
        .into_iter()
        .map(|l| l.to_string())
        .collect::<Vec<_>>()
        .join("  ");

    let tabs_block = Block::default()
        .title(Span::styled(
            " Atlas Mode ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(STYLE_ACCENT);

    let tabs_paragraph = Paragraph::new(tabs_text).block(tabs_block);
    f.render_widget(tabs_paragraph, chunks[0]);

    // Main content area
    let content_block = Block::default()
        .title(Span::styled(
            format!(" {} ", app.atlas.current_view.label()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(STYLE_INFO);

    let content = match app.atlas.current_view {
        AtlasView::RealmMap => render_atlas_realm_map(app),
        AtlasView::SpreadingActivation => render_atlas_spreading_activation(app),
        AtlasView::KnowledgeAtoms => render_atlas_knowledge_atoms(app),
        AtlasView::GenerationPipeline => render_atlas_generation_pipeline(app),
        AtlasView::ViewTraversal => render_atlas_view_traversal(app),
        AtlasView::PageComposition => render_atlas_page_composition(app),
    };

    let content_paragraph = Paragraph::new(content)
        .block(content_block)
        .wrap(Wrap { trim: true });
    f.render_widget(content_paragraph, chunks[1]);
}

/// Render the Realm Map view in Atlas mode.
fn render_atlas_realm_map(app: &App) -> String {
    let mut lines = Vec::new();

    // Mode indicator
    let mode_label = if app.atlas.demo_mode {
        "[d] DEMO MODE — Press [d] for live data"
    } else {
        "[d] LIVE MODE — Press [d] for demo"
    };
    lines.push(format!("  {}  |  j/k: navigate  Enter: zoom", mode_label));
    lines.push(String::new());

    // Use live data if available, otherwise demo
    if let Some(ref stats) = app.atlas.realm_stats {
        render_realm_map_live(
            &mut lines,
            stats,
            app.atlas.realm_cursor,
            app.atlas.realm_zoomed,
        );
    } else if app.atlas.demo_mode {
        render_realm_map_demo(&mut lines, app.atlas.realm_cursor);
    } else {
        lines.push("Loading realm statistics from Neo4j...".to_string());
    }

    lines.join("\n")
}

/// Render Realm Map with live Neo4j data.
fn render_realm_map_live(
    lines: &mut Vec<String>,
    stats: &super::data::AtlasRealmStats,
    cursor: usize,
    zoomed: bool,
) {
    let total_items = stats
        .realms
        .iter()
        .map(|r| 1 + r.layers.len())
        .sum::<usize>();
    lines.push(
        "╔═══════════════════════════════════════════════════════════════════════════╗".to_string(),
    );
    lines.push(format!(
        "║  2-REALM ARCHITECTURE (v10.6)        {} NodeKinds total        ║",
        stats.total_kinds
    ));
    lines.push(
        "╠═══════════════════════════════════════════════════════════════════════════╣".to_string(),
    );

    let mut item_index = 0;
    for realm in &stats.realms {
        let is_realm_selected = cursor == item_index;
        let realm_prefix = if is_realm_selected { "▶" } else { " " };
        let realm_style = if realm.key == "global" {
            "READ-ONLY"
        } else {
            "per-org"
        };

        lines.push(
            "║                                                                           ║"
                .to_string(),
        );
        lines.push(format!(
            "║  {} ┌─ {} ({}) ─────────────────────── {} kinds ─────┐  ║",
            realm_prefix,
            realm.display_name.to_uppercase(),
            realm_style,
            realm.total_kinds
        ));

        item_index += 1;

        for layer in &realm.layers {
            let is_layer_selected = cursor == item_index;
            let layer_prefix = if is_layer_selected { "▶" } else { " " };
            let expanded = if zoomed && is_layer_selected {
                " [expanded]"
            } else {
                ""
            };

            // Pad layer name to align counts
            let padded_name = format!("{:<20}", layer.display_name);
            lines.push(format!(
                "║  {}  │  {} {:>3} kinds{}",
                layer_prefix, padded_name, layer.kind_count, expanded
            ));

            // If zoomed into this layer, show more detail
            if zoomed && is_layer_selected {
                lines.push("║     │    └─ (press Enter to see Kind list)".to_string());
            }

            item_index += 1;
        }

        lines.push(
            "║    └─────────────────────────────────────────────────────────────────┘  ║"
                .to_string(),
        );
    }

    // Arrow between realms
    if stats.realms.len() > 1 {
        lines.push(
            "║                              │                                          ║"
                .to_string(),
        );
        lines.push(
            "║                              ▼ cross_realm arcs                        ║"
                .to_string(),
        );
    }

    lines.push(
        "║                                                                           ║".to_string(),
    );
    lines.push(
        "╚═══════════════════════════════════════════════════════════════════════════╝".to_string(),
    );

    // Navigation hint
    lines.push(String::new());
    lines.push(format!(
        "  Cursor: {}/{} │ Press Enter to {} │ Press [d] to toggle demo mode",
        cursor + 1,
        total_items,
        if zoomed { "collapse" } else { "expand" }
    ));
}

/// Render Realm Map in demo mode (static example).
fn render_realm_map_demo(lines: &mut Vec<String>, cursor: usize) {
    lines.push(
        "╔═══════════════════════════════════════════════════════════════════════════╗".to_string(),
    );
    lines.push(
        "║  2-REALM ARCHITECTURE (v10.6)             DEMO DATA                       ║".to_string(),
    );
    lines.push(
        "╠═══════════════════════════════════════════════════════════════════════════╣".to_string(),
    );
    lines.push(
        "║                                                                           ║".to_string(),
    );

    let global_selected = cursor == 0;
    let g_prefix = if global_selected { "▶" } else { " " };
    lines.push(format!(
        "║  {} ┌─ GLOBAL (READ-ONLY) ───────────────────── 24 kinds ──────┐          ║",
        g_prefix
    ));
    lines.push(
        "║    │  config              2 kinds   (Taxonomy, VisualEncoding)│          ║".to_string(),
    );
    lines.push(
        "║    │  locale-knowledge   12 kinds   (Locale, TermSet, Term...)│          ║".to_string(),
    );
    lines.push(
        "║    │  seo                 4 kinds   (SEOKeyword, Metrics...)  │          ║".to_string(),
    );
    lines.push(
        "║    └──────────────────────────────────────────────────────────┘          ║".to_string(),
    );
    lines.push(
        "║                              │                                          ║".to_string(),
    );
    lines.push(
        "║                              ▼                                          ║".to_string(),
    );

    let tenant_selected = cursor == 1;
    let t_prefix = if tenant_selected { "▶" } else { " " };
    lines.push(format!(
        "║  {} ┌─ TENANT (per-organization) ───────────── 22 kinds ──────┐          ║",
        t_prefix
    ));
    lines.push(
        "║    │  foundation          4 kinds   (Entity, EntityL10n...)  │          ║".to_string(),
    );
    lines.push(
        "║    │  structure           4 kinds   (Page, Block, PageL10n.) │          ║".to_string(),
    );
    lines.push(
        "║    │  semantic            3 kinds   (Knowledge atom usage)   │          ║".to_string(),
    );
    lines.push(
        "║    │  instruction         4 kinds   (Prompt, GenerationJob)  │          ║".to_string(),
    );
    lines.push(
        "║    │  output              3 kinds   (BlockL10n, PageSEO...)  │          ║".to_string(),
    );
    lines.push(
        "║    └──────────────────────────────────────────────────────────┘          ║".to_string(),
    );
    lines.push(
        "║                                                                           ║".to_string(),
    );
    lines.push(
        "╚═══════════════════════════════════════════════════════════════════════════╝".to_string(),
    );

    // Legend
    lines.push(String::new());
    lines.push("  DEMO: Static example data │ Press [d] to load live data from Neo4j".to_string());
}

/// Render the Knowledge Atoms view in Atlas mode.
/// Shows selective loading vs monolithic blob approach.
fn render_atlas_knowledge_atoms(app: &App) -> String {
    let mut lines = Vec::new();

    // Header
    let mode = if app.atlas.demo_mode { "DEMO" } else { "LIVE" };
    lines.push(format!(
        "  [{}]  |  Knowledge Atoms: Selective vs Monolithic Loading",
        mode
    ));
    lines.push(String::new());

    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(
        "║  KNOWLEDGE ATOMS ARCHITECTURE                                              ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // Problem: Monolithic approach
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  ❌ MONOLITHIC APPROACH (Traditional)                                       ║"
            .to_string(),
    );
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Locale                                                                  ║"
            .to_string(),
    );
    lines.push(
        "║      └── knowledge_data: {                                                 ║"
            .to_string(),
    );
    lines.push(
        "║            \"terms\": [... 20,000 entries ...],      ← 2MB JSON blob        ║"
            .to_string(),
    );
    lines.push(
        "║            \"expressions\": [... 5,000 entries ...],                        ║"
            .to_string(),
    );
    lines.push(
        "║            \"patterns\": [... 1,000 entries ...],                           ║"
            .to_string(),
    );
    lines.push(
        "║          }                                                                 ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Problems:                                                               ║"
            .to_string(),
    );
    lines.push(
        "║    • Load 2MB to use 50 terms                                              ║"
            .to_string(),
    );
    lines.push(
        "║    • Can't query: \"Which terms does this Block use?\"                       ║"
            .to_string(),
    );
    lines.push(
        "║    • Can't trace: \"Which Blocks use 'conversion' term?\"                    ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Solution: Knowledge Atoms
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  ✅ KNOWLEDGE ATOMS (NovaNet)                                               ║"
            .to_string(),
    );
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Locale ──[:HAS_TERMS]──> TermSet ──[:CONTAINS]──> Term                  ║"
            .to_string(),
    );
    lines.push(
        "║           ──[:HAS_EXPRESSIONS]──> ExpressionSet ──[:CONTAINS]──> Expression║"
            .to_string(),
    );
    lines.push(
        "║           ──[:HAS_PATTERNS]──> PatternSet ──[:CONTAINS]──> Pattern         ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Block ──[:USES_TERM]──> Term                                            ║"
            .to_string(),
    );
    lines.push(
        "║          ──[:USES_EXPRESSION]──> Expression                                ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Benefits:                                                               ║"
            .to_string(),
    );
    lines.push(
        "║    • Load 50 relevant terms, not 20K blob                                  ║"
            .to_string(),
    );
    lines.push(
        "║    • Query: MATCH (b:Block)-[:USES_TERM]->(t:Term) WHERE b.key = $key      ║"
            .to_string(),
    );
    lines.push(
        "║    • Trace: MATCH (t:Term)<-[:USES_TERM]-(b:Block) WHERE t.term = $term    ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Atom types
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  ATOM TYPES (6 Sets + 6 Atoms = 12 NodeKinds)                              ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  ┌─────────────────┬─────────────────┬──────────────────────────────────┐  ║"
            .to_string(),
    );
    lines.push(
        "║  │ Container (Set) │ Atom            │ Purpose                          │  ║"
            .to_string(),
    );
    lines.push(
        "║  ├─────────────────┼─────────────────┼──────────────────────────────────┤  ║"
            .to_string(),
    );
    lines.push(
        "║  │ TermSet         │ Term            │ Vocabulary, definitions          │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ ExpressionSet   │ Expression      │ Idioms, phrases, collocations    │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ PatternSet      │ Pattern         │ Sentence structures, templates   │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ CultureSet      │ CultureRef      │ Cultural references, symbols     │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ TabooSet        │ Taboo           │ Words/topics to avoid            │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ AudienceSet     │ AudienceTrait   │ Reader characteristics           │  ║"
            .to_string(),
    );
    lines.push(
        "║  └─────────────────┴─────────────────┴──────────────────────────────────┘  ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  KEY PRINCIPLE: Containers are EMPTY — all data lives in atoms            ║".to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    lines.join("\n")
}

/// Render the Generation Pipeline view in Atlas mode.
/// Shows the 6-stage native content generation flow (NOT translation).
fn render_atlas_generation_pipeline(app: &App) -> String {
    let mut lines = Vec::new();
    let stage = app.atlas.pipeline_stage;

    // Navigation hint
    lines.push(format!(
        "  h/l: prev/next stage  |  Stage {}/5  |  Generation, NOT Translation",
        stage + 1
    ));
    lines.push(String::new());

    // Pipeline stages
    let stages = [
        (
            "ENTITY (invariant)",
            "The core concept that exists independently of locale",
        ),
        (
            "TASK (job)",
            "What kind of content to generate (Hero, FAQ, CTA...)",
        ),
        (
            "CONTEXT ASSEMBLY",
            "Spreading activation + selective knowledge atom loading",
        ),
        (
            "PROMPT ENGINEERING",
            "Rules, style guides, locale-specific patterns",
        ),
        (
            "GENERATION",
            "LLM call with assembled context → native content",
        ),
        ("OUTPUT (L10n)", "Localized content stored as *L10n node"),
    ];

    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(
        "║  NATIVE CONTENT GENERATION PIPELINE                                       ║".to_string(),
    );
    lines.push(
        "║  ════════════════════════════════════════════════════════════════════════ ║".to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Show flow diagram
    for (i, (name, desc)) in stages.iter().enumerate() {
        let is_current = i == stage;
        let prefix = if is_current { "▶" } else { " " };
        let highlight = if is_current { "★" } else { " " };

        // Stage box
        if is_current {
            lines.push(format!(
                "║  {} ┌────────────────────────────────────────────────────────────────┐ {} ║",
                prefix, highlight
            ));
            lines.push(format!(
                "║    │  {}. {}                                                    │   ║",
                i + 1,
                name
            ));
            lines.push(
                "║    │                                                                  │   ║"
                    .to_string(),
            );
            lines.push(format!("║    │  {}  │   ║", truncate_str(desc, 60)));
            lines.push(
                "║    └────────────────────────────────────────────────────────────────┘   ║"
                    .to_string(),
            );
        } else {
            lines.push(format!(
                "║  {} [ {}. {} ]                                                       ║",
                prefix,
                i + 1,
                truncate_str(name, 50)
            ));
        }

        // Arrow between stages
        if i < stages.len() - 1 {
            lines.push(
                "║                              │                                          ║"
                    .to_string(),
            );
            lines.push(
                "║                              ▼                                          ║"
                    .to_string(),
            );
        }
    }

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // Stage-specific details
    lines.push(format!(
        "║  STAGE {} DETAILS                                                           ║",
        stage + 1
    ));
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    match stage {
        0 => {
            // Entity stage
            lines.push(
                "║  ENTITY: The invariant concept                                           ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  • Entity.key = \"qrcode-ai\" (universal identifier)                         ║"
                    .to_string(),
            );
            lines.push(
                "║  • Entity.display_name = \"QR Code AI\"                                      ║"
                    .to_string(),
            );
            lines.push(
                "║  • NO locale-specific content here                                         ║"
                    .to_string(),
            );
            lines.push(
                "║  • Links to EntityL10n for each locale                                     ║"
                    .to_string(),
            );
        }
        1 => {
            // Task stage
            lines.push(
                "║  TASK: What type of content to generate                                   ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push("║  • GenerationTask.task_type = \"hero\" | \"faq\" | \"cta\" | ...                 ║".to_string());
            lines.push(
                "║  • Determines which knowledge atoms get boosted                           ║"
                    .to_string(),
            );
            lines.push(
                "║  • FAQ → boost definitions, Hero → boost benefits                         ║"
                    .to_string(),
            );
        }
        2 => {
            // Context Assembly stage
            lines.push(
                "║  CONTEXT ASSEMBLY: Build relevant context for LLM                         ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  1. Spreading Activation from Entity (decay over hops)                    ║"
                    .to_string(),
            );
            lines.push(
                "║  2. Task-specific boosts (urgency×1.3 for CTA, etc.)                      ║"
                    .to_string(),
            );
            lines.push(
                "║  3. Selective Knowledge Atom loading (50 Terms, not 20K blob)             ║"
                    .to_string(),
            );
            lines.push(
                "║  4. Temperature cutoff: only include atoms > threshold                    ║"
                    .to_string(),
            );
        }
        3 => {
            // Prompt Engineering stage
            lines.push(
                "║  PROMPT ENGINEERING: Structure the LLM request                            ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  • System prompt: role, constraints, output format                        ║"
                    .to_string(),
            );
            lines.push(
                "║  • Context: activated knowledge atoms + entity info                       ║"
                    .to_string(),
            );
            lines.push(
                "║  • Locale rules: fr-FR formal vous, ja-JP honorifics                      ║"
                    .to_string(),
            );
            lines.push(
                "║  • Style guide: brand voice, tone, terminology                            ║"
                    .to_string(),
            );
        }
        4 => {
            // Generation stage
            lines.push(
                "║  GENERATION: Native content creation (NOT translation!)                   ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  ┌─ WRONG ───────────────────────────────────────────────────────────┐    ║"
                    .to_string(),
            );
            lines.push(
                "║  │  Source (en-US) → Translate → Target (fr-FR)                      │    ║"
                    .to_string(),
            );
            lines.push(
                "║  └───────────────────────────────────────────────────────────────────┘    ║"
                    .to_string(),
            );
            lines.push(
                "║  ┌─ RIGHT ───────────────────────────────────────────────────────────┐    ║"
                    .to_string(),
            );
            lines.push(
                "║  │  Entity + Context → Generate natively → L10n (locale-specific)    │    ║"
                    .to_string(),
            );
            lines.push(
                "║  └───────────────────────────────────────────────────────────────────┘    ║"
                    .to_string(),
            );
        }
        5 => {
            // Output stage
            lines.push(
                "║  OUTPUT: Store localized content                                          ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  • EntityL10n.name = \"IA pour QR Code\" (fr-FR)                             ║"
                    .to_string(),
            );
            lines.push(
                "║  • EntityL10n.description = \"Créez des QR codes...\" (native French)       ║"
                    .to_string(),
            );
            lines.push(
                "║  • BlockL10n.content = native locale-specific content                     ║"
                    .to_string(),
            );
            lines.push(
                "║  • Links: Entity -[:HAS_L10N]-> EntityL10n -[:FOR_LOCALE]-> Locale        ║"
                    .to_string(),
            );
        }
        _ => {}
    }

    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    lines.join("\n")
}

/// Render the Page Composition view in Atlas mode.
fn render_atlas_page_composition(app: &App) -> String {
    let mut lines = Vec::new();

    // Mode and navigation indicator
    let mode_label = if app.atlas.demo_mode {
        "[d] DEMO"
    } else {
        "[d] LIVE"
    };
    lines.push(format!(
        "  {}  |  h/l: prev/next page  l: locale ({})  j/k: scroll",
        mode_label, app.atlas.selected_locale
    ));
    lines.push(String::new());

    // Check if we have data
    if app.atlas.pages_list.is_empty() {
        lines.push("Loading pages list from Neo4j...".to_string());
        return lines.join("\n");
    }

    // Page selector
    if app.atlas.page_index < app.atlas.pages_list.len() {
        let page_info = &app.atlas.pages_list[app.atlas.page_index];
        lines.push(format!(
            "  Page {}/{}: {} ({})",
            app.atlas.page_index + 1,
            app.atlas.pages_list.len(),
            page_info.display_name,
            page_info.project_name
        ));
        lines.push(String::new());
    }

    // Page composition data
    if let Some(ref data) = app.atlas.page_data {
        render_page_composition_data(&mut lines, data, &app.atlas.selected_locale);
    } else {
        lines.push("Loading page composition...".to_string());
    }

    lines.join("\n")
}

/// Render the page composition data.
fn render_page_composition_data(
    lines: &mut Vec<String>,
    data: &super::atlas::PageCompositionData,
    locale: &str,
) {
    // Header
    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(format!(
        "║  PAGE: {:<66}║",
        truncate_str(&data.page_display_name, 66)
    ));
    if let Some(ref page_type) = data.page_type {
        lines.push(format!("║  Type: {:<66}║", truncate_str(page_type, 66)));
    }
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // L10n info
    if let Some(ref l10n) = data.page_l10n {
        lines.push(format!(
            "║  L10N [{}]:                                                               ║",
            locale
        ));
        if let Some(ref title) = l10n.title {
            lines.push(format!("║    Title: {:<63}║", truncate_str(title, 63)));
        }
        if let Some(ref slug) = l10n.slug {
            lines.push(format!("║    Slug:  /{:<62}║", truncate_str(slug, 62)));
        }
        lines.push(
            "╠════════════════════════════════════════════════════════════════════════════╣"
                .to_string(),
        );
    }

    // Blocks
    lines.push(format!(
        "║  BLOCKS ({})                                                               ║",
        data.blocks.len()
    ));
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );

    for (i, block) in data.blocks.iter().enumerate() {
        let block_type = block.block_type.as_deref().unwrap_or("generic");
        let marker = if i == data.blocks.len() - 1 {
            "└──"
        } else {
            "├──"
        };
        lines.push(format!(
            "║  {} #{} {} [{}]                                               ║",
            marker,
            block.order,
            truncate_str(&block.display_name, 35),
            truncate_str(block_type, 12)
        ));

        // Block L10n preview
        if let Some(ref l10n) = block.l10n {
            let preview = truncate_str(&l10n.content_preview, 55);
            lines.push(format!(
                "║       └─ \"{}...\"                                 ║",
                preview
            ));
        }
    }

    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // Entities
    lines.push(format!(
        "║  ENTITIES ({})                                                             ║",
        data.entities.len()
    ));
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );

    for entity in &data.entities {
        let blocks_str = entity.connected_blocks.join(", ");
        lines.push(format!(
            "║  • {} → [{}]                                      ║",
            truncate_str(&entity.display_name, 25),
            truncate_str(&blocks_str, 30)
        ));

        if let Some(ref l10n) = entity.l10n {
            if let Some(ref name) = l10n.name {
                lines.push(format!(
                    "║      L10N: {}                                            ║",
                    truncate_str(name, 45)
                ));
            }
        }
    }

    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // SEO Keywords
    lines.push(format!(
        "║  SEO KEYWORDS ({})                                                        ║",
        data.seo_keywords.len()
    ));
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );

    for kw in data.seo_keywords.iter().take(10) {
        let vol = kw
            .volume
            .map(|v| format!("{}/mo", v))
            .unwrap_or_else(|| "?".to_string());
        lines.push(format!(
            "║  • \"{}\" ({}) → [{}]                               ║",
            truncate_str(&kw.keyword, 25),
            vol,
            truncate_str(&kw.connected_entities.join(", "), 15)
        ));
    }

    if data.seo_keywords.len() > 10 {
        lines.push(format!(
            "║    ... and {} more keywords                                             ║",
            data.seo_keywords.len() - 10
        ));
    }

    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    // Summary
    lines.push(String::new());
    lines.push(format!(
        "  Summary: {} blocks, {} entities, {} SEO keywords │ Locale: {}",
        data.blocks.len(),
        data.entities.len(),
        data.seo_keywords.len(),
        locale
    ));
}

/// Render the Spreading Activation view in Atlas mode.
/// Shows cognitive science math behind context assembly.
fn render_atlas_spreading_activation(app: &App) -> String {
    use super::atlas::ActivationTask;

    let mut lines = Vec::new();
    let step = app.atlas.activation_step;
    let task = &app.atlas.activation_task;

    // Header with navigation hints
    lines.push(format!(
        "  h/l: step activation  t: cycle task [{}]  Enter: reset  |  Step {}",
        task.label(),
        step
    ));
    lines.push(String::new());

    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(
        "║  SPREADING ACTIVATION — Context Assembly for LLM Prompts                   ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // Formula section
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  ACTIVATION FORMULA:                                                       ║"
            .to_string(),
    );
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    A(t) = A₀ × e^(-λt) × task_boost                                        ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Where:                                                                  ║"
            .to_string(),
    );
    lines.push(
        "║      A₀   = Initial activation (1.0 for root entity)                       ║"
            .to_string(),
    );
    lines.push(
        "║      λ    = Decay rate (0.3 per hop)                                       ║"
            .to_string(),
    );
    lines.push(
        "║      t    = Distance from root (hop count)                                 ║"
            .to_string(),
    );
    lines.push(
        "║      boost = Task-specific multiplier                                      ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Task boosts
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  TASK-SPECIFIC BOOSTS:                                                     ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    let tasks = [
        (ActivationTask::CTA, "urgency", 1.3),
        (ActivationTask::FAQ, "definition", 1.3),
        (ActivationTask::Hero, "benefit", 1.2),
        (ActivationTask::Pricing, "value", 1.2),
        (ActivationTask::Features, "capability", 1.2),
    ];

    for (t, concept, boost) in &tasks {
        let marker = if t == task { "►" } else { " " };
        let highlight = if t == task { " ◄─ ACTIVE" } else { "" };
        lines.push(format!(
            "║  {} {:12} boosts {:12} by ×{:.1}{}                      ║",
            marker,
            t.label(),
            concept,
            boost,
            highlight
        ));
    }

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Visualization of activation spreading
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  ACTIVATION PROPAGATION:                                                   ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Simulate activation values based on step
    let decay = 0.3_f32;
    let calc_activation = |hop: usize| -> f32 {
        if hop > step {
            0.0
        } else {
            1.0_f32 * (-decay * hop as f32).exp()
        }
    };

    // Visual network representation
    let a0 = calc_activation(0);
    let a1 = calc_activation(1);
    let a2 = calc_activation(2);
    let a3 = calc_activation(3);

    let bar = |a: f32| -> String {
        let filled = (a * 10.0) as usize;
        format!(
            "[{}{}] {:.2}",
            "█".repeat(filled),
            "░".repeat(10 - filled),
            a
        )
    };

    lines.push(
        "║                           ┌─────────────────┐                             ║".to_string(),
    );
    lines.push(
        "║                           │  ROOT ENTITY    │                             ║".to_string(),
    );
    lines.push(format!(
        "║                           │   A₀ = {}   │                             ║",
        bar(a0)
    ));
    lines.push(
        "║                           └────────┬────────┘                             ║".to_string(),
    );
    lines.push(
        "║                        ┌───────────┼───────────┐                          ║".to_string(),
    );
    lines.push(
        "║                        ▼           ▼           ▼                          ║".to_string(),
    );
    lines.push(
        "║              ┌───────────────┐ ┌───────────┐ ┌───────────────┐            ║".to_string(),
    );
    lines.push(
        "║              │   Concept A   │ │ Concept B │ │   Concept C   │            ║".to_string(),
    );
    lines.push(format!(
        "║              │ A₁ = {}│ │A₁ = {} │ │ A₁ = {}│            ║",
        bar(a1),
        bar(a1),
        bar(a1)
    ));
    lines.push(
        "║              └───────┬───────┘ └─────┬─────┘ └───────┬───────┘            ║".to_string(),
    );
    lines.push(
        "║                      ▼               ▼               ▼                    ║".to_string(),
    );
    lines.push(
        "║          ┌─────────────────┐ ┌─────────────┐ ┌─────────────────┐          ║".to_string(),
    );
    lines.push(
        "║          │    Sub-concept  │ │ Sub-concept │ │    Sub-concept  │          ║".to_string(),
    );
    lines.push(format!(
        "║          │  A₂ = {}  │ │A₂ = {} │ │  A₂ = {}  │          ║",
        bar(a2),
        bar(a2),
        bar(a2)
    ));
    lines.push(
        "║          └────────┬────────┘ └──────┬──────┘ └────────┬────────┘          ║".to_string(),
    );
    lines.push(
        "║                   ▼                 ▼                 ▼                   ║".to_string(),
    );
    lines.push(
        "║          ┌─────────────────────────────────────────────────────┐          ║".to_string(),
    );
    lines.push(
        "║          │                   Distant nodes                     │          ║".to_string(),
    );
    lines.push(format!(
        "║          │                A₃ = {}                    │          ║",
        bar(a3)
    ));
    lines.push(
        "║          └─────────────────────────────────────────────────────┘          ║".to_string(),
    );

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Threshold and selection
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  CONTEXT ASSEMBLY:                                                         ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Threshold: 0.40                                                         ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Count nodes above threshold
    let above_threshold = [a0, a1, a1, a1, a2, a2, a2, a3]
        .iter()
        .filter(|&&a| a >= 0.40)
        .count();
    let total_nodes = 8;

    lines.push(format!(
        "║    Nodes above threshold: {}/{}                                             ║",
        above_threshold, total_nodes
    ));
    lines.push(
        "║    → These nodes become LLM context                                        ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(format!(
        "║    Step {} result: Activation has propagated {} hops from root              ║",
        step, step
    ));
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    lines.join("\n")
}

/// Render the View Traversal Debugger in Atlas mode.
/// Shows the 12 view definitions with traversal patterns.
fn render_atlas_view_traversal(app: &App) -> String {
    let mut lines = Vec::new();
    let cursor = app.atlas.view_cursor;

    // View definitions (static for now, matches _registry.yaml)
    let views = [
        (
            "complete-graph",
            "overview",
            "Full NovaNet graph (all nodes)",
        ),
        (
            "global-layer",
            "overview",
            "Locale config and knowledge (15 nodes)",
        ),
        (
            "seo-keywords",
            "overview",
            "SEO keywords and metrics (3 nodes)",
        ),
        ("project-layer", "overview", "Per-project nodes (14 nodes)"),
        (
            "page-generation-context",
            "generation",
            "Full context for page orchestrator",
        ),
        (
            "block-generation",
            "generation",
            "Context for block sub-agent",
        ),
        (
            "block-semantic-network",
            "generation",
            "Block with spreading activation",
        ),
        (
            "locale-full-knowledge",
            "knowledge",
            "Complete locale knowledge",
        ),
        ("entity-ecosystem", "knowledge", "Entity with L10n and SEO"),
        (
            "project-context",
            "project",
            "Project with locales and pages",
        ),
        ("project-overview", "project", "Project dashboard"),
        ("seo-pipeline", "mining", "SEO keyword mining workflow"),
    ];

    // Header
    lines.push(format!(
        "  j/k: navigate  |  View {}/{}  |  Temperature: {:.1}  Depth: {}",
        cursor + 1,
        views.len(),
        app.atlas.traversal_temperature,
        app.atlas.traversal_depth
    ));
    lines.push(String::new());

    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(
        "║  VIEW TRAVERSAL DEBUGGER — 12 View Definitions                             ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // View list (left side) + Detail (right side)
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  VIEWS                           │ SELECTED VIEW DETAILS                   ║"
            .to_string(),
    );
    lines.push(
        "║  ────────────────────────────────┼─────────────────────────────────────────║"
            .to_string(),
    );

    let selected_idx = cursor.min(views.len() - 1);
    let (sel_id, sel_cat, sel_desc) = views[selected_idx];

    for (i, (id, cat, _desc)) in views.iter().enumerate() {
        let marker = if i == selected_idx { "►" } else { " " };
        let cat_icon = match *cat {
            "overview" => "▣",
            "generation" => "◇",
            "knowledge" => "▤",
            "project" => "▢",
            "mining" => "◆",
            _ => "•",
        };

        // Build the left side (view list)
        let left = format!("  {} {} {:<20}", marker, cat_icon, id);

        // Build the right side (details for selected view only at certain rows)
        let right = match i {
            0 => format!("ID: {}", sel_id),
            1 => format!("Category: {}", sel_cat),
            2 => "Description:".to_string(),
            3 => format!("  {}", truncate_str(sel_desc, 35)),
            5 => "Traversal Pattern:".to_string(),
            6 => "  root → relations → depth".to_string(),
            8 => "Filters:".to_string(),
            9 => "  locale: $locale".to_string(),
            10 => "  temperature: >= 0.3".to_string(),
            _ => String::new(),
        };

        lines.push(format!("║{:<34}│ {:<40}║", left, right));
    }

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Traversal pattern visualization
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  TRAVERSAL ALGORITHM:                                                      ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    1. Start at ROOT node (e.g., Block, Page, Entity)                       ║"
            .to_string(),
    );
    lines.push(
        "║    2. Follow INCLUDE relations:                                            ║"
            .to_string(),
    );
    lines.push(
        "║       • direction: outgoing/incoming                                       ║"
            .to_string(),
    );
    lines.push(
        "║       • depth: max hops (default 1)                                        ║"
            .to_string(),
    );
    lines.push(
        "║       • nested includes for deeper traversal                               ║"
            .to_string(),
    );
    lines.push(
        "║    3. Apply FILTERS:                                                       ║"
            .to_string(),
    );
    lines.push(
        "║       • locale: $locale parameter                                          ║"
            .to_string(),
    );
    lines.push(
        "║       • temperature: >= threshold for spreading activation                 ║"
            .to_string(),
    );
    lines.push(
        "║    4. Return assembled context as LLM prompt input                         ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Example view structure
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(format!(
        "║  EXAMPLE: {} view structure:{}║",
        sel_id,
        " ".repeat(56 - sel_id.len())
    ));
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Show example traversal for selected view
    match sel_id {
        "block-generation" => {
            lines.push(
                "║    Block ─────┬─[:HAS_PROMPT]──────> BlockPrompt                         ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:OF_TYPE]──────────> BlockType ─[:HAS_RULES]> Rules     ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:HAS_INSTRUCTION]──> BlockInstruction                   ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:USES_ENTITY]──────> Entity ─[:HAS_L10N]──> EntityL10n  ║"
                    .to_string(),
            );
            lines.push(
                "║               │                              └─[:SEMANTIC_LINK]> Entity  ║"
                    .to_string(),
            );
            lines.push(
                "║               └─[:HAS_OUTPUT]───────> BlockL10n                          ║"
                    .to_string(),
            );
        }
        "page-generation-context" => {
            lines.push(
                "║    Page ──────┬─[:HAS_PROMPT]──────> PagePrompt                          ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:HAS_BLOCK]────────> Block (ordered)                    ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:HAS_PAGE_L10N]────> PageL10n                           ║"
                    .to_string(),
            );
            lines.push(
                "║               └─[:BELONGS_TO]───────> Project                            ║"
                    .to_string(),
            );
        }
        "entity-ecosystem" => {
            lines.push(
                "║    Entity ────┬─[:HAS_L10N]─────────> EntityL10n                         ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:SEMANTIC_LINK]───> Entity (related)                   ║"
                    .to_string(),
            );
            lines.push(
                "║               └─[:EXPRESSES]<──────── SEOKeyword                         ║"
                    .to_string(),
            );
        }
        _ => {
            lines.push(format!(
                "║    Root ──────┬─ [relations defined in {}.yaml]{}║",
                sel_id,
                " ".repeat(31 - sel_id.len().min(30))
            ));
            lines.push(
                "║               └─ See: packages/core/models/views/                        ║"
                    .to_string(),
            );
        }
    }

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    lines.join("\n")
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
        assert_eq!(truncate_str("hello world", 5), "hello...");
    }

    #[test]
    fn test_truncate_str_utf8_bengali() {
        // Bengali: "বাংলা (বাংলাদেশ)" - this caused the original panic
        let bengali = "বাংলা (বাংলাদেশ)";
        // Should not panic even when truncating in the middle of multi-byte chars
        let result = truncate_str(bengali, 5);
        assert_eq!(result.chars().count(), 8); // 5 chars + "..."
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_str_utf8_emoji() {
        let emoji = "Hello 👋🏻 World 🌍";
        let result = truncate_str(emoji, 8);
        // "Hello 👋🏻" = 8 chars (emoji with skin tone is 2 chars)
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_str_chinese() {
        let chinese = "你好世界这是中文测试";
        let result = truncate_str(chinese, 4);
        assert_eq!(result, "你好世界...");
    }

    #[test]
    fn test_truncate_str_empty() {
        assert_eq!(truncate_str("", 10), "");
    }
}
