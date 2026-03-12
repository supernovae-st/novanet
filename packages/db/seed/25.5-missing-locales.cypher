// ============================================================
// 25.5 - Missing Locales: fil-PH, lo-LA, or-IN (Task C)
// Generated: 2026-03-10T19:02:22.409Z
// ============================================================

// === fil-PH - Filipino (Philippines) ===

MERGE (cr:CultureRef {key: 'cultureref:pakikisama-harmony@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Pakikisama (Harmony)',
    cr.importance = 'critical',
    cr.expression = 'Maintaining smooth interpersonal relationships and group harmony is paramount',
    cr.marketing_angle = 'Position products as enhancing group belonging and social harmony',
    cr.display_name = 'Pakikisama (Harmony)',
    cr.content = 'Maintaining smooth interpersonal relationships and group harmony is paramount',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hiya-sense-of-shame@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Hiya (Sense of Shame)',
    cr.importance = 'critical',
    cr.expression = 'Deep concern for social acceptance and avoiding embarrassment or losing face',
    cr.marketing_angle = 'Avoid direct confrontation; use indirect, respectful messaging',
    cr.display_name = 'Hiya (Sense of Shame)',
    cr.content = 'Deep concern for social acceptance and avoiding embarrassment or losing face',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:utang-na-loob-debt-of-gratitude@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Utang na Loob (Debt of Gratitude)',
    cr.importance = 'high',
    cr.expression = 'Strong sense of reciprocity and obligation to return favors',
    cr.marketing_angle = 'Frame loyalty programs as mutual gratitude; emphasize lasting relationships',
    cr.display_name = 'Utang na Loob (Debt of Gratitude)',
    cr.content = 'Strong sense of reciprocity and obligation to return favors',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:bayanihan-community-spirit@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Bayanihan (Community Spirit)',
    cr.importance = 'high',
    cr.expression = 'Community helping each other, especially in times of need',
    cr.marketing_angle = 'Highlight community benefits and collective success stories',
    cr.display_name = 'Bayanihan (Community Spirit)',
    cr.content = 'Community helping each other, especially in times of need',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:family-first-values@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Family-First Values',
    cr.importance = 'critical',
    cr.expression = 'Family is the center of Filipino life; decisions consider family impact',
    cr.marketing_angle = 'Show multi-generational benefits; family testimonials resonate',
    cr.display_name = 'Family-First Values',
    cr.content = 'Family is the center of Filipino life; decisions consider family impact',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:respect-for-elders-po-opo@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Respect for Elders (Po/Opo)',
    cr.importance = 'high',
    cr.expression = 'Use of honorifics and deference to older or senior individuals',
    cr.marketing_angle = 'Use respectful language; show deference in formal communications',
    cr.display_name = 'Respect for Elders (Po/Opo)',
    cr.content = 'Use of honorifics and deference to older or senior individuals',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:hospitality-mabuhay-spirit@fil-PH'})
SET cr.locale = 'fil-PH',
    cr.text = 'Hospitality (Mabuhay Spirit)',
    cr.importance = 'high',
    cr.expression = 'Welcoming guests warmly is a core cultural value',
    cr.marketing_angle = 'Position brand as welcoming and inclusive',
    cr.display_name = 'Hospitality (Mabuhay Spirit)',
    cr.content = 'Welcoming guests warmly is a core cultural value',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@fil-PH'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (t:Taboo {key: 'taboo:public-confrontation@fil-PH'})
SET t.locale = 'fil-PH',
    t.text = 'Public confrontation',
    t.severity = 'high',
    t.reason = 'Causes loss of face (hiya) and damages relationships permanently',
    t.alternative = 'Use indirect communication, private discussions, or intermediaries',
    t.display_name = 'Public confrontation',
    t.content = 'Causes loss of face (hiya) and damages relationships permanently',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fil-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:refusing-hospitality@fil-PH'})
SET t.locale = 'fil-PH',
    t.text = 'Refusing hospitality',
    t.severity = 'medium',
    t.reason = 'Declining food or gifts is seen as rejecting the host\'s generosity',
    t.alternative = 'Accept graciously, even a small portion',
    t.display_name = 'Refusing hospitality',
    t.content = 'Declining food or gifts is seen as rejecting the host\'s generosity',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fil-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:loud-or-aggressive-behavior@fil-PH'})
SET t.locale = 'fil-PH',
    t.text = 'Loud or aggressive behavior',
    t.severity = 'high',
    t.reason = 'Filipinos avoid raising voices and showing anger publicly',
    t.alternative = 'Maintain calm, polite demeanor in all interactions',
    t.display_name = 'Loud or aggressive behavior',
    t.content = 'Filipinos avoid raising voices and showing anger publicly',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fil-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:rushing-decisions@fil-PH'})
SET t.locale = 'fil-PH',
    t.text = 'Rushing decisions',
    t.severity = 'medium',
    t.reason = 'Relationships take time to build; hasty decisions are seen as disrespectful',
    t.alternative = 'Allow time for relationship building and consensus',
    t.display_name = 'Rushing decisions',
    t.content = 'Relationships take time to build; hasty decisions are seen as disrespectful',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fil-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:disrespecting-elders@fil-PH'})
SET t.locale = 'fil-PH',
    t.text = 'Disrespecting elders',
    t.severity = 'critical',
    t.reason = 'Age hierarchy is fundamental; ignoring it is deeply offensive',
    t.alternative = 'Always greet elders first; use honorific titles',
    t.display_name = 'Disrespecting elders',
    t.content = 'Age hierarchy is fundamental; ignoring it is deeply offensive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@fil-PH'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (at:AudienceTrait {key: 'audience:mobile-first-consumers@fil-PH'})
SET at.locale = 'fil-PH',
    at.text = 'Mobile-first consumers',
    at.content = 'Philippines has one of highest social media usage rates globally',
    at.display_name = 'Mobile-first consumers',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@fil-PH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

MERGE (at:AudienceTrait {key: 'audience:ofw-influence@fil-PH'})
SET at.locale = 'fil-PH',
    at.text = 'OFW influence',
    at.content = 'Overseas Filipino Workers shape consumption patterns and aspirations',
    at.display_name = 'OFW influence',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@fil-PH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

MERGE (at:AudienceTrait {key: 'audience:celebrity-endorsements@fil-PH'})
SET at.locale = 'fil-PH',
    at.text = 'Celebrity endorsements',
    at.content = 'High trust in celebrity and influencer recommendations',
    at.display_name = 'Celebrity endorsements',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@fil-PH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

MERGE (e:Expression {key: 'expr:mabuhay@fil-PH'})
SET e.locale = 'fil-PH',
    e.text = 'Mabuhay!',
    e.register = 'formal',
    e.context = 'Traditional welcome greeting',
    e.display_name = 'Mabuhay!',
    e.content = 'Traditional welcome greeting',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@fil-PH'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

MERGE (e:Expression {key: 'expr:salamat-po@fil-PH'})
SET e.locale = 'fil-PH',
    e.text = 'Salamat po',
    e.register = 'polite',
    e.context = 'Thank you (with respect marker)',
    e.display_name = 'Salamat po',
    e.content = 'Thank you (with respect marker)',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@fil-PH'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

MERGE (e:Expression {key: 'expr:diskarte-lang@fil-PH'})
SET e.locale = 'fil-PH',
    e.text = 'Diskarte lang',
    e.register = 'casual',
    e.context = 'Being resourceful and street-smart',
    e.display_name = 'Diskarte lang',
    e.content = 'Being resourceful and street-smart',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@fil-PH'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

MERGE (p:Pattern {key: 'pattern:cta-sumali-na-sa-produc@fil-PH'})
SET p.locale = 'fil-PH',
    p.text = 'Sumali na sa {product} at maranasan ang pagkakaiba!',
    p.type = 'cta',
    p.translation = 'Join {product} and experience the difference!',
    p.display_name = 'cta pattern',
    p.content = 'Join {product} and experience the difference!',
    p.created_at = datetime(),
    p.updated_at = datetime()
WITH p
MATCH (ps:PatternSet {key: 'pattern-set:cta@fil-PH'})
MERGE (ps)-[:CONTAINS_PATTERN]->(p);

MERGE (p:Pattern {key: 'pattern:social-proof-pinagkakatiwalaan-ng@fil-PH'})
SET p.locale = 'fil-PH',
    p.text = 'Pinagkakatiwalaan ng milyun-milyong pamilyang Pilipino',
    p.type = 'social_proof',
    p.translation = 'Trusted by millions of Filipino families',
    p.display_name = 'social_proof pattern',
    p.content = 'Trusted by millions of Filipino families',
    p.created_at = datetime(),
    p.updated_at = datetime()
WITH p
MATCH (ps:PatternSet {key: 'pattern-set:cta@fil-PH'})
MERGE (ps)-[:CONTAINS_PATTERN]->(p);

// === lo-LA - Lao (Laos) ===

MERGE (cr:CultureRef {key: 'cultureref:theravada-buddhism@lo-LA'})
SET cr.locale = 'lo-LA',
    cr.text = 'Theravada Buddhism',
    cr.importance = 'critical',
    cr.expression = 'Buddhism permeates daily life; monks and temples hold central reverence',
    cr.marketing_angle = 'Respect Buddhist values; avoid frivolous or materialistic messaging',
    cr.display_name = 'Theravada Buddhism',
    cr.content = 'Buddhism permeates daily life; monks and temples hold central reverence',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@lo-LA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:harmony-and-non-confrontation@lo-LA'})
SET cr.locale = 'lo-LA',
    cr.text = 'Harmony and Non-confrontation',
    cr.importance = 'critical',
    cr.expression = 'Preserving face and avoiding conflict is paramount in all interactions',
    cr.marketing_angle = 'Use soft, harmonious imagery and indirect communication',
    cr.display_name = 'Harmony and Non-confrontation',
    cr.content = 'Preserving face and avoiding conflict is paramount in all interactions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@lo-LA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:sabaidee-spirit@lo-LA'})
