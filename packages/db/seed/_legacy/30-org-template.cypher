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
// PROJECTS
// ─────────────────────────────────────────────────────────────────────────────

// QR Code AI - Primary product
MERGE (proj:Project {key: 'qrcode-ai'})
SET proj.display_name = 'QR Code AI',
    proj.slug = 'qrcode-ai',
    proj.description = 'AI-powered QR code generator with analytics',
    proj.website = 'https://qrcode-ai.com',
    proj.status = 'active',
    proj.created_at = datetime(),
    proj.updated_at = datetime();

// Link Project to Kind
MATCH (k:Kind {label: 'Project'})
MERGE (proj)-[:OF_KIND]->(k);

// Link Organization → Project (HAS_PROJECT arc)
MATCH (org:Organization {key: 'supernovae-studio'}),
      (proj:Project {key: 'qrcode-ai'})
MERGE (org)-[:HAS_PROJECT]->(proj);

// See 31-project-*.cypher for project-specific data (BrandIdentity, ProjectL10n)

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION
// ─────────────────────────────────────────────────────────────────────────────

// MATCH (org:Organization {key: 'supernovae-studio'})-[r]->(n)
// RETURN type(r), labels(n), n.key;
