# Unified Tree Architecture Design

**Version**: v11.7.0
**Date**: 2026-02-11
**Status**: Approved

## Problem Statement

NovaNet has inconsistent behavior between Neo4j and UI:

1. **Realm, Layer, Trait ARE nodes in Neo4j** (`:Meta:Realm`, `:Meta:Layer`, etc.)
2. **But TUI/Studio treat them as visual groupings**, not clickable nodes
3. **5 separate modes** (Meta/Data/Overlay/Query/Atlas) create confusion
4. **Emoji in code** instead of proper icon system

**Principle**: "If it's a node in Neo4j, it's a node everywhere"

## Solution

### Header Simplification

**Before (v11.6)**: `[1]Meta [2]Data [3]Overlay [4]Query [5]Atlas`
**After (v11.7)**: `[1]Graph [2]Nexus`

- All 5 modes merged into single `[1]Graph` with unified tree
- Search via `[/]` overlay (no separate Query mode)
- Atlas removed, Audit moved to `[2]Nexus`

### Nexus Hub

```
[2]Nexus
├── Quiz (existing)
├── Audit (moved from Atlas)
├── Stats (dashboard)
└── Help
```

### Unified Tree Structure

Single tree view where everything is a node:

```
▼ ◉ Realm:shared                    <- Node :Meta:Realm (clickable)
  ├─▼ ⚙ Layer:config                <- Node :Meta:Layer (clickable)
  │   ├─▼ ◆ Kind:Locale [20]        <- Node :Meta:Kind + instance count
  │   │   ├─ ● Locale:fr-FR         <- Node :Locale (DATA instance)
  │   │   ├─ ● Locale:en-US         <- Node :Locale (DATA instance)
  │   │   └─ ● Locale:de-DE         <- Node :Locale (DATA instance)
  │   └─▼ ◆ Kind:EntityCategory [13]
  │       ├─ ● EntityCategory:thing
  │       └─ ● EntityCategory:person
  ├─▼ ◌ Layer:locale
  ├─▼ ▧ Layer:geography
  └─▼ ◇ Layer:knowledge
▼ ◎ Realm:org
  ├─▼ ⚙ Layer:config
  │   └─▼ ◆ Kind:OrgConfig [1]
  │       └─ ● OrgConfig:acme-corp
  └─▼ ...
```

### Header Changes

**Before (v11.6)**:
```
[1]Meta [2]Data [3]Overlay [4]Query [5]Atlas | ... | Nexus |
```

**After (v11.7)**:
```
[1]Graph [2]Arcs [3]Query [4]Nexus | ... |
```

- `[1]Meta + [2]Data + [3]Overlay` -> `[1]Graph` (unified tree)
- `[5]Atlas` -> REMOVED
- `Audit` -> Moved into `[4]Nexus`

### Nexus Hub

```
Nexus
├── Quiz (existing)
├── Audit (moved from Atlas)
├── Stats (dashboard)
└── Help
```

## Current UI Elements to Keep

The current TUI has excellent visual indicators that must be preserved:

### 1. Trait Icons (Shape = Locale Behavior)

| Trait | Icon | Abbrev | Meaning |
|-------|------|--------|---------|
| invariant | `■` | (inv) | Same across all locales |
| localized | `□` | (loc) | Generated per locale |
| knowledge | `◇` | (kno) | Deep locale expertise |
| generated | `★` | (gen) | LLM output |
| aggregated | `⋆` | (agg) | Computed metrics |

### 2. Arc Counts (Schema Connections)

```
→N  = outgoing arcs count
←N  = incoming arcs count

Example: Entity →30 ←35
```

### 3. Property Counts

```
⊞required/total

Example: ⊞6/9 = 6 required properties, 9 total
```

### 4. Instance Counts (Data Mode)

```
Kind (N)   = number of data instances
Layer (N)  = total instances in layer

Example: Entity (281), Locale (200), Semantic Layer (562)
```

### 5. Colored Badges (Right Column)

```
●org   = realm Organization (purple)
◎shd   = realm Shared (teal)
◎cfg   = layer Config (gray)
◆sem   = layer Semantic (blue)
▣str   = layer Structure (cyan)
▤fnd   = layer Foundation (orange)
▥ins   = layer Instruction (yellow)
●out   = layer Output (green)
◇kno   = layer Knowledge (purple)
▧geo   = layer Geography (green)
```

### 6. Layer Headers with Kind Count

```
Foundation         ◇3   = 3 kinds in this layer
Semantic Layer     ◇4   = 4 kinds
Instructions       ◇7   = 7 kinds
```

### 7. Unified Kind Line Format

```
[trait](abbr) Name (instances) →out ←in ⊞req/total      [realm] [layer]

Examples:
■(inv) Entity (281) →30 ←35 ⊞6/9                        ●org   ◆sem
□(loc) EntityContent (281) →5 ←4 ⊞13/22                 ●org   ◆sem
◇(kno) Term (0) →1 ←1 ⊞8/12                             ◎shd   ◇kno
★(gen) PageGenerated (0) →9 ←10 ⊞10/13                  ●org   ●out
```

### 8. Color Architecture (Single Source of Truth)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  taxonomy.yaml                   visual-encoding.yaml                        │
│  (SOURCE OF TRUTH)               (HOW TO USE)                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  node_realms:                    channel_mapping:                           │
│    shared:                         node:                                    │
│      color: "#2aa198"    ───────>    fill_color: layer                      │
│    org:                              border_color: realm                    │
│      color: "#6c71c4"                                                       │
│                                                                             │
│  node_layers:                    "Use taxonomy colors,                      │
│    config:                        don't define new ones"                    │
│      color: "#64748b"                                                       │
│    semantic:                                                                │
│      color: "#f97316"                                                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Colors from taxonomy.yaml** (TUI + Studio use the SAME values):

| Category | Key | Color | Hex |
|----------|-----|-------|-----|
| Realm | shared | Teal | #2aa198 |
| Realm | org | Violet | #6c71c4 |
| Layer | config | Gray | #64748b |
| Layer | locale | Gray | #64748b |
| Layer | geography | Green | #10b981 |
| Layer | knowledge | Purple | #8b5cf6 |
| Layer | semantic | Orange | #f97316 |
| Layer | foundation | Blue | #3b82f6 |
| Layer | structure | Cyan | #06b6d4 |
| Layer | instruction | Yellow | #eab308 |
| Layer | output | Green | #22c55e |

**Rule**: NO hex values in visual-encoding.yaml. Reference taxonomy.yaml only.

## Visual Encoding

### Icons (No Emoji)

File: `packages/core/models/visual-encoding.yaml`

```yaml
icons:
  meta_types:
    realm:
      terminal: "◉"
      web: "globe"         # Lucide
    layer:
      terminal: "▣"
      web: "layers"
    kind:
      terminal: "◆"
      web: "box"
    instance:
      terminal: "●"
      web: "circle"

  realms:
    shared:
      terminal: "◉"
      web: "globe"
    org:
      terminal: "◎"
      web: "building"

  layers:
    config:
      terminal: "⚙"
      web: "settings"
    locale:
      terminal: "◌"
      web: "languages"
    geography:
      terminal: "▧"
      web: "map"
    knowledge:
      terminal: "◇"
      web: "brain"
    foundation:
      terminal: "▤"
      web: "home"
    structure:
      terminal: "▦"
      web: "layout"
    semantic:
      terminal: "◈"
      web: "sparkles"
    instruction:
      terminal: "▥"
      web: "file-text"
    output:
      terminal: "●"
      web: "package"
```

## Behavior Changes

| Aspect | Before (v11.6) | After (v11.7) |
|--------|----------------|---------------|
| Modes | 4 (Data/Meta/Overlay/Query) | 1 unified + Query |
| Realm/Layer | Visual groupings | Clickable nodes |
| Kind instances | Hidden | Expandable under Kind |
| Icons | Emoji in code | Unicode (TUI) + Lucide (Studio) |
| Atlas | Separate mode | Removed |
| Audit | In Atlas | In Nexus |

### Interactions

| Action | Result |
|--------|--------|
| Click Realm:shared | Panel shows Realm props (color, display_name, etc.) |
| Click Layer:config | Panel shows Layer props + HAS_LAYER relationships |
| Click Kind:Locale | Panel shows schema (props, arcs, llm_context) |
| Click Locale:fr-FR | Panel shows instance data |
| Expand Kind:Locale | Lazy-loads instances from Neo4j |

### Cypher Queries (Simplified)

```cypher
-- All nodes in shared realm (via graph traversal, not property filter)
MATCH (r:Realm {key:'shared'})-[*]->(n)
RETURN n

-- All Locale instances
MATCH (k:Kind {label:'Locale'})<-[:OF_KIND]-(i)
RETURN i

-- Layer hierarchy
MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
RETURN r, l, k
```

## Files to Update

### YAML (Source of Truth)
- `packages/core/models/visual-encoding.yaml` - Add meta_types icons
- `packages/core/models/taxonomy.yaml` - Add icon refs

### Rust (TUI)
- `tools/novanet/src/tui/data.rs` - Load Realm/Layer as nodes
- `tools/novanet/src/tui/ui.rs` - Make all nodes clickable + detail panel
- `tools/novanet/src/tui/theme.rs` - Unicode icons from YAML
- `tools/novanet/src/tui/app.rs` - Remove Data/Meta/Overlay modes, add Nexus hub

### TypeScript (Studio)
- `packages/core/src/graph/visual-encoding.ts` - Generated from YAML
- `apps/studio/src/components/graph/` - Lucide icons

### Generators (Rust)
- `tools/novanet/src/generators/visual_encoding.rs` - Generate TS from YAML

### Documentation
- `.claude/rules/novanet-terminology.md` - "Everything is a node"
- `.claude/rules/novanet-decisions.md` - ADR-022: Unified Tree
- `tools/novanet/CLAUDE.md` - TUI unified mode
- `CHANGELOG.md` - v11.7.0

## ADR-022: Unified Tree Architecture

**Status**: Approved (v11.7)

**Decision**: Unify Data/Meta/Overlay modes into single tree where Realm, Layer, Kind, and Instance are all clickable nodes.

**Rationale**:
- Consistency: "Node in Neo4j = Node everywhere"
- Simplicity: 1 mode instead of 4
- Discoverability: Users see the full hierarchy
- Query-friendly: Graph traversal instead of property filtering

**Consequences**:
- Remove `[1]Meta [2]Data [3]Overlay [5]Atlas` from header
- Add `[1]Graph` unified mode
- Move Audit to Nexus hub
- Update all icon references (no emoji)

