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
MATCH (tool:Entity {key: 'qr-code-generator'})
UNWIND ['customize-qr-code', 'download-qr-code', 'print-qr-code', 'share-qr-code', 'add-logo', 'change-colors'] AS action_key
MATCH (action:Entity {key: action_key})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le générateur QR est utilisé pour effectuer cette action'
;

// barcode-generator → create-barcode
MATCH (tool:Entity {key: 'barcode-generator'})
MATCH (action:Entity {key: 'create-barcode'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le générateur code-barres est utilisé pour créer des codes-barres'
;

// barcode-scanner → scan-barcode
MATCH (tool:Entity {key: 'barcode-scanner'})
MATCH (action:Entity {key: 'scan-barcode'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le scanner code-barres est utilisé pour scanner des codes-barres'
;

// url-shortener → shorten-url
MATCH (tool:Entity {key: 'url-shortener'})
MATCH (action:Entity {key: 'shorten-url'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le raccourcisseur URL est utilisé pour raccourcir des liens'
;

// batch-qr-generator → bulk-creation
MATCH (tool:Entity {key: 'batch-qr-generator'})
MATCH (action:Entity {key: 'bulk-creation'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le générateur QR par lot est utilisé pour créer des QR codes en masse'
;

// batch-qr-generator also used for create-qr-code
MATCH (tool:Entity {key: 'batch-qr-generator'})
MATCH (action:Entity {key: 'create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.90,
    r.llm_context = 'USED_FOR: Le générateur par lot peut aussi créer des QR codes individuels'
;

// landing-page-builder → create-landing-page
MATCH (tool:Entity {key: 'landing-page-builder'})
MATCH (action:Entity {key: 'create-landing-page'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur landing page est utilisé pour créer des pages de destination'
;

// link-in-bio-builder → create-smart-link
MATCH (tool:Entity {key: 'link-in-bio-builder'})
MATCH (action:Entity {key: 'create-smart-link'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur link-in-bio est utilisé pour créer des liens intelligents'
;

// vcard-generator → create-qr-code
MATCH (tool:Entity {key: 'vcard-generator'})
MATCH (action:Entity {key: 'create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.90,
    r.llm_context = 'USED_FOR: Le générateur vCard crée un QR code contenant des informations de contact'
;

// menu-builder → create-qr-code
MATCH (tool:Entity {key: 'menu-builder'})
MATCH (action:Entity {key: 'create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.90,
    r.llm_context = 'USED_FOR: Le constructeur de menu crée un QR code pour menu de restaurant'
;

// utm-builder → track-scans
MATCH (tool:Entity {key: 'utm-builder'})
MATCH (action:Entity {key: 'track-scans'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.90,
    r.llm_context = 'USED_FOR: Le constructeur UTM est utilisé pour suivre les scans avec des paramètres de tracking'
;

// ============================================================================
// 2. TOOL → OBJECT (used_for, temperature=0.85)
// ============================================================================

// qr-code-generator is used_for various QR code types
MATCH (tool:Entity {key: 'qr-code-generator'})
UNWIND [
  'qr-code-facebook', 'qr-code-linkedin', 'qr-code-twitter', 'qr-code-youtube',
  'qr-code-spotify', 'qr-code-tiktok', 'qr-code-whatsapp', 'qr-code-telegram',
  'qr-code-email', 'qr-code-sms', 'qr-code-location', 'qr-code-event',
  'qr-code-app-store', 'qr-code-google-play', 'static-qr-code'
] AS object_key
MATCH (obj:Entity {key: object_key})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.85,
    r.llm_context = 'USED_FOR: Le générateur QR crée ce type de QR code spécialisé'
;

// barcode-generator is used_for barcode types
MATCH (tool:Entity {key: 'barcode-generator'})
UNWIND [
  'code-128', 'code-39', 'ean-13', 'ean-8', 'upc-a', 'upc-e',
  'gs1-128', 'gs1-datamatrix', 'itf-14', 'codabar', 'msi-plessey',
  'pdf417', 'data-matrix', 'aztec-code', 'maxicode'
] AS barcode_key
MATCH (barcode:Entity {key: barcode_key})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(barcode)
SET r.temperature = 0.85,
    r.llm_context = 'USED_FOR: Le générateur code-barres crée ce type de code-barres'
;

// vcard-generator is used_for vcard qr
MATCH (tool:Entity {key: 'vcard-generator'})
MATCH (obj:Entity {key: 'qr-code-vcard'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le générateur vCard crée des QR codes de cartes de visite'
;

// wifi-qr-generator already has link to qr-code-wifi (from 30-entity-semantic-arcs)

// menu-builder is used_for menu qr
MATCH (tool:Entity {key: 'menu-builder'})
MATCH (obj:Entity {key: 'qr-code-menu'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur de menu crée des QR codes pour menus de restaurant'
;

// landing-page-builder is used_for landing-page
MATCH (tool:Entity {key: 'landing-page-builder'})
MATCH (obj:Entity {key: 'landing-page'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur landing page crée des pages de destination'
;

// link-in-bio-builder is used_for link-in-bio
MATCH (tool:Entity {key: 'link-in-bio-builder'})
MATCH (obj:Entity {key: 'link-in-bio'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le constructeur link-in-bio crée des pages de liens intelligents'
;

// url-shortener is used_for short-link
MATCH (tool:Entity {key: 'url-shortener'})
MATCH (obj:Entity {key: 'short-link'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.95,
    r.llm_context = 'USED_FOR: Le raccourcisseur URL crée des liens courts'
;

// ============================================================================
// 3. OBJECT → BRAND (related_to, temperature=0.50)
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
MERGE (qr)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(brand)
SET r.temperature = 0.50,
    r.llm_context = 'RELATED_TO: Ce type de QR code est associé à cette marque/plateforme'
;

// ============================================================================
// 4. FEATURE → ACTION (enables, temperature=0.80)
// ============================================================================

// Analytics enables track-scans
MATCH (feature:Entity {key: 'analytics'})
MATCH (action:Entity {key: 'track-scans'})
MERGE (feature)-[r:SEMANTIC_LINK {link_type: 'enables'}]->(action)
SET r.temperature = 0.80,
    r.llm_context = 'ENABLES: La fonctionnalité analytics permet le suivi des scans'
;

// Dynamic QR enables edit-destination
MATCH (feature:Entity {key: 'dynamic-qr-code'})
MATCH (action:Entity {key: 'edit-destination'})
MERGE (feature)-[r:SEMANTIC_LINK {link_type: 'enables'}]->(action)
SET r.temperature = 0.85,
    r.llm_context = 'ENABLES: Le QR dynamique permet de modifier la destination sans réimprimer'
;

// Password protection (if exists)
MATCH (feature:Entity {key: 'password-protection'})
MATCH (action:Entity {key: 'scan-limit'})
MERGE (feature)-[r:SEMANTIC_LINK {link_type: 'enables'}]->(action)
SET r.temperature = 0.80,
    r.llm_context = 'ENABLES: La protection par mot de passe permet de limiter les scans'
;

// ============================================================================
// 5. OBJECT variant_of relationships (temperature=0.90)
// ============================================================================

// Dynamic vs static QR codes
MATCH (dynamic:Entity {key: 'dynamic-qr-code'})
MATCH (static:Entity {key: 'static-qr-code'})
MERGE (dynamic)-[r:SEMANTIC_LINK {link_type: 'variant_of'}]->(static)
SET r.temperature = 0.90,
    r.llm_context = 'Dynamic QR is an editable variant of static QR'
;

// Custom QR is variant of QR code
MATCH (custom:Entity {key: 'custom-qr-code'})
MATCH (base:Entity {key: 'qr-code'})
MERGE (custom)-[r:SEMANTIC_LINK {link_type: 'variant_of'}]->(base)
SET r.temperature = 0.90,
    r.llm_context = 'VARIANT_OF: QR code personnalisé avec logo et couleurs'
;

// ============================================================================
// 6. INDUSTRY → OBJECT (uses pattern, enables convergence)
// ============================================================================

// Restaurants use menu QR
MATCH (industry:Entity {key: 'restaurants'})
MATCH (obj:Entity {key: 'qr-code-menu'})
MERGE (industry)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(obj)
SET r.temperature = 0.70,
    r.llm_context = 'Restaurant industry commonly uses menu QR codes'
;

// Healthcare uses vCard QR
MATCH (industry:Entity {key: 'healthcare'})
MATCH (obj:Entity {key: 'qr-code-vcard'})
MERGE (industry)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(obj)
SET r.temperature = 0.60,
    r.llm_context = 'INDUSTRY: Le secteur de la santé utilise les QR codes vCard pour les contacts médicaux'
;

// Real estate uses vCard and location QR
MATCH (industry:Entity {key: 'real-estate'})
UNWIND ['qr-code-vcard', 'qr-code-location'] AS obj_key
MATCH (obj:Entity {key: obj_key})
MERGE (industry)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(obj)
SET r.temperature = 0.60,
    r.llm_context = 'INDUSTRY: Le secteur immobilier utilise les QR codes vCard et localisation pour les annonces'
;

// Retail uses product/payment QR
MATCH (industry:Entity {key: 'retail'})
MATCH (obj:Entity {key: 'qr-code-url'})
MERGE (industry)-[r:SEMANTIC_LINK {link_type: 'related_to'}]->(obj)
SET r.temperature = 0.60,
    r.llm_context = 'INDUSTRY: Le commerce de détail utilise les QR codes URL pour les produits et paiements'
;

// Event management uses event QR
MATCH (industry:Entity {key: 'event-management'})
MATCH (obj:Entity {key: 'qr-code-event'})
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
MATCH (base:Entity {key: 'qr-code'})
UNWIND [
  'qr-code-instagram', 'qr-code-facebook', 'qr-code-linkedin', 'qr-code-twitter',
  'qr-code-youtube', 'qr-code-spotify', 'qr-code-tiktok', 'qr-code-whatsapp',
  'qr-code-telegram', 'qr-code-snapchat', 'qr-code-pinterest', 'qr-code-soundcloud'
] AS child_key
MATCH (child:Entity {key: child_key})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(base)
SET r.temperature = 0.95,
    r.llm_context = 'TYPE_OF: QR code spécialisé pour une plateforme sociale'
;

// Utility QR codes
MATCH (base:Entity {key: 'qr-code'})
UNWIND [
  'qr-code-wifi', 'qr-code-vcard', 'qr-code-menu', 'qr-code-email',
  'qr-code-sms', 'qr-code-location', 'qr-code-event', 'qr-code-url',
  'qr-code-app-store', 'qr-code-google-play'
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
MATCH (primary:Entity {key: 'create-qr-code'})
UNWIND [
  'customize-qr-code', 'download-qr-code', 'print-qr-code', 'share-qr-code'
] AS secondary_key
MATCH (secondary:Entity {key: secondary_key})
MERGE (secondary)-[r:SEMANTIC_LINK {link_type: 'is_action_on'}]->(primary)
SET r.temperature = 0.85,
    r.llm_context = 'IS_ACTION_ON: Action effectuée sur un QR code créé'
;

// Customization sub-actions
MATCH (primary:Entity {key: 'customize-qr-code'})
UNWIND [
  'add-logo', 'change-colors'
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
MATCH (base:Entity {key: 'barcode'})
UNWIND [
  'code-128', 'code-39', 'ean-13', 'ean-8', 'upc-a', 'upc-e',
  'gs1-128', 'itf-14', 'codabar', 'msi-plessey'
] AS child_key
MATCH (child:Entity {key: child_key})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(base)
SET r.temperature = 0.90,
    r.llm_context = 'TYPE_OF: Code-barres 1D linéaire'
;

// 2D barcodes (matrix)
MATCH (base:Entity {key: 'barcode'})
UNWIND [
  'pdf417', 'data-matrix', 'aztec-code', 'maxicode', 'gs1-datamatrix'
] AS child_key
MATCH (child:Entity {key: child_key})
MERGE (child)-[r:SEMANTIC_LINK {link_type: 'type_of'}]->(base)
SET r.temperature = 0.90,
    r.llm_context = 'TYPE_OF: Code-barres 2D matriciel'
;

