---
paths:
  - "packages/db/seed/**/*.cypher"
  - "packages/db/migrations/**/*.cypher"
---

# Cypher/Neo4j Rules

## Meta-Graph Navigation (v9)
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
- Files numbered: `00-constraints.cypher`, `01-realms.cypher`...
- MERGE for idempotent operations
- Parameters use `$paramName` syntax

## Performance
- Use indexes for property lookups
- Limit `*..n` patterns to reasonable depth
- Profile with `EXPLAIN` before complex queries

## EdgeFamily Classification
| Family | Pattern | Example |
|--------|---------|---------|
| ownership | `HAS_*`, `*_OF` | `HAS_PAGE`, `PAGE_OF` |
| localization | `HAS_L10N`, `FOR_LOCALE` | `Concept-[:HAS_L10N]->ConceptL10n` |
| semantic | `USES_*`, `SEMANTIC_LINK` | `Block-[:USES_CONCEPT]->Concept` |
| generation | `HAS_OUTPUT` | `Page-[:HAS_OUTPUT]->PageL10n` |
| mining | `TARGETS_*` | `SEOMiningRun-[:TARGETS_SEO]->SEOKeyword` |
