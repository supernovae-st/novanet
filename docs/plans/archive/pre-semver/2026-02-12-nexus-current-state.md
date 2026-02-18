# Nexus Current State Documentation (v11.7)

**Date**: 2026-02-12
**Purpose**: Capture current Nexus implementation before refactoring
**Total Lines**: 6,106 lines of Rust across 7 files

---

## Executive Summary

Nexus Mode is NovaNet's **gamified learning hub** for understanding the taxonomy. It provides 6 tabs with interactive exploration of Traits, Layers, Arcs, Pipeline, Quiz, and Views.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NEXUS MODE — Educational Hub                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  [1]■ Traits   — 5-trait constellation (invariant→aggregated)               │
│  [2]▦ Layers   — 2-realm split view (Shared 4 | Org 6)                      │
│  [3]≡ Arcs     — 5 arc families grid                                        │
│  [4]⚡ Pipeline — Animated generation flow (6 stages)                        │
│  [5]? Quiz     — 15-question interactive quiz                               │
│  [6]▶ Views    — Schema views explorer (Query-First)                        │
│                                                                             │
│  Stats: 6,106 lines | 150+ tests | 10 tips | 15 quiz questions              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## File Structure

```
tools/novanet/src/tui/nexus/
├── mod.rs       (2,087 lines) — Main state, tabs, rendering, 100+ tests
├── traits.rs    (1,214 lines) — Constellation + code examples + drill-down
├── views.rs     (  887 lines) — Schema views + Query-First concept panel
├── pipeline.rs  (  585 lines) — Animated generation flow (6 stages)
├── quiz.rs      (  541 lines) — 15 questions + score tracking
├── arcs.rs      (  442 lines) — Arc families grid (5 families)
└── layers.rs    (  350 lines) — Realm split view (Shared | Org)
                 ─────────────
                  6,106 total
```

---

## NexusTab Enum

```rust
pub enum NexusTab {
    Traits,      // [1] Default tab — 5-trait constellation
    Layers,      // [2] Shared/Org split view
    Arcs,        // [3] Arc families visualization
    Pipeline,    // [4] Animated generation flow
    Quiz,        // [5] Interactive 15-question quiz
    Views,       // [6] Schema views explorer
}
```

**Methods**:
- `shortcut()` → char ('1'-'6')
- `label()` → display name
- `next()` / `prev()` → cyclic navigation
- `all()` → static slice of all tabs

---

## NexusState Struct

```rust
pub struct NexusState {
    // Tab management
    pub tab: NexusTab,                          // Current active tab

    // === Traits tab state ===
    pub trait_cursor: usize,                    // 0-4 (5 traits)

    // === Layers tab state ===
    pub layer_cursor: usize,                    // Layer cursor
    pub layer_realm: usize,                     // 0=shared, 1=org

    // === Arcs tab state ===
    pub arc_cursor: usize,                      // 0-4 (5 families)

    // === Pipeline tab state ===
    pub pipeline_stage: usize,                  // 0-5 (6 stages)
    pub pipeline_animating: bool,               // Animation toggle

    // === Quiz tab state ===
    pub quiz: quiz::QuizState,                  // Nested quiz state

    // === Views tab state ===
    pub views: views::ViewsState,               // Nested views state

    // === Drill-down state ===
    pub drill_depth: usize,                     // 0=overview, 1=kinds, 2=instances
    pub drill_cursor: usize,                    // Cursor within drill list

    // === Quick jump state ===
    pub pending_g: bool,                        // 'g' key prefix

    // === Tips state ===
    pub tip_index: usize,                       // Current tip (0-9)

    // === Clipboard state ===
    pub clipboard_message: Option<String>,      // "Copied: X"
    pub clipboard_message_time: Option<Instant>,// Auto-clear timer
}
```

---

## Tab Details

### [1] Traits Tab (traits.rs — 1,214 lines)

**Purpose**: Visualize the 5-trait constellation and their relationships.

