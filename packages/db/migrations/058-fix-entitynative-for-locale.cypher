// Migration 058: Fix EntityNative nodes with NULL locale and missing FOR_LOCALE arcs
// Issue: 18 EntityNatives have locale=NULL and no FOR_LOCALE relationship
// Solution: Extract locale from key pattern (entity:slug@locale) and create arcs

// Step 1: Find EntityNatives without FOR_LOCALE arcs and extract locale from key
// Key pattern: entity:qr-code@fr-FR → locale = fr-FR
MATCH (en:EntityNative)
WHERE NOT (en)-[:FOR_LOCALE]->(:Locale)
  AND en.key CONTAINS '@'
WITH en, split(en.key, '@')[1] AS extracted_locale
MATCH (l:Locale {key: extracted_locale})
// Set the locale property if NULL
SET en.locale = COALESCE(en.locale, extracted_locale)
// Create FOR_LOCALE arc (MERGE for idempotency)
MERGE (en)-[:FOR_LOCALE]->(l)
RETURN en.key AS fixed_entitynative, extracted_locale AS locale, l.key AS linked_locale;

// Step 2: Verify no orphaned EntityNatives remain
MATCH (en:EntityNative)
WHERE NOT (en)-[:FOR_LOCALE]->(:Locale)
RETURN en.key AS still_missing_arc, en.locale AS locale_value;

// Step 3: Summary - count EntityNatives with proper FOR_LOCALE arcs
MATCH (en:EntityNative)-[:FOR_LOCALE]->(l:Locale)
RETURN l.key AS locale, count(en) AS entitynative_count
ORDER BY entitynative_count DESC;
