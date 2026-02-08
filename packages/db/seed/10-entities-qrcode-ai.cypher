// ============================================================================
// Entity nodes for QR Code AI (281 entities)
// Generated: 2026-02-08
// ============================================================================

// First ensure Project node exists
MERGE (proj:Project {key: 'qrcode-ai'})
ON CREATE SET proj.display_name = 'QR Code AI',
              proj.created_at = datetime();

// ============================================================================
// Entity nodes
// ============================================================================

// --- Core Products ---

MERGE (e:Entity {key: 'qr-code'})
SET e.type = 'THING',
    e.display_name = 'QR Code',
    e.description = '2D matrix barcode for encoding data',
    e.llm_context = 'USE: when discussing QR codes, scanning, 2D barcodes, quick response codes. TRIGGERS: qr, qr code, qr-code, scan code, 2d barcode, matrix code. NOT: barcode 1D (use Barcode), data matrix (use Data Matrix), link shortener without QR (use Smart Link).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'smart-link'})
SET e.type = 'THING',
    e.display_name = 'Smart Link',
    e.description = 'Intelligent shortened URL with routing rules',
    e.llm_context = 'USE: when discussing intelligent URLs, link routing, device targeting, geo-targeting links. TRIGGERS: smart link, intelligent url, routing link, conditional redirect, targeted link. NOT: basic short URL (use Short Link), QR code (use QR Code), landing page (use Landing Page).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'barcode'})
SET e.type = 'THING',
    e.display_name = 'Barcode',
    e.description = '1D linear barcode (EAN, UPC, Code 128)',
    e.llm_context = 'USE: when discussing 1D barcodes, linear barcodes, retail barcodes, product codes. TRIGGERS: barcode, 1d barcode, ean, upc, code 128, linear barcode, product code. NOT: QR code (use QR Code), 2D codes (use QR Code or Data Matrix).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'landing-page'})
SET e.type = 'THING',
    e.display_name = 'Landing Page',
    e.description = 'Destination page created via page builder',
    e.llm_context = 'USE: when discussing destination pages, page builder, no-code pages, mobile pages. TRIGGERS: landing page, destination page, page builder, mobile page, microsite. NOT: full website (external), QR code itself (use QR Code), link shortener (use Smart Link).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'short-link'})
SET e.type = 'THING',
    e.display_name = 'Short Link',
    e.description = 'Shortened URL, shared technology layer',
    e.llm_context = 'USE: when discussing URL shortening, link tracking, shortened URLs as technology. TRIGGERS: short link, shortened url, url shortener, link shortening, tiny url. NOT: smart routing (use Smart Link), QR code (use QR Code), vanity URL only (mention custom domain).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-style'})
SET e.type = 'THING',
    e.display_name = 'QR Code Style',
    e.description = 'Visual style category for QR codes',
    e.llm_context = 'USE: when discussing QR code visual approaches, style categories, design types. TRIGGERS: qr style, qr code style, visual style, design approach. NOT: specific styles (use Custom QR, QR Art, etc.), colors only (use QR Code Colors).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-content'})
SET e.type = 'THING',
    e.display_name = 'QR Code Content',
    e.description = 'Content type category for QR codes',
    e.llm_context = 'USE: when discussing QR code data types, what QR codes encode, content categories. TRIGGERS: qr content, content type, qr data, what to encode. NOT: specific content types (use URL QR, WiFi QR, etc.), QR appearance (use QR Code Style).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-frame'})
SET e.type = 'THING',
    e.display_name = 'QR Code Frame',
    e.description = 'Physical placement category for QR codes',
    e.llm_context = 'USE: when discussing QR code templates, physical placement, print templates. TRIGGERS: qr frame, template, placement, print size, frame template. NOT: specific frames (use Business Card QR, Poster QR, etc.), digital only (use Landing Page).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'barcode-format'})
SET e.type = 'THING',
    e.display_name = 'Barcode Format',
    e.description = 'Technical format category for barcodes',
    e.llm_context = 'USE: when discussing barcode standards, encoding formats, barcode types. TRIGGERS: barcode format, barcode type, barcode standard, encoding format. NOT: specific formats (use EAN-13, UPC-A, etc.), QR codes (use QR Code).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'landing-page-type'})
SET e.type = 'THING',
    e.display_name = 'Landing Page Type',
    e.description = 'Template category for landing pages',
    e.llm_context = 'USE: when discussing landing page templates, page types, use case templates. TRIGGERS: landing page type, page template, page category, template type. NOT: specific types (use Link in Bio, Digital Menu, etc.), external websites.',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'custom-qr-code'})
SET e.type = 'THING',
    e.display_name = 'Custom QR Code',
    e.description = 'Parametric QR code with user-configured elements',
    e.llm_context = 'USE: when discussing fully customizable QR codes, parametric design, manual customization. TRIGGERS: custom qr, customize qr, design qr, parametric qr, branded qr. NOT: AI-generated (use QR Code Art), photo overlay (use QR Code with Image).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-image'})
SET e.type = 'THING',
    e.display_name = 'QR Code with Image',
    e.description = 'QR code with photo/image background overlay',
    e.llm_context = 'USE: when discussing QR codes with photos, image overlays, background images on QR. TRIGGERS: qr with image, qr photo, image qr, photo background qr. NOT: AI art (use QR Code Art), logo only (use QR Code with Logo).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-art'})
SET e.type = 'THING',
    e.display_name = 'QR Code Art',
    e.description = 'AI-generated artistic QR code',
    e.llm_context = 'USE: when discussing AI QR codes, artistic QR generation, creative AI QR. TRIGGERS: qr art, ai qr, artistic qr, ai generated qr, creative qr. NOT: manual design (use Custom QR Code), photo overlay (use QR Code with Image).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-photo'})
SET e.type = 'THING',
    e.display_name = 'QR Code Photo',
    e.description = 'Alias for QR Code with Image',
    e.llm_context = 'USE: when user says \'QR photo\' specifically, redirect to QR Code with Image. TRIGGERS: qr photo, photo qr code. NOT: primary term (use QR Code with Image instead).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-with-logo'})
SET e.type = 'THING',
    e.display_name = 'QR Code with Logo',
    e.description = 'QR code featuring a centered logo',
    e.llm_context = 'USE: when discussing adding logos to QR codes, branded QR with logo, center logo. TRIGGERS: qr with logo, add logo, logo qr, branded qr, center logo. NOT: full custom design (use Custom QR Code), background image (use QR Code with Image).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-with-text'})
SET e.type = 'THING',
    e.display_name = 'QR Code with Text',
    e.description = 'QR code with call-to-action text',
    e.llm_context = 'USE: when discussing adding text to QR codes, CTA text, scan me text. TRIGGERS: qr with text, add text, scan me, call to action, text qr. NOT: encoded text content (use Text QR), logo (use QR Code with Logo).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-color'})
SET e.type = 'THING',
    e.display_name = 'QR Code Colors',
    e.description = 'Custom foreground and background colors',
    e.llm_context = 'USE: when discussing QR code colors, color customization, foreground/background. TRIGGERS: qr color, colored qr, change color, qr colors, color scheme. NOT: shapes (use QR Code Shapes), gradients specifically (use Background Gradient).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-shapes'})
SET e.type = 'THING',
    e.display_name = 'QR Code Shapes',
    e.description = 'Custom module and eye shapes',
    e.llm_context = 'USE: when discussing QR module shapes, dot patterns, eye patterns, shape customization. TRIGGERS: qr shapes, module shape, dot pattern, eye pattern, rounded qr. NOT: colors (use QR Code Colors), logo (use QR Code with Logo).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-transparent-background'})
SET e.type = 'THING',
    e.display_name = 'Transparent Background',
    e.description = 'QR code with transparent background for overlays',
    e.llm_context = 'USE: when discussing transparent QR codes, PNG with alpha, overlay QR. TRIGGERS: transparent qr, transparent background, png alpha, overlay qr, no background. NOT: white background (default), image background (use Background Image).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-background'})
SET e.type = 'THING',
    e.display_name = 'QR Code Background',
    e.description = 'Background customization category',
    e.llm_context = 'USE: when discussing QR background options generally, background customization category. TRIGGERS: qr background, background options, background type. NOT: specific types (use Background Color, Gradient, or Image).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-background-color'})
SET e.type = 'THING',
    e.display_name = 'Background Color',
    e.description = 'Solid color background',
    e.llm_context = 'USE: when discussing solid color backgrounds for QR codes. TRIGGERS: background color, solid background, fill color. NOT: gradient (use Background Gradient), image (use Background Image).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-background-gradient'})
SET e.type = 'THING',
    e.display_name = 'Background Gradient',
    e.description = 'Gradient color background',
    e.llm_context = 'USE: when discussing gradient backgrounds for QR codes. TRIGGERS: gradient background, color gradient, gradient qr. NOT: solid color (use Background Color), image (use Background Image).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-background-image'})
SET e.type = 'THING',
    e.display_name = 'Background Image',
    e.description = 'Image or pattern background',
    e.llm_context = 'USE: when discussing image backgrounds for QR codes, patterns, textures. TRIGGERS: background image, image background, pattern background, texture. NOT: QR Code with Image style (different feature), solid color (use Background Color).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'dynamic-qr-code'})
SET e.type = 'CONCEPT',
    e.display_name = 'Dynamic QR Code',
    e.description = 'Editable QR code that uses a short link',
    e.llm_context = 'USE: when discussing editable QR codes, trackable QR, changeable destination. TRIGGERS: dynamic qr, editable qr, trackable qr, change destination, update qr. NOT: fixed content (use Static QR Code), smart routing (use Smart Link).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'static-qr-code'})
SET e.type = 'CONCEPT',
    e.display_name = 'Static QR Code',
    e.description = 'Fixed QR code with data encoded directly',
    e.llm_context = 'USE: when discussing fixed QR codes, permanent QR, direct encoding. TRIGGERS: static qr, fixed qr, permanent qr, free qr, direct encode. NOT: editable (use Dynamic QR Code), trackable (use Dynamic QR Code).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-light-mode'})
SET e.type = 'CONCEPT',
    e.display_name = 'Light Mode',
    e.description = 'QR code optimized for light backgrounds',
    e.llm_context = 'USE: when discussing light theme QR codes, standard contrast QR. TRIGGERS: light mode, light theme, light background, standard qr. NOT: dark theme (use Dark Mode), inverted colors.',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-dark-mode'})
SET e.type = 'CONCEPT',
    e.display_name = 'Dark Mode',
    e.description = 'QR code optimized for dark backgrounds',
    e.llm_context = 'USE: when discussing dark theme QR codes, inverted QR, night mode. TRIGGERS: dark mode, dark theme, dark background, inverted qr, night mode. NOT: light theme (use Light Mode), standard appearance.',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-business-card'})
SET e.type = 'THING',
    e.display_name = 'Business Card QR',
    e.description = 'QR code template for business cards',
    e.llm_context = 'USE: when discussing QR codes for business cards, professional networking QR. TRIGGERS: business card qr, card qr, professional qr, networking qr. NOT: email signature (use Email Signature QR), vCard content (use vCard QR).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-email-signature'})
SET e.type = 'THING',
    e.display_name = 'Email Signature QR',
    e.description = 'QR code template for email signatures',
    e.llm_context = 'USE: when discussing QR codes for email signatures, small signature QR. TRIGGERS: email signature qr, signature qr, email qr. NOT: business card (use Business Card QR), contact form (use Forms).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-flyer'})
SET e.type = 'THING',
    e.display_name = 'Flyer QR',
    e.description = 'QR code template for flyers',
    e.llm_context = 'USE: when discussing QR codes for flyers, promotional print QR. TRIGGERS: flyer qr, handout qr, promotional qr, print qr. NOT: poster size (use Poster QR), product packaging (use Packaging Label QR).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-poster'})
SET e.type = 'THING',
    e.display_name = 'Poster QR',
    e.description = 'QR code template for posters and billboards',
    e.llm_context = 'USE: when discussing QR codes for posters, large format QR, billboard QR. TRIGGERS: poster qr, billboard qr, large qr, high resolution qr. NOT: flyer size (use Flyer QR), table display (use Table Tent QR).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-table-tent'})
SET e.type = 'THING',
    e.display_name = 'Table Tent QR',
    e.description = 'QR code template for table tents',
    e.llm_context = 'USE: when discussing QR codes for table displays, restaurant table QR. TRIGGERS: table tent qr, table qr, restaurant qr, menu qr. NOT: digital menu content (use Digital Menu), poster (use Poster QR).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-packaging-label'})
SET e.type = 'THING',
    e.display_name = 'Packaging Label QR',
    e.description = 'QR code template for product packaging',
    e.llm_context = 'USE: when discussing QR codes for product packaging, label QR, product QR. TRIGGERS: packaging qr, label qr, product qr, package qr. NOT: retail barcode (use Barcode), poster (use Poster QR).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'link-in-bio'})
SET e.type = 'THING',
    e.display_name = 'Link in Bio',
    e.description = 'Social media bio link page',
    e.llm_context = 'USE: when discussing bio link pages, social media link aggregation, creator links. TRIGGERS: link in bio, bio link, linktree alternative, social links, creator page. NOT: full landing page (use Landing Page), single URL (use Short Link).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'menu-restaurant'})
SET e.type = 'THING',
    e.display_name = 'Digital Menu',
    e.description = 'Restaurant digital menu page',
    e.llm_context = 'USE: when discussing restaurant menus, digital menus, QR menus, contactless menus. TRIGGERS: digital menu, restaurant menu, qr menu, contactless menu, menu page. NOT: table tent template (use Table Tent QR), generic landing page.',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'forms'})
