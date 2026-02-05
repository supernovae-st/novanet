# TUI Atlas Mode Design

**Date:** 2026-02-05
**Status:** Approved
**Version:** v10.6.1

## Overview

Atlas Mode is a new navigation mode `[5]` in the TUI that provides interactive visualizations of NovaNet's architecture. Unlike Meta/Data modes which show tree hierarchies, Atlas Mode shows **conceptual diagrams** that explain HOW the system works.

## Goals

1. **Self-explanatory TUI** — Understand architecture without external docs
2. **Interactive exploration** — Not static diagrams, but navigable views
3. **Context-aware navigation** — Switching modes preserves selection context
4. **Live + Demo modes** — Real Neo4j data by default, demo mode for teaching

## The 6 Atlas Views

### View A: Spreading Activation

**Purpose:** Visualize the cognitive science math behind context assembly.

**Content:**
- Selected Entity as root node (activation = 1.0)
- Connected entities via SEMANTIC_LINK with activation scores
- Visual decay over hops (bar chart showing activation level)
- Task-specific boosts (CTA: urgency×1.3, FAQ: definition×1.3)
- Threshold line showing cutoff (default 0.3)

**Interactions:**
- `[Space]` Step forward one propagation cycle
- `[r]` Reset to initial state
- `[t]` Change task type (CTA/FAQ/HERO/PRICING)
- `[l]` Change locale
- `[Enter]` Select entity as new root

**Data source:** Neo4j query for Entity + SEMANTIC_LINK relationships

### View B: Knowledge Atoms

**Purpose:** Explain WHY the atoms architecture exists (the "aha moment").

**Content:**
- Side-by-side comparison:
  - LEFT: Monolithic JSON blob (450KB, 112K tokens)
  - RIGHT: Selective atom loading (2.4K tokens for 50 terms)
- Visual hierarchy: Locale → TermSet → Term with USES_TERM from Block
- Stats: token cost, load time comparison

**Interactions:**
- `[Tab]` Toggle focus between blob/atoms sides
- `[Enter]` Expand a container to show its atoms
- `[b]` Show Block that uses these atoms

**Data source:** Static demo data (concept explanation, not live data)

### View C: Generation Pipeline

**Purpose:** Show how a Block is generated (NOT translated).

**Content:**
- Split view: INVARIANT SIDE | LOCALIZED SIDE
- Flow: Block → BlockType → BlockRules → BlockPrompt
- Context Assembly box with assembled inputs
- LLM processing indicator
- Output: BlockL10n

**Interactions:**
- `[←][→]` Step through pipeline stages
- `[Enter]` Expand current node details
- `[l]` Change locale to see different L10n outputs
- `[d]` Toggle demo/live mode

**Data source:** Live Neo4j (default) or static demo

### View D: View Traversal Debugger

**Purpose:** Debug the 12 view definitions and understand include rules.

**Content:**
- Tree showing traversal from root node
- Each arc with depth, temperature threshold
- Checkmarks for included nodes, X for filtered
- Stats: nodes reached, filtered, estimated tokens

**Interactions:**
- `[Tab]` Switch between the 12 views
- `[Enter]` Expand a traversal branch
- `[t]` Adjust temperature threshold
- `[d]` Adjust max depth

**Data source:** View YAML definitions + live Neo4j traversal

### View E: Page Composition

**Purpose:** Show the complete anatomy of a Page with all connections.

**Content:**
- Project → Page hierarchy
- Page with PagePrompt, PageType, PageL10n
- Ordered Blocks with their L10n, Prompts, Rules
- Entities connected via USES_ENTITY
- SEO Keywords connected via EXPRESSES
- Locale connections throughout

**Visual encoding:**
- `━━` solid line = invariant nodes
- `┄┄` dashed line = localized nodes
- `┈┈` dotted line = knowledge nodes
- `══` double line = derived nodes

**Interactions:**
- `[←][→]` Navigate between Pages in project
- `[↑][↓]` Scroll through composition
- `[Enter]` Zoom into selected Block
- `[e]` Focus on Entities section
- `[k]` Focus on Keywords section
- `[l]` Change locale
- `[d]` Toggle demo/live mode

**Data source:** Live Neo4j (default) or static demo

