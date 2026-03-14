//! Key handler for Flow mode.
//!
//! Navigation:
//! - Tab: Switch between Schema/Pipeline tabs
//! - j/k or Up/Down: Scroll vertically
//! - h/l or Left/Right: Scroll horizontally
//! - Enter: Select/highlight node
//! - n/p: Next/previous selectable node

use crossterm::event::{KeyCode, KeyEvent};

use super::KeyResult;
use crate::tui::app::App;

pub fn handle_flow_key(app: &mut App, key: KeyEvent) -> KeyResult {
    match key.code {
        // Tab switching
        KeyCode::Tab => {
            app.flow.tab = app.flow.tab.toggle();
            app.flow.scroll_y = 0;
            app.flow.scroll_x = 0;
            app.flow.selected = 0;
            KeyResult::Handled
        }

        // Vertical scroll
        KeyCode::Char('j') | KeyCode::Down => {
            app.flow.scroll_y = app.flow.scroll_y.saturating_add(1);
            KeyResult::Handled
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.flow.scroll_y = app.flow.scroll_y.saturating_sub(1);
            KeyResult::Handled
        }

        // Horizontal scroll
        KeyCode::Char('l') | KeyCode::Right => {
            app.flow.scroll_x = app.flow.scroll_x.saturating_add(2);
            KeyResult::Handled
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app.flow.scroll_x = app.flow.scroll_x.saturating_sub(2);
            KeyResult::Handled
        }

        // Page scroll
        KeyCode::Char('d') => {
            app.flow.scroll_y = app.flow.scroll_y.saturating_add(10);
            KeyResult::Handled
        }
        KeyCode::Char('u') => {
            app.flow.scroll_y = app.flow.scroll_y.saturating_sub(10);
            KeyResult::Handled
        }

        // Node selection
        KeyCode::Char('n') => {
            if app.flow.total_nodes > 0 {
                app.flow.selected = (app.flow.selected + 1) % app.flow.total_nodes;
            }
            KeyResult::Handled
        }
        KeyCode::Char('p') => {
            if app.flow.total_nodes > 0 {
                app.flow.selected = if app.flow.selected == 0 {
                    app.flow.total_nodes.saturating_sub(1)
                } else {
                    app.flow.selected - 1
                };
            }
            KeyResult::Handled
        }

        // Home/End
        KeyCode::Home => {
            app.flow.scroll_y = 0;
            app.flow.scroll_x = 0;
            KeyResult::Handled
        }

        _ => KeyResult::FallThrough,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tab_switches_flow_tab() {
        // Placeholder: needs mock App (App::test_default())
    }

    #[test]
    fn test_scroll_keys() {
        // Placeholder: verify j/k/h/l change scroll positions
    }
}
