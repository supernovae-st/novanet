# Nexus Tree Layout Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refactor Nexus mode to have a tree panel on the left (like Views mode) instead of full-screen tabbed layout.

**Architecture:** Split Nexus into 25% tree | 75% content. Tree shows 3 sections (📖 Learn, 🔍 Explore, 🎯 Practice) with expandable tabs. Content area shows selected tab content.

**Tech Stack:** Rust, ratatui, crossterm

---

## Current State

```
┌─────────────────────────────────────────────────────────────────┐
│  LEARN          │  EXPLORE              │  PRACTICE             │  ← Tab bar (full width)
├─────────────────────────────────────────────────────────────────┤
│  [Intro breadcrumb]                                             │  ← Breadcrumb
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                    TAB CONTENT (100% width)                     │  ← Content
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  → [g] Glossary  [t] Traits  [q] Quiz                          │  ← Cross-tab hints
└─────────────────────────────────────────────────────────────────┘
```

## Target State

```
┌─────────────────────────────────────────────────────────────────┐
│ [1]Graph [2]Views [3]Nexus                    NovaNet v0.13.0   │
├───────────────────┬─────────────────────────────────────────────┤
│  NEXUS            │                                             │
│ ─────────────────│  📖 INTRO                                   │
│                   │  ────────────────────────────────────────   │
│ ▼ 📖 Learn        │                                             │
│   ▸ Intro      ◀──│  Welcome to NovaNet!                        │
│     Glossary      │                                             │
│     Tutorial      │  NovaNet generates content NATIVELY         │
│                   │  per locale (NOT translation).              │
│ ▶ 🔍 Explore      │                                             │
│                   │  Entity → Generate → EntityNative           │
│ ▶ 🎯 Practice     │                                             │
│                   │                                             │
│ ─────────────────│                                             │
│ [j/k] nav         │                                             │
│ [h/l] expand      │                                             │
│ [Enter] select    │                                             │
├───────────────────┴─────────────────────────────────────────────┤
│ → [g] Glossary  [t] Traits  [q] Quiz                            │
└─────────────────────────────────────────────────────────────────┘
```

---

## Task 1: Add NexusTreeState to NexusState

**Files:**
- Modify: `/Users/thibaut/supernovae-st/novanet-hq/tools/novanet/src/tui/nexus/mod.rs`

**Step 1: Add section enum and tree state struct**

After line 108 (after `NexusTab` enum), add:

```rust
/// Nexus tree sections with emojis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NexusSection {
    #[default]
    Learn,
    Explore,
    Practice,
}

impl NexusSection {
    /// Get emoji for this section.
    pub fn emoji(&self) -> &'static str {
        match self {
            NexusSection::Learn => "📖",
            NexusSection::Explore => "🔍",
            NexusSection::Practice => "🎯",
        }
    }

    /// Get label for this section.
    pub fn label(&self) -> &'static str {
        match self {
            NexusSection::Learn => "Learn",
            NexusSection::Explore => "Explore",
            NexusSection::Practice => "Practice",
        }
    }

    /// Get tabs in this section.
    pub fn tabs(&self) -> &'static [NexusTab] {
        match self {
            NexusSection::Learn => &[NexusTab::Intro, NexusTab::Glossary, NexusTab::Tutorial],
            NexusSection::Explore => &[NexusTab::Traits, NexusTab::Layers, NexusTab::Arcs, NexusTab::Arch],
            NexusSection::Practice => &[NexusTab::Pipeline, NexusTab::Quiz, NexusTab::Stats],
        }
    }

    /// Get all sections.
    pub fn all() -> &'static [NexusSection] {
        &[NexusSection::Learn, NexusSection::Explore, NexusSection::Practice]
    }

    /// Next section (wraps).
    pub fn next(&self) -> NexusSection {
        match self {
            NexusSection::Learn => NexusSection::Explore,
            NexusSection::Explore => NexusSection::Practice,
            NexusSection::Practice => NexusSection::Learn,
        }
    }

    /// Previous section (wraps).
    pub fn prev(&self) -> NexusSection {
        match self {
            NexusSection::Learn => NexusSection::Practice,
            NexusSection::Explore => NexusSection::Learn,
            NexusSection::Practice => NexusSection::Explore,
        }
    }
}
```