**Layout**:
```
┌─ CONSTELLATION ──────────────────┬─ DETAIL ────────────────────────────┐
│                                  │ ■ INVARIANT                         │
│         ◇ KNOWLEDGE (20 K)       │                                     │
│              /\                  │ Nodes that do not change between    │
│             /  \                 │ locales. Structural definitions...  │
│    ■ INVARIANT ─── □ LOCALIZED   │                                     │
│             \  /                 │ BY LAYER ────────────────────────── │
│              \/                  │ config: OrgConfig, Locale, ...      │
│     ★ GENERATED    ▪ AGGREGATED  │ foundation: BrandIdentity, Project  │
│                                  │ ...                                 │
│ ─────────────────────────────────│                                     │
│ [0]■ ■ INVARIANT                 │ PATTERN ─────────────────────────── │
│ [1]□ □ LOCALIZED                 │ ■ Page ────→ □ PageGenerated        │
│ [2]◇ ◇ KNOWLEDGE                 │ ■ Entity ──→ □ EntityContent        │
│ [3]★ ★ GENERATED                 │                                     │
│ [4]▪ ▪ AGGREGATED                │ CODE EXAMPLES ───────────────────── │
│                                  │ YAML: node: name: Entity ...        │
└──────────────────────────────────┴─────────────────────────────────────┘
```

**Data Structures**:
```rust
pub struct TraitStats {
    pub key: String,
    pub display_name: String,
    pub symbol: &'static str,      // ■ □ ◇ ★ ▪
    pub kind_count: usize,
    pub llm_context: String,
    pub kinds_by_layer: Vec<(String, Vec<String>)>,
}

pub struct CodeExample {
    pub title: &'static str,
    pub yaml: &'static str,
    pub neo4j: &'static str,
    pub cypher: &'static str,
}
```

**Features**:
- Constellation ASCII diagram with connections
- Trait list with counts (32 K, 2 K, 20 K, 4 K, 2 K)
- Detail panel: description, BY LAYER breakdown, PATTERN diagram
- CODE EXAMPLES: YAML, Neo4j, Cypher for each trait
- Drill-down: overview → kinds list → instances

---

### [2] Layers Tab (layers.rs — 350 lines)

**Purpose**: Split-view showing Shared (4 layers) and Org (6 layers) realms.

**Layout**:
```
┌─ SHARED (4 layers, 40 nodes) ────┬─ ORG (6 layers, 21 nodes) ──────────┐
│                                  │                                     │
│ ⚙ config (3)                     │ ⚙ config (1)                        │
│   OrgConfig, EntityCategory,     │   OrgConfig                         │
│   Locale, SEOKeywordFormat       │                                     │
│                                  │ ▣ foundation (3)                    │
│ ⊕ locale (6)                     │   Project, ProjectContent,          │
│   Culture, Style, Formatting,    │   BrandIdentity                     │
│   Adaptation, Market, Slugify    │                                     │
│                                  │ ▤ structure (3)                     │
│ ⊙ geography (6)                  │   Page, Block, ContentSlot          │
│   Continent, Region, SubRegion,  │                                     │
│   EconomicRegion, IncomeGroup,   │ ◆ semantic (4)                      │
│   LendingCategory                │   Entity, EntityContent,            │
│                                  │   AudiencePersona, ChannelSurface   │
│ ◈ knowledge (24)                 │                                     │
│   Term, Expression, Pattern,     │ ▧ instruction (7)                   │
│   TermSet, ExpressionSet, ...    │   PageType, BlockType, PagePrompt,  │
│   SEOKeyword, GEOQuery, ...      │   BlockPrompt, BlockInstruction,    │
│                                  │   BlockRules, PromptArtifact        │
│                                  │                                     │
│                                  │ ● output (3)                        │
│                                  │   PageGenerated, BlockGenerated,    │
│                                  │   OutputArtifact                    │
└──────────────────────────────────┴─────────────────────────────────────┘
```

**Navigation**:
- `h/l` or `←/→`: Switch realm (shared ↔ org)
- `j/k` or `↑/↓`: Navigate layers within realm
- Cursor resets when switching realms

---

### [3] Arcs Tab (arcs.rs — 442 lines)

**Purpose**: Visualize the 5 arc families and scope patterns.

