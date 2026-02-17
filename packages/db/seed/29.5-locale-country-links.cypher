// =============================================================================
// LOCALE → COUNTRY RELATIONSHIPS - NovaNet v0.12.4
// =============================================================================
// Creates IN_COUNTRY relationships from Locale to Country using country_code.
//
// v0.12.4: ADR-028 - Page-Entity Architecture (Geographic Hierarchy)
//   - Locale.country_code (e.g., "FR") → Country.key (e.g., "FR")
//   - Completes hierarchy: Locale → Country → GeoRegion → Continent
// =============================================================================

// ─── Create IN_COUNTRY Relationships ────────────────────────────────────────

// Match all Locales with their corresponding Countries by country_code
// The Locale.country_code property matches the Country.key (ISO alpha-2)
MATCH (l:Locale)
WHERE l.country_code IS NOT NULL
MATCH (c:Country {key: l.country_code})
MERGE (l)-[:IN_COUNTRY]->(c);

// ─── Verification Query ─────────────────────────────────────────────────────

// Run this after seeding to verify relationships:
// MATCH (l:Locale)-[:IN_COUNTRY]->(c:Country)-[:IN_REGION]->(r:GeoRegion)-[:IN_CONTINENT]->(cont:Continent)
// RETURN l.key AS locale, c.key AS country, r.key AS region, cont.key AS continent
// ORDER BY l.key LIMIT 10;
