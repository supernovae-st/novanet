# TUI Graph View Refactor — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refactor the NovaNet TUI Graph View to have clear panel separation, eliminate duplication, and clean up GOD files.

**Architecture:** 4-panel layout with distinct visual boundaries. Center splits into Identity+Provenance (top) and Data Viewer (bottom). Right splits into Properties+Stats (top) and Arcs (bottom). Extract reusable widget abstractions and split 4500+ LOC files into modules.

**Tech Stack:** Rust, ratatui, crossterm, tokio (async)

---

## Current State Analysis

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CURRENT PROBLEMS                                                               │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. LAYOUT CONFUSION                                                            │
│     └── Center panel = "Header" + "YAML" — unclear what Header contains         │
│     └── Duplication between center (Instance data) and right (Properties)       │
│     └── No visual separation between Identity/Provenance/Instance               │
│                                                                                 │
│  2. GOD FILES                                                                   │
│     └── data.rs: 4,551 LOC (15+ structs, mixed concerns)                        │
│     └── info.rs: 3,148 LOC (11 build_* + 30 utilities)                          │
│     └── nexus/mod.rs: 3,073 LOC                                                 │
│                                                                                 │
│  3. NO WIDGET ABSTRACTION                                                       │
│     └── 70+ BOX_BORDER occurrences across 18 files                              │
│     └── Duplicated focus/scroll patterns                                        │
│                                                                                 │
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Target Layout

```
┌────────────┬─────────────────────────┬──────────────────┐
│   TREE     │        MIDDLE           │      RIGHT       │
│    25%     │          40%            │       35%        │
│            │ ╔═══════════════════╗   │ ╔══════════════╗ │
│            │ ║ IDENTITY+PROVENANCE║   │ ║ PROPERTIES   ║ │
│            │ ╚═══════════════════╝   │ ║ + STATS      ║ │
│            │ ╔═══════════════════╗   │ ╚══════════════╝ │
│            │ ║ DATA VIEWER       ║   │ ╔══════════════╗ │
│            │ ║ (Neo4j OR YAML)   ║   │ ║ ARCS         ║ │
│            │ ╚═══════════════════╝   │ ╚══════════════╝ │
└────────────┴─────────────────────────┴──────────────────┘
```

**Panel contents:**
- **TREE (25%)**: Unchanged — unified tree navigation
- **IDENTITY+PROVENANCE (top middle)**: Node key, display_name, realm, layer, trait, timestamps
- **DATA VIEWER (bottom middle)**: Contextual — YAML source OR Neo4j data depending on selection
- **PROPERTIES+STATS (top right)**: All properties + stats (instance count, property count, progress bars)
- **ARCS (bottom right)**: Arc relationships

---

## Phase Overview

| Phase | Description | Files Changed | Tests |
|-------|-------------|---------------|-------|
| **1** | Widget Extraction | +2 new files | +15 tests |
| **2** | Layout Refactor | 3 files | +8 tests |
| **3** | data.rs Split | +4 new files | +20 tests |
| **4** | info.rs Split | +3 new files | +12 tests |
| **5** | State Cleanup | 2 files | +10 tests |

**Estimated commits:** 25-30 granular commits

---

## Phase 1: Widget Extraction

### Task 1.1: Create FocusablePanel widget

**Files:**
- Create: `tools/novanet/src/tui/widgets/mod.rs`
- Create: `tools/novanet/src/tui/widgets/panel.rs`
- Modify: `tools/novanet/src/tui/mod.rs:16-30` (add `pub mod widgets;`)

**Step 1: Write the failing test**

Create `tools/novanet/src/tui/widgets/panel.rs`:

```rust
//! Reusable panel widgets with focus state management.

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Widget};

/// A panel with automatic focus styling.
pub struct FocusablePanel<'a> {
    title: &'a str,
    focused: bool,
    focused_color: Color,
    unfocused_color: Color,
}

impl<'a> FocusablePanel<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            focused: false,
            focused_color: Color::Cyan,
            unfocused_color: Color::Rgb(60, 60, 70),
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn focused_color(mut self, color: Color) -> Self {
        self.focused_color = color;
        self
    }

    pub fn unfocused_color(mut self, color: Color) -> Self {
        self.unfocused_color = color;
        self
    }

    /// Build the Block with appropriate styling.
    pub fn block(&self) -> Block<'a> {
        let border_color = if self.focused {
            self.focused_color
        } else {
            self.unfocused_color
        };

        Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focusable_panel_default_unfocused() {
        let panel = FocusablePanel::new("Test");
        let block = panel.block();
        // Block should have unfocused color (gray)
        assert!(!panel.focused);
    }

    #[test]
    fn test_focusable_panel_focused_changes_style() {
        let panel = FocusablePanel::new("Test").focused(true);
        assert!(panel.focused);
    }

    #[test]
    fn test_focusable_panel_custom_colors() {
        let panel = FocusablePanel::new("Test")
            .focused_color(Color::Green)
            .unfocused_color(Color::Red);
        assert_eq!(panel.focused_color, Color::Green);
        assert_eq!(panel.unfocused_color, Color::Red);
    }
}
```

**Step 2: Create module file**

Create `tools/novanet/src/tui/widgets/mod.rs`:

```rust
//! Reusable TUI widget components.
//!
//! v0.18.3: Extracted from ui/mod.rs to reduce duplication.

mod panel;

pub use panel::FocusablePanel;
```

**Step 3: Run test to verify it compiles**

```bash
cd /Users/thibaut/dev/supernovae/novanet/tools/novanet
cargo test widgets::panel --no-run
```

Expected: Compilation succeeds

**Step 4: Run tests**

```bash
cargo test widgets::panel -v
```

Expected: 3 tests pass

**Step 5: Commit**

```bash
git add src/tui/widgets/
git commit -m "feat(tui): add FocusablePanel widget abstraction

Extracted reusable panel widget with focus state management.
Reduces BOX_BORDER duplication across 18 files.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

### Task 1.2: Create ScrollableContent widget

**Files:**
- Modify: `tools/novanet/src/tui/widgets/mod.rs`
- Create: `tools/novanet/src/tui/widgets/scrollable.rs`

**Step 1: Write the failing test**

Create `tools/novanet/src/tui/widgets/scrollable.rs`:

```rust
//! Scrollable content widget with scroll indicators.

use ratatui::text::Line;

/// Manages scroll state for content that may exceed visible area.
#[derive(Debug, Default, Clone)]
pub struct ScrollState {
    /// Current scroll position (line offset).
    pub offset: usize,
    /// Total number of content lines.
    pub total_lines: usize,
    /// Visible height in lines.
    pub visible_height: usize,
}

impl ScrollState {
    pub fn new(total_lines: usize, visible_height: usize) -> Self {
        Self {
            offset: 0,
            total_lines,
            visible_height,
        }
    }

    /// Whether content is scrollable (total > visible).
    pub fn is_scrollable(&self) -> bool {
        self.total_lines > self.visible_height
    }

    /// Maximum scroll offset.
    pub fn max_offset(&self) -> usize {
        self.total_lines.saturating_sub(self.visible_height)
    }

    /// Scroll down by N lines.
    pub fn scroll_down(&mut self, amount: usize) {
        self.offset = (self.offset + amount).min(self.max_offset());
    }

    /// Scroll up by N lines.
    pub fn scroll_up(&mut self, amount: usize) {
        self.offset = self.offset.saturating_sub(amount);
    }

    /// Get scroll indicator string.
    /// Returns "" if not scrollable, otherwise "↓ [1/N]", "↑ [N/N]", or "↕ [M/N]"
    pub fn indicator(&self) -> String {
        if !self.is_scrollable() {
            return String::new();
        }

        let current_page = self.offset + 1;
        let total_pages = self.max_offset() + 1;

        let arrow = if self.offset == 0 {
            "↓"
        } else if self.offset >= self.max_offset() {
            "↑"
        } else {
            "↕"
        };

        format!(" {} [{}/{}] ", arrow, current_page, total_pages)
    }

