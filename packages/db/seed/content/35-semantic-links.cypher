// packages/db/seed/35-semantic-links.cypher
// v0.13.0 - SEMANTIC_LINK Arcs for Entity Relationships
//
// Valid link_type values (from entity.yaml):
//   type_of, variant_of, includes      (Hierarchical)
//   requires, enables                   (Dependency)
//   exhibits, contrasts                 (Behavioral)
//   is_action_on, used_for, part_of    (Functional)
//   related_to                          (Associative)
//   same_as                             (Identity)
//
// Temperature coefficients (0.0-1.0):
//   used_for:     0.95 (TOOL is used for ACTION/OBJECT)
//   variant_of:   0.90 (OBJECT is variant of OBJECT)
//   enables:      0.80 (FEATURE enables ACTION)
//   related_to:   0.50-0.70 (associative relationship)

// ============================================================================
// 1. TOOL → ACTION (used_for, temperature=0.95)
// ============================================================================

// qr-code-generator is used for multiple actions
MATCH (tool:Entity {key: 'entity:qr-code-generator'})
UNWIND ['entity:customize-qr-code', 'entity:download-qr-code', 'entity:print-qr-code', 'entity:share-qr-code', 'entity:add-logo', 'entity:change-colors'] AS action_key
MATCH (action:Entity {key: action_key})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le générateur QR est utilisé pour effectuer cette action'
;

