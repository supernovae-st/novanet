# NovaNet TUI Keybindings

Complete keyboard shortcuts reference for the NovaNet Terminal UI.

> **v0.13.0**: Three navigation modes with lazy instance loading.
> Key `1` = Graph (unified tree), Key `2` = Views (schema views explorer), Key `3` = Nexus (hub).
> Search via `/` overlay. Stats tab redesigned as "Matrix Control Tower".
> Source of truth: `src/tui/app.rs` (handle_key)

---

## Navigation Modes (v0.13.0)

| Key | Mode | Description |
|-----|------|-------------|
| `1` | Graph | Unified tree (Realm > Layer > Class > Instance + Arcs) |
| `2` | Views | Schema views explorer (queries, categories, Cypher) |
| `3` | Nexus | Hub (Quiz, Audit, Stats, Help) |
| `/` | Search | Overlay search (filter nodes/arcs) |

**Mode indicator**: Header shows `[1]Graph [2]Views [3]Nexus` with active mode highlighted.

> **Deprecated (v11.6)**: Keys `4` and beyond are no longer used for modes.
> Audit and Stats are now accessible from Nexus hub via `A` and `S`.

---

## Unified Tree Navigation (v0.12.0)

The unified tree combines schema (meta) and instances (data) in a single hierarchy:

```
Realm > Layer > Class > Instance
              > Arc Family > Arc Class
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

## Instance Loading (v0.12.0)

Instances are loaded lazily when expanding a Class node.

| Key | Action |
|-----|--------|
| `Enter` on Class | Expand to show first 10 instances |
| `Enter` on "Load more..." | Load next 50 instances |
| `r` | Refresh instance count for current Class |
| `R` | Refresh all instance counts |

**Visual indicators**:
- `(42)` after Class name = instance count
- `[+50 more]` = additional instances available
- Spinner shown while loading

---

## Views Mode (v0.13.0)

The Views mode provides a schema views explorer for browsing predefined Cypher queries.

| Key | Action |
|-----|--------|
| `j` / `Down` | Move down in view list |
| `k` / `Up` | Move up in view list |
| `Enter` | Select view / expand details |
| `y` | Yank view Cypher to clipboard |
| `1` | Switch to Graph mode |
| `3` | Switch to Nexus mode |

---

## Nexus Hub (v0.13.0)

The Nexus hub provides gamified learning and system tools.

### Quick Access (from Graph mode)

| Key | Action |
|-----|--------|
| `3` | Enter Nexus hub |
| `Q` | Jump to Quiz mode |
| `A` | Jump to Audit mode |
| `S` | Jump to Stats dashboard |
| `?` | Jump to Help overlay |

### Within Nexus Mode

| Key | Action |
|-----|--------|
| `1` | Switch to Graph mode |
| `2` | Switch to Views mode |
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

### Stats Dashboard (Matrix Control Tower)

The Stats tab displays NovaNet schema statistics with cyberpunk aesthetics:
- **Hero Panel**: Big animated counters (NODE, ARC, LAYR, TRAT, RELM)
- **Heartbeat**: Pulsing sparkline (system status indicator)
- **Bar Charts**: Realm, Layer, and Arc Family distributions

Boot animation plays on first view (~2s), then heartbeat pulses continuously.

| Key | Action |
|-----|--------|
| `r` | Refresh stats from Neo4j |
| `y` | Yank stats summary |

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

Quick filter to show only Classes matching a specific trait (ADR-024 Data Origin).

| Key | Action |
|-----|--------|
| `fd` | Filter: defined |
| `fa` | Filter: authored |
| `fi` | Filter: imported |
| `fg` | Filter: generated |
| `fr` | Filter: retrieved |
| `ff` | Clear filter (show all) |

When a filter is active, the title bar shows the active trait.

---

## Panel Scrolling

When Info or Content panel is focused (v0.17.3: renamed from Detail/YAML):

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
Filter:      fd/fa/fi/fg/fr (trait)  ff (clear)
Search:      / (search)  ? (help)  F1 (legend)
Actions:     r (refresh)  y/Y (yank key/JSON)  J (JSON toggle)  Ctrl+o/i (back/forward)
Modes:       2 (Views)  3 (Nexus)
Exit:        q or Esc

VIEWS MODE (2):
Navigation:  j/k (up/down)  Enter (select/expand)
Actions:     y (yank Cypher)
Modes:       1 (Graph)  3 (Nexus)

NEXUS MODE (3):
Access:      Q (quiz)  A (audit)  S (stats)  ? (help)
Tabs:        [ ] (prev/next)  Tab (cycle)
i18n:        I (toggle En/Fr)
Quiz:        j/k (select)  Enter (submit/next)  r (restart)
Modes:       1 (Graph)  2 (Views)
```

---

## Deprecated Keybindings (v11.6 -> v0.17.3)

| Old Key | Old Action | New Equivalent |
|---------|------------|----------------|
| `4` | Old Nexus mode | `3` (Nexus) |
| `0` | Hide empty (Graph mode) | Removed (unified tree shows all) |
| `s` | Schema overlay | Removed (unified tree shows schema inline) |
| `t` | Toggle YAML/Data in Source panel | Removed (v0.17.3: context-aware content) |

### Content Panel Modes (v0.17.3)

The Content panel (center) now automatically shows context-aware content:

| Selection | Panel Title | Content |
|-----------|-------------|---------|
| Class/ArcClass | **SCHEMA** | YAML definition file |
| Instance | **INFO** | Message pointing to PROPERTIES panel |
| Realm/Layer | **INFO** | Section overview with navigation hints |
