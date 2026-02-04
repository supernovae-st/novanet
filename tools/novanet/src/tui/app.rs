//! App state for TUI v2.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};

use super::data::{TaxonomyTree, TreeItem};

/// Navigation mode (matches Studio).
/// Order: 1:Meta 2:Data 3:Overlay 4:Query
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavMode {
    #[default]
    Meta,
    Data,
    Overlay,
    Query,
}

impl NavMode {
    pub fn label(&self) -> &'static str {
        match self {
            NavMode::Data => "Data",
            NavMode::Meta => "Meta",
            NavMode::Overlay => "Overlay",
            NavMode::Query => "Query",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            NavMode::Meta => NavMode::Data,
            NavMode::Data => NavMode::Overlay,
            NavMode::Overlay => NavMode::Query,
            NavMode::Query => NavMode::Meta,
        }
    }
}

/// Which panel has focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    #[default]
    Tree,
    Info,
    Yaml,
}

impl Focus {
    /// Cycle to next focus panel.
    pub fn next(self) -> Self {
        match self {
            Focus::Tree => Focus::Info,
            Focus::Info => Focus::Yaml,
            Focus::Yaml => Focus::Tree,
        }
    }

    /// Cycle to previous focus panel.
    pub fn prev(self) -> Self {
        match self {
            Focus::Tree => Focus::Yaml,
            Focus::Info => Focus::Tree,
            Focus::Yaml => Focus::Info,
        }
    }
}

/// Main app state.
#[allow(dead_code)]
pub struct App {
    pub mode: NavMode,
    pub focus: Focus,
    pub tree_cursor: usize,
    pub tree_scroll: usize, // Scroll offset for tree
    pub tree_height: usize, // Visible height (set by UI)
    pub tree: TaxonomyTree,
    // Search state
    pub search_active: bool,
    pub search_query: String,
    pub search_results: Vec<usize>, // indices into flattened tree
    pub search_cursor: usize,
    // Help overlay
    pub help_active: bool,
    // YAML preview
    pub yaml_content: String,
    pub yaml_path: String,
    pub yaml_scroll: usize,
    pub yaml_line_count: usize, // Cached line count (avoids per-scroll recomputation)
    // Info panel scroll (separate from yaml)
    pub info_scroll: usize,
    pub info_line_count: usize, // Set by UI after building lines
    pub root_path: String,
    /// Cache of YAML file contents (path -> content).
    /// Avoids re-reading files on every scroll/navigation.
    pub yaml_cache: HashMap<String, String>,
}

impl App {
    pub fn new(tree: TaxonomyTree, root_path: String) -> Self {
        let mut app = Self {
            mode: NavMode::Meta,
            focus: Focus::Tree,
            tree_cursor: 0,
            tree_scroll: 0,
            tree_height: 20, // Default, updated by UI
            tree,
            search_active: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_cursor: 0,
            help_active: false,
            yaml_content: String::new(),
            yaml_path: String::new(),
            yaml_scroll: 0,
            yaml_line_count: 0,
            info_scroll: 0,
            info_line_count: 0,
            root_path,
            yaml_cache: HashMap::new(),
        };
        app.load_yaml_for_current();
        app
    }

    /// Load YAML content for the current cursor position.
    pub fn load_yaml_for_current(&mut self) {
        // Reset both scroll positions when changing items
        self.yaml_scroll = 0;
        self.info_scroll = 0;

        match self.tree.item_at(self.tree_cursor) {
            // Kind → individual YAML file
            Some(TreeItem::Kind(_, _, kind)) => {
                self.load_yaml_cached(&kind.yaml_path.clone());
            }
            // Realm, Layer → taxonomy.yaml
            Some(TreeItem::Realm(_))
            | Some(TreeItem::Layer(_, _))
            | Some(TreeItem::KindsSection) => {
                self.load_yaml_cached("packages/core/models/taxonomy.yaml");
            }
            // ArcFamily, ArcKind → relations.yaml
            Some(TreeItem::ArcFamily(_))
            | Some(TreeItem::ArcKind(_, _))
            | Some(TreeItem::ArcsSection) => {
                self.load_yaml_cached("packages/core/models/relations.yaml");
            }
            None => {
                self.yaml_path.clear();
                self.yaml_content.clear();
                self.yaml_line_count = 0;
            }
        }
    }

