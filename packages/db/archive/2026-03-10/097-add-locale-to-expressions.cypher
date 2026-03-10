// ============================================================================
// PLAN C - Migration 097: Add Locale Property to Expression Nodes
// ============================================================================
// Priority: CRITICAL (17,036 expressions missing locale property)
// Fixes: Expressions have locale in container relationship but not as property
// CSR Impact: Enables direct locale filtering without traversal
// ============================================================================

// Add locale property by traversing from ExpressionSet to Locale
MATCH (l:Locale)-[:HAS_EXPRESSIONS]->(es:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e:Expression)
WHERE e.locale IS NULL
SET e.locale = l.key,
    e.updated_at = datetime();

// For expressions connected directly to locale (alternative structure)
MATCH (e:Expression)<-[:CONTAINS_EXPRESSION]-(es:ExpressionSet)<-[:HAS_EXPRESSIONS]-(l:Locale)
WHERE e.locale IS NULL
SET e.locale = l.key,
    e.updated_at = datetime();

// Try to extract locale from key if pattern matches (e.g., "greeting_formal_fr-FR")
MATCH (e:Expression)
WHERE e.locale IS NULL
  AND e.key =~ '.*_[a-z]{2}-[A-Z]{2}$'
WITH e,
     reverse(split(reverse(e.key), '_')[0]) AS extracted_locale
WHERE extracted_locale =~ '^[a-z]{2}-[A-Z]{2}$'
SET e.locale = extracted_locale,
    e.updated_at = datetime();

// Create FOR_LOCALE arcs for expressions that now have locale property
MATCH (e:Expression)
WHERE e.locale IS NOT NULL
  AND NOT (e)-[:FOR_LOCALE]->(:Locale)
MATCH (l:Locale {key: e.locale})
MERGE (e)-[:FOR_LOCALE]->(l);

// Create index on Expression.locale for fast filtering
CREATE INDEX expression_locale IF NOT EXISTS FOR (e:Expression) ON (e.locale);

// Verify locale coverage on Expressions
MATCH (e:Expression)
WITH count(*) AS total,
     count(e.locale) AS with_locale_property,
     count { MATCH (e)-[:FOR_LOCALE]->(:Locale) } AS with_locale_arc
RETURN total,
       with_locale_property,
       with_locale_arc AS expressions_with_arc,
       CASE WHEN total > 0 THEN round(100.0 * with_locale_property / total) + '%' ELSE 'N/A' END AS property_coverage;

// Sample verification
MATCH (e:Expression)
WHERE e.locale IS NOT NULL
RETURN e.key, e.locale, e.text
ORDER BY e.locale, e.key
LIMIT 10;

// Find remaining orphans
MATCH (e:Expression)
WHERE e.locale IS NULL
RETURN count(*) AS orphan_expressions,
       'May need manual investigation' AS note;
