// Migration 106: Populate Adaptation and Style nodes for critical locales
// Purpose: SEO anti-cannibalization - each same-language locale MUST be distinct
// Reference: PLAN-locale-distinction-complete.md
// Date: 2026-03-10

// ============================================================================
// FRENCH LOCALES (fr-FR, fr-BE, fr-CH, fr-CA) - MUST BE DISTINCT
// ============================================================================

// --- fr-FR (France) ---
MERGE (a:Adaptation {key: 'fr-FR'})
SET a.display_name = 'French (France) Adaptation',
    a.content = 'Content adaptation rules for France French - prioritizes linguistic elegance and precision',
    a.llm_context = 'USE: when generating content for France audience. TRIGGERS: fr-FR, France, French metropolitan. NOT: for Belgian/Swiss/Canadian French. RELATES: Locale fr-FR, Style fr-FR.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","statistics":"FACT","legal_terms":"FACT","idioms":"ILLUSTRATION","metaphors":"ILLUSTRATION","cultural_references":"ILLUSTRATION","sports_analogies":"ILLUSTRATION"}',
    a.adaptation_summary = 'France French prioritizes linguistic elegance and intellectual precision. Use refined vocabulary - avoid oversimplification. Prefer French terms (courriel over email) but accept common anglicisms (smartphone, Wi-Fi). Literary flair appreciated. Cultural references: art de vivre, patrimoine, exception culturelle. Problem-solution rhetorical style with long-form tolerance.',
    a.decision_algorithm = '1. Proper noun? -> FACT (preserve)\n2. Technical spec? -> FACT (keep original)\n3. Idiom/metaphor? -> ILLUSTRATION (use refined French equivalent)\n4. Cultural reference? -> ILLUSTRATION (adapt to French patrimoine)\n5. Default: Favor elegant French expression',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'fr-FR'})
SET s.display_name = 'French (France) Style',
    s.content = 'Communication style for France - formal, indirect, intellectual',
    s.llm_context = 'USE: when setting tone for France audience. TRIGGERS: fr-FR, France, formality. NOT: for Belgian warmth or Canadian casualness. RELATES: Locale fr-FR, Adaptation fr-FR.',
    s.formality_score = 75,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'high',
    s.individualism_level = 'individualist',
    s.warmth_level = 'neutral',
    s.directness_score = 45,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b_corporate":{"formality":"very_formal","notes":"Use Monsieur/Madame titles"},"b2c_standard":{"formality":"formal","notes":"Vous by default"},"b2c_youth":{"formality":"moderate","notes":"Tu acceptable for youth brands"}}',
    s.pronoun_preference = 'vous',
    s.humor_score = 40,
    s.humor_types = '{"wordplay":"encouraged","irony":"subtle_only","self_deprecating":"acceptable","slapstick":"avoid"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- fr-BE (Belgium) ---
MERGE (a:Adaptation {key: 'fr-BE'})
SET a.display_name = 'Belgian French Adaptation',
    a.content = 'Content adaptation rules for Belgian French - pragmatic, Germanic-influenced directness',
    a.llm_context = 'USE: when generating content for Belgium Francophone audience. TRIGGERS: fr-BE, Belgium, Wallonia, Brussels. NOT: for France or Swiss French. RELATES: Locale fr-BE, Style fr-BE.',
    a.technical_terms_approach = 'english_accepted',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","statistics":"FACT","numbers_70_90":"FACT_LOCAL","idioms":"ILLUSTRATION","cultural_references":"ILLUSTRATION"}',
    a.adaptation_summary = 'Belgian French is more direct and pragmatic than France French due to Germanic (Dutch/German) influence. Less language purism - anglicisms more accepted. CRITICAL: Use septante (70), nonante (90) but NOT huitante (Swiss only). Use GSM for mobile phone, not portable. Meal vocabulary: dejeuner/diner/souper NOT petit-dejeuner/dejeuner/diner. Consensus-driven, self-deprecating humor valued.',
    a.decision_algorithm = '1. Number 70? -> septante (NOT soixante-dix)\n2. Number 90? -> nonante (NOT quatre-vingt-dix)\n3. Mobile phone? -> GSM (NOT portable)\n4. Meal times? -> Belgian vocabulary (souper=dinner)\n5. Default: More direct than France, less rhetorical flourish',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'fr-BE'})
SET s.display_name = 'Belgian French Style',
    s.content = 'Communication style for Belgium - warm, pragmatic, consensus-driven',
    s.llm_context = 'USE: when setting tone for Belgium Francophone audience. TRIGGERS: fr-BE, Belgium, Brussels, Wallonia. NOT: for France formality. RELATES: Locale fr-BE, Adaptation fr-BE.',
    s.formality_score = 60,
    s.default_formality = 'mixed',
    s.formality_default = 'neutral',
    s.directness_level = 'balanced',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'mixed',
    s.warmth_level = 'warm',
    s.directness_score = 65,
    s.directness_style = 'balanced',
    s.context_matrix = '{"b2b":{"formality":"moderate","notes":"Flatter hierarchy than France"},"b2c":{"formality":"casual","notes":"Faster tu transition acceptable"},"eu_context":{"formality":"formal","notes":"International EU headquarters influence"}}',
    s.pronoun_preference = 'vous',
    s.humor_score = 60,
    s.humor_types = '{"self_deprecating":"highly_encouraged","irony":"acceptable","wordplay":"acceptable","belgitude":"embrace"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- fr-CH (Switzerland) ---
MERGE (a:Adaptation {key: 'fr-CH'})
SET a.display_name = 'Swiss French Adaptation',
    a.content = 'Content adaptation rules for Swiss French - precision-oriented, neutral, methodical',
    a.llm_context = 'USE: when generating content for Swiss Romandie audience. TRIGGERS: fr-CH, Switzerland, Romandie, Geneva, Lausanne. NOT: for France or Belgian French. RELATES: Locale fr-CH, Style fr-CH.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'low',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","statistics":"FACT","technical_specs":"FACT_PRECISE","idioms":"ILLUSTRATION","cultural_references":"ILLUSTRATION_NEUTRAL"}',
    a.adaptation_summary = 'Swiss French emphasizes precision, neutrality, and methodical communication. CRITICAL: Use septante (70), huitante (80), nonante (90) - all three differ from France. German-influenced directness with formal politeness. Value exactitude over rhetorical flourish. Financial discretion - avoid overt price/money discussions. Punctuality sacred. Consensus-seeking but more direct than France.',
    a.decision_algorithm = '1. Number 70? -> septante\n2. Number 80? -> huitante (UNIQUE to Swiss)\n3. Number 90? -> nonante\n4. Financial info? -> Handle with discretion\n5. Default: Precision and clarity over elegance',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'fr-CH'})
SET s.display_name = 'Swiss French Style',
    s.content = 'Communication style for Swiss Romandie - formal, direct, precision-focused',
    s.llm_context = 'USE: when setting tone for Swiss Romandie audience. TRIGGERS: fr-CH, Switzerland, Geneva, Lausanne. NOT: for France indirectness or Belgian warmth. RELATES: Locale fr-CH, Adaptation fr-CH.',
    s.formality_score = 72,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'direct',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'individualist',
    s.warmth_level = 'reserved',
    s.directness_score = 70,
    s.directness_style = 'direct',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Punctuality and precision expected"},"b2c":{"formality":"moderate","notes":"Quality over price messaging"},"banking":{"formality":"very_formal","notes":"Financial discretion paramount"}}',
    s.pronoun_preference = 'vous',
    s.humor_score = 25,
    s.humor_types = '{"wordplay":"rare","irony":"avoid","self_deprecating":"rare","understated":"acceptable"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- fr-CA (Canada/Quebec) ---