## Migration

No data migration needed. This is a UI/UX change only.

The Neo4j nodes (`:Meta:Realm`, `:Meta:Layer`, etc.) already exist in `00.5-taxonomy.cypher`.

## ASCII Mockup: Final TUI Design

### Main View ([1]Graph)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet v11.7.0                              [1]Graph [2]Nexus    [?]Help [q]Quit│
├────────────────────────────────────────────┬────────────────────────────────────┤
│ [/] Search...                              │ SHARED > CONFIG > LOCALE           │
├────────────────────────────────────────────┼────────────────────────────────────┤
│                                            │                                    │
│ ▼ Nodes (60)                               │  ◆ Kind: Locale                    │
│   ▼ ◉ Shared                    ◇39  ◎shd │  ─────────────────────────────────  │
│     ▼ ⚙ Config                  ◇3   ◎cfg │                                    │
│       ▶ ■(inv) EntityCategory (13)        │  Realm:      shared                │
│              →1 ←1 ⊞7/8         ◎shd c    │  Layer:      config                │
│     > ▼ ■(inv) Locale (200)               │  Trait:      invariant             │
│              →29 ←10 ⊞13/14     ◎shd c    │  Instances:  200                   │
│       ▶ ■(inv) SEOKeywordFormat (0)       │                                    │
│              →1 ←1 ⊞6/8         ◎shd c    │  Properties:                       │
│     ▶ ◌ Locale                  ◇6   ◎loc │  ├─ key: string (required)         │
│     ▶ ▧ Geography               ◇6   ◎geo │  ├─ display_name: string (req)     │
│     ▶ ◇ Knowledge              ◇24   ◇kno │  ├─ bcp47_code: string (req)       │
│   ▼ ◎ Organization             ◇21   ●org │  └─ fallback_chain: string[]       │
│     ▼ ⚙ Config                  ◇1   ◎cfg │                                    │
│       ▶ ■(inv) OrgConfig (1)              │  Outgoing Arcs (29):               │
│              →1 ←1 ⊞6/9         ●org c    │  ├─ HAS_CULTURE → CultureSet       │
│     ▼ ▤ Foundation              ◇3   ▤fnd │  ├─ HAS_TERMS → TermSet            │
│       ▶ ■(inv) BrandIdentity (1)          │  ├─ HAS_STYLE → Style              │
│       ▶ ■(inv) Project (2)                │  └─ ... +26 more                   │
│       ▶ □(loc) ProjectContent (2)         │                                    │
│     ▼ ◆ Semantic               ◇4    ◆sem │  Incoming Arcs (10):               │
│       ▶ ■(inv) Entity (281) →30 ←35       │  ├─ FOR_LOCALE ← EntityContent     │
│       ▶ □(loc) EntityContent (281)        │  └─ ... +9 more                    │
│     ▶ ▣ Structure               ◇3   ▣str │                                    │
│     ▶ ▥ Instructions            ◇7   ▥ins │                                    │
│     ▶ ● Output                  ◇3   ●out │                                    │
│                                            │                                    │
│ ▶ Arcs (114)                               │                                    │
│   ▶ → Ownership                [46]        │                                    │
│   ▶ ⇢ Localization             [15]        │                                    │
│   ▶ ⋯ Semantic                 [41]        │                                    │
│   ▶ ═ Generation               [11]        │                                    │
│   ▶ ┄ Mining                    [1]        │                                    │
│                                            │                                    │
├────────────────────────────────────────────┴────────────────────────────────────┤
│ j/k:nav  h/l:expand  /:search  Enter:select  Tab:panel  ?:help  q:quit          │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Expanded Kind with Instances

```
│     ▼ ■(inv) Locale (200) →29 ←10 ⊞13/14     ◎shd c    │
│       ├─ ● fr-FR "Français (France)"                   │
│       ├─ ● en-US "English (US)"                        │
│       ├─ ● de-DE "Deutsch (Deutschland)"               │
│       ├─ ● es-ES "Español (España)"                    │
│       ├─ ● ja-JP "日本語 (日本)"                         │
│       └─ ... +195 more [Enter to load]                 │
```

**Instance Pagination**: First 10 instances loaded by default, "Load more" fetches next batch.

### Arcs Tree with ArcFamily Nodes

ArcFamily and ArcKind are NODES in Neo4j (`:Meta:ArcFamily`, `:Meta:ArcKind`).
Badge indicators show this clearly:

```
▶ Arcs (114)
  ▼ → ownership [46]                                 ●fam   ← ArcFamily node
    → HAS_PROJECT →1 ←1 (OrgConfig → Project)       ●arc   ← ArcKind node
    → HAS_PAGE →1 ←1 (Project → Page)               ●arc
    → HAS_BLOCK →1 ←M (Page → Block)                ●arc
    → HAS_ENTITY →1 ←M (Project → Entity)           ●arc
    → ... +42 more
  ▶ ⇢ localization [15]                              ●fam
  ▶ ⋯ semantic [41]                                  ●fam
  ▶ ═ generation [11]                                ●fam
  ▶ ┄ mining [1]                                     ●fam
```

**ArcKind Display Format**: `→ ARC_NAME →out ←in (Source → Target)`

| Element | Meaning |
|---------|---------|
| `→ HAS_PROJECT` | Arc direction + name |
| `→1` | Outgoing cardinality (1) |
| `←1` | Incoming cardinality (1) |
| `(OrgConfig → Project)` | Source → Target types |
| `●fam` | Badge: this is an ArcFamily node |
| `●arc` | Badge: this is an ArcKind node |

**Cardinality notation**: `1` = exactly one, `M` = many, `?` = zero-or-one

### Search Overlay ([/])

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ Search: [entity________]                                                        │
│ ─────────────────────────────────────────────────────────────────────────────── │
│ ▶ ■(inv) Entity (org/semantic)                                        [Enter]  │
│   □(loc) EntityContent (org/semantic)                                           │
│   ■(inv) EntityCategory (shared/config)                                         │
│   → USES_ENTITY (semantic family)                                               │
│   → HAS_ENTITY (ownership family)                                               │
│ ─────────────────────────────────────────────────────────────────────────────── │
│ [Tab] Filters  [Enter] Select  [Esc] Cancel                                     │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### [2]Nexus Hub

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet v11.7.0                              [1]Graph [2]Nexus    [?]Help [q]Quit│
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ╔═══════════════════════════════════════════════════════════════════════════╗  │
│  ║                           N E X U S                                       ║  │
│  ╠═══════════════════════════════════════════════════════════════════════════╣  │
│  ║                                                                           ║  │
│  ║   [Q] Quiz          Test your NovaNet knowledge                           ║  │
│  ║                                                                           ║  │
│  ║   [A] Audit         Validate schema consistency                           ║  │
│  ║                                                                           ║  │
│  ║   [S] Stats         Dashboard with graph metrics                          ║  │
│  ║                                                                           ║  │
│  ║   [H] Help          Keybindings and documentation                         ║  │
│  ║                                                                           ║  │
│  ╚═══════════════════════════════════════════════════════════════════════════╝  │
│                                                                                 │
│  Stats Preview:                                                                 │
│  ├─ Nodes: 60 kinds, 1,247 instances                                           │
│  ├─ Arcs: 114 kinds, 3,891 relationships                                       │
│  ├─ Realms: 2 (shared: 39 kinds, org: 21 kinds)                                │
│  └─ Coverage: 85% properties documented                                        │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│ Press key to select  [1] Back to Graph  [q] Quit                                │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Panel Details by Node Type

Clicking any node shows its properties and relationships in the right panel.

### Realm Panel (e.g., Realm:shared)

```
┌─────────────────────────────────────────┐
│  ◉ Realm: shared                        │
│  ─────────────────────────────────────  │
│                                         │
│  key:          shared                   │
│  display_name: Shared                   │
│  color:        #2aa198                  │
│  description:  Universal locale knowledge│
│                                         │
│  Stats:                                 │
│  ├─ Layers:    4 (config, locale, ...)  │
│  ├─ Kinds:     39                       │
│  └─ Instances: 847                      │
│                                         │
│  Arcs:                                  │
│  ├─ HAS_LAYER → Layer (4)               │
│  └─ PART_OF ← Layer (4)                 │
└─────────────────────────────────────────┘
```

### Layer Panel (e.g., Layer:config)

```
┌─────────────────────────────────────────┐
│  ⚙ Layer: config                        │
│  ─────────────────────────────────────  │
│                                         │
│  key:          config                   │
│  realm:        shared                   │
│  display_name: Config                   │
│  color:        #64748b                  │
│  description:  Configuration definitions│
│                                         │
│  Stats:                                 │
│  ├─ Kinds:     3                        │
│  └─ Instances: 214                      │
│                                         │
│  Arcs:                                  │
│  ├─ HAS_KIND → Kind (3)                 │
│  └─ PART_OF ← Realm:shared              │
└─────────────────────────────────────────┘
```

### ArcFamily Panel (e.g., ArcFamily:ownership)

```
┌─────────────────────────────────────────┐
│  → ArcFamily: ownership                 │
│  ─────────────────────────────────────  │
│                                         │
│  key:          ownership                │
│  display_name: Ownership                │
│  color:        #3b82f6                  │
│  description:  Parent-child hierarchy   │
│                                         │
│  Stats:                                 │
│  ├─ ArcKinds:  46                       │
│  └─ Instances: 2,847                    │
│                                         │
│  Contains:                              │
│  ├─ HAS_PROJECT (OrgConfig → Project)   │
│  ├─ HAS_PAGE (Project → Page)           │
│  └─ ... +44 more                        │
└─────────────────────────────────────────┘
```

## View System Integration

The unified tree works with the **Unified View System** (see `2026-02-11-unified-view-system-design.md`).

### View Categories (29 views)

| Category | Count | Purpose |
|----------|-------|---------|
| META | 3 | Schema exploration (Realm, Layer, Kind, ArcKind) |
| DATA | 10 | Instance exploration by realm/layer/purpose |
| OVERLAY | 3 | Meta + Data combined for debugging |
| CONTEXTUAL | 13 | Node-centered subgraphs |

### Icon Format (NO EMOJI)

All icons use **dual format** - different for Studio and TUI:

```yaml
icon:
  web: "diamond"      # Lucide icon name (Studio/React)
  terminal: "◆"       # Unicode symbol (TUI/Rust)
```