SET e.type = 'THING',
    e.display_name = 'Forms',
    e.description = 'Contact and lead capture forms',
    e.llm_context = 'USE: when discussing contact forms, lead capture, form pages, data collection. TRIGGERS: form, contact form, lead form, survey, registration form. NOT: event RSVP specifically (use Event RSVP), booking (use Booking/Appointment).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'announcement'})
SET e.type = 'THING',
    e.display_name = 'Announcement',
    e.description = 'Message or announcement page',
    e.llm_context = 'USE: when discussing announcement pages, message pages, notification pages. TRIGGERS: announcement, message page, notification, alert page, info page. NOT: event invitation (use Event RSVP), ongoing content (use Link in Bio).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'event-rsvp'})
SET e.type = 'THING',
    e.display_name = 'Event RSVP',
    e.description = 'Event registration and RSVP page',
    e.llm_context = 'USE: when discussing event registration, RSVP pages, guest management. TRIGGERS: event rsvp, event registration, rsvp page, guest list, invitation. NOT: general forms (use Forms), booking slots (use Booking/Appointment).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'booking-appointment'})
SET e.type = 'THING',
    e.display_name = 'Booking/Appointment',
    e.description = 'Scheduling and booking page',
    e.llm_context = 'USE: when discussing appointment booking, scheduling pages, reservation systems. TRIGGERS: booking, appointment, schedule, reservation, calendar booking. NOT: event RSVP (use Event RSVP), contact form (use Forms).',
    e.phase = 'Core Products',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Content Types ---

MERGE (e:Entity {key: 'qr-code-url'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'URL QR Code',
    e.description = 'QR code that redirects to any URL',
    e.llm_context = 'USE: when discussing QR codes linking to websites, URL encoding. TRIGGERS: url qr, website qr, link qr, web qr. NOT: specific platforms (use Instagram QR, YouTube QR, etc.), WiFi (use WiFi QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-wifi'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'WiFi QR Code',
    e.description = 'QR code with WiFi credentials (SSID, password, encryption)',
    e.llm_context = 'USE: when discussing WiFi sharing, network credentials in QR. TRIGGERS: wifi qr, wireless qr, network qr, wifi password qr. NOT: hotspot login page (use URL QR), Bluetooth.',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-vcard'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'vCard QR Code',
    e.description = 'Digital business card in vCard format',
    e.llm_context = 'USE: when discussing digital business cards, contact sharing QR. TRIGGERS: vcard qr, contact qr, business card qr, digital card. NOT: MeCard (use MeCard QR), LinkedIn profile (use LinkedIn QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-mecard'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'MeCard QR Code',
    e.description = 'Japanese contact format (more compact than vCard)',
    e.llm_context = 'USE: when discussing Japanese contact format, compact contact QR. TRIGGERS: mecard, mecard qr, japanese contact qr. NOT: vCard (use vCard QR), standard contact.',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-pdf'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'PDF QR Code',
    e.description = 'Links to PDF document for viewing or download',
    e.llm_context = 'USE: when discussing PDF links, document QR codes. TRIGGERS: pdf qr, document qr, brochure qr, manual qr. NOT: generic file (use File Download QR), image gallery (use Image Gallery QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-text'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Text QR Code',
    e.description = 'Plain text content encoded directly',
    e.llm_context = 'USE: when discussing plain text encoding, static text in QR. TRIGGERS: text qr, plain text qr, message qr. NOT: URL (use URL QR), email (use Email QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-email'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Email QR Code',
    e.description = 'Opens email composer with prefilled fields',
    e.llm_context = 'USE: when discussing email QR codes, mailto links in QR. TRIGGERS: email qr, mailto qr, compose email qr. NOT: SMS (use SMS QR), contact (use vCard QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-sms'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'SMS QR Code',
    e.description = 'Opens SMS with prefilled phone number and message',
    e.llm_context = 'USE: when discussing SMS QR codes, text message QR. TRIGGERS: sms qr, text message qr, message qr. NOT: WhatsApp (use WhatsApp QR), phone call (use Phone QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-phone'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Phone QR Code',
    e.description = 'Phone number for direct calling',
    e.llm_context = 'USE: when discussing phone call QR codes, tel: links. TRIGGERS: phone qr, call qr, tel qr, phone number qr. NOT: SMS (use SMS QR), WhatsApp (use WhatsApp QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-video'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Video QR Code',
    e.description = 'Links to video content (YouTube, Vimeo, hosted)',
    e.llm_context = 'USE: when discussing video link QR codes, video sharing. TRIGGERS: video qr, youtube qr link, vimeo qr, video link. NOT: YouTube channel (use YouTube QR), audio (use Audio QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-audio'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Audio QR Code',
    e.description = 'Links to audio or podcast content',
    e.llm_context = 'USE: when discussing audio link QR codes, podcast QR. TRIGGERS: audio qr, podcast qr, music link qr, voice message qr. NOT: Spotify profile (use Spotify QR), video (use Video QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-image-gallery'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Image Gallery QR Code',
    e.description = 'Links to image gallery or photo album',
    e.llm_context = 'USE: when discussing photo album QR, image collection links. TRIGGERS: gallery qr, photo album qr, image gallery qr, photos qr. NOT: single image, PDF (use PDF QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-coupon'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Coupon QR Code',
    e.description = 'Digital coupon or discount code',
    e.llm_context = 'USE: when discussing discount QR codes, promotional codes. TRIGGERS: coupon qr, discount qr, promo qr, deal qr. NOT: payment (use Payment QR), ticket (use Ticket QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-social'})
SET e.type = 'THING',
    e.display_name = 'Social Media QR Code',
    e.description = 'Category for social media platform QR codes',
    e.llm_context = 'USE: when discussing social media QR codes generally. TRIGGERS: social qr, social media qr, social link qr. NOT: specific platforms (use Instagram QR, LinkedIn QR, etc.).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-instagram'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Instagram QR Code',
    e.description = 'Links to Instagram profile or post',
    e.llm_context = 'USE: when discussing Instagram profile QR, Instagram links. TRIGGERS: instagram qr, ig qr, insta qr. NOT: other social (use Facebook QR, TikTok QR, etc.), generic social (use Social Media QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-linkedin'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'LinkedIn QR Code',
    e.description = 'Links to LinkedIn profile or company page',
    e.llm_context = 'USE: when discussing LinkedIn profile QR, professional networking QR. TRIGGERS: linkedin qr, professional qr, company page qr. NOT: vCard (use vCard QR), business card frame (use Business Card QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-facebook'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Facebook QR Code',
    e.description = 'Links to Facebook page or profile',
    e.llm_context = 'USE: when discussing Facebook page QR, Facebook profile links. TRIGGERS: facebook qr, fb qr, facebook page qr. NOT: Instagram (use Instagram QR), WhatsApp (use WhatsApp QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-twitter'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Twitter/X QR Code',
    e.description = 'Links to Twitter/X profile or tweet',
    e.llm_context = 'USE: when discussing Twitter profile QR, X platform links. TRIGGERS: twitter qr, x qr, tweet qr. NOT: other social platforms.',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-youtube'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'YouTube QR Code',
    e.description = 'Links to YouTube channel or video',
    e.llm_context = 'USE: when discussing YouTube channel QR, YouTube video links. TRIGGERS: youtube qr, channel qr, yt qr. NOT: generic video (use Video QR), TikTok (use TikTok QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-tiktok'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'TikTok QR Code',
    e.description = 'Links to TikTok profile or video',
    e.llm_context = 'USE: when discussing TikTok profile QR, TikTok video links. TRIGGERS: tiktok qr, tt qr. NOT: Instagram Reels (use Instagram QR), YouTube Shorts (use YouTube QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-snapchat'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Snapchat QR Code',
    e.description = 'Links to Snapchat profile',
    e.llm_context = 'USE: when discussing Snapchat profile QR, Snapcode alternatives. TRIGGERS: snapchat qr, snap qr, snapcode. NOT: Instagram Stories (use Instagram QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-whatsapp'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'WhatsApp QR Code',
    e.description = 'Opens WhatsApp chat with prefilled message',
    e.llm_context = 'USE: when discussing WhatsApp chat QR, wa.me links. TRIGGERS: whatsapp qr, wa qr, whatsapp chat qr. NOT: SMS (use SMS QR), Telegram (use Telegram QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-telegram'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Telegram QR Code',
    e.description = 'Links to Telegram channel, group, or bot',
    e.llm_context = 'USE: when discussing Telegram channel QR, Telegram links. TRIGGERS: telegram qr, tg qr, telegram channel qr, telegram bot qr. NOT: WhatsApp (use WhatsApp QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-pinterest'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Pinterest QR Code',
    e.description = 'Links to Pinterest profile or board',
    e.llm_context = 'USE: when discussing Pinterest profile QR, Pinterest board links. TRIGGERS: pinterest qr, pin qr, board qr. NOT: Instagram (use Instagram QR), image gallery (use Image Gallery QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-spotify'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Spotify QR Code',
    e.description = 'Links to Spotify artist, album, or playlist',
    e.llm_context = 'USE: when discussing Spotify link QR, music sharing QR. TRIGGERS: spotify qr, playlist qr, music qr. NOT: Apple Music (use Apple Music QR), SoundCloud (use SoundCloud QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-apple-music'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Apple Music QR Code',
    e.description = 'Links to Apple Music artist, album, or playlist',
    e.llm_context = 'USE: when discussing Apple Music link QR. TRIGGERS: apple music qr, itunes qr. NOT: Spotify (use Spotify QR), generic audio (use Audio QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-soundcloud'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'SoundCloud QR Code',
    e.description = 'Links to SoundCloud artist or track',
    e.llm_context = 'USE: when discussing SoundCloud link QR, indie music QR. TRIGGERS: soundcloud qr, sc qr. NOT: Spotify (use Spotify QR), generic audio (use Audio QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-payment'})
SET e.type = 'THING',
    e.display_name = 'Payment QR Code',
    e.description = 'Category for payment-related QR codes',
    e.llm_context = 'USE: when discussing payment QR codes generally. TRIGGERS: payment qr, pay qr, money qr. NOT: specific systems (use PIX QR, UPI QR, PayPal QR, etc.).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-pix'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'PIX QR Code',
    e.description = 'Brazil instant payment system',
    e.llm_context = 'USE: when discussing Brazilian payments, PIX system. TRIGGERS: pix qr, brazil payment qr, pix code. NOT: UPI India (use UPI QR), generic payment.',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-upi'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'UPI QR Code',
    e.description = 'India Unified Payments Interface',
    e.llm_context = 'USE: when discussing Indian payments, UPI system. TRIGGERS: upi qr, india payment qr, bharat qr. NOT: PIX Brazil (use PIX QR), generic payment.',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-paypal'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'PayPal QR Code',
    e.description = 'PayPal payment or money request',
    e.llm_context = 'USE: when discussing PayPal payments, PayPal.me links. TRIGGERS: paypal qr, paypal.me qr. NOT: Venmo (use Venmo QR), bank transfer (use Bank Transfer QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-venmo'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Venmo QR Code',
    e.description = 'Venmo payment (US only)',
    e.llm_context = 'USE: when discussing Venmo payments, US P2P payments. TRIGGERS: venmo qr, venmo code. NOT: PayPal (use PayPal QR), Cash App.',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-bitcoin'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Bitcoin QR Code',
    e.description = 'Bitcoin wallet address for payments',
    e.llm_context = 'USE: when discussing Bitcoin payments, BTC address QR. TRIGGERS: bitcoin qr, btc qr, crypto wallet qr. NOT: Ethereum (use Ethereum QR), generic crypto (use Crypto QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-ethereum'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Ethereum QR Code',
    e.description = 'Ethereum wallet address for payments',
    e.llm_context = 'USE: when discussing Ethereum payments, ETH address QR. TRIGGERS: ethereum qr, eth qr, erc20 qr. NOT: Bitcoin (use Bitcoin QR), generic crypto (use Crypto QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-crypto'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Crypto QR Code',
    e.description = 'Generic cryptocurrency payment',
    e.llm_context = 'USE: when discussing generic crypto payments, multi-coin wallets. TRIGGERS: crypto qr, cryptocurrency qr, multi-coin qr. NOT: specific coins (use Bitcoin QR, Ethereum QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-bank-transfer'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Bank Transfer QR Code',
    e.description = 'SEPA or domestic bank transfer',
    e.llm_context = 'USE: when discussing bank transfer QR, IBAN encoding. TRIGGERS: bank transfer qr, sepa qr, iban qr, wire transfer qr. NOT: PIX (use PIX QR), UPI (use UPI QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-location'})
