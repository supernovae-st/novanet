// ═══════════════════════════════════════════════════════════════════════════════
// 52-targets-arcs.cypher — TARGETS arcs linking EntityNative to SEOKeyword
// v0.13.1 - ADR-032 (URL Slugification), semantic arc family
// ═══════════════════════════════════════════════════════════════════════════════
//
// TARGETS arc: EntityNative -> SEOKeyword
// Properties:
//   - rank: 'primary' (main target) | 'secondary' (supporting keywords)
//
// Purpose: Links locale-specific entity content to SEO keywords for that locale
// ═══════════════════════════════════════════════════════════════════════════════

// =============================================================================
// en-US: entity:qr-code@en-US -> SEOKeywords
// =============================================================================

// Primary: "qr code generator" (699K volume)
MATCH (en:EntityNative {key: 'entity:qr-code@en-US'})
MATCH (kw:SEOKeyword {key: 'seo:qr-code-generator@en-US'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'primary', t.created_at = datetime()
ON MATCH SET t.rank = 'primary';

// Secondary: "qr code" (589K volume)
MATCH (en:EntityNative {key: 'entity:qr-code@en-US'})
MATCH (kw:SEOKeyword {key: 'seo:qr-code@en-US'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'secondary', t.created_at = datetime()
ON MATCH SET t.rank = 'secondary';

// =============================================================================
// fr-FR: entity:qr-code@fr-FR -> SEOKeywords
// =============================================================================

// Primary: "qr code" (109K volume)
MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
MATCH (kw:SEOKeyword {key: 'seo:qr-code@fr-FR'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'primary', t.created_at = datetime()
ON MATCH SET t.rank = 'primary';

// Secondary: "générateur qr code" (28K volume)
MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
MATCH (kw:SEOKeyword {key: 'seo:generateur-qr-code@fr-FR'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'secondary', t.created_at = datetime()
ON MATCH SET t.rank = 'secondary';

// =============================================================================
// es-MX: entity:qr-code@es-MX -> SEOKeywords
// =============================================================================

// Primary: "código qr" (44K volume)
MATCH (en:EntityNative {key: 'entity:qr-code@es-MX'})
MATCH (kw:SEOKeyword {key: 'seo:codigo-qr@es-MX'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'primary', t.created_at = datetime()
ON MATCH SET t.rank = 'primary';

// Secondary: "generador código qr" (15K volume)
MATCH (en:EntityNative {key: 'entity:qr-code@es-MX'})
MATCH (kw:SEOKeyword {key: 'seo:generador-codigo-qr@es-MX'})
MERGE (en)-[t:TARGETS]->(kw)
ON CREATE SET t.rank = 'secondary', t.created_at = datetime()
ON MATCH SET t.rank = 'secondary';
