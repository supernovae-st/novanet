// ═══════════════════════════════════════════════════════════════════════════════
// Page & Block Instance Data for MCP Testing
// ═══════════════════════════════════════════════════════════════════════════════
//
// Creates Page and Block nodes linked to existing Entities
// Required for novanet_generate MCP tool to work
//
// v0.14.0 - ADR-029 *Native Pattern, ADR-030 Slug Ownership
//

// ─────────────────────────────────────────────────────────────────────────────
// Page: QR Code Landing Page
// ─────────────────────────────────────────────────────────────────────────────

// 1. Create the Page node for the 'qr-code' Entity
MATCH (e:Entity {key: 'entity:qr-code'})
MERGE (p:Page {key: 'page:qr-code-landing'})
ON CREATE SET
  p.slug = 'qr-code',
  p.display_name = 'QR Code Landing Page',
  p.description = 'Main landing page for QR Code AI - explains what QR codes are and how to create them.',
  p.llm_context = 'USE: For generating the main QR Code landing page. TRIGGERS: qr code, landing, home. NOT: specific use cases or advanced features.',
  p.created_by = 'content:bootstrap',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.updated_at = datetime();

// Create REPRESENTS relationship (Page represents Entity)
MATCH (p:Page {key: 'page:qr-code-landing'})
MATCH (e:Entity {key: 'entity:qr-code'})
MERGE (p)-[:REPRESENTS]->(e);


// ─────────────────────────────────────────────────────────────────────────────
// Blocks: QR Code Landing Page Blocks
// ─────────────────────────────────────────────────────────────────────────────

// Block 1: Hero Section
MERGE (b1:Block {key: 'block:qr-code-hero'})
ON CREATE SET
  b1.display_name = 'QR Code Hero',
  b1.anchor_id = 'hero',
  b1.description = 'Hero section introducing QR codes with a CTA to the generator.',
  b1.llm_context = 'USE: For hero section generation. Include: headline, subheadline, CTA button.',
  b1.block_type = 'hero',
  b1.created_by = 'content:bootstrap',
  b1.created_at = datetime(),
  b1.updated_at = datetime()
ON MATCH SET
  b1.updated_at = datetime();

// Block 2: What is a QR Code
MERGE (b2:Block {key: 'block:qr-code-what-is'})
ON CREATE SET
  b2.display_name = 'What is a QR Code',
  b2.anchor_id = 'what-is-qr-code',
  b2.description = 'Educational section explaining what QR codes are.',
  b2.llm_context = 'USE: For educational content. Explain QR code basics, history, how they work.',
  b2.block_type = 'content',
  b2.created_by = 'content:bootstrap',
  b2.created_at = datetime(),
  b2.updated_at = datetime()
ON MATCH SET
  b2.updated_at = datetime();

// Block 3: Use Cases
MERGE (b3:Block {key: 'block:qr-code-use-cases'})
ON CREATE SET
  b3.display_name = 'QR Code Use Cases',
  b3.anchor_id = 'use-cases',
  b3.description = 'Grid showcasing different QR code use cases.',
  b3.llm_context = 'USE: For use case showcase. Include: business, marketing, events, personal.',
  b3.block_type = 'grid',
  b3.created_by = 'content:bootstrap',
  b3.created_at = datetime(),
  b3.updated_at = datetime()
ON MATCH SET
  b3.updated_at = datetime();

// Block 4: CTA
MERGE (b4:Block {key: 'block:qr-code-cta'})
ON CREATE SET
  b4.display_name = 'Create QR Code CTA',
  b4.anchor_id = 'create-cta',
  b4.description = 'Call-to-action to start creating QR codes.',
  b4.llm_context = 'USE: For final CTA. Strong conversion-focused copy.',
  b4.block_type = 'cta',
  b4.created_by = 'content:bootstrap',
  b4.created_at = datetime(),
  b4.updated_at = datetime()
ON MATCH SET
  b4.updated_at = datetime();


// ─────────────────────────────────────────────────────────────────────────────
// Connect Blocks to Page with order
// ─────────────────────────────────────────────────────────────────────────────

MATCH (p:Page {key: 'page:qr-code-landing'})
MATCH (b1:Block {key: 'block:qr-code-hero'})
MERGE (p)-[r1:HAS_BLOCK]->(b1) ON CREATE SET r1.order = 1;

MATCH (p:Page {key: 'page:qr-code-landing'})
MATCH (b2:Block {key: 'block:qr-code-what-is'})
MERGE (p)-[r2:HAS_BLOCK]->(b2) ON CREATE SET r2.order = 2;

MATCH (p:Page {key: 'page:qr-code-landing'})
MATCH (b3:Block {key: 'block:qr-code-use-cases'})
MERGE (p)-[r3:HAS_BLOCK]->(b3) ON CREATE SET r3.order = 3;

MATCH (p:Page {key: 'page:qr-code-landing'})
MATCH (b4:Block {key: 'block:qr-code-cta'})
MERGE (p)-[r4:HAS_BLOCK]->(b4) ON CREATE SET r4.order = 4;


// ─────────────────────────────────────────────────────────────────────────────
// Verification Query
// ─────────────────────────────────────────────────────────────────────────────

MATCH (p:Page {key: 'page:qr-code-landing'})-[r:HAS_BLOCK]->(b:Block)
RETURN p.key AS page, b.key AS block, r.order AS order
ORDER BY r.order;