MERGE (a:Adaptation {key: 'fr-CA'})
SET a.display_name = 'Canadian French Adaptation',
    a.content = 'Content adaptation rules for Quebec French - North American casual, language-preservation conscious',
    a.llm_context = 'USE: when generating content for Quebec audience. TRIGGERS: fr-CA, Quebec, Canada French. NOT: for France or European French. RELATES: Locale fr-CA, Style fr-CA.',
    a.technical_terms_approach = 'local_only',
    a.illustration_density = 'high',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","statistics":"FACT","anglicisms":"ILLUSTRATION_REPLACE","idioms":"ILLUSTRATION","cultural_references":"ILLUSTRATION_QUEBEC"}',
    a.adaptation_summary = 'Quebec French is distinct from France French - NOT a dialect but a separate linguistic identity. CRITICAL: Use courriel NOT email, cellulaire NOT portable, fin de semaine NOT week-end. Language preservation is law (Bill 101). More casual than France, North American friendliness. Local references: St-Jean Baptiste, hockey, Quebec seasons. Fierté québécoise is core value.',
    a.decision_algorithm = '1. Email? -> courriel (NEVER email)\n2. Weekend? -> fin de semaine\n3. Mobile? -> cellulaire\n4. Cultural reference? -> Quebec-specific (hockey, festivals)\n5. Default: Quebec vocabulary, North American casualness',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'fr-CA'})
SET s.display_name = 'Canadian French Style',
    s.content = 'Communication style for Quebec - casual, warm, North American directness',
    s.llm_context = 'USE: when setting tone for Quebec audience. TRIGGERS: fr-CA, Quebec, Montreal. NOT: for France formality. RELATES: Locale fr-CA, Adaptation fr-CA.',
    s.formality_score = 45,
    s.default_formality = 'casual',
    s.formality_default = 'informal',
    s.directness_level = 'balanced',
    s.hierarchy_importance = 'low',
    s.individualism_level = 'mixed',
    s.warmth_level = 'warm',
    s.directness_score = 60,
    s.directness_style = 'balanced',
    s.context_matrix = '{"b2b":{"formality":"moderate","notes":"North American business style"},"b2c":{"formality":"casual","notes":"Tu acceptable earlier"},"government":{"formality":"formal","notes":"Bill 101 compliance"}}',
    s.pronoun_preference = 'tu',
    s.humor_score = 55,
    s.humor_types = '{"self_deprecating":"encouraged","wordplay":"acceptable","local_references":"encouraged","slapstick":"acceptable"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- fr-LU (Luxembourg) ---
MERGE (a:Adaptation {key: 'fr-LU'})
SET a.display_name = 'Luxembourg French Adaptation',
    a.content = 'Content adaptation rules for Luxembourg French - trilingual context, EU/banking focus',
    a.llm_context = 'USE: when generating content for Luxembourg Francophone audience. TRIGGERS: fr-LU, Luxembourg. NOT: for France or Belgian French. RELATES: Locale fr-LU, Style fr-LU.',
    a.technical_terms_approach = 'english_accepted',
    a.illustration_density = 'low',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","financial_terms":"FACT","legal_terms":"FACT","idioms":"ILLUSTRATION"}',
    a.adaptation_summary = 'Luxembourg French operates in trilingual context (French, German, Luxembourgish). Strong EU and banking sector influence. International workforce. More formal than Belgian French but less rigid than France. Financial services vocabulary important. Cross-border worker audience (France, Belgium, Germany). Multilingual code-switching accepted.',
    a.decision_algorithm = '1. Financial term? -> Standard international term acceptable\n2. EU context? -> Formal EU language\n3. Cross-border reference? -> Acknowledge multinational context\n4. Default: Belgian-style pragmatism with international polish',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'fr-LU'})
SET s.display_name = 'Luxembourg French Style',
    s.content = 'Communication style for Luxembourg - international, multilingual, banking-influenced',
    s.llm_context = 'USE: when setting tone for Luxembourg audience. TRIGGERS: fr-LU, Luxembourg, Grand Duchy. NOT: for purely Belgian or French context. RELATES: Locale fr-LU, Adaptation fr-LU.',
    s.formality_score = 68,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'direct',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'individualist',
    s.warmth_level = 'neutral',
    s.directness_score = 62,
    s.directness_style = 'balanced',
    s.context_matrix = '{"banking":{"formality":"very_formal","notes":"Financial discretion"},"eu_institutions":{"formality":"formal","notes":"International protocol"},"b2c":{"formality":"moderate","notes":"Cosmopolitan audience"}}',
    s.pronoun_preference = 'vous',
    s.humor_score = 35,
    s.humor_types = '{"wordplay":"acceptable","irony":"rare","self_deprecating":"acceptable"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// ============================================================================
// SPANISH LOCALES (es-ES, es-MX, es-AR, es-CO, es-CL) - MUST BE DISTINCT
// ============================================================================

// --- es-ES (Spain) ---
MERGE (a:Adaptation {key: 'es-ES'})
SET a.display_name = 'Spanish (Spain) Adaptation',
    a.content = 'Content adaptation rules for Spain Spanish - regional pride, vosotros form, Mediterranean warmth',
    a.llm_context = 'USE: when generating content for Spain audience. TRIGGERS: es-ES, Spain, Iberian. NOT: for Latin American Spanish. RELATES: Locale es-ES, Style es-ES.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'high',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","regional_references":"ILLUSTRATION","idioms":"ILLUSTRATION","cultural_references":"ILLUSTRATION_SPANISH"}',
    a.adaptation_summary = 'Spain Spanish uses VOSOTROS for plural you (Latin America uses ustedes). Regional pride is strong - avoid Madrid-centrism. Mediterranean social culture values relationships over efficiency. Tapas, sobremesa (post-meal conversation), siesta references resonate. Avoid Franco-era references. Use castellano vocabulary, not Latin American variants.',
    a.decision_algorithm = '1. Plural you? -> vosotros (NEVER ustedes for Spain)\n2. Cultural reference? -> Spanish-specific (tapas, siesta, feria)\n3. Regional mention? -> Acknowledge diversity (Cataluña, País Vasco)\n4. Default: Warm, social, relationship-focused',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'es-ES'})
SET s.display_name = 'Spanish (Spain) Style',
    s.content = 'Communication style for Spain - warm, indirect, relationship-focused',
    s.llm_context = 'USE: when setting tone for Spain audience. TRIGGERS: es-ES, Spain. NOT: for Latin American directness variations. RELATES: Locale es-ES, Adaptation es-ES.',
    s.formality_score = 55,
    s.default_formality = 'mixed',
    s.formality_default = 'neutral',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'individualist',
    s.warmth_level = 'warm',
    s.directness_score = 50,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"moderate","notes":"Relationship-first, then business"},"b2c":{"formality":"casual","notes":"Informal tú acceptable"},"regional":{"formality":"varies","notes":"Respect regional identities"}}',
    s.pronoun_preference = 'mixed',
    s.humor_score = 65,
    s.humor_types = '{"irony":"encouraged","wordplay":"acceptable","regional":"acceptable","self_deprecating":"acceptable"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- es-MX (Mexico) ---
MERGE (a:Adaptation {key: 'es-MX'})
SET a.display_name = 'Mexican Spanish Adaptation',
    a.content = 'Content adaptation rules for Mexican Spanish - respectful, family-focused, diminutives',
    a.llm_context = 'USE: when generating content for Mexico audience. TRIGGERS: es-MX, Mexico. NOT: for Spain or South American Spanish. RELATES: Locale es-MX, Style es-MX.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'high',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","family_references":"ILLUSTRATION","cultural_references":"ILLUSTRATION_MEXICAN"}',
    a.adaptation_summary = 'Mexican Spanish uses USTED as default with strangers (more formal than Spain). Diminutives show warmth (ahorita, tantito, poquito). Family is central to decisions. Avoid stereotypes (sombrero/cactus). Modern Mexico is sophisticated and tech-savvy. Respeto and cortesía paramount. Pre-Hispanic heritage pride. Indirect communication to preserve harmony.',
    a.decision_algorithm = '1. Addressing stranger? -> usted (formal)\n2. Showing warmth? -> Use diminutives (-ito, -ita)\n3. Family context? -> Highlight collective benefit\n4. Cultural reference? -> Modern Mexico (avoid stereotypes)\n5. Default: Warm, respectful, indirect',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'es-MX'})
