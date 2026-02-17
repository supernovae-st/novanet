// Migration 002: Split Brand node - move design properties to BrandDesign
// Problem: Brand instance had 13 BrandDesign properties (ADR-028 violation)
// Run: AFTER 31-project-qrcode-ai.cypher has been seeded
// Idempotent: MERGE on BrandDesign key

MATCH (b:Brand {key: "brand-qrcode-ai"})
MERGE (b)-[:HAS_DESIGN]->(bd:BrandDesign {key: "brand-design-qrcode-ai"})
SET bd.display_name = "QR Code AI Design System",
    bd.description = "Visual design tokens and guidelines for QR Code AI",
    bd.color_primary = coalesce(b.color_primary, "#6366F1"),
    bd.color_secondary = coalesce(b.color_secondary, "#8B5CF6"),
    bd.color_accent = coalesce(b.color_accent, "#F59E0B"),
    bd.color_background = coalesce(b.color_background, "#FFFFFF"),
    bd.color_text = coalesce(b.color_text, "#1F2937"),
    bd.color_palette = coalesce(b.color_palette, '["#6366F1", "#8B5CF6", "#F59E0B", "#10B981", "#EF4444"]'),
    bd.font_primary = coalesce(b.font_primary, "Inter"),
    bd.font_secondary = coalesce(b.font_secondary, "Poppins"),
    bd.font_mono = coalesce(b.font_mono, "JetBrains Mono"),
    bd.typography_scale = coalesce(b.typography_scale, '[{"name": "h1", "size": "3rem", "weight": "700"}]'),
    bd.border_radius = coalesce(b.border_radius, "0.75rem"),
    bd.shadow_style = coalesce(b.shadow_style, "soft"),
    bd.animation_style = coalesce(b.animation_style, "smooth"),
    bd.llm_context = "USE: when generating visual content or applying brand design tokens. TRIGGERS: colors, fonts, design tokens, @brand.design.",
    bd.created_at = coalesce(bd.created_at, datetime()),
    bd.updated_at = datetime()
WITH b, bd
REMOVE b.color_primary, b.color_secondary, b.color_accent,
       b.color_background, b.color_text, b.color_palette,
       b.font_primary, b.font_secondary, b.font_mono,
       b.typography_scale, b.border_radius, b.shadow_style, b.animation_style
SET b.brand_name = coalesce(b.brand_name, "QR Code AI"),
    b.updated_at = datetime()
RETURN b.key AS brand_cleaned, bd.key AS brand_design_created;
