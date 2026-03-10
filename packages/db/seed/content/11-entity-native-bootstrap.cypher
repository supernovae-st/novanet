// ═══════════════════════════════════════════════════════════════════════════════
// 11-entity-native-bootstrap.cypher
// Bootstrap EntityNative nodes for 9 pillar entities
// v0.17.3 - Remove content.features (use HAS_FEATURE arcs instead)
// NOTE: Features are expressed via EntityNative nodes of feature entities + HAS_FEATURE arcs
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// FR-FR LOCALES
// ─────────────────────────────────────────────────────────────────────────────────

// qr-code@fr-FR
MERGE (en:EntityNative {key: 'qr-code@fr-FR'})
SET en.display_name = 'QR Code',
    en.content = '{"definition": "Code-barres bidimensionnel encodant des données dans un motif visuel scannable. Lisible par les appareils photo des smartphones sans application spécialisée.", "context": "Entité pilier de QR Code AI. Toutes les fonctionnalités de la plateforme se connectent à ce concept central.", "cultural_context": "Les QR codes sont très répandus en France depuis le COVID-19 (menus restaurants, pass sanitaire). Les Français utilisent couramment scanner ou flasher."}',
    en.denomination_forms = '[{"type": "text", "value": "qr code", "priority": 1}, {"type": "title", "value": "QR Code", "priority": 1}, {"type": "abbrev", "value": "qr", "priority": 1}, {"type": "url", "value": "creer-un-qr-code", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"Quand on parle de codes-barres 2D, de scan mobile, ou de codes scannables sur QR Code AI.","triggers":["qr code","code qr","code matriciel","scanner","flasher","scannable"],"not_for":["QR codes spécifiques (utiliser qr-code-dynamique, qr-code-statique)","Codes-barres 1D"]}',
    en.entity_key = 'entity:qr-code',
    en.locale_key = 'fr-FR',
    en.locale = 'fr-FR',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// custom-qr-code@fr-FR
MERGE (en:EntityNative {key: 'custom-qr-code@fr-FR'})
SET en.display_name = 'QR Code Personnalisé',
    en.content = '{"definition": "QR Code entièrement personnalisable avec couleurs, formes et logo de votre marque.", "context": "Pilier personnalisation de QR Code AI. Permet de créer des QR codes aux couleurs de sa marque.", "cultural_context": "Les entreprises françaises apprécient particulièrement l aspect made in France et le RGPD compliance."}',
    en.denomination_forms = '[{"type": "text", "value": "qr code personnalisé", "priority": 1}, {"type": "title", "value": "QR Code Personnalisé", "priority": 1}, {"type": "abbrev", "value": "qr perso", "priority": 1}, {"type": "url", "value": "qr-code-personnalise", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"Pour du contenu français sur la personnalisation de QR codes.","triggers":["personnalisé","personnalisation","couleurs","logo","design","marque"],"not_for":["QR codes artistiques IA (utiliser qr-code-art)"]}',
    en.entity_key = 'entity:custom-qr-code',
    en.locale_key = 'fr-FR',
    en.locale = 'fr-FR',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// qr-code-art@fr-FR
MERGE (en:EntityNative {key: 'qr-code-art@fr-FR'})
SET en.display_name = 'QR Code Art',
    en.content = '{"definition": "QR Code artistique généré par intelligence artificielle, fusionnant le code scannable avec une œuvre d art.", "context": "Différenciateur clé de QR Code AI. Utilise Stable Diffusion pour créer des QR codes uniques.", "cultural_context": "L art généré par IA suscite un fort intérêt en France, pays de tradition artistique."}',
    en.denomination_forms = '[{"type": "text", "value": "qr code art", "priority": 1}, {"type": "title", "value": "QR Code Art", "priority": 1}, {"type": "abbrev", "value": "qr art", "priority": 1}, {"type": "url", "value": "qr-code-art", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"Pour du contenu français sur les QR codes artistiques générés par IA.","triggers":["art","artistique","IA","intelligence artificielle","créatif","image","design"],"not_for":["Personnalisation simple (utiliser qr-code-personnalise)"]}',
    en.entity_key = 'entity:qr-code-art',
    en.locale_key = 'fr-FR',
    en.locale = 'fr-FR',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// dynamic-qr-code@fr-FR
