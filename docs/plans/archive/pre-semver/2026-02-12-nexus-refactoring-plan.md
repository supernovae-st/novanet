# Nexus Full Refactoring Plan (v11.8)

**Date**: 2026-02-12
**Status**: DRAFT — Awaiting approval
**Scope**: Full restructuration with Tutorial + Persistence

---

## Executive Summary

Transform Nexus from a 6-tab flat structure into a **3-section learning journey** with guided progression, persistent state, and clearer conceptual hierarchy.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  BEFORE (v11.7)                        AFTER (v11.8)                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  [1]Traits [2]Layers [3]Arcs           LEARN    EXPLORE    PRACTICE         │
│  [4]Pipeline [5]Quiz [6]Views          ───────  ─────────  ──────────       │
│                                        Intro    Traits     Quiz             │
│  Flat, no hierarchy                    Tutorial Layers     Playground       │
│  Jumps into details                    Glossary Arcs                        │
│  No guided path                                 Pipeline                    │
│  No persistence                                 Views                       │
│                                                                             │
│  6 tabs, same level                    3 sections, progressive depth        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## New Architecture

### Section-Based Navigation

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NovaNet    [1]Graph  [2]Nexus●                                             │
│  Nexus Mode                                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  LEARN              │  EXPLORE           │  PRACTICE                        │
│  ════════════════   │  ════════════════  │  ════════════════                │
│  [I] Intro      ●   │  [T] Traits        │  [Q] Quiz                        │
│  [G] Glossary       │  [L] Layers        │  [P] Playground                  │
│  [U] Tutorial   ◐   │  [A] Arcs          │                                  │
│                     │  [F] Flow          │                                  │
│                     │  [V] Views         │                                  │
│                                                                             │
│  ● = current  ◐ = in progress (50%)  ○ = not started                        │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  [Tab: next section]  [1-9: jump to item]  [?: help]                        │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Section Breakdown

| Section | Purpose | Items | New? |
|---------|---------|-------|------|
| **LEARN** | Fundamentals for newcomers | Intro, Glossary, Tutorial | ✅ NEW |
| **EXPLORE** | Deep-dive into schema | Traits, Layers, Arcs, Flow*, Views | Existing (renamed) |
| **PRACTICE** | Test & experiment | Quiz, Playground* | Quiz exists, Playground NEW |

*Flow = renamed Pipeline, *Playground = interactive sandbox (future)

---

## New Tabs Detail

### [I] Intro — "The Big Picture"

**Purpose**: First stop for newcomers. Explains WHY NovaNet exists and WHAT it does.

