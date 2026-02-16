// packages/db/seed/31-en-us-seo-keywords.cypher
// v0.13.0 - en-US SEO Keywords with Entity Classification
//
// ADR-029: *Native Pattern (EntityNative with entity_type)
// ADR-030: Entity owns semantics, Page owns URL
// ADR-032: TARGETS arc with semantic_coef for convergence boost
//
// Entity Types:
//   TOOL   = qr-code-generator, free-qr-code-generator (the product)
//   ACTION = create-qr-code, make-qr-code, scan-qr-code (user intent, verb)
//   OBJECT = qr-code-instagram, qr-code-wifi, etc. (the created thing)
//   GUIDE  = how-to-create-qr-code, qr-code-design-guide (educational)
//
// Volume data: Estimated US monthly search volumes (2026)

// ============================================================================
// 1. EntityNative en-US: TOOL entities
// ============================================================================

MATCH (l:Locale {key: 'en-US'})

// qr-code-generator (TOOL)
MATCH (e1:Entity {key: 'qr-code-generator'})
MERGE (en1:EntityNative {key: 'entity:qr-code-generator@en-US'})
SET en1.locale = 'en-US',
    en1.title = 'QR Code Generator',
    en1.description = 'Free online tool to create custom QR codes instantly',
    en1.entity_type = 'TOOL',
    en1.slug_terms = ['qr', 'code', 'generator', 'create', 'make', 'free'],
    en1.updated_at = datetime()
MERGE (e1)-[:HAS_NATIVE]->(en1)
MERGE (en1)-[:FOR_LOCALE]->(l)

// batch-qr-generator (TOOL)
WITH l
MATCH (e2:Entity {key: 'batch-qr-generator'})
MERGE (en2:EntityNative {key: 'entity:batch-qr-generator@en-US'})
SET en2.locale = 'en-US',
    en2.title = 'Bulk QR Code Generator',
    en2.description = 'Generate multiple QR codes at once from spreadsheet data',
    en2.entity_type = 'TOOL',
    en2.slug_terms = ['bulk', 'batch', 'qr', 'generator', 'multiple', 'mass'],
    en2.updated_at = datetime()
MERGE (e2)-[:HAS_NATIVE]->(en2)
MERGE (en2)-[:FOR_LOCALE]->(l)

// wifi-qr-generator (TOOL)
WITH l
MATCH (e3:Entity {key: 'wifi-qr-generator'})
MERGE (en3:EntityNative {key: 'entity:wifi-qr-generator@en-US'})
SET en3.locale = 'en-US',
    en3.title = 'WiFi QR Code Generator',
    en3.description = 'Create QR codes for WiFi network sharing',
    en3.entity_type = 'TOOL',
    en3.slug_terms = ['wifi', 'qr', 'generator', 'network', 'password', 'share'],
    en3.updated_at = datetime()
MERGE (e3)-[:HAS_NATIVE]->(en3)
MERGE (en3)-[:FOR_LOCALE]->(l)

// vcard-generator (TOOL)
WITH l
MATCH (e4:Entity {key: 'vcard-generator'})
MERGE (en4:EntityNative {key: 'entity:vcard-generator@en-US'})
SET en4.locale = 'en-US',
    en4.title = 'vCard QR Code Generator',
    en4.description = 'Create digital business card QR codes with contact info',
    en4.entity_type = 'TOOL',
    en4.slug_terms = ['vcard', 'business', 'card', 'contact', 'qr', 'generator'],
    en4.updated_at = datetime()
MERGE (e4)-[:HAS_NATIVE]->(en4)
MERGE (en4)-[:FOR_LOCALE]->(l)

// qr-code-scanner (TOOL)
WITH l
MATCH (e5:Entity {key: 'qr-code-scanner'})
MERGE (en5:EntityNative {key: 'entity:qr-code-scanner@en-US'})
SET en5.locale = 'en-US',
    en5.title = 'QR Code Scanner',
    en5.description = 'Free online QR code reader and scanner',
    en5.entity_type = 'TOOL',
    en5.slug_terms = ['qr', 'code', 'scanner', 'reader', 'scan', 'decode'],
    en5.updated_at = datetime()
