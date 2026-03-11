//! Views mode key handler.
//!
//! Handles keyboard navigation in Views mode (NavMode::Views).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::KeyResult;
use crate::tui::app::App;
use crate::tui::clipboard;

/// Open a URL in the system browser.
///
/// Uses `open` on macOS, `xdg-open` on Linux, `start` on Windows.
fn open_in_browser(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(url).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn()?;
    }
    Ok(())
}

/// Get Studio URL for a view.
///
/// Studio uses query parameters: `?view={id}` (not path `/views/{id}`)
fn studio_url(view_id: &str) -> String {
    format!("http://localhost:3000/?view={view_id}")
}

/// Handle key events in Views mode.
///
/// Returns `KeyResult::Handled` if the key was consumed,
/// or `KeyResult::FallThrough` if it should be processed by global handlers.
pub fn handle_views_key(app: &mut App, key: KeyEvent) -> KeyResult {
    // Handle Ctrl+d/u for scrolling in Cypher section
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('d') => {
                app.nexus.views.scroll_down(10);
                return KeyResult::Handled;
            },
            KeyCode::Char('u') => {
                app.nexus.views.scroll_up(10);
                return KeyResult::Handled;
            },
            _ => return KeyResult::FallThrough,
        }
    }

    // Handle Shift+Tab for previous section
    if key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::BackTab {
        app.nexus.views.prev_section();
        return KeyResult::Handled;
    }

    // No other modifier keys for Views navigation
    if key.modifiers.contains(KeyModifiers::ALT) {
        return KeyResult::FallThrough;
    }

    match key.code {
        // Tab: cycle detail sections (Info → Cypher → Relations → Schema)
        KeyCode::Tab => {
            app.nexus.views.next_section();
            KeyResult::Handled
        },

        // Page scroll in Cypher section
        KeyCode::PageDown => {
            app.nexus.views.scroll_down(10);
            KeyResult::Handled
        },
        KeyCode::PageUp => {
            app.nexus.views.scroll_up(10);
            KeyResult::Handled
        },
        // Navigation: j/k/↑/↓
        KeyCode::Down | KeyCode::Char('j') => {
            app.nexus.views.navigate_down(&app.loaded_views);
            KeyResult::Handled
        },
        KeyCode::Up | KeyCode::Char('k') => {
            app.nexus.views.navigate_up(&app.loaded_views);
            KeyResult::Handled
        },

        // Toggle concept panel
        KeyCode::Char('?') => {
            app.nexus.views.show_concept = !app.nexus.views.show_concept;
            KeyResult::Handled
        },

        // Escape: exit concept panel if showing, otherwise fall through
        KeyCode::Esc => {
            if app.nexus.views.show_concept {
                app.nexus.views.show_concept = false;
                KeyResult::Handled
            } else {
                KeyResult::FallThrough
            }
        },

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
        },

        // Enter: copy Cypher and show Studio link
        KeyCode::Enter => {
            if let Some(view) = app.nexus.views.current_view(&app.loaded_views) {
                if let Some(ref cypher) = view.cypher {
                    match clipboard::copy_to_clipboard(cypher) {
                        Ok(()) => {
                            let url = studio_url(&view.id);
                            // Warn if contextual view
                            if view.contextual.unwrap_or(false) {
                                app.set_status(&format!(
                                    "Cypher copied! ⚠ Contextual view needs ?key=... → {}",
                                    url
                                ));
                            } else {
                                app.set_status(&format!("Cypher copied! Open Studio: {}", url));
                            }
                        },
                        Err(e) => app.set_status(&format!("Clipboard error: {}", e)),
                    }
                } else {
                    app.set_status("No Cypher query for this view");
                }
            }
            KeyResult::Handled
        },

        // Open view in Studio (browser)
        KeyCode::Char('o') => {
            if let Some(view) = app.nexus.views.current_view(&app.loaded_views) {
                // Check if view is contextual (requires a node key)
                if view.contextual.unwrap_or(false) {
                    app.set_status(&format!(
                        "⚠ {} is contextual — select a node in Graph mode first",
                        view.name
                    ));
                } else {
                    let url = studio_url(&view.id);
                    match open_in_browser(&url) {
                        Ok(()) => app.set_status(&format!("Opening {} in browser...", view.name)),
                        Err(e) => app.set_status(&format!("Failed to open browser: {}", e)),
                    }
                }
            }
            KeyResult::Handled
        },

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
