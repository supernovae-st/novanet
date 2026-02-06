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
| `N` | Cycle through all modes |
| `Tab` | Cycle focus: Tree → Info → Graph → YAML |
| `Shift+Tab` | Cycle focus backwards |

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
| `e` | Expand subtree under cursor |
| `c` | Collapse subtree under cursor |
| `H` | Collapse all (global) |
| `L` | Expand all (global) |
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
| `Ctrl+o` | Navigate back in history |
| `Ctrl+i` | Navigate forward in history |

---

## Context-Aware Actions

The status bar shows context-aware hints:

| Context | Hint | Action |
|---------|------|--------|
| On Kind (Meta mode) | `2:→Data` | Press 2 to drill into instances |
| On Instance (Data mode) | `1:→Kind` | Press 1 to jump to Kind |

---

## General

| Key | Action |
|-----|--------|
| `q` | Quit (when no overlay open) |
| `Esc` | Close overlay / quit |

---

## Vim-Style Summary

```
Navigation:  j/k (up/down)  h/l (toggle)  d/u (page)  g/G (top/bottom)
Expand:      e (subtree)    c (collapse)  H/L (global collapse/expand)
Filter:      0 (hide empty in Data mode)
Modes:       1-5 (direct)   N (cycle)
Focus:       Tab (cycle panels)
Search:      / or f (search)  ? (help)  F1 (legend)
Actions:     r (refresh)  y (yank key)  Ctrl+o/i (back/forward)
Exit:        q or Esc
```
