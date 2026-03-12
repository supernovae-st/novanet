// packages/db/seed/40-popular-in-geographic.cypher
// v0.13.0 - POPULAR_IN arcs: SEOKeyword geographic popularity
//
// ADR-032: URL Slugification - geographic targeting for localized content
//
// POPULAR_IN links SEOKeyword to Country/GeoRegion where it has high volume.
// This enables:
// 1. Geographic content targeting (fr-FR keywords popular in France)
// 2. Regional SEO strategy (different keywords for different markets)
// 3. Locale-specific slug derivation

// ============================================================================
// 1. FRENCH KEYWORDS → FRANCE
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.locale = 'fr-FR'
   OR k.key CONTAINS '-fr-fr-'
MATCH (c:Country {key: 'FR'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.95,
    r.source = 'locale_inference',
    r.llm_context = 'French keyword is popular in France market',
    r.created_at = datetime();

// ============================================================================
// 2. FRENCH KEYWORDS → FRANCOPHONE COUNTRIES
// ============================================================================
// Belgian French, Swiss French, Canadian French, African Francophone

MATCH (k:SEOKeyword)
WHERE k.locale = 'fr-FR'
   OR k.key CONTAINS '-fr-fr-'
MATCH (c:Country)
WHERE c.key IN ['BE', 'CH', 'CA', 'LU', 'MC']
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.70,
    r.source = 'francophone_inference',
    r.llm_context = 'French keyword likely popular in Francophone markets',
    r.created_at = datetime();

// African Francophone (lower confidence due to different search patterns)
MATCH (k:SEOKeyword)
WHERE k.locale = 'fr-FR'
   OR k.key CONTAINS '-fr-fr-'
MATCH (c:Country)
WHERE c.key IN ['SN', 'CI', 'ML', 'BF', 'NE', 'TG', 'BJ', 'CM', 'CD', 'CG', 'GA', 'MG', 'MA', 'DZ', 'TN']
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.50,
    r.source = 'francophone_africa_inference',
    r.llm_context = 'French keyword may be popular in African Francophone markets',
    r.created_at = datetime();

// ============================================================================
// 3. ENGLISH KEYWORDS → ANGLOPHONE REGIONS
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.locale = 'en-US'
   OR k.key CONTAINS '-en-us-'
MATCH (c:Country {key: 'US'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.95,
    r.source = 'locale_inference',
    r.llm_context = 'US English keyword popular in United States market',
    r.created_at = datetime();

// UK English
MATCH (k:SEOKeyword)
WHERE k.locale = 'en-GB'
   OR k.key CONTAINS '-en-gb-'
MATCH (c:Country {key: 'GB'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.95,
    r.source = 'locale_inference',
    r.llm_context = 'UK English keyword popular in United Kingdom market',
    r.created_at = datetime();

// ============================================================================
// 4. SPANISH KEYWORDS → HISPANIC COUNTRIES
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.locale STARTS WITH 'es-'
   OR k.key CONTAINS '-es-es-'
   OR k.key CONTAINS '-es-mx-'
MATCH (c:Country {key: 'ES'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.90,
    r.source = 'locale_inference',
    r.llm_context = 'Spanish keyword popular in Spain market',
    r.created_at = datetime();

// Latin America
MATCH (k:SEOKeyword)
WHERE k.locale STARTS WITH 'es-'
MATCH (c:Country)
WHERE c.key IN ['MX', 'AR', 'CO', 'PE', 'VE', 'CL', 'EC', 'GT', 'CU', 'BO', 'DO', 'HN', 'PY', 'SV', 'NI', 'CR', 'PA', 'UY']
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.70,
    r.source = 'hispanic_inference',
    r.llm_context = 'Spanish keyword likely popular in Latin American markets',
    r.created_at = datetime();

// ============================================================================
// 5. GERMAN KEYWORDS → DACH REGION
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.locale = 'de-DE'
   OR k.key CONTAINS '-de-de-'
MATCH (c:Country)
WHERE c.key IN ['DE', 'AT', 'CH']
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.90,
    r.source = 'dach_inference',
    r.llm_context = 'German keyword popular in DACH region (Germany, Austria, Switzerland)',
    r.created_at = datetime();

// ============================================================================
// 6. PORTUGUESE KEYWORDS → LUSOPHONE COUNTRIES
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.locale = 'pt-BR'
   OR k.key CONTAINS '-pt-br-'
MATCH (c:Country {key: 'BR'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.95,
    r.source = 'locale_inference',
    r.llm_context = 'Brazilian Portuguese keyword popular in Brazil',
    r.created_at = datetime();

MATCH (k:SEOKeyword)
WHERE k.locale = 'pt-PT'
   OR k.key CONTAINS '-pt-pt-'
MATCH (c:Country {key: 'PT'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.95,
    r.source = 'locale_inference',
    r.llm_context = 'European Portuguese keyword popular in Portugal',
    r.created_at = datetime();

// ============================================================================
// 7. ASIAN LANGUAGE KEYWORDS
// ============================================================================

// Japanese
MATCH (k:SEOKeyword)
WHERE k.locale = 'ja-JP'
   OR k.key CONTAINS '-ja-jp-'
MATCH (c:Country {key: 'JP'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.98,
    r.source = 'locale_inference',
    r.llm_context = 'Japanese keyword exclusively popular in Japan',
    r.created_at = datetime();

// Korean
MATCH (k:SEOKeyword)
WHERE k.locale = 'ko-KR'
   OR k.key CONTAINS '-ko-kr-'
MATCH (c:Country {key: 'KR'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.98,
    r.source = 'locale_inference',
    r.llm_context = 'Korean keyword exclusively popular in South Korea',
    r.created_at = datetime();

// Chinese (Simplified → China, Traditional → Taiwan/HK)
MATCH (k:SEOKeyword)
WHERE k.locale = 'zh-CN'
   OR k.key CONTAINS '-zh-cn-'
MATCH (c:Country {key: 'CN'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.95,
    r.source = 'locale_inference',
    r.llm_context = 'Simplified Chinese keyword popular in China',
    r.created_at = datetime();

MATCH (k:SEOKeyword)
WHERE k.locale = 'zh-TW'
   OR k.key CONTAINS '-zh-tw-'
MATCH (c:Country {key: 'TW'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.95,
    r.source = 'locale_inference',
    r.llm_context = 'Traditional Chinese keyword popular in Taiwan',
    r.created_at = datetime();

// ============================================================================
// 8. ITALIAN KEYWORDS → ITALY
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.locale = 'it-IT'
   OR k.key CONTAINS '-it-it-'
MATCH (c:Country {key: 'IT'})
MERGE (k)-[r:POPULAR_IN]->(c)
SET r.confidence = 0.95,
    r.source = 'locale_inference',
    r.llm_context = 'Italian keyword popular in Italy market',
    r.created_at = datetime();

// ============================================================================
// 9. VERIFICATION QUERY
// ============================================================================
// Run this to verify POPULAR_IN arcs:
//
// MATCH (k:SEOKeyword)-[r:POPULAR_IN]->(c:Country)
// RETURN c.key AS country, c.name, count(k) AS keywords, round(avg(r.confidence) * 100) / 100 AS avg_confidence
// ORDER BY keywords DESC
// LIMIT 15;
//
