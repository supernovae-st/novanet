// ============================================================================
// MIGRATION 078: Fix ExpressionSet Total Expression Counts
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Update total_expressions property to reflect actual count
//
// Issue: 199/200 ExpressionSets have inaccurate total_expressions
//        (declared ~90 but actual ~170 per locale)
//
// Fix: Count actual CONTAINS_EXPRESSION arcs and update property
// ============================================================================

// Step 1: Update total_expressions based on actual arc count
MATCH (es:ExpressionSet)
OPTIONAL MATCH (es)-[:CONTAINS_EXPRESSION]->(e:Expression)
WITH es, count(e) AS actual_count
SET es.total_expressions = actual_count,
    es.updated_at = datetime()
RETURN count(es) AS expressionsets_updated;

// Step 2: Verification - show sample of updated counts
MATCH (es:ExpressionSet)
OPTIONAL MATCH (es)-[:CONTAINS_EXPRESSION]->(e:Expression)
WITH es, count(e) AS actual_count
RETURN es.key AS key, es.total_expressions AS total_expressions, actual_count
ORDER BY es.key
LIMIT 10;
