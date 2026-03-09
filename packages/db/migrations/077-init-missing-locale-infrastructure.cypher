// ============================================================================
// MIGRATION 077: Initialize Missing Locale Infrastructure
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Create complete locale infrastructure for fil-PH and lo-LA
//
// ROOT CAUSE: These locales were never added to the base seed files.
// This migration creates EVERYTHING needed for these locales to work.
//
// Missing (all must be created):
//   - Locale nodes themselves (fil-PH, lo-LA)
//   - Slugification nodes
//   - Formatting nodes
//   - Culture nodes
//   - ExpressionSet nodes
//   - All HAS_* and FOR_LOCALE arcs
//
// Note: fil-PH uses Latin script, lo-LA uses Lao script
// ============================================================================

// ============================================================================
// STEP 0: Create the Locale nodes (ROOT CAUSE - these were missing!)
// ============================================================================

// fil-PH: Filipino (Philippines) - Latin script
MERGE (l:Locale {key: 'fil-PH'})
ON CREATE SET
  l.display_name = 'Filipino (Philippines)',
  l.description = 'Filipino locale for Philippines market',
  l.llm_context = 'USE: for Filipino content targeting Philippines. TRIGGERS: fil-PH, Filipino, Pilipinas. NOT: for Tagalog (use tl-PH).',
  l.language_code = 'fil',
  l.country_code = 'PH',
  l.name_native = 'Filipino (Pilipinas)',
  l.is_primary = false,
  l.region = 'asia',
  l.language_family = 'austronesian',
  l.script = 'latin',
  l.text_direction = 'ltr',
  l.created_by = 'migration:077',
  l.created_at = datetime(),
  l.updated_at = datetime()
ON MATCH SET
  l.display_name = 'Filipino (Philippines)',
  l.region = 'asia',
  l.language_family = 'austronesian',
  l.script = 'latin',
  l.text_direction = 'ltr',
  l.updated_at = datetime()
RETURN 'fil-PH Locale created' AS status;

// lo-LA: Lao (Laos) - Lao script
MERGE (l:Locale {key: 'lo-LA'})
ON CREATE SET
  l.display_name = 'Lao (Laos)',
  l.description = 'Lao locale for Laos market',
  l.llm_context = 'USE: for Lao content targeting Laos. TRIGGERS: lo-LA, Lao, ລາວ. NOT: for Thai.',
  l.language_code = 'lo',
  l.country_code = 'LA',
  l.name_native = 'ລາວ (ລາວ)',
  l.is_primary = true,
  l.region = 'asia',
  l.language_family = 'tai-kadai',
  l.script = 'lao',
  l.text_direction = 'ltr',
  l.created_by = 'migration:077',
  l.created_at = datetime(),
  l.updated_at = datetime()
ON MATCH SET
  l.display_name = 'Lao (Laos)',
  l.region = 'asia',
  l.language_family = 'tai-kadai',
  l.script = 'lao',
  l.text_direction = 'ltr',
  l.updated_at = datetime()
RETURN 'lo-LA Locale created' AS status;

// ============================================================================
// STEP 1: Create Slugification nodes
// ============================================================================

// fil-PH: Latin script, standard slugification
MERGE (s:Slugification {key: 'fil-PH'})
SET s.display_name = 'Filipino (Philippines) Slugification',
    s.description = 'URL slug generation rules for fil-PH',
    s.llm_context = 'USE: for Filipino content URL slugs. TRIGGERS: fil-PH, tagalog, filipino. NOT: for other Philippine languages.',
    s.slug_rule = 'latin_strip',
    s.preserve_diacritics = false,
    s.unicode_normalization = 'NFC',
    s.min_length = 3,
    s.max_length = 80,
    s.separator = '-',
    s.lowercase = true,
    s.preserve_numbers = true,
    s.stop_words = ['ang', 'ng', 'sa', 'na', 'at', 'ay', 'mga'],
    s.created_at = datetime(),
    s.updated_at = datetime()
