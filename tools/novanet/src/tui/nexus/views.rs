//! Views Mode — Schema views explorer (v0.12.5).
//!
//! Loads views from `views.yaml` (single source of truth shared with Studio).
//! Shows available views organized by category.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

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
        let mut categories: Vec<(String, ViewCategoryDef)> = file
            .categories
            .into_iter()
            .collect();
        categories.sort_by_key(|(k, _)| {
            category_order.iter().position(|&c| c == k).unwrap_or(99)
        });

        Self {
            categories,
            views: file.views,
        }
    }

    /// Get views in a category.
    pub fn views_in_category(&self, category: &str) -> Vec<&SimpleViewEntry> {
        self.views.iter().filter(|v| v.category == category).collect()
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

/// State for the Views mode.
#[derive(Debug, Clone, Default)]
pub struct ViewsState {
    /// Currently selected category index.
    pub category_cursor: usize,
    /// Currently selected view within category.
    pub view_cursor: usize,
    /// Whether showing Query-First concept panel.
    pub show_concept: bool,
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
    icon.as_ref().map(|i| i.terminal.as_str()).unwrap_or(default)
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
        // Normal view: categories + views + detail
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        render_views_list(f, views_state, loaded, chunks[0]);
        render_view_detail(f, views_state, loaded, chunks[1]);
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
        let cat_color = cat_def.color.as_deref().map(parse_color).unwrap_or(Color::Cyan);

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

                lines.push(Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(&view.name, style),
                ]));
            }
        }

        lines.push(Line::from(""));
    }

    // Navigation hints
    lines.push(Line::from(Span::styled(
        "[j/k:nav] [h/l:category] [?:concept]",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Render the detail panel for selected view.
#[allow(clippy::vec_init_then_push)]
fn render_view_detail(f: &mut Frame, state: &ViewsState, loaded: &LoadedViews, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " VIEW DETAIL ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let Some(view) = state.current_view(loaded) else {
        let empty = Paragraph::new("Select a view to see details");
        f.render_widget(empty, inner);
        return;
    };

    let view_color = view.color.as_deref().map(parse_color).unwrap_or(Color::Cyan);

    let mut lines: Vec<Line> = Vec::new();

    // View name
    lines.push(Line::from(Span::styled(
        &view.name,
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));

    // Separator
    lines.push(Line::from(Span::styled(
        "\u{2550}".repeat(inner.width.saturating_sub(2) as usize),
        Style::default().fg(view_color),
    )));
    lines.push(Line::from(""));

    // Metadata
    lines.push(Line::from(vec![
        Span::styled("ID:       ", Style::default().fg(Color::DarkGray)),
        Span::styled(&view.id, Style::default().fg(Color::Cyan)),
    ]));

    let view_icon = terminal_icon(&view.icon, "●");
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
            Span::styled("contextual (sidebar)", Style::default().fg(Color::Green)),
        ]));
    }

    if let Some(ref types) = view.applicable_types {
        if !types.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("Types:    ", Style::default().fg(Color::DarkGray)),
                Span::styled(types.join(", "), Style::default().fg(Color::Rgb(180, 180, 180))),
            ]));
        }
    }

    lines.push(Line::from(""));

    // Description
    lines.push(Line::from(Span::styled(
        "Description:",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        &view.description,
        Style::default().fg(Color::Rgb(180, 180, 180)),
    )));
    lines.push(Line::from(""));

    // Cypher preview (truncated)
    if let Some(ref cypher) = view.cypher {
        lines.push(Line::from(Span::styled(
            "Cypher:",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        for (i, line) in cypher.lines().take(6).enumerate() {
            let style = if i == 5 {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::Rgb(150, 200, 150))
            };
            let display = if i == 5 { "  ..." } else { line };
            lines.push(Line::from(Span::styled(format!("  {display}"), style)));
        }
        lines.push(Line::from(""));
    }

    // YAML path
    lines.push(Line::from(Span::styled(
        "Source:",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  packages/core/models/views.yaml",
        Style::default().fg(Color::Rgb(150, 180, 220)),
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
        assert_eq!(loaded.views.len(), 10, "expected 10 views");

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
}
