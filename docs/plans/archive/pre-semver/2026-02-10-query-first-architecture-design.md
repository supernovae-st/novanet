# Query-First Architecture Design

> **Date:** 2026-02-10
> **Status:** Approved
> **Author:** Thibaut + Claude

## Overview

Refactor NovaNet Studio to use a **Query-First Architecture** where:
- **Cypher Query = Source of Truth** - Graph always displays the result of the executed Cypher query
- **YAML Views = Single Definition Source** - All views defined in YAML files, no hardcoded queries
- **Auto-Execute with Edit Option** - Views auto-execute on select, Ctrl+click to load without executing

## Architecture

### YAML Structure

```
packages/core/models/views/
├── _registry.yaml          # Index of all views
├── global/                  # Global views (full graph)
│   ├── complete-graph.yaml
│   ├── shared-layer.yaml
│   └── project-layer.yaml
├── contextual/              # Contextual views (node-specific)
│   ├── composition.yaml     # Page/Block hierarchy
│   ├── knowledge.yaml       # Locale knowledge atoms
│   ├── entities.yaml        # Entity connections
│   ├── geographic.yaml      # Country → Region → Continent
│   ├── generation.yaml      # Generation pipeline
│   └── seo-intel.yaml       # SEO keywords & metrics
└── generation/              # AI agent context views
    ├── block-generation.yaml
    └── page-generation-context.yaml
```

### YAML View Schema

```yaml
id: composition
description: Page/Block composition hierarchy
category: contextual         # global | contextual | generation | mining
contextual: true             # appears in node sidebar
applicable_types: [Page, Block]  # compatible node types
modes: [data, meta, overlay, query]
cypher: |
  MATCH (root {key: $nodeKey})
  WHERE root:Page OR root:Block
  OPTIONAL MATCH path = (root)-[:HAS_BLOCK*1..3]->(block:Block)
  WITH root, collect(DISTINCT block) AS blocks
  UNWIND ([root] + blocks) AS n
  WITH collect(DISTINCT n) AS nodes
  UNWIND nodes AS n
  OPTIONAL MATCH (n)-[r:HAS_BLOCK]->(m)
  WHERE m IN nodes
  RETURN nodes, collect(DISTINCT r) AS relationships
```

## Data Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ ViewPicker  │───▶│  viewStore  │───▶│ /api/views  │───▶│   Neo4j     │
│ (Select)    │    │ executeView │    │ /:id/query  │    │  (Cypher)   │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                  │                  │                  │
       ▼                  ▼                  ▼                  ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  QueryPill  │◀───│ queryStore  │◀───│    YAML     │◀───│   Results   │
│ (Display)   │    │ setQuery()  │    │   cypher    │    │ nodes/edges │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

### Interactions

| Action | Behavior |
|--------|----------|
| Click view | `executeView()` → auto-run query → update graph |
| Ctrl+Click view | `loadQueryOnly()` → QueryPill shows query (no execute) |
| Edit QueryPill | Manual changes → click ▶️ to run |
| Context view card | `executeView()` with `nodeKey` param |

### Stores

| Store | Responsibility |
|-------|----------------|
| `viewStore` | Active view, params, categories |
| `queryStore` | Current cypher, execution state, results |
| `graphStore` | Nodes, edges (populated by query results) |

## UI Components

### ViewPicker (refactored)

- Loads registry on mount (`loadRegistry`)
- Displays ALL views (global + contextual + generation)
- Grouped by category with colored badges
- Click = auto-execute
- Ctrl+Click = load query only
- Tooltip "Ctrl+click to edit before running"

### QueryPill (enhanced)

- Always displays the source query of current graph
- Badge "VIEW: composition" when query comes from a view
- Editable (expand mode)
- ▶️ Run button
- Bidirectional sync with viewStore

### Node Sidebar (Context Views)

- Filters views where `contextual: true`
- Filters by `applicable_types` (matches `node.type`)
- Click = `executeView` with `nodeKey`
- Same behavior as ViewPicker (auto-execute)

### Visual Indicators

- QueryPill green glow during execution
- StatsCounter updates after each query
- View badge on QueryPill when view is active

## Implementation Plan

### Phase 1: Immediate Fix (10 min)

- [ ] Add `loadRegistry()` to ViewPicker mount
- [ ] Verify 12 existing YAML views display

### Phase 2: Migrate viewQueries.ts → YAML (30 min)

Create `contextual/*.yaml` for each view in viewQueries.ts:

- [ ] `composition.yaml` - Page/Block hierarchy
- [ ] `knowledge.yaml` - Locale knowledge atoms
- [ ] `geographic.yaml` - Country → Region → Continent
- [ ] `entities.yaml` - Entity connections
- [ ] `seo-intel.yaml` - SEO keywords & metrics
- [ ] `geo-intel.yaml` - GEO queries & answers
- [ ] `generation.yaml` - Generation pipeline
- [ ] `categories.yaml` - Entity categories
- [ ] `cross-realm.yaml` - Cross-realm connections
- [ ] `locales.yaml` - Locale coverage
- [ ] `content.yaml` - Content per locale
- [ ] `metrics.yaml` - Performance metrics

- [ ] Update `_registry.yaml` with new views
- [ ] Deprecate/remove `viewQueries.ts`

### Phase 3: Auto-execute + Ctrl+click (20 min)

- [ ] ViewPicker: `onClick` → `executeView()`
- [ ] ViewPicker: `onCtrlClick` → `loadQueryOnly()`
- [ ] viewStore: Add `loadQueryOnly` action
- [ ] QueryPill: Display active view badge

### Phase 4: Node Sidebar Integration (15 min)

- [ ] ViewSelector uses same registry
- [ ] Filter `contextual: true` + `applicable_types`
- [ ] Pass `nodeKey` as param to executeView

## Files to Modify

| File | Changes |
|------|---------|
| `ViewPicker.tsx` | Add loadRegistry + auto-execute logic |
| `viewStore.ts` | Add loadQueryOnly action |
| `QueryPill.tsx` | Add view badge display |
| `ViewLoader.ts` | Load contextual views from new paths |
| `_registry.yaml` | Add contextual view entries |
| `contextual/*.yaml` | New files (12 views) |

## Success Criteria

1. ViewPicker shows all views (should be ~24 total after migration)
2. Clicking a view auto-executes and displays results
3. Ctrl+clicking loads query without executing
4. QueryPill shows which view is active
5. Node sidebar context views work with same system
6. 2D and 3D modes show identical data (same query results)
