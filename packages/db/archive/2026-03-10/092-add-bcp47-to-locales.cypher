// ============================================================================
// PLAN B - Migration 092: Add BCP47 Property to Locale Nodes
// ============================================================================
// Priority: STRUCTURE (Locale identification standard)
// Fixes: 203 Locale nodes missing BCP47 property
// CSR Impact: Enables standard locale identification across systems
// Note: BCP47 is same as key for our locales (xx-XX format)
// ============================================================================

// Add bcp47 property to all locales (mirrors key for standard compliance)
MATCH (l:Locale)
WHERE l.bcp47 IS NULL
SET l.bcp47 = l.key;

// Also ensure language_code and country_code are set from key if missing
MATCH (l:Locale)
WHERE l.language_code IS NULL AND l.key =~ '^[a-z]{2}-[A-Z]{2}$'
SET l.language_code = substring(l.key, 0, 2);

MATCH (l:Locale)
WHERE l.country_code IS NULL AND l.key =~ '^[a-z]{2}-[A-Z]{2}$'
SET l.country_code = substring(l.key, 3, 2);

// Verify BCP47 coverage
MATCH (l:Locale)
WITH count(*) AS total,
     count(l.bcp47) AS with_bcp47,
     count(l.language_code) AS with_language_code,
     count(l.country_code) AS with_country_code
RETURN total,
       with_bcp47,
       with_language_code,
       with_country_code,
       CASE WHEN with_bcp47 = total THEN 'COMPLETE' ELSE 'INCOMPLETE' END AS bcp47_status;

// Sample verification
MATCH (l:Locale)
RETURN l.key, l.bcp47, l.language_code, l.country_code
ORDER BY l.key
LIMIT 10;
