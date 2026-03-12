// ============================================================================
// PLAN A - Migration 085: Fix supernovae-studio Project
// ============================================================================
// Priority: FOUNDATION (Required for project-locale relationships)
// Fixes: Missing default_locale, supported_locales, Brand node
// CSR Impact: Completes project configuration for supernovae-studio
// ============================================================================

// Link OrgConfig to existing projects
MATCH (oc:OrgConfig {key: 'supernovae'})
MATCH (p:Project)
MERGE (oc)-[:HAS_PROJECT]->(p);

// Fix qrcode-ai project (ensure it has proper locale config)
MATCH (p:Project {key: 'qrcode-ai'})
MATCH (l:Locale {key: 'en-US'})
MERGE (p)-[:DEFAULT_LOCALE]->(l);

// Create supernovae-studio project if it doesn't exist
MERGE (p:Project {key: 'supernovae-studio'})
ON CREATE SET
  p.display_name = 'SuperNovae Studio',
  p.description = 'Internal tools and documentation for SuperNovae AI ecosystem',
  p.llm_context = 'USE: for internal tooling, developer docs, and ecosystem management. TRIGGERS: studio, internal, tools, dx, developer. NOT: customer-facing products (use qrcode-ai). RELATES: OrgConfig (parent), Brand (has), Locale (supports).',
  p.brand_name = 'SuperNovae Studio',
  p.website_url = 'https://supernovae.studio',
  p.category = 'Developer Tools',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.updated_at = datetime();

// Create Brand node for supernovae-studio if missing
MERGE (b:Brand {key: 'brand-supernovae-studio'})
ON CREATE SET
  b.display_name = 'SuperNovae Studio Brand',
  b.description = 'Brand identity for SuperNovae Studio developer tools',
  b.llm_context = 'USE: for brand voice, visual identity, and tone guidelines. TRIGGERS: brand, identity, voice, tone, supernovae. NOT: product features (use Project).',
  b.primary_color = '#8B5CF6',
  b.created_at = datetime(),
  b.updated_at = datetime()
ON MATCH SET
  b.updated_at = datetime();

// Link project to brand
MATCH (p:Project {key: 'supernovae-studio'})
MATCH (b:Brand {key: 'brand-supernovae-studio'})
MERGE (p)-[:HAS_BRAND]->(b);

// Set default locale for supernovae-studio to en-US
MATCH (p:Project {key: 'supernovae-studio'})
MATCH (l:Locale {key: 'en-US'})
MERGE (p)-[:DEFAULT_LOCALE]->(l);

// Add supported locales (en-US, fr-FR for dev team)
MATCH (p:Project {key: 'supernovae-studio'})
MATCH (l:Locale) WHERE l.key IN ['en-US', 'fr-FR']
MERGE (p)-[:SUPPORTS_LOCALE]->(l);

// Link supernovae-studio to OrgConfig
MATCH (oc:OrgConfig {key: 'supernovae'})
MATCH (p:Project {key: 'supernovae-studio'})
MERGE (oc)-[:HAS_PROJECT]->(p);

// Verify project configuration
MATCH (p:Project {key: 'supernovae-studio'})
OPTIONAL MATCH (p)-[:DEFAULT_LOCALE]->(dl:Locale)
OPTIONAL MATCH (p)-[:SUPPORTS_LOCALE]->(sl:Locale)
OPTIONAL MATCH (p)-[:HAS_BRAND]->(b:Brand)
RETURN p.key AS project,
       dl.key AS default_locale,
       collect(DISTINCT sl.key) AS supported_locales,
       b.key AS brand,
       'Project configured' AS status;
