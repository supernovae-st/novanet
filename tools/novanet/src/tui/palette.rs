//! Command palette: fuzzy-searchable command overlay.
//!
//! Triggered by `:` key. Provides access to all TUI commands via fuzzy
//! matching (nucleo). Commands are grouped by category and dispatched
//! to the appropriate action on selection.

use crate::tui::app::NavMode;

/// Command category for grouping in the palette UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandCategory {
    Navigation,
    Action,
    View,
    #[allow(dead_code)] // Used when filter commands are added in Phase 7C
    Filter,
    System,
}

impl CommandCategory {
    pub fn label(self) -> &'static str {
        match self {
            CommandCategory::Navigation => "nav",
            CommandCategory::Action => "act",
            CommandCategory::View => "view",
            CommandCategory::Filter => "filter",
            CommandCategory::System => "sys",
        }
    }
}

/// What happens when a command is selected.
#[derive(Debug, Clone)]
pub enum CommandAction {
    SwitchMode(NavMode),
    OpenCreateNode,
    OpenCreateRelation,
    ToggleDashboard,
    ToggleEdgeExplorer,
    ShowHelp,
    Refresh,
    Quit,
}

/// A registered command in the palette.
#[derive(Debug, Clone)]
pub struct Command {
    #[allow(dead_code)] // Programmatic key (useful for keybindings/scripting in Phase 7C)
    pub key: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub category: CommandCategory,
    pub shortcut: Option<&'static str>,
    pub action: CommandAction,
}

/// Full command registry.
pub fn all_commands() -> Vec<Command> {
    vec![
        // Navigation
        Command {
            key: "data.mode",
            label: "data mode",
            description: "Switch to Data navigation",
            category: CommandCategory::Navigation,
            shortcut: Some("1"),
            action: CommandAction::SwitchMode(NavMode::Data),
        },
        Command {
            key: "meta.mode",
            label: "meta mode",
            description: "Switch to Meta navigation",
            category: CommandCategory::Navigation,
            shortcut: Some("2"),
            action: CommandAction::SwitchMode(NavMode::Meta),
        },
        Command {
            key: "overlay.mode",
            label: "overlay mode",
            description: "Switch to Overlay navigation",
            category: CommandCategory::Navigation,
            shortcut: Some("3"),
            action: CommandAction::SwitchMode(NavMode::Overlay),
        },
        Command {
            key: "query.mode",
            label: "query mode",
            description: "Switch to Query navigation",
            category: CommandCategory::Navigation,
            shortcut: Some("4"),
            action: CommandAction::SwitchMode(NavMode::Query),
        },
        // Actions
        Command {
            key: "create.node",
            label: "create node",
            description: "Open node creation dialog",
            category: CommandCategory::Action,
            shortcut: Some("n"),
            action: CommandAction::OpenCreateNode,
        },
        Command {
            key: "create.relation",
            label: "create relation",
            description: "Open relation creation dialog",
            category: CommandCategory::Action,
            shortcut: Some("r"),
            action: CommandAction::OpenCreateRelation,
        },
        Command {
            key: "edge.explorer",
            label: "edge explorer",
            description: "Toggle edge explorer for selected Kind",
            category: CommandCategory::Action,
            shortcut: Some("e"),
            action: CommandAction::ToggleEdgeExplorer,
        },
        // View
        Command {
            key: "toggle.dashboard",
            label: "toggle dashboard",
            description: "Show/hide dashboard stats panel",
            category: CommandCategory::View,
            shortcut: Some("s"),
            action: CommandAction::ToggleDashboard,
        },
        // System
        Command {
            key: "refresh",
            label: "refresh",
            description: "Refresh all data from Neo4j",
            category: CommandCategory::System,
            shortcut: Some("F5"),
            action: CommandAction::Refresh,
        },
        Command {
            key: "help",
            label: "help",
            description: "Show keyboard reference",
            category: CommandCategory::System,
            shortcut: Some("?"),
            action: CommandAction::ShowHelp,
        },
        Command {
            key: "quit",
            label: "quit",
            description: "Exit application",
            category: CommandCategory::System,
            shortcut: Some("q"),
            action: CommandAction::Quit,
        },
    ]
}

/// State for the command palette overlay.
#[derive(Debug, Clone)]
pub struct PaletteState {
    pub query: String,
    pub results: Vec<PaletteResult>,
    pub cursor: usize,
    commands: Vec<Command>,
}

/// A matched command with its score.
#[derive(Debug, Clone)]
pub struct PaletteResult {
    pub command_idx: usize,
    #[allow(dead_code)] // Score available for match quality display in Phase 7C
    pub score: u32,
}

impl PaletteState {
    pub fn new() -> Self {
        let commands = all_commands();
        let results: Vec<PaletteResult> = commands
            .iter()
            .enumerate()
            .map(|(i, _)| PaletteResult {
                command_idx: i,
                score: 0,
            })
            .collect();
        PaletteState {
            query: String::new(),
            results,
            cursor: 0,
            commands,
        }
    }

    /// Push a character and re-filter.
    pub fn push_char(&mut self, c: char) {
        self.query.push(c);
        self.refilter();
    }

    /// Pop a character and re-filter.
    pub fn pop_char(&mut self) {
        self.query.pop();
        self.refilter();
    }

    /// Move cursor up.
    pub fn cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor down.
    pub fn cursor_down(&mut self) {
        if !self.results.is_empty() && self.cursor < self.results.len() - 1 {
            self.cursor += 1;
        }
    }

    /// Get the selected command action (if any).
    pub fn selected_action(&self) -> Option<&CommandAction> {
        self.results
            .get(self.cursor)
            .map(|r| &self.commands[r.command_idx].action)
    }

