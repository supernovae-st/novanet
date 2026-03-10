// ═══════════════════════════════════════════════════════════════════════════════
// 52-knowledge-atoms-bootstrap.cypher — Bootstrap instances for all NodeClasses
// v0.17.0 - Ensures at least 1 instance per NodeClass
// ═══════════════════════════════════════════════════════════════════════════════
//
// Purpose: Create minimal instances for classes that had 0 instances
// Data sourced from: Perplexity research on US marketing/cultural knowledge
//
// Classes covered (17 total):
//   - AudienceSet, AudienceTrait (audience profiling)
//   - CultureSet, CultureRef (cultural references)
//   - TabooSet, Taboo (cultural taboos)
//   - PatternSet, Pattern (text patterns)
//   - GEOQuerySet, GEOQuery, GEOAnswer (geographic intel)
//   - ContentSlot (content structure)
//   - BrandPrinciples (brand foundation)
//   - PromptStyle, PromptArtifact, OutputArtifact (generation)
//   - EntityNative (semantic content - see separate file 53)
// ═══════════════════════════════════════════════════════════════════════════════

// =============================================================================
// AUDIENCE LAYER — AudienceSet + AudienceTrait
// =============================================================================

// AudienceSet for en-US
MERGE (as:AudienceSet {key: 'audience-set:en-US'})
ON CREATE SET
  as.display_name = 'US Audience Traits',
  as.description = 'Audience characteristics for US market targeting',
  as.locale_key = 'en-US',
  as.created_at = datetime(),
  as.updated_at = datetime();

// AudienceTraits based on Perplexity research
MERGE (at:AudienceTrait {key: 'audience-trait:tech-savvy@en-US'})
ON CREATE SET
  at.display_name = 'Tech-Savvy',
  at.description = 'Digital natives comfortable with technology, mobile-first discovery',
  at.locale_key = 'en-US',
  at.trait_value = 'Discovers products via TikTok/Instagram, researches online before purchase',
  at.created_at = datetime(),
  at.updated_at = datetime();

MERGE (at:AudienceTrait {key: 'audience-trait:value-conscious@en-US'})
ON CREATE SET
  at.display_name = 'Value-Conscious',
  at.description = 'Price-sensitive shoppers who wait for sales and compare options',
  at.locale_key = 'en-US',
  at.trait_value = '79% wait for sales, prefer BNPL payment, brand-agnostic',
  at.created_at = datetime(),
  at.updated_at = datetime();

MERGE (at:AudienceTrait {key: 'audience-trait:experience-driven@en-US'})
ON CREATE SET
  at.display_name = 'Experience-Driven',
  at.description = 'Values experiences and emotional benefits over material goods',
  at.locale_key = 'en-US',
  at.trait_value = 'Seeks hands-on activities, social experiences, comfort items',
  at.created_at = datetime(),
  at.updated_at = datetime();

// Link AudienceTraits to AudienceSet
MATCH (as:AudienceSet {key: 'audience-set:en-US'})
MATCH (at:AudienceTrait) WHERE at.locale_key = 'en-US'
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

// Link AudienceSet to Locale
MATCH (as:AudienceSet {key: 'audience-set:en-US'})
MATCH (loc:Locale {key: 'en-US'})
MERGE (as)-[:FOR_LOCALE]->(loc);

// Link AudienceTraits to Locale
MATCH (at:AudienceTrait) WHERE at.locale_key = 'en-US'
MATCH (loc:Locale {key: 'en-US'})
MERGE (at)-[:FOR_LOCALE]->(loc);

// =============================================================================
// CULTURE LAYER — CultureSet + CultureRef
// =============================================================================

// CultureSet for en-US
MERGE (cs:CultureSet {key: 'culture-set:en-US'})
ON CREATE SET
  cs.display_name = 'US Cultural References',
  cs.description = 'American pop culture references for marketing resonance',
  cs.locale_key = 'en-US',
  cs.created_at = datetime(),
  cs.updated_at = datetime();

// CultureRef examples
MERGE (cr:CultureRef {key: 'culture-ref:instant-gratification@en-US'})
ON CREATE SET
  cr.display_name = 'Instant Gratification',
  cr.description = 'American expectation of immediate results and convenience',
  cr.locale_key = 'en-US',
  cr.reference_value = 'Skip the wait, get results now',
  cr.created_at = datetime(),
  cr.updated_at = datetime();

