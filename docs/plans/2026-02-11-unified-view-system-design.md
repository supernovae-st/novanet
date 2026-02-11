# Unified View System Design (v11.6)

**Date**: 2026-02-11
**Status**: Draft
**Author**: Thibaut + Claude

## Scope

This design applies to **BOTH**:
- **Studio** (web) — ViewPicker modal with visual graph
- **TUI** (terminal) — "Atlas" mode renamed to "View" mode with ASCII rendering

## Problem

NovaNet Studio has **two separate systems** for views:

1. **viewTypes.ts** (TypeScript) - 14 hardcoded context views with `applicableTo: NodeType[]`
2. **_registry.yaml** (YAML) - 26 views with `applicable_types` and Cypher queries

This causes:
- Duplicate configuration
- Different behavior between sidebar context views and ViewPicker modal
- Maintenance burden (two places to update)

## Solution

**Unify everything into `_registry.yaml`** as the single source of truth.

## Design Decisions

### 1. Single Source of Truth

All views defined in `packages/core/models/views/_registry.yaml`:

```yaml
views:
  - id: composition
    description: Page/Block composition hierarchy
    category: contextual
    applicable_types: [Page, Block]  # For filtering
    icon: "📦"                        # NEW: emoji icon
    color: "#0ea5e9"                  # NEW: hex color
    cypher: |
      MATCH (root {key: $nodeKey})
      ...
```

### 2. Files to Delete

- `apps/studio/src/config/viewTypes.ts` - hardcoded view config
- `apps/studio/src/hooks/useContextViews.ts` - duplicate logic
- `apps/studio/src/components/sidebar/footer/ContextViewFooter.tsx` - cards UI
- `apps/studio/src/components/sidebar/footer/ActionCard.tsx` - card component
- `apps/studio/src/components/sidebar/footer/AsciiPreview.tsx` - ASCII preview

### 3. ViewPicker Behavior

**Without node selected:**
- Show ALL views grouped by category
- Click view → execute Cypher → display new graph

**With node selected:**
- Show ONLY views where `applicable_types` includes node type
- Title: "Vues pour {node.displayName} ({node.type})"
- Bottom link: "Voir toutes les vues"
  - Click → deselect node → reset view → show full ViewPicker

### 4. View Execution Flow

```
User clicks view in ViewPicker
    ↓
viewStore.executeView(viewId, { key: nodeKey })
    ↓
Close sidebar (if open)
Reset node selection
    ↓
Fetch /api/views/{id}?key={nodeKey}
    ↓
Execute Cypher query
    ↓
Animation transition
    ↓
Display new graph
```

### 5. YAML Schema Addition

Add to each view in `_registry.yaml`:

```yaml
# Visual properties (optional, defaults provided)
icon:
  web: "package"     # Lucide icon name (Studio)
  terminal: "◆"      # Unicode symbol (TUI)
color: "#0ea5e9"     # Hex color for visual identity
```

**NO EMOJI** - Use dual icons:
- `web`: Lucide icon name for Studio (React)
- `terminal`: Unicode symbol for TUI (Rust/ratatui)

This follows the same pattern as `visual-encoding.yaml`.

## Implementation Tasks

### Phase 1: YAML Enhancement
- [ ] Add `icon` and `color` to all views in `_registry.yaml`
- [ ] Update TypeScript types for `ViewRegistryEntry`

### Phase 2: Delete Duplicate Code
- [ ] Delete `viewTypes.ts`
- [ ] Delete `useContextViews.ts`
- [ ] Delete `ContextViewFooter.tsx`, `ActionCard.tsx`, `AsciiPreview.tsx`
- [ ] Remove imports and usages

### Phase 3: Modify ViewPicker
- [ ] Add filtering by `applicable_types` when node selected
- [ ] Add "Voir toutes les vues" link at bottom
- [ ] Handle deselect + reset flow

### Phase 4: Cleanup
- [ ] Remove unused dependencies
- [ ] Update tests
- [ ] Verify all views work correctly

## Out of Scope (for now)

- Stats display on view cards (node count, arc count)
- ASCII preview of view structure
- Effects/animations per view type

These can be added later if needed.

---

## View Modes (4 Categories)

Based on the Neo4j database structure:

```
META-GRAPH (Schema)     — :Meta label
├── :Meta:Realm (2)     → SHARED, ORG
├── :Meta:Layer (10)    → config, locale, geography, knowledge, etc.
├── :Meta:Kind (60)     → Project, Entity, Page, Block, etc.
├── :Meta:Trait (5)     → invariant, localized, knowledge, etc.
├── :Meta:ArcFamily (5) → ownership, localization, semantic, etc.
└── :Meta:ArcKind (114) → HAS_PAGE, USES_ENTITY, etc.

DATA-GRAPH (Instances)  — without :Meta label
├── :Project, :Entity, :Page, :Block, etc.
└── realm/layer are PROPERTIES on Kind nodes, not on data nodes
```

### Mode 1: META
- Explore the schema: Realm → Layer → Kind → Trait
- Understand available node types and arc types
- `MATCH (n:Meta) ...`

### Mode 2: DATA
- Explore instances: Project → Entity → Page → Block
- See actual content in the database
- `MATCH (n) WHERE NOT n:Meta ...`

### Mode 3: OVERLAY
- See Kind nodes connected to their instances
- Debug / understand complete structure
- `MATCH (n) ...` (both meta and data)

### Mode 4: CONTEXTUAL
- Subgraph centered on a selected node
- Works in DATA or META depending on selected node

---

## Icon Mapping (NO EMOJI)

All views use **dual icons** - different for Studio (web) and TUI (terminal):

| View ID | Web (Lucide) | Terminal (Unicode) | Color |
|---------|--------------|-------------------|-------|
| **META** |
| meta-complete | `diamond` | `◆` | #8b5cf6 |
| meta-realm | `building-2` | `◉` | #6366f1 |
| meta-arcs | `git-branch` | `→` | #f59e0b |
| **DATA** |
| data-complete | `globe` | `●` | #6366f1 |
| data-org | `building` | `◎` | #0ea5e9 |
| data-shared | `earth` | `◉` | #2aa198 |
| data-geography | `map` | `▧` | #14b8a6 |
| data-locale | `languages` | `◌` | #64748b |
| data-knowledge | `brain` | `◇` | #8b5cf6 |
| data-foundation | `home` | `▤` | #3b82f6 |
| data-structure | `layout` | `▦` | #06b6d4 |
| data-semantic | `sparkles` | `◈` | #f97316 |
| data-instruction | `file-text` | `▥` | #eab308 |
| data-output | `package` | `●` | #22c55e |
| **OVERLAY** |
| overlay-complete | `layers` | `▣` | #a855f7 |
| overlay-realm | `layers-2` | `▤` | #7c3aed |
| overlay-layer | `layers-3` | `▥` | #6366f1 |
| **CONTEXTUAL** |
| composition | `box` | `□` | #0ea5e9 |
| knowledge | `brain` | `◇` | #8b5cf6 |
| generation | `wand-2` | `★` | #f59e0b |
| seo-pipeline | `search` | `◎` | #10b981 |
| locale-context | `languages` | `◌` | #64748b |
| entity-relations | `network` | `◈` | #f97316 |
| page-structure | `layout` | `▦` | #06b6d4 |
| block-content | `square` | `□` | #3b82f6 |
| project-overview | `folder` | `▤` | #eab308 |
| brand-identity | `palette` | `◆` | #ec4899 |
| seo-keywords | `tag` | `#` | #10b981 |
| geo-queries | `map-pin` | `▧` | #14b8a6 |
| term-usage | `book` | `◇` | #8b5cf6 |

> **NOTE**: All `- **Icon**: 🔷` entries below are DEPRECATED.
> Use the dual icon format from the table above: `{ web: "lucide-name", terminal: "◆" }`

---

## View Definitions

### META VIEWS (3)

#### meta-complete
- **Icon**: `{ web: "diamond", terminal: "◆" }`
- **Color**: #8b5cf6 (violet)
- **Mode**: META
- **Description**: Tout le meta-graph (Realm, Layer, Kind, Trait, ArcKind)
- **applicable_types**: [] (global)
- **Cypher**:
```cypher
MATCH (n:Meta)
OPTIONAL MATCH (n)-[r]->(m:Meta)
RETURN n, r, m
```

#### meta-realm
- **Icon**: 🏛️
- **Color**: #6366f1 (indigo)
- **Mode**: META
- **Params**: realm (shared | org)
- **Description**: Kinds d'un realm spécifique avec leurs layers
- **applicable_types**: [] (global, needs realm param)
- **Cypher**:
```cypher
MATCH (r:Meta:Realm {key: $realm})-[:HAS_LAYER]->(l:Meta:Layer)
OPTIONAL MATCH (l)-[:HAS_KIND]->(k:Meta:Kind)
OPTIONAL MATCH (k)-[:HAS_TRAIT]->(t:Meta:Trait)
RETURN r, l, k, t
```

#### meta-arcs
- **Icon**: 🔀
- **Color**: #f59e0b (amber)
- **Mode**: META
- **Description**: ArcKinds groupés par ArcFamily avec source/target
- **applicable_types**: [] (global)
- **Cypher**:
```cypher
MATCH (af:Meta:ArcFamily)<-[:BELONGS_TO_FAMILY]-(ak:Meta:ArcKind)
MATCH (ak)-[:FROM_KIND]->(source:Meta:Kind)
MATCH (ak)-[:TO_KIND]->(target:Meta:Kind)
RETURN af, ak, source, target
```

---

### DATA VIEWS (10)