SET e.type = 'THING',
    e.display_name = 'Location QR Code',
    e.description = 'Category for location and maps QR codes',
    e.llm_context = 'USE: when discussing location QR codes generally, maps category. TRIGGERS: location qr, maps qr, navigation qr. NOT: specific apps (use Google Maps QR, Apple Maps QR, Waze QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-google-maps'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Google Maps QR Code',
    e.description = 'Opens location in Google Maps',
    e.llm_context = 'USE: when discussing Google Maps links, Google location QR. TRIGGERS: google maps qr, gmaps qr, google location qr. NOT: Apple Maps (use Apple Maps QR), Waze (use Waze QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-apple-maps'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Apple Maps QR Code',
    e.description = 'Opens location in Apple Maps',
    e.llm_context = 'USE: when discussing Apple Maps links, iOS maps QR. TRIGGERS: apple maps qr, ios maps qr. NOT: Google Maps (use Google Maps QR), Waze (use Waze QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-waze'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Waze QR Code',
    e.description = 'Opens navigation in Waze',
    e.llm_context = 'USE: when discussing Waze navigation links. TRIGGERS: waze qr, waze navigation qr. NOT: Google Maps (use Google Maps QR), Apple Maps (use Apple Maps QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-coordinates'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Coordinates QR Code',
    e.description = 'Raw GPS coordinates (geo: URI)',
    e.llm_context = 'USE: when discussing raw GPS encoding, geo: URI format. TRIGGERS: coordinates qr, gps qr, geo qr, lat long qr. NOT: specific map apps (use Google Maps QR, Apple Maps QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-app'})
SET e.type = 'THING',
    e.display_name = 'App Download QR Code',
    e.description = 'Category for app store download QR codes',
    e.llm_context = 'USE: when discussing app download QR codes generally. TRIGGERS: app qr, download app qr, app store qr. NOT: specific stores (use App Store QR, Play Store QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-app-store'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'App Store QR Code',
    e.description = 'iOS App Store download link',
    e.llm_context = 'USE: when discussing iOS app downloads, Apple App Store links. TRIGGERS: app store qr, ios app qr, apple app qr. NOT: Play Store (use Play Store QR), smart link (use Smart App Download QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-play-store'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Play Store QR Code',
    e.description = 'Google Play Store download link',
    e.llm_context = 'USE: when discussing Android app downloads, Google Play links. TRIGGERS: play store qr, android app qr, google play qr. NOT: App Store (use App Store QR), smart link (use Smart App Download QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-app-download'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Smart App Download QR Code',
    e.description = 'Auto-detects OS and redirects to correct store',
    e.llm_context = 'USE: when discussing smart app links, cross-platform app download. TRIGGERS: smart app qr, universal app link, cross platform app qr. NOT: specific stores (use App Store QR, Play Store QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-review'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Review QR Code',
    e.description = 'Links to Google, TripAdvisor, or other review platform',
    e.llm_context = 'USE: when discussing review collection QR, Google review links. TRIGGERS: review qr, google review qr, tripadvisor qr, yelp qr. NOT: feedback form (use Feedback QR), survey (use Survey QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-survey'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Survey QR Code',
    e.description = 'Links to online survey or form',
    e.llm_context = 'USE: when discussing survey QR codes, questionnaire links. TRIGGERS: survey qr, questionnaire qr, typeform qr, google form qr. NOT: simple feedback (use Feedback QR), review (use Review QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-feedback'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Feedback QR Code',
    e.description = 'Links to feedback collection form',
    e.llm_context = 'USE: when discussing simple feedback collection, rating QR. TRIGGERS: feedback qr, rating qr, quick feedback qr. NOT: full survey (use Survey QR), review platform (use Review QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-menu'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Menu QR Code',
    e.description = 'Restaurant or cafe digital menu',
    e.llm_context = 'USE: when discussing restaurant menu QR, contactless menu. TRIGGERS: menu qr, restaurant qr, cafe qr, food menu qr. NOT: Digital Menu landing page (use Digital Menu), table tent frame (use Table Tent QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-resume'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Resume QR Code',
    e.description = 'Links to digital resume or CV',
    e.llm_context = 'USE: when discussing resume QR, CV links, portfolio QR. TRIGGERS: resume qr, cv qr, portfolio qr. NOT: LinkedIn profile (use LinkedIn QR), vCard (use vCard QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-certificate'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Certificate QR Code',
    e.description = 'Verifies authenticity of certificates and credentials',
    e.llm_context = 'USE: when discussing certificate verification, credential QR. TRIGGERS: certificate qr, diploma qr, credential qr, verification qr. NOT: ticket (use Ticket QR), ID badge.',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-ticket'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Ticket QR Code',
    e.description = 'Event or transport ticket',
    e.llm_context = 'USE: when discussing event tickets, transport tickets, entry QR. TRIGGERS: ticket qr, event ticket qr, boarding pass qr, concert ticket qr. NOT: attendance check-in (use Attendance QR), coupon (use Coupon QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-attendance'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Attendance QR Code',
    e.description = 'Check-in for events, classes, or meetings',
    e.llm_context = 'USE: when discussing check-in QR, attendance tracking. TRIGGERS: attendance qr, check-in qr, sign-in qr, class attendance qr. NOT: event ticket (use Ticket QR), access badge.',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-pet-tag'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Pet Tag QR Code',
    e.description = 'Pet identification with owner contact info',
    e.llm_context = 'USE: when discussing pet ID QR, lost pet tags. TRIGGERS: pet tag qr, pet id qr, dog tag qr, cat tag qr. NOT: medical ID (use Medical ID QR), vCard (use vCard QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-medical-id'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Medical ID QR Code',
    e.description = 'Emergency medical information',
    e.llm_context = 'USE: when discussing medical alert QR, health info QR. TRIGGERS: medical id qr, health qr, emergency info qr, medical alert qr. NOT: pet tag (use Pet Tag QR), certificate (use Certificate QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-file'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'File Download QR Code',
    e.description = 'Links to any downloadable file',
    e.llm_context = 'USE: when discussing file download QR, document download links. TRIGGERS: file qr, download qr, zip qr, document download qr. NOT: PDF specifically (use PDF QR), image gallery (use Image Gallery QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-calendar'})
SET e.type = 'CONTENT_TYPE',
    e.display_name = 'Calendar Event QR Code',
    e.description = 'Adds event to calendar',
    e.llm_context = 'USE: when discussing calendar event QR, iCal links. TRIGGERS: calendar qr, event qr, ical qr, add to calendar qr. NOT: event RSVP page (use Event RSVP), ticket (use Ticket QR).',
    e.phase = 'Content Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Barcode Types ---

MERGE (e:Entity {key: 'ean-13'})
SET e.type = 'THING',
    e.display_name = 'EAN-13',
    e.description = 'European Article Number, 13 digits for retail products',
    e.llm_context = 'USE: when discussing EAN-13 barcodes, European retail product identification, 13-digit barcodes, or GTIN-13 standard. TRIGGERS: ean-13, ean13, european article number, 13-digit barcode, gtin-13, retail barcode europe. NOT: EAN-8 (compact version), UPC-A (North American), ISBN (books).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'ean-8'})
SET e.type = 'THING',
    e.display_name = 'EAN-8',
    e.description = 'Compact 8-digit barcode for small products',
    e.llm_context = 'USE: when discussing EAN-8 barcodes, compact retail barcodes, 8-digit product codes, or small product labeling. TRIGGERS: ean-8, ean8, 8-digit barcode, compact barcode, small product barcode. NOT: EAN-13 (full version), UPC-E (North American compact).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'upc-a'})
SET e.type = 'THING',
    e.display_name = 'UPC-A',
    e.description = 'Universal Product Code, 12 digits for US/Canada retail',
    e.llm_context = 'USE: when discussing UPC-A barcodes, North American retail product codes, 12-digit barcodes, or US/Canada product identification. TRIGGERS: upc-a, upca, universal product code, 12-digit barcode, us barcode, canada barcode, gtin-12. NOT: UPC-E (compressed), EAN-13 (European).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'upc-e'})
SET e.type = 'THING',
    e.display_name = 'UPC-E',
    e.description = 'Compressed 6-digit UPC for small packages',
    e.llm_context = 'USE: when discussing UPC-E barcodes, compressed product codes, 6-digit barcodes, or small package identification in North America. TRIGGERS: upc-e, upce, 6-digit barcode, compressed upc, zero-suppressed barcode. NOT: UPC-A (full version), EAN-8 (European compact).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'code-128'})
SET e.type = 'THING',
    e.display_name = 'Code 128',
    e.description = 'High-density alphanumeric barcode for logistics',
    e.llm_context = 'USE: when discussing Code 128 barcodes, logistics barcodes, shipping labels, high-density alphanumeric encoding, or ASCII barcodes. TRIGGERS: code 128, code128, logistics barcode, shipping barcode, alphanumeric barcode, ascii barcode. NOT: Code 39 (simpler), GS1-128 (with application identifiers).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'code-39'})
SET e.type = 'THING',
    e.display_name = 'Code 39',
    e.description = 'Full alphanumeric barcode for industrial use',
    e.llm_context = 'USE: when discussing Code 39 barcodes, industrial barcodes, automotive parts labeling, or self-checking alphanumeric codes. TRIGGERS: code 39, code39, code 3 of 9, industrial barcode, automotive barcode, defense barcode. NOT: Code 128 (higher density), Codabar (numeric).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'itf-14'})
SET e.type = 'THING',
    e.display_name = 'ITF-14',
    e.description = 'Interleaved 2 of 5 for shipping cartons',
    e.llm_context = 'USE: when discussing ITF-14 barcodes, shipping carton barcodes, pallet labeling, or GTIN-14 encoding. TRIGGERS: itf-14, itf14, interleaved 2 of 5, carton barcode, pallet barcode, gtin-14, case barcode. NOT: EAN-13 (retail), GS1-128 (with dates/lots).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'codabar'})
SET e.type = 'THING',
    e.display_name = 'Codabar',
    e.description = 'Numeric barcode for libraries and blood banks',
    e.llm_context = 'USE: when discussing Codabar barcodes, library barcodes, blood bank barcodes, or legacy numeric codes. TRIGGERS: codabar, library barcode, blood bank barcode, fedex barcode, photo lab barcode. NOT: Code 39 (alphanumeric), Code 128 (modern logistics).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'msi-plessey'})
SET e.type = 'THING',
    e.display_name = 'MSI Plessey',
    e.description = 'Inventory control barcode with check digits',
    e.llm_context = 'USE: when discussing MSI Plessey barcodes, warehouse inventory barcodes, grocery store shelf labeling, or check-digit numeric codes. TRIGGERS: msi plessey, msi barcode, plessey barcode, inventory barcode, warehouse barcode, shelf barcode. NOT: Code 128 (modern), ITF-14 (shipping).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'gs1-128'})
SET e.type = 'THING',
    e.display_name = 'GS1-128',
    e.description = 'Supply chain barcode with Application Identifiers',
    e.llm_context = 'USE: when discussing GS1-128 barcodes, supply chain traceability, batch/lot tracking, expiration dates on barcodes, or application identifiers. TRIGGERS: gs1-128, gs1128, ean-128, ucc-128, application identifier, batch barcode, lot barcode, expiry barcode. NOT: Code 128 (without AI), ITF-14 (simpler).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'data-matrix'})
SET e.type = 'THING',
    e.display_name = 'Data Matrix',
    e.description = '2D code for electronics and pharmaceutical marking',
    e.llm_context = 'USE: when discussing Data Matrix codes, electronics component marking, pharmaceutical serialization, or small 2D codes for industrial use. TRIGGERS: data matrix, datamatrix, ecc200, electronics marking, pharma barcode, component marking, small 2d code. NOT: QR code (consumer), GS1 DataMatrix (with identifiers).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'pdf417'})
SET e.type = 'THING',
    e.display_name = 'PDF417',
    e.description = 'Stacked 2D barcode for IDs and tickets',
    e.llm_context = 'USE: when discussing PDF417 codes, driver\'s license barcodes, boarding passes, ID cards, or stacked 2D barcodes. TRIGGERS: pdf417, pdf 417, driver license barcode, id barcode, boarding pass barcode, stacked barcode. NOT: QR code (square), Aztec (no quiet zone).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'aztec-code'})
SET e.type = 'THING',
    e.display_name = 'Aztec Code',
    e.description = '2D code for tickets and transport',
    e.llm_context = 'USE: when discussing Aztec codes, airline boarding passes, train tickets, transport tickets, or 2D codes without quiet zone. TRIGGERS: aztec code, aztec barcode, boarding pass code, train ticket barcode, transport barcode, no quiet zone barcode. NOT: QR code (needs quiet zone), PDF417 (rectangular).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'maxicode'})
SET e.type = 'THING',
    e.display_name = 'MaxiCode',
    e.description = 'Fixed-size 2D code for high-speed package sorting',
    e.llm_context = 'USE: when discussing MaxiCode, UPS package tracking, high-speed conveyor scanning, or hexagonal 2D codes. TRIGGERS: maxicode, ups barcode, package sorting code, hexagonal barcode, conveyor barcode. NOT: QR code (square), Data Matrix (small items).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'gs1-datamatrix'})
SET e.type = 'THING',
    e.display_name = 'GS1 DataMatrix',
    e.description = 'Data Matrix with GS1 Application Identifiers',
    e.llm_context = 'USE: when discussing GS1 DataMatrix, pharmaceutical serialization, medical device UDI, food traceability, or regulated 2D codes. TRIGGERS: gs1 datamatrix, gs1 data matrix, pharma serialization, udi barcode, fmd barcode, medical device barcode, food traceability code. NOT: plain Data Matrix (no AI), QR code (consumer).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'barcode-generator'})
SET e.type = 'TOOL',
    e.display_name = 'Barcode Generator',
    e.description = 'Tool to create barcode images',
    e.llm_context = 'USE: when discussing barcode creation tools, generating EAN/UPC/Code 128 images, or barcode image software. TRIGGERS: barcode generator, create barcode, generate barcode, barcode maker, barcode image, ean generator, upc generator. NOT: QR code generator (2D square), barcode scanner (reading).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'barcode-scanner'})
