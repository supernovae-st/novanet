// novanet-core/src/types/shared.ts
// Shared realm types v8.2.0 (was locale-knowledge.ts, renamed v11.3)
//
// v8.2.0 CHANGES:
//   - REMOVED: icon, priority, freshness from all interfaces (YAML v7.11.0 alignment)
//   - Standard properties now: key, display_name, description, llm_context, created_at, updated_at
//
// v7.11.0 STANDARD PROPERTIES (all nodes):
//   key, display_name, description, llm_context, created_at, updated_at

// REMOVED v8.2.0: Priority and Freshness types (never implemented, YAGNI)
// export type Priority = 'critical' | 'high' | 'medium' | 'low';
// export type Freshness = 'realtime' | 'hourly' | 'daily' | 'static';

export interface Locale {
  // Standard properties (v8.2.0)
  key: string;               // BCP 47: "fr-FR"
  display_name: string;      // "French (France)"
  description: string;       // "French locale for France market"
  llm_context: string;       // "USE: French content. TRIGGERS: fr-FR. NOT: Canadian French."

  // Locale-specific
  language_code: string;     // ISO 639-1: "fr"
  country_code: string;      // ISO 3166-1: "FR"
  name_native: string;
  is_primary: boolean;
  fallback_chain: string[];
  created_at: Date;
  updated_at: Date;
}

export interface LocaleIdentity {
  // Standard properties (v8.2.0 - no key, linked via HAS_IDENTITY)
  display_name: string;      // "French Identity"
  description: string;       // "Identity characteristics for fr-FR"
  llm_context: string;       // "USE: script/encoding decisions."

  // Script & Writing
  script_code: string;
  script_name: string;
  script_direction: 'ltr' | 'rtl';
  has_case: boolean;
  special_characters: string;
  diacritics: boolean;
  ligatures: boolean;

  // Geographic
  continent: string;
  region: string;
  capital: string;
  timezone: string;
  utc_offset: string;
  dst_observed: boolean;

  // Technical
  keyboard_layout: string;
  encoding: string;

  // Language family
  language_family: string;
  related_languages: string[];

  created_at: Date;
  updated_at: Date;
}

export interface LocaleVoice {
  // Standard properties (v8.2.0 - no key, linked via HAS_VOICE)
  display_name: string;      // "French Voice"
  description: string;       // "Voice characteristics for fr-FR"
  llm_context: string;       // "USE: tone/formality decisions."

  // Voice characteristics
  formality_score: number;      // 0-100
  default_formality: 'formal' | 'casual' | 'mixed';
  default_pronoun: string | null;
  pronoun_rules: Record<string, unknown>;

  directness_score: number;
  directness_style: 'direct' | 'indirect' | 'balanced';
  softening_patterns: Record<string, string>;

  warmth_score: number;
  warmth_by_stage: Record<string, number>;

  humor_score: number;
  humor_types: Record<string, string>;

  avg_sentence_length: number;
  preferred_voice: 'active' | 'passive' | 'mixed';
  rhythm_style: string;

  punctuation_rules: Record<string, string>;
  honorific_system?: Record<string, unknown>;

  created_at: Date;
  updated_at: Date;
}

export interface LocaleCulture {
  // Standard properties (v8.2.0 - no key, linked via HAS_CULTURE)
  display_name: string;      // "French Culture"
  description: string;       // "Cultural norms for fr-FR"
  llm_context: string;       // "USE: cultural sensitivity."

  // Culture characteristics
  dominant_values: Array<{ value: string; importance: string; marketing_angle: string }>;
  positive_triggers: Array<{ theme: string; why: string; example: string }>;
  national_pride: Array<{ topic: string; sensitivity: string; notes: string }>;

  context_level: 'high' | 'medium' | 'low';
  hierarchy_sensitivity: 'high' | 'medium' | 'low';

  taboo_topics: Array<{ topic: string; severity: string; notes: string }>;
  historical_sensitivities: Array<{ event: string; sensitivity: string; handling: string }>;
  political_sensitivities: Array<{ topic: string; sensitivity: string; safe_approach: string }>;

  content_prohibitions?: Array<{ category: string; restriction: string; legal_basis: string }>;
  restricted_imagery?: Array<{ type: string; restriction: string }>;

  gender_considerations: Record<string, string>;
  age_norms: Record<string, string>;
  time_norms: Record<string, string>;

  cultural_phrases?: Array<{ phrase: string; meaning: string; when_to_use: string }>;
  phrases_to_avoid?: Array<{ context: string; avoid: string; reason: string }>;

  created_at: Date;
  updated_at: Date;
}

export interface LocaleMarket {
  // Standard properties (v8.2.0 - no key, linked via HAS_MARKET)
  display_name: string;      // "French Market"
  description: string;       // "Market data for fr-FR"
  llm_context: string;       // "USE: market positioning."

  // Market characteristics
  population: number;
  growth_rate: number;
  median_age: number;
  age_distribution: Array<{ group: string; percentage: number; notes: string }>;
  income_levels: Array<{ level: string; percentage: number; threshold: string }>;
  urban_rural_split: Record<string, number>;

