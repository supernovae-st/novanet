// NovaNet Tenant Configuration v10.6
// Creates tenant-level config nodes (Tenant, Project)
// Must run before 31-project-*.cypher files
//
// v10.6: 2-Realm Architecture (global + tenant)
// Tenant realm contains all business-specific content

// ═══════════════════════════════════════════════════════════════════════════════
// TENANT: supernovae-studio
// ═══════════════════════════════════════════════════════════════════════════════

MERGE (t:Tenant {key: "supernovae-studio"})
SET t.display_name = "SuperNovae Studio",
    t.description = "AI-powered tools for the creative economy",
    t.llm_context = "USE: when needing tenant-level context. TRIGGERS: supernovae, studio, organization. NOT: project-specific (use Project).",
    t.created_at = coalesce(t.created_at, datetime()),
    t.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT: qrcode-ai
// ═══════════════════════════════════════════════════════════════════════════════

MERGE (p:Project {key: "qrcode-ai"})
SET p.display_name = "QR Code AI",
    p.description = "AI-powered QR code generator with analytics",
    p.llm_context = "USE: when generating QR Code AI content. TRIGGERS: qr code, qrcode-ai, qr generator. NOT: other projects.",
    p.domain = "qrcode-ai.com",
    p.default_locale = "en-US",
    p.supported_locales = '["en-US", "fr-FR", "de-DE", "es-ES", "ja-JP"]',
    p.created_at = coalesce(p.created_at, datetime()),
    p.updated_at = datetime();

// Link Project to Tenant
MATCH (t:Tenant {key: "supernovae-studio"}), (p:Project {key: "qrcode-ai"})
MERGE (t)-[:HAS_PROJECT]->(p);
