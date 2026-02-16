// packages/db/seed/44-seo-keyword-metrics.cypher
// v0.13.0 - SEOKeywordMetrics time-series snapshots
//
// ADR-032: URL Slugification - metrics inform content strategy
//
// Creates 3 monthly snapshots per keyword (Dec 2025, Jan 2026, Feb 2026)
// Volume variations: ±5% monthly (realistic market fluctuation)
// Difficulty: Stable (changes slowly)
// Source: ahrefs (simulated data)

// ============================================================================
// 0. CLEANUP - Remove existing SEOKeywordMetrics
// ============================================================================

MATCH (m:SEOKeywordMetrics)
DETACH DELETE m;

// ============================================================================
// 1. HIGH-VOLUME KEYWORDS (>10K) - 3 snapshots each
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.volume >= 10000
WITH k, k.volume AS base_vol, k.difficulty AS diff

// December 2025 snapshot
CREATE (m1:SEOKeywordMetrics {
  key: "metrics-" + k.key + "-2025-12",
  display_name: k.key + " - Dec 2025",
  observed_at: datetime("2025-12-15T10:00:00Z"),
  source: "ahrefs",
  volume: toInteger(base_vol * 0.95),  // 5% lower (pre-holiday)
  difficulty: CASE WHEN diff IS NOT NULL THEN diff ELSE toInteger(rand() * 40 + 30) END,
  cpc: round(rand() * 2 + 0.5, 2),
  clicks: toInteger(base_vol * 0.3),
  traffic_potential: toInteger(base_vol * 0.25),
  created_at: datetime()
})

// January 2026 snapshot
CREATE (m2:SEOKeywordMetrics {
  key: "metrics-" + k.key + "-2026-01",
  display_name: k.key + " - Jan 2026",
  observed_at: datetime("2026-01-15T10:00:00Z"),
  source: "ahrefs",
  volume: toInteger(base_vol * 1.02),  // 2% higher (new year)
  difficulty: CASE WHEN diff IS NOT NULL THEN diff ELSE toInteger(rand() * 40 + 30) END,
  cpc: round(rand() * 2 + 0.5, 2),
  clicks: toInteger(base_vol * 0.32),
  traffic_potential: toInteger(base_vol * 0.27),
  created_at: datetime()
})

// February 2026 snapshot (current)
CREATE (m3:SEOKeywordMetrics {
  key: "metrics-" + k.key + "-2026-02",
  display_name: k.key + " - Feb 2026",
  observed_at: datetime("2026-02-15T10:00:00Z"),
  source: "ahrefs",
  volume: base_vol,  // Current volume
  difficulty: CASE WHEN diff IS NOT NULL THEN diff ELSE toInteger(rand() * 40 + 30) END,
  cpc: round(rand() * 2 + 0.5, 2),
  clicks: toInteger(base_vol * 0.31),
  traffic_potential: toInteger(base_vol * 0.26),
  created_at: datetime()
})

// Create HAS_METRICS relationships
MERGE (k)-[:HAS_METRICS]->(m1)
MERGE (k)-[:HAS_METRICS]->(m2)
MERGE (k)-[:HAS_METRICS]->(m3);

// ============================================================================
// 2. MEDIUM-VOLUME KEYWORDS (1K-10K) - 2 snapshots each
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.volume >= 1000 AND k.volume < 10000
WITH k, k.volume AS base_vol, k.difficulty AS diff

// January 2026 snapshot
CREATE (m1:SEOKeywordMetrics {
  key: "metrics-" + k.key + "-2026-01",
  display_name: k.key + " - Jan 2026",
  observed_at: datetime("2026-01-15T10:00:00Z"),
  source: "ahrefs",
  volume: toInteger(base_vol * 0.97),
  difficulty: CASE WHEN diff IS NOT NULL THEN diff ELSE toInteger(rand() * 35 + 25) END,
  cpc: round(rand() * 1.5 + 0.3, 2),
  clicks: toInteger(base_vol * 0.25),
  traffic_potential: toInteger(base_vol * 0.20),
  created_at: datetime()
})

