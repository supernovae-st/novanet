// ═══════════════════════════════════════════════════════════════════════════════
// Full Pipeline Example for: Custom QR Code Page (v2 - COMPREHENSIVE)
// ═══════════════════════════════════════════════════════════════════════════════
// This file creates a complete vertical slice for a single page, including
// all template, context, and generated nodes to validate the end-to-end architecture.
// ═══════════════════════════════════════════════════════════════════════════════


// ─────────────────────────────────────────────────────────────────────────────
// 0. SETUP: Locale, Main Entity, and Base Context
// ─────────────────────────────────────────────────────────────────────────────

// Ensure fr-FR Locale and custom-qr-code Entity exist
MERGE (l:Locale {key: "fr-FR"})
MERGE (e:Entity {key: 'custom-qr-code'})
ON CREATE SET e.display_name = 'Custom QR Code', e.created_at = datetime()
SET e.updated_at = datetime();

// Create localized EntityContent for the page slug & SEO
MATCH (e:Entity {key: 'custom-qr-code'}), (l:Locale {key: 'fr-FR'})
MERGE (ec:EntityContent {key: 'entity:custom-qr-code@fr-FR'})
ON CREATE SET ec.entity_key = e.key, ec.locale_key = l.key, ec.slug = 'qr-code-personnalise', ec.full_path = '/fr/qr-code-personnalise', ec.display_name = 'QR Code Personnalisé', ec.description = 'Créez des QR codes uniques avec vos couleurs, formes et logo.', ec.created_at = datetime()
SET ec.updated_at = datetime()
MERGE (e)-[:HAS_CONTENT]->(ec)
MERGE (ec)-[:FOR_LOCALE]->(l);

// Create associated context nodes for the fr-FR Locale
MERGE (c:Culture {key: 'culture-fr-FR', name: 'French Culture'}) SET c.tone = 'formal', c.formality = 'vous';
MERGE (m:Market {key: 'market-fr-FR', name: 'French Market'}) SET m.main_competitor = 'competitor.fr';
MERGE (l)-[:HAS_CULTURE]->(c);
MERGE (l)-[:HAS_MARKET]->(m);

// Create SEO Keywords and link them
MERGE (kw1:SEOKeyword {key: 'qr-code-sur-mesure'});
MERGE (kw2:SEOKeyword {key: 'generateur-qr-france'});
MERGE (ec)-[:TARGETS]->(kw1);
MERGE (ec)-[:TARGETS]->(kw2);


// ─────────────────────────────────────────────────────────────────────────────
// 1. INVARIANT BLUEPRINT (Templates & Rules)
// ─────────────────────────────────────────────────────────────────────────────

// 1a. Create reusable BlockRules
MERGE (br:BlockRules {key: 'std-text-rules-v1'})
ON CREATE SET br.display_name = 'Standard Text Rules v1', br.rules = 'title max 80 chars, description max 300 chars', br.version = '1.0', br.active = true;

// 1b. Create BlockTypes
MERGE (bt1:BlockType {key: 'hero-interactive-v1'}) SET bt1.display_name = 'Interactive Hero v1', bt1.structure = '{"title", "subtitle", "cta"}';
MERGE (bt2:BlockType {key: 'feature-grid-v1'}) SET bt2.display_name = 'Feature Grid v1', bt2.structure = '{"title", "features": []}';
MERGE (bt3:BlockType {key: 'steps-guide-v1'}) SET bt3.display_name = 'Steps Guide v1', bt3.structure = '{"title", "steps": []}';
MERGE (bt4:BlockType {key: 'cta-banner-v1'}) SET bt4.display_name = 'CTA Banner v1', bt4.structure = '{"title", "cta_text"}';

// 1c. Link BlockTypes to their rules
MATCH (br:BlockRules {key: 'std-text-rules-v1'})
MATCH (bt1:BlockType {key: 'hero-interactive-v1'})
MATCH (bt2:BlockType {key: 'feature-grid-v1'})
MATCH (bt3:BlockType {key: 'steps-guide-v1'})
MATCH (bt4:BlockType {key: 'cta-banner-v1'})
MERGE (bt1)-[:HAS_RULES]->(br);
MERGE (bt2)-[:HAS_RULES]->(br);
MERGE (bt3)-[:HAS_RULES]->(br);
MERGE (bt4)-[:HAS_RULES]->(br);