SET cr.locale = 'lo-LA',
    cr.text = 'Sabaidee Spirit',
    cr.importance = 'high',
    cr.expression = 'The universal greeting reflects cultural warmth and well-wishing',
    cr.marketing_angle = 'Incorporate Sabaidee in campaigns for cultural authenticity',
    cr.display_name = 'Sabaidee Spirit',
    cr.content = 'The universal greeting reflects cultural warmth and well-wishing',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@lo-LA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:respect-for-hierarchy@lo-LA'})
SET cr.locale = 'lo-LA',
    cr.text = 'Respect for Hierarchy',
    cr.importance = 'high',
    cr.expression = 'Age and social status determine appropriate behavior and language',
    cr.marketing_angle = 'Use formal titles (Than, Nai, Nang) appropriately',
    cr.display_name = 'Respect for Hierarchy',
    cr.content = 'Age and social status determine appropriate behavior and language',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@lo-LA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:modesty@lo-LA'})
SET cr.locale = 'lo-LA',
    cr.text = 'Modesty',
    cr.importance = 'high',
    cr.expression = 'Physical and behavioral modesty is expected in public',
    cr.marketing_angle = 'Avoid flashy or ostentatious imagery; promote simplicity',
    cr.display_name = 'Modesty',
    cr.content = 'Physical and behavioral modesty is expected in public',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@lo-LA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:community-support@lo-LA'})