See `2026-02-11-unified-view-system-design.md` → "Icon Mapping" table for complete list.

### Files to Update

| File | Changes |
|------|---------|
| `packages/core/models/views/_registry.yaml` | Replace emoji → dual icons |
| `packages/core/models/views/*.yaml` (14) | Replace `icon: "🔷"` → `icon: { web, terminal }` |
| `packages/core/models/views/contextual/*.yaml` (13) | Same icon format change |
| `apps/studio/src/components/views/ViewPicker.tsx` | Use Lucide icons from view.icon.web |
| `tools/novanet/src/tui/views.rs` | Use Unicode icons from view.icon.terminal |

---

## TUI Implementation Details (Rust)

> **Skills/Agents to use**: `rust-core` (ownership, error handling, type-state), `rust-async` (tokio channels, spawn, select), `rust-pro` agent for architecture decisions.

### 1. Unified Tree Data Structure

Replace current `TreeItem<'a>` enum with an owned, unified structure that represents ALL node types equally.

```rust
// src/tui/data.rs — NEW unified tree node

/// Unique identifier for any tree node (enables O(1) lookups).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeId {
    Section(SectionKind),           // "nodes", "arcs"
    Realm(String),                  // "shared", "org"
    Layer(String, String),          // (realm, layer)
    Kind(String),                   // Kind label (e.g., "Locale")
    Instance(String, String),       // (kind, instance_key)
    ArcFamily(String),              // "ownership", "semantic"
    ArcKind(String),                // "HAS_PAGE", "USES_ENTITY"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionKind {
    Nodes,
    Arcs,
}

/// Unified tree node — every item is a "node" (clickable, has detail panel).
#[derive(Debug, Clone)]
pub struct UnifiedNode {
    pub id: NodeId,
    pub depth: u8,                          // 0=section, 1=realm/family, 2=layer, 3=kind, 4=instance
    pub display: NodeDisplay,               // How to render this node
    pub children: LazyChildren,             // Child loading state
    pub neo4j_labels: SmallVec<[String; 2]>, // e.g., [":Meta", ":Realm"]
}

/// Display properties for a tree node.
#[derive(Debug, Clone)]
pub struct NodeDisplay {
    pub icon: char,                         // Unicode icon (◉, ⚙, ◆, ●)
    pub label: String,                      // Display name
    pub badge: Option<Badge>,               // Right-side badge (●fam, ●arc, etc.)
    pub count: Option<usize>,               // Instance/child count
    pub stats: Option<NodeStats>,           // Arc counts, property counts
}

/// Badge for tree node (right column indicator).
#[derive(Debug, Clone, Copy)]
pub struct Badge {
    pub icon: char,                         // ●, ◎, ◆, etc.
    pub abbrev: &'static str,               // "fam", "arc", "shd", "org"
    pub color_key: &'static str,            // Key into Theme color map
}

/// Arc/property statistics for a node.
#[derive(Debug, Clone, Default)]
pub struct NodeStats {
    pub outgoing_arcs: u16,
    pub incoming_arcs: u16,
    pub required_props: u8,
    pub total_props: u8,
}

/// Lazy loading state for children.
#[derive(Debug, Clone)]
pub enum LazyChildren {
    /// Children not yet loaded (show "..." or spinner)
    NotLoaded,
    /// Loading in progress
    Loading,
    /// Loaded with pagination
    Loaded {
        items: Vec<NodeId>,
        total: usize,                       // Total available (may be > items.len())
        has_more: bool,                     // True if more pages available
    },
    /// Leaf node (no children possible)
    Leaf,
}
```

### 2. State Machine: `[1]Graph` / `[2]Nexus`

Replace current 5-mode `NavMode` with simplified 2-mode enum.

```rust
// src/tui/app.rs — NEW state machine

/// Navigation mode — v11.7 unified design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavMode {
    /// Unified graph view (Realm > Layer > Kind > Instance + Arcs)
    #[default]
    Graph,
    /// Nexus hub (Quiz, Audit, Stats, Help)
    Nexus,
}

impl NavMode {
    pub fn label(&self) -> &'static str {
        match self {
            NavMode::Graph => "Graph",
            NavMode::Nexus => "Nexus",
        }
    }

    pub fn key(&self) -> char {
        match self {
            NavMode::Graph => '1',
            NavMode::Nexus => '2',
        }
    }
}

/// Sub-state for Nexus hub navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NexusTab {
    #[default]
    Quiz,
    Audit,
    Stats,
    Help,
}

impl NexusTab {
    pub fn all() -> &'static [NexusTab] {
        &[NexusTab::Quiz, NexusTab::Audit, NexusTab::Stats, NexusTab::Help]
    }

    pub fn key(&self) -> char {
        match self {
            NexusTab::Quiz => 'Q',
            NexusTab::Audit => 'A',
            NexusTab::Stats => 'S',
            NexusTab::Help => 'H',
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            NexusTab::Quiz => "Quiz",
            NexusTab::Audit => "Audit",
            NexusTab::Stats => "Stats",
            NexusTab::Help => "Help",
        }
    }
}

/// Focus panel in Graph mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphFocus {
    #[default]
    Tree,       // Left panel (unified tree)
    Detail,     // Right panel (node properties)
}
```

### 3. Lazy Loading Pattern for Instances

Pagination with 10 initial items + "load more" trigger.

```rust
// src/tui/data.rs — Instance loading

/// Pagination constants.
pub const INITIAL_INSTANCE_BATCH: usize = 10;
pub const INSTANCE_PAGE_SIZE: usize = 50;

/// Request to load instances (sent to async task).
#[derive(Debug, Clone)]
pub struct InstanceLoadRequest {
    pub kind_label: String,
    pub offset: usize,
    pub limit: usize,
}

/// Response from instance loading (received from async task).
#[derive(Debug)]
pub struct InstanceLoadResponse {
    pub kind_label: String,
    pub instances: Vec<InstanceInfo>,
    pub total_count: usize,
    pub offset: usize,
}

impl TaxonomyTree {
    /// Load initial batch of instances for a Kind (first 10).
    /// Called when Kind node is expanded.
    pub async fn load_instances_batch(
        db: &Db,
        kind_label: &str,
        offset: usize,
        limit: usize,
    ) -> crate::Result<InstanceLoadResponse> {
        // Validate label (injection prevention)
        validate_cypher_label(kind_label)?;

        let cypher = format!(
            r#"
            MATCH (n:{label})-[:OF_KIND]->(k:Kind:Meta {{label: $kind}})
            WITH n ORDER BY n.key
            SKIP $offset LIMIT $limit
            RETURN n.key AS key,
                   coalesce(n.display_name, n.key) AS display_name,
                   labels(n) AS labels,
                   properties(n) AS props
            "#,
            label = kind_label
        );

        // Also get total count (separate query for performance)
        let count_cypher = format!(
            r#"
            MATCH (n:{label})-[:OF_KIND]->(k:Kind:Meta {{label: $kind}})
            RETURN count(n) AS total
            "#,
            label = kind_label
        );

        // Execute both in parallel
        let (rows, count_rows) = tokio::join!(
            db.execute_with_params(&cypher, &[("kind", kind_label), ("offset", &offset), ("limit", &limit)]),
            db.execute_with_params(&count_cypher, &[("kind", kind_label)])
        );

        let instances = rows?.into_iter().map(|row| {
            InstanceInfo {
                key: row.get("key").unwrap_or_default(),
                display_name: row.get("display_name").unwrap_or_default(),
                labels: row.get("labels").unwrap_or_default(),
                properties: row.get("props").unwrap_or_default(),
                ..Default::default()
            }
        }).collect();

        let total_count = count_rows?
            .first()
            .and_then(|r| r.get::<i64>("total").ok())
            .unwrap_or(0) as usize;

        Ok(InstanceLoadResponse {
            kind_label: kind_label.to_string(),
            instances,
            total_count,
            offset,
        })
    }

    /// Check if "load more" should be shown for a Kind.
    pub fn has_more_instances(&self, kind_label: &str) -> bool {
        let loaded = self.instances.get(kind_label).map(|v| v.len()).unwrap_or(0);
        let total = self.instance_totals.get(kind_label).copied().unwrap_or(0);
        loaded < total
    }
}
```

### 4. Async Task Communication (Tokio Channels)

Use `mpsc` channels for TUI-to-async communication.

```rust
// src/tui/mod.rs — Channel architecture

use tokio::sync::mpsc;

/// Commands sent from TUI to async worker.
#[derive(Debug)]
pub enum AsyncCommand {
    LoadInstances(InstanceLoadRequest),
    LoadRealmDetails(String),
    LoadLayerDetails(String, String),  // (realm, layer)
    LoadArcKindDetails(String),
    RefreshTree,
    Shutdown,
}

/// Events received by TUI from async worker.
#[derive(Debug)]
pub enum AsyncEvent {
    InstancesLoaded(InstanceLoadResponse),
    RealmDetailsLoaded(RealmDetails),
    LayerDetailsLoaded(LayerDetails),
    ArcKindDetailsLoaded(ArcKindDetails),
    TreeRefreshed(TaxonomyTree),
    Error(String),
}

/// Async worker task — handles all Neo4j queries off the main thread.
pub async fn async_worker(
    db: Db,
    mut rx: mpsc::Receiver<AsyncCommand>,
    tx: mpsc::Sender<AsyncEvent>,
) {
    while let Some(cmd) = rx.recv().await {
        let result = match cmd {
            AsyncCommand::LoadInstances(req) => {
                match TaxonomyTree::load_instances_batch(&db, &req.kind_label, req.offset, req.limit).await {
                    Ok(resp) => tx.send(AsyncEvent::InstancesLoaded(resp)).await,
                    Err(e) => tx.send(AsyncEvent::Error(e.to_string())).await,
                }
            }
            AsyncCommand::Shutdown => break,
            // ... other commands
        };
        if result.is_err() {
            break; // TUI closed
        }
    }
}
```

### 5. Badge Pattern (`●fam` / `●arc`)

Consistent badge rendering with color lookup.

