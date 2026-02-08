// ===================================================================
// EntityL10n fr-FR for QR Code AI (281 entities)
// Generated: 2026-02-08
// ===================================================================

// -------------------------------------------------------------------
// Create EntityL10n nodes
// -------------------------------------------------------------------

MERGE (el:EntityL10n {entity_key: 'qr-code', locale_key: 'fr-FR'})
SET el.slug = 'qr-code',
    el.display_name = 'QR Code',
    el.description = 'Code matriciel 2D permettant d\'encoder des données accessibles par scan mobile.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes, scanning, 2D barcodes, quick response codes. DECLENCHEURS: qr, qr code, qr-code, scan code, 2d barcode, matrix code. EXCLURE: barcode 1D (utiliser Barcode), data matrix (utiliser Data Matrix), link shortener without QR (utiliser Smart Link).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'smart-link', locale_key: 'fr-FR'})
SET el.slug = 'lien-intelligent',
    el.display_name = 'Lien Intelligent',
    el.description = 'URL intelligente avec règles de routage pour rediriger selon l\'appareil ou la localisation.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Lien Intelligent.',
    el.purpose = 'Optimisez vos campagnes marketing avec Lien Intelligent personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de intelligent URLs, link routing, device targeting, geo-targeting links. DECLENCHEURS: smart link, intelligent url, routing link, conditional redirect, targeted link. EXCLURE: basic short URL (utiliser Short Link), QR code (utiliser QR Code), landing page (utiliser Landing Page).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'barcode', locale_key: 'fr-FR'})
SET el.slug = 'code-barres',
    el.display_name = 'Code-barres',
    el.description = 'Code-barres linéaire 1D pour l\'identification des produits (EAN, UPC, Code 128).',
    el.definition = 'Solution QR Code AI pour la création et gestion de Code-barres.',
    el.purpose = 'Optimisez vos campagnes marketing avec Code-barres personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de 1D barcodes, linear barcodes, retail barcodes, product codes. DECLENCHEURS: barcode, 1d barcode, ean, upc, code 128, linear barcode, product code. EXCLURE: QR code (utiliser QR Code), 2D codes (utiliser QR Code or Data Matrix).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'landing-page', locale_key: 'fr-FR'})
SET el.slug = 'page-destination',
    el.display_name = 'Page de Destination',
    el.description = 'Page web de destination créée via un constructeur no-code intégré.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Page de Destination.',
    el.purpose = 'Optimisez vos campagnes marketing avec Page de Destination personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de destination pages, page builder, no-code pages, mobile pages. DECLENCHEURS: landing page, destination page, page builder, mobile page, microsite. EXCLURE: full website (external), QR code itself (utiliser QR Code), link shortener (utiliser Smart Link).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'short-link', locale_key: 'fr-FR'})
SET el.slug = 'lien-court',
    el.display_name = 'Lien Court',
    el.description = 'URL raccourcie avec suivi des clics et statistiques de performance.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Lien Court.',
    el.purpose = 'Optimisez vos campagnes marketing avec Lien Court personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de URL shortening, link tracking, shortened URLs as technology. DECLENCHEURS: short link, shortened url, url shortener, link shortening, tiny url. EXCLURE: smart routing (utiliser Smart Link), QR code (utiliser QR Code), vanity URL only (mention custom domain).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-style', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-style',
    el.display_name = 'QR Code Style',
    el.description = 'Catégorie de style visuel pour personnaliser l\'apparence des QR Codes.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code Style.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code Style personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR code visual approaches, style categories, design types. DECLENCHEURS: qr style, qr code style, visual style, design approach. EXCLURE: specific styles (utiliser Custom QR, QR Art, etc.), colors only (utiliser QR Code Colors).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-content', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-contenu',
    el.display_name = 'QR Code Contenu',
    el.description = 'Catégorie de contenu définissant le type de données encodées dans un QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code Contenu.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code Contenu personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR code data types, what QR codes encode, content categories. DECLENCHEURS: qr content, content type, qr data, what to encode. EXCLURE: specific content types (utiliser URL QR, WiFi QR, etc.), QR appearance (utiliser QR Code Style).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-frame', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-cadre',
    el.display_name = 'QR Code Cadre',
    el.description = 'Modèle de placement physique optimisé pour l\'impression des QR Codes.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code Cadre.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code Cadre personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR code templates, physical placement, print templates. DECLENCHEURS: qr frame, template, placement, print size, frame template. EXCLURE: specific frames (utiliser Business Card QR, Poster QR, etc.), digital only (utiliser Landing Page).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'barcode-format', locale_key: 'fr-FR'})
SET el.slug = 'code-barres-format',
    el.display_name = 'Code-barres Format',
    el.description = 'Format technique de code-barres définissant la structure d\'encodage.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Code-barres Format.',
    el.purpose = 'Optimisez vos campagnes marketing avec Code-barres Format personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de barcode standards, encoding formats, barcode types. DECLENCHEURS: barcode format, barcode type, barcode standard, encoding format. EXCLURE: specific formats (utiliser EAN-13, UPC-A, etc.), QR codes (utiliser QR Code).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'landing-page-type', locale_key: 'fr-FR'})
SET el.slug = 'page-destination-type',
    el.display_name = 'Page de Destination Type',
    el.description = 'Type de page de destination adapté à un cas d\'usage spécifique.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Page de Destination Type.',
    el.purpose = 'Optimisez vos campagnes marketing avec Page de Destination Type personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de landing page templates, page types, utiliser case templates. DECLENCHEURS: landing page type, page template, page category, template type. EXCLURE: specific types (utiliser Link in Bio, Digital Menu, etc.), external websites.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'custom-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'personnalise-qr-code',
    el.display_name = 'QR Code Personnalisé',
    el.description = 'QR Code entièrement personnalisable avec couleurs, formes et logo.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code Personnalisé.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code Personnalisé personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de fully customizable QR codes, parametric design, manual customization. DECLENCHEURS: custom qr, customize qr, design qr, parametric qr, branded qr. EXCLURE: AI-generated (utiliser QR Code Art), photo overlay (utiliser QR Code with Image).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-image', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-image',
    el.display_name = 'QR Code avec Image',
    el.description = 'QR Code avec image ou photo en arrière-plan.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code avec Image.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code avec Image personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes with photos, image overlays, background images on QR. DECLENCHEURS: qr with image, qr photo, image qr, photo background qr. EXCLURE: AI art (utiliser QR Code Art), logo only (utiliser QR Code with Logo).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-art', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-art',
    el.display_name = 'QR Code Art',
    el.description = 'QR Code artistique généré par intelligence artificielle.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code Art.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code Art personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de AI QR codes, artistic QR generation, creative AI QR. DECLENCHEURS: qr art, ai qr, artistic qr, ai generated qr, creative qr. EXCLURE: manual design (utiliser Custom QR Code), photo overlay (utiliser QR Code with Image).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-photo', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-photo',
    el.display_name = 'QR Code Photo',
    el.description = 'QR Code intégrant une photographie en fond.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code Photo.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code Photo personnalisés.',
    el.llm_context = 'UTILISER: quand l\'utilisateur says \'QR photo\' specifically, rediriger vers QR Code with Image. DECLENCHEURS: qr photo, photo qr code. EXCLURE: primary term (utiliser QR Code with Image plutôt).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-with-logo', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-avec-logo',
    el.display_name = 'QR Code avec Logo',
    el.description = 'QR Code affichant un logo au centre pour renforcer l\'identité de marque.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code avec Logo.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code avec Logo personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de adding logos to QR codes, branded QR with logo, center logo. DECLENCHEURS: qr with logo, add logo, logo qr, branded qr, center logo. EXCLURE: full custom design (utiliser Custom QR Code), background image (utiliser QR Code with Image).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-with-text', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-avec-texte',
    el.display_name = 'QR Code avec Textee',
    el.description = 'QR Code accompagné d\'un texte d\'appel à l\'action.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code avec Textee.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code avec Textee personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de adding text to QR codes, CTA text, scan me text. DECLENCHEURS: qr with text, add text, scan me, call to action, text qr. EXCLURE: encoded text content (utiliser Text QR), logo (utiliser QR Code with Logo).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-color', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-color',
    el.display_name = 'QR Code Couleurs',
    el.description = 'Personnalisation des couleurs de premier plan et d\'arrière-plan du QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code Couleurs.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code Couleurs personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR code colors, color customization, foreground/background. DECLENCHEURS: qr color, colored qr, change color, qr colors, color scheme. EXCLURE: shapes (utiliser QR Code Shapes), gradients specifically (utiliser Background Gradient).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-shapes', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-formes',
    el.display_name = 'QR Code Formes',
    el.description = 'Personnalisation des formes des modules et des yeux du QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code Formes.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code Formes personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR module shapes, dot patterns, eye patterns, shape customization. DECLENCHEURS: qr shapes, module shape, dot pattern, eye pattern, rounded qr. EXCLURE: colors (utiliser QR Code Colors), logo (utiliser QR Code with Logo).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-transparent-background', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-fond-transparent',
    el.display_name = 'Fond Transparent',
    el.description = 'QR Code avec fond transparent pour superposition sur d\'autres visuels.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Fond Transparent.',
    el.purpose = 'Optimisez vos campagnes marketing avec Fond Transparent personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de transparent QR codes, PNG with alpha, overlay QR. DECLENCHEURS: transparent qr, transparent background, png alpha, overlay qr, no background. EXCLURE: white background (default), image background (utiliser Background Image).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-background', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-background',
    el.display_name = 'QR Code Arrière-plan',
    el.description = 'Options de personnalisation de l\'arrière-plan des QR Codes.',
    el.definition = 'Solution QR Code AI pour la création et gestion de QR Code Arrière-plan.',
    el.purpose = 'Optimisez vos campagnes marketing avec QR Code Arrière-plan personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR background options generally, background customization category. DECLENCHEURS: qr background, background options, background type. EXCLURE: specific types (utiliser Background Color, Gradient, or Image).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-background-color', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-couleur-fond',
    el.display_name = 'Couleur de Fond',
    el.description = 'Couleur d\'arrière-plan unie pour les QR Codes.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Couleur de Fond.',
    el.purpose = 'Optimisez vos campagnes marketing avec Couleur de Fond personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de solid color backgrounds for QR codes. DECLENCHEURS: background color, solid background, fill color. EXCLURE: gradient (utiliser Background Gradient), image (utiliser Background Image).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-background-gradient', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-fond-degrade',
    el.display_name = 'Fond en Dégradé',
    el.description = 'Dégradé de couleurs en arrière-plan des QR Codes.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Fond en Dégradé.',
    el.purpose = 'Optimisez vos campagnes marketing avec Fond en Dégradé personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de gradient backgrounds for QR codes. DECLENCHEURS: gradient background, color gradient, gradient qr. EXCLURE: solid color (utiliser Background Color), image (utiliser Background Image).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-background-image', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-image-fond',
    el.display_name = 'Image de Fond',
    el.description = 'Image ou motif en arrière-plan des QR Codes.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Image de Fond.',
    el.purpose = 'Optimisez vos campagnes marketing avec Image de Fond personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de image backgrounds for QR codes, patterns, textures. DECLENCHEURS: background image, image background, pattern background, texture. EXCLURE: QR Code with Image style (different feature), solid color (utiliser Background Color).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'dynamic-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'dynamique-qr-code',
    el.display_name = 'QR Code Dynamique',
    el.description = 'QR Code modifiable après impression dont la destination peut être mise à jour sans régénérer le code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les QR Code Dynamique.',
    el.purpose = 'Exploitez les avantages des QR Code Dynamique pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de editable QR codes, trackable QR, changeable destination. DECLENCHEURS: dynamic qr, editable qr, trackable qr, change destination, update qr. EXCLURE: fixed content (utiliser Static QR Code), smart routing (utiliser Smart Link).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'static-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'statique-qr-code',
    el.display_name = 'QR Code Statique',
    el.description = 'QR Code fixe avec données encodées directement, gratuit et permanent.',
    el.definition = 'Concept clé pour comprendre et maîtriser les QR Code Statique.',
    el.purpose = 'Exploitez les avantages des QR Code Statique pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de fixed QR codes, permanent QR, direct encoding. DECLENCHEURS: static qr, fixed qr, permanent qr, free qr, direct encode. EXCLURE: editable (utiliser Dynamic QR Code), trackable (utiliser Dynamic QR Code).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-light-mode', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-light-mode',
    el.display_name = 'Light Mode',
    el.description = 'La Light Mode pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les Light Mode.',
    el.purpose = 'Exploitez les avantages des Light Mode pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de light theme QR codes, standard contrast QR. DECLENCHEURS: light mode, light theme, light background, standard qr. EXCLURE: dark theme (utiliser Dark Mode), inverted colors.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-dark-mode', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-dark-mode',
    el.display_name = 'Dark Mode',
    el.description = 'La Dark Mode pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les Dark Mode.',
    el.purpose = 'Exploitez les avantages des Dark Mode pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de dark theme QR codes, inverted QR, night mode. DECLENCHEURS: dark mode, dark theme, dark background, inverted qr, night mode. EXCLURE: light theme (utiliser Light Mode), standard appearance.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-business-card', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-carte-visite',
    el.display_name = 'Carte de Visite QR',
    el.description = 'La Carte de Visite QR pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Carte de Visite QR.',
    el.purpose = 'Optimisez vos campagnes marketing avec Carte de Visite QR personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for business cards, professional networking QR. DECLENCHEURS: business card qr, card qr, professional qr, networking qr. EXCLURE: email signature (utiliser Email Signature QR), vCard content (utiliser vCard QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-email-signature', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-email-signature',
    el.display_name = 'Email Signature QR',
    el.description = 'L\'Email Signature QR pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Email Signature QR.',
    el.purpose = 'Optimisez vos campagnes marketing avec Email Signature QR personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for email signatures, small signature QR. DECLENCHEURS: email signature qr, signature qr, email qr. EXCLURE: business card (utiliser Business Card QR), contact form (utiliser Forms).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-flyer', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-flyer',
    el.display_name = 'Flyer QR',
    el.description = 'Le Flyer QR pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Flyer QR.',
    el.purpose = 'Optimisez vos campagnes marketing avec Flyer QR personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for flyers, promotional print QR. DECLENCHEURS: flyer qr, handout qr, promotional qr, print qr. EXCLURE: poster size (utiliser Poster QR), product packaging (utiliser Packaging Label QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-poster', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-poster',
    el.display_name = 'Affiche QR',
    el.description = 'L\'Affiche QR pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Affiche QR.',
    el.purpose = 'Optimisez vos campagnes marketing avec Affiche QR personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for posters, large format QR, billboard QR. DECLENCHEURS: poster qr, billboard qr, large qr, high resolution qr. EXCLURE: flyer size (utiliser Flyer QR), table display (utiliser Table Tent QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-table-tent', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-table-tent',
    el.display_name = 'Table Tent QR',
    el.description = 'La Table Tent QR pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Table Tent QR.',
    el.purpose = 'Optimisez vos campagnes marketing avec Table Tent QR personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for table displays, restaurant table QR. DECLENCHEURS: table tent qr, table qr, restaurant qr, menu qr. EXCLURE: digital menu content (utiliser Digital Menu), poster (utiliser Poster QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-packaging-label', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-packaging-label',
    el.display_name = 'Emballage Label QR',
    el.description = 'L\'Emballage Label QR pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Emballage Label QR.',
    el.purpose = 'Optimisez vos campagnes marketing avec Emballage Label QR personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for product packaging, label QR, product QR. DECLENCHEURS: packaging qr, label qr, product qr, package qr. EXCLURE: retail barcode (utiliser Barcode), poster (utiliser Poster QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'link-in-bio', locale_key: 'fr-FR'})
SET el.slug = 'lien-en-bio',
    el.display_name = 'Lien en Bio',
    el.description = 'Le Lien en Bio pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Lien en Bio.',
    el.purpose = 'Optimisez vos campagnes marketing avec Lien en Bio personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de bio link pages, social media link aggregation, creator links. DECLENCHEURS: link in bio, bio link, linktree alternative, social links, creator page. EXCLURE: full landing page (utiliser Landing Page), single URL (utiliser Short Link).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'menu-restaurant', locale_key: 'fr-FR'})
SET el.slug = 'menu-restaurant',
    el.display_name = 'Menu Digital',
    el.description = 'Le Menu Digital pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Menu Digital.',
    el.purpose = 'Optimisez vos campagnes marketing avec Menu Digital personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de restaurant menus, digital menus, QR menus, contactless menus. DECLENCHEURS: digital menu, restaurant menu, qr menu, contactless menu, menu page. EXCLURE: table tent template (utiliser Table Tent QR), generic landing page.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'forms', locale_key: 'fr-FR'})
SET el.slug = 'forms',
    el.display_name = 'Forms',
    el.description = 'Les Forms pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Forms.',
    el.purpose = 'Optimisez vos campagnes marketing avec Forms personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de contact forms, lead capture, form pages, data collection. DECLENCHEURS: form, contact form, lead form, survey, registration form. EXCLURE: event RSVP specifically (utiliser Event RSVP), booking (utiliser Booking/Appointment).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'announcement', locale_key: 'fr-FR'})
SET el.slug = 'announcement',
    el.display_name = 'Announcement',
    el.description = 'L\'Announcement pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Announcement.',
    el.purpose = 'Optimisez vos campagnes marketing avec Announcement personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de announcement pages, message pages, notification pages. DECLENCHEURS: announcement, message page, notification, alert page, info page. EXCLURE: event invitation (utiliser Event RSVP), ongoing content (utiliser Link in Bio).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'event-rsvp', locale_key: 'fr-FR'})
SET el.slug = 'event-rsvp',
    el.display_name = 'Événement RSVP',
    el.description = 'Le Événement RSVP pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Événement RSVP.',
    el.purpose = 'Optimisez vos campagnes marketing avec Événement RSVP personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de event registration, RSVP pages, guest management. DECLENCHEURS: event rsvp, event registration, rsvp page, guest list, invitation. EXCLURE: general forms (utiliser Forms), booking slots (utiliser Booking/Appointment).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'booking-appointment', locale_key: 'fr-FR'})
