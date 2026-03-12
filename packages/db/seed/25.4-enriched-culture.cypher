// ============================================================
// 25.4 - Enriched CultureRefs from Research (Task B)
// Generated: 2026-03-10T18:59:52.960Z
// ============================================================

// --- CultureRefs with full context ---
MERGE (cr:CultureRef {key: 'cultureref:individual-achievement@en-US'})
SET cr.locale = 'en-US',
    cr.text = 'Individual Achievement',
    cr.importance = 'high',
    cr.expression = 'Self-made success stories, pulling yourself up by your bootstraps',
    cr.marketing_angle = 'Emphasize personal empowerment and individual control over outcomes',
    cr.display_name = 'Individual Achievement',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-US. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Self-made success stories, pulling yourself up by your bootstraps',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-US'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:positive-thinking-culture@en-US'})
SET cr.locale = 'en-US',
    cr.text = 'Positive Thinking Culture',
    cr.importance = 'high',
    cr.expression = 'Can-do attitude, growth mindset, possibilities over limitations',
    cr.marketing_angle = 'Lead with benefits and opportunities, minimize problem-dwelling',
    cr.display_name = 'Positive Thinking Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-US. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Can-do attitude, growth mindset, possibilities over limitations',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-US'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:convenience-priority@en-US'})
SET cr.locale = 'en-US',
    cr.text = 'Convenience Priority',
    cr.importance = 'high',
    cr.expression = 'Time is money, instant gratification, on-demand everything',
    cr.marketing_angle = 'Highlight speed, ease of use, time savings prominently',
    cr.display_name = 'Convenience Priority',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-US. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Time is money, instant gratification, on-demand everything',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-US'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:super-bowl@en-US'})
SET cr.locale = 'en-US',
    cr.text = 'Super Bowl',
    cr.importance = 'high',
    cr.expression = 'Annual cultural phenomenon, advertising showcase',
    cr.marketing_angle = 'Major marketing moment, sports metaphors resonate',
    cr.display_name = 'Super Bowl',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-US. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Annual cultural phenomenon, advertising showcase',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-US'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:black-friday@en-US'})
SET cr.locale = 'en-US',
    cr.text = 'Black Friday',
    cr.importance = 'high',
    cr.expression = 'Post-Thanksgiving shopping frenzy, deal culture',
    cr.marketing_angle = 'Urgency and scarcity messaging highly effective',
    cr.display_name = 'Black Friday',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-US. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Post-Thanksgiving shopping frenzy, deal culture',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-US'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:british-understatement@en-GB'})
SET cr.locale = 'en-GB',
    cr.text = 'British Understatement',
    cr.importance = 'high',
    cr.expression = 'Not bad (meaning excellent), quite good (meaning very good)',
    cr.marketing_angle = 'Avoid hyperbole; let quality speak for itself',
    cr.display_name = 'British Understatement',
    cr.content = 'Not bad (meaning excellent), quite good (meaning very good)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-GB'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:self-deprecating-humour@en-GB'})
SET cr.locale = 'en-GB',
    cr.text = 'Self-Deprecating Humour',
    cr.importance = 'high',
    cr.expression = 'Brands that don\'t take themselves too seriously',
    cr.marketing_angle = 'Wit and gentle self-mockery build authenticity',
    cr.display_name = 'Self-Deprecating Humour',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-GB. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Brands that don\'t take themselves too seriously',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-GB'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:queue-culture@en-GB'})
SET cr.locale = 'en-GB',
    cr.text = 'Queue Culture',
    cr.importance = 'medium',
    cr.expression = 'Fairness, taking turns, orderly process',
    cr.marketing_angle = 'Emphasize fair dealing and transparent processes',
    cr.display_name = 'Queue Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-GB. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Fairness, taking turns, orderly process',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-GB'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:class-awareness@en-GB'})
SET cr.locale = 'en-GB',
    cr.text = 'Class Awareness',
    cr.importance = 'medium',
    cr.expression = 'Subtle social stratification, accent and education markers',
    cr.marketing_angle = 'Be aware of class-coded language; aim for accessible sophistication',
    cr.display_name = 'Class Awareness',
    cr.content = 'Subtle social stratification, accent and education markers',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-GB'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bank-holidays@en-GB'})
SET cr.locale = 'en-GB',
    cr.text = 'Bank Holidays',
    cr.importance = 'medium',
    cr.expression = 'May Day, Spring Bank Holiday, August Bank Holiday',
    cr.marketing_angle = 'Natural promotional moments for travel and leisure',
    cr.display_name = 'Bank Holidays',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-GB. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'May Day, Spring Bank Holiday, August Bank Holiday',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-GB'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:tall-poppy-syndrome@en-AU'})
SET cr.locale = 'en-AU',
    cr.text = 'Tall Poppy Syndrome',
    cr.importance = 'high',
    cr.expression = 'Cutting down those who stand out or show off',
    cr.marketing_angle = 'Avoid appearing superior; emphasize equality and mateship',
    cr.display_name = 'Tall Poppy Syndrome',
    cr.content = 'Cutting down those who stand out or show off',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-AU'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:mateship-culture@en-AU'})
SET cr.locale = 'en-AU',
    cr.text = 'Mateship Culture',
    cr.importance = 'high',
    cr.expression = 'Looking out for your mates, loyalty, helping each other',
    cr.marketing_angle = 'Peer recommendations and community aspects resonate strongly',
    cr.display_name = 'Mateship Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-AU. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Looking out for your mates, loyalty, helping each other',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-AU'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:larrikin-spirit@en-AU'})
SET cr.locale = 'en-AU',
    cr.text = 'Larrikin Spirit',
    cr.importance = 'high',
    cr.expression = 'Playful irreverence, not taking authority too seriously',
    cr.marketing_angle = 'Humor and irreverence work well; avoid stuffiness',
    cr.display_name = 'Larrikin Spirit',
    cr.content = 'Playful irreverence, not taking authority too seriously',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-AU'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:outdoor-culture@en-AU'})
SET cr.locale = 'en-AU',
    cr.text = 'Outdoor Culture',
    cr.importance = 'high',
    cr.expression = 'Beach, BBQ, active lifestyle, connection to nature',
    cr.marketing_angle = 'Lifestyle imagery resonates; health and outdoor activities',
    cr.display_name = 'Outdoor Culture',
    cr.content = 'Beach, BBQ, active lifestyle, connection to nature',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-AU'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:australia-day@en-AU'})
SET cr.locale = 'en-AU',
    cr.text = 'Australia Day',
    cr.importance = 'medium',
    cr.expression = 'National holiday, increasingly contested',
    cr.marketing_angle = 'Handle sensitively due to Indigenous perspectives',
    cr.display_name = 'Australia Day',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-AU. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'National holiday, increasingly contested',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-AU'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:multicultural-mosaic@en-CA'})
SET cr.locale = 'en-CA',
    cr.text = 'Multicultural Mosaic',
    cr.importance = 'high',
    cr.expression = 'Celebrating diversity rather than melting pot assimilation',
    cr.marketing_angle = 'Inclusive imagery and messaging; diverse representation',
    cr.display_name = 'Multicultural Mosaic',
    cr.content = 'Celebrating diversity rather than melting pot assimilation',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:canadian-politeness@en-CA'})
SET cr.locale = 'en-CA',
    cr.text = 'Canadian Politeness',
    cr.importance = 'high',
    cr.expression = 'Sorry culture, apologizing frequently, conflict avoidance',
    cr.marketing_angle = 'Polite tone, avoid aggressive or confrontational messaging',
    cr.display_name = 'Canadian Politeness',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-CA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Sorry culture, apologizing frequently, conflict avoidance',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bilingual-sensitivity@en-CA'})
SET cr.locale = 'en-CA',
    cr.text = 'Bilingual Sensitivity',
    cr.importance = 'high',
    cr.expression = 'French-English duality, official bilingualism',
    cr.marketing_angle = 'Consider bilingual options; respect linguistic duality',
    cr.display_name = 'Bilingual Sensitivity',
    cr.content = 'French-English duality, official bilingualism',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hockey-nation@en-CA'})
SET cr.locale = 'en-CA',
    cr.text = 'Hockey Nation',
    cr.importance = 'high',
    cr.expression = 'National sport, cultural touchstone',
    cr.marketing_angle = 'Hockey metaphors and references resonate widely',
    cr.display_name = 'Hockey Nation',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-CA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'National sport, cultural touchstone',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:canada-day@en-CA'})
SET cr.locale = 'en-CA',
    cr.text = 'Canada Day',
    cr.importance = 'high',
    cr.expression = 'National celebration, summer festivities',
    cr.marketing_angle = 'Patriotic promotions, red and white themes',
    cr.display_name = 'Canada Day',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-CA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'National celebration, summer festivities',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:family-centric-values@en-IN'})
SET cr.locale = 'en-IN',
    cr.text = 'Family-Centric Values',
    cr.importance = 'high',
    cr.expression = 'Extended family involvement, family approval matters',
    cr.marketing_angle = 'Family benefits and multi-generational appeal',
    cr.display_name = 'Family-Centric Values',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-IN. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Extended family involvement, family approval matters',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:aspirational-mobility@en-IN'})
SET cr.locale = 'en-IN',
    cr.text = 'Aspirational Mobility',
    cr.importance = 'high',
    cr.expression = 'Education as pathway, social mobility, progress narrative',
    cr.marketing_angle = 'Emphasize growth, advancement, and premium positioning',
    cr.display_name = 'Aspirational Mobility',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-IN. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Education as pathway, social mobility, progress narrative',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:diwali@en-IN'})
SET cr.locale = 'en-IN',
    cr.text = 'Diwali',
    cr.importance = 'high',
    cr.expression = 'Festival of lights, major shopping season',
    cr.marketing_angle = 'Peak promotional period, gifting themes',
    cr.display_name = 'Diwali',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-IN. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Festival of lights, major shopping season',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:cricket-culture@en-IN'})
SET cr.locale = 'en-IN',
    cr.text = 'Cricket Culture',
    cr.importance = 'high',
    cr.expression = 'National obsession, IPL, World Cup fever',
    cr.marketing_angle = 'Cricket metaphors and celebrity endorsements highly effective',
    cr.display_name = 'Cricket Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-IN. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'National obsession, IPL, World Cup fever',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:value-seeking@en-IN'})
SET cr.locale = 'en-IN',
    cr.text = 'Value Seeking',
    cr.importance = 'high',
    cr.expression = 'Price comparison, deal hunting, maximum value',
    cr.marketing_angle = 'Emphasize value proposition, savings, and bundled offers',
    cr.display_name = 'Value Seeking',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-IN. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Price comparison, deal hunting, maximum value',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kiasu-mentality@en-SG'})
SET cr.locale = 'en-SG',
    cr.text = 'Kiasu Mentality',
    cr.importance = 'high',
    cr.expression = 'Fear of missing out, wanting the best deal, competitive',
    cr.marketing_angle = 'Limited time offers and exclusive access drive action',
    cr.display_name = 'Kiasu Mentality',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-SG. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Fear of missing out, wanting the best deal, competitive',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-SG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:meritocratic-values@en-SG'})
SET cr.locale = 'en-SG',
    cr.text = 'Meritocratic Values',
    cr.importance = 'high',
    cr.expression = 'Hard work rewarded, education paramount, achievement focus',
    cr.marketing_angle = 'Quality and credentials matter; emphasize excellence',
    cr.display_name = 'Meritocratic Values',
    cr.content = 'Hard work rewarded, education paramount, achievement focus',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-SG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:racial-harmony@en-SG'})
SET cr.locale = 'en-SG',
    cr.text = 'Racial Harmony',
    cr.importance = 'high',
    cr.expression = 'Chinese, Malay, Indian, Others (CMIO) framework',
    cr.marketing_angle = 'Diverse representation expected; inclusive imagery',
    cr.display_name = 'Racial Harmony',
    cr.content = 'Chinese, Malay, Indian, Others (CMIO) framework',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-SG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:pragmatic-efficiency@en-SG'})
SET cr.locale = 'en-SG',
    cr.text = 'Pragmatic Efficiency',
    cr.importance = 'high',
    cr.expression = 'What works matters most, practical solutions',
    cr.marketing_angle = 'Focus on practical benefits and proven results',
    cr.display_name = 'Pragmatic Efficiency',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-SG. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'What works matters most, practical solutions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-SG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:national-day@en-SG'})
SET cr.locale = 'en-SG',
    cr.text = 'National Day',
    cr.importance = 'high',
    cr.expression = 'August 9th, strong patriotic sentiment',
    cr.marketing_angle = 'Red and white themes, national pride',
    cr.display_name = 'National Day',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-SG. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'August 9th, strong patriotic sentiment',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-SG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ubuntu-philosophy@en-ZA'})
SET cr.locale = 'en-ZA',
    cr.text = 'Ubuntu Philosophy',
    cr.importance = 'high',
    cr.expression = 'I am because we are; interconnectedness and community',
    cr.marketing_angle = 'Community benefits and collective impact resonate',
    cr.display_name = 'Ubuntu Philosophy',
    cr.content = 'I am because we are; interconnectedness and community',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-ZA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:rainbow-nation@en-ZA'})
SET cr.locale = 'en-ZA',
    cr.text = 'Rainbow Nation',
    cr.importance = 'high',
    cr.expression = 'Diversity as strength, post-apartheid unity',
    cr.marketing_angle = 'Diverse representation essential; unity themes',
    cr.display_name = 'Rainbow Nation',
    cr.content = 'Diversity as strength, post-apartheid unity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-ZA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:braai-culture@en-ZA'})
SET cr.locale = 'en-ZA',
    cr.text = 'Braai Culture',
    cr.importance = 'high',
    cr.expression = 'BBQ as social institution, Heritage Day as National Braai Day',
    cr.marketing_angle = 'Social gathering and lifestyle imagery',
    cr.display_name = 'Braai Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-ZA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'BBQ as social institution, Heritage Day as National Braai Day',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-ZA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:rugby-nation@en-ZA'})
SET cr.locale = 'en-ZA',
    cr.text = 'Rugby Nation',
    cr.importance = 'high',
    cr.expression = 'Springboks as unifying force, World Cup pride',
    cr.marketing_angle = 'Rugby metaphors, national team associations',
    cr.display_name = 'Rugby Nation',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-ZA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Springboks as unifying force, World Cup pride',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-ZA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:heritage-day@en-ZA'})
SET cr.locale = 'en-ZA',
    cr.text = 'Heritage Day',
    cr.importance = 'high',
    cr.expression = 'September 24, celebrating cultural diversity',
    cr.marketing_angle = 'Cultural celebration themes, braai marketing',
    cr.display_name = 'Heritage Day',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-ZA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'September 24, celebrating cultural diversity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-ZA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kiwi-humility@en-NZ'})
SET cr.locale = 'en-NZ',
    cr.text = 'Kiwi Humility',
    cr.importance = 'high',
    cr.expression = 'Tall poppy syndrome even stronger than Australia; modesty valued',
    cr.marketing_angle = 'Understated excellence, avoid boastfulness',
    cr.display_name = 'Kiwi Humility',
    cr.content = 'Tall poppy syndrome even stronger than Australia; modesty valued',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-NZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:maori-cultural-integration@en-NZ'})
SET cr.locale = 'en-NZ',
    cr.text = 'Maori Cultural Integration',
    cr.importance = 'high',
    cr.expression = 'Te Reo, tikanga, Treaty of Waitangi awareness',
    cr.marketing_angle = 'Bicultural awareness expected; Maori greetings accepted',
    cr.display_name = 'Maori Cultural Integration',
    cr.content = 'Te Reo, tikanga, Treaty of Waitangi awareness',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-NZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:clean-green-image@en-NZ'})
SET cr.locale = 'en-NZ',
    cr.text = 'Clean Green Image',
    cr.importance = 'high',
    cr.expression = 'Environmental consciousness, sustainability, nature connection',
    cr.marketing_angle = 'Sustainability messaging resonates strongly',
    cr.display_name = 'Clean Green Image',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-NZ. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Environmental consciousness, sustainability, nature connection',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-NZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:all-blacks-pride@en-NZ'})
SET cr.locale = 'en-NZ',
    cr.text = 'All Blacks Pride',
    cr.importance = 'high',
    cr.expression = 'Rugby as national identity, haka, team excellence',
    cr.marketing_angle = 'Excellence and teamwork themes; rugby references',
    cr.display_name = 'All Blacks Pride',
    cr.content = 'Rugby as national identity, haka, team excellence',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-NZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:waitangi-day@en-NZ'})
SET cr.locale = 'en-NZ',
    cr.text = 'Waitangi Day',
    cr.importance = 'high',
    cr.expression = 'February 6, Treaty commemoration, national reflection',
    cr.marketing_angle = 'Handled sensitively; focus on unity',
    cr.display_name = 'Waitangi Day',
    cr.content = 'February 6, Treaty commemoration, national reflection',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-NZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:craic-culture@en-IE'})
SET cr.locale = 'en-IE',
    cr.text = 'Craic Culture',
    cr.importance = 'high',
    cr.expression = 'Fun, good conversation, social enjoyment',
    cr.marketing_angle = 'Warmth and sociability; avoid sterile messaging',
    cr.display_name = 'Craic Culture',
    cr.content = 'Fun, good conversation, social enjoyment',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:storytelling-tradition@en-IE'})
SET cr.locale = 'en-IE',
    cr.text = 'Storytelling Tradition',
    cr.importance = 'high',
    cr.expression = 'Narrative heritage, literary culture, gift of the gab',
    cr.marketing_angle = 'Story-driven content resonates; narrative over bullet points',
    cr.display_name = 'Storytelling Tradition',
    cr.content = 'Narrative heritage, literary culture, gift of the gab',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:local-community-focus@en-IE'})
SET cr.locale = 'en-IE',
    cr.text = 'Local Community Focus',
    cr.importance = 'high',
    cr.expression = 'Parish, GAA club, local identity strong',
    cr.marketing_angle = 'Local references and community connection',
    cr.display_name = 'Local Community Focus',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-IE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Parish, GAA club, local identity strong',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:st-patrick-s-day@en-IE'})
SET cr.locale = 'en-IE',
    cr.text = 'St. Patrick\'s Day',
    cr.importance = 'high',
    cr.expression = 'National celebration, global Irish identity',
    cr.marketing_angle = 'Major promotional moment; avoid leprechaun cliches',
    cr.display_name = 'St. Patrick\'s Day',
    cr.content = 'National celebration, global Irish identity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:gaa-sports@en-IE'})
SET cr.locale = 'en-IE',
    cr.text = 'GAA Sports',
    cr.importance = 'high',
    cr.expression = 'Hurling and Gaelic football, county pride',
    cr.marketing_angle = 'GAA references resonate locally; county rivalries',
    cr.display_name = 'GAA Sports',
    cr.content = 'Hurling and Gaelic football, county pride',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-IE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bayanihan-spirit@en-PH'})
SET cr.locale = 'en-PH',
    cr.text = 'Bayanihan Spirit',
    cr.importance = 'high',
    cr.expression = 'Community cooperation, helping neighbors, collective effort',
    cr.marketing_angle = 'Community and family themes resonate strongly',
    cr.display_name = 'Bayanihan Spirit',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-PH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Community cooperation, helping neighbors, collective effort',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:pakikisama@en-PH'})
SET cr.locale = 'en-PH',
    cr.text = 'Pakikisama',
    cr.importance = 'high',
    cr.expression = 'Smooth interpersonal relationships, group harmony',
    cr.marketing_angle = 'Social proof and peer approval important',
    cr.display_name = 'Pakikisama',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-PH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Smooth interpersonal relationships, group harmony',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:pinoy-pride@en-PH'})
SET cr.locale = 'en-PH',
    cr.text = 'Pinoy Pride',
    cr.importance = 'high',
    cr.expression = 'Pride in Filipino achievements globally',
    cr.marketing_angle = 'Filipino success stories, international recognition',
    cr.display_name = 'Pinoy Pride',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-PH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Pride in Filipino achievements globally',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:fiesta-culture@en-PH'})
SET cr.locale = 'en-PH',
    cr.text = 'Fiesta Culture',
    cr.importance = 'high',
    cr.expression = 'Town fiestas, celebrations, hospitality',
    cr.marketing_angle = 'Celebration and festive themes work well',
    cr.display_name = 'Fiesta Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-PH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Town fiestas, celebrations, hospitality',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:extended-christmas@en-PH'})
SET cr.locale = 'en-PH',
    cr.text = 'Extended Christmas',
    cr.importance = 'high',
    cr.expression = 'Ber months (September-December), longest Christmas season',
    cr.marketing_angle = 'Christmas marketing starts September',
    cr.display_name = 'Extended Christmas',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for en-PH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Ber months (September-December), longest Christmas season',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@en-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:gruendlichkeit-thoroughness@de-DE'})
SET cr.locale = 'de-DE',
    cr.text = 'Gruendlichkeit (Thoroughness)',
    cr.importance = 'high',
    cr.expression = 'Attention to detail, quality engineering, doing things properly',
    cr.marketing_angle = 'Detailed specifications, quality certifications, engineering excellence',
    cr.display_name = 'Gruendlichkeit (Thoroughness)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-DE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Attention to detail, quality engineering, doing things properly',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-DE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:datenschutz-data-protection@de-DE'})
SET cr.locale = 'de-DE',
    cr.text = 'Datenschutz (Data Protection)',
    cr.importance = 'high',
    cr.expression = 'Privacy paramount, GDPR birthplace, skepticism of data collection',
    cr.marketing_angle = 'Privacy-first messaging, GDPR compliance prominent',
    cr.display_name = 'Datenschutz (Data Protection)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-DE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Privacy paramount, GDPR birthplace, skepticism of data collection',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-DE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:mittelstand-pride@de-DE'})
SET cr.locale = 'de-DE',
    cr.text = 'Mittelstand Pride',
    cr.importance = 'high',
    cr.expression = 'Family-owned businesses, hidden champions, long-term thinking',
    cr.marketing_angle = 'Heritage, expertise, generational knowledge',
    cr.display_name = 'Mittelstand Pride',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-DE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Family-owned businesses, hidden champions, long-term thinking',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-DE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:environmental-consciousness@de-DE'})
SET cr.locale = 'de-DE',
    cr.text = 'Environmental Consciousness',
    cr.importance = 'high',
    cr.expression = 'Green party origins, recycling culture, renewable energy',
    cr.marketing_angle = 'Sustainability credentials essential; verified claims',
    cr.display_name = 'Environmental Consciousness',
    cr.content = 'Green party origins, recycling culture, renewable energy',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-DE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:oktoberfest@de-DE'})
SET cr.locale = 'de-DE',
    cr.text = 'Oktoberfest',
    cr.importance = 'medium',
    cr.expression = 'Bavarian tradition, tourism draw',
    cr.marketing_angle = 'Regional identity; careful with stereotypes',
    cr.display_name = 'Oktoberfest',
    cr.content = 'Bavarian tradition, tourism draw',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-DE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:gemuetlichkeit@de-AT'})
SET cr.locale = 'de-AT',
    cr.text = 'Gemuetlichkeit',
    cr.importance = 'high',
    cr.expression = 'Coziness, comfort, unhurried enjoyment of life',
    cr.marketing_angle = 'Quality of life, comfort, relaxed excellence',
    cr.display_name = 'Gemuetlichkeit',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-AT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Coziness, comfort, unhurried enjoyment of life',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-AT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kaffeehauskultur@de-AT'})
SET cr.locale = 'de-AT',
    cr.text = 'Kaffeehauskultur',
    cr.importance = 'high',
    cr.expression = 'Coffee house tradition, intellectual discourse, taking time',
    cr.marketing_angle = 'Sophistication, tradition, refined experiences',
    cr.display_name = 'Kaffeehauskultur',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-AT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Coffee house tradition, intellectual discourse, taking time',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-AT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:title-culture@de-AT'})
SET cr.locale = 'de-AT',
    cr.text = 'Title Culture',
    cr.importance = 'high',
    cr.expression = 'Academic and professional titles highly valued (Herr Magister, Frau Doktor)',
    cr.marketing_angle = 'Credentials and expertise prominent; respect hierarchy',
    cr.display_name = 'Title Culture',
    cr.content = 'Academic and professional titles highly valued (Herr Magister, Frau Doktor)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-AT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:habsburg-heritage@de-AT'})
SET cr.locale = 'de-AT',
    cr.text = 'Habsburg Heritage',
    cr.importance = 'medium',
    cr.expression = 'Imperial legacy, classical music, architecture',
    cr.marketing_angle = 'Tradition and heritage; premium positioning',
    cr.display_name = 'Habsburg Heritage',
    cr.content = 'Imperial legacy, classical music, architecture',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-AT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ski-culture@de-AT'})
SET cr.locale = 'de-AT',
    cr.text = 'Ski Culture',
    cr.importance = 'high',
    cr.expression = 'Alpine skiing national passion, winter sports excellence',
    cr.marketing_angle = 'Winter sports, alpine imagery, outdoor lifestyle',
    cr.display_name = 'Ski Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-AT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Alpine skiing national passion, winter sports excellence',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-AT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:swiss-precision@de-CH'})
