//! Input handling for the TUI app.
//!
//! Keyboard and mouse event handlers extracted from `app/mod.rs`.

use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

use super::constants::*;
use super::state::{Focus, NavMode, Panel};
use super::App;
use crate::tui::handlers::dispatch_mode_handler;

// =============================================================================
// Event Handlers
// =============================================================================

impl App {
    /// Handle mouse input. Returns true if state changed (needs re-render).
    /// Mouse scroll works on the panel under the cursor, regardless of focus.
    pub fn handle_mouse(&mut self, event: MouseEvent) -> bool {
        match event.kind {
            MouseEventKind::ScrollUp => {
                if let Some(panel) = self.panel_rects.hit_test(event.column, event.row) {
                    if panel == Panel::Tree {
                        if self.tree_cursor > 0 {
                            self.tree_cursor -= 1;
                            self.ensure_cursor_visible();
                            self.load_yaml_for_current();
                            return true;
                        }
                    } else if let Some((scroll, _)) = self.panel_scroll_mut(panel.to_focus()) {
                        if *scroll > 0 {
                            *scroll = scroll.saturating_sub(MOUSE_SCROLL_LINES);
                            return true;
                        }
                    }
                }
            },
            MouseEventKind::ScrollDown => {
                if let Some(panel) = self.panel_rects.hit_test(event.column, event.row) {
                    if panel == Panel::Tree {
                        let max = self.current_item_count().saturating_sub(1);
                        if self.tree_cursor < max {
                            self.tree_cursor += 1;
                            self.ensure_cursor_visible();
                            self.load_yaml_for_current();
                            return true;
                        }
                    } else if let Some((scroll, max)) = self.panel_scroll_mut(panel.to_focus()) {
                        if *scroll < max {
                            *scroll = (*scroll + MOUSE_SCROLL_LINES).min(max);
                            return true;
                        }
                    }
                }
            },
            // Click to focus: change panel focus when clicking
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(panel) = self.panel_rects.hit_test(event.column, event.row) {
                    let new_focus = panel.to_focus();
                    if self.focus != new_focus {
                        self.focus = new_focus;
                        return true;
                    }
                }
            },
            // Ignore other mouse events (right-click, middle-click, drags, etc.)
            _ => {},
        }
        false
    }

    /// Get mutable scroll position and max scroll value for a scrollable panel.
    ///
    /// Returns `None` for Tree (cursor-based navigation).
    /// Used by keyboard and mouse scroll handlers to avoid repetitive match arms.
    fn panel_scroll_mut(&mut self, focus: Focus) -> Option<(&mut usize, usize)> {
        match focus {
            Focus::Identity => {
                let max = self.identity_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                Some((&mut self.identity_scroll, max))
            },
            Focus::Content => {
                let max = self.yaml.line_count.saturating_sub(YAML_SCROLL_MARGIN);
                Some((&mut self.yaml.scroll, max))
            },
            Focus::Props => {
                let max = self.props_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                Some((&mut self.props_scroll, max))
            },
            Focus::Arcs => {
                let max = self.arcs_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                Some((&mut self.arcs_scroll, max))
            },
            Focus::Tree => None,
        }
    }

    /// Handle key input. Returns true if state changed (needs re-render).
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Help overlay - any key closes it
        if self.overlays.help_active {
            self.overlays.help_active = false;
            return true;
        }

        // Legend overlay - any key closes it
        if self.overlays.legend_active {
            self.overlays.legend_active = false;
            return true;
        }

        // Recent items overlay - handles navigation and selection
        if self.overlays.recent_items_active {
            return self.handle_recent_items_key(key);
        }


        // Search mode captures all input
        if self.search.active {
            return self.handle_search_key(key);
        }

        // Ctrl modifiers: search nav (n/p) + vertical panel nav (up/down)
        if key
            .modifiers
            .contains(crossterm::event::KeyModifiers::CONTROL)
        {
            match key.code {
                KeyCode::Char('n') => {
                    self.next_search_result();
                    return true;
                },
                KeyCode::Char('p') => {
                    self.prev_search_result();
                    return true;
                },
                // Ctrl+Up/Down: vertical panel switching (Identity↔Content, Props↔Arcs)
                KeyCode::Up => {
                    self.focus = self.focus.up();
                    self.set_status(self.focus.name());
                    return true;
                },
                KeyCode::Down => {
                    self.focus = self.focus.down();
                    self.set_status(self.focus.name());
                    return true;
                },
                _ => {},
            }
        }

        // Mode-specific key handling (Graph only)
        // Returns early if handled, falls through to global handlers otherwise
        if let Some(result) = dispatch_mode_handler(self, key) {
            return result;
        }

        match key.code {
            // Open help (? = vim-style help)
            KeyCode::Char('?') => {
                self.overlays.help_active = true;
                true
            },

            // Open search (/ = vim-style search)
            KeyCode::Char('/') => {
                self.search.active = true;
                true
            },


            // Open color legend (F1 = accessible, out of flow)
            KeyCode::F(1) => {
                self.overlays.legend_active = true;
                true
            },

            // Open recent items popup (` = backtick)
            KeyCode::Char('`') => {
                if !self.nav_history.is_empty() {
                    self.overlays.recent_items_active = true;
                    self.overlays.recent_items_cursor = 0;
                }
                true
            },

            // Mode switching: Graph [1] + Flow [2]
            KeyCode::Char('1') => {
                self.mode_cursors[self.mode.index()] = self.tree_cursor;
                self.mode = NavMode::Graph;
                self.tree_cursor = self.mode_cursors[0];
                true
            },
            KeyCode::Char('2') => {
                self.mode_cursors[self.mode.index()] = self.tree_cursor;
                self.mode = NavMode::Flow;
                true
            },

            // Panel navigation: Tab cycles through 5 panels
            // Tree [1] → Identity [2] → Content [3] → Props [4] → Arcs [5]
            KeyCode::Tab => {
                self.focus = self.focus.next();
                self.set_status(self.focus.name());
                true
            },
            KeyCode::BackTab => {
                self.focus = self.focus.prev();
                self.set_status(self.focus.name());
                true
            },
            KeyCode::Left => {
                // Left arrow: spatial navigation left
                self.focus = self.focus.left();
                self.set_status(self.focus.name());
                true
            },
            KeyCode::Right => {
                // Right arrow: spatial navigation right
                self.focus = self.focus.right();
                self.set_status(self.focus.name());
                true
            },
            // NOTE: Up/Down arrows handled below for in-panel navigation (cursor/scroll)
            // Left/Right = panel switching, Up/Down/j/k = in-panel (vim/lazygit pattern)

            // Enter: toggle collapse/expand (Tree), toggle sections (Content), or expand property (Info)
            KeyCode::Enter => {
                match self.focus {
                    Focus::Tree => {
                        self.toggle_tree_item();
                    },
                    Focus::Identity => {
                        // Identity panel - no action on Enter yet
                        // Future: could toggle between expanded/collapsed view
                    },
                    Focus::Content => {
                        // Toggle instance panel sections collapse/expand
                        // Cycles: all expanded → STANDARD collapsed → both collapsed → all expanded
                        if !self.instance_standard_collapsed && !self.instance_specific_collapsed {
                            // Both expanded → collapse STANDARD
                            self.instance_standard_collapsed = true;
                        } else if self.instance_standard_collapsed
                            && !self.instance_specific_collapsed
                        {
                            // STANDARD collapsed → collapse both
                            self.instance_specific_collapsed = true;
                        } else {
                            // Any other state → expand both
                            self.instance_standard_collapsed = false;
                            self.instance_specific_collapsed = false;
                        }
                    },
                    Focus::Props => {
                        // Toggle expanded property text (word-wrap on multiple lines)
                        self.expanded_property = !self.expanded_property;
                    },
                    Focus::Arcs => {
                        // No-op for Arcs focus (future: could navigate to selected arc)
                    },
                }
                true
            },

            // h/l: Panel navigation OR tree toggle
            // When in Tree: toggle collapse/expand (existing behavior)
            // When in other panels: linear panel navigation (h=prev, l=next)
            KeyCode::Char('h') => {
                if self.focus == Focus::Tree {
                    self.toggle_tree_item();
                } else {
                    self.focus = self.focus.prev();
                    self.set_status(self.focus.name());
                }
                true
            },
            KeyCode::Char('l') => {
                if self.focus == Focus::Tree {
                    self.toggle_tree_item();
                } else {
                    self.focus = self.focus.next();
                    self.set_status(self.focus.name());
                }
                true
            },
            // Space: toggle collapse/expand (Tree only)
            KeyCode::Char(' ') => {
                if self.focus == Focus::Tree {
                    self.toggle_tree_item();
                }
                true
            },
            KeyCode::Char('H') => {
                self.tree.collapse_all();
                self.tree_cursor = 0;
                self.tree_scroll = 0;
                true
            },
            KeyCode::Char('L') => {
                self.tree.expand_all();
                true
            },

            // Expand/Collapse subtree under cursor (e/c)
            KeyCode::Char('e') | KeyCode::Char('E') if key.modifiers.is_empty() => {
                // E = Expand subtree under cursor
                if self.focus == Focus::Tree {
                    let data_mode = self.is_graph_mode();
                    if let Some(key) =
                        self.tree
                            .collapse_key_at(self.tree_cursor, data_mode, self.hide_empty)
                    {
                        self.tree.expand_subtree(&key);
                    }
                }
                true
            },
            KeyCode::Char('c') => {
                // c = Collapse subtree under cursor (Tree) OR copy property value (Info/Properties)
                if self.focus == Focus::Tree {
                    let data_mode = self.is_graph_mode();
                    if let Some(key) =
                        self.tree
                            .collapse_key_at(self.tree_cursor, data_mode, self.hide_empty)
                    {
                        self.tree.collapse_subtree(&key);
                    }
                } else if self.focus == Focus::Props {
                    // Copy focused property value
                    self.copy_focused_property();
                }
                true
            },

            // Toggle hide empty (0) - only in Data mode
            KeyCode::Char('0') => {
                if self.is_graph_mode() {
                    self.hide_empty = !self.hide_empty;
                    self.set_status(if self.hide_empty {
                        "Hide empty: ON"
                    } else {
                        "Hide empty: OFF"
                    });
                    // Reset cursor to avoid pointing to hidden item
                    self.tree_cursor = 0;
                    self.tree_scroll = 0;
                }
                true
            },

            // Jump to first/last (vim-style: g/G)
            KeyCode::Char('g') => {
                if self.focus == Focus::Tree {
                    self.tree_cursor = 0;
                    self.tree_scroll = 0;
                    self.load_yaml_for_current();
                } else if let Some((scroll, _)) = self.panel_scroll_mut(self.focus) {
                    *scroll = 0;
                }
                true
            },
            KeyCode::Char('G') => {
                if self.focus == Focus::Tree {
                    let max = self.current_item_count().saturating_sub(1);
                    self.tree_cursor = max;
                    self.ensure_cursor_visible();
                    self.load_yaml_for_current();
                } else if let Some((scroll, max)) = self.panel_scroll_mut(self.focus) {
                    *scroll = max;
                }
                true
            },

            // Navigation: ↑/k scroll up, ↓/j scroll down (in focused panel)
            KeyCode::Up | KeyCode::Char('k') => {
                if self.focus == Focus::Tree {
                    if self.tree_cursor > 0 {
                        self.tree_cursor -= 1;
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                    }
                } else if self.focus == Focus::Props {
                    // Navigate properties with j/k (dual-mode)
                    if self.schema_overlay.matched_properties.is_some()
                        && self.focused_property_idx > 0
                    {
                        self.focused_property_idx -= 1;
                        self.expanded_property = false;
                    } else if self.props_scroll > 0 {
                        self.props_scroll -= 1;
                    }
                } else if let Some((scroll, _)) = self.panel_scroll_mut(self.focus) {
                    if *scroll > 0 {
                        *scroll -= 1;
                    }
                }
                true
            },
            KeyCode::Down | KeyCode::Char('j') => {
                if self.focus == Focus::Tree {
                    let max = self.current_item_count().saturating_sub(1);
                    if self.tree_cursor < max {
                        self.tree_cursor += 1;
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                    }
                } else if self.focus == Focus::Props {
                    // Navigate properties with j/k (dual-mode)
                    if let Some(matched) = &self.schema_overlay.matched_properties {
                        let max_idx = matched.len().saturating_sub(1);
                        if self.focused_property_idx < max_idx {
                            self.focused_property_idx += 1;
                            self.expanded_property = false;
                        }
                    } else {
                        let max_scroll =
                            self.props_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                        if self.props_scroll < max_scroll {
                            self.props_scroll += 1;
                        }
                    }
                } else if let Some((scroll, max)) = self.panel_scroll_mut(self.focus) {
                    if *scroll < max {
                        *scroll += 1;
                    }
                }
                true
            },

            // Page scroll: d/u vim-style
            KeyCode::Char('d') => {
                if self.focus == Focus::Tree {
                    let max = self.current_item_count().saturating_sub(1);
                    self.tree_cursor = (self.tree_cursor + PAGE_SCROLL_AMOUNT).min(max);
                    self.ensure_cursor_visible();
                    self.load_yaml_for_current();
                } else if let Some((scroll, max)) = self.panel_scroll_mut(self.focus) {
                    *scroll = (*scroll + PAGE_SCROLL_AMOUNT).min(max);
                }
                true
            },
            KeyCode::Char('u') => {
                if self.focus == Focus::Tree {
                    self.tree_cursor = self.tree_cursor.saturating_sub(PAGE_SCROLL_AMOUNT);
                    self.ensure_cursor_visible();
                    self.load_yaml_for_current();
                } else if let Some((scroll, _)) = self.panel_scroll_mut(self.focus) {
                    *scroll = scroll.saturating_sub(PAGE_SCROLL_AMOUNT);
                }
                true
            },

            // 'r' key: Refresh
            KeyCode::Char('r') => {
                self.pending_refresh = true;
                self.set_status("Refreshing...");
                true
            },

            // Yank (smart copy based on selected box)
            KeyCode::Char('y') => {
                self.yank_focused_content();
                true
            },

            // Yank JSON properties (Y)
            KeyCode::Char('Y') => {
                self.yank_current_json();
                true
            },

            // Jump to parent [p]
            KeyCode::Char('p') => {
                if let Some(parent_cursor) = self.tree.find_parent_cursor(
                    self.tree_cursor,
                    self.is_graph_mode(),
                    self.hide_empty,
                ) {
                    self.tree_cursor = parent_cursor;
                    self.ensure_cursor_visible();
                    self.set_status("↑ Parent");
                }
                true
            },

            // Toggle schema overlay (s) - only in Data mode
            KeyCode::Char('s') => {
                if self.is_graph_mode() {
                    self.schema_overlay.enabled = !self.schema_overlay.enabled;
                    // Load/clear matched properties based on new state
                    self.update_schema_match_for_current();
                    self.set_status(if self.schema_overlay.enabled {
                        "Schema overlay ON"
                    } else {
                        "Schema overlay OFF"
                    });
                }
                true
            },

            // Toggle JSON pretty-print (J) - only when viewing properties
            KeyCode::Char('J') => {
                self.json_pretty = !self.json_pretty;
                self.set_status(if self.json_pretty {
                    "JSON pretty-print ON"
                } else {
                    "JSON compact mode"
                });
                true
            },

            // Property focus navigation (+/-) - Feature 3: Truncate Intelligent
            // Navigate focused property in schema overlay
            KeyCode::Char('+') | KeyCode::Char('=') => {
                if self.is_graph_mode() && self.schema_overlay.enabled {
                    if let Some(matched) = &self.schema_overlay.matched_properties {
                        let max_idx = matched.len().saturating_sub(1);
                        self.focused_property_idx = (self.focused_property_idx + 1).min(max_idx);
                        self.expanded_property = false; // Collapse when changing property
                    }
                }
                true
            },
            KeyCode::Char('-') | KeyCode::Char('_') => {
                if self.is_graph_mode() && self.schema_overlay.enabled {
                    self.focused_property_idx = self.focused_property_idx.saturating_sub(1);
                    self.expanded_property = false; // Collapse when changing property
                }
                true
            },

            // Navigation history: back (Ctrl+o)
            KeyCode::Char('o')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.nav_back();
                true
            },

            // Navigation history: forward (Ctrl+i)
            KeyCode::Char('i')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.nav_forward();
                true
            },

            // Esc: exit filtered mode
            KeyCode::Esc => {
                if self.is_filtered_graph_mode() {
                    self.exit_filtered_data_mode();
                    return true;
                }
                false
            },

            // Open YAML in external editor (O = shift+o)
            KeyCode::Char('O') => {
                self.open_yaml_in_editor();
                true
            },

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
            },

            // Select result
            KeyCode::Enter => {
                self.select_search_result();
                true
            },

            // Navigate results (arrow keys and vim j/k)
            KeyCode::Up | KeyCode::Char('k') => {
                if self.search.cursor > 0 {
                    self.search.cursor -= 1;
                }
                true
            },
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.search.results.len().saturating_sub(1);
                if self.search.cursor < max {
                    self.search.cursor += 1;
                }
                true
            },

            // Type character (j/k are handled above for navigation)
            // Security: Limit search query length to prevent memory exhaustion
            KeyCode::Char(c) => {
                const MAX_SEARCH_LEN: usize = 256;
                if self.search.query.len() < MAX_SEARCH_LEN {
                    self.search.query.push(c);
                    self.update_search();
                }
                true
            },

            // Delete character
            KeyCode::Backspace => {
                self.search.query.pop();
                self.update_search();
                true
            },

            _ => false,
        }
    }

    /// Handle key events for the recent items popup.
    fn handle_recent_items_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            // Close popup
            KeyCode::Esc | KeyCode::Char('`') => {
                self.overlays.recent_items_active = false;
                true
            },

            // Select and jump to item
            KeyCode::Enter => {
                self.select_recent_item();
                true
            },

            // Navigate up (arrows, vim j/k)
            KeyCode::Up | KeyCode::Char('k') => {
                if self.overlays.recent_items_cursor > 0 {
                    self.overlays.recent_items_cursor -= 1;
                }
                true
            },

            // Navigate down
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.nav_history.len().saturating_sub(1);
                if self.overlays.recent_items_cursor < max {
                    self.overlays.recent_items_cursor += 1;
                }
                true
            },

            _ => true, // Consume all other keys while popup is open
        }
    }

    /// Select and jump to the currently highlighted recent item.
    fn select_recent_item(&mut self) {
        // History is stored oldest→newest, but we display newest first
        let display_idx = self.overlays.recent_items_cursor;
        let history_idx = self.nav_history.len().saturating_sub(1 + display_idx);

        if let Some(&(mode, cursor)) = self.nav_history.get(history_idx) {
            self.overlays.recent_items_active = false;
            self.mode = mode;
            self.tree_cursor = cursor;
            self.ensure_cursor_visible();
            self.load_yaml_for_current();
            self.set_status("↩ Jumped to recent item");
        }
    }
}
