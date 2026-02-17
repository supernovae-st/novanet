// Migration 001: Add missing timestamps to Slugification nodes
// Problem: 22-slugification.cypher did not include created_at/updated_at
// Run: AFTER 22-slugification.cypher has been seeded
// Idempotent: uses coalesce to preserve existing timestamps

MATCH (s:Slugification)
WHERE s.created_at IS NULL OR s.updated_at IS NULL
SET s.created_at = coalesce(s.created_at, datetime()),
    s.updated_at = datetime()
RETURN count(s) AS fixed_nodes;