SET cr.locale = 'lo-LA',
    cr.text = 'Community Support',
    cr.importance = 'medium',
    cr.expression = 'Supporting local businesses and community reflects goodwill',
    cr.marketing_angle = 'Highlight local partnerships and community benefits',
    cr.display_name = 'Community Support',
    cr.content = 'Supporting local businesses and community reflects goodwill',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@lo-LA'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (t:Taboo {key: 'taboo:touching-heads@lo-LA'})
SET t.locale = 'lo-LA',
    t.text = 'Touching heads',
    t.severity = 'critical',
    t.reason = 'The head is considered sacred and the highest part of the body',
    t.alternative = 'Never touch anyone\'s head, even children',
    t.display_name = 'Touching heads',
    t.content = 'The head is considered sacred and the highest part of the body',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@lo-LA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:pointing-with-feet@lo-LA'})
SET t.locale = 'lo-LA',
    t.text = 'Pointing with feet',
    t.severity = 'critical',
    t.reason = 'Feet are considered the lowest and dirtiest part of the body',
    t.alternative = 'Never point feet at people, Buddha images, or temples',
    t.display_name = 'Pointing with feet',
    t.content = 'Feet are considered the lowest and dirtiest part of the body',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@lo-LA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:public-displays-of-affection@lo-LA'})
