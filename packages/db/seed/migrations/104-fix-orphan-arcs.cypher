// ============================================================================
// Migration 104: Fix orphan FOR_LOCALE and HAS_NATIVE arcs
// ============================================================================
// Fixes CSR issues identified in audit:
// 1. Pattern/CultureRef/Taboo/AudienceTrait missing FOR_LOCALE arcs
// 2. EntityNative missing HAS_NATIVE arcs (create Entities if needed)
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

// --- Create missing Entity nodes (9 core entities) ---
MERGE (e:Entity {key: 'entity:qr-code'})
SET e.display_name = 'QR Code',
    e.description = 'Two-dimensional barcode storing data as square pattern',
    e.entity_category = 'THING',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'entity:dynamic-qr-code'})
SET e.display_name = 'Dynamic QR Code',
    e.description = 'Editable QR code with analytics tracking',
    e.entity_category = 'THING',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'entity:static-qr-code'})
SET e.display_name = 'Static QR Code',
    e.description = 'Fixed QR code with encoded data',
    e.entity_category = 'THING',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'entity:custom-qr-code'})
SET e.display_name = 'Custom QR Code',
    e.description = 'QR code with configurable visual elements',
    e.entity_category = 'THING',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'entity:qr-code-art'})
SET e.display_name = 'QR Code Art',
    e.description = 'AI-generated artistic QR code',
    e.entity_category = 'THING',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'entity:qr-code-generator'})
SET e.display_name = 'QR Code Generator',
    e.description = 'Primary QR code creation tool',
    e.entity_category = 'TOOL',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'entity:barcode'})
SET e.display_name = 'Barcode',
    e.description = 'One-dimensional linear barcode formats',
    e.entity_category = 'THING',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'entity:smart-link'})
SET e.display_name = 'Smart Link',
    e.description = 'Intelligent shortened URL with conditional routing',
    e.entity_category = 'FEATURE',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'entity:landing-page'})
SET e.display_name = 'Landing Page',
    e.description = 'Mobile-optimized destination page',
    e.entity_category = 'FEATURE',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// --- Link Entities to EntityCategory ---
MATCH (e:Entity), (ec:EntityCategory {key: e.entity_category})
MERGE (e)-[:BELONGS_TO_CATEGORY]->(ec);

// --- Fix HAS_NATIVE arcs for EntityNative nodes ---
MATCH (en:EntityNative)
WHERE NOT (:Entity)-[:HAS_NATIVE]->(en)
WITH en, split(en.key, '@')[0] AS entitySlug
MATCH (e:Entity {key: 'entity:' + entitySlug})
MERGE (e)-[:HAS_NATIVE]->(en);

// --- Fix FOR_LOCALE arcs for EntityNative nodes ---
MATCH (en:EntityNative)
WHERE NOT (en)-[:FOR_LOCALE]->(:Locale)
AND en.locale IS NOT NULL
MATCH (l:Locale {key: en.locale})
MERGE (en)-[:FOR_LOCALE]->(l);
