// packages/db/seed/43-seo-keyword-format-classification.cypher
// v0.13.0 - SEOKeyword format classification (multiple formats per keyword)
//
// ADR-032: URL Slugification - format drives content strategy
//
// SEOKeywordFormat types:
//   standard    - Direct product/feature keywords
//   question    - Question-format (how to, what is, pourquoi, comment)
//   comparison  - Comparison keywords (vs, versus, alternative, meilleur)
//   preposition - Use-case keywords (for, pour, with, avec)
//   long_tail   - Multi-word niche keywords (4+ words)
//   brand       - Contains brand names
//   local       - Geographic modifiers

// ============================================================================
// 0. CLEANUP - Remove existing HAS_FORMAT relationships
// ============================================================================
// We'll rebuild all classifications from scratch for accuracy

MATCH ()-[r:HAS_FORMAT]->()
DELETE r;

// ============================================================================
// 1. QUESTION FORMAT - How-to, what-is, informational intent
// ============================================================================

MATCH (k:SEOKeyword), (f:SEOKeywordFormat {key: 'question'})
WHERE k.key CONTAINS 'how-to'
   OR k.key CONTAINS 'what-is'
   OR k.key CONTAINS 'why-'
   OR k.key CONTAINS 'can-'
   OR k.key CONTAINS 'does-'
   OR k.key CONTAINS 'is-'
   OR k.key STARTS WITH 'seo-how-'
   OR k.key STARTS WITH 'seo-what-'
   OR k.key STARTS WITH 'seo-why-'
   OR k.key CONTAINS 'comment-'
   OR k.key CONTAINS 'pourquoi-'
   OR k.key CONTAINS 'quest-ce-'
   OR k.key CONTAINS 'est-ce-'
MERGE (k)-[:HAS_FORMAT]->(f);

// ============================================================================
// 2. COMPARISON FORMAT - vs, alternative, best, meilleur
// ============================================================================

MATCH (k:SEOKeyword), (f:SEOKeywordFormat {key: 'comparison'})
WHERE k.key CONTAINS '-vs-'
   OR k.key CONTAINS '-versus-'
   OR k.key CONTAINS 'alternative'
   OR k.key CONTAINS 'meilleur'
   OR k.key CONTAINS 'best-'
   OR k.key CONTAINS 'top-'
   OR k.key CONTAINS 'compare'
   OR k.key CONTAINS 'comparison'
   OR k.key CONTAINS 'difference'
MERGE (k)-[:HAS_FORMAT]->(f);

// ============================================================================
// 3. PREPOSITION FORMAT - for, pour, with, avec (use-case)
// ============================================================================

MATCH (k:SEOKeyword), (f:SEOKeywordFormat {key: 'preposition'})
WHERE k.key CONTAINS '-for-'
   OR k.key CONTAINS '-pour-'
   OR k.key CONTAINS '-with-'
   OR k.key CONTAINS '-avec-'
   OR k.key CONTAINS '-to-'
   OR k.key CONTAINS '-de-'
   OR k.key CONTAINS '-sur-'
   OR k.key CONTAINS '-en-'
MERGE (k)-[:HAS_FORMAT]->(f);

// ============================================================================
// 4. BRAND FORMAT - Contains brand names
// ============================================================================

MATCH (k:SEOKeyword), (f:SEOKeywordFormat {key: 'brand'})
WHERE k.key CONTAINS 'instagram'
   OR k.key CONTAINS 'facebook'
   OR k.key CONTAINS 'tiktok'
   OR k.key CONTAINS 'youtube'
   OR k.key CONTAINS 'linkedin'
   OR k.key CONTAINS 'twitter'
   OR k.key CONTAINS 'whatsapp'
   OR k.key CONTAINS 'telegram'
   OR k.key CONTAINS 'snapchat'
   OR k.key CONTAINS 'pinterest'
   OR k.key CONTAINS 'paypal'
   OR k.key CONTAINS 'stripe'
   OR k.key CONTAINS 'bitcoin'
   OR k.key CONTAINS 'google'
   OR k.key CONTAINS 'apple'
   OR k.key CONTAINS 'spotify'
   OR k.key CONTAINS 'netflix'
   OR k.key CONTAINS 'amazon'
   OR k.key CONTAINS 'shopify'
   OR k.key CONTAINS 'canva'
   OR k.key CONTAINS 'mailchimp'
