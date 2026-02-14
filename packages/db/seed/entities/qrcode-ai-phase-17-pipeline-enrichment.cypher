// ═══════════════════════════════════════════════════════════════════════════════
// Phase 17: Pipeline v2.0.0 Enrichment for custom-qr-code
// ═══════════════════════════════════════════════════════════════════════════════
// This file adds missing elements for the page-pipeline-overview v2.0.0 view:
// - Project links (HAS_PAGE, HAS_ENTITY, SUPPORTS_LOCALE)
// - Slugification for fr-FR
// - Style for fr-FR
// - USES_ENTITY with purpose and temperature
// - REFERENCES (Block → Entity aggregated)
// - REFERENCES_ENTITY (BlockInstruction → Entity granular)
// - HAS_KEYWORD with rank (replaces TARGETS)
// - SEOKeywordMetrics
// ═══════════════════════════════════════════════════════════════════════════════


// ─────────────────────────────────────────────────────────────────────────────
// 1. PROJECT LAYER: Link Project to Page and Entity
// ─────────────────────────────────────────────────────────────────────────────

// Ensure Project exists and link to Page
MATCH (proj:Project {key: 'qrcode-ai'})
MATCH (p:Page {key: 'custom-qr-code'})
MERGE (proj)-[:HAS_PAGE]->(p);

// Link Project to Entity
MATCH (proj:Project {key: 'qrcode-ai'})
MATCH (e:Entity {key: 'custom-qr-code'})
MERGE (proj)-[:HAS_ENTITY]->(e);

// Ensure Project supports fr-FR locale
MATCH (proj:Project {key: 'qrcode-ai'})
MATCH (l:Locale {key: 'fr-FR'})
MERGE (proj)-[:SUPPORTS_LOCALE]->(l);


// ─────────────────────────────────────────────────────────────────────────────
// 2. LOCALE LAYER: Slugification and Style for fr-FR
// ─────────────────────────────────────────────────────────────────────────────

// Create Slugification for fr-FR
MERGE (slug:Slugification {key: 'slugification-fr-FR'})
ON CREATE SET
  slug.display_name = 'French Slugification Rules',
  slug.separator = '-',
  slug.lowercase = true,
  slug.transliterate = true,
  slug.max_length = 60,
  slug.strip_accents = true,
  slug.word_separator = '-',
  slug.llm_context = 'USE: French URL slugs. Transliterate accents (é→e, ç→c). Max 60 chars.',
  slug.created_at = datetime()
SET slug.updated_at = datetime();