SET el.slug = 'booking-appointment',
    el.display_name = 'Booking/Applicationointment',
    el.description = 'La Booking/Applicationointment pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Booking/Applicationointment.',
    el.purpose = 'Optimisez vos campagnes marketing avec Booking/Applicationointment personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de appointment booking, scheduling pages, reservation systems. DECLENCHEURS: booking, appointment, schedule, reservation, calendar booking. EXCLURE: event RSVP (utiliser Event RSVP), contact form (utiliser Forms).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-url', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-url',
    el.display_name = 'QR Code URL',
    el.description = 'QR Code redirigeant vers une adresse web.',
    el.definition = 'Format de données QR Code URL compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes QR Code URL pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de QR codes linking to websites, URL encoding. DECLENCHEURS: url qr, website qr, link qr, web qr. EXCLURE: specific platforms (utiliser Instagram QR, YouTube QR, etc.), WiFi (utiliser WiFi QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-wifi', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-wifi',
    el.display_name = 'QR Code WiFi',
    el.description = 'QR Code pour connexion WiFi automatique.',
    el.definition = 'Format de données QR Code WiFi compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes QR Code WiFi pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de WiFi sharing, network credentials in QR. DECLENCHEURS: wifi qr, wireless qr, network qr, wifi password qr. EXCLURE: hotspot login page (utiliser URL QR), Bluetooth.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-vcard', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-vcard',
    el.display_name = 'vCard QR Code',
    el.description = 'QR Code contenant une carte de visite électronique.',
    el.definition = 'Format de données vCard QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes vCard QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de digital business cards, contact sharing QR. DECLENCHEURS: vcard qr, contact qr, business card qr, digital card. EXCLURE: MeCard (utiliser MeCard QR), LinkedIn profile (utiliser LinkedIn QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-mecard', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-mecard',
    el.display_name = 'MeCard QR Code',
    el.description = 'Le MeCard QR Code pour vos projets QR Code.',
    el.definition = 'Format de données MeCard QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes MeCard QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Japanese contact format, compact contact QR. DECLENCHEURS: mecard, mecard qr, japanese contact qr. EXCLURE: vCard (utiliser vCard QR), standard contact.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-pdf', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-pdf',
    el.display_name = 'PDF QR Code',
    el.description = 'QR Code donnant accès à un document PDF.',
    el.definition = 'Format de données PDF QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes PDF QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de PDF links, document QR codes. DECLENCHEURS: pdf qr, document qr, brochure qr, manual qr. EXCLURE: generic file (utiliser File Download QR), image gallery (utiliser Image Gallery QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-text', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-text',
    el.display_name = 'Texte QR Code',
    el.description = 'QR Code encodant du texte brut.',
    el.definition = 'Format de données Texte QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Texte QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de plain text encoding, static text in QR. DECLENCHEURS: text qr, plain text qr, message qr. EXCLURE: URL (utiliser URL QR), email (utiliser Email QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-email', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-email',
    el.display_name = 'Email QR Code',
    el.description = 'QR Code ouvrant un email pré-rédigé.',
    el.definition = 'Format de données Email QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Email QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de email QR codes, mailto links in QR. DECLENCHEURS: email qr, mailto qr, compose email qr. EXCLURE: SMS (utiliser SMS QR), contact (utiliser vCard QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-sms', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-sms',
    el.display_name = 'SMS QR Code',
    el.description = 'QR Code pré-remplissant un SMS.',
    el.definition = 'Format de données SMS QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes SMS QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de SMS QR codes, text message QR. DECLENCHEURS: sms qr, text message qr, message qr. EXCLURE: WhatsApp (utiliser WhatsApp QR), phone call (utiliser Phone QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-phone', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-phone',
    el.display_name = 'Téléphone QR Code',
    el.description = 'QR Code pour appel téléphonique direct.',
    el.definition = 'Format de données Téléphone QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Téléphone QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de phone call QR codes, tel: links. DECLENCHEURS: phone qr, call qr, tel qr, phone number qr. EXCLURE: SMS (utiliser SMS QR), WhatsApp (utiliser WhatsApp QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-video', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-video',
    el.display_name = 'Vidéo QR Code',
    el.description = 'QR Code redirigeant vers une vidéo.',
    el.definition = 'Format de données Vidéo QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Vidéo QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de video link QR codes, video sharing. DECLENCHEURS: video qr, youtube qr link, vimeo qr, video link. EXCLURE: YouTube channel (utiliser YouTube QR), audio (utiliser Audio QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-audio', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-audio',
    el.display_name = 'Audio QR Code',
    el.description = 'QR Code donnant accès à un fichier audio.',
    el.definition = 'Format de données Audio QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Audio QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de audio link QR codes, podcast QR. DECLENCHEURS: audio qr, podcast qr, music link qr, voice message qr. EXCLURE: Spotify profile (utiliser Spotify QR), video (utiliser Video QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-image-gallery', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-image-gallery',
    el.display_name = 'Image Gallery QR Code',
    el.description = 'L\'Image Gallery QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Image Gallery QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Image Gallery QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de photo album QR, image collection links. DECLENCHEURS: gallery qr, photo album qr, image gallery qr, photos qr. EXCLURE: single image, PDF (utiliser PDF QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-coupon', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-coupon',
    el.display_name = 'Coupon QR Code',
    el.description = 'QR Code offrant une réduction ou un bon d\'achat.',
    el.definition = 'Format de données Coupon QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Coupon QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de discount QR codes, promotional codes. DECLENCHEURS: coupon qr, discount qr, promo qr, deal qr. EXCLURE: payment (utiliser Payment QR), ticket (utiliser Ticket QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-social', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-social',
    el.display_name = 'Réseaux Sociaux Media QR Code',
    el.description = 'Le Réseaux Sociaux Media QR Code pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Réseaux Sociaux Media QR Code.',
    el.purpose = 'Optimisez vos campagnes marketing avec Réseaux Sociaux Media QR Code personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de social media QR codes generally. DECLENCHEURS: social qr, social media qr, social link qr. EXCLURE: specific platforms (utiliser Instagram QR, LinkedIn QR, etc.).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-instagram', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-instagram',
    el.display_name = 'Instagram QR Code',
    el.description = 'L\'Instagram QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Instagram QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Instagram QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Instagram profile QR, Instagram links. DECLENCHEURS: instagram qr, ig qr, insta qr. EXCLURE: other social (utiliser Facebook QR, TikTok QR, etc.), generic social (utiliser Social Media QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-linkedin', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-linkedin',
    el.display_name = 'LinkedIn QR Code',
    el.description = 'Le LinkedIn QR Code pour vos projets QR Code.',
    el.definition = 'Format de données LinkedIn QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes LinkedIn QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de LinkedIn profile QR, professional networking QR. DECLENCHEURS: linkedin qr, professional qr, company page qr. EXCLURE: vCard (utiliser vCard QR), business card frame (utiliser Business Card QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-facebook', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-facebook',
    el.display_name = 'Facebook QR Code',
    el.description = 'Le Facebook QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Facebook QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Facebook QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Facebook page QR, Facebook profile links. DECLENCHEURS: facebook qr, fb qr, facebook page qr. EXCLURE: Instagram (utiliser Instagram QR), WhatsApp (utiliser WhatsApp QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-twitter', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-twitter',
    el.display_name = 'Twitter/X QR Code',
    el.description = 'Le Twitter/X QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Twitter/X QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Twitter/X QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Twitter profile QR, X platform links. DECLENCHEURS: twitter qr, x qr, tweet qr. EXCLURE: other social platforms.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-youtube', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-youtube',
    el.display_name = 'YouTube QR Code',
    el.description = 'Le YouTube QR Code pour vos projets QR Code.',
    el.definition = 'Format de données YouTube QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes YouTube QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de YouTube channel QR, YouTube video links. DECLENCHEURS: youtube qr, channel qr, yt qr. EXCLURE: generic video (utiliser Video QR), TikTok (utiliser TikTok QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-tiktok', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-tiktok',
    el.display_name = 'TikTok QR Code',
    el.description = 'Le TikTok QR Code pour vos projets QR Code.',
    el.definition = 'Format de données TikTok QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes TikTok QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de TikTok profile QR, TikTok video links. DECLENCHEURS: tiktok qr, tt qr. EXCLURE: Instagram Reels (utiliser Instagram QR), YouTube Shorts (utiliser YouTube QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-snapchat', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-snapchat',
    el.display_name = 'Snapchat QR Code',
    el.description = 'Le Snapchat QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Snapchat QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Snapchat QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Snapchat profile QR, Snapcode alternatives. DECLENCHEURS: snapchat qr, snap qr, snapcode. EXCLURE: Instagram Stories (utiliser Instagram QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-whatsapp', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-whatsapp',
    el.display_name = 'WhatsApplication QR Code',
    el.description = 'La WhatsApplication QR Code pour vos projets QR Code.',
    el.definition = 'Format de données WhatsApplication QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes WhatsApplication QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de WhatsApp chat QR, wa.me links. DECLENCHEURS: whatsapp qr, wa qr, whatsapp chat qr. EXCLURE: SMS (utiliser SMS QR), Telegram (utiliser Telegram QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-telegram', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-telegram',
    el.display_name = 'Telegram QR Code',
    el.description = 'Le Telegram QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Telegram QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Telegram QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Telegram channel QR, Telegram links. DECLENCHEURS: telegram qr, tg qr, telegram channel qr, telegram bot qr. EXCLURE: WhatsApp (utiliser WhatsApp QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-pinterest', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-pinterest',
    el.display_name = 'Pinterest QR Code',
    el.description = 'Le Pinterest QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Pinterest QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Pinterest QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Pinterest profile QR, Pinterest board links. DECLENCHEURS: pinterest qr, pin qr, board qr. EXCLURE: Instagram (utiliser Instagram QR), image gallery (utiliser Image Gallery QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-spotify', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-spotify',
    el.display_name = 'Spotify QR Code',
    el.description = 'Le Spotify QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Spotify QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Spotify QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Spotify link QR, music sharing QR. DECLENCHEURS: spotify qr, playlist qr, music qr. EXCLURE: Apple Music (utiliser Apple Music QR), SoundCloud (utiliser SoundCloud QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-apple-music', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-apple-music',
    el.display_name = 'Applicationle Music QR Code',
    el.description = 'L\'Applicationle Music QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Applicationle Music QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Applicationle Music QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Apple Music link QR. DECLENCHEURS: apple music qr, itunes qr. EXCLURE: Spotify (utiliser Spotify QR), generic audio (utiliser Audio QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-soundcloud', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-soundcloud',
    el.display_name = 'SoundCloud QR Code',
    el.description = 'Le SoundCloud QR Code pour vos projets QR Code.',
    el.definition = 'Format de données SoundCloud QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes SoundCloud QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de SoundCloud link QR, indie music QR. DECLENCHEURS: soundcloud qr, sc qr. EXCLURE: Spotify (utiliser Spotify QR), generic audio (utiliser Audio QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-payment', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-payment',
    el.display_name = 'Paiement QR Code',
    el.description = 'Le Paiement QR Code pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Paiement QR Code.',
    el.purpose = 'Optimisez vos campagnes marketing avec Paiement QR Code personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de payment QR codes generally. DECLENCHEURS: payment qr, pay qr, money qr. EXCLURE: specific systems (utiliser PIX QR, UPI QR, PayPal QR, etc.).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-pix', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-pix',
    el.display_name = 'PIX QR Code',
    el.description = 'Le PIX QR Code pour vos projets QR Code.',
    el.definition = 'Format de données PIX QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes PIX QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Brazilian payments, PIX system. DECLENCHEURS: pix qr, brazil payment qr, pix code. EXCLURE: UPI India (utiliser UPI QR), generic payment.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-upi', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-upi',
    el.display_name = 'UPI QR Code',
    el.description = 'L\'UPI QR Code pour vos projets QR Code.',
    el.definition = 'Format de données UPI QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes UPI QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Indian payments, UPI system. DECLENCHEURS: upi qr, india payment qr, bharat qr. EXCLURE: PIX Brazil (utiliser PIX QR), generic payment.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-paypal', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-paypal',
    el.display_name = 'PayPal QR Code',
    el.description = 'Le PayPal QR Code pour vos projets QR Code.',
    el.definition = 'Format de données PayPal QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes PayPal QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de PayPal payments, PayPal.me links. DECLENCHEURS: paypal qr, paypal.me qr. EXCLURE: Venmo (utiliser Venmo QR), bank transfer (utiliser Bank Transfer QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-venmo', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-venmo',
    el.display_name = 'Venmo QR Code',
    el.description = 'Le Venmo QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Venmo QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Venmo QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Venmo payments, US P2P payments. DECLENCHEURS: venmo qr, venmo code. EXCLURE: PayPal (utiliser PayPal QR), Cash App.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-bitcoin', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-bitcoin',
    el.display_name = 'Bitcoin QR Code',
    el.description = 'QR Code pour paiement en Bitcoin.',
    el.definition = 'Format de données Bitcoin QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Bitcoin QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Bitcoin payments, BTC address QR. DECLENCHEURS: bitcoin qr, btc qr, crypto wallet qr. EXCLURE: Ethereum (utiliser Ethereum QR), generic crypto (utiliser Crypto QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-ethereum', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-ethereum',
    el.display_name = 'Ethereum QR Code',
    el.description = 'L\'Ethereum QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Ethereum QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Ethereum QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Ethereum payments, ETH address QR. DECLENCHEURS: ethereum qr, eth qr, erc20 qr. EXCLURE: Bitcoin (utiliser Bitcoin QR), generic crypto (utiliser Crypto QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-crypto', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-crypto',
    el.display_name = 'Crypto QR Code',
    el.description = 'QR Code pour transactions en cryptomonnaie.',
    el.definition = 'Format de données Crypto QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Crypto QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de generic crypto payments, multi-coin wallets. DECLENCHEURS: crypto qr, cryptocurrency qr, multi-coin qr. EXCLURE: specific coins (utiliser Bitcoin QR, Ethereum QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-bank-transfer', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-bank-transfer',
    el.display_name = 'Bank Transfer QR Code',
    el.description = 'Le Bank Transfer QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Bank Transfer QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Bank Transfer QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de bank transfer QR, IBAN encoding. DECLENCHEURS: bank transfer qr, sepa qr, iban qr, wire transfer qr. EXCLURE: PIX (utiliser PIX QR), UPI (utiliser UPI QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-location', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-location',
    el.display_name = 'Localisation QR Code',
    el.description = 'La Localisation QR Code pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Localisation QR Code.',
    el.purpose = 'Optimisez vos campagnes marketing avec Localisation QR Code personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de location QR codes generally, maps category. DECLENCHEURS: location qr, maps qr, navigation qr. EXCLURE: specific apps (utiliser Google Maps QR, Apple Maps QR, Waze QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-google-maps', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-google-maps',
    el.display_name = 'Google Maps QR Code',
    el.description = 'Le Google Maps QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Google Maps QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Google Maps QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Google Maps links, Google location QR. DECLENCHEURS: google maps qr, gmaps qr, google location qr. EXCLURE: Apple Maps (utiliser Apple Maps QR), Waze (utiliser Waze QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-apple-maps', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-apple-maps',
    el.display_name = 'Applicationle Maps QR Code',
    el.description = 'L\'Applicationle Maps QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Applicationle Maps QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Applicationle Maps QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Apple Maps links, iOS maps QR. DECLENCHEURS: apple maps qr, ios maps qr. EXCLURE: Google Maps (utiliser Google Maps QR), Waze (utiliser Waze QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-waze', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-waze',
    el.display_name = 'Waze QR Code',
    el.description = 'Le Waze QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Waze QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Waze QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Waze navigation links. DECLENCHEURS: waze qr, waze navigation qr. EXCLURE: Google Maps (utiliser Google Maps QR), Apple Maps (utiliser Apple Maps QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-coordinates', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-coordinates',
    el.display_name = 'Coordinates QR Code',
    el.description = 'Le Coordinates QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Coordinates QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Coordinates QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de raw GPS encoding, geo: URI format. DECLENCHEURS: coordinates qr, gps qr, geo qr, lat long qr. EXCLURE: specific map apps (utiliser Google Maps QR, Apple Maps QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-app', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-app',
    el.display_name = 'Application Télécharger QR Code',
    el.description = 'L\'Application Télécharger QR Code pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Application Télécharger QR Code.',
    el.purpose = 'Optimisez vos campagnes marketing avec Application Télécharger QR Code personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de app download QR codes generally. DECLENCHEURS: app qr, download app qr, app store qr. EXCLURE: specific stores (utiliser App Store QR, Play Store QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-app-store', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-app-store',
    el.display_name = 'Application Store QR Code',
    el.description = 'L\'Application Store QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Application Store QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Application Store QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de iOS app downloads, Apple App Store links. DECLENCHEURS: app store qr, ios app qr, apple app qr. EXCLURE: Play Store (utiliser Play Store QR), smart link (utiliser Smart App Download QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-play-store', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-play-store',
    el.display_name = 'Play Store QR Code',
    el.description = 'Le Play Store QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Play Store QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Play Store QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de Android app downloads, Google Play links. DECLENCHEURS: play store qr, android app qr, google play qr. EXCLURE: App Store (utiliser App Store QR), smart link (utiliser Smart App Download QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-app-download', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-app-download',
    el.display_name = 'Smart Application Télécharger QR Code',
    el.description = 'La Smart Application Télécharger QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Smart Application Télécharger QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Smart Application Télécharger QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de smart app links, cross-platform app download. DECLENCHEURS: smart app qr, universal app link, cross platform app qr. EXCLURE: specific stores (utiliser App Store QR, Play Store QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-review', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-review',
    el.display_name = 'Review QR Code',
    el.description = 'Le Review QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Review QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Review QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de review collection QR, Google review links. DECLENCHEURS: review qr, google review qr, tripadvisor qr, yelp qr. EXCLURE: feedback form (utiliser Feedback QR), survey (utiliser Survey QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-survey', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-survey',
    el.display_name = 'Sondage QR Code',
    el.description = 'QR Code vers un formulaire de sondage.',
    el.definition = 'Format de données Sondage QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Sondage QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de survey QR codes, questionnaire links. DECLENCHEURS: survey qr, questionnaire qr, typeform qr, google form qr. EXCLURE: simple feedback (utiliser Feedback QR), review (utiliser Review QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-feedback', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-feedback',
    el.display_name = 'Avis QR Code',
    el.description = 'QR Code pour recueillir des avis clients.',
    el.definition = 'Format de données Avis QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Avis QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de simple feedback collection, rating QR. DECLENCHEURS: feedback qr, rating qr, quick feedback qr. EXCLURE: full survey (utiliser Survey QR), review platform (utiliser Review QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-menu', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-menu',
    el.display_name = 'Menu QR Code',
    el.description = 'QR Code affichant un menu de restaurant.',
    el.definition = 'Format de données Menu QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Menu QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de restaurant menu QR, contactless menu. DECLENCHEURS: menu qr, restaurant qr, cafe qr, food menu qr. EXCLURE: Digital Menu landing page (utiliser Digital Menu), table tent frame (utiliser Table Tent QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-resume', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-resume',
    el.display_name = 'Resume QR Code',
    el.description = 'Le Resume QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Resume QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Resume QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de resume QR, CV links, portfolio QR. DECLENCHEURS: resume qr, cv qr, portfolio qr. EXCLURE: LinkedIn profile (utiliser LinkedIn QR), vCard (utiliser vCard QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-certificate', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-certificate',
    el.display_name = 'Certificate QR Code',
    el.description = 'Le Certificate QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Certificate QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Certificate QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de certificate verification, credential QR. DECLENCHEURS: certificate qr, diploma qr, credential qr, verification qr. EXCLURE: ticket (utiliser Ticket QR), ID badge.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-ticket', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-ticket',
    el.display_name = 'Ticket QR Code',
    el.description = 'Le Ticket QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Ticket QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Ticket QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de event tickets, transport tickets, entry QR. DECLENCHEURS: ticket qr, event ticket qr, boarding pass qr, concert ticket qr. EXCLURE: attendance check-in (utiliser Attendance QR), coupon (utiliser Coupon QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-attendance', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-attendance',
    el.display_name = 'Attendance QR Code',
    el.description = 'L\'Attendance QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Attendance QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Attendance QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de check-in QR, attendance tracking. DECLENCHEURS: attendance qr, check-in qr, sign-in qr, class attendance qr. EXCLURE: event ticket (utiliser Ticket QR), access badge.',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-pet-tag', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-pet-tag',
    el.display_name = 'Pet Tag QR Code',
    el.description = 'Le Pet Tag QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Pet Tag QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Pet Tag QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de pet ID QR, lost pet tags. DECLENCHEURS: pet tag qr, pet id qr, dog tag qr, cat tag qr. EXCLURE: medical ID (utiliser Medical ID QR), vCard (utiliser vCard QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-medical-id', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-medical-id',
    el.display_name = 'Medical ID QR Code',
    el.description = 'Le Medical ID QR Code pour vos projets QR Code.',
    el.definition = 'Format de données Medical ID QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Medical ID QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de medical alert QR, health info QR. DECLENCHEURS: medical id qr, health qr, emergency info qr, medical alert qr. EXCLURE: pet tag (utiliser Pet Tag QR), certificate (utiliser Certificate QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-file', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-file',
    el.display_name = 'File Télécharger QR Code',
    el.description = 'Le File Télécharger QR Code pour vos projets QR Code.',
    el.definition = 'Format de données File Télécharger QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes File Télécharger QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de file download QR, document download links. DECLENCHEURS: file qr, download qr, zip qr, document download qr. EXCLURE: PDF specifically (utiliser PDF QR), image gallery (utiliser Image Gallery QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-calendar', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-calendar',
    el.display_name = 'Calendrier Événement QR Code',
    el.description = 'QR Code pour événement calendrier.',
    el.definition = 'Format de données Calendrier Événement QR Code compatible avec les QR Codes.',
    el.purpose = 'Créez des QR Codes Calendrier Événement QR Code pour un accès instantané à vos contenus.',
    el.llm_context = 'UTILISER: pour discuter de calendar event QR, iCal links. DECLENCHEURS: calendar qr, event qr, ical qr, add to calendar qr. EXCLURE: event RSVP page (utiliser Event RSVP), ticket (utiliser Ticket QR).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'ean-13', locale_key: 'fr-FR'})
SET el.slug = 'ean-13',
    el.display_name = 'EAN-13',
    el.description = 'L\'EAN-13 pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de EAN-13.',
    el.purpose = 'Optimisez vos campagnes marketing avec EAN-13 personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de EAN-13 barcodes, European retail product identification, 13-digit barcodes, or GTIN-13 standard. DECLENCHEURS: ean-13, ean13, european article number, 13-digit barcode, gtin-13, retail barcode europe. EXCLURE: EAN-8 (compact version), UPC-A (North American), ISBN (books).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'ean-8', locale_key: 'fr-FR'})
