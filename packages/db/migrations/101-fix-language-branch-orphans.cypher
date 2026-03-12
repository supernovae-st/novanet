// ============================================================================
// Migration 101: Fix LanguageBranch Orphans and ISO Codes
// ============================================================================
// Purpose: Connect 17 orphan locales to their LanguageBranch nodes
// Also adds missing iso_code properties to existing branches
// ============================================================================

// ============================================================================
// SECTION 1: Connect Orphan Locales to LanguageBranch
// ============================================================================

// --- Hellenic (Greek) ---
MATCH (l:Locale) WHERE l.key IN ['el-GR', 'el-CY']
MATCH (lb:LanguageBranch {key: 'grk'})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// --- Celtic ---
MATCH (l:Locale) WHERE l.key IN ['cy-GB', 'ga-IE']
MATCH (lb:LanguageBranch {key: 'cel'})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// --- Iranian ---
MATCH (l:Locale {key: 'fa-IR'})
MATCH (lb:LanguageBranch {key: 'ira'})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// --- Baltic ---
MATCH (l:Locale) WHERE l.key IN ['lt-LT', 'lv-LV']
MATCH (lb:LanguageBranch {key: 'bal'})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// --- Mon-Khmer (Austroasiatic) ---
MATCH (l:Locale) WHERE l.key IN ['vi-VN', 'km-KH']
MATCH (lb:LanguageBranch {key: 'mkh'})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// --- Tai-Kadai (Thai, Lao) ---
// Create tai-kadai branch if not exists
MERGE (lb:LanguageBranch {key: 'tai'})
ON CREATE SET
  lb.display_name = 'Tai-Kadai Languages',
  lb.description = 'Language family including Thai, Lao, Shan, and related languages of Southeast Asia',
  lb.iso_code = 'tai',
  lb.llm_context = 'USE: for Thai, Lao, Shan locales. TRIGGERS: Southeast Asian tonal languages, Thai script. NOT: for Vietnamese (Mon-Khmer).',
  lb.created_at = datetime(),
  lb.updated_at = datetime();

MATCH (l:Locale) WHERE l.key IN ['th-TH', 'lo-LA']
MATCH (lb:LanguageBranch {key: 'tai'})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// --- Indo-Aryan (Odia) ---
MATCH (l:Locale {key: 'or-IN'})
MATCH (lb:LanguageBranch {key: 'inc'})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// --- Bantu (Swahili) ---
// Create Bantu branch if not exists
MERGE (lb:LanguageBranch {key: 'bnt'})
ON CREATE SET
  lb.display_name = 'Bantu Languages',
  lb.description = 'Language family including Swahili, Zulu, and related languages of Sub-Saharan Africa',
  lb.iso_code = 'bnt',
  lb.llm_context = 'USE: for Swahili, Zulu, Xhosa locales. TRIGGERS: African Bantu languages. NOT: for Afrikaans (Germanic).',
  lb.created_at = datetime(),
  lb.updated_at = datetime();

MATCH (l:Locale) WHERE l.key IN ['sw-KE', 'sw-TZ']
MATCH (lb:LanguageBranch {key: 'bnt'})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// --- Isolates (Armenian, Georgian, Albanian) → Use 'other' branch ---
MATCH (l:Locale) WHERE l.key IN ['hy-AM', 'ka-GE', 'sq-AL']
MATCH (lb:LanguageBranch {key: 'other'})
MERGE (l)-[:SPEAKS_BRANCH]->(lb);

// ============================================================================
// SECTION 2: Add iso_code to LanguageBranch nodes missing it
// ============================================================================

// Romance languages
MATCH (lb:LanguageBranch) WHERE lb.key IN ['romance', 'roa'] AND lb.iso_code IS NULL
SET lb.iso_code = 'roa', lb.updated_at = datetime();

// Germanic languages
MATCH (lb:LanguageBranch) WHERE lb.key IN ['germanic', 'gem'] AND lb.iso_code IS NULL
SET lb.iso_code = 'gem', lb.updated_at = datetime();

// Slavic languages
MATCH (lb:LanguageBranch) WHERE lb.key IN ['slavic', 'sla'] AND lb.iso_code IS NULL
SET lb.iso_code = 'sla', lb.updated_at = datetime();

// Sino-Tibetan
MATCH (lb:LanguageBranch {key: 'sino_tibetan'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'sit', lb.updated_at = datetime();

// Japonic
MATCH (lb:LanguageBranch {key: 'japonic'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'jpx', lb.updated_at = datetime();

// Koreanic
MATCH (lb:LanguageBranch {key: 'koreanic'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'kor', lb.updated_at = datetime();

// Austronesian
MATCH (lb:LanguageBranch {key: 'austronesian'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'map', lb.updated_at = datetime();

// Indo-Aryan
MATCH (lb:LanguageBranch) WHERE lb.key IN ['indo_aryan', 'inc'] AND lb.iso_code IS NULL
SET lb.iso_code = 'inc', lb.updated_at = datetime();

// Dravidian
MATCH (lb:LanguageBranch {key: 'dravidian'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'dra', lb.updated_at = datetime();

// Turkic
MATCH (lb:LanguageBranch {key: 'turkic'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'trk', lb.updated_at = datetime();

// Uralic
MATCH (lb:LanguageBranch {key: 'uralic'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'urj', lb.updated_at = datetime();

// Semitic
MATCH (lb:LanguageBranch) WHERE lb.key IN ['semitic', 'sem'] AND lb.iso_code IS NULL
SET lb.iso_code = 'sem', lb.updated_at = datetime();

// Hellenic
MATCH (lb:LanguageBranch {key: 'grk'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'grk', lb.updated_at = datetime();

// Celtic
MATCH (lb:LanguageBranch {key: 'cel'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'cel', lb.updated_at = datetime();

// Iranian
MATCH (lb:LanguageBranch {key: 'ira'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'ira', lb.updated_at = datetime();

// Mon-Khmer
MATCH (lb:LanguageBranch {key: 'mkh'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'mkh', lb.updated_at = datetime();

// Baltic
MATCH (lb:LanguageBranch {key: 'bal'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'bat', lb.updated_at = datetime();

// Finno-Ugric
MATCH (lb:LanguageBranch {key: 'fiu'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'fiu', lb.updated_at = datetime();

// Other
MATCH (lb:LanguageBranch {key: 'other'}) WHERE lb.iso_code IS NULL
SET lb.iso_code = 'mis', lb.updated_at = datetime();

// ============================================================================
// SECTION 3: Verification Queries (run manually to verify)
// ============================================================================

// Verify orphan locales are connected:
// MATCH (l:Locale)
// OPTIONAL MATCH (l)-[:SPEAKS_BRANCH]->(lb:LanguageBranch)
// WITH count(l) AS total, count(lb) AS linked
// RETURN total, linked, round(100.0 * linked / total, 1) AS coverage_pct;

// Verify iso_code coverage:
// MATCH (lb:LanguageBranch)
// WITH count(*) AS total, count(lb.iso_code) AS with_iso
// RETURN total, with_iso, round(100.0 * with_iso / total, 1) AS iso_coverage_pct;

