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
