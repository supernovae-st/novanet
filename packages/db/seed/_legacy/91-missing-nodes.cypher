// NovaNet Missing Nodes Seed v8.2.0
//
// Creates 9 missing node types with standard properties:
//   - PageType: Template type for pages
//   - LocaleRulesAdaptation: Content adaptation rules
//   - LocaleRulesFormatting: Date/number formatting rules
//   - LocaleRulesSlug: URL slug generation rules
//   - LocaleCultureReferences: Cultural references container
//   - Reference: Specific cultural reference
//   - Metaphor: Cultural metaphor
//   - Pattern: Reusable formatting pattern
//   - Constraint: Cultural constraint
//
// Standard v8.2.0 properties: key, display_name, description, llm_context, created_at, updated_at

// =============================================================================
// PAGE TYPE - Template types for pages
// =============================================================================

CREATE (:PageType {
  key: "landing",
  display_name: "Landing Page",
  description: "High-conversion landing page template with hero, features, and CTA sections",
  llm_context: "USE: for marketing pages optimized for conversion. TRIGGERS: landing, conversion, hero, CTA. INCLUDES: hero section, value props, social proof, final CTA.",
  created_at: datetime(),
  updated_at: datetime()
});

CREATE (:PageType {
  key: "pricing",
  display_name: "Pricing Page",
  description: "Pricing comparison page with tier breakdown and feature matrix",
  llm_context: "USE: for pricing/plan comparison pages. TRIGGERS: pricing, plans, tiers, comparison. INCLUDES: tier cards, feature comparison, FAQ, money-back guarantee.",
  created_at: datetime(),
  updated_at: datetime()
});

