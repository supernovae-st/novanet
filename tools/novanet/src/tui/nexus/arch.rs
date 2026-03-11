//! Architecture Decision Records (ADRs) browser.
//!
//! This tab allows browsing NovaNet's ADRs in the TUI.
//! Layout:
//! - Left: Category list (Core Principles, Schema Architecture, etc.)
//! - Right: ADR list + details panel

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::App;
use crate::tui::data::{AdrCategory, AdrEntry, get_all_adrs};
use crate::tui::ui::COLOR_UNFOCUSED_BORDER;

// =============================================================================
// CONSTANTS
// =============================================================================

/// Category icons for visual distinction.
fn category_icon(cat: &AdrCategory) -> &'static str {
    match cat {
        AdrCategory::CorePrinciples => "◆",
        AdrCategory::SchemaArchitecture => "◇",
        AdrCategory::UxArchitecture => "◈",
        AdrCategory::ArcPolicies => "→",
        AdrCategory::LayerEvolution => "▣",
    }
}

/// Category colors for visual distinction.
fn category_color(cat: &AdrCategory) -> Color {
    match cat {
        AdrCategory::CorePrinciples => Color::Rgb(42, 161, 152), // Teal (cyan)
        AdrCategory::SchemaArchitecture => Color::Rgb(38, 139, 210), // Blue
        AdrCategory::UxArchitecture => Color::Rgb(211, 54, 130), // Magenta
        AdrCategory::ArcPolicies => Color::Rgb(181, 137, 0),     // Yellow
        AdrCategory::LayerEvolution => Color::Rgb(133, 153, 0),  // Green
    }
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Arch tab (ADR browser).
pub fn render_arch_tab(f: &mut Frame, app: &App, area: Rect) {
    // Split into categories (left 30%) and ADR content (right 70%)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    render_category_list(f, app, chunks[0]);
    render_adr_content(f, app, chunks[1]);
}

/// Render the category list on the left.
fn render_category_list(f: &mut Frame, app: &App, area: Rect) {
    let categories = AdrCategory::all();
    let all_adrs = get_all_adrs();
    let selected_idx = app.nexus.arch_adr_index;

    // Find the category of the selected ADR
    let selected_category = all_adrs
        .get(selected_idx)
        .map(|adr| adr.category)
        .unwrap_or(AdrCategory::CorePrinciples);

    let block = Block::default()
        .title(Span::styled(
            " CATEGORIES ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line<'static>> = Vec::new();
    lines.push(Line::from("")); // Top padding

    for cat in categories {
        let is_selected = *cat == selected_category;
        let icon = category_icon(cat);
        let color = category_color(cat);

        // Count ADRs in this category
        let count = all_adrs.iter().filter(|adr| adr.category == *cat).count();

        let (style, prefix) = if is_selected {
            (
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED),
                "▶ ",
            )
        } else {
            (Style::default().fg(color), "  ")
        };

        lines.push(Line::from(vec![
            Span::raw(prefix),
            Span::styled(format!("{} ", icon), style),
            Span::styled(cat.label().to_string(), style),
            Span::styled(
                format!(" ({})", count),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }

    // Add navigation hint
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑/↓ navigate ADRs",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Render the ADR list and details on the right.
fn render_adr_content(f: &mut Frame, app: &App, area: Rect) {
    let all_adrs = get_all_adrs();
    let selected_idx = app.nexus.arch_adr_index;

    // Split into ADR list (top 40%) and details (bottom 60%)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    render_adr_list(f, &all_adrs, selected_idx, chunks[0]);
    render_adr_details(f, &all_adrs, selected_idx, chunks[1]);
}

/// Render the ADR list.
fn render_adr_list(f: &mut Frame, adrs: &[AdrEntry], selected_idx: usize, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " ADR LIST ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line<'static>> = Vec::new();

    // Calculate visible range (scrolling)
    let visible_height = inner.height as usize;
    let start = selected_idx.saturating_sub(visible_height / 2);
    let end = (start + visible_height).min(adrs.len());
    let start = if end == adrs.len() {
        end.saturating_sub(visible_height)
    } else {
        start
    };

    for (i, adr) in adrs.iter().enumerate().skip(start).take(end - start) {
        let is_selected = i == selected_idx;
        let color = category_color(&adr.category);
        let icon = category_icon(&adr.category);

        let (style, prefix) = if is_selected {
            (
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED),
                "▶ ",
            )
        } else {
            (Style::default().fg(Color::Rgb(180, 180, 190)), "  ")
        };

        let id_style = if is_selected {
            Style::default()
                .fg(color)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else {
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        };

        lines.push(Line::from(vec![
            Span::raw(prefix),
            Span::styled(format!("{} ", icon), id_style),
            Span::styled(format!("{}: ", adr.id), id_style),
            Span::styled(adr.title.clone(), style),
        ]));
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Render the ADR details panel.
fn render_adr_details(f: &mut Frame, adrs: &[AdrEntry], selected_idx: usize, area: Rect) {
    let adr = match adrs.get(selected_idx) {
        Some(a) => a,
        None => {
            let block = Block::default()
                .title(" DETAILS ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));
            f.render_widget(Paragraph::new("No ADR selected").block(block), area);
            return;
        },
    };

    let color = category_color(&adr.category);

    let block = Block::default()
        .title(Span::styled(
            format!(" {} - {} ", adr.id, adr.title),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line<'static>> = Vec::new();

    // Header info
    lines.push(Line::from(vec![
        Span::styled("Version: ", Style::default().fg(Color::DarkGray)),
        Span::styled(adr.version.clone(), Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
        Span::styled(adr.status.clone(), Style::default().fg(Color::Green)),
    ]));
    lines.push(Line::from(""));

    // Summary
    lines.push(Line::from(Span::styled(
        "SUMMARY",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    for item in &adr.summary {
        lines.push(Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::Yellow)),
            Span::styled(item.clone(), Style::default().fg(Color::White)),
        ]));
    }
    lines.push(Line::from(""));

    // Diagram (if present)
    if !adr.diagram.is_empty() {
        lines.push(Line::from(Span::styled(
            "DIAGRAM",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        for line in &adr.diagram {
            lines.push(Line::from(Span::styled(
                format!("  {}", line),
                Style::default().fg(Color::Rgb(150, 150, 160)),
            )));
        }
        lines.push(Line::from(""));
    }

    // Key rules (if present)
    if !adr.key_rules.is_empty() {
        lines.push(Line::from(Span::styled(
            "KEY RULES",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        for rule in &adr.key_rules {
            lines.push(Line::from(vec![
                Span::styled("  ▸ ", Style::default().fg(Color::Green)),
                Span::styled(rule.clone(), Style::default().fg(Color::White)),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Related classes (if present)
    if !adr.related_classes.is_empty() {
        lines.push(Line::from(Span::styled(
            "RELATED CLASSES",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        let classes_str = adr.related_classes.join(", ");
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(classes_str, Style::default().fg(Color::Magenta)),
        ]));
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Handle key events in the Arch tab.
/// Returns true if the event was handled.
pub fn handle_arch_key(app: &mut App, key: crossterm::event::KeyEvent) -> bool {
    use crossterm::event::KeyCode;

    let adrs = get_all_adrs();
    let max_idx = adrs.len().saturating_sub(1);

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.nexus.arch_adr_index > 0 {
                app.nexus.arch_adr_index -= 1;
            }
            true
        },
        KeyCode::Down | KeyCode::Char('j') => {
            if app.nexus.arch_adr_index < max_idx {
                app.nexus.arch_adr_index += 1;
            }
            true
        },
        KeyCode::Home | KeyCode::Char('g') => {
            app.nexus.arch_adr_index = 0;
            true
        },
        KeyCode::End | KeyCode::Char('G') => {
            app.nexus.arch_adr_index = max_idx;
            true
        },
        _ => false,
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_icons() {
        assert_eq!(category_icon(&AdrCategory::CorePrinciples), "◆");
        assert_eq!(category_icon(&AdrCategory::SchemaArchitecture), "◇");
        assert_eq!(category_icon(&AdrCategory::UxArchitecture), "◈");
        assert_eq!(category_icon(&AdrCategory::ArcPolicies), "→");
        assert_eq!(category_icon(&AdrCategory::LayerEvolution), "▣");
    }

    #[test]
    fn test_get_all_adrs_not_empty() {
        let adrs = get_all_adrs();
        assert!(!adrs.is_empty());
        assert!(adrs.len() >= 10); // We have at least 10 ADRs
    }

    #[test]
    fn test_adrs_have_categories() {
        let adrs = get_all_adrs();
        for adr in &adrs {
            // Verify each ADR has a valid category
            let _ = adr.category.label();
        }
    }

    #[test]
    fn test_core_adrs_present() {
        let adrs = get_all_adrs();
        let ids: Vec<&str> = adrs.iter().map(|a| a.id.as_str()).collect();

        // Core ADRs should be present
        assert!(ids.contains(&"ADR-007")); // Generation NOT Translation
        assert!(ids.contains(&"ADR-003")); // YAML-First
        assert!(ids.contains(&"ADR-028")); // Page-Entity Architecture
    }
}
