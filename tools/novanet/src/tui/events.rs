//! Keyboard event handling for the TUI.
//!
//! Maps crossterm key events to app actions. The event loop lives in
//! `runtime.rs`; this module handles the dispatch logic.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{ActivePanel, AppState, NavMode};

/// Result of handling a key event.
pub enum Action {
    /// No state change needed.
    None,
    /// App state changed, re-render.
    Render,
    /// Quit the TUI.
    Quit,
    /// Trigger an async fetch for the current mode.
    Fetch,
}

/// Handle a key event against the current app state.
pub fn handle_key(state: &mut AppState, key: KeyEvent) -> Action {
    // Ctrl+C or 'q' always quits
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Action::Quit;
    }

    match state {
        AppState::Loading { .. } => {
            if key.code == KeyCode::Char('q') {
                Action::Quit
            } else {
                Action::None
            }
        }
        AppState::Ready {
            mode,
            tree,
            active_panel,
            detail_lines,
            status,
            facets,
            node_count: _,
        } => match key.code {
            KeyCode::Char('q') => Action::Quit,

            // Mode switching: 1-4 or Tab to cycle
            KeyCode::Char(c @ '1'..='4') => {
                if let Some(new_mode) = NavMode::from_key(c) {
                    *mode = new_mode;
                    *status = format!("Mode: {}", new_mode.label());
                    Action::Fetch
                } else {
                    Action::None
                }
            }
            KeyCode::Tab => {
                *mode = mode.cycle();
                *status = format!("Mode: {}", mode.label());
                Action::Fetch
            }

            // Panel switching
            KeyCode::Left => {
                *active_panel = ActivePanel::Tree;
                Action::Render
            }
            KeyCode::Right => {
                *active_panel = ActivePanel::Detail;
                Action::Render
            }

            // Tree navigation (when tree panel is active)
            KeyCode::Up if *active_panel == ActivePanel::Tree => {
                tree.cursor_up();
                update_detail(tree, detail_lines);
                Action::Render
            }
            KeyCode::Down if *active_panel == ActivePanel::Tree => {
                tree.cursor_down();
                update_detail(tree, detail_lines);
                Action::Render
            }
            KeyCode::Enter if *active_panel == ActivePanel::Tree => {
                tree.toggle_expand();
                Action::Render
            }
            KeyCode::Char(' ') if *active_panel == ActivePanel::Tree => {
                tree.toggle_expand();
                Action::Render
            }

            // Facet filter popup (Query mode)
            KeyCode::Char('f') if *mode == NavMode::Query => {
                facets.show_popup = !facets.show_popup;
                if facets.show_popup {
                    *status = "Filter popup (Esc to close)".to_string();
                } else {
                    *status = format!("Mode: {}", mode.label());
                }
                Action::Render
            }
            KeyCode::Esc if facets.show_popup => {
                facets.show_popup = false;
                *status = format!("Mode: {}", mode.label());
                Action::Render
            }

            // Help
            KeyCode::Char('?') => {
                *detail_lines = help_text();
                Action::Render
            }

            _ => Action::None,
        },
    }
}

fn update_detail(tree: &crate::tui::tree::TaxonomyTree, detail_lines: &mut Vec<String>) {
    if let Some(node) = tree.selected() {
        *detail_lines = vec![
            format!("Key: {}", node.key),
            format!("Name: {}", node.display_name),
            format!("Type: {:?}", node.node_type),
            format!(
                "Children: {}",
                if node.children.is_empty() {
                    "none".to_string()
                } else {
                    format!("{}", node.children.len())
                }
            ),
        ];
    } else {
        *detail_lines = vec!["No selection".to_string()];
    }
}

fn help_text() -> Vec<String> {
    vec![
        "NovaNet TUI — Keyboard Shortcuts".to_string(),
        "".to_string(),
        "Navigation:".to_string(),
        "  1-4     Switch mode (Data/Meta/Overlay/Query)".to_string(),
        "  Tab     Cycle mode".to_string(),
        "  ←/→     Switch panel (Tree/Detail)".to_string(),
        "  ↑/↓     Navigate tree".to_string(),
        "  Enter   Expand/collapse node".to_string(),
        "  Space   Expand/collapse node".to_string(),
        "".to_string(),
        "Filters:".to_string(),
        "  f       Toggle facet filter (Query mode)".to_string(),
        "  Esc     Close popup".to_string(),
        "".to_string(),
        "Other:".to_string(),
        "  ?       Show this help".to_string(),
        "  q       Quit".to_string(),
        "  Ctrl+C  Quit".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::tree::{MetaRow, TaxonomyTree};

    fn make_ready_state() -> AppState {
        let rows = vec![
            MetaRow {
                label: "Realm".to_string(),
                key: "global".to_string(),
                display_name: "Global".to_string(),
                parent_key: None,
            },
            MetaRow {
                label: "Layer".to_string(),
                key: "knowledge".to_string(),
                display_name: "Knowledge".to_string(),
                parent_key: Some("global".to_string()),
            },
        ];
        let tree = TaxonomyTree::from_meta_rows(&rows);
        AppState::ready(tree)
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn quit_on_q() {
        let mut state = make_ready_state();
        assert!(matches!(
            handle_key(&mut state, key(KeyCode::Char('q'))),
            Action::Quit
        ));
    }

    #[test]
    fn mode_switch_on_number() {
        let mut state = make_ready_state();
        let action = handle_key(&mut state, key(KeyCode::Char('2')));
        assert!(matches!(action, Action::Fetch));
        if let AppState::Ready { mode, .. } = &state {
            assert_eq!(*mode, NavMode::Meta);
        }
    }

    #[test]
    fn tab_cycles_mode() {
        let mut state = make_ready_state();
        handle_key(&mut state, key(KeyCode::Tab));
        if let AppState::Ready { mode, .. } = &state {
            assert_eq!(*mode, NavMode::Overlay); // Meta -> Overlay
        }
    }

    #[test]
    fn arrow_keys_navigate_tree() {
        let mut state = make_ready_state();
        handle_key(&mut state, key(KeyCode::Down));
        if let AppState::Ready { tree, .. } = &state {
            assert_eq!(tree.cursor, 1);
        }
    }

    #[test]
    fn help_shows_shortcuts() {
        let mut state = make_ready_state();
        handle_key(&mut state, key(KeyCode::Char('?')));
        if let AppState::Ready { detail_lines, .. } = &state {
            assert!(detail_lines[0].contains("Keyboard Shortcuts"));
        }
    }

    #[test]
    fn facet_popup_toggle_in_query_mode() {
        let mut state = make_ready_state();
        // Switch to query mode first
        handle_key(&mut state, key(KeyCode::Char('4')));
        // Toggle facet popup
        let action = handle_key(&mut state, key(KeyCode::Char('f')));
        assert!(matches!(action, Action::Render));
        if let AppState::Ready { facets, .. } = &state {
            assert!(facets.show_popup);
        }
    }

    #[test]
    fn loading_state_only_quit() {
        let mut state = AppState::loading("Connecting...");
        assert!(matches!(
            handle_key(&mut state, key(KeyCode::Char('q'))),
            Action::Quit
        ));
        assert!(matches!(
            handle_key(&mut state, key(KeyCode::Down)),
            Action::None
        ));
    }
}
