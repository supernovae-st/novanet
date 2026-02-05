// NovaNet Golden Data Seed v10.4.0
// Complete dataset for testing all views and features
// Uses MERGE for idempotent operations
// All nodes follow property order: key, display_name, description, llm_context, [node-specific], created_at, updated_at
// v10.4: Entity-Centric Architecture (Entity/EntityL10n), GEO layer removed

// =============================================================================
// NEW ENTITIES (10 additional entities for rich demo)
// =============================================================================

// Feature Entities
MERGE (c:Entity {key: "feature-customization"})
ON CREATE SET
  c.key = "feature-customization",
  c.display_name = "Customization",
  c.description = "Ability to customize QR code appearance with colors, logos, and designs",
  c.llm_context = "USE: when discussing visual customization options. TRIGGERS: customize, design, colors, logo, style, brand. NOT: analytics or tracking features.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

MERGE (c:Entity {key: "feature-bulk"})
ON CREATE SET
  c.key = "feature-bulk",
  c.display_name = "Bulk Generation",
  c.description = "Generate multiple QR codes at once from spreadsheets or APIs",
  c.llm_context = "USE: when discussing batch or bulk operations. TRIGGERS: bulk, batch, multiple, mass, import, spreadsheet. NOT: single QR code creation.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

MERGE (c:Entity {key: "feature-dynamic"})
ON CREATE SET
  c.key = "feature-dynamic",
  c.display_name = "Dynamic QR Codes",
  c.description = "Editable QR codes that can be updated without reprinting",
  c.llm_context = "USE: when discussing editable or changeable QR codes. TRIGGERS: dynamic, editable, changeable, update, modify. NOT: static QR codes.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

MERGE (c:Entity {key: "feature-templates"})
ON CREATE SET
  c.key = "feature-templates",
  c.display_name = "Design Templates",
  c.description = "Pre-designed QR code templates for quick creation",
  c.llm_context = "USE: when discussing ready-made designs. TRIGGERS: templates, presets, designs, ready-made. NOT: custom design from scratch.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

// Benefit Entities
MERGE (c:Entity {key: "benefit-time-saving"})
ON CREATE SET
  c.key = "benefit-time-saving",
  c.display_name = "Time Saving",
  c.description = "Save time with instant QR code generation and bulk processing",
  c.llm_context = "USE: when emphasizing efficiency and speed. TRIGGERS: fast, quick, instant, save time, efficient, seconds. NOT: feature descriptions or pricing.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

MERGE (c:Entity {key: "benefit-cost-effective"})
ON CREATE SET
  c.key = "benefit-cost-effective",
  c.display_name = "Cost Effective",
  c.description = "Affordable pricing with generous free tier and transparent costs",
  c.llm_context = "USE: when discussing value and pricing benefits. TRIGGERS: affordable, cheap, free, value, cost, budget, ROI. NOT: feature or time benefits.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

// Use Case Entities
MERGE (c:Entity {key: "usecase-marketing"})
ON CREATE SET
  c.key = "usecase-marketing",
  c.display_name = "Marketing Campaigns",
  c.description = "Using QR codes for marketing, advertising, and promotional campaigns",
  c.llm_context = "USE: when discussing marketing applications. TRIGGERS: marketing, campaign, advertising, promotion, ads. NOT: restaurant or retail use cases.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

MERGE (c:Entity {key: "usecase-restaurant"})
ON CREATE SET
  c.key = "usecase-restaurant",
  c.display_name = "Restaurant Menus",
  c.description = "Digital menus, table ordering, and contactless dining experiences",
  c.llm_context = "USE: when discussing hospitality and food service. TRIGGERS: restaurant, menu, dining, food, cafe, bar. NOT: retail or marketing use cases.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

MERGE (c:Entity {key: "usecase-retail"})
ON CREATE SET
  c.key = "usecase-retail",
  c.display_name = "Retail and Products",
  c.description = "Product packaging, in-store displays, and retail promotions",
  c.llm_context = "USE: when discussing retail and product applications. TRIGGERS: retail, product, packaging, store, shop, inventory. NOT: restaurant or event use cases.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

MERGE (c:Entity {key: "usecase-events"})
ON CREATE SET
  c.key = "usecase-events",
  c.display_name = "Events and Tickets",
  c.description = "Event ticketing, check-in systems, and conference materials",
  c.llm_context = "USE: when discussing events and ticketing. TRIGGERS: event, ticket, conference, check-in, badge, registration. NOT: restaurant or marketing use cases.",
  c.created_at = datetime(),
  c.updated_at = datetime()
WITH c
MATCH (p:Project {key: "project-qrcode-ai"})
MERGE (p)-[:USES_ENTITY]->(c);

// =============================================================================
// ENTITYL10N - English (en-US)
// =============================================================================

