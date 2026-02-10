//! Atlas mode key handler.
//!
//! Handles keys in the Atlas mode (architecture visualizations).
//! Most keys are delegated to AtlasState for view-specific handling.
//!
//! # Keys
//!
//! - `?`: Open help overlay (handled here)
//! - `1-5`: Fall through to global mode switching
//! - All other keys: Delegated to AtlasState (a-f for views, d for demo, etc.)

use crossterm::event::{KeyCode, KeyEvent};

use super::KeyResult;
use crate::tui::app::App;

/// Handle key events in Atlas mode.
///
/// Returns `KeyResult::Handled` if the key was consumed,
/// `KeyResult::FallThrough` if it should be processed by global handlers.
pub fn handle_atlas_key(app: &mut App, key: KeyEvent) -> KeyResult {
    match key.code {
        // Help overlay
        KeyCode::Char('?') => {
            app.help_active = true;
            KeyResult::Handled
        }

        // Mode switching keys fall through to global handlers
        KeyCode::Char('1')
        | KeyCode::Char('2')
        | KeyCode::Char('3')
        | KeyCode::Char('4')
        | KeyCode::Char('5') => KeyResult::FallThrough,

        // All other keys handled by AtlasState (a-f views, d demo, l locale, etc.)
        _ => {
            if app.atlas.handle_key(key) {
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
    fn test_atlas_help_key() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Atlas;
        app.help_active = false;

        let result = handle_atlas_key(&mut app, key(KeyCode::Char('?')));

        assert_eq!(result, KeyResult::Handled);
        assert!(app.help_active);
    }

    #[test]
    fn test_atlas_view_switch_delegates() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Atlas;

        // These should delegate to AtlasState which handles a-f for views
        let result = handle_atlas_key(&mut app, key(KeyCode::Char('a')));
        assert_eq!(result, KeyResult::Handled);
    }

    #[test]
    fn test_atlas_mode_switch_falls_through() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Atlas;

        assert_eq!(
            handle_atlas_key(&mut app, key(KeyCode::Char('1'))),
            KeyResult::FallThrough
        );
        assert_eq!(
            handle_atlas_key(&mut app, key(KeyCode::Char('5'))),
            KeyResult::FallThrough
        );
    }

    #[test]
    fn test_atlas_unknown_key_falls_through() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Atlas;

        // An unhandled key should fall through
        assert_eq!(
            handle_atlas_key(&mut app, key(KeyCode::Char('z'))),
            KeyResult::FallThrough
        );
    }
}
