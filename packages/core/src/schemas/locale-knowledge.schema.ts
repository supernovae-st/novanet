/**
 * @fileoverview NovaNet Locale Knowledge Schemas
 * @module @novanet/core/schemas/locale-knowledge
 * @version 11.6.0
 *
 * Zod validation schemas for locale-specific knowledge in the NovaNet knowledge graph.
 * These schemas define the structure for locale identity, voice, culture, market data,
 * and lexicon resources used in native content generation.
 *
 * **Schema Hierarchy:**
 * - `LocaleSchema`: Core locale identification (BCP-47 codes, fallback chains)
 * - `LocaleIdentitySchema`: Script, geography, timezone, language family
 * - `LocaleVoiceSchema`: Tone, formality, communication style
 * - `LocaleCultureSchema`: Values, taboos, sensitivities
 * - `LocaleMarketSchema`: Demographics, digital adoption, commerce
 * - `LocaleLexiconSchema`: Expressions, idioms, loanwords policy
 *
 * @example
 * ```typescript
 * import { LocaleSchema, LocaleVoiceSchema } from '@novanet/core/schemas/locale-knowledge';
 *
 * const locale = LocaleSchema.parse({
 *   code: 'fr-FR',
 *   language_code: 'fr',
 *   country_code: 'FR',
 *   name_english: 'French (France)',
 *   name_native: 'Français (France)',
 *   is_primary: true,
 *   fallback_chain: ['fr-BE', 'en-US'],
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 *
 * @see ADR-007 — Generation, Not Translation
 * @see packages/core/models/node-kinds/shared/locale/ — Locale YAML definitions
 */

import { z } from 'zod';

// =============================================================================
// COMMON ENUMS (REMOVED v8.2.0 - YAML v7.11.0 alignment)
// =============================================================================

// REMOVED v8.2.0: Priority and Freshness never implemented at YAML level
// export const PrioritySchema = z.enum(['critical', 'high', 'medium', 'low']);
// export const FreshnessSchema = z.enum(['realtime', 'hourly', 'daily', 'static']);
// export type Priority = z.infer<typeof PrioritySchema>;
// export type Freshness = z.infer<typeof FreshnessSchema>;

// =============================================================================
// LOCALE
// =============================================================================

/**
 * Core locale identification schema.
 *
 * Defines the fundamental properties of a locale using BCP-47 codes.
 * Each locale has a unique code (e.g., 'fr-FR') and optional fallback chain
 * for content resolution when locale-specific content is unavailable.
 *
 * @example
 * ```typescript
 * const locale = LocaleSchema.parse({
 *   code: 'ja-JP',
 *   language_code: 'ja',
 *   country_code: 'JP',
 *   name_english: 'Japanese (Japan)',
 *   name_native: '日本語',
 *   is_primary: true,
 *   fallback_chain: ['en-US'],
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 */
export const LocaleSchema = z.object({
  code: z.string()
    .regex(/^[a-z]{2}-[A-Z]{2}$/, 'Must be BCP 47 format: xx-XX')
    .describe('BCP-47 locale code (e.g., fr-FR, ja-JP)'),
  language_code: z.string()
    .length(2)
    .describe('ISO 639-1 language code (e.g., fr, ja)'),
  country_code: z.string()
    .length(2)
    .describe('ISO 3166-1 alpha-2 country code (e.g., FR, JP)'),
  name_english: z.string()
    .min(1)
    .describe('English name of the locale (e.g., French (France))'),
  name_native: z.string()
    .min(1)
    .describe('Native name of the locale (e.g., Français (France))'),
  is_primary: z.boolean()
    .describe('Whether this is the primary locale for the language'),
  fallback_chain: z.array(z.string())
    .describe('Ordered list of fallback locale codes for content resolution'),
  created_at: z.date()
    .describe('Timestamp when the locale was created'),
  updated_at: z.date()
    .describe('Timestamp when the locale was last updated'),
}).describe('Core locale identification with BCP-47 codes and fallback chain');

// =============================================================================
// LOCALE IDENTITY
// =============================================================================

/**
 * Locale identity schema with script, geography, and technical details.
 *
 * Contains comprehensive metadata about the locale including writing system,
 * geographic information, timezone, keyboard layout, and language family.
 *
 * @example
 * ```typescript
 * const identity = LocaleIdentitySchema.parse({
 *   script_code: 'Latn',
 *   script_name: 'Latin',
 *   script_direction: 'ltr',
 *   has_case: true,
 *   // ... other properties
 * });
 * ```
 */
