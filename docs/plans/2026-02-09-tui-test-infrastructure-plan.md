# TUI Test Infrastructure Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reach 80% test coverage for TUI modules using TestBackend + insta snapshots

**Architecture:** Headless rendering with ratatui::TestBackend, snapshot testing with insta, property testing with proptest

**Tech Stack:** ratatui (TestBackend), insta (snapshots), proptest (property-based)

---

## Current State

| Metric | Current | Target |
|--------|---------|--------|
| TUI tests | ~282 | ~400 |
| Line coverage | 52% | 80% |
| Modules with tests | 15/25 | 25/25 |

**Well-covered:** app.rs (75 tests), unicode.rs (48), guide/ (91 tests)
**Zero coverage:** ui/graph.rs, ui/atlas.rs, ui/info.rs, ui/overlays.rs, atlas/

---

## Task 1: Create Testing Module

### Task 1.1: Create testing.rs skeleton

**Files:**
- Create: `src/tui/testing.rs`

**Step 1: Write the failing test**

```rust
// In src/tui/testing.rs
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
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test tui::testing::tests::test_render_widget_returns_buffer`
Expected: FAIL with "cannot find function `render_widget`"

**Step 3: Write minimal implementation**

```rust
//! Test utilities for TUI snapshot testing.
//!
//! Uses ratatui's TestBackend for headless rendering.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

/// Render a widget to a buffer for testing.
pub fn render_widget<W: Widget>(widget: W, width: u16, height: u16) -> Buffer {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| {
        frame.render_widget(widget, Rect::new(0, 0, width, height));
    }).unwrap();
    terminal.backend().buffer().clone()
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
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test tui::testing::tests::test_render_widget_returns_buffer`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/testing.rs
git commit -m "test(tui): create testing module with render_widget helper

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.2: Add buffer_to_string function

**Files:**
- Modify: `src/tui/testing.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_buffer_to_string_simple() {
    let widget = ratatui::widgets::Paragraph::new("AB");
    let buffer = render_widget(widget, 5, 1);
    let output = buffer_to_string(&buffer);
    assert!(output.contains("AB"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test tui::testing::tests::test_buffer_to_string_simple`
Expected: FAIL with "cannot find function `buffer_to_string`"

**Step 3: Write minimal implementation**

```rust
/// Convert a Buffer to a string for snapshot testing.
pub fn buffer_to_string(buffer: &Buffer) -> String {
    let mut output = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            let cell = buffer.get(x, y);
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    output
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test tui::testing::tests::test_buffer_to_string_simple`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/testing.rs
git commit -m "test(tui): add buffer_to_string for snapshot output

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.3: Add render_to_snapshot convenience function

**Files:**
- Modify: `src/tui/testing.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_render_to_snapshot() {
    let widget = ratatui::widgets::Paragraph::new("Test");
    let output = render_to_snapshot(widget, 10, 1);
    assert!(output.contains("Test"));
    assert!(output.ends_with('\n'));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test tui::testing::tests::test_render_to_snapshot`
Expected: FAIL with "cannot find function `render_to_snapshot`"

**Step 3: Write minimal implementation**

```rust
/// Render a widget and return as snapshot-ready string.
pub fn render_to_snapshot<W: Widget>(widget: W, width: u16, height: u16) -> String {
    let buffer = render_widget(widget, width, height);
    buffer_to_string(&buffer)
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test tui::testing::tests::test_render_to_snapshot`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/testing.rs
git commit -m "test(tui): add render_to_snapshot convenience function

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.4: Export testing module

**Files:**
- Modify: `src/tui/mod.rs:23-32`

**Step 1: Write the failing test**

```rust
// In any test that tries to use testing module
use crate::tui::testing::render_to_snapshot;
```

**Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL with "unresolved import `crate::tui::testing`"

**Step 3: Write minimal implementation**

In `src/tui/mod.rs`, add after line 32 (after `mod yaml;`):

```rust
#[cfg(test)]
pub mod testing;
```

**Step 4: Run test to verify it passes**

Run: `cargo test tui::testing`
Expected: PASS (3 tests)

**Step 5: Commit**

```bash
git add src/tui/mod.rs src/tui/testing.rs
git commit -m "test(tui): export testing module for TUI snapshot tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: Theme Color Tests

### Task 2.1: Add realm color resolution tests

**Files:**
- Modify: `src/tui/theme.rs:742+`

**Step 1: Write the failing test**

```rust
#[test]
fn test_realm_color_global_truecolor() {
    let color = realm::color("global", ColorMode::TrueColor);
    assert!(matches!(color, Color::Rgb(..)));
}

