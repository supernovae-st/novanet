// Migration 083: Populate Knowledge Atoms with Real Locale-Specific Data
// Source: Perplexity research 2026-03-10
// Creates authentic Taboo, Pattern, CultureRef, AudienceTrait atoms
//
// Run after: 082-knowledge-atom-containers.cypher

// =============================================================================
// PART 1: FRENCH (fr-FR) TABOOS - Topics to Avoid
// =============================================================================
MATCH (l:Locale {key: 'fr-FR'})
MERGE (ts:TabooSet {key: 'taboo-set:avoid@fr-FR'})
ON CREATE SET
  ts.display_name = 'French (France) Topics to Avoid',
  ts.description = 'Cultural taboos for French content',
  ts.severity = 'avoid',
  ts.llm_context = 'USE: when filtering content for French audience. TRIGGERS: taboo, avoid, forbidden. NOT: for legal issues. RELATES: Locale, Taboo.',
  ts.created_at = datetime(),
  ts.updated_at = datetime()
MERGE (l)-[:HAS_TABOOS]->(ts)
MERGE (ts)-[:FOR_LOCALE]->(l)

WITH ts, l
// Create individual taboo atoms
UNWIND [
  {key: 'fr-FR/AVOID/001', text: 'Discussing personal wealth or salary openly', context: 'French culture considers discussing money vulgar (vulgarit\u00e9) and lacking class. Signal status subtly through refined taste, not boasting.', category: 'money'},
  {key: 'fr-FR/AVOID/002', text: 'Flaunting luxury purchases or prices', context: 'Post-Revolution ideals make visible inequality suspect. Focus on quality craftsmanship over wealth displays.', category: 'money'},
  {key: 'fr-FR/AVOID/003', text: 'Using identifiable people in photos without consent', context: 'Strict French privacy laws require written permission for commercial use of identifiable individuals.', category: 'legal'},
  {key: 'fr-FR/AVOID/004', text: 'Overly familiar gestures in professional settings', context: 'Avoid forcing la bise (cheek kisses) in business - opt for handshakes. Maintain professional distance initially.', category: 'etiquette'},
  {key: 'fr-FR/AVOID/005', text: 'Aggressive or rushed communication style', context: 'French value savoir-vivre (art of living well). Do not snap fingers, wave aggressively, or expect rushed service.', category: 'communication'}
] AS taboo
MERGE (t:Taboo {key: taboo.key})
ON CREATE SET
  t.text = taboo.text,
  t.context = taboo.context,
  t.category = taboo.category,
  t.display_name = taboo.text,
  t.description = taboo.context,
  t.created_at = datetime(),
  t.updated_at = datetime()
MERGE (ts)-[:CONTAINS_TABOO]->(t)
MERGE (t)-[:FOR_LOCALE]->(l);

// =============================================================================
// PART 2: JAPANESE (ja-JP) TABOOS - Topics to Avoid
// =============================================================================
MATCH (l:Locale {key: 'ja-JP'})
MERGE (ts:TabooSet {key: 'taboo-set:avoid@ja-JP'})
ON CREATE SET
  ts.display_name = 'Japanese Topics to Avoid',
  ts.description = 'Cultural taboos for Japanese content',
  ts.severity = 'avoid',
  ts.llm_context = 'USE: when filtering content for Japanese audience. TRIGGERS: taboo, avoid. NOT: for legal. RELATES: Locale, Taboo.',
  ts.created_at = datetime(),
  ts.updated_at = datetime()
MERGE (l)-[:HAS_TABOOS]->(ts)
MERGE (ts)-[:FOR_LOCALE]->(l)

WITH ts, l
UNWIND [
  {key: 'ja-JP/AVOID/001', text: 'Using red ink for names', context: 'Red ink symbolizes death in Japanese culture. Never write names in red.', category: 'symbolism'},
  {key: 'ja-JP/AVOID/002', text: 'Sets of four items in promotions', context: 'The number 4 (shi) sounds like death. Avoid groupings of 4 in gifts or promotions.', category: 'symbolism'},
  {key: 'ja-JP/AVOID/003', text: 'Direct confrontation or finger-pointing', context: 'Disrupts wa (harmony). Use subtle, indirect cues instead of direct criticism.', category: 'communication'},
  {key: 'ja-JP/AVOID/004', text: 'Overly aggressive sales pitches', context: 'Japanese value humility and trust-building. Direct sales pitches feel rude.', category: 'marketing'},
  {key: 'ja-JP/AVOID/005', text: 'Casual first-name use in business', context: 'Always use formal titles like -san. Casual address shows disrespect for hierarchy.', category: 'etiquette'},
  {key: 'ja-JP/AVOID/006', text: 'Tipping references in service content', context: 'Tipping is seen as insulting in Japan. Never suggest or depict tipping.', category: 'etiquette'}
] AS taboo
MERGE (t:Taboo {key: taboo.key})
ON CREATE SET
  t.text = taboo.text,
  t.context = taboo.context,
  t.category = taboo.category,
  t.display_name = taboo.text,
  t.description = taboo.context,
  t.created_at = datetime(),
  t.updated_at = datetime()
