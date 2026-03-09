// ═══════════════════════════════════════════════════════════════════════════════
// 066-remove-entitynative-workflow-id.cypher
// Cleanup: Remove workflow_id property from EntityNative nodes
// v0.17.3 - Schema/Data alignment per ADR-024
// ═══════════════════════════════════════════════════════════════════════════════

// Remove workflow_id property from all EntityNative nodes
// This property was used during bootstrap but is not part of the standard schema
MATCH (en:EntityNative)
WHERE en.workflow_id IS NOT NULL
REMOVE en.workflow_id
RETURN count(en) AS nodes_cleaned;
