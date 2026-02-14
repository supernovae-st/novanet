// ═══════════════════════════════════════════════════════════════════════════════
// Page & Block Structure for Custom QR Code Page
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────
// Page: Custom QR Code
// ─────────────────────────────────────────────────────────────────────────────

// 1. Create the Page node for the 'custom-qr-code' Entity
MATCH (e:Entity {key: 'custom-qr-code'})
MERGE (p:Page {key: e.key})
ON CREATE SET
  p.slug = 'custom-qr-code',
  p.display_name = 'Custom QR Code Page',
  p.description = 'Page for designing and creating fully customized QR codes.',
  p.llm_context = 'USE: For generating the Custom QR Code page structure.',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.updated_at = datetime()
MERGE (p)-[:REPRESENTS]->(e);


// 2. Create the Block nodes for the Custom QR Code page
MERGE (b1:Block {key: 'blk-custom-qr-code-designer-hero'})
SET
  b1.display_name = 'Custom QR Code Designer Hero',
  b1.anchor_id = 'designer-hero',
  b1.description = 'Hero section with interactive QR code designer.',
  b1.llm_context = 'USE: For the hero section of the Custom QR Code page.',
  b1.created_at = datetime(),
  b1.updated_at = datetime();

MERGE (b2:Block {key: 'blk-custom-qr-code-design-features'})
SET
  b2.display_name = 'Design Features',
  b2.anchor_id = 'design-features',
  b2.description = 'Grid showcasing design features like colors, shapes, and logos.',
  b2.llm_context = 'USE: For the design features section.',
  b2.created_at = datetime(),
  b2.updated_at = datetime();

MERGE (b3:Block {key: 'blk-custom-qr-code-how-it-works'})
SET
  b3.display_name = 'How It Works',
  b3.anchor_id = 'how-it-works',
  b3.description = 'A step-by-step guide on how to create a custom QR code.',
  b3.llm_context = 'USE: For the step-by-step guide section.',
  b3.created_at = datetime(),
  b3.updated_at = datetime();

MERGE (b4:Block {key: 'blk-custom-qr-code-generator-cta'})
SET
  b4.display_name = 'Generator CTA',
  b4.anchor_id = 'generator-cta',
  b4.description = 'A call-to-action banner linking to the generator tool.',
  b4.llm_context = 'USE: For the final call-to-action.',
  b4.created_at = datetime(),
  b4.updated_at = datetime();


// 3. Connect Blocks to the Page with order
MATCH (p:Page {key: 'custom-qr-code'})
MATCH (b1:Block {key: 'blk-custom-qr-code-designer-hero'})
MATCH (b2:Block {key: 'blk-custom-qr-code-design-features'})
MATCH (b3:Block {key: 'blk-custom-qr-code-how-it-works'})
MATCH (b4:Block {key: 'blk-custom-qr-code-generator-cta'})
MERGE (p)-[r1:HAS_BLOCK]->(b1) SET r1.order = 1;
MERGE (p)-[r2:HAS_BLOCK]->(b2) SET r2.order = 2;
MERGE (p)-[r3:HAS_BLOCK]->(b3) SET r3.order = 3;
MERGE (p)-[r4:HAS_BLOCK]->(b4) SET r4.order = 4;