RETURN 'fil-PH Slugification created' AS status;

// lo-LA: Lao script (non-Latin)
MERGE (s:Slugification {key: 'lo-LA'})
SET s.display_name = 'Lao (Laos) Slugification',
    s.description = 'URL slug generation rules for lo-LA',
    s.llm_context = 'USE: for Lao content URL slugs. TRIGGERS: lo-LA, lao, laos. NOT: for Thai.',
    s.slug_rule = 'native_script',
    s.preserve_diacritics = true,
    s.unicode_normalization = 'NFC',
    s.min_length = 3,
    s.max_length = 80,
    s.separator = '-',
    s.lowercase = false,
    s.preserve_numbers = true,
    s.stop_words = [],
    s.created_at = datetime(),
    s.updated_at = datetime()
RETURN 'lo-LA Slugification created' AS status;

// ============================================================================
// STEP 2: Create Formatting nodes
// ============================================================================

MERGE (f:Formatting {key: 'fil-PH'})
SET f.display_name = 'fil-PH Formatting',
    f.description = 'Formatting rules for fil-PH',
    f.llm_context = 'USE: for Filipino number/date/time formatting.',
    f.created_at = datetime(),
    f.updated_at = datetime()
RETURN 'fil-PH Formatting created' AS status;

MERGE (f:Formatting {key: 'lo-LA'})
SET f.display_name = 'lo-LA Formatting',
    f.description = 'Formatting rules for lo-LA',
    f.llm_context = 'USE: for Lao number/date/time formatting.',
    f.created_at = datetime(),
    f.updated_at = datetime()
RETURN 'lo-LA Formatting created' AS status;

// ============================================================================
// STEP 3: Create Culture nodes
// ============================================================================

MERGE (c:Culture {key: 'fil-PH'})
SET c.display_name = 'fil-PH Culture Norms',
    c.description = 'Cultural context and norms for fil-PH',
    c.llm_context = 'USE: for Filipino cultural context. TRIGGERS: filipino culture, philippines customs.',
    c.created_at = datetime(),
    c.updated_at = datetime()
RETURN 'fil-PH Culture created' AS status;

MERGE (c:Culture {key: 'lo-LA'})
SET c.display_name = 'lo-LA Culture Norms',
    c.description = 'Cultural context and norms for lo-LA',
    c.llm_context = 'USE: for Lao cultural context. TRIGGERS: lao culture, laos customs.',
    c.created_at = datetime(),
    c.updated_at = datetime()
RETURN 'lo-LA Culture created' AS status;

// ============================================================================
// STEP 4: Create ExpressionSet nodes (empty containers)
// ============================================================================

MERGE (e:ExpressionSet {key: 'expression-set@fil-PH'})
SET e.display_name = 'Filipino Expression Set',
    e.description = 'Container for fil-PH expressions',
    e.llm_context = 'Container for Filipino idiomatic expressions.',
    e.total_expressions = 0,
    e.created_at = datetime(),
    e.updated_at = datetime()
RETURN 'fil-PH ExpressionSet created' AS status;

MERGE (e:ExpressionSet {key: 'expression-set@lo-LA'})
SET e.display_name = 'Lao Expression Set',
    e.description = 'Container for lo-LA expressions',
    e.llm_context = 'Container for Lao idiomatic expressions.',
    e.total_expressions = 0,
    e.created_at = datetime(),
    e.updated_at = datetime()
RETURN 'lo-LA ExpressionSet created' AS status;

// ============================================================================
// STEP 5: Create HAS_SLUGIFICATION arcs
// ============================================================================