MERGE (e5)-[:HAS_NATIVE]->(en5)
MERGE (en5)-[:FOR_LOCALE]->(l)
;

// ============================================================================
// 2. EntityNative en-US: ACTION entities
// ============================================================================

MATCH (l:Locale {key: 'en-US'})

// create-qr-code (ACTION)
MATCH (e1:Entity {key: 'create-qr-code'})
MERGE (en1:EntityNative {key: 'entity:create-qr-code@en-US'})
SET en1.locale = 'en-US',
    en1.title = 'Create QR Code',
    en1.description = 'Generate a custom QR code for any purpose',
    en1.entity_type = 'ACTION',
    en1.slug_terms = ['create', 'make', 'generate', 'qr', 'code'],
    en1.updated_at = datetime()
MERGE (e1)-[:HAS_NATIVE]->(en1)
MERGE (en1)-[:FOR_LOCALE]->(l)

// scan-qr-code (ACTION)
WITH l
MATCH (e2:Entity {key: 'scan-qr-code'})
MERGE (en2:EntityNative {key: 'entity:scan-qr-code@en-US'})
SET en2.locale = 'en-US',
    en2.title = 'Scan QR Code',
    en2.description = 'Read and decode QR codes with your device',
    en2.entity_type = 'ACTION',
    en2.slug_terms = ['scan', 'read', 'decode', 'qr', 'code'],
    en2.updated_at = datetime()
MERGE (e2)-[:HAS_NATIVE]->(en2)
MERGE (en2)-[:FOR_LOCALE]->(l)

// download-qr-code (ACTION)
WITH l
MATCH (e3:Entity {key: 'download-qr-code'})
MERGE (en3:EntityNative {key: 'entity:download-qr-code@en-US'})
SET en3.locale = 'en-US',
    en3.title = 'Download QR Code',
    en3.description = 'Save your QR code in PNG, SVG, or PDF format',
    en3.entity_type = 'ACTION',
    en3.slug_terms = ['download', 'save', 'export', 'qr', 'code', 'png', 'svg'],
    en3.updated_at = datetime()
MERGE (e3)-[:HAS_NATIVE]->(en3)
MERGE (en3)-[:FOR_LOCALE]->(l)

// print-qr-code (ACTION)
WITH l
MATCH (e4:Entity {key: 'print-qr-code'})
MERGE (en4:EntityNative {key: 'entity:print-qr-code@en-US'})
SET en4.locale = 'en-US',
    en4.title = 'Print QR Code',
    en4.description = 'Print your QR code on business cards, flyers, or products',
    en4.entity_type = 'ACTION',
    en4.slug_terms = ['print', 'qr', 'code', 'business', 'card', 'flyer'],
    en4.updated_at = datetime()
MERGE (e4)-[:HAS_NATIVE]->(en4)
MERGE (en4)-[:FOR_LOCALE]->(l)

// customize-qr-code (ACTION)
WITH l
MATCH (e5:Entity {key: 'customize-qr-code'})
MERGE (en5:EntityNative {key: 'entity:customize-qr-code@en-US'})
SET en5.locale = 'en-US',
    en5.title = 'Customize QR Code',
    en5.description = 'Add colors, logos, and design to your QR code',
    en5.entity_type = 'ACTION',
    en5.slug_terms = ['customize', 'design', 'color', 'logo', 'qr', 'code'],
    en5.updated_at = datetime()
MERGE (e5)-[:HAS_NATIVE]->(en5)
MERGE (en5)-[:FOR_LOCALE]->(l)

// share-qr-code (ACTION)
WITH l
MATCH (e6:Entity {key: 'share-qr-code'})
MERGE (en6:EntityNative {key: 'entity:share-qr-code@en-US'})
SET en6.locale = 'en-US',
    en6.title = 'Share QR Code',
    en6.description = 'Share your QR code via email, social media, or link',
    en6.entity_type = 'ACTION',
    en6.slug_terms = ['share', 'send', 'qr', 'code', 'email', 'link'],
    en6.updated_at = datetime()
MERGE (e6)-[:HAS_NATIVE]->(en6)
MERGE (en6)-[:FOR_LOCALE]->(l)
;

// ============================================================================
// 3. EntityNative en-US: OBJECT entities (QR code types)
// ============================================================================

