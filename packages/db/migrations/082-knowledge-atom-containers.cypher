// Migration 082: Create Knowledge Atom Container Infrastructure
// Creates AudienceSet, CultureSet, TabooSet, PatternSet for all 203 locales
// These are SHARED realm, knowledge layer containers (imported trait)
//
// Run after: 081-remove-orphan-legacy-expressions.cypher
// Prerequisite: All 203 Locale nodes must exist

// =============================================================================
// PART 1: Create AudienceSet (general segment) for each Locale
// =============================================================================
MATCH (l:Locale)
WHERE NOT EXISTS {
  MATCH (l)-[:HAS_AUDIENCE]->(as:AudienceSet)
  WHERE as.segment = 'general'
}
WITH l
CREATE (as:AudienceSet {
  key: 'audience-set:general@' + l.key,
  display_name: l.display_name + ' General Audience',
  description: 'General audience characteristics for ' + l.key,
  segment: 'general',
  llm_context: 'USE: when generating content for general audience in this locale. TRIGGERS: audience, demographics, behavior, preferences. NOT: for specific B2B/B2C segments (create separate AudienceSet). RELATES: Locale (parent), AudienceTrait (contains).',
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (l)-[:HAS_AUDIENCE]->(as)
CREATE (as)-[:FOR_LOCALE]->(l)
RETURN count(as) AS audience_sets_created;

// =============================================================================
// PART 2: Create CultureSet (values type) for each Locale
// =============================================================================
MATCH (l:Locale)
WHERE NOT EXISTS {
  MATCH (l)-[:HAS_CULTURE_SET]->(cs:CultureSet)
  WHERE cs.culture_type = 'values'
}
WITH l
CREATE (cs:CultureSet {
  key: 'culture-set:values@' + l.key,
  display_name: l.display_name + ' Cultural Values',
  description: 'Core cultural values and beliefs for ' + l.key,
  culture_type: 'values',
  llm_context: 'USE: when incorporating cultural values in content. TRIGGERS: culture, values, beliefs, customs, traditions. NOT: for celebrity references (use references type). RELATES: Locale (parent), CultureRef (contains).',
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (l)-[:HAS_CULTURE_SET]->(cs)
CREATE (cs)-[:FOR_LOCALE]->(l)
RETURN count(cs) AS culture_sets_created;

// =============================================================================
// PART 3: Create TabooSet (avoid severity) for each Locale
// =============================================================================
MATCH (l:Locale)
WHERE NOT EXISTS {
  MATCH (l)-[:HAS_TABOOS]->(ts:TabooSet)
  WHERE ts.severity = 'avoid'
}
WITH l
CREATE (ts:TabooSet {
  key: 'taboo-set:avoid@' + l.key,
  display_name: l.display_name + ' Topics to Avoid',
  description: 'Topics, words, and references to never mention for ' + l.key,
  severity: 'avoid',
  llm_context: 'USE: when filtering content for cultural appropriateness. TRIGGERS: taboo, avoid, forbidden, sensitive, inappropriate. NOT: for legal restrictions (use legal type). RELATES: Locale (parent), Taboo (contains).',
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (l)-[:HAS_TABOOS]->(ts)
CREATE (ts)-[:FOR_LOCALE]->(l)
RETURN count(ts) AS taboo_sets_created;

// =============================================================================
// PART 4: Create PatternSet (cta usage) for each Locale
// =============================================================================
MATCH (l:Locale)
WHERE NOT EXISTS {
  MATCH (l)-[:HAS_PATTERNS]->(ps:PatternSet)
  WHERE ps.usage = 'cta'
}
WITH l
CREATE (ps:PatternSet {
  key: 'pattern-set:cta@' + l.key,
  display_name: l.display_name + ' CTA Patterns',
  description: 'Call-to-action text patterns effective for ' + l.key,
  usage: 'cta',
  llm_context: 'USE: when generating CTAs, buttons, action prompts. TRIGGERS: cta, call-to-action, button, action, convert. NOT: for headlines (use headlines type). RELATES: Locale (parent), Pattern (contains).',
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (l)-[:HAS_PATTERNS]->(ps)
CREATE (ps)-[:FOR_LOCALE]->(l)
RETURN count(ps) AS pattern_sets_created;

// =============================================================================
// VERIFICATION: Count all containers
// =============================================================================
MATCH (as:AudienceSet) WITH count(as) AS audience_count
MATCH (cs:CultureSet) WITH audience_count, count(cs) AS culture_count
MATCH (ts:TabooSet) WITH audience_count, culture_count, count(ts) AS taboo_count
MATCH (ps:PatternSet) WITH audience_count, culture_count, taboo_count, count(ps) AS pattern_count
RETURN
  audience_count AS AudienceSet,
  culture_count AS CultureSet,
  taboo_count AS TabooSet,
  pattern_count AS PatternSet,
  audience_count + culture_count + taboo_count + pattern_count AS total_containers;