MATCH (c:Entity {key: "feature-customization"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-feature-customization-en-US"})
ON CREATE SET
  cl.key = "entityl10n-feature-customization-en-US",
  cl.display_name = "Customize Your QR Codes",
  cl.description = "English localization for customization feature",
  cl.llm_context = "USE: when generating customization content in English. TRIGGERS: en-US, customization, design. NOT: other locales.",
  cl.title = "Full Customization",
  cl.definition = "Add your brand colors, logo, and custom designs to any QR code.",
  cl.purpose = "Make QR codes match your brand identity perfectly.",
  cl.benefits = ["Brand consistency", "Professional appearance", "Higher scan rates"],
  cl.use_cases = ["Business cards", "Product packaging", "Marketing materials"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "feature-bulk"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-feature-bulk-en-US"})
ON CREATE SET
  cl.key = "entityl10n-feature-bulk-en-US",
  cl.display_name = "Bulk QR Code Generation",
  cl.description = "English localization for bulk feature",
  cl.llm_context = "USE: when generating bulk feature content in English. TRIGGERS: en-US, bulk, batch. NOT: other locales.",
  cl.title = "Bulk Generation",
  cl.definition = "Create hundreds of unique QR codes from a spreadsheet in minutes.",
  cl.purpose = "Scale QR code creation for large campaigns and inventory.",
  cl.benefits = ["Save hours of work", "Consistent formatting", "Easy management"],
  cl.use_cases = ["Event tickets", "Inventory tracking", "Mass mailings"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "feature-dynamic"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-feature-dynamic-en-US"})
ON CREATE SET
  cl.key = "entityl10n-feature-dynamic-en-US",
  cl.display_name = "Dynamic QR Codes",
  cl.description = "English localization for dynamic QR codes",
  cl.llm_context = "USE: when generating dynamic QR content in English. TRIGGERS: en-US, dynamic, editable. NOT: other locales.",
  cl.title = "Editable QR Codes",
  cl.definition = "Change where your QR code points without reprinting.",
  cl.purpose = "Update destinations anytime, even after printing.",
  cl.benefits = ["No reprinting costs", "A/B test destinations", "Fix mistakes instantly"],
  cl.use_cases = ["Print materials", "Signage", "Business cards"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "feature-templates"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-feature-templates-en-US"})
ON CREATE SET
  cl.key = "entityl10n-feature-templates-en-US",
  cl.display_name = "Design Templates",
  cl.description = "English localization for templates feature",
  cl.llm_context = "USE: when generating templates content in English. TRIGGERS: en-US, templates, designs. NOT: other locales.",
  cl.title = "Ready-Made Templates",
  cl.definition = "Choose from dozens of professionally designed QR code templates.",
  cl.purpose = "Create stunning QR codes without design skills.",
  cl.benefits = ["Professional results", "Quick creation", "Industry-specific options"],
  cl.use_cases = ["Quick projects", "Non-designers", "Seasonal campaigns"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "benefit-time-saving"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-benefit-time-saving-en-US"})
ON CREATE SET
  cl.key = "entityl10n-benefit-time-saving-en-US",
  cl.display_name = "Save Valuable Time",
  cl.description = "English localization for time saving benefit",
  cl.llm_context = "USE: when emphasizing time benefits in English. TRIGGERS: en-US, fast, quick, instant. NOT: other locales or cost benefits.",
  cl.title = "Lightning Fast",
  cl.definition = "Generate QR codes in seconds, not hours.",
  cl.purpose = "Maximize productivity with instant generation.",
  cl.benefits = ["Instant results", "No learning curve", "Focus on what matters"],
  cl.use_cases = ["Last-minute campaigns", "Quick iterations", "Rapid prototyping"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "benefit-cost-effective"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-benefit-cost-effective-en-US"})
ON CREATE SET
  cl.key = "entityl10n-benefit-cost-effective-en-US",
  cl.display_name = "Maximize Your ROI",
  cl.description = "English localization for cost benefit",
  cl.llm_context = "USE: when emphasizing cost benefits in English. TRIGGERS: en-US, affordable, free, value. NOT: other locales or time benefits.",
  cl.title = "Cost Effective Solution",
  cl.definition = "Start free, scale affordably as you grow.",
  cl.purpose = "Get professional results without breaking the bank.",
  cl.benefits = ["Generous free tier", "Transparent pricing", "No hidden fees"],
  cl.use_cases = ["Startups", "Small businesses", "Budget-conscious teams"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-marketing"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-usecase-marketing-en-US"})
ON CREATE SET
  cl.key = "entityl10n-usecase-marketing-en-US",
  cl.display_name = "Marketing with QR Codes",
  cl.description = "English localization for marketing use case",
  cl.llm_context = "USE: when describing marketing applications in English. TRIGGERS: en-US, marketing, campaign. NOT: other locales or use cases.",
  cl.title = "Marketing Campaigns",
  cl.definition = "Bridge offline and online with trackable QR codes.",
  cl.purpose = "Measure and optimize marketing campaign performance.",
  cl.benefits = ["Track conversions", "A/B test designs", "Real-time analytics"],
  cl.use_cases = ["Print ads", "Billboards", "Direct mail"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-restaurant"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-usecase-restaurant-en-US"})
ON CREATE SET
  cl.key = "entityl10n-usecase-restaurant-en-US",
  cl.display_name = "Restaurant QR Solutions",
  cl.description = "English localization for restaurant use case",
  cl.llm_context = "USE: when describing restaurant applications in English. TRIGGERS: en-US, restaurant, menu, dining. NOT: other locales or use cases.",
  cl.title = "Digital Menus",
  cl.definition = "Contactless menus and ordering for modern dining.",
  cl.purpose = "Enhance guest experience and reduce printing costs.",
  cl.benefits = ["Update menus instantly", "Multi-language support", "Reduce contact points"],
  cl.use_cases = ["Table menus", "Takeout ordering", "Wine lists"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-retail"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-usecase-retail-en-US"})
ON CREATE SET
  cl.key = "entityl10n-usecase-retail-en-US",
  cl.display_name = "Retail QR Solutions",
  cl.description = "English localization for retail use case",
  cl.llm_context = "USE: when describing retail applications in English. TRIGGERS: en-US, retail, product, packaging. NOT: other locales or use cases.",
  cl.title = "Product and Retail",
  cl.definition = "Connect products to digital experiences.",
  cl.purpose = "Drive engagement from physical products to online content.",
  cl.benefits = ["Product authentication", "Extended content", "Customer engagement"],
  cl.use_cases = ["Packaging", "Shelf displays", "Price tags"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-events"}), (l:Locale {key: "en-US"})
MERGE (cl:EntityL10n {key: "entityl10n-usecase-events-en-US"})
ON CREATE SET
  cl.key = "entityl10n-usecase-events-en-US",
  cl.display_name = "Event QR Solutions",
  cl.description = "English localization for events use case",
  cl.llm_context = "USE: when describing event applications in English. TRIGGERS: en-US, event, ticket, conference. NOT: other locales or use cases.",
  cl.title = "Events and Ticketing",
  cl.definition = "Streamline event management with QR-powered check-in.",
  cl.purpose = "Speed up registration and enhance attendee experience.",
  cl.benefits = ["Faster check-in", "Real-time attendance", "Paperless tickets"],
  cl.use_cases = ["Conferences", "Concerts", "Workshops"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

// =============================================================================
// ENTITYL10N - French (fr-FR)
// =============================================================================

MATCH (c:Entity {key: "feature-customization"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-feature-customization-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-feature-customization-fr-FR",
  cl.display_name = "Personnalisez vos QR Codes",
  cl.description = "Localisation francaise pour la personnalisation",
  cl.llm_context = "USE: when generating customization content in French. TRIGGERS: fr-FR, personnalisation, design. NOT: other locales.",
  cl.title = "Personnalisation complete",
  cl.definition = "Ajoutez vos couleurs, logo et designs personnalises a tout QR code.",
  cl.purpose = "Faire correspondre les QR codes a votre identite de marque.",
  cl.benefits = ["Coherence de marque", "Aspect professionnel", "Meilleurs taux de scan"],
  cl.use_cases = ["Cartes de visite", "Emballage produit", "Supports marketing"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "feature-bulk"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-feature-bulk-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-feature-bulk-fr-FR",
  cl.display_name = "Generation en masse",
  cl.description = "Localisation francaise pour la fonction bulk",
  cl.llm_context = "USE: when generating bulk content in French. TRIGGERS: fr-FR, masse, lot. NOT: other locales.",
  cl.title = "Generation en masse",
  cl.definition = "Creez des centaines de QR codes uniques depuis un tableur en minutes.",
  cl.purpose = "Passer a l echelle pour les grandes campagnes.",
  cl.benefits = ["Gagnez des heures", "Formatage coherent", "Gestion facile"],
  cl.use_cases = ["Billets evenements", "Suivi inventaire", "Envois massifs"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "feature-dynamic"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-feature-dynamic-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-feature-dynamic-fr-FR",
  cl.display_name = "QR Codes Dynamiques",
  cl.description = "Localisation francaise pour les QR codes dynamiques",
  cl.llm_context = "USE: when generating dynamic QR content in French. TRIGGERS: fr-FR, dynamique, modifiable. NOT: other locales.",
  cl.title = "QR Codes Modifiables",
  cl.definition = "Changez la destination de votre QR code sans reimprimer.",
  cl.purpose = "Mettez a jour vos liens a tout moment apres impression.",
  cl.benefits = ["Pas de reimpression", "Tests A/B", "Correction instantanee"],
  cl.use_cases = ["Supports imprimes", "Signaletique", "Cartes de visite"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "feature-templates"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-feature-templates-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-feature-templates-fr-FR",
  cl.display_name = "Modeles de Design",
  cl.description = "Localisation francaise pour les modeles",
  cl.llm_context = "USE: when generating templates content in French. TRIGGERS: fr-FR, modeles, designs. NOT: other locales.",
  cl.title = "Modeles Prets a l Emploi",
  cl.definition = "Choisissez parmi des dizaines de modeles professionnels.",
  cl.purpose = "Creez des QR codes superbes sans competences en design.",
  cl.benefits = ["Resultats professionnels", "Creation rapide", "Options par secteur"],
  cl.use_cases = ["Projets rapides", "Non-designers", "Campagnes saisonnieres"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "benefit-time-saving"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-benefit-time-saving-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-benefit-time-saving-fr-FR",
  cl.display_name = "Gagnez un temps precieux",
  cl.description = "Localisation francaise pour le gain de temps",
  cl.llm_context = "USE: when emphasizing time benefits in French. TRIGGERS: fr-FR, rapide, instantane. NOT: other locales.",
  cl.title = "Ultra rapide",
  cl.definition = "Generez des QR codes en secondes, pas en heures.",
  cl.purpose = "Maximisez votre productivite avec la generation instantanee.",
  cl.benefits = ["Resultats instantanes", "Pas de courbe d apprentissage", "Focus sur l essentiel"],
  cl.use_cases = ["Campagnes urgentes", "Iterations rapides", "Prototypage"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "benefit-cost-effective"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-benefit-cost-effective-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-benefit-cost-effective-fr-FR",
  cl.display_name = "Maximisez votre ROI",
  cl.description = "Localisation francaise pour le rapport qualite-prix",
  cl.llm_context = "USE: when emphasizing cost benefits in French. TRIGGERS: fr-FR, abordable, gratuit. NOT: other locales.",
  cl.title = "Solution economique",
  cl.definition = "Commencez gratuitement, evoluez a prix abordable.",
  cl.purpose = "Obtenez des resultats pro sans vous ruiner.",
  cl.benefits = ["Offre gratuite genereuse", "Tarification transparente", "Pas de frais caches"],
  cl.use_cases = ["Startups", "PME", "Equipes avec budget limite"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-marketing"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-usecase-marketing-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-usecase-marketing-fr-FR",
  cl.display_name = "Marketing avec QR Codes",
  cl.description = "Localisation francaise pour le marketing",
  cl.llm_context = "USE: when describing marketing in French. TRIGGERS: fr-FR, marketing, campagne. NOT: other locales.",
  cl.title = "Campagnes Marketing",
  cl.definition = "Reliez offline et online avec des QR codes tracables.",
  cl.purpose = "Mesurez et optimisez la performance de vos campagnes.",
  cl.benefits = ["Suivez les conversions", "Testez les designs", "Analytics temps reel"],
  cl.use_cases = ["Publicites print", "Affiches", "Mailing direct"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-restaurant"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-usecase-restaurant-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-usecase-restaurant-fr-FR",
  cl.display_name = "Solutions Restaurant",
  cl.description = "Localisation francaise pour la restauration",
  cl.llm_context = "USE: when describing restaurant use in French. TRIGGERS: fr-FR, restaurant, menu. NOT: other locales.",
  cl.title = "Menus Numeriques",
  cl.definition = "Menus sans contact et commandes pour une restauration moderne.",
  cl.purpose = "Ameliorez l experience client et reduisez les couts d impression.",
  cl.benefits = ["MAJ instantanee", "Multi-langues", "Moins de contacts"],
  cl.use_cases = ["Menus de table", "Commandes a emporter", "Cartes des vins"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-retail"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-usecase-retail-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-usecase-retail-fr-FR",
  cl.display_name = "Solutions Retail",
  cl.description = "Localisation francaise pour le retail",
  cl.llm_context = "USE: when describing retail use in French. TRIGGERS: fr-FR, retail, produit. NOT: other locales.",
  cl.title = "Produits et Commerce",
  cl.definition = "Connectez vos produits a des experiences numeriques.",
  cl.purpose = "Favorisez l engagement des produits physiques vers le digital.",
  cl.benefits = ["Authentification produit", "Contenu etendu", "Engagement client"],
  cl.use_cases = ["Emballages", "Presentoirs", "Etiquettes prix"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-events"}), (l:Locale {key: "fr-FR"})
MERGE (cl:EntityL10n {key: "entityl10n-usecase-events-fr-FR"})
ON CREATE SET
  cl.key = "entityl10n-usecase-events-fr-FR",
  cl.display_name = "Solutions Evenements",
  cl.description = "Localisation francaise pour les evenements",
  cl.llm_context = "USE: when describing events use in French. TRIGGERS: fr-FR, evenement, billet. NOT: other locales.",
  cl.title = "Evenements et Billetterie",
  cl.definition = "Simplifiez la gestion des evenements avec le check-in QR.",
  cl.purpose = "Accelerez l enregistrement et ameliorez l experience participant.",
  cl.benefits = ["Check-in rapide", "Presence temps reel", "Billets sans papier"],
  cl.use_cases = ["Conferences", "Concerts", "Ateliers"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

// =============================================================================
// ENTITYL10N - Japanese (ja-JP) for key entities
// =============================================================================

MATCH (c:Entity {key: "feature-customization"}), (l:Locale {key: "ja-JP"})
MERGE (cl:EntityL10n {key: "entityl10n-feature-customization-ja-JP"})
ON CREATE SET
  cl.key = "entityl10n-feature-customization-ja-JP",
  cl.display_name = "QRコードをカスタマイズ",
  cl.description = "カスタマイズ機能の日本語ローカライゼーション",
  cl.llm_context = "USE: when generating customization content in Japanese. TRIGGERS: ja-JP, カスタマイズ, デザイン. NOT: other locales.",
  cl.title = "フルカスタマイズ",
  cl.definition = "ブランドカラー、ロゴ、カスタムデザインを任意のQRコードに追加できます。",
  cl.purpose = "QRコードをブランドアイデンティティに完璧に合わせます。",
  cl.benefits = ["ブランドの一貫性", "プロフェッショナルな外観", "高いスキャン率"],
  cl.use_cases = ["名刺", "商品パッケージ", "マーケティング資料"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "benefit-time-saving"}), (l:Locale {key: "ja-JP"})
MERGE (cl:EntityL10n {key: "entityl10n-benefit-time-saving-ja-JP"})
ON CREATE SET
  cl.key = "entityl10n-benefit-time-saving-ja-JP",
  cl.display_name = "貴重な時間を節約",
  cl.description = "時間節約メリットの日本語ローカライゼーション",
  cl.llm_context = "USE: when emphasizing time benefits in Japanese. TRIGGERS: ja-JP, 高速, 即座. NOT: other locales.",
  cl.title = "超高速",
  cl.definition = "数時間ではなく、数秒でQRコードを生成します。",
  cl.purpose = "即時生成で生産性を最大化します。",
  cl.benefits = ["即座の結果", "学習曲線なし", "重要なことに集中"],
  cl.use_cases = ["緊急キャンペーン", "高速イテレーション", "ラピッドプロトタイピング"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-restaurant"}), (l:Locale {key: "ja-JP"})
MERGE (cl:EntityL10n {key: "entityl10n-usecase-restaurant-ja-JP"})
ON CREATE SET
  cl.key = "entityl10n-usecase-restaurant-ja-JP",
  cl.display_name = "レストランQRソリューション",
  cl.description = "レストラン向けの日本語ローカライゼーション",
  cl.llm_context = "USE: when describing restaurant use in Japanese. TRIGGERS: ja-JP, レストラン, メニュー. NOT: other locales.",
  cl.title = "デジタルメニュー",
  cl.definition = "モダンな飲食店向けの非接触メニューと注文システム。",
  cl.purpose = "ゲスト体験を向上させ、印刷コストを削減します。",
  cl.benefits = ["即座にメニュー更新", "多言語対応", "接触ポイント削減"],
  cl.use_cases = ["テーブルメニュー", "テイクアウト注文", "ワインリスト"],
  cl.version = 1,
  cl.influence_count = 0,
  cl.created_at = datetime(),
  cl.updated_at = datetime()
MERGE (c)-[:HAS_L10N]->(cl)
MERGE (cl)-[:FOR_LOCALE]->(l);

// =============================================================================
// SEMANTIC LINKS - Dense concept graph for spreading activation
// =============================================================================

// Feature-to-Feature links
MATCH (c1:Entity {key: "feature-customization"}), (c2:Entity {key: "feature-templates"})
MERGE (c1)-[:SEMANTIC_LINK {type: "complements", temperature: 0.7, context: "templates enable quick customization"}]->(c2);

MATCH (c1:Entity {key: "feature-dynamic"}), (c2:Entity {key: "feature-analytics"})
MERGE (c1)-[:SEMANTIC_LINK {type: "enables", temperature: 0.85, context: "dynamic QR enables tracking"}]->(c2);

MATCH (c1:Entity {key: "feature-bulk"}), (c2:Entity {key: "feature-templates"})
MERGE (c1)-[:SEMANTIC_LINK {type: "uses", temperature: 0.6, context: "bulk generation uses templates"}]->(c2);

// Action-to-Feature links
MATCH (c1:Entity {key: "action-create-qr"}), (c2:Entity {key: "feature-customization"})
MERGE (c1)-[:SEMANTIC_LINK {type: "includes", temperature: 0.8, context: "creation includes customization options"}]->(c2);

MATCH (c1:Entity {key: "action-create-qr"}), (c2:Entity {key: "feature-templates"})
MERGE (c1)-[:SEMANTIC_LINK {type: "uses", temperature: 0.7, context: "creation can use templates"}]->(c2);

MATCH (c1:Entity {key: "action-create-qr"}), (c2:Entity {key: "feature-dynamic"})
MERGE (c1)-[:SEMANTIC_LINK {type: "produces", temperature: 0.75, context: "can create dynamic QR codes"}]->(c2);

// Tier-to-Feature links
MATCH (c1:Entity {key: "tier-pro"}), (c2:Entity {key: "feature-bulk"})
MERGE (c1)-[:SEMANTIC_LINK {type: "includes", temperature: 0.9, context: "bulk is pro feature"}]->(c2);

MATCH (c1:Entity {key: "tier-pro"}), (c2:Entity {key: "feature-dynamic"})
MERGE (c1)-[:SEMANTIC_LINK {type: "includes", temperature: 0.85, context: "dynamic QR is pro feature"}]->(c2);

MATCH (c1:Entity {key: "tier-free"}), (c2:Entity {key: "feature-customization"})
MERGE (c1)-[:SEMANTIC_LINK {type: "includes", temperature: 0.7, context: "basic customization in free tier"}]->(c2);

MATCH (c1:Entity {key: "tier-free"}), (c2:Entity {key: "feature-templates"})
MERGE (c1)-[:SEMANTIC_LINK {type: "includes", temperature: 0.6, context: "some templates in free tier"}]->(c2);

// Tier-to-Benefit links
MATCH (c1:Entity {key: "tier-free"}), (c2:Entity {key: "benefit-cost-effective"})
MERGE (c1)-[:SEMANTIC_LINK {type: "provides", temperature: 0.95, context: "free tier is cost effective"}]->(c2);

MATCH (c1:Entity {key: "tier-pro"}), (c2:Entity {key: "benefit-time-saving"})
MERGE (c1)-[:SEMANTIC_LINK {type: "provides", temperature: 0.8, context: "pro features save time"}]->(c2);

// Use Case-to-Feature links
MATCH (c1:Entity {key: "usecase-marketing"}), (c2:Entity {key: "feature-analytics"})
MERGE (c1)-[:SEMANTIC_LINK {type: "requires", temperature: 0.9, context: "marketing needs analytics"}]->(c2);

MATCH (c1:Entity {key: "usecase-marketing"}), (c2:Entity {key: "feature-customization"})
MERGE (c1)-[:SEMANTIC_LINK {type: "benefits_from", temperature: 0.75, context: "marketing benefits from branding"}]->(c2);

MATCH (c1:Entity {key: "usecase-restaurant"}), (c2:Entity {key: "feature-dynamic"})
MERGE (c1)-[:SEMANTIC_LINK {type: "requires", temperature: 0.9, context: "menus need dynamic updates"}]->(c2);

MATCH (c1:Entity {key: "usecase-retail"}), (c2:Entity {key: "feature-bulk"})
MERGE (c1)-[:SEMANTIC_LINK {type: "benefits_from", temperature: 0.8, context: "retail needs bulk for inventory"}]->(c2);

MATCH (c1:Entity {key: "usecase-events"}), (c2:Entity {key: "feature-bulk"})
MERGE (c1)-[:SEMANTIC_LINK {type: "requires", temperature: 0.85, context: "events need bulk tickets"}]->(c2);

MATCH (c1:Entity {key: "usecase-events"}), (c2:Entity {key: "feature-analytics"})
MERGE (c1)-[:SEMANTIC_LINK {type: "benefits_from", temperature: 0.8, context: "events benefit from check-in tracking"}]->(c2);

// Benefit-to-Feature links
MATCH (c1:Entity {key: "benefit-time-saving"}), (c2:Entity {key: "feature-templates"})
MERGE (c1)-[:SEMANTIC_LINK {type: "enabled_by", temperature: 0.8, context: "templates save time"}]->(c2);

MATCH (c1:Entity {key: "benefit-time-saving"}), (c2:Entity {key: "feature-bulk"})
MERGE (c1)-[:SEMANTIC_LINK {type: "enabled_by", temperature: 0.85, context: "bulk saves time"}]->(c2);

// =============================================================================
// PAGE TYPES
// =============================================================================

MERGE (pt:PageType {key: "pagetype-landing"})
ON CREATE SET
  pt.key = "pagetype-landing",
  pt.display_name = "Landing Page",
  pt.description = "High-conversion landing page template with hero, features, and CTAs",
  pt.llm_context = "USE: for conversion-focused pages. TRIGGERS: landing, conversion, hero. NOT: content or blog pages.",
  pt.structure = '["hero", "features", "social-proof", "cta"]',
  pt.created_at = datetime(),
  pt.updated_at = datetime();

MERGE (pt:PageType {key: "pagetype-pricing"})
ON CREATE SET
  pt.key = "pagetype-pricing",
  pt.display_name = "Pricing Page",
  pt.description = "Pricing comparison page with tier tables and FAQs",
  pt.llm_context = "USE: for pricing displays. TRIGGERS: pricing, plans, tiers. NOT: feature pages.",
  pt.structure = '["pricing-header", "pricing-table", "faq", "cta"]',
  pt.created_at = datetime(),
  pt.updated_at = datetime();

MERGE (pt:PageType {key: "pagetype-features"})
ON CREATE SET
  pt.key = "pagetype-features",
  pt.display_name = "Features Page",
  pt.description = "Product features showcase with grids and comparisons",
  pt.llm_context = "USE: for feature showcases. TRIGGERS: features, capabilities. NOT: pricing pages.",
  pt.structure = '["features-hero", "feature-grid", "comparison", "cta"]',
  pt.created_at = datetime(),
  pt.updated_at = datetime();

MERGE (pt:PageType {key: "pagetype-usecase"})
ON CREATE SET
  pt.key = "pagetype-usecase",
  pt.display_name = "Use Case Page",
  pt.description = "Industry-specific use case page with examples and benefits",
  pt.llm_context = "USE: for vertical use cases. TRIGGERS: use case, industry, solution. NOT: generic feature pages.",
  pt.structure = '["usecase-hero", "problem-solution", "features", "testimonials", "cta"]',
  pt.created_at = datetime(),
  pt.updated_at = datetime();

// =============================================================================
// BLOCK TYPES (additional)
// =============================================================================

MERGE (bt:BlockType {key: "blocktype-features-grid"})
ON CREATE SET
  bt.key = "blocktype-features-grid",
  bt.display_name = "Features Grid",
  bt.description = "Grid layout showcasing multiple features with icons and descriptions",
  bt.llm_context = "USE: for feature lists. TRIGGERS: features, grid, cards. NOT: single feature spotlight.",
  bt.rules = "Display 3-6 features in responsive grid. Each feature: icon, title, description.",
  bt.created_at = datetime(),
  bt.updated_at = datetime();

MERGE (bt:BlockType {key: "blocktype-pricing-table"})
ON CREATE SET
  bt.key = "blocktype-pricing-table",
  bt.display_name = "Pricing Table",
  bt.description = "Side-by-side comparison table for pricing tiers",
  bt.llm_context = "USE: for pricing comparison. TRIGGERS: pricing, tiers, compare. NOT: single plan display.",
  bt.rules = "Show all tiers side by side. Highlight recommended tier. List features with checkmarks.",
  bt.created_at = datetime(),
  bt.updated_at = datetime();

MERGE (bt:BlockType {key: "blocktype-testimonials"})
ON CREATE SET
  bt.key = "blocktype-testimonials",
  bt.display_name = "Testimonials",
  bt.description = "Customer testimonials and social proof section",
  bt.llm_context = "USE: for social proof. TRIGGERS: testimonials, reviews, quotes. NOT: case studies.",
  bt.rules = "Display 2-4 testimonials with photo, name, title, company, quote.",
  bt.created_at = datetime(),
  bt.updated_at = datetime();

MERGE (bt:BlockType {key: "blocktype-faq"})
ON CREATE SET
  bt.key = "blocktype-faq",
  bt.display_name = "FAQ Section",
  bt.description = "Frequently asked questions in collapsible accordion format",
  bt.llm_context = "USE: for FAQs. TRIGGERS: faq, questions, help. NOT: documentation.",
  bt.rules = "Collapsible Q&A format. 5-10 questions. Schema.org FAQ markup.",
  bt.created_at = datetime(),
  bt.updated_at = datetime();

MERGE (bt:BlockType {key: "blocktype-cta-banner"})
ON CREATE SET
  bt.key = "blocktype-cta-banner",
  bt.display_name = "CTA Banner",
  bt.description = "Call-to-action banner with headline and buttons",
  bt.llm_context = "USE: for conversion CTAs. TRIGGERS: cta, action, signup. NOT: navigation.",
  bt.rules = "Strong headline (5-8 words). Supporting text (15-25 words). Primary and secondary buttons.",
  bt.created_at = datetime(),
  bt.updated_at = datetime();

MERGE (bt:BlockType {key: "blocktype-stats"})
ON CREATE SET
  bt.key = "blocktype-stats",
  bt.display_name = "Statistics",
  bt.description = "Key metrics and statistics showcase",
  bt.llm_context = "USE: for social proof with numbers. TRIGGERS: stats, metrics, numbers. NOT: testimonials.",
  bt.rules = "Display 3-4 key statistics. Large numbers with context labels.",
  bt.created_at = datetime(),
  bt.updated_at = datetime();

// =============================================================================
// PAGES
// =============================================================================

MATCH (p:Project {key: "project-qrcode-ai"}), (pt:PageType {key: "pagetype-landing"})
MERGE (page:Page {key: "page-home"})
ON CREATE SET
  page.key = "page-home",
  page.display_name = "Home Page",
  page.description = "Main landing page for QR Code AI with value proposition and CTAs",
  page.llm_context = "USE: main conversion page. TRIGGERS: home, landing, main. NOT: subpages.",
  page.slug = "/",
  page.meta_title = "QR Code AI - Free QR Code Generator with Analytics",
  page.meta_description = "Create free, customizable QR codes with built-in analytics. Track scans, customize designs, and boost engagement.",
  page.instructions = "Generate compelling home page that converts visitors. Focus on value proposition, trust signals, and clear CTAs.",
  page.created_at = datetime(),
  page.updated_at = datetime()
MERGE (p)-[:HAS_PAGE]->(page)
MERGE (page)-[:OF_TYPE]->(pt);

MATCH (p:Project {key: "project-qrcode-ai"}), (pt:PageType {key: "pagetype-features"})
MERGE (page:Page {key: "page-features"})
ON CREATE SET
  page.key = "page-features",
  page.display_name = "Features Page",
  page.description = "Complete features showcase page for all QR Code AI capabilities",
  page.llm_context = "USE: feature exploration. TRIGGERS: features, capabilities. NOT: pricing.",
  page.slug = "/features",
  page.meta_title = "Features - QR Code AI | All Capabilities",
  page.meta_description = "Explore all QR Code AI features: customization, analytics, bulk generation, API access, and more.",
  page.instructions = "Showcase all features with clear benefits. Use visuals and examples.",
  page.created_at = datetime(),
  page.updated_at = datetime()
MERGE (p)-[:HAS_PAGE]->(page)
MERGE (page)-[:OF_TYPE]->(pt);

MATCH (p:Project {key: "project-qrcode-ai"}), (pt:PageType {key: "pagetype-usecase"})
MERGE (page:Page {key: "page-use-cases"})
ON CREATE SET
  page.key = "page-use-cases",
  page.display_name = "Use Cases",
  page.description = "Hub page linking to all industry-specific use cases",
  page.llm_context = "USE: use case overview. TRIGGERS: use cases, industries, solutions. NOT: single use case.",
  page.slug = "/use-cases",
  page.meta_title = "QR Code Use Cases - Industries & Applications",
  page.meta_description = "Discover how businesses use QR codes: restaurants, retail, events, marketing, and more.",
  page.instructions = "Showcase top use cases with industry examples. Link to detailed use case pages.",
  page.created_at = datetime(),
  page.updated_at = datetime()
MERGE (p)-[:HAS_PAGE]->(page)
MERGE (page)-[:OF_TYPE]->(pt);

// =============================================================================
// BLOCKS FOR HOME PAGE
// =============================================================================

MATCH (page:Page {key: "page-home"}), (bt:BlockType {key: "blocktype-hero"})
MERGE (b:Block {key: "block-home-hero"})
ON CREATE SET
  b.key = "block-home-hero",
  b.display_name = "Home Hero",
  b.description = "Main hero section with value proposition and primary CTA",
  b.llm_context = "USE: first impression. TRIGGERS: hero, headline, value prop. NOT: features list.",
  b.instructions = "Create impactful hero with headline @action-create-qr, subheadline @benefit-time-saving, primary CTA.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 1}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (page:Page {key: "page-home"}), (bt:BlockType {key: "blocktype-stats"})
MERGE (b:Block {key: "block-home-stats"})
ON CREATE SET
  b.key = "block-home-stats",
  b.display_name = "Home Stats",
  b.description = "Key statistics showing platform credibility",
  b.llm_context = "USE: social proof with numbers. TRIGGERS: stats, metrics, trust. NOT: testimonials.",
  b.instructions = "Show 4 key metrics: QR codes created, scans tracked, customers, satisfaction rate.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 2}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (page:Page {key: "page-home"}), (bt:BlockType {key: "blocktype-features-grid"})
MERGE (b:Block {key: "block-home-features"})
ON CREATE SET
  b.key = "block-home-features",
  b.display_name = "Home Features Grid",
  b.description = "Key features overview highlighting main capabilities",
  b.llm_context = "USE: feature highlights. TRIGGERS: features, capabilities, cards. NOT: hero or pricing blocks.",
  b.instructions = "Highlight 4 key features: @action-create-qr, @feature-customization, @feature-analytics, @feature-bulk.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 3}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (page:Page {key: "page-home"}), (bt:BlockType {key: "blocktype-testimonials"})
MERGE (b:Block {key: "block-home-testimonials"})
ON CREATE SET
  b.key = "block-home-testimonials",
  b.display_name = "Home Testimonials",
  b.description = "Customer testimonials for social proof",
  b.llm_context = "USE: social proof. TRIGGERS: testimonials, reviews, trust. NOT: stats or features.",
  b.instructions = "Show 3 testimonials from different industries: @usecase-marketing, @usecase-restaurant, @usecase-retail.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 4}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (page:Page {key: "page-home"}), (bt:BlockType {key: "blocktype-cta-banner"})
MERGE (b:Block {key: "block-home-cta"})
ON CREATE SET
  b.key = "block-home-cta",
  b.display_name = "Home CTA Banner",
  b.description = "Final conversion CTA with strong value proposition",
  b.llm_context = "USE: conversion push. TRIGGERS: cta, action, signup. NOT: navigation or informational blocks.",
  b.instructions = "Strong closing CTA emphasizing @benefit-cost-effective and @tier-free.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 5}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

// =============================================================================
// BLOCKS FOR PRICING PAGE (existing from MVP)
// =============================================================================

MATCH (page:Page {key: "page-pricing"}), (bt:BlockType {key: "blocktype-pricing-table"})
MERGE (b:Block {key: "block-pricing-table"})
ON CREATE SET
  b.key = "block-pricing-table",
  b.display_name = "Pricing Table",
  b.description = "Tier comparison table showing Free vs Pro",
  b.llm_context = "USE: plan comparison. TRIGGERS: tiers, compare, pricing. NOT: features or FAQ blocks.",
  b.instructions = "Compare @tier-free and @tier-pro. Highlight Pro with @feature-analytics and @feature-bulk.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 2}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (page:Page {key: "page-pricing"}), (bt:BlockType {key: "blocktype-faq"})
MERGE (b:Block {key: "block-pricing-faq"})
ON CREATE SET
  b.key = "block-pricing-faq",
  b.display_name = "Pricing FAQ",
  b.description = "Frequently asked questions about pricing",
  b.llm_context = "USE: pricing questions. TRIGGERS: faq, questions, help. NOT: documentation or feature blocks.",
  b.instructions = "Address common pricing questions: upgrades, refunds, enterprise options.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 3}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

// =============================================================================
// BLOCKS FOR FEATURES PAGE
// =============================================================================

MATCH (page:Page {key: "page-features"}), (bt:BlockType {key: "blocktype-hero"})
MERGE (b:Block {key: "block-features-hero"})
ON CREATE SET
  b.key = "block-features-hero",
  b.display_name = "Features Hero",
  b.description = "Features page header section",
  b.llm_context = "USE: features intro. TRIGGERS: features, header, hero. NOT: pricing or testimonial blocks.",
  b.instructions = "Header showcasing @product-generator capabilities.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 1}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (page:Page {key: "page-features"}), (bt:BlockType {key: "blocktype-features-grid"})
MERGE (b:Block {key: "block-features-grid"})
ON CREATE SET
  b.key = "block-features-grid",
  b.display_name = "Full Features Grid",
  b.description = "Complete features showcase grid",
  b.llm_context = "USE: all features. TRIGGERS: feature list, grid, cards. NOT: hero or pricing blocks.",
  b.instructions = "Display all features: @feature-customization, @feature-analytics, @feature-bulk, @feature-dynamic, @feature-templates.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 2}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (page:Page {key: "page-features"}), (bt:BlockType {key: "blocktype-testimonials"})
MERGE (b:Block {key: "block-features-testimonials"})
ON CREATE SET
  b.key = "block-features-testimonials",
  b.display_name = "Feature Testimonials",
  b.description = "Customer testimonials about specific features",
  b.llm_context = "USE: social proof. TRIGGERS: testimonials, reviews, quotes. NOT: case study or feature blocks.",
  b.instructions = "Show 3 testimonials highlighting @usecase-marketing success stories.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 3}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

// =============================================================================
// BLOCKS FOR USE CASES PAGE
// =============================================================================

MATCH (page:Page {key: "page-use-cases"}), (bt:BlockType {key: "blocktype-hero"})
MERGE (b:Block {key: "block-usecases-hero"})
ON CREATE SET
  b.key = "block-usecases-hero",
  b.display_name = "Use Cases Hero",
  b.description = "Use cases page header",
  b.llm_context = "USE: use cases intro. TRIGGERS: industries, solutions, applications. NOT: single use case.",
  b.instructions = "Header showcasing diverse use cases. Emphasize versatility.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 1}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

MATCH (page:Page {key: "page-use-cases"}), (bt:BlockType {key: "blocktype-features-grid"})
MERGE (b:Block {key: "block-usecases-grid"})
ON CREATE SET
  b.key = "block-usecases-grid",
  b.display_name = "Use Cases Grid",
  b.description = "Grid of industry use cases",
  b.llm_context = "USE: use case overview. TRIGGERS: industries, grid, use cases. NOT: features.",
  b.instructions = "Show 4 use cases: @usecase-marketing, @usecase-restaurant, @usecase-retail, @usecase-events.",
  b.created_at = datetime(),
  b.updated_at = datetime()
MERGE (page)-[:HAS_BLOCK {position: 2}]->(b)
MERGE (b)-[:OF_TYPE]->(bt);

// =============================================================================
// USES_ENTITY relationships (v10.4: Entity-Centric Architecture)
// =============================================================================

MATCH (b:Block {key: "block-home-hero"}), (c:Entity {key: "action-create-qr"})
MERGE (b)-[:USES_ENTITY {purpose: "headline", temperature: 0.9}]->(c);

MATCH (b:Block {key: "block-home-hero"}), (c:Entity {key: "benefit-time-saving"})
MERGE (b)-[:USES_ENTITY {purpose: "subheadline", temperature: 0.7}]->(c);

MATCH (b:Block {key: "block-home-features"}), (c:Entity {key: "feature-customization"})
MERGE (b)-[:USES_ENTITY {purpose: "feature", temperature: 0.8}]->(c);

MATCH (b:Block {key: "block-home-features"}), (c:Entity {key: "feature-analytics"})
MERGE (b)-[:USES_ENTITY {purpose: "feature", temperature: 0.8}]->(c);

MATCH (b:Block {key: "block-home-features"}), (c:Entity {key: "feature-bulk"})
MERGE (b)-[:USES_ENTITY {purpose: "feature", temperature: 0.8}]->(c);

MATCH (b:Block {key: "block-home-features"}), (c:Entity {key: "feature-dynamic"})
MERGE (b)-[:USES_ENTITY {purpose: "feature", temperature: 0.8}]->(c);

MATCH (b:Block {key: "block-home-cta"}), (c:Entity {key: "tier-free"})
MERGE (b)-[:USES_ENTITY {purpose: "cta", temperature: 0.9}]->(c);

MATCH (b:Block {key: "block-home-cta"}), (c:Entity {key: "benefit-cost-effective"})
MERGE (b)-[:USES_ENTITY {purpose: "value_prop", temperature: 0.8}]->(c);

MATCH (b:Block {key: "block-pricing-table"}), (c:Entity {key: "tier-free"})
MERGE (b)-[:USES_ENTITY {purpose: "tier", temperature: 1.0}]->(c);

MATCH (b:Block {key: "block-pricing-table"}), (c:Entity {key: "tier-pro"})
MERGE (b)-[:USES_ENTITY {purpose: "tier", temperature: 1.0}]->(c);

MATCH (b:Block {key: "block-features-grid"}), (c:Entity {key: "feature-customization"})
MERGE (b)-[:USES_ENTITY {purpose: "feature", temperature: 0.9}]->(c);

MATCH (b:Block {key: "block-features-grid"}), (c:Entity {key: "feature-bulk"})
MERGE (b)-[:USES_ENTITY {purpose: "feature", temperature: 0.9}]->(c);

MATCH (b:Block {key: "block-features-grid"}), (c:Entity {key: "feature-dynamic"})
MERGE (b)-[:USES_ENTITY {purpose: "feature", temperature: 0.9}]->(c);

MATCH (b:Block {key: "block-features-grid"}), (c:Entity {key: "feature-templates"})
MERGE (b)-[:USES_ENTITY {purpose: "feature", temperature: 0.9}]->(c);

MATCH (b:Block {key: "block-usecases-grid"}), (c:Entity {key: "usecase-marketing"})
MERGE (b)-[:USES_ENTITY {purpose: "usecase", temperature: 0.9}]->(c);

MATCH (b:Block {key: "block-usecases-grid"}), (c:Entity {key: "usecase-restaurant"})
MERGE (b)-[:USES_ENTITY {purpose: "usecase", temperature: 0.9}]->(c);

MATCH (b:Block {key: "block-usecases-grid"}), (c:Entity {key: "usecase-retail"})
MERGE (b)-[:USES_ENTITY {purpose: "usecase", temperature: 0.9}]->(c);

MATCH (b:Block {key: "block-usecases-grid"}), (c:Entity {key: "usecase-events"})
MERGE (b)-[:USES_ENTITY {purpose: "usecase", temperature: 0.9}]->(c);

// =============================================================================
// SEO MINING DATA
// =============================================================================

MERGE (run:SEOMiningRun {key: "seo-run-2024-01"})
ON CREATE SET
  run.key = "seo-run-2024-01",
  run.display_name = "January 2024 SEO Mining",
  run.description = "Initial SEO keyword mining run for QR Code AI",
  run.llm_context = "USE: SEO mining context. TRIGGERS: seo run, mining. NOT: GEO mining.",
  run.status = "completed",
  run.started_at = datetime() - duration('P30D'),
  run.completed_at = datetime() - duration('P29D'),
  run.keywords_found = 50,
  run.keywords_qualified = 25,
  run.created_at = datetime(),
  run.updated_at = datetime();

// SEO Keywords - English
MATCH (c:Entity {key: "action-create-qr"})-[:HAS_L10N]->(cl:EntityL10n)-[:FOR_LOCALE]->(l:Locale {key: "en-US"})
MERGE (seo:SEOKeyword {key: "seo-qr-code-generator-en"})
ON CREATE SET
  seo.key = "seo-qr-code-generator-en",
  seo.display_name = "QR Code Generator",
  seo.description = "Primary SEO keyword for QR code creation",
  seo.llm_context = "USE: primary SEO target. TRIGGERS: qr code generator search. NOT: GEO queries.",
  seo.keyword = "qr code generator",
  seo.search_volume = 165000,
  seo.keyword_difficulty = 72,
  seo.cpc = 1.85,
  seo.search_intent = "transactional",
  seo.created_at = datetime(),
  seo.updated_at = datetime()
MERGE (cl)-[:HAS_SEO_TARGET]->(seo)
MERGE (seo)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "action-create-qr"})-[:HAS_L10N]->(cl:EntityL10n)-[:FOR_LOCALE]->(l:Locale {key: "en-US"})
MERGE (seo:SEOKeyword {key: "seo-free-qr-code-en"})
ON CREATE SET
  seo.key = "seo-free-qr-code-en",
  seo.display_name = "Free QR Code",
  seo.description = "High-volume keyword for free QR codes",
  seo.llm_context = "USE: free tier SEO. TRIGGERS: free qr code search. NOT: paid features.",
  seo.keyword = "free qr code",
  seo.search_volume = 110000,
  seo.keyword_difficulty = 65,
  seo.cpc = 1.20,
  seo.search_intent = "transactional",
  seo.created_at = datetime(),
  seo.updated_at = datetime()
MERGE (cl)-[:HAS_SEO_TARGET]->(seo)
MERGE (seo)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "usecase-restaurant"})-[:HAS_L10N]->(cl:EntityL10n)-[:FOR_LOCALE]->(l:Locale {key: "en-US"})
MERGE (seo:SEOKeyword {key: "seo-restaurant-qr-menu-en"})
ON CREATE SET
  seo.key = "seo-restaurant-qr-menu-en",
  seo.display_name = "Restaurant QR Menu",
  seo.description = "Long-tail keyword for restaurant use case",
  seo.llm_context = "USE: restaurant vertical SEO. TRIGGERS: restaurant menu qr. NOT: other industries.",
  seo.keyword = "qr code menu for restaurant",
  seo.search_volume = 18000,
  seo.keyword_difficulty = 45,
  seo.cpc = 2.10,
  seo.search_intent = "transactional",
  seo.created_at = datetime(),
  seo.updated_at = datetime()
MERGE (cl)-[:HAS_SEO_TARGET]->(seo)
MERGE (seo)-[:FOR_LOCALE]->(l);

// SEO Keywords - French
MATCH (c:Entity {key: "action-create-qr"})-[:HAS_L10N]->(cl:EntityL10n)-[:FOR_LOCALE]->(l:Locale {key: "fr-FR"})
MERGE (seo:SEOKeyword {key: "seo-generateur-qr-code-fr"})
ON CREATE SET
  seo.key = "seo-generateur-qr-code-fr",
  seo.display_name = "Generateur QR Code",
  seo.description = "Primary SEO keyword for French market",
  seo.llm_context = "USE: French SEO target. TRIGGERS: generateur qr code. NOT: English keywords.",
  seo.keyword = "generateur qr code",
  seo.search_volume = 40500,
  seo.keyword_difficulty = 58,
  seo.cpc = 0.95,
  seo.search_intent = "transactional",
  seo.created_at = datetime(),
  seo.updated_at = datetime()
MERGE (cl)-[:HAS_SEO_TARGET]->(seo)
MERGE (seo)-[:FOR_LOCALE]->(l);

MATCH (c:Entity {key: "action-create-qr"})-[:HAS_L10N]->(cl:EntityL10n)-[:FOR_LOCALE]->(l:Locale {key: "fr-FR"})
MERGE (seo:SEOKeyword {key: "seo-qr-code-gratuit-fr"})
ON CREATE SET
  seo.key = "seo-qr-code-gratuit-fr",
  seo.display_name = "QR Code Gratuit",
  seo.description = "Free tier keyword for French market",
  seo.llm_context = "USE: French free tier SEO. TRIGGERS: qr code gratuit. NOT: paid features.",
  seo.keyword = "qr code gratuit",
  seo.search_volume = 33100,
  seo.keyword_difficulty = 52,
  seo.cpc = 0.75,
  seo.search_intent = "transactional",
  seo.created_at = datetime(),
  seo.updated_at = datetime()
MERGE (cl)-[:HAS_SEO_TARGET]->(seo)
MERGE (seo)-[:FOR_LOCALE]->(l);

// SEO Metrics
MATCH (run:SEOMiningRun {key: "seo-run-2024-01"}), (seo:SEOKeyword {key: "seo-qr-code-generator-en"})
MERGE (run)-[:SEO_MINES]->(seo);

MATCH (run:SEOMiningRun {key: "seo-run-2024-01"}), (seo:SEOKeyword {key: "seo-free-qr-code-en"})
MERGE (run)-[:SEO_MINES]->(seo);

MATCH (run:SEOMiningRun {key: "seo-run-2024-01"}), (seo:SEOKeyword {key: "seo-restaurant-qr-menu-en"})
MERGE (run)-[:SEO_MINES]->(seo);

MATCH (run:SEOMiningRun {key: "seo-run-2024-01"}), (seo:SEOKeyword {key: "seo-generateur-qr-code-fr"})
MERGE (run)-[:SEO_MINES]->(seo);

MATCH (run:SEOMiningRun {key: "seo-run-2024-01"}), (seo:SEOKeyword {key: "seo-qr-code-gratuit-fr"})
MERGE (run)-[:SEO_MINES]->(seo);

MATCH (seo:SEOKeyword {key: "seo-qr-code-generator-en"})
MERGE (m:SEOKeywordMetrics {key: "seo-metrics-qr-gen-en-2024-01"})
ON CREATE SET
  m.key = "seo-metrics-qr-gen-en-2024-01",
  m.display_name = "QR Code Generator Metrics Jan 2024",
  m.description = "Monthly SEO metrics snapshot",
  m.llm_context = "USE: SEO performance tracking. TRIGGERS: metrics, ranking. NOT: GEO metrics.",
  m.recorded_at = datetime() - duration('P7D'),
  m.position = 12,
  m.impressions = 45000,
  m.clicks = 2800,
  m.ctr = 0.062,
  m.trend = "improving",
  m.created_at = datetime(),
  m.updated_at = datetime()
MERGE (seo)-[:HAS_METRICS]->(m);

// =============================================================================
// GENERATED OUTPUTS (PageL10n, BlockL10n)
// =============================================================================

MATCH (page:Page {key: "page-home"}), (l:Locale {key: "en-US"})
MERGE (pl:PageL10n {key: "pagel10n-home-en-v1"})
ON CREATE SET
  pl.key = "pagel10n-home-en-v1",
  pl.display_name = "Home Page - English v1",
  pl.description = "Generated English home page content",
  pl.llm_context = "USE: generated page content. TRIGGERS: home page output. NOT: source content.",
  pl.version = 1,
  pl.status = "published",
  pl.meta_title = "QR Code AI - Create Free QR Codes with Analytics",
  pl.meta_description = "Generate customizable QR codes instantly. Track scans, customize designs, and boost engagement. Free forever, no signup required.",
  pl.generated_at = datetime() - duration('P10D'),
  pl.published_at = datetime() - duration('P9D'),
  pl.word_count = 850,
  pl.created_at = datetime(),
  pl.updated_at = datetime()
MERGE (page)-[:HAS_OUTPUT]->(pl)
MERGE (pl)-[:FOR_LOCALE]->(l);

MATCH (page:Page {key: "page-home"}), (l:Locale {key: "fr-FR"})
MERGE (pl:PageL10n {key: "pagel10n-home-fr-v1"})
ON CREATE SET
  pl.key = "pagel10n-home-fr-v1",
  pl.display_name = "Page Accueil - Francais v1",
  pl.description = "Contenu page accueil genere en francais",
  pl.llm_context = "USE: contenu page genere. TRIGGERS: page accueil output. NOT: contenu source.",
  pl.version = 1,
  pl.status = "published",
  pl.meta_title = "QR Code AI - Creez des QR Codes Gratuits avec Analytics",
  pl.meta_description = "Generez des QR codes personnalisables instantanement. Suivez les scans, personnalisez les designs. Gratuit pour toujours.",
  pl.generated_at = datetime() - duration('P10D'),
  pl.published_at = datetime() - duration('P9D'),
  pl.word_count = 920,
  pl.created_at = datetime(),
  pl.updated_at = datetime()
MERGE (page)-[:HAS_OUTPUT]->(pl)
MERGE (pl)-[:FOR_LOCALE]->(l);

// BlockL10n for Home Hero
MATCH (b:Block {key: "block-home-hero"}), (l:Locale {key: "en-US"})
MERGE (bl:BlockL10n {key: "blockl10n-home-hero-en-v1"})
ON CREATE SET
  bl.key = "blockl10n-home-hero-en-v1",
  bl.display_name = "Home Hero - English v1",
  bl.description = "Generated hero block content",
  bl.llm_context = "USE: generated block content. TRIGGERS: hero output. NOT: source content.",
  bl.version = 1,
  bl.status = "published",
  bl.content = '{"headline": "Create QR Codes in Seconds", "subheadline": "Free, customizable, and trackable. No signup required.", "cta_primary": "Create Free QR Code", "cta_secondary": "See How It Works"}',
  bl.generated_at = datetime() - duration('P10D'),
  bl.word_count = 25,
  bl.created_at = datetime(),
  bl.updated_at = datetime()
MERGE (b)-[:HAS_OUTPUT]->(bl)
MERGE (bl)-[:FOR_LOCALE]->(l);

MATCH (b:Block {key: "block-home-hero"}), (l:Locale {key: "fr-FR"})
MERGE (bl:BlockL10n {key: "blockl10n-home-hero-fr-v1"})
ON CREATE SET
  bl.key = "blockl10n-home-hero-fr-v1",
  bl.display_name = "Hero Accueil - Francais v1",
  bl.description = "Contenu hero genere en francais",
  bl.llm_context = "USE: contenu bloc genere. TRIGGERS: hero output. NOT: contenu source.",
  bl.version = 1,
  bl.status = "published",
  bl.content = '{"headline": "Creez des QR Codes en Quelques Secondes", "subheadline": "Gratuit, personnalisable et tracable. Pas d inscription.", "cta_primary": "Creer un QR Code Gratuit", "cta_secondary": "Voir Comment Ca Marche"}',
  bl.generated_at = datetime() - duration('P10D'),
  bl.word_count = 28,
  bl.created_at = datetime(),
  bl.updated_at = datetime()
MERGE (b)-[:HAS_OUTPUT]->(bl)
MERGE (bl)-[:FOR_LOCALE]->(l);

// BlockL10n for Features Grid
MATCH (b:Block {key: "block-home-features"}), (l:Locale {key: "en-US"})
MERGE (bl:BlockL10n {key: "blockl10n-home-features-en-v1"})
ON CREATE SET
  bl.key = "blockl10n-home-features-en-v1",
  bl.display_name = "Home Features - English v1",
  bl.description = "Generated features block content",
  bl.llm_context = "USE: generated block content. TRIGGERS: features output. NOT: source content.",
  bl.version = 1,
  bl.status = "published",
  bl.content = '{"features": [{"icon": "wand", "title": "Instant Generation", "description": "Create QR codes in under 3 seconds"}, {"icon": "palette", "title": "Full Customization", "description": "Add colors, logos, and custom shapes"}, {"icon": "chart", "title": "Real-time Analytics", "description": "Track every scan with detailed insights"}, {"icon": "layers", "title": "Bulk Generation", "description": "Create hundreds of codes at once"}]}',
  bl.generated_at = datetime() - duration('P10D'),
  bl.word_count = 45,
  bl.created_at = datetime(),
  bl.updated_at = datetime()
MERGE (b)-[:HAS_OUTPUT]->(bl)
MERGE (bl)-[:FOR_LOCALE]->(l);

// BlockL10n for Pricing Table
MATCH (b:Block {key: "block-pricing-table"}), (l:Locale {key: "en-US"})
MERGE (bl:BlockL10n {key: "blockl10n-pricing-table-en-v1"})
ON CREATE SET
  bl.key = "blockl10n-pricing-table-en-v1",
  bl.display_name = "Pricing Table - English v1",
  bl.description = "Generated pricing table content",
  bl.llm_context = "USE: generated block content. TRIGGERS: pricing output. NOT: source content.",
  bl.version = 1,
  bl.status = "published",
  bl.content = '{"tiers": [{"name": "Free", "price": "$0", "period": "forever", "features": ["Unlimited static QR codes", "Basic customization", "7-day scan history"], "cta": "Get Started Free"}, {"name": "Pro", "price": "$9", "period": "/month", "features": ["Everything in Free", "Dynamic QR codes", "Advanced analytics", "Bulk generation", "API access"], "cta": "Start Pro Trial", "highlighted": true}]}',
  bl.generated_at = datetime() - duration('P8D'),
  bl.word_count = 60,
  bl.created_at = datetime(),
  bl.updated_at = datetime()
MERGE (b)-[:HAS_OUTPUT]->(bl)
MERGE (bl)-[:FOR_LOCALE]->(l);

// =============================================================================
// PROMPTS
// =============================================================================

MATCH (b:Block {key: "block-home-hero"})
MERGE (bp:BlockPrompt {key: "blockprompt-home-hero-v1"})
ON CREATE SET
  bp.key = "blockprompt-home-hero-v1",
  bp.display_name = "Home Hero Prompt v1.0",
  bp.description = "Generation instructions for home hero",
  bp.llm_context = "USE: when generating home hero content. TRIGGERS: hero, headline, cta. NOT: features or pricing blocks.",
  bp.prompt = "Generate a compelling hero section. Headline should emphasize instant QR code creation. Subheadline highlights free + no signup. Include primary and secondary CTAs.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-home-features"})
MERGE (bp:BlockPrompt {key: "blockprompt-home-features-v1"})
ON CREATE SET
  bp.key = "blockprompt-home-features-v1",
  bp.display_name = "Home Features Prompt v1.0",
  bp.description = "Generation instructions for features grid",
  bp.llm_context = "USE: when generating features grid content. TRIGGERS: features, cards, grid. NOT: hero or CTA blocks.",
  bp.prompt = "Generate 4 feature cards highlighting: instant creation, customization, analytics, and bulk generation. Each card needs icon name, title (3-4 words), description (8-12 words).",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-home-stats"})
MERGE (bp:BlockPrompt {key: "blockprompt-home-stats-v1"})
ON CREATE SET
  bp.key = "blockprompt-home-stats-v1",
  bp.display_name = "Home Stats Prompt v1.0",
  bp.description = "Generation instructions for stats section",
  bp.llm_context = "USE: when generating stats content. TRIGGERS: stats, metrics, numbers. NOT: testimonials.",
  bp.prompt = "Generate 4 impressive statistics: total QR codes created (millions), scans tracked (millions), happy customers (thousands), satisfaction rate (percentage).",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-home-testimonials"})
MERGE (bp:BlockPrompt {key: "blockprompt-home-testimonials-v1"})
ON CREATE SET
  bp.key = "blockprompt-home-testimonials-v1",
  bp.display_name = "Home Testimonials Prompt v1.0",
  bp.description = "Generation instructions for testimonials",
  bp.llm_context = "USE: when generating testimonials. TRIGGERS: testimonials, reviews, quotes. NOT: stats.",
  bp.prompt = "Generate 3 testimonials from: marketing manager, restaurant owner, event coordinator. Each with name, title, company, photo URL placeholder, and 2-sentence quote.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-home-cta"})
MERGE (bp:BlockPrompt {key: "blockprompt-home-cta-v1"})
ON CREATE SET
  bp.key = "blockprompt-home-cta-v1",
  bp.display_name = "Home CTA Prompt v1.0",
  bp.description = "Generation instructions for CTA banner",
  bp.llm_context = "USE: when generating CTA content. TRIGGERS: cta, action, signup. NOT: features.",
  bp.prompt = "Generate a compelling CTA banner. Strong headline emphasizing free tier and value. Primary button for signup, secondary for learn more.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-pricing-table"})
MERGE (bp:BlockPrompt {key: "blockprompt-pricing-table-v1"})
ON CREATE SET
  bp.key = "blockprompt-pricing-table-v1",
  bp.display_name = "Pricing Table Prompt v1.0",
  bp.description = "Generation instructions for pricing table",
  bp.llm_context = "USE: when generating pricing content. TRIGGERS: pricing, tiers, comparison. NOT: features.",
  bp.prompt = "Generate pricing comparison for Free and Pro tiers. Highlight Pro as recommended. Include feature checkmarks, pricing, and CTAs for each tier.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-pricing-faq"})
MERGE (bp:BlockPrompt {key: "blockprompt-pricing-faq-v1"})
ON CREATE SET
  bp.key = "blockprompt-pricing-faq-v1",
  bp.display_name = "Pricing FAQ Prompt v1.0",
  bp.description = "Generation instructions for pricing FAQ",
  bp.llm_context = "USE: when generating FAQ content. TRIGGERS: faq, questions. NOT: features.",
  bp.prompt = "Generate 5-7 FAQ items about pricing: upgrade process, refunds, enterprise options, payment methods, billing cycles.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-features-hero"})
MERGE (bp:BlockPrompt {key: "blockprompt-features-hero-v1"})
ON CREATE SET
  bp.key = "blockprompt-features-hero-v1",
  bp.display_name = "Features Hero Prompt v1.0",
  bp.description = "Generation instructions for features hero",
  bp.llm_context = "USE: when generating features page hero. TRIGGERS: features, hero. NOT: pricing.",
  bp.prompt = "Generate an engaging features page header. Headline about powerful capabilities, subheadline about ease of use.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-features-grid"})
MERGE (bp:BlockPrompt {key: "blockprompt-features-grid-v1"})
ON CREATE SET
  bp.key = "blockprompt-features-grid-v1",
  bp.display_name = "Features Grid Prompt v1.0",
  bp.description = "Generation instructions for full features grid",
  bp.llm_context = "USE: when generating full features grid. TRIGGERS: features, grid. NOT: hero.",
  bp.prompt = "Generate 6 feature cards covering: customization, analytics, bulk generation, dynamic QR, templates, integrations. Each with icon, title, description.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-features-testimonials"})
