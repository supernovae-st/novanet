// ============================================================================
// Migration 100: Fix Remaining Data Quality Gaps
// ============================================================================
// Fixes discovered in verification 099:
// 1. Schema nodes missing updated_at (ArcScope, Cardinality nodes)
// 2. Missing BlockTypes (grid, content, seo-meta)
// 3. Blocks missing OF_TYPE links
// 4. Expression locale property (derived from FOR_LOCALE arc)
// ============================================================================

// 1. Fix Schema nodes missing updated_at
MATCH (n:Schema)
WHERE n.updated_at IS NULL
SET n.updated_at = datetime();

// 2. Create missing BlockType nodes
MERGE (bt:BlockType {key: 'grid'})
ON CREATE SET
  bt.display_name = 'Grid',
  bt.description = 'Grid layout block for displaying multiple items in a grid format',
  bt.llm_context = 'USE: for grid layouts showing multiple items. TRIGGERS: grid, cards, gallery, showcase. NOT: for single content blocks.',
  bt.created_at = datetime(),
  bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'content'})
ON CREATE SET
  bt.display_name = 'Content',
  bt.description = 'General content block for text, explanations, and informational content',
  bt.llm_context = 'USE: for body content, explanations, descriptions. TRIGGERS: content, text, body, explanation. NOT: for headers, CTAs, or structured blocks.',
  bt.created_at = datetime(),
  bt.updated_at = datetime();

MERGE (bt:BlockType {key: 'seo-meta'})
ON CREATE SET
  bt.display_name = 'SEO Meta',
  bt.description = 'SEO metadata block for page title, description, and structured data',
  bt.llm_context = 'USE: for SEO metadata, meta tags, structured data. TRIGGERS: meta, seo, title, description, schema. NOT: for visible content.',
  bt.created_at = datetime(),
  bt.updated_at = datetime();

// 3. Link orphan blocks to their BlockTypes
MATCH (b:Block {key: 'block:qr-code-use-cases'})
MATCH (bt:BlockType {key: 'grid'})
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (b:Block {key: 'block:qr-code-what-is'})
MATCH (bt:BlockType {key: 'content'})
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (b:Block {key: 'block:qr-code:head-seo-meta:1'})
SET b.block_type = 'seo-meta'
WITH b
MATCH (bt:BlockType {key: 'seo-meta'})
MERGE (b)-[:OF_TYPE]->(bt);

// 4. Add locale property to Expressions (derived from FOR_LOCALE arc)
MATCH (e:Expression)-[:FOR_LOCALE]->(l:Locale)
WHERE e.locale IS NULL
SET e.locale = l.key;

// 5. For expressions without FOR_LOCALE arc, derive from ExpressionSet
MATCH (es:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e:Expression)
WHERE e.locale IS NULL
MATCH (l:Locale)-[:HAS_EXPRESSIONS]->(es)
SET e.locale = l.key;

// Verify Block-Type Links
MATCH (b:Block)
OPTIONAL MATCH (b)-[:OF_TYPE]->(bt:BlockType)
RETURN b.key AS block_key,
       bt.key AS block_type,
       CASE WHEN bt IS NOT NULL THEN 'LINKED' ELSE 'ORPHAN' END AS status
ORDER BY status DESC, b.key;

// Verify Schema timestamps
MATCH (n)
WHERE n:Schema OR n:Realm OR n:Layer OR n:Trait OR n:ArcFamily
WITH count(*) AS total,
     count(n.updated_at) AS with_timestamp
RETURN 'Schema Timestamps' AS check,
       with_timestamp AS complete,
       total AS total,
       CASE WHEN with_timestamp = total THEN 'PASS' ELSE 'FAIL' END AS status;

// Verify Expression locale property
MATCH (e:Expression)
WITH count(*) AS total,
     count(e.locale) AS with_locale
RETURN 'Expression Locale Property' AS check,
       with_locale AS complete,
       total AS total,
       round(100.0 * with_locale / total, 1) AS coverage_pct,
       CASE WHEN with_locale >= total * 0.95 THEN 'PASS' ELSE 'PARTIAL' END AS status;
