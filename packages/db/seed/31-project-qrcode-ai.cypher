// NovaNet Project Data v8.2.0 - QRCode-AI Complete Setup
// YAML v7.11.0 alignment - icon, priority, freshness removed (managed in YAML views)
//
// Creates project-specific nodes:
// - BrandIdentity (invariant)
// - ProjectL10n (localized identity + CTAs + SEO + target_audience)
//
// Removed in v7.2.5: Audience, AudienceL10n, ValuePropL10n, SocialProofL10n
// Removed in v8.2.0: icon, priority, freshness (now in YAML views)
//
// STANDARD PROPERTIES:
//   key, display_name, description, llm_context, created_at, updated_at

// ═══════════════════════════════════════════════════════════════════════════════
// BRAND IDENTITY (invariant - visual/artistic direction)
// ═══════════════════════════════════════════════════════════════════════════════

MATCH (p:Project {key: "project-qrcode-ai"})
CREATE (p)-[:HAS_BRAND_IDENTITY]->(bi:BrandIdentity {
  // 1. IDENTIFICATION
  key: "brand-qrcode-ai",
  display_name: "QR Code AI Brand",
  // 2. DOCUMENTATION
  description: "Visual identity and design system for QR Code AI",
  llm_context: "USE: when generating visual content, images, or style decisions. TRIGGERS: brand, colors, fonts, style. NOT: text content (use ProjectL10n).",
  // 3. COLORS
  color_primary: "#6366F1",
  color_secondary: "#8B5CF6",
  color_accent: "#F59E0B",
  color_background: "#FFFFFF",
  color_text: "#1F2937",
  color_palette: '["#6366F1", "#8B5CF6", "#F59E0B", "#10B981", "#EF4444"]',
  // 5. TYPOGRAPHY
  font_primary: "Inter",
  font_secondary: "Poppins",
  font_mono: "JetBrains Mono",
  typography_scale: '[{"name": "h1", "size": "3rem", "weight": "700"}, {"name": "body", "size": "1rem", "weight": "400"}]',
  // 6. VISUAL STYLE
  style_keywords: '["modern", "clean", "professional", "tech-forward"]',
  style_mood: "Professional yet approachable, tech-savvy but accessible",
  style_influences: '["Apple", "Stripe", "Linear"]',
  image_style: "Clean product shots, abstract QR patterns, tech illustrations",
  image_do: '["Use gradient overlays", "Show QR codes in context", "Modern devices"]',
  image_dont: '["Stock photos with obvious poses", "Cluttered backgrounds", "Outdated devices"]',
  // 7. UI PATTERNS
  border_radius: "0.75rem",
  shadow_style: "soft",
  animation_style: "smooth",
  logo_usage_rules: '["Min size 32px", "Clear space equal to height", "No rotation"]',
  // 8. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT L10N (localized identity + CTAs + SEO + target_audience) - v7.2.5
// ═══════════════════════════════════════════════════════════════════════════════

// ProjectL10n: en-US
MATCH (p:Project {key: "project-qrcode-ai"}), (l:Locale {key: "en-US"})
CREATE (p)-[:HAS_L10N]->(pl:ProjectL10n {
  // 1. IDENTIFICATION
  display_name: "QR Code AI",
  // 2. DOCUMENTATION
  description: "English localization for QR Code AI project identity",
  llm_context: "USE: native English brand messaging. TRIGGERS: en-US project content. NOT: translation.",
  // 3. IDENTITY
  what_short: "AI-powered QR code generator",
  what_medium: "Create customizable QR codes with built-in analytics and AI design suggestions",
  what_long: "QR Code AI is a comprehensive platform for creating, managing, and tracking QR codes. Generate custom designs, track scans in real-time, and leverage AI to optimize your codes for maximum engagement.",
  // 5. MESSAGING
  tagline: "Smart QR Codes, Smarter Marketing",
  pitch_one_liner: "Create professional QR codes in seconds with AI-powered design and analytics.",
  pitch_elevator: "QR Code AI helps businesses create professional, trackable QR codes instantly. Our AI suggests optimal designs while real-time analytics show you exactly how your codes perform.",
  pitch_detailed: "QR Code AI transforms how businesses use QR codes. Create stunning, customized codes in seconds with AI-powered design suggestions. Track every scan with detailed analytics including location, device, and time data. Perfect for marketing campaigns, business cards, menus, and more.",
  // 6. VOICE
  voice_personality: '["Professional", "Helpful", "Tech-savvy"]',
  voice_tone: '{"formal_casual": 0.4, "serious_playful": 0.3, "technical_simple": 0.4, "reserved_enthusiastic": 0.5}',
  voice_do: '["Be clear and direct", "Use action verbs", "Highlight benefits"]',
  voice_dont: '["Use jargon", "Be overly casual", "Make false promises"]',
  // 7. CTAs
  cta_primary: "Create Free QR Code",
  cta_secondary: "See Pricing",
  cta_tertiary: "View Demo",
  // 8. SEO
  meta_description: "Create free QR codes with AI-powered design and analytics. Professional QR code generator for business cards, marketing, menus and more.",
  primary_keywords: '["qr code generator", "free qr code", "qr code maker"]',
  secondary_keywords: '["dynamic qr code", "qr code analytics", "custom qr code"]',
  // 9. TARGET AUDIENCE (merged from Audience in v7.2.5)
  target_audience: '[{"type": "primary", "title": "Small Business Owner", "pain_points": ["Limited marketing budget", "Need professional tools", "Time constraints"], "goals": ["Grow customer base", "Track marketing ROI", "Look professional"]}, {"type": "secondary", "title": "Marketing Professional", "pain_points": ["Need detailed analytics", "Campaign tracking complexity"], "goals": ["Measure campaign performance", "Optimize conversion rates"]}]',
  // 10. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (pl)-[:FOR_LOCALE]->(l);

// ProjectL10n: fr-FR
MATCH (p:Project {key: "project-qrcode-ai"}), (l:Locale {key: "fr-FR"})
CREATE (p)-[:HAS_L10N]->(pl:ProjectL10n {
  display_name: "QR Code AI",
  description: "Localisation française de l'identité QR Code AI",
  llm_context: "USE: messaging de marque français natif. TRIGGERS: fr-FR contenu projet. NOT: traduction.",
  what_short: "Générateur de QR codes propulsé par l'IA",
  what_medium: "Créez des QR codes personnalisables avec analytics intégrées et suggestions de design IA",
  what_long: "QR Code AI est une plateforme complète pour créer, gérer et suivre vos QR codes. Générez des designs sur mesure, suivez les scans en temps réel, et utilisez l'IA pour optimiser l'engagement.",
  tagline: "QR Codes intelligents, marketing plus malin",
  pitch_one_liner: "Créez des QR codes professionnels en quelques secondes avec design IA et analytics.",
  pitch_elevator: "QR Code AI aide les entreprises à créer des QR codes professionnels et traçables instantanément. Notre IA suggère des designs optimaux tandis que les analytics temps réel montrent la performance.",
  pitch_detailed: "QR Code AI transforme l'utilisation des QR codes en entreprise. Créez des codes personnalisés en quelques secondes avec les suggestions de design IA. Suivez chaque scan avec des analytics détaillées. Parfait pour les campagnes marketing, cartes de visite, menus et plus.",
  voice_personality: '["Professionnel", "Utile", "Tech-savvy"]',
  voice_tone: '{"formal_casual": 0.5, "serious_playful": 0.3, "technical_simple": 0.4, "reserved_enthusiastic": 0.4}',
  voice_do: '["Être clair et direct", "Utiliser des verbes d\'action", "Mettre en avant les bénéfices"]',
  voice_dont: '["Utiliser du jargon", "Être trop familier", "Faire de fausses promesses"]',
  cta_primary: "Créer un QR Code gratuit",
  cta_secondary: "Voir les tarifs",
  cta_tertiary: "Voir la démo",
  meta_description: "Créez des QR codes gratuits avec design IA et analytics. Générateur de QR codes professionnel pour cartes de visite, marketing, menus et plus.",
  primary_keywords: '["générateur qr code", "qr code gratuit", "créer qr code"]',
  secondary_keywords: '["qr code dynamique", "qr code analytics", "qr code personnalisé"]',
  target_audience: '[{"type": "primary", "title": "Propriétaire de PME", "pain_points": ["Budget marketing limité", "Besoin d\'outils professionnels", "Contraintes de temps"], "goals": ["Développer la clientèle", "Suivre le ROI marketing", "Image professionnelle"]}, {"type": "secondary", "title": "Professionnel Marketing", "pain_points": ["Besoin d\'analytics détaillées", "Complexité du suivi de campagne"], "goals": ["Mesurer la performance", "Optimiser les conversions"]}]',
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (pl)-[:FOR_LOCALE]->(l);