  internet_penetration: number;
  mobile_penetration: number;
  mobile_first_users: number;
  dominant_os: Record<string, number>;
  ecommerce_adoption: number;
  ecommerce_revenue: number;

  payment_methods: Array<{ method: string; usage: number; trend: string }>;

  roi_score: number;
  roi_factors: Record<string, number>;

  social_platforms: Array<{ platform: string; penetration: number; audience: string }>;
  messaging_apps: Array<{ app: string; penetration: number; use_case: string }>;
  search_engines: Array<{ engine: string; share: number }>;

  avg_order_value: Record<string, number>;
  conversion_rate: number;
  cart_abandonment: number;

  peak_periods: Array<{ name: string; months: string; impact: string }>;
  low_periods: Array<{ name: string; strategy: string }>;
  shopping_events: Array<{ event: string; date: string; impact: string }>;

  major_players: Array<{ company: string; share: number; strength: string }>;
  market_concentration: 'fragmented' | 'moderate' | 'consolidated';

  created_at: Date;
  updated_at: Date;
}

export interface LocaleLexicon {
  // Standard properties (v8.2.0 - no key, linked via HAS_LEXICON)
  display_name: string;      // "French Lexicon"
  description: string;       // "Lexicon rules for fr-FR"
  llm_context: string;       // "USE: vocabulary choices."

  // Lexicon characteristics
  expression_density: string;
  rotation_rule: string;
  register_matching: boolean;

  loanwords_policy: 'native_only' | 'mixed' | 'english_ok';
  accepted_loanwords: Array<{ word: string; context: string }>;
  prefer_native: Array<{ loanword: string; native: string; when: string }>;

  connectors: Record<string, Record<string, string>>;

  unique_concepts: Array<{ expression: string; meaning: string; when: string }>;
  common_idioms: Array<{ idiom: string; meaning: string; context: string }>;

  created_at: Date;
  updated_at: Date;
}

export interface Expression {
  // Standard properties (v8.2.0 - no key, linked via HAS_EXPRESSION)
  display_name: string;      // "Gratuit"
  description: string;       // "Expression for value semantic field"
  llm_context: string;       // "USE: expression selection."

  // Expression-specific
  semantic_field: string;
  intention: string;
  text: string;
  register: 'formal' | 'semi-formal' | 'casual';
  context: string;
  example_sentence: string;
  created_at: Date;
  updated_at: Date;
}

// ═══════════════════════════════════════════════════════════════════════════════
// NEW: LocaleRulesAdaptation - FACTS vs ILLUSTRATIONS framework
// ═══════════════════════════════════════════════════════════════════════════════

export interface LocaleRulesAdaptation {
  // Core framework
  facts_categories: Array<{
    category: string;
    examples: string;
    treatment: string;
  }>;
  illustrations_categories: Array<{
    category: string;
    examples: string;
    treatment: string;
  }>;

  // Priority hierarchy
  priority_levels: Array<{
    level: number;
    name: string;
    items: string[];
    rule: string;
  }>;

  // Decision algorithm (stored as text for LLM)
  decision_algorithm: string;

  // Locale-specific parameters
  formality_baseline: 'formal' | 'casual' | 'mixed';
  formality_rules: Array<{ context: string; formality: string; notes: string }>;
  measurement_system: 'metric' | 'imperial' | 'mixed';
  measurement_exceptions: Array<{ category: string; unit: string; notes: string }>;
  technical_terms_approach: 'native_only' | 'mixed' | 'english_ok';
  french_preferred_terms: Array<{ english: string; native: string }>;
  english_accepted_terms: string[];

  // Calendar context
  hemisphere: 'northern' | 'southern';
  work_week: string;
  major_shopping_events: Array<{ name: string; date: string; notes: string }>;
  cultural_calendar: Array<{ event: string; date: string; significance: string }>;

  // Common errors
  common_errors: Array<{
    error: string;
    why_wrong: string;
    correct: string;
  }>;

  // Validation checklist (for LLM)
  validation_checklist: string;

  created_at: Date;
  updated_at: Date;
}

// ═══════════════════════════════════════════════════════════════════════════════
// NEW: LocaleRulesFormatting - Date/Number/Currency formatting rules
// ═══════════════════════════════════════════════════════════════════════════════

export interface LocaleRulesFormatting {
  // Number formatting
  decimal_separator: string;
  thousands_separator: string;
  negative_sign: string;
  grouping_pattern: number;
  number_examples_correct: string[];
  number_examples_incorrect: string[];

  // Date formatting
  date_pattern: string;
  date_pattern_short: string;
  date_pattern_long: string;
  date_pattern_full: string;
  date_separator: string;
  month_names: Record<string, string>;
  month_names_short: Record<string, string>;
  day_names: Record<string, string>;
  day_names_short: Record<string, string>;

  // Time formatting
  time_system: '12-hour' | '24-hour';
  time_pattern: string;
  time_separator: string;
  am_indicator: string | null;
  pm_indicator: string | null;

  // Currency formatting
  currency_code: string;
  currency_symbol: string;
  currency_symbol_position: 'before' | 'after';
  currency_decimal_places: number;
  currency_examples_correct: string[];
  currency_examples_incorrect: string[];

