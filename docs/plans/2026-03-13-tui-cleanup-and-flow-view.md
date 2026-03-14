# TUI Cleanup + Flow View Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Delete Nexus + Views modules (~13,300 lines), fix security/bugs from audit, and create a new navigable ASCII Flow view showing NovaNet architecture and data pipelines.

**Architecture:** 3 phases — (1) Delete dead code (Nexus + Views), (2) Fix critical security + bugs from code audit, (3) Build new Flow view with two navigable diagrams (Schema Architecture + Data Pipeline). Graph mode stays intact.

**Tech Stack:** Rust (ratatui, crossterm), Neo4j (neo4rs), TUI patterns (KeyResult dispatch, render_with_scroll)

---

## Phase 1: Delete Nexus + Views (~13,300 lines)

### Task 1: Delete Nexus module files

**Files:**
- Delete: `tools/novanet/src/tui/nexus/arch.rs` (390 lines)
- Delete: `tools/novanet/src/tui/nexus/arcs.rs` (464 lines)
- Delete: `tools/novanet/src/tui/nexus/glossary.rs` (1,166 lines)
- Delete: `tools/novanet/src/tui/nexus/i18n.rs` (935 lines)
- Delete: `tools/novanet/src/tui/nexus/intro.rs` (670 lines)
- Delete: `tools/novanet/src/tui/nexus/layers.rs` (351 lines)
- Delete: `tools/novanet/src/tui/nexus/mod.rs` (2,723 lines)
- Delete: `tools/novanet/src/tui/nexus/persistence.rs` (949 lines)
- Delete: `tools/novanet/src/tui/nexus/pipeline.rs` (652 lines)
- Delete: `tools/novanet/src/tui/nexus/quiz.rs` (1,624 lines)
- Delete: `tools/novanet/src/tui/nexus/stats.rs` (670 lines)
- Delete: `tools/novanet/src/tui/nexus/tutorial.rs` (924 lines)
- Delete: `tools/novanet/src/tui/nexus/views.rs` (1,245 lines)

**Step 1: Delete the nexus directory**

```bash
rm -rf tools/novanet/src/tui/nexus/
```

**Step 2: Delete handler files**

```bash
rm tools/novanet/src/tui/handlers/nexus.rs
rm tools/novanet/src/tui/handlers/views.rs
```

**Step 3: Verify deletion**

```bash
ls tools/novanet/src/tui/nexus/ 2>&1  # Should fail: No such file or directory
ls tools/novanet/src/tui/handlers/nexus.rs 2>&1  # Should fail
ls tools/novanet/src/tui/handlers/views.rs 2>&1  # Should fail
```

---

### Task 2: Remove Nexus module declaration from tui/mod.rs

**Files:**
- Modify: `tools/novanet/src/tui/mod.rs:24`

**Step 1: Remove the `pub mod nexus;` declaration**

In `tools/novanet/src/tui/mod.rs`, delete this line:

```rust
// DELETE this line:
pub mod nexus;
```

The module list should go from:

```rust
mod app;
pub mod cache;
pub mod clipboard;
#[path = "colors.generated.rs"]
pub mod colors;
mod data;
mod handlers;
pub mod icons;
pub mod nexus;  // <-- DELETE
mod schema;
pub mod theme;
mod ui;
pub mod unicode;
pub mod unified_types;
pub mod widgets;
mod yaml;
```

To:

```rust
mod app;
pub mod cache;
pub mod clipboard;
#[path = "colors.generated.rs"]
pub mod colors;
mod data;
mod handlers;
pub mod icons;
mod schema;
pub mod theme;
mod ui;
pub mod unicode;
pub mod unified_types;
pub mod widgets;
mod yaml;
```

**Step 2: Attempt compile to see remaining errors**

```bash
cd tools/novanet && cargo check 2>&1 | head -80
```

Expected: Errors about `NexusState`, `NexusTab`, `LoadedViews`, `NavMode::Nexus`, `NavMode::Views`, `handle_nexus_key`, `handle_views_key`

---

### Task 3: Update NavMode enum — remove Views and Nexus

**Files:**
- Modify: `tools/novanet/src/tui/app/state.rs:22-56`

**Step 1: Simplify NavMode to Graph-only (for now, Flow added later)**

Replace the full `NavMode` enum and impl block:

```rust
/// Navigation mode — simplified in v0.20.0.
/// Views and Nexus removed. Flow will be added in Phase 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavMode {
    /// Graph mode: Unified tree view (Realm > Layer > Class hierarchy with instances)
    #[default]
    Graph,
}

impl NavMode {
    pub fn label(&self) -> &'static str {
        match self {
            NavMode::Graph => "Graph",
        }
    }

    /// Get array index for mode_cursors.
    pub fn index(&self) -> usize {
        match self {
            NavMode::Graph => 0,
        }
    }

    /// Get all modes in order.
    pub fn all() -> &'static [NavMode] {
        &[NavMode::Graph]
    }
}
```

**Step 2: Update the snapshot test if it exists**

Delete or update snapshot file:
```bash
rm -f tools/novanet/src/tui/app/snapshots/novanet__tui__app__state__tests__navmode_labels.snap
```

**Step 3: Attempt compile**