MATCH (l:Locale {key: 'en-US'})
UNWIND [
  // Social Media QR Codes
  {key: 'qr-code-instagram', title: 'Instagram QR Code', desc: 'QR code linking to your Instagram profile', type: 'OBJECT', terms: ['instagram', 'qr', 'code', 'social', 'profile']},
  {key: 'qr-code-facebook', title: 'Facebook QR Code', desc: 'QR code for Facebook page or profile', type: 'OBJECT', terms: ['facebook', 'qr', 'code', 'social', 'page']},
  {key: 'qr-code-tiktok', title: 'TikTok QR Code', desc: 'QR code for TikTok profile', type: 'OBJECT', terms: ['tiktok', 'qr', 'code', 'social', 'video']},
  {key: 'qr-code-youtube', title: 'YouTube QR Code', desc: 'QR code for YouTube channel or video', type: 'OBJECT', terms: ['youtube', 'qr', 'code', 'video', 'channel']},
  {key: 'qr-code-linkedin', title: 'LinkedIn QR Code', desc: 'QR code for LinkedIn profile', type: 'OBJECT', terms: ['linkedin', 'qr', 'code', 'professional', 'profile']},
  {key: 'qr-code-twitter', title: 'Twitter QR Code', desc: 'QR code for Twitter/X profile', type: 'OBJECT', terms: ['twitter', 'x', 'qr', 'code', 'social']},
  {key: 'qr-code-whatsapp', title: 'WhatsApp QR Code', desc: 'QR code to start WhatsApp chat', type: 'OBJECT', terms: ['whatsapp', 'qr', 'code', 'chat', 'message']},

  // Utility QR Codes
  {key: 'qr-code-wifi', title: 'WiFi QR Code', desc: 'QR code to share WiFi credentials', type: 'OBJECT', terms: ['wifi', 'qr', 'code', 'network', 'password']},
  {key: 'qr-code-url', title: 'URL QR Code', desc: 'QR code linking to any website', type: 'OBJECT', terms: ['url', 'link', 'website', 'qr', 'code']},
  {key: 'qr-code-vcard', title: 'vCard QR Code', desc: 'Digital business card QR code', type: 'OBJECT', terms: ['vcard', 'business', 'card', 'contact', 'qr']},
  {key: 'qr-code-pdf', title: 'PDF QR Code', desc: 'QR code linking to PDF document', type: 'OBJECT', terms: ['pdf', 'document', 'file', 'qr', 'code']},
  {key: 'qr-code-email', title: 'Email QR Code', desc: 'QR code to send pre-filled email', type: 'OBJECT', terms: ['email', 'mail', 'qr', 'code', 'message']},
  {key: 'qr-code-sms', title: 'SMS QR Code', desc: 'QR code to send pre-filled SMS', type: 'OBJECT', terms: ['sms', 'text', 'message', 'qr', 'code']},
  {key: 'qr-code-phone', title: 'Phone QR Code', desc: 'QR code to dial phone number', type: 'OBJECT', terms: ['phone', 'call', 'dial', 'qr', 'code']},

  // Business QR Codes
  {key: 'qr-code-menu', title: 'Menu QR Code', desc: 'Digital restaurant menu QR code', type: 'OBJECT', terms: ['menu', 'restaurant', 'food', 'qr', 'code']},
  {key: 'qr-code-business-card', title: 'Business Card QR Code', desc: 'QR code for digital business card', type: 'OBJECT', terms: ['business', 'card', 'contact', 'qr', 'code']},
  {key: 'qr-code-payment', title: 'Payment QR Code', desc: 'QR code for payments', type: 'OBJECT', terms: ['payment', 'pay', 'money', 'qr', 'code']},
  {key: 'qr-code-review', title: 'Review QR Code', desc: 'QR code for Google reviews', type: 'OBJECT', terms: ['review', 'google', 'feedback', 'qr', 'code']},
  {key: 'qr-code-coupon', title: 'Coupon QR Code', desc: 'QR code for discount coupons', type: 'OBJECT', terms: ['coupon', 'discount', 'promo', 'qr', 'code']},

  // Content QR Codes
  {key: 'qr-code-video', title: 'Video QR Code', desc: 'QR code linking to video content', type: 'OBJECT', terms: ['video', 'youtube', 'vimeo', 'qr', 'code']},
  {key: 'qr-code-image', title: 'Image QR Code', desc: 'QR code linking to image gallery', type: 'OBJECT', terms: ['image', 'photo', 'gallery', 'qr', 'code']},
  {key: 'qr-code-audio', title: 'Audio QR Code', desc: 'QR code linking to audio content', type: 'OBJECT', terms: ['audio', 'music', 'podcast', 'qr', 'code']},

  // Location QR Codes
  {key: 'qr-code-location', title: 'Location QR Code', desc: 'QR code for GPS location', type: 'OBJECT', terms: ['location', 'maps', 'gps', 'qr', 'code']},
  {key: 'qr-code-google-maps', title: 'Google Maps QR Code', desc: 'QR code for Google Maps location', type: 'OBJECT', terms: ['google', 'maps', 'location', 'qr', 'code']},

  // Special QR Codes
  {key: 'dynamic-qr-code', title: 'Dynamic QR Code', desc: 'Editable QR code with tracking', type: 'OBJECT', terms: ['dynamic', 'editable', 'trackable', 'qr', 'code']},
  {key: 'static-qr-code', title: 'Static QR Code', desc: 'Permanent unchangeable QR code', type: 'OBJECT', terms: ['static', 'permanent', 'qr', 'code']},
  {key: 'custom-qr-code', title: 'Custom QR Code', desc: 'Personalized QR code with branding', type: 'OBJECT', terms: ['custom', 'branded', 'design', 'qr', 'code']}
] AS obj
MATCH (e:Entity {key: obj.key})
MERGE (en:EntityNative {key: 'entity:' + obj.key + '@en-US'})
SET en.locale = 'en-US',
    en.title = obj.title,
    en.description = obj.desc,
    en.entity_type = obj.type,
    en.slug_terms = obj.terms,
    en.updated_at = datetime()
