# Context Views Action Cards Design (v11.6)

**Date**: 2026-02-10
**Status**: Approved (Brainstorm Complete)
**Author**: Brainstorm session with Claude

## Summary

Design for Context Views displayed as Action Cards in the TabbedDetailPanel footer, with ASCII preview, Cypher queries, and Matrix transition effects.

## Decisions

| Question | Choice |
|----------|--------|
| Approach | **Hybrid**: Action Cards in footer + "More Views" opens Select View modal |
| Layout | **Horizontal Scroll** footer with cards |
| ASCII Style | **Mixed** (auto-select per view type) + detailed stats |
| Stats Display | **Full Breakdown** (arc labels + node keys + summary footer) |
| Click Behavior | **Replace Graph** with Matrix transition + Cypher query + Effects |

---

## Architecture

### Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│ [Icon] Entity: qr-generator                               [X]      │ HEADER
├─────────────────────────────────────────────────────────────────────┤
│  [Overview]  [Data]  [Graph]  [Code]                               │ TABS
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│                                                                     │
│                    TAB CONTENT                                      │ BODY
│                    (scrollable)                                     │
│                                                                     │
│                                                                     │
├─────────────────────────────────────────────────────────────────────┤
│  Context Views                                      [More Views ▸] │ FOOTER
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐  ─────────────▸   │
│  │ Card 1  │ │ Card 2  │ │ Card 3  │ │ Card 4  │   scroll horiz    │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘                   │
└─────────────────────────────────────────────────────────────────────┘
```

### Components

| Component | Description |
|-----------|-------------|
| `ContextViewFooter.tsx` | Container footer with horizontal scroll |
| `ActionCard.tsx` | Individual card with ASCII preview |
| `AsciiPreview.tsx` | Render ASCII by style (Tree/Flow/Compact) |
| `useContextViews.ts` | Hook: node → available views |
| `MatrixTransition.tsx` | Matrix rain transition effect |

---

## Action Card Design

### Full Breakdown Style

Each card shows:
- **Header**: Icon + View name
- **Stats**: Node count + Arc count
- **ASCII Preview**: Style varies by view type
- **Arc Labels**: Visible relationship types
- **Node Keys**: Actual instance names
- **Summary Footer**: Counts by NodeKind + ArcKind

### Examples by Node Type

#### Page Node

```
┌─────────────────────────┐  ┌─────────────────────────┐  ┌─────────────────────────┐
│ 📦 Composition          │  │ 🔗 Entity Links         │  │ 🌍 Locale Coverage      │
│ 8 nodes · 7 arcs · 3lvl │  │ 5 nodes · 8 arcs        │  │ 12 locales · 36 content │
│                         │  │                         │  │                         │
│  homepage               │  │  homepage               │  │  homepage               │
│   ├─HAS_BLOCK           │  │   ══╦══► Entity (2)     │  │   ├─fr: ██████████ 100% │
│   │  └─hero (1)         │  │     ╠══► SEOKey (4)     │  │   ├─en: ████████░░  80% │
│   │  └─features (4)     │  │     ╚══► Category (1)   │  │   ├─de: ██████░░░░  60% │
│   └─cta (2)             │  │                         │  │   └─ja: ████░░░░░░  40% │
│                         │  │                         │  │                         │
│ 1 Page · 7 Block        │  │ USES:2 TARGETS:4 BELONGS:1│ │ 3/12 complete           │
└─────────────────────────┘  └─────────────────────────┘  └─────────────────────────┘
```

#### Entity Node

```
┌─────────────────────────┐  ┌─────────────────────────┐  ┌─────────────────────────┐
│ 🎯 SEO Intelligence     │  │ 📝 Content Locales      │  │ 🏷️ Category Tree        │
│ 23 nodes · 31 arcs      │  │ 8 locales · 8 content   │  │ 4 nodes · 3 arcs        │
│                         │  │                         │  │                         │
│  qr-generator           │  │  qr-generator           │  │  thing                  │
│   ──┬──► Keyword (12)   │  │   └─HAS_CONTENT         │  │   └─BELONGS_TO          │
│     │    └─► Cluster(3) │  │      ├─fr-FR ✓          │  │      └─qr-generator     │
│     └──► GEOQuery (5)   │  │      ├─en-US ✓          │  │         ├─qr-code       │
│          └─► Answer(3)  │  │      ├─de-DE ○          │  │         └─generator     │
│                         │  │      └─ja-JP ○          │  │                         │
│ TARGETS:12 MONITORS:5   │  │ 2/8 generated           │  │ 1 Category · 3 Entity   │
└─────────────────────────┘  └─────────────────────────┘  └─────────────────────────┘
```

#### Locale Node

```
┌─────────────────────────┐  ┌─────────────────────────┐  ┌─────────────────────────┐
│ 🧠 Knowledge Atoms      │  │ 🗣️ Voice Config         │  │ 🗺️ Geographic Reach     │
│ 127 nodes · 126 arcs    │  │ 5 nodes · 4 arcs        │  │ 6 nodes · 5 arcs        │
│                         │  │                         │  │                         │
│  fr-FR                  │  │  fr-FR                  │  │  fr-FR                  │
│   ├─HAS_TERMS           │  │   └─HAS_VOICE           │  │   └─FOR_COUNTRY         │
│   │  └─TermSet (3)      │  │      └─LocaleVoice      │  │      └─France           │
│   │     └─Term (45)     │  │         ├─tone: formal  │  │         ├─IN_REGION     │
│   ├─HAS_EXPRESSIONS     │  │         ├─register: std │  │         │  └─Europe     │
│   │  └─ExprSet (2)      │  │         └─humor: subtle │  │         └─IN_CONTINENT  │
│   │     └─Expr (38)     │  │                         │  │            └─EU         │
│   └─HAS_PATTERNS        │  │                         │  │                         │
│      └─PatternSet (1)   │  │ 1 Voice configured      │  │ 1 Country · 1 Region    │
│         └─Pattern (12)  │  │                         │  │                         │
└─────────────────────────┘  └─────────────────────────┘  └─────────────────────────┘
```

---

## ASCII Preview Styles

| Style | Use Case | Characteristics |
|-------|----------|-----------------|
| **Tree** | Hierarchies (Composition, Knowledge, Project) | `├─` `└─` `│` indentation |
| **Flow** | Relations (Entity Links, SEO, Generation) | `══►` `──►` `╦` `╠` arrows |
| **Compact** | Distributions (Locales, Geographic, Metrics) | `████` progress bars, `✓` `○` status |

---

## Click Flow

```
User clicks Card
     │
     ▼