SET s.display_name = 'Mexican Spanish Style',
    s.content = 'Communication style for Mexico - formal, warm, family-oriented',
    s.llm_context = 'USE: when setting tone for Mexico audience. TRIGGERS: es-MX, Mexico. NOT: for Spain vosotros or Argentine voseo. RELATES: Locale es-MX, Adaptation es-MX.',
    s.formality_score = 68,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'high',
    s.individualism_level = 'collectivist',
    s.warmth_level = 'warm',
    s.directness_score = 40,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Usted default, respect hierarchy"},"b2c":{"formality":"moderate","notes":"Warm but respectful"},"family":{"formality":"informal","notes":"Collective decision-making"}}',
    s.pronoun_preference = 'usted',
    s.humor_score = 50,
    s.humor_types = '{"wordplay":"acceptable","self_deprecating":"common","dark_humor":"avoid","family_friendly":"preferred"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- es-AR (Argentina) ---
MERGE (a:Adaptation {key: 'es-AR'})
SET a.display_name = 'Argentine Spanish Adaptation',
    a.content = 'Content adaptation rules for Argentine Spanish - voseo, passionate, European-influenced',
    a.llm_context = 'USE: when generating content for Argentina audience. TRIGGERS: es-AR, Argentina, Buenos Aires. NOT: for other Latin American or Spain Spanish. RELATES: Locale es-AR, Style es-AR.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","malvinas_reference":"FACT_SENSITIVE","cultural_references":"ILLUSTRATION_ARGENTINE"}',
    a.adaptation_summary = 'Argentine Spanish uses VOSEO (vos instead of tú) with unique conjugations (vos tenés, vos podés). Highly expressive and passionate communication. European heritage pride (Italian/Spanish immigration). Debate culture valued. Avoid Malvinas/Falklands topic. Porteño sophistication. Picardía criolla (street-smart cleverness) appreciated. Economic crisis references sensitive.',
    a.decision_algorithm = '1. Second person? -> VOS (vos tenés, NOT tú tienes)\n2. Imperative? -> Voseo form (probá, mirá, vení)\n3. Emotional expression? -> Embrace passion\n4. Malvinas reference? -> AVOID completely\n5. Default: Direct, passionate, sophisticated',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'es-AR'})
SET s.display_name = 'Argentine Spanish Style',
    s.content = 'Communication style for Argentina - direct, passionate, debate-friendly',
    s.llm_context = 'USE: when setting tone for Argentina audience. TRIGGERS: es-AR, Argentina, Buenos Aires. NOT: for other LATAM formality norms. RELATES: Locale es-AR, Adaptation es-AR.',
    s.formality_score = 40,
    s.default_formality = 'casual',
    s.formality_default = 'informal',
    s.directness_level = 'direct',
    s.hierarchy_importance = 'low',
    s.individualism_level = 'individualist',
    s.warmth_level = 'warm',
    s.directness_score = 75,
    s.directness_style = 'direct',
    s.context_matrix = '{"b2b":{"formality":"moderate","notes":"Still relationship-focused but more direct"},"b2c":{"formality":"casual","notes":"Voseo expected"},"debate":{"formality":"informal","notes":"Intellectual sparring valued"}}',
    s.pronoun_preference = 'mixed',
    s.humor_score = 70,
    s.humor_types = '{"irony":"highly_encouraged","self_deprecating":"valued","dark_humor":"acceptable","political":"acceptable_carefully"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- es-CO (Colombia) ---
MERGE (a:Adaptation {key: 'es-CO'})
SET a.display_name = 'Colombian Spanish Adaptation',
    a.content = 'Content adaptation rules for Colombian Spanish - exceptionally polite, warm, entrepreneurial',
    a.llm_context = 'USE: when generating content for Colombia audience. TRIGGERS: es-CO, Colombia. NOT: for other Latin American Spanish. RELATES: Locale es-CO, Style es-CO.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'high',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","regional_references":"ILLUSTRATION","transformation_narrative":"ILLUSTRATION_POSITIVE"}',
    a.adaptation_summary = 'Colombian Spanish is exceptionally polite - USTED used even among friends in some regions. Strong entrepreneurial spirit (pujante). Regional diversity (Paisa, Costeño, Cachaco, Caleño). CRITICAL: Avoid any drug/cartel/violence references - deeply offensive. Colombia transformation narrative is source of pride. Future-focused, innovation-ready positioning works well.',
    a.decision_algorithm = '1. Addressing anyone? -> usted (extremely common)\n2. Regional mention? -> Acknowledge diversity\n3. Historical reference? -> Future-focused preferred\n4. Violence/drug reference? -> ABSOLUTE TABOO\n5. Default: Exceptionally warm and polite',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'es-CO'})
SET s.display_name = 'Colombian Spanish Style',
    s.content = 'Communication style for Colombia - highly formal, exceptionally warm, future-focused',
    s.llm_context = 'USE: when setting tone for Colombia audience. TRIGGERS: es-CO, Colombia. NOT: for Argentine informality. RELATES: Locale es-CO, Adaptation es-CO.',
    s.formality_score = 72,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'high',
    s.individualism_level = 'collectivist',
    s.warmth_level = 'warm',
    s.directness_score = 35,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Usted even with peers"},"b2c":{"formality":"moderate","notes":"Exceptional warmth"},"entrepreneurial":{"formality":"moderate","notes":"Innovation-friendly"}}',
    s.pronoun_preference = 'usted',
    s.humor_score = 55,
    s.humor_types = '{"warmth":"valued","self_deprecating":"acceptable","regional":"acceptable","dark":"avoid"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- es-CL (Chile) ---
MERGE (a:Adaptation {key: 'es-CL'})
SET a.display_name = 'Chilean Spanish Adaptation',
    a.content = 'Content adaptation rules for Chilean Spanish - pragmatic, tech-forward, unique modismos',
    a.llm_context = 'USE: when generating content for Chile audience. TRIGGERS: es-CL, Chile. NOT: for other Latin American Spanish. RELATES: Locale es-CL, Style es-CL.',
    a.technical_terms_approach = 'english_accepted',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","chilean_modismos":"ILLUSTRATION","cultural_references":"ILLUSTRATION_CHILEAN"}',
    a.adaptation_summary = 'Chilean Spanish has unique modismos (cachai, po, al tiro, fome). LATAM tech hub (Start-Up Chile) - innovation culture. More pragmatic and business-oriented than other LATAM. Results-focused. Mix of tú/usted. Chilenidad - strong distinct identity from rest of LATAM. Avoid Pinochet era politics. Aspirational middle class audience.',
    a.decision_algorithm = '1. Urgency? -> al tiro (right away)\n2. Understanding check? -> cachai?\n3. Emphasis? -> Add po (pues)\n4. Tech context? -> English terms acceptable\n5. Default: Pragmatic, efficient, results-focused',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'es-CL'})
SET s.display_name = 'Chilean Spanish Style',
    s.content = 'Communication style for Chile - pragmatic, tech-savvy, results-oriented',
    s.llm_context = 'USE: when setting tone for Chile audience. TRIGGERS: es-CL, Chile. NOT: for Colombian formality or Argentine passion. RELATES: Locale es-CL, Adaptation es-CL.',
    s.formality_score = 52,
    s.default_formality = 'mixed',
    s.formality_default = 'neutral',
    s.directness_level = 'balanced',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'mixed',
    s.warmth_level = 'neutral',
    s.directness_score = 60,
    s.directness_style = 'balanced',
    s.context_matrix = '{"b2b":{"formality":"moderate","notes":"Results-focused"},"b2c":{"formality":"casual","notes":"Tech-savvy audience"},"startup":{"formality":"casual","notes":"Innovation culture"}}',
    s.pronoun_preference = 'mixed',
    s.humor_score = 50,
    s.humor_types = '{"irony":"acceptable","self_deprecating":"common","wordplay":"acceptable","political":"careful"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// ============================================================================
// PORTUGUESE LOCALES (pt-BR, pt-PT) - MUST BE DISTINCT
// ============================================================================