SET el.slug = 'ean-8',
    el.display_name = 'EAN-8',
    el.description = 'L\'EAN-8 pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de EAN-8.',
    el.purpose = 'Optimisez vos campagnes marketing avec EAN-8 personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de EAN-8 barcodes, compact retail barcodes, 8-digit product codes, or small product labeling. DECLENCHEURS: ean-8, ean8, 8-digit barcode, compact barcode, small product barcode. EXCLURE: EAN-13 (full version), UPC-E (North American compact).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'upc-a', locale_key: 'fr-FR'})
SET el.slug = 'upc-a',
    el.display_name = 'UPC-A',
    el.description = 'L\'UPC-A pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de UPC-A.',
    el.purpose = 'Optimisez vos campagnes marketing avec UPC-A personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de UPC-A barcodes, North American retail product codes, 12-digit barcodes, or US/Canada product identification. DECLENCHEURS: upc-a, upca, universal product code, 12-digit barcode, us barcode, canada barcode, gtin-12. EXCLURE: UPC-E (compressed), EAN-13 (European).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'upc-e', locale_key: 'fr-FR'})
SET el.slug = 'upc-e',
    el.display_name = 'UPC-E',
    el.description = 'L\'UPC-E pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de UPC-E.',
    el.purpose = 'Optimisez vos campagnes marketing avec UPC-E personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de UPC-E barcodes, compressed product codes, 6-digit barcodes, or small package identification in North America. DECLENCHEURS: upc-e, upce, 6-digit barcode, compressed upc, zero-suppressed barcode. EXCLURE: UPC-A (full version), EAN-8 (European compact).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'code-128', locale_key: 'fr-FR'})
SET el.slug = 'code-128',
    el.display_name = 'Code 128',
    el.description = 'Le Code 128 pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Code 128.',
    el.purpose = 'Optimisez vos campagnes marketing avec Code 128 personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de Code 128 barcodes, logistics barcodes, shipping labels, high-density alphanumeric encoding, or ASCII barcodes. DECLENCHEURS: code 128, code128, logistics barcode, shipping barcode, alphanumeric barcode, ascii barcode. EXCLURE: Code 39 (simpler), GS1-128 (with application identifiers).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'code-39', locale_key: 'fr-FR'})
SET el.slug = 'code-39',
    el.display_name = 'Code 39',
    el.description = 'Le Code 39 pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Code 39.',
    el.purpose = 'Optimisez vos campagnes marketing avec Code 39 personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de Code 39 barcodes, industrial barcodes, automotive parts labeling, or self-checking alphanumeric codes. DECLENCHEURS: code 39, code39, code 3 of 9, industrial barcode, automotive barcode, defense barcode. EXCLURE: Code 128 (higher density), Codabar (numeric).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'itf-14', locale_key: 'fr-FR'})
SET el.slug = 'itf-14',
    el.display_name = 'ITF-14',
    el.description = 'L\'ITF-14 pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de ITF-14.',
    el.purpose = 'Optimisez vos campagnes marketing avec ITF-14 personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de ITF-14 barcodes, shipping carton barcodes, pallet labeling, or GTIN-14 encoding. DECLENCHEURS: itf-14, itf14, interleaved 2 of 5, carton barcode, pallet barcode, gtin-14, case barcode. EXCLURE: EAN-13 (retail), GS1-128 (with dates/lots).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'codabar', locale_key: 'fr-FR'})
SET el.slug = 'codabar',
    el.display_name = 'Codabar',
    el.description = 'Le Codabar pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Codabar.',
    el.purpose = 'Optimisez vos campagnes marketing avec Codabar personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de Codabar barcodes, library barcodes, blood bank barcodes, or legacy numeric codes. DECLENCHEURS: codabar, library barcode, blood bank barcode, fedex barcode, photo lab barcode. EXCLURE: Code 39 (alphanumeric), Code 128 (modern logistics).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'msi-plessey', locale_key: 'fr-FR'})
SET el.slug = 'msi-plessey',
    el.display_name = 'MSI Plessey',
    el.description = 'Le MSI Plessey pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de MSI Plessey.',
    el.purpose = 'Optimisez vos campagnes marketing avec MSI Plessey personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de MSI Plessey barcodes, warehoutiliser inventory barcodes, grocery store shelf labeling, or check-digit numeric codes. DECLENCHEURS: msi plessey, msi barcode, plessey barcode, inventory barcode, warehoutiliser barcode, shelf barcode. EXCLURE: Code 128 (modern), ITF-14 (shipping).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'gs1-128', locale_key: 'fr-FR'})
SET el.slug = 'gs1-128',
    el.display_name = 'GS1-128',
    el.description = 'Le GS1-128 pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de GS1-128.',
    el.purpose = 'Optimisez vos campagnes marketing avec GS1-128 personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de GS1-128 barcodes, supply chain traceability, batch/lot tracking, expiration dates on barcodes, or application identifiers. DECLENCHEURS: gs1-128, gs1128, ean-128, ucc-128, application identifier, batch barcode, lot barcode, expiry barcode. EXCLURE: Code 128 (without AI), ITF-14 (simpler).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'data-matrix', locale_key: 'fr-FR'})
SET el.slug = 'data-matrix',
    el.display_name = 'Data Matrix',
    el.description = 'Le Data Matrix pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Data Matrix.',
    el.purpose = 'Optimisez vos campagnes marketing avec Data Matrix personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de Data Matrix codes, electronics component marking, pharmaceutical serialization, or small 2D codes for industrial utiliser. DECLENCHEURS: data matrix, datamatrix, ecc200, electronics marking, pharma barcode, component marking, small 2d code. EXCLURE: QR code (consumer), GS1 DataMatrix (with identifiers).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'pdf417', locale_key: 'fr-FR'})
SET el.slug = 'pdf417',
    el.display_name = 'PDF417',
    el.description = 'Le PDF417 pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de PDF417.',
    el.purpose = 'Optimisez vos campagnes marketing avec PDF417 personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de PDF417 codes, driver\'s license barcodes, boarding passes, ID cards, or stacked 2D barcodes. DECLENCHEURS: pdf417, pdf 417, driver license barcode, id barcode, boarding pass barcode, stacked barcode. EXCLURE: QR code (square), Aztec (no quiet zone).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'aztec-code', locale_key: 'fr-FR'})
SET el.slug = 'aztec-code',
    el.display_name = 'Aztec Code',
    el.description = 'L\'Aztec Code pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Aztec Code.',
    el.purpose = 'Optimisez vos campagnes marketing avec Aztec Code personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de Aztec codes, airline boarding passes, train tickets, transport tickets, or 2D codes without quiet zone. DECLENCHEURS: aztec code, aztec barcode, boarding pass code, train ticket barcode, transport barcode, no quiet zone barcode. EXCLURE: QR code (needs quiet zone), PDF417 (rectangular).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'maxicode', locale_key: 'fr-FR'})
SET el.slug = 'maxicode',
    el.display_name = 'MaxiCode',
    el.description = 'Le MaxiCode pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de MaxiCode.',
    el.purpose = 'Optimisez vos campagnes marketing avec MaxiCode personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de MaxiCode, UPS package tracking, high-speed conveyor scanning, or hexagonal 2D codes. DECLENCHEURS: maxicode, ups barcode, package sorting code, hexagonal barcode, conveyor barcode. EXCLURE: QR code (square), Data Matrix (small items).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'gs1-datamatrix', locale_key: 'fr-FR'})
SET el.slug = 'gs1-datamatrix',
    el.display_name = 'GS1 DataMatrix',
    el.description = 'Le GS1 DataMatrix pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de GS1 DataMatrix.',
    el.purpose = 'Optimisez vos campagnes marketing avec GS1 DataMatrix personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de GS1 DataMatrix, pharmaceutical serialization, medical device UDI, food traceability, or regulated 2D codes. DECLENCHEURS: gs1 datamatrix, gs1 data matrix, pharma serialization, udi barcode, fmd barcode, medical device barcode, food traceability code. EXCLURE: plain Data Matrix (no AI), QR code (consumer).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'barcode-generator', locale_key: 'fr-FR'})
SET el.slug = 'code-barres-generateur',
    el.display_name = 'Code-barres Générateur',
    el.description = 'Générateur de codes-barres 1D en ligne.',
    el.definition = 'Outil professionnel Code-barres Générateur intégré à QR Code AI.',
    el.purpose = 'Utilisez notre Code-barres Générateur pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de barcode creation tools, generating EAN/UPC/Code 128 images, or barcode image software. DECLENCHEURS: barcode generator, create barcode, generate barcode, barcode maker, barcode image, ean generator, upc generator. EXCLURE: QR code generator (2D square), barcode scanner (reading).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'barcode-scanner', locale_key: 'fr-FR'})
SET el.slug = 'code-barres-scanner',
    el.display_name = 'Code-barres Scannerner',
    el.description = 'Le Code-barres Scannerner pour vos projets QR Code.',
    el.definition = 'Outil professionnel Code-barres Scannerner intégré à QR Code AI.',
    el.purpose = 'Utilisez notre Code-barres Scannerner pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de barcode reading tools, scanning EAN/UPC codes, barcode reader apps, or point-of-sale scanning. DECLENCHEURS: barcode scanner, barcode reader, scan barcode, pos scanner, barcode app, read barcode. EXCLURE: QR code scanner (2D), barcode generator (creation).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'analytics', locale_key: 'fr-FR'})
SET el.slug = 'analytiques',
    el.display_name = 'Analytiques',
    el.description = 'Les Analytiques pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Analytiques de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Analytiques.',
    el.llm_context = 'UTILISER: pour discuter de QR code analytics, scan tracking, click statistics, or performance metrics. DECLENCHEURS: analytics, statistics, tracking, metrics, reports, data, insights, performance. EXCLURE: specific analytics types (utiliser click-tracking, geo-tracking, etc.).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'click-tracking', locale_key: 'fr-FR'})
SET el.slug = 'click-suivi',
    el.display_name = 'Click Suivi',
    el.description = 'Le Click Suivi pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Click Suivi de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Click Suivi.',
    el.llm_context = 'UTILISER: pour discuter de link click tracking, click events, referrer data, or conversion tracking. DECLENCHEURS: click tracking, track clicks, click events, link clicks, click data, referrer tracking. EXCLURE: scan counting (QR specific), analytics (umbrella term).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'scan-counting', locale_key: 'fr-FR'})
SET el.slug = 'scanner-counting',
    el.display_name = 'Scanner Counting',
    el.description = 'Le Scanner Counting pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Scanner Counting de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Scanner Counting.',
    el.llm_context = 'UTILISER: pour discuter de QR code scan counts, scan statistics, or scan volume metrics. DECLENCHEURS: scan counting, count scans, scan stats, scan volume, scan numbers, how many scans. EXCLURE: click tracking (links), geo-tracking (location).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'geo-tracking', locale_key: 'fr-FR'})
SET el.slug = 'geo-suivi',
    el.display_name = 'Geographic Suivi',
    el.description = 'Le Geographic Suivi pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Geographic Suivi de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Geographic Suivi.',
    el.llm_context = 'UTILISER: pour discuter de geographic tracking, location data, country/city analytics, or IP geolocation for scans. DECLENCHEURS: geo tracking, geographic, location tracking, country data, city data, ip location, where scanned. EXCLURE: device detection (what device), time-series (when).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'device-detection', locale_key: 'fr-FR'})
SET el.slug = 'device-detection',
    el.display_name = 'Device Detection',
    el.description = 'La Device Detection pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Device Detection de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Device Detection.',
    el.llm_context = 'UTILISER: pour discuter de device detection, OS tracking, browser detection, or mobile vs desktop analytics. DECLENCHEURS: device detection, os detection, browser detection, device type, mobile or desktop, what device, utiliserr agent. EXCLURE: geo-tracking (location), contextual routing (redirect).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'time-series', locale_key: 'fr-FR'})
SET el.slug = 'time-series',
    el.display_name = 'Time Series Analytiques',
    el.description = 'Les Time Series Analytiques pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Time Series Analytiques de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Time Series Analytiques.',
    el.llm_context = 'UTILISER: pour discuter de time-series analytics, historical scan data, trend analysis, or scans over time. DECLENCHEURS: time series, historical data, trends, over time, date range, peak times, daily scans, weekly stats. EXCLURE: real-time (immediate), analytics (umbrella).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'contextual-routing', locale_key: 'fr-FR'})
SET el.slug = 'contextual-routing',
    el.display_name = 'Contextual Routing',
    el.description = 'Le Contextual Routing pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Contextual Routing de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Contextual Routing.',
    el.llm_context = 'UTILISER: pour discuter de contextual routing, device-based redirects, OS-specific destinations, or smart link routing rules. DECLENCHEURS: contextual routing, smart routing, device redirect, os redirect, app store redirect, conditional redirect, dynamic destination. EXCLURE: device detection (analytics only), edit destination (manual change).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'custom-domain-name', locale_key: 'fr-FR'})
SET el.slug = 'domaine-personnalise-name',
    el.display_name = 'Domaine Personnalisé',
    el.description = 'Le Domaine Personnalisé pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Domaine Personnalisé de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Domaine Personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de custom domains, branded short links, white-label URLs, or vanity domains. DECLENCHEURS: custom domain, branded domain, own domain, vanity url, white label domain, custom short url. EXCLURE: url shortener (action), white label (full branding).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'custom-link-preview', locale_key: 'fr-FR'})
SET el.slug = 'personnalise-link-preview',
    el.display_name = 'Personnalisé Link Preview',
    el.description = 'Le Personnalisé Link Preview pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Personnalisé Link Preview de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Personnalisé Link Preview.',
    el.llm_context = 'UTILISER: pour discuter de custom link previews, Open Graph meta tags, social media previews, or thumbnail customization. DECLENCHEURS: link preview, og tags, open graph, social preview, thumbnail, share preview, meta tags. EXCLURE: landing page (full page), custom domain (url).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'utm-builder', locale_key: 'fr-FR'})
SET el.slug = 'utm-builder',
    el.display_name = 'UTM Builder',
    el.description = 'L\'UTM Builder pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée UTM Builder de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité UTM Builder.',
    el.llm_context = 'UTILISER: pour discuter de UTM parameters, campaign tracking, Google Analytics parameters, or marketing attribution. DECLENCHEURS: utm builder, utm parameters, campaign tracking, utm source, utm medium, utm campaign, google analytics tracking. EXCLURE: analytics (viewing data), retargeting (ads).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'bulk-creation', locale_key: 'fr-FR'})
SET el.slug = 'masse-creation',
    el.display_name = 'En Masse Creation',
    el.description = 'L\'En Masse Creation pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée En Masse Creation de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité En Masse Creation.',
    el.llm_context = 'UTILISER: pour discuter de bulk QR code creation, mass generation, spreadsheet import, or enterprise-scale QR codes. DECLENCHEURS: bulk creation, bulk generate, mass create, batch qr, spreadsheet import, csv upload, multiple qr codes at once. EXCLURE: batch-qr-generator (tool), api (programmatic).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'team-workspaces', locale_key: 'fr-FR'})
SET el.slug = 'team-workspaces',
    el.display_name = 'Team Workspaces',
    el.description = 'Les Team Workspaces pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Team Workspaces de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Team Workspaces.',
    el.llm_context = 'UTILISER: pour discuter de team collaboration, multi-utiliserr access, shared QR code management, or role-based permissions. DECLENCHEURS: team workspace, collaboration, multi-utiliserr, shared access, team members, roles, permissions, organization. EXCLURE: white-label (branding), api (integration).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'api', locale_key: 'fr-FR'})
SET el.slug = 'api',
    el.display_name = 'API Access',
    el.description = 'L\'API Access pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée API Access de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité API Access.',
    el.llm_context = 'UTILISER: pour discuter de API access, developer integration, programmatic QR code creation, or RESTful endpoints. DECLENCHEURS: api, api access, developer api, rest api, integration, programmatic, endpoints. EXCLURE: webhooks (events), qr-code-api (specific tool).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'webhooks', locale_key: 'fr-FR'})
SET el.slug = 'webhooks',
    el.display_name = 'Webhooks',
    el.description = 'Les Webhooks pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Webhooks de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Webhooks.',
    el.llm_context = 'UTILISER: pour discuter de webhooks, event notifications, real-time callbacks, or scan event triggers. DECLENCHEURS: webhooks, webhook, event notification, callback, trigger, real-time event, scan webhook. EXCLURE: api (request/response), analytics (viewing).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'white-label', locale_key: 'fr-FR'})
SET el.slug = 'marque-blanche',
    el.display_name = 'Marque Blanche',
    el.description = 'Solution en marque blanche personnalisable.',
    el.definition = 'Fonctionnalité avancée Marque Blanche de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Marque Blanche.',
    el.llm_context = 'UTILISER: pour discuter de white-label solutions, removing platform branding, reseller programs, or agency branding. DECLENCHEURS: white label, whitelabel, remove branding, no branding, agency solution, reseller, own branding. EXCLURE: custom domain (url only), team workspaces (collaboration).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'password-protection', locale_key: 'fr-FR'})
SET el.slug = 'protection-mot-de-passe',
    el.display_name = 'Protection par Mot de Passe',
    el.description = 'Protection par mot de passe des QR Codes.',
    el.definition = 'Fonctionnalité avancée Protection par Mot de Passe de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Protection par Mot de Passe.',
    el.llm_context = 'UTILISER: pour discuter de password-protected QR codes, gated content, secure access, or password-required links. DECLENCHEURS: password protection, password protected, require password, gated content, secure qr, locked qr, access code. EXCLURE: expiration (time limit), scan limit (count limit).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'expiration', locale_key: 'fr-FR'})
SET el.slug = 'expiration',
    el.display_name = 'Link Expiration',
    el.description = 'La Link Expiration pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Link Expiration de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Link Expiration.',
    el.llm_context = 'UTILISER: pour discuter de link expiration, time-limited QR codes, expiry dates, or temporary access. DECLENCHEURS: expiration, expire, time limit, temporary qr, expiry date, auto-disable, limited time. EXCLURE: scan limit (count), password protection (access control).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'scan-limit', locale_key: 'fr-FR'})
SET el.slug = 'limite-scans',
    el.display_name = 'Limite de Scanners',
    el.description = 'Limitation du nombre de scans autorisés.',
    el.definition = 'Fonctionnalité avancée Limite de Scanners de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Limite de Scanners.',
    el.llm_context = 'UTILISER: pour discuter de scan limits, maximum scans, limited-utiliser QR codes, or scan quotas. DECLENCHEURS: scan limit, max scans, limited scans, scan quota, one-time scan, single utiliser, limited utiliser. EXCLURE: expiration (time), password protection (access).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'retargeting-pixel', locale_key: 'fr-FR'})
SET el.slug = 'reciblage-pixel',
    el.display_name = 'Reciblage Pixel',
    el.description = 'Le Reciblage Pixel pour vos projets QR Code.',
    el.definition = 'Fonctionnalité avancée Reciblage Pixel de QR Code AI.',
    el.purpose = 'Améliorez vos QR Codes avec la fonctionnalité Reciblage Pixel.',
    el.llm_context = 'UTILISER: pour discuter de retargeting pixels, Facebook pixel, Google pixel, remarketing, or ad tracking integration. DECLENCHEURS: retargeting pixel, facebook pixel, google pixel, remarketing, ad tracking, pixel integration, audience building. EXCLURE: utm-builder (attribution), analytics (internal).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-generator', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-generateur',
    el.display_name = 'QR Code Générateur',
    el.description = 'Outil en ligne pour créer des QR Codes personnalisés.',
    el.definition = 'Outil professionnel QR Code Générateur intégré à QR Code AI.',
    el.purpose = 'Utilisez notre QR Code Générateur pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de QR code creation tools, generating QR codes, or QR maker software. DECLENCHEURS: qr code generator, qr generator, qr maker, create qr, generate qr, make qr code, qr code creator. EXCLURE: barcode generator (1D), qr code scanner (reading), qr code api (programmatic).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-scanner', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-scanner',
    el.display_name = 'QR Code Scannerner',
    el.description = 'Application pour lire les QR Codes.',
    el.definition = 'Outil professionnel QR Code Scannerner intégré à QR Code AI.',
    el.purpose = 'Utilisez notre QR Code Scannerner pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de QR code scanning, reading QR codes, QR reader apps, or camera-based scanning. DECLENCHEURS: qr code scanner, qr scanner, qr reader, scan qr, read qr code, qr code app, camera scan. EXCLURE: barcode scanner (1D), qr code generator (creation).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-api', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-api',
    el.display_name = 'QR Code API',
    el.description = 'Le QR Code API pour vos projets QR Code.',
    el.definition = 'Outil professionnel QR Code API intégré à QR Code AI.',
    el.purpose = 'Utilisez notre QR Code API pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de QR code APIs, programmatic QR generation, developer QR tools, or REST QR endpoints. DECLENCHEURS: qr code api, qr api, programmatic qr, developer qr, rest qr, api qr generation. EXCLURE: api access (general feature), qr code generator (ui tool).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'landing-page-builder', locale_key: 'fr-FR'})
