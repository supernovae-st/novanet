// ═══════════════════════════════════════════════════════════════════════════════
// 51-seokeywords-ahrefs.cypher — SEOKeyword nodes with real Ahrefs data
// v0.13.1 - ADR-029 (composite key: seo:{slug}@{locale}), ADR-032 (slugification)
// ═══════════════════════════════════════════════════════════════════════════════
//
// Data source: Ahrefs keyword research (2026-02-16/17)
// Key format: seo:{slug}@{locale}  ← ADR-029 standard
//
// KEYWORDS BY LOCALE:
//   en-US: 2 keywords (699K + 589K SV)
//   fr-FR: 3 keywords (slug-source: créer-un-qr-code 14K/KD31)
//   es-MX: 4 keywords (slug-source: crear-codigo-qr 11K/KD44)
//   de-DE: 1 keyword  (slug-source: qr-code-erstellen 95K/KD52)
//   ja-JP: 1 keyword  (slug-source: qrコード作成 274K/KD35)
// ═══════════════════════════════════════════════════════════════════════════════

// =============================================================================
// en-US — 2 keywords
// =============================================================================

// PRIMARY: "qr code generator" — 699K SV, KD89, Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-generator@en-US'})
ON CREATE SET
  kw.display_name = 'qr code generator (en-US)',
  kw.value = 'qr code generator',
  kw.locale_key = 'en-US',
  kw.volume = 699000,
  kw.difficulty = 89,
  kw.cpc = 0.15,
  kw.intent = 'Transactional',
  kw.slug_form = 'qr-code-generator',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 699000,
  kw.difficulty = 89,
  kw.slug_form = 'qr-code-generator',
  kw.updated_at = datetime();

