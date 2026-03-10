// NovaNet Organization Configuration v11.3
// Creates org-level config nodes (OrgConfig, Projects)
// Must run before 31-project-*.cypher files
//
// v11.3: OrgConfig (merged from Organization + Tenant)
// OrgConfig is the root of the org realm, owns all Projects

// ═══════════════════════════════════════════════════════════════════════════════
// ORGCONFIG: supernovae
// ═══════════════════════════════════════════════════════════════════════════════

MERGE (org:OrgConfig {key: "supernovae"})
SET org.display_name = "SuperNovae",
    org.description = "AI-powered content localization",
    org.llm_context = "USE: org-level context. TRIGGERS: supernovae, company. NOT: project-specific.",
    org.domain = "supernovae.studio",
    org.website_url = "https://supernovae.studio",
    org.industry = "Software / AI / Localization",
    org.created_at = coalesce(org.created_at, datetime()),
    org.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT: supernovae-studio (company project)
// ═══════════════════════════════════════════════════════════════════════════════

MERGE (p1:Project {key: "supernovae-studio"})
SET p1.display_name = "SuperNovae Studio",
    p1.description = "Company website and branding",
    p1.llm_context = "USE: company branding. TRIGGERS: studio, website, brand. NOT: products.",
    p1.domain = "supernovae.studio",
    p1.priority = 1,
    p1.created_at = coalesce(p1.created_at, datetime()),
    p1.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT: qrcode-ai (product project)
// ═══════════════════════════════════════════════════════════════════════════════

MERGE (p2:Project {key: "qrcode-ai"})
SET p2.display_name = "QR Code AI",
    p2.description = "AI-powered QR code generator with analytics",
    p2.llm_context = "USE: when generating QR Code AI content. TRIGGERS: qr code, qrcode-ai, qr generator. NOT: other projects.",
    p2.domain = "qrcode-ai.com",
    p2.default_locale = "en-US",
    p2.supported_locales = '["en-US", "fr-FR", "de-DE", "es-ES", "ja-JP"]',
    p2.priority = 2,
    p2.created_at = coalesce(p2.created_at, datetime()),
    p2.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════════
// LINKS: OrgConfig → Projects
// ═══════════════════════════════════════════════════════════════════════════════

MATCH (org:OrgConfig {key: "supernovae"})
MATCH (p1:Project {key: "supernovae-studio"})
MATCH (p2:Project {key: "qrcode-ai"})
MERGE (org)-[:HAS_PROJECT]->(p1)
MERGE (org)-[:HAS_PROJECT]->(p2);