#### 1. data-complete
- **Icon**: 🌐
- **Color**: #6366f1 (indigo)
- **Description**: Tout le data-graph (instances sans meta)
- **applicable_types**: [] (global)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-COMPLETE: All Instance Nodes                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Project ──┬──[HAS_PAGE]──▶ Page ──[HAS_BLOCK]──▶ Block                    │
│            │                  │                      │                      │
│            │                  ├──[HAS_GENERATED]──▶ PageGenerated          │
│            │                  │                                             │
│            ├──[HAS_ENTITY]──▶ Entity ──[HAS_CONTENT]──▶ EntityContent      │
│            │                     │                                          │
│            │                     └──[BELONGS_TO]──▶ EntityCategory          │
│            │                                                                │
│            └──[HAS_BRAND]──▶ BrandIdentity                                 │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (n) WHERE NOT n:Meta
OPTIONAL MATCH (n)-[r]->(m) WHERE NOT m:Meta
RETURN n, r, m
```

#### 2. data-org
- **Icon**: 🏢
- **Color**: #0ea5e9 (sky blue)
- **Description**: Nodes du realm ORG uniquement (business content)
- **applicable_types**: [] (global)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-ORG: Organization Realm (6 layers, 21 nodes)                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  config ─────▶ OrgConfig                                                   │
│                                                                             │
│  foundation ─▶ Project ──▶ ProjectContent ──▶ BrandIdentity                │
│                                                                             │
│  structure ──▶ Page ──▶ Block ──▶ ContentSlot                              │
│                                                                             │
│  semantic ───▶ Entity ──▶ EntityContent ──▶ AudiencePersona                │
│                                                                             │
│  instruction ▶ PageType ──▶ BlockType ──▶ PagePrompt ──▶ BlockPrompt       │
│                                                                             │
│  output ─────▶ PageGenerated ──▶ BlockGenerated ──▶ OutputArtifact         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (n) WHERE NOT n:Meta
WITH n
MATCH (k:Meta:Kind {name: labels(n)[0]})
WHERE k.realm = 'org'
OPTIONAL MATCH (n)-[r]->(m) WHERE NOT m:Meta
RETURN n, r, m
```

#### 3. data-shared
- **Icon**: 🌍
- **Color**: #2aa198 (teal)
- **Description**: Nodes du realm SHARED (universal knowledge)
- **applicable_types**: [] (global)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-SHARED: Shared Realm (4 layers, 39 nodes)                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  config ─────▶ EntityCategory ──▶ Locale ──▶ SEOKeywordFormat              │
│                                                                             │
│  locale ─────▶ Culture ──▶ Style ──▶ Formatting ──▶ Grammar                │
│                                                                             │
│  geography ──▶ Continent ──▶ GeoRegion ──▶ Country ──▶ GeoZone             │
│                                                                             │
│  knowledge ──┬▶ TermSet ──▶ Term                                           │
│              ├▶ ExpressionSet ──▶ Expression                               │
│              ├▶ PatternSet ──▶ Pattern                                     │
│              ├▶ CultureSet ──▶ CultureRef                                  │
│              ├▶ SEOKeywordSet ──▶ SEOKeyword ──▶ SEOKeywordMetrics         │
│              └▶ GEOQuerySet ──▶ GEOQuery ──▶ GEOAnswer                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (n) WHERE NOT n:Meta
WITH n
MATCH (k:Meta:Kind {name: labels(n)[0]})
WHERE k.realm = 'shared'
OPTIONAL MATCH (n)-[r]->(m) WHERE NOT m:Meta
RETURN n, r, m
```

#### 4. data-geography
- **Icon**: 🗺️
- **Color**: #14b8a6 (teal)
- **Description**: Hiérarchie géographique complète
- **applicable_types**: [] (global)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-GEOGRAPHY: Geographic Hierarchy                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Continent (7)                                                              │
│      │                                                                      │
│      └──[HAS_REGION]──▶ GeoRegion (25)                                     │
│                              │                                              │
│                              └──[HAS_COUNTRY]──▶ Country (195)             │
│                                                      │                      │
│                                                      └──[HAS_ZONE]──▶ GeoZone │
│                                                                             │
│  Example: Europe ──▶ Western Europe ──▶ France ──▶ Paris                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (c:Continent)
OPTIONAL MATCH (c)-[:HAS_REGION]->(r:GeoRegion)
OPTIONAL MATCH (r)-[:HAS_COUNTRY]->(co:Country)
OPTIONAL MATCH (co)-[:HAS_ZONE]->(z:GeoZone)
RETURN c, r, co, z
```

#### 5. data-project
- **Icon**: 🏗️
- **Color**: #6366f1 (indigo)
- **Description**: Vue projet avec toutes ses dépendances
- **applicable_types**: [Project]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-PROJECT: Project Structure                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OWNERSHIP  (◀0 ▶4)                                                         │
│    Project ─[HAS_PAGE]────▶ Page (N)                                       │
│    Project ─[HAS_ENTITY]──▶ Entity (N)                                     │
│    Project ─[HAS_BRAND]───▶ BrandIdentity (1)                              │
│    Project ─[HAS_CONTENT]─▶ ProjectContent (N)                             │
│                                                                             │
│  LOCALIZATION  (◀0 ▶1)                                                      │
│    ProjectContent ─[FOR_LOCALE]──▶ Locale                                  │
│                                                                             │
│  SEMANTIC  (◀0 ▶1)                                                          │
│    Project ─[HAS_AUDIENCE]──▶ AudiencePersona                              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (p:Project {key: $nodeKey})
OPTIONAL MATCH (p)-[:HAS_PAGE]->(page:Page)
OPTIONAL MATCH (p)-[:HAS_ENTITY]->(entity:Entity)
OPTIONAL MATCH (p)-[:HAS_BRAND]->(brand:BrandIdentity)
OPTIONAL MATCH (p)-[:HAS_CONTENT]->(content:ProjectContent)
OPTIONAL MATCH (content)-[:FOR_LOCALE]->(locale:Locale)
WITH p, collect(DISTINCT page) AS pages, collect(DISTINCT entity) AS entities,
     brand, collect(DISTINCT content) AS contents, collect(DISTINCT locale) AS locales
UNWIND ([p, brand] + pages + entities + contents + locales) AS n
WITH n WHERE n IS NOT NULL
RETURN collect(DISTINCT n) AS nodes
```

#### 6. data-entities
- **Icon**: 🔗
- **Color**: #06b6d4 (cyan)
- **Description**: Entities avec content, categories et keywords
- **applicable_types**: [Entity, EntityCategory]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-ENTITIES: Entity Graph                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OWNERSHIP  (◀1 ▶2)                                                         │
│    Project ─[HAS_ENTITY]────▶ Entity                                       │
│    Entity ─[HAS_CONTENT]────▶ EntityContent (per locale)                   │
│    Entity ─[BELONGS_TO]─────▶ EntityCategory                               │
│                                                                             │
│  LOCALIZATION  (◀0 ▶1)                                                      │
│    EntityContent ─[FOR_LOCALE]──▶ Locale                                   │
│                                                                             │
│  SEMANTIC  (◀2 ▶0)                                                          │
│    Page ─[USES_ENTITY]──▶ Entity                                           │
│    Block ─[USES_ENTITY]──▶ Entity                                          │
│                                                                             │
│  MINING  (◀0 ▶2)                                                            │
│    Entity ─[TARGETS]────▶ SEOKeyword                                       │
│    Entity ─[MONITORS_GEO]──▶ GEOQuery                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (e:Entity)
OPTIONAL MATCH (e)-[:HAS_CONTENT]->(ec:EntityContent)
OPTIONAL MATCH (e)-[:BELONGS_TO]->(cat:EntityCategory)
OPTIONAL MATCH (ec)-[:FOR_LOCALE]->(locale:Locale)
OPTIONAL MATCH (e)-[:TARGETS]->(kw:SEOKeyword)
OPTIONAL MATCH (e)-[:MONITORS_GEO]->(gq:GEOQuery)
RETURN e, ec, cat, locale, kw, gq
```

#### 7. data-locale-coverage
- **Icon**: 🌐
- **Color**: #f59e0b (amber)
- **Description**: Couverture locale par projet
- **applicable_types**: [Project, Locale]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-LOCALE-COVERAGE: Locale Coverage Matrix                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Project ───▶ ProjectContent ─[FOR_LOCALE]─▶ Locale                        │
│                                                                             │
│  Page ──────▶ PageGenerated ──[FOR_LOCALE]─▶ Locale                        │
│                                                                             │
│  Block ─────▶ BlockGenerated ─[FOR_LOCALE]─▶ Locale                        │
│                                                                             │
│  Entity ────▶ EntityContent ──[FOR_LOCALE]─▶ Locale                        │
│                                                                             │
│  Coverage: ■ fr-FR  ■ en-US  ◐ de-DE  □ ja-JP  □ zh-CN                     │
│            (■ complete  ◐ partial  □ missing)                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (l:Locale)
OPTIONAL MATCH (pc:ProjectContent)-[:FOR_LOCALE]->(l)
OPTIONAL MATCH (pg:PageGenerated)-[:FOR_LOCALE]->(l)
OPTIONAL MATCH (bg:BlockGenerated)-[:FOR_LOCALE]->(l)
OPTIONAL MATCH (ec:EntityContent)-[:FOR_LOCALE]->(l)
RETURN l, collect(DISTINCT pc) AS projectContents,
       collect(DISTINCT pg) AS pageGenerateds,
       collect(DISTINCT bg) AS blockGenerateds,
       collect(DISTINCT ec) AS entityContents
