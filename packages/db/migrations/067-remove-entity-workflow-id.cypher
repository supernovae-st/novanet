// ═══════════════════════════════════════════════════════════════════════════════
// 067-remove-entity-workflow-id.cypher
// Cleanup: Remove workflow_id property from Entity nodes
// v0.17.3 - Schema/Data alignment per ADR-024
// ═══════════════════════════════════════════════════════════════════════════════

// Remove workflow_id property from all Entity nodes
// This property was used during bootstrap but is not part of the standard schema
MATCH (e:Entity)
WHERE e.workflow_id IS NOT NULL
REMOVE e.workflow_id
RETURN count(e) AS nodes_cleaned;