SET t.locale = 'lo-LA',
    t.text = 'Public displays of affection',
    t.severity = 'high',
    t.reason = 'Physical intimacy in public is considered inappropriate',
    t.alternative = 'Maintain modest, respectful distance in public',
    t.display_name = 'Public displays of affection',
    t.content = 'Physical intimacy in public is considered inappropriate',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@lo-LA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:raising-voice-or-arguing@lo-LA'})
SET t.locale = 'lo-LA',
    t.text = 'Raising voice or arguing',
    t.severity = 'high',
    t.reason = 'Causes loss of face and disrupts harmony',
    t.alternative = 'Speak softly and calmly; resolve issues privately',
    t.display_name = 'Raising voice or arguing',
    t.content = 'Causes loss of face and disrupts harmony',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@lo-LA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:disrespecting-monks@lo-LA'})
SET t.locale = 'lo-LA',
    t.text = 'Disrespecting monks',
    t.severity = 'critical',
    t.reason = 'Monks are highly revered; women must not touch them',
    t.alternative = 'Show utmost respect; women maintain physical distance',
    t.display_name = 'Disrespecting monks',
    t.content = 'Monks are highly revered; women must not touch them',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@lo-LA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:rushing-decisions@lo-LA'})
SET t.locale = 'lo-LA',
    t.text = 'Rushing decisions',
    t.severity = 'medium',
    t.reason = 'Patience is valued; haste can seem disrespectful',
    t.alternative = 'Allow time for consensus and relationship building',
    t.display_name = 'Rushing decisions',
    t.content = 'Patience is valued; haste can seem disrespectful',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@lo-LA'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (at:AudienceTrait {key: 'audience:mobile-penetration-growing@lo-LA'})
SET at.locale = 'lo-LA',
    at.text = 'Mobile penetration growing',
    at.content = 'Increasing smartphone adoption, especially among youth',
    at.display_name = 'Mobile penetration growing',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@lo-LA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

MERGE (at:AudienceTrait {key: 'audience:traditional-values@lo-LA'})
SET at.locale = 'lo-LA',
    at.text = 'Traditional values',
    at.content = 'Strong adherence to Buddhist and traditional practices',
    at.display_name = 'Traditional values',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@lo-LA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

MERGE (at:AudienceTrait {key: 'audience:face-conscious@lo-LA'})
SET at.locale = 'lo-LA',
    at.text = 'Face-conscious',
    at.content = 'Social standing and reputation highly important',
    at.display_name = 'Face-conscious',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@lo-LA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

MERGE (e:Expression {key: 'expr:sabaidee@lo-LA'})
SET e.locale = 'lo-LA',
    e.text = 'ສະບາຍດີ (Sabaidee)',
    e.register = 'standard',
    e.context = 'Universal greeting meaning well-being',
    e.display_name = 'ສະບາຍດີ (Sabaidee)',
    e.content = 'Universal greeting meaning well-being',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@lo-LA'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

MERGE (e:Expression {key: 'expr:khop-jai@lo-LA'})
SET e.locale = 'lo-LA',
    e.text = 'ຂອບໃຈ (Khop jai)',
    e.register = 'casual',
    e.context = 'Thank you',
    e.display_name = 'ຂອບໃຈ (Khop jai)',
    e.content = 'Thank you',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@lo-LA'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