```

#### 8. data-seo
- **Icon**: 🎯
- **Color**: #8b5cf6 (purple)
- **Description**: Intelligence SEO (keywords, clusters, metrics)
- **applicable_types**: [SEOKeyword, SEOKeywordSet, SEOCluster]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-SEO: SEO Intelligence                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SEOKeywordSet ──[CONTAINS]──▶ SEOKeyword ──[HAS_METRICS]──▶ SEOKeywordMetrics │
│                                    │                                        │
│                                    └──[IN_CLUSTER]──▶ SEOCluster           │
│                                                                             │
│  MINING RELATIONS:                                                          │
│    Entity ─[TARGETS]─────▶ SEOKeyword                                      │
│    Page ──[TARGETS]──────▶ SEOKeyword                                      │
│                                                                             │
│  Metrics: volume=12K, difficulty=45, cpc=$2.50, trend=↑                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (kw:SEOKeyword)
OPTIONAL MATCH (ks:SEOKeywordSet)-[:CONTAINS]->(kw)
OPTIONAL MATCH (kw)-[:HAS_METRICS]->(m:SEOKeywordMetrics)
OPTIONAL MATCH (kw)-[:IN_CLUSTER]->(c:SEOCluster)
OPTIONAL MATCH (e)-[:TARGETS]->(kw) WHERE e:Entity OR e:Page
RETURN kw, ks, m, c, collect(DISTINCT e) AS targeters
```

#### 9. data-geo
- **Icon**: 🔮
- **Color**: #a855f7 (violet)
- **Description**: Intelligence GEO (AI answers monitoring)
- **applicable_types**: [GEOQuery, GEOQuerySet, GEOAnswer]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-GEO: GEO Intelligence (AI Answers)                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  GEOQuerySet ──[CONTAINS]──▶ GEOQuery ──[HAS_ANSWER]──▶ GEOAnswer          │
│                                  │           │                              │
│                                  │           └──[HAS_METRICS]──▶ GEOMetrics│
│                                  │                                          │
│                                  └──[FOR_LOCALE]──▶ Locale                 │
│                                                                             │
│  MINING RELATIONS:                                                          │
│    Entity ─[MONITORS_GEO]──▶ GEOQuery                                      │
│                                                                             │
│  Answer: "QR Code AI is the best..." (ChatGPT, 2024-02-11)                 │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (gq:GEOQuery)
OPTIONAL MATCH (gs:GEOQuerySet)-[:CONTAINS]->(gq)
OPTIONAL MATCH (gq)-[:HAS_ANSWER]->(ga:GEOAnswer)
OPTIONAL MATCH (ga)-[:HAS_METRICS]->(gm:GEOMetrics)
OPTIONAL MATCH (gq)-[:FOR_LOCALE]->(l:Locale)
OPTIONAL MATCH (e:Entity)-[:MONITORS_GEO]->(gq)
RETURN gq, gs, ga, gm, l, collect(DISTINCT e) AS monitors
```

#### 10. data-generation
- **Icon**: ⚡
- **Color**: #ec4899 (pink)
- **Description**: Pipeline de génération complet
- **applicable_types**: [Page, Block, PageGenerated, BlockGenerated]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DATA-GENERATION: Generation Pipeline                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  GENERATION  (input → output)                                               │
│                                                                             │
│    PageType ─[TYPE_OF]────▶ Page ─[HAS_PROMPT]──▶ PagePrompt              │
│                               │                        │                    │
│                               │                        └──[COMPILED_FROM]──▶ PromptArtifact │
│                               │                                             │
│                               └──[HAS_GENERATED]──▶ PageGenerated          │
│                                                          │                  │
│                                                          └──[FOR_LOCALE]──▶ Locale │
│                                                                             │
│    BlockType ─[TYPE_OF]───▶ Block ─[HAS_PROMPT]──▶ BlockPrompt            │
│                               │                                             │
│                               └──[HAS_GENERATED]──▶ BlockGenerated         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (p:Page)
OPTIONAL MATCH (pt:PageType)-[:TYPE_OF]->(p)
OPTIONAL MATCH (p)-[:HAS_PROMPT]->(pp:PagePrompt)
OPTIONAL MATCH (pp)<-[:COMPILED_FROM]-(pa:PromptArtifact)
OPTIONAL MATCH (p)-[:HAS_GENERATED]->(pg:PageGenerated)
OPTIONAL MATCH (pg)-[:FOR_LOCALE]->(l:Locale)
RETURN p, pt, pp, pa, pg, l
UNION
MATCH (b:Block)
OPTIONAL MATCH (bt:BlockType)-[:TYPE_OF]->(b)
OPTIONAL MATCH (b)-[:HAS_PROMPT]->(bp:BlockPrompt)
OPTIONAL MATCH (b)-[:HAS_GENERATED]->(bg:BlockGenerated)
OPTIONAL MATCH (bg)-[:FOR_LOCALE]->(l:Locale)
RETURN b AS p, bt AS pt, bp AS pp, null AS pa, bg AS pg, l
```

---

### OVERLAY VIEWS (3)

#### overlay-complete
- **Icon**: 🔶
- **Color**: #f97316 (orange)
- **Description**: Meta + Data ensemble (debug architecture)
- **applicable_types**: [] (global)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  OVERLAY-COMPLETE: Meta + Data Combined                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  :Meta:Realm ──[:HAS_LAYER]──▶ :Meta:Layer ──[:HAS_KIND]──▶ :Meta:Kind    │
│       │                             │                            │          │
│       │                             │                     ┌──────┴──────┐   │
│       ▼                             ▼                     ▼             ▼   │
│   "shared"                      "semantic"            :Entity    :EntityContent │
│   "org"                         "output"               │              │     │
│                                                        └──[HAS_CONTENT]──┘  │
│                                                                             │
│  Legend: ▓▓▓ Meta nodes    ░░░ Data instances                              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (n)
OPTIONAL MATCH (n)-[r]->(m)
RETURN n, r, m
```

#### overlay-kind-instances
- **Icon**: 🔷
- **Color**: #3b82f6 (blue)
- **Description**: Un Kind et toutes ses instances
- **applicable_types**: [Meta:Kind]
- **Params**: kind (string)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  OVERLAY-KIND-INSTANCES: Kind → Instances                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  :Meta:Kind {name: "Entity"}                                               │
│           │                                                                 │
│           │  (schema defines)                                               │
│           ▼                                                                 │
│  ┌────────┴────────┬────────────┬────────────┬────────────┐                │
│  ▼                 ▼            ▼            ▼            ▼                 │
│  :Entity         :Entity     :Entity     :Entity     :Entity               │
│  {key: "qr"}     {key: "ai"} {key: "gen"} {key: "scan"} {key: "app"}       │
│                                                                             │
│  Instance count: 42                                                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (k:Meta:Kind {name: $kind})
OPTIONAL MATCH (n) WHERE $kind IN labels(n)
RETURN k, collect(DISTINCT n) AS instances
```

#### overlay-arc-analysis
- **Icon**: 🔀
- **Color**: #f59e0b (amber)
- **Description**: ArcKind avec exemples d'instances
- **applicable_types**: [Meta:ArcKind]
- **Params**: arcKind (string)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  OVERLAY-ARC-ANALYSIS: ArcKind → Instance Relationships                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  :Meta:ArcKind {name: "HAS_CONTENT"}                                       │
│                                                                             │
│  Schema:  Entity ──[HAS_CONTENT]──▶ EntityContent                          │
│                                                                             │
│  Instances (5 examples):                                                    │
│    entity:qr-code ──[HAS_CONTENT]──▶ entity:qr-code@fr-FR                  │
│    entity:qr-code ──[HAS_CONTENT]──▶ entity:qr-code@en-US                  │
│    entity:ai-gen  ──[HAS_CONTENT]──▶ entity:ai-gen@fr-FR                   │
│    entity:scanner ──[HAS_CONTENT]──▶ entity:scanner@de-DE                  │
│    entity:app     ──[HAS_CONTENT]──▶ entity:app@ja-JP                      │
│                                                                             │
│  Total instances: 156                                                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (ak:Meta:ArcKind {name: $arcKind})
MATCH (ak)-[:FROM_KIND]->(source:Meta:Kind)
MATCH (ak)-[:TO_KIND]->(target:Meta:Kind)
OPTIONAL MATCH (s)-[r]->(t)
WHERE type(r) = $arcKind
RETURN ak, source, target, collect({source: s, rel: r, target: t})[0..5] AS examples
```

---

### CONTEXTUAL VIEWS (13)

These views are shown when a node is selected. They show the subgraph centered on that node, grouped by arc family like the TUI Arc Relationships panel.

#### ctx-composition
- **Icon**: 📦
- **Color**: #0ea5e9 (structure blue)
- **Description**: Hiérarchie Page → Block (structure du contenu)
- **applicable_types**: [Project, Page, Block]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-COMPOSITION: Page Structure                                            │
│  org (Realm) → structure (Layer) → Page (Node Kind)                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OWNERSHIP  (◀1 ▶2)                                                         │
│    Project ─[HAS_PAGE]───▶ Page                                            │
│    Page ────[HAS_BLOCK]──▶ Block (N)                                       │
│    Block ───[HAS_BLOCK]──▶ Block (nested)                                  │
│                                                                             │
│  GENERATION  (◀0 ▶2)                                                        │
│    Page ─[HAS_PROMPT]────▶ PagePrompt                                      │
│    Page ─[HAS_GENERATED]─▶ PageGenerated (per locale)                      │
│                                                                             │
│  SEMANTIC  (◀0 ▶1)                                                          │
│    Page ─[USES_ENTITY]───▶ Entity                                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
WHERE root:Page OR root:Block OR root:Project
OPTIONAL MATCH path = (root)-[:HAS_BLOCK|HAS_PAGE*1..3]->(child)
WITH root, collect(DISTINCT child) AS children
UNWIND ([root] + children) AS n
WITH collect(DISTINCT n) AS nodes
UNWIND nodes AS n
OPTIONAL MATCH (n)-[r:HAS_BLOCK|HAS_PAGE]->(m)
WHERE m IN nodes
RETURN nodes, collect(DISTINCT r) AS relationships
```