```
┌─ INTRO ─────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  ╔═══════════════════════════════════════════════════════════════════════╗  │
│  ║  NOVANET: Native Content Generation at Scale                          ║  │
│  ╚═══════════════════════════════════════════════════════════════════════╝  │
│                                                                             │
│  THE PROBLEM                          THE SOLUTION                          │
│  ───────────                          ────────────                          │
│  Traditional translation:             NovaNet generation:                   │
│  • Loses cultural nuance              • Native quality per locale           │
│  • 200 locales = 200× cost            • Write once, generate 200×           │
│  • Sync nightmares                    • Graph-based consistency             │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                                                                     │   │
│  │   WRONG:  Source ──translate──▶ Target                              │   │
│  │                                                                     │   │
│  │   RIGHT:  Entity (invariant)                                        │   │
│  │              │                                                      │   │
│  │              ├── + Knowledge (fr-FR: Terms, Culture, Style)         │   │
│  │              │                                                      │   │
│  │              └──▶ EntityContent@fr-FR (generated natively)          │   │
│  │                                                                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  TWO TYPES OF NODES                                                         │
│  ──────────────────                                                         │
│                                                                             │
│  ┌─────────────┐     OF_KIND      ┌─────────────┐                          │
│  │ META (60)   │◄─────────────────│ DATA (200K+)│                          │
│  │ ───────────  │                  │ ───────────  │                          │
│  │ Kind:Locale │  "fr-FR is a     │ Locale:fr-FR│                          │
│  │ (definition)│   Locale"        │ (instance)  │                          │
│  └─────────────┘                  └─────────────┘                          │
│                                                                             │
│  CLASSIFICATION (3 axes)                                                    │
│  ───────────────────────                                                    │
│  WHERE?  Realm  → shared (universal) / org (business-specific)              │
│  WHAT?   Layer  → 10 functional categories (config→output)                  │
│  HOW?    Trait  → 5 locale behaviors (invariant→aggregated)                 │
│                                                                             │
│  [Enter: Start Tutorial]  [G: Glossary]  [→: Explore Traits]                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Content**:
1. Problem/Solution framing
2. Generation vs Translation diagram
3. Meta vs Data explanation
4. 3-axis classification overview
5. Call-to-action: Tutorial or Explore

---

### [G] Glossary — "Concept Dictionary"

**Purpose**: Searchable reference for all NovaNet concepts.

```
┌─ GLOSSARY ──────────────────────────────────────────────────────────────────┐
│                                                                             │
│  [/] Search: _                                          15 concepts         │
│                                                                             │
│  ┌─ CATEGORIES ──────────┬─ DEFINITION ─────────────────────────────────┐  │
│  │                       │                                               │  │
│  │ ▼ Graph Basics (4)    │ ■ META NODE                                   │  │
│  │   ● Meta Node     ←   │ ═══════════                                   │  │
│  │   ○ Data Node         │                                               │  │
│  │   ○ Kind              │ A node that defines the SCHEMA (type system). │  │
│  │   ○ Instance          │ Meta nodes have the :Meta label in Neo4j.     │  │
│  │                       │                                               │  │
│  │ ▷ Classification (3)  │ COUNT: 60 Meta nodes (Kinds) + taxonomy       │  │
│  │   ○ Realm             │                                               │  │
│  │   ○ Layer             │ EXAMPLES:                                     │  │
│  │   ○ Trait             │ • :Meta:Kind {label: 'Locale'}                │  │
│  │                       │ • :Meta:Realm {key: 'shared'}                 │  │
│  │ ▷ Relationships (3)   │ • :Meta:Layer {key: 'semantic'}               │  │
│  │   ○ Arc               │                                               │  │
│  │   ○ Family            │ CONTRAST WITH:                                │  │
│  │   ○ Scope             │ Data nodes are INSTANCES (no :Meta label)     │  │
│  │                       │ e.g., :Locale {key: 'fr-FR'}                  │  │
│  │ ▷ Philosophy (2)      │                                               │  │
│  │   ○ Generation        │ ┌─────────────────────────────────────────┐   │  │
│  │   ○ Native Content    │ │  :Meta:Kind ◄── OF_KIND ── :Locale      │   │  │
│  │                       │ │  (schema)                   (data)      │   │  │
│  │ ▷ Architecture (3)    │ └─────────────────────────────────────────┘   │  │
│  │   ○ Query-First       │                                               │  │
│  │   ○ YAML-First        │ SEE ALSO: Data Node, Kind, Instance           │  │
│  │   ○ Unified Tree      │                                               │  │
│  │                       │ [y: copy]  [Enter: drill]  [Esc: back]        │  │
│  └───────────────────────┴───────────────────────────────────────────────┘  │
│                                                                             │
│  [j/k: navigate]  [/: search]  [Enter: expand]  [Tab: next section]         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Concepts (15)**:
| Category | Concepts |
|----------|----------|
| Graph Basics | Meta Node, Data Node, Kind, Instance |
| Classification | Realm, Layer, Trait |
| Relationships | Arc, Family, Scope |
| Philosophy | Generation, Native Content |
| Architecture | Query-First, YAML-First, Unified Tree |

**Features**:
- Fuzzy search with `/`
- Expand/collapse categories
- SEE ALSO cross-references
- Code examples for each concept
- Copy definition with `y`

---

### [U] Tutorial — "Guided Learning Journey"

**Purpose**: Step-by-step onboarding with hands-on practice.

