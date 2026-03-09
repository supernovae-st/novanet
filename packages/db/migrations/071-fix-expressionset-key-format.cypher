// ============================================================================
// MIGRATION 071: Fix ExpressionSet Key Format
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Fix ExpressionSet key format to match schema convention
//
// Current: key = "ceb-PH" (just locale code)
// Expected: key = "expression-set@ceb-PH" (with class prefix)
// ============================================================================

// Step 1: Update ExpressionSet keys to include class prefix
MATCH (es:ExpressionSet)
WHERE NOT es.key STARTS WITH 'expression-set@'
SET es.key = 'expression-set@' + es.key
RETURN count(es) AS expressionsets_key_fixed;

// Step 2: Add missing created_at timestamps
MATCH (es:ExpressionSet)
WHERE es.created_at IS NULL
SET es.created_at = datetime()
RETURN count(es) AS expressionsets_created_at_fixed;

// Verification query
MATCH (es:ExpressionSet)
WHERE es.key STARTS WITH 'expression-set@'
RETURN count(es) AS expressionsets_now_valid;