MERGE (en:EntityNative {key: 'dynamic-qr-code@fr-FR'})
SET en.display_name = 'QR Code Dynamique',
    en.content = '{"definition": "QR Code modifiable après impression dont la destination peut être mise à jour sans régénérer le code.", "context": "Fonctionnalité premium de QR Code AI. Permet de modifier l URL cible et de suivre les scans.", "cultural_context": "Les entreprises françaises privilégient la conformité RGPD pour l analytique des scans."}',
    en.denomination_forms = '[{"type": "text", "value": "qr code dynamique", "priority": 1}, {"type": "title", "value": "QR Code Dynamique", "priority": 1}, {"type": "abbrev", "value": "qr dynamique", "priority": 1}, {"type": "url", "value": "qr-code-dynamique", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"Pour du contenu français sur les QR codes éditables et traçables.","triggers":["dynamique","modifiable","éditable","traçable","analytique","statistiques"],"not_for":["QR codes permanents (utiliser qr-code-statique)"]}',
    en.entity_key = 'entity:dynamic-qr-code',
    en.locale_key = 'fr-FR',
    en.locale = 'fr-FR',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// static-qr-code@fr-FR
MERGE (en:EntityNative {key: 'static-qr-code@fr-FR'})
SET en.display_name = 'QR Code Statique',
    en.content = '{"definition": "QR Code fixe avec données encodées directement dans le motif. Gratuit et permanent.", "context": "Offre gratuite de QR Code AI. Point d entrée pour l acquisition utilisateurs.", "cultural_context": "Les consommateurs français apprécient la gratuité et l absence d inscription."}',
    en.denomination_forms = '[{"type": "text", "value": "qr code statique", "priority": 1}, {"type": "title", "value": "QR Code Statique", "priority": 1}, {"type": "abbrev", "value": "qr statique", "priority": 1}, {"type": "url", "value": "qr-code-statique", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"Pour du contenu français sur les QR codes gratuits et permanents.","triggers":["statique","permanent","fixe","gratuit","simple","basique"],"not_for":["QR codes éditables (utiliser qr-code-dynamique)"]}',
    en.entity_key = 'entity:static-qr-code',
    en.locale_key = 'fr-FR',
    en.locale = 'fr-FR',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// smart-link@fr-FR
MERGE (en:EntityNative {key: 'smart-link@fr-FR'})
SET en.display_name = 'Lien Intelligent',
    en.content = '{"definition": "URL intelligente avec règles de routage pour rediriger selon l appareil ou la localisation.", "context": "Fonctionnalité de gestion de liens de QR Code AI. Alimente les QR codes dynamiques.", "cultural_context": "Le terme lien intelligent est préféré à smart link en français professionnel."}',
    en.denomination_forms = '[{"type": "text", "value": "lien intelligent", "priority": 1}, {"type": "title", "value": "Lien Intelligent", "priority": 1}, {"type": "abbrev", "value": "lien", "priority": 1}, {"type": "url", "value": "lien-intelligent", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"Pour du contenu français sur les liens intelligents et le routage d URL.","triggers":["lien intelligent","URL courte","raccourcisseur","routage","redirection","géolocalisation"],"not_for":["QR codes eux-mêmes"]}',
    en.entity_key = 'entity:smart-link',
    en.locale_key = 'fr-FR',
    en.locale = 'fr-FR',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// landing-page@fr-FR
MERGE (en:EntityNative {key: 'landing-page@fr-FR'})
SET en.display_name = 'Page de Destination',
    en.content = '{"definition": "Page web de destination créée via un constructeur no-code intégré, optimisée mobile.", "context": "Constructeur de pages de QR Code AI. Crée des destinations dédiées pour les scans QR.", "cultural_context": "Le terme landing page est souvent utilisé tel quel en France, mais page de destination est préféré dans les communications officielles."}',
    en.denomination_forms = '[{"type": "text", "value": "page de destination", "priority": 1}, {"type": "title", "value": "Page de Destination", "priority": 1}, {"type": "abbrev", "value": "page", "priority": 1}, {"type": "url", "value": "page-de-destination", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"Pour du contenu français sur les pages de destination et le constructeur de pages.","triggers":["page de destination","landing page","no-code","conversion","mobile"],"not_for":["Pages web générales"]}',
    en.entity_key = 'entity:landing-page',
    en.locale_key = 'fr-FR',
    en.locale = 'fr-FR',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// barcode@fr-FR
