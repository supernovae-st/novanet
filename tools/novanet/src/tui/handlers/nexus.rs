//! Nexus mode key handler.
//!
//! Handles keys in the Nexus mode (gamified learning hub).
//! Most keys are delegated to the NexusState for tab navigation.
//!
//! # Keys
//!
//! - `?`: Open help overlay (handled here)
//! - `/` or `f`: Open search (handled here)
//! - `F1`: Open legend (handled here)
//! - `1-5`: Fall through to global mode switching
//! - All other keys: Delegated to NexusState (tab switching, navigation)

use crossterm::event::{KeyCode, KeyEvent};

use super::KeyResult;
use crate::tui::app::{App, NavMode};
use crate::tui::data::get_all_adrs;
use crate::tui::nexus::NexusTab;

/// Handle key events in Nexus mode.
///
/// Returns `KeyResult::Handled` if the key was consumed,
/// `KeyResult::FallThrough` if it should be processed by global handlers.
pub fn handle_nexus_key(app: &mut App, key: KeyEvent) -> KeyResult {
    match key.code {
        // Help overlay
        KeyCode::Char('?') => {
            app.help_active = true;
            KeyResult::Handled
        }

        // Search overlay
        KeyCode::Char('/') | KeyCode::Char('f') => {
            app.search.active = true;
            KeyResult::Handled
        }

        // Legend overlay
        KeyCode::F(1) => {
            app.legend_active = true;
            KeyResult::Handled
        }

        // Mode switching keys fall through to global handlers
        KeyCode::Char('1')
        | KeyCode::Char('2')
        | KeyCode::Char('3')
        | KeyCode::Char('4')
        | KeyCode::Char('5') => KeyResult::FallThrough,

        // Enter in Arch tab: jump to first related class in Graph mode
        KeyCode::Enter if app.nexus.tab == NexusTab::Arch => {
            let adrs = get_all_adrs();
            if let Some(adr) = adrs.get(app.nexus.arch_adr_index) {
                if let Some(class_key) = adr.related_classes.first() {
                    // Switch to Graph mode and navigate to the class
                    app.save_mode_cursor();
                    app.mode = NavMode::Graph;
                    if app.navigate_to_class(class_key) {
                        app.set_status(&format!("Jumped to {} ({})", class_key, adr.id));
                    } else {
                        app.set_status(&format!("Class {} not found", class_key));
                    }
                    return KeyResult::Handled;
                } else {
                    app.set_status(&format!("{}: no related classes", adr.id));
                    return KeyResult::Handled;
                }
            }
            KeyResult::Handled
        }

        // All other keys handled by NexusState ([ ] for tabs, j/k for nav, etc.)
        _ => {
            if app.nexus.handle_key(key) {
                KeyResult::Handled
            } else {
                KeyResult::FallThrough
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::data::TaxonomyTree;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::empty())
    }

    fn test_app() -> App {
        App::new(TaxonomyTree::default(), "/test".to_string())
    }

    #[test]
    fn test_nexus_help_key() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Nexus;
        app.help_active = false;

        let result = handle_nexus_key(&mut app, key(KeyCode::Char('?')));

        assert_eq!(result, KeyResult::Handled);
        assert!(app.help_active);
    }

    #[test]
    fn test_nexus_search_key() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Nexus;
        app.search.active = false;

        let result = handle_nexus_key(&mut app, key(KeyCode::Char('/')));

        assert_eq!(result, KeyResult::Handled);
        assert!(app.search.active);
    }

    #[test]
    fn test_nexus_legend_key() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Nexus;
        app.legend_active = false;

        let result = handle_nexus_key(&mut app, key(KeyCode::F(1)));

        assert_eq!(result, KeyResult::Handled);
        assert!(app.legend_active);
    }

    #[test]
    fn test_nexus_mode_switch_falls_through() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Nexus;

        assert_eq!(
            handle_nexus_key(&mut app, key(KeyCode::Char('1'))),
            KeyResult::FallThrough
        );
        assert_eq!(
            handle_nexus_key(&mut app, key(KeyCode::Char('5'))),
            KeyResult::FallThrough
        );
    }

    #[test]
    fn test_arch_tab_enter_with_related_class() {
        let mut app = test_app();
        app.mode = NavMode::Nexus;
        app.nexus.tab = NexusTab::Arch;
        // ADR-028 has related_classes: ["Page", "Entity", "Block"]
        // Find its index in the ADR list
        let adrs = get_all_adrs();
        let adr_028_idx = adrs.iter().position(|adr| adr.id == "ADR-028").unwrap();
        app.nexus.arch_adr_index = adr_028_idx;

        let result = handle_nexus_key(&mut app, key(KeyCode::Enter));

        assert_eq!(result, KeyResult::Handled);
        // Should switch to Graph mode
        assert_eq!(app.mode, NavMode::Graph);
    }

    #[test]
    fn test_arch_tab_enter_with_no_related_class() {
        let mut app = test_app();
        app.mode = NavMode::Nexus;
        app.nexus.tab = NexusTab::Arch;
        // ADR-001 has no related classes
        let adrs = get_all_adrs();
        let adr_001_idx = adrs.iter().position(|adr| adr.id == "ADR-001").unwrap();
        app.nexus.arch_adr_index = adr_001_idx;

        let result = handle_nexus_key(&mut app, key(KeyCode::Enter));

        assert_eq!(result, KeyResult::Handled);
        // Should stay in Nexus mode (no class to jump to)
        assert_eq!(app.mode, NavMode::Nexus);
    }
}
