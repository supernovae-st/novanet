# App Struct Refactoring + Markdown Utils Extraction

**Date**: 2026-02-27
**Author**: Claude + Thibaut
**Status**: In Progress

## Executive Summary

Refactor the 55-field App struct into logical sub-structs and extract duplicated markdown parsing utilities.

## Part 1: App Struct Refactoring

### Current State

The `App` struct in `src/tui/app.rs` has **55 fields**, making it hard to maintain.

### Proposed Sub-Structs

| Sub-Struct | Fields | Purpose |
|------------|--------|---------|
| **PendingLoads** | 8 | All async load requests |
| **LoadedDetails** | 4 | Neo4j data for current selection |
| **SchemaOverlayState** | 5 | Schema overlay feature state |
| **YamlPreviewState** | 4 | YAML panel state |
| **OverlayState** | 4 | Help/Legend/Recent overlays |
| **Total** | 25 | ~45% of App fields |

### Implementation Details

#### 1. PendingLoads (8 fields)

```rust
/// All pending async load requests.
/// v0.14.0: Extracted from App struct for clarity.
#[derive(Debug, Default)]
pub struct PendingLoads {
    pub instance: Option<String>,
    pub arcs: Option<String>,
    pub instance_arcs: Option<(String, Vec<String>)>,
    pub entity_categories: bool,
    pub category_instances: Option<String>,
    pub arc_class: Option<String>,
    pub realm: Option<String>,
    pub layer: Option<String>,
}

impl PendingLoads {
    /// Check if any load is pending.
    pub fn has_pending(&self) -> bool {
        self.instance.is_some()
            || self.arcs.is_some()
            || self.instance_arcs.is_some()
            || self.entity_categories
            || self.category_instances.is_some()
            || self.arc_class.is_some()
            || self.realm.is_some()
            || self.layer.is_some()
    }

    /// Clear all pending loads.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
```

**Mapping:**
- `pending_instance_load` → `pending.instance`
- `pending_arcs_load` → `pending.arcs`
- `pending_instance_arcs_load` → `pending.instance_arcs`
- `pending_entity_categories_load` → `pending.entity_categories`
- `pending_category_instances_load` → `pending.category_instances`
- `pending_arc_class_load` → `pending.arc_class`
- `pending_realm_load` → `pending.realm`
- `pending_layer_load` → `pending.layer`

#### 2. LoadedDetails (4 fields)

```rust
/// Neo4j details for current selection.
/// Loaded async when user selects Realm/Layer/Class/Arc.
#[derive(Debug, Default)]
pub struct LoadedDetails {
    pub class_arcs: Option<ClassArcsData>,
    pub arc_class: Option<ArcClassDetails>,
    pub realm: Option<RealmDetails>,
    pub layer: Option<LayerDetails>,
}

impl LoadedDetails {
    /// Clear all loaded details.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
```

**Mapping:**
- `class_arcs` → `details.class_arcs`
- `arc_class_details` → `details.arc_class`
- `realm_details` → `details.realm`
- `layer_details` → `details.layer`

#### 3. SchemaOverlayState (5 fields)

```rust
/// Schema overlay state for Data mode.
/// Shows YAML schema properties alongside Neo4j instance data.
#[derive(Debug)]
pub struct SchemaOverlayState {
    /// Whether schema overlay is enabled (toggle with 's')
    pub enabled: bool,
    /// Matched properties for current instance
    pub matched_properties: Option<Vec<MatchedProperty>>,
    /// Coverage stats for current instance
    pub coverage_stats: Option<CoverageStats>,
    /// Validated properties for current Class
    pub validated_class_properties: Option<Vec<ValidatedProperty>>,
    /// Validation stats for current Class
    pub validation_stats: Option<ValidationStats>,
}

impl Default for SchemaOverlayState {
    fn default() -> Self {
        Self {
            enabled: true, // Enabled by default
            matched_properties: None,
            coverage_stats: None,
            validated_class_properties: None,
            validation_stats: None,
        }
    }
}
```