**Layout**:
```
┌─ ARC FAMILIES ───────────────────┬─ DETAIL ────────────────────────────┐
│                                  │ → OWNERSHIP                         │
│  → ownership (48)                │                                     │
│  ⇢ localization (22)             │ Hierarchical containment arcs.      │
│  ~ semantic (15)                 │ Parent owns children.               │
│  ⇒ generation (13)               │                                     │
│  ⇝ mining (8)                    │ Examples:                           │
│                                  │   HAS_PAGE, HAS_BLOCK, HAS_ENTITY   │
│  ─────────────────────────────── │   HAS_CONTENT, HAS_GENERATED        │
│                                  │                                     │
│  SCOPE                           │ Scope: intra_realm                  │
│  ─── intra_realm (solid)         │ Cardinality: 1:N                    │
│  --- cross_realm (dashed)        │                                     │
│                                  │                                     │
└──────────────────────────────────┴─────────────────────────────────────┘
```

**Arc Families**:
| Family | Symbol | Count | Purpose |
|--------|--------|-------|---------|
| ownership | → | 48 | Hierarchical containment |
| localization | ⇢ | 22 | Locale-specific links |
| semantic | ~ | 15 | Knowledge connections |
| generation | ⇒ | 13 | LLM flow |
| mining | ⇝ | 8 | Data extraction |

---

### [4] Pipeline Tab (pipeline.rs — 585 lines)

**Purpose**: Animated visualization of the Generation principle (NOT translation).

**Layout**:
```
┌─ GENERATION PIPELINE ───────────────────────────────────────────────────┐
│                                                                         │
│  NovaNet generates content NATIVELY per locale.                         │
│  It does NOT translate from a source language.                          │
│                                                                         │
│  ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐   │
│  │KNOWLEDGE│──▶│INVARIANT│──▶│ CONTEXT │──▶│GENERATE │──▶│LOCALIZED│   │
│  │   ◇     │   │   ■     │   │   ↓     │   │   ⇒     │   │   □     │   │
│  │ Terms   │   │ Entity  │   │  LLM    │   │ Native  │   │ Content │   │
│  │ Culture │   │ Page    │   │ Context │   │  Gen    │   │ Output  │   │
│  └─────────┘   └─────────┘   └─────────┘   └─────────┘   └─────────┘   │
│      ▲                                                                  │
│      │ Currently animating: KNOWLEDGE (stage 0)                         │
│                                                                         │
│  [Space: play/pause]  [j/k: step through]                               │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**6 Stages**:
1. KNOWLEDGE (◇) — Load locale knowledge (Terms, Expressions, Patterns)
2. INVARIANT (■) — Load structural templates (Page, Entity, Block)
3. CONTEXT (↓) — Inject into LLM context
4. GENERATE (⇒) — LLM native generation
5. LOCALIZED (□) — Output localized content
6. COMPLETE (✓) — Generation complete

**Features**:
- Auto-advance animation with color transitions
- Manual navigation (j/k)
- Blinking indicator on current stage

---

### [5] Quiz Tab (quiz.rs — 541 lines)

**Purpose**: Interactive 15-question quiz testing NovaNet taxonomy knowledge.

**Layout**:
```
┌─ QUIZ ──────────────────────────────────────────────────────────────────┐
│                                                                         │
│  Question 3 of 15                                                       │
│                                                                         │
│  How many node traits exist in NovaNet v11.2+?                          │
│                                                                         │
│    A) 3 (invariant, localized, derived)                                 │
│  ▶ B) 5 (invariant, localized, knowledge, generated, aggregated)        │
│    C) 4 (invariant, localized, knowledge, generated)                    │
│    D) 6 (invariant, localized, knowledge, generated, aggregated, job)   │
│                                                                         │
│  [j/k: select]  [Enter: submit]                                         │
│                                                                         │
│  ─────────────────────────────────────────────────────────────────────  │
│  Score: 2/2 (100%)                                                      │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Data Structure**:
```rust
pub struct QuizQuestion {
    pub question: &'static str,
    pub options: [&'static str; 4],
    pub correct: usize,           // 0-3
    pub explanation: &'static str,
}

pub struct QuizState {
    pub current_question: usize,
    pub selected_option: usize,
    pub score: usize,
    pub answered: bool,
    pub complete: bool,
}
```

