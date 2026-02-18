// NovaNet Migration: Entity.type → BELONGS_TO arc (ADR-017 compliance)
// ============================================================================
//
// Migrates deprecated Entity.type property to proper graph model:
// - Creates BELONGS_TO arcs from Entity to EntityCategory
// - Removes deprecated `type` and `phase` properties from Entity nodes
//
// Must run after: 02.5-entity-categories.cypher, 10-entities-qrcode-ai.cypher
// ============================================================================

// =============================================================================
// STEP 1: Create BELONGS_TO arcs from Entity.type values
// =============================================================================

// For each Entity with a type property, create BELONGS_TO arc to matching EntityCategory
MATCH (e:Entity)
WHERE e.type IS NOT NULL
MATCH (ec:EntityCategory {key: e.type})
MERGE (e)-[:BELONGS_TO]->(ec);

// Verification: count created arcs
MATCH (e:Entity)-[r:BELONGS_TO]->(ec:EntityCategory)
RETURN count(r) AS belongs_to_count,
       count(DISTINCT e) AS entities_with_category,
       count(DISTINCT ec) AS categories_used;

// =============================================================================
// STEP 2: Remove deprecated properties
// =============================================================================

// Remove `type` property (now represented by BELONGS_TO arc)
MATCH (e:Entity)
WHERE e.type IS NOT NULL
REMOVE e.type;

// Remove `phase` property (organizational, not schema property)
MATCH (e:Entity)
WHERE e.phase IS NOT NULL
REMOVE e.phase;

// =============================================================================
// VERIFICATION
// =============================================================================

// Verify no Entity has type or phase properties
MATCH (e:Entity)
WHERE e.type IS NOT NULL OR e.phase IS NOT NULL
RETURN count(e) AS entities_with_deprecated_props;
// Expected: 0

// Verify all entities have BELONGS_TO arc
MATCH (e:Entity)
OPTIONAL MATCH (e)-[:BELONGS_TO]->(ec:EntityCategory)
WITH e, ec
WHERE ec IS NULL
RETURN count(e) AS entities_without_category;
// Expected: 0 (after all seeds run)
