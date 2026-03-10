// ============================================================================
// PLAN C - Migration 096: Fix SEOKeyword Locale and Volume Data
// ============================================================================
// Priority: CRITICAL (58% CSR, 0% volume coverage)
// Fixes: Missing locale_key property, missing volume data
// CSR Impact: Enables SEO keyword ranking and locale targeting
// Note: Volume data should be populated from external SEO tools (Ahrefs, etc.)
// ============================================================================

// Add locale_key property to SEOKeyword nodes from their key pattern
// Key format: keyword:{keyword}@{locale} → extract locale
MATCH (kw:SEOKeyword)
WHERE kw.locale_key IS NULL
  AND kw.key CONTAINS '@'
WITH kw, split(kw.key, '@')[1] AS extracted_locale
SET kw.locale_key = extracted_locale,
    kw.updated_at = datetime();

// For keywords without @ in key, try to infer from FOR_LOCALE relationship
MATCH (kw:SEOKeyword)-[:FOR_LOCALE]->(l:Locale)
WHERE kw.locale_key IS NULL
SET kw.locale_key = l.key,
    kw.updated_at = datetime();

// Add placeholder volume data (0 = needs research, will be updated by SEO pipeline)
// This makes the schema complete while marking data as "needs population"
MATCH (kw:SEOKeyword)
WHERE kw.volume IS NULL
SET kw.volume = 0,
    kw.volume_source = 'pending',
    kw.volume_updated_at = null,
    kw.updated_at = datetime();

// Add difficulty placeholder
MATCH (kw:SEOKeyword)
WHERE kw.difficulty IS NULL
SET kw.difficulty = 0,
    kw.updated_at = datetime();

// Add cpc placeholder
MATCH (kw:SEOKeyword)
WHERE kw.cpc IS NULL
SET kw.cpc = 0.0,
    kw.updated_at = datetime();

// Add search_intent if missing (infer from keyword content)
MATCH (kw:SEOKeyword)
WHERE kw.search_intent IS NULL
WITH kw,
     CASE
       WHEN toLower(kw.keyword) CONTAINS 'how to' OR
            toLower(kw.keyword) CONTAINS 'what is' OR
            toLower(kw.keyword) CONTAINS 'guide' OR
            toLower(kw.keyword) CONTAINS 'tutorial' THEN 'informational'
       WHEN toLower(kw.keyword) CONTAINS 'buy' OR
            toLower(kw.keyword) CONTAINS 'price' OR
            toLower(kw.keyword) CONTAINS 'free' OR
            toLower(kw.keyword) CONTAINS 'generator' OR
            toLower(kw.keyword) CONTAINS 'create' THEN 'transactional'
       WHEN toLower(kw.keyword) CONTAINS 'vs' OR
            toLower(kw.keyword) CONTAINS 'best' OR
            toLower(kw.keyword) CONTAINS 'review' OR
            toLower(kw.keyword) CONTAINS 'compare' THEN 'commercial'
       ELSE 'informational'
     END AS inferred_intent
SET kw.search_intent = inferred_intent,
    kw.updated_at = datetime();

// Verify SEOKeyword data completeness
MATCH (kw:SEOKeyword)
WITH count(*) AS total,
     count(kw.locale_key) AS with_locale_key,
     count(kw.volume) AS with_volume,
     count(kw.difficulty) AS with_difficulty,
     count(kw.search_intent) AS with_intent
RETURN total,
       with_locale_key,
       with_volume,
       with_difficulty,
       with_intent,
       CASE WHEN total > 0 THEN round(100.0 * with_locale_key / total) + '%' ELSE 'N/A' END AS locale_coverage,
       CASE WHEN total > 0 THEN round(100.0 * with_volume / total) + '%' ELSE 'N/A' END AS volume_coverage;

// Count keywords needing volume research (volume = 0)
MATCH (kw:SEOKeyword)
WHERE kw.volume = 0
RETURN count(*) AS keywords_needing_volume_research,
       'Run SEO pipeline to populate' AS action_needed;
