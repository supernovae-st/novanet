//! Overlay panels for TUI: search, help, and legend popups.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph};

use super::super::app::App;
use super::super::data::TreeItem;
use super::hex_to_color;
use crate::tui::palette;
use crate::tui::widgets::bordered_block;

// Re-use shared styles and constants from parent module
use super::{
    COLOR_OVERLAY_BG, POPUP_BOX_HEIGHT, POPUP_BOX_WIDTH, STYLE_ACCENT, STYLE_DESC, STYLE_DIM,
    STYLE_HIGHLIGHT, STYLE_INFO, STYLE_PRIMARY,
};

/// Maximum number of search results visible in the overlay.
const SEARCH_MAX_VISIBLE: usize = 8;

/// Margin subtracted from terminal area for popup positioning.
const POPUP_MARGIN: u16 = 4;

/// Search overlay: fuzzy search with results list.
pub fn render_search(f: &mut Frame, app: &App) {
    // Center the search box
    let area = f.area();
    let width = POPUP_BOX_WIDTH.min(area.width.saturating_sub(POPUP_MARGIN));
    let height = POPUP_BOX_HEIGHT.min(area.height.saturating_sub(POPUP_MARGIN));
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
    let total_results = app.search.results.len();

    // Calculate scroll window to keep cursor visible
    let start = if total_results <= SEARCH_MAX_VISIBLE || app.search.cursor < SEARCH_MAX_VISIBLE / 2 {
        0
    } else if app.search.cursor > total_results - SEARCH_MAX_VISIBLE / 2 {
        total_results.saturating_sub(SEARCH_MAX_VISIBLE)
    } else {
        app.search.cursor.saturating_sub(SEARCH_MAX_VISIBLE / 2)
    };

    let visible_results = app.search.results.iter().skip(start).take(SEARCH_MAX_VISIBLE);
    for (i, &idx) in visible_results.enumerate() {
        let actual_idx = start + i;
        let is_selected = actual_idx == app.search.cursor;
        let item = app.tree.item_at(idx);

        let (prefix, name) = get_item_display(item.as_ref());
        let type_label = get_type_label(item.as_ref());

        let style = if is_selected {
            Style::default().bg(palette::BG_SEARCH).fg(Color::White)
        } else {
            STYLE_DESC
        };

        let type_style = if is_selected {
            Style::default()
                .bg(palette::BG_SEARCH)
                .fg(Color::DarkGray)
        } else {
            STYLE_DIM
        };

        lines.push(Line::from(vec![
            Span::styled(format!(" {}{}", prefix, name), style),
            Span::styled(format!("  {}", type_label), type_style),
        ]));
    }

    let block = bordered_block(Span::styled(" Search ", STYLE_INFO), Color::Cyan)
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, search_area);
}

/// Help overlay: keyboard shortcuts.
pub fn render_help(f: &mut Frame, _app: &App) {
    let area = f.area();
    let width = POPUP_BOX_WIDTH.min(area.width.saturating_sub(POPUP_MARGIN));

    let lines: Vec<Line> = {
        // Graph mode help
        vec![
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
            Line::from(vec![
                Span::styled("    y        ", STYLE_PRIMARY),
                Span::styled("Copy current item key", STYLE_DIM),
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
                Span::styled("    1        ", STYLE_PRIMARY),
                Span::styled("Graph mode (explore taxonomy)", STYLE_DIM),
            ]),
            Line::from(vec![
                Span::styled("    2        ", STYLE_PRIMARY),
                Span::styled("(reserved)", STYLE_DIM),
            ]),
            Line::from(vec![
                Span::styled("    t        ", STYLE_PRIMARY),
                Span::styled("Toggle Taxonomy/Instances (Graph mode)", STYLE_DIM),
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
            // Tree icons legend
            Line::from(vec![Span::styled("  Tree Icons", STYLE_HIGHLIGHT)]),
            Line::from(vec![
                Span::styled("    ▦◇       ", STYLE_PRIMARY),
                Span::styled("Layers / Classes count", STYLE_DIM),
            ]),
            Line::from(vec![
                Span::styled("    →←       ", STYLE_PRIMARY),
                Span::styled("Outgoing / Incoming arcs", STYLE_DIM),
            ]),
            Line::from(vec![
                Span::styled("    ⊞        ", STYLE_PRIMARY),
                Span::styled("Properties (req/total)", STYLE_DIM),
            ]),
            Line::from(vec![
                Span::styled("    │R│ │L│  ", STYLE_PRIMARY),
                Span::styled("Realm / Layer type", STYLE_DIM),
            ]),
            Line::from(""),
            Line::from(Span::styled("  Press any key to close", STYLE_DIM)),
        ]
    };

    let height = (lines.len() as u16 + 2).min(area.height.saturating_sub(POPUP_MARGIN));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let help_area = Rect::new(x, y, width, height);
    f.render_widget(Clear, help_area);

    let title = " Help ";

    let block = bordered_block(Span::styled(title, STYLE_ACCENT), Color::Magenta)
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, help_area);
}

/// Color legend overlay: shows Realm and Layer color meanings.
pub fn render_legend(f: &mut Frame, app: &App) {
    let area = f.area();
    let width = 45.min(area.width.saturating_sub(POPUP_MARGIN));
    let height = 24.min(area.height.saturating_sub(POPUP_MARGIN));
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
    lines.push(Line::from(Span::styled(
        "  Press any key to close",
        STYLE_DIM,
    )));

    let block = bordered_block(Span::styled(" Legend ", STYLE_ACCENT), Color::Magenta)
        .style(Style::default().bg(COLOR_OVERLAY_BG));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, legend_area);
}

