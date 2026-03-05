// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 009: v0.13.0 *Native Pattern (ADR-029)
// ═══════════════════════════════════════════════════════════════════════════════
//
// This migration implements ADR-029 *Native pattern:
// - EntityContent → EntityNative
// - ProjectContent → ProjectNative
// - PageGenerated → PageNative
// - BlockGenerated → BlockNative
// - HAS_CONTENT/HAS_GENERATED → HAS_NATIVE
// - CONTENT_OF/GENERATED_FOR → NATIVE_OF
//
// The script is idempotent - safe to run multiple times.
//
// RUN: docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < migrations/009-v013-native-pattern.cypher
// ═══════════════════════════════════════════════════════════════════════════════

// ---------------------------------------------------------------------------
// STEP 1: Add new labels to existing nodes (additive, safe)
// ---------------------------------------------------------------------------

// EntityContent → EntityNative
MATCH (n:EntityContent)
WHERE NOT n:EntityNative
SET n:EntityNative;

// ProjectContent → ProjectNative
MATCH (n:ProjectContent)
WHERE NOT n:ProjectNative
SET n:ProjectNative;

// PageGenerated → PageNative
MATCH (n:PageGenerated)
WHERE NOT n:PageNative
SET n:PageNative;

// BlockGenerated → BlockNative
MATCH (n:BlockGenerated)
WHERE NOT n:BlockNative
SET n:BlockNative;

// ---------------------------------------------------------------------------
// STEP 2: Create HAS_NATIVE relationships from HAS_CONTENT
// ---------------------------------------------------------------------------

// Entity → EntityNative
MATCH (e:Entity)-[r:HAS_CONTENT]->(en:EntityNative)
WHERE NOT (e)-[:HAS_NATIVE]->(en)
CREATE (e)-[:HAS_NATIVE {locale: r.locale}]->(en);

// Project → ProjectNative
MATCH (p:Project)-[r:HAS_CONTENT]->(pn:ProjectNative)
WHERE NOT (p)-[:HAS_NATIVE]->(pn)
CREATE (p)-[:HAS_NATIVE {locale: r.locale}]->(pn);

// ---------------------------------------------------------------------------
// STEP 3: Create HAS_NATIVE from HAS_GENERATED (for Page/Block)
// ---------------------------------------------------------------------------

// Page → PageNative
MATCH (p:Page)-[r:HAS_GENERATED]->(pn:PageNative)
WHERE NOT (p)-[:HAS_NATIVE]->(pn)
CREATE (p)-[:HAS_NATIVE {locale: r.locale}]->(pn);

// Block → BlockNative
MATCH (b:Block)-[r:HAS_GENERATED]->(bn:BlockNative)
WHERE NOT (b)-[:HAS_NATIVE]->(bn)
CREATE (b)-[:HAS_NATIVE {locale: r.locale}]->(bn);

// ---------------------------------------------------------------------------
// STEP 4: Create NATIVE_OF from CONTENT_OF
// ---------------------------------------------------------------------------

// EntityNative → Entity
MATCH (en:EntityNative)-[r:CONTENT_OF]->(e:Entity)
WHERE NOT (en)-[:NATIVE_OF]->(e)
CREATE (en)-[:NATIVE_OF]->(e);

// ProjectNative → Project
MATCH (pn:ProjectNative)-[r:CONTENT_OF]->(p:Project)
WHERE NOT (pn)-[:NATIVE_OF]->(p)
CREATE (pn)-[:NATIVE_OF]->(p);

// ---------------------------------------------------------------------------
// STEP 5: Create NATIVE_OF from GENERATED_FOR (for Page/Block)
// ---------------------------------------------------------------------------

// PageNative → Page
MATCH (pn:PageNative)-[r:GENERATED_FOR]->(p:Page)
WHERE NOT (pn)-[:NATIVE_OF]->(p)
CREATE (pn)-[:NATIVE_OF]->(p);

// BlockNative → Block
MATCH (bn:BlockNative)-[r:GENERATED_FOR]->(b:Block)
WHERE NOT (bn)-[:NATIVE_OF]->(b)
CREATE (bn)-[:NATIVE_OF]->(b);

