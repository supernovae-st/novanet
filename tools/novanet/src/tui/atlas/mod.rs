//! Atlas Mode — Interactive architecture visualizations.
//!
//! Atlas Mode provides 6 views for understanding NovaNet's architecture:
//! - [a] Spreading Activation — Cognitive science math behind context assembly
//! - [b] Knowledge Atoms — Selective loading vs monolithic blobs
//! - [c] Generation Pipeline — Block generation flow (not translation)
//! - [d] View Traversal — Debug the 12 view definitions
//! - [e] Page Composition — Complete anatomy of a Page
//! - [f] Realm Map — Bird's-eye view of 2-realm architecture

mod state;

/// Available locales for Atlas mode.
const ATLAS_LOCALES: &[&str] = &["en-US", "fr-FR", "es-ES", "de-DE", "ja-JP", "ar-SA"];

pub use state::{
    ActivationTask, AtlasState, AtlasView, BlockData, BlockL10nData, EntityData, EntityL10nData,
    PageCompositionData, PageL10nData, SeoKeywordData,
};

use crossterm::event::{KeyCode, KeyEvent};

impl AtlasState {
    /// Handle key input in Atlas mode. Returns true if state changed.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            // View switching: a-f
            KeyCode::Char('a') => {
                self.current_view = AtlasView::SpreadingActivation;
                true
            }
            KeyCode::Char('b') => {
                self.current_view = AtlasView::KnowledgeAtoms;
                true
            }
            KeyCode::Char('c') => {
                self.current_view = AtlasView::GenerationPipeline;
                true
            }
            KeyCode::Char('v') => {
                // 'd' is used for demo toggle, use 'v' for View traversal
                self.current_view = AtlasView::ViewTraversal;
                true
            }
            KeyCode::Char('e') => {
                self.current_view = AtlasView::PageComposition;
                true
            }
            KeyCode::Char('r') => {
                // 'f' is used for search, use 'r' for Realm map
                self.current_view = AtlasView::RealmMap;
                true
            }

            // Demo mode toggle
            KeyCode::Char('d') => {
                self.demo_mode = !self.demo_mode;
                true
            }

            // Locale switching
            KeyCode::Char('l') => {
                self.cycle_locale();
                true
            }

            // Task cycling (Spreading Activation)
            KeyCode::Char('t') => {
                if self.current_view == AtlasView::SpreadingActivation {
                    self.activation_task = self.activation_task.next();
                    true
                } else {
                    false
                }
            }

            // Navigation within views
            KeyCode::Left | KeyCode::Char('h') => self.navigate_left(),
            KeyCode::Right => self.navigate_right(),
            KeyCode::Up | KeyCode::Char('k') => self.navigate_up(),
            KeyCode::Down | KeyCode::Char('j') => self.navigate_down(),

            // Scroll
            KeyCode::Char('[') => {
                self.scroll = self.scroll.saturating_sub(1);
                true
            }
            KeyCode::Char(']') => {
                self.scroll = self.scroll.saturating_add(1);
                true
            }

            // Enter/Space for interaction
            KeyCode::Enter | KeyCode::Char(' ') => self.activate_selection(),

            _ => false,
        }
    }

    /// Navigate left within current view.
    fn navigate_left(&mut self) -> bool {
        match self.current_view {
            AtlasView::PageComposition => {
                if self.page_index > 0 {
                    self.page_index -= 1;
                    // Update current page key from pages list
                    if self.page_index < self.pages_list.len() {
                        self.current_page_key = Some(self.pages_list[self.page_index].key.clone());
                    }
                    self.pending_page_load = true;
                    true
                } else {
                    false
                }
            }
            AtlasView::SpreadingActivation => {
                // Step backward in activation
                if self.activation_step > 0 {
                    self.activation_step -= 1;
                    true
                } else {
                    false
                }
            }
            AtlasView::GenerationPipeline => {
                if self.pipeline_stage > 0 {
                    self.pipeline_stage -= 1;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Navigate right within current view.
    fn navigate_right(&mut self) -> bool {
        match self.current_view {
            AtlasView::PageComposition => {
                let max_index = self.pages_list.len().saturating_sub(1);
                if self.page_index < max_index {
                    self.page_index += 1;
                    // Update current page key from pages list
                    if self.page_index < self.pages_list.len() {
                        self.current_page_key = Some(self.pages_list[self.page_index].key.clone());
                    }
                    self.pending_page_load = true;
                    true
                } else {
                    false
                }
            }
            AtlasView::SpreadingActivation => {
                self.activation_step += 1;
                true
            }
            AtlasView::GenerationPipeline => {
                if self.pipeline_stage < 5 {
                    self.pipeline_stage += 1;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Navigate up within current view.
    fn navigate_up(&mut self) -> bool {
        match self.current_view {
            AtlasView::RealmMap => {
                if self.realm_cursor > 0 {
                    self.realm_cursor -= 1;
                    true
                } else {
                    false
                }
            }
            AtlasView::ViewTraversal => {
                if self.view_cursor > 0 {
                    self.view_cursor -= 1;
                    true
                } else {
                    false
                }
            }
            _ => {
                self.scroll = self.scroll.saturating_sub(1);
                true
            }
        }
    }

    /// Navigate down within current view.
    fn navigate_down(&mut self) -> bool {
        match self.current_view {
            AtlasView::RealmMap => {
                self.realm_cursor += 1;
                true
            }
            AtlasView::ViewTraversal => {
                self.view_cursor += 1;
                true
            }
            _ => {
                self.scroll += 1;
                true
            }
        }
    }

    /// Activate current selection (Enter/Space).
    fn activate_selection(&mut self) -> bool {
        match self.current_view {
            AtlasView::RealmMap => {
                // Zoom into selected layer
                self.realm_zoomed = !self.realm_zoomed;
                true
            }
            AtlasView::SpreadingActivation => {
                // Reset activation
                self.activation_step = 0;
                true
            }
            _ => false,
        }
    }

    /// Cycle through available locales.
    fn cycle_locale(&mut self) {
        if let Some(pos) = ATLAS_LOCALES
            .iter()
            .position(|&l| l == self.selected_locale)
        {
            let next = (pos + 1) % ATLAS_LOCALES.len();
            self.selected_locale = ATLAS_LOCALES[next].to_string();
        }
    }
}
