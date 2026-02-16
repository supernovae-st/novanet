// packages/db/seed/42-seo-keyword-trends.cypher
// v0.13.0 - SEOKeyword.trend property population
//
// ADR-032: URL Slugification - trend informs content strategy
//
// trend values: "rising" | "stable" | "declining"
// Based on realistic SEO market patterns for QR code industry (2024-2026)

// ============================================================================
// 1. HIGH-VOLUME CORE KEYWORDS = STABLE
// ============================================================================
// Mature keywords with established search volume (>50K monthly)

MATCH (k:SEOKeyword)
WHERE k.volume >= 50000
SET k.trend = "stable",
    k.trend_confidence = 0.90,
    k.trend_source = "volume_inference";

// ============================================================================
// 2. DYNAMIC QR CODE KEYWORDS = RISING
// ============================================================================
// Dynamic QR codes are growing in popularity (tracking, analytics)

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "dynamic"
   OR k.key CONTAINS "tracking"
   OR k.key CONTAINS "analytics"
   OR k.key CONTAINS "statistique"
   OR k.key CONTAINS "suivi"
SET k.trend = "rising",
    k.trend_confidence = 0.85,
    k.trend_source = "category_inference";

// ============================================================================
// 3. SOCIAL MEDIA QR CODES = RISING
// ============================================================================
// Social media integration is a growing trend

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "instagram"
   OR k.key CONTAINS "tiktok"
   OR k.key CONTAINS "youtube"
   OR k.key CONTAINS "facebook"
   OR k.key CONTAINS "linkedin"
   OR k.key CONTAINS "whatsapp"
SET k.trend = "rising",
    k.trend_confidence = 0.80,
    k.trend_source = "social_media_trend";

// ============================================================================
// 4. PAYMENT QR CODES = RISING
// ============================================================================
// Payment and contactless transactions growing fast

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "payment"
   OR k.key CONTAINS "paiement"
   OR k.key CONTAINS "paypal"
   OR k.key CONTAINS "stripe"
   OR k.key CONTAINS "crypto"
   OR k.key CONTAINS "bitcoin"
SET k.trend = "rising",
    k.trend_confidence = 0.85,
    k.trend_source = "fintech_trend";

// ============================================================================
// 5. MENU QR CODES = STABLE (post-COVID maturity)
// ============================================================================
// Restaurant menu QR codes exploded during COVID, now stable

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "menu"
   OR k.key CONTAINS "restaurant"
SET k.trend = "stable",
    k.trend_confidence = 0.85,
    k.trend_source = "post_covid_maturity";

// ============================================================================
// 6. VCARD QR CODES = STABLE
// ============================================================================
// Business cards are stable, not growing significantly

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "vcard"
   OR k.key CONTAINS "carte-de-visite"
   OR k.key CONTAINS "business-card"
SET k.trend = "stable",
    k.trend_confidence = 0.80,
    k.trend_source = "category_inference";

// ============================================================================
// 7. WIFI QR CODES = STABLE
// ============================================================================
// WiFi sharing is mature

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "wifi"
SET k.trend = "stable",
    k.trend_confidence = 0.85,
    k.trend_source = "category_inference";

// ============================================================================
// 8. API/DEVELOPER KEYWORDS = RISING
// ============================================================================
// API integration and automation growing

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "api"
   OR k.key CONTAINS "integration"
   OR k.key CONTAINS "automatisation"
   OR k.key CONTAINS "automation"
SET k.trend = "rising",
    k.trend_confidence = 0.80,
    k.trend_source = "developer_trend";

// ============================================================================
// 9. COMPARISON KEYWORDS = STABLE
// ============================================================================
// Comparison searches are always steady

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "-vs-"
   OR k.key CONTAINS "alternative"
   OR k.key CONTAINS "meilleur"
   OR k.key CONTAINS "best"
SET k.trend = "stable",
    k.trend_confidence = 0.75,
    k.trend_source = "comparison_pattern";

// ============================================================================
// 10. QUESTION KEYWORDS = RISING
// ============================================================================
// How-to and question queries growing with voice search

MATCH (k:SEOKeyword)
WHERE k.key STARTS WITH "seo-how-"
   OR k.key STARTS WITH "seo-what-"
   OR k.key STARTS WITH "seo-comment-"
   OR k.key STARTS WITH "seo-pourquoi-"
SET k.trend = "rising",
    k.trend_confidence = 0.75,
    k.trend_source = "voice_search_trend";

// ============================================================================
// 11. AI/INTELLIGENT QR CODES = RISING
// ============================================================================
// AI features are the latest trend

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "intelligent"
   OR k.key CONTAINS "ai"
   OR k.key CONTAINS "smart"
SET k.trend = "rising",
    k.trend_confidence = 0.90,
    k.trend_source = "ai_trend";

// ============================================================================
// 12. PRINT/PDF/DOWNLOAD = STABLE
// ============================================================================
// Traditional output formats are mature

MATCH (k:SEOKeyword)
WHERE k.key CONTAINS "print"
   OR k.key CONTAINS "imprimer"
   OR k.key CONTAINS "pdf"
   OR k.key CONTAINS "download"
   OR k.key CONTAINS "telecharger"
SET k.trend = "stable",
    k.trend_confidence = 0.80,
    k.trend_source = "output_format_mature";

// ============================================================================
// 13. DEFAULT: REMAINING = STABLE
// ============================================================================
// Any unclassified keywords default to stable

MATCH (k:SEOKeyword)
WHERE k.trend IS NULL
SET k.trend = "stable",
    k.trend_confidence = 0.60,
    k.trend_source = "default";

// ============================================================================
// VERIFICATION QUERY
// ============================================================================
// Run this to verify trend distribution:
//
// MATCH (k:SEOKeyword)
// RETURN k.trend AS trend, count(*) AS count,
//        round(avg(k.trend_confidence) * 100) / 100 AS avg_confidence
// ORDER BY count DESC;
//
