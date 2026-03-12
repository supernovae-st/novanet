// ============================================================================
// Migration 116: Key Prefix - Tier 1 (Locale, Country, Continent)
// ============================================================================
// These nodes have no dependencies - safe to migrate first.
// Pattern: {prefix}:{identifier}
//
// Locale:    locale:fr-FR (BCP-47)
// Country:   country:FR (ISO 3166-1 alpha-2)
// Continent: continent:EU (standard codes)
// ============================================================================

// --- Locale: locale:{bcp47} ---
// Current: "fr-FR" → Target: "locale:fr-FR"
MATCH (n:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'locale:'
SET n.key = 'locale:' + n.key;

// --- Country: country:{iso2} ---
// Current: "FR" or "France" → Target: "country:FR"
// Use iso2 if available, otherwise first 2 chars uppercase
MATCH (n:Country)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'country:'
WITH n,
     CASE
         WHEN n.iso2 IS NOT NULL THEN n.iso2
         WHEN size(n.key) = 2 THEN toUpper(n.key)
         ELSE toUpper(left(n.key, 2))
     END AS iso2Code
SET n.key = 'country:' + iso2Code;

// --- Continent: continent:{code} ---
// Current: "EU" or "Europe" → Target: "continent:EU"
MATCH (n:Continent)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'continent:'
WITH n,
     CASE n.key
         WHEN 'Africa' THEN 'AF'
         WHEN 'Antarctica' THEN 'AN'
         WHEN 'Asia' THEN 'AS'
         WHEN 'Europe' THEN 'EU'
         WHEN 'North America' THEN 'NA'
         WHEN 'Oceania' THEN 'OC'
         WHEN 'South America' THEN 'SA'
         ELSE toUpper(left(n.key, 2))
     END AS code
SET n.key = 'continent:' + code;

// --- Verification ---
// MATCH (n:Locale) WHERE NOT n.key STARTS WITH 'locale:' RETURN count(n);
// MATCH (n:Country) WHERE NOT n.key STARTS WITH 'country:' RETURN count(n);
// MATCH (n:Continent) WHERE NOT n.key STARTS WITH 'continent:' RETURN count(n);
// All should return 0.