SET el.slug = 'page-destination-builder',
    el.display_name = 'Page de Destination Builder',
    el.description = 'La Page de Destination Builder pour vos projets QR Code.',
    el.definition = 'Outil professionnel Page de Destination Builder intégré à QR Code AI.',
    el.purpose = 'Utilisez notre Page de Destination Builder pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de landing page builders, no-code page creation, drag-and-drop page editors, or destination page tools. DECLENCHEURS: landing page builder, page builder, no-code page, drag and drop, page editor, bio page builder. EXCLURE: link-in-bio builder (social specific), menu builder (restaurant).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'url-shortener', locale_key: 'fr-FR'})
SET el.slug = 'url-shortener',
    el.display_name = 'URL Shortener',
    el.description = 'L\'URL Shortener pour vos projets QR Code.',
    el.definition = 'Outil professionnel URL Shortener intégré à QR Code AI.',
    el.purpose = 'Utilisez notre URL Shortener pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de URL shorteners, link shortening, creating short links, or compact URLs. DECLENCHEURS: url shortener, link shortener, short url, short link, shorten url, compact link, tiny url. EXCLURE: smart link (intelligent routing), custom domain (branded).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'link-in-bio-builder', locale_key: 'fr-FR'})
SET el.slug = 'lien-en-bio-builder',
    el.display_name = 'Lien en Bio Builder',
    el.description = 'Le Lien en Bio Builder pour vos projets QR Code.',
    el.definition = 'Outil professionnel Lien en Bio Builder intégré à QR Code AI.',
    el.purpose = 'Utilisez notre Lien en Bio Builder pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de link-in-bio pages, Instagram bio links, social media link hubs, or bio page tools. DECLENCHEURS: link in bio, bio link, instagram bio, tiktok bio, linktree alternative, bio page, social link hub. EXCLURE: landing page builder (general), menu builder (restaurant).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'menu-builder', locale_key: 'fr-FR'})
SET el.slug = 'menu-builder',
    el.display_name = 'Menu Builder',
    el.description = 'Le Menu Builder pour vos projets QR Code.',
    el.definition = 'Outil professionnel Menu Builder intégré à QR Code AI.',
    el.purpose = 'Utilisez notre Menu Builder pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de digital menu builders, restaurant menu creation, or QR menu tools. DECLENCHEURS: menu builder, restaurant menu, digital menu, qr menu, menu creator, food menu builder, cafe menu. EXCLURE: landing page builder (general), link-in-bio builder (social).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'vcard-generator', locale_key: 'fr-FR'})
SET el.slug = 'vcard-generateur',
    el.display_name = 'vCard Générateur',
    el.description = 'Le vCard Générateur pour vos projets QR Code.',
    el.definition = 'Outil professionnel vCard Générateur intégré à QR Code AI.',
    el.purpose = 'Utilisez notre vCard Générateur pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de vCard generators, digital business card creators, contact QR tools, or VCF file creation. DECLENCHEURS: vcard generator, vcard creator, digital business card, contact qr, vcf generator, electronic business card. EXCLURE: business cards (medium), qr code generator (general).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'wifi-qr-generator', locale_key: 'fr-FR'})
SET el.slug = 'wifi-qr-generateur',
    el.display_name = 'WiFi QR Générateur',
    el.description = 'Le WiFi QR Générateur pour vos projets QR Code.',
    el.definition = 'Outil professionnel WiFi QR Générateur intégré à QR Code AI.',
    el.purpose = 'Utilisez notre WiFi QR Générateur pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de WiFi QR generators, WiFi password sharing via QR, or wireless network QR tools. DECLENCHEURS: wifi qr generator, wifi qr code, share wifi password, wifi qr, wireless qr, network qr code. EXCLURE: qr code generator (general), qr code wifi (content type).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'batch-qr-generator', locale_key: 'fr-FR'})
SET el.slug = 'batch-qr-generateur',
    el.display_name = 'Batch QR Générateur',
    el.description = 'Le Batch QR Générateur pour vos projets QR Code.',
    el.definition = 'Outil professionnel Batch QR Générateur intégré à QR Code AI.',
    el.purpose = 'Utilisez notre Batch QR Générateur pour créer des QR Codes professionnels.',
    el.llm_context = 'UTILISER: pour discuter de batch QR generation, bulk QR creation tools, spreadsheet QR generation, or mass QR production. DECLENCHEURS: batch qr generator, bulk qr tool, mass qr, spreadsheet qr, csv qr, multiple qr generator. EXCLURE: bulk creation (feature), qr code generator (single).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'business-cards', locale_key: 'fr-FR'})
SET el.slug = 'carte-visites',
    el.display_name = 'Carte de Visites',
    el.description = 'Les Carte de Visites pour vos projets QR Code.',
    el.definition = 'Support physique Carte de Visites optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Carte de Visites pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on business cards, networking cards, or professional contact materials. DECLENCHEURS: business cards, business card qr, card qr code, networking card, contact card, visiting card. EXCLURE: vcard (digital format), flyers (larger print).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'flyers', locale_key: 'fr-FR'})
SET el.slug = 'flyers',
    el.display_name = 'Flyers',
    el.description = 'Les Flyers pour vos projets QR Code.',
    el.definition = 'Support physique Flyers optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Flyers pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on flyers, promotional handouts, or single-sheet marketing materials. DECLENCHEURS: flyers, flyer qr, handout, leaflet, promotional flyer, marketing flyer. EXCLURE: brochures (folded), posters (large format).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'posters-billboards', locale_key: 'fr-FR'})
SET el.slug = 'posters-billboards',
    el.display_name = 'Affiches & Panneaus',
    el.description = 'L\'Affiches & Panneaus pour vos projets QR Code.',
    el.definition = 'Support physique Affiches & Panneaus optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Affiches & Panneaus pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on posters, billboards, outdoor advertising, or large-format displays. DECLENCHEURS: poster qr, billboard qr, outdoor qr, large qr, signage qr, advertising poster. EXCLURE: banners (fabric), flyers (small print).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'brochures', locale_key: 'fr-FR'})
SET el.slug = 'brochures',
    el.display_name = 'Brochures',
    el.description = 'Les Brochures pour vos projets QR Code.',
    el.definition = 'Support physique Brochures optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Brochures pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on brochures, tri-folds, pamphlets, or folded marketing materials. DECLENCHEURS: brochure qr, tri-fold, pamphlet, folded brochure, informational brochure. EXCLURE: flyers (single sheet), catalogs (bound).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'catalogs', locale_key: 'fr-FR'})
SET el.slug = 'catalogs',
    el.display_name = 'Catalogues',
    el.description = 'Les Catalogues pour vos projets QR Code.',
    el.definition = 'Support physique Catalogues optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Catalogues pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes in product catalogs, print catalogs, or catalog shopping materials. DECLENCHEURS: catalog qr, product catalog, print catalog, shopping catalog, catalogue. EXCLURE: brochures (folded), magazines (editorial).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'magazines', locale_key: 'fr-FR'})
SET el.slug = 'magazines',
    el.display_name = 'Magazines',
    el.description = 'Les Magazines pour vos projets QR Code.',
    el.definition = 'Support physique Magazines optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Magazines pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes in magazines, print publications, or editorial content. DECLENCHEURS: magazine qr, print magazine, editorial qr, publication qr, magazine ad. EXCLURE: newspapers (daily), brochures (marketing).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'newspapers', locale_key: 'fr-FR'})
SET el.slug = 'newspapers',
    el.display_name = 'Journals',
    el.description = 'Les Journals pour vos projets QR Code.',
    el.definition = 'Support physique Journals optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Journals pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes in newspapers, print news, or daily/weekly publications. DECLENCHEURS: newspaper qr, print news, daily paper, news publication, newspaper ad. EXCLURE: magazines (glossy), digital news (websites).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'direct-mail', locale_key: 'fr-FR'})
SET el.slug = 'direct-mail',
    el.display_name = 'Direct Mail',
    el.description = 'Le Direct Mail pour vos projets QR Code.',
    el.definition = 'Support physique Direct Mail optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Direct Mail pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on direct mail, postcards, mailers, or shipped marketing materials. DECLENCHEURS: direct mail qr, postcard qr, mailer, mailed marketing, postal qr. EXCLURE: flyers (handed out), email (digital).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'stickers-labels', locale_key: 'fr-FR'})
SET el.slug = 'stickers-labels',
    el.display_name = 'Autocollants & Labels',
    el.description = 'Les Autocollants & Labels pour vos projets QR Code.',
    el.definition = 'Support physique Autocollants & Labels optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Autocollants & Labels pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on stickers, adhesive labels, or peel-and-stick materials. DECLENCHEURS: sticker qr, label qr, adhesive qr, peel and stick, vinyl sticker. EXCLURE: product labels (packaging), product packaging (boxes).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'banners', locale_key: 'fr-FR'})
SET el.slug = 'banners',
    el.display_name = 'Bannières',
    el.description = 'Les Bannières pour vos projets QR Code.',
    el.definition = 'Support physique Bannières optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Bannières pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on banners, trade show displays, event signage, or fabric/vinyl banners. DECLENCHEURS: banner qr, trade show banner, event banner, fabric banner, vinyl banner, roll-up banner. EXCLURE: posters (paper), billboards (outdoor).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'product-packaging', locale_key: 'fr-FR'})
SET el.slug = 'product-packaging',
    el.display_name = 'Produit Emballage',
    el.description = 'Le Produit Emballage pour vos projets QR Code.',
    el.definition = 'Support physique Produit Emballage optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Produit Emballage pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on product packaging, boxes, containers, or retail packaging. DECLENCHEURS: packaging qr, product box, container qr, retail packaging, package qr code. EXCLURE: product labels (adhesive), shipping labels (logistics).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'product-labels', locale_key: 'fr-FR'})
SET el.slug = 'product-labels',
    el.display_name = 'Produit Labels',
    el.description = 'Les Produit Labels pour vos projets QR Code.',
    el.definition = 'Support physique Produit Labels optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Produit Labels pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on product labels, applied labels, or product tag labels. DECLENCHEURS: product label qr, applied label, product tag, label qr code, item label. EXCLURE: stickers (decorative), product packaging (boxes).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'receipts', locale_key: 'fr-FR'})
SET el.slug = 'receipts',
    el.display_name = 'Tickets',
    el.description = 'Les Tickets pour vos projets QR Code.',
    el.definition = 'Support physique Tickets optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Tickets pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on receipts, transaction slips, or purchase confirmations. DECLENCHEURS: receipt qr, transaction receipt, purchase receipt, pos receipt, sales receipt. EXCLURE: tickets (entry), invoices (billing).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'table-tents', locale_key: 'fr-FR'})
SET el.slug = 'table-tents',
    el.display_name = 'Table Tents',
    el.description = 'Les Table Tents pour vos projets QR Code.',
    el.definition = 'Support physique Table Tents optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Table Tents pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on table tents, restaurant table stands, or tabletop displays. DECLENCHEURS: table tent qr, table stand, restaurant table qr, tabletop display, table card. EXCLURE: printed menus (booklet), flyers (handed out).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'menus-printed', locale_key: 'fr-FR'})
SET el.slug = 'menus-printed',
    el.display_name = 'Imprimered Menus',
    el.description = 'L\'Imprimered Menus pour vos projets QR Code.',
    el.definition = 'Support physique Imprimered Menus optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Imprimered Menus pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on printed menus, physical restaurant menus, or paper menus. DECLENCHEURS: printed menu qr, paper menu, physical menu, restaurant menu qr, laminated menu. EXCLURE: digital menu (online), table tents (standing).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'tickets-physical', locale_key: 'fr-FR'})
SET el.slug = 'tickets-physical',
    el.display_name = 'Physical Tickets',
    el.description = 'Les Physical Tickets pour vos projets QR Code.',
    el.definition = 'Support physique Physical Tickets optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Physical Tickets pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on physical tickets, event tickets, admission tickets, or printed tickets. DECLENCHEURS: ticket qr, event ticket, physical ticket, admission ticket, printed ticket, concert ticket. EXCLURE: digital tickets (mobile), receipts (purchase).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'emails', locale_key: 'fr-FR'})
SET el.slug = 'emails',
    el.display_name = 'Emails',
    el.description = 'Les Emails pour vos projets QR Code.',
    el.definition = 'Support physique Emails optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Emails pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes in emails, email signatures, email campaigns, or email marketing. DECLENCHEURS: email qr, email signature, email campaign, email marketing qr, newsletter qr. EXCLURE: websites (web pages), documents (pdfs).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'presentations', locale_key: 'fr-FR'})
SET el.slug = 'presentations',
    el.display_name = 'Presentations',
    el.description = 'Les Presentations pour vos projets QR Code.',
    el.definition = 'Support physique Presentations optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Presentations pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes in presentations, slide decks, PowerPoint, or meeting materials. DECLENCHEURS: presentation qr, slide qr, powerpoint qr, deck qr, meeting slides, conference presentation. EXCLURE: documents (static), websites (interactive).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'documents', locale_key: 'fr-FR'})
SET el.slug = 'documents',
    el.display_name = 'Documents',
    el.description = 'Les Documents pour vos projets QR Code.',
    el.definition = 'Support physique Documents optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Documents pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes in documents, PDFs, reports, contracts, or printed documents. DECLENCHEURS: document qr, pdf qr, report qr, contract qr, printed document. EXCLURE: presentations (slides), emails (messages).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'websites', locale_key: 'fr-FR'})
SET el.slug = 'websites',
    el.display_name = 'Websites',
    el.description = 'Les Websites pour vos projets QR Code.',
    el.definition = 'Support physique Websites optimisé pour l\'affichage de QR Codes.',
    el.purpose = 'Intégrez vos QR Codes sur Websites pour maximiser l\'engagement.',
    el.llm_context = 'UTILISER: pour discuter de QR codes displayed on websites, web pages, or online platforms. DECLENCHEURS: website qr, web page qr, online qr, site qr, desktop to mobile. EXCLURE: emails (messages), presentations (slides).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'create-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'creer-qr-code',
    el.display_name = 'Créer QR Code',
    el.description = 'Générer un QR Code personnalisé en quelques clics.',
    el.definition = 'Fonctionnalité QR Code AI permettant de créer qr code facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour créer qr code.',
    el.llm_context = 'UTILISER: pour discuter de QR code creation, making QR codes, or generating new QR codes. DECLENCHEURS: create qr code, make qr, generate qr, new qr code, create qr, make qr code. EXCLURE: customize qr (design), scan qr (reading), edit destination (update).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'scan-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'scanner-qr-code',
    el.display_name = 'Scanner QR Code',
    el.description = 'Lire un QR Code avec la caméra de votre smartphone.',
    el.definition = 'Fonctionnalité QR Code AI permettant de scanner qr code facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour scanner qr code.',
    el.llm_context = 'UTILISER: pour discuter de scanning QR codes, reading QR codes, or decoding QR content. DECLENCHEURS: scan qr code, read qr, decode qr, scan qr, qr scan, utiliser camera to scan. EXCLURE: create qr (making), track scans (analytics).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'customize-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'personnaliser-qr-code',
    el.display_name = 'Personnaliser QR Code',
    el.description = 'Personnaliser l\'apparence de votre QR Code avec vos couleurs et logo.',
    el.definition = 'Fonctionnalité QR Code AI permettant de personnaliser qr code facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour personnaliser qr code.',
    el.llm_context = 'UTILISER: pour discuter de QR code customization, styling QR codes, or designing QR appearance. DECLENCHEURS: customize qr, design qr, style qr, qr design, qr customization, personalize qr. EXCLURE: create qr (initial), add logo (specific), change colors (specific).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'download-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'telecharger-qr-code',
    el.display_name = 'Télécharger QR Code',
    el.description = 'Télécharger votre QR Code en PNG, SVG ou PDF haute résolution.',
    el.definition = 'Fonctionnalité QR Code AI permettant de télécharger qr code facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour télécharger qr code.',
    el.llm_context = 'UTILISER: pour discuter de downloading QR codes, exporting QR images, or saving QR files. DECLENCHEURS: download qr, export qr, save qr, qr download, get qr image, qr file. EXCLURE: print qr (physical), share qr (distribute).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'print-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'imprimer-qr-code',
    el.display_name = 'Imprimer QR Code',
    el.description = 'Imprimer votre QR Code avec les paramètres optimaux.',
    el.definition = 'Fonctionnalité QR Code AI permettant de imprimer qr code facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour imprimer qr code.',
    el.llm_context = 'UTILISER: pour discuter de printing QR codes, physical QR output, or QR print requirements. DECLENCHEURS: print qr, qr printing, physical qr, print qr code, qr for print, printable qr. EXCLURE: download qr (digital), qr size (specification).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'add-logo', locale_key: 'fr-FR'})
SET el.slug = 'add-logo',
    el.display_name = 'Add Logo',
    el.description = 'L\'Add Logo pour vos projets QR Code.',
    el.definition = 'Fonctionnalité QR Code AI permettant de add logo facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour add logo.',
    el.llm_context = 'UTILISER: pour discuter de adding logos to QR codes, branded QR codes, or logo placement in QR. DECLENCHEURS: add logo, logo qr, branded qr, qr with logo, center logo, embed logo. EXCLURE: customize qr (general), change colors (different aspect).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'change-colors', locale_key: 'fr-FR'})
SET el.slug = 'change-couleurs',
    el.display_name = 'Change Couleurs',
    el.description = 'Les Change Couleurs pour vos projets QR Code.',
    el.definition = 'Fonctionnalité QR Code AI permettant de change couleurs facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour change couleurs.',
    el.llm_context = 'UTILISER: pour discuter de QR code color customization, changing QR colors, or colored QR codes. DECLENCHEURS: change colors, qr colors, colored qr, custom color qr, qr color scheme. EXCLURE: add logo (different aspect), customize qr (general).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'edit-destination', locale_key: 'fr-FR'})
SET el.slug = 'modifier-destination',
    el.display_name = 'Modifier Destination',
    el.description = 'La Modifier Destination pour vos projets QR Code.',
    el.definition = 'Fonctionnalité QR Code AI permettant de modifier destination facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour modifier destination.',
    el.llm_context = 'UTILISER: pour discuter de editing QR destinations, changing where QR points, or updating QR URLs. DECLENCHEURS: edit destination, change url, update link, modify destination, redirect qr, change qr target. EXCLURE: create qr (initial), dynamic qr (code type).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'share-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'partager-qr-code',
    el.display_name = 'Partager QR Code',
    el.description = 'Partager votre QR Code par email, réseaux sociaux ou lien direct.',
    el.definition = 'Fonctionnalité QR Code AI permettant de partager qr code facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour partager qr code.',
    el.llm_context = 'UTILISER: pour discuter de sharing QR codes, distributing QR images, or sending QR codes to others. DECLENCHEURS: share qr, send qr, distribute qr, qr sharing, email qr, share qr code. EXCLURE: download qr (save), print qr (physical).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'track-scans', locale_key: 'fr-FR'})
SET el.slug = 'suivre-scans',
    el.display_name = 'Suivre Scanners',
    el.description = 'Les Suivre Scanners pour vos projets QR Code.',
    el.definition = 'Fonctionnalité QR Code AI permettant de suivre scanners facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour suivre scanners.',
    el.llm_context = 'UTILISER: pour discuter de scan tracking, monitoring QR performance, or viewing scan analytics. DECLENCHEURS: track scans, scan analytics, monitor scans, scan statistics, scan tracking, qr tracking. EXCLURE: scan qr (action), analytics (feature).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'shorten-url', locale_key: 'fr-FR'})
SET el.slug = 'shorten-url',
    el.display_name = 'Shorten URL',
    el.description = 'Le Shorten URL pour vos projets QR Code.',
    el.definition = 'Fonctionnalité QR Code AI permettant de shorten url facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour shorten url.',
    el.llm_context = 'UTILISER: pour discuter de URL shortening, creating short links, or compacting long URLs. DECLENCHEURS: shorten url, short link, short url, url shortener, compact link, tiny url. EXCLURE: create smart link (intelligent), custom domain (branding).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'create-smart-link', locale_key: 'fr-FR'})
