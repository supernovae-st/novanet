// ============================================================================
// Migration 119: Key Prefix - Tier 4 (Knowledge Atoms)
// ============================================================================
// Knowledge atoms: Expression, CultureRef, Taboo, Pattern, AudienceTrait
// These are locale-specific atoms linked via FOR_LOCALE arcs.
//
// Pattern: {prefix}:{hash}@{locale} or {prefix}:{slug}@{locale}
// Using hash for Expression (17k+ with possible duplicates)
// Using slug for others (smaller sets, more readable)
// ============================================================================

// --- Expression: expr:{slug}@{locale} ---
// Create slug from text field (first 20 chars, slugified)
MATCH (n:Expression)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'expr:'
WITH n, l.key AS localeKey, n.text AS text
WITH n, localeKey,
     toLower(replace(replace(left(coalesce(text, 'unknown'), 20), ' ', '-'), '"', '')) AS slug
SET n.key = 'expr:' + slug + '@' + replace(localeKey, 'locale:', '');

// Fallback for orphan Expressions
MATCH (n:Expression)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'expr:'
SET n.key = 'expr:orphan-' + id(n) + '@unknown';

// --- CultureRef: cultureref:{slug}@{locale} ---
MATCH (n:CultureRef)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'cultureref:'
WITH n, l.key AS localeKey, n.text AS text
WITH n, localeKey,
     toLower(replace(replace(coalesce(text, n.key), ' ', '-'), '"', '')) AS slug
SET n.key = 'cultureref:' + left(slug, 30) + '@' + replace(localeKey, 'locale:', '');

MATCH (n:CultureRef)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'cultureref:'
SET n.key = 'cultureref:' + left(toLower(replace(n.key, ' ', '-')), 30) + '@unknown';

// --- Taboo: taboo:{slug}@{locale} ---
MATCH (n:Taboo)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'taboo:'
WITH n, l.key AS localeKey, n.text AS text
WITH n, localeKey,
     toLower(replace(replace(coalesce(text, n.key), ' ', '-'), '"', '')) AS slug
SET n.key = 'taboo:' + left(slug, 30) + '@' + replace(localeKey, 'locale:', '');

MATCH (n:Taboo)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'taboo:'
SET n.key = 'taboo:' + left(toLower(replace(n.key, ' ', '-')), 30) + '@unknown';

// --- Pattern: pattern:{slug}@{locale} ---
MATCH (n:Pattern)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'pattern:'
WITH n, l.key AS localeKey
SET n.key = 'pattern:' + toLower(replace(n.key, ' ', '-')) + '@' + replace(localeKey, 'locale:', '');

MATCH (n:Pattern)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'pattern:'
SET n.key = 'pattern:' + toLower(replace(n.key, ' ', '-')) + '@unknown';

// --- AudienceTrait: audience:{slug}@{locale} ---
MATCH (n:AudienceTrait)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'audience:'
WITH n, l.key AS localeKey, n.text AS text
WITH n, localeKey,
     toLower(replace(replace(coalesce(text, n.key), ' ', '-'), '"', '')) AS slug
SET n.key = 'audience:' + left(slug, 30) + '@' + replace(localeKey, 'locale:', '');

MATCH (n:AudienceTrait)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'audience:'
SET n.key = 'audience:' + left(toLower(replace(n.key, ' ', '-')), 30) + '@unknown';

// --- Verification ---
// MATCH (n) WHERE labels(n)[0] IN ['Expression', 'CultureRef', 'Taboo', 'Pattern', 'AudienceTrait']
//   AND NOT n.key CONTAINS ':'
// RETURN labels(n)[0], count(n);
