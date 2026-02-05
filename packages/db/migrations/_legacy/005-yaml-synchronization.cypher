// =============================================================================
// Migration 005: YAML Synchronization
// =============================================================================
//
// This migration aligns the Neo4j database with YAML v7.11.0 schema definitions.
//
// FIXES:
//   1. Add missing `name_native` property to Locale nodes (REQUIRED in YAML)
//   2. Create LocaleRulesFormatting nodes (currency, date_format, number_format)
//   3. Create LocaleRulesAdaptation nodes (content adaptation rules)
//   4. Create LocaleRulesSlug nodes (URL slug generation rules)
//   5. Create LocaleCultureReferences nodes (cultural references)
//   6. Add sample Reference, Metaphor, Pattern, Constraint nodes
//
// NOTE: Existing properties (currency, date_format, etc.) on Locale are kept
//       for backward compatibility but canonical location is now in child nodes.
//
// =============================================================================

// =============================================================================
// 1. ADD MISSING name_native TO LOCALE NODES
// =============================================================================

MATCH (l:Locale {key: "en-US"})
SET l.name_native = "English (United States)",
    l.is_primary = true,
    l.updated_at = datetime();

MATCH (l:Locale {key: "fr-FR"})
SET l.name_native = "Français (France)",
    l.is_primary = true,
    l.updated_at = datetime();

MATCH (l:Locale {key: "fr-CA"})
SET l.name_native = "Français (Canada)",
    l.is_primary = false,
    l.updated_at = datetime();

MATCH (l:Locale {key: "es-ES"})
SET l.name_native = "Español (España)",
    l.is_primary = true,
    l.updated_at = datetime();

MATCH (l:Locale {key: "de-DE"})
SET l.name_native = "Deutsch (Deutschland)",
    l.is_primary = true,
    l.updated_at = datetime();

MATCH (l:Locale {key: "ja-JP"})
SET l.name_native = "日本語（日本）",
    l.is_primary = true,
    l.updated_at = datetime();

// =============================================================================
// 2. CREATE LocaleRulesFormatting NODES
// =============================================================================
// These contain the canonical location for currency, date_format, number_format

