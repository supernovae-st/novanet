// ============================================================================
// PLAN B - Migration 088: Create EntityCategory Nodes
// ============================================================================
// Priority: STRUCTURE (Semantic taxonomy for entities)
// Fixes: Missing EntityCategory nodes (0 exist, need 6)
// CSR Impact: Enables entity classification and semantic grouping
// ============================================================================

// Create THING category
MERGE (c:EntityCategory {key: 'THING'})
ON CREATE SET
  c.display_name = 'Thing',
  c.description = 'Tangible or abstract objects that exist as nouns (products, concepts, entities)',
  c.llm_context = 'USE: for nouns representing products, concepts, or entities. TRIGGERS: thing, product, object, concept, item, entity. NOT: actions (use ACTION), content formats (use CONTENT_TYPE).',
  c.question = 'WHAT?',
  c.sort_order = 1,
  c.created_at = datetime(),
  c.updated_at = datetime()
ON MATCH SET
  c.updated_at = datetime();

// Create CONTENT_TYPE category
MERGE (c:EntityCategory {key: 'CONTENT_TYPE'})
ON CREATE SET
  c.display_name = 'Content Type',
  c.description = 'Formats and structures for presenting information (articles, videos, guides)',
  c.llm_context = 'USE: for content formats and media types. TRIGGERS: content, format, article, video, guide, tutorial, post. NOT: features (use FEATURE), tools (use TOOL).',
  c.question = 'WHAT?',
  c.sort_order = 2,
  c.created_at = datetime(),
  c.updated_at = datetime()
ON MATCH SET
  c.updated_at = datetime();

// Create ACTION category
MERGE (c:EntityCategory {key: 'ACTION'})
ON CREATE SET
  c.display_name = 'Action',
  c.description = 'Verbs and operations users can perform (create, scan, customize, share)',
  c.llm_context = 'USE: for verbs and user operations. TRIGGERS: action, create, generate, scan, share, customize, download. NOT: features (use FEATURE), things (use THING).',
  c.question = 'HOW?',
  c.sort_order = 3,
  c.created_at = datetime(),
  c.updated_at = datetime()
ON MATCH SET
  c.updated_at = datetime();

// Create FEATURE category
MERGE (c:EntityCategory {key: 'FEATURE'})
ON CREATE SET
  c.display_name = 'Feature',
  c.description = 'Product capabilities and functionalities (customization, analytics, integrations)',
  c.llm_context = 'USE: for product capabilities and features. TRIGGERS: feature, capability, functionality, option, setting. NOT: actions (use ACTION), benefits (use BENEFIT).',
  c.question = 'WHAT?',
  c.sort_order = 4,
  c.created_at = datetime(),
  c.updated_at = datetime()
ON MATCH SET
  c.updated_at = datetime();

// Create BENEFIT category
MERGE (c:EntityCategory {key: 'BENEFIT'})
ON CREATE SET
  c.display_name = 'Benefit',
  c.description = 'Value propositions and outcomes for users (efficiency, convenience, professional image)',
  c.llm_context = 'USE: for value propositions and user outcomes. TRIGGERS: benefit, advantage, value, outcome, result. NOT: features (use FEATURE), actions (use ACTION).',
  c.question = 'WHY?',
  c.sort_order = 5,
  c.created_at = datetime(),
  c.updated_at = datetime()
ON MATCH SET
  c.updated_at = datetime();

// Create TOOL category
MERGE (c:EntityCategory {key: 'TOOL'})
ON CREATE SET
  c.display_name = 'Tool',
  c.description = 'External tools, platforms, and integrations (scanners, generators, APIs)',
  c.llm_context = 'USE: for external tools and integrations. TRIGGERS: tool, platform, api, integration, service, app. NOT: features (use FEATURE), things (use THING).',
  c.question = 'EXTERNAL',
  c.sort_order = 6,
  c.created_at = datetime(),
  c.updated_at = datetime()
ON MATCH SET
  c.updated_at = datetime();

// Verify categories created
MATCH (c:EntityCategory)
RETURN c.key AS category,
       c.display_name AS name,
       c.question AS question,
       c.sort_order AS sort_order,
       'Created' AS status
ORDER BY c.sort_order;
