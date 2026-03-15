//! Fuzzy search logic for the TUI tree.
//!
//! Uses nucleo for smart-case fuzzy matching across all visible
//! tree items (Classes, Arcs, Realms, Layers, ArcFamilies).

use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Normalization};
use nucleo_matcher::{Config, Matcher, Utf32Str};

use super::App;
use crate::tui::data::CollapseKey;

impl App {
    /// Update search results based on current query using nucleo fuzzy matching.
    /// Results are sorted by match score (best matches first).
    pub fn update_search(&mut self) {
        self.search.results.clear();
        self.search.scores.clear();
        self.search.matches.clear();

        if self.search.query.is_empty() {
            return;
        }

        // Setup nucleo matcher with smart case matching
        let mut matcher = Matcher::new(Config::DEFAULT);
        let pattern = Atom::new(
            &self.search.query,
            CaseMatching::Smart, // Case-insensitive unless query has uppercase
            Normalization::Smart,
            AtomKind::Fuzzy, // Fuzzy matching (not exact)
            false,           // No append
        );

        // Collect all (idx, score, match_positions) tuples
        let mut matches: Vec<(usize, u16, Vec<u32>)> = Vec::new();
        let mut idx = 0;

        // Helper to check fuzzy match and collect positions
        let fuzzy_match =
            |text: &str, matcher: &mut Matcher, pattern: &Atom| -> Option<(u16, Vec<u32>)> {
                let mut buf = Vec::new();
                let haystack = Utf32Str::new(text, &mut buf);
                let mut indices = Vec::new();
                pattern
                    .indices(haystack, matcher, &mut indices)
                    .map(|score| (score, indices))
            };

        // Classes section header
        if let Some((score, indices)) = fuzzy_match("Node Classes", &mut matcher, &pattern) {
            matches.push((idx, score, indices));
        }
        idx += 1;

        if !self.tree.is_collapsed(&CollapseKey::Classes) {
            for realm in &self.tree.realms {
                // Check display_name and key, take best match
                let match_display = fuzzy_match(&realm.display_name, &mut matcher, &pattern);
                let match_key = fuzzy_match(&realm.key, &mut matcher, &pattern);
                if let Some((score, indices)) = match_display.or(match_key) {
                    matches.push((idx, score, indices));
                }
                idx += 1;

                if !self
                    .tree
                    .is_collapsed(&CollapseKey::Realm(realm.key.clone()))
                {
                    for layer in &realm.layers {
                        let match_display =
                            fuzzy_match(&layer.display_name, &mut matcher, &pattern);
                        let match_key = fuzzy_match(&layer.key, &mut matcher, &pattern);
                        if let Some((score, indices)) = match_display.or(match_key) {
                            matches.push((idx, score, indices));
                        }
                        idx += 1;

                        if !self.tree.is_collapsed(&CollapseKey::Layer {
                            realm: realm.key.clone(),
                            layer: layer.key.clone(),
                        }) {
                            for class_info in &layer.classes {
                                let match_display =
                                    fuzzy_match(&class_info.display_name, &mut matcher, &pattern);
                                let match_key =
                                    fuzzy_match(&class_info.key, &mut matcher, &pattern);
                                if let Some((score, indices)) = match_display.or(match_key) {
                                    matches.push((idx, score, indices));
                                }
                                idx += 1;
                            }
                        }
                    }
                }
            }
        }

        // Arcs section header
        if let Some((score, indices)) = fuzzy_match("Arcs", &mut matcher, &pattern) {
            matches.push((idx, score, indices));
        }
        idx += 1;

        if !self.tree.is_collapsed(&CollapseKey::Arcs) {
            for family in &self.tree.arc_families {
                let match_display = fuzzy_match(&family.display_name, &mut matcher, &pattern);
                let match_key = fuzzy_match(&family.key, &mut matcher, &pattern);
                if let Some((score, indices)) = match_display.or(match_key) {
                    matches.push((idx, score, indices));
                }
                idx += 1;

                if !self
                    .tree
                    .is_collapsed(&CollapseKey::Family(family.key.clone()))
                {
                    for arc_class in &family.arc_classes {
                        let match_display =
                            fuzzy_match(&arc_class.display_name, &mut matcher, &pattern);
                        let match_key = fuzzy_match(&arc_class.key, &mut matcher, &pattern);
                        if let Some((score, indices)) = match_display.or(match_key) {
                            matches.push((idx, score, indices));
                        }
                        idx += 1;
                    }
                }
            }
        }

        // Sort by score (descending - best matches first)
        matches.sort_by(|a, b| b.1.cmp(&a.1));

        // Extract into separate vectors
        for (idx, score, indices) in matches {
            self.search.results.push(idx);
            self.search.scores.push(score);
            self.search.matches.insert(idx, indices);
        }

        // Reset cursor if out of bounds
        if self.search.cursor >= self.search.results.len() {
            self.search.cursor = 0;
        }
    }

    /// Select current search result and close search.
    pub fn select_search_result(&mut self) {
        if let Some(&idx) = self.search.results.get(self.search.cursor) {
            self.tree_cursor = idx;
            self.ensure_cursor_visible();
        }
        self.close_search();
    }

    /// Close search overlay.
    pub fn close_search(&mut self) {
        self.search.clear();
    }

    /// Navigate to next search result (n key).
    pub fn next_search_result(&mut self) {
        if self.search.results.is_empty() {
            return;
        }
        let max = self.search.results.len().saturating_sub(1);
        self.search.cursor = (self.search.cursor + 1).min(max);
        if let Some(&target_idx) = self.search.results.get(self.search.cursor) {
            self.tree_cursor = target_idx;
            self.ensure_cursor_visible();
            self.load_yaml_for_current();
        }
    }

    /// Navigate to previous search result (N key).
    pub fn prev_search_result(&mut self) {
        if self.search.results.is_empty() {
            return;
        }
        self.search.cursor = self.search.cursor.saturating_sub(1);
        if let Some(&target_idx) = self.search.results.get(self.search.cursor) {
            self.tree_cursor = target_idx;
            self.ensure_cursor_visible();
            self.load_yaml_for_current();
        }
    }
}