// en-US Formatting Rules
MATCH (l:Locale {key: "en-US"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting) }
CREATE (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting {
  display_name: "US English Formatting Rules",
  description: "Date, number, and currency formatting for en-US",
  llm_context: "USE: when formatting FACTS (dates, numbers, prices). TRIGGERS: format, date, price, number. NOT: content adaptation.",
  // Number formatting
  decimal_separator: ".",
  thousands_separator: ",",
  grouping_pattern: 3,
  // Date formatting
  date_pattern: "MM/DD/YYYY",
  date_pattern_short: "M/D/YY",
  date_pattern_long: "MMMM D, YYYY",
  first_day_of_week: "sunday",
  // Time formatting
  time_format: "12h",
  time_separator: ":",
  // Currency
  currency_code: "USD",
  currency_symbol: "$",
  currency_position: "before",
  currency_spacing: false,
  // Percentage
  percentage_spacing: false,
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR Formatting Rules
MATCH (l:Locale {key: "fr-FR"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting) }
CREATE (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting {
  display_name: "French (France) Formatting Rules",
  description: "Date, number, and currency formatting for fr-FR",
  llm_context: "USE: when formatting FACTS. TRIGGERS: format, date, prix, nombre. NOT: content adaptation.",
  decimal_separator: ",",
  thousands_separator: " ",
  grouping_pattern: 3,
  date_pattern: "DD/MM/YYYY",
  date_pattern_short: "DD/MM/YY",
  date_pattern_long: "D MMMM YYYY",
  first_day_of_week: "monday",
  time_format: "24h",
  time_separator: "h",
  currency_code: "EUR",
  currency_symbol: "€",
  currency_position: "after",
  currency_spacing: true,
  percentage_spacing: true,
  created_at: datetime(),
  updated_at: datetime()
});

// fr-CA Formatting Rules
MATCH (l:Locale {key: "fr-CA"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting) }
CREATE (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting {
  display_name: "French (Canada) Formatting Rules",
  description: "Date, number, and currency formatting for fr-CA",
  llm_context: "USE: when formatting FACTS for Quebec. TRIGGERS: format, date, prix, nombre. NOT: France French formats.",
  decimal_separator: ",",
  thousands_separator: " ",
  grouping_pattern: 3,
  date_pattern: "YYYY-MM-DD",
  date_pattern_short: "YY-MM-DD",
  date_pattern_long: "D MMMM YYYY",
  first_day_of_week: "sunday",
  time_format: "24h",
  time_separator: "h",
  currency_code: "CAD",
  currency_symbol: "$",
  currency_position: "after",
  currency_spacing: true,
  percentage_spacing: true,
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES Formatting Rules
MATCH (l:Locale {key: "es-ES"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting) }
CREATE (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting {
  display_name: "Spanish (Spain) Formatting Rules",
  description: "Date, number, and currency formatting for es-ES",
  llm_context: "USE: when formatting FACTS for Spain. TRIGGERS: formato, fecha, precio, número. NOT: Latin American formats.",
  decimal_separator: ",",
  thousands_separator: ".",
  grouping_pattern: 3,
  date_pattern: "DD/MM/YYYY",
  date_pattern_short: "DD/MM/YY",
  date_pattern_long: "D de MMMM de YYYY",
  first_day_of_week: "monday",
  time_format: "24h",
  time_separator: ":",
  currency_code: "EUR",
  currency_symbol: "€",
  currency_position: "after",
  currency_spacing: true,
  percentage_spacing: true,
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE Formatting Rules
MATCH (l:Locale {key: "de-DE"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting) }
CREATE (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting {
  display_name: "German (Germany) Formatting Rules",
  description: "Date, number, and currency formatting for de-DE",
  llm_context: "USE: when formatting FACTS for Germany. TRIGGERS: Format, Datum, Preis, Zahl. NOT: Austrian or Swiss formats.",
  decimal_separator: ",",
  thousands_separator: ".",
  grouping_pattern: 3,
  date_pattern: "DD.MM.YYYY",
  date_pattern_short: "DD.MM.YY",
  date_pattern_long: "D. MMMM YYYY",
  first_day_of_week: "monday",
  time_format: "24h",
  time_separator: ":",
  currency_code: "EUR",
  currency_symbol: "€",
  currency_position: "after",
  currency_spacing: true,
  percentage_spacing: true,
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP Formatting Rules
MATCH (l:Locale {key: "ja-JP"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting) }
CREATE (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting {
  display_name: "Japanese Formatting Rules",
  description: "Date, number, and currency formatting for ja-JP",
  llm_context: "USE: when formatting FACTS for Japan. TRIGGERS: フォーマット, 日付, 価格, 数字. NOT: other Asian formats.",
  decimal_separator: ".",
  thousands_separator: ",",
  grouping_pattern: 3,
  date_pattern: "YYYY/MM/DD",
  date_pattern_short: "YY/MM/DD",
  date_pattern_long: "YYYY年M月D日",
  first_day_of_week: "sunday",
  time_format: "24h",
  time_separator: ":",
  currency_code: "JPY",
  currency_symbol: "¥",
  currency_position: "before",
  currency_spacing: false,
  percentage_spacing: false,
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// 3. CREATE LocaleRulesAdaptation NODES
// =============================================================================

// en-US Adaptation Rules
MATCH (l:Locale {key: "en-US"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation) }
CREATE (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation {
  display_name: "US English Adaptation Rules",
  description: "Content adaptation guidelines for en-US",
  llm_context: "USE: when deciding WHAT to adapt. TRIGGERS: adapt, localize, FACTS, ILLUSTRATIONS. NOT: formatting rules.",
  key_directive: "GENERATE native content, direct and action-oriented",
  facts_categories: '["Financial: translate literally, local number format", "Legal: translate literally, never adapt", "Brand: preserve exactly"]',
  illustrations_treatment: '{"definition": "Examples, metaphors, comparisons", "treatment": "Use American references, sports, pop culture"}',
  tone_calibration: '{"general_tone": "Enthusiastic, direct", "emotional_intensity": "High energy OK", "superlatives": "Acceptable in marketing"}',
  structural_rules: '{"cta_placement": "Multiple CTAs OK", "bullet_points": "Short, punchy", "headlines": "Clever wordplay encouraged"}',
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR Adaptation Rules
MATCH (l:Locale {key: "fr-FR"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation) }
CREATE (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation {
  display_name: "French (France) Adaptation Rules",
  description: "Content adaptation guidelines for fr-FR",
  llm_context: "USE: when deciding WHAT to adapt. TRIGGERS: adapter, localiser, FACTS, ILLUSTRATIONS. NOT: formatting rules.",
  key_directive: "GENERATE native content, reduce hyperbole, prioritize elegance",
  facts_categories: '["Financial: translate literally, local number format", "Legal: RGPD compliance, never adapt", "Brand: preserve exactly"]',
  illustrations_treatment: '{"definition": "Examples, metaphors, comparisons", "treatment": "Use French references, avoid Americanisms"}',
  tone_calibration: '{"general_tone": "Match enthusiasm but reduce hyperbole by 30%", "emotional_intensity": "Tone down from 10 to 7-8", "superlatives": "Avoid best in world, use parmi les meilleurs"}',
  structural_rules: '{"cta_placement": "End of section, not interrupting", "bullet_points": "Complete sentences, start with verbs", "headlines": "Informative over clever"}',
  created_at: datetime(),
  updated_at: datetime()
});

// fr-CA Adaptation Rules
MATCH (l:Locale {key: "fr-CA"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation) }
CREATE (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation {
  display_name: "French (Canada) Adaptation Rules",
  description: "Content adaptation guidelines for fr-CA",
  llm_context: "USE: when deciding WHAT to adapt for Quebec. TRIGGERS: adapter, localiser. NOT: France French style.",
  key_directive: "GENERATE native Quebec French, balance formality between FR and US",
  facts_categories: '["Financial: translate literally, CAD format", "Legal: Quebec law compliance", "Brand: preserve exactly"]',
  illustrations_treatment: '{"definition": "Examples, metaphors, comparisons", "treatment": "Use Quebec references, hockey, local culture"}',
  tone_calibration: '{"general_tone": "Warmer than France, less formal", "emotional_intensity": "Moderate", "superlatives": "More acceptable than fr-FR"}',
  structural_rules: '{"cta_placement": "Flexible", "bullet_points": "Mix of styles", "headlines": "Clear and direct"}',
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES Adaptation Rules
MATCH (l:Locale {key: "es-ES"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation) }
CREATE (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation {
  display_name: "Spanish (Spain) Adaptation Rules",
  description: "Content adaptation guidelines for es-ES",
  llm_context: "USE: when deciding WHAT to adapt for Spain. TRIGGERS: adaptar, localizar. NOT: Latin American Spanish.",
  key_directive: "GENERATE native Castilian Spanish, use vosotros, European references",
  facts_categories: '["Financial: translate literally, EUR format", "Legal: RGPD compliance", "Brand: preserve exactly"]',
  illustrations_treatment: '{"definition": "Examples, metaphors, comparisons", "treatment": "Use Spanish references, football, local culture"}',
  tone_calibration: '{"general_tone": "Warm but professional", "emotional_intensity": "Moderate", "superlatives": "Use with care"}',
  structural_rules: '{"cta_placement": "End of section", "bullet_points": "Complete sentences", "headlines": "Clear and informative"}',
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE Adaptation Rules
MATCH (l:Locale {key: "de-DE"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation) }
CREATE (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation {
  display_name: "German (Germany) Adaptation Rules",
  description: "Content adaptation guidelines for de-DE",
  llm_context: "USE: when deciding WHAT to adapt for Germany. TRIGGERS: anpassen, lokalisieren. NOT: Austrian or Swiss German.",
  key_directive: "GENERATE native German, precise and detailed, Sie form",
  facts_categories: '["Financial: translate literally, EUR format", "Legal: DSGVO compliance, exact terms", "Brand: preserve exactly"]',
  illustrations_treatment: '{"definition": "Examples, metaphors, comparisons", "treatment": "Use German references, precision valued"}',
  tone_calibration: '{"general_tone": "Professional and precise", "emotional_intensity": "Low, factual", "superlatives": "Avoid, use facts instead"}',
  structural_rules: '{"cta_placement": "End of section", "bullet_points": "Detailed, complete info", "headlines": "Clear and factual"}',
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP Adaptation Rules
MATCH (l:Locale {key: "ja-JP"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation) }
CREATE (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation {
  display_name: "Japanese Adaptation Rules",
  description: "Content adaptation guidelines for ja-JP",
  llm_context: "USE: when deciding WHAT to adapt for Japan. TRIGGERS: 適応, ローカライズ. NOT: other Asian markets.",
  key_directive: "GENERATE native Japanese, keigo appropriate, indirect style",
  facts_categories: '["Financial: translate literally, JPY format", "Legal: Japanese law compliance", "Brand: preserve with katakana if needed"]',
  illustrations_treatment: '{"definition": "Examples, metaphors, comparisons", "treatment": "Use Japanese references, seasonal awareness, group harmony"}',
  tone_calibration: '{"general_tone": "Polite, humble, indirect", "emotional_intensity": "Understated", "superlatives": "Avoid direct claims, use hedging"}',
  structural_rules: '{"cta_placement": "Subtle, end of section", "bullet_points": "Detailed, complete sentences", "headlines": "Polite and informative"}',
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// 4. CREATE LocaleRulesSlug NODES
// =============================================================================

// en-US Slug Rules
MATCH (l:Locale {key: "en-US"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug) }
CREATE (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug {
  display_name: "US English Slug Rules",
  description: "URL slug generation rules for en-US",
  llm_context: "USE: when generating URL slugs. TRIGGERS: slug, url, path. NOT: page structure.",
  slug_rule: "latin_preserve",
  diacritics_handling: "strip",
  case_handling: "lowercase",
  space_handling: "hyphen",
  special_chars_handling: "removed",
  unicode_normalization: "NFC",
  max_length: 80,
  stopwords: '{"articles": ["the", "a", "an"], "prepositions": ["of", "in", "to", "for", "with"], "conjunctions": ["and", "or", "but"]}',
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR Slug Rules
MATCH (l:Locale {key: "fr-FR"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug) }
CREATE (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug {
  display_name: "French (France) Slug Rules",
  description: "URL slug generation rules for fr-FR",
  llm_context: "USE: when generating URL slugs for French. TRIGGERS: slug, url, chemin. NOT: page structure.",
  slug_rule: "latin_preserve",
  diacritics_handling: "preserve",
  case_handling: "lowercase",
  space_handling: "hyphen",
  special_chars_handling: "removed",
  unicode_normalization: "NFC",
  max_length: 80,
  stopwords: '{"articles": ["le", "la", "les", "un", "une", "des"], "prepositions": ["de", "du", "à", "au", "aux", "en"], "conjunctions": ["et", "ou", "mais"]}',
  ligature_handling: '{"œ": "oe", "æ": "ae"}',
  created_at: datetime(),
  updated_at: datetime()
});

// fr-CA Slug Rules
MATCH (l:Locale {key: "fr-CA"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug) }
CREATE (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug {
  display_name: "French (Canada) Slug Rules",
  description: "URL slug generation rules for fr-CA",
  llm_context: "USE: when generating URL slugs for Quebec. TRIGGERS: slug, url, chemin. NOT: page structure.",
  slug_rule: "latin_preserve",
  diacritics_handling: "preserve",
  case_handling: "lowercase",
  space_handling: "hyphen",
  special_chars_handling: "removed",
  unicode_normalization: "NFC",
  max_length: 80,
  stopwords: '{"articles": ["le", "la", "les", "un", "une", "des"], "prepositions": ["de", "du", "à", "au", "aux", "en"], "conjunctions": ["et", "ou", "mais"]}',
  ligature_handling: '{"œ": "oe", "æ": "ae"}',
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES Slug Rules
MATCH (l:Locale {key: "es-ES"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug) }
CREATE (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug {
  display_name: "Spanish (Spain) Slug Rules",
  description: "URL slug generation rules for es-ES",
  llm_context: "USE: when generating URL slugs for Spain. TRIGGERS: slug, url, ruta. NOT: page structure.",
  slug_rule: "latin_preserve",
  diacritics_handling: "preserve",
  case_handling: "lowercase",
  space_handling: "hyphen",
  special_chars_handling: "removed",
  unicode_normalization: "NFC",
  max_length: 80,
  stopwords: '{"articles": ["el", "la", "los", "las", "un", "una"], "prepositions": ["de", "del", "a", "al", "en", "con"], "conjunctions": ["y", "o", "pero"]}',
  character_map: '{"ñ": "n"}',
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE Slug Rules
MATCH (l:Locale {key: "de-DE"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug) }
CREATE (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug {
  display_name: "German (Germany) Slug Rules",
  description: "URL slug generation rules for de-DE",
  llm_context: "USE: when generating URL slugs for Germany. TRIGGERS: slug, url, pfad. NOT: page structure.",
  slug_rule: "latin_transliterate",
  diacritics_handling: "transliterate",
  case_handling: "lowercase",
  space_handling: "hyphen",
  special_chars_handling: "removed",
  unicode_normalization: "NFC",
  max_length: 80,
  stopwords: '{"articles": ["der", "die", "das", "ein", "eine"], "prepositions": ["von", "zu", "in", "mit", "für"], "conjunctions": ["und", "oder", "aber"]}',
  character_map: '{"ä": "ae", "ö": "oe", "ü": "ue", "ß": "ss"}',
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP Slug Rules
MATCH (l:Locale {key: "ja-JP"})
WHERE NOT EXISTS { (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug) }
CREATE (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug {
  display_name: "Japanese Slug Rules",
  description: "URL slug generation rules for ja-JP",
  llm_context: "USE: when generating URL slugs for Japan. TRIGGERS: slug, url, パス. NOT: page structure.",
  slug_rule: "script_romanize",
  diacritics_handling: "strip",
  non_latin_handling: "romanize",
  case_handling: "lowercase",
  space_handling: "hyphen",
  special_chars_handling: "removed",
  unicode_normalization: "NFKC",
  max_length: 80,
  stopwords: '{"particles": ["の", "は", "が", "を", "に", "で"]}',
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// 5. CREATE LocaleCultureReferences NODES
// =============================================================================

// en-US Culture References
MATCH (l:Locale {key: "en-US"})-[:HAS_CULTURE]->(lc:LocaleCulture)
WHERE NOT EXISTS { (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences) }
CREATE (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences {
  display_name: "American Culture References",
  description: "Concrete cultural references for en-US content",
  llm_context: "USE: when replacing ILLUSTRATIONS. TRIGGERS: example, metaphor, reference. NOT: values/taboos.",
  emblematic_dishes: '["Hamburger", "Apple Pie", "BBQ", "Pizza"]',
  major_sports: '["Football (NFL)", "Basketball (NBA)", "Baseball (MLB)"]',
  landmark_cities: '["New York", "Los Angeles", "Chicago", "San Francisco"]',
  safe_celebrities: '["Tech founders", "Athletes", "Astronauts"]',
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR Culture References
MATCH (l:Locale {key: "fr-FR"})-[:HAS_CULTURE]->(lc:LocaleCulture)
WHERE NOT EXISTS { (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences) }
CREATE (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences {
  display_name: "French Culture References",
  description: "Concrete cultural references for fr-FR content",
  llm_context: "USE: when replacing ILLUSTRATIONS. TRIGGERS: exemple, métaphore, référence. NOT: valeurs/tabous.",
  emblematic_dishes: '["Baguette", "Croissant", "Fromage", "Vin"]',
  major_sports: '["Football", "Rugby", "Tennis", "Cyclisme"]',
  landmark_cities: '["Paris", "Lyon", "Marseille", "Bordeaux"]',
  landmark_places: '["Tour Eiffel", "Mont Saint-Michel", "Château de Versailles"]',
  safe_celebrities: '["Thomas Pesquet", "Artistes", "Sportifs"]',
  national_holidays: '["14 juillet", "11 novembre", "8 mai"]',
  created_at: datetime(),
  updated_at: datetime()
});

// fr-CA Culture References
MATCH (l:Locale {key: "fr-CA"})-[:HAS_CULTURE]->(lc:LocaleCulture)
WHERE lc IS NOT NULL AND NOT EXISTS { (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences) }
CREATE (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences {
  display_name: "Quebec Culture References",
  description: "Concrete cultural references for fr-CA content",
  llm_context: "USE: when replacing ILLUSTRATIONS for Quebec. TRIGGERS: exemple, métaphore, référence. NOT: France references.",
  emblematic_dishes: '["Poutine", "Tourtière", "Sirop érable", "Smoked meat"]',
  major_sports: '["Hockey", "Football canadien", "Baseball"]',
  landmark_cities: '["Montréal", "Québec", "Laval", "Gatineau"]',
  safe_celebrities: '["Céline Dion", "Guy Laliberté", "Sportifs Canadiens"]',
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES Culture References
MATCH (l:Locale {key: "es-ES"})-[:HAS_CULTURE]->(lc:LocaleCulture)
WHERE lc IS NOT NULL AND NOT EXISTS { (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences) }
CREATE (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences {
  display_name: "Spanish Culture References",
  description: "Concrete cultural references for es-ES content",
  llm_context: "USE: when replacing ILLUSTRATIONS for Spain. TRIGGERS: ejemplo, metáfora, referencia. NOT: Latin American references.",
  emblematic_dishes: '["Paella", "Tapas", "Jamón ibérico", "Tortilla"]',
  major_sports: '["Fútbol", "Baloncesto", "Tenis", "Ciclismo"]',
  landmark_cities: '["Madrid", "Barcelona", "Valencia", "Sevilla"]',
  landmark_places: '["Sagrada Familia", "Alhambra", "Museo del Prado"]',
  national_holidays: '["12 octubre", "6 diciembre", "Semana Santa"]',
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE Culture References
MATCH (l:Locale {key: "de-DE"})-[:HAS_CULTURE]->(lc:LocaleCulture)
WHERE lc IS NOT NULL AND NOT EXISTS { (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences) }
CREATE (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences {
  display_name: "German Culture References",
  description: "Concrete cultural references for de-DE content",
  llm_context: "USE: when replacing ILLUSTRATIONS for Germany. TRIGGERS: Beispiel, Metapher, Referenz. NOT: Austrian/Swiss references.",
  emblematic_dishes: '["Bratwurst", "Pretzel", "Sauerkraut", "Bier"]',
  major_sports: '["Fußball", "Handball", "Basketball", "Motorsport"]',
  landmark_cities: '["Berlin", "München", "Hamburg", "Frankfurt"]',
  landmark_places: '["Brandenburger Tor", "Neuschwanstein", "Kölner Dom"]',
  national_holidays: '["3. Oktober", "Weihnachten", "Ostern"]',
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP Culture References
MATCH (l:Locale {key: "ja-JP"})-[:HAS_CULTURE]->(lc:LocaleCulture)
WHERE lc IS NOT NULL AND NOT EXISTS { (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences) }
CREATE (lc)-[:HAS_CULTURE_REFERENCES]->(:LocaleCultureReferences {
  display_name: "Japanese Culture References",
  description: "Concrete cultural references for ja-JP content",
  llm_context: "USE: when replacing ILLUSTRATIONS for Japan. TRIGGERS: 例え, 比喩, 参照. NOT: other Asian references.",
  emblematic_dishes: '["寿司", "ラーメン", "天ぷら", "うどん"]',
  major_sports: '["野球", "サッカー", "相撲", "ゴルフ"]',
  landmark_cities: '["東京", "大阪", "京都", "名古屋"]',
  landmark_places: '["富士山", "東京タワー", "金閣寺", "浅草寺"]',
  national_holidays: '["正月", "桜", "ゴールデンウィーク"]',
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// 6. CREATE SAMPLE Reference, Metaphor, Pattern, Constraint NODES
// =============================================================================

// --- SAMPLE REFERENCES (fr-FR) ---
MATCH (l:Locale {key: "fr-FR"})-[:HAS_CULTURE]->(lc:LocaleCulture)-[:HAS_CULTURE_REFERENCES]->(lcr:LocaleCultureReferences)
WHERE NOT EXISTS { (lcr)-[:HAS_REFERENCE]->(:Reference {name: "Tour Eiffel"}) }
CREATE (lcr)-[:HAS_REFERENCE]->(:Reference {
  display_name: "Tour Eiffel",
  description: "Iconic landmark in Paris, universal French symbol",
  llm_context: "USE: for universal French symbolism. TRIGGERS: Paris, France, tourism, iconic. NOT: modern architecture.",
  reference_type: "landmark",
  name: "Tour Eiffel",
  name_english: "Eiffel Tower",
  recognition_score: 0.99,
  safety_level: "safe_all",
  category: "architecture",
  associations: '["elegance", "romance", "tourism", "France"]',
  usage_contexts: '["travel", "luxury", "culture", "international"]',
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})-[:HAS_CULTURE]->(lc:LocaleCulture)-[:HAS_CULTURE_REFERENCES]->(lcr:LocaleCultureReferences)
WHERE NOT EXISTS { (lcr)-[:HAS_REFERENCE]->(:Reference {name: "Thomas Pesquet"}) }
CREATE (lcr)-[:HAS_REFERENCE]->(:Reference {
  display_name: "Thomas Pesquet",
  description: "French astronaut, universally admired",
  llm_context: "USE: for science/space references. TRIGGERS: astronaut, space, science, ESA. NOT: political figures.",
  reference_type: "celebrity",
  name: "Thomas Pesquet",
  recognition_score: 0.90,
  safety_level: "safe_all",
  category: "science",
  associations: '["innovation", "exploration", "France", "science"]',
  usage_contexts: '["tech", "science", "inspiration", "achievement"]',
  created_at: datetime(),
  updated_at: datetime()
});

// --- SAMPLE METAPHORS (fr-FR) ---
MATCH (l:Locale {key: "fr-FR"})-[:HAS_CULTURE]->(lc:LocaleCulture)-[:HAS_CULTURE_REFERENCES]->(lcr:LocaleCultureReferences)
WHERE NOT EXISTS { (lcr)-[:HAS_METAPHOR]->(:Metaphor {concept_key: "easy_as"}) }
CREATE (lcr)-[:HAS_METAPHOR]->(:Metaphor {
  display_name: "Simple comme bonjour",
  description: "French equivalent of easy as pie",
  llm_context: "USE: for simplicity emphasis. TRIGGERS: easy, simple, straightforward. NOT: formal technical docs.",
  concept_key: "easy_as",
  local_text: "Simple comme bonjour",
  literal_translation: "Simple as hello",
  domain: "general",
  register: "casual",
  example_usage: "C'est simple comme bonjour à configurer.",
  alternatives: '[{"text": "Facile comme tout", "register": "casual"}, {"text": "Un jeu d enfant", "register": "casual"}]',
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})-[:HAS_CULTURE]->(lc:LocaleCulture)-[:HAS_CULTURE_REFERENCES]->(lcr:LocaleCultureReferences)
WHERE NOT EXISTS { (lcr)-[:HAS_METAPHOR]->(:Metaphor {concept_key: "fast_as"}) }
CREATE (lcr)-[:HAS_METAPHOR]->(:Metaphor {
  display_name: "En un clin d'œil",
  description: "French equivalent of in the blink of an eye",
  llm_context: "USE: for speed emphasis. TRIGGERS: fast, quick, instant. NOT: formal contexts.",
  concept_key: "fast_as",
  local_text: "En un clin d'œil",
  literal_translation: "In a wink of an eye",
  domain: "general",
  register: "semi-formal",
  example_usage: "Créez votre QR code en un clin d'œil.",
  created_at: datetime(),
  updated_at: datetime()
});

// --- SAMPLE PATTERNS (fr-FR) ---
MATCH (l:Locale {key: "fr-FR"})-[:HAS_RULES_FORMATTING]->(lrf:LocaleRulesFormatting)
WHERE NOT EXISTS { (lrf)-[:HAS_PATTERN]->(:Pattern {pattern_type: "currency"}) }
CREATE (lrf)-[:HAS_PATTERN]->(:Pattern {
  display_name: "French Currency Format",
  description: "Euro currency formatting for France",
  llm_context: "USE: when formatting prices. TRIGGERS: price, cost, euro, €. NOT: US dollar amounts.",
  pattern_type: "currency",
  format_string: "#,##0.00 €",
  correct_examples: '["99,90 €", "1 234,56 €", "10 000,00 €"]',
  incorrect_examples: '[{"value": "€99.90", "issue": "Wrong position and decimal separator"}, {"value": "99.90€", "issue": "No space, wrong decimal"}]',
  separator_decimal: ",",
  separator_thousands: " ",
  symbol_position: "after",
  symbol_spacing: true,
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})-[:HAS_RULES_FORMATTING]->(lrf:LocaleRulesFormatting)
WHERE NOT EXISTS { (lrf)-[:HAS_PATTERN]->(:Pattern {pattern_type: "date"}) }
CREATE (lrf)-[:HAS_PATTERN]->(:Pattern {
  display_name: "French Date Format",
  description: "Date formatting for France",
  llm_context: "USE: when formatting dates. TRIGGERS: date, jour. NOT: US date format.",
  pattern_type: "date",
  format_string: "DD/MM/YYYY",
  correct_examples: '["25/12/2024", "01/01/2025", "14/07/2024"]',
  incorrect_examples: '[{"value": "12/25/2024", "issue": "US format MM/DD/YYYY"}, {"value": "2024-12-25", "issue": "ISO format"}]',
  created_at: datetime(),
  updated_at: datetime()
});

// --- SAMPLE CONSTRAINTS (fr-FR) ---
MATCH (l:Locale {key: "fr-FR"})-[:HAS_CULTURE]->(lc:LocaleCulture)
WHERE NOT EXISTS { (lc)-[:HAS_CONSTRAINT]->(:Constraint {topic: "political_figures"}) }
CREATE (lc)-[:HAS_CONSTRAINT]->(:Constraint {
  display_name: "Political Neutrality",
  description: "Avoid political references in commercial content",
  llm_context: "USE: always check before political content. TRIGGERS: government, election, policy. NOT: tech regulations.",
  constraint_type: "taboo_topic",
  severity: "high",
  enforcement: "suggest_alternative",
  topic: "political_figures",
  trigger_keywords: '["président", "ministre", "élection", "parti", "vote"]',
  reason: "Political references can polarize audience and damage brand neutrality",
  cultural_context: "France has strong opinions across the political spectrum",
  alternatives: '["Use neutral civic terms", "Focus on product benefits", "Reference institutions not individuals"]',
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})-[:HAS_CULTURE]->(lc:LocaleCulture)
WHERE NOT EXISTS { (lc)-[:HAS_CONSTRAINT]->(:Constraint {topic: "religion"}) }
CREATE (lc)-[:HAS_CONSTRAINT]->(:Constraint {
  display_name: "Religious Neutrality",
  description: "Avoid religious references in commercial content",
  llm_context: "USE: always check before religious content. TRIGGERS: church, religion, faith. NOT: cultural traditions.",
  constraint_type: "taboo_topic",
  severity: "high",
  enforcement: "suggest_alternative",
  topic: "religion",
  trigger_keywords: '["religion", "église", "foi", "croyance", "dieu"]',
  reason: "France has strong laïcité (secularism) tradition",
  cultural_context: "Separation of church and state is a core French value",
  alternatives: '["Focus on universal values", "Use secular language", "Reference cultural traditions instead"]',
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// 7. CREATE MISSING LocaleCulture FOR fr-CA, es-ES, de-DE
// =============================================================================

// fr-CA Culture (if missing)
MATCH (l:Locale {key: "fr-CA"})
WHERE NOT EXISTS { (l)-[:HAS_CULTURE]->(:LocaleCulture) }
CREATE (l)-[:HAS_CULTURE]->(:LocaleCulture {
  display_name: "Quebec Culture",
  description: "Cultural norms for fr-CA",
  llm_context: "USE: for culturally appropriate Quebec content. TRIGGERS: culture, norms, valeurs québécoises. NOT: France French culture.",
  context_level: "medium",
  hierarchy_sensitivity: "low",
  values: '["bilingualism", "North American lifestyle", "French heritage", "distinct society"]',
  taboos: '["assuming France French", "ignoring Quebec identity"]',
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES Culture (if missing)
MATCH (l:Locale {key: "es-ES"})
WHERE NOT EXISTS { (l)-[:HAS_CULTURE]->(:LocaleCulture) }
CREATE (l)-[:HAS_CULTURE]->(:LocaleCulture {
  display_name: "Spanish Culture",
  description: "Cultural norms for es-ES",
  llm_context: "USE: for culturally appropriate Spain content. TRIGGERS: cultura, normas, valores españoles. NOT: Latin American culture.",
  context_level: "medium",
  hierarchy_sensitivity: "medium",
  values: '["family", "tradition", "celebration", "regional pride"]',
  taboos: '["political tensions", "regional conflicts", "religion in business"]',
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE Culture (if missing)
MATCH (l:Locale {key: "de-DE"})
WHERE NOT EXISTS { (l)-[:HAS_CULTURE]->(:LocaleCulture) }
CREATE (l)-[:HAS_CULTURE]->(:LocaleCulture {
  display_name: "German Culture",
  description: "Cultural norms for de-DE",
  llm_context: "USE: for culturally appropriate Germany content. TRIGGERS: Kultur, Normen, deutsche Werte. NOT: Austrian/Swiss culture.",
  context_level: "low",
  hierarchy_sensitivity: "medium",
  values: '["precision", "quality", "punctuality", "directness", "privacy"]',
  taboos: '["Nazi references", "stereotypes", "excessive informality in business"]',
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// 8. ADD MISSING LocaleIdentity FOR fr-CA, es-ES, de-DE
// =============================================================================

// fr-CA Identity (if missing)
MATCH (l:Locale {key: "fr-CA"})
WHERE NOT EXISTS { (l)-[:HAS_IDENTITY]->(:LocaleIdentity) }
CREATE (l)-[:HAS_IDENTITY]->(:LocaleIdentity {
  display_name: "French (Canada) Identity",
  description: "Technical identity characteristics for fr-CA locale",
  llm_context: "USE: for script/encoding decisions. TRIGGERS: accents, diacritics. NOT: voice or cultural aspects.",
  script_code: "Latn",
  script_name: "Latin",
  script_direction: "ltr",
  has_case: true,
  diacritics: true,
  special_characters: "éèêëàâùûîïôœç",
  timezone: "America/Montreal",
  utc_offset: "-05:00",
  keyboard_layout: "QWERTY (Canadian French)",
  encoding: "UTF-8",
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES Identity (if missing)
MATCH (l:Locale {key: "es-ES"})
WHERE NOT EXISTS { (l)-[:HAS_IDENTITY]->(:LocaleIdentity) }
CREATE (l)-[:HAS_IDENTITY]->(:LocaleIdentity {
  display_name: "Spanish (Spain) Identity",
  description: "Technical identity characteristics for es-ES locale",
  llm_context: "USE: for script/encoding decisions. TRIGGERS: accents, ñ, diacritics. NOT: voice or cultural aspects.",
  script_code: "Latn",
  script_name: "Latin",
  script_direction: "ltr",
  has_case: true,
  diacritics: true,
  special_characters: "áéíóúüñ¿¡",
  timezone: "Europe/Madrid",
  utc_offset: "+01:00",
  keyboard_layout: "Spanish",
  encoding: "UTF-8",
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE Identity (if missing)
MATCH (l:Locale {key: "de-DE"})
WHERE NOT EXISTS { (l)-[:HAS_IDENTITY]->(:LocaleIdentity) }
CREATE (l)-[:HAS_IDENTITY]->(:LocaleIdentity {
  display_name: "German (Germany) Identity",
  description: "Technical identity characteristics for de-DE locale",
  llm_context: "USE: for script/encoding decisions. TRIGGERS: umlauts, eszett, ß. NOT: voice or cultural aspects.",
  script_code: "Latn",
  script_name: "Latin",
  script_direction: "ltr",
  has_case: true,
  diacritics: true,
  special_characters: "äöüÄÖÜß",
  timezone: "Europe/Berlin",
  utc_offset: "+01:00",
  keyboard_layout: "QWERTZ",
  encoding: "UTF-8",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// 9. ADD MISSING LocaleVoice, LocaleMarket, LocaleLexicon FOR fr-CA, es-ES, de-DE
// =============================================================================

// fr-CA Voice (if missing)
MATCH (l:Locale {key: "fr-CA"})
WHERE NOT EXISTS { (l)-[:HAS_VOICE]->(:LocaleVoice) }
CREATE (l)-[:HAS_VOICE]->(:LocaleVoice {
  display_name: "Quebec French Voice",
  description: "Voice characteristics for fr-CA",
  llm_context: "USE: for tone/formality decisions. TRIGGERS: tu/vous, warm, direct. NOT: France French formality.",
  formality_score: 50,
  default_formality: "semi-formal",
  default_pronoun: "vous",
  directness_score: 65,
  directness_style: "semi-direct",
  warmth_score: 75,
  humor_score: 60,
  avg_sentence_length: 18,
  preferred_voice: "active",
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES Voice (if missing)
MATCH (l:Locale {key: "es-ES"})
WHERE NOT EXISTS { (l)-[:HAS_VOICE]->(:LocaleVoice) }
CREATE (l)-[:HAS_VOICE]->(:LocaleVoice {
  display_name: "Spanish (Spain) Voice",
  description: "Voice characteristics for es-ES",
  llm_context: "USE: for tone/formality decisions. TRIGGERS: tú/usted, vosotros. NOT: Latin American formality.",
  formality_score: 60,
  default_formality: "semi-formal",
  directness_score: 55,
  directness_style: "semi-direct",
  warmth_score: 70,
  humor_score: 55,
  avg_sentence_length: 18,
  preferred_voice: "active",
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE Voice (if missing)
MATCH (l:Locale {key: "de-DE"})
WHERE NOT EXISTS { (l)-[:HAS_VOICE]->(:LocaleVoice) }
CREATE (l)-[:HAS_VOICE]->(:LocaleVoice {
  display_name: "German (Germany) Voice",
  description: "Voice characteristics for de-DE",
  llm_context: "USE: for tone/formality decisions. TRIGGERS: Sie/du, formal, precise. NOT: Austrian/Swiss German.",
  formality_score: 80,
  default_formality: "formal",
  directness_score: 75,
  directness_style: "direct",
  warmth_score: 45,
  humor_score: 35,
  avg_sentence_length: 22,
  preferred_voice: "active",
  created_at: datetime(),
  updated_at: datetime()
});

// fr-CA Market (if missing)
MATCH (l:Locale {key: "fr-CA"})
WHERE NOT EXISTS { (l)-[:HAS_MARKET]->(:LocaleMarket) }
CREATE (l)-[:HAS_MARKET]->(:LocaleMarket {
  display_name: "Quebec Market",
  description: "Market data for fr-CA",
  llm_context: "USE: for market-specific content. TRIGGERS: marché, démographie, paiement. NOT: France market.",
  population: 8500000,
  internet_penetration: 94,
  mobile_penetration: 88,
  ecommerce_adoption: 85,
  payment_methods: '["Credit card", "Interac", "PayPal"]',
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES Market (if missing)
MATCH (l:Locale {key: "es-ES"})
WHERE NOT EXISTS { (l)-[:HAS_MARKET]->(:LocaleMarket) }
CREATE (l)-[:HAS_MARKET]->(:LocaleMarket {
  display_name: "Spain Market",
  description: "Market data for es-ES",
  llm_context: "USE: for market-specific content. TRIGGERS: mercado, demografía, pago. NOT: Latin American markets.",
  population: 47000000,
  internet_penetration: 93,
  mobile_penetration: 85,
  ecommerce_adoption: 80,
  payment_methods: '["Tarjeta", "Bizum", "PayPal"]',
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE Market (if missing)
MATCH (l:Locale {key: "de-DE"})
WHERE NOT EXISTS { (l)-[:HAS_MARKET]->(:LocaleMarket) }
CREATE (l)-[:HAS_MARKET]->(:LocaleMarket {
  display_name: "Germany Market",
  description: "Market data for de-DE",
  llm_context: "USE: for market-specific content. TRIGGERS: Markt, Demographie, Zahlung. NOT: Austrian/Swiss markets.",
  population: 83000000,
  internet_penetration: 93,
  mobile_penetration: 86,
  ecommerce_adoption: 82,
  payment_methods: '["Kreditkarte", "PayPal", "Sofortüberweisung", "SEPA"]',
  created_at: datetime(),
  updated_at: datetime()
});

// fr-CA Lexicon (if missing)
MATCH (l:Locale {key: "fr-CA"})
WHERE NOT EXISTS { (l)-[:HAS_LEXICON]->(:LocaleLexicon) }
CREATE (l)-[:HAS_LEXICON]->(:LocaleLexicon {
  display_name: "Quebec French Lexicon",
  description: "Lexicon rules for fr-CA",
  llm_context: "USE: for vocabulary/word choice decisions. TRIGGERS: vocabulaire, termes, expressions québécoises. NOT: France French vocabulary.",
  loanwords_policy: "english_ok",
  register_matching: true,
  style_notes: "English loanwords more acceptable. Use Quebec-specific terms (courriel, fin de semaine).",
  created_at: datetime(),
  updated_at: datetime()
});

// es-ES Lexicon (if missing)
MATCH (l:Locale {key: "es-ES"})
WHERE NOT EXISTS { (l)-[:HAS_LEXICON]->(:LocaleLexicon) }
CREATE (l)-[:HAS_LEXICON]->(:LocaleLexicon {
  display_name: "Spanish (Spain) Lexicon",
  description: "Lexicon rules for es-ES",
  llm_context: "USE: for vocabulary/word choice decisions. TRIGGERS: vocabulario, términos, expresiones. NOT: Latin American vocabulary.",
  loanwords_policy: "native_preferred",
  register_matching: true,
  style_notes: "Use Castilian terms. Vosotros conjugation. Avoid Latin American slang.",
  created_at: datetime(),
  updated_at: datetime()
});

// de-DE Lexicon (if missing)
MATCH (l:Locale {key: "de-DE"})
WHERE NOT EXISTS { (l)-[:HAS_LEXICON]->(:LocaleLexicon) }
CREATE (l)-[:HAS_LEXICON]->(:LocaleLexicon {
  display_name: "German Lexicon",
  description: "Lexicon rules for de-DE",
  llm_context: "USE: for vocabulary/word choice decisions. TRIGGERS: Vokabular, Begriffe, Ausdrücke. NOT: Austrian/Swiss vocabulary.",
  loanwords_policy: "mixed",
  register_matching: true,
  style_notes: "English tech terms OK in compounds. Prefer German where natural equivalent exists.",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// END MIGRATION 005
// =============================================================================
