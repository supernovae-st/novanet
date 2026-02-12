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
/// Key includes: bar_width, realm count, and kinds per realm.
/// Changes to any of these will invalidate the cache.
pub(crate) fn compute_mini_bar_cache_key(app: &App, bar_width: usize) -> u64 {
    let mut keys: Vec<u64> = Vec::with_capacity(app.tree.realms.len() + 2);
    keys.push(bar_width as u64);
    keys.push(app.tree.realms.len() as u64);
    for realm in &app.tree.realms {
        let count: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
        keys.push(count as u64);
    }
    combine_hashes(&keys)
}

/// Get contextual keyboard shortcuts based on mode, focus, and selection.
///
/// Returns a static string with the most relevant keyboard hints for the current context.
/// Note: Nexus hints are generated dynamically from context_actions() in render_status.
/// Format: arrows for nav, Enter for action (silent alternatives: hjkl, Space)
pub(crate) fn get_contextual_shortcuts(
    mode: NavMode,
    focus: Focus,
    is_instance: bool,
    is_kind: bool,
) -> &'static str {
    // v11.7: 2 modes, unified tree
    // Display: ↑/↓=vertical, ←/→=horizontal, Enter=action
    // Silent alternatives: hjkl, Space (handled in key processing)
    match mode {
        // Nexus hints are handled separately via context_actions()
        NavMode::Nexus => "",
        NavMode::Graph => match focus {
            Focus::Tree => {
                if is_instance {
                    "↑/↓:nav y:copy Esc:back"
                } else if is_kind {
                    "↑/↓:nav ←/→:expand y:copy"
                } else {
                    "↑/↓:nav ←/→:toggle y:copy"
                }
            }
            Focus::Yaml | Focus::Info => "↑/↓:scroll Enter:page y:copy",
            Focus::Graph => "Tab:panel",
        },
    }
}

