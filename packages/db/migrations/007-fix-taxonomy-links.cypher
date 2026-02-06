// =============================================================================
// MIGRATION 007: Fix Geographic Taxonomy Links
// =============================================================================
// Adds missing language branches and fixes relationship mappings
// =============================================================================

// -----------------------------------------------------------------------------
// 1. ADD MISSING LANGUAGE BRANCHES
// -----------------------------------------------------------------------------

// Japanese (Japonic family - language isolate)
MERGE (b:LanguageBranch:Meta {key: 'jpx'})
ON CREATE SET
  b.name = 'Japonic',
  b.iso_639_5 = 'jpx',
  b.parent_family = 'jpx',
  b.includes = ['ja'],
  b.llm_context = 'Japanese. Unique script (hiragana, katakana, kanji). East Asian culture. High-context.'
ON MATCH SET
  b.name = 'Japonic';

// Korean (Koreanic family - language isolate)
MERGE (b:LanguageBranch:Meta {key: 'kor'})
ON CREATE SET
  b.name = 'Koreanic',
  b.iso_639_5 = 'kor',
  b.parent_family = 'kor',
  b.includes = ['ko'],
  b.llm_context = 'Korean. Hangul script. East Asian culture with unique identity. High-context.'
ON MATCH SET
  b.name = 'Koreanic';

// Tai (Kra-Dai family)
MERGE (b:LanguageBranch:Meta {key: 'tai'})
ON CREATE SET
  b.name = 'Tai',
  b.iso_639_5 = 'tai',
  b.parent_family = 'tai',
  b.includes = ['th', 'lo'],
  b.llm_context = 'Thai, Lao. Unique scripts. Tonal languages. Theravada Buddhist culture.'
ON MATCH SET
  b.name = 'Tai';

// Kartvelian (South Caucasian)
MERGE (b:LanguageBranch:Meta {key: 'ccs'})
ON CREATE SET
  b.name = 'Kartvelian',
  b.iso_639_5 = 'ccs',
  b.parent_family = 'ccs',
  b.includes = ['ka'],
  b.llm_context = 'Georgian. Unique script. Caucasian culture. Orthodox Christian heritage.'
ON MATCH SET
  b.name = 'Kartvelian';

// -----------------------------------------------------------------------------
// 2. LINK JAPANESE, KOREAN, THAI LOCALES TO BRANCHES
// -----------------------------------------------------------------------------

// Japanese
MATCH (l:Locale), (b:LanguageBranch {key: 'jpx'})
WHERE l.language_code = 'ja'
MERGE (l)-[:SPEAKS_BRANCH]->(b);

// Korean
MATCH (l:Locale), (b:LanguageBranch {key: 'kor'})
WHERE l.language_code = 'ko'
MERGE (l)-[:SPEAKS_BRANCH]->(b);

// Thai/Lao
MATCH (l:Locale), (b:LanguageBranch {key: 'tai'})
WHERE l.language_code IN ['th', 'lo']
MERGE (l)-[:SPEAKS_BRANCH]->(b);

// Georgian
MATCH (l:Locale), (b:LanguageBranch {key: 'ccs'})
WHERE l.language_code = 'ka'
MERGE (l)-[:SPEAKS_BRANCH]->(b);

// -----------------------------------------------------------------------------
// 3. FIX CULTURAL SUBREALM LINKS (use correct keys)
// -----------------------------------------------------------------------------

// Delete old incorrect relationships first
MATCH (l:Locale)-[r:IN_CULTURAL_SUBREALM]->()
DELETE r;