SET el.slug = 'creer-lien-intelligent',
    el.display_name = 'Créer Lien Intelligent',
    el.description = 'Le Créer Lien Intelligent pour vos projets QR Code.',
    el.definition = 'Fonctionnalité QR Code AI permettant de créer lien intelligent facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour créer lien intelligent.',
    el.llm_context = 'UTILISER: pour discuter de smart link creation, intelligent links, or advanced short links with routing. DECLENCHEURS: create smart link, smart link, intelligent link, routing link, conditional link. EXCLURE: shorten url (basic), contextual routing (feature).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'create-landing-page', locale_key: 'fr-FR'})
SET el.slug = 'creer-page-destination',
    el.display_name = 'Créer Page de Destination',
    el.description = 'La Créer Page de Destination pour vos projets QR Code.',
    el.definition = 'Fonctionnalité QR Code AI permettant de créer page de destination facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour créer page de destination.',
    el.llm_context = 'UTILISER: pour discuter de landing page creation, building destination pages, or creating bio pages. DECLENCHEURS: create landing page, build page, make landing page, destination page, bio page creation. EXCLURE: landing page builder (tool), link in bio (specific type).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'create-barcode', locale_key: 'fr-FR'})
SET el.slug = 'creer-code-barres',
    el.display_name = 'Créer Code-barres',
    el.description = 'Les Créer Code-barres pour vos projets QR Code.',
    el.definition = 'Fonctionnalité QR Code AI permettant de créer code-barres facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour créer code-barres.',
    el.llm_context = 'UTILISER: pour discuter de barcode creation, generating 1D barcodes, or making EAN/UPC codes. DECLENCHEURS: create barcode, generate barcode, make barcode, barcode creation, ean barcode, upc barcode. EXCLURE: create qr (2D), scan barcode (reading).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'scan-barcode', locale_key: 'fr-FR'})
SET el.slug = 'scanner-code-barres',
    el.display_name = 'Scanner Code-barres',
    el.description = 'Les Scanner Code-barres pour vos projets QR Code.',
    el.definition = 'Fonctionnalité QR Code AI permettant de scanner code-barres facilement.',
    el.purpose = 'Simplifiez votre workflow en utilisant notre outil pour scanner code-barres.',
    el.llm_context = 'UTILISER: pour discuter de barcode scanning, reading 1D barcodes, or decoding product barcodes. DECLENCHEURS: scan barcode, read barcode, barcode scan, decode barcode, barcode reader. EXCLURE: scan qr (2D), create barcode (making).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'restaurants', locale_key: 'fr-FR'})
SET el.slug = 'restaurants',
    el.display_name = 'Restaurants',
    el.description = 'Solutions QR Code pour la restauration : menus, paiements, avis.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Restaurants.',
    el.purpose = 'Transformez votre activité Restaurants avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for restaurants, food service, cafes, bars, or digital menus. DECLENCHEURS: restaurant qr, cafe qr, bar qr, food service qr, dine-in qr, table qr. EXCLURE: retail (shopping), hospitality (hotels).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'retail', locale_key: 'fr-FR'})
SET el.slug = 'commerce-detail',
    el.display_name = 'Commerce de Détail',
    el.description = 'QR Codes pour le commerce de détail : produits, promotions, fidélité.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Commerce de Détail.',
    el.purpose = 'Transformez votre activité Commerce de Détail avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for retail, shops, stores, or e-commerce product labeling. DECLENCHEURS: retail qr, store qr, shop qr, product qr, e-commerce qr, shopping qr. EXCLURE: restaurants (food), hospitality (hotels).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'hospitality', locale_key: 'fr-FR'})
SET el.slug = 'hotellerie',
    el.display_name = 'Hôtellerie',
    el.description = 'QR Codes pour l\'hôtellerie : check-in, services, informations.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Hôtellerie.',
    el.purpose = 'Transformez votre activité Hôtellerie avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for hotels, resorts, tourism, or guest services. DECLENCHEURS: hotel qr, resort qr, hospitality qr, tourism qr, guest qr, room qr. EXCLURE: restaurants (food), retail (shopping).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'healthcare', locale_key: 'fr-FR'})
SET el.slug = 'sante',
    el.display_name = 'Santé',
    el.description = 'QR Codes pour la santé : informations patient, médicaments, rendez-vous.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Santé.',
    el.purpose = 'Transformez votre activité Santé avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for healthcare, hospitals, clinics, or pharmaceutical applications. DECLENCHEURS: healthcare qr, hospital qr, clinic qr, pharma qr, medical qr, patient qr. EXCLURE: fitness (gyms), beauty (spas).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'education', locale_key: 'fr-FR'})
SET el.slug = 'education',
    el.display_name = 'Éducation',
    el.description = 'QR Codes pour l\'éducation : ressources, cours, examens.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Éducation.',
    el.purpose = 'Transformez votre activité Éducation avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for schools, universities, education, or training materials. DECLENCHEURS: education qr, school qr, university qr, student qr, classroom qr, learning qr. EXCLURE: entertainment (events), government (public).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'real-estate', locale_key: 'fr-FR'})
SET el.slug = 'immobilier',
    el.display_name = 'Immobilier',
    el.description = 'QR Codes pour l\'immobilier : visites virtuelles, annonces, contacts.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Immobilier.',
    el.purpose = 'Transformez votre activité Immobilier avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for real estate, property listings, or virtual tours. DECLENCHEURS: real estate qr, property qr, houtiliser qr, listing qr, virtual tour qr, for sale qr. EXCLURE: construction (building), hospitality (hotels).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'fitness', locale_key: 'fr-FR'})
SET el.slug = 'fitness',
    el.display_name = 'Fitness',
    el.description = 'QR Codes pour le fitness : programmes, équipements, cours.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Fitness.',
    el.purpose = 'Transformez votre activité Fitness avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for gyms, fitness centers, sports, or wellness applications. DECLENCHEURS: gym qr, fitness qr, sports qr, wellness qr, workout qr, exercise qr. EXCLURE: healthcare (medical), beauty (cosmetics).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'beauty', locale_key: 'fr-FR'})
SET el.slug = 'beaute',
    el.display_name = 'Beauté',
    el.description = 'QR Codes pour la beauté : tutoriels, ingrédients, achats.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Beauté.',
    el.purpose = 'Transformez votre activité Beauté avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for salons, spas, cosmetics, or beauty products. DECLENCHEURS: beauty qr, salon qr, spa qr, cosmetics qr, skincare qr, makeup qr. EXCLURE: fitness (gyms), healthcare (medical).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'entertainment', locale_key: 'fr-FR'})
SET el.slug = 'divertissement',
    el.display_name = 'Divertissement',
    el.description = 'QR Codes pour le divertissement : billets, contenus exclusifs.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Divertissement.',
    el.purpose = 'Transformez votre activité Divertissement avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for entertainment, movies, games, or live events. DECLENCHEURS: entertainment qr, movie qr, event qr, concert qr, theater qr, gaming qr. EXCLURE: education (learning), hospitality (hotels).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'transportation', locale_key: 'fr-FR'})
SET el.slug = 'transportation',
    el.display_name = 'Transportation',
    el.description = 'La Transportation pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Transportation.',
    el.purpose = 'Transformez votre activité Transportation avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for airlines, transit, or transportation ticketing. DECLENCHEURS: airline qr, transit qr, transportation qr, boarding pass qr, train qr, bus qr. EXCLURE: logistics (warehoutiliser), manufacturing (production).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'manufacturing', locale_key: 'fr-FR'})
SET el.slug = 'industrie',
    el.display_name = 'Industrie',
    el.description = 'QR Codes pour l\'industrie : traçabilité, maintenance, inventaire.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Industrie.',
    el.purpose = 'Transformez votre activité Industrie avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for manufacturing, production, or assembly lines. DECLENCHEURS: manufacturing qr, factory qr, production qr, assembly qr, industrial qr, parts qr. EXCLURE: logistics (shipping), construction (building).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'logistics', locale_key: 'fr-FR'})
SET el.slug = 'logistique',
    el.display_name = 'Logistique',
    el.description = 'QR Codes pour la logistique : suivi colis, inventaire, livraison.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Logistique.',
    el.purpose = 'Transformez votre activité Logistique avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for logistics, shipping, or warehoutiliser management. DECLENCHEURS: logistics qr, shipping qr, warehoutiliser qr, delivery qr, supply chain qr, tracking qr. EXCLURE: manufacturing (production), retail (stores).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'construction', locale_key: 'fr-FR'})
SET el.slug = 'btp',
    el.display_name = 'BTP',
    el.description = 'QR Codes pour le BTP : plans, sécurité, documentation.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur BTP.',
    el.purpose = 'Transformez votre activité BTP avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for construction sites, building projects, or infrastructure. DECLENCHEURS: construction qr, building qr, site qr, contractor qr, blueprint qr, safety qr. EXCLURE: real estate (sales), manufacturing (production).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'finance', locale_key: 'fr-FR'})
SET el.slug = 'finance',
    el.display_name = 'Finance',
    el.description = 'QR Codes pour la finance : paiements, documents, authentification.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Finance.',
    el.purpose = 'Transformez votre activité Finance avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for banking, finance, insurance, or financial services. DECLENCHEURS: finance qr, banking qr, insurance qr, payment qr, atm qr, fintech qr. EXCLURE: retail (shopping), government (public sector).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'government', locale_key: 'fr-FR'})
SET el.slug = 'services-publics',
    el.display_name = 'Services Publics',
    el.description = 'QR Codes pour services publics : formulaires, informations, accès.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Services Publics.',
    el.purpose = 'Transformez votre activité Services Publics avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for government, public sector, or citizen services. DECLENCHEURS: government qr, public sector qr, citizen qr, municipal qr, civic qr, id card qr. EXCLURE: nonprofit (charity), enterprise (business).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'marketing-agencies', locale_key: 'fr-FR'})
SET el.slug = 'marketing-agencies',
    el.display_name = 'Marketing Agencies',
    el.description = 'Les Marketing Agencies pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Marketing Agencies.',
    el.purpose = 'Transformez votre activité Marketing Agencies avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for marketing agencies, digital marketing, or advertising campaigns. DECLENCHEURS: marketing agency qr, digital marketing qr, campaign qr, advertising qr, agency qr. EXCLURE: creative agencies (design), consulting (business).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'creative-agencies', locale_key: 'fr-FR'})
SET el.slug = 'creative-agencies',
    el.display_name = 'Creative Agencies',
    el.description = 'Les Creative Agencies pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Creative Agencies.',
    el.purpose = 'Transformez votre activité Creative Agencies avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for creative agencies, design firms, or branding work. DECLENCHEURS: creative agency qr, design agency qr, branding qr, designer qr, creative qr. EXCLURE: marketing agencies (advertising), consulting (business).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'event-management', locale_key: 'fr-FR'})
SET el.slug = 'event-management',
    el.display_name = 'Événement Management',
    el.description = 'Le Événement Management pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Événement Management.',
    el.purpose = 'Transformez votre activité Événement Management avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for event management, conferences, weddings, or event planning. DECLENCHEURS: event management qr, conference qr, wedding qr, event planner qr, venue qr. EXCLURE: entertainment (movies), hospitality (hotels).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'nonprofits', locale_key: 'fr-FR'})
SET el.slug = 'associationss',
    el.display_name = 'Associationss',
    el.description = 'L\'Associationss pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Associationss.',
    el.purpose = 'Transformez votre activité Associationss avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for nonprofits, charities, NGOs, or donation collection. DECLENCHEURS: nonprofit qr, charity qr, ngo qr, donation qr, fundraising qr, volunteer qr. EXCLURE: government (public sector), small business (commercial).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'consulting', locale_key: 'fr-FR'})
SET el.slug = 'consulting',
    el.display_name = 'Consulting',
    el.description = 'Le Consulting pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Consulting.',
    el.purpose = 'Transformez votre activité Consulting avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for consulting firms, business consulting, or professional services. DECLENCHEURS: consulting qr, consultant qr, advisory qr, professional services qr, firm qr. EXCLURE: agencies (marketing), enterprise (large corp).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'developers', locale_key: 'fr-FR'})
SET el.slug = 'developers',
    el.display_name = 'Developers',
    el.description = 'Les Developers pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Developers.',
    el.purpose = 'Transformez votre activité Developers avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de developers, API integration, or programmatic QR code generation. DECLENCHEURS: developer qr, api utiliserr, programmer qr, integrator, coder qr, tech qr. EXCLURE: enterprise (organization), agencies (marketing).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'enterprise', locale_key: 'fr-FR'})
SET el.slug = 'enterprise',
    el.display_name = 'Enterprise',
    el.description = 'L\'Enterprise pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Enterprise.',
    el.purpose = 'Transformez votre activité Enterprise avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for enterprise, large organizations, or corporate deployments. DECLENCHEURS: enterprise qr, corporate qr, large organization qr, company-wide qr, sso qr. EXCLURE: small business (smb), freelancers (individual).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'agencies', locale_key: 'fr-FR'})
SET el.slug = 'agencies',
    el.display_name = 'Agencies',
    el.description = 'Les Agencies pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Agencies.',
    el.purpose = 'Transformez votre activité Agencies avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de agencies managing QR codes for multiple clients. DECLENCHEURS: agency qr, multi-client qr, client management qr, agency workspaces. EXCLURE: enterprise (internal), small business (single owner).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'small-business', locale_key: 'fr-FR'})
SET el.slug = 'small-business',
    el.display_name = 'Small Business',
    el.description = 'Le Small Business pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Small Business.',
    el.purpose = 'Transformez votre activité Small Business avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for small businesses, SMBs, or local businesses. DECLENCHEURS: small business qr, smb qr, local business qr, shop owner qr, mom and pop qr. EXCLURE: enterprise (large), freelancers (individual).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'freelancers', locale_key: 'fr-FR'})
SET el.slug = 'freelancers',
    el.display_name = 'Gratuitlancers',
    el.description = 'Les Gratuitlancers pour vos projets QR Code.',
    el.definition = 'Solutions QR Code spécialisées pour le secteur Gratuitlancers.',
    el.purpose = 'Transformez votre activité Gratuitlancers avec des QR Codes sur mesure.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for freelancers, independent professionals, or solo entrepreneurs. DECLENCHEURS: freelancer qr, independent qr, solo qr, self-employed qr, contractor qr. EXCLURE: small business (employees), agencies (teams).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'instagram', locale_key: 'fr-FR'})
SET el.slug = 'instagram',
    el.display_name = 'Instagram',
    el.description = 'QR Code vers votre profil Instagram.',
    el.definition = 'Intégration QR Code pour la plateforme Instagram.',
    el.purpose = 'Connecter votre audience à Instagram via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Instagram QR codes, Instagram profiles, or Meta social sharing. DECLENCHEURS: instagram, ig, instagram qr, instagram profile, instagram link. EXCLURE: facebook (separate platform), tiktok (competitor).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'linkedin', locale_key: 'fr-FR'})
SET el.slug = 'linkedin',
    el.display_name = 'LinkedIn',
    el.description = 'QR Code vers votre profil LinkedIn.',
    el.definition = 'Intégration QR Code pour la plateforme LinkedIn.',
    el.purpose = 'Connecter votre audience à LinkedIn via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de LinkedIn QR codes, professional networking, or business profiles. DECLENCHEURS: linkedin, linkedin qr, linkedin profile, professional network, business network. EXCLURE: facebook (social), twitter (microblog).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'facebook', locale_key: 'fr-FR'})
SET el.slug = 'facebook',
    el.display_name = 'Facebook',
    el.description = 'QR Code vers votre page Facebook.',
    el.definition = 'Intégration QR Code pour la plateforme Facebook.',
    el.purpose = 'Connecter votre audience à Facebook via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Facebook QR codes, Facebook pages, or Meta social networking. DECLENCHEURS: facebook, fb, facebook qr, facebook page, facebook group. EXCLURE: instagram (visual), linkedin (professional).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'twitter', locale_key: 'fr-FR'})
SET el.slug = 'twitter',
    el.display_name = 'Twitter / X',
    el.description = 'QR Code vers votre compte Twitter/X.',
    el.definition = 'Intégration QR Code pour la plateforme Twitter / X.',
    el.purpose = 'Connecter votre audience à Twitter / X via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Twitter/X QR codes, tweets, or microblogging. DECLENCHEURS: twitter, x, twitter qr, tweet, x platform, twitter profile. EXCLURE: facebook (social network), linkedin (professional).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'youtube', locale_key: 'fr-FR'})
SET el.slug = 'youtube',
    el.display_name = 'YouTube',
    el.description = 'QR Code vers votre chaîne YouTube.',
    el.definition = 'Intégration QR Code pour la plateforme YouTube.',
    el.purpose = 'Connecter votre audience à YouTube via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de YouTube QR codes, YouTube channels, or video linking. DECLENCHEURS: youtube, yt, youtube qr, youtube channel, youtube video, video platform. EXCLURE: tiktok (short-form), spotify (audio).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'tiktok', locale_key: 'fr-FR'})
SET el.slug = 'tiktok',
    el.display_name = 'TikTok',
    el.description = 'QR Code vers votre compte TikTok.',
    el.definition = 'Intégration QR Code pour la plateforme TikTok.',
    el.purpose = 'Connecter votre audience à TikTok via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de TikTok QR codes, short-form video, or Gen Z marketing. DECLENCHEURS: tiktok, tik tok, tiktok qr, tiktok profile, short video. EXCLURE: youtube (long-form), instagram (photos).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'snapchat', locale_key: 'fr-FR'})
SET el.slug = 'snapchat',
    el.display_name = 'Snapchat',
    el.description = 'QR Code vers votre profil Snapchat.',
    el.definition = 'Intégration QR Code pour la plateforme Snapchat.',
    el.purpose = 'Connecter votre audience à Snapchat via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Snapchat QR codes, Snapcodes, or AR filters. DECLENCHEURS: snapchat, snapcode, snapchat qr, snap, snapchat filter, ar filter. EXCLURE: instagram (stories), tiktok (video).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'whatsapp', locale_key: 'fr-FR'})
SET el.slug = 'whatsapp',
    el.display_name = 'WhatsApp',
    el.description = 'QR Code pour démarrer une conversation WhatsApp.',
    el.definition = 'Intégration QR Code pour la plateforme WhatsApp.',
    el.purpose = 'Connecter votre audience à WhatsApp via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de WhatsApp QR codes, click-to-chat, or WhatsApp business. DECLENCHEURS: whatsapp, wa, whatsapp qr, whatsapp chat, click to chat, whatsapp business. EXCLURE: telegram (alternative), messenger (facebook).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'telegram', locale_key: 'fr-FR'})
SET el.slug = 'telegram',
    el.display_name = 'Telegram',
    el.description = 'QR Code vers votre canal Telegram.',
    el.definition = 'Intégration QR Code pour la plateforme Telegram.',
    el.purpose = 'Connecter votre audience à Telegram via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Telegram QR codes, Telegram channels, or secure messaging. DECLENCHEURS: telegram, telegram qr, telegram channel, telegram group, secure chat. EXCLURE: whatsapp (alternative), signal (privacy).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'pinterest', locale_key: 'fr-FR'})
SET el.slug = 'pinterest',
    el.display_name = 'Pinterest',
    el.description = 'QR Code vers vos tableaux Pinterest.',
    el.definition = 'Intégration QR Code pour la plateforme Pinterest.',
    el.purpose = 'Connecter votre audience à Pinterest via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Pinterest QR codes, pins, or visual discovery. DECLENCHEURS: pinterest, pin, pinterest qr, pinterest board, visual discovery, pincode. EXCLURE: instagram (social), etsy (commerce).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'spotify', locale_key: 'fr-FR'})
SET el.slug = 'spotify',
    el.display_name = 'Spotify',
    el.description = 'QR Code vers votre musique Spotify.',
    el.definition = 'Intégration QR Code pour la plateforme Spotify.',
    el.purpose = 'Connecter votre audience à Spotify via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Spotify QR codes, Spotify Codes, or music streaming links. DECLENCHEURS: spotify, spotify code, spotify qr, music streaming, playlist qr, spotify playlist. EXCLURE: apple music (competitor), soundcloud (indie).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'apple', locale_key: 'fr-FR'})
SET el.slug = 'apple',
    el.display_name = 'Apple',
    el.description = 'L\'Applicationle pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme Apple.',
    el.purpose = 'Connecter votre audience à Apple via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Apple Music QR codes, App Store links, or Apple ecosystem. DECLENCHEURS: apple, apple music, app store, itunes, ios app, apple qr. EXCLURE: spotify (music streaming), google play (android).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'soundcloud', locale_key: 'fr-FR'})
