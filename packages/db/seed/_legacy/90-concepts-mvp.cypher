// NovaNet MVP Seed v10.4.0 - Entity-Centric Architecture
//
// v10.4.0 STANDARD PROPERTIES (all nodes):
//   key, display_name, description, llm_context, created_at, updated_at
//
// v10.4: Entity replaces Concept, 2 realms (global, project), GEO layer removed
//
// KEY NAMING: {semantic-role}-{identifier}
//   action-*, product-*, feature-*, tier-*, page-*, block-*, blocktype-*, project-*
//
// LLM_CONTEXT FORMAT: "USE: [when/why]. TRIGGERS: [keywords]. NOT: [disambiguation]."

// ═══════════════════════════════════════════════════════════════════════════════
// LOCALE NODES (v8.2.0 - YAML-aligned Property Order)
// ═══════════════════════════════════════════════════════════════════════════════
//
// PROPERTY ORDER (YAML v7.11.0):
//   1. IDENTIFICATION     → key, display_name
//   2. DOCUMENTATION      → description, llm_context
//   3. LOCALE-SPECIFIC    → language_code, country_code, name_native, is_primary
//   4. LEGACY             → currency, timezone, date_format, number_format, rtl (canonical: LocaleRulesFormatting)
//   5. TIMESTAMPS         → created_at, updated_at
//
// ───────────────────────────────────────────────────────────────────────────────

