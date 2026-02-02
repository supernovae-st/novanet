//! Fuzzy search overlay for Kind nodes.
//!
//! Triggered by `/` key. Uses nucleo-matcher for fuzzy matching against
//! Kind labels in the taxonomy tree. Results are ranked by score with
//! realm/layer context shown alongside each match.

use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher};

use crate::tui::tree::KindEntry;

/// A single search result with its match score.
#[derive(Debug, Clone)]
#[allow(dead_code)] // score used in Phase 7B result ranking display
pub struct SearchResult {
    pub label: String,
    pub display_name: String,
    pub realm: String,
    pub layer: String,
    pub score: u32,
}

/// Interactive search state.
#[derive(Debug, Clone)]
pub struct SearchState {
    pub query: String,
    pub cursor: usize,
    pub results: Vec<SearchResult>,
    matcher: MatcherState,
}

/// Opaque wrapper so we can derive Debug/Clone on SearchState.
#[derive(Clone)]
struct MatcherState {
    config: Config,
}

impl std::fmt::Debug for MatcherState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MatcherState").finish()
    }
}

impl SearchState {
    /// Create a new search state.
    pub fn new() -> Self {
        SearchState {
            query: String::new(),
            cursor: 0,
            results: Vec::new(),
            matcher: MatcherState {
                config: Config::DEFAULT,
            },
        }
    }

    /// Append a character to the query and re-filter.
    pub fn push_char(&mut self, c: char, kinds: &[KindEntry]) {
        self.query.push(c);
        self.refilter(kinds);
    }

    /// Remove the last character from the query and re-filter.
    pub fn pop_char(&mut self, kinds: &[KindEntry]) {
        self.query.pop();
        self.refilter(kinds);
    }

    /// Move cursor down in results.
    pub fn cursor_down(&mut self) {
        if !self.results.is_empty() && self.cursor < self.results.len() - 1 {
            self.cursor += 1;
        }
    }

    /// Move cursor up in results.
    pub fn cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Get the selected result's label (if any).
    pub fn selected_label(&self) -> Option<&str> {
        self.results.get(self.cursor).map(|r| r.label.as_str())
    }

    /// Re-run fuzzy match against all kinds.
    fn refilter(&mut self, kinds: &[KindEntry]) {
        if self.query.is_empty() {
            self.results.clear();
            self.cursor = 0;
            return;
        }

        let mut matcher = Matcher::new(self.matcher.config.clone());
        let pattern = Pattern::new(
            &self.query,
            CaseMatching::Ignore,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );

        let labels: Vec<&str> = kinds.iter().map(|k| k.label.as_str()).collect();
        let matches = pattern.match_list(&labels, &mut matcher);

        self.results = matches
            .into_iter()
            .map(|(label, score)| {
                let entry = kinds.iter().find(|k| k.label == *label).unwrap();
                SearchResult {
                    label: entry.label.clone(),
                    display_name: entry.display_name.clone(),
                    realm: entry.realm.clone(),
                    layer: entry.layer.clone(),
                    score,
                }
            })
            .collect();

        // Reset cursor if out of bounds
        if self.cursor >= self.results.len() {
            self.cursor = if self.results.is_empty() {
                0
            } else {
                self.results.len() - 1
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kinds() -> Vec<KindEntry> {
        vec![
            KindEntry {
                label: "Page".to_string(),
                display_name: "Page".to_string(),
                realm: "project".to_string(),
                layer: "structure".to_string(),
            },
            KindEntry {
                label: "PageType".to_string(),
                display_name: "Page Type".to_string(),
                realm: "project".to_string(),
                layer: "instruction".to_string(),
            },
            KindEntry {
                label: "PageL10n".to_string(),
                display_name: "Page L10n".to_string(),
                realm: "project".to_string(),
                layer: "output".to_string(),
            },
            KindEntry {
                label: "Locale".to_string(),
                display_name: "Locale".to_string(),
                realm: "global".to_string(),
                layer: "knowledge".to_string(),
            },
            KindEntry {
                label: "Concept".to_string(),
                display_name: "Concept".to_string(),
                realm: "project".to_string(),
                layer: "knowledge".to_string(),
            },
        ]
    }

    #[test]
    fn empty_query_no_results() {
        let state = SearchState::new();
        assert!(state.results.is_empty());
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn fuzzy_match_page() {
        let kinds = sample_kinds();
        let mut state = SearchState::new();
        state.push_char('p', &kinds);
        state.push_char('a', &kinds);
        state.push_char('g', &kinds);
        // Should match Page, PageType, PageL10n
        assert!(state.results.len() >= 3);
        assert!(state.results.iter().any(|r| r.label == "Page"));
        assert!(state.results.iter().any(|r| r.label == "PageType"));
        assert!(state.results.iter().any(|r| r.label == "PageL10n"));
    }

    #[test]
    fn fuzzy_match_locale() {
        let kinds = sample_kinds();
        let mut state = SearchState::new();
        state.push_char('l', &kinds);
        state.push_char('o', &kinds);
        state.push_char('c', &kinds);
        assert!(state.results.iter().any(|r| r.label == "Locale"));
    }

    #[test]
    fn cursor_navigation() {
        let kinds = sample_kinds();
        let mut state = SearchState::new();
        state.push_char('p', &kinds);
        // Should have multiple results
        assert!(state.results.len() > 1);
        assert_eq!(state.cursor, 0);
        state.cursor_down();
        assert_eq!(state.cursor, 1);
        state.cursor_up();
        assert_eq!(state.cursor, 0);
        state.cursor_up(); // stays at 0
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn pop_char_refilters() {
        let kinds = sample_kinds();
        let mut state = SearchState::new();
        state.push_char('p', &kinds);
        state.push_char('a', &kinds);
        state.push_char('g', &kinds);
        let count_pag = state.results.len();
        state.pop_char(&kinds); // back to "pa"
        // "pa" might match more broadly
        assert!(state.results.len() >= count_pag);
    }

    #[test]
    fn pop_all_clears_results() {
        let kinds = sample_kinds();
        let mut state = SearchState::new();
        state.push_char('p', &kinds);
        assert!(!state.results.is_empty());
        state.pop_char(&kinds);
        assert!(state.results.is_empty());
    }

    #[test]
    fn selected_label_returns_correct() {
        let kinds = sample_kinds();
        let mut state = SearchState::new();
        state.push_char('p', &kinds);
        state.push_char('a', &kinds);
        state.push_char('g', &kinds);
        state.push_char('e', &kinds);
        // First result should be something
        let label = state.selected_label();
        assert!(label.is_some());
    }

    #[test]
    fn results_include_realm_layer() {
        let kinds = sample_kinds();
        let mut state = SearchState::new();
        state.push_char('l', &kinds);
        state.push_char('o', &kinds);
        state.push_char('c', &kinds);
        let locale = state.results.iter().find(|r| r.label == "Locale").unwrap();
        assert_eq!(locale.realm, "global");
        assert_eq!(locale.layer, "knowledge");
    }

    #[test]
    fn no_match_returns_empty() {
        let kinds = sample_kinds();
        let mut state = SearchState::new();
        state.push_char('z', &kinds);
        state.push_char('z', &kinds);
        state.push_char('z', &kinds);
        assert!(state.results.is_empty());
    }
}
