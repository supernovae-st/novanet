//! Status bar rendering for TUI.
//!
//! Displays mode indicator, breadcrumb, loading spinner, stats, realm distribution,
//! and contextual keyboard shortcuts.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::{STYLE_DIM, STYLE_HINT, STYLE_MUTED, STYLE_SEPARATOR};
use crate::tui::app::{App, Focus, NavMode};
use crate::tui::cache::combine_hashes;
use crate::tui::data::TreeItem;
use crate::tui::theme::hex_to_color;
use crate::tui::unicode::truncate_start_to_width;

// =============================================================================
// PURE HELPER FUNCTIONS (testable)
// =============================================================================

/// Compute cache key for realm mini-bar.
///
/// Key includes: bar_width, realm count, and classes per realm.
/// Changes to any of these will invalidate the cache.
pub(crate) fn compute_mini_bar_cache_key(app: &App, bar_width: usize) -> u64 {
    let mut keys: Vec<u64> = Vec::with_capacity(app.tree.realms.len() + 2);
    keys.push(bar_width as u64);
    keys.push(app.tree.realms.len() as u64);
    for realm in &app.tree.realms {
        let count: usize = realm.layers.iter().map(|l| l.classes.len()).sum();
        keys.push(count as u64);
    }
    combine_hashes(&keys)
}

/// Collapse state for contextual shortcuts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapseState {
    /// Item cannot be collapsed (leaf nodes, instances)
    NotCollapsible,
    /// Item is currently collapsed (can expand)
    Collapsed,
    /// Item is currently expanded (can collapse)
    Expanded,
}

/// Get contextual keyboard shortcuts based on focus and selection.
///
/// Returns a string with the most relevant keyboard hints for the current context.
/// Format: arrows for nav, Enter for action (silent alternatives: hjkl, Space)
/// v0.17.3: Added collapse_state for dynamic expand/collapse hints.
pub(crate) fn get_contextual_shortcuts(
    _mode: NavMode,
    focus: Focus,
    is_instance: bool,
    collapse_state: CollapseState,
) -> String {
    // Display: ↑/↓=vertical, ←/→=horizontal, Enter=action
    // Silent alternatives: hjkl, Space (handled in key processing)
    match focus {
        Focus::Tree => {
            if is_instance {
                "↑/↓:nav y:copy Esc:back".to_string()
            } else {
                // v0.17.3: Show expand/collapse based on state (classes, layers, realms)
                match collapse_state {
                    CollapseState::Collapsed => "↑/↓:nav →:expand y:copy".to_string(),
                    CollapseState::Expanded => "↑/↓:nav ←:collapse y:copy".to_string(),
                    CollapseState::NotCollapsible => "↑/↓:nav y:copy".to_string(),
                }
            }
        },
        Focus::Identity => "Tab:panel y:copy".to_string(), // v0.18.3
        Focus::Content | Focus::Props => "↑/↓:scroll Enter:page y:copy".to_string(),
        Focus::Arcs => "↑/↓:scroll Tab:panel".to_string(),
    }
}

/// Build filter indicator string based on app state.
///
/// Returns:
/// - " [ClassKey]" if in filtered Data mode
/// - " [hide-empty]" if hide_empty is enabled
/// - Empty string otherwise
pub(crate) fn build_filter_indicator(
    is_filtered: bool,
    filter_class: Option<&str>,
    hide_empty: bool,
) -> String {
    if is_filtered {
        if let Some(class_key) = filter_class {
            format!(" [{}]", class_key)
        } else {
            String::new()
        }
    } else if hide_empty {
        " [hide-empty]".to_string()
    } else {
        String::new()
    }
}

/// Format stats string for status bar.
///
/// Returns a formatted string like "5 nodes.10 arcs | 3 Classes.2 ArcClasses"
pub(crate) fn format_stats(
    node_count: i64,
    arc_count: i64,
    class_count: i64,
    arc_class_count: i64,
) -> String {
    format!(
        "{} nodes.{} arcs | {} Classes.{} ArcClasses",
        node_count, arc_count, class_count, arc_class_count
    )
}

