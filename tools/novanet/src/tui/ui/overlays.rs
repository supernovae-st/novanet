//! Overlay panels for TUI: search, help, and legend popups.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::super::app::App;
use super::super::data::TreeItem;
use super::hex_to_color;

// Re-use shared styles and constants from parent module
use super::{
    COLOR_OVERLAY_BG, POPUP_BOX_HEIGHT, POPUP_BOX_WIDTH, STYLE_ACCENT, STYLE_DESC, STYLE_DIM,
    STYLE_HIGHLIGHT, STYLE_INFO, STYLE_PRIMARY,
};

/// Search overlay: fuzzy search with results list.
pub fn render_search(f: &mut Frame, app: &App) {
    // Center the search box
    let area = f.area();
    let width = POPUP_BOX_WIDTH.min(area.width.saturating_sub(4));
    let height = POPUP_BOX_HEIGHT.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 3; // Slightly above center

    let search_area = Rect::new(x, y, width, height);

    // Clear the area behind the overlay
    f.render_widget(Clear, search_area);

    // Build content
    let mut lines: Vec<Line> = Vec::new();

    // Input line with cursor
    lines.push(Line::from(vec![
        Span::styled(" > ", STYLE_INFO),
        Span::styled(&app.search.query, STYLE_PRIMARY),
        Span::styled("█", STYLE_INFO), // Cursor
    ]));

    lines.push(Line::from(""));

    // Results count
    let count_text = if app.search.results.is_empty() {
        if app.search.query.is_empty() {
            "Type to search...".to_string()
        } else {
            "No results".to_string()
        }
    } else {
        format!("{} results", app.search.results.len())
    };
    lines.push(Line::from(Span::styled(count_text, STYLE_DIM)));

    lines.push(Line::from(""));

    // Results list with scroll window around cursor
    let max_visible = 8;
    let total_results = app.search.results.len();

    // Calculate scroll window to keep cursor visible
    let start = if total_results <= max_visible || app.search.cursor < max_visible / 2 {
        0
    } else if app.search.cursor > total_results - max_visible / 2 {
        total_results.saturating_sub(max_visible)
    } else {
        app.search.cursor.saturating_sub(max_visible / 2)
    };

    let visible_results = app.search.results.iter().skip(start).take(max_visible);
    for (i, &idx) in visible_results.enumerate() {
        let actual_idx = start + i;
        let is_selected = actual_idx == app.search.cursor;
        let item = app.tree.item_at(idx);

        let (prefix, name, type_label) = match item {
            Some(TreeItem::KindsSection) => ("", "Node Kinds".to_string(), "Section"),
            Some(TreeItem::ArcsSection) => ("", "Arcs".to_string(), "Section"),
            Some(TreeItem::Realm(r)) => (r.icon, r.display_name.clone(), "Realm"),
            Some(TreeItem::Layer(_, l)) => ("  ", l.display_name.clone(), "Layer"),
            Some(TreeItem::Kind(_, _, k)) => ("    ", k.display_name.clone(), "Node Kind"),
            Some(TreeItem::ArcFamily(f)) => ("  ", f.display_name.clone(), "ArcFamily"),
            Some(TreeItem::ArcKind(_, ek)) => ("    ", ek.display_name.clone(), "Arc Kind"),
            Some(TreeItem::Instance(_, _, _, inst)) => {
                ("      ", inst.display_name.clone(), "Instance")
            }
            None => ("?", "Unknown".to_string(), ""),
        };

        let style = if is_selected {
            Style::default().bg(Color::Rgb(30, 50, 70)).fg(Color::White)
        } else {
            STYLE_DESC
        };

        let type_style = if is_selected {
            Style::default()
                .bg(Color::Rgb(30, 50, 70))
                .fg(Color::DarkGray)
        } else {
            STYLE_DIM
        };

        lines.push(Line::from(vec![
            Span::styled(format!(" {}{}", prefix, name), style),
            Span::styled(format!("  {}", type_label), type_style),
        ]));
    }

    let block = Block::default()
        .title(Span::styled(" Search ", STYLE_INFO))
        .borders(Borders::ALL)
        .border_style(STYLE_INFO)
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, search_area);
}