SET el.slug = 'soundcloud',
    el.display_name = 'SoundCloud',
    el.description = 'Le SoundCloud pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme SoundCloud.',
    el.purpose = 'Connecter votre audience à SoundCloud via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de SoundCloud QR codes, indie music, or audio sharing. DECLENCHEURS: soundcloud, soundcloud qr, indie music, dj music, audio platform, music upload. EXCLURE: spotify (mainstream), apple music (apple).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'paypal', locale_key: 'fr-FR'})
SET el.slug = 'paypal',
    el.display_name = 'PayPal',
    el.description = 'QR Code pour paiement PayPal.',
    el.definition = 'Intégration QR Code pour la plateforme PayPal.',
    el.purpose = 'Connecter votre audience à PayPal via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de PayPal QR codes, PayPal.me links, or PayPal payments. DECLENCHEURS: paypal, paypal qr, paypal.me, paypal payment, online payment. EXCLURE: venmo (p2p), stripe (developer).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'venmo', locale_key: 'fr-FR'})
SET el.slug = 'venmo',
    el.display_name = 'Venmo',
    el.description = 'QR Code pour paiement Venmo.',
    el.definition = 'Intégration QR Code pour la plateforme Venmo.',
    el.purpose = 'Connecter votre audience à Venmo via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Venmo QR codes, Venmo payments, or peer-to-peer US payments. DECLENCHEURS: venmo, venmo qr, venmo payment, split bill, peer payment, p2p payment. EXCLURE: paypal (parent), zelle (bank).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'google', locale_key: 'fr-FR'})
SET el.slug = 'google',
    el.display_name = 'Google',
    el.description = 'Le Google pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme Google.',
    el.purpose = 'Connecter votre audience à Google via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Google QR codes, Google Maps, Google Reviews, or Google Business. DECLENCHEURS: google, google maps, google review, google business, google qr, play store. EXCLURE: apple (competitor), waze (navigation).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'waze', locale_key: 'fr-FR'})
SET el.slug = 'waze',
    el.display_name = 'Waze',
    el.description = 'QR Code vers une navigation Waze.',
    el.definition = 'Intégration QR Code pour la plateforme Waze.',
    el.purpose = 'Connecter votre audience à Waze via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Waze QR codes, Waze navigation, or community GPS. DECLENCHEURS: waze, waze qr, waze navigation, waze directions, community navigation. EXCLURE: google maps (google), apple maps (apple).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'zapier', locale_key: 'fr-FR'})
SET el.slug = 'zapier',
    el.display_name = 'Zapier',
    el.description = 'Le Zapier pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme Zapier.',
    el.purpose = 'Connecter votre audience à Zapier via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Zapier, workflow automation, or app integrations. DECLENCHEURS: zapier, zap, zapier automation, workflow automation, app connector. EXCLURE: make (integromat), n8n (self-hosted).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'make', locale_key: 'fr-FR'})
SET el.slug = 'make',
    el.display_name = 'Make (Integromat)',
    el.description = 'Le Make (Integromat) pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme Make (Integromat).',
    el.purpose = 'Connecter votre audience à Make (Integromat) via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Make/Integromat, visual automation, or complex workflows. DECLENCHEURS: make, integromat, make automation, visual automation, scenario builder. EXCLURE: zapier (simpler), n8n (self-hosted).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'n8n', locale_key: 'fr-FR'})
SET el.slug = 'n8n',
    el.display_name = 'n8n',
    el.description = 'Le n8n pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme n8n.',
    el.purpose = 'Connecter votre audience à n8n via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de n8n, self-hosted automation, or open-source workflows. DECLENCHEURS: n8n, self-hosted automation, open source automation, privacy automation. EXCLURE: zapier (hosted), make (hosted).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'hubspot', locale_key: 'fr-FR'})
SET el.slug = 'hubspot',
    el.display_name = 'HubSpot',
    el.description = 'L\'HubSpot pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme HubSpot.',
    el.purpose = 'Connecter votre audience à HubSpot via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de HubSpot, CRM integration, or marketing automation with QR codes. DECLENCHEURS: hubspot, hubspot crm, hubspot marketing, inbound marketing, hubspot integration. EXCLURE: salesforce (enterprise), mailchimp (email only).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'salesforce', locale_key: 'fr-FR'})
SET el.slug = 'salesforce',
    el.display_name = 'Salesforce',
    el.description = 'Le Salesforce pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme Salesforce.',
    el.purpose = 'Connecter votre audience à Salesforce via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Salesforce, enterprise CRM, or sales automation. DECLENCHEURS: salesforce, salesforce crm, enterprise crm, salesforce integration, sales cloud. EXCLURE: hubspot (smb), zoho (alternative).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'mailchimp', locale_key: 'fr-FR'})
SET el.slug = 'mailchimp',
    el.display_name = 'Mailchimp',
    el.description = 'Le Mailchimp pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme Mailchimp.',
    el.purpose = 'Connecter votre audience à Mailchimp via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Mailchimp, email marketing, or newsletter QR codes. DECLENCHEURS: mailchimp, email marketing, mailchimp integration, newsletter qr, email list qr. EXCLURE: hubspot (full crm), sendgrid (api).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'shopify', locale_key: 'fr-FR'})
SET el.slug = 'shopify',
    el.display_name = 'Shopify',
    el.description = 'Le Shopify pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme Shopify.',
    el.purpose = 'Connecter votre audience à Shopify via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de Shopify, e-commerce QR codes, or online store integration. DECLENCHEURS: shopify, shopify qr, shopify store, e-commerce platform, shopify product. EXCLURE: woocommerce (wordpress), amazon (marketplace).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'woocommerce', locale_key: 'fr-FR'})
SET el.slug = 'woocommerce',
    el.display_name = 'WooCommerce',
    el.description = 'Le WooCommerce pour vos projets QR Code.',
    el.definition = 'Intégration QR Code pour la plateforme WooCommerce.',
    el.purpose = 'Connecter votre audience à WooCommerce via un QR Code personnalisé.',
    el.llm_context = 'UTILISER: pour discuter de WooCommerce, WordPress e-commerce, or open-source stores. DECLENCHEURS: woocommerce, woo commerce, wordpress store, woocommerce qr, wordpress e-commerce. EXCLURE: shopify (hosted), magento (enterprise).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'zapier-integration', locale_key: 'fr-FR'})
SET el.slug = 'zapier-integration',
    el.display_name = 'Zapier Integration',
    el.description = 'La Zapier Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et Zapier Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec Zapier Integration.',
    el.llm_context = 'UTILISER: pour discuter de Zapier integration with QR Code AI, automated QR workflows, or zap connections. DECLENCHEURS: zapier integration, qr zapier, zap qr, automate qr, zapier connection. EXCLURE: make integration (different platform), direct api (not integration).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'make-integration', locale_key: 'fr-FR'})
SET el.slug = 'make-integration',
    el.display_name = 'Make Integration',
    el.description = 'La Make Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et Make Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec Make Integration.',
    el.llm_context = 'UTILISER: pour discuter de Make/Integromat integration with QR Code AI or visual scenario automation. DECLENCHEURS: make integration, integromat integration, qr make, qr integromat, visual automation integration. EXCLURE: zapier integration (different platform), n8n integration (self-hosted).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'n8n-integration', locale_key: 'fr-FR'})
SET el.slug = 'n8n-integration',
    el.display_name = 'n8n Integration',
    el.description = 'La n8n Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et n8n Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec n8n Integration.',
    el.llm_context = 'UTILISER: pour discuter de n8n integration with QR Code AI or self-hosted privacy-first automation. DECLENCHEURS: n8n integration, qr n8n, self-hosted qr automation, privacy qr automation. EXCLURE: zapier integration (hosted), make integration (hosted).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'hubspot-integration', locale_key: 'fr-FR'})
SET el.slug = 'hubspot-integration',
    el.display_name = 'HubSpot Integration',
    el.description = 'L\'HubSpot Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et HubSpot Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec HubSpot Integration.',
    el.llm_context = 'UTILISER: pour discuter de HubSpot integration with QR Code AI or QR-to-CRM lead syncing. DECLENCHEURS: hubspot integration, qr hubspot, crm qr integration, hubspot qr sync, marketing automation qr. EXCLURE: salesforce integration (enterprise), mailchimp integration (email).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'salesforce-integration', locale_key: 'fr-FR'})
SET el.slug = 'salesforce-integration',
    el.display_name = 'Salesforce Integration',
    el.description = 'La Salesforce Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et Salesforce Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec Salesforce Integration.',
    el.llm_context = 'UTILISER: pour discuter de Salesforce integration with QR Code AI or enterprise CRM QR connections. DECLENCHEURS: salesforce integration, qr salesforce, enterprise crm qr, salesforce qr sync. EXCLURE: hubspot integration (smb), dynamics integration (microsoft).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'mailchimp-integration', locale_key: 'fr-FR'})
SET el.slug = 'mailchimp-integration',
    el.display_name = 'Mailchimp Integration',
    el.description = 'La Mailchimp Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et Mailchimp Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec Mailchimp Integration.',
    el.llm_context = 'UTILISER: pour discuter de Mailchimp integration with QR Code AI or QR-to-email list building. DECLENCHEURS: mailchimp integration, qr mailchimp, email qr integration, newsletter qr signup, list building qr. EXCLURE: hubspot integration (full crm), sendgrid (api only).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'google-sheets-integration', locale_key: 'fr-FR'})
SET el.slug = 'google-sheets-integration',
    el.display_name = 'Google Sheets Integration',
    el.description = 'La Google Sheets Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et Google Sheets Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec Google Sheets Integration.',
    el.llm_context = 'UTILISER: pour discuter de Google Sheets integration with QR Code AI or spreadsheet QR data sync. DECLENCHEURS: google sheets integration, qr google sheets, spreadsheet qr, sheets qr sync, batch qr from sheets. EXCLURE: notion integration (workspace), airtable (database).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'notion-integration', locale_key: 'fr-FR'})
SET el.slug = 'notion-integration',
    el.display_name = 'Notion Integration',
    el.description = 'La Notion Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et Notion Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec Notion Integration.',
    el.llm_context = 'UTILISER: pour discuter de Notion integration with QR Code AI or workspace QR management. DECLENCHEURS: notion integration, qr notion, notion qr sync, workspace qr, notion database qr. EXCLURE: google sheets integration (spreadsheet), coda (alternative).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'slack-integration', locale_key: 'fr-FR'})
SET el.slug = 'slack-integration',
    el.display_name = 'Slack Integration',
    el.description = 'La Slack Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et Slack Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec Slack Integration.',
    el.llm_context = 'UTILISER: pour discuter de Slack integration with QR Code AI or QR scan notifications. DECLENCHEURS: slack integration, qr slack, slack notifications qr, team alert qr, slack channel qr. EXCLURE: teams integration (microsoft), discord (community).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'shopify-integration', locale_key: 'fr-FR'})
SET el.slug = 'shopify-integration',
    el.display_name = 'Shopify Integration',
    el.description = 'La Shopify Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et Shopify Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec Shopify Integration.',
    el.llm_context = 'UTILISER: pour discuter de Shopify integration with QR Code AI or e-commerce product QR codes. DECLENCHEURS: shopify integration, qr shopify, shopify product qr, e-commerce qr integration, shopify store qr. EXCLURE: woocommerce integration (wordpress), amazon (marketplace).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'woocommerce-integration', locale_key: 'fr-FR'})
SET el.slug = 'woocommerce-integration',
    el.display_name = 'WooCommerce Integration',
    el.description = 'La WooCommerce Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et WooCommerce Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec WooCommerce Integration.',
    el.llm_context = 'UTILISER: pour discuter de WooCommerce integration with QR Code AI or WordPress e-commerce QR codes. DECLENCHEURS: woocommerce integration, qr woocommerce, wordpress qr store, woo qr integration. EXCLURE: shopify integration (hosted), magento (enterprise).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'wordpress-integration', locale_key: 'fr-FR'})
SET el.slug = 'wordpress-integration',
    el.display_name = 'WordPress Integration',
    el.description = 'La WordPress Integration pour vos projets QR Code.',
    el.definition = 'Connexion API entre QR Code AI et WordPress Integration.',
    el.purpose = 'Automatiser la création et la gestion de QR Codes avec WordPress Integration.',
    el.llm_context = 'UTILISER: pour discuter de WordPress integration with QR Code AI or embedding QR codes in WordPress. DECLENCHEURS: wordpress integration, qr wordpress, wordpress qr plugin, wp qr shortcode, cms qr integration. EXCLURE: woocommerce integration (e-commerce), squarespace (different cms).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'quiet-zone', locale_key: 'fr-FR'})
SET el.slug = 'quiet-zone',
    el.display_name = 'Quiet Zone',
    el.description = 'Le Quiet Zone pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les Quiet Zone.',
    el.purpose = 'Exploitez les avantages des Quiet Zone pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de QR code margins, white border requirements, or scan failures due to cropping. DECLENCHEURS: quiet zone, qr margin, white border, qr padding, module margin, quiet area. EXCLURE: finder pattern (corners), timing pattern (lines).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'error-correction', locale_key: 'fr-FR'})
SET el.slug = 'error-correction',
    el.display_name = 'Error Correction',
    el.description = 'L\'Error Correction pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les Error Correction.',
    el.purpose = 'Exploitez les avantages des Error Correction pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de QR code damage tolerance, logo placement, or error correction levels L/M/Q/H. DECLENCHEURS: error correction, qr damage, reed solomon, correction level, damaged qr, logo error correction. EXCLURE: data capacity (size), encoding mode (character type).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'data-capacity', locale_key: 'fr-FR'})
SET el.slug = 'data-capacity',
    el.display_name = 'Data Capacity',
    el.description = 'Le Data Capacity pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les Data Capacity.',
    el.purpose = 'Exploitez les avantages des Data Capacity pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de how much data a QR code can hold, character limits, or QR size requirements. DECLENCHEURS: data capacity, qr capacity, character limit, qr data size, how much data, qr storage. EXCLURE: error correction (damage), encoding mode (format).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-version', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-version',
    el.display_name = 'QR Code Version',
    el.description = 'La QR Code Version pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les QR Code Version.',
    el.purpose = 'Exploitez les avantages des QR Code Version pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de QR code size, version numbers 1-40, or module grid dimensions. DECLENCHEURS: qr version, qr size, version 1, version 40, module grid, qr dimensions. EXCLURE: data capacity (characters), error correction (damage).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'encoding-mode', locale_key: 'fr-FR'})
SET el.slug = 'encoding-mode',
    el.display_name = 'Encoding Mode',
    el.description = 'L\'Encoding Mode pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les Encoding Mode.',
    el.purpose = 'Exploitez les avantages des Encoding Mode pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de QR encoding efficiency, character types, or numeric vs alphanumeric modes. DECLENCHEURS: encoding mode, numeric mode, alphanumeric mode, byte mode, kanji mode, qr encoding. EXCLURE: error correction (damage), data capacity (size).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'module', locale_key: 'fr-FR'})
SET el.slug = 'module',
    el.display_name = 'Module',
    el.description = 'Le Module pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les Module.',
    el.purpose = 'Exploitez les avantages des Module pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de QR code pixels, individual squares, or module-level design. DECLENCHEURS: module, qr pixel, qr square, black module, white module, qr unit. EXCLURE: finder pattern (corner squares), quiet zone (border).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'finder-pattern', locale_key: 'fr-FR'})
SET el.slug = 'finder-pattern',
    el.display_name = 'Finder Pattern',
    el.description = 'Le Finder Pattern pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les Finder Pattern.',
    el.purpose = 'Exploitez les avantages des Finder Pattern pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de QR code corner squares, orientation markers, or why QR codes have three big squares. DECLENCHEURS: finder pattern, corner squares, qr orientation, position detection, three squares, qr corners. EXCLURE: timing pattern (lines), alignment pattern (small squares).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'timing-pattern', locale_key: 'fr-FR'})
SET el.slug = 'timing-pattern',
    el.display_name = 'Timing Pattern',
    el.description = 'Le Timing Pattern pour vos projets QR Code.',
    el.definition = 'Concept clé pour comprendre et maîtriser les Timing Pattern.',
    el.purpose = 'Exploitez les avantages des Timing Pattern pour vos campagnes marketing.',
    el.llm_context = 'UTILISER: pour discuter de QR code alignment, alternating black/white lines, or grid calibration. DECLENCHEURS: timing pattern, alignment lines, alternating modules, grid calibration, qr timing. EXCLURE: finder pattern (corners), module (single square).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-messaging', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-messaging',
    el.display_name = 'Messaging QR Codes',
    el.description = 'Les Messaging QR Codes pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Messaging QR Codes.',
    el.purpose = 'Optimisez vos campagnes marketing avec Messaging QR Codes personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for messaging apps like WhatsApp, Telegram, or chat platforms. DECLENCHEURS: messaging qr, chat qr, whatsapp category, telegram category, direct message qr. EXCLURE: video platform (video content), professional (business networking).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-video-platform', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-video-platform',
    el.display_name = 'Vidéo Platform QR Codes',
    el.description = 'Les Vidéo Platform QR Codes pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Vidéo Platform QR Codes.',
    el.purpose = 'Optimisez vos campagnes marketing avec Vidéo Platform QR Codes personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for video platforms like YouTube, TikTok, or Snapchat. DECLENCHEURS: video platform qr, youtube category, tiktok category, video content qr, streaming qr. EXCLURE: messaging (chat), music platform (audio).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-professional', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-professional',
    el.display_name = 'Professional Network QR Codes',
    el.description = 'Les Professional Network QR Codes pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Professional Network QR Codes.',
    el.purpose = 'Optimisez vos campagnes marketing avec Professional Network QR Codes personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for professional networking like LinkedIn. DECLENCHEURS: professional qr, linkedin category, business networking qr, career qr, professional network. EXCLURE: messaging (chat), video platform (content).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-music-platform', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-music-platform',
    el.display_name = 'Music Platform QR Codes',
    el.description = 'Les Music Platform QR Codes pour vos projets QR Code.',
    el.definition = 'Solution QR Code AI pour la création et gestion de Music Platform QR Codes.',
    el.purpose = 'Optimisez vos campagnes marketing avec Music Platform QR Codes personnalisés.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for music platforms like Spotify, Apple Music, or SoundCloud. DECLENCHEURS: music platform qr, spotify category, apple music category, music streaming qr, playlist category. EXCLURE: video platform (video), audio file (direct file).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'funny-qr-codes', locale_key: 'fr-FR'})