// 1d. Create BlockInstructions
MERGE (bi1:BlockInstruction {key: 'instr-custom-qr-hero'}) SET bi1.content = 'Generate a compelling hero section for @entity:custom-qr-code.';
MERGE (bi2:BlockInstruction {key: 'instr-custom-qr-features'}) SET bi2.content = 'List the key design features, referencing @entity:qr-code-color and @entity:qr-code-shapes.';
MERGE (bi3:BlockInstruction {key: 'instr-custom-qr-steps'}) SET bi3.content = 'Explain the creation process step-by-step.';
MERGE (bi4:BlockInstruction {key: 'instr-custom-qr-cta'}) SET bi4.content = 'Create a strong call to action to try the @entity:qr-code-generator.';


// ─────────────────────────────────────────────────────────────────────────────
// 2. INVARIANT STRUCTURE (Page and Blocks)
// ─────────────────────────────────────────────────────────────────────────────

// 2a. Create the Page node
MATCH (e:Entity {key: 'custom-qr-code'})
MERGE (p:Page {key: 'custom-qr-code'})
ON CREATE SET p.slug = 'custom-qr-code', p.display_name = 'Custom QR Code Page', p.created_at = datetime()
SET p.updated_at = datetime()
MERGE (p)-[:REPRESENTS]->(e);

// 2b. Create the Block nodes
MERGE (b1:Block {key: 'blk-custom-qr-code-designer-hero'}) SET b1.display_name = 'Custom QR Code Designer Hero', b1.anchor_id = 'designer-hero';
MERGE (b2:Block {key: 'blk-custom-qr-code-design-features'}) SET b2.display_name = 'Design Features', b2.anchor_id = 'design-features';
MERGE (b3:Block {key: 'blk-custom-qr-code-how-it-works'}) SET b3.display_name = 'How It Works', b3.anchor_id = 'how-it-works';
MERGE (b4:Block {key: 'blk-custom-qr-code-generator-cta'}) SET b4.display_name = 'Generator CTA', b4.anchor_id = 'generator-cta';

// 2c. Connect Page to Blocks (single statement to preserve variable scope)
MATCH (p:Page {key: 'custom-qr-code'})
MATCH (b1:Block {key: 'blk-custom-qr-code-designer-hero'})
MATCH (b2:Block {key: 'blk-custom-qr-code-design-features'})
MATCH (b3:Block {key: 'blk-custom-qr-code-how-it-works'})
MATCH (b4:Block {key: 'blk-custom-qr-code-generator-cta'})
MERGE (p)-[r1:HAS_BLOCK]->(b1) SET r1.order = 1
MERGE (p)-[r2:HAS_BLOCK]->(b2) SET r2.order = 2
MERGE (p)-[r3:HAS_BLOCK]->(b3) SET r3.order = 3
MERGE (p)-[r4:HAS_BLOCK]->(b4) SET r4.order = 4;

// 2d. Connect Blocks to their templates and instructions
MATCH (b1:Block {key: 'blk-custom-qr-code-designer-hero'}), (bt1:BlockType {key: 'hero-interactive-v1'}), (bi1:BlockInstruction {key: 'instr-custom-qr-hero'})
MERGE (b1)-[:OF_TYPE]->(bt1)
MERGE (b1)-[:HAS_INSTRUCTION]->(bi1);

MATCH (b2:Block {key: 'blk-custom-qr-code-design-features'}), (bt2:BlockType {key: 'feature-grid-v1'}), (bi2:BlockInstruction {key: 'instr-custom-qr-features'})
MERGE (b2)-[:OF_TYPE]->(bt2)
MERGE (b2)-[:HAS_INSTRUCTION]->(bi2);

MATCH (b3:Block {key: 'blk-custom-qr-code-how-it-works'}), (bt3:BlockType {key: 'steps-guide-v1'}), (bi3:BlockInstruction {key: 'instr-custom-qr-steps'})
MERGE (b3)-[:OF_TYPE]->(bt3)
MERGE (b3)-[:HAS_INSTRUCTION]->(bi3);

MATCH (b4:Block {key: 'blk-custom-qr-code-generator-cta'}), (bt4:BlockType {key: 'cta-banner-v1'}), (bi4:BlockInstruction {key: 'instr-custom-qr-cta'})
MERGE (b4)-[:OF_TYPE]->(bt4)
MERGE (b4)-[:HAS_INSTRUCTION]->(bi4);


// ─────────────────────────────────────────────────────────────────────────────
// 3. GENERATED OUTPUT (Mock data for fr-FR locale)
// ─────────────────────────────────────────────────────────────────────────────

