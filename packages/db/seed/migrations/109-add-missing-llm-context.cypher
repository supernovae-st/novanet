// ============================================================================
// Migration 109: Add missing llm_context property (v0.19.0 Standard Properties)
// ============================================================================
//
// v0.19.0 introduces the `llm_context` property following ADR-027 pattern.
// This property provides semantic guidance for LLM agents interacting with
// the knowledge graph via MCP tools.
//
// ADR-027 llm_context format:
//   USE: when [primary use case for this element].
//   TRIGGERS: "keyword1", "keyword2", "keyword3".
//   NOT: for [what NOT to use this for] (use [alternative] instead).
//   RELATES: [Source] (role), [Target] (role), [Related] (relationship).
//
// This migration adds default llm_context values for all nodes missing
// this property, categorized by their semantic role in the graph.
//
// ============================================================================

// -----------------------------------------------------------------------------
// KNOWLEDGE ATOMS: Locale-specific content building blocks
// -----------------------------------------------------------------------------

MATCH (n:Expression) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when writing natural prose in target locale with idiomatic expressions. TRIGGERS: "idiom", "phrase", "expression", "natural", "colloquial". NOT: for technical terms (use EntityNative.denomination_forms). RELATES: Locale (belongs to), ExpressionSet (container).';

MATCH (n:Pattern) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when generating structured content following locale-specific templates. TRIGGERS: "template", "pattern", "format", "structure". NOT: for free-form prose (use Expression). RELATES: Locale (belongs to), PatternSet (container).';

MATCH (n:CultureRef) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when adding cultural references appropriate for target audience. TRIGGERS: "culture", "reference", "local", "familiar". NOT: for taboos or expressions (use Taboo or Expression). RELATES: Locale (belongs to), CultureSet (container).';

MATCH (n:Taboo) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when filtering content to avoid cultural taboos and sensitivities. TRIGGERS: "avoid", "taboo", "sensitive", "forbidden", "inappropriate". NOT: for positive cultural references (use CultureRef). RELATES: Locale (belongs to), TabooSet (container).';

MATCH (n:AudienceTrait) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when tailoring content tone and style for target audience characteristics. TRIGGERS: "audience", "demographic", "trait", "persona", "reader". NOT: for locale-specific expressions (use Expression). RELATES: Locale (belongs to), AudienceSet (container).';

// -----------------------------------------------------------------------------
// KNOWLEDGE CONTAINERS: Grouping nodes for atoms
// -----------------------------------------------------------------------------

MATCH (n:ExpressionSet) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when loading all expressions for a locale in bulk. TRIGGERS: "expression set", "all expressions", "locale expressions". NOT: for individual expressions (traverse to Expression nodes). RELATES: Locale (owner), Expression (contains).';

MATCH (n:PatternSet) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when loading all patterns for a locale in bulk. TRIGGERS: "pattern set", "all patterns", "locale patterns". NOT: for individual patterns (traverse to Pattern nodes). RELATES: Locale (owner), Pattern (contains).';

MATCH (n:CultureSet) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when loading all cultural references for a locale in bulk. TRIGGERS: "culture set", "all references", "locale culture". NOT: for individual references (traverse to CultureRef nodes). RELATES: Locale (owner), CultureRef (contains).';

MATCH (n:TabooSet) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when loading all taboos for a locale in bulk. TRIGGERS: "taboo set", "all taboos", "locale restrictions". NOT: for individual taboos (traverse to Taboo nodes). RELATES: Locale (owner), Taboo (contains).';

MATCH (n:AudienceSet) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when loading all audience traits for a locale in bulk. TRIGGERS: "audience set", "all traits", "locale audience". NOT: for individual traits (traverse to AudienceTrait nodes). RELATES: Locale (owner), AudienceTrait (contains).';

// -----------------------------------------------------------------------------
// LOCALE LAYER: Configuration nodes
// -----------------------------------------------------------------------------

MATCH (n:Locale) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when determining language and cultural settings for content generation. TRIGGERS: "locale", "language", "region", "BCP-47", "l10n". NOT: for content (use EntityNative, PageNative). RELATES: Country (located in), Culture (follows), Adaptation (uses).';

MATCH (n:Slugification) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when generating URL-safe slugs following locale-specific rules. TRIGGERS: "slug", "url", "transliteration", "ascii". NOT: for display text (use EntityNative.denomination_forms). RELATES: Locale (applies to).';

MATCH (n:Formatting) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when formatting dates, numbers, currencies for target locale. TRIGGERS: "format", "date", "number", "currency", "decimal". NOT: for text content (use Expression). RELATES: Locale (applies to).';

