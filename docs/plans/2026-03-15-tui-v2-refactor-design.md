# TUI v2 Refactor Design

**Date**: 2026-03-15
**Version**: v0.20.0 post
**Scope**: Visual polish, structural splits, cleanup, widget extraction, UX improvements
**Status**: APPROVED

---

## Context

The NovaNet TUI (34 files, ~28,877 lines) has accumulated technical debt across its rapid
feature development. A comprehensive audit identified:

- **216 inline `Color::Rgb()`** values across 14 files, **100+ duplicates**
- **19 Block builder chains** repeating the same 4-line pattern
- **95+ `Line::from(Span::styled(...))`** single-span patterns
- **73 `Style::default().fg().add_modifier(BOLD)`** chains
- **20+ badge patterns** `[Label]` across 6 files
- **12+ progress bar patterns** `"block".repeat(n)` across 5 files
- **2 duplicate scroll_indicator** implementations
- **3 files over 1,500 lines** (tree.rs: 2,205, yaml_panel.rs: 1,960, info/builders.rs: 1,847)

## Research Sources

- Ratatui v0.30.0 official docs: const Style, border merging, WidgetRef, builder-lite
- Production apps: gitui (21.6k stars), yazi (34k stars), bottom
- TEA vs Component Architecture patterns
- tachyonfx animation library

---

## Phase 1: Centralized Color Palette (`palette.rs`)

**Goal**: Replace 216 inline `Color::Rgb()` with named constants.
**Impact**: 100+ duplicate eliminations, single source of truth for all colors.

### Create `src/tui/palette.rs`