MERGE (bp:BlockPrompt {key: "blockprompt-features-testimonials-v1"})
ON CREATE SET
  bp.key = "blockprompt-features-testimonials-v1",
  bp.display_name = "Features Testimonials Prompt v1.0",
  bp.description = "Generation instructions for feature testimonials",
  bp.llm_context = "USE: when generating feature testimonials. TRIGGERS: testimonials, features. NOT: pricing.",
  bp.prompt = "Generate 3 testimonials from marketing professionals highlighting feature benefits: analytics, customization, bulk generation.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-usecases-hero"})
MERGE (bp:BlockPrompt {key: "blockprompt-usecases-hero-v1"})
ON CREATE SET
  bp.key = "blockprompt-usecases-hero-v1",
  bp.display_name = "Use Cases Hero Prompt v1.0",
  bp.description = "Generation instructions for use cases hero",
  bp.llm_context = "USE: when generating use cases hero. TRIGGERS: use cases, industries. NOT: features.",
  bp.prompt = "Generate use cases page header. Emphasize versatility across industries. List key verticals in subheadline.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

MATCH (b:Block {key: "block-usecases-grid"})
MERGE (bp:BlockPrompt {key: "blockprompt-usecases-grid-v1"})
ON CREATE SET
  bp.key = "blockprompt-usecases-grid-v1",
  bp.display_name = "Use Cases Grid Prompt v1.0",
  bp.description = "Generation instructions for use cases grid",
  bp.llm_context = "USE: when generating use cases grid. TRIGGERS: industries, use cases. NOT: features.",
  bp.prompt = "Generate 4 use case cards: marketing campaigns, restaurant menus, retail products, event tickets. Each with icon, title, brief description, and example.",
  bp.version = "1.0",
  bp.active = true,
  bp.created_at = datetime(),
  bp.updated_at = datetime()