    /// Load YAML content with caching (avoids re-reading files on every navigation).
    fn load_yaml_cached(&mut self, relative_path: &str) {
        self.yaml_path = relative_path.to_string();

        // Check cache first
        if let Some(cached) = self.yaml_cache.get(relative_path) {
            self.yaml_content = cached.clone();
            self.yaml_line_count = self.yaml_content.lines().count();
            return;
        }

        // Load from disk
        let full_path = Path::new(&self.root_path).join(relative_path);
        let content = fs::read_to_string(&full_path)
            .unwrap_or_else(|_| format!("# File not found: {}", full_path.display()));

        // Update cache
        self.yaml_cache
            .insert(relative_path.to_string(), content.clone());

        // Update display
        self.yaml_content = content;
        self.yaml_line_count = self.yaml_content.lines().count();
    }

    /// Clear YAML cache (useful after external file modifications).
    #[allow(dead_code)]
    pub fn clear_yaml_cache(&mut self) {
        self.yaml_cache.clear();
    }

    /// Ensure cursor is visible by adjusting scroll.
    pub fn ensure_cursor_visible(&mut self) {
        // Scroll up if cursor is above viewport
        if self.tree_cursor < self.tree_scroll {
            self.tree_scroll = self.tree_cursor;
        }
        // Scroll down if cursor is below viewport
        if self.tree_cursor >= self.tree_scroll + self.tree_height {
            self.tree_scroll = self.tree_cursor.saturating_sub(self.tree_height - 1);
        }
    }

