// ============================================================================
// Migration 123: Add content to Knowledge Atoms (Template-based)
// ============================================================================
// This creates initial template-based content for all knowledge atoms.
// Cultural nuances will be enriched via Perplexity in a separate workflow.
//
// Templates follow the locale-specific principle: content describes usage
// specifically in that locale, not generic "French" or "Spanish".
// ============================================================================

// --- Expression: Template content ---
// Template: "Expression '{text}' used in {locale} ({domain}, {register} register)."
MATCH (e:Expression)-[:FOR_LOCALE]->(l:Locale)
WHERE e.content IS NULL
WITH e, replace(l.key, 'locale:', '') AS locale
SET e.content = 'Expression used in ' + locale +
  CASE WHEN e.domain IS NOT NULL THEN ' for ' + e.domain + ' contexts' ELSE '' END +
  CASE WHEN e.register IS NOT NULL THEN ' (' + e.register + ' register)' ELSE '' END +
  ': "' + coalesce(e.text, e.display_name) + '"';

// Fallback for orphan Expressions
MATCH (e:Expression)
WHERE e.content IS NULL
SET e.content = 'Expression: "' + coalesce(e.text, e.display_name, 'unknown') + '"';

// --- CultureRef: Template content ---
// Template: "Cultural reference in {locale}: {text}. Significance varies by region."
MATCH (c:CultureRef)-[:FOR_LOCALE]->(l:Locale)
WHERE c.content IS NULL
WITH c, replace(l.key, 'locale:', '') AS locale
SET c.content = 'Cultural reference specific to ' + locale + ': "' +
  coalesce(c.text, c.display_name) + '". Significance and recognition may vary within the locale.';

// Fallback for orphan CultureRef
MATCH (c:CultureRef)
WHERE c.content IS NULL
SET c.content = 'Cultural reference: "' + coalesce(c.text, c.display_name, 'unknown') + '"';

// --- Taboo: Template content ---
// Template: "Topic considered sensitive or taboo in {locale}: {text}."
MATCH (t:Taboo)-[:FOR_LOCALE]->(l:Locale)
WHERE t.content IS NULL
WITH t, replace(l.key, 'locale:', '') AS locale
SET t.content = 'Topic considered sensitive or taboo specifically in ' + locale + ': "' +
  coalesce(t.text, t.display_name) + '". Avoid in content targeting this locale.';

// Fallback for orphan Taboo
MATCH (t:Taboo)
WHERE t.content IS NULL
SET t.content = 'Sensitive topic: "' + coalesce(t.text, t.display_name, 'unknown') + '"';

// --- Pattern: Template content ---
// Template: "Text pattern for {locale}: {description}."
MATCH (p:Pattern)-[:FOR_LOCALE]->(l:Locale)
WHERE p.content IS NULL
WITH p, replace(l.key, 'locale:', '') AS locale
SET p.content = 'Text pattern template for ' + locale +
  CASE WHEN p.domain IS NOT NULL THEN ' (' + p.domain + ')' ELSE '' END +
  ': ' + coalesce(p.display_name, p.key);

// Fallback for orphan Pattern
MATCH (p:Pattern)
WHERE p.content IS NULL
SET p.content = 'Text pattern: ' + coalesce(p.display_name, p.key, 'unknown');

// --- AudienceTrait: Template content ---
// Template: "Audience characteristic in {locale}: {text}."
MATCH (a:AudienceTrait)-[:FOR_LOCALE]->(l:Locale)
WHERE a.content IS NULL
WITH a, replace(l.key, 'locale:', '') AS locale
SET a.content = 'Audience trait characteristic of ' + locale + ': "' +
  coalesce(a.text, a.display_name) + '".';

// Fallback for orphan AudienceTrait
MATCH (a:AudienceTrait)
WHERE a.content IS NULL
SET a.content = 'Audience trait: "' + coalesce(a.text, a.display_name, 'unknown') + '"';

// --- Verification ---
// MATCH (n) WHERE labels(n)[0] IN ['Expression', 'CultureRef', 'Taboo', 'Pattern', 'AudienceTrait']
//   AND n.content IS NULL
// RETURN labels(n)[0], count(n);
// Should return 0
