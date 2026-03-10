// ============================================================================
// MIGRATION 079: Fix EntityNative HAS_NATIVE Arcs
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Create missing HAS_NATIVE arcs from Entity to EntityNative
//
// Issue: EntityNative.entity_key = "qr-code" but Entity.key = "entity:qr-code"
// Fix: Match using prefix and create HAS_NATIVE arcs
// ============================================================================

// Step 1: Create HAS_NATIVE arcs using entity_key (add prefix)
MATCH (en:EntityNative)
WHERE NOT EXISTS { MATCH (:Entity)-[:HAS_NATIVE]->(en) }
  AND en.entity_key IS NOT NULL
WITH en, 'entity:' + en.entity_key AS full_entity_key
MATCH (e:Entity {key: full_entity_key})
MERGE (e)-[:HAS_NATIVE]->(en)
RETURN count(*) AS has_native_arcs_created;

// Step 2: Update entity_key to include prefix for consistency
MATCH (en:EntityNative)
WHERE en.entity_key IS NOT NULL
  AND NOT en.entity_key STARTS WITH 'entity:'
SET en.entity_key = 'entity:' + en.entity_key
RETURN count(en) AS entity_keys_updated;

// Step 3: Also create FOR_LOCALE arcs if missing
MATCH (en:EntityNative)
WHERE NOT EXISTS { MATCH (en)-[:FOR_LOCALE]->(:Locale) }
  AND en.key CONTAINS '@'
WITH en, split(en.key, '@') AS parts
WITH en, parts[size(parts)-1] AS locale_code
MATCH (l:Locale {key: locale_code})
MERGE (en)-[:FOR_LOCALE]->(l)
RETURN count(*) AS for_locale_arcs_created;

// Verification query
MATCH (en:EntityNative)
OPTIONAL MATCH (e:Entity)-[:HAS_NATIVE]->(en)
OPTIONAL MATCH (en)-[:FOR_LOCALE]->(l:Locale)
RETURN en.key AS native_key,
       e IS NOT NULL AS has_entity_parent,
       l IS NOT NULL AS has_locale
ORDER BY en.key;
