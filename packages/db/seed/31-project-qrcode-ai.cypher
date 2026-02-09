// NovaNet Project Data v8.2.0 - QRCode-AI Complete Setup
// YAML v7.11.0 alignment - icon, priority, freshness removed (managed in YAML views)
//
// Creates project-specific nodes:
// - BrandIdentity (invariant)
// - ProjectContent (localized identity + CTAs + SEO + target_audience)
//
// Removed in v7.2.5: Audience, AudienceL10n, ValuePropL10n, SocialProofL10n
// Removed in v8.2.0: icon, priority, freshness (now in YAML views)
// v10.9.0: ProjectL10n renamed to ProjectContent (Decision 11)
//
// STANDARD PROPERTIES:
//   key, display_name, description, llm_context, created_at, updated_at

// ═══════════════════════════════════════════════════════════════════════════════
// BRAND IDENTITY (invariant - visual/artistic direction)
// ═══════════════════════════════════════════════════════════════════════════════

MATCH (p:Project {key: "qrcode-ai"})
MERGE (p)-[:HAS_BRAND_IDENTITY]->(bi:BrandIdentity {key: "brand-qrcode-ai"})
SET bi.display_name = "QR Code AI Brand",
    bi.description = "Visual identity and design system for QR Code AI",
    bi.llm_context = "USE: when generating visual content, images, or style decisions. TRIGGERS: brand, colors, fonts, style. NOT: text content (use ProjectContent).",
    bi.color_primary = "#6366F1",
    bi.color_secondary = "#8B5CF6",
    bi.color_accent = "#F59E0B",
    bi.color_background = "#FFFFFF",
    bi.color_text = "#1F2937",
    bi.color_palette = '["#6366F1", "#8B5CF6", "#F59E0B", "#10B981", "#EF4444"]',
    bi.font_primary = "Inter",
    bi.font_secondary = "Poppins",
    bi.font_mono = "JetBrains Mono",
    bi.typography_scale = '[{"name": "h1", "size": "3rem", "weight": "700"}, {"name": "body", "size": "1rem", "weight": "400"}]',
    bi.style_keywords = '["modern", "clean", "professional", "tech-forward"]',
    bi.style_mood = "Professional yet approachable, tech-savvy but accessible",
    bi.style_influences = '["Apple", "Stripe", "Linear"]',
    bi.image_style = "Clean product shots, abstract QR patterns, tech illustrations",
    bi.image_do = '["Use gradient overlays", "Show QR codes in context", "Modern devices"]',
    bi.image_dont = '["Stock photos with obvious poses", "Cluttered backgrounds", "Outdated devices"]',
    bi.border_radius = "0.75rem",
    bi.shadow_style = "soft",
    bi.animation_style = "smooth",
    bi.logo_usage_rules = '["Min size 32px", "Clear space equal to height", "No rotation"]',
    bi.created_at = coalesce(bi.created_at, datetime()),
    bi.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT CONTENT (localized identity + CTAs + SEO + target_audience) - v7.2.5
// v10.9.0: Renamed from ProjectL10n to ProjectContent (Decision 11)
// ═══════════════════════════════════════════════════════════════════════════════

// ProjectContent: en-US
MATCH (p:Project {key: "qrcode-ai"}), (l:Locale {key: "en-US"})
MERGE (p)-[:HAS_CONTENT]->(pl:ProjectContent {key: "qrcode-ai-en-US"})
MERGE (pl)-[:FOR_LOCALE]->(l)
SET pl.display_name = "QR Code AI",
    pl.description = "English localization for QR Code AI project identity",
    pl.llm_context = "USE: native English brand messaging. TRIGGERS: en-US project content. NOT: translation.",
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

// ProjectContent: fr-FR
MATCH (p:Project {key: "qrcode-ai"}), (l:Locale {key: "fr-FR"})
MERGE (p)-[:HAS_CONTENT]->(pl:ProjectContent {key: "qrcode-ai-fr-FR"})
MERGE (pl)-[:FOR_LOCALE]->(l)
SET pl.display_name = "QR Code AI",
    pl.description = "Localisation française de l'identité QR Code AI",
    pl.llm_context = "USE: messaging de marque français natif. TRIGGERS: fr-FR contenu projet. NOT: traduction.",
    pl.what_short = "Générateur de QR codes propulsé par l'IA",
    pl.what_medium = "Créez des QR codes personnalisables avec analytics intégrées et suggestions de design IA",
    pl.what_long = "QR Code AI est une plateforme complète pour créer, gérer et suivre vos QR codes. Générez des designs sur mesure, suivez les scans en temps réel, et utilisez l'IA pour optimiser l'engagement.",
    pl.tagline = "QR Codes intelligents, marketing plus malin",
    pl.pitch_one_liner = "Créez des QR codes professionnels en quelques secondes avec design IA et analytics.",
    pl.pitch_elevator = "QR Code AI aide les entreprises à créer des QR codes professionnels et traçables instantanément. Notre IA suggère des designs optimaux tandis que les analytics temps réel montrent la performance.",
    pl.pitch_detailed = "QR Code AI transforme l'utilisation des QR codes en entreprise. Créez des codes personnalisés en quelques secondes avec les suggestions de design IA. Suivez chaque scan avec des analytics détaillées. Parfait pour les campagnes marketing, cartes de visite, menus et plus.",
    pl.voice_personality = '["Professionnel", "Utile", "Tech-savvy"]',
    pl.voice_tone = '{"formal_casual": 0.5, "serious_playful": 0.3, "technical_simple": 0.4, "reserved_enthusiastic": 0.4}',
    pl.voice_do = "[\"Être clair et direct\", \"Utiliser des verbes d'action\", \"Mettre en avant les bénéfices\"]",
    pl.voice_dont = "[\"Utiliser du jargon\", \"Être trop familier\", \"Faire de fausses promesses\"]",
    pl.cta_primary = "Créer un QR Code gratuit",
    pl.cta_secondary = "Voir les tarifs",
    pl.cta_tertiary = "Voir la démo",
    pl.meta_description = "Créez des QR codes gratuits avec design IA et analytics. Générateur de QR codes professionnel pour cartes de visite, marketing, menus et plus.",
    pl.primary_keywords = '["générateur qr code", "qr code gratuit", "créer qr code"]',
    pl.secondary_keywords = '["qr code dynamique", "qr code analytics", "qr code personnalisé"]',
    pl.target_audience = "[{\"type\": \"primary\", \"title\": \"Propriétaire de PME\", \"pain_points\": [\"Budget marketing limité\", \"Besoin d'outils professionnels\", \"Contraintes de temps\"], \"goals\": [\"Développer la clientèle\", \"Suivre le ROI marketing\", \"Image professionnelle\"]}, {\"type\": \"secondary\", \"title\": \"Professionnel Marketing\", \"pain_points\": [\"Besoin d'analytics détaillées\", \"Complexité du suivi de campagne\"], \"goals\": [\"Mesurer la performance\", \"Optimiser les conversions\"]}]",
    pl.created_at = coalesce(pl.created_at, datetime()),
    pl.updated_at = datetime();