MERGE (en:EntityNative {key: 'barcode@fr-FR'})
SET en.display_name = 'Code-barres',
    en.content = '{"definition": "Code-barres linéaire 1D pour l identification des produits (EAN, UPC, Code 128).", "context": "Support des formats legacy sur QR Code AI. Génération de codes-barres traditionnels.", "cultural_context": "L EAN-13 est le format standard en France et en Europe pour les produits de consommation."}',
    en.denomination_forms = '[{"type": "text", "value": "code-barres", "priority": 1}, {"type": "title", "value": "Code-barres", "priority": 1}, {"type": "abbrev", "value": "code-barres", "priority": 1}, {"type": "url", "value": "code-barres", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"Pour du contenu français sur les codes-barres 1D et les formats produits.","triggers":["code-barres","EAN","UPC","Code 128","1D","linéaire","produit"],"not_for":["QR codes 2D (utiliser qr-code)"]}',
    en.entity_key = 'entity:barcode',
    en.locale_key = 'fr-FR',
    en.locale = 'fr-FR',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// qr-code-generator@fr-FR
MERGE (en:EntityNative {key: 'qr-code-generator@fr-FR'})
SET en.display_name = 'Générateur de QR Code',
    en.content = '{"definition": "Outil de création de QR codes personnalisés, gratuit et sans inscription.", "context": "Point d entrée principal de QR Code AI. L outil qui génère tous les types de QR codes.", "cultural_context": "Les utilisateurs français recherchent souvent générateur qr code gratuit ou créer qr code."}',
    en.denomination_forms = '[{"type": "text", "value": "générateur de qr code", "priority": 1}, {"type": "title", "value": "Générateur de QR Code", "priority": 1}, {"type": "abbrev", "value": "générateur", "priority": 1}, {"type": "url", "value": "generateur-qr-code", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"Pour du contenu français sur l outil de création de QR codes.","triggers":["générateur","créer","faire","outil","gratuit","en ligne"],"not_for":["Concepts QR code (utiliser qr-code)"]}',
    en.entity_key = 'entity:qr-code-generator',
    en.locale_key = 'fr-FR',
    en.locale = 'fr-FR',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// ─────────────────────────────────────────────────────────────────────────────────
// EN-US LOCALES
// ─────────────────────────────────────────────────────────────────────────────────

// qr-code@en-US (root entity)
MERGE (en:EntityNative {key: 'qr-code@en-US'})
SET en.display_name = 'QR Code',
    en.content = '{"definition": "2D matrix barcode that encodes data in a scannable visual pattern. Readable by smartphone cameras without specialized apps.", "context": "Pillar entity of QR Code AI. All platform features connect to this core concept.", "cultural_context": "QR codes gained mainstream US adoption during COVID-19 (contactless menus, health passes). Americans commonly use scan or scan the code."}',
    en.denomination_forms = '[{"type": "text", "value": "qr code", "priority": 1}, {"type": "title", "value": "QR Code", "priority": 1}, {"type": "abbrev", "value": "qr", "priority": 1}, {"type": "url", "value": "qr-code-generator", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"When generating English content about QR codes, 2D barcodes, or mobile scanning on QR Code AI.","triggers":["qr code","qr","barcode","scan","mobile","2d code","matrix code"],"not_for":["Specific QR types (use dynamic-qr-code, static-qr-code)","1D barcodes (use barcode entity)"]}',
    en.entity_key = 'entity:qr-code',
    en.locale_key = 'en-US',
    en.locale = 'en-US',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// custom-qr-code@en-US
MERGE (en:EntityNative {key: 'custom-qr-code@en-US'})
SET en.display_name = 'Custom QR Code',
    en.content = '{"definition": "Fully customizable QR code with brand colors, shapes, logos and decorative frames.", "context": "Key differentiator of QR Code AI. Enables brand-aligned QR codes that stand out.", "cultural_context": "Brand customization is highly valued in US marketing. Companies expect QR codes to match brand identity."}',
    en.denomination_forms = '[{"type": "text", "value": "custom qr code", "priority": 1}, {"type": "title", "value": "Custom QR Code", "priority": 1}, {"type": "abbrev", "value": "custom qr", "priority": 1}, {"type": "url", "value": "custom-qr-code", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"When generating English content about customizable, branded, or personalized QR codes.","triggers":["custom","personalized","branded","logo","colors","design","brand"],"not_for":["AI-generated artistic QR codes (use qr-code-art)"]}',
    en.entity_key = 'entity:custom-qr-code',
    en.locale_key = 'en-US',
    en.locale = 'en-US',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// dynamic-qr-code@en-US