```rust
// src/tui/theme.rs — Badge definitions

/// Pre-defined badges for tree nodes.
pub mod badges {
    use super::Badge;

    // Meta-type badges
    pub const REALM_SHARED: Badge = Badge { icon: '◎', abbrev: "shd", color_key: "realm.shared" };
    pub const REALM_ORG: Badge = Badge { icon: '●', abbrev: "org", color_key: "realm.org" };
    pub const ARC_FAMILY: Badge = Badge { icon: '●', abbrev: "fam", color_key: "arc.family" };
    pub const ARC_KIND: Badge = Badge { icon: '●', abbrev: "arc", color_key: "arc.kind" };

    // Layer badges
    pub const LAYER_CONFIG: Badge = Badge { icon: '◎', abbrev: "cfg", color_key: "layer.config" };
    pub const LAYER_LOCALE: Badge = Badge { icon: '◎', abbrev: "loc", color_key: "layer.locale" };
    pub const LAYER_GEOGRAPHY: Badge = Badge { icon: '▧', abbrev: "geo", color_key: "layer.geography" };
    pub const LAYER_KNOWLEDGE: Badge = Badge { icon: '◇', abbrev: "kno", color_key: "layer.knowledge" };
    pub const LAYER_FOUNDATION: Badge = Badge { icon: '▤', abbrev: "fnd", color_key: "layer.foundation" };
    pub const LAYER_STRUCTURE: Badge = Badge { icon: '▣', abbrev: "str", color_key: "layer.structure" };
    pub const LAYER_SEMANTIC: Badge = Badge { icon: '◆', abbrev: "sem", color_key: "layer.semantic" };
    pub const LAYER_INSTRUCTION: Badge = Badge { icon: '▥', abbrev: "ins", color_key: "layer.instruction" };
    pub const LAYER_OUTPUT: Badge = Badge { icon: '●', abbrev: "out", color_key: "layer.output" };
}

impl Badge {
    /// Render badge as ratatui Span with appropriate color.
    pub fn to_span(&self, theme: &Theme) -> Span<'static> {
        let color = theme.color_for_key(self.color_key);
        Span::styled(
            format!("{}{}", self.icon, self.abbrev),
            Style::default().fg(color)
        )
    }
}

// src/tui/ui.rs — Badge in tree line

fn render_tree_line(node: &UnifiedNode, theme: &Theme) -> Line<'static> {
    let mut spans = vec![
        // Indent
        Span::raw("  ".repeat(node.depth as usize)),
        // Expand indicator
        Span::raw(match &node.children {
            LazyChildren::Leaf => "  ",
            LazyChildren::NotLoaded | LazyChildren::Loading => "▶ ",
            LazyChildren::Loaded { .. } => "▼ ",
        }),
        // Icon
        Span::styled(format!("{} ", node.display.icon), theme.icon_style(&node.id)),
        // Label
        Span::raw(&node.display.label),
    ];

    // Count (if present)
    if let Some(count) = node.display.count {
        spans.push(Span::styled(format!(" ({})", count), Style::default().fg(Color::DarkGray)));
    }

    // Stats (→N ←N ⊞r/t)
    if let Some(stats) = &node.display.stats {
        spans.push(Span::styled(
            format!(" →{} ←{} ⊞{}/{}",
                stats.outgoing_arcs, stats.incoming_arcs,
                stats.required_props, stats.total_props),
            Style::default().fg(Color::DarkGray)
        ));
    }

    // Right-align badge
    if let Some(badge) = &node.display.badge {
        spans.push(Span::raw(" ")); // Spacer (filled by right-align in render)
        spans.push(badge.to_span(theme));
    }

    Line::from(spans)
}
```

### 6. Tree Flattening with Lazy Children

Efficient flattening that respects collapsed state and lazy loading.

```rust
// src/tui/data.rs — Flattening

impl TaxonomyTree {
    /// Flatten tree for display, respecting collapsed state and lazy children.
    /// Returns (flattened nodes, node_id -> flat_index map).
    pub fn flatten(&self) -> (Vec<&UnifiedNode>, FxHashMap<NodeId, usize>) {
        let mut flat = Vec::with_capacity(256); // Pre-allocate for typical tree size
        let mut index_map = FxHashMap::default();

        fn recurse<'a>(
            nodes: &'a [UnifiedNode],
            collapsed: &FxHashSet<String>,
            flat: &mut Vec<&'a UnifiedNode>,
            index_map: &mut FxHashMap<NodeId, usize>,
        ) {
            for node in nodes {
                let idx = flat.len();
                flat.push(node);
                index_map.insert(node.id.clone(), idx);

                // Only recurse if expanded and children are loaded
                let key = node.id.collapse_key();
                if !collapsed.contains(&key) {
                    if let LazyChildren::Loaded { items, .. } = &node.children {
                        // Items are NodeIds — need to resolve to actual nodes
                        // (In practice, maintain a HashMap<NodeId, UnifiedNode>)
                    }
                }
            }
        }

        // Start with root sections
        recurse(&self.root_nodes, &self.collapsed, &mut flat, &mut index_map);

        (flat, index_map)
    }
}

impl NodeId {
    /// Key for collapsed state tracking.
    pub fn collapse_key(&self) -> String {
        match self {
            NodeId::Section(s) => format!("section:{:?}", s),
            NodeId::Realm(r) => format!("realm:{}", r),
            NodeId::Layer(r, l) => format!("layer:{}:{}", r, l),
            NodeId::Kind(k) => format!("kind:{}", k),
            NodeId::Instance(k, i) => format!("instance:{}:{}", k, i),
            NodeId::ArcFamily(f) => format!("arcfamily:{}", f),
            NodeId::ArcKind(a) => format!("arckind:{}", a),
        }
    }
}
```

### 7. Implementation Roadmap

| Phase | Files | Changes | Effort |
|-------|-------|---------|--------|
| 1 | `app.rs` | Replace `NavMode` enum (5 -> 2), update key handlers | 2h |
| 2 | `data.rs` | Add `UnifiedNode`, `NodeId`, `LazyChildren` types | 4h |
| 3 | `data.rs` | Refactor `TaxonomyTree::load()` to build unified nodes | 4h |
| 4 | `data.rs` | Implement `load_instances_batch()` with pagination | 2h |
| 5 | `theme.rs` | Add `Badge`, `badges::*` constants, `to_span()` | 1h |
| 6 | `ui.rs` | Update `render_tree_line()` for unified nodes | 3h |
| 7 | `ui.rs` | Update detail panel for ALL node types (Realm, Layer, ArcFamily) | 3h |
| 8 | `mod.rs` | Add `AsyncCommand`/`AsyncEvent` channels | 2h |
| 9 | `nexus.rs` | Add Audit tab (move from Atlas) | 2h |
| 10 | Tests | Update 50+ tests for new structures | 4h |

**Total**: ~27 hours (3-4 days focused work)

### 8. Key Considerations

1. **Memory**: `UnifiedNode` is larger than `TreeItem<'a>` (owned vs borrowed). For 60 kinds + ~1000 instances, this is ~200KB — acceptable.

2. **Performance**: Lazy loading means initial render is fast. Use `SmallVec<[String; 2]>` for labels to avoid heap allocation for common cases.

3. **Backwards compatibility**: Keep old `TreeItem` temporarily with `#[deprecated]` attribute during migration.

4. **Testing**: Use `proptest` for edge cases in flattening logic. Use `insta` snapshots for rendered output.

5. **Error handling**: All Neo4j queries return `crate::Result<T>`. Display errors in status bar, never panic.

---

## Summary of Changes

| Aspect | Before (v11.6) | After (v11.7) |
|--------|----------------|---------------|
| Header tabs | 5 (Meta/Data/Overlay/Query/Atlas) | 2 (Graph/Nexus) |
| Tree structure | Realm/Layer as folders | Realm/Layer as clickable nodes |
| Instances | Hidden or separate Data mode | Under Kind, expandable (10 + load more) |
| ArcFamily/ArcKind | Visual groupings | Clickable nodes with `●fam`/`●arc` badges |
| Search | Separate Query mode | `[/]` overlay in Graph |
| Atlas | Separate mode | Removed |
| Audit | In Atlas | In Nexus hub |
| Icons | Mixed emoji | Dual: Lucide (web) + Unicode (terminal) |
| Node panels | Schema-only for Kind | Props + stats for ALL node types |
| Views | 29 views with emoji | 29 views with dual icons |

---

## Testing Strategy

This section defines the comprehensive testing approach for the Unified Tree Architecture implementation.

### Recommended Skills and Agents

**Before coding**, invoke these skills:

| Skill/Agent | When to Use | Purpose |
|-------------|-------------|---------|
| `test-driven-development` | Before ANY implementation | Write tests first, watch fail, implement, pass |
| `spn-powers:testing-anti-patterns` | When writing tests | Avoid mocking without understanding, test-only production code |
| `feature-dev:code-reviewer` | After each batch | Review implementation against tests and design |

**Workflow**:
```
1. /test-driven-development → Write failing test
2. Implement minimal code to pass
3. /spn-powers:testing-anti-patterns → Verify test quality
4. Repeat for next feature
5. /feature-dev:code-reviewer → Review completed batch
```

### 1. Unit Tests: Rust (TUI)

**Framework**: `cargo test` + `insta` (snapshots) + `proptest` (property-based)

#### Tree Rendering Snapshots (insta)

File: `tools/novanet/src/tui/tests/tree_render_test.rs`

```rust
use insta::assert_snapshot;

#[test]
fn test_unified_tree_collapsed() {
    let tree = UnifiedTree::mock_schema();
    let rendered = tree.render_collapsed();
    assert_snapshot!("tree_collapsed", rendered);
}

#[test]
fn test_unified_tree_expanded_realm() {
    let tree = UnifiedTree::mock_schema();
    tree.expand("Realm:shared");
    let rendered = tree.render();
    assert_snapshot!("tree_expanded_realm", rendered);
}

#[test]
fn test_unified_tree_with_instances() {
    let tree = UnifiedTree::mock_schema_with_instances();
    tree.expand("Kind:Locale");
    let rendered = tree.render();
    assert_snapshot!("tree_with_instances", rendered);
}

#[test]
fn test_arcs_tree_collapsed() {
    let tree = ArcsTree::mock_schema();
    let rendered = tree.render_collapsed();
    assert_snapshot!("arcs_tree_collapsed", rendered);
}

#[test]
fn test_arcs_tree_expanded_family() {
    let tree = ArcsTree::mock_schema();
    tree.expand("ArcFamily:ownership");
    let rendered = tree.render();
    assert_snapshot!("arcs_tree_expanded_ownership", rendered);
}
```

**Snapshot location**: `tools/novanet/src/tui/tests/snapshots/`

#### Data Loading Tests

File: `tools/novanet/src/tui/tests/data_test.rs`