/// Get block character for a realm.
///
/// - "shared" -> "▓" (lighter block for reference data)
/// - Other realms -> "█" (solid block for business data)
pub(crate) fn realm_block_char(realm_key: &str) -> &'static str {
    match realm_key {
        "shared" => "▓",
        _ => "█",
    }
}

/// Get display label for a realm key.
///
/// - "shared" -> "Shared"
/// - "org" -> "Org"
/// - Other -> returns input as-is
pub(crate) fn realm_display_label(realm_key: &str) -> &str {
    match realm_key {
        "shared" => "Shared",
        "org" => "Org",
        _ => realm_key,
    }
}

// =============================================================================
// STATUS BAR
// =============================================================================

/// Build a mini-bar showing realm distribution (proportional widths).
///
/// Returns a vector of styled spans representing:
/// 1. A bar of block characters (different patterns per realm)
/// 2. Percentage labels for each realm
fn build_realm_mini_bar(app: &App, bar_width: usize) -> Vec<Span<'static>> {
    let mut spans = Vec::with_capacity(8);

    // Calculate total classes from all realms
    let mut realm_counts: Vec<(&str, usize, Color)> = Vec::with_capacity(app.tree.realms.len());
    let mut total_classes: usize = 0;

    for realm in &app.tree.realms {
        let count: usize = realm.layers.iter().map(|l| l.classes.len()).sum();
        let color = hex_to_color(&realm.color);
        realm_counts.push((&realm.key, count, color));
        total_classes += count;
    }

    if total_classes == 0 {
        spans.push(Span::styled("░".repeat(bar_width), STYLE_DIM));
        return spans;
    }

    // Calculate proportional widths and build the bar
    let mut used_width = 0;
    let mut percentages: Vec<(&str, u8, Color)> = Vec::with_capacity(realm_counts.len());

    for (i, (key, count, color)) in realm_counts.iter().enumerate() {
        let proportion = *count as f64 / total_classes as f64;
        let percent = (proportion * 100.0).round() as u8;
        percentages.push((key, percent, *color));

        let width = if i == realm_counts.len() - 1 {
            // Last realm gets remaining width to avoid rounding errors
            bar_width.saturating_sub(used_width)
        } else {
            (proportion * bar_width as f64).round() as usize
        };

        if width > 0 {
            // Use different block characters for distinction
            let block = realm_block_char(key);
            spans.push(Span::styled(
                block.repeat(width),
                Style::default().fg(*color),
            ));
        }
        used_width += width;
    }

    // Add percentages after the bar: " Shared:30% Org:70%"
    for (key, percent, color) in percentages {
        let label = realm_display_label(key);
        spans.push(Span::styled(format!(" {}:", label), STYLE_DIM));
        spans.push(Span::styled(
            format!("{}%", percent),
            Style::default().fg(color),
        ));
    }

    spans
}