export const LocaleIdentitySchema = z.object({
  // Script & Writing
  script_code: z.string()
    .describe('ISO 15924 script code (e.g., Latn, Cyrl, Arab)'),
  script_name: z.string()
    .describe('Human-readable script name (e.g., Latin, Cyrillic)'),
  script_direction: z.enum(['ltr', 'rtl'])
    .describe('Text direction: left-to-right or right-to-left'),
  has_case: z.boolean()
    .describe('Whether the script has uppercase/lowercase distinction'),
  special_characters: z.string()
    .describe('Special characters used in this locale'),
  diacritics: z.boolean()
    .describe('Whether the locale uses diacritical marks'),
  ligatures: z.boolean()
    .describe('Whether the locale uses ligatures'),

  // Geographic
  continent: z.string()
    .describe('Continent where the locale is primarily used'),
  region: z.string()
    .describe('Geographic region (e.g., Western Europe)'),
  capital: z.string()
    .describe('Capital city of the country'),
  timezone: z.string()
    .describe('Primary timezone identifier (e.g., Europe/Paris)'),
  utc_offset: z.string()
    .describe('UTC offset (e.g., +01:00)'),
  dst_observed: z.boolean()
    .describe('Whether daylight saving time is observed'),

  // Technical
  keyboard_layout: z.string()
    .describe('Standard keyboard layout (e.g., AZERTY, QWERTY)'),
  encoding: z.string()
    .describe('Character encoding (typically UTF-8)'),

  // Language family
  language_family: z.string()
    .describe('Language family (e.g., Indo-European, Sino-Tibetan)'),
  related_languages: z.array(z.string())
    .describe('List of closely related languages'),

  created_at: z.date()
    .describe('Timestamp when the identity was created'),
  updated_at: z.date()
    .describe('Timestamp when the identity was last updated'),
}).describe('Locale identity with script, geography, timezone, and language family');

// =============================================================================
// LOCALE VOICE
// =============================================================================

/**
 * Locale voice schema defining communication style and tone.
 *
 * Captures the preferred communication style for a locale including
 * formality levels, directness, warmth, and punctuation conventions.
 * Used by LLM to generate culturally appropriate content.
 *
 * @example
 * ```typescript
 * const voice = LocaleVoiceSchema.parse({
 *   formality_score: 75,
 *   default_formality: 'formal',
 *   directness_score: 40,
 *   directness_style: 'indirect',
 *   // ... other properties
 * });
 * ```
 */
export const LocaleVoiceSchema = z.object({
  formality_score: z.number()
    .min(0).max(100)
    .describe('Formality level from 0 (very casual) to 100 (very formal)'),
  default_formality: z.enum(['formal', 'casual', 'mixed'])
    .describe('Default formality register for content'),
  default_pronoun: z.string()
    .nullable()
    .describe('Default pronoun form (e.g., vous for French formal)'),
  pronoun_rules: z.record(z.unknown())
    .describe('Pronoun usage rules by context'),

  directness_score: z.number()
    .min(0).max(100)
    .describe('Directness level from 0 (very indirect) to 100 (very direct)'),
  directness_style: z.enum(['direct', 'indirect', 'balanced'])
    .describe('Preferred communication directness'),
  softening_patterns: z.record(z.string())
    .describe('Patterns for softening direct statements'),

  warmth_score: z.number()
    .min(0).max(100)
    .describe('Warmth level from 0 (distant) to 100 (warm)'),
  warmth_by_stage: z.record(z.number())
    .describe('Warmth levels by customer journey stage'),

  humor_score: z.number()
    .min(0).max(100)
    .describe('Humor acceptance from 0 (no humor) to 100 (very humorous)'),
  humor_types: z.record(z.string())
    .describe('Appropriate humor types by context'),

  avg_sentence_length: z.number()
    .positive()
    .describe('Preferred average sentence length in words'),
  preferred_voice: z.enum(['active', 'passive', 'mixed'])
    .describe('Preferred grammatical voice'),
  rhythm_style: z.string()
    .describe('Preferred sentence rhythm and pacing'),

  punctuation_rules: z.record(z.string())
    .describe('Locale-specific punctuation conventions'),
  honorific_system: z.record(z.unknown())
    .optional()
    .describe('Honorific system rules if applicable'),

  created_at: z.date()
    .describe('Timestamp when the voice was created'),
  updated_at: z.date()
    .describe('Timestamp when the voice was last updated'),
}).describe('Communication style, tone, formality, and punctuation for a locale');