// --- pt-BR (Brazil) ---
MERGE (a:Adaptation {key: 'pt-BR'})
SET a.display_name = 'Brazilian Portuguese Adaptation',
    a.content = 'Content adaptation rules for Brazilian Portuguese - warm, optimistic, creative problem-solving',
    a.llm_context = 'USE: when generating content for Brazil audience. TRIGGERS: pt-BR, Brazil. NOT: for Portugal Portuguese. RELATES: Locale pt-BR, Style pt-BR.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'high',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","regional_references":"ILLUSTRATION","cultural_references":"ILLUSTRATION_BRAZILIAN"}',
    a.adaptation_summary = 'Brazilian Portuguese uses VOCE (not tu in most regions). Jeitinho brasileiro - creative problem-solving. Calor humano (warmth) and otimismo resiliente are core values. Continental diversity - avoid Rio/SP only focus. Relationship-first culture. CRITICAL: NOT Portugal Portuguese - different grammar, vocabulary, spelling. Political polarization sensitive - stay neutral.',
    a.decision_algorithm = '1. Second person? -> você (not tu)\n2. Gerund usage? -> Brazilian style (estou fazendo)\n3. Regional reference? -> Include Northeast, South diversity\n4. Portugal term? -> Use Brazilian equivalent\n5. Default: Warm, optimistic, relationship-first',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'pt-BR'})
SET s.display_name = 'Brazilian Portuguese Style',
    s.content = 'Communication style for Brazil - warm, optimistic, relationship-focused',
    s.llm_context = 'USE: when setting tone for Brazil audience. TRIGGERS: pt-BR, Brazil. NOT: for Portugal formality. RELATES: Locale pt-BR, Adaptation pt-BR.',
    s.formality_score = 42,
    s.default_formality = 'casual',
    s.formality_default = 'informal',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'high',
    s.individualism_level = 'collectivist',
    s.warmth_level = 'warm',
    s.directness_score = 45,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"moderate","notes":"Relationship before business"},"b2c":{"formality":"casual","notes":"Warmth expected"},"carnival":{"formality":"very_casual","notes":"Celebration culture"}}',
    s.pronoun_preference = 'mixed',
    s.humor_score = 70,
    s.humor_types = '{"self_deprecating":"common","wordplay":"valued","warmth":"essential","irony":"acceptable"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- pt-PT (Portugal) ---
MERGE (a:Adaptation {key: 'pt-PT'})
SET a.display_name = 'European Portuguese Adaptation',
    a.content = 'Content adaptation rules for Portugal Portuguese - modest, saudade-inflected, resourceful',
    a.llm_context = 'USE: when generating content for Portugal audience. TRIGGERS: pt-PT, Portugal. NOT: for Brazilian Portuguese. RELATES: Locale pt-PT, Style pt-PT.',
    a.technical_terms_approach = 'local_only',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","maritime_references":"ILLUSTRATION","cultural_references":"ILLUSTRATION_PORTUGUESE"}',
    a.adaptation_summary = 'European Portuguese uses TU (not você which sounds Brazilian). Saudade - bittersweet nostalgia is cultural concept. Desenrascanço - resourceful improvisation. Modest self-presentation. Maritime heritage pride. CRITICAL: NOT Brazilian Portuguese - different spelling (facto not fato), grammar (a fazer not fazendo), vocabulary. Avoid American-style hype.',
    a.decision_algorithm = '1. Second person? -> tu (você sounds Brazilian)\n2. Infinitive vs gerund? -> a fazer (NOT fazendo)\n3. Spelling? -> European (facto, acção)\n4. Cultural reference? -> Portuguese heritage\n5. Default: Modest, precise, resourceful',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'pt-PT'})
SET s.display_name = 'European Portuguese Style',
    s.content = 'Communication style for Portugal - formal, modest, heritage-conscious',
    s.llm_context = 'USE: when setting tone for Portugal audience. TRIGGERS: pt-PT, Portugal. NOT: for Brazilian warmth. RELATES: Locale pt-PT, Adaptation pt-PT.',
    s.formality_score = 62,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'balanced',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'mixed',
    s.warmth_level = 'neutral',
    s.directness_score = 55,
    s.directness_style = 'balanced',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"European business norms"},"b2c":{"formality":"moderate","notes":"Modest claims preferred"},"heritage":{"formality":"formal","notes":"Respect tradition"}}',
    s.pronoun_preference = 'tu',
    s.humor_score = 40,
    s.humor_types = '{"irony":"acceptable","self_deprecating":"valued","understated":"preferred","slapstick":"rare"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// ============================================================================
// ENGLISH LOCALES (en-US, en-GB, en-AU) - MUST BE DISTINCT
// ============================================================================

// --- en-US (United States) ---
MERGE (a:Adaptation {key: 'en-US'})
SET a.display_name = 'American English Adaptation',
    a.content = 'Content adaptation rules for American English - direct, optimistic, action-oriented',
    a.llm_context = 'USE: when generating content for US audience. TRIGGERS: en-US, United States, American. NOT: for British or Australian English. RELATES: Locale en-US, Style en-US.',
    a.technical_terms_approach = 'english_accepted',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","sports_references":"ILLUSTRATION_US","cultural_references":"ILLUSTRATION_AMERICAN"}',
    a.adaptation_summary = 'American English is direct, optimistic, and action-oriented. Can-do attitude. Spelling: color, organize, center. Sports references: baseball, football (American), basketball. Dream big messaging works. Regional diversity matters (NYC vs Texas vs California). Individualism valued. Quick to first-name basis. Results and ROI-focused.',
    a.decision_algorithm = '1. Spelling? -> American (color, organize, center)\n2. Sports reference? -> American sports\n3. Measurement? -> Imperial (feet, miles, Fahrenheit)\n4. Tone? -> Optimistic, action-oriented\n5. Default: Direct, results-focused',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'en-US'})
SET s.display_name = 'American English Style',
    s.content = 'Communication style for US - direct, optimistic, action-oriented',
    s.llm_context = 'USE: when setting tone for US audience. TRIGGERS: en-US, United States. NOT: for British understatement. RELATES: Locale en-US, Adaptation en-US.',
    s.formality_score = 35,
    s.default_formality = 'casual',
    s.formality_default = 'informal',
    s.directness_level = 'direct',
    s.hierarchy_importance = 'low',
    s.individualism_level = 'individualist',
    s.warmth_level = 'warm',
    s.directness_score = 80,
    s.directness_style = 'direct',
    s.context_matrix = '{"b2b":{"formality":"moderate","notes":"Results-focused, first names common"},"b2c":{"formality":"casual","notes":"Friendly, approachable"},"startup":{"formality":"very_casual","notes":"Move fast culture"}}',
    s.pronoun_preference = 'n_a',
    s.humor_score = 65,
    s.humor_types = '{"self_deprecating":"acceptable","optimistic":"preferred","sarcasm":"careful","wordplay":"acceptable"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- en-GB (United Kingdom) ---
MERGE (a:Adaptation {key: 'en-GB'})
SET a.display_name = 'British English Adaptation',
    a.content = 'Content adaptation rules for British English - understated, class-conscious, tradition-aware',
    a.llm_context = 'USE: when generating content for UK audience. TRIGGERS: en-GB, United Kingdom, British. NOT: for American or Australian English. RELATES: Locale en-GB, Style en-GB.',
    a.technical_terms_approach = 'english_accepted',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","sports_references":"ILLUSTRATION_UK","cultural_references":"ILLUSTRATION_BRITISH"}',
    a.adaptation_summary = 'British English values understatement and indirectness. Spelling: colour, organise, centre. Irony and dry humor appreciated. Class consciousness still relevant. Sports: football (soccer), cricket, rugby. Regional diversity: England, Scotland, Wales, Northern Ireland. Queue culture. Apologizing as politeness. Avoid American hyperbole.',
    a.decision_algorithm = '1. Spelling? -> British (colour, organise, centre)\n2. Sports reference? -> UK sports (football=soccer)\n3. Measurement? -> Metric with miles\n4. Enthusiasm? -> Understate (quite good = excellent)\n5. Default: Understated, polite, ironic',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'en-GB'})
