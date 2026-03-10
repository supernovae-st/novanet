// ============================================================================
// PLAN B - Migration 090: Create BlockType Nodes
// ============================================================================
// Priority: STRUCTURE (Block template definitions)
// Fixes: Missing BlockType nodes for content blocks
// CSR Impact: Enables block structure validation and template reuse
// ============================================================================

// Create Hero block type
MERGE (bt:BlockType {key: 'hero'})
ON CREATE SET
  bt.display_name = 'Hero Section',
  bt.description = 'Full-width hero with title, subtitle, and primary CTA button',
  bt.llm_context = 'USE: for main page headers with attention-grabbing content. TRIGGERS: hero, header, banner, landing, above-the-fold. NOT: content sections (use body blocks), navigation (use nav blocks). RELATES: Block (typed_by), BlockNative (generates to).',
  bt.category = 'header',
  bt.structure = 'schemas/hero.json',
  bt.created_at = datetime(),
  bt.updated_at = datetime()
ON MATCH SET
  bt.updated_at = datetime();

// Create FAQ block type
MERGE (bt:BlockType {key: 'faq'})
ON CREATE SET
  bt.display_name = 'FAQ Section',
  bt.description = 'Accordion-style frequently asked questions with expandable answers',
  bt.llm_context = 'USE: for question-answer content targeting search snippets. TRIGGERS: faq, questions, answers, help, support. NOT: general content (use prose), pricing (use pricing block). RELATES: Block (typed_by), SEOKeyword (targets).',
  bt.category = 'body',
  bt.structure = 'schemas/faq.json',
  bt.created_at = datetime(),
  bt.updated_at = datetime()
ON MATCH SET
  bt.updated_at = datetime();

// Create CTA block type
MERGE (bt:BlockType {key: 'cta'})
ON CREATE SET
  bt.display_name = 'Call to Action',
  bt.description = 'Conversion-focused section with primary action button',
  bt.llm_context = 'USE: for conversion points and action prompts. TRIGGERS: cta, action, button, convert, signup. NOT: informational content (use prose), navigation (use nav). RELATES: Block (typed_by), Entity (references).',
  bt.category = 'body',
  bt.structure = 'schemas/cta.json',
  bt.created_at = datetime(),
  bt.updated_at = datetime()
ON MATCH SET
  bt.updated_at = datetime();

// Create Pricing block type
MERGE (bt:BlockType {key: 'pricing'})
ON CREATE SET
  bt.display_name = 'Pricing Table',
  bt.description = 'Comparison table with pricing tiers and features',
  bt.llm_context = 'USE: for pricing comparisons and tier displays. TRIGGERS: pricing, plans, tiers, cost, subscription. NOT: feature lists (use features block), CTA only (use cta block). RELATES: Block (typed_by).',
  bt.category = 'body',
  bt.structure = 'schemas/pricing.json',
  bt.created_at = datetime(),
  bt.updated_at = datetime()
ON MATCH SET
  bt.updated_at = datetime();

// Create Features block type
MERGE (bt:BlockType {key: 'features'})
ON CREATE SET
  bt.display_name = 'Features Grid',
  bt.description = 'Grid layout showcasing product features with icons',
  bt.llm_context = 'USE: for feature highlights and capability showcases. TRIGGERS: features, capabilities, benefits, grid, icons. NOT: pricing (use pricing block), testimonials (use testimonials block). RELATES: Block (typed_by), Entity (references features).',
  bt.category = 'body',
  bt.structure = 'schemas/features.json',
  bt.created_at = datetime(),
  bt.updated_at = datetime()
ON MATCH SET
  bt.updated_at = datetime();

// Create Testimonials block type
MERGE (bt:BlockType {key: 'testimonials'})
ON CREATE SET
  bt.display_name = 'Testimonials',
  bt.description = 'Customer quotes and social proof slider',
  bt.llm_context = 'USE: for social proof and customer validation. TRIGGERS: testimonials, reviews, quotes, social proof, customers. NOT: features (use features block), pricing (use pricing block).',
  bt.category = 'body',
  bt.structure = 'schemas/testimonials.json',
  bt.created_at = datetime(),
  bt.updated_at = datetime()
ON MATCH SET
  bt.updated_at = datetime();

// Create Footer block type
MERGE (bt:BlockType {key: 'footer'})
ON CREATE SET
  bt.display_name = 'Footer Section',
  bt.description = 'Page footer with links, legal info, and secondary navigation',
  bt.llm_context = 'USE: for page footers with navigation and legal links. TRIGGERS: footer, bottom, links, legal, navigation. NOT: main content (use body blocks), header (use hero).',
  bt.category = 'footer',
  bt.structure = 'schemas/footer.json',
  bt.created_at = datetime(),
  bt.updated_at = datetime()
ON MATCH SET
  bt.updated_at = datetime();

// Verify block types created
MATCH (bt:BlockType)
RETURN bt.key AS block_type,
       bt.display_name AS name,
       bt.category AS category,
       'Created' AS status
ORDER BY bt.category, bt.key;
