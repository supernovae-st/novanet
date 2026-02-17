// packages/db/seed/39-semantic-link-enrichment.cypher
// v0.13.0 - SEMANTIC_LINK enrichment with additional types
//
// ADR-032: URL Slugification - semantic coefficients for slug scoring
//
// Completes the SEMANTIC_LINK types beyond used_for (from seed 30):
// - type_of:      0.95 — X is a type of Y (taxonomy)
// - includes:     0.85 — X includes Y (composition)
// - requires:     0.70 — X requires Y (dependency)
// - exhibits:     0.80 — X exhibits/shows Y (feature display)
// - contrasts:    0.50 — X contrasts with Y (opposition)
// - is_action_on: 0.90 — X is an action on Y (verb→noun)
// - part_of:      0.85 — X is part of Y (component)

// ============================================================================
// 1. TYPE_OF ARCS — Taxonomy hierarchy (0.95)
// ============================================================================
// QR code types are TYPE_OF qr-code

MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-wifi', 'qr-code-vcard', 'qr-code-url', 'qr-code-text',
  'qr-code-email', 'qr-code-sms', 'qr-code-phone', 'qr-code-location',
  'qr-code-calendar', 'qr-code-pdf', 'qr-code-image', 'qr-code-video',
  'qr-code-audio', 'qr-code-file'
]
MATCH (parent:Entity {key: 'qr-code'})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(parent)
SET r.temperature = 0.95,
    r.llm_context = 'TYPE_OF: This entity is a specific type/variant of the parent category'
;

// Social media QR codes TYPE_OF qr-code-social-media (if exists) or qr-code
MATCH (child:Entity)
WHERE child.key IN [
  'qr-code-instagram', 'qr-code-facebook', 'qr-code-tiktok',
  'qr-code-youtube', 'qr-code-linkedin', 'qr-code-twitter',
  'qr-code-pinterest', 'qr-code-snapchat', 'qr-code-whatsapp', 'qr-code-telegram'
]
MATCH (parent:Entity {key: 'qr-code'})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(parent)
SET r.temperature = 0.95,
    r.semantic_cluster = 'social-media',
    r.llm_context = 'TYPE_OF: Social media QR code variant of the parent QR code category'
;

// Barcode types TYPE_OF barcode
MATCH (child:Entity)
WHERE child.key IN [
  'aztec-code', 'data-matrix', 'pdf417', 'maxicode',
  'codabar', 'code-128', 'code-39', 'gs1-128', 'gs1-datamatrix',
  'ean-13', 'ean-8', 'upc-a', 'upc-e', 'itf-14', 'msi-plessey'
]
MATCH (parent:Entity {key: 'barcode'})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(parent)
SET r.temperature = 0.95,
    r.llm_context = 'TYPE_OF: This barcode format is a specific type of barcode'
;

// ============================================================================
// 2. INCLUDES ARCS — Composition (0.85)
// ============================================================================
// Generator INCLUDES various QR code types

MATCH (tool:Entity {key: 'qr-code-generator'})
MATCH (included:Entity)
WHERE included.key IN [
  'qr-code-wifi', 'qr-code-vcard', 'qr-code-menu', 'qr-code-url',
  'dynamic-qr-code', 'static-qr-code', 'custom-qr-code'
]
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'includes'}]->(included)
SET r.temperature = 0.85,
    r.llm_context = 'INCLUDES: The tool/product includes this feature or capability'
;

// Custom QR code INCLUDES design features
MATCH (parent:Entity {key: 'custom-qr-code'})
MATCH (feature:Entity)
WHERE feature.key IN [
  'qr-code-with-logo', 'qr-code-color', 'qr-code-shapes',
  'qr-code-frame', 'qr-code-style'
]
MERGE (parent)-[r:SEMANTIC_LINK {link_type: 'includes'}]->(feature)
SET r.temperature = 0.85,
    r.llm_context = 'INCLUDES: Custom QR code includes this design/customization feature'
;

// ============================================================================
// 3. REQUIRES ARCS — Dependencies (0.70)
// ============================================================================

// Dynamic QR code requires subscription/account features
MATCH (entity:Entity {key: 'dynamic-qr-code'})
MATCH (requirement:Entity)
WHERE requirement.key IN ['analytics', 'click-tracking']
MERGE (entity)-[r:SEMANTIC_LINK {link_type: 'requires'}]->(requirement)
SET r.temperature = 0.70,
    r.llm_context = 'REQUIRES: This feature requires or depends on the target capability'
;

// API requires authentication
MATCH (entity:Entity {key: 'api'})
MATCH (requirement:Entity {key: 'api-documentation'})
WHERE EXISTS { MATCH (e:Entity {key: 'api-documentation'}) }
MERGE (entity)-[r:SEMANTIC_LINK {link_type: 'requires'}]->(requirement)
SET r.temperature = 0.70,
    r.llm_context = 'REQUIRES: API functionality requires documentation for proper usage'
;

// ============================================================================
// 4. EXHIBITS ARCS — Feature display (0.80)
// ============================================================================
// Entity EXHIBITS a visible feature/characteristic

MATCH (entity:Entity {key: 'qr-code-with-logo'})
MATCH (feature:Entity {key: 'custom-qr-code'})
MERGE (feature)-[r:SEMANTIC_LINK {link_type: 'exhibits'}]->(entity)
SET r.temperature = 0.80,
    r.llm_context = 'EXHIBITS: Custom QR code exhibits/displays logo embedding capability'