    /// Update search results based on current query (respects collapsed state).
    pub fn update_search(&mut self) {
        self.search_results.clear();
        if self.search_query.is_empty() {
            return;
        }

        let query = self.search_query.to_lowercase();
        let mut idx = 0;

        // Kinds section header
        if "kinds".contains(&query) {
            self.search_results.push(idx);
        }
        idx += 1;

        if !self.tree.is_collapsed("kinds") {
            for realm in &self.tree.realms {
                if realm.display_name.to_lowercase().contains(&query)
                    || realm.key.to_lowercase().contains(&query)
                {
                    self.search_results.push(idx);
                }
                idx += 1;

                if !self.tree.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        if layer.display_name.to_lowercase().contains(&query)
                            || layer.key.to_lowercase().contains(&query)
                        {
                            self.search_results.push(idx);
                        }
                        idx += 1;

                        if !self.tree.is_collapsed(&format!("layer:{}", layer.key)) {
                            for kind in &layer.kinds {
                                if kind.display_name.to_lowercase().contains(&query)
                                    || kind.key.to_lowercase().contains(&query)
                                {
                                    self.search_results.push(idx);
                                }
                                idx += 1;
                            }
                        }
                    }
                }
            }
        }

        // Arcs section header
        if "arcs".contains(&query) {
            self.search_results.push(idx);
        }
        idx += 1;

        if !self.tree.is_collapsed("arcs") {
            for family in &self.tree.arc_families {
                if family.display_name.to_lowercase().contains(&query)
                    || family.key.to_lowercase().contains(&query)
                {
                    self.search_results.push(idx);
                }
                idx += 1;

                if !self.tree.is_collapsed(&format!("family:{}", family.key)) {
                    for arc_kind in &family.arc_kinds {
                        if arc_kind.display_name.to_lowercase().contains(&query)
                            || arc_kind.key.to_lowercase().contains(&query)
                        {
                            self.search_results.push(idx);
                        }
                        idx += 1;
                    }
                }
            }
        }

        // Reset cursor if out of bounds
        if self.search_cursor >= self.search_results.len() {
            self.search_cursor = 0;
        }
    }

    /// Select current search result and close search.
    pub fn select_search_result(&mut self) {
        if let Some(&idx) = self.search_results.get(self.search_cursor) {
            self.tree_cursor = idx;
            self.ensure_cursor_visible();
        }
        self.close_search();
    }

    /// Close search overlay.
    pub fn close_search(&mut self) {
        self.search_active = false;
        self.search_query.clear();
        self.search_results.clear();
        self.search_cursor = 0;
    }

    /// Handle key input. Returns true if state changed (needs re-render).
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Help overlay - any key closes it
        if self.help_active {
            self.help_active = false;
            return true;
        }

        // Search mode captures all input
        if self.search_active {
            return self.handle_search_key(key);
        }

        match key.code {
            // Open help
            KeyCode::Char('/') => {
                self.help_active = true;
                true
            }

            // Open search (f = find)
            KeyCode::Char('f') => {
                self.search_active = true;
                true
            }

            // Mode switching: 1-4 direct (1=Meta, 2=Data, 3=Overlay, 4=Query)
            KeyCode::Char('1') => {
                self.mode = NavMode::Meta;
                true
            }
            KeyCode::Char('2') => {
                self.mode = NavMode::Data;
                true
            }
            KeyCode::Char('3') => {
                self.mode = NavMode::Overlay;
                true
            }
            KeyCode::Char('4') => {
                self.mode = NavMode::Query;
                true
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.mode = self.mode.cycle();
                true
            }

            // Panel focus: Tab cycles, ←→ for quick nav
            KeyCode::Tab => {
                self.focus = self.focus.next();
                true
            }
            KeyCode::BackTab => {
                self.focus = self.focus.prev();
                true
            }
            KeyCode::Left => {
                self.focus = Focus::Tree;
                true
            }
            KeyCode::Right => {
                // Right goes to Info (or Yaml if already on Info)
                self.focus = match self.focus {
                    Focus::Tree => Focus::Info,
                    Focus::Info => Focus::Yaml,
                    Focus::Yaml => Focus::Yaml,
                };
                true
            }

            // Toggle collapse/expand: h/l/Space/Enter - only when Tree focused
            KeyCode::Char('h') | KeyCode::Char('l') | KeyCode::Char(' ') | KeyCode::Enter => {
                if self.focus == Focus::Tree {
                    if let Some(key) = self.tree.collapse_key_at(self.tree_cursor) {
                        self.tree.toggle(&key);
                    }
                }
                true
            }
            KeyCode::Char('H') => {
                self.tree.collapse_all();
                self.tree_cursor = 0;
                self.tree_scroll = 0;
                true
            }
            KeyCode::Char('L') => {
                self.tree.expand_all();
                true
            }

            // Navigation: ↑↓ and j/k are context-aware (scroll focused panel)
            KeyCode::Up | KeyCode::Char('k') => {
                match self.focus {
                    Focus::Tree => {
                        if self.tree_cursor > 0 {
                            self.tree_cursor -= 1;
                            self.ensure_cursor_visible();
                            self.load_yaml_for_current();
                        }
                    }
                    Focus::Info => {
                        if self.info_scroll > 0 {
                            self.info_scroll -= 1;
                        }
                    }
                    Focus::Yaml => {
                        if self.yaml_scroll > 0 {
                            self.yaml_scroll -= 1;
                        }
                    }
                }
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                match self.focus {
                    Focus::Tree => {
                        let max = self.tree.item_count().saturating_sub(1);
                        if self.tree_cursor < max {
                            self.tree_cursor += 1;
                            self.ensure_cursor_visible();
                            self.load_yaml_for_current();
                        }
                    }
                    Focus::Info => {
                        let max_scroll = self.info_line_count.saturating_sub(5);
                        if self.info_scroll < max_scroll {
                            self.info_scroll += 1;
                        }
                    }
                    Focus::Yaml => {
                        let max_scroll = self.yaml_line_count.saturating_sub(10);
                        if self.yaml_scroll < max_scroll {
                            self.yaml_scroll += 1;
                        }
                    }
                }
                true
            }

            // Page scroll: d/u (vim-style, context-aware)
            KeyCode::Char('d') => {
                match self.focus {
                    Focus::Tree => {
                        let max = self.tree.item_count().saturating_sub(1);
                        self.tree_cursor = (self.tree_cursor + 10).min(max);
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                    }
                    Focus::Info => {
                        let max_scroll = self.info_line_count.saturating_sub(5);
                        self.info_scroll = (self.info_scroll + 10).min(max_scroll);
                    }
                    Focus::Yaml => {
                        let max_scroll = self.yaml_line_count.saturating_sub(10);
                        self.yaml_scroll = (self.yaml_scroll + 10).min(max_scroll);
                    }
                }
                true
            }
            KeyCode::Char('u') => {
                match self.focus {
                    Focus::Tree => {
                        self.tree_cursor = self.tree_cursor.saturating_sub(10);
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                    }
                    Focus::Info => {
                        self.info_scroll = self.info_scroll.saturating_sub(10);
                    }
                    Focus::Yaml => {
                        self.yaml_scroll = self.yaml_scroll.saturating_sub(10);
                    }
                }
                true
            }

            _ => false,
        }
    }

    /// Handle key input in search mode.
    fn handle_search_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            // Close search
            KeyCode::Esc => {
                self.close_search();
                true
            }

            // Select result
            KeyCode::Enter => {
                self.select_search_result();
                true
            }

            // Navigate results
            KeyCode::Up => {
                if self.search_cursor > 0 {
                    self.search_cursor -= 1;
                }
                true
            }
            KeyCode::Down => {
                let max = self.search_results.len().saturating_sub(1);
                if self.search_cursor < max {
                    self.search_cursor += 1;
                }
                true
            }

            // Type character
            KeyCode::Char(c) => {
                self.search_query.push(c);
                self.update_search();
                true
            }

            // Delete character
            KeyCode::Backspace => {
                self.search_query.pop();
                self.update_search();
                true
            }

            _ => false,
        }
    }
}
