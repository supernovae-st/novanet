// ═══════════════════════════════════════════════════════════════════════════════
// 52-knowledge-atoms-bootstrap.cypher — Bootstrap instances for all NodeClasses
// v0.19.0 - Ensures at least 1 instance per NodeClass with ADR-042 standard properties
// ═══════════════════════════════════════════════════════════════════════════════
//
// Purpose: Create minimal instances for classes that had 0 instances
// Data sourced from: Perplexity research on US marketing/cultural knowledge
//
// Standard Properties (ADR-042):
//   - key, display_name, content (identity)
//   - node_class (PascalCase class name)
//   - provenance (object: source, method)
//   - llm_context (USE/TRIGGERS/NOT/RELATES)
//   - created_at, updated_at (timestamps)
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
  as.node_class = 'AudienceSet',
  as.content = 'Audience characteristics for US market targeting',
  as.llm_context = 'USE: container for audience traits specific to en-US locale. TRIGGERS: audience, traits, US market, targeting. NOT: individual traits (use AudienceTrait). RELATES: Locale (en-US), AudienceTrait (contains).',
  as.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  as.locale_key = 'en-US',
  as.created_at = datetime(),
  as.updated_at = datetime();

// AudienceTraits based on Perplexity research
MERGE (at:AudienceTrait {key: 'audience-trait:tech-savvy@en-US'})
ON CREATE SET
  at.display_name = 'Tech-Savvy',
  at.node_class = 'AudienceTrait',
  at.content = 'Digital natives comfortable with technology, mobile-first discovery',
  at.llm_context = 'USE: when targeting digitally native audience in US. TRIGGERS: tech, digital, mobile, Gen Z, millennial. NOT: traditional media audiences. RELATES: AudienceSet (en-US), marketing tone.',
  at.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  at.locale_key = 'en-US',
  at.trait_value = 'Discovers products via TikTok/Instagram, researches online before purchase',
  at.created_at = datetime(),
  at.updated_at = datetime();

MERGE (at:AudienceTrait {key: 'audience-trait:value-conscious@en-US'})
ON CREATE SET
  at.display_name = 'Value-Conscious',
  at.node_class = 'AudienceTrait',
  at.content = 'Price-sensitive shoppers who wait for sales and compare options',
  at.llm_context = 'USE: when writing copy for budget-conscious US audience. TRIGGERS: price, value, deal, discount, savings. NOT: luxury positioning. RELATES: AudienceSet (en-US), CTA patterns.',
  at.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  at.locale_key = 'en-US',
  at.trait_value = '79% wait for sales, prefer BNPL payment, brand-agnostic',
  at.created_at = datetime(),
  at.updated_at = datetime();

MERGE (at:AudienceTrait {key: 'audience-trait:experience-driven@en-US'})
ON CREATE SET
  at.display_name = 'Experience-Driven',
  at.node_class = 'AudienceTrait',
  at.content = 'Values experiences and emotional benefits over material goods',
  at.llm_context = 'USE: when emphasizing experience over features. TRIGGERS: experience, journey, lifestyle, emotional. NOT: feature lists. RELATES: AudienceSet (en-US), hero copy.',
  at.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
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
  cs.node_class = 'CultureSet',
  cs.content = 'American pop culture references for marketing resonance',
  cs.llm_context = 'USE: container for cultural references specific to US audience. TRIGGERS: culture, American, references, resonance. NOT: individual references (use CultureRef). RELATES: Locale (en-US), CultureRef (contains).',
  cs.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  cs.locale_key = 'en-US',
  cs.created_at = datetime(),
  cs.updated_at = datetime();

// CultureRef examples
MERGE (cr:CultureRef {key: 'culture-ref:instant-gratification@en-US'})
ON CREATE SET
  cr.display_name = 'Instant Gratification',
  cr.node_class = 'CultureRef',
  cr.content = 'American expectation of immediate results and convenience',
  cr.llm_context = 'USE: when emphasizing speed and immediacy in US marketing. TRIGGERS: instant, now, fast, immediate. NOT: slow/careful positioning. RELATES: CultureSet (en-US), CTA copy.',
  cr.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  cr.locale_key = 'en-US',
  cr.reference_value = 'Skip the wait, get results now',
  cr.created_at = datetime(),
  cr.updated_at = datetime();

