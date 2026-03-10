// ═══════════════════════════════════════════════════════════════════════════════
// 53-entity-natives-bootstrap.cypher — EntityNative content for 9 pillars
// v0.17.0 - Native content for en-US and fr-FR locales
// ═══════════════════════════════════════════════════════════════════════════════
//
// 9 Pillar Entities:
//   1. qr-code
//   2. custom-qr-code
//   3. qr-code-art
//   4. dynamic-qr-code
//   5. static-qr-code
//   6. smart-link
//   7. landing-page
//   8. barcode
//   9. qr-code-generator
//
// Each gets EntityNative for en-US and fr-FR (18 total)
// Key format: {entity-slug}@{locale} (ADR-029)
//
// denomination_forms stored as separate properties (Neo4j doesn't support Maps)
// ═══════════════════════════════════════════════════════════════════════════════

// =============================================================================
// 1. QR CODE — Core concept
// =============================================================================

// en-US
MERGE (en:EntityNative {key: 'qr-code@en-US'})
ON CREATE SET
  en.display_name = 'QR Code',
  en.description = 'A QR code (Quick Response code) is a two-dimensional barcode that stores information readable by smartphones and scanners.',
  en.locale_key = 'en-US',
  en.entity_key = 'entity:qr-code',
  en.denomination_text = 'QR code',
  en.denomination_title = 'QR Code',
  en.denomination_abbrev = 'QR',
  en.denomination_url = 'qr-code',
  en.llm_context = 'USE: when generating English content about QR codes. TRIGGERS: qr code, qr, barcode, scan, mobile, 2d code. NOT: for specific types (use dynamic-qr-code, static-qr-code).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// fr-FR
MERGE (en:EntityNative {key: 'qr-code@fr-FR'})
ON CREATE SET
  en.display_name = 'Code QR',
  en.description = 'Un code QR (Quick Response) est un code-barres bidimensionnel qui stocke des informations lisibles par smartphones et scanners.',
  en.locale_key = 'fr-FR',
  en.entity_key = 'entity:qr-code',
  en.denomination_text = 'code QR',
  en.denomination_title = 'Code QR',
  en.denomination_abbrev = 'QR',
  en.denomination_url = 'code-qr',
  en.llm_context = 'USE: when generating French content about QR codes. TRIGGERS: code qr, qr code, code matriciel, scanner. NOT: for specific types (use qr-code-dynamique, qr-code-statique).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// =============================================================================
// 2. CUSTOM QR CODE — Branded/styled QR codes
// =============================================================================

// en-US
MERGE (en:EntityNative {key: 'custom-qr-code@en-US'})
ON CREATE SET
  en.display_name = 'Custom QR Code',
  en.description = 'A custom QR code with personalized design elements like colors, logos, and patterns while maintaining scannability.',
  en.locale_key = 'en-US',
  en.entity_key = 'entity:custom-qr-code',
  en.denomination_text = 'custom QR code',
  en.denomination_title = 'Custom QR Code',
  en.denomination_abbrev = 'custom QR',
  en.denomination_url = 'custom-qr-code',
  en.llm_context = 'USE: when generating English content about branded QR codes. TRIGGERS: custom, personalized, branded, logo, colors, design. NOT: for AI art QR codes (use qr-code-art).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// fr-FR
MERGE (en:EntityNative {key: 'custom-qr-code@fr-FR'})
ON CREATE SET
  en.display_name = 'Code QR Personnalisé',
  en.description = 'Un code QR personnalisé avec des éléments de design comme couleurs, logos et motifs tout en restant scannable.',
  en.locale_key = 'fr-FR',
  en.entity_key = 'entity:custom-qr-code',
  en.denomination_text = 'code QR personnalisé',
  en.denomination_title = 'Code QR Personnalisé',
  en.denomination_abbrev = 'QR personnalisé',
  en.denomination_url = 'code-qr-personnalise',
  en.llm_context = 'USE: when generating French content about branded QR codes. TRIGGERS: personnalisé, personnaliser, logo, couleurs, design, marque. NOT: for AI art QR codes (use art-code-qr).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// =============================================================================
// 3. QR CODE ART — Artistic QR codes
// =============================================================================

