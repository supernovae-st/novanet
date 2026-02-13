# NovaNet TUI Keybindings

Complete keyboard shortcuts reference for the NovaNet Terminal UI.

> **v11.7**: Unified Tree Architecture - 2 modes (Graph, Nexus) with lazy instance loading.
> Key `1` = Graph (unified tree), Key `2` = Nexus (hub). Search via `/` overlay.
> Source of truth: `src/tui/app.rs` (handle_key)

---

## Navigation Modes (v11.7)

| Key | Mode | Description |
|-----|------|-------------|
| `1` | Graph | Unified tree (Realm > Layer > Kind > Instance + Arcs) |
| `2` | Nexus | Hub (Quiz, Audit, Stats, Help) |
| `/` | Search | Overlay search (filter nodes/arcs) |

**Mode indicator**: Header shows `[1]Graph [2]Nexus` with active mode highlighted.

> **Deprecated (v11.6)**: Keys `3` (Audit) and `4` (Nexus) are no longer separate modes.
> Audit and Stats are now accessible from Nexus hub via `A` and `S`.

---

## Unified Tree Navigation (v11.7)

The unified tree combines schema (meta) and instances (data) in a single hierarchy:

```
Realm > Layer > Kind > Instance
              > Arc Family > Arc Kind
```

| Key | Action |
|-----|--------|
| `j` / `Down` | Move down in tree |
| `k` / `Up` | Move up in tree |
| `h` / `Left` | Collapse node |
| `l` / `Right` / `Enter` | Expand node / Select |
| `Space` | Toggle expand/collapse |
| `Tab` | Switch focus (Tree <-> Detail panel) |
| `Shift+Tab` | Switch focus backwards |
| `g` | Go to top of tree |
| `G` | Go to bottom of tree |
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

## Instance Loading (v11.7)

Instances are loaded lazily when expanding a Kind node.

| Key | Action |
|-----|--------|
| `Enter` on Kind | Expand to show first 10 instances |
| `Enter` on "Load more..." | Load next 50 instances |
| `r` | Refresh instance count for current Kind |
| `R` | Refresh all instance counts |

**Visual indicators**:
- `(42)` after Kind name = instance count
- `[+50 more]` = additional instances available
- Spinner shown while loading

---

## Nexus Hub (v11.7)

The Nexus hub provides gamified learning and system tools.

### Quick Access (from Graph mode)

| Key | Action |
|-----|--------|
| `2` | Enter Nexus hub |
| `Q` | Jump to Quiz mode |
| `A` | Jump to Audit mode |
| `S` | Jump to Stats dashboard |
| `?` | Jump to Help overlay |

### Within Nexus Mode

| Key | Action |
|-----|--------|
| `1` | Back to Graph mode |
| `[` | Previous tab |
| `]` | Next tab |
| `Tab` | Cycle to next tab |
| `Shift+Tab` | Cycle to previous tab |
| `I` | Toggle language (En/Fr) |

**Tabs**: Quiz | Audit | Stats | Help

### Quiz Mode

| Key | Action |
|-----|--------|
| `j` / `Down` | Select next answer |
| `k` / `Up` | Select previous answer |
| `Enter` | Submit answer / Next question |
| `r` | Restart quiz (when complete) |
| `y` | Yank current question text |

### Audit Mode

| Key | Action |
|-----|--------|
| `j` / `Down` | Navigate audit results |
| `k` / `Up` | Navigate audit results |
| `Enter` | View details |
| `r` | Re-run audit |

### Stats Dashboard

| Key | Action |
|-----|--------|
| `r` | Refresh stats |
| `y` | Yank stats as JSON |

---

## Search Overlay

Press `/` from anywhere to open search.

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

## Trait Filter

Quick filter to show only Kinds matching a specific trait.

| Key | Action |
|-----|--------|
| `fi` | Filter: invariant |
| `fl` | Filter: localized |
| `fk` | Filter: knowledge |
| `fg` | Filter: generated |
| `fa` | Filter: aggregated |
| `ff` | Clear filter (show all) |

When a filter is active, the title bar shows the active trait.

---

## Panel Scrolling

When Detail or YAML panel is focused:

| Key | Action |
|-----|--------|
| `j` / `Down` | Scroll down 1 line |
| `k` / `Up` | Scroll up 1 line |
| `d` | Scroll down half page |
| `u` | Scroll up half page |
| `g` | Scroll to top |
| `G` | Scroll to bottom |

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
| `` ` `` | Open recent items popup (navigation history) |

---

## Help & Legend

| Key | Action |
|-----|--------|
| `?` | Open help overlay (keyboard shortcuts) |
| `F1` | Open color legend overlay (Realm/Layer/Trait colors) |
| `Esc` | Close current overlay |

---

## General

| Key | Action |
|-----|--------|
| `q` | Quit (when no overlay open) |
| `Esc` | Close overlay / quit |

---

## Vim-Style Summary

```
GRAPH MODE (1):
Navigation:  j/k (up/down)  h/l (collapse/expand)  d/u (page)  g/G (top/bottom)  p (parent)
Expand:      e (subtree)    c (collapse subtree)  H/L (global collapse/expand)
Instances:   Enter (load)   r (refresh count)
Focus:       Tab (cycle panels)
Filter:      fi/fl/fk/fg/fa (trait)  ff (clear)
Search:      / (search)  ? (help)  F1 (legend)
Actions:     r (refresh)  y/Y (yank key/JSON)  J (JSON toggle)  Ctrl+o/i (back/forward)
Exit:        q or Esc

NEXUS MODE (2):
Access:      Q (quiz)  A (audit)  S (stats)  ? (help)
Tabs:        [ ] (prev/next)  Tab (cycle)
i18n:        I (toggle En/Fr)
Quiz:        j/k (select)  Enter (submit/next)  r (restart)
Back:        1 (Graph mode)
```

---

## Deprecated Keybindings (v11.6 -> v11.7)

| Old Key | Old Action | New Equivalent |
|---------|------------|----------------|
| `3` | Audit mode | `2` then `A` (Nexus > Audit) |
| `4` | Nexus mode | `2` (Nexus) |
| `0` | Hide empty (Data mode) | Removed (unified tree shows all) |
| `s` | Schema overlay | Removed (unified tree shows schema inline) |
