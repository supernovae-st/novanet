// ============================================================================
// Migration 104: Fix orphan FOR_LOCALE and HAS_NATIVE arcs
// ============================================================================
// Fixes CSR issues identified in audit:
// 1. Pattern/CultureRef/Taboo/AudienceTrait missing FOR_LOCALE arcs
// 2. EntityNative missing HAS_NATIVE arcs
// 3. EntityNative missing FOR_LOCALE arcs
//
// NOTE: Entity creation is handled by 10-entities-bootstrap.cypher
// This migration ONLY fixes missing arcs, not node properties.
// ============================================================================

// --- Fix FOR_LOCALE for Pattern nodes ---
MATCH (p:Pattern)
WHERE p.locale IS NOT NULL
AND NOT (p)-[:FOR_LOCALE]->(:Locale)
MATCH (l:Locale {key: p.locale})
MERGE (p)-[:FOR_LOCALE]->(l);

// --- Fix FOR_LOCALE for CultureRef nodes ---
MATCH (cr:CultureRef)
WHERE cr.locale IS NOT NULL
AND NOT (cr)-[:FOR_LOCALE]->(:Locale)
MATCH (l:Locale {key: cr.locale})
MERGE (cr)-[:FOR_LOCALE]->(l);

// --- Fix FOR_LOCALE for Taboo nodes ---
MATCH (t:Taboo)
WHERE t.locale IS NOT NULL
AND NOT (t)-[:FOR_LOCALE]->(:Locale)
MATCH (l:Locale {key: t.locale})
MERGE (t)-[:FOR_LOCALE]->(l);

// --- Fix FOR_LOCALE for AudienceTrait nodes ---
MATCH (at:AudienceTrait)
WHERE at.locale IS NOT NULL
AND NOT (at)-[:FOR_LOCALE]->(:Locale)
MATCH (l:Locale {key: at.locale})
MERGE (at)-[:FOR_LOCALE]->(l);

// --- Fix HAS_NATIVE arcs for EntityNative nodes ---
MATCH (en:EntityNative)
WHERE NOT (:Entity)-[:HAS_NATIVE]->(en)
WITH en, split(en.key, '@')[0] AS entityKey
MATCH (e:Entity {key: entityKey})
MERGE (e)-[:HAS_NATIVE]->(en);

// --- Fix FOR_LOCALE arcs for EntityNative nodes ---
MATCH (en:EntityNative)
WHERE NOT (en)-[:FOR_LOCALE]->(:Locale)
AND en.locale IS NOT NULL
MATCH (l:Locale {key: en.locale})
MERGE (en)-[:FOR_LOCALE]->(l);
