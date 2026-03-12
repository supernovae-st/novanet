// ============================================================================
// Migration 114: Remove parasitic properties (Wave 2)
// ============================================================================
// CSR Audit after reseed revealed additional non-schema properties.
// This migration removes them to ensure data matches schema exactly.
//
// AudienceTrait parasitic properties:
//   - locale: Should use FOR_LOCALE arc, not property
//   - category: NOT in schema
//   - text: NOT in schema
//   - trait_type: NOT in schema
//   - context: NOT in schema (use llm_context)
//   - trait_value: NOT in schema
//   - locale_key: Should use FOR_LOCALE arc, not property
//
// AudienceSet parasitic properties:
//   - locale_key: Should use arc relationships
//
// BlockType parasitic properties:
//   - schema: NOT in schema (use structure)
// ============================================================================

// --- AudienceTrait: Remove locale property ---
MATCH (at:AudienceTrait)
WHERE at.locale IS NOT NULL
REMOVE at.locale;

// --- AudienceTrait: Remove category property ---
MATCH (at:AudienceTrait)
WHERE at.category IS NOT NULL
REMOVE at.category;

// --- AudienceTrait: Remove text property ---
MATCH (at:AudienceTrait)
WHERE at.text IS NOT NULL
REMOVE at.text;

// --- AudienceTrait: Remove trait_type property ---
MATCH (at:AudienceTrait)
WHERE at.trait_type IS NOT NULL
REMOVE at.trait_type;

// --- AudienceTrait: Remove context property ---
MATCH (at:AudienceTrait)
WHERE at.context IS NOT NULL
REMOVE at.context;

// --- AudienceTrait: Remove trait_value property ---
MATCH (at:AudienceTrait)
WHERE at.trait_value IS NOT NULL
REMOVE at.trait_value;

// --- AudienceTrait: Remove locale_key property ---
MATCH (at:AudienceTrait)
WHERE at.locale_key IS NOT NULL
REMOVE at.locale_key;

// --- AudienceSet: Remove locale_key property ---
MATCH (as:AudienceSet)
WHERE as.locale_key IS NOT NULL
REMOVE as.locale_key;

// --- BlockType: Remove schema property ---
MATCH (bt:BlockType)
WHERE bt.schema IS NOT NULL
REMOVE bt.schema;

// --- Summary query (for verification) ---
// Run this after migration to verify cleanup:
// MATCH (at:AudienceTrait) WHERE at.locale IS NOT NULL OR at.category IS NOT NULL OR at.text IS NOT NULL RETURN count(at);
// MATCH (as:AudienceSet) WHERE as.locale_key IS NOT NULL RETURN count(as);
// MATCH (bt:BlockType) WHERE bt.schema IS NOT NULL RETURN count(bt);
// All should return 0.