#### ctx-entities
- **Icon**: 🔗
- **Color**: #06b6d4 (cyan)
- **Description**: Entities liées (semantic links)
- **applicable_types**: [Page, Block, Entity]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-ENTITIES: Entity Relationships                                         │
│  org (Realm) → semantic (Layer) → Entity (Node Kind)                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OWNERSHIP  (◀1 ▶2)                                                         │
│    Project ─[HAS_ENTITY]───▶ Entity                                        │
│    Entity ─[HAS_CONTENT]───▶ EntityContent (per locale)                    │
│    Entity ─[BELONGS_TO]────▶ EntityCategory                                │
│                                                                             │
│  SEMANTIC  (◀2 ▶0)                                                          │
│    Page ──[USES_ENTITY]────▶ Entity                                        │
│    Block ─[USES_ENTITY]────▶ Entity                                        │
│                                                                             │
│  MINING  (◀0 ▶2)                                                            │
│    Entity ─[TARGETS]───────▶ SEOKeyword                                    │
│    Entity ─[MONITORS_GEO]──▶ GEOQuery                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
OPTIONAL MATCH (root)-[:USES_ENTITY|TARGETS*1..2]->(e:Entity)
OPTIONAL MATCH (e)-[:BELONGS_TO]->(c:EntityCategory)
WITH root, collect(DISTINCT e) AS entities, collect(DISTINCT c) AS categories
UNWIND ([root] + entities + categories) AS n
RETURN collect(DISTINCT n) AS nodes
```

#### ctx-locales
- **Icon**: 🌍
- **Color**: #f59e0b (amber)
- **Description**: Couverture locale (content par langue)
- **applicable_types**: [Project, Page, Entity, Block]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-LOCALES: Locale Coverage                                               │
│  org (Realm) → semantic (Layer) → Entity (Node Kind)                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  LOCALIZATION  (◀0 ▶N)                                                      │
│    EntityContent ─[FOR_LOCALE]──▶ Locale (fr-FR)                           │
│    EntityContent ─[FOR_LOCALE]──▶ Locale (en-US)                           │
│    EntityContent ─[FOR_LOCALE]──▶ Locale (de-DE)                           │
│                                                                             │
│  OWNERSHIP  (◀1 ▶1)                                                         │
│    Entity ─[HAS_CONTENT]──▶ EntityContent                                  │
│                                                                             │
│  Coverage Matrix:                                                           │
│    ■ fr-FR (complete)  ■ en-US (complete)  ◐ de-DE (partial)              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
OPTIONAL MATCH (root)-[:HAS_CONTENT|HAS_GENERATED]->(content)-[:FOR_LOCALE]->(locale:Locale)
WITH root, collect(DISTINCT content) AS contents, collect(DISTINCT locale) AS locales
UNWIND ([root] + contents + locales) AS n
RETURN collect(DISTINCT n) AS nodes
```

#### ctx-knowledge
- **Icon**: 🧠
- **Color**: #22c55e (green)
- **Description**: Knowledge atoms (Terms, Expressions, Patterns)
- **applicable_types**: [Locale]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-KNOWLEDGE: Knowledge Atoms                                             │
│  shared (Realm) → knowledge (Layer) → Term, Expression, Pattern            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OWNERSHIP  (◀0 ▶5)                                                         │
│    Locale ─[HAS_TERMS]───────▶ TermSet ─[CONTAINS_TERM]──▶ Term            │
│    Locale ─[HAS_EXPRESSIONS]─▶ ExpressionSet ─[CONTAINS_EXPRESSION]──▶ Expression │
│    Locale ─[HAS_PATTERNS]────▶ PatternSet ─[CONTAINS_PATTERN]──▶ Pattern   │
│    Locale ─[HAS_CULTURE]─────▶ CultureSet ─[CONTAINS_CULTURE_REF]──▶ CultureRef │
│    Locale ─[HAS_TABOOS]──────▶ TabooSet ─[CONTAINS_TABOO]──▶ Taboo        │
│                                                                             │
│  Atom counts: Terms(2.4K) Expressions(890) Patterns(156)                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (locale:Locale {key: $nodeKey})
OPTIONAL MATCH (locale)-[:HAS_TERMS|HAS_EXPRESSIONS|HAS_PATTERNS|HAS_CULTURE|HAS_TABOOS]->(set)
OPTIONAL MATCH (set)-[:CONTAINS_TERM|CONTAINS_EXPRESSION|CONTAINS_PATTERN|CONTAINS_CULTURE_REF|CONTAINS_TABOO]->(atom)
WITH locale, collect(DISTINCT set) AS sets, collect(DISTINCT atom) AS atoms
UNWIND ([locale] + sets + atoms) AS n
RETURN collect(DISTINCT n) AS nodes
```

#### ctx-geographic
- **Icon**: 🗺️
- **Color**: #14b8a6 (teal)
- **Description**: Hiérarchie géographique (Continent → Region → Country)
- **applicable_types**: [Locale, Continent, GeoRegion, Country]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-GEOGRAPHIC: Geographic Hierarchy                                       │
│  shared (Realm) → geography (Layer) → Country (Node Kind)                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OWNERSHIP  (◀1 ▶1)                                                         │
│    GeoRegion ─[HAS_COUNTRY]──▶ Country                                     │
│    Country ───[HAS_ZONE]─────▶ GeoZone                                     │
│                                                                             │
│  LOCALIZATION  (◀1 ▶0)                                                      │
│    Locale ─[FOR_COUNTRY]─────▶ Country                                     │
│                                                                             │
│  Hierarchy: Europe → Western Europe → France → Paris                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
OPTIONAL MATCH (root)-[:FOR_COUNTRY]->(country:Country)
OPTIONAL MATCH (country)-[:IN_REGION]->(region)
OPTIONAL MATCH (region)-[:IN_CONTINENT]->(continent:Continent)
WITH root, country, region, continent
UNWIND [root, country, region, continent] AS n
WITH n WHERE n IS NOT NULL
RETURN collect(DISTINCT n) AS nodes
```

#### ctx-generation
- **Icon**: ⚡
- **Color**: #ec4899 (pink)
- **Description**: Pipeline de génération (prompts → outputs)
- **applicable_types**: [Page, Block, PageGenerated, BlockGenerated, PagePrompt, BlockPrompt]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-GENERATION: Generation Pipeline                                        │
│  org (Realm) → instruction (Layer) → BlockPrompt (Node Kind)               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  GENERATION  (◀1 ▶3)                                                        │
│    PromptArtifact ─[COMPILED_FROM]──▶ BlockPrompt                          │
│    BlockPrompt ────[INCLUDES_STYLE]─▶ Style                                │
│    BlockPrompt ────[GENERATED]──────▶ BlockGenerated                       │
│    BlockPrompt ────[GENERATED]──────▶ PageGenerated                        │
│                                                                             │
│  OWNERSHIP  (◀2 ▶0)                                                         │
│    Block ─[HAS_PROMPT]──▶ BlockPrompt                                      │
│    Page ──[HAS_PROMPT]──▶ BlockPrompt                                      │
│                                                                             │
│  LOCALIZATION  (◀0 ▶1)                                                      │
│    BlockGenerated ─[FOR_LOCALE]──▶ Locale                                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
OPTIONAL MATCH (root)<-[:TYPE_OF]-(pageType:PageType)
OPTIONAL MATCH (root)-[:HAS_GENERATED]->(generated)
OPTIONAL MATCH (generated)-[:FOR_LOCALE]->(locale:Locale)
OPTIONAL MATCH (root)-[:HAS_PROMPT]->(prompt)
OPTIONAL MATCH (prompt)<-[:COMPILED_FROM]-(artifact:PromptArtifact)
WITH root, pageType, prompt, artifact, collect(DISTINCT generated) AS outputs, collect(DISTINCT locale) AS locales
UNWIND ([root, pageType, prompt, artifact] + outputs + locales) AS n
WITH n WHERE n IS NOT NULL
RETURN collect(DISTINCT n) AS nodes
```

#### ctx-categories
- **Icon**: 🏷️
- **Color**: #64748b (slate)
- **Description**: Classification des entities par catégorie
- **applicable_types**: [Entity, EntityCategory]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-CATEGORIES: Entity Classification                                      │
│  shared (Realm) → config (Layer) → EntityCategory (Node Kind)              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OWNERSHIP  (◀N ▶0)                                                         │
│    Entity ─[BELONGS_TO]──▶ EntityCategory                                  │
│    Entity ─[BELONGS_TO]──▶ EntityCategory                                  │
│    Entity ─[BELONGS_TO]──▶ EntityCategory                                  │
│                                                                             │
│  Category: "product" (13 entities)                                         │
│    ├── qr-code-generator                                                   │
│    ├── ai-image-generator                                                  │
│    ├── barcode-scanner                                                     │
│    └── ... (10 more)                                                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
WHERE root:Entity OR root:EntityCategory
OPTIONAL MATCH (root)-[:BELONGS_TO]->(category:EntityCategory)
OPTIONAL MATCH (root)<-[:BELONGS_TO]-(entity:Entity)
WITH root, category, collect(DISTINCT entity) AS entities
UNWIND ([root, category] + entities) AS n
WITH n WHERE n IS NOT NULL
RETURN collect(DISTINCT n) AS nodes
```