SET cr.locale = 'de-CH',
    cr.text = 'Swiss Precision',
    cr.importance = 'high',
    cr.expression = 'Watches, banking, engineering precision, quality obsession',
    cr.marketing_angle = 'Premium quality, precision, reliability',
    cr.display_name = 'Swiss Precision',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Watches, banking, engineering precision, quality obsession',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:political-neutrality@de-CH'})
SET cr.locale = 'de-CH',
    cr.text = 'Political Neutrality',
    cr.importance = 'high',
    cr.expression = 'Non-alignment, diplomatic tradition, balanced perspective',
    cr.marketing_angle = 'Balanced, trustworthy, neutral positioning',
    cr.display_name = 'Political Neutrality',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Non-alignment, diplomatic tradition, balanced perspective',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:direct-democracy@de-CH'})
SET cr.locale = 'de-CH',
    cr.text = 'Direct Democracy',
    cr.importance = 'high',
    cr.expression = 'Referendums, citizen involvement, consensus',
    cr.marketing_angle = 'Customer voice matters; transparent processes',
    cr.display_name = 'Direct Democracy',
    cr.content = 'Referendums, citizen involvement, consensus',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:swiss-discretion@de-CH'})
SET cr.locale = 'de-CH',
    cr.text = 'Swiss Discretion',
    cr.importance = 'high',
    cr.expression = 'Privacy, confidentiality, understated wealth',
    cr.marketing_angle = 'Privacy-focused, discreet premium, no flashiness',
    cr.display_name = 'Swiss Discretion',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Privacy, confidentiality, understated wealth',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bundesfeier-national-day@de-CH'})
SET cr.locale = 'de-CH',
    cr.text = 'Bundesfeier (National Day)',
    cr.importance = 'medium',
    cr.expression = 'August 1, modest celebration, community bonfires',
    cr.marketing_angle = 'Patriotic but understated themes',
    cr.display_name = 'Bundesfeier (National Day)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for de-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'August 1, modest celebration, community bonfires',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@de-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:dutch-directness@nl-NL'})
SET cr.locale = 'nl-NL',
    cr.text = 'Dutch Directness',
    cr.importance = 'high',
    cr.expression = 'Saying what you mean, no hidden meanings, honest feedback',
    cr.marketing_angle = 'Clear, honest messaging; no excessive persuasion',
    cr.display_name = 'Dutch Directness',
    cr.content = 'Saying what you mean, no hidden meanings, honest feedback',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-NL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:nuchterheid-sobriety@nl-NL'})
SET cr.locale = 'nl-NL',
    cr.text = 'Nuchterheid (Sobriety)',
    cr.importance = 'high',
    cr.expression = 'Down-to-earth, practical, anti-pretentiousness',
    cr.marketing_angle = 'Value and practicality over luxury positioning',
    cr.display_name = 'Nuchterheid (Sobriety)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for nl-NL. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Down-to-earth, practical, anti-pretentiousness',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-NL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:poldermodel-consensus@nl-NL'})
SET cr.locale = 'nl-NL',
    cr.text = 'Poldermodel (Consensus)',
    cr.importance = 'high',
    cr.expression = 'Collaboration, compromise, stakeholder involvement',
    cr.marketing_angle = 'Collaborative benefits, win-win propositions',
    cr.display_name = 'Poldermodel (Consensus)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for nl-NL. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Collaboration, compromise, stakeholder involvement',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-NL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bicycle-culture@nl-NL'})
SET cr.locale = 'nl-NL',
    cr.text = 'Bicycle Culture',
    cr.importance = 'high',
    cr.expression = 'Cycling as lifestyle, infrastructure pride, sustainability',
    cr.marketing_angle = 'Sustainability, practical solutions, Dutch design',
    cr.display_name = 'Bicycle Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for nl-NL. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Cycling as lifestyle, infrastructure pride, sustainability',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-NL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:koningsdag-king-s-day@nl-NL'})
SET cr.locale = 'nl-NL',
    cr.text = 'Koningsdag (King\'s Day)',
    cr.importance = 'high',
    cr.expression = 'April 27, orange everything, street markets, celebration',
    cr.marketing_angle = 'Orange themes, celebration, vrijmarkt spirit',
    cr.display_name = 'Koningsdag (King\'s Day)',
    cr.content = 'April 27, orange everything, street markets, celebration',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-NL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:flemish-identity@nl-BE'})
SET cr.locale = 'nl-BE',
    cr.text = 'Flemish Identity',
    cr.importance = 'high',
    cr.expression = 'Distinct from both Netherlands and Wallonia; regional pride',
    cr.marketing_angle = 'Local Flemish references; not Dutch, not French',
    cr.display_name = 'Flemish Identity',
    cr.content = 'Distinct from both Netherlands and Wallonia; regional pride',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-BE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:flemish-modesty@nl-BE'})
SET cr.locale = 'nl-BE',
    cr.text = 'Flemish Modesty',
    cr.importance = 'high',
    cr.expression = 'Less direct than Dutch, more reserved, modest self-presentation',
    cr.marketing_angle = 'Quality speaks for itself; understated messaging',
    cr.display_name = 'Flemish Modesty',
    cr.content = 'Less direct than Dutch, more reserved, modest self-presentation',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-BE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:gezelligheid@nl-BE'})
SET cr.locale = 'nl-BE',
    cr.text = 'Gezelligheid',
    cr.importance = 'high',
    cr.expression = 'Cozy togetherness, cafe culture, social warmth',
    cr.marketing_angle = 'Social and warm imagery; togetherness',
    cr.display_name = 'Gezelligheid',
    cr.content = 'Cozy togetherness, cafe culture, social warmth',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-BE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:comic-strip-heritage@nl-BE'})
SET cr.locale = 'nl-BE',
    cr.text = 'Comic Strip Heritage',
    cr.importance = 'medium',
    cr.expression = 'Tintin, Smurfs, comic murals, BD tradition',
    cr.marketing_angle = 'Visual storytelling, playful graphics acceptable',
    cr.display_name = 'Comic Strip Heritage',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for nl-BE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Tintin, Smurfs, comic murals, BD tradition',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-BE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:flemish-community-day@nl-BE'})
SET cr.locale = 'nl-BE',
    cr.text = 'Flemish Community Day',
    cr.importance = 'medium',
    cr.expression = 'July 11, Battle of the Golden Spurs commemoration',
    cr.marketing_angle = 'Flemish pride, regional identity',
    cr.display_name = 'Flemish Community Day',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for nl-BE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'July 11, Battle of the Golden Spurs commemoration',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@nl-BE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:lagom@sv-SE'})
SET cr.locale = 'sv-SE',
    cr.text = 'Lagom',
    cr.importance = 'high',
    cr.expression = 'Just right, not too much, not too little, balanced moderation',
    cr.marketing_angle = 'Balanced solutions, no excess, sustainable choices',
    cr.display_name = 'Lagom',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for sv-SE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Just right, not too much, not too little, balanced moderation',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sv-SE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:jantelagen-law-of-jante@sv-SE'})
SET cr.locale = 'sv-SE',
    cr.text = 'Jantelagen (Law of Jante)',
    cr.importance = 'high',
    cr.expression = 'Don\'t think you\'re special, collective over individual',
    cr.marketing_angle = 'Community benefits, avoid elitism, inclusive messaging',
    cr.display_name = 'Jantelagen (Law of Jante)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for sv-SE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Don\'t think you\'re special, collective over individual',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sv-SE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:fika-culture@sv-SE'})
SET cr.locale = 'sv-SE',
    cr.text = 'Fika Culture',
    cr.importance = 'high',
    cr.expression = 'Coffee break as social institution, work-life balance',
    cr.marketing_angle = 'Quality of life, social connection, breaks valued',
    cr.display_name = 'Fika Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for sv-SE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Coffee break as social institution, work-life balance',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sv-SE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:sustainability-leadership@sv-SE'})
SET cr.locale = 'sv-SE',
    cr.text = 'Sustainability Leadership',
    cr.importance = 'high',
    cr.expression = 'Environmental consciousness, recycling, green energy',
    cr.marketing_angle = 'Sustainability essential; verified environmental claims',
    cr.display_name = 'Sustainability Leadership',
    cr.content = 'Environmental consciousness, recycling, green energy',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sv-SE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:midsommar@sv-SE'})
SET cr.locale = 'sv-SE',
    cr.text = 'Midsommar',
    cr.importance = 'high',
    cr.expression = 'Summer solstice celebration, maypole, traditional',
    cr.marketing_angle = 'Summer themes, tradition, togetherness',
    cr.display_name = 'Midsommar',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for sv-SE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Summer solstice celebration, maypole, traditional',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sv-SE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hygge@da-DK'})
SET cr.locale = 'da-DK',
    cr.text = 'Hygge',
    cr.importance = 'high',
    cr.expression = 'Cozy contentment, candles, comfort, togetherness',
    cr.marketing_angle = 'Comfort, warmth, quality experiences over things',
    cr.display_name = 'Hygge',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for da-DK. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Cozy contentment, candles, comfort, togetherness',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@da-DK'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:janteloven@da-DK'})
SET cr.locale = 'da-DK',
    cr.text = 'Janteloven',
    cr.importance = 'high',
    cr.expression = 'Danish origin of Jante Law; collective modesty',
    cr.marketing_angle = 'Inclusive, not elite, community focus',
    cr.display_name = 'Janteloven',
    cr.content = 'Danish origin of Jante Law; collective modesty',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@da-DK'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:danish-design@da-DK'})
SET cr.locale = 'da-DK',
    cr.text = 'Danish Design',
    cr.importance = 'high',
    cr.expression = 'Minimalism, functionality, aesthetic simplicity',
    cr.marketing_angle = 'Clean design, functionality, aesthetic quality',
    cr.display_name = 'Danish Design',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for da-DK. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Minimalism, functionality, aesthetic simplicity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@da-DK'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:work-life-balance@da-DK'})
SET cr.locale = 'da-DK',
    cr.text = 'Work-Life Balance',
    cr.importance = 'high',
    cr.expression = 'Leaving work on time, family priority, flexibility',
    cr.marketing_angle = 'Time-saving, efficiency, life quality',
    cr.display_name = 'Work-Life Balance',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for da-DK. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Leaving work on time, family priority, flexibility',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@da-DK'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:sankt-hans-aften@da-DK'})
SET cr.locale = 'da-DK',
    cr.text = 'Sankt Hans Aften',
    cr.importance = 'high',
    cr.expression = 'Midsummer bonfire, June 23, community gathering',
    cr.marketing_angle = 'Summer, community, tradition',
    cr.display_name = 'Sankt Hans Aften',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for da-DK. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Midsummer bonfire, June 23, community gathering',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@da-DK'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:friluftsliv@no-NO'})
SET cr.locale = 'no-NO',
    cr.text = 'Friluftsliv',
    cr.importance = 'high',
    cr.expression = 'Outdoor life, nature connection, skiing, hiking',
    cr.marketing_angle = 'Nature imagery, outdoor lifestyle, sustainability',
    cr.display_name = 'Friluftsliv',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for no-NO. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Outdoor life, nature connection, skiing, hiking',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@no-NO'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:norwegian-egalitarianism@no-NO'})
SET cr.locale = 'no-NO',
    cr.text = 'Norwegian Egalitarianism',
    cr.importance = 'high',
    cr.expression = 'Flat hierarchy, everyone equal, oil wealth shared',
    cr.marketing_angle = 'Accessible to all, no elitism, democratic design',
    cr.display_name = 'Norwegian Egalitarianism',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for no-NO. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Flat hierarchy, everyone equal, oil wealth shared',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@no-NO'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kos@no-NO'})
SET cr.locale = 'no-NO',
    cr.text = 'Kos',
    cr.importance = 'high',
    cr.expression = 'Norwegian coziness, similar to hygge',
    cr.marketing_angle = 'Comfort, quality time, simple pleasures',
    cr.display_name = 'Kos',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for no-NO. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Norwegian coziness, similar to hygge',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@no-NO'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:dugnad@no-NO'})
SET cr.locale = 'no-NO',
    cr.text = 'Dugnad',
    cr.importance = 'high',
    cr.expression = 'Community volunteer work, collective effort',
    cr.marketing_angle = 'Community involvement, collective good',
    cr.display_name = 'Dugnad',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for no-NO. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Community volunteer work, collective effort',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@no-NO'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:17-mai-constitution-day@no-NO'})
SET cr.locale = 'no-NO',
    cr.text = '17. Mai (Constitution Day)',
    cr.importance = 'high',
    cr.expression = 'National Day, children\'s parades, bunads',
    cr.marketing_angle = 'National pride, celebration, tradition',
    cr.display_name = '17. Mai (Constitution Day)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for no-NO. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'National Day, children\'s parades, bunads',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@no-NO'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:sisu@fi-FI'})
SET cr.locale = 'fi-FI',
    cr.text = 'Sisu',
    cr.importance = 'high',
    cr.expression = 'Inner strength, resilience, determination in adversity',
    cr.marketing_angle = 'Reliability, perseverance, trusted performance',
    cr.display_name = 'Sisu',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fi-FI. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Inner strength, resilience, determination in adversity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fi-FI'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:sauna-culture@fi-FI'})
SET cr.locale = 'fi-FI',
    cr.text = 'Sauna Culture',
    cr.importance = 'high',
    cr.expression = 'Sauna as social institution, wellness, authenticity',
    cr.marketing_angle = 'Wellness, authenticity, Finnish heritage',
    cr.display_name = 'Sauna Culture',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fi-FI. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Sauna as social institution, wellness, authenticity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fi-FI'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:comfortable-silence@fi-FI'})
SET cr.locale = 'fi-FI',
    cr.text = 'Comfortable Silence',
    cr.importance = 'high',
    cr.expression = 'Silence is not awkward; talking only when meaningful',
    cr.marketing_angle = 'Substance over fluff; less is more',
    cr.display_name = 'Comfortable Silence',
    cr.content = 'Silence is not awkward; talking only when meaningful',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fi-FI'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:education-excellence@fi-FI'})
SET cr.locale = 'fi-FI',
    cr.text = 'Education Excellence',
    cr.importance = 'high',
    cr.expression = 'PISA rankings, teacher respect, learning culture',
    cr.marketing_angle = 'Quality, research-backed, expertise',
    cr.display_name = 'Education Excellence',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fi-FI. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'PISA rankings, teacher respect, learning culture',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fi-FI'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:juhannus-midsummer@fi-FI'})
SET cr.locale = 'fi-FI',
    cr.text = 'Juhannus (Midsummer)',
    cr.importance = 'high',
    cr.expression = 'Midsummer celebration, countryside retreat, bonfires',
    cr.marketing_angle = 'Summer, nature, traditional celebrations',
    cr.display_name = 'Juhannus (Midsummer)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fi-FI. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Midsummer celebration, countryside retreat, bonfires',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fi-FI'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:mianzi-face@zh-CN'})
SET cr.locale = 'zh-CN',
    cr.text = '面子 (Miànzi - Face)',
    cr.importance = 'critical',
    cr.expression = 'Social standing and reputation preservation',
    cr.marketing_angle = 'Position products as status enhancers; emphasize prestige and recognition',
    cr.display_name = '面子 (Miànzi - Face)',
    cr.content = 'Social standing and reputation preservation',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-CN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:guanxi-relationships@zh-CN'})
SET cr.locale = 'zh-CN',
    cr.text = '关系 (Guānxi - Relationships)',
    cr.importance = 'high',
    cr.expression = 'Network-based trust and reciprocal obligations',
    cr.marketing_angle = 'Use testimonials, referrals, KOL endorsements; build community',
    cr.display_name = '关系 (Guānxi - Relationships)',
    cr.content = 'Network-based trust and reciprocal obligations',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-CN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hexie-harmony@zh-CN'})
SET cr.locale = 'zh-CN',
    cr.text = '和谐 (Héxié - Harmony)',
    cr.importance = 'high',
    cr.expression = 'Social cohesion and conflict avoidance',
    cr.marketing_angle = 'Frame solutions as bringing balance; avoid confrontational messaging',
    cr.display_name = '和谐 (Héxié - Harmony)',
    cr.content = 'Social cohesion and conflict avoidance',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-CN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:jiti-zhuyi-collectivism@zh-CN'})
SET cr.locale = 'zh-CN',
    cr.text = '集体主义 (Jítǐ zhǔyì - Collectivism)',
    cr.importance = 'high',
    cr.expression = 'Group identity over individual; family and national pride',
    cr.marketing_angle = 'Emphasize family benefits, national brands, \'Made in China\' quality narratives',
    cr.display_name = '集体主义 (Jítǐ zhǔyì - Collectivism)',
    cr.content = 'Group identity over individual; family and national pride',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-CN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:wushi-pragmatism@zh-CN'})
SET cr.locale = 'zh-CN',
    cr.text = '务实 (Wùshí - Pragmatism)',
    cr.importance = 'medium',
    cr.expression = 'Practical results and value-for-money orientation',
    cr.marketing_angle = 'Show concrete ROI, practical benefits, value propositions',
    cr.display_name = '务实 (Wùshí - Pragmatism)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for zh-CN. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Practical results and value-for-money orientation',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-CN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:mianzi-face@zh-TW'})
SET cr.locale = 'zh-TW',
    cr.text = '面子 (Miànzi - Face)',
    cr.importance = 'high',
    cr.expression = 'Personal dignity and social reputation',
    cr.marketing_angle = 'Quality and refinement messaging; less about status display than zh-CN',
    cr.display_name = '面子 (Miànzi - Face)',
    cr.content = 'Personal dignity and social reputation',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-TW'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:renqingwei-human-touch@zh-TW'})
SET cr.locale = 'zh-TW',
    cr.text = '人情味 (Rénqíngwèi - Human touch)',
    cr.importance = 'high',
    cr.expression = 'Warmth, empathy, personal connection in service',
    cr.marketing_angle = 'Customer service excellence; personalized attention; boutique feel',
    cr.display_name = '人情味 (Rénqíngwèi - Human touch)',
    cr.content = 'Warmth, empathy, personal connection in service',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-TW'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:xiao-quexing-small-certain-happiness@zh-TW'})
SET cr.locale = 'zh-TW',
    cr.text = '小確幸 (Xiǎo quèxìng - Small certain happiness)',
    cr.importance = 'medium',
    cr.expression = 'Appreciating small pleasures in daily life',
    cr.marketing_angle = 'Lifestyle products; premium everyday items; self-care narratives',
    cr.display_name = '小確幸 (Xiǎo quèxìng - Small certain happiness)',
    cr.content = 'Appreciating small pleasures in daily life',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-TW'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ziyou-minzhu-freedom-and-democracy@zh-TW'})
SET cr.locale = 'zh-TW',
    cr.text = '自由民主 (Zìyóu mínzhǔ - Freedom and democracy)',
    cr.importance = 'high',
    cr.expression = 'Individual choice and democratic values',
    cr.marketing_angle = 'Emphasize choice, customization, personal expression',
    cr.display_name = '自由民主 (Zìyóu mínzhǔ - Freedom and democracy)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for zh-TW. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Individual choice and democratic values',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-TW'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:lion-rock-spirit@zh-HK'})
SET cr.locale = 'zh-HK',
    cr.text = '獅子山精神 (Lion Rock Spirit)',
    cr.importance = 'high',
    cr.expression = 'Resilience, hard work, self-made success',
    cr.marketing_angle = 'Entrepreneurial spirit; self-improvement; professional growth',
    cr.display_name = '獅子山精神 (Lion Rock Spirit)',
    cr.content = 'Resilience, hard work, self-made success',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-HK'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:mouh-saht-pragmatism@zh-HK'})
SET cr.locale = 'zh-HK',
    cr.text = '務實 (Mouh saht - Pragmatism)',
    cr.importance = 'high',
    cr.expression = 'Practical efficiency and results-orientation',
    cr.marketing_angle = 'Time-saving solutions; efficiency gains; ROI clarity',
    cr.display_name = '務實 (Mouh saht - Pragmatism)',
    cr.content = 'Practical efficiency and results-orientation',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-HK'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:gwok-jai-fa-internationalism@zh-HK'})
SET cr.locale = 'zh-HK',
    cr.text = '國際化 (Gwok jai fa - Internationalism)',
    cr.importance = 'high',
    cr.expression = 'Cosmopolitan identity; East-meets-West sensibility',
    cr.marketing_angle = 'Global brands with local adaptation; bilingual content',
    cr.display_name = '國際化 (Gwok jai fa - Internationalism)',
    cr.content = 'Cosmopolitan identity; East-meets-West sensibility',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-HK'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:min-face@zh-HK'})
SET cr.locale = 'zh-HK',
    cr.text = '面 (Mín - Face)',
    cr.importance = 'high',
    cr.expression = 'Professional reputation and social standing',
    cr.marketing_angle = 'Premium positioning; luxury associations',
    cr.display_name = '面 (Mín - Face)',
    cr.content = 'Professional reputation and social standing',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@zh-HK'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:wa-harmony@ja-JP'})
SET cr.locale = 'ja-JP',
    cr.text = '和 (Wa - Harmony)',
    cr.importance = 'critical',
    cr.expression = 'Group harmony and avoiding conflict; consensus-based decisions',
    cr.marketing_angle = 'Solutions that integrate smoothly; team benefits; non-disruptive positioning',
    cr.display_name = '和 (Wa - Harmony)',
    cr.content = 'Group harmony and avoiding conflict; consensus-based decisions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ja-JP'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:honne-to-tatemae@ja-JP'})
SET cr.locale = 'ja-JP',
    cr.text = '本音と建前 (Honne to Tatemae)',
    cr.importance = 'high',
    cr.expression = 'Private feelings vs. public facade; reading between lines',
    cr.marketing_angle = 'Subtle messaging; implications over explicit claims; respect privacy',
    cr.display_name = '本音と建前 (Honne to Tatemae)',
    cr.content = 'Private feelings vs. public facade; reading between lines',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ja-JP'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:omotenashi-hospitality@ja-JP'})
SET cr.locale = 'ja-JP',
    cr.text = 'おもてなし (Omotenashi - Hospitality)',
    cr.importance = 'high',
    cr.expression = 'Anticipatory service; exceeding expectations without being asked',
    cr.marketing_angle = 'Premium customer service; attention to detail; surprise and delight',
    cr.display_name = 'おもてなし (Omotenashi - Hospitality)',
    cr.content = 'Anticipatory service; exceeding expectations without being asked',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ja-JP'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kaizen-continuous-improvement@ja-JP'})
SET cr.locale = 'ja-JP',
    cr.text = '改善 (Kaizen - Continuous improvement)',
    cr.importance = 'high',
    cr.expression = 'Incremental betterment; process optimization',
    cr.marketing_angle = 'Version updates; iterative improvements; long-term partnership',
    cr.display_name = '改善 (Kaizen - Continuous improvement)',
    cr.content = 'Incremental betterment; process optimization',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ja-JP'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hinshitsu-quality@ja-JP'})
SET cr.locale = 'ja-JP',
    cr.text = '品質 (Hinshitsu - Quality)',
    cr.importance = 'critical',
    cr.expression = 'Meticulous attention to quality and craftsmanship',
    cr.marketing_angle = 'Quality certifications; detailed specifications; Japanese-made associations',
    cr.display_name = '品質 (Hinshitsu - Quality)',
    cr.content = 'Meticulous attention to quality and craftsmanship',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ja-JP'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ppalli-ppalli-quick-quick@ko-KR'})
SET cr.locale = 'ko-KR',
    cr.text = '빨리빨리 (Ppalli-ppalli - Quick quick)',
    cr.importance = 'high',
    cr.expression = 'Speed and urgency culture; fast execution',
    cr.marketing_angle = 'Fast delivery; instant results; efficiency messaging',
    cr.display_name = '빨리빨리 (Ppalli-ppalli - Quick quick)',
    cr.content = 'Speed and urgency culture; fast execution',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ko-KR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:nunchi-social-awareness@ko-KR'})
SET cr.locale = 'ko-KR',
    cr.text = '눈치 (Nunchi - Social awareness)',
    cr.importance = 'high',
    cr.expression = 'Reading the room; understanding unspoken social cues',
    cr.marketing_angle = 'Intuitive UX; anticipatory features; \'we understand you\'',
    cr.display_name = '눈치 (Nunchi - Social awareness)',
    cr.content = 'Reading the room; understanding unspoken social cues',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ko-KR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:chaemyeon-face@ko-KR'})
SET cr.locale = 'ko-KR',
    cr.text = '체면 (Chaemyeon - Face)',
    cr.importance = 'high',
    cr.expression = 'Social reputation and dignity',
    cr.marketing_angle = 'Premium positioning; status association; avoid embarrassment',
    cr.display_name = '체면 (Chaemyeon - Face)',
    cr.content = 'Social reputation and dignity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ko-KR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:jeong-affection-bond@ko-KR'})
SET cr.locale = 'ko-KR',
    cr.text = '정 (Jeong - Affection/bond)',
    cr.importance = 'high',
    cr.expression = 'Deep emotional attachment formed over time',
    cr.marketing_angle = 'Loyalty programs; community building; long-term relationships',
    cr.display_name = '정 (Jeong - Affection/bond)',
    cr.content = 'Deep emotional attachment formed over time',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ko-KR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:han-collective-grief-resilience@ko-KR'})