#[test]
fn test_realm_color_tenant_truecolor() {
    let color = realm::color("tenant", ColorMode::TrueColor);
    assert!(matches!(color, Color::Rgb(..)));
}

#[test]
fn test_realm_color_unknown_returns_white() {
    let color = realm::color("unknown", ColorMode::TrueColor);
    assert_eq!(color, Color::White);
}
```

**Step 2: Run tests to verify they pass**

Run: `cargo test tui::theme::tests::test_realm_color`
Expected: PASS (these should work with existing code)

**Step 3: Commit**

```bash
git add src/tui/theme.rs
git commit -m "test(tui): add realm color resolution tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2.2: Add layer color resolution tests

**Files:**
- Modify: `src/tui/theme.rs:742+`

**Step 1: Write the tests**

```rust
#[test]
fn test_layer_color_all_layers_truecolor() {
    let layers = [
        "config", "locale-knowledge", "seo", "foundation",
        "structure", "semantic", "instruction", "output"
    ];
    for layer_key in layers {
        let color = layer::color(layer_key, ColorMode::TrueColor);
        assert!(
            matches!(color, Color::Rgb(..)),
            "Layer {} should have RGB color",
            layer_key
        );
    }
}

#[test]
fn test_layer_color_256_mode() {
    let color = layer::color("config", ColorMode::Color256);
    assert!(matches!(color, Color::Indexed(..)));
}

#[test]
fn test_layer_color_16_mode() {
    let color = layer::color("config", ColorMode::Color16);
    assert!(!matches!(color, Color::Rgb(..))); // Should be basic color
}
```

**Step 2: Run tests**

Run: `cargo test tui::theme::tests::test_layer_color`
Expected: PASS

**Step 3: Commit**

```bash
git add src/tui/theme.rs
git commit -m "test(tui): add layer color resolution tests for all color modes

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2.3: Add trait border style tests

**Files:**
- Modify: `src/tui/theme.rs:742+`

**Step 1: Write the tests**

```rust
#[test]
fn test_trait_border_invariant() {
    let border = trait_mod::border("invariant");
    assert_eq!(border, "solid");
}

#[test]
fn test_trait_border_localized() {
    let border = trait_mod::border("localized");
    assert_eq!(border, "dashed");
}

#[test]
fn test_trait_border_knowledge() {
    let border = trait_mod::border("knowledge");
    assert_eq!(border, "double");
}

#[test]
fn test_trait_border_derived() {
    let border = trait_mod::border("derived");
    assert_eq!(border, "dotted");
}

#[test]
fn test_trait_border_job() {
    let border = trait_mod::border("job");
    assert_eq!(border, "none");
}
```

**Step 2: Run tests**

Run: `cargo test tui::theme::tests::test_trait_border`
Expected: PASS

**Step 3: Commit**

```bash
git add src/tui/theme.rs
git commit -m "test(tui): add trait border style tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2.4: Add arc family color tests

**Files:**
- Modify: `src/tui/theme.rs:742+`

**Step 1: Write the tests**

```rust
#[test]
fn test_arc_family_color_all_families() {
    let families = ["ownership", "localization", "semantic", "generation", "mining"];
    for family in families {
        let color = arc_family::color(family, ColorMode::TrueColor);
        assert!(
            matches!(color, Color::Rgb(..)),
            "ArcFamily {} should have RGB color",
            family
        );
    }
}

#[test]
fn test_arc_family_color_unknown() {
    let color = arc_family::color("unknown", ColorMode::TrueColor);
    assert_eq!(color, Color::White);
}
```

**Step 2: Run tests**

Run: `cargo test tui::theme::tests::test_arc_family`
Expected: PASS

**Step 3: Commit**