// en-US
CREATE (:Locale {
  // 1. IDENTIFICATION
  key: "en-US",
  display_name: "English (US)",
  // 2. DOCUMENTATION
  description: "American English locale for United States market",
  llm_context: "USE: primary English locale, default fallback. TRIGGERS: english, US, american, en. NOT: British English (use en-GB).",
  // 3. LOCALE-SPECIFIC (YAML v7.11.0)
  language_code: "en",
  country_code: "US",
  name_native: "English (United States)",
  is_primary: true,
  // 4. LEGACY (kept for compatibility, canonical location: LocaleRulesFormatting)
  currency: "USD",
  timezone: "America/New_York",
  date_format: "MM/DD/YYYY",
  number_format: "1,234.56",
  rtl: false,
  // 5. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR
CREATE (:Locale {
  key: "fr-FR",
  display_name: "French (France)",
  description: "Metropolitan French locale for France market",
  llm_context: "USE: French content for France. TRIGGERS: french, français, france, fr-FR. NOT: Canadian French (use fr-CA).",
  language_code: "fr",
  country_code: "FR",
  name_native: "Français (France)",
  is_primary: true,
  // LEGACY (canonical location: LocaleRulesFormatting)
  currency: "EUR",
  timezone: "Europe/Paris",
  date_format: "DD/MM/YYYY",
  number_format: "1 234,56",
  rtl: false,
  created_at: datetime(),
  updated_at: datetime()
});

// fr-CA
CREATE (:Locale {
  key: "fr-CA",
  display_name: "French (Canada)",
  description: "Canadian French locale for Quebec market",
  llm_context: "USE: French content for Canada/Quebec. TRIGGERS: quebec, canadien, fr-CA. NOT: France French (use fr-FR).",
  language_code: "fr",
  country_code: "CA",
  name_native: "Français (Canada)",
  is_primary: false,
  // LEGACY (canonical location: LocaleRulesFormatting)
  currency: "CAD",
  timezone: "America/Montreal",
  date_format: "YYYY-MM-DD",
  number_format: "1 234,56",
  rtl: false,
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES
CREATE (:Locale {
  key: "es-ES",
  display_name: "Spanish (Spain)",
  description: "Castilian Spanish locale for Spain market",
  llm_context: "USE: Spanish content for Spain. TRIGGERS: spanish, español, spain, es-ES. NOT: Latin American Spanish.",
  language_code: "es",
  country_code: "ES",
  name_native: "Español (España)",
  is_primary: true,
  // LEGACY (canonical location: LocaleRulesFormatting)
  currency: "EUR",
  timezone: "Europe/Madrid",
  date_format: "DD/MM/YYYY",
  number_format: "1.234,56",
  rtl: false,
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE
CREATE (:Locale {
  key: "de-DE",
  display_name: "German (Germany)",
  description: "Standard German locale for Germany market",
  llm_context: "USE: German content for Germany. TRIGGERS: german, deutsch, de-DE. NOT: Austrian or Swiss German.",
  language_code: "de",
  country_code: "DE",
  name_native: "Deutsch (Deutschland)",
  is_primary: true,
  // LEGACY (canonical location: LocaleRulesFormatting)
  currency: "EUR",
  timezone: "Europe/Berlin",
  date_format: "DD.MM.YYYY",
  number_format: "1.234,56",
  rtl: false,
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP
CREATE (:Locale {
  key: "ja-JP",
  display_name: "Japanese (Japan)",
  description: "Standard Japanese locale for Japan market",
  llm_context: "USE: Japanese content for Japan. TRIGGERS: japanese, 日本語, ja-JP. NOT: other Asian languages.",
  language_code: "ja",
  country_code: "JP",
  name_native: "日本語（日本）",
  is_primary: true,
  // LEGACY (canonical location: LocaleRulesFormatting)
  currency: "JPY",
  timezone: "Asia/Tokyo",
  date_format: "YYYY/MM/DD",
  number_format: "1,234",
  rtl: false,
  created_at: datetime(),
  updated_at: datetime()
});

// Fallback chains (fr-CA → fr-FR → en-US)
MATCH (frCA:Locale {key: "fr-CA"}), (frFR:Locale {key: "fr-FR"})
CREATE (frCA)-[:FALLBACK_TO]->(frFR);

MATCH (frFR:Locale {key: "fr-FR"}), (enUS:Locale {key: "en-US"})
CREATE (frFR)-[:FALLBACK_TO]->(enUS);

MATCH (esES:Locale {key: "es-ES"}), (enUS:Locale {key: "en-US"})
CREATE (esES)-[:FALLBACK_TO]->(enUS);

MATCH (deDE:Locale {key: "de-DE"}), (enUS:Locale {key: "en-US"})
CREATE (deDE)-[:FALLBACK_TO]->(enUS);

MATCH (jaJP:Locale {key: "ja-JP"}), (enUS:Locale {key: "en-US"})
CREATE (jaJP)-[:FALLBACK_TO]->(enUS);

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT (v8.2.0 - YAML-aligned Property Order)
// ═══════════════════════════════════════════════════════════════════════════════
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     → key, display_name
//   2. DOCUMENTATION      → description, llm_context
//   3. PROJECT-SPECIFIC   → brand_name, website_url
//   4. TIMESTAMPS         → created_at, updated_at
//
// ───────────────────────────────────────────────────────────────────────────────

CREATE (p:Project {
  // 1. IDENTIFICATION
  key: "project-qrcode-ai",
  display_name: "QR Code AI",
  // 2. DOCUMENTATION
  description: "Multilingual QR code generation SaaS platform",
  llm_context: "USE: root context for QR Code AI project. TRIGGERS: qrcode, qr-code, qrcode-ai. NOT: other projects.",
  // 3. PROJECT-SPECIFIC
  brand_name: "QR Code AI",
  website_url: "https://qrcode-ai.com",
  // 4. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// Project supports locales (v7.1.0: uses l.key)
MATCH (p:Project {key: "project-qrcode-ai"}), (l:Locale)
WHERE l.key IN ["en-US", "fr-FR", "fr-CA", "es-ES", "de-DE", "ja-JP"]
CREATE (p)-[:SUPPORTS_LOCALE {default: l.key = "en-US"}]->(l);

// ═══════════════════════════════════════════════════════════════════════════════
// L10N CATEGORIES - DEPRECATED
// Replaced by Locale Knowledge nodes: LocaleIdentity, LocaleVoice, LocaleCulture,
// LocaleMarket, LocaleLexicon (see locale-knowledge migration plan)
// ═══════════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════════
// ENTITIES (v8.2.0 - YAML-aligned Property Order)
// ═══════════════════════════════════════════════════════════════════════════════
//
// KEY NAMING: {semantic-role}-{identifier}
//   action-*   → verbs/actions
//   product-*  → tools/products
//   feature-*  → features
//   tier-*     → pricing tiers
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     → key, display_name
//   2. DOCUMENTATION      → description, llm_context
//   3. TIMESTAMPS         → created_at, updated_at
//
// ───────────────────────────────────────────────────────────────────────────────

// action-create-qr
MATCH (p:Project {key: "project-qrcode-ai"})
CREATE (p)-[:USES_ENTITY]->(c:Entity {
  // 1. IDENTIFICATION
  key: "action-create-qr",
  display_name: "Create QR Code",
  // 2. DOCUMENTATION
  description: "Action of creating a new QR code from input data",
  llm_context: "USE: when user wants to create/generate a new QR code. TRIGGERS: create, generate, make, créer, erstellen, 作成. NOT: viewing or editing existing codes.",
  // 3. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// product-generator
MATCH (p:Project {key: "project-qrcode-ai"})
CREATE (p)-[:USES_ENTITY]->(c:Entity {
  key: "product-generator",
  display_name: "QR Code Generator",
  description: "The main QR code generator tool/product",
  llm_context: "USE: when referring to the generator tool itself. TRIGGERS: generator, tool, maker, outil, générateur. NOT: the action of creating (use action-create-qr).",
  created_at: datetime(),
  updated_at: datetime()
});

// feature-analytics
MATCH (p:Project {key: "project-qrcode-ai"})
CREATE (p)-[:USES_ENTITY]->(c:Entity {
  key: "feature-analytics",
  display_name: "Analytics",
  description: "Tracking and statistics feature for QR code scans",
  llm_context: "USE: when discussing tracking, statistics, or scan data. TRIGGERS: analytics, track, stats, scans, statistiques, données. NOT: pricing or tier features.",
  created_at: datetime(),
  updated_at: datetime()
});

// tier-free
MATCH (p:Project {key: "project-qrcode-ai"})
CREATE (p)-[:USES_ENTITY]->(c:Entity {
  key: "tier-free",
  display_name: "Free Tier",
  description: "Free pricing tier with basic features",
  llm_context: "USE: when discussing free plan or no-cost option. TRIGGERS: free, gratuit, gratis, kostenlos, 無料. NOT: paid tiers (use tier-pro).",
  created_at: datetime(),
  updated_at: datetime()
});

// tier-pro
MATCH (p:Project {key: "project-qrcode-ai"})
CREATE (p)-[:USES_ENTITY]->(c:Entity {
  key: "tier-pro",
  display_name: "Pro Tier",
  description: "Premium pricing tier with advanced features",
  llm_context: "USE: when discussing paid/premium plan. TRIGGERS: pro, premium, paid, professional, payant. NOT: free tier (use tier-free).",
  created_at: datetime(),
  updated_at: datetime()
});

// ═══════════════════════════════════════════════════════════════════════════════
// ENTITY L10N (v10.4.0 - Entity-Centric Architecture)
// ═══════════════════════════════════════════════════════════════════════════════
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     → display_name
//   2. DOCUMENTATION      → description, llm_context
//   3. L10N-SPECIFIC      → title, definition, purpose, benefits, use_cases
//   4. VERSIONING         → version, influence_count
//   5. TIMESTAMPS         → created_at, updated_at
//
// ───────────────────────────────────────────────────────────────────────────────

// EntityL10n for action-create-qr: en-US
MATCH (c:Entity {key: "action-create-qr"}), (l:Locale {key: "en-US"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  // 1. IDENTIFICATION
  display_name: "Create a QR Code",
  // 2. DOCUMENTATION
  description: "English localization for action-create-qr concept",
  llm_context: "USE: native English title/definition for generation. TRIGGERS: en-US content needed. NOT: translation source.",
  // 3. L10N-SPECIFIC
  title: "Create a QR Code",
  definition: "Generate a QR code from URL, text, or contact info.",
  purpose: "Convert info to scannable format.",
  benefits: ["Fast", "Free", "No signup required"],
  use_cases: ["Marketing", "Business cards", "Event tickets"],
  // 4. VERSIONING
  version: 1,
  influence_count: 0,
  // 5. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for action-create-qr: fr-FR
MATCH (c:Entity {key: "action-create-qr"}), (l:Locale {key: "fr-FR"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Créer un QR Code",
  description: "French (France) localization for action-create-qr concept",
  llm_context: "USE: native French title/definition for generation. TRIGGERS: fr-FR content needed. NOT: Quebec French.",
  title: "Créer un QR Code",
  definition: "Générez un QR code à partir d'une URL, texte ou contact.",
  purpose: "Convertir des informations en format scannable.",
  benefits: ["Rapide", "Gratuit", "Sans inscription"],
  use_cases: ["Marketing", "Cartes de visite", "Billets événements"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for action-create-qr: es-ES
MATCH (c:Entity {key: "action-create-qr"}), (l:Locale {key: "es-ES"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Crear un código QR",
  description: "Spanish (Spain) localization for action-create-qr concept",
  llm_context: "USE: native Spanish title/definition for generation. TRIGGERS: es-ES content needed. NOT: Latin American Spanish.",
  title: "Crear un código QR",
  definition: "Genera un código QR desde URL, texto o contacto.",
  purpose: "Convertir información en formato escaneable.",
  benefits: ["Rápido", "Gratis", "Sin registro"],
  use_cases: ["Marketing", "Tarjetas de visita", "Entradas eventos"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for action-create-qr: de-DE
MATCH (c:Entity {key: "action-create-qr"}), (l:Locale {key: "de-DE"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "QR-Code erstellen",
  description: "German localization for action-create-qr concept",
  llm_context: "USE: native German title/definition for generation. TRIGGERS: de-DE content needed. NOT: Austrian German.",
  title: "QR-Code erstellen",
  definition: "Erstellen Sie einen QR-Code aus URL, Text oder Kontaktdaten.",
  purpose: "Informationen in scannbares Format umwandeln.",
  benefits: ["Schnell", "Kostenlos", "Ohne Registrierung"],
  use_cases: ["Marketing", "Visitenkarten", "Eventtickets"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for action-create-qr: ja-JP
MATCH (c:Entity {key: "action-create-qr"}), (l:Locale {key: "ja-JP"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "QRコードを作成",
  description: "Japanese localization for action-create-qr concept",
  llm_context: "USE: native Japanese title/definition for generation. TRIGGERS: ja-JP content needed. NOT: other Asian languages.",
  title: "QRコードを作成",
  definition: "URL、テキスト、連絡先からQRコードを生成します。",
  purpose: "情報をスキャン可能な形式に変換。",
  benefits: ["高速", "無料", "登録不要"],
  use_cases: ["マーケティング", "名刺", "イベントチケット"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// ───────────────────────────────────────────────────────────────────────────────
// EntityL10n for product-generator (5 locales)
// ───────────────────────────────────────────────────────────────────────────────

// EntityL10n for product-generator: en-US
MATCH (c:Entity {key: "product-generator"}), (l:Locale {key: "en-US"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "QR Code Generator",
  description: "The QR code generation tool",
  llm_context: "USE: native English title/definition for generation. TRIGGERS: en-US content needed. NOT: translation source.",
  title: "QR Code Generator",
  definition: "A powerful tool that creates customizable QR codes from any URL, text, or contact information.",
  benefits: ["Fast generation", "Customizable design", "Multiple formats"],
  use_cases: ["Marketing campaigns", "Business cards", "Product packaging"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for product-generator: fr-FR
MATCH (c:Entity {key: "product-generator"}), (l:Locale {key: "fr-FR"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Générateur de QR Code",
  description: "L'outil de génération de QR codes",
  llm_context: "USE: native French title/definition for generation. TRIGGERS: fr-FR content needed. NOT: Quebec French.",
  title: "Générateur de QR Code",
  definition: "Un outil puissant pour créer des QR codes personnalisables à partir de n'importe quelle URL, texte ou contact.",
  benefits: ["Génération rapide", "Design personnalisable", "Multiples formats"],
  use_cases: ["Campagnes marketing", "Cartes de visite", "Emballages produits"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for product-generator: es-ES
MATCH (c:Entity {key: "product-generator"}), (l:Locale {key: "es-ES"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Generador de códigos QR",
  description: "La herramienta de generación de códigos QR",
  llm_context: "USE: native Spanish title/definition for generation. TRIGGERS: es-ES content needed. NOT: Latin American Spanish.",
  title: "Generador de códigos QR",
  definition: "Una potente herramienta que crea códigos QR personalizables desde cualquier URL, texto o información de contacto.",
  benefits: ["Generación rápida", "Diseño personalizable", "Múltiples formatos"],
  use_cases: ["Campañas de marketing", "Tarjetas de visita", "Embalaje de productos"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for product-generator: de-DE
MATCH (c:Entity {key: "product-generator"}), (l:Locale {key: "de-DE"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "QR-Code-Generator",
  description: "Das QR-Code-Erstellungswerkzeug",
  llm_context: "USE: native German title/definition for generation. TRIGGERS: de-DE content needed. NOT: Austrian German.",
  title: "QR-Code-Generator",
  definition: "Ein leistungsstarkes Tool zur Erstellung anpassbarer QR-Codes aus beliebigen URLs, Texten oder Kontaktdaten.",
  benefits: ["Schnelle Erstellung", "Anpassbares Design", "Mehrere Formate"],
  use_cases: ["Marketingkampagnen", "Visitenkarten", "Produktverpackungen"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for product-generator: ja-JP
MATCH (c:Entity {key: "product-generator"}), (l:Locale {key: "ja-JP"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "QRコードジェネレーター",
  description: "QRコード生成ツール",
  llm_context: "USE: native Japanese title/definition for generation. TRIGGERS: ja-JP content needed. NOT: other Asian languages.",
  title: "QRコードジェネレーター",
  definition: "URLやテキスト、連絡先情報からカスタマイズ可能なQRコードを作成できる高性能ツール。",
  benefits: ["高速生成", "カスタマイズ可能", "複数フォーマット"],
  use_cases: ["マーケティング", "名刺", "製品パッケージ"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// ───────────────────────────────────────────────────────────────────────────────
// EntityL10n for feature-analytics (5 locales)
// ───────────────────────────────────────────────────────────────────────────────

// EntityL10n for feature-analytics: en-US
MATCH (c:Entity {key: "feature-analytics"}), (l:Locale {key: "en-US"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Analytics",
  description: "Scan tracking and statistics feature",
  llm_context: "USE: native English title/definition for generation. TRIGGERS: en-US content needed. NOT: translation source.",
  title: "Analytics & Tracking",
  definition: "Track every scan of your QR codes with detailed statistics including location, device type, and time.",
  benefits: ["Real-time tracking", "Detailed reports", "Geographic insights"],
  use_cases: ["Campaign ROI", "User behavior", "Performance optimization"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for feature-analytics: fr-FR
MATCH (c:Entity {key: "feature-analytics"}), (l:Locale {key: "fr-FR"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Analytiques",
  description: "Suivi des scans et statistiques",
  llm_context: "USE: native French title/definition for generation. TRIGGERS: fr-FR content needed. NOT: Quebec French.",
  title: "Analytiques & Suivi",
  definition: "Suivez chaque scan de vos QR codes avec des statistiques détaillées incluant la localisation, le type d'appareil et l'heure.",
  benefits: ["Suivi en temps réel", "Rapports détaillés", "Insights géographiques"],
  use_cases: ["ROI campagnes", "Comportement utilisateur", "Optimisation"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for feature-analytics: es-ES
MATCH (c:Entity {key: "feature-analytics"}), (l:Locale {key: "es-ES"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Analíticas",
  description: "Seguimiento de escaneos y estadísticas",
  llm_context: "USE: native Spanish title/definition for generation. TRIGGERS: es-ES content needed. NOT: Latin American Spanish.",
  title: "Analíticas y Seguimiento",
  definition: "Rastrea cada escaneo de tus códigos QR con estadísticas detalladas incluyendo ubicación, tipo de dispositivo y hora.",
  benefits: ["Seguimiento en tiempo real", "Informes detallados", "Insights geográficos"],
  use_cases: ["ROI de campañas", "Comportamiento usuario", "Optimización"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for feature-analytics: de-DE
MATCH (c:Entity {key: "feature-analytics"}), (l:Locale {key: "de-DE"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Analysen",
  description: "Scan-Tracking und Statistiken",
  llm_context: "USE: native German title/definition for generation. TRIGGERS: de-DE content needed. NOT: Austrian German.",
  title: "Analysen & Tracking",
  definition: "Verfolgen Sie jeden Scan Ihrer QR-Codes mit detaillierten Statistiken zu Standort, Gerätetyp und Uhrzeit.",
  benefits: ["Echtzeit-Tracking", "Detaillierte Berichte", "Geografische Einblicke"],
  use_cases: ["Kampagnen-ROI", "Nutzerverhalten", "Leistungsoptimierung"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for feature-analytics: ja-JP
MATCH (c:Entity {key: "feature-analytics"}), (l:Locale {key: "ja-JP"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "アナリティクス",
  description: "スキャン追跡と統計",
  llm_context: "USE: native Japanese title/definition for generation. TRIGGERS: ja-JP content needed. NOT: other Asian languages.",
  title: "アナリティクス＆トラッキング",
  definition: "QRコードのすべてのスキャンを、場所、デバイスタイプ、時間などの詳細な統計で追跡。",
  benefits: ["リアルタイム追跡", "詳細レポート", "地理的インサイト"],
  use_cases: ["キャンペーンROI", "ユーザー行動", "パフォーマンス最適化"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// ───────────────────────────────────────────────────────────────────────────────
// EntityL10n for tier-free (5 locales)
// ───────────────────────────────────────────────────────────────────────────────

// EntityL10n for tier-free: en-US
MATCH (c:Entity {key: "tier-free"}), (l:Locale {key: "en-US"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Free Plan",
  description: "Free tier pricing plan",
  llm_context: "USE: native English title/definition for generation. TRIGGERS: en-US content needed. NOT: translation source.",
  title: "Free Plan",
  definition: "Get started with our free plan. Create unlimited static QR codes with basic customization.",
  benefits: ["No credit card required", "Unlimited static QR codes", "Basic customization"],
  use_cases: ["Personal use", "Testing", "Small projects"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for tier-free: fr-FR
MATCH (c:Entity {key: "tier-free"}), (l:Locale {key: "fr-FR"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Offre Gratuite",
  description: "Plan tarifaire gratuit",
  llm_context: "USE: native French title/definition for generation. TRIGGERS: fr-FR content needed. NOT: Quebec French.",
  title: "Offre Gratuite",
  definition: "Commencez gratuitement. Créez des QR codes statiques illimités avec personnalisation de base.",
  benefits: ["Sans carte bancaire", "QR codes statiques illimités", "Personnalisation basique"],
  use_cases: ["Usage personnel", "Tests", "Petits projets"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for tier-free: es-ES
MATCH (c:Entity {key: "tier-free"}), (l:Locale {key: "es-ES"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Plan Gratuito",
  description: "Plan de precios gratuito",
  llm_context: "USE: native Spanish title/definition for generation. TRIGGERS: es-ES content needed. NOT: Latin American Spanish.",
  title: "Plan Gratuito",
  definition: "Empieza gratis. Crea códigos QR estáticos ilimitados con personalización básica.",
  benefits: ["Sin tarjeta de crédito", "QR estáticos ilimitados", "Personalización básica"],
  use_cases: ["Uso personal", "Pruebas", "Proyectos pequeños"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for tier-free: de-DE
MATCH (c:Entity {key: "tier-free"}), (l:Locale {key: "de-DE"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Kostenloser Plan",
  description: "Kostenloses Tarifpaket",
  llm_context: "USE: native German title/definition for generation. TRIGGERS: de-DE content needed. NOT: Austrian German.",
  title: "Kostenloser Plan",
  definition: "Starten Sie kostenlos. Erstellen Sie unbegrenzt statische QR-Codes mit Basisanpassung.",
  benefits: ["Keine Kreditkarte nötig", "Unbegrenzt statische QR-Codes", "Basisanpassung"],
  use_cases: ["Persönliche Nutzung", "Tests", "Kleine Projekte"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for tier-free: ja-JP
MATCH (c:Entity {key: "tier-free"}), (l:Locale {key: "ja-JP"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "無料プラン",
  description: "無料料金プラン",
  llm_context: "USE: native Japanese title/definition for generation. TRIGGERS: ja-JP content needed. NOT: other Asian languages.",
  title: "無料プラン",
  definition: "無料で始めましょう。基本的なカスタマイズ付きの静的QRコードを無制限に作成できます。",
  benefits: ["クレジットカード不要", "静的QRコード無制限", "基本カスタマイズ"],
  use_cases: ["個人利用", "テスト", "小規模プロジェクト"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// ───────────────────────────────────────────────────────────────────────────────
// EntityL10n for tier-pro (5 locales)
// ───────────────────────────────────────────────────────────────────────────────

// EntityL10n for tier-pro: en-US
MATCH (c:Entity {key: "tier-pro"}), (l:Locale {key: "en-US"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Pro Plan",
  description: "Professional tier pricing plan",
  llm_context: "USE: native English title/definition for generation. TRIGGERS: en-US content needed. NOT: translation source.",
  title: "Pro Plan",
  definition: "Unlock the full power of QR codes. Dynamic QR codes, advanced analytics, custom branding, and priority support.",
  benefits: ["Dynamic QR codes", "Advanced analytics", "Custom branding", "Priority support"],
  use_cases: ["Business", "Marketing teams", "Enterprise"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for tier-pro: fr-FR
MATCH (c:Entity {key: "tier-pro"}), (l:Locale {key: "fr-FR"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Offre Pro",
  description: "Plan tarifaire professionnel",
  llm_context: "USE: native French title/definition for generation. TRIGGERS: fr-FR content needed. NOT: Quebec French.",
  title: "Offre Pro",
  definition: "Débloquez toute la puissance des QR codes. QR codes dynamiques, analytics avancés, branding personnalisé et support prioritaire.",
  benefits: ["QR codes dynamiques", "Analytics avancés", "Branding personnalisé", "Support prioritaire"],
  use_cases: ["Entreprises", "Équipes marketing", "Grands comptes"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for tier-pro: es-ES
MATCH (c:Entity {key: "tier-pro"}), (l:Locale {key: "es-ES"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Plan Pro",
  description: "Plan de precios profesional",
  llm_context: "USE: native Spanish title/definition for generation. TRIGGERS: es-ES content needed. NOT: Latin American Spanish.",
  title: "Plan Pro",
  definition: "Desbloquea todo el potencial de los códigos QR. Códigos dinámicos, analíticas avanzadas, marca personalizada y soporte prioritario.",
  benefits: ["Códigos QR dinámicos", "Analíticas avanzadas", "Marca personalizada", "Soporte prioritario"],
  use_cases: ["Empresas", "Equipos de marketing", "Enterprise"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for tier-pro: de-DE
MATCH (c:Entity {key: "tier-pro"}), (l:Locale {key: "de-DE"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "Pro-Plan",
  description: "Professionelles Tarifpaket",
  llm_context: "USE: native German title/definition for generation. TRIGGERS: de-DE content needed. NOT: Austrian German.",
  title: "Pro-Plan",
  definition: "Nutzen Sie die volle Leistung von QR-Codes. Dynamische QR-Codes, erweiterte Analysen, individuelles Branding und Prioritäts-Support.",
  benefits: ["Dynamische QR-Codes", "Erweiterte Analysen", "Individuelles Branding", "Prioritäts-Support"],
  use_cases: ["Unternehmen", "Marketing-Teams", "Enterprise"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// EntityL10n for tier-pro: ja-JP
MATCH (c:Entity {key: "tier-pro"}), (l:Locale {key: "ja-JP"})
CREATE (c)-[:HAS_L10N]->(cl:EntityL10n {
  display_name: "プロプラン",
  description: "プロフェッショナル料金プラン",
  llm_context: "USE: native Japanese title/definition for generation. TRIGGERS: ja-JP content needed. NOT: other Asian languages.",
  title: "プロプラン",
  definition: "QRコードの全機能を解放。ダイナミックQRコード、高度なアナリティクス、カスタムブランディング、優先サポート。",
  benefits: ["ダイナミックQRコード", "高度なアナリティクス", "カスタムブランディング", "優先サポート"],
  use_cases: ["ビジネス", "マーケティングチーム", "エンタープライズ"],
  version: 1,
  influence_count: 0,
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (cl)-[:FOR_LOCALE]->(l);

// Semantic links (v7.1.0: updated to new key names)
MATCH (c1:Entity {key: "action-create-qr"}), (c2:Entity {key: "product-generator"})
CREATE (c1)-[:SEMANTIC_LINK {type: "is_action_on", temperature: 0.95}]->(c2);

MATCH (c1:Entity {key: "product-generator"}), (c2:Entity {key: "feature-analytics"})
CREATE (c1)-[:SEMANTIC_LINK {type: "includes", temperature: 0.80}]->(c2);

MATCH (c1:Entity {key: "tier-pro"}), (c2:Entity {key: "feature-analytics"})
CREATE (c1)-[:SEMANTIC_LINK {type: "includes", temperature: 0.90}]->(c2);

// ═══════════════════════════════════════════════════════════════════════════════
// L10N CONTENT - DEPRECATED
// Replaced by Locale Knowledge nodes with rich structured data
// Voice/culture info now in LocaleVoice and LocaleCulture nodes
// ═══════════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE (v8.2.0 - YAML-aligned Property Order)
// ═══════════════════════════════════════════════════════════════════════════════
//
// KEY NAMING: page-{identifier}
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     → key, display_name
//   2. DOCUMENTATION      → description, llm_context
//   3. PAGE-SPECIFIC      → instructions
//   4. TIMESTAMPS         → created_at, updated_at
//
// ───────────────────────────────────────────────────────────────────────────────

// page-pricing
MATCH (p:Project {key: "project-qrcode-ai"})
CREATE (p)-[:HAS_PAGE]->(page:Page {
  // 1. IDENTIFICATION
  key: "page-pricing",
  display_name: "Pricing Page",
  // 2. DOCUMENTATION
  description: "Main pricing page comparing free and pro tiers",
  llm_context: "USE: when generating pricing/plans content. TRIGGERS: pricing, tarifs, plans, abonnement, preise. NOT: feature pages or landing pages.",
  // 3. PAGE-SPECIFIC
  instructions: "[GENERATE] Create a conversion-focused pricing page comparing tiers",
  // 4. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// Page uses entities (v7.1.0: USES_ENTITY unified)
MATCH (page:Page {key: "page-pricing"}), (c:Entity)
WHERE c.key IN ["tier-free", "tier-pro"]
CREATE (page)-[:USES_ENTITY {purpose: "primary", temperature: 1.0}]->(c);

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCKTYPE (v8.2.0 - YAML-aligned Property Order)
// ═══════════════════════════════════════════════════════════════════════════════
//
// KEY NAMING: blocktype-{identifier}
//
// PROPERTY ORDER:
//   1. IDENTIFICATION       → key, display_name
//   2. DOCUMENTATION        → description, llm_context
//   3. BLOCKTYPE-SPECIFIC   → category, structure, rules
//   4. TIMESTAMPS           → created_at, updated_at
//
// ───────────────────────────────────────────────────────────────────────────────

// blocktype-hero
CREATE (bt:BlockType {
  // 1. IDENTIFICATION
  key: "blocktype-hero",
  display_name: "Hero Block",
  // 2. DOCUMENTATION
  description: "Hero section template for landing pages with CTA",
  llm_context: "USE: for page headers with strong value proposition. TRIGGERS: hero, header, banner, above-fold. NOT: body content or footers.",
  // 3. BLOCKTYPE-SPECIFIC
  category: "header",
  structure: "schemas/hero.json",
  rules: "Title: 6-10 words, action verb. Subtitle: value prop. CTA: action button.",
  // 4. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// NOTE: REQUIRES_L10N relation removed - BlockType now implicitly uses
// LocaleVoice and LocaleCulture from Locale Knowledge graph

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK (v8.2.0 - YAML-aligned Property Order)
// ═══════════════════════════════════════════════════════════════════════════════
//
// KEY NAMING: block-{page}-{type} or block-{identifier}
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     → key, display_name
//   2. DOCUMENTATION      → description, llm_context
//   3. BLOCK-SPECIFIC     → instructions
//   4. TIMESTAMPS         → created_at, updated_at
//
// ───────────────────────────────────────────────────────────────────────────────

// block-pricing-hero
MATCH (page:Page {key: "page-pricing"})
CREATE (block:Block {
  // 1. IDENTIFICATION
  key: "block-pricing-hero",
  display_name: "Pricing Hero",
  // 2. DOCUMENTATION
  description: "Hero block for pricing page highlighting value proposition",
  llm_context: "USE: for pricing page header. TRIGGERS: pricing hero, pricing header. NOT: other page heroes.",
  // 3. BLOCK-SPECIFIC
  instructions: "[GENERATE] Hero for pricing page, highlight @tier-pro benefits",
  // 4. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (page)-[:HAS_BLOCK {position: 0}]->(block);

// Block uses concept (v7.1.0: USES_ENTITY unified)
MATCH (block:Block {key: "block-pricing-hero"}), (c:Entity {key: "tier-pro"})
CREATE (block)-[:USES_ENTITY {purpose: "primary", temperature: 1.0}]->(c);

// Block of type
MATCH (block:Block {key: "block-pricing-hero"}), (bt:BlockType {key: "blocktype-hero"})
CREATE (block)-[:OF_TYPE]->(bt);