MERGE (e:Expression {key: 'expr:khop-jai-jao@lo-LA'})
SET e.locale = 'lo-LA',
    e.text = 'ຂອບໃຈຈ້າວ (Khop jai jao)',
    e.register = 'polite',
    e.context = 'Thank you (polite, male speaker)',
    e.display_name = 'ຂອບໃຈຈ້າວ (Khop jai jao)',
    e.content = 'Thank you (polite, male speaker)',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@lo-LA'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

MERGE (p:Pattern {key: 'pattern:cta-product@lo-LA'})
SET p.locale = 'lo-LA',
    p.text = 'ເຂົ້າຮ່ວມ {product} ມື້ນີ້',
    p.type = 'cta',
    p.translation = 'Join {product} today',
    p.display_name = 'cta pattern',
    p.content = 'Join {product} today',
    p.created_at = datetime(),
    p.updated_at = datetime()
WITH p
MATCH (ps:PatternSet {key: 'pattern-set:cta@lo-LA'})
MERGE (ps)-[:CONTAINS_PATTERN]->(p);

MERGE (p:Pattern {key: 'pattern:greeting@lo-LA'})
SET p.locale = 'lo-LA',
    p.text = 'ສະບາຍດີ! ຍິນດີຕ້ອນຮັບ',
    p.type = 'greeting',
    p.translation = 'Hello! Welcome',
    p.display_name = 'greeting pattern',
    p.content = 'Hello! Welcome',
    p.created_at = datetime(),
    p.updated_at = datetime()
WITH p
MATCH (ps:PatternSet {key: 'pattern-set:cta@lo-LA'})
MERGE (ps)-[:CONTAINS_PATTERN]->(p);

// === or-IN - Odia (Odisha, India) ===

MERGE (cr:CultureRef {key: 'cultureref:lord-jagannath-devotion@or-IN'})
SET cr.locale = 'or-IN',
    cr.text = 'Lord Jagannath Devotion',
    cr.importance = 'critical',
    cr.expression = 'Lord Jagannath holds central significance in Odia life and culture',
    cr.marketing_angle = 'Respect Jagannath traditions; avoid commercial exploitation of sacred imagery',
    cr.display_name = 'Lord Jagannath Devotion',
    cr.content = 'Lord Jagannath holds central significance in Odia life and culture',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@or-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:temple-heritage@or-IN'})
SET cr.locale = 'or-IN',
    cr.text = 'Temple Heritage',
    cr.importance = 'high',
    cr.expression = 'Odisha\'s temples are central to cultural identity and daily practice',
    cr.marketing_angle = 'Reference temple heritage respectfully; emphasize spiritual connections',
    cr.display_name = 'Temple Heritage',
    cr.content = 'Odisha\'s temples are central to cultural identity and daily practice',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@or-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:odissi-classical-arts@or-IN'})
SET cr.locale = 'or-IN',
    cr.text = 'Odissi Classical Arts',
    cr.importance = 'high',
    cr.expression = 'Odissi dance originated in temples; represents regional pride',
    cr.marketing_angle = 'Celebrate classical arts in culturally appropriate ways',
    cr.display_name = 'Odissi Classical Arts',
    cr.content = 'Odissi dance originated in temples; represents regional pride',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@or-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:dayakari-politeness@or-IN'})
SET cr.locale = 'or-IN',
    cr.text = 'ଦୟାକରି (Dayakari - Politeness)',
    cr.importance = 'high',
    cr.expression = 'Respect and politeness are fundamental to Odia interactions',
    cr.marketing_angle = 'Use respectful, courteous language; show patience',
    cr.display_name = 'ଦୟାକରି (Dayakari - Politeness)',
    cr.content = 'Respect and politeness are fundamental to Odia interactions',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@or-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:folk-culture-preservation@or-IN'})
