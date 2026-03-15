//! Test utilities for TUI snapshot testing and data construction.
//!
//! Uses ratatui's TestBackend for headless rendering.
//! Provides shared test factories for `ClassInfo`, `LayerInfo`, `RealmInfo`,
//! and `TaxonomyTree` to eliminate duplication across test modules.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::tui::data::{ClassInfo, LayerInfo, RealmInfo, TaxonomyTree};
use crate::tui::theme::{ColorMode, Theme};
use rustc_hash::FxHashMap;

/// Render a widget to a buffer for testing.
pub fn render_widget<W: Widget>(widget: W, width: u16, height: u16) -> Buffer {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            frame.render_widget(widget, Rect::new(0, 0, width, height));
        })
        .unwrap();
    terminal.backend().buffer().clone()
}

/// Convert a Buffer to a string for snapshot testing.
pub fn buffer_to_string(buffer: &Buffer) -> String {
    let mut output = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            let cell = buffer.cell((x, y)).unwrap();
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    output
}

/// Render a widget and return as snapshot-ready string.
pub fn render_to_snapshot<W: Widget>(widget: W, width: u16, height: u16) -> String {
    let buffer = render_widget(widget, width, height);
    buffer_to_string(&buffer)
}

// =============================================================================
// DATA CONSTRUCTION FACTORIES
// =============================================================================

/// Create a `Theme` for tests (TrueColor mode).
pub fn create_test_theme() -> Theme {
    Theme::with_mode(ColorMode::TrueColor)
}

/// Create a minimal `ClassInfo` with the given key.
pub fn create_test_class(key: &str) -> ClassInfo {
    ClassInfo {
        key: key.to_string(),
        display_name: key.to_string(),
        description: String::new(),
        icon: String::new(),
        instance_count: 0,
        arcs: Vec::new(),
        yaml_path: String::new(),
        properties: Vec::new(),
        required_properties: Vec::new(),
        schema_hint: String::new(),
        context_budget: String::new(),
        knowledge_tier: None,
        health_percent: None,
        issues_count: None,
    }
}

/// Create a minimal `LayerInfo` with the given key and classes.
pub fn create_test_layer(key: &str, classes: Vec<ClassInfo>) -> LayerInfo {
    LayerInfo {
        key: key.to_string(),
        display_name: key.to_string(),
        color: "#ffffff".to_string(),
        classes,
        content: String::new(),
    }
}

/// Create a minimal `RealmInfo` with the given key and layers.
pub fn create_test_realm(key: &str, layers: Vec<LayerInfo>) -> RealmInfo {
    RealmInfo {
        key: key.to_string(),
        display_name: key.to_string(),
        color: "#ffffff".to_string(),
        icon: "\u{25cb}",
        layers,
        content: String::new(),
    }
}

/// Create an empty `TaxonomyTree`.
pub fn create_empty_tree() -> TaxonomyTree {
    TaxonomyTree {
        realms: Vec::new(),
        arc_families: Vec::new(),
        stats: Default::default(),
        collapsed: Default::default(),
        instances: Default::default(),
        instance_totals: Default::default(),
        class_index: FxHashMap::default(),
        entity_categories: Vec::new(),
        entity_category_instances: Default::default(),
        entity_native_groups: Vec::new(),
        entity_native_by_entity: Default::default(),
    }
}

/// Create a `TaxonomyTree` with the given realms, auto-building the class index.
pub fn create_tree_with_realms(realms: Vec<RealmInfo>) -> TaxonomyTree {
    let mut class_index = FxHashMap::default();
    for (r_idx, realm) in realms.iter().enumerate() {
        for (l_idx, layer) in realm.layers.iter().enumerate() {
            for (k_idx, class_info) in layer.classes.iter().enumerate() {
                class_index.insert(class_info.key.clone(), (r_idx, l_idx, k_idx));
            }
        }
    }

    TaxonomyTree {
        realms,
        arc_families: Vec::new(),
        stats: Default::default(),
        collapsed: Default::default(),
        instances: Default::default(),
        instance_totals: Default::default(),
        class_index,
        entity_categories: Vec::new(),
        entity_category_instances: Default::default(),
        entity_native_groups: Vec::new(),
        entity_native_by_entity: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_widget_returns_buffer() {
        let widget = ratatui::widgets::Paragraph::new("Hello");
        let buffer = render_widget(widget, 10, 1);
        assert_eq!(buffer.area.width, 10);
        assert_eq!(buffer.area.height, 1);
    }

    #[test]
    fn test_buffer_to_string_simple() {
        let widget = ratatui::widgets::Paragraph::new("AB");
        let buffer = render_widget(widget, 5, 1);
        let output = buffer_to_string(&buffer);
        assert!(output.contains("AB"));
    }

    #[test]
    fn test_render_to_snapshot() {
        let widget = ratatui::widgets::Paragraph::new("Test");
        let output = render_to_snapshot(widget, 10, 1);
        assert!(output.contains("Test"));
        assert!(output.ends_with('\n'));
    }
}
