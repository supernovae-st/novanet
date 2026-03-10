// =============================================================================
// Seed: BlockType head-seo-meta
// NovaNet v0.13.1
// =============================================================================
// Creates the head-seo-meta BlockType instance.
// This BlockType MUST be the first block (order=0) of every page.
// Contains: slug, meta_title, meta_description
//
// CRITICAL: SEOKeyword.slug_form = INPUT REFERENCE for slug derivation.
// LLM derives slug using: copy / extract / merge / modify / derive.
// Diacritics and locale rules (ADR-032) apply to the final derived value.
// =============================================================================

// Create the BlockType instance for head-seo-meta
MERGE (bt:BlockType {key: 'head-seo-meta'})
ON CREATE SET
  bt.display_name = 'SEO Metadata',
  bt.description = 'SEO metadata block - MUST be first block (order=0) of every page. Contains URL slug, meta title, meta description.',
  bt.category = 'header',
  bt.structure = 'schemas/head-seo-meta.json',
  bt.created_at = datetime(),
  bt.updated_at = datetime(),
  // JSON Schema for validation
  bt.schema = '{
    "type": "object",
    "properties": {
      "slug": {
        "type": "string",
        "pattern": "^[\\\\p{Ll}\\\\p{N}\\\\-]+$",
        "description": "URL-safe localized slug (UTF-8 allowed). Derived from SEOKeyword.slug_form via copy/extract/merge/modify/derive (ADR-030)."
      },
      "meta_title": {
        "type": "string",
        "maxLength": 60,
        "description": "Page title for search engines (<title> tag)"
      },
      "meta_description": {
        "type": "string",
        "maxLength": 160,
        "description": "Page description for search engines (<meta name=\"description\">)"
      }
    },
    "required": ["slug", "meta_title", "meta_description"]
  }',
  bt.llm_context = 'USE: when generating SEO metadata for a page.
TRIGGERS: slug, URL, meta title, meta description, SEO, head.
NOT: for page content (use other block types), for keywords (use TARGETS).
RELATES: BlockNative (output), Slugification (rules via SLUGIFIED_BY), SEOKeyword (source via DERIVED_SLUG_FROM).

CRITICAL: SEOKeyword.slug_form = INPUT REFERENCE, not final output.
Derivation modes: copy (direct), extract (no-repetition rule, ADR-032),
merge (multiple keywords), modify (add brand/tech), derive (strategic).
Locale rules (ADR-032) apply to final value.
Example: es-MX with latin_preserve → "crear-código-qr" (ó retained)'
ON MATCH SET
  bt.updated_at = datetime()
RETURN 'Created BlockType: head-seo-meta' AS status, bt.key AS key;