// =============================================================================
// LOCALE CULTURE
// =============================================================================

/**
 * Schema for cultural values used in LocaleCultureSchema.
 */
const ValueItemSchema = z.object({
  value: z.string()
    .describe('Cultural value name'),
  importance: z.string()
    .describe('Importance level in the culture'),
  marketing_angle: z.string()
    .describe('How to leverage this value in marketing'),
}).describe('Cultural value with importance and marketing angle');

/**
 * Schema for positive emotional triggers.
 */
const TriggerItemSchema = z.object({
  theme: z.string()
    .describe('Theme that triggers positive response'),
  why: z.string()
    .describe('Why this theme resonates'),
  example: z.string()
    .describe('Example of effective usage'),
}).describe('Positive emotional trigger with context');

/**
 * Schema for taboo topics.
 */
const TabooItemSchema = z.object({
  topic: z.string()
    .describe('Topic to avoid'),
  severity: z.string()
    .describe('Severity level (critical, high, moderate)'),
  notes: z.string()
    .describe('Additional context and exceptions'),
}).describe('Taboo topic with severity and notes');

/**
 * Locale culture schema defining values, taboos, and sensitivities.
 *
 * Captures cultural nuances essential for native content generation including
 * dominant values, positive triggers, national pride points, taboos, and
 * various sensitivities that content must navigate.
 *
 * @example
 * ```typescript
 * const culture = LocaleCultureSchema.parse({
 *   dominant_values: [{ value: 'Quality', importance: 'high', marketing_angle: 'Craftsmanship' }],
 *   context_level: 'high',
 *   // ... other properties
 * });
 * ```
 */
export const LocaleCultureSchema = z.object({
  dominant_values: z.array(ValueItemSchema)
    .describe('Core cultural values with marketing angles'),
  positive_triggers: z.array(TriggerItemSchema)
    .describe('Themes that evoke positive emotional responses'),
  national_pride: z.array(z.object({
    topic: z.string().describe('Source of national pride'),
    sensitivity: z.string().describe('Sensitivity level'),
    notes: z.string().describe('Usage guidance'),
  })).describe('National pride points and handling guidelines'),

  context_level: z.enum(['high', 'medium', 'low'])
    .describe('High-context (implicit) vs low-context (explicit) communication'),
  hierarchy_sensitivity: z.enum(['high', 'medium', 'low'])
    .describe('Sensitivity to social hierarchy'),

  taboo_topics: z.array(TabooItemSchema)
    .describe('Topics to avoid in content'),
  historical_sensitivities: z.array(z.object({
    event: z.string().describe('Historical event'),
    sensitivity: z.string().describe('Sensitivity level'),
    handling: z.string().describe('How to handle references'),
  })).describe('Historical events requiring careful handling'),
  political_sensitivities: z.array(z.object({
    topic: z.string().describe('Political topic'),
    sensitivity: z.string().describe('Sensitivity level'),
    safe_approach: z.string().describe('Safe approach guidelines'),
  })).describe('Political topics requiring caution'),

  content_prohibitions: z.array(z.object({
    category: z.string().describe('Prohibited content category'),
    restriction: z.string().describe('Nature of restriction'),
    legal_basis: z.string().describe('Legal or regulatory basis'),
  })).optional().describe('Legally prohibited content categories'),

  restricted_imagery: z.array(z.object({
    type: z.string().describe('Image type'),
    restriction: z.string().describe('Restriction details'),
  })).optional().describe('Imagery restrictions'),

  gender_considerations: z.record(z.string())
    .describe('Gender-related content considerations'),
  age_norms: z.record(z.string())
    .describe('Age-related norms and expectations'),
  time_norms: z.record(z.string())
    .describe('Time-related cultural norms'),

  cultural_phrases: z.array(z.object({
    phrase: z.string().describe('Cultural phrase or expression'),
    meaning: z.string().describe('Meaning and connotation'),
    when_to_use: z.string().describe('Appropriate usage context'),
  })).optional().describe('Cultural phrases to incorporate'),

  phrases_to_avoid: z.array(z.object({
    context: z.string().describe('Context where phrase is problematic'),
    avoid: z.string().describe('Phrase to avoid'),
    reason: z.string().describe('Reason for avoidance'),
  })).optional().describe('Phrases to avoid in content'),

  created_at: z.date()
    .describe('Timestamp when the culture was created'),
  updated_at: z.date()
    .describe('Timestamp when the culture was last updated'),
}).describe('Cultural values, taboos, sensitivities, and content guidelines');

