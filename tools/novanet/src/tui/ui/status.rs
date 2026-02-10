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
use crate::tui::data::TreeItem;
use crate::tui::theme::hex_to_color;
use crate::tui::unicode::truncate_start_to_width;

// =============================================================================
// PURE HELPER FUNCTIONS (testable)
// =============================================================================

/// Get contextual keyboard shortcuts based on mode, focus, and selection.
///
/// Returns a static string with the most relevant keyboard hints for the current context.
pub(crate) fn get_contextual_shortcuts(
    mode: NavMode,
    focus: Focus,
    is_instance: bool,
    is_kind: bool,
) -> &'static str {
    // v11.3: 3 modes - Graph, Audit, Nexus
    match mode {
        NavMode::Audit => "j/k:nav  1-3:modes  r:refresh  ?:help",
        NavMode::Nexus => "j/k:nav  Enter:select  Esc:back  ?:help",
        NavMode::Graph => match focus {
            Focus::Tree => {
                if is_instance {
                    "j/k:nav  y/Y:copy  t:toggle view  ?:help"
                } else if is_kind {
                    "j/k:nav  y:copy  t:toggle view  h/l:expand  ?:help"
                } else {
                    "j/k:nav  y:copy  h/l:toggle  t:toggle view  ?:help"
                }
            }
            Focus::Yaml | Focus::Info => "j/k:scroll  d/u:page  g/G:jump",
            Focus::Graph => "Tab:panel  1-3:modes",
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
        app.is_filtered_data_mode(),
        app.get_filter_kind(),
        app.hide_empty,
    );

    // Build status line spans
    let mut spans = vec![
        // Mode indicator: [M META]
        Span::raw(" "),
        Span::styled(
            format!("{} {}", mode_icon, mode_label.to_uppercase()),
            mode_style,
        ),
    ];

    // Add filter indicator if active
    if !filter_indicator.is_empty() {
        spans.push(Span::styled(
            filter_indicator,
            Style::default().fg(Color::Yellow),
        ));
    }

    spans.push(Span::styled(" | ", STYLE_SEPARATOR));
    // Breadcrumb
    spans.push(Span::styled(breadcrumb_display, STYLE_HINT));

    // Loading spinner (if pending load)
    if app.has_pending_load() {
        spans.push(Span::styled(" | ", STYLE_SEPARATOR));
        spans.push(Span::styled(
            format!("{} Loading...", app.spinner_frame()),
            Style::default().fg(Color::Yellow),
        ));
    }

    // Status message (temporary, e.g., "Copied: key")
    if let Some((msg, _)) = &app.status_message {
        spans.push(Span::styled(" | ", STYLE_SEPARATOR));
        spans.push(Span::styled(msg.clone(), Style::default().fg(Color::Green)));
    }

    // Spacer to push shortcuts to the right
    spans.push(Span::raw("  "));

    // Stats (full words: nodes.arcs | kinds.arc-kinds)
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

    // Mini realm distribution bar (8 char width) - shows proportion of kinds per realm
    spans.push(Span::styled(" ", STYLE_SEPARATOR));
    spans.extend(build_realm_mini_bar(app, 8));

    spans.push(Span::styled(" | ", STYLE_SEPARATOR));

    // Contextual shortcuts
    spans.push(Span::styled(shortcuts, STYLE_DIM));

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
    // get_contextual_shortcuts tests (v11.3: 3 modes - Graph, Audit, Nexus)
    // =========================================================================

    #[test]
    fn test_shortcuts_audit_mode() {
        let result = get_contextual_shortcuts(NavMode::Audit, Focus::Tree, false, false);
        assert_eq!(result, "j/k:nav  1-3:modes  r:refresh  ?:help");
    }

    #[test]
    fn test_shortcuts_nexus_mode() {
        let result = get_contextual_shortcuts(NavMode::Nexus, Focus::Tree, false, false);
        assert_eq!(result, "j/k:nav  Enter:select  Esc:back  ?:help");
    }

    #[test]
    fn test_shortcuts_graph_mode_tree_focus_on_kind() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, true);
        assert!(result.contains("j/k:nav"));
        assert!(result.contains("?:help"));
    }

    #[test]
    fn test_shortcuts_graph_mode_tree_focus_not_on_kind() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Tree, false, false);
        assert!(result.contains("j/k:nav"));
        assert!(result.contains("?:help"));
    }

    #[test]
    fn test_shortcuts_graph_mode_yaml_focus() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Yaml, false, false);
        assert!(result.contains("j/k:scroll"));
    }

    #[test]
    fn test_shortcuts_graph_mode_info_focus() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Info, false, false);
        assert!(result.contains("j/k:scroll"));
    }

    #[test]
    fn test_shortcuts_graph_mode_graph_focus() {
        let result = get_contextual_shortcuts(NavMode::Graph, Focus::Graph, false, false);
        assert!(result.contains("1-3:modes"));
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
    // NavMode tests (v11.3: 3 modes - Graph, Audit, Nexus)
    // =========================================================================

    #[test]
    fn test_nav_mode_labels() {
        assert_eq!(NavMode::Graph.label(), "Graph");
        assert_eq!(NavMode::Audit.label(), "Audit");
        assert_eq!(NavMode::Nexus.label(), "Nexus");
    }

    #[test]
    fn test_nav_mode_cycle() {
        assert_eq!(NavMode::Graph.cycle(), NavMode::Audit);
        assert_eq!(NavMode::Audit.cycle(), NavMode::Nexus);
        assert_eq!(NavMode::Nexus.cycle(), NavMode::Graph);
    }

    #[test]
    fn test_nav_mode_index() {
        assert_eq!(NavMode::Graph.index(), 0);
        assert_eq!(NavMode::Audit.index(), 1);
        assert_eq!(NavMode::Nexus.index(), 2);
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
    fn test_shortcuts_all_modes_have_output() {
        // All modes should produce non-empty shortcuts
        for mode in [NavMode::Graph, NavMode::Audit, NavMode::Nexus] {
            let result = get_contextual_shortcuts(mode, Focus::Tree, false, false);
            assert!(!result.is_empty(), "Mode {:?} should have shortcuts", mode);
        }
    }

    #[test]
    fn test_filter_indicator_empty_kind_key() {
        // Empty kind key in filtered mode
        let result = build_filter_indicator(true, Some(""), false);
        assert_eq!(result, " []");
    }
}