// February 2026 snapshot (current)
CREATE (m2:SEOKeywordMetrics {
  key: "metrics-" + k.key + "-2026-02",
  display_name: k.key + " - Feb 2026",
  observed_at: datetime("2026-02-15T10:00:00Z"),
  source: "ahrefs",
  volume: base_vol,
  difficulty: CASE WHEN diff IS NOT NULL THEN diff ELSE toInteger(rand() * 35 + 25) END,
  cpc: round(rand() * 1.5 + 0.3, 2),
  clicks: toInteger(base_vol * 0.26),
  traffic_potential: toInteger(base_vol * 0.21),
  created_at: datetime()
})

MERGE (k)-[:HAS_METRICS]->(m1)
MERGE (k)-[:HAS_METRICS]->(m2);

// ============================================================================
// 3. LOW-VOLUME KEYWORDS (<1K) - 1 snapshot each (current only)
// ============================================================================

MATCH (k:SEOKeyword)
WHERE k.volume < 1000 AND k.volume IS NOT NULL
WITH k, k.volume AS base_vol, k.difficulty AS diff

// February 2026 snapshot only
CREATE (m:SEOKeywordMetrics {
  key: "metrics-" + k.key + "-2026-02",
  display_name: k.key + " - Feb 2026",
  observed_at: datetime("2026-02-15T10:00:00Z"),
  source: "ahrefs",
  volume: base_vol,
  difficulty: CASE WHEN diff IS NOT NULL THEN diff ELSE toInteger(rand() * 30 + 20) END,
  cpc: round(rand() * 1.0 + 0.2, 2),
  clicks: toInteger(base_vol * 0.20),
  traffic_potential: toInteger(base_vol * 0.15),
  created_at: datetime()
})

MERGE (k)-[:HAS_METRICS]->(m);

// ============================================================================
// 4. RISING TREND KEYWORDS - Boost recent volume
// ============================================================================
// Keywords with trend = "rising" should show volume increase

MATCH (k:SEOKeyword {trend: "rising"})-[:HAS_METRICS]->(m:SEOKeywordMetrics)
WHERE m.observed_at > datetime("2026-01-01")
SET m.volume = toInteger(m.volume * 1.15),  // 15% boost for rising
    m.clicks = toInteger(m.clicks * 1.12),
    m.traffic_potential = toInteger(m.traffic_potential * 1.10);

// ============================================================================
// 5. ADD POSITION DATA FOR TOP KEYWORDS
// ============================================================================
// Simulate our ranking positions for high-value keywords

MATCH (k:SEOKeyword)-[:HAS_METRICS]->(m:SEOKeywordMetrics)
WHERE k.volume >= 10000
  AND m.observed_at >= datetime("2026-02-01")
SET m.position = round(rand() * 15 + 5, 1),  // Position 5-20
    m.best_position = toInteger(rand() * 10 + 3),  // Best 3-13
    m.url = "/" +
      CASE
        WHEN k.locale = "fr-FR" THEN "fr/"
        WHEN k.locale = "en-US" THEN "en/"
        ELSE ""
      END +
      CASE
        WHEN k.key CONTAINS "generator" OR k.key CONTAINS "generateur" THEN "qr-code-generator"
        WHEN k.key CONTAINS "scanner" THEN "qr-code-scanner"
        WHEN k.key CONTAINS "wifi" THEN "qr-code-wifi"
        WHEN k.key CONTAINS "instagram" THEN "qr-code-instagram"
        WHEN k.key CONTAINS "menu" THEN "qr-code-menu"
        ELSE "qr-code"
      END;

// ============================================================================
// VERIFICATION QUERIES
// ============================================================================
// Run these to verify SEOKeywordMetrics:
//
// -- Count metrics by month
// MATCH (m:SEOKeywordMetrics)
// RETURN substring(toString(m.observed_at), 0, 7) AS month, count(m) AS metrics
// ORDER BY month;
//
// -- Keywords with time-series data
// MATCH (k:SEOKeyword)-[:HAS_METRICS]->(m:SEOKeywordMetrics)
// WITH k, count(m) AS snapshots
// RETURN snapshots, count(k) AS keywords
// ORDER BY snapshots DESC;
//
// -- Sample metrics history
// MATCH (k:SEOKeyword {key: "seo-qr-code"})-[:HAS_METRICS]->(m:SEOKeywordMetrics)
// RETURN m.observed_at, m.volume, m.difficulty, m.position
// ORDER BY m.observed_at;
//
