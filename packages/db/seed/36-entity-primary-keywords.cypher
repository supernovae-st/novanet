// packages/db/seed/36-entity-primary-keywords.cypher
// v0.13.0 - HAS_KEYWORD arcs: Entity → SEOKeyword
//
// ADR-030: Slug Ownership - Entity owns semantics, Page owns URL
// ADR-032: URL Slugification - convergence_boost = 1 + (N × 0.2)
//
// This seed creates HAS_KEYWORD arcs linking Entity to its target keywords.
// rank: primary = main keyword (1 per Entity), secondary = supporting keywords
//
// These arcs enable:
// 1. Slug derivation via HAS_KEYWORD → SEOKeyword.volume scoring
// 2. Content optimization via Entity → keyword targeting
// 3. Convergence boost calculation (multiple entities targeting same keyword)

// ============================================================================
// 1. PILLAR ENTITIES → PRIMARY KEYWORDS
// ============================================================================

// qr-code-generator (PILLAR)
MATCH (e:Entity {key: 'qr-code-generator'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-generator'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

MATCH (e:Entity {key: 'qr-code-generator'})
MATCH (k:SEOKeyword)
WHERE k.key IN ['seo-free-qr-code-generator', 'seo-qr-code-maker', 'seo-make-qr-code', 'seo-generate-qr-code']
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'secondary',
    r.created_at = datetime();

// create-qr-code (ACTION)
MATCH (e:Entity {key: 'create-qr-code'})
MATCH (k:SEOKeyword {key: 'seo-create-qr-code'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

MATCH (e:Entity {key: 'create-qr-code'})
MATCH (k:SEOKeyword)
WHERE k.key IN ['seo-how-to-create-qr-code', 'seo-how-to-make-qr-code']
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'secondary',
    r.created_at = datetime();

// qr-code (ROOT)
MATCH (e:Entity {key: 'qr-code'})
MATCH (k:SEOKeyword {key: 'seo-qr-code'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

MATCH (e:Entity {key: 'qr-code'})
MATCH (k:SEOKeyword)
WHERE k.key IN ['seo-free-qr-code']
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'secondary',
    r.created_at = datetime();

// ============================================================================
// 2. QR CODE TYPE ENTITIES → PRIMARY KEYWORDS
// ============================================================================

// qr-code-wifi
MATCH (e:Entity {key: 'qr-code-wifi'})
MATCH (k:SEOKeyword {key: 'seo-wifi-qr-code'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// qr-code-instagram
MATCH (e:Entity {key: 'qr-code-instagram'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-instagram'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

MATCH (e:Entity {key: 'qr-code-instagram'})
MATCH (k:SEOKeyword {key: 'seo-instagram-qr-code'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'secondary',
    r.created_at = datetime();

// qr-code-menu
MATCH (e:Entity {key: 'qr-code-menu'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-menu'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

MATCH (e:Entity {key: 'qr-code-menu'})
MATCH (k:SEOKeyword {key: 'seo-menu-qr-code'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'secondary',
    r.created_at = datetime();

// qr-code-vcard
MATCH (e:Entity {key: 'qr-code-vcard'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-business-card'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// qr-code-whatsapp
MATCH (e:Entity {key: 'qr-code-whatsapp'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-whatsapp'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// qr-code-facebook
MATCH (e:Entity {key: 'qr-code-facebook'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-facebook'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// qr-code-tiktok
MATCH (e:Entity {key: 'qr-code-tiktok'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-tiktok'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// qr-code-youtube
MATCH (e:Entity {key: 'qr-code-youtube'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-youtube'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// ============================================================================
// 3. DYNAMIC/STATIC QR CODES
// ============================================================================

// dynamic-qr-code
MATCH (e:Entity {key: 'dynamic-qr-code'})
MATCH (k:SEOKeyword {key: 'seo-dynamic-qr-code'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// custom-qr-code
MATCH (e:Entity {key: 'custom-qr-code'})
MATCH (k:SEOKeyword {key: 'seo-custom-qr-code'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// qr-code-with-logo
MATCH (e:Entity {key: 'qr-code-with-logo'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-with-logo'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// ============================================================================
// 4. QR CODE SCANNER
// ============================================================================

// qr-code-scanner
MATCH (e:Entity {key: 'qr-code-scanner'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-scanner'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

MATCH (e:Entity {key: 'qr-code-scanner'})
MATCH (k:SEOKeyword {key: 'seo-qr-code-reader'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'secondary',
    r.created_at = datetime();

// scan-qr-code
MATCH (e:Entity {key: 'scan-qr-code'})
MATCH (k:SEOKeyword {key: 'seo-scan-qr-code'})
MERGE (e)-[r:HAS_KEYWORD]->(k)
SET r.rank = 'primary',
    r.created_at = datetime();

// ============================================================================
// 5. VERIFICATION QUERY
// ============================================================================
// Run this to verify HAS_KEYWORD arcs were created:
//
// MATCH (e:Entity)-[r:HAS_KEYWORD]->(k:SEOKeyword)
// RETURN e.key, r.rank, k.key, k.volume
// ORDER BY k.volume DESC;
//