// Link Locale to Slugification
MATCH (l:Locale {key: 'fr-FR'})
MATCH (slug:Slugification {key: 'slugification-fr-FR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(slug);

// Create Style for fr-FR
MERGE (style:Style {key: 'style-fr-FR'})
ON CREATE SET
  style.display_name = 'French Writing Style',
  style.tone = 'professionnel',
  style.formality = 'vous',
  style.humor_level = 0.2,
  style.directness = 0.7,
  style.sentence_length = 'medium',
  style.paragraph_length = 'medium',
  style.use_active_voice = true,
  style.llm_context = 'USE: Professional French tone. Use "vous" (formal). Moderate humor. Clear and direct.',
  style.created_at = datetime()
SET style.updated_at = datetime();

// Link Locale to Style
MATCH (l:Locale {key: 'fr-FR'})
MATCH (style:Style {key: 'style-fr-FR'})
MERGE (l)-[:HAS_STYLE]->(style);


// ─────────────────────────────────────────────────────────────────────────────
// 3. SEMANTIC LAYER: USES_ENTITY with purpose and temperature
// ─────────────────────────────────────────────────────────────────────────────

// Page USES_ENTITY: custom-qr-code (primary, high temperature)
MATCH (p:Page {key: 'custom-qr-code'})
MATCH (e:Entity {key: 'custom-qr-code'})
MERGE (p)-[ue:USES_ENTITY]->(e)
SET ue.purpose = 'primary', ue.temperature = 1.0;

// Page USES_ENTITY: qr-code-generator (secondary, medium temperature)
MATCH (p:Page {key: 'custom-qr-code'})
MERGE (e2:Entity {key: 'qr-code-generator'})
ON CREATE SET
  e2.display_name = 'QR Code Generator',
  e2.description = 'Online tool for creating QR codes',
  e2.llm_context = 'USE: Reference to the QR code generator tool.',
  e2.created_at = datetime()
SET e2.updated_at = datetime()
MERGE (p)-[ue2:USES_ENTITY]->(e2)
SET ue2.purpose = 'secondary', ue2.temperature = 0.7;

// Create supporting entities for blocks
MERGE (e3:Entity {key: 'qr-code-color'})
ON CREATE SET
  e3.display_name = 'QR Code Colors',
  e3.description = 'Color customization options for QR codes',
  e3.llm_context = 'USE: Color features like foreground, background, gradient.',
  e3.created_at = datetime()
SET e3.updated_at = datetime();

MERGE (e4:Entity {key: 'qr-code-shapes'})
ON CREATE SET
  e4.display_name = 'QR Code Shapes',
  e4.description = 'Shape and pattern options for QR codes',
  e4.llm_context = 'USE: Shape features like dots, squares, rounded corners.',
  e4.created_at = datetime()
SET e4.updated_at = datetime();


// ─────────────────────────────────────────────────────────────────────────────
// 4. BLOCK REFERENCES: Aggregated Block → Entity links
// ─────────────────────────────────────────────────────────────────────────────

// Block design-features REFERENCES qr-code-color (inject, count 2)
MATCH (b:Block {key: 'blk-custom-qr-code-design-features'})
MATCH (e:Entity {key: 'qr-code-color'})
MERGE (b)-[ref1:REFERENCES]->(e)
SET ref1.purpose = 'inject', ref1.count = 2;

// Block design-features REFERENCES qr-code-shapes (inject, count 1)
MATCH (b:Block {key: 'blk-custom-qr-code-design-features'})
MATCH (e:Entity {key: 'qr-code-shapes'})
MERGE (b)-[ref2:REFERENCES]->(e)
SET ref2.purpose = 'inject', ref2.count = 1;

// Block generator-cta REFERENCES qr-code-generator (link, count 1)
MATCH (b:Block {key: 'blk-custom-qr-code-generator-cta'})
MATCH (e:Entity {key: 'qr-code-generator'})
MERGE (b)-[ref3:REFERENCES]->(e)
SET ref3.purpose = 'link', ref3.count = 1;


// ─────────────────────────────────────────────────────────────────────────────
// 5. INSTRUCTION REFERENCES: Granular BlockInstruction → Entity
// ─────────────────────────────────────────────────────────────────────────────

// BlockInstruction instr-custom-qr-hero → Entity custom-qr-code
MATCH (bi:BlockInstruction {key: 'instr-custom-qr-hero'})
MATCH (e:Entity {key: 'custom-qr-code'})
MERGE (bi)-[re1:REFERENCES_ENTITY]->(e)
SET re1.syntax = '@entity:custom-qr-code', re1.context = 'hero section main entity';

// BlockInstruction instr-custom-qr-features → Entity qr-code-color
MATCH (bi:BlockInstruction {key: 'instr-custom-qr-features'})
MATCH (e:Entity {key: 'qr-code-color'})
MERGE (bi)-[re2:REFERENCES_ENTITY]->(e)
SET re2.syntax = '@entity:qr-code-color', re2.context = 'design features color options';

// BlockInstruction instr-custom-qr-features → Entity qr-code-shapes
MATCH (bi:BlockInstruction {key: 'instr-custom-qr-features'})
MATCH (e:Entity {key: 'qr-code-shapes'})
MERGE (bi)-[re3:REFERENCES_ENTITY]->(e)
SET re3.syntax = '@entity:qr-code-shapes', re3.context = 'design features shape options';

// BlockInstruction instr-custom-qr-cta → Entity qr-code-generator
MATCH (bi:BlockInstruction {key: 'instr-custom-qr-cta'})
MATCH (e:Entity {key: 'qr-code-generator'})
MERGE (bi)-[re4:REFERENCES_ENTITY]->(e)
SET re4.syntax = '@entity:qr-code-generator', re4.context = 'CTA link to generator';


// ─────────────────────────────────────────────────────────────────────────────
// 6. SEO LAYER: HAS_KEYWORD with rank + SEOKeywordMetrics
// ─────────────────────────────────────────────────────────────────────────────

// Create SEO Keywords with proper keys
MERGE (kw1:SEOKeyword {key: 'kw-qr-code-personnalise'})
ON CREATE SET
  kw1.display_name = 'qr code personnalisé',
  kw1.keyword = 'qr code personnalisé',
  kw1.locale_key = 'fr-FR',
  kw1.intent = 'transactional',
  kw1.llm_context = 'USE: Primary French keyword for custom QR codes.',
  kw1.created_at = datetime()
SET kw1.updated_at = datetime();

MERGE (kw2:SEOKeyword {key: 'kw-creer-qr-code-personnalise'})
ON CREATE SET
  kw2.display_name = 'créer qr code personnalisé',
  kw2.keyword = 'créer qr code personnalisé',
  kw2.locale_key = 'fr-FR',
  kw2.intent = 'transactional',
  kw2.llm_context = 'USE: Action-oriented French keyword.',
  kw2.created_at = datetime()
SET kw2.updated_at = datetime();

MERGE (kw3:SEOKeyword {key: 'kw-generateur-qr-code-design'})
ON CREATE SET
  kw3.display_name = 'générateur qr code design',
  kw3.keyword = 'générateur qr code design',
  kw3.locale_key = 'fr-FR',
  kw3.intent = 'informational',
  kw3.llm_context = 'USE: Design-focused French keyword.',
  kw3.created_at = datetime()
SET kw3.updated_at = datetime();

MERGE (kw4:SEOKeyword {key: 'kw-qr-code-avec-logo'})
ON CREATE SET
  kw4.display_name = 'qr code avec logo',
  kw4.keyword = 'qr code avec logo',
  kw4.locale_key = 'fr-FR',
  kw4.intent = 'transactional',
  kw4.llm_context = 'USE: Logo feature French keyword.',
  kw4.created_at = datetime()
SET kw4.updated_at = datetime();


// Link Entity → SEOKeyword with HAS_KEYWORD and rank
MATCH (e:Entity {key: 'custom-qr-code'})
MATCH (kw1:SEOKeyword {key: 'kw-qr-code-personnalise'})
MERGE (e)-[hk1:HAS_KEYWORD]->(kw1)
SET hk1.rank = 'primary';

MATCH (e:Entity {key: 'custom-qr-code'})
MATCH (kw2:SEOKeyword {key: 'kw-creer-qr-code-personnalise'})
MERGE (e)-[hk2:HAS_KEYWORD]->(kw2)
SET hk2.rank = 'primary';

MATCH (e:Entity {key: 'custom-qr-code'})
MATCH (kw3:SEOKeyword {key: 'kw-generateur-qr-code-design'})
MERGE (e)-[hk3:HAS_KEYWORD]->(kw3)
SET hk3.rank = 'secondary';

MATCH (e:Entity {key: 'custom-qr-code'})
MATCH (kw4:SEOKeyword {key: 'kw-qr-code-avec-logo'})
MERGE (e)-[hk4:HAS_KEYWORD]->(kw4)
SET hk4.rank = 'secondary';


// Create SEOKeywordMetrics for each keyword
MERGE (m1:SEOKeywordMetrics {key: 'metrics-kw-qr-code-personnalise'})
ON CREATE SET
  m1.keyword_key = 'kw-qr-code-personnalise',
  m1.volume = 2400,
  m1.difficulty = 35,
  m1.cpc = 0.45,
  m1.trend = 'stable',
  m1.competition = 'medium',
  m1.serp_features = ['featured_snippet', 'images', 'related_questions'],
  m1.data_source = 'semrush',
  m1.fetched_at = datetime()
SET m1.updated_at = datetime();

MERGE (m2:SEOKeywordMetrics {key: 'metrics-kw-creer-qr-code-personnalise'})
ON CREATE SET
  m2.keyword_key = 'kw-creer-qr-code-personnalise',
  m2.volume = 1800,
  m2.difficulty = 28,
  m2.cpc = 0.38,
  m2.trend = 'rising',
  m2.competition = 'low',
  m2.serp_features = ['featured_snippet', 'videos'],
  m2.data_source = 'semrush',
  m2.fetched_at = datetime()
SET m2.updated_at = datetime();

MERGE (m3:SEOKeywordMetrics {key: 'metrics-kw-generateur-qr-code-design'})
ON CREATE SET
  m3.keyword_key = 'kw-generateur-qr-code-design',
  m3.volume = 880,
  m3.difficulty = 22,
  m3.cpc = 0.25,
  m3.trend = 'stable',
  m3.competition = 'low',
  m3.serp_features = ['images'],
  m3.data_source = 'semrush',
  m3.fetched_at = datetime()
SET m3.updated_at = datetime();

MERGE (m4:SEOKeywordMetrics {key: 'metrics-kw-qr-code-avec-logo'})
ON CREATE SET
  m4.keyword_key = 'kw-qr-code-avec-logo',
  m4.volume = 1200,
  m4.difficulty = 30,
  m4.cpc = 0.42,
  m4.trend = 'rising',
  m4.competition = 'medium',
  m4.serp_features = ['featured_snippet', 'images', 'videos'],
  m4.data_source = 'semrush',
  m4.fetched_at = datetime()
SET m4.updated_at = datetime();


// Link SEOKeyword → SEOKeywordMetrics
MATCH (kw:SEOKeyword {key: 'kw-qr-code-personnalise'})
MATCH (m:SEOKeywordMetrics {key: 'metrics-kw-qr-code-personnalise'})
MERGE (kw)-[:HAS_METRICS]->(m);

MATCH (kw:SEOKeyword {key: 'kw-creer-qr-code-personnalise'})
MATCH (m:SEOKeywordMetrics {key: 'metrics-kw-creer-qr-code-personnalise'})
MERGE (kw)-[:HAS_METRICS]->(m);

MATCH (kw:SEOKeyword {key: 'kw-generateur-qr-code-design'})
MATCH (m:SEOKeywordMetrics {key: 'metrics-kw-generateur-qr-code-design'})
MERGE (kw)-[:HAS_METRICS]->(m);

MATCH (kw:SEOKeyword {key: 'kw-qr-code-avec-logo'})
MATCH (m:SEOKeywordMetrics {key: 'metrics-kw-qr-code-avec-logo'})
MERGE (kw)-[:HAS_METRICS]->(m);


// ─────────────────────────────────────────────────────────────────────────────
// 7. ENTITY CONTENT: Add content for supporting entities (fr-FR)
// ─────────────────────────────────────────────────────────────────────────────

// EntityContent for qr-code-color
MATCH (e:Entity {key: 'qr-code-color'}), (l:Locale {key: 'fr-FR'})
MERGE (ec:EntityContent {key: 'entity:qr-code-color@fr-FR'})
ON CREATE SET
  ec.entity_key = e.key,
  ec.locale_key = l.key,
  ec.display_name = 'Couleurs QR Code',
  ec.title = 'Personnalisez les Couleurs de Votre QR Code',
  ec.tagline = 'Des millions de combinaisons possibles',
  ec.description = 'Choisissez parmi une palette infinie de couleurs pour créer un QR code unique à votre image de marque.',
  ec.llm_context = 'USE: Color customization features for QR codes in French.',
  ec.created_at = datetime()
SET ec.updated_at = datetime()
MERGE (e)-[:HAS_CONTENT]->(ec)
MERGE (ec)-[:FOR_LOCALE]->(l);

// EntityContent for qr-code-shapes
MATCH (e:Entity {key: 'qr-code-shapes'}), (l:Locale {key: 'fr-FR'})
MERGE (ec:EntityContent {key: 'entity:qr-code-shapes@fr-FR'})
ON CREATE SET
  ec.entity_key = e.key,
  ec.locale_key = l.key,
  ec.display_name = 'Formes QR Code',
  ec.title = 'Choisissez la Forme de Vos Modules QR',
  ec.tagline = 'Carrés, ronds, ou personnalisés',
  ec.description = 'Sélectionnez parmi différentes formes de modules pour un QR code original et reconnaissable.',
  ec.llm_context = 'USE: Shape customization features for QR codes in French.',
  ec.created_at = datetime()
SET ec.updated_at = datetime()
MERGE (e)-[:HAS_CONTENT]->(ec)
MERGE (ec)-[:FOR_LOCALE]->(l);

// EntityContent for qr-code-generator
MATCH (e:Entity {key: 'qr-code-generator'}), (l:Locale {key: 'fr-FR'})
MERGE (ec:EntityContent {key: 'entity:qr-code-generator@fr-FR'})
ON CREATE SET
  ec.entity_key = e.key,
  ec.locale_key = l.key,
  ec.display_name = 'Générateur QR Code',
  ec.title = 'Générateur de QR Code Gratuit',
  ec.tagline = 'Créez votre QR code en 30 secondes',
  ec.description = 'Notre outil en ligne vous permet de créer des QR codes professionnels gratuitement.',
  ec.llm_context = 'USE: QR code generator tool description in French.',
  ec.created_at = datetime()
SET ec.updated_at = datetime()
MERGE (e)-[:HAS_CONTENT]->(ec)
MERGE (ec)-[:FOR_LOCALE]->(l);


// ─────────────────────────────────────────────────────────────────────────────
// 8. FIX: Update HAS_BLOCK to use 'position' instead of 'order'
// ─────────────────────────────────────────────────────────────────────────────

// Update HAS_BLOCK relationships to use 'position' property
MATCH (p:Page {key: 'custom-qr-code'})-[r:HAS_BLOCK]->(b:Block)
SET r.position = r.order
REMOVE r.order;


// ═══════════════════════════════════════════════════════════════════════════════
// SUMMARY: Phase 17 adds the following for page-pipeline-overview v2.0.0
// ═══════════════════════════════════════════════════════════════════════════════
//
// PROJECT LAYER:
//   - Project(qrcode-ai) -[:HAS_PAGE]-> Page(custom-qr-code)
//   - Project(qrcode-ai) -[:HAS_ENTITY]-> Entity(custom-qr-code)
//   - Project(qrcode-ai) -[:SUPPORTS_LOCALE]-> Locale(fr-FR)
//
// LOCALE LAYER:
//   - Locale(fr-FR) -[:HAS_SLUGIFICATION]-> Slugification(fr-FR)
//   - Locale(fr-FR) -[:HAS_STYLE]-> Style(fr-FR)
//
// SEMANTIC LAYER:
//   - Page -[:USES_ENTITY {purpose, temperature}]-> Entity (2 links)
//   - Block -[:REFERENCES {purpose, count}]-> Entity (3 links)
//   - BlockInstruction -[:REFERENCES_ENTITY {syntax, context}]-> Entity (4 links)
//
// SEO LAYER:
//   - Entity -[:HAS_KEYWORD {rank}]-> SEOKeyword (4 keywords)
//   - SEOKeyword -[:HAS_METRICS]-> SEOKeywordMetrics (4 metrics)
//
// ENTITY CONTENT:
//   - qr-code-color@fr-FR
//   - qr-code-shapes@fr-FR
//   - qr-code-generator@fr-FR
//
// ═══════════════════════════════════════════════════════════════════════════════