// =============================================================================
// LOCALE MARKET
// =============================================================================

/**
 * Locale market schema with demographics and digital adoption.
 *
 * Contains comprehensive market data including population demographics,
 * digital penetration, e-commerce metrics, social platforms, and
 * seasonal patterns essential for market strategy.
 *
 * @example
 * ```typescript
 * const market = LocaleMarketSchema.parse({
 *   population: 67000000,
 *   internet_penetration: 92,
 *   ecommerce_adoption: 78,
 *   // ... other properties
 * });
 * ```
 */
export const LocaleMarketSchema = z.object({
  population: z.number()
    .positive()
    .describe('Total population'),
  growth_rate: z.number()
    .describe('Annual population growth rate'),
  median_age: z.number()
    .positive()
    .describe('Median age of population'),
  age_distribution: z.array(z.object({
    group: z.string().describe('Age group range'),
    percentage: z.number().describe('Percentage of population'),
    notes: z.string().describe('Behavioral notes for group'),
  })).describe('Population distribution by age groups'),
  income_levels: z.array(z.object({
    level: z.string().describe('Income level category'),
    percentage: z.number().describe('Percentage of population'),
    threshold: z.string().describe('Income threshold'),
  })).describe('Income distribution'),
  urban_rural_split: z.record(z.number())
    .describe('Urban vs rural population percentages'),

  internet_penetration: z.number()
    .min(0).max(100)
    .describe('Percentage of population with internet access'),
  mobile_penetration: z.number()
    .min(0).max(100)
    .describe('Percentage of population with mobile devices'),
  mobile_first_users: z.number()
    .min(0).max(100)
    .describe('Percentage who primarily use mobile'),
  dominant_os: z.record(z.number())
    .describe('Operating system market share'),
  ecommerce_adoption: z.number()
    .min(0).max(100)
    .describe('Percentage who shop online'),
  ecommerce_revenue: z.number()
    .describe('Total e-commerce revenue in local currency'),

  payment_methods: z.array(z.object({
    method: z.string().describe('Payment method name'),
    usage: z.number().describe('Usage percentage'),
    trend: z.string().describe('Growth trend'),
  })).describe('Preferred payment methods'),

  roi_score: z.number()
    .min(0).max(100)
    .describe('Overall ROI potential score'),
  roi_factors: z.record(z.number())
    .describe('ROI factor breakdown'),

  social_platforms: z.array(z.object({
    platform: z.string().describe('Platform name'),
    penetration: z.number().describe('User penetration percentage'),
    audience: z.string().describe('Primary audience demographics'),
  })).describe('Social media platform usage'),
  messaging_apps: z.array(z.object({
    app: z.string().describe('Messaging app name'),
    penetration: z.number().describe('User penetration percentage'),
    use_case: z.string().describe('Primary use case'),
  })).describe('Messaging app usage'),
  search_engines: z.array(z.object({
    engine: z.string().describe('Search engine name'),
    share: z.number().describe('Market share percentage'),
  })).describe('Search engine market share'),

  avg_order_value: z.record(z.number())
    .describe('Average order value by category'),
  conversion_rate: z.number()
    .describe('Average e-commerce conversion rate'),
  cart_abandonment: z.number()
    .describe('Cart abandonment rate'),

  peak_periods: z.array(z.object({
    name: z.string().describe('Period name'),
    months: z.string().describe('Active months'),
    impact: z.string().describe('Sales impact'),
  })).describe('Peak shopping periods'),
  low_periods: z.array(z.object({
    name: z.string().describe('Period name'),
    strategy: z.string().describe('Recommended strategy'),
  })).describe('Low shopping periods'),
  shopping_events: z.array(z.object({
    event: z.string().describe('Shopping event name'),
    date: z.string().describe('Event date(s)'),
    impact: z.string().describe('Sales impact'),
  })).describe('Major shopping events'),

  major_players: z.array(z.object({
    company: z.string().describe('Company name'),
    share: z.number().describe('Market share'),
    strength: z.string().describe('Competitive strength'),
  })).describe('Major market players'),
  market_concentration: z.enum(['fragmented', 'moderate', 'consolidated'])
    .describe('Market concentration level'),

  created_at: z.date()
    .describe('Timestamp when the market data was created'),
  updated_at: z.date()
    .describe('Timestamp when the market data was last updated'),
}).describe('Market demographics, digital adoption, e-commerce, and seasonal patterns');