SET s.display_name = 'British English Style',
    s.content = 'Communication style for UK - understated, indirect, tradition-aware',
    s.llm_context = 'USE: when setting tone for UK audience. TRIGGERS: en-GB, United Kingdom, British. NOT: for American directness. RELATES: Locale en-GB, Adaptation en-GB.',
    s.formality_score = 58,
    s.default_formality = 'mixed',
    s.formality_default = 'neutral',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'individualist',
    s.warmth_level = 'reserved',
    s.directness_score = 45,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"moderate","notes":"Class-aware, understated"},"b2c":{"formality":"moderate","notes":"Polite but not stiff"},"luxury":{"formality":"formal","notes":"Heritage and tradition"}}',
    s.pronoun_preference = 'n_a',
    s.humor_score = 70,
    s.humor_types = '{"irony":"essential","self_deprecating":"valued","dry":"preferred","slapstick":"acceptable"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- en-AU (Australia) ---
MERGE (a:Adaptation {key: 'en-AU'})
SET a.display_name = 'Australian English Adaptation',
    a.content = 'Content adaptation rules for Australian English - egalitarian, laid-back, anti-pretension',
    a.llm_context = 'USE: when generating content for Australia audience. TRIGGERS: en-AU, Australia, Australian. NOT: for British or American English. RELATES: Locale en-AU, Style en-AU.',
    a.technical_terms_approach = 'english_accepted',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","aussie_slang":"ILLUSTRATION","cultural_references":"ILLUSTRATION_AUSTRALIAN"}',
    a.adaptation_summary = 'Australian English is egalitarian - tall poppy syndrome (dislike of showing off). Laid-back, casual tone. British spelling but unique vocabulary (arvo, servo, barbie). Anti-pretension - authenticity valued. Mate culture. Outdoor lifestyle references. Indigenous acknowledgment increasingly important. Self-deprecating humor essential. No worries attitude.',
    a.decision_algorithm = '1. Spelling? -> British (colour, centre)\n2. Vocabulary? -> Australian (arvo, servo)\n3. Formality? -> Casual by default\n4. Boasting? -> AVOID (tall poppy syndrome)\n5. Default: Laid-back, authentic, mate-focused',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'en-AU'})
SET s.display_name = 'Australian English Style',
    s.content = 'Communication style for Australia - casual, egalitarian, authentic',
    s.llm_context = 'USE: when setting tone for Australia audience. TRIGGERS: en-AU, Australia. NOT: for British formality or American enthusiasm. RELATES: Locale en-AU, Adaptation en-AU.',
    s.formality_score = 30,
    s.default_formality = 'casual',
    s.formality_default = 'informal',
    s.directness_level = 'direct',
    s.hierarchy_importance = 'low',
    s.individualism_level = 'mixed',
    s.warmth_level = 'warm',
    s.directness_score = 70,
    s.directness_style = 'direct',
    s.context_matrix = '{"b2b":{"formality":"casual","notes":"Egalitarian, first names"},"b2c":{"formality":"very_casual","notes":"Mate culture"},"corporate":{"formality":"moderate","notes":"Still informal by global standards"}}',
    s.pronoun_preference = 'n_a',
    s.humor_score = 75,
    s.humor_types = '{"self_deprecating":"essential","irony":"valued","taking_the_piss":"expected","pretension":"mock"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// ============================================================================
// GERMAN LOCALES (de-DE, de-CH, de-AT) - MUST BE DISTINCT
// ============================================================================

// --- de-DE (Germany) ---
MERGE (a:Adaptation {key: 'de-DE'})
SET a.display_name = 'German (Germany) Adaptation',
    a.content = 'Content adaptation rules for Germany German - direct, precise, quality-focused',
    a.llm_context = 'USE: when generating content for Germany audience. TRIGGERS: de-DE, Germany, German. NOT: for Swiss or Austrian German. RELATES: Locale de-DE, Style de-DE.',
    a.technical_terms_approach = 'local_only',
    a.illustration_density = 'low',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","technical_specs":"FACT_PRECISE","cultural_references":"ILLUSTRATION_GERMAN"}',
    a.adaptation_summary = 'German German prioritizes directness and precision. Quality and reliability valued over price. Technical excellence appreciated. Umwelt (environmental) consciousness. Ordnung (order) important. Sie form default in business. Direct communication is respectful, not rude. Avoid Nazi-era references. Engineering excellence and Made in Germany pride.',
    a.decision_algorithm = '1. Technical term? -> German equivalent preferred\n2. Quality claim? -> Back with specifications\n3. Environmental angle? -> Emphasize sustainability\n4. Addressing someone? -> Sie (formal)\n5. Default: Direct, precise, quality-focused',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'de-DE'})
SET s.display_name = 'German (Germany) Style',
    s.content = 'Communication style for Germany - direct, precise, formal',
    s.llm_context = 'USE: when setting tone for Germany audience. TRIGGERS: de-DE, Germany. NOT: for Swiss neutrality or Austrian warmth. RELATES: Locale de-DE, Adaptation de-DE.',
    s.formality_score = 75,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'direct',
    s.hierarchy_importance = 'high',
    s.individualism_level = 'individualist',
    s.warmth_level = 'reserved',
    s.directness_score = 85,
    s.directness_style = 'direct',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Sie default, titles important"},"b2c":{"formality":"moderate","notes":"Still formal by international standards"},"tech":{"formality":"moderate","notes":"Precision valued"}}',
    s.pronoun_preference = 'sie',
    s.humor_score = 35,
    s.humor_types = '{"wordplay":"acceptable","irony":"careful","self_deprecating":"rare","slapstick":"acceptable"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- de-CH (Switzerland) ---
MERGE (a:Adaptation {key: 'de-CH'})
SET a.display_name = 'Swiss German Adaptation',
    a.content = 'Content adaptation rules for Swiss German - precise, neutral, consensus-driven',
    a.llm_context = 'USE: when generating content for German-speaking Switzerland. TRIGGERS: de-CH, Switzerland, Swiss German. NOT: for Germany or Austrian German. RELATES: Locale de-CH, Style de-CH.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'low',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","financial_terms":"FACT_DISCREET","cultural_references":"ILLUSTRATION_SWISS"}',
    a.adaptation_summary = 'Swiss German has unique vocabulary and written standard (Helvetisms). Grüezi as greeting (not Guten Tag). Velo (bike), Trottoir (sidewalk), Billett (ticket). Neutrality valued. Financial discretion paramount. Consensus-seeking (not German directness). Punctuality sacred. Quality over price. Cantonal identity strong.',
    a.decision_algorithm = '1. Greeting? -> Grüezi (Swiss)\n2. Vocabulary? -> Swiss Helvetisms\n3. Financial info? -> Handle with discretion\n4. Directness? -> Softer than Germany\n5. Default: Precise, neutral, consensus-seeking',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'de-CH'})
SET s.display_name = 'Swiss German Style',
    s.content = 'Communication style for German Switzerland - formal, precise, consensus-driven',
    s.llm_context = 'USE: when setting tone for Swiss German audience. TRIGGERS: de-CH, Switzerland. NOT: for German directness. RELATES: Locale de-CH, Adaptation de-CH.',
    s.formality_score = 72,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'balanced',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'individualist',
    s.warmth_level = 'reserved',
    s.directness_score = 65,
    s.directness_style = 'balanced',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Punctuality paramount"},"b2c":{"formality":"moderate","notes":"Quality messaging"},"banking":{"formality":"very_formal","notes":"Discretion essential"}}',
    s.pronoun_preference = 'sie',
    s.humor_score = 25,
    s.humor_types = '{"understatement":"preferred","wordplay":"acceptable","irony":"careful","exaggeration":"avoid"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- de-AT (Austria) ---
