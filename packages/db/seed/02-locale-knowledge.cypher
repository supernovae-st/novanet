// NovaNet Locale Knowledge Seed v8.2.0
//
// Creates Locale Knowledge nodes for existing locales:
//   - LocaleIdentity: Script, timezone, encoding
//   - LocaleVoice: Formality, pronouns, tone
//   - LocaleCulture: Values, taboos, sensitivities
//   - LocaleMarket: Demographics, ecommerce
//   - LocaleLexicon: Vocabulary, idioms
//
// Aligned with YAML v7.11.0 - icon, priority, freshness removed (view-layer only)

// =============================================================================
// LOCALE IDENTITY - Script & Technical Characteristics
// =============================================================================

// en-US Identity
MATCH (l:Locale {key: "en-US"})
CREATE (l)-[:HAS_IDENTITY]->(:LocaleIdentity {
  display_name: "American English Identity",
  description: "Technical identity characteristics for en-US locale",
  llm_context: "USE: for script/encoding decisions. TRIGGERS: character set, keyboard, encoding. NOT: voice or cultural aspects.",
  script_code: "Latn",
  script_name: "Latin",
  script_direction: "ltr",
  has_case: true,
  diacritics: false,
  timezone: "America/New_York",
  utc_offset: "-05:00",
  keyboard_layout: "QWERTY",
  encoding: "UTF-8",
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR Identity
MATCH (l:Locale {key: "fr-FR"})
CREATE (l)-[:HAS_IDENTITY]->(:LocaleIdentity {
  display_name: "French (France) Identity",
  description: "Technical identity characteristics for fr-FR locale",
  llm_context: "USE: for script/encoding decisions. TRIGGERS: accents, diacritics, AZERTY. NOT: voice or cultural aspects.",
  script_code: "Latn",
  script_name: "Latin",
  script_direction: "ltr",
  has_case: true,
  diacritics: true,
  special_characters: "éèêëàâùûîïôœç",
  timezone: "Europe/Paris",
  utc_offset: "+01:00",
  keyboard_layout: "AZERTY",
  encoding: "UTF-8",
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP Identity
MATCH (l:Locale {key: "ja-JP"})
CREATE (l)-[:HAS_IDENTITY]->(:LocaleIdentity {
  display_name: "Japanese Identity",
  description: "Technical identity characteristics for ja-JP locale",
  llm_context: "USE: for script/encoding decisions. TRIGGERS: kanji, hiragana, katakana, JIS. NOT: voice or cultural aspects.",
  script_code: "Jpan",
  script_name: "Japanese (Hiragana + Katakana + Kanji)",
  script_direction: "ltr",
  has_case: false,
  diacritics: false,
  timezone: "Asia/Tokyo",
  utc_offset: "+09:00",
  keyboard_layout: "JIS",
  encoding: "UTF-8",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// LOCALE VOICE - Formality & Tone
// =============================================================================

// en-US Voice
MATCH (l:Locale {key: "en-US"})
CREATE (l)-[:HAS_VOICE]->(:LocaleVoice {
  display_name: "American English Voice",
  description: "Voice characteristics for en-US",
  llm_context: "USE: for tone/formality decisions in content. TRIGGERS: casual, direct, contractions, first-name. NOT: script or cultural norms.",
  formality_score: 30,
  default_formality: "casual",
  directness_score: 85,
  directness_style: "direct",
  warmth_score: 75,
  humor_score: 60,
  avg_sentence_length: 15,
  preferred_voice: "active",
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR Voice
MATCH (l:Locale {key: "fr-FR"})
CREATE (l)-[:HAS_VOICE]->(:LocaleVoice {
  display_name: "French (France) Voice",
  description: "Voice characteristics for fr-FR",
  llm_context: "USE: for tone/formality decisions. TRIGGERS: vous/tu, formal, indirect, elegant. NOT: script or cultural norms.",
  formality_score: 75,
  default_formality: "formal",
  default_pronoun: "vous",
  directness_score: 45,
  directness_style: "indirect",
  warmth_score: 60,
  humor_score: 50,
  avg_sentence_length: 20,
  preferred_voice: "active",
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP Voice
MATCH (l:Locale {key: "ja-JP"})
CREATE (l)-[:HAS_VOICE]->(:LocaleVoice {
  display_name: "Japanese Voice",
  description: "Voice characteristics for ja-JP",
  llm_context: "USE: for tone/formality decisions. TRIGGERS: keigo, honorifics, humble, indirect, 敬語. NOT: script or cultural norms.",
  formality_score: 90,
  default_formality: "formal",
  directness_score: 20,
  directness_style: "indirect",
  warmth_score: 65,
  humor_score: 30,
  avg_sentence_length: 25,
  preferred_voice: "passive",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// LOCALE CULTURE - Values & Sensitivities
// =============================================================================

// en-US Culture
MATCH (l:Locale {key: "en-US"})
CREATE (l)-[:HAS_CULTURE]->(:LocaleCulture {
  display_name: "American Culture",
  description: "Cultural norms for en-US",
  llm_context: "USE: for culturally appropriate content. TRIGGERS: values, norms, sensitivities, taboos. NOT: voice/tone or technical aspects.",
  context_level: "low",
  hierarchy_sensitivity: "low",
  values: ["individuality", "innovation", "success", "optimism"],
  taboos: ["politics", "religion in business"],
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR Culture
MATCH (l:Locale {key: "fr-FR"})
CREATE (l)-[:HAS_CULTURE]->(:LocaleCulture {
  display_name: "French Culture",
  description: "Cultural norms for fr-FR",
  llm_context: "USE: for culturally appropriate content. TRIGGERS: culture, norms, valeurs, sensibilités. NOT: voice/tone or technical aspects.",
  context_level: "high",
  hierarchy_sensitivity: "medium",
  values: ["quality", "tradition", "art de vivre", "intellectual debate"],
  taboos: ["religion", "politics", "money discussions"],
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP Culture
MATCH (l:Locale {key: "ja-JP"})
CREATE (l)-[:HAS_CULTURE]->(:LocaleCulture {
  display_name: "Japanese Culture",
  description: "Cultural norms for ja-JP",
  llm_context: "USE: for culturally appropriate content. TRIGGERS: 文化, harmony, hierarchy, wa, 和. NOT: voice/tone or technical aspects.",
  context_level: "high",
  hierarchy_sensitivity: "high",
  values: ["harmony (wa)", "respect", "group consensus", "attention to detail"],
  taboos: ["confrontation", "standing out", "direct criticism"],
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// LOCALE MARKET - Demographics & E-commerce
// =============================================================================

// en-US Market
MATCH (l:Locale {key: "en-US"})
CREATE (l)-[:HAS_MARKET]->(:LocaleMarket {
  display_name: "US Market",
  description: "Market data for en-US",
  llm_context: "USE: for market-specific content (pricing, features). TRIGGERS: market, demographics, ecommerce, payment. NOT: voice or cultural aspects.",
  population: 330000000,
  internet_penetration: 92,
  mobile_penetration: 85,
  ecommerce_adoption: 88,
  payment_methods: ["credit card", "Apple Pay", "PayPal"],
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR Market
MATCH (l:Locale {key: "fr-FR"})
CREATE (l)-[:HAS_MARKET]->(:LocaleMarket {
  display_name: "France Market",
  description: "Market data for fr-FR",
  llm_context: "USE: for market-specific content. TRIGGERS: marché, démographie, paiement, e-commerce. NOT: voice or cultural aspects.",
  population: 67000000,
  internet_penetration: 93,
  mobile_penetration: 82,
  ecommerce_adoption: 85,
  payment_methods: ["Carte Bancaire", "PayPal", "virement"],
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP Market
MATCH (l:Locale {key: "ja-JP"})
CREATE (l)-[:HAS_MARKET]->(:LocaleMarket {
  display_name: "Japan Market",
  description: "Market data for ja-JP",
  llm_context: "USE: for market-specific content. TRIGGERS: 市場, ecommerce, payment, QR code origin. NOT: voice or cultural aspects.",
  population: 125000000,
  internet_penetration: 94,
  mobile_penetration: 90,
  ecommerce_adoption: 82,
  payment_methods: ["credit card", "konbini", "PayPay", "LINE Pay"],
  market_notes: "QR codes invented here (Denso Wave 1994)",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// LOCALE LEXICON - Vocabulary & Idioms
// =============================================================================

// en-US Lexicon
MATCH (l:Locale {key: "en-US"})
CREATE (l)-[:HAS_LEXICON]->(:LocaleLexicon {
  display_name: "American English Lexicon",
  description: "Lexicon rules for en-US",
  llm_context: "USE: for vocabulary/word choice decisions. TRIGGERS: wording, vocabulary, terms, expressions. NOT: tone or grammar.",
  loanwords_policy: "english_ok",
  register_matching: true,
  style_notes: "Tech terms OK. Action verbs preferred. Short words over long.",
  created_at: datetime(),
  updated_at: datetime()
});

// fr-FR Lexicon
MATCH (l:Locale {key: "fr-FR"})
CREATE (l)-[:HAS_LEXICON]->(:LocaleLexicon {
  display_name: "French Lexicon",
  description: "Lexicon rules for fr-FR",
  llm_context: "USE: for vocabulary/word choice decisions. TRIGGERS: vocabulaire, termes, expressions, anglicismes. NOT: tone or grammar.",
  loanwords_policy: "native_only",
  register_matching: true,
  style_notes: "Prefer native terms. Avoid anglicisms when French equivalent exists.",
  created_at: datetime(),
  updated_at: datetime()
});

// ja-JP Lexicon
MATCH (l:Locale {key: "ja-JP"})
CREATE (l)-[:HAS_LEXICON]->(:LocaleLexicon {
  display_name: "Japanese Lexicon",
  description: "Lexicon rules for ja-JP",
  llm_context: "USE: for vocabulary/word choice decisions. TRIGGERS: 語彙, katakana, loanwords, 用語. NOT: tone or grammar.",
  loanwords_policy: "mixed",
  register_matching: true,
  style_notes: "Mix Japanese + English loanwords (katakana). Tech terms often in English.",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// SAMPLE EXPRESSIONS (for fr-FR Lexicon)
// =============================================================================

MATCH (l:Locale {key: "fr-FR"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_EXPRESSION]->(:Expression {
  display_name: "C'est parti !",
  description: "Expression for action semantic field",
  llm_context: "USE: for CTAs and action buttons. TRIGGERS: action, go, start, commencer. NOT: formal contexts.",
  semantic_field: "success",
  intention: "encouragement",
  text: "C'est parti !",
  register: "casual",
  context: "CTA buttons, onboarding",
  example_sentence: "Votre QR code est prêt. C'est parti !",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_EXPRESSION]->(:Expression {
  display_name: "En un clin d'œil",
  description: "Expression for speed semantic field",
  llm_context: "USE: to emphasize quick processes. TRIGGERS: fast, quick, instant, rapide. NOT: formal legal contexts.",
  semantic_field: "speed",
  intention: "reassurance",
  text: "En un clin d'œil",
  register: "semi-formal",
  context: "Process descriptions",
  example_sentence: "Créez votre QR code en un clin d'œil.",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_EXPRESSION]->(:Expression {
  display_name: "Sans prise de tête",
  description: "Expression for simplicity semantic field",
  llm_context: "USE: to emphasize ease of use. TRIGGERS: easy, simple, no hassle, facile. NOT: formal or professional contexts.",
  semantic_field: "simplicity",
  intention: "reassurance",
  text: "Sans prise de tête",
  register: "casual",
  context: "Feature descriptions",
  example_sentence: "Personnalisez vos codes sans prise de tête.",
  created_at: datetime(),
  updated_at: datetime()
});