```rust
//! Centralized color palette for the NovaNet TUI.
//!
//! All colors used across the TUI are defined here as `const` values.
//! NO inline `Color::Rgb()` should exist outside this module.

use ratatui::style::Color;

// ── Semantic Status ──────────────────────────────────────────────

pub const SUCCESS: Color = Color::Rgb(34, 197, 94);     // green-500
pub const WARNING: Color = Color::Rgb(249, 115, 22);    // orange-500
pub const ERROR: Color = Color::Rgb(239, 68, 68);       // red-500
pub const INFO: Color = Color::Rgb(59, 130, 246);       // blue-500

// ── Solarized Base ───────────────────────────────────────────────

pub const SOLARIZED_CYAN: Color = Color::Rgb(42, 161, 152);
pub const SOLARIZED_VIOLET: Color = Color::Rgb(108, 113, 196);
pub const SOLARIZED_GREEN: Color = Color::Rgb(133, 153, 0);
pub const SOLARIZED_ORANGE: Color = Color::Rgb(203, 75, 22);
pub const SOLARIZED_RED: Color = Color::Rgb(220, 50, 47);
pub const SOLARIZED_BLUE: Color = Color::Rgb(38, 139, 210);
pub const SOLARIZED_MAGENTA: Color = Color::Rgb(211, 54, 130);
pub const SOLARIZED_GOLD: Color = Color::Rgb(181, 137, 0);

// ── Nord Theme ───────────────────────────────────────────────────

pub const NORD_FROST: Color = Color::Rgb(136, 192, 208);
pub const NORD_AURORA_GREEN: Color = Color::Rgb(163, 190, 140);
pub const NORD_BORDER_UNFOCUSED: Color = Color::Rgb(59, 66, 82);
pub const NORD_BORDER_FOCUSED: Color = Color::Rgb(76, 86, 106);

// ── Tailwind-500 ─────────────────────────────────────────────────

pub const GREEN_500: Color = Color::Rgb(34, 197, 94);
pub const BLUE_500: Color = Color::Rgb(59, 130, 246);
pub const ORANGE_500: Color = Color::Rgb(249, 115, 22);
pub const VIOLET_500: Color = Color::Rgb(139, 92, 246);
pub const RED_500: Color = Color::Rgb(239, 68, 68);
pub const YELLOW_500: Color = Color::Rgb(234, 179, 8);
pub const CYAN_500: Color = Color::Rgb(6, 182, 212);
pub const PINK_500: Color = Color::Rgb(236, 72, 153);
pub const PURPLE_500: Color = Color::Rgb(168, 85, 247);
pub const SLATE_500: Color = Color::Rgb(100, 116, 139);

// ── Grays (Background/Border scale) ─────────────────────────────

pub const BG_DARK: Color = Color::Rgb(15, 15, 20);
pub const BG_OVERLAY: Color = Color::Rgb(20, 20, 30);
pub const BG_EMPTY: Color = Color::Rgb(25, 25, 35);
pub const BG_ACTIVE: Color = Color::Rgb(25, 35, 45);
pub const BG_HIGHLIGHT: Color = Color::Rgb(30, 40, 50);
pub const BG_SEARCH: Color = Color::Rgb(30, 50, 70);
pub const BG_SELECTED: Color = Color::Rgb(30, 35, 50);
pub const BG_PROPERTY_FOCUSED: Color = Color::Rgb(30, 50, 80);
pub const BORDER_UNFOCUSED: Color = Color::Rgb(60, 60, 70);
pub const SEPARATOR: Color = Color::Rgb(70, 70, 80);
pub const HINT_TEXT: Color = Color::Rgb(80, 80, 100);
pub const DIM: Color = Color::Rgb(100, 100, 100);
pub const DIM_110: Color = Color::Rgb(100, 100, 110);
pub const MUTED: Color = Color::Rgb(100, 100, 120);
pub const MUTED_130: Color = Color::Rgb(130, 130, 140);
pub const BRIGHT_DIM: Color = Color::Rgb(140, 140, 140);
pub const DESC_TEXT: Color = Color::Rgb(150, 150, 150);
pub const COUNT_TEXT: Color = Color::Rgb(180, 180, 180);
pub const FILE_TEXT: Color = Color::Rgb(180, 180, 200);

// ── Realm colors ─────────────────────────────────────────────────

pub const REALM_SHARED: Color = SOLARIZED_CYAN;
pub const REALM_ORG: Color = SOLARIZED_VIOLET;

// ── Layer colors ─────────────────────────────────────────────────

pub const LAYER_CONFIG: Color = BLUE_500;
pub const LAYER_LOCALE: Color = PINK_500;
pub const LAYER_GEOGRAPHY: Color = GREEN_500;
pub const LAYER_KNOWLEDGE: Color = VIOLET_500;
pub const LAYER_FOUNDATION: Color = PURPLE_500;
pub const LAYER_STRUCTURE: Color = BLUE_500;
pub const LAYER_SEMANTIC: Color = ORANGE_500;
pub const LAYER_INSTRUCTION: Color = SOLARIZED_GOLD;
pub const LAYER_OUTPUT: Color = GREEN_500;

// ── Arc family colors ────────────────────────────────────────────

pub const FAMILY_OWNERSHIP: Color = BLUE_500;
pub const FAMILY_SEMANTIC: Color = ORANGE_500;
pub const FAMILY_GENERATION: Color = SOLARIZED_GOLD;
pub const FAMILY_LOCALIZATION: Color = GREEN_500;
pub const FAMILY_MINING: Color = VIOLET_500;
pub const FAMILY_LABEL: Color = Color::Rgb(180, 140, 80);

// ── YAML syntax highlighting ─────────────────────────────────────

pub const YAML_KEY: Color = Color::Rgb(86, 182, 194);
pub const YAML_STRING: Color = Color::Rgb(229, 192, 123);
pub const YAML_NUMBER: Color = Color::Rgb(209, 154, 102);
pub const YAML_BOOL: Color = Color::Rgb(198, 120, 221);
pub const YAML_COMMENT: Color = SOLARIZED_VIOLET;
pub const YAML_BRACKET: Color = Color::Rgb(97, 175, 239);
pub const YAML_SECTION_HEADER: Color = Color::Rgb(92, 99, 112);

// ── Data value type colors ───────────────────────────────────────

pub const VALUE_BOOL: Color = Color::Rgb(189, 147, 249);
pub const VALUE_NUMBER: Color = Color::Rgb(249, 226, 175);
pub const VALUE_STRING: Color = Color::Rgb(166, 227, 161);
pub const VALUE_ARRAY: Color = Color::Rgb(137, 180, 250);
pub const VALUE_OBJECT: Color = Color::Rgb(245, 194, 231);

// ── Specialty ────────────────────────────────────────────────────

pub const PROP_KEY: Color = Color::Rgb(139, 233, 253);
pub const ENTITY_SLUG: Color = Color::Rgb(148, 163, 184);
pub const STATUS_BAR_BG: Color = BG_DARK;
pub const EMPTY_SLOT: Color = Color::Rgb(40, 40, 50);
```

