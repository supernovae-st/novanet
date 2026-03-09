// ═══════════════════════════════════════════════════════════════════════════════
// 069-fix-schema-data-drift.cypher
// Fix arc naming mismatches between YAML schema and Neo4j data
// v0.17.3 - Schema-Data Alignment (ADR-036)
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// PHASE 1: Delete EXHIBITS arcs (trait system removed in ADR-036)
// ─────────────────────────────────────────────────────────────────────────────────

// These arcs linked Class nodes to Trait nodes, now obsolete
MATCH ()-[r:EXHIBITS]->()
DELETE r
RETURN count(*) AS exhibits_arcs_deleted;

// ─────────────────────────────────────────────────────────────────────────────────
// PHASE 2: Rename TARGETS to TARGETS_KEYWORD
// ─────────────────────────────────────────────────────────────────────────────────

// The schema defines TARGETS_KEYWORD, but data used TARGETS
MATCH (a)-[r:TARGETS]->(k:SEOKeyword)
WITH a, k, properties(r) AS props
CREATE (a)-[r2:TARGETS_KEYWORD]->(k)
SET r2 = props
WITH a, k
MATCH (a)-[r:TARGETS]->(k)
DELETE r
RETURN count(*) AS targets_renamed_to_targets_keyword;

// ─────────────────────────────────────────────────────────────────────────────────
// PHASE 3: Note TARGETS_ENTITY - requires new arc definition
// ─────────────────────────────────────────────────────────────────────────────────

// TARGETS_ENTITY: SEOKeyword -> Entity (57 relationships)
// This arc indicates which Entity a keyword is about (semantic link)
// Action: Either add arc definition or rename to semantic arc
// For now, documenting the issue. Manual review needed.

// Count TARGETS_ENTITY for verification
MATCH ()-[r:TARGETS_ENTITY]->()
RETURN count(r) AS targets_entity_count;

// ─────────────────────────────────────────────────────────────────────────────────
// PHASE 4: Note OF_CATEGORY - requires new arc definition
// ─────────────────────────────────────────────────────────────────────────────────

// OF_CATEGORY: Entity -> EntityCategory (9 relationships)
// This is category membership - should have arc definition
// For now, documenting the issue. Manual review needed.

// Count OF_CATEGORY for verification
MATCH ()-[r:OF_CATEGORY]->()
RETURN count(r) AS of_category_count;

// ─────────────────────────────────────────────────────────────────────────────────
// VERIFICATION: Check schema metadata arcs (expected, internal)
// ─────────────────────────────────────────────────────────────────────────────────

// These are internal arcs used by schema system, not in YAML:
// HAS_ARC_CLASS, HAS_CLASS, HAS_LAYER, IN_FAMILY, IN_LAYER, IN_REALM
// They are expected and don't need action.

MATCH ()-[r]->()
WHERE type(r) IN ['HAS_ARC_CLASS', 'HAS_CLASS', 'HAS_LAYER', 'IN_FAMILY', 'IN_LAYER', 'IN_REALM']
RETURN type(r) AS schema_arc, count(r) AS count
ORDER BY type(r);