```bash
cd tools/novanet && cargo check 2>&1 | head -80
```

---

### Task 4: Clean App struct — remove Nexus/Views fields and imports

**Files:**
- Modify: `tools/novanet/src/tui/app/mod.rs:34-35` (imports)
- Modify: `tools/novanet/src/tui/app/mod.rs:61` (mode_cursors)
- Modify: `tools/novanet/src/tui/app/mod.rs:83` (nexus field)

**Step 1: Remove nexus imports**

Delete these two lines from `tools/novanet/src/tui/app/mod.rs`:

```rust
// DELETE these imports:
use super::nexus::views::LoadedViews;
use super::nexus::{NexusState, NexusTab};
```

**Step 2: Remove `nexus` field from App struct**

Delete this field:

```rust
// DELETE:
/// Nexus mode state (gamified learning hub).
pub nexus: NexusState,
```

**Step 3: Resize mode_cursors**

Change:
```rust
pub mode_cursors: [usize; 3],
```
To:
```rust
pub mode_cursors: [usize; 1],
```

**Step 4: Remove all `LoadedViews` usage**

Search for and remove any `loaded_views` field and its initialization.

**Step 5: Update App::new() — remove NexusState and LoadedViews init**

Find `NexusState::new()` and `LoadedViews::default()` in `App::new()` and remove them.

**Step 6: Attempt compile**

```bash
cd tools/novanet && cargo check 2>&1 | head -80
```

Expected: More errors from UI rendering code referencing nexus/views.

---

### Task 5: Clean handlers/mod.rs — remove dispatch for Nexus/Views

**Files:**
- Modify: `tools/novanet/src/tui/handlers/mod.rs`

**Step 1: Replace the entire file**

```rust
//! Mode-specific key handlers for TUI.
//!
//! v0.20.0: Simplified — only Graph mode (uses global handlers directly).

use crossterm::event::KeyEvent;

use super::app::App;

/// Result of mode-specific key handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyResult {
    /// Key was handled, consumed
    Handled,
    /// Key should fall through to global handlers
    FallThrough,
}

impl KeyResult {
    /// Convert to Option<bool> for easy integration with existing code.
    pub fn as_option(self) -> Option<bool> {
        match self {
            KeyResult::Handled => Some(true),
            KeyResult::FallThrough => None,
        }
    }
}

/// Dispatch key event to mode-specific handler.
///
/// Returns `Some(true)` if handled, `None` if should fall through to global handlers.
pub fn dispatch_mode_handler(_app: &mut App, _key: KeyEvent) -> Option<bool> {
    // Graph mode uses global handlers directly — no mode-specific dispatch needed
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_result_as_option() {
        assert_eq!(KeyResult::Handled.as_option(), Some(true));
        assert_eq!(KeyResult::FallThrough.as_option(), None);
    }
}
```

**Step 2: Attempt compile**

```bash
cd tools/novanet && cargo check 2>&1 | head -80
```

---

### Task 6: Clean UI rendering — remove Nexus/Views branches

**Files:**
- Modify: `tools/novanet/src/tui/ui/mod.rs:599-605` (render dispatch)
- Modify: `tools/novanet/src/tui/ui/mod.rs:821-822` (mode indicators)
- Modify: `tools/novanet/src/tui/ui/status.rs` (status bar references)
- Modify: `tools/novanet/src/tui/ui/overlays.rs:120` (nexus check)

**Step 1: In `ui/mod.rs`, remove Nexus/Views render branches**

Find and remove the blocks around lines 599-605:

```rust
// DELETE these blocks:
if app.mode == NavMode::Nexus {
    // ... render nexus ...
}
if app.mode == NavMode::Views {
    // ... render views ...
}
```

**Step 2: In `ui/mod.rs`, remove mode indicator labels**

Around line 821, remove:
```rust
// DELETE:
crate::tui::app::NavMode::Nexus => "[N]",
crate::tui::app::NavMode::Views => "[V]",
```

**Step 3: In `ui/status.rs`, remove Nexus/Views status bar code**

Remove all `NavMode::Nexus` and `NavMode::Views` match arms:
- Line 66: `NavMode::Nexus => String::new()` — delete
- Line 84: `NavMode::Views => ...` — delete
- Lines 273-277: nexus context_actions block — delete
- Lines 314-317: nexus breadcrumb — delete
- Lines 369-406: nexus_style — delete

Remove all test assertions for Views/Nexus (lines 608-616, 680).

**Step 4: In `ui/overlays.rs`, remove nexus mode check**

Line 120: remove the `is_nexus_mode` variable and its usage.

**Step 5: Compile and fix remaining errors**

```bash
cd tools/novanet && cargo check 2>&1 | head -80
```

Iterate until `cargo check` passes.

---

### Task 7: Clean data.rs — remove Nexus-only data functions

**Files:**
- Modify: `tools/novanet/src/tui/data.rs`

**Step 1: Remove `get_all_adrs()` function**

This function (starting at line 500) is only used by Nexus Arch tab. Delete the entire function and the `AdrEntry`, `AdrCategory` structs/enums if they're only used there.

**Step 2: Remove the import from app/mod.rs**

