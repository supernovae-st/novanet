# TUI Box Navigation Enhancement

**Date**: 2026-02-15
**Status**: Approved
**Version**: v0.13.0

## Problem

The TUI Graph mode has multiple info boxes (IDENTITY, LOCATION, METRICS, etc.) but:
1. All boxes have identical border styles — unclear which is "selected"
2. No navigation between boxes — can't Tab between them
3. Copy (`y`) only works for Cypher in Views mode, not for other content

## Solution

Implement "Focusable Box" pattern with:
- 6 consolidated boxes (down from 12)
- Tab/Arrow navigation between boxes
- Smart copy format per box type

## Layout (6 Boxes)

```
┌─ TREE [1] ──┐  ┌─ DETAIL ───────────────────┐  ┌─ CONTEXT ──────────┐
│             │  │                            │  │                    │
│ ▼ Node...   │  │ ┌─ HEADER [2] ────────────┐│  │ ┌─ SOURCE [5] ────┐│
│   ▼ Org     │  │ │ Entity      ◈ Schema    ││  │ │                 ││
│     ▶ Page  │  │ │ key: page   org/struct  ││  │ │ # YAML content  ││
│             │  │ │ realms: 2   classes: 61 ││  │ │ (50% height)    ││
│             │  │ │ Org ████░░ 34%          ││  │ └─────────────────┘│
│             │  │ └─────────────────────────┘│  │ ┌─ DIAGRAM [6] ───┐│
│             │  │ ┌─ PROPERTIES [3] ────────┐│  │ │                 ││
│             │  │ │ * key         string    ││  │ │  REALM HIER.    ││
│             │  │ │   (scrollable)          ││  │ │  ◉ SHARED       ││
│             │  │ └─────────────────────────┘│  │ │  ◎ ORG          ││
│             │  │ ┌─ ARCS [4] ──────────────┐│  │ │ (50% height)    ││
│             │  │ │ → 5 out  ← 3 in         ││  │ └─────────────────┘│
│             │  │ └─────────────────────────┘│  │                    │
└─────────────┘  └────────────────────────────┘  └────────────────────┘
```

## Box Consolidation

| Box | Contents | Purpose |
|-----|----------|---------|
| [1] TREE | Class/Instance navigation | Navigate schema |
| [2] HEADER | Identity + Location + Metrics + Coverage | Overview at a glance |
| [3] PROPERTIES | Property list with types | Schema details (scrollable) |
| [4] ARCS | Incoming/outgoing relationships | Graph connections |
| [5] SOURCE | YAML file content | Raw definition |
| [6] DIAGRAM | Architecture/Hierarchy visual | Visual context |

## Visual States

```
UNFOCUSED (panel not active):
└── Border: dim gray (#3B4252)
└── Title: gray

FOCUSED but not SELECTED (panel active, other box selected):
└── Border: light gray (#4C566A)
└── Title: light gray

SELECTED (active box for copy/scroll):
└── Border: cyan bright + double-line (╔═══╗)
└── Title: cyan + BOLD + indicator "▶"
```

## Keybindings

| Key | Action |
|-----|--------|
| `Tab` | Next box (cycle 1→2→3→4→5→6→1) |
| `Shift+Tab` | Previous box |
| `→` / `l` | Next box (alias) |
| `←` / `h` | Previous box (alias) |
| `y` | Copy selected box content (smart format) |
| `j/k` | Scroll/navigate within selected box |
| `Ctrl+d/u` | Page scroll in PROPERTIES |

## Copy Formats

| Box | Format | Example |
|-----|--------|---------|
| TREE | Path string | `"org/structure/Page"` |
| HEADER | JSON metadata | `{"type":"Class","key":"Page","realm":"org",...}` |
| PROPERTIES | JSON schema | `{"key":{"type":"string","required":true},...}` |
| ARCS | JSON array | `{"outgoing":[...],"incoming":[...]}` |
| SOURCE | YAML raw | `"node:\n  name: Page\n..."` |
| DIAGRAM | ASCII/Mermaid | Depends on content type |

## Implementation

### New Types (src/tui/app.rs)

```rust
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum InfoBox {
    #[default]
    Tree,
    Header,
    Properties,
    Arcs,
    Source,
    Diagram,
}

impl InfoBox {
    pub fn next(self) -> Self {
        match self {
            Self::Tree => Self::Header,
            Self::Header => Self::Properties,
            Self::Properties => Self::Arcs,
            Self::Arcs => Self::Source,
            Self::Source => Self::Diagram,
            Self::Diagram => Self::Tree,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Tree => Self::Diagram,
            Self::Header => Self::Tree,
            Self::Properties => Self::Header,
            Self::Arcs => Self::Properties,
            Self::Source => Self::Arcs,
            Self::Diagram => Self::Source,
        }
    }
}
```

### App State

```rust
pub struct App {
    // ... existing fields ...
    pub selected_box: InfoBox,
}
```

### Files to Modify

1. `src/tui/app.rs` — Add `InfoBox` enum and `selected_box` field
2. `src/tui/handlers/mod.rs` — Tab/Arrow handlers for box navigation
3. `src/tui/ui/info.rs` — Refactor to 6 boxes with selected/unselected styles
4. `src/tui/ui/graph.rs` — Pass `selected_box` to rendering functions
5. `src/tui/clipboard.rs` — Add `copy_box_content(box, app)` with smart formats
6. `src/tui/ui/yaml_panel.rs` — Support selected state styling

## Proportions

- Left column (TREE): 25%
- Middle column (DETAIL): 45%
- Right column (CONTEXT): 30%
  - SOURCE: 50% height
  - DIAGRAM: 50% height