// en-US
MERGE (en:EntityNative {key: 'qr-code-art@en-US'})
ON CREATE SET
  en.display_name = 'QR Code Art',
  en.description = 'QR code art transforms functional codes into visually stunning designs using AI-generated imagery and artistic styles.',
  en.locale_key = 'en-US',
  en.entity_key = 'entity:qr-code-art',
  en.denomination_text = 'QR code art',
  en.denomination_title = 'QR Code Art',
  en.denomination_abbrev = 'QR art',
  en.denomination_url = 'qr-code-art',
  en.llm_context = 'USE: when generating English content about artistic AI-generated QR codes. TRIGGERS: art, artistic, AI, generative, creative, beautiful, design, image. NOT: for simple color/logo customization (use custom-qr-code).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// fr-FR
MERGE (en:EntityNative {key: 'qr-code-art@fr-FR'})
ON CREATE SET
  en.display_name = 'Art Code QR',
  en.description = 'L\'art code QR transforme les codes fonctionnels en designs visuellement époustouflants grâce à l\'IA et des styles artistiques.',
  en.locale_key = 'fr-FR',
  en.entity_key = 'entity:qr-code-art',
  en.denomination_text = 'art code QR',
  en.denomination_title = 'Art Code QR',
  en.denomination_abbrev = 'art QR',
  en.denomination_url = 'art-code-qr',
  en.llm_context = 'USE: when generating French content about artistic AI-generated QR codes. TRIGGERS: art, artistique, IA, génératif, créatif, beau, design, image. NOT: for simple customization (use code-qr-personnalise).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// =============================================================================
// 4. DYNAMIC QR CODE — Editable destination
// =============================================================================

// en-US
MERGE (en:EntityNative {key: 'dynamic-qr-code@en-US'})
ON CREATE SET
  en.display_name = 'Dynamic QR Code',
  en.description = 'A dynamic QR code allows you to change the destination URL anytime without reprinting the code, plus track scans and analytics.',
  en.locale_key = 'en-US',
  en.entity_key = 'entity:dynamic-qr-code',
  en.denomination_text = 'dynamic QR code',
  en.denomination_title = 'Dynamic QR Code',
  en.denomination_abbrev = 'dynamic QR',
  en.denomination_url = 'dynamic-qr-code',
  en.llm_context = 'USE: when generating English content about editable trackable QR codes. TRIGGERS: dynamic, editable, changeable, trackable, analytics, statistics, update URL. NOT: for permanent fixed QR codes (use static-qr-code).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// fr-FR
MERGE (en:EntityNative {key: 'dynamic-qr-code@fr-FR'})
ON CREATE SET
  en.display_name = 'Code QR Dynamique',
  en.description = 'Un code QR dynamique permet de changer l\'URL de destination à tout moment sans réimprimer le code, avec suivi des scans et analytics.',
  en.locale_key = 'fr-FR',
  en.entity_key = 'entity:dynamic-qr-code',
  en.denomination_text = 'code QR dynamique',
  en.denomination_title = 'Code QR Dynamique',
  en.denomination_abbrev = 'QR dynamique',
  en.denomination_url = 'code-qr-dynamique',
  en.llm_context = 'USE: when generating French content about editable trackable QR codes. TRIGGERS: dynamique, modifiable, traçable, analytique, statistiques, modifier URL. NOT: for permanent fixed QR codes (use code-qr-statique).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// =============================================================================
// 5. STATIC QR CODE — Fixed destination
// =============================================================================

// en-US
MERGE (en:EntityNative {key: 'static-qr-code@en-US'})
ON CREATE SET
  en.display_name = 'Static QR Code',
  en.description = 'A static QR code has a fixed destination that cannot be changed after creation. Best for permanent content like WiFi credentials.',
  en.locale_key = 'en-US',
  en.entity_key = 'entity:static-qr-code',
  en.denomination_text = 'static QR code',
  en.denomination_title = 'Static QR Code',
  en.denomination_abbrev = 'static QR',
  en.denomination_url = 'static-qr-code',
  en.llm_context = 'USE: when generating English content about permanent fixed QR codes. TRIGGERS: static, permanent, fixed, free, simple, basic, no tracking. NOT: for editable trackable QR codes (use dynamic-qr-code).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// fr-FR