  // Phone formatting
  phone_country_code: string;
  phone_national_pattern: string;
  phone_international_pattern: string;
  phone_mobile_prefixes: string[];
  phone_landline_prefixes: string[];
  phone_examples_correct: string[];

  // Address formatting
  address_pattern: string;
  postal_code_pattern: string;
  postal_code_position: 'before' | 'after';
  city_format: 'uppercase' | 'titlecase' | 'lowercase';
  address_example: string;

  // Measurement
  temperature_unit: 'celsius' | 'fahrenheit';
  distance_unit: 'km' | 'miles';
  weight_unit: 'kg' | 'lbs';
  volume_unit: 'liters' | 'gallons';

  // Percentage & misc
  percentage_space_before: boolean;

  // Validation patterns (regex)
  validation_patterns: Record<string, string>;

  created_at: Date;
  updated_at: Date;
}

// ═══════════════════════════════════════════════════════════════════════════════
// NEW: LocaleRulesSlug - URL slug generation rules
// ═══════════════════════════════════════════════════════════════════════════════

export interface LocaleRulesSlug {
  // Base rule
  slug_rule: 'latin_preserve' | 'latin_strip' | 'transliterate' | 'script_specific';
  output_encoding: string;

  // Character handling
  diacritics_handling: 'preserve' | 'strip' | 'transliterate';
  case_handling: 'lowercase' | 'preserve';
  space_replacement: string;
  special_chars_handling: 'removed' | 'transliterate';

  // Character mapping (for transliteration)
  character_mapping: Record<string, string>;

  // Stopwords
  stopwords: string[];
  stopwords_locale_additions: string[];

  // Validation
  max_length: number;
  min_length: number;
  double_hyphens_allowed: boolean;
  leading_trailing_hyphens_allowed: boolean;

  // Examples
  slug_examples: Array<{
    input: string;
    output: string;
    rules_applied: string;
  }>;

  // Implementation code (for reference)
  implementation_code: string;

  created_at: Date;
  updated_at: Date;
}

// ═══════════════════════════════════════════════════════════════════════════════
// NEW: LocaleCultureReferences - Concrete cultural references
// ═══════════════════════════════════════════════════════════════════════════════

export interface LocaleCultureReferences {
  // Food & Cuisine
  emblematic_dishes: Array<{
    dish: string;
    recognition: number;
    usage_context: string;
    symbolism: string;
  }>;
  food_metaphors: Array<{
    concept: string;
    local_reference: string;
    example_usage: string;
  }>;
  dining_culture: Array<{
    aspect: string;
    local_practice: string;
    notes: string;
  }>;

  // Sports
  dominant_sports: Array<{
    sport: string;
    recognition: number;
    usage_context: string;
    season: string;
  }>;
  teams_events: Array<{
    name: string;
    recognition: number;
    symbolism: string;
  }>;
  sports_metaphors: Array<{
    concept: string;
    local_expression: string;
    source_sport: string;
  }>;

  // Geography
  iconic_places: Array<{
    place: string;
    recognition: number;
    usage_context: string;
    symbolism: string;
  }>;
  size_comparisons: Array<{
    comparison_type: string;
    local_reference: string;
    equivalent: string;
  }>;
  regional_distinctions: Array<{
    region: string;
    stereotype: string;
    usage_notes: string;
  }>;

  // Brands
  local_champions: Array<{
    brand: string;
    sector: string;
    recognition: number;
    usage_context: string;
  }>;
  global_brands_presence: Array<{
    brand: string;
    local_status: string;
    local_perception: string;
  }>;

  // Personalities
  business_leaders: Array<{
    name: string;
    domain: string;
    recognition: number;
    usage_context: string;
  }>;
  cultural_icons: Array<{
    name: string;
    domain: string;
    recognition: number;
    symbolism: string;
  }>;
  sports_figures: Array<{
    name: string;
    sport: string;
    recognition: number;
    status: string;
  }>;

  // Events & Holidays
  major_holidays: Array<{
    holiday: string;
    date: string;
    significance: string;
    commercial_impact: string;
  }>;
  shopping_events: Array<{
    event: string;
    date: string;
    type: string;
    notes: string;
  }>;
  cultural_events: Array<{
    event: string;
    when: string;
    recognition: number;
    commercial_value: string;
  }>;

  // Media & Entertainment
  tv_streaming: Array<{
    name: string;
    type: string;
    recognition: number;
    audience: string;
  }>;
  music: Array<{
    artist_genre: string;
    recognition: number;
    usage_context: string;
  }>;
  cinema: Array<{
    film_franchise: string;
    recognition: number;
    cultural_impact: string;
  }>;

  created_at: Date;
  updated_at: Date;
}

// Export all new types
export type LocaleKnowledgeNode =
  | Locale
  | LocaleIdentity
  | LocaleVoice
  | LocaleCulture
  | LocaleMarket
  | LocaleLexicon
  | Expression
  | LocaleRulesAdaptation
  | LocaleRulesFormatting
  | LocaleRulesSlug
  | LocaleCultureReferences;
