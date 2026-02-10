//! Audit mode key handler.
//!
//! Handles navigation in the Audit mode (data quality overview).
//!
//! # Keys
//!
//! - `j/k` or `↑↓`: Navigate audit cursor
//! - `r`: Refresh audit data
//! - `?`: Open help overlay
//! - `1-5`: Fall through to global mode switching

use crossterm::event::{KeyCode, KeyEvent};

use super::KeyResult;
use crate::tui::app::App;

/// Handle key events in Audit mode.
///
/// Returns `KeyResult::Handled` if the key was consumed,
/// `KeyResult::FallThrough` if it should be processed by global handlers.
pub fn handle_audit_key(app: &mut App, key: KeyEvent) -> KeyResult {
    match key.code {
        // Navigation in Audit mode
        KeyCode::Up | KeyCode::Char('k') => {
            if app.audit_cursor > 0 {
                app.audit_cursor -= 1;
            }
            KeyResult::Handled
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(stats) = &app.audit_stats {
                let max = stats.kinds.len().saturating_sub(1);
                if app.audit_cursor < max {
                    app.audit_cursor += 1;
                }
            }
            KeyResult::Handled
        }

        // Refresh audit stats
        KeyCode::Char('r') => {
            app.pending_audit_load = true;
            app.audit_cursor = 0;
            app.set_status("Refreshing audit data...");
            KeyResult::Handled
        }

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

        // All other keys fall through to global handlers
        _ => KeyResult::FallThrough,
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
    fn test_audit_help_key() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Audit;
        app.help_active = false;

        let result = handle_audit_key(&mut app, key(KeyCode::Char('?')));

        assert_eq!(result, KeyResult::Handled);
        assert!(app.help_active);
    }

    #[test]
    fn test_audit_refresh_key() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Audit;
        app.pending_audit_load = false;

        let result = handle_audit_key(&mut app, key(KeyCode::Char('r')));

        assert_eq!(result, KeyResult::Handled);
        assert!(app.pending_audit_load);
    }

    #[test]
    fn test_audit_mode_switch_falls_through() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Audit;

        assert_eq!(
            handle_audit_key(&mut app, key(KeyCode::Char('1'))),
            KeyResult::FallThrough
        );
        assert_eq!(
            handle_audit_key(&mut app, key(KeyCode::Char('5'))),
            KeyResult::FallThrough
        );
    }

    #[test]
    fn test_audit_unknown_key_falls_through() {
        let mut app = test_app();
        app.mode = crate::tui::app::NavMode::Audit;

        assert_eq!(
            handle_audit_key(&mut app, key(KeyCode::Char('x'))),
            KeyResult::FallThrough
        );
    }
}
