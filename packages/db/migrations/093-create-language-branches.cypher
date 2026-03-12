// ============================================================================
// PLAN B - Migration 093: Create Missing LanguageBranch Nodes
// ============================================================================
// Priority: STRUCTURE (Language family taxonomy)
// Fixes: 13 locales missing LanguageBranch connections
// CSR Impact: Enables language family grouping and expression sharing
// ============================================================================

// Create LanguageBranch nodes for each language family
MERGE (lb:LanguageBranch {key: 'romance'})
ON CREATE SET
  lb.display_name = 'Romance Languages',
  lb.description = 'Languages derived from Latin: French, Spanish, Portuguese, Italian, Romanian',
  lb.llm_context = 'USE: for shared expressions and grammatical patterns across Romance languages. TRIGGERS: romance, latin, french, spanish, portuguese, italian. RELATES: Locale (contains), Expression (shares).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'germanic'})
ON CREATE SET
  lb.display_name = 'Germanic Languages',
  lb.description = 'Languages including English, German, Dutch, Swedish, Norwegian, Danish',
  lb.llm_context = 'USE: for shared patterns in Germanic language family. TRIGGERS: germanic, english, german, dutch, swedish, norwegian. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'slavic'})
ON CREATE SET
  lb.display_name = 'Slavic Languages',
  lb.description = 'Languages including Russian, Polish, Czech, Ukrainian, Bulgarian',
  lb.llm_context = 'USE: for Slavic language patterns and Cyrillic script handling. TRIGGERS: slavic, russian, polish, czech, ukrainian. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'sino_tibetan'})
ON CREATE SET
  lb.display_name = 'Sino-Tibetan Languages',
  lb.description = 'Languages including Mandarin, Cantonese, Tibetan, Burmese',
  lb.llm_context = 'USE: for CJK character handling and tonal language patterns. TRIGGERS: chinese, mandarin, cantonese, sino, tibetan. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'semitic'})
ON CREATE SET
  lb.display_name = 'Semitic Languages',
  lb.description = 'Languages including Arabic, Hebrew, Amharic, Maltese',
  lb.llm_context = 'USE: for RTL text handling and Semitic morphology. TRIGGERS: semitic, arabic, hebrew, amharic, rtl. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'japonic'})
ON CREATE SET
  lb.display_name = 'Japonic Languages',
  lb.description = 'Japanese and related languages/dialects',
  lb.llm_context = 'USE: for Japanese writing systems and honorific patterns. TRIGGERS: japanese, japonic, kanji, hiragana, katakana. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'koreanic'})
ON CREATE SET
  lb.display_name = 'Koreanic Languages',
  lb.description = 'Korean and related languages',
  lb.llm_context = 'USE: for Korean Hangul handling and honorific levels. TRIGGERS: korean, koreanic, hangul. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'austronesian'})
ON CREATE SET
  lb.display_name = 'Austronesian Languages',
  lb.description = 'Languages including Malay, Indonesian, Filipino, Maori, Hawaiian',
  lb.llm_context = 'USE: for Austronesian language patterns. TRIGGERS: austronesian, malay, indonesian, filipino, tagalog. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'indo_aryan'})
ON CREATE SET
  lb.display_name = 'Indo-Aryan Languages',
  lb.description = 'Languages including Hindi, Bengali, Punjabi, Urdu, Gujarati',
  lb.llm_context = 'USE: for Devanagari and Indo-Aryan script handling. TRIGGERS: indo-aryan, hindi, bengali, punjabi, urdu, gujarati. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'dravidian'})
ON CREATE SET
  lb.display_name = 'Dravidian Languages',
  lb.description = 'Languages including Tamil, Telugu, Kannada, Malayalam',
  lb.llm_context = 'USE: for Dravidian script handling. TRIGGERS: dravidian, tamil, telugu, kannada, malayalam. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'turkic'})
ON CREATE SET
  lb.display_name = 'Turkic Languages',
  lb.description = 'Languages including Turkish, Azerbaijani, Uzbek, Kazakh',
  lb.llm_context = 'USE: for Turkic language patterns and agglutination. TRIGGERS: turkic, turkish, azerbaijani, uzbek, kazakh. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'uralic'})
ON CREATE SET
  lb.display_name = 'Uralic Languages',
  lb.description = 'Languages including Finnish, Hungarian, Estonian',
  lb.llm_context = 'USE: for Uralic language patterns and case systems. TRIGGERS: uralic, finnish, hungarian, estonian. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

MERGE (lb:LanguageBranch {key: 'other'})
ON CREATE SET
  lb.display_name = 'Other Languages',
  lb.description = 'Languages not fitting into major family classifications',
  lb.llm_context = 'USE: for isolated or less common language families. TRIGGERS: other, isolated, basque, georgian. RELATES: Locale (contains).',
  lb.created_at = datetime(),
  lb.updated_at = datetime()
ON MATCH SET
  lb.updated_at = datetime();

// Link locales to language branches based on language_family property
MATCH (l:Locale)
WHERE l.language_family IS NOT NULL
MATCH (lb:LanguageBranch {key: l.language_family})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// Verify language branch coverage
MATCH (lb:LanguageBranch)
OPTIONAL MATCH (l:Locale)-[:SPEAKS_BRANCH]->(lb)
RETURN lb.key AS branch,
       lb.display_name AS name,
       count(l) AS locale_count
ORDER BY lb.key;

// Find orphan locales (missing branch connection)
MATCH (l:Locale)
WHERE NOT (l)-[:SPEAKS_BRANCH]->(:LanguageBranch)
RETURN l.key AS locale,
       l.language_family AS expected_branch,
       'ORPHAN' AS status
LIMIT 20;
