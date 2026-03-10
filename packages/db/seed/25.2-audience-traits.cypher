// ============================================================================
// AUDIENCE TRAITS SEED - Extracted from Culture.communication_norms
// Generated: 2026-03-10T18:26:19.682Z
// Source: 24-culture.cypher
// ============================================================================

// Note: Each AudienceTrait describes communication style for a locale
// These inform tone, formality, and content structure

// ----------------------------------------------------------------------------
// ceb-PH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ceb-PH'})
MATCH (as:AudienceSet {key: 'audience-set:general@ceb-PH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ceb-PH'})
SET at.display_name = 'Communication Style for ceb-PH',
    at.locale = 'ceb-PH',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ceb-PH. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ceb-PH'})
MATCH (at:AudienceTrait {key: 'communication-style@ceb-PH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// mn-MN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mn-MN'})
MATCH (as:AudienceSet {key: 'audience-set:general@mn-MN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@mn-MN'})
SET at.display_name = 'Communication Style for mn-MN',
    at.locale = 'mn-MN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for mn-MN. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@mn-MN'})
MATCH (at:AudienceTrait {key: 'communication-style@mn-MN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-MY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-MY'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-MY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-MY'})
SET at.display_name = 'Communication Style for en-MY',
    at.locale = 'en-MY',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-MY. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-MY'})
MATCH (at:AudienceTrait {key: 'communication-style@en-MY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// bn-BD AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'bn-BD'})
MATCH (as:AudienceSet {key: 'audience-set:general@bn-BD'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@bn-BD'})
SET at.display_name = 'Communication Style for bn-BD',
    at.locale = 'bn-BD',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for bn-BD. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@bn-BD'})
MATCH (at:AudienceTrait {key: 'communication-style@bn-BD'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-TZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-TZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-TZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-TZ'})
SET at.display_name = 'Communication Style for en-TZ',
    at.locale = 'en-TZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-TZ. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-TZ'})
MATCH (at:AudienceTrait {key: 'communication-style@en-TZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-SG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-SG'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-SG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-SG'})
SET at.display_name = 'Communication Style for en-SG',
    at.locale = 'en-SG',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-SG. STYLE: balanced communication, medium hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-SG'})
MATCH (at:AudienceTrait {key: 'communication-style@en-SG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ta-LK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ta-LK'})
MATCH (as:AudienceSet {key: 'audience-set:general@ta-LK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ta-LK'})
SET at.display_name = 'Communication Style for ta-LK',
    at.locale = 'ta-LK',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ta-LK. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ta-LK'})
MATCH (at:AudienceTrait {key: 'communication-style@ta-LK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-ZA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-ZA'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-ZA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-ZA'})
SET at.display_name = 'Communication Style for en-ZA',
    at.locale = 'en-ZA',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-ZA. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-ZA'})
MATCH (at:AudienceTrait {key: 'communication-style@en-ZA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-BB AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-BB'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-BB'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-BB'})
SET at.display_name = 'Communication Style for en-BB',
    at.locale = 'en-BB',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-BB. STYLE: balanced communication, medium hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-BB'})
MATCH (at:AudienceTrait {key: 'communication-style@en-BB'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// si-LK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'si-LK'})
MATCH (as:AudienceSet {key: 'audience-set:general@si-LK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@si-LK'})
SET at.display_name = 'Communication Style for si-LK',
    at.locale = 'si-LK',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for si-LK. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@si-LK'})
MATCH (at:AudienceTrait {key: 'communication-style@si-LK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// bs-BA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'bs-BA'})
MATCH (as:AudienceSet {key: 'audience-set:general@bs-BA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@bs-BA'})
SET at.display_name = 'Communication Style for bs-BA',
    at.locale = 'bs-BA',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for bs-BA. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@bs-BA'})
MATCH (at:AudienceTrait {key: 'communication-style@bs-BA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// it-CH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'it-CH'})
MATCH (as:AudienceSet {key: 'audience-set:general@it-CH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@it-CH'})
SET at.display_name = 'Communication Style for it-CH',
    at.locale = 'it-CH',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for it-CH. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@it-CH'})
MATCH (at:AudienceTrait {key: 'communication-style@it-CH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// tr-TR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'tr-TR'})
MATCH (as:AudienceSet {key: 'audience-set:general@tr-TR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@tr-TR'})
SET at.display_name = 'Communication Style for tr-TR',
    at.locale = 'tr-TR',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for tr-TR. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@tr-TR'})
MATCH (at:AudienceTrait {key: 'communication-style@tr-TR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-MX AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-MX'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-MX'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-MX'})
SET at.display_name = 'Communication Style for es-MX',
    at.locale = 'es-MX',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-MX. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-MX'})
MATCH (at:AudienceTrait {key: 'communication-style@es-MX'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-MA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-MA'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-MA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-MA'})
SET at.display_name = 'Communication Style for ar-MA',
    at.locale = 'ar-MA',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-MA. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-MA'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-MA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-LY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-LY'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-LY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-LY'})
SET at.display_name = 'Communication Style for ar-LY',
    at.locale = 'ar-LY',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-LY. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-LY'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-LY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-MG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-MG'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-MG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-MG'})
SET at.display_name = 'Communication Style for fr-MG',
    at.locale = 'fr-MG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-MG. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-MG'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-MG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-EG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-EG'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-EG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-EG'})
SET at.display_name = 'Communication Style for ar-EG',
    at.locale = 'ar-EG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-EG. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-EG'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-EG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// my-MM AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'my-MM'})
MATCH (as:AudienceSet {key: 'audience-set:general@my-MM'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@my-MM'})
SET at.display_name = 'Communication Style for my-MM',
    at.locale = 'my-MM',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for my-MM. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@my-MM'})
MATCH (at:AudienceTrait {key: 'communication-style@my-MM'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ta-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ta-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@ta-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ta-IN'})
SET at.display_name = 'Communication Style for ta-IN',
    at.locale = 'ta-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ta-IN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ta-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@ta-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-CD AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CD'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-CD'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-CD'})
SET at.display_name = 'Communication Style for fr-CD',
    at.locale = 'fr-CD',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-CD. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-CD'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-CD'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// as-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'as-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@as-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@as-IN'})
SET at.display_name = 'Communication Style for as-IN',
    at.locale = 'as-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for as-IN. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@as-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@as-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// lt-LT AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'lt-LT'})
MATCH (as:AudienceSet {key: 'audience-set:general@lt-LT'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@lt-LT'})
SET at.display_name = 'Communication Style for lt-LT',
    at.locale = 'lt-LT',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for lt-LT. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@lt-LT'})
MATCH (at:AudienceTrait {key: 'communication-style@lt-LT'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// sw-KE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sw-KE'})
MATCH (as:AudienceSet {key: 'audience-set:general@sw-KE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@sw-KE'})
SET at.display_name = 'Communication Style for sw-KE',
    at.locale = 'sw-KE',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for sw-KE. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@sw-KE'})
MATCH (at:AudienceTrait {key: 'communication-style@sw-KE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// hy-AM AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'hy-AM'})
MATCH (as:AudienceSet {key: 'audience-set:general@hy-AM'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@hy-AM'})
SET at.display_name = 'Communication Style for hy-AM',
    at.locale = 'hy-AM',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for hy-AM. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@hy-AM'})
MATCH (at:AudienceTrait {key: 'communication-style@hy-AM'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// pt-MZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-MZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@pt-MZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@pt-MZ'})
SET at.display_name = 'Communication Style for pt-MZ',
    at.locale = 'pt-MZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for pt-MZ. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@pt-MZ'})
MATCH (at:AudienceTrait {key: 'communication-style@pt-MZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// nl-BE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'nl-BE'})
MATCH (as:AudienceSet {key: 'audience-set:general@nl-BE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@nl-BE'})
SET at.display_name = 'Communication Style for nl-BE',
    at.locale = 'nl-BE',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for nl-BE. STYLE: balanced communication, medium hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@nl-BE'})
MATCH (at:AudienceTrait {key: 'communication-style@nl-BE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// te-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'te-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@te-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@te-IN'})
SET at.display_name = 'Communication Style for te-IN',
    at.locale = 'te-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for te-IN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@te-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@te-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ml-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ml-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@ml-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ml-IN'})
SET at.display_name = 'Communication Style for ml-IN',
    at.locale = 'ml-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for ml-IN. STYLE: balanced communication, medium hierarchy importance, MEDIUM_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ml-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@ml-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// vi-VN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'vi-VN'})
MATCH (as:AudienceSet {key: 'audience-set:general@vi-VN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@vi-VN'})
SET at.display_name = 'Communication Style for vi-VN',
    at.locale = 'vi-VN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for vi-VN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@vi-VN'})
MATCH (at:AudienceTrait {key: 'communication-style@vi-VN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-BE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-BE'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-BE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-BE'})
SET at.display_name = 'Communication Style for fr-BE',
    at.locale = 'fr-BE',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for fr-BE. STYLE: balanced communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-BE'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-BE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ru-IL AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-IL'})
MATCH (as:AudienceSet {key: 'audience-set:general@ru-IL'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ru-IL'})
SET at.display_name = 'Communication Style for ru-IL',
    at.locale = 'ru-IL',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for ru-IL. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ru-IL'})
MATCH (at:AudienceTrait {key: 'communication-style@ru-IL'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// rw-RW AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'rw-RW'})
MATCH (as:AudienceSet {key: 'audience-set:general@rw-RW'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@rw-RW'})
SET at.display_name = 'Communication Style for rw-RW',
    at.locale = 'rw-RW',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for rw-RW. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@rw-RW'})
MATCH (at:AudienceTrait {key: 'communication-style@rw-RW'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-KY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-KY'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-KY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-KY'})
SET at.display_name = 'Communication Style for en-KY',
    at.locale = 'en-KY',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_TO_MODERATE_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-KY. STYLE: balanced communication, medium hierarchy importance, LOW_TO_MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-KY'})
MATCH (at:AudienceTrait {key: 'communication-style@en-KY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ky-KG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ky-KG'})
MATCH (as:AudienceSet {key: 'audience-set:general@ky-KG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ky-KG'})
SET at.display_name = 'Communication Style for ky-KG',
    at.locale = 'ky-KG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ky-KG. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ky-KG'})
MATCH (at:AudienceTrait {key: 'communication-style@ky-KG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-IE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-IE'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-IE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-IE'})
SET at.display_name = 'Communication Style for en-IE',
    at.locale = 'en-IE',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-IE. STYLE: balanced communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-IE'})
MATCH (at:AudienceTrait {key: 'communication-style@en-IE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// su-ID AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'su-ID'})
MATCH (as:AudienceSet {key: 'audience-set:general@su-ID'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@su-ID'})
SET at.display_name = 'Communication Style for su-ID',
    at.locale = 'su-ID',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for su-ID. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@su-ID'})
MATCH (at:AudienceTrait {key: 'communication-style@su-ID'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-UG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-UG'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-UG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-UG'})
SET at.display_name = 'Communication Style for en-UG',
    at.locale = 'en-UG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-UG. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-UG'})
MATCH (at:AudienceTrait {key: 'communication-style@en-UG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// sw-TZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sw-TZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@sw-TZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@sw-TZ'})
SET at.display_name = 'Communication Style for sw-TZ',
    at.locale = 'sw-TZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for sw-TZ. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@sw-TZ'})
MATCH (at:AudienceTrait {key: 'communication-style@sw-TZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// hu-HU AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'hu-HU'})
MATCH (as:AudienceSet {key: 'audience-set:general@hu-HU'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@hu-HU'})
SET at.display_name = 'Communication Style for hu-HU',
    at.locale = 'hu-HU',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for hu-HU. STYLE: direct communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@hu-HU'})
MATCH (at:AudienceTrait {key: 'communication-style@hu-HU'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-EC AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-EC'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-EC'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-EC'})
SET at.display_name = 'Communication Style for es-EC',
    at.locale = 'es-EC',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-EC. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-EC'})
MATCH (at:AudienceTrait {key: 'communication-style@es-EC'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// mg-MG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mg-MG'})
MATCH (as:AudienceSet {key: 'audience-set:general@mg-MG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@mg-MG'})
SET at.display_name = 'Communication Style for mg-MG',
    at.locale = 'mg-MG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for mg-MG. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@mg-MG'})
MATCH (at:AudienceTrait {key: 'communication-style@mg-MG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ca-AD AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ca-AD'})
MATCH (as:AudienceSet {key: 'audience-set:general@ca-AD'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ca-AD'})
SET at.display_name = 'Communication Style for ca-AD',
    at.locale = 'ca-AD',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for ca-AD. STYLE: balanced communication, low hierarchy importance, LOW.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ca-AD'})
MATCH (at:AudienceTrait {key: 'communication-style@ca-AD'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ko-KR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ko-KR'})
MATCH (as:AudienceSet {key: 'audience-set:general@ko-KR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ko-KR'})
SET at.display_name = 'Communication Style for ko-KR',
    at.locale = 'ko-KR',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ko-KR. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ko-KR'})
MATCH (at:AudienceTrait {key: 'communication-style@ko-KR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ln-CD AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ln-CD'})
MATCH (as:AudienceSet {key: 'audience-set:general@ln-CD'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ln-CD'})
SET at.display_name = 'Communication Style for ln-CD',
    at.locale = 'ln-CD',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ln-CD. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ln-CD'})
MATCH (at:AudienceTrait {key: 'communication-style@ln-CD'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-KE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-KE'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-KE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-KE'})
SET at.display_name = 'Communication Style for en-KE',
    at.locale = 'en-KE',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-KE. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-KE'})
MATCH (at:AudienceTrait {key: 'communication-style@en-KE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-DZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-DZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-DZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-DZ'})
SET at.display_name = 'Communication Style for ar-DZ',
    at.locale = 'ar-DZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-DZ. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-DZ'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-DZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-CA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CA'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-CA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-CA'})
SET at.display_name = 'Communication Style for fr-CA',
    at.locale = 'fr-CA',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for fr-CA. STYLE: balanced communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-CA'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-CA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-GB AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-GB'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-GB'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-GB'})
SET at.display_name = 'Communication Style for en-GB',
    at.locale = 'en-GB',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-GB. STYLE: indirect communication, medium hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-GB'})
MATCH (at:AudienceTrait {key: 'communication-style@en-GB'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-RW AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-RW'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-RW'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-RW'})
SET at.display_name = 'Communication Style for fr-RW',
    at.locale = 'fr-RW',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-RW. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-RW'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-RW'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ru-BY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-BY'})
MATCH (as:AudienceSet {key: 'audience-set:general@ru-BY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ru-BY'})
SET at.display_name = 'Communication Style for ru-BY',
    at.locale = 'ru-BY',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ru-BY. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ru-BY'})
MATCH (at:AudienceTrait {key: 'communication-style@ru-BY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-GT AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-GT'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-GT'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-GT'})
SET at.display_name = 'Communication Style for es-GT',
    at.locale = 'es-GT',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-GT. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-GT'})
MATCH (at:AudienceTrait {key: 'communication-style@es-GT'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ms-SG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ms-SG'})
MATCH (as:AudienceSet {key: 'audience-set:general@ms-SG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ms-SG'})
SET at.display_name = 'Communication Style for ms-SG',
    at.locale = 'ms-SG',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for ms-SG. STYLE: balanced communication, medium hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ms-SG'})
MATCH (at:AudienceTrait {key: 'communication-style@ms-SG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ht-HT AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ht-HT'})
MATCH (as:AudienceSet {key: 'audience-set:general@ht-HT'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ht-HT'})
SET at.display_name = 'Communication Style for ht-HT',
    at.locale = 'ht-HT',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ht-HT. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ht-HT'})
MATCH (at:AudienceTrait {key: 'communication-style@ht-HT'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ms-MY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ms-MY'})
MATCH (as:AudienceSet {key: 'audience-set:general@ms-MY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ms-MY'})
SET at.display_name = 'Communication Style for ms-MY',
    at.locale = 'ms-MY',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ms-MY. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ms-MY'})
MATCH (at:AudienceTrait {key: 'communication-style@ms-MY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// sq-AL AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sq-AL'})
MATCH (as:AudienceSet {key: 'audience-set:general@sq-AL'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@sq-AL'})
SET at.display_name = 'Communication Style for sq-AL',
    at.locale = 'sq-AL',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for sq-AL. STYLE: balanced communication, medium hierarchy importance, MODERATE.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@sq-AL'})
MATCH (at:AudienceTrait {key: 'communication-style@sq-AL'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// mk-MK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mk-MK'})
MATCH (as:AudienceSet {key: 'audience-set:general@mk-MK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@mk-MK'})
SET at.display_name = 'Communication Style for mk-MK',
    at.locale = 'mk-MK',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for mk-MK. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@mk-MK'})
MATCH (at:AudienceTrait {key: 'communication-style@mk-MK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// mr-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mr-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@mr-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@mr-IN'})
SET at.display_name = 'Communication Style for mr-IN',
    at.locale = 'mr-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for mr-IN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@mr-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@mr-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-CU AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-CU'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-CU'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-CU'})
SET at.display_name = 'Communication Style for es-CU',
    at.locale = 'es-CU',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-CU. STYLE: balanced communication, low hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-CU'})
MATCH (at:AudienceTrait {key: 'communication-style@es-CU'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ru-KG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-KG'})
MATCH (as:AudienceSet {key: 'audience-set:general@ru-KG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ru-KG'})
SET at.display_name = 'Communication Style for ru-KG',
    at.locale = 'ru-KG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ru-KG. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ru-KG'})
MATCH (at:AudienceTrait {key: 'communication-style@ru-KG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ig-NG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ig-NG'})
MATCH (as:AudienceSet {key: 'audience-set:general@ig-NG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ig-NG'})
SET at.display_name = 'Communication Style for ig-NG',
    at.locale = 'ig-NG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ig-NG. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ig-NG'})
MATCH (at:AudienceTrait {key: 'communication-style@ig-NG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ka-GE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ka-GE'})
MATCH (as:AudienceSet {key: 'audience-set:general@ka-GE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ka-GE'})
SET at.display_name = 'Communication Style for ka-GE',
    at.locale = 'ka-GE',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ka-GE. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ka-GE'})
MATCH (at:AudienceTrait {key: 'communication-style@ka-GE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-BH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-BH'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-BH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-BH'})
SET at.display_name = 'Communication Style for ar-BH',
    at.locale = 'ar-BH',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-BH. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-BH'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-BH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-LU AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-LU'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-LU'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-LU'})
SET at.display_name = 'Communication Style for fr-LU',
    at.locale = 'fr-LU',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for fr-LU. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-LU'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-LU'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-ZW AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-ZW'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-ZW'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-ZW'})
SET at.display_name = 'Communication Style for en-ZW',
    at.locale = 'en-ZW',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-ZW. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-ZW'})
MATCH (at:AudienceTrait {key: 'communication-style@en-ZW'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-IN'})
SET at.display_name = 'Communication Style for en-IN',
    at.locale = 'en-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-IN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@en-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// zh-TW AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-TW'})
MATCH (as:AudienceSet {key: 'audience-set:general@zh-TW'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@zh-TW'})
SET at.display_name = 'Communication Style for zh-TW',
    at.locale = 'zh-TW',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for zh-TW. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@zh-TW'})
MATCH (at:AudienceTrait {key: 'communication-style@zh-TW'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// de-CH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'de-CH'})
MATCH (as:AudienceSet {key: 'audience-set:general@de-CH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@de-CH'})
SET at.display_name = 'Communication Style for de-CH',
    at.locale = 'de-CH',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for de-CH. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@de-CH'})
MATCH (at:AudienceTrait {key: 'communication-style@de-CH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// pt-BR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-BR'})
MATCH (as:AudienceSet {key: 'audience-set:general@pt-BR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@pt-BR'})
SET at.display_name = 'Communication Style for pt-BR',
    at.locale = 'pt-BR',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for pt-BR. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@pt-BR'})
MATCH (at:AudienceTrait {key: 'communication-style@pt-BR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fa-IR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fa-IR'})
MATCH (as:AudienceSet {key: 'audience-set:general@fa-IR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fa-IR'})
SET at.display_name = 'Communication Style for fa-IR',
    at.locale = 'fa-IR',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fa-IR. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fa-IR'})
MATCH (at:AudienceTrait {key: 'communication-style@fa-IR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// de-AT AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'de-AT'})
MATCH (as:AudienceSet {key: 'audience-set:general@de-AT'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@de-AT'})
SET at.display_name = 'Communication Style for de-AT',
    at.locale = 'de-AT',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for de-AT. STYLE: indirect communication, high hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@de-AT'})
MATCH (at:AudienceTrait {key: 'communication-style@de-AT'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-TN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-TN'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-TN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-TN'})
SET at.display_name = 'Communication Style for fr-TN',
    at.locale = 'fr-TN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-TN. STYLE: direct communication, low hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-TN'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-TN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-TT AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-TT'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-TT'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-TT'})
SET at.display_name = 'Communication Style for en-TT',
    at.locale = 'en-TT',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-TT. STYLE: balanced communication, medium hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-TT'})
MATCH (at:AudienceTrait {key: 'communication-style@en-TT'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// is-IS AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'is-IS'})
MATCH (as:AudienceSet {key: 'audience-set:general@is-IS'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@is-IS'})
SET at.display_name = 'Communication Style for is-IS',
    at.locale = 'is-IS',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for is-IS. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@is-IS'})
MATCH (at:AudienceTrait {key: 'communication-style@is-IS'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ca-ES AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ca-ES'})
MATCH (as:AudienceSet {key: 'audience-set:general@ca-ES'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ca-ES'})
SET at.display_name = 'Communication Style for ca-ES',
    at.locale = 'ca-ES',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for ca-ES. STYLE: balanced communication, medium hierarchy importance, MEDIUM_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ca-ES'})
MATCH (at:AudienceTrait {key: 'communication-style@ca-ES'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ru-KZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-KZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@ru-KZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ru-KZ'})
SET at.display_name = 'Communication Style for ru-KZ',
    at.locale = 'ru-KZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ru-KZ. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ru-KZ'})
MATCH (at:AudienceTrait {key: 'communication-style@ru-KZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// kn-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'kn-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@kn-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@kn-IN'})
SET at.display_name = 'Communication Style for kn-IN',
    at.locale = 'kn-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for kn-IN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@kn-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@kn-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ro-RO AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ro-RO'})
MATCH (as:AudienceSet {key: 'audience-set:general@ro-RO'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ro-RO'})
SET at.display_name = 'Communication Style for ro-RO',
    at.locale = 'ro-RO',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for ro-RO. STYLE: balanced communication, medium hierarchy importance, MEDIUM.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ro-RO'})
MATCH (at:AudienceTrait {key: 'communication-style@ro-RO'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// gu-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'gu-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@gu-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@gu-IN'})
SET at.display_name = 'Communication Style for gu-IN',
    at.locale = 'gu-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for gu-IN. STYLE: balanced communication, medium hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@gu-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@gu-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// pt-AO AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-AO'})
MATCH (as:AudienceSet {key: 'audience-set:general@pt-AO'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@pt-AO'})
SET at.display_name = 'Communication Style for pt-AO',
    at.locale = 'pt-AO',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for pt-AO. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@pt-AO'})
MATCH (at:AudienceTrait {key: 'communication-style@pt-AO'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ny-MW AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ny-MW'})
MATCH (as:AudienceSet {key: 'audience-set:general@ny-MW'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ny-MW'})
SET at.display_name = 'Communication Style for ny-MW',
    at.locale = 'ny-MW',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ny-MW. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ny-MW'})
MATCH (at:AudienceTrait {key: 'communication-style@ny-MW'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// or-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'or-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@or-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@or-IN'})
SET at.display_name = 'Communication Style for or-IN',
    at.locale = 'or-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for or-IN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@or-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@or-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// zh-CN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-CN'})
MATCH (as:AudienceSet {key: 'audience-set:general@zh-CN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@zh-CN'})
SET at.display_name = 'Communication Style for zh-CN',
    at.locale = 'zh-CN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for zh-CN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@zh-CN'})
MATCH (at:AudienceTrait {key: 'communication-style@zh-CN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// sn-ZW AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sn-ZW'})
MATCH (as:AudienceSet {key: 'audience-set:general@sn-ZW'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@sn-ZW'})
SET at.display_name = 'Communication Style for sn-ZW',
    at.locale = 'sn-ZW',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for sn-ZW. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@sn-ZW'})
MATCH (at:AudienceTrait {key: 'communication-style@sn-ZW'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-JO AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-JO'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-JO'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-JO'})
SET at.display_name = 'Communication Style for ar-JO',
    at.locale = 'ar-JO',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-JO. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-JO'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-JO'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-KW AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-KW'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-KW'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-KW'})
SET at.display_name = 'Communication Style for ar-KW',
    at.locale = 'ar-KW',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-KW. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-KW'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-KW'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-AU AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-AU'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-AU'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-AU'})
SET at.display_name = 'Communication Style for en-AU',
    at.locale = 'en-AU',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-AU. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-AU'})
MATCH (at:AudienceTrait {key: 'communication-style@en-AU'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-FR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-FR'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-FR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-FR'})
SET at.display_name = 'Communication Style for fr-FR',
    at.locale = 'fr-FR',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-FR. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-FR'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-FR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-SN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-SN'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-SN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-SN'})
SET at.display_name = 'Communication Style for fr-SN',
    at.locale = 'fr-SN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-SN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-SN'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-SN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// pa-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pa-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@pa-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@pa-IN'})
SET at.display_name = 'Communication Style for pa-IN',
    at.locale = 'pa-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for pa-IN. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@pa-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@pa-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// cs-CZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'cs-CZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@cs-CZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@cs-CZ'})
SET at.display_name = 'Communication Style for cs-CZ',
    at.locale = 'cs-CZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for cs-CZ. STYLE: direct communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@cs-CZ'})
MATCH (at:AudienceTrait {key: 'communication-style@cs-CZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-CL AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-CL'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-CL'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-CL'})
SET at.display_name = 'Communication Style for es-CL',
    at.locale = 'es-CL',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for es-CL. STYLE: balanced communication, medium hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-CL'})
MATCH (at:AudienceTrait {key: 'communication-style@es-CL'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// nl-NL AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'nl-NL'})
MATCH (as:AudienceSet {key: 'audience-set:general@nl-NL'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@nl-NL'})
SET at.display_name = 'Communication Style for nl-NL',
    at.locale = 'nl-NL',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for nl-NL. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@nl-NL'})
MATCH (at:AudienceTrait {key: 'communication-style@nl-NL'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-HN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-HN'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-HN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-HN'})
SET at.display_name = 'Communication Style for es-HN',
    at.locale = 'es-HN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-HN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-HN'})
MATCH (at:AudienceTrait {key: 'communication-style@es-HN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// bg-BG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'bg-BG'})
MATCH (as:AudienceSet {key: 'audience-set:general@bg-BG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@bg-BG'})
SET at.display_name = 'Communication Style for bg-BG',
    at.locale = 'bg-BG',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for bg-BG. STYLE: balanced communication, medium hierarchy importance, MEDIUM.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@bg-BG'})
MATCH (at:AudienceTrait {key: 'communication-style@bg-BG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// sd-PK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sd-PK'})
MATCH (as:AudienceSet {key: 'audience-set:general@sd-PK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@sd-PK'})
SET at.display_name = 'Communication Style for sd-PK',
    at.locale = 'sd-PK',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for sd-PK. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@sd-PK'})
MATCH (at:AudienceTrait {key: 'communication-style@sd-PK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-GH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-GH'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-GH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-GH'})
SET at.display_name = 'Communication Style for en-GH',
    at.locale = 'en-GH',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-GH. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-GH'})
MATCH (at:AudienceTrait {key: 'communication-style@en-GH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-PH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-PH'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-PH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-PH'})
SET at.display_name = 'Communication Style for en-PH',
    at.locale = 'en-PH',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-PH. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-PH'})
MATCH (at:AudienceTrait {key: 'communication-style@en-PH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// sl-SI AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sl-SI'})
MATCH (as:AudienceSet {key: 'audience-set:general@sl-SI'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@sl-SI'})
SET at.display_name = 'Communication Style for sl-SI',
    at.locale = 'sl-SI',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for sl-SI. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@sl-SI'})
MATCH (at:AudienceTrait {key: 'communication-style@sl-SI'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-HK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-HK'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-HK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-HK'})
SET at.display_name = 'Communication Style for en-HK',
    at.locale = 'en-HK',
    at.segment = 'general',
    at.behavior = '{"context_type":"MODERATE_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-HK. STYLE: balanced communication, medium hierarchy importance, MODERATE_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-HK'})
MATCH (at:AudienceTrait {key: 'communication-style@en-HK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-MU AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-MU'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-MU'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-MU'})
SET at.display_name = 'Communication Style for en-MU',
    at.locale = 'en-MU',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-MU. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-MU'})
MATCH (at:AudienceTrait {key: 'communication-style@en-MU'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// et-EE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'et-EE'})
MATCH (as:AudienceSet {key: 'audience-set:general@et-EE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@et-EE'})
SET at.display_name = 'Communication Style for et-EE',
    at.locale = 'et-EE',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for et-EE. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@et-EE'})
MATCH (at:AudienceTrait {key: 'communication-style@et-EE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-SV AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-SV'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-SV'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-SV'})
SET at.display_name = 'Communication Style for es-SV',
    at.locale = 'es-SV',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-SV. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-SV'})
MATCH (at:AudienceTrait {key: 'communication-style@es-SV'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// pt-CH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-CH'})
MATCH (as:AudienceSet {key: 'audience-set:general@pt-CH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@pt-CH'})
SET at.display_name = 'Communication Style for pt-CH',
    at.locale = 'pt-CH',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for pt-CH. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@pt-CH'})
MATCH (at:AudienceTrait {key: 'communication-style@pt-CH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-PR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-PR'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-PR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-PR'})
SET at.display_name = 'Communication Style for es-PR',
    at.locale = 'es-PR',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for es-PR. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-PR'})
MATCH (at:AudienceTrait {key: 'communication-style@es-PR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ur-PK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ur-PK'})
MATCH (as:AudienceSet {key: 'audience-set:general@ur-PK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ur-PK'})
SET at.display_name = 'Communication Style for ur-PK',
    at.locale = 'ur-PK',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ur-PK. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ur-PK'})
MATCH (at:AudienceTrait {key: 'communication-style@ur-PK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-ZM AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-ZM'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-ZM'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-ZM'})
SET at.display_name = 'Communication Style for en-ZM',
    at.locale = 'en-ZM',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-ZM. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-ZM'})
MATCH (at:AudienceTrait {key: 'communication-style@en-ZM'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-OM AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-OM'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-OM'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-OM'})
SET at.display_name = 'Communication Style for ar-OM',
    at.locale = 'ar-OM',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-OM. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-OM'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-OM'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ru-MD AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-MD'})
MATCH (as:AudienceSet {key: 'audience-set:general@ru-MD'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ru-MD'})
SET at.display_name = 'Communication Style for ru-MD',
    at.locale = 'ru-MD',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ru-MD. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ru-MD'})
MATCH (at:AudienceTrait {key: 'communication-style@ru-MD'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// mt-MT AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mt-MT'})
MATCH (as:AudienceSet {key: 'audience-set:general@mt-MT'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@mt-MT'})
SET at.display_name = 'Communication Style for mt-MT',
    at.locale = 'mt-MT',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM_CONTEXT","directness":"direct","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for mt-MT. STYLE: direct communication, medium hierarchy importance, MEDIUM_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@mt-MT'})
MATCH (at:AudienceTrait {key: 'communication-style@mt-MT'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// eu-ES AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'eu-ES'})
MATCH (as:AudienceSet {key: 'audience-set:general@eu-ES'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@eu-ES'})
SET at.display_name = 'Communication Style for eu-ES',
    at.locale = 'eu-ES',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for eu-ES. STYLE: direct communication, low hierarchy importance, LOW.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@eu-ES'})
MATCH (at:AudienceTrait {key: 'communication-style@eu-ES'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ja-JP AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ja-JP'})
MATCH (as:AudienceSet {key: 'audience-set:general@ja-JP'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ja-JP'})
SET at.display_name = 'Communication Style for ja-JP',
    at.locale = 'ja-JP',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ja-JP. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ja-JP'})
MATCH (at:AudienceTrait {key: 'communication-style@ja-JP'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// bn-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'bn-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@bn-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@bn-IN'})
SET at.display_name = 'Communication Style for bn-IN',
    at.locale = 'bn-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for bn-IN. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@bn-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@bn-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// he-IL AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'he-IL'})
MATCH (as:AudienceSet {key: 'audience-set:general@he-IL'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@he-IL'})
SET at.display_name = 'Communication Style for he-IL',
    at.locale = 'he-IL',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for he-IL. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@he-IL'})
MATCH (at:AudienceTrait {key: 'communication-style@he-IL'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// zh-HK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-HK'})
MATCH (as:AudienceSet {key: 'audience-set:general@zh-HK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@zh-HK'})
SET at.display_name = 'Communication Style for zh-HK',
    at.locale = 'zh-HK',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for zh-HK. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@zh-HK'})
MATCH (at:AudienceTrait {key: 'communication-style@zh-HK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-CO AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-CO'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-CO'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-CO'})
SET at.display_name = 'Communication Style for es-CO',
    at.locale = 'es-CO',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-CO. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-CO'})
MATCH (at:AudienceTrait {key: 'communication-style@es-CO'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// de-LU AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'de-LU'})
MATCH (as:AudienceSet {key: 'audience-set:general@de-LU'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@de-LU'})
SET at.display_name = 'Communication Style for de-LU',
    at.locale = 'de-LU',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for de-LU. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@de-LU'})
MATCH (at:AudienceTrait {key: 'communication-style@de-LU'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-BO AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-BO'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-BO'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-BO'})
SET at.display_name = 'Communication Style for es-BO',
    at.locale = 'es-BO',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-BO. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-BO'})
MATCH (at:AudienceTrait {key: 'communication-style@es-BO'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// th-TH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'th-TH'})
MATCH (as:AudienceSet {key: 'audience-set:general@th-TH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@th-TH'})
SET at.display_name = 'Communication Style for th-TH',
    at.locale = 'th-TH',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for th-TH. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@th-TH'})
MATCH (at:AudienceTrait {key: 'communication-style@th-TH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// kk-KZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'kk-KZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@kk-KZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@kk-KZ'})
SET at.display_name = 'Communication Style for kk-KZ',
    at.locale = 'kk-KZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for kk-KZ. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@kk-KZ'})
MATCH (at:AudienceTrait {key: 'communication-style@kk-KZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fi-FI AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fi-FI'})
MATCH (as:AudienceSet {key: 'audience-set:general@fi-FI'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fi-FI'})
SET at.display_name = 'Communication Style for fi-FI',
    at.locale = 'fi-FI',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for fi-FI. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fi-FI'})
MATCH (at:AudienceTrait {key: 'communication-style@fi-FI'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-TN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-TN'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-TN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-TN'})
SET at.display_name = 'Communication Style for ar-TN',
    at.locale = 'ar-TN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-TN. STYLE: direct communication, low hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-TN'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-TN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-VN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-VN'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-VN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-VN'})
SET at.display_name = 'Communication Style for en-VN',
    at.locale = 'en-VN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-VN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-VN'})
MATCH (at:AudienceTrait {key: 'communication-style@en-VN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// tl-PH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'tl-PH'})
MATCH (as:AudienceSet {key: 'audience-set:general@tl-PH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@tl-PH'})
SET at.display_name = 'Communication Style for tl-PH',
    at.locale = 'tl-PH',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for tl-PH. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@tl-PH'})
MATCH (at:AudienceTrait {key: 'communication-style@tl-PH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-PK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-PK'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-PK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-PK'})
SET at.display_name = 'Communication Style for en-PK',
    at.locale = 'en-PK',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-PK. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-PK'})
MATCH (at:AudienceTrait {key: 'communication-style@en-PK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-CH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CH'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-CH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-CH'})
SET at.display_name = 'Communication Style for fr-CH',
    at.locale = 'fr-CH',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for fr-CH. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-CH'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-CH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// cy-GB AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'cy-GB'})
MATCH (as:AudienceSet {key: 'audience-set:general@cy-GB'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@cy-GB'})
SET at.display_name = 'Communication Style for cy-GB',
    at.locale = 'cy-GB',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM_CONTEXT","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for cy-GB. STYLE: balanced communication, low hierarchy importance, MEDIUM_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@cy-GB'})
MATCH (at:AudienceTrait {key: 'communication-style@cy-GB'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-FJ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-FJ'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-FJ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-FJ'})
SET at.display_name = 'Communication Style for en-FJ',
    at.locale = 'en-FJ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-FJ. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-FJ'})
MATCH (at:AudienceTrait {key: 'communication-style@en-FJ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// af-ZA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'af-ZA'})
MATCH (as:AudienceSet {key: 'audience-set:general@af-ZA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@af-ZA'})
SET at.display_name = 'Communication Style for af-ZA',
    at.locale = 'af-ZA',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for af-ZA. STYLE: direct communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@af-ZA'})
MATCH (at:AudienceTrait {key: 'communication-style@af-ZA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-JM AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-JM'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-JM'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-JM'})
SET at.display_name = 'Communication Style for en-JM',
    at.locale = 'en-JM',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-JM. STYLE: direct communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-JM'})
MATCH (at:AudienceTrait {key: 'communication-style@en-JM'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// da-DK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'da-DK'})
MATCH (as:AudienceSet {key: 'audience-set:general@da-DK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@da-DK'})
SET at.display_name = 'Communication Style for da-DK',
    at.locale = 'da-DK',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for da-DK. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@da-DK'})
MATCH (at:AudienceTrait {key: 'communication-style@da-DK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-US AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-US'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-US'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-US'})
SET at.display_name = 'Communication Style for en-US',
    at.locale = 'en-US',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-US. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-US'})
MATCH (at:AudienceTrait {key: 'communication-style@en-US'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// pt-PT AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pt-PT'})
MATCH (as:AudienceSet {key: 'audience-set:general@pt-PT'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@pt-PT'})
SET at.display_name = 'Communication Style for pt-PT',
    at.locale = 'pt-PT',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for pt-PT. STYLE: balanced communication, medium hierarchy importance, HIGH.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@pt-PT'})
MATCH (at:AudienceTrait {key: 'communication-style@pt-PT'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ga-IE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ga-IE'})
MATCH (as:AudienceSet {key: 'audience-set:general@ga-IE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ga-IE'})
SET at.display_name = 'Communication Style for ga-IE',
    at.locale = 'ga-IE',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM_HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for ga-IE. STYLE: balanced communication, low hierarchy importance, MEDIUM_HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ga-IE'})
MATCH (at:AudienceTrait {key: 'communication-style@ga-IE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-AR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-AR'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-AR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-AR'})
SET at.display_name = 'Communication Style for es-AR',
    at.locale = 'es-AR',
    at.segment = 'general',
    at.behavior = '{"context_type":"MIXED","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for es-AR. STYLE: balanced communication, low hierarchy importance, MIXED.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-AR'})
MATCH (at:AudienceTrait {key: 'communication-style@es-AR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// gl-ES AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'gl-ES'})
MATCH (as:AudienceSet {key: 'audience-set:general@gl-ES'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@gl-ES'})
SET at.display_name = 'Communication Style for gl-ES',
    at.locale = 'gl-ES',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for gl-ES. STYLE: balanced communication, medium hierarchy importance, MEDIUM_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@gl-ES'})
MATCH (at:AudienceTrait {key: 'communication-style@gl-ES'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-ES AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-ES'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-ES'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-ES'})
SET at.display_name = 'Communication Style for es-ES',
    at.locale = 'es-ES',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for es-ES. STYLE: balanced communication, medium hierarchy importance, MEDIUM.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-ES'})
MATCH (at:AudienceTrait {key: 'communication-style@es-ES'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// zh-TH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-TH'})
MATCH (as:AudienceSet {key: 'audience-set:general@zh-TH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@zh-TH'})
SET at.display_name = 'Communication Style for zh-TH',
    at.locale = 'zh-TH',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for zh-TH. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@zh-TH'})
MATCH (at:AudienceTrait {key: 'communication-style@zh-TH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// be-BY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'be-BY'})
MATCH (as:AudienceSet {key: 'audience-set:general@be-BY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@be-BY'})
SET at.display_name = 'Communication Style for be-BY',
    at.locale = 'be-BY',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for be-BY. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@be-BY'})
MATCH (at:AudienceTrait {key: 'communication-style@be-BY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// wo-SN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'wo-SN'})
MATCH (as:AudienceSet {key: 'audience-set:general@wo-SN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@wo-SN'})
SET at.display_name = 'Communication Style for wo-SN',
    at.locale = 'wo-SN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for wo-SN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@wo-SN'})
MATCH (at:AudienceTrait {key: 'communication-style@wo-SN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-NI AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-NI'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-NI'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-NI'})
SET at.display_name = 'Communication Style for es-NI',
    at.locale = 'es-NI',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-NI. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-NI'})
MATCH (at:AudienceTrait {key: 'communication-style@es-NI'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// sv-SE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sv-SE'})
MATCH (as:AudienceSet {key: 'audience-set:general@sv-SE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@sv-SE'})
SET at.display_name = 'Communication Style for sv-SE',
    at.locale = 'sv-SE',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for sv-SE. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@sv-SE'})
MATCH (at:AudienceTrait {key: 'communication-style@sv-SE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// hr-HR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'hr-HR'})
MATCH (as:AudienceSet {key: 'audience-set:general@hr-HR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@hr-HR'})
SET at.display_name = 'Communication Style for hr-HR',
    at.locale = 'hr-HR',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for hr-HR. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@hr-HR'})
MATCH (at:AudienceTrait {key: 'communication-style@hr-HR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-CI AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CI'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-CI'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-CI'})
SET at.display_name = 'Communication Style for fr-CI',
    at.locale = 'fr-CI',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-CI. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-CI'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-CI'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ku-TR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ku-TR'})
MATCH (as:AudienceSet {key: 'audience-set:general@ku-TR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ku-TR'})
SET at.display_name = 'Communication Style for ku-TR',
    at.locale = 'ku-TR',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ku-TR. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ku-TR'})
MATCH (at:AudienceTrait {key: 'communication-style@ku-TR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-BW AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-BW'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-BW'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-BW'})
SET at.display_name = 'Communication Style for en-BW',
    at.locale = 'en-BW',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-BW. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-BW'})
MATCH (at:AudienceTrait {key: 'communication-style@en-BW'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-IQ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-IQ'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-IQ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-IQ'})
SET at.display_name = 'Communication Style for ar-IQ',
    at.locale = 'ar-IQ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-IQ. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-IQ'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-IQ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// pa-PK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pa-PK'})
MATCH (as:AudienceSet {key: 'audience-set:general@pa-PK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@pa-PK'})
SET at.display_name = 'Communication Style for pa-PK',
    at.locale = 'pa-PK',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for pa-PK. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@pa-PK'})
MATCH (at:AudienceTrait {key: 'communication-style@pa-PK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-DO AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-DO'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-DO'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-DO'})
SET at.display_name = 'Communication Style for es-DO',
    at.locale = 'es-DO',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"direct","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"direct"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-DO. STYLE: direct communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-DO'})
MATCH (at:AudienceTrait {key: 'communication-style@es-DO'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-CR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-CR'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-CR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-CR'})
SET at.display_name = 'Communication Style for es-CR',
    at.locale = 'es-CR',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-CR. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-CR'})
MATCH (at:AudienceTrait {key: 'communication-style@es-CR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ps-AF AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ps-AF'})
MATCH (as:AudienceSet {key: 'audience-set:general@ps-AF'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ps-AF'})
SET at.display_name = 'Communication Style for ps-AF',
    at.locale = 'ps-AF',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ps-AF. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ps-AF'})
MATCH (at:AudienceTrait {key: 'communication-style@ps-AF'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-CM AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-CM'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-CM'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-CM'})
SET at.display_name = 'Communication Style for fr-CM',
    at.locale = 'fr-CM',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-CM. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-CM'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-CM'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// el-GR AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'el-GR'})
MATCH (as:AudienceSet {key: 'audience-set:general@el-GR'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@el-GR'})
SET at.display_name = 'Communication Style for el-GR',
    at.locale = 'el-GR',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for el-GR. STYLE: balanced communication, medium hierarchy importance, HIGH.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@el-GR'})
MATCH (at:AudienceTrait {key: 'communication-style@el-GR'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// hi-IN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'hi-IN'})
MATCH (as:AudienceSet {key: 'audience-set:general@hi-IN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@hi-IN'})
SET at.display_name = 'Communication Style for hi-IN',
    at.locale = 'hi-IN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for hi-IN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@hi-IN'})
MATCH (at:AudienceTrait {key: 'communication-style@hi-IN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// qu-PE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'qu-PE'})
MATCH (as:AudienceSet {key: 'audience-set:general@qu-PE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@qu-PE'})
SET at.display_name = 'Communication Style for qu-PE',
    at.locale = 'qu-PE',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for qu-PE. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@qu-PE'})
MATCH (at:AudienceTrait {key: 'communication-style@qu-PE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// sr-RS AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sr-RS'})
MATCH (as:AudienceSet {key: 'audience-set:general@sr-RS'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@sr-RS'})
SET at.display_name = 'Communication Style for sr-RS',
    at.locale = 'sr-RS',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for sr-RS. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@sr-RS'})
MATCH (at:AudienceTrait {key: 'communication-style@sr-RS'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ms-BN AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ms-BN'})
MATCH (as:AudienceSet {key: 'audience-set:general@ms-BN'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ms-BN'})
SET at.display_name = 'Communication Style for ms-BN',
    at.locale = 'ms-BN',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ms-BN. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ms-BN'})
MATCH (at:AudienceTrait {key: 'communication-style@ms-BN'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-UY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-UY'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-UY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-UY'})
SET at.display_name = 'Communication Style for es-UY',
    at.locale = 'es-UY',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for es-UY. STYLE: balanced communication, low hierarchy importance, MEDIUM.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-UY'})
MATCH (at:AudienceTrait {key: 'communication-style@es-UY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// zu-ZA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zu-ZA'})
MATCH (as:AudienceSet {key: 'audience-set:general@zu-ZA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@zu-ZA'})
SET at.display_name = 'Communication Style for zu-ZA',
    at.locale = 'zu-ZA',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for zu-ZA. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@zu-ZA'})
MATCH (at:AudienceTrait {key: 'communication-style@zu-ZA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-BF AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-BF'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-BF'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-BF'})
SET at.display_name = 'Communication Style for fr-BF',
    at.locale = 'fr-BF',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-BF. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-BF'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-BF'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-SA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-SA'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-SA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-SA'})
SET at.display_name = 'Communication Style for en-SA',
    at.locale = 'en-SA',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-SA. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-SA'})
MATCH (at:AudienceTrait {key: 'communication-style@en-SA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// id-ID AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'id-ID'})
MATCH (as:AudienceSet {key: 'audience-set:general@id-ID'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@id-ID'})
SET at.display_name = 'Communication Style for id-ID',
    at.locale = 'id-ID',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for id-ID. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@id-ID'})
MATCH (at:AudienceTrait {key: 'communication-style@id-ID'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-QA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-QA'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-QA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-QA'})
SET at.display_name = 'Communication Style for ar-QA',
    at.locale = 'ar-QA',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-QA. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-QA'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-QA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-VE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-VE'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-VE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-VE'})
SET at.display_name = 'Communication Style for es-VE',
    at.locale = 'es-VE',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-VE. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-VE'})
MATCH (at:AudienceTrait {key: 'communication-style@es-VE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// az-AZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'az-AZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@az-AZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@az-AZ'})
SET at.display_name = 'Communication Style for az-AZ',
    at.locale = 'az-AZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for az-AZ. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@az-AZ'})
MATCH (at:AudienceTrait {key: 'communication-style@az-AZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// de-DE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'de-DE'})
MATCH (as:AudienceSet {key: 'audience-set:general@de-DE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@de-DE'})
SET at.display_name = 'Communication Style for de-DE',
    at.locale = 'de-DE',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for de-DE. STYLE: direct communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@de-DE'})
MATCH (at:AudienceTrait {key: 'communication-style@de-DE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-SA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-SA'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-SA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-SA'})
SET at.display_name = 'Communication Style for ar-SA',
    at.locale = 'ar-SA',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-SA. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-SA'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-SA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-MA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-MA'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-MA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-MA'})
SET at.display_name = 'Communication Style for fr-MA',
    at.locale = 'fr-MA',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-MA. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-MA'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-MA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// uz-UZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'uz-UZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@uz-UZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@uz-UZ'})
SET at.display_name = 'Communication Style for uz-UZ',
    at.locale = 'uz-UZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for uz-UZ. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@uz-UZ'})
MATCH (at:AudienceTrait {key: 'communication-style@uz-UZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// lv-LV AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'lv-LV'})
MATCH (as:AudienceSet {key: 'audience-set:general@lv-LV'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@lv-LV'})
SET at.display_name = 'Communication Style for lv-LV',
    at.locale = 'lv-LV',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for lv-LV. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@lv-LV'})
MATCH (at:AudienceTrait {key: 'communication-style@lv-LV'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// xh-ZA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'xh-ZA'})
MATCH (as:AudienceSet {key: 'audience-set:general@xh-ZA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@xh-ZA'})
SET at.display_name = 'Communication Style for xh-ZA',
    at.locale = 'xh-ZA',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for xh-ZA. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@xh-ZA'})
MATCH (at:AudienceTrait {key: 'communication-style@xh-ZA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ro-MD AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ro-MD'})
MATCH (as:AudienceSet {key: 'audience-set:general@ro-MD'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ro-MD'})
SET at.display_name = 'Communication Style for ro-MD',
    at.locale = 'ro-MD',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ro-MD. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ro-MD'})
MATCH (at:AudienceTrait {key: 'communication-style@ro-MD'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-NG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-NG'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-NG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-NG'})
SET at.display_name = 'Communication Style for en-NG',
    at.locale = 'en-NG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-NG. STYLE: balanced communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-NG'})
MATCH (at:AudienceTrait {key: 'communication-style@en-NG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// it-IT AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'it-IT'})
MATCH (as:AudienceSet {key: 'audience-set:general@it-IT'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@it-IT'})
SET at.display_name = 'Communication Style for it-IT',
    at.locale = 'it-IT',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for it-IT. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@it-IT'})
MATCH (at:AudienceTrait {key: 'communication-style@it-IT'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// no-NO AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'no-NO'})
MATCH (as:AudienceSet {key: 'audience-set:general@no-NO'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@no-NO'})
SET at.display_name = 'Communication Style for no-NO',
    at.locale = 'no-NO',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for no-NO. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@no-NO'})
MATCH (at:AudienceTrait {key: 'communication-style@no-NO'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// sk-SK AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'sk-SK'})
MATCH (as:AudienceSet {key: 'audience-set:general@sk-SK'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@sk-SK'})
SET at.display_name = 'Communication Style for sk-SK',
    at.locale = 'sk-SK',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM_CONTEXT","directness":"balanced","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for sk-SK. STYLE: balanced communication, high hierarchy importance, MEDIUM_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@sk-SK'})
MATCH (at:AudienceTrait {key: 'communication-style@sk-SK'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// gn-PY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'gn-PY'})
MATCH (as:AudienceSet {key: 'audience-set:general@gn-PY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@gn-PY'})
SET at.display_name = 'Communication Style for gn-PY',
    at.locale = 'gn-PY',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for gn-PY. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@gn-PY'})
MATCH (at:AudienceTrait {key: 'communication-style@gn-PY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// uk-UA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'uk-UA'})
MATCH (as:AudienceSet {key: 'audience-set:general@uk-UA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@uk-UA'})
SET at.display_name = 'Communication Style for uk-UA',
    at.locale = 'uk-UA',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for uk-UA. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@uk-UA'})
MATCH (at:AudienceTrait {key: 'communication-style@uk-UA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// pl-PL AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'pl-PL'})
MATCH (as:AudienceSet {key: 'audience-set:general@pl-PL'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@pl-PL'})
SET at.display_name = 'Communication Style for pl-PL',
    at.locale = 'pl-PL',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for pl-PL. STYLE: balanced communication, medium hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@pl-PL'})
MATCH (at:AudienceTrait {key: 'communication-style@pl-PL'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// zh-SG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'zh-SG'})
MATCH (as:AudienceSet {key: 'audience-set:general@zh-SG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@zh-SG'})
SET at.display_name = 'Communication Style for zh-SG',
    at.locale = 'zh-SG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for zh-SG. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@zh-SG'})
MATCH (at:AudienceTrait {key: 'communication-style@zh-SG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-CA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-CA'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-CA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-CA'})
SET at.display_name = 'Communication Style for en-CA',
    at.locale = 'en-CA',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"balanced","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-CA. STYLE: balanced communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-CA'})
MATCH (at:AudienceTrait {key: 'communication-style@en-CA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-AE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-AE'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-AE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-AE'})
SET at.display_name = 'Communication Style for en-AE',
    at.locale = 'en-AE',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for en-AE. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-AE'})
MATCH (at:AudienceTrait {key: 'communication-style@en-AE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-CY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-CY'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-CY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-CY'})
SET at.display_name = 'Communication Style for en-CY',
    at.locale = 'en-CY',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-CY. STYLE: balanced communication, medium hierarchy importance, MEDIUM_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-CY'})
MATCH (at:AudienceTrait {key: 'communication-style@en-CY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ru-RU AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ru-RU'})
MATCH (as:AudienceSet {key: 'audience-set:general@ru-RU'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ru-RU'})
SET at.display_name = 'Communication Style for ru-RU',
    at.locale = 'ru-RU',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ru-RU. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ru-RU'})
MATCH (at:AudienceTrait {key: 'communication-style@ru-RU'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// mi-NZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'mi-NZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@mi-NZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@mi-NZ'})
SET at.display_name = 'Communication Style for mi-NZ',
    at.locale = 'mi-NZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for mi-NZ. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@mi-NZ'})
MATCH (at:AudienceTrait {key: 'communication-style@mi-NZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-PE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-PE'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-PE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-PE'})
SET at.display_name = 'Communication Style for es-PE',
    at.locale = 'es-PE',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-PE. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-PE'})
MATCH (at:AudienceTrait {key: 'communication-style@es-PE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// yo-NG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'yo-NG'})
MATCH (as:AudienceSet {key: 'audience-set:general@yo-NG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@yo-NG'})
SET at.display_name = 'Communication Style for yo-NG',
    at.locale = 'yo-NG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for yo-NG. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@yo-NG'})
MATCH (at:AudienceTrait {key: 'communication-style@yo-NG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// tk-TM AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'tk-TM'})
MATCH (as:AudienceSet {key: 'audience-set:general@tk-TM'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@tk-TM'})
SET at.display_name = 'Communication Style for tk-TM',
    at.locale = 'tk-TM',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for tk-TM. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@tk-TM'})
MATCH (at:AudienceTrait {key: 'communication-style@tk-TM'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-PY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-PY'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-PY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-PY'})
SET at.display_name = 'Communication Style for es-PY',
    at.locale = 'es-PY',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-PY. STYLE: indirect communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-PY'})
MATCH (at:AudienceTrait {key: 'communication-style@es-PY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ha-NG AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ha-NG'})
MATCH (as:AudienceSet {key: 'audience-set:general@ha-NG'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ha-NG'})
SET at.display_name = 'Communication Style for ha-NG',
    at.locale = 'ha-NG',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ha-NG. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ha-NG'})
MATCH (at:AudienceTrait {key: 'communication-style@ha-NG'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// el-CY AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'el-CY'})
MATCH (as:AudienceSet {key: 'audience-set:general@el-CY'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@el-CY'})
SET at.display_name = 'Communication Style for el-CY',
    at.locale = 'el-CY',
    at.segment = 'general',
    at.behavior = '{"context_type":"MEDIUM_HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for el-CY. STYLE: balanced communication, medium hierarchy importance, MEDIUM_HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@el-CY'})
MATCH (at:AudienceTrait {key: 'communication-style@el-CY'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// jv-ID AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'jv-ID'})
MATCH (as:AudienceSet {key: 'audience-set:general@jv-ID'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@jv-ID'})
SET at.display_name = 'Communication Style for jv-ID',
    at.locale = 'jv-ID',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for jv-ID. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@jv-ID'})
MATCH (at:AudienceTrait {key: 'communication-style@jv-ID'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// km-KH AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'km-KH'})
MATCH (as:AudienceSet {key: 'audience-set:general@km-KH'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@km-KH'})
SET at.display_name = 'Communication Style for km-KH',
    at.locale = 'km-KH',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for km-KH. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@km-KH'})
MATCH (at:AudienceTrait {key: 'communication-style@km-KH'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// so-SO AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'so-SO'})
MATCH (as:AudienceSet {key: 'audience-set:general@so-SO'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@so-SO'})
SET at.display_name = 'Communication Style for so-SO',
    at.locale = 'so-SO',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for so-SO. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@so-SO'})
MATCH (at:AudienceTrait {key: 'communication-style@so-SO'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-AE AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-AE'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-AE'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-AE'})
SET at.display_name = 'Communication Style for ar-AE',
    at.locale = 'ar-AE',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-AE. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-AE'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-AE'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// en-NZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'en-NZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@en-NZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@en-NZ'})
SET at.display_name = 'Communication Style for en-NZ',
    at.locale = 'en-NZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"LOW_CONTEXT","directness":"direct","hierarchy_sensitivity":"low"}',
    at.preferences = '{"formality":"informal","communication_style":"direct"}',
    at.content_length = 'detailed',
    at.llm_context = 'USE: when generating content for en-NZ. STYLE: direct communication, low hierarchy importance, LOW_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@en-NZ'})
MATCH (at:AudienceTrait {key: 'communication-style@en-NZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ne-NP AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ne-NP'})
MATCH (as:AudienceSet {key: 'audience-set:general@ne-NP'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ne-NP'})
SET at.display_name = 'Communication Style for ne-NP',
    at.locale = 'ne-NP',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ne-NP. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ne-NP'})
MATCH (at:AudienceTrait {key: 'communication-style@ne-NP'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// fr-DZ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-DZ'})
MATCH (as:AudienceSet {key: 'audience-set:general@fr-DZ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@fr-DZ'})
SET at.display_name = 'Communication Style for fr-DZ',
    at.locale = 'fr-DZ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for fr-DZ. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@fr-DZ'})
MATCH (at:AudienceTrait {key: 'communication-style@fr-DZ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// ar-LB AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ar-LB'})
MATCH (as:AudienceSet {key: 'audience-set:general@ar-LB'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@ar-LB'})
SET at.display_name = 'Communication Style for ar-LB',
    at.locale = 'ar-LB',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for ar-LB. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@ar-LB'})
MATCH (at:AudienceTrait {key: 'communication-style@ar-LB'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// tg-TJ AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'tg-TJ'})
MATCH (as:AudienceSet {key: 'audience-set:general@tg-TJ'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@tg-TJ'})
SET at.display_name = 'Communication Style for tg-TJ',
    at.locale = 'tg-TJ',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"indirect","hierarchy_sensitivity":"high"}',
    at.preferences = '{"formality":"formal","communication_style":"indirect"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for tg-TJ. STYLE: indirect communication, high hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@tg-TJ'})
MATCH (at:AudienceTrait {key: 'communication-style@tg-TJ'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// ----------------------------------------------------------------------------
// es-PA AudienceTrait
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'es-PA'})
MATCH (as:AudienceSet {key: 'audience-set:general@es-PA'})
MERGE (l)-[:HAS_AUDIENCE]->(as);

MERGE (at:AudienceTrait {key: 'communication-style@es-PA'})
SET at.display_name = 'Communication Style for es-PA',
    at.locale = 'es-PA',
    at.segment = 'general',
    at.behavior = '{"context_type":"HIGH_CONTEXT","directness":"balanced","hierarchy_sensitivity":"medium"}',
    at.preferences = '{"formality":"moderate","communication_style":"balanced"}',
    at.content_length = 'moderate',
    at.llm_context = 'USE: when generating content for es-PA. STYLE: balanced communication, medium hierarchy importance, HIGH_CONTEXT.',
    at.provenance = 'ath-know-l10n',
    at.confidence = 0.9,
    at.created_at = datetime(),
    at.updated_at = datetime();

MATCH (as:AudienceSet {key: 'audience-set:general@es-PA'})
MATCH (at:AudienceTrait {key: 'communication-style@es-PA'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);