MATCH (l:Locale {key: 'fil-PH'}), (s:Slugification {key: 'fil-PH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s)
RETURN 'fil-PH HAS_SLUGIFICATION created' AS status;

MATCH (l:Locale {key: 'lo-LA'}), (s:Slugification {key: 'lo-LA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s)
RETURN 'lo-LA HAS_SLUGIFICATION created' AS status;

// ============================================================================
// STEP 6: Create HAS_FORMATTING arcs
// ============================================================================

MATCH (l:Locale {key: 'fil-PH'}), (f:Formatting {key: 'fil-PH'})
MERGE (l)-[:HAS_FORMATTING]->(f)
RETURN 'fil-PH HAS_FORMATTING created' AS status;

MATCH (l:Locale {key: 'lo-LA'}), (f:Formatting {key: 'lo-LA'})
MERGE (l)-[:HAS_FORMATTING]->(f)
RETURN 'lo-LA HAS_FORMATTING created' AS status;

// ============================================================================
// STEP 7: Create HAS_CULTURE arcs
// ============================================================================

MATCH (l:Locale {key: 'fil-PH'}), (c:Culture {key: 'fil-PH'})
MERGE (l)-[:HAS_CULTURE]->(c)
RETURN 'fil-PH HAS_CULTURE created' AS status;

MATCH (l:Locale {key: 'lo-LA'}), (c:Culture {key: 'lo-LA'})
MERGE (l)-[:HAS_CULTURE]->(c)
RETURN 'lo-LA HAS_CULTURE created' AS status;

// ============================================================================
// STEP 8: Create HAS_EXPRESSIONS arcs
// ============================================================================

MATCH (l:Locale {key: 'fil-PH'}), (e:ExpressionSet {key: 'expression-set@fil-PH'})
MERGE (l)-[:HAS_EXPRESSIONS]->(e)
RETURN 'fil-PH HAS_EXPRESSIONS created' AS status;

MATCH (l:Locale {key: 'lo-LA'}), (e:ExpressionSet {key: 'expression-set@lo-LA'})
MERGE (l)-[:HAS_EXPRESSIONS]->(e)
RETURN 'lo-LA HAS_EXPRESSIONS created' AS status;

// ============================================================================
// STEP 9: Create FOR_LOCALE arcs (reverse direction)
// ============================================================================

MATCH (e:ExpressionSet {key: 'expression-set@fil-PH'}), (l:Locale {key: 'fil-PH'})
MERGE (e)-[:FOR_LOCALE]->(l)
RETURN 'fil-PH FOR_LOCALE created' AS status;

MATCH (e:ExpressionSet {key: 'expression-set@lo-LA'}), (l:Locale {key: 'lo-LA'})
MERGE (e)-[:FOR_LOCALE]->(l)
RETURN 'lo-LA FOR_LOCALE created' AS status;

// ============================================================================
// STEP 10: Create FALLBACK_TO arcs
// ============================================================================

// fil-PH falls back to en-US (English is widely spoken in Philippines)
MATCH (from:Locale {key: 'fil-PH'}), (to:Locale {key: 'en-US'})
MERGE (from)-[:FALLBACK_TO]->(to)
RETURN 'fil-PH FALLBACK_TO en-US created' AS status;

// lo-LA falls back to en-US (no regional Lao variant)
MATCH (from:Locale {key: 'lo-LA'}), (to:Locale {key: 'en-US'})
MERGE (from)-[:FALLBACK_TO]->(to)
RETURN 'lo-LA FALLBACK_TO en-US created' AS status;

// ============================================================================
// VERIFICATION
// ============================================================================

MATCH (l:Locale)
WHERE l.key IN ['fil-PH', 'lo-LA']
OPTIONAL MATCH (l)-[:HAS_SLUGIFICATION]->(s:Slugification)
OPTIONAL MATCH (l)-[:HAS_FORMATTING]->(f:Formatting)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(c:Culture)
OPTIONAL MATCH (l)-[:HAS_EXPRESSIONS]->(e:ExpressionSet)
OPTIONAL MATCH (l)-[:FALLBACK_TO]->(fb:Locale)
RETURN l.key AS locale,
       l.script AS script,
       l.region AS region,
       s IS NOT NULL AS has_slugification,
       f IS NOT NULL AS has_formatting,
       c IS NOT NULL AS has_culture,
       e IS NOT NULL AS has_expressions,
       fb.key AS fallback_locale;