MERGE (a:Adaptation {key: 'de-AT'})
SET a.display_name = 'Austrian German Adaptation',
    a.content = 'Content adaptation rules for Austrian German - formal, traditional, gemütlich',
    a.llm_context = 'USE: when generating content for Austria audience. TRIGGERS: de-AT, Austria, Austrian. NOT: for Germany or Swiss German. RELATES: Locale de-AT, Style de-AT.',
    a.technical_terms_approach = 'local_only',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","austrian_terms":"ILLUSTRATION","cultural_references":"ILLUSTRATION_AUSTRIAN"}',
    a.adaptation_summary = 'Austrian German has unique vocabulary (Paradeiser not Tomate, Erdäpfel not Kartoffel, Jänner not Januar). Gemütlichkeit (coziness, comfort) valued. More formal titles than Germany (Herr Doktor, Herr Magister). Habsburg heritage pride. Kaffeehaus culture. Music and arts appreciation. Warmer than German directness. Avoid confusing with Germany.',
    a.decision_algorithm = '1. Vocabulary? -> Austrian variants (Paradeiser, Erdäpfel)\n2. Month names? -> Austrian (Jänner, Feber)\n3. Titles? -> Use academic titles\n4. Tone? -> Warmer than Germany\n5. Default: Formal, traditional, gemütlich',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'de-AT'})
SET s.display_name = 'Austrian German Style',
    s.content = 'Communication style for Austria - formal, warm, tradition-conscious',
    s.llm_context = 'USE: when setting tone for Austria audience. TRIGGERS: de-AT, Austria. NOT: for German directness or Swiss neutrality. RELATES: Locale de-AT, Adaptation de-AT.',
    s.formality_score = 70,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'high',
    s.individualism_level = 'mixed',
    s.warmth_level = 'warm',
    s.directness_score = 55,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Titles very important"},"b2c":{"formality":"moderate","notes":"Gemütlich warmth"},"culture":{"formality":"formal","notes":"Arts and heritage appreciation"}}',
    s.pronoun_preference = 'sie',
    s.humor_score = 50,
    s.humor_types = '{"irony":"acceptable","self_deprecating":"common","wiener_schmäh":"valued","wordplay":"appreciated"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// ============================================================================
// ITALIAN LOCALES (it-IT, it-CH) - MUST BE DISTINCT
// ============================================================================

// --- it-IT (Italy) ---
MERGE (a:Adaptation {key: 'it-IT'})
SET a.display_name = 'Italian (Italy) Adaptation',
    a.content = 'Content adaptation rules for Italy Italian - bella figura, relationship-focused, regional pride',
    a.llm_context = 'USE: when generating content for Italy audience. TRIGGERS: it-IT, Italy, Italian. NOT: for Swiss Italian. RELATES: Locale it-IT, Style it-IT.',
    a.technical_terms_approach = 'local_only',
    a.illustration_density = 'high',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","regional_references":"ILLUSTRATION","cultural_references":"ILLUSTRATION_ITALIAN"}',
    a.adaptation_summary = 'Italian Italian values bella figura (presenting oneself well). Aesthetics matter in everything. Regional identity strong (campanilismo) - avoid North-South stereotypes. Lei form default in business. Family and relationships central. Made in Italy quality pride. Food and design references resonate. Flexible timing. Avoid mafia references (deeply offensive).',
    a.decision_algorithm = '1. Addressing someone? -> Lei (formal)\n2. Regional mention? -> Respect campanilismo\n3. Quality claim? -> Artigianale (artisan quality)\n4. Aesthetics? -> Visual excellence matters\n5. Default: Relationship-first, bella figura',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'it-IT'})
SET s.display_name = 'Italian (Italy) Style',
    s.content = 'Communication style for Italy - formal, aesthetic, relationship-focused',
    s.llm_context = 'USE: when setting tone for Italy audience. TRIGGERS: it-IT, Italy. NOT: for Swiss Italian reserve. RELATES: Locale it-IT, Adaptation it-IT.',
    s.formality_score = 65,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'high',
    s.individualism_level = 'mixed',
    s.warmth_level = 'warm',
    s.directness_score = 40,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Relationship before transaction"},"b2c":{"formality":"moderate","notes":"Bella figura matters"},"luxury":{"formality":"very_formal","notes":"Made in Italy excellence"}}',
    s.pronoun_preference = 'mixed',
    s.humor_score = 55,
    s.humor_types = '{"irony":"acceptable","wordplay":"valued","regional":"careful","self_deprecating":"acceptable"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- it-CH (Swiss Italian/Ticino) ---
MERGE (a:Adaptation {key: 'it-CH'})
SET a.display_name = 'Swiss Italian Adaptation',
    a.content = 'Content adaptation rules for Ticino Italian - Swiss precision meets Italian warmth',
    a.llm_context = 'USE: when generating content for Ticino/Swiss Italian audience. TRIGGERS: it-CH, Ticino, Swiss Italian. NOT: for Italy Italian. RELATES: Locale it-CH, Style it-CH.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'low',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","swiss_context":"ILLUSTRATION","cultural_references":"ILLUSTRATION_TICINESE"}',
    a.adaptation_summary = 'Swiss Italian (Ticino) combines Italian warmth with Swiss precision. Ticinesi are SWISS, not expat Italians. Punctuality expected (unlike Italy). Financial discretion (Swiss banking culture). More reserved than Italians. Alpine lifestyle. Data privacy emphasis. Lei form even more consistent. Avoid treating as extension of Italy.',
    a.decision_algorithm = '1. Punctuality? -> Swiss standards apply\n2. Financial info? -> Discretion paramount\n3. Ticinese identity? -> Emphasize Swiss, not Italian\n4. Formality? -> Consistent Lei usage\n5. Default: Italian warmth + Swiss precision',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'it-CH'})
SET s.display_name = 'Swiss Italian Style',
    s.content = 'Communication style for Ticino - formal, precise, reserved warmth',
    s.llm_context = 'USE: when setting tone for Ticino audience. TRIGGERS: it-CH, Ticino. NOT: for Italian expressiveness. RELATES: Locale it-CH, Adaptation it-CH.',
    s.formality_score = 70,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'balanced',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'individualist',
    s.warmth_level = 'neutral',
    s.directness_score = 60,
    s.directness_style = 'balanced',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Swiss business norms"},"b2c":{"formality":"moderate","notes":"Reserved warmth"},"banking":{"formality":"very_formal","notes":"Privacy essential"}}',
    s.pronoun_preference = 'mixed',
    s.humor_score = 35,
    s.humor_types = '{"understatement":"preferred","irony":"careful","warmth":"subdued","exaggeration":"avoid"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// ============================================================================
// ARABIC LOCALES (ar-SA, ar-EG) - MUST BE DISTINCT
// ============================================================================

// --- ar-SA (Saudi Arabia) ---
MERGE (a:Adaptation {key: 'ar-SA'})
SET a.display_name = 'Saudi Arabic Adaptation',
    a.content = 'Content adaptation rules for Saudi Arabia - formal, Islamic-conscious, hierarchical',
    a.llm_context = 'USE: when generating content for Saudi Arabia audience. TRIGGERS: ar-SA, Saudi Arabia, Gulf Arabic. NOT: for Egyptian or Levantine Arabic. RELATES: Locale ar-SA, Style ar-SA.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'low',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","islamic_references":"ILLUSTRATION_RESPECTFUL","cultural_references":"ILLUSTRATION_SAUDI"}',
    a.adaptation_summary = 'Saudi Arabic is Modern Standard Arabic (MSA) influenced by Gulf dialect. Islamic values central - avoid haram content. Vision 2030 modernization embrace. Hospitality culture. Gender considerations in imagery. Family honor important. Hierarchy respected. Royal family reference with respect. Ramadan and Islamic calendar awareness.',
    a.decision_algorithm = '1. Religious content? -> Respectful Islamic framing\n2. Gender imagery? -> Conservative approach\n3. Calendar reference? -> Include Hijri dates\n4. Modernization? -> Vision 2030 positive\n5. Default: Formal, respectful, hierarchical',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'ar-SA'})
SET s.display_name = 'Saudi Arabic Style',
    s.content = 'Communication style for Saudi Arabia - formal, respectful, hierarchical',
    s.llm_context = 'USE: when setting tone for Saudi audience. TRIGGERS: ar-SA, Saudi Arabia. NOT: for Egyptian informality. RELATES: Locale ar-SA, Adaptation ar-SA.',
    s.formality_score = 80,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'high',
    s.individualism_level = 'collectivist',
    s.warmth_level = 'warm',
    s.directness_score = 35,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"very_formal","notes":"Hierarchy and titles essential"},"b2c":{"formality":"formal","notes":"Family-oriented messaging"},"religious":{"formality":"very_formal","notes":"Islamic framing"}}',
    s.pronoun_preference = 'n_a',
    s.humor_score = 30,
    s.humor_types = '{"warmth":"acceptable","irony":"careful","religious":"never","self_deprecating":"rare"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- ar-EG (Egypt) ---
