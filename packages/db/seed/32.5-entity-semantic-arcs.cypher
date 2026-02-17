// packages/db/seed/30-entity-semantic-arcs.cypher
// v0.13.0 - Multi-Entity Keyword Convergence Architecture
//
// ADR-029: *Native Pattern (EntityNative, PageNative)
// ADR-030: Slug Ownership (Page owns URL, Entity owns semantics)
// ADR-032: URL Slugification (convergence_boost = 1 + N × 0.2)
//
// Entity Types:
//   TOOL   = qr-code-generator (the product)
//   ACTION = create-qr-code (user intent, verb)
//   OBJECT = qr-code-instagram, etc. (the created thing)
//
// Semantic Coefficients (ADR-032):
//   used_for:     0.95 (tool is used_for action)
//   used_for:     0.85 (tool is used_for object)
//   type_of:      0.95 (object is type_of parent)
//   enables:      0.80 (feature enables action)
//   variant_of:   0.90 (object is variant of base)
//   same_as:      1.00 (synonym relationship)

// ============================================================================
// 1. SEMANTIC ARCS: TOOL → ACTION (USED_FOR)
// ============================================================================
// qr-code-generator is USED_FOR create-qr-code

MATCH (tool:Entity {key: 'qr-code-generator'})
MATCH (action:Entity {key: 'create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(action)
SET r.temperature = 0.95,
    r.llm_context = 'TOOL → ACTION: Le générateur est utilisé pour créer des QR codes'
;

// ============================================================================
// 2. SEMANTIC ARCS: TOOL → OBJECTs (USED_FOR)
// ============================================================================
// qr-code-generator USED_FOR qr-code-instagram, qr-code-wifi, etc.

MATCH (tool:Entity {key: 'qr-code-generator'})
MATCH (obj:Entity)
WHERE obj.key IN ['qr-code-instagram', 'qr-code-wifi', 'qr-code-vcard', 'qr-code-menu']
MERGE (tool)-[r:SEMANTIC_LINK {link_type: 'used_for'}]->(obj)
SET r.temperature = 0.85,
    r.llm_context = 'TOOL → OBJECT: Le générateur est utilisé pour créer ce type de QR code'
;

// ============================================================================
// 3. EntityNative fr-FR: TOOL
// ============================================================================

MATCH (e:Entity {key: 'qr-code-generator'})
MATCH (l:Locale {key: 'fr-FR'})
MERGE (en:EntityNative {key: 'entity:qr-code-generator@fr-FR'})
SET en.locale = 'fr-FR',
    en.title = 'Générateur de QR Code',
    en.description = 'Outil de création de QR codes personnalisés',
    en.entity_type = 'TOOL',
    en.slug_terms = ['générateur', 'qr', 'code', 'créer'],
    en.updated_at = datetime()
MERGE (e)-[:HAS_NATIVE]->(en)
MERGE (en)-[:FOR_LOCALE]->(l)
;

// ============================================================================
// 4. EntityNative fr-FR: ACTION
// ============================================================================

MATCH (e:Entity {key: 'create-qr-code'})
MATCH (l:Locale {key: 'fr-FR'})
MERGE (en:EntityNative {key: 'entity:create-qr-code@fr-FR'})
SET en.locale = 'fr-FR',
    en.title = 'Créer un QR Code',
    en.description = 'Action de générer un QR code personnalisé',
    en.entity_type = 'ACTION',
    en.slug_terms = ['créer', 'générer', 'qr', 'code'],
    en.updated_at = datetime()
MERGE (e)-[:HAS_NATIVE]->(en)
MERGE (en)-[:FOR_LOCALE]->(l)
;

// ============================================================================
// 5. EntityNative fr-FR: OBJECTs
// ============================================================================

MATCH (l:Locale {key: 'fr-FR'})
UNWIND [
  {key: 'qr-code-instagram', title: 'QR Code Instagram', desc: 'QR code pour lien Instagram', type: 'OBJECT'},
  {key: 'qr-code-wifi', title: 'QR Code WiFi', desc: 'QR code pour partager un réseau WiFi', type: 'OBJECT'},
  {key: 'qr-code-vcard', title: 'QR Code vCard', desc: 'QR code carte de visite', type: 'OBJECT'},
  {key: 'qr-code-menu', title: 'QR Code Menu', desc: 'QR code pour menu restaurant', type: 'OBJECT'}
] AS obj
MATCH (e:Entity {key: obj.key})
MERGE (en:EntityNative {key: 'entity:' + obj.key + '@fr-FR'})
SET en.locale = 'fr-FR',
    en.title = obj.title,
    en.description = obj.desc,
    en.entity_type = obj.type,
    en.updated_at = datetime()
MERGE (e)-[:HAS_NATIVE]->(en)
MERGE (en)-[:FOR_LOCALE]->(l)
;

// ============================================================================
// 6. MULTI-ENTITY KEYWORD TARGETING (Convergence Boost)
// ============================================================================
// Keywords liés à PLUSIEURS entités pour convergence_boost = 1 + (N × 0.2)
// "créer qr code" vol=6300 → create-qr-code (primary) + qr-code-generator (secondary)
// convergence_boost = 1 + (2 × 0.2) = 1.4
// final_score = 6300 × 1.4 = 8820

// Link "créer qr code" to both ACTION (primary) and TOOL (secondary)
MATCH (k:SEOKeyword)
WHERE k.key STARTS WITH 'seo-creer-qr-code' OR k.key STARTS WITH 'seo-creer-un-qr-code'
   OR k.key STARTS WITH 'seo-generer-qr-code' OR k.key STARTS WITH 'seo-generer-un-qr-code'
MATCH (en_action:EntityNative {key: 'entity:create-qr-code@fr-FR'})
MATCH (en_tool:EntityNative {key: 'entity:qr-code-generator@fr-FR'})
MERGE (en_action)-[r1:TARGETS]->(k)
SET r1.rank = 'primary',
    r1.semantic_coef = 1.0
MERGE (en_tool)-[r2:TARGETS]->(k)
SET r2.rank = 'secondary',
    r2.semantic_coef = 0.95
;

// ============================================================================
// SLUG DERIVATION FORMULA (ADR-032)
// ============================================================================
//
// score = base_volume × semantic_coef × convergence_boost
//
// Example for Page:qr-code-generator @fr-FR:
//   keyword: "créer qr code" (vol: 6300)
//   entities_targeting: [create-qr-code, qr-code-generator]
//   convergence_boost: 1 + (2 × 0.2) = 1.4
//   semantic_coef: 0.95 (USED_FOR relationship)
//
//   score = 6300 × 0.95 × 1.4 = 8379
//
//   → Winner slug: "créer-qr-code" (meilleur score)
//
// For Page:qr-code-instagram @fr-FR:
//   No-repetition rule: parent_terms = {créer, qr, code}
//   keyword: "qr code instagram" (vol: 2400)
//   new_terms = {instagram}
//
//   → Winner slug: "instagram" (différenciateur only)
//