MERGE (e)-[:HAS_NATIVE]->(en)
MERGE (en)-[:FOR_LOCALE]->(l)
;

// ============================================================================
// 4. EntityNative en-US: GUIDE entities (Educational content)
// ============================================================================

MATCH (l:Locale {key: 'en-US'})
UNWIND [
  {key: 'how-to-create-qr-code', title: 'How to Create a QR Code', desc: 'Step-by-step guide to creating QR codes', type: 'GUIDE', terms: ['how', 'to', 'create', 'qr', 'code', 'guide']},
  {key: 'qr-code-design-guide', title: 'QR Code Design Guide', desc: 'Best practices for QR code design', type: 'GUIDE', terms: ['design', 'guide', 'qr', 'code', 'best', 'practices']},
  {key: 'qr-code-marketing-guide', title: 'QR Code Marketing Guide', desc: 'How to use QR codes in marketing', type: 'GUIDE', terms: ['marketing', 'guide', 'qr', 'code', 'campaign']},
  {key: 'qr-code-print-guide', title: 'QR Code Print Guide', desc: 'Guide to printing QR codes correctly', type: 'GUIDE', terms: ['print', 'guide', 'qr', 'code', 'size', 'resolution']},
  {key: 'dynamic-vs-static-guide', title: 'Dynamic vs Static QR Codes', desc: 'Comparison guide for QR code types', type: 'GUIDE', terms: ['dynamic', 'static', 'comparison', 'guide', 'qr']}
] AS obj
MATCH (e:Entity {key: obj.key})
MERGE (en:EntityNative {key: 'entity:' + obj.key + '@en-US'})
SET en.locale = 'en-US',
    en.title = obj.title,
    en.description = obj.desc,
    en.entity_type = obj.type,
    en.slug_terms = obj.terms,
    en.updated_at = datetime()
MERGE (e)-[:HAS_NATIVE]->(en)
MERGE (en)-[:FOR_LOCALE]->(l)
;

// ============================================================================
// 5. SEO Keywords en-US: Tier 1 (100K+ monthly volume)
// ============================================================================

MATCH (l:Locale {key: 'en-US'})