```rust
#[test]
fn test_taxonomy_tree_loads_realms_as_nodes() {
    let tree = TaxonomyTree::from_yaml(mock_taxonomy());
    assert!(tree.nodes.contains_key("Realm:shared"));
    assert!(tree.nodes.contains_key("Realm:org"));
}

#[test]
fn test_taxonomy_tree_loads_layers_as_nodes() {
    let tree = TaxonomyTree::from_yaml(mock_taxonomy());
    assert!(tree.nodes.contains_key("Layer:config"));
    assert!(tree.nodes.get("Layer:config").unwrap().parent == Some("Realm:shared"));
}

#[test]
fn test_kind_counts_per_layer() {
    let tree = TaxonomyTree::from_yaml(mock_taxonomy());
    let config_layer = tree.nodes.get("Layer:config").unwrap();
    assert_eq!(config_layer.child_count, 3); // EntityCategory, Locale, SEOKeywordFormat
}

#[test]
fn test_instance_count_badge() {
    let tree = TaxonomyTree::with_mock_instances();
    let locale_kind = tree.nodes.get("Kind:Locale").unwrap();
    assert_eq!(locale_kind.instance_count, 200);
}
```

#### Theme/Icon Tests

File: `tools/novanet/src/tui/tests/theme_test.rs`

```rust
#[test]
fn test_icons_loaded_from_yaml() {
    let theme = Theme::with_root(Path::new("../../packages/core"));
    assert_eq!(theme.icons.realms.shared.terminal, "◉");
    assert_eq!(theme.icons.layers.config.terminal, "⚙");
}

#[test]
fn test_no_emoji_in_icons() {
    let theme = Theme::with_root(Path::new("../../packages/core"));
    for icon in theme.all_terminal_icons() {
        assert!(!is_emoji(icon), "Found emoji: {}", icon);
    }
}

#[test]
fn test_fallback_icons_on_yaml_failure() {
    let theme = Theme::with_root(Path::new("/nonexistent"));
    assert!(!theme.icons.realms.shared.terminal.is_empty());
}
```

### 2. Unit Tests: TypeScript (Studio)

**Framework**: Vitest + React Testing Library

#### Store Tests (Vitest)

File: `apps/studio/src/stores/__tests__/graphStore.test.ts`

```typescript
import { describe, it, expect, beforeEach } from 'vitest';
import { useGraphStore } from '../graphStore';

describe('graphStore', () => {
  beforeEach(() => {
    useGraphStore.getState().reset();
  });

  it('should treat Realm nodes as clickable', () => {
    const store = useGraphStore.getState();
    store.setNodes([{ id: 'Realm:shared', type: 'realm', data: { key: 'shared' } }]);

    expect(store.isClickable('Realm:shared')).toBe(true);
  });

  it('should treat Layer nodes as clickable', () => {
    const store = useGraphStore.getState();
    store.setNodes([{ id: 'Layer:config', type: 'layer', data: { key: 'config' } }]);

    expect(store.isClickable('Layer:config')).toBe(true);
  });

  it('should load instances under Kind on expand', async () => {
    const store = useGraphStore.getState();
    store.setNodes([{ id: 'Kind:Locale', type: 'kind', data: { name: 'Locale' } }]);

    await store.expandKind('Kind:Locale');

    const instances = store.getChildrenOf('Kind:Locale');
    expect(instances.length).toBeGreaterThan(0);
    expect(instances[0].type).toBe('instance');
  });
});
```

#### Component Tests (React Testing Library)

File: `apps/studio/src/components/graph/__tests__/UnifiedTreeNode.test.tsx`

```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { UnifiedTreeNode } from '../UnifiedTreeNode';

describe('UnifiedTreeNode', () => {
  it('renders Realm with correct icon', () => {
    render(<UnifiedTreeNode type="realm" data={{ key: 'shared' }} />);

    // Lucide Globe icon for shared realm
    expect(screen.getByTestId('icon-globe')).toBeInTheDocument();
    expect(screen.getByText('Shared')).toBeInTheDocument();
  });

  it('renders Layer with kind count badge', () => {
    render(<UnifiedTreeNode type="layer" data={{ key: 'config', kindCount: 3 }} />);

    expect(screen.getByText('◇3')).toBeInTheDocument();
  });

  it('renders Kind with instance count', () => {
    render(<UnifiedTreeNode type="kind" data={{ name: 'Locale', instanceCount: 200 }} />);

    expect(screen.getByText('(200)')).toBeInTheDocument();
  });

  it('fires onClick for all node types', () => {
    const onClick = vi.fn();
    render(<UnifiedTreeNode type="realm" data={{ key: 'shared' }} onClick={onClick} />);

    fireEvent.click(screen.getByRole('button'));
    expect(onClick).toHaveBeenCalledWith('Realm:shared');
  });

  it('uses Lucide icons, not emoji', () => {
    render(<UnifiedTreeNode type="layer" data={{ key: 'config' }} />);

    // Should use Settings icon from Lucide, not emoji
    expect(screen.queryByText(/[\u{1F300}-\u{1F9FF}]/u)).not.toBeInTheDocument();
    expect(screen.getByTestId('icon-settings')).toBeInTheDocument();
  });
});
```

#### Panel Tests

File: `apps/studio/src/components/panels/__tests__/NodePanel.test.tsx`

```typescript
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { NodePanel } from '../NodePanel';

describe('NodePanel', () => {
  it('shows Realm properties when Realm selected', () => {
    render(<NodePanel nodeId="Realm:shared" />);

    expect(screen.getByText('key:')).toBeInTheDocument();
    expect(screen.getByText('shared')).toBeInTheDocument();
    expect(screen.getByText('color:')).toBeInTheDocument();
    expect(screen.getByText('#2aa198')).toBeInTheDocument();
  });

  it('shows Layer stats with kind count', () => {
    render(<NodePanel nodeId="Layer:config" />);

    expect(screen.getByText('Kinds:')).toBeInTheDocument();
    expect(screen.getByText('3')).toBeInTheDocument();
  });

  it('shows ArcFamily with arc list', () => {
    render(<NodePanel nodeId="ArcFamily:ownership" />);

    expect(screen.getByText('ArcKinds:')).toBeInTheDocument();
    expect(screen.getByText('HAS_PROJECT')).toBeInTheDocument();
  });
});
```

### 3. Integration Tests

#### TUI with Neo4j Mock

File: `tools/novanet/src/tui/tests/integration_test.rs`

```rust
use neo4rs::Graph;
use testcontainers::{clients::Cli, images::neo4j::Neo4j};

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn test_tui_loads_real_tree() {
    let docker = Cli::default();
    let neo4j = docker.run(Neo4j::default());
    let graph = Graph::new(&neo4j.get_host_port_ipv4(7687), "neo4j", "password").await.unwrap();

    // Seed taxonomy
    seed_taxonomy(&graph).await;

    let tree = TaxonomyTree::from_neo4j(&graph).await.unwrap();

    assert!(tree.nodes.contains_key("Realm:shared"));
    assert!(tree.nodes.contains_key("Realm:org"));
    assert_eq!(tree.nodes.len(), 60 + 10 + 5 + 2); // Kinds + Layers + Families + Realms
}

#[tokio::test]
#[ignore]
async fn test_kind_expand_loads_instances() {
    let docker = Cli::default();
    let neo4j = docker.run(Neo4j::default());
    let graph = Graph::new(&neo4j.get_host_port_ipv4(7687), "neo4j", "password").await.unwrap();

    seed_taxonomy(&graph).await;
    seed_locales(&graph, 200).await;

    let mut tree = TaxonomyTree::from_neo4j(&graph).await.unwrap();
    tree.expand_with_instances(&graph, "Kind:Locale", 10).await.unwrap();

    let locale_children = tree.children_of("Kind:Locale");
    assert_eq!(locale_children.len(), 10); // First batch
}
```

#### Studio E2E (Playwright)

File: `apps/studio/e2e/unified-tree.spec.ts`

```typescript
import { test, expect } from '@playwright/test';

test.describe('Unified Tree', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="graph-loaded"]');
  });

  test('clicking Realm shows Realm panel', async ({ page }) => {
    await page.click('[data-testid="node-Realm:shared"]');

    await expect(page.locator('[data-testid="panel-title"]')).toContainText('Realm: shared');
    await expect(page.locator('[data-testid="panel-content"]')).toContainText('#2aa198');
  });

  test('clicking Layer shows Layer panel', async ({ page }) => {
    await page.click('[data-testid="node-Realm:shared"]'); // Expand
    await page.click('[data-testid="node-Layer:config"]');

    await expect(page.locator('[data-testid="panel-title"]')).toContainText('Layer: config');
  });

  test('expanding Kind loads instances', async ({ page }) => {
    await page.click('[data-testid="node-Realm:shared"]');
    await page.click('[data-testid="node-Layer:config"]');
    await page.dblclick('[data-testid="node-Kind:Locale"]');

    await expect(page.locator('[data-testid="instance-Locale:fr-FR"]')).toBeVisible();
  });

  test('search overlay finds nodes', async ({ page }) => {
    await page.keyboard.press('/');
    await page.fill('[data-testid="search-input"]', 'entity');

    await expect(page.locator('[data-testid="search-result-Kind:Entity"]')).toBeVisible();
    await expect(page.locator('[data-testid="search-result-Kind:EntityContent"]')).toBeVisible();
  });

  test('no emoji icons in UI', async ({ page }) => {
    const pageContent = await page.textContent('body');
    const emojiRegex = /[\u{1F300}-\u{1F9FF}]/gu;
    expect(pageContent).not.toMatch(emojiRegex);
  });
});
```

### 4. Property-Based Tests (proptest)

