---
paths:
  - "packages/db/seed/**/*.cypher"
  - "packages/db/migrations/**/*.cypher"
---

# Cypher/Neo4j Rules

## Schema-Graph Navigation (v12)
```cypher
// Taxonomy hierarchy
MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[:HAS_CLASS]->(c:Class)

// Class context assembly
MATCH (c:Class {label: $class})
MATCH (c)-[:IN_REALM]->(r:Realm)
MATCH (c)-[:IN_LAYER]->(l:Layer)
MATCH (c)-[:HAS_TRAIT]->(t:Trait)

// Instance bridge (data → schema)
MATCH (n:Page)-[:OF_CLASS]->(c:Class {label: 'Page'})

// Arc class navigation
MATCH (ac:ArcClass)-[:FROM_CLASS]->(source:Class)
MATCH (ac)-[:TO_CLASS]->(target:Class)
```

## Constraints & Indexes
- All node types need unique constraint on `key`
- Schema-nodes have `:Schema` double-label (v12: was `:Meta`)
- Use APOC for complex operations

## Seed File Conventions
- Files numbered: `00-constraints.cypher`, `00.5-taxonomy.cypher`, `01-classes.cypher`...
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
