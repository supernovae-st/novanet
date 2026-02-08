// ═══════════════════════════════════════════════════════════════════════════════
// EntityL10n fr-FR — 281 nodes
// Generated: 2026-02-08T23:12:45.933022
// ═══════════════════════════════════════════════════════════════════════════════

// Create constraint for uniqueness
CREATE CONSTRAINT entityl10n_unique IF NOT EXISTS
FOR (el:EntityL10n)
REQUIRE (el.entity_key, el.locale_key) IS UNIQUE;

// ───────────────────────────────────────────────────────────────────────────────
// EntityL10n Nodes
// ───────────────────────────────────────────────────────────────────────────────

MERGE (el:EntityL10n {entity_key: "qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code",
  el.display_name = "QR Code",
  el.description = "2D matrix barcode for encoding data",
  el.llm_context = "UTILISER: when discussing QR codes, scanning, 2D barcodes, quick response codes. DECLENCHEURS: qr, qr code, qr-code, scan code, 2d barcode, matrix code. PAS: barcode 1D (use Barcode), data matrix (use Data Matrix), link shortener without QR (use Smart Link).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "smart-link", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "lien-intelligent",
  el.display_name = "Lien Intelligent",
  el.description = "Intelligent shortened URL with routing rules",
  el.llm_context = "UTILISER: when discussing intelligent URLs, link routing, device targeting, geo-targeting links. DECLENCHEURS: smart link, intelligent url, routing link, conditional redirect, targeted link. PAS: basic short URL (use Short Link), QR code (use QR Code), landing page (use Landing Page).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "barcode", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "barcode",
  el.display_name = "Code-barres",
  el.description = "1D linear barcode (EAN, UPC, Code 128)",
  el.llm_context = "UTILISER: when discussing 1D barcodes, linear barcodes, retail barcodes, product codes. DECLENCHEURS: barcode, 1d barcode, ean, upc, code 128, linear barcode, product code. PAS: QR code (use QR Code), 2D codes (use QR Code or Data Matrix).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "landing-page", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "page-atterrissage",
  el.display_name = "Page d\'Atterrissage",
  el.description = "Destination page created via page builder",
  el.llm_context = "UTILISER: when discussing destination pages, page builder, no-code pages, mobile pages. DECLENCHEURS: landing page, destination page, page builder, mobile page, microsite. PAS: full website (external), QR code itself (use QR Code), link shortener (use Smart Link).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "short-link", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "lien-court",
  el.display_name = "Lien Court",
  el.description = "Shortened URL, shared technology layer",
  el.llm_context = "UTILISER: when discussing URL shortening, link tracking, shortened URLs as technology. DECLENCHEURS: short link, shortened url, url shortener, link shortening, tiny url. PAS: smart routing (use Smart Link), QR code (use QR Code), vanity URL only (mention custom domain).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-style", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-style",
  el.display_name = "QR Code Style",
  el.description = "Visual style category for QR codes",
  el.llm_context = "UTILISER: when discussing QR code visual approaches, style categories, design types. DECLENCHEURS: qr style, qr code style, visual style, design approach. PAS: specific styles (use Custom QR, QR Art, etc.), colors only (use QR Code Colors).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-content", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-content",
  el.display_name = "QR Code Content",
  el.description = "Content type category for QR codes",
  el.llm_context = "UTILISER: when discussing QR code data types, what QR codes encode, content categories. DECLENCHEURS: qr content, content type, qr data, what to encode. PAS: specific content types (use URL QR, WiFi QR, etc.), QR appearance (use QR Code Style).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-frame", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-frame",
  el.display_name = "QR Code Frame",
  el.description = "Physical placement category for QR codes",
  el.llm_context = "UTILISER: when discussing QR code templates, physical placement, print templates. DECLENCHEURS: qr frame, template, placement, print size, frame template. PAS: specific frames (use Business Card QR, Poster QR, etc.), digital only (use Landing Page).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "barcode-format", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "barcode-format",
  el.display_name = "Code-barres Format",
  el.description = "Technical format category for barcodes",
  el.llm_context = "UTILISER: when discussing barcode standards, encoding formats, barcode types. DECLENCHEURS: barcode format, barcode type, barcode standard, encoding format. PAS: specific formats (use EAN-13, UPC-A, etc.), QR codes (use QR Code).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "landing-page-type", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "page-atterrissage-type",
  el.display_name = "Page d\'Atterrissage Type",
  el.description = "Template category for landing pages",
  el.llm_context = "UTILISER: when discussing landing page templates, page types, use case templates. DECLENCHEURS: landing page type, page template, page category, template type. PAS: specific types (use Link in Bio, Digital Menu, etc.), external websites.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "custom-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "personnalise-qr-code",
  el.display_name = "Personnalisé QR Code",
  el.description = "Parametric QR code with user-configured elements",
  el.llm_context = "UTILISER: when discussing fully customizable QR codes, parametric design, manual customization. DECLENCHEURS: custom qr, customize qr, design qr, parametric qr, branded qr. PAS: AI-generated (use QR Code Art), photo overlay (use QR Code with Image).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-image", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-image",
  el.display_name = "QR Code avec Image",
  el.description = "QR code with photo/image background overlay",
  el.llm_context = "UTILISER: when discussing QR codes with photos, image overlays, background images on QR. DECLENCHEURS: qr with image, qr photo, image qr, photo background qr. PAS: AI art (use QR Code Art), logo only (use QR Code with Logo).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-art", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-art",
  el.display_name = "QR Code Art",
  el.description = "AI-generated artistic QR code",
  el.llm_context = "UTILISER: when discussing AI QR codes, artistic QR generation, creative AI QR. DECLENCHEURS: qr art, ai qr, artistic qr, ai generated qr, creative qr. PAS: manual design (use Custom QR Code), photo overlay (use QR Code with Image).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-photo", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-photo",
  el.display_name = "QR Code Photo",
  el.description = "Alias for QR Code with Image",
  el.llm_context = "UTILISER: when user says \'QR photo\' specifically, redirect to QR Code with Image. DECLENCHEURS: qr photo, photo qr code. PAS: primary term (use QR Code with Image instead).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-with-logo", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-with-logo",
  el.display_name = "QR Code avec Logo",
  el.description = "QR code featuring a centered logo",
  el.llm_context = "UTILISER: when discussing adding logos to QR codes, branded QR with logo, center logo. DECLENCHEURS: qr with logo, add logo, logo qr, branded qr, center logo. PAS: full custom design (use Custom QR Code), background image (use QR Code with Image).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-with-text", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-with-text",
  el.display_name = "QR Code avec Texte",
  el.description = "QR code with call-to-action text",
  el.llm_context = "UTILISER: when discussing adding text to QR codes, CTA text, scan me text. DECLENCHEURS: qr with text, add text, scan me, call to action, text qr. PAS: encoded text content (use Text QR), logo (use QR Code with Logo).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-color", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-color",
  el.display_name = "QR Code Couleurs",
  el.description = "Custom foreground and background colors",
  el.llm_context = "UTILISER: when discussing QR code colors, color customization, foreground/background. DECLENCHEURS: qr color, colored qr, change color, qr colors, color scheme. PAS: shapes (use QR Code Shapes), gradients specifically (use Background Gradient).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-shapes", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-shapes",
  el.display_name = "QR Code Formes",
  el.description = "Custom module and eye shapes",
  el.llm_context = "UTILISER: when discussing QR module shapes, dot patterns, eye patterns, shape customization. DECLENCHEURS: qr shapes, module shape, dot pattern, eye pattern, rounded qr. PAS: colors (use QR Code Colors), logo (use QR Code with Logo).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-transparent-background", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-transparent-background",
  el.display_name = "Transparent Arrière-plan",
  el.description = "QR code with transparent background for overlays",
  el.llm_context = "UTILISER: when discussing transparent QR codes, PNG with alpha, overlay QR. DECLENCHEURS: transparent qr, transparent background, png alpha, overlay qr, no background. PAS: white background (default), image background (use Background Image).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-background", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-background",
  el.display_name = "QR Code Arrière-plan",
  el.description = "Background customization category",
  el.llm_context = "UTILISER: when discussing QR background options generally, background customization category. DECLENCHEURS: qr background, background options, background type. PAS: specific types (use Background Color, Gradient, or Image).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-background-color", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-background-color",
  el.display_name = "Arrière-plan Couleur",
  el.description = "Solid color background",
  el.llm_context = "UTILISER: when discussing solid color backgrounds for QR codes. DECLENCHEURS: background color, solid background, fill color. PAS: gradient (use Background Gradient), image (use Background Image).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-background-gradient", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-background-gradient",
  el.display_name = "Arrière-plan Gradient",
  el.description = "Gradient color background",
  el.llm_context = "UTILISER: when discussing gradient backgrounds for QR codes. DECLENCHEURS: gradient background, color gradient, gradient qr. PAS: solid color (use Background Color), image (use Background Image).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-background-image", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-background-image",
  el.display_name = "Arrière-plan Image",
  el.description = "Image or pattern background",
  el.llm_context = "UTILISER: when discussing image backgrounds for QR codes, patterns, textures. DECLENCHEURS: background image, image background, pattern background, texture. PAS: QR Code with Image style (different feature), solid color (use Background Color).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "dynamic-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "dynamique-qr-code",
  el.display_name = "Dynamique QR Code",
  el.description = "Editable QR code that uses a short link",
  el.llm_context = "UTILISER: when discussing editable QR codes, trackable QR, changeable destination. DECLENCHEURS: dynamic qr, editable qr, trackable qr, change destination, update qr. PAS: fixed content (use Static QR Code), smart routing (use Smart Link).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "static-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "statique-qr-code",
  el.display_name = "Statique QR Code",
  el.description = "Fixed QR code with data encoded directly",
  el.llm_context = "UTILISER: when discussing fixed QR codes, permanent QR, direct encoding. DECLENCHEURS: static qr, fixed qr, permanent qr, free qr, direct encode. PAS: editable (use Dynamic QR Code), trackable (use Dynamic QR Code).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-light-mode", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-light-mode",
  el.display_name = "Light Mode",
  el.description = "QR code optimized for light backgrounds",
  el.llm_context = "UTILISER: when discussing light theme QR codes, standard contrast QR. DECLENCHEURS: light mode, light theme, light background, standard qr. PAS: dark theme (use Dark Mode), inverted colors.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-dark-mode", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-dark-mode",
  el.display_name = "Dark Mode",
  el.description = "QR code optimized for dark backgrounds",
  el.llm_context = "UTILISER: when discussing dark theme QR codes, inverted QR, night mode. DECLENCHEURS: dark mode, dark theme, dark background, inverted qr, night mode. PAS: light theme (use Light Mode), standard appearance.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-business-card", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-carte-visite",
  el.display_name = "Carte de Visite QR",
  el.description = "QR code template for business cards",
  el.llm_context = "UTILISER: when discussing QR codes for business cards, professional networking QR. DECLENCHEURS: business card qr, card qr, professional qr, networking qr. PAS: email signature (use Email Signature QR), vCard content (use vCard QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-email-signature", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-email-signature",
  el.display_name = "Email Signature QR",
  el.description = "QR code template for email signatures",
  el.llm_context = "UTILISER: when discussing QR codes for email signatures, small signature QR. DECLENCHEURS: email signature qr, signature qr, email qr. PAS: business card (use Business Card QR), contact form (use Forms).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-flyer", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-flyer",
  el.display_name = "Flyer QR",
  el.description = "QR code template for flyers",
  el.llm_context = "UTILISER: when discussing QR codes for flyers, promotional print QR. DECLENCHEURS: flyer qr, handout qr, promotional qr, print qr. PAS: poster size (use Poster QR), product packaging (use Packaging Label QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-poster", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-poster",
  el.display_name = "Poster QR",
  el.description = "QR code template for posters and billboards",
  el.llm_context = "UTILISER: when discussing QR codes for posters, large format QR, billboard QR. DECLENCHEURS: poster qr, billboard qr, large qr, high resolution qr. PAS: flyer size (use Flyer QR), table display (use Table Tent QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-table-tent", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-table-tent",
  el.display_name = "Table Tent QR",
  el.description = "QR code template for table tents",
  el.llm_context = "UTILISER: when discussing QR codes for table displays, restaurant table QR. DECLENCHEURS: table tent qr, table qr, restaurant qr, menu qr. PAS: digital menu content (use Digital Menu), poster (use Poster QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-packaging-label", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-packaging-label",
  el.display_name = "Packaging Label QR",
  el.description = "QR code template for product packaging",
  el.llm_context = "UTILISER: when discussing QR codes for product packaging, label QR, product QR. DECLENCHEURS: packaging qr, label qr, product qr, package qr. PAS: retail barcode (use Barcode), poster (use Poster QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "link-in-bio", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "link-in-bio",
  el.display_name = "Link in Bio",
  el.description = "Social media bio link page",
  el.llm_context = "UTILISER: when discussing bio link pages, social media link aggregation, creator links. DECLENCHEURS: link in bio, bio link, linktree alternative, social links, creator page. PAS: full landing page (use Landing Page), single URL (use Short Link).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "menu-restaurant", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "menu-restaurant",
  el.display_name = "Digital Menu",
  el.description = "Restaurant digital menu page",
  el.llm_context = "UTILISER: when discussing restaurant menus, digital menus, QR menus, contactless menus. DECLENCHEURS: digital menu, restaurant menu, qr menu, contactless menu, menu page. PAS: table tent template (use Table Tent QR), generic landing page.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "forms", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "forms",
  el.display_name = "Forms",
  el.description = "Contact and lead capture forms",
  el.llm_context = "UTILISER: when discussing contact forms, lead capture, form pages, data collection. DECLENCHEURS: form, contact form, lead form, survey, registration form. PAS: event RSVP specifically (use Event RSVP), booking (use Booking/Appointment).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "announcement", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "announcement",
  el.display_name = "Announcement",
  el.description = "Message or announcement page",
  el.llm_context = "UTILISER: when discussing announcement pages, message pages, notification pages. DECLENCHEURS: announcement, message page, notification, alert page, info page. PAS: event invitation (use Event RSVP), ongoing content (use Link in Bio).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "event-rsvp", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "event-rsvp",
  el.display_name = "Event RSVP",
  el.description = "Event registration and RSVP page",
  el.llm_context = "UTILISER: when discussing event registration, RSVP pages, guest management. DECLENCHEURS: event rsvp, event registration, rsvp page, guest list, invitation. PAS: general forms (use Forms), booking slots (use Booking/Appointment).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "booking-appointment", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "booking-appointment",
  el.display_name = "Booking/Appointment",
  el.description = "Scheduling and booking page",
  el.llm_context = "UTILISER: when discussing appointment booking, scheduling pages, reservation systems. DECLENCHEURS: booking, appointment, schedule, reservation, calendar booking. PAS: event RSVP (use Event RSVP), contact form (use Forms).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-url", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-url",
  el.display_name = "URL QR Code",
  el.description = "QR code that redirects to any URL",
  el.llm_context = "UTILISER: when discussing QR codes linking to websites, URL encoding. DECLENCHEURS: url qr, website qr, link qr, web qr. PAS: specific platforms (use Instagram QR, YouTube QR, etc.), WiFi (use WiFi QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-wifi",
  el.display_name = "WiFi QR Code",
  el.description = "QR code with WiFi credentials (SSID, password, encryption)",
  el.llm_context = "UTILISER: when discussing WiFi sharing, network credentials in QR. DECLENCHEURS: wifi qr, wireless qr, network qr, wifi password qr. PAS: hotspot login page (use URL QR), Bluetooth.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-vcard", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-vcard",
  el.display_name = "vCard QR Code",
  el.description = "Digital business card in vCard format",
  el.llm_context = "UTILISER: when discussing digital business cards, contact sharing QR. DECLENCHEURS: vcard qr, contact qr, business card qr, digital card. PAS: MeCard (use MeCard QR), LinkedIn profile (use LinkedIn QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-mecard", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-mecard",
  el.display_name = "MeCard QR Code",
  el.description = "Japanese contact format (more compact than vCard)",
  el.llm_context = "UTILISER: when discussing Japanese contact format, compact contact QR. DECLENCHEURS: mecard, mecard qr, japanese contact qr. PAS: vCard (use vCard QR), standard contact.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-pdf",
  el.display_name = "PDF QR Code",
  el.description = "Links to PDF document for viewing or download",
  el.llm_context = "UTILISER: when discussing PDF links, document QR codes. DECLENCHEURS: pdf qr, document qr, brochure qr, manual qr. PAS: generic file (use File Download QR), image gallery (use Image Gallery QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-text", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-text",
  el.display_name = "Text QR Code",
  el.description = "Plain text content encoded directly",
  el.llm_context = "UTILISER: when discussing plain text encoding, static text in QR. DECLENCHEURS: text qr, plain text qr, message qr. PAS: URL (use URL QR), email (use Email QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-email", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-email",
  el.display_name = "Email QR Code",
  el.description = "Opens email composer with prefilled fields",
  el.llm_context = "UTILISER: when discussing email QR codes, mailto links in QR. DECLENCHEURS: email qr, mailto qr, compose email qr. PAS: SMS (use SMS QR), contact (use vCard QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-sms", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-sms",
  el.display_name = "SMS QR Code",
  el.description = "Opens SMS with prefilled phone number and message",
  el.llm_context = "UTILISER: when discussing SMS QR codes, text message QR. DECLENCHEURS: sms qr, text message qr, message qr. PAS: WhatsApp (use WhatsApp QR), phone call (use Phone QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-phone", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-phone",
  el.display_name = "Phone QR Code",
  el.description = "Phone number for direct calling",
  el.llm_context = "UTILISER: when discussing phone call QR codes, tel: links. DECLENCHEURS: phone qr, call qr, tel qr, phone number qr. PAS: SMS (use SMS QR), WhatsApp (use WhatsApp QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-video", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-video",
  el.display_name = "Video QR Code",
  el.description = "Links to video content (YouTube, Vimeo, hosted)",
  el.llm_context = "UTILISER: when discussing video link QR codes, video sharing. DECLENCHEURS: video qr, youtube qr link, vimeo qr, video link. PAS: YouTube channel (use YouTube QR), audio (use Audio QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-audio", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-audio",
  el.display_name = "Audio QR Code",
  el.description = "Links to audio or podcast content",
  el.llm_context = "UTILISER: when discussing audio link QR codes, podcast QR. DECLENCHEURS: audio qr, podcast qr, music link qr, voice message qr. PAS: Spotify profile (use Spotify QR), video (use Video QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-image-gallery", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-image-gallery",
  el.display_name = "Image Gallery QR Code",
  el.description = "Links to image gallery or photo album",
  el.llm_context = "UTILISER: when discussing photo album QR, image collection links. DECLENCHEURS: gallery qr, photo album qr, image gallery qr, photos qr. PAS: single image, PDF (use PDF QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-coupon", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-coupon",
  el.display_name = "Coupon QR Code",
  el.description = "Digital coupon or discount code",
  el.llm_context = "UTILISER: when discussing discount QR codes, promotional codes. DECLENCHEURS: coupon qr, discount qr, promo qr, deal qr. PAS: payment (use Payment QR), ticket (use Ticket QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-social", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-social",
  el.display_name = "Social Media QR Code",
  el.description = "Category for social media platform QR codes",
  el.llm_context = "UTILISER: when discussing social media QR codes generally. DECLENCHEURS: social qr, social media qr, social link qr. PAS: specific platforms (use Instagram QR, LinkedIn QR, etc.).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-instagram", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-instagram",
  el.display_name = "Instagram QR Code",
  el.description = "Links to Instagram profile or post",
  el.llm_context = "UTILISER: when discussing Instagram profile QR, Instagram links. DECLENCHEURS: instagram qr, ig qr, insta qr. PAS: other social (use Facebook QR, TikTok QR, etc.), generic social (use Social Media QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-linkedin", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-linkedin",
  el.display_name = "LinkedIn QR Code",
  el.description = "Links to LinkedIn profile or company page",
  el.llm_context = "UTILISER: when discussing LinkedIn profile QR, professional networking QR. DECLENCHEURS: linkedin qr, professional qr, company page qr. PAS: vCard (use vCard QR), business card frame (use Business Card QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-facebook", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-facebook",
  el.display_name = "Facebook QR Code",
  el.description = "Links to Facebook page or profile",
  el.llm_context = "UTILISER: when discussing Facebook page QR, Facebook profile links. DECLENCHEURS: facebook qr, fb qr, facebook page qr. PAS: Instagram (use Instagram QR), WhatsApp (use WhatsApp QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-twitter", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-twitter",
  el.display_name = "Twitter/X QR Code",
  el.description = "Links to Twitter/X profile or tweet",
  el.llm_context = "UTILISER: when discussing Twitter profile QR, X platform links. DECLENCHEURS: twitter qr, x qr, tweet qr. PAS: other social platforms.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-youtube", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-youtube",
  el.display_name = "YouTube QR Code",
  el.description = "Links to YouTube channel or video",
  el.llm_context = "UTILISER: when discussing YouTube channel QR, YouTube video links. DECLENCHEURS: youtube qr, channel qr, yt qr. PAS: generic video (use Video QR), TikTok (use TikTok QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-tiktok", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-tiktok",
  el.display_name = "TikTok QR Code",
  el.description = "Links to TikTok profile or video",
  el.llm_context = "UTILISER: when discussing TikTok profile QR, TikTok video links. DECLENCHEURS: tiktok qr, tt qr. PAS: Instagram Reels (use Instagram QR), YouTube Shorts (use YouTube QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-snapchat", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-snapchat",
  el.display_name = "Snapchat QR Code",
  el.description = "Links to Snapchat profile",
  el.llm_context = "UTILISER: when discussing Snapchat profile QR, Snapcode alternatives. DECLENCHEURS: snapchat qr, snap qr, snapcode. PAS: Instagram Stories (use Instagram QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-whatsapp", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-whatsapp",
  el.display_name = "WhatsApp QR Code",
  el.description = "Opens WhatsApp chat with prefilled message",
  el.llm_context = "UTILISER: when discussing WhatsApp chat QR, wa.me links. DECLENCHEURS: whatsapp qr, wa qr, whatsapp chat qr. PAS: SMS (use SMS QR), Telegram (use Telegram QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-telegram", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-telegram",
  el.display_name = "Telegram QR Code",
  el.description = "Links to Telegram channel, group, or bot",
  el.llm_context = "UTILISER: when discussing Telegram channel QR, Telegram links. DECLENCHEURS: telegram qr, tg qr, telegram channel qr, telegram bot qr. PAS: WhatsApp (use WhatsApp QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-pinterest", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-pinterest",
  el.display_name = "Pinterest QR Code",
  el.description = "Links to Pinterest profile or board",
  el.llm_context = "UTILISER: when discussing Pinterest profile QR, Pinterest board links. DECLENCHEURS: pinterest qr, pin qr, board qr. PAS: Instagram (use Instagram QR), image gallery (use Image Gallery QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-spotify", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-spotify",
  el.display_name = "Spotify QR Code",
  el.description = "Links to Spotify artist, album, or playlist",
  el.llm_context = "UTILISER: when discussing Spotify link QR, music sharing QR. DECLENCHEURS: spotify qr, playlist qr, music qr. PAS: Apple Music (use Apple Music QR), SoundCloud (use SoundCloud QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-apple-music", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-apple-music",
  el.display_name = "Apple Music QR Code",
  el.description = "Links to Apple Music artist, album, or playlist",
  el.llm_context = "UTILISER: when discussing Apple Music link QR. DECLENCHEURS: apple music qr, itunes qr. PAS: Spotify (use Spotify QR), generic audio (use Audio QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-soundcloud", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-soundcloud",
  el.display_name = "SoundCloud QR Code",
  el.description = "Links to SoundCloud artist or track",
  el.llm_context = "UTILISER: when discussing SoundCloud link QR, indie music QR. DECLENCHEURS: soundcloud qr, sc qr. PAS: Spotify (use Spotify QR), generic audio (use Audio QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-payment", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-payment",
  el.display_name = "Payment QR Code",
  el.description = "Category for payment-related QR codes",
  el.llm_context = "UTILISER: when discussing payment QR codes generally. DECLENCHEURS: payment qr, pay qr, money qr. PAS: specific systems (use PIX QR, UPI QR, PayPal QR, etc.).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-pix", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-pix",
  el.display_name = "PIX QR Code",
  el.description = "Brazil instant payment system",
  el.llm_context = "UTILISER: when discussing Brazilian payments, PIX system. DECLENCHEURS: pix qr, brazil payment qr, pix code. PAS: UPI India (use UPI QR), generic payment.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-upi", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-upi",
  el.display_name = "UPI QR Code",
  el.description = "India Unified Payments Interface",
  el.llm_context = "UTILISER: when discussing Indian payments, UPI system. DECLENCHEURS: upi qr, india payment qr, bharat qr. PAS: PIX Brazil (use PIX QR), generic payment.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-paypal", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-paypal",
  el.display_name = "PayPal QR Code",
  el.description = "PayPal payment or money request",
  el.llm_context = "UTILISER: when discussing PayPal payments, PayPal.me links. DECLENCHEURS: paypal qr, paypal.me qr. PAS: Venmo (use Venmo QR), bank transfer (use Bank Transfer QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-venmo", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-venmo",
  el.display_name = "Venmo QR Code",
  el.description = "Venmo payment (US only)",
  el.llm_context = "UTILISER: when discussing Venmo payments, US P2P payments. DECLENCHEURS: venmo qr, venmo code. PAS: PayPal (use PayPal QR), Cash App.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-bitcoin", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-bitcoin",
  el.display_name = "Bitcoin QR Code",
  el.description = "Bitcoin wallet address for payments",
  el.llm_context = "UTILISER: when discussing Bitcoin payments, BTC address QR. DECLENCHEURS: bitcoin qr, btc qr, crypto wallet qr. PAS: Ethereum (use Ethereum QR), generic crypto (use Crypto QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-ethereum", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-ethereum",
  el.display_name = "Ethereum QR Code",
  el.description = "Ethereum wallet address for payments",
  el.llm_context = "UTILISER: when discussing Ethereum payments, ETH address QR. DECLENCHEURS: ethereum qr, eth qr, erc20 qr. PAS: Bitcoin (use Bitcoin QR), generic crypto (use Crypto QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-crypto", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-crypto",
  el.display_name = "Crypto QR Code",
  el.description = "Generic cryptocurrency payment",
  el.llm_context = "UTILISER: when discussing generic crypto payments, multi-coin wallets. DECLENCHEURS: crypto qr, cryptocurrency qr, multi-coin qr. PAS: specific coins (use Bitcoin QR, Ethereum QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-bank-transfer", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-bank-transfer",
  el.display_name = "Bank Transfer QR Code",
  el.description = "SEPA or domestic bank transfer",
  el.llm_context = "UTILISER: when discussing bank transfer QR, IBAN encoding. DECLENCHEURS: bank transfer qr, sepa qr, iban qr, wire transfer qr. PAS: PIX (use PIX QR), UPI (use UPI QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-location", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-location",
  el.display_name = "Location QR Code",
  el.description = "Category for location and maps QR codes",
  el.llm_context = "UTILISER: when discussing location QR codes generally, maps category. DECLENCHEURS: location qr, maps qr, navigation qr. PAS: specific apps (use Google Maps QR, Apple Maps QR, Waze QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-google-maps", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-google-maps",
  el.display_name = "Google Maps QR Code",
  el.description = "Opens location in Google Maps",
  el.llm_context = "UTILISER: when discussing Google Maps links, Google location QR. DECLENCHEURS: google maps qr, gmaps qr, google location qr. PAS: Apple Maps (use Apple Maps QR), Waze (use Waze QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-apple-maps", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-apple-maps",
  el.display_name = "Apple Maps QR Code",
  el.description = "Opens location in Apple Maps",
  el.llm_context = "UTILISER: when discussing Apple Maps links, iOS maps QR. DECLENCHEURS: apple maps qr, ios maps qr. PAS: Google Maps (use Google Maps QR), Waze (use Waze QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-waze", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-waze",
  el.display_name = "Waze QR Code",
  el.description = "Opens navigation in Waze",
  el.llm_context = "UTILISER: when discussing Waze navigation links. DECLENCHEURS: waze qr, waze navigation qr. PAS: Google Maps (use Google Maps QR), Apple Maps (use Apple Maps QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-coordinates", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-coordinates",
  el.display_name = "Coordinates QR Code",
  el.description = "Raw GPS coordinates (geo: URI)",
  el.llm_context = "UTILISER: when discussing raw GPS encoding, geo: URI format. DECLENCHEURS: coordinates qr, gps qr, geo qr, lat long qr. PAS: specific map apps (use Google Maps QR, Apple Maps QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-app", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-app",
  el.display_name = "App Télécharger QR Code",
  el.description = "Category for app store download QR codes",
  el.llm_context = "UTILISER: when discussing app download QR codes generally. DECLENCHEURS: app qr, download app qr, app store qr. PAS: specific stores (use App Store QR, Play Store QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-app-store", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-app-store",
  el.display_name = "App Store QR Code",
  el.description = "iOS App Store download link",
  el.llm_context = "UTILISER: when discussing iOS app downloads, Apple App Store links. DECLENCHEURS: app store qr, ios app qr, apple app qr. PAS: Play Store (use Play Store QR), smart link (use Smart App Download QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-play-store", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-play-store",
  el.display_name = "Play Store QR Code",
  el.description = "Google Play Store download link",
  el.llm_context = "UTILISER: when discussing Android app downloads, Google Play links. DECLENCHEURS: play store qr, android app qr, google play qr. PAS: App Store (use App Store QR), smart link (use Smart App Download QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-app-download", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-app-download",
  el.display_name = "Smart App Télécharger QR Code",
  el.description = "Auto-detects OS and redirects to correct store",
  el.llm_context = "UTILISER: when discussing smart app links, cross-platform app download. DECLENCHEURS: smart app qr, universal app link, cross platform app qr. PAS: specific stores (use App Store QR, Play Store QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-review", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-review",
  el.display_name = "Review QR Code",
  el.description = "Links to Google, TripAdvisor, or other review platform",
  el.llm_context = "UTILISER: when discussing review collection QR, Google review links. DECLENCHEURS: review qr, google review qr, tripadvisor qr, yelp qr. PAS: feedback form (use Feedback QR), survey (use Survey QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-survey", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-survey",
  el.display_name = "Survey QR Code",
  el.description = "Links to online survey or form",
  el.llm_context = "UTILISER: when discussing survey QR codes, questionnaire links. DECLENCHEURS: survey qr, questionnaire qr, typeform qr, google form qr. PAS: simple feedback (use Feedback QR), review (use Review QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-feedback", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-feedback",
  el.display_name = "Feedback QR Code",
  el.description = "Links to feedback collection form",
  el.llm_context = "UTILISER: when discussing simple feedback collection, rating QR. DECLENCHEURS: feedback qr, rating qr, quick feedback qr. PAS: full survey (use Survey QR), review platform (use Review QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-menu", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-menu",
  el.display_name = "Menu QR Code",
  el.description = "Restaurant or cafe digital menu",
  el.llm_context = "UTILISER: when discussing restaurant menu QR, contactless menu. DECLENCHEURS: menu qr, restaurant qr, cafe qr, food menu qr. PAS: Digital Menu landing page (use Digital Menu), table tent frame (use Table Tent QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-resume", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-resume",
  el.display_name = "Resume QR Code",
  el.description = "Links to digital resume or CV",
  el.llm_context = "UTILISER: when discussing resume QR, CV links, portfolio QR. DECLENCHEURS: resume qr, cv qr, portfolio qr. PAS: LinkedIn profile (use LinkedIn QR), vCard (use vCard QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-certificate", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-certificate",
  el.display_name = "Certificate QR Code",
  el.description = "Verifies authenticity of certificates and credentials",
  el.llm_context = "UTILISER: when discussing certificate verification, credential QR. DECLENCHEURS: certificate qr, diploma qr, credential qr, verification qr. PAS: ticket (use Ticket QR), ID badge.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-ticket", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-ticket",
  el.display_name = "Ticket QR Code",
  el.description = "Event or transport ticket",
  el.llm_context = "UTILISER: when discussing event tickets, transport tickets, entry QR. DECLENCHEURS: ticket qr, event ticket qr, boarding pass qr, concert ticket qr. PAS: attendance check-in (use Attendance QR), coupon (use Coupon QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-attendance", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-attendance",
  el.display_name = "Attendance QR Code",
  el.description = "Check-in for events, classes, or meetings",
  el.llm_context = "UTILISER: when discussing check-in QR, attendance tracking. DECLENCHEURS: attendance qr, check-in qr, sign-in qr, class attendance qr. PAS: event ticket (use Ticket QR), access badge.",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-pet-tag", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-pet-tag",
  el.display_name = "Pet Tag QR Code",
  el.description = "Pet identification with owner contact info",
  el.llm_context = "UTILISER: when discussing pet ID QR, lost pet tags. DECLENCHEURS: pet tag qr, pet id qr, dog tag qr, cat tag qr. PAS: medical ID (use Medical ID QR), vCard (use vCard QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-medical-id", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-medical-id",
  el.display_name = "Medical ID QR Code",
  el.description = "Emergency medical information",
  el.llm_context = "UTILISER: when discussing medical alert QR, health info QR. DECLENCHEURS: medical id qr, health qr, emergency info qr, medical alert qr. PAS: pet tag (use Pet Tag QR), certificate (use Certificate QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-file", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-file",
  el.display_name = "File Télécharger QR Code",
  el.description = "Links to any downloadable file",
  el.llm_context = "UTILISER: when discussing file download QR, document download links. DECLENCHEURS: file qr, download qr, zip qr, document download qr. PAS: PDF specifically (use PDF QR), image gallery (use Image Gallery QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-calendar", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-calendar",
  el.display_name = "Calendar Event QR Code",
  el.description = "Adds event to calendar",
  el.llm_context = "UTILISER: when discussing calendar event QR, iCal links. DECLENCHEURS: calendar qr, event qr, ical qr, add to calendar qr. PAS: event RSVP page (use Event RSVP), ticket (use Ticket QR).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "ean-13", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "ean-13",
  el.display_name = "EAN-13",
  el.description = "European Article Number, 13 digits for retail products",
  el.llm_context = "UTILISER: when discussing EAN-13 barcodes, European retail product identification, 13-digit barcodes, or GTIN-13 standard. DECLENCHEURS: ean-13, ean13, european article number, 13-digit barcode, gtin-13, retail barcode europe. PAS: EAN-8 (compact version), UPC-A (North American), ISBN (books).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "ean-8", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "ean-8",
  el.display_name = "EAN-8",
  el.description = "Compact 8-digit barcode for small products",
  el.llm_context = "UTILISER: when discussing EAN-8 barcodes, compact retail barcodes, 8-digit product codes, or small product labeling. DECLENCHEURS: ean-8, ean8, 8-digit barcode, compact barcode, small product barcode. PAS: EAN-13 (full version), UPC-E (North American compact).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "upc-a", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "upc-a",
  el.display_name = "UPC-A",
  el.description = "Universal Product Code, 12 digits for US/Canada retail",
  el.llm_context = "UTILISER: when discussing UPC-A barcodes, North American retail product codes, 12-digit barcodes, or US/Canada product identification. DECLENCHEURS: upc-a, upca, universal product code, 12-digit barcode, us barcode, canada barcode, gtin-12. PAS: UPC-E (compressed), EAN-13 (European).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "upc-e", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "upc-e",
  el.display_name = "UPC-E",
  el.description = "Compressed 6-digit UPC for small packages",
  el.llm_context = "UTILISER: when discussing UPC-E barcodes, compressed product codes, 6-digit barcodes, or small package identification in North America. DECLENCHEURS: upc-e, upce, 6-digit barcode, compressed upc, zero-suppressed barcode. PAS: UPC-A (full version), EAN-8 (European compact).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "code-128", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "code-128",
  el.display_name = "Code 128",
  el.description = "High-density alphanumeric barcode for logistics",
  el.llm_context = "UTILISER: when discussing Code 128 barcodes, logistics barcodes, shipping labels, high-density alphanumeric encoding, or ASCII barcodes. DECLENCHEURS: code 128, code128, logistics barcode, shipping barcode, alphanumeric barcode, ascii barcode. PAS: Code 39 (simpler), GS1-128 (with application identifiers).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "code-39", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "code-39",
  el.display_name = "Code 39",
  el.description = "Full alphanumeric barcode for industrial use",
  el.llm_context = "UTILISER: when discussing Code 39 barcodes, industrial barcodes, automotive parts labeling, or self-checking alphanumeric codes. DECLENCHEURS: code 39, code39, code 3 of 9, industrial barcode, automotive barcode, defense barcode. PAS: Code 128 (higher density), Codabar (numeric).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "itf-14", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "itf-14",
  el.display_name = "ITF-14",
  el.description = "Interleaved 2 of 5 for shipping cartons",
  el.llm_context = "UTILISER: when discussing ITF-14 barcodes, shipping carton barcodes, pallet labeling, or GTIN-14 encoding. DECLENCHEURS: itf-14, itf14, interleaved 2 of 5, carton barcode, pallet barcode, gtin-14, case barcode. PAS: EAN-13 (retail), GS1-128 (with dates/lots).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "codabar", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "codabar",
  el.display_name = "Codabar",
  el.description = "Numeric barcode for libraries and blood banks",
  el.llm_context = "UTILISER: when discussing Codabar barcodes, library barcodes, blood bank barcodes, or legacy numeric codes. DECLENCHEURS: codabar, library barcode, blood bank barcode, fedex barcode, photo lab barcode. PAS: Code 39 (alphanumeric), Code 128 (modern logistics).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "msi-plessey", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "msi-plessey",
  el.display_name = "MSI Plessey",
  el.description = "Inventory control barcode with check digits",
  el.llm_context = "UTILISER: when discussing MSI Plessey barcodes, warehouse inventory barcodes, grocery store shelf labeling, or check-digit numeric codes. DECLENCHEURS: msi plessey, msi barcode, plessey barcode, inventory barcode, warehouse barcode, shelf barcode. PAS: Code 128 (modern), ITF-14 (shipping).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "gs1-128", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "gs1-128",
  el.display_name = "GS1-128",
  el.description = "Supply chain barcode with Application Identifiers",
  el.llm_context = "UTILISER: when discussing GS1-128 barcodes, supply chain traceability, batch/lot tracking, expiration dates on barcodes, or application identifiers. DECLENCHEURS: gs1-128, gs1128, ean-128, ucc-128, application identifier, batch barcode, lot barcode, expiry barcode. PAS: Code 128 (without AI), ITF-14 (simpler).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "data-matrix", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "data-matrix",
  el.display_name = "Data Matrix",
  el.description = "2D code for electronics and pharmaceutical marking",
  el.llm_context = "UTILISER: when discussing Data Matrix codes, electronics component marking, pharmaceutical serialization, or small 2D codes for industrial use. DECLENCHEURS: data matrix, datamatrix, ecc200, electronics marking, pharma barcode, component marking, small 2d code. PAS: QR code (consumer), GS1 DataMatrix (with identifiers).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "pdf417", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "pdf417",
  el.display_name = "PDF417",
  el.description = "Stacked 2D barcode for IDs and tickets",
  el.llm_context = "UTILISER: when discussing PDF417 codes, driver\'s license barcodes, boarding passes, ID cards, or stacked 2D barcodes. DECLENCHEURS: pdf417, pdf 417, driver license barcode, id barcode, boarding pass barcode, stacked barcode. PAS: QR code (square), Aztec (no quiet zone).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "aztec-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "aztec-code",
  el.display_name = "Aztec Code",
  el.description = "2D code for tickets and transport",
  el.llm_context = "UTILISER: when discussing Aztec codes, airline boarding passes, train tickets, transport tickets, or 2D codes without quiet zone. DECLENCHEURS: aztec code, aztec barcode, boarding pass code, train ticket barcode, transport barcode, no quiet zone barcode. PAS: QR code (needs quiet zone), PDF417 (rectangular).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "maxicode", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "maxicode",
  el.display_name = "MaxiCode",
  el.description = "Fixed-size 2D code for high-speed package sorting",
  el.llm_context = "UTILISER: when discussing MaxiCode, UPS package tracking, high-speed conveyor scanning, or hexagonal 2D codes. DECLENCHEURS: maxicode, ups barcode, package sorting code, hexagonal barcode, conveyor barcode. PAS: QR code (square), Data Matrix (small items).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "gs1-datamatrix", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "gs1-datamatrix",
  el.display_name = "GS1 DataMatrix",
  el.description = "Data Matrix with GS1 Application Identifiers",
  el.llm_context = "UTILISER: when discussing GS1 DataMatrix, pharmaceutical serialization, medical device UDI, food traceability, or regulated 2D codes. DECLENCHEURS: gs1 datamatrix, gs1 data matrix, pharma serialization, udi barcode, fmd barcode, medical device barcode, food traceability code. PAS: plain Data Matrix (no AI), QR code (consumer).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "barcode-generator", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "barcode-generator",
  el.display_name = "Code-barres Générateur",
  el.description = "Tool to create barcode images",
  el.llm_context = "UTILISER: when discussing barcode creation tools, generating EAN/UPC/Code 128 images, or barcode image software. DECLENCHEURS: barcode generator, create barcode, generate barcode, barcode maker, barcode image, ean generator, upc generator. PAS: QR code generator (2D square), barcode scanner (reading).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "barcode-scanner", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "barcode-scanner",
  el.display_name = "Code-barres Scannerner",
  el.description = "Tool to read barcode data",
  el.llm_context = "UTILISER: when discussing barcode reading tools, scanning EAN/UPC codes, barcode reader apps, or point-of-sale scanning. DECLENCHEURS: barcode scanner, barcode reader, scan barcode, pos scanner, barcode app, read barcode. PAS: QR code scanner (2D), barcode generator (creation).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "analytics", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "analytics",
  el.display_name = "Analytiques",
  el.description = "Umbrella feature for all tracking and statistics",
  el.llm_context = "UTILISER: when discussing QR code analytics, scan tracking, click statistics, or performance metrics. DECLENCHEURS: analytics, statistics, tracking, metrics, reports, data, insights, performance. PAS: specific analytics types (use click-tracking, geo-tracking, etc.).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "click-tracking", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "click-tracking",
  el.display_name = "Click Suivi",
  el.description = "Tracks link click events",
  el.llm_context = "UTILISER: when discussing link click tracking, click events, referrer data, or conversion tracking. DECLENCHEURS: click tracking, track clicks, click events, link clicks, click data, referrer tracking. PAS: scan counting (QR specific), analytics (umbrella term).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "scan-counting", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "scanner-counting",
  el.display_name = "Scanner Counting",
  el.description = "Counts QR code scans",
  el.llm_context = "UTILISER: when discussing QR code scan counts, scan statistics, or scan volume metrics. DECLENCHEURS: scan counting, count scans, scan stats, scan volume, scan numbers, how many scans. PAS: click tracking (links), geo-tracking (location).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "geo-tracking", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "geo-tracking",
  el.display_name = "Geographic Suivi",
  el.description = "Geographic location of scans",
  el.llm_context = "UTILISER: when discussing geographic tracking, location data, country/city analytics, or IP geolocation for scans. DECLENCHEURS: geo tracking, geographic, location tracking, country data, city data, ip location, where scanned. PAS: device detection (what device), time-series (when).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "device-detection", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "device-detection",
  el.display_name = "Device Detection",
  el.description = "OS/browser/device information",
  el.llm_context = "UTILISER: when discussing device detection, OS tracking, browser detection, or mobile vs desktop analytics. DECLENCHEURS: device detection, os detection, browser detection, device type, mobile or desktop, what device, user agent. PAS: geo-tracking (location), contextual routing (redirect).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "time-series", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "time-series",
  el.display_name = "Time Series Analytiques",
  el.description = "Historical data over time",
  el.llm_context = "UTILISER: when discussing time-series analytics, historical scan data, trend analysis, or scans over time. DECLENCHEURS: time series, historical data, trends, over time, date range, peak times, daily scans, weekly stats. PAS: real-time (immediate), analytics (umbrella).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "contextual-routing", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "contextual-routing",
  el.display_name = "Contextual Routing",
  el.description = "OS/device/location-based redirect",
  el.llm_context = "UTILISER: when discussing contextual routing, device-based redirects, OS-specific destinations, or smart link routing rules. DECLENCHEURS: contextual routing, smart routing, device redirect, os redirect, app store redirect, conditional redirect, dynamic destination. PAS: device detection (analytics only), edit destination (manual change).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "custom-domain-name", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "personnalise-domain-name",
  el.display_name = "Personnalisé Domain",
  el.description = "Branded short domains",
  el.llm_context = "UTILISER: when discussing custom domains, branded short links, white-label URLs, or vanity domains. DECLENCHEURS: custom domain, branded domain, own domain, vanity url, white label domain, custom short url. PAS: url shortener (action), white label (full branding).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "custom-link-preview", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "personnalise-link-preview",
  el.display_name = "Personnalisé Link Preview",
  el.description = "OG meta customization",
  el.llm_context = "UTILISER: when discussing custom link previews, Open Graph meta tags, social media previews, or thumbnail customization. DECLENCHEURS: link preview, og tags, open graph, social preview, thumbnail, share preview, meta tags. PAS: landing page (full page), custom domain (url).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "utm-builder", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "utm-builder",
  el.display_name = "UTM Builder",
  el.description = "Campaign parameter builder",
  el.llm_context = "UTILISER: when discussing UTM parameters, campaign tracking, Google Analytics parameters, or marketing attribution. DECLENCHEURS: utm builder, utm parameters, campaign tracking, utm source, utm medium, utm campaign, google analytics tracking. PAS: analytics (viewing data), retargeting (ads).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "bulk-creation", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "bulk-creation",
  el.display_name = "Bulk Creation",
  el.description = "Mass QR code generation",
  el.llm_context = "UTILISER: when discussing bulk QR code creation, mass generation, spreadsheet import, or enterprise-scale QR codes. DECLENCHEURS: bulk creation, bulk generate, mass create, batch qr, spreadsheet import, csv upload, multiple qr codes at once. PAS: batch-qr-generator (tool), api (programmatic).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "team-workspaces", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "team-workspaces",
  el.display_name = "Team Workspaces",
  el.description = "Multi-user collaboration",
  el.llm_context = "UTILISER: when discussing team collaboration, multi-user access, shared QR code management, or role-based permissions. DECLENCHEURS: team workspace, collaboration, multi-user, shared access, team members, roles, permissions, organization. PAS: white-label (branding), api (integration).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "api", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "api",
  el.display_name = "API Access",
  el.description = "Developer API access",
  el.llm_context = "UTILISER: when discussing API access, developer integration, programmatic QR code creation, or RESTful endpoints. DECLENCHEURS: api, api access, developer api, rest api, integration, programmatic, endpoints. PAS: webhooks (events), qr-code-api (specific tool).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "webhooks", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "webhooks",
  el.display_name = "Webhooks",
  el.description = "Event notifications",
  el.llm_context = "UTILISER: when discussing webhooks, event notifications, real-time callbacks, or scan event triggers. DECLENCHEURS: webhooks, webhook, event notification, callback, trigger, real-time event, scan webhook. PAS: api (request/response), analytics (viewing).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "white-label", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "white-label",
  el.display_name = "White Label",
  el.description = "Remove platform branding",
  el.llm_context = "UTILISER: when discussing white-label solutions, removing platform branding, reseller programs, or agency branding. DECLENCHEURS: white label, whitelabel, remove branding, no branding, agency solution, reseller, own branding. PAS: custom domain (url only), team workspaces (collaboration).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "password-protection", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "password-protection",
  el.display_name = "Password Protection",
  el.description = "Require password to access",
  el.llm_context = "UTILISER: when discussing password-protected QR codes, gated content, secure access, or password-required links. DECLENCHEURS: password protection, password protected, require password, gated content, secure qr, locked qr, access code. PAS: expiration (time limit), scan limit (count limit).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "expiration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "expiration",
  el.display_name = "Link Expiration",
  el.description = "Time-limited QR codes",
  el.llm_context = "UTILISER: when discussing link expiration, time-limited QR codes, expiry dates, or temporary access. DECLENCHEURS: expiration, expire, time limit, temporary qr, expiry date, auto-disable, limited time. PAS: scan limit (count), password protection (access control).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "scan-limit", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "scanner-limit",
  el.display_name = "Scanner Limit",
  el.description = "Maximum scan count",
  el.llm_context = "UTILISER: when discussing scan limits, maximum scans, limited-use QR codes, or scan quotas. DECLENCHEURS: scan limit, max scans, limited scans, scan quota, one-time scan, single use, limited use. PAS: expiration (time), password protection (access).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "retargeting-pixel", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "retargeting-pixel",
  el.display_name = "Retargeting Pixel",
  el.description = "Facebook/Google pixel integration",
  el.llm_context = "UTILISER: when discussing retargeting pixels, Facebook pixel, Google pixel, remarketing, or ad tracking integration. DECLENCHEURS: retargeting pixel, facebook pixel, google pixel, remarketing, ad tracking, pixel integration, audience building. PAS: utm-builder (attribution), analytics (internal).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-generator", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-generator",
  el.display_name = "QR Code Générateur",
  el.description = "Primary QR code creation tool",
  el.llm_context = "UTILISER: when discussing QR code creation tools, generating QR codes, or QR maker software. DECLENCHEURS: qr code generator, qr generator, qr maker, create qr, generate qr, make qr code, qr code creator. PAS: barcode generator (1D), qr code scanner (reading), qr code api (programmatic).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-scanner", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-scanner",
  el.display_name = "QR Code Scannerner",
  el.description = "QR code reader application",
  el.llm_context = "UTILISER: when discussing QR code scanning, reading QR codes, QR reader apps, or camera-based scanning. DECLENCHEURS: qr code scanner, qr scanner, qr reader, scan qr, read qr code, qr code app, camera scan. PAS: barcode scanner (1D), qr code generator (creation).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-api", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-api",
  el.display_name = "QR Code API",
  el.description = "API for QR generation",
  el.llm_context = "UTILISER: when discussing QR code APIs, programmatic QR generation, developer QR tools, or REST QR endpoints. DECLENCHEURS: qr code api, qr api, programmatic qr, developer qr, rest qr, api qr generation. PAS: api access (general feature), qr code generator (ui tool).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "landing-page-builder", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "page-atterrissage-builder",
  el.display_name = "Page d\'Atterrissage Builder",
  el.description = "No-code page builder",
  el.llm_context = "UTILISER: when discussing landing page builders, no-code page creation, drag-and-drop page editors, or destination page tools. DECLENCHEURS: landing page builder, page builder, no-code page, drag and drop, page editor, bio page builder. PAS: link-in-bio builder (social specific), menu builder (restaurant).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "url-shortener", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "url-shortener",
  el.display_name = "URL Shortener",
  el.description = "Link shortening tool",
  el.llm_context = "UTILISER: when discussing URL shorteners, link shortening, creating short links, or compact URLs. DECLENCHEURS: url shortener, link shortener, short url, short link, shorten url, compact link, tiny url. PAS: smart link (intelligent routing), custom domain (branded).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "link-in-bio-builder", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "link-in-bio-builder",
  el.display_name = "Link in Bio Builder",
  el.description = "Social bio page builder",
  el.llm_context = "UTILISER: when discussing link-in-bio pages, Instagram bio links, social media link hubs, or bio page tools. DECLENCHEURS: link in bio, bio link, instagram bio, tiktok bio, linktree alternative, bio page, social link hub. PAS: landing page builder (general), menu builder (restaurant).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "menu-builder", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "menu-builder",
  el.display_name = "Menu Builder",
  el.description = "Restaurant menu builder",
  el.llm_context = "UTILISER: when discussing digital menu builders, restaurant menu creation, or QR menu tools. DECLENCHEURS: menu builder, restaurant menu, digital menu, qr menu, menu creator, food menu builder, cafe menu. PAS: landing page builder (general), link-in-bio builder (social).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "vcard-generator", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "vcard-generator",
  el.display_name = "vCard Générateur",
  el.description = "Digital business card creator",
  el.llm_context = "UTILISER: when discussing vCard generators, digital business card creators, contact QR tools, or VCF file creation. DECLENCHEURS: vcard generator, vcard creator, digital business card, contact qr, vcf generator, electronic business card. PAS: business cards (medium), qr code generator (general).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "wifi-qr-generator", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "wifi-qr-generator",
  el.display_name = "WiFi QR Générateur",
  el.description = "WiFi credential QR creator",
  el.llm_context = "UTILISER: when discussing WiFi QR generators, WiFi password sharing via QR, or wireless network QR tools. DECLENCHEURS: wifi qr generator, wifi qr code, share wifi password, wifi qr, wireless qr, network qr code. PAS: qr code generator (general), qr code wifi (content type).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "batch-qr-generator", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "batch-qr-generator",
  el.display_name = "Batch QR Générateur",
  el.description = "Bulk QR creation tool",
  el.llm_context = "UTILISER: when discussing batch QR generation, bulk QR creation tools, spreadsheet QR generation, or mass QR production. DECLENCHEURS: batch qr generator, bulk qr tool, mass qr, spreadsheet qr, csv qr, multiple qr generator. PAS: bulk creation (feature), qr code generator (single).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "business-cards", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "carte-visites",
  el.display_name = "Carte de Visites",
  el.description = "Professional contact cards",
  el.llm_context = "UTILISER: when discussing QR codes on business cards, networking cards, or professional contact materials. DECLENCHEURS: business cards, business card qr, card qr code, networking card, contact card, visiting card. PAS: vcard (digital format), flyers (larger print).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "flyers", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "flyers",
  el.display_name = "Flyers",
  el.description = "Promotional flyers",
  el.llm_context = "UTILISER: when discussing QR codes on flyers, promotional handouts, or single-sheet marketing materials. DECLENCHEURS: flyers, flyer qr, handout, leaflet, promotional flyer, marketing flyer. PAS: brochures (folded), posters (large format).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "posters-billboards", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "posters-billboards",
  el.display_name = "Posters & Billboards",
  el.description = "Large format displays",
  el.llm_context = "UTILISER: when discussing QR codes on posters, billboards, outdoor advertising, or large-format displays. DECLENCHEURS: poster qr, billboard qr, outdoor qr, large qr, signage qr, advertising poster. PAS: banners (fabric), flyers (small print).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "brochures", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "brochures",
  el.display_name = "Brochures",
  el.description = "Folded informational materials",
  el.llm_context = "UTILISER: when discussing QR codes on brochures, tri-folds, pamphlets, or folded marketing materials. DECLENCHEURS: brochure qr, tri-fold, pamphlet, folded brochure, informational brochure. PAS: flyers (single sheet), catalogs (bound).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "catalogs", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "catalogs",
  el.display_name = "Catalogs",
  el.description = "Product catalogs",
  el.llm_context = "UTILISER: when discussing QR codes in product catalogs, print catalogs, or catalog shopping materials. DECLENCHEURS: catalog qr, product catalog, print catalog, shopping catalog, catalogue. PAS: brochures (folded), magazines (editorial).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "magazines", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "magazines",
  el.display_name = "Magazines",
  el.description = "Print publications",
  el.llm_context = "UTILISER: when discussing QR codes in magazines, print publications, or editorial content. DECLENCHEURS: magazine qr, print magazine, editorial qr, publication qr, magazine ad. PAS: newspapers (daily), brochures (marketing).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "newspapers", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "newspapers",
  el.display_name = "Newspapers",
  el.description = "News publications",
  el.llm_context = "UTILISER: when discussing QR codes in newspapers, print news, or daily/weekly publications. DECLENCHEURS: newspaper qr, print news, daily paper, news publication, newspaper ad. PAS: magazines (glossy), digital news (websites).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "direct-mail", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "direct-mail",
  el.display_name = "Direct Mail",
  el.description = "Mailed marketing materials",
  el.llm_context = "UTILISER: when discussing QR codes on direct mail, postcards, mailers, or shipped marketing materials. DECLENCHEURS: direct mail qr, postcard qr, mailer, mailed marketing, postal qr. PAS: flyers (handed out), email (digital).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "stickers-labels", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "stickers-labels",
  el.display_name = "Stickers & Labels",
  el.description = "Adhesive prints",
  el.llm_context = "UTILISER: when discussing QR codes on stickers, adhesive labels, or peel-and-stick materials. DECLENCHEURS: sticker qr, label qr, adhesive qr, peel and stick, vinyl sticker. PAS: product labels (packaging), product packaging (boxes).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "banners", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "banners",
  el.display_name = "Banners",
  el.description = "Event and trade show banners",
  el.llm_context = "UTILISER: when discussing QR codes on banners, trade show displays, event signage, or fabric/vinyl banners. DECLENCHEURS: banner qr, trade show banner, event banner, fabric banner, vinyl banner, roll-up banner. PAS: posters (paper), billboards (outdoor).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "product-packaging", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "product-packaging",
  el.display_name = "Product Packaging",
  el.description = "Product boxes and containers",
  el.llm_context = "UTILISER: when discussing QR codes on product packaging, boxes, containers, or retail packaging. DECLENCHEURS: packaging qr, product box, container qr, retail packaging, package qr code. PAS: product labels (adhesive), shipping labels (logistics).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "product-labels", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "product-labels",
  el.display_name = "Product Labels",
  el.description = "Applied product labels",
  el.llm_context = "UTILISER: when discussing QR codes on product labels, applied labels, or product tag labels. DECLENCHEURS: product label qr, applied label, product tag, label qr code, item label. PAS: stickers (decorative), product packaging (boxes).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "receipts", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "receipts",
  el.display_name = "Receipts",
  el.description = "Transaction receipts",
  el.llm_context = "UTILISER: when discussing QR codes on receipts, transaction slips, or purchase confirmations. DECLENCHEURS: receipt qr, transaction receipt, purchase receipt, pos receipt, sales receipt. PAS: tickets (entry), invoices (billing).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "table-tents", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "table-tents",
  el.display_name = "Table Tents",
  el.description = "Restaurant table displays",
  el.llm_context = "UTILISER: when discussing QR codes on table tents, restaurant table stands, or tabletop displays. DECLENCHEURS: table tent qr, table stand, restaurant table qr, tabletop display, table card. PAS: printed menus (booklet), flyers (handed out).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "menus-printed", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "menus-printed",
  el.display_name = "Imprimered Menus",
  el.description = "Physical restaurant menus",
  el.llm_context = "UTILISER: when discussing QR codes on printed menus, physical restaurant menus, or paper menus. DECLENCHEURS: printed menu qr, paper menu, physical menu, restaurant menu qr, laminated menu. PAS: digital menu (online), table tents (standing).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "tickets-physical", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "tickets-physical",
  el.display_name = "Physical Tickets",
  el.description = "Event tickets",
  el.llm_context = "UTILISER: when discussing QR codes on physical tickets, event tickets, admission tickets, or printed tickets. DECLENCHEURS: ticket qr, event ticket, physical ticket, admission ticket, printed ticket, concert ticket. PAS: digital tickets (mobile), receipts (purchase).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "emails", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "emails",
  el.display_name = "Emails",
  el.description = "Email signatures and campaigns",
  el.llm_context = "UTILISER: when discussing QR codes in emails, email signatures, email campaigns, or email marketing. DECLENCHEURS: email qr, email signature, email campaign, email marketing qr, newsletter qr. PAS: websites (web pages), documents (pdfs).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "presentations", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "presentations",
  el.display_name = "Presentations",
  el.description = "Slides and decks",
  el.llm_context = "UTILISER: when discussing QR codes in presentations, slide decks, PowerPoint, or meeting materials. DECLENCHEURS: presentation qr, slide qr, powerpoint qr, deck qr, meeting slides, conference presentation. PAS: documents (static), websites (interactive).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "documents", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "documents",
  el.display_name = "Documents",
  el.description = "PDFs, reports, and contracts",
  el.llm_context = "UTILISER: when discussing QR codes in documents, PDFs, reports, contracts, or printed documents. DECLENCHEURS: document qr, pdf qr, report qr, contract qr, printed document. PAS: presentations (slides), emails (messages).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "websites", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "websites",
  el.display_name = "Websites",
  el.description = "Web pages",
  el.llm_context = "UTILISER: when discussing QR codes displayed on websites, web pages, or online platforms. DECLENCHEURS: website qr, web page qr, online qr, site qr, desktop to mobile. PAS: emails (messages), presentations (slides).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "create-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "creer-qr-code",
  el.display_name = "Créer QR Code",
  el.description = "Generate a new QR code",
  el.llm_context = "UTILISER: when discussing QR code creation, making QR codes, or generating new QR codes. DECLENCHEURS: create qr code, make qr, generate qr, new qr code, create qr, make qr code. PAS: customize qr (design), scan qr (reading), edit destination (update).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "scan-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "scanner-qr-code",
  el.display_name = "Scanner QR Code",
  el.description = "Read QR code data",
  el.llm_context = "UTILISER: when discussing scanning QR codes, reading QR codes, or decoding QR content. DECLENCHEURS: scan qr code, read qr, decode qr, scan qr, qr scan, use camera to scan. PAS: create qr (making), track scans (analytics).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "customize-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "customize-qr-code",
  el.display_name = "Personnaliséize QR Code",
  el.description = "Design and style a QR code",
  el.llm_context = "UTILISER: when discussing QR code customization, styling QR codes, or designing QR appearance. DECLENCHEURS: customize qr, design qr, style qr, qr design, qr customization, personalize qr. PAS: create qr (initial), add logo (specific), change colors (specific).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "download-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "telecharger-qr-code",
  el.display_name = "Télécharger QR Code",
  el.description = "Export QR as image file",
  el.llm_context = "UTILISER: when discussing downloading QR codes, exporting QR images, or saving QR files. DECLENCHEURS: download qr, export qr, save qr, qr download, get qr image, qr file. PAS: print qr (physical), share qr (distribute).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "print-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "imprimer-qr-code",
  el.display_name = "Imprimer QR Code",
  el.description = "Print for physical use",
  el.llm_context = "UTILISER: when discussing printing QR codes, physical QR output, or QR print requirements. DECLENCHEURS: print qr, qr printing, physical qr, print qr code, qr for print, printable qr. PAS: download qr (digital), qr size (specification).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "add-logo", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "add-logo",
  el.display_name = "Add Logo",
  el.description = "Add logo to QR center",
  el.llm_context = "UTILISER: when discussing adding logos to QR codes, branded QR codes, or logo placement in QR. DECLENCHEURS: add logo, logo qr, branded qr, qr with logo, center logo, embed logo. PAS: customize qr (general), change colors (different aspect).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "change-colors", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "change-colors",
  el.display_name = "Change Couleurs",
  el.description = "Customize QR colors",
  el.llm_context = "UTILISER: when discussing QR code color customization, changing QR colors, or colored QR codes. DECLENCHEURS: change colors, qr colors, colored qr, custom color qr, qr color scheme. PAS: add logo (different aspect), customize qr (general).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "edit-destination", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "edit-destination",
  el.display_name = "Edit Destination",
  el.description = "Change QR target URL",
  el.llm_context = "UTILISER: when discussing editing QR destinations, changing where QR points, or updating QR URLs. DECLENCHEURS: edit destination, change url, update link, modify destination, redirect qr, change qr target. PAS: create qr (initial), dynamic qr (code type).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "share-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "partager-qr-code",
  el.display_name = "Partager QR Code",
  el.description = "Share via link or email",
  el.llm_context = "UTILISER: when discussing sharing QR codes, distributing QR images, or sending QR codes to others. DECLENCHEURS: share qr, send qr, distribute qr, qr sharing, email qr, share qr code. PAS: download qr (save), print qr (physical).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "track-scans", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "track-scans",
  el.display_name = "Track Scanners",
  el.description = "Monitor scan analytics",
  el.llm_context = "UTILISER: when discussing scan tracking, monitoring QR performance, or viewing scan analytics. DECLENCHEURS: track scans, scan analytics, monitor scans, scan statistics, scan tracking, qr tracking. PAS: scan qr (action), analytics (feature).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "shorten-url", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "shorten-url",
  el.display_name = "Shorten URL",
  el.description = "Create short URL",
  el.llm_context = "UTILISER: when discussing URL shortening, creating short links, or compacting long URLs. DECLENCHEURS: shorten url, short link, short url, url shortener, compact link, tiny url. PAS: create smart link (intelligent), custom domain (branding).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "create-smart-link", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "creer-lien-intelligent",
  el.display_name = "Créer Lien Intelligent",
  el.description = "Create intelligent link",
  el.llm_context = "UTILISER: when discussing smart link creation, intelligent links, or advanced short links with routing. DECLENCHEURS: create smart link, smart link, intelligent link, routing link, conditional link. PAS: shorten url (basic), contextual routing (feature).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "create-landing-page", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "creer-page-atterrissage",
  el.display_name = "Créer Page d\'Atterrissage",
  el.description = "Build destination page",
  el.llm_context = "UTILISER: when discussing landing page creation, building destination pages, or creating bio pages. DECLENCHEURS: create landing page, build page, make landing page, destination page, bio page creation. PAS: landing page builder (tool), link in bio (specific type).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "create-barcode", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "creer-barcode",
  el.display_name = "Créer Code-barres",
  el.description = "Generate barcode",
  el.llm_context = "UTILISER: when discussing barcode creation, generating 1D barcodes, or making EAN/UPC codes. DECLENCHEURS: create barcode, generate barcode, make barcode, barcode creation, ean barcode, upc barcode. PAS: create qr (2D), scan barcode (reading).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "scan-barcode", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "scanner-barcode",
  el.display_name = "Scanner Code-barres",
  el.description = "Read barcode data",
  el.llm_context = "UTILISER: when discussing barcode scanning, reading 1D barcodes, or decoding product barcodes. DECLENCHEURS: scan barcode, read barcode, barcode scan, decode barcode, barcode reader. PAS: scan qr (2D), create barcode (making).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "restaurants", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "restaurants",
  el.display_name = "Restaurants",
  el.description = "Food service, cafes, bars",
  el.llm_context = "UTILISER: when discussing QR codes for restaurants, food service, cafes, bars, or digital menus. DECLENCHEURS: restaurant qr, cafe qr, bar qr, food service qr, dine-in qr, table qr. PAS: retail (shopping), hospitality (hotels).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "retail", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "retail",
  el.display_name = "Retail",
  el.description = "Shops, stores, e-commerce",
  el.llm_context = "UTILISER: when discussing QR codes for retail, shops, stores, or e-commerce product labeling. DECLENCHEURS: retail qr, store qr, shop qr, product qr, e-commerce qr, shopping qr. PAS: restaurants (food), hospitality (hotels).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "hospitality", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "hospitality",
  el.display_name = "Hospitality",
  el.description = "Hotels, resorts, tourism",
  el.llm_context = "UTILISER: when discussing QR codes for hotels, resorts, tourism, or guest services. DECLENCHEURS: hotel qr, resort qr, hospitality qr, tourism qr, guest qr, room qr. PAS: restaurants (food), retail (shopping).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "healthcare", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "healthcare",
  el.display_name = "Healthcare",
  el.description = "Hospitals, clinics, pharma",
  el.llm_context = "UTILISER: when discussing QR codes for healthcare, hospitals, clinics, or pharmaceutical applications. DECLENCHEURS: healthcare qr, hospital qr, clinic qr, pharma qr, medical qr, patient qr. PAS: fitness (gyms), beauty (spas).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "education", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "education",
  el.display_name = "Education",
  el.description = "Schools, universities, training",
  el.llm_context = "UTILISER: when discussing QR codes for schools, universities, education, or training materials. DECLENCHEURS: education qr, school qr, university qr, student qr, classroom qr, learning qr. PAS: entertainment (events), government (public).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "real-estate", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "real-estate",
  el.display_name = "Real Estate",
  el.description = "Property sales and rentals",
  el.llm_context = "UTILISER: when discussing QR codes for real estate, property listings, or virtual tours. DECLENCHEURS: real estate qr, property qr, house qr, listing qr, virtual tour qr, for sale qr. PAS: construction (building), hospitality (hotels).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "fitness", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "fitness",
  el.display_name = "Fitness",
  el.description = "Gyms, sports, wellness",
  el.llm_context = "UTILISER: when discussing QR codes for gyms, fitness centers, sports, or wellness applications. DECLENCHEURS: gym qr, fitness qr, sports qr, wellness qr, workout qr, exercise qr. PAS: healthcare (medical), beauty (cosmetics).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "beauty", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "beauty",
  el.display_name = "Beauty",
  el.description = "Salons, spas, cosmetics",
  el.llm_context = "UTILISER: when discussing QR codes for salons, spas, cosmetics, or beauty products. DECLENCHEURS: beauty qr, salon qr, spa qr, cosmetics qr, skincare qr, makeup qr. PAS: fitness (gyms), healthcare (medical).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "entertainment", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "entertainment",
  el.display_name = "Entertainment",
  el.description = "Movies, games, events",
  el.llm_context = "UTILISER: when discussing QR codes for entertainment, movies, games, or live events. DECLENCHEURS: entertainment qr, movie qr, event qr, concert qr, theater qr, gaming qr. PAS: education (learning), hospitality (hotels).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "transportation", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "transportation",
  el.display_name = "Transportation",
  el.description = "Airlines, transit, logistics",
  el.llm_context = "UTILISER: when discussing QR codes for airlines, transit, or transportation ticketing. DECLENCHEURS: airline qr, transit qr, transportation qr, boarding pass qr, train qr, bus qr. PAS: logistics (warehouse), manufacturing (production).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "manufacturing", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "manufacturing",
  el.display_name = "Manufacturing",
  el.description = "Production and assembly",
  el.llm_context = "UTILISER: when discussing QR codes for manufacturing, production, or assembly lines. DECLENCHEURS: manufacturing qr, factory qr, production qr, assembly qr, industrial qr, parts qr. PAS: logistics (shipping), construction (building).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "logistics", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "logistics",
  el.display_name = "Logistics",
  el.description = "Shipping and warehousing",
  el.llm_context = "UTILISER: when discussing QR codes for logistics, shipping, or warehouse management. DECLENCHEURS: logistics qr, shipping qr, warehouse qr, delivery qr, supply chain qr, tracking qr. PAS: manufacturing (production), retail (stores).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "construction", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "construction",
  el.display_name = "Construction",
  el.description = "Building and infrastructure",
  el.llm_context = "UTILISER: when discussing QR codes for construction sites, building projects, or infrastructure. DECLENCHEURS: construction qr, building qr, site qr, contractor qr, blueprint qr, safety qr. PAS: real estate (sales), manufacturing (production).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "finance", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "finance",
  el.display_name = "Finance",
  el.description = "Banking and insurance",
  el.llm_context = "UTILISER: when discussing QR codes for banking, finance, insurance, or financial services. DECLENCHEURS: finance qr, banking qr, insurance qr, payment qr, atm qr, fintech qr. PAS: retail (shopping), government (public sector).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "government", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "government",
  el.display_name = "Government",
  el.description = "Public sector and administration",
  el.llm_context = "UTILISER: when discussing QR codes for government, public sector, or citizen services. DECLENCHEURS: government qr, public sector qr, citizen qr, municipal qr, civic qr, id card qr. PAS: nonprofit (charity), enterprise (business).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "marketing-agencies", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "marketing-agencies",
  el.display_name = "Marketing Agencies",
  el.description = "Digital marketing firms",
  el.llm_context = "UTILISER: when discussing QR codes for marketing agencies, digital marketing, or advertising campaigns. DECLENCHEURS: marketing agency qr, digital marketing qr, campaign qr, advertising qr, agency qr. PAS: creative agencies (design), consulting (business).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "creative-agencies", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "creative-agencies",
  el.display_name = "Creative Agencies",
  el.description = "Design and branding",
  el.llm_context = "UTILISER: when discussing QR codes for creative agencies, design firms, or branding work. DECLENCHEURS: creative agency qr, design agency qr, branding qr, designer qr, creative qr. PAS: marketing agencies (advertising), consulting (business).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "event-management", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "event-management",
  el.display_name = "Event Management",
  el.description = "Conferences, weddings, events",
  el.llm_context = "UTILISER: when discussing QR codes for event management, conferences, weddings, or event planning. DECLENCHEURS: event management qr, conference qr, wedding qr, event planner qr, venue qr. PAS: entertainment (movies), hospitality (hotels).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "nonprofits", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "nonprofits",
  el.display_name = "Nonprofits",
  el.description = "Charities and NGOs",
  el.llm_context = "UTILISER: when discussing QR codes for nonprofits, charities, NGOs, or donation collection. DECLENCHEURS: nonprofit qr, charity qr, ngo qr, donation qr, fundraising qr, volunteer qr. PAS: government (public sector), small business (commercial).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "consulting", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "consulting",
  el.display_name = "Consulting",
  el.description = "Business consulting",
  el.llm_context = "UTILISER: when discussing QR codes for consulting firms, business consulting, or professional services. DECLENCHEURS: consulting qr, consultant qr, advisory qr, professional services qr, firm qr. PAS: agencies (marketing), enterprise (large corp).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "developers", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "developers",
  el.display_name = "Developers",
  el.description = "API users and integrators",
  el.llm_context = "UTILISER: when discussing developers, API integration, or programmatic QR code generation. DECLENCHEURS: developer qr, api user, programmer qr, integrator, coder qr, tech qr. PAS: enterprise (organization), agencies (marketing).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "enterprise", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "enterprise",
  el.display_name = "Enterprise",
  el.description = "Large organizations",
  el.llm_context = "UTILISER: when discussing QR codes for enterprise, large organizations, or corporate deployments. DECLENCHEURS: enterprise qr, corporate qr, large organization qr, company-wide qr, sso qr. PAS: small business (smb), freelancers (individual).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "agencies", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "agencies",
  el.display_name = "Agencies",
  el.description = "Marketing and creative agencies",
  el.llm_context = "UTILISER: when discussing agencies managing QR codes for multiple clients. DECLENCHEURS: agency qr, multi-client qr, client management qr, agency workspaces. PAS: enterprise (internal), small business (single owner).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "small-business", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "small-business",
  el.display_name = "Small Business",
  el.description = "SMBs and local businesses",
  el.llm_context = "UTILISER: when discussing QR codes for small businesses, SMBs, or local businesses. DECLENCHEURS: small business qr, smb qr, local business qr, shop owner qr, mom and pop qr. PAS: enterprise (large), freelancers (individual).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "freelancers", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "freelancers",
  el.display_name = "Gratuitlancers",
  el.description = "Independent professionals",
  el.llm_context = "UTILISER: when discussing QR codes for freelancers, independent professionals, or solo entrepreneurs. DECLENCHEURS: freelancer qr, independent qr, solo qr, self-employed qr, contractor qr. PAS: small business (employees), agencies (teams).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "instagram", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "instagram",
  el.display_name = "Instagram",
  el.description = "Photo and video social network",
  el.llm_context = "UTILISER: when discussing Instagram QR codes, Instagram profiles, or Meta social sharing. DECLENCHEURS: instagram, ig, instagram qr, instagram profile, instagram link. PAS: facebook (separate platform), tiktok (competitor).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "linkedin", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "linkedin",
  el.display_name = "LinkedIn",
  el.description = "Professional networking platform",
  el.llm_context = "UTILISER: when discussing LinkedIn QR codes, professional networking, or business profiles. DECLENCHEURS: linkedin, linkedin qr, linkedin profile, professional network, business network. PAS: facebook (social), twitter (microblog).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "facebook", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "facebook",
  el.display_name = "Facebook",
  el.description = "Social networking platform",
  el.llm_context = "UTILISER: when discussing Facebook QR codes, Facebook pages, or Meta social networking. DECLENCHEURS: facebook, fb, facebook qr, facebook page, facebook group. PAS: instagram (visual), linkedin (professional).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "twitter", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "twitter",
  el.display_name = "Twitter / X",
  el.description = "Microblogging social platform",
  el.llm_context = "UTILISER: when discussing Twitter/X QR codes, tweets, or microblogging. DECLENCHEURS: twitter, x, twitter qr, tweet, x platform, twitter profile. PAS: facebook (social network), linkedin (professional).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "youtube", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "youtube",
  el.display_name = "YouTube",
  el.description = "Video sharing platform",
  el.llm_context = "UTILISER: when discussing YouTube QR codes, YouTube channels, or video linking. DECLENCHEURS: youtube, yt, youtube qr, youtube channel, youtube video, video platform. PAS: tiktok (short-form), spotify (audio).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "tiktok", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "tiktok",
  el.display_name = "TikTok",
  el.description = "Short-form video platform",
  el.llm_context = "UTILISER: when discussing TikTok QR codes, short-form video, or Gen Z marketing. DECLENCHEURS: tiktok, tik tok, tiktok qr, tiktok profile, short video. PAS: youtube (long-form), instagram (photos).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "snapchat", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "snapchat",
  el.display_name = "Snapchat",
  el.description = "Ephemeral messaging and AR platform",
  el.llm_context = "UTILISER: when discussing Snapchat QR codes, Snapcodes, or AR filters. DECLENCHEURS: snapchat, snapcode, snapchat qr, snap, snapchat filter, ar filter. PAS: instagram (stories), tiktok (video).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "whatsapp", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "whatsapp",
  el.display_name = "WhatsApp",
  el.description = "Messaging application",
  el.llm_context = "UTILISER: when discussing WhatsApp QR codes, click-to-chat, or WhatsApp business. DECLENCHEURS: whatsapp, wa, whatsapp qr, whatsapp chat, click to chat, whatsapp business. PAS: telegram (alternative), messenger (facebook).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "telegram", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "telegram",
  el.display_name = "Telegram",
  el.description = "Secure messaging platform",
  el.llm_context = "UTILISER: when discussing Telegram QR codes, Telegram channels, or secure messaging. DECLENCHEURS: telegram, telegram qr, telegram channel, telegram group, secure chat. PAS: whatsapp (alternative), signal (privacy).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "pinterest", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "pinterest",
  el.display_name = "Pinterest",
  el.description = "Visual discovery platform",
  el.llm_context = "UTILISER: when discussing Pinterest QR codes, pins, or visual discovery. DECLENCHEURS: pinterest, pin, pinterest qr, pinterest board, visual discovery, pincode. PAS: instagram (social), etsy (commerce).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "spotify", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "spotify",
  el.display_name = "Spotify",
  el.description = "Music streaming service",
  el.llm_context = "UTILISER: when discussing Spotify QR codes, Spotify Codes, or music streaming links. DECLENCHEURS: spotify, spotify code, spotify qr, music streaming, playlist qr, spotify playlist. PAS: apple music (competitor), soundcloud (indie).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "apple", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "apple",
  el.display_name = "Apple",
  el.description = "Technology company with music and app platforms",
  el.llm_context = "UTILISER: when discussing Apple Music QR codes, App Store links, or Apple ecosystem. DECLENCHEURS: apple, apple music, app store, itunes, ios app, apple qr. PAS: spotify (music streaming), google play (android).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "soundcloud", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "soundcloud",
  el.display_name = "SoundCloud",
  el.description = "Audio streaming and sharing platform",
  el.llm_context = "UTILISER: when discussing SoundCloud QR codes, indie music, or audio sharing. DECLENCHEURS: soundcloud, soundcloud qr, indie music, dj music, audio platform, music upload. PAS: spotify (mainstream), apple music (apple).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "paypal", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "paypal",
  el.display_name = "PayPal",
  el.description = "Digital payment platform",
  el.llm_context = "UTILISER: when discussing PayPal QR codes, PayPal.me links, or PayPal payments. DECLENCHEURS: paypal, paypal qr, paypal.me, paypal payment, online payment. PAS: venmo (p2p), stripe (developer).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "venmo", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "venmo",
  el.display_name = "Venmo",
  el.description = "Mobile payment service",
  el.llm_context = "UTILISER: when discussing Venmo QR codes, Venmo payments, or peer-to-peer US payments. DECLENCHEURS: venmo, venmo qr, venmo payment, split bill, peer payment, p2p payment. PAS: paypal (parent), zelle (bank).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "google", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "google",
  el.display_name = "Google",
  el.description = "Technology company with maps, reviews, and business tools",
  el.llm_context = "UTILISER: when discussing Google QR codes, Google Maps, Google Reviews, or Google Business. DECLENCHEURS: google, google maps, google review, google business, google qr, play store. PAS: apple (competitor), waze (navigation).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "waze", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "waze",
  el.display_name = "Waze",
  el.description = "Community-driven navigation app",
  el.llm_context = "UTILISER: when discussing Waze QR codes, Waze navigation, or community GPS. DECLENCHEURS: waze, waze qr, waze navigation, waze directions, community navigation. PAS: google maps (google), apple maps (apple).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "zapier", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "zapier",
  el.display_name = "Zapier",
  el.description = "Workflow automation platform",
  el.llm_context = "UTILISER: when discussing Zapier, workflow automation, or app integrations. DECLENCHEURS: zapier, zap, zapier automation, workflow automation, app connector. PAS: make (integromat), n8n (self-hosted).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "make", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "make",
  el.display_name = "Make (Integromat)",
  el.description = "Visual automation platform",
  el.llm_context = "UTILISER: when discussing Make/Integromat, visual automation, or complex workflows. DECLENCHEURS: make, integromat, make automation, visual automation, scenario builder. PAS: zapier (simpler), n8n (self-hosted).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "n8n", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "n8n",
  el.display_name = "n8n",
  el.description = "Open-source automation tool",
  el.llm_context = "UTILISER: when discussing n8n, self-hosted automation, or open-source workflows. DECLENCHEURS: n8n, self-hosted automation, open source automation, privacy automation. PAS: zapier (hosted), make (hosted).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "hubspot", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "hubspot",
  el.display_name = "HubSpot",
  el.description = "CRM and marketing platform",
  el.llm_context = "UTILISER: when discussing HubSpot, CRM integration, or marketing automation with QR codes. DECLENCHEURS: hubspot, hubspot crm, hubspot marketing, inbound marketing, hubspot integration. PAS: salesforce (enterprise), mailchimp (email only).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "salesforce", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "salesforce",
  el.display_name = "Salesforce",
  el.description = "Enterprise CRM platform",
  el.llm_context = "UTILISER: when discussing Salesforce, enterprise CRM, or sales automation. DECLENCHEURS: salesforce, salesforce crm, enterprise crm, salesforce integration, sales cloud. PAS: hubspot (smb), zoho (alternative).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "mailchimp", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "mailchimp",
  el.display_name = "Mailchimp",
  el.description = "Email marketing platform",
  el.llm_context = "UTILISER: when discussing Mailchimp, email marketing, or newsletter QR codes. DECLENCHEURS: mailchimp, email marketing, mailchimp integration, newsletter qr, email list qr. PAS: hubspot (full crm), sendgrid (api).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "shopify", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "shopify",
  el.display_name = "Shopify",
  el.description = "E-commerce platform",
  el.llm_context = "UTILISER: when discussing Shopify, e-commerce QR codes, or online store integration. DECLENCHEURS: shopify, shopify qr, shopify store, e-commerce platform, shopify product. PAS: woocommerce (wordpress), amazon (marketplace).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "woocommerce", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "woocommerce",
  el.display_name = "WooCommerce",
  el.description = "WordPress e-commerce plugin",
  el.llm_context = "UTILISER: when discussing WooCommerce, WordPress e-commerce, or open-source stores. DECLENCHEURS: woocommerce, woo commerce, wordpress store, woocommerce qr, wordpress e-commerce. PAS: shopify (hosted), magento (enterprise).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "zapier-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "zapier-integration",
  el.display_name = "Zapier Integration",
  el.description = "Workflow automation via Zapier",
  el.llm_context = "UTILISER: when discussing Zapier integration with QR Code AI, automated QR workflows, or zap connections. DECLENCHEURS: zapier integration, qr zapier, zap qr, automate qr, zapier connection. PAS: make integration (different platform), direct api (not integration).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "make-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "make-integration",
  el.display_name = "Make Integration",
  el.description = "Visual automation via Make (Integromat)",
  el.llm_context = "UTILISER: when discussing Make/Integromat integration with QR Code AI or visual scenario automation. DECLENCHEURS: make integration, integromat integration, qr make, qr integromat, visual automation integration. PAS: zapier integration (different platform), n8n integration (self-hosted).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "n8n-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "n8n-integration",
  el.display_name = "n8n Integration",
  el.description = "Self-hosted automation via n8n",
  el.llm_context = "UTILISER: when discussing n8n integration with QR Code AI or self-hosted privacy-first automation. DECLENCHEURS: n8n integration, qr n8n, self-hosted qr automation, privacy qr automation. PAS: zapier integration (hosted), make integration (hosted).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "hubspot-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "hubspot-integration",
  el.display_name = "HubSpot Integration",
  el.description = "CRM and marketing automation via HubSpot",
  el.llm_context = "UTILISER: when discussing HubSpot integration with QR Code AI or QR-to-CRM lead syncing. DECLENCHEURS: hubspot integration, qr hubspot, crm qr integration, hubspot qr sync, marketing automation qr. PAS: salesforce integration (enterprise), mailchimp integration (email).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "salesforce-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "salesforce-integration",
  el.display_name = "Salesforce Integration",
  el.description = "Enterprise CRM via Salesforce",
  el.llm_context = "UTILISER: when discussing Salesforce integration with QR Code AI or enterprise CRM QR connections. DECLENCHEURS: salesforce integration, qr salesforce, enterprise crm qr, salesforce qr sync. PAS: hubspot integration (smb), dynamics integration (microsoft).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "mailchimp-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "mailchimp-integration",
  el.display_name = "Mailchimp Integration",
  el.description = "Email marketing via Mailchimp",
  el.llm_context = "UTILISER: when discussing Mailchimp integration with QR Code AI or QR-to-email list building. DECLENCHEURS: mailchimp integration, qr mailchimp, email qr integration, newsletter qr signup, list building qr. PAS: hubspot integration (full crm), sendgrid (api only).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "google-sheets-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "google-sheets-integration",
  el.display_name = "Google Sheets Integration",
  el.description = "Spreadsheet sync via Google Sheets",
  el.llm_context = "UTILISER: when discussing Google Sheets integration with QR Code AI or spreadsheet QR data sync. DECLENCHEURS: google sheets integration, qr google sheets, spreadsheet qr, sheets qr sync, batch qr from sheets. PAS: notion integration (workspace), airtable (database).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "notion-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "notion-integration",
  el.display_name = "Notion Integration",
  el.description = "Workspace sync via Notion",
  el.llm_context = "UTILISER: when discussing Notion integration with QR Code AI or workspace QR management. DECLENCHEURS: notion integration, qr notion, notion qr sync, workspace qr, notion database qr. PAS: google sheets integration (spreadsheet), coda (alternative).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "slack-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "slack-integration",
  el.display_name = "Slack Integration",
  el.description = "Team notifications via Slack",
  el.llm_context = "UTILISER: when discussing Slack integration with QR Code AI or QR scan notifications. DECLENCHEURS: slack integration, qr slack, slack notifications qr, team alert qr, slack channel qr. PAS: teams integration (microsoft), discord (community).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "shopify-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "shopify-integration",
  el.display_name = "Shopify Integration",
  el.description = "E-commerce QR codes via Shopify",
  el.llm_context = "UTILISER: when discussing Shopify integration with QR Code AI or e-commerce product QR codes. DECLENCHEURS: shopify integration, qr shopify, shopify product qr, e-commerce qr integration, shopify store qr. PAS: woocommerce integration (wordpress), amazon (marketplace).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "woocommerce-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "woocommerce-integration",
  el.display_name = "WooCommerce Integration",
  el.description = "WordPress e-commerce via WooCommerce",
  el.llm_context = "UTILISER: when discussing WooCommerce integration with QR Code AI or WordPress e-commerce QR codes. DECLENCHEURS: woocommerce integration, qr woocommerce, wordpress qr store, woo qr integration. PAS: shopify integration (hosted), magento (enterprise).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "wordpress-integration", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "wordpress-integration",
  el.display_name = "WordPress Integration",
  el.description = "CMS integration via WordPress",
  el.llm_context = "UTILISER: when discussing WordPress integration with QR Code AI or embedding QR codes in WordPress. DECLENCHEURS: wordpress integration, qr wordpress, wordpress qr plugin, wp qr shortcode, cms qr integration. PAS: woocommerce integration (e-commerce), squarespace (different cms).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "quiet-zone", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "quiet-zone",
  el.display_name = "Quiet Zone",
  el.description = "White margin around QR (minimum 4 modules)",
  el.llm_context = "UTILISER: when discussing QR code margins, white border requirements, or scan failures due to cropping. DECLENCHEURS: quiet zone, qr margin, white border, qr padding, module margin, quiet area. PAS: finder pattern (corners), timing pattern (lines).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "error-correction", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "error-correction",
  el.display_name = "Error Correction",
  el.description = "Reed-Solomon encoding (L/M/Q/H levels)",
  el.llm_context = "UTILISER: when discussing QR code damage tolerance, logo placement, or error correction levels L/M/Q/H. DECLENCHEURS: error correction, qr damage, reed solomon, correction level, damaged qr, logo error correction. PAS: data capacity (size), encoding mode (character type).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "data-capacity", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "data-capacity",
  el.display_name = "Data Capacity",
  el.description = "Maximum characters based on version and error correction",
  el.llm_context = "UTILISER: when discussing how much data a QR code can hold, character limits, or QR size requirements. DECLENCHEURS: data capacity, qr capacity, character limit, qr data size, how much data, qr storage. PAS: error correction (damage), encoding mode (format).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-version", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-version",
  el.display_name = "QR Code Version",
  el.description = "Size grid (21x21 v1 to 177x177 v40)",
  el.llm_context = "UTILISER: when discussing QR code size, version numbers 1-40, or module grid dimensions. DECLENCHEURS: qr version, qr size, version 1, version 40, module grid, qr dimensions. PAS: data capacity (characters), error correction (damage).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "encoding-mode", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "encoding-mode",
  el.display_name = "Encoding Mode",
  el.description = "Numeric/Alphanumeric/Byte/Kanji modes",
  el.llm_context = "UTILISER: when discussing QR encoding efficiency, character types, or numeric vs alphanumeric modes. DECLENCHEURS: encoding mode, numeric mode, alphanumeric mode, byte mode, kanji mode, qr encoding. PAS: error correction (damage), data capacity (size).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "module", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "module",
  el.display_name = "Module",
  el.description = "Single black or white square unit",
  el.llm_context = "UTILISER: when discussing QR code pixels, individual squares, or module-level design. DECLENCHEURS: module, qr pixel, qr square, black module, white module, qr unit. PAS: finder pattern (corner squares), quiet zone (border).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "finder-pattern", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "finder-pattern",
  el.display_name = "Finder Pattern",
  el.description = "Three corner squares for orientation",
  el.llm_context = "UTILISER: when discussing QR code corner squares, orientation markers, or why QR codes have three big squares. DECLENCHEURS: finder pattern, corner squares, qr orientation, position detection, three squares, qr corners. PAS: timing pattern (lines), alignment pattern (small squares).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "timing-pattern", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "timing-pattern",
  el.display_name = "Timing Pattern",
  el.description = "Alternating modules for alignment",
  el.llm_context = "UTILISER: when discussing QR code alignment, alternating black/white lines, or grid calibration. DECLENCHEURS: timing pattern, alignment lines, alternating modules, grid calibration, qr timing. PAS: finder pattern (corners), module (single square).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-messaging", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-messaging",
  el.display_name = "Messaging QR Codes",
  el.description = "Subcategory for messaging app QR codes",
  el.llm_context = "UTILISER: when discussing QR codes for messaging apps like WhatsApp, Telegram, or chat platforms. DECLENCHEURS: messaging qr, chat qr, whatsapp category, telegram category, direct message qr. PAS: video platform (video content), professional (business networking).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-video-platform", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-video-platform",
  el.display_name = "Video Platform QR Codes",
  el.description = "Subcategory for video platform QR codes",
  el.llm_context = "UTILISER: when discussing QR codes for video platforms like YouTube, TikTok, or Snapchat. DECLENCHEURS: video platform qr, youtube category, tiktok category, video content qr, streaming qr. PAS: messaging (chat), music platform (audio).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-professional", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-professional",
  el.display_name = "Professional Network QR Codes",
  el.description = "Subcategory for professional networking QR codes",
  el.llm_context = "UTILISER: when discussing QR codes for professional networking like LinkedIn. DECLENCHEURS: professional qr, linkedin category, business networking qr, career qr, professional network. PAS: messaging (chat), video platform (content).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-music-platform", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-music-platform",
  el.display_name = "Music Platform QR Codes",
  el.description = "Subcategory for music streaming QR codes",
  el.llm_context = "UTILISER: when discussing QR codes for music platforms like Spotify, Apple Music, or SoundCloud. DECLENCHEURS: music platform qr, spotify category, apple music category, music streaming qr, playlist category. PAS: video platform (video), audio file (direct file).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "funny-qr-codes", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "funny-qr-codes",
  el.display_name = "Funny QR Codes",
  el.description = "Humorous and creative QR code applications",
  el.llm_context = "UTILISER: when discussing humorous, prank, or creative QR code applications like rickrolling. DECLENCHEURS: funny qr, prank qr, rickroll qr, meme qr, easter egg qr, joke qr. PAS: art installation (serious art), tattoo (permanent).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-tattoo", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-tattoo",
  el.display_name = "QR Code Tattoo",
  el.description = "Permanent QR codes as body art",
  el.llm_context = "UTILISER: when discussing QR code tattoos, permanent body art QR codes, or skin-based QR. DECLENCHEURS: qr tattoo, tattoo qr code, body art qr, permanent qr, skin qr, inked qr. PAS: temporary (sticker), art installation (public).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-art-installation", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-art-installation",
  el.display_name = "QR Art Installation",
  el.description = "Large-scale artistic QR code displays",
  el.llm_context = "UTILISER: when discussing large-scale QR art, public installations, or artistic QR displays. DECLENCHEURS: qr art, art installation qr, mural qr, public art qr, interactive art qr, projection qr. PAS: tattoo (body), funny qr (humor).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-scavenger-hunt", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-scavenger-hunt",
  el.display_name = "QR Scavenger Hunt",
  el.description = "Interactive treasure hunts using QR codes",
  el.llm_context = "UTILISER: when discussing QR scavenger hunts, treasure hunts, or gamified QR experiences. DECLENCHEURS: scavenger hunt qr, treasure hunt qr, qr game, interactive qr hunt, clue qr, quest qr. PAS: event check-in (registration), museum (educational).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-reviews", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-reviews",
  el.display_name = "QR for Reviews",
  el.description = "Collecting customer reviews via QR",
  el.llm_context = "UTILISER: when discussing QR codes for collecting reviews, feedback, or ratings. DECLENCHEURS: review qr, feedback qr, rating qr, google review qr, yelp qr, customer feedback qr. PAS: loyalty program (rewards), payment (transaction).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-contactless-payment", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-contactless-payment",
  el.display_name = "Contactless Payment",
  el.description = "Touch-free payments via QR codes",
  el.llm_context = "UTILISER: when discussing QR-based contactless payments, touch-free transactions, or scan-to-pay. DECLENCHEURS: contactless payment qr, scan to pay, touch-free payment, qr payment, mobile payment qr. PAS: loyalty program (points), reviews (feedback).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-loyalty-program", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-loyalty-program",
  el.display_name = "Loyalty Program",
  el.description = "Customer loyalty via QR scanning",
  el.llm_context = "UTILISER: when discussing QR-based loyalty programs, digital punch cards, or rewards via QR. DECLENCHEURS: loyalty qr, punch card qr, rewards qr, points qr, member qr, stamp card qr. PAS: payment (transaction), reviews (feedback).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-product-authentication", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-product-authentication",
  el.display_name = "Product Authentication",
  el.description = "Anti-counterfeit verification via QR",
  el.llm_context = "UTILISER: when discussing QR codes for product authentication, anti-counterfeiting, or verification. DECLENCHEURS: authentication qr, anti-counterfeit qr, verify product qr, genuine qr, counterfeit detection qr. PAS: loyalty (rewards), payment (transaction).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-event-checkin", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-event-checkin",
  el.display_name = "Event Check-in",
  el.description = "Attendee registration via QR",
  el.llm_context = "UTILISER: when discussing QR codes for event check-in, registration, or attendee verification. DECLENCHEURS: event check-in qr, registration qr, attendee qr, ticket scan, conference check-in, entry qr. PAS: networking (contact exchange), wedding (personal event).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-networking", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-networking",
  el.display_name = "Networking QR",
  el.description = "Contact exchange at events",
  el.llm_context = "UTILISER: when discussing QR codes for professional networking, contact exchange, or event connections. DECLENCHEURS: networking qr, contact exchange qr, meet and greet qr, business card qr, connection qr. PAS: event check-in (registration), wedding (personal).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-wedding", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-wedding",
  el.display_name = "Wedding QR Codes",
  el.description = "QR applications for weddings",
  el.llm_context = "UTILISER: when discussing QR codes for weddings, wedding invitations, or wedding RSVPs. DECLENCHEURS: wedding qr, wedding invitation qr, rsvp qr, wedding registry qr, wedding photo qr, marriage qr. PAS: event check-in (corporate), networking (business).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-museum-exhibit", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-museum-exhibit",
  el.display_name = "Museum Exhibits",
  el.description = "Interactive museum experiences via QR",
  el.llm_context = "UTILISER: when discussing QR codes for museums, exhibits, galleries, or educational displays. DECLENCHEURS: museum qr, exhibit qr, gallery qr, audio guide qr, art museum qr, exhibition qr. PAS: scavenger hunt (game), art installation (creative).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "how-to-create-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "comment-creer-qr-code",
  el.display_name = "Comment Créer QR Code",
  el.description = "Step-by-step QR code creation guide",
  el.llm_context = "UTILISER: when discussing how to create QR codes, QR creation tutorials, or step-by-step QR generation. DECLENCHEURS: how to create qr, make qr code, qr tutorial, create qr guide, generate qr how to. PAS: design guide (aesthetics), print guide (physical output).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-design-guide", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-design-guide",
  el.display_name = "QR Code Design Guide",
  el.description = "Best practices for QR code aesthetics",
  el.llm_context = "UTILISER: when discussing QR code design, styling, customization best practices, or visual aesthetics. DECLENCHEURS: qr design, qr styling, qr aesthetics, beautiful qr, custom qr design, qr appearance. PAS: creation guide (basic), print guide (output).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-print-guide", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-imprimer-guide",
  el.display_name = "QR Code Imprimer Guide",
  el.description = "Guidelines for printing scannable QR codes",
  el.llm_context = "UTILISER: when discussing QR code printing, print requirements, or physical QR production. DECLENCHEURS: qr print, print qr guide, qr printing tips, physical qr, qr size for print, print quality qr. PAS: design guide (digital), creation guide (generation).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "dynamic-vs-static-guide", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "dynamique-vs-statique-guide",
  el.display_name = "Dynamique vs Statique Guide",
  el.description = "When to use dynamic vs static QR codes",
  el.llm_context = "UTILISER: when discussing choosing between dynamic and static QR codes or comparing QR types. DECLENCHEURS: dynamic vs static, which qr type, qr type comparison, editable qr, trackable qr choice. PAS: comparison entity (detailed), creation guide (how-to).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-marketing-guide", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-marketing-guide",
  el.display_name = "QR Marketing Guide",
  el.description = "Using QR codes for marketing campaigns",
  el.llm_context = "UTILISER: when discussing QR codes for marketing, campaign strategies, or print-to-digital marketing. DECLENCHEURS: qr marketing, marketing qr guide, campaign qr, advertising qr, roi qr, print to digital. PAS: restaurant guide (industry), business card guide (specific use).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-restaurant-guide", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-restaurant-guide",
  el.display_name = "Restaurant QR Guide",
  el.description = "QR codes for restaurants and cafes",
  el.llm_context = "UTILISER: when discussing QR codes specifically for restaurants, cafes, or food service. DECLENCHEURS: restaurant qr guide, menu qr, cafe qr, food service qr, dining qr, table qr guide. PAS: marketing guide (general), business card guide (networking).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-business-card-guide", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-carte-visite-guide",
  el.display_name = "Carte de Visite QR Guide",
  el.description = "Adding QR codes to business cards",
  el.llm_context = "UTILISER: when discussing QR codes on business cards, networking cards, or professional contact sharing. DECLENCHEURS: business card qr guide, vcard qr, networking card qr, professional qr, contact card qr. PAS: restaurant guide (food), marketing guide (campaigns).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-api-guide", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-api-guide",
  el.display_name = "QR Code API Guide",
  el.description = "Developer guide for QR code API integration",
  el.llm_context = "UTILISER: when discussing QR code API documentation, developer integration, or programmatic QR generation. DECLENCHEURS: qr api guide, developer qr, api integration qr, programmatic qr, qr api docs. PAS: analytics guide (tracking), security guide (safety).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-analytics-guide", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-analytics-guide",
  el.display_name = "Analytiques Guide",
  el.description = "Understanding QR code scan analytics",
  el.llm_context = "UTILISER: when discussing QR scan analytics, tracking interpretation, or campaign measurement. DECLENCHEURS: qr analytics guide, scan tracking guide, qr metrics, analytics interpretation, campaign analytics. PAS: api guide (development), marketing guide (strategy).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-security-guide", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-security-guide",
  el.display_name = "QR Security Guide",
  el.description = "Security best practices for QR codes",
  el.llm_context = "UTILISER: when discussing QR code security, phishing prevention, or safe QR practices. DECLENCHEURS: qr security, safe qr, qr phishing, qr safety, malicious qr, secure qr. PAS: api guide (development), analytics guide (tracking).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-vs-barcode", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-vs-barcode",
  el.display_name = "QR Code vs Code-barres",
  el.description = "Comparing 2D QR codes with 1D barcodes",
  el.llm_context = "UTILISER: when discussing differences between QR codes and traditional barcodes or 1D vs 2D codes. DECLENCHEURS: qr vs barcode, barcode vs qr, 1d vs 2d, qr or barcode, qr code barcode difference. PAS: qr vs nfc (wireless), qr vs data matrix (both 2D).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "dynamic-vs-static-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "dynamique-vs-statique-qr-code",
  el.display_name = "Dynamique vs Statique QR Code",
  el.description = "Comparing editable vs fixed QR codes",
  el.llm_context = "UTILISER: when discussing differences between dynamic and static QR codes or editable vs permanent QR. DECLENCHEURS: dynamic vs static qr, editable qr, trackable qr, static vs dynamic, permanent qr vs editable. PAS: guide (how-to), qr vs barcode (different formats).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-vs-nfc", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-vs-nfc",
  el.display_name = "QR Code vs NFC",
  el.description = "Comparing QR codes with NFC technology",
  el.llm_context = "UTILISER: when discussing differences between QR codes and NFC or visual vs tap technology. DECLENCHEURS: qr vs nfc, nfc vs qr, scan vs tap, qr or nfc, contactless comparison. PAS: qr vs barcode (both visual), qr vs data matrix (both 2D).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-vs-data-matrix", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-vs-data-matrix",
  el.display_name = "QR Code vs Data Matrix",
  el.description = "Comparing QR codes with Data Matrix codes",
  el.llm_context = "UTILISER: when discussing differences between QR codes and Data Matrix or consumer vs industrial 2D codes. DECLENCHEURS: qr vs data matrix, data matrix vs qr, which 2d code, qr or datamatrix. PAS: qr vs barcode (1D), qr vs nfc (wireless).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "free-vs-paid-qr-generator", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "free-vs-paid-qr-generator",
  el.display_name = "Gratuit vs Payant QR Générateur",
  el.description = "Comparing free and premium QR code tools",
  el.llm_context = "UTILISER: when discussing free vs paid QR generators, pricing tiers, or premium QR features. DECLENCHEURS: free vs paid qr, qr generator pricing, premium qr, free qr limitations, paid qr benefits. PAS: platform comparison (specific tools), dynamic vs static (code types).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "qr-code-ai-vs-competitors", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "qr-code-ai-vs-competitors",
  el.display_name = "QR Code AI vs Competitors",
  el.description = "How QR Code AI compares to other platforms",
  el.llm_context = "UTILISER: when discussing QR Code AI platform comparisons or competitive analysis. DECLENCHEURS: qr code ai vs, compare qr platforms, qr code ai alternative, qr generator comparison, best qr platform. PAS: free vs paid (pricing), dynamic vs static (code types).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "short-link-vs-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "lien-court-vs-qr-code",
  el.display_name = "Lien Court vs QR Code",
  el.description = "When to use short links vs QR codes",
  el.llm_context = "UTILISER: when discussing short links vs QR codes or when to use each technology. DECLENCHEURS: short link vs qr, url vs qr, link or qr, bitly vs qr, when to use qr. PAS: dynamic vs static (both QR), qr vs nfc (hardware).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

MERGE (el:EntityL10n {entity_key: "spotify-code-vs-qr-code", locale_key: "fr-FR"})
ON CREATE SET
  el.slug = "spotify-code-vs-qr-code",
  el.display_name = "Spotify Code vs QR Code",
  el.description = "Comparing Spotify Codes with standard QR codes",
  el.llm_context = "UTILISER: when discussing Spotify Codes vs standard QR codes or proprietary vs universal codes. DECLENCHEURS: spotify code vs qr, spotify qr, spotify code difference, proprietary qr, music qr comparison. PAS: qr vs barcode (format), platform comparison (generators).",
  el.version = 1,
  el.created_at = datetime(),
  el.updated_at = datetime();

// ───────────────────────────────────────────────────────────────────────────────
// Link EntityL10n to Entity via HAS_L10N
// ───────────────────────────────────────────────────────────────────────────────

MATCH (e:Entity {key: "qr-code"})
MATCH (el:EntityL10n {entity_key: "qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "smart-link"})
MATCH (el:EntityL10n {entity_key: "smart-link", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "barcode"})
MATCH (el:EntityL10n {entity_key: "barcode", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "landing-page"})
MATCH (el:EntityL10n {entity_key: "landing-page", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "short-link"})
MATCH (el:EntityL10n {entity_key: "short-link", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-style"})
MATCH (el:EntityL10n {entity_key: "qr-code-style", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-content"})
MATCH (el:EntityL10n {entity_key: "qr-code-content", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-frame"})
MATCH (el:EntityL10n {entity_key: "qr-code-frame", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "barcode-format"})
MATCH (el:EntityL10n {entity_key: "barcode-format", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "landing-page-type"})
MATCH (el:EntityL10n {entity_key: "landing-page-type", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "custom-qr-code"})
MATCH (el:EntityL10n {entity_key: "custom-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-image"})
MATCH (el:EntityL10n {entity_key: "qr-code-image", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-art"})
MATCH (el:EntityL10n {entity_key: "qr-code-art", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-photo"})
MATCH (el:EntityL10n {entity_key: "qr-code-photo", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-with-logo"})
MATCH (el:EntityL10n {entity_key: "qr-code-with-logo", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-with-text"})
MATCH (el:EntityL10n {entity_key: "qr-code-with-text", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-color"})
MATCH (el:EntityL10n {entity_key: "qr-code-color", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-shapes"})
MATCH (el:EntityL10n {entity_key: "qr-code-shapes", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-transparent-background"})
MATCH (el:EntityL10n {entity_key: "qr-code-transparent-background", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-background"})
MATCH (el:EntityL10n {entity_key: "qr-code-background", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-background-color"})
MATCH (el:EntityL10n {entity_key: "qr-code-background-color", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-background-gradient"})
MATCH (el:EntityL10n {entity_key: "qr-code-background-gradient", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-background-image"})
MATCH (el:EntityL10n {entity_key: "qr-code-background-image", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "dynamic-qr-code"})
MATCH (el:EntityL10n {entity_key: "dynamic-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "static-qr-code"})
MATCH (el:EntityL10n {entity_key: "static-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-light-mode"})
MATCH (el:EntityL10n {entity_key: "qr-code-light-mode", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-dark-mode"})
MATCH (el:EntityL10n {entity_key: "qr-code-dark-mode", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-business-card"})
MATCH (el:EntityL10n {entity_key: "qr-code-business-card", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-email-signature"})
MATCH (el:EntityL10n {entity_key: "qr-code-email-signature", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-flyer"})
MATCH (el:EntityL10n {entity_key: "qr-code-flyer", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-poster"})
MATCH (el:EntityL10n {entity_key: "qr-code-poster", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-table-tent"})
MATCH (el:EntityL10n {entity_key: "qr-code-table-tent", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-packaging-label"})
MATCH (el:EntityL10n {entity_key: "qr-code-packaging-label", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "link-in-bio"})
MATCH (el:EntityL10n {entity_key: "link-in-bio", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "menu-restaurant"})
MATCH (el:EntityL10n {entity_key: "menu-restaurant", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "forms"})
MATCH (el:EntityL10n {entity_key: "forms", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "announcement"})
MATCH (el:EntityL10n {entity_key: "announcement", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "event-rsvp"})
MATCH (el:EntityL10n {entity_key: "event-rsvp", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "booking-appointment"})
MATCH (el:EntityL10n {entity_key: "booking-appointment", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-url"})
MATCH (el:EntityL10n {entity_key: "qr-code-url", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-wifi"})
MATCH (el:EntityL10n {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-vcard"})
MATCH (el:EntityL10n {entity_key: "qr-code-vcard", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-mecard"})
MATCH (el:EntityL10n {entity_key: "qr-code-mecard", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-pdf"})
MATCH (el:EntityL10n {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-text"})
MATCH (el:EntityL10n {entity_key: "qr-code-text", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-email"})
MATCH (el:EntityL10n {entity_key: "qr-code-email", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-sms"})
MATCH (el:EntityL10n {entity_key: "qr-code-sms", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-phone"})
MATCH (el:EntityL10n {entity_key: "qr-code-phone", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-video"})
MATCH (el:EntityL10n {entity_key: "qr-code-video", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-audio"})
MATCH (el:EntityL10n {entity_key: "qr-code-audio", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-image-gallery"})
MATCH (el:EntityL10n {entity_key: "qr-code-image-gallery", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-coupon"})
MATCH (el:EntityL10n {entity_key: "qr-code-coupon", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-social"})
MATCH (el:EntityL10n {entity_key: "qr-code-social", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-instagram"})
MATCH (el:EntityL10n {entity_key: "qr-code-instagram", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-linkedin"})
MATCH (el:EntityL10n {entity_key: "qr-code-linkedin", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-facebook"})
MATCH (el:EntityL10n {entity_key: "qr-code-facebook", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-twitter"})
MATCH (el:EntityL10n {entity_key: "qr-code-twitter", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-youtube"})
MATCH (el:EntityL10n {entity_key: "qr-code-youtube", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-tiktok"})
MATCH (el:EntityL10n {entity_key: "qr-code-tiktok", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-snapchat"})
MATCH (el:EntityL10n {entity_key: "qr-code-snapchat", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-whatsapp"})
MATCH (el:EntityL10n {entity_key: "qr-code-whatsapp", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-telegram"})
MATCH (el:EntityL10n {entity_key: "qr-code-telegram", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-pinterest"})
MATCH (el:EntityL10n {entity_key: "qr-code-pinterest", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-spotify"})
MATCH (el:EntityL10n {entity_key: "qr-code-spotify", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-apple-music"})
MATCH (el:EntityL10n {entity_key: "qr-code-apple-music", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-soundcloud"})
MATCH (el:EntityL10n {entity_key: "qr-code-soundcloud", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-payment"})
MATCH (el:EntityL10n {entity_key: "qr-code-payment", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-pix"})
MATCH (el:EntityL10n {entity_key: "qr-code-pix", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-upi"})
MATCH (el:EntityL10n {entity_key: "qr-code-upi", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-paypal"})
MATCH (el:EntityL10n {entity_key: "qr-code-paypal", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-venmo"})
MATCH (el:EntityL10n {entity_key: "qr-code-venmo", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-bitcoin"})
MATCH (el:EntityL10n {entity_key: "qr-code-bitcoin", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-ethereum"})
MATCH (el:EntityL10n {entity_key: "qr-code-ethereum", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-crypto"})
MATCH (el:EntityL10n {entity_key: "qr-code-crypto", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-bank-transfer"})
MATCH (el:EntityL10n {entity_key: "qr-code-bank-transfer", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-location"})
MATCH (el:EntityL10n {entity_key: "qr-code-location", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-google-maps"})
MATCH (el:EntityL10n {entity_key: "qr-code-google-maps", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-apple-maps"})
MATCH (el:EntityL10n {entity_key: "qr-code-apple-maps", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-waze"})
MATCH (el:EntityL10n {entity_key: "qr-code-waze", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-coordinates"})
MATCH (el:EntityL10n {entity_key: "qr-code-coordinates", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-app"})
MATCH (el:EntityL10n {entity_key: "qr-code-app", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-app-store"})
MATCH (el:EntityL10n {entity_key: "qr-code-app-store", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-play-store"})
MATCH (el:EntityL10n {entity_key: "qr-code-play-store", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-app-download"})
MATCH (el:EntityL10n {entity_key: "qr-code-app-download", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-review"})
MATCH (el:EntityL10n {entity_key: "qr-code-review", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-survey"})
MATCH (el:EntityL10n {entity_key: "qr-code-survey", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-feedback"})
MATCH (el:EntityL10n {entity_key: "qr-code-feedback", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-menu"})
MATCH (el:EntityL10n {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-resume"})
MATCH (el:EntityL10n {entity_key: "qr-code-resume", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-certificate"})
MATCH (el:EntityL10n {entity_key: "qr-code-certificate", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-ticket"})
MATCH (el:EntityL10n {entity_key: "qr-code-ticket", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-attendance"})
MATCH (el:EntityL10n {entity_key: "qr-code-attendance", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-pet-tag"})
MATCH (el:EntityL10n {entity_key: "qr-code-pet-tag", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-medical-id"})
MATCH (el:EntityL10n {entity_key: "qr-code-medical-id", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-file"})
MATCH (el:EntityL10n {entity_key: "qr-code-file", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-calendar"})
MATCH (el:EntityL10n {entity_key: "qr-code-calendar", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "ean-13"})
MATCH (el:EntityL10n {entity_key: "ean-13", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "ean-8"})
MATCH (el:EntityL10n {entity_key: "ean-8", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "upc-a"})
MATCH (el:EntityL10n {entity_key: "upc-a", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "upc-e"})
MATCH (el:EntityL10n {entity_key: "upc-e", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "code-128"})
MATCH (el:EntityL10n {entity_key: "code-128", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "code-39"})
MATCH (el:EntityL10n {entity_key: "code-39", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "itf-14"})
MATCH (el:EntityL10n {entity_key: "itf-14", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "codabar"})
MATCH (el:EntityL10n {entity_key: "codabar", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "msi-plessey"})
MATCH (el:EntityL10n {entity_key: "msi-plessey", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "gs1-128"})
MATCH (el:EntityL10n {entity_key: "gs1-128", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "data-matrix"})
MATCH (el:EntityL10n {entity_key: "data-matrix", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "pdf417"})
MATCH (el:EntityL10n {entity_key: "pdf417", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "aztec-code"})
MATCH (el:EntityL10n {entity_key: "aztec-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "maxicode"})
MATCH (el:EntityL10n {entity_key: "maxicode", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "gs1-datamatrix"})
MATCH (el:EntityL10n {entity_key: "gs1-datamatrix", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "barcode-generator"})
MATCH (el:EntityL10n {entity_key: "barcode-generator", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "barcode-scanner"})
MATCH (el:EntityL10n {entity_key: "barcode-scanner", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "analytics"})
MATCH (el:EntityL10n {entity_key: "analytics", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "click-tracking"})
MATCH (el:EntityL10n {entity_key: "click-tracking", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "scan-counting"})
MATCH (el:EntityL10n {entity_key: "scan-counting", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "geo-tracking"})
MATCH (el:EntityL10n {entity_key: "geo-tracking", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "device-detection"})
MATCH (el:EntityL10n {entity_key: "device-detection", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "time-series"})
MATCH (el:EntityL10n {entity_key: "time-series", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "contextual-routing"})
MATCH (el:EntityL10n {entity_key: "contextual-routing", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "custom-domain-name"})
MATCH (el:EntityL10n {entity_key: "custom-domain-name", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "custom-link-preview"})
MATCH (el:EntityL10n {entity_key: "custom-link-preview", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "utm-builder"})
MATCH (el:EntityL10n {entity_key: "utm-builder", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "bulk-creation"})
MATCH (el:EntityL10n {entity_key: "bulk-creation", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "team-workspaces"})
MATCH (el:EntityL10n {entity_key: "team-workspaces", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "api"})
MATCH (el:EntityL10n {entity_key: "api", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "webhooks"})
MATCH (el:EntityL10n {entity_key: "webhooks", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "white-label"})
MATCH (el:EntityL10n {entity_key: "white-label", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "password-protection"})
MATCH (el:EntityL10n {entity_key: "password-protection", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "expiration"})
MATCH (el:EntityL10n {entity_key: "expiration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "scan-limit"})
MATCH (el:EntityL10n {entity_key: "scan-limit", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "retargeting-pixel"})
MATCH (el:EntityL10n {entity_key: "retargeting-pixel", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-generator"})
MATCH (el:EntityL10n {entity_key: "qr-code-generator", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-scanner"})
MATCH (el:EntityL10n {entity_key: "qr-code-scanner", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-api"})
MATCH (el:EntityL10n {entity_key: "qr-code-api", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "landing-page-builder"})
MATCH (el:EntityL10n {entity_key: "landing-page-builder", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "url-shortener"})
MATCH (el:EntityL10n {entity_key: "url-shortener", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "link-in-bio-builder"})
MATCH (el:EntityL10n {entity_key: "link-in-bio-builder", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "menu-builder"})
MATCH (el:EntityL10n {entity_key: "menu-builder", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "vcard-generator"})
MATCH (el:EntityL10n {entity_key: "vcard-generator", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "wifi-qr-generator"})
MATCH (el:EntityL10n {entity_key: "wifi-qr-generator", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "batch-qr-generator"})
MATCH (el:EntityL10n {entity_key: "batch-qr-generator", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "business-cards"})
MATCH (el:EntityL10n {entity_key: "business-cards", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "flyers"})
MATCH (el:EntityL10n {entity_key: "flyers", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "posters-billboards"})
MATCH (el:EntityL10n {entity_key: "posters-billboards", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "brochures"})
MATCH (el:EntityL10n {entity_key: "brochures", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "catalogs"})
MATCH (el:EntityL10n {entity_key: "catalogs", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "magazines"})
MATCH (el:EntityL10n {entity_key: "magazines", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "newspapers"})
MATCH (el:EntityL10n {entity_key: "newspapers", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "direct-mail"})
MATCH (el:EntityL10n {entity_key: "direct-mail", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "stickers-labels"})
MATCH (el:EntityL10n {entity_key: "stickers-labels", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "banners"})
MATCH (el:EntityL10n {entity_key: "banners", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "product-packaging"})
MATCH (el:EntityL10n {entity_key: "product-packaging", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "product-labels"})
MATCH (el:EntityL10n {entity_key: "product-labels", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "receipts"})
MATCH (el:EntityL10n {entity_key: "receipts", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "table-tents"})
MATCH (el:EntityL10n {entity_key: "table-tents", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "menus-printed"})
MATCH (el:EntityL10n {entity_key: "menus-printed", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "tickets-physical"})
MATCH (el:EntityL10n {entity_key: "tickets-physical", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "emails"})
MATCH (el:EntityL10n {entity_key: "emails", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "presentations"})
MATCH (el:EntityL10n {entity_key: "presentations", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "documents"})
MATCH (el:EntityL10n {entity_key: "documents", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "websites"})
MATCH (el:EntityL10n {entity_key: "websites", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "create-qr-code"})
MATCH (el:EntityL10n {entity_key: "create-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "scan-qr-code"})
MATCH (el:EntityL10n {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "customize-qr-code"})
MATCH (el:EntityL10n {entity_key: "customize-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "download-qr-code"})
MATCH (el:EntityL10n {entity_key: "download-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "print-qr-code"})
MATCH (el:EntityL10n {entity_key: "print-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "add-logo"})
MATCH (el:EntityL10n {entity_key: "add-logo", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "change-colors"})
MATCH (el:EntityL10n {entity_key: "change-colors", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "edit-destination"})
MATCH (el:EntityL10n {entity_key: "edit-destination", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "share-qr-code"})
MATCH (el:EntityL10n {entity_key: "share-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "track-scans"})
MATCH (el:EntityL10n {entity_key: "track-scans", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "shorten-url"})
MATCH (el:EntityL10n {entity_key: "shorten-url", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "create-smart-link"})
MATCH (el:EntityL10n {entity_key: "create-smart-link", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "create-landing-page"})
MATCH (el:EntityL10n {entity_key: "create-landing-page", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "create-barcode"})
MATCH (el:EntityL10n {entity_key: "create-barcode", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "scan-barcode"})
MATCH (el:EntityL10n {entity_key: "scan-barcode", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "restaurants"})
MATCH (el:EntityL10n {entity_key: "restaurants", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "retail"})
MATCH (el:EntityL10n {entity_key: "retail", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "hospitality"})
MATCH (el:EntityL10n {entity_key: "hospitality", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "healthcare"})
MATCH (el:EntityL10n {entity_key: "healthcare", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "education"})
MATCH (el:EntityL10n {entity_key: "education", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "real-estate"})
MATCH (el:EntityL10n {entity_key: "real-estate", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "fitness"})
MATCH (el:EntityL10n {entity_key: "fitness", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "beauty"})
MATCH (el:EntityL10n {entity_key: "beauty", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "entertainment"})
MATCH (el:EntityL10n {entity_key: "entertainment", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "transportation"})
MATCH (el:EntityL10n {entity_key: "transportation", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "manufacturing"})
MATCH (el:EntityL10n {entity_key: "manufacturing", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "logistics"})
MATCH (el:EntityL10n {entity_key: "logistics", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "construction"})
MATCH (el:EntityL10n {entity_key: "construction", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "finance"})
MATCH (el:EntityL10n {entity_key: "finance", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "government"})
MATCH (el:EntityL10n {entity_key: "government", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "marketing-agencies"})
MATCH (el:EntityL10n {entity_key: "marketing-agencies", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "creative-agencies"})
MATCH (el:EntityL10n {entity_key: "creative-agencies", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "event-management"})
MATCH (el:EntityL10n {entity_key: "event-management", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "nonprofits"})
MATCH (el:EntityL10n {entity_key: "nonprofits", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "consulting"})
MATCH (el:EntityL10n {entity_key: "consulting", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "developers"})
MATCH (el:EntityL10n {entity_key: "developers", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "enterprise"})
MATCH (el:EntityL10n {entity_key: "enterprise", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "agencies"})
MATCH (el:EntityL10n {entity_key: "agencies", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "small-business"})
MATCH (el:EntityL10n {entity_key: "small-business", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "freelancers"})
MATCH (el:EntityL10n {entity_key: "freelancers", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "instagram"})
MATCH (el:EntityL10n {entity_key: "instagram", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "linkedin"})
MATCH (el:EntityL10n {entity_key: "linkedin", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "facebook"})
MATCH (el:EntityL10n {entity_key: "facebook", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "twitter"})
MATCH (el:EntityL10n {entity_key: "twitter", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "youtube"})
MATCH (el:EntityL10n {entity_key: "youtube", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "tiktok"})
MATCH (el:EntityL10n {entity_key: "tiktok", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "snapchat"})
MATCH (el:EntityL10n {entity_key: "snapchat", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "whatsapp"})
MATCH (el:EntityL10n {entity_key: "whatsapp", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "telegram"})
MATCH (el:EntityL10n {entity_key: "telegram", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "pinterest"})
MATCH (el:EntityL10n {entity_key: "pinterest", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "spotify"})
MATCH (el:EntityL10n {entity_key: "spotify", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "apple"})
MATCH (el:EntityL10n {entity_key: "apple", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "soundcloud"})
MATCH (el:EntityL10n {entity_key: "soundcloud", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "paypal"})
MATCH (el:EntityL10n {entity_key: "paypal", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "venmo"})
MATCH (el:EntityL10n {entity_key: "venmo", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "google"})
MATCH (el:EntityL10n {entity_key: "google", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "waze"})
MATCH (el:EntityL10n {entity_key: "waze", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "zapier"})
MATCH (el:EntityL10n {entity_key: "zapier", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "make"})
MATCH (el:EntityL10n {entity_key: "make", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "n8n"})
MATCH (el:EntityL10n {entity_key: "n8n", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "hubspot"})
MATCH (el:EntityL10n {entity_key: "hubspot", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "salesforce"})
MATCH (el:EntityL10n {entity_key: "salesforce", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "mailchimp"})
MATCH (el:EntityL10n {entity_key: "mailchimp", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "shopify"})
MATCH (el:EntityL10n {entity_key: "shopify", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "woocommerce"})
MATCH (el:EntityL10n {entity_key: "woocommerce", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "zapier-integration"})
MATCH (el:EntityL10n {entity_key: "zapier-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "make-integration"})
MATCH (el:EntityL10n {entity_key: "make-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "n8n-integration"})
MATCH (el:EntityL10n {entity_key: "n8n-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "hubspot-integration"})
MATCH (el:EntityL10n {entity_key: "hubspot-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "salesforce-integration"})
MATCH (el:EntityL10n {entity_key: "salesforce-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "mailchimp-integration"})
MATCH (el:EntityL10n {entity_key: "mailchimp-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "google-sheets-integration"})
MATCH (el:EntityL10n {entity_key: "google-sheets-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "notion-integration"})
MATCH (el:EntityL10n {entity_key: "notion-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "slack-integration"})
MATCH (el:EntityL10n {entity_key: "slack-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "shopify-integration"})
MATCH (el:EntityL10n {entity_key: "shopify-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "woocommerce-integration"})
MATCH (el:EntityL10n {entity_key: "woocommerce-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "wordpress-integration"})
MATCH (el:EntityL10n {entity_key: "wordpress-integration", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "quiet-zone"})
MATCH (el:EntityL10n {entity_key: "quiet-zone", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "error-correction"})
MATCH (el:EntityL10n {entity_key: "error-correction", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "data-capacity"})
MATCH (el:EntityL10n {entity_key: "data-capacity", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-version"})
MATCH (el:EntityL10n {entity_key: "qr-code-version", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "encoding-mode"})
MATCH (el:EntityL10n {entity_key: "encoding-mode", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "module"})
MATCH (el:EntityL10n {entity_key: "module", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "finder-pattern"})
MATCH (el:EntityL10n {entity_key: "finder-pattern", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "timing-pattern"})
MATCH (el:EntityL10n {entity_key: "timing-pattern", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-messaging"})
MATCH (el:EntityL10n {entity_key: "qr-code-messaging", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-video-platform"})
MATCH (el:EntityL10n {entity_key: "qr-code-video-platform", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-professional"})
MATCH (el:EntityL10n {entity_key: "qr-code-professional", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-music-platform"})
MATCH (el:EntityL10n {entity_key: "qr-code-music-platform", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "funny-qr-codes"})
MATCH (el:EntityL10n {entity_key: "funny-qr-codes", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-tattoo"})
MATCH (el:EntityL10n {entity_key: "qr-code-tattoo", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-art-installation"})
MATCH (el:EntityL10n {entity_key: "qr-code-art-installation", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-scavenger-hunt"})
MATCH (el:EntityL10n {entity_key: "qr-code-scavenger-hunt", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-reviews"})
MATCH (el:EntityL10n {entity_key: "qr-code-reviews", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-contactless-payment"})
MATCH (el:EntityL10n {entity_key: "qr-code-contactless-payment", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-loyalty-program"})
MATCH (el:EntityL10n {entity_key: "qr-code-loyalty-program", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-product-authentication"})
MATCH (el:EntityL10n {entity_key: "qr-code-product-authentication", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-event-checkin"})
MATCH (el:EntityL10n {entity_key: "qr-code-event-checkin", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-networking"})
MATCH (el:EntityL10n {entity_key: "qr-code-networking", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-wedding"})
MATCH (el:EntityL10n {entity_key: "qr-code-wedding", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-museum-exhibit"})
MATCH (el:EntityL10n {entity_key: "qr-code-museum-exhibit", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "how-to-create-qr-code"})
MATCH (el:EntityL10n {entity_key: "how-to-create-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-design-guide"})
MATCH (el:EntityL10n {entity_key: "qr-code-design-guide", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-print-guide"})
MATCH (el:EntityL10n {entity_key: "qr-code-print-guide", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "dynamic-vs-static-guide"})
MATCH (el:EntityL10n {entity_key: "dynamic-vs-static-guide", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-marketing-guide"})
MATCH (el:EntityL10n {entity_key: "qr-code-marketing-guide", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-restaurant-guide"})
MATCH (el:EntityL10n {entity_key: "qr-code-restaurant-guide", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-business-card-guide"})
MATCH (el:EntityL10n {entity_key: "qr-code-business-card-guide", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-api-guide"})
MATCH (el:EntityL10n {entity_key: "qr-code-api-guide", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-analytics-guide"})
MATCH (el:EntityL10n {entity_key: "qr-code-analytics-guide", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-security-guide"})
MATCH (el:EntityL10n {entity_key: "qr-code-security-guide", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-vs-barcode"})
MATCH (el:EntityL10n {entity_key: "qr-code-vs-barcode", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "dynamic-vs-static-qr-code"})
MATCH (el:EntityL10n {entity_key: "dynamic-vs-static-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-vs-nfc"})
MATCH (el:EntityL10n {entity_key: "qr-code-vs-nfc", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-vs-data-matrix"})
MATCH (el:EntityL10n {entity_key: "qr-code-vs-data-matrix", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "free-vs-paid-qr-generator"})
MATCH (el:EntityL10n {entity_key: "free-vs-paid-qr-generator", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "qr-code-ai-vs-competitors"})
MATCH (el:EntityL10n {entity_key: "qr-code-ai-vs-competitors", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "short-link-vs-qr-code"})
MATCH (el:EntityL10n {entity_key: "short-link-vs-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

MATCH (e:Entity {key: "spotify-code-vs-qr-code"})
MATCH (el:EntityL10n {entity_key: "spotify-code-vs-qr-code", locale_key: "fr-FR"})
MERGE (e)-[:HAS_L10N]->(el);

// ───────────────────────────────────────────────────────────────────────────────
// Link EntityL10n to Locale via FOR_LOCALE
// ───────────────────────────────────────────────────────────────────────────────

MATCH (el:EntityL10n {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "smart-link", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "barcode", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "landing-page", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "short-link", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-style", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-content", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-frame", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "barcode-format", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "landing-page-type", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "custom-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-image", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-photo", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-with-logo", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-with-text", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-color", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-shapes", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-transparent-background", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-background", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-background-color", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-background-gradient", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-background-image", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "dynamic-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "static-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-light-mode", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-dark-mode", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-business-card", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-email-signature", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-flyer", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-poster", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-table-tent", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-packaging-label", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "link-in-bio", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "menu-restaurant", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "forms", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "announcement", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "event-rsvp", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "booking-appointment", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-url", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-vcard", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-mecard", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-text", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-email", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-sms", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-phone", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-video", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-audio", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-image-gallery", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-coupon", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-social", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-instagram", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-linkedin", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-facebook", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-twitter", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-youtube", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-tiktok", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-snapchat", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-whatsapp", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-telegram", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-pinterest", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-spotify", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-apple-music", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-soundcloud", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-payment", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-pix", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-upi", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-paypal", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-venmo", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-bitcoin", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-ethereum", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-crypto", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-bank-transfer", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-location", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-google-maps", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-apple-maps", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-waze", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-coordinates", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-app", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-app-store", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-play-store", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-app-download", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-review", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-survey", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-feedback", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-resume", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-certificate", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-ticket", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-attendance", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-pet-tag", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-medical-id", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-file", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-calendar", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "ean-13", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "ean-8", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "upc-a", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "upc-e", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "code-128", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "code-39", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "itf-14", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "codabar", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "msi-plessey", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "gs1-128", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "data-matrix", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "pdf417", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "aztec-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "maxicode", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "gs1-datamatrix", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "barcode-generator", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "barcode-scanner", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "analytics", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "click-tracking", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "scan-counting", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "geo-tracking", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "device-detection", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "time-series", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "contextual-routing", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "custom-domain-name", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "custom-link-preview", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "utm-builder", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "bulk-creation", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "team-workspaces", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "api", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "webhooks", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "white-label", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "password-protection", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "expiration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "scan-limit", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "retargeting-pixel", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-generator", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-scanner", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-api", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "landing-page-builder", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "url-shortener", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "link-in-bio-builder", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "menu-builder", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "vcard-generator", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "wifi-qr-generator", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "batch-qr-generator", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "business-cards", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "flyers", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "posters-billboards", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "brochures", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "catalogs", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "magazines", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "newspapers", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "direct-mail", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "stickers-labels", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "banners", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "product-packaging", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "product-labels", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "receipts", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "table-tents", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "menus-printed", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "tickets-physical", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "emails", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "presentations", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "documents", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "websites", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "customize-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "download-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "print-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "add-logo", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "change-colors", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "edit-destination", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "share-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "track-scans", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "shorten-url", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "create-smart-link", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "create-landing-page", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "create-barcode", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "scan-barcode", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "restaurants", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "retail", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "hospitality", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "healthcare", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "education", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "real-estate", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "fitness", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "beauty", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "entertainment", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "transportation", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "manufacturing", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "logistics", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "construction", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "finance", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "government", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "marketing-agencies", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "creative-agencies", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "event-management", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "nonprofits", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "consulting", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "developers", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "enterprise", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "agencies", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "small-business", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "freelancers", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "linkedin", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "facebook", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "twitter", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "youtube", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "tiktok", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "snapchat", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "whatsapp", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "telegram", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "pinterest", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "spotify", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "apple", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "soundcloud", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "paypal", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "venmo", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "google", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "waze", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "zapier", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "make", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "n8n", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "hubspot", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "salesforce", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "mailchimp", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "shopify", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "woocommerce", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "zapier-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "make-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "n8n-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "hubspot-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "salesforce-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "mailchimp-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "google-sheets-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "notion-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "slack-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "shopify-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "woocommerce-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "wordpress-integration", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "quiet-zone", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "error-correction", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "data-capacity", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-version", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "encoding-mode", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "module", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "finder-pattern", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "timing-pattern", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-messaging", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-video-platform", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-professional", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-music-platform", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "funny-qr-codes", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-tattoo", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-art-installation", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-scavenger-hunt", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-reviews", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-contactless-payment", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-loyalty-program", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-product-authentication", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-event-checkin", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-networking", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-wedding", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-museum-exhibit", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "how-to-create-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-design-guide", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-print-guide", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "dynamic-vs-static-guide", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-marketing-guide", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-restaurant-guide", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-business-card-guide", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-api-guide", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-analytics-guide", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-security-guide", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-vs-barcode", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "dynamic-vs-static-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-vs-nfc", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-vs-data-matrix", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "free-vs-paid-qr-generator", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "qr-code-ai-vs-competitors", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "short-link-vs-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);

MATCH (el:EntityL10n {entity_key: "spotify-code-vs-qr-code", locale_key: "fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
MERGE (el)-[:FOR_LOCALE]->(l);
