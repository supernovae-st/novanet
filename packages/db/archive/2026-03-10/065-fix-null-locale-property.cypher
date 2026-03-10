// Migration 065: Fix *Native nodes with NULL locale property
// Issue: Many *Native nodes have locale=NULL despite having FOR_LOCALE arcs
// Solution: Extract locale from key pattern and set locale property

// Step 1: Fix EntityNative nodes (key pattern: entity@locale)
MATCH (n:EntityNative)
WHERE n.locale IS NULL AND n.key CONTAINS '@'
SET n.locale = split(n.key, '@')[1]
RETURN 'EntityNative' as type, count(n) as fixed;

// Step 2: Fix PageNative nodes (key pattern: page:slug@locale)
MATCH (n:PageNative)
WHERE n.locale IS NULL AND n.key CONTAINS '@'
SET n.locale = split(n.key, '@')[1]
RETURN 'PageNative' as type, count(n) as fixed;

// Step 3: Fix BlockNative nodes (key pattern: block:slug@locale)
MATCH (n:BlockNative)
WHERE n.locale IS NULL AND n.key CONTAINS '@'
SET n.locale = split(n.key, '@')[1]
RETURN 'BlockNative' as type, count(n) as fixed;

// Step 4: Fix ProjectNative nodes (key pattern: project-locale)
MATCH (n:ProjectNative)
WHERE n.locale IS NULL AND n.key CONTAINS '-'
WITH n, split(n.key, '-') as parts
SET n.locale = parts[size(parts)-2] + '-' + parts[size(parts)-1]
RETURN 'ProjectNative' as type, count(n) as fixed;

// Step 5: Verify no *Native nodes remain with NULL locale
MATCH (n)
WHERE (n:EntityNative OR n:PageNative OR n:BlockNative OR n:ProjectNative)
  AND n.locale IS NULL
RETURN labels(n)[0] as type, n.key as key;
