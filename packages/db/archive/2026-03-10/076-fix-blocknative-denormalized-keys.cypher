// ============================================================================
// MIGRATION 076: Fix BlockNative Denormalized Keys
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Add missing block_key, locale_key, status to BlockNative nodes
//
// Schema requires: block_key, locale_key, status (denormalized for queries)
// Key format: "block:{page-slug}-{block-type}@{locale}"
//
// Fix:
//   - block_key = substring before @ (using split)
//   - locale_key = substring after @ (using split)
//   - status = 'draft' (default)
// ============================================================================

// Step 1: Add block_key (everything before @)
MATCH (bn:BlockNative)
WHERE bn.block_key IS NULL
  AND bn.key CONTAINS '@'
WITH bn, split(bn.key, '@') AS parts
SET bn.block_key = parts[0]
RETURN count(bn) AS blocknatives_block_key_fixed;

// Step 2: Add locale_key (everything after @)
MATCH (bn:BlockNative)
WHERE bn.locale_key IS NULL
  AND bn.key CONTAINS '@'
WITH bn, split(bn.key, '@') AS parts
SET bn.locale_key = parts[1]
RETURN count(bn) AS blocknatives_locale_key_fixed;

// Step 3: Add status default
MATCH (bn:BlockNative)
WHERE bn.status IS NULL
SET bn.status = 'draft'
RETURN count(bn) AS blocknatives_status_fixed;

// Step 4: Add updated_at timestamp
MATCH (bn:BlockNative)
WHERE bn.updated_at IS NULL
SET bn.updated_at = datetime()
RETURN count(bn) AS blocknatives_timestamp_fixed;

// Verification query
MATCH (bn:BlockNative)
RETURN bn.key AS key,
       bn.block_key IS NOT NULL AS has_block_key,
       bn.locale_key IS NOT NULL AS has_locale_key,
       bn.status IS NOT NULL AS has_status
ORDER BY bn.key;
