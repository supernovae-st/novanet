# NovaNet TUI Keybindings

Complete keyboard shortcuts reference for the NovaNet Terminal UI.

> **Auto-maintained**: This file is updated via hook when keybindings change in code.
> Source of truth: `src/tui/app.rs` (handle_key)

---

## Navigation Modes

| Key | Action |
|-----|--------|
| `1` | Switch to Meta mode (remembers cursor position) |
| `2` | Switch to Data mode (drill into instances from Kind) |
| `3` | Switch to Overlay mode |
| `4` | Switch to Query mode |
| `5` | Switch to Atlas mode |
| `6` | Switch to Audit mode |
| `N` | Cycle through all modes |
| `Tab` | Cycle focus: Tree → Info → Graph → YAML |
| `Shift+Tab` | Cycle focus backwards |
| `` ` `` | Open recent items popup (navigation history) |

---

## Tree Navigation

| Key | Action |
|-----|--------|
| `j` / `Down` | Move cursor down |
| `k` / `Up` | Move cursor up |
| `h` | Collapse node |
| `l` | Expand node |
| `Space` | Toggle collapse/expand |
| `Enter` | Toggle collapse/expand |
| `e` / `E` | Expand subtree under cursor |
| `c` | Collapse subtree under cursor |
| `H` | Collapse all (global) |
| `L` | Expand all (global) |
| `p` | Jump to parent node |
| `0` | Toggle hide empty (Data mode only) |
| `d` | Page down (half screen) |
| `u` | Page up (half screen) |
| `g` | Jump to first item |
| `G` | Jump to last item |

---

## Panel Scrolling

When Info, YAML, or JSON panel is focused:

| Key | Action |
|-----|--------|
| `j` / `Down` | Scroll down 1 line |
| `k` / `Up` | Scroll up 1 line |
| `d` | Scroll down half page |
| `u` | Scroll up half page |
| `g` | Scroll to top |
| `G` | Scroll to bottom |

Scrollbars are displayed when content exceeds visible area.

---

## Search & Help

| Key | Action |
|-----|--------|
| `/` | Open search overlay (vim-style) |
| `f` | Open search overlay (alias) |
| `?` | Open help overlay (keyboard shortcuts) |
| `F1` | Open color legend overlay (Realm/Layer/Trait colors) |
| `Esc` | Close current overlay |

---

## Search Overlay

| Key | Action |
|-----|--------|
| `Enter` | Jump to selected result |
| `Esc` | Close search |
| `j` / `Down` | Next result |
| `k` / `Up` | Previous result |
| `Ctrl+N` | Next search result (works globally) |
| `Ctrl+P` | Previous search result (works globally) |
| `Backspace` | Delete character |

---

## Actions

| Key | Action |
|-----|--------|
| `r` | Refresh data from Neo4j |
| `y` | Yank (copy current item's key to clipboard) |
| `Y` | Yank JSON (copy current item's properties as JSON) |
| `J` | Toggle JSON pretty-print / compact mode |
| `Ctrl+o` | Navigate back in history |
| `Ctrl+i` | Navigate forward in history |

---

## Schema Overlay (Data Mode)

| Key | Action |
|-----|--------|
| `s` | Toggle schema overlay (show property match) |
| `+` / `=` | Focus next property in schema overlay |
| `-` / `_` | Focus previous property in schema overlay |

---

## Data Mode Hierarchy (v11.0)

The tree structure in Data mode follows this hierarchy:

```
Realm → Layer → Kind → EntityCategory (Entity only) → Instance
```

**Note**: EntityCategory only appears when viewing Entity instances. It groups Entity instances
by semantic type (THING, ACTION, FEATURE, etc.). Other kinds jump directly from Kind to Instance.

Navigation through EntityCategory uses the same keys as all other tree levels:
- `h` collapse EntityCategory, `l` expand it
- `j`/`k` move between categories or instances
- `Space`/`Enter` toggle expand/collapse

## Context-Aware Actions

The status bar shows context-aware hints:

| Context | Hint | Action |
|---------|------|--------|
| On Kind (Meta mode) | `2:→Data` | Press 2 to drill into instances |
| On Instance (Data mode) | `1:→Kind` | Press 1 to jump to Kind |
| On EntityCategory (Data mode) | `l:expand` | Press l to expand and see instances in category |

---

## Guide Mode (Mode 7)

Educational mode for learning NovaNet concepts.

### Tab Switching (within Guide mode)

| Key | Action |
|-----|--------|
| `1` | Switch to Traits tab |
| `2` | Switch to Layers tab |
| `3` | Switch to Arcs tab |
| `4` | Switch to Pipeline tab |
| `Tab` | Cycle to next tab |
| `Shift+Tab` | Cycle to previous tab |

### Quick Jump (g prefix)

| Key | Action |
|-----|--------|
| `gi` | Jump to invariant trait |
| `gl` | Jump to localized trait |
| `gk` | Jump to knowledge trait |
| `gd` | Jump to derived trait |
| `gj` | Jump to job trait |
| `gg` | Reset all cursors to top |

### Navigation

| Key | Action |
|-----|--------|
| `j` / `Down` | Move cursor down |
| `k` / `Up` | Move cursor up |
| `h` | Switch realm (Layers) / drill up (others) |
| `l` | Switch realm (Layers) / drill down (others) |
| `Enter` | Drill down into selection |
| `Esc` | Drill up / cancel pending `g` |
| `Space` | Toggle animation (Pipeline tab only) |

### Actions

| Key | Action |
|-----|--------|
| `y` | Yank (copy current item to clipboard) |
| `n` | Next "Did you know?" tip |

### Tips Bar

The bottom bar shows educational tips with trait-colored keywords.
- Press `n` to cycle through tips
- Press `y` to copy current selection
- Clipboard feedback shown temporarily (2s)

---

## General

| Key | Action |
|-----|--------|
| `q` | Quit (when no overlay open) |
| `Esc` | Close overlay / quit |

---

## Vim-Style Summary

```
Navigation:  j/k (up/down)  h/l (toggle)  d/u (page)  g/G (top/bottom)  p (parent)
Expand:      e (subtree)    c (collapse)  H/L (global collapse/expand)
Filter:      0 (hide empty in Data mode)
Modes:       1-6 (direct)   N (cycle)   ` (recent items)
Focus:       Tab (cycle panels)
Search:      / or f (search)  ? (help)  F1 (legend)
Actions:     r (refresh)  y/Y (yank key/JSON)  J (JSON toggle)  Ctrl+o/i (back/forward)
Schema:      s (overlay)  +/- (focus property)
Exit:        q or Esc

Guide Mode (7):
Tabs:        1-4 (Traits/Layers/Arcs/Pipeline)  Tab (cycle)
Quick Jump:  gi/gl/gk/gd/gj (traits)  gg (top)
Actions:     y (yank)  n (next tip)  Enter/Esc (drill)
```