### Tasks

| # | File | Action | Lines affected |
|---|------|--------|----------------|
| 1.1 | `palette.rs` | Create module with all constants | ~120 new lines |
| 1.2 | `mod.rs` (tui) | Add `pub mod palette;` | 1 line |
| 1.3 | `ui/mod.rs` | Replace 11 inline `COLOR_*` constants with `palette::*` | 11 consts |
| 1.4 | `ui/yaml_panel.rs` | Replace 54 inline `Color::Rgb()` with `palette::*` | ~54 lines |
| 1.5 | `ui/graph.rs` | Replace 22 inline `Color::Rgb()` | ~22 lines |
| 1.6 | `ui/info/mod.rs` | Replace 27 inline colors | ~27 lines |
| 1.7 | `ui/info/builders.rs` | Replace 26 inline colors | ~26 lines |
| 1.8 | `ui/tree.rs` | Replace 16 inline colors | ~16 lines |
| 1.9 | `ui/identity_panel.rs` | Replace 24 inline colors | ~24 lines |
| 1.10 | `ui/overlays.rs` | Replace 4 inline colors | ~4 lines |
| 1.11 | `ui/flow.rs` | Replace inline colors | ~6 lines |
| 1.12 | `ui/status.rs` | Replace 1 inline color | ~1 line |
| 1.13 | `widgets/panel.rs` | Replace 1 inline color | ~1 line |
| 1.14 | Remove per-file `const COLOR_*` | Delete redundant per-file constants | ~40 lines removed |

**Verification**: `cargo check && cargo clippy && grep -rn 'Color::Rgb' src/tui/ | grep -v palette | grep -v generated | grep -v tests | grep -v theme`

**Commit**: `refactor(tui): centralize color palette (216 inline → constants)`

---

## Phase 2: Widget Extraction

**Goal**: Extract 4 reusable widgets from repeated patterns.

### 2A: Badge Widget (`widgets/badge.rs`)

```rust
//! Colored badge widget: [Label] or Label with semantic styling.

pub struct Badge<'a> {
    text: &'a str,
    style: Style,
    brackets: bool,
}

impl<'a> Badge<'a> {
    pub fn new(text: &'a str) -> Self { ... }
    pub fn bracketed(mut self) -> Self { self.brackets = true; self }
    pub fn style(mut self, style: Style) -> Self { ... }
}

// Returns Line<'static> for composability
impl<'a> Badge<'a> {
    pub fn to_span(&self) -> Span<'static> { ... }
}
```

**Consolidates**: 20+ badge patterns across `graph.rs`, `info/builders.rs`, `yaml_panel.rs`, `status.rs`, `info/mod.rs`.

| # | Task |
|---|------|
| 2A.1 | Create `widgets/badge.rs` with Badge struct |
| 2A.2 | Add tests |
| 2A.3 | Replace badge patterns in `graph.rs` (4 locations) |
| 2A.4 | Replace badge patterns in `info/builders.rs` (6 locations) |
| 2A.5 | Replace badge patterns in `yaml_panel.rs` (2 locations) |
| 2A.6 | Replace badge patterns in `status.rs` (2 locations) |

**Commit**: `refactor(tui): extract Badge widget (20+ inline → reusable)`

### 2B: ProgressBar Widget (`widgets/progress_bar.rs`)

```rust
//! Reusable progress bar: ████░░░░ with configurable characters.

pub struct ProgressBar {
    ratio: f64,
    width: u16,
    filled_char: &'static str,
    empty_char: &'static str,
    filled_style: Style,
    empty_style: Style,
}

impl ProgressBar {
    pub fn new(ratio: f64, width: u16) -> Self { ... }
    pub fn chars(mut self, filled: &'static str, empty: &'static str) -> Self { ... }
    pub fn to_spans(&self) -> Vec<Span<'static>> { ... }
}
```

**Consolidates**: 12+ patterns across `info/builders.rs` (8), `tree.rs` (2), `graph.rs` (2), `identity_panel.rs` (1).

| # | Task |
|---|------|
| 2B.1 | Create `widgets/progress_bar.rs` |
| 2B.2 | Add tests (including edge cases: 0%, 100%, empty width) |
| 2B.3 | Replace in `info/builders.rs` (8 patterns) |
| 2B.4 | Replace in `tree.rs` (2 patterns) |
| 2B.5 | Replace in `graph.rs` (2 patterns) |
| 2B.6 | Replace in `identity_panel.rs` (1 pattern) |

