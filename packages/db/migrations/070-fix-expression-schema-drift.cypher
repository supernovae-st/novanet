// ============================================================================
// MIGRATION 070: Fix Expression Schema Drift
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Add missing required properties to Expression atoms
//
// Schema requires: key, display_name, text, tone, formality, use_case,
//                  llm_context, created_at, updated_at
// Seed has: key, text, register, intention, semantic_field, context, example
//
// Mapping:
//   - display_name ← intention (or fallback to text)
//   - tone ← derived from register
//   - formality ← register (with enum mapping)
//   - use_case ← derived from semantic_field
//   - created_at/updated_at ← datetime()
// ============================================================================

// Step 1: Add display_name from intention (or text as fallback)
MATCH (e:Expression)
WHERE e.display_name IS NULL
SET e.display_name = COALESCE(e.intention, e.text)
RETURN count(e) AS expressions_display_name_fixed;

// Step 2: Add tone derived from register
// Mapping: formal→formal, semi-formal→formal, casual→casual, else→friendly
MATCH (e:Expression)
WHERE e.tone IS NULL
SET e.tone = CASE
  WHEN e.register = 'formal' THEN 'formal'
  WHEN e.register = 'semi-formal' THEN 'formal'
  WHEN e.register = 'casual' THEN 'casual'
  ELSE 'friendly'
END
RETURN count(e) AS expressions_tone_fixed;

// Step 3: Add formality from register
// Mapping: formal→formal, semi-formal→formal, casual→casual, else→neutral
MATCH (e:Expression)
WHERE e.formality IS NULL
SET e.formality = CASE
  WHEN e.register = 'formal' THEN 'formal'
  WHEN e.register = 'semi-formal' THEN 'formal'
  WHEN e.register = 'casual' THEN 'casual'
  ELSE 'neutral'
END
RETURN count(e) AS expressions_formality_fixed;

// Step 4: Add use_case derived from semantic_field
// Mapping semantic fields to use_case enum values
MATCH (e:Expression)
WHERE e.use_case IS NULL
SET e.use_case = CASE
  WHEN e.semantic_field = 'SUCCESS' THEN 'celebration'
  WHEN e.semantic_field = 'SPEED' THEN 'confirmation'
  WHEN e.semantic_field = 'SIMPLICITY' THEN 'confirmation'
  WHEN e.semantic_field = 'QUALITY' THEN 'confirmation'
  WHEN e.semantic_field = 'URGENCY' THEN 'warning'
  WHEN e.semantic_field = 'TRUST' THEN 'thanks'
  WHEN e.semantic_field = 'INNOVATION' THEN 'celebration'
  WHEN e.semantic_field = 'VALUE' THEN 'confirmation'
  WHEN e.semantic_field = 'EXCLUSIVITY' THEN 'celebration'
  WHEN e.semantic_field = 'COMMUNITY' THEN 'greeting'
  ELSE 'confirmation'
END
RETURN count(e) AS expressions_use_case_fixed;

// Step 5: Add timestamps
MATCH (e:Expression)
WHERE e.created_at IS NULL
SET e.created_at = datetime(),
    e.updated_at = datetime()
RETURN count(e) AS expressions_timestamps_fixed;

// Step 6: Add created_by for provenance tracking
MATCH (e:Expression)
WHERE e.created_by IS NULL
SET e.created_by = 'seed:locale'
RETURN count(e) AS expressions_provenance_fixed;

// Verification query
MATCH (e:Expression)
WHERE e.display_name IS NOT NULL
  AND e.tone IS NOT NULL
  AND e.formality IS NOT NULL
  AND e.use_case IS NOT NULL
  AND e.created_at IS NOT NULL
RETURN count(e) AS expressions_now_valid;
