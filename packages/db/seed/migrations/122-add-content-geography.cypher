// ============================================================================
// Migration 122: Add content to Geography nodes
// ============================================================================
// Geography nodes have deterministic content based on their properties.
// Template: "{display_name} is a {type_description}."
// ============================================================================

// --- Continent ---
MATCH (n:Continent)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is one of the seven major landmasses on Earth.';

// --- CulturalRealm ---
MATCH (n:CulturalRealm)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a major cultural region sharing historical, linguistic, and social characteristics.';

// --- CulturalSubRealm ---
MATCH (n:CulturalSubRealm)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a cultural sub-region with distinct traditions and practices.';

// --- PopulationCluster ---
MATCH (n:PopulationCluster)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a demographic grouping based on shared characteristics.';

// --- PopulationSubCluster ---
MATCH (n:PopulationSubCluster)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a specific demographic sub-group.';

// --- GeoRegion ---
MATCH (n:GeoRegion)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a geographic region as defined by the UN geoscheme.';

// --- GeoSubRegion ---
MATCH (n:GeoSubRegion)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a geographic sub-region.';

// --- EconomicRegion ---
MATCH (n:EconomicRegion)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is an economic bloc or trade region.';

// --- IncomeGroup ---
MATCH (n:IncomeGroup)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a World Bank income classification.';

// --- LendingCategory ---
MATCH (n:LendingCategory)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a World Bank lending classification.';

// --- LanguageFamily ---
MATCH (n:LanguageFamily)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a language family grouping related languages by common ancestry.';

// --- LanguageBranch ---
MATCH (n:LanguageBranch)
WHERE n.content IS NULL
SET n.content = coalesce(n.display_name, n.name) + ' is a branch within a language family.';

// --- Verification ---
// MATCH (n) WHERE labels(n)[0] IN ['Continent', 'CulturalRealm', 'CulturalSubRealm',
//   'PopulationCluster', 'PopulationSubCluster', 'GeoRegion', 'GeoSubRegion',
//   'EconomicRegion', 'IncomeGroup', 'LendingCategory', 'LanguageFamily', 'LanguageBranch']
//   AND n.content IS NULL
// RETURN labels(n)[0], count(n);
// Should return 0