SET e.type = 'TOOL',
    e.display_name = 'Barcode Scanner',
    e.description = 'Tool to read barcode data',
    e.llm_context = 'USE: when discussing barcode reading tools, scanning EAN/UPC codes, barcode reader apps, or point-of-sale scanning. TRIGGERS: barcode scanner, barcode reader, scan barcode, pos scanner, barcode app, read barcode. NOT: QR code scanner (2D), barcode generator (creation).',
    e.phase = 'Barcode Types',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Features & Tools ---

MERGE (e:Entity {key: 'analytics'})
SET e.type = 'FEATURE',
    e.display_name = 'Analytics',
    e.description = 'Umbrella feature for all tracking and statistics',
    e.llm_context = 'USE: when discussing QR code analytics, scan tracking, click statistics, or performance metrics. TRIGGERS: analytics, statistics, tracking, metrics, reports, data, insights, performance. NOT: specific analytics types (use click-tracking, geo-tracking, etc.).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'click-tracking'})
SET e.type = 'FEATURE',
    e.display_name = 'Click Tracking',
    e.description = 'Tracks link click events',
    e.llm_context = 'USE: when discussing link click tracking, click events, referrer data, or conversion tracking. TRIGGERS: click tracking, track clicks, click events, link clicks, click data, referrer tracking. NOT: scan counting (QR specific), analytics (umbrella term).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'scan-counting'})
SET e.type = 'FEATURE',
    e.display_name = 'Scan Counting',
    e.description = 'Counts QR code scans',
    e.llm_context = 'USE: when discussing QR code scan counts, scan statistics, or scan volume metrics. TRIGGERS: scan counting, count scans, scan stats, scan volume, scan numbers, how many scans. NOT: click tracking (links), geo-tracking (location).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'geo-tracking'})
SET e.type = 'FEATURE',
    e.display_name = 'Geographic Tracking',
    e.description = 'Geographic location of scans',
    e.llm_context = 'USE: when discussing geographic tracking, location data, country/city analytics, or IP geolocation for scans. TRIGGERS: geo tracking, geographic, location tracking, country data, city data, ip location, where scanned. NOT: device detection (what device), time-series (when).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'device-detection'})
SET e.type = 'FEATURE',
    e.display_name = 'Device Detection',
    e.description = 'OS/browser/device information',
    e.llm_context = 'USE: when discussing device detection, OS tracking, browser detection, or mobile vs desktop analytics. TRIGGERS: device detection, os detection, browser detection, device type, mobile or desktop, what device, user agent. NOT: geo-tracking (location), contextual routing (redirect).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'time-series'})
SET e.type = 'FEATURE',
    e.display_name = 'Time Series Analytics',
    e.description = 'Historical data over time',
    e.llm_context = 'USE: when discussing time-series analytics, historical scan data, trend analysis, or scans over time. TRIGGERS: time series, historical data, trends, over time, date range, peak times, daily scans, weekly stats. NOT: real-time (immediate), analytics (umbrella).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'contextual-routing'})
SET e.type = 'FEATURE',
    e.display_name = 'Contextual Routing',
    e.description = 'OS/device/location-based redirect',
    e.llm_context = 'USE: when discussing contextual routing, device-based redirects, OS-specific destinations, or smart link routing rules. TRIGGERS: contextual routing, smart routing, device redirect, os redirect, app store redirect, conditional redirect, dynamic destination. NOT: device detection (analytics only), edit destination (manual change).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'custom-domain-name'})
SET e.type = 'FEATURE',
    e.display_name = 'Custom Domain',
    e.description = 'Branded short domains',
    e.llm_context = 'USE: when discussing custom domains, branded short links, white-label URLs, or vanity domains. TRIGGERS: custom domain, branded domain, own domain, vanity url, white label domain, custom short url. NOT: url shortener (action), white label (full branding).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'custom-link-preview'})
SET e.type = 'FEATURE',
    e.display_name = 'Custom Link Preview',
    e.description = 'OG meta customization',
    e.llm_context = 'USE: when discussing custom link previews, Open Graph meta tags, social media previews, or thumbnail customization. TRIGGERS: link preview, og tags, open graph, social preview, thumbnail, share preview, meta tags. NOT: landing page (full page), custom domain (url).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'utm-builder'})
SET e.type = 'FEATURE',
    e.display_name = 'UTM Builder',
    e.description = 'Campaign parameter builder',
    e.llm_context = 'USE: when discussing UTM parameters, campaign tracking, Google Analytics parameters, or marketing attribution. TRIGGERS: utm builder, utm parameters, campaign tracking, utm source, utm medium, utm campaign, google analytics tracking. NOT: analytics (viewing data), retargeting (ads).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'bulk-creation'})
SET e.type = 'FEATURE',
    e.display_name = 'Bulk Creation',
    e.description = 'Mass QR code generation',
    e.llm_context = 'USE: when discussing bulk QR code creation, mass generation, spreadsheet import, or enterprise-scale QR codes. TRIGGERS: bulk creation, bulk generate, mass create, batch qr, spreadsheet import, csv upload, multiple qr codes at once. NOT: batch-qr-generator (tool), api (programmatic).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'team-workspaces'})
SET e.type = 'FEATURE',
    e.display_name = 'Team Workspaces',
    e.description = 'Multi-user collaboration',
    e.llm_context = 'USE: when discussing team collaboration, multi-user access, shared QR code management, or role-based permissions. TRIGGERS: team workspace, collaboration, multi-user, shared access, team members, roles, permissions, organization. NOT: white-label (branding), api (integration).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'api'})
SET e.type = 'FEATURE',
    e.display_name = 'API Access',
    e.description = 'Developer API access',
    e.llm_context = 'USE: when discussing API access, developer integration, programmatic QR code creation, or RESTful endpoints. TRIGGERS: api, api access, developer api, rest api, integration, programmatic, endpoints. NOT: webhooks (events), qr-code-api (specific tool).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'webhooks'})
SET e.type = 'FEATURE',
    e.display_name = 'Webhooks',
    e.description = 'Event notifications',
    e.llm_context = 'USE: when discussing webhooks, event notifications, real-time callbacks, or scan event triggers. TRIGGERS: webhooks, webhook, event notification, callback, trigger, real-time event, scan webhook. NOT: api (request/response), analytics (viewing).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'white-label'})
SET e.type = 'FEATURE',
    e.display_name = 'White Label',
    e.description = 'Remove platform branding',
    e.llm_context = 'USE: when discussing white-label solutions, removing platform branding, reseller programs, or agency branding. TRIGGERS: white label, whitelabel, remove branding, no branding, agency solution, reseller, own branding. NOT: custom domain (url only), team workspaces (collaboration).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'password-protection'})
SET e.type = 'FEATURE',
    e.display_name = 'Password Protection',
    e.description = 'Require password to access',
    e.llm_context = 'USE: when discussing password-protected QR codes, gated content, secure access, or password-required links. TRIGGERS: password protection, password protected, require password, gated content, secure qr, locked qr, access code. NOT: expiration (time limit), scan limit (count limit).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'expiration'})
SET e.type = 'FEATURE',
    e.display_name = 'Link Expiration',
    e.description = 'Time-limited QR codes',
    e.llm_context = 'USE: when discussing link expiration, time-limited QR codes, expiry dates, or temporary access. TRIGGERS: expiration, expire, time limit, temporary qr, expiry date, auto-disable, limited time. NOT: scan limit (count), password protection (access control).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'scan-limit'})
SET e.type = 'FEATURE',
    e.display_name = 'Scan Limit',
    e.description = 'Maximum scan count',
    e.llm_context = 'USE: when discussing scan limits, maximum scans, limited-use QR codes, or scan quotas. TRIGGERS: scan limit, max scans, limited scans, scan quota, one-time scan, single use, limited use. NOT: expiration (time), password protection (access).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'retargeting-pixel'})
SET e.type = 'FEATURE',
    e.display_name = 'Retargeting Pixel',
    e.description = 'Facebook/Google pixel integration',
    e.llm_context = 'USE: when discussing retargeting pixels, Facebook pixel, Google pixel, remarketing, or ad tracking integration. TRIGGERS: retargeting pixel, facebook pixel, google pixel, remarketing, ad tracking, pixel integration, audience building. NOT: utm-builder (attribution), analytics (internal).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-generator'})
SET e.type = 'TOOL',
    e.display_name = 'QR Code Generator',
    e.description = 'Primary QR code creation tool',
    e.llm_context = 'USE: when discussing QR code creation tools, generating QR codes, or QR maker software. TRIGGERS: qr code generator, qr generator, qr maker, create qr, generate qr, make qr code, qr code creator. NOT: barcode generator (1D), qr code scanner (reading), qr code api (programmatic).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-scanner'})
SET e.type = 'TOOL',
    e.display_name = 'QR Code Scanner',
    e.description = 'QR code reader application',
    e.llm_context = 'USE: when discussing QR code scanning, reading QR codes, QR reader apps, or camera-based scanning. TRIGGERS: qr code scanner, qr scanner, qr reader, scan qr, read qr code, qr code app, camera scan. NOT: barcode scanner (1D), qr code generator (creation).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-api'})
SET e.type = 'TOOL',
    e.display_name = 'QR Code API',
    e.description = 'API for QR generation',
    e.llm_context = 'USE: when discussing QR code APIs, programmatic QR generation, developer QR tools, or REST QR endpoints. TRIGGERS: qr code api, qr api, programmatic qr, developer qr, rest qr, api qr generation. NOT: api access (general feature), qr code generator (ui tool).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'landing-page-builder'})
SET e.type = 'TOOL',
    e.display_name = 'Landing Page Builder',
    e.description = 'No-code page builder',
    e.llm_context = 'USE: when discussing landing page builders, no-code page creation, drag-and-drop page editors, or destination page tools. TRIGGERS: landing page builder, page builder, no-code page, drag and drop, page editor, bio page builder. NOT: link-in-bio builder (social specific), menu builder (restaurant).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'url-shortener'})
SET e.type = 'TOOL',
    e.display_name = 'URL Shortener',
    e.description = 'Link shortening tool',
    e.llm_context = 'USE: when discussing URL shorteners, link shortening, creating short links, or compact URLs. TRIGGERS: url shortener, link shortener, short url, short link, shorten url, compact link, tiny url. NOT: smart link (intelligent routing), custom domain (branded).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'link-in-bio-builder'})
SET e.type = 'TOOL',
    e.display_name = 'Link in Bio Builder',
    e.description = 'Social bio page builder',
    e.llm_context = 'USE: when discussing link-in-bio pages, Instagram bio links, social media link hubs, or bio page tools. TRIGGERS: link in bio, bio link, instagram bio, tiktok bio, linktree alternative, bio page, social link hub. NOT: landing page builder (general), menu builder (restaurant).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'menu-builder'})
SET e.type = 'TOOL',
    e.display_name = 'Menu Builder',
    e.description = 'Restaurant menu builder',
    e.llm_context = 'USE: when discussing digital menu builders, restaurant menu creation, or QR menu tools. TRIGGERS: menu builder, restaurant menu, digital menu, qr menu, menu creator, food menu builder, cafe menu. NOT: landing page builder (general), link-in-bio builder (social).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'vcard-generator'})
SET e.type = 'TOOL',
    e.display_name = 'vCard Generator',
    e.description = 'Digital business card creator',
    e.llm_context = 'USE: when discussing vCard generators, digital business card creators, contact QR tools, or VCF file creation. TRIGGERS: vcard generator, vcard creator, digital business card, contact qr, vcf generator, electronic business card. NOT: business cards (medium), qr code generator (general).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'wifi-qr-generator'})
SET e.type = 'TOOL',
    e.display_name = 'WiFi QR Generator',
    e.description = 'WiFi credential QR creator',
    e.llm_context = 'USE: when discussing WiFi QR generators, WiFi password sharing via QR, or wireless network QR tools. TRIGGERS: wifi qr generator, wifi qr code, share wifi password, wifi qr, wireless qr, network qr code. NOT: qr code generator (general), qr code wifi (content type).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'batch-qr-generator'})
SET e.type = 'TOOL',
    e.display_name = 'Batch QR Generator',
    e.description = 'Bulk QR creation tool',
    e.llm_context = 'USE: when discussing batch QR generation, bulk QR creation tools, spreadsheet QR generation, or mass QR production. TRIGGERS: batch qr generator, bulk qr tool, mass qr, spreadsheet qr, csv qr, multiple qr generator. NOT: bulk creation (feature), qr code generator (single).',
    e.phase = 'Features & Tools',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Mediums ---

MERGE (e:Entity {key: 'business-cards'})
SET e.type = 'MEDIUM',
    e.display_name = 'Business Cards',
    e.description = 'Professional contact cards',
    e.llm_context = 'USE: when discussing QR codes on business cards, networking cards, or professional contact materials. TRIGGERS: business cards, business card qr, card qr code, networking card, contact card, visiting card. NOT: vcard (digital format), flyers (larger print).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'flyers'})
SET e.type = 'MEDIUM',
    e.display_name = 'Flyers',
    e.description = 'Promotional flyers',
    e.llm_context = 'USE: when discussing QR codes on flyers, promotional handouts, or single-sheet marketing materials. TRIGGERS: flyers, flyer qr, handout, leaflet, promotional flyer, marketing flyer. NOT: brochures (folded), posters (large format).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'posters-billboards'})