In `tools/novanet/src/tui/app/mod.rs:31`, remove the `get_all_adrs` import:

```rust
// Change from:
use super::data::{
    ArcClassDetails, ClassArcsData, LayerDetails, RealmDetails, TaxonomyTree, TreeItem,
    get_all_adrs, get_architecture_diagram,
};
// To:
use super::data::{
    ArcClassDetails, ClassArcsData, LayerDetails, RealmDetails, TaxonomyTree, TreeItem,
    get_architecture_diagram,
};
```

**Step 3: Compile**

```bash
cd tools/novanet && cargo check 2>&1 | head -80
```

---

### Task 8: Fix all remaining compile errors and run tests

**Step 1: Fix any remaining compile errors iteratively**

```bash
cd tools/novanet && cargo check 2>&1 | head -100
```

Fix each error. Common patterns:
- Dead `use` statements referencing removed types
- Match arms for removed NavMode variants
- Struct initializers with removed fields
- Test code referencing Nexus/Views

**Step 2: Run full test suite**

```bash
cd tools/novanet && cargo test 2>&1 | tail -30
```

Expected: Some test failures from tests referencing Nexus.

**Step 3: Delete or update failing tests**

Any test that references `NavMode::Nexus`, `NavMode::Views`, `NexusState`, `NexusTab`, etc. should be deleted.

**Step 4: Verify clean build**

```bash
cd tools/novanet && cargo clippy -- -D warnings 2>&1 | tail -20
cd tools/novanet && cargo test 2>&1 | tail -10
```

Both must pass with zero warnings.

**Step 5: Commit**

```bash
cd tools/novanet && git add -A && git commit -m "$(cat <<'EOF'
refactor(tui): delete Nexus + Views modules (~13,300 lines)

Remove Nexus (quiz, glossary, tutorial, stats, arch, pipeline,
persistence, intro, i18n, arcs, layers, views) and Views mode.
Simplify NavMode to Graph-only. Clean all UI rendering branches,
status bar, overlays, and handlers.

Preparation for new Flow view in Phase 3.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

## Phase 2: Security + Bug Fixes (from audit)

### Task 9: Fix Cypher injection in MCP search — property names

**Files:**
- Modify: `tools/novanet-mcp/src/tools/search.rs:548-556`

**Step 1: Write the failing test**

In `tools/novanet-mcp/src/tools/search.rs`, add to existing test module:

```rust
#[test]
fn test_property_search_rejects_injection() {
    // Property names like "key}) RETURN n //--" must be rejected
    assert!(!is_valid_label("key}) RETURN n"));
    assert!(!is_valid_label("name; DROP"));
    // Valid property names
    assert!(is_valid_label("key"));
    assert!(is_valid_label("display_name"));
    assert!(is_valid_label("content"));
}
```

**Step 2: Run test to verify it passes (is_valid_label already exists)**

```bash
cd tools/novanet-mcp && cargo test test_property_search_rejects_injection -- --nocapture
```

**Step 3: Apply validation to property names in property search**

Around line 548, where `properties.iter()` builds conditions, add filtering:

```rust
// BEFORE the for loop that builds conditions:
let properties: Vec<&String> = properties.iter().filter(|p| is_valid_label(p)).collect();
if properties.is_empty() {
    return Err(McpError::invalid_params(
        "No valid property names provided",
        None,
    ));
}
```

**Step 4: Run tests**

```bash
cd tools/novanet-mcp && cargo test 2>&1 | tail -10
```

**Step 5: Commit**

```bash
cd tools/novanet-mcp && git add -A && git commit -m "$(cat <<'EOF'
fix(mcp): validate property names against Cypher injection

Apply is_valid_label() regex to property names in property search
mode. Rejects injection attempts like "key}) RETURN n".

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 10: Fix arc_kinds injection in MCP search walk mode

**Files:**
- Modify: `tools/novanet-mcp/src/tools/search.rs:804-811`

**Step 1: Write the failing test**

```rust
#[test]
fn test_arc_filter_rejects_injection() {
    // arc_kinds with injection payload
    let kinds_injection = Some(vec!["HAS_NATIVE|DETACH DELETE n//".to_string()]);
    let result = build_arc_filter(&None, &kinds_injection);
    // Should NOT contain the injection — only valid arc labels pass through
    assert!(!result.contains("DETACH"));
}
```

**Step 2: Run test to verify it fails**

```bash
cd tools/novanet-mcp && cargo test test_arc_filter_rejects_injection -- --nocapture
```

Expected: FAIL (currently no validation on arc_kinds)

**Step 3: Add validation to build_arc_filter**

In `build_arc_filter` at line 804, filter `arc_kinds`:

```rust
fn build_arc_filter(families: &Option<Vec<String>>, kinds: &Option<Vec<String>>) -> String {
    let mut filters = Vec::new();

    if let Some(kinds) = kinds {
        if !kinds.is_empty() {
            // Validate each arc kind against injection
            filters.extend(kinds.iter().filter(|k| is_valid_label(k)).cloned());
        }
    }
    // ... rest unchanged
```

**Step 4: Run test to verify it passes**

```bash
cd tools/novanet-mcp && cargo test test_arc_filter -- --nocapture
```

**Step 5: Run full test suite**