MERGE (b)-[:HAS_PROMPT]->(bp);

// Page Prompts
MATCH (p:Page {key: "page-home"})
MERGE (pp:PagePrompt {key: "pageprompt-home-v1"})
ON CREATE SET
  pp.key = "pageprompt-home-v1",
  pp.display_name = "Home Page Prompt v1.0",
  pp.description = "Generation instructions for home page",
  pp.llm_context = "USE: when generating home page content. TRIGGERS: home, landing. NOT: subpages.",
  pp.prompt = "Generate a high-converting home page. Focus on value proposition (free, fast, customizable), trust signals, and clear CTAs. Maintain consistent brand voice.",
  pp.version = "1.0",
  pp.active = true,
  pp.created_at = datetime(),
  pp.updated_at = datetime()
MERGE (p)-[:HAS_PROMPT]->(pp);

MATCH (p:Page {key: "page-features"})
MERGE (pp:PagePrompt {key: "pageprompt-features-v1"})
ON CREATE SET
  pp.key = "pageprompt-features-v1",
  pp.display_name = "Features Page Prompt v1.0",
  pp.description = "Generation instructions for features page",
  pp.llm_context = "USE: when generating features page. TRIGGERS: features, capabilities. NOT: pricing.",
  pp.prompt = "Generate a comprehensive features page. Cover all capabilities with clear benefits. Use comparison with competitors where appropriate.",
  pp.version = "1.0",
  pp.active = true,
  pp.created_at = datetime(),
  pp.updated_at = datetime()