SET cr.locale = 'ko-KR',
    cr.text = '한 (Han - Collective grief/resilience)',
    cr.importance = 'medium',
    cr.expression = 'Shared historical suffering transformed into strength',
    cr.marketing_angle = 'Underdog narratives; overcoming challenges; national pride',
    cr.display_name = '한 (Han - Collective grief/resilience)',
    cr.content = 'Shared historical suffering transformed into strength',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ko-KR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:jugaad-creative-improvisation@hi-IN'})
SET cr.locale = 'hi-IN',
    cr.text = 'जुगाड़ (Jugaad - Creative improvisation)',
    cr.importance = 'high',
    cr.expression = 'Resourceful problem-solving; making do with less',
    cr.marketing_angle = 'Value for money; clever solutions; affordability with quality',
    cr.display_name = 'जुगाड़ (Jugaad - Creative improvisation)',
    cr.content = 'Resourceful problem-solving; making do with less',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@hi-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:parivaar-family@hi-IN'})
SET cr.locale = 'hi-IN',
    cr.text = 'परिवार (Parivaar - Family)',
    cr.importance = 'critical',
    cr.expression = 'Joint family values; family-centric decisions',
    cr.marketing_angle = 'Family benefits; multi-generational appeal; family plans',
    cr.display_name = 'परिवार (Parivaar - Family)',
    cr.content = 'Joint family values; family-centric decisions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@hi-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:atithi-devo-bhava-guest-is-god@hi-IN'})
SET cr.locale = 'hi-IN',
    cr.text = 'आतिथ्य (Atithi Devo Bhava - Guest is God)',
    cr.importance = 'high',
    cr.expression = 'Exceptional hospitality; customer as honored guest',
    cr.marketing_angle = 'Customer respect; premium service; \'you deserve the best\'',
    cr.display_name = 'आतिथ्य (Atithi Devo Bhava - Guest is God)',
    cr.content = 'Exceptional hospitality; customer as honored guest',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@hi-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:aashavaad-optimism@hi-IN'})
SET cr.locale = 'hi-IN',
    cr.text = 'आशावाद (Aashavaad - Optimism)',
    cr.importance = 'medium',
    cr.expression = 'Positive outlook; aspirational mindset',
    cr.marketing_angle = 'Aspirational messaging; better future; growth stories',
    cr.display_name = 'आशावाद (Aashavaad - Optimism)',
    cr.content = 'Positive outlook; aspirational mindset',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@hi-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:atmiyota-kinship@bn-BD'})
SET cr.locale = 'bn-BD',
    cr.text = 'আত্মীয়তা (Atmiyota - Kinship)',
    cr.importance = 'high',
    cr.expression = 'Strong family and community bonds',
    cr.marketing_angle = 'Community benefits; family packages; group discounts',
    cr.display_name = 'আত্মীয়তা (Atmiyota - Kinship)',
    cr.content = 'Strong family and community bonds',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@bn-BD'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:atithi-guest-respect@bn-BD'})
SET cr.locale = 'bn-BD',
    cr.text = 'অতিথি (Atithi - Guest respect)',
    cr.importance = 'high',
    cr.expression = 'Hospitality and warmth to outsiders',
    cr.marketing_angle = 'Customer appreciation; welcoming messaging',
    cr.display_name = 'অতিথি (Atithi - Guest respect)',
    cr.content = 'Hospitality and warmth to outsiders',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@bn-BD'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:sanskriti-cultural-pride@bn-BD'})
SET cr.locale = 'bn-BD',
    cr.text = 'সংস্কৃতি (Sanskriti - Cultural pride)',
    cr.importance = 'high',
    cr.expression = 'Bengali language and cultural heritage pride',
    cr.marketing_angle = 'Bengali language content; cultural references; local festivals',
    cr.display_name = 'সংস্কৃতি (Sanskriti - Cultural pride)',
    cr.content = 'Bengali language and cultural heritage pride',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@bn-BD'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:uddyam-entrepreneurship@bn-BD'})
SET cr.locale = 'bn-BD',
    cr.text = 'উদ্যম (Uddyam - Entrepreneurship)',
    cr.importance = 'medium',
    cr.expression = 'Rising entrepreneurial spirit; self-improvement',
    cr.marketing_angle = 'Business growth; skill development; economic empowerment',
    cr.display_name = 'উদ্যম (Uddyam - Entrepreneurship)',
    cr.content = 'Rising entrepreneurial spirit; self-improvement',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@bn-BD'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:tamil-perumai-tamil-pride@ta-IN'})
SET cr.locale = 'ta-IN',
    cr.text = 'தமிழ் பெருமை (Tamil Perumai - Tamil pride)',
    cr.importance = 'critical',
    cr.expression = 'Strong pride in Tamil language and culture; oldest living language',
    cr.marketing_angle = 'Tamil-first content; acknowledge cultural heritage; avoid Hindi imposition',
    cr.display_name = 'தமிழ் பெருமை (Tamil Perumai - Tamil pride)',
    cr.content = 'Strong pride in Tamil language and culture; oldest living language',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ta-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kalvi-education@ta-IN'})
SET cr.locale = 'ta-IN',
    cr.text = 'கல்வி (Kalvi - Education)',
    cr.importance = 'high',
    cr.expression = 'High value on education and learning',
    cr.marketing_angle = 'Educational benefits; skill improvement; knowledge-focused',
    cr.display_name = 'கல்வி (Kalvi - Education)',
    cr.content = 'High value on education and learning',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ta-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kudumbam-family@ta-IN'})
SET cr.locale = 'ta-IN',
    cr.text = 'குடும்பம் (Kudumbam - Family)',
    cr.importance = 'high',
    cr.expression = 'Strong family bonds and joint family system',
    cr.marketing_angle = 'Family benefits; multi-user plans; elder respect',
    cr.display_name = 'குடும்பம் (Kudumbam - Family)',
    cr.content = 'Strong family bonds and joint family system',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ta-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:thozhil-industry-work-ethic@ta-IN'})
SET cr.locale = 'ta-IN',
    cr.text = 'தொழில் (Thozhil - Industry/work ethic)',
    cr.importance = 'medium',
    cr.expression = 'Strong work ethic; industrial state',
    cr.marketing_angle = 'Productivity tools; business growth; professional development',
    cr.display_name = 'தொழில் (Thozhil - Industry/work ethic)',
    cr.content = 'Strong work ethic; industrial state',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ta-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:telugu-garvam-telugu-pride@te-IN'})
SET cr.locale = 'te-IN',
    cr.text = 'తెలుగు గర్వం (Telugu Garvam - Telugu pride)',
    cr.importance = 'high',
    cr.expression = 'Pride in Telugu language; \'Italian of the East\'',
    cr.marketing_angle = 'Telugu language content; cultural respect; regional focus',
    cr.display_name = 'తెలుగు గర్వం (Telugu Garvam - Telugu pride)',
    cr.content = 'Pride in Telugu language; \'Italian of the East\'',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@te-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:atithi-devo-bhava@te-IN'})
SET cr.locale = 'te-IN',
    cr.text = 'అతిథి దేవో భవ (Atithi Devo Bhava)',
    cr.importance = 'high',
    cr.expression = 'Guest is God; exceptional hospitality',
    cr.marketing_angle = 'Customer respect; VIP treatment messaging',
    cr.display_name = 'అతిథి దేవో భవ (Atithi Devo Bhava)',
    cr.content = 'Guest is God; exceptional hospitality',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@te-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kutumbam-family@te-IN'})
SET cr.locale = 'te-IN',
    cr.text = 'కుటుంబం (Kutumbam - Family)',
    cr.importance = 'high',
    cr.expression = 'Strong family values; joint family decisions',
    cr.marketing_angle = 'Family plans; multi-generational benefits',
    cr.display_name = 'కుటుంబం (Kutumbam - Family)',
    cr.content = 'Strong family values; joint family decisions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@te-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:vyaapaaram-business@te-IN'})
SET cr.locale = 'te-IN',
    cr.text = 'వ్యాపారం (Vyaapaaram - Business)',
    cr.importance = 'high',
    cr.expression = 'Strong business community; entrepreneurial culture',
    cr.marketing_angle = 'Business growth; SME focus; Hyderabad tech hub associations',
    cr.display_name = 'వ్యాపారం (Vyaapaaram - Business)',
    cr.content = 'Strong business community; entrepreneurial culture',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@te-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:tinh-cam-gia-inh-family-affection@vi-VN'})
SET cr.locale = 'vi-VN',
    cr.text = 'Tình cảm gia đình (Family affection)',
    cr.importance = 'critical',
    cr.expression = 'Deep family bonds; filial piety; ancestral respect',
    cr.marketing_angle = 'Family benefits; honoring parents; multi-generational appeal',
    cr.display_name = 'Tình cảm gia đình (Family affection)',
    cr.content = 'Deep family bonds; filial piety; ancestral respect',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@vi-VN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:tiet-kiem-thrift@vi-VN'})
SET cr.locale = 'vi-VN',
    cr.text = 'Tiết kiệm (Thrift)',
    cr.importance = 'high',
    cr.expression = 'Value-consciousness; saving mindset; practical spending',
    cr.marketing_angle = 'Value for money; savings messaging; practical benefits',
    cr.display_name = 'Tiết kiệm (Thrift)',
    cr.content = 'Value-consciousness; saving mindset; practical spending',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@vi-VN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hoc-hoi-learning@vi-VN'})
SET cr.locale = 'vi-VN',
    cr.text = 'Học hỏi (Learning)',
    cr.importance = 'high',
    cr.expression = 'High value on education and self-improvement',
    cr.marketing_angle = 'Educational benefits; skill development; career advancement',
    cr.display_name = 'Học hỏi (Learning)',
    cr.content = 'High value on education and self-improvement',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@vi-VN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:tu-hao-dan-toc-national-pride@vi-VN'})
SET cr.locale = 'vi-VN',
    cr.text = 'Tự hào dân tộc (National pride)',
    cr.importance = 'high',
    cr.expression = 'Pride in Vietnamese independence and resilience',
    cr.marketing_angle = 'Vietnamese success stories; local brand partnerships',
    cr.display_name = 'Tự hào dân tộc (National pride)',
    cr.content = 'Pride in Vietnamese independence and resilience',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@vi-VN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:sanuk-fun@th-TH'})
SET cr.locale = 'th-TH',
    cr.text = 'สนุก (Sanuk - Fun)',
    cr.importance = 'high',
    cr.expression = 'Life should be enjoyable; work should be pleasant',
    cr.marketing_angle = 'Enjoyable experience; fun messaging; not too serious',
    cr.display_name = 'สนุก (Sanuk - Fun)',
    cr.content = 'Life should be enjoyable; work should be pleasant',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@th-TH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kreng-jai-considerate-heart@th-TH'})
SET cr.locale = 'th-TH',
    cr.text = 'เกรงใจ (Kreng Jai - Considerate heart)',
    cr.importance = 'critical',
    cr.expression = 'Not imposing on others; consideration; avoiding burden',
    cr.marketing_angle = 'No-pressure sales; respectful approach; easy opt-out',
    cr.display_name = 'เกรงใจ (Kreng Jai - Considerate heart)',
    cr.content = 'Not imposing on others; consideration; avoiding burden',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@th-TH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:na-face@th-TH'})
SET cr.locale = 'th-TH',
    cr.text = 'หน้า (Na - Face)',
    cr.importance = 'high',
    cr.expression = 'Social reputation and dignity preservation',
    cr.marketing_angle = 'Premium positioning; discretion; status enhancement',
    cr.display_name = 'หน้า (Na - Face)',
    cr.content = 'Social reputation and dignity preservation',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@th-TH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:monarchy-reverence@th-TH'})
SET cr.locale = 'th-TH',
    cr.text = 'พระมหากษัตริย์ (Monarchy reverence)',
    cr.importance = 'critical',
    cr.expression = 'Deep reverence for the Thai royal family',
    cr.marketing_angle = 'Royal warrant associations where applicable; never negative royal references',
    cr.display_name = 'พระมหากษัตริย์ (Monarchy reverence)',
    cr.content = 'Deep reverence for the Thai royal family',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@th-TH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:gotong-royong-mutual-cooperation@id-ID'})
SET cr.locale = 'id-ID',
    cr.text = 'Gotong Royong (Mutual cooperation)',
    cr.importance = 'critical',
    cr.expression = 'Community cooperation; helping each other; collective effort',
    cr.marketing_angle = 'Community benefits; collaborative features; shared success',
    cr.display_name = 'Gotong Royong (Mutual cooperation)',
    cr.content = 'Community cooperation; helping each other; collective effort',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@id-ID'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bhinneka-tunggal-ika-unity-in-diversity@id-ID'})
SET cr.locale = 'id-ID',
    cr.text = 'Bhinneka Tunggal Ika (Unity in Diversity)',
    cr.importance = 'high',
    cr.expression = 'National motto; embracing diversity; inclusive identity',
    cr.marketing_angle = 'Inclusive messaging; pan-Indonesian appeal; avoid ethnic focus',
    cr.display_name = 'Bhinneka Tunggal Ika (Unity in Diversity)',
    cr.content = 'National motto; embracing diversity; inclusive identity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@id-ID'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kekeluargaan-family-spirit@id-ID'})
SET cr.locale = 'id-ID',
    cr.text = 'Kekeluargaan (Family spirit)',
    cr.importance = 'high',
    cr.expression = 'Treating everyone like family; warmth and belonging',
    cr.marketing_angle = 'Welcoming tone; \'part of the family\' messaging',
    cr.display_name = 'Kekeluargaan (Family spirit)',
    cr.content = 'Treating everyone like family; warmth and belonging',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@id-ID'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hormat-respect@id-ID'})
SET cr.locale = 'id-ID',
    cr.text = 'Hormat (Respect)',
    cr.importance = 'high',
    cr.expression = 'Respect for elders, authority, and religion',
    cr.marketing_angle = 'Respectful communication; appropriate honorifics',
    cr.display_name = 'Hormat (Respect)',
    cr.content = 'Respect for elders, authority, and religion',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@id-ID'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:budi-kindness-courtesy@ms-MY'})
SET cr.locale = 'ms-MY',
    cr.text = 'Budi (Kindness/courtesy)',
    cr.importance = 'high',
    cr.expression = 'Graciousness; reciprocal kindness; polite conduct',
    cr.marketing_angle = 'Courteous messaging; appreciation; gentle approach',
    cr.display_name = 'Budi (Kindness/courtesy)',
    cr.content = 'Graciousness; reciprocal kindness; polite conduct',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ms-MY'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:muhibbah-goodwill-harmony@ms-MY'})
SET cr.locale = 'ms-MY',
    cr.text = 'Muhibbah (Goodwill/harmony)',
    cr.importance = 'high',
    cr.expression = 'Inter-ethnic harmony; multicultural coexistence',
    cr.marketing_angle = 'Inclusive imagery; multi-ethnic representation; unity themes',
    cr.display_name = 'Muhibbah (Goodwill/harmony)',
    cr.content = 'Inter-ethnic harmony; multicultural coexistence',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ms-MY'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hormat-respect@ms-MY'})
SET cr.locale = 'ms-MY',
    cr.text = 'Hormat (Respect)',
    cr.importance = 'high',
    cr.expression = 'Respect for elders and authority; hierarchy awareness',
    cr.marketing_angle = 'Respectful tone; appropriate formality',
    cr.display_name = 'Hormat (Respect)',
    cr.content = 'Respect for elders and authority; hierarchy awareness',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ms-MY'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bangsa-malaysia-malaysian-identity@ms-MY'})
SET cr.locale = 'ms-MY',
    cr.text = 'Bangsa Malaysia (Malaysian identity)',
    cr.importance = 'medium',
    cr.expression = 'Unified national identity across ethnicities',
    cr.marketing_angle = 'Pan-Malaysian appeal; national pride; local success stories',
    cr.display_name = 'Bangsa Malaysia (Malaysian identity)',
    cr.content = 'Unified national identity across ethnicities',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ms-MY'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bayanihan-community-spirit@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Bayanihan (Community spirit)',
    cr.importance = 'critical',
    cr.expression = 'Neighbors helping neighbors; community cooperation',
    cr.marketing_angle = 'Community features; helping others; shared success',
    cr.display_name = 'Bayanihan (Community spirit)',
    cr.content = 'Neighbors helping neighbors; community cooperation',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:pakikisama-smooth-relationships@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Pakikisama (Smooth relationships)',
    cr.importance = 'high',
    cr.expression = 'Getting along; maintaining harmony; group acceptance',
    cr.marketing_angle = 'Social features; group plans; \'share with friends\'',
    cr.display_name = 'Pakikisama (Smooth relationships)',
    cr.content = 'Getting along; maintaining harmony; group acceptance',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hiya-shame-social-propriety@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Hiya (Shame/social propriety)',
    cr.importance = 'high',
    cr.expression = 'Avoiding shame; maintaining social dignity',
    cr.marketing_angle = 'Discrete purchases; privacy features; face-saving options',
    cr.display_name = 'Hiya (Shame/social propriety)',
    cr.content = 'Avoiding shame; maintaining social dignity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:utang-na-loob-debt-of-gratitude@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Utang na Loob (Debt of gratitude)',
    cr.importance = 'high',
    cr.expression = 'Reciprocal obligations; loyalty from kindness received',
    cr.marketing_angle = 'Loyalty programs; referral rewards; gratitude messaging',
    cr.display_name = 'Utang na Loob (Debt of gratitude)',
    cr.content = 'Reciprocal obligations; loyalty from kindness received',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:masayahin-joyful-disposition@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Masayahin (Joyful disposition)',
    cr.importance = 'medium',
    cr.expression = 'Positive outlook; finding joy despite challenges',
    cr.marketing_angle = 'Upbeat messaging; humor acceptable; celebration themes',
    cr.display_name = 'Masayahin (Joyful disposition)',
    cr.content = 'Positive outlook; finding joy despite challenges',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:al-diyafa-hospitality@ar-SA'})
SET cr.locale = 'ar-SA',
    cr.text = 'الضيافة (Al-Diyafa - Hospitality)',
    cr.importance = 'critical',
    cr.expression = 'Welcoming guests is a sacred duty; generosity reflects family honor',
    cr.marketing_angle = 'Position product as enhancing ability to serve/welcome others generously',
    cr.display_name = 'الضيافة (Al-Diyafa - Hospitality)',
    cr.content = 'Welcoming guests is a sacred duty; generosity reflects family honor',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-SA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:al-sharaf-honor@ar-SA'})
SET cr.locale = 'ar-SA',
    cr.text = 'الشرف (Al-Sharaf - Honor)',
    cr.importance = 'critical',
    cr.expression = 'Family and tribal honor guide all decisions; reputation is paramount',
    cr.marketing_angle = 'Frame purchases as enhancing family prestige and social standing',
    cr.display_name = 'الشرف (Al-Sharaf - Honor)',
    cr.content = 'Family and tribal honor guide all decisions; reputation is paramount',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-SA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:al-iman-faith@ar-SA'})
SET cr.locale = 'ar-SA',
    cr.text = 'الإيمان (Al-Iman - Faith)',
    cr.importance = 'critical',
    cr.expression = 'Islam permeates daily life; five daily prayers structure the day',
    cr.marketing_angle = 'Align with Islamic values; avoid scheduling during prayer times',
    cr.display_name = 'الإيمان (Al-Iman - Faith)',
    cr.content = 'Islam permeates daily life; five daily prayers structure the day',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-SA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:family-loyalty@ar-SA'})
SET cr.locale = 'ar-SA',
    cr.text = 'الولاء للعائلة (Family Loyalty)',
    cr.importance = 'high',
    cr.expression = 'Extended family decisions made collectively; elders consulted',
    cr.marketing_angle = 'Target family units; show multi-generational appeal',
    cr.display_name = 'الولاء للعائلة (Family Loyalty)',
    cr.content = 'Extended family decisions made collectively; elders consulted',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-SA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:2030-vision-2030-modernization@ar-SA'})
SET cr.locale = 'ar-SA',
    cr.text = 'رؤية 2030 (Vision 2030 - Modernization)',
    cr.importance = 'high',
    cr.expression = 'National pride in transformation; openness to innovation within Islamic framework',
    cr.marketing_angle = 'Position as supporting national development goals; tech-forward messaging',
    cr.display_name = 'رؤية 2030 (Vision 2030 - Modernization)',
    cr.content = 'National pride in transformation; openness to innovation within Islamic framework',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-SA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ibn-el-balad-authenticity@ar-EG'})
SET cr.locale = 'ar-EG',
    cr.text = 'ابن البلد (Ibn el-Balad - Authenticity)',
    cr.importance = 'high',
    cr.expression = 'Being genuinely Egyptian; street-smart, warm, and unpretentious',
    cr.marketing_angle = 'Use authentic Egyptian dialect (Masri); avoid overly formal MSA',
    cr.display_name = 'ابن البلد (Ibn el-Balad - Authenticity)',
    cr.content = 'Being genuinely Egyptian; street-smart, warm, and unpretentious',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-EG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:egyptian-humor@ar-EG'})
SET cr.locale = 'ar-EG',
    cr.text = 'الفكاهة المصرية (Egyptian Humor)',
    cr.importance = 'high',
    cr.expression = 'Egyptians use humor to cope with challenges; witty wordplay valued',
    cr.marketing_angle = 'Humor in advertising is effective and appreciated; puns work well',
    cr.display_name = 'الفكاهة المصرية (Egyptian Humor)',
    cr.content = 'Egyptians use humor to cope with challenges; witty wordplay valued',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-EG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:pharaonic-heritage@ar-EG'})
SET cr.locale = 'ar-EG',
    cr.text = 'الحضارة الفرعونية (Pharaonic Heritage)',
    cr.importance = 'medium',
    cr.expression = 'Pride in ancient civilization; 7000 years of history',
    cr.marketing_angle = 'Heritage references resonate; show respect for Egypt\'s historical significance',
    cr.display_name = 'الحضارة الفرعونية (Pharaonic Heritage)',
    cr.content = 'Pride in ancient civilization; 7000 years of history',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-EG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:el-aila-family@ar-EG'})
SET cr.locale = 'ar-EG',
    cr.text = 'العيلة (El-Aila - Family)',
    cr.importance = 'high',
    cr.expression = 'Close-knit family bonds; children highly valued',
    cr.marketing_angle = 'Family-oriented messaging; multi-generational appeal',
    cr.display_name = 'العيلة (El-Aila - Family)',
    cr.content = 'Close-knit family bonds; children highly valued',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-EG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:nostalgia@ar-EG'})
SET cr.locale = 'ar-EG',
    cr.text = 'الحنين للماضي (Nostalgia)',
    cr.importance = 'medium',
    cr.expression = 'Fondness for Egypt\'s golden era (1950s-60s cinema, music)',
    cr.marketing_angle = 'Retro aesthetics and nostalgic references can be powerful',
    cr.display_name = 'الحنين للماضي (Nostalgia)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for ar-EG. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Fondness for Egypt\'s golden era (1950s-60s cinema, music)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-EG'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:al-tasamuh-tolerance@ar-AE'})
SET cr.locale = 'ar-AE',
    cr.text = 'التسامح (Al-Tasamuh - Tolerance)',
    cr.importance = 'high',
    cr.expression = 'UAE positions itself as beacon of tolerance; Year of Tolerance (2019) codified this',
    cr.marketing_angle = 'Inclusive messaging works; multicultural imagery appropriate',
    cr.display_name = 'التسامح (Al-Tasamuh - Tolerance)',
    cr.content = 'UAE positions itself as beacon of tolerance; Year of Tolerance (2019) codified this',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-AE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:al-tumuh-ambition@ar-AE'})
SET cr.locale = 'ar-AE',
    cr.text = 'الطموح (Al-Tumuh - Ambition)',
    cr.importance = 'critical',
    cr.expression = 'National ethos of achieving the impossible (tallest building, Mars mission)',
    cr.marketing_angle = 'Innovation and future-forward messaging highly effective',
    cr.display_name = 'الطموح (Al-Tumuh - Ambition)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for ar-AE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'National ethos of achieving the impossible (tallest building, Mars mission)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-AE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:al-rafahiya-luxury@ar-AE'})
SET cr.locale = 'ar-AE',
    cr.text = 'الرفاهية (Al-Rafahiya - Luxury)',
    cr.importance = 'high',
    cr.expression = 'Appreciation for premium quality and exclusive experiences',
    cr.marketing_angle = 'Premium positioning works; quality over price messaging',
    cr.display_name = 'الرفاهية (Al-Rafahiya - Luxury)',
    cr.content = 'Appreciation for premium quality and exclusive experiences',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-AE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:emirati-identity@ar-AE'})
SET cr.locale = 'ar-AE',
    cr.text = 'الهوية الإماراتية (Emirati Identity)',
    cr.importance = 'high',
    cr.expression = 'Despite expat majority (85%), Emirati culture and leadership respected',
    cr.marketing_angle = 'Respect for local traditions while embracing global outlook',
    cr.display_name = 'الهوية الإماراتية (Emirati Identity)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for ar-AE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Despite expat majority (85%), Emirati culture and leadership respected',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-AE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:artisanal-craftsmanship@ar-MA'})