┌─────────────────┐
│ 1. Start Effect │ ← Border Beam on clicked card
└────────┬────────┘
         ▼
┌─────────────────┐
│ 2. Build Cypher │ ← getCypherForView(nodeId, viewType)
└────────┬────────┘
         ▼
┌─────────────────┐
│ 3. Execute Neo4j│ ← Fetch nodes + arcs
└────────┬────────┘
         ▼
┌─────────────────┐
│ 4. Matrix Trans │ ← 3-phase transition (1000ms total)
└────────┬────────┘
         ▼
┌─────────────────┐
│ 5. Render View  │ ← Apply view-specific effect
└─────────────────┘
```

---

## Matrix Transition Effect

### Phase 1: Dissolve (300ms)

Current graph pixelates into Matrix characters.
- Color: source node layer color → fade to black
- Characters: `░▒▓█` cascading down

### Phase 2: Rain (400ms)

Matrix rain with view-type specific color:

| View Type | Color | Hex |
|-----------|-------|-----|
| Composition | Structure Blue | `#0ea5e9` |
| Knowledge | Knowledge Green | `#22c55e` |
| Locales | Locale Amber | `#f59e0b` |
| Geographic | Geo Teal | `#14b8a6` |
| SEO/GEO | Mining Purple | `#8b5cf6` |
| Generation | Gen Pink | `#ec4899` |
| Entities | Semantic Cyan | `#06b6d4` |
| Project | Foundation Indigo | `#6366f1` |

### Phase 3: Materialize (300ms)

New nodes appear from characters.
- Scale: 0 → 1 with spring bounce
- Arcs: draw progressively (stroke-dashoffset animation)

---

## View Types (14 Total)

