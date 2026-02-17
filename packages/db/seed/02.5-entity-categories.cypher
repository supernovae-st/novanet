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
    ec.description = 'Core products and objects (QR Code, Smart Link, Barcode)',
    ec.question = 'WHAT?',
    ec.sort_order = 1,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'CONTENT_TYPE'})
SET ec.display_name = 'Content Type',
    ec.description = 'What data QR encodes (URL, WiFi, vCard, Instagram)',
    ec.question = 'WHAT?',
    ec.sort_order = 2,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'FEATURE'})
SET ec.display_name = 'Feature',
    ec.description = 'Software capabilities (Analytics, Tracking, Bulk QR)',
    ec.question = 'WHAT?',
    ec.sort_order = 3,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'TOOL'})
SET ec.display_name = 'Tool',
    ec.description = 'Generators, scanners, builders',
    ec.question = 'WHAT?',
    ec.sort_order = 4,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- WHERE? (Medium/Placement) ---

MERGE (ec:EntityCategory {key: 'MEDIUM'})
SET ec.display_name = 'Medium',
    ec.description = 'Surfaces and placements (posters, cards, packaging)',
    ec.question = 'WHERE?',
    ec.sort_order = 5,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- WHY? (Use Cases) ---

MERGE (ec:EntityCategory {key: 'USE_CASE'})
SET ec.display_name = 'Use Case',
    ec.description = 'Application scenarios (marketing, events, file sharing)',
    ec.question = 'WHY?',
    ec.sort_order = 6,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- WHO? (Industries/Verticals) ---

MERGE (ec:EntityCategory {key: 'INDUSTRY'})
SET ec.display_name = 'Industry',
    ec.description = 'Vertical markets (restaurants, retail, healthcare)',
    ec.question = 'WHO?',
    ec.sort_order = 7,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- HOW? (Actions, Guides, Comparisons) ---

MERGE (ec:EntityCategory {key: 'ACTION'})
SET ec.display_name = 'Action',
    ec.description = 'User verbs (create, scan, track, design)',
    ec.question = 'HOW?',
    ec.sort_order = 8,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'GUIDE'})
SET ec.display_name = 'Guide',
    ec.description = 'How-to instructional content',
    ec.question = 'HOW?',
    ec.sort_order = 9,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'COMPARISON'})
SET ec.display_name = 'Comparison',
    ec.description = 'Versus content (static vs dynamic)',
    ec.question = 'HOW?',
    ec.sort_order = 10,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- ABSTRACT (Concepts) ---

MERGE (ec:EntityCategory {key: 'CONCEPT'})
SET ec.display_name = 'Concept',
    ec.description = 'Educational ideas (dynamic, static, quiet zone)',
    ec.question = 'ABSTRACT',
    ec.sort_order = 11,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- EXTERNAL (Brands, Integrations) ---

MERGE (ec:EntityCategory {key: 'BRAND'})
SET ec.display_name = 'Brand',
    ec.description = 'Third-party brands (Google, Instagram, PayPal)',
    ec.question = 'EXTERNAL',
    ec.sort_order = 12,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

MERGE (ec:EntityCategory {key: 'INTEGRATION'})
SET ec.display_name = 'Integration',
    ec.description = 'Third-party integrations (Zapier, HubSpot)',
    ec.question = 'EXTERNAL',
    ec.sort_order = 13,
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();