MERGE (ts)-[:CONTAINS_TABOO]->(t)
MERGE (t)-[:FOR_LOCALE]->(l);

// =============================================================================
// PART 3: ARABIC SAUDI (ar-SA) TABOOS - Topics to Avoid
// =============================================================================
MATCH (l:Locale {key: 'ar-SA'})
MERGE (ts:TabooSet {key: 'taboo-set:avoid@ar-SA'})
ON CREATE SET
  ts.display_name = 'Saudi Arabia Topics to Avoid',
  ts.description = 'Cultural and religious taboos for Saudi content',
  ts.severity = 'avoid',
  ts.llm_context = 'USE: when filtering content for Saudi audience. TRIGGERS: taboo, haram, avoid. NOT: for general Arabic. RELATES: Locale, Taboo.',
  ts.created_at = datetime(),
  ts.updated_at = datetime()
MERGE (l)-[:HAS_TABOOS]->(ts)
MERGE (ts)-[:FOR_LOCALE]->(l)

WITH ts, l
UNWIND [
  {key: 'ar-SA/AVOID/001', text: 'Alcohol references or imagery', context: 'Alcohol is strictly forbidden (haram). Never depict, promote, or imply consumption.', category: 'religious'},
  {key: 'ar-SA/AVOID/002', text: 'Pork or non-halal food products', context: 'Pork consumption is illegal and forbidden. Ensure all food content is halal-certified.', category: 'religious'},
  {key: 'ar-SA/AVOID/003', text: 'Suggestive or immodest imagery', context: 'Deep conservatism requires modest dress. Avoid revealing clothing even in all-female contexts.', category: 'modesty'},
  {key: 'ar-SA/AVOID/004', text: 'Public displays of affection', context: 'No kissing, hugging between opposite sexes in visuals. Keep interactions discreet.', category: 'modesty'},
  {key: 'ar-SA/AVOID/005', text: 'Political or royal family topics', context: 'Avoid politics, criticism of royal family, or controversial religious topics.', category: 'political'},
  {key: 'ar-SA/AVOID/006', text: 'Eating/drinking during Ramadan daylight in content', context: 'During Ramadan, do not show eating or drinking during daylight hours.', category: 'religious'}
] AS taboo
MERGE (t:Taboo {key: taboo.key})
ON CREATE SET
  t.text = taboo.text,
  t.context = taboo.context,
  t.category = taboo.category,
  t.display_name = taboo.text,
  t.description = taboo.context,
  t.created_at = datetime(),
  t.updated_at = datetime()
MERGE (ts)-[:CONTAINS_TABOO]->(t)
MERGE (t)-[:FOR_LOCALE]->(l);

// =============================================================================
// PART 4: FRENCH (fr-FR) CTA PATTERNS
// =============================================================================
MATCH (l:Locale {key: 'fr-FR'})
MERGE (ps:PatternSet {key: 'pattern-set:cta@fr-FR'})
ON CREATE SET
  ps.display_name = 'French CTA Patterns',
  ps.description = 'High-converting call-to-action patterns for French e-commerce',
  ps.usage = 'cta',
  ps.llm_context = 'USE: when generating CTAs for French audience. TRIGGERS: cta, button, action. NOT: for headlines. RELATES: Locale, Pattern.',
  ps.created_at = datetime(),
  ps.updated_at = datetime()
MERGE (l)-[:HAS_PATTERNS]->(ps)
MERGE (ps)-[:FOR_LOCALE]->(l)

