// NovaNet Project Data v0.13.0 - QRCode-AI Complete Setup
// v0.13.0 ADR-029: *Native pattern (ProjectNative->ProjectNative, HAS_CONTENT->HAS_NATIVE)
// v0.19.0: Standard properties (ADR-042)
// YAML v7.11.0 alignment - icon, priority, freshness removed (managed in YAML views)
//
// Creates project-specific nodes:
// - BrandIdentity (invariant)
// - ProjectNative (localized identity + CTAs + SEO + target_audience)
//
// Removed in v7.2.5: Audience, AudienceL10n, ValuePropL10n, SocialProofL10n
// Removed in v8.2.0: icon, priority, freshness (now in YAML views)
// v10.9.0: ProjectL10n renamed to ProjectNative (Decision 11)
// v0.13.0: ProjectNative renamed to ProjectNative (ADR-029)
//
// STANDARD PROPERTIES:
//   key, display_name, node_class, content, llm_context, provenance, created_at, updated_at

// ═══════════════════════════════════════════════════════════════════════════════
// BRAND IDENTITY (invariant - visual/artistic direction)
// ═══════════════════════════════════════════════════════════════════════════════

MATCH (p:Project {key: "qrcode-ai"})
MERGE (p)-[:HAS_BRAND]->(bi:Brand {key: "brand-qrcode-ai"}) // v0.12.4 ADR-028: was HAS_BRAND_IDENTITY + BrandIdentity
SET bi.display_name = "QR Code AI Brand",
    bi.node_class = "Brand",
    bi.brand_name = "QR Code AI",
    bi.content = "Visual identity and design system for QR Code AI",
    bi.llm_context = "USE: when generating visual content, images, or style decisions. TRIGGERS: brand, colors, fonts, style. NOT: text content (use ProjectNative).",
    bi.provenance = '{"source":"seed","file":"31-project-qrcode-ai.cypher"}',
    bi.style_keywords = '["modern", "clean", "professional", "tech-forward"]',
    bi.style_mood = "Professional yet approachable, tech-savvy but accessible",
    bi.style_influences = '["Apple", "Stripe", "Linear"]',
    bi.image_style = "Clean product shots, abstract QR patterns, tech illustrations",
    bi.image_do = '["Use gradient overlays", "Show QR codes in context", "Modern devices"]',
    bi.image_dont = '["Stock photos with obvious poses", "Cluttered backgrounds", "Outdated devices"]',
    bi.logo_usage_rules = '["Min size 32px", "Clear space equal to height", "No rotation"]',
    bi.created_at = coalesce(bi.created_at, datetime()),
    bi.updated_at = datetime();

// BrandDesign (visual design system - ADR-028)
MATCH (bi:Brand {key: "brand-qrcode-ai"})
MERGE (bi)-[:HAS_DESIGN]->(bd:BrandDesign {key: "brand-design-qrcode-ai"})
SET bd.display_name = "QR Code AI Design System",
    bd.node_class = "BrandDesign",
    bd.content = "Visual design tokens and guidelines for QR Code AI",
    bd.color_primary = "#6366F1",
    bd.color_secondary = "#8B5CF6",
    bd.color_accent = "#F59E0B",
    bd.color_background = "#FFFFFF",
    bd.color_text = "#1F2937",
    bd.color_palette = '["#6366F1", "#8B5CF6", "#F59E0B", "#10B981", "#EF4444"]',
    bd.font_primary = "Inter",
    bd.font_secondary = "Poppins",
    bd.font_mono = "JetBrains Mono",
    bd.typography_scale = '[{"name": "h1", "size": "3rem", "weight": "700"}, {"name": "body", "size": "1rem", "weight": "400"}]',
    bd.border_radius = "0.75rem",
    bd.shadow_style = "soft",
    bd.animation_style = "smooth",
    bd.llm_context = "USE: when generating visual content or applying brand design tokens. TRIGGERS: colors, fonts, design tokens, @brand.design.",
    bd.provenance = '{"source":"seed","file":"31-project-qrcode-ai.cypher"}',
    bd.created_at = coalesce(bd.created_at, datetime()),
    bd.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT CONTENT (localized identity + CTAs + SEO + target_audience) - v7.2.5
// v10.9.0: Renamed from ProjectL10n to ProjectNative (Decision 11)
// ═══════════════════════════════════════════════════════════════════════════════

