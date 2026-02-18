// ═══════════════════════════════════════════════════════════════════════════════
// 54-denomination-forms.cypher — denomination_forms on Entity + EntityNative
// v0.13.1 - Design doc: docs/plans/2026-02-18-entity-denomination-forms-design.md
// ═══════════════════════════════════════════════════════════════════════════════
//
// denomination_forms: prescriptive canonical forms for entity naming.
// ABSOLUTE RULE: LLM MUST use only these forms — no invention or paraphrase.
//
// Form types:
//   text   → prose and body content
//   title  → H1, H2, meta_title
//   abbrev → after first mention, short text
//   mixed  → native_script locales: tech/brand hybrid (e.g. "QR码")
//   base   → native_script locales: international reference form
//   url    → post-SEO pipeline (written back after slug derivation, ADR-030)
//            MUST follow locale slugification rule (ADR-032)
//
// Storage: Neo4j array of maps [{type, value} or {type, value, priority}]
//
// Requires: 10-entities-qrcode-ai.cypher (Entity nodes)
//           11-entity-content-fr-fr.cypher + similar (EntityNative nodes)
// ═══════════════════════════════════════════════════════════════════════════════

// =============================================================================
// Entity:qr-code (invariant EN) — text + title + abbrev only (no url, ADR-030)
// =============================================================================

MATCH (e:Entity {key: 'qr-code'})
SET e.denomination_forms = [
  {type: 'text',   value: 'qr code'},
  {type: 'title',  value: 'QR Code'},
  {type: 'abbrev', value: 'qr'}
];

// =============================================================================
// EntityNative:qr-code@en-US — latin_strip rule (url: remove diacritics)
// url = "qr-code-generator" (from slug source keyword, ADR-030)
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@en-US'})
SET en.denomination_forms = [
  {type: 'text',   value: 'qr code',           priority: 1},
  {type: 'title',  value: 'QR Code',            priority: 1},
  {type: 'abbrev', value: 'qr',                 priority: 1},
  {type: 'url',    value: 'qr-code-generator',  priority: 1}
];

// =============================================================================
// EntityNative:qr-code@fr-FR — latin_preserve rule (url: keep diacritics)
// url = "créer-un-qr-code" (from slug source keyword, ADR-030)
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
SET en.denomination_forms = [
  {type: 'text',   value: 'qr code',           priority: 1},
  {type: 'title',  value: 'QR Code',            priority: 1},
  {type: 'abbrev', value: 'qr',                 priority: 1},
  {type: 'url',    value: 'créer-un-qr-code',   priority: 1}
];

// =============================================================================
// EntityNative:qr-code@es-MX — latin_preserve rule (url: keep diacritics)
// url = "crear-código-qr" (from secondary slug source keyword, ADR-030)
// Strategy: secondary keyword used (no competitor positioned on "crear")
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@es-MX'})
SET en.denomination_forms = [
  {type: 'text',   value: 'código qr',          priority: 1},
  {type: 'title',  value: 'Código QR',           priority: 1},
  {type: 'abbrev', value: 'qr',                  priority: 1},
  {type: 'url',    value: 'crear-código-qr',     priority: 1}
];

// =============================================================================
// EntityNative:qr-code@de-DE — latin_transform rule (ü→ue, ö→oe, ä→ae, ß→ss)
// url = "qr-code-erstellen" (no umlauts in this keyword, unchanged)
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@de-DE'})
SET en.denomination_forms = [
  {type: 'text',   value: 'qr code',             priority: 1},
  {type: 'title',  value: 'QR Code',              priority: 1},
  {type: 'abbrev', value: 'qr',                   priority: 1},
  {type: 'url',    value: 'qr-code-erstellen',    priority: 1}
];

// =============================================================================
// EntityNative:qr-code@ja-JP — native_script rule
// text/title: Japanese characters allowed
// url: ALWAYS romanized ASCII (native_script rule for url type, ADR-032)
// url = "qr-code-sakusei" (romanized form of qrコード作成)
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@ja-JP'})
SET en.denomination_forms = [
  {type: 'text',   value: 'QRコード',             priority: 1},
  {type: 'title',  value: 'QRコード作成',          priority: 1},
  {type: 'abbrev', value: 'QR',                   priority: 1},
  {type: 'base',   value: 'QR Code',              priority: 2},
  {type: 'url',    value: 'qr-code-sakusei',       priority: 1}
];

// =============================================================================
// Log
// =============================================================================
RETURN 'denomination_forms set: 1 Entity + 5 EntityNative nodes (en-US, fr-FR, es-MX, de-DE, ja-JP)' AS status;
