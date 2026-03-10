// ============================================================================
// Migration 103: Fix Expression Container Arcs
// ============================================================================
// Purpose: Create CONTAINS_EXPRESSION arcs between ExpressionSet and Expression
// Bug: Seed file used wrong key format for ExpressionSet matching
// Fix: Match by locale property instead
// ============================================================================

// Create CONTAINS_EXPRESSION arcs by matching locale
MATCH (e:Expression)
WHERE e.locale IS NOT NULL
MATCH (es:ExpressionSet)
WHERE es.key = 'expression-set@' + e.locale
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

// Also ensure Locale has HAS_EXPRESSIONS arc to ExpressionSet
MATCH (l:Locale)
MATCH (es:ExpressionSet)
WHERE es.key = 'expression-set@' + l.key
MERGE (l)-[:HAS_EXPRESSIONS]->(es);

