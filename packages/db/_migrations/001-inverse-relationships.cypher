// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 001: Create Inverse Relationships (v0.13.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
// This migration creates inverse relationships for bidirectional traversal.
// The script is idempotent - safe to run multiple times.
//
// v0.13.0 Updates (ADR-029 *Native pattern):
// - HAS_CONTENT/HAS_GENERATED → HAS_NATIVE (Entity/Page/Block → *Native)
// - CONTENT_OF/GENERATED_FOR → NATIVE_OF (*Native → Entity/Page/Block)
// - EntityContent → EntityNative
// - ProjectContent → ProjectNative
// - PageGenerated → PageNative
// - BlockGenerated → BlockNative
//
// RUN: docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < migrations/001-inverse-relationships.cypher
// ═══════════════════════════════════════════════════════════════════════════════

// ---------------------------------------------------------------------------
// NATIVE_OF from HAS_NATIVE (v0.13.0)
// EntityNative/ProjectNative/PageNative/BlockNative → Entity/Project/Page/Block
// ---------------------------------------------------------------------------
MATCH (parent)-[:HAS_NATIVE]->(native)
WHERE NOT (native)-[:NATIVE_OF]->(parent)
MERGE (native)-[:NATIVE_OF]->(parent);

// ---------------------------------------------------------------------------
// BLOCK_OF from HAS_BLOCK
// Block → Page (with position property)
// ---------------------------------------------------------------------------
MATCH (page:Page)-[r:HAS_BLOCK]->(block:Block)
WHERE NOT (block)-[:BLOCK_OF]->(page)
MERGE (block)-[:BLOCK_OF {position: r.position}]->(page);

// ---------------------------------------------------------------------------
// USED_BY from USES_ENTITY
// Entity → Page/Block
// ---------------------------------------------------------------------------
MATCH (user)-[:USES_ENTITY]->(entity:Entity)
WHERE NOT (entity)-[:USED_BY]->(user)
MERGE (entity)-[:USED_BY]->(user);

// ---------------------------------------------------------------------------
// HAS_LOCALIZED_CONTENT from FOR_LOCALE
// Locale → all localized native nodes
// ---------------------------------------------------------------------------
MATCH (native)-[:FOR_LOCALE]->(locale:Locale)
WHERE NOT (locale)-[:HAS_LOCALIZED_CONTENT]->(native)
MERGE (locale)-[:HAS_LOCALIZED_CONTENT]->(native);

// ---------------------------------------------------------------------------
// VERIFICATION: Show counts for all inverse relationships
// ---------------------------------------------------------------------------
MATCH ()-[r:NATIVE_OF]->() WITH 'NATIVE_OF' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:BLOCK_OF]->() WITH 'BLOCK_OF' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:USED_BY]->() WITH 'USED_BY' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:HAS_LOCALIZED_CONTENT]->() WITH 'HAS_LOCALIZED_CONTENT' AS rel, count(r) AS count RETURN rel, count;