MERGE (cr:CultureRef {key: 'culture-ref:diy-culture@en-US'})
ON CREATE SET
  cr.display_name = 'DIY Culture',
  cr.node_class = 'CultureRef',
  cr.content = 'American self-reliance and do-it-yourself mentality',
  cr.llm_context = 'USE: when positioning tools for self-starters. TRIGGERS: DIY, build, create, yourself. NOT: managed services. RELATES: CultureSet (en-US), feature copy.',
  cr.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  cr.locale_key = 'en-US',
  cr.reference_value = 'Build it yourself, no coding required',
  cr.created_at = datetime(),
  cr.updated_at = datetime();

MERGE (cr:CultureRef {key: 'culture-ref:hustle-culture@en-US'})
ON CREATE SET
  cr.display_name = 'Hustle Culture',
  cr.node_class = 'CultureRef',
  cr.content = 'American work ethic and entrepreneurial spirit',
  cr.llm_context = 'USE: when targeting entrepreneurs and small business. TRIGGERS: hustle, grow, scale, business. NOT: work-life balance messaging. RELATES: CultureSet (en-US), B2B copy.',
  cr.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
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
  ts.node_class = 'TabooSet',
  ts.content = 'Topics to avoid in US marketing content',
  ts.llm_context = 'USE: container for topics to avoid in US marketing. TRIGGERS: avoid, taboo, sensitive, prohibited. NOT: individual taboos (use Taboo). RELATES: Locale (en-US), Taboo (contains).',
  ts.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  ts.locale_key = 'en-US',
  ts.created_at = datetime(),
  ts.updated_at = datetime();

// Taboo examples from Perplexity research
MERGE (t:Taboo {key: 'taboo:body-shaming@en-US'})
ON CREATE SET
  t.display_name = 'Body Shaming',
  t.node_class = 'Taboo',
  t.content = 'Content that demeans physical appearance or promotes unrealistic beauty standards',
  t.llm_context = 'USE: to filter out body-shaming language in US content. TRIGGERS: weight, size, before/after, beauty. NOT: positive body messaging. RELATES: TabooSet (en-US), content filtering.',
  t.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  t.locale_key = 'en-US',
  t.severity = 'high',
  t.avoid_pattern = 'Avoid weight/size comments, before/after comparisons, beauty standards',
  t.created_at = datetime(),
  t.updated_at = datetime();

MERGE (t:Taboo {key: 'taboo:religious-messaging@en-US'})
ON CREATE SET
  t.display_name = 'Religious Messaging',
  t.node_class = 'Taboo',
  t.content = 'Ads containing religious messages or exploiting religious symbolism',
  t.llm_context = 'USE: to avoid religious references in US marketing. TRIGGERS: religious, faith, worship, blessing. NOT: secular language. RELATES: TabooSet (en-US), content filtering.',
  t.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  t.locale_key = 'en-US',
  t.severity = 'medium',
  t.avoid_pattern = 'Avoid religious references, symbols, or messaging',
  t.created_at = datetime(),
  t.updated_at = datetime();

MERGE (t:Taboo {key: 'taboo:cultural-appropriation@en-US'})
ON CREATE SET
  t.display_name = 'Cultural Appropriation',
  t.node_class = 'Taboo',
  t.content = 'Using cultural elements superficially or disrespectfully',
  t.llm_context = 'USE: to prevent cultural appropriation in US content. TRIGGERS: cultural, ethnic, tradition, heritage. NOT: respectful cultural acknowledgment. RELATES: TabooSet (en-US), diversity review.',
  t.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
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
  ps.node_class = 'PatternSet',
  ps.content = 'Call-to-action text patterns for US audience',
  ps.llm_context = 'USE: container for CTA patterns specific to US audience. TRIGGERS: CTA, patterns, call-to-action, buttons. NOT: individual patterns (use Pattern). RELATES: Locale (en-US), Pattern (contains).',
  ps.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  ps.locale_key = 'en-US',
  ps.domain = 'cta',
  ps.created_at = datetime(),
  ps.updated_at = datetime();