MATCH (n:Culture) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when understanding cultural context dimensions (Hofstede). TRIGGERS: "culture", "hofstede", "dimension", "context", "collectivism". NOT: for specific references (use CultureRef). RELATES: Locale (belongs to), CulturalRealm (part of).';

MATCH (n:Adaptation) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when determining content adaptation strategy for locale. TRIGGERS: "adaptation", "localization", "transcreation", "translation". NOT: for formatting rules (use Formatting). RELATES: Locale (applies to).';

MATCH (n:Style) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when setting tone, register, and stylistic conventions for locale. TRIGGERS: "style", "tone", "register", "formal", "casual". NOT: for content templates (use Pattern). RELATES: Locale (applies to), Adaptation (influences).';

MATCH (n:LanguageFamily) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when understanding linguistic relationships between locales. TRIGGERS: "language family", "indo-european", "sino-tibetan", "linguistic". NOT: for individual locales (use Locale). RELATES: LanguageBranch (contains).';

MATCH (n:LanguageBranch) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when understanding language sub-groupings. TRIGGERS: "branch", "romance", "germanic", "slavic". NOT: for cultural aspects (use CulturalRealm). RELATES: LanguageFamily (belongs to), Locale (contains).';

MATCH (n:CulturalRealm) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when understanding broad cultural regions. TRIGGERS: "cultural realm", "western", "eastern", "civilization". NOT: for specific countries (use Country). RELATES: CulturalSubRealm (contains).';

MATCH (n:CulturalSubRealm) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when understanding regional cultural variations. TRIGGERS: "sub-realm", "nordic", "mediterranean", "regional". NOT: for individual cultures (use Culture). RELATES: CulturalRealm (belongs to), Locale (influences).';

// -----------------------------------------------------------------------------
// GEOGRAPHY LAYER: Location and economic groupings
// -----------------------------------------------------------------------------

MATCH (n:Country) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when determining geographic and political context. TRIGGERS: "country", "nation", "ISO-3166", "territory". NOT: for language (use Locale). RELATES: Continent (located in), GeoRegion (part of), Locale (hosts).';

MATCH (n:Continent) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when grouping countries by major landmass. TRIGGERS: "continent", "africa", "europe", "asia", "americas". NOT: for economic groupings (use EconomicRegion). RELATES: Country (contains), GeoRegion (contains).';

MATCH (n:GeoRegion) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when grouping by UN M49 geographic regions. TRIGGERS: "region", "M49", "geographic", "UN". NOT: for economic groupings (use EconomicRegion). RELATES: Continent (part of), GeoSubRegion (contains).';

MATCH (n:GeoSubRegion) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when grouping by UN M49 geographic sub-regions. TRIGGERS: "sub-region", "western europe", "southeast asia". NOT: for population clusters (use PopulationCluster). RELATES: GeoRegion (part of), Country (contains).';

MATCH (n:PopulationCluster) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when targeting content by population scale. TRIGGERS: "population", "cluster", "mega", "large", "medium", "small". NOT: for geography (use GeoRegion). RELATES: PopulationSubCluster (contains).';

MATCH (n:PopulationSubCluster) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when fine-tuning population-based targeting. TRIGGERS: "sub-cluster", "population band", "demographic". NOT: for geography (use GeoSubRegion). RELATES: PopulationCluster (part of), Country (categorizes).';

MATCH (n:EconomicRegion) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when grouping by World Bank economic regions. TRIGGERS: "economic region", "world bank", "development". NOT: for income (use IncomeGroup). RELATES: Country (contains).';

MATCH (n:IncomeGroup) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when targeting content by World Bank income classification. TRIGGERS: "income", "high income", "low income", "middle income". NOT: for lending (use LendingCategory). RELATES: Country (categorizes).';

MATCH (n:LendingCategory) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when understanding World Bank lending eligibility. TRIGGERS: "lending", "IDA", "IBRD", "blend". NOT: for income (use IncomeGroup). RELATES: Country (categorizes).';

// -----------------------------------------------------------------------------
// SEMANTIC LAYER: Entity definitions
// -----------------------------------------------------------------------------

MATCH (n:EntityNative) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when loading locale-specific entity content for generation. TRIGGERS: "native", "localized", "entity content", "denomination". NOT: for entity definition (use Entity). RELATES: Entity (native of), Locale (for locale).';

MATCH (n:EntityCategory) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when grouping entities by semantic category. TRIGGERS: "category", "group", "type", "classification". NOT: for individual entities (use Entity). RELATES: Entity (categorizes).';

