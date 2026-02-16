//! Views Mode — Schema views explorer (v0.12.5).
//!
//! Loads views from `views.yaml` (single source of truth shared with Studio).
//! Shows available views organized by category.
//! Enhanced with full Cypher display, Studio links, relation colors, and ASCII schemas.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use regex::Regex;

use crate::parsers::views::{SimpleViewEntry, SimpleViewsFile, ViewCategoryDef, ViewIcon};
use crate::tui::app::App;
use crate::tui::ui::COLOR_UNFOCUSED_BORDER;

// =============================================================================
// LOADED VIEWS DATA
// =============================================================================

/// Loaded views data from views.yaml.
#[derive(Debug, Clone, Default)]
pub struct LoadedViews {
    pub categories: Vec<(String, ViewCategoryDef)>,
    pub views: Vec<SimpleViewEntry>,
}

impl LoadedViews {
    /// Load views from views.yaml.
    pub fn load(root_path: &str) -> Self {
        let root = std::path::Path::new(root_path);
        match crate::parsers::views::load_simple_views(root) {
            Ok(file) => Self::from_file(file),
            Err(e) => {
                eprintln!("Warning: Failed to load views.yaml: {e}");
                Self::default()
            }
        }
    }

    /// Convert from parsed file.
    fn from_file(file: SimpleViewsFile) -> Self {
        // Sort categories in display order
        let category_order = ["schema", "data", "generation", "contextual"];
        let mut categories: Vec<(String, ViewCategoryDef)> = file.categories.into_iter().collect();
        categories.sort_by_key(|(k, _)| category_order.iter().position(|&c| c == k).unwrap_or(99));

        Self {
            categories,
            views: file.views,
        }
    }

    /// Get views in a category.
    pub fn views_in_category(&self, category: &str) -> Vec<&SimpleViewEntry> {
        self.views
            .iter()
            .filter(|v| v.category == category)
            .collect()
    }

    /// Get category count.
    pub fn category_count(&self) -> usize {
        self.categories.len()
    }

    /// Get category at index.
    pub fn category_at(&self, idx: usize) -> Option<&(String, ViewCategoryDef)> {
        self.categories.get(idx)
    }
}

// =============================================================================
// STATE
// =============================================================================

/// Detail panel sections (Tab to cycle).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewDetailSection {
    #[default]
    Info,
    Cypher,
    Relations,
    Schema,
}

impl ViewDetailSection {
    /// Cycle to next section.
    pub fn next(self) -> Self {
        match self {
            Self::Info => Self::Cypher,
            Self::Cypher => Self::Relations,
            Self::Relations => Self::Schema,
            Self::Schema => Self::Info,
        }
    }

    /// Cycle to previous section.
    pub fn prev(self) -> Self {
        match self {
            Self::Info => Self::Schema,
            Self::Cypher => Self::Info,
            Self::Relations => Self::Cypher,
            Self::Schema => Self::Relations,
        }
    }

    /// Display name for section.
    pub fn label(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Cypher => "Cypher",
            Self::Relations => "Relations",
            Self::Schema => "Schema",
        }
    }
}

/// State for the Views mode.
#[derive(Debug, Clone, Default)]
pub struct ViewsState {
    /// Currently selected category index.
    pub category_cursor: usize,
    /// Currently selected view within category.
    pub view_cursor: usize,
    /// Whether showing Query-First concept panel.
    pub show_concept: bool,
    /// Current detail section (Tab to cycle).
    pub detail_section: ViewDetailSection,
    /// Scroll offset for Cypher section.
    pub cypher_scroll: u16,
}

impl ViewsState {
    /// Create new ViewsState.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset cursors.
    pub fn reset(&mut self) {
        self.category_cursor = 0;
        self.view_cursor = 0;
        self.show_concept = false;
        self.detail_section = ViewDetailSection::default();
        self.cypher_scroll = 0;
    }

    /// Cycle to next detail section.
    pub fn next_section(&mut self) {
        self.detail_section = self.detail_section.next();
        self.cypher_scroll = 0; // Reset scroll when changing section
    }

    /// Cycle to previous detail section.
    pub fn prev_section(&mut self) {
        self.detail_section = self.detail_section.prev();
        self.cypher_scroll = 0;
    }

