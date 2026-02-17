// ═══════════════════════════════════════════════════════════════════════════════
// 51-seokeywords-ahrefs.cypher — SEOKeyword nodes with real Ahrefs data
// v0.13.0 - ADR-032 (URL Slugification), real keyword research
// ═══════════════════════════════════════════════════════════════════════════════
//
// Data source: Ahrefs keyword research (2026-02-16)
// Export: User-provided CSV with volume, difficulty, CPC
//
// Key format: seo-{slug}-{locale}
// ═══════════════════════════════════════════════════════════════════════════════

// =============================================================================
// en-US Keywords (2)
// =============================================================================

// "qr code generator" - 699K volume, #1 in US
MERGE (kw:SEOKeyword {key: 'seo-qr-code-generator-en-US'})
ON CREATE SET
  kw.value = 'qr code generator',
  kw.locale_key = 'en-US',
  kw.volume = 699000,
  kw.difficulty = 89,
  kw.cpc = 0.15,
  kw.intent = 'Transactional,Non-branded,Non-local',
  kw.slug_form = 'qr-code-generator',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.value = 'qr code generator',
  kw.volume = 699000,
  kw.difficulty = 89,
  kw.cpc = 0.15,
  kw.updated_at = datetime();

// "qr code" - 589K volume, #2 in US
MERGE (kw:SEOKeyword {key: 'seo-qr-code-en-US'})
ON CREATE SET
  kw.value = 'qr code',
  kw.locale_key = 'en-US',
  kw.volume = 589000,
  kw.difficulty = 91,
  kw.cpc = 0.20,
  kw.intent = 'Informational,Non-branded,Non-local',
  kw.slug_form = 'qr-code',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.value = 'qr code',
  kw.volume = 589000,
  kw.difficulty = 91,
  kw.cpc = 0.20,
  kw.updated_at = datetime();

// =============================================================================
// fr-FR Keywords (2)
// =============================================================================

// "qr code" - 109K volume, #1 in France
// Note: French speakers use "QR code" (not "code QR")
MERGE (kw:SEOKeyword {key: 'seo-qr-code-fr-FR'})
ON CREATE SET
  kw.value = 'qr code',
  kw.locale_key = 'fr-FR',
  kw.volume = 109000,
  kw.difficulty = 79,
  kw.cpc = 0.05,
  kw.intent = 'Informational,Non-branded,Non-local',
  kw.slug_form = 'qr-code',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.value = 'qr code',
  kw.volume = 109000,
  kw.difficulty = 79,
  kw.cpc = 0.05,
  kw.updated_at = datetime();

// "générateur qr code" - 28K volume, #2 in France
MERGE (kw:SEOKeyword {key: 'seo-generateur-qr-code-fr-FR'})
ON CREATE SET
  kw.value = 'générateur qr code',
  kw.locale_key = 'fr-FR',
  kw.volume = 28000,
  kw.difficulty = 51,
  kw.cpc = 0.04,
  kw.intent = 'Transactional,Non-branded,Non-local',
  kw.slug_form = 'generateur-qr-code',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.value = 'générateur qr code',
  kw.volume = 28000,
  kw.difficulty = 51,
  kw.cpc = 0.04,
  kw.updated_at = datetime();

// =============================================================================
// es-MX Keywords (2)
// =============================================================================

// "código qr" - 44K volume, #1 in Mexico
// ⚠️ CRITICAL: WITH accent (ó) - latin_preserve slugification rule
MERGE (kw:SEOKeyword {key: 'seo-codigo-qr-es-MX'})
ON CREATE SET
  kw.value = 'código qr',
  kw.locale_key = 'es-MX',
  kw.volume = 44000,
  kw.difficulty = 72,
  kw.cpc = 0.02,
  kw.intent = 'Informational,Non-branded,Non-local',
  kw.slug_form = 'código-qr',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.value = 'código qr',
  kw.volume = 44000,
  kw.difficulty = 72,
  kw.cpc = 0.02,
  kw.updated_at = datetime();

// "generador código qr" - 15K volume, #2 in Mexico
MERGE (kw:SEOKeyword {key: 'seo-generador-codigo-qr-es-MX'})
ON CREATE SET
  kw.value = 'generador código qr',
  kw.locale_key = 'es-MX',
  kw.volume = 15000,
  kw.difficulty = 54,
  kw.cpc = 0.02,
  kw.intent = 'Transactional,Non-branded,Non-local',
  kw.slug_form = 'generador-código-qr',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-02-16'),
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.value = 'generador código qr',
  kw.volume = 15000,
  kw.difficulty = 54,
  kw.cpc = 0.02,
  kw.updated_at = datetime();
