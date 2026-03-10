// ============================================================================
// PLAN C - Migration 098: Populate fr-FR Pattern Nodes
// ============================================================================
// Priority: CRITICAL (8/11 Pattern nodes are hollow)
// Fixes: Empty Pattern nodes for fr-FR locale
// CSR Impact: Enables French content generation with proper templates
// ============================================================================

// First check which patterns exist but are empty
MATCH (l:Locale {key: 'fr-FR'})-[:HAS_PATTERNS]->(ps:PatternSet)-[:CONTAINS_PATTERN]->(p:Pattern)
WHERE p.template IS NULL OR p.template = ''
RETURN p.key AS pattern_key,
       p.display_name AS name,
       'HOLLOW' AS status;

// Create or update standard CTA patterns for fr-FR
MERGE (p:Pattern {key: 'cta_primary_fr-FR'})
ON CREATE SET
  p.display_name = 'Primary CTA Pattern',
  p.description = 'Main call-to-action button text pattern',
  p.template = '{{action}} {{object}} {{modifier}}',
  p.examples = ['Créer mon code QR', 'Générer votre code', 'Commencer gratuitement'],
  p.tone = 'energetic',
  p.formality = 'neutral',
  p.locale = 'fr-FR',
  p.use_case = 'cta',
  p.llm_context = 'USE: for primary action buttons. TRIGGERS: cta, button, action, click. NOT: secondary actions, links.',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.template = COALESCE(p.template, '{{action}} {{object}} {{modifier}}'),
  p.examples = COALESCE(p.examples, ['Créer mon code QR', 'Générer votre code', 'Commencer gratuitement']),
  p.updated_at = datetime();

// Create secondary CTA pattern
MERGE (p:Pattern {key: 'cta_secondary_fr-FR'})
ON CREATE SET
  p.display_name = 'Secondary CTA Pattern',
  p.description = 'Secondary action or learn more pattern',
  p.template = '{{action}} {{detail}}',
  p.examples = ['En savoir plus', 'Découvrir les fonctionnalités', 'Voir les tarifs'],
  p.tone = 'warm',
  p.formality = 'neutral',
  p.locale = 'fr-FR',
  p.use_case = 'cta',
  p.llm_context = 'USE: for secondary actions and links. TRIGGERS: learn more, discover, see. NOT: primary CTAs.',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.template = COALESCE(p.template, '{{action}} {{detail}}'),
  p.updated_at = datetime();

// Create headline pattern
MERGE (p:Pattern {key: 'headline_hero_fr-FR'})
ON CREATE SET
  p.display_name = 'Hero Headline Pattern',
  p.description = 'Main headline for hero sections',
  p.template = '{{benefit}} {{product}} {{qualifier}}',
  p.examples = ['Créez des codes QR professionnels en quelques secondes', 'Le générateur de codes QR le plus simple'],
  p.tone = 'authoritative',
  p.formality = 'formal',
  p.locale = 'fr-FR',
  p.use_case = 'headline',
  p.llm_context = 'USE: for hero section headlines. TRIGGERS: hero, headline, h1. NOT: subheadlines, body text.',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.template = COALESCE(p.template, '{{benefit}} {{product}} {{qualifier}}'),
  p.updated_at = datetime();

// Create subheadline pattern
MERGE (p:Pattern {key: 'subheadline_hero_fr-FR'})
ON CREATE SET
  p.display_name = 'Hero Subheadline Pattern',
  p.description = 'Supporting text for hero headlines',
  p.template = '{{context}}. {{value_prop}}.',
  p.examples = ['Rejoignez plus de 10 000 professionnels. Générez, personnalisez et suivez vos codes QR.'],
  p.tone = 'warm',
  p.formality = 'neutral',
  p.locale = 'fr-FR',
  p.use_case = 'subheadline',
  p.llm_context = 'USE: for hero subheadlines and supporting text. TRIGGERS: subheadline, subtitle, h2. NOT: headlines, body.',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.template = COALESCE(p.template, '{{context}}. {{value_prop}}.'),
  p.updated_at = datetime();

