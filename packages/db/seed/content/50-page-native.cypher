// ═══════════════════════════════════════════════════════════════════════════════
// 50-page-native.cypher — PageNative nodes for qr-code page
// v0.13.1 - ADR-029 (*Native Pattern), ADR-030 (Slug Ownership)
// ═══════════════════════════════════════════════════════════════════════════════
//
// PageNative = locale-specific assembled output for a Page (generated trait)
//
// NOTE: URL routing (slug, full_path) is NOW owned by BlockNative:head-seo-meta
// (the first block of every page). See ADR-030 and seed 49-blocknative-head-seo-meta.cypher.
//
// PageNative contains:
//   - assembled: JSON of compiled blocks
//   - assembled_at: timestamp
//   - assembler_version: version tracking
//   - status: draft/published/archived
//   - version: version number (1, 2, 3...)
//
// Data source: Ahrefs keyword research (2026-02-16)
// ═══════════════════════════════════════════════════════════════════════════════

// -----------------------------------------------------------------------------
// en-US: PageNative for qr-code page
// Slug lives in BlockNative:head-seo-meta (see 49-blocknative-head-seo-meta.cypher)
// -----------------------------------------------------------------------------
MERGE (pn:PageNative {key: 'page:qr-code@en-US'})
ON CREATE SET
  pn.page_key = 'qr-code',
  pn.locale_key = 'en-US',
  pn.display_name = 'QR Code Generator (en-US)',
  pn.description = 'Assembled qr-code page for US market',
  pn.llm_context = 'USE: for assembled page display. TRIGGERS: page native, en-US. NOT: for structure (use Page).',
  pn.assembled = '{"blocks": ["head-seo-meta", "hero", "features", "cta"]}',
  pn.assembled_at = datetime(),
  pn.assembler_version = '1.0.0',
  pn.status = 'published',
  pn.version = 1,
  pn.published_at = datetime(),
  pn.created_at = datetime(),
  pn.updated_at = datetime()
ON MATCH SET
  pn.updated_at = datetime();

// -----------------------------------------------------------------------------
// fr-FR: PageNative for qr-code page
// Slug lives in BlockNative:head-seo-meta (see 49-blocknative-head-seo-meta.cypher)
// -----------------------------------------------------------------------------
MERGE (pn:PageNative {key: 'page:qr-code@fr-FR'})
ON CREATE SET
  pn.page_key = 'qr-code',
  pn.locale_key = 'fr-FR',
  pn.display_name = 'Générateur de QR Code (fr-FR)',
  pn.description = 'Page qr-code assemblée pour le marché français',
  pn.llm_context = 'USE: for assembled page display. TRIGGERS: page native, fr-FR. NOT: for structure (use Page).',
  pn.assembled = '{"blocks": ["head-seo-meta", "hero", "features", "cta"]}',
  pn.assembled_at = datetime(),
  pn.assembler_version = '1.0.0',
  pn.status = 'published',
  pn.version = 1,
  pn.published_at = datetime(),
  pn.created_at = datetime(),
  pn.updated_at = datetime()
ON MATCH SET
  pn.updated_at = datetime();

// -----------------------------------------------------------------------------
// es-MX: PageNative for qr-code page
// Slug lives in BlockNative:head-seo-meta (see 49-blocknative-head-seo-meta.cypher)
// CRITICAL: Slug with diacritics "código-qr" is in head-seo-meta block
// -----------------------------------------------------------------------------
MERGE (pn:PageNative {key: 'page:qr-code@es-MX'})
ON CREATE SET
  pn.page_key = 'qr-code',
  pn.locale_key = 'es-MX',
  pn.display_name = 'Generador de Código QR (es-MX)',
  pn.description = 'Página qr-code ensamblada para el mercado mexicano',
  pn.llm_context = 'USE: for assembled page display. TRIGGERS: page native, es-MX. NOT: for structure (use Page).',
  pn.assembled = '{"blocks": ["head-seo-meta", "hero", "features", "cta"]}',
  pn.assembled_at = datetime(),
  pn.assembler_version = '1.0.0',
  pn.status = 'published',
  pn.version = 1,
  pn.published_at = datetime(),
  pn.created_at = datetime(),
  pn.updated_at = datetime()
ON MATCH SET
  pn.updated_at = datetime();

// -----------------------------------------------------------------------------
// ja-JP: PageNative for qr-code page
// Slug lives in BlockNative:head-seo-meta (see 49-blocknative-head-seo-meta.cypher)
// native_script rule: Japanese characters allowed in slug
// -----------------------------------------------------------------------------
MERGE (pn:PageNative {key: 'page:qr-code@ja-JP'})
ON CREATE SET
  pn.page_key = 'qr-code',
  pn.locale_key = 'ja-JP',
  pn.display_name = 'QRコード作成 (ja-JP)',
  pn.description = '日本市場向けのqr-codeページ',
  pn.llm_context = 'USE: for assembled page display. TRIGGERS: page native, ja-JP. NOT: for structure (use Page).',
  pn.assembled = '{"blocks": ["head-seo-meta", "hero", "features", "cta"]}',
  pn.assembled_at = datetime(),
  pn.assembler_version = '1.0.0',
  pn.status = 'published',
  pn.version = 1,
  pn.published_at = datetime(),
  pn.created_at = datetime(),
  pn.updated_at = datetime()
