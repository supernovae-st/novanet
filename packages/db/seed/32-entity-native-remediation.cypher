// packages/db/seed/32-entity-native-remediation.cypher
// v0.13.0 - EntityNative Data Quality Remediation
//
// Fixes:
// 1. EntityNative with NULL locale (275 nodes) → extract from key
// 2. Entity missing entity_type → classify by key pattern
// 3. EntityNative missing title → derive from Entity.display_name
// 4. TARGETS arcs missing semantic_coef → set default 1.0

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
// 2. CLASSIFY Entity BY KEY PATTERN → entity_type
// ============================================================================

// TOOL = generators, builders, scanners, API endpoints
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND (e.key ENDS WITH '-generator'
    OR e.key ENDS WITH '-builder'
    OR e.key ENDS WITH '-scanner'
    OR e.key ENDS WITH '-api'
    OR e.key = 'api'
    OR e.key = 'url-shortener'
    OR e.key = 'utm-builder')
SET e.entity_type = 'TOOL'
;

// ACTION = verbs (create, customize, download, share, scan, print, edit, add, track)
MATCH (e:Entity)
WHERE e.entity_type IS NULL
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
SET e.entity_type = 'ACTION'
;

// GUIDE = educational content (how-to, vs, comparison, guide suffix)
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND (e.key STARTS WITH 'how-to-'
    OR e.key ENDS WITH '-guide'
    OR e.key CONTAINS '-vs-')
SET e.entity_type = 'GUIDE'
;

// BRAND = social platforms and payment providers
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND e.key IN [
    'instagram', 'facebook', 'linkedin', 'twitter', 'pinterest', 'snapchat',
    'tiktok', 'youtube', 'spotify', 'soundcloud', 'telegram', 'whatsapp',
    'apple', 'google', 'paypal', 'venmo', 'waze',
    'hubspot', 'mailchimp', 'salesforce', 'shopify', 'woocommerce',
    'zapier', 'make', 'n8n', 'slack'
  ]
SET e.entity_type = 'BRAND'
;

// INTEGRATION = connector plugins
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND e.key ENDS WITH '-integration'
SET e.entity_type = 'INTEGRATION'
;

// INDUSTRY = business verticals
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND e.key IN [
    'healthcare', 'real-estate', 'restaurants', 'retail', 'education',
    'hospitality', 'manufacturing', 'logistics', 'transportation',
    'entertainment', 'fitness', 'beauty', 'finance', 'government',
    'construction', 'consulting', 'nonprofits', 'agencies',
    'marketing-agencies', 'creative-agencies', 'event-management',
    'small-business', 'enterprise', 'freelancers', 'developers'
  ]
SET e.entity_type = 'INDUSTRY'
;

// CATEGORY = classification groupings
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND (e.key ENDS WITH '-style'
    OR e.key ENDS WITH '-content'
    OR e.key ENDS WITH '-format'
    OR e.key ENDS WITH '-type'
    OR e.key ENDS WITH '-mode'
    OR e.key ENDS WITH '-pattern')
SET e.entity_type = 'CATEGORY'
;

// FEATURE = product capabilities
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND e.key IN [
    'analytics', 'password-protection', 'dynamic-qr-code', 'static-qr-code',
    'click-tracking', 'geo-tracking', 'device-detection', 'contextual-routing',
    'custom-domain-name', 'custom-link-preview', 'retargeting-pixel',
    'team-workspaces', 'white-label', 'webhooks', 'expiration', 'scan-limit',
    'scan-counting', 'time-series', 'data-capacity', 'error-correction',
    'booking-appointment', 'event-rsvp', 'link-in-bio'
  ]
SET e.entity_type = 'FEATURE'
;

// OBJECT = QR code types (the created thing)
// All qr-code-* that aren't already classified as something else
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND e.key STARTS WITH 'qr-code-'
SET e.entity_type = 'OBJECT'
;

// OBJECT = other product outputs (smart-link, landing-page, barcode, short-link)
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND e.key IN [
    'qr-code', 'smart-link', 'landing-page', 'barcode', 'short-link',
    'custom-qr-code', 'dynamic-qr-code', 'static-qr-code'
  ]
SET e.entity_type = 'OBJECT'
;

// OBJECT = barcode formats
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND e.key IN [
    'aztec-code', 'data-matrix', 'pdf417', 'maxicode',
    'codabar', 'code-128', 'code-39', 'gs1-128', 'gs1-datamatrix',
    'ean-13', 'ean-8', 'upc-a', 'upc-e', 'itf-14', 'msi-plessey'
  ]
SET e.entity_type = 'OBJECT'
;

// OBJECT = print media types
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND e.key IN [
    'business-cards', 'flyers', 'posters-billboards', 'brochures',
    'magazines', 'newspapers', 'catalogs', 'stickers-labels',
    'product-labels', 'product-packaging', 'table-tents', 'menus-printed',
    'banners', 'direct-mail', 'receipts', 'tickets-physical',
    'presentations', 'documents', 'emails', 'websites', 'forms'
  ]
SET e.entity_type = 'OBJECT'
;

// OBJECT = technical components (module, finder, timing, quiet zone, version)
MATCH (e:Entity)
WHERE e.entity_type IS NULL
  AND e.key IN [
    'module', 'finder-pattern', 'timing-pattern', 'quiet-zone',
    'encoding-mode', 'qr-code-version', 'menu-restaurant'
  ]
SET e.entity_type = 'OBJECT'
;

// Catch-all: remaining unclassified → OBJECT (safe default)
MATCH (e:Entity)
WHERE e.entity_type IS NULL
SET e.entity_type = 'OBJECT'
;

// ============================================================================
// 3. PROPAGATE entity_type FROM Entity TO EntityNative
// ============================================================================

MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
WHERE en.entity_type IS NULL AND e.entity_type IS NOT NULL
SET en.entity_type = e.entity_type
;

// ============================================================================
// 4. SET title ON EntityNative FROM Entity.display_name (if NULL)
// ============================================================================
// Note: For fr-FR, these would ideally be French translations,
// but Entity.display_name is English. A future pass can translate.

MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
WHERE en.title IS NULL AND e.display_name IS NOT NULL
SET en.title = e.display_name
;

// ============================================================================
// 5. FIX TARGETS ARCS MISSING semantic_coef (default = 1.0 for primary)
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
// 6. FIX TARGETS ARCS MISSING rank (default = 'primary')
// ============================================================================

MATCH (en:EntityNative)-[t:TARGETS]->(k:SEOKeyword)
WHERE t.rank IS NULL
SET t.rank = 'primary'
;

// ============================================================================
// 7. CONNECT EntityNative TO Locale (FOR_LOCALE arc)
// ============================================================================

MATCH (en:EntityNative)
WHERE en.locale IS NOT NULL
MATCH (l:Locale {key: en.locale})
MERGE (en)-[:FOR_LOCALE]->(l)
;