// ---------------------------------------------------------------------------
// STEP 6: Update Schema:Class nodes for *Native labels
// ---------------------------------------------------------------------------

// Rename EntityContent schema node to EntityNative
MATCH (c:Schema:Class {label: 'EntityContent'})
SET c.label = 'EntityNative', c.name = 'EntityNative';

// Rename ProjectContent schema node to ProjectNative
MATCH (c:Schema:Class {label: 'ProjectContent'})
SET c.label = 'ProjectNative', c.name = 'ProjectNative';

// Rename PageGenerated schema node to PageNative
MATCH (c:Schema:Class {label: 'PageGenerated'})
SET c.label = 'PageNative', c.name = 'PageNative';

// Rename BlockGenerated schema node to BlockNative
MATCH (c:Schema:Class {label: 'BlockGenerated'})
SET c.label = 'BlockNative', c.name = 'BlockNative';

// ---------------------------------------------------------------------------
// STEP 7: Update Schema:ArcClass nodes
// ---------------------------------------------------------------------------

// Rename HAS_CONTENT arc schema to HAS_NATIVE
MATCH (a:Schema:ArcClass {name: 'HAS_CONTENT'})
SET a.name = 'HAS_NATIVE';

// Rename HAS_GENERATED arc schema to HAS_NATIVE (merge if exists)
MATCH (a:Schema:ArcClass {name: 'HAS_GENERATED'})
SET a.name = 'HAS_NATIVE';

// Rename CONTENT_OF arc schema to NATIVE_OF
MATCH (a:Schema:ArcClass {name: 'CONTENT_OF'})
SET a.name = 'NATIVE_OF';

// Rename GENERATED_FOR arc schema to NATIVE_OF (merge if exists)
MATCH (a:Schema:ArcClass {name: 'GENERATED_FOR'})
SET a.name = 'NATIVE_OF';

// ---------------------------------------------------------------------------
// STEP 8: Remove old labels (cleanup)
// ---------------------------------------------------------------------------

// Remove EntityContent label (keep EntityNative)
MATCH (n:EntityContent:EntityNative)
REMOVE n:EntityContent;

// Remove ProjectContent label (keep ProjectNative)
MATCH (n:ProjectContent:ProjectNative)
REMOVE n:ProjectContent;

// Remove PageGenerated label (keep PageNative)
MATCH (n:PageGenerated:PageNative)
REMOVE n:PageGenerated;

// Remove BlockGenerated label (keep BlockNative)
MATCH (n:BlockGenerated:BlockNative)
REMOVE n:BlockGenerated;

// ---------------------------------------------------------------------------
// STEP 9: Delete deprecated relationships
// ---------------------------------------------------------------------------

// Delete HAS_CONTENT relationships (now replaced by HAS_NATIVE)
MATCH ()-[r:HAS_CONTENT]->()
DELETE r;

// Delete HAS_GENERATED relationships (now replaced by HAS_NATIVE)
MATCH ()-[r:HAS_GENERATED]->()
DELETE r;

// Delete CONTENT_OF relationships (now replaced by NATIVE_OF)
MATCH ()-[r:CONTENT_OF]->()
DELETE r;

// Delete GENERATED_FOR relationships (now replaced by NATIVE_OF)
MATCH ()-[r:GENERATED_FOR]->()
DELETE r;

// ---------------------------------------------------------------------------
// VERIFICATION: Show final counts
// ---------------------------------------------------------------------------
MATCH (n:EntityNative) WITH 'EntityNative' AS label, count(n) AS count RETURN label, count
UNION ALL
MATCH (n:ProjectNative) WITH 'ProjectNative' AS label, count(n) AS count RETURN label, count
UNION ALL
MATCH (n:PageNative) WITH 'PageNative' AS label, count(n) AS count RETURN label, count
UNION ALL
MATCH (n:BlockNative) WITH 'BlockNative' AS label, count(n) AS count RETURN label, count
UNION ALL
MATCH ()-[r:HAS_NATIVE]->() WITH 'HAS_NATIVE' AS label, count(r) AS count RETURN label, count
UNION ALL
MATCH ()-[r:NATIVE_OF]->() WITH 'NATIVE_OF' AS label, count(r) AS count RETURN label, count;
