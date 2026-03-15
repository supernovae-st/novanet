//! Breadcrumb rendering for tree panel (v11.6).
//!
//! Shows hierarchical position as: Realm → Layer → Class → Instance.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::tui::app::App;
use crate::tui::palette;
use crate::tui::theme::hex_to_color;
use crate::tui::ui::{COLOR_INSTANCE, layer_badge_icon, realm_badge_icon};

/// A single level in the breadcrumb path.
struct BreadcrumbLevel {
    icon: &'static str,
    label: String,
    color: Color,
}

/// Build breadcrumb path from current selection.
/// Returns a vector of levels from root to current item.
fn build_breadcrumb_path(app: &App) -> Vec<BreadcrumbLevel> {
    use crate::tui::data::TreeItem;

    let mut path = Vec::new();

    match app.current_item() {
        Some(TreeItem::Realm(r)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
        },
        Some(TreeItem::Layer(r, l)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
        },
        Some(TreeItem::Class(r, l, k)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            let class_label = if app.is_graph_mode() && k.instance_count > 0 {
                format!("{} ({})", k.display_name, k.instance_count)
            } else {
                k.display_name.clone()
            };
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: class_label,
                color: hex_to_color(&l.color),
            });
        },
        Some(TreeItem::EntityCategory(r, l, k, cat)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: k.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: "◫",
                label: cat.display_name.clone(),
                color: Color::Gray,
            });
        },
        Some(TreeItem::LocaleGroup(r, l, k, group)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: k.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: "🌐",
                label: format!(
                    "{} {} ({})",
                    group.flag, group.locale_code, group.locale_name
                ),
                color: Color::Cyan,
            });
        },
        Some(TreeItem::EntityGroup(r, l, k, group)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: k.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: "◈",
                label: group.entity_display_name.clone(),
                color: Color::Yellow,
            });
        },
        Some(TreeItem::Instance(r, l, k, inst)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: k.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: "►",
                label: inst.display_name.clone(),
                color: COLOR_INSTANCE,
            });
        },
        Some(TreeItem::ArcFamily(f)) => {
            path.push(BreadcrumbLevel {
                icon: "⊶",
                label: "Arcs".to_string(),
                color: Color::Magenta,
            });
            path.push(BreadcrumbLevel {
                icon: "◇",
                label: f.display_name.clone(),
                color: Color::Magenta,
            });
        },
        Some(TreeItem::ArcClass(f, ak)) => {
            path.push(BreadcrumbLevel {
                icon: "⊶",
                label: "Arcs".to_string(),
                color: Color::Magenta,
            });
            path.push(BreadcrumbLevel {
                icon: "◇",
                label: f.display_name.clone(),
                color: Color::Magenta,
            });
            path.push(BreadcrumbLevel {
                icon: "→",
                label: ak.display_name.clone(),
                color: Color::White,
            });
        },
        Some(TreeItem::ClassesSection) => {
            path.push(BreadcrumbLevel {
                icon: "◈",
                label: "Node Classes".to_string(),
                color: Color::Cyan,
            });
        },
        Some(TreeItem::ArcsSection) => {
            path.push(BreadcrumbLevel {
                icon: "⊶",
                label: "Arcs".to_string(),
                color: Color::Magenta,
            });
        },
        Some(TreeItem::EntityNativeItem(r, l, k, native)) => {
            path.push(BreadcrumbLevel {
                icon: realm_badge_icon(&r.key),
                label: r.display_name.clone(),
                color: hex_to_color(&r.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: l.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: layer_badge_icon(&l.key),
                label: k.display_name.clone(),
                color: hex_to_color(&l.color),
            });
            path.push(BreadcrumbLevel {
                icon: "◆",
                label: native.display_name.clone(),
                color: COLOR_INSTANCE,
            });
        },
        None => {},
    }

    path
}

/// Render sticky breadcrumb at top of tree panel.
/// Returns the height used (always 1 line for consistent layout).
pub fn render_breadcrumb(f: &mut Frame, area: Rect, app: &App) -> u16 {
    let path = build_breadcrumb_path(app);

    // Always render 1 line for consistent header height
    let breadcrumb_area = Rect::new(area.x, area.y, area.width, 1);

    if path.is_empty() {
        let line = Line::from(Span::styled(
            " ◇ Select an item",
            Style::default().fg(palette::HINT_TEXT),
        ));
        let paragraph = Paragraph::new(line).style(Style::default().bg(palette::BG_EMPTY));
        f.render_widget(paragraph, breadcrumb_area);
        return 1;
    }

    // Build horizontal breadcrumb: ◎ Org → ⚙ Config → ■ Class
    let mut spans: Vec<Span> = Vec::with_capacity(path.len() * 3);
    spans.push(Span::raw(" "));

    for (i, level) in path.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(
                " → ",
                Style::default().fg(palette::MUTED),
            ));
        }
        spans.push(Span::styled(
            format!("{} ", level.icon),
            Style::default().fg(level.color),
        ));
        let label_style = if i == path.len() - 1 {
            Style::default()
                .fg(level.color)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(level.color)
        };
        spans.push(Span::styled(level.label.clone(), label_style));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(palette::BG_EMPTY));
    f.render_widget(paragraph, breadcrumb_area);

    1
}