**Step 2: Add tree state fields to NexusState**

Find `NexusState` struct (around line 342) and add these fields:

```rust
    // Tree navigation state (v0.13.0)
    /// Which section is currently focused in the tree.
    pub tree_section: NexusSection,
    /// Whether each section is expanded [Learn, Explore, Practice].
    pub tree_expanded: [bool; 3],
    /// Cursor position within the current section (0 = section header, 1+ = tabs).
    pub tree_cursor: usize,
```

**Step 3: Initialize tree state in NexusState::default()**

Find `impl Default for NexusState` and add initialization:

```rust
    tree_section: NexusSection::Learn,
    tree_expanded: [true, false, false], // Learn expanded by default
    tree_cursor: 1, // First tab selected
```

**Step 4: Run tests to verify compilation**

Run: `cargo test -p novanet --lib nexus`
Expected: All existing tests pass

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/nexus/mod.rs
git commit -m "feat(tui): add NexusSection enum and tree state to NexusState

- Add NexusSection enum with emoji(), label(), tabs() methods
- Add tree navigation fields to NexusState
- Prepare for tree layout refactor

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 2: Add NexusTab.emoji() method

**Files:**
- Modify: `/Users/thibaut/supernovae-st/novanet-hq/tools/novanet/src/tui/nexus/mod.rs`

**Step 1: Add emoji method to NexusTab impl**

Find `impl NexusTab` (around line 111) and add after `label()` method:

```rust
    /// Get emoji for this tab.
    pub fn emoji(&self) -> &'static str {
        match self {
            // LEARN section
            NexusTab::Intro => "🌟",
            NexusTab::Glossary => "📚",
            NexusTab::Tutorial => "🎓",
            // EXPLORE section
            NexusTab::Traits => "🏷️",
            NexusTab::Layers => "📊",
            NexusTab::Arcs => "🔗",
            NexusTab::Arch => "🏛️",
            // PRACTICE section
            NexusTab::Pipeline => "⚡",
            NexusTab::Quiz => "❓",
            NexusTab::Stats => "📈",
        }
    }
```

**Step 2: Run tests**

Run: `cargo test -p novanet --lib nexus`
Expected: All tests pass

**Step 3: Commit**

