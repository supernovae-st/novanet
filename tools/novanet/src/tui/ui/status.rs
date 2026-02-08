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
            let block = match *key {
                "global" => "▓", // Lighter block for global (reference data)
                _ => "█",        // Solid block for tenant (business data)
            };
            spans.push(Span::styled(
                block.repeat(width),
                Style::default().fg(*color),
            ));
        }
        used_width += width;
    }

    // Add percentages after the bar: " Global:30% Tenant:70%"
    for (key, percent, color) in percentages {
        let label = match key {
            "global" => "Global",
            "tenant" => "Tenant",
            _ => key,
        };
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

    // Contextual shortcuts based on mode, focus, and selection
    let shortcuts = match app.mode {
        NavMode::Atlas => "j/k:nav  1-4:modes  ?:help",
        NavMode::Audit => "j/k:nav  1-4:modes  ?:help",
        NavMode::Guide => "j/k:nav  Enter:select  q:back  ?:help",
        NavMode::Data => {
            // Check if on an Instance (can navigate to Kind with '1')
            if matches!(app.current_item(), Some(TreeItem::Instance(..))) {
                "j/k:nav  y/Y:copy  1:Kind  ?:help"
            } else {
                "j/k:nav  y:copy  h/l:toggle  0:hide  ?:help"
            }
        }
        NavMode::Query => "j/k:nav  f:filter  ?:help",
        NavMode::Meta | NavMode::Overlay => match app.focus {
            Focus::Tree => {
                // Check if on a Kind (can drill into instances with '2')
                if matches!(app.current_item(), Some(TreeItem::Kind(..))) {
                    "j/k:nav  y:copy  2:Data  h/l:toggle  ?:help"
                } else {
                    "j/k:nav  y:copy  h/l:toggle  ?:help"
                }
            }
            Focus::Yaml | Focus::Info => "j/k:scroll  d/u:page  g/G:jump",
            Focus::Graph => "Tab:panel  1-4:modes",
        },
    };

    // Filter indicator (show when filter is active)
    let filter_indicator = if app.is_filtered_data_mode() {
        if let Some(kind_key) = app.get_filter_kind() {
            format!(" [{}]", kind_key)
        } else {
            String::new()
        }
    } else if app.hide_empty {
        " [hide-empty]".to_string()
    } else {
        String::new()
    };

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
        format!(
            "{} nodes.{} arcs | {} Kinds.{} ArcKinds",
            stats.node_count, stats.arc_count, stats.kind_count, stats.arc_kind_count
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