    /// Get visible slice of content.
    pub fn visible_lines<'a>(&self, lines: &'a [Line<'a>]) -> &'a [Line<'a>] {
        let start = self.offset.min(lines.len());
        let end = (start + self.visible_height).min(lines.len());
        &lines[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_state_not_scrollable() {
        let state = ScrollState::new(5, 10);
        assert!(!state.is_scrollable());
        assert_eq!(state.indicator(), "");
    }

    #[test]
    fn test_scroll_state_scrollable() {
        let state = ScrollState::new(20, 10);
        assert!(state.is_scrollable());
        assert_eq!(state.max_offset(), 10);
    }

    #[test]
    fn test_scroll_down() {
        let mut state = ScrollState::new(20, 10);
        state.scroll_down(5);
        assert_eq!(state.offset, 5);
    }

    #[test]
    fn test_scroll_down_clamps_to_max() {
        let mut state = ScrollState::new(20, 10);
        state.scroll_down(100);
        assert_eq!(state.offset, 10); // max_offset
    }

    #[test]
    fn test_scroll_up() {
        let mut state = ScrollState::new(20, 10);
        state.offset = 5;
        state.scroll_up(3);
        assert_eq!(state.offset, 2);
    }

    #[test]
    fn test_scroll_up_clamps_to_zero() {
        let mut state = ScrollState::new(20, 10);
        state.offset = 2;
        state.scroll_up(10);
        assert_eq!(state.offset, 0);
    }

    #[test]
    fn test_indicator_at_top() {
        let state = ScrollState::new(20, 10);
        assert!(state.indicator().contains("↓"));
        assert!(state.indicator().contains("[1/"));
    }

    #[test]
    fn test_indicator_at_bottom() {
        let mut state = ScrollState::new(20, 10);
        state.offset = 10;
        assert!(state.indicator().contains("↑"));
    }

    #[test]
    fn test_indicator_in_middle() {
        let mut state = ScrollState::new(20, 10);
        state.offset = 5;
        assert!(state.indicator().contains("↕"));
    }
}
```

**Step 2: Update module file**

Modify `tools/novanet/src/tui/widgets/mod.rs`:

```rust
//! Reusable TUI widget components.
//!
//! v0.18.3: Extracted from ui/mod.rs to reduce duplication.

mod panel;
mod scrollable;

pub use panel::FocusablePanel;
pub use scrollable::ScrollState;
```

**Step 3: Run tests**

```bash
cargo test widgets::scrollable -v
```

Expected: 9 tests pass

**Step 4: Commit**

```bash
git add src/tui/widgets/
git commit -m "feat(tui): add ScrollState widget for scroll management

Extracted scroll logic into reusable widget.
Provides indicator(), scroll_up(), scroll_down(), visible_lines().

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

### Task 1.3: Wire widgets module into TUI

**Files:**
- Modify: `tools/novanet/src/tui/mod.rs:16-30`

**Step 1: Add module declaration**

In `tools/novanet/src/tui/mod.rs`, after line 30 (after `mod yaml;`), add:

```rust
pub mod widgets;
```

**Step 2: Run full TUI tests**

```bash
cargo test tui:: --lib
```

Expected: All existing tests still pass + new widget tests pass

**Step 3: Commit**

```bash
git add src/tui/mod.rs
git commit -m "chore(tui): wire widgets module into TUI

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Phase 2: Layout Refactor

### Task 2.1: Update layout constants

**Files:**
- Modify: `tools/novanet/src/tui/ui/mod.rs:88-106`

**Step 1: Update constants**

Replace the layout constants section (lines 88-106) with:

```rust
// =============================================================================
// LAYOUT CONSTANTS (v0.18.3: New 4-panel layout)
// =============================================================================

/// Wide layout column percentages: Tree | Center | Right
const LAYOUT_TREE_PCT: u16 = 25;
const LAYOUT_CENTER_PCT: u16 = 40;
const LAYOUT_RIGHT_PCT: u16 = 35;

/// Center column split: Identity+Provenance (top) | Data Viewer (bottom)
const LAYOUT_IDENTITY_PCT: u16 = 30;
const LAYOUT_DATA_VIEWER_PCT: u16 = 70;

/// Right column split: Properties+Stats (top) | Arcs (bottom)
const LAYOUT_PROPS_STATS_PCT: u16 = 50;
const LAYOUT_ARCS_PCT: u16 = 50;

/// Narrow layout: Tree panel percentage (compact sidebar).
const LAYOUT_NARROW_TREE_PCT: u16 = 30;
/// Narrow layout: Detail panel percentage.
const LAYOUT_NARROW_DETAIL_PCT: u16 = 70;
```

**Step 2: Run tests**

```bash
cargo test tui::ui --lib
```

Expected: Tests pass (constants are internal)

**Step 3: Commit**

```bash
git add src/tui/ui/mod.rs
git commit -m "refactor(tui): update layout constants for new 4-panel design

Tree: 25% | Center: 40% | Right: 35%
Center splits: Identity (30%) + Data Viewer (70%)
Right splits: Props+Stats (50%) + Arcs (50%)

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

### Task 2.2: Create Identity+Provenance panel

**Files:**
- Create: `tools/novanet/src/tui/ui/identity_panel.rs`
- Modify: `tools/novanet/src/tui/ui/mod.rs`

**Step 1: Create identity panel**

Create `tools/novanet/src/tui/ui/identity_panel.rs`:

```rust
//! Identity + Provenance panel (top center).
//!
//! v0.18.3: New panel showing node identity and provenance info.
//! Combines what was scattered across Header and Info panels.

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::tui::app::{App, Focus};
use crate::tui::data::TreeItem;
use crate::tui::theme;

/// Render the Identity + Provenance panel.
pub fn render_identity_panel(f: &mut Frame, area: Rect, app: &App) {
    let is_focused = app.focus == Focus::Content; // Share focus with data viewer

    let border_color = if is_focused {
        theme::ui::focused_color()
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
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(class_info.display_name.clone()),
                ]),
                Line::from(vec![
                    Span::styled("Realm: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(realm.display_name.clone(), Style::default().fg(Color::Green)),
                    Span::styled(" → ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Layer: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(layer.display_name.clone(), Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::styled("Description: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        truncate(&class_info.description, 60),
                        Style::default().fg(Color::Rgb(150, 150, 150)),
                    ),
                ]),
            ]
        }
        Some(TreeItem::Instance(_, _, class_info, instance)) => {
            vec![
                Line::from(vec![
                    Span::styled("Key: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        instance.key.clone(),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(instance.display_name.clone()),
                ]),
                Line::from(vec![
                    Span::styled("Class: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(class_info.display_name.clone(), Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::styled("Provenance: ", Style::default().fg(Color::DarkGray)),
                    Span::styled("seed", Style::default().fg(Color::Magenta)), // TODO: dynamic
                ]),
            ]
        }
        Some(TreeItem::Realm(realm)) => {
            vec![
                Line::from(vec![
                    Span::styled("Realm: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Key: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(realm.key.clone()),
                ]),
            ]
        }
        Some(TreeItem::Layer(realm, layer)) => {
            vec![
                Line::from(vec![
                    Span::styled("Layer: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        layer.display_name.clone(),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Realm: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(realm.display_name.clone(), Style::default().fg(Color::Green)),
                ]),
            ]
        }
        _ => {
            vec![Line::from(Span::styled(
                "Select an item to see identity",
                Style::default().fg(Color::DarkGray),
            ))]
        }
    }
}

/// Truncate string to max length with ellipsis.
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
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
        assert!(result.ends_with('…'));
        assert!(result.len() <= 11); // 10 + ellipsis
    }
}
```

**Step 2: Add to ui/mod.rs**

After line 10 (`mod yaml_panel;`), add:

```rust
mod identity_panel;
pub use identity_panel::render_identity_panel;
```

**Step 3: Run tests**

```bash
cargo test identity_panel -v
```

Expected: 2 tests pass

**Step 4: Commit**

```bash
git add src/tui/ui/identity_panel.rs src/tui/ui/mod.rs
git commit -m "feat(tui): add Identity+Provenance panel

New panel showing node key, name, realm, layer, provenance.
Consolidates scattered identity info into single panel.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

### Task 2.3: Refactor render_main_wide for new layout

**Files:**
- Modify: `tools/novanet/src/tui/ui/mod.rs:615-662`

**Step 1: Update render_main_wide**

Replace the `render_main_wide` function (lines 615-662) with:

```rust
/// Wide layout: Tree [1] | Center (Identity+DataViewer [2]) | Right (Props+Stats [3] + Arcs [4]).
/// v0.18.3: Refactored for new 4-panel layout with clear separation.
fn render_main_wide(f: &mut Frame, area: Rect, app: &mut App) {
    // v0.16.4: Build unified content ONCE
    let content = build_unified_content(app);

    // 3-column horizontal layout: Tree | Center | Right
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(LAYOUT_TREE_PCT),
            Constraint::Percentage(LAYOUT_CENTER_PCT),
            Constraint::Percentage(LAYOUT_RIGHT_PCT),
        ])
        .split(area);

    // LEFT: Tree [1]
    render_tree(f, h_chunks[0], app);

    // CENTER: Identity+Provenance (top) + Data Viewer (bottom)
    let center_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(LAYOUT_IDENTITY_PCT),
            Constraint::Percentage(LAYOUT_DATA_VIEWER_PCT),
        ])
        .split(h_chunks[1]);

    render_identity_panel(f, center_chunks[0], app); // Identity+Provenance
    render_content_panel(f, center_chunks[1], app);  // Data Viewer [2]

    // RIGHT: Props+Stats (top) + Arcs (bottom)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(LAYOUT_PROPS_STATS_PCT),
            Constraint::Percentage(LAYOUT_ARCS_PCT),
        ])
        .split(h_chunks[2]);

    render_props_panel(f, right_chunks[0], app, &content); // Props+Stats [3]
    render_graph_panel(f, right_chunks[1], app);           // Arcs [4]

    // v0.17.3: Capture panel rects for mouse hit-testing
    app.panel_rects.tree = Some(h_chunks[0]);
    app.panel_rects.content = Some(center_chunks[1]);
    app.panel_rects.props = Some(right_chunks[0]);
    app.panel_rects.arcs = Some(right_chunks[1]);
}
```

**Step 2: Add import**

At the top of the file, add to the `pub use` section:

```rust
pub use identity_panel::render_identity_panel;
```

**Step 3: Run TUI compilation**

```bash
cargo build --features tui
```

Expected: Compiles without errors

**Step 4: Visual test (manual)**

```bash
cargo run -- tui
```

Verify: New layout shows Identity+Provenance panel at top center

**Step 5: Commit**

```bash
git add src/tui/ui/mod.rs
git commit -m "refactor(tui): wire new 4-panel layout in render_main_wide

