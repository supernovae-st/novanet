# TUI Navigation Context Design (v11.6)

**Date**: 2026-02-10
**Status**: Implemented
**Author**: Brainstorm session with Claude

## Problem Statement

Navigation in the TUI is confusing:
- Users lose context of where they are in the hierarchy (Realm > Layer > Kind > Instance)
- No visual indicator of position within the full tree
- Scrolling makes it easy to lose track of parent nodes

## Solution

Two complementary features:

### 1. Sticky Breadcrumb (Top of Tree Panel)

Hierarchical path display showing current position with arrows and icons.

```
┌─────────────────────────────────────────────────────────────┐
│ ◉ org                                                       │
│  → ◆ semantic                                               │
│    → ■ Entity (281) →3←2                                    │
│      → 📂 thing                                             │
│        → ► qr-code-generator                                │
├─────────────────────────────────────────────────────────────┤
│ [scrollable tree content below]                             │
└─────────────────────────────────────────────────────────────┘
```

**Icons by level:**
| Level | Icon | Color |
|-------|------|-------|
| Realm | ◉/◎ (org/shared) | cyan/teal |
| Layer | ⚙◆▣▢●▷ (per layer) | layer color |
| Kind | ■□◇ (trait icon) | white + trait color |
| Category | 📂 | gray |
| Instance | ► | yellow/gold |

**Height:** Dynamic 1-5 lines based on depth.

### 2. Mini-Map (Right Side of Tree Panel)

Vertical band (2 chars wide) showing position in full tree.

```
┌─────────────────────────────────────────┬──┐
│ ▼ ◉ Shared                              │░░│
│   ▼ ⚙ config                            │░░│
│ ▼ ◉ Org                                 │██│ ← cursor here
│   ▼ ◆ semantic                          │██│
│     ► Entity (281)                      │░░│
└─────────────────────────────────────────┴──┘
```

**Symbols:**
- `██` = Current cursor position
- `░░` = Visible viewport area
- `▒▒` = Items outside viewport
- (empty) = No items

**Colors:** Realm-based (teal for Shared, magenta for Org)

**Proportionality:** Height scales to represent total items.

## Layout

```
┌─ Tree Panel (28%) ──────────────────────────────┬──┐
│                                                 │  │
│  BREADCRUMB SECTION (1-5 lines, sticky)         │  │
│                                                 │  │
├─────────────────────────────────────────────────┤  │
│                                                 │  │
│  TREE SECTION (scrollable)                      │MM│ ← Mini-Map
│                                                 │  │
│                                                 │  │
└─────────────────────────────────────────────────┴──┘
```

## Implementation

### Files to Modify

| File | Changes |
|------|---------|
| `src/tui/ui/tree.rs` | Add `render_breadcrumb()`, `render_minimap()` |
| `src/tui/ui/mod.rs` | Update layout to split tree area |
| `src/tui/app.rs` | Use existing `current_breadcrumb()` or enhance |

### New Functions

```rust
/// Render sticky breadcrumb at top of tree panel
fn render_breadcrumb(f: &mut Frame, area: Rect, app: &App) -> u16 {
    // Returns height used (1-5 lines)
}

/// Render mini-map on right side of tree panel
fn render_minimap(f: &mut Frame, area: Rect, app: &App) {
    // 2-char wide vertical bar
}
```

### Breadcrumb Path Builder

```rust
struct BreadcrumbLevel {
    icon: &'static str,
    label: String,
    color: Color,
    depth: usize,
}

fn build_breadcrumb_path(app: &App) -> Vec<BreadcrumbLevel> {
    // Extract path from current TreeItem
}
```

## Estimation

- Breadcrumb: ~100 lines
- Mini-map: ~80 lines
- Layout changes: ~20 lines
- **Total: ~200 lines**

## Implementation Details

**Completed**: 2026-02-10

### Files Modified

| File | Changes |
|------|---------|
| `src/tui/ui/tree.rs` | Added `BreadcrumbLevel`, `MiniMapInfo` structs; `build_breadcrumb_path()`, `render_breadcrumb()`, `build_minimap_info()`, `render_minimap()` functions; modified `render_tree()` to split layout |
| `src/tui/ui/mod.rs` | Updated tree panel width from 25% to 28% |

### Line Counts

- Breadcrumb rendering: ~90 lines (struct + 2 functions)
- Mini-map rendering: ~80 lines (struct + 2 functions)
- Layout integration: ~50 lines (in render_tree)
- **Total: ~220 lines**

### Key Implementation Notes

1. **Breadcrumb**: Dynamically builds path from current `TreeItem` using pattern matching. Each level shows icon + label with appropriate color. Height is 1-5 lines based on depth.

2. **Mini-map**: Uses proportional mapping to show cursor and viewport position. Symbols: `██` (cursor), `░░` (viewport), `▒▒` (outside). Color reflects current realm.

3. **Layout**: Tree panel reserves 3 chars on right (2 for mini-map + 1 for separator). Breadcrumb renders at top with separator line, tree content scrolls below.

## Future Enhancements

- Click navigation on breadcrumb segments
- Mini-map click-to-jump
- Keyboard shortcut to focus breadcrumb (navigate up quickly)
