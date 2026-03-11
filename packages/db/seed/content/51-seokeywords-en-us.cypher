// ═══════════════════════════════════════════════════════════════════════════════
// 51-seokeywords-en-us.cypher — SEOKeyword nodes for en-US locale
// v0.17.3 - Updated with Ahrefs data (2026-03-08)
// v0.19.0 - Standard properties: node_class + created_by (ADR-042)
// ═══════════════════════════════════════════════════════════════════════════════
//
// Key format: seo:{slug}@{locale} (ADR-029)
// All keywords have trait = 'imported' (ADR-024)
// All keywords link to Locale via FOR_LOCALE arc
//
// Data: 61 keywords with Ahrefs metrics from 2026-03-08
// ═══════════════════════════════════════════════════════════════════════════════

// =============================================================================
// PRIMARY KEYWORDS (with Ahrefs data - highest volume)
// =============================================================================

// "qr code generator" — 708K SV, KD93, Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-generator@en-US'})
ON CREATE SET
  kw.display_name = 'qr code generator',
  kw.content = 'SEO keyword for en-US: qr code generator',
  kw.value = 'qr code generator',
  kw.slug_form = 'qr-code-generator',
  kw.locale_key = 'en-US',
  kw.volume = 708000,
  kw.difficulty = 93,
  kw.cpc = 1.10,
  kw.traffic_potential = 940000,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 708000,
  kw.difficulty = 93,
  kw.traffic_potential = 940000,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code" — 303K SV, KD94, Informational
