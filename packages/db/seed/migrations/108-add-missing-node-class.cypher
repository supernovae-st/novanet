// ============================================================================
// Migration 108: Add missing node_class property (v0.19.0 Standard Properties)
// ============================================================================
//
// v0.19.0 introduces standardized properties across all 61 node classes.
// The `node_class` property is REQUIRED on all data nodes to enable:
//   - Schema validation at runtime
//   - Type-safe MCP tool operations
//   - Unified tree navigation in TUI
//
// This migration sets node_class to the Neo4j label name for all nodes
// missing this property. Schema nodes are excluded (they use :Schema label).
//
// Canonical property order (ADR-029):
//   key, display_name, node_class, content, llm_context, provenance,
//   created_at, updated_at
//
// ============================================================================

// -----------------------------------------------------------------------------
// SHARED REALM: Knowledge Layer
// -----------------------------------------------------------------------------

MATCH (n:Expression) WHERE n.node_class IS NULL
SET n.node_class = 'Expression';

MATCH (n:ExpressionSet) WHERE n.node_class IS NULL
SET n.node_class = 'ExpressionSet';

MATCH (n:Pattern) WHERE n.node_class IS NULL
SET n.node_class = 'Pattern';

MATCH (n:PatternSet) WHERE n.node_class IS NULL
SET n.node_class = 'PatternSet';

MATCH (n:CultureRef) WHERE n.node_class IS NULL
SET n.node_class = 'CultureRef';

MATCH (n:CultureSet) WHERE n.node_class IS NULL
SET n.node_class = 'CultureSet';

MATCH (n:Taboo) WHERE n.node_class IS NULL
SET n.node_class = 'Taboo';

MATCH (n:TabooSet) WHERE n.node_class IS NULL
SET n.node_class = 'TabooSet';

MATCH (n:AudienceTrait) WHERE n.node_class IS NULL
SET n.node_class = 'AudienceTrait';

MATCH (n:AudienceSet) WHERE n.node_class IS NULL
SET n.node_class = 'AudienceSet';

// -----------------------------------------------------------------------------
// SHARED REALM: Locale Layer
// -----------------------------------------------------------------------------

MATCH (n:Locale) WHERE n.node_class IS NULL
SET n.node_class = 'Locale';

MATCH (n:Slugification) WHERE n.node_class IS NULL
SET n.node_class = 'Slugification';

MATCH (n:Formatting) WHERE n.node_class IS NULL
SET n.node_class = 'Formatting';

MATCH (n:Culture) WHERE n.node_class IS NULL
SET n.node_class = 'Culture';

MATCH (n:Adaptation) WHERE n.node_class IS NULL
SET n.node_class = 'Adaptation';

MATCH (n:Style) WHERE n.node_class IS NULL
SET n.node_class = 'Style';

MATCH (n:LanguageFamily) WHERE n.node_class IS NULL
SET n.node_class = 'LanguageFamily';

MATCH (n:LanguageBranch) WHERE n.node_class IS NULL
SET n.node_class = 'LanguageBranch';

MATCH (n:CulturalRealm) WHERE n.node_class IS NULL
SET n.node_class = 'CulturalRealm';

MATCH (n:CulturalSubRealm) WHERE n.node_class IS NULL
SET n.node_class = 'CulturalSubRealm';

// -----------------------------------------------------------------------------
// SHARED REALM: Geography Layer
// -----------------------------------------------------------------------------

MATCH (n:Country) WHERE n.node_class IS NULL
SET n.node_class = 'Country';

MATCH (n:Continent) WHERE n.node_class IS NULL
SET n.node_class = 'Continent';

MATCH (n:GeoRegion) WHERE n.node_class IS NULL
SET n.node_class = 'GeoRegion';

MATCH (n:GeoSubRegion) WHERE n.node_class IS NULL
SET n.node_class = 'GeoSubRegion';

MATCH (n:PopulationCluster) WHERE n.node_class IS NULL
SET n.node_class = 'PopulationCluster';

MATCH (n:PopulationSubCluster) WHERE n.node_class IS NULL
SET n.node_class = 'PopulationSubCluster';

MATCH (n:EconomicRegion) WHERE n.node_class IS NULL
SET n.node_class = 'EconomicRegion';

MATCH (n:IncomeGroup) WHERE n.node_class IS NULL
SET n.node_class = 'IncomeGroup';

MATCH (n:LendingCategory) WHERE n.node_class IS NULL
SET n.node_class = 'LendingCategory';

// -----------------------------------------------------------------------------
// ORG REALM: Semantic Layer
// -----------------------------------------------------------------------------

MATCH (n:EntityNative) WHERE n.node_class IS NULL
SET n.node_class = 'EntityNative';

MATCH (n:EntityCategory) WHERE n.node_class IS NULL
SET n.node_class = 'EntityCategory';

// -----------------------------------------------------------------------------
// ORG REALM: Structure Layer
// -----------------------------------------------------------------------------

MATCH (n:BlockType) WHERE n.node_class IS NULL
SET n.node_class = 'BlockType';

MATCH (n:ContentSlot) WHERE n.node_class IS NULL
SET n.node_class = 'ContentSlot';

// -----------------------------------------------------------------------------
// ORG REALM: Output Layer
// -----------------------------------------------------------------------------

MATCH (n:PageNative) WHERE n.node_class IS NULL
SET n.node_class = 'PageNative';

MATCH (n:BlockNative) WHERE n.node_class IS NULL
SET n.node_class = 'BlockNative';

// -----------------------------------------------------------------------------
// ORG REALM: Foundation Layer (SEO/GEO)
// -----------------------------------------------------------------------------

MATCH (n:ProjectSEOScope) WHERE n.node_class IS NULL
SET n.node_class = 'ProjectSEOScope';

MATCH (n:ProjectGEOScope) WHERE n.node_class IS NULL
SET n.node_class = 'ProjectGEOScope';

MATCH (n:SEOKeywordFormat) WHERE n.node_class IS NULL
SET n.node_class = 'SEOKeywordFormat';

MATCH (n:GEOQuerySet) WHERE n.node_class IS NULL
SET n.node_class = 'GEOQuerySet';

MATCH (n:GEOQuery) WHERE n.node_class IS NULL
SET n.node_class = 'GEOQuery';

MATCH (n:GEOAnswer) WHERE n.node_class IS NULL
SET n.node_class = 'GEOAnswer';

MATCH (n:Brand) WHERE n.node_class IS NULL
SET n.node_class = 'Brand';

MATCH (n:BrandPrinciples) WHERE n.node_class IS NULL
SET n.node_class = 'BrandPrinciples';

// -----------------------------------------------------------------------------
// ORG REALM: Instruction Layer
// -----------------------------------------------------------------------------

MATCH (n:PromptStyle) WHERE n.node_class IS NULL
SET n.node_class = 'PromptStyle';

MATCH (n:PromptArtifact) WHERE n.node_class IS NULL
SET n.node_class = 'PromptArtifact';

MATCH (n:OutputArtifact) WHERE n.node_class IS NULL
SET n.node_class = 'OutputArtifact';

// -----------------------------------------------------------------------------
// Verification query (run manually):
// MATCH (n) WHERE n.node_class IS NULL AND NOT n:Schema
// RETURN labels(n)[0] as label, count(*) as cnt
// ORDER BY cnt DESC;
// Expected: 0 results after migration
// -----------------------------------------------------------------------------
