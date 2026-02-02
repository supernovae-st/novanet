//! Keyboard event handling for the TUI.
//!
//! Maps crossterm key events to app actions. The event loop lives in
//! `runtime.rs`; this module handles the dispatch logic.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{ActivePanel, AppState, NavMode};
use crate::tui::detail::{self, KindDetail};
use crate::tui::search::SearchState;
use crate::tui::tree::TreeNodeType;

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
    /// Trigger an async fetch for Kind detail (label).
    FetchDetail(String),
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
            cypher_preview,
            kind_detail,
            search,
            edge_explorer_idx,
        } => {
            // When search overlay is active, intercept all keys
            if search.is_some() {
                return handle_search_key(
                    search,
                    tree,
                    detail_lines,
                    cypher_preview,
                    kind_detail,
                    status,
                    *mode,
                    key,
                );
            }

            match key.code {
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

                // Panel switching (cycle through Tree → Detail → CypherPreview)
                KeyCode::Left => {
                    *active_panel = active_panel.cycle_prev();
                    Action::Render
                }
                KeyCode::Right => {
                    *active_panel = active_panel.cycle_next();
                    Action::Render
                }

                // Edge explorer navigation (when detail panel is active and explorer is open)
                KeyCode::Up
                    if *active_panel == ActivePanel::Detail && edge_explorer_idx.is_some() =>
                {
                    if let Some(idx) = edge_explorer_idx {
                        if *idx > 0 {
                            *idx -= 1;
                        }
                    }
                    Action::Render
                }
                KeyCode::Down
                    if *active_panel == ActivePanel::Detail && edge_explorer_idx.is_some() =>
                {
                    if let (Some(idx), Some(kd)) =
                        (edge_explorer_idx.as_mut(), kind_detail.as_ref())
                    {
                        let total = detail::edge_count(kd);
                        if *idx + 1 < total {
                            *idx += 1;
                        }
                    }
                    Action::Render
                }

                // Tree navigation (when tree panel is active)
                KeyCode::Up if *active_panel == ActivePanel::Tree => {
                    tree.cursor_up();
                    *edge_explorer_idx = None;
                    if let Some(label) =
                        update_detail(tree, detail_lines, cypher_preview, kind_detail)
                    {
                        Action::FetchDetail(label)
                    } else {
                        Action::Render
                    }
                }
                KeyCode::Down if *active_panel == ActivePanel::Tree => {
                    tree.cursor_down();
                    *edge_explorer_idx = None;
                    if let Some(label) =
                        update_detail(tree, detail_lines, cypher_preview, kind_detail)
                    {
                        Action::FetchDetail(label)
                    } else {
                        Action::Render
                    }
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

                // Edge explorer toggle
                KeyCode::Char('e') if kind_detail.is_some() => {
                    if edge_explorer_idx.is_some() {
                        // Close explorer
                        *edge_explorer_idx = None;
                        *status = format!("Mode: {}", mode.label());
                    } else {
                        // Open explorer at first edge
                        let total = detail::edge_count(kind_detail.as_ref().unwrap());
                        if total > 0 {
                            *edge_explorer_idx = Some(0);
                            *status = "Edge Explorer (e to close, Up/Down to navigate)".to_string();
                        } else {
                            *status = "No edges to explore".to_string();
                        }
                    }
                    Action::Render
                }

                // Search overlay
                KeyCode::Char('/') => {
                    *search = Some(SearchState::new());
                    *status = "Search: type to filter Kinds".to_string();
                    Action::Render
                }

                // Help
                KeyCode::Char('?') => {
                    *detail_lines = help_text();
                    *kind_detail = None;
                    *edge_explorer_idx = None;
                    Action::Render
                }

                _ => Action::None,
            }
        }
    }
}