SET e.type = 'MEDIUM',
    e.display_name = 'Posters & Billboards',
    e.description = 'Large format displays',
    e.llm_context = 'USE: when discussing QR codes on posters, billboards, outdoor advertising, or large-format displays. TRIGGERS: poster qr, billboard qr, outdoor qr, large qr, signage qr, advertising poster. NOT: banners (fabric), flyers (small print).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'brochures'})
SET e.type = 'MEDIUM',
    e.display_name = 'Brochures',
    e.description = 'Folded informational materials',
    e.llm_context = 'USE: when discussing QR codes on brochures, tri-folds, pamphlets, or folded marketing materials. TRIGGERS: brochure qr, tri-fold, pamphlet, folded brochure, informational brochure. NOT: flyers (single sheet), catalogs (bound).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'catalogs'})
SET e.type = 'MEDIUM',
    e.display_name = 'Catalogs',
    e.description = 'Product catalogs',
    e.llm_context = 'USE: when discussing QR codes in product catalogs, print catalogs, or catalog shopping materials. TRIGGERS: catalog qr, product catalog, print catalog, shopping catalog, catalogue. NOT: brochures (folded), magazines (editorial).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'magazines'})
SET e.type = 'MEDIUM',
    e.display_name = 'Magazines',
    e.description = 'Print publications',
    e.llm_context = 'USE: when discussing QR codes in magazines, print publications, or editorial content. TRIGGERS: magazine qr, print magazine, editorial qr, publication qr, magazine ad. NOT: newspapers (daily), brochures (marketing).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'newspapers'})
SET e.type = 'MEDIUM',
    e.display_name = 'Newspapers',
    e.description = 'News publications',
    e.llm_context = 'USE: when discussing QR codes in newspapers, print news, or daily/weekly publications. TRIGGERS: newspaper qr, print news, daily paper, news publication, newspaper ad. NOT: magazines (glossy), digital news (websites).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'direct-mail'})
SET e.type = 'MEDIUM',
    e.display_name = 'Direct Mail',
    e.description = 'Mailed marketing materials',
    e.llm_context = 'USE: when discussing QR codes on direct mail, postcards, mailers, or shipped marketing materials. TRIGGERS: direct mail qr, postcard qr, mailer, mailed marketing, postal qr. NOT: flyers (handed out), email (digital).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'stickers-labels'})
SET e.type = 'MEDIUM',
    e.display_name = 'Stickers & Labels',
    e.description = 'Adhesive prints',
    e.llm_context = 'USE: when discussing QR codes on stickers, adhesive labels, or peel-and-stick materials. TRIGGERS: sticker qr, label qr, adhesive qr, peel and stick, vinyl sticker. NOT: product labels (packaging), product packaging (boxes).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'banners'})
SET e.type = 'MEDIUM',
    e.display_name = 'Banners',
    e.description = 'Event and trade show banners',
    e.llm_context = 'USE: when discussing QR codes on banners, trade show displays, event signage, or fabric/vinyl banners. TRIGGERS: banner qr, trade show banner, event banner, fabric banner, vinyl banner, roll-up banner. NOT: posters (paper), billboards (outdoor).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'product-packaging'})
SET e.type = 'MEDIUM',
    e.display_name = 'Product Packaging',
    e.description = 'Product boxes and containers',
    e.llm_context = 'USE: when discussing QR codes on product packaging, boxes, containers, or retail packaging. TRIGGERS: packaging qr, product box, container qr, retail packaging, package qr code. NOT: product labels (adhesive), shipping labels (logistics).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'product-labels'})
SET e.type = 'MEDIUM',
    e.display_name = 'Product Labels',
    e.description = 'Applied product labels',
    e.llm_context = 'USE: when discussing QR codes on product labels, applied labels, or product tag labels. TRIGGERS: product label qr, applied label, product tag, label qr code, item label. NOT: stickers (decorative), product packaging (boxes).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'receipts'})
SET e.type = 'MEDIUM',
    e.display_name = 'Receipts',
    e.description = 'Transaction receipts',
    e.llm_context = 'USE: when discussing QR codes on receipts, transaction slips, or purchase confirmations. TRIGGERS: receipt qr, transaction receipt, purchase receipt, pos receipt, sales receipt. NOT: tickets (entry), invoices (billing).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'table-tents'})
SET e.type = 'MEDIUM',
    e.display_name = 'Table Tents',
    e.description = 'Restaurant table displays',
    e.llm_context = 'USE: when discussing QR codes on table tents, restaurant table stands, or tabletop displays. TRIGGERS: table tent qr, table stand, restaurant table qr, tabletop display, table card. NOT: printed menus (booklet), flyers (handed out).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'menus-printed'})
SET e.type = 'MEDIUM',
    e.display_name = 'Printed Menus',
    e.description = 'Physical restaurant menus',
    e.llm_context = 'USE: when discussing QR codes on printed menus, physical restaurant menus, or paper menus. TRIGGERS: printed menu qr, paper menu, physical menu, restaurant menu qr, laminated menu. NOT: digital menu (online), table tents (standing).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'tickets-physical'})
SET e.type = 'MEDIUM',
    e.display_name = 'Physical Tickets',
    e.description = 'Event tickets',
    e.llm_context = 'USE: when discussing QR codes on physical tickets, event tickets, admission tickets, or printed tickets. TRIGGERS: ticket qr, event ticket, physical ticket, admission ticket, printed ticket, concert ticket. NOT: digital tickets (mobile), receipts (purchase).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'emails'})
SET e.type = 'MEDIUM',
    e.display_name = 'Emails',
    e.description = 'Email signatures and campaigns',
    e.llm_context = 'USE: when discussing QR codes in emails, email signatures, email campaigns, or email marketing. TRIGGERS: email qr, email signature, email campaign, email marketing qr, newsletter qr. NOT: websites (web pages), documents (pdfs).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'presentations'})
SET e.type = 'MEDIUM',
    e.display_name = 'Presentations',
    e.description = 'Slides and decks',
    e.llm_context = 'USE: when discussing QR codes in presentations, slide decks, PowerPoint, or meeting materials. TRIGGERS: presentation qr, slide qr, powerpoint qr, deck qr, meeting slides, conference presentation. NOT: documents (static), websites (interactive).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'documents'})
SET e.type = 'MEDIUM',
    e.display_name = 'Documents',
    e.description = 'PDFs, reports, and contracts',
    e.llm_context = 'USE: when discussing QR codes in documents, PDFs, reports, contracts, or printed documents. TRIGGERS: document qr, pdf qr, report qr, contract qr, printed document. NOT: presentations (slides), emails (messages).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'websites'})
SET e.type = 'MEDIUM',
    e.display_name = 'Websites',
    e.description = 'Web pages',
    e.llm_context = 'USE: when discussing QR codes displayed on websites, web pages, or online platforms. TRIGGERS: website qr, web page qr, online qr, site qr, desktop to mobile. NOT: emails (messages), presentations (slides).',
    e.phase = 'Mediums',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Actions ---

MERGE (e:Entity {key: 'create-qr-code'})
SET e.type = 'ACTION',
    e.display_name = 'Create QR Code',
    e.description = 'Generate a new QR code',
    e.llm_context = 'USE: when discussing QR code creation, making QR codes, or generating new QR codes. TRIGGERS: create qr code, make qr, generate qr, new qr code, create qr, make qr code. NOT: customize qr (design), scan qr (reading), edit destination (update).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'scan-qr-code'})
SET e.type = 'ACTION',
    e.display_name = 'Scan QR Code',
    e.description = 'Read QR code data',
    e.llm_context = 'USE: when discussing scanning QR codes, reading QR codes, or decoding QR content. TRIGGERS: scan qr code, read qr, decode qr, scan qr, qr scan, use camera to scan. NOT: create qr (making), track scans (analytics).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'customize-qr-code'})
SET e.type = 'ACTION',
    e.display_name = 'Customize QR Code',
    e.description = 'Design and style a QR code',
    e.llm_context = 'USE: when discussing QR code customization, styling QR codes, or designing QR appearance. TRIGGERS: customize qr, design qr, style qr, qr design, qr customization, personalize qr. NOT: create qr (initial), add logo (specific), change colors (specific).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'download-qr-code'})
SET e.type = 'ACTION',
    e.display_name = 'Download QR Code',
    e.description = 'Export QR as image file',
    e.llm_context = 'USE: when discussing downloading QR codes, exporting QR images, or saving QR files. TRIGGERS: download qr, export qr, save qr, qr download, get qr image, qr file. NOT: print qr (physical), share qr (distribute).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'print-qr-code'})
SET e.type = 'ACTION',
    e.display_name = 'Print QR Code',
    e.description = 'Print for physical use',
    e.llm_context = 'USE: when discussing printing QR codes, physical QR output, or QR print requirements. TRIGGERS: print qr, qr printing, physical qr, print qr code, qr for print, printable qr. NOT: download qr (digital), qr size (specification).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'add-logo'})
SET e.type = 'ACTION',
    e.display_name = 'Add Logo',
    e.description = 'Add logo to QR center',
    e.llm_context = 'USE: when discussing adding logos to QR codes, branded QR codes, or logo placement in QR. TRIGGERS: add logo, logo qr, branded qr, qr with logo, center logo, embed logo. NOT: customize qr (general), change colors (different aspect).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'change-colors'})
SET e.type = 'ACTION',
    e.display_name = 'Change Colors',
    e.description = 'Customize QR colors',
    e.llm_context = 'USE: when discussing QR code color customization, changing QR colors, or colored QR codes. TRIGGERS: change colors, qr colors, colored qr, custom color qr, qr color scheme. NOT: add logo (different aspect), customize qr (general).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'edit-destination'})
SET e.type = 'ACTION',
    e.display_name = 'Edit Destination',
    e.description = 'Change QR target URL',
    e.llm_context = 'USE: when discussing editing QR destinations, changing where QR points, or updating QR URLs. TRIGGERS: edit destination, change url, update link, modify destination, redirect qr, change qr target. NOT: create qr (initial), dynamic qr (code type).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'share-qr-code'})
SET e.type = 'ACTION',
    e.display_name = 'Share QR Code',
    e.description = 'Share via link or email',
    e.llm_context = 'USE: when discussing sharing QR codes, distributing QR images, or sending QR codes to others. TRIGGERS: share qr, send qr, distribute qr, qr sharing, email qr, share qr code. NOT: download qr (save), print qr (physical).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'track-scans'})
SET e.type = 'ACTION',
    e.display_name = 'Track Scans',
    e.description = 'Monitor scan analytics',
    e.llm_context = 'USE: when discussing scan tracking, monitoring QR performance, or viewing scan analytics. TRIGGERS: track scans, scan analytics, monitor scans, scan statistics, scan tracking, qr tracking. NOT: scan qr (action), analytics (feature).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'shorten-url'})
SET e.type = 'ACTION',
    e.display_name = 'Shorten URL',
    e.description = 'Create short URL',
    e.llm_context = 'USE: when discussing URL shortening, creating short links, or compacting long URLs. TRIGGERS: shorten url, short link, short url, url shortener, compact link, tiny url. NOT: create smart link (intelligent), custom domain (branding).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'create-smart-link'})
SET e.type = 'ACTION',
    e.display_name = 'Create Smart Link',
    e.description = 'Create intelligent link',
    e.llm_context = 'USE: when discussing smart link creation, intelligent links, or advanced short links with routing. TRIGGERS: create smart link, smart link, intelligent link, routing link, conditional link. NOT: shorten url (basic), contextual routing (feature).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'create-landing-page'})
SET e.type = 'ACTION',
    e.display_name = 'Create Landing Page',
    e.description = 'Build destination page',
    e.llm_context = 'USE: when discussing landing page creation, building destination pages, or creating bio pages. TRIGGERS: create landing page, build page, make landing page, destination page, bio page creation. NOT: landing page builder (tool), link in bio (specific type).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'create-barcode'})
SET e.type = 'ACTION',
    e.display_name = 'Create Barcode',
    e.description = 'Generate barcode',
    e.llm_context = 'USE: when discussing barcode creation, generating 1D barcodes, or making EAN/UPC codes. TRIGGERS: create barcode, generate barcode, make barcode, barcode creation, ean barcode, upc barcode. NOT: create qr (2D), scan barcode (reading).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'scan-barcode'})
SET e.type = 'ACTION',
    e.display_name = 'Scan Barcode',
    e.description = 'Read barcode data',
    e.llm_context = 'USE: when discussing barcode scanning, reading 1D barcodes, or decoding product barcodes. TRIGGERS: scan barcode, read barcode, barcode scan, decode barcode, barcode reader. NOT: scan qr (2D), create barcode (making).',
    e.phase = 'Actions',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Industries ---

