// ═══════════════════════════════════════════════════════════════════════════════
// Migration: v0.12.1 Kind→Class Terminology
// ═══════════════════════════════════════════════════════════════════════════════
//
// ADR-023 renamed:
// - OF_KIND → OF_CLASS (instance→Class relationship)
// - :Meta:Kind → :Schema:Class (already done by generators)
//
// This migration renames any stale OF_KIND relationships that exist in the
// database from before the ADR-023 migration was implemented.
//
// IDEMPOTENT: Safe to run multiple times.
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────
// Step 1: Create OF_CLASS relationships from existing OF_KIND
// ─────────────────────────────────────────────────────────────────────────────
// Neo4j doesn't support renaming relationships directly.
// We create new OF_CLASS relationships and delete the old OF_KIND ones.

CALL apoc.periodic.iterate(
  'MATCH (n)-[r:OF_KIND]->(c) RETURN n, r, c',
  'MERGE (n)-[:OF_CLASS]->(c) DELETE r',
  {batchSize: 1000, parallel: false}
);

// ─────────────────────────────────────────────────────────────────────────────
// Step 2: Verify no OF_KIND relationships remain
// ─────────────────────────────────────────────────────────────────────────────

MATCH ()-[r:OF_KIND]->()
WITH count(r) AS remaining
RETURN
  CASE
    WHEN remaining = 0 THEN 'SUCCESS: All OF_KIND relationships migrated to OF_CLASS'
    ELSE 'WARNING: ' + toString(remaining) + ' OF_KIND relationships still exist'
  END AS migration_status;

// ─────────────────────────────────────────────────────────────────────────────
// Step 3: Summary
// ─────────────────────────────────────────────────────────────────────────────

MATCH ()-[r:OF_CLASS]->()
RETURN count(r) AS total_of_class_relationships;
