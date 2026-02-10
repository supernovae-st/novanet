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
use crate::tui::app::App;

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
}