    /// Get a command by result index.
    pub fn command_at(&self, result_idx: usize) -> Option<&Command> {
        self.results
            .get(result_idx)
            .map(|r| &self.commands[r.command_idx])
    }

    /// Re-filter commands based on current query.
    fn refilter(&mut self) {
        self.cursor = 0;
        if self.query.is_empty() {
            // Show all commands when query is empty
            self.results = self
                .commands
                .iter()
                .enumerate()
                .map(|(i, _)| PaletteResult {
                    command_idx: i,
                    score: 0,
                })
                .collect();
            return;
        }

        let query_lower = self.query.to_lowercase();
        let mut scored: Vec<(usize, u32)> = self
            .commands
            .iter()
            .enumerate()
            .filter_map(|(i, cmd)| {
                let score = fuzzy_score(&query_lower, cmd.label, cmd.description);
                if score > 0 { Some((i, score)) } else { None }
            })
            .collect();

        // Sort by score descending
        scored.sort_by(|a, b| b.1.cmp(&a.1));

        self.results = scored
            .into_iter()
            .map(|(idx, score)| PaletteResult {
                command_idx: idx,
                score,
            })
            .collect();
    }
}

/// Simple fuzzy scoring: prefix match on label or description words.
///
/// Returns 0 if no match, higher scores for better matches.
fn fuzzy_score(query: &str, label: &str, description: &str) -> u32 {
    let label_lower = label.to_lowercase();
    let desc_lower = description.to_lowercase();

    // Exact prefix match on label is highest score
    if label_lower.starts_with(query) {
        return 100;
    }

    // Word-start match in label
    if label_lower.split_whitespace().any(|w| w.starts_with(query)) {
        return 80;
    }

    // Substring match in label
    if label_lower.contains(query) {
        return 60;
    }

    // Word-start match in description
    if desc_lower.split_whitespace().any(|w| w.starts_with(query)) {
        return 40;
    }

    // Substring match in description
    if desc_lower.contains(query) {
        return 20;
    }

    // Fuzzy character sequence match in label
    if fuzzy_chars_match(query, &label_lower) {
        return 10;
    }

    0
}

/// Check if all query chars appear in order in the target string.
fn fuzzy_chars_match(query: &str, target: &str) -> bool {
    let mut target_chars = target.chars();
    for qc in query.chars() {
        loop {
            match target_chars.next() {
                Some(tc) if tc == qc => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_commands_not_empty() {
        let cmds = all_commands();
        assert!(cmds.len() >= 10);
    }

    #[test]
    fn palette_starts_with_all_commands() {
        let palette = PaletteState::new();
        assert_eq!(palette.results.len(), palette.commands.len());
        assert_eq!(palette.cursor, 0);
    }

    #[test]
    fn palette_filter_narrows_results() {
        let mut palette = PaletteState::new();
        palette.push_char('q');
        assert!(!palette.results.is_empty());
        // "quit" and "query mode" should match
        let labels: Vec<&str> = palette
            .results
            .iter()
            .map(|r| palette.commands[r.command_idx].label)
            .collect();
        assert!(labels.contains(&"quit"));
        assert!(labels.contains(&"query mode"));
    }

    #[test]
    fn palette_filter_empty_query_shows_all() {
        let mut palette = PaletteState::new();
        palette.push_char('x');
        palette.pop_char();
        assert_eq!(palette.results.len(), palette.commands.len());
    }

    #[test]
    fn palette_cursor_navigation() {
        let mut palette = PaletteState::new();
        assert_eq!(palette.cursor, 0);
        palette.cursor_down();
        assert_eq!(palette.cursor, 1);
        palette.cursor_up();
        assert_eq!(palette.cursor, 0);
        palette.cursor_up(); // stays at 0
        assert_eq!(palette.cursor, 0);
    }

    #[test]
    fn palette_selected_action() {
        let palette = PaletteState::new();
        let action = palette.selected_action();
        assert!(action.is_some());
    }

    #[test]
    fn fuzzy_score_exact_prefix() {
        assert_eq!(
            fuzzy_score("data", "data mode", "Switch to Data navigation"),
            100
        );
    }

    #[test]
    fn fuzzy_score_word_start() {
        assert_eq!(
            fuzzy_score("mode", "data mode", "Switch to Data navigation"),
            80
        );
    }

    #[test]
    fn fuzzy_score_substring() {
        assert_eq!(
            fuzzy_score("ode", "data mode", "Switch to Data navigation"),
            60
        );
    }

    #[test]
    fn fuzzy_score_description_match() {
        assert_eq!(
            fuzzy_score("switch", "data mode", "Switch to Data navigation"),
            40
        );
    }

    #[test]
    fn fuzzy_score_no_match() {
        assert_eq!(fuzzy_score("zzzzz", "data mode", "Switch to Data"), 0);
    }

    #[test]
    fn fuzzy_chars_match_works() {
        assert!(fuzzy_chars_match("dm", "data mode"));
        assert!(fuzzy_chars_match("qm", "query mode"));
        assert!(!fuzzy_chars_match("zz", "data mode"));
    }

    #[test]
    fn fuzzy_score_char_sequence() {
        // "cn" matches "create node" via char sequence
        assert!(fuzzy_score("cn", "create node", "Open node creation") > 0);
    }

    #[test]
    fn command_at_returns_correct() {
        let palette = PaletteState::new();
        let cmd = palette.command_at(0).unwrap();
        assert!(!cmd.label.is_empty());
    }

    #[test]
    fn category_labels() {
        assert_eq!(CommandCategory::Navigation.label(), "nav");
        assert_eq!(CommandCategory::System.label(), "sys");
    }
}
