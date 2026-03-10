// ============================================================================
// PLAN A - Migration 086: Create ProjectGEOScope and ProjectSEOScope Nodes
// ============================================================================
// Priority: FOUNDATION (Project-level scope configuration)
// Fixes: Missing scope nodes for both projects
// CSR Impact: Enables project-specific SEO and GEO configuration
// ============================================================================

// Create ProjectSEOScope for qrcode-ai
MERGE (seo:ProjectSEOScope {key: 'seo-scope-qrcode-ai'})
ON CREATE SET
  seo.display_name = 'QR Code AI SEO Scope',
  seo.description = 'SEO configuration and keyword targeting for QR Code AI',
  seo.llm_context = 'USE: for SEO strategy, keyword priorities, and content optimization. TRIGGERS: seo, keywords, ranking, search, optimization. NOT: content generation (use PageNative). RELATES: Project (parent), SEOKeyword (targets).',
  seo.primary_keywords = ['qr code generator', 'create qr code', 'qr code maker'],
  seo.target_positions = [1, 2, 3],
  seo.focus_intents = ['transactional', 'informational'],
  seo.created_at = datetime(),
  seo.updated_at = datetime()
ON MATCH SET
  seo.updated_at = datetime();

// Link SEO scope to qrcode-ai project
MATCH (p:Project {key: 'qrcode-ai'})
MATCH (seo:ProjectSEOScope {key: 'seo-scope-qrcode-ai'})
MERGE (p)-[:HAS_SEO_SCOPE]->(seo);

// Create ProjectGEOScope for qrcode-ai
MERGE (geo:ProjectGEOScope {key: 'geo-scope-qrcode-ai'})
ON CREATE SET
  geo.display_name = 'QR Code AI GEO Scope',
  geo.description = 'Geographic targeting and market prioritization for QR Code AI',
  geo.llm_context = 'USE: for market prioritization, regional content strategies, and GEO targeting. TRIGGERS: geo, market, region, country, international. NOT: SEO keywords (use ProjectSEOScope). RELATES: Project (parent), Locale (targets), Country (covers).',
  geo.priority_markets = ['US', 'FR', 'DE', 'JP', 'BR'],
  geo.expansion_markets = ['MX', 'ES', 'IT', 'KR', 'IN'],
  geo.created_at = datetime(),
  geo.updated_at = datetime()
ON MATCH SET
  geo.updated_at = datetime();

// Link GEO scope to qrcode-ai project
MATCH (p:Project {key: 'qrcode-ai'})
MATCH (geo:ProjectGEOScope {key: 'geo-scope-qrcode-ai'})
MERGE (p)-[:HAS_GEO_SCOPE]->(geo);

// Create ProjectSEOScope for supernovae-studio (minimal)
MERGE (seo:ProjectSEOScope {key: 'seo-scope-supernovae-studio'})
ON CREATE SET
  seo.display_name = 'SuperNovae Studio SEO Scope',
  seo.description = 'SEO configuration for developer documentation and tools',
  seo.llm_context = 'USE: for documentation SEO and developer discoverability. TRIGGERS: seo, docs, documentation, api, developer. NOT: product SEO (use qrcode-ai scope).',
  seo.primary_keywords = ['ai workflow engine', 'knowledge graph', 'mcp server'],
  seo.focus_intents = ['informational', 'navigational'],
  seo.created_at = datetime(),
  seo.updated_at = datetime()
ON MATCH SET
  seo.updated_at = datetime();

// Link SEO scope to supernovae-studio project
MATCH (p:Project {key: 'supernovae-studio'})
MATCH (seo:ProjectSEOScope {key: 'seo-scope-supernovae-studio'})
MERGE (p)-[:HAS_SEO_SCOPE]->(seo);

// Create ProjectGEOScope for supernovae-studio (minimal - developer audience)
MERGE (geo:ProjectGEOScope {key: 'geo-scope-supernovae-studio'})
ON CREATE SET
  geo.display_name = 'SuperNovae Studio GEO Scope',
  geo.description = 'Geographic targeting for developer tools (global, English-first)',
  geo.llm_context = 'USE: for developer market targeting. TRIGGERS: geo, developers, global, international. NOT: consumer markets (use qrcode-ai scope).',
  geo.priority_markets = ['US', 'GB', 'DE', 'FR'],
  geo.created_at = datetime(),
  geo.updated_at = datetime()
ON MATCH SET
  geo.updated_at = datetime();

// Link GEO scope to supernovae-studio project
MATCH (p:Project {key: 'supernovae-studio'})
MATCH (geo:ProjectGEOScope {key: 'geo-scope-supernovae-studio'})
MERGE (p)-[:HAS_GEO_SCOPE]->(geo);

// Verify scope configuration
MATCH (p:Project)
OPTIONAL MATCH (p)-[:HAS_SEO_SCOPE]->(seo:ProjectSEOScope)
OPTIONAL MATCH (p)-[:HAS_GEO_SCOPE]->(geo:ProjectGEOScope)
RETURN p.key AS project,
       seo.key AS seo_scope,
       geo.key AS geo_scope,
       'Scopes configured' AS status;
