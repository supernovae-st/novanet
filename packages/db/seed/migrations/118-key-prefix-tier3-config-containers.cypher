// ============================================================================
// Migration 118: Key Prefix - Tier 3 (Config + Set Containers)
// ============================================================================
// Config nodes and Set containers depend on Locale.
// Pattern: {prefix}:{locale} or {prefix}:{domain}@{locale}
//
// Config (per-locale): slugify:fr-FR, format:fr-FR, culture:fr-FR, etc.
// Sets (per-domain per-locale): exprset:greetings@fr-FR
// ============================================================================

// --- Slugification: slugify:{locale} ---
MATCH (n:Slugification)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'slugify:'
SET n.key = 'slugify:' + n.key;

// --- Formatting: format:{locale} ---
MATCH (n:Formatting)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'format:'
SET n.key = 'format:' + n.key;

// --- Culture: culture:{locale} ---
MATCH (n:Culture)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'culture:'
SET n.key = 'culture:' + n.key;

// --- Adaptation: adaptation:{locale} ---
MATCH (n:Adaptation)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'adaptation:'
SET n.key = 'adaptation:' + n.key;

// --- Style: style:{locale} ---
MATCH (n:Style)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'style:'
SET n.key = 'style:' + n.key;

// --- ExpressionSet: exprset:{domain}@{locale} ---
// Current key might be "fr-FR" or "greetings_fr-FR"
// We need to extract locale from FOR_LOCALE arc
MATCH (n:ExpressionSet)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'exprset:'
WITH n, l.key AS localeKey,
     CASE
         WHEN n.domain IS NOT NULL THEN n.domain
         ELSE 'default'
     END AS domain
// Extract just the locale code from locale:fr-FR
SET n.key = 'exprset:' + domain + '@' + replace(localeKey, 'locale:', '');

// Fallback for orphan ExpressionSets (no FOR_LOCALE arc)
MATCH (n:ExpressionSet)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'exprset:'
SET n.key = 'exprset:default@' + n.key;

// --- PatternSet: patternset:{domain}@{locale} ---
MATCH (n:PatternSet)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'patternset:'
WITH n, l.key AS localeKey,
     CASE WHEN n.domain IS NOT NULL THEN n.domain ELSE 'default' END AS domain
SET n.key = 'patternset:' + domain + '@' + replace(localeKey, 'locale:', '');

MATCH (n:PatternSet)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'patternset:'
SET n.key = 'patternset:default@' + n.key;

// --- CultureSet: cultureset:{domain}@{locale} ---
MATCH (n:CultureSet)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'cultureset:'
WITH n, l.key AS localeKey,
     CASE WHEN n.domain IS NOT NULL THEN n.domain ELSE 'default' END AS domain
SET n.key = 'cultureset:' + domain + '@' + replace(localeKey, 'locale:', '');

MATCH (n:CultureSet)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'cultureset:'
SET n.key = 'cultureset:default@' + n.key;

// --- TabooSet: tabooset:{domain}@{locale} ---
MATCH (n:TabooSet)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'tabooset:'
WITH n, l.key AS localeKey,
     CASE WHEN n.domain IS NOT NULL THEN n.domain ELSE 'default' END AS domain
SET n.key = 'tabooset:' + domain + '@' + replace(localeKey, 'locale:', '');

MATCH (n:TabooSet)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'tabooset:'
SET n.key = 'tabooset:default@' + n.key;

// --- AudienceSet: audienceset:{domain}@{locale} ---
MATCH (n:AudienceSet)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'audienceset:'
WITH n, l.key AS localeKey,
     CASE WHEN n.domain IS NOT NULL THEN n.domain ELSE 'default' END AS domain
SET n.key = 'audienceset:' + domain + '@' + replace(localeKey, 'locale:', '');

MATCH (n:AudienceSet)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'audienceset:'
SET n.key = 'audienceset:default@' + n.key;

// --- Verification ---
// MATCH (n) WHERE labels(n)[0] IN ['Slugification', 'Formatting', 'Culture', 'Adaptation', 'Style',
//   'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet']
//   AND NOT n.key CONTAINS ':'
// RETURN labels(n)[0], count(n);