;

// ============================================================================
// 5. CONTRASTS ARCS — Opposition (0.50)
// ============================================================================
// Dynamic vs Static contrast

MATCH (a:Entity {key: 'dynamic-qr-code'})
MATCH (b:Entity {key: 'static-qr-code'})
MERGE (a)-[r:SEMANTIC_LINK {link_type: 'contrasts'}]->(b)
SET r.temperature = 0.50,
    r.comparison_axis = 'editability',
    r.llm_context = 'CONTRASTS: These entities are opposites/alternatives for comparison content'
;

// QR code vs Barcode
MATCH (a:Entity {key: 'qr-code'})
MATCH (b:Entity {key: 'barcode'})
MERGE (a)-[r:SEMANTIC_LINK {link_type: 'contrasts'}]->(b)
SET r.temperature = 0.50,
    r.comparison_axis = 'dimensionality',
    r.llm_context = 'CONTRASTS: 2D vs 1D code comparison for educational content'
;

// ============================================================================
// 6. IS_ACTION_ON ARCS — Verb→Noun (0.90)
// ============================================================================
// Actions that operate on entities

MATCH (action:Entity {key: 'create-qr-code'})
MATCH (target:Entity {key: 'qr-code'})
MERGE (action)-[r:SEMANTIC_LINK {link_type: 'is_action_on'}]->(target)
SET r.temperature = 0.90,
    r.llm_context = 'IS_ACTION_ON: This action operates on the target entity'
;

MATCH (action:Entity {key: 'scan-qr-code'})
MATCH (target:Entity {key: 'qr-code'})
MERGE (action)-[r:SEMANTIC_LINK {link_type: 'is_action_on'}]->(target)
SET r.temperature = 0.90,
    r.llm_context = 'IS_ACTION_ON: Scanning action operates on QR code entity'
;

MATCH (action:Entity {key: 'customize-qr-code'})
MATCH (target:Entity {key: 'qr-code'})
MERGE (action)-[r:SEMANTIC_LINK {link_type: 'is_action_on'}]->(target)
SET r.temperature = 0.90,
    r.llm_context = 'IS_ACTION_ON: Customization action operates on QR code entity'
;

MATCH (action:Entity {key: 'download-qr-code'})
MATCH (target:Entity {key: 'qr-code'})
MERGE (action)-[r:SEMANTIC_LINK {link_type: 'is_action_on'}]->(target)
SET r.temperature = 0.90,
    r.llm_context = 'IS_ACTION_ON: Download action operates on QR code entity'
;

MATCH (action:Entity {key: 'share-qr-code'})
MATCH (target:Entity {key: 'qr-code'})
MERGE (action)-[r:SEMANTIC_LINK {link_type: 'is_action_on'}]->(target)
SET r.temperature = 0.90,
    r.llm_context = 'IS_ACTION_ON: Share action operates on QR code entity'
;

MATCH (action:Entity {key: 'print-qr-code'})
MATCH (target:Entity {key: 'qr-code'})
MERGE (action)-[r:SEMANTIC_LINK {link_type: 'is_action_on'}]->(target)
SET r.temperature = 0.90,
    r.llm_context = 'IS_ACTION_ON: Print action operates on QR code entity'
;

// ============================================================================
// 7. PART_OF ARCS — Components (0.85)
// ============================================================================
// Design elements that are part of custom QR code

MATCH (part:Entity)
WHERE part.key IN [
  'qr-code-frame', 'qr-code-background', 'qr-code-color'
]
MATCH (whole:Entity {key: 'custom-qr-code'})
MERGE (part)-[r:SEMANTIC_LINK {link_type: 'part_of'}]->(whole)
SET r.temperature = 0.85,
    r.llm_context = 'PART_OF: This element is a component/part of the whole'
;

// Scanner PART_OF scanning workflow
MATCH (part:Entity {key: 'qr-code-scanner'})
MATCH (whole:Entity {key: 'scan-qr-code'})
MERGE (part)-[r:SEMANTIC_LINK {link_type: 'part_of'}]->(whole)
SET r.temperature = 0.85,
    r.llm_context = 'PART_OF: Scanner is the tool component of the scanning action'
;

// ============================================================================
// 8. CONVERGENCE BOOST — Multi-entity keyword targeting
// ============================================================================
// Update TARGETS arcs with convergence_boost based on entity count

// Calculate and set convergence_boost on existing TARGETS arcs
MATCH (en:EntityNative)-[t:TARGETS]->(k:SEOKeyword)
WITH k, count(DISTINCT en) AS entity_count
MATCH (en2:EntityNative)-[t2:TARGETS]->(k)
SET t2.convergence_boost = 1.0 + (entity_count * 0.2)
;

// ============================================================================
// 9. VERIFICATION QUERIES
// ============================================================================
// Run these to verify SEMANTIC_LINK arcs:
//
// -- Count by type
// MATCH ()-[r:SEMANTIC_LINK]->()
// RETURN r.type AS type, count(*) AS count, avg(r.temperature) AS avg_temp
// ORDER BY count DESC;
//
// -- Verify convergence boost
// MATCH (en:EntityNative)-[t:TARGETS]->(k:SEOKeyword)
// WHERE t.convergence_boost > 1.0
// RETURN k.key, t.convergence_boost, count(DISTINCT en) AS entities
// ORDER BY t.convergence_boost DESC
// LIMIT 10;
//