// SECONDARY: "qr code" — 589K SV, KD91, Informational
MERGE (kw:SEOKeyword {key: 'seo:qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'qr code (en-US)',
  kw.value = 'qr code',
  kw.locale_key = 'en-US',
  kw.volume = 589000,
  kw.difficulty = 91,
  kw.cpc = 0.20,
  kw.intent = 'Informational',
  kw.slug_form = 'qr-code',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 589000,
  kw.difficulty = 91,
  kw.updated_at = datetime();

// =============================================================================
// fr-FR — 3 keywords
// Strategy: "créer un qr code" (14K/KD31) = slug source + primary
//           "générateur qr code" (28K) = secondary → capture dans H1
//           "qr code" (109K) = broad secondary
// =============================================================================

// PRIMARY / SLUG SOURCE: "créer un qr code" — 14K SV, KD31, Transactional
// latin_preserve: accent on "créer" → créer-un-qr-code
MERGE (kw:SEOKeyword {key: 'seo:creer-un-qr-code@fr-FR'})
ON CREATE SET
  kw.display_name = 'créer un qr code (fr-FR)',
  kw.value = 'créer un qr code',
  kw.locale_key = 'fr-FR',
  kw.volume = 14000,
  kw.difficulty = 31,
  kw.cpc = 0.08,
  kw.intent = 'Transactional',
  kw.slug_form = 'créer-un-qr-code',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-17'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 14000,
  kw.difficulty = 31,
  kw.slug_form = 'créer-un-qr-code',
  kw.updated_at = datetime();

// SECONDARY: "générateur qr code" — 28K SV, KD51, Transactional
// High volume → H1, meta_title, body copy
MERGE (kw:SEOKeyword {key: 'seo:generateur-qr-code@fr-FR'})
ON CREATE SET
  kw.display_name = 'générateur qr code (fr-FR)',
  kw.value = 'générateur qr code',
  kw.locale_key = 'fr-FR',
  kw.volume = 28000,
  kw.difficulty = 51,
  kw.cpc = 0.04,
  kw.intent = 'Transactional',
  kw.slug_form = 'generateur-qr-code',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 28000,
  kw.difficulty = 51,
  kw.updated_at = datetime();

// SECONDARY: "qr code" — 109K SV, KD79, Informational (broad)
MERGE (kw:SEOKeyword {key: 'seo:qr-code@fr-FR'})
ON CREATE SET
  kw.display_name = 'qr code (fr-FR)',
  kw.value = 'qr code',
  kw.locale_key = 'fr-FR',
  kw.volume = 109000,
  kw.difficulty = 79,
  kw.cpc = 0.05,
  kw.intent = 'Informational',
  kw.slug_form = 'qr-code',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 109000,
  kw.difficulty = 79,
  kw.updated_at = datetime();

// =============================================================================
// es-MX — 4 keywords
// Strategy: "código qr" (44K) = primary (market dominant signal)
//           "crear codigo qr" (11K/KD44) = SLUG SOURCE (différenciation)
//           "generador de qr" (26K) = secondary (concurrent → capturer en H1)
//           "generador código qr" (15K) = secondary
// =============================================================================

// PRIMARY: "código qr" — 44K SV, KD72, Informational (dominant market signal)
// WITH accent (ó) — latin_preserve rule
MERGE (kw:SEOKeyword {key: 'seo:codigo-qr@es-MX'})
ON CREATE SET
  kw.display_name = 'código qr (es-MX)',
  kw.value = 'código qr',
  kw.locale_key = 'es-MX',
  kw.volume = 44000,
  kw.difficulty = 72,
  kw.cpc = 0.02,
  kw.intent = 'Informational',
  kw.slug_form = 'código-qr',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 44000,
  kw.difficulty = 72,
  kw.updated_at = datetime();

// SECONDARY / SLUG SOURCE: "crear codigo qr" — 11K SV, KD44, Transactional
// Slug = crear-código-qr (latin_preserve: ó retained)
// Strategy: aucun concurrent positionné → différenciation + signal "código" dans tout le cluster
MERGE (kw:SEOKeyword {key: 'seo:crear-codigo-qr@es-MX'})
ON CREATE SET
  kw.display_name = 'crear código qr (es-MX)',
  kw.value = 'crear código qr',
  kw.locale_key = 'es-MX',
  kw.volume = 11000,
  kw.difficulty = 44,
  kw.cpc = 0.02,
  kw.intent = 'Transactional',
  kw.slug_form = 'crear-código-qr',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-17'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 11000,
  kw.difficulty = 44,
  kw.slug_form = 'crear-código-qr',
  kw.updated_at = datetime();

// SECONDARY: "generador de qr" — 26K SV, KD46, Transactional
// Concurrent saturé → capturer dans H1 + body, pas dans le slug
MERGE (kw:SEOKeyword {key: 'seo:generador-de-qr@es-MX'})
ON CREATE SET
  kw.display_name = 'generador de qr (es-MX)',
  kw.value = 'generador de qr',
  kw.locale_key = 'es-MX',
  kw.volume = 26000,
  kw.difficulty = 46,
  kw.cpc = 0.02,
  kw.intent = 'Transactional',
  kw.slug_form = 'generador-de-qr',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-17'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 26000,
  kw.difficulty = 46,
  kw.updated_at = datetime();

// SECONDARY: "generador código qr" — 15K SV, KD54, Transactional
MERGE (kw:SEOKeyword {key: 'seo:generador-codigo-qr@es-MX'})
ON CREATE SET
  kw.display_name = 'generador código qr (es-MX)',
  kw.value = 'generador código qr',
  kw.locale_key = 'es-MX',
  kw.volume = 15000,
  kw.difficulty = 54,
  kw.cpc = 0.02,
  kw.intent = 'Transactional',
  kw.slug_form = 'generador-código-qr',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 15000,
  kw.difficulty = 54,
  kw.updated_at = datetime();

// =============================================================================
// de-DE — 1 keyword
// =============================================================================

// PRIMARY / SLUG SOURCE: "qr code erstellen" — 95K SV, KD52, Transactional
// latin_transform: no umlauts in this keyword → qr-code-erstellen (unchanged)
MERGE (kw:SEOKeyword {key: 'seo:qr-code-erstellen@de-DE'})
ON CREATE SET
  kw.display_name = 'qr code erstellen (de-DE)',
  kw.value = 'qr code erstellen',
  kw.locale_key = 'de-DE',
  kw.volume = 95000,
  kw.difficulty = 52,
  kw.cpc = 0.10,
  kw.intent = 'Transactional',
  kw.slug_form = 'qr-code-erstellen',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-17'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 95000,
  kw.difficulty = 52,
  kw.slug_form = 'qr-code-erstellen',
  kw.updated_at = datetime();

// =============================================================================
// ja-JP — 1 keyword
// =============================================================================

// PRIMARY / SLUG SOURCE: "qrコード作成" — 274K SV, KD35, Transactional
// native_script rule: Japanese characters preserved in slug
// Key uses romanized form (sakusei = 作成) for ASCII-safe key
MERGE (kw:SEOKeyword {key: 'seo:qr-code-sakusei@ja-JP'})
ON CREATE SET
  kw.display_name = 'qrコード作成 (ja-JP)',
  kw.value = 'qrコード作成',
  kw.locale_key = 'ja-JP',
  kw.volume = 274000,
  kw.difficulty = 35,
  kw.cpc = 0.05,
  kw.intent = 'Transactional',
  kw.slug_form = 'qrコード作成',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-17'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 274000,
  kw.difficulty = 35,
  kw.slug_form = 'qrコード作成',
  kw.updated_at = datetime();

// =============================================================================
// Log
// =============================================================================
RETURN 'SEOKeyword nodes created/updated: 11 keywords across 5 locales' AS status;
