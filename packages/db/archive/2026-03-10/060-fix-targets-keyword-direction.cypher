// Migration 060: Fix TARGETS_KEYWORD arc direction
// Issue: Migration 056 created SEOKeyword → EntityNative (wrong!)
// Schema: TARGETS_KEYWORD should be EntityNative → SEOKeyword (content targets keyword)
//
// Fix: Reverse all existing arcs to match the arc class definition
// This enables queries like: (en:EntityNative)-[:TARGETS_KEYWORD]->(kw:SEOKeyword)

// Step 1: Count current wrong-direction arcs
MATCH (sk:SEOKeyword)-[r:TARGETS_KEYWORD]->(en:EntityNative)
WITH count(r) AS wrong_direction_count
RETURN "Wrong direction arcs found: " + wrong_direction_count AS step1_count;

// Step 2: Create correct direction arcs (EntityNative → SEOKeyword)
MATCH (sk:SEOKeyword)-[r:TARGETS_KEYWORD]->(en:EntityNative)
MERGE (en)-[:TARGETS_KEYWORD {
  priority: COALESCE(r.priority, 'primary'),
  weight: COALESCE(r.weight, 0.9),
  is_slug_source: COALESCE(r.is_slug_source, false),
  curator: 'auto-derived',
  targeted_at: datetime()
}]->(sk)
WITH count(*) AS created
RETURN "Correct direction arcs created: " + created AS step2_created;

// Step 3: Delete wrong direction arcs
MATCH (sk:SEOKeyword)-[r:TARGETS_KEYWORD]->(en:EntityNative)
DELETE r
WITH count(*) AS deleted
RETURN "Wrong direction arcs deleted: " + deleted AS step3_deleted;

// Step 4: Verify final state
MATCH (en:EntityNative)-[r:TARGETS_KEYWORD]->(sk:SEOKeyword)
WITH count(r) AS correct_count
MATCH (sk:SEOKeyword)-[r2:TARGETS_KEYWORD]->(en:EntityNative)
WITH correct_count, count(r2) AS wrong_count
RETURN "Final state: " + correct_count + " correct, " + wrong_count + " wrong" AS step4_verify;

// Step 5: Summary by locale
MATCH (en:EntityNative)-[r:TARGETS_KEYWORD]->(sk:SEOKeyword)
WITH split(en.key, '@')[1] AS locale, count(r) AS arc_count
RETURN locale, arc_count
ORDER BY arc_count DESC;