**Mapping:**
- `schema_overlay_enabled` → `schema_overlay.enabled`
- `matched_properties` → `schema_overlay.matched_properties`
- `coverage_stats` → `schema_overlay.coverage_stats`
- `validated_class_properties` → `schema_overlay.validated_class_properties`
- `validation_stats` → `schema_overlay.validation_stats`

#### 4. YamlPreviewState (4 fields)

```rust
/// YAML panel state.
/// Displays Class YAML or Instance data in the right panel.
#[derive(Debug, Default)]
pub struct YamlPreviewState {
    pub content: String,
    pub path: String,
    pub scroll: usize,
    pub line_count: usize,
}
```

**Mapping:**
- `yaml_content` → `yaml.content`
- `yaml_path` → `yaml.path`
- `yaml_scroll` → `yaml.scroll`
- `yaml_line_count` → `yaml.line_count`

#### 5. OverlayState (4 fields)

```rust
/// Overlay visibility state.
/// Overlays are modal panels that appear on top of the main UI.
#[derive(Debug, Default)]
pub struct OverlayState {
    pub help_active: bool,
    pub legend_active: bool,
    pub recent_items_active: bool,
    pub recent_items_cursor: usize,
}

impl OverlayState {
    /// Check if any overlay is active.
    pub fn is_active(&self) -> bool {
        self.help_active || self.legend_active || self.recent_items_active
    }

    /// Close all overlays.
    pub fn close_all(&mut self) {
        self.help_active = false;
        self.legend_active = false;
        self.recent_items_active = false;
    }
}
```

### App Struct After Refactoring

```rust
pub struct App {
    // Core state (unchanged)
    pub theme: Theme,
    pub mode: NavMode,
    pub focus: Focus,
    pub selected_box: InfoBox,
    pub tree_cursor: usize,
    pub mode_cursors: [usize; 3],
    pub tree_scroll: usize,
    pub tree_height: usize,
    pub tree: TaxonomyTree,
    pub root_path: String,

    // Extracted state (already)
    pub search: SearchState,
    pub nexus: NexusState,

    // NEW: Extracted sub-structs
    pub pending: PendingLoads,
    pub details: LoadedDetails,
    pub schema_overlay: SchemaOverlayState,
    pub yaml: YamlPreviewState,
    pub overlays: OverlayState,

    // Remaining state (keep as-is for now)
    pub nav_history: Vec<(NavMode, usize)>,
    pub nav_history_pos: usize,
    pub status_message: Option<(String, std::time::Instant)>,
    pub pending_refresh: bool,
    pub source_tab: SourceTab,
    source_tab_class_cursor: Option<usize>,
    pub info_scroll: usize,
    pub info_line_count: usize,
    pub yaml_cache: FxHashMap<String, String>,
    pub data_filter_class: Option<String>,
    pub data_cursor_before_filter: usize,
    pub hide_empty: bool,
    pub loaded_views: LoadedViews,
    pub tick: u16,
    pub navigation_generation: u64,
    pub focused_property_idx: usize,
    pub expanded_property: bool,
    pub json_pretty: bool,
    pub trait_filter: Option<String>,
    pub filter_pending: bool,
    pub mini_bar_cache: RefCell<RenderCache<Vec<Span<'static>>>>,
}
```

**Result**: 55 fields → 30 direct + 25 in sub-structs

### Files to Update

1. **`src/tui/app.rs`**: Define sub-structs, update App, update `new()`
2. **`src/tui/mod.rs`**: Update field access (e.g., `app.pending.arcs`)
3. **`src/tui/handlers/*.rs`**: Update any handler accessing these fields
4. **`src/tui/ui.rs`**: Update render code

---

## Part 2: Markdown Utils Extraction

### Current State

The files `market.rs`, `culture.rs`, `expression.rs`, and `formatting.rs` have duplicated patterns:

1. **`parse_frontmatter()`** - Identical in all 4 files
2. **`split_sections()`** - Nearly identical in culture.rs and formatting.rs
3. **Common regex patterns** - RE_VERSION, RE_DATE, RE_SECTION

### Proposed Module

Create `src/parsers/markdown_utils.rs`:

