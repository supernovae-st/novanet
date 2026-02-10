//! Atlas Mode — Interactive architecture visualizations.
//!
//! Atlas Mode provides 6 views for understanding NovaNet's architecture:
//! - `a` Spreading Activation — Cognitive science math behind context assembly
//! - `b` Knowledge Atoms — Selective loading vs monolithic blobs
//! - `c` Generation Pipeline — Block generation flow (not translation)
//! - `v` View Traversal — Debug the 12 view definitions
//! - `e` Page Composition — Complete anatomy of a Page
//! - `r` Realm Map — Bird's-eye view of 2-realm architecture

mod state;

/// Available locales for Atlas mode.
const ATLAS_LOCALES: &[&str] = &["en-US", "fr-FR", "es-ES", "de-DE", "ja-JP", "ar-SA"];

pub use state::{
    ActivationTask, AtlasState, AtlasView, BlockData, BlockGeneratedData, EntityContentData,
    EntityData, PageCompositionData, PageGeneratedData, SeoKeywordData,
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

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use proptest::prelude::*;

    /// Create a key event for testing.
    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    // =========================================================================
    // AtlasView tests
    // =========================================================================

    #[test]
    fn test_atlas_view_default() {
        let view = AtlasView::default();
        assert_eq!(view, AtlasView::RealmMap);
    }

    #[test]
    fn test_atlas_view_shortcuts() {
        assert_eq!(AtlasView::SpreadingActivation.shortcut(), 'a');
        assert_eq!(AtlasView::KnowledgeAtoms.shortcut(), 'b');
        assert_eq!(AtlasView::GenerationPipeline.shortcut(), 'c');
        assert_eq!(AtlasView::ViewTraversal.shortcut(), 'v');
        assert_eq!(AtlasView::PageComposition.shortcut(), 'e');
        assert_eq!(AtlasView::RealmMap.shortcut(), 'r');
    }

    #[test]
    fn test_atlas_view_labels() {
        assert_eq!(AtlasView::SpreadingActivation.label(), "Activation");
        assert_eq!(AtlasView::KnowledgeAtoms.label(), "Atoms");
        assert_eq!(AtlasView::GenerationPipeline.label(), "Pipeline");
        assert_eq!(AtlasView::ViewTraversal.label(), "Traversal");
        assert_eq!(AtlasView::PageComposition.label(), "Page");
        assert_eq!(AtlasView::RealmMap.label(), "Realm");
    }

    #[test]
    fn test_atlas_view_all() {
        let all = AtlasView::all();
        assert_eq!(all.len(), 6);
        assert!(all.contains(&AtlasView::SpreadingActivation));
        assert!(all.contains(&AtlasView::RealmMap));
    }

    // =========================================================================
    // AtlasState default tests
    // =========================================================================

    #[test]
    fn test_atlas_state_default() {
        let state = AtlasState::default();

        assert_eq!(state.current_view, AtlasView::RealmMap);
        assert!(!state.demo_mode);
        assert_eq!(state.selected_locale, "en-US");
        assert_eq!(state.scroll, 0);
        assert_eq!(state.activation_step, 0);
        assert_eq!(state.pipeline_stage, 0);
        assert_eq!(state.realm_cursor, 0);
        assert!(!state.realm_zoomed);
    }

    // =========================================================================
    // View switching tests (handle_key)
    // =========================================================================

    #[test]
    fn test_handle_key_view_switching() {
        let mut state = AtlasState::default();

        // Switch to each view
        assert!(state.handle_key(key(KeyCode::Char('a'))));
        assert_eq!(state.current_view, AtlasView::SpreadingActivation);

        assert!(state.handle_key(key(KeyCode::Char('b'))));
        assert_eq!(state.current_view, AtlasView::KnowledgeAtoms);

        assert!(state.handle_key(key(KeyCode::Char('c'))));
        assert_eq!(state.current_view, AtlasView::GenerationPipeline);

        assert!(state.handle_key(key(KeyCode::Char('v'))));
        assert_eq!(state.current_view, AtlasView::ViewTraversal);

        assert!(state.handle_key(key(KeyCode::Char('e'))));
        assert_eq!(state.current_view, AtlasView::PageComposition);

        assert!(state.handle_key(key(KeyCode::Char('r'))));
        assert_eq!(state.current_view, AtlasView::RealmMap);
    }

    #[test]
    fn test_handle_key_demo_toggle() {
        let mut state = AtlasState::default();
        assert!(!state.demo_mode);

        assert!(state.handle_key(key(KeyCode::Char('d'))));
        assert!(state.demo_mode);

        assert!(state.handle_key(key(KeyCode::Char('d'))));
        assert!(!state.demo_mode);
    }

    #[test]
    fn test_handle_key_unknown_returns_false() {
        let mut state = AtlasState::default();

        assert!(!state.handle_key(key(KeyCode::Char('z'))));
        assert!(!state.handle_key(key(KeyCode::F(1))));
        assert!(!state.handle_key(key(KeyCode::Tab)));
    }

    // =========================================================================
    // Locale cycling tests
    // =========================================================================

    #[test]
    fn test_cycle_locale() {
        let mut state = AtlasState::default();
        assert_eq!(state.selected_locale, "en-US");

        assert!(state.handle_key(key(KeyCode::Char('l'))));
        assert_eq!(state.selected_locale, "fr-FR");

        assert!(state.handle_key(key(KeyCode::Char('l'))));
        assert_eq!(state.selected_locale, "es-ES");

        assert!(state.handle_key(key(KeyCode::Char('l'))));
        assert_eq!(state.selected_locale, "de-DE");

        assert!(state.handle_key(key(KeyCode::Char('l'))));
        assert_eq!(state.selected_locale, "ja-JP");

        assert!(state.handle_key(key(KeyCode::Char('l'))));
        assert_eq!(state.selected_locale, "ar-SA");

        // Should wrap around to en-US
        assert!(state.handle_key(key(KeyCode::Char('l'))));
        assert_eq!(state.selected_locale, "en-US");
    }

    #[test]
    fn test_cycle_locale_all_atlas_locales() {
        // Verify ATLAS_LOCALES has expected values
        assert_eq!(ATLAS_LOCALES.len(), 6);
        assert_eq!(ATLAS_LOCALES[0], "en-US");
        assert_eq!(ATLAS_LOCALES[5], "ar-SA");
    }

    // =========================================================================
    // ActivationTask tests
    // =========================================================================

    #[test]
    fn test_activation_task_default() {
        let task = ActivationTask::default();
        assert_eq!(task, ActivationTask::CTA);
    }

    #[test]
    fn test_activation_task_next_cycles() {
        let mut task = ActivationTask::CTA;

        task = task.next();
        assert_eq!(task, ActivationTask::FAQ);

        task = task.next();
        assert_eq!(task, ActivationTask::Hero);

        task = task.next();
        assert_eq!(task, ActivationTask::Pricing);

        task = task.next();
        assert_eq!(task, ActivationTask::Features);

        task = task.next();
        assert_eq!(task, ActivationTask::CTA); // Wraps around
    }

    #[test]
    fn test_activation_task_labels() {
        assert_eq!(ActivationTask::CTA.label(), "CTA");
        assert_eq!(ActivationTask::FAQ.label(), "FAQ");
        assert_eq!(ActivationTask::Hero.label(), "Hero");
        assert_eq!(ActivationTask::Pricing.label(), "Pricing");
        assert_eq!(ActivationTask::Features.label(), "Features");
    }

    #[test]
    fn test_activation_task_boost_values() {
        // Task-specific boosts
        assert_eq!(ActivationTask::CTA.boost("urgency"), 1.3);
        assert_eq!(ActivationTask::FAQ.boost("definition"), 1.3);
        assert_eq!(ActivationTask::Hero.boost("benefit"), 1.2);
        assert_eq!(ActivationTask::Pricing.boost("value"), 1.2);
        assert_eq!(ActivationTask::Features.boost("capability"), 1.2);

        // Default boost for non-matching concepts
        assert_eq!(ActivationTask::CTA.boost("unrelated"), 1.0);
        assert_eq!(ActivationTask::FAQ.boost("urgency"), 1.0);
    }

    #[test]
    fn test_task_cycling_via_handle_key() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::SpreadingActivation;

        assert!(state.handle_key(key(KeyCode::Char('t'))));
        assert_eq!(state.activation_task, ActivationTask::FAQ);

        assert!(state.handle_key(key(KeyCode::Char('t'))));
        assert_eq!(state.activation_task, ActivationTask::Hero);
    }

    #[test]
    fn test_task_cycling_only_in_spreading_activation() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::RealmMap;

        // Should return false and not change task
        assert!(!state.handle_key(key(KeyCode::Char('t'))));
        assert_eq!(state.activation_task, ActivationTask::CTA);
    }

    // =========================================================================
    // Navigation tests - Realm Map
    // =========================================================================

    #[test]
    fn test_realm_map_navigate_down() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::RealmMap;
        state.realm_cursor = 0;

        assert!(state.handle_key(key(KeyCode::Down)));
        assert_eq!(state.realm_cursor, 1);

        assert!(state.handle_key(key(KeyCode::Char('j'))));
        assert_eq!(state.realm_cursor, 2);
    }

    #[test]
    fn test_realm_map_navigate_up_boundary() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::RealmMap;
        state.realm_cursor = 0;

        // Should return false at boundary
        assert!(!state.handle_key(key(KeyCode::Up)));
        assert_eq!(state.realm_cursor, 0);
    }

    #[test]
    fn test_realm_map_navigate_up() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::RealmMap;
        state.realm_cursor = 3;

        assert!(state.handle_key(key(KeyCode::Up)));
        assert_eq!(state.realm_cursor, 2);

        assert!(state.handle_key(key(KeyCode::Char('k'))));
        assert_eq!(state.realm_cursor, 1);
    }

    // =========================================================================
    // Navigation tests - Generation Pipeline
    // =========================================================================

    #[test]
    fn test_pipeline_navigate_right() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::GenerationPipeline;
        state.pipeline_stage = 0;

        assert!(state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.pipeline_stage, 1);

        assert!(state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.pipeline_stage, 2);
    }

    #[test]
    fn test_pipeline_navigate_right_boundary() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::GenerationPipeline;
        state.pipeline_stage = 5;

        // Should return false at max stage (5)
        assert!(!state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.pipeline_stage, 5);
    }

    #[test]
    fn test_pipeline_navigate_left() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::GenerationPipeline;
        state.pipeline_stage = 3;

        assert!(state.handle_key(key(KeyCode::Left)));
        assert_eq!(state.pipeline_stage, 2);

        assert!(state.handle_key(key(KeyCode::Char('h'))));
        assert_eq!(state.pipeline_stage, 1);
    }

    #[test]
    fn test_pipeline_navigate_left_boundary() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::GenerationPipeline;
        state.pipeline_stage = 0;

        // Should return false at boundary
        assert!(!state.handle_key(key(KeyCode::Left)));
        assert_eq!(state.pipeline_stage, 0);
    }

    // =========================================================================
    // Navigation tests - Spreading Activation
    // =========================================================================

    #[test]
    fn test_spreading_activation_navigate_right() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::SpreadingActivation;
        state.activation_step = 0;

        // No upper bound for activation_step
        assert!(state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.activation_step, 1);

        assert!(state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.activation_step, 2);

        // Can keep going
        assert!(state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.activation_step, 3);
    }

    #[test]
    fn test_spreading_activation_navigate_left() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::SpreadingActivation;
        state.activation_step = 3;

        assert!(state.handle_key(key(KeyCode::Left)));
        assert_eq!(state.activation_step, 2);

        assert!(state.handle_key(key(KeyCode::Char('h'))));
        assert_eq!(state.activation_step, 1);
    }

    #[test]
    fn test_spreading_activation_navigate_left_boundary() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::SpreadingActivation;
        state.activation_step = 0;

        assert!(!state.handle_key(key(KeyCode::Left)));
        assert_eq!(state.activation_step, 0);
    }

    // =========================================================================
    // Navigation tests - View Traversal
    // =========================================================================

    #[test]
    fn test_view_traversal_navigate() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::ViewTraversal;
        state.view_cursor = 0;

        assert!(state.handle_key(key(KeyCode::Down)));
        assert_eq!(state.view_cursor, 1);

        // Navigate up
        assert!(state.handle_key(key(KeyCode::Up)));
        assert_eq!(state.view_cursor, 0);

        // At boundary
        assert!(!state.handle_key(key(KeyCode::Up)));
        assert_eq!(state.view_cursor, 0);
    }

    // =========================================================================
    // Navigation tests - Page Composition
    // =========================================================================

    #[test]
    fn test_page_composition_navigate_with_empty_list() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::PageComposition;
        // pages_list is empty by default

        // Should return false when no pages
        assert!(!state.handle_key(key(KeyCode::Right)));
        assert!(!state.handle_key(key(KeyCode::Left)));
    }

    #[test]
    fn test_page_composition_navigate_with_pages() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::PageComposition;
        state.pages_list = vec![
            crate::tui::data::AtlasPageInfo {
                key: "page-1".to_string(),
                display_name: "Page 1".to_string(),
                project_key: "proj".to_string(),
                project_name: "Project".to_string(),
            },
            crate::tui::data::AtlasPageInfo {
                key: "page-2".to_string(),
                display_name: "Page 2".to_string(),
                project_key: "proj".to_string(),
                project_name: "Project".to_string(),
            },
            crate::tui::data::AtlasPageInfo {
                key: "page-3".to_string(),
                display_name: "Page 3".to_string(),
                project_key: "proj".to_string(),
                project_name: "Project".to_string(),
            },
        ];
        state.page_index = 0;

        // Navigate right
        assert!(state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.page_index, 1);
        assert_eq!(state.current_page_key, Some("page-2".to_string()));
        assert!(state.pending_page_load);

        // Navigate right again
        state.pending_page_load = false;
        assert!(state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.page_index, 2);
        assert_eq!(state.current_page_key, Some("page-3".to_string()));

        // At max, should return false
        state.pending_page_load = false;
        assert!(!state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.page_index, 2);

        // Navigate left
        assert!(state.handle_key(key(KeyCode::Left)));
        assert_eq!(state.page_index, 1);

        // Navigate to beginning
        state.page_index = 0;
        assert!(!state.handle_key(key(KeyCode::Left)));
        assert_eq!(state.page_index, 0);
    }

    // =========================================================================
    // Scroll tests
    // =========================================================================

    #[test]
    fn test_scroll_controls() {
        let mut state = AtlasState::default();
        state.scroll = 5;

        assert!(state.handle_key(key(KeyCode::Char(']'))));
        assert_eq!(state.scroll, 6);

        assert!(state.handle_key(key(KeyCode::Char('['))));
        assert_eq!(state.scroll, 5);
    }

    #[test]
    fn test_scroll_boundary_at_zero() {
        let mut state = AtlasState::default();
        state.scroll = 0;

        // Should not underflow
        assert!(state.handle_key(key(KeyCode::Char('['))));
        assert_eq!(state.scroll, 0);
    }

    // =========================================================================
    // Activate selection tests
    // =========================================================================

    #[test]
    fn test_activate_realm_map_toggle_zoom() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::RealmMap;
        assert!(!state.realm_zoomed);

        assert!(state.handle_key(key(KeyCode::Enter)));
        assert!(state.realm_zoomed);

        assert!(state.handle_key(key(KeyCode::Char(' '))));
        assert!(!state.realm_zoomed);
    }

    #[test]
    fn test_activate_spreading_activation_reset() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::SpreadingActivation;
        state.activation_step = 5;

        assert!(state.handle_key(key(KeyCode::Enter)));
        assert_eq!(state.activation_step, 0);
    }

    #[test]
    fn test_activate_other_views_returns_false() {
        let mut state = AtlasState::default();

        state.current_view = AtlasView::KnowledgeAtoms;
        assert!(!state.handle_key(key(KeyCode::Enter)));

        state.current_view = AtlasView::GenerationPipeline;
        assert!(!state.handle_key(key(KeyCode::Enter)));

        state.current_view = AtlasView::ViewTraversal;
        assert!(!state.handle_key(key(KeyCode::Enter)));

        state.current_view = AtlasView::PageComposition;
        assert!(!state.handle_key(key(KeyCode::Enter)));
    }

    // =========================================================================
    // Spreading Activation Formula Tests
    // =========================================================================

    /// Test the activation decay formula: A(t) = A_0 * e^(-lambda * t)
    /// where A_0 = 1.0 and lambda = 0.3
    #[test]
    fn test_activation_formula_values() {
        let decay = 0.3_f32;
        let calc_activation = |hop: usize| -> f32 { 1.0_f32 * (-decay * hop as f32).exp() };

        // A_0 = 1.0
        let a0 = calc_activation(0);
        assert!((a0 - 1.0).abs() < 0.001, "A_0 should be 1.0, got {}", a0);

        // A_1 = e^(-0.3) = 0.7408
        let a1 = calc_activation(1);
        assert!(
            (a1 - 0.7408).abs() < 0.001,
            "A_1 should be ~0.741, got {}",
            a1
        );

        // A_2 = e^(-0.6) = 0.5488
        let a2 = calc_activation(2);
        assert!(
            (a2 - 0.5488).abs() < 0.001,
            "A_2 should be ~0.549, got {}",
            a2
        );

        // A_3 = e^(-0.9) = 0.4066
        let a3 = calc_activation(3);
        assert!(
            (a3 - 0.4066).abs() < 0.001,
            "A_3 should be ~0.407, got {}",
            a3
        );
    }

    #[test]
    fn test_activation_monotonic_decrease() {
        let decay = 0.3_f32;
        let calc_activation = |hop: usize| -> f32 { 1.0_f32 * (-decay * hop as f32).exp() };

        // Activation should decrease monotonically
        let mut prev = calc_activation(0);
        for hop in 1..=10 {
            let curr = calc_activation(hop);
            assert!(
                curr < prev,
                "Activation should decrease: A_{} ({}) < A_{} ({})",
                hop,
                curr,
                hop - 1,
                prev
            );
            prev = curr;
        }
    }

    #[test]
    fn test_activation_always_positive() {
        let decay = 0.3_f32;
        let calc_activation = |hop: usize| -> f32 { 1.0_f32 * (-decay * hop as f32).exp() };

        // Activation should always be positive (exponential decay asymptotes to 0)
        for hop in 0..=100 {
            let a = calc_activation(hop);
            assert!(
                a > 0.0,
                "Activation at hop {} should be positive, got {}",
                hop,
                a
            );
        }
    }

    // =========================================================================
    // Property-based tests
    // =========================================================================

    proptest! {
        /// Property: realm_cursor can only increase via navigate_down.
        #[test]
        fn test_realm_cursor_increases_on_down(initial in 0usize..1000) {
            let mut state = AtlasState::default();
            state.current_view = AtlasView::RealmMap;
            state.realm_cursor = initial;

            state.handle_key(key(KeyCode::Down));

            prop_assert_eq!(state.realm_cursor, initial + 1);
        }

        /// Property: realm_cursor saturates at 0 on navigate_up.
        #[test]
        fn test_realm_cursor_saturates_at_zero(initial in 0usize..100) {
            let mut state = AtlasState::default();
            state.current_view = AtlasView::RealmMap;
            state.realm_cursor = initial;

            // Navigate up `initial + 1` times to ensure we hit the boundary
            for _ in 0..=initial {
                state.handle_key(key(KeyCode::Up));
            }

            prop_assert_eq!(state.realm_cursor, 0);
        }

        /// Property: pipeline_stage stays in valid range [0, 5].
        #[test]
        fn test_pipeline_stage_bounds(initial in 0usize..=5, right_presses in 0usize..20) {
            let mut state = AtlasState::default();
            state.current_view = AtlasView::GenerationPipeline;
            state.pipeline_stage = initial;

            for _ in 0..right_presses {
                state.handle_key(key(KeyCode::Right));
            }

            prop_assert!(state.pipeline_stage <= 5, "pipeline_stage exceeded max");
        }

        /// Property: pipeline_stage >= 0 after any number of left presses.
        #[test]
        fn test_pipeline_stage_min_bound(initial in 0usize..=5, left_presses in 0usize..20) {
            let mut state = AtlasState::default();
            state.current_view = AtlasView::GenerationPipeline;
            state.pipeline_stage = initial;

            for _ in 0..left_presses {
                state.handle_key(key(KeyCode::Left));
            }

            prop_assert!(state.pipeline_stage <= 5, "pipeline_stage should stay valid");
        }

        /// Property: page_index stays within bounds of pages_list.
        #[test]
        fn test_page_index_bounds(page_count in 1usize..20, nav_count in 0usize..50) {
            let mut state = AtlasState::default();
            state.current_view = AtlasView::PageComposition;

            // Create mock pages list
            state.pages_list = (0..page_count)
                .map(|i| crate::tui::data::AtlasPageInfo {
                    key: format!("page-{}", i),
                    display_name: format!("Page {}", i),
                    project_key: "proj".to_string(),
                    project_name: "Project".to_string(),
                })
                .collect();
            state.page_index = 0;

            // Randomly navigate left and right
            for i in 0..nav_count {
                if i % 2 == 0 {
                    state.handle_key(key(KeyCode::Right));
                } else {
                    state.handle_key(key(KeyCode::Left));
                }
            }

            let max_valid = page_count.saturating_sub(1);
            prop_assert!(
                state.page_index <= max_valid,
                "page_index {} exceeded max {} for {} pages",
                state.page_index,
                max_valid,
                page_count
            );
        }

        /// Property: scroll never underflows on repeated [ presses.
        #[test]
        fn test_scroll_no_underflow(initial in 0usize..100, presses in 0usize..200) {
            let mut state = AtlasState::default();
            state.scroll = initial;

            for _ in 0..presses {
                state.handle_key(key(KeyCode::Char('[')));
            }

            // scroll uses saturating_sub, so should never panic or underflow
            prop_assert!(state.scroll <= initial);
        }

        /// Property: activation formula is non-negative for any hop count.
        /// Note: For very large hop values (> ~350), f32 precision causes underflow to 0.
        #[test]
        fn test_activation_non_negative_for_any_hop(hop in 0usize..1000) {
            let decay = 0.3_f32;
            let activation = 1.0_f32 * (-decay * hop as f32).exp();
            prop_assert!(activation >= 0.0, "Activation should be non-negative for hop {}", hop);
        }
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_locale_unknown_position() {
        let mut state = AtlasState::default();
        // Set to a locale not in ATLAS_LOCALES
        state.selected_locale = "zh-CN".to_string();

        // cycle_locale should do nothing if locale not found
        state.cycle_locale();
        assert_eq!(state.selected_locale, "zh-CN");
    }

    #[test]
    fn test_view_cursor_no_upper_bound_check() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::ViewTraversal;
        state.view_cursor = 100;

        // Navigate down should still work (no upper bound in code)
        assert!(state.handle_key(key(KeyCode::Down)));
        assert_eq!(state.view_cursor, 101);
    }

    #[test]
    fn test_activation_step_high_values() {
        let mut state = AtlasState::default();
        state.current_view = AtlasView::SpreadingActivation;
        state.activation_step = 1000;

        // Should still work at high values
        assert!(state.handle_key(key(KeyCode::Right)));
        assert_eq!(state.activation_step, 1001);
    }
}