// Pattern examples
MERGE (p:Pattern {key: 'pattern:cta-create@en-US'})
ON CREATE SET
  p.display_name = 'Create CTA',
  p.node_class = 'Pattern',
  p.content = 'Action-oriented create button text',
  p.llm_context = 'USE: for primary action buttons in US interfaces. TRIGGERS: create, make, build, generate. NOT: passive actions. RELATES: PatternSet (CTA), hero sections.',
  p.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  p.locale_key = 'en-US',
  p.pattern_template = 'Create {noun} Now',
  p.example = 'Create QR Code Now',
  p.created_at = datetime(),
  p.updated_at = datetime();

MERGE (p:Pattern {key: 'pattern:cta-get-started@en-US'})
ON CREATE SET
  p.display_name = 'Get Started CTA',
  p.node_class = 'Pattern',
  p.content = 'Onboarding call-to-action',
  p.llm_context = 'USE: for signup/onboarding flows in US. TRIGGERS: start, begin, onboard, join. NOT: checkout/purchase. RELATES: PatternSet (CTA), hero sections.',
  p.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  p.locale_key = 'en-US',
  p.pattern_template = 'Get Started {modifier}',
  p.example = 'Get Started Free',
  p.created_at = datetime(),
  p.updated_at = datetime();

MERGE (p:Pattern {key: 'pattern:cta-try@en-US'})
ON CREATE SET
  p.display_name = 'Try CTA',
  p.node_class = 'Pattern',
  p.content = 'Low-commitment trial invitation',
  p.llm_context = 'USE: for trial/demo CTAs in US. TRIGGERS: try, test, demo, sample. NOT: purchase/commit. RELATES: PatternSet (CTA), secondary CTAs.',
  p.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
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
  gqs.node_class = 'GEOQuerySet',
  gqs.content = 'Geographic intelligence queries for QR Code domain',
  gqs.llm_context = 'USE: container for GEO queries about QR codes in US. TRIGGERS: geographic, market, regional, queries. NOT: individual queries (use GEOQuery). RELATES: Locale (en-US), GEOQuery (contains).',
  gqs.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  gqs.locale_key = 'en-US',
  gqs.domain = 'qr-code',
  gqs.created_at = datetime(),
  gqs.updated_at = datetime();

// GEOQuery example
MERGE (gq:GEOQuery {key: 'geo-query:qr-adoption-us@en-US'})
ON CREATE SET
  gq.display_name = 'QR Code Adoption US',
  gq.node_class = 'GEOQuery',
  gq.content = 'Query about QR code adoption rates in United States',
  gq.llm_context = 'USE: when needing US QR adoption statistics. TRIGGERS: adoption, statistics, usage, market. NOT: other regions. RELATES: GEOQuerySet (qr-code), GEOAnswer.',
  gq.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  gq.locale_key = 'en-US',
  gq.query_text = 'What is the QR code adoption rate in the United States?',
  gq.status = 'pending',
  gq.created_at = datetime(),
  gq.updated_at = datetime();

// GEOAnswer example (placeholder)
MERGE (ga:GEOAnswer {key: 'geo-answer:qr-adoption-us@en-US'})
ON CREATE SET
  ga.display_name = 'QR Adoption Answer',
  ga.node_class = 'GEOAnswer',
  ga.content = 'Answer about QR code adoption in US',
  ga.llm_context = 'USE: when citing US QR adoption data. TRIGGERS: statistics, adoption rate, Statista, 2022. NOT: other regions. RELATES: GEOQuery (qr-adoption), content generation.',
  ga.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
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
  cs.node_class = 'ContentSlot',
  cs.content = 'Primary hero section at top of landing page',
  cs.llm_context = 'USE: when defining content slot for hero section. TRIGGERS: hero, header, top, primary. NOT: body content. RELATES: Page (structure), Block (content).',
  cs.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  cs.slot_type = 'hero',
  cs.position = 1,
  cs.created_at = datetime(),
  cs.updated_at = datetime();