SET cr.locale = 'ar-MA',
    cr.text = 'الصناعة التقليدية (Artisanal Craftsmanship)',
    cr.importance = 'high',
    cr.expression = 'Pride in traditional crafts; medina artisans; handmade quality',
    cr.marketing_angle = 'Authenticity and craftsmanship messaging resonates',
    cr.display_name = 'الصناعة التقليدية (Artisanal Craftsmanship)',
    cr.content = 'Pride in traditional crafts; medina artisans; handmade quality',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-MA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:cultural-diversity@ar-MA'})
SET cr.locale = 'ar-MA',
    cr.text = 'التعدد الثقافي (Cultural Diversity)',
    cr.importance = 'high',
    cr.expression = 'Arab, Berber (Amazigh), African, and European influences coexist',
    cr.marketing_angle = 'Acknowledge diverse heritage; Amazigh culture respected',
    cr.display_name = 'التعدد الثقافي (Cultural Diversity)',
    cr.content = 'Arab, Berber (Amazigh), African, and European influences coexist',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-MA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:monarchy@ar-MA'})
SET cr.locale = 'ar-MA',
    cr.text = 'الملكية (Monarchy)',
    cr.importance = 'high',
    cr.expression = 'King Mohammed VI highly respected; symbol of stability',
    cr.marketing_angle = 'Avoid any criticism; royal initiatives can be referenced positively',
    cr.display_name = 'الملكية (Monarchy)',
    cr.content = 'King Mohammed VI highly respected; symbol of stability',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-MA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:openness@ar-MA'})
SET cr.locale = 'ar-MA',
    cr.text = 'الانفتاح (Openness)',
    cr.importance = 'medium',
    cr.expression = 'Morocco positions as bridge between Africa, Europe, and Arab world',
    cr.marketing_angle = 'Modern, globally-connected positioning works',
    cr.display_name = 'الانفتاح (Openness)',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for ar-MA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Morocco positions as bridge between Africa, Europe, and Arab world',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ar-MA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:dusha-soul@ru-RU'})
SET cr.locale = 'ru-RU',
    cr.text = 'Душа (Dusha - Soul)',
    cr.importance = 'high',
    cr.expression = 'Deep emotional/spiritual dimension to Russian character; soulfulness valued',
    cr.marketing_angle = 'Emotional depth in messaging; avoid superficial cheerfulness',
    cr.display_name = 'Душа (Dusha - Soul)',
    cr.content = 'Deep emotional/spiritual dimension to Russian character; soulfulness valued',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ru-RU'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:collectivism@ru-RU'})
SET cr.locale = 'ru-RU',
    cr.text = 'Коллективизм (Collectivism)',
    cr.importance = 'high',
    cr.expression = 'Group identity important; \'we\' over \'I\'; shared experiences valued',
    cr.marketing_angle = 'Community and shared benefit messaging; avoid excessive individualism',
    cr.display_name = 'Коллективизм (Collectivism)',
    cr.content = 'Group identity important; \'we\' over \'I\'; shared experiences valued',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ru-RU'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:stoikost-resilience@ru-RU'})
SET cr.locale = 'ru-RU',
    cr.text = 'Стойкость (Stoikost - Resilience)',
    cr.importance = 'high',
    cr.expression = 'Endurance through hardship; winter as metaphor; overcoming adversity',
    cr.marketing_angle = 'Durability and reliability messaging; products that last',
    cr.display_name = 'Стойкость (Stoikost - Resilience)',
    cr.content = 'Endurance through hardship; winter as metaphor; overcoming adversity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ru-RU'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:uvazhenie-respect@ru-RU'})
SET cr.locale = 'ru-RU',
    cr.text = 'Уважение (Uvazhenie - Respect)',
    cr.importance = 'high',
    cr.expression = 'Formal respect in professional contexts; hierarchy acknowledged',
    cr.marketing_angle = 'Professional, respectful tone; use formal \'Вы\' (you) address',
    cr.display_name = 'Уважение (Uvazhenie - Respect)',
    cr.content = 'Formal respect in professional contexts; hierarchy acknowledged',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@ru-RU'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:nezalezhnist-independence@uk-UA'})
SET cr.locale = 'uk-UA',
    cr.text = 'Незалежність (Nezalezhnist - Independence)',
    cr.importance = 'critical',
    cr.expression = 'National sovereignty and self-determination; especially since 2022',
    cr.marketing_angle = 'Support for Ukraine; independence and resilience themes powerful',
    cr.display_name = 'Незалежність (Nezalezhnist - Independence)',
    cr.content = 'National sovereignty and self-determination; especially since 2022',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@uk-UA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:national-dignity@uk-UA'})
SET cr.locale = 'uk-UA',
    cr.text = 'Національна гідність (National Dignity)',
    cr.importance = 'critical',
    cr.expression = 'Pride in Ukrainian identity, language, and culture distinct from Russia',
    cr.marketing_angle = 'MUST use Ukrainian language, not Russian; acknowledge distinct identity',
    cr.display_name = 'Національна гідність (National Dignity)',
    cr.content = 'Pride in Ukrainian identity, language, and culture distinct from Russia',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@uk-UA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:cossack-spirit@uk-UA'})
SET cr.locale = 'uk-UA',
    cr.text = 'Козацький дух (Cossack Spirit)',
    cr.importance = 'high',
    cr.expression = 'Historical warrior tradition; freedom-loving, brave, democratic',
    cr.marketing_angle = 'Bravery and self-reliance messaging resonates',
    cr.display_name = 'Козацький дух (Cossack Spirit)',
    cr.content = 'Historical warrior tradition; freedom-loving, brave, democratic',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@uk-UA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:embroidery-heritage@uk-UA'})
SET cr.locale = 'uk-UA',
    cr.text = 'Вишиванка (Embroidery Heritage)',
    cr.importance = 'medium',
    cr.expression = 'Traditional embroidered clothing; cultural symbol of identity',
    cr.marketing_angle = 'Folk art and traditional motifs appreciated',
    cr.display_name = 'Вишиванка (Embroidery Heritage)',
    cr.content = 'Traditional embroidered clothing; cultural symbol of identity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@uk-UA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:rodzina-family@pl-PL'})
SET cr.locale = 'pl-PL',
    cr.text = 'Rodzina (Family)',
    cr.importance = 'critical',
    cr.expression = 'Family is central social unit; multi-generational bonds strong',
    cr.marketing_angle = 'Family-oriented messaging; products that bring family together',
    cr.display_name = 'Rodzina (Family)',
    cr.content = 'Family is central social unit; multi-generational bonds strong',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pl-PL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:tradycja-tradition@pl-PL'})
SET cr.locale = 'pl-PL',
    cr.text = 'Tradycja (Tradition)',
    cr.importance = 'high',
    cr.expression = 'Catholic traditions, national holidays, historical commemorations',
    cr.marketing_angle = 'Respect for tradition; seasonal/holiday marketing important',
    cr.display_name = 'Tradycja (Tradition)',
    cr.content = 'Catholic traditions, national holidays, historical commemorations',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pl-PL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:honor-narodowy-national-honor@pl-PL'})
SET cr.locale = 'pl-PL',
    cr.text = 'Honor narodowy (National Honor)',
    cr.importance = 'high',
    cr.expression = 'Pride in Polish history, especially WWII resistance and Solidarity movement',
    cr.marketing_angle = 'Quality and Polish pride; \'Made in Poland\' resonates',
    cr.display_name = 'Honor narodowy (National Honor)',
    cr.content = 'Pride in Polish history, especially WWII resistance and Solidarity movement',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pl-PL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:goscinnosc-hospitality@pl-PL'})
SET cr.locale = 'pl-PL',
    cr.text = 'Gościnność (Hospitality)',
    cr.importance = 'high',
    cr.expression = 'Welcoming guests generously; food and drink abundance',
    cr.marketing_angle = 'Generous offers; hospitality imagery works',
    cr.display_name = 'Gościnność (Hospitality)',
    cr.content = 'Welcoming guests generously; food and drink abundance',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pl-PL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:skepse-skepticism@cs-CZ'})
SET cr.locale = 'cs-CZ',
    cr.text = 'Skepse (Skepticism)',
    cr.importance = 'high',
    cr.expression = 'Czechs are naturally skeptical; distrust of grand claims and institutions',
    cr.marketing_angle = 'Understated claims work better; avoid hyperbole and superlatives',
    cr.display_name = 'Skepse (Skepticism)',
    cr.content = 'Czechs are naturally skeptical; distrust of grand claims and institutions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@cs-CZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:humor-a-ironie-humor-and-irony@cs-CZ'})
SET cr.locale = 'cs-CZ',
    cr.text = 'Humor a ironie (Humor and Irony)',
    cr.importance = 'high',
    cr.expression = 'Dry, self-deprecating humor; absurdist tradition (Kafka, Hašek)',
    cr.marketing_angle = 'Clever, witty advertising appreciated; avoid taking yourself too seriously',
    cr.display_name = 'Humor a ironie (Humor and Irony)',
    cr.content = 'Dry, self-deprecating humor; absurdist tradition (Kafka, Hašek)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@cs-CZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kvalita-quality@cs-CZ'})
SET cr.locale = 'cs-CZ',
    cr.text = 'Kvalita (Quality)',
    cr.importance = 'high',
    cr.expression = 'Pride in Czech engineering and craftsmanship (Škoda, Czech glass)',
    cr.marketing_angle = 'Quality and precision messaging; \'well-made\' is valued',
    cr.display_name = 'Kvalita (Quality)',
    cr.content = 'Pride in Czech engineering and craftsmanship (Škoda, Czech glass)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@cs-CZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:pivni-kultura-beer-culture@cs-CZ'})
SET cr.locale = 'cs-CZ',
    cr.text = 'Pivní kultura (Beer Culture)',
    cr.importance = 'medium',
    cr.expression = 'World\'s highest beer consumption; beer is cultural institution',
    cr.marketing_angle = 'Social, pub culture imagery; community gatherings',
    cr.display_name = 'Pivní kultura (Beer Culture)',
    cr.content = 'World\'s highest beer consumption; beer is cultural institution',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@cs-CZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:misafirperverlik-hospitality@tr-TR'})
SET cr.locale = 'tr-TR',
    cr.text = 'Misafirperverlik (Hospitality)',
    cr.importance = 'critical',
    cr.expression = 'Guest is sacred; tea offering is ritual; generous hosting expected',
    cr.marketing_angle = 'Welcoming, generous offers; hospitality imagery powerful',
    cr.display_name = 'Misafirperverlik (Hospitality)',
    cr.content = 'Guest is sacred; tea offering is ritual; generous hosting expected',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@tr-TR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:aile-family-honor@tr-TR'})
SET cr.locale = 'tr-TR',
    cr.text = 'Aile (Family Honor)',
    cr.importance = 'critical',
    cr.expression = 'Family reputation paramount; collective family identity',
    cr.marketing_angle = 'Family-oriented messaging; multi-generational appeal',
    cr.display_name = 'Aile (Family Honor)',
    cr.content = 'Family reputation paramount; collective family identity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@tr-TR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ataturk-miras-ataturk-s-legacy@tr-TR'})
SET cr.locale = 'tr-TR',
    cr.text = 'Atatürk mirası (Atatürk\'s Legacy)',
    cr.importance = 'critical',
    cr.expression = 'Founder Mustafa Kemal Atatürk deeply revered; secularism principle',
    cr.marketing_angle = 'Modernization messaging works; respect for founder essential',
    cr.display_name = 'Atatürk mirası (Atatürk\'s Legacy)',
    cr.content = 'Founder Mustafa Kemal Atatürk deeply revered; secularism principle',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@tr-TR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:turk-gururu-turkish-pride@tr-TR'})
SET cr.locale = 'tr-TR',
    cr.text = 'Türk gururu (Turkish Pride)',
    cr.importance = 'high',
    cr.expression = 'Pride in Turkish history, culture, and recent economic growth',
    cr.marketing_angle = 'Local production valued; Turkish brands gaining preference',
    cr.display_name = 'Türk gururu (Turkish Pride)',
    cr.content = 'Pride in Turkish history, culture, and recent economic growth',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@tr-TR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:qonaqp-rv-rlik-hospitality@az-AZ'})
SET cr.locale = 'az-AZ',
    cr.text = 'Qonaqpərvərlik (Hospitality)',
    cr.importance = 'critical',
    cr.expression = 'Guests are honored; tea ceremony (çay) is essential ritual',
    cr.marketing_angle = 'Generous, welcoming positioning; hospitality imagery',
    cr.display_name = 'Qonaqpərvərlik (Hospitality)',
    cr.content = 'Guests are honored; tea ceremony (çay) is essential ritual',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@az-AZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:milli-qurur-national-pride@az-AZ'})
SET cr.locale = 'az-AZ',
    cr.text = 'Milli qürur (National Pride)',
    cr.importance = 'high',
    cr.expression = 'Pride in independence from USSR; oil wealth; modern Baku',
    cr.marketing_angle = 'Modern, progressive Azerbaijan imagery; development narrative',
    cr.display_name = 'Milli qürur (National Pride)',
    cr.content = 'Pride in independence from USSR; oil wealth; modern Baku',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@az-AZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ail-d-y-rl-ri-family-values@az-AZ'})
SET cr.locale = 'az-AZ',
    cr.text = 'Ailə dəyərləri (Family Values)',
    cr.importance = 'high',
    cr.expression = 'Strong family bonds; respect for elders; collective decisions',
    cr.marketing_angle = 'Family-centered messaging; elder respect',
    cr.display_name = 'Ailə dəyərləri (Family Values)',
    cr.content = 'Strong family bonds; respect for elders; collective decisions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@az-AZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:muasirlik-modernity@az-AZ'})
SET cr.locale = 'az-AZ',
    cr.text = 'Müasirlik (Modernity)',
    cr.importance = 'high',
    cr.expression = 'Baku as modern metropolis; Formula 1; Eurovision host',
    cr.marketing_angle = 'Modern, innovative positioning alongside tradition',
    cr.display_name = 'Müasirlik (Modernity)',
    cr.content = 'Baku as modern metropolis; Formula 1; Eurovision host',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@az-AZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:chutzpah-audacity@he-IL'})
SET cr.locale = 'he-IL',
    cr.text = 'חוצפה (Chutzpah - Audacity)',
    cr.importance = 'high',
    cr.expression = 'Audacious confidence; challenging authority; entrepreneurial boldness',
    cr.marketing_angle = 'Bold, innovative claims acceptable; disruption messaging works',
    cr.display_name = 'חוצפה (Chutzpah - Audacity)',
    cr.content = 'Audacious confidence; challenging authority; entrepreneurial boldness',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@he-IL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:startup-nation@he-IL'})
SET cr.locale = 'he-IL',
    cr.text = 'סטארט-אפ ניישן (Startup Nation)',
    cr.importance = 'high',
    cr.expression = 'Pride in tech innovation; high-tech entrepreneurship',
    cr.marketing_angle = 'Innovation and tech-forward positioning resonates strongly',
    cr.display_name = 'סטארט-אפ ניישן (Startup Nation)',
    cr.content = 'Pride in tech innovation; high-tech entrepreneurship',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@he-IL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:mishpachtiyut-family-focus@he-IL'})
SET cr.locale = 'he-IL',
    cr.text = 'משפחתיות (Mishpachtiyut - Family Focus)',
    cr.importance = 'high',
    cr.expression = 'Strong family bonds; children highly valued; Friday dinner sacred',
    cr.marketing_angle = 'Family-oriented messaging; Shabbat timing considerations',
    cr.display_name = 'משפחתיות (Mishpachtiyut - Family Focus)',
    cr.content = 'Strong family bonds; children highly valued; Friday dinner sacred',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@he-IL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bitachon-security@he-IL'})
SET cr.locale = 'he-IL',
    cr.text = 'ביטחון (Bitachon - Security)',
    cr.importance = 'high',
    cr.expression = 'Security consciousness pervasive; resilience valued',
    cr.marketing_angle = 'Reliability and security messaging resonates',
    cr.display_name = 'ביטחון (Bitachon - Security)',
    cr.content = 'Security consciousness pervasive; resilience valued',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@he-IL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:harambee-pulling-together@sw-KE'})
SET cr.locale = 'sw-KE',
    cr.text = 'Harambee (Pulling Together)',
    cr.importance = 'critical',
    cr.expression = 'Community self-help; collective effort; national motto',
    cr.marketing_angle = 'Community benefit messaging; collective improvement themes',
    cr.display_name = 'Harambee (Pulling Together)',
    cr.content = 'Community self-help; collective effort; national motto',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sw-KE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ubuntu-utu-humanity@sw-KE'})
SET cr.locale = 'sw-KE',
    cr.text = 'Ubuntu/Utu (Humanity)',
    cr.importance = 'high',
    cr.expression = 'Shared humanity; interconnectedness; compassion',
    cr.marketing_angle = 'Humanistic brand values; social responsibility',
    cr.display_name = 'Ubuntu/Utu (Humanity)',
    cr.content = 'Shared humanity; interconnectedness; compassion',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sw-KE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:entrepreneurship-hustler-culture@sw-KE'})
SET cr.locale = 'sw-KE',
    cr.text = 'Entrepreneurship (Hustler Culture)',
    cr.importance = 'high',
    cr.expression = 'Self-made success; side hustles; mobile commerce (M-Pesa)',
    cr.marketing_angle = 'Enable success messaging; mobile-first is essential',
    cr.display_name = 'Entrepreneurship (Hustler Culture)',
    cr.content = 'Self-made success; side hustles; mobile commerce (M-Pesa)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sw-KE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:safari-nature-heritage@sw-KE'})
SET cr.locale = 'sw-KE',
    cr.text = 'Safari/Nature Heritage',
    cr.importance = 'medium',
    cr.expression = 'Pride in wildlife and natural beauty; conservation awareness',
    cr.marketing_angle = 'Nature imagery; environmental consciousness',
    cr.display_name = 'Safari/Nature Heritage',
    cr.content = 'Pride in wildlife and natural beauty; conservation awareness',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sw-KE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ujamaa-familyhood-socialism@sw-TZ'})
SET cr.locale = 'sw-TZ',
    cr.text = 'Ujamaa (Familyhood/Socialism)',
    cr.importance = 'high',
    cr.expression = 'Nyerere\'s socialist philosophy; collective welfare; equality',
    cr.marketing_angle = 'Egalitarian messaging; community benefit over individual gain',
    cr.display_name = 'Ujamaa (Familyhood/Socialism)',
    cr.content = 'Nyerere\'s socialist philosophy; collective welfare; equality',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sw-TZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:umoja-wa-kitaifa-national-unity@sw-TZ'})
SET cr.locale = 'sw-TZ',
    cr.text = 'Umoja wa Kitaifa (National Unity)',
    cr.importance = 'high',
    cr.expression = 'No tribal divisions like neighbors; Swahili unifies; 120+ ethnic groups coexist',
    cr.marketing_angle = 'Unity messaging; Swahili-first (less English mixing than Kenya)',
    cr.display_name = 'Umoja wa Kitaifa (National Unity)',
    cr.content = 'No tribal divisions like neighbors; Swahili unifies; 120+ ethnic groups coexist',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sw-TZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:kilimanjaro-serengeti-pride@sw-TZ'})
SET cr.locale = 'sw-TZ',
    cr.text = 'Kilimanjaro/Serengeti Pride',
    cr.importance = 'medium',
    cr.expression = 'Natural wonders; Zanzibar heritage; tourism identity',
    cr.marketing_angle = 'Natural beauty imagery; heritage pride',
    cr.display_name = 'Kilimanjaro/Serengeti Pride',
    cr.content = 'Natural wonders; Zanzibar heritage; tourism identity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sw-TZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:heshima-respect@sw-TZ'})
SET cr.locale = 'sw-TZ',
    cr.text = 'Heshima (Respect)',
    cr.importance = 'high',
    cr.expression = 'Respect for elders and authority; polite communication norms',
    cr.marketing_angle = 'Respectful, humble brand voice; avoid aggressive marketing',
    cr.display_name = 'Heshima (Respect)',
    cr.content = 'Respect for elders and authority; polite communication norms',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@sw-TZ'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ancient-heritage@am-ET'})
SET cr.locale = 'am-ET',
    cr.text = 'ጥንታዊ ታሪክ (Ancient Heritage)',
    cr.importance = 'critical',
    cr.expression = '3000+ years of history; never colonized; Axumite civilization; Ark of Covenant',
    cr.marketing_angle = 'Heritage and pride messaging; respect for ancient civilization',
    cr.display_name = 'ጥንታዊ ታሪክ (Ancient Heritage)',
    cr.content = '3000+ years of history; never colonized; Axumite civilization; Ark of Covenant',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@am-ET'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:orthodox-christianity@am-ET'})
SET cr.locale = 'am-ET',
    cr.text = 'ኦርቶዶክስ ክርስትና (Orthodox Christianity)',
    cr.importance = 'critical',
    cr.expression = 'Ethiopian Orthodox Church central to identity; fasting periods; unique Christianity',
    cr.marketing_angle = 'Respect fasting seasons; religious calendar awareness essential',
    cr.display_name = 'ኦርቶዶክስ ክርስትና (Orthodox Christianity)',
    cr.content = 'Ethiopian Orthodox Church central to identity; fasting periods; unique Christianity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@am-ET'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:buna-coffee-culture@am-ET'})
SET cr.locale = 'am-ET',
    cr.text = 'ቡና (Buna - Coffee Culture)',
    cr.importance = 'high',
    cr.expression = 'Ethiopia is birthplace of coffee; coffee ceremony is social ritual',
    cr.marketing_angle = 'Coffee imagery powerful; ceremony as community metaphor',
    cr.display_name = 'ቡና (Buna - Coffee Culture)',
    cr.content = 'Ethiopia is birthplace of coffee; coffee ceremony is social ritual',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@am-ET'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:family-bonds@am-ET'})
SET cr.locale = 'am-ET',
    cr.text = 'የቤተሰብ ትስስር (Family Bonds)',
    cr.importance = 'high',
    cr.expression = 'Extended family central; collective decision-making; elder respect',
    cr.marketing_angle = 'Family and community messaging; multi-generational appeal',
    cr.display_name = 'የቤተሰብ ትስስር (Family Bonds)',
    cr.content = 'Extended family central; collective decision-making; elder respect',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@am-ET'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:filotimo-honor-dignity@el-GR'})
SET cr.locale = 'el-GR',
    cr.text = 'Φιλότιμο (Filotimo - Honor/Dignity)',
    cr.importance = 'critical',
    cr.expression = 'Untranslatable virtue combining honor, dignity, pride, and doing the right thing',
    cr.marketing_angle = 'Quality and integrity messaging; products worthy of philotimo',
    cr.display_name = 'Φιλότιμο (Filotimo - Honor/Dignity)',
    cr.content = 'Untranslatable virtue combining honor, dignity, pride, and doing the right thing',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@el-GR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:filoxenia-hospitality@el-GR'})
SET cr.locale = 'el-GR',
    cr.text = 'Φιλοξενία (Filoxenia - Hospitality)',
    cr.importance = 'high',
    cr.expression = 'Love of strangers/guests; sacred duty to welcome; generous hosting',
    cr.marketing_angle = 'Welcoming, generous brand positioning; hospitality imagery',
    cr.display_name = 'Φιλοξενία (Filoxenia - Hospitality)',
    cr.content = 'Love of strangers/guests; sacred duty to welcome; generous hosting',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@el-GR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:oikogeneia-family@el-GR'})
SET cr.locale = 'el-GR',
    cr.text = 'Οικογένεια (Oikogeneia - Family)',
    cr.importance = 'high',
    cr.expression = 'Extended family bonds; Sunday family meals; children highly valued',
    cr.marketing_angle = 'Family-oriented messaging; multi-generational products',
    cr.display_name = 'Οικογένεια (Oikogeneia - Family)',
    cr.content = 'Extended family bonds; Sunday family meals; children highly valued',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@el-GR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:ancient-heritage@el-GR'})
SET cr.locale = 'el-GR',
    cr.text = 'Αρχαία κληρονομιά (Ancient Heritage)',
    cr.importance = 'high',
    cr.expression = 'Pride in ancient Greek civilization; democracy, philosophy, Olympics origins',
    cr.marketing_angle = 'Heritage references can work; classical quality associations',
    cr.display_name = 'Αρχαία κληρονομιά (Ancient Heritage)',
    cr.content = 'Pride in ancient Greek civilization; democracy, philosophy, Olympics origins',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@el-GR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:cypriot-identity@el-CY'})
SET cr.locale = 'el-CY',
    cr.text = 'Κυπριακή ταυτότητα (Cypriot Identity)',
    cr.importance = 'high',
    cr.expression = 'Distinct from Greece; unique Cypriot Greek dialect; island identity',
    cr.marketing_angle = 'Acknowledge Cypriot distinctiveness; not just \'Greek\'',
    cr.display_name = 'Κυπριακή ταυτότητα (Cypriot Identity)',
    cr.content = 'Distinct from Greece; unique Cypriot Greek dialect; island identity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@el-CY'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hospitality@el-CY'})