```bash
cd tools/novanet-mcp && cargo test 2>&1 | tail -10
```

**Step 6: Commit**

```bash
cd tools/novanet-mcp && git add -A && git commit -m "$(cat <<'EOF'
fix(mcp): validate arc_kinds against Cypher injection in walk mode

Filter arc_kinds through is_valid_label() before interpolating
into Cypher. Rejects payloads like "HAS_NATIVE|DETACH DELETE n".

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 11: Fix has_pending_load() missing entity_natives check

**Files:**
- Modify: `tools/novanet/src/tui/app/mod.rs` (search for `has_pending_load`)

**Step 1: Find and read the function**

```bash
cd tools/novanet && grep -n "fn has_pending_load" src/tui/app/mod.rs
```

**Step 2: Add the missing check**

The function should include `self.pending.entity_natives` in its check. Add:

```rust
|| self.pending.entity_natives
```

to the boolean chain.

**Step 3: Compile and test**

```bash
cd tools/novanet && cargo check && cargo test 2>&1 | tail -10
```

**Step 4: Commit**

```bash
cd tools/novanet && git add -A && git commit -m "$(cat <<'EOF'
fix(tui): include entity_natives in has_pending_load() check

The loading spinner was not shown when entity natives were being
loaded because the pending flag was missing from the check.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

## Phase 3: New Flow View

### Task 12: Add NavMode::Flow variant

**Files:**
- Modify: `tools/novanet/src/tui/app/state.rs`

**Step 1: Add Flow variant to NavMode**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavMode {
    #[default]
    Graph,
    /// Flow mode: Navigable ASCII diagrams of NovaNet architecture + data pipelines
    Flow,
}

impl NavMode {
    pub fn label(&self) -> &'static str {
        match self {
            NavMode::Graph => "Graph",
            NavMode::Flow => "Flow",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            NavMode::Graph => 0,
            NavMode::Flow => 1,
        }
    }

    pub fn all() -> &'static [NavMode] {
        &[NavMode::Graph, NavMode::Flow]
    }
}
```

**Step 2: Update mode_cursors in App struct**

```rust
pub mode_cursors: [usize; 2],
```

**Step 3: Add FlowTab enum**

Add to `tools/novanet/src/tui/app/state.rs`:

```rust
/// Flow view tabs — two navigable ASCII diagrams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlowTab {
    /// Schema Architecture: Realm > Layer > Class hierarchy with arcs
    #[default]
    Schema,
    /// Data Pipeline: Entity → EntityNative → Page → Block → BlockNative flow
    Pipeline,
}

impl FlowTab {
    pub fn label(&self) -> &'static str {
        match self {
            FlowTab::Schema => "Schema Architecture",
            FlowTab::Pipeline => "Data Pipeline",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            FlowTab::Schema => FlowTab::Pipeline,
            FlowTab::Pipeline => FlowTab::Schema,
        }
    }
}
```

**Step 4: Add FlowState struct**

```rust
/// State for the Flow view (navigable ASCII diagrams).
#[derive(Debug, Clone, Default)]
pub struct FlowState {
    /// Current tab (Schema or Pipeline)
    pub tab: FlowTab,
    /// Vertical scroll position
    pub scroll_y: usize,
    /// Horizontal scroll position (diagrams can be wide)
    pub scroll_x: usize,
    /// Selected node index in the current diagram (for highlighting)
    pub selected: usize,
    /// Total number of selectable nodes in current diagram
    pub total_nodes: usize,
}
```

**Step 5: Export new types from state.rs**

Add `FlowTab`, `FlowState` to the `pub use state::{}` list in `app/mod.rs`.

**Step 6: Compile**

```bash
cd tools/novanet && cargo check 2>&1 | head -40
```

**Step 7: Commit**

```bash
cd tools/novanet && git add -A && git commit -m "$(cat <<'EOF'
feat(tui): add NavMode::Flow + FlowState + FlowTab types

Add Flow navigation mode with two tabs: Schema Architecture
and Data Pipeline. Includes FlowState with scroll and selection.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 13: Add flow field to App struct and wire mode switching

**Files:**
- Modify: `tools/novanet/src/tui/app/mod.rs`

**Step 1: Add flow field to App**

After `pub schema_overlay: SchemaOverlayState,` add:

```rust
/// Flow mode state (navigable architecture diagrams).
pub flow: FlowState,
```

**Step 2: Initialize in App::new()**

Add `flow: FlowState::default(),` to the struct initialization.

**Step 3: Wire `[2]` key to Flow mode**

Find where mode switching is handled (key code matching `KeyCode::Char('1')`, `KeyCode::Char('2')`, etc.). Update so that `2` switches to `NavMode::Flow`.

**Step 4: Compile and test**

```bash
cd tools/novanet && cargo check && cargo test 2>&1 | tail -10
```

**Step 5: Commit**

