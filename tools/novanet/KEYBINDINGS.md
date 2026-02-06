# NovaNet TUI Keybindings

Complete keyboard shortcuts reference for the NovaNet Terminal UI.

> **Auto-maintained**: This file is updated via hook when keybindings change in code.
> Source of truth: `src/tui/app.rs` (handle_key_event)

---

## Navigation Modes

| Key | Action |
|-----|--------|
| `1` | Switch to Meta mode |
| `2` | Switch to Data mode |
| `3` | Switch to Overlay mode |
| `4` | Switch to Query mode |
| `Tab` | Cycle to next mode |
| `Shift+Tab` | Cycle to previous mode |

---

## Tree Navigation

| Key | Action |
|-----|--------|
| `j` / `Down` | Move cursor down |
| `k` / `Up` | Move cursor up |
| `h` | Toggle collapse/expand |
| `l` | Toggle collapse/expand |
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

## Scrolling (Encoder-Driven)

Scroll is handled by physical encoders:
- **ENC1**: Tree scroll
- **ENC2**: YAML scroll

Keyboard fallback (when panel focused):
| Key | Action |
|-----|--------|
| `j/k` | Scroll up/down 1 line |
| `d/u` | Scroll up/down 1 page |

---

## Overlays

| Key | Action |
|-----|--------|
| `?` | Toggle help overlay |
| `/` | Open search overlay |
| `Ctrl+P` | Open command palette |
| `Esc` | Close current overlay |

---

## Search Overlay

| Key | Action |
|-----|--------|
| `Enter` | Select result / confirm |
| `Esc` | Close search |
| `Up` / `Ctrl+P` | Previous result |
| `Down` / `Ctrl+N` | Next result |

---

## Command Palette

| Key | Action |
|-----|--------|
| `Enter` | Execute command |
| `Esc` | Close palette |
| `Up` / `Ctrl+P` | Previous command |
| `Down` / `Ctrl+N` | Next command |

---

## CRUD Dialogs

| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `Shift+Tab` | Previous field |
| `Enter` | Submit form |
| `Esc` | Cancel / close |

---

## General

| Key | Action |
|-----|--------|
| `q` | Quit (when no overlay open) |
| `Esc` | Close overlay / quit |
| `r` | Refresh data from Neo4j |

---

## Vim-Style Summary

```
Navigation:  j/k (up/down)  h/l (toggle)  d/u (page)  g/G (top/bottom)
Expand:      e (subtree)    c (collapse)  H/L (global collapse/expand)
Filter:      0 (hide empty in Data mode)
Modes:       1-4 (direct)   Tab (cycle)
Scroll:      ENC1 (tree)    ENC2 (yaml)   j/k/d/u (keyboard fallback)
Overlays:    / (help)       f (search)
Exit:        q or Esc
```
