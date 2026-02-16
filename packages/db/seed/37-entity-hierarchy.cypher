// packages/db/seed/37-entity-hierarchy.cypher
// v0.13.0 - HAS_CHILD arcs: Entity hierarchy
//
// ADR-030: Entity owns semantic hierarchy
// ADR-032: URL Slugification uses hierarchy for no-repetition rule
//
// HAS_CHILD creates a semantic hierarchy for entities.
// This enables:
// 1. URL no-repetition: child.slug excludes parent terms
// 2. Pillar/cluster content strategy
// 3. Spreading activation for context loading
//
// Hierarchy structure:
// - qr-code (ROOT)
//   └── qr-code-generator (PILLAR)
//       └── qr-code-wifi, qr-code-instagram, etc. (SUBTOPICS)
//   └── qr-code-scanner
//   └── dynamic-qr-code
//   └── static-qr-code
//
// - barcode (ROOT)
//   └── barcode-generator
//   └── barcode-scanner
//   └── aztec-code, data-matrix, pdf417, etc.

// ============================================================================
// 1. QR CODE ROOT HIERARCHY
// ============================================================================

// qr-code → top-level children
MATCH (parent:Entity {key: 'qr-code'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-generator',
  'qr-code-scanner',
  'dynamic-qr-code',
  'static-qr-code',
  'custom-qr-code'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.created_at = datetime();

// ============================================================================
// 2. QR CODE GENERATOR → QR CODE TYPES (PILLAR → SUBTOPICS)
// ============================================================================

// qr-code-generator → social media QR codes
MATCH (parent:Entity {key: 'qr-code-generator'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-instagram',
  'qr-code-facebook',
  'qr-code-tiktok',
  'qr-code-youtube',
  'qr-code-linkedin',
  'qr-code-twitter',
  'qr-code-pinterest',
  'qr-code-snapchat',
  'qr-code-whatsapp',
  'qr-code-telegram'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.cluster = 'social-media',
    r.created_at = datetime();

// qr-code-generator → utility QR codes
MATCH (parent:Entity {key: 'qr-code-generator'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-wifi',
  'qr-code-vcard',
  'qr-code-menu',
  'qr-code-url',
  'qr-code-email',
  'qr-code-sms',
  'qr-code-phone',
  'qr-code-text',
  'qr-code-calendar',
  'qr-code-location'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.cluster = 'utility',
    r.created_at = datetime();

// qr-code-generator → media QR codes
MATCH (parent:Entity {key: 'qr-code-generator'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-pdf',
  'qr-code-image',
  'qr-code-video',
  'qr-code-audio',
  'qr-code-file',
  'qr-code-image-gallery'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.cluster = 'media',
    r.created_at = datetime();

// qr-code-generator → music platform QR codes
MATCH (parent:Entity {key: 'qr-code-generator'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-spotify',
  'qr-code-soundcloud',
  'qr-code-apple-music'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.cluster = 'music',
    r.created_at = datetime();

// qr-code-generator → payment QR codes
MATCH (parent:Entity {key: 'qr-code-generator'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-payment',
  'qr-code-paypal',
  'qr-code-venmo',
  'qr-code-bitcoin',
  'qr-code-ethereum',
  'qr-code-crypto',
  'qr-code-pix',
  'qr-code-upi',
  'qr-code-bank-transfer',
  'qr-code-contactless-payment'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.cluster = 'payment',
    r.created_at = datetime();

// qr-code-generator → app store QR codes
MATCH (parent:Entity {key: 'qr-code-generator'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-app',
  'qr-code-app-download',
  'qr-code-app-store',
  'qr-code-play-store'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.cluster = 'app-store',
    r.created_at = datetime();

// qr-code-generator → maps QR codes
MATCH (parent:Entity {key: 'qr-code-generator'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-google-maps',
  'qr-code-apple-maps',
  'qr-code-waze',
  'qr-code-coordinates'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.cluster = 'maps',
    r.created_at = datetime();

// qr-code-generator → business QR codes
MATCH (parent:Entity {key: 'qr-code-generator'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-business-card',
  'qr-code-review',
  'qr-code-feedback',
  'qr-code-survey',
  'qr-code-coupon',
  'qr-code-loyalty-program',
  'qr-code-event-checkin',
  'qr-code-attendance',
  'qr-code-ticket',
  'qr-code-certificate'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.cluster = 'business',
    r.created_at = datetime();

// ============================================================================
// 3. ACTIONS (CREATE, CUSTOMIZE, SCAN, etc.)
// ============================================================================

// create-qr-code → related actions
MATCH (parent:Entity {key: 'create-qr-code'})
MATCH (child:Entity)
WHERE child.key IN [
  'customize-qr-code',
  'download-qr-code',
  'share-qr-code',
  'print-qr-code'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.created_at = datetime();

// ============================================================================
// 4. BARCODE HIERARCHY
// ============================================================================

// barcode → barcode types
MATCH (parent:Entity {key: 'barcode'})
MATCH (child:Entity)
WHERE child.key IN [
  'barcode-generator',
  'barcode-scanner',
  'aztec-code',
  'data-matrix',
  'pdf417',
  'maxicode',
  'codabar',
  'code-128',
  'code-39',
  'gs1-128',
  'gs1-datamatrix',
  'ean-13',
  'ean-8',
  'upc-a',
  'upc-e',
  'itf-14',
  'msi-plessey'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.created_at = datetime();

// ============================================================================
// 5. URL SHORTENING HIERARCHY
// ============================================================================

// smart-link → related products
MATCH (parent:Entity {key: 'smart-link'})
MATCH (child:Entity)
WHERE child.key IN [
  'short-link',
  'url-shortener',
  'shorten-url'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.created_at = datetime();

// ============================================================================
// 6. LANDING PAGE HIERARCHY
// ============================================================================

// landing-page → related products
MATCH (parent:Entity {key: 'landing-page'})
MATCH (child:Entity)
WHERE child.key IN [
  'landing-page-builder',
  'link-in-bio',
  'link-in-bio-builder',
  'menu-builder'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.created_at = datetime();

// ============================================================================
// 7. DESIGN/CUSTOMIZATION HIERARCHY
// ============================================================================

// custom-qr-code → design options
MATCH (parent:Entity {key: 'custom-qr-code'})
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-with-logo',
  'qr-code-color',
  'qr-code-shapes',
  'qr-code-frame',
  'qr-code-style',
  'qr-code-background',
  'qr-code-background-color',
  'qr-code-background-gradient',
  'qr-code-background-image',
  'qr-code-transparent-background',
  'qr-code-dark-mode',
  'qr-code-light-mode'
]
MERGE (parent)-[r:HAS_CHILD]->(child)
SET r.semantic_distance = 1,
    r.cluster = 'design',
    r.created_at = datetime();

// ============================================================================
// 8. VERIFICATION QUERY
// ============================================================================
// Run this to verify HAS_CHILD arcs were created:
//
// MATCH (parent:Entity)-[r:HAS_CHILD]->(child:Entity)
// RETURN parent.key AS parent, count(child) AS children, collect(child.key)[0..5] AS sample
// ORDER BY children DESC;
//
