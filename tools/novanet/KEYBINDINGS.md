# NovaNet TUI Keybindings

Complete keyboard shortcuts reference for the NovaNet Terminal UI.

> **v0.20.0**: Two navigation modes with 5-panel layout.
> Key `1` = Graph (unified tree), Key `2` = Flow (architecture diagrams).
> Source of truth: `src/tui/app/input.rs` (handle_key)

---

## Navigation Modes

| Key | Mode | Description |
|-----|------|-------------|
| `1` | Graph | Unified tree (Realm > Layer > Class > Instance + Arcs) |
| `2` | Flow | Architecture diagrams (Schema + Pipeline) |

**Mode indicator**: Header shows `[1]Graph [2]Flow` with active mode highlighted.

---

## 5-Panel Layout

```
+-------------------+-------------------+-------------------+
|                   |                   |                   |
|  [1] TREE         |  [2] IDENTITY     |  [4] PROPERTIES   |
|  Cursor-based     |  Scrollable       |  Scrollable       |
|                   |                   |                   |
|                   +-------------------+-------------------+
|                   |                   |                   |
|                   |  [3] CONTENT      |  [5] ARCS          |
|                   |  Scrollable       |  Scrollable       |
|                   |                   |                   |
+-------------------+-------------------+-------------------+
```

**Panel focus**: `Tab`/`Shift+Tab` cycles linearly. Arrow keys navigate spatially.

---

## Unified Tree Navigation

The unified tree combines schema and instances in a single hierarchy:

```
Realm > Layer > Class > Instance
              > Arc Family > Arc Class
```

| Key | Action |
|-----|--------|
| `j` / `Down` | Move down in tree |
| `k` / `Up` | Move up in tree |
| `h` | Collapse node (Tree) / Prev panel (other) |
| `l` | Expand node (Tree) / Next panel (other) |
| `Space` | Toggle expand/collapse |
| `Enter` | Expand node / toggle sections |
| `g` | Go to top |
| `G` | Go to bottom |
| `d` | Page down (half screen) |
| `u` | Page up (half screen) |
| `p` | Jump to parent node |

### Subtree Operations

| Key | Action |
|-----|--------|
| `e` / `E` | Expand subtree under cursor |
| `c` | Collapse subtree under cursor |
| `H` | Collapse all (global) |
| `L` | Expand all (global) |

---

## Instance Loading

Instances are loaded lazily when expanding a Class node.

| Key | Action |
|-----|--------|
| `Enter` on Class | Expand to show first 10 instances |
| `Enter` on "Load more..." | Load next 50 instances |
| `r` | Refresh data |

**Visual indicators**:
- `(42)` after Class name = instance count
- `[+50 more]` = additional instances available
- Spinner shown while loading

---

## Panel Focus & Navigation

| Key | Action |
|-----|--------|
| `Tab` | Next panel (linear: Tree > Identity > Content > Props > Arcs) |
| `Shift+Tab` | Previous panel |
| `Left` / `Right` | Spatial panel navigation |
| `Ctrl+Up` / `Ctrl+Down` | Vertical panel switching |

---

## Panel Scrolling

When any scrollable panel is focused (Identity, Content, Props, Arcs):

| Key | Action |
|-----|--------|
| `j` / `Down` | Scroll down 1 line |
| `k` / `Up` | Scroll up 1 line |
| `d` | Scroll down half page |
| `u` | Scroll up half page |
| `g` | Scroll to top |
| `G` | Scroll to bottom |

**Mouse**: Scroll wheel works on any panel, regardless of focus.

---

## Search Overlay

Press `/` from anywhere to open search.

| Key | Action |
|-----|--------|
| `Enter` | Jump to selected result |
| `Esc` | Close search |
| `j` / `Down` | Next result |
| `k` / `Up` | Previous result |
| `Ctrl+N` | Next search result (global) |
| `Ctrl+P` | Previous search result (global) |
| `Backspace` | Delete character |

---

## Actions

| Key | Action |
|-----|--------|
| `r` | Refresh data from Neo4j |
| `y` | Yank (copy current item's key to clipboard) |
| `Y` | Yank JSON (copy properties as JSON) |
| `J` | Toggle JSON pretty-print / compact mode |
| `c` | Copy focused property value (Props panel) |
| `O` | Open YAML in external editor ($EDITOR) |
| `Ctrl+o` | Navigate back in history |
| `Ctrl+i` | Navigate forward in history |
| `` ` `` | Open recent items popup (history) |

---

## Schema Overlay

| Key | Action |
|-----|--------|
| `s` | Toggle schema overlay (shows YAML definition alongside instance) |
| `+` / `=` | Next property in schema match |
| `-` / `_` | Previous property in schema match |
| `Enter` | Toggle expanded property view (Props panel) |

---

## Display Options

| Key | Action |
|-----|--------|
| `0` | Toggle hide empty classes |

---

## Help & Legend

| Key | Action |
|-----|--------|
| `?` | Open help overlay |
| `F1` | Open color legend overlay |
| `Esc` | Close current overlay |

---

## General

| Key | Action |
|-----|--------|
| `q` | Quit (when no overlay open) |
| `Ctrl+C` | Force quit |
| `Esc` | Close overlay / exit filtered mode |

---

## Vim-Style Summary

```
GRAPH MODE (1):
Navigation:  j/k (up/down)  h/l (collapse/expand)  d/u (page)  g/G (top/bottom)  p (parent)
Subtree:     e (expand)  c (collapse)  H/L (global collapse/expand)
Instances:   Enter (load)  r (refresh)
Focus:       Tab/Shift+Tab (cycle)  Left/Right (spatial)  Ctrl+Up/Down (vertical)
Search:      / (search)  Ctrl+n/p (next/prev result)
Actions:     y/Y (yank key/JSON)  J (JSON toggle)  O (open editor)  Ctrl+o/i (back/forward)
Schema:      s (overlay)  +/- (navigate properties)
Modes:       1 (Graph)  2 (Flow)
Display:     0 (hide empty)
Help:        ? (help)  F1 (legend)
Exit:        q or Ctrl+C

FLOW MODE (2):
Displays architecture diagrams (Schema + Pipeline views).
Press 1 to return to Graph mode.
```

---

## Content Panel Modes

The Content panel (center) shows context-aware content:

| Selection | Panel Title | Content |
|-----------|-------------|---------|
| Class/ArcClass | **SCHEMA** | YAML definition file |
| Instance | **INFO** | Message pointing to PROPERTIES panel |
| Realm/Layer | **INFO** | Section overview with navigation hints |
