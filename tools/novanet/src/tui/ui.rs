//! UI rendering: layout + widgets for the TUI.
//!
//! Three-pane layout: [Tree | Detail | Status Bar].
//! Uses ratatui for terminal rendering.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use crate::tui::app::{ActivePanel, AppState, NavMode};
use crate::tui::tree::{TaxonomyTree, TreeNodeType};

/// Mode tab colors.
fn mode_color(mode: NavMode, active: bool) -> Color {
    if !active {
        return Color::DarkGray;
    }
    match mode {
        NavMode::Data => Color::Cyan,
        NavMode::Meta => Color::Magenta,
        NavMode::Overlay => Color::Yellow,
        NavMode::Query => Color::Green,
    }
}

/// Render the full UI frame.
pub fn render(frame: &mut Frame, state: &AppState) {
    match state {
        AppState::Loading { message } => render_loading(frame, message),
        AppState::Ready {
            mode,
            tree,
            active_panel,
            detail_lines,
            status,
            facets,
            node_count,
        } => render_ready(
            frame,
            *mode,
            tree,
            *active_panel,
            detail_lines,
            status,
            facets.show_popup,
            *node_count,
        ),
    }
}

fn render_loading(frame: &mut Frame, message: &str) {
    let area = frame.area();
    let block = Block::default()
        .title(" NovaNet ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "NovaNet Context Graph",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(message, Style::default().fg(Color::DarkGray))),
    ])
    .block(block)
    .centered();

    frame.render_widget(paragraph, area);
}

#[allow(clippy::too_many_arguments)]
fn render_ready(
    frame: &mut Frame,
    mode: NavMode,
    tree: &TaxonomyTree,
    active_panel: ActivePanel,
    detail_lines: &[String],
    status: &str,
    show_facet_popup: bool,
    node_count: usize,
) {
    let area = frame.area();

    // Vertical: [mode tabs | main content | status bar]
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // mode tabs
            Constraint::Min(5),    // main content
            Constraint::Length(1), // status bar
        ])
        .split(area);

    render_mode_tabs(frame, vertical[0], mode);
    render_main_content(frame, vertical[1], mode, tree, active_panel, detail_lines);
    render_status_bar(frame, vertical[2], status, node_count);

    if show_facet_popup {
        render_facet_popup(frame, area);
    }
}

fn render_mode_tabs(frame: &mut Frame, area: Rect, current_mode: NavMode) {
    let modes = [
        NavMode::Data,
        NavMode::Meta,
        NavMode::Overlay,
        NavMode::Query,
    ];
    let spans: Vec<Span> = modes
        .iter()
        .enumerate()
        .flat_map(|(i, &m)| {
            let active = m == current_mode;
            let style = Style::default()
                .fg(mode_color(m, active))
                .add_modifier(if active {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                });
            let mut spans = vec![Span::styled(format!(" {} {} ", i + 1, m.label()), style)];
            if i < 3 {
                spans.push(Span::styled(" │ ", Style::default().fg(Color::DarkGray)));
            }
            spans
        })
        .collect();

    let tabs = Paragraph::new(Line::from(spans));
    frame.render_widget(tabs, area);
}

fn render_main_content(
    frame: &mut Frame,
    area: Rect,
    _mode: NavMode,
    tree: &TaxonomyTree,
    active_panel: ActivePanel,
    detail_lines: &[String],
) {
    // Horizontal: [tree | detail]
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    render_tree_panel(
        frame,
        horizontal[0],
        tree,
        active_panel == ActivePanel::Tree,
    );
    render_detail_panel(
        frame,
        horizontal[1],
        detail_lines,
        active_panel == ActivePanel::Detail,
    );
}

fn render_tree_panel(frame: &mut Frame, area: Rect, tree: &TaxonomyTree, active: bool) {
    let border_color = if active { Color::Cyan } else { Color::DarkGray };
    let block = Block::default()
        .title(" Taxonomy ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let items: Vec<ListItem> = tree
        .visible_items()
        .iter()
        .enumerate()
        .map(|(i, (depth, node))| {
            let indent = "  ".repeat(*depth);
            let icon = match node.node_type {
                TreeNodeType::Realm => {
                    if node.expanded {
                        "v "
                    } else {
                        "> "
                    }
                }
                TreeNodeType::Layer => {
                    if node.expanded {
                        "v "
                    } else {
                        "> "
                    }
                }
                TreeNodeType::Kind => "  ",
            };
            let label = format!("{indent}{icon}{}", node.display_name);

            let style = if i == tree.cursor {
                Style::default()
                    .fg(match node.node_type {
                        TreeNodeType::Realm => Color::Cyan,
                        TreeNodeType::Layer => Color::Yellow,
                        TreeNodeType::Kind => Color::White,
                    })
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(match node.node_type {
                    TreeNodeType::Realm => Color::Cyan,
                    TreeNodeType::Layer => Color::Yellow,
                    TreeNodeType::Kind => Color::White,
                })
            };

            ListItem::new(Span::styled(label, style))
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn render_detail_panel(frame: &mut Frame, area: Rect, lines: &[String], active: bool) {
    let border_color = if active { Color::Cyan } else { Color::DarkGray };
    let block = Block::default()
        .title(" Detail ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let text: Vec<Line> = lines
        .iter()
        .map(|l| Line::from(Span::raw(l.as_str())))
        .collect();

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, status: &str, node_count: usize) {
    let left = Span::styled(format!(" {status} "), Style::default().fg(Color::White));
    let right = Span::styled(
        format!(" {node_count} nodes | q:quit ?:help "),
        Style::default().fg(Color::DarkGray),
    );

    let line =
        Line::from(vec![
            left,
            Span::raw(" ".repeat(
                area.width.saturating_sub(
                    status.len() as u16 + 2 + node_count.to_string().len() as u16 + 22,
                ) as usize,
            )),
            right,
        ]);

    let bar = Paragraph::new(line).style(Style::default().bg(Color::DarkGray).fg(Color::White));
    frame.render_widget(bar, area);
}

fn render_facet_popup(frame: &mut Frame, area: Rect) {
    let popup_width = 50.min(area.width.saturating_sub(4));
    let popup_height = 15.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    let block = Block::default()
        .title(" Facet Filters (Esc to close) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Realms: global, project, shared",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Layers: knowledge, structure, ...",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Traits: invariant, localized, ...",
            Style::default().fg(Color::Magenta),
        )),
        Line::from(""),
        Line::from(Span::raw("  (Interactive selection: Phase 7B)")),
    ];

    // Clear area behind popup
    let clear = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(clear, popup_area);

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, popup_area);
}