MERGE (p)-[:HAS_PROMPT]->(pp);

MATCH (p:Page {key: "page-use-cases"})
MERGE (pp:PagePrompt {key: "pageprompt-usecases-v1"})
ON CREATE SET
  pp.key = "pageprompt-usecases-v1",
  pp.display_name = "Use Cases Page Prompt v1.0",
  pp.description = "Generation instructions for use cases page",
  pp.llm_context = "USE: when generating use cases page. TRIGGERS: use cases, industries. NOT: features.",
  pp.prompt = "Generate a use cases hub page. Showcase diverse industries. Include real-world examples and success metrics. Link to detailed use case pages.",
  pp.version = "1.0",
  pp.active = true,
  pp.created_at = datetime(),
  pp.updated_at = datetime()
MERGE (p)-[:HAS_PROMPT]->(pp);

// =============================================================================
// BLOCK RULES
// =============================================================================

MATCH (bt:BlockType {key: "blocktype-features-grid"})
MERGE (br:BlockRules {key: "blockrules-features-grid-v1"})
ON CREATE SET
  br.key = "blockrules-features-grid-v1",
  br.display_name = "Features Grid Rules v1.0",
  br.description = "Generation rules for features grid",
  br.llm_context = "USE: when validating features grid output. TRIGGERS: features, grid. NOT: hero rules.",
  br.rules = "Display 3-6 features in responsive grid. Each feature must have: icon (Lucide icon name), title (max 4 words), description (max 15 words). Use benefit-focused language.",
  br.version = "1.0",
  br.active = true,
  br.created_at = datetime(),
  br.updated_at = datetime()