// Islamic Arab (covers Gulf, Levant, Maghreb, Nile)
MATCH (l:Locale), (s:CulturalSubRealm {key: 'islamic-arab'})
WHERE l.country_code IN ['AE', 'BH', 'KW', 'OM', 'QA', 'SA', 'IQ', 'JO', 'LB', 'PS', 'SY', 'DZ', 'LY', 'MA', 'TN', 'EG', 'SD', 'YE']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Islamic Persian
MATCH (l:Locale), (s:CulturalSubRealm {key: 'islamic-persian'})
WHERE l.country_code IN ['AF', 'IR', 'TJ']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Islamic Turkic
MATCH (l:Locale), (s:CulturalSubRealm {key: 'islamic-turkic'})
WHERE l.country_code IN ['AZ', 'KG', 'KZ', 'TM', 'TR', 'UZ']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// South Asian Islamic
MATCH (l:Locale), (s:CulturalSubRealm {key: 'south-asian-islamic'})
WHERE l.country_code IN ['BD', 'PK']
   OR (l.country_code = 'IN' AND l.language_code = 'ur')
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Maritime Islamic (Indonesia, Malaysia, Brunei)
MATCH (l:Locale), (s:CulturalSubRealm {key: 'maritime-islamic'})
WHERE l.country_code IN ['BN', 'ID', 'MY']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Anglo-American
MATCH (l:Locale), (s:CulturalSubRealm {key: 'anglo-american'})
WHERE l.country_code IN ['US', 'CA', 'GB', 'IE', 'AU', 'NZ']
   OR (l.country_code IN ['ZA', 'NG', 'KE', 'GH', 'UG', 'ZM', 'ZW', 'BW', 'TZ', 'JM', 'TT', 'BB'] AND l.language_code = 'en')
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Nordic
MATCH (l:Locale), (s:CulturalSubRealm {key: 'nordic'})
WHERE l.country_code IN ['DK', 'FI', 'IS', 'NO', 'SE']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Western European
MATCH (l:Locale), (s:CulturalSubRealm {key: 'west-european'})
WHERE l.country_code IN ['AT', 'BE', 'CH', 'DE', 'LI', 'LU', 'MC', 'NL']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Mediterranean
MATCH (l:Locale), (s:CulturalSubRealm {key: 'mediterranean'})
WHERE l.country_code IN ['AD', 'CY', 'ES', 'FR', 'GR', 'HR', 'IT', 'MT', 'PT', 'SI', 'SM', 'VA']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Eastern European (Slavic Orthodox)
MATCH (l:Locale), (s:CulturalSubRealm {key: 'slavic-orthodox'})
WHERE l.country_code IN ['BG', 'BY', 'MD', 'MK', 'ME', 'RO', 'RS', 'RU', 'UA']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Latin American
MATCH (l:Locale), (s:CulturalSubRealm {key: 'latin-american'})
WHERE l.country_code IN ['AR', 'BO', 'BR', 'CL', 'CO', 'CR', 'CU', 'DO', 'EC', 'GT', 'HN', 'MX', 'NI', 'PA', 'PE', 'PR', 'PY', 'SV', 'UY', 'VE', 'HT']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Eastern European (Central)
MATCH (l:Locale), (s:CulturalSubRealm {key: 'eastern-european'})
WHERE l.country_code IN ['AL', 'BA', 'CZ', 'EE', 'HU', 'LT', 'LV', 'PL', 'SK', 'XK']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// North Indian
MATCH (l:Locale), (s:CulturalSubRealm {key: 'north-indian'})
WHERE l.country_code = 'IN' AND l.language_code IN ['hi', 'pa', 'gu', 'mr']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// South Indian
MATCH (l:Locale), (s:CulturalSubRealm {key: 'south-indian'})
WHERE l.country_code = 'IN' AND l.language_code IN ['ta', 'te', 'kn', 'ml']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Himalayan Buddhist
MATCH (l:Locale), (s:CulturalSubRealm {key: 'himalayan-buddhist'})
WHERE l.country_code IN ['BT', 'NP']
   OR (l.country_code = 'IN' AND l.language_code = 'ne')
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Sinosphere
MATCH (l:Locale), (s:CulturalSubRealm {key: 'sinosphere'})
WHERE l.country_code IN ['CN', 'HK', 'MO', 'TW', 'SG'] AND l.language_code = 'zh'
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Japanese
MATCH (l:Locale), (s:CulturalSubRealm {key: 'japanese'})
WHERE l.country_code = 'JP'
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Korean
MATCH (l:Locale), (s:CulturalSubRealm {key: 'korean'})
WHERE l.country_code = 'KR'
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Vietnamese Sinosphere
MATCH (l:Locale), (s:CulturalSubRealm {key: 'vietnamese-sinosphere'})
WHERE l.country_code = 'VN'
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Theravada Buddhist (Mainland SEA)
MATCH (l:Locale), (s:CulturalSubRealm {key: 'theravada-buddhist'})
WHERE l.country_code IN ['KH', 'LA', 'MM', 'TH', 'LK']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Philippine Syncretic
MATCH (l:Locale), (s:CulturalSubRealm {key: 'philippine-syncretic'})
WHERE l.country_code = 'PH'
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// West African
MATCH (l:Locale), (s:CulturalSubRealm {key: 'west-african'})
WHERE l.country_code IN ['BF', 'BJ', 'CI', 'GH', 'GM', 'GN', 'GW', 'LR', 'ML', 'MR', 'NE', 'NG', 'SL', 'SN', 'TG']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// East African
MATCH (l:Locale), (s:CulturalSubRealm {key: 'east-african'})
WHERE l.country_code IN ['BI', 'DJ', 'ER', 'ET', 'KE', 'MG', 'MW', 'MZ', 'RW', 'SO', 'TZ', 'UG', 'ZM', 'ZW']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// Southern African
MATCH (l:Locale), (s:CulturalSubRealm {key: 'southern-african'})
WHERE l.country_code IN ['AO', 'BW', 'CD', 'LS', 'NA', 'SZ', 'ZA']
MERGE (l)-[:IN_CULTURAL_SUBREALM]->(s);

// -----------------------------------------------------------------------------
// 4. FIX POPULATION SUBCLUSTER LINKS (use correct keys)
// -----------------------------------------------------------------------------

// Delete old incorrect relationships first
MATCH (l:Locale)-[r:HAS_PRIMARY_POPULATION]->()
DELETE r;