    /// Scroll Cypher section down.
    pub fn scroll_down(&mut self, amount: u16) {
        if self.detail_section == ViewDetailSection::Cypher {
            self.cypher_scroll = self.cypher_scroll.saturating_add(amount);
        }
    }

    /// Scroll Cypher section up.
    pub fn scroll_up(&mut self, amount: u16) {
        if self.detail_section == ViewDetailSection::Cypher {
            self.cypher_scroll = self.cypher_scroll.saturating_sub(amount);
        }
    }

    /// Navigate up in views list.
    pub fn navigate_up(&mut self, loaded: &LoadedViews) {
        if self.view_cursor > 0 {
            self.view_cursor -= 1;
        } else if self.category_cursor > 0 {
            // Move to previous category
            self.category_cursor -= 1;
            if let Some((cat_key, _)) = loaded.category_at(self.category_cursor) {
                let views = loaded.views_in_category(cat_key);
                self.view_cursor = views.len().saturating_sub(1);
            }
        }
    }

    /// Navigate down in views list.
    pub fn navigate_down(&mut self, loaded: &LoadedViews) {
        if let Some((cat_key, _)) = loaded.category_at(self.category_cursor) {
            let views = loaded.views_in_category(cat_key);
            if self.view_cursor + 1 < views.len() {
                self.view_cursor += 1;
            } else if self.category_cursor + 1 < loaded.category_count() {
                // Move to next category
                self.category_cursor += 1;
                self.view_cursor = 0;
            }
        }
    }

    /// Jump to next category.
    pub fn next_category(&mut self, loaded: &LoadedViews) {
        if self.category_cursor + 1 < loaded.category_count() {
            self.category_cursor += 1;
            self.view_cursor = 0;
        }
    }

    /// Jump to previous category.
    pub fn prev_category(&mut self) {
        if self.category_cursor > 0 {
            self.category_cursor -= 1;
            self.view_cursor = 0;
        }
    }

    /// Toggle Query-First concept panel.
    pub fn toggle_concept(&mut self) {
        self.show_concept = !self.show_concept;
    }

    /// Get currently selected view.
    pub fn current_view<'a>(&self, loaded: &'a LoadedViews) -> Option<&'a SimpleViewEntry> {
        let (cat_key, _) = loaded.category_at(self.category_cursor)?;
        let views = loaded.views_in_category(cat_key);
        views.get(self.view_cursor).copied()
    }

    /// Get text for yanking.
    pub fn get_yank_text(&self, loaded: &LoadedViews) -> Option<String> {
        self.current_view(loaded).map(|v| v.id.clone())
    }
}

// =============================================================================
// COLORS & ICONS
// =============================================================================

/// Parse hex color to ratatui Color.
fn parse_color(hex: &str) -> Color {
    if hex.len() == 7 && hex.starts_with('#') {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(128);
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(128);
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(128);
        Color::Rgb(r, g, b)
    } else {
        Color::Gray
    }
}

/// Get terminal icon from ViewIcon or default.
fn terminal_icon<'a>(icon: &'a Option<ViewIcon>, default: &'a str) -> &'a str {
    icon.as_ref()
        .map(|i| i.terminal.as_str())
        .unwrap_or(default)
}

// =============================================================================
// RELATION EXTRACTION & SCHEMA (future: v0.14.0 schema visualization)
// =============================================================================

/// Extracted relation info from Cypher query.
#[derive(Debug, Clone)]
struct ExtractedRelation {
    name: String,
    family: &'static str,
}

/// Extract relation names from a Cypher query.
/// Matches patterns like `[:HAS_BLOCK]`, `[r:USES_ENTITY]`, `[r1:OF_TYPE]`
fn extract_relations_from_cypher(cypher: &str) -> Vec<ExtractedRelation> {
    let re = Regex::new(r"\[r?\d*:([A-Z_]+)\]").unwrap();
    let mut relations: Vec<ExtractedRelation> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for cap in re.captures_iter(cypher) {
        if let Some(m) = cap.get(1) {
            let name = m.as_str().to_string();
            if !seen.contains(&name) {
                seen.insert(name.clone());
                let family = infer_arc_family(&name);
                relations.push(ExtractedRelation { name, family });
            }
        }
    }
    relations
}