```rust
//! Common markdown parsing utilities for ATH data files.
//!
//! Shared by: market.rs, culture.rs, expression.rs, formatting.rs

use std::collections::HashMap;
use std::sync::LazyLock;
use regex::Regex;

// ============================================================================
// Common Lazy Regex Patterns
// ============================================================================

/// Template version extraction: `template_version: X.X`
pub static RE_VERSION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"template_version:\s*(.+)").expect("valid version regex")
});

/// Last updated date extraction: `last_updated: YYYY-MM-DD`
pub static RE_DATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"last_updated:\s*(.+)").expect("valid date regex")
});

/// Section header: `## N. Title`
pub static RE_SECTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"##\s+\d+\.\s+(.+)").expect("valid section regex")
});

// ============================================================================
// Common Parsing Functions
// ============================================================================

/// Parse YAML frontmatter for template_version and last_updated.
/// Returns (version, date) with defaults if not found.
pub fn parse_frontmatter(content: &str) -> (String, String) {
    let version = RE_VERSION
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "2.0".to_string());

    let date = RE_DATE
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    (version, date)
}

/// Split markdown content into sections by ## headers.
/// Returns HashMap<normalized_section_name, section_content>.
pub fn split_sections(
    content: &str,
    normalizer: fn(&str) -> String,
) -> HashMap<String, String> {
    let mut sections = HashMap::new();
    let mut current_section: Option<String> = None;
    let mut current_content = String::new();

    for line in content.lines() {
        if let Some(caps) = RE_SECTION.captures(line) {
            // Save previous section
            if let Some(ref name) = current_section {
                sections.insert(name.clone(), current_content.clone());
            }

            // Start new section
            let section_name = caps
                .get(1)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            current_section = Some(normalizer(&section_name));
            current_content = String::new();
        } else if current_section.is_some() {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Save last section
    if let Some(name) = current_section {
        sections.insert(name, current_content);
    }

    sections
}

/// Extract locale key from filename (e.g., "fr-FR.md" -> "fr-FR")
pub fn locale_from_filename(filename: &str) -> &str {
    filename.trim_end_matches(".md")
}
```

### Files to Update

1. **Create `src/parsers/markdown_utils.rs`**
2. **Update `src/parsers/mod.rs`**: Add `pub mod markdown_utils;`
3. **Update `market.rs`**: Use `markdown_utils::{RE_VERSION, RE_DATE, parse_frontmatter}`
4. **Update `culture.rs`**: Use `markdown_utils::{parse_frontmatter, split_sections}`
5. **Update `expression.rs`**: Use `markdown_utils::{RE_VERSION, RE_DATE, parse_frontmatter}`
6. **Update `formatting.rs`**: Use `markdown_utils::{RE_VERSION, RE_DATE, parse_frontmatter, split_sections}`

### Estimated Line Reduction

| File | Before | After | Saved |
|------|--------|-------|-------|
| market.rs | 927 | 915 | 12 |
| culture.rs | 877 | 845 | 32 |
| expression.rs | 629 | 615 | 14 |
| formatting.rs | 1428 | 1395 | 33 |
| **Total** | | | ~91 lines |

---

## Execution Order

### Phase 1: App Struct (Lower Risk)

1. Create sub-structs in `app.rs`
2. Add `pub pending: PendingLoads` to App
3. Update `App::new()` initialization
4. Update `take_pending_*` methods to use new struct
5. Run tests

### Phase 2: Remaining Sub-Structs

6. Add `LoadedDetails`, `SchemaOverlayState`, `YamlPreviewState`, `OverlayState`
7. Update all references in `mod.rs`, `handlers/`, `ui.rs`
8. Run tests

### Phase 3: Markdown Utils (Independent)

9. Create `markdown_utils.rs`
10. Update parsers one at a time (test after each)
11. Run full test suite

---

## Testing Strategy

After each change:
1. `cargo test` - all 1192+ tests pass
2. `cargo clippy -- -D warnings` - zero warnings
3. Manual TUI test for navigation

---

## Rollback Plan

Git commit after each phase. If issues arise:
```bash
git revert HEAD  # Revert last phase
```
