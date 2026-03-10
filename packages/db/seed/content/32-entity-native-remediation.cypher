// packages/db/seed/32-entity-native-remediation.cypher
// v0.13.0 - EntityNative Data Quality Remediation
//
// Fixes:
// 1. EntityNative with NULL locale (275 nodes) → extract from key
// 2. Entity missing classification → link to EntityCategory via BELONGS_TO
// 3. EntityNative missing title → derive from Entity.display_name
// 4. TARGETS arcs missing semantic_coef → set default 1.0
//
// Valid EntityCategory keys (from entity-category.yaml):
//   THING, CONTENT_TYPE, ACTION, FEATURE, TOOL, BENEFIT

// ============================================================================
// 1. FIX NULL LOCALE ON EntityNative (extract from composite key)
// ============================================================================
// Keys are in format: entity:{entity_key}@{locale}

MATCH (en:EntityNative)
WHERE en.locale IS NULL AND en.key CONTAINS '@'
WITH en, split(en.key, '@')[1] AS extracted_locale
SET en.locale = extracted_locale
;

// ============================================================================
// 2. CLASSIFY Entity VIA BELONGS_TO → EntityCategory
// ============================================================================
// Pattern: Match entities by key pattern, link to appropriate category

// TOOL = generators, builders, scanners, API endpoints
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
  AND (e.key ENDS WITH '-generator'
    OR e.key ENDS WITH '-builder'
    OR e.key ENDS WITH '-scanner'
    OR e.key ENDS WITH '-api'
    OR e.key = 'api'
    OR e.key = 'url-shortener'
    OR e.key = 'utm-builder')
WITH e
MATCH (cat:EntityCategory {key: 'TOOL'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// ACTION = verbs (create, customize, download, share, scan, print, edit, add, track)
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
  AND (e.key STARTS WITH 'create-'
    OR e.key STARTS WITH 'customize-'
    OR e.key STARTS WITH 'download-'
    OR e.key STARTS WITH 'share-'
    OR e.key STARTS WITH 'scan-'
    OR e.key STARTS WITH 'print-'
    OR e.key STARTS WITH 'edit-'
    OR e.key STARTS WITH 'add-'
    OR e.key STARTS WITH 'track-'
    OR e.key STARTS WITH 'shorten-'
    OR e.key = 'change-colors'
    OR e.key = 'bulk-creation')
WITH e
MATCH (cat:EntityCategory {key: 'ACTION'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// CONTENT_TYPE = educational content (how-to, vs, comparison, guide suffix)
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
  AND (e.key STARTS WITH 'how-to-'
    OR e.key ENDS WITH '-guide'
    OR e.key CONTAINS '-vs-')
WITH e
MATCH (cat:EntityCategory {key: 'CONTENT_TYPE'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// FEATURE = product capabilities
// Note: dynamic-qr-code and static-qr-code are QR code TYPES (THING), not features
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
  AND e.key IN [
    'analytics', 'password-protection',
    'click-tracking', 'geo-tracking', 'device-detection', 'contextual-routing',
    'custom-domain-name', 'custom-link-preview', 'retargeting-pixel',
    'team-workspaces', 'white-label', 'webhooks', 'expiration', 'scan-limit',
    'scan-counting', 'time-series', 'data-capacity', 'error-correction',
    'booking-appointment', 'event-rsvp', 'link-in-bio'
  ]
WITH e
MATCH (cat:EntityCategory {key: 'FEATURE'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// INTEGRATION = external service integrations
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
  AND e.key ENDS WITH '-integration'
WITH e
MATCH (cat:EntityCategory {key: 'INTEGRATION'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// THING = QR code types (the created thing)
// Includes: qr-code-*, dynamic-qr-code, static-qr-code, barcode formats
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
  AND (e.key STARTS WITH 'qr-code-'
    OR e.key IN [
      'qr-code', 'smart-link', 'landing-page', 'barcode', 'short-link',
      'custom-qr-code', 'dynamic-qr-code', 'static-qr-code',
      'aztec-code', 'data-matrix', 'pdf417', 'maxicode',
      'codabar', 'code-128', 'code-39', 'gs1-128', 'gs1-datamatrix',
      'ean-13', 'ean-8', 'upc-a', 'upc-e', 'itf-14', 'msi-plessey'
    ])
WITH e
MATCH (cat:EntityCategory {key: 'THING'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// THING = print media types
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
  AND e.key IN [
    'business-cards', 'flyers', 'posters-billboards', 'brochures',
    'magazines', 'newspapers', 'catalogs', 'stickers-labels',
    'product-labels', 'product-packaging', 'table-tents', 'menus-printed',
    'banners', 'direct-mail', 'receipts', 'tickets-physical',
    'presentations', 'documents', 'emails', 'websites', 'forms'
  ]
WITH e
MATCH (cat:EntityCategory {key: 'THING'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// THING = technical components (module, finder, timing, quiet zone, version)
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
  AND e.key IN [
    'module', 'finder-pattern', 'timing-pattern', 'quiet-zone',
    'encoding-mode', 'qr-code-version', 'menu-restaurant'
  ]
WITH e
MATCH (cat:EntityCategory {key: 'THING'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// THING = classification groupings (style, content, format, type, mode, pattern)
// These are conceptual things, not categories in the EntityCategory sense
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
  AND (e.key ENDS WITH '-style'
    OR e.key ENDS WITH '-content'
    OR e.key ENDS WITH '-format'
    OR e.key ENDS WITH '-type'
    OR e.key ENDS WITH '-mode'
    OR e.key ENDS WITH '-pattern')
WITH e
MATCH (cat:EntityCategory {key: 'THING'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// Catch-all: remaining unclassified → THING (safe default)
MATCH (e:Entity)
WHERE NOT EXISTS { (e)-[:BELONGS_TO]->(:EntityCategory) }
WITH e
MATCH (cat:EntityCategory {key: 'THING'})
MERGE (e)-[:BELONGS_TO]->(cat)
;

// ============================================================================
// 3. SET title ON EntityNative FROM Entity.display_name (if NULL)
// ============================================================================
// Note: For fr-FR, these would ideally be French translations,
// but Entity.display_name is English. A future pass can translate.

MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
WHERE en.title IS NULL AND e.display_name IS NOT NULL
SET en.title = e.display_name
;

// ============================================================================
// 4. FIX TARGETS ARCS MISSING semantic_coef (default = 1.0 for primary)
// ============================================================================

MATCH (en:EntityNative)-[t:TARGETS]->(k:SEOKeyword)
WHERE t.semantic_coef IS NULL
SET t.semantic_coef = CASE
  WHEN t.rank = 'primary' THEN 1.0
  WHEN t.rank = 'secondary' THEN 0.9
  ELSE 1.0
END
;

// ============================================================================
// 5. FIX TARGETS ARCS MISSING rank (default = 'primary')
// ============================================================================

MATCH (en:EntityNative)-[t:TARGETS]->(k:SEOKeyword)
WHERE t.rank IS NULL
SET t.rank = 'primary'
;

// ============================================================================
// 6. CONNECT EntityNative TO Locale (FOR_LOCALE arc)
// ============================================================================

MATCH (en:EntityNative)
WHERE en.locale IS NOT NULL
MATCH (l:Locale {key: en.locale})
MERGE (en)-[:FOR_LOCALE]->(l)
;