/// Status bar: enriched with mode indicator, breadcrumb, shortcuts, spinner.
pub fn render_status(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.theme;

    // Mode indicator with icon and color
    let mode_label = app.mode.label();
    let mode_icon = theme.nav_mode_icon(mode_label);
    let mode_style = theme.nav_mode_style(mode_label);

    // Breadcrumb (truncated from start if too long - uses UTF-8 safe truncation)
    let breadcrumb = app.current_breadcrumb();
    let max_breadcrumb_len = (area.width as usize).saturating_sub(60).min(40);
    let breadcrumb_display = truncate_start_to_width(&breadcrumb, max_breadcrumb_len);

    // Determine if current item is instance or class for contextual shortcuts
    let current_item = app.current_item();
    let is_instance = matches!(current_item, Some(TreeItem::Instance(..)));

    // v0.17.3: Determine collapse state for contextual hints
    let collapse_state = if is_instance {
        CollapseState::NotCollapsible
    } else {
        // Get the collapse key for the current item
        let collapse_key =
            app.tree
                .collapse_key_at(app.tree_cursor, app.is_graph_mode(), app.hide_empty);
        match collapse_key {
            Some(key) => {
                if app.tree.is_collapsed(&key) {
                    CollapseState::Collapsed
                } else {
                    CollapseState::Expanded
                }
            },
            None => CollapseState::NotCollapsible,
        }
    };

    // Get contextual shortcuts using extracted pure function
    let shortcuts = get_contextual_shortcuts(app.mode, app.focus, is_instance, collapse_state);

    // Build filter indicator using extracted pure function
    let filter_indicator = build_filter_indicator(
        app.is_filtered_graph_mode(),
        app.get_filter_class(),
        app.hide_empty,
    );

    // Build status line spans - UNIFIED FORMAT for both modes:
    // HINTS │ MODE │ BREADCRUMB │ STATS │ REALM-BAR │ [?]
    let mut spans = Vec::with_capacity(20);
    spans.push(Span::raw(" "));

    // 1. HINTS (context-aware)
    spans.push(Span::styled(shortcuts, STYLE_DIM));

    // 2. MODE indicator: │ ◆ GRAPH │
    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    spans.push(Span::styled(
        format!("{} {}", mode_icon, mode_label.to_uppercase()),
        mode_style,
    ));

    // Add filter indicator if active (Graph mode only)
    if !filter_indicator.is_empty() {
        spans.push(Span::styled(
            filter_indicator,
            Style::default().fg(Color::Yellow),
        ));
    }

    // v0.18.3: Add focus indicator for debugging panel navigation
    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    spans.push(Span::styled(
        format!("[{}]", app.focus.name()),
        Style::default().fg(Color::Cyan),
    ));

    // 3. BREADCRUMB
    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    spans.push(Span::styled(breadcrumb_display, STYLE_HINT));

    // Loading spinner (if pending load)
    if app.has_pending_load() {
        spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
        spans.push(Span::styled(
            format!("{} Loading...", app.spinner_frame()),
            Style::default().fg(Color::Yellow),
        ));
    }

    // Status message (temporary, e.g., "Copied: key")
    if let Some((msg, _)) = &app.status_message {
        spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
        spans.push(Span::styled(msg.clone(), Style::default().fg(Color::Green)));
    }

    // 4. STATS (full words: nodes.arcs │ classes.arc-classes)
    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    let stats = &app.tree.stats;
    spans.push(Span::styled(
        format_stats(
            stats.node_count,
            stats.arc_count,
            stats.class_count,
            stats.arc_class_count,
        ),
        STYLE_MUTED,
    ));

    // 5. REALM-BAR (mini bar showing realm distribution)
    spans.push(Span::raw(" "));
    let bar_width: usize = 8;
    let cache_key = compute_mini_bar_cache_key(app, bar_width);
    let cached_spans = app
        .mini_bar_cache
        .borrow_mut()
        .get_clone_or_compute(cache_key, || build_realm_mini_bar(app, bar_width));
    spans.extend(cached_spans);

    // 6. MINI-CHEATSHEET: help shortcut
    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    spans.push(Span::styled("?:help", STYLE_DIM));

    spans.push(Span::raw(" "));

    let status = Line::from(spans);
    let paragraph = Paragraph::new(status).style(Style::default().bg(Color::Rgb(15, 15, 20)));

    f.render_widget(paragraph, area);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // =========================================================================
    // get_contextual_shortcuts tests
    // v0.17.3: Added collapse_state parameter tests
    // =========================================================================

    #[test]
    fn test_shortcuts_graph_mode_tree_focus_on_class_collapsed() {
        // v0.17.3: Show "expand" when collapsed
        let result =
            get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, CollapseState::Collapsed);
        assert!(result.contains("↑/↓:nav"));
        assert!(result.contains("→:expand"));
        assert!(result.contains("y:copy"));
    }

    #[test]
    fn test_shortcuts_graph_mode_tree_focus_on_class_expanded() {
        // v0.17.3: Show "collapse" when expanded
        let result =
            get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, CollapseState::Expanded);
        assert!(result.contains("↑/↓:nav"));
        assert!(result.contains("←:collapse"));
        assert!(result.contains("y:copy"));
    }

    #[test]
    fn test_shortcuts_graph_mode_tree_focus_not_on_kind_collapsed() {
        let result =
            get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, CollapseState::Collapsed);
        assert!(result.contains("↑/↓:nav"));
        assert!(result.contains("→:expand"));
        assert!(result.contains("y:copy"));
    }

    #[test]
    fn test_shortcuts_graph_mode_tree_focus_not_on_kind_expanded() {
        let result =
            get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, CollapseState::Expanded);
        assert!(result.contains("↑/↓:nav"));
        assert!(result.contains("←:collapse"));
        assert!(result.contains("y:copy"));
    }

    #[test]
    fn test_shortcuts_graph_mode_yaml_focus() {
        let result = get_contextual_shortcuts(
            NavMode::Graph,
            Focus::Content,
            false,
            CollapseState::NotCollapsible,
        );
        assert!(result.contains("↑/↓:scroll"));
    }

    #[test]
    fn test_shortcuts_graph_mode_props_focus() {
        let result = get_contextual_shortcuts(
            NavMode::Graph,
            Focus::Props,
            false,
            CollapseState::NotCollapsible,
        );
        assert!(result.contains("↑/↓:scroll"));
    }

    #[test]
    fn test_shortcuts_graph_mode_arcs_focus() {
        let result = get_contextual_shortcuts(
            NavMode::Graph,
            Focus::Arcs,
            false,
            CollapseState::NotCollapsible,
        );
        assert!(result.contains("Tab:panel"));
    }

    // =========================================================================
    // build_filter_indicator tests
    // =========================================================================

    #[test]
    fn test_filter_indicator_not_filtered_no_hide() {
        let result = build_filter_indicator(false, None, false);
        assert_eq!(result, "");
    }

    #[test]
    fn test_filter_indicator_hide_empty() {
        let result = build_filter_indicator(false, None, true);
        assert_eq!(result, " [hide-empty]");
    }

    #[test]
    fn test_filter_indicator_filtered_with_kind() {
        let result = build_filter_indicator(true, Some("Page"), false);
        assert_eq!(result, " [Page]");
    }

    #[test]
    fn test_filter_indicator_filtered_no_kind() {
        let result = build_filter_indicator(true, None, false);
        assert_eq!(result, "");
    }

    #[test]
    fn test_filter_indicator_filtered_takes_precedence_over_hide() {
        // When filtered, the filter class should show, not hide-empty
        let result = build_filter_indicator(true, Some("Entity"), true);
        assert_eq!(result, " [Entity]");
    }

    #[test]
    fn test_filter_indicator_with_special_chars_in_class() {
        // Test hyphen handling in class names (v11.3: locale-knowledge split into 3 layers)
        let result = build_filter_indicator(true, Some("knowledge"), false);
        assert_eq!(result, " [knowledge]");
    }

    // =========================================================================
    // format_stats tests
    // =========================================================================

    #[test]
    fn test_format_stats_zeros() {
        let result = format_stats(0, 0, 0, 0);
        assert_eq!(result, "0 nodes.0 arcs | 0 Classes.0 ArcClasses");
    }

    #[test]
    fn test_format_stats_typical_values() {
        let result = format_stats(150, 200, 45, 30);
        assert_eq!(result, "150 nodes.200 arcs | 45 Classes.30 ArcClasses");
    }

    #[test]
    fn test_format_stats_large_values() {
        let result = format_stats(10000, 50000, 500, 250);
        assert_eq!(
            result,
            "10000 nodes.50000 arcs | 500 Classes.250 ArcClasses"
        );
    }

    #[test]
    fn test_format_stats_negative_not_expected_but_handles() {
        // Negative counts shouldn't happen but function handles them
        let result = format_stats(-1, -1, -1, -1);
        assert_eq!(result, "-1 nodes.-1 arcs | -1 Classes.-1 ArcClasses");
    }

    // =========================================================================
    // realm_block_char tests
    // =========================================================================

    #[test]
    fn test_realm_block_char_shared() {
        assert_eq!(realm_block_char("shared"), "▓");
    }

    #[test]
    fn test_realm_block_char_org() {
        assert_eq!(realm_block_char("org"), "█");
    }

    #[test]
    fn test_realm_block_char_unknown() {
        // Any unknown realm should use solid block (fallback)
        assert_eq!(realm_block_char("custom"), "█");
        assert_eq!(realm_block_char("unknown"), "█");
    }

    // =========================================================================
    // realm_display_label tests
    // =========================================================================

    #[test]
    fn test_realm_display_label_shared() {
        assert_eq!(realm_display_label("shared"), "Shared");
    }

    #[test]
    fn test_realm_display_label_org() {
        assert_eq!(realm_display_label("org"), "Org");
    }

    #[test]
    fn test_realm_display_label_unknown() {
        // Unknown realm returns the key as-is (fallback)
        assert_eq!(realm_display_label("custom"), "custom");
        assert_eq!(realm_display_label("unknown"), "unknown");
    }

    // =========================================================================
    // NavMode tests (Graph-only)
    // =========================================================================

    #[test]
    fn test_nav_mode_labels() {
        assert_eq!(NavMode::Graph.label(), "Graph");
    }

    #[test]
    fn test_nav_mode_index() {
        assert_eq!(NavMode::Graph.index(), 0);
    }

    // =========================================================================
    // Focus tests
    // =========================================================================

    #[test]
    fn test_focus_next() {
        // v0.18.3: 5-panel cycle: Tree → Identity → Content → Props → Arcs → Tree
        assert_eq!(Focus::Tree.next(), Focus::Identity);
        assert_eq!(Focus::Identity.next(), Focus::Content);
        assert_eq!(Focus::Content.next(), Focus::Props);
        assert_eq!(Focus::Props.next(), Focus::Arcs);
        assert_eq!(Focus::Arcs.next(), Focus::Tree);
    }

    #[test]
    fn test_focus_prev() {
        // v0.18.3: 5-panel cycle reverse
        assert_eq!(Focus::Tree.prev(), Focus::Arcs);
        assert_eq!(Focus::Arcs.prev(), Focus::Props);
        assert_eq!(Focus::Props.prev(), Focus::Content);
        assert_eq!(Focus::Content.prev(), Focus::Identity);
        assert_eq!(Focus::Identity.prev(), Focus::Tree);
    }

    // =========================================================================
    // Edge cases and combinations
    // =========================================================================

    #[test]
    fn test_shortcuts_all_focus_panels_for_graph() {
        // Ensure all focus panels produce valid shortcuts for Graph mode
        // v0.18.3: 5 panels including Identity
        for focus in [
            Focus::Tree,
            Focus::Identity,
            Focus::Content,
            Focus::Props,
            Focus::Arcs,
        ] {
            let result =
                get_contextual_shortcuts(NavMode::Graph, focus, false, CollapseState::Collapsed);
            assert!(
                !result.is_empty(),
                "Focus {:?} should have shortcuts",
                focus
            );
        }
    }

    #[test]
    fn test_shortcuts_graph_mode_has_output() {
        // Graph mode should produce non-empty shortcuts
        let result =
            get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, CollapseState::Collapsed);
        assert!(!result.is_empty(), "Graph mode should have shortcuts");
    }

    #[test]
    fn test_filter_indicator_empty_class_key() {
        // Empty class key in filtered mode
        let result = build_filter_indicator(true, Some(""), false);
        assert_eq!(result, " []");
    }
}
