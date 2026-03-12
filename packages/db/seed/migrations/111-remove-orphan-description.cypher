// ============================================================================
// Migration 111: Remove orphan description properties (v0.19.0 cleanup)
// ============================================================================
//
// v0.19.0 ADR-044 renames description → content.
//
// Migration 107 handled nodes with description BUT NOT content.
// This migration handles nodes that have BOTH description AND content
// (content was already populated by seeds, description remained from old data).
//
// After this migration, NO nodes should have description property.
//
// ============================================================================

// Remove description from nodes that have BOTH description AND content
// (content takes precedence, description is legacy)
MATCH (n)
WHERE n.description IS NOT NULL AND n.content IS NOT NULL
REMOVE n.description;

// Safety pass: remove any remaining description properties
// (should be 0 after the above, but ensures clean state)
MATCH (n)
WHERE n.description IS NOT NULL
REMOVE n.description;

// Verification query (run manually):
// MATCH (n) WHERE n.description IS NOT NULL RETURN labels(n), count(*) as cnt;
// Expected: 0 results after migration