MERGE (cr:CultureRef {key: 'culture-ref:diy-culture@en-US'})
ON CREATE SET
  cr.display_name = 'DIY Culture',
  cr.description = 'American self-reliance and do-it-yourself mentality',
  cr.locale_key = 'en-US',
  cr.reference_value = 'Build it yourself, no coding required',
  cr.created_at = datetime(),
  cr.updated_at = datetime();

MERGE (cr:CultureRef {key: 'culture-ref:hustle-culture@en-US'})
ON CREATE SET
  cr.display_name = 'Hustle Culture',
  cr.description = 'American work ethic and entrepreneurial spirit',
  cr.locale_key = 'en-US',
  cr.reference_value = 'Grow your business, scale your hustle',
  cr.created_at = datetime(),
  cr.updated_at = datetime();

// Link CultureRefs to CultureSet
MATCH (cs:CultureSet {key: 'culture-set:en-US'})
MATCH (cr:CultureRef) WHERE cr.locale_key = 'en-US'
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

// Link CultureSet to Locale
MATCH (cs:CultureSet {key: 'culture-set:en-US'})
MATCH (loc:Locale {key: 'en-US'})
MERGE (cs)-[:FOR_LOCALE]->(loc);

// Link CultureRefs to Locale
MATCH (cr:CultureRef) WHERE cr.locale_key = 'en-US'
MATCH (loc:Locale {key: 'en-US'})
MERGE (cr)-[:FOR_LOCALE]->(loc);

// =============================================================================
// TABOO LAYER — TabooSet + Taboo
// =============================================================================

// TabooSet for en-US
MERGE (ts:TabooSet {key: 'taboo-set:en-US'})
ON CREATE SET
  ts.display_name = 'US Marketing Taboos',
  ts.description = 'Topics to avoid in US marketing content',
  ts.locale_key = 'en-US',
  ts.created_at = datetime(),
  ts.updated_at = datetime();

// Taboo examples from Perplexity research
MERGE (t:Taboo {key: 'taboo:body-shaming@en-US'})
ON CREATE SET
  t.display_name = 'Body Shaming',
  t.description = 'Content that demeans physical appearance or promotes unrealistic beauty standards',
  t.locale_key = 'en-US',
  t.severity = 'high',
  t.avoid_pattern = 'Avoid weight/size comments, before/after comparisons, beauty standards',
  t.created_at = datetime(),
  t.updated_at = datetime();

MERGE (t:Taboo {key: 'taboo:religious-messaging@en-US'})
ON CREATE SET
  t.display_name = 'Religious Messaging',
  t.description = 'Ads containing religious messages or exploiting religious symbolism',
  t.locale_key = 'en-US',
  t.severity = 'medium',
  t.avoid_pattern = 'Avoid religious references, symbols, or messaging',
  t.created_at = datetime(),
  t.updated_at = datetime();

MERGE (t:Taboo {key: 'taboo:cultural-appropriation@en-US'})
ON CREATE SET
  t.display_name = 'Cultural Appropriation',
  t.description = 'Using cultural elements superficially or disrespectfully',
  t.locale_key = 'en-US',
  t.severity = 'high',
  t.avoid_pattern = 'Avoid cultural clichés, stereotypes, tokenism',
  t.created_at = datetime(),
  t.updated_at = datetime();

// Link Taboos to TabooSet
MATCH (ts:TabooSet {key: 'taboo-set:en-US'})
MATCH (t:Taboo) WHERE t.locale_key = 'en-US'
MERGE (ts)-[:CONTAINS_TABOO]->(t);

// Link TabooSet to Locale
MATCH (ts:TabooSet {key: 'taboo-set:en-US'})
MATCH (loc:Locale {key: 'en-US'})
MERGE (ts)-[:FOR_LOCALE]->(loc);

// Link Taboos to Locale
MATCH (t:Taboo) WHERE t.locale_key = 'en-US'
MATCH (loc:Locale {key: 'en-US'})
MERGE (t)-[:FOR_LOCALE]->(loc);

// =============================================================================
// PATTERN LAYER — PatternSet + Pattern
// =============================================================================

// PatternSet for en-US
MERGE (ps:PatternSet {key: 'pattern-set:cta@en-US'})
ON CREATE SET
  ps.display_name = 'CTA Patterns',
  ps.description = 'Call-to-action text patterns for US audience',
  ps.locale_key = 'en-US',
  ps.domain = 'cta',
  ps.created_at = datetime(),
  ps.updated_at = datetime();