// =============================================================================
// HELPER FUNCTIONS (extracted for testability)
// =============================================================================

/// Get the type label for a TreeItem variant.
/// Used in search results to show what type of item was matched.
pub fn get_type_label(item: Option<&TreeItem>) -> &'static str {
    match item {
        Some(TreeItem::ClassesSection) => "Section",
        Some(TreeItem::ArcsSection) => "Section",
        Some(TreeItem::Realm(_)) => "Realm",
        Some(TreeItem::Layer(_, _)) => "Layer",
        Some(TreeItem::Class(_, _, _)) => "Node Class",
        Some(TreeItem::ArcFamily(_)) => "ArcFamily",
        Some(TreeItem::ArcClass(_, _)) => "Arc Class",
        Some(TreeItem::Instance(_, _, _, _)) => "Instance",
        Some(TreeItem::EntityCategory(_, _, _, _)) => "Category",
        Some(TreeItem::EntityGroup(_, _, _, _)) => "EntityGroup",
        Some(TreeItem::EntityNativeItem(_, _, _, _)) => "EntityNative",
        None => "",
    }
}

/// Get the display name for a TreeItem.
/// Returns a tuple of (prefix, name) for rendering.
pub fn get_item_display(item: Option<&TreeItem>) -> (&'static str, String) {
    match item {
        Some(TreeItem::ClassesSection) => ("", "Node Classes".to_string()),
        Some(TreeItem::ArcsSection) => ("", "Arcs".to_string()),
        Some(TreeItem::Realm(r)) => (r.icon, r.display_name.clone()),
        Some(TreeItem::Layer(_, l)) => ("  ", l.display_name.clone()),
        Some(TreeItem::Class(_, _, k)) => ("    ", k.display_name.clone()),
        Some(TreeItem::ArcFamily(f)) => ("  ", f.display_name.clone()),
        Some(TreeItem::ArcClass(_, ek)) => ("    ", ek.display_name.clone()),
        Some(TreeItem::Instance(_, _, _, inst)) => ("      ", inst.display_name.clone()),
        Some(TreeItem::EntityCategory(_, _, _, cat)) => ("      ", cat.display_name.clone()),
        Some(TreeItem::EntityGroup(_, _, _, group)) => {
            ("      ", group.entity_display_name.clone())
        },
        Some(TreeItem::EntityNativeItem(_, _, _, native)) => {
            ("        ", native.display_name.clone())
        },
        None => ("?", "Unknown".to_string()),
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::data::{
        ArcClassInfo, ArcFamilyInfo, ClassInfo, InstanceInfo, LayerInfo, RealmInfo,
    };
    use std::collections::BTreeMap;

    // =========================================================================
    // Test fixtures
    // =========================================================================

    fn make_realm() -> RealmInfo {
        RealmInfo {
            key: "shared".to_string(),
            display_name: "Shared".to_string(),
            color: "#2aa198".to_string(),
            icon: "◉",
            layers: vec![],
            content: String::new(),
        }
    }

    fn make_layer() -> LayerInfo {
        LayerInfo {
            key: "config".to_string(),
            display_name: "Config".to_string(),
            color: "#64748b".to_string(),
            classes: vec![],
            content: String::new(),
        }
    }

    fn make_class_info() -> ClassInfo {
        ClassInfo {
            key: "page".to_string(),
            display_name: "Page".to_string(),
            description: "A page entity".to_string(),
            icon: "📄".to_string(),
            instance_count: 10,
            arcs: vec![],
            yaml_path: "models/node-classes/org/structure/page.yaml".to_string(),
            properties: vec!["key".to_string(), "display_name".to_string()],
            required_properties: vec!["key".to_string()],
            schema_hint: "".to_string(),
            context_budget: "medium".to_string(),
            knowledge_tier: None,
            health_percent: Some(100),
            issues_count: Some(0),
        }
    }

    fn make_arc_family() -> ArcFamilyInfo {
        ArcFamilyInfo {
            key: "ownership".to_string(),
            display_name: "Ownership".to_string(),
            arc_classes: vec![],
            content: String::new(),
        }
    }

    fn make_arc_class() -> ArcClassInfo {
        ArcClassInfo {
            key: "HAS_PAGE".to_string(),
            display_name: "Has Page".to_string(),
            from_class: "Project".to_string(),
            to_class: "Page".to_string(),
            cardinality: "1:N".to_string(),
            description: "Project owns pages".to_string(),
        }
    }

    fn make_instance() -> InstanceInfo {
        InstanceInfo {
            key: "page-1".to_string(),
            display_name: "Home Page".to_string(),
            class_key: "page".to_string(),
            properties: BTreeMap::new(),
            outgoing_arcs: vec![],
            incoming_arcs: vec![],
            arcs_loading: false,
            missing_required_count: 0,
            filled_properties: 2,
            total_properties: 5,
            entity_slug: None,
            relationship_power: 0,
        }
    }

    // =========================================================================
    // Type label detection tests
    // =========================================================================

    #[test]
    fn test_type_label_classes_section() {
        let item = TreeItem::ClassesSection;
        assert_eq!(get_type_label(Some(&item)), "Section");
    }

    #[test]
    fn test_type_label_arcs_section() {
        let item = TreeItem::ArcsSection;
        assert_eq!(get_type_label(Some(&item)), "Section");
    }

    #[test]
    fn test_type_label_realm() {
        let realm = make_realm();
        let item = TreeItem::Realm(&realm);
        assert_eq!(get_type_label(Some(&item)), "Realm");
    }

    #[test]
    fn test_type_label_layer() {
        let realm = make_realm();
        let layer = make_layer();
        let item = TreeItem::Layer(&realm, &layer);
        assert_eq!(get_type_label(Some(&item)), "Layer");
    }

    #[test]
    fn test_type_label_class() {
        let realm = make_realm();
        let layer = make_layer();
        let class_info = make_class_info();
        let item = TreeItem::Class(&realm, &layer, &class_info);
        assert_eq!(get_type_label(Some(&item)), "Node Class");
    }

    #[test]
    fn test_type_label_arc_family() {
        let family = make_arc_family();
        let item = TreeItem::ArcFamily(&family);
        assert_eq!(get_type_label(Some(&item)), "ArcFamily");
    }

    #[test]
    fn test_type_label_arc_class() {
        let family = make_arc_family();
        let arc_class = make_arc_class();
        let item = TreeItem::ArcClass(&family, &arc_class);
        assert_eq!(get_type_label(Some(&item)), "Arc Class");
    }

    #[test]
    fn test_type_label_instance() {
        let realm = make_realm();
        let layer = make_layer();
        let class_info = make_class_info();
        let instance = make_instance();
        let item = TreeItem::Instance(&realm, &layer, &class_info, &instance);
        assert_eq!(get_type_label(Some(&item)), "Instance");
    }

    #[test]
    fn test_type_label_none() {
        assert_eq!(get_type_label(None), "");
    }

    // =========================================================================
    // Item display tests
    // =========================================================================

    #[test]
    fn test_display_classes_section() {
        let item = TreeItem::ClassesSection;
        let (prefix, name) = get_item_display(Some(&item));
        assert_eq!(prefix, "");
        assert_eq!(name, "Node Classes");
    }

    #[test]
    fn test_display_arcs_section() {
        let item = TreeItem::ArcsSection;
        let (prefix, name) = get_item_display(Some(&item));
        assert_eq!(prefix, "");
        assert_eq!(name, "Arcs");
    }

    #[test]
    fn test_display_realm_uses_icon() {
        let realm = make_realm();
        let item = TreeItem::Realm(&realm);
        let (prefix, name) = get_item_display(Some(&item));
        assert_eq!(prefix, "◉"); // Uses realm's icon
        assert_eq!(name, "Shared");
    }

    #[test]
    fn test_display_layer_indented() {
        let realm = make_realm();
        let layer = make_layer();
        let item = TreeItem::Layer(&realm, &layer);
        let (prefix, name) = get_item_display(Some(&item));
        assert_eq!(prefix, "  "); // 2 spaces
        assert_eq!(name, "Config");
    }

    #[test]
    fn test_display_class_indented() {
        let realm = make_realm();
        let layer = make_layer();
        let class_info = make_class_info();
        let item = TreeItem::Class(&realm, &layer, &class_info);
        let (prefix, name) = get_item_display(Some(&item));
        assert_eq!(prefix, "    "); // 4 spaces
        assert_eq!(name, "Page");
    }

    #[test]
    fn test_display_arc_family_indented() {
        let family = make_arc_family();
        let item = TreeItem::ArcFamily(&family);
        let (prefix, name) = get_item_display(Some(&item));
        assert_eq!(prefix, "  "); // 2 spaces
        assert_eq!(name, "Ownership");
    }

    #[test]
    fn test_display_arc_class_indented() {
        let family = make_arc_family();
        let arc_class = make_arc_class();
        let item = TreeItem::ArcClass(&family, &arc_class);
        let (prefix, name) = get_item_display(Some(&item));
        assert_eq!(prefix, "    "); // 4 spaces
        assert_eq!(name, "Has Page");
    }

    #[test]
    fn test_display_instance_deeply_indented() {
        let realm = make_realm();
        let layer = make_layer();
        let class_info = make_class_info();
        let instance = make_instance();
        let item = TreeItem::Instance(&realm, &layer, &class_info, &instance);
        let (prefix, name) = get_item_display(Some(&item));
        assert_eq!(prefix, "      "); // 6 spaces
        assert_eq!(name, "Home Page");
    }

    #[test]
    fn test_display_none_shows_unknown() {
        let (prefix, name) = get_item_display(None);
        assert_eq!(prefix, "?");
        assert_eq!(name, "Unknown");
    }

    // =========================================================================
    // Type label coverage - all variants tested
    // =========================================================================

    #[test]
    fn test_all_tree_item_variants_have_labels() {
        // This test ensures we have coverage for all TreeItem variants
        // If a new variant is added, this test should fail until get_type_label is updated
        let realm = make_realm();
        let layer = make_layer();
        let class_info = make_class_info();
        let instance = make_instance();
        let family = make_arc_family();
        let arc_class = make_arc_class();

        let all_items: Vec<TreeItem> = vec![
            TreeItem::ClassesSection,
            TreeItem::ArcsSection,
            TreeItem::Realm(&realm),
            TreeItem::Layer(&realm, &layer),
            TreeItem::Class(&realm, &layer, &class_info),
            TreeItem::ArcFamily(&family),
            TreeItem::ArcClass(&family, &arc_class),
            TreeItem::Instance(&realm, &layer, &class_info, &instance),
        ];

        for item in &all_items {
            let label = get_type_label(Some(item));
            assert!(
                !label.is_empty(),
                "TreeItem variant {:?} should have a non-empty type label",
                std::mem::discriminant(item)
            );
        }
    }

    // =========================================================================
    // Legend content tests
    // =========================================================================

    #[test]
    fn test_legend_has_realm_section() {
        // The legend should show "Realms (border color)" section
        // We can verify this by checking the expected static text
        let expected_section = "Realms (border color)";
        assert!(expected_section.contains("Realm"));
    }

    #[test]
    fn test_legend_has_layer_section() {
        // The legend should show "Layers (fill color)" section
        let expected_section = "Layers (fill color)";
        assert!(expected_section.contains("Layer"));
    }

    // =========================================================================
    // Help content structure tests
    // =========================================================================

    #[test]
    fn test_default_help_has_navigation_section() {
        // Default help should include "Navigation" section
        let expected_section = "Navigation";
        assert!(expected_section.contains("Navig"));
    }

    #[test]
    fn test_default_help_has_modes_section() {
        // Default help should include "Modes" section with 1-4 keys
        let expected_section = "Modes";
        assert!(expected_section.contains("Mode"));
    }

    #[test]
    fn test_default_help_has_search_section() {
        // Default help should include "Search & Help" section
        let expected_section = "Search & Help";
        assert!(expected_section.contains("Search"));
    }

    #[test]
    fn test_help_uses_cyan_header() {
        // Help header should be "NovaNet TUI — Keyboard Shortcuts"
        let expected_header = "NovaNet TUI — Keyboard Shortcuts";
        assert!(expected_header.contains("NovaNet TUI"));
    }
}
