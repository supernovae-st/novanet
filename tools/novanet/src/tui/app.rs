//! App state for TUI v2.

use std::fs;
use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};

use super::data::{TaxonomyTree, TreeItem};

/// Navigation mode (matches Studio).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavMode {
    Data,
    #[default]
    Meta,
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
            NavMode::Data => NavMode::Meta,
            NavMode::Meta => NavMode::Overlay,
            NavMode::Overlay => NavMode::Query,
            NavMode::Query => NavMode::Data,
        }
    }
}

/// Which panel has focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    #[default]
    Tree,
    Detail,
}

/// Main app state.
#[allow(dead_code)]
pub struct App {
    pub mode: NavMode,
    pub focus: Focus,
    pub tree_cursor: usize,
    pub tree_scroll: usize,    // Scroll offset for tree
    pub tree_height: usize,    // Visible height (set by UI)
    pub detail_scroll: usize,
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
    pub root_path: String,
}

impl App {
    pub fn new(tree: TaxonomyTree, root_path: String) -> Self {
        let mut app = Self {
            mode: NavMode::Meta,
            focus: Focus::Tree,
            tree_cursor: 0,
            tree_scroll: 0,
            tree_height: 20, // Default, updated by UI
            detail_scroll: 0,
            tree,
            search_active: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_cursor: 0,
            help_active: false,
            yaml_content: String::new(),
            yaml_path: String::new(),
            yaml_scroll: 0,
            root_path,
        };
        app.load_yaml_for_current();
        app
    }

    /// Load YAML content for the current cursor position.
    pub fn load_yaml_for_current(&mut self) {
        match self.tree.item_at(self.tree_cursor) {
            Some(TreeItem::Kind(_, _, kind)) => {
                let full_path = Path::new(&self.root_path).join(&kind.yaml_path);
                self.yaml_path = kind.yaml_path.clone();
                self.yaml_content = fs::read_to_string(&full_path)
                    .unwrap_or_else(|_| format!("# File not found: {}", full_path.display()));
                self.yaml_scroll = 0;
            }
            Some(TreeItem::EdgeKind(_, _)) => {
                // EdgeKinds are in relations.yaml
                let rel_path = "packages/core/models/relations.yaml";
                let full_path = Path::new(&self.root_path).join(rel_path);
                self.yaml_path = rel_path.to_string();
                self.yaml_content = fs::read_to_string(&full_path)
                    .unwrap_or_else(|_| format!("# File not found: {}", full_path.display()));
                self.yaml_scroll = 0;
            }
            _ => {
                self.yaml_path.clear();
                self.yaml_content.clear();
                self.yaml_scroll = 0;
            }
        }
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

        // Relations section header
        if "relations".contains(&query) {
            self.search_results.push(idx);
        }
        idx += 1;

        if !self.tree.is_collapsed("relations") {
            for family in &self.tree.edge_families {
                if family.display_name.to_lowercase().contains(&query)
                    || family.key.to_lowercase().contains(&query)
                {
                    self.search_results.push(idx);
                }
                idx += 1;

                if !self.tree.is_collapsed(&format!("family:{}", family.key)) {
                    for edge_kind in &family.edge_kinds {
                        if edge_kind.display_name.to_lowercase().contains(&query)
                            || edge_kind.key.to_lowercase().contains(&query)
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

            // Mode switching: 1-4 direct, N cycle
            KeyCode::Char('1') => {
                self.mode = NavMode::Data;
                true
            }
            KeyCode::Char('2') => {
                self.mode = NavMode::Meta;
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

            // Panel focus: ←→ or Tab
            KeyCode::Left => {
                self.focus = Focus::Tree;
                true
            }
            KeyCode::Right | KeyCode::Tab => {
                self.focus = if self.focus == Focus::Tree {
                    Focus::Detail
                } else {
                    Focus::Tree
                };
                true
            }

            // Collapse/expand: h/l/H/L (vim-style)
            KeyCode::Char('h') => {
                // Collapse current node
                if let Some(key) = self.tree.collapse_key_at(self.tree_cursor) {
                    self.tree.collapse(&key);
                }
                true
            }
            KeyCode::Char('l') => {
                // Expand current node
                if let Some(key) = self.tree.collapse_key_at(self.tree_cursor) {
                    self.tree.expand(&key);
                }
                true
            }
            KeyCode::Char('H') => {
                // Collapse all
                self.tree.collapse_all();
                self.tree_cursor = 0;
                self.tree_scroll = 0;
                true
            }
            KeyCode::Char('L') => {
                // Expand all
                self.tree.expand_all();
                true
            }

            // Tree navigation: ↑↓
            KeyCode::Up => {
                if self.tree_cursor > 0 {
                    self.tree_cursor -= 1;
                    self.ensure_cursor_visible();
                    self.load_yaml_for_current();
                }
                true
            }
            KeyCode::Down => {
                let max = self.tree.item_count().saturating_sub(1);
                if self.tree_cursor < max {
                    self.tree_cursor += 1;
                    self.ensure_cursor_visible();
                    self.load_yaml_for_current();
                }
                true
            }

            // YAML scroll: j/k (vim-style)
            KeyCode::Char('j') => {
                let max_scroll = self.yaml_content.lines().count().saturating_sub(10);
                if self.yaml_scroll < max_scroll {
                    self.yaml_scroll += 1;
                }
                true
            }
            KeyCode::Char('k') => {
                if self.yaml_scroll > 0 {
                    self.yaml_scroll -= 1;
                }
                true
            }
            // Page scroll: Ctrl+d / Ctrl+u
            KeyCode::Char('d') => {
                let max_scroll = self.yaml_content.lines().count().saturating_sub(10);
                self.yaml_scroll = (self.yaml_scroll + 10).min(max_scroll);
                true
            }
            KeyCode::Char('u') => {
                self.yaml_scroll = self.yaml_scroll.saturating_sub(10);
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