// ProjectNative: en-US
MATCH (p:Project {key: "qrcode-ai"}), (l:Locale {key: "en-US"})
MERGE (p)-[:HAS_NATIVE]->(pl:ProjectNative {key: "qrcode-ai-en-US"})
MERGE (pl)-[:FOR_LOCALE]->(l)
SET pl.display_name = "QR Code AI",
    pl.node_class = "ProjectNative",
    pl.content = "English localization for QR Code AI project identity",
    pl.llm_context = "USE: native English brand messaging. TRIGGERS: en-US project content. NOT: translation.",
    pl.provenance = '{"source":"seed","file":"31-project-qrcode-ai.cypher"}',
    pl.what_short = "AI-powered QR code generator",
    pl.what_medium = "Create customizable QR codes with built-in analytics and AI design suggestions",
    pl.what_long = "QR Code AI is a comprehensive platform for creating, managing, and tracking QR codes. Generate custom designs, track scans in real-time, and leverage AI to optimize your codes for maximum engagement.",
    pl.tagline = "Smart QR Codes, Smarter Marketing",
    pl.pitch_one_liner = "Create professional QR codes in seconds with AI-powered design and analytics.",
    pl.pitch_elevator = "QR Code AI helps businesses create professional, trackable QR codes instantly. Our AI suggests optimal designs while real-time analytics show you exactly how your codes perform.",
    pl.pitch_detailed = "QR Code AI transforms how businesses use QR codes. Create stunning, customized codes in seconds with AI-powered design suggestions. Track every scan with detailed analytics including location, device, and time data. Perfect for marketing campaigns, business cards, menus, and more.",
    pl.voice_personality = '["Professional", "Helpful", "Tech-savvy"]',
    pl.voice_tone = '{"formal_casual": 0.4, "serious_playful": 0.3, "technical_simple": 0.4, "reserved_enthusiastic": 0.5}',
    pl.voice_do = '["Be clear and direct", "Use action verbs", "Highlight benefits"]',
    pl.voice_dont = '["Use jargon", "Be overly casual", "Make false promises"]',
    pl.cta_primary = "Create Free QR Code",
    pl.cta_secondary = "See Pricing",
    pl.cta_tertiary = "View Demo",
    pl.meta_description = "Create free QR codes with AI-powered design and analytics. Professional QR code generator for business cards, marketing, menus and more.",
    pl.primary_keywords = '["qr code generator", "free qr code", "qr code maker"]',
    pl.secondary_keywords = '["dynamic qr code", "qr code analytics", "custom qr code"]',
    pl.target_audience = '[{"type": "primary", "title": "Small Business Owner", "pain_points": ["Limited marketing budget", "Need professional tools", "Time constraints"], "goals": ["Grow customer base", "Track marketing ROI", "Look professional"]}, {"type": "secondary", "title": "Marketing Professional", "pain_points": ["Need detailed analytics", "Campaign tracking complexity"], "goals": ["Measure campaign performance", "Optimize conversion rates"]}]',
    pl.created_at = coalesce(pl.created_at, datetime()),
    pl.updated_at = datetime();

// ProjectNative: fr-FR
MATCH (p:Project {key: "qrcode-ai"}), (l:Locale {key: "fr-FR"})
MERGE (p)-[:HAS_NATIVE]->(pl:ProjectNative {key: "qrcode-ai-fr-FR"})
MERGE (pl)-[:FOR_LOCALE]->(l)
SET pl.display_name = "QR Code AI",
    pl.node_class = "ProjectNative",
    pl.content = "Localisation francaise de l'identite QR Code AI",
    pl.llm_context = "USE: messaging de marque francais natif. TRIGGERS: fr-FR contenu projet. NOT: traduction.",
    pl.provenance = '{"source":"seed","file":"31-project-qrcode-ai.cypher"}',
    pl.what_short = "Generateur de QR codes propulse par l'IA",
    pl.what_medium = "Creez des QR codes personnalisables avec analytics integrees et suggestions de design IA",
    pl.what_long = "QR Code AI est une plateforme complete pour creer, gerer et suivre vos QR codes. Generez des designs sur mesure, suivez les scans en temps reel, et utilisez l'IA pour optimiser l'engagement.",
    pl.tagline = "QR Codes intelligents, marketing plus malin",
    pl.pitch_one_liner = "Creez des QR codes professionnels en quelques secondes avec design IA et analytics.",
    pl.pitch_elevator = "QR Code AI aide les entreprises a creer des QR codes professionnels et tracables instantanement. Notre IA suggere des designs optimaux tandis que les analytics temps reel montrent la performance.",
    pl.pitch_detailed = "QR Code AI transforme l'utilisation des QR codes en entreprise. Creez des codes personnalises en quelques secondes avec les suggestions de design IA. Suivez chaque scan avec des analytics detaillees. Parfait pour les campagnes marketing, cartes de visite, menus et plus.",
    pl.voice_personality = '["Professionnel", "Utile", "Tech-savvy"]',
    pl.voice_tone = '{"formal_casual": 0.5, "serious_playful": 0.3, "technical_simple": 0.4, "reserved_enthusiastic": 0.4}',
    pl.voice_do = "[\"Etre clair et direct\", \"Utiliser des verbes d'action\", \"Mettre en avant les benefices\"]",
    pl.voice_dont = "[\"Utiliser du jargon\", \"Etre trop familier\", \"Faire de fausses promesses\"]",
    pl.cta_primary = "Creer un QR Code gratuit",
    pl.cta_secondary = "Voir les tarifs",
    pl.cta_tertiary = "Voir la demo",
    pl.meta_description = "Creez des QR codes gratuits avec design IA et analytics. Generateur de QR codes professionnel pour cartes de visite, marketing, menus et plus.",
    pl.primary_keywords = '["generateur qr code", "qr code gratuit", "creer qr code"]',
    pl.secondary_keywords = '["qr code dynamique", "qr code analytics", "qr code personnalise"]',
    pl.target_audience = "[{\"type\": \"primary\", \"title\": \"Proprietaire de PME\", \"pain_points\": [\"Budget marketing limite\", \"Besoin d'outils professionnels\", \"Contraintes de temps\"], \"goals\": [\"Developper la clientele\", \"Suivre le ROI marketing\", \"Image professionnelle\"]}, {\"type\": \"secondary\", \"title\": \"Professionnel Marketing\", \"pain_points\": [\"Besoin d'analytics detaillees\", \"Complexite du suivi de campagne\"], \"goals\": [\"Mesurer la performance\", \"Optimiser les conversions\"]}]",
    pl.created_at = coalesce(pl.created_at, datetime()),
    pl.updated_at = datetime();