MERGE (a:Adaptation {key: 'ar-EG'})
SET a.display_name = 'Egyptian Arabic Adaptation',
    a.content = 'Content adaptation rules for Egyptian Arabic - warm, humorous, culturally rich',
    a.llm_context = 'USE: when generating content for Egypt audience. TRIGGERS: ar-EG, Egypt, Egyptian Arabic. NOT: for Gulf or Levantine Arabic. RELATES: Locale ar-EG, Style ar-EG.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'high',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT","egyptian_dialect":"ILLUSTRATION","cultural_references":"ILLUSTRATION_EGYPTIAN"}',
    a.adaptation_summary = 'Egyptian Arabic (Masri) is most widely understood Arabic dialect due to media influence. More informal than Gulf Arabic. Humor and warmth valued. Ancient heritage pride alongside Islamic identity. Cairo-centric but acknowledge regional diversity (Upper Egypt, Alexandria). Economic pragmatism. Strong family values. Egyptian ingenuity (fahlawa).',
    a.decision_algorithm = '1. Dialect? -> Egyptian colloquial acceptable\n2. Humor? -> Egyptian wit valued\n3. Heritage? -> Pharaonic + Islamic blend\n4. Economy? -> Price-conscious messaging\n5. Default: Warm, humorous, culturally rich',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'ar-EG'})
SET s.display_name = 'Egyptian Arabic Style',
    s.content = 'Communication style for Egypt - warm, humorous, relationship-focused',
    s.llm_context = 'USE: when setting tone for Egypt audience. TRIGGERS: ar-EG, Egypt. NOT: for Gulf formality. RELATES: Locale ar-EG, Adaptation ar-EG.',
    s.formality_score = 55,
    s.default_formality = 'mixed',
    s.formality_default = 'neutral',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'collectivist',
    s.warmth_level = 'warm',
    s.directness_score = 50,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Relationship-first"},"b2c":{"formality":"casual","notes":"Humor resonates"},"family":{"formality":"informal","notes":"Collective decision"}}',
    s.pronoun_preference = 'n_a',
    s.humor_score = 75,
    s.humor_types = '{"wordplay":"highly_valued","irony":"common","self_deprecating":"valued","warmth":"essential"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// ============================================================================
// CHINESE LOCALES (zh-CN, zh-TW) - MUST BE DISTINCT
// ============================================================================

// --- zh-CN (Mainland China) ---
MERGE (a:Adaptation {key: 'zh-CN'})
SET a.display_name = 'Simplified Chinese Adaptation',
    a.content = 'Content adaptation rules for Mainland China - collective harmony, face-conscious, government-aware',
    a.llm_context = 'USE: when generating content for Mainland China audience. TRIGGERS: zh-CN, China, Simplified Chinese. NOT: for Taiwan or Hong Kong. RELATES: Locale zh-CN, Style zh-CN.',
    a.technical_terms_approach = 'local_only',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT_LOCALIZED","political_references":"ILLUSTRATION_CAREFUL","cultural_references":"ILLUSTRATION_CHINESE"}',
    a.adaptation_summary = 'Simplified Chinese for Mainland China. Face (面子 miànzi) and harmony (和谐 héxié) paramount. Collective over individual. Government-sensitive topics avoid. Local brand names (Apple = 苹果 Píngguǒ). Guanxi (relationships) essential in business. Lucky numbers (8 = prosperity) matter. Family and education values. WeChat/Weibo ecosystem.',
    a.decision_algorithm = '1. Script? -> Simplified Chinese (简体)\n2. Brand names? -> Localized Chinese names\n3. Political content? -> Avoid sensitive topics\n4. Numbers? -> Leverage lucky numbers (8)\n5. Default: Collective harmony, face-conscious',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'zh-CN'})
SET s.display_name = 'Simplified Chinese Style',
    s.content = 'Communication style for Mainland China - formal, collective, harmony-focused',
    s.llm_context = 'USE: when setting tone for Mainland China audience. TRIGGERS: zh-CN, China. NOT: for Taiwan individualism. RELATES: Locale zh-CN, Adaptation zh-CN.',
    s.formality_score = 70,
    s.default_formality = 'formal',
    s.formality_default = 'formal',
    s.directness_level = 'indirect',
    s.hierarchy_importance = 'high',
    s.individualism_level = 'collectivist',
    s.warmth_level = 'neutral',
    s.directness_score = 40,
    s.directness_style = 'indirect',
    s.context_matrix = '{"b2b":{"formality":"formal","notes":"Guanxi and face essential"},"b2c":{"formality":"moderate","notes":"Mobile-first, social proof"},"government":{"formality":"very_formal","notes":"Political sensitivity"}}',
    s.pronoun_preference = 'n_a',
    s.humor_score = 45,
    s.humor_types = '{"wordplay":"valued","self_deprecating":"acceptable","political":"never","puns":"lucky_numbers"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// --- zh-TW (Taiwan) ---
MERGE (a:Adaptation {key: 'zh-TW'})
SET a.display_name = 'Traditional Chinese (Taiwan) Adaptation',
    a.content = 'Content adaptation rules for Taiwan - democratic values, individualistic, tech-forward',
    a.llm_context = 'USE: when generating content for Taiwan audience. TRIGGERS: zh-TW, Taiwan, Traditional Chinese. NOT: for Mainland China. RELATES: Locale zh-TW, Style zh-TW.',
    a.technical_terms_approach = 'mixed',
    a.illustration_density = 'medium',
    a.facts_classification = '{"proper_names":"FACT","brand_names":"FACT_ORIGINAL","democratic_references":"ILLUSTRATION_POSITIVE","cultural_references":"ILLUSTRATION_TAIWANESE"}',
    a.adaptation_summary = 'Traditional Chinese (繁體) for Taiwan. Democratic values and free speech celebrated. More individualistic than Mainland. Tech-forward (semiconductor industry pride). Japanese cultural influence. Night market and local food culture. Avoid One China political sensitivity. Taiwan identity distinct from PRC. LINE messaging platform dominant.',
    a.decision_algorithm = '1. Script? -> Traditional Chinese (繁體)\n2. Brand names? -> Original English often kept\n3. Political content? -> Taiwan identity positive\n4. Tech references? -> Semiconductor pride\n5. Default: Individual, democratic, tech-savvy',
    a.created_at = datetime(),
    a.updated_at = datetime();

MERGE (s:Style {key: 'zh-TW'})
SET s.display_name = 'Traditional Chinese (Taiwan) Style',
    s.content = 'Communication style for Taiwan - moderate, individualistic, tech-forward',
    s.llm_context = 'USE: when setting tone for Taiwan audience. TRIGGERS: zh-TW, Taiwan. NOT: for Mainland collectivism. RELATES: Locale zh-TW, Adaptation zh-TW.',
    s.formality_score = 55,
    s.default_formality = 'mixed',
    s.formality_default = 'neutral',
    s.directness_level = 'balanced',
    s.hierarchy_importance = 'medium',
    s.individualism_level = 'mixed',
    s.warmth_level = 'warm',
    s.directness_score = 55,
    s.directness_style = 'balanced',
    s.context_matrix = '{"b2b":{"formality":"moderate","notes":"Less hierarchical than Mainland"},"b2c":{"formality":"casual","notes":"Tech-savvy, quality-conscious"},"youth":{"formality":"casual","notes":"Japanese influence"}}',
    s.pronoun_preference = 'n_a',
    s.humor_score = 60,
    s.humor_types = '{"wordplay":"valued","irony":"acceptable","self_deprecating":"common","cute_culture":"embraced"}',
    s.created_at = datetime(),
    s.updated_at = datetime();

// ============================================================================
// CREATE ARCS: Locale -[:HAS_ADAPTATION]-> Adaptation, Locale -[:HAS_STYLE]-> Style
// ============================================================================