/// Build filter indicator string based on app state.
///
/// Returns:
/// - " [KindKey]" if in filtered Data mode
/// - " [hide-empty]" if hide_empty is enabled
/// - Empty string otherwise
pub(crate) fn build_filter_indicator(
    is_filtered: bool,
    filter_kind: Option<&str>,
    hide_empty: bool,
) -> String {
    if is_filtered {
        if let Some(kind_key) = filter_kind {
            format!(" [{}]", kind_key)
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
/// Returns a formatted string like "5 nodes.10 arcs | 3 Kinds.2 ArcKinds"
pub(crate) fn format_stats(
    node_count: i64,
    arc_count: i64,
    kind_count: i64,
    arc_kind_count: i64,
) -> String {
    format!(
        "{} nodes.{} arcs | {} Kinds.{} ArcKinds",
        node_count, arc_count, kind_count, arc_kind_count
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

    // Calculate total kinds from all realms
    let mut realm_counts: Vec<(&str, usize, Color)> = Vec::with_capacity(app.tree.realms.len());
    let mut total_kinds: usize = 0;

    for realm in &app.tree.realms {
        let count: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
        let color = hex_to_color(&realm.color);
        realm_counts.push((&realm.key, count, color));
        total_kinds += count;
    }

    if total_kinds == 0 {
        spans.push(Span::styled("░".repeat(bar_width), STYLE_DIM));
        return spans;
    }

    // Calculate proportional widths and build the bar
    let mut used_width = 0;
    let mut percentages: Vec<(&str, u8, Color)> = Vec::with_capacity(realm_counts.len());

    for (i, (key, count, color)) in realm_counts.iter().enumerate() {
        let proportion = *count as f64 / total_kinds as f64;
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

    // Determine if current item is instance or kind for contextual shortcuts
    let current_item = app.current_item();
    let is_instance = matches!(current_item, Some(TreeItem::Instance(..)));
    let is_kind = matches!(current_item, Some(TreeItem::Kind(..)));

    // Get contextual shortcuts using extracted pure function
    let shortcuts = get_contextual_shortcuts(app.mode, app.focus, is_instance, is_kind);

    // Build filter indicator using extracted pure function
    let filter_indicator = build_filter_indicator(
        app.is_filtered_graph_mode(),
        app.get_filter_kind(),
        app.hide_empty,
    );

    // Build status line spans - UNIFIED FORMAT for both modes:
    // HINTS │ MODE │ BREADCRUMB │ STATS │ REALM-BAR │ [?]
    let mut spans = Vec::with_capacity(20);
    spans.push(Span::raw(" "));

    // 1. HINTS (context-aware, at START for both modes)
    if app.mode == NavMode::Nexus {
        // Nexus: dynamic hints from context_actions()
        let actions = app.nexus.context_actions();
        for (i, (key, action)) in actions.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw(" "));
            }
            spans.push(Span::styled(*key, STYLE_HINT));
            spans.push(Span::styled(format!(":{}", action), STYLE_DIM));
        }
    } else {
        // Graph: static hints from get_contextual_shortcuts()
        spans.push(Span::styled(shortcuts, STYLE_DIM));
    }

    // 2. MODE indicator: │ ✦ NEXUS │ or │ ◆ GRAPH │
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

    // 3. BREADCRUMB (context-aware)
    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    if app.mode == NavMode::Nexus {
        // Nexus: show section > tab (e.g., "LEARN > Intro")
        let nexus_breadcrumb = app.nexus.status_breadcrumb();
        spans.push(Span::styled(nexus_breadcrumb, STYLE_HINT));
    } else {
        // Graph: show tree path (truncated)
        spans.push(Span::styled(breadcrumb_display, STYLE_HINT));
    }

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

    // 4. STATS (full words: nodes.arcs │ kinds.arc-kinds)
    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    let stats = &app.tree.stats;
    spans.push(Span::styled(
        format_stats(
            stats.node_count,
            stats.arc_count,
            stats.kind_count,
            stats.arc_kind_count,
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

    // 6. HELP indicator (always shown, both modes)
    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    spans.push(Span::styled("[?]", STYLE_DIM));

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
    // get_contextual_shortcuts tests (v11.7: 2 modes - Graph, Nexus)
    // =========================================================================

    #[test]
    fn test_shortcuts_nexus_mode() {
        // v11.7: Nexus has its own action bar - status bar returns empty to avoid duplication
        let result = get_contextual_shortcuts(NavMode::Nexus, Focus::Tree, false, false);
        assert_eq!(result, "");
    }

    #[test]
    fn test_shortcuts_graph_mode_tree_focus_on_kind() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, true);
        assert!(result.contains("↑/↓:nav"));
        assert!(result.contains("←/→:expand"));
        assert!(result.contains("y:copy"));
    }

    #[test]
    fn test_shortcuts_graph_mode_tree_focus_not_on_kind() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, false);
        assert!(result.contains("↑/↓:nav"));
        assert!(result.contains("←/→:toggle"));
        assert!(result.contains("y:copy"));
    }

    #[test]
    fn test_shortcuts_graph_mode_yaml_focus() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Yaml, false, false);
        assert!(result.contains("↑/↓:scroll"));
    }

    #[test]
    fn test_shortcuts_graph_mode_info_focus() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Info, false, false);
        assert!(result.contains("↑/↓:scroll"));
    }

    #[test]
    fn test_shortcuts_graph_mode_graph_focus() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Graph, false, false);
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
        // When filtered, the filter kind should show, not hide-empty
        let result = build_filter_indicator(true, Some("Entity"), true);
        assert_eq!(result, " [Entity]");
    }

    #[test]
    fn test_filter_indicator_with_special_chars_in_kind() {
        // Test hyphen handling in kind names (v11.3: locale-knowledge split into 3 layers)
        let result = build_filter_indicator(true, Some("knowledge"), false);
        assert_eq!(result, " [knowledge]");
    }

    // =========================================================================
    // format_stats tests
    // =========================================================================

    #[test]
    fn test_format_stats_zeros() {
        let result = format_stats(0, 0, 0, 0);
        assert_eq!(result, "0 nodes.0 arcs | 0 Kinds.0 ArcKinds");
    }

    #[test]
    fn test_format_stats_typical_values() {
        let result = format_stats(150, 200, 45, 30);
        assert_eq!(result, "150 nodes.200 arcs | 45 Kinds.30 ArcKinds");
    }

    #[test]
    fn test_format_stats_large_values() {
        let result = format_stats(10000, 50000, 500, 250);
        assert_eq!(result, "10000 nodes.50000 arcs | 500 Kinds.250 ArcKinds");
    }

    #[test]
    fn test_format_stats_negative_not_expected_but_handles() {
        // Negative counts shouldn't happen but function handles them
        let result = format_stats(-1, -1, -1, -1);
        assert_eq!(result, "-1 nodes.-1 arcs | -1 Kinds.-1 ArcKinds");
    }

    // =========================================================================
    // realm_block_char tests
    // =========================================================================

    #[test]
    fn test_realm_block_char_global() {
        assert_eq!(realm_block_char("shared"), "▓");
    }

    #[test]
    fn test_realm_block_char_tenant() {
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
    fn test_realm_display_label_global() {
        assert_eq!(realm_display_label("shared"), "Shared");
    }

    #[test]
    fn test_realm_display_label_tenant() {
        assert_eq!(realm_display_label("org"), "Org");
    }

    #[test]
    fn test_realm_display_label_unknown() {
        // Unknown realm returns the key as-is (fallback)
        assert_eq!(realm_display_label("custom"), "custom");
        assert_eq!(realm_display_label("unknown"), "unknown");
    }

    // =========================================================================
    // NavMode tests (v11.7: 2 modes - Graph, Nexus)
    // =========================================================================

    #[test]
    fn test_nav_mode_labels() {
        assert_eq!(NavMode::Graph.label(), "Graph");
        assert_eq!(NavMode::Nexus.label(), "Nexus");
    }

    #[test]
    fn test_nav_mode_index() {
        assert_eq!(NavMode::Graph.index(), 0);
        assert_eq!(NavMode::Nexus.index(), 1);
    }

    // =========================================================================
    // Focus tests
    // =========================================================================

    #[test]
    fn test_focus_next() {
        assert_eq!(Focus::Tree.next(), Focus::Info);
        assert_eq!(Focus::Info.next(), Focus::Graph);
        assert_eq!(Focus::Graph.next(), Focus::Yaml);
        assert_eq!(Focus::Yaml.next(), Focus::Tree);
    }

    #[test]
    fn test_focus_prev() {
        assert_eq!(Focus::Tree.prev(), Focus::Yaml);
        assert_eq!(Focus::Yaml.prev(), Focus::Graph);
        assert_eq!(Focus::Graph.prev(), Focus::Info);
        assert_eq!(Focus::Info.prev(), Focus::Tree);
    }

    // =========================================================================
    // Edge cases and combinations
    // =========================================================================

    #[test]
    fn test_shortcuts_all_focus_panels_for_graph() {
        // Ensure all focus panels produce valid shortcuts for Graph mode
        for focus in [Focus::Tree, Focus::Info, Focus::Graph, Focus::Yaml] {
            let result = get_contextual_shortcuts(NavMode::Graph, focus, false, false);
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
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, false);
        assert!(!result.is_empty(), "Graph mode should have shortcuts");
    }

    #[test]
    fn test_shortcuts_nexus_mode_is_empty() {
        // Nexus has its own action bar, so status bar shortcuts are empty
        let result = get_contextual_shortcuts(NavMode::Nexus, Focus::Tree, false, false);
        assert!(result.is_empty(), "Nexus mode should not have status bar shortcuts");
    }

    #[test]
    fn test_filter_indicator_empty_kind_key() {
        // Empty kind key in filtered mode
        let result = build_filter_indicator(true, Some(""), false);
        assert_eq!(result, " []");
    }
}