WITH ps, l
UNWIND [
  {key: 'fr-FR/CTA/001', text: 'Acheter maintenant', context: 'Primary purchase CTA - direct and clear. Most effective for checkout buttons.', usage: 'purchase'},
  {key: 'fr-FR/CTA/002', text: 'Commander', context: 'Professional, trusted order button. Preferred for B2B contexts.', usage: 'purchase'},
  {key: 'fr-FR/CTA/003', text: 'Ajouter au panier', context: 'Standard e-commerce add to cart. Universal and expected.', usage: 'cart'},
  {key: 'fr-FR/CTA/004', text: 'Profiter de l\'offre', context: 'Urgency-driven CTA. Effective for limited-time promotions.', usage: 'urgency'},
  {key: 'fr-FR/CTA/005', text: 'D\u00e9couvrir', context: 'Soft engagement CTA. Perfect for content discovery or collections.', usage: 'engagement'},
  {key: 'fr-FR/CTA/006', text: 'En savoir plus', context: 'Learn more CTA. Used for additional information or details.', usage: 'engagement'},
  {key: 'fr-FR/CTA/007', text: 'Essayer gratuitement', context: 'Free trial CTA. Trust-building for SaaS or subscription services.', usage: 'trial'},
  {key: 'fr-FR/CTA/008', text: 'Saisir l\'occasion', context: 'Seize the opportunity. Strong urgency CTA for flash sales.', usage: 'urgency'}
] AS pattern
MERGE (p:Pattern {key: pattern.key})
ON CREATE SET
  p.text = pattern.text,
  p.context = pattern.context,
  p.usage = pattern.usage,
  p.display_name = pattern.text,
  p.description = pattern.context,
  p.created_at = datetime(),
  p.updated_at = datetime()
MERGE (ps)-[:CONTAINS_PATTERN]->(p)
MERGE (p)-[:FOR_LOCALE]->(l);

// =============================================================================
// PART 5: GERMAN (de-DE) AUDIENCE TRAITS
// =============================================================================
MATCH (l:Locale {key: 'de-DE'})
MERGE (as:AudienceSet {key: 'audience-set:b2b@de-DE'})
ON CREATE SET
  as.display_name = 'German B2B Audience',
  as.description = 'B2B audience behavior and preferences for Germany',
  as.segment = 'b2b',
  as.llm_context = 'USE: when generating B2B content for German audience. TRIGGERS: business, B2B, professional. NOT: for B2C casual. RELATES: Locale, AudienceTrait.',
  as.created_at = datetime(),
  as.updated_at = datetime()
MERGE (l)-[:HAS_AUDIENCE]->(as)
MERGE (as)-[:FOR_LOCALE]->(l)

WITH as, l
UNWIND [
  {key: 'de-DE/B2B/001', text: 'Values directness and clarity', context: 'German B2B buyers expect concise, unambiguous communication. Propose specific dates/times rather than vague scheduling.', trait_type: 'communication'},
  {key: 'de-DE/B2B/002', text: 'Requires data-backed information', context: 'Provide statistics, ROI projections, and detailed timelines. Vague promises or exaggerations damage credibility.', trait_type: 'decision_making'},
  {key: 'de-DE/B2B/003', text: 'Prefers formal language (Sie over du)', context: 'Use formal address and professional titles. Informal tone appears unprofessional in initial B2B interactions.', trait_type: 'communication'},
  {key: 'de-DE/B2B/004', text: 'Risk-averse with longer sales cycles', context: 'Expect proof-of-concept and pilot projects. Trust-building takes time before commitments.', trait_type: 'decision_making'},
  {key: 'de-DE/B2B/005', text: 'Separates business from personal matters', context: 'Avoid humor, colloquialisms, or personal topics in business contexts. Focus on substance.', trait_type: 'etiquette'}
] AS trait
MERGE (at:AudienceTrait {key: trait.key})
ON CREATE SET
  at.text = trait.text,
  at.context = trait.context,
  at.trait_type = trait.trait_type,
  at.display_name = trait.text,
  at.description = trait.context,
  at.created_at = datetime(),
  at.updated_at = datetime()
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at)
MERGE (at)-[:FOR_LOCALE]->(l);

// =============================================================================
// PART 6: JAPANESE (ja-JP) CULTURAL REFERENCES
// =============================================================================
MATCH (l:Locale {key: 'ja-JP'})
MERGE (cs:CultureSet {key: 'culture-set:values@ja-JP'})
ON CREATE SET
  cs.display_name = 'Japanese Cultural Values',
  cs.description = 'Core cultural values and concepts for Japanese content',
  cs.culture_type = 'values',
  cs.llm_context = 'USE: when incorporating Japanese cultural context. TRIGGERS: culture, values, tradition. NOT: for modern slang. RELATES: Locale, CultureRef.',
  cs.created_at = datetime(),
  cs.updated_at = datetime()
MERGE (l)-[:HAS_CULTURE_SET]->(cs)
MERGE (cs)-[:FOR_LOCALE]->(l)