// =============================================================================
// LOCALE LEXICON & EXPRESSION
// =============================================================================

/**
 * Expression schema for locale-specific phrases.
 *
 * Defines individual expressions with their semantic field, register,
 * and usage context for native content generation.
 *
 * @example
 * ```typescript
 * const expression = ExpressionSchema.parse({
 *   semantic_field: 'greeting',
 *   intention: 'welcome',
 *   text: 'Bienvenue chez nous',
 *   register: 'semi-formal',
 *   context: 'business welcome',
 *   example_sentence: 'Bienvenue chez nous, nous sommes ravis de vous accueillir.',
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 */
export const ExpressionSchema = z.object({
  semantic_field: z.string()
    .describe('Semantic category (greeting, farewell, thanks, etc.)'),
  intention: z.string()
    .describe('Communicative intention'),
  text: z.string()
    .describe('The expression text'),
  register: z.enum(['formal', 'semi-formal', 'casual'])
    .describe('Formality register'),
  context: z.string()
    .describe('Appropriate usage context'),
  example_sentence: z.string()
    .describe('Example sentence using the expression'),
  created_at: z.date()
    .describe('Timestamp when the expression was created'),
  updated_at: z.date()
    .describe('Timestamp when the expression was last updated'),
}).describe('Locale-specific expression with register and context');

/**
 * Locale lexicon schema with expressions and loanword policies.
 *
 * Defines the lexical resources for a locale including expression density,
 * loanword handling, connectors, idioms, and unique cultural concepts.
 *
 * @example
 * ```typescript
 * const lexicon = LocaleLexiconSchema.parse({
 *   expression_density: 'high',
 *   loanwords_policy: 'mixed',
 *   // ... other properties
 * });
 * ```
 */
export const LocaleLexiconSchema = z.object({
  expression_density: z.string()
    .describe('How densely to use locale-specific expressions'),
  rotation_rule: z.string()
    .describe('Rule for rotating expressions to avoid repetition'),
  register_matching: z.boolean()
    .describe('Whether to match expression register to content register'),

  loanwords_policy: z.enum(['native_only', 'mixed', 'english_ok'])
    .describe('Policy for using loanwords vs native terms'),
  accepted_loanwords: z.array(z.object({
    word: z.string().describe('Accepted loanword'),
    context: z.string().describe('Acceptable usage context'),
  })).describe('Loanwords acceptable in this locale'),
  prefer_native: z.array(z.object({
    loanword: z.string().describe('Loanword to avoid'),
    native: z.string().describe('Preferred native equivalent'),
    when: z.string().describe('When to use native form'),
  })).describe('Loanwords with preferred native alternatives'),

  connectors: z.record(z.record(z.string()))
    .describe('Connector phrases by category and register'),

  unique_concepts: z.array(z.object({
    expression: z.string().describe('Unique cultural expression'),
    meaning: z.string().describe('Meaning and connotation'),
    when: z.string().describe('When to use'),
  })).describe('Untranslatable cultural concepts'),
  common_idioms: z.array(z.object({
    idiom: z.string().describe('Idiomatic expression'),
    meaning: z.string().describe('Meaning'),
    context: z.string().describe('Usage context'),
  })).describe('Common idioms for this locale'),

  created_at: z.date()
    .describe('Timestamp when the lexicon was created'),
  updated_at: z.date()
    .describe('Timestamp when the lexicon was last updated'),
}).describe('Lexical resources including expressions, loanwords, and idioms');

// =============================================================================
// TYPE EXPORTS (inferred from Zod schemas)
// =============================================================================

export type Locale = z.infer<typeof LocaleSchema>;
export type LocaleIdentity = z.infer<typeof LocaleIdentitySchema>;
export type LocaleVoice = z.infer<typeof LocaleVoiceSchema>;
export type LocaleCulture = z.infer<typeof LocaleCultureSchema>;
export type LocaleMarket = z.infer<typeof LocaleMarketSchema>;
export type LocaleLexicon = z.infer<typeof LocaleLexiconSchema>;
export type Expression = z.infer<typeof ExpressionSchema>;