SET el.slug = 'funny-qr-codes',
    el.display_name = 'Funny QR Codes',
    el.description = 'Les Funny QR Codes pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel Funny QR Codes avec QR Codes.',
    el.purpose = 'Implémentez Funny QR Codes dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de humorous, prank, or creative QR code applications like rickrolling. DECLENCHEURS: funny qr, prank qr, rickroll qr, meme qr, easter egg qr, joke qr. EXCLURE: art installation (serious art), tattoo (permanent).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-tattoo', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-tattoo',
    el.display_name = 'QR Code Tattoo',
    el.description = 'Le QR Code Tattoo pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel QR Code Tattoo avec QR Codes.',
    el.purpose = 'Implémentez QR Code Tattoo dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR code tattoos, permanent body art QR codes, or skin-based QR. DECLENCHEURS: qr tattoo, tattoo qr code, body art qr, permanent qr, skin qr, inked qr. EXCLURE: temporary (sticker), art installation (public).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-art-installation', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-art-installation',
    el.display_name = 'QR Art Installation',
    el.description = 'La QR Art Installation pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel QR Art Installation avec QR Codes.',
    el.purpose = 'Implémentez QR Art Installation dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de large-scale QR art, public installations, or artistic QR displays. DECLENCHEURS: qr art, art installation qr, mural qr, public art qr, interactive art qr, projection qr. EXCLURE: tattoo (body), funny qr (humor).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-scavenger-hunt', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-scavenger-hunt',
    el.display_name = 'QR Scavenger Hunt',
    el.description = 'Le QR Scavenger Hunt pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel QR Scavenger Hunt avec QR Codes.',
    el.purpose = 'Implémentez QR Scavenger Hunt dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR scavenger hunts, treasure hunts, or gamified QR experiences. DECLENCHEURS: scavenger hunt qr, treasure hunt qr, qr game, interactive qr hunt, clue qr, quest qr. EXCLURE: event check-in (registration), mutiliserum (educational).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-reviews', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-reviews',
    el.display_name = 'QR for Reviews',
    el.description = 'Les QR for Reviews pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel QR for Reviews avec QR Codes.',
    el.purpose = 'Implémentez QR for Reviews dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for collecting reviews, feedback, or ratings. DECLENCHEURS: review qr, feedback qr, rating qr, google review qr, yelp qr, customer feedback qr. EXCLURE: loyalty program (rewards), payment (transaction).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-contactless-payment', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-contactless-payment',
    el.display_name = 'Contactless Paiement',
    el.description = 'Le Contactless Paiement pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel Contactless Paiement avec QR Codes.',
    el.purpose = 'Implémentez Contactless Paiement dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR-based contactless payments, touch-free transactions, or scan-to-pay. DECLENCHEURS: contactless payment qr, scan to pay, touch-free payment, qr payment, mobile payment qr. EXCLURE: loyalty program (points), reviews (feedback).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-loyalty-program', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-programme-fidelite',
    el.display_name = 'Programme de Fidélité',
    el.description = 'La Programme de Fidélité pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel Programme de Fidélité avec QR Codes.',
    el.purpose = 'Implémentez Programme de Fidélité dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR-based loyalty programs, digital punch cards, or rewards via QR. DECLENCHEURS: loyalty qr, punch card qr, rewards qr, points qr, member qr, stamp card qr. EXCLURE: payment (transaction), reviews (feedback).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-product-authentication', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-authentification-produit',
    el.display_name = 'Authentification Produit',
    el.description = 'L\'Authentification Produit pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel Authentification Produit avec QR Codes.',
    el.purpose = 'Implémentez Authentification Produit dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for product authentication, anti-counterfeiting, or verification. DECLENCHEURS: authentication qr, anti-counterfeit qr, verify product qr, genuine qr, counterfeit detection qr. EXCLURE: loyalty (rewards), payment (transaction).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-event-checkin', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-event-checkin',
    el.display_name = 'Événement Check-in',
    el.description = 'Le Événement Check-in pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel Événement Check-in avec QR Codes.',
    el.purpose = 'Implémentez Événement Check-in dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for event check-in, registration, or attendee verification. DECLENCHEURS: event check-in qr, registration qr, attendee qr, ticket scan, conference check-in, entry qr. EXCLURE: networking (contact exchange), wedding (personal event).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-networking', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-networking',
    el.display_name = 'Networking QR',
    el.description = 'Le Networking QR pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel Networking QR avec QR Codes.',
    el.purpose = 'Implémentez Networking QR dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for professional networking, contact exchange, or event connections. DECLENCHEURS: networking qr, contact exchange qr, meet and greet qr, business card qr, connection qr. EXCLURE: event check-in (registration), wedding (personal).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-wedding', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-wedding',
    el.display_name = 'Wedding QR Codes',
    el.description = 'Les Wedding QR Codes pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel Wedding QR Codes avec QR Codes.',
    el.purpose = 'Implémentez Wedding QR Codes dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for weddings, wedding invitations, or wedding RSVPs. DECLENCHEURS: wedding qr, wedding invitation qr, rsvp qr, wedding registry qr, wedding photo qr, marriage qr. EXCLURE: event check-in (corporate), networking (business).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-museum-exhibit', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-museum-exhibit',
    el.display_name = 'Museum Exhibits',
    el.description = 'Les Museum Exhibits pour vos projets QR Code.',
    el.definition = 'Cas d\'usage professionnel Museum Exhibits avec QR Codes.',
    el.purpose = 'Implémentez Museum Exhibits dans votre organisation grâce aux QR Codes.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for mutiliserums, exhibits, galleries, or educational displays. DECLENCHEURS: mutiliserum qr, exhibit qr, gallery qr, audio guide qr, art mutiliserum qr, exhibition qr. EXCLURE: scavenger hunt (game), art installation (creative).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'how-to-create-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'comment-creer-qr-code',
    el.display_name = 'Comment Créer QR Code',
    el.description = 'Guide complet pour créer votre premier QR Code.',
    el.definition = 'Guide pratique et tutoriel : Comment Créer QR Code.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide Comment Créer QR Code.',
    el.llm_context = 'UTILISER: pour discuter de how to create QR codes, QR creation tutorials, or step-by-step QR generation. DECLENCHEURS: how to create qr, make qr code, qr tutorial, create qr guide, generate qr how to. EXCLURE: design guide (aesthetics), print guide (physical output).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-design-guide', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-design-guide',
    el.display_name = 'QR Code Design Guide',
    el.description = 'Le QR Code Design Guide pour vos projets QR Code.',
    el.definition = 'Guide pratique et tutoriel : QR Code Design Guide.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide QR Code Design Guide.',
    el.llm_context = 'UTILISER: pour discuter de QR code design, styling, customization best practices, or visual aesthetics. DECLENCHEURS: qr design, qr styling, qr aesthetics, beautiful qr, custom qr design, qr appearance. EXCLURE: creation guide (basic), print guide (output).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-print-guide', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-imprimer-guide',
    el.display_name = 'QR Code Imprimer Guide',
    el.description = 'Le QR Code Imprimer Guide pour vos projets QR Code.',
    el.definition = 'Guide pratique et tutoriel : QR Code Imprimer Guide.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide QR Code Imprimer Guide.',
    el.llm_context = 'UTILISER: pour discuter de QR code printing, print requirements, or physical QR production. DECLENCHEURS: qr print, print qr guide, qr printing tips, physical qr, qr size for print, print quality qr. EXCLURE: design guide (digital), creation guide (generation).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'dynamic-vs-static-guide', locale_key: 'fr-FR'})
SET el.slug = 'dynamique-vs-statique-guide',
    el.display_name = 'Dynamique vs Statique Guide',
    el.description = 'Le Dynamique vs Statique Guide pour vos projets QR Code.',
    el.definition = 'Guide pratique et tutoriel : Dynamique vs Statique Guide.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide Dynamique vs Statique Guide.',
    el.llm_context = 'UTILISER: pour discuter de choosing between dynamic and static QR codes or comparing QR types. DECLENCHEURS: dynamic vs static, which qr type, qr type comparison, editable qr, trackable qr choice. EXCLURE: comparison entity (detailed), creation guide (how-to).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-marketing-guide', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-marketing-guide',
    el.display_name = 'QR Marketing Guide',
    el.description = 'Le QR Marketing Guide pour vos projets QR Code.',
    el.definition = 'Guide pratique et tutoriel : QR Marketing Guide.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide QR Marketing Guide.',
    el.llm_context = 'UTILISER: pour discuter de QR codes for marketing, campaign strategies, or print-to-digital marketing. DECLENCHEURS: qr marketing, marketing qr guide, campaign qr, advertising qr, roi qr, print to digital. EXCLURE: restaurant guide (industry), business card guide (specific utiliser).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-restaurant-guide', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-restaurant-guide',
    el.display_name = 'Restaurant QR Guide',
    el.description = 'Le Restaurant QR Guide pour vos projets QR Code.',
    el.definition = 'Guide pratique et tutoriel : Restaurant QR Guide.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide Restaurant QR Guide.',
    el.llm_context = 'UTILISER: pour discuter de QR codes specifically for restaurants, cafes, or food service. DECLENCHEURS: restaurant qr guide, menu qr, cafe qr, food service qr, dining qr, table qr guide. EXCLURE: marketing guide (general), business card guide (networking).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-business-card-guide', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-carte-visite-guide',
    el.display_name = 'Carte de Visite QR Guide',
    el.description = 'La Carte de Visite QR Guide pour vos projets QR Code.',
    el.definition = 'Guide pratique et tutoriel : Carte de Visite QR Guide.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide Carte de Visite QR Guide.',
    el.llm_context = 'UTILISER: pour discuter de QR codes on business cards, networking cards, or professional contact sharing. DECLENCHEURS: business card qr guide, vcard qr, networking card qr, professional qr, contact card qr. EXCLURE: restaurant guide (food), marketing guide (campaigns).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-api-guide', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-api-guide',
    el.display_name = 'QR Code API Guide',
    el.description = 'Le QR Code API Guide pour vos projets QR Code.',
    el.definition = 'Guide pratique et tutoriel : QR Code API Guide.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide QR Code API Guide.',
    el.llm_context = 'UTILISER: pour discuter de QR code API documentation, developer integration, or programmatic QR generation. DECLENCHEURS: qr api guide, developer qr, api integration qr, programmatic qr, qr api docs. EXCLURE: analytics guide (tracking), security guide (safety).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-analytics-guide', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-analytiques-guide',
    el.display_name = 'Analytiques Guide',
    el.description = 'L\'Analytiques Guide pour vos projets QR Code.',
    el.definition = 'Guide pratique et tutoriel : Analytiques Guide.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide Analytiques Guide.',
    el.llm_context = 'UTILISER: pour discuter de QR scan analytics, tracking interpretation, or campaign measurement. DECLENCHEURS: qr analytics guide, scan tracking guide, qr metrics, analytics interpretation, campaign analytics. EXCLURE: api guide (development), marketing guide (strategy).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-security-guide', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-security-guide',
    el.display_name = 'QR Security Guide',
    el.description = 'Le QR Security Guide pour vos projets QR Code.',
    el.definition = 'Guide pratique et tutoriel : QR Security Guide.',
    el.purpose = 'Apprenez à maîtriser les QR Codes avec ce guide QR Security Guide.',
    el.llm_context = 'UTILISER: pour discuter de QR code security, phishing prevention, or safe QR practices. DECLENCHEURS: qr security, safe qr, qr phishing, qr safety, malicious qr, secure qr. EXCLURE: api guide (development), analytics guide (tracking).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-vs-barcode', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-vs-code-barres',
    el.display_name = 'QR Code vs Code-barres',
    el.description = 'Différences entre QR Codes et codes-barres traditionnels.',
    el.definition = 'Comparatif détaillé : QR Code vs Code-barres.',
    el.purpose = 'Faites le bon choix grâce à notre analyse QR Code vs Code-barres.',
    el.llm_context = 'UTILISER: pour discuter de differences between QR codes and traditional barcodes or 1D vs 2D codes. DECLENCHEURS: qr vs barcode, barcode vs qr, 1d vs 2d, qr or barcode, qr code barcode difference. EXCLURE: qr vs nfc (wireless), qr vs data matrix (both 2D).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'dynamic-vs-static-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'dynamique-vs-statique-qr-code',
    el.display_name = 'Dynamique vs QR Code Statique',
    el.description = 'Comparatif entre QR Codes dynamiques et statiques.',
    el.definition = 'Comparatif détaillé : Dynamique vs QR Code Statique.',
    el.purpose = 'Faites le bon choix grâce à notre analyse Dynamique vs QR Code Statique.',
    el.llm_context = 'UTILISER: pour discuter de differences between dynamic and static QR codes or editable vs permanent QR. DECLENCHEURS: dynamic vs static qr, editable qr, trackable qr, static vs dynamic, permanent qr vs editable. EXCLURE: guide (how-to), qr vs barcode (different formats).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-vs-nfc', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-vs-nfc',
    el.display_name = 'QR Code vs NFC',
    el.description = 'QR Code vs NFC : quelle technologie choisir ?',
    el.definition = 'Comparatif détaillé : QR Code vs NFC.',
    el.purpose = 'Faites le bon choix grâce à notre analyse QR Code vs NFC.',
    el.llm_context = 'UTILISER: pour discuter de differences between QR codes and NFC or visual vs tap technology. DECLENCHEURS: qr vs nfc, nfc vs qr, scan vs tap, qr or nfc, contactless comparison. EXCLURE: qr vs barcode (both visual), qr vs data matrix (both 2D).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-vs-data-matrix', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-vs-data-matrix',
    el.display_name = 'QR Code vs Data Matrix',
    el.description = 'Le QR Code vs Data Matrix pour vos projets QR Code.',
    el.definition = 'Comparatif détaillé : QR Code vs Data Matrix.',
    el.purpose = 'Faites le bon choix grâce à notre analyse QR Code vs Data Matrix.',
    el.llm_context = 'UTILISER: pour discuter de differences between QR codes and Data Matrix or consumer vs industrial 2D codes. DECLENCHEURS: qr vs data matrix, data matrix vs qr, which 2d code, qr or datamatrix. EXCLURE: qr vs barcode (1D), qr vs nfc (wireless).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'free-vs-paid-qr-generator', locale_key: 'fr-FR'})
SET el.slug = 'free-vs-paid-qr-generateur',
    el.display_name = 'Gratuit vs Premium QR Générateur',
    el.description = 'Le Gratuit vs Premium QR Générateur pour vos projets QR Code.',
    el.definition = 'Comparatif détaillé : Gratuit vs Premium QR Générateur.',
    el.purpose = 'Faites le bon choix grâce à notre analyse Gratuit vs Premium QR Générateur.',
    el.llm_context = 'UTILISER: pour discuter de free vs paid QR generators, pricing tiers, or premium QR features. DECLENCHEURS: free vs paid qr, qr generator pricing, premium qr, free qr limitations, paid qr benefits. EXCLURE: platform comparison (specific tools), dynamic vs static (code types).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'qr-code-ai-vs-competitors', locale_key: 'fr-FR'})
SET el.slug = 'qr-code-ai-vs-competitors',
    el.display_name = 'QR Code AI vs Competitors',
    el.description = 'Les QR Code AI vs Competitors pour vos projets QR Code.',
    el.definition = 'Comparatif détaillé : QR Code AI vs Competitors.',
    el.purpose = 'Faites le bon choix grâce à notre analyse QR Code AI vs Competitors.',
    el.llm_context = 'UTILISER: pour discuter de QR Code AI platform comparisons or competitive analysis. DECLENCHEURS: qr code ai vs, compare qr platforms, qr code ai alternative, qr generator comparison, best qr platform. EXCLURE: free vs paid (pricing), dynamic vs static (code types).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'short-link-vs-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'lien-court-vs-qr-code',
    el.display_name = 'Lien Court vs QR Code',
    el.description = 'Le Lien Court vs QR Code pour vos projets QR Code.',
    el.definition = 'Comparatif détaillé : Lien Court vs QR Code.',
    el.purpose = 'Faites le bon choix grâce à notre analyse Lien Court vs QR Code.',
    el.llm_context = 'UTILISER: pour discuter de short links vs QR codes or when to utiliser each technology. DECLENCHEURS: short link vs qr, url vs qr, link or qr, bitly vs qr, when to utiliser qr. EXCLURE: dynamic vs static (both QR), qr vs nfc (hardware).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: 'spotify-code-vs-qr-code', locale_key: 'fr-FR'})
SET el.slug = 'spotify-code-vs-qr-code',
    el.display_name = 'Spotify Code vs QR Code',
    el.description = 'Le Spotify Code vs QR Code pour vos projets QR Code.',
    el.definition = 'Comparatif détaillé : Spotify Code vs QR Code.',
    el.purpose = 'Faites le bon choix grâce à notre analyse Spotify Code vs QR Code.',
    el.llm_context = 'UTILISER: pour discuter de Spotify Codes vs standard QR codes or proprietary vs universal codes. DECLENCHEURS: spotify code vs qr, spotify qr, spotify code difference, proprietary qr, music qr comparison. EXCLURE: qr vs barcode (format), platform comparison (generators).',
    el.version = 1,
    el.created_at = datetime(),
    el.updated_at = datetime();

// -------------------------------------------------------------------
// Create relations to parent Entity (HAS_L10N)
// -------------------------------------------------------------------

