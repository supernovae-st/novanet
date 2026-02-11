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