MERGE (e:Entity {key: 'restaurants'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Restaurants',
    e.description = 'Food service, cafes, bars',
    e.llm_context = 'USE: when discussing QR codes for restaurants, food service, cafes, bars, or digital menus. TRIGGERS: restaurant qr, cafe qr, bar qr, food service qr, dine-in qr, table qr. NOT: retail (shopping), hospitality (hotels).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'retail'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Retail',
    e.description = 'Shops, stores, e-commerce',
    e.llm_context = 'USE: when discussing QR codes for retail, shops, stores, or e-commerce product labeling. TRIGGERS: retail qr, store qr, shop qr, product qr, e-commerce qr, shopping qr. NOT: restaurants (food), hospitality (hotels).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'hospitality'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Hospitality',
    e.description = 'Hotels, resorts, tourism',
    e.llm_context = 'USE: when discussing QR codes for hotels, resorts, tourism, or guest services. TRIGGERS: hotel qr, resort qr, hospitality qr, tourism qr, guest qr, room qr. NOT: restaurants (food), retail (shopping).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'healthcare'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Healthcare',
    e.description = 'Hospitals, clinics, pharma',
    e.llm_context = 'USE: when discussing QR codes for healthcare, hospitals, clinics, or pharmaceutical applications. TRIGGERS: healthcare qr, hospital qr, clinic qr, pharma qr, medical qr, patient qr. NOT: fitness (gyms), beauty (spas).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'education'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Education',
    e.description = 'Schools, universities, training',
    e.llm_context = 'USE: when discussing QR codes for schools, universities, education, or training materials. TRIGGERS: education qr, school qr, university qr, student qr, classroom qr, learning qr. NOT: entertainment (events), government (public).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'real-estate'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Real Estate',
    e.description = 'Property sales and rentals',
    e.llm_context = 'USE: when discussing QR codes for real estate, property listings, or virtual tours. TRIGGERS: real estate qr, property qr, house qr, listing qr, virtual tour qr, for sale qr. NOT: construction (building), hospitality (hotels).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'fitness'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Fitness',
    e.description = 'Gyms, sports, wellness',
    e.llm_context = 'USE: when discussing QR codes for gyms, fitness centers, sports, or wellness applications. TRIGGERS: gym qr, fitness qr, sports qr, wellness qr, workout qr, exercise qr. NOT: healthcare (medical), beauty (cosmetics).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'beauty'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Beauty',
    e.description = 'Salons, spas, cosmetics',
    e.llm_context = 'USE: when discussing QR codes for salons, spas, cosmetics, or beauty products. TRIGGERS: beauty qr, salon qr, spa qr, cosmetics qr, skincare qr, makeup qr. NOT: fitness (gyms), healthcare (medical).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'entertainment'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Entertainment',
    e.description = 'Movies, games, events',
    e.llm_context = 'USE: when discussing QR codes for entertainment, movies, games, or live events. TRIGGERS: entertainment qr, movie qr, event qr, concert qr, theater qr, gaming qr. NOT: education (learning), hospitality (hotels).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'transportation'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Transportation',
    e.description = 'Airlines, transit, logistics',
    e.llm_context = 'USE: when discussing QR codes for airlines, transit, or transportation ticketing. TRIGGERS: airline qr, transit qr, transportation qr, boarding pass qr, train qr, bus qr. NOT: logistics (warehouse), manufacturing (production).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'manufacturing'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Manufacturing',
    e.description = 'Production and assembly',
    e.llm_context = 'USE: when discussing QR codes for manufacturing, production, or assembly lines. TRIGGERS: manufacturing qr, factory qr, production qr, assembly qr, industrial qr, parts qr. NOT: logistics (shipping), construction (building).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'logistics'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Logistics',
    e.description = 'Shipping and warehousing',
    e.llm_context = 'USE: when discussing QR codes for logistics, shipping, or warehouse management. TRIGGERS: logistics qr, shipping qr, warehouse qr, delivery qr, supply chain qr, tracking qr. NOT: manufacturing (production), retail (stores).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'construction'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Construction',
    e.description = 'Building and infrastructure',
    e.llm_context = 'USE: when discussing QR codes for construction sites, building projects, or infrastructure. TRIGGERS: construction qr, building qr, site qr, contractor qr, blueprint qr, safety qr. NOT: real estate (sales), manufacturing (production).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'finance'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Finance',
    e.description = 'Banking and insurance',
    e.llm_context = 'USE: when discussing QR codes for banking, finance, insurance, or financial services. TRIGGERS: finance qr, banking qr, insurance qr, payment qr, atm qr, fintech qr. NOT: retail (shopping), government (public sector).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'government'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Government',
    e.description = 'Public sector and administration',
    e.llm_context = 'USE: when discussing QR codes for government, public sector, or citizen services. TRIGGERS: government qr, public sector qr, citizen qr, municipal qr, civic qr, id card qr. NOT: nonprofit (charity), enterprise (business).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'marketing-agencies'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Marketing Agencies',
    e.description = 'Digital marketing firms',
    e.llm_context = 'USE: when discussing QR codes for marketing agencies, digital marketing, or advertising campaigns. TRIGGERS: marketing agency qr, digital marketing qr, campaign qr, advertising qr, agency qr. NOT: creative agencies (design), consulting (business).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'creative-agencies'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Creative Agencies',
    e.description = 'Design and branding',
    e.llm_context = 'USE: when discussing QR codes for creative agencies, design firms, or branding work. TRIGGERS: creative agency qr, design agency qr, branding qr, designer qr, creative qr. NOT: marketing agencies (advertising), consulting (business).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'event-management'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Event Management',
    e.description = 'Conferences, weddings, events',
    e.llm_context = 'USE: when discussing QR codes for event management, conferences, weddings, or event planning. TRIGGERS: event management qr, conference qr, wedding qr, event planner qr, venue qr. NOT: entertainment (movies), hospitality (hotels).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'nonprofits'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Nonprofits',
    e.description = 'Charities and NGOs',
    e.llm_context = 'USE: when discussing QR codes for nonprofits, charities, NGOs, or donation collection. TRIGGERS: nonprofit qr, charity qr, ngo qr, donation qr, fundraising qr, volunteer qr. NOT: government (public sector), small business (commercial).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'consulting'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Consulting',
    e.description = 'Business consulting',
    e.llm_context = 'USE: when discussing QR codes for consulting firms, business consulting, or professional services. TRIGGERS: consulting qr, consultant qr, advisory qr, professional services qr, firm qr. NOT: agencies (marketing), enterprise (large corp).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'developers'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Developers',
    e.description = 'API users and integrators',
    e.llm_context = 'USE: when discussing developers, API integration, or programmatic QR code generation. TRIGGERS: developer qr, api user, programmer qr, integrator, coder qr, tech qr. NOT: enterprise (organization), agencies (marketing).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'enterprise'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Enterprise',
    e.description = 'Large organizations',
    e.llm_context = 'USE: when discussing QR codes for enterprise, large organizations, or corporate deployments. TRIGGERS: enterprise qr, corporate qr, large organization qr, company-wide qr, sso qr. NOT: small business (smb), freelancers (individual).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'agencies'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Agencies',
    e.description = 'Marketing and creative agencies',
    e.llm_context = 'USE: when discussing agencies managing QR codes for multiple clients. TRIGGERS: agency qr, multi-client qr, client management qr, agency workspaces. NOT: enterprise (internal), small business (single owner).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'small-business'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Small Business',
    e.description = 'SMBs and local businesses',
    e.llm_context = 'USE: when discussing QR codes for small businesses, SMBs, or local businesses. TRIGGERS: small business qr, smb qr, local business qr, shop owner qr, mom and pop qr. NOT: enterprise (large), freelancers (individual).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'freelancers'})
SET e.type = 'INDUSTRY',
    e.display_name = 'Freelancers',
    e.description = 'Independent professionals',
    e.llm_context = 'USE: when discussing QR codes for freelancers, independent professionals, or solo entrepreneurs. TRIGGERS: freelancer qr, independent qr, solo qr, self-employed qr, contractor qr. NOT: small business (employees), agencies (teams).',
    e.phase = 'Industries',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Brands ---

MERGE (e:Entity {key: 'instagram'})
SET e.type = 'BRAND',
    e.display_name = 'Instagram',
    e.description = 'Photo and video social network',
    e.llm_context = 'USE: when discussing Instagram QR codes, Instagram profiles, or Meta social sharing. TRIGGERS: instagram, ig, instagram qr, instagram profile, instagram link. NOT: facebook (separate platform), tiktok (competitor).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'linkedin'})
SET e.type = 'BRAND',
    e.display_name = 'LinkedIn',
    e.description = 'Professional networking platform',
    e.llm_context = 'USE: when discussing LinkedIn QR codes, professional networking, or business profiles. TRIGGERS: linkedin, linkedin qr, linkedin profile, professional network, business network. NOT: facebook (social), twitter (microblog).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'facebook'})
SET e.type = 'BRAND',
    e.display_name = 'Facebook',
    e.description = 'Social networking platform',
    e.llm_context = 'USE: when discussing Facebook QR codes, Facebook pages, or Meta social networking. TRIGGERS: facebook, fb, facebook qr, facebook page, facebook group. NOT: instagram (visual), linkedin (professional).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'twitter'})
SET e.type = 'BRAND',
    e.display_name = 'Twitter / X',
    e.description = 'Microblogging social platform',
    e.llm_context = 'USE: when discussing Twitter/X QR codes, tweets, or microblogging. TRIGGERS: twitter, x, twitter qr, tweet, x platform, twitter profile. NOT: facebook (social network), linkedin (professional).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'youtube'})
SET e.type = 'BRAND',
    e.display_name = 'YouTube',
    e.description = 'Video sharing platform',
    e.llm_context = 'USE: when discussing YouTube QR codes, YouTube channels, or video linking. TRIGGERS: youtube, yt, youtube qr, youtube channel, youtube video, video platform. NOT: tiktok (short-form), spotify (audio).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'tiktok'})
SET e.type = 'BRAND',
    e.display_name = 'TikTok',
    e.description = 'Short-form video platform',
    e.llm_context = 'USE: when discussing TikTok QR codes, short-form video, or Gen Z marketing. TRIGGERS: tiktok, tik tok, tiktok qr, tiktok profile, short video. NOT: youtube (long-form), instagram (photos).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'snapchat'})
SET e.type = 'BRAND',
    e.display_name = 'Snapchat',
    e.description = 'Ephemeral messaging and AR platform',
    e.llm_context = 'USE: when discussing Snapchat QR codes, Snapcodes, or AR filters. TRIGGERS: snapchat, snapcode, snapchat qr, snap, snapchat filter, ar filter. NOT: instagram (stories), tiktok (video).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'whatsapp'})
SET e.type = 'BRAND',
    e.display_name = 'WhatsApp',
    e.description = 'Messaging application',
    e.llm_context = 'USE: when discussing WhatsApp QR codes, click-to-chat, or WhatsApp business. TRIGGERS: whatsapp, wa, whatsapp qr, whatsapp chat, click to chat, whatsapp business. NOT: telegram (alternative), messenger (facebook).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'telegram'})
SET e.type = 'BRAND',
    e.display_name = 'Telegram',
    e.description = 'Secure messaging platform',
    e.llm_context = 'USE: when discussing Telegram QR codes, Telegram channels, or secure messaging. TRIGGERS: telegram, telegram qr, telegram channel, telegram group, secure chat. NOT: whatsapp (alternative), signal (privacy).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'pinterest'})
SET e.type = 'BRAND',
    e.display_name = 'Pinterest',
    e.description = 'Visual discovery platform',
    e.llm_context = 'USE: when discussing Pinterest QR codes, pins, or visual discovery. TRIGGERS: pinterest, pin, pinterest qr, pinterest board, visual discovery, pincode. NOT: instagram (social), etsy (commerce).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'spotify'})
SET e.type = 'BRAND',
    e.display_name = 'Spotify',
    e.description = 'Music streaming service',
    e.llm_context = 'USE: when discussing Spotify QR codes, Spotify Codes, or music streaming links. TRIGGERS: spotify, spotify code, spotify qr, music streaming, playlist qr, spotify playlist. NOT: apple music (competitor), soundcloud (indie).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'apple'})
SET e.type = 'BRAND',
    e.display_name = 'Apple',
    e.description = 'Technology company with music and app platforms',
    e.llm_context = 'USE: when discussing Apple Music QR codes, App Store links, or Apple ecosystem. TRIGGERS: apple, apple music, app store, itunes, ios app, apple qr. NOT: spotify (music streaming), google play (android).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'soundcloud'})
SET e.type = 'BRAND',
    e.display_name = 'SoundCloud',
    e.description = 'Audio streaming and sharing platform',
    e.llm_context = 'USE: when discussing SoundCloud QR codes, indie music, or audio sharing. TRIGGERS: soundcloud, soundcloud qr, indie music, dj music, audio platform, music upload. NOT: spotify (mainstream), apple music (apple).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'paypal'})
SET e.type = 'BRAND',
    e.display_name = 'PayPal',
    e.description = 'Digital payment platform',
    e.llm_context = 'USE: when discussing PayPal QR codes, PayPal.me links, or PayPal payments. TRIGGERS: paypal, paypal qr, paypal.me, paypal payment, online payment. NOT: venmo (p2p), stripe (developer).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'venmo'})
SET e.type = 'BRAND',
    e.display_name = 'Venmo',
    e.description = 'Mobile payment service',
    e.llm_context = 'USE: when discussing Venmo QR codes, Venmo payments, or peer-to-peer US payments. TRIGGERS: venmo, venmo qr, venmo payment, split bill, peer payment, p2p payment. NOT: paypal (parent), zelle (bank).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'google'})
SET e.type = 'BRAND',
    e.display_name = 'Google',
    e.description = 'Technology company with maps, reviews, and business tools',
    e.llm_context = 'USE: when discussing Google QR codes, Google Maps, Google Reviews, or Google Business. TRIGGERS: google, google maps, google review, google business, google qr, play store. NOT: apple (competitor), waze (navigation).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'waze'})
SET e.type = 'BRAND',
    e.display_name = 'Waze',
    e.description = 'Community-driven navigation app',
    e.llm_context = 'USE: when discussing Waze QR codes, Waze navigation, or community GPS. TRIGGERS: waze, waze qr, waze navigation, waze directions, community navigation. NOT: google maps (google), apple maps (apple).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'zapier'})
SET e.type = 'BRAND',
    e.display_name = 'Zapier',
    e.description = 'Workflow automation platform',
    e.llm_context = 'USE: when discussing Zapier, workflow automation, or app integrations. TRIGGERS: zapier, zap, zapier automation, workflow automation, app connector. NOT: make (integromat), n8n (self-hosted).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'make'})
SET e.type = 'BRAND',
    e.display_name = 'Make (Integromat)',
    e.description = 'Visual automation platform',
    e.llm_context = 'USE: when discussing Make/Integromat, visual automation, or complex workflows. TRIGGERS: make, integromat, make automation, visual automation, scenario builder. NOT: zapier (simpler), n8n (self-hosted).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'n8n'})
SET e.type = 'BRAND',
    e.display_name = 'n8n',
    e.description = 'Open-source automation tool',
    e.llm_context = 'USE: when discussing n8n, self-hosted automation, or open-source workflows. TRIGGERS: n8n, self-hosted automation, open source automation, privacy automation. NOT: zapier (hosted), make (hosted).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'hubspot'})
SET e.type = 'BRAND',
    e.display_name = 'HubSpot',
    e.description = 'CRM and marketing platform',
    e.llm_context = 'USE: when discussing HubSpot, CRM integration, or marketing automation with QR codes. TRIGGERS: hubspot, hubspot crm, hubspot marketing, inbound marketing, hubspot integration. NOT: salesforce (enterprise), mailchimp (email only).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'salesforce'})
SET e.type = 'BRAND',
    e.display_name = 'Salesforce',
    e.description = 'Enterprise CRM platform',
    e.llm_context = 'USE: when discussing Salesforce, enterprise CRM, or sales automation. TRIGGERS: salesforce, salesforce crm, enterprise crm, salesforce integration, sales cloud. NOT: hubspot (smb), zoho (alternative).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'mailchimp'})
SET e.type = 'BRAND',
    e.display_name = 'Mailchimp',
    e.description = 'Email marketing platform',
    e.llm_context = 'USE: when discussing Mailchimp, email marketing, or newsletter QR codes. TRIGGERS: mailchimp, email marketing, mailchimp integration, newsletter qr, email list qr. NOT: hubspot (full crm), sendgrid (api).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'shopify'})
SET e.type = 'BRAND',
    e.display_name = 'Shopify',
    e.description = 'E-commerce platform',
    e.llm_context = 'USE: when discussing Shopify, e-commerce QR codes, or online store integration. TRIGGERS: shopify, shopify qr, shopify store, e-commerce platform, shopify product. NOT: woocommerce (wordpress), amazon (marketplace).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'woocommerce'})
SET e.type = 'BRAND',
    e.display_name = 'WooCommerce',
    e.description = 'WordPress e-commerce plugin',
    e.llm_context = 'USE: when discussing WooCommerce, WordPress e-commerce, or open-source stores. TRIGGERS: woocommerce, woo commerce, wordpress store, woocommerce qr, wordpress e-commerce. NOT: shopify (hosted), magento (enterprise).',
    e.phase = 'Brands',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Integrations ---

MERGE (e:Entity {key: 'zapier-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'Zapier Integration',
    e.description = 'Workflow automation via Zapier',
    e.llm_context = 'USE: when discussing Zapier integration with QR Code AI, automated QR workflows, or zap connections. TRIGGERS: zapier integration, qr zapier, zap qr, automate qr, zapier connection. NOT: make integration (different platform), direct api (not integration).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'make-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'Make Integration',
    e.description = 'Visual automation via Make (Integromat)',
    e.llm_context = 'USE: when discussing Make/Integromat integration with QR Code AI or visual scenario automation. TRIGGERS: make integration, integromat integration, qr make, qr integromat, visual automation integration. NOT: zapier integration (different platform), n8n integration (self-hosted).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'n8n-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'n8n Integration',
    e.description = 'Self-hosted automation via n8n',
    e.llm_context = 'USE: when discussing n8n integration with QR Code AI or self-hosted privacy-first automation. TRIGGERS: n8n integration, qr n8n, self-hosted qr automation, privacy qr automation. NOT: zapier integration (hosted), make integration (hosted).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'hubspot-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'HubSpot Integration',
    e.description = 'CRM and marketing automation via HubSpot',
    e.llm_context = 'USE: when discussing HubSpot integration with QR Code AI or QR-to-CRM lead syncing. TRIGGERS: hubspot integration, qr hubspot, crm qr integration, hubspot qr sync, marketing automation qr. NOT: salesforce integration (enterprise), mailchimp integration (email).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'salesforce-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'Salesforce Integration',
    e.description = 'Enterprise CRM via Salesforce',
    e.llm_context = 'USE: when discussing Salesforce integration with QR Code AI or enterprise CRM QR connections. TRIGGERS: salesforce integration, qr salesforce, enterprise crm qr, salesforce qr sync. NOT: hubspot integration (smb), dynamics integration (microsoft).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'mailchimp-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'Mailchimp Integration',
    e.description = 'Email marketing via Mailchimp',
    e.llm_context = 'USE: when discussing Mailchimp integration with QR Code AI or QR-to-email list building. TRIGGERS: mailchimp integration, qr mailchimp, email qr integration, newsletter qr signup, list building qr. NOT: hubspot integration (full crm), sendgrid (api only).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'google-sheets-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'Google Sheets Integration',
    e.description = 'Spreadsheet sync via Google Sheets',
    e.llm_context = 'USE: when discussing Google Sheets integration with QR Code AI or spreadsheet QR data sync. TRIGGERS: google sheets integration, qr google sheets, spreadsheet qr, sheets qr sync, batch qr from sheets. NOT: notion integration (workspace), airtable (database).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'notion-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'Notion Integration',
    e.description = 'Workspace sync via Notion',
    e.llm_context = 'USE: when discussing Notion integration with QR Code AI or workspace QR management. TRIGGERS: notion integration, qr notion, notion qr sync, workspace qr, notion database qr. NOT: google sheets integration (spreadsheet), coda (alternative).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'slack-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'Slack Integration',
    e.description = 'Team notifications via Slack',
    e.llm_context = 'USE: when discussing Slack integration with QR Code AI or QR scan notifications. TRIGGERS: slack integration, qr slack, slack notifications qr, team alert qr, slack channel qr. NOT: teams integration (microsoft), discord (community).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'shopify-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'Shopify Integration',
    e.description = 'E-commerce QR codes via Shopify',
    e.llm_context = 'USE: when discussing Shopify integration with QR Code AI or e-commerce product QR codes. TRIGGERS: shopify integration, qr shopify, shopify product qr, e-commerce qr integration, shopify store qr. NOT: woocommerce integration (wordpress), amazon (marketplace).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'woocommerce-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'WooCommerce Integration',
    e.description = 'WordPress e-commerce via WooCommerce',
    e.llm_context = 'USE: when discussing WooCommerce integration with QR Code AI or WordPress e-commerce QR codes. TRIGGERS: woocommerce integration, qr woocommerce, wordpress qr store, woo qr integration. NOT: shopify integration (hosted), magento (enterprise).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'wordpress-integration'})
SET e.type = 'INTEGRATION',
    e.display_name = 'WordPress Integration',
    e.description = 'CMS integration via WordPress',
    e.llm_context = 'USE: when discussing WordPress integration with QR Code AI or embedding QR codes in WordPress. TRIGGERS: wordpress integration, qr wordpress, wordpress qr plugin, wp qr shortcode, cms qr integration. NOT: woocommerce integration (e-commerce), squarespace (different cms).',
    e.phase = 'Integrations',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Technical Concepts ---

MERGE (e:Entity {key: 'quiet-zone'})
SET e.type = 'CONCEPT',
    e.display_name = 'Quiet Zone',
    e.description = 'White margin around QR (minimum 4 modules)',
    e.llm_context = 'USE: when discussing QR code margins, white border requirements, or scan failures due to cropping. TRIGGERS: quiet zone, qr margin, white border, qr padding, module margin, quiet area. NOT: finder pattern (corners), timing pattern (lines).',
    e.phase = 'Technical Concepts',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'error-correction'})
SET e.type = 'CONCEPT',
    e.display_name = 'Error Correction',
    e.description = 'Reed-Solomon encoding (L/M/Q/H levels)',
    e.llm_context = 'USE: when discussing QR code damage tolerance, logo placement, or error correction levels L/M/Q/H. TRIGGERS: error correction, qr damage, reed solomon, correction level, damaged qr, logo error correction. NOT: data capacity (size), encoding mode (character type).',
    e.phase = 'Technical Concepts',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'data-capacity'})