/// Handle key events when the search overlay is active.
#[allow(clippy::too_many_arguments)]
fn handle_search_key(
    search: &mut Option<SearchState>,
    tree: &mut crate::tui::tree::TaxonomyTree,
    detail_lines: &mut Vec<String>,
    cypher_preview: &mut Vec<String>,
    kind_detail: &mut Option<Box<KindDetail>>,
    status: &mut String,
    mode: NavMode,
    key: KeyEvent,
) -> Action {
    let kinds = tree.all_kinds();

    match key.code {
        KeyCode::Esc => {
            *search = None;
            *status = format!("Mode: {}", mode.label());
            Action::Render
        }
        KeyCode::Enter => {
            if let Some(s) = &*search {
                if let Some(label) = s.selected_label() {
                    let label = label.to_string();
                    tree.jump_to_key(&label);
                    *search = None;
                    *status = format!("Mode: {}", mode.label());
                    // Update detail for the selected node
                    if let Some(fetch_label) =
                        update_detail(tree, detail_lines, cypher_preview, kind_detail)
                    {
                        return Action::FetchDetail(fetch_label);
                    }
                    return Action::Render;
                }
            }
            *search = None;
            *status = format!("Mode: {}", mode.label());
            Action::Render
        }
        KeyCode::Up => {
            if let Some(s) = search {
                s.cursor_up();
            }
            Action::Render
        }
        KeyCode::Down => {
            if let Some(s) = search {
                s.cursor_down();
            }
            Action::Render
        }
        KeyCode::Backspace => {
            if let Some(s) = search {
                s.pop_char(&kinds);
            }
            Action::Render
        }
        KeyCode::Char(c) => {
            if let Some(s) = search {
                s.push_char(c, &kinds);
            }
            Action::Render
        }
        _ => Action::None,
    }
}

/// Update detail pane and cypher preview based on the selected tree node.
/// Returns `Some(label)` if the selected node is a Kind (triggers async fetch).
fn update_detail(
    tree: &crate::tui::tree::TaxonomyTree,
    detail_lines: &mut Vec<String>,
    cypher_preview: &mut Vec<String>,
    kind_detail: &mut Option<Box<KindDetail>>,
) -> Option<String> {
    // Clear previous Kind detail (will be repopulated by async fetch if needed)
    *kind_detail = None;

    if let Some(node) = tree.selected() {
        *cypher_preview = match node.node_type {
            TreeNodeType::Kind => vec![
                format!("MATCH (n:{})", node.key),
                "RETURN n".to_string(),
                "LIMIT 25".to_string(),
            ],
            TreeNodeType::Realm => vec![
                format!("MATCH (r:Realm {{key: '{}'}})", node.key),
                "  -[:HAS_LAYER]->(l:Layer)".to_string(),
                "  -[:HAS_KIND]->(k:Kind)".to_string(),
                "RETURN r, l, k".to_string(),
            ],
            TreeNodeType::Layer => vec![
                format!("MATCH (l:Layer {{key: '{}'}})", node.key),
                "  <-[:IN_LAYER]-(k:Kind)".to_string(),
                "RETURN l, k".to_string(),
            ],
        };

        if node.node_type == TreeNodeType::Kind {
            *detail_lines = vec![
                format!("{} (Kind)", node.display_name),
                String::new(),
                "Loading detail\u{2026}".to_string(),
            ];
            Some(node.key.clone())
        } else {
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
            None
        }
    } else {
        *detail_lines = vec!["No selection".to_string()];
        *cypher_preview = Vec::new();
        None
    }
}

