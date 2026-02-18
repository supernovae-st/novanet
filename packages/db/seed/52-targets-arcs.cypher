// ═══════════════════════════════════════════════════════════════════════════════
// 52-targets-arcs.cypher — TARGETS arcs: EntityNative -> SEOKeyword
// v0.13.1 - ADR-029 (composite keys), ADR-030 Option B (is_slug_source on TARGETS)
// ═══════════════════════════════════════════════════════════════════════════════
//
// TARGETS arc: EntityNative -> SEOKeyword
// Properties:
//   - rank: 'primary'   = main market intent signal (informs LLM context weighting)
//   - rank: 'secondary' = supporting keywords (captured in H1, body, meta)
//   - is_slug_source: true = THIS keyword's slug_form was used for the URL slug
//
// RULE: primary ≠ necessarily slug source. primary = dominant market intent.
//       The slug source may be secondary if it's a differentiation strategy.
//
// ADR-030 Option B: is_slug_source lives on TARGETS, NOT on DERIVED_SLUG_FROM.
// DERIVED_SLUG_FROM now points BlockNative -> EntityNative (not SEOKeyword).
//
// Requires: 51-seokeywords-ahrefs.cypher (SEOKeyword nodes)
// Requires: EntityNative nodes (from org entity seed)
// ═══════════════════════════════════════════════════════════════════════════════

// =============================================================================
// en-US: entity:qr-code@en-US -> SEOKeywords
// primary: "qr code generator" (699K) ← slug source (primary = slug source here)
// secondary: "qr code" (589K)
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@en-US'})
MATCH (kw:SEOKeyword {key: 'seo:qr-code-generator@en-US'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'primary', t.is_slug_source = true, t.created_at = datetime()
ON MATCH SET t.rank = 'primary', t.is_slug_source = true;

MATCH (en:EntityNative {key: 'entity:qr-code@en-US'})
MATCH (kw:SEOKeyword {key: 'seo:qr-code@en-US'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'secondary', t.is_slug_source = false, t.created_at = datetime()
ON MATCH SET t.rank = 'secondary', t.is_slug_source = false;

// =============================================================================
// fr-FR: entity:qr-code@fr-FR -> SEOKeywords
// primary: "créer un qr code" (14K/KD31) ← lower KD, slug source, our target
// secondary: "générateur qr code" (28K) ← high volume → H1 + body
// secondary: "qr code" (109K) ← broad signal
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
MATCH (kw:SEOKeyword {key: 'seo:creer-un-qr-code@fr-FR'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'primary', t.is_slug_source = true, t.created_at = datetime()
ON MATCH SET t.rank = 'primary', t.is_slug_source = true;

MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
MATCH (kw:SEOKeyword {key: 'seo:generateur-qr-code@fr-FR'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'secondary', t.is_slug_source = false, t.created_at = datetime()
ON MATCH SET t.rank = 'secondary', t.is_slug_source = false;

MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
MATCH (kw:SEOKeyword {key: 'seo:qr-code@fr-FR'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'secondary', t.is_slug_source = false, t.created_at = datetime()
ON MATCH SET t.rank = 'secondary', t.is_slug_source = false;

// =============================================================================
// es-MX: entity:qr-code@es-MX -> SEOKeywords
// primary: "código qr" (44K) ← dominant market signal (43% of market)
// secondary: "crear codigo qr" (11K/KD44) ← SLUG SOURCE, differentiation
//   (is_slug_source on secondary — no competitor positioned on "crear")
// secondary: "generador de qr" (26K) ← competitor keyword → capture in H1
// secondary: "generador código qr" (15K) ← reinforcing secondary
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@es-MX'})
MATCH (kw:SEOKeyword {key: 'seo:codigo-qr@es-MX'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'primary', t.is_slug_source = false, t.created_at = datetime()
ON MATCH SET t.rank = 'primary', t.is_slug_source = false;

MATCH (en:EntityNative {key: 'entity:qr-code@es-MX'})
MATCH (kw:SEOKeyword {key: 'seo:crear-codigo-qr@es-MX'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'secondary', t.is_slug_source = true, t.created_at = datetime()
ON MATCH SET t.rank = 'secondary', t.is_slug_source = true;

MATCH (en:EntityNative {key: 'entity:qr-code@es-MX'})
MATCH (kw:SEOKeyword {key: 'seo:generador-de-qr@es-MX'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'secondary', t.is_slug_source = false, t.created_at = datetime()
ON MATCH SET t.rank = 'secondary', t.is_slug_source = false;

MATCH (en:EntityNative {key: 'entity:qr-code@es-MX'})
MATCH (kw:SEOKeyword {key: 'seo:generador-codigo-qr@es-MX'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'secondary', t.is_slug_source = false, t.created_at = datetime()
ON MATCH SET t.rank = 'secondary', t.is_slug_source = false;

// =============================================================================
// de-DE: entity:qr-code@de-DE -> SEOKeywords
// primary: "qr code erstellen" (95K/KD52) ← slug source, transactional
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@de-DE'})
MATCH (kw:SEOKeyword {key: 'seo:qr-code-erstellen@de-DE'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'primary', t.is_slug_source = true, t.created_at = datetime()
ON MATCH SET t.rank = 'primary', t.is_slug_source = true;

// =============================================================================
// ja-JP: entity:qr-code@ja-JP -> SEOKeywords
// primary: "qrコード作成" (274K/KD35) ← slug source, native script
// =============================================================================

MATCH (en:EntityNative {key: 'entity:qr-code@ja-JP'})
MATCH (kw:SEOKeyword {key: 'seo:qr-code-sakusei@ja-JP'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'primary', t.is_slug_source = true, t.created_at = datetime()
ON MATCH SET t.rank = 'primary', t.is_slug_source = true;

// =============================================================================
// Log
// =============================================================================
RETURN 'TARGETS arcs created/updated: 9 arcs across 5 locales (with is_slug_source)' AS status;