MERGE (bt)-[:HAS_RULES]->(br);

MATCH (bt:BlockType {key: "blocktype-pricing-table"})
MERGE (br:BlockRules {key: "blockrules-pricing-table-v1"})
ON CREATE SET
  br.key = "blockrules-pricing-table-v1",
  br.display_name = "Pricing Table Rules v1.0",
  br.description = "Generation rules for pricing table",
  br.llm_context = "USE: when validating pricing table output. TRIGGERS: pricing, tiers. NOT: features rules.",
  br.rules = "Show all tiers side by side. Highlight recommended tier with visual emphasis. List features with checkmarks. Include clear CTA for each tier.",
  br.version = "1.0",
  br.active = true,
  br.created_at = datetime(),
  br.updated_at = datetime()
MERGE (bt)-[:HAS_RULES]->(br);

MATCH (bt:BlockType {key: "blocktype-testimonials"})
MERGE (br:BlockRules {key: "blockrules-testimonials-v1"})
ON CREATE SET
  br.key = "blockrules-testimonials-v1",
  br.display_name = "Testimonials Rules v1.0",
  br.description = "Generation rules for testimonials",
  br.llm_context = "USE: when validating testimonials output. TRIGGERS: testimonials, reviews. NOT: stats rules.",
  br.rules = "Display 2-4 testimonials. Each must have: photo (placeholder URL), full name, job title, company name, quote (2-3 sentences).",
  br.version = "1.0",
  br.active = true,
  br.created_at = datetime(),
  br.updated_at = datetime()