/// Infer arc family from arc name using common patterns.
fn infer_arc_family(arc_name: &str) -> &'static str {
    // Check specific patterns BEFORE generic HAS_* ownership pattern

    // Localization family: FOR_LOCALE, HAS_VOICE, HAS_CULTURE, HAS_EXPRESSIONS, etc.
    // (checked first because these start with HAS_)
    if arc_name == "FOR_LOCALE"
        || arc_name == "HAS_VOICE"
        || arc_name == "HAS_CULTURE"
        || arc_name == "HAS_EXPRESSIONS"
        || arc_name == "HAS_TERMS"
        || arc_name == "HAS_PATTERNS"
        || arc_name == "HAS_FORMATTING"
        || arc_name == "HAS_MARKET"
        || arc_name == "HAS_TABOOS"
        || arc_name == "HAS_AUDIENCE"
    {
        return "localization";
    }
    // Generation family: GENERATED*, COMPILED*, ASSEMBLES, etc.
    if arc_name.starts_with("GENERATED")
        || arc_name.starts_with("COMPILED")
        || arc_name == "ASSEMBLES"
        || arc_name.starts_with("INCLUDES_")
        || arc_name == "INFLUENCED_BY"
    {
        return "generation";
    }
    // Mining family: TARGETS_*, MONITORS_*
    if arc_name.starts_with("TARGETS_") || arc_name.starts_with("MONITORS_") {
        return "mining";
    }
    // Ownership family: HAS_*, *_OF, CONTAINS_*, BELONGS_TO*
    // (checked AFTER localization because some localization arcs start with HAS_)
    if arc_name.starts_with("HAS_")
        || arc_name.ends_with("_OF")
        || arc_name.starts_with("CONTAINS_")
        || arc_name.starts_with("BELONGS_TO")
    {
        return "ownership";
    }
    // Default: semantic
    "semantic"
}

/// Get Studio link for a view.
fn studio_link(view_id: &str) -> String {
    format!("http://localhost:3000/?view={view_id}")
}

/// Generate ASCII schema diagram for a view's traversal pattern.
fn generate_ascii_schema(view: &SimpleViewEntry) -> Vec<String> {
    let mut lines = Vec::new();
    let cypher = view.cypher.as_deref().unwrap_or("");
    let relations = extract_relations_from_cypher(cypher);

    if relations.is_empty() {
        lines.push("  (no traversal pattern detected)".to_string());
        return lines;
    }

    // Extract root type
    let root = view.root_type.as_deref().unwrap_or("?");

    // Build a simple pattern representation
    lines.push("  ┌────────────────────────────────────────┐".to_string());
    lines.push(format!(
        "  │  {} ({}){}│",
        view.name,
        view.id,
        " ".repeat(38_usize.saturating_sub(view.name.len() + view.id.len() + 4))
    ));
    lines.push("  ├────────────────────────────────────────┤".to_string());

    // Show root node
    let root_box = "  │  ┌──────────┐                          │".to_string();
    let root_lbl = format!(
        "  │  │ {}{} │  (root)                   │",
        root,
        " ".repeat(8_usize.saturating_sub(root.len()))
    );
    let root_btm = "  │  └────┬─────┘                          │".to_string();
    lines.push(root_box);
    lines.push(root_lbl);
    lines.push(root_btm);

    // Show relations
    for (i, rel) in relations.iter().take(5).enumerate() {
        let arrow = if i < relations.len() - 1 { "│" } else { " " };
        lines.push(format!("  │       {arrow}                               │"));
        lines.push(format!(
            "  │       ├──[{}]──▶              │",
            &rel.name[..rel.name.len().min(20)]
        ));
    }

    if relations.len() > 5 {
        lines.push(format!(
            "  │       └── ... ({} more)                 │",
            relations.len() - 5
        ));
    }

    lines.push("  │                                        │".to_string());
    lines.push("  └────────────────────────────────────────┘".to_string());

    lines
}

// =============================================================================
// RENDERING
// =============================================================================

/// Query-First architecture explanation.
const QUERY_FIRST_CONCEPT: &[&str] = &[
    "QUERY-FIRST ARCHITECTURE",
    "",
    "Views define Cypher queries for graph traversal.",
    "Instead of loading the entire graph, views specify:",
    "",
    "  1. ROOT: Starting node type (e.g., Block, Page)",
    "  2. CYPHER: Query with $nodeKey parameter",
    "  3. CATEGORY: schema, data, generation, contextual",
    "",
    "This enables:",
    "  - Selective context loading for LLM prompts",
    "  - Efficient Neo4j queries",
    "  - Shared views between TUI and Studio",
    "",
    "Views are defined in YAML:",
    "  packages/core/models/views.yaml",
    "",
    "Single source of truth for TUI + Studio (v0.12.5)",
];

