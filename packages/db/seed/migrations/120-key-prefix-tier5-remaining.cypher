// ============================================================================
// Migration 120: Key Prefix - Tier 5 (Remaining nodes)
// ============================================================================
// Geography, Config, and Org nodes that still need prefixes.
// ============================================================================

// --- Geography nodes ---

// CulturalRealm: cultrealm:{code}
MATCH (n:CulturalRealm)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'cultrealm:'
SET n.key = 'cultrealm:' + toLower(replace(n.key, ' ', '-'));

// CulturalSubRealm: cultsubrealm:{code}
MATCH (n:CulturalSubRealm)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'cultsubrealm:'
SET n.key = 'cultsubrealm:' + toLower(replace(n.key, ' ', '-'));

// PopulationCluster: popcluster:{code}
MATCH (n:PopulationCluster)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'popcluster:'
SET n.key = 'popcluster:' + toLower(replace(n.key, ' ', '-'));

// PopulationSubCluster: popsubcluster:{code}
MATCH (n:PopulationSubCluster)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'popsubcluster:'
SET n.key = 'popsubcluster:' + toLower(replace(n.key, ' ', '-'));

// GeoRegion: georegion:{code}
MATCH (n:GeoRegion)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'georegion:'
SET n.key = 'georegion:' + toLower(replace(n.key, ' ', '-'));

// GeoSubRegion: geosubregion:{code}
MATCH (n:GeoSubRegion)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'geosubregion:'
SET n.key = 'geosubregion:' + toLower(replace(n.key, ' ', '-'));

// EconomicRegion: econregion:{code}
MATCH (n:EconomicRegion)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'econregion:'
SET n.key = 'econregion:' + toLower(replace(n.key, ' ', '-'));

// IncomeGroup: incomegroup:{code}
MATCH (n:IncomeGroup)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'incomegroup:'
SET n.key = 'incomegroup:' + toLower(replace(n.key, ' ', '-'));

// LendingCategory: lendingcat:{code}
MATCH (n:LendingCategory)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'lendingcat:'
SET n.key = 'lendingcat:' + toLower(replace(n.key, ' ', '-'));

// --- Config nodes ---

// EntityCategory: category:{slug}
MATCH (n:EntityCategory)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'category:'
SET n.key = 'category:' + toLower(replace(n.key, ' ', '-'));

// BlockType: blocktype:{slug}
MATCH (n:BlockType)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'blocktype:'
SET n.key = 'blocktype:' + toLower(replace(n.key, ' ', '-'));

// SEOKeywordFormat: seoformat:{slug}
MATCH (n:SEOKeywordFormat)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'seoformat:'
SET n.key = 'seoformat:' + toLower(replace(n.key, ' ', '-'));

// OrgConfig: orgconfig:{org}
MATCH (n:OrgConfig)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'orgconfig:'
SET n.key = 'orgconfig:' + toLower(replace(n.key, ' ', '-'));

// --- Org Foundation nodes ---

// Project: project:{slug}
MATCH (n:Project)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'project:'
SET n.key = 'project:' + toLower(replace(n.key, ' ', '-'));

// ProjectNative: project:{slug}@{locale}
MATCH (n:ProjectNative)-[:FOR_LOCALE]->(l:Locale)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'project:'
WITH n, l.key AS localeKey
SET n.key = 'project:' + toLower(replace(split(n.key, '@')[0], ' ', '-')) + '@' + replace(localeKey, 'locale:', '');

MATCH (n:ProjectNative)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'project:'
SET n.key = 'project:' + toLower(replace(n.key, ' ', '-'));

// Brand: brand:{slug}
MATCH (n:Brand)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'brand:'
SET n.key = 'brand:' + toLower(replace(n.key, ' ', '-'));

// BrandDesign: branddesign:{brand}
MATCH (n:BrandDesign)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'branddesign:'
SET n.key = 'branddesign:' + toLower(replace(n.key, ' ', '-'));

// ProjectSEOScope: seoscope:{project}
MATCH (n:ProjectSEOScope)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'seoscope:'
SET n.key = 'seoscope:' + toLower(replace(n.key, ' ', '-'));

// ProjectGEOScope: geoscope:{project}
MATCH (n:ProjectGEOScope)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'geoscope:'
SET n.key = 'geoscope:' + toLower(replace(n.key, ' ', '-'));

// --- Verification ---
// MATCH (n) WHERE NOT n:Schema AND (n.key IS NULL OR NOT n.key CONTAINS ':')
// RETURN labels(n)[0], count(n) ORDER BY count(n) DESC;
// Should return empty or only Schema nodes.