MERGE (en:EntityNative {key: 'dynamic-qr-code@en-US'})
SET en.display_name = 'Dynamic QR Code',
    en.content = '{"definition": "Editable QR code with built-in analytics tracking. Change destination URL anytime without reprinting.", "context": "Premium feature of QR Code AI. Enables post-print flexibility and ROI measurement.", "cultural_context": "US businesses prioritize ROI tracking. Dynamic QR codes appeal to data-driven marketers."}',
    en.denomination_forms = '[{"type": "text", "value": "dynamic qr code", "priority": 1}, {"type": "title", "value": "Dynamic QR Code", "priority": 1}, {"type": "abbrev", "value": "dynamic qr", "priority": 1}, {"type": "url", "value": "dynamic-qr-code", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"When generating English content about editable, trackable, or analytics-enabled QR codes.","triggers":["dynamic","editable","trackable","analytics","changeable","update URL","flexible"],"not_for":["Permanent QR codes (use static-qr-code)"]}',
    en.entity_key = 'entity:dynamic-qr-code',
    en.locale_key = 'en-US',
    en.locale = 'en-US',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// static-qr-code@en-US
MERGE (en:EntityNative {key: 'static-qr-code@en-US'})
SET en.display_name = 'Static QR Code',
    en.content = '{"definition": "Permanent QR code with data encoded directly in the pattern. Free forever, no account needed.", "context": "Entry-level offering of QR Code AI. Great for simple, unchanging use cases.", "cultural_context": "Americans appreciate free tools with no strings attached. Static QR codes serve cost-conscious users."}',
    en.denomination_forms = '[{"type": "text", "value": "static qr code", "priority": 1}, {"type": "title", "value": "Static QR Code", "priority": 1}, {"type": "abbrev", "value": "static qr", "priority": 1}, {"type": "url", "value": "static-qr-code", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"When generating English content about permanent, fixed, or free QR codes.","triggers":["static","permanent","fixed","free","simple","no tracking","basic"],"not_for":["Editable QR codes (use dynamic-qr-code)"]}',
    en.entity_key = 'entity:static-qr-code',
    en.locale_key = 'en-US',
    en.locale = 'en-US',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// qr-code-generator@en-US
MERGE (en:EntityNative {key: 'qr-code-generator@en-US'})
SET en.display_name = 'QR Code Generator',
    en.content = '{"definition": "Free online tool to create custom QR codes instantly. No signup or account required.", "context": "Main entry point for QR Code AI. The tool that generates all QR code types.", "cultural_context": "US users expect free tools with immediate results. No signup required is a key selling point."}',
    en.denomination_forms = '[{"type": "text", "value": "qr code generator", "priority": 1}, {"type": "title", "value": "QR Code Generator", "priority": 1}, {"type": "abbrev", "value": "generator", "priority": 1}, {"type": "url", "value": "qr-code-generator", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"When generating English content about the QR code creation tool.","triggers":["generator","create","make","tool","free","online","app","builder"],"not_for":["QR code concepts (use qr-code)"]}',
    en.entity_key = 'entity:qr-code-generator',
    en.locale_key = 'en-US',
    en.locale = 'en-US',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// qr-code-art@en-US
MERGE (en:EntityNative {key: 'qr-code-art@en-US'})
SET en.display_name = 'QR Code Art',
    en.content = '{"definition": "AI-generated QR codes that transform functional codes into visually stunning artistic designs while maintaining scannability.", "context": "Premium creative feature of QR Code AI. Merges art and technology for unique marketing materials.", "cultural_context": "Americans love combining tech and creativity. QR code art appeals to marketers seeking standout visuals."}',
    en.denomination_forms = '[{"type": "text", "value": "qr code art", "priority": 1}, {"type": "title", "value": "QR Code Art", "priority": 1}, {"type": "abbrev", "value": "qr art", "priority": 1}, {"type": "url", "value": "qr-code-art", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"When generating English content about AI-generated artistic QR codes.","triggers":["art","artistic","AI","generative","creative","beautiful","design","image","aesthetic"],"not_for":["Simple color/logo customization (use custom-qr-code)"]}',
    en.entity_key = 'entity:qr-code-art',
    en.locale_key = 'en-US',
    en.locale = 'en-US',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// smart-link@en-US
MERGE (en:EntityNative {key: 'smart-link@en-US'})
SET en.display_name = 'Smart Link',
    en.content = '{"definition": "Intelligent URL that routes users to different destinations based on device, location, language, or time.", "context": "Advanced routing feature of QR Code AI. Enables personalized user experiences from a single link.", "cultural_context": "US marketers value personalization and targeting. Smart links enable sophisticated campaign optimization."}',
    en.denomination_forms = '[{"type": "text", "value": "smart link", "priority": 1}, {"type": "title", "value": "Smart Link", "priority": 1}, {"type": "abbrev", "value": "smart link", "priority": 1}, {"type": "url", "value": "smart-link", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"When generating English content about intelligent URL routing and link management.","triggers":["smart link","short URL","link","routing","redirect","device detection","geo-targeting"],"not_for":["QR codes themselves (use qr-code)"]}',
    en.entity_key = 'entity:smart-link',
    en.locale_key = 'en-US',
    en.locale = 'en-US',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// landing-page@en-US
