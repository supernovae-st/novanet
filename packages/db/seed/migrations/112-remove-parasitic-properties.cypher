// ============================================================================
// Migration 112: Remove parasitic properties from Entity and EntityNative
// ============================================================================
// CSR Audit revealed properties that are NOT in the YAML schema definitions.
// This migration removes them to ensure data matches schema exactly.
//
// Entity parasitic properties:
//   - workflow_id: Was added by 10-entities-bootstrap.cypher (now removed)
//   - entity_category: Should be a BELONGS_TO arc, not a property
//
// EntityNative parasitic properties:
//   - title: Should be display_name (standard property)
//   - entity_type: NOT in schema
//   - slug_terms: NOT in schema
// ============================================================================

// --- Remove workflow_id from Entity nodes ---
MATCH (e:Entity)
WHERE e.workflow_id IS NOT NULL
REMOVE e.workflow_id;

// --- Remove entity_category property from Entity nodes ---
// (The BELONGS_TO arc relationship is the correct way to link to EntityCategory)
MATCH (e:Entity)
WHERE e.entity_category IS NOT NULL
REMOVE e.entity_category;

// --- Remove title from EntityNative nodes ---
// (Should use display_name instead - migrate if needed)
MATCH (en:EntityNative)
WHERE en.title IS NOT NULL
SET en.display_name = coalesce(en.display_name, en.title)
REMOVE en.title;

// --- Remove entity_type from EntityNative nodes ---
MATCH (en:EntityNative)
WHERE en.entity_type IS NOT NULL
REMOVE en.entity_type;

// --- Remove slug_terms from EntityNative nodes ---
MATCH (en:EntityNative)
WHERE en.slug_terms IS NOT NULL
REMOVE en.slug_terms;

// --- Summary query (for verification) ---
// Run this after migration to verify cleanup:
// MATCH (e:Entity) WHERE e.workflow_id IS NOT NULL OR e.entity_category IS NOT NULL RETURN count(e) AS entity_parasitic;
// MATCH (en:EntityNative) WHERE en.title IS NOT NULL OR en.entity_type IS NOT NULL OR en.slug_terms IS NOT NULL RETURN count(en) AS native_parasitic;
// Both should return 0.