SET cr.locale = 'or-IN',
    cr.text = 'Folk Culture Preservation',
    cr.importance = 'medium',
    cr.expression = 'Active efforts to preserve indigenous heritage against Western influence',
    cr.marketing_angle = 'Celebrate traditional practices; avoid undermining local culture',
    cr.display_name = 'Folk Culture Preservation',
    cr.content = 'Active efforts to preserve indigenous heritage against Western influence',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@or-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (cr:CultureRef {key: 'cultureref:festival-traditions@or-IN'})
SET cr.locale = 'or-IN',
    cr.text = 'Festival Traditions',
    cr.importance = 'high',
    cr.expression = 'Major festivals like Rath Yatra, Nuakhai, Pousa Purnima are central',
    cr.marketing_angle = 'Align campaigns with festival calendars; respect traditions',
    cr.display_name = 'Festival Traditions',
    cr.content = 'Major festivals like Rath Yatra, Nuakhai, Pousa Purnima are central',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@or-IN'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

MERGE (t:Taboo {key: 'taboo:disrespecting-lord-jagannath@or-IN'})
SET t.locale = 'or-IN',
    t.text = 'Disrespecting Lord Jagannath',
    t.severity = 'critical',
    t.reason = 'Jagannath is the heart of Odia identity; any disrespect is deeply offensive',
    t.alternative = 'Show utmost reverence; avoid commercial use of sacred imagery',
    t.display_name = 'Disrespecting Lord Jagannath',
    t.content = 'Jagannath is the heart of Odia identity; any disrespect is deeply offensive',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@or-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:temple-photography-without-permission@or-IN'})
SET t.locale = 'or-IN',
    t.text = 'Temple photography without permission',
    t.severity = 'high',
    t.reason = 'Sacred spaces require permission and respect',
    t.alternative = 'Seek appropriate permissions; be mindful of sacred areas',
    t.display_name = 'Temple photography without permission',
    t.content = 'Sacred spaces require permission and respect',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@or-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:western-cultural-imposition@or-IN'})
SET t.locale = 'or-IN',
    t.text = 'Western cultural imposition',
    t.severity = 'medium',
    t.reason = 'Sensitivity about preserving indigenous culture from external influences',
    t.alternative = 'Adapt to local culture; don\'t impose foreign values',
    t.display_name = 'Western cultural imposition',
    t.content = 'Sensitivity about preserving indigenous culture from external influences',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@or-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (t:Taboo {key: 'taboo:impolite-language@or-IN'})
SET t.locale = 'or-IN',
    t.text = 'Impolite language',
    t.severity = 'medium',
    t.reason = 'Politeness markers are expected in commercial and social interactions',
    t.alternative = 'Use respectful phrases like dayakari (please)',
    t.display_name = 'Impolite language',
    t.content = 'Politeness markers are expected in commercial and social interactions',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@or-IN'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

MERGE (at:AudienceTrait {key: 'audience:regional-pride@or-IN'})
SET at.locale = 'or-IN',
    at.text = 'Regional pride',
    at.content = 'Strong identification with Odia heritage and Odisha state',
    at.display_name = 'Regional pride',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@or-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

MERGE (at:AudienceTrait {key: 'audience:festival-oriented@or-IN'})
SET at.locale = 'or-IN',
    at.text = 'Festival-oriented',
    at.content = 'Consumer behavior strongly tied to festival calendar',
    at.display_name = 'Festival-oriented',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@or-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

MERGE (at:AudienceTrait {key: 'audience:spiritually-connected@or-IN'})
SET at.locale = 'or-IN',
    at.text = 'Spiritually connected',
    at.content = 'Daily life intertwined with religious practices',
    at.display_name = 'Spiritually connected',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@or-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

MERGE (e:Expression {key: 'expr:namaskara@or-IN'})
SET e.locale = 'or-IN',
    e.text = 'ନମସ୍କାର (Namaskara)',
    e.register = 'formal',
    e.context = 'Formal greeting with respect',
    e.display_name = 'ନମସ୍କାର (Namaskara)',
    e.content = 'Formal greeting with respect',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@or-IN'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

