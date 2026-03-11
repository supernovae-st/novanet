//! Identity + Provenance panel (top center).
//!
//! v0.18.3: New panel showing node identity and provenance info.
//! Combines what was scattered across Header and Info panels.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::app::{App, Focus};
use crate::tui::data::TreeItem;
use crate::tui::theme;

/// Render the Identity + Provenance panel.
pub fn render_identity_panel(f: &mut Frame, area: Rect, app: &App) {
    let is_focused = app.focus == Focus::Identity; // v0.18.3: Dedicated focus state

    let border_color = if is_focused {
        theme::ui::ACCENT
    } else {
        Color::Rgb(60, 60, 70)
    };

    let block = Block::default()
        .title(" Identity & Provenance ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let lines = build_identity_content(app);
    let paragraph = Paragraph::new(lines).block(block);

    f.render_widget(paragraph, area);
}

/// Build identity content based on current selection.
fn build_identity_content(app: &App) -> Vec<Line<'static>> {
    let item = app.tree.item_at(app.tree_cursor);

    match item {
        Some(TreeItem::Class(realm, layer, class_info)) => {
            vec![
                Line::from(vec![
                    Span::styled("Key: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        class_info.key.clone(),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(class_info.display_name.clone()),
                ]),
                Line::from(vec![
                    Span::styled("Realm: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(Color::Green),
                    ),
                    Span::styled(" -> ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Layer: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        layer.display_name.clone(),
                        Style::default().fg(Color::Yellow),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Description: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        truncate(&class_info.description, 60),
                        Style::default().fg(Color::Rgb(150, 150, 150)),
                    ),
                ]),
            ]
        },
        Some(TreeItem::Instance(_, _, class_info, instance)) => {
            vec![
                Line::from(vec![
                    Span::styled("Key: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        instance.key.clone(),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(instance.display_name.clone()),
                ]),
                Line::from(vec![
                    Span::styled("Class: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        class_info.display_name.clone(),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Provenance: ", Style::default().fg(Color::DarkGray)),
                    Span::styled("seed", Style::default().fg(Color::Magenta)), // TODO: dynamic
                ]),
            ]
        },
        Some(TreeItem::EntityNativeItem(_, _, class_info, native)) => {
            vec![
                Line::from(vec![
                    Span::styled("Key: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        native.key.clone(),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(native.display_name.clone()),
                ]),
                Line::from(vec![
                    Span::styled("Class: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        class_info.display_name.clone(),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Entity: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        native.entity_display_name.clone(),
                        Style::default().fg(Color::Green),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Locale: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        native.locale_code.clone(),
                        Style::default().fg(Color::Magenta),
                    ),
                ]),
            ]
        },
        Some(TreeItem::Realm(realm)) => {
            vec![
                Line::from(vec![
                    Span::styled("Realm: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Key: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(realm.key.clone()),
                ]),
            ]
        },
        Some(TreeItem::Layer(realm, layer)) => {
            vec![
                Line::from(vec![
                    Span::styled("Layer: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        layer.display_name.clone(),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Realm: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(Color::Green),
                    ),
                ]),
            ]
        },
        Some(TreeItem::ArcFamily(family)) => {
            vec![
                Line::from(vec![
                    Span::styled("Arc Family: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        family.display_name.clone(),
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Key: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(family.key.clone()),
                ]),
            ]
        },
        Some(TreeItem::ArcClass(family, arc_class)) => {
            vec![
                Line::from(vec![
                    Span::styled("Arc: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        arc_class.display_name.clone(),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Family: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        family.display_name.clone(),
                        Style::default().fg(Color::Magenta),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Pattern: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        arc_class.from_class.clone(),
                        Style::default().fg(Color::Green),
                    ),
                    Span::styled(" -> ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        arc_class.to_class.clone(),
                        Style::default().fg(Color::Yellow),
                    ),
                ]),
            ]
        },
        Some(TreeItem::ClassesSection) => {
            vec![
                Line::from(vec![
                    Span::styled("Section: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        "Node Classes",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Description: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        "Browse all node classes by realm and layer",
                        Style::default().fg(Color::Rgb(150, 150, 150)),
                    ),
                ]),
            ]
        },
        Some(TreeItem::ArcsSection) => {
            vec![
                Line::from(vec![
                    Span::styled("Section: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        "Arc Classes",
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Description: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        "Browse all arc classes by family",
                        Style::default().fg(Color::Rgb(150, 150, 150)),
                    ),
                ]),
            ]
        },
        Some(TreeItem::EntityCategory(_, _, class_info, category)) => {
            vec![
                Line::from(vec![
                    Span::styled("Category: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        category.display_name.clone(),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Class: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        class_info.display_name.clone(),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
            ]
        },
        Some(TreeItem::LocaleGroup(_, _, class_info, group)) => {
            vec![
                Line::from(vec![
                    Span::styled("Locale: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{} {}", group.flag, group.locale_name),
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Class: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        class_info.display_name.clone(),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
            ]
        },
        Some(TreeItem::EntityGroup(_, _, class_info, group)) => {
            vec![
                Line::from(vec![
                    Span::styled("Entity Group: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        group.entity_display_name.clone(),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Class: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        class_info.display_name.clone(),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Entity Key: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(group.entity_key.clone()),
                ]),
            ]
        },
        _ => {
            vec![Line::from(Span::styled(
                "Select an item to see identity",
                Style::default().fg(Color::DarkGray),
            ))]
        },
    }
}

/// Truncate string to max length with ellipsis.
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_short_string() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_long_string() {
        let result = truncate("hello world this is long", 10);
        assert!(result.ends_with("..."));
        assert!(result.len() <= 10);
    }

    #[test]
    fn test_truncate_exact_length() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_empty_string() {
        assert_eq!(truncate("", 10), "");
    }
}
