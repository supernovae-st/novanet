// Migration 061: Create NATIVE_OF inverse arcs for bidirectional traversal
// Issue: EntityNative, PageNative, BlockNative have HAS_NATIVE from parent
//        but MISSING the inverse NATIVE_OF arc for spreading activation
//
// Per ADR-029 (*Native Pattern) and ADR-026 (Inverse Arc Tiers):
// - NATIVE_OF is TIER 1 (Required) inverse of HAS_NATIVE
// - Enables bidirectional traversal: Entity ↔ EntityNative
//
// This migration creates missing NATIVE_OF arcs to enable:
// - Spreading activation from EntityNative → Entity
// - Traversal: (en:EntityNative)-[:NATIVE_OF]->(e:Entity)

// Step 1: Count EntityNative nodes missing NATIVE_OF
MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
WHERE NOT (en)-[:NATIVE_OF]->(e)
WITH count(en) AS missing_count
RETURN "EntityNative missing NATIVE_OF: " + missing_count AS step1_entity;

// Step 2: Create NATIVE_OF for EntityNative → Entity
MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
WHERE NOT (en)-[:NATIVE_OF]->(e)
MERGE (en)-[:NATIVE_OF {
  created_at: datetime(),
  migration: "061-create-native-of-arcs"
}]->(e)
WITH count(*) AS created
RETURN "EntityNative NATIVE_OF arcs created: " + created AS step2_entity_created;

// Step 3: Count PageNative nodes missing NATIVE_OF
MATCH (p:Page)-[:HAS_NATIVE]->(pn:PageNative)
WHERE NOT (pn)-[:NATIVE_OF]->(p)
WITH count(pn) AS missing_count
RETURN "PageNative missing NATIVE_OF: " + missing_count AS step3_page;

// Step 4: Create NATIVE_OF for PageNative → Page
MATCH (p:Page)-[:HAS_NATIVE]->(pn:PageNative)
WHERE NOT (pn)-[:NATIVE_OF]->(p)
MERGE (pn)-[:NATIVE_OF {
  created_at: datetime(),
  migration: "061-create-native-of-arcs"
}]->(p)
WITH count(*) AS created
RETURN "PageNative NATIVE_OF arcs created: " + created AS step4_page_created;

// Step 5: Count BlockNative nodes missing NATIVE_OF
MATCH (b:Block)-[:HAS_NATIVE]->(bn:BlockNative)
WHERE NOT (bn)-[:NATIVE_OF]->(b)
WITH count(bn) AS missing_count
RETURN "BlockNative missing NATIVE_OF: " + missing_count AS step5_block;

// Step 6: Create NATIVE_OF for BlockNative → Block
MATCH (b:Block)-[:HAS_NATIVE]->(bn:BlockNative)
WHERE NOT (bn)-[:NATIVE_OF]->(b)
MERGE (bn)-[:NATIVE_OF {
  created_at: datetime(),
  migration: "061-create-native-of-arcs"
}]->(b)
WITH count(*) AS created
RETURN "BlockNative NATIVE_OF arcs created: " + created AS step6_block_created;

// Step 7: Count ProjectNative nodes missing NATIVE_OF
MATCH (p:Project)-[:HAS_NATIVE]->(pn:ProjectNative)
WHERE NOT (pn)-[:NATIVE_OF]->(p)
WITH count(pn) AS missing_count
RETURN "ProjectNative missing NATIVE_OF: " + missing_count AS step7_project;

// Step 8: Create NATIVE_OF for ProjectNative → Project
MATCH (p:Project)-[:HAS_NATIVE]->(pn:ProjectNative)
WHERE NOT (pn)-[:NATIVE_OF]->(p)
MERGE (pn)-[:NATIVE_OF {
  created_at: datetime(),
  migration: "061-create-native-of-arcs"
}]->(p)
WITH count(*) AS created
RETURN "ProjectNative NATIVE_OF arcs created: " + created AS step8_project_created;

// Step 9: Final verification - count all NATIVE_OF arcs
MATCH ()-[r:NATIVE_OF]->()
WITH count(r) AS total_native_of
RETURN "Total NATIVE_OF arcs after migration: " + total_native_of AS step9_total;

// Step 10: Breakdown by type
MATCH (n)-[r:NATIVE_OF]->(parent)
WITH labels(n)[0] AS native_type, count(r) AS arc_count
RETURN native_type, arc_count
ORDER BY arc_count DESC;
