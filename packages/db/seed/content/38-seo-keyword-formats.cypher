// packages/db/seed/38-seo-keyword-formats.cypher
// v0.13.0 - SEOKeywordFormat classification nodes
//
// ADR-032: URL Slugification - keyword formats influence content strategy
//
// SEOKeywordFormat classifies keywords by their structure/intent.
// This enables targeted content generation:
// - question → FAQ, how-to guides
// - comparison → vs pages, comparatifs
// - preposition → use-case landing pages
// - long_tail → niche content, specific intents

// ============================================================================
// 1. CREATE SEOKeywordFormat NODES (7 types)
// ============================================================================

// Standard keywords (direct product/feature terms)
MERGE (f:SEOKeywordFormat {key: 'standard'})
SET f.display_name = 'Standard',
    f.description = 'Direct product or feature keywords without modifiers',
    f.pattern = '^[a-z0-9\\-]+$',
    f.content_type = 'landing_page',
    f.examples = ['qr code generator', 'wifi qr code', 'barcode scanner'],
    f.llm_context = 'Standard keywords target core product features. Generate direct, conversion-focused landing pages.',
    f.created_at = datetime();

// Question keywords (informational intent)
MERGE (f:SEOKeywordFormat {key: 'question'})
SET f.display_name = 'Question',
    f.description = 'Question-format keywords indicating informational search intent',
    f.pattern = '^(how|what|why|when|where|can|does|is|are|which)\\s',
    f.content_type = 'faq',
    f.examples = ['how to create qr code', 'what is a dynamic qr code', 'how to scan qr code'],
    f.llm_context = 'Question keywords indicate informational intent. Generate educational content, FAQs, step-by-step guides.',
    f.created_at = datetime();

// Comparison keywords (vs, versus, alternative)
MERGE (f:SEOKeywordFormat {key: 'comparison'})
SET f.display_name = 'Comparison',
    f.description = 'Comparison keywords for competitive or feature comparison content',
    f.pattern = '(vs|versus|alternative|compared|better|difference)',
    f.content_type = 'comparison',
    f.examples = ['qr code vs barcode', 'dynamic vs static qr code', 'qrcode ai alternative'],
    f.llm_context = 'Comparison keywords indicate evaluation intent. Generate balanced comparison tables, pros/cons, feature matrices.',
    f.created_at = datetime();

// Preposition keywords (for, with, in, on)
MERGE (f:SEOKeywordFormat {key: 'preposition'})
SET f.display_name = 'Preposition',
    f.description = 'Keywords with prepositions indicating specific use-cases',
    f.pattern = '\\s(for|with|in|on|to|at)\\s',
    f.content_type = 'use_case',
    f.examples = ['qr code for restaurant', 'qr code for business card', 'qr code with logo'],
    f.llm_context = 'Preposition keywords indicate use-case search. Generate targeted landing pages for specific industries/applications.',
    f.created_at = datetime();

// Long-tail keywords (3+ words, specific intent)
MERGE (f:SEOKeywordFormat {key: 'long_tail'})
SET f.display_name = 'Long Tail',
    f.description = 'Multi-word keywords with specific, niche intent',
    f.pattern = '^([a-z]+\\s){3,}',
    f.content_type = 'niche',
    f.examples = ['free wifi qr code generator no signup', 'custom qr code generator with logo free', 'dynamic qr code tracking analytics'],
    f.llm_context = 'Long-tail keywords indicate specific intent. Generate highly targeted content addressing exact user needs.',
    f.created_at = datetime();

// Brand keywords (include brand names)
MERGE (f:SEOKeywordFormat {key: 'brand'})
SET f.display_name = 'Brand',
    f.description = 'Keywords containing brand names (ours or competitors)',
    f.pattern = '(qrcode ai|qr monkey|beaconstac|flowcode|bitly)',
    f.content_type = 'branded',
    f.examples = ['qrcode ai generator', 'qr monkey alternative', 'bitly vs qrcode ai'],
    f.llm_context = 'Brand keywords indicate brand awareness. For our brand: reinforce value props. For competitors: tactful comparison.',
    f.created_at = datetime();

// Local keywords (geographic modifiers)
MERGE (f:SEOKeywordFormat {key: 'local'})
SET f.display_name = 'Local',
    f.description = 'Keywords with geographic modifiers for local SEO',
    f.pattern = '(paris|london|new york|tokyo|berlin|france|usa|uk|deutschland)',
    f.content_type = 'local',
    f.examples = ['qr code generator france', 'menu qr code paris', 'qr code scanner uk'],
    f.llm_context = 'Local keywords indicate geographic intent. Generate locale-specific content with regional context.',
    f.created_at = datetime();

// ============================================================================
// 2. LINK EXISTING SEOKeywords TO FORMATS
// ============================================================================

// Question keywords
MATCH (k:SEOKeyword)
WHERE k.key STARTS WITH 'seo-how-to-'
   OR k.key STARTS WITH 'seo-what-is-'
   OR k.key STARTS WITH 'seo-can-'
   OR k.key STARTS WITH 'seo-does-'
MATCH (f:SEOKeywordFormat {key: 'question'})
MERGE (k)-[r:HAS_FORMAT]->(f)
SET r.created_at = datetime();

// Comparison keywords
MATCH (k:SEOKeyword)
WHERE k.key CONTAINS '-vs-'
   OR k.key CONTAINS '-versus-'
   OR k.key CONTAINS '-alternative'
   OR k.key CONTAINS '-compared'
MATCH (f:SEOKeywordFormat {key: 'comparison'})
MERGE (k)-[r:HAS_FORMAT]->(f)
SET r.created_at = datetime();

// Preposition keywords (for specific use-cases)
MATCH (k:SEOKeyword)
WHERE k.key CONTAINS '-for-'
   OR k.key CONTAINS '-with-'
   OR k.key ENDS WITH '-for-restaurant'
   OR k.key ENDS WITH '-for-business'
MATCH (f:SEOKeywordFormat {key: 'preposition'})
MERGE (k)-[r:HAS_FORMAT]->(f)
SET r.created_at = datetime();

// Brand keywords
MATCH (k:SEOKeyword)
WHERE k.key CONTAINS 'qrcode-ai'
   OR k.key CONTAINS 'qr-monkey'
   OR k.key CONTAINS 'beaconstac'
   OR k.key CONTAINS 'flowcode'
MATCH (f:SEOKeywordFormat {key: 'brand'})
MERGE (k)-[r:HAS_FORMAT]->(f)
SET r.created_at = datetime();

// Standard keywords (fallback for unclassified)
MATCH (k:SEOKeyword)
WHERE NOT EXISTS { (k)-[:HAS_FORMAT]->(:SEOKeywordFormat) }
MATCH (f:SEOKeywordFormat {key: 'standard'})
MERGE (k)-[r:HAS_FORMAT]->(f)
SET r.created_at = datetime();

// ============================================================================
// 3. VERIFICATION QUERY
// ============================================================================
// Run this to verify SEOKeywordFormat nodes and arcs:
//
// MATCH (f:SEOKeywordFormat)
// OPTIONAL MATCH (k:SEOKeyword)-[:HAS_FORMAT]->(f)
// RETURN f.key AS format, f.content_type, count(k) AS keywords
// ORDER BY keywords DESC;
//
