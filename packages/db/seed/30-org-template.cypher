// =============================================================================
// 30-org-template.cypher
// Organization Template (Tenant Data)
// =============================================================================
// Category: Tenant Data (30-39)
// Generated: No (manually curated)
//
// TEMPLATE FILE - Copy and customize for each organization:
//   30-org-supernovae.cypher
//   30-org-acme.cypher
//   etc.
//
// Organization is the root of tenant data hierarchy:
//   Organization → Project → Page → Block
//                         → Entity
//                         → BrandIdentity
// =============================================================================

// ─────────────────────────────────────────────────────────────────────────────
// ORGANIZATION
// ─────────────────────────────────────────────────────────────────────────────

MERGE (org:Organization {key: 'supernovae-studio'})
SET org.display_name = 'SuperNovae Studio',
    org.slug = 'supernovae',
    org.description = 'AI-powered localization platform',
    org.website = 'https://supernovae.studio',
    org.created_at = datetime(),
    org.updated_at = datetime();

// Link to Kind
MATCH (k:Kind {label: 'Organization'})
MERGE (org)-[:OF_KIND]->(k);

// ─────────────────────────────────────────────────────────────────────────────
// BRAND IDENTITY
// ─────────────────────────────────────────────────────────────────────────────

MERGE (brand:BrandIdentity {key: 'supernovae-studio-brand'})
SET brand.display_name = 'SuperNovae Brand',
    brand.voice_tone = 'Professional yet approachable, technically precise',
    brand.primary_color = '#6366f1',
    brand.secondary_color = '#22d3ee',
    brand.typography = 'Inter, system-ui',
    brand.logo_url = '/assets/logo.svg',
    brand.updated_at = datetime();

// Link to Organization
MATCH (org:Organization {key: 'supernovae-studio'})
MERGE (org)-[:HAS_BRAND]->(brand);

// Link to Kind
MATCH (k:Kind {label: 'BrandIdentity'})
MERGE (brand)-[:OF_KIND]->(k);

// ─────────────────────────────────────────────────────────────────────────────
// PROJECTS (add more as needed)
// ─────────────────────────────────────────────────────────────────────────────

// See 31-project-*.cypher for project definitions

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION
// ─────────────────────────────────────────────────────────────────────────────

// MATCH (org:Organization {key: 'supernovae-studio'})-[r]->(n)
// RETURN type(r), labels(n), n.key;