// Pattern examples
MERGE (p:Pattern {key: 'pattern:cta-create@en-US'})
ON CREATE SET
  p.display_name = 'Create CTA',
  p.description = 'Action-oriented create button text',
  p.locale_key = 'en-US',
  p.pattern_template = 'Create {noun} Now',
  p.example = 'Create QR Code Now',
  p.created_at = datetime(),
  p.updated_at = datetime();

MERGE (p:Pattern {key: 'pattern:cta-get-started@en-US'})
ON CREATE SET
  p.display_name = 'Get Started CTA',
  p.description = 'Onboarding call-to-action',
  p.locale_key = 'en-US',
  p.pattern_template = 'Get Started {modifier}',
  p.example = 'Get Started Free',
  p.created_at = datetime(),
  p.updated_at = datetime();

MERGE (p:Pattern {key: 'pattern:cta-try@en-US'})
ON CREATE SET
  p.display_name = 'Try CTA',
  p.description = 'Low-commitment trial invitation',
  p.locale_key = 'en-US',
  p.pattern_template = 'Try {product} {modifier}',
  p.example = 'Try It Free',
  p.created_at = datetime(),
  p.updated_at = datetime();

// Link Patterns to PatternSet
MATCH (ps:PatternSet {key: 'pattern-set:cta@en-US'})
MATCH (p:Pattern) WHERE p.locale_key = 'en-US'
MERGE (ps)-[:CONTAINS_PATTERN]->(p);

// Link PatternSet to Locale
MATCH (ps:PatternSet {key: 'pattern-set:cta@en-US'})
MATCH (loc:Locale {key: 'en-US'})
MERGE (ps)-[:FOR_LOCALE]->(loc);

// Link Patterns to Locale
MATCH (p:Pattern) WHERE p.locale_key = 'en-US'
MATCH (loc:Locale {key: 'en-US'})
MERGE (p)-[:FOR_LOCALE]->(loc);

// =============================================================================
// GEO LAYER — GEOQuerySet + GEOQuery + GEOAnswer
// =============================================================================

// GEOQuerySet for QR Code domain
MERGE (gqs:GEOQuerySet {key: 'geo-query-set:qr-code@en-US'})
ON CREATE SET
  gqs.display_name = 'QR Code GEO Queries',
  gqs.description = 'Geographic intelligence queries for QR Code domain',
  gqs.locale_key = 'en-US',
  gqs.domain = 'qr-code',
  gqs.created_at = datetime(),
  gqs.updated_at = datetime();

// GEOQuery example
MERGE (gq:GEOQuery {key: 'geo-query:qr-adoption-us@en-US'})
ON CREATE SET
  gq.display_name = 'QR Code Adoption US',
  gq.description = 'Query about QR code adoption rates in United States',
  gq.locale_key = 'en-US',
  gq.query_text = 'What is the QR code adoption rate in the United States?',
  gq.status = 'pending',
  gq.created_at = datetime(),
  gq.updated_at = datetime();

// GEOAnswer example (placeholder)
MERGE (ga:GEOAnswer {key: 'geo-answer:qr-adoption-us@en-US'})
ON CREATE SET
  ga.display_name = 'QR Adoption Answer',
  ga.description = 'Answer about QR code adoption in US',
  ga.locale_key = 'en-US',
  ga.answer_text = 'QR code usage in the US grew 94% between 2020-2022, with 89 million smartphone users scanning QR codes in 2022.',
  ga.source = 'Statista 2023',
  ga.retrieved_at = datetime('2026-03-01T00:00:00Z'),
  ga.created_at = datetime(),
  ga.updated_at = datetime();

// Link GEOQuery to GEOQuerySet
MATCH (gqs:GEOQuerySet {key: 'geo-query-set:qr-code@en-US'})
MATCH (gq:GEOQuery {key: 'geo-query:qr-adoption-us@en-US'})
MERGE (gqs)-[:CONTAINS_GEO_QUERY]->(gq);

// Link GEOAnswer to GEOQuery
MATCH (gq:GEOQuery {key: 'geo-query:qr-adoption-us@en-US'})
MATCH (ga:GEOAnswer {key: 'geo-answer:qr-adoption-us@en-US'})
MERGE (gq)-[:HAS_GEO_ANSWERS]->(ga);