```
┌─ TUTORIAL ──────────────────────────────────────────────────────────────────┐
│                                                                             │
│  YOUR PROGRESS                                                              │
│  ◉━━━━━━●━━━━━━○━━━━━━○━━━━━━○  Step 2/5: Classification                    │
│  Graph   Class  Arcs   Gen    Tree                                          │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                                                                       │  │
│  │  🎯 OBJECTIVE                                                         │  │
│  │  Understand how nodes are classified with Realm, Layer, and Trait.    │  │
│  │                                                                       │  │
│  │  📖 EXPLANATION                                                       │  │
│  │  Every node in NovaNet has 3 classification axes:                     │  │
│  │                                                                       │  │
│  │  ┌─────────────────────────────────────────────────────────────────┐  │  │
│  │  │  WHERE?   Realm   shared ◉────────────◎ org                     │  │  │
│  │  │                   (universal)         (business)                │  │  │
│  │  │                                                                 │  │  │
│  │  │  WHAT?    Layer   config → locale → geography → knowledge       │  │  │
│  │  │                   foundation → structure → semantic → ...       │  │  │
│  │  │                                                                 │  │  │
│  │  │  HOW?     Trait   ■ inv  □ loc  ◇ kno  ★ gen  ▪ agg            │  │  │
│  │  │                   (same)  (per   (ref)  (LLM)  (metrics)        │  │  │
│  │  │                          locale)                                │  │  │
│  │  └─────────────────────────────────────────────────────────────────┘  │  │
│  │                                                                       │  │
│  │  ✋ PRACTICE                                                          │  │
│  │  Press [1] to switch to Graph mode. Find these nodes:                 │  │
│  │                                                                       │  │
│  │  ☐ 1. A node in shared/config layer (hint: expand shared → config)   │  │
│  │  ☐ 2. A node with trait 'localized' (hint: look for □ symbol)        │  │
│  │  ☐ 3. A node in org/semantic layer                                   │  │
│  │                                                                       │  │
│  │  When done, press [2] to return here and mark complete.               │  │
│  │                                                                       │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  [Enter: Mark step complete]  [n: Next]  [p: Previous]  [r: Reset]          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**5 Tutorial Steps**:

| Step | Title | Objective | Practice Task |
|------|-------|-----------|---------------|
| 1 | **Graph Fundamentals** | Meta vs Data distinction | Find a Kind and its instances |
| 2 | **Classification** | Realm, Layer, Trait | Find nodes by classification |
| 3 | **Arcs & Relationships** | Family, Scope, Cardinality | Trace an arc path |
| 4 | **Generation Flow** | NOT translation | Watch Pipeline animation |
| 5 | **Unified Tree** | v11.7 navigation | Navigate the full tree |

**Features**:
- Progress bar with step indicators
- Objective → Explanation → Practice pattern
- Hands-on tasks that require Graph mode exploration
- Checkbox-style task completion
- Persistent progress (survives TUI restart)

---

## Persistence System

### File Location

```
~/.novanet/
├── tutorial_progress.json    # Tutorial state
├── preferences.json          # UI preferences (future)
└── history.json              # Command history (future)
```

### Tutorial Progress Schema

```json
{
  "version": "1.0",
  "started_at": "2026-02-12T15:30:00Z",
  "updated_at": "2026-02-12T16:45:00Z",
  "steps": [
    {
      "id": 1,
      "title": "Graph Fundamentals",
      "completed": true,
      "completed_at": "2026-02-12T15:45:00Z",
      "tasks": [true, true, true]
    },
    {
      "id": 2,
      "title": "Classification",
      "completed": false,
      "completed_at": null,
      "tasks": [true, false, false]
    }
  ],
  "quiz_high_score": 13,
  "total_time_minutes": 45
}
```

### Rust Implementation

```rust
// src/tui/persistence.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialProgress {
    pub version: String,
    pub started_at: Option<String>,
    pub updated_at: Option<String>,
    pub steps: Vec<StepProgress>,
    pub quiz_high_score: Option<usize>,
    pub total_time_minutes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepProgress {
    pub id: usize,
    pub title: String,
    pub completed: bool,
    pub completed_at: Option<String>,
    pub tasks: Vec<bool>,
}

impl TutorialProgress {
    pub fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)
    }

    fn path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".novanet")
            .join("tutorial_progress.json")
    }
}
```

---

## New File Structure

```
tools/novanet/src/tui/nexus/
├── mod.rs           # Main state + section navigation (refactored)
├── sections.rs      # NEW: NexusSection enum + section logic
├── intro.rs         # NEW: Big Picture introduction
├── glossary.rs      # NEW: Concept dictionary
├── tutorial.rs      # NEW: Guided learning journey
├── traits.rs        # Existing (unchanged)
├── layers.rs        # Existing (unchanged)
├── arcs.rs          # Existing (unchanged)
├── pipeline.rs      # Existing → renamed to flow.rs
├── quiz.rs          # Existing (enhanced with high score)
├── views.rs         # Existing (unchanged)
└── persistence.rs   # NEW: Progress saving

tools/novanet/src/tui/
├── ...
└── persistence.rs   # NEW: Shared persistence utilities
```

---

## State Changes

### New NexusState

```rust
pub struct NexusState {
    // === Section navigation ===
    pub section: NexusSection,      // NEW: LEARN, EXPLORE, PRACTICE
    pub section_cursor: usize,      // NEW: Cursor within section