MERGE (k)-[:HAS_FORMAT]->(f);

// ============================================================================
// 5. LOCAL FORMAT - Geographic modifiers
// ============================================================================

MATCH (k:SEOKeyword), (f:SEOKeywordFormat {key: 'local'})
WHERE k.key CONTAINS 'france'
   OR k.key CONTAINS 'paris'
   OR k.key CONTAINS 'usa'
   OR k.key CONTAINS 'london'
   OR k.key CONTAINS 'canada'
   OR k.key CONTAINS 'belgique'
   OR k.key CONTAINS 'suisse'
   OR k.key CONTAINS 'quebec'
   OR k.key CONTAINS 'montreal'
   OR k.key CONTAINS 'near-me'
   OR k.key CONTAINS 'pres-de'
   OR k.key CONTAINS 'local'
MERGE (k)-[:HAS_FORMAT]->(f);

// ============================================================================
// 6. LONG_TAIL FORMAT - 4+ word keywords (niche, specific intent)
// ============================================================================
// Count hyphens in key (proxy for word count in slugified format)

MATCH (k:SEOKeyword), (f:SEOKeywordFormat {key: 'long_tail'})
WHERE size(split(k.key, '-')) >= 6  // seo-word1-word2-word3-word4 = 5+ parts
   OR k.key CONTAINS 'gratuit-en-ligne'
   OR k.key CONTAINS 'free-online'
   OR k.key CONTAINS 'sans-inscription'
   OR k.key CONTAINS 'without-registration'
   OR k.key CONTAINS 'step-by-step'
   OR k.key CONTAINS 'etape-par-etape'
MERGE (k)-[:HAS_FORMAT]->(f);

// ============================================================================
// 7. STANDARD FORMAT - Default for remaining keywords
// ============================================================================
// Keywords without any format yet get standard

MATCH (k:SEOKeyword), (f:SEOKeywordFormat {key: 'standard'})
WHERE NOT (k)-[:HAS_FORMAT]->(:SEOKeywordFormat)
MERGE (k)-[:HAS_FORMAT]->(f);

// ============================================================================
// 8. SPECIAL COMBINATIONS - Ensure multi-format accuracy
// ============================================================================

// Question + Long-tail (long questions)
MATCH (k:SEOKeyword)-[:HAS_FORMAT]->(:SEOKeywordFormat {key: 'question'})
MATCH (f:SEOKeywordFormat {key: 'long_tail'})
WHERE size(split(k.key, '-')) >= 7
MERGE (k)-[:HAS_FORMAT]->(f);

// Brand + Preposition (e.g., "qr code for instagram")
MATCH (k:SEOKeyword)-[:HAS_FORMAT]->(:SEOKeywordFormat {key: 'brand'})
MATCH (f:SEOKeywordFormat {key: 'preposition'})
WHERE k.key CONTAINS '-for-' OR k.key CONTAINS '-pour-'
MERGE (k)-[:HAS_FORMAT]->(f);

// Comparison + Brand (e.g., "qrcode-ai vs qr-monkey")
MATCH (k:SEOKeyword)-[:HAS_FORMAT]->(:SEOKeywordFormat {key: 'comparison'})
MATCH (f:SEOKeywordFormat {key: 'brand'})
WHERE k.key CONTAINS 'qrcode' OR k.key CONTAINS 'bitly' OR k.key CONTAINS 'flowcode'
MERGE (k)-[:HAS_FORMAT]->(f);

// ============================================================================
// VERIFICATION QUERY
// ============================================================================
// Run this to verify format distribution:
//
// MATCH (k:SEOKeyword)-[:HAS_FORMAT]->(f:SEOKeywordFormat)
// RETURN f.key AS format, count(k) AS keywords
// ORDER BY keywords DESC;
//
// Multi-format keywords:
// MATCH (k:SEOKeyword)-[:HAS_FORMAT]->(f:SEOKeywordFormat)
// WITH k, collect(f.key) AS formats
// WHERE size(formats) > 1
// RETURN k.key, formats
// LIMIT 20;
//