MERGE (kw:SEOKeyword {key: 'seo:qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'qr code',
  kw.content = 'SEO keyword for en-US: qr code',
  kw.value = 'qr code',
  kw.slug_form = 'qr-code',
  kw.locale_key = 'en-US',
  kw.volume = 303000,
  kw.difficulty = 94,
  kw.cpc = 0.90,
  kw.traffic_potential = 450000,
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 303000,
  kw.difficulty = 94,
  kw.traffic_potential = 450000,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code scanner" — 145K SV, KD61, Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-scanner@en-US'})
ON CREATE SET
  kw.display_name = 'qr code scanner',
  kw.content = 'SEO keyword for en-US: qr code scanner',
  kw.value = 'qr code scanner',
  kw.slug_form = 'qr-code-scanner',
  kw.locale_key = 'en-US',
  kw.volume = 145000,
  kw.difficulty = 61,
  kw.cpc = 0.80,
  kw.traffic_potential = 200000,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 145000,
  kw.difficulty = 61,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "free qr code generator" — 117K SV, KD92, Transactional
MERGE (kw:SEOKeyword {key: 'seo:free-qr-code-generator@en-US'})
ON CREATE SET
  kw.display_name = 'free qr code generator',
  kw.content = 'SEO keyword for en-US: free qr code generator',
  kw.value = 'free qr code generator',
  kw.slug_form = 'free-qr-code-generator',
  kw.locale_key = 'en-US',
  kw.volume = 117000,
  kw.difficulty = 92,
  kw.cpc = 1.00,
  kw.traffic_potential = 160000,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 117000,
  kw.difficulty = 92,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code generator free" — 67K SV, KD90, Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-generator-free@en-US'})
ON CREATE SET
  kw.display_name = 'qr code generator free',
  kw.content = 'SEO keyword for en-US: qr code generator free',
  kw.value = 'qr code generator free',
  kw.slug_form = 'qr-code-generator-free',
  kw.locale_key = 'en-US',
  kw.volume = 67000,
  kw.difficulty = 90,
  kw.cpc = 0.95,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 67000,
  kw.difficulty = 90,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "scan qr code" — 66K SV, KD72, Transactional
MERGE (kw:SEOKeyword {key: 'seo:scan-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'scan qr code',
  kw.content = 'SEO keyword for en-US: scan qr code',
  kw.value = 'scan qr code',
  kw.slug_form = 'scan-qr-code',
  kw.locale_key = 'en-US',
  kw.volume = 66000,
  kw.difficulty = 72,
  kw.cpc = 0.70,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 66000,
  kw.difficulty = 72,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code maker" — 52K SV, KD94, Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-maker@en-US'})
ON CREATE SET
  kw.display_name = 'qr code maker',
  kw.content = 'SEO keyword for en-US: qr code maker',
  kw.value = 'qr code maker',
  kw.slug_form = 'qr-code-maker',
  kw.locale_key = 'en-US',
  kw.volume = 52000,
  kw.difficulty = 94,
  kw.cpc = 1.00,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 52000,
  kw.difficulty = 94,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr scanner" — 50K SV, KD73, Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-scanner@en-US'})
ON CREATE SET
  kw.display_name = 'qr scanner',
  kw.content = 'SEO keyword for en-US: qr scanner',
  kw.value = 'qr scanner',
  kw.slug_form = 'qr-scanner',
  kw.locale_key = 'en-US',
  kw.volume = 50000,
  kw.difficulty = 73,
  kw.cpc = 0.70,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 50000,
  kw.difficulty = 73,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "create qr code" — 49K SV, KD89, Transactional
MERGE (kw:SEOKeyword {key: 'seo:create-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'create qr code',
  kw.content = 'SEO keyword for en-US: create qr code',
  kw.value = 'create qr code',
  kw.slug_form = 'create-qr-code',
  kw.locale_key = 'en-US',
  kw.volume = 49000,
  kw.difficulty = 89,
  kw.cpc = 0.90,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 49000,
  kw.difficulty = 89,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "how to scan a qr code" — 38K SV, KD61, Informational
MERGE (kw:SEOKeyword {key: 'seo:how-to-scan-a-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'how to scan a qr code',
  kw.content = 'SEO keyword for en-US: how to scan a qr code',
  kw.value = 'how to scan a qr code',
  kw.slug_form = 'how-to-scan-a-qr-code',
  kw.locale_key = 'en-US',
  kw.volume = 38000,
  kw.difficulty = 61,
  kw.cpc = 0.50,
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 38000,
  kw.difficulty = 61,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "create a qr code" — 37K SV, KD89, Transactional
MERGE (kw:SEOKeyword {key: 'seo:create-a-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'create a qr code',
  kw.content = 'SEO keyword for en-US: create a qr code',
  kw.value = 'create a qr code',
  kw.slug_form = 'create-a-qr-code',
  kw.locale_key = 'en-US',
  kw.volume = 37000,
  kw.difficulty = 89,
  kw.cpc = 0.90,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 37000,
  kw.difficulty = 89,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "how to make a qr code" — 37K SV, KD84, Informational
MERGE (kw:SEOKeyword {key: 'seo:how-to-make-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'how to make a qr code',
  kw.content = 'SEO keyword for en-US: how to make a qr code',
  kw.value = 'how to make a qr code',
  kw.slug_form = 'how-to-make-qr-code',
  kw.locale_key = 'en-US',
  kw.volume = 37000,
  kw.difficulty = 84,
  kw.cpc = 0.70,
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 37000,
  kw.difficulty = 84,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "how to create a qr code" — 33K SV, KD90, Informational
MERGE (kw:SEOKeyword {key: 'seo:how-to-create-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'how to create a qr code',
  kw.content = 'SEO keyword for en-US: how to create a qr code',
  kw.value = 'how to create a qr code',
  kw.slug_form = 'how-to-create-qr-code',
  kw.locale_key = 'en-US',
  kw.volume = 33000,
  kw.difficulty = 90,
  kw.cpc = 0.80,
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 33000,
  kw.difficulty = 90,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr generator" — 31K SV, KD91, Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-generator@en-US'})
ON CREATE SET
  kw.display_name = 'qr generator',
  kw.content = 'SEO keyword for en-US: qr generator',
  kw.value = 'qr generator',
  kw.slug_form = 'qr-generator',
  kw.locale_key = 'en-US',
  kw.volume = 31000,
  kw.difficulty = 91,
  kw.cpc = 0.95,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 31000,
  kw.difficulty = 91,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "make a qr code" — 26K SV, KD85, Transactional
MERGE (kw:SEOKeyword {key: 'seo:make-a-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'make a qr code',
  kw.content = 'SEO keyword for en-US: make a qr code',
  kw.value = 'make a qr code',
  kw.slug_form = 'make-a-qr-code',
  kw.locale_key = 'en-US',
  kw.volume = 26000,
  kw.difficulty = 85,
  kw.cpc = 0.85,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 26000,
  kw.difficulty = 85,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code reader" — 20K SV, KD75, Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-reader@en-US'})
ON CREATE SET
  kw.display_name = 'qr code reader',
  kw.content = 'SEO keyword for en-US: qr code reader',
  kw.value = 'qr code reader',
  kw.slug_form = 'qr-code-reader',
  kw.locale_key = 'en-US',
  kw.volume = 20000,
  kw.difficulty = 75,
  kw.cpc = 0.70,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 20000,
  kw.difficulty = 75,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "free qr code" — 19K SV, KD93, Transactional
MERGE (kw:SEOKeyword {key: 'seo:free-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'free qr code',
  kw.content = 'SEO keyword for en-US: free qr code',
  kw.value = 'free qr code',
  kw.slug_form = 'free-qr-code',
  kw.locale_key = 'en-US',
  kw.volume = 19000,
  kw.difficulty = 93,
  kw.cpc = 0.90,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 19000,
  kw.difficulty = 93,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code creator" — 19K SV, KD92, Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-creator@en-US'})
ON CREATE SET
  kw.display_name = 'qr code creator',
  kw.content = 'SEO keyword for en-US: qr code creator',
  kw.value = 'qr code creator',
  kw.slug_form = 'qr-code-creator',
  kw.locale_key = 'en-US',
  kw.volume = 19000,
  kw.difficulty = 92,
  kw.cpc = 0.95,
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.volume = 19000,
  kw.difficulty = 92,
  kw.trait = 'imported',
  kw.updated_at = datetime();

// =============================================================================
// SECONDARY KEYWORDS (QR Code domain coverage)
// v0.17.3: All keywords updated with trait = 'imported', source = 'ahrefs'
// =============================================================================

// "create qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:create-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'create qr code',
  kw.content = 'SEO keyword for en-US: create qr code',
  kw.value = 'create qr code',
  kw.slug_form = 'create-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "custom qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:custom-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'custom qr code',
  kw.content = 'SEO keyword for en-US: custom qr code',
  kw.value = 'custom qr code',
  kw.slug_form = 'custom-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "download qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:download-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'download qr code',
  kw.content = 'SEO keyword for en-US: download qr code',
  kw.value = 'download qr code',
  kw.slug_form = 'download-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "dynamic qr code generator" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:dynamic-qr-code-generator@en-US'})
ON CREATE SET
  kw.display_name = 'dynamic qr code generator',
  kw.content = 'SEO keyword for en-US: dynamic qr code generator',
  kw.value = 'dynamic qr code generator',
  kw.slug_form = 'dynamic-qr-code-generator',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "dynamic qr code" — Informational
MERGE (kw:SEOKeyword {key: 'seo:dynamic-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'dynamic qr code',
  kw.content = 'SEO keyword for en-US: dynamic qr code',
  kw.value = 'dynamic qr code',
  kw.slug_form = 'dynamic-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "free qr code generator" — Transactional (High Volume)
MERGE (kw:SEOKeyword {key: 'seo:free-qr-code-generator@en-US'})
ON CREATE SET
  kw.display_name = 'free qr code generator',
  kw.content = 'SEO keyword for en-US: free qr code generator',
  kw.value = 'free qr code generator',
  kw.slug_form = 'free-qr-code-generator',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "free qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:free-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'free qr code',
  kw.content = 'SEO keyword for en-US: free qr code',
  kw.value = 'free qr code',
  kw.slug_form = 'free-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "generate qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:generate-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'generate qr code',
  kw.content = 'SEO keyword for en-US: generate qr code',
  kw.value = 'generate qr code',
  kw.slug_form = 'generate-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "how to create qr code" — Informational
MERGE (kw:SEOKeyword {key: 'seo:how-to-create-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'how to create qr code',
  kw.content = 'SEO keyword for en-US: how to create qr code',
  kw.value = 'how to create qr code',
  kw.slug_form = 'how-to-create-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "how to make qr code" — Informational
MERGE (kw:SEOKeyword {key: 'seo:how-to-make-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'how to make qr code',
  kw.content = 'SEO keyword for en-US: how to make qr code',
  kw.value = 'how to make qr code',
  kw.slug_form = 'how-to-make-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "instagram qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:instagram-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'instagram qr code',
  kw.content = 'SEO keyword for en-US: instagram qr code',
  kw.value = 'instagram qr code',
  kw.slug_form = 'instagram-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "make qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:make-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'make qr code',
  kw.content = 'SEO keyword for en-US: make qr code',
  kw.value = 'make qr code',
  kw.slug_form = 'make-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "menu qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:menu-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'menu qr code',
  kw.content = 'SEO keyword for en-US: menu qr code',
  kw.value = 'menu qr code',
  kw.slug_form = 'menu-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "print qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:print-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'print qr code',
  kw.content = 'SEO keyword for en-US: print qr code',
  kw.value = 'print qr code',
  kw.slug_form = 'print-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code business card" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-business-card@en-US'})
ON CREATE SET
  kw.display_name = 'qr code business card',
  kw.content = 'SEO keyword for en-US: qr code business card',
  kw.value = 'qr code business card',
  kw.slug_form = 'qr-code-business-card',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code coupon" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-coupon@en-US'})
ON CREATE SET
  kw.display_name = 'qr code coupon',
  kw.content = 'SEO keyword for en-US: qr code coupon',
  kw.value = 'qr code coupon',
  kw.slug_form = 'qr-code-coupon',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code design" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-design@en-US'})
ON CREATE SET
  kw.display_name = 'qr code design',
  kw.content = 'SEO keyword for en-US: qr code design',
  kw.value = 'qr code design',
  kw.slug_form = 'qr-code-design',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code email" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-email@en-US'})
ON CREATE SET
  kw.display_name = 'qr code email',
  kw.content = 'SEO keyword for en-US: qr code email',
  kw.value = 'qr code email',
  kw.slug_form = 'qr-code-email',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code facebook" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-facebook@en-US'})
ON CREATE SET
  kw.display_name = 'qr code facebook',
  kw.content = 'SEO keyword for en-US: qr code facebook',
  kw.value = 'qr code facebook',
  kw.slug_form = 'qr-code-facebook',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code google maps" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-google-maps@en-US'})
ON CREATE SET
  kw.display_name = 'qr code google maps',
  kw.content = 'SEO keyword for en-US: qr code google maps',
  kw.value = 'qr code google maps',
  kw.slug_form = 'qr-code-google-maps',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code google review" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-google-review@en-US'})
ON CREATE SET
  kw.display_name = 'qr code google review',
  kw.content = 'SEO keyword for en-US: qr code google review',
  kw.value = 'qr code google review',
  kw.slug_form = 'qr-code-google-review',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code image" — Informational
MERGE (kw:SEOKeyword {key: 'seo:qr-code-image@en-US'})
ON CREATE SET
  kw.display_name = 'qr code image',
  kw.content = 'SEO keyword for en-US: qr code image',
  kw.value = 'qr code image',
  kw.slug_form = 'qr-code-image',
  kw.locale_key = 'en-US',
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code instagram" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-instagram@en-US'})
ON CREATE SET
  kw.display_name = 'qr code instagram',
  kw.content = 'SEO keyword for en-US: qr code instagram',
  kw.value = 'qr code instagram',
  kw.slug_form = 'qr-code-instagram',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code link" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-link@en-US'})
ON CREATE SET
  kw.display_name = 'qr code link',
  kw.content = 'SEO keyword for en-US: qr code link',
  kw.value = 'qr code link',
  kw.slug_form = 'qr-code-link',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code linkedin" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-linkedin@en-US'})
ON CREATE SET
  kw.display_name = 'qr code linkedin',
  kw.content = 'SEO keyword for en-US: qr code linkedin',
  kw.value = 'qr code linkedin',
  kw.slug_form = 'qr-code-linkedin',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code location" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-location@en-US'})
ON CREATE SET
  kw.display_name = 'qr code location',
  kw.content = 'SEO keyword for en-US: qr code location',
  kw.value = 'qr code location',
  kw.slug_form = 'qr-code-location',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code maker" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-maker@en-US'})
ON CREATE SET
  kw.display_name = 'qr code maker',
  kw.content = 'SEO keyword for en-US: qr code maker',
  kw.value = 'qr code maker',
  kw.slug_form = 'qr-code-maker',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code menu" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-menu@en-US'})
ON CREATE SET
  kw.display_name = 'qr code menu',
  kw.content = 'SEO keyword for en-US: qr code menu',
  kw.value = 'qr code menu',
  kw.slug_form = 'qr-code-menu',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code payment" — Informational
MERGE (kw:SEOKeyword {key: 'seo:qr-code-payment@en-US'})
ON CREATE SET
  kw.display_name = 'qr code payment',
  kw.content = 'SEO keyword for en-US: qr code payment',
  kw.value = 'qr code payment',
  kw.slug_form = 'qr-code-payment',
  kw.locale_key = 'en-US',
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code paypal" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-paypal@en-US'})
ON CREATE SET
  kw.display_name = 'qr code paypal',
  kw.content = 'SEO keyword for en-US: qr code paypal',
  kw.value = 'qr code paypal',
  kw.slug_form = 'qr-code-paypal',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code pdf" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-pdf@en-US'})
ON CREATE SET
  kw.display_name = 'qr code pdf',
  kw.content = 'SEO keyword for en-US: qr code pdf',
  kw.value = 'qr code pdf',
  kw.slug_form = 'qr-code-pdf',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code phone" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-phone@en-US'})
ON CREATE SET
  kw.display_name = 'qr code phone',
  kw.content = 'SEO keyword for en-US: qr code phone',
  kw.value = 'qr code phone',
  kw.slug_form = 'qr-code-phone',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code reader" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-reader@en-US'})
ON CREATE SET
  kw.display_name = 'qr code reader',
  kw.content = 'SEO keyword for en-US: qr code reader',
  kw.value = 'qr code reader',
  kw.slug_form = 'qr-code-reader',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code scanner" — Transactional (High Volume)
MERGE (kw:SEOKeyword {key: 'seo:qr-code-scanner@en-US'})
ON CREATE SET
  kw.display_name = 'qr code scanner',
  kw.content = 'SEO keyword for en-US: qr code scanner',
  kw.value = 'qr code scanner',
  kw.slug_form = 'qr-code-scanner',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code sms" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-sms@en-US'})
ON CREATE SET
  kw.display_name = 'qr code sms',
  kw.content = 'SEO keyword for en-US: qr code sms',
  kw.value = 'qr code sms',
  kw.slug_form = 'qr-code-sms',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code spotify" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-spotify@en-US'})
ON CREATE SET
  kw.display_name = 'qr code spotify',
  kw.content = 'SEO keyword for en-US: qr code spotify',
  kw.value = 'qr code spotify',
  kw.slug_form = 'qr-code-spotify',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code ticket" — Informational
MERGE (kw:SEOKeyword {key: 'seo:qr-code-ticket@en-US'})
ON CREATE SET
  kw.display_name = 'qr code ticket',
  kw.content = 'SEO keyword for en-US: qr code ticket',
  kw.value = 'qr code ticket',
  kw.slug_form = 'qr-code-ticket',
  kw.locale_key = 'en-US',
  kw.intent = 'informational',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code tiktok" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-tiktok@en-US'})
ON CREATE SET
  kw.display_name = 'qr code tiktok',
  kw.content = 'SEO keyword for en-US: qr code tiktok',
  kw.value = 'qr code tiktok',
  kw.slug_form = 'qr-code-tiktok',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code twitter" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-twitter@en-US'})
ON CREATE SET
  kw.display_name = 'qr code twitter',
  kw.content = 'SEO keyword for en-US: qr code twitter',
  kw.value = 'qr code twitter',
  kw.slug_form = 'qr-code-twitter',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code url" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-url@en-US'})
ON CREATE SET
  kw.display_name = 'qr code url',
  kw.content = 'SEO keyword for en-US: qr code url',
  kw.value = 'qr code url',
  kw.slug_form = 'qr-code-url',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code venmo" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-venmo@en-US'})
ON CREATE SET
  kw.display_name = 'qr code venmo',
  kw.content = 'SEO keyword for en-US: qr code venmo',
  kw.value = 'qr code venmo',
  kw.slug_form = 'qr-code-venmo',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code video" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-video@en-US'})
ON CREATE SET
  kw.display_name = 'qr code video',
  kw.content = 'SEO keyword for en-US: qr code video',
  kw.value = 'qr code video',
  kw.slug_form = 'qr-code-video',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code wedding" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-wedding@en-US'})
ON CREATE SET
  kw.display_name = 'qr code wedding',
  kw.content = 'SEO keyword for en-US: qr code wedding',
  kw.value = 'qr code wedding',
  kw.slug_form = 'qr-code-wedding',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code whatsapp" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-whatsapp@en-US'})
ON CREATE SET
  kw.display_name = 'qr code whatsapp',
  kw.content = 'SEO keyword for en-US: qr code whatsapp',
  kw.value = 'qr code whatsapp',
  kw.slug_form = 'qr-code-whatsapp',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code wifi generator" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-wifi-generator@en-US'})
ON CREATE SET
  kw.display_name = 'qr code wifi generator',
  kw.content = 'SEO keyword for en-US: qr code wifi generator',
  kw.value = 'qr code wifi generator',
  kw.slug_form = 'qr-code-wifi-generator',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code with logo" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-with-logo@en-US'})
ON CREATE SET
  kw.display_name = 'qr code with logo',
  kw.content = 'SEO keyword for en-US: qr code with logo',
  kw.value = 'qr code with logo',
  kw.slug_form = 'qr-code-with-logo',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "qr code youtube" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:qr-code-youtube@en-US'})
ON CREATE SET
  kw.display_name = 'qr code youtube',
  kw.content = 'SEO keyword for en-US: qr code youtube',
  kw.value = 'qr code youtube',
  kw.slug_form = 'qr-code-youtube',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "scan qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:scan-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'scan qr code',
  kw.content = 'SEO keyword for en-US: scan qr code',
  kw.value = 'scan qr code',
  kw.slug_form = 'scan-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "vcard qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:vcard-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'vcard qr code',
  kw.content = 'SEO keyword for en-US: vcard qr code',
  kw.value = 'vcard qr code',
  kw.slug_form = 'vcard-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// "wifi qr code" — Transactional
MERGE (kw:SEOKeyword {key: 'seo:wifi-qr-code@en-US'})
ON CREATE SET
  kw.display_name = 'wifi qr code',
  kw.content = 'SEO keyword for en-US: wifi qr code',
  kw.value = 'wifi qr code',
  kw.slug_form = 'wifi-qr-code',
  kw.locale_key = 'en-US',
  kw.intent = 'transactional',
  kw.trait = 'imported',
  kw.source = 'ahrefs',
  kw.source_date = date('2026-03-08'),
  kw.llm_context = 'USE: when targeting en-US search traffic. TRIGGERS: SEO, keywords, ranking. RELATES: EntityNative (via TARGETS), Locale (via FOR_LOCALE).',
  kw.node_class = 'SEOKeyword',
  kw.created_by = 'seed:content',
  kw.created_by_file = '51-seokeywords-en-us.cypher',
  kw.created_at = datetime(),
  kw.updated_at = datetime()
ON MATCH SET
  kw.trait = 'imported',
  kw.updated_at = datetime();

// =============================================================================
// Link all SEOKeywords to en-US Locale
// =============================================================================

MATCH (kw:SEOKeyword)
WHERE kw.locale_key = 'en-US'
MATCH (loc:Locale {key: 'en-US'})
MERGE (kw)-[:FOR_LOCALE]->(loc);

// =============================================================================
// Summary
// =============================================================================
RETURN 'SEOKeyword seed complete: 52 en-US keywords' AS status;
