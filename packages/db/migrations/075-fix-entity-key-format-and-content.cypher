// ============================================================================
// MIGRATION 075: Fix Entity Key Format and Content
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Merge duplicate Entity nodes and ensure correct key format
//
// Current state:
//   - Some entities have key="qr-code" (old format, HAS content)
//   - Some entities have key="entity:qr-code" (new format, NO content)
//   - Schema expects: key="entity:qr-code" WITH content
//
// Fix:
//   1. Copy content from old format to new format
//   2. Delete old format nodes (they're duplicates)
// ============================================================================

// Step 1: Copy content from old-format to new-format entities
MATCH (old:Entity), (new:Entity)
WHERE NOT old.key STARTS WITH 'entity:'
  AND new.key = 'entity:' + old.key
  AND old.content IS NOT NULL
  AND new.content IS NULL
SET new.content = old.content,
    new.llm_context = COALESCE(new.llm_context, old.llm_context),
    new.description = COALESCE(new.description, old.description)
RETURN count(new) AS entities_content_copied;

// Step 2: Copy any other properties from old to new
MATCH (old:Entity), (new:Entity)
WHERE NOT old.key STARTS WITH 'entity:'
  AND new.key = 'entity:' + old.key
SET new.created_at = COALESCE(new.created_at, old.created_at),
    new.updated_at = datetime()
RETURN count(new) AS entities_timestamps_copied;

// Step 3: Delete old-format entities that have new-format duplicates
MATCH (old:Entity)
WHERE NOT old.key STARTS WITH 'entity:'
  AND EXISTS {
    MATCH (new:Entity)
    WHERE new.key = 'entity:' + old.key
  }
DETACH DELETE old
RETURN count(*) AS old_format_entities_deleted;

// Step 4: Add prefix to any remaining old-format entities (no duplicate exists)
MATCH (e:Entity)
WHERE NOT e.key STARTS WITH 'entity:'
SET e.key = 'entity:' + e.key
RETURN count(e) AS remaining_entities_prefixed;

// Verification query
MATCH (e:Entity)
RETURN e.key AS key,
       e.key STARTS WITH 'entity:' AS has_prefix,
       e.content IS NOT NULL AS has_content
ORDER BY e.key;
