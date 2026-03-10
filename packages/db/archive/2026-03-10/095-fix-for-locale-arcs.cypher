// ============================================================================
// PLAN C - Migration 095: Fix FOR_LOCALE Arcs Pointing to Null
// ============================================================================
// Priority: CRITICAL (Broken referential integrity)
// Fixes: 60 nodes have FOR_LOCALE pointing to non-existent locales
// CSR Impact: Fixes locale linking for SEO/GEO nodes
// ============================================================================

// First, identify the broken relationships
MATCH (n)-[r:FOR_LOCALE]->(target)
WHERE target IS NULL OR NOT (target:Locale)
RETURN labels(n)[0] AS node_type,
       n.key AS node_key,
       type(r) AS relationship,
       'BROKEN' AS status
LIMIT 10;

// Delete broken FOR_LOCALE relationships where target doesn't exist
MATCH (n)-[r:FOR_LOCALE]->(target)
WHERE target IS NULL
DELETE r;

// For nodes with locale in their key (e.g., keyword:create-qr-code@fr-FR),
// extract locale and create proper relationship
MATCH (n)
WHERE n.key CONTAINS '@'
  AND NOT (n)-[:FOR_LOCALE]->(:Locale)
WITH n, split(n.key, '@')[1] AS locale_key
MATCH (l:Locale {key: locale_key})
MERGE (n)-[:FOR_LOCALE]->(l);

// For SEOKeyword nodes, extract locale from locale_key property if present
MATCH (kw:SEOKeyword)
WHERE kw.locale_key IS NOT NULL
  AND NOT (kw)-[:FOR_LOCALE]->(:Locale)
MATCH (l:Locale {key: kw.locale_key})
MERGE (kw)-[:FOR_LOCALE]->(l);

// For GEOQuery/GEOAnswer nodes with locale in key
MATCH (g)
WHERE (g:GEOQuery OR g:GEOAnswer)
  AND g.key CONTAINS '@'
  AND NOT (g)-[:FOR_LOCALE]->(:Locale)
WITH g, split(g.key, '@')[1] AS locale_key
MATCH (l:Locale {key: locale_key})
MERGE (g)-[:FOR_LOCALE]->(l);

// Verify FOR_LOCALE integrity
MATCH (n)-[:FOR_LOCALE]->(l:Locale)
WITH labels(n)[0] AS node_type, count(*) AS linked_count
RETURN node_type, linked_count, 'LINKED' AS status
ORDER BY linked_count DESC;

// Find remaining orphans (nodes that should have FOR_LOCALE but don't)
MATCH (n)
WHERE (n:SEOKeyword OR n:GEOQuery OR n:GEOAnswer OR n:EntityNative OR n:PageNative OR n:BlockNative)
  AND NOT (n)-[:FOR_LOCALE]->(:Locale)
RETURN labels(n)[0] AS node_type,
       count(*) AS orphan_count,
       'NEEDS_ATTENTION' AS status;