MERGE (en:EntityNative {key: 'static-qr-code@fr-FR'})
ON CREATE SET
  en.display_name = 'Code QR Statique',
  en.description = 'Un code QR statique a une destination fixe qui ne peut pas être modifiée après création. Idéal pour le contenu permanent comme les identifiants WiFi.',
  en.locale_key = 'fr-FR',
  en.entity_key = 'entity:static-qr-code',
  en.denomination_text = 'code QR statique',
  en.denomination_title = 'Code QR Statique',
  en.denomination_abbrev = 'QR statique',
  en.denomination_url = 'code-qr-statique',
  en.llm_context = 'USE: when generating French content about permanent fixed QR codes. TRIGGERS: statique, permanent, fixe, gratuit, simple, basique, sans suivi. NOT: for editable trackable QR codes (use code-qr-dynamique).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// =============================================================================
// 6. SMART LINK — Intelligent routing
// =============================================================================

// en-US
MERGE (en:EntityNative {key: 'smart-link@en-US'})
ON CREATE SET
  en.display_name = 'Smart Link',
  en.description = 'A smart link intelligently routes users to different destinations based on device, location, language, or time.',
  en.locale_key = 'en-US',
  en.entity_key = 'entity:smart-link',
  en.denomination_text = 'smart link',
  en.denomination_title = 'Smart Link',
  en.denomination_abbrev = 'smart link',
  en.denomination_url = 'smart-link',
  en.llm_context = 'USE: when generating English content about intelligent URL routing. TRIGGERS: smart link, short URL, link, routing, redirect, device detection, geo-targeting. NOT: for QR codes themselves (use qr-code).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// fr-FR
MERGE (en:EntityNative {key: 'smart-link@fr-FR'})
ON CREATE SET
  en.display_name = 'Lien Intelligent',
  en.description = 'Un lien intelligent redirige les utilisateurs vers différentes destinations selon l\'appareil, la localisation, la langue ou l\'heure.',
  en.locale_key = 'fr-FR',
  en.entity_key = 'entity:smart-link',
  en.denomination_text = 'lien intelligent',
  en.denomination_title = 'Lien Intelligent',
  en.denomination_abbrev = 'lien intelligent',
  en.denomination_url = 'lien-intelligent',
  en.llm_context = 'USE: when generating French content about intelligent URL routing. TRIGGERS: lien intelligent, URL courte, lien, routage, redirection, détection appareil, ciblage géographique. NOT: for QR codes themselves (use code-qr).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// =============================================================================
// 7. LANDING PAGE — Destination page
// =============================================================================

// en-US
MERGE (en:EntityNative {key: 'landing-page@en-US'})
ON CREATE SET
  en.display_name = 'Landing Page',
  en.description = 'A landing page is a standalone web page designed for marketing campaigns, optimized for conversion.',
  en.locale_key = 'en-US',
  en.entity_key = 'entity:landing-page',
  en.denomination_text = 'landing page',
  en.denomination_title = 'Landing Page',
  en.denomination_abbrev = 'landing page',
  en.denomination_url = 'landing-page',
  en.llm_context = 'USE: when generating English content about QR code destinations and mobile pages. TRIGGERS: landing page, destination, mobile page, page builder, no-code, conversion. NOT: for general website pages.',
  en.created_at = datetime(),
  en.updated_at = datetime();

// fr-FR
MERGE (en:EntityNative {key: 'landing-page@fr-FR'})
ON CREATE SET
  en.display_name = 'Page de Destination',
  en.description = 'Une page de destination est une page web autonome conçue pour les campagnes marketing, optimisée pour la conversion.',
  en.locale_key = 'fr-FR',
  en.entity_key = 'entity:landing-page',
  en.denomination_text = 'page de destination',
  en.denomination_title = 'Page de Destination',
  en.denomination_abbrev = 'landing page',
  en.denomination_url = 'page-de-destination',
  en.llm_context = 'USE: when generating French content about QR code destinations and mobile pages. TRIGGERS: page de destination, destination, page mobile, créateur de page, sans code, conversion. NOT: for general website pages.',
  en.created_at = datetime(),
  en.updated_at = datetime();

