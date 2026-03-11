// ============================================================================
// Migration 107: Rename description → content (v0.19.0 Standard Properties)
// ============================================================================
//
// v0.19.0 introduces standardized properties across all 61 node classes.
// The canonical property order is:
//   key, display_name, node_class, content, llm_context, provenance,
//   created_at, updated_at
//
// This migration renames the legacy `description` property to `content`
// for all nodes that still use the old naming convention.
//
// Affected node types (from database audit):
//   - AudienceSet (205), CultureSet (205), PatternSet (204), TabooSet (204)
//   - Taboo (17), EntityNative (17), Pattern (16)
//   - LanguageBranch (14), BlockType (10), Entity (9), CultureRef (9)
//   - AudienceTrait (5), Culture (3), Formatting (3), ExpressionSet (3)
//   - Slugification (3), Locale (2), ProjectGEOScope (2), ProjectSEOScope (2)
//   - EntityCategory (1)
//
// ============================================================================

// Step 1: Rename description → content for all nodes that have description
// but do NOT have content (to avoid overwriting existing content values)
MATCH (n)
WHERE n.description IS NOT NULL AND n.content IS NULL
SET n.content = n.description
REMOVE n.description;

// Step 2: For nodes that have BOTH description AND content,
// we keep content and remove description
MATCH (n)
WHERE n.description IS NOT NULL AND n.content IS NOT NULL
REMOVE n.description;

// Verification query (run manually):
// MATCH (n) WHERE n.description IS NOT NULL RETURN labels(n), count(*) as cnt;
// Expected: 0 results after migration
