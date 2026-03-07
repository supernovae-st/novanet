// Migration 056: Create TARGETS_KEYWORD arcs for SEOKeywords
// Links SEOKeyword to EntityNative based on key pattern matching
// Pattern: seo:<keyword>@<locale> → entity:<keyword>@<locale>

// Strategy 1: Direct match (seo:X@locale → entity:X@locale)
MATCH (sk:SEOKeyword)
WHERE sk.key STARTS WITH "seo:"
WITH sk,
     replace(sk.key, "seo:", "entity:") AS direct_target_key
MATCH (en:EntityNative {key: direct_target_key})
MERGE (sk)-[:TARGETS_KEYWORD]->(en)
WITH count(*) AS direct_matches
RETURN "Direct matches created: " + direct_matches AS result;

// Strategy 2: Fuzzy match - link keywords to their primary entity
// e.g., seo:create-qr-code@en-US → entity:qr-code@en-US
MATCH (sk:SEOKeyword)
WHERE NOT (sk)-[:TARGETS_KEYWORD]->(:EntityNative)
WITH sk,
     split(sk.key, "@")[1] AS locale,
     split(replace(sk.key, "seo:", ""), "@")[0] AS keyword_slug
// Find EntityNative with matching locale that the keyword is about
MATCH (en:EntityNative)
WHERE en.key ENDS WITH ("@" + locale)
  AND keyword_slug CONTAINS split(replace(en.key, "entity:", ""), "@")[0]
WITH sk, en, keyword_slug,
     size(split(replace(en.key, "entity:", ""), "@")[0]) AS entity_slug_length
ORDER BY entity_slug_length DESC
WITH sk, collect(en)[0] AS best_match
WHERE best_match IS NOT NULL
MERGE (sk)-[:TARGETS_KEYWORD]->(best_match)
RETURN count(*) AS fuzzy_matches_created;

// Final summary
MATCH (sk:SEOKeyword)-[r:TARGETS_KEYWORD]->(en:EntityNative)
RETURN count(r) AS total_arcs_created;