// barcode-generator → create-barcode
MATCH (tool:Entity {key: 'entity:barcode-generator'})
MATCH (action:Entity {key: 'entity:create-barcode'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le générateur code-barres est utilisé pour créer des codes-barres'
;

// barcode-scanner → scan-barcode
MATCH (tool:Entity {key: 'entity:barcode-scanner'})
MATCH (action:Entity {key: 'entity:scan-barcode'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le scanner code-barres est utilisé pour scanner des codes-barres'
;

// url-shortener → shorten-url
MATCH (tool:Entity {key: 'entity:url-shortener'})
MATCH (action:Entity {key: 'entity:shorten-url'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le raccourcisseur URL est utilisé pour raccourcir des liens'
;

// batch-qr-generator → bulk-creation
MATCH (tool:Entity {key: 'entity:batch-qr-generator'})
MATCH (action:Entity {key: 'entity:bulk-creation'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le générateur QR par lot est utilisé pour créer des QR codes en masse'
;

// batch-qr-generator also used for create-qr-code
MATCH (tool:Entity {key: 'entity:batch-qr-generator'})
MATCH (action:Entity {key: 'entity:create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.90,
    r.llm_context = 'USED_FOR: Le générateur par lot peut aussi créer des QR codes individuels'
;

// landing-page-builder → create-landing-page
MATCH (tool:Entity {key: 'entity:landing-page-builder'})
MATCH (action:Entity {key: 'entity:create-landing-page'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur landing page est utilisé pour créer des pages de destination'
;

// link-in-bio-builder → create-smart-link
MATCH (tool:Entity {key: 'entity:link-in-bio-builder'})
MATCH (action:Entity {key: 'entity:create-smart-link'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur link-in-bio est utilisé pour créer des liens intelligents'
;

// vcard-generator → create-qr-code
MATCH (tool:Entity {key: 'entity:vcard-generator'})
MATCH (action:Entity {key: 'entity:create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.90,
    r.llm_context = 'USED_FOR: Le générateur vCard crée un QR code contenant des informations de contact'
;

// menu-builder → create-qr-code
MATCH (tool:Entity {key: 'entity:menu-builder'})
MATCH (action:Entity {key: 'entity:create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.90,
    r.llm_context = 'USED_FOR: Le constructeur de menu crée un QR code pour menu de restaurant'
;

// utm-builder → track-scans
MATCH (tool:Entity {key: 'entity:utm-builder'})
MATCH (action:Entity {key: 'entity:track-scans'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.90,
    r.llm_context = 'USED_FOR: Le constructeur UTM est utilisé pour suivre les scans avec des paramètres de tracking'
;

// ============================================================================
// 2. TOOL → OBJECT (used_for, temperature=0.85)
// ============================================================================

// qr-code-generator is used_for various QR code types
MATCH (tool:Entity {key: 'entity:qr-code-generator'})
UNWIND [
  'entity:qr-code-facebook', 'entity:qr-code-linkedin', 'entity:qr-code-twitter', 'entity:qr-code-youtube',
  'entity:qr-code-spotify', 'entity:qr-code-tiktok', 'entity:qr-code-whatsapp', 'entity:qr-code-telegram',
  'entity:qr-code-email', 'entity:qr-code-sms', 'entity:qr-code-location', 'entity:qr-code-event',
  'entity:qr-code-app-store', 'entity:qr-code-google-play', 'entity:static-qr-code'
] AS object_key
MATCH (obj:Entity {key: object_key})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.85,
    r.llm_context = 'USED_FOR: Le générateur QR crée ce type de QR code spécialisé'
;

// barcode-generator is used_for barcode types
MATCH (tool:Entity {key: 'entity:barcode-generator'})
UNWIND [
  'entity:code-128', 'entity:code-39', 'entity:ean-13', 'entity:ean-8', 'entity:upc-a', 'entity:upc-e',
  'entity:gs1-128', 'entity:gs1-datamatrix', 'entity:itf-14', 'entity:codabar', 'entity:msi-plessey',
  'entity:pdf417', 'entity:data-matrix', 'entity:aztec-code', 'entity:maxicode'
] AS barcode_key
MATCH (barcode:Entity {key: barcode_key})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(barcode)
SET r.temperature = 0.85,
    r.llm_context = 'USED_FOR: Le générateur code-barres crée ce type de code-barres'
;

// vcard-generator is used_for vcard qr
MATCH (tool:Entity {key: 'entity:vcard-generator'})
MATCH (obj:Entity {key: 'entity:qr-code-vcard'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le générateur vCard crée des QR codes de cartes de visite'
;

// wifi-qr-generator already has link to qr-code-wifi (from 30-entity-semantic-arcs)

// menu-builder is used_for menu qr
MATCH (tool:Entity {key: 'entity:menu-builder'})
MATCH (obj:Entity {key: 'entity:qr-code-menu'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur de menu crée des QR codes pour menus de restaurant'
;

// landing-page-builder is used_for landing-page
MATCH (tool:Entity {key: 'entity:landing-page-builder'})
MATCH (obj:Entity {key: 'entity:landing-page'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur landing page crée des pages de destination'
;

// link-in-bio-builder is used_for link-in-bio
MATCH (tool:Entity {key: 'entity:link-in-bio-builder'})
MATCH (obj:Entity {key: 'entity:link-in-bio'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur link-in-bio crée des pages de liens intelligents'
;

// url-shortener is used_for short-link
MATCH (tool:Entity {key: 'entity:url-shortener'})
MATCH (obj:Entity {key: 'entity:short-link'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le raccourcisseur URL crée des liens courts'
;

// ============================================================================
// 3. OBJECT → BRAND (related_to, temperature=0.50)
// ============================================================================

// QR code types associated with their brands
UNWIND [
  {qr: 'entity:qr-code-instagram', brand: 'entity:instagram'},
  {qr: 'entity:qr-code-facebook', brand: 'entity:facebook'},
  {qr: 'entity:qr-code-linkedin', brand: 'entity:linkedin'},
  {qr: 'entity:qr-code-twitter', brand: 'entity:twitter'},
  {qr: 'entity:qr-code-youtube', brand: 'entity:youtube'},
  {qr: 'entity:qr-code-spotify', brand: 'entity:spotify'},
  {qr: 'entity:qr-code-tiktok', brand: 'entity:tiktok'},
  {qr: 'entity:qr-code-whatsapp', brand: 'entity:whatsapp'},
  {qr: 'entity:qr-code-telegram', brand: 'entity:telegram'},
  {qr: 'entity:qr-code-snapchat', brand: 'entity:snapchat'},
  {qr: 'entity:qr-code-pinterest', brand: 'entity:pinterest'},
  {qr: 'entity:qr-code-soundcloud', brand: 'entity:soundcloud'}
] AS pair
MATCH (qr:Entity {key: pair.qr})
MATCH (brand:Entity {key: pair.brand})
MERGE (qr)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(brand)
SET r.temperature = 0.50,
    r.llm_context = 'RELATED_TO: Ce type de QR code est associé à cette marque/plateforme'
;

// ============================================================================
// 4. FEATURE → ACTION (enables, temperature=0.80)
// ============================================================================

// Analytics enables track-scans
MATCH (feature:Entity {key: 'entity:analytics'})
MATCH (action:Entity {key: 'entity:track-scans'})
MERGE (feature)-[r:SEMANTIC_LINK {link_type: 'enables'}]->(action)
SET r.temperature = 0.80,
    r.llm_context = 'ENABLES: La fonctionnalité analytics permet le suivi des scans'
;

// Dynamic QR enables edit-destination
MATCH (feature:Entity {key: 'entity:dynamic-qr-code'})
MATCH (action:Entity {key: 'entity:edit-destination'})
MERGE (feature)-[r:SEMANTIC_LINK {link_type: 'enables'}]->(action)
SET r.temperature = 0.85,
    r.llm_context = 'ENABLES: Le QR dynamique permet de modifier la destination sans réimprimer'
;

// Password protection (if exists)
MATCH (feature:Entity {key: 'entity:password-protection'})
MATCH (action:Entity {key: 'entity:scan-limit'})
MERGE (feature)-[r:SEMANTIC_LINK {link_type: 'enables'}]->(action)
SET r.temperature = 0.80,
    r.llm_context = 'ENABLES: La protection par mot de passe permet de limiter les scans'
;

// ============================================================================
// 5. OBJECT variant_of relationships (temperature=0.90)
// ============================================================================

// Dynamic vs static QR codes
MATCH (dynamic:Entity {key: 'entity:dynamic-qr-code'})
MATCH (static:Entity {key: 'entity:static-qr-code'})
MERGE (dynamic)-[r:SEMANTIC_LINK {link_type: 'variant_of'}]->(static)
SET r.temperature = 0.90,
    r.llm_context = 'Dynamic QR is an editable variant of static QR'
;

// Custom QR is variant of QR code
MATCH (custom:Entity {key: 'entity:custom-qr-code'})
MATCH (base:Entity {key: 'entity:qr-code'})
MERGE (custom)-[r:SEMANTIC_LINK {link_type: 'variant_of'}]->(base)
SET r.temperature = 0.90,
    r.llm_context = 'VARIANT_OF: QR code personnalisé avec logo et couleurs'
;

// ============================================================================
// 6. INDUSTRY → OBJECT (uses pattern, enables convergence)
// ============================================================================

// Restaurants use menu QR
MATCH (industry:Entity {key: 'entity:restaurants'})
MATCH (obj:Entity {key: 'entity:qr-code-menu'})
MERGE (industry)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(obj)
SET r.temperature = 0.70,
    r.llm_context = 'Restaurant industry commonly uses menu QR codes'
;

// Healthcare uses vCard QR
MATCH (industry:Entity {key: 'entity:healthcare'})
MATCH (obj:Entity {key: 'entity:qr-code-vcard'})
MERGE (industry)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(obj)
SET r.temperature = 0.60,
    r.llm_context = 'INDUSTRY: Le secteur de la santé utilise les QR codes vCard pour les contacts médicaux'
;

// Real estate uses vCard and location QR
MATCH (industry:Entity {key: 'entity:real-estate'})
UNWIND ['entity:qr-code-vcard', 'entity:qr-code-location'] AS obj_key
MATCH (obj:Entity {key: obj_key})
MERGE (industry)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(obj)
SET r.temperature = 0.60,
    r.llm_context = 'INDUSTRY: Le secteur immobilier utilise les QR codes vCard et localisation pour les annonces'
;

// Retail uses product/payment QR
MATCH (industry:Entity {key: 'entity:retail'})
MATCH (obj:Entity {key: 'entity:qr-code-url'})
MERGE (industry)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(obj)
SET r.temperature = 0.60,
    r.llm_context = 'INDUSTRY: Le commerce de détail utilise les QR codes URL pour les produits et paiements'
;

// Event management uses event QR
MATCH (industry:Entity {key: 'entity:event-management'})
MATCH (obj:Entity {key: 'entity:qr-code-event'})
MERGE (industry)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(obj)
SET r.temperature = 0.80,
    r.llm_context = 'INDUSTRY → OBJECT: Event management commonly uses event QR codes'
;

// ============================================================================
// 7. OBJECT type_of PARENT (hierarchical, temperature=0.95)
// ============================================================================
// QR code types are type_of the base qr-code entity
// This enables SEO convergence: searching "QR code" should also match specific types

// Social media QR codes
MATCH (base:Entity {key: 'entity:qr-code'})
UNWIND [
  'entity:qr-code-instagram', 'entity:qr-code-facebook', 'entity:qr-code-linkedin', 'entity:qr-code-twitter',
  'entity:qr-code-youtube', 'entity:qr-code-spotify', 'entity:qr-code-tiktok', 'entity:qr-code-whatsapp',
  'entity:qr-code-telegram', 'entity:qr-code-snapchat', 'entity:qr-code-pinterest', 'entity:qr-code-soundcloud'
] AS child_key
MATCH (child:Entity {key: child_key})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(base)
SET r.temperature = 0.95,
    r.llm_context = 'TYPE_OF: QR code spécialisé pour une plateforme sociale'
;

// Utility QR codes
MATCH (base:Entity {key: 'entity:qr-code'})
UNWIND [
  'entity:qr-code-wifi', 'entity:qr-code-vcard', 'entity:qr-code-menu', 'entity:qr-code-email',
  'entity:qr-code-sms', 'entity:qr-code-location', 'entity:qr-code-event', 'entity:qr-code-url',
  'entity:qr-code-app-store', 'entity:qr-code-google-play'
] AS child_key
MATCH (child:Entity {key: child_key})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(base)
SET r.temperature = 0.95,
    r.llm_context = 'TYPE_OF: QR code spécialisé pour un usage utilitaire'
;

// Static and dynamic are variants, not types
// (already defined in variant_of section)

// ============================================================================
// 8. ACTION hierarchy (is_action_on, temperature=0.85)
// ============================================================================
// Secondary actions are is_action_on the primary action

// Actions performed on a QR code after creation
MATCH (primary:Entity {key: 'entity:create-qr-code'})
UNWIND [
  'entity:customize-qr-code', 'entity:download-qr-code', 'entity:print-qr-code', 'entity:share-qr-code'
] AS secondary_key
MATCH (secondary:Entity {key: secondary_key})
MERGE (secondary)-[r:SEMANTIC_LINK {link_type: 'is_action_on'}]->(primary)
SET r.temperature = 0.85,
    r.llm_context = 'IS_ACTION_ON: Action effectuée sur un QR code créé'
;

// Customization sub-actions
MATCH (primary:Entity {key: 'entity:customize-qr-code'})
UNWIND [
  'entity:add-logo', 'entity:change-colors'
] AS secondary_key
MATCH (secondary:Entity {key: secondary_key})
MERGE (secondary)-[r:SEMANTIC_LINK {link_type: 'is_action_on'}]->(primary)
SET r.temperature = 0.85,
    r.llm_context = 'IS_ACTION_ON: Action de personnalisation du QR code'
;

// ============================================================================
// 9. Barcode type_of hierarchy (temperature=0.90)
// ============================================================================

// 1D barcodes
MATCH (base:Entity {key: 'entity:barcode'})
UNWIND [
  'entity:code-128', 'entity:code-39', 'entity:ean-13', 'entity:ean-8', 'entity:upc-a', 'entity:upc-e',
  'entity:gs1-128', 'entity:itf-14', 'entity:codabar', 'entity:msi-plessey'
] AS child_key
MATCH (child:Entity {key: child_key})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(base)
SET r.temperature = 0.90,
    r.llm_context = 'TYPE_OF: Code-barres 1D linéaire'
;

// 2D barcodes (matrix)
MATCH (base:Entity {key: 'entity:barcode'})
UNWIND [
  'entity:pdf417', 'entity:data-matrix', 'entity:aztec-code', 'entity:maxicode', 'entity:gs1-datamatrix'
] AS child_key
MATCH (child:Entity {key: child_key})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(base)
SET r.temperature = 0.90,
    r.llm_context = 'TYPE_OF: Code-barres 2D matriciel'
;

