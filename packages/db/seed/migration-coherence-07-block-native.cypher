// =============================================================================
// migration-coherence-07-block-native.cypher
// NovaNet v0.13.1 coherence — BlockNative
// =============================================================================
//
// SCOPE: 9 BlockNative instances across 2 groups
//
// GROUP 1 (4 instances: blk-custom-qr-code-*@fr-FR)
//   Have: key, created_at, updated_at, anchor_slug, generated
//   Missing required: block_key, locale_key, display_name, description,
//                     generated_at, generator_version, status, version
//
// GROUP 2 (5 instances: qr-code:head-seo-meta:1@*)
//   Orphan props: block_type (should be arc/label only), content (rename → generated)
//   Missing required: generated (rename from content), generated_at,
//                     generator_version, anchor_slug, version
//
// IDEMPOTENT: safe to run multiple times (all steps use WHERE guards)
//
// Rollback: All REMOVE operations target orphan props only.
//   content → if needed: SET n.content = n.generated WHERE block_type was head-seo-meta
// =============================================================================

// =============================================================================
// STEP 1 (GROUP 1): Derive missing standard props from composite key
// Pattern: block:{block_key}@{locale_key}
// =============================================================================

MATCH (n:BlockNative)
WHERE n.key STARTS WITH 'block:blk-custom-qr-code'
  AND n.block_key IS NULL
SET
  n.block_key        = split(split(n.key, 'block:')[1], '@')[0],
  n.locale_key       = split(n.key, '@')[1],
  n.display_name     = 'BlockNative: ' + split(split(n.key, 'block:')[1], '@')[0],
  n.description      = 'Generated block content (custom QR code page pipeline)',
  n.generated_at     = n.created_at,
  n.generator_version = 'pipeline-0.13.1',
  n.status           = 'draft',
  n.version          = 1
RETURN count(n) AS group1_standard_props_added
;

// =============================================================================
// STEP 2 (GROUP 2): Rename content → generated (preserve data)
// =============================================================================

MATCH (n:BlockNative)
WHERE n.content IS NOT NULL
  AND n.generated IS NULL
SET n.generated = n.content
RETURN count(n) AS group2_content_renamed_to_generated
;

// =============================================================================
// STEP 3 (GROUP 2): Remove orphan props (block_type, content)
// block_type is NOT a property — it is encoded in the arc :OF_TYPE -> :BlockType
// content is an old property name, now renamed to generated in STEP 2
// =============================================================================

MATCH (n:BlockNative)
WHERE n.block_type IS NOT NULL OR n.content IS NOT NULL
REMOVE n.block_type, n.content
RETURN count(n) AS group2_orphans_removed
;

// =============================================================================
// STEP 4 (GROUP 2): Add missing required props to head-seo-meta instances
// =============================================================================

MATCH (n:BlockNative)
WHERE n.key CONTAINS 'head-seo-meta'
  AND n.generated_at IS NULL
SET
  n.generated_at      = n.created_at,
  n.generator_version = 'manual-seed-0.13.1',
  n.version           = 1
RETURN count(n) AS group2_missing_props_added
;

// =============================================================================
// STEP 5 (GROUP 2): Set anchor_slug for head-seo-meta instances
// anchor_slug = HTML 'id' attribute for the block
// For head-seo-meta blocks, use "head-seo-meta-1" (type-index format)
// =============================================================================

MATCH (n:BlockNative)
WHERE n.key CONTAINS 'head-seo-meta'
  AND n.anchor_slug IS NULL
SET n.anchor_slug = 'head-seo-meta-1'
RETURN count(n) AS group2_anchor_slug_added
;

// =============================================================================
// STEP 6: Verification — should return 0 rows if clean
// =============================================================================

MATCH (n:BlockNative)
WITH n,
  [k IN keys(n) WHERE k IN ['block_type', 'content']] AS remaining_orphans,
  [k IN ['key', 'block_key', 'locale_key', 'display_name', 'description',
         'created_at', 'updated_at', 'generated', 'generated_at',
         'generator_version', 'status', 'anchor_slug', 'version']
   WHERE NOT k IN keys(n)] AS still_missing
WHERE size(remaining_orphans) > 0 OR size(still_missing) > 0
RETURN n.key AS still_dirty, remaining_orphans, still_missing
LIMIT 10
;
