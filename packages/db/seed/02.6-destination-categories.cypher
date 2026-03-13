// NovaNet Destination Categories Seed
//
// 6 new categories for QR Code destination classification (Session 3 Decision D3).
// Adds: PLATFORM, SERVICE, CHANNEL, PROTOCOL, FORMAT, DESTINATION
//
// Based on: SESSION.md D3, KEY_FORMAT_RULES.md v2.0
// Must run after: 02.5-entity-categories.cypher
// Must run before: 10-entities-*.cypher

// =============================================================================
// DESTINATION ENTITY CATEGORIES (6 new)
// =============================================================================

// --- EXTERNAL: Social Media Platforms ---

MERGE (ec:EntityCategory {key: 'PLATFORM'})
SET ec.display_name = 'Platform',
    ec.content = 'Social media platforms (Instagram, YouTube, TikTok, Facebook)',
    ec.question = 'EXTERNAL',
    ec.sort_order = 14,
    ec.llm_context = 'USE: for social media platforms as QR destinations. TRIGGERS: platform, instagram, youtube, tiktok, facebook, social media, social network. NOT: services (use SERVICE), integrations (use INTEGRATION).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.6-destination-categories.cypher","session":"D3"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- EXTERNAL: Third-Party Services ---

MERGE (ec:EntityCategory {key: 'SERVICE'})
SET ec.display_name = 'Service',
    ec.content = 'Third-party services (Google Maps, PayPal, Airbnb, Spotify)',
    ec.question = 'EXTERNAL',
    ec.sort_order = 15,
    ec.llm_context = 'USE: for third-party services as QR destinations. TRIGGERS: service, google maps, paypal, airbnb, spotify, app store, booking. NOT: platforms (use PLATFORM), integrations (use INTEGRATION).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.6-destination-categories.cypher","session":"D3"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- HOW: Communication Channels ---

MERGE (ec:EntityCategory {key: 'CHANNEL'})
SET ec.display_name = 'Channel',
    ec.content = 'Communication channels (Email, Phone, SMS)',
    ec.question = 'HOW?',
    ec.sort_order = 16,
    ec.llm_context = 'USE: for direct communication channels as QR destinations. TRIGGERS: channel, email, phone, sms, call, message, contact. NOT: protocols (use PROTOCOL), platforms (use PLATFORM).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.6-destination-categories.cypher","session":"D3"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- HOW: Technical Protocols ---

MERGE (ec:EntityCategory {key: 'PROTOCOL'})
SET ec.display_name = 'Protocol',
    ec.content = 'Technical protocols (WiFi, Pix payment)',
    ec.question = 'HOW?',
    ec.sort_order = 17,
    ec.llm_context = 'USE: for technical protocols encoded in QR. TRIGGERS: protocol, wifi, pix, network, connection, technical. NOT: formats (use FORMAT), channels (use CHANNEL).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.6-destination-categories.cypher","session":"D3"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- WHAT: Data Formats ---

MERGE (ec:EntityCategory {key: 'FORMAT'})
SET ec.display_name = 'Format',
    ec.content = 'Data encoding formats (vCard, MeCard, PDF, URL, Plain Text)',
    ec.question = 'WHAT?',
    ec.sort_order = 18,
    ec.llm_context = 'USE: for data encoding formats in QR. TRIGGERS: format, vcard, mecard, pdf, url, text, encoding. NOT: protocols (use PROTOCOL), content types (use CONTENT_TYPE).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.6-destination-categories.cypher","session":"D3"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// --- WHERE: Meta-Category for All Destinations ---

MERGE (ec:EntityCategory {key: 'DESTINATION'})
SET ec.display_name = 'Destination',
    ec.content = 'Meta-category for all QR code destinations (platforms, services, channels, protocols, formats)',
    ec.question = 'WHERE?',
    ec.sort_order = 19,
    ec.llm_context = 'USE: as meta-category for ANY QR destination. All PLATFORM, SERVICE, CHANNEL, PROTOCOL, FORMAT entities also belong to DESTINATION. TRIGGERS: destination, target, endpoint, where QR points. NOT: as sole category (always combine with specific type).',
    ec.node_class = 'EntityCategory',
    ec.provenance = '{"source":"seed:content","version":"v0.19.0","file":"02.6-destination-categories.cypher","session":"D3"}',
    ec.created_at = coalesce(ec.created_at, datetime()),
    ec.updated_at = datetime();

// =============================================================================
// VERIFICATION
// =============================================================================

// Count new categories
MATCH (ec:EntityCategory)
WHERE ec.key IN ['PLATFORM', 'SERVICE', 'CHANNEL', 'PROTOCOL', 'FORMAT', 'DESTINATION']
RETURN count(ec) AS destination_categories_count;
