// ============================================================================
// MIGRATION 080: Initialize or-IN (Odia/India) Locale Infrastructure
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Create complete locale infrastructure for or-IN (Odia)
//
// ROOT CAUSE: or-IN locale was referenced by 80 Expression nodes but the
//             Locale node and its infrastructure were never created.
//
// Missing (all must be created):
//   - Locale node (or-IN)
//   - Slugification node
//   - Formatting node
//   - Culture node
//   - ExpressionSet node
//   - All HAS_* and FOR_LOCALE arcs
//
// Note: Odia uses Odia script (derived from Brahmi), LTR direction
// ============================================================================

// ============================================================================
// STEP 0: Create the Locale node
// ============================================================================

MERGE (l:Locale {key: 'or-IN'})
ON CREATE SET
  l.display_name = 'Odia (India)',
  l.description = 'Odia locale for India market',
  l.llm_context = 'USE: for Odia content targeting India. TRIGGERS: or-IN, Odia, ଓଡ଼ିଆ. NOT: for Hindi or Bengali.',
  l.language_code = 'or',
  l.country_code = 'IN',
  l.name_native = 'ଓଡ଼ିଆ (ଭାରତ)',
  l.is_primary = true,
  l.region = 'asia',
  l.language_family = 'indo-aryan',
  l.script = 'odia',
  l.text_direction = 'ltr',
  l.created_by = 'migration:080',
  l.created_at = datetime(),
  l.updated_at = datetime()
ON MATCH SET
  l.display_name = 'Odia (India)',
  l.region = 'asia',
  l.language_family = 'indo-aryan',
  l.script = 'odia',
  l.text_direction = 'ltr',
  l.updated_at = datetime()
RETURN 'or-IN Locale created' AS status;

// ============================================================================
// STEP 1: Create Slugification node (Odia script - non-Latin)
// ============================================================================

MERGE (s:Slugification {key: 'or-IN'})
SET s.display_name = 'Odia (India) Slugification',
    s.description = 'URL slug generation rules for or-IN',
    s.llm_context = 'USE: for Odia content URL slugs. TRIGGERS: or-IN, odia, odisha. NOT: for Hindi or Bengali.',
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
RETURN 'or-IN Slugification created' AS status;

// ============================================================================
// STEP 2: Create Formatting node
// ============================================================================

MERGE (f:Formatting {key: 'or-IN'})
SET f.display_name = 'or-IN Formatting',
    f.description = 'Formatting rules for or-IN',
    f.llm_context = 'USE: for Odia number/date/time formatting. Indian numbering system (lakhs, crores).',
    f.created_at = datetime(),
    f.updated_at = datetime()
RETURN 'or-IN Formatting created' AS status;

// ============================================================================
// STEP 3: Create Culture node
// ============================================================================

MERGE (c:Culture {key: 'or-IN'})
SET c.display_name = 'or-IN Culture Norms',
    c.description = 'Cultural context and norms for or-IN',
    c.llm_context = 'USE: for Odia cultural context. TRIGGERS: odia culture, odisha customs, jagannath temple.',
    c.created_at = datetime(),
    c.updated_at = datetime()
RETURN 'or-IN Culture created' AS status;

// ============================================================================
// STEP 4: Create ExpressionSet node (container for expressions)
// ============================================================================

MERGE (e:ExpressionSet {key: 'expression-set@or-IN'})
SET e.display_name = 'Odia Expression Set',
    e.description = 'Container for or-IN expressions',
    e.llm_context = 'Container for Odia idiomatic expressions.',
    e.total_expressions = 0,
    e.created_at = datetime(),
    e.updated_at = datetime()
RETURN 'or-IN ExpressionSet created' AS status;

// ============================================================================
// STEP 5: Create HAS_SLUGIFICATION arc
// ============================================================================

MATCH (l:Locale {key: 'or-IN'}), (s:Slugification {key: 'or-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s)
RETURN 'or-IN HAS_SLUGIFICATION created' AS status;

// ============================================================================
// STEP 6: Create HAS_FORMATTING arc
// ============================================================================

MATCH (l:Locale {key: 'or-IN'}), (f:Formatting {key: 'or-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f)
RETURN 'or-IN HAS_FORMATTING created' AS status;

// ============================================================================
// STEP 7: Create HAS_CULTURE arc
// ============================================================================

MATCH (l:Locale {key: 'or-IN'}), (c:Culture {key: 'or-IN'})
MERGE (l)-[:HAS_CULTURE]->(c)
RETURN 'or-IN HAS_CULTURE created' AS status;

// ============================================================================
// STEP 8: Create HAS_EXPRESSIONS arc
// ============================================================================

MATCH (l:Locale {key: 'or-IN'}), (e:ExpressionSet {key: 'expression-set@or-IN'})
MERGE (l)-[:HAS_EXPRESSIONS]->(e)
RETURN 'or-IN HAS_EXPRESSIONS created' AS status;

// ============================================================================
// STEP 9: Create FOR_LOCALE arc (reverse direction)
// ============================================================================

MATCH (e:ExpressionSet {key: 'expression-set@or-IN'}), (l:Locale {key: 'or-IN'})
MERGE (e)-[:FOR_LOCALE]->(l)
RETURN 'or-IN FOR_LOCALE created' AS status;

// ============================================================================
// STEP 10: Create FALLBACK_TO arc (to Hindi as regional fallback)
// ============================================================================

MATCH (from:Locale {key: 'or-IN'}), (to:Locale {key: 'hi-IN'})
MERGE (from)-[:FALLBACK_TO]->(to)
RETURN 'or-IN FALLBACK_TO hi-IN created' AS status;

// ============================================================================
// STEP 11: Fix orphan Expression FOR_LOCALE arcs
// ============================================================================

// Expression keys use format: or-IN/CATEGORY/INDEX (not @or-IN suffix)
MATCH (e:Expression)
WHERE e.key STARTS WITH 'or-IN/'
  AND NOT EXISTS { MATCH (e)-[:FOR_LOCALE]->(:Locale) }
MATCH (l:Locale {key: 'or-IN'})
MERGE (e)-[:FOR_LOCALE]->(l)
RETURN count(*) AS expression_for_locale_arcs_created;

// Link expressions to ExpressionSet container
MATCH (e:Expression)
WHERE e.key STARTS WITH 'or-IN/'
MATCH (es:ExpressionSet {key: 'expression-set@or-IN'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e)
RETURN count(*) AS contains_expression_arcs_created;

// ============================================================================
// STEP 12: Update ExpressionSet count
// ============================================================================

MATCH (es:ExpressionSet {key: 'expression-set@or-IN'})
OPTIONAL MATCH (es)-[:CONTAINS_EXPRESSION]->(e:Expression)
WITH es, count(e) AS actual_count
SET es.total_expressions = actual_count,
    es.updated_at = datetime()
RETURN es.key AS key, es.total_expressions AS total_expressions;

// ============================================================================
// VERIFICATION
// ============================================================================

MATCH (l:Locale {key: 'or-IN'})
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

// Check orphan expressions are fixed
MATCH (e:Expression)
WHERE e.key STARTS WITH 'or-IN/'
OPTIONAL MATCH (e)-[:FOR_LOCALE]->(l:Locale)
RETURN count(e) AS total_orin_expressions,
       count(l) AS with_for_locale_arc,
       count(e) - count(l) AS orphans_remaining;