File: `tools/novanet/src/tui/tests/proptest_tree.rs`

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn tree_never_panics_on_expand(
        node_id in "[A-Za-z:_-]{1,50}"
    ) {
        let mut tree = TaxonomyTree::mock_schema();
        // Should not panic even with invalid node_id
        let _ = tree.expand(&node_id);
    }

    #[test]
    fn tree_never_panics_on_collapse(
        node_id in "[A-Za-z:_-]{1,50}"
    ) {
        let mut tree = TaxonomyTree::mock_schema();
        let _ = tree.collapse(&node_id);
    }

    #[test]
    fn tree_render_never_empty(
        expanded_nodes in prop::collection::vec("[A-Za-z:_-]{1,30}", 0..20)
    ) {
        let mut tree = TaxonomyTree::mock_schema();
        for node in &expanded_nodes {
            let _ = tree.expand(node);
        }
        let rendered = tree.render();
        prop_assert!(!rendered.is_empty());
    }

    #[test]
    fn instance_count_always_non_negative(
        count in any::<i32>()
    ) {
        let tree = TaxonomyTree::mock_schema();
        let locale = tree.nodes.get("Kind:Locale").unwrap();
        // Instance count should always be >= 0
        prop_assert!(locale.instance_count >= 0);
    }

    #[test]
    fn node_id_parsing_roundtrips(
        node_type in "(Realm|Layer|Kind|ArcFamily|ArcKind)",
        key in "[a-z_-]{1,30}"
    ) {
        let id = format!("{}:{}", node_type, key);
        let parsed = NodeId::parse(&id);
        prop_assert!(parsed.is_some());
        prop_assert_eq!(parsed.unwrap().to_string(), id);
    }

    #[test]
    fn breadcrumb_never_exceeds_depth(
        depth in 0usize..10
    ) {
        let tree = TaxonomyTree::mock_schema();
        let breadcrumb = tree.breadcrumb_for("Kind:Locale");
        // Max depth: Realm > Layer > Kind = 3
        prop_assert!(breadcrumb.len() <= 3);
    }
}
```

### 5. Test Matrix

This matrix maps files to required tests:

| File Changed | Unit Tests (Rust) | Unit Tests (TS) | Integration | E2E | Proptest |
|--------------|-------------------|-----------------|-------------|-----|----------|
| `tui/data.rs` | `data_test.rs` | - | `integration_test.rs` | - | `proptest_tree.rs` |
| `tui/ui.rs` | `tree_render_test.rs` | - | `integration_test.rs` | - | `proptest_tree.rs` |
| `tui/theme.rs` | `theme_test.rs` | - | - | - | - |
| `tui/app.rs` | `app_test.rs` | - | `integration_test.rs` | - | - |
| `graphStore.ts` | - | `graphStore.test.ts` | - | `unified-tree.spec.ts` | - |
| `UnifiedTreeNode.tsx` | - | `UnifiedTreeNode.test.tsx` | - | `unified-tree.spec.ts` | - |
| `NodePanel.tsx` | - | `NodePanel.test.tsx` | - | `unified-tree.spec.ts` | - |
| `visual-encoding.yaml` | `theme_test.rs` | `visual-encoding.test.ts` | - | - | - |
| `taxonomy.yaml` | `data_test.rs` | - | `integration_test.rs` | - | - |

### Test Commands

```bash
# Rust unit tests (fast, no Neo4j)
cargo nextest run --package novanet

# Rust snapshot tests (update with: cargo insta review)
cargo insta test --package novanet

# Rust property tests
cargo test proptest --package novanet

# Rust integration tests (requires Neo4j)
cargo test -- --ignored

# TypeScript unit tests
pnpm --filter @novanet/studio test

# TypeScript coverage
pnpm --filter @novanet/studio test:coverage

# E2E tests
pnpm --filter @novanet/studio e2e

# Full test suite (pre-commit)
cargo nextest run && pnpm test && pnpm e2e
```

### Snapshot Management

**Rust (insta)**:
```bash
# Run tests, review new snapshots
cargo insta test
cargo insta review

