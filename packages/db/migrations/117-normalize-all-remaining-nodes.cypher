// ============================================================================
// Migration 117: Normalize ALL Remaining Nodes to v0.19.0 Standard
// ============================================================================
// Purpose: Add missing node_class property to all node types
// This is a comprehensive cleanup migration to achieve 100% CSR
// Executed: 2026-03-12
// ============================================================================

// === CONTAINER NODES ===

// CultureSet
MATCH (n:CultureSet) WHERE n.node_class IS NULL
SET n.node_class = 'CultureSet'
RETURN count(n) AS cultureset_fixed;

// AudienceSet
MATCH (n:AudienceSet) WHERE n.node_class IS NULL
SET n.node_class = 'AudienceSet'
RETURN count(n) AS audienceset_fixed;

// PatternSet
MATCH (n:PatternSet) WHERE n.node_class IS NULL
SET n.node_class = 'PatternSet'
RETURN count(n) AS patternset_fixed;

// TabooSet
MATCH (n:TabooSet) WHERE n.node_class IS NULL
SET n.node_class = 'TabooSet'
RETURN count(n) AS tabooset_fixed;

// ExpressionSet
MATCH (n:ExpressionSet) WHERE n.node_class IS NULL
SET n.node_class = 'ExpressionSet'
RETURN count(n) AS expressionset_fixed;

// === LANGUAGE NODES ===

// LanguageBranch
MATCH (n:LanguageBranch) WHERE n.node_class IS NULL
SET n.node_class = 'LanguageBranch'
RETURN count(n) AS languagebranch_fixed;

MATCH (n:LanguageBranch) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS languagebranch_updated_at_fixed;

// LanguageFamily
MATCH (n:LanguageFamily) WHERE n.node_class IS NULL
SET n.node_class = 'LanguageFamily'
RETURN count(n) AS languagefamily_fixed;

MATCH (n:LanguageFamily) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS languagefamily_updated_at_fixed;

// === GEOGRAPHIC NODES ===

// CulturalSubRealm
MATCH (n:CulturalSubRealm) WHERE n.node_class IS NULL
SET n.node_class = 'CulturalSubRealm'
RETURN count(n) AS culturalsubrealm_fixed;

MATCH (n:CulturalSubRealm) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS culturalsubrealm_updated_at_fixed;

// CulturalRealm
MATCH (n:CulturalRealm) WHERE n.node_class IS NULL
SET n.node_class = 'CulturalRealm'
RETURN count(n) AS culturalrealm_fixed;

MATCH (n:CulturalRealm) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS culturalrealm_updated_at_fixed;

// PopulationSubCluster
MATCH (n:PopulationSubCluster) WHERE n.node_class IS NULL
SET n.node_class = 'PopulationSubCluster'
RETURN count(n) AS populationsubcluster_fixed;

MATCH (n:PopulationSubCluster) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS populationsubcluster_updated_at_fixed;

// PopulationCluster
MATCH (n:PopulationCluster) WHERE n.node_class IS NULL
SET n.node_class = 'PopulationCluster'
RETURN count(n) AS populationcluster_fixed;

MATCH (n:PopulationCluster) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS populationcluster_updated_at_fixed;

// GeoRegion
MATCH (n:GeoRegion) WHERE n.node_class IS NULL
SET n.node_class = 'GeoRegion'
RETURN count(n) AS georegion_fixed;

MATCH (n:GeoRegion) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS georegion_updated_at_fixed;

// GeoSubRegion
MATCH (n:GeoSubRegion) WHERE n.node_class IS NULL
SET n.node_class = 'GeoSubRegion'
RETURN count(n) AS geosubregion_fixed;

MATCH (n:GeoSubRegion) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS geosubregion_updated_at_fixed;

// Continent
MATCH (n:Continent) WHERE n.node_class IS NULL
SET n.node_class = 'Continent'
RETURN count(n) AS continent_fixed;

MATCH (n:Continent) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS continent_updated_at_fixed;

// EconomicRegion
MATCH (n:EconomicRegion) WHERE n.node_class IS NULL
SET n.node_class = 'EconomicRegion'
RETURN count(n) AS economicregion_fixed;