CREATE (:PageType {
  key: "blog",
  display_name: "Blog Post",
  description: "Long-form content template optimized for SEO and readability",
  llm_context: "USE: for blog articles and content marketing. TRIGGERS: blog, article, post, content. INCLUDES: headline, intro, body sections, conclusion, related posts.",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// LOCALE RULES - ADAPTATION
// =============================================================================

MATCH (l:Locale {key: "en-US"})
CREATE (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation {
  key: "en-US-adaptation",
  display_name: "US English Adaptation Rules",
  description: "Content adaptation rules for American English",
  llm_context: "USE: when adapting content for US audience. TRIGGERS: localize, adapt, Americanize. RULES: use US spelling (color, center), imperial units, MM/DD/YYYY dates.",
  spelling_variant: "US",
  measurement_system: "imperial",
  date_order: "MDY",
  number_grouping: ",",
  decimal_separator: ".",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})
CREATE (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation {
  key: "fr-FR-adaptation",
  display_name: "French Adaptation Rules",
  description: "Content adaptation rules for French (France)",
  llm_context: "USE: when adapting content for French audience. TRIGGERS: localiser, adapter, franciser. RULES: metric units, DD/MM/YYYY, space before punctuation (!?;:).",
  spelling_variant: "FR",
  measurement_system: "metric",
  date_order: "DMY",
  number_grouping: " ",
  decimal_separator: ",",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "ja-JP"})
CREATE (l)-[:HAS_RULES_ADAPTATION]->(:LocaleRulesAdaptation {
  key: "ja-JP-adaptation",
  display_name: "Japanese Adaptation Rules",
  description: "Content adaptation rules for Japanese",
  llm_context: "USE: when adapting content for Japanese audience. TRIGGERS: localize, adapt, 日本語化. RULES: YYYY/MM/DD dates, metric, full-width punctuation, keigo levels.",
  spelling_variant: "JP",
  measurement_system: "metric",
  date_order: "YMD",
  number_grouping: ",",
  decimal_separator: ".",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// LOCALE RULES - FORMATTING
// =============================================================================

MATCH (l:Locale {key: "en-US"})
CREATE (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting {
  key: "en-US-formatting",
  display_name: "US English Formatting Rules",
  description: "Date, number, and currency formatting for en-US",
  llm_context: "USE: for formatting dates/numbers/currency. TRIGGERS: format, display, render. FORMAT: $1,234.56, 12/31/2024, 2:30 PM.",
  date_format: "MM/DD/YYYY",
  time_format: "h:mm A",
  currency_format: "$#,##0.00",
  currency_position: "before",
  percentage_format: "#0.0%",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})
CREATE (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting {
  key: "fr-FR-formatting",
  display_name: "French Formatting Rules",
  description: "Date, number, and currency formatting for fr-FR",
  llm_context: "USE: for formatting dates/numbers/currency. TRIGGERS: format, afficher. FORMAT: 1 234,56 EUR, 31/12/2024, 14h30.",
  date_format: "DD/MM/YYYY",
  time_format: "HH[h]mm",
  currency_format: "#,##0.00 EUR",
  currency_position: "after",
  percentage_format: "#0,0 %",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "ja-JP"})
CREATE (l)-[:HAS_RULES_FORMATTING]->(:LocaleRulesFormatting {
  key: "ja-JP-formatting",
  display_name: "Japanese Formatting Rules",
  description: "Date, number, and currency formatting for ja-JP",
  llm_context: "USE: for formatting dates/numbers/currency. TRIGGERS: format, 表示. FORMAT: ¥1,234, 2024年12月31日, 14:30.",
  date_format: "YYYY年MM月DD日",
  time_format: "HH:mm",
  currency_format: "¥#,##0",
  currency_position: "before",
  percentage_format: "#0.0%",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// LOCALE RULES - SLUG
// =============================================================================

MATCH (l:Locale {key: "en-US"})
CREATE (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug {
  key: "en-US-slug",
  display_name: "US English Slug Rules",
  description: "URL slug generation rules for en-US",
  llm_context: "USE: for URL slug generation. TRIGGERS: slug, URL, permalink. RULES: lowercase, hyphens, no stop words (the, a, an), max 60 chars.",
  transliteration: "none",
  word_separator: "-",
  case_style: "lowercase",
  max_length: 60,
  stop_words: ["the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for"],
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})
CREATE (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug {
  key: "fr-FR-slug",
  display_name: "French Slug Rules",
  description: "URL slug generation rules for fr-FR",
  llm_context: "USE: for URL slug generation. TRIGGERS: slug, URL, permalien. RULES: remove accents (é->e), hyphens, remove articles (le, la, les).",
  transliteration: "remove_accents",
  word_separator: "-",
  case_style: "lowercase",
  max_length: 60,
  stop_words: ["le", "la", "les", "un", "une", "des", "de", "du", "et", "ou", "mais"],
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "ja-JP"})
CREATE (l)-[:HAS_RULES_SLUG]->(:LocaleRulesSlug {
  key: "ja-JP-slug",
  display_name: "Japanese Slug Rules",
  description: "URL slug generation rules for ja-JP",
  llm_context: "USE: for URL slug generation. TRIGGERS: slug, URL, パーマリンク. RULES: romaji transliteration or English equivalent, hyphens.",
  transliteration: "romaji",
  word_separator: "-",
  case_style: "lowercase",
  max_length: 60,
  stop_words: [],
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// LOCALE CULTURE REFERENCES - Container for cultural references
// =============================================================================

MATCH (l:Locale {key: "en-US"})
MATCH (lc:LocaleCulture)<-[:HAS_CULTURE]-(l)
CREATE (lc)-[:HAS_REFERENCE]->(:LocaleCultureReferences {
  key: "en-US-culture-refs",
  display_name: "US Cultural References",
  description: "Container for American cultural references used in content",
  llm_context: "USE: for culturally resonant content. TRIGGERS: reference, allusion, cultural. CONTAINS: sports (NFL, NBA), holidays (Thanksgiving), pop culture.",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})
MATCH (lc:LocaleCulture)<-[:HAS_CULTURE]-(l)
CREATE (lc)-[:HAS_REFERENCE]->(:LocaleCultureReferences {
  key: "fr-FR-culture-refs",
  display_name: "French Cultural References",
  description: "Container for French cultural references used in content",
  llm_context: "USE: for culturally resonant content. TRIGGERS: référence, allusion, culturel. CONTAINS: gastronomy, art de vivre, literature, cinema.",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "ja-JP"})
MATCH (lc:LocaleCulture)<-[:HAS_CULTURE]-(l)
CREATE (lc)-[:HAS_REFERENCE]->(:LocaleCultureReferences {
  key: "ja-JP-culture-refs",
  display_name: "Japanese Cultural References",
  description: "Container for Japanese cultural references used in content",
  llm_context: "USE: for culturally resonant content. TRIGGERS: 文化的参照, reference, 日本文化. CONTAINS: seasons (sakura, momiji), traditions, anime/manga.",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// REFERENCE - Specific cultural references
// =============================================================================

MATCH (lcr:LocaleCultureReferences {key: "en-US-culture-refs"})
CREATE (lcr)-[:HAS_REFERENCE]->(:Reference {
  key: "ref-super-bowl",
  display_name: "Super Bowl",
  description: "Annual NFL championship game, major cultural event",
  llm_context: "USE: for sports analogies, peak performance. TRIGGERS: championship, big game, victory. CONTEXT: February, ads, halftime show.",
  category: "sports",
  resonance_level: "high",
  usage_context: "competition, achievement, celebration",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (lcr:LocaleCultureReferences {key: "fr-FR-culture-refs"})
CREATE (lcr)-[:HAS_REFERENCE]->(:Reference {
  key: "ref-tour-de-france",
  display_name: "Tour de France",
  description: "Annual cycling race, national sporting event",
  llm_context: "USE: for endurance, journey analogies. TRIGGERS: course, étape, maillot jaune. CONTEXT: July, summer, perseverance.",
  category: "sports",
  resonance_level: "high",
  usage_context: "journey, stages, achievement",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (lcr:LocaleCultureReferences {key: "ja-JP-culture-refs"})
CREATE (lcr)-[:HAS_REFERENCE]->(:Reference {
  key: "ref-sakura",
  display_name: "Sakura (Cherry Blossoms)",
  description: "Cherry blossom season, symbol of renewal and transience",
  llm_context: "USE: for new beginnings, beauty, fleeting moments. TRIGGERS: 桜, spring, renewal, hanami. CONTEXT: March-April, new fiscal year.",
  category: "nature",
  resonance_level: "high",
  usage_context: "new beginnings, beauty, impermanence",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// METAPHOR - Cultural metaphors
// =============================================================================

MATCH (l:Locale {key: "en-US"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_METAPHOR]->(:Metaphor {
  key: "metaphor-home-run",
  display_name: "Hit a home run",
  description: "Baseball metaphor for complete success",
  llm_context: "USE: for major success, exceeding expectations. TRIGGERS: success, achievement, scoring. AVOID: audiences unfamiliar with baseball.",
  source_domain: "baseball",
  target_domain: "success",
  text: "hit a home run",
  example_sentence: "With this feature, you'll hit a home run every time.",
  register: "casual",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_METAPHOR]->(:Metaphor {
  key: "metaphor-piece-montee",
  display_name: "La cerise sur le gâteau",
  description: "French metaphor for the perfect finishing touch",
  llm_context: "USE: for added bonus, perfect complement. TRIGGERS: bonus, extra, finishing touch. EQUIVALENT: cherry on top.",
  source_domain: "pastry",
  target_domain: "bonus",
  text: "la cerise sur le gâteau",
  example_sentence: "Et la cerise sur le gâteau : l'export en PDF illimité.",
  register: "semi-formal",
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "ja-JP"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_METAPHOR]->(:Metaphor {
  key: "metaphor-ichigo-ichie",
  display_name: "一期一会 (Ichigo Ichie)",
  description: "Japanese concept of treasuring each encounter",
  llm_context: "USE: for unique opportunities, special moments. TRIGGERS: unique, once-in-a-lifetime, special. CONTEXT: formal, meaningful interactions.",
  source_domain: "tea ceremony",
  target_domain: "opportunity",
  text: "一期一会",
  example_sentence: "一期一会の出会いを大切に。",
  register: "formal",
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// PATTERN - Reusable formatting patterns
// =============================================================================

MATCH (l:Locale {key: "en-US"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_PATTERN]->(:Pattern {
  key: "pattern-cta-action",
  display_name: "Action CTA Pattern",
  description: "Pattern for action-oriented CTA buttons",
  llm_context: "USE: for CTA buttons. TRIGGERS: button, CTA, action. PATTERN: [Verb] + [Object] + [Benefit]. EXAMPLE: 'Start your free trial'.",
  pattern_type: "CTA",
  template: "[Verb] [Object] [Benefit]",
  examples: ["Start your free trial", "Get instant access", "Create your first QR code"],
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_PATTERN]->(:Pattern {
  key: "pattern-cta-action-fr",
  display_name: "Pattern CTA Action",
  description: "Pattern for action-oriented CTA buttons in French",
  llm_context: "USE: for CTA buttons. TRIGGERS: bouton, CTA, action. PATTERN: [Verbe infinitif] + [Objet]. EXAMPLE: 'Créer mon QR code'.",
  pattern_type: "CTA",
  template: "[Verbe infinitif] [Possessif] [Objet]",
  examples: ["Créer mon QR code", "Démarrer l'essai gratuit", "Découvrir les fonctionnalités"],
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "ja-JP"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_PATTERN]->(:Pattern {
  key: "pattern-cta-action-ja",
  display_name: "CTAアクションパターン",
  description: "Pattern for action-oriented CTA buttons in Japanese",
  llm_context: "USE: for CTA buttons. TRIGGERS: ボタン, CTA, アクション. PATTERN: [Object]を[Verb]. EXAMPLE: 'QRコードを作成する'.",
  pattern_type: "CTA",
  template: "[Object]を[Verb]",
  examples: ["QRコードを作成する", "無料で始める", "今すぐ試す"],
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// CONSTRAINT - Cultural constraints
// =============================================================================

MATCH (l:Locale {key: "en-US"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_CONSTRAINT]->(:Constraint {
  key: "constraint-us-legal",
  display_name: "US Legal Disclaimers",
  description: "Legal constraints for US marketing content",
  llm_context: "USE: when generating marketing copy. TRIGGERS: legal, disclaimer, terms. CONSTRAINT: include 'Terms apply' for offers, avoid absolute claims without proof.",
  constraint_type: "legal",
  applies_to: ["marketing", "pricing", "offers"],
  rules: ["Include 'Terms apply' for promotional offers", "Avoid unsubstantiated superlatives", "Disclose material conditions"],
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "fr-FR"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_CONSTRAINT]->(:Constraint {
  key: "constraint-fr-legal",
  display_name: "French Legal Constraints",
  description: "Legal constraints for French marketing content",
  llm_context: "USE: when generating marketing copy. TRIGGERS: légal, mentions, RGPD. CONSTRAINT: RGPD compliance, prix TTC, conditions générales.",
  constraint_type: "legal",
  applies_to: ["marketing", "pricing", "data"],
  rules: ["Prix TTC obligatoire", "Mentions RGPD pour données personnelles", "Conditions générales accessibles"],
  created_at: datetime(),
  updated_at: datetime()
});

MATCH (l:Locale {key: "ja-JP"})-[:HAS_LEXICON]->(ll:LocaleLexicon)
CREATE (ll)-[:HAS_CONSTRAINT]->(:Constraint {
  key: "constraint-ja-politeness",
  display_name: "Japanese Politeness Constraints",
  description: "Politeness level constraints for Japanese content",
  llm_context: "USE: when generating Japanese content. TRIGGERS: 敬語, politeness, keigo. CONSTRAINT: maintain consistent keigo level, avoid mixing です/ます with である.",
  constraint_type: "linguistic",
  applies_to: ["all content"],
  rules: ["Consistent keigo level throughout", "です/ます for customer-facing", "Avoid mixing politeness registers"],
  created_at: datetime(),
  updated_at: datetime()
});
