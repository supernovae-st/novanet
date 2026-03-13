// NovaNet EntityCategory Seed v11.0
//
// 13 categories for Entity.type classification.
// Based on semantic question groups (WHAT? WHERE? WHY? WHO? HOW? ABSTRACT EXTERNAL).
//
// Must run after: 00-constraints.cypher, 01-classes.cypher
// Must run before: 10-entities-*.cypher

// =============================================================================
// CONSTRAINT
// =============================================================================

CREATE CONSTRAINT entity_category_key IF NOT EXISTS FOR (ec:EntityCategory) REQUIRE ec.key IS UNIQUE;
CREATE INDEX entity_category_question IF NOT EXISTS FOR (ec:EntityCategory) ON (ec.question);

// =============================================================================
// ENTITY CATEGORIES (13)
// =============================================================================

// --- WHAT? (Products, Content, Features, Tools) ---

MERGE (ec:EntityCategory {key: 'THING'})
SET ec.display_name = 'Thing',
    ec.content = 'Core products and objects (QR Code, Smart Link, Barcode)',
    ec.question = 'WHAT?',
    ec.sort_order = 1,
    ec.llm_context = 'USE: for nouns representing products, concepts, or entities. TRIGGERS: thing, product, object, concept, item, entity. NOT: actions (use ACTION), content formats (use CONTENT_TYPE).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'CONTENT_TYPE'})
SET ec.display_name = 'Content Type',
    ec.content = 'What data QR encodes (URL, WiFi, vCard, Instagram)',
    ec.question = 'WHAT?',
    ec.sort_order = 2,
    ec.llm_context = 'USE: for content formats and data types QR encodes. TRIGGERS: content, format, URL, WiFi, vCard, data type. NOT: features (use FEATURE), tools (use TOOL).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'FEATURE'})
SET ec.display_name = 'Feature',
    ec.content = 'Software capabilities (Analytics, Tracking, Bulk QR)',
    ec.question = 'WHAT?',
    ec.sort_order = 3,
    ec.llm_context = 'USE: for software capabilities and functions. TRIGGERS: feature, capability, analytics, tracking, bulk. NOT: tools (use TOOL), things (use THING).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'TOOL'})
SET ec.display_name = 'Tool',
    ec.content = 'Generators, scanners, builders',
    ec.question = 'WHAT?',
    ec.sort_order = 4,
    ec.llm_context = 'USE: for generators, scanners, and builders. TRIGGERS: tool, generator, scanner, builder, app. NOT: features (use FEATURE), things (use THING).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- WHERE? (Medium/Placement) ---

MERGE (ec:EntityCategory {key: 'MEDIUM'})
SET ec.display_name = 'Medium',
    ec.content = 'Surfaces and placements (posters, cards, packaging)',
    ec.question = 'WHERE?',
    ec.sort_order = 5,
    ec.llm_context = 'USE: for surfaces and placements for QR codes. TRIGGERS: medium, surface, poster, card, packaging, placement. NOT: content types (use CONTENT_TYPE).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- WHY? (Use Cases) ---

MERGE (ec:EntityCategory {key: 'USE_CASE'})
SET ec.display_name = 'Use Case',
    ec.content = 'Application scenarios (marketing, events, file sharing)',
    ec.question = 'WHY?',
    ec.sort_order = 6,
    ec.llm_context = 'USE: for application scenarios and business purposes. TRIGGERS: use case, scenario, marketing, events, purpose. NOT: industries (use INDUSTRY).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- WHO? (Industries/Verticals) ---

MERGE (ec:EntityCategory {key: 'INDUSTRY'})
SET ec.display_name = 'Industry',
    ec.content = 'Vertical markets (restaurants, retail, healthcare)',
    ec.question = 'WHO?',
    ec.sort_order = 7,
    ec.llm_context = 'USE: for vertical markets and business sectors. TRIGGERS: industry, vertical, restaurant, retail, healthcare, sector. NOT: use cases (use USE_CASE).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- HOW? (Actions, Guides, Comparisons) ---

MERGE (ec:EntityCategory {key: 'ACTION'})
SET ec.display_name = 'Action',
    ec.content = 'User verbs (create, scan, track, design)',
    ec.question = 'HOW?',
    ec.sort_order = 8,
    ec.llm_context = 'USE: for verbs and user operations. TRIGGERS: action, create, generate, scan, share, customize, track. NOT: features (use FEATURE), things (use THING).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'GUIDE'})
SET ec.display_name = 'Guide',
    ec.content = 'How-to instructional content',
    ec.question = 'HOW?',
    ec.sort_order = 9,
    ec.llm_context = 'USE: for instructional how-to content. TRIGGERS: guide, how-to, tutorial, instructions, steps. NOT: actions (use ACTION).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'COMPARISON'})
SET ec.display_name = 'Comparison',
    ec.content = 'Versus content (static vs dynamic)',
    ec.question = 'HOW?',
    ec.sort_order = 10,
    ec.llm_context = 'USE: for versus and comparison content. TRIGGERS: comparison, versus, vs, compare, difference. NOT: guides (use GUIDE).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- ABSTRACT (Concepts) ---

MERGE (ec:EntityCategory {key: 'CONCEPT'})
SET ec.display_name = 'Concept',
    ec.content = 'Educational ideas (dynamic, static, quiet zone)',
    ec.question = 'ABSTRACT',
    ec.sort_order = 11,
    ec.llm_context = 'USE: for educational ideas and terminology. TRIGGERS: concept, idea, theory, definition, explanation. NOT: things (use THING).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- EXTERNAL (Brands, Integrations) ---

MERGE (ec:EntityCategory {key: 'BRAND'})
SET ec.display_name = 'Brand',
    ec.content = 'Third-party brands (Google, Instagram, PayPal)',
    ec.question = 'EXTERNAL',
    ec.sort_order = 12,
    ec.llm_context = 'USE: for third-party brand references. TRIGGERS: brand, Google, Instagram, PayPal, company. NOT: integrations (use INTEGRATION).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'INTEGRATION'})
SET ec.display_name = 'Integration',
    ec.content = 'Third-party integrations (Zapier, HubSpot)',
    ec.question = 'EXTERNAL',
    ec.sort_order = 13,
    ec.llm_context = 'USE: for third-party integrations and connections. TRIGGERS: integration, Zapier, HubSpot, API, connect. NOT: brands (use BRAND).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.5-entity-categories.cypher"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();
