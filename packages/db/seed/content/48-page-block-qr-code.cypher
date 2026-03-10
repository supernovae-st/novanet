// ═══════════════════════════════════════════════════════════════════════════════
// 48-page-block-qr-code.cypher — Page and Block parents for QR Code
// v0.18.0 - Creates parent nodes BEFORE their *Native children
// ═══════════════════════════════════════════════════════════════════════════════
//
// IMPORTANT: This file MUST run BEFORE:
//   - 49-blocknative-head-seo-meta.cypher (creates BlockNative children)
//   - 50-page-native.cypher (creates PageNative children)
//
// ═══════════════════════════════════════════════════════════════════════════════

// -----------------------------------------------------------------------------
// Page: QR Code Generator (main page)
// -----------------------------------------------------------------------------
MERGE (p:Page {key: 'page:qr-code'})
ON CREATE SET
  p.display_name = 'QR Code Generator',
  p.description = 'Create custom QR codes for any purpose - free online QR code generator',
  p.llm_context = 'USE: as the main landing page for QR code creation. TRIGGERS: homepage, main page, generator page, create qr. NOT: for specific QR code types (use subpages). RELATES: Entity:qr-code (represents), Block:head-seo-meta (contains).',
  p.created_by = 'content:bootstrap',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.updated_at = datetime();

// Link Page → Entity (REPRESENTS)
MATCH (p:Page {key: 'page:qr-code'})
MATCH (e:Entity {key: 'entity:qr-code'})
MERGE (p)-[:REPRESENTS]->(e);

// -----------------------------------------------------------------------------
// Block: QR Code SEO Meta
// -----------------------------------------------------------------------------
MERGE (b:Block {key: 'block:qr-code:head-seo-meta:1'})
ON CREATE SET
  b.display_name = 'QR Code SEO Meta',
  b.description = 'SEO metadata block for QR code page - contains meta title, description, and keywords',
  b.llm_context = 'USE: for meta title, description, keywords in HTML head. TRIGGERS: seo, meta, head, title tag, meta description. NOT: for visible page content (use content blocks). RELATES: Page:qr-code (belongs to), BlockType:head-seo-meta (type).',
  b.anchor_id = 'head-seo-meta',
  b.created_by = 'content:bootstrap',
  b.created_at = datetime(),
  b.updated_at = datetime()
ON MATCH SET
  b.updated_at = datetime();

// Link Block → Page (HAS_BLOCK)
MATCH (b:Block {key: 'block:qr-code:head-seo-meta:1'})
MATCH (p:Page {key: 'page:qr-code'})
MERGE (p)-[:HAS_BLOCK]->(b);

// Link Block → BlockType (OF_TYPE)
MATCH (b:Block {key: 'block:qr-code:head-seo-meta:1'})
MATCH (bt:BlockType {key: 'block-type:head-seo-meta'})
MERGE (b)-[:OF_TYPE]->(bt);

// =============================================================================
// Verification
// =============================================================================
RETURN 'Created Page:qr-code and Block:qr-code:head-seo-meta:1' AS status;