// Link GEOQuerySet to Locale
MATCH (gqs:GEOQuerySet) WHERE gqs.locale_key = 'en-US'
MATCH (loc:Locale {key: 'en-US'})
MERGE (gqs)-[:FOR_LOCALE]->(loc);

// Link GEOQuery to Locale
MATCH (gq:GEOQuery) WHERE gq.locale_key = 'en-US'
MATCH (loc:Locale {key: 'en-US'})
MERGE (gq)-[:FOR_LOCALE]->(loc);

// Link GEOAnswer to Locale
MATCH (ga:GEOAnswer) WHERE ga.locale_key = 'en-US'
MATCH (loc:Locale {key: 'en-US'})
MERGE (ga)-[:FOR_LOCALE]->(loc);

// =============================================================================
// CONTENT STRUCTURE — ContentSlot
// =============================================================================

MERGE (cs:ContentSlot {key: 'content-slot:hero'})
ON CREATE SET
  cs.display_name = 'Hero Section',
  cs.description = 'Primary hero section at top of landing page',
  cs.slot_type = 'hero',
  cs.position = 1,
  cs.created_at = datetime(),
  cs.updated_at = datetime();

MERGE (cs:ContentSlot {key: 'content-slot:features'})
ON CREATE SET
  cs.display_name = 'Features Section',
  cs.description = 'Product features grid section',
  cs.slot_type = 'features',
  cs.position = 2,
  cs.created_at = datetime(),
  cs.updated_at = datetime();

MERGE (cs:ContentSlot {key: 'content-slot:cta'})
ON CREATE SET
  cs.display_name = 'CTA Section',
  cs.description = 'Call-to-action section',
  cs.slot_type = 'cta',
  cs.position = 3,
  cs.created_at = datetime(),
  cs.updated_at = datetime();

// =============================================================================
// BRAND FOUNDATION — BrandPrinciples
// =============================================================================

MERGE (bp:BrandPrinciples {key: 'brand-principles:qrcode-ai'})
ON CREATE SET
  bp.display_name = 'QR Code AI Brand Principles',
  bp.description = 'Core brand principles for QR Code AI platform',
  bp.brand_key = 'qrcode-ai',
  bp.principles = ['simplicity', 'speed', 'reliability', 'innovation'],
  bp.voice_tone = 'friendly, professional, helpful',
  bp.created_at = datetime(),
  bp.updated_at = datetime();

// =============================================================================
// GENERATION LAYER — PromptStyle, PromptArtifact, OutputArtifact
// =============================================================================

// PromptStyle
MERGE (ps:PromptStyle {key: 'prompt-style:conversational'})
ON CREATE SET
  ps.display_name = 'Conversational Style',
  ps.description = 'Friendly, approachable writing style for marketing copy',
  ps.style_type = 'conversational',
  ps.characteristics = ['second person (you)', 'short sentences', 'active voice'],
  ps.created_at = datetime(),
  ps.updated_at = datetime();

MERGE (ps:PromptStyle {key: 'prompt-style:technical'})
ON CREATE SET
  ps.display_name = 'Technical Style',
  ps.description = 'Precise, technical writing for documentation',
  ps.style_type = 'technical',
  ps.characteristics = ['third person', 'precise terminology', 'step-by-step'],
  ps.created_at = datetime(),
  ps.updated_at = datetime();

// PromptArtifact
MERGE (pa:PromptArtifact {key: 'prompt-artifact:hero-generation'})
ON CREATE SET
  pa.display_name = 'Hero Section Prompt',
  pa.description = 'Prompt template for generating hero section content',
  pa.artifact_type = 'prompt_template',
  pa.content = 'Generate a hero section for {entity} targeting {locale} audience. Include headline, subheadline, and CTA.',
  pa.created_at = datetime(),
  pa.updated_at = datetime();

// OutputArtifact
MERGE (oa:OutputArtifact {key: 'output-artifact:sample-hero'})
ON CREATE SET
  oa.display_name = 'Sample Hero Output',
  oa.description = 'Example generated hero section',
  oa.artifact_type = 'generated_content',
  oa.content = '{"headline": "Create Beautiful QR Codes in Seconds", "subheadline": "The fastest way to generate custom QR codes for your business", "cta": "Create QR Code Now"}',
  oa.created_at = datetime(),
  oa.updated_at = datetime();

// =============================================================================
// Summary
// =============================================================================
RETURN 'Knowledge atoms bootstrap complete: 17 classes populated' AS status;