```bash
git add tools/novanet/src/tui/nexus/mod.rs
git commit -m "feat(tui): add emoji() method to NexusTab

- Each tab now has an emoji for tree display
- Learn: 🌟📚🎓, Explore: 🏷️📊🔗🏛️, Practice: ⚡❓📈

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 3: Create render_nexus_tree() function

**Files:**
- Modify: `/Users/thibaut/supernovae-st/novanet-hq/tools/novanet/src/tui/nexus/mod.rs`

**Step 1: Add render_nexus_tree function**

Add before `render_nexus()` function (around line 1430):

```rust
/// Render the Nexus tree panel (left side).
/// Shows 3 sections with expandable tabs.
fn render_nexus_tree(f: &mut Frame, area: Rect, app: &App) {
    use ratatui::widgets::{Paragraph, Wrap};

    let block = Block::default()
        .title(Span::styled(
            " NEXUS ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    // Header
    lines.push(Line::from(vec![
        Span::styled(
            "Learning Hub ",
            Style::default().fg(Color::Rgb(180, 180, 180)),
        ),
        Span::styled("v0.13.0", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(Span::styled(
        "\u{2500}".repeat(inner.width.saturating_sub(2) as usize),
        Style::default().fg(COLOR_UNFOCUSED_BORDER),
    )));
    lines.push(Line::from(""));

    // Current section for comparison
    let current_section = app.nexus.tree_section;
    let current_tab = app.nexus.tab;

    // Render each section
    for (sec_idx, section) in NexusSection::all().iter().enumerate() {
        let is_current_section = *section == current_section;
        let is_expanded = app.nexus.tree_expanded[sec_idx];

        // Section header
        let expand_icon = if is_expanded { "▼" } else { "▶" };
        let section_style = if is_current_section && app.nexus.tree_cursor == 0 {
            Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(50, 30, 70))
                .add_modifier(Modifier::BOLD)
        } else if is_current_section {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(150, 150, 150))
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{} ", expand_icon), section_style),
            Span::styled(format!("{} ", section.emoji()), section_style),
            Span::styled(section.label(), section_style),
        ]));

        // Tabs in section (only if expanded)
        if is_expanded {
            for (tab_idx, tab) in section.tabs().iter().enumerate() {
                let is_selected = *tab == current_tab;
                let is_cursor = is_current_section && app.nexus.tree_cursor == tab_idx + 1;

                let prefix = if is_selected { "  ▸ " } else { "    " };

                let style = if is_cursor {
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Rgb(30, 50, 70))
                        .add_modifier(Modifier::BOLD)
                } else if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Rgb(180, 180, 180))
                };

                lines.push(Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(format!("{} ", tab.emoji()), style),
                    Span::styled(tab.label(), style),
                ]));
            }
        }

        lines.push(Line::from(""));
    }

    // Navigation hints
    lines.push(Line::from(Span::styled(
        "─".repeat(inner.width.saturating_sub(2) as usize),
        Style::default().fg(COLOR_UNFOCUSED_BORDER),
    )));
    lines.push(Line::from(vec![
        Span::styled("[j/k] ", Style::default().fg(Color::Cyan)),
        Span::styled("nav", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("[h/l] ", Style::default().fg(Color::Cyan)),
        Span::styled("expand", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("[Enter] ", Style::default().fg(Color::Cyan)),
        Span::styled("select", Style::default().fg(Color::DarkGray)),
    ]));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}
```

**Step 2: Run clippy to check for warnings**

Run: `cargo clippy -p novanet -- -D warnings`
Expected: No warnings

**Step 3: Commit**

```bash
git add tools/novanet/src/tui/nexus/mod.rs
git commit -m "feat(tui): add render_nexus_tree() function

- Tree panel with 3 sections: 📖 Learn, 🔍 Explore, 🎯 Practice
- Expandable sections with ▼/▶ icons
- Tab emojis and selection highlighting
- Navigation hints at bottom

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 4: Refactor render_nexus() to use tree layout

**Files:**
- Modify: `/Users/thibaut/supernovae-st/novanet-hq/tools/novanet/src/tui/nexus/mod.rs`

**Step 1: Replace render_nexus() function**

Replace the entire `render_nexus()` function (around line 1437-1474) with:

```rust
/// Render Nexus mode with tree layout (25% tree | 75% content).
pub fn render_nexus(f: &mut Frame, area: Rect, app: &App) {
    // Split into tree (25%) and content (75%)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    // Left: Tree panel
    render_nexus_tree(f, main_chunks[0], app);

    // Right: Content with cross-tab hints at bottom
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Content
            Constraint::Length(1), // Cross-tab hints
        ])
        .split(main_chunks[1]);

    // Render content based on active tab
    match app.nexus.tab {
        // LEARN section
        NexusTab::Intro => intro::render_intro_tab(f, app, content_chunks[0]),
        NexusTab::Glossary => glossary::render_glossary_tab(f, app, content_chunks[0]),
        NexusTab::Tutorial => tutorial::render_tutorial_tab(f, app, content_chunks[0]),
        // EXPLORE section
        NexusTab::Traits => traits::render_traits_tab(f, app, content_chunks[0]),
        NexusTab::Layers => layers::render_layers_tab(f, app, content_chunks[0]),
        NexusTab::Arcs => arcs::render_arcs_tab(f, app, content_chunks[0]),
        NexusTab::Arch => arch::render_arch_tab(f, app, content_chunks[0]),
        // PRACTICE section
        NexusTab::Pipeline => pipeline::render_pipeline_tab(f, app, content_chunks[0]),
        NexusTab::Quiz => quiz::render_quiz_tab(f, app, content_chunks[0]),
        NexusTab::Stats => stats::render_stats_tab(f, app, content_chunks[0]),
    }

    // Render cross-tab navigation hints
    render_cross_tab_hints(f, content_chunks[1], app);
}
```

**Step 2: Remove render_tab_bar() and render_breadcrumb() calls**

The old tab bar and breadcrumb are no longer needed. They can be deleted or left as dead code for now.

**Step 3: Run cargo build**

Run: `cargo build -p novanet`
Expected: Build succeeds

**Step 4: Test visually**

Run: `cargo run -p novanet -- tui` then press `3` to go to Nexus
Expected: See tree on left, content on right

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/nexus/mod.rs
git commit -m "feat(tui): refactor render_nexus() to tree layout

- Replace full-screen tabbed with 25% tree | 75% content
- Remove tab bar and breadcrumb (replaced by tree)
- Keep cross-tab hints at bottom of content area

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 5: Update handle_nexus_key() for tree navigation

**Files:**
- Modify: `/Users/thibaut/supernovae-st/novanet-hq/tools/novanet/src/tui/handlers/nexus.rs`

**Step 1: Read the current handler**

Read the file to understand current key handling.

**Step 2: Add tree navigation keys**

Add these key handlers:

```rust
// Tree navigation (new)
KeyCode::Char('j') | KeyCode::Down => {
    // Move cursor down in tree
    let section = app.nexus.tree_section;
    let sec_idx = section as usize;
    let is_expanded = app.nexus.tree_expanded[sec_idx];
    let tab_count = section.tabs().len();

    if is_expanded {
        // Within expanded section: move through tabs
        if app.nexus.tree_cursor < tab_count {
            app.nexus.tree_cursor += 1;
        } else {
            // Move to next section
            app.nexus.tree_section = section.next();
            app.nexus.tree_cursor = 0;
        }
    } else {
        // Section collapsed: move to next section
        app.nexus.tree_section = section.next();
        app.nexus.tree_cursor = 0;
    }
    HandlerResult::Consumed
}

KeyCode::Char('k') | KeyCode::Up => {
    // Move cursor up in tree
    let section = app.nexus.tree_section;

    if app.nexus.tree_cursor > 0 {
        app.nexus.tree_cursor -= 1;
    } else {
        // Move to previous section
        let prev_section = section.prev();
        let prev_idx = prev_section as usize;
        app.nexus.tree_section = prev_section;
        if app.nexus.tree_expanded[prev_idx] {
            app.nexus.tree_cursor = prev_section.tabs().len();
        } else {
            app.nexus.tree_cursor = 0;
        }
    }
    HandlerResult::Consumed
}

KeyCode::Char('h') | KeyCode::Left => {
    // Collapse current section
    let sec_idx = app.nexus.tree_section as usize;
    app.nexus.tree_expanded[sec_idx] = false;
    app.nexus.tree_cursor = 0;
    HandlerResult::Consumed
}

KeyCode::Char('l') | KeyCode::Right => {
    // Expand current section
    let sec_idx = app.nexus.tree_section as usize;
    app.nexus.tree_expanded[sec_idx] = true;
    HandlerResult::Consumed
}

KeyCode::Enter => {
    // Select tab under cursor
    if app.nexus.tree_cursor > 0 {
        let section = app.nexus.tree_section;
        let tabs = section.tabs();
        if let Some(tab) = tabs.get(app.nexus.tree_cursor - 1) {
            app.nexus.tab = *tab;
        }
    }
    HandlerResult::Consumed
}
```

**Step 3: Run tests**

Run: `cargo test -p novanet --lib handlers`
Expected: All tests pass

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/handlers/nexus.rs
git commit -m "feat(tui): add tree navigation to Nexus key handler

- j/k: move cursor up/down through sections and tabs
- h/l: collapse/expand sections
- Enter: select tab under cursor
- Keep existing letter shortcuts (i/g/u/t/l/a/A/p/q/s)

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 6: Sync tree state with tab selection

**Files:**
- Modify: `/Users/thibaut/supernovae-st/novanet-hq/tools/novanet/src/tui/nexus/mod.rs`

**Step 1: Add helper method to NexusTab**

Add to `impl NexusTab`:

```rust
    /// Get which section this tab belongs to.
    pub fn section(&self) -> NexusSection {
        match self {
            NexusTab::Intro | NexusTab::Glossary | NexusTab::Tutorial => NexusSection::Learn,
            NexusTab::Traits | NexusTab::Layers | NexusTab::Arcs | NexusTab::Arch => NexusSection::Explore,
            NexusTab::Pipeline | NexusTab::Quiz | NexusTab::Stats => NexusSection::Practice,
        }
    }

    /// Get index within section.
    pub fn index_in_section(&self) -> usize {
        match self {
            NexusTab::Intro | NexusTab::Traits | NexusTab::Pipeline => 0,
            NexusTab::Glossary | NexusTab::Layers | NexusTab::Quiz => 1,
            NexusTab::Tutorial | NexusTab::Arcs | NexusTab::Stats => 2,
            NexusTab::Arch => 3,
        }
    }
```

**Step 2: Update NexusState to sync tree when tab changes**

Add method to `impl NexusState`:

```rust
    /// Sync tree state when tab is changed via shortcut.
    pub fn sync_tree_to_tab(&mut self) {
        let section = self.tab.section();
        let sec_idx = section as usize;

        self.tree_section = section;
        self.tree_expanded[sec_idx] = true;
        self.tree_cursor = self.tab.index_in_section() + 1;
    }
```

**Step 3: Call sync in handlers when tab changes via shortcut**

In handlers/nexus.rs, after setting `app.nexus.tab = NexusTab::X`, add:
```rust
app.nexus.sync_tree_to_tab();
```

**Step 4: Run full test suite**

Run: `cargo nextest run -p novanet`
Expected: All tests pass

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/nexus/mod.rs tools/novanet/src/tui/handlers/nexus.rs
git commit -m "feat(tui): sync tree state with tab shortcuts

- Add section() and index_in_section() methods to NexusTab
- Add sync_tree_to_tab() to keep tree and shortcuts in sync
- Tree auto-expands when using letter shortcuts (i/g/u/t/l/a/A/p/q/s)

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 7: Clean up dead code and run final tests

**Files:**
- Modify: `/Users/thibaut/supernovae-st/novanet-hq/tools/novanet/src/tui/nexus/mod.rs`

**Step 1: Remove or mark unused functions**

The following may be unused after refactor:
- `render_tab_bar()` - old tab bar
- `render_breadcrumb()` - old breadcrumb

Either delete them or add `#[allow(dead_code)]` if they might be useful later.

**Step 2: Run clippy**

Run: `cargo clippy -p novanet -- -D warnings`
Expected: No warnings

**Step 3: Run full test suite**

Run: `cargo nextest run -p novanet`
Expected: All tests pass

**Step 4: Visual test**

Run: `cargo run -p novanet -- tui`
Test:
- Press `3` to go to Nexus
- See tree on left with sections
- Press `j/k` to navigate
- Press `l` to expand section
- Press `Enter` to select tab
- Press `i` to jump to Intro (verify tree syncs)

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/nexus/mod.rs
git commit -m "chore(tui): clean up dead code after Nexus refactor

- Remove unused render_tab_bar() and render_breadcrumb()
- All clippy warnings resolved
- All tests passing

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Summary

| Task | Description | Files |
|------|-------------|-------|
| 1 | Add NexusSection enum and tree state | nexus/mod.rs |
| 2 | Add emoji() to NexusTab | nexus/mod.rs |
| 3 | Create render_nexus_tree() | nexus/mod.rs |
| 4 | Refactor render_nexus() layout | nexus/mod.rs |
| 5 | Update key handler for tree nav | handlers/nexus.rs |
| 6 | Sync tree state with shortcuts | nexus/mod.rs, handlers/nexus.rs |
| 7 | Clean up and final tests | nexus/mod.rs |

**Total commits:** 7
**Estimated time:** 2-3 hours

---

## Verification Checklist

- [ ] `cargo clippy -p novanet -- -D warnings` passes
- [ ] `cargo nextest run -p novanet` passes (all tests)
- [ ] TUI shows tree layout in Nexus mode
- [ ] j/k navigation works
- [ ] h/l expand/collapse works
- [ ] Enter selects tab
- [ ] Letter shortcuts (i/g/u/t/l/a/A/p/q/s) still work and sync tree
- [ ] Emojis display correctly (📖 🔍 🎯 🌟 📚 🎓 🏷️ 📊 🔗 🏛️ ⚡ ❓ 📈)