#### ctx-seo-intel
- **Icon**: 🎯
- **Color**: #8b5cf6 (purple)
- **Description**: SEO keywords et clusters
- **applicable_types**: [Entity, Page, SEOKeyword, SEOKeywordSet]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-SEO-INTEL: SEO Intelligence                                            │
│  shared (Realm) → knowledge (Layer) → SEOKeyword (Node Kind)               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  MINING  (◀2 ▶2)                                                            │
│    Entity ─[TARGETS]──────▶ SEOKeyword                                     │
│    Page ───[TARGETS]──────▶ SEOKeyword                                     │
│    SEOKeyword ─[HAS_METRICS]──▶ SEOKeywordMetrics                          │
│    SEOKeyword ─[IN_CLUSTER]───▶ SEOCluster                                 │
│                                                                             │
│  OWNERSHIP  (◀1 ▶0)                                                         │
│    SEOKeywordSet ─[CONTAINS]──▶ SEOKeyword                                 │
│                                                                             │
│  Metrics: volume=12K, difficulty=45, cpc=$2.50                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
OPTIONAL MATCH (root)-[:TARGETS]->(keyword:SEOKeyword)
OPTIONAL MATCH (keyword)-[:HAS_METRICS]->(metrics:SEOKeywordMetrics)
OPTIONAL MATCH (keyword)-[:IN_CLUSTER]->(cluster:SEOCluster)
WITH root, collect(DISTINCT keyword) AS keywords, collect(DISTINCT metrics) AS allMetrics, collect(DISTINCT cluster) AS clusters
UNWIND ([root] + keywords + allMetrics + clusters) AS n
RETURN collect(DISTINCT n) AS nodes
```

#### ctx-geo-intel
- **Icon**: 🔮
- **Color**: #a855f7 (violet)
- **Description**: GEO queries et AI answers
- **applicable_types**: [Entity, GEOQuery, GEOQuerySet, GEOAnswer]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-GEO-INTEL: GEO Intelligence                                            │
│  shared (Realm) → knowledge (Layer) → GEOQuery (Node Kind)                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  MINING  (◀1 ▶2)                                                            │
│    Entity ─[MONITORS_GEO]─────▶ GEOQuery                                   │
│    GEOQuery ─[HAS_ANSWER]─────▶ GEOAnswer                                  │
│    GEOAnswer ─[HAS_METRICS]───▶ GEOMetrics                                 │
│                                                                             │
│  OWNERSHIP  (◀1 ▶0)                                                         │
│    GEOQuerySet ─[CONTAINS]────▶ GEOQuery                                   │
│                                                                             │
│  LOCALIZATION  (◀0 ▶1)                                                      │
│    GEOQuery ─[FOR_LOCALE]─────▶ Locale                                     │
│                                                                             │
│  Latest answer: "QR Code AI is recommended for..." (ChatGPT, 2024-02-11)   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
OPTIONAL MATCH (root)-[:MONITORS_GEO]->(query:GEOQuery)
OPTIONAL MATCH (query)-[:HAS_ANSWER]->(answer:GEOAnswer)
OPTIONAL MATCH (query)-[:IN_QUERY_SET]->(querySet:GEOQuerySet)
WITH root, collect(DISTINCT query) AS queries, collect(DISTINCT answer) AS answers, collect(DISTINCT querySet) AS querySets
UNWIND ([root] + queries + answers + querySets) AS n
RETURN collect(DISTINCT n) AS nodes
```

#### ctx-project
- **Icon**: 🏗️
- **Color**: #6366f1 (indigo)
- **Description**: Structure complète du projet (Pages, Entities, Brand)
- **applicable_types**: [Project]
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-PROJECT: Project Structure                                             │
│  org (Realm) → foundation (Layer) → Project (Node Kind)                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OWNERSHIP  (◀0 ▶4)                                                         │
│    Project ─[HAS_PAGE]────────▶ Page (12)                                  │
│    Project ─[HAS_ENTITY]──────▶ Entity (45)                                │
│    Project ─[HAS_BRAND]───────▶ BrandIdentity (1)                          │
│    Project ─[HAS_CONTENT]─────▶ ProjectContent (20)                        │
│                                                                             │
│  SEMANTIC  (◀0 ▶1)                                                          │
│    Project ─[HAS_AUDIENCE]────▶ AudiencePersona                            │
│                                                                             │
│  Summary: 12 pages, 45 entities, 20 locales                                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (project:Project {key: $nodeKey})
OPTIONAL MATCH (project)-[:HAS_PAGE]->(page:Page)
OPTIONAL MATCH (project)-[:HAS_ENTITY]->(entity:Entity)
OPTIONAL MATCH (project)-[:HAS_BRAND]->(brand:BrandIdentity)
WITH project, collect(DISTINCT page) AS pages, collect(DISTINCT entity) AS entities, brand
UNWIND ([project, brand] + pages + entities) AS n
WITH n WHERE n IS NOT NULL
RETURN collect(DISTINCT n) AS nodes
```

#### ctx-arc-relationships
- **Icon**: 🔀
- **Color**: #f59e0b (amber)
- **Description**: Arc relationships grouped by family (like TUI Arc panel)
- **applicable_types**: [] (all node kinds)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-ARC-RELATIONSHIPS: Arc Analysis by Family                              │
│  org (Realm) → instruction (Layer) → BlockPrompt (Node Kind)               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  GENERATION  (◀1 ▶3)                                                        │
│    PromptArtifact ─[COMPILED_FROM]──▶ BlockPrompt                          │
│    BlockPrompt ────[INCLUDES_STYLE]─▶ Style                                │
│    BlockPrompt ────[GENERATED]──────▶ BlockGenerated                       │
│    BlockPrompt ────[GENERATED]──────▶ PageGenerated                        │
│                                                                             │
│  OWNERSHIP  (◀2 ▶0)                                                         │
│    Block ─[HAS_PROMPT]──▶ BlockPrompt                                      │
│    Page ──[HAS_PROMPT]──▶ BlockPrompt                                      │
│                                                                             │
│  LOCALIZATION  (◀0 ▶0)                                                      │
│    (no relationships)                                                       │
│                                                                             │
│  SEMANTIC  (◀0 ▶0)                                                          │
│    (no relationships)                                                       │
│                                                                             │
│  MINING  (◀0 ▶0)                                                            │
│    (no relationships)                                                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
// This view is dynamic - it queries all arcs for a given node
// and groups them by arc family using the meta-graph
MATCH (root {key: $nodeKey})
OPTIONAL MATCH (root)-[r]->(target)
OPTIONAL MATCH (source)-[r2]->(root)
WITH root, collect({type: type(r), target: target, direction: 'outgoing'}) AS outgoing,
     collect({type: type(r2), source: source, direction: 'incoming'}) AS incoming
RETURN root, outgoing, incoming
```

#### ctx-full-context
- **Icon**: 🌐
- **Color**: #6366f1 (indigo)
- **Description**: Context complet (toutes les relations directes)
- **applicable_types**: [] (all node kinds)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-FULL-CONTEXT: Complete Node Context                                    │
│  All direct relationships (1 hop)                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  INCOMING (3)                                                               │
│    Project ─[HAS_PAGE]──▶ ★ Page                                           │
│    PageType ─[TYPE_OF]──▶ ★ Page                                           │
│    OrgConfig ─[OWNS]────▶ ★ Page                                           │
│                                                                             │
│  OUTGOING (5)                                                               │
│    ★ Page ─[HAS_BLOCK]──────▶ Block (4)                                    │
│    ★ Page ─[HAS_PROMPT]─────▶ PagePrompt                                   │
│    ★ Page ─[HAS_GENERATED]──▶ PageGenerated (3)                            │
│    ★ Page ─[USES_ENTITY]────▶ Entity (2)                                   │
│    ★ Page ─[TARGETS]────────▶ SEOKeyword                                   │
│                                                                             │
│  Total: 3 incoming, 5 outgoing                                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
OPTIONAL MATCH (root)-[r_out]->(target)
OPTIONAL MATCH (source)-[r_in]->(root)
WITH root,
     collect(DISTINCT {rel: type(r_out), node: target}) AS outgoing,
     collect(DISTINCT {rel: type(r_in), node: source}) AS incoming