# Accept all snapshots (after visual inspection)
cargo insta accept
```

**Snapshot files location**:
- `tools/novanet/src/tui/tests/snapshots/*.snap`
- Named: `{test_name}.snap`

### Coverage Targets

| Component | Target | Tool |
|-----------|--------|------|
| Rust TUI | 80% | `cargo llvm-cov` |
| TypeScript Stores | 90% | Vitest coverage |
| TypeScript Components | 75% | Vitest coverage |
| E2E Critical Paths | 100% | Playwright |

### CI Integration

```yaml
# .github/workflows/test.yml
jobs:
  rust-tests:
    steps:
      - run: cargo nextest run
      - run: cargo insta test --check  # Fail if snapshots changed

  ts-tests:
    steps:
      - run: pnpm test
      - run: pnpm test:coverage

  e2e-tests:
    needs: [rust-tests, ts-tests]
    steps:
      - run: pnpm infra:up
      - run: pnpm e2e
```

### Anti-Patterns to Avoid

Per `spn-powers:testing-anti-patterns`:

1. **Don't mock Neo4j for unit tests** - Use real YAML fixtures instead
2. **Don't add test-only methods** to production code (e.g., `pub fn for_testing()`)
3. **Don't test implementation details** - Test behavior (click Realm -> panel shows)
4. **Don't skip snapshot review** - Always `cargo insta review` before accepting
5. **Don't ignore flaky tests** - Fix timing issues with proper async handling

---

## YAML Schema Updates

This section documents the exact YAML schema changes required to migrate from emoji icons to dual icons (web + terminal) across all configuration files.

### 1. Dual Icon Schema for visual-encoding.yaml

The `visual-encoding.yaml` already follows the dual icon pattern. Here's the canonical schema:

```yaml
# Schema for dual icons in visual-encoding.yaml
icons:
  <category>:         # e.g., realms, layers, traits, arc_families, states, etc.
    <key>:            # e.g., shared, org, config, semantic, etc.
      web: <string>   # Lucide icon name (https://lucide.dev/icons)
      terminal: <string>  # Single-width Unicode character for monospace alignment
      description: <string>  # Human-readable description (optional but recommended)
```

**Validation Rules:**
- `web`: Must be a valid Lucide icon name (lowercase, hyphenated)
- `terminal`: Must be a single Unicode character (for monospace alignment in TUI)
- `description`: Optional, max 80 characters

**Example:**
```yaml
icons:
  layers:
    config:
      web: settings
      terminal: "⚙"
      description: "Configuration and definitions"
    semantic:
      web: lightbulb
      terminal: "◆"
      description: "Entities and meaning"
```

### 2. View Schema with Dual Icons for _registry.yaml

**Current format (DEPRECATED):**
```yaml
views:
  - id: meta-complete
    icon: "🔷"           # WRONG: emoji
    color: "#8b5cf6"
    # ...
```

**Target format (v11.7):**
```yaml
views:
  - id: meta-complete
    icon:
      web: diamond       # Lucide icon name
      terminal: "◆"      # Unicode symbol
    color: "#8b5cf6"     # Hex color stays the same
    description: Complete meta-graph (Realm, Layer, Kind, Trait, ArcKind)
    category: meta
    modes: [meta]
    cypher: |
      MATCH (n:Meta) ...
```

**Full View Schema:**
```yaml
# packages/core/models/views/_registry.yaml
version: "11.7.0"
description: NovaNet Unified View System (29 views)

views:
  - id: <string>                    # Unique kebab-case identifier
    description: <string>           # Human-readable description
    icon:                           # NEW: dual icon format
      web: <string>                 # Lucide icon name
      terminal: <string>            # Unicode symbol
    color: <hex>                    # Hex color (e.g., "#8b5cf6")
    category: <meta|data|overlay|contextual>
    contextual: <boolean>           # true if shown in node sidebar (optional)
    applicable_types: [<NodeKind>]  # Filter for contextual views (optional)
    modes: [<mode>]                 # Navigation modes where view appears
    params: [<string>]              # Query parameters (optional)
    cypher: |                       # Neo4j Cypher query
      MATCH ...
```

### 3. Migration Path: Emoji to Dual Icons

**Phase 1: Update _registry.yaml**

Replace all emoji icons with dual icon format:

| Before (emoji) | After (dual icon) |
|----------------|-------------------|
| `icon: "🔷"` | `icon: { web: "diamond", terminal: "◆" }` |
| `icon: "🏛️"` | `icon: { web: "building-2", terminal: "◉" }` |
| `icon: "🔀"` | `icon: { web: "git-branch", terminal: "→" }` |
| `icon: "🌐"` | `icon: { web: "globe", terminal: "●" }` |
| (see complete list below) |

**Phase 2: Update TypeScript Types**

```typescript
// packages/core/src/types/views.ts

export interface ViewIcon {
  web: string;      // Lucide icon name
  terminal: string; // Unicode symbol
}

export interface ViewRegistryEntry {
  id: string;
  description: string;
  icon: ViewIcon;   // Changed from icon: string
  color: string;
  category: 'meta' | 'data' | 'overlay' | 'contextual';
  contextual?: boolean;
  applicable_types?: string[];
  modes: NavigationMode[];
  params?: string[];
  cypher: string;
}
```

**Phase 3: Update Rust TUI**

```rust
// tools/novanet/src/tui/views.rs

#[derive(Debug, Clone, Deserialize)]
pub struct ViewIcon {
    pub web: String,
    pub terminal: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ViewRegistryEntry {
    pub id: String,
    pub description: String,
    pub icon: ViewIcon,
    pub color: String,
    pub category: String,
    pub contextual: Option<bool>,
    pub applicable_types: Option<Vec<String>>,
    pub modes: Vec<String>,
    pub params: Option<Vec<String>>,
    pub cypher: String,
}
```

**Phase 4: Update Components**

- Studio: Use `<Icon name={view.icon.web} />` from `lucide-react`
- TUI: Use `view.icon.terminal` for ratatui rendering

### 4. Validation Schema (Zod)

```typescript
// packages/core/src/schema/views.ts
import { z } from 'zod';

// Lucide icon names (partial list - validate against lucide-react exports)
const LucideIconName = z.string().regex(/^[a-z][a-z0-9-]*$/);

// Unicode terminal icon (single character or small string)
const TerminalIcon = z.string().min(1).max(4);

export const ViewIconSchema = z.object({
  web: LucideIconName,
  terminal: TerminalIcon,
});

export const ViewRegistryEntrySchema = z.object({
  id: z.string().regex(/^[a-z][a-z0-9-]*$/),
  description: z.string().min(1).max(200),
  icon: ViewIconSchema,
  color: z.string().regex(/^#[0-9a-fA-F]{6}$/),
  category: z.enum(['meta', 'data', 'overlay', 'contextual']),
  contextual: z.boolean().optional(),
  applicable_types: z.array(z.string()).optional(),
  modes: z.array(z.enum(['meta', 'data', 'overlay', 'query'])),
  params: z.array(z.string()).optional(),
  cypher: z.string().min(1),
});

export const ViewRegistrySchema = z.object({
  version: z.string(),
  description: z.string(),
  views: z.array(ViewRegistryEntrySchema),
});
```

**JSON Schema (alternative):**
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "definitions": {
    "ViewIcon": {
      "type": "object",
      "properties": {
        "web": { "type": "string", "pattern": "^[a-z][a-z0-9-]*$" },
        "terminal": { "type": "string", "minLength": 1, "maxLength": 4 }
      },
      "required": ["web", "terminal"]
    },
    "ViewEntry": {
      "type": "object",
      "properties": {
        "id": { "type": "string", "pattern": "^[a-z][a-z0-9-]*$" },
        "description": { "type": "string" },
        "icon": { "$ref": "#/definitions/ViewIcon" },
        "color": { "type": "string", "pattern": "^#[0-9a-fA-F]{6}$" },
        "category": { "enum": ["meta", "data", "overlay", "contextual"] },
        "contextual": { "type": "boolean" },
        "applicable_types": { "type": "array", "items": { "type": "string" } },
        "modes": { "type": "array", "items": { "enum": ["meta", "data", "overlay", "query"] } },
        "params": { "type": "array", "items": { "type": "string" } },
        "cypher": { "type": "string" }
      },
      "required": ["id", "description", "icon", "color", "category", "modes", "cypher"]
    }
  }
}
```

### 5. Complete View Icon Mapping (29 Views)

| # | View ID | Category | Web (Lucide) | Terminal | Color |
|---|---------|----------|--------------|----------|-------|
| **META (3)** |
| 1 | `meta-complete` | meta | `diamond` | `◆` | #8b5cf6 |
| 2 | `meta-realm` | meta | `building-2` | `◉` | #6366f1 |
| 3 | `meta-arcs` | meta | `git-branch` | `→` | #f59e0b |
| **DATA (10)** |
| 4 | `data-complete` | data | `globe` | `●` | #6366f1 |
| 5 | `data-org` | data | `building` | `◎` | #0ea5e9 |
| 6 | `data-shared` | data | `earth` | `◉` | #2aa198 |
| 7 | `data-geography` | data | `map` | `▧` | #14b8a6 |
| 8 | `data-project` | data | `folder-kanban` | `▤` | #6366f1 |
| 9 | `data-entities` | data | `link` | `◈` | #06b6d4 |
| 10 | `data-locale-coverage` | data | `languages` | `◌` | #f59e0b |
| 11 | `data-seo` | data | `target` | `◎` | #8b5cf6 |
| 12 | `data-geo` | data | `radar` | `◇` | #a855f7 |
| 13 | `data-generation` | data | `zap` | `★` | #ec4899 |
| **OVERLAY (3)** |
| 14 | `overlay-complete` | overlay | `layers` | `▣` | #f97316 |
| 15 | `overlay-kind-instances` | overlay | `square-stack` | `▤` | #3b82f6 |
| 16 | `overlay-arc-analysis` | overlay | `git-compare` | `⇄` | #f59e0b |
| **CONTEXTUAL (13)** |
| 17 | `ctx-composition` | contextual | `box` | `□` | #0ea5e9 |
| 18 | `ctx-entities` | contextual | `link-2` | `◈` | #06b6d4 |
| 19 | `ctx-locales` | contextual | `globe-2` | `◌` | #f59e0b |
| 20 | `ctx-knowledge` | contextual | `brain` | `◇` | #22c55e |
| 21 | `ctx-geographic` | contextual | `map-pin` | `▧` | #14b8a6 |
| 22 | `ctx-generation` | contextual | `sparkles` | `★` | #ec4899 |
| 23 | `ctx-categories` | contextual | `tag` | `#` | #64748b |
| 24 | `ctx-seo-intel` | contextual | `search` | `◎` | #8b5cf6 |
| 25 | `ctx-geo-intel` | contextual | `radar` | `◇` | #a855f7 |
| 26 | `ctx-project` | contextual | `folder-open` | `▤` | #6366f1 |
| 27 | `ctx-arc-relationships` | contextual | `git-fork` | `⇢` | #f59e0b |
| 28 | `ctx-full-context` | contextual | `maximize-2` | `⊕` | #6366f1 |
| 29 | `ctx-neighborhood` | contextual | `scan` | `◎` | #22c55e |

### 6. Icon Selection Guidelines

**Lucide Icon Naming Conventions:**
- Use semantic names that match the view's purpose
- Prefer simpler icons over complex ones
- Check availability at https://lucide.dev/icons

**Terminal Icon Guidelines:**
- Use single-width Unicode for monospace alignment
- Prefer geometric shapes from the Unicode Block Elements range
- Ensure visibility in both light and dark terminal themes

**Recommended Unicode Ranges:**
- Geometric Shapes: U+25A0–U+25FF (`■ □ ▪ ▫ ▣ ▤ ▥ ▦ ▧`)
- Miscellaneous Symbols: U+2600–U+26FF (`◆ ◇ ◈ ◉ ◎ ● ○`)
- Arrows: U+2190–U+21FF (`→ ← ↑ ↓ ⇢ ⇠ ⇒`)
- Mathematical Operators: U+2200–U+22FF (`⊕ ⊗ ⊞`)

### 7. Files to Update (Summary)

| File | Change |
|------|--------|
| `packages/core/models/views/_registry.yaml` | Replace all emoji icons with dual format |
| `packages/core/src/types/views.ts` | Add `ViewIcon` interface |
| `packages/core/src/schema/views.ts` | Add Zod validation schema |
| `apps/studio/src/components/views/ViewPicker.tsx` | Use `view.icon.web` with Lucide |
| `tools/novanet/src/tui/views.rs` | Parse dual icon format |
| `tools/novanet/src/tui/ui.rs` | Use `view.icon.terminal` for rendering |

### 8. Backward Compatibility

During migration, the parser should support both formats:

```typescript
// Temporary backward compatibility layer
function parseViewIcon(icon: string | ViewIcon): ViewIcon {
  if (typeof icon === 'string') {
    // Legacy emoji format - map to dual icons
    return EMOJI_TO_DUAL_ICON[icon] ?? { web: 'help-circle', terminal: '?' };
  }
  return icon;
}

const EMOJI_TO_DUAL_ICON: Record<string, ViewIcon> = {
  '🔷': { web: 'diamond', terminal: '◆' },
  '🏛️': { web: 'building-2', terminal: '◉' },
  '🔀': { web: 'git-branch', terminal: '→' },
  '🌐': { web: 'globe', terminal: '●' },
  '🏢': { web: 'building', terminal: '◎' },
  '🌍': { web: 'earth', terminal: '◉' },
  '🗺️': { web: 'map', terminal: '▧' },
  '🏗️': { web: 'folder-kanban', terminal: '▤' },
  '🔗': { web: 'link', terminal: '◈' },
  '🎯': { web: 'target', terminal: '◎' },
  '🔮': { web: 'radar', terminal: '◇' },
  '⚡': { web: 'zap', terminal: '★' },
  '🔶': { web: 'layers', terminal: '▣' },
  '📦': { web: 'box', terminal: '□' },
  '🧠': { web: 'brain', terminal: '◇' },
  '🏷️': { web: 'tag', terminal: '#' },
  '🔍': { web: 'scan', terminal: '◎' },
};
```

This compatibility layer can be removed once all YAML files are migrated.

---

## Revised Execution Plan (Post-Review)

> **Source**: Code architect review + Context7 (Ratatui, neo4rs) + Perplexity research
> **Date**: 2026-02-11

### Critical Issues Fixed

| Issue | Severity | Fix |
|-------|----------|-----|
| Neo4j migration missing | **CRITICAL** | Added Phase 0: Neo4j schema migration |
| Phase ordering wrong | **HIGH** | Types (Phase 2) now before Generators (Phase 3) |
| Performance risk (200K instances) | **HIGH** | Added index + approximate count strategy |
| Backward compat missing | **MEDIUM** | Added nav mode migration shim |
| Test fixtures undefined | **MEDIUM** | Added fixtures directory structure |

### Corrected Phase Execution Order

```
Phase 0: Neo4j Schema Migration (BLOCKING)
    ↓
Phase 1: YAML Updates (visual-encoding, views, taxonomy)
    ↓
Phase 2: TypeScript/Rust Type Definitions (BEFORE generators!)
    ↓
Phase 3: Generators (consume types, produce artifacts)
    ↓
Phase 4A: TUI (Rust)  ──┐
                        ├── PARALLEL
Phase 4B: Studio (TS) ──┘
    ↓
Phase 5: Testing (unit + integration + E2E)
    ↓
Phase 6: Documentation + CHANGELOG
```

### Phase 0: Neo4j Schema Migration (NEW - BLOCKING)

**Why**: Without these relationships, unified tree queries will fail.

```cypher
// File: packages/db/seed/migrations/v11.7-unified-tree.cypher

// 1. Create HAS_LAYER relationships (Realm → Layer)
MATCH (r:Realm:Meta), (l:Layer:Meta)
WHERE l.realm = r.key
MERGE (r)-[:HAS_LAYER]->(l);

// 2. Create HAS_KIND relationships (Layer → Kind)
MATCH (l:Layer:Meta), (k:Kind:Meta)
WHERE k.layer = l.key AND k.realm = l.realm
MERGE (l)-[:HAS_KIND]->(k);

// 3. Create BELONGS_TO_FAMILY relationships (ArcKind → ArcFamily)
MATCH (af:ArcFamily:Meta), (ak:ArcKind:Meta)
WHERE ak.family = af.key
MERGE (ak)-[:BELONGS_TO_FAMILY]->(af);

// 4. Add index for instance loading performance
CREATE INDEX kind_instances_idx IF NOT EXISTS
FOR (n)-[r:OF_KIND]->()
ON (r);

// 5. Add constraint for unique node keys
CREATE CONSTRAINT node_key_unique IF NOT EXISTS
FOR (n:Node) REQUIRE n.key IS UNIQUE;
```

**Validation**:
```cypher
// Verify migration success
MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
RETURN r.key AS realm, count(DISTINCT l) AS layers, count(DISTINCT k) AS kinds;
// Expected: shared=4 layers/39 kinds, org=6 layers/21 kinds
```

### Phase 1: YAML Updates

**Files**:
- `packages/core/models/visual-encoding.yaml` - Add meta_types icons
- `packages/core/models/views/_registry.yaml` - Replace emoji → dual icons
- `packages/core/models/taxonomy.yaml` - Add `node_sections` config

**Validation**: `cargo run -- schema validate --strict`

### Phase 2: Type Definitions (BEFORE Generators!)

**Rust** (`tools/novanet/src/tui/`):
```rust
// types.rs - NEW FILE
pub enum NodeId { Section, Realm, Layer, Kind, Instance, ArcFamily, ArcKind }
pub struct UnifiedNode { id, depth, display, children, neo4j_labels }
pub enum LazyChildren { NotLoaded, Loading, Loaded { items, total, has_more }, Leaf }
pub struct Badge { icon, abbrev, color_key }
```

**TypeScript** (`packages/core/src/types/`):
```typescript
// unified-tree.ts - NEW FILE
export interface ViewIcon { web: string; terminal: string; }
export interface UnifiedTreeNode { id: string; type: NodeType; /* ... */ }
export type NodeType = 'realm' | 'layer' | 'kind' | 'instance' | 'arcFamily' | 'arcKind';
```

### Phase 3: Generators

**Only after Phase 2 types exist!**

- `icon_generator.rs` - Generate `tui/icons.rs` from `visual-encoding.yaml`
- `view_generator.rs` - Update to handle dual icon format
- Validate: `cargo run -- schema generate --dry-run`

### Phase 4A: TUI Implementation (Rust)

**Async Architecture** (from Ratatui best practices):

```rust
// src/tui/mod.rs - Channel-based async pattern
use tokio::sync::mpsc;