// TIER 1 Keywords - Highest volume
UNWIND [
  // TOOL keywords
  {key: 'seo-qr-code-generator', phrase: 'qr code generator', vol: 150000, slug: 'qr-code-generator', entity: 'qr-code-generator', entity_type: 'TOOL', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-free-qr-code-generator', phrase: 'free qr code generator', vol: 180000, slug: 'free-qr-code-generator', entity: 'qr-code-generator', entity_type: 'TOOL', sem_coef: 0.95, rank: 'primary'},
  {key: 'seo-qr-code-maker', phrase: 'qr code maker', vol: 45000, slug: 'qr-code-maker', entity: 'qr-code-generator', entity_type: 'TOOL', sem_coef: 0.95, rank: 'primary'},

  // ACTION keywords
  {key: 'seo-create-qr-code', phrase: 'create qr code', vol: 120000, slug: 'create-qr-code', entity: 'create-qr-code', entity_type: 'ACTION', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-make-qr-code', phrase: 'make qr code', vol: 38000, slug: 'make-qr-code', entity: 'create-qr-code', entity_type: 'ACTION', sem_coef: 0.95, rank: 'primary'},
  {key: 'seo-generate-qr-code', phrase: 'generate qr code', vol: 25000, slug: 'generate-qr-code', entity: 'create-qr-code', entity_type: 'ACTION', sem_coef: 0.95, rank: 'primary'},

  // OBJECT keywords (generic)
  {key: 'seo-qr-code', phrase: 'qr code', vol: 2200000, slug: 'qr-code', entity: 'qr-code', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-free-qr-code', phrase: 'free qr code', vol: 90000, slug: 'free-qr-code', entity: 'qr-code', entity_type: 'OBJECT', sem_coef: 0.9, rank: 'primary'}
] AS kw
MERGE (k:SEOKeyword {key: kw.key})
SET k.phrase = kw.phrase,
    k.volume = kw.vol,
    k.locale = 'en-US',
    k.slug_form = kw.slug,
    k.entity_type = kw.entity_type,
    k.updated_at = datetime()

// Link to EntityNative with TARGETS (rank + semantic_coef)
WITH kw, k, l
MATCH (en:EntityNative {key: 'entity:' + kw.entity + '@en-US'})
MERGE (en)-[r:TARGETS]->(k)
SET r.rank = kw.rank,
    r.semantic_coef = kw.sem_coef
;

// ============================================================================
// 6. SEO Keywords en-US: Tier 2 (20K-100K monthly volume)
// ============================================================================

MATCH (l:Locale {key: 'en-US'})

UNWIND [
  // Social Media QR keywords
  {key: 'seo-qr-code-instagram', phrase: 'qr code for instagram', vol: 42000, slug: 'instagram', entity: 'qr-code-instagram', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-instagram-qr-code', phrase: 'instagram qr code', vol: 38000, slug: 'instagram-qr-code', entity: 'qr-code-instagram', entity_type: 'OBJECT', sem_coef: 0.95, rank: 'primary'},
  {key: 'seo-qr-code-facebook', phrase: 'facebook qr code', vol: 28000, slug: 'facebook', entity: 'qr-code-facebook', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-tiktok', phrase: 'tiktok qr code', vol: 32000, slug: 'tiktok', entity: 'qr-code-tiktok', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-youtube', phrase: 'youtube qr code', vol: 25000, slug: 'youtube', entity: 'qr-code-youtube', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-whatsapp', phrase: 'whatsapp qr code', vol: 55000, slug: 'whatsapp', entity: 'qr-code-whatsapp', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},

  // Utility QR keywords
  {key: 'seo-wifi-qr-code', phrase: 'wifi qr code', vol: 35000, slug: 'wifi', entity: 'qr-code-wifi', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-wifi-generator', phrase: 'wifi qr code generator', vol: 22000, slug: 'wifi-generator', entity: 'wifi-qr-generator', entity_type: 'TOOL', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-menu', phrase: 'qr code menu', vol: 26000, slug: 'menu', entity: 'qr-code-menu', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-menu-qr-code', phrase: 'menu qr code', vol: 24000, slug: 'menu-qr-code', entity: 'qr-code-menu', entity_type: 'OBJECT', sem_coef: 0.95, rank: 'primary'},
  {key: 'seo-qr-code-pdf', phrase: 'qr code for pdf', vol: 20000, slug: 'pdf', entity: 'qr-code-pdf', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},

  // Business QR keywords
  {key: 'seo-qr-code-business-card', phrase: 'qr code business card', vol: 28000, slug: 'business-card', entity: 'qr-code-business-card', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-vcard-qr-code', phrase: 'vcard qr code', vol: 18000, slug: 'vcard', entity: 'qr-code-vcard', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-google-review', phrase: 'qr code google review', vol: 22000, slug: 'google-review', entity: 'qr-code-review', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},

  // Scanner keywords
  {key: 'seo-qr-code-scanner', phrase: 'qr code scanner', vol: 85000, slug: 'scanner', entity: 'qr-code-scanner', entity_type: 'TOOL', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-scan-qr-code', phrase: 'scan qr code', vol: 65000, slug: 'scan', entity: 'scan-qr-code', entity_type: 'ACTION', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-reader', phrase: 'qr code reader', vol: 45000, slug: 'reader', entity: 'qr-code-scanner', entity_type: 'TOOL', sem_coef: 0.95, rank: 'primary'},

  // Dynamic QR keywords
  {key: 'seo-dynamic-qr-code', phrase: 'dynamic qr code', vol: 28000, slug: 'dynamic', entity: 'dynamic-qr-code', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-dynamic-qr-code-generator', phrase: 'dynamic qr code generator', vol: 15000, slug: 'dynamic-generator', entity: 'qr-code-generator', entity_type: 'TOOL', sem_coef: 0.9, rank: 'secondary'}
] AS kw
MERGE (k:SEOKeyword {key: kw.key})
SET k.phrase = kw.phrase,
    k.volume = kw.vol,
    k.locale = 'en-US',
    k.slug_form = kw.slug,
    k.entity_type = kw.entity_type,
    k.updated_at = datetime()

WITH kw, k, l
MATCH (en:EntityNative {key: 'entity:' + kw.entity + '@en-US'})
MERGE (en)-[r:TARGETS]->(k)
SET r.rank = kw.rank,
    r.semantic_coef = kw.sem_coef
;

// ============================================================================
// 7. SEO Keywords en-US: Tier 3 (5K-20K monthly volume)
// ============================================================================

MATCH (l:Locale {key: 'en-US'})

UNWIND [
  // More social platforms
  {key: 'seo-qr-code-linkedin', phrase: 'linkedin qr code', vol: 15000, slug: 'linkedin', entity: 'qr-code-linkedin', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-twitter', phrase: 'twitter qr code', vol: 12000, slug: 'twitter', entity: 'qr-code-twitter', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-spotify', phrase: 'spotify qr code', vol: 18000, slug: 'spotify', entity: 'qr-code-spotify', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},

  // Utility keywords
  {key: 'seo-qr-code-url', phrase: 'qr code for url', vol: 15000, slug: 'url', entity: 'qr-code-url', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-link', phrase: 'qr code link', vol: 22000, slug: 'link', entity: 'qr-code-url', entity_type: 'OBJECT', sem_coef: 0.95, rank: 'primary'},
  {key: 'seo-qr-code-email', phrase: 'email qr code', vol: 8000, slug: 'email', entity: 'qr-code-email', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-sms', phrase: 'sms qr code', vol: 5000, slug: 'sms', entity: 'qr-code-sms', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-phone', phrase: 'phone number qr code', vol: 8000, slug: 'phone', entity: 'qr-code-phone', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},

  // Location keywords
  {key: 'seo-qr-code-location', phrase: 'location qr code', vol: 12000, slug: 'location', entity: 'qr-code-location', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-google-maps', phrase: 'google maps qr code', vol: 18000, slug: 'google-maps', entity: 'qr-code-google-maps', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},

  // Content keywords
  {key: 'seo-qr-code-video', phrase: 'video qr code', vol: 12000, slug: 'video', entity: 'qr-code-video', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-image', phrase: 'image qr code', vol: 8000, slug: 'image', entity: 'qr-code-image', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},

  // Customization keywords
  {key: 'seo-custom-qr-code', phrase: 'custom qr code', vol: 25000, slug: 'custom', entity: 'custom-qr-code', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-with-logo', phrase: 'qr code with logo', vol: 35000, slug: 'with-logo', entity: 'qr-code-with-logo', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-design', phrase: 'qr code design', vol: 18000, slug: 'design', entity: 'customize-qr-code', entity_type: 'ACTION', sem_coef: 0.9, rank: 'primary'},

  // Action keywords
  {key: 'seo-download-qr-code', phrase: 'download qr code', vol: 15000, slug: 'download', entity: 'download-qr-code', entity_type: 'ACTION', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-print-qr-code', phrase: 'print qr code', vol: 10000, slug: 'print', entity: 'print-qr-code', entity_type: 'ACTION', sem_coef: 1.0, rank: 'primary'},

  // Payment keywords
  {key: 'seo-qr-code-payment', phrase: 'qr code payment', vol: 22000, slug: 'payment', entity: 'qr-code-payment', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-venmo', phrase: 'venmo qr code', vol: 15000, slug: 'venmo', entity: 'qr-code-venmo', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-paypal', phrase: 'paypal qr code', vol: 12000, slug: 'paypal', entity: 'qr-code-paypal', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},

  // Event/Special keywords
  {key: 'seo-qr-code-wedding', phrase: 'wedding qr code', vol: 18000, slug: 'wedding', entity: 'qr-code-wedding', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-ticket', phrase: 'qr code ticket', vol: 12000, slug: 'ticket', entity: 'qr-code-ticket', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-qr-code-coupon', phrase: 'qr code coupon', vol: 8000, slug: 'coupon', entity: 'qr-code-coupon', entity_type: 'OBJECT', sem_coef: 1.0, rank: 'primary'},

  // Guide keywords
  {key: 'seo-how-to-create-qr-code', phrase: 'how to create qr code', vol: 35000, slug: 'how-to-create', entity: 'how-to-create-qr-code', entity_type: 'GUIDE', sem_coef: 1.0, rank: 'primary'},
  {key: 'seo-how-to-make-qr-code', phrase: 'how to make qr code', vol: 28000, slug: 'how-to-make', entity: 'how-to-create-qr-code', entity_type: 'GUIDE', sem_coef: 0.95, rank: 'primary'}
] AS kw
MERGE (k:SEOKeyword {key: kw.key})
SET k.phrase = kw.phrase,
    k.volume = kw.vol,
    k.locale = 'en-US',
    k.slug_form = kw.slug,
    k.entity_type = kw.entity_type,
    k.updated_at = datetime()

WITH kw, k, l
MATCH (en:EntityNative {key: 'entity:' + kw.entity + '@en-US'})
MERGE (en)-[r:TARGETS]->(k)
SET r.rank = kw.rank,
    r.semantic_coef = kw.sem_coef
;

// ============================================================================
// 8. MULTI-ENTITY KEYWORD TARGETING (Convergence Boost)
// ============================================================================
// High-volume keywords linked to MULTIPLE entities for convergence_boost
// Formula: score = volume × semantic_coef × (1 + N × 0.2)

// "create qr code" → both ACTION (primary) and TOOL (secondary)
MATCH (k:SEOKeyword {key: 'seo-create-qr-code'})
MATCH (en_tool:EntityNative {key: 'entity:qr-code-generator@en-US'})
MERGE (en_tool)-[r:TARGETS]->(k)
SET r.rank = 'secondary',
    r.semantic_coef = 0.95
;

// "qr code generator" → both TOOL (primary) and ACTION (secondary)
MATCH (k:SEOKeyword {key: 'seo-qr-code-generator'})
MATCH (en_action:EntityNative {key: 'entity:create-qr-code@en-US'})
MERGE (en_action)-[r:TARGETS]->(k)
SET r.rank = 'secondary',
    r.semantic_coef = 0.95
;

// "make qr code" → both ACTION (primary) and TOOL (secondary)
MATCH (k:SEOKeyword {key: 'seo-make-qr-code'})
MATCH (en_tool:EntityNative {key: 'entity:qr-code-generator@en-US'})
MERGE (en_tool)-[r:TARGETS]->(k)
SET r.rank = 'secondary',
    r.semantic_coef = 0.9
;

// "free qr code generator" → TOOL + ACTION + OBJECT
MATCH (k:SEOKeyword {key: 'seo-free-qr-code-generator'})
MATCH (en_action:EntityNative {key: 'entity:create-qr-code@en-US'})
MATCH (en_qr:EntityNative {key: 'entity:qr-code@en-US'})
MERGE (en_action)-[r1:TARGETS]->(k)
SET r1.rank = 'secondary', r1.semantic_coef = 0.9
// Note: qr-code entity might not exist, handle gracefully
;

// "wifi qr code" → both OBJECT (primary) and TOOL (secondary)
MATCH (k:SEOKeyword {key: 'seo-wifi-qr-code'})
MATCH (en_tool:EntityNative {key: 'entity:wifi-qr-generator@en-US'})
MERGE (en_tool)-[r:TARGETS]->(k)
SET r.rank = 'secondary',
    r.semantic_coef = 0.85
;

// "qr code menu" → OBJECT + restaurant entity
MATCH (k:SEOKeyword {key: 'seo-qr-code-menu'})
MATCH (en_rest:EntityNative {key: 'entity:menu-restaurant@en-US'})
MERGE (en_rest)-[r:TARGETS]->(k)
SET r.rank = 'secondary',
    r.semantic_coef = 0.8
;

// ============================================================================
// 9. SEMANTIC_LINK ARCS: TOOL → ACTION (USED_FOR)
// ============================================================================

// qr-code-generator USED_FOR create-qr-code (en-US)
MATCH (tool:Entity {key: 'qr-code-generator'})
MATCH (action:Entity {key: 'create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.95,
    r.temperature = 0.9,
    r.llm_context = 'TOOL → ACTION: The generator is used to create QR codes'
;

// wifi-qr-generator USED_FOR create-qr-code
MATCH (tool:Entity {key: 'wifi-qr-generator'})
MATCH (action:Entity {key: 'create-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.9,
    r.llm_context = 'TOOL → ACTION: WiFi generator is used to create WiFi QR codes'
;

// qr-code-scanner USED_FOR scan-qr-code
MATCH (tool:Entity {key: 'qr-code-scanner'})
MATCH (action:Entity {key: 'scan-qr-code'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'used_for'}]->(action)
SET r.strength = 0.95,
    r.llm_context = 'TOOL → ACTION: Scanner is used to scan/read QR codes'
;

// ============================================================================
// 10. SEMANTIC_LINK ARCS: TOOL → OBJECTs (CREATES)
// ============================================================================

// qr-code-generator CREATES various QR code objects
MATCH (tool:Entity {key: 'qr-code-generator'})
MATCH (obj:Entity)
WHERE obj.key IN [
  'qr-code-instagram', 'qr-code-wifi', 'qr-code-vcard', 'qr-code-menu',
  'qr-code-pdf', 'qr-code-url', 'qr-code-business-card', 'dynamic-qr-code'
]
MERGE (tool)-[r:SEMANTIC_LINK {type: 'creates'}]->(obj)
SET r.strength = 0.85,
    r.temperature = 0.85,
    r.llm_context = 'TOOL → OBJECT: The generator creates this type of QR code'
;

// wifi-qr-generator CREATES qr-code-wifi specifically
MATCH (tool:Entity {key: 'wifi-qr-generator'})
MATCH (obj:Entity {key: 'qr-code-wifi'})
MERGE (tool)-[r:SEMANTIC_LINK {type: 'creates'}]->(obj)
SET r.strength = 0.95,
    r.llm_context = 'TOOL → OBJECT: WiFi generator specifically creates WiFi QR codes'
;

// ============================================================================
// 11. STATS QUERY (Run after import to verify)
// ============================================================================
// MATCH (k:SEOKeyword) WHERE k.locale = 'en-US'
// RETURN count(k) AS total_keywords, sum(k.volume) AS total_volume
//
// MATCH (en:EntityNative) WHERE en.locale = 'en-US'
// RETURN count(en) AS total_entity_native
//
// MATCH ()-[r:TARGETS]->(:SEOKeyword {locale: 'en-US'})
// RETURN count(r) AS total_targets,
//        count(CASE WHEN r.rank IS NOT NULL THEN 1 END) AS with_rank,
//        count(CASE WHEN r.semantic_coef IS NOT NULL THEN 1 END) AS with_coef
