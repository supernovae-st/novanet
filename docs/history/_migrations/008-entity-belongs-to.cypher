// =============================================================================
// MIGRATION 008: Entity.type -> BELONGS_TO arcs
// =============================================================================
// Version: v11.1
// Date: 2025-02-09
//
// Converts Entity.type property to [:BELONGS_TO]->(:EntityCategory) arcs.
// This replaces a string property with a proper graph relationship for:
// - Better querying: MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory)
// - Referential integrity: Categories must exist before linking
// - LLM context: Arc has llm_context explaining the semantic relationship
//
// The script is idempotent - safe to run multiple times.
//
// RUN: docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < migrations/008-entity-belongs-to.cypher
// =============================================================================

// ---------------------------------------------------------------------------
// STEP 1: Pre-migration validation
// ---------------------------------------------------------------------------
// Check that all Entity.type values have matching EntityCategory nodes

MATCH (e:Entity)
WHERE e.type IS NOT NULL
WITH e.type AS type, count(*) AS entity_count
OPTIONAL MATCH (c:EntityCategory {key: type})
RETURN type, entity_count, c IS NOT NULL AS category_exists
ORDER BY type;

// ---------------------------------------------------------------------------
// STEP 2: Create BELONGS_TO arcs from Entity.type property
// ---------------------------------------------------------------------------
// Only creates arcs where:
// - Entity has a type property
// - Matching EntityCategory exists
// - Arc doesn't already exist (idempotent)

MATCH (e:Entity)
WHERE e.type IS NOT NULL
MATCH (c:EntityCategory {key: e.type})
WHERE NOT (e)-[:BELONGS_TO]->(c)
MERGE (e)-[:BELONGS_TO]->(c);

// ---------------------------------------------------------------------------
// STEP 3: Verify no orphaned entities
// ---------------------------------------------------------------------------
// All entities should now have a BELONGS_TO relationship
// Expected result: 0 orphaned entities

MATCH (e:Entity)
WHERE e.type IS NOT NULL
  AND NOT (e)-[:BELONGS_TO]->(:EntityCategory)
RETURN count(e) AS orphaned_entities,
       collect(e.key)[0..5] AS sample_orphans;
// Expected: orphaned_entities = 0

// ---------------------------------------------------------------------------
// STEP 4: Remove type property from Entity nodes
// ---------------------------------------------------------------------------
// Only remove after confirming BELONGS_TO arcs exist

MATCH (e:Entity)
WHERE e.type IS NOT NULL
  AND (e)-[:BELONGS_TO]->(:EntityCategory)
REMOVE e.type
RETURN count(e) AS entities_cleaned;

// ---------------------------------------------------------------------------
// STEP 5: Final verification - Category distribution
// ---------------------------------------------------------------------------
// Shows entity count per category, ordered by sort_order

MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory)
RETURN c.key AS category,
       c.display_name AS display_name,
       c.question AS question,
       count(e) AS entity_count,
       c.sort_order AS sort_order
ORDER BY sort_order;

// ---------------------------------------------------------------------------
// STEP 6: Verify no remaining type properties
// ---------------------------------------------------------------------------

MATCH (e:Entity)
WHERE e.type IS NOT NULL
RETURN count(e) AS entities_with_type_property;
// Expected: 0
