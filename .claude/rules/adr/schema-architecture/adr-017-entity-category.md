---
id: "017"
title: "EntityCategory Classification"
version: "v11.1"
status: "active"
domain: "schema-architecture"
updated: "v11.2"
---

# ADR-017: EntityCategory Classification

**Status**: Approved (v11.1, updated v11.2 for realm renames)

**Decision**: Replace Entity.type enum property with EntityCategory nodes and BELONGS_TO arcs.

**Problem**:
Entity had a `type` enum property with 13 hardcoded values (THING, CONTENT_TYPE, PLACE, PERSON, ORGANIZATION, EVENT, CONCEPT, PRODUCT, SERVICE, RESOURCE, MEDIA, DOCUMENT, ABSTRACT).
Properties are difficult to query and extend. Moving classification to the graph enables queryable, extensible categorization.

**Solution**:
- Create EntityCategory node type in `shared/config` layer (13 nodes, invariant trait)
- Add BELONGS_TO arc from Entity (org/semantic) to EntityCategory (shared/config) - cross_realm, ownership family
- Remove Entity.type enum property

**Structure**:
```
EntityCategory (shared/config, invariant, 13 nodes)
  |- category_key: "thing"
  |- category_key: "content-type"
  |- ... (11 more)

Entity (org/semantic) -[:BELONGS_TO]-> EntityCategory (shared/config)
```

**Arc Properties**:
- Name: `BELONGS_TO`
- Family: `ownership`
- Scope: `cross_realm` (org/semantic -> shared/config)
- Cardinality: `many_to_one` (many Entities can belong to one category)
- Source: Entity
- Target: EntityCategory

**Benefits**:
1. **Queryable**: Find all entities by category with `MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory {category_key: 'person'})`
2. **Extensible**: Add new categories without code changes (just YAML + Neo4j nodes)
3. **Uniform**: Classification follows ADR-006 (realm differentiates scope) - universal categories in shared, instance relationships in org
4. **Graph-native**: Classification is now part of the knowledge graph, not a buried enum property

**Migration**:
1. Create EntityCategory YAML definition in `packages/core/models/node-classes/shared/config/entity-category.yaml`
2. Create BELONGS_TO arc definition in `packages/core/models/arc-classes/ownership/belongs-to.yaml`
3. Generate schema artifacts: `cargo run -- schema generate`
4. Create Neo4j migration to insert 13 EntityCategory nodes and create BELONGS_TO relationships from existing Entity nodes
5. Remove Entity.type property from Entity node definition

**No breaking changes** - API clients can still categorize entities, just through graph traversal instead of property lookup.
