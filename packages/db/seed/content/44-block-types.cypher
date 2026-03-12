// =============================================================================
// NovaNet BlockType Seed v0.19.0
// =============================================================================
// All BlockType nodes for page structure.
// Must run before: 45-blocktype-head-seo-meta.cypher, 40-page-block-instances.cypher
//
// Categories:
// - header: Above-the-fold blocks (hero, head-seo-meta)
// - body: Main content blocks (faq, cta, features, pricing, testimonials, grid, content)
// - footer: Page footer blocks
// =============================================================================

// -----------------------------------------------------------------------------
// CONSTRAINT
// -----------------------------------------------------------------------------
CREATE CONSTRAINT block_type_key IF NOT EXISTS FOR (bt:BlockType) REQUIRE bt.key IS UNIQUE;
CREATE INDEX block_type_category IF NOT EXISTS FOR (bt:BlockType) ON (bt.category);

// -----------------------------------------------------------------------------
// HEADER BLOCK TYPES
// -----------------------------------------------------------------------------

MERGE (bt:BlockType {key: 'hero'})
SET bt.display_name = 'Hero Section',
    bt.content = 'Full-width hero with title, subtitle, and primary CTA button. Used for main page headers.',
    bt.category = 'header',
    bt.structure = 'schemas/hero.json',
    bt.llm_context = 'USE: for main page headers with attention-grabbing content. TRIGGERS: hero, header, banner, landing, above-the-fold. NOT: content sections (use body blocks), navigation (use nav blocks). RELATES: Block (typed_by), BlockNative (generates to).',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'head-seo-meta'})
SET bt.display_name = 'SEO Metadata',
    bt.content = 'SEO metadata block - MUST be first block (order=0) of every page. Contains URL slug, meta title, meta description.',
    bt.category = 'header',
    bt.structure = 'schemas/head-seo-meta.json',
    bt.schema = '{"type":"object","properties":{"slug":{"type":"string","pattern":"^[\\p{Ll}\\p{N}\\-]+$"},"meta_title":{"type":"string","maxLength":60},"meta_description":{"type":"string","maxLength":160}},"required":["slug","meta_title","meta_description"]}',
    bt.llm_context = 'USE: when generating SEO metadata for a page. TRIGGERS: slug, URL, meta title, meta description, SEO, head. NOT: for page content (use other block types), for keywords (use TARGETS). RELATES: BlockNative (output), Slugification (rules via SLUGIFIED_BY), SEOKeyword (source via DERIVED_SLUG_FROM).',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

// -----------------------------------------------------------------------------
// BODY BLOCK TYPES
// -----------------------------------------------------------------------------

MERGE (bt:BlockType {key: 'faq'})
SET bt.display_name = 'FAQ Section',
    bt.content = 'Accordion-style frequently asked questions with expandable answers. Targets search snippets.',
    bt.category = 'body',
    bt.structure = 'schemas/faq.json',
    bt.llm_context = 'USE: for question-answer content targeting search snippets. TRIGGERS: faq, questions, answers, help, support. NOT: general content (use prose), pricing (use pricing block). RELATES: Block (typed_by), SEOKeyword (targets).',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'cta'})
SET bt.display_name = 'Call to Action',
    bt.content = 'Conversion-focused section with primary action button.',
    bt.category = 'body',
    bt.structure = 'schemas/cta.json',
    bt.llm_context = 'USE: for conversion points and action prompts. TRIGGERS: cta, action, button, convert, signup. NOT: informational content (use prose), navigation (use nav). RELATES: Block (typed_by), Entity (references).',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'pricing'})
SET bt.display_name = 'Pricing Table',
    bt.content = 'Comparison table with pricing tiers and features.',
    bt.category = 'body',
    bt.structure = 'schemas/pricing.json',
    bt.llm_context = 'USE: for pricing comparisons and tier displays. TRIGGERS: pricing, plans, tiers, cost, subscription. NOT: feature lists (use features block), CTA only (use cta block). RELATES: Block (typed_by).',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'features'})
SET bt.display_name = 'Features Grid',
    bt.content = 'Grid layout showcasing product features with icons.',
    bt.category = 'body',
    bt.structure = 'schemas/features.json',
    bt.llm_context = 'USE: for feature highlights and capability showcases. TRIGGERS: features, capabilities, benefits, grid, icons. NOT: pricing (use pricing block), testimonials (use testimonials block). RELATES: Block (typed_by), Entity (references features).',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'testimonials'})
SET bt.display_name = 'Testimonials',
    bt.content = 'Customer quotes and social proof slider.',
    bt.category = 'body',
    bt.structure = 'schemas/testimonials.json',
    bt.llm_context = 'USE: for social proof and customer validation. TRIGGERS: testimonials, reviews, quotes, social proof, customers. NOT: features (use features block), pricing (use pricing block).',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'grid'})
SET bt.display_name = 'Grid',
    bt.content = 'Grid layout block for displaying multiple items in a grid format.',
    bt.category = 'body',
    bt.structure = 'schemas/grid.json',
    bt.llm_context = 'USE: for grid layouts showing multiple items. TRIGGERS: grid, cards, gallery, showcase. NOT: for single content blocks.',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'content'})
SET bt.display_name = 'Content',
    bt.content = 'General content block for text, explanations, and informational content.',
    bt.category = 'body',
    bt.structure = 'schemas/content.json',
    bt.llm_context = 'USE: for body content, explanations, descriptions. TRIGGERS: content, text, body, explanation. NOT: for headers, CTAs, or structured blocks.',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'seo-meta'})
SET bt.display_name = 'SEO Meta',
    bt.content = 'SEO metadata block for page title, description, and structured data.',
    bt.category = 'body',
    bt.structure = 'schemas/seo-meta.json',
    bt.llm_context = 'USE: for SEO metadata, meta tags, structured data. TRIGGERS: meta, seo, title, description, schema. NOT: for visible content.',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

// -----------------------------------------------------------------------------
// FOOTER BLOCK TYPES
// -----------------------------------------------------------------------------

MERGE (bt:BlockType {key: 'footer'})
SET bt.display_name = 'Footer Section',
    bt.content = 'Page footer with links, legal info, and secondary navigation.',
    bt.category = 'footer',
    bt.structure = 'schemas/footer.json',
    bt.llm_context = 'USE: for page footers with navigation and legal links. TRIGGERS: footer, bottom, links, legal, navigation. NOT: main content (use body blocks), header (use hero).',
    bt.node_class = 'BlockType',
    bt.provenance = '{"source":"seed","file":"content/44-block-types.cypher"}',
    bt.created_at = coalesce(bt.created_at, datetime()),
    bt.updated_at = datetime();

// -----------------------------------------------------------------------------
// VERIFICATION
// -----------------------------------------------------------------------------
MATCH (bt:BlockType)
RETURN bt.category AS category, collect(bt.key) AS types, count(*) AS count
ORDER BY bt.category;