// 3a. Create mock BlockGenerated nodes
MATCH (b1:Block {key: 'blk-custom-qr-code-designer-hero'}), (l:Locale {key: 'fr-FR'})
MERGE (bg1:BlockGenerated {key: 'block:blk-custom-qr-code-designer-hero@fr-FR'})
ON CREATE SET bg1.anchor_slug = 'designer-hero', bg1.generated = '{"title": "Votre QR Code, Votre Style", "subtitle": "Créez un design unique en quelques clics."}', bg1.created_at = datetime()
SET bg1.updated_at = datetime()
MERGE (b1)-[:HAS_GENERATED]->(bg1) MERGE (bg1)-[:FOR_LOCALE]->(l);

MATCH (b2:Block {key: 'blk-custom-qr-code-design-features'}), (l:Locale {key: 'fr-FR'})
MERGE (bg2:BlockGenerated {key: 'block:blk-custom-qr-code-design-features@fr-FR'})
ON CREATE SET bg2.anchor_slug = 'fonctionnalites-design', bg2.generated = '{"title": "Personnalisation Complète", "features": ["Couleurs", "Formes", "Logos"]}', bg2.created_at = datetime()
SET bg2.updated_at = datetime()
MERGE (b2)-[:HAS_GENERATED]->(bg2) MERGE (bg2)-[:FOR_LOCALE]->(l);

MATCH (b3:Block {key: 'blk-custom-qr-code-how-it-works'}), (l:Locale {key: 'fr-FR'})
MERGE (bg3:BlockGenerated {key: 'block:blk-custom-qr-code-how-it-works@fr-FR'})
ON CREATE SET bg3.anchor_slug = 'comment-ca-marche', bg3.generated = '{"title": "Simple comme 1-2-3", "steps": ["Choisissez", "Personnalisez", "Téléchargez"]}', bg3.created_at = datetime()
SET bg3.updated_at = datetime()
MERGE (b3)-[:HAS_GENERATED]->(bg3) MERGE (bg3)-[:FOR_LOCALE]->(l);

MATCH (b4:Block {key: 'blk-custom-qr-code-generator-cta'}), (l:Locale {key: 'fr-FR'})
MERGE (bg4:BlockGenerated {key: 'block:blk-custom-qr-code-generator-cta@fr-FR'})
ON CREATE SET bg4.anchor_slug = 'creer-mon-qr', bg4.generated = '{"title": "Prêt à créer ?", "cta_text": "Essayer le générateur"}', bg4.created_at = datetime()
SET bg4.updated_at = datetime()
MERGE (b4)-[:HAS_GENERATED]->(bg4) MERGE (bg4)-[:FOR_LOCALE]->(l);


// ─────────────────────────────────────────────────────────────────────────────
// 4. FINAL ASSEMBLY
// ─────────────────────────────────────────────────────────────────────────────

// 4a. Create the PageGenerated node
MATCH (p:Page {key: 'custom-qr-code'}), (l:Locale {key: 'fr-FR'})
MERGE (pg:PageGenerated {key: 'page:custom-qr-code@fr-FR'})
ON CREATE SET pg.created_at = datetime()
SET pg.updated_at = datetime()
MERGE (p)-[:HAS_GENERATED]->(pg) MERGE (pg)-[:FOR_LOCALE]->(l);

// 4b. Assemble the BlockGenerated nodes into the PageGenerated node (single statement)
MATCH (pg:PageGenerated {key: 'page:custom-qr-code@fr-FR'})
MATCH (bg1:BlockGenerated {key: 'block:blk-custom-qr-code-designer-hero@fr-FR'})
MATCH (bg2:BlockGenerated {key: 'block:blk-custom-qr-code-design-features@fr-FR'})
MATCH (bg3:BlockGenerated {key: 'block:blk-custom-qr-code-how-it-works@fr-FR'})
MATCH (bg4:BlockGenerated {key: 'block:blk-custom-qr-code-generator-cta@fr-FR'})
MERGE (pg)-[r1:ASSEMBLES]->(bg1) SET r1.order = 1
MERGE (pg)-[r2:ASSEMBLES]->(bg2) SET r2.order = 2
MERGE (pg)-[r3:ASSEMBLES]->(bg3) SET r3.order = 3
MERGE (pg)-[r4:ASSEMBLES]->(bg4) SET r4.order = 4;
