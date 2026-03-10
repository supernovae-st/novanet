// ============================================================================
// PLAN B - Migration 091: Link Blocks to BlockTypes
// ============================================================================
// Priority: STRUCTURE (Block-type relationships)
// Fixes: 5 blocks missing OF_TYPE connections
// CSR Impact: Enables block template validation and type-based queries
// ============================================================================

// Link blocks to their types based on naming conventions
// Hero blocks → hero type
MATCH (b:Block)
WHERE b.key CONTAINS 'hero' OR b.key STARTS WITH 'hero-'
MATCH (bt:BlockType {key: 'hero'})
MERGE (b)-[:OF_TYPE]->(bt);

// FAQ blocks → faq type
MATCH (b:Block)
WHERE b.key CONTAINS 'faq' OR b.key STARTS WITH 'faq-'
MATCH (bt:BlockType {key: 'faq'})
MERGE (b)-[:OF_TYPE]->(bt);

// CTA blocks → cta type
MATCH (b:Block)
WHERE b.key CONTAINS 'cta' OR b.key STARTS WITH 'cta-'
MATCH (bt:BlockType {key: 'cta'})
MERGE (b)-[:OF_TYPE]->(bt);

// Pricing blocks → pricing type
MATCH (b:Block)
WHERE b.key CONTAINS 'pricing' OR b.key STARTS WITH 'pricing-'
MATCH (bt:BlockType {key: 'pricing'})
MERGE (b)-[:OF_TYPE]->(bt);

// Features blocks → features type
MATCH (b:Block)
WHERE b.key CONTAINS 'features' OR b.key STARTS WITH 'features-'
MATCH (bt:BlockType {key: 'features'})
MERGE (b)-[:OF_TYPE]->(bt);

// Footer blocks → footer type
MATCH (b:Block)
WHERE b.key CONTAINS 'footer' OR b.key STARTS WITH 'footer-'
MATCH (bt:BlockType {key: 'footer'})
MERGE (b)-[:OF_TYPE]->(bt);

// For blocks that don't match patterns, try to infer from display_name
MATCH (b:Block)
WHERE NOT (b)-[:OF_TYPE]->(:BlockType)
  AND b.display_name IS NOT NULL
WITH b,
     CASE
       WHEN toLower(b.display_name) CONTAINS 'hero' THEN 'hero'
       WHEN toLower(b.display_name) CONTAINS 'faq' THEN 'faq'
       WHEN toLower(b.display_name) CONTAINS 'cta' OR toLower(b.display_name) CONTAINS 'action' THEN 'cta'
       WHEN toLower(b.display_name) CONTAINS 'pricing' THEN 'pricing'
       WHEN toLower(b.display_name) CONTAINS 'feature' THEN 'features'
       WHEN toLower(b.display_name) CONTAINS 'testimonial' THEN 'testimonials'
       WHEN toLower(b.display_name) CONTAINS 'footer' THEN 'footer'
       ELSE NULL
     END AS inferred_type
WHERE inferred_type IS NOT NULL
MATCH (bt:BlockType {key: inferred_type})
MERGE (b)-[:OF_TYPE]->(bt);

// Verify block-type links
MATCH (b:Block)
OPTIONAL MATCH (b)-[:OF_TYPE]->(bt:BlockType)
RETURN b.key AS block,
       bt.key AS block_type,
       CASE WHEN bt IS NOT NULL THEN 'LINKED' ELSE 'ORPHAN' END AS status
ORDER BY b.key;

// Count by block type
MATCH (bt:BlockType)
OPTIONAL MATCH (b:Block)-[:OF_TYPE]->(bt)
RETURN bt.key AS block_type,
       count(b) AS block_count
ORDER BY bt.key;