SET e.type = 'CONCEPT',
    e.display_name = 'Data Capacity',
    e.description = 'Maximum characters based on version and error correction',
    e.llm_context = 'USE: when discussing how much data a QR code can hold, character limits, or QR size requirements. TRIGGERS: data capacity, qr capacity, character limit, qr data size, how much data, qr storage. NOT: error correction (damage), encoding mode (format).',
    e.phase = 'Technical Concepts',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-version'})
SET e.type = 'CONCEPT',
    e.display_name = 'QR Code Version',
    e.description = 'Size grid (21x21 v1 to 177x177 v40)',
    e.llm_context = 'USE: when discussing QR code size, version numbers 1-40, or module grid dimensions. TRIGGERS: qr version, qr size, version 1, version 40, module grid, qr dimensions. NOT: data capacity (characters), error correction (damage).',
    e.phase = 'Technical Concepts',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'encoding-mode'})
SET e.type = 'CONCEPT',
    e.display_name = 'Encoding Mode',
    e.description = 'Numeric/Alphanumeric/Byte/Kanji modes',
    e.llm_context = 'USE: when discussing QR encoding efficiency, character types, or numeric vs alphanumeric modes. TRIGGERS: encoding mode, numeric mode, alphanumeric mode, byte mode, kanji mode, qr encoding. NOT: error correction (damage), data capacity (size).',
    e.phase = 'Technical Concepts',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'module'})
SET e.type = 'CONCEPT',
    e.display_name = 'Module',
    e.description = 'Single black or white square unit',
    e.llm_context = 'USE: when discussing QR code pixels, individual squares, or module-level design. TRIGGERS: module, qr pixel, qr square, black module, white module, qr unit. NOT: finder pattern (corner squares), quiet zone (border).',
    e.phase = 'Technical Concepts',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'finder-pattern'})
SET e.type = 'CONCEPT',
    e.display_name = 'Finder Pattern',
    e.description = 'Three corner squares for orientation',
    e.llm_context = 'USE: when discussing QR code corner squares, orientation markers, or why QR codes have three big squares. TRIGGERS: finder pattern, corner squares, qr orientation, position detection, three squares, qr corners. NOT: timing pattern (lines), alignment pattern (small squares).',
    e.phase = 'Technical Concepts',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'timing-pattern'})
SET e.type = 'CONCEPT',
    e.display_name = 'Timing Pattern',
    e.description = 'Alternating modules for alignment',
    e.llm_context = 'USE: when discussing QR code alignment, alternating black/white lines, or grid calibration. TRIGGERS: timing pattern, alignment lines, alternating modules, grid calibration, qr timing. NOT: finder pattern (corners), module (single square).',
    e.phase = 'Technical Concepts',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Social Subcategories ---

MERGE (e:Entity {key: 'qr-code-messaging'})
SET e.type = 'THING',
    e.display_name = 'Messaging QR Codes',
    e.description = 'Subcategory for messaging app QR codes',
    e.llm_context = 'USE: when discussing QR codes for messaging apps like WhatsApp, Telegram, or chat platforms. TRIGGERS: messaging qr, chat qr, whatsapp category, telegram category, direct message qr. NOT: video platform (video content), professional (business networking).',
    e.phase = 'Social Subcategories',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-video-platform'})
SET e.type = 'THING',
    e.display_name = 'Video Platform QR Codes',
    e.description = 'Subcategory for video platform QR codes',
    e.llm_context = 'USE: when discussing QR codes for video platforms like YouTube, TikTok, or Snapchat. TRIGGERS: video platform qr, youtube category, tiktok category, video content qr, streaming qr. NOT: messaging (chat), music platform (audio).',
    e.phase = 'Social Subcategories',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-professional'})
SET e.type = 'THING',
    e.display_name = 'Professional Network QR Codes',
    e.description = 'Subcategory for professional networking QR codes',
    e.llm_context = 'USE: when discussing QR codes for professional networking like LinkedIn. TRIGGERS: professional qr, linkedin category, business networking qr, career qr, professional network. NOT: messaging (chat), video platform (content).',
    e.phase = 'Social Subcategories',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-music-platform'})
SET e.type = 'THING',
    e.display_name = 'Music Platform QR Codes',
    e.description = 'Subcategory for music streaming QR codes',
    e.llm_context = 'USE: when discussing QR codes for music platforms like Spotify, Apple Music, or SoundCloud. TRIGGERS: music platform qr, spotify category, apple music category, music streaming qr, playlist category. NOT: video platform (video), audio file (direct file).',
    e.phase = 'Social Subcategories',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Use Cases ---

MERGE (e:Entity {key: 'funny-qr-codes'})
SET e.type = 'USE_CASE',
    e.display_name = 'Funny QR Codes',
    e.description = 'Humorous and creative QR code applications',
    e.llm_context = 'USE: when discussing humorous, prank, or creative QR code applications like rickrolling. TRIGGERS: funny qr, prank qr, rickroll qr, meme qr, easter egg qr, joke qr. NOT: art installation (serious art), tattoo (permanent).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-tattoo'})