WITH cs, l
UNWIND [
  {key: 'ja-JP/VALUES/001', text: '\u4e00\u671f\u4e00\u4f1a (ichigo ichie)', context: 'Treasure every encounter as a once-in-a-lifetime opportunity. Perfect for limited-time offers or exclusive experiences.', ref_type: 'yojijukugo'},
  {key: 'ja-JP/VALUES/002', text: '\u4e00\u77f3\u4e8c\u9ce5 (isseki nichou)', context: 'Killing two birds with one stone. Highlights dual benefits in promotions showing multi-value deals.', ref_type: 'yojijukugo'},
  {key: 'ja-JP/VALUES/003', text: '\u8210\u306f\u8210\u5c4b (mochi wa mochiya)', context: 'Leave mochi to the mochi maker. Trust the experts. Effective in B2B or expert-service marketing.', ref_type: 'kotowaza'},
  {key: 'ja-JP/VALUES/004', text: '\u4f8e\u3073\u5bc2\u3073 (wabi-sabi)', context: 'Beauty in simplicity and imperfection. Use serene imagery and quiet design over feature lists.', ref_type: 'aesthetic'},
  {key: 'ja-JP/VALUES/005', text: '\u7a7a\u6c17\u3092\u8aad\u3080 (kuuki wo yomu)', context: 'Reading the air - understanding implicit social norms. Align content with subtle cultural expectations.', ref_type: 'social_concept'}
] AS ref
MERGE (cr:CultureRef {key: ref.key})
ON CREATE SET
  cr.text = ref.text,
  cr.context = ref.context,
  cr.ref_type = ref.ref_type,
  cr.display_name = ref.text,
  cr.description = ref.context,
  cr.created_at = datetime(),
  cr.updated_at = datetime()
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr)
MERGE (cr)-[:FOR_LOCALE]->(l);

// =============================================================================
// PART 7: SPANISH (es-ES) CULTURAL REFERENCES
// =============================================================================
MATCH (l:Locale {key: 'es-ES'})
MERGE (cs:CultureSet {key: 'culture-set:references@es-ES'})
ON CREATE SET
  cs.display_name = 'Spanish Cultural References',
  cs.description = 'Pop culture references and celebrities for Spanish content',
  cs.culture_type = 'references',
  cs.llm_context = 'USE: when adding cultural relevance for Spanish audience. TRIGGERS: culture, celebrity, reference. NOT: for formal business. RELATES: Locale, CultureRef.',
  cs.created_at = datetime(),
  cs.updated_at = datetime()
MERGE (l)-[:HAS_CULTURE_SET]->(cs)
MERGE (cs)-[:FOR_LOCALE]->(l)

WITH cs, l
UNWIND [
  {key: 'es-ES/REFS/001', text: 'El paseo (evening stroll)', context: 'Traditional evening walk that fosters community. Great for community-driven or local business campaigns.', ref_type: 'tradition'},
  {key: 'es-ES/REFS/002', text: 'Antonio Banderas', context: 'Iconic Spanish actor. Used effectively in storytelling campaigns with cultural Easter eggs.', ref_type: 'celebrity'},
  {key: 'es-ES/REFS/003', text: 'Los Goya (film awards)', context: 'Spanish equivalent of Oscars. Reference for quality, prestige, and cultural achievement.', ref_type: 'cultural_event'},
  {key: 'es-ES/REFS/004', text: 'Mediterranean cuisine and lifestyle', context: 'Spanish value authentic Mediterranean recipes and lifestyle. Drive loyalty through regional food references.', ref_type: 'lifestyle'}
] AS ref
MERGE (cr:CultureRef {key: ref.key})
ON CREATE SET
  cr.text = ref.text,
  cr.context = ref.context,
  cr.ref_type = ref.ref_type,
  cr.display_name = ref.text,
  cr.description = ref.context,
  cr.created_at = datetime(),
  cr.updated_at = datetime()
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr)
MERGE (cr)-[:FOR_LOCALE]->(l);

// =============================================================================
// VERIFICATION: Count all atoms created
// =============================================================================
MATCH (t:Taboo) WITH count(t) AS taboo_count
MATCH (p:Pattern) WITH taboo_count, count(p) AS pattern_count
MATCH (cr:CultureRef) WITH taboo_count, pattern_count, count(cr) AS culture_count
MATCH (at:AudienceTrait) WITH taboo_count, pattern_count, culture_count, count(at) AS audience_count
RETURN
  taboo_count AS Taboos,
  pattern_count AS Patterns,
  culture_count AS CultureRefs,
  audience_count AS AudienceTraits,
  taboo_count + pattern_count + culture_count + audience_count AS total_atoms;
