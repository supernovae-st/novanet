// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 001: Create Inverse Relationships (v11.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
// This migration creates inverse relationships for bidirectional traversal.
// The script is idempotent - safe to run multiple times.
//
// v11.0 Updates:
// - HAS_L10N → HAS_CONTENT (Entity → EntityContent)
// - HAS_OUTPUT → HAS_GENERATED (Page/Block → PageGenerated/BlockGenerated)
// - L10N_OF → CONTENT_OF
// - OUTPUT_OF → GENERATED_FOR
// - USES_CONCEPT → USES_ENTITY
// - Concept → Entity
//
// RUN: docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < migrations/001-inverse-relationships.cypher
// ═══════════════════════════════════════════════════════════════════════════════

// ---------------------------------------------------------------------------
// CONTENT_OF from HAS_CONTENT
// EntityContent → Entity
// ---------------------------------------------------------------------------
MATCH (parent)-[:HAS_CONTENT]->(content)
WHERE NOT (content)-[:CONTENT_OF]->(parent)
MERGE (content)-[:CONTENT_OF]->(parent);

// ---------------------------------------------------------------------------
// GENERATED_FOR from HAS_GENERATED
// PageGenerated/BlockGenerated → Page/Block
// ---------------------------------------------------------------------------
MATCH (parent)-[:HAS_GENERATED]->(output)
WHERE NOT (output)-[:GENERATED_FOR]->(parent)
MERGE (output)-[:GENERATED_FOR]->(parent);

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
// Locale → all localized content nodes
// ---------------------------------------------------------------------------
MATCH (content)-[:FOR_LOCALE]->(locale:Locale)
WHERE NOT (locale)-[:HAS_LOCALIZED_CONTENT]->(content)
MERGE (locale)-[:HAS_LOCALIZED_CONTENT]->(content);

// ---------------------------------------------------------------------------
// VERIFICATION: Show counts for all inverse relationships
// ---------------------------------------------------------------------------
MATCH ()-[r:CONTENT_OF]->() WITH 'CONTENT_OF' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:GENERATED_FOR]->() WITH 'GENERATED_FOR' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:BLOCK_OF]->() WITH 'BLOCK_OF' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:USED_BY]->() WITH 'USED_BY' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:HAS_LOCALIZED_CONTENT]->() WITH 'HAS_LOCALIZED_CONTENT' AS rel, count(r) AS count RETURN rel, count;
