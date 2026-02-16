// packages/db/seed/33-seo-keyword-remediation.cypher
// v0.13.0 - SEOKeyword Data Quality Remediation
//
// Fixes:
// 1. SEOKeyword with NULL locale → extract from key pattern
// 2. SEOKeyword with NULL phrase → extract from key pattern

// ============================================================================
// 1. FIX NULL LOCALE ON SEOKeyword (extract from key pattern)
// ============================================================================
// Keys are in format: seo-{phrase}-{locale}-{hash}
// Example: seo-qr-code-fr-fr-0b9d04 → locale = fr-FR

// fr-FR keywords
MATCH (k:SEOKeyword)
WHERE k.locale IS NULL AND k.key CONTAINS '-fr-fr-'
SET k.locale = 'fr-FR'
;

// en-US keywords
MATCH (k:SEOKeyword)
WHERE k.locale IS NULL AND k.key CONTAINS '-en-us-'
SET k.locale = 'en-US'
;

// es-ES keywords
MATCH (k:SEOKeyword)
WHERE k.locale IS NULL AND k.key CONTAINS '-es-es-'
SET k.locale = 'es-ES'
;

// de-DE keywords
MATCH (k:SEOKeyword)
WHERE k.locale IS NULL AND k.key CONTAINS '-de-de-'
SET k.locale = 'de-DE'
;

// pt-BR keywords
MATCH (k:SEOKeyword)
WHERE k.locale IS NULL AND k.key CONTAINS '-pt-br-'
SET k.locale = 'pt-BR'
;

// it-IT keywords
MATCH (k:SEOKeyword)
WHERE k.locale IS NULL AND k.key CONTAINS '-it-it-'
SET k.locale = 'it-IT'
;

// nl-NL keywords
MATCH (k:SEOKeyword)
WHERE k.locale IS NULL AND k.key CONTAINS '-nl-nl-'
SET k.locale = 'nl-NL'
;

// ja-JP keywords
MATCH (k:SEOKeyword)
WHERE k.locale IS NULL AND k.key CONTAINS '-ja-jp-'
SET k.locale = 'ja-JP'
;

// ============================================================================
// 2. DERIVE slug_form FROM key (for display)
// ============================================================================
// Extract phrase portion: seo-{phrase}-{locale}-{hash} → phrase

// For fr-FR keywords without slug_form
MATCH (k:SEOKeyword)
WHERE k.locale = 'fr-FR' AND k.slug_form IS NULL AND k.key STARTS WITH 'seo-'
WITH k,
  // Remove 'seo-' prefix and '-fr-fr-{hash}' suffix
  replace(k.key, 'seo-', '') AS without_prefix
WITH k, without_prefix,
  split(without_prefix, '-fr-fr-')[0] AS phrase_slug
SET k.slug_form = phrase_slug
;

// ============================================================================
// 3. CONNECT SEOKeyword TO Locale
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.locale IS NOT NULL
MATCH (l:Locale {key: k.locale})
MERGE (k)-[:FOR_LOCALE]->(l)
;