    // === Tab (item within section) ===
    pub tab: NexusTab,              // Current active tab

    // === Existing per-tab state ===
    pub trait_cursor: usize,
    pub layer_cursor: usize,
    pub layer_realm: usize,
    pub arc_cursor: usize,
    pub pipeline_stage: usize,
    pub pipeline_animating: bool,
    pub quiz: quiz::QuizState,
    pub views: views::ViewsState,

    // === NEW: Learn section state ===
    pub intro_section: usize,       // Current section in Intro
    pub glossary_cursor: usize,     // Glossary category cursor
    pub glossary_concept: usize,    // Concept within category
    pub glossary_search: String,    // Search query
    pub tutorial: TutorialState,    // Tutorial progress

    // === Existing ===
    pub drill_depth: usize,
    pub drill_cursor: usize,
    pub pending_g: bool,
    pub tip_index: usize,
    pub clipboard_message: Option<String>,
    pub clipboard_message_time: Option<Instant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NexusSection {
    #[default]
    Learn,
    Explore,
    Practice,
}

pub struct TutorialState {
    pub current_step: usize,
    pub steps: Vec<StepState>,
    pub persistence: TutorialProgress,
}

pub struct StepState {
    pub tasks_completed: Vec<bool>,
    pub started_at: Option<Instant>,
}
```

---

## Keybinding Changes

### New Global Keys

| Key | Action |
|-----|--------|
| `Tab` | Next section (LEARN→EXPLORE→PRACTICE) |
| `Shift+Tab` | Previous section |
| `I` | Jump to Intro |
| `G` | Jump to Glossary |
| `U` | Jump to Tutorial |
| `T` | Jump to Traits |
| `L` | Jump to Layers |
| `A` | Jump to Arcs |
| `F` | Jump to Flow (Pipeline) |
| `V` | Jump to Views |
| `Q` | Jump to Quiz |
| `1-9` | Jump to item by position |

### Section-Specific

| Section | Keys | Action |
|---------|------|--------|
| Glossary | `/` | Start search |
| Glossary | `Esc` | Clear search |
| Tutorial | `Enter` | Mark task/step complete |
| Tutorial | `r` | Reset progress |
| Tutorial | `1` | Go to Graph mode (for practice) |

---

## Migration Strategy

### Phase 1: Add New Tabs (Non-Breaking)
1. Create `intro.rs`, `glossary.rs`, `tutorial.rs`, `persistence.rs`
2. Add to `NexusTab` enum (keeps existing values)
3. Add `NexusSection` concept (optional at first)
4. Update tab bar to show all 9 items

### Phase 2: Section Navigation
1. Implement `NexusSection` grouping
2. Update tab bar to show sections
3. Update keybindings for section navigation
4. Rename `pipeline.rs` → `flow.rs`

### Phase 3: Persistence
1. Implement `TutorialProgress` save/load
2. Load on TUI startup
3. Save on step completion
4. Add quiz high score tracking

### Phase 4: Polish
1. Add progress indicators (●/◐/○)
2. Add cross-references in Glossary
3. Add Tutorial task detection (auto-complete)
4. Update tests (150+ existing + new)

---

## Estimated Effort

| Component | New Lines | Effort |
|-----------|-----------|--------|
| intro.rs | ~300 | 1h |
| glossary.rs | ~500 | 2h |
| tutorial.rs | ~600 | 3h |
| persistence.rs | ~150 | 1h |
| sections.rs | ~200 | 1h |
| mod.rs updates | ~300 | 2h |
| Tests | ~400 | 2h |
| **Total** | **~2,450** | **12h** |

---

## Open Questions

1. **Playground tab** — Include in v11.8 or defer to v12?
2. **Tutorial task auto-detection** — Track Graph mode actions or manual checkbox?
3. **Glossary content** — Static in code or load from YAML?
4. **Section header style** — Tabs or sidebar?

---

## Approval Checklist

- [ ] Architecture approved
- [ ] File structure approved
- [ ] Keybindings approved
- [ ] Persistence location approved (~/.novanet/)
- [ ] Migration strategy approved
- [ ] Ready to implement

---

## References

- `docs/plans/2026-02-12-nexus-current-state.md` — Current state documentation
- `.claude/rules/novanet-decisions.md` — ADR-022 (Unified Tree)
- `KEYBINDINGS.md` — Current keyboard reference