MERGE (en:EntityNative {key: 'landing-page@en-US'})
SET en.display_name = 'Landing Page',
    en.content = '{"definition": "Standalone web page designed for QR code destinations, optimized for mobile and conversion.", "context": "No-code page builder feature of QR Code AI. Creates mobile-optimized destinations without technical skills.", "cultural_context": "US businesses expect quick, professional results without coding. Landing page builders are mainstream tools."}',
    en.denomination_forms = '[{"type": "text", "value": "landing page", "priority": 1}, {"type": "title", "value": "Landing Page", "priority": 1}, {"type": "abbrev", "value": "landing page", "priority": 1}, {"type": "url", "value": "landing-page", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"When generating English content about QR code destinations and mobile page builders.","triggers":["landing page","destination","mobile page","page builder","no-code","conversion"],"not_for":["General website pages"]}',
    en.entity_key = 'entity:landing-page',
    en.locale_key = 'en-US',
    en.locale = 'en-US',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// barcode@en-US
MERGE (en:EntityNative {key: 'barcode@en-US'})
SET en.display_name = 'Barcode',
    en.content = '{"definition": "Machine-readable 1D code with parallel lines representing data. Traditional format for retail and inventory.", "context": "Complementary offering of QR Code AI. Supports legacy barcode needs alongside modern QR codes.", "cultural_context": "Barcodes remain ubiquitous in US retail. Many businesses need both 1D barcodes and QR codes."}',
    en.denomination_forms = '[{"type": "text", "value": "barcode", "priority": 1}, {"type": "title", "value": "Barcode", "priority": 1}, {"type": "abbrev", "value": "barcode", "priority": 1}, {"type": "url", "value": "barcode", "priority": 1}]',
    en.status = 'active',
    en.llm_context = '{"use":"When generating English content about 1D linear barcodes.","triggers":["barcode","EAN","UPC","Code 128","Code 39","1D","linear"],"not_for":["2D codes like QR codes (use qr-code)"]}',
    en.entity_key = 'entity:barcode',
    en.locale_key = 'en-US',
    en.locale = 'en-US',
    en.version = 1,
    en.created_by = 'content:bootstrap',
    en.created_at = datetime(),
    en.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════════
// LINK EntityNative TO Entity (HAS_NATIVE)
// ═══════════════════════════════════════════════════════════════════════════════

// fr-FR links
MATCH (e:Entity {key: 'entity:qr-code'}), (en:EntityNative {key: 'qr-code@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:custom-qr-code'}), (en:EntityNative {key: 'custom-qr-code@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:qr-code-art'}), (en:EntityNative {key: 'qr-code-art@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:dynamic-qr-code'}), (en:EntityNative {key: 'dynamic-qr-code@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:static-qr-code'}), (en:EntityNative {key: 'static-qr-code@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:smart-link'}), (en:EntityNative {key: 'smart-link@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:landing-page'}), (en:EntityNative {key: 'landing-page@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:barcode'}), (en:EntityNative {key: 'barcode@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:qr-code-generator'}), (en:EntityNative {key: 'qr-code-generator@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

// en-US links
MATCH (e:Entity {key: 'entity:qr-code'}), (en:EntityNative {key: 'qr-code@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:custom-qr-code'}), (en:EntityNative {key: 'custom-qr-code@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:dynamic-qr-code'}), (en:EntityNative {key: 'dynamic-qr-code@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:static-qr-code'}), (en:EntityNative {key: 'static-qr-code@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:qr-code-generator'}), (en:EntityNative {key: 'qr-code-generator@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:qr-code-art'}), (en:EntityNative {key: 'qr-code-art@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:smart-link'}), (en:EntityNative {key: 'smart-link@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:landing-page'}), (en:EntityNative {key: 'landing-page@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:barcode'}), (en:EntityNative {key: 'barcode@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

// ═══════════════════════════════════════════════════════════════════════════════
// LINK EntityNative TO Locale (FOR_LOCALE)
// ═══════════════════════════════════════════════════════════════════════════════

// fr-FR locale links
MATCH (en:EntityNative), (l:Locale {key: 'fr-FR'})
WHERE en.key ENDS WITH '@fr-FR'
MERGE (en)-[:FOR_LOCALE]->(l);

// en-US locale links
MATCH (en:EntityNative), (l:Locale {key: 'en-US'})
WHERE en.key ENDS WITH '@en-US'
MERGE (en)-[:FOR_LOCALE]->(l);