/// Render the Views mode.
pub fn render_views_tab(f: &mut Frame, app: &App, area: Rect) {
    let views_state = &app.nexus.views;
    let loaded = &app.loaded_views;

    if views_state.show_concept {
        // Show Query-First concept panel
        render_concept_panel(f, area);
    } else {
        // Normal view: 25% tree | 75% detail (matches Graph mode)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        render_views_list(f, views_state, loaded, chunks[0]);
        render_view_detail_multibox(f, views_state, loaded, chunks[1]);
    }
}

/// Render the views list organized by category.
fn render_views_list(f: &mut Frame, state: &ViewsState, loaded: &LoadedViews, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " VIEWS ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    // Header
    lines.push(Line::from(vec![
        Span::styled(
            "Source: views.yaml ",
            Style::default().fg(Color::Rgb(180, 180, 180)),
        ),
        Span::styled("[?:concept]", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(Span::styled(
        "\u{2500}".repeat(inner.width.saturating_sub(2) as usize),
        Style::default().fg(COLOR_UNFOCUSED_BORDER),
    )));
    lines.push(Line::from(""));

    // Categories and views
    for (cat_idx, (cat_key, cat_def)) in loaded.categories.iter().enumerate() {
        let is_selected_cat = cat_idx == state.category_cursor;
        let cat_color = cat_def
            .color
            .as_deref()
            .map(parse_color)
            .unwrap_or(Color::Cyan);

        // Category header
        let cat_style = if is_selected_cat {
            Style::default().fg(cat_color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(cat_color)
        };

        let cat_views = loaded.views_in_category(cat_key);
        let cat_icon = terminal_icon(&cat_def.icon, "●");

        lines.push(Line::from(vec![
            Span::styled(format!("{cat_icon} "), cat_style),
            Span::styled(&cat_def.label, cat_style),
            Span::styled(
                format!(" ({})", cat_views.len()),
                Style::default().fg(Color::DarkGray),
            ),
        ]));

        // Views in category (only show if selected)
        if is_selected_cat {
            for (view_idx, view) in cat_views.iter().enumerate() {
                let is_selected = view_idx == state.view_cursor;

                let prefix = if is_selected { "  \u{25b8} " } else { "    " }; // ▸

                let style = if is_selected {
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Rgb(30, 50, 70))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Rgb(180, 180, 180))
                };

                // Add contextual badge if needed
                let mut spans = vec![Span::styled(prefix, style), Span::styled(&view.name, style)];
                if view.contextual.unwrap_or(false) {
                    spans.push(Span::styled(
                        " ◎",
                        Style::default().fg(Color::Rgb(34, 197, 94)), // Green
                    ));
                }
                lines.push(Line::from(spans));
            }
        }

        lines.push(Line::from(""));
    }

    // Legend for contextual indicator
    lines.push(Line::from(vec![
        Span::styled("◎", Style::default().fg(Color::Rgb(34, 197, 94))),
        Span::styled(
            " = contextual (requires node)",
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    lines.push(Line::from(""));

    // Navigation hints
    lines.push(Line::from(Span::styled(
        "[j/k:nav] [Tab:section] [Enter:copy] [o:open] [y:yank] [?:concept]",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Render the Query-First concept panel.
fn render_concept_panel(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " QUERY-FIRST ARCHITECTURE ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    for (i, line) in QUERY_FIRST_CONCEPT.iter().enumerate() {
        let style = if i == 0 {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else if line.starts_with("  ") && line.contains(':') {
            // Numbered items
            Style::default().fg(Color::Yellow)
        } else if line.starts_with("  -") || line.starts_with("  packages") {
            Style::default().fg(Color::Rgb(150, 200, 150))
        } else {
            Style::default().fg(Color::Rgb(180, 180, 180))
        };

        lines.push(Line::from(Span::styled(*line, style)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[? or Esc: back to views]",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

// =============================================================================
// MULTI-BOX DETAIL RENDERING (v0.12.5)
// =============================================================================

/// Render Info box (top-left).
fn render_info_box(f: &mut Frame, view: &SimpleViewEntry, state: &ViewsState, area: Rect) {
    let is_selected = state.detail_section == ViewDetailSection::Info;
    let border_color = if is_selected {
        Color::Cyan
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    let block = Block::default()
        .title(Span::styled(
            " INFO ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let view_color = view
        .color
        .as_deref()
        .map(parse_color)
        .unwrap_or(Color::Cyan);
    let view_icon = terminal_icon(&view.icon, "●");

    let mut lines: Vec<Line> = Vec::new();

    // View name
    lines.push(Line::from(Span::styled(
        &view.name,
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    // Metadata
    lines.push(Line::from(vec![
        Span::styled("ID:       ", Style::default().fg(Color::DarkGray)),
        Span::styled(&view.id, Style::default().fg(Color::Cyan)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Category: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{view_icon} {}", view.category),
            Style::default().fg(view_color),
        ),
    ]));

    if let Some(ref root) = view.root_type {
        lines.push(Line::from(vec![
            Span::styled("Root:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(root, Style::default().fg(Color::Yellow)),
        ]));
    }

    if view.contextual.unwrap_or(false) {
        lines.push(Line::from(vec![
            Span::styled("Context:  ", Style::default().fg(Color::DarkGray)),
            Span::styled("contextual", Style::default().fg(Color::Green)),
        ]));
    }

    lines.push(Line::from(""));

    // Studio link
    let link = studio_link(&view.id);
    lines.push(Line::from(vec![
        Span::styled("Studio:   ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            link,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::UNDERLINED),
        ),
    ]));

    lines.push(Line::from(""));

    // Description
    lines.push(Line::from(Span::styled(
        &view.description,
        Style::default().fg(Color::Rgb(180, 180, 180)),
    )));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Highlight Cypher keywords for syntax highlighting.
fn highlight_cypher_line(line: &str) -> Line<'static> {
    let keywords = [
        "MATCH", "OPTIONAL", "RETURN", "WHERE", "WITH", "ORDER", "BY", "LIMIT", "DISTINCT", "AND",
        "OR", "NOT", "AS",
    ];
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut remaining = line.to_string();

    while !remaining.is_empty() {
        let mut found = false;
        for kw in &keywords {
            if remaining.to_uppercase().starts_with(kw) {
                let next_char = remaining.chars().nth(kw.len());
                if next_char.is_none() || !next_char.unwrap().is_alphanumeric() {
                    spans.push(Span::styled(
                        kw.to_string(),
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ));
                    remaining = remaining[kw.len()..].to_string();
                    found = true;
                    break;
                }
            }
        }
        if !found {
            // Check for relationship pattern [:NAME] or [r:NAME]
            if remaining.starts_with("[:") || remaining.starts_with("[r") {
                if let Some(end) = remaining.find(']') {
                    spans.push(Span::styled(
                        remaining[..=end].to_string(),
                        Style::default().fg(Color::Yellow),
                    ));
                    remaining = remaining[end + 1..].to_string();
                    continue;
                }
            }
            // Check for parameter $name
            if remaining.starts_with('$') {
                let end = remaining[1..]
                    .find(|c: char| !c.is_alphanumeric() && c != '_')
                    .map(|i| i + 1)
                    .unwrap_or(remaining.len());
                spans.push(Span::styled(
                    remaining[..end].to_string(),
                    Style::default().fg(Color::Cyan),
                ));
                remaining = remaining[end..].to_string();
                continue;
            }
            // Regular character - append to previous span if same style
            let c = remaining.remove(0);
            let green_style = Style::default().fg(Color::Rgb(150, 200, 150));
            if let Some(last) = spans.last_mut() {
                if last.style == green_style {
                    let content = format!("{}{}", last.content, c);
                    *last = Span::styled(content, last.style);
                    continue;
                }
            }
            spans.push(Span::styled(c.to_string(), green_style));
        }
    }

    Line::from(spans)
}

/// Render Cypher box (top-right) with syntax highlighting.
fn render_cypher_box(f: &mut Frame, view: &SimpleViewEntry, state: &ViewsState, area: Rect) {
    let is_selected = state.detail_section == ViewDetailSection::Cypher;
    let border_color = if is_selected {
        Color::Green
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    let scroll_hint = if is_selected {
        format!(" CYPHER [↓{}] ", state.cypher_scroll)
    } else {
        " CYPHER ".to_string()
    };

    let block = Block::default()
        .title(Span::styled(
            scroll_hint,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let cypher = view.cypher.as_deref().unwrap_or("(no query)");
    let mut lines: Vec<Line> = Vec::new();

    for line in cypher.lines().skip(state.cypher_scroll as usize) {
        lines.push(highlight_cypher_line(line));
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Render Relations box (bottom-left) with family colors.
fn render_relations_box(f: &mut Frame, view: &SimpleViewEntry, state: &ViewsState, area: Rect) {
    let is_selected = state.detail_section == ViewDetailSection::Relations;
    let border_color = if is_selected {
        Color::Yellow
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    let block = Block::default()
        .title(Span::styled(
            " RELATIONS ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let cypher = view.cypher.as_deref().unwrap_or("");
    let relations = extract_relations_from_cypher(cypher);

    let mut lines: Vec<Line> = Vec::new();

    if relations.is_empty() {
        lines.push(Line::from(Span::styled(
            "  (no relations detected)",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for rel in &relations {
            let family_color = arc_family_color(rel.family);
            let family_icon = match rel.family {
                "ownership" => "●",
                "localization" => "◐",
                "semantic" => "◆",
                "generation" => "⚡",
                "mining" => "◈",
                _ => "○",
            };
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {family_icon} "),
                    Style::default().fg(family_color),
                ),
                Span::styled(&rel.name, Style::default().fg(Color::White)),
                Span::styled(
                    format!("  [{}]", rel.family),
                    Style::default().fg(family_color),
                ),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  {} relations", relations.len()),
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Get color for arc family.
fn arc_family_color(family: &str) -> Color {
    match family {
        "ownership" => Color::Rgb(14, 165, 233),   // sky-500
        "localization" => Color::Rgb(34, 197, 94), // green-500
        "semantic" => Color::Rgb(168, 85, 247),    // purple-500
        "generation" => Color::Rgb(236, 72, 153),  // pink-500
        "mining" => Color::Rgb(245, 158, 11),      // amber-500
        _ => Color::Gray,
    }
}

/// Render Schema box (bottom-right) with ASCII diagram.
fn render_schema_box(f: &mut Frame, view: &SimpleViewEntry, state: &ViewsState, area: Rect) {
    let is_selected = state.detail_section == ViewDetailSection::Schema;
    let border_color = if is_selected {
        Color::Magenta
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    let block = Block::default()
        .title(Span::styled(
            " SCHEMA ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let schema_lines = generate_ascii_schema(view);
    let mut lines: Vec<Line> = Vec::new();

    for line in schema_lines {
        lines.push(Line::from(Span::styled(
            line,
            Style::default().fg(Color::Rgb(180, 180, 180)),
        )));
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Render multi-box detail panel (v0.12.5 enhanced view).
/// Layout: Middle column (stacked boxes) + Right column (schema)
fn render_view_detail_multibox(
    f: &mut Frame,
    state: &ViewsState,
    loaded: &LoadedViews,
    area: Rect,
) {
    let Some(view) = state.current_view(loaded) else {
        let block = Block::default()
            .title(" VIEW DETAIL ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));
        let inner = block.inner(area);
        f.render_widget(block, area);
        let empty = Paragraph::new("Select a view to see details");
        f.render_widget(empty, inner);
        return;
    };

    // Split into middle (60%) + right (40%) columns
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Middle column: stacked boxes (INFO, CYPHER, RELATIONS)
    let middle_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),  // INFO box
            Constraint::Min(8),     // CYPHER box (expandable)
            Constraint::Length(10), // RELATIONS box
        ])
        .split(cols[0]);

    // Right column: SCHEMA (full height)
    render_info_box(f, view, state, middle_rows[0]);
    render_cypher_box(f, view, state, middle_rows[1]);
    render_relations_box(f, view, state, middle_rows[2]);
    render_schema_box(f, view, state, cols[1]);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_views_state_default() {
        let state = ViewsState::new();
        assert_eq!(state.category_cursor, 0);
        assert_eq!(state.view_cursor, 0);
        assert!(!state.show_concept);
    }

    #[test]
    fn test_toggle_concept() {
        let mut state = ViewsState::new();
        assert!(!state.show_concept);

        state.toggle_concept();
        assert!(state.show_concept);

        state.toggle_concept();
        assert!(!state.show_concept);
    }

    #[test]
    fn test_reset() {
        let mut state = ViewsState::new();
        state.category_cursor = 3;
        state.view_cursor = 2;
        state.show_concept = true;

        state.reset();

        assert_eq!(state.category_cursor, 0);
        assert_eq!(state.view_cursor, 0);
        assert!(!state.show_concept);
    }

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("#ffffff"), Color::Rgb(255, 255, 255));
        assert_eq!(parse_color("#000000"), Color::Rgb(0, 0, 0));
        assert_eq!(parse_color("#8b5cf6"), Color::Rgb(139, 92, 246));
        assert_eq!(parse_color("invalid"), Color::Gray);
    }

    #[test]
    fn test_loaded_views_empty() {
        let loaded = LoadedViews::default();
        assert_eq!(loaded.category_count(), 0);
        assert!(loaded.views.is_empty());
    }

    fn test_root() -> Option<std::path::PathBuf> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());
        let root = root?;
        if !root.join("pnpm-workspace.yaml").exists() {
            return None;
        }
        Some(root.to_path_buf())
    }

    #[test]
    fn test_loaded_views_from_yaml() {
        let Some(root) = test_root() else { return };
        let loaded = LoadedViews::load(root.to_str().unwrap());

        assert_eq!(loaded.category_count(), 4, "expected 4 categories");
        assert_eq!(loaded.views.len(), 12, "expected 12 views");

        // Check schema views
        let schema_views = loaded.views_in_category("schema");
        assert_eq!(schema_views.len(), 2);

        // Check generation views
        let gen_views = loaded.views_in_category("generation");
        assert_eq!(gen_views.len(), 3);
    }

    #[test]
    fn test_navigation() {
        let Some(root) = test_root() else { return };
        let loaded = LoadedViews::load(root.to_str().unwrap());
        let mut state = ViewsState::new();

        // Navigate down within category
        state.navigate_down(&loaded);
        assert_eq!(state.view_cursor, 1);

        // Navigate up
        state.navigate_up(&loaded);
        assert_eq!(state.view_cursor, 0);

        // Next category
        state.next_category(&loaded);
        assert_eq!(state.category_cursor, 1);
        assert_eq!(state.view_cursor, 0);

        // Previous category
        state.prev_category();
        assert_eq!(state.category_cursor, 0);
    }

    #[test]
    fn test_current_view() {
        let Some(root) = test_root() else { return };
        let loaded = LoadedViews::load(root.to_str().unwrap());
        let state = ViewsState::new();

        let view = state.current_view(&loaded);
        assert!(view.is_some());
        assert_eq!(view.unwrap().category, "schema");
    }

    #[test]
    fn test_yank_text() {
        let Some(root) = test_root() else { return };
        let loaded = LoadedViews::load(root.to_str().unwrap());
        let state = ViewsState::new();

        let text = state.get_yank_text(&loaded);
        assert!(text.is_some());
        assert!(!text.unwrap().is_empty());
    }

    // =============================================================================
    // v0.12.5 Multi-box detail tests
    // =============================================================================

    #[test]
    fn test_extract_relations_from_cypher() {
        // Test with typical Cypher query
        let cypher = r#"
            MATCH (p:Page {key: $nodeKey})
            OPTIONAL MATCH (p)-[r1:HAS_BLOCK]->(b:Block)
            OPTIONAL MATCH (b)-[r2:USES_ENTITY]->(e:Entity)
            OPTIONAL MATCH (e)-[r3:HAS_NATIVE]->(ec:EntityNative)
            RETURN p, r1, b, r2, e, r3, ec
        "#;

        let relations = extract_relations_from_cypher(cypher);

        assert_eq!(relations.len(), 3);
        assert!(relations.iter().any(|r| r.name == "HAS_BLOCK"));
        assert!(relations.iter().any(|r| r.name == "USES_ENTITY"));
        assert!(relations.iter().any(|r| r.name == "HAS_NATIVE"));
    }

    #[test]
    fn test_extract_relations_no_duplicates() {
        let cypher = r#"
            MATCH (p)-[:HAS_BLOCK]->(b1)
            MATCH (p)-[:HAS_BLOCK]->(b2)
        "#;

        let relations = extract_relations_from_cypher(cypher);
        assert_eq!(relations.len(), 1); // Only one HAS_BLOCK
    }

    #[test]
    fn test_extract_relations_empty() {
        let cypher = "MATCH (n) RETURN n";
        let relations = extract_relations_from_cypher(cypher);
        assert!(relations.is_empty());
    }

    #[test]
    fn test_infer_arc_family() {
        // Ownership
        assert_eq!(infer_arc_family("HAS_BLOCK"), "ownership");
        assert_eq!(infer_arc_family("HAS_PAGE"), "ownership");
        assert_eq!(infer_arc_family("BLOCK_OF"), "ownership");
        assert_eq!(infer_arc_family("CONTAINS_TERM"), "ownership");
        assert_eq!(infer_arc_family("BELONGS_TO_PROJECT"), "ownership");

        // Localization
        assert_eq!(infer_arc_family("FOR_LOCALE"), "localization");
        assert_eq!(infer_arc_family("HAS_VOICE"), "localization");
        assert_eq!(infer_arc_family("HAS_CULTURE"), "localization");

        // Generation
        assert_eq!(infer_arc_family("GENERATED"), "generation");
        assert_eq!(infer_arc_family("GENERATED_FROM"), "generation");
        assert_eq!(infer_arc_family("COMPILED_FROM"), "generation");
        assert_eq!(infer_arc_family("ASSEMBLES"), "generation");

        // Mining
        assert_eq!(infer_arc_family("TARGETS_KEYWORD"), "mining");
        assert_eq!(infer_arc_family("MONITORS_GEO"), "mining");

        // Semantic (default)
        assert_eq!(infer_arc_family("USES_ENTITY"), "semantic");
        assert_eq!(infer_arc_family("REFERENCES"), "semantic");
        assert_eq!(infer_arc_family("LINKS_TO"), "semantic");
    }

    #[test]
    fn test_studio_link() {
        let link = studio_link("gen-page");
        assert_eq!(link, "http://localhost:3000/?view=gen-page");
    }

    #[test]
    fn test_arc_family_color() {
        // Ensure all families have distinct colors
        let ownership = arc_family_color("ownership");
        let localization = arc_family_color("localization");
        let semantic = arc_family_color("semantic");
        let generation = arc_family_color("generation");
        let mining = arc_family_color("mining");
        let unknown = arc_family_color("unknown");

        assert_ne!(ownership, localization);
        assert_ne!(semantic, generation);
        assert_ne!(mining, unknown);
        assert_eq!(unknown, Color::Gray);
    }

    #[test]
    fn test_view_detail_section_cycle() {
        let mut state = ViewsState::new();
        assert_eq!(state.detail_section, ViewDetailSection::Info);

        state.next_section();
        assert_eq!(state.detail_section, ViewDetailSection::Cypher);

        state.next_section();
        assert_eq!(state.detail_section, ViewDetailSection::Relations);

        state.next_section();
        assert_eq!(state.detail_section, ViewDetailSection::Schema);

        state.next_section();
        assert_eq!(state.detail_section, ViewDetailSection::Info); // Wraps around

        state.prev_section();
        assert_eq!(state.detail_section, ViewDetailSection::Schema);
    }

    #[test]
    fn test_cypher_scroll() {
        let mut state = ViewsState::new();
        state.detail_section = ViewDetailSection::Cypher;

        state.scroll_down(5);
        assert_eq!(state.cypher_scroll, 5);

        state.scroll_down(5);
        assert_eq!(state.cypher_scroll, 10);

        state.scroll_up(3);
        assert_eq!(state.cypher_scroll, 7);

        state.scroll_up(10); // Saturating subtraction
        assert_eq!(state.cypher_scroll, 0);
    }

    #[test]
    fn test_cypher_scroll_only_in_cypher_section() {
        let mut state = ViewsState::new();
        assert_eq!(state.detail_section, ViewDetailSection::Info);

        state.scroll_down(5);
        assert_eq!(state.cypher_scroll, 0); // Should not change in Info section

        state.detail_section = ViewDetailSection::Cypher;
        state.scroll_down(5);
        assert_eq!(state.cypher_scroll, 5); // Now it should change
    }
}
