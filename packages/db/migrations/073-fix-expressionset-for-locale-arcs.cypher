// ============================================================================
// MIGRATION 073: Fix ExpressionSet FOR_LOCALE Arcs
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Create missing FOR_LOCALE arcs from ExpressionSet to Locale
//
// Issue: ExpressionSet nodes are not connected to their Locale via FOR_LOCALE
// Fix: Create FOR_LOCALE arc from ExpressionSet to matching Locale
// ============================================================================

// Step 1: Create FOR_LOCALE arcs for ExpressionSets
// Extract locale code from key (expression-set@fr-FR → fr-FR)
MATCH (es:ExpressionSet)
WHERE NOT EXISTS {
  MATCH (es)-[:FOR_LOCALE]->(:Locale)
}
WITH es, replace(es.key, 'expression-set@', '') AS locale_code
MATCH (l:Locale {key: locale_code})
MERGE (es)-[:FOR_LOCALE]->(l)
RETURN count(*) AS for_locale_arcs_created;

// Step 2: Also ensure HAS_EXPRESSIONS arcs exist from Locale to ExpressionSet
MATCH (es:ExpressionSet)
WHERE NOT EXISTS {
  MATCH (:Locale)-[:HAS_EXPRESSIONS]->(es)
}
WITH es, replace(es.key, 'expression-set@', '') AS locale_code
MATCH (l:Locale {key: locale_code})
MERGE (l)-[:HAS_EXPRESSIONS]->(es)
RETURN count(*) AS has_expressions_arcs_created;

// Verification query
MATCH (es:ExpressionSet)-[:FOR_LOCALE]->(l:Locale)
RETURN count(*) AS expressionsets_with_for_locale;
