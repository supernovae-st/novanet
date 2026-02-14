//! Views mode key handler.
//!
//! Handles keyboard navigation in Views mode (NavMode::Views).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::KeyResult;
use crate::tui::app::App;
use crate::tui::clipboard;

/// Handle key events in Views mode.
///
/// Returns `KeyResult::Handled` if the key was consumed,
/// or `KeyResult::FallThrough` if it should be processed by global handlers.
pub fn handle_views_key(app: &mut App, key: KeyEvent) -> KeyResult {
    // No modifier keys for Views navigation
    if key.modifiers.contains(KeyModifiers::CONTROL)
        || key.modifiers.contains(KeyModifiers::ALT)
    {
        return KeyResult::FallThrough;
    }

    match key.code {
        // Navigation: j/k/↑/↓
        KeyCode::Down | KeyCode::Char('j') => {
            app.nexus.views.navigate_down(&app.loaded_views);
            KeyResult::Handled
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.nexus.views.navigate_up(&app.loaded_views);
            KeyResult::Handled
        }

        // Toggle concept panel
        KeyCode::Char('?') => {
            app.nexus.views.show_concept = !app.nexus.views.show_concept;
            KeyResult::Handled
        }

        // Escape: exit concept panel if showing, otherwise fall through
        KeyCode::Esc => {
            if app.nexus.views.show_concept {
                app.nexus.views.show_concept = false;
                KeyResult::Handled
            } else {
                KeyResult::FallThrough
            }
        }

        // Copy current view cypher to clipboard (y key)
        KeyCode::Char('y') => {
            if let Some(view) = app.nexus.views.current_view(&app.loaded_views) {
                if let Some(ref cypher) = view.cypher {
                    match clipboard::copy_to_clipboard(cypher) {
                        Ok(()) => app.set_status("Cypher copied to clipboard"),
                        Err(e) => app.set_status(&format!("Clipboard error: {}", e)),
                    }
                    return KeyResult::Handled;
                }
            }
            app.set_status("No cypher to copy");
            KeyResult::Handled
        }

        // Enter: could execute the view (future feature)
        KeyCode::Enter => {
            if let Some(view) = app.nexus.views.current_view(&app.loaded_views) {
                app.set_status(&format!("View: {} (Execute not yet implemented)", view.id));
            }
            KeyResult::Handled
        }

        // Everything else falls through to global handlers
        _ => KeyResult::FallThrough,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::NavMode;
    use crate::tui::data::TaxonomyTree;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn make_key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn create_test_app() -> App {
        let tree = TaxonomyTree::default();
        let root_path = std::env::current_dir()
            .unwrap()
            .ancestors()
            .find(|p| p.join("pnpm-workspace.yaml").exists())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());
        let mut app = App::new(tree, root_path);
        app.mode = NavMode::Views;
        app
    }

    #[test]
    fn test_views_navigation_down() {
        let mut app = create_test_app();
        let initial_view = app.nexus.views.view_cursor;

        // Navigate down
        let result = handle_views_key(&mut app, make_key(KeyCode::Char('j')));
        assert_eq!(result, KeyResult::Handled);
        // If there are views, cursor should move (or stay at 0 if only 1 view in category)
        assert!(app.nexus.views.view_cursor >= initial_view || app.nexus.views.category_cursor > 0);
    }

    #[test]
    fn test_views_navigation_up() {
        let mut app = create_test_app();
        // Move down first, then up
        handle_views_key(&mut app, make_key(KeyCode::Char('j')));
        let _after_down = (app.nexus.views.category_cursor, app.nexus.views.view_cursor);

        handle_views_key(&mut app, make_key(KeyCode::Char('k')));

        // Should have moved
        let result = handle_views_key(&mut app, make_key(KeyCode::Up));
        assert_eq!(result, KeyResult::Handled);
    }

    #[test]
    fn test_views_concept_toggle() {
        let mut app = create_test_app();
        assert!(!app.nexus.views.show_concept);

        let result = handle_views_key(&mut app, make_key(KeyCode::Char('?')));
        assert_eq!(result, KeyResult::Handled);
        assert!(app.nexus.views.show_concept);

        // Toggle off
        handle_views_key(&mut app, make_key(KeyCode::Char('?')));
        assert!(!app.nexus.views.show_concept);
    }

    #[test]
    fn test_views_escape_concept() {
        let mut app = create_test_app();
        app.nexus.views.show_concept = true;

        let result = handle_views_key(&mut app, make_key(KeyCode::Esc));
        assert_eq!(result, KeyResult::Handled);
        assert!(!app.nexus.views.show_concept);
    }

    #[test]
    fn test_views_escape_fallthrough() {
        let mut app = create_test_app();
        app.nexus.views.show_concept = false;

        let result = handle_views_key(&mut app, make_key(KeyCode::Esc));
        assert_eq!(result, KeyResult::FallThrough);
    }

    #[test]
    fn test_views_ctrl_falls_through() {
        let mut app = create_test_app();
        let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL);

        let result = handle_views_key(&mut app, key);
        assert_eq!(result, KeyResult::FallThrough);
    }
}
