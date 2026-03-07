// Migration 059: Create TARGETS_ENTITY arcs for SEOKeywords
// Links SEOKeyword to Entity based on key pattern matching
// Pattern: seo:<keyword>@<locale> → entity:<slug>
//
// Purpose: Connect SEO keywords to their target semantic entities
// This enables traversal from keyword discovery to entity context
//
// Strategy:
//   1. Direct match: seo:X@locale → entity:X (exact slug match)
//   2. Fuzzy match: seo:action-X@locale → entity:X (keyword contains entity slug)

// ═══════════════════════════════════════════════════════════════════════════════
// STRATEGY 1: Direct match
// seo:qr-code@en-US → entity:qr-code
// ═══════════════════════════════════════════════════════════════════════════════
MATCH (sk:SEOKeyword)
WHERE sk.key STARTS WITH "seo:"
WITH sk,
     // Extract slug from SEOKeyword key: seo:<slug>@<locale> → <slug>
     split(replace(sk.key, "seo:", ""), "@")[0] AS keyword_slug,
     // Build target Entity key: entity:<slug>
     "entity:" + split(replace(sk.key, "seo:", ""), "@")[0] AS direct_target_key
MATCH (e:Entity {key: direct_target_key})
MERGE (sk)-[:TARGETS_ENTITY]->(e)
WITH count(*) AS direct_matches
RETURN "Strategy 1 (Direct): " + direct_matches + " arcs created" AS result;

// ═══════════════════════════════════════════════════════════════════════════════
// STRATEGY 2: Fuzzy match - keyword contains entity slug
// e.g., seo:create-qr-code@en-US → entity:qr-code
// e.g., seo:free-qr-code-generator@fr-FR → entity:qr-code
// ═══════════════════════════════════════════════════════════════════════════════
MATCH (sk:SEOKeyword)
WHERE NOT (sk)-[:TARGETS_ENTITY]->(:Entity)
  AND sk.key STARTS WITH "seo:"
WITH sk,
     // Extract slug from SEOKeyword key
     split(replace(sk.key, "seo:", ""), "@")[0] AS keyword_slug
// Find Entity whose slug is contained in the keyword slug
MATCH (e:Entity)
WHERE e.key STARTS WITH "entity:"
WITH sk, e, keyword_slug,
     replace(e.key, "entity:", "") AS entity_slug
WHERE keyword_slug CONTAINS entity_slug
  AND size(entity_slug) >= 3  // Avoid matching very short entity slugs
// Prefer longer entity slugs (more specific matches)
WITH sk, e, entity_slug,
     size(entity_slug) AS entity_slug_length
ORDER BY entity_slug_length DESC
// Take the best match (longest entity slug that fits)
WITH sk, collect(e)[0] AS best_match
WHERE best_match IS NOT NULL
MERGE (sk)-[:TARGETS_ENTITY]->(best_match)
WITH count(*) AS fuzzy_matches
RETURN "Strategy 2 (Fuzzy): " + fuzzy_matches + " arcs created" AS result;

// ═══════════════════════════════════════════════════════════════════════════════
// SUMMARY: Total arcs created
// ═══════════════════════════════════════════════════════════════════════════════
MATCH (sk:SEOKeyword)-[r:TARGETS_ENTITY]->(e:Entity)
WITH count(r) AS total_arcs
MATCH (sk:SEOKeyword) WHERE NOT (sk)-[:TARGETS_ENTITY]->(:Entity)
WITH total_arcs, count(sk) AS unlinked
RETURN "Summary: " + total_arcs + " TARGETS_ENTITY arcs total, " + unlinked + " keywords still unlinked" AS summary;
