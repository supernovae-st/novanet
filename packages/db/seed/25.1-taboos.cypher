// ============================================================================
// TABOOS SEED - Extracted from Culture.taboos_summary
// Generated: 2026-03-10T18:26:19.680Z
// Source: 24-culture.cypher
// ============================================================================

// Note: Each Taboo represents a topic to avoid in content generation
// These are CRITICAL for avoiding cultural/legal issues

// ----------------------------------------------------------------------------
// ceb-PH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ceb-PH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ceb-PH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'disrespecting-religious-symbols-santo-nino@ceb-PH'})
SET t.display_name = 'Disrespecting religious symbols (Santo Nino',
    t.locale = 'ceb-PH',
    t.term = 'Disrespecting religious symbols (Santo Nino',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting religious symbols (Santo Nino in ceb-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ceb-PH'})
MATCH (t:Taboo {key: 'disrespecting-religious-symbols-santo-nino@ceb-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'cross@ceb-PH'})
SET t.display_name = 'cross',
    t.locale = 'ceb-PH',
    t.term = 'cross',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: cross in ceb-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ceb-PH'})
MATCH (t:Taboo {key: 'cross@ceb-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'saints@ceb-PH'})
SET t.display_name = 'saints)',
    t.locale = 'ceb-PH',
    t.term = 'saints)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: saints) in ceb-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ceb-PH'})
MATCH (t:Taboo {key: 'saints@ceb-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'mocking-or-disrespecting-elders@ceb-PH'})
SET t.display_name = 'Mocking or disrespecting elders',
    t.locale = 'ceb-PH',
    t.term = 'Mocking or disrespecting elders',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mocking or disrespecting elders in ceb-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ceb-PH'})
MATCH (t:Taboo {key: 'mocking-or-disrespecting-elders@ceb-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'causing-public-hiya-shameembarrassment@ceb-PH'})
SET t.display_name = 'Causing public hiya (shame/embarrassment)',
    t.locale = 'ceb-PH',
    t.term = 'Causing public hiya (shame/embarrassment)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Causing public hiya (shame/embarrassment) in ceb-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ceb-PH'})
MATCH (t:Taboo {key: 'causing-public-hiya-shameembarrassment@ceb-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// mn-MN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mn-MN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@mn-MN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'disrespecting-chinggis-khaan@mn-MN'})
SET t.display_name = 'Disrespecting Chinggis Khaan',
    t.locale = 'mn-MN',
    t.term = 'Disrespecting Chinggis Khaan',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting Chinggis Khaan in mn-MN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mn-MN'})
MATCH (t:Taboo {key: 'disrespecting-chinggis-khaan@mn-MN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'inner-mongolia-and-territorial-issues@mn-MN'})
SET t.display_name = 'Inner Mongolia and territorial issues',
    t.locale = 'mn-MN',
    t.term = 'Inner Mongolia and territorial issues',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Inner Mongolia and territorial issues in mn-MN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mn-MN'})
MATCH (t:Taboo {key: 'inner-mongolia-and-territorial-issues@mn-MN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-elders-or-family@mn-MN'})
SET t.display_name = 'Criticism of elders or family',
    t.locale = 'mn-MN',
    t.term = 'Criticism of elders or family',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of elders or family in mn-MN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mn-MN'})
MATCH (t:Taboo {key: 'criticism-of-elders-or-family@mn-MN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-MY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-MY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-MY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'race-relations-and-racial-policies@en-MY'})
SET t.display_name = 'Race relations and racial policies',
    t.locale = 'en-MY',
    t.term = 'Race relations and racial policies',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Race relations and racial policies in en-MY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-MY'})
MATCH (t:Taboo {key: 'race-relations-and-racial-policies@en-MY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-issues-and-comparisons@en-MY'})
SET t.display_name = 'Religious issues and comparisons',
    t.locale = 'en-MY',
    t.term = 'Religious issues and comparisons',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious issues and comparisons in en-MY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-MY'})
MATCH (t:Taboo {key: 'religious-issues-and-comparisons@en-MY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'royalty-raja-raja-melayu@en-MY'})
SET t.display_name = 'Royalty (Raja-raja Melayu)',
    t.locale = 'en-MY',
    t.term = 'Royalty (Raja-raja Melayu)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Royalty (Raja-raja Melayu) in en-MY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-MY'})
MATCH (t:Taboo {key: 'royalty-raja-raja-melayu@en-MY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// bn-BD Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'bn-BD'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-BD'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam@bn-BD'})
SET t.display_name = 'Insulting Islam',
    t.locale = 'bn-BD',
    t.term = 'Insulting Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam in bn-BD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-BD'})
MATCH (t:Taboo {key: 'insulting-islam@bn-BD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '1971-liberation-war-denialism@bn-BD'})
SET t.display_name = '1971 Liberation War denialism',
    t.locale = 'bn-BD',
    t.term = '1971 Liberation War denialism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 1971 Liberation War denialism in bn-BD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-BD'})
MATCH (t:Taboo {key: '1971-liberation-war-denialism@bn-BD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'hindu-muslim-communal-tensions@bn-BD'})
SET t.display_name = 'Hindu-Muslim communal tensions',
    t.locale = 'bn-BD',
    t.term = 'Hindu-Muslim communal tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Hindu-Muslim communal tensions in bn-BD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-BD'})
MATCH (t:Taboo {key: 'hindu-muslim-communal-tensions@bn-BD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-TZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-TZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-TZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticizing-nyerere@en-TZ'})
SET t.display_name = 'Criticizing Nyerere',
    t.locale = 'en-TZ',
    t.term = 'Criticizing Nyerere',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticizing Nyerere in en-TZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-TZ'})
MATCH (t:Taboo {key: 'criticizing-nyerere@en-TZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'zanzibar-separatismpolitics@en-TZ'})
SET t.display_name = 'Zanzibar separatism/politics',
    t.locale = 'en-TZ',
    t.term = 'Zanzibar separatism/politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Zanzibar separatism/politics in en-TZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-TZ'})
MATCH (t:Taboo {key: 'zanzibar-separatismpolitics@en-TZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-division@en-TZ'})
SET t.display_name = 'Religious division',
    t.locale = 'en-TZ',
    t.term = 'Religious division',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious division in en-TZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-TZ'})
MATCH (t:Taboo {key: 'religious-division@en-TZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-SG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-SG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-SG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'racial-or-religious-denigration@en-SG'})
SET t.display_name = 'Racial or religious denigration',
    t.locale = 'en-SG',
    t.term = 'Racial or religious denigration',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racial or religious denigration in en-SG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-SG'})
MATCH (t:Taboo {key: 'racial-or-religious-denigration@en-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ta-LK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ta-LK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-LK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-conflictcivil-war@ta-LK'})
SET t.display_name = 'Ethnic conflict/civil war',
    t.locale = 'ta-LK',
    t.term = 'Ethnic conflict/civil war',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic conflict/civil war in ta-LK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-LK'})
MATCH (t:Taboo {key: 'ethnic-conflictcivil-war@ta-LK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ltteterrorism-references@ta-LK'})
SET t.display_name = 'LTTE/terrorism references',
    t.locale = 'ta-LK',
    t.term = 'LTTE/terrorism references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: LTTE/terrorism references in ta-LK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-LK'})
MATCH (t:Taboo {key: 'ltteterrorism-references@ta-LK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'tamil-sinhalese-comparisons@ta-LK'})
SET t.display_name = 'Tamil-Sinhalese comparisons',
    t.locale = 'ta-LK',
    t.term = 'Tamil-Sinhalese comparisons',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Tamil-Sinhalese comparisons in ta-LK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-LK'})
MATCH (t:Taboo {key: 'tamil-sinhalese-comparisons@ta-LK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-ZA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-ZA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'racial-stereotypes@en-ZA'})
SET t.display_name = 'Racial stereotypes',
    t.locale = 'en-ZA',
    t.term = 'Racial stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racial stereotypes in en-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZA'})
MATCH (t:Taboo {key: 'racial-stereotypes@en-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'apartheid-nostalgia@en-ZA'})
SET t.display_name = 'Apartheid nostalgia',
    t.locale = 'en-ZA',
    t.term = 'Apartheid nostalgia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Apartheid nostalgia in en-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZA'})
MATCH (t:Taboo {key: 'apartheid-nostalgia@en-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'land-reform-debates@en-ZA'})
SET t.display_name = 'Land reform debates',
    t.locale = 'en-ZA',
    t.term = 'Land reform debates',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Land reform debates in en-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZA'})
MATCH (t:Taboo {key: 'land-reform-debates@en-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-BB Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-BB'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-BB'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'mocking-bajan-accent-or-dialect@en-BB'})
SET t.display_name = 'Mocking Bajan accent or dialect',
    t.locale = 'en-BB',
    t.term = 'Mocking Bajan accent or dialect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mocking Bajan accent or dialect in en-BB content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-BB'})
MATCH (t:Taboo {key: 'mocking-bajan-accent-or-dialect@en-BB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'slavery-glorification-or-minimization@en-BB'})
SET t.display_name = 'Slavery glorification or minimization',
    t.locale = 'en-BB',
    t.term = 'Slavery glorification or minimization',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Slavery glorification or minimization in en-BB content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-BB'})
MATCH (t:Taboo {key: 'slavery-glorification-or-minimization@en-BB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// si-LK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'si-LK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@si-LK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-conflictcivil-war@si-LK'})
SET t.display_name = 'Ethnic conflict/civil war',
    t.locale = 'si-LK',
    t.term = 'Ethnic conflict/civil war',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic conflict/civil war in si-LK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@si-LK'})
MATCH (t:Taboo {key: 'ethnic-conflictcivil-war@si-LK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ltteterrorism-references@si-LK'})
SET t.display_name = 'LTTE/terrorism references',
    t.locale = 'si-LK',
    t.term = 'LTTE/terrorism references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: LTTE/terrorism references in si-LK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@si-LK'})
MATCH (t:Taboo {key: 'ltteterrorism-references@si-LK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-insulting@si-LK'})
SET t.display_name = 'Religious insulting',
    t.locale = 'si-LK',
    t.term = 'Religious insulting',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious insulting in si-LK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@si-LK'})
MATCH (t:Taboo {key: 'religious-insulting@si-LK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// bs-BA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'bs-BA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@bs-BA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-attribution-or-nationalism@bs-BA'})
SET t.display_name = 'Ethnic attribution or nationalism',
    t.locale = 'bs-BA',
    t.term = 'Ethnic attribution or nationalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic attribution or nationalism in bs-BA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bs-BA'})
MATCH (t:Taboo {key: 'ethnic-attribution-or-nationalism@bs-BA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '1992-1995-war-references@bs-BA'})
SET t.display_name = '1992-1995 war references',
    t.locale = 'bs-BA',
    t.term = '1992-1995 war references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 1992-1995 war references in bs-BA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bs-BA'})
MATCH (t:Taboo {key: '1992-1995-war-references@bs-BA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-comparisons@bs-BA'})
SET t.display_name = 'Religious comparisons',
    t.locale = 'bs-BA',
    t.term = 'Religious comparisons',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious comparisons in bs-BA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bs-BA'})
MATCH (t:Taboo {key: 'religious-comparisons@bs-BA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// it-CH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'it-CH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-CH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'treating-swiss-italian-as-just-italian@it-CH'})
SET t.display_name = 'Treating Swiss Italian as "just Italian"',
    t.locale = 'it-CH',
    t.term = 'Treating Swiss Italian as "just Italian"',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Treating Swiss Italian as "just Italian" in it-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-CH'})
MATCH (t:Taboo {key: 'treating-swiss-italian-as-just-italian@it-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'implying-ticino-is-less-swiss@it-CH'})
SET t.display_name = 'Implying Ticino is less Swiss',
    t.locale = 'it-CH',
    t.term = 'Implying Ticino is less Swiss',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Implying Ticino is less Swiss in it-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-CH'})
MATCH (t:Taboo {key: 'implying-ticino-is-less-swiss@it-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'language-region-favoritism@it-CH'})
SET t.display_name = 'Language region favoritism',
    t.locale = 'it-CH',
    t.term = 'Language region favoritism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Language region favoritism in it-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-CH'})
MATCH (t:Taboo {key: 'language-region-favoritism@it-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// tr-TR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'tr-TR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@tr-TR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-or-disrespect-of-ataturk@tr-TR'})
SET t.display_name = 'Criticism or disrespect of Ataturk',
    t.locale = 'tr-TR',
    t.term = 'Criticism or disrespect of Ataturk',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism or disrespect of Ataturk in tr-TR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tr-TR'})
MATCH (t:Taboo {key: 'criticism-or-disrespect-of-ataturk@tr-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'recognition-of-armenian-genocide@tr-TR'})
SET t.display_name = 'Recognition of Armenian "genocide"',
    t.locale = 'tr-TR',
    t.term = 'Recognition of Armenian "genocide"',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Recognition of Armenian "genocide" in tr-TR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tr-TR'})
MATCH (t:Taboo {key: 'recognition-of-armenian-genocide@tr-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'kurdish-separatismpkk-support@tr-TR'})
SET t.display_name = 'Kurdish separatism/PKK support',
    t.locale = 'tr-TR',
    t.term = 'Kurdish separatism/PKK support',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Kurdish separatism/PKK support in tr-TR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tr-TR'})
MATCH (t:Taboo {key: 'kurdish-separatismpkk-support@tr-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-MX Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-MX'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-MX'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'us-mexico-war-1846-1848@es-MX'})
SET t.display_name = 'US-Mexico War 1846-1848',
    t.locale = 'es-MX',
    t.term = 'US-Mexico War 1846-1848',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: US-Mexico War 1846-1848 in es-MX content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-MX'})
MATCH (t:Taboo {key: 'us-mexico-war-1846-1848@es-MX'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'la-malincheconquest-narrative@es-MX'})
SET t.display_name = 'La Malinche/Conquest narrative',
    t.locale = 'es-MX',
    t.term = 'La Malinche/Conquest narrative',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: La Malinche/Conquest narrative in es-MX content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-MX'})
MATCH (t:Taboo {key: 'la-malincheconquest-narrative@es-MX'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'drug-cartels-and-narco-violence@es-MX'})
SET t.display_name = 'Drug cartels and narco-violence',
    t.locale = 'es-MX',
    t.term = 'Drug cartels and narco-violence',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Drug cartels and narco-violence in es-MX content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-MX'})
MATCH (t:Taboo {key: 'drug-cartels-and-narco-violence@es-MX'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-MA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-MA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-MA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'western-sahara-sovereignty@ar-MA'})
SET t.display_name = 'Western Sahara sovereignty',
    t.locale = 'ar-MA',
    t.term = 'Western Sahara sovereignty',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Western Sahara sovereignty in ar-MA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-MA'})
MATCH (t:Taboo {key: 'western-sahara-sovereignty@ar-MA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-the-monarchy@ar-MA'})
SET t.display_name = 'Criticism of the monarchy',
    t.locale = 'ar-MA',
    t.term = 'Criticism of the monarchy',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of the monarchy in ar-MA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-MA'})
MATCH (t:Taboo {key: 'criticism-of-the-monarchy@ar-MA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insulting-islam-or-the-prophet@ar-MA'})
SET t.display_name = 'Insulting Islam or the Prophet',
    t.locale = 'ar-MA',
    t.term = 'Insulting Islam or the Prophet',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or the Prophet in ar-MA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-MA'})
MATCH (t:Taboo {key: 'insulting-islam-or-the-prophet@ar-MA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-LY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-LY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-LY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-islam-or-islamic-values@ar-LY'})
SET t.display_name = 'Criticism of Islam or Islamic values',
    t.locale = 'ar-LY',
    t.term = 'Criticism of Islam or Islamic values',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Islam or Islamic values in ar-LY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-LY'})
MATCH (t:Taboo {key: 'criticism-of-islam-or-islamic-values@ar-LY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-libya-or-libyan-identity@ar-LY'})
SET t.display_name = 'Criticism of Libya or Libyan identity',
    t.locale = 'ar-LY',
    t.term = 'Criticism of Libya or Libyan identity',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Libya or Libyan identity in ar-LY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-LY'})
MATCH (t:Taboo {key: 'criticism-of-libya-or-libyan-identity@ar-LY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'current-political-factions@ar-LY'})
SET t.display_name = 'Current political factions',
    t.locale = 'ar-LY',
    t.term = 'Current political factions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Current political factions in ar-LY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-LY'})
MATCH (t:Taboo {key: 'current-political-factions@ar-LY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-MG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-MG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-MG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'violations-de-fady@fr-MG'})
SET t.display_name = 'Violations de fady',
    t.locale = 'fr-MG',
    t.term = 'Violations de fady',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Violations de fady in fr-MG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-MG'})
MATCH (t:Taboo {key: 'violations-de-fady@fr-MG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'irrespect-envers-les-ancetres-razana@fr-MG'})
SET t.display_name = 'Irrespect envers les ancetres (razana)',
    t.locale = 'fr-MG',
    t.term = 'Irrespect envers les ancetres (razana)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Irrespect envers les ancetres (razana) in fr-MG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-MG'})
MATCH (t:Taboo {key: 'irrespect-envers-les-ancetres-razana@fr-MG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-EG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-EG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-EG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-religious-figures@ar-EG'})
SET t.display_name = 'Insulting Islam or religious figures',
    t.locale = 'ar-EG',
    t.term = 'Insulting Islam or religious figures',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or religious figures in ar-EG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-EG'})
MATCH (t:Taboo {key: 'insulting-islam-or-religious-figures@ar-EG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-commentary-on-current-events@ar-EG'})
SET t.display_name = 'Political commentary on current events',
    t.locale = 'ar-EG',
    t.term = 'Political commentary on current events',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political commentary on current events in ar-EG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-EG'})
MATCH (t:Taboo {key: 'political-commentary-on-current-events@ar-EG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'israel-references@ar-EG'})
SET t.display_name = 'Israel references',
    t.locale = 'ar-EG',
    t.term = 'Israel references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Israel references in ar-EG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-EG'})
MATCH (t:Taboo {key: 'israel-references@ar-EG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// my-MM Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'my-MM'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@my-MM'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'political-situation-and-military@my-MM'})
SET t.display_name = 'Political situation and military',
    t.locale = 'my-MM',
    t.term = 'Political situation and military',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political situation and military in my-MM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@my-MM'})
MATCH (t:Taboo {key: 'political-situation-and-military@my-MM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'buddhist-disrespect@my-MM'})
SET t.display_name = 'Buddhist disrespect',
    t.locale = 'my-MM',
    t.term = 'Buddhist disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Buddhist disrespect in my-MM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@my-MM'})
MATCH (t:Taboo {key: 'buddhist-disrespect@my-MM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-conflicts@my-MM'})
SET t.display_name = 'Ethnic conflicts',
    t.locale = 'my-MM',
    t.term = 'Ethnic conflicts',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic conflicts in my-MM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@my-MM'})
MATCH (t:Taboo {key: 'ethnic-conflicts@my-MM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ta-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ta-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'tamil-language-disrespect@ta-IN'})
SET t.display_name = 'Tamil language disrespect',
    t.locale = 'ta-IN',
    t.term = 'Tamil language disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Tamil language disrespect in ta-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-IN'})
MATCH (t:Taboo {key: 'tamil-language-disrespect@ta-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'caste-discrimination@ta-IN'})
SET t.display_name = 'Caste discrimination',
    t.locale = 'ta-IN',
    t.term = 'Caste discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Caste discrimination in ta-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-IN'})
MATCH (t:Taboo {key: 'caste-discrimination@ta-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'hindi-imposition@ta-IN'})
SET t.display_name = 'Hindi imposition',
    t.locale = 'ta-IN',
    t.term = 'Hindi imposition',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Hindi imposition in ta-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-IN'})
MATCH (t:Taboo {key: 'hindi-imposition@ta-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-CD Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CD'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CD'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'divisions-ethniques-et-tribalisme@fr-CD'})
SET t.display_name = 'Divisions ethniques et tribalisme',
    t.locale = 'fr-CD',
    t.term = 'Divisions ethniques et tribalisme',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Divisions ethniques et tribalisme in fr-CD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CD'})
MATCH (t:Taboo {key: 'divisions-ethniques-et-tribalisme@fr-CD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'commentaire-politique-gouvernement-actuel@fr-CD'})
SET t.display_name = 'Commentaire politique (gouvernement actuel)',
    t.locale = 'fr-CD',
    t.term = 'Commentaire politique (gouvernement actuel)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Commentaire politique (gouvernement actuel) in fr-CD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CD'})
MATCH (t:Taboo {key: 'commentaire-politique-gouvernement-actuel@fr-CD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'conflit-a-l@fr-CD'})
SET t.display_name = 'Conflit a l\\',
    t.locale = 'fr-CD',
    t.term = 'Conflit a l\\',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Conflit a l\\ in fr-CD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CD'})
MATCH (t:Taboo {key: 'conflit-a-l@fr-CD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// as-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'as-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@as-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'nrccitizenship-issues@as-IN'})
SET t.display_name = 'NRC/citizenship issues',
    t.locale = 'as-IN',
    t.term = 'NRC/citizenship issues',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: NRC/citizenship issues in as-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@as-IN'})
MATCH (t:Taboo {key: 'nrccitizenship-issues@as-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'immigrationdemographic-change@as-IN'})
SET t.display_name = 'Immigration/demographic change',
    t.locale = 'as-IN',
    t.term = 'Immigration/demographic change',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Immigration/demographic change in as-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@as-IN'})
MATCH (t:Taboo {key: 'immigrationdemographic-change@as-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'bodo-assamese-relations@as-IN'})
SET t.display_name = 'Bodo-Assamese relations',
    t.locale = 'as-IN',
    t.term = 'Bodo-Assamese relations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Bodo-Assamese relations in as-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@as-IN'})
MATCH (t:Taboo {key: 'bodo-assamese-relations@as-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// lt-LT Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'lt-LT'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@lt-LT'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'soviet-nostalgia-or-symbolism@lt-LT'})
SET t.display_name = 'Soviet nostalgia or symbolism',
    t.locale = 'lt-LT',
    t.term = 'Soviet nostalgia or symbolism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Soviet nostalgia or symbolism in lt-LT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@lt-LT'})
MATCH (t:Taboo {key: 'soviet-nostalgia-or-symbolism@lt-LT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'pro-russian-political-messaging@lt-LT'})
SET t.display_name = 'Pro-Russian political messaging',
    t.locale = 'lt-LT',
    t.term = 'Pro-Russian political messaging',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Pro-Russian political messaging in lt-LT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@lt-LT'})
MATCH (t:Taboo {key: 'pro-russian-political-messaging@lt-LT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'comparing-lithuania-to-russia@lt-LT'})
SET t.display_name = 'Comparing Lithuania to Russia',
    t.locale = 'lt-LT',
    t.term = 'Comparing Lithuania to Russia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparing Lithuania to Russia in lt-LT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@lt-LT'})
MATCH (t:Taboo {key: 'comparing-lithuania-to-russia@lt-LT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// sw-KE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sw-KE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-KE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-stereotyping-tribalism@sw-KE'})
SET t.display_name = 'Ethnic stereotyping (tribalism)',
    t.locale = 'sw-KE',
    t.term = 'Ethnic stereotyping (tribalism)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic stereotyping (tribalism) in sw-KE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-KE'})
MATCH (t:Taboo {key: 'ethnic-stereotyping-tribalism@sw-KE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'electionscampaigns@sw-KE'})
SET t.display_name = 'Elections/campaigns',
    t.locale = 'sw-KE',
    t.term = 'Elections/campaigns',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Elections/campaigns in sw-KE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-KE'})
MATCH (t:Taboo {key: 'electionscampaigns@sw-KE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// hy-AM Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'hy-AM'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@hy-AM'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'armenian-genocide-denial-or-minimization@hy-AM'})
SET t.display_name = 'Armenian Genocide denial or minimization',
    t.locale = 'hy-AM',
    t.term = 'Armenian Genocide denial or minimization',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Armenian Genocide denial or minimization in hy-AM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hy-AM'})
MATCH (t:Taboo {key: 'armenian-genocide-denial-or-minimization@hy-AM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'pro-turkish-or-pro-azerbaijani-political-messaging@hy-AM'})
SET t.display_name = 'Pro-Turkish or pro-Azerbaijani political messaging',
    t.locale = 'hy-AM',
    t.term = 'Pro-Turkish or pro-Azerbaijani political messaging',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Pro-Turkish or pro-Azerbaijani political messaging in hy-AM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hy-AM'})
MATCH (t:Taboo {key: 'pro-turkish-or-pro-azerbaijani-political-messaging@hy-AM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'questioning-armenian-christianity@hy-AM'})
SET t.display_name = 'Questioning Armenian Christianity',
    t.locale = 'hy-AM',
    t.term = 'Questioning Armenian Christianity',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Questioning Armenian Christianity in hy-AM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hy-AM'})
MATCH (t:Taboo {key: 'questioning-armenian-christianity@hy-AM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// pt-MZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-MZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-MZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'civil-war-blame@pt-MZ'})
SET t.display_name = 'Civil War blame',
    t.locale = 'pt-MZ',
    t.term = 'Civil War blame',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Civil War blame in pt-MZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-MZ'})
MATCH (t:Taboo {key: 'civil-war-blame@pt-MZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-divisions@pt-MZ'})
SET t.display_name = 'Ethnic divisions',
    t.locale = 'pt-MZ',
    t.term = 'Ethnic divisions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic divisions in pt-MZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-MZ'})
MATCH (t:Taboo {key: 'ethnic-divisions@pt-MZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'north-south-disparities@pt-MZ'})
SET t.display_name = 'North-South disparities',
    t.locale = 'pt-MZ',
    t.term = 'North-South disparities',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: North-South disparities in pt-MZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-MZ'})
MATCH (t:Taboo {key: 'north-south-disparities@pt-MZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// nl-BE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'nl-BE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-BE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'flemish-walloon-tensions@nl-BE'})
SET t.display_name = 'Flemish-Walloon tensions',
    t.locale = 'nl-BE',
    t.term = 'Flemish-Walloon tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Flemish-Walloon tensions in nl-BE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-BE'})
MATCH (t:Taboo {key: 'flemish-walloon-tensions@nl-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'linguistic-politics@nl-BE'})
SET t.display_name = 'Linguistic politics',
    t.locale = 'nl-BE',
    t.term = 'Linguistic politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Linguistic politics in nl-BE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-BE'})
MATCH (t:Taboo {key: 'linguistic-politics@nl-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// te-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'te-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@te-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'telugu-language-disrespect@te-IN'})
SET t.display_name = 'Telugu language disrespect',
    t.locale = 'te-IN',
    t.term = 'Telugu language disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Telugu language disrespect in te-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@te-IN'})
MATCH (t:Taboo {key: 'telugu-language-disrespect@te-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'andhra-telangana-division@te-IN'})
SET t.display_name = 'Andhra-Telangana division',
    t.locale = 'te-IN',
    t.term = 'Andhra-Telangana division',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Andhra-Telangana division in te-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@te-IN'})
MATCH (t:Taboo {key: 'andhra-telangana-division@te-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-communalism@te-IN'})
SET t.display_name = 'Religious communalism',
    t.locale = 'te-IN',
    t.term = 'Religious communalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious communalism in te-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@te-IN'})
MATCH (t:Taboo {key: 'religious-communalism@te-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ml-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ml-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ml-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'caste-discrimination@ml-IN'})
SET t.display_name = 'Caste discrimination',
    t.locale = 'ml-IN',
    t.term = 'Caste discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Caste discrimination in ml-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ml-IN'})
MATCH (t:Taboo {key: 'caste-discrimination@ml-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-communalism@ml-IN'})
SET t.display_name = 'Religious communalism',
    t.locale = 'ml-IN',
    t.term = 'Religious communalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious communalism in ml-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ml-IN'})
MATCH (t:Taboo {key: 'religious-communalism@ml-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'communal-incidents@ml-IN'})
SET t.display_name = 'Communal incidents',
    t.locale = 'ml-IN',
    t.term = 'Communal incidents',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Communal incidents in ml-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ml-IN'})
MATCH (t:Taboo {key: 'communal-incidents@ml-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// vi-VN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'vi-VN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@vi-VN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-communist-partygovernment@vi-VN'})
SET t.display_name = 'Criticism of Communist Party/government',
    t.locale = 'vi-VN',
    t.term = 'Criticism of Communist Party/government',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Communist Party/government in vi-VN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@vi-VN'})
MATCH (t:Taboo {key: 'criticism-of-communist-partygovernment@vi-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'challenging-national-sovereignty-south-china-sea@vi-VN'})
SET t.display_name = 'Challenging national sovereignty (South China Sea',
    t.locale = 'vi-VN',
    t.term = 'Challenging national sovereignty (South China Sea',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Challenging national sovereignty (South China Sea in vi-VN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@vi-VN'})
MATCH (t:Taboo {key: 'challenging-national-sovereignty-south-china-sea@vi-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'etc@vi-VN'})
SET t.display_name = 'etc.)',
    t.locale = 'vi-VN',
    t.term = 'etc.)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: etc.) in vi-VN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@vi-VN'})
MATCH (t:Taboo {key: 'etc@vi-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'south-china-sea-bien-dong@vi-VN'})
SET t.display_name = 'South China Sea / Bien Dong',
    t.locale = 'vi-VN',
    t.term = 'South China Sea / Bien Dong',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: South China Sea / Bien Dong in vi-VN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@vi-VN'})
MATCH (t:Taboo {key: 'south-china-sea-bien-dong@vi-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-BE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-BE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'linguistic-tensions-flemish-vs-walloon@fr-BE'})
SET t.display_name = 'Linguistic tensions (Flemish vs Walloon)',
    t.locale = 'fr-BE',
    t.term = 'Linguistic tensions (Flemish vs Walloon)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Linguistic tensions (Flemish vs Walloon) in fr-BE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BE'})
MATCH (t:Taboo {key: 'linguistic-tensions-flemish-vs-walloon@fr-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'comparing-to-france-dismissively@fr-BE'})
SET t.display_name = 'Comparing to France dismissively',
    t.locale = 'fr-BE',
    t.term = 'Comparing to France dismissively',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparing to France dismissively in fr-BE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BE'})
MATCH (t:Taboo {key: 'comparing-to-france-dismissively@fr-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'congo-colonization-1885-1960@fr-BE'})
SET t.display_name = 'Congo colonization (1885-1960)',
    t.locale = 'fr-BE',
    t.term = 'Congo colonization (1885-1960)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Congo colonization (1885-1960) in fr-BE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BE'})
MATCH (t:Taboo {key: 'congo-colonization-1885-1960@fr-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ru-IL Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-IL'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-IL'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'holocaustshoah@ru-IL'})
SET t.display_name = 'Holocaust/Shoah',
    t.locale = 'ru-IL',
    t.term = 'Holocaust/Shoah',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Holocaust/Shoah in ru-IL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-IL'})
MATCH (t:Taboo {key: 'holocaustshoah@ru-IL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'soviet-persecution-of-jews@ru-IL'})
SET t.display_name = 'Soviet Persecution of Jews',
    t.locale = 'ru-IL',
    t.term = 'Soviet Persecution of Jews',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Soviet Persecution of Jews in ru-IL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-IL'})
MATCH (t:Taboo {key: 'soviet-persecution-of-jews@ru-IL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'israeli-palestinian-conflict@ru-IL'})
SET t.display_name = 'Israeli-Palestinian Conflict',
    t.locale = 'ru-IL',
    t.term = 'Israeli-Palestinian Conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Israeli-Palestinian Conflict in ru-IL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-IL'})
MATCH (t:Taboo {key: 'israeli-palestinian-conflict@ru-IL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// rw-RW Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'rw-RW'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@rw-RW'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-references-hutututsitwa@rw-RW'})
SET t.display_name = 'Ethnic references (Hutu/Tutsi/Twa)',
    t.locale = 'rw-RW',
    t.term = 'Ethnic references (Hutu/Tutsi/Twa)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic references (Hutu/Tutsi/Twa) in rw-RW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@rw-RW'})
MATCH (t:Taboo {key: 'ethnic-references-hutututsitwa@rw-RW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '1994-genocide-denialminimization@rw-RW'})
SET t.display_name = '1994 Genocide denial/minimization',
    t.locale = 'rw-RW',
    t.term = '1994 Genocide denial/minimization',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 1994 Genocide denial/minimization in rw-RW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@rw-RW'})
MATCH (t:Taboo {key: '1994-genocide-denialminimization@rw-RW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'divisionism-ubwoko@rw-RW'})
SET t.display_name = 'Divisionism (ubwoko)',
    t.locale = 'rw-RW',
    t.term = 'Divisionism (ubwoko)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Divisionism (ubwoko) in rw-RW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@rw-RW'})
MATCH (t:Taboo {key: 'divisionism-ubwoko@rw-RW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-KY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-KY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-KY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'tax-haven-stereotypes@en-KY'})
SET t.display_name = 'Tax haven stereotypes',
    t.locale = 'en-KY',
    t.term = 'Tax haven stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Tax haven stereotypes in en-KY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-KY'})
MATCH (t:Taboo {key: 'tax-haven-stereotypes@en-KY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'money-laundering-accusations@en-KY'})
SET t.display_name = 'Money laundering accusations',
    t.locale = 'en-KY',
    t.term = 'Money laundering accusations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Money laundering accusations in en-KY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-KY'})
MATCH (t:Taboo {key: 'money-laundering-accusations@en-KY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ky-KG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ky-KG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ky-KG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-tensions-kyrgyz-uzbek@ky-KG'})
SET t.display_name = 'Ethnic tensions (Kyrgyz-Uzbek)',
    t.locale = 'ky-KG',
    t.term = 'Ethnic tensions (Kyrgyz-Uzbek)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic tensions (Kyrgyz-Uzbek) in ky-KG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ky-KG'})
MATCH (t:Taboo {key: 'ethnic-tensions-kyrgyz-uzbek@ky-KG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-criticism-current-government@ky-KG'})
SET t.display_name = 'Political criticism (current government)',
    t.locale = 'ky-KG',
    t.term = 'Political criticism (current government)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political criticism (current government) in ky-KG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ky-KG'})
MATCH (t:Taboo {key: 'political-criticism-current-government@ky-KG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'border-disputes@ky-KG'})
SET t.display_name = 'Border disputes',
    t.locale = 'ky-KG',
    t.term = 'Border disputes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Border disputes in ky-KG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ky-KG'})
MATCH (t:Taboo {key: 'border-disputes@ky-KG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-IE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-IE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'british-stereotyping-of-ireland@en-IE'})
SET t.display_name = 'British stereotyping of Ireland',
    t.locale = 'en-IE',
    t.term = 'British stereotyping of Ireland',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: British stereotyping of Ireland in en-IE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IE'})
MATCH (t:Taboo {key: 'british-stereotyping-of-ireland@en-IE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'northern-ireland-conflict@en-IE'})
SET t.display_name = 'Northern Ireland conflict',
    t.locale = 'en-IE',
    t.term = 'Northern Ireland conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Northern Ireland conflict in en-IE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IE'})
MATCH (t:Taboo {key: 'northern-ireland-conflict@en-IE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'famine-references@en-IE'})
SET t.display_name = 'Famine references',
    t.locale = 'en-IE',
    t.term = 'Famine references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Famine references in en-IE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IE'})
MATCH (t:Taboo {key: 'famine-references@en-IE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// su-ID Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'su-ID'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@su-ID'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-religion@su-ID'})
SET t.display_name = 'Insulting Islam or religion',
    t.locale = 'su-ID',
    t.term = 'Insulting Islam or religion',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or religion in su-ID content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@su-ID'})
MATCH (t:Taboo {key: 'insulting-islam-or-religion@su-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'causing-public-isin-shame@su-ID'})
SET t.display_name = 'Causing public isin (shame)',
    t.locale = 'su-ID',
    t.term = 'Causing public isin (shame)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Causing public isin (shame) in su-ID content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@su-ID'})
MATCH (t:Taboo {key: 'causing-public-isin-shame@su-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'direct-confrontation-or-criticism@su-ID'})
SET t.display_name = 'Direct confrontation or criticism',
    t.locale = 'su-ID',
    t.term = 'Direct confrontation or criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Direct confrontation or criticism in su-ID content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@su-ID'})
MATCH (t:Taboo {key: 'direct-confrontation-or-criticism@su-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-UG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-UG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-UG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'lgbtq-topics@en-UG'})
SET t.display_name = 'LGBTQ+ topics',
    t.locale = 'en-UG',
    t.term = 'LGBTQ+ topics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: LGBTQ+ topics in en-UG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-UG'})
MATCH (t:Taboo {key: 'lgbtq-topics@en-UG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnictribal-tensions@en-UG'})
SET t.display_name = 'Ethnic/tribal tensions',
    t.locale = 'en-UG',
    t.term = 'Ethnic/tribal tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic/tribal tensions in en-UG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-UG'})
MATCH (t:Taboo {key: 'ethnictribal-tensions@en-UG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'current-political-opposition@en-UG'})
SET t.display_name = 'Current political opposition',
    t.locale = 'en-UG',
    t.term = 'Current political opposition',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Current political opposition in en-UG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-UG'})
MATCH (t:Taboo {key: 'current-political-opposition@en-UG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// sw-TZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sw-TZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-TZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-nyerere@sw-TZ'})
SET t.display_name = 'Criticism of Nyerere',
    t.locale = 'sw-TZ',
    t.term = 'Criticism of Nyerere',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Nyerere in sw-TZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-TZ'})
MATCH (t:Taboo {key: 'criticism-of-nyerere@sw-TZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'tribalethnic-divisions@sw-TZ'})
SET t.display_name = 'Tribal/Ethnic divisions',
    t.locale = 'sw-TZ',
    t.term = 'Tribal/Ethnic divisions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Tribal/Ethnic divisions in sw-TZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-TZ'})
MATCH (t:Taboo {key: 'tribalethnic-divisions@sw-TZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'zanzibar-mainland-tensions@sw-TZ'})
SET t.display_name = 'Zanzibar-Mainland tensions',
    t.locale = 'sw-TZ',
    t.term = 'Zanzibar-Mainland tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Zanzibar-Mainland tensions in sw-TZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-TZ'})
MATCH (t:Taboo {key: 'zanzibar-mainland-tensions@sw-TZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// hu-HU Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'hu-HU'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@hu-HU'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'treaty-of-trianon-1920@hu-HU'})
SET t.display_name = 'Treaty of Trianon (1920)',
    t.locale = 'hu-HU',
    t.term = 'Treaty of Trianon (1920)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Treaty of Trianon (1920) in hu-HU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hu-HU'})
MATCH (t:Taboo {key: 'treaty-of-trianon-1920@hu-HU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'conflating-hungarians-with-slavs@hu-HU'})
SET t.display_name = 'Conflating Hungarians with Slavs',
    t.locale = 'hu-HU',
    t.term = 'Conflating Hungarians with Slavs',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Conflating Hungarians with Slavs in hu-HU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hu-HU'})
MATCH (t:Taboo {key: 'conflating-hungarians-with-slavs@hu-HU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'treaty-of-trianon-1920@hu-HU'})
SET t.display_name = 'Treaty of Trianon (1920)',
    t.locale = 'hu-HU',
    t.term = 'Treaty of Trianon (1920)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Treaty of Trianon (1920) in hu-HU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hu-HU'})
MATCH (t:Taboo {key: 'treaty-of-trianon-1920@hu-HU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-EC Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-EC'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-EC'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'dollarization-criticism@es-EC'})
SET t.display_name = 'Dollarization criticism',
    t.locale = 'es-EC',
    t.term = 'Dollarization criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Dollarization criticism in es-EC content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-EC'})
MATCH (t:Taboo {key: 'dollarization-criticism@es-EC'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'indigenous-discrimination@es-EC'})
SET t.display_name = 'Indigenous discrimination',
    t.locale = 'es-EC',
    t.term = 'Indigenous discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Indigenous discrimination in es-EC content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-EC'})
MATCH (t:Taboo {key: 'indigenous-discrimination@es-EC'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'regional-mockery@es-EC'})
SET t.display_name = 'Regional mockery',
    t.locale = 'es-EC',
    t.term = 'Regional mockery',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Regional mockery in es-EC content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-EC'})
MATCH (t:Taboo {key: 'regional-mockery@es-EC'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// mg-MG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mg-MG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@mg-MG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'fady-violations@mg-MG'})
SET t.display_name = 'Fady violations',
    t.locale = 'mg-MG',
    t.term = 'Fady violations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Fady violations in mg-MG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mg-MG'})
MATCH (t:Taboo {key: 'fady-violations@mg-MG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'disrespecting-ancestors-razana@mg-MG'})
SET t.display_name = 'Disrespecting ancestors (razana)',
    t.locale = 'mg-MG',
    t.term = 'Disrespecting ancestors (razana)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting ancestors (razana) in mg-MG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mg-MG'})
MATCH (t:Taboo {key: 'disrespecting-ancestors-razana@mg-MG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ca-AD Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ca-AD'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ca-AD'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'sovereignty-threats@ca-AD'})
SET t.display_name = 'Sovereignty threats',
    t.locale = 'ca-AD',
    t.term = 'Sovereignty threats',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Sovereignty threats in ca-AD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ca-AD'})
MATCH (t:Taboo {key: 'sovereignty-threats@ca-AD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'tax-haven-characterization@ca-AD'})
SET t.display_name = 'Tax haven characterization',
    t.locale = 'ca-AD',
    t.term = 'Tax haven characterization',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Tax haven characterization in ca-AD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ca-AD'})
MATCH (t:Taboo {key: 'tax-haven-characterization@ca-AD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ko-KR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ko-KR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ko-KR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'japanese-colonial-period-1910-1945@ko-KR'})
SET t.display_name = 'Japanese Colonial Period (1910-1945)',
    t.locale = 'ko-KR',
    t.term = 'Japanese Colonial Period (1910-1945)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Japanese Colonial Period (1910-1945) in ko-KR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ko-KR'})
MATCH (t:Taboo {key: 'japanese-colonial-period-1910-1945@ko-KR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'north-korea-politics@ko-KR'})
SET t.display_name = 'North Korea Politics',
    t.locale = 'ko-KR',
    t.term = 'North Korea Politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: North Korea Politics in ko-KR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ko-KR'})
MATCH (t:Taboo {key: 'north-korea-politics@ko-KR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'territorial-disputes-dokdo@ko-KR'})
SET t.display_name = 'Territorial Disputes (Dokdo/독도)',
    t.locale = 'ko-KR',
    t.term = 'Territorial Disputes (Dokdo/독도)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Territorial Disputes (Dokdo/독도) in ko-KR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ko-KR'})
MATCH (t:Taboo {key: 'territorial-disputes-dokdo@ko-KR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ln-CD Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ln-CD'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ln-CD'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-divisions-and-tribalism@ln-CD'})
SET t.display_name = 'Ethnic divisions and tribalism',
    t.locale = 'ln-CD',
    t.term = 'Ethnic divisions and tribalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic divisions and tribalism in ln-CD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ln-CD'})
MATCH (t:Taboo {key: 'ethnic-divisions-and-tribalism@ln-CD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-commentary-current-government@ln-CD'})
SET t.display_name = 'Political commentary (current government)',
    t.locale = 'ln-CD',
    t.term = 'Political commentary (current government)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political commentary (current government) in ln-CD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ln-CD'})
MATCH (t:Taboo {key: 'political-commentary-current-government@ln-CD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'eastern-drc-conflict@ln-CD'})
SET t.display_name = 'Eastern DRC conflict',
    t.locale = 'ln-CD',
    t.term = 'Eastern DRC conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Eastern DRC conflict in ln-CD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ln-CD'})
MATCH (t:Taboo {key: 'eastern-drc-conflict@ln-CD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-KE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-KE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-KE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnictribal-tensions@en-KE'})
SET t.display_name = 'Ethnic/tribal tensions',
    t.locale = 'en-KE',
    t.term = 'Ethnic/tribal tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic/tribal tensions in en-KE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-KE'})
MATCH (t:Taboo {key: 'ethnictribal-tensions@en-KE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-tribalism@en-KE'})
SET t.display_name = 'Political tribalism',
    t.locale = 'en-KE',
    t.term = 'Political tribalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political tribalism in en-KE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-KE'})
MATCH (t:Taboo {key: 'political-tribalism@en-KE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '2007-2008-post-election-violence@en-KE'})
SET t.display_name = '2007-2008 Post-Election Violence',
    t.locale = 'en-KE',
    t.term = '2007-2008 Post-Election Violence',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 2007-2008 Post-Election Violence in en-KE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-KE'})
MATCH (t:Taboo {key: '2007-2008-post-election-violence@en-KE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-DZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-DZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-DZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam@ar-DZ'})
SET t.display_name = 'Insulting Islam',
    t.locale = 'ar-DZ',
    t.term = 'Insulting Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam in ar-DZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-DZ'})
MATCH (t:Taboo {key: 'insulting-islam@ar-DZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'french-colonization-praise@ar-DZ'})
SET t.display_name = 'French colonization praise',
    t.locale = 'ar-DZ',
    t.term = 'French colonization praise',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: French colonization praise in ar-DZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-DZ'})
MATCH (t:Taboo {key: 'french-colonization-praise@ar-DZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'morocco-algeria-relations@ar-DZ'})
SET t.display_name = 'Morocco-Algeria relations',
    t.locale = 'ar-DZ',
    t.term = 'Morocco-Algeria relations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Morocco-Algeria relations in ar-DZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-DZ'})
MATCH (t:Taboo {key: 'morocco-algeria-relations@ar-DZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-CA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'souverainete-independance@fr-CA'})
SET t.display_name = 'Souverainete / independance',
    t.locale = 'fr-CA',
    t.term = 'Souverainete / independance',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Souverainete / independance in fr-CA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CA'})
MATCH (t:Taboo {key: 'souverainete-independance@fr-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'comparaisons-defavorables-avec-la-france@fr-CA'})
SET t.display_name = 'Comparaisons defavorables avec la France',
    t.locale = 'fr-CA',
    t.term = 'Comparaisons defavorables avec la France',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparaisons defavorables avec la France in fr-CA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CA'})
MATCH (t:Taboo {key: 'comparaisons-defavorables-avec-la-france@fr-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'federalisme-vs-souverainisme@fr-CA'})
SET t.display_name = 'Federalisme vs souverainisme',
    t.locale = 'fr-CA',
    t.term = 'Federalisme vs souverainisme',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Federalisme vs souverainisme in fr-CA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CA'})
MATCH (t:Taboo {key: 'federalisme-vs-souverainisme@fr-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-GB Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-GB'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-GB'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'the-troubles-northern-ireland@en-GB'})
SET t.display_name = 'The Troubles (Northern Ireland)',
    t.locale = 'en-GB',
    t.term = 'The Troubles (Northern Ireland)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: The Troubles (Northern Ireland) in en-GB content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-GB'})
MATCH (t:Taboo {key: 'the-troubles-northern-ireland@en-GB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-RW Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-RW'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-RW'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'references-ethniques-hutututsitwa@fr-RW'})
SET t.display_name = 'References ethniques (Hutu/Tutsi/Twa)',
    t.locale = 'fr-RW',
    t.term = 'References ethniques (Hutu/Tutsi/Twa)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: References ethniques (Hutu/Tutsi/Twa) in fr-RW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-RW'})
MATCH (t:Taboo {key: 'references-ethniques-hutututsitwa@fr-RW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'negationminimisation-du-genocide-de-1994@fr-RW'})
SET t.display_name = 'Negation/minimisation du genocide de 1994',
    t.locale = 'fr-RW',
    t.term = 'Negation/minimisation du genocide de 1994',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Negation/minimisation du genocide de 1994 in fr-RW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-RW'})
MATCH (t:Taboo {key: 'negationminimisation-du-genocide-de-1994@fr-RW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'divisionnisme-ubwoko@fr-RW'})
SET t.display_name = 'Divisionnisme (ubwoko)',
    t.locale = 'fr-RW',
    t.term = 'Divisionnisme (ubwoko)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Divisionnisme (ubwoko) in fr-RW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-RW'})
MATCH (t:Taboo {key: 'divisionnisme-ubwoko@fr-RW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ru-BY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-BY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-BY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'current-political-situation@ru-BY'})
SET t.display_name = 'Текущая политическая ситуация (Current political situation)',
    t.locale = 'ru-BY',
    t.term = 'Текущая политическая ситуация (Current political situation)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Текущая политическая ситуация (Current political situation) in ru-BY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-BY'})
MATCH (t:Taboo {key: 'current-political-situation@ru-BY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'government-criticism@ru-BY'})
SET t.display_name = 'Критика государственных институтов (Government criticism)',
    t.locale = 'ru-BY',
    t.term = 'Критика государственных институтов (Government criticism)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Критика государственных институтов (Government criticism) in ru-BY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-BY'})
MATCH (t:Taboo {key: 'government-criticism@ru-BY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '2020-2020-events-and-aftermath@ru-BY'})
SET t.display_name = 'События 2020 года (2020 events and aftermath)',
    t.locale = 'ru-BY',
    t.term = 'События 2020 года (2020 events and aftermath)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: События 2020 года (2020 events and aftermath) in ru-BY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-BY'})
MATCH (t:Taboo {key: '2020-2020-events-and-aftermath@ru-BY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-GT Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-GT'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-GT'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'civil-wararmed-conflict-1960-1996@es-GT'})
SET t.display_name = 'Civil War/Armed Conflict (1960-1996)',
    t.locale = 'es-GT',
    t.term = 'Civil War/Armed Conflict (1960-1996)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Civil War/Armed Conflict (1960-1996) in es-GT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-GT'})
MATCH (t:Taboo {key: 'civil-wararmed-conflict-1960-1996@es-GT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'genocide-and-massacres@es-GT'})
SET t.display_name = 'Genocide and massacres',
    t.locale = 'es-GT',
    t.term = 'Genocide and massacres',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Genocide and massacres in es-GT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-GT'})
MATCH (t:Taboo {key: 'genocide-and-massacres@es-GT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'racialethnic-discrimination@es-GT'})
SET t.display_name = 'Racial/ethnic discrimination',
    t.locale = 'es-GT',
    t.term = 'Racial/ethnic discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racial/ethnic discrimination in es-GT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-GT'})
MATCH (t:Taboo {key: 'racialethnic-discrimination@es-GT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ms-SG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ms-SG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-SG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-any-religion@ms-SG'})
SET t.display_name = 'Insulting Islam or any religion',
    t.locale = 'ms-SG',
    t.term = 'Insulting Islam or any religion',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or any religion in ms-SG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-SG'})
MATCH (t:Taboo {key: 'insulting-islam-or-any-religion@ms-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'non-halal-content-prominently@ms-SG'})
SET t.display_name = 'Non-halal content prominently',
    t.locale = 'ms-SG',
    t.term = 'Non-halal content prominently',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Non-halal content prominently in ms-SG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-SG'})
MATCH (t:Taboo {key: 'non-halal-content-prominently@ms-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'racial-denigration-of-any-group@ms-SG'})
SET t.display_name = 'Racial denigration of any group',
    t.locale = 'ms-SG',
    t.term = 'Racial denigration of any group',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racial denigration of any group in ms-SG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-SG'})
MATCH (t:Taboo {key: 'racial-denigration-of-any-group@ms-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ht-HT Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ht-HT'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ht-HT'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'vodou-mockerysensationalism@ht-HT'})
SET t.display_name = 'Vodou mockery/sensationalism',
    t.locale = 'ht-HT',
    t.term = 'Vodou mockery/sensationalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Vodou mockery/sensationalism in ht-HT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ht-HT'})
MATCH (t:Taboo {key: 'vodou-mockerysensationalism@ht-HT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'voodoo-spellingstereotypes@ht-HT'})
SET t.display_name = '"Voodoo" spelling/stereotypes',
    t.locale = 'ht-HT',
    t.term = '"Voodoo" spelling/stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: "Voodoo" spelling/stereotypes in ht-HT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ht-HT'})
MATCH (t:Taboo {key: 'voodoo-spellingstereotypes@ht-HT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'poverty-tourism@ht-HT'})
SET t.display_name = 'Poverty tourism',
    t.locale = 'ht-HT',
    t.term = 'Poverty tourism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Poverty tourism in ht-HT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ht-HT'})
MATCH (t:Taboo {key: 'poverty-tourism@ht-HT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ms-MY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ms-MY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-MY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-any-religion@ms-MY'})
SET t.display_name = 'Insulting Islam or any religion',
    t.locale = 'ms-MY',
    t.term = 'Insulting Islam or any religion',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or any religion in ms-MY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-MY'})
MATCH (t:Taboo {key: 'insulting-islam-or-any-religion@ms-MY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticizing-royalty@ms-MY'})
SET t.display_name = 'Criticizing royalty',
    t.locale = 'ms-MY',
    t.term = 'Criticizing royalty',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticizing royalty in ms-MY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-MY'})
MATCH (t:Taboo {key: 'criticizing-royalty@ms-MY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'racial-tension-or-preferential-policies@ms-MY'})
SET t.display_name = 'Racial tension or preferential policies',
    t.locale = 'ms-MY',
    t.term = 'Racial tension or preferential policies',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racial tension or preferential policies in ms-MY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-MY'})
MATCH (t:Taboo {key: 'racial-tension-or-preferential-policies@ms-MY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// sq-AL Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sq-AL'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sq-AL'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'communist-era-1944-1991@sq-AL'})
SET t.display_name = 'Communist Era (1944-1991)',
    t.locale = 'sq-AL',
    t.term = 'Communist Era (1944-1991)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Communist Era (1944-1991) in sq-AL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sq-AL'})
MATCH (t:Taboo {key: 'communist-era-1944-1991@sq-AL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'blood-feuds-gjakmarrja@sq-AL'})
SET t.display_name = 'Blood Feuds (Gjakmarrja)',
    t.locale = 'sq-AL',
    t.term = 'Blood Feuds (Gjakmarrja)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Blood Feuds (Gjakmarrja) in sq-AL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sq-AL'})
MATCH (t:Taboo {key: 'blood-feuds-gjakmarrja@sq-AL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// mk-MK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mk-MK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@mk-MK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'country-name-and-dispute-with-greece@mk-MK'})
SET t.display_name = 'Име на државата и спорот со Грција (Country name and dispute with Greece)',
    t.locale = 'mk-MK',
    t.term = 'Име на државата и спорот со Грција (Country name and dispute with Greece)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Име на државата и спорот со Грција (Country name and dispute with Greece) in mk-MK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mk-MK'})
MATCH (t:Taboo {key: 'country-name-and-dispute-with-greece@mk-MK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'bulgarianmacedonian-history-debate@mk-MK'})
SET t.display_name = 'Бугарска/Македонска историја (Bulgarian/Macedonian history debate)',
    t.locale = 'mk-MK',
    t.term = 'Бугарска/Македонска историја (Bulgarian/Macedonian history debate)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Бугарска/Македонска историја (Bulgarian/Macedonian history debate) in mk-MK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mk-MK'})
MATCH (t:Taboo {key: 'bulgarianmacedonian-history-debate@mk-MK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// mr-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mr-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@mr-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'shivaji-maharaj-disrespect@mr-IN'})
SET t.display_name = 'Shivaji Maharaj disrespect',
    t.locale = 'mr-IN',
    t.term = 'Shivaji Maharaj disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Shivaji Maharaj disrespect in mr-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mr-IN'})
MATCH (t:Taboo {key: 'shivaji-maharaj-disrespect@mr-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-communalism@mr-IN'})
SET t.display_name = 'Religious communalism',
    t.locale = 'mr-IN',
    t.term = 'Religious communalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious communalism in mr-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mr-IN'})
MATCH (t:Taboo {key: 'religious-communalism@mr-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'caste-discrimination@mr-IN'})
SET t.display_name = 'Caste discrimination',
    t.locale = 'mr-IN',
    t.term = 'Caste discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Caste discrimination in mr-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mr-IN'})
MATCH (t:Taboo {key: 'caste-discrimination@mr-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-CU Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-CU'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CU'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'political-criticism-within-cuba@es-CU'})
SET t.display_name = 'Political criticism (within Cuba)',
    t.locale = 'es-CU',
    t.term = 'Political criticism (within Cuba)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political criticism (within Cuba) in es-CU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CU'})
MATCH (t:Taboo {key: 'political-criticism-within-cuba@es-CU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'us-cuba-relations-positioning@es-CU'})
SET t.display_name = 'US-Cuba relations positioning',
    t.locale = 'es-CU',
    t.term = 'US-Cuba relations positioning',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: US-Cuba relations positioning in es-CU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CU'})
MATCH (t:Taboo {key: 'us-cuba-relations-positioning@es-CU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'government-and-leadership@es-CU'})
SET t.display_name = 'Government and leadership',
    t.locale = 'es-CU',
    t.term = 'Government and leadership',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Government and leadership in es-CU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CU'})
MATCH (t:Taboo {key: 'government-and-leadership@es-CU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ru-KG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-KG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-KG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: '@ru-KG'})
SET t.display_name = 'Межэтническая напряженность',
    t.locale = 'ru-KG',
    t.term = 'Межэтническая напряженность',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Межэтническая напряженность in ru-KG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-KG'})
MATCH (t:Taboo {key: '@ru-KG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '@ru-KG'})
SET t.display_name = 'Критика правительства',
    t.locale = 'ru-KG',
    t.term = 'Критика правительства',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Критика правительства in ru-KG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-KG'})
MATCH (t:Taboo {key: '@ru-KG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '@ru-KG'})
SET t.display_name = 'Пограничные споры',
    t.locale = 'ru-KG',
    t.term = 'Пограничные споры',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Пограничные споры in ru-KG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-KG'})
MATCH (t:Taboo {key: '@ru-KG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ig-NG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ig-NG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ig-NG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'biafracivil-war@ig-NG'})
SET t.display_name = 'Biafra/Civil War',
    t.locale = 'ig-NG',
    t.term = 'Biafra/Civil War',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Biafra/Civil War in ig-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ig-NG'})
MATCH (t:Taboo {key: 'biafracivil-war@ig-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-stereotypes@ig-NG'})
SET t.display_name = 'Ethnic stereotypes',
    t.locale = 'ig-NG',
    t.term = 'Ethnic stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic stereotypes in ig-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ig-NG'})
MATCH (t:Taboo {key: 'ethnic-stereotypes@ig-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-mockery@ig-NG'})
SET t.display_name = 'Religious mockery',
    t.locale = 'ig-NG',
    t.term = 'Religious mockery',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious mockery in ig-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ig-NG'})
MATCH (t:Taboo {key: 'religious-mockery@ig-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ka-GE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ka-GE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ka-GE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'russian-occupation-of-territories@ka-GE'})
SET t.display_name = 'Russian occupation of territories',
    t.locale = 'ka-GE',
    t.term = 'Russian occupation of territories',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Russian occupation of territories in ka-GE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ka-GE'})
MATCH (t:Taboo {key: 'russian-occupation-of-territories@ka-GE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'pro-russian-political-messaging@ka-GE'})
SET t.display_name = 'Pro-Russian political messaging',
    t.locale = 'ka-GE',
    t.term = 'Pro-Russian political messaging',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Pro-Russian political messaging in ka-GE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ka-GE'})
MATCH (t:Taboo {key: 'pro-russian-political-messaging@ka-GE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'questioning-orthodox-christianity@ka-GE'})
SET t.display_name = 'Questioning Orthodox Christianity',
    t.locale = 'ka-GE',
    t.term = 'Questioning Orthodox Christianity',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Questioning Orthodox Christianity in ka-GE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ka-GE'})
MATCH (t:Taboo {key: 'questioning-orthodox-christianity@ka-GE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-BH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-BH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-BH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'sectarian-divisions-sunnishia@ar-BH'})
SET t.display_name = 'Sectarian divisions (Sunni/Shia)',
    t.locale = 'ar-BH',
    t.term = 'Sectarian divisions (Sunni/Shia)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Sectarian divisions (Sunni/Shia) in ar-BH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-BH'})
MATCH (t:Taboo {key: 'sectarian-divisions-sunnishia@ar-BH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '2011-protests-and-aftermath@ar-BH'})
SET t.display_name = '2011 protests and aftermath',
    t.locale = 'ar-BH',
    t.term = '2011 protests and aftermath',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 2011 protests and aftermath in ar-BH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-BH'})
MATCH (t:Taboo {key: '2011-protests-and-aftermath@ar-BH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-royal-family@ar-BH'})
SET t.display_name = 'Criticism of royal family',
    t.locale = 'ar-BH',
    t.term = 'Criticism of royal family',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of royal family in ar-BH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-BH'})
MATCH (t:Taboo {key: 'criticism-of-royal-family@ar-BH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-LU Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-LU'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-LU'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'cadrage-paradis-fiscal@fr-LU'})
SET t.display_name = 'Cadrage "paradis fiscal"',
    t.locale = 'fr-LU',
    t.term = 'Cadrage "paradis fiscal"',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Cadrage "paradis fiscal" in fr-LU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-LU'})
MATCH (t:Taboo {key: 'cadrage-paradis-fiscal@fr-LU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'confondre-luxembourg-avec-belgique-ou-allemagne@fr-LU'})
SET t.display_name = 'Confondre Luxembourg avec Belgique ou Allemagne',
    t.locale = 'fr-LU',
    t.term = 'Confondre Luxembourg avec Belgique ou Allemagne',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Confondre Luxembourg avec Belgique ou Allemagne in fr-LU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-LU'})
MATCH (t:Taboo {key: 'confondre-luxembourg-avec-belgique-ou-allemagne@fr-LU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'discuter-de-richesse-personnelle-ou-de-revenus@fr-LU'})
SET t.display_name = 'Discuter de richesse personnelle ou de revenus',
    t.locale = 'fr-LU',
    t.term = 'Discuter de richesse personnelle ou de revenus',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Discuter de richesse personnelle ou de revenus in fr-LU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-LU'})
MATCH (t:Taboo {key: 'discuter-de-richesse-personnelle-ou-de-revenus@fr-LU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-ZW Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-ZW'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZW'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'current-political-situation@en-ZW'})
SET t.display_name = 'Current political situation',
    t.locale = 'en-ZW',
    t.term = 'Current political situation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Current political situation in en-ZW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZW'})
MATCH (t:Taboo {key: 'current-political-situation@en-ZW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'economic-crisis-references@en-ZW'})
SET t.display_name = 'Economic crisis references',
    t.locale = 'en-ZW',
    t.term = 'Economic crisis references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Economic crisis references in en-ZW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZW'})
MATCH (t:Taboo {key: 'economic-crisis-references@en-ZW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'land-reform-farm-seizures@en-ZW'})
SET t.display_name = 'Land reform / farm seizures',
    t.locale = 'en-ZW',
    t.term = 'Land reform / farm seizures',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Land reform / farm seizures in en-ZW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZW'})
MATCH (t:Taboo {key: 'land-reform-farm-seizures@en-ZW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'religious-communalism@en-IN'})
SET t.display_name = 'Religious communalism',
    t.locale = 'en-IN',
    t.term = 'Religious communalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious communalism in en-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IN'})
MATCH (t:Taboo {key: 'religious-communalism@en-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'caste@en-IN'})
SET t.display_name = 'Caste',
    t.locale = 'en-IN',
    t.term = 'Caste',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Caste in en-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IN'})
MATCH (t:Taboo {key: 'caste@en-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'kashmir-and-territorial-disputes@en-IN'})
SET t.display_name = 'Kashmir and territorial disputes',
    t.locale = 'en-IN',
    t.term = 'Kashmir and territorial disputes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Kashmir and territorial disputes in en-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IN'})
MATCH (t:Taboo {key: 'kashmir-and-territorial-disputes@en-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// zh-TW Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-TW'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TW'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'taiwan-sovereignty-status@zh-TW'})
SET t.display_name = '台灣主權地位 (Taiwan sovereignty status)',
    t.locale = 'zh-TW',
    t.term = '台灣主權地位 (Taiwan sovereignty status)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 台灣主權地位 (Taiwan sovereignty status) in zh-TW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TW'})
MATCH (t:Taboo {key: 'taiwan-sovereignty-status@zh-TW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'unification-independence-debate@zh-TW'})
SET t.display_name = '統獨議題 (Unification-independence debate)',
    t.locale = 'zh-TW',
    t.term = '統獨議題 (Unification-independence debate)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 統獨議題 (Unification-independence debate) in zh-TW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TW'})
MATCH (t:Taboo {key: 'unification-independence-debate@zh-TW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'cross-strait-relations@zh-TW'})
SET t.display_name = '兩岸關係 (Cross-strait relations)',
    t.locale = 'zh-TW',
    t.term = '兩岸關係 (Cross-strait relations)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 兩岸關係 (Cross-strait relations) in zh-TW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TW'})
MATCH (t:Taboo {key: 'cross-strait-relations@zh-TW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// de-CH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'de-CH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-CH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'comparing-swiss-to-germans-unfavorably@de-CH'})
SET t.display_name = 'Comparing Swiss to Germans unfavorably',
    t.locale = 'de-CH',
    t.term = 'Comparing Swiss to Germans unfavorably',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparing Swiss to Germans unfavorably in de-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-CH'})
MATCH (t:Taboo {key: 'comparing-swiss-to-germans-unfavorably@de-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'language-region-favoritism@de-CH'})
SET t.display_name = 'Language region favoritism',
    t.locale = 'de-CH',
    t.term = 'Language region favoritism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Language region favoritism in de-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-CH'})
MATCH (t:Taboo {key: 'language-region-favoritism@de-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'discussing-personal-finances-or-income@de-CH'})
SET t.display_name = 'Discussing personal finances or income',
    t.locale = 'de-CH',
    t.term = 'Discussing personal finances or income',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Discussing personal finances or income in de-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-CH'})
MATCH (t:Taboo {key: 'discussing-personal-finances-or-income@de-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// pt-BR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-BR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-BR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'racism-and-racial-inequality@pt-BR'})
SET t.display_name = 'Racism and racial inequality',
    t.locale = 'pt-BR',
    t.term = 'Racism and racial inequality',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racism and racial inequality in pt-BR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-BR'})
MATCH (t:Taboo {key: 'racism-and-racial-inequality@pt-BR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'class-discrimination@pt-BR'})
SET t.display_name = 'Class discrimination',
    t.locale = 'pt-BR',
    t.term = 'Class discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Class discrimination in pt-BR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-BR'})
MATCH (t:Taboo {key: 'class-discrimination@pt-BR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-polarization@pt-BR'})
SET t.display_name = 'Political polarization',
    t.locale = 'pt-BR',
    t.term = 'Political polarization',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political polarization in pt-BR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-BR'})
MATCH (t:Taboo {key: 'political-polarization@pt-BR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fa-IR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fa-IR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fa-IR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-islam-or-islamic-values@fa-IR'})
SET t.display_name = 'Criticism of Islam or Islamic values',
    t.locale = 'fa-IR',
    t.term = 'Criticism of Islam or Islamic values',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Islam or Islamic values in fa-IR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fa-IR'})
MATCH (t:Taboo {key: 'criticism-of-islam-or-islamic-values@fa-IR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-supreme-leader-or-political-system@fa-IR'})
SET t.display_name = 'Criticism of Supreme Leader or political system',
    t.locale = 'fa-IR',
    t.term = 'Criticism of Supreme Leader or political system',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Supreme Leader or political system in fa-IR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fa-IR'})
MATCH (t:Taboo {key: 'criticism-of-supreme-leader-or-political-system@fa-IR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'pre-marital-relationships-or-sexuality@fa-IR'})
SET t.display_name = 'Pre-marital relationships or sexuality',
    t.locale = 'fa-IR',
    t.term = 'Pre-marital relationships or sexuality',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Pre-marital relationships or sexuality in fa-IR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fa-IR'})
MATCH (t:Taboo {key: 'pre-marital-relationships-or-sexuality@fa-IR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// de-AT Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'de-AT'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-AT'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'calling-austrians-german@de-AT'})
SET t.display_name = 'Calling Austrians "German"',
    t.locale = 'de-AT',
    t.term = 'Calling Austrians "German"',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Calling Austrians "German" in de-AT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-AT'})
MATCH (t:Taboo {key: 'calling-austrians-german@de-AT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'nazi-periodholocaust@de-AT'})
SET t.display_name = 'Nazi period/Holocaust',
    t.locale = 'de-AT',
    t.term = 'Nazi period/Holocaust',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Nazi period/Holocaust in de-AT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-AT'})
MATCH (t:Taboo {key: 'nazi-periodholocaust@de-AT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'nazi-imagery-or-symbols@de-AT'})
SET t.display_name = 'Nazi imagery or symbols',
    t.locale = 'de-AT',
    t.term = 'Nazi imagery or symbols',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Nazi imagery or symbols in de-AT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-AT'})
MATCH (t:Taboo {key: 'nazi-imagery-or-symbols@de-AT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-TN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-TN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-TN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulter-l@fr-TN'})
SET t.display_name = 'Insulter l\\',
    t.locale = 'fr-TN',
    t.term = 'Insulter l\\',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulter l\\ in fr-TN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-TN'})
MATCH (t:Taboo {key: 'insulter-l@fr-TN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-TT Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-TT'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-TT'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'mocking-trinidadian-dialect@en-TT'})
SET t.display_name = 'Mocking Trinidadian dialect',
    t.locale = 'en-TT',
    t.term = 'Mocking Trinidadian dialect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mocking Trinidadian dialect in en-TT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-TT'})
MATCH (t:Taboo {key: 'mocking-trinidadian-dialect@en-TT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-stereotyping@en-TT'})
SET t.display_name = 'Ethnic stereotyping',
    t.locale = 'en-TT',
    t.term = 'Ethnic stereotyping',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic stereotyping in en-TT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-TT'})
MATCH (t:Taboo {key: 'ethnic-stereotyping@en-TT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'slavery-glorification@en-TT'})
SET t.display_name = 'Slavery glorification',
    t.locale = 'en-TT',
    t.term = 'Slavery glorification',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Slavery glorification in en-TT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-TT'})
MATCH (t:Taboo {key: 'slavery-glorification@en-TT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// is-IS Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'is-IS'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@is-IS'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'whaling-controversy@is-IS'})
SET t.display_name = 'Whaling controversy',
    t.locale = 'is-IS',
    t.term = 'Whaling controversy',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Whaling controversy in is-IS content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@is-IS'})
MATCH (t:Taboo {key: 'whaling-controversy@is-IS'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'whaling-policy@is-IS'})
SET t.display_name = 'Whaling policy',
    t.locale = 'is-IS',
    t.term = 'Whaling policy',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Whaling policy in is-IS content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@is-IS'})
MATCH (t:Taboo {key: 'whaling-policy@is-IS'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ca-ES Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ca-ES'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ca-ES'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'independence-politics@ca-ES'})
SET t.display_name = 'Independence politics',
    t.locale = 'ca-ES',
    t.term = 'Independence politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Independence politics in ca-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ca-ES'})
MATCH (t:Taboo {key: 'independence-politics@ca-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'spanish-vs-catalan-framing@ca-ES'})
SET t.display_name = 'Spanish vs Catalan framing',
    t.locale = 'ca-ES',
    t.term = 'Spanish vs Catalan framing',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Spanish vs Catalan framing in ca-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ca-ES'})
MATCH (t:Taboo {key: 'spanish-vs-catalan-framing@ca-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '2017-events@ca-ES'})
SET t.display_name = '2017 events',
    t.locale = 'ca-ES',
    t.term = '2017 events',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 2017 events in ca-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ca-ES'})
MATCH (t:Taboo {key: '2017-events@ca-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ru-KZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-KZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-KZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-government-or-president@ru-KZ'})
SET t.display_name = 'Criticism of government or president',
    t.locale = 'ru-KZ',
    t.term = 'Criticism of government or president',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of government or president in ru-KZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-KZ'})
MATCH (t:Taboo {key: 'criticism-of-government-or-president@ru-KZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'inter-ethnic-tensions-or-divisions@ru-KZ'})
SET t.display_name = 'Inter-ethnic tensions or divisions',
    t.locale = 'ru-KZ',
    t.term = 'Inter-ethnic tensions or divisions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Inter-ethnic tensions or divisions in ru-KZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-KZ'})
MATCH (t:Taboo {key: 'inter-ethnic-tensions-or-divisions@ru-KZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'territorial-integrity-questions@ru-KZ'})
SET t.display_name = 'Territorial integrity questions',
    t.locale = 'ru-KZ',
    t.term = 'Territorial integrity questions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Territorial integrity questions in ru-KZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-KZ'})
MATCH (t:Taboo {key: 'territorial-integrity-questions@ru-KZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// kn-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'kn-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@kn-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'kannada-language-disrespect@kn-IN'})
SET t.display_name = 'Kannada language disrespect',
    t.locale = 'kn-IN',
    t.term = 'Kannada language disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Kannada language disrespect in kn-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@kn-IN'})
MATCH (t:Taboo {key: 'kannada-language-disrespect@kn-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-communalism@kn-IN'})
SET t.display_name = 'Religious communalism',
    t.locale = 'kn-IN',
    t.term = 'Religious communalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious communalism in kn-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@kn-IN'})
MATCH (t:Taboo {key: 'religious-communalism@kn-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'caste-discrimination@kn-IN'})
SET t.display_name = 'Caste discrimination',
    t.locale = 'kn-IN',
    t.term = 'Caste discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Caste discrimination in kn-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@kn-IN'})
MATCH (t:Taboo {key: 'caste-discrimination@kn-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ro-RO Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ro-RO'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ro-RO'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ceauescu-era-communist-dictatorship@ro-RO'})
SET t.display_name = 'Ceaușescu era / communist dictatorship',
    t.locale = 'ro-RO',
    t.term = 'Ceaușescu era / communist dictatorship',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ceaușescu era / communist dictatorship in ro-RO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ro-RO'})
MATCH (t:Taboo {key: 'ceauescu-era-communist-dictatorship@ro-RO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'roma-stereotypes@ro-RO'})
SET t.display_name = 'Roma stereotypes',
    t.locale = 'ro-RO',
    t.term = 'Roma stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Roma stereotypes in ro-RO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ro-RO'})
MATCH (t:Taboo {key: 'roma-stereotypes@ro-RO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'hungarian-romanian-tensions@ro-RO'})
SET t.display_name = 'Hungarian-Romanian tensions',
    t.locale = 'ro-RO',
    t.term = 'Hungarian-Romanian tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Hungarian-Romanian tensions in ro-RO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ro-RO'})
MATCH (t:Taboo {key: 'hungarian-romanian-tensions@ro-RO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// gu-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'gu-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@gu-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'beefmeat-content@gu-IN'})
SET t.display_name = 'Beef/meat content',
    t.locale = 'gu-IN',
    t.term = 'Beef/meat content',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Beef/meat content in gu-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@gu-IN'})
MATCH (t:Taboo {key: 'beefmeat-content@gu-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'alcohol-promotion@gu-IN'})
SET t.display_name = 'Alcohol promotion',
    t.locale = 'gu-IN',
    t.term = 'Alcohol promotion',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Alcohol promotion in gu-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@gu-IN'})
MATCH (t:Taboo {key: 'alcohol-promotion@gu-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-communalism@gu-IN'})
SET t.display_name = 'Religious communalism',
    t.locale = 'gu-IN',
    t.term = 'Religious communalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious communalism in gu-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@gu-IN'})
MATCH (t:Taboo {key: 'religious-communalism@gu-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// pt-AO Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-AO'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-AO'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'guerra-civil-1975-2002@pt-AO'})
SET t.display_name = 'Guerra Civil (1975-2002)',
    t.locale = 'pt-AO',
    t.term = 'Guerra Civil (1975-2002)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Guerra Civil (1975-2002) in pt-AO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-AO'})
MATCH (t:Taboo {key: 'guerra-civil-1975-2002@pt-AO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'unita-vs-mpla-historico@pt-AO'})
SET t.display_name = 'UNITA vs MPLA historico',
    t.locale = 'pt-AO',
    t.term = 'UNITA vs MPLA historico',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: UNITA vs MPLA historico in pt-AO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-AO'})
MATCH (t:Taboo {key: 'unita-vs-mpla-historico@pt-AO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'jonas-savimbi@pt-AO'})
SET t.display_name = 'Jonas Savimbi',
    t.locale = 'pt-AO',
    t.term = 'Jonas Savimbi',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Jonas Savimbi in pt-AO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-AO'})
MATCH (t:Taboo {key: 'jonas-savimbi@pt-AO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ny-MW Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ny-MW'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ny-MW'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'witchcraft-ufiti@ny-MW'})
SET t.display_name = 'Witchcraft (ufiti)',
    t.locale = 'ny-MW',
    t.term = 'Witchcraft (ufiti)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Witchcraft (ufiti) in ny-MW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ny-MW'})
MATCH (t:Taboo {key: 'witchcraft-ufiti@ny-MW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'sexual-content@ny-MW'})
SET t.display_name = 'Sexual content',
    t.locale = 'ny-MW',
    t.term = 'Sexual content',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Sexual content in ny-MW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ny-MW'})
MATCH (t:Taboo {key: 'sexual-content@ny-MW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'disrespecting-elders@ny-MW'})
SET t.display_name = 'Disrespecting elders',
    t.locale = 'ny-MW',
    t.term = 'Disrespecting elders',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting elders in ny-MW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ny-MW'})
MATCH (t:Taboo {key: 'disrespecting-elders@ny-MW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// or-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'or-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@or-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'disrespecting-religious-sentiments@or-IN'})
SET t.display_name = 'Disrespecting religious sentiments',
    t.locale = 'or-IN',
    t.term = 'Disrespecting religious sentiments',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting religious sentiments in or-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@or-IN'})
MATCH (t:Taboo {key: 'disrespecting-religious-sentiments@or-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insults-to-jagannath-temple@or-IN'})
SET t.display_name = 'Insults to Jagannath temple',
    t.locale = 'or-IN',
    t.term = 'Insults to Jagannath temple',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insults to Jagannath temple in or-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@or-IN'})
MATCH (t:Taboo {key: 'insults-to-jagannath-temple@or-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// zh-CN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-CN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-CN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'taiwan@zh-CN'})
SET t.display_name = 'Taiwan',
    t.locale = 'zh-CN',
    t.term = 'Taiwan',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Taiwan in zh-CN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-CN'})
MATCH (t:Taboo {key: 'taiwan@zh-CN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'tibet@zh-CN'})
SET t.display_name = 'Tibet',
    t.locale = 'zh-CN',
    t.term = 'Tibet',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Tibet in zh-CN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-CN'})
MATCH (t:Taboo {key: 'tibet@zh-CN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'xinjiang@zh-CN'})
SET t.display_name = 'Xinjiang',
    t.locale = 'zh-CN',
    t.term = 'Xinjiang',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Xinjiang in zh-CN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-CN'})
MATCH (t:Taboo {key: 'xinjiang@zh-CN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'tiananmen-square-1989@zh-CN'})
SET t.display_name = 'Tiananmen Square (1989)',
    t.locale = 'zh-CN',
    t.term = 'Tiananmen Square (1989)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Tiananmen Square (1989) in zh-CN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-CN'})
MATCH (t:Taboo {key: 'tiananmen-square-1989@zh-CN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-criticism@zh-CN'})
SET t.display_name = 'Political criticism',
    t.locale = 'zh-CN',
    t.term = 'Political criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political criticism in zh-CN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-CN'})
MATCH (t:Taboo {key: 'political-criticism@zh-CN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// sn-ZW Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sn-ZW'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sn-ZW'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'current-political-situation@sn-ZW'})
SET t.display_name = 'Current political situation',
    t.locale = 'sn-ZW',
    t.term = 'Current political situation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Current political situation in sn-ZW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sn-ZW'})
MATCH (t:Taboo {key: 'current-political-situation@sn-ZW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'economic-crisis-references@sn-ZW'})
SET t.display_name = 'Economic crisis references',
    t.locale = 'sn-ZW',
    t.term = 'Economic crisis references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Economic crisis references in sn-ZW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sn-ZW'})
MATCH (t:Taboo {key: 'economic-crisis-references@sn-ZW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'land-reform-farm-seizures@sn-ZW'})
SET t.display_name = 'Land reform / farm seizures',
    t.locale = 'sn-ZW',
    t.term = 'Land reform / farm seizures',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Land reform / farm seizures in sn-ZW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sn-ZW'})
MATCH (t:Taboo {key: 'land-reform-farm-seizures@sn-ZW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-JO Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-JO'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-JO'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-the-monarchy@ar-JO'})
SET t.display_name = 'Criticism of the monarchy',
    t.locale = 'ar-JO',
    t.term = 'Criticism of the monarchy',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of the monarchy in ar-JO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-JO'})
MATCH (t:Taboo {key: 'criticism-of-the-monarchy@ar-JO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insulting-islam@ar-JO'})
SET t.display_name = 'Insulting Islam',
    t.locale = 'ar-JO',
    t.term = 'Insulting Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam in ar-JO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-JO'})
MATCH (t:Taboo {key: 'insulting-islam@ar-JO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'israeli-palestinian-conflict-taking-israeli-side@ar-JO'})
SET t.display_name = 'Israeli-Palestinian conflict (taking Israeli side)',
    t.locale = 'ar-JO',
    t.term = 'Israeli-Palestinian conflict (taking Israeli side)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Israeli-Palestinian conflict (taking Israeli side) in ar-JO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-JO'})
MATCH (t:Taboo {key: 'israeli-palestinian-conflict-taking-israeli-side@ar-JO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-KW Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-KW'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-KW'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-prophet@ar-KW'})
SET t.display_name = 'Insulting Islam or Prophet',
    t.locale = 'ar-KW',
    t.term = 'Insulting Islam or Prophet',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or Prophet in ar-KW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-KW'})
MATCH (t:Taboo {key: 'insulting-islam-or-prophet@ar-KW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-emir-or-ruling-family@ar-KW'})
SET t.display_name = 'Criticism of Emir or ruling family',
    t.locale = 'ar-KW',
    t.term = 'Criticism of Emir or ruling family',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Emir or ruling family in ar-KW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-KW'})
MATCH (t:Taboo {key: 'criticism-of-emir-or-ruling-family@ar-KW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-criticism@ar-KW'})
SET t.display_name = 'Political criticism',
    t.locale = 'ar-KW',
    t.term = 'Political criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political criticism in ar-KW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-KW'})
MATCH (t:Taboo {key: 'political-criticism@ar-KW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-AU Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-AU'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AU'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'indigenous-mistreatmentappropriation@en-AU'})
SET t.display_name = 'Indigenous mistreatment/appropriation',
    t.locale = 'en-AU',
    t.term = 'Indigenous mistreatment/appropriation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Indigenous mistreatment/appropriation in en-AU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AU'})
MATCH (t:Taboo {key: 'indigenous-mistreatmentappropriation@en-AU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'racism-and-discrimination@en-AU'})
SET t.display_name = 'Racism and discrimination',
    t.locale = 'en-AU',
    t.term = 'Racism and discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racism and discrimination in en-AU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AU'})
MATCH (t:Taboo {key: 'racism-and-discrimination@en-AU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'stolen-generations-indigenous-treatment@en-AU'})
SET t.display_name = 'Stolen Generations / Indigenous treatment',
    t.locale = 'en-AU',
    t.term = 'Stolen Generations / Indigenous treatment',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Stolen Generations / Indigenous treatment in en-AU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AU'})
MATCH (t:Taboo {key: 'stolen-generations-indigenous-treatment@en-AU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-FR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-FR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-FR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'personal-incomewealth@fr-FR'})
SET t.display_name = 'Personal income/wealth',
    t.locale = 'fr-FR',
    t.term = 'Personal income/wealth',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Personal income/wealth in fr-FR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-FR'})
MATCH (t:Taboo {key: 'personal-incomewealth@fr-FR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-affiliation@fr-FR'})
SET t.display_name = 'Religious affiliation',
    t.locale = 'fr-FR',
    t.term = 'Religious affiliation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious affiliation in fr-FR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-FR'})
MATCH (t:Taboo {key: 'religious-affiliation@fr-FR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-extremism@fr-FR'})
SET t.display_name = 'Political extremism',
    t.locale = 'fr-FR',
    t.term = 'Political extremism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political extremism in fr-FR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-FR'})
MATCH (t:Taboo {key: 'political-extremism@fr-FR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-SN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-SN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-SN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'irrespect-envers-les-autorites-religieuses@fr-SN'})
SET t.display_name = 'Irrespect envers les autorites religieuses',
    t.locale = 'fr-SN',
    t.term = 'Irrespect envers les autorites religieuses',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Irrespect envers les autorites religieuses in fr-SN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-SN'})
MATCH (t:Taboo {key: 'irrespect-envers-les-autorites-religieuses@fr-SN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insultes-aux-parents-ou-ancetres@fr-SN'})
SET t.display_name = 'Insultes aux parents ou ancetres',
    t.locale = 'fr-SN',
    t.term = 'Insultes aux parents ou ancetres',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insultes aux parents ou ancetres in fr-SN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-SN'})
MATCH (t:Taboo {key: 'insultes-aux-parents-ou-ancetres@fr-SN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'irrespect-envers-l@fr-SN'})
SET t.display_name = 'Irrespect envers l\\',
    t.locale = 'fr-SN',
    t.term = 'Irrespect envers l\\',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Irrespect envers l\\ in fr-SN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-SN'})
MATCH (t:Taboo {key: 'irrespect-envers-l@fr-SN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// pa-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pa-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pa-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'disrespect-to-sikh-faithgurus@pa-IN'})
SET t.display_name = 'Disrespect to Sikh faith/Gurus',
    t.locale = 'pa-IN',
    t.term = 'Disrespect to Sikh faith/Gurus',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespect to Sikh faith/Gurus in pa-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pa-IN'})
MATCH (t:Taboo {key: 'disrespect-to-sikh-faithgurus@pa-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '1984-references@pa-IN'})
SET t.display_name = '1984 references',
    t.locale = 'pa-IN',
    t.term = '1984 references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 1984 references in pa-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pa-IN'})
MATCH (t:Taboo {key: '1984-references@pa-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'caste-system-mentions@pa-IN'})
SET t.display_name = 'Caste system mentions',
    t.locale = 'pa-IN',
    t.term = 'Caste system mentions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Caste system mentions in pa-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pa-IN'})
MATCH (t:Taboo {key: 'caste-system-mentions@pa-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// cs-CZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'cs-CZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@cs-CZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'communist-era-nostalgia@cs-CZ'})
SET t.display_name = 'Communist era nostalgia',
    t.locale = 'cs-CZ',
    t.term = 'Communist era nostalgia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Communist era nostalgia in cs-CZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@cs-CZ'})
MATCH (t:Taboo {key: 'communist-era-nostalgia@cs-CZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'nazi-occupation@cs-CZ'})
SET t.display_name = 'Nazi occupation',
    t.locale = 'cs-CZ',
    t.term = 'Nazi occupation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Nazi occupation in cs-CZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@cs-CZ'})
MATCH (t:Taboo {key: 'nazi-occupation@cs-CZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-CL Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-CL'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CL'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'pinochet-dictatorship-1973-1990@es-CL'})
SET t.display_name = 'Pinochet dictatorship (1973-1990)',
    t.locale = 'es-CL',
    t.term = 'Pinochet dictatorship (1973-1990)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Pinochet dictatorship (1973-1990) in es-CL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CL'})
MATCH (t:Taboo {key: 'pinochet-dictatorship-1973-1990@es-CL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'allende-government-and-1973-coup@es-CL'})
SET t.display_name = 'Allende government and 1973 coup',
    t.locale = 'es-CL',
    t.term = 'Allende government and 1973 coup',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Allende government and 1973 coup in es-CL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CL'})
MATCH (t:Taboo {key: 'allende-government-and-1973-coup@es-CL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '2019-estallido-social@es-CL'})
SET t.display_name = '2019 Estallido Social',
    t.locale = 'es-CL',
    t.term = '2019 Estallido Social',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 2019 Estallido Social in es-CL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CL'})
MATCH (t:Taboo {key: '2019-estallido-social@es-CL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// nl-NL Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'nl-NL'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-NL'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'colonialism-indonesia@nl-NL'})
SET t.display_name = 'Colonialism (Indonesia',
    t.locale = 'nl-NL',
    t.term = 'Colonialism (Indonesia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Colonialism (Indonesia in nl-NL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-NL'})
MATCH (t:Taboo {key: 'colonialism-indonesia@nl-NL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'suriname@nl-NL'})
SET t.display_name = 'Suriname',
    t.locale = 'nl-NL',
    t.term = 'Suriname',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Suriname in nl-NL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-NL'})
MATCH (t:Taboo {key: 'suriname@nl-NL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'slavery@nl-NL'})
SET t.display_name = 'slavery)',
    t.locale = 'nl-NL',
    t.term = 'slavery)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: slavery) in nl-NL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-NL'})
MATCH (t:Taboo {key: 'slavery@nl-NL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'zwarte-piet-controversy@nl-NL'})
SET t.display_name = 'Zwarte Piet controversy',
    t.locale = 'nl-NL',
    t.term = 'Zwarte Piet controversy',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Zwarte Piet controversy in nl-NL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-NL'})
MATCH (t:Taboo {key: 'zwarte-piet-controversy@nl-NL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'immigration-politics@nl-NL'})
SET t.display_name = 'Immigration politics',
    t.locale = 'nl-NL',
    t.term = 'Immigration politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Immigration politics in nl-NL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-NL'})
MATCH (t:Taboo {key: 'immigration-politics@nl-NL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-HN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-HN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-HN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'gang-violence-maras@es-HN'})
SET t.display_name = 'Gang violence (maras)',
    t.locale = 'es-HN',
    t.term = 'Gang violence (maras)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Gang violence (maras) in es-HN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-HN'})
MATCH (t:Taboo {key: 'gang-violence-maras@es-HN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-corruption@es-HN'})
SET t.display_name = 'Political corruption',
    t.locale = 'es-HN',
    t.term = 'Political corruption',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political corruption in es-HN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-HN'})
MATCH (t:Taboo {key: 'political-corruption@es-HN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// bg-BG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'bg-BG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@bg-BG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ottoman-empire-period@bg-BG'})
SET t.display_name = 'Ottoman Empire period',
    t.locale = 'bg-BG',
    t.term = 'Ottoman Empire period',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ottoman Empire period in bg-BG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bg-BG'})
MATCH (t:Taboo {key: 'ottoman-empire-period@bg-BG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'communist-era-nostalgiacriticism@bg-BG'})
SET t.display_name = 'Communist era nostalgia/criticism',
    t.locale = 'bg-BG',
    t.term = 'Communist era nostalgia/criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Communist era nostalgia/criticism in bg-BG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bg-BG'})
MATCH (t:Taboo {key: 'communist-era-nostalgiacriticism@bg-BG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// sd-PK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sd-PK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sd-PK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'blasphemy-against-islam@sd-PK'})
SET t.display_name = 'Blasphemy against Islam',
    t.locale = 'sd-PK',
    t.term = 'Blasphemy against Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Blasphemy against Islam in sd-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sd-PK'})
MATCH (t:Taboo {key: 'blasphemy-against-islam@sd-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'sectarian-divisions@sd-PK'})
SET t.display_name = 'Sectarian divisions',
    t.locale = 'sd-PK',
    t.term = 'Sectarian divisions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Sectarian divisions in sd-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sd-PK'})
MATCH (t:Taboo {key: 'sectarian-divisions@sd-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ahmadiyya-references@sd-PK'})
SET t.display_name = 'Ahmadiyya references',
    t.locale = 'sd-PK',
    t.term = 'Ahmadiyya references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ahmadiyya references in sd-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sd-PK'})
MATCH (t:Taboo {key: 'ahmadiyya-references@sd-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-GH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-GH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-GH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-stereotyping@en-GH'})
SET t.display_name = 'Ethnic stereotyping',
    t.locale = 'en-GH',
    t.term = 'Ethnic stereotyping',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic stereotyping in en-GH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-GH'})
MATCH (t:Taboo {key: 'ethnic-stereotyping@en-GH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'chieftaincy-disputes@en-GH'})
SET t.display_name = 'Chieftaincy disputes',
    t.locale = 'en-GH',
    t.term = 'Chieftaincy disputes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Chieftaincy disputes in en-GH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-GH'})
MATCH (t:Taboo {key: 'chieftaincy-disputes@en-GH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'witchcraft-allegations@en-GH'})
SET t.display_name = 'Witchcraft allegations',
    t.locale = 'en-GH',
    t.term = 'Witchcraft allegations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Witchcraft allegations in en-GH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-GH'})
MATCH (t:Taboo {key: 'witchcraft-allegations@en-GH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-PH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-PH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'disrespecting-religious-symbols-or-beliefs@en-PH'})
SET t.display_name = 'Disrespecting religious symbols or beliefs',
    t.locale = 'en-PH',
    t.term = 'Disrespecting religious symbols or beliefs',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting religious symbols or beliefs in en-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PH'})
MATCH (t:Taboo {key: 'disrespecting-religious-symbols-or-beliefs@en-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'causing-public-embarrassment-hiya@en-PH'})
SET t.display_name = 'Causing public embarrassment (hiya)',
    t.locale = 'en-PH',
    t.term = 'Causing public embarrassment (hiya)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Causing public embarrassment (hiya) in en-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PH'})
MATCH (t:Taboo {key: 'causing-public-embarrassment-hiya@en-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'mocking-or-disrespecting-elders@en-PH'})
SET t.display_name = 'Mocking or disrespecting elders',
    t.locale = 'en-PH',
    t.term = 'Mocking or disrespecting elders',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mocking or disrespecting elders in en-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PH'})
MATCH (t:Taboo {key: 'mocking-or-disrespecting-elders@en-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// sl-SI Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sl-SI'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sl-SI'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'yugoslav-period-nostalgia-or-criticism@sl-SI'})
SET t.display_name = 'Yugoslav period nostalgia or criticism',
    t.locale = 'sl-SI',
    t.term = 'Yugoslav period nostalgia or criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Yugoslav period nostalgia or criticism in sl-SI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sl-SI'})
MATCH (t:Taboo {key: 'yugoslav-period-nostalgia-or-criticism@sl-SI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-comparisons-with-neighbors@sl-SI'})
SET t.display_name = 'Ethnic comparisons with neighbors',
    t.locale = 'sl-SI',
    t.term = 'Ethnic comparisons with neighbors',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic comparisons with neighbors in sl-SI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sl-SI'})
MATCH (t:Taboo {key: 'ethnic-comparisons-with-neighbors@sl-SI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-HK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-HK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-HK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'hong-kong-china-political-relations@en-HK'})
SET t.display_name = 'Hong Kong-China political relations',
    t.locale = 'en-HK',
    t.term = 'Hong Kong-China political relations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Hong Kong-China political relations in en-HK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-HK'})
MATCH (t:Taboo {key: 'hong-kong-china-political-relations@en-HK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '2019-2020-protests@en-HK'})
SET t.display_name = '2019-2020 protests',
    t.locale = 'en-HK',
    t.term = '2019-2020 protests',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 2019-2020 protests in en-HK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-HK'})
MATCH (t:Taboo {key: '2019-2020-protests@en-HK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taiwan-independence@en-HK'})
SET t.display_name = 'Taiwan independence',
    t.locale = 'en-HK',
    t.term = 'Taiwan independence',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Taiwan independence in en-HK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-HK'})
MATCH (t:Taboo {key: 'taiwan-independence@en-HK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-MU Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-MU'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-MU'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-divisions-or-favouritism@en-MU'})
SET t.display_name = 'Ethnic divisions or favouritism',
    t.locale = 'en-MU',
    t.term = 'Ethnic divisions or favouritism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic divisions or favouritism in en-MU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-MU'})
MATCH (t:Taboo {key: 'ethnic-divisions-or-favouritism@en-MU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'communalism@en-MU'})
SET t.display_name = 'Communalism',
    t.locale = 'en-MU',
    t.term = 'Communalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Communalism in en-MU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-MU'})
MATCH (t:Taboo {key: 'communalism@en-MU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// et-EE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'et-EE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@et-EE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'soviet-nostalgia-or-symbolism@et-EE'})
SET t.display_name = 'Soviet nostalgia or symbolism',
    t.locale = 'et-EE',
    t.term = 'Soviet nostalgia or symbolism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Soviet nostalgia or symbolism in et-EE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@et-EE'})
MATCH (t:Taboo {key: 'soviet-nostalgia-or-symbolism@et-EE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'comparing-to-russia-positively@et-EE'})
SET t.display_name = 'Comparing to Russia positively',
    t.locale = 'et-EE',
    t.term = 'Comparing to Russia positively',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparing to Russia positively in et-EE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@et-EE'})
MATCH (t:Taboo {key: 'comparing-to-russia-positively@et-EE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'soviet-occupation-1940-1991@et-EE'})
SET t.display_name = 'Soviet Occupation (1940-1991)',
    t.locale = 'et-EE',
    t.term = 'Soviet Occupation (1940-1991)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Soviet Occupation (1940-1991) in et-EE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@et-EE'})
MATCH (t:Taboo {key: 'soviet-occupation-1940-1991@et-EE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-SV Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-SV'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-SV'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'civil-war-1979-1992@es-SV'})
SET t.display_name = 'Civil War (1979-1992)',
    t.locale = 'es-SV',
    t.term = 'Civil War (1979-1992)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Civil War (1979-1992) in es-SV content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-SV'})
MATCH (t:Taboo {key: 'civil-war-1979-1992@es-SV'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'el-mozote-massacre@es-SV'})
SET t.display_name = 'El Mozote massacre',
    t.locale = 'es-SV',
    t.term = 'El Mozote massacre',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: El Mozote massacre in es-SV content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-SV'})
MATCH (t:Taboo {key: 'el-mozote-massacre@es-SV'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'gang-violence-maras@es-SV'})
SET t.display_name = 'Gang violence (maras)',
    t.locale = 'es-SV',
    t.term = 'Gang violence (maras)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Gang violence (maras) in es-SV content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-SV'})
MATCH (t:Taboo {key: 'gang-violence-maras@es-SV'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// pt-CH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-CH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-CH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'immigration-status-assumptions@pt-CH'})
SET t.display_name = 'Immigration status assumptions',
    t.locale = 'pt-CH',
    t.term = 'Immigration status assumptions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Immigration status assumptions in pt-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-CH'})
MATCH (t:Taboo {key: 'immigration-status-assumptions@pt-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'comparing-portuguese-to-spanish@pt-CH'})
SET t.display_name = 'Comparing Portuguese to Spanish',
    t.locale = 'pt-CH',
    t.term = 'Comparing Portuguese to Spanish',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparing Portuguese to Spanish in pt-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-CH'})
MATCH (t:Taboo {key: 'comparing-portuguese-to-spanish@pt-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'discussing-personal-finances-or-income@pt-CH'})
SET t.display_name = 'Discussing personal finances or income',
    t.locale = 'pt-CH',
    t.term = 'Discussing personal finances or income',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Discussing personal finances or income in pt-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-CH'})
MATCH (t:Taboo {key: 'discussing-personal-finances-or-income@pt-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-PR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-PR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'political-status-debate@es-PR'})
SET t.display_name = 'Political status debate',
    t.locale = 'es-PR',
    t.term = 'Political status debate',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political status debate in es-PR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PR'})
MATCH (t:Taboo {key: 'political-status-debate@es-PR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'hurricane-maria-mismanagement@es-PR'})
SET t.display_name = 'Hurricane Maria mismanagement',
    t.locale = 'es-PR',
    t.term = 'Hurricane Maria mismanagement',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Hurricane Maria mismanagement in es-PR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PR'})
MATCH (t:Taboo {key: 'hurricane-maria-mismanagement@es-PR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'hurricane-maria-2017@es-PR'})
SET t.display_name = 'Hurricane Maria (2017)',
    t.locale = 'es-PR',
    t.term = 'Hurricane Maria (2017)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Hurricane Maria (2017) in es-PR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PR'})
MATCH (t:Taboo {key: 'hurricane-maria-2017@es-PR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ur-PK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ur-PK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ur-PK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'blasphemy@ur-PK'})
SET t.display_name = 'توہین مذہب (Blasphemy)',
    t.locale = 'ur-PK',
    t.term = 'توہین مذہب (Blasphemy)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: توہین مذہب (Blasphemy) in ur-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ur-PK'})
MATCH (t:Taboo {key: 'blasphemy@ur-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-armed-forces@ur-PK'})
SET t.display_name = 'Criticism of Armed Forces',
    t.locale = 'ur-PK',
    t.term = 'Criticism of Armed Forces',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Armed Forces in ur-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ur-PK'})
MATCH (t:Taboo {key: 'criticism-of-armed-forces@ur-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ahmadiyya-references@ur-PK'})
SET t.display_name = 'Ahmadiyya references',
    t.locale = 'ur-PK',
    t.term = 'Ahmadiyya references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ahmadiyya references in ur-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ur-PK'})
MATCH (t:Taboo {key: 'ahmadiyya-references@ur-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-ZM Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-ZM'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZM'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'tribal-divisionspreferences@en-ZM'})
SET t.display_name = 'Tribal divisions/preferences',
    t.locale = 'en-ZM',
    t.term = 'Tribal divisions/preferences',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Tribal divisions/preferences in en-ZM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZM'})
MATCH (t:Taboo {key: 'tribal-divisionspreferences@en-ZM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'disrespecting-elders@en-ZM'})
SET t.display_name = 'Disrespecting elders',
    t.locale = 'en-ZM',
    t.term = 'Disrespecting elders',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting elders in en-ZM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZM'})
MATCH (t:Taboo {key: 'disrespecting-elders@en-ZM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticizing-christianity@en-ZM'})
SET t.display_name = 'Criticizing Christianity',
    t.locale = 'en-ZM',
    t.term = 'Criticizing Christianity',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticizing Christianity in en-ZM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZM'})
MATCH (t:Taboo {key: 'criticizing-christianity@en-ZM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-OM Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-OM'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-OM'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-sultan-or-royal-family@ar-OM'})
SET t.display_name = 'Criticism of Sultan or Royal Family',
    t.locale = 'ar-OM',
    t.term = 'Criticism of Sultan or Royal Family',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Sultan or Royal Family in ar-OM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-OM'})
MATCH (t:Taboo {key: 'criticism-of-sultan-or-royal-family@ar-OM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insulting-islam-or-ibadi-practices@ar-OM'})
SET t.display_name = 'Insulting Islam or Ibadi practices',
    t.locale = 'ar-OM',
    t.term = 'Insulting Islam or Ibadi practices',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or Ibadi practices in ar-OM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-OM'})
MATCH (t:Taboo {key: 'insulting-islam-or-ibadi-practices@ar-OM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-government-policies@ar-OM'})
SET t.display_name = 'Criticism of government policies',
    t.locale = 'ar-OM',
    t.term = 'Criticism of government policies',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of government policies in ar-OM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-OM'})
MATCH (t:Taboo {key: 'criticism-of-government-policies@ar-OM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ru-MD Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-MD'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-MD'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'transnistria-conflict@ru-MD'})
SET t.display_name = 'Transnistria conflict',
    t.locale = 'ru-MD',
    t.term = 'Transnistria conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Transnistria conflict in ru-MD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-MD'})
MATCH (t:Taboo {key: 'transnistria-conflict@ru-MD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'moldova-romania-relations@ru-MD'})
SET t.display_name = 'Moldova-Romania relations',
    t.locale = 'ru-MD',
    t.term = 'Moldova-Romania relations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Moldova-Romania relations in ru-MD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-MD'})
MATCH (t:Taboo {key: 'moldova-romania-relations@ru-MD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'russian-vs-romanian-language-debates@ru-MD'})
SET t.display_name = 'Russian vs Romanian language debates',
    t.locale = 'ru-MD',
    t.term = 'Russian vs Romanian language debates',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Russian vs Romanian language debates in ru-MD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-MD'})
MATCH (t:Taboo {key: 'russian-vs-romanian-language-debates@ru-MD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// mt-MT Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mt-MT'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@mt-MT'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'political-party-affiliation@mt-MT'})
SET t.display_name = 'Political party affiliation',
    t.locale = 'mt-MT',
    t.term = 'Political party affiliation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political party affiliation in mt-MT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mt-MT'})
MATCH (t:Taboo {key: 'political-party-affiliation@mt-MT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'abortion@mt-MT'})
SET t.display_name = 'Abortion',
    t.locale = 'mt-MT',
    t.term = 'Abortion',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Abortion in mt-MT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mt-MT'})
MATCH (t:Taboo {key: 'abortion@mt-MT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'pn-vs-pl-politics@mt-MT'})
SET t.display_name = 'PN vs PL politics',
    t.locale = 'mt-MT',
    t.term = 'PN vs PL politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: PN vs PL politics in mt-MT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mt-MT'})
MATCH (t:Taboo {key: 'pn-vs-pl-politics@mt-MT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// eu-ES Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'eu-ES'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@eu-ES'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'political-violence-eta-references@eu-ES'})
SET t.display_name = 'Political violence / ETA references',
    t.locale = 'eu-ES',
    t.term = 'Political violence / ETA references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political violence / ETA references in eu-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@eu-ES'})
MATCH (t:Taboo {key: 'political-violence-eta-references@eu-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'independence-politics@eu-ES'})
SET t.display_name = 'Independence politics',
    t.locale = 'eu-ES',
    t.term = 'Independence politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Independence politics in eu-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@eu-ES'})
MATCH (t:Taboo {key: 'independence-politics@eu-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'spanish-nationalism@eu-ES'})
SET t.display_name = 'Spanish nationalism',
    t.locale = 'eu-ES',
    t.term = 'Spanish nationalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Spanish nationalism in eu-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@eu-ES'})
MATCH (t:Taboo {key: 'spanish-nationalism@eu-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ja-JP Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ja-JP'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ja-JP'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'world-war-ii-and-imperial-history@ja-JP'})
SET t.display_name = 'World War II and imperial history',
    t.locale = 'ja-JP',
    t.term = 'World War II and imperial history',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: World War II and imperial history in ja-JP content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ja-JP'})
MATCH (t:Taboo {key: 'world-war-ii-and-imperial-history@ja-JP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'atomic-bombings-hiroshima@ja-JP'})
SET t.display_name = 'Atomic bombings (Hiroshima',
    t.locale = 'ja-JP',
    t.term = 'Atomic bombings (Hiroshima',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Atomic bombings (Hiroshima in ja-JP content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ja-JP'})
MATCH (t:Taboo {key: 'atomic-bombings-hiroshima@ja-JP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'nagasaki@ja-JP'})
SET t.display_name = 'Nagasaki)',
    t.locale = 'ja-JP',
    t.term = 'Nagasaki)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Nagasaki) in ja-JP content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ja-JP'})
MATCH (t:Taboo {key: 'nagasaki@ja-JP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'territorial-disputes@ja-JP'})
SET t.display_name = 'Territorial disputes',
    t.locale = 'ja-JP',
    t.term = 'Territorial disputes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Territorial disputes in ja-JP content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ja-JP'})
MATCH (t:Taboo {key: 'territorial-disputes@ja-JP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// bn-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'bn-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'hindu-muslim-communalism@bn-IN'})
SET t.display_name = 'Hindu-Muslim communalism',
    t.locale = 'bn-IN',
    t.term = 'Hindu-Muslim communalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Hindu-Muslim communalism in bn-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-IN'})
MATCH (t:Taboo {key: 'hindu-muslim-communalism@bn-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'partition-of-bengal@bn-IN'})
SET t.display_name = 'Partition of Bengal',
    t.locale = 'bn-IN',
    t.term = 'Partition of Bengal',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Partition of Bengal in bn-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-IN'})
MATCH (t:Taboo {key: 'partition-of-bengal@bn-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'caste-discrimination@bn-IN'})
SET t.display_name = 'Caste discrimination',
    t.locale = 'bn-IN',
    t.term = 'Caste discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Caste discrimination in bn-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-IN'})
MATCH (t:Taboo {key: 'caste-discrimination@bn-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// he-IL Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'he-IL'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@he-IL'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'holocaustshoah@he-IL'})
SET t.display_name = 'Holocaust/Shoah',
    t.locale = 'he-IL',
    t.term = 'Holocaust/Shoah',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Holocaust/Shoah in he-IL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@he-IL'})
MATCH (t:Taboo {key: 'holocaustshoah@he-IL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'israeli-palestinian-conflict@he-IL'})
SET t.display_name = 'Israeli-Palestinian Conflict',
    t.locale = 'he-IL',
    t.term = 'Israeli-Palestinian Conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Israeli-Palestinian Conflict in he-IL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@he-IL'})
MATCH (t:Taboo {key: 'israeli-palestinian-conflict@he-IL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'holocaustshoah@he-IL'})
SET t.display_name = 'Holocaust/Shoah',
    t.locale = 'he-IL',
    t.term = 'Holocaust/Shoah',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Holocaust/Shoah in he-IL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@he-IL'})
MATCH (t:Taboo {key: 'holocaustshoah@he-IL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// zh-HK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-HK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-HK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'political-status-and-governance@zh-HK'})
SET t.display_name = 'Political status and governance',
    t.locale = 'zh-HK',
    t.term = 'Political status and governance',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political status and governance in zh-HK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-HK'})
MATCH (t:Taboo {key: 'political-status-and-governance@zh-HK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'mainland-china-comparisons@zh-HK'})
SET t.display_name = 'Mainland China comparisons',
    t.locale = 'zh-HK',
    t.term = 'Mainland China comparisons',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mainland China comparisons in zh-HK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-HK'})
MATCH (t:Taboo {key: 'mainland-china-comparisons@zh-HK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taiwan-and-sovereignty-issues@zh-HK'})
SET t.display_name = 'Taiwan and sovereignty issues',
    t.locale = 'zh-HK',
    t.term = 'Taiwan and sovereignty issues',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Taiwan and sovereignty issues in zh-HK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-HK'})
MATCH (t:Taboo {key: 'taiwan-and-sovereignty-issues@zh-HK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-CO Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-CO'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CO'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'drug-traffickingnarcos-stereotypes@es-CO'})
SET t.display_name = 'Drug trafficking/narcos stereotypes',
    t.locale = 'es-CO',
    t.term = 'Drug trafficking/narcos stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Drug trafficking/narcos stereotypes in es-CO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CO'})
MATCH (t:Taboo {key: 'drug-traffickingnarcos-stereotypes@es-CO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'internal-armed-conflict-details@es-CO'})
SET t.display_name = 'Internal armed conflict details',
    t.locale = 'es-CO',
    t.term = 'Internal armed conflict details',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Internal armed conflict details in es-CO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CO'})
MATCH (t:Taboo {key: 'internal-armed-conflict-details@es-CO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'false-positives-scandal-falsos-positivos@es-CO'})
SET t.display_name = 'False positives scandal (falsos positivos)',
    t.locale = 'es-CO',
    t.term = 'False positives scandal (falsos positivos)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: False positives scandal (falsos positivos) in es-CO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CO'})
MATCH (t:Taboo {key: 'false-positives-scandal-falsos-positivos@es-CO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// de-LU Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'de-LU'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-LU'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'tax-haven-framing@de-LU'})
SET t.display_name = '"Tax haven" framing',
    t.locale = 'de-LU',
    t.term = '"Tax haven" framing',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: "Tax haven" framing in de-LU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-LU'})
MATCH (t:Taboo {key: 'tax-haven-framing@de-LU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'conflating-luxembourg-with-belgium-or-germany@de-LU'})
SET t.display_name = 'Conflating Luxembourg with Belgium or Germany',
    t.locale = 'de-LU',
    t.term = 'Conflating Luxembourg with Belgium or Germany',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Conflating Luxembourg with Belgium or Germany in de-LU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-LU'})
MATCH (t:Taboo {key: 'conflating-luxembourg-with-belgium-or-germany@de-LU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'discussing-personal-wealth-or-income@de-LU'})
SET t.display_name = 'Discussing personal wealth or income',
    t.locale = 'de-LU',
    t.term = 'Discussing personal wealth or income',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Discussing personal wealth or income in de-LU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-LU'})
MATCH (t:Taboo {key: 'discussing-personal-wealth-or-income@de-LU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-BO Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-BO'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-BO'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'sea-access-to-chile@es-BO'})
SET t.display_name = 'Sea access to Chile',
    t.locale = 'es-BO',
    t.term = 'Sea access to Chile',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Sea access to Chile in es-BO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-BO'})
MATCH (t:Taboo {key: 'sea-access-to-chile@es-BO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'indigenous-discrimination@es-BO'})
SET t.display_name = 'Indigenous discrimination',
    t.locale = 'es-BO',
    t.term = 'Indigenous discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Indigenous discrimination in es-BO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-BO'})
MATCH (t:Taboo {key: 'indigenous-discrimination@es-BO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-instability-jokes@es-BO'})
SET t.display_name = 'Political instability jokes',
    t.locale = 'es-BO',
    t.term = 'Political instability jokes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political instability jokes in es-BO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-BO'})
MATCH (t:Taboo {key: 'political-instability-jokes@es-BO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// th-TH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'th-TH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@th-TH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-monarchy@th-TH'})
SET t.display_name = 'Criticism of monarchy',
    t.locale = 'th-TH',
    t.term = 'Criticism of monarchy',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of monarchy in th-TH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@th-TH'})
MATCH (t:Taboo {key: 'criticism-of-monarchy@th-TH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'disrespecting-buddha-images@th-TH'})
SET t.display_name = 'Disrespecting Buddha images',
    t.locale = 'th-TH',
    t.term = 'Disrespecting Buddha images',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting Buddha images in th-TH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@th-TH'})
MATCH (t:Taboo {key: 'disrespecting-buddha-images@th-TH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insulting-buddhism-or-monks@th-TH'})
SET t.display_name = 'Insulting Buddhism or monks',
    t.locale = 'th-TH',
    t.term = 'Insulting Buddhism or monks',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Buddhism or monks in th-TH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@th-TH'})
MATCH (t:Taboo {key: 'insulting-buddhism-or-monks@th-TH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// kk-KZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'kk-KZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@kk-KZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-government-or-leadership@kk-KZ'})
SET t.display_name = 'Criticism of government or leadership',
    t.locale = 'kk-KZ',
    t.term = 'Criticism of government or leadership',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of government or leadership in kk-KZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@kk-KZ'})
MATCH (t:Taboo {key: 'criticism-of-government-or-leadership@kk-KZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'inter-ethnic-tensions-historical-or-current@kk-KZ'})
SET t.display_name = 'Inter-ethnic tensions (historical or current)',
    t.locale = 'kk-KZ',
    t.term = 'Inter-ethnic tensions (historical or current)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Inter-ethnic tensions (historical or current) in kk-KZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@kk-KZ'})
MATCH (t:Taboo {key: 'inter-ethnic-tensions-historical-or-current@kk-KZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'territorial-disputes-or-border-issues@kk-KZ'})
SET t.display_name = 'Territorial disputes or border issues',
    t.locale = 'kk-KZ',
    t.term = 'Territorial disputes or border issues',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Territorial disputes or border issues in kk-KZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@kk-KZ'})
MATCH (t:Taboo {key: 'territorial-disputes-or-border-issues@kk-KZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fi-FI Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fi-FI'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fi-FI'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'bragging-or-self-promotion@fi-FI'})
SET t.display_name = 'Bragging or self-promotion',
    t.locale = 'fi-FI',
    t.term = 'Bragging or self-promotion',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Bragging or self-promotion in fi-FI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fi-FI'})
MATCH (t:Taboo {key: 'bragging-or-self-promotion@fi-FI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'comparing-finland-to-russia@fi-FI'})
SET t.display_name = 'Comparing Finland to Russia',
    t.locale = 'fi-FI',
    t.term = 'Comparing Finland to Russia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparing Finland to Russia in fi-FI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fi-FI'})
MATCH (t:Taboo {key: 'comparing-finland-to-russia@fi-FI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'promising-more-than-you-deliver@fi-FI'})
SET t.display_name = 'Promising more than you deliver',
    t.locale = 'fi-FI',
    t.term = 'Promising more than you deliver',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Promising more than you deliver in fi-FI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fi-FI'})
MATCH (t:Taboo {key: 'promising-more-than-you-deliver@fi-FI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-TN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-TN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-TN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-prophet@ar-TN'})
SET t.display_name = 'Insulting Islam or Prophet',
    t.locale = 'ar-TN',
    t.term = 'Insulting Islam or Prophet',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or Prophet in ar-TN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-TN'})
MATCH (t:Taboo {key: 'insulting-islam-or-prophet@ar-TN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticizing-the-military@ar-TN'})
SET t.display_name = 'Criticizing the military',
    t.locale = 'ar-TN',
    t.term = 'Criticizing the military',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticizing the military in ar-TN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-TN'})
MATCH (t:Taboo {key: 'criticizing-the-military@ar-TN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taking-sides-on-2011-revolution-politics@ar-TN'})
SET t.display_name = 'Taking sides on 2011 Revolution politics',
    t.locale = 'ar-TN',
    t.term = 'Taking sides on 2011 Revolution politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Taking sides on 2011 Revolution politics in ar-TN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-TN'})
MATCH (t:Taboo {key: 'taking-sides-on-2011-revolution-politics@ar-TN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-VN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-VN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-VN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-communist-party-or-government@en-VN'})
SET t.display_name = 'Criticism of Communist Party or government',
    t.locale = 'en-VN',
    t.term = 'Criticism of Communist Party or government',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Communist Party or government in en-VN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-VN'})
MATCH (t:Taboo {key: 'criticism-of-communist-party-or-government@en-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'territorial-disputes-south-china-sea@en-VN'})
SET t.display_name = 'Territorial disputes (South China Sea)',
    t.locale = 'en-VN',
    t.term = 'Territorial disputes (South China Sea)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Territorial disputes (South China Sea) in en-VN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-VN'})
MATCH (t:Taboo {key: 'territorial-disputes-south-china-sea@en-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'historical-war-commentary@en-VN'})
SET t.display_name = 'Historical war commentary',
    t.locale = 'en-VN',
    t.term = 'Historical war commentary',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Historical war commentary in en-VN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-VN'})
MATCH (t:Taboo {key: 'historical-war-commentary@en-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// tl-PH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'tl-PH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@tl-PH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'disrespecting-religious-symbols-or-beliefs@tl-PH'})
SET t.display_name = 'Disrespecting religious symbols or beliefs',
    t.locale = 'tl-PH',
    t.term = 'Disrespecting religious symbols or beliefs',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting religious symbols or beliefs in tl-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tl-PH'})
MATCH (t:Taboo {key: 'disrespecting-religious-symbols-or-beliefs@tl-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'causing-public-hiya-shameembarrassment@tl-PH'})
SET t.display_name = 'Causing public hiya (shame/embarrassment)',
    t.locale = 'tl-PH',
    t.term = 'Causing public hiya (shame/embarrassment)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Causing public hiya (shame/embarrassment) in tl-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tl-PH'})
MATCH (t:Taboo {key: 'causing-public-hiya-shameembarrassment@tl-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'mocking-or-disrespecting-elders@tl-PH'})
SET t.display_name = 'Mocking or disrespecting elders',
    t.locale = 'tl-PH',
    t.term = 'Mocking or disrespecting elders',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mocking or disrespecting elders in tl-PH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tl-PH'})
MATCH (t:Taboo {key: 'mocking-or-disrespecting-elders@tl-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-PK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-PK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'blasphemy-and-disrespect-to-islam@en-PK'})
SET t.display_name = 'Blasphemy and disrespect to Islam',
    t.locale = 'en-PK',
    t.term = 'Blasphemy and disrespect to Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Blasphemy and disrespect to Islam in en-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PK'})
MATCH (t:Taboo {key: 'blasphemy-and-disrespect-to-islam@en-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'anti-pakistan-sentiment@en-PK'})
SET t.display_name = 'Anti-Pakistan sentiment',
    t.locale = 'en-PK',
    t.term = 'Anti-Pakistan sentiment',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Anti-Pakistan sentiment in en-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PK'})
MATCH (t:Taboo {key: 'anti-pakistan-sentiment@en-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'kashmir-dispute-taking-india@en-PK'})
SET t.display_name = 'Kashmir dispute (taking India\\',
    t.locale = 'en-PK',
    t.term = 'Kashmir dispute (taking India\\',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Kashmir dispute (taking India\\ in en-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PK'})
MATCH (t:Taboo {key: 'kashmir-dispute-taking-india@en-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-CH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'comparer-les-suisses-aux-francais-de-maniere-defav@fr-CH'})
SET t.display_name = 'Comparer les Suisses aux Francais de maniere defavorable',
    t.locale = 'fr-CH',
    t.term = 'Comparer les Suisses aux Francais de maniere defavorable',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparer les Suisses aux Francais de maniere defavorable in fr-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CH'})
MATCH (t:Taboo {key: 'comparer-les-suisses-aux-francais-de-maniere-defav@fr-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'confondre-suisse-romande-et-france@fr-CH'})
SET t.display_name = 'Confondre Suisse romande et France',
    t.locale = 'fr-CH',
    t.term = 'Confondre Suisse romande et France',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Confondre Suisse romande et France in fr-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CH'})
MATCH (t:Taboo {key: 'confondre-suisse-romande-et-france@fr-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'favoritisme-entre-regions-linguistiques@fr-CH'})
SET t.display_name = 'Favoritisme entre regions linguistiques',
    t.locale = 'fr-CH',
    t.term = 'Favoritisme entre regions linguistiques',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Favoritisme entre regions linguistiques in fr-CH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CH'})
MATCH (t:Taboo {key: 'favoritisme-entre-regions-linguistiques@fr-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// cy-GB Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'cy-GB'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@cy-GB'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'welsh-language-politics@cy-GB'})
SET t.display_name = 'Welsh language politics',
    t.locale = 'cy-GB',
    t.term = 'Welsh language politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Welsh language politics in cy-GB content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@cy-GB'})
MATCH (t:Taboo {key: 'welsh-language-politics@cy-GB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'linguistic-imperialism@cy-GB'})
SET t.display_name = 'Linguistic imperialism',
    t.locale = 'cy-GB',
    t.term = 'Linguistic imperialism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Linguistic imperialism in cy-GB content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@cy-GB'})
MATCH (t:Taboo {key: 'linguistic-imperialism@cy-GB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-FJ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-FJ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-FJ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'touching-someone@en-FJ'})
SET t.display_name = 'Touching someone\\',
    t.locale = 'en-FJ',
    t.term = 'Touching someone\\',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Touching someone\\ in en-FJ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-FJ'})
MATCH (t:Taboo {key: 'touching-someone@en-FJ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// af-ZA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'af-ZA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@af-ZA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'racial-stereotypes@af-ZA'})
SET t.display_name = 'Racial stereotypes',
    t.locale = 'af-ZA',
    t.term = 'Racial stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racial stereotypes in af-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@af-ZA'})
MATCH (t:Taboo {key: 'racial-stereotypes@af-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'apartheid-nostalgia@af-ZA'})
SET t.display_name = 'Apartheid nostalgia',
    t.locale = 'af-ZA',
    t.term = 'Apartheid nostalgia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Apartheid nostalgia in af-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@af-ZA'})
MATCH (t:Taboo {key: 'apartheid-nostalgia@af-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'land-ownership-debates@af-ZA'})
SET t.display_name = 'Land ownership debates',
    t.locale = 'af-ZA',
    t.term = 'Land ownership debates',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Land ownership debates in af-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@af-ZA'})
MATCH (t:Taboo {key: 'land-ownership-debates@af-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-JM Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-JM'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-JM'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'mocking-patoisjamaican-english@en-JM'})
SET t.display_name = 'Mocking Patois/Jamaican English',
    t.locale = 'en-JM',
    t.term = 'Mocking Patois/Jamaican English',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mocking Patois/Jamaican English in en-JM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-JM'})
MATCH (t:Taboo {key: 'mocking-patoisjamaican-english@en-JM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'slavery-glorification@en-JM'})
SET t.display_name = 'Slavery glorification',
    t.locale = 'en-JM',
    t.term = 'Slavery glorification',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Slavery glorification in en-JM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-JM'})
MATCH (t:Taboo {key: 'slavery-glorification@en-JM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'violencecrime-stereotypes@en-JM'})
SET t.display_name = 'Violence/crime stereotypes',
    t.locale = 'en-JM',
    t.term = 'Violence/crime stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Violence/crime stereotypes in en-JM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-JM'})
MATCH (t:Taboo {key: 'violencecrime-stereotypes@en-JM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// da-DK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'da-DK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@da-DK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'immigration-politics@da-DK'})
SET t.display_name = 'Immigration politics',
    t.locale = 'da-DK',
    t.term = 'Immigration politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Immigration politics in da-DK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@da-DK'})
MATCH (t:Taboo {key: 'immigration-politics@da-DK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'immigrationintegration@da-DK'})
SET t.display_name = 'Immigration/integration',
    t.locale = 'da-DK',
    t.term = 'Immigration/integration',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Immigration/integration in da-DK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@da-DK'})
MATCH (t:Taboo {key: 'immigrationintegration@da-DK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-US Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-US'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-US'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'racism-and-discrimination@en-US'})
SET t.display_name = 'Racism and discrimination',
    t.locale = 'en-US',
    t.term = 'Racism and discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racism and discrimination in en-US content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-US'})
MATCH (t:Taboo {key: 'racism-and-discrimination@en-US'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'gun-violence@en-US'})
SET t.display_name = 'Gun violence',
    t.locale = 'en-US',
    t.term = 'Gun violence',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Gun violence in en-US content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-US'})
MATCH (t:Taboo {key: 'gun-violence@en-US'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'slavery-and-jim-crow@en-US'})
SET t.display_name = 'Slavery and Jim Crow',
    t.locale = 'en-US',
    t.term = 'Slavery and Jim Crow',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Slavery and Jim Crow in en-US content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-US'})
MATCH (t:Taboo {key: 'slavery-and-jim-crow@en-US'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// pt-PT Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-PT'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-PT'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'colonial-history-criticism@pt-PT'})
SET t.display_name = 'Colonial history criticism',
    t.locale = 'pt-PT',
    t.term = 'Colonial history criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Colonial history criticism in pt-PT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-PT'})
MATCH (t:Taboo {key: 'colonial-history-criticism@pt-PT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ga-IE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ga-IE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ga-IE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'the-troubles-and-partition@ga-IE'})
SET t.display_name = 'The Troubles and partition',
    t.locale = 'ga-IE',
    t.term = 'The Troubles and partition',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: The Troubles and partition in ga-IE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ga-IE'})
MATCH (t:Taboo {key: 'the-troubles-and-partition@ga-IE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'british-irish-relations@ga-IE'})
SET t.display_name = 'British-Irish relations',
    t.locale = 'ga-IE',
    t.term = 'British-Irish relations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: British-Irish relations in ga-IE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ga-IE'})
MATCH (t:Taboo {key: 'british-irish-relations@ga-IE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'the-great-famine-an-gorta-mor@ga-IE'})
SET t.display_name = 'The Great Famine (An Gorta Mor)',
    t.locale = 'ga-IE',
    t.term = 'The Great Famine (An Gorta Mor)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: The Great Famine (An Gorta Mor) in ga-IE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ga-IE'})
MATCH (t:Taboo {key: 'the-great-famine-an-gorta-mor@ga-IE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-AR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-AR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-AR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'military-dictatorship-1976-1983@es-AR'})
SET t.display_name = 'Military dictatorship (1976-1983)',
    t.locale = 'es-AR',
    t.term = 'Military dictatorship (1976-1983)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Military dictatorship (1976-1983) in es-AR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-AR'})
MATCH (t:Taboo {key: 'military-dictatorship-1976-1983@es-AR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'malvinasfalklands@es-AR'})
SET t.display_name = 'Malvinas/Falklands',
    t.locale = 'es-AR',
    t.term = 'Malvinas/Falklands',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Malvinas/Falklands in es-AR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-AR'})
MATCH (t:Taboo {key: 'malvinasfalklands@es-AR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'defending-dictatorship-actions@es-AR'})
SET t.display_name = 'Defending dictatorship actions',
    t.locale = 'es-AR',
    t.term = 'Defending dictatorship actions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Defending dictatorship actions in es-AR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-AR'})
MATCH (t:Taboo {key: 'defending-dictatorship-actions@es-AR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// gl-ES Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'gl-ES'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@gl-ES'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'dismissing-galician-language@gl-ES'})
SET t.display_name = 'Dismissing Galician language',
    t.locale = 'gl-ES',
    t.term = 'Dismissing Galician language',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Dismissing Galician language in gl-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@gl-ES'})
MATCH (t:Taboo {key: 'dismissing-galician-language@gl-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'mocking-rural-galicia@gl-ES'})
SET t.display_name = 'Mocking rural Galicia',
    t.locale = 'gl-ES',
    t.term = 'Mocking rural Galicia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mocking rural Galicia in gl-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@gl-ES'})
MATCH (t:Taboo {key: 'mocking-rural-galicia@gl-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'franco-dictatorship-nostalgia@gl-ES'})
SET t.display_name = 'Franco dictatorship nostalgia',
    t.locale = 'gl-ES',
    t.term = 'Franco dictatorship nostalgia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Franco dictatorship nostalgia in gl-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@gl-ES'})
MATCH (t:Taboo {key: 'franco-dictatorship-nostalgia@gl-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-ES Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-ES'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-ES'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'guerra-civil-espanola@es-ES'})
SET t.display_name = 'Guerra Civil Espanola',
    t.locale = 'es-ES',
    t.term = 'Guerra Civil Espanola',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Guerra Civil Espanola in es-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-ES'})
MATCH (t:Taboo {key: 'guerra-civil-espanola@es-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'terrorismo-eta@es-ES'})
SET t.display_name = 'Terrorismo ETA',
    t.locale = 'es-ES',
    t.term = 'Terrorismo ETA',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Terrorismo ETA in es-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-ES'})
MATCH (t:Taboo {key: 'terrorismo-eta@es-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'independentismo-catalanvasco@es-ES'})
SET t.display_name = 'Independentismo catalan/vasco',
    t.locale = 'es-ES',
    t.term = 'Independentismo catalan/vasco',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Independentismo catalan/vasco in es-ES content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-ES'})
MATCH (t:Taboo {key: 'independentismo-catalanvasco@es-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// zh-TH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-TH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'thai-monarchy-criticism@zh-TH'})
SET t.display_name = 'Thai monarchy criticism',
    t.locale = 'zh-TH',
    t.term = 'Thai monarchy criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Thai monarchy criticism in zh-TH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TH'})
MATCH (t:Taboo {key: 'thai-monarchy-criticism@zh-TH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-divisions@zh-TH'})
SET t.display_name = 'Political divisions',
    t.locale = 'zh-TH',
    t.term = 'Political divisions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political divisions in zh-TH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TH'})
MATCH (t:Taboo {key: 'political-divisions@zh-TH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'buddhist-disrespect@zh-TH'})
SET t.display_name = 'Buddhist disrespect',
    t.locale = 'zh-TH',
    t.term = 'Buddhist disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Buddhist disrespect in zh-TH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TH'})
MATCH (t:Taboo {key: 'buddhist-disrespect@zh-TH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// be-BY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'be-BY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@be-BY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'current-political-situation@be-BY'})
SET t.display_name = 'Current political situation',
    t.locale = 'be-BY',
    t.term = 'Current political situation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Current political situation in be-BY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@be-BY'})
MATCH (t:Taboo {key: 'current-political-situation@be-BY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'government-criticism@be-BY'})
SET t.display_name = 'Government criticism',
    t.locale = 'be-BY',
    t.term = 'Government criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Government criticism in be-BY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@be-BY'})
MATCH (t:Taboo {key: 'government-criticism@be-BY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '2020-protests-and-aftermath@be-BY'})
SET t.display_name = '2020 protests and aftermath',
    t.locale = 'be-BY',
    t.term = '2020 protests and aftermath',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 2020 protests and aftermath in be-BY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@be-BY'})
MATCH (t:Taboo {key: '2020-protests-and-aftermath@be-BY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// wo-SN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'wo-SN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@wo-SN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-the-prophet@wo-SN'})
SET t.display_name = 'Insulting Islam or the Prophet',
    t.locale = 'wo-SN',
    t.term = 'Insulting Islam or the Prophet',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or the Prophet in wo-SN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@wo-SN'})
MATCH (t:Taboo {key: 'insulting-islam-or-the-prophet@wo-SN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'disrespecting-sufi-brotherhoods@wo-SN'})
SET t.display_name = 'Disrespecting Sufi brotherhoods',
    t.locale = 'wo-SN',
    t.term = 'Disrespecting Sufi brotherhoods',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting Sufi brotherhoods in wo-SN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@wo-SN'})
MATCH (t:Taboo {key: 'disrespecting-sufi-brotherhoods@wo-SN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'lgbtq-references@wo-SN'})
SET t.display_name = 'LGBTQ+ references',
    t.locale = 'wo-SN',
    t.term = 'LGBTQ+ references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: LGBTQ+ references in wo-SN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@wo-SN'})
MATCH (t:Taboo {key: 'lgbtq-references@wo-SN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-NI Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-NI'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-NI'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'political-commentary@es-NI'})
SET t.display_name = 'Political commentary',
    t.locale = 'es-NI',
    t.term = 'Political commentary',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political commentary in es-NI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-NI'})
MATCH (t:Taboo {key: 'political-commentary@es-NI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'sandinistacontra-history@es-NI'})
SET t.display_name = 'Sandinista/Contra history',
    t.locale = 'es-NI',
    t.term = 'Sandinista/Contra history',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Sandinista/Contra history in es-NI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-NI'})
MATCH (t:Taboo {key: 'sandinistacontra-history@es-NI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '2018-protests@es-NI'})
SET t.display_name = '2018 protests',
    t.locale = 'es-NI',
    t.term = '2018 protests',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 2018 protests in es-NI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-NI'})
MATCH (t:Taboo {key: '2018-protests@es-NI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// sv-SE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sv-SE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sv-SE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'bragging-or-self-promotion@sv-SE'})
SET t.display_name = 'Bragging or self-promotion',
    t.locale = 'sv-SE',
    t.term = 'Bragging or self-promotion',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Bragging or self-promotion in sv-SE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sv-SE'})
MATCH (t:Taboo {key: 'bragging-or-self-promotion@sv-SE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'immigration-politics@sv-SE'})
SET t.display_name = 'Immigration politics',
    t.locale = 'sv-SE',
    t.term = 'Immigration politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Immigration politics in sv-SE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sv-SE'})
MATCH (t:Taboo {key: 'immigration-politics@sv-SE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'overpromising@sv-SE'})
SET t.display_name = 'Overpromising',
    t.locale = 'sv-SE',
    t.term = 'Overpromising',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Overpromising in sv-SE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sv-SE'})
MATCH (t:Taboo {key: 'overpromising@sv-SE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// hr-HR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'hr-HR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@hr-HR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'yugoslav-period-nostalgia@hr-HR'})
SET t.display_name = 'Yugoslav period nostalgia',
    t.locale = 'hr-HR',
    t.term = 'Yugoslav period nostalgia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Yugoslav period nostalgia in hr-HR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hr-HR'})
MATCH (t:Taboo {key: 'yugoslav-period-nostalgia@hr-HR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '1991-1995-homeland-war-references@hr-HR'})
SET t.display_name = '1991-1995 Homeland War references',
    t.locale = 'hr-HR',
    t.term = '1991-1995 Homeland War references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 1991-1995 Homeland War references in hr-HR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hr-HR'})
MATCH (t:Taboo {key: '1991-1995-homeland-war-references@hr-HR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-tensions-or-comparisons@hr-HR'})
SET t.display_name = 'Ethnic tensions or comparisons',
    t.locale = 'hr-HR',
    t.term = 'Ethnic tensions or comparisons',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic tensions or comparisons in hr-HR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hr-HR'})
MATCH (t:Taboo {key: 'ethnic-tensions-or-comparisons@hr-HR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-CI Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CI'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CI'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'crise-post-electorale-2010-2011@fr-CI'})
SET t.display_name = 'Crise post-electorale 2010-2011',
    t.locale = 'fr-CI',
    t.term = 'Crise post-electorale 2010-2011',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Crise post-electorale 2010-2011 in fr-CI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CI'})
MATCH (t:Taboo {key: 'crise-post-electorale-2010-2011@fr-CI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnicisme-et-clivages-nord-sud@fr-CI'})
SET t.display_name = 'Ethnicisme et clivages Nord-Sud',
    t.locale = 'fr-CI',
    t.term = 'Ethnicisme et clivages Nord-Sud',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnicisme et clivages Nord-Sud in fr-CI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CI'})
MATCH (t:Taboo {key: 'ethnicisme-et-clivages-nord-sud@fr-CI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insultes-aux-parents-ou-ancetres@fr-CI'})
SET t.display_name = 'Insultes aux parents ou ancetres',
    t.locale = 'fr-CI',
    t.term = 'Insultes aux parents ou ancetres',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insultes aux parents ou ancetres in fr-CI content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CI'})
MATCH (t:Taboo {key: 'insultes-aux-parents-ou-ancetres@fr-CI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ku-TR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ku-TR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ku-TR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'pkk-and-armed-conflict@ku-TR'})
SET t.display_name = 'PKK and armed conflict',
    t.locale = 'ku-TR',
    t.term = 'PKK and armed conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: PKK and armed conflict in ku-TR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ku-TR'})
MATCH (t:Taboo {key: 'pkk-and-armed-conflict@ku-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'kurdish-political-parties-and-movements@ku-TR'})
SET t.display_name = 'Kurdish political parties and movements',
    t.locale = 'ku-TR',
    t.term = 'Kurdish political parties and movements',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Kurdish political parties and movements in ku-TR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ku-TR'})
MATCH (t:Taboo {key: 'kurdish-political-parties-and-movements@ku-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'turkey-kurdistan-territorial-discussions@ku-TR'})
SET t.display_name = 'Turkey-Kurdistan territorial discussions',
    t.locale = 'ku-TR',
    t.term = 'Turkey-Kurdistan territorial discussions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Turkey-Kurdistan territorial discussions in ku-TR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ku-TR'})
MATCH (t:Taboo {key: 'turkey-kurdistan-territorial-discussions@ku-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-BW Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-BW'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-BW'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'disrespecting-elderschiefs@en-BW'})
SET t.display_name = 'Disrespecting elders/chiefs',
    t.locale = 'en-BW',
    t.term = 'Disrespecting elders/chiefs',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting elders/chiefs in en-BW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-BW'})
MATCH (t:Taboo {key: 'disrespecting-elderschiefs@en-BW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'witchcraft-accusations@en-BW'})
SET t.display_name = 'Witchcraft accusations',
    t.locale = 'en-BW',
    t.term = 'Witchcraft accusations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Witchcraft accusations in en-BW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-BW'})
MATCH (t:Taboo {key: 'witchcraft-accusations@en-BW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'hivaids-stigmatization@en-BW'})
SET t.display_name = 'HIV/AIDS stigmatization',
    t.locale = 'en-BW',
    t.term = 'HIV/AIDS stigmatization',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: HIV/AIDS stigmatization in en-BW content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-BW'})
MATCH (t:Taboo {key: 'hivaids-stigmatization@en-BW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-IQ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-IQ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-IQ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'sectarian-references-or-division-sunnishia@ar-IQ'})
SET t.display_name = 'Sectarian references or division (Sunni/Shia)',
    t.locale = 'ar-IQ',
    t.term = 'Sectarian references or division (Sunni/Shia)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Sectarian references or division (Sunni/Shia) in ar-IQ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-IQ'})
MATCH (t:Taboo {key: 'sectarian-references-or-division-sunnishia@ar-IQ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-islam-or-religious-figures@ar-IQ'})
SET t.display_name = 'Criticism of Islam or religious figures',
    t.locale = 'ar-IQ',
    t.term = 'Criticism of Islam or religious figures',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Islam or religious figures in ar-IQ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-IQ'})
MATCH (t:Taboo {key: 'criticism-of-islam-or-religious-figures@ar-IQ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-marja-religious-authorities@ar-IQ'})
SET t.display_name = 'Criticism of Marja (religious authorities)',
    t.locale = 'ar-IQ',
    t.term = 'Criticism of Marja (religious authorities)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Marja (religious authorities) in ar-IQ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-IQ'})
MATCH (t:Taboo {key: 'criticism-of-marja-religious-authorities@ar-IQ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// pa-PK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pa-PK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pa-PK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'blasphemy@pa-PK'})
SET t.display_name = 'اسلام دی توہین (Blasphemy)',
    t.locale = 'pa-PK',
    t.term = 'اسلام دی توہین (Blasphemy)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: اسلام دی توہین (Blasphemy) in pa-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pa-PK'})
MATCH (t:Taboo {key: 'blasphemy@pa-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'pro-india-sentiment@pa-PK'})
SET t.display_name = 'بھارت نال ہمدردی (Pro-India sentiment)',
    t.locale = 'pa-PK',
    t.term = 'بھارت نال ہمدردی (Pro-India sentiment)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: بھارت نال ہمدردی (Pro-India sentiment) in pa-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pa-PK'})
MATCH (t:Taboo {key: 'pro-india-sentiment@pa-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'sectarian-content@pa-PK'})
SET t.display_name = 'فرقہ واریت (Sectarian content)',
    t.locale = 'pa-PK',
    t.term = 'فرقہ واریت (Sectarian content)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: فرقہ واریت (Sectarian content) in pa-PK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pa-PK'})
MATCH (t:Taboo {key: 'sectarian-content@pa-PK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-DO Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-DO'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-DO'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'trujillo-dictatorship-1930-1961@es-DO'})
SET t.display_name = 'Trujillo dictatorship (1930-1961)',
    t.locale = 'es-DO',
    t.term = 'Trujillo dictatorship (1930-1961)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Trujillo dictatorship (1930-1961) in es-DO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-DO'})
MATCH (t:Taboo {key: 'trujillo-dictatorship-1930-1961@es-DO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'haitihaitian-relations@es-DO'})
SET t.display_name = 'Haiti/Haitian relations',
    t.locale = 'es-DO',
    t.term = 'Haiti/Haitian relations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Haiti/Haitian relations in es-DO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-DO'})
MATCH (t:Taboo {key: 'haitihaitian-relations@es-DO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'colorismracial-hierarchy@es-DO'})
SET t.display_name = 'Colorism/racial hierarchy',
    t.locale = 'es-DO',
    t.term = 'Colorism/racial hierarchy',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Colorism/racial hierarchy in es-DO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-DO'})
MATCH (t:Taboo {key: 'colorismracial-hierarchy@es-DO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-CR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-CR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'nicaragua-tensions@es-CR'})
SET t.display_name = 'Nicaragua tensions',
    t.locale = 'es-CR',
    t.term = 'Nicaragua tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Nicaragua tensions in es-CR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CR'})
MATCH (t:Taboo {key: 'nicaragua-tensions@es-CR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'calling-costa-ricans-nicaraguans-or-implying-centr@es-CR'})
SET t.display_name = 'Calling Costa Ricans "Nicaraguans" or implying Central American sameness',
    t.locale = 'es-CR',
    t.term = 'Calling Costa Ricans "Nicaraguans" or implying Central American sameness',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Calling Costa Ricans "Nicaraguans" or implying Central American sameness in es-CR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CR'})
MATCH (t:Taboo {key: 'calling-costa-ricans-nicaraguans-or-implying-centr@es-CR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ps-AF Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ps-AF'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ps-AF'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-islam-or-islamic-governance@ps-AF'})
SET t.display_name = 'Criticism of Islam or Islamic governance',
    t.locale = 'ps-AF',
    t.term = 'Criticism of Islam or Islamic governance',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Islam or Islamic governance in ps-AF content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ps-AF'})
MATCH (t:Taboo {key: 'criticism-of-islam-or-islamic-governance@ps-AF'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'women@ps-AF'})
SET t.display_name = 'Women\\',
    t.locale = 'ps-AF',
    t.term = 'Women\\',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Women\\ in ps-AF content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ps-AF'})
MATCH (t:Taboo {key: 'women@ps-AF'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-CM Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CM'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CM'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'anglophone-crisisseparatism@fr-CM'})
SET t.display_name = 'Anglophone crisis/separatism',
    t.locale = 'fr-CM',
    t.term = 'Anglophone crisis/separatism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Anglophone crisis/separatism in fr-CM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CM'})
MATCH (t:Taboo {key: 'anglophone-crisisseparatism@fr-CM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-favoritism-or-tribalism@fr-CM'})
SET t.display_name = 'Ethnic favoritism or tribalism',
    t.locale = 'fr-CM',
    t.term = 'Ethnic favoritism or tribalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic favoritism or tribalism in fr-CM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CM'})
MATCH (t:Taboo {key: 'ethnic-favoritism-or-tribalism@fr-CM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-commentary-government@fr-CM'})
SET t.display_name = 'Political commentary (government)',
    t.locale = 'fr-CM',
    t.term = 'Political commentary (government)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political commentary (government) in fr-CM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CM'})
MATCH (t:Taboo {key: 'political-commentary-government@fr-CM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// el-GR Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'el-GR'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-GR'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'turkey-relations-and-cyprus-issue@el-GR'})
SET t.display_name = 'Turkey relations and Cyprus issue',
    t.locale = 'el-GR',
    t.term = 'Turkey relations and Cyprus issue',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Turkey relations and Cyprus issue in el-GR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-GR'})
MATCH (t:Taboo {key: 'turkey-relations-and-cyprus-issue@el-GR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'macedonia-naming-dispute@el-GR'})
SET t.display_name = 'Macedonia naming dispute',
    t.locale = 'el-GR',
    t.term = 'Macedonia naming dispute',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Macedonia naming dispute in el-GR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-GR'})
MATCH (t:Taboo {key: 'macedonia-naming-dispute@el-GR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'cyprus-invasion-1974@el-GR'})
SET t.display_name = 'Cyprus invasion (1974)',
    t.locale = 'el-GR',
    t.term = 'Cyprus invasion (1974)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Cyprus invasion (1974) in el-GR content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-GR'})
MATCH (t:Taboo {key: 'cyprus-invasion-1974@el-GR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// hi-IN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'hi-IN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@hi-IN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'religious-communalism@hi-IN'})
SET t.display_name = 'Religious communalism',
    t.locale = 'hi-IN',
    t.term = 'Religious communalism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious communalism in hi-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hi-IN'})
MATCH (t:Taboo {key: 'religious-communalism@hi-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'caste-discrimination@hi-IN'})
SET t.display_name = 'Caste discrimination',
    t.locale = 'hi-IN',
    t.term = 'Caste discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Caste discrimination in hi-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hi-IN'})
MATCH (t:Taboo {key: 'caste-discrimination@hi-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'kashmir-and-territorial-issues@hi-IN'})
SET t.display_name = 'Kashmir and territorial issues',
    t.locale = 'hi-IN',
    t.term = 'Kashmir and territorial issues',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Kashmir and territorial issues in hi-IN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@hi-IN'})
MATCH (t:Taboo {key: 'kashmir-and-territorial-issues@hi-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// qu-PE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'qu-PE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@qu-PE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'spanish-conquest-and-colonization@qu-PE'})
SET t.display_name = 'Spanish conquest and colonization',
    t.locale = 'qu-PE',
    t.term = 'Spanish conquest and colonization',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Spanish conquest and colonization in qu-PE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@qu-PE'})
MATCH (t:Taboo {key: 'spanish-conquest-and-colonization@qu-PE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'indigenous-poverty-stereotypes@qu-PE'})
SET t.display_name = 'Indigenous poverty stereotypes',
    t.locale = 'qu-PE',
    t.term = 'Indigenous poverty stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Indigenous poverty stereotypes in qu-PE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@qu-PE'})
MATCH (t:Taboo {key: 'indigenous-poverty-stereotypes@qu-PE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'linguistic-discrimination@qu-PE'})
SET t.display_name = 'Linguistic discrimination',
    t.locale = 'qu-PE',
    t.term = 'Linguistic discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Linguistic discrimination in qu-PE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@qu-PE'})
MATCH (t:Taboo {key: 'linguistic-discrimination@qu-PE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// sr-RS Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sr-RS'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sr-RS'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'kosovo-references@sr-RS'})
SET t.display_name = 'Kosovo references',
    t.locale = 'sr-RS',
    t.term = 'Kosovo references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Kosovo references in sr-RS content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sr-RS'})
MATCH (t:Taboo {key: 'kosovo-references@sr-RS'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: '1990s-wars-and-nato-bombing@sr-RS'})
SET t.display_name = '1990s wars and NATO bombing',
    t.locale = 'sr-RS',
    t.term = '1990s wars and NATO bombing',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: 1990s wars and NATO bombing in sr-RS content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sr-RS'})
MATCH (t:Taboo {key: '1990s-wars-and-nato-bombing@sr-RS'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-tensions-or-comparisons@sr-RS'})
SET t.display_name = 'Ethnic tensions or comparisons',
    t.locale = 'sr-RS',
    t.term = 'Ethnic tensions or comparisons',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic tensions or comparisons in sr-RS content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sr-RS'})
MATCH (t:Taboo {key: 'ethnic-tensions-or-comparisons@sr-RS'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ms-BN Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ms-BN'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-BN'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-sultan-or-royal-family@ms-BN'})
SET t.display_name = 'Criticism of Sultan or Royal Family',
    t.locale = 'ms-BN',
    t.term = 'Criticism of Sultan or Royal Family',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Sultan or Royal Family in ms-BN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-BN'})
MATCH (t:Taboo {key: 'criticism-of-sultan-or-royal-family@ms-BN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-islam-or-islamic-law-sharia@ms-BN'})
SET t.display_name = 'Criticism of Islam or Islamic law (Sharia)',
    t.locale = 'ms-BN',
    t.term = 'Criticism of Islam or Islamic law (Sharia)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Islam or Islamic law (Sharia) in ms-BN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-BN'})
MATCH (t:Taboo {key: 'criticism-of-islam-or-islamic-law-sharia@ms-BN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'alcohol-references@ms-BN'})
SET t.display_name = 'Alcohol references',
    t.locale = 'ms-BN',
    t.term = 'Alcohol references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Alcohol references in ms-BN content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-BN'})
MATCH (t:Taboo {key: 'alcohol-references@ms-BN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-UY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-UY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-UY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'military-dictatorship-1973-1985@es-UY'})
SET t.display_name = 'Military dictatorship (1973-1985)',
    t.locale = 'es-UY',
    t.term = 'Military dictatorship (1973-1985)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Military dictatorship (1973-1985) in es-UY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-UY'})
MATCH (t:Taboo {key: 'military-dictatorship-1973-1985@es-UY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'disappearances-and-human-rights-violations@es-UY'})
SET t.display_name = 'Disappearances and human rights violations',
    t.locale = 'es-UY',
    t.term = 'Disappearances and human rights violations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disappearances and human rights violations in es-UY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-UY'})
MATCH (t:Taboo {key: 'disappearances-and-human-rights-violations@es-UY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'assuming-uruguay-is-part-of-argentina@es-UY'})
SET t.display_name = 'Assuming Uruguay is part of Argentina',
    t.locale = 'es-UY',
    t.term = 'Assuming Uruguay is part of Argentina',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Assuming Uruguay is part of Argentina in es-UY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-UY'})
MATCH (t:Taboo {key: 'assuming-uruguay-is-part-of-argentina@es-UY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// zu-ZA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zu-ZA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zu-ZA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'apartheid-dismissal-or-minimization@zu-ZA'})
SET t.display_name = 'Apartheid dismissal or minimization',
    t.locale = 'zu-ZA',
    t.term = 'Apartheid dismissal or minimization',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Apartheid dismissal or minimization in zu-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zu-ZA'})
MATCH (t:Taboo {key: 'apartheid-dismissal-or-minimization@zu-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-stereotyping@zu-ZA'})
SET t.display_name = 'Ethnic stereotyping',
    t.locale = 'zu-ZA',
    t.term = 'Ethnic stereotyping',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic stereotyping in zu-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zu-ZA'})
MATCH (t:Taboo {key: 'ethnic-stereotyping@zu-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'witchcrafttraditional-medicine-mockery@zu-ZA'})
SET t.display_name = 'Witchcraft/traditional medicine mockery',
    t.locale = 'zu-ZA',
    t.term = 'Witchcraft/traditional medicine mockery',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Witchcraft/traditional medicine mockery in zu-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zu-ZA'})
MATCH (t:Taboo {key: 'witchcrafttraditional-medicine-mockery@zu-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-BF Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-BF'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BF'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnicisme-et-tribalisme@fr-BF'})
SET t.display_name = 'Ethnicisme et tribalisme',
    t.locale = 'fr-BF',
    t.term = 'Ethnicisme et tribalisme',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnicisme et tribalisme in fr-BF content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BF'})
MATCH (t:Taboo {key: 'ethnicisme-et-tribalisme@fr-BF'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'critique-de-thomas-sankara@fr-BF'})
SET t.display_name = 'Critique de Thomas Sankara',
    t.locale = 'fr-BF',
    t.term = 'Critique de Thomas Sankara',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Critique de Thomas Sankara in fr-BF content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BF'})
MATCH (t:Taboo {key: 'critique-de-thomas-sankara@fr-BF'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'situation-securitaire-actuelle@fr-BF'})
SET t.display_name = 'Situation securitaire actuelle',
    t.locale = 'fr-BF',
    t.term = 'Situation securitaire actuelle',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Situation securitaire actuelle in fr-BF content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BF'})
MATCH (t:Taboo {key: 'situation-securitaire-actuelle@fr-BF'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-SA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-SA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-SA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-islam@en-SA'})
SET t.display_name = 'Criticism of Islam',
    t.locale = 'en-SA',
    t.term = 'Criticism of Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Islam in en-SA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-SA'})
MATCH (t:Taboo {key: 'criticism-of-islam@en-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-royal-family-or-leadership@en-SA'})
SET t.display_name = 'Criticism of Royal Family or Leadership',
    t.locale = 'en-SA',
    t.term = 'Criticism of Royal Family or Leadership',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Royal Family or Leadership in en-SA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-SA'})
MATCH (t:Taboo {key: 'criticism-of-royal-family-or-leadership@en-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'atheism-or-anti-religious-content@en-SA'})
SET t.display_name = 'Atheism or anti-religious content',
    t.locale = 'en-SA',
    t.term = 'Atheism or anti-religious content',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Atheism or anti-religious content in en-SA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-SA'})
MATCH (t:Taboo {key: 'atheism-or-anti-religious-content@en-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// id-ID Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'id-ID'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@id-ID'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-any-religion@id-ID'})
SET t.display_name = 'Insulting Islam or any religion',
    t.locale = 'id-ID',
    t.term = 'Insulting Islam or any religion',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or any religion in id-ID content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@id-ID'})
MATCH (t:Taboo {key: 'insulting-islam-or-any-religion@id-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'separatism-and-territorial-issues@id-ID'})
SET t.display_name = 'Separatism and territorial issues',
    t.locale = 'id-ID',
    t.term = 'Separatism and territorial issues',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Separatism and territorial issues in id-ID content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@id-ID'})
MATCH (t:Taboo {key: 'separatism-and-territorial-issues@id-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'pkicommunism@id-ID'})
SET t.display_name = 'PKI/Communism',
    t.locale = 'id-ID',
    t.term = 'PKI/Communism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: PKI/Communism in id-ID content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@id-ID'})
MATCH (t:Taboo {key: 'pkicommunism@id-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-QA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-QA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-QA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-amir-or-ruling-family@ar-QA'})
SET t.display_name = 'Criticism of Amir or ruling family',
    t.locale = 'ar-QA',
    t.term = 'Criticism of Amir or ruling family',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Amir or ruling family in ar-QA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-QA'})
MATCH (t:Taboo {key: 'criticism-of-amir-or-ruling-family@ar-QA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insulting-islam-or-prophet@ar-QA'})
SET t.display_name = 'Insulting Islam or Prophet',
    t.locale = 'ar-QA',
    t.term = 'Insulting Islam or Prophet',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or Prophet in ar-QA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-QA'})
MATCH (t:Taboo {key: 'insulting-islam-or-prophet@ar-QA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'lgbtq-content@ar-QA'})
SET t.display_name = 'LGBTQ+ content',
    t.locale = 'ar-QA',
    t.term = 'LGBTQ+ content',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: LGBTQ+ content in ar-QA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-QA'})
MATCH (t:Taboo {key: 'lgbtq-content@ar-QA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-VE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-VE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-VE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'current-political-situation@es-VE'})
SET t.display_name = 'Current political situation',
    t.locale = 'es-VE',
    t.term = 'Current political situation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Current political situation in es-VE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-VE'})
MATCH (t:Taboo {key: 'current-political-situation@es-VE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'economic-crisishyperinflation@es-VE'})
SET t.display_name = 'Economic crisis/hyperinflation',
    t.locale = 'es-VE',
    t.term = 'Economic crisis/hyperinflation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Economic crisis/hyperinflation in es-VE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-VE'})
MATCH (t:Taboo {key: 'economic-crisishyperinflation@es-VE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'venezuelan-diasporaemigration@es-VE'})
SET t.display_name = 'Venezuelan diaspora/emigration',
    t.locale = 'es-VE',
    t.term = 'Venezuelan diaspora/emigration',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Venezuelan diaspora/emigration in es-VE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-VE'})
MATCH (t:Taboo {key: 'venezuelan-diasporaemigration@es-VE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// az-AZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'az-AZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@az-AZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'armenian-azerbaijani-conflict-pro-armenian-stance@az-AZ'})
SET t.display_name = 'Armenian-Azerbaijani conflict (pro-Armenian stance)',
    t.locale = 'az-AZ',
    t.term = 'Armenian-Azerbaijani conflict (pro-Armenian stance)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Armenian-Azerbaijani conflict (pro-Armenian stance) in az-AZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@az-AZ'})
MATCH (t:Taboo {key: 'armenian-azerbaijani-conflict-pro-armenian-stance@az-AZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-government-or-leadership@az-AZ'})
SET t.display_name = 'Criticism of government or leadership',
    t.locale = 'az-AZ',
    t.term = 'Criticism of government or leadership',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of government or leadership in az-AZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@az-AZ'})
MATCH (t:Taboo {key: 'criticism-of-government-or-leadership@az-AZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-mockery-or-disrespect@az-AZ'})
SET t.display_name = 'Religious mockery or disrespect',
    t.locale = 'az-AZ',
    t.term = 'Religious mockery or disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious mockery or disrespect in az-AZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@az-AZ'})
MATCH (t:Taboo {key: 'religious-mockery-or-disrespect@az-AZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// de-DE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'de-DE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-DE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'nazi-era-and-holocaust@de-DE'})
SET t.display_name = 'Nazi era and Holocaust',
    t.locale = 'de-DE',
    t.term = 'Nazi era and Holocaust',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Nazi era and Holocaust in de-DE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-DE'})
MATCH (t:Taboo {key: 'nazi-era-and-holocaust@de-DE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'swastika-and-nazi-symbols@de-DE'})
SET t.display_name = 'Swastika and Nazi symbols',
    t.locale = 'de-DE',
    t.term = 'Swastika and Nazi symbols',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Swastika and Nazi symbols in de-DE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-DE'})
MATCH (t:Taboo {key: 'swastika-and-nazi-symbols@de-DE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'comparing-anything-to-nazishitler@de-DE'})
SET t.display_name = 'Comparing anything to Nazis/Hitler',
    t.locale = 'de-DE',
    t.term = 'Comparing anything to Nazis/Hitler',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparing anything to Nazis/Hitler in de-DE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-DE'})
MATCH (t:Taboo {key: 'comparing-anything-to-nazishitler@de-DE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-SA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-SA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-SA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-islam@ar-SA'})
SET t.display_name = 'Criticism of Islam',
    t.locale = 'ar-SA',
    t.term = 'Criticism of Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Islam in ar-SA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-SA'})
MATCH (t:Taboo {key: 'criticism-of-islam@ar-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-royal-family@ar-SA'})
SET t.display_name = 'Criticism of Royal Family',
    t.locale = 'ar-SA',
    t.term = 'Criticism of Royal Family',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Royal Family in ar-SA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-SA'})
MATCH (t:Taboo {key: 'criticism-of-royal-family@ar-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'atheism@ar-SA'})
SET t.display_name = 'الإلحاد / Atheism',
    t.locale = 'ar-SA',
    t.term = 'الإلحاد / Atheism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: الإلحاد / Atheism in ar-SA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-SA'})
MATCH (t:Taboo {key: 'atheism@ar-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-MA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-MA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-MA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'souverainete-du-sahara-occidental@fr-MA'})
SET t.display_name = 'Souverainete du Sahara occidental',
    t.locale = 'fr-MA',
    t.term = 'Souverainete du Sahara occidental',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Souverainete du Sahara occidental in fr-MA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-MA'})
MATCH (t:Taboo {key: 'souverainete-du-sahara-occidental@fr-MA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'critique-de-la-monarchie@fr-MA'})
SET t.display_name = 'Critique de la monarchie',
    t.locale = 'fr-MA',
    t.term = 'Critique de la monarchie',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Critique de la monarchie in fr-MA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-MA'})
MATCH (t:Taboo {key: 'critique-de-la-monarchie@fr-MA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insulte-a-l@fr-MA'})
SET t.display_name = 'Insulte a l\\',
    t.locale = 'fr-MA',
    t.term = 'Insulte a l\\',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulte a l\\ in fr-MA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-MA'})
MATCH (t:Taboo {key: 'insulte-a-l@fr-MA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// uz-UZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'uz-UZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@uz-UZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-government-or-leadership@uz-UZ'})
SET t.display_name = 'Criticism of government or leadership',
    t.locale = 'uz-UZ',
    t.term = 'Criticism of government or leadership',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of government or leadership in uz-UZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@uz-UZ'})
MATCH (t:Taboo {key: 'criticism-of-government-or-leadership@uz-UZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-mockery-or-disrespect-to-islam@uz-UZ'})
SET t.display_name = 'Religious mockery or disrespect to Islam',
    t.locale = 'uz-UZ',
    t.term = 'Religious mockery or disrespect to Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious mockery or disrespect to Islam in uz-UZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@uz-UZ'})
MATCH (t:Taboo {key: 'religious-mockery-or-disrespect-to-islam@uz-UZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'territorial-disputes-or-regional-tensions@uz-UZ'})
SET t.display_name = 'Territorial disputes or regional tensions',
    t.locale = 'uz-UZ',
    t.term = 'Territorial disputes or regional tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Territorial disputes or regional tensions in uz-UZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@uz-UZ'})
MATCH (t:Taboo {key: 'territorial-disputes-or-regional-tensions@uz-UZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// lv-LV Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'lv-LV'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@lv-LV'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'soviet-nostalgia-or-symbolism@lv-LV'})
SET t.display_name = 'Soviet nostalgia or symbolism',
    t.locale = 'lv-LV',
    t.term = 'Soviet nostalgia or symbolism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Soviet nostalgia or symbolism in lv-LV content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@lv-LV'})
MATCH (t:Taboo {key: 'soviet-nostalgia-or-symbolism@lv-LV'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'comparing-to-russia-positively@lv-LV'})
SET t.display_name = 'Comparing to Russia positively',
    t.locale = 'lv-LV',
    t.term = 'Comparing to Russia positively',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparing to Russia positively in lv-LV content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@lv-LV'})
MATCH (t:Taboo {key: 'comparing-to-russia-positively@lv-LV'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'questioning-latvian-language-importance@lv-LV'})
SET t.display_name = 'Questioning Latvian language importance',
    t.locale = 'lv-LV',
    t.term = 'Questioning Latvian language importance',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Questioning Latvian language importance in lv-LV content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@lv-LV'})
MATCH (t:Taboo {key: 'questioning-latvian-language-importance@lv-LV'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// xh-ZA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'xh-ZA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@xh-ZA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'apartheid-nostalgia@xh-ZA'})
SET t.display_name = 'Apartheid nostalgia',
    t.locale = 'xh-ZA',
    t.term = 'Apartheid nostalgia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Apartheid nostalgia in xh-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@xh-ZA'})
MATCH (t:Taboo {key: 'apartheid-nostalgia@xh-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'racial-stereotypes@xh-ZA'})
SET t.display_name = 'Racial stereotypes',
    t.locale = 'xh-ZA',
    t.term = 'Racial stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racial stereotypes in xh-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@xh-ZA'})
MATCH (t:Taboo {key: 'racial-stereotypes@xh-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'initiation-ceremony-details@xh-ZA'})
SET t.display_name = 'Initiation ceremony details',
    t.locale = 'xh-ZA',
    t.term = 'Initiation ceremony details',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Initiation ceremony details in xh-ZA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@xh-ZA'})
MATCH (t:Taboo {key: 'initiation-ceremony-details@xh-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ro-MD Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ro-MD'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ro-MD'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'transnistria-conflict@ro-MD'})
SET t.display_name = 'Transnistria conflict',
    t.locale = 'ro-MD',
    t.term = 'Transnistria conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Transnistria conflict in ro-MD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ro-MD'})
MATCH (t:Taboo {key: 'transnistria-conflict@ro-MD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'moldova-vs-romania-identity@ro-MD'})
SET t.display_name = 'Moldova vs Romania identity',
    t.locale = 'ro-MD',
    t.term = 'Moldova vs Romania identity',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Moldova vs Romania identity in ro-MD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ro-MD'})
MATCH (t:Taboo {key: 'moldova-vs-romania-identity@ro-MD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'russian-influencepolitics@ro-MD'})
SET t.display_name = 'Russian influence/politics',
    t.locale = 'ro-MD',
    t.term = 'Russian influence/politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Russian influence/politics in ro-MD content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ro-MD'})
MATCH (t:Taboo {key: 'russian-influencepolitics@ro-MD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-NG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-NG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'ethnic-stereotyping@en-NG'})
SET t.display_name = 'Ethnic stereotyping',
    t.locale = 'en-NG',
    t.term = 'Ethnic stereotyping',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic stereotyping in en-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NG'})
MATCH (t:Taboo {key: 'ethnic-stereotyping@en-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-conflict@en-NG'})
SET t.display_name = 'Religious conflict',
    t.locale = 'en-NG',
    t.term = 'Religious conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious conflict in en-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NG'})
MATCH (t:Taboo {key: 'religious-conflict@en-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'biafracivil-war@en-NG'})
SET t.display_name = 'Biafra/Civil War',
    t.locale = 'en-NG',
    t.term = 'Biafra/Civil War',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Biafra/Civil War in en-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NG'})
MATCH (t:Taboo {key: 'biafracivil-war@en-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// it-IT Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'it-IT'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-IT'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'mafia-stereotypes@it-IT'})
SET t.display_name = 'Mafia stereotypes',
    t.locale = 'it-IT',
    t.term = 'Mafia stereotypes',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mafia stereotypes in it-IT content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-IT'})
MATCH (t:Taboo {key: 'mafia-stereotypes@it-IT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// no-NO Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'no-NO'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@no-NO'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'bragging-or-self-promotion@no-NO'})
SET t.display_name = 'Bragging or self-promotion',
    t.locale = 'no-NO',
    t.term = 'Bragging or self-promotion',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Bragging or self-promotion in no-NO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@no-NO'})
MATCH (t:Taboo {key: 'bragging-or-self-promotion@no-NO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'immigration-politics@no-NO'})
SET t.display_name = 'Immigration politics',
    t.locale = 'no-NO',
    t.term = 'Immigration politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Immigration politics in no-NO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@no-NO'})
MATCH (t:Taboo {key: 'immigration-politics@no-NO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'promising-more-than-delivering@no-NO'})
SET t.display_name = 'Promising more than delivering',
    t.locale = 'no-NO',
    t.term = 'Promising more than delivering',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Promising more than delivering in no-NO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@no-NO'})
MATCH (t:Taboo {key: 'promising-more-than-delivering@no-NO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// sk-SK Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sk-SK'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sk-SK'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'conflating-slovaks-with-czechs@sk-SK'})
SET t.display_name = 'Conflating Slovaks with Czechs',
    t.locale = 'sk-SK',
    t.term = 'Conflating Slovaks with Czechs',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Conflating Slovaks with Czechs in sk-SK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sk-SK'})
MATCH (t:Taboo {key: 'conflating-slovaks-with-czechs@sk-SK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'hungarian-minority-tensions@sk-SK'})
SET t.display_name = 'Hungarian minority tensions',
    t.locale = 'sk-SK',
    t.term = 'Hungarian minority tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Hungarian minority tensions in sk-SK content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@sk-SK'})
MATCH (t:Taboo {key: 'hungarian-minority-tensions@sk-SK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// gn-PY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'gn-PY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@gn-PY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'triple-alliance-war-1864-1870@gn-PY'})
SET t.display_name = 'Triple Alliance War (1864-1870)',
    t.locale = 'gn-PY',
    t.term = 'Triple Alliance War (1864-1870)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Triple Alliance War (1864-1870) in gn-PY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@gn-PY'})
MATCH (t:Taboo {key: 'triple-alliance-war-1864-1870@gn-PY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'indigenous-exploitation@gn-PY'})
SET t.display_name = 'Indigenous exploitation',
    t.locale = 'gn-PY',
    t.term = 'Indigenous exploitation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Indigenous exploitation in gn-PY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@gn-PY'})
MATCH (t:Taboo {key: 'indigenous-exploitation@gn-PY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'triple-alliance-war-1864-1870@gn-PY'})
SET t.display_name = 'Triple Alliance War (1864-1870)',
    t.locale = 'gn-PY',
    t.term = 'Triple Alliance War (1864-1870)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Triple Alliance War (1864-1870) in gn-PY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@gn-PY'})
MATCH (t:Taboo {key: 'triple-alliance-war-1864-1870@gn-PY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// uk-UA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'uk-UA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@uk-UA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'russia-and-russian-aggression@uk-UA'})
SET t.display_name = 'Russia and Russian aggression',
    t.locale = 'uk-UA',
    t.term = 'Russia and Russian aggression',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Russia and Russian aggression in uk-UA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@uk-UA'})
MATCH (t:Taboo {key: 'russia-and-russian-aggression@uk-UA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'soviet-nostalgia@uk-UA'})
SET t.display_name = 'Soviet nostalgia',
    t.locale = 'uk-UA',
    t.term = 'Soviet nostalgia',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Soviet nostalgia in uk-UA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@uk-UA'})
MATCH (t:Taboo {key: 'soviet-nostalgia@uk-UA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'holodomor-denial-or-minimization@uk-UA'})
SET t.display_name = 'Holodomor denial or minimization',
    t.locale = 'uk-UA',
    t.term = 'Holodomor denial or minimization',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Holodomor denial or minimization in uk-UA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@uk-UA'})
MATCH (t:Taboo {key: 'holodomor-denial-or-minimization@uk-UA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// pl-PL Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pl-PL'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pl-PL'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'nazi-occupation-and-holocaust@pl-PL'})
SET t.display_name = 'Nazi occupation and Holocaust',
    t.locale = 'pl-PL',
    t.term = 'Nazi occupation and Holocaust',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Nazi occupation and Holocaust in pl-PL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pl-PL'})
MATCH (t:Taboo {key: 'nazi-occupation-and-holocaust@pl-PL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'communist-era-glorification@pl-PL'})
SET t.display_name = 'Communist era glorification',
    t.locale = 'pl-PL',
    t.term = 'Communist era glorification',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Communist era glorification in pl-PL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pl-PL'})
MATCH (t:Taboo {key: 'communist-era-glorification@pl-PL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'russia-and-soviet-references@pl-PL'})
SET t.display_name = 'Russia and Soviet references',
    t.locale = 'pl-PL',
    t.term = 'Russia and Soviet references',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Russia and Soviet references in pl-PL content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@pl-PL'})
MATCH (t:Taboo {key: 'russia-and-soviet-references@pl-PL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// zh-SG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-SG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-SG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'racial-commentary@zh-SG'})
SET t.display_name = 'Racial commentary',
    t.locale = 'zh-SG',
    t.term = 'Racial commentary',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Racial commentary in zh-SG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-SG'})
MATCH (t:Taboo {key: 'racial-commentary@zh-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-criticism@zh-SG'})
SET t.display_name = 'Religious criticism',
    t.locale = 'zh-SG',
    t.term = 'Religious criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious criticism in zh-SG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-SG'})
MATCH (t:Taboo {key: 'religious-criticism@zh-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-criticism-of-government@zh-SG'})
SET t.display_name = 'Political criticism of government',
    t.locale = 'zh-SG',
    t.term = 'Political criticism of government',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political criticism of government in zh-SG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-SG'})
MATCH (t:Taboo {key: 'political-criticism-of-government@zh-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-CA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-CA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-CA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'indigenous-peoples@en-CA'})
SET t.display_name = 'Indigenous peoples\\',
    t.locale = 'en-CA',
    t.term = 'Indigenous peoples\\',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Indigenous peoples\\ in en-CA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-CA'})
MATCH (t:Taboo {key: 'indigenous-peoples@en-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-AE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-AE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-uae-leadership-or-ruling-families@en-AE'})
SET t.display_name = 'Criticism of UAE leadership or ruling families',
    t.locale = 'en-AE',
    t.term = 'Criticism of UAE leadership or ruling families',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of UAE leadership or ruling families in en-AE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AE'})
MATCH (t:Taboo {key: 'criticism-of-uae-leadership-or-ruling-families@en-AE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'disrespect-toward-islam@en-AE'})
SET t.display_name = 'Disrespect toward Islam',
    t.locale = 'en-AE',
    t.term = 'Disrespect toward Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespect toward Islam in en-AE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AE'})
MATCH (t:Taboo {key: 'disrespect-toward-islam@en-AE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'lgbtq-content@en-AE'})
SET t.display_name = 'LGBTQ+ content',
    t.locale = 'en-AE',
    t.term = 'LGBTQ+ content',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: LGBTQ+ content in en-AE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AE'})
MATCH (t:Taboo {key: 'lgbtq-content@en-AE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-CY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-CY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-CY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'cyprus-divisionoccupation@en-CY'})
SET t.display_name = 'Cyprus division/occupation',
    t.locale = 'en-CY',
    t.term = 'Cyprus division/occupation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Cyprus division/occupation in en-CY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-CY'})
MATCH (t:Taboo {key: 'cyprus-divisionoccupation@en-CY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'turkey-relations@en-CY'})
SET t.display_name = 'Turkey relations',
    t.locale = 'en-CY',
    t.term = 'Turkey relations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Turkey relations in en-CY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-CY'})
MATCH (t:Taboo {key: 'turkey-relations@en-CY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'northern-cyprustrnc@en-CY'})
SET t.display_name = 'Northern Cyprus/TRNC',
    t.locale = 'en-CY',
    t.term = 'Northern Cyprus/TRNC',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Northern Cyprus/TRNC in en-CY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-CY'})
MATCH (t:Taboo {key: 'northern-cyprustrnc@en-CY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ru-RU Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-RU'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-RU'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'political-commentary-or-criticism@ru-RU'})
SET t.display_name = 'Political commentary or criticism',
    t.locale = 'ru-RU',
    t.term = 'Political commentary or criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political commentary or criticism in ru-RU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-RU'})
MATCH (t:Taboo {key: 'political-commentary-or-criticism@ru-RU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'comparisons-with-other-countries-negative-framing@ru-RU'})
SET t.display_name = 'Comparisons with other countries (negative framing)',
    t.locale = 'ru-RU',
    t.term = 'Comparisons with other countries (negative framing)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Comparisons with other countries (negative framing) in ru-RU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-RU'})
MATCH (t:Taboo {key: 'comparisons-with-other-countries-negative-framing@ru-RU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'lgbt-themes-in-marketing@ru-RU'})
SET t.display_name = 'LGBT themes in marketing',
    t.locale = 'ru-RU',
    t.term = 'LGBT themes in marketing',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: LGBT themes in marketing in ru-RU content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-RU'})
MATCH (t:Taboo {key: 'lgbt-themes-in-marketing@ru-RU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// mi-NZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mi-NZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@mi-NZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'disrespecting-tapu-sacred-places-or-objects@mi-NZ'})
SET t.display_name = 'Disrespecting tapu (sacred) places or objects',
    t.locale = 'mi-NZ',
    t.term = 'Disrespecting tapu (sacred) places or objects',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting tapu (sacred) places or objects in mi-NZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mi-NZ'})
MATCH (t:Taboo {key: 'disrespecting-tapu-sacred-places-or-objects@mi-NZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'mocking-or-misusing-ta-moko@mi-NZ'})
SET t.display_name = 'Mocking or misusing ta moko',
    t.locale = 'mi-NZ',
    t.term = 'Mocking or misusing ta moko',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Mocking or misusing ta moko in mi-NZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mi-NZ'})
MATCH (t:Taboo {key: 'mocking-or-misusing-ta-moko@mi-NZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'disrespecting-kaumatua-or-elders@mi-NZ'})
SET t.display_name = 'Disrespecting kaumatua or elders',
    t.locale = 'mi-NZ',
    t.term = 'Disrespecting kaumatua or elders',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting kaumatua or elders in mi-NZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@mi-NZ'})
MATCH (t:Taboo {key: 'disrespecting-kaumatua-or-elders@mi-NZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-PE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-PE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'internal-armed-conflict-sendero-luminoso@es-PE'})
SET t.display_name = 'Internal armed conflict (Sendero Luminoso',
    t.locale = 'es-PE',
    t.term = 'Internal armed conflict (Sendero Luminoso',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Internal armed conflict (Sendero Luminoso in es-PE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PE'})
MATCH (t:Taboo {key: 'internal-armed-conflict-sendero-luminoso@es-PE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'mrta@es-PE'})
SET t.display_name = 'MRTA)',
    t.locale = 'es-PE',
    t.term = 'MRTA)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: MRTA) in es-PE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PE'})
MATCH (t:Taboo {key: 'mrta@es-PE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'fujimori-era-controversies@es-PE'})
SET t.display_name = 'Fujimori era controversies',
    t.locale = 'es-PE',
    t.term = 'Fujimori era controversies',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Fujimori era controversies in es-PE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PE'})
MATCH (t:Taboo {key: 'fujimori-era-controversies@es-PE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'internal-armed-conflict-1980-2000@es-PE'})
SET t.display_name = 'Internal Armed Conflict (1980-2000)',
    t.locale = 'es-PE',
    t.term = 'Internal Armed Conflict (1980-2000)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Internal Armed Conflict (1980-2000) in es-PE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PE'})
MATCH (t:Taboo {key: 'internal-armed-conflict-1980-2000@es-PE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// yo-NG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'yo-NG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@yo-NG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'religious-mockery@yo-NG'})
SET t.display_name = 'Religious mockery',
    t.locale = 'yo-NG',
    t.term = 'Religious mockery',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious mockery in yo-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@yo-NG'})
MATCH (t:Taboo {key: 'religious-mockery@yo-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-conflicts@yo-NG'})
SET t.display_name = 'Ethnic conflicts',
    t.locale = 'yo-NG',
    t.term = 'Ethnic conflicts',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic conflicts in yo-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@yo-NG'})
MATCH (t:Taboo {key: 'ethnic-conflicts@yo-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'traditional-rulers-disrespect@yo-NG'})
SET t.display_name = 'Traditional rulers disrespect',
    t.locale = 'yo-NG',
    t.term = 'Traditional rulers disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Traditional rulers disrespect in yo-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@yo-NG'})
MATCH (t:Taboo {key: 'traditional-rulers-disrespect@yo-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// tk-TM Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'tk-TM'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@tk-TM'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-president-or-government@tk-TM'})
SET t.display_name = 'Criticism of president or government',
    t.locale = 'tk-TM',
    t.term = 'Criticism of president or government',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of president or government in tk-TM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tk-TM'})
MATCH (t:Taboo {key: 'criticism-of-president-or-government@tk-TM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-commentary-of-any-kind@tk-TM'})
SET t.display_name = 'Political commentary of any kind',
    t.locale = 'tk-TM',
    t.term = 'Political commentary of any kind',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political commentary of any kind in tk-TM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tk-TM'})
MATCH (t:Taboo {key: 'political-commentary-of-any-kind@tk-TM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-mockery-or-disrespect@tk-TM'})
SET t.display_name = 'Religious mockery or disrespect',
    t.locale = 'tk-TM',
    t.term = 'Religious mockery or disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious mockery or disrespect in tk-TM content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tk-TM'})
MATCH (t:Taboo {key: 'religious-mockery-or-disrespect@tk-TM'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-PY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-PY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'triple-alliance-war-1864-1870@es-PY'})
SET t.display_name = 'Triple Alliance War (1864-1870)',
    t.locale = 'es-PY',
    t.term = 'Triple Alliance War (1864-1870)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Triple Alliance War (1864-1870) in es-PY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PY'})
MATCH (t:Taboo {key: 'triple-alliance-war-1864-1870@es-PY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'indigenous-exploitation@es-PY'})
SET t.display_name = 'Indigenous exploitation',
    t.locale = 'es-PY',
    t.term = 'Indigenous exploitation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Indigenous exploitation in es-PY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PY'})
MATCH (t:Taboo {key: 'indigenous-exploitation@es-PY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'triple-alliance-war-1864-1870@es-PY'})
SET t.display_name = 'Triple Alliance War (1864-1870)',
    t.locale = 'es-PY',
    t.term = 'Triple Alliance War (1864-1870)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Triple Alliance War (1864-1870) in es-PY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PY'})
MATCH (t:Taboo {key: 'triple-alliance-war-1864-1870@es-PY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ha-NG Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ha-NG'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ha-NG'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-the-prophet@ha-NG'})
SET t.display_name = 'Insulting Islam or the Prophet',
    t.locale = 'ha-NG',
    t.term = 'Insulting Islam or the Prophet',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or the Prophet in ha-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ha-NG'})
MATCH (t:Taboo {key: 'insulting-islam-or-the-prophet@ha-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'boko-haramterrorism@ha-NG'})
SET t.display_name = 'Boko Haram/terrorism',
    t.locale = 'ha-NG',
    t.term = 'Boko Haram/terrorism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Boko Haram/terrorism in ha-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ha-NG'})
MATCH (t:Taboo {key: 'boko-haramterrorism@ha-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnicreligious-conflict@ha-NG'})
SET t.display_name = 'Ethnic/religious conflict',
    t.locale = 'ha-NG',
    t.term = 'Ethnic/religious conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic/religious conflict in ha-NG content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ha-NG'})
MATCH (t:Taboo {key: 'ethnicreligious-conflict@ha-NG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// el-CY Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'el-CY'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-CY'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'cyprus-division1974@el-CY'})
SET t.display_name = 'Cyprus division/1974',
    t.locale = 'el-CY',
    t.term = 'Cyprus division/1974',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Cyprus division/1974 in el-CY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-CY'})
MATCH (t:Taboo {key: 'cyprus-division1974@el-CY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'greek-turkish-relations@el-CY'})
SET t.display_name = 'Greek-Turkish relations',
    t.locale = 'el-CY',
    t.term = 'Greek-Turkish relations',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Greek-Turkish relations in el-CY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-CY'})
MATCH (t:Taboo {key: 'greek-turkish-relations@el-CY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'recognition-of-trnc@el-CY'})
SET t.display_name = 'Recognition of "TRNC"',
    t.locale = 'el-CY',
    t.term = 'Recognition of "TRNC"',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Recognition of "TRNC" in el-CY content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-CY'})
MATCH (t:Taboo {key: 'recognition-of-trnc@el-CY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// jv-ID Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'jv-ID'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@jv-ID'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'disrespecting-the-keraton-royal-courts@jv-ID'})
SET t.display_name = 'Disrespecting the Keraton (Royal Courts)',
    t.locale = 'jv-ID',
    t.term = 'Disrespecting the Keraton (Royal Courts)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Disrespecting the Keraton (Royal Courts) in jv-ID content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@jv-ID'})
MATCH (t:Taboo {key: 'disrespecting-the-keraton-royal-courts@jv-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'causing-public-isin-shameembarrassment@jv-ID'})
SET t.display_name = 'Causing public isin (shame/embarrassment)',
    t.locale = 'jv-ID',
    t.term = 'Causing public isin (shame/embarrassment)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Causing public isin (shame/embarrassment) in jv-ID content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@jv-ID'})
MATCH (t:Taboo {key: 'causing-public-isin-shameembarrassment@jv-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'direct-criticism-or-confrontation@jv-ID'})
SET t.display_name = 'Direct criticism or confrontation',
    t.locale = 'jv-ID',
    t.term = 'Direct criticism or confrontation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Direct criticism or confrontation in jv-ID content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@jv-ID'})
MATCH (t:Taboo {key: 'direct-criticism-or-confrontation@jv-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// km-KH Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'km-KH'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@km-KH'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'khmer-rouge-era-1975-1979@km-KH'})
SET t.display_name = 'Khmer Rouge era (1975-1979)',
    t.locale = 'km-KH',
    t.term = 'Khmer Rouge era (1975-1979)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Khmer Rouge era (1975-1979) in km-KH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@km-KH'})
MATCH (t:Taboo {key: 'khmer-rouge-era-1975-1979@km-KH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'criticism-of-royal-family@km-KH'})
SET t.display_name = 'Criticism of Royal Family',
    t.locale = 'km-KH',
    t.term = 'Criticism of Royal Family',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of Royal Family in km-KH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@km-KH'})
MATCH (t:Taboo {key: 'criticism-of-royal-family@km-KH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'buddhist-disrespect@km-KH'})
SET t.display_name = 'Buddhist disrespect',
    t.locale = 'km-KH',
    t.term = 'Buddhist disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Buddhist disrespect in km-KH content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@km-KH'})
MATCH (t:Taboo {key: 'buddhist-disrespect@km-KH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// so-SO Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'so-SO'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@so-SO'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulting-islam-or-the-prophet@so-SO'})
SET t.display_name = 'Insulting Islam or the Prophet',
    t.locale = 'so-SO',
    t.term = 'Insulting Islam or the Prophet',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam or the Prophet in so-SO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@so-SO'})
MATCH (t:Taboo {key: 'insulting-islam-or-the-prophet@so-SO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'clan-politics-and-conflict@so-SO'})
SET t.display_name = 'Clan politics and conflict',
    t.locale = 'so-SO',
    t.term = 'Clan politics and conflict',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Clan politics and conflict in so-SO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@so-SO'})
MATCH (t:Taboo {key: 'clan-politics-and-conflict@so-SO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'civil-war-and-al-shabaab@so-SO'})
SET t.display_name = 'Civil war and Al-Shabaab',
    t.locale = 'so-SO',
    t.term = 'Civil war and Al-Shabaab',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Civil war and Al-Shabaab in so-SO content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@so-SO'})
MATCH (t:Taboo {key: 'civil-war-and-al-shabaab@so-SO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-AE Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-AE'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-AE'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'criticism-of-ruling-families@ar-AE'})
SET t.display_name = 'Criticism of ruling families',
    t.locale = 'ar-AE',
    t.term = 'Criticism of ruling families',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Criticism of ruling families in ar-AE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-AE'})
MATCH (t:Taboo {key: 'criticism-of-ruling-families@ar-AE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'insulting-islam@ar-AE'})
SET t.display_name = 'Insulting Islam',
    t.locale = 'ar-AE',
    t.term = 'Insulting Islam',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulting Islam in ar-AE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-AE'})
MATCH (t:Taboo {key: 'insulting-islam@ar-AE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'lgbtq-content@ar-AE'})
SET t.display_name = 'LGBTQ+ content',
    t.locale = 'ar-AE',
    t.term = 'LGBTQ+ content',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: LGBTQ+ content in ar-AE content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-AE'})
MATCH (t:Taboo {key: 'lgbtq-content@ar-AE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// en-NZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-NZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'maori-cultural-appropriation@en-NZ'})
SET t.display_name = 'Maori cultural appropriation',
    t.locale = 'en-NZ',
    t.term = 'Maori cultural appropriation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Maori cultural appropriation in en-NZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NZ'})
MATCH (t:Taboo {key: 'maori-cultural-appropriation@en-NZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'treaty-breaches-land-confiscation@en-NZ'})
SET t.display_name = 'Treaty breaches / Land confiscation',
    t.locale = 'en-NZ',
    t.term = 'Treaty breaches / Land confiscation',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Treaty breaches / Land confiscation in en-NZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NZ'})
MATCH (t:Taboo {key: 'treaty-breaches-land-confiscation@en-NZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'land-wars-raupatu-confiscation@en-NZ'})
SET t.display_name = 'Land Wars / Raupatu (confiscation)',
    t.locale = 'en-NZ',
    t.term = 'Land Wars / Raupatu (confiscation)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Land Wars / Raupatu (confiscation) in en-NZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NZ'})
MATCH (t:Taboo {key: 'land-wars-raupatu-confiscation@en-NZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ne-NP Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ne-NP'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ne-NP'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'caste-discrimination@ne-NP'})
SET t.display_name = 'Caste discrimination',
    t.locale = 'ne-NP',
    t.term = 'Caste discrimination',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Caste discrimination in ne-NP content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ne-NP'})
MATCH (t:Taboo {key: 'caste-discrimination@ne-NP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnic-tensions@ne-NP'})
SET t.display_name = 'Ethnic tensions',
    t.locale = 'ne-NP',
    t.term = 'Ethnic tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic tensions in ne-NP content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ne-NP'})
MATCH (t:Taboo {key: 'ethnic-tensions@ne-NP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'religious-disrespect@ne-NP'})
SET t.display_name = 'Religious disrespect',
    t.locale = 'ne-NP',
    t.term = 'Religious disrespect',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Religious disrespect in ne-NP content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ne-NP'})
MATCH (t:Taboo {key: 'religious-disrespect@ne-NP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// fr-DZ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-DZ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-DZ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'insulte-a-l@fr-DZ'})
SET t.display_name = 'Insulte a l\\',
    t.locale = 'fr-DZ',
    t.term = 'Insulte a l\\',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Insulte a l\\ in fr-DZ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-DZ'})
MATCH (t:Taboo {key: 'insulte-a-l@fr-DZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// ar-LB Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-LB'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-LB'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'sectarian-politics@ar-LB'})
SET t.display_name = 'Sectarian Politics',
    t.locale = 'ar-LB',
    t.term = 'Sectarian Politics',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Sectarian Politics in ar-LB content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-LB'})
MATCH (t:Taboo {key: 'sectarian-politics@ar-LB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'civil-war-1975-1990@ar-LB'})
SET t.display_name = 'Civil War (1975-1990)',
    t.locale = 'ar-LB',
    t.term = 'Civil War (1975-1990)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Civil War (1975-1990) in ar-LB content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-LB'})
MATCH (t:Taboo {key: 'civil-war-1975-1990@ar-LB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'hezbollahpolitical-parties@ar-LB'})
SET t.display_name = 'Hezbollah/Political Parties',
    t.locale = 'ar-LB',
    t.term = 'Hezbollah/Political Parties',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Hezbollah/Political Parties in ar-LB content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-LB'})
MATCH (t:Taboo {key: 'hezbollahpolitical-parties@ar-LB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// tg-TJ Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'tg-TJ'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@tg-TJ'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'civil-war-1992-1997@tg-TJ'})
SET t.display_name = 'Civil War (1992-1997)',
    t.locale = 'tg-TJ',
    t.term = 'Civil War (1992-1997)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Civil War (1992-1997) in tg-TJ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tg-TJ'})
MATCH (t:Taboo {key: 'civil-war-1992-1997@tg-TJ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'political-criticism@tg-TJ'})
SET t.display_name = 'Political criticism',
    t.locale = 'tg-TJ',
    t.term = 'Political criticism',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Political criticism in tg-TJ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tg-TJ'})
MATCH (t:Taboo {key: 'political-criticism@tg-TJ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'ethnicregional-tensions@tg-TJ'})
SET t.display_name = 'Ethnic/Regional tensions',
    t.locale = 'tg-TJ',
    t.term = 'Ethnic/Regional tensions',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Ethnic/Regional tensions in tg-TJ content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@tg-TJ'})
MATCH (t:Taboo {key: 'ethnicregional-tensions@tg-TJ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// ----------------------------------------------------------------------------
// es-PA Taboos
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-PA'})
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PA'})
MERGE (l)-[:HAS_TABOOS]->(ts);

MERGE (t:Taboo {key: 'us-invasion-of-1989-operation-just-cause@es-PA'})
SET t.display_name = 'US invasion of 1989 (Operation Just Cause)',
    t.locale = 'es-PA',
    t.term = 'US invasion of 1989 (Operation Just Cause)',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: US invasion of 1989 (Operation Just Cause) in es-PA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PA'})
MATCH (t:Taboo {key: 'us-invasion-of-1989-operation-just-cause@es-PA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'manuel-noriega-and-dictatorship-era@es-PA'})
SET t.display_name = 'Manuel Noriega and dictatorship era',
    t.locale = 'es-PA',
    t.term = 'Manuel Noriega and dictatorship era',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: Manuel Noriega and dictatorship era in es-PA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PA'})
MATCH (t:Taboo {key: 'manuel-noriega-and-dictatorship-era@es-PA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'us-invasion-december-1989@es-PA'})
SET t.display_name = 'US invasion December 1989',
    t.locale = 'es-PA',
    t.term = 'US invasion December 1989',
    t.type = 'topic',
    t.severity = 'critical',
    t.category = 'cultural',
    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',
    t.alternatives = [],
    t.llm_context = 'AVOID: US invasion December 1989 in es-PA content. This is a CRITICAL taboo that can cause serious issues.',
    t.provenance = 'ath-know-l10n',
    t.confidence = 0.95,
    t.created_at = datetime(),
    t.updated_at = datetime();

MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-PA'})
MATCH (t:Taboo {key: 'us-invasion-december-1989@es-PA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);