ON MATCH SET
  pn.updated_at = datetime();

// -----------------------------------------------------------------------------
// de-DE: PageNative for qr-code page
// Slug lives in BlockNative:head-seo-meta (see 49-blocknative-head-seo-meta.cypher)
// latin_transform rule: ü→ue, ö→oe, ä→ae, ß→ss
// -----------------------------------------------------------------------------
MERGE (pn:PageNative {key: 'page:qr-code@de-DE'})
ON CREATE SET
  pn.page_key = 'qr-code',
  pn.locale_key = 'de-DE',
  pn.display_name = 'QR-Code Generator (de-DE)',
  pn.description = 'QR-Code-Seite für den deutschen Markt',
  pn.llm_context = 'USE: for assembled page display. TRIGGERS: page native, de-DE. NOT: for structure (use Page).',
  pn.assembled = '{"blocks": ["head-seo-meta", "hero", "features", "cta"]}',
  pn.assembled_at = datetime(),
  pn.assembler_version = '1.0.0',
  pn.status = 'published',
  pn.version = 1,
  pn.published_at = datetime(),
  pn.created_at = datetime(),
  pn.updated_at = datetime()
ON MATCH SET
  pn.updated_at = datetime();

// =============================================================================
// Link PageNative to Locale via FOR_LOCALE
// =============================================================================
MATCH (pn:PageNative {key: 'page:qr-code@en-US'})
MATCH (l:Locale {key: 'en-US'})
MERGE (pn)-[:FOR_LOCALE]->(l);

MATCH (pn:PageNative {key: 'page:qr-code@fr-FR'})
MATCH (l:Locale {key: 'fr-FR'})
MERGE (pn)-[:FOR_LOCALE]->(l);

MATCH (pn:PageNative {key: 'page:qr-code@es-MX'})
MATCH (l:Locale {key: 'es-MX'})
MERGE (pn)-[:FOR_LOCALE]->(l);

MATCH (pn:PageNative {key: 'page:qr-code@ja-JP'})
MATCH (l:Locale {key: 'ja-JP'})
MERGE (pn)-[:FOR_LOCALE]->(l);

MATCH (pn:PageNative {key: 'page:qr-code@de-DE'})
MATCH (l:Locale {key: 'de-DE'})
MERGE (pn)-[:FOR_LOCALE]->(l);

// =============================================================================
// Link Page to PageNative via HAS_NATIVE (ADR-029)
// =============================================================================
MATCH (p:Page {key: 'page:qr-code'})
MATCH (pn:PageNative {key: 'page:qr-code@en-US'})
MERGE (p)-[:HAS_NATIVE {locale: 'en-US'}]->(pn);

MATCH (p:Page {key: 'page:qr-code'})
MATCH (pn:PageNative {key: 'page:qr-code@fr-FR'})
MERGE (p)-[:HAS_NATIVE {locale: 'fr-FR'}]->(pn);

MATCH (p:Page {key: 'page:qr-code'})
MATCH (pn:PageNative {key: 'page:qr-code@es-MX'})
MERGE (p)-[:HAS_NATIVE {locale: 'es-MX'}]->(pn);

MATCH (p:Page {key: 'page:qr-code'})
MATCH (pn:PageNative {key: 'page:qr-code@ja-JP'})
MERGE (p)-[:HAS_NATIVE {locale: 'ja-JP'}]->(pn);

MATCH (p:Page {key: 'page:qr-code'})
MATCH (pn:PageNative {key: 'page:qr-code@de-DE'})
MERGE (p)-[:HAS_NATIVE {locale: 'de-DE'}]->(pn);

// =============================================================================
// Link PageNative to BlockNative (ASSEMBLES relationship)
// The head-seo-meta block contains the slug - see 49-blocknative-head-seo-meta.cypher
// =============================================================================
MATCH (pn:PageNative {key: 'page:qr-code@en-US'})
MATCH (bn:BlockNative {key: 'block:qr-code:head-seo-meta:1@en-US'})
MERGE (pn)-[:ASSEMBLES {order: 0}]->(bn);

MATCH (pn:PageNative {key: 'page:qr-code@fr-FR'})
MATCH (bn:BlockNative {key: 'block:qr-code:head-seo-meta:1@fr-FR'})
MERGE (pn)-[:ASSEMBLES {order: 0}]->(bn);

MATCH (pn:PageNative {key: 'page:qr-code@es-MX'})
MATCH (bn:BlockNative {key: 'block:qr-code:head-seo-meta:1@es-MX'})
MERGE (pn)-[:ASSEMBLES {order: 0}]->(bn);

MATCH (pn:PageNative {key: 'page:qr-code@ja-JP'})
MATCH (bn:BlockNative {key: 'block:qr-code:head-seo-meta:1@ja-JP'})
MERGE (pn)-[:ASSEMBLES {order: 0}]->(bn);

MATCH (pn:PageNative {key: 'page:qr-code@de-DE'})
MATCH (bn:BlockNative {key: 'block:qr-code:head-seo-meta:1@de-DE'})
MERGE (pn)-[:ASSEMBLES {order: 0}]->(bn);

// =============================================================================
// Log creation
// =============================================================================
RETURN 'Created PageNative nodes with ASSEMBLES links to head-seo-meta blocks' AS status;
