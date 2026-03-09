// ═══════════════════════════════════════════════════════════════════════════════
// 068-migrate-description-to-content.cypher
// Schema Change: description (string) → content (object), llm_context string → object
// v0.17.3 - Structured Entity Schema (ADR-037)
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// PHASE 1: Entity nodes - Convert description string to content object
// ─────────────────────────────────────────────────────────────────────────────────

// Step 1a: Create content object from existing description string
MATCH (e:Entity)
WHERE e.description IS NOT NULL AND e.content IS NULL
SET e.content = {
  definition: e.description,
  context: "Role and usage in QR Code AI platform (to be defined)"
}
RETURN count(e) AS entity_content_migrated;

// Step 1b: Parse and convert llm_context string to object
// Note: This creates a basic structure - manual curation may be needed
MATCH (e:Entity)
WHERE e.llm_context IS NOT NULL AND (e.llm_context STARTS WITH "USE:" OR e.llm_context CONTAINS "TRIGGERS:")
WITH e, e.llm_context AS ctx
SET e.llm_context = {
  use: CASE
    WHEN ctx CONTAINS "USE:" THEN trim(split(split(ctx, "USE:")[1], "TRIGGERS:")[0])
    ELSE "To be defined"
  END,
  triggers: [],
  not_for: []
}
RETURN count(e) AS entity_llm_context_migrated;

// Step 1c: Remove old description property after migration
MATCH (e:Entity)
WHERE e.description IS NOT NULL AND e.content IS NOT NULL
REMOVE e.description
RETURN count(e) AS entity_description_removed;

// ─────────────────────────────────────────────────────────────────────────────────
// PHASE 2: EntityNative nodes - Same conversion
// ─────────────────────────────────────────────────────────────────────────────────

// Step 2a: Create content object from existing description string
MATCH (en:EntityNative)
WHERE en.description IS NOT NULL AND en.content IS NULL
SET en.content = {
  definition: en.description,
  context: "Role and usage in QR Code AI platform (localized - to be defined)"
}
RETURN count(en) AS entity_native_content_migrated;

// Step 2b: Parse and convert llm_context string to object
MATCH (en:EntityNative)
WHERE en.llm_context IS NOT NULL AND (en.llm_context STARTS WITH "USE:" OR en.llm_context CONTAINS "TRIGGERS:")
WITH en, en.llm_context AS ctx
SET en.llm_context = {
  use: CASE
    WHEN ctx CONTAINS "USE:" THEN trim(split(split(ctx, "USE:")[1], "TRIGGERS:")[0])
    ELSE "To be defined"
  END,
  triggers: [],
  not_for: []
}
RETURN count(en) AS entity_native_llm_context_migrated;

// Step 2c: Remove old description property after migration
MATCH (en:EntityNative)
WHERE en.description IS NOT NULL AND en.content IS NOT NULL
REMOVE en.description
RETURN count(en) AS entity_native_description_removed;

// ─────────────────────────────────────────────────────────────────────────────────
// VERIFICATION: Check migration status
// ─────────────────────────────────────────────────────────────────────────────────

// Count nodes with old vs new structure
MATCH (e:Entity)
RETURN
  count(CASE WHEN e.description IS NOT NULL THEN 1 END) AS entities_with_old_description,
  count(CASE WHEN e.content IS NOT NULL THEN 1 END) AS entities_with_new_content;

MATCH (en:EntityNative)
RETURN
  count(CASE WHEN en.description IS NOT NULL THEN 1 END) AS entity_natives_with_old_description,
  count(CASE WHEN en.content IS NOT NULL THEN 1 END) AS entity_natives_with_new_content;
