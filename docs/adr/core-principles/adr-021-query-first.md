---
id: "021"
title: "Query-First Architecture"
version: "v11.6"
status: "active"
domain: "core-principles"
---

# ADR-021: Query-First Architecture

**Status**: Approved (v11.6)

**Problem**: NovaNet Studio had multiple sources of truth for graph visualization:
1. Hardcoded queries in `viewQueries.ts`
2. YAML view definitions in `packages/core/models/views/`
3. Ad-hoc Cypher queries from QueryPill
4. Mode-specific logic (data/meta/overlay) scattered across components

This caused:
- Inconsistent behavior between 2D and 3D views
- Difficulty understanding "what query produced this graph?"
- Duplicate query definitions (TypeScript + YAML)
- Complex state management across viewStore, queryStore, graphStore

**Decision**: Adopt **Query-First Architecture** where Cypher is the single source of truth.

## Core Principles

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  QUERY-FIRST ARCHITECTURE                                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. CYPHER QUERY = SOURCE OF TRUTH                                          │
│     └─ Graph always displays the result of the executed Cypher query        │
│     └─ No hidden state or mode-specific filtering                           │
│     └─ QueryPill shows the exact query that produced visible graph          │
│                                                                             │
│  2. YAML VIEWS = SINGLE DEFINITION SOURCE                                   │
│     └─ All views defined in packages/core/models/views/*.yaml               │
│     └─ No hardcoded queries in TypeScript                                   │
│     └─ Views are parameterized Cypher templates                             │
│                                                                             │
│  3. AUTO-EXECUTE WITH EDIT OPTION                                           │
│     └─ Click view → execute immediately → update graph                      │
│     └─ Ctrl+click → load query into QueryPill without executing             │
│     └─ Edit QueryPill → click ▶️ to run modified query                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
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

## SCHEMA Mode: CLASSES_QUERY + ARCS_QUERY

SCHEMA mode uses two foundational queries to build the schema graph:

```cypher
// CLASSES_QUERY: Fetch all NodeClass instances
MATCH (k:Class)
RETURN k.name AS name, k.realm AS realm, k.layer AS layer,
       k.trait AS trait, k.display_name AS display_name

// ARCS_QUERY: Fetch all ArcClass instances
MATCH (a:ArcClass)
RETURN a.name AS name, a.family AS family, a.scope AS scope,
       a.cardinality AS cardinality, a.source AS source, a.target AS target
```

These queries are executed by `cargo run -- blueprint` and populate the schema graph for class exploration.

## View Categories

| Category | Purpose | Example Views |
|----------|---------|---------------|
| `global` | Full graph exploration | complete-graph, shared-layer, project-layer |
| `contextual` | Node-specific subgraph | composition, knowledge, geographic |
| `generation` | AI agent context | block-generation, page-generation-context |
| `mining` | SEO/GEO intelligence | seo-intel, geo-intel |

## YAML View Schema

```yaml
id: composition
description: Page/Block composition hierarchy
category: contextual
contextual: true
applicable_types: [Page, Block]
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

## Benefits

1. **Debuggability**: QueryPill shows exact query → easy to understand/modify
2. **Consistency**: 2D and 3D views show identical data (same query results)
3. **Extensibility**: Add views by creating YAML files, no code changes
4. **Transparency**: No hidden mode logic, query is the complete specification
5. **Testability**: Views are pure Cypher, testable independently

## Impact

- `viewQueries.ts` deprecated (moved to YAML)
- ViewPicker loads from `_registry.yaml` on mount
- QueryPill displays active view badge
- All navigation modes (data/meta/overlay/query) use same view system

**Reference**: `docs/plans/2026-02-10-query-first-architecture-design.md`