// =============================================================================
// 8. BARCODE — Traditional barcode
// =============================================================================

// en-US
MERGE (en:EntityNative {key: 'barcode@en-US'})
ON CREATE SET
  en.display_name = 'Barcode',
  en.description = 'A barcode is a machine-readable code consisting of parallel lines or patterns that represents data about an item.',
  en.locale_key = 'en-US',
  en.entity_key = 'entity:barcode',
  en.denomination_text = 'barcode',
  en.denomination_title = 'Barcode',
  en.denomination_abbrev = 'barcode',
  en.denomination_url = 'barcode',
  en.llm_context = 'USE: when generating English content about 1D barcodes. TRIGGERS: barcode, EAN, UPC, Code 128, Code 39, 1D, linear. NOT: for 2D codes like QR codes (use qr-code).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// fr-FR
MERGE (en:EntityNative {key: 'barcode@fr-FR'})
ON CREATE SET
  en.display_name = 'Code-Barres',
  en.description = 'Un code-barres est un code lisible par machine composé de lignes parallèles ou de motifs représentant des données sur un article.',
  en.locale_key = 'fr-FR',
  en.entity_key = 'entity:barcode',
  en.denomination_text = 'code-barres',
  en.denomination_title = 'Code-Barres',
  en.denomination_abbrev = 'code-barres',
  en.denomination_url = 'code-barres',
  en.llm_context = 'USE: when generating French content about 1D barcodes. TRIGGERS: code-barres, EAN, UPC, Code 128, Code 39, 1D, linéaire. NOT: for 2D codes like QR codes (use code-qr).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// =============================================================================
// 9. QR CODE GENERATOR — The tool itself
// =============================================================================

// en-US
MERGE (en:EntityNative {key: 'qr-code-generator@en-US'})
ON CREATE SET
  en.display_name = 'QR Code Generator',
  en.description = 'A QR code generator is a tool that creates QR codes from URLs, text, contact info, WiFi credentials, and other data types.',
  en.locale_key = 'en-US',
  en.entity_key = 'entity:qr-code-generator',
  en.denomination_text = 'QR code generator',
  en.denomination_title = 'QR Code Generator',
  en.denomination_abbrev = 'QR generator',
  en.denomination_url = 'qr-code-generator',
  en.llm_context = 'USE: when generating English content about QR code creation tools. TRIGGERS: generator, create, make, tool, app, free, online. NOT: for QR code concepts (use qr-code).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// fr-FR
MERGE (en:EntityNative {key: 'qr-code-generator@fr-FR'})
ON CREATE SET
  en.display_name = 'Générateur de Code QR',
  en.description = 'Un générateur de code QR est un outil qui crée des codes QR à partir d\'URLs, texte, coordonnées, identifiants WiFi et autres types de données.',
  en.locale_key = 'fr-FR',
  en.entity_key = 'entity:qr-code-generator',
  en.denomination_text = 'générateur de code QR',
  en.denomination_title = 'Générateur de Code QR',
  en.denomination_abbrev = 'générateur QR',
  en.denomination_url = 'generateur-code-qr',
  en.llm_context = 'USE: when generating French content about QR code creation tools. TRIGGERS: générateur, créer, faire, outil, application, gratuit, en ligne. NOT: for QR code concepts (use code-qr).',
  en.created_at = datetime(),
  en.updated_at = datetime();

// =============================================================================
// Link EntityNatives to Entities via HAS_NATIVE
// =============================================================================

// Link all EntityNatives to their parent Entities
MATCH (en:EntityNative)
WHERE en.entity_key IS NOT NULL
MATCH (e:Entity {key: en.entity_key})
MERGE (e)-[:HAS_NATIVE]->(en);

// Link all EntityNatives to their Locales
MATCH (en:EntityNative)
WHERE en.locale_key IS NOT NULL
MATCH (loc:Locale {key: en.locale_key})
MERGE (en)-[:FOR_LOCALE]->(loc);

// =============================================================================
// Summary
// =============================================================================
RETURN 'EntityNative bootstrap complete: 18 natives for 9 pillars (en-US + fr-FR)' AS status;
