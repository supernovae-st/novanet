// ═══════════════════════════════════════════════════════════════════════════════
// 55-denomination-forms-all.cypher — denomination_forms for all remaining EntityNative
// v0.13.1 - ADR-033: Denomination Forms
// ═══════════════════════════════════════════════════════════════════════════════
//
// COVERAGE: All EntityNative nodes not yet seeded in 54-denomination-forms.cypher
// (54 covers: qr-code × en-US, fr-FR, es-MX, de-DE, ja-JP only)
//
// APPROACH: Bulk derivation from display_name (or entity_key fallback).
//   - text:   toLower(display_name) — prose form
//   - title:  display_name — heading form (Title Case)
//   - abbrev: first word of display_name — approximate, needs editorial review
//   - url:    NOT set — populated by SEO pipeline post-derivation (ADR-030)
//
// EDITORIAL NOTE:
//   text and title forms are reasonable starting points.
//   abbrev forms are naive (first word) and MUST be reviewed before publishing.
//   url forms are intentionally omitted — written back by SEO pipeline only.
//
// Requires: 11-entity-content-fr-fr.cypher (EntityNative nodes must exist)
//           54-denomination-forms.cypher (excludes qr-code from this bulk)
// ═══════════════════════════════════════════════════════════════════════════════

// =============================================================================
// Bulk SET: all EntityNative nodes missing denomination_forms
// (excludes qr-code × 5 locales already seeded in seed 54)
// =============================================================================

MATCH (en:EntityNative)
WHERE en.denomination_forms IS NULL
  AND NOT en.key STARTS WITH 'entity:qr-code@'
WITH en,
  coalesce(en.display_name, replace(en.entity_key, '-', ' ')) AS base_name
SET en.denomination_forms = [
  {type: 'text',   value: toLower(base_name), priority: 1},
  {type: 'title',  value: base_name,          priority: 1},
  {type: 'abbrev', value: split(toLower(base_name), ' ')[0], priority: 1}
]
RETURN count(en) AS denomination_forms_set;

// =============================================================================
// Verification
// =============================================================================

MATCH (en:EntityNative)
WHERE en.denomination_forms IS NULL
RETURN count(en) AS still_missing_after_seed;
