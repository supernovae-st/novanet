// packages/db/seed/34-semantic-links.cypher
// v0.13.0 - SEMANTIC_LINK Arcs for Entity Relationships
//
// ADR-032 Semantic Coefficients:
//   used_for:     0.95 (TOOL is used for ACTION)
//   creates:      0.85 (TOOL creates OBJECT)
//   variant_of:   0.90 (OBJECT is variant of OBJECT)
//   enables:      0.80 (FEATURE enables ACTION)
//   associated_with: 0.50 (generic association)

// ============================================================================
// 1. TOOL → ACTION (used_for, strength=0.95)
// ============================================================================

// qr-code-generator is used for multiple actions
MATCH (tool:Entity {key: 'qr-code-generator'})
UNWIND ['customize-qr-code', 'download-qr-code', 'print-qr-code', 'share-qr-code', 'add-logo', 'change-colors'] AS action_key
MATCH (action:Entity {key: action_key})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.95, r.temperature = 0.9
;

// barcode-generator → create-barcode
MATCH (tool:Entity {key: 'barcode-generator'})
MATCH (action:Entity {key: 'create-barcode'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.95, r.temperature = 0.9
;

// barcode-scanner → scan-barcode
MATCH (tool:Entity {key: 'barcode-scanner'})
MATCH (action:Entity {key: 'scan-barcode'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.95, r.temperature = 0.9
;

// url-shortener → shorten-url
MATCH (tool:Entity {key: 'url-shortener'})
MATCH (action:Entity {key: 'shorten-url'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.95, r.temperature = 0.9
;

// batch-qr-generator → bulk-creation
MATCH (tool:Entity {key: 'batch-qr-generator'})
MATCH (action:Entity {key: 'bulk-creation'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.95, r.temperature = 0.9
;

// batch-qr-generator also used for create-qr-code
MATCH (tool:Entity {key: 'batch-qr-generator'})
MATCH (action:Entity {key: 'create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.90, r.temperature = 0.85
;

// landing-page-builder → create-landing-page
MATCH (tool:Entity {key: 'landing-page-builder'})
MATCH (action:Entity {key: 'create-landing-page'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.95, r.temperature = 0.9
;

// link-in-bio-builder → create-smart-link
MATCH (tool:Entity {key: 'link-in-bio-builder'})
MATCH (action:Entity {key: 'create-smart-link'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.95, r.temperature = 0.9
;

// vcard-generator → create-qr-code
MATCH (tool:Entity {key: 'vcard-generator'})
MATCH (action:Entity {key: 'create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.90, r.temperature = 0.85
;

// menu-builder → create-qr-code
MATCH (tool:Entity {key: 'menu-builder'})
MATCH (action:Entity {key: 'create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.90, r.temperature = 0.85
;

// utm-builder → track-scans
MATCH (tool:Entity {key: 'utm-builder'})
MATCH (action:Entity {key: 'track-scans'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.90, r.temperature = 0.85
;

// ============================================================================
// 2. TOOL → OBJECT (creates, strength=0.85)
// ============================================================================

// qr-code-generator creates various QR code types
MATCH (tool:Entity {key: 'qr-code-generator'})
UNWIND [
  'qr-code-facebook', 'qr-code-linkedin', 'qr-code-twitter', 'qr-code-youtube',
  'qr-code-spotify', 'qr-code-tiktok', 'qr-code-whatsapp', 'qr-code-telegram',
  'qr-code-email', 'qr-code-sms', 'qr-code-location', 'qr-code-event',
  'qr-code-app-store', 'qr-code-google-play', 'static-qr-code'
] AS object_key
MATCH (obj:Entity {key: object_key})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'creates'}]->(obj)
SET r.strength = 0.85, r.temperature = 0.85
;

// barcode-generator creates barcode types
MATCH (tool:Entity {key: 'barcode-generator'})
UNWIND [
  'code-128', 'code-39', 'ean-13', 'ean-8', 'upc-a', 'upc-e',
  'gs1-128', 'gs1-datamatrix', 'itf-14', 'codabar', 'msi-plessey',
  'pdf417', 'data-matrix', 'aztec-code', 'maxicode'
] AS barcode_key
MATCH (barcode:Entity {key: barcode_key})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'creates'}]->(barcode)
SET r.strength = 0.85, r.temperature = 0.85
;

// vcard-generator creates vcard qr
MATCH (tool:Entity {key: 'vcard-generator'})
MATCH (obj:Entity {key: 'qr-code-vcard'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'creates'}]->(obj)
SET r.strength = 0.95, r.temperature = 0.9
;

// wifi-qr-generator already has link to qr-code-wifi (from 30-entity-semantic-arcs)

// menu-builder creates menu qr
MATCH (tool:Entity {key: 'menu-builder'})
MATCH (obj:Entity {key: 'qr-code-menu'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'creates'}]->(obj)
SET r.strength = 0.95, r.temperature = 0.9
;

// landing-page-builder creates landing-page
MATCH (tool:Entity {key: 'landing-page-builder'})
MATCH (obj:Entity {key: 'landing-page'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'creates'}]->(obj)
SET r.strength = 0.95, r.temperature = 0.9
;

// link-in-bio-builder creates link-in-bio
MATCH (tool:Entity {key: 'link-in-bio-builder'})
MATCH (obj:Entity {key: 'link-in-bio'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'creates'}]->(obj)
SET r.strength = 0.95, r.temperature = 0.9
;

// url-shortener creates short-link
MATCH (tool:Entity {key: 'url-shortener'})
MATCH (obj:Entity {key: 'short-link'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'creates'}]->(obj)
SET r.strength = 0.95, r.temperature = 0.9
;

// ============================================================================
// 3. OBJECT → BRAND (associated_with, strength=0.50)
// ============================================================================

// QR code types associated with their brands
UNWIND [
  {qr: 'qr-code-instagram', brand: 'instagram'},
  {qr: 'qr-code-facebook', brand: 'facebook'},
  {qr: 'qr-code-linkedin', brand: 'linkedin'},
  {qr: 'qr-code-twitter', brand: 'twitter'},
  {qr: 'qr-code-youtube', brand: 'youtube'},
  {qr: 'qr-code-spotify', brand: 'spotify'},
  {qr: 'qr-code-tiktok', brand: 'tiktok'},
  {qr: 'qr-code-whatsapp', brand: 'whatsapp'},
  {qr: 'qr-code-telegram', brand: 'telegram'},
  {qr: 'qr-code-snapchat', brand: 'snapchat'},
  {qr: 'qr-code-pinterest', brand: 'pinterest'},
  {qr: 'qr-code-soundcloud', brand: 'soundcloud'}
] AS pair
MATCH (qr:Entity {key: pair.qr})
MATCH (brand:Entity {key: pair.brand})
MERGE (qr)-[r:SEMANTIC_LINK {type: 'associated_with'}]->(brand)
SET r.strength = 0.50, r.temperature = 0.7
;

// ============================================================================
// 4. FEATURE → ACTION (enables, strength=0.80)
// ============================================================================

// Analytics enables track-scans
MATCH (feature:Entity {key: 'analytics'})
MATCH (action:Entity {key: 'track-scans'})
MERGE (feature)-[r:SEMANTIC_LINK {type: 'enables'}]->(action)
SET r.strength = 0.80, r.temperature = 0.8
;

// Dynamic QR enables edit-destination
MATCH (feature:Entity {key: 'dynamic-qr-code'})
MATCH (action:Entity {key: 'edit-destination'})
MERGE (feature)-[r:SEMANTIC_LINK {type: 'enables'}]->(action)
SET r.strength = 0.85, r.temperature = 0.85
;

// Password protection (if exists)
MATCH (feature:Entity {key: 'password-protection'})
MATCH (action:Entity {key: 'scan-limit'})
MERGE (feature)-[r:SEMANTIC_LINK {type: 'enables'}]->(action)
SET r.strength = 0.80, r.temperature = 0.8
;

// ============================================================================
// 5. OBJECT variant_of relationships (strength=0.90)
// ============================================================================

// Dynamic vs static QR codes
MATCH (dynamic:Entity {key: 'dynamic-qr-code'})
MATCH (static:Entity {key: 'static-qr-code'})
MERGE (dynamic)-[r:SEMANTIC_LINK {type: 'variant_of'}]->(static)
SET r.strength = 0.90, r.temperature = 0.85,
    r.llm_context = 'Dynamic QR is an editable variant of static QR'
;

// Custom QR is variant of QR code
MATCH (custom:Entity {key: 'custom-qr-code'})
MATCH (base:Entity {key: 'qr-code'})
MERGE (custom)-[r:SEMANTIC_LINK {type: 'variant_of'}]->(base)
SET r.strength = 0.90, r.temperature = 0.85
;

// ============================================================================
// 6. INDUSTRY → OBJECT (uses pattern, enables convergence)
// ============================================================================

// Restaurants use menu QR
MATCH (industry:Entity {key: 'restaurants'})
MATCH (obj:Entity {key: 'qr-code-menu'})
MERGE (industry)-[r:SEMANTIC_LINK {type: 'associated_with'}]->(obj)
SET r.strength = 0.70, r.temperature = 0.8,
    r.llm_context = 'Restaurant industry commonly uses menu QR codes'
;

// Healthcare uses vCard QR
MATCH (industry:Entity {key: 'healthcare'})
MATCH (obj:Entity {key: 'qr-code-vcard'})
MERGE (industry)-[r:SEMANTIC_LINK {type: 'associated_with'}]->(obj)
SET r.strength = 0.60, r.temperature = 0.7
;

// Real estate uses vCard and location QR
MATCH (industry:Entity {key: 'real-estate'})
UNWIND ['qr-code-vcard', 'qr-code-location'] AS obj_key
MATCH (obj:Entity {key: obj_key})
MERGE (industry)-[r:SEMANTIC_LINK {type: 'associated_with'}]->(obj)
SET r.strength = 0.60, r.temperature = 0.7
;

// Retail uses product/payment QR
MATCH (industry:Entity {key: 'retail'})
MATCH (obj:Entity {key: 'qr-code-url'})
MERGE (industry)-[r:SEMANTIC_LINK {type: 'associated_with'}]->(obj)
SET r.strength = 0.60, r.temperature = 0.7
;

// Event management uses event QR
MATCH (industry:Entity {key: 'event-management'})
MATCH (obj:Entity {key: 'qr-code-event'})
MERGE (industry)-[r:SEMANTIC_LINK {type: 'associated_with'}]->(obj)
SET r.strength = 0.80, r.temperature = 0.85
;

