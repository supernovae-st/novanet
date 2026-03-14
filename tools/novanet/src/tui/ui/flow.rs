//! Flow view renderer — navigable ASCII diagrams.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};

use crate::tui::app::{App, FlowTab};
use crate::tui::flow::{FlowDiagram, data_pipeline, schema_architecture};

/// Render the Flow view into the given frame area.
pub fn render_flow(f: &mut Frame, app: &mut App, area: Rect) {
    // Layout: tabs at top, diagram in center, info at bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Tab bar
            Constraint::Min(10),   // Diagram
            Constraint::Length(5), // Node info
        ])
        .split(area);

    render_tabs(f, app, chunks[0]);

    let diagram = match app.flow.tab {
        FlowTab::Schema => schema_architecture(),
        FlowTab::Pipeline => data_pipeline(),
    };

    // Update total_nodes for navigation
    app.flow.total_nodes = diagram.node_count;

    render_diagram(f, app, &diagram, chunks[1]);
    render_node_info(f, app, &diagram, chunks[2]);
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = [FlowTab::Schema, FlowTab::Pipeline]
        .iter()
        .map(|t| {
            let style = if *t == app.flow.tab {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Line::from(Span::styled(t.label(), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Flow ── [Tab] switch  [j/k] scroll  [n/p] select "),
        )
        .select(match app.flow.tab {
            FlowTab::Schema => 0,
            FlowTab::Pipeline => 1,
        })
        .highlight_style(Style::default().fg(Color::Cyan));

    f.render_widget(tabs, area);
}

fn render_diagram(f: &mut Frame, app: &App, diagram: &FlowDiagram, area: Rect) {
    let inner_height = area.height.saturating_sub(2) as usize;

    let styled_lines: Vec<Line> = diagram
        .lines
        .iter()
        .skip(app.flow.scroll_y)
        .take(inner_height)
        .map(|flow_line| {
            if flow_line.highlights.is_empty() {
                // Plain line
                let text = if app.flow.scroll_x < flow_line.text.len() {
                    &flow_line.text[app.flow.scroll_x..]
                } else {
                    ""
                };
                Line::from(Span::styled(text.to_string(), Style::default().fg(Color::White)))
            } else {
                // Line with highlighted segments
                build_highlighted_line(flow_line, app.flow.selected, app.flow.scroll_x)
            }
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", diagram.title));

    let paragraph = Paragraph::new(styled_lines).block(block);
    f.render_widget(paragraph, area);
}

fn build_highlighted_line(
    flow_line: &crate::tui::flow::FlowLine,
    selected: usize,
    scroll_x: usize,
) -> Line<'static> {
    let text = &flow_line.text;
    let mut spans = Vec::new();
    let mut pos = scroll_x;

    // Sort highlights by start position
    let mut highlights = flow_line.highlights.clone();
    highlights.sort_by_key(|h| h.0);

    for (start, end, node_idx) in &highlights {
        let start = *start;
        let end = *end;
        let node_idx = *node_idx;

        if end <= scroll_x {
            continue; // Entirely scrolled past
        }

        let effective_start = start.max(scroll_x);

        // Add plain text before highlight
        if pos < effective_start && pos < text.len() {
            let plain = &text[pos..effective_start.min(text.len())];
            spans.push(Span::styled(plain.to_string(), Style::default().fg(Color::White)));
        }

        // Add highlighted text
        if effective_start < text.len() {
            let hl_text = &text[effective_start..end.min(text.len())];
            let style = if node_idx == selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            };
            spans.push(Span::styled(hl_text.to_string(), style));
        }

        pos = end;
    }

    // Add remaining text
    if pos < text.len() {
        let remaining = &text[pos..];
        spans.push(Span::styled(remaining.to_string(), Style::default().fg(Color::White)));
    }

    Line::from(spans)
}

fn render_node_info(f: &mut Frame, app: &App, diagram: &FlowDiagram, area: Rect) {
    let (label, desc) = if app.flow.selected < diagram.node_labels.len() {
        (
            diagram.node_labels[app.flow.selected].as_str(),
            diagram.node_descriptions[app.flow.selected].as_str(),
        )
    } else {
        ("", "Navigate with n/p to select nodes")
    };

    let text = vec![
        Line::from(Span::styled(
            format!("  {} ", label),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("  {}", desc),
            Style::default().fg(Color::White),
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Node Info ");

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
