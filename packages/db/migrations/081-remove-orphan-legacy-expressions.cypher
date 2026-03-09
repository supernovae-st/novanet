// ============================================================================
// Migration 081: Remove orphan legacy-format Expression nodes
// Date: 2026-03-10
// Issue: 114 Expression nodes in legacy format (word@locale) not linked to
//        any ExpressionSet via CONTAINS_EXPRESSION arc
// Affected locales: fil-PH (42), km-KH (37), lo-LA (35)
// Root cause: Old seed data created expressions with @locale suffix format
//             instead of standard locale/CATEGORY/INDEX format
// ============================================================================

// Step 1: Verify orphan counts before deletion
MATCH (e:Expression)
WHERE NOT EXISTS { MATCH (:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e) }
WITH e,
  CASE
    WHEN e.key CONTAINS '@' THEN split(e.key, '@')[1]
    ELSE 'unknown'
  END AS locale
RETURN locale, count(*) AS orphan_count
ORDER BY orphan_count DESC;

// Step 2: Delete orphan expressions (legacy format only - have @ in key)
MATCH (e:Expression)
WHERE NOT EXISTS { MATCH (:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e) }
  AND e.key CONTAINS '@'
DETACH DELETE e
RETURN count(*) AS deleted_legacy_expressions;

// Step 3: Verify no orphans remain
MATCH (e:Expression)
WHERE NOT EXISTS { MATCH (:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e) }
RETURN count(*) AS remaining_orphans;