// -----------------------------------------------------------------------------
// STRUCTURE LAYER: Page/Block structure
// -----------------------------------------------------------------------------

MATCH (n:BlockType) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when determining block semantics and generation strategy. TRIGGERS: "block type", "hero", "faq", "cta", "feature". NOT: for block content (use BlockNative). RELATES: Block (types), ContentSlot (defines).';

MATCH (n:ContentSlot) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when understanding required content within a block type. TRIGGERS: "slot", "field", "content area", "placeholder". NOT: for block layout (use BlockType). RELATES: BlockType (belongs to).';

// -----------------------------------------------------------------------------
// OUTPUT LAYER: Generated content
// -----------------------------------------------------------------------------

MATCH (n:PageNative) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when reading or writing locale-specific page content. TRIGGERS: "page native", "localized page", "slug", "meta". NOT: for page structure (use Page). RELATES: Page (native of), Locale (for locale).';

MATCH (n:BlockNative) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when reading or writing locale-specific block content. TRIGGERS: "block native", "localized block", "content". NOT: for block structure (use Block). RELATES: Block (native of), Locale (for locale).';

// -----------------------------------------------------------------------------
// FOUNDATION LAYER: Project configuration
// -----------------------------------------------------------------------------

MATCH (n:ProjectSEOScope) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when determining SEO strategy and keyword targets. TRIGGERS: "seo", "scope", "keywords", "ranking". NOT: for GEO (use ProjectGEOScope). RELATES: Project (belongs to), SEOKeyword (contains).';

MATCH (n:ProjectGEOScope) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when determining GEO strategy and query targets. TRIGGERS: "geo", "scope", "queries", "ai search". NOT: for SEO (use ProjectSEOScope). RELATES: Project (belongs to), GEOQuery (contains).';

MATCH (n:SEOKeywordFormat) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when formatting SEO keywords for specific contexts. TRIGGERS: "keyword format", "seo format", "url", "title". NOT: for keyword data (use SEOKeyword). RELATES: SEOKeyword (formats).';

MATCH (n:GEOQuerySet) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when loading all GEO queries for a scope. TRIGGERS: "geo set", "all queries", "query collection". NOT: for individual queries (use GEOQuery). RELATES: ProjectGEOScope (belongs to), GEOQuery (contains).';

MATCH (n:GEOQuery) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when targeting AI search engine queries. TRIGGERS: "geo query", "ai search", "perplexity", "chatgpt search". NOT: for SEO keywords (use SEOKeyword). RELATES: GEOQuerySet (belongs to), GEOAnswer (has).';

MATCH (n:GEOAnswer) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when reading retrieved AI search engine answers. TRIGGERS: "geo answer", "ai response", "search result". NOT: for generating (this is retrieved). RELATES: GEOQuery (answers).';

MATCH (n:Brand) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when understanding brand identity for content generation. TRIGGERS: "brand", "identity", "voice", "personality". NOT: for principles (use BrandPrinciples). RELATES: Project (belongs to), BrandPrinciples (has).';

MATCH (n:BrandPrinciples) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when applying brand guidelines to content. TRIGGERS: "principles", "guidelines", "tone", "values". NOT: for brand identity (use Brand). RELATES: Brand (belongs to).';

// -----------------------------------------------------------------------------
// INSTRUCTION LAYER: Prompts and artifacts
// -----------------------------------------------------------------------------

MATCH (n:PromptStyle) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when setting prompt engineering style for generation. TRIGGERS: "prompt style", "instruction", "system prompt". NOT: for artifacts (use PromptArtifact). RELATES: Block (applies to).';

MATCH (n:PromptArtifact) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when loading prompt templates and instructions. TRIGGERS: "prompt artifact", "template", "instruction set". NOT: for output (use OutputArtifact). RELATES: PromptStyle (uses).';

MATCH (n:OutputArtifact) WHERE n.llm_context IS NULL
SET n.llm_context = 'USE: when storing generated content artifacts. TRIGGERS: "output artifact", "generated", "result". NOT: for prompts (use PromptArtifact). RELATES: Block (produced by).';

// -----------------------------------------------------------------------------
// Verification query (run manually):
// MATCH (n) WHERE n.llm_context IS NULL AND NOT n:Schema
// RETURN labels(n)[0] as label, count(*) as cnt
// ORDER BY cnt DESC;
// Expected: minimal results after migration (some nodes may not need llm_context)
// -----------------------------------------------------------------------------