// Create feature highlight pattern
MERGE (p:Pattern {key: 'feature_highlight_fr-FR'})
ON CREATE SET
  p.display_name = 'Feature Highlight Pattern',
  p.description = 'Short feature description pattern',
  p.template = '{{feature_name}} : {{benefit}}',
  p.examples = ['Personnalisation avancée : Ajoutez votre logo et vos couleurs', 'Suivi en temps réel : Analysez vos scans'],
  p.tone = 'energetic',
  p.formality = 'neutral',
  p.locale = 'fr-FR',
  p.use_case = 'feature',
  p.llm_context = 'USE: for feature cards and highlights. TRIGGERS: feature, capability, benefit. NOT: full descriptions.',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.template = COALESCE(p.template, '{{feature_name}} : {{benefit}}'),
  p.updated_at = datetime();

// Create FAQ question pattern
MERGE (p:Pattern {key: 'faq_question_fr-FR'})
ON CREATE SET
  p.display_name = 'FAQ Question Pattern',
  p.description = 'Natural question format for FAQs',
  p.template = '{{interrogative}} {{action}} {{object}} ?',
  p.examples = ['Comment créer un code QR ?', 'Qu\'est-ce qu\'un code QR dynamique ?', 'Pourquoi utiliser un code QR ?'],
  p.tone = 'friendly',
  p.formality = 'neutral',
  p.locale = 'fr-FR',
  p.use_case = 'faq',
  p.llm_context = 'USE: for FAQ questions. TRIGGERS: faq, question, how, what, why. NOT: answers, explanations.',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.template = COALESCE(p.template, '{{interrogative}} {{action}} {{object}} ?'),
  p.updated_at = datetime();

// Create meta title pattern
MERGE (p:Pattern {key: 'meta_title_fr-FR'})
ON CREATE SET
  p.display_name = 'Meta Title Pattern',
  p.description = 'SEO meta title pattern (50-60 chars)',
  p.template = '{{primary_keyword}} | {{brand}} - {{value_prop}}',
  p.examples = ['Générateur de Code QR | QR Code AI - Gratuit et Facile'],
  p.tone = 'authoritative',
  p.formality = 'neutral',
  p.locale = 'fr-FR',
  p.use_case = 'seo',
  p.max_length = 60,
  p.llm_context = 'USE: for SEO meta titles. TRIGGERS: meta title, seo title, page title. NOT: h1, content titles.',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.template = COALESCE(p.template, '{{primary_keyword}} | {{brand}} - {{value_prop}}'),
  p.updated_at = datetime();

// Create meta description pattern
MERGE (p:Pattern {key: 'meta_description_fr-FR'})
ON CREATE SET
  p.display_name = 'Meta Description Pattern',
  p.description = 'SEO meta description pattern (150-160 chars)',
  p.template = '{{hook}}. {{features}}. {{cta}}.',
  p.examples = ['Créez des codes QR personnalisés en quelques clics. Logo, couleurs, suivi des scans. Commencez gratuitement.'],
  p.tone = 'energetic',
  p.formality = 'neutral',
  p.locale = 'fr-FR',
  p.use_case = 'seo',
  p.max_length = 160,
  p.llm_context = 'USE: for SEO meta descriptions. TRIGGERS: meta description, seo description. NOT: body content.',
  p.created_at = datetime(),
  p.updated_at = datetime()
ON MATCH SET
  p.template = COALESCE(p.template, '{{hook}}. {{features}}. {{cta}}.'),
  p.updated_at = datetime();

// Link patterns to fr-FR PatternSet
MATCH (l:Locale {key: 'fr-FR'})-[:HAS_PATTERNS]->(ps:PatternSet)
MATCH (p:Pattern)
WHERE p.locale = 'fr-FR'
MERGE (ps)-[:CONTAINS_PATTERN]->(p);

// Create FOR_LOCALE arcs
MATCH (p:Pattern)
WHERE p.locale = 'fr-FR'
  AND NOT (p)-[:FOR_LOCALE]->(:Locale)
MATCH (l:Locale {key: 'fr-FR'})
MERGE (p)-[:FOR_LOCALE]->(l);

// Verify pattern completeness
MATCH (p:Pattern)
WHERE p.locale = 'fr-FR'
RETURN p.key AS pattern,
       CASE WHEN p.template IS NOT NULL AND p.template <> '' THEN 'HAS_TEMPLATE' ELSE 'HOLLOW' END AS status,
       p.use_case AS use_case
ORDER BY p.use_case, p.key;