Layout: Tree (25%) | Center (40%) | Right (35%)
Center: Identity+Provenance (30%) + Data Viewer (70%)
Right: Props+Stats (50%) + Arcs (50%)

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

### Task 2.4: Refactor render_main_narrow for new layout

**Files:**
- Modify: `tools/novanet/src/tui/ui/mod.rs:664-701`

**Step 1: Update render_main_narrow**

Replace the `render_main_narrow` function with:

```rust
/// Narrow layout: Tree [1] | Stacked (Identity, DataViewer [2], Props+Stats [3], Arcs [4]).
/// v0.18.3: Updated for 4-panel layout on smaller screens.
fn render_main_narrow(f: &mut Frame, area: Rect, app: &mut App) {
    // v0.16.4: Build unified content ONCE
    let content = build_unified_content(app);

    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(LAYOUT_NARROW_TREE_PCT),
            Constraint::Percentage(LAYOUT_NARROW_DETAIL_PCT),
        ])
        .split(area);

    render_tree(f, h_chunks[0], app);

    // Stack Identity, DataViewer, Props+Stats, Arcs vertically
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15), // Identity+Provenance
            Constraint::Percentage(30), // Data Viewer [2]
            Constraint::Percentage(30), // Props+Stats [3]
            Constraint::Percentage(25), // Arcs [4]
        ])
        .split(h_chunks[1]);

    render_identity_panel(f, v_chunks[0], app);            // Identity
    render_content_panel(f, v_chunks[1], app);             // Data Viewer [2]
    render_props_panel(f, v_chunks[2], app, &content);     // Props+Stats [3]
    render_graph_panel(f, v_chunks[3], app);               // Arcs [4]

    // v0.17.3: Capture panel rects for mouse hit-testing
    app.panel_rects.tree = Some(h_chunks[0]);
    app.panel_rects.content = Some(v_chunks[1]);
    app.panel_rects.props = Some(v_chunks[2]);
    app.panel_rects.arcs = Some(v_chunks[3]);
}
```