**Commit**: `refactor(tui): extract ProgressBar widget (12+ inline → reusable)`

### 2C: Scroll Indicator Consolidation

| # | Task |
|---|------|
| 2C.1 | Delete duplicate `widgets/scrollable.rs::scroll_indicator` |
| 2C.2 | Move canonical impl to `widgets/scrollable.rs` (keep the module) |
| 2C.3 | Update all callers to use single implementation |

**Commit**: `refactor(tui): consolidate scroll_indicator (2 impls → 1)`

### 2D: `FocusableBlock` helper function

Extend `widgets/panel.rs` with a standalone function:

```rust
/// Create a Block with consistent border styling.
pub fn bordered_block(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
}

/// Create a Block with focus-aware border color.
pub fn focus_block(title: &str, focused: bool) -> Block<'_> {
    bordered_block(title).border_style(Style::default().fg(
        if focused { Color::Cyan } else { palette::BORDER_UNFOCUSED }
    ))
}
```

| # | Task |
|---|------|
| 2D.1 | Add `bordered_block()` and `focus_block()` to `widgets/panel.rs` |
| 2D.2 | Replace 19 Block builder chains across all files |

**Commit**: `refactor(tui): add bordered_block/focus_block helpers (19 chains → 2 fns)`

---

## Phase 3: File Splits (Structural)

**Goal**: No file over 1,500 lines. Split by concern.

### 3A: Split `tree.rs` (2,205 lines)

```
ui/tree.rs (2,205L) → ui/tree/
  mod.rs          ← render_tree() entry + layout (~200L)
  header.rs       ← render_header, render_realm_header, render_layer_header (~400L)
  node_rows.rs    ← render_node_line, render_class_line, render_instance_line (~800L)
  separators.rs   ← render_separator, render_section_divider (~200L)
  power_bar.rs    ← render_power_bar, power color logic (~150L)
  scroll.rs       ← tree scroll handling, scroll indicators (~200L)
  tests.rs        ← all tree tests (~200L)
```

| # | Task |
|---|------|
| 3A.1 | Create `ui/tree/` directory |
| 3A.2 | Extract `header.rs` (realm + layer headers) |
| 3A.3 | Extract `node_rows.rs` (class + instance rendering) |
| 3A.4 | Extract `separators.rs` + `power_bar.rs` |
| 3A.5 | Extract `scroll.rs` |
| 3A.6 | Create `mod.rs` with re-exports |
| 3A.7 | Move tests to `tests.rs` |

**Commit**: `refactor(tui): split tree.rs into 7 modules (2205L → max 800L)`

### 3B: Split `yaml_panel.rs` (1,960 lines)

```
ui/yaml_panel.rs (1,960L) → ui/yaml_panel/
  mod.rs          ← render_yaml_panel() entry + layout (~200L)
  syntax.rs       ← YAML syntax highlighting, render_yaml_content (~500L)
  arc_detail.rs   ← render_arc_detail, arc badge colors (~300L)
  color_maps.rs   ← layer_color(), scope_color(), cardinality_color() (~150L)
  empty_states.rs ← render_no_selection, render_error (~100L)
  tests.rs        ← all YAML panel tests (~300L)
```

| # | Task |
|---|------|
| 3B.1 | Create `ui/yaml_panel/` directory |
| 3B.2 | Extract `syntax.rs` (YAML rendering) |
| 3B.3 | Extract `arc_detail.rs` (arc-specific rendering) |
| 3B.4 | Extract `color_maps.rs` (layer/scope/cardinality colors) |
| 3B.5 | Extract `empty_states.rs` |
| 3B.6 | Create `mod.rs` with re-exports |
| 3B.7 | Move tests to `tests.rs` |

**Commit**: `refactor(tui): split yaml_panel.rs into 6 modules (1960L → max 500L)`

### 3C: Split `info/builders.rs` (1,847 lines)

```
ui/info/builders.rs (1,847L) → ui/info/builders/
  mod.rs              ← builder dispatch (~200L)
  node_class.rs       ← NodeClass panel builder (~400L)
  arc_class.rs        ← ArcClass panel builder (~400L)
  instance.rs         ← Instance panel builder (~400L)
  validation.rs       ← validation status rendering (~200L)
  tests.rs            ← all builder tests (~200L)
```