SET cr.locale = 'el-CY',
    cr.text = 'Φιλοξενία (Hospitality)',
    cr.importance = 'high',
    cr.expression = 'Mediterranean hospitality; coffee culture; generous hosting',
    cr.marketing_angle = 'Welcoming, warm brand positioning',
    cr.display_name = 'Φιλοξενία (Hospitality)',
    cr.content = 'Mediterranean hospitality; coffee culture; generous hosting',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@el-CY'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:community@el-CY'})
SET cr.locale = 'el-CY',
    cr.text = 'Κοινότητα (Community)',
    cr.importance = 'high',
    cr.expression = 'Small island; everyone knows everyone; community bonds strong',
    cr.marketing_angle = 'Community and local connection; word-of-mouth important',
    cr.display_name = 'Κοινότητα (Community)',
    cr.content = 'Small island; everyone knows everyone; community bonds strong',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@el-CY'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:sun-and-sea@el-CY'})
SET cr.locale = 'el-CY',
    cr.text = 'Ήλιος και θάλασσα (Sun and Sea)',
    cr.importance = 'medium',
    cr.expression = 'Mediterranean lifestyle; beach culture; outdoor living',
    cr.marketing_angle = 'Lifestyle imagery; outdoor/leisure associations',
    cr.display_name = 'Ήλιος και θάλασσα (Sun and Sea)',
    cr.content = 'Mediterranean lifestyle; beach culture; outdoor living',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@el-CY'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:elegance-intellectuelle@fr-FR'})
SET cr.locale = 'fr-FR',
    cr.text = 'Élégance intellectuelle',
    cr.importance = 'high',
    cr.expression = 'French audiences value sophisticated, well-crafted language with literary flair',
    cr.marketing_angle = 'Use refined vocabulary, avoid oversimplification, intellectual rigor signals quality',
    cr.display_name = 'Élégance intellectuelle',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-FR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'French audiences value sophisticated, well-crafted language with literary flair',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-FR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:patrimoine-et-tradition@fr-FR'})
SET cr.locale = 'fr-FR',
    cr.text = 'Patrimoine et tradition',
    cr.importance = 'high',
    cr.expression = 'Deep connection to French heritage, art de vivre, savoir-faire',
    cr.marketing_angle = 'Reference established expertise, longevity, craftsmanship over novelty',
    cr.display_name = 'Patrimoine et tradition',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-FR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Deep connection to French heritage, art de vivre, savoir-faire',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-FR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:laicite@fr-FR'})
SET cr.locale = 'fr-FR',
    cr.text = 'Laïcité',
    cr.importance = 'medium',
    cr.expression = 'Strong separation of religion and public life, secularism as national identity',
    cr.marketing_angle = 'Avoid religious references, focus on universal humanist values',
    cr.display_name = 'Laïcité',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-FR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Strong separation of religion and public life, secularism as national identity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-FR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:exception-culturelle-francaise@fr-FR'})
SET cr.locale = 'fr-FR',
    cr.text = 'Exception culturelle française',
    cr.importance = 'medium',
    cr.expression = 'Pride in French uniqueness, resistance to cultural homogenization',
    cr.marketing_angle = 'Position as French-adapted, not American import; highlight local relevance',
    cr.display_name = 'Exception culturelle française',
    cr.content = 'Pride in French uniqueness, resistance to cultural homogenization',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-FR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:art-de-vivre@fr-FR'})
SET cr.locale = 'fr-FR',
    cr.text = 'Art de vivre',
    cr.importance = 'high',
    cr.expression = 'Quality of life, gastronomy, aesthetics as daily practice',
    cr.marketing_angle = 'Frame products as enhancing life quality, not just productivity',
    cr.display_name = 'Art de vivre',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-FR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Quality of life, gastronomy, aesthetics as daily practice',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-FR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:fierte-quebecoise@fr-CA'})
SET cr.locale = 'fr-CA',
    cr.text = 'Fierté québécoise',
    cr.importance = 'high',
    cr.expression = 'Strong Quebecois identity distinct from both France and English Canada',
    cr.marketing_angle = 'Use Quebec French vocabulary (not Parisian), acknowledge distinct identity',
    cr.display_name = 'Fierté québécoise',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-CA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Strong Quebecois identity distinct from both France and English Canada',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:survivance-culturelle@fr-CA'})
SET cr.locale = 'fr-CA',
    cr.text = 'Survivance culturelle',
    cr.importance = 'high',
    cr.expression = 'Pride in maintaining French language in North American context',
    cr.marketing_angle = 'Support language preservation, avoid anglicisms that feel like erasure',
    cr.display_name = 'Survivance culturelle',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-CA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Pride in maintaining French language in North American context',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:convivialite-nord-americaine@fr-CA'})
SET cr.locale = 'fr-CA',
    cr.text = 'Convivialité nord-américaine',
    cr.importance = 'medium',
    cr.expression = 'Warmer, more casual than French formality; North American friendliness',
    cr.marketing_angle = 'Tu form acceptable faster, casual tone works well',
    cr.display_name = 'Convivialité nord-américaine',
    cr.content = 'Warmer, more casual than French formality; North American friendliness',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:entrepreneuriat-local@fr-CA'})
SET cr.locale = 'fr-CA',
    cr.text = 'Entrepreneuriat local',
    cr.importance = 'medium',
    cr.expression = 'Support for local Quebec businesses, buy local movement strong',
    cr.marketing_angle = 'Highlight local presence, Quebec-based support',
    cr.display_name = 'Entrepreneuriat local',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-CA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Support for local Quebec businesses, buy local movement strong',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:joie-de-vivre-nordique@fr-CA'})
SET cr.locale = 'fr-CA',
    cr.text = 'Joie de vivre nordique',
    cr.importance = 'medium',
    cr.expression = 'Celebration of Quebec winters, outdoor culture, festivals',
    cr.marketing_angle = 'Reference local seasons, festivals (St-Jean), outdoor lifestyle',
    cr.display_name = 'Joie de vivre nordique',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-CA. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Celebration of Quebec winters, outdoor culture, festivals',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-CA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:compromis-belge@fr-BE'})
SET cr.locale = 'fr-BE',
    cr.text = 'Compromis belge',
    cr.importance = 'high',
    cr.expression = 'Belgian talent for finding middle ground, consensus culture',
    cr.marketing_angle = 'Avoid absolutist claims, present balanced options',
    cr.display_name = 'Compromis belge',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-BE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Belgian talent for finding middle ground, consensus culture',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-BE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:auto-derision@fr-BE'})
SET cr.locale = 'fr-BE',
    cr.text = 'Auto-dérision',
    cr.importance = 'high',
    cr.expression = 'Self-deprecating humor, not taking oneself too seriously',
    cr.marketing_angle = 'Lighter tone acceptable, modest claims over grandiose',
    cr.display_name = 'Auto-dérision',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-BE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Self-deprecating humor, not taking oneself too seriously',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-BE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:multilinguisme-quotidien@fr-BE'})
SET cr.locale = 'fr-BE',
    cr.text = 'Multilinguisme quotidien',
    cr.importance = 'medium',
    cr.expression = 'Daily navigation between French, Dutch, German; code-switching normal',
    cr.marketing_angle = 'Language choice signals identity; provide French-first but acknowledge multilingual context',
    cr.display_name = 'Multilinguisme quotidien',
    cr.content = 'Daily navigation between French, Dutch, German; code-switching normal',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-BE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:qualite-de-vie-discrete@fr-BE'})
SET cr.locale = 'fr-BE',
    cr.text = 'Qualité de vie discrète',
    cr.importance = 'medium',
    cr.expression = 'Belgian comfort culture: beer, chocolate, frites, quiet prosperity',
    cr.marketing_angle = 'Understated quality over flashy, reliability over innovation hype',
    cr.display_name = 'Qualité de vie discrète',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-BE. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Belgian comfort culture: beer, chocolate, frites, quiet prosperity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-BE'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:precision-suisse@fr-CH'})
SET cr.locale = 'fr-CH',
    cr.text = 'Précision suisse',
    cr.importance = 'high',
    cr.expression = 'Exactitude, punctuality, quality craftsmanship as core values',
    cr.marketing_angle = 'Emphasize precision, reliability, exact specifications',
    cr.display_name = 'Précision suisse',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Exactitude, punctuality, quality craftsmanship as core values',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:neutralite@fr-CH'})
SET cr.locale = 'fr-CH',
    cr.text = 'Neutralité',
    cr.importance = 'high',
    cr.expression = 'Political and ideological neutrality deeply ingrained',
    cr.marketing_angle = 'Avoid political positioning, present balanced information',
    cr.display_name = 'Neutralité',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Political and ideological neutrality deeply ingrained',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:discretion-financiere@fr-CH'})
SET cr.locale = 'fr-CH',
    cr.text = 'Discrétion financière',
    cr.importance = 'high',
    cr.expression = 'Privacy around money matters, discretion valued',
    cr.marketing_angle = 'Subtle value proposition, not price-focused messaging',
    cr.display_name = 'Discrétion financière',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Privacy around money matters, discretion valued',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:federalisme-cantonal@fr-CH'})
SET cr.locale = 'fr-CH',
    cr.text = 'Fédéralisme cantonal',
    cr.importance = 'medium',
    cr.expression = 'Strong local canton identity, decentralized governance',
    cr.marketing_angle = 'Acknowledge regional diversity, avoid Swiss generalization',
    cr.display_name = 'Fédéralisme cantonal',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for fr-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Strong local canton identity, decentralized governance',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fr-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:orgullo-regional@es-ES'})
SET cr.locale = 'es-ES',
    cr.text = 'Orgullo regional',
    cr.importance = 'high',
    cr.expression = 'Strong regional identities: Cataluña, País Vasco, Galicia, Andalucía',
    cr.marketing_angle = 'Avoid Madrid-centric view; acknowledge regional diversity',
    cr.display_name = 'Orgullo regional',
    cr.content = 'Strong regional identities: Cataluña, País Vasco, Galicia, Andalucía',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-ES'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:vida-social-intensa@es-ES'})
SET cr.locale = 'es-ES',
    cr.text = 'Vida social intensa',
    cr.importance = 'high',
    cr.expression = 'Social life, relationships, family gatherings are central',
    cr.marketing_angle = 'Frame products as enabling connection, not isolation',
    cr.display_name = 'Vida social intensa',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-ES. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Social life, relationships, family gatherings are central',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-ES'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:estetica-del-disfrute@es-ES'})
SET cr.locale = 'es-ES',
    cr.text = 'Estética del disfrute',
    cr.importance = 'medium',
    cr.expression = 'Enjoying life (tapas, sobremesa, siesta culture)',
    cr.marketing_angle = 'Quality of experience over efficiency metrics',
    cr.display_name = 'Estética del disfrute',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-ES. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Enjoying life (tapas, sobremesa, siesta culture)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-ES'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:autenticidad-castiza@es-ES'})
SET cr.locale = 'es-ES',
    cr.text = 'Autenticidad castiza',
    cr.importance = 'medium',
    cr.expression = 'Appreciation for authentic Spanish character, traditions',
    cr.marketing_angle = 'Genuine positioning over manufactured hype',
    cr.display_name = 'Autenticidad castiza',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-ES. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Appreciation for authentic Spanish character, traditions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-ES'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:orgullo-mexicano@es-MX'})
SET cr.locale = 'es-MX',
    cr.text = 'Orgullo mexicano',
    cr.importance = 'high',
    cr.expression = 'Strong national identity, cultural richness, pre-Hispanic heritage',
    cr.marketing_angle = 'Celebrate Mexican culture, avoid U.S. condescension',
    cr.display_name = 'Orgullo mexicano',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-MX. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Strong national identity, cultural richness, pre-Hispanic heritage',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-MX'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:calidez-familiar@es-MX'})
SET cr.locale = 'es-MX',
    cr.text = 'Calidez familiar',
    cr.importance = 'high',
    cr.expression = 'Family as cornerstone, extended family involvement',
    cr.marketing_angle = 'Family benefit messaging, collective decision-making',
    cr.display_name = 'Calidez familiar',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-MX. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Family as cornerstone, extended family involvement',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-MX'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:creatividad-y-resourcefulness@es-MX'})
SET cr.locale = 'es-MX',
    cr.text = 'Creatividad y resourcefulness',
    cr.importance = 'medium',
    cr.expression = 'Mexican ingenuity (\'hacer de tripas corazón\'), finding solutions',
    cr.marketing_angle = 'Adaptability, flexibility, problem-solving features',
    cr.display_name = 'Creatividad y resourcefulness',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-MX. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Mexican ingenuity (\'hacer de tripas corazón\'), finding solutions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-MX'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:respeto-y-cortesia@es-MX'})
SET cr.locale = 'es-MX',
    cr.text = 'Respeto y cortesía',
    cr.importance = 'high',
    cr.expression = 'Politeness rituals, indirect communication, saving face',
    cr.marketing_angle = 'Warm tone, avoid confrontational language',
    cr.display_name = 'Respeto y cortesía',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-MX. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Politeness rituals, indirect communication, saving face',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-MX'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:pasion-rioplatense@es-AR'})
SET cr.locale = 'es-AR',
    cr.text = 'Pasión rioplatense',
    cr.importance = 'high',
    cr.expression = 'Intense emotional expression, passion in everything (fútbol, tango)',
    cr.marketing_angle = 'Emotional appeals work well, passion language resonates',
    cr.display_name = 'Pasión rioplatense',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-AR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Intense emotional expression, passion in everything (fútbol, tango)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-AR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:cultura-psicoanalitica@es-AR'})
SET cr.locale = 'es-AR',
    cr.text = 'Cultura psicoanalítica',
    cr.importance = 'medium',
    cr.expression = 'Argentina has most psychologists per capita; self-reflection valued',
    cr.marketing_angle = 'Depth over surface, introspection-friendly messaging',
    cr.display_name = 'Cultura psicoanalítica',
    cr.content = 'Argentina has most psychologists per capita; self-reflection valued',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-AR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:orgullo-porteno@es-AR'})
SET cr.locale = 'es-AR',
    cr.text = 'Orgullo porteño',
    cr.importance = 'high',
    cr.expression = 'Buenos Aires identity, European heritage pride, cultural sophistication',
    cr.marketing_angle = 'Sophisticated positioning, cosmopolitan references',
    cr.display_name = 'Orgullo porteño',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-AR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Buenos Aires identity, European heritage pride, cultural sophistication',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-AR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:picardia-criolla@es-AR'})
SET cr.locale = 'es-AR',
    cr.text = 'Picardía criolla',
    cr.importance = 'medium',
    cr.expression = 'Street-smart cleverness, finding angles, informal workarounds',
    cr.marketing_angle = 'Clever solutions, outsmarting complexity',
    cr.display_name = 'Picardía criolla',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-AR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Street-smart cleverness, finding angles, informal workarounds',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-AR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:amabilidad-paisa@es-CO'})
SET cr.locale = 'es-CO',
    cr.text = 'Amabilidad paisa',
    cr.importance = 'high',
    cr.expression = 'Exceptional friendliness, hospitality, warmth in communication',
    cr.marketing_angle = 'Warm, personal tone; relationship-building language',
    cr.display_name = 'Amabilidad paisa',
    cr.content = 'Exceptional friendliness, hospitality, warmth in communication',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-CO'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:emprendimiento-pujante@es-CO'})
SET cr.locale = 'es-CO',
    cr.text = 'Emprendimiento pujante',
    cr.importance = 'high',
    cr.expression = 'Strong entrepreneurial spirit, resilience, self-improvement drive',
    cr.marketing_angle = 'Growth mindset messaging, business enablement',
    cr.display_name = 'Emprendimiento pujante',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-CO. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Strong entrepreneurial spirit, resilience, self-improvement drive',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-CO'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:diversidad-regional@es-CO'})
SET cr.locale = 'es-CO',
    cr.text = 'Diversidad regional',
    cr.importance = 'medium',
    cr.expression = 'Strong regional identities (Costeño, Paisa, Cachaco, Caleño)',
    cr.marketing_angle = 'Avoid Bogotá-only focus, acknowledge regional diversity',
    cr.display_name = 'Diversidad regional',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-CO. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Strong regional identities (Costeño, Paisa, Cachaco, Caleño)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-CO'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:transformacion-positiva@es-CO'})
SET cr.locale = 'es-CO',
    cr.text = 'Transformación positiva',
    cr.importance = 'high',
    cr.expression = 'Pride in Colombia\'s transformation narrative, moving beyond past',
    cr.marketing_angle = 'Future-focused, innovation-ready positioning',
    cr.display_name = 'Transformación positiva',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-CO. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Pride in Colombia\'s transformation narrative, moving beyond past',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-CO'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:pragmatismo-chileno@es-CL'})
SET cr.locale = 'es-CL',
    cr.text = 'Pragmatismo chileno',
    cr.importance = 'high',
    cr.expression = 'Practical, business-oriented, less ceremonial than other LATAM',
    cr.marketing_angle = 'Results-focused messaging, efficiency valued',
    cr.display_name = 'Pragmatismo chileno',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-CL. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Practical, business-oriented, less ceremonial than other LATAM',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-CL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:emprendimiento-tech@es-CL'})
SET cr.locale = 'es-CL',
    cr.text = 'Emprendimiento tech',
    cr.importance = 'high',
    cr.expression = 'Chile as LATAM tech hub (Start-Up Chile), innovation culture',
    cr.marketing_angle = 'Innovation-friendly, early adopter positioning',
    cr.display_name = 'Emprendimiento tech',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-CL. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Chile as LATAM tech hub (Start-Up Chile), innovation culture',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-CL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:chilenidad-distintiva@es-CL'})
SET cr.locale = 'es-CL',
    cr.text = 'Chilenidad distintiva',
    cr.importance = 'medium',
    cr.expression = 'Unique Chilean modismos, distinct from Argentine/Mexican Spanish',
    cr.marketing_angle = 'Use Chilean expressions (cachai, po, al tiro)',
    cr.display_name = 'Chilenidad distintiva',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-CL. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Unique Chilean modismos, distinct from Argentine/Mexican Spanish',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-CL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:estabilidad-valorada@es-CL'})
SET cr.locale = 'es-CL',
    cr.text = 'Estabilidad valorada',
    cr.importance = 'medium',
    cr.expression = 'Appreciation for stability, reliability (post-2019 nuanced)',
    cr.marketing_angle = 'Trust, reliability, proven solutions',
    cr.display_name = 'Estabilidad valorada',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for es-CL. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Appreciation for stability, reliability (post-2019 nuanced)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@es-CL'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:jeitinho-brasileiro@pt-BR'})
SET cr.locale = 'pt-BR',
    cr.text = 'Jeitinho brasileiro',
    cr.importance = 'high',
    cr.expression = 'Creative problem-solving, flexibility, finding workarounds',
    cr.marketing_angle = 'Adaptable solutions, flexibility as feature',
    cr.display_name = 'Jeitinho brasileiro',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for pt-BR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Creative problem-solving, flexibility, finding workarounds',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pt-BR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:calor-humano@pt-BR'})
SET cr.locale = 'pt-BR',
    cr.text = 'Calor humano',
    cr.importance = 'high',
    cr.expression = 'Warmth, physical closeness, emotional expressiveness',
    cr.marketing_angle = 'Personal connection emphasis, warm supportive tone',
    cr.display_name = 'Calor humano',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for pt-BR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Warmth, physical closeness, emotional expressiveness',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pt-BR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:otimismo-resiliente@pt-BR'})
SET cr.locale = 'pt-BR',
    cr.text = 'Otimismo resiliente',
    cr.importance = 'high',
    cr.expression = 'Optimism despite challenges, \'vai dar tudo certo\' mentality',
    cr.marketing_angle = 'Positive framing, solution over problem',
    cr.display_name = 'Otimismo resiliente',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for pt-BR. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Optimism despite challenges, \'vai dar tudo certo\' mentality',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pt-BR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:diversidade-continental@pt-BR'})
SET cr.locale = 'pt-BR',
    cr.text = 'Diversidade continental',
    cr.importance = 'medium',
    cr.expression = 'Brazil\'s continental size = huge regional diversity',
    cr.marketing_angle = 'Avoid Rio/SP only; acknowledge Northeast, South differences',
    cr.display_name = 'Diversidade continental',
    cr.content = 'Brazil\'s continental size = huge regional diversity',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pt-BR'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:saudade@pt-PT'})
SET cr.locale = 'pt-PT',
    cr.text = 'Saudade',
    cr.importance = 'high',
    cr.expression = 'Portuguese melancholy, nostalgia for past glories, bittersweet longing',
    cr.marketing_angle = 'Emotional depth resonates, heritage references work',
    cr.display_name = 'Saudade',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for pt-PT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Portuguese melancholy, nostalgia for past glories, bittersweet longing',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pt-PT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:desenrascanco@pt-PT'})
SET cr.locale = 'pt-PT',
    cr.text = 'Desenrascanço',
    cr.importance = 'high',
    cr.expression = 'Portuguese resourcefulness, improvising solutions with limited means',
    cr.marketing_angle = 'Efficiency, doing more with less, practical tools',
    cr.display_name = 'Desenrascanço',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for pt-PT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Portuguese resourcefulness, improvising solutions with limited means',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pt-PT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:humildade-europeia@pt-PT'})
SET cr.locale = 'pt-PT',
    cr.text = 'Humildade europeia',
    cr.importance = 'medium',
    cr.expression = 'Modest self-presentation, understatement over boasting',
    cr.marketing_angle = 'Subtle claims, avoid American-style hype',
    cr.display_name = 'Humildade europeia',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for pt-PT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Modest self-presentation, understatement over boasting',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pt-PT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:heranca-maritima@pt-PT'})
SET cr.locale = 'pt-PT',
    cr.text = 'Herança marítima',
    cr.importance = 'medium',
    cr.expression = 'Seafaring heritage, exploration history, global connections',
    cr.marketing_angle = 'International reach, global connectivity',
    cr.display_name = 'Herança marítima',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for pt-PT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Seafaring heritage, exploration history, global connections',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@pt-PT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bella-figura@it-IT'})
SET cr.locale = 'it-IT',
    cr.text = 'Bella figura',
    cr.importance = 'high',
    cr.expression = 'Presenting oneself well, aesthetics in everything, style matters',
    cr.marketing_angle = 'Beautiful design, visual excellence, elegant presentation',
    cr.display_name = 'Bella figura',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for it-IT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Presenting oneself well, aesthetics in everything, style matters',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@it-IT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:campanilismo@it-IT'})
SET cr.locale = 'it-IT',
    cr.text = 'Campanilismo',
    cr.importance = 'high',
    cr.expression = 'Strong local city/region identity (Milan vs Rome vs Naples)',
    cr.marketing_angle = 'Avoid North-South stereotypes, acknowledge regional diversity',
    cr.display_name = 'Campanilismo',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for it-IT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Strong local city/region identity (Milan vs Rome vs Naples)',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@it-IT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:qualita-artigianale@it-IT'})
SET cr.locale = 'it-IT',
    cr.text = 'Qualità artigianale',
    cr.importance = 'high',
    cr.expression = 'Artisan quality, Made in Italy pride, craftsmanship',
    cr.marketing_angle = 'Quality over quantity, attention to detail',
    cr.display_name = 'Qualità artigianale',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for it-IT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Artisan quality, Made in Italy pride, craftsmanship',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@it-IT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:famiglia-e-relazioni@it-IT'})
SET cr.locale = 'it-IT',
    cr.text = 'Famiglia e relazioni',
    cr.importance = 'high',
    cr.expression = 'Family networks, personal relationships in business',
    cr.marketing_angle = 'Trust-building, relationship-first approach',
    cr.display_name = 'Famiglia e relazioni',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for it-IT. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Family networks, personal relationships in business',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@it-IT'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:svizzera-italiana@it-CH'})
SET cr.locale = 'it-CH',
    cr.text = 'Svizzera italiana',
    cr.importance = 'high',
    cr.expression = 'Distinct Ticinese identity, neither Italian nor German-Swiss',
    cr.marketing_angle = 'Acknowledge Ticino specificity, not Italy extension',
    cr.display_name = 'Svizzera italiana',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for it-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Distinct Ticinese identity, neither Italian nor German-Swiss',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@it-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:precisione-elvetica@it-CH'})
SET cr.locale = 'it-CH',
    cr.text = 'Precisione elvetica',
    cr.importance = 'high',
    cr.expression = 'Swiss precision values applied to Italian warmth',
    cr.marketing_angle = 'Reliability, exactitude, quality assurance',
    cr.display_name = 'Precisione elvetica',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for it-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Swiss precision values applied to Italian warmth',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@it-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:riservatezza@it-CH'})
SET cr.locale = 'it-CH',
    cr.text = 'Riservatezza',
    cr.importance = 'medium',
    cr.expression = 'More reserved than Italians, Swiss privacy norms',
    cr.marketing_angle = 'Data privacy emphasis, discretion valued',
    cr.display_name = 'Riservatezza',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for it-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'More reserved than Italians, Swiss privacy norms',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@it-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:qualita-di-vita-alpina@it-CH'})