**Step 2: Run compilation**

```bash
cargo build --features tui
```

**Step 3: Visual test with narrow terminal**

Resize terminal to <160 cols and verify layout

**Step 4: Commit**

```bash
git add src/tui/ui/mod.rs
git commit -m "refactor(tui): update render_main_narrow for 4-panel layout

Narrow layout stacks: Identity (15%) + DataViewer (30%) + Props (30%) + Arcs (25%)

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Phase 3: data.rs Split

### Task 3.1: Extract tree types to data/types.rs

**Files:**
- Create: `tools/novanet/src/tui/data/types.rs`
- Create: `tools/novanet/src/tui/data/mod.rs`
- Modify: `tools/novanet/src/tui/mod.rs`

**Goal:** Move core type definitions (RealmInfo, LayerInfo, ClassInfo, InstanceInfo, ArcInfo, etc.) to a dedicated types module.

**Step 1: Create data/types.rs**

Create `tools/novanet/src/tui/data/types.rs` with the struct definitions extracted from data.rs (lines 1-400 approximately):

```rust
//! Core type definitions for TUI data structures.
//!
//! v0.18.3: Extracted from data.rs (was 4551 LOC).

use serde::{Deserialize, Serialize};

/// Realm information from taxonomy.yaml.
#[derive(Debug, Clone)]
pub struct RealmInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub icon: &'static str,
    pub color: String,
}

