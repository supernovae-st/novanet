---
paths:
  - "packages/db/seed/**/*.cypher"
  - "packages/db/migrations/**/*.cypher"
---

# Cypher/Neo4j Rules

## Meta-Graph Navigation (v10.9)
```cypher
// Taxonomy hierarchy
MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)

// Kind context assembly
MATCH (k:Kind {label: $kind})
MATCH (k)-[:IN_REALM]->(r:Realm)
MATCH (k)-[:IN_LAYER]->(l:Layer)
MATCH (k)-[:HAS_TRAIT]->(t:Trait)

// Instance bridge (data → meta)
MATCH (n:Page)-[:OF_KIND]->(k:Kind {label: 'Page'})
```

## Constraints & Indexes
- All node types need unique constraint on `key`
- Meta-nodes have `:Meta` double-label
- Use APOC for complex operations

## Seed File Conventions
- Files numbered: `00-constraints.cypher`, `00.5-taxonomy.cypher`, `01-kinds.cypher`...
- MERGE for idempotent operations
- Parameters use `$paramName` syntax

## Performance
- Use indexes for property lookups
- Limit `*..n` patterns to reasonable depth
- Profile with `EXPLAIN` before complex queries

## ArcFamily Classification (v10.9)
| Family | Pattern | Example |
|--------|---------|---------|
| ownership | `HAS_*`, `*_OF` | `HAS_PAGE`, `PAGE_OF` |
| localization | `HAS_CONTENT`, `FOR_LOCALE` | `Entity-[:HAS_CONTENT]->EntityContent` |
| semantic | `USES_*`, `SEMANTIC_LINK` | `Block-[:USES_ENTITY]->Entity` |
| generation | `HAS_GENERATED` | `Page-[:HAS_GENERATED]->PageGenerated` |
| mining | `TARGETS_*` | `SEOMiningRun-[:TARGETS_SEO]->SEOKeyword` |

## Composite Key Pattern (v10.9)

EntityContent uses a composite key combining entity and locale:

```cypher
// EntityContent composite key: entity:{entity_key}@{locale_key}
MATCH (e:Entity {key: 'qr-code'})
MATCH (ec:EntityContent {key: 'entity:qr-code@en-US'})
MATCH (e)-[:HAS_CONTENT]->(ec)

// Query all content for an entity
MATCH (e:Entity {key: $entityKey})
MATCH (e)-[:HAS_CONTENT]->(ec:EntityContent)
WHERE ec.key STARTS WITH 'entity:' + $entityKey + '@'
RETURN ec

// Query entity content for a locale
MATCH (ec:EntityContent)
WHERE ec.key ENDS WITH '@' + $localeKey
RETURN ec
```

## v10.9 Node Naming

| v10.8 (deprecated) | v10.9 (current) | Arc |
|--------------------|-----------------|-----|
| `EntityL10n` | `EntityContent` | `HAS_CONTENT` |
| `PageL10n` | `PageGenerated` | `HAS_GENERATED` |
| `BlockL10n` | `BlockGenerated` | `HAS_GENERATED` |