SET cr.locale = 'it-CH',
    cr.text = 'Qualità di vita alpina',
    cr.importance = 'medium',
    cr.expression = 'Alpine lifestyle, nature, outdoor values',
    cr.marketing_angle = 'Work-life balance, sustainable approaches',
    cr.display_name = 'Qualità di vita alpina',
    cr.node_class = 'CultureRef',
    cr.llm_context = 'USE: when generating content for it-CH. CONTEXT: This cultural reference informs content tone and approach.',
    cr.provenance = '{\"source\": \"seed:enriched\", \"version\": \"v0.19.0\"}',
    cr.content = 'Alpine lifestyle, nature, outdoor values',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@it-CH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);


// --- Taboos with full context ---
MERGE (t:Taboo {key: 'taboo:partisan-politics@en-US'})
SET t.locale = 'en-US',
    t.text = 'Partisan Politics',
    t.severity = 'high',
    t.reason = 'Highly polarized political environment; brands face backlash from either side',
    t.alternative = '',
    t.display_name = 'Partisan Politics',
    t.content = 'Highly polarized political environment; brands face backlash from either side',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-US'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:unsubstantiated-health-claims@en-US'})
SET t.locale = 'en-US',
    t.text = 'Unsubstantiated Health Claims',
    t.severity = 'critical',
    t.reason = 'FTC and FDA strictly regulate health-related marketing claims',
    t.alternative = '',
    t.display_name = 'Unsubstantiated Health Claims',
    t.content = 'FTC and FDA strictly regulate health-related marketing claims',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-US'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:negative-framing@en-US'})
SET t.locale = 'en-US',
    t.text = 'Negative Framing',
    t.severity = 'medium',
    t.reason = 'US audiences prefer positive aspiration over fear-based messaging',
    t.alternative = '',
    t.display_name = 'Negative Framing',
    t.content = 'US audiences prefer positive aspiration over fear-based messaging',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-US'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:overt-self-promotion@en-GB'})
SET t.locale = 'en-GB',
    t.text = 'Overt Self-Promotion',
    t.severity = 'high',
    t.reason = 'Seen as gauche and American; modesty is valued',
    t.alternative = '',
    t.display_name = 'Overt Self-Promotion',
    t.content = 'Seen as gauche and American; modesty is valued',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-GB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:exaggerated-claims@en-GB'})
SET t.locale = 'en-GB',
    t.text = 'Exaggerated Claims',
    t.severity = 'medium',
    t.reason = 'ASA strictly enforces truthful advertising; skeptical audience',
    t.alternative = '',
    t.display_name = 'Exaggerated Claims',
    t.content = 'ASA strictly enforces truthful advertising; skeptical audience',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-GB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:direct-royal-criticism@en-GB'})
SET t.locale = 'en-GB',
    t.text = 'Direct Royal Criticism',
    t.severity = 'medium',
    t.reason = 'Complex relationship with monarchy; best avoided in commercial contexts',
    t.alternative = '',
    t.display_name = 'Direct Royal Criticism',
    t.content = 'Complex relationship with monarchy; best avoided in commercial contexts',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-GB'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:elitist-messaging@en-AU'})
SET t.locale = 'en-AU',
    t.text = 'Elitist Messaging',
    t.severity = 'high',
    t.reason = 'Egalitarian culture rejects perceived snobbery or exclusivity',
    t.alternative = '',
    t.display_name = 'Elitist Messaging',
    t.content = 'Egalitarian culture rejects perceived snobbery or exclusivity',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:indigenous-cultural-insensitivity@en-AU'})
SET t.locale = 'en-AU',
    t.text = 'Indigenous Cultural Insensitivity',
    t.severity = 'critical',
    t.reason = 'Strong social awareness of Indigenous rights and history',
    t.alternative = '',
    t.display_name = 'Indigenous Cultural Insensitivity',
    t.content = 'Strong social awareness of Indigenous rights and history',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:hard-sell-tactics@en-AU'})
SET t.locale = 'en-AU',
    t.text = 'Hard Sell Tactics',
    t.severity = 'medium',
    t.reason = 'Australians value authenticity and dislike pushy sales',
    t.alternative = '',
    t.display_name = 'Hard Sell Tactics',
    t.content = 'Australians value authenticity and dislike pushy sales',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-AU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:treating-canada-as-american@en-CA'})
SET t.locale = 'en-CA',
    t.text = 'Treating Canada as American',
    t.severity = 'high',
    t.reason = 'Canadians strongly identify as distinct from Americans',
    t.alternative = '',
    t.display_name = 'Treating Canada as American',
    t.content = 'Canadians strongly identify as distinct from Americans',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:indigenous-first-nations-insensitivity@en-CA'})
SET t.locale = 'en-CA',
    t.text = 'Indigenous/First Nations Insensitivity',
    t.severity = 'critical',
    t.reason = 'Heightened awareness post-TRC; cultural sensitivity required',
    t.alternative = '',
    t.display_name = 'Indigenous/First Nations Insensitivity',
    t.content = 'Heightened awareness post-TRC; cultural sensitivity required',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:excessive-nationalism@en-CA'})
SET t.locale = 'en-CA',
    t.text = 'Excessive Nationalism',
    t.severity = 'medium',
    t.reason = 'Quiet patriotism preferred over flag-waving',
    t.alternative = '',
    t.display_name = 'Excessive Nationalism',
    t.content = 'Quiet patriotism preferred over flag-waving',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-insensitivity@en-IN'})
SET t.locale = 'en-IN',
    t.text = 'Religious Insensitivity',
    t.severity = 'critical',
    t.reason = 'Multi-religious society; sensitivities around Hindu, Muslim, Sikh, Christian themes',
    t.alternative = '',
    t.display_name = 'Religious Insensitivity',
    t.content = 'Multi-religious society; sensitivities around Hindu, Muslim, Sikh, Christian themes',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:caste-system-references@en-IN'})
SET t.locale = 'en-IN',
    t.text = 'Caste System References',
    t.severity = 'critical',
    t.reason = 'Legally prohibited discrimination; socially sensitive',
    t.alternative = '',
    t.display_name = 'Caste System References',
    t.content = 'Legally prohibited discrimination; socially sensitive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:beef-and-pork-in-food-marketing@en-IN'})
SET t.locale = 'en-IN',
    t.text = 'Beef and Pork in Food Marketing',
    t.severity = 'high',
    t.reason = 'Beef offensive to Hindus, pork to Muslims',
    t.alternative = '',
    t.display_name = 'Beef and Pork in Food Marketing',
    t.content = 'Beef offensive to Hindus, pork to Muslims',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:racial-stereotyping@en-SG'})
SET t.locale = 'en-SG',
    t.text = 'Racial Stereotyping',
    t.severity = 'critical',
    t.reason = 'Strict laws against racial incitement; government actively monitors',
    t.alternative = '',
    t.display_name = 'Racial Stereotyping',
    t.content = 'Strict laws against racial incitement; government actively monitors',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:government-criticism@en-SG'})
SET t.locale = 'en-SG',
    t.text = 'Government Criticism',
    t.severity = 'high',
    t.reason = 'Defamation laws strictly enforced; avoid political commentary',
    t.alternative = '',
    t.display_name = 'Government Criticism',
    t.content = 'Defamation laws strictly enforced; avoid political commentary',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-commentary@en-SG'})
SET t.locale = 'en-SG',
    t.text = 'Religious Commentary',
    t.severity = 'high',
    t.reason = 'Multi-religious society with strict harmony laws',
    t.alternative = '',
    t.display_name = 'Religious Commentary',
    t.content = 'Multi-religious society with strict harmony laws',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-SG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:racial-insensitivity@en-ZA'})
SET t.locale = 'en-ZA',
    t.text = 'Racial Insensitivity',
    t.severity = 'critical',
    t.reason = 'Post-apartheid sensitivity; representation and language critical',
    t.alternative = '',
    t.display_name = 'Racial Insensitivity',
    t.content = 'Post-apartheid sensitivity; representation and language critical',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:poverty-exploitation@en-ZA'})
SET t.locale = 'en-ZA',
    t.text = 'Poverty Exploitation',
    t.severity = 'high',
    t.reason = 'High inequality; avoid exploitative or patronizing messaging',
    t.alternative = '',
    t.display_name = 'Poverty Exploitation',
    t.content = 'High inequality; avoid exploitative or patronizing messaging',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:casual-apartheid-references@en-ZA'})
SET t.locale = 'en-ZA',
    t.text = 'Casual Apartheid References',
    t.severity = 'critical',
    t.reason = 'Traumatic history; references require extreme sensitivity',
    t.alternative = '',
    t.display_name = 'Casual Apartheid References',
    t.content = 'Traumatic history; references require extreme sensitivity',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-ZA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:maori-cultural-appropriation@en-NZ'})
SET t.locale = 'en-NZ',
    t.text = 'Maori Cultural Appropriation',
    t.severity = 'critical',
    t.reason = 'Strong legal and cultural protections for Maori intellectual property',
    t.alternative = '',
    t.display_name = 'Maori Cultural Appropriation',
    t.content = 'Strong legal and cultural protections for Maori intellectual property',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:australia-nz-conflation@en-NZ'})
SET t.locale = 'en-NZ',
    t.text = 'Australia-NZ Conflation',
    t.severity = 'high',
    t.reason = 'Strong national identity distinct from Australia',
    t.alternative = '',
    t.display_name = 'Australia-NZ Conflation',
    t.content = 'Strong national identity distinct from Australia',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:greenwashing@en-NZ'})
SET t.locale = 'en-NZ',
    t.text = 'Greenwashing',
    t.severity = 'high',
    t.reason = 'Strong environmental values; fake green claims backfire',
    t.alternative = '',
    t.display_name = 'Greenwashing',
    t.content = 'Strong environmental values; fake green claims backfire',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-NZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:british-irish-conflation@en-IE'})
SET t.locale = 'en-IE',
    t.text = 'British-Irish Conflation',
    t.severity = 'high',
    t.reason = 'Strong national identity; history of British rule',
    t.alternative = '',
    t.display_name = 'British-Irish Conflation',
    t.content = 'Strong national identity; history of British rule',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:northern-ireland-troubles@en-IE'})
SET t.locale = 'en-IE',
    t.text = 'Northern Ireland Troubles',
    t.severity = 'critical',
    t.reason = 'Sensitive historical and ongoing political situation',
    t.alternative = '',
    t.display_name = 'Northern Ireland Troubles',
    t.content = 'Sensitive historical and ongoing political situation',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:irish-stereotypes@en-IE'})
SET t.locale = 'en-IE',
    t.text = 'Irish Stereotypes',
    t.severity = 'medium',
    t.reason = 'Leprechauns, excessive drinking stereotypes offensive',
    t.alternative = '',
    t.display_name = 'Irish Stereotypes',
    t.content = 'Leprechauns, excessive drinking stereotypes offensive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-IE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:causing-loss-of-face@en-PH'})
SET t.locale = 'en-PH',
    t.text = 'Causing Loss of Face',
    t.severity = 'high',
    t.reason = 'Hiya (shame) culture; public criticism devastating',
    t.alternative = '',
    t.display_name = 'Causing Loss of Face',
    t.content = 'Hiya (shame) culture; public criticism devastating',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:colonial-history-insensitivity@en-PH'})
SET t.locale = 'en-PH',
    t.text = 'Colonial History Insensitivity',
    t.severity = 'high',
    t.reason = 'Complex relationship with US and Spanish colonial past',
    t.alternative = '',
    t.display_name = 'Colonial History Insensitivity',
    t.content = 'Complex relationship with US and Spanish colonial past',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:overt-wealth-display@en-PH'})
SET t.locale = 'en-PH',
    t.text = 'Overt Wealth Display',
    t.severity = 'medium',
    t.reason = 'High inequality; flaunting wealth seen as insensitive',
    t.alternative = '',
    t.display_name = 'Overt Wealth Display',
    t.content = 'High inequality; flaunting wealth seen as insensitive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@en-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:nazi-era-references@de-DE'})
SET t.locale = 'de-DE',
    t.text = 'Nazi Era References',
    t.severity = 'critical',
    t.reason = 'Illegal in many contexts; extreme sensitivity',
    t.alternative = '',
    t.display_name = 'Nazi Era References',
    t.content = 'Illegal in many contexts; extreme sensitivity',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-DE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:unsubstantiated-superlatives@de-DE'})
SET t.locale = 'de-DE',
    t.text = 'Unsubstantiated Superlatives',
    t.severity = 'high',
    t.reason = 'German advertising law strictly enforces truthfulness; Wettbewerbszentrale active',
    t.alternative = '',
    t.display_name = 'Unsubstantiated Superlatives',
    t.content = 'German advertising law strictly enforces truthfulness; Wettbewerbszentrale active',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-DE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:pushy-sales-tactics@de-DE'})
SET t.locale = 'de-DE',
    t.text = 'Pushy Sales Tactics',
    t.severity = 'high',
    t.reason = 'Germans value facts over emotion; hard sells backfire',
    t.alternative = '',
    t.display_name = 'Pushy Sales Tactics',
    t.content = 'Germans value facts over emotion; hard sells backfire',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-DE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:german-austrian-conflation@de-AT'})
SET t.locale = 'de-AT',
    t.text = 'German-Austrian Conflation',
    t.severity = 'high',
    t.reason = 'Strong Austrian identity distinct from Germany',
    t.alternative = '',
    t.display_name = 'German-Austrian Conflation',
    t.content = 'Strong Austrian identity distinct from Germany',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-AT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:nazi-era-references@de-AT'})
SET t.locale = 'de-AT',
    t.text = 'Nazi Era References',
    t.severity = 'critical',
    t.reason = 'Complex history; extreme sensitivity; legal restrictions',
    t.alternative = '',
    t.display_name = 'Nazi Era References',
    t.content = 'Complex history; extreme sensitivity; legal restrictions',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-AT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:excessive-informality@de-AT'})
SET t.locale = 'de-AT',
    t.text = 'Excessive Informality',
    t.severity = 'medium',
    t.reason = 'More formal than Germany; titles and Sie form expected longer',
    t.alternative = '',
    t.display_name = 'Excessive Informality',
    t.content = 'More formal than Germany; titles and Sie form expected longer',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-AT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:german-swiss-conflation@de-CH'})
SET t.locale = 'de-CH',
    t.text = 'German-Swiss Conflation',
    t.severity = 'high',
    t.reason = 'Swiss identity distinct; language differences (Swiss German)',
    t.alternative = '',
    t.display_name = 'German-Swiss Conflation',
    t.content = 'Swiss identity distinct; language differences (Swiss German)',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:ostentatious-displays@de-CH'})
SET t.locale = 'de-CH',
    t.text = 'Ostentatious Displays',
    t.severity = 'high',
    t.reason = 'Wealth exists but is kept private; flashiness frowned upon',
    t.alternative = '',
    t.display_name = 'Ostentatious Displays',
    t.content = 'Wealth exists but is kept private; flashiness frowned upon',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:banking-secrecy-jokes@de-CH'})
SET t.locale = 'de-CH',
    t.text = 'Banking Secrecy Jokes',
    t.severity = 'medium',
    t.reason = 'Sensitive given historical controversies; banking evolved',
    t.alternative = '',
    t.display_name = 'Banking Secrecy Jokes',
    t.content = 'Sensitive given historical controversies; banking evolved',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@de-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:pretentious-messaging@nl-NL'})
SET t.locale = 'nl-NL',
    t.text = 'Pretentious Messaging',
    t.severity = 'high',
    t.reason = 'Doe maar gewoon mentality; showing off rejected',
    t.alternative = '',
    t.display_name = 'Pretentious Messaging',
    t.content = 'Doe maar gewoon mentality; showing off rejected',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-NL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:over-formality@nl-NL'})
SET t.locale = 'nl-NL',
    t.text = 'Over-Formality',
    t.severity = 'medium',
    t.reason = 'Egalitarian society; excessive formality seems insincere',
    t.alternative = '',
    t.display_name = 'Over-Formality',
    t.content = 'Egalitarian society; excessive formality seems insincere',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-NL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:royal-family-disrespect@nl-NL'})
SET t.locale = 'nl-NL',
    t.text = 'Royal Family Disrespect',
    t.severity = 'medium',
    t.reason = 'Popular monarchy; affectionate relationship with royals',
    t.alternative = '',
    t.display_name = 'Royal Family Disrespect',
    t.content = 'Popular monarchy; affectionate relationship with royals',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-NL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:dutch-flemish-conflation@nl-BE'})
SET t.locale = 'nl-BE',
    t.text = 'Dutch-Flemish Conflation',
    t.severity = 'high',
    t.reason = 'Flemish is NOT Dutch; language and cultural differences',
    t.alternative = '',
    t.display_name = 'Dutch-Flemish Conflation',
    t.content = 'Flemish is NOT Dutch; language and cultural differences',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:belgian-linguistic-politics@nl-BE'})
SET t.locale = 'nl-BE',
    t.text = 'Belgian Linguistic Politics',
    t.severity = 'high',
    t.reason = 'Sensitive Flemish-Walloon relations; avoid taking sides',
    t.alternative = '',
    t.display_name = 'Belgian Linguistic Politics',
    t.content = 'Sensitive Flemish-Walloon relations; avoid taking sides',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:dutch-style-bluntness@nl-BE'})
SET t.locale = 'nl-BE',
    t.text = 'Dutch-Style Bluntness',
    t.severity = 'medium',
    t.reason = 'Flemish communication is softer than Dutch',
    t.alternative = '',
    t.display_name = 'Dutch-Style Bluntness',
    t.content = 'Flemish communication is softer than Dutch',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@nl-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:self-promotion-elitism@sv-SE'})
SET t.locale = 'sv-SE',
    t.text = 'Self-Promotion/Elitism',
    t.severity = 'high',
    t.reason = 'Jantelagen culture; standing out negatively perceived',
    t.alternative = '',
    t.display_name = 'Self-Promotion/Elitism',
    t.content = 'Jantelagen culture; standing out negatively perceived',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sv-SE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:aggressive-sales@sv-SE'})
SET t.locale = 'sv-SE',
    t.text = 'Aggressive Sales',
    t.severity = 'high',
    t.reason = 'Reserved culture; pushy tactics backfire strongly',
    t.alternative = '',
    t.display_name = 'Aggressive Sales',
    t.content = 'Reserved culture; pushy tactics backfire strongly',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sv-SE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:gender-stereotyping@sv-SE'})
SET t.locale = 'sv-SE',
    t.text = 'Gender Stereotyping',
    t.severity = 'high',
    t.reason = 'Progressive gender equality; stereotypical imagery rejected',
    t.alternative = '',
    t.display_name = 'Gender Stereotyping',
    t.content = 'Progressive gender equality; stereotypical imagery rejected',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sv-SE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:showing-off@da-DK'})
SET t.locale = 'da-DK',
    t.text = 'Showing Off',
    t.severity = 'high',
    t.reason = 'Janteloven applies; visible success displays frowned upon',
    t.alternative = '',
    t.display_name = 'Showing Off',
    t.content = 'Janteloven applies; visible success displays frowned upon',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@da-DK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:excessive-formality@da-DK'})
SET t.locale = 'da-DK',
    t.text = 'Excessive Formality',
    t.severity = 'medium',
    t.reason = 'Very informal society; stiff communication seems fake',
    t.alternative = '',
    t.display_name = 'Excessive Formality',
    t.content = 'Very informal society; stiff communication seems fake',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@da-DK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:swedish-danish-conflation@da-DK'})
SET t.locale = 'da-DK',
    t.text = 'Swedish-Danish Conflation',
    t.severity = 'medium',
    t.reason = 'Rivalry with Sweden; distinct identity important',
    t.alternative = '',
    t.display_name = 'Swedish-Danish Conflation',
    t.content = 'Rivalry with Sweden; distinct identity important',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@da-DK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:wealth-display@no-NO'})
SET t.locale = 'no-NO',
    t.text = 'Wealth Display',
    t.severity = 'high',
    t.reason = 'Despite oil wealth, showing off strongly rejected',
    t.alternative = '',
    t.display_name = 'Wealth Display',
    t.content = 'Despite oil wealth, showing off strongly rejected',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@no-NO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:environmental-hypocrisy@no-NO'})
SET t.locale = 'no-NO',
    t.text = 'Environmental Hypocrisy',
    t.severity = 'high',
    t.reason = 'Oil nation with green values; authenticity required',
    t.alternative = '',
    t.display_name = 'Environmental Hypocrisy',
    t.content = 'Oil nation with green values; authenticity required',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@no-NO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:swedish-comparisons@no-NO'})
SET t.locale = 'no-NO',
    t.text = 'Swedish Comparisons',
    t.severity = 'low',
    t.reason = 'Friendly rivalry; avoid making Norway secondary',
    t.alternative = '',
    t.display_name = 'Swedish Comparisons',
    t.content = 'Friendly rivalry; avoid making Norway secondary',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@no-NO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:excessive-small-talk@fi-FI'})
SET t.locale = 'fi-FI',
    t.text = 'Excessive Small Talk',
    t.severity = 'medium',
    t.reason = 'Finns value directness and substance; chattiness seems insincere',
    t.alternative = '',
    t.display_name = 'Excessive Small Talk',
    t.content = 'Finns value directness and substance; chattiness seems insincere',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fi-FI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:exaggerated-claims@fi-FI'})
SET t.locale = 'fi-FI',
    t.text = 'Exaggerated Claims',
    t.severity = 'high',
    t.reason = 'Highly skeptical audience; overpromising backfires',
    t.alternative = '',
    t.display_name = 'Exaggerated Claims',
    t.content = 'Highly skeptical audience; overpromising backfires',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fi-FI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:swedish-finnish-language-politics@fi-FI'})
SET t.locale = 'fi-FI',
    t.text = 'Swedish-Finnish Language Politics',
    t.severity = 'medium',
    t.reason = 'Swedish minority; sensitive linguistic history',
    t.alternative = '',
    t.display_name = 'Swedish-Finnish Language Politics',
    t.content = 'Swedish minority; sensitive linguistic history',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fi-FI'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:political-sensitivity-taiwan-tibet-xinji@zh-CN'})
SET t.locale = 'zh-CN',
    t.text = 'Political sensitivity (Taiwan, Tibet, Xinjiang)',
    t.severity = 'critical',
    t.reason = 'Legal restrictions and strong nationalist sentiment; can result in brand boycotts',
    t.alternative = '',
    t.display_name = 'Political sensitivity (Taiwan, Tibet, Xinjiang)',
    t.content = 'Legal restrictions and strong nationalist sentiment; can result in brand boycotts',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-CN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:number-4-si@zh-CN'})
SET t.locale = 'zh-CN',
    t.text = 'Number 4 (四 sì)',
    t.severity = 'high',
    t.reason = 'Homophone for death (死 sǐ); avoid in pricing, numbering, product names',
    t.alternative = '',
    t.display_name = 'Number 4 (四 sì)',
    t.content = 'Homophone for death (死 sǐ); avoid in pricing, numbering, product names',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-CN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:japan-related-imagery-in-nationalist-con@zh-CN'})
SET t.locale = 'zh-CN',
    t.text = 'Japan-related imagery in nationalist contexts',
    t.severity = 'medium',
    t.reason = 'Historical tensions; avoid imperial Japanese symbols or insensitive WWII references',
    t.alternative = '',
    t.display_name = 'Japan-related imagery in nationalist contexts',
    t.content = 'Historical tensions; avoid imperial Japanese symbols or insensitive WWII references',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-CN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:implying-taiwan-is-part-of-china@zh-TW'})
SET t.locale = 'zh-TW',
    t.text = 'Implying Taiwan is part of China',
    t.severity = 'critical',
    t.reason = 'Highly sensitive political issue; use \'Taiwan\' independently, never \'Taiwan, China\'',
    t.alternative = '',
    t.display_name = 'Implying Taiwan is part of China',
    t.content = 'Highly sensitive political issue; use \'Taiwan\' independently, never \'Taiwan, China\'',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:ccp-symbolism-or-propaganda-aesthetics@zh-TW'})
SET t.locale = 'zh-TW',
    t.text = 'CCP symbolism or propaganda aesthetics',
    t.severity = 'high',
    t.reason = 'Negative associations with authoritarian governance',
    t.alternative = '',
    t.display_name = 'CCP symbolism or propaganda aesthetics',
    t.content = 'Negative associations with authoritarian governance',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:number-4-si@zh-TW'})
SET t.locale = 'zh-TW',
    t.text = 'Number 4 (四 sì)',
    t.severity = 'medium',
    t.reason = 'Same death association as zh-CN, though slightly less rigid',
    t.alternative = '',
    t.display_name = 'Number 4 (四 sì)',
    t.content = 'Same death association as zh-CN, though slightly less rigid',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-TW'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:political-stance-on-democracy-beijing-re@zh-HK'})
SET t.locale = 'zh-HK',
    t.text = 'Political stance on democracy/Beijing relations',
    t.severity = 'critical',
    t.reason = 'Extremely polarized; brands should remain neutral',
    t.alternative = '',
    t.display_name = 'Political stance on democracy/Beijing relations',
    t.content = 'Extremely polarized; brands should remain neutral',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-HK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:simplified-chinese-characters@zh-HK'})