/// Layer information from taxonomy.yaml.
#[derive(Debug, Clone)]
pub struct LayerInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub color: String,
    pub realm_key: String,
}

/// Node class information.
#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub yaml_path: String,
    pub realm_key: String,
    pub layer_key: String,
    pub properties: Vec<PropertyDef>,
    pub instance_count: Option<usize>,
    pub instance_total: Option<usize>,
}

/// Property definition from YAML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDef {
    pub name: String,
    #[serde(rename = "type")]
    pub prop_type: String,
    pub required: bool,
    #[serde(default)]
    pub description: Option<String>,
}

/// Instance data loaded from Neo4j.
#[derive(Debug, Clone)]
pub struct InstanceInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub properties: Vec<(String, serde_json::Value)>,
    pub arc_count: Option<usize>,
}

/// Arc family information.
#[derive(Debug, Clone)]
pub struct ArcFamilyInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub icon: &'static str,
    pub color: String,
}

/// Arc class information.
#[derive(Debug, Clone)]
pub struct ArcClassInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub family_key: String,
    pub from_class: String,
    pub to_class: String,
    pub cardinality: String,
    pub yaml_path: String,
}

// ... (continue with remaining type definitions)
```

**Step 2: Create data/mod.rs**

```rust
//! TUI data structures and Neo4j loading.
//!
//! v0.18.3: Split from single 4551 LOC file into modules:
//! - types.rs: Core type definitions
//! - tree.rs: TaxonomyTree and TreeItem
//! - neo4j.rs: Database loading functions
//! - adrs.rs: ADR handling

mod types;
mod tree;
mod neo4j;
mod adrs;

pub use types::*;
pub use tree::*;
pub use neo4j::*;
pub use adrs::*;
```

**Note:** This is a partial example. The full implementation requires extracting ~4500 lines systematically.

**Step 3: Update mod.rs**

In `tools/novanet/src/tui/mod.rs`, change:

```rust
// Before
mod data;

// After
pub mod data;
```

**Step 4: Run tests**

```bash
cargo test tui::data -v
```

**Step 5: Commit**

```bash
git add src/tui/data/
git commit -m "refactor(tui): extract data types to data/types.rs