MERGE (e:Expression {key: 'expr:dhanyabad@or-IN'})
SET e.locale = 'or-IN',
    e.text = 'ଧନ୍ୟବାଦ (Dhanyabad)',
    e.register = 'standard',
    e.context = 'Thank you',
    e.display_name = 'ଧନ୍ୟବାଦ (Dhanyabad)',
    e.content = 'Thank you',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@or-IN'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

MERGE (e:Expression {key: 'expr:jai-jagannath@or-IN'})
SET e.locale = 'or-IN',
    e.text = 'ଜୟ ଜଗନ୍ନାଥ (Jai Jagannath)',
    e.register = 'devotional',
    e.context = 'Victory to Lord Jagannath - common blessing',
    e.display_name = 'ଜୟ ଜଗନ୍ନାଥ (Jai Jagannath)',
    e.content = 'Victory to Lord Jagannath - common blessing',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@or-IN'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

MERGE (p:Pattern {key: 'pattern:cta-product@or-IN'})
SET p.locale = 'or-IN',
    p.text = 'ଆଜି {product} ସହ ଯୋଗ ଦିଅନ୍ତୁ',
    p.type = 'cta',
    p.translation = 'Join {product} today',
    p.display_name = 'cta pattern',
    p.content = 'Join {product} today',
    p.created_at = datetime(),
    p.updated_at = datetime()
WITH p
MATCH (ps:PatternSet {key: 'pattern-set:cta@or-IN'})
MERGE (ps)-[:CONTAINS_PATTERN]->(p);

MERGE (p:Pattern {key: 'pattern:greeting@or-IN'})
SET p.locale = 'or-IN',
    p.text = 'ନମସ୍କାର! ସ୍ୱାଗତ',
    p.type = 'greeting',
    p.translation = 'Greetings! Welcome',
    p.display_name = 'greeting pattern',
    p.content = 'Greetings! Welcome',
    p.created_at = datetime(),
    p.updated_at = datetime()
WITH p
MATCH (ps:PatternSet {key: 'pattern-set:cta@or-IN'})
MERGE (ps)-[:CONTAINS_PATTERN]->(p);

// ============================================================
// FOR_LOCALE arcs for Expressions (ADR-029 compliance)
// ============================================================

// fil-PH expressions
MATCH (e:Expression {key: 'expr:mabuhay@fil-PH'})
MATCH (l:Locale {key: 'fil-PH'})
MERGE (e)-[:FOR_LOCALE]->(l);

MATCH (e:Expression {key: 'expr:salamat-po@fil-PH'})
MATCH (l:Locale {key: 'fil-PH'})
MERGE (e)-[:FOR_LOCALE]->(l);

MATCH (e:Expression {key: 'expr:diskarte-lang@fil-PH'})
MATCH (l:Locale {key: 'fil-PH'})
MERGE (e)-[:FOR_LOCALE]->(l);

// lo-LA expressions
MATCH (e:Expression {key: 'expr:sabaidee@lo-LA'})
MATCH (l:Locale {key: 'lo-LA'})
MERGE (e)-[:FOR_LOCALE]->(l);

MATCH (e:Expression {key: 'expr:khop-jai@lo-LA'})
MATCH (l:Locale {key: 'lo-LA'})
MERGE (e)-[:FOR_LOCALE]->(l);

MATCH (e:Expression {key: 'expr:khop-jai-jao@lo-LA'})
MATCH (l:Locale {key: 'lo-LA'})
MERGE (e)-[:FOR_LOCALE]->(l);

// or-IN expressions
MATCH (e:Expression {key: 'expr:namaskara@or-IN'})
MATCH (l:Locale {key: 'or-IN'})
MERGE (e)-[:FOR_LOCALE]->(l);

MATCH (e:Expression {key: 'expr:dhanyabad@or-IN'})
MATCH (l:Locale {key: 'or-IN'})
MERGE (e)-[:FOR_LOCALE]->(l);

MATCH (e:Expression {key: 'expr:jai-jagannath@or-IN'})
MATCH (l:Locale {key: 'or-IN'})
MERGE (e)-[:FOR_LOCALE]->(l);