MATCH (n:EconomicRegion) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS economicregion_updated_at_fixed;

// IncomeGroup
MATCH (n:IncomeGroup) WHERE n.node_class IS NULL
SET n.node_class = 'IncomeGroup'
RETURN count(n) AS incomegroup_fixed;

MATCH (n:IncomeGroup) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS incomegroup_updated_at_fixed;

// LendingCategory
MATCH (n:LendingCategory) WHERE n.node_class IS NULL
SET n.node_class = 'LendingCategory'
RETURN count(n) AS lendingcategory_fixed;

MATCH (n:LendingCategory) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS lendingcategory_updated_at_fixed;

// === CONTENT STRUCTURE NODES ===

// BlockType
MATCH (n:BlockType) WHERE n.node_class IS NULL
SET n.node_class = 'BlockType'
RETURN count(n) AS blocktype_fixed;

// SEOKeywordFormat
MATCH (n:SEOKeywordFormat) WHERE n.node_class IS NULL
SET n.node_class = 'SEOKeywordFormat'
RETURN count(n) AS seokeywordformat_fixed;

MATCH (n:SEOKeywordFormat) WHERE n.updated_at IS NULL
SET n.updated_at = datetime()
RETURN count(n) AS seokeywordformat_updated_at_fixed;

// ProjectSEOScope
MATCH (n:ProjectSEOScope) WHERE n.node_class IS NULL
SET n.node_class = 'ProjectSEOScope'
RETURN count(n) AS projectseoscope_fixed;

// ProjectGEOScope
MATCH (n:ProjectGEOScope) WHERE n.node_class IS NULL
SET n.node_class = 'ProjectGEOScope'
RETURN count(n) AS projectgeoscope_fixed;

// GEOQuerySet
MATCH (n:GEOQuerySet) WHERE n.node_class IS NULL
SET n.node_class = 'GEOQuerySet'
RETURN count(n) AS geoqueryset_fixed;

// GEOQuery
MATCH (n:GEOQuery) WHERE n.node_class IS NULL
SET n.node_class = 'GEOQuery'
RETURN count(n) AS geoquery_fixed;

// GEOAnswer
MATCH (n:GEOAnswer) WHERE n.node_class IS NULL
SET n.node_class = 'GEOAnswer'
RETURN count(n) AS geoanswer_fixed;

// ContentSlot
MATCH (n:ContentSlot) WHERE n.node_class IS NULL
SET n.node_class = 'ContentSlot'
RETURN count(n) AS contentslot_fixed;

// PromptArtifact
MATCH (n:PromptArtifact) WHERE n.node_class IS NULL
SET n.node_class = 'PromptArtifact'
RETURN count(n) AS promptartifact_fixed;

// OutputArtifact
MATCH (n:OutputArtifact) WHERE n.node_class IS NULL
SET n.node_class = 'OutputArtifact'
RETURN count(n) AS outputartifact_fixed;

// === VERIFICATION ===

// Check for any remaining nodes without node_class
MATCH (n)
WHERE NOT n:Schema AND NOT n:Realm AND NOT n:Layer AND NOT n:ArcFamily
  AND n.node_class IS NULL
WITH labels(n)[0] AS nodeType, count(*) AS cnt
RETURN nodeType, cnt
ORDER BY cnt DESC;

// CSR Summary
MATCH (n)
WHERE NOT n:Schema AND NOT n:Realm AND NOT n:Layer AND NOT n:ArcFamily
WITH count(*) AS total,
     sum(CASE WHEN n.node_class IS NOT NULL THEN 1 ELSE 0 END) AS with_node_class,
     sum(CASE WHEN n.created_at IS NOT NULL THEN 1 ELSE 0 END) AS with_created_at,
     sum(CASE WHEN n.updated_at IS NOT NULL THEN 1 ELSE 0 END) AS with_updated_at
RETURN total,
       with_node_class,
       round(100.0 * with_node_class / total, 2) AS node_class_pct,
       with_created_at,
       round(100.0 * with_created_at / total, 2) AS created_at_pct,
       with_updated_at,
       round(100.0 * with_updated_at / total, 2) AS updated_at_pct;
