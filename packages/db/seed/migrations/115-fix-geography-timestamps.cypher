// ============================================================================
// Migration 115: Fix missing updated_at on Geography nodes
// ============================================================================
// Some geography nodes from early seeds are missing updated_at timestamp.
// This migration adds updated_at = created_at for consistency.
// ============================================================================

// --- CulturalSubRealm ---
MATCH (n:CulturalSubRealm)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- PopulationSubCluster ---
MATCH (n:PopulationSubCluster)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- GeoRegion ---
MATCH (n:GeoRegion)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- GeoSubRegion ---
MATCH (n:GeoSubRegion)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- LanguageFamily ---
MATCH (n:LanguageFamily)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- LanguageBranch ---
MATCH (n:LanguageBranch)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- EconomicRegion ---
MATCH (n:EconomicRegion)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- PopulationCluster ---
MATCH (n:PopulationCluster)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- Continent ---
MATCH (n:Continent)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- CulturalRealm ---
MATCH (n:CulturalRealm)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- IncomeGroup ---
MATCH (n:IncomeGroup)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- LendingCategory ---
MATCH (n:LendingCategory)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- SEOKeywordFormat ---
MATCH (n:SEOKeywordFormat)
WHERE n.updated_at IS NULL AND n.created_at IS NOT NULL
SET n.updated_at = n.created_at;

// --- Verification ---
// MATCH (n) WHERE n.updated_at IS NULL AND NOT n:Schema RETURN labels(n)[0] AS class, count(*) AS missing;
// Should return 0 for all geography classes.