/// Help overlay: keyboard shortcuts.
pub fn render_help(f: &mut Frame) {
    let area = f.area();
    let width = POPUP_BOX_WIDTH.min(area.width.saturating_sub(4));
    let height = 32.min(area.height.saturating_sub(4)); // Taller for many shortcuts
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let help_area = Rect::new(x, y, width, height);
    f.render_widget(Clear, help_area);

    let lines = vec![
        Line::from(Span::styled(
            " NovaNet TUI — Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![Span::styled("  Navigation", STYLE_HIGHLIGHT)]),
        Line::from(vec![
            Span::styled("    Tab      ", STYLE_PRIMARY),
            Span::styled("Cycle: Tree→Info→Graph→YAML", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    ←→       ", STYLE_PRIMARY),
            Span::styled("Quick panel switch", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    j/k ↑↓   ", STYLE_PRIMARY),
            Span::styled("Move cursor / scroll", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Tree (vim-style)", STYLE_HIGHLIGHT)]),
        Line::from(vec![
            Span::styled("    h/l      ", STYLE_PRIMARY),
            Span::styled("Collapse/expand node", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    H/L      ", STYLE_PRIMARY),
            Span::styled("Collapse/expand all", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    g/G      ", STYLE_PRIMARY),
            Span::styled("Jump to first/last", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Graph panel", STYLE_ACCENT)]),
        Line::from(vec![
            Span::styled("    j/k ↑↓   ", STYLE_PRIMARY),
            Span::styled("Select neighbor node", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    h/l ←→   ", STYLE_PRIMARY),
            Span::styled("Navigate incoming/outgoing", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    Enter    ", STYLE_PRIMARY),
            Span::styled("Jump to selected node", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Scrolling", STYLE_HIGHLIGHT)]),
        Line::from(vec![
            Span::styled("    d/u      ", STYLE_PRIMARY),
            Span::styled("Page down/up", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Modes", STYLE_HIGHLIGHT)]),
        Line::from(vec![
            Span::styled("    1-6      ", STYLE_PRIMARY),
            Span::styled("Meta/Data/Overlay/Query/Atlas/Audit", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    N        ", STYLE_PRIMARY),
            Span::styled("Cycle through modes", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Search & Help", STYLE_HIGHLIGHT)]),
        Line::from(vec![
            Span::styled("    /        ", STYLE_PRIMARY),
            Span::styled("Search (vim-style)", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    ?        ", STYLE_PRIMARY),
            Span::styled("Show this help", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    F1       ", STYLE_PRIMARY),
            Span::styled("Color legend", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Actions", STYLE_HIGHLIGHT)]),
        Line::from(vec![
            Span::styled("    r        ", STYLE_PRIMARY),
            Span::styled("Refresh data", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    y        ", STYLE_PRIMARY),
            Span::styled("Yank (copy key)", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    Ctrl+o   ", STYLE_PRIMARY),
            Span::styled("Go back in history", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    Ctrl+i   ", STYLE_PRIMARY),
            Span::styled("Go forward in history", STYLE_DIM),
        ]),
        Line::from(vec![
            Span::styled("    q        ", STYLE_PRIMARY),
            Span::styled("Quit", STYLE_DIM),
        ]),
        Line::from(""),
        Line::from(Span::styled("  Press any key to close", STYLE_DIM)),
    ];

    let block = Block::default()
        .title(Span::styled(" Help ", STYLE_ACCENT))
        .borders(Borders::ALL)
        .border_style(STYLE_ACCENT)
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, help_area);
}

/// Color legend overlay: shows Realm, Layer, and Trait color meanings.
pub fn render_legend(f: &mut Frame, app: &App) {
    let area = f.area();
    let width = 45.min(area.width.saturating_sub(4));
    let height = 24.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let legend_area = Rect::new(x, y, width, height);
    f.render_widget(Clear, legend_area);

    let mut lines = vec![
        Line::from(Span::styled(
            " NovaNet — Color Legend",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Realms (border color)",
            STYLE_HIGHLIGHT,
        )]),
    ];

    // Add realm colors from taxonomy
    for realm in &app.tree.realms {
        let color = hex_to_color(&realm.color);
        lines.push(Line::from(vec![
            Span::styled("    ██ ", Style::default().fg(color)),
            Span::styled(&realm.display_name, Style::default().fg(Color::White)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "  Layers (fill color)",
        STYLE_HIGHLIGHT,
    )]));

    // Add layer colors from first realm (layers are same across realms)
    if let Some(realm) = app.tree.realms.first() {
        for layer in &realm.layers {
            let color = hex_to_color(&layer.color);
            lines.push(Line::from(vec![
                Span::styled("    ██ ", Style::default().fg(color)),
                Span::styled(&layer.display_name, Style::default().fg(Color::White)),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "  Traits (border style)",
        STYLE_HIGHLIGHT,
    )]));
    lines.push(Line::from(vec![
        Span::styled("    ─── ", STYLE_PRIMARY),
        Span::styled("invariant (solid)", STYLE_DIM),
    ]));
    lines.push(Line::from(vec![
        Span::styled("    ╌╌╌ ", STYLE_PRIMARY),
        Span::styled("localized (dashed)", STYLE_DIM),
    ]));
    lines.push(Line::from(vec![
        Span::styled("    ═══ ", STYLE_PRIMARY),
        Span::styled("knowledge (double)", STYLE_DIM),
    ]));
    lines.push(Line::from(vec![
        Span::styled("    ··· ", STYLE_PRIMARY),
        Span::styled("derived (dotted)", STYLE_DIM),
    ]));
    lines.push(Line::from(vec![
        Span::styled("    ─ ─ ", STYLE_PRIMARY),
        Span::styled("job (thin)", STYLE_DIM),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Press any key to close",
        STYLE_DIM,
    )));

    let block = Block::default()
        .title(Span::styled(" Legend ", STYLE_ACCENT))
        .borders(Borders::ALL)
        .border_style(STYLE_ACCENT)
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, legend_area);
}
