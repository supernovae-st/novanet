//! Filtered Data mode rendering.
//!
//! Shows only instances of a selected Class with breadcrumb navigation.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::app::App;
use crate::tui::theme::hex_to_color;
use crate::tui::ui::{
    COLOR_HIGHLIGHT_BG, COLOR_INSTANCE, STYLE_DIM, STYLE_HIGHLIGHT, STYLE_PRIMARY,
    STYLE_UNFOCUSED, spinner,
};

/// Render filtered Data mode: only instances of a specific Class with breadcrumb.
pub fn render_filtered_instances(
    f: &mut Frame,
    area: Rect,
    app: &App,
    class_key: &str,
    visible_height: usize,
    focused: bool,
    border_color: Color,
) {
    // Get Class info for display with full hierarchy
    let class_info = app.tree.find_class(class_key);
    let (realm_display, realm_color, layer_display, layer_color, class_display) = class_info
        .map(|(realm, layer, class)| {
            (
                realm.display_name.clone(),
                hex_to_color(&realm.color),
                layer.display_name.clone(),
                hex_to_color(&layer.color),
                class.display_name.clone(),
            )
        })
        .unwrap_or_else(|| {
            (
                "Unknown".to_string(),
                Color::White,
                "Unknown".to_string(),
                Color::White,
                class_key.to_string(),
            )
        });

    // Build lines: breadcrumb + instances
    let mut all_lines: Vec<Line> = Vec::new();

    // Breadcrumb header with full hierarchy: Realm → Layer → Class
    all_lines.push(Line::from(vec![
        Span::styled("← ", STYLE_DIM),
        Span::styled("Esc", STYLE_HIGHLIGHT),
        Span::styled(" │ ", STYLE_DIM),
        Span::styled(&realm_display, Style::default().fg(realm_color)),
        Span::styled(" → ", STYLE_DIM),
        Span::styled(&layer_display, Style::default().fg(layer_color)),
        Span::styled(" → ", STYLE_DIM),
        Span::styled(&class_display, STYLE_PRIMARY),
    ]));
    all_lines.push(Line::from(Span::styled(
        "─".repeat(area.width.saturating_sub(2) as usize),
        STYLE_UNFOCUSED,
    )));

    // Get instances and total count
    let instances = app.tree.get_instances(class_key);
    let instance_count = instances.map(|i| i.len()).unwrap_or(0);
    let total_count = app
        .tree
        .get_instance_total(class_key)
        .unwrap_or(instance_count);
    let is_truncated = total_count > instance_count;
    let is_loading = app
        .pending
        .instance
        .as_ref()
        .is_some_and(|k| k == class_key);

    if instance_count == 0 {
        if is_loading {
            all_lines.push(Line::from(Span::styled(
                format!("  {} Loading instances...", spinner(app.tick)),
                STYLE_HIGHLIGHT,
            )));
        } else {
            all_lines.push(Line::from(Span::styled(
                "  No instances exist for this Class",
                STYLE_DIM,
            )));
        }
    } else if let Some(instances) = instances {
        for (idx, instance) in instances.iter().enumerate() {
            let is_cursor = idx == app.tree_cursor;

            let is_primary = instance
                .properties
                .get("is_primary")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let fallback_count = if is_primary {
                instance
                    .incoming_arcs
                    .iter()
                    .filter(|arc| arc.arc_type == "FALLBACK_TO")
                    .count()
            } else {
                0
            };

            let icon = if is_primary { "●" } else { "○" };
            let base_color = COLOR_INSTANCE;

            let style = if is_cursor && focused {
                Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
            } else {
                Style::default().fg(base_color)
            };

            let prefix = if is_cursor { "› " } else { "  " };

            let display = if is_primary && fallback_count > 0 {
                format!(
                    "{}{} {} [{}↓]",
                    prefix, icon, instance.display_name, fallback_count
                )
            } else {
                format!("{}{} {}", prefix, icon, instance.display_name)
            };

            all_lines.push(Line::from(Span::styled(display, style)));
        }
    }

    // Apply scroll
    let lines: Vec<Line> = all_lines
        .into_iter()
        .skip(app.tree_scroll)
        .take(visible_height)
        .collect();

    // Title with Class name and count + position indicator
    let title = if instance_count > 0 {
        if is_truncated {
            format!(
                " {} ({}/{} of {}) ",
                class_display,
                app.tree_cursor + 1,
                instance_count,
                total_count
            )
        } else {
            format!(
                " {} ({}/{}) ",
                class_display,
                app.tree_cursor + 1,
                instance_count
            )
        }
    } else {
        format!(" {} (0) ", class_display)
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(layer_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}