MERGE (cs:ContentSlot {key: 'content-slot:features'})
ON CREATE SET
  cs.display_name = 'Features Section',
  cs.node_class = 'ContentSlot',
  cs.content = 'Product features grid section',
  cs.llm_context = 'USE: when defining content slot for features. TRIGGERS: features, benefits, grid, cards. NOT: hero or CTA. RELATES: Page (structure), Block (content).',
  cs.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  cs.slot_type = 'features',
  cs.position = 2,
  cs.created_at = datetime(),
  cs.updated_at = datetime();

MERGE (cs:ContentSlot {key: 'content-slot:cta'})
ON CREATE SET
  cs.display_name = 'CTA Section',
  cs.node_class = 'ContentSlot',
  cs.content = 'Call-to-action section',
  cs.llm_context = 'USE: when defining content slot for CTA. TRIGGERS: CTA, action, conversion, button. NOT: informational sections. RELATES: Page (structure), Block (content).',
  cs.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
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
  bp.node_class = 'BrandPrinciples',
  bp.content = 'Core brand principles for QR Code AI platform',
  bp.llm_context = 'USE: when generating content for QR Code AI brand. TRIGGERS: brand, principles, voice, tone. NOT: competitor brands. RELATES: Brand (qrcode-ai), content generation.',
  bp.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
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
  ps.node_class = 'PromptStyle',
  ps.content = 'Friendly, approachable writing style for marketing copy',
  ps.llm_context = 'USE: for consumer-facing marketing copy. TRIGGERS: friendly, casual, conversational, approachable. NOT: formal/technical. RELATES: Block (marketing), generation prompts.',
  ps.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  ps.style_type = 'conversational',
  ps.characteristics = ['second person (you)', 'short sentences', 'active voice'],
  ps.created_at = datetime(),
  ps.updated_at = datetime();

MERGE (ps:PromptStyle {key: 'prompt-style:technical'})
ON CREATE SET
  ps.display_name = 'Technical Style',
  ps.node_class = 'PromptStyle',
  ps.content = 'Precise, technical writing for documentation',
  ps.llm_context = 'USE: for technical documentation and API docs. TRIGGERS: technical, precise, documentation, reference. NOT: marketing copy. RELATES: Block (docs), generation prompts.',
  ps.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  ps.style_type = 'technical',
  ps.characteristics = ['third person', 'precise terminology', 'step-by-step'],
  ps.created_at = datetime(),
  ps.updated_at = datetime();

// PromptArtifact
MERGE (pa:PromptArtifact {key: 'prompt-artifact:hero-generation'})
ON CREATE SET
  pa.display_name = 'Hero Section Prompt',
  pa.node_class = 'PromptArtifact',
  pa.content = 'Generate a hero section for {entity} targeting {locale} audience. Include headline, subheadline, and CTA.',
  pa.llm_context = 'USE: as template for hero section generation. TRIGGERS: hero, headline, prompt, template. NOT: final content. RELATES: Block (hero), generation pipeline.',
  pa.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  pa.artifact_type = 'prompt_template',
  pa.created_at = datetime(),
  pa.updated_at = datetime();

// OutputArtifact
MERGE (oa:OutputArtifact {key: 'output-artifact:sample-hero'})
ON CREATE SET
  oa.display_name = 'Sample Hero Output',
  oa.node_class = 'OutputArtifact',
  oa.content = '{"headline": "Create Beautiful QR Codes in Seconds", "subheadline": "The fastest way to generate custom QR codes for your business", "cta": "Create QR Code Now"}',
  oa.llm_context = 'USE: as example of generated hero content. TRIGGERS: example, sample, output, generated. NOT: live content. RELATES: PromptArtifact (hero), BlockNative.',
  oa.provenance = {source: 'seed', method: 'bootstrap', version: 'v0.19.0'},
  oa.artifact_type = 'generated_content',
  oa.created_at = datetime(),
  oa.updated_at = datetime();

// =============================================================================
// Summary
// =============================================================================
RETURN 'Knowledge atoms bootstrap complete: 17 classes populated with ADR-042 standard properties' AS status;