// French locales
MATCH (l:Locale {key: 'fr-FR'}), (a:Adaptation {key: 'fr-FR'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'fr-FR'}), (s:Style {key: 'fr-FR'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'fr-FR'}), (l:Locale {key: 'fr-FR'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'fr-FR'}), (l:Locale {key: 'fr-FR'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'fr-BE'}), (a:Adaptation {key: 'fr-BE'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'fr-BE'}), (s:Style {key: 'fr-BE'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'fr-BE'}), (l:Locale {key: 'fr-BE'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'fr-BE'}), (l:Locale {key: 'fr-BE'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'fr-CH'}), (a:Adaptation {key: 'fr-CH'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'fr-CH'}), (s:Style {key: 'fr-CH'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'fr-CH'}), (l:Locale {key: 'fr-CH'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'fr-CH'}), (l:Locale {key: 'fr-CH'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'fr-CA'}), (a:Adaptation {key: 'fr-CA'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'fr-CA'}), (s:Style {key: 'fr-CA'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'fr-CA'}), (l:Locale {key: 'fr-CA'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'fr-CA'}), (l:Locale {key: 'fr-CA'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'fr-LU'}), (a:Adaptation {key: 'fr-LU'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'fr-LU'}), (s:Style {key: 'fr-LU'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'fr-LU'}), (l:Locale {key: 'fr-LU'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'fr-LU'}), (l:Locale {key: 'fr-LU'})
MERGE (s)-[:STYLE_OF]->(l);

// Spanish locales
MATCH (l:Locale {key: 'es-ES'}), (a:Adaptation {key: 'es-ES'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'es-ES'}), (s:Style {key: 'es-ES'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'es-ES'}), (l:Locale {key: 'es-ES'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'es-ES'}), (l:Locale {key: 'es-ES'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'es-MX'}), (a:Adaptation {key: 'es-MX'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'es-MX'}), (s:Style {key: 'es-MX'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'es-MX'}), (l:Locale {key: 'es-MX'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'es-MX'}), (l:Locale {key: 'es-MX'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'es-AR'}), (a:Adaptation {key: 'es-AR'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'es-AR'}), (s:Style {key: 'es-AR'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'es-AR'}), (l:Locale {key: 'es-AR'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'es-AR'}), (l:Locale {key: 'es-AR'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'es-CO'}), (a:Adaptation {key: 'es-CO'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'es-CO'}), (s:Style {key: 'es-CO'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'es-CO'}), (l:Locale {key: 'es-CO'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'es-CO'}), (l:Locale {key: 'es-CO'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'es-CL'}), (a:Adaptation {key: 'es-CL'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'es-CL'}), (s:Style {key: 'es-CL'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'es-CL'}), (l:Locale {key: 'es-CL'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'es-CL'}), (l:Locale {key: 'es-CL'})
MERGE (s)-[:STYLE_OF]->(l);

// Portuguese locales
MATCH (l:Locale {key: 'pt-BR'}), (a:Adaptation {key: 'pt-BR'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'pt-BR'}), (s:Style {key: 'pt-BR'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'pt-BR'}), (l:Locale {key: 'pt-BR'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'pt-BR'}), (l:Locale {key: 'pt-BR'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'pt-PT'}), (a:Adaptation {key: 'pt-PT'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'pt-PT'}), (s:Style {key: 'pt-PT'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'pt-PT'}), (l:Locale {key: 'pt-PT'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'pt-PT'}), (l:Locale {key: 'pt-PT'})
MERGE (s)-[:STYLE_OF]->(l);

// English locales
MATCH (l:Locale {key: 'en-US'}), (a:Adaptation {key: 'en-US'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'en-US'}), (s:Style {key: 'en-US'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'en-US'}), (l:Locale {key: 'en-US'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'en-US'}), (l:Locale {key: 'en-US'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'en-GB'}), (a:Adaptation {key: 'en-GB'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'en-GB'}), (s:Style {key: 'en-GB'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'en-GB'}), (l:Locale {key: 'en-GB'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'en-GB'}), (l:Locale {key: 'en-GB'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'en-AU'}), (a:Adaptation {key: 'en-AU'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'en-AU'}), (s:Style {key: 'en-AU'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'en-AU'}), (l:Locale {key: 'en-AU'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'en-AU'}), (l:Locale {key: 'en-AU'})
MERGE (s)-[:STYLE_OF]->(l);

// German locales
MATCH (l:Locale {key: 'de-DE'}), (a:Adaptation {key: 'de-DE'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'de-DE'}), (s:Style {key: 'de-DE'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'de-DE'}), (l:Locale {key: 'de-DE'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'de-DE'}), (l:Locale {key: 'de-DE'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'de-CH'}), (a:Adaptation {key: 'de-CH'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'de-CH'}), (s:Style {key: 'de-CH'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'de-CH'}), (l:Locale {key: 'de-CH'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'de-CH'}), (l:Locale {key: 'de-CH'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'de-AT'}), (a:Adaptation {key: 'de-AT'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'de-AT'}), (s:Style {key: 'de-AT'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'de-AT'}), (l:Locale {key: 'de-AT'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'de-AT'}), (l:Locale {key: 'de-AT'})
MERGE (s)-[:STYLE_OF]->(l);

// Italian locales
MATCH (l:Locale {key: 'it-IT'}), (a:Adaptation {key: 'it-IT'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'it-IT'}), (s:Style {key: 'it-IT'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'it-IT'}), (l:Locale {key: 'it-IT'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'it-IT'}), (l:Locale {key: 'it-IT'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'it-CH'}), (a:Adaptation {key: 'it-CH'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'it-CH'}), (s:Style {key: 'it-CH'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'it-CH'}), (l:Locale {key: 'it-CH'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'it-CH'}), (l:Locale {key: 'it-CH'})
MERGE (s)-[:STYLE_OF]->(l);

// Arabic locales
MATCH (l:Locale {key: 'ar-SA'}), (a:Adaptation {key: 'ar-SA'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'ar-SA'}), (s:Style {key: 'ar-SA'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'ar-SA'}), (l:Locale {key: 'ar-SA'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'ar-SA'}), (l:Locale {key: 'ar-SA'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'ar-EG'}), (a:Adaptation {key: 'ar-EG'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'ar-EG'}), (s:Style {key: 'ar-EG'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'ar-EG'}), (l:Locale {key: 'ar-EG'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'ar-EG'}), (l:Locale {key: 'ar-EG'})
MERGE (s)-[:STYLE_OF]->(l);

// Chinese locales
MATCH (l:Locale {key: 'zh-CN'}), (a:Adaptation {key: 'zh-CN'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'zh-CN'}), (s:Style {key: 'zh-CN'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'zh-CN'}), (l:Locale {key: 'zh-CN'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'zh-CN'}), (l:Locale {key: 'zh-CN'})
MERGE (s)-[:STYLE_OF]->(l);

MATCH (l:Locale {key: 'zh-TW'}), (a:Adaptation {key: 'zh-TW'})
MERGE (l)-[:HAS_ADAPTATION]->(a);
MATCH (l:Locale {key: 'zh-TW'}), (s:Style {key: 'zh-TW'})
MERGE (l)-[:HAS_STYLE]->(s);
MATCH (a:Adaptation {key: 'zh-TW'}), (l:Locale {key: 'zh-TW'})
MERGE (a)-[:ADAPTATION_OF]->(l);
MATCH (s:Style {key: 'zh-TW'}), (l:Locale {key: 'zh-TW'})
MERGE (s)-[:STYLE_OF]->(l);

// ============================================================================
// VERIFICATION QUERY
// ============================================================================

// Return summary of created nodes
MATCH (a:Adaptation)
WITH count(a) AS adaptation_count
MATCH (s:Style)
WITH adaptation_count, count(s) AS style_count
MATCH ()-[ha:HAS_ADAPTATION]->()
WITH adaptation_count, style_count, count(ha) AS has_adaptation_arcs
MATCH ()-[hs:HAS_STYLE]->()
RETURN adaptation_count, style_count, has_adaptation_arcs, count(hs) AS has_style_arcs;
