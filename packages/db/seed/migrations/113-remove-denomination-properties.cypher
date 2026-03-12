// ============================================================================
// Migration 113: Remove parasitic denomination_* properties from EntityNative
// ============================================================================
// File 53-entity-natives-bootstrap.cypher used non-schema properties:
//   - denomination_text (should be in denomination_forms JSON array)
//   - denomination_title (should be in denomination_forms JSON array)
//   - denomination_abbrev (should be in denomination_forms JSON array)
//   - denomination_url (should be in denomination_forms JSON array)
//
// These are NOT in the schema. The correct property is:
//   denomination_forms: '[{"type": "text", "value": "..."}, ...]'
//
// File 53 has been deleted. This migration cleans up existing data.
// ============================================================================

// --- Remove denomination_text from EntityNative nodes ---
MATCH (en:EntityNative)
WHERE en.denomination_text IS NOT NULL
REMOVE en.denomination_text;

// --- Remove denomination_title from EntityNative nodes ---
MATCH (en:EntityNative)
WHERE en.denomination_title IS NOT NULL
REMOVE en.denomination_title;

// --- Remove denomination_abbrev from EntityNative nodes ---
MATCH (en:EntityNative)
WHERE en.denomination_abbrev IS NOT NULL
REMOVE en.denomination_abbrev;

// --- Remove denomination_url from EntityNative nodes ---
MATCH (en:EntityNative)
WHERE en.denomination_url IS NOT NULL
REMOVE en.denomination_url;

// --- Summary query (for verification) ---
// Run this after migration to verify cleanup:
// MATCH (en:EntityNative) WHERE en.denomination_text IS NOT NULL OR en.denomination_title IS NOT NULL OR en.denomination_abbrev IS NOT NULL OR en.denomination_url IS NOT NULL RETURN count(en);
// Should return 0.
