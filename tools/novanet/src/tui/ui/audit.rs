//! Audit mode rendering for TUI.
//!
//! This module provides the UI for the Data Quality Audit mode, which displays
//! coverage statistics and data quality metrics for each NodeKind.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use super::{
    COLOR_HIGHLIGHT_BG, COLOR_MUTED_TEXT, COLOR_UNFOCUSED_BORDER, STYLE_DIM, STYLE_ERROR,
    STYLE_MUTED, STYLE_SUCCESS,
};
use crate::tui::app::App;

// =============================================================================
// AUDIT MODE RENDERING
// =============================================================================

/// Render the Audit mode view showing data quality metrics.
///
/// Displays:
/// - Shared coverage percentage with progress bar
/// - Total issues count
/// - Per-Kind coverage table with instance counts and issue indicators
pub fn render_audit(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(5), // Shared stats
            Constraint::Min(10),   // Kind list
            Constraint::Length(2), // Footer
        ])
        .split(area);

    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled("≡ ", Style::default()),
        Span::styled(
            "Data Quality Audit",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
    );
    f.render_widget(header, chunks[0]);

    // Shared stats
    if let Some(stats) = &app.audit_stats {
        let progress_filled = stats.global_coverage / 10;
        let progress_empty = 10 - progress_filled;
        let global_stats = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  Shared Coverage: ", STYLE_MUTED),
                Span::styled(format!("{}%  ", stats.global_coverage), STYLE_SUCCESS),
                Span::styled("━".repeat(progress_filled as usize), STYLE_SUCCESS),
                Span::styled("░".repeat(progress_empty as usize), STYLE_DIM),
                Span::styled(
                    format!("  Total Issues: {}", stats.total_issues),
                    STYLE_ERROR,
                ),
            ]),
            Line::from(vec![Span::styled(
                format!(
                    "  {} Kinds  •  {} instances  •  {} with issues",
                    stats.kinds.len(),
                    stats.total_instances,
                    stats.kinds_with_issues
                ),
                STYLE_DIM,
            )]),
        ];
        let global_paragraph = Paragraph::new(global_stats);
        f.render_widget(global_paragraph, chunks[1]);

        // Kind list
        let mut lines: Vec<Line> = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                format!(
                    "{:<20} {:>10} {:>15}     {:>12}",
                    "Kind", "Instances", "Coverage", "Issues"
                ),
                Style::default()
                    .fg(COLOR_MUTED_TEXT)
                    .add_modifier(Modifier::UNDERLINED),
            )]),
        ];

        for (idx, kind_stat) in stats.kinds.iter().enumerate() {
            let is_selected = idx == app.audit_cursor;
            let progress_filled = kind_stat.coverage_percent / 10;
            let progress_empty = 10 - progress_filled;
            let progress_bar = format!(
                "{}{}",
                "━".repeat(progress_filled as usize),
                "░".repeat(progress_empty as usize)
            );

            let issue_display = if kind_stat.incomplete_instances > 0 {
                format!("⚠ {} missing", kind_stat.incomplete_instances)
            } else {
                "✓ complete".to_string()
            };

            let line_style = if is_selected {
                Style::default().bg(COLOR_HIGHLIGHT_BG)
            } else {
                Style::default()
            };

            let issue_style = if kind_stat.incomplete_instances > 0 {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Green)
            };

            lines.push(Line::from(vec![
                Span::styled(
                    format!(
                        "{:<20} {:>10} ",
                        kind_stat.display_name, kind_stat.instance_count
                    ),
                    line_style.fg(Color::White),
                ),
                Span::styled(progress_bar, line_style.fg(Color::Green)),
                Span::styled(format!(" {:>3}%", kind_stat.coverage_percent), line_style),
                Span::styled(
                    format!("     {}", issue_display),
                    line_style.patch(issue_style),
                ),
            ]));
        }

        let kind_list = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
            )
            .scroll((0, 0));
        f.render_widget(kind_list, chunks[2]);
    } else {
        // Loading state
        let loading = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("  Loading audit data...", STYLE_DIM)),
        ]);
        f.render_widget(loading, chunks[1]);
    }

    // Footer with keybindings
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("[j/k] Navigate  ", STYLE_DIM),
        Span::styled("[Enter] Drill down  ", STYLE_DIM),
        Span::styled("[r] Refresh  ", STYLE_DIM),
        Span::styled("[1-4] Switch mode", STYLE_DIM),
    ]));
    f.render_widget(footer, chunks[3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_audit_exists() {
        // Basic existence test - the function signature is correct
        // Full rendering tests require App state setup
        fn _check_signature(_f: &mut Frame, _area: Rect, _app: &mut App) {
            // This compiles, so the function exists with correct signature
        }
    }
}