```bash
git add src/tui/theme.rs
git commit -m "test(tui): add arc family color tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: Data Module Tests

### Task 3.1: Add TaxonomyTree mock helper

**Files:**
- Modify: `src/tui/data.rs:2288+`

**Step 1: Write the failing test**

```rust
#[test]
fn test_mock_tree_has_realms() {
    let tree = TaxonomyTree::mock_for_testing();
    assert!(!tree.realms.is_empty());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test tui::data::tests::test_mock_tree`
Expected: FAIL with "no method named `mock_for_testing`"

**Step 3: Write minimal implementation**

Add to `impl TaxonomyTree`:

```rust
#[cfg(test)]
pub fn mock_for_testing() -> Self {
    Self {
        realms: vec![
            RealmNode {
                key: "global".to_string(),
                display_name: "Global".to_string(),
                color: "#2aa198".to_string(),
                layers: vec![
                    LayerNode {
                        key: "config".to_string(),
                        display_name: "Config".to_string(),
                        color: "#64748b".to_string(),
                        kinds: vec![
                            KindNode {
                                key: "AppConfig".to_string(),
                                display_name: "App Config".to_string(),
                                node_trait: "invariant".to_string(),
                                description: Some("Application config".to_string()),
                                layer_key: "config".to_string(),
                                realm_key: "global".to_string(),
                                llm_context: None,
                                instances: vec![],
                                total_instance_count: 0,
                                arcs: vec![],
                            }
                        ],
                    },
                ],
            },
            RealmNode {
                key: "tenant".to_string(),
                display_name: "Tenant".to_string(),
                color: "#6c71c4".to_string(),
                layers: vec![
                    LayerNode {
                        key: "foundation".to_string(),
                        display_name: "Foundation".to_string(),
                        color: "#3b82f6".to_string(),
                        kinds: vec![
                            KindNode {
                                key: "Entity".to_string(),
                                display_name: "Entity".to_string(),
                                node_trait: "invariant".to_string(),
                                description: Some("Base entity".to_string()),
                                layer_key: "foundation".to_string(),
                                realm_key: "tenant".to_string(),
                                llm_context: None,
                                instances: vec![],
                                total_instance_count: 0,
                                arcs: vec![],
                            }
                        ],
                    },
                ],
            },
        ],
        arc_families: vec![],
        stats: TreeStats::default(),
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test tui::data::tests::test_mock_tree`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/data.rs
git commit -m "test(tui): add TaxonomyTree::mock_for_testing helper

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 3.2: Add tree structure navigation tests

**Files:**
- Modify: `src/tui/data.rs:2288+`

**Step 1: Write the tests**

```rust
#[test]
fn test_mock_tree_has_two_realms() {
    let tree = TaxonomyTree::mock_for_testing();
    assert_eq!(tree.realms.len(), 2);
}

#[test]
fn test_mock_tree_global_realm() {
    let tree = TaxonomyTree::mock_for_testing();
    let global = tree.realms.iter().find(|r| r.key == "global");
    assert!(global.is_some());
}

#[test]
fn test_mock_tree_tenant_realm() {
    let tree = TaxonomyTree::mock_for_testing();
    let tenant = tree.realms.iter().find(|r| r.key == "tenant");
    assert!(tenant.is_some());
}

#[test]
fn test_mock_tree_global_has_config_layer() {
    let tree = TaxonomyTree::mock_for_testing();
    let global = tree.realms.iter().find(|r| r.key == "global").unwrap();
    let config = global.layers.iter().find(|l| l.key == "config");
    assert!(config.is_some());
}
```

**Step 2: Run tests**

Run: `cargo test tui::data::tests::test_mock_tree`
Expected: PASS (4 tests)

**Step 3: Commit**

```bash
git add src/tui/data.rs
git commit -m "test(tui): add tree structure navigation tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 3.3: Add item_count tests

**Files:**
- Modify: `src/tui/data.rs:2288+`

**Step 1: Write the tests**

```rust
#[test]
fn test_item_count_collapsed() {
    let tree = TaxonomyTree::mock_for_testing();
    // When all collapsed, only realm nodes are visible
    assert_eq!(tree.item_count(), 2); // global + tenant
}

#[test]
fn test_toggle_expands_realm() {
    let mut tree = TaxonomyTree::mock_for_testing();
    tree.toggle("global");
    // Now global's layers are visible
    assert!(tree.item_count() > 2);
}

#[test]
fn test_toggle_twice_collapses() {
    let mut tree = TaxonomyTree::mock_for_testing();
    tree.toggle("global");
    let expanded_count = tree.item_count();
    tree.toggle("global");
    assert_eq!(tree.item_count(), 2);
    assert!(tree.item_count() < expanded_count);
}
```

**Step 2: Run tests**

Run: `cargo test tui::data::tests::test_item_count`
Expected: PASS

**Step 3: Commit**

```bash
git add src/tui/data.rs
git commit -m "test(tui): add item_count and toggle tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: Schema Module Tests

### Task 4.1: Add PropertyStatus tests

**Files:**
- Modify: `src/tui/schema.rs:391+`

**Step 1: Write the tests**

```rust
#[test]
fn test_property_status_filled() {
    let prop = MatchedProperty {
        schema: SchemaProperty {
            name: "key".to_string(),
            prop_type: "string".to_string(),
            required: true,
            example: None,
            description: None,
            enum_values: None,
        },
        value: Some("test-value".to_string()),
        status: PropertyStatus::Filled,
    };
    assert_eq!(prop.status, PropertyStatus::Filled);
}

#[test]
fn test_property_status_empty_optional() {
    let prop = MatchedProperty {
        schema: SchemaProperty {
            name: "description".to_string(),
            prop_type: "string".to_string(),
            required: false,
            example: None,
            description: None,
            enum_values: None,
        },
        value: None,
        status: PropertyStatus::EmptyOptional,
    };
    assert_eq!(prop.status, PropertyStatus::EmptyOptional);
}

#[test]
fn test_property_status_missing_required() {
    let prop = MatchedProperty {
        schema: SchemaProperty {
            name: "key".to_string(),
            prop_type: "string".to_string(),
            required: true,
            example: None,
            description: None,
            enum_values: None,
        },
        value: None,
        status: PropertyStatus::MissingRequired,
    };
    assert_eq!(prop.status, PropertyStatus::MissingRequired);
}
```

**Step 2: Run tests**

Run: `cargo test tui::schema::tests::test_property_status`
Expected: PASS

**Step 3: Commit**

```bash
git add src/tui/schema.rs
git commit -m "test(tui): add PropertyStatus enum tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 4.2: Add CoverageStats tests

**Files:**
- Modify: `src/tui/schema.rs:391+`

**Step 1: Write the tests**

```rust
#[test]
fn test_coverage_stats_all_filled() {
    let props = vec![
        MatchedProperty {
            schema: SchemaProperty {
                name: "a".to_string(),
                prop_type: "string".to_string(),
                required: true,
                example: None,
                description: None,
                enum_values: None,
            },
            value: Some("val".to_string()),
            status: PropertyStatus::Filled,
        },
        MatchedProperty {
            schema: SchemaProperty {
                name: "b".to_string(),
                prop_type: "string".to_string(),
                required: false,
                example: None,
                description: None,
                enum_values: None,
            },
            value: Some("val".to_string()),
            status: PropertyStatus::Filled,
        },
    ];
    let stats = CoverageStats::from_matched(&props);
    assert_eq!(stats.total, 2);
    assert_eq!(stats.filled, 2);
    assert_eq!(stats.missing_required, 0);
    assert_eq!(stats.percent, 100);
}

#[test]
fn test_coverage_stats_partial() {
    let props = vec![
        MatchedProperty {
            schema: SchemaProperty {
                name: "a".to_string(),
                prop_type: "string".to_string(),
                required: true,
                example: None,
                description: None,
                enum_values: None,
            },
            value: Some("val".to_string()),
            status: PropertyStatus::Filled,
        },
        MatchedProperty {
            schema: SchemaProperty {
                name: "b".to_string(),
                prop_type: "string".to_string(),
                required: true,
                example: None,
                description: None,
                enum_values: None,
            },
            value: None,
            status: PropertyStatus::MissingRequired,
        },
    ];
    let stats = CoverageStats::from_matched(&props);
    assert_eq!(stats.total, 2);
    assert_eq!(stats.filled, 1);
    assert_eq!(stats.missing_required, 1);
    assert_eq!(stats.percent, 50);
}

#[test]
fn test_coverage_stats_empty_list() {
    let props: Vec<MatchedProperty> = vec![];
    let stats = CoverageStats::from_matched(&props);
    assert_eq!(stats.total, 0);
    assert_eq!(stats.filled, 0);
    assert_eq!(stats.percent, 100); // Empty = 100% by convention
}
```

**Step 2: Run tests**

Run: `cargo test tui::schema::tests::test_coverage_stats`
Expected: PASS

**Step 3: Commit**

```bash
git add src/tui/schema.rs
git commit -m "test(tui): add CoverageStats calculation tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Snapshot Tests for UI Rendering

### Task 5.1: Add first insta snapshot test for tree panel

**Files:**
- Modify: `src/tui/ui/tree.rs`

**Step 1: Write the snapshot test**

Add to test module:

```rust
#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use crate::tui::testing::render_to_snapshot;
    use crate::tui::data::TaxonomyTree;
    use insta::assert_snapshot;

    #[test]
    fn test_highlight_matches_no_match() {
        let spans = highlight_matches_with_bg("Hello World", None, Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content.as_ref(), "Hello World");
    }

    #[test]
    fn test_highlight_matches_empty_positions() {
        let spans = highlight_matches_with_bg("Hello", Some(&[]), Color::White, None);
        assert_eq!(spans.len(), 1);
    }

    #[test]
    fn test_highlight_matches_with_positions() {
        let spans = highlight_matches_with_bg("Hello", Some(&[0, 2]), Color::White, None);
        // Should have multiple spans due to match highlighting
        assert!(spans.len() > 1);
    }
}
```

**Step 2: Run tests**

Run: `cargo test tui::ui::tree::snapshot_tests`
Expected: PASS

**Step 3: Commit**

```bash
git add src/tui/ui/tree.rs
git commit -m "test(tui): add highlight_matches tests for tree panel

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 5.2: Add hex_to_rgb proptest

**Files:**
- Modify: `src/tui/theme.rs:742+`

**Step 1: Write the proptest**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_hex_to_rgb_valid_format(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let result = hex_to_rgb(&hex);
        prop_assert!(result.is_some());
        let (rr, gg, bb) = result.unwrap();
        prop_assert_eq!(rr, r);
        prop_assert_eq!(gg, g);
        prop_assert_eq!(bb, b);
    }

    #[test]
    fn test_hex_to_rgb_without_hash(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let hex = format!("{:02x}{:02x}{:02x}", r, g, b);
        let result = hex_to_rgb(&hex);
        prop_assert!(result.is_some());
    }
}

#[test]
fn test_hex_to_rgb_invalid_length() {
    assert!(hex_to_rgb("#fff").is_none());
    assert!(hex_to_rgb("#fffffff").is_none());
    assert!(hex_to_rgb("").is_none());
}

#[test]
fn test_hex_to_rgb_invalid_chars() {
    assert!(hex_to_rgb("#gggggg").is_none());
    assert!(hex_to_rgb("#zzzzzz").is_none());
}
```

**Step 2: Run tests**

Run: `cargo test tui::theme::tests::test_hex_to_rgb`
Expected: PASS

**Step 3: Commit**

```bash
git add src/tui/theme.rs
git commit -m "test(tui): add proptest for hex_to_rgb color conversion

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 6: validate_cypher_label Tests

### Task 6.1: Add label validation tests

**Files:**
- Modify: `src/tui/data.rs:2288+`

**Step 1: Write the tests**

```rust
#[test]
fn test_validate_cypher_label_valid() {
    assert!(validate_cypher_label("Entity").is_ok());
    assert!(validate_cypher_label("locale-knowledge").is_ok());
    assert!(validate_cypher_label("Page_L10n").is_ok());
}

#[test]
fn test_validate_cypher_label_empty() {
    assert!(validate_cypher_label("").is_err());
}

#[test]
fn test_validate_cypher_label_invalid_chars() {
    assert!(validate_cypher_label("Entity;DROP").is_err());
    assert!(validate_cypher_label("Page'").is_err());
    assert!(validate_cypher_label("Node\"").is_err());
    assert!(validate_cypher_label("Entity{").is_err());
}
```

**Step 2: Run tests**

Run: `cargo test tui::data::tests::test_validate_cypher_label`
Expected: PASS

**Step 3: Commit**

```bash
git add src/tui/data.rs
git commit -m "test(tui): add Cypher label injection prevention tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Summary

| Phase | Tasks | Tests Added | Coverage Impact |
|-------|-------|-------------|-----------------|
| 1: Testing module | 1.1-1.4 | 3 | +2% |
| 2: Theme tests | 2.1-2.4 | 12 | +5% |
| 3: Data tests | 3.1-3.3 | 10 | +4% |
| 4: Schema tests | 4.1-4.2 | 6 | +3% |
| 5: Snapshot tests | 5.1-5.2 | 5 | +4% |
| 6: Security tests | 6.1 | 3 | +2% |

**Total: 39 new tests, ~20% coverage increase**

---

## Execution

After saving this plan, two options:

**1. Subagent-Driven (this session):**
- Use superpowers:subagent-driven-development
- Fresh subagent per task + code review

**2. Parallel Session:**
- Open new session with superpowers:executing-plans
- Batch execution with checkpoints