SET t.locale = 'zh-HK',
    t.text = 'Simplified Chinese characters',
    t.severity = 'medium',
    t.reason = 'Use Traditional Chinese; Simplified associated with mainland influence',
    t.alternative = '',
    t.display_name = 'Simplified Chinese characters',
    t.content = 'Use Traditional Chinese; Simplified associated with mainland influence',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-HK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:number-4@zh-HK'})
SET t.locale = 'zh-HK',
    t.text = 'Number 4',
    t.severity = 'medium',
    t.reason = 'Cantonese \'sei\' also sounds like death; avoid in pricing',
    t.alternative = '',
    t.display_name = 'Number 4',
    t.content = 'Cantonese \'sei\' also sounds like death; avoid in pricing',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@zh-HK'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:direct-confrontation-or-criticism@ja-JP'})
SET t.locale = 'ja-JP',
    t.text = 'Direct confrontation or criticism',
    t.severity = 'critical',
    t.reason = 'Destroys wa (harmony); use indirect suggestions or third-party examples',
    t.alternative = '',
    t.display_name = 'Direct confrontation or criticism',
    t.content = 'Destroys wa (harmony); use indirect suggestions or third-party examples',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ja-JP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:number-4-shi-and-9-ku@ja-JP'})
SET t.locale = 'ja-JP',
    t.text = 'Number 4 (四 shi) and 9 (九 ku)',
    t.severity = 'high',
    t.reason = '4=death (死), 9=suffering (苦); avoid in pricing, packaging quantities',
    t.alternative = '',
    t.display_name = 'Number 4 (四 shi) and 9 (九 ku)',
    t.content = '4=death (死), 9=suffering (苦); avoid in pricing, packaging quantities',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ja-JP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:overly-casual-language-in-business-conte@ja-JP'})
SET t.locale = 'ja-JP',
    t.text = 'Overly casual language in business contexts',
    t.severity = 'high',
    t.reason = 'Keigo (honorific language) expected; casual = disrespectful',
    t.alternative = '',
    t.display_name = 'Overly casual language in business contexts',
    t.content = 'Keigo (honorific language) expected; casual = disrespectful',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ja-JP'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:japan-korea-historical-conflicts@ko-KR'})
SET t.locale = 'ko-KR',
    t.text = 'Japan-Korea historical conflicts',
    t.severity = 'high',
    t.reason = 'Colonial history sensitive; avoid Japanese associations in certain contexts',
    t.alternative = '',
    t.display_name = 'Japan-Korea historical conflicts',
    t.content = 'Colonial history sensitive; avoid Japanese associations in certain contexts',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ko-KR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:red-ink-for-names@ko-KR'})
SET t.locale = 'ko-KR',
    t.text = 'Red ink for names',
    t.severity = 'high',
    t.reason = 'Traditionally used for deceased persons\' names',
    t.alternative = '',
    t.display_name = 'Red ink for names',
    t.content = 'Traditionally used for deceased persons\' names',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ko-KR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:age-hierarchy-disrespect@ko-KR'})
SET t.locale = 'ko-KR',
    t.text = 'Age/hierarchy disrespect',
    t.severity = 'high',
    t.reason = 'Strong Confucian hierarchy; address seniors appropriately',
    t.alternative = '',
    t.display_name = 'Age/hierarchy disrespect',
    t.content = 'Strong Confucian hierarchy; address seniors appropriately',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ko-KR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:cow-beef-references@hi-IN'})
SET t.locale = 'hi-IN',
    t.text = 'Cow/beef references',
    t.severity = 'critical',
    t.reason = 'Sacred animal for majority Hindu population; avoid in food/leather contexts',
    t.alternative = '',
    t.display_name = 'Cow/beef references',
    t.content = 'Sacred animal for majority Hindu population; avoid in food/leather contexts',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@hi-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-insensitivity-hindu-muslim-dyn@hi-IN'})
SET t.locale = 'hi-IN',
    t.text = 'Religious insensitivity (Hindu-Muslim dynamics)',
    t.severity = 'critical',
    t.reason = 'Complex religious diversity; maintain neutrality',
    t.alternative = '',
    t.display_name = 'Religious insensitivity (Hindu-Muslim dynamics)',
    t.content = 'Complex religious diversity; maintain neutrality',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@hi-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:left-hand-usage-in-imagery@hi-IN'})
SET t.locale = 'hi-IN',
    t.text = 'Left hand usage in imagery',
    t.severity = 'medium',
    t.reason = 'Left hand considered impure; show products being used with right hand',
    t.alternative = '',
    t.display_name = 'Left hand usage in imagery',
    t.content = 'Left hand considered impure; show products being used with right hand',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@hi-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-imagery-conflicts@bn-BD'})
SET t.locale = 'bn-BD',
    t.text = 'Religious imagery conflicts',
    t.severity = 'critical',
    t.reason = 'Muslim-majority country; respect Islamic values',
    t.alternative = '',
    t.display_name = 'Religious imagery conflicts',
    t.content = 'Muslim-majority country; respect Islamic values',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-BD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:india-bangladesh-political-tensions@bn-BD'})
SET t.locale = 'bn-BD',
    t.text = 'India-Bangladesh political tensions',
    t.severity = 'medium',
    t.reason = 'Avoid implicit comparisons that diminish Bangladeshi identity',
    t.alternative = '',
    t.display_name = 'India-Bangladesh political tensions',
    t.content = 'Avoid implicit comparisons that diminish Bangladeshi identity',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-BD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:immodest-imagery@bn-BD'})
SET t.locale = 'bn-BD',
    t.text = 'Immodest imagery',
    t.severity = 'high',
    t.reason = 'Conservative Muslim majority; modest dress and behavior in visuals',
    t.alternative = '',
    t.display_name = 'Immodest imagery',
    t.content = 'Conservative Muslim majority; modest dress and behavior in visuals',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@bn-BD'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:hindi-imposition@ta-IN'})
SET t.locale = 'ta-IN',
    t.text = 'Hindi imposition',
    t.severity = 'critical',
    t.reason = 'Strong anti-Hindi sentiment historically; always provide Tamil-first content',
    t.alternative = '',
    t.display_name = 'Hindi imposition',
    t.content = 'Strong anti-Hindi sentiment historically; always provide Tamil-first content',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:caste-references@ta-IN'})
SET t.locale = 'ta-IN',
    t.text = 'Caste references',
    t.severity = 'high',
    t.reason = 'Sensitive topic; Dravidian movement\'s anti-caste stance',
    t.alternative = '',
    t.display_name = 'Caste references',
    t.content = 'Sensitive topic; Dravidian movement\'s anti-caste stance',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-conversion-messaging@ta-IN'})
SET t.locale = 'ta-IN',
    t.text = 'Religious conversion messaging',
    t.severity = 'high',
    t.reason = 'Sensitive religious dynamics between Hindu majority and minorities',
    t.alternative = '',
    t.display_name = 'Religious conversion messaging',
    t.content = 'Sensitive religious dynamics between Hindu majority and minorities',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ta-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:ap-telangana-tensions@te-IN'})
SET t.locale = 'te-IN',
    t.text = 'AP-Telangana tensions',
    t.severity = 'medium',
    t.reason = 'Recent state bifurcation; be neutral between regions',
    t.alternative = '',
    t.display_name = 'AP-Telangana tensions',
    t.content = 'Recent state bifurcation; be neutral between regions',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@te-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:caste-references@te-IN'})
SET t.locale = 'te-IN',
    t.text = 'Caste references',
    t.severity = 'high',
    t.reason = 'Complex caste dynamics; avoid explicit/implicit references',
    t.alternative = '',
    t.display_name = 'Caste references',
    t.content = 'Complex caste dynamics; avoid explicit/implicit references',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@te-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-favoritism@te-IN'})
SET t.locale = 'te-IN',
    t.text = 'Religious favoritism',
    t.severity = 'medium',
    t.reason = 'Diverse religious population; maintain neutrality',
    t.alternative = '',
    t.display_name = 'Religious favoritism',
    t.content = 'Diverse religious population; maintain neutrality',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@te-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:war-references-american-war@vi-VN'})
SET t.locale = 'vi-VN',
    t.text = 'War references (American War)',
    t.severity = 'high',
    t.reason = 'Called \'American War\' in Vietnam; avoid glorifying; sensitive for older generation',
    t.alternative = '',
    t.display_name = 'War references (American War)',
    t.content = 'Called \'American War\' in Vietnam; avoid glorifying; sensitive for older generation',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@vi-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:china-vietnam-tensions@vi-VN'})
SET t.locale = 'vi-VN',
    t.text = 'China-Vietnam tensions',
    t.severity = 'high',
    t.reason = 'South China Sea disputes; avoid perceived Chinese dominance',
    t.alternative = '',
    t.display_name = 'China-Vietnam tensions',
    t.content = 'South China Sea disputes; avoid perceived Chinese dominance',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@vi-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:political-criticism@vi-VN'})
SET t.locale = 'vi-VN',
    t.text = 'Political criticism',
    t.severity = 'critical',
    t.reason = 'One-party state; avoid any political commentary',
    t.alternative = '',
    t.display_name = 'Political criticism',
    t.content = 'One-party state; avoid any political commentary',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@vi-VN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:monarchy-criticism-lese-majeste@th-TH'})
SET t.locale = 'th-TH',
    t.text = 'Monarchy criticism (lèse-majesté)',
    t.severity = 'critical',
    t.reason = 'Illegal; severe prison sentences; never joke about or criticize royalty',
    t.alternative = '',
    t.display_name = 'Monarchy criticism (lèse-majesté)',
    t.content = 'Illegal; severe prison sentences; never joke about or criticize royalty',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@th-TH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:feet-head-touching@th-TH'})
SET t.locale = 'th-TH',
    t.text = 'Feet/head touching',
    t.severity = 'high',
    t.reason = 'Feet are lowest/dirtiest; head is sacred; avoid in imagery',
    t.alternative = '',
    t.display_name = 'Feet/head touching',
    t.content = 'Feet are lowest/dirtiest; head is sacred; avoid in imagery',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@th-TH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:buddha-imagery-misuse@th-TH'})
SET t.locale = 'th-TH',
    t.text = 'Buddha imagery misuse',
    t.severity = 'high',
    t.reason = 'Sacred; don\'t use decoratively or in inappropriate contexts',
    t.alternative = '',
    t.display_name = 'Buddha imagery misuse',
    t.content = 'Sacred; don\'t use decoratively or in inappropriate contexts',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@th-TH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-insensitivity-especially-islam@id-ID'})
SET t.locale = 'id-ID',
    t.text = 'Religious insensitivity (especially Islam)',
    t.severity = 'critical',
    t.reason = 'World\'s largest Muslim population; respect Islamic values and holidays',
    t.alternative = '',
    t.display_name = 'Religious insensitivity (especially Islam)',
    t.content = 'World\'s largest Muslim population; respect Islamic values and holidays',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@id-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:left-hand-usage@id-ID'})
SET t.locale = 'id-ID',
    t.text = 'Left hand usage',
    t.severity = 'medium',
    t.reason = 'Left hand considered unclean; show right hand in product usage',
    t.alternative = '',
    t.display_name = 'Left hand usage',
    t.content = 'Left hand considered unclean; show right hand in product usage',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@id-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:ethnic-religious-tensions@id-ID'})
SET t.locale = 'id-ID',
    t.text = 'Ethnic/religious tensions',
    t.severity = 'high',
    t.reason = 'Complex ethnic diversity; avoid favoritism or stereotypes',
    t.alternative = '',
    t.display_name = 'Ethnic/religious tensions',
    t.content = 'Complex ethnic diversity; avoid favoritism or stereotypes',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@id-ID'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-insensitivity@ms-MY'})
SET t.locale = 'ms-MY',
    t.text = 'Religious insensitivity',
    t.severity = 'critical',
    t.reason = 'Islam is state religion; respect halal requirements and Islamic values',
    t.alternative = '',
    t.display_name = 'Religious insensitivity',
    t.content = 'Islam is state religion; respect halal requirements and Islamic values',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-MY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:ethnic-tensions-bumiputera-policies@ms-MY'})
SET t.locale = 'ms-MY',
    t.text = 'Ethnic tensions (Bumiputera policies)',
    t.severity = 'high',
    t.reason = 'Complex ethnic policies; avoid race-based messaging',
    t.alternative = '',
    t.display_name = 'Ethnic tensions (Bumiputera policies)',
    t.content = 'Complex ethnic policies; avoid race-based messaging',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-MY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:pork-alcohol-in-malay-focused-content@ms-MY'})
SET t.locale = 'ms-MY',
    t.text = 'Pork/alcohol in Malay-focused content',
    t.severity = 'high',
    t.reason = 'Haram for Muslims; separate messaging for different ethnic audiences',
    t.alternative = '',
    t.display_name = 'Pork/alcohol in Malay-focused content',
    t.content = 'Haram for Muslims; separate messaging for different ethnic audiences',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ms-MY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:catholic-church-criticism@fil-PH'})
SET t.locale = 'fil-PH',
    t.text = 'Catholic Church criticism',
    t.severity = 'high',
    t.reason = 'Strong Catholic majority; respect religious institutions',
    t.alternative = '',
    t.display_name = 'Catholic Church criticism',
    t.content = 'Strong Catholic majority; respect religious institutions',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fil-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:colonial-history-insensitivity@fil-PH'})
SET t.locale = 'fil-PH',
    t.text = 'Colonial history insensitivity',
    t.severity = 'medium',
    t.reason = 'Complex Spanish/American colonial history; avoid patronizing',
    t.alternative = '',
    t.display_name = 'Colonial history insensitivity',
    t.content = 'Complex Spanish/American colonial history; avoid patronizing',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fil-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:class-poverty-stereotypes@fil-PH'})
SET t.locale = 'fil-PH',
    t.text = 'Class/poverty stereotypes',
    t.severity = 'medium',
    t.reason = 'Significant inequality; avoid classist messaging',
    t.alternative = '',
    t.display_name = 'Class/poverty stereotypes',
    t.content = 'Significant inequality; avoid classist messaging',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fil-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-criticism-or-comparison@ar-SA'})
SET t.locale = 'ar-SA',
    t.text = 'Religious criticism or comparison',
    t.severity = 'critical',
    t.reason = 'Islam is state religion; blasphemy laws strictly enforced; any perceived disrespect can result in severe legal consequences',
    t.alternative = '',
    t.display_name = 'Religious criticism or comparison',
    t.content = 'Islam is state religion; blasphemy laws strictly enforced; any perceived disrespect can result in severe legal consequences',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:royal-family-criticism@ar-SA'})
SET t.locale = 'ar-SA',
    t.text = 'Royal family criticism',
    t.severity = 'critical',
    t.reason = 'Lese-majeste laws protect royal family; criticism results in imprisonment',
    t.alternative = '',
    t.display_name = 'Royal family criticism',
    t.content = 'Lese-majeste laws protect royal family; criticism results in imprisonment',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:gender-mixing-in-imagery@ar-SA'})
SET t.locale = 'ar-SA',
    t.text = 'Gender mixing in imagery',
    t.severity = 'high',
    t.reason = 'Gender segregation is cultural norm; mixed-gender casual imagery inappropriate',
    t.alternative = '',
    t.display_name = 'Gender mixing in imagery',
    t.content = 'Gender segregation is cultural norm; mixed-gender casual imagery inappropriate',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:alcohol-pork-gambling-references@ar-SA'})
SET t.locale = 'ar-SA',
    t.text = 'Alcohol, pork, gambling references',
    t.severity = 'critical',
    t.reason = 'Haram (forbidden) in Islam; illegal in Saudi Arabia',
    t.alternative = '',
    t.display_name = 'Alcohol, pork, gambling references',
    t.content = 'Haram (forbidden) in Islam; illegal in Saudi Arabia',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-SA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-mockery@ar-EG'})
SET t.locale = 'ar-EG',
    t.text = 'Religious mockery',
    t.severity = 'critical',
    t.reason = 'Muslim-majority with significant Coptic Christian minority; religious harmony is sensitive',
    t.alternative = '',
    t.display_name = 'Religious mockery',
    t.content = 'Muslim-majority with significant Coptic Christian minority; religious harmony is sensitive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-EG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:political-commentary@ar-EG'})
SET t.locale = 'ar-EG',
    t.text = 'Political commentary',
    t.severity = 'high',
    t.reason = 'Political climate is sensitive; avoid references to government or military',
    t.alternative = '',
    t.display_name = 'Political commentary',
    t.content = 'Political climate is sensitive; avoid references to government or military',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-EG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:nile-water-scarcity-politics@ar-EG'})
SET t.locale = 'ar-EG',
    t.text = 'Nile/water scarcity politics',
    t.severity = 'medium',
    t.reason = 'Ethiopian dam dispute is nationally sensitive topic',
    t.alternative = '',
    t.display_name = 'Nile/water scarcity politics',
    t.content = 'Ethiopian dam dispute is nationally sensitive topic',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-EG'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:criticism-of-leadership-or-ruling-famili@ar-AE'})
SET t.locale = 'ar-AE',
    t.text = 'Criticism of leadership or ruling families',
    t.severity = 'critical',
    t.reason = 'Strict cyber laws; any criticism of government can result in deportation or imprisonment',
    t.alternative = '',
    t.display_name = 'Criticism of leadership or ruling families',
    t.content = 'Strict cyber laws; any criticism of government can result in deportation or imprisonment',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-AE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:lgbtq-content@ar-AE'})
SET t.locale = 'ar-AE',
    t.text = 'LGBTQ+ content',
    t.severity = 'critical',
    t.reason = 'Illegal under UAE law; cannot be referenced in any marketing',
    t.alternative = '',
    t.display_name = 'LGBTQ+ content',
    t.content = 'Illegal under UAE law; cannot be referenced in any marketing',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-AE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:regional-political-conflicts@ar-AE'})
SET t.locale = 'ar-AE',
    t.text = 'Regional political conflicts',
    t.severity = 'high',
    t.reason = 'Avoid Qatar, Iran, or Yemen conflict references',
    t.alternative = '',
    t.display_name = 'Regional political conflicts',
    t.content = 'Avoid Qatar, Iran, or Yemen conflict references',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-AE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:western-sahara-sovereignty@ar-MA'})
SET t.locale = 'ar-MA',
    t.text = 'Western Sahara sovereignty',
    t.severity = 'critical',
    t.reason = 'Highly sensitive territorial dispute; always refer to as part of Morocco',
    t.alternative = '',
    t.display_name = 'Western Sahara sovereignty',
    t.content = 'Highly sensitive territorial dispute; always refer to as part of Morocco',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-MA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:monarchy-criticism@ar-MA'})
SET t.locale = 'ar-MA',
    t.text = 'Monarchy criticism',
    t.severity = 'critical',
    t.reason = 'Legal consequences for disrespecting the King',
    t.alternative = '',
    t.display_name = 'Monarchy criticism',
    t.content = 'Legal consequences for disrespecting the King',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-MA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:algeria-relations@ar-MA'})
SET t.locale = 'ar-MA',
    t.text = 'Algeria relations',
    t.severity = 'high',
    t.reason = 'Closed border and diplomatic tensions; avoid comparative references',
    t.alternative = '',
    t.display_name = 'Algeria relations',
    t.content = 'Closed border and diplomatic tensions; avoid comparative references',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ar-MA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:political-criticism-of-leadership@ru-RU'})
SET t.locale = 'ru-RU',
    t.text = 'Political criticism of leadership',
    t.severity = 'critical',
    t.reason = 'Current political climate makes any criticism dangerous; foreign brands should stay neutral',
    t.alternative = '',
    t.display_name = 'Political criticism of leadership',
    t.content = 'Current political climate makes any criticism dangerous; foreign brands should stay neutral',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-RU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:wwii-minimization@ru-RU'})
SET t.locale = 'ru-RU',
    t.text = 'WWII minimization',
    t.severity = 'critical',
    t.reason = 'Great Patriotic War is sacred; 27 million deaths; Victory Day (May 9) sacrosanct',
    t.alternative = '',
    t.display_name = 'WWII minimization',
    t.content = 'Great Patriotic War is sacred; 27 million deaths; Victory Day (May 9) sacrosanct',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-RU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:territorial-sovereignty-questions@ru-RU'})
SET t.locale = 'ru-RU',
    t.text = 'Territorial sovereignty questions',
    t.severity = 'critical',
    t.reason = 'Avoid any references to disputed territories or geopolitical conflicts',
    t.alternative = '',
    t.display_name = 'Territorial sovereignty questions',
    t.content = 'Avoid any references to disputed territories or geopolitical conflicts',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@ru-RU'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:russian-language-use@uk-UA'})
SET t.locale = 'uk-UA',
    t.text = 'Russian language use',
    t.severity = 'critical',
    t.reason = 'Post-2022, using Russian instead of Ukrainian is deeply offensive; seen as erasure',
    t.alternative = '',
    t.display_name = 'Russian language use',
    t.content = 'Post-2022, using Russian instead of Ukrainian is deeply offensive; seen as erasure',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@uk-UA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:soviet-nostalgia@uk-UA'})
SET t.locale = 'uk-UA',
    t.text = 'Soviet nostalgia',
    t.severity = 'critical',
    t.reason = 'USSR symbols banned; Soviet era seen as occupation',
    t.alternative = '',
    t.display_name = 'Soviet nostalgia',
    t.content = 'USSR symbols banned; Soviet era seen as occupation',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@uk-UA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:both-sides-ism-on-conflict@uk-UA'})
SET t.locale = 'uk-UA',
    t.text = 'Both-sides-ism on conflict',
    t.severity = 'critical',
    t.reason = 'Any neutrality on Russian invasion seen as complicity',
    t.alternative = '',
    t.display_name = 'Both-sides-ism on conflict',
    t.content = 'Any neutrality on Russian invasion seen as complicity',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@uk-UA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:wwii-polish-complicity-narratives@pl-PL'})
SET t.locale = 'pl-PL',
    t.text = 'WWII Polish complicity narratives',
    t.severity = 'critical',
    t.reason = 'Illegal to suggest Polish nation was complicit in Holocaust; Poland was victim',
    t.alternative = '',
    t.display_name = 'WWII Polish complicity narratives',
    t.content = 'Illegal to suggest Polish nation was complicit in Holocaust; Poland was victim',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pl-PL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:german-russian-historical-grievances@pl-PL'})
SET t.locale = 'pl-PL',
    t.text = 'German/Russian historical grievances',
    t.severity = 'high',
    t.reason = 'Sensitive historical wounds; avoid trivializing invasions/occupations',
    t.alternative = '',
    t.display_name = 'German/Russian historical grievances',
    t.content = 'Sensitive historical wounds; avoid trivializing invasions/occupations',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pl-PL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:anti-catholic-messaging@pl-PL'})
SET t.locale = 'pl-PL',
    t.text = 'Anti-Catholic messaging',
    t.severity = 'high',
    t.reason = 'Catholic Church deeply influential; ~90% identify as Catholic',
    t.alternative = '',
    t.display_name = 'Anti-Catholic messaging',
    t.content = 'Catholic Church deeply influential; ~90% identify as Catholic',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pl-PL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:communist-era-glorification@cs-CZ'})
SET t.locale = 'cs-CZ',
    t.text = 'Communist era glorification',
    t.severity = 'high',
    t.reason = 'Negative associations with Soviet occupation; Velvet Revolution pride',
    t.alternative = '',
    t.display_name = 'Communist era glorification',
    t.content = 'Negative associations with Soviet occupation; Velvet Revolution pride',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@cs-CZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:excessive-nationalism@cs-CZ'})
SET t.locale = 'cs-CZ',
    t.text = 'Excessive nationalism',
    t.severity = 'medium',
    t.reason = 'Czechs are patriotic but uncomfortable with flag-waving nationalism',
    t.alternative = '',
    t.display_name = 'Excessive nationalism',
    t.content = 'Czechs are patriotic but uncomfortable with flag-waving nationalism',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@cs-CZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-proselytizing@cs-CZ'})
SET t.locale = 'cs-CZ',
    t.text = 'Religious proselytizing',
    t.severity = 'medium',
    t.reason = 'One of world\'s most secular countries; religious messaging falls flat',
    t.alternative = '',
    t.display_name = 'Religious proselytizing',
    t.content = 'One of world\'s most secular countries; religious messaging falls flat',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@cs-CZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:ataturk-criticism-or-disrespect@tr-TR'})
SET t.locale = 'tr-TR',
    t.text = 'Atatürk criticism or disrespect',
    t.severity = 'critical',
    t.reason = 'Illegal to insult Atatürk\'s memory; deeply offensive culturally',
    t.alternative = '',
    t.display_name = 'Atatürk criticism or disrespect',
    t.content = 'Illegal to insult Atatürk\'s memory; deeply offensive culturally',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@tr-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:armenian-genocide-acknowledgment@tr-TR'})