SET e.type = 'USE_CASE',
    e.display_name = 'QR Code Tattoo',
    e.description = 'Permanent QR codes as body art',
    e.llm_context = 'USE: when discussing QR code tattoos, permanent body art QR codes, or skin-based QR. TRIGGERS: qr tattoo, tattoo qr code, body art qr, permanent qr, skin qr, inked qr. NOT: temporary (sticker), art installation (public).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-art-installation'})
SET e.type = 'USE_CASE',
    e.display_name = 'QR Art Installation',
    e.description = 'Large-scale artistic QR code displays',
    e.llm_context = 'USE: when discussing large-scale QR art, public installations, or artistic QR displays. TRIGGERS: qr art, art installation qr, mural qr, public art qr, interactive art qr, projection qr. NOT: tattoo (body), funny qr (humor).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-scavenger-hunt'})
SET e.type = 'USE_CASE',
    e.display_name = 'QR Scavenger Hunt',
    e.description = 'Interactive treasure hunts using QR codes',
    e.llm_context = 'USE: when discussing QR scavenger hunts, treasure hunts, or gamified QR experiences. TRIGGERS: scavenger hunt qr, treasure hunt qr, qr game, interactive qr hunt, clue qr, quest qr. NOT: event check-in (registration), museum (educational).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-reviews'})
SET e.type = 'USE_CASE',
    e.display_name = 'QR for Reviews',
    e.description = 'Collecting customer reviews via QR',
    e.llm_context = 'USE: when discussing QR codes for collecting reviews, feedback, or ratings. TRIGGERS: review qr, feedback qr, rating qr, google review qr, yelp qr, customer feedback qr. NOT: loyalty program (rewards), payment (transaction).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-contactless-payment'})
SET e.type = 'USE_CASE',
    e.display_name = 'Contactless Payment',
    e.description = 'Touch-free payments via QR codes',
    e.llm_context = 'USE: when discussing QR-based contactless payments, touch-free transactions, or scan-to-pay. TRIGGERS: contactless payment qr, scan to pay, touch-free payment, qr payment, mobile payment qr. NOT: loyalty program (points), reviews (feedback).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-loyalty-program'})
SET e.type = 'USE_CASE',
    e.display_name = 'Loyalty Program',
    e.description = 'Customer loyalty via QR scanning',
    e.llm_context = 'USE: when discussing QR-based loyalty programs, digital punch cards, or rewards via QR. TRIGGERS: loyalty qr, punch card qr, rewards qr, points qr, member qr, stamp card qr. NOT: payment (transaction), reviews (feedback).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-product-authentication'})
SET e.type = 'USE_CASE',
    e.display_name = 'Product Authentication',
    e.description = 'Anti-counterfeit verification via QR',
    e.llm_context = 'USE: when discussing QR codes for product authentication, anti-counterfeiting, or verification. TRIGGERS: authentication qr, anti-counterfeit qr, verify product qr, genuine qr, counterfeit detection qr. NOT: loyalty (rewards), payment (transaction).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-event-checkin'})
SET e.type = 'USE_CASE',
    e.display_name = 'Event Check-in',
    e.description = 'Attendee registration via QR',
    e.llm_context = 'USE: when discussing QR codes for event check-in, registration, or attendee verification. TRIGGERS: event check-in qr, registration qr, attendee qr, ticket scan, conference check-in, entry qr. NOT: networking (contact exchange), wedding (personal event).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-networking'})
SET e.type = 'USE_CASE',
    e.display_name = 'Networking QR',
    e.description = 'Contact exchange at events',
    e.llm_context = 'USE: when discussing QR codes for professional networking, contact exchange, or event connections. TRIGGERS: networking qr, contact exchange qr, meet and greet qr, business card qr, connection qr. NOT: event check-in (registration), wedding (personal).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-wedding'})
SET e.type = 'USE_CASE',
    e.display_name = 'Wedding QR Codes',
    e.description = 'QR applications for weddings',
    e.llm_context = 'USE: when discussing QR codes for weddings, wedding invitations, or wedding RSVPs. TRIGGERS: wedding qr, wedding invitation qr, rsvp qr, wedding registry qr, wedding photo qr, marriage qr. NOT: event check-in (corporate), networking (business).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-museum-exhibit'})
SET e.type = 'USE_CASE',
    e.display_name = 'Museum Exhibits',
    e.description = 'Interactive museum experiences via QR',
    e.llm_context = 'USE: when discussing QR codes for museums, exhibits, galleries, or educational displays. TRIGGERS: museum qr, exhibit qr, gallery qr, audio guide qr, art museum qr, exhibition qr. NOT: scavenger hunt (game), art installation (creative).',
    e.phase = 'Use Cases',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Guides ---

MERGE (e:Entity {key: 'how-to-create-qr-code'})
SET e.type = 'GUIDE',
    e.display_name = 'How to Create QR Code',
    e.description = 'Step-by-step QR code creation guide',
    e.llm_context = 'USE: when discussing how to create QR codes, QR creation tutorials, or step-by-step QR generation. TRIGGERS: how to create qr, make qr code, qr tutorial, create qr guide, generate qr how to. NOT: design guide (aesthetics), print guide (physical output).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-design-guide'})
SET e.type = 'GUIDE',
    e.display_name = 'QR Code Design Guide',
    e.description = 'Best practices for QR code aesthetics',
    e.llm_context = 'USE: when discussing QR code design, styling, customization best practices, or visual aesthetics. TRIGGERS: qr design, qr styling, qr aesthetics, beautiful qr, custom qr design, qr appearance. NOT: creation guide (basic), print guide (output).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-print-guide'})
SET e.type = 'GUIDE',
    e.display_name = 'QR Code Print Guide',
    e.description = 'Guidelines for printing scannable QR codes',
    e.llm_context = 'USE: when discussing QR code printing, print requirements, or physical QR production. TRIGGERS: qr print, print qr guide, qr printing tips, physical qr, qr size for print, print quality qr. NOT: design guide (digital), creation guide (generation).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'dynamic-vs-static-guide'})
SET e.type = 'GUIDE',
    e.display_name = 'Dynamic vs Static Guide',
    e.description = 'When to use dynamic vs static QR codes',
    e.llm_context = 'USE: when discussing choosing between dynamic and static QR codes or comparing QR types. TRIGGERS: dynamic vs static, which qr type, qr type comparison, editable qr, trackable qr choice. NOT: comparison entity (detailed), creation guide (how-to).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-marketing-guide'})
SET e.type = 'GUIDE',
    e.display_name = 'QR Marketing Guide',
    e.description = 'Using QR codes for marketing campaigns',
    e.llm_context = 'USE: when discussing QR codes for marketing, campaign strategies, or print-to-digital marketing. TRIGGERS: qr marketing, marketing qr guide, campaign qr, advertising qr, roi qr, print to digital. NOT: restaurant guide (industry), business card guide (specific use).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-restaurant-guide'})
SET e.type = 'GUIDE',
    e.display_name = 'Restaurant QR Guide',
    e.description = 'QR codes for restaurants and cafes',
    e.llm_context = 'USE: when discussing QR codes specifically for restaurants, cafes, or food service. TRIGGERS: restaurant qr guide, menu qr, cafe qr, food service qr, dining qr, table qr guide. NOT: marketing guide (general), business card guide (networking).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-business-card-guide'})
SET e.type = 'GUIDE',
    e.display_name = 'Business Card QR Guide',
    e.description = 'Adding QR codes to business cards',
    e.llm_context = 'USE: when discussing QR codes on business cards, networking cards, or professional contact sharing. TRIGGERS: business card qr guide, vcard qr, networking card qr, professional qr, contact card qr. NOT: restaurant guide (food), marketing guide (campaigns).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-api-guide'})
SET e.type = 'GUIDE',
    e.display_name = 'QR Code API Guide',
    e.description = 'Developer guide for QR code API integration',
    e.llm_context = 'USE: when discussing QR code API documentation, developer integration, or programmatic QR generation. TRIGGERS: qr api guide, developer qr, api integration qr, programmatic qr, qr api docs. NOT: analytics guide (tracking), security guide (safety).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-analytics-guide'})
SET e.type = 'GUIDE',
    e.display_name = 'Analytics Guide',
    e.description = 'Understanding QR code scan analytics',
    e.llm_context = 'USE: when discussing QR scan analytics, tracking interpretation, or campaign measurement. TRIGGERS: qr analytics guide, scan tracking guide, qr metrics, analytics interpretation, campaign analytics. NOT: api guide (development), marketing guide (strategy).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-security-guide'})
SET e.type = 'GUIDE',
    e.display_name = 'QR Security Guide',
    e.description = 'Security best practices for QR codes',
    e.llm_context = 'USE: when discussing QR code security, phishing prevention, or safe QR practices. TRIGGERS: qr security, safe qr, qr phishing, qr safety, malicious qr, secure qr. NOT: api guide (development), analytics guide (tracking).',
    e.phase = 'Guides',
    e.created_at = datetime(),
    e.updated_at = datetime();

// --- Comparisons ---

MERGE (e:Entity {key: 'qr-code-vs-barcode'})
SET e.type = 'COMPARISON',
    e.display_name = 'QR Code vs Barcode',
    e.description = 'Comparing 2D QR codes with 1D barcodes',
    e.llm_context = 'USE: when discussing differences between QR codes and traditional barcodes or 1D vs 2D codes. TRIGGERS: qr vs barcode, barcode vs qr, 1d vs 2d, qr or barcode, qr code barcode difference. NOT: qr vs nfc (wireless), qr vs data matrix (both 2D).',
    e.phase = 'Comparisons',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'dynamic-vs-static-qr-code'})
SET e.type = 'COMPARISON',
    e.display_name = 'Dynamic vs Static QR Code',
    e.description = 'Comparing editable vs fixed QR codes',
    e.llm_context = 'USE: when discussing differences between dynamic and static QR codes or editable vs permanent QR. TRIGGERS: dynamic vs static qr, editable qr, trackable qr, static vs dynamic, permanent qr vs editable. NOT: guide (how-to), qr vs barcode (different formats).',
    e.phase = 'Comparisons',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-vs-nfc'})
SET e.type = 'COMPARISON',
    e.display_name = 'QR Code vs NFC',
    e.description = 'Comparing QR codes with NFC technology',
    e.llm_context = 'USE: when discussing differences between QR codes and NFC or visual vs tap technology. TRIGGERS: qr vs nfc, nfc vs qr, scan vs tap, qr or nfc, contactless comparison. NOT: qr vs barcode (both visual), qr vs data matrix (both 2D).',
    e.phase = 'Comparisons',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-vs-data-matrix'})
SET e.type = 'COMPARISON',
    e.display_name = 'QR Code vs Data Matrix',
    e.description = 'Comparing QR codes with Data Matrix codes',
    e.llm_context = 'USE: when discussing differences between QR codes and Data Matrix or consumer vs industrial 2D codes. TRIGGERS: qr vs data matrix, data matrix vs qr, which 2d code, qr or datamatrix. NOT: qr vs barcode (1D), qr vs nfc (wireless).',
    e.phase = 'Comparisons',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'free-vs-paid-qr-generator'})
SET e.type = 'COMPARISON',
    e.display_name = 'Free vs Paid QR Generator',
    e.description = 'Comparing free and premium QR code tools',
    e.llm_context = 'USE: when discussing free vs paid QR generators, pricing tiers, or premium QR features. TRIGGERS: free vs paid qr, qr generator pricing, premium qr, free qr limitations, paid qr benefits. NOT: platform comparison (specific tools), dynamic vs static (code types).',
    e.phase = 'Comparisons',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'qr-code-ai-vs-competitors'})
SET e.type = 'COMPARISON',
    e.display_name = 'QR Code AI vs Competitors',
    e.description = 'How QR Code AI compares to other platforms',
    e.llm_context = 'USE: when discussing QR Code AI platform comparisons or competitive analysis. TRIGGERS: qr code ai vs, compare qr platforms, qr code ai alternative, qr generator comparison, best qr platform. NOT: free vs paid (pricing), dynamic vs static (code types).',
    e.phase = 'Comparisons',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'short-link-vs-qr-code'})
SET e.type = 'COMPARISON',
    e.display_name = 'Short Link vs QR Code',
    e.description = 'When to use short links vs QR codes',
    e.llm_context = 'USE: when discussing short links vs QR codes or when to use each technology. TRIGGERS: short link vs qr, url vs qr, link or qr, bitly vs qr, when to use qr. NOT: dynamic vs static (both QR), qr vs nfc (hardware).',
    e.phase = 'Comparisons',
    e.created_at = datetime(),
    e.updated_at = datetime();

MERGE (e:Entity {key: 'spotify-code-vs-qr-code'})
SET e.type = 'COMPARISON',
    e.display_name = 'Spotify Code vs QR Code',
    e.description = 'Comparing Spotify Codes with standard QR codes',
    e.llm_context = 'USE: when discussing Spotify Codes vs standard QR codes or proprietary vs universal codes. TRIGGERS: spotify code vs qr, spotify qr, spotify code difference, proprietary qr, music qr comparison. NOT: qr vs barcode (format), platform comparison (generators).',
    e.phase = 'Comparisons',
    e.created_at = datetime(),
    e.updated_at = datetime();

// ============================================================================
// Link all Entity nodes to Project
// ============================================================================

MATCH (proj:Project {key: 'qrcode-ai'})
MATCH (e:Entity)
WHERE e.phase IS NOT NULL
MERGE (proj)-[:HAS_ENTITY]->(e);