### View F: Realm Map

**Purpose:** Bird's-eye view of the 2-realm architecture.

**Content:**
- Two large boxes: GLOBAL | TENANT
- Layers nested within each realm with node counts
- Arrow showing GLOBAL → TENANT read-through
- Layer colors matching theme.rs

**Interactions:**
- `[Tab]` Switch between realms
- `[Enter]` Zoom into selected layer (shows Kinds)
- `[?]` Show layer description (llm_context from taxonomy)

**Data source:** Taxonomy YAML + Neo4j node counts

## Navigation Context

### Mode Switching Behavior

| From | To | Behavior |
|------|----|----------|
| Atlas (any view) | Meta | Focus on Kind of selected element |
| Atlas (any view) | Data | Focus on Instance of selected element |
| Meta/Data | Atlas | Open relevant view for current Kind |

**Kind → View mapping:**
- Page, Block, BlockType → Page Composition
- Entity, EntityL10n → Generation Pipeline
- SEOKeyword → Spreading Activation
- Locale, Term, Expression → Knowledge Atoms
- Any Kind → Realm Map (fallback)

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `5` | Enter Atlas Mode |
| `a-f` | Switch to view A-F |
| `d` | Toggle demo/live mode |
| `l` | Change locale (where applicable) |
| `?` | Show view-specific help |
| `1-4` | Exit to other modes (context-aware) |

## Visual Design

### Header in Atlas Mode

```
NovaNet     [1]Meta [2]Data [3]Overlay [4]Query [5]Atlas•

Atlas: [a]Activation [b]Atoms [c]Pipeline [d]Traversal [e]Page• [f]Realm
```

### Demo Mode Indicator

```
┌─────────────────────────────────────────────────────┐
│  [LIVE] pricing (3/12 pages)   [d] switch to demo   │
│  [DEMO] pricing (example)      [d] switch to live   │
└─────────────────────────────────────────────────────┘
```

### Trait Legend (shown in Page Composition)

```
━━ invariant   ┄┄ localized   ┈┈ knowledge   ══ derived
```

## Implementation Architecture

### New Files

```
src/tui/
  atlas/
    mod.rs           — Atlas mode state machine
    views/
      mod.rs         — View enum and shared traits
      activation.rs  — Spreading Activation view
      atoms.rs       — Knowledge Atoms view
      pipeline.rs    — Generation Pipeline view
      traversal.rs   — View Traversal Debugger
      page.rs        — Page Composition view
      realm.rs       — Realm Map view
    demo.rs          — Static demo data
    render.rs        — Atlas-specific rendering helpers
```

### State Structure

```rust
pub struct AtlasState {
    pub current_view: AtlasView,
    pub demo_mode: bool,
    pub selected_locale: String,

    // View-specific state
    pub activation: ActivationState,
    pub page: PageCompositionState,
    pub pipeline: PipelineState,
    pub traversal: TraversalState,
    pub realm: RealmState,
}

pub enum AtlasView {
    Activation,
    Atoms,
    Pipeline,
    Traversal,
    PageComposition,
    RealmMap,
}
```

### Neo4j Queries

Each view needs specific Cypher queries:

1. **Spreading Activation:** Entity + SEMANTIC_LINK with temperature
2. **Page Composition:** Page → Blocks → Entities → Keywords full tree
3. **Generation Pipeline:** Block with all connected nodes
4. **View Traversal:** Dynamic query from view YAML rules
5. **Realm Map:** Count nodes per realm/layer

## Testing Strategy

1. **Unit tests:** Each view's state transitions
2. **Snapshot tests:** Rendered output for demo mode
3. **Integration tests:** Neo4j queries return expected structure
4. **Manual verification:** Visual inspection of each view

## Implementation Order

1. **Phase 1:** Atlas mode infrastructure (state, routing, header)
2. **Phase 2:** Realm Map (simplest, good foundation)
3. **Phase 3:** Page Composition (most requested)
4. **Phase 4:** Generation Pipeline
5. **Phase 5:** Knowledge Atoms (static, no Neo4j)
6. **Phase 6:** Spreading Activation
7. **Phase 7:** View Traversal Debugger
8. **Phase 8:** Polish (transitions, help overlays)