MATCH (e:Entity {key: 'qr-code'})
MATCH (el:EntityL10n {entity_key: 'qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'smart-link'})
MATCH (el:EntityL10n {entity_key: 'smart-link', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'barcode'})
MATCH (el:EntityL10n {entity_key: 'barcode', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'landing-page'})
MATCH (el:EntityL10n {entity_key: 'landing-page', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'short-link'})
MATCH (el:EntityL10n {entity_key: 'short-link', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-style'})
MATCH (el:EntityL10n {entity_key: 'qr-code-style', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-content'})
MATCH (el:EntityL10n {entity_key: 'qr-code-content', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-frame'})
MATCH (el:EntityL10n {entity_key: 'qr-code-frame', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'barcode-format'})
MATCH (el:EntityL10n {entity_key: 'barcode-format', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'landing-page-type'})
MATCH (el:EntityL10n {entity_key: 'landing-page-type', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'custom-qr-code'})
MATCH (el:EntityL10n {entity_key: 'custom-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-image'})
MATCH (el:EntityL10n {entity_key: 'qr-code-image', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-art'})
MATCH (el:EntityL10n {entity_key: 'qr-code-art', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-photo'})
MATCH (el:EntityL10n {entity_key: 'qr-code-photo', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-with-logo'})
MATCH (el:EntityL10n {entity_key: 'qr-code-with-logo', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-with-text'})
MATCH (el:EntityL10n {entity_key: 'qr-code-with-text', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-color'})
MATCH (el:EntityL10n {entity_key: 'qr-code-color', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-shapes'})
MATCH (el:EntityL10n {entity_key: 'qr-code-shapes', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-transparent-background'})
MATCH (el:EntityL10n {entity_key: 'qr-code-transparent-background', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-background'})
MATCH (el:EntityL10n {entity_key: 'qr-code-background', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-background-color'})
MATCH (el:EntityL10n {entity_key: 'qr-code-background-color', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-background-gradient'})
MATCH (el:EntityL10n {entity_key: 'qr-code-background-gradient', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-background-image'})
MATCH (el:EntityL10n {entity_key: 'qr-code-background-image', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'dynamic-qr-code'})
MATCH (el:EntityL10n {entity_key: 'dynamic-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'static-qr-code'})
MATCH (el:EntityL10n {entity_key: 'static-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-light-mode'})
MATCH (el:EntityL10n {entity_key: 'qr-code-light-mode', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-dark-mode'})
MATCH (el:EntityL10n {entity_key: 'qr-code-dark-mode', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-business-card'})
MATCH (el:EntityL10n {entity_key: 'qr-code-business-card', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-email-signature'})
MATCH (el:EntityL10n {entity_key: 'qr-code-email-signature', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-flyer'})
MATCH (el:EntityL10n {entity_key: 'qr-code-flyer', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-poster'})
MATCH (el:EntityL10n {entity_key: 'qr-code-poster', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-table-tent'})
MATCH (el:EntityL10n {entity_key: 'qr-code-table-tent', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-packaging-label'})
MATCH (el:EntityL10n {entity_key: 'qr-code-packaging-label', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'link-in-bio'})
MATCH (el:EntityL10n {entity_key: 'link-in-bio', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'menu-restaurant'})
MATCH (el:EntityL10n {entity_key: 'menu-restaurant', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'forms'})
MATCH (el:EntityL10n {entity_key: 'forms', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'announcement'})
MATCH (el:EntityL10n {entity_key: 'announcement', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'event-rsvp'})
MATCH (el:EntityL10n {entity_key: 'event-rsvp', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'booking-appointment'})
MATCH (el:EntityL10n {entity_key: 'booking-appointment', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-url'})
MATCH (el:EntityL10n {entity_key: 'qr-code-url', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-wifi'})
MATCH (el:EntityL10n {entity_key: 'qr-code-wifi', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-vcard'})
MATCH (el:EntityL10n {entity_key: 'qr-code-vcard', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-mecard'})
MATCH (el:EntityL10n {entity_key: 'qr-code-mecard', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-pdf'})
MATCH (el:EntityL10n {entity_key: 'qr-code-pdf', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-text'})
MATCH (el:EntityL10n {entity_key: 'qr-code-text', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-email'})
MATCH (el:EntityL10n {entity_key: 'qr-code-email', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-sms'})
MATCH (el:EntityL10n {entity_key: 'qr-code-sms', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-phone'})
MATCH (el:EntityL10n {entity_key: 'qr-code-phone', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-video'})
MATCH (el:EntityL10n {entity_key: 'qr-code-video', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-audio'})
MATCH (el:EntityL10n {entity_key: 'qr-code-audio', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-image-gallery'})
MATCH (el:EntityL10n {entity_key: 'qr-code-image-gallery', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-coupon'})
MATCH (el:EntityL10n {entity_key: 'qr-code-coupon', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-social'})
MATCH (el:EntityL10n {entity_key: 'qr-code-social', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-instagram'})
MATCH (el:EntityL10n {entity_key: 'qr-code-instagram', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-linkedin'})
MATCH (el:EntityL10n {entity_key: 'qr-code-linkedin', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-facebook'})
MATCH (el:EntityL10n {entity_key: 'qr-code-facebook', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-twitter'})
MATCH (el:EntityL10n {entity_key: 'qr-code-twitter', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-youtube'})
MATCH (el:EntityL10n {entity_key: 'qr-code-youtube', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-tiktok'})
MATCH (el:EntityL10n {entity_key: 'qr-code-tiktok', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-snapchat'})
MATCH (el:EntityL10n {entity_key: 'qr-code-snapchat', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-whatsapp'})
MATCH (el:EntityL10n {entity_key: 'qr-code-whatsapp', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-telegram'})
MATCH (el:EntityL10n {entity_key: 'qr-code-telegram', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-pinterest'})
MATCH (el:EntityL10n {entity_key: 'qr-code-pinterest', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-spotify'})
MATCH (el:EntityL10n {entity_key: 'qr-code-spotify', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-apple-music'})
MATCH (el:EntityL10n {entity_key: 'qr-code-apple-music', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-soundcloud'})
MATCH (el:EntityL10n {entity_key: 'qr-code-soundcloud', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-payment'})
MATCH (el:EntityL10n {entity_key: 'qr-code-payment', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-pix'})
MATCH (el:EntityL10n {entity_key: 'qr-code-pix', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-upi'})
MATCH (el:EntityL10n {entity_key: 'qr-code-upi', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-paypal'})
MATCH (el:EntityL10n {entity_key: 'qr-code-paypal', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-venmo'})
MATCH (el:EntityL10n {entity_key: 'qr-code-venmo', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-bitcoin'})
MATCH (el:EntityL10n {entity_key: 'qr-code-bitcoin', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-ethereum'})
MATCH (el:EntityL10n {entity_key: 'qr-code-ethereum', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-crypto'})
MATCH (el:EntityL10n {entity_key: 'qr-code-crypto', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-bank-transfer'})
MATCH (el:EntityL10n {entity_key: 'qr-code-bank-transfer', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-location'})
MATCH (el:EntityL10n {entity_key: 'qr-code-location', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-google-maps'})
MATCH (el:EntityL10n {entity_key: 'qr-code-google-maps', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-apple-maps'})
MATCH (el:EntityL10n {entity_key: 'qr-code-apple-maps', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-waze'})
MATCH (el:EntityL10n {entity_key: 'qr-code-waze', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-coordinates'})
MATCH (el:EntityL10n {entity_key: 'qr-code-coordinates', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-app'})
MATCH (el:EntityL10n {entity_key: 'qr-code-app', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-app-store'})
MATCH (el:EntityL10n {entity_key: 'qr-code-app-store', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-play-store'})
MATCH (el:EntityL10n {entity_key: 'qr-code-play-store', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-app-download'})
MATCH (el:EntityL10n {entity_key: 'qr-code-app-download', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-review'})
MATCH (el:EntityL10n {entity_key: 'qr-code-review', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-survey'})
MATCH (el:EntityL10n {entity_key: 'qr-code-survey', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-feedback'})
MATCH (el:EntityL10n {entity_key: 'qr-code-feedback', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-menu'})
MATCH (el:EntityL10n {entity_key: 'qr-code-menu', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-resume'})
MATCH (el:EntityL10n {entity_key: 'qr-code-resume', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-certificate'})
MATCH (el:EntityL10n {entity_key: 'qr-code-certificate', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-ticket'})
MATCH (el:EntityL10n {entity_key: 'qr-code-ticket', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-attendance'})
MATCH (el:EntityL10n {entity_key: 'qr-code-attendance', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-pet-tag'})
MATCH (el:EntityL10n {entity_key: 'qr-code-pet-tag', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-medical-id'})
MATCH (el:EntityL10n {entity_key: 'qr-code-medical-id', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-file'})
MATCH (el:EntityL10n {entity_key: 'qr-code-file', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-calendar'})
MATCH (el:EntityL10n {entity_key: 'qr-code-calendar', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'ean-13'})
MATCH (el:EntityL10n {entity_key: 'ean-13', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'ean-8'})
MATCH (el:EntityL10n {entity_key: 'ean-8', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'upc-a'})
MATCH (el:EntityL10n {entity_key: 'upc-a', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'upc-e'})
MATCH (el:EntityL10n {entity_key: 'upc-e', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'code-128'})
MATCH (el:EntityL10n {entity_key: 'code-128', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'code-39'})
MATCH (el:EntityL10n {entity_key: 'code-39', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'itf-14'})
MATCH (el:EntityL10n {entity_key: 'itf-14', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'codabar'})
MATCH (el:EntityL10n {entity_key: 'codabar', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'msi-plessey'})
MATCH (el:EntityL10n {entity_key: 'msi-plessey', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'gs1-128'})
MATCH (el:EntityL10n {entity_key: 'gs1-128', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'data-matrix'})
MATCH (el:EntityL10n {entity_key: 'data-matrix', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'pdf417'})
MATCH (el:EntityL10n {entity_key: 'pdf417', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'aztec-code'})
MATCH (el:EntityL10n {entity_key: 'aztec-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'maxicode'})
MATCH (el:EntityL10n {entity_key: 'maxicode', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'gs1-datamatrix'})
MATCH (el:EntityL10n {entity_key: 'gs1-datamatrix', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'barcode-generator'})
MATCH (el:EntityL10n {entity_key: 'barcode-generator', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'barcode-scanner'})
MATCH (el:EntityL10n {entity_key: 'barcode-scanner', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'analytics'})
MATCH (el:EntityL10n {entity_key: 'analytics', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'click-tracking'})
MATCH (el:EntityL10n {entity_key: 'click-tracking', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'scan-counting'})
MATCH (el:EntityL10n {entity_key: 'scan-counting', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'geo-tracking'})
MATCH (el:EntityL10n {entity_key: 'geo-tracking', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'device-detection'})
MATCH (el:EntityL10n {entity_key: 'device-detection', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'time-series'})
MATCH (el:EntityL10n {entity_key: 'time-series', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'contextual-routing'})
MATCH (el:EntityL10n {entity_key: 'contextual-routing', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'custom-domain-name'})
MATCH (el:EntityL10n {entity_key: 'custom-domain-name', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'custom-link-preview'})
MATCH (el:EntityL10n {entity_key: 'custom-link-preview', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'utm-builder'})
MATCH (el:EntityL10n {entity_key: 'utm-builder', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'bulk-creation'})
MATCH (el:EntityL10n {entity_key: 'bulk-creation', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'team-workspaces'})
MATCH (el:EntityL10n {entity_key: 'team-workspaces', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'api'})
MATCH (el:EntityL10n {entity_key: 'api', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'webhooks'})
MATCH (el:EntityL10n {entity_key: 'webhooks', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'white-label'})
MATCH (el:EntityL10n {entity_key: 'white-label', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'password-protection'})
MATCH (el:EntityL10n {entity_key: 'password-protection', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'expiration'})
MATCH (el:EntityL10n {entity_key: 'expiration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'scan-limit'})
MATCH (el:EntityL10n {entity_key: 'scan-limit', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'retargeting-pixel'})
MATCH (el:EntityL10n {entity_key: 'retargeting-pixel', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-generator'})
MATCH (el:EntityL10n {entity_key: 'qr-code-generator', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-scanner'})
MATCH (el:EntityL10n {entity_key: 'qr-code-scanner', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-api'})
MATCH (el:EntityL10n {entity_key: 'qr-code-api', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'landing-page-builder'})
MATCH (el:EntityL10n {entity_key: 'landing-page-builder', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'url-shortener'})
MATCH (el:EntityL10n {entity_key: 'url-shortener', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'link-in-bio-builder'})
MATCH (el:EntityL10n {entity_key: 'link-in-bio-builder', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'menu-builder'})
MATCH (el:EntityL10n {entity_key: 'menu-builder', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'vcard-generator'})
MATCH (el:EntityL10n {entity_key: 'vcard-generator', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'wifi-qr-generator'})
MATCH (el:EntityL10n {entity_key: 'wifi-qr-generator', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'batch-qr-generator'})
MATCH (el:EntityL10n {entity_key: 'batch-qr-generator', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'business-cards'})
MATCH (el:EntityL10n {entity_key: 'business-cards', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'flyers'})
MATCH (el:EntityL10n {entity_key: 'flyers', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'posters-billboards'})
MATCH (el:EntityL10n {entity_key: 'posters-billboards', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'brochures'})
MATCH (el:EntityL10n {entity_key: 'brochures', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'catalogs'})
MATCH (el:EntityL10n {entity_key: 'catalogs', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'magazines'})
MATCH (el:EntityL10n {entity_key: 'magazines', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'newspapers'})
MATCH (el:EntityL10n {entity_key: 'newspapers', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'direct-mail'})
MATCH (el:EntityL10n {entity_key: 'direct-mail', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'stickers-labels'})
MATCH (el:EntityL10n {entity_key: 'stickers-labels', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'banners'})
MATCH (el:EntityL10n {entity_key: 'banners', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'product-packaging'})
MATCH (el:EntityL10n {entity_key: 'product-packaging', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'product-labels'})
MATCH (el:EntityL10n {entity_key: 'product-labels', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'receipts'})
MATCH (el:EntityL10n {entity_key: 'receipts', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'table-tents'})
MATCH (el:EntityL10n {entity_key: 'table-tents', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'menus-printed'})
MATCH (el:EntityL10n {entity_key: 'menus-printed', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'tickets-physical'})
MATCH (el:EntityL10n {entity_key: 'tickets-physical', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'emails'})
MATCH (el:EntityL10n {entity_key: 'emails', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'presentations'})
MATCH (el:EntityL10n {entity_key: 'presentations', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'documents'})
MATCH (el:EntityL10n {entity_key: 'documents', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'websites'})
MATCH (el:EntityL10n {entity_key: 'websites', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'create-qr-code'})
MATCH (el:EntityL10n {entity_key: 'create-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'scan-qr-code'})
MATCH (el:EntityL10n {entity_key: 'scan-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'customize-qr-code'})
MATCH (el:EntityL10n {entity_key: 'customize-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'download-qr-code'})
MATCH (el:EntityL10n {entity_key: 'download-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'print-qr-code'})
MATCH (el:EntityL10n {entity_key: 'print-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'add-logo'})
MATCH (el:EntityL10n {entity_key: 'add-logo', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'change-colors'})
MATCH (el:EntityL10n {entity_key: 'change-colors', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'edit-destination'})
MATCH (el:EntityL10n {entity_key: 'edit-destination', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'share-qr-code'})
MATCH (el:EntityL10n {entity_key: 'share-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'track-scans'})
MATCH (el:EntityL10n {entity_key: 'track-scans', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'shorten-url'})
MATCH (el:EntityL10n {entity_key: 'shorten-url', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'create-smart-link'})
MATCH (el:EntityL10n {entity_key: 'create-smart-link', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'create-landing-page'})
MATCH (el:EntityL10n {entity_key: 'create-landing-page', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'create-barcode'})
MATCH (el:EntityL10n {entity_key: 'create-barcode', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'scan-barcode'})
MATCH (el:EntityL10n {entity_key: 'scan-barcode', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'restaurants'})
MATCH (el:EntityL10n {entity_key: 'restaurants', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'retail'})
MATCH (el:EntityL10n {entity_key: 'retail', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'hospitality'})
MATCH (el:EntityL10n {entity_key: 'hospitality', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'healthcare'})
MATCH (el:EntityL10n {entity_key: 'healthcare', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'education'})
MATCH (el:EntityL10n {entity_key: 'education', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'real-estate'})
MATCH (el:EntityL10n {entity_key: 'real-estate', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'fitness'})
MATCH (el:EntityL10n {entity_key: 'fitness', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'beauty'})
MATCH (el:EntityL10n {entity_key: 'beauty', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'entertainment'})
MATCH (el:EntityL10n {entity_key: 'entertainment', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'transportation'})
MATCH (el:EntityL10n {entity_key: 'transportation', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'manufacturing'})
MATCH (el:EntityL10n {entity_key: 'manufacturing', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'logistics'})
MATCH (el:EntityL10n {entity_key: 'logistics', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'construction'})
MATCH (el:EntityL10n {entity_key: 'construction', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'finance'})
MATCH (el:EntityL10n {entity_key: 'finance', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'government'})
MATCH (el:EntityL10n {entity_key: 'government', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'marketing-agencies'})
MATCH (el:EntityL10n {entity_key: 'marketing-agencies', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'creative-agencies'})
MATCH (el:EntityL10n {entity_key: 'creative-agencies', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'event-management'})
MATCH (el:EntityL10n {entity_key: 'event-management', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'nonprofits'})
MATCH (el:EntityL10n {entity_key: 'nonprofits', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'consulting'})
MATCH (el:EntityL10n {entity_key: 'consulting', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'developers'})
MATCH (el:EntityL10n {entity_key: 'developers', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'enterprise'})
MATCH (el:EntityL10n {entity_key: 'enterprise', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'agencies'})
MATCH (el:EntityL10n {entity_key: 'agencies', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'small-business'})
MATCH (el:EntityL10n {entity_key: 'small-business', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'freelancers'})
MATCH (el:EntityL10n {entity_key: 'freelancers', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'instagram'})
MATCH (el:EntityL10n {entity_key: 'instagram', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'linkedin'})
MATCH (el:EntityL10n {entity_key: 'linkedin', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'facebook'})
MATCH (el:EntityL10n {entity_key: 'facebook', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'twitter'})
MATCH (el:EntityL10n {entity_key: 'twitter', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'youtube'})
MATCH (el:EntityL10n {entity_key: 'youtube', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'tiktok'})
MATCH (el:EntityL10n {entity_key: 'tiktok', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'snapchat'})
MATCH (el:EntityL10n {entity_key: 'snapchat', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'whatsapp'})
MATCH (el:EntityL10n {entity_key: 'whatsapp', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'telegram'})
MATCH (el:EntityL10n {entity_key: 'telegram', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'pinterest'})
MATCH (el:EntityL10n {entity_key: 'pinterest', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'spotify'})
MATCH (el:EntityL10n {entity_key: 'spotify', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'apple'})
MATCH (el:EntityL10n {entity_key: 'apple', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'soundcloud'})
MATCH (el:EntityL10n {entity_key: 'soundcloud', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'paypal'})
MATCH (el:EntityL10n {entity_key: 'paypal', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'venmo'})
MATCH (el:EntityL10n {entity_key: 'venmo', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'google'})
MATCH (el:EntityL10n {entity_key: 'google', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'waze'})
MATCH (el:EntityL10n {entity_key: 'waze', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'zapier'})
MATCH (el:EntityL10n {entity_key: 'zapier', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'make'})
MATCH (el:EntityL10n {entity_key: 'make', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'n8n'})
MATCH (el:EntityL10n {entity_key: 'n8n', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'hubspot'})
MATCH (el:EntityL10n {entity_key: 'hubspot', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'salesforce'})
MATCH (el:EntityL10n {entity_key: 'salesforce', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'mailchimp'})
MATCH (el:EntityL10n {entity_key: 'mailchimp', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'shopify'})
MATCH (el:EntityL10n {entity_key: 'shopify', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'woocommerce'})
MATCH (el:EntityL10n {entity_key: 'woocommerce', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'zapier-integration'})
MATCH (el:EntityL10n {entity_key: 'zapier-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'make-integration'})
MATCH (el:EntityL10n {entity_key: 'make-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'n8n-integration'})
MATCH (el:EntityL10n {entity_key: 'n8n-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'hubspot-integration'})
MATCH (el:EntityL10n {entity_key: 'hubspot-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'salesforce-integration'})
MATCH (el:EntityL10n {entity_key: 'salesforce-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'mailchimp-integration'})
MATCH (el:EntityL10n {entity_key: 'mailchimp-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'google-sheets-integration'})
MATCH (el:EntityL10n {entity_key: 'google-sheets-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'notion-integration'})
MATCH (el:EntityL10n {entity_key: 'notion-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'slack-integration'})
MATCH (el:EntityL10n {entity_key: 'slack-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'shopify-integration'})
MATCH (el:EntityL10n {entity_key: 'shopify-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'woocommerce-integration'})
MATCH (el:EntityL10n {entity_key: 'woocommerce-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'wordpress-integration'})
MATCH (el:EntityL10n {entity_key: 'wordpress-integration', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'quiet-zone'})
MATCH (el:EntityL10n {entity_key: 'quiet-zone', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'error-correction'})
MATCH (el:EntityL10n {entity_key: 'error-correction', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'data-capacity'})
MATCH (el:EntityL10n {entity_key: 'data-capacity', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-version'})
MATCH (el:EntityL10n {entity_key: 'qr-code-version', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'encoding-mode'})
MATCH (el:EntityL10n {entity_key: 'encoding-mode', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'module'})
MATCH (el:EntityL10n {entity_key: 'module', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'finder-pattern'})
MATCH (el:EntityL10n {entity_key: 'finder-pattern', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'timing-pattern'})
MATCH (el:EntityL10n {entity_key: 'timing-pattern', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-messaging'})
MATCH (el:EntityL10n {entity_key: 'qr-code-messaging', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-video-platform'})
MATCH (el:EntityL10n {entity_key: 'qr-code-video-platform', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-professional'})
MATCH (el:EntityL10n {entity_key: 'qr-code-professional', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-music-platform'})
MATCH (el:EntityL10n {entity_key: 'qr-code-music-platform', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'funny-qr-codes'})
MATCH (el:EntityL10n {entity_key: 'funny-qr-codes', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-tattoo'})
MATCH (el:EntityL10n {entity_key: 'qr-code-tattoo', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-art-installation'})
MATCH (el:EntityL10n {entity_key: 'qr-code-art-installation', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-scavenger-hunt'})
MATCH (el:EntityL10n {entity_key: 'qr-code-scavenger-hunt', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-reviews'})
MATCH (el:EntityL10n {entity_key: 'qr-code-reviews', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-contactless-payment'})
MATCH (el:EntityL10n {entity_key: 'qr-code-contactless-payment', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-loyalty-program'})
MATCH (el:EntityL10n {entity_key: 'qr-code-loyalty-program', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-product-authentication'})
MATCH (el:EntityL10n {entity_key: 'qr-code-product-authentication', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-event-checkin'})
MATCH (el:EntityL10n {entity_key: 'qr-code-event-checkin', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-networking'})
MATCH (el:EntityL10n {entity_key: 'qr-code-networking', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-wedding'})
MATCH (el:EntityL10n {entity_key: 'qr-code-wedding', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-museum-exhibit'})
MATCH (el:EntityL10n {entity_key: 'qr-code-museum-exhibit', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'how-to-create-qr-code'})
MATCH (el:EntityL10n {entity_key: 'how-to-create-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-design-guide'})
MATCH (el:EntityL10n {entity_key: 'qr-code-design-guide', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-print-guide'})
MATCH (el:EntityL10n {entity_key: 'qr-code-print-guide', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'dynamic-vs-static-guide'})
MATCH (el:EntityL10n {entity_key: 'dynamic-vs-static-guide', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-marketing-guide'})
MATCH (el:EntityL10n {entity_key: 'qr-code-marketing-guide', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-restaurant-guide'})
MATCH (el:EntityL10n {entity_key: 'qr-code-restaurant-guide', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-business-card-guide'})
MATCH (el:EntityL10n {entity_key: 'qr-code-business-card-guide', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-api-guide'})
MATCH (el:EntityL10n {entity_key: 'qr-code-api-guide', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-analytics-guide'})
MATCH (el:EntityL10n {entity_key: 'qr-code-analytics-guide', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-security-guide'})
MATCH (el:EntityL10n {entity_key: 'qr-code-security-guide', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-vs-barcode'})
MATCH (el:EntityL10n {entity_key: 'qr-code-vs-barcode', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'dynamic-vs-static-qr-code'})
MATCH (el:EntityL10n {entity_key: 'dynamic-vs-static-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-vs-nfc'})
MATCH (el:EntityL10n {entity_key: 'qr-code-vs-nfc', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-vs-data-matrix'})
MATCH (el:EntityL10n {entity_key: 'qr-code-vs-data-matrix', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'free-vs-paid-qr-generator'})
MATCH (el:EntityL10n {entity_key: 'free-vs-paid-qr-generator', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'qr-code-ai-vs-competitors'})
MATCH (el:EntityL10n {entity_key: 'qr-code-ai-vs-competitors', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'short-link-vs-qr-code'})
MATCH (el:EntityL10n {entity_key: 'short-link-vs-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: 'spotify-code-vs-qr-code'})
MATCH (el:EntityL10n {entity_key: 'spotify-code-vs-qr-code', locale_key: 'fr-FR'})
MERGE (e)-[:HAS_L10N]->(el);

// -------------------------------------------------------------------
// Create relations to Locale (FOR_LOCALE)
// -------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-FR'})
MATCH (el:EntityL10n {locale_key: 'fr-FR'})
MERGE (el)-[:FOR_LOCALE]->(l);