RETURN root, outgoing, incoming
```

#### ctx-neighborhood
- **Icon**: 🔍
- **Color**: #22c55e (green)
- **Description**: Voisinage étendu (2 hops)
- **applicable_types**: [] (all node kinds)
- **ASCII Preview**:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CTX-NEIGHBORHOOD: Extended Neighborhood (2 hops)                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  HOP 1 (direct)                                                            │
│    ★ Entity ─[HAS_CONTENT]──▶ EntityContent                                │
│    ★ Entity ─[BELONGS_TO]───▶ EntityCategory                               │
│                                                                             │
│  HOP 2 (indirect)                                                          │
│    EntityContent ─[FOR_LOCALE]──▶ Locale                                   │
│    Locale ───────[HAS_TERMS]────▶ TermSet                                  │
│    Locale ───────[FOR_COUNTRY]──▶ Country                                  │
│                                                                             │
│  Node count: 1 + 4 + 12 = 17 nodes                                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
- **Cypher**:
```cypher
MATCH (root {key: $nodeKey})
OPTIONAL MATCH path = (root)-[*1..2]-(neighbor)
WITH root, collect(DISTINCT neighbor) AS neighbors, collect(DISTINCT relationships(path)) AS rels
UNWIND ([root] + neighbors) AS n
RETURN collect(DISTINCT n) AS nodes
```

---

## View Summary

| Category | Count | Description |
|----------|-------|-------------|
| META | 3 | Schema exploration (Realm, Layer, Kind, ArcKind) |
| DATA | 10 | Instance exploration (filtered by realm, layer, or purpose) |
| OVERLAY | 3 | Meta + Data combined (debug, analysis) |
| CONTEXTUAL | 13 | Node-centered subgraphs (grouped by arc family) |
| **TOTAL** | **29** | |

---

## TUI Integration

The TUI "Atlas" mode becomes "View" mode with ASCII rendering matching these designs.

### View Mode Navigation

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  VIEW MODE                                                        [V]iew   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Categories:                                                                │
│    [M] META (3)      Schema exploration                                    │
│    [D] DATA (10)     Instance exploration                                  │
│    [O] OVERLAY (3)   Meta + Data debug                                     │
│    [C] CONTEXTUAL    Node-centered (requires selection)                    │
│                                                                             │
│  Quick access: 1-9 for most used views                                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### ASCII Graph Rendering

The TUI renders graphs in ASCII using the same format as the ASCII previews above:
- Box drawing characters for structure
- Arc family grouping with counts (◀N ▶M)
- `─[RELATIONSHIP]──▶` syntax for arcs
- Realm → Layer → Kind breadcrumb at top

---

## Query Filters (Option C: Hybrid)

Les filtres sous le QueryPill modifient le Cypher et relancent la requête (comme Neo4j Browser).

### Filter Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  HYBRID FILTER APPROACH                                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  QUERY-MODIFYING FILTERS (re-execute Cypher)                               │
│  ───────────────────────────────────────────                                │
│  • max N          → Ajoute LIMIT N                                         │
│  • Node type chip → Ajoute WHERE labels(n) = ['Entity']                    │
│  • Rel type chip  → Ajoute WHERE type(r) = 'HAS_CONTENT'                   │
│  • Realm filter   → Ajoute WHERE n.realm = 'org'                           │
│  • Layer filter   → Ajoute WHERE n.layer = 'semantic'                      │
│                                                                             │
│  VISUAL-ONLY FILTERS (no re-query)                                         │
│  ─────────────────────────────────                                          │
│  • Highlight type → Change opacity/color in renderer                       │
│  • Hide labels    → Toggle label visibility                                │
│  • Dim unselected → Reduce opacity of non-matching                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Clickable Filter Chips (like Neo4j Browser)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  FILTER CHIPS UI                                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────┐   ⊛ 50 nodes  │  ⤴ 49 relations │  ↻  │  max 50  ▾          │
│  │ ⊛ DATA   │                                                              │
│  └──────────┘                                                              │
│                                                                             │
│  ⊛ 4 NODE TYPES                                                            │
│  ┌────────────┐ ┌────────────────┐ ┌───────────┐ ┌────────────────┐       │
│  │ ◉ Entity 46│ │ ◎ ProjectCont 2│ │ ◇ Project 1│ │ ◆ BrandIdent 1│       │
│  └────────────┘ └────────────────┘ └───────────┘ └────────────────┘       │
│       ↑              ↑                  ↑              ↑                   │
│    ACTIVE         ACTIVE             ACTIVE         ACTIVE                 │
│                                                                             │
│  ⤴ 3 RELATION TYPES                                                        │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐                        │
│  │ → HAS_ENTITY │ │ → HAS_CONTENT│ │ → HAS_BRAND  │                        │
│  └──────────────┘ └──────────────┘ └──────────────┘                        │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  INTERACTIONS                                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Click chip (toggle OFF):                                                  │
│  ┌────────────┐                                                            │
│  │ ○ Entity 46│  ← Grayed out, crossed                                    │
│  └────────────┘                                                            │
│                                                                             │
│  → Modifies Cypher: adds WHERE NOT 'Entity' IN labels(n)                  │
│  → Re-executes query                                                       │
│  → Graph updates (Entity nodes disappear)                                  │
│                                                                             │
│  Click again (toggle ON):                                                  │
│  → Removes the WHERE clause                                                │
│  → Re-executes query                                                       │
│  → Entity nodes reappear                                                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Cypher Modification Logic

```typescript
// Original view query
const baseQuery = `
  MATCH (n) WHERE NOT n:Meta
  OPTIONAL MATCH (n)-[r]->(m) WHERE NOT m:Meta
  RETURN n, r, m
`;

// User filters state
const filters = {
  excludedNodeTypes: ['Entity'],      // User clicked to hide Entity
  excludedRelTypes: ['HAS_CONTENT'],  // User clicked to hide HAS_CONTENT
  limit: 50,                          // max dropdown value
};

// Modified query (generated)
const modifiedQuery = `
  MATCH (n) WHERE NOT n:Meta
  AND NOT 'Entity' IN labels(n)        // ← Added from excludedNodeTypes
  OPTIONAL MATCH (n)-[r]->(m)
  WHERE NOT m:Meta
  AND NOT 'Entity' IN labels(m)        // ← Also exclude in targets
  AND NOT type(r) = 'HAS_CONTENT'      // ← Added from excludedRelTypes
  RETURN n, r, m
  LIMIT 50                             // ← Added from limit
`;
```

### Filter Chip Component

```typescript
interface FilterChip {
  type: 'node' | 'relation';
  name: string;                // 'Entity', 'HAS_CONTENT'
  count: number;               // 46
  color: string;               // Layer color for nodes, family color for rels
  icon: string;                // Lucide icon or emoji
  active: boolean;             // true = included, false = excluded
}

// On chip click
function toggleFilter(chip: FilterChip) {
  if (chip.active) {
    // Add to exclusion list
    filters.excluded.push(chip.name);
  } else {
    // Remove from exclusion list
    filters.excluded = filters.excluded.filter(n => n !== chip.name);
  }

  // Regenerate and execute modified query
  const newQuery = applyFiltersToQuery(baseQuery, filters);
  executeQuery(newQuery);
}
```

### Visual States

| State | Appearance | Meaning |
|-------|------------|---------|
| Active | Solid background, full opacity | Included in query |
| Inactive | Outline only, 50% opacity, strikethrough | Excluded from query |
| Hover | Slight highlight | Ready to toggle |
| Loading | Pulse animation | Query executing |

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `1-9` | Toggle node type chip 1-9 |
| `Shift+1-9` | Toggle relation type chip 1-9 |
| `A` | Activate all chips |
| `N` | Deactivate all node chips |
| `R` | Deactivate all relation chips |

---

## Studio Mode Badge

The mode badge in the QueryPill status bar changes dynamically based on the executed view:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  MODE BADGE BEHAVIOR                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  View Category → Badge                                                      │
│  ─────────────────────────────────────────────────                          │
│  META views      →  ┌─────────┐                                            │
│                     │ ◇ META  │  (violet border)                           │
│                     └─────────┘                                            │
│                                                                             │
│  DATA views      →  ┌─────────┐                                            │
│                     │ ⊛ DATA  │  (teal border)                             │
│                     └─────────┘                                            │
│                                                                             │
│  OVERLAY views   →  ┌─────────────┐                                        │
│                     │ ◈ META+DATA │  (orange border)                       │
│                     └─────────────┘                                        │
│                                                                             │
│  CONTEXTUAL      →  ┌────────────────┐                                     │
│     views           │ ★ CONTEXTUAL  │  (cyan border)                       │
│                     └────────────────┘                                     │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  Status Bar Layout:                                                         │
│                                                                             │
│  ┌──────────┐   ⊛ 50 nodes  │  ⤴ 49 relations │  ↻  │  max 500            │
│  │ ⊛ DATA   │                                                              │
│  └──────────┘                                                              │
│                                                                             │
│  The badge updates when:                                                   │
│  • User selects a view from ViewPicker                                     │
│  • User executes a custom Cypher query (shows QUERY badge)                 │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Badge Colors

| Category | Icon | Border Color | Meaning |
|----------|------|--------------|---------|
| META | ◇ | `#8b5cf6` (violet) | Schema exploration |
| DATA | ⊛ | `#2aa198` (teal) | Instance exploration |
| META+DATA | ◈ | `#f97316` (orange) | Overlay/debug mode |
| CONTEXTUAL | ★ | `#06b6d4` (cyan) | Node-centered subgraph |
| QUERY | ⌘ | `#64748b` (slate) | Custom Cypher query |

---

## Schema Explorer Panel

Le panel de filtres s'étend pour montrer TOUS les types de nodes et relations (60 nodes + 114 arcs), pas seulement ceux du résultat actuel.