// Nordic
MATCH (l:Locale), (p:PopulationSubCluster {key: 'nordic'})
WHERE l.country_code IN ['DK', 'EE', 'FI', 'IS', 'LT', 'LV', 'NO', 'SE']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Mediterranean
MATCH (l:Locale), (p:PopulationSubCluster {key: 'mediterranean'})
WHERE l.country_code IN ['AD', 'AL', 'BA', 'CY', 'ES', 'FR', 'GR', 'HR', 'IT', 'ME', 'MK', 'MT', 'PT', 'RS', 'SI', 'SM', 'VA', 'XK']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Slavic
MATCH (l:Locale), (p:PopulationSubCluster {key: 'slavic'})
WHERE l.country_code IN ['BG', 'BY', 'CZ', 'HU', 'MD', 'PL', 'RO', 'RU', 'SK', 'UA']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Celtic
MATCH (l:Locale), (p:PopulationSubCluster {key: 'celtic'})
WHERE l.country_code IN ['GB', 'IE'] OR (l.country_code = 'FR' AND l.language_code IN ['br', 'cy'])
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Middle Eastern
MATCH (l:Locale), (p:PopulationSubCluster {key: 'middle-eastern'})
WHERE l.country_code IN ['AE', 'BH', 'IL', 'IQ', 'JO', 'KW', 'LB', 'OM', 'PS', 'QA', 'SA', 'SY', 'YE']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// North African
MATCH (l:Locale), (p:PopulationSubCluster {key: 'north-african'})
WHERE l.country_code IN ['DZ', 'EG', 'LY', 'MA', 'SD', 'TN']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Central Asian
MATCH (l:Locale), (p:PopulationSubCluster {key: 'central-asian'})
WHERE l.country_code IN ['AF', 'AZ', 'IR', 'KG', 'KZ', 'TJ', 'TM', 'TR', 'UZ']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// North Indian
MATCH (l:Locale), (p:PopulationSubCluster {key: 'north-indian'})
WHERE l.country_code IN ['BD', 'NP', 'PK']
   OR (l.country_code = 'IN' AND l.language_code IN ['hi', 'pa', 'gu', 'mr', 'ur', 'bn', 'as', 'ne'])
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// South Indian
MATCH (l:Locale), (p:PopulationSubCluster {key: 'south-indian'})
WHERE l.country_code = 'LK'
   OR (l.country_code = 'IN' AND l.language_code IN ['ta', 'te', 'kn', 'ml'])
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Han Chinese
MATCH (l:Locale), (p:PopulationSubCluster {key: 'han-chinese'})
WHERE l.country_code IN ['CN', 'HK', 'MO', 'TW'] OR (l.country_code = 'SG' AND l.language_code = 'zh')
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Japanese
MATCH (l:Locale), (p:PopulationSubCluster {key: 'japanese'})
WHERE l.country_code = 'JP'
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Korean
MATCH (l:Locale), (p:PopulationSubCluster {key: 'korean'})
WHERE l.country_code = 'KR'
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Mainland SEA
MATCH (l:Locale), (p:PopulationSubCluster {key: 'mainland-sea'})
WHERE l.country_code IN ['KH', 'LA', 'MM', 'TH', 'VN']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Maritime SEA
MATCH (l:Locale), (p:PopulationSubCluster {key: 'maritime-sea'})
WHERE l.country_code IN ['BN', 'ID', 'MY', 'PH', 'SG', 'TL']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// West African
MATCH (l:Locale), (p:PopulationSubCluster {key: 'west-african'})
WHERE l.country_code IN ['BF', 'BJ', 'CI', 'GH', 'GM', 'GN', 'GW', 'LR', 'ML', 'MR', 'NE', 'NG', 'SL', 'SN', 'TG']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// East African
MATCH (l:Locale), (p:PopulationSubCluster {key: 'east-african'})
WHERE l.country_code IN ['BI', 'DJ', 'ER', 'ET', 'KE', 'MG', 'MW', 'MZ', 'RW', 'SO', 'TZ', 'UG', 'ZM', 'ZW']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Southern African Bantu
MATCH (l:Locale), (p:PopulationSubCluster {key: 'southern-african-bantu'})
WHERE l.country_code IN ['AO', 'BW', 'CD', 'LS', 'NA', 'SZ', 'ZA']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Mesoamerican
MATCH (l:Locale), (p:PopulationSubCluster {key: 'mesoamerican'})
WHERE l.country_code IN ['GT', 'HN', 'MX', 'NI', 'SV']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Andean
MATCH (l:Locale), (p:PopulationSubCluster {key: 'andean'})
WHERE l.country_code IN ['BO', 'EC', 'PE']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Polynesian
MATCH (l:Locale), (p:PopulationSubCluster {key: 'polynesian'})
WHERE l.country_code IN ['NZ', 'FJ', 'TO', 'WS', 'CK', 'NU', 'PF', 'TK', 'TV', 'WF']
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);

// Latin American general (for remaining countries)
MATCH (l:Locale), (p:PopulationSubCluster {key: 'mesoamerican'})
WHERE l.country_code IN ['AR', 'BR', 'CL', 'CO', 'CR', 'CU', 'DO', 'PA', 'PR', 'PY', 'UY', 'VE', 'HT', 'JM', 'TT', 'BB']
AND NOT (l)-[:HAS_PRIMARY_POPULATION]->()
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(p);
