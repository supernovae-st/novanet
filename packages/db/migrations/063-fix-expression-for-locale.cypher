// Migration 063: Fix Expression nodes missing FOR_LOCALE arcs
// Issue: 17,036 Expression nodes have no FOR_LOCALE relationship
// Key format: locale/category/index (e.g., "ceb-PH/SUCCESS/0")
// Solution: Extract locale from key and create FOR_LOCALE arcs

// Step 1: Create FOR_LOCALE arcs for Expression nodes
// Extract locale from key pattern: "ceb-PH/SUCCESS/0" -> "ceb-PH"
MATCH (e:Expression)
WHERE NOT (e)-[:FOR_LOCALE]->(:Locale)
  AND e.key CONTAINS '/'
WITH e, split(e.key, '/')[0] AS extracted_locale
MATCH (l:Locale {key: extracted_locale})
MERGE (e)-[:FOR_LOCALE]->(l)
RETURN extracted_locale AS locale, count(e) AS expressions_linked
ORDER BY expressions_linked DESC;

// Step 2: Check for any Expression nodes that couldn't be linked
// (locale doesn't exist in database)
MATCH (e:Expression)
WHERE NOT (e)-[:FOR_LOCALE]->(:Locale)
WITH split(e.key, '/')[0] AS missing_locale, count(e) AS count
RETURN missing_locale, count
ORDER BY count DESC;

// Step 3: Summary - total Expression nodes with FOR_LOCALE arcs
MATCH (e:Expression)-[:FOR_LOCALE]->(l:Locale)
RETURN count(DISTINCT e) AS expressions_with_for_locale,
       count(DISTINCT l) AS locales_used;