fn help_text() -> Vec<String> {
    vec![
        "NovaNet TUI \u{2014} Keyboard Shortcuts".to_string(),
        "".to_string(),
        "Navigation:".to_string(),
        "  1-4        Switch mode (Data/Meta/Overlay/Query)".to_string(),
        "  Tab        Cycle mode".to_string(),
        "  Left/Right Cycle panel (Tree/Detail/Cypher)".to_string(),
        "  Up/Down    Navigate tree".to_string(),
        "  Enter      Expand/collapse node".to_string(),
        "  Space      Expand/collapse node".to_string(),
        "".to_string(),
        "Filters:".to_string(),
        "  f          Toggle facet filter (Query mode)".to_string(),
        "  Esc        Close popup".to_string(),
        "".to_string(),
        "Search:".to_string(),
        "  /          Open fuzzy search".to_string(),
        "  Esc        Close search".to_string(),
        "  Enter      Select result".to_string(),
        "".to_string(),
        "Edge Explorer:".to_string(),
        "  e          Toggle edge explorer (when Kind selected)".to_string(),
        "  Up/Down    Navigate edges (when explorer open)".to_string(),
        "".to_string(),
        "Other:".to_string(),
        "  ?          Show this help".to_string(),
        "  q          Quit".to_string(),
        "  Ctrl+C     Quit".to_string(),
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

    fn make_ready_state_with_kind() -> AppState {
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
            MetaRow {
                label: "Kind".to_string(),
                key: "Locale".to_string(),
                display_name: "Locale".to_string(),
                parent_key: Some("knowledge".to_string()),
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

    #[test]
    fn panel_cycling_right() {
        let mut state = make_ready_state();
        // Initial: Tree
        handle_key(&mut state, key(KeyCode::Right));
        if let AppState::Ready { active_panel, .. } = &state {
            assert_eq!(*active_panel, ActivePanel::Detail);
        }
        handle_key(&mut state, key(KeyCode::Right));
        if let AppState::Ready { active_panel, .. } = &state {
            assert_eq!(*active_panel, ActivePanel::CypherPreview);
        }
        handle_key(&mut state, key(KeyCode::Right));
        if let AppState::Ready { active_panel, .. } = &state {
            assert_eq!(*active_panel, ActivePanel::Tree);
        }
    }

    #[test]
    fn panel_cycling_left() {
        let mut state = make_ready_state();
        // Initial: Tree, Left wraps to CypherPreview
        handle_key(&mut state, key(KeyCode::Left));
        if let AppState::Ready { active_panel, .. } = &state {
            assert_eq!(*active_panel, ActivePanel::CypherPreview);
        }
    }

    #[test]
    fn cypher_preview_updates_on_navigation() {
        let mut state = make_ready_state();
        // Navigate down to "Knowledge" layer
        handle_key(&mut state, key(KeyCode::Down));
        if let AppState::Ready { cypher_preview, .. } = &state {
            assert!(!cypher_preview.is_empty());
            assert!(cypher_preview[0].contains("Layer"));
        }
    }

    #[test]
    fn kind_selection_returns_fetch_detail() {
        let mut state = make_ready_state_with_kind();
        // Navigate: global -> knowledge -> Locale (Kind)
        handle_key(&mut state, key(KeyCode::Down)); // knowledge
        let action = handle_key(&mut state, key(KeyCode::Down)); // Locale
        assert!(matches!(action, Action::FetchDetail(ref label) if label == "Locale"));
    }

    #[test]
    fn kind_shows_loading_placeholder() {
        let mut state = make_ready_state_with_kind();
        handle_key(&mut state, key(KeyCode::Down)); // knowledge
        handle_key(&mut state, key(KeyCode::Down)); // Locale
        if let AppState::Ready { detail_lines, .. } = &state {
            assert!(detail_lines[0].contains("Locale"));
            assert!(detail_lines.iter().any(|l| l.contains("Loading")));
        }
    }

    #[test]
    fn navigating_away_from_kind_clears_detail() {
        let mut state = make_ready_state_with_kind();
        // Navigate to Locale (Kind)
        handle_key(&mut state, key(KeyCode::Down));
        handle_key(&mut state, key(KeyCode::Down));
        // Manually set kind_detail as if fetch completed
        if let AppState::Ready { kind_detail, .. } = &mut state {
            *kind_detail = Some(Box::new(KindDetail {
                label: "Locale".to_string(),
                display_name: "Locale".to_string(),
                realm: None,
                layer: None,
                trait_key: None,
                context_budget: None,
                schema_hint: None,
                edges_in: vec![],
                edges_out: vec![],
            }));
        }
        // Navigate back up to Layer
        handle_key(&mut state, key(KeyCode::Up));
        if let AppState::Ready { kind_detail, .. } = &state {
            assert!(kind_detail.is_none());
        }
    }

    #[test]
    fn help_clears_kind_detail() {
        let mut state = make_ready_state_with_kind();
        // Navigate to Locale and set kind_detail
        handle_key(&mut state, key(KeyCode::Down));
        handle_key(&mut state, key(KeyCode::Down));
        if let AppState::Ready { kind_detail, .. } = &mut state {
            *kind_detail = Some(Box::new(KindDetail {
                label: "Locale".to_string(),
                display_name: "Locale".to_string(),
                realm: None,
                layer: None,
                trait_key: None,
                context_budget: None,
                schema_hint: None,
                edges_in: vec![],
                edges_out: vec![],
            }));
        }
        // Press ? for help
        handle_key(&mut state, key(KeyCode::Char('?')));
        if let AppState::Ready { kind_detail, .. } = &state {
            assert!(kind_detail.is_none());
        }
    }

    #[test]
    fn slash_opens_search() {
        let mut state = make_ready_state_with_kind();
        let action = handle_key(&mut state, key(KeyCode::Char('/')));
        assert!(matches!(action, Action::Render));
        if let AppState::Ready { search, .. } = &state {
            assert!(search.is_some());
        }
    }

    #[test]
    fn esc_closes_search() {
        let mut state = make_ready_state_with_kind();
        handle_key(&mut state, key(KeyCode::Char('/')));
        let action = handle_key(&mut state, key(KeyCode::Esc));
        assert!(matches!(action, Action::Render));
        if let AppState::Ready { search, .. } = &state {
            assert!(search.is_none());
        }
    }

    #[test]
    fn search_typing_filters() {
        let mut state = make_ready_state_with_kind();
        handle_key(&mut state, key(KeyCode::Char('/')));
        // Type 'l' to match Locale
        handle_key(&mut state, key(KeyCode::Char('l')));
        if let AppState::Ready { search, .. } = &state {
            let s = search.as_ref().unwrap();
            assert_eq!(s.query, "l");
            assert!(!s.results.is_empty());
        }
    }

    #[test]
    fn search_enter_selects_and_jumps() {
        let mut state = make_ready_state_with_kind();
        handle_key(&mut state, key(KeyCode::Char('/')));
        handle_key(&mut state, key(KeyCode::Char('l')));
        handle_key(&mut state, key(KeyCode::Char('o')));
        handle_key(&mut state, key(KeyCode::Char('c')));
        let action = handle_key(&mut state, key(KeyCode::Enter));
        // Search should be closed
        if let AppState::Ready { search, tree, .. } = &state {
            assert!(search.is_none());
            // Should have jumped to Locale
            let selected = tree.selected().unwrap();
            assert_eq!(selected.key, "Locale");
        }
        // Should return FetchDetail since Locale is a Kind
        assert!(matches!(action, Action::FetchDetail(ref l) if l == "Locale"));
    }

    #[test]
    fn search_backspace_removes_char() {
        let mut state = make_ready_state_with_kind();
        handle_key(&mut state, key(KeyCode::Char('/')));
        handle_key(&mut state, key(KeyCode::Char('l')));
        handle_key(&mut state, key(KeyCode::Char('o')));
        handle_key(&mut state, key(KeyCode::Backspace));
        if let AppState::Ready { search, .. } = &state {
            assert_eq!(search.as_ref().unwrap().query, "l");
        }
    }

    #[test]
    fn e_opens_edge_explorer_when_kind_detail_present() {
        let mut state = make_ready_state_with_kind();
        // Navigate to Locale Kind
        handle_key(&mut state, key(KeyCode::Down));
        handle_key(&mut state, key(KeyCode::Down));
        // Set kind_detail with edges
        if let AppState::Ready { kind_detail, .. } = &mut state {
            *kind_detail = Some(Box::new(KindDetail {
                label: "Locale".to_string(),
                display_name: "Locale".to_string(),
                realm: Some("global".to_string()),
                layer: Some("knowledge".to_string()),
                trait_key: None,
                context_budget: None,
                schema_hint: None,
                edges_in: vec![crate::tui::detail::EdgeInfo {
                    key: "IN_LOCALE".to_string(),
                    family: "semantic".to_string(),
                }],
                edges_out: vec![],
            }));
        }
        let action = handle_key(&mut state, key(KeyCode::Char('e')));
        assert!(matches!(action, Action::Render));
        if let AppState::Ready {
            edge_explorer_idx, ..
        } = &state
        {
            assert_eq!(*edge_explorer_idx, Some(0));
        }
    }

    #[test]
    fn e_closes_edge_explorer() {
        let mut state = make_ready_state_with_kind();
        handle_key(&mut state, key(KeyCode::Down));
        handle_key(&mut state, key(KeyCode::Down));
        if let AppState::Ready {
            kind_detail,
            edge_explorer_idx,
            ..
        } = &mut state
        {
            *kind_detail = Some(Box::new(KindDetail {
                label: "Locale".to_string(),
                display_name: "Locale".to_string(),
                realm: None,
                layer: None,
                trait_key: None,
                context_budget: None,
                schema_hint: None,
                edges_in: vec![crate::tui::detail::EdgeInfo {
                    key: "IN_LOCALE".to_string(),
                    family: "semantic".to_string(),
                }],
                edges_out: vec![],
            }));
            *edge_explorer_idx = Some(0);
        }
        // Press e again to close
        handle_key(&mut state, key(KeyCode::Char('e')));
        if let AppState::Ready {
            edge_explorer_idx, ..
        } = &state
        {
            assert!(edge_explorer_idx.is_none());
        }
    }

    #[test]
    fn e_without_kind_detail_does_nothing() {
        let mut state = make_ready_state();
        let action = handle_key(&mut state, key(KeyCode::Char('e')));
        assert!(matches!(action, Action::None));
        if let AppState::Ready {
            edge_explorer_idx, ..
        } = &state
        {
            assert!(edge_explorer_idx.is_none());
        }
    }

    #[test]
    fn edge_explorer_no_edges_shows_status() {
        let mut state = make_ready_state_with_kind();
        handle_key(&mut state, key(KeyCode::Down));
        handle_key(&mut state, key(KeyCode::Down));
        if let AppState::Ready { kind_detail, .. } = &mut state {
            *kind_detail = Some(Box::new(KindDetail {
                label: "Locale".to_string(),
                display_name: "Locale".to_string(),
                realm: None,
                layer: None,
                trait_key: None,
                context_budget: None,
                schema_hint: None,
                edges_in: vec![],
                edges_out: vec![],
            }));
        }
        handle_key(&mut state, key(KeyCode::Char('e')));
        if let AppState::Ready {
            edge_explorer_idx,
            status,
            ..
        } = &state
        {
            // No edges = explorer doesn't open
            assert!(edge_explorer_idx.is_none());
            assert!(status.contains("No edges"));
        }
    }

    #[test]
    fn tree_navigation_clears_edge_explorer() {
        let mut state = make_ready_state_with_kind();
        handle_key(&mut state, key(KeyCode::Down));
        handle_key(&mut state, key(KeyCode::Down));
        if let AppState::Ready {
            kind_detail,
            edge_explorer_idx,
            ..
        } = &mut state
        {
            *kind_detail = Some(Box::new(KindDetail {
                label: "Locale".to_string(),
                display_name: "Locale".to_string(),
                realm: None,
                layer: None,
                trait_key: None,
                context_budget: None,
                schema_hint: None,
                edges_in: vec![crate::tui::detail::EdgeInfo {
                    key: "IN_LOCALE".to_string(),
                    family: "semantic".to_string(),
                }],
                edges_out: vec![],
            }));
            *edge_explorer_idx = Some(0);
        }
        // Navigate tree up — should clear explorer
        handle_key(&mut state, key(KeyCode::Up));
        if let AppState::Ready {
            edge_explorer_idx, ..
        } = &state
        {
            assert!(edge_explorer_idx.is_none());
        }
    }
}