**15 Questions Cover**:
- Realm count (2)
- Generation vs Translation
- Trait count (5)
- Layer counts (4 shared, 6 org)
- Node count (60)
- Naming conventions (EntityL10n → EntityContent)
- Arc families (5)
- Arc scope (cross_realm)
- Visual encoding (border styles)
- Keybindings (gg = generated)
- Knowledge atoms loading

**Grade Scale**:
- 90-100%: "Expert!" (Green)
- 75-89%: "Great job!" (Cyan)
- 60-74%: "Good effort!" (Yellow)
- <60%: "Keep learning!" (Magenta)

---

### [6] Views Tab (views.rs — 887 lines)

**Purpose**: Schema views explorer teaching Query-First architecture.

**Layout**:
```
┌─ VIEW CATEGORIES ────────────────┬─ VIEW DETAILS ──────────────────────┐
│                                  │ ▶ composition                       │
│ ▶ Overview (3)                   │                                     │
│   ○ complete-graph               │ Page/Block composition hierarchy    │
│   ○ shared-layer                 │                                     │
│   ○ project-layer                │ Root: Page, Block                   │
│                                  │ Depth: 3                            │
│ ▷ Generation (4)                 │ Direction: outgoing                 │
│ ▷ Knowledge (3)                  │                                     │
│ ▷ Project (2)                    │ ─────────────────────────────────── │
│ ▷ Mining (2)                     │ QUERY-FIRST CONCEPT                 │
│ ▷ Contextual (4)                 │                                     │
│                                  │ Cypher query = source of truth      │
│                                  │ Graph displays query results ONLY   │
│                                  │                                     │
│                                  │ [?: toggle concept panel]           │
└──────────────────────────────────┴─────────────────────────────────────┘
```

**View Categories**:
| Category | Count | Purpose |
|----------|-------|---------|
| Overview | 3 | Full graph exploration |
| Generation | 4 | AI agent context |
| Knowledge | 3 | Locale and entity knowledge |
| Project | 2 | Project structure |
| Mining | 2 | SEO/GEO intelligence |
| Contextual | 4 | Node-specific views |

**Query-First Concept Panel** (toggle with `?`):
- ROOT: Starting node type
- INCLUDE: Relations to follow
- DIRECTION: outgoing, incoming, both
- DEPTH: Max hops

---

## Keybindings

### Global (All Tabs)

| Key | Action |
|-----|--------|
| `1-6` | Jump to tab by number |
| `Tab` | Next tab |
| `Shift+Tab` | Previous tab |
| `[` / `]` | Previous / Next tab (vim-style) |
| `j/k` or `↑/↓` | Navigate up/down |
| `h/l` or `←/→` | Navigate left/right (context-specific) |
| `Enter` | Drill down / Submit |
| `Esc` | Drill up / Cancel |
| `y` | Yank (copy to clipboard) |
| `n` | Next tip |
| `?` | Help overlay |

### Quick Jump (Traits Tab)

| Sequence | Action |
|----------|--------|
| `gi` | Jump to invariant |
| `gl` | Jump to localized |
| `gk` | Jump to knowledge |
| `gg` | Jump to generated |
| `ga` | Jump to aggregated |
| `g0` | Reset all cursors |

### Tab-Specific

| Tab | Key | Action |
|-----|-----|--------|
| Pipeline | `Space` | Play/pause animation |
| Quiz | `Enter` | Submit answer / Next |
| Quiz | `r` | Restart quiz |
| Views | `?` | Toggle concept panel |

---

## Tips System

10 rotating educational tips shown in footer:

```rust
pub const TIPS: &[&str] = &[
    "Knowledge is INPUT (savoir) - Localized is OUTPUT (generated)",
    "Layers define WHAT a node does, Traits define HOW it behaves with locale",
    "Content/Generated nodes have invariant parents (Entity→EntityContent, Page→PageGenerated)",
    "Generation, NOT translation: Knowledge + Structure -> Native content",
    "Shared realm is READ-ONLY - all business content lives in Org",
    "Quick jump: gi=invariant, gl=localized, gk=knowledge, gg=generated, ga=aggregated",
    "Knowledge nodes exist ONLY where needed (fr-FR: 20K Terms, sw-KE: 500)",
    "Arc families: ownership, localization, semantic, generation, mining",
    "Invariant = structure (solid border), Localized = output (dashed border)",
    "Press 'n' to see the next tip!",
];
```

**Features**:
- Colorized keywords (knowledge=purple, localized=green, etc.)
- Counter: `[N/10] [n: next] [y: yank]`
- Press `n` to cycle

---

## Clipboard System

- `y` yanks current selection (trait name, layer name, view ID, etc.)
- Shows message: "Copied: Entity" with green styling
- Auto-clears after 2 seconds
- Error messages in red

---

## Drill-Down Pattern

Three-level exploration for Traits tab:

```
Depth 0: Overview (constellation)
    ↓ Enter
Depth 1: Kinds list (32 kinds under invariant)
    ↓ Enter
Depth 2: Instances (data nodes)
    ↑ Esc
Back to depth 1
    ↑ Esc
Back to overview
```

---

## Rendering Pipeline

```rust
pub fn render_nexus(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Length(1), // Breadcrumb
            Constraint::Min(1),    // Content
            Constraint::Length(2), // Tips bar
        ])
        .split(area);

    render_tab_bar(f, chunks[0], app);
    render_breadcrumb(f, chunks[1], app);

    match app.nexus.tab {
        NexusTab::Traits => traits::render_traits_tab(f, app, chunks[2]),
        NexusTab::Layers => layers::render_layers_tab(f, app, chunks[2]),
        NexusTab::Arcs => arcs::render_arcs_tab(f, app, chunks[2]),
        NexusTab::Pipeline => pipeline::render_pipeline_tab(f, app, chunks[2]),
        NexusTab::Quiz => quiz::render_quiz_tab(f, app, chunks[2]),
        NexusTab::Views => views::render_views_tab(f, app, chunks[2]),
    }

    render_tips_bar(f, chunks[3], app);
}
```

---

## Test Coverage

**150+ tests** across all files:

| File | Tests | Coverage |
|------|-------|----------|
| mod.rs | 100+ | Tab switching, navigation, quick jump, drill-down, clipboard |
| quiz.rs | 10 | Question navigation, scoring, completion, reset |
| traits.rs | 15 | Constellation, code examples, drill-down |
| layers.rs | 10 | Realm switching, cursor bounds |
| arcs.rs | 8 | Family navigation |
| pipeline.rs | 10 | Animation, stage navigation |
| views.rs | 12 | Category navigation, concept panel |

---

## What's Missing (Identified Gaps)

Based on user feedback, the current Nexus lacks:

1. **"Big Picture" Introduction**
   - No explanation of Meta vs Data
   - No overview of "why NovaNet exists"
   - Jumps straight into Traits without context

2. **Glossary / Concepts Reference**
   - Terms like "Kind", "Instance", "Realm" not explicitly defined
   - No searchable concept dictionary

3. **Guided Tutorial**
   - No step-by-step onboarding
   - Quiz tests knowledge but doesn't teach it

4. **Visual Hierarchy**
   - Tab bar gets crowded with 6 tabs
   - No logical grouping (educational vs reference)

---

## Proposed Refactoring

See: `docs/plans/2026-02-12-nexus-refactoring-plan.md` (to be created)

**Possible Improvements**:
1. Add `[0] Intro` or `[7] Concepts` tab for fundamentals
2. Add `[8] Tutorial` for guided learning
3. Group tabs: `[Fundamentals] [Schema] [Tools]`
4. Enhanced Help with concept definitions
5. Persistent tutorial progress

---

## References

- `tools/novanet/src/tui/nexus/` — Source files
- `.claude/rules/novanet-decisions.md` — ADR-022 (Unified Tree)
- `.claude/rules/novanet-terminology.md` — Canonical terms
- `KEYBINDINGS.md` — Full keyboard reference
