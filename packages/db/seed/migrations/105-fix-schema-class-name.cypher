// ============================================================================
// Migration 105: Fix Schema:Class name property
// ============================================================================
// The novanet_introspect tool expects `name` but Schema nodes only have `label`
// This sets name = label for all Schema:Class nodes
// ============================================================================

MATCH (c:Schema:Class)
WHERE c.name IS NULL AND c.label IS NOT NULL
SET c.name = c.label;

// Verify fix
MATCH (c:Schema:Class)
WHERE c.name IS NULL
RETURN count(c) AS remaining_null_names;
