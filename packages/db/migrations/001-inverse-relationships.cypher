// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 001: Create Inverse Relationships (v7.8.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
// This migration creates inverse relationships for bidirectional traversal.
// The script is idempotent - safe to run multiple times.
//
// RUN: docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < migrations/001-inverse-relationships.cypher
// ═══════════════════════════════════════════════════════════════════════════════

// ---------------------------------------------------------------------------
// L10N_OF from HAS_L10N
// ConceptL10n/ProjectL10n → Concept/Project
// ---------------------------------------------------------------------------
MATCH (parent)-[:HAS_L10N]->(l10n)
WHERE NOT (l10n)-[:L10N_OF]->(parent)
MERGE (l10n)-[:L10N_OF]->(parent);

// ---------------------------------------------------------------------------
// OUTPUT_OF from HAS_OUTPUT
// PageL10n/BlockL10n → Page/Block
// ---------------------------------------------------------------------------
MATCH (parent)-[:HAS_OUTPUT]->(output)
WHERE NOT (output)-[:OUTPUT_OF]->(parent)
MERGE (output)-[:OUTPUT_OF]->(parent);

// ---------------------------------------------------------------------------
// BLOCK_OF from HAS_BLOCK
// Block → Page (with position property)
// ---------------------------------------------------------------------------
MATCH (page:Page)-[r:HAS_BLOCK]->(block:Block)
WHERE NOT (block)-[:BLOCK_OF]->(page)
MERGE (block)-[:BLOCK_OF {position: r.position}]->(page);

// ---------------------------------------------------------------------------
// USED_BY from USES_CONCEPT
// Concept → Page/Block
// ---------------------------------------------------------------------------
MATCH (user)-[:USES_CONCEPT]->(concept:Concept)
WHERE NOT (concept)-[:USED_BY]->(user)
MERGE (concept)-[:USED_BY]->(user);

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
MATCH ()-[r:L10N_OF]->() WITH 'L10N_OF' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:OUTPUT_OF]->() WITH 'OUTPUT_OF' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:BLOCK_OF]->() WITH 'BLOCK_OF' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:USED_BY]->() WITH 'USED_BY' AS rel, count(r) AS count RETURN rel, count
UNION ALL
MATCH ()-[r:HAS_LOCALIZED_CONTENT]->() WITH 'HAS_LOCALIZED_CONTENT' AS rel, count(r) AS count RETURN rel, count;