| # | Task |
|---|------|
| 3C.1 | Create `ui/info/builders/` directory |
| 3C.2 | Extract `node_class.rs` |
| 3C.3 | Extract `arc_class.rs` |
| 3C.4 | Extract `instance.rs` |
| 3C.5 | Extract `validation.rs` |
| 3C.6 | Create `mod.rs` with dispatch |
| 3C.7 | Move tests to `tests.rs` |

**Commit**: `refactor(tui): split info/builders.rs into 6 modules (1847L → max 400L)`

---

## Phase 4: Cleanup & Polish

### 4A: Remove per-file color constants

After Phase 1, each file will still have `const COLOR_*` at the top that now just
re-export from palette. Clean these up:

| # | Task |
|---|------|
| 4A.1 | Remove `BOX_BORDER_UNFOCUSED` from yaml_panel.rs, graph.rs, info/mod.rs (use `palette::NORD_BORDER_UNFOCUSED`) |
| 4A.2 | Remove `BOX_BORDER_SELECTED` from yaml_panel.rs, graph.rs (use `palette::SOLARIZED_CYAN`) |
| 4A.3 | Remove `COLOR_*` from identity_panel.rs (6 consts) |
| 4A.4 | Remove `COLOR_*` from tree.rs (5 consts) |
| 4A.5 | Remove `COLOR_*` from mod.rs (10 consts) |

**Commit**: `refactor(tui): remove redundant per-file color constants`

### 4B: Style helper constants

Add common Style combinations to palette.rs or a new `styles.rs`:

```rust
pub const STYLE_BOLD_CYAN: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
pub const STYLE_DIM: Style = Style::new().fg(DIM);
pub const STYLE_MUTED: Style = Style::new().fg(MUTED);
pub const STYLE_SUCCESS: Style = Style::new().fg(SUCCESS);
pub const STYLE_WARNING: Style = Style::new().fg(WARNING);
pub const STYLE_ERROR: Style = Style::new().fg(ERROR);
```

| # | Task |
|---|------|
| 4B.1 | Add `STYLE_*` constants to `palette.rs` |
| 4B.2 | Replace `Style::default().fg(Color::Rgb(100,100,100))` (10 in graph.rs) with `palette::STYLE_DIM` |
| 4B.3 | Replace common `.fg().add_modifier(BOLD)` chains (73 occurrences) where practical |

**Commit**: `refactor(tui): add const Style helpers, replace inline chains`

---

## Execution Order & Dependencies

```
Phase 1 (palette.rs)           ← Foundation, do FIRST
    │
    ├── Phase 2A (Badge)       ← Independent
    ├── Phase 2B (ProgressBar) ← Independent
    ├── Phase 2C (Scroll)      ← Independent
    └── Phase 2D (FocusBlock)  ← Independent
         │
         ├── Phase 3A (tree split)         ← After Phase 2
         ├── Phase 3B (yaml_panel split)   ← After Phase 2
         └── Phase 3C (builders split)     ← After Phase 2
              │
              ├── Phase 4A (cleanup)       ← After Phase 1+3
              └── Phase 4B (style helpers) ← After Phase 1+3
```

**Phase 2 tasks are independent** of each other and can be done in parallel.
**Phase 3 tasks are independent** of each other and can be done in parallel.

---

## Verification Checklist

After each phase:

- [ ] `cargo check` — zero errors
- [ ] `cargo clippy -- -D warnings` — zero warnings
- [ ] `cargo nextest run --workspace` — all 1039 tests pass
- [ ] No regression in TUI rendering (manual visual check)
- [ ] `grep -rn 'Color::Rgb' src/tui/ | grep -v palette | grep -v generated | grep -v tests | grep -v theme` — decreasing count

## Final Metrics

| Metric | Before | After |
|--------|--------|-------|
| Inline `Color::Rgb()` | 216 | ~20 (tests + theme.rs dynamic) |
| Duplicate colors | 100+ | 0 |
| Badge patterns | 20+ | 0 (use Badge widget) |
| Progress bar patterns | 12+ | 0 (use ProgressBar widget) |
| Block builder chains | 19 | 0 (use bordered_block/focus_block) |
| scroll_indicator impls | 2 | 1 |
| Files > 1,500 lines | 3 | 0 |
| Largest file | 2,205L | ~800L |
| Total commits | - | ~12 granular |