MERGE (bt)-[:HAS_RULES]->(br);

MATCH (bt:BlockType {key: "blocktype-faq"})
MERGE (br:BlockRules {key: "blockrules-faq-v1"})
ON CREATE SET
  br.key = "blockrules-faq-v1",
  br.display_name = "FAQ Rules v1.0",
  br.description = "Generation rules for FAQ section",
  br.llm_context = "USE: when validating FAQ output. TRIGGERS: faq, questions. NOT: testimonials rules.",
  br.rules = "Collapsible Q&A format. 5-10 questions. Questions end with ?. Answers 2-4 sentences. Include Schema.org FAQ markup hints.",
  br.version = "1.0",
  br.active = true,
  br.created_at = datetime(),
  br.updated_at = datetime()
MERGE (bt)-[:HAS_RULES]->(br);

MATCH (bt:BlockType {key: "blocktype-cta-banner"})
MERGE (br:BlockRules {key: "blockrules-cta-banner-v1"})
ON CREATE SET
  br.key = "blockrules-cta-banner-v1",
  br.display_name = "CTA Banner Rules v1.0",
  br.description = "Generation rules for CTA banner",
  br.llm_context = "USE: when validating CTA banner output. TRIGGERS: cta, action. NOT: hero rules.",
  br.rules = "Strong headline (5-8 words). Supporting text (15-25 words). Primary button (action verb + benefit). Optional secondary button.",
  br.version = "1.0",
  br.active = true,
  br.created_at = datetime(),
  br.updated_at = datetime()