```bash
cd tools/novanet && git add -A && git commit -m "$(cat <<'EOF'
feat(tui): wire Flow mode to App struct and [2] key

FlowState initialized in App::new(). Key [2] switches to Flow mode.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 14: Create Flow handler module

**Files:**
- Create: `tools/novanet/src/tui/handlers/flow.rs`
- Modify: `tools/novanet/src/tui/handlers/mod.rs`

**Step 1: Create flow.rs handler**

```rust
//! Key handler for Flow mode.
//!
//! Navigation:
//! - Tab: Switch between Schema/Pipeline tabs
//! - j/k or Up/Down: Scroll vertically
//! - h/l or Left/Right: Scroll horizontally
//! - Enter: Select/highlight node
//! - n/p: Next/previous selectable node

use crossterm::event::{KeyCode, KeyEvent};

use super::KeyResult;
use crate::tui::app::App;

pub fn handle_flow_key(app: &mut App, key: KeyEvent) -> KeyResult {
    match key.code {
        // Tab switching
        KeyCode::Tab => {
            app.flow.tab = app.flow.tab.toggle();
            app.flow.scroll_y = 0;
            app.flow.scroll_x = 0;
            app.flow.selected = 0;
            KeyResult::Handled
        }

        // Vertical scroll
        KeyCode::Char('j') | KeyCode::Down => {
            app.flow.scroll_y = app.flow.scroll_y.saturating_add(1);
            KeyResult::Handled
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.flow.scroll_y = app.flow.scroll_y.saturating_sub(1);
            KeyResult::Handled
        }

        // Horizontal scroll
        KeyCode::Char('l') | KeyCode::Right => {
            app.flow.scroll_x = app.flow.scroll_x.saturating_add(2);
            KeyResult::Handled
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app.flow.scroll_x = app.flow.scroll_x.saturating_sub(2);
            KeyResult::Handled
        }

        // Page scroll
        KeyCode::Char('d') => {
            app.flow.scroll_y = app.flow.scroll_y.saturating_add(10);
            KeyResult::Handled
        }
        KeyCode::Char('u') => {
            app.flow.scroll_y = app.flow.scroll_y.saturating_sub(10);
            KeyResult::Handled
        }

        // Node selection
        KeyCode::Char('n') => {
            if app.flow.total_nodes > 0 {
                app.flow.selected = (app.flow.selected + 1) % app.flow.total_nodes;
            }
            KeyResult::Handled
        }
        KeyCode::Char('p') => {
            if app.flow.total_nodes > 0 {
                app.flow.selected = if app.flow.selected == 0 {
                    app.flow.total_nodes.saturating_sub(1)
                } else {
                    app.flow.selected - 1
                };
            }
            KeyResult::Handled
        }

        // Home/End
        KeyCode::Home => {
            app.flow.scroll_y = 0;
            app.flow.scroll_x = 0;
            KeyResult::Handled
        }

        _ => KeyResult::FallThrough,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::state::FlowTab;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn test_tab_switches_flow_tab() {
        // This test needs a mock App — placeholder for now
        // Will be implemented when App::test_default() is available
    }

    #[test]
    fn test_scroll_keys() {
        // Placeholder — verify j/k/h/l change scroll positions
    }
}
```

**Step 2: Wire into handlers/mod.rs**

```rust
//! Mode-specific key handlers for TUI.
//!
//! v0.20.0: Two modes — Graph (global handlers) and Flow (dedicated handler).

mod flow;

pub use flow::handle_flow_key;

use crossterm::event::KeyEvent;

use super::app::{App, NavMode};

/// Result of mode-specific key handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyResult {
    Handled,
    FallThrough,
}

impl KeyResult {
    pub fn as_option(self) -> Option<bool> {
        match self {
            KeyResult::Handled => Some(true),
            KeyResult::FallThrough => None,
        }
    }
}