First step of data.rs split (4551 LOC → modules).
Extracts: RealmInfo, LayerInfo, ClassInfo, InstanceInfo, ArcInfo, etc.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

### Task 3.2: Extract tree logic to data/tree.rs

**Files:**
- Create: `tools/novanet/src/tui/data/tree.rs`

**Goal:** Move TaxonomyTree struct and TreeItem enum to dedicated module.

(Continue with similar TDD pattern...)

---

### Task 3.3: Extract Neo4j loading to data/neo4j.rs

**Files:**
- Create: `tools/novanet/src/tui/data/neo4j.rs`

**Goal:** Move all `load_*` async functions that query Neo4j.

---

### Task 3.4: Extract ADR handling to data/adrs.rs

**Files:**
- Create: `tools/novanet/src/tui/data/adrs.rs`

**Goal:** Move `get_all_adrs`, `get_architecture_diagram`, ADR parsing logic.

---

## Phase 4: info.rs Split

### Task 4.1: Extract content builders to info/builders/

**Files:**
- Create: `tools/novanet/src/tui/ui/info/mod.rs`
- Create: `tools/novanet/src/tui/ui/info/builders.rs`
- Create: `tools/novanet/src/tui/ui/info/formatters.rs`

**Goal:** Split the 11 `build_*_content` functions into a builders module, and utility functions into formatters.

---

### Task 4.2: Extract formatters to info/formatters.rs

**Goal:** Move `truncate_str`, `format_json_value`, `wrap_text`, color utilities.

---

### Task 4.3: Extract rendering to info/rendering.rs

**Goal:** Move `render_props_panel`, `render_unified_info_panel`, and related render functions.

---

## Phase 5: State Cleanup

### Task 5.1: Create NavigationState sub-struct

**Files:**
- Modify: `tools/novanet/src/tui/app/state.rs`

**Goal:** Group navigation-related fields into `NavigationState`:
- `tree_cursor`, `tree_scroll`, `tree_height`
- `mode_cursors`, `nav_history`, `nav_history_pos`
- `navigation_generation`

---

### Task 5.2: Create UiState sub-struct

**Goal:** Group UI state fields:
- `status_message`, `tick`, `panel_rects`
- `focused_property_idx`, `expanded_property`, `json_pretty`
- Scroll states (`props_scroll`, `arcs_scroll`, etc.)

---

## Summary

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  REFACTOR SUMMARY                                                             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  FILES CREATED:                                                               ║
║  ├── src/tui/widgets/mod.rs         (widget abstractions)                     ║
║  ├── src/tui/widgets/panel.rs       (FocusablePanel)                          ║
║  ├── src/tui/widgets/scrollable.rs  (ScrollState)                             ║
║  ├── src/tui/ui/identity_panel.rs   (Identity+Provenance panel)               ║
║  ├── src/tui/data/mod.rs            (data module split)                       ║
║  ├── src/tui/data/types.rs          (type definitions)                        ║
║  ├── src/tui/data/tree.rs           (TaxonomyTree)                            ║
║  ├── src/tui/data/neo4j.rs          (DB loading)                              ║
║  ├── src/tui/data/adrs.rs           (ADR handling)                            ║
║  └── src/tui/ui/info/               (info module split)                       ║
║                                                                               ║
║  LAYOUT CHANGES:                                                              ║
║  ├── Tree: 28% → 25%                                                          ║
║  ├── Center: Header+YAML → Identity+DataViewer                                ║
║  └── Right: Props+Arcs → Props+Stats / Arcs                                   ║
║                                                                               ║
║  CODE REDUCTION:                                                              ║
║  ├── data.rs: 4551 LOC → 4 modules (~1000 LOC each)                           ║
║  ├── info.rs: 3148 LOC → 3 modules (~1000 LOC each)                           ║
║  └── BOX_BORDER: 70 occurrences → FocusablePanel widget                       ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Execution Notes

**Test command after each phase:**
```bash
cargo test tui:: --lib && cargo clippy -- -D warnings
```

**Visual verification:**
```bash
cargo run -- tui
```

**Commit discipline:** One logical change per commit. Run tests before each commit.