MERGE (bt)-[:HAS_RULES]->(br);

MATCH (bt:BlockType {key: "blocktype-stats"})
MERGE (br:BlockRules {key: "blockrules-stats-v1"})
ON CREATE SET
  br.key = "blockrules-stats-v1",
  br.display_name = "Stats Rules v1.0",
  br.description = "Generation rules for statistics section",
  br.llm_context = "USE: when validating stats output. TRIGGERS: stats, metrics. NOT: testimonials rules.",
  br.rules = "Display 3-4 key statistics. Large numbers with context labels. Use commas for thousands. Include + or % symbols where appropriate.",
  br.version = "1.0",
  br.active = true,
  br.created_at = datetime(),
  br.updated_at = datetime()
MERGE (bt)-[:HAS_RULES]->(br);

// =============================================================================
// VERIFICATION QUERIES
// =============================================================================
// Run these to verify the seed worked:
// MATCH (n) RETURN labels(n)[0] AS type, count(*) AS count ORDER BY count DESC;
// MATCH ()-[r]->() RETURN type(r) AS type, count(*) AS count ORDER BY count DESC;
// MATCH (c:Entity)-[:HAS_L10N]->(cl:EntityL10n)-[:FOR_LOCALE]->(l:Locale) RETURN c.key, l.key, cl.display_name LIMIT 20;