| View | Node Types | Style | Effect | Cypher Pattern |
|------|------------|-------|--------|----------------|
| 📦 Composition | Page, Block | Tree | Particles | `(p:Page)-[:HAS_BLOCK*1..3]->(b:Block)` |
| 🔗 Entities | Page, Entity | Flow | Border Beam | `(e)-[r:USES_ENTITY\|TARGETS]->(t)` |
| 🧠 Knowledge | Locale | Tree | Matrix Rain | `(l:Locale)-[:HAS_*]->(set)-[:CONTAINS_*]->(atom)` |
| 🌍 Locales | Page, Entity | Compact | Orbiting Circles | `(n)-[:HAS_CONTENT\|HAS_GENERATED]->(c)` |
| 🗺️ Geographic | Locale | Tree | Globe Pulse | `(l)-[:FOR_COUNTRY]->(c)-[:IN_REGION]->(r)` |
| 🎯 SEO Intel | Entity, Page | Flow | Ripple | `(e)-[:TARGETS]->(k:SEOKeyword)-[:IN_CLUSTER]->(c)` |
| 🔮 GEO Intel | Entity | Flow | Radar Sweep | `(e)-[:MONITORS_GEO]->(q:GEOQuery)-[:HAS_ANSWER]->(a)` |
| ⚡ Generation | Page, Block | Flow | Meteor | `(p)-[:HAS_GENERATED]->(g:PageGenerated)` |
| 🏗️ Project | Project | Tree | Shimmer | `(p:Project)-[:HAS_PAGE\|HAS_ENTITY]->(n)` |
| 🏷️ Categories | Entity | Flow | Pulse | `(e:Entity)-[:BELONGS_TO]->(c:EntityCategory)` |
| 📝 Content | Entity, Project | Compact | Typewriter | `(e)-[:HAS_CONTENT]->(c:EntityContent)` |
| 🎨 Brand | Project | Tree | Color Wave | `(p:Project)-[:HAS_BRAND]->(b:BrandIdentity)` |
| 📊 Metrics | SEO, GEO | Compact | Chart Rise | `(k)-[:HAS_METRICS]->(m:*Metrics)` |
| 🔄 Cross-Realm | Any | Flow | Portal | `(org)-[r]->(shared) WHERE r.scope = 'cross_realm'` |

---

## Views by Node Kind

### Page
- 📦 Composition (blocks hierarchy)
- 🔗 Entities (used entities)
- 🌍 Locales (PageGenerated per locale)
- ⚡ Generation (generation pipeline)

### Entity
- 🎯 SEO Intel (keywords + clusters)
- 🔮 GEO Intel (queries + answers)
- 🏷️ Categories (EntityCategory)
- 📝 Content (EntityContent per locale)
- 🔗 Pages (pages using this entity)

### Locale
- 🧠 Knowledge (all knowledge atoms)
- 🗺️ Geographic (country + region)
- 🗣️ Voice (LocaleVoice config)
- 🌍 Coverage (content stats)

### Project
- 🏗️ Overview (pages + entities)
- 🎨 Brand (brand identity)
- 📝 Content (ProjectContent)
- 📊 Metrics (aggregated stats)

### Block
- 📦 Parent (page hierarchy)
- 🔗 Entities (used entities)
- ⚡ Generation (BlockGenerated)

### SEOKeyword / GEOQuery
- 📊 Metrics (performance data)
- 🔗 Entities (targeting entities)
- 🎯 Cluster (related keywords)

---

## Implementation

### Files to Create

| File | Description |
|------|-------------|
| `components/sidebar/footer/ContextViewFooter.tsx` | Footer container with horizontal scroll |
| `components/sidebar/footer/ActionCard.tsx` | Individual action card component |
| `components/sidebar/footer/AsciiPreview.tsx` | ASCII renderer (Tree/Flow/Compact) |
| `components/sidebar/footer/MatrixTransition.tsx` | Matrix rain transition effect |
| `hooks/useContextViews.ts` | Node → available views mapping |
| `lib/cypher/viewQueries.ts` | Cypher query builders per view |
| `config/viewTypes.ts` | View type definitions + effects mapping |

### Files to Modify

| File | Changes |
|------|---------|
| `TabbedDetailPanel.tsx` | Add ContextViewFooter below tabs |
| `graphStore.ts` | Add `loadView(nodeId, viewType)` action |
| `uiStore.ts` | Add `activeView`, `viewTransitionState` |

### Estimation

| Component | Lines |
|-----------|-------|
| ContextViewFooter | ~80 |
| ActionCard | ~120 |
| AsciiPreview | ~200 |
| MatrixTransition | ~150 |
| useContextViews | ~100 |
| viewQueries | ~200 |
| viewTypes config | ~150 |
| **Total** | **~1000 lines** |

---

## Future Enhancements

- Drag & drop to reorder favorite views
- Pin views to quick access bar
- View history with back/forward navigation
- Compare mode (split view with 2 context views)
- Export view as image/SVG
- Share view link with encoded state
