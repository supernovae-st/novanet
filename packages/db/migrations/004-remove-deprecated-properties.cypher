// Migration 004: Remove deprecated properties (v8.2.0)
// YAML v7.11.0 alignment - removes icon, priority, freshness from all nodes
//
// Run with: cypher-shell -u neo4j -p novanetpassword < migrations/004-remove-deprecated-properties.cypher

// Count affected nodes before migration
MATCH (n)
WHERE n.icon IS NOT NULL OR n.priority IS NOT NULL OR n.freshness IS NOT NULL
RETURN labels(n)[0] AS label, count(*) AS count
ORDER BY count DESC;

// Remove properties from ALL nodes
MATCH (n)
WHERE n.icon IS NOT NULL OR n.priority IS NOT NULL OR n.freshness IS NOT NULL
REMOVE n.icon, n.priority, n.freshness
RETURN count(n) AS nodes_updated;

// Verify no remaining deprecated properties
MATCH (n)
WHERE n.icon IS NOT NULL OR n.priority IS NOT NULL OR n.freshness IS NOT NULL
RETURN count(n) AS remaining;
// Expected: 0
