// ============================================================================
// MIGRATION 074: Fix Expression.tone Enum Values
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Convert invalid 'neutral' tone to valid 'friendly' enum value
//
// Schema allows: formal, warm, casual, energetic, empathetic, authoritative, friendly
// Found: 70.86% of expressions have 'neutral' which is NOT in the enum
// Fix: Map 'neutral' → 'friendly' (closest semantic match)
// ============================================================================

// Step 1: Fix 'neutral' tone → 'friendly'
MATCH (e:Expression)
WHERE e.tone = 'neutral'
SET e.tone = 'friendly'
RETURN count(e) AS expressions_tone_fixed;

// Step 2: Also fix any other invalid tone values
MATCH (e:Expression)
WHERE e.tone IS NOT NULL
  AND NOT e.tone IN ['formal', 'warm', 'casual', 'energetic', 'empathetic', 'authoritative', 'friendly']
SET e.tone = 'friendly'
RETURN count(e) AS expressions_invalid_tone_fixed;

// Verification query
MATCH (e:Expression)
WHERE e.tone IN ['formal', 'warm', 'casual', 'energetic', 'empathetic', 'authoritative', 'friendly']
RETURN e.tone AS tone, count(e) AS count
ORDER BY count DESC;