SET t.locale = 'tr-TR',
    t.text = 'Armenian Genocide acknowledgment',
    t.severity = 'critical',
    t.reason = 'Official Turkish position denies genocide designation; avoid entirely',
    t.alternative = '',
    t.display_name = 'Armenian Genocide acknowledgment',
    t.content = 'Official Turkish position denies genocide designation; avoid entirely',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@tr-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:kurdish-political-issues@tr-TR'})
SET t.locale = 'tr-TR',
    t.text = 'Kurdish political issues',
    t.severity = 'critical',
    t.reason = 'Highly sensitive; PKK designated as terrorist organization',
    t.alternative = '',
    t.display_name = 'Kurdish political issues',
    t.content = 'Highly sensitive; PKK designated as terrorist organization',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@tr-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:cyprus-greek-tensions@tr-TR'})
SET t.locale = 'tr-TR',
    t.text = 'Cyprus/Greek tensions',
    t.severity = 'high',
    t.reason = 'Ongoing territorial and political disputes',
    t.alternative = '',
    t.display_name = 'Cyprus/Greek tensions',
    t.content = 'Ongoing territorial and political disputes',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@tr-TR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:armenia-nagorno-karabakh@az-AZ'})
SET t.locale = 'az-AZ',
    t.text = 'Armenia/Nagorno-Karabakh',
    t.severity = 'critical',
    t.reason = 'Recent war (2020); deep animosity; Armenian references strictly avoided',
    t.alternative = '',
    t.display_name = 'Armenia/Nagorno-Karabakh',
    t.content = 'Recent war (2020); deep animosity; Armenian references strictly avoided',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@az-AZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:territorial-integrity-questions@az-AZ'})
SET t.locale = 'az-AZ',
    t.text = 'Territorial integrity questions',
    t.severity = 'critical',
    t.reason = 'Karabakh is Azerbaijan; any suggestion otherwise unacceptable',
    t.alternative = '',
    t.display_name = 'Territorial integrity questions',
    t.content = 'Karabakh is Azerbaijan; any suggestion otherwise unacceptable',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@az-AZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:iranian-azeri-politics@az-AZ'})
SET t.locale = 'az-AZ',
    t.text = 'Iranian Azeri politics',
    t.severity = 'high',
    t.reason = 'Complex relationship with Iran; more Azeris in Iran than Azerbaijan',
    t.alternative = '',
    t.display_name = 'Iranian Azeri politics',
    t.content = 'Complex relationship with Iran; more Azeris in Iran than Azerbaijan',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@az-AZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:palestinian-conflict-politics@he-IL'})
SET t.locale = 'he-IL',
    t.text = 'Palestinian conflict politics',
    t.severity = 'critical',
    t.reason = 'Highly divisive internally; brands should stay completely neutral',
    t.alternative = '',
    t.display_name = 'Palestinian conflict politics',
    t.content = 'Highly divisive internally; brands should stay completely neutral',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@he-IL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:holocaust-trivialization@he-IL'})
SET t.locale = 'he-IL',
    t.text = 'Holocaust trivialization',
    t.severity = 'critical',
    t.reason = 'Shoah is sacred memory; never use for marketing analogies',
    t.alternative = '',
    t.display_name = 'Holocaust trivialization',
    t.content = 'Shoah is sacred memory; never use for marketing analogies',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@he-IL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:shabbat-business-operations@he-IL'})
SET t.locale = 'he-IL',
    t.text = 'Shabbat business operations',
    t.severity = 'high',
    t.reason = 'Friday sunset to Saturday sunset; many don\'t transact; respect observance',
    t.alternative = '',
    t.display_name = 'Shabbat business operations',
    t.content = 'Friday sunset to Saturday sunset; many don\'t transact; respect observance',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@he-IL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:tribal-ethnic-politics@sw-KE'})
SET t.locale = 'sw-KE',
    t.text = 'Tribal/ethnic politics',
    t.severity = 'critical',
    t.reason = 'Ethnic tensions can be volatile; 2007 violence memory; avoid tribal references',
    t.alternative = '',
    t.display_name = 'Tribal/ethnic politics',
    t.content = 'Ethnic tensions can be volatile; 2007 violence memory; avoid tribal references',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-KE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:corruption-accusations@sw-KE'})
SET t.locale = 'sw-KE',
    t.text = 'Corruption accusations',
    t.severity = 'high',
    t.reason = 'Sensitive topic; avoid implying corruption in messaging',
    t.alternative = '',
    t.display_name = 'Corruption accusations',
    t.content = 'Sensitive topic; avoid implying corruption in messaging',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-KE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:colonial-era-glorification@sw-KE'})
SET t.locale = 'sw-KE',
    t.text = 'Colonial era glorification',
    t.severity = 'high',
    t.reason = 'British colonial history sensitive; independence hard-won',
    t.alternative = '',
    t.display_name = 'Colonial era glorification',
    t.content = 'British colonial history sensitive; independence hard-won',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-KE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:zanzibar-independence-movements@sw-TZ'})
SET t.locale = 'sw-TZ',
    t.text = 'Zanzibar independence movements',
    t.severity = 'high',
    t.reason = 'Zanzibar union with mainland sensitive; separatism taboo',
    t.alternative = '',
    t.display_name = 'Zanzibar independence movements',
    t.content = 'Zanzibar union with mainland sensitive; separatism taboo',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-TZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-tensions@sw-TZ'})
SET t.locale = 'sw-TZ',
    t.text = 'Religious tensions',
    t.severity = 'high',
    t.reason = 'Muslim Zanzibar, Christian mainland; religious harmony important',
    t.alternative = '',
    t.display_name = 'Religious tensions',
    t.content = 'Muslim Zanzibar, Christian mainland; religious harmony important',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-TZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:criticism-of-founding-father-nyerere@sw-TZ'})
SET t.locale = 'sw-TZ',
    t.text = 'Criticism of founding father Nyerere',
    t.severity = 'medium',
    t.reason = 'Mwalimu (Teacher) Nyerere highly respected despite economic failures',
    t.alternative = '',
    t.display_name = 'Criticism of founding father Nyerere',
    t.content = 'Mwalimu (Teacher) Nyerere highly respected despite economic failures',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@sw-TZ'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:colonialism-comparisons@am-ET'})
SET t.locale = 'am-ET',
    t.text = 'Colonialism comparisons',
    t.severity = 'critical',
    t.reason = 'Ethiopia was never colonized (except brief Italian occupation); unique African history',
    t.alternative = '',
    t.display_name = 'Colonialism comparisons',
    t.content = 'Ethiopia was never colonized (except brief Italian occupation); unique African history',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@am-ET'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:religious-disrespect@am-ET'})
SET t.locale = 'am-ET',
    t.text = 'Religious disrespect',
    t.severity = 'critical',
    t.reason = 'Orthodox Christianity (45%) and Islam (35%) both sensitive; religious harmony valued',
    t.alternative = '',
    t.display_name = 'Religious disrespect',
    t.content = 'Orthodox Christianity (45%) and Islam (35%) both sensitive; religious harmony valued',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@am-ET'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:ethnic-federalism-politics@am-ET'})
SET t.locale = 'am-ET',
    t.text = 'Ethnic federalism politics',
    t.severity = 'critical',
    t.reason = 'Recent Tigray conflict; ethnic tensions; avoid regional/ethnic references',
    t.alternative = '',
    t.display_name = 'Ethnic federalism politics',
    t.content = 'Recent Tigray conflict; ethnic tensions; avoid regional/ethnic references',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@am-ET'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:nile-gerd-dam-politics@am-ET'})
SET t.locale = 'am-ET',
    t.text = 'Nile/GERD dam politics',
    t.severity = 'high',
    t.reason = 'Grand Ethiopian Renaissance Dam is national pride; Egypt disputes sensitive',
    t.alternative = '',
    t.display_name = 'Nile/GERD dam politics',
    t.content = 'Grand Ethiopian Renaissance Dam is national pride; Egypt disputes sensitive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@am-ET'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:macedonia-naming-dispute@el-GR'})
SET t.locale = 'el-GR',
    t.text = 'Macedonia naming dispute',
    t.severity = 'high',
    t.reason = 'North Macedonia naming still sensitive for many; ancient Macedon is Greek',
    t.alternative = '',
    t.display_name = 'Macedonia naming dispute',
    t.content = 'North Macedonia naming still sensitive for many; ancient Macedon is Greek',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-GR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:turkey-cyprus-conflicts@el-GR'})
SET t.locale = 'el-GR',
    t.text = 'Turkey/Cyprus conflicts',
    t.severity = 'high',
    t.reason = 'Historical and ongoing tensions; avoid Turkish comparisons',
    t.alternative = '',
    t.display_name = 'Turkey/Cyprus conflicts',
    t.content = 'Historical and ongoing tensions; avoid Turkish comparisons',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-GR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:economic-crisis-mockery@el-GR'})
SET t.locale = 'el-GR',
    t.text = 'Economic crisis mockery',
    t.severity = 'high',
    t.reason = '2010s crisis painful; avoid lazy/profligate Greek stereotypes',
    t.alternative = '',
    t.display_name = 'Economic crisis mockery',
    t.content = '2010s crisis painful; avoid lazy/profligate Greek stereotypes',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-GR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:turkish-occupation-of-north-cyprus@el-CY'})
SET t.locale = 'el-CY',
    t.text = 'Turkish occupation of North Cyprus',
    t.severity = 'critical',
    t.reason = '1974 invasion and ongoing division deeply traumatic; Turkish Republic of Northern Cyprus not recognized',
    t.alternative = '',
    t.display_name = 'Turkish occupation of North Cyprus',
    t.content = '1974 invasion and ongoing division deeply traumatic; Turkish Republic of Northern Cyprus not recognized',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-CY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:reunification-politics@el-CY'})
SET t.locale = 'el-CY',
    t.text = 'Reunification politics',
    t.severity = 'high',
    t.reason = 'Complex political issue; avoid taking sides on federation proposals',
    t.alternative = '',
    t.display_name = 'Reunification politics',
    t.content = 'Complex political issue; avoid taking sides on federation proposals',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-CY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:implying-cyprus-is-part-of-greece@el-CY'})
SET t.locale = 'el-CY',
    t.text = 'Implying Cyprus is part of Greece',
    t.severity = 'medium',
    t.reason = 'Cyprus is independent republic since 1960; distinct identity important',
    t.alternative = '',
    t.display_name = 'Implying Cyprus is part of Greece',
    t.content = 'Cyprus is independent republic since 1960; distinct identity important',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@el-CY'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:aggressive-sales-tactics@fr-FR'})
SET t.locale = 'fr-FR',
    t.text = 'Aggressive sales tactics',
    t.severity = 'high',
    t.reason = 'Perceived as vulgar and American; French prefer subtle persuasion',
    t.alternative = '',
    t.display_name = 'Aggressive sales tactics',
    t.content = 'Perceived as vulgar and American; French prefer subtle persuasion',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-FR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:money-wealth-display@fr-FR'})
SET t.locale = 'fr-FR',
    t.text = 'Money/wealth display',
    t.severity = 'medium',
    t.reason = 'Overt wealth talk is considered gauche; focus on value, not price savings',
    t.alternative = '',
    t.display_name = 'Money/wealth display',
    t.content = 'Overt wealth talk is considered gauche; focus on value, not price savings',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-FR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:simplistic-language@fr-FR'})
SET t.locale = 'fr-FR',
    t.text = 'Simplistic language',
    t.severity = 'medium',
    t.reason = 'Dumbing down content insults intelligence; maintain linguistic sophistication',
    t.alternative = '',
    t.display_name = 'Simplistic language',
    t.content = 'Dumbing down content insults intelligence; maintain linguistic sophistication',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-FR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:france-quebec-comparisons@fr-CA'})
SET t.locale = 'fr-CA',
    t.text = 'France-Quebec comparisons',
    t.severity = 'high',
    t.reason = 'Implying Quebec French is inferior to Parisian French is deeply offensive',
    t.alternative = '',
    t.display_name = 'France-Quebec comparisons',
    t.content = 'Implying Quebec French is inferior to Parisian French is deeply offensive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:parisian-french-vocabulary@fr-CA'})
SET t.locale = 'fr-CA',
    t.text = 'Parisian French vocabulary',
    t.severity = 'medium',
    t.reason = 'Using France-specific terms (portable for cellulaire, courriel not e-mail) feels foreign',
    t.alternative = '',
    t.display_name = 'Parisian French vocabulary',
    t.content = 'Using France-specific terms (portable for cellulaire, courriel not e-mail) feels foreign',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:ignoring-bill-101@fr-CA'})
SET t.locale = 'fr-CA',
    t.text = 'Ignoring Bill 101',
    t.severity = 'high',
    t.reason = 'French-first is law; English-dominant materials signal disrespect',
    t.alternative = '',
    t.display_name = 'Ignoring Bill 101',
    t.content = 'French-first is law; English-dominant materials signal disrespect',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:flamand-wallon-tensions@fr-BE'})
SET t.locale = 'fr-BE',
    t.text = 'Flamand-Wallon tensions',
    t.severity = 'high',
    t.reason = 'Linguistic community conflicts are politically sensitive; stay neutral',
    t.alternative = '',
    t.display_name = 'Flamand-Wallon tensions',
    t.content = 'Linguistic community conflicts are politically sensitive; stay neutral',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:french-superiority-claims@fr-BE'})
SET t.locale = 'fr-BE',
    t.text = 'French superiority claims',
    t.severity = 'medium',
    t.reason = 'Belgians are sensitive to being treated as provincial French',
    t.alternative = '',
    t.display_name = 'French superiority claims',
    t.content = 'Belgians are sensitive to being treated as provincial French',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:ignoring-belgian-specificity@fr-BE'})
SET t.locale = 'fr-BE',
    t.text = 'Ignoring Belgian specificity',
    t.severity = 'medium',
    t.reason = 'Using \'septante\' not \'soixante-dix\' shows respect for Belgian French',
    t.alternative = '',
    t.display_name = 'Ignoring Belgian specificity',
    t.content = 'Using \'septante\' not \'soixante-dix\' shows respect for Belgian French',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-BE'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:tax-discussions@fr-CH'})
SET t.locale = 'fr-CH',
    t.text = 'Tax discussions',
    t.severity = 'high',
    t.reason = 'Financial privacy is sacrosanct; avoid money-related assumptions',
    t.alternative = '',
    t.display_name = 'Tax discussions',
    t.content = 'Financial privacy is sacrosanct; avoid money-related assumptions',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:rushing-decisions@fr-CH'})
SET t.locale = 'fr-CH',
    t.text = 'Rushing decisions',
    t.severity = 'medium',
    t.reason = 'Swiss deliberation valued; avoid urgency pressure tactics',
    t.alternative = '',
    t.display_name = 'Rushing decisions',
    t.content = 'Swiss deliberation valued; avoid urgency pressure tactics',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:franco-centricite@fr-CH'})
SET t.locale = 'fr-CH',
    t.text = 'Franco-centricité',
    t.severity = 'medium',
    t.reason = 'Swiss French has its own character; not a French colony',
    t.alternative = '',
    t.display_name = 'Franco-centricité',
    t.content = 'Swiss French has its own character; not a French colony',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fr-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:regional-political-conflicts@es-ES'})
SET t.locale = 'es-ES',
    t.text = 'Regional political conflicts',
    t.severity = 'high',
    t.reason = 'Catalonia/Basque independence is politically explosive',
    t.alternative = '',
    t.display_name = 'Regional political conflicts',
    t.content = 'Catalonia/Basque independence is politically explosive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:latin-american-spanish@es-ES'})
SET t.locale = 'es-ES',
    t.text = 'Latin American Spanish',
    t.severity = 'medium',
    t.reason = 'Using ustedes for plural you (instead of vosotros) sounds foreign',
    t.alternative = '',
    t.display_name = 'Latin American Spanish',
    t.content = 'Using ustedes for plural you (instead of vosotros) sounds foreign',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:franco-era-references@es-ES'})
SET t.locale = 'es-ES',
    t.text = 'Franco era references',
    t.severity = 'high',
    t.reason = 'Historical sensitivities around dictatorship period',
    t.alternative = '',
    t.display_name = 'Franco era references',
    t.content = 'Historical sensitivities around dictatorship period',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-ES'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:mexican-stereotypes@es-MX'})
SET t.locale = 'es-MX',
    t.text = 'Mexican stereotypes',
    t.severity = 'high',
    t.reason = 'Sombrero/cactus imagery offensive; modern Mexico is sophisticated',
    t.alternative = '',
    t.display_name = 'Mexican stereotypes',
    t.content = 'Sombrero/cactus imagery offensive; modern Mexico is sophisticated',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-MX'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:u-s-mexico-border-politics@es-MX'})
SET t.locale = 'es-MX',
    t.text = 'U.S.-Mexico border politics',
    t.severity = 'high',
    t.reason = 'Immigration politics extremely sensitive',
    t.alternative = '',
    t.display_name = 'U.S.-Mexico border politics',
    t.content = 'Immigration politics extremely sensitive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-MX'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:spain-mexico-colonial-history@es-MX'})
SET t.locale = 'es-MX',
    t.text = 'Spain-Mexico colonial history',
    t.severity = 'medium',
    t.reason = 'Colonial references can feel patronizing',
    t.alternative = '',
    t.display_name = 'Spain-Mexico colonial history',
    t.content = 'Colonial references can feel patronizing',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-MX'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:malvinas-falklands@es-AR'})
SET t.locale = 'es-AR',
    t.text = 'Malvinas/Falklands',
    t.severity = 'high',
    t.reason = 'Deeply patriotic issue; never use \'Falklands\' in Argentina',
    t.alternative = '',
    t.display_name = 'Malvinas/Falklands',
    t.content = 'Deeply patriotic issue; never use \'Falklands\' in Argentina',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-AR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:economic-crisis-references@es-AR'})
SET t.locale = 'es-AR',
    t.text = 'Economic crisis references',
    t.severity = 'medium',
    t.reason = 'Repeated economic crises are painful; don\'t highlight financial instability',
    t.alternative = '',
    t.display_name = 'Economic crisis references',
    t.content = 'Repeated economic crises are painful; don\'t highlight financial instability',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-AR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:comparison-to-other-latam@es-AR'})
SET t.locale = 'es-AR',
    t.text = 'Comparison to other LATAM',
    t.severity = 'medium',
    t.reason = 'Argentines see themselves as distinct from \'rest of Latin America\'',
    t.alternative = '',
    t.display_name = 'Comparison to other LATAM',
    t.content = 'Argentines see themselves as distinct from \'rest of Latin America\'',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-AR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:drug-trafficking-cartel-references@es-CO'})
SET t.locale = 'es-CO',
    t.text = 'Drug trafficking/cartel references',
    t.severity = 'high',
    t.reason = 'Narcos-style stereotypes deeply offensive; Colombia has moved on',
    t.alternative = '',
    t.display_name = 'Drug trafficking/cartel references',
    t.content = 'Narcos-style stereotypes deeply offensive; Colombia has moved on',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:violence-conflict-history@es-CO'})
SET t.locale = 'es-CO',
    t.text = 'Violence/conflict history',
    t.severity = 'high',
    t.reason = 'Armed conflict is painful history, not marketing material',
    t.alternative = '',
    t.display_name = 'Violence/conflict history',
    t.content = 'Armed conflict is painful history, not marketing material',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:oversimplifying-colombia@es-CO'})
SET t.locale = 'es-CO',
    t.text = 'Oversimplifying Colombia',
    t.severity = 'medium',
    t.reason = 'Colombia is highly diverse; avoid coffee/emerald only associations',
    t.alternative = '',
    t.display_name = 'Oversimplifying Colombia',
    t.content = 'Colombia is highly diverse; avoid coffee/emerald only associations',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CO'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:pinochet-era-politics@es-CL'})
SET t.locale = 'es-CL',
    t.text = 'Pinochet era politics',
    t.severity = 'high',
    t.reason = 'Still divisive; avoid political references entirely',
    t.alternative = '',
    t.display_name = 'Pinochet era politics',
    t.content = 'Still divisive; avoid political references entirely',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:peru-bolivia-border-conflicts@es-CL'})
SET t.locale = 'es-CL',
    t.text = 'Peru/Bolivia border conflicts',
    t.severity = 'medium',
    t.reason = 'Historical tensions with neighbors are sensitive',
    t.alternative = '',
    t.display_name = 'Peru/Bolivia border conflicts',
    t.content = 'Historical tensions with neighbors are sensitive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:treating-chile-as-generic-latam@es-CL'})
SET t.locale = 'es-CL',
    t.text = 'Treating Chile as generic LATAM',
    t.severity = 'medium',
    t.reason = 'Chileans proud of distinct identity and progress',
    t.alternative = '',
    t.display_name = 'Treating Chile as generic LATAM',
    t.content = 'Chileans proud of distinct identity and progress',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@es-CL'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:portugal-comparison@pt-BR'})
SET t.locale = 'pt-BR',
    t.text = 'Portugal comparison',
    t.severity = 'high',
    t.reason = 'Brazilian Portuguese is distinct language variant, not dialect',
    t.alternative = '',
    t.display_name = 'Portugal comparison',
    t.content = 'Brazilian Portuguese is distinct language variant, not dialect',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-BR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:crime-violence-stereotypes@pt-BR'})
SET t.locale = 'pt-BR',
    t.text = 'Crime/violence stereotypes',
    t.severity = 'high',
    t.reason = 'Favela/crime tourism imagery deeply offensive',
    t.alternative = '',
    t.display_name = 'Crime/violence stereotypes',
    t.content = 'Favela/crime tourism imagery deeply offensive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-BR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:political-polarization@pt-BR'})
SET t.locale = 'pt-BR',
    t.text = 'Political polarization',
    t.severity = 'high',
    t.reason = 'Deep political division; stay completely neutral',
    t.alternative = '',
    t.display_name = 'Political polarization',
    t.content = 'Deep political division; stay completely neutral',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-BR'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:brazilian-portuguese-usage@pt-PT'})
SET t.locale = 'pt-PT',
    t.text = 'Brazilian Portuguese usage',
    t.severity = 'high',
    t.reason = 'Using Brazilian terms (você instead of tu) signals laziness or ignorance',
    t.alternative = '',
    t.display_name = 'Brazilian Portuguese usage',
    t.content = 'Using Brazilian terms (você instead of tu) signals laziness or ignorance',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-PT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:colonial-past-glorification@pt-PT'})
SET t.locale = 'pt-PT',
    t.text = 'Colonial past glorification',
    t.severity = 'medium',
    t.reason = 'Complex history; avoid both glorification and excessive criticism',
    t.alternative = '',
    t.display_name = 'Colonial past glorification',
    t.content = 'Complex history; avoid both glorification and excessive criticism',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-PT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:spain-confusion@pt-PT'})
SET t.locale = 'pt-PT',
    t.text = 'Spain confusion',
    t.severity = 'medium',
    t.reason = 'Portugal is NOT Spain; distinct language and culture',
    t.alternative = '',
    t.display_name = 'Spain confusion',
    t.content = 'Portugal is NOT Spain; distinct language and culture',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@pt-PT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:north-south-divide-stereotypes@it-IT'})
SET t.locale = 'it-IT',
    t.text = 'North-South divide stereotypes',
    t.severity = 'high',
    t.reason = 'Terroni/Polentoni slurs; regional tensions are real and sensitive',
    t.alternative = '',
    t.display_name = 'North-South divide stereotypes',
    t.content = 'Terroni/Polentoni slurs; regional tensions are real and sensitive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-IT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:mafia-references@it-IT'})
SET t.locale = 'it-IT',
    t.text = 'Mafia references',
    t.severity = 'high',
    t.reason = 'Deeply offensive stereotype, especially to Southerners',
    t.alternative = '',
    t.display_name = 'Mafia references',
    t.content = 'Deeply offensive stereotype, especially to Southerners',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-IT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:criticizing-italian-systems@it-IT'})
SET t.locale = 'it-IT',
    t.text = 'Criticizing Italian systems',
    t.severity = 'medium',
    t.reason = 'Italians can criticize Italy; foreigners cannot',
    t.alternative = '',
    t.display_name = 'Criticizing Italian systems',
    t.content = 'Italians can criticize Italy; foreigners cannot',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-IT'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:treating-ticino-as-italy@it-CH'})
SET t.locale = 'it-CH',
    t.text = 'Treating Ticino as Italy',
    t.severity = 'high',
    t.reason = 'Ticinesi are Swiss, not expat Italians',
    t.alternative = '',
    t.display_name = 'Treating Ticino as Italy',
    t.content = 'Ticinesi are Swiss, not expat Italians',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:aggressive-sales@it-CH'})
SET t.locale = 'it-CH',
    t.text = 'Aggressive sales',
    t.severity = 'medium',
    t.reason = 'Swiss reserve means softer approach needed',
    t.alternative = '',
    t.display_name = 'Aggressive sales',
    t.content = 'Swiss reserve means softer approach needed',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:financial-prying@it-CH'})
SET t.locale = 'it-CH',
    t.text = 'Financial prying',
    t.severity = 'high',
    t.reason = 'Swiss banking discretion culture applies',
    t.alternative = '',
    t.display_name = 'Financial prying',
    t.content = 'Swiss banking discretion culture applies',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@it-CH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