/// Dispatch key event to mode-specific handler.
pub fn dispatch_mode_handler(app: &mut App, key: KeyEvent) -> Option<bool> {
    match app.mode {
        NavMode::Flow => handle_flow_key(app, key).as_option(),
        NavMode::Graph => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_result_as_option() {
        assert_eq!(KeyResult::Handled.as_option(), Some(true));
        assert_eq!(KeyResult::FallThrough.as_option(), None);
    }
}
```

**Step 3: Compile**

```bash
cd tools/novanet && cargo check 2>&1 | head -40
```

**Step 4: Commit**

```bash
cd tools/novanet && git add -A && git commit -m "$(cat <<'EOF'
feat(tui): add Flow mode key handler with navigation

Tab switch, j/k/h/l scroll, n/p node selection, d/u page scroll.
Wired into dispatch_mode_handler for NavMode::Flow.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 15: Create Flow diagrams module — ASCII content

**Files:**
- Create: `tools/novanet/src/tui/flow.rs`

**Step 1: Create the flow diagram data module**

This module defines the static ASCII diagrams. It produces `Vec<FlowLine>` where each line can have highlighted segments (the selectable "nodes" in the diagram).

```rust
//! Flow diagram content for TUI Flow view.
//!
//! Two diagrams:
//! 1. Schema Architecture — Realm > Layer > Class hierarchy with key arcs
//! 2. Data Pipeline — Entity → EntityNative → Page → Block → BlockNative flow

/// A single line in a flow diagram, with optional highlighted segments.
#[derive(Debug, Clone)]
pub struct FlowLine {
    pub text: String,
    /// Highlighted segments: (start_col, end_col, node_index)
    /// node_index is used for selection navigation
    pub highlights: Vec<(usize, usize, usize)>,
}

/// A complete flow diagram with metadata.
#[derive(Debug, Clone)]
pub struct FlowDiagram {
    pub title: String,
    pub lines: Vec<FlowLine>,
    pub node_count: usize,
    /// Node labels for the info/description panel
    pub node_labels: Vec<String>,
    pub node_descriptions: Vec<String>,
}

/// Build the Schema Architecture diagram.
pub fn schema_architecture() -> FlowDiagram {
    let mut node_idx: usize = 0;
    let mut node_labels = Vec::new();
    let mut node_descriptions = Vec::new();
    let mut lines = Vec::new();

    // Helper to register a node
    let mut add_node = |label: &str, desc: &str| -> usize {
        let idx = node_idx;
        node_labels.push(label.to_string());
        node_descriptions.push(desc.to_string());
        node_idx += 1;
        idx
    };

    let shared = add_node("SHARED", "Read-only universal definitions (36 nodes)");
    let org = add_node("ORG", "Organization-specific content (23 nodes)");
    let config_s = add_node("config (shared)", "BCP-47 locale definitions, ScriptSystem, WritingDirection");
    let locale = add_node("locale", "LocaleVoice, LocaleCulture, LocaleFormatting, LocaleAdaptation, LocaleSlugification");
    let geo = add_node("geography", "Continent, Country, Region, City, PopulationCluster, EconomicRegion, CulturalRealm");
    let knowledge = add_node("knowledge", "ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet + atoms (21 nodes)");
    let config_o = add_node("config (org)", "OrgConfig");
    let foundation = add_node("foundation", "Project, Brand, BrandDesign, BrandPrinciples, PromptStyle + 3 more (8 nodes)");
    let structure = add_node("structure", "Page, Block, EntityCategory (3 nodes)");
    let semantic = add_node("semantic", "Entity, EntityNative (2 nodes)");
    let instruction = add_node("instruction", "BlockType, BlockInstruction, PageInstruction (3 nodes)");
    let output = add_node("output", "PageNative, BlockNative, ProjectNative + 3 more (6 nodes)");

    let plain = |text: &str| FlowLine { text: text.to_string(), highlights: vec![] };
    let with_hl = |text: &str, highlights: Vec<(usize, usize, usize)>| FlowLine { text: text.to_string(), highlights };

    lines.push(plain(""));
    lines.push(plain("  NovaNet v0.20.0 — 59 Nodes, 159 Arcs, 6 Families"));
    lines.push(plain("  ═══════════════════════════════════════════════════════════════════════"));
    lines.push(plain(""));
    lines.push(with_hl(
        "  ┌─── SHARED (36 nodes, READ-ONLY) ──────┐   ┌─── ORG (23 nodes) ──────────────┐",
        vec![(6, 40, shared), (46, 77, org)],
    ));
    lines.push(plain("  │                                        │   │                                  │"));
    lines.push(with_hl(
        "  │  config ─────── Locale, Script, Dir    │   │  config ──── OrgConfig            │",
        vec![(5, 11, config_s), (46, 52, config_o)],
    ));
    lines.push(plain("  │     │                                  │   │     │                              │"));
    lines.push(with_hl(
        "  │  locale ─────── Voice, Culture, Format │   │  foundation ── Project, Brand     │",
        vec![(5, 11, locale), (46, 56, foundation)],
    ));
    lines.push(plain("  │     │                                  │   │     │                              │"));
    lines.push(with_hl(
        "  │  geography ──── Continent..City (7)    │   │  structure ─── Page, Block        │",
        vec![(5, 14, geo), (46, 55, structure)],
    ));
    lines.push(plain("  │     │                                  │   │     │                              │"));
    lines.push(with_hl(
        "  │  knowledge ──── Expressions, Patterns  │   │  semantic ──── Entity, Native     │",
        vec![(5, 14, knowledge), (46, 54, semantic)],
    ));
    lines.push(plain("  │                  Taboos, Audience (21)  │   │     │                              │"));
    lines.push(with_hl(
        "  │                                        │   │  instruction ─ BlockType, Instr   │",
        vec![(46, 57, instruction)],
    ));
    lines.push(plain("  │                                        │   │     │                              │"));
    lines.push(with_hl(
        "  │                                        │   │  output ────── PageNative, Block  │",
        vec![(46, 52, output)],
    ));
    lines.push(plain("  │                                        │   │                Native, Project   │"));
    lines.push(plain("  └────────────────────────────────────────┘   └──────────────────────────────────┘"));
    lines.push(plain(""));
    lines.push(plain("  ── ARC FAMILIES (6) ──────────────────────────────────────────────────"));
    lines.push(plain(""));
    lines.push(plain("  ownership ─────── HAS_PAGE, HAS_BLOCK, HAS_ENTITY, HAS_BRAND, ..."));
    lines.push(plain("  localization ──── HAS_NATIVE, FOR_LOCALE, NATIVE_OF"));
    lines.push(plain("  semantic ──────── USES_ENTITY, ABOUT, SEMANTIC_LINK, REFERENCES_PAGE"));
    lines.push(plain("  generation ────── GENERATES, DERIVED_FROM"));
    lines.push(plain("  mining ────────── TARGETS, ANSWERS, CLUSTERS"));
    lines.push(plain("  schema ────────── OF_CLASS, FROM_CLASS, TO_CLASS"));
    lines.push(plain(""));

    FlowDiagram {
        title: "Schema Architecture".to_string(),
        lines,
        node_count: node_idx,
        node_labels,
        node_descriptions,
    }
}

/// Build the Data Pipeline diagram.
pub fn data_pipeline() -> FlowDiagram {
    let mut node_idx: usize = 0;
    let mut node_labels = Vec::new();
    let mut node_descriptions = Vec::new();
    let mut lines = Vec::new();

    let mut add_node = |label: &str, desc: &str| -> usize {
        let idx = node_idx;
        node_labels.push(label.to_string());
        node_descriptions.push(desc.to_string());
        node_idx += 1;
        idx
    };

    let project = add_node("Project", "Root container — owns Pages, Entities, Brand");
    let entity = add_node("Entity", "Semantic concept (defined). Key: kebab-case");
    let entity_native = add_node("EntityNative", "Locale-specific content (authored). Key: entity@locale");
    let page = add_node("Page", "URL structure (defined). Owns Blocks");
    let block = add_node("Block", "Content section (defined). Has BlockType + instruction");
    let block_native = add_node("BlockNative", "Generated content (LLM output). One per Block+Locale");
    let page_native = add_node("PageNative", "Assembled page (generated). Slug lives here (ADR-030)");
    let locale = add_node("Locale", "BCP-47 locale config. Voice + Culture + Knowledge");
    let brand = add_node("Brand", "Brand identity — Design, Principles, PromptStyle");
    let seo = add_node("SEOKeyword", "Imported from Ahrefs. Targets EntityNative");

    let plain = |text: &str| FlowLine { text: text.to_string(), highlights: vec![] };
    let with_hl = |text: &str, highlights: Vec<(usize, usize, usize)>| FlowLine { text: text.to_string(), highlights };

    lines.push(plain(""));
    lines.push(plain("  NovaNet Content Generation Pipeline"));
    lines.push(plain("  ═══════════════════════════════════════════════════════════════════════"));
    lines.push(plain(""));
    lines.push(plain("  CRITICAL: Generation, NOT Translation"));
    lines.push(plain("  Entity (defined) ──▶ Generate natively ──▶ EntityNative (authored)"));
    lines.push(plain(""));
    lines.push(with_hl(
        "                              Project",
        vec![(30, 37, project)],
    ));
    lines.push(plain("                           ┌────┼────────┐"));
    lines.push(plain("                 [:HAS_ENTITY] [:HAS_PAGE] [:HAS_BRAND]"));
    lines.push(plain("                           │    │          │"));
    lines.push(with_hl(
        "                        Entity  Page       Brand",
        vec![(24, 30, entity), (32, 36, page), (43, 48, brand)],
    ));
    lines.push(plain("                           │    │"));
    lines.push(plain("              [:HAS_NATIVE] │   │ [:HAS_BLOCK {order}]"));
    lines.push(plain("                           │    │"));
    lines.push(with_hl(
        "                    EntityNative  Block ──[:OF_TYPE]──▶ BlockType",
        vec![(20, 32, entity_native), (34, 39, block)],
    ));
    lines.push(plain("                           │    │"));
    lines.push(plain("                [:FOR_LOCALE]    │ [:HAS_INSTRUCTION]"));
    lines.push(plain("                           │    │"));
    lines.push(with_hl(
        "                        Locale  BlockInstruction",
        vec![(24, 30, locale)],
    ));
    lines.push(plain("                                 │"));
    lines.push(plain("                                 │ [:USES_ENTITY] ──▶ Entity"));
    lines.push(plain("                                 │"));
    lines.push(plain("  ── GENERATION PHASE ────────────────────────────────────────────────"));
    lines.push(plain(""));
    lines.push(plain("    novanet_context(mode=page) assembles:"));
    lines.push(plain("    ├── EntityNative.denomination_forms (ADR-033)"));
    lines.push(plain("    ├── Locale knowledge (Expressions, Patterns, Taboos)"));
    lines.push(plain("    ├── Brand voice (PromptStyle, BrandPrinciples)"));
    lines.push(plain("    └── Block instructions + cross-page anchors"));
    lines.push(plain(""));
    lines.push(plain("                    ┌──────────────────────────────┐"));
    lines.push(with_hl(
        "                    │      BlockNative (per block)   │",
        vec![(27, 39, block_native)],
    ));
    lines.push(plain("                    │  LLM-generated content       │"));
    lines.push(plain("                    │  with denomination_forms     │"));
    lines.push(plain("                    └──────────────────────────────┘"));
    lines.push(plain("                                 │"));
    lines.push(plain("                                 ▼ assembled"));
    lines.push(plain("                    ┌──────────────────────────────┐"));
    lines.push(with_hl(
        "                    │      PageNative               │",
        vec![(27, 37, page_native)],
    ));
    lines.push(plain("                    │  slug: /fr/code-qr (ADR-030) │"));
    lines.push(plain("                    │  meta_title, meta_description │"));
    lines.push(plain("                    └──────────────────────────────┘"));
    lines.push(plain(""));
    lines.push(plain("  ── SEO FEEDBACK LOOP ───────────────────────────────────────────────"));
    lines.push(plain(""));
    lines.push(with_hl(
        "    SEOKeyword ──[:TARGETS]──▶ EntityNative (url form write-back)",
        vec![(4, 14, seo)],
    ));
    lines.push(plain("    GEOQuery ──[:ANSWERS]──▶ GEOAnswer (mined from search)"));
    lines.push(plain(""));

    FlowDiagram {
        title: "Data Pipeline".to_string(),
        lines,
        node_count: node_idx,
        node_labels,
        node_descriptions,
    }
}
```

**Step 2: Register module in tui/mod.rs**

Add `pub mod flow;` to the module declarations.

**Step 3: Compile**

```bash
cd tools/novanet && cargo check 2>&1 | head -40
```

**Step 4: Commit**

```bash
cd tools/novanet && git add -A && git commit -m "$(cat <<'EOF'
feat(tui): add Flow diagram content — Schema + Pipeline ASCII art

Two navigable diagrams:
- Schema Architecture: Realm > Layer > Class with arc families
- Data Pipeline: Entity → EntityNative → Page → Block → BlockNative

Selectable nodes with descriptions for info panel.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 16: Create Flow UI renderer

**Files:**
- Create: `tools/novanet/src/tui/ui/flow.rs`
- Modify: `tools/novanet/src/tui/ui/mod.rs`

**Step 1: Create the renderer**

```rust
//! Flow view renderer — navigable ASCII diagrams.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};

use crate::tui::app::App;
use crate::tui::app::state::FlowTab;
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
```

**Step 2: Wire into ui/mod.rs render function**

In the main `render()` function in `ui/mod.rs`, add:

```rust
if app.mode == NavMode::Flow {
    flow::render_flow(f, app, main_area);
    return;
}
```

Add `mod flow;` at the top of `ui/mod.rs`.

**Step 3: Compile and test**

```bash
cd tools/novanet && cargo check && cargo test 2>&1 | tail -10
```

**Step 4: Commit**

```bash
cd tools/novanet && git add -A && git commit -m "$(cat <<'EOF'
feat(tui): add Flow view renderer with highlighted navigation

Full-screen ASCII diagram view with:
- Tab bar for Schema/Pipeline switching
- Highlighted selectable nodes (cyan)
- Selected node highlight (inverse cyan)
- Bottom info panel with node descriptions
- Horizontal + vertical scrolling

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 17: Update status bar for Flow mode

**Files:**
- Modify: `tools/novanet/src/tui/ui/status.rs`

**Step 1: Add Flow mode hints**

In the status bar hints function, add a match arm for `NavMode::Flow`:

```rust
NavMode::Flow => "Tab:switch j/k:scroll n/p:select h/l:pan d/u:page".to_string(),
```

**Step 2: Add Flow mode breadcrumb**

In the breadcrumb function, add:

```rust
NavMode::Flow => format!("Flow > {}", app.flow.tab.label()),
```

**Step 3: Add Flow mode indicator**

In the mode indicator rendering, add:

```rust
NavMode::Flow => "[F]",
```

**Step 4: Update tests**

Fix any tests that assert on mode count or specific mode labels.

**Step 5: Compile and test**

```bash
cd tools/novanet && cargo check && cargo test 2>&1 | tail -10
```

**Step 6: Commit**

```bash
cd tools/novanet && git add -A && git commit -m "$(cat <<'EOF'
feat(tui): add Flow mode to status bar, breadcrumb, and indicators

Status hints show Tab/j/k/n/p/h/l keys. Breadcrumb shows
"Flow > Schema Architecture" or "Flow > Data Pipeline".

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 18: Final verification — full test suite + clippy

**Step 1: Run clippy with deny warnings**

```bash
cd tools/novanet && cargo clippy -- -D warnings 2>&1 | tail -20
```

**Step 2: Run full test suite**

```bash
cd tools/novanet && cargo test 2>&1 | tail -20
```

**Step 3: Run MCP tests**

```bash
cd tools/novanet-mcp && cargo clippy -- -D warnings 2>&1 | tail -20
cd tools/novanet-mcp && cargo test 2>&1 | tail -20
```

**Step 4: Verify binary runs**

```bash
cd tools/novanet && cargo run -- --help
```

**Step 5: Count lines deleted**

```bash
git diff --stat main
```

Expected: ~13,000+ lines removed, ~600 lines added.

---

### Task 19: Final commit + push

**Step 1: Review all changes**

```bash
git log --oneline main..HEAD
git diff --stat main
```

**Step 2: Push**

```bash
git push
```

---

## Summary

| Phase | Tasks | Lines Removed | Lines Added | Commits |
|-------|-------|---------------|-------------|---------|
| 1. Delete Nexus+Views | 1-8 | ~13,300 | 0 | 1 |
| 2. Security+Bugs | 9-11 | 0 | ~30 | 3 |
| 3. Flow View | 12-18 | 0 | ~600 | 6 |
| **Total** | **19** | **~13,300** | **~630** | **10** |

Net result: ~12,670 lines removed. Cleaner, more focused TUI.