pub enum AsyncCommand {
    LoadInstances { kind: String, offset: usize, limit: usize },
    LoadRealmDetails(String),
    LoadLayerDetails { realm: String, layer: String },
    Shutdown,
}

pub enum AsyncEvent {
    InstancesLoaded(InstanceLoadResponse),
    RealmDetailsLoaded(RealmDetails),
    Error(String),
}

// Main loop with tokio::select!
async fn run(db: Db) -> Result<()> {
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<AsyncCommand>();
    let (evt_tx, mut evt_rx) = mpsc::unbounded_channel::<AsyncEvent>();

    // Spawn async worker for Neo4j queries
    let worker_db = db.clone();  // Arc<Graph> - cheap clone
    tokio::spawn(async move {
        while let Some(cmd) = cmd_rx.recv().await {
            let result = match cmd {
                AsyncCommand::LoadInstances { kind, offset, limit } => {
                    match load_instances(&worker_db, &kind, offset, limit).await {
                        Ok(resp) => evt_tx.send(AsyncEvent::InstancesLoaded(resp)),
                        Err(e) => evt_tx.send(AsyncEvent::Error(e.to_string())),
                    }
                }
                AsyncCommand::Shutdown => break,
                // ... other commands
            };
        }
    });

    // TUI event loop
    loop {
        tokio::select! {
            event = tui.next() => { /* handle key/mouse/tick */ }
            Some(async_evt) = evt_rx.recv() => { /* handle async results */ }
        }
    }
}
```

**Instance Loading with Approximate Count** (performance fix):

```rust
async fn load_instances(
    db: &Db,
    kind: &str,
    offset: usize,
    limit: usize,
) -> Result<InstanceLoadResponse> {
    // Use APOC for approximate count on large datasets
    let count_cypher = r#"
        CALL apoc.meta.stats() YIELD labels
        WITH labels[$kind] AS count
        RETURN CASE WHEN count > 10000 THEN count ELSE null END AS approx
    "#;

    let exact_cypher = format!(r#"
        MATCH (n:{kind})-[:OF_KIND]->(k:Kind:Meta {{label: $kind}})
        WITH count(n) AS total
        MATCH (n:{kind})-[:OF_KIND]->(k:Kind:Meta {{label: $kind}})
        WITH total, n ORDER BY n.key SKIP $offset LIMIT $limit
        RETURN total, collect(n) AS instances
    "#, kind = kind);

    // ... execute and return
}
```

### Phase 4B: Studio Implementation (TypeScript)

**Zustand Store** (from Perplexity research):

```typescript
// apps/studio/src/stores/treeStore.ts
import { create } from 'zustand';

interface TreeState {
  expanded: Set<string>;
  loadedChildren: Map<string, string[]>;
  loadingNodes: Set<string>;

  toggleExpand: (nodeId: string) => void;
  loadChildren: (nodeId: string) => Promise<void>;
}

export const useTreeStore = create<TreeState>((set, get) => ({
  expanded: new Set(),
  loadedChildren: new Map(),
  loadingNodes: new Set(),

  toggleExpand: (nodeId) => {
    const { expanded, loadedChildren, loadChildren } = get();
    const newExpanded = new Set(expanded);

    if (newExpanded.has(nodeId)) {
      newExpanded.delete(nodeId);
    } else {
      newExpanded.add(nodeId);
      // Lazy load if not already loaded
      if (!loadedChildren.has(nodeId)) {
        loadChildren(nodeId);
      }
    }
    set({ expanded: newExpanded });
  },

  loadChildren: async (nodeId) => {
    set((s) => ({ loadingNodes: new Set(s.loadingNodes).add(nodeId) }));

    try {
      const children = await fetchChildren(nodeId);
      set((s) => ({
        loadedChildren: new Map(s.loadedChildren).set(nodeId, children),
        loadingNodes: new Set([...s.loadingNodes].filter(id => id !== nodeId)),
      }));
    } catch (error) {
      set((s) => ({
        loadingNodes: new Set([...s.loadingNodes].filter(id => id !== nodeId)),
      }));
    }
  },
}));
```

**Tree Node with Suspense**:

```tsx
// apps/studio/src/components/graph/UnifiedTreeNode.tsx
import { Suspense, lazy, startTransition } from 'react';
import { useTreeStore } from '@/stores/treeStore';
import * as Icons from 'lucide-react';

const TreeChildren = lazy(() => import('./TreeChildren'));

export function UnifiedTreeNode({ node }: { node: UnifiedNode }) {
  const { expanded, loadingNodes, toggleExpand } = useTreeStore();
  const isExpanded = expanded.has(node.id);
  const isLoading = loadingNodes.has(node.id);

  const handleClick = () => {
    startTransition(() => {
      toggleExpand(node.id);
    });
  };

  const IconComponent = Icons[node.icon.web as keyof typeof Icons];

  return (
    <div className="tree-node">
      <button onClick={handleClick}>
        {isLoading ? <Icons.Loader2 className="animate-spin" /> : <IconComponent />}
        <span>{node.label}</span>
        {node.count && <span className="count">({node.count})</span>}
      </button>

      {isExpanded && (
        <Suspense fallback={<div className="loading">Loading...</div>}>
          <TreeChildren parentId={node.id} />
        </Suspense>
      )}
    </div>
  );
}
```

### Phase 5: Testing

**Test Fixtures Directory** (new):

```
tools/novanet/tests/fixtures/
├── minimal-schema.yaml       # 5 nodes, 3 arcs (fast unit tests)
├── realistic-schema.yaml     # 60 nodes, 114 arcs (integration)
├── stress-test-schema.yaml   # 1000 nodes, 5000 arcs (performance)
└── README.md                 # How to maintain fixtures
```

**Performance Benchmark** (new):

```rust
// tools/novanet/benches/instance_loading.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_load_10k_instances(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let db = rt.block_on(setup_test_db_with_10k_locales());

    c.bench_function("load_10k_instances_first_page", |b| {
        b.iter(|| {
            rt.block_on(load_instances(&db, "Locale", 0, 10))
        })
    });
}

criterion_group!(benches, bench_load_10k_instances);
criterion_main!(benches);
```

### Backward Compatibility Shim

**TUI** (`app.rs`):

```rust
// Migrate old config on startup
fn migrate_legacy_config(config: &mut Config) {
    if let Some(old_mode) = config.get("nav_mode") {
        let new_mode = match old_mode.as_str() {
            "Meta" | "Data" | "Overlay" | "Query" => NavMode::Graph,
            "Atlas" | "Nexus" => NavMode::Nexus,
            _ => NavMode::Graph,
        };
        config.set("nav_mode_v11_7", new_mode);
        config.remove("nav_mode");
        config.save()?;
    }
}
```

**Studio** (`localStorage`):

```typescript
// apps/studio/src/lib/migrations.ts
export function migrateLocalStorage() {
  const oldMode = localStorage.getItem('navMode');
  if (oldMode && ['Meta', 'Data', 'Overlay', 'Query', 'Atlas'].includes(oldMode)) {
    const newMode = oldMode === 'Atlas' ? 'Nexus' : 'Graph';
    localStorage.setItem('navMode', newMode);
    localStorage.setItem('navMode_migrated_from', oldMode);
  }
}
```

### Realistic Timeline

```
Week 1 (Person A):
  Day 1: Phase 0 (Neo4j migration) + Phase 1 (YAML)
  Day 2: Phase 2 (Types - Rust + TypeScript)
  Day 3: Phase 3 (Generators)
  Day 4-5: Phase 4A start (TUI - data structures)

Week 1 (Person B, starting Day 3):
  Day 3-5: Phase 4B (Studio - all)

Week 2:
  Day 1-2: Phase 4A finish (TUI - UI + async)
  Day 3: Integration testing (TUI ↔ Neo4j)
  Day 4: Phase 5 (Testing - fix bugs)
  Day 5: Phase 6 (Documentation)

Total: 10 working days (2 weeks) with 2 people
       15 working days (3 weeks) with 1 person
```

### Skills/Agents Per Phase

| Phase | Skills to Use | Agents to Use |
|-------|---------------|---------------|
| 0 | - | `neo4j-architect` |
| 1 | `spn-writing:mermaid` for diagrams | - |
| 2 | `rust-core` (types), `spn-rust:rust-architect` | `rust-architect` |
| 3 | `rust-core` | `rust-pro` |
| 4A | `rust-async` (tokio), `test-driven-development` | `rust-async-expert` |
| 4B | - | `feature-dev:code-architect` |
| 5 | `testing-anti-patterns`, `systematic-debugging` | `feature-dev:code-reviewer` |
| 6 | `spn-writing:writing` | - |

### Pre-Implementation Checklist

Before starting Phase 0:

- [ ] Neo4j is running (`pnpm infra:up`)
- [ ] Current schema is seeded (`pnpm infra:seed`)
- [ ] All tests pass (`cargo nextest run && pnpm test`)
- [ ] No uncommitted changes (`git status` clean)
- [ ] Create git worktree for isolation (`/spn-powers:using-git-worktrees`)

### Risk Mitigation Summary

| Risk | Mitigation |
|------|------------|
| Generator fails without types | Phase 2 (Types) now before Phase 3 (Generators) |
| 200K instance query timeout | Index + APOC approximate count |
| Users lose nav mode preference | Backward compat shim on startup |
| Icon YAML invalid | Fallback + warning message |
| Test fixtures drift from schema | Automated sync test in CI |