### Architecture: Expandable Filter Bar

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  EXPANDABLE FILTER BAR                                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  COLLAPSED (default):                                                       │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ [DATA ▼] ⊛ 50 nodes │ ⤴ 49 rels │ [▼ Show All Types]              │   │
│  │─────────────────────────────────────────────────────────────────────│   │
│  │ Entity(46) Project(1) Page(12) Block(8) Brand(1) ...               │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  EXPANDED (click "Show All Types"):                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ [DATA ▼] ⊛ 50 nodes │ ⤴ 49 rels │ [▲ Hide Types]  🔍 Filter...    │   │
│  │─────────────────────────────────────────────────────────────────────│   │
│  │ ▼ SHARED (39 types)                                     [Select All]│   │
│  │   ▼ config (3)                                                      │   │
│  │     ┌────────────────┐ ┌──────────┐ ┌─────────────────┐            │   │
│  │     │EntityCategory 0│ │ Locale 0 │ │SEOKeywordFormat 0│            │   │
│  │     └┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┘ └┄┄┄┄┄┄┄┄┄┄┘ └┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┘            │   │
│  │   ▼ locale (6)                                                      │   │
│  │     Culture(0) Style(0) Formatting(0) ...                          │   │
│  │   ▼ geography (6)                                                   │   │
│  │     Continent(7) GeoRegion(25) Country(195) ...                    │   │
│  │   ▼ knowledge (24)                                                  │   │
│  │     Term(2400) Expression(890) Pattern(156) ...                    │   │
│  │                                                                     │   │
│  │ ▼ ORG (21 types)                                        [Select All]│   │
│  │   ▼ config (1)                                                      │   │
│  │     ┌───────────┐                                                   │   │
│  │     │ OrgConfig 1│                                                  │   │
│  │     └───────────┘                                                   │   │
│  │   ▼ foundation (3)                                                  │   │
│  │     ┌──────────┐ ┌───────────────┐ ┌──────────────┐                │   │
│  │     │Project  1│ │ProjectContent 2│ │BrandIdentity 1│                │   │
│  │     └──────────┘ └───────────────┘ └──────────────┘                │   │
│  │   ▼ semantic (4)                                                    │   │
│  │     ┌──────────┐ ┌──────────────┐ ┌────────────┐ ┌─────────────┐   │   │
│  │     │Entity  46│ │EntityContent 92│ │   Thing  0  │ │ Category  0 │   │   │
│  │     └──────────┘ └──────────────┘ └┄┄┄┄┄┄┄┄┄┄┄┄┘ └┄┄┄┄┄┄┄┄┄┄┄┄┄┘   │   │
│  │   ...                                                               │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Chip States

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CHIP VISUAL STATES                                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────┐   ACTIVE (in query + visible)                                │
│  │ Entity ● │   → Background: layer color (semantic = #22c55e)             │
│  │    46    │   → Solid border                                              │
│  └──────────┘   → Click: exclude from query                                │
│                                                                             │
│  ┌──────────┐   INACTIVE (excluded from query)                             │
│  │ Project  │   → Background: gray                                         │
│  │   1  ○   │   → Strikethrough text                                       │
│  └──────────┘   → Click: re-include in query                               │
│                                                                             │
│  ┌┄┄┄┄┄┄┄┄┄┄┐   EMPTY (exists in DB but 0 in current view)                 │
│  │ Locale   │   → Dashed border                                            │
│  │   0  ◌   │   → Transparent background                                   │
│  └┄┄┄┄┄┄┄┄┄┄┘   → Click: add to query with OPTIONAL MATCH                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Interactions

| Action | Behavior |
|--------|----------|
| Left Click | Toggle active/inactive (modifies Cypher, re-executes) |
| Shift+Click | Solo mode (show ONLY this type) |
| Right Click | Context menu (show all, hide all, etc.) |
| Double Click | Execute view for this type (e.g., "All Entities") |
| Hover | Highlight nodes of this type in graph |

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Tab | Navigate between chips |
| Space | Toggle selected chip |
| A | Select All |
| N | Select None |
| I | Invert selection |
| / | Focus search bar |

### Data Flow: Schema Discovery

```typescript
// 1. FETCH SCHEMA (once at boot)
const schemaQuery = `
  CALL db.labels() YIELD label
  WITH collect(label) AS nodeLabels
  CALL db.relationshipTypes() YIELD relationshipType
  WITH nodeLabels, collect(relationshipType) AS relTypes
  RETURN nodeLabels, relTypes
`;
// Result: { nodeLabels: [...60], relTypes: [...114] }

// 2. MERGE WITH _registry.yaml METADATA
// Add colors, icons, layer info from YAML
const enrichedSchema = mergeWithRegistry(dbSchema, yamlRegistry);

// 3. COUNT CURRENT VIEW (after each query)
const counts = countByLabel(queryResults.nodes);
// { Entity: 46, Project: 1, Page: 12, ... }

// 4. RENDER CHIPS
enrichedSchema.forEach(type => {
  const count = counts[type.name] || 0;
  const state = count > 0 ? 'active' : 'empty';
  renderChip(type, count, state);
});
```

### Click on Empty Chip (0 count)

Clicking an "empty" chip (type exists in DB but not in current query result) adds it to the query:

```
Before click (Entity chip shows 0):
  Current query: MATCH (p:Page)-[:HAS_BLOCK]->(b:Block) RETURN p, b

After click:
  Modified query:
    MATCH (p:Page)-[:HAS_BLOCK]->(b:Block)
    OPTIONAL MATCH (b)-[:USES_ENTITY]->(e:Entity)   // ← ADDED
    RETURN p, b, e

  Result: Now shows Entity nodes connected to those Blocks
  Chip updates: Entity (12) with solid border
```

---

## ViewPicker Modal

### Layout Without Node Selected

```
╔═══════════════════════════════════════════════════════════════════╗
║  📊 VIEWS                                            [×] Close    ║
╠═══════════════════════════════════════════════════════════════════╣
║  🔍 Search views...                                               ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  ◇ META (3)                                                       ║
║  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐        ║
║  │ 🔷 Complete    │ │ 🏛️ Realm       │ │ 🔀 Arcs        │        ║
║  │ Full meta-graph│ │ Filter by realm│ │ Arc families   │        ║
║  └────────────────┘ └────────────────┘ └────────────────┘        ║
║                                                                   ║
║  ⊛ DATA (10)                                                      ║
║  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐        ║
║  │ 🌐 Complete    │ │ 🏢 Org Realm   │ │ 🌍 Shared Realm│        ║
║  │ All instances  │ │ Business nodes │ │ Universal nodes│        ║
║  └────────────────┘ └────────────────┘ └────────────────┘        ║
║  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐ ...    ║
║  │ 🗺️ Geography   │ │ 🏗️ Project     │ │ 🔗 Entities    │        ║
║  └────────────────┘ └────────────────┘ └────────────────┘        ║
║                                                                   ║
║  ◈ OVERLAY (3)                                                    ║
║  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐        ║
║  │ 🔶 Complete    │ │ 🔷 Kind→Inst   │ │ 🔀 Arc Analysis│        ║
║  └────────────────┘ └────────────────┘ └────────────────┘        ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

### Layout With Node Selected

```
╔═══════════════════════════════════════════════════════════════════╗
║  📊 VIEWS FOR: Entity "qr-code"                      [×] Close    ║
╠═══════════════════════════════════════════════════════════════════╣
║  🔍 Search views...                                               ║
║───────────────────────────────────────────────────────────────────║
║                                                                   ║
║  ★ CONTEXTUAL (6 matching)                                        ║
║  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐        ║
║  │ 🔗 Entities    │ │ 🌍 Locales     │ │ 🏷️ Categories  │        ║
║  │ Related links  │ │ Locale coverage│ │ Classification │        ║
║  └────────────────┘ └────────────────┘ └────────────────┘        ║
║  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐        ║
║  │ 🎯 SEO Intel   │ │ 🔮 GEO Intel   │ │ 🌐 Full Context│        ║
║  │ Keywords/clust.│ │ AI answers    │ │ All relations  │        ║
║  └────────────────┘ └────────────────┘ └────────────────┘        ║
║                                                                   ║
║───────────────────────────────────────────────────────────────────║
║  🔗 Voir toutes les vues (29)                                     ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

### View Card Component

```
┌────────────────────────────────────────┐
│ 🔗 Entities                            │  ← Icon + Title
│ Related entity links and categories   │  ← Description
│                                        │
│ ┌────────────────────────────────────┐ │
│ │ Entity ──▶ EntityContent           │ │  ← Mini ASCII preview
│ │    └────▶ EntityCategory           │ │
│ └────────────────────────────────────┘ │
│                                        │
│ semantic • org               [Ctrl+E] │  ← Layer + Realm + Shortcut
└────────────────────────────────────────┘

STATES:
├── Default:   bg-slate-800, border-slate-700
├── Hover:     bg-slate-700, border-[layer-color]
├── Selected:  bg-[layer-color]/20, border-[layer-color]
└── Disabled:  opacity-50, cursor-not-allowed
```

### Keyboard Navigation

| Key | Action |
|-----|--------|
| ↑↓←→ | Navigate between view cards |
| Enter | Execute selected view |
| Ctrl+Enter | Load query without executing (edit mode) |
| / | Focus search |
| Esc | Close modal (or clear search if focused) |
| M | Jump to META section |
| D | Jump to DATA section |
| O | Jump to OVERLAY section |
| C | Jump to CONTEXTUAL section |
| 1-9 | Quick select view 1-9 in current section |
| Tab | Cycle between sections |

---

## View Parameters

Certaines vues acceptent des paramètres dynamiques.

### YAML Schema

```yaml
id: meta-realm
description: Kinds d'un realm spécifique
params:
  - name: realm
    type: enum
    values: [shared, org]
    default: org
    label: "Realm"
cypher: |
  MATCH (r:Meta:Realm {key: $realm})-[:HAS_LAYER]->(l)
  ...
```

### Parameter Types

| Type | Description | UI |
|------|-------------|-----|
| `enum` | Predefined values | Dropdown |
| `string` | Free text | Input field |
| `number` | Numeric (min/max) | Number input |
| `boolean` | On/off | Toggle switch |
| `nodeKey` | Auto-filled from selection | Hidden (auto) |

### UI Flow for Views with Parameters

```
1. User clicks view with params
   ↓
2. IF has required params without defaults
   → Show inline param inputs in card
   ┌────────────────────────────────────┐
   │ 🏛️ Realm View                      │
   │ Kinds d'un realm spécifique       │
   │                                    │
   │ Realm: [shared ▼]                 │  ← Param input
   │                                    │
   │           [Execute View]          │
   └────────────────────────────────────┘
   ↓
3. User fills params + clicks Execute
   ↓
4. executeView(viewId, { realm: 'shared' })
```

### Special Parameter: $nodeKey

Contextual views use `$nodeKey` auto-filled from selected node:

```cypher
MATCH (root {key: $nodeKey}) ...
```

- If node selected: `$nodeKey = node.key` (auto)
- If no node selected: view disabled OR prompts to select

---

## QueryPill Integration

Le QueryPill affiche et permet d'éditer la requête Cypher courante.

### Collapsed State (Default)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [⊛ DATA] ctx-entities │ MATCH (root {key: "qr-code"})... │ [▶] [✎]        │
└─────────────────────────────────────────────────────────────────────────────┘
     ↑           ↑                    ↑                        ↑    ↑
   Mode       View ID          Truncated Cypher              Run  Edit
```

### Expanded State (Click ✎)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [⊛ DATA] ctx-entities                                     [▶] [✎] [×]     │
│───────────────────────────────────────────────────────────────────────────│
│ MATCH (root {key: "qr-code"})                                             │
│ WHERE root:Entity                                                         │
│ OPTIONAL MATCH (root)-[:HAS_CONTENT]->(ec:EntityContent)                 │
│ OPTIONAL MATCH (root)-[:BELONGS_TO]->(cat:EntityCategory)                │
│ RETURN root, ec, cat                                                      │
│ LIMIT 100                                                                 │
│                                                                           │
│ ┌─────────────────────────────────────────────────────────────────┐      │
│ │ [Monaco Editor with Cypher syntax highlighting]                 │      │
│ └─────────────────────────────────────────────────────────────────┘      │
│                                                                           │
│ Params: nodeKey="qr-code"                              [Reset] [Run]     │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Edit Detection

When user edits the Cypher:

1. Badge changes: `[⊛ DATA] ctx-entities` → `[⌘ QUERY] (modified)`
2. View ID shows: `"ctx-entities (modified)"`
3. Reset button appears to restore original view query

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [⌘ QUERY] ctx-entities (modified)                    [Reset] [▶] [✎]      │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Interactions

| Action | Result |
|--------|--------|
| Click badge | Open ViewPicker modal |
| Click view ID | Show view details popover |
| Click ▶ (Run) | Execute current query |
| Click ✎ (Edit) | Expand QueryPill to edit mode |
| Click Reset | Restore original view query |
| Ctrl+Enter in editor | Execute modified query |
| Esc in editor | Collapse back to pill |

---

## TUI Integration

The TUI "Atlas" mode becomes "View" mode with the same unified system.

### Main Layout (3 Panels)

```
┌──────────────────┐┌──────────────────────────┐┌──────────────────────┐
│ VIEW PICKER      ││ GRAPH (ASCII)            ││ NODE DETAIL          │
│                  ││                          ││                      │
│ ▼ META (3)       ││   Project                ││ Entity: qr-code      │
│   ◇ complete     ││      │                   ││ ──────────────────── │
│   ◇ realm        ││      ├──▶ Page           ││ realm: org           │
│   ◇ arcs         ││      │     │             ││ layer: semantic      │
│                  ││      │     └──▶ Block    ││ trait: invariant     │
│ ▼ DATA (10)      ││      │                   ││                      │
│   ⊛ complete     ││      └──▶ Entity ◀───────││ ★ VIEWS (6)          │
│   ⊛ org          ││            │             ││   1. ctx-entities    │
│   ⊛ shared       ││            └──▶ Content  ││   2. ctx-locales     │
│ ► ⊛ entities ◀───││──────────────────────────││   3. ctx-categories  │
│   ...            ││                          ││   4. ctx-seo-intel   │
│                  ││                          ││   ...                │
│ ▶ OVERLAY (3)    ││                          ││                      │
│ ▶ CONTEXTUAL (13)││                          ││ [Enter] Execute      │
│                  ││                          ││ [e] Edit query       │
└──────────────────┘└──────────────────────────┘└──────────────────────┘
┌──────────────────────────────────────────────────────────────────────┐
│ [⊛ DATA] entities │ 8 nodes │ 12 arcs │ MATCH (e:Entity)... │ [Tab] │
└──────────────────────────────────────────────────────────────────────┘
```

### TUI Schema Explorer ([f] Toggle)

```
┌──────────────────────────────────────────────────────────────────────┐
│ SCHEMA EXPLORER                                          [f] close  │
│ /search...                                                          │
│──────────────────────────────────────────────────────────────────────│
│                                                                      │
│ ▼ SHARED (39 types)                                      [a] all    │
│   ▼ config (3)                                                      │
│     [●] EntityCategory ···· 0   ← dashed = 0 in view               │
│     [●] Locale ············ 0                                       │
│     [●] SEOKeywordFormat ·· 0                                       │
│   ▼ locale (6)                                                      │
│     [●] Culture ··········· 0                                       │
│     [●] Style ············· 0                                       │
│   ▼ knowledge (24)                                                  │
│     [●] Term ··········· 2400   ← solid = in current view          │
│     [●] Expression ······ 890                                       │
│                                                                      │
│ ▼ ORG (21 types)                                         [a] all    │
│   ▼ semantic (4)                                                    │
│     [●] Entity ·········· 46   ← highlighted = in view             │
│     [○] EntityContent ··· 92   ← [○] = excluded by user            │
│     [●] Thing ············ 0                                        │
│                                                                      │
│──────────────────────────────────────────────────────────────────────│
│ [Space] toggle │ [a] all │ [n] none │ [i] invert │ [Enter] apply   │
└──────────────────────────────────────────────────────────────────────┘

VISUAL ENCODING:
├── [●] Active (included in query)
├── [○] Inactive (excluded by user)
├── Solid dots ···· = has nodes in current view
├── No dots (dashed) = 0 nodes in view (but exists in DB)
└── Highlighted = currently selected for toggle
```

### TUI Query Editor ([e] to Open)

```
┌──────────────────────────────────────────────────────────────────────┐
│ QUERY EDITOR                                   [⊛ DATA] ctx-entities │
│──────────────────────────────────────────────────────────────────────│
│                                                                      │
│  1 │ MATCH (root {key: $nodeKey})                                   │
│  2 │ WHERE root:Entity                                              │
│  3 │ OPTIONAL MATCH (root)-[:HAS_CONTENT]->(ec:EntityContent)       │
│  4 │ OPTIONAL MATCH (root)-[:BELONGS_TO]->(cat:EntityCategory)      │
│  5 │ OPTIONAL MATCH (ec)-[:FOR_LOCALE]->(locale:Locale)             │
│  6 │ WITH root, collect(DISTINCT ec) AS contents,                   │
│  7 │      collect(DISTINCT cat) AS categories,                      │
│  8 │      collect(DISTINCT locale) AS locales                       │
│  9 │ RETURN root, contents, categories, locales                     │
│ 10 │ LIMIT 100                                                      │
│                                                                      │
│──────────────────────────────────────────────────────────────────────│
│ Params: nodeKey = "qr-code"                                         │
│──────────────────────────────────────────────────────────────────────│
│ [Ctrl+Enter] Execute │ [Esc] Cancel │ [Ctrl+R] Reset to original    │
└──────────────────────────────────────────────────────────────────────┘

FEATURES:
├── Line numbers
├── Syntax highlighting (MATCH, WHERE, RETURN in colors)
├── Params shown below editor
├── vi-style navigation (hjkl, gg, G, etc.)
└── Modified indicator if query changed from original view
```

### TUI Keyboard Reference

| Category | Key | Action |
|----------|-----|--------|
| **Navigation** | 1/2/3/4 | Switch to META/DATA/OVERLAY/CONTEXTUAL |
| | Tab | Cycle focus: ViewPicker → Graph → NodeDetail |
| | j/k | Navigate up/down in lists |
| | h/l | Collapse/expand tree nodes |
| | Enter | Execute selected view |
| | Esc | Close overlay / deselect |
| **View Picker** | / | Search views |
| | m/d/o/c | Jump to META/DATA/OVERLAY/CONTEXTUAL |
| **Graph** | hjkl | Pan graph |
| | +/- | Zoom in/out |
| | 0 | Reset zoom |
| | Space | Select node under cursor |
| **Filters** | f | Toggle Schema Explorer overlay |
| | Space | Toggle selected type (in filter panel) |
| | a/n/i | Select all / none / invert |
| **Query** | e | Open Query Editor |
| | Ctrl+e | Execute current query |
| | Ctrl+r | Reset to original view query |
| **Global** | ? | Show help overlay |
| | q | Quit |
| | : | Command palette |

### TUI Status Bar

```
┌──────────────────────────────────────────────────────────────────────┐
│ [◇ META] complete │ 196 nodes │ 312 arcs │ MATCH (n:Meta)... │ ?:help│
└──────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────┐
│ [⊛ DATA] entities │ 8 nodes │ 12 arcs │ Entity: qr-code │ f:filter  │
└──────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────┐
│ [★ CTX] ctx-entities │ 8 nodes │ 12 arcs │ (modified) │ Ctrl+E:run  │
└──────────────────────────────────────────────────────────────────────┘

COLOR CODING (same as Studio):
├── ◇ META      → violet
├── ⊛ DATA      → teal
├── ◈ OVERLAY   → orange
├── ★ CTX       → cyan
└── ⌘ QUERY     → gray (custom/modified)
```

---

## References

- ADR-021: Query-First Architecture
- `packages/core/models/views/_registry.yaml`
- `apps/studio/src/stores/viewStore.ts`
