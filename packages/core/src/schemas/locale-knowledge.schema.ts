// NovaNet Core - Zod Schemas for Locale Knowledge
// Runtime validation + TypeScript type inference

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

export const LocaleSchema = z.object({
  code: z.string().regex(/^[a-z]{2}-[A-Z]{2}$/, 'Must be BCP 47 format: xx-XX'),
  language_code: z.string().length(2),
  country_code: z.string().length(2),
  name_english: z.string().min(1),
  name_native: z.string().min(1),
  is_primary: z.boolean(),
  fallback_chain: z.array(z.string()),
  created_at: z.date(),
  updated_at: z.date(),
});

// =============================================================================
// LOCALE IDENTITY
// =============================================================================

export const LocaleIdentitySchema = z.object({
  // Script & Writing
  script_code: z.string(),
  script_name: z.string(),
  script_direction: z.enum(['ltr', 'rtl']),
  has_case: z.boolean(),
  special_characters: z.string(),
  diacritics: z.boolean(),
  ligatures: z.boolean(),

  // Geographic
  continent: z.string(),
  region: z.string(),
  capital: z.string(),
  timezone: z.string(),
  utc_offset: z.string(),
  dst_observed: z.boolean(),

  // Technical
  keyboard_layout: z.string(),
  encoding: z.string(),

  // Language family
  language_family: z.string(),
  related_languages: z.array(z.string()),

  created_at: z.date(),
  updated_at: z.date(),
});

// =============================================================================
// LOCALE VOICE
// =============================================================================

export const LocaleVoiceSchema = z.object({
  formality_score: z.number().min(0).max(100),
  default_formality: z.enum(['formal', 'casual', 'mixed']),
  default_pronoun: z.string().nullable(),
  pronoun_rules: z.record(z.unknown()),

  directness_score: z.number().min(0).max(100),
  directness_style: z.enum(['direct', 'indirect', 'balanced']),
  softening_patterns: z.record(z.string()),

  warmth_score: z.number().min(0).max(100),
  warmth_by_stage: z.record(z.number()),

  humor_score: z.number().min(0).max(100),
  humor_types: z.record(z.string()),

  avg_sentence_length: z.number().positive(),
  preferred_voice: z.enum(['active', 'passive', 'mixed']),
  rhythm_style: z.string(),

  punctuation_rules: z.record(z.string()),
  honorific_system: z.record(z.unknown()).optional(),

  created_at: z.date(),
  updated_at: z.date(),
});

// =============================================================================
// LOCALE CULTURE
// =============================================================================

const ValueItemSchema = z.object({
  value: z.string(),
  importance: z.string(),
  marketing_angle: z.string(),
});

const TriggerItemSchema = z.object({
  theme: z.string(),
  why: z.string(),
  example: z.string(),
});

const TabooItemSchema = z.object({
  topic: z.string(),
  severity: z.string(),
  notes: z.string(),
});

export const LocaleCultureSchema = z.object({
  dominant_values: z.array(ValueItemSchema),
  positive_triggers: z.array(TriggerItemSchema),
  national_pride: z.array(z.object({
    topic: z.string(),
    sensitivity: z.string(),
    notes: z.string(),
  })),

  context_level: z.enum(['high', 'medium', 'low']),
  hierarchy_sensitivity: z.enum(['high', 'medium', 'low']),

  taboo_topics: z.array(TabooItemSchema),
  historical_sensitivities: z.array(z.object({
    event: z.string(),
    sensitivity: z.string(),
    handling: z.string(),
  })),
  political_sensitivities: z.array(z.object({
    topic: z.string(),
    sensitivity: z.string(),
    safe_approach: z.string(),
  })),

  content_prohibitions: z.array(z.object({
    category: z.string(),
    restriction: z.string(),
    legal_basis: z.string(),
  })).optional(),

  restricted_imagery: z.array(z.object({
    type: z.string(),
    restriction: z.string(),
  })).optional(),

  gender_considerations: z.record(z.string()),
  age_norms: z.record(z.string()),
  time_norms: z.record(z.string()),

  cultural_phrases: z.array(z.object({
    phrase: z.string(),
    meaning: z.string(),
    when_to_use: z.string(),
  })).optional(),

  phrases_to_avoid: z.array(z.object({
    context: z.string(),
    avoid: z.string(),
    reason: z.string(),
  })).optional(),

  created_at: z.date(),
  updated_at: z.date(),
});

// =============================================================================
// LOCALE MARKET
// =============================================================================

export const LocaleMarketSchema = z.object({
  population: z.number().positive(),
  growth_rate: z.number(),
  median_age: z.number().positive(),
  age_distribution: z.array(z.object({
    group: z.string(),
    percentage: z.number(),
    notes: z.string(),
  })),
  income_levels: z.array(z.object({
    level: z.string(),
    percentage: z.number(),
    threshold: z.string(),
  })),
  urban_rural_split: z.record(z.number()),

  internet_penetration: z.number().min(0).max(100),
  mobile_penetration: z.number().min(0).max(100),
  mobile_first_users: z.number().min(0).max(100),
  dominant_os: z.record(z.number()),
  ecommerce_adoption: z.number().min(0).max(100),
  ecommerce_revenue: z.number(),

  payment_methods: z.array(z.object({
    method: z.string(),
    usage: z.number(),
    trend: z.string(),
  })),

  roi_score: z.number().min(0).max(100),
  roi_factors: z.record(z.number()),

  social_platforms: z.array(z.object({
    platform: z.string(),
    penetration: z.number(),
    audience: z.string(),
  })),
  messaging_apps: z.array(z.object({
    app: z.string(),
    penetration: z.number(),
    use_case: z.string(),
  })),
  search_engines: z.array(z.object({
    engine: z.string(),
    share: z.number(),
  })),

  avg_order_value: z.record(z.number()),
  conversion_rate: z.number(),
  cart_abandonment: z.number(),

  peak_periods: z.array(z.object({
    name: z.string(),
    months: z.string(),
    impact: z.string(),
  })),
  low_periods: z.array(z.object({
    name: z.string(),
    strategy: z.string(),
  })),
  shopping_events: z.array(z.object({
    event: z.string(),
    date: z.string(),
    impact: z.string(),
  })),

  major_players: z.array(z.object({
    company: z.string(),
    share: z.number(),
    strength: z.string(),
  })),
  market_concentration: z.enum(['fragmented', 'moderate', 'consolidated']),

  created_at: z.date(),
  updated_at: z.date(),
});

// =============================================================================
// LOCALE LEXICON & EXPRESSION
// =============================================================================

export const ExpressionSchema = z.object({
  semantic_field: z.string(),
  intention: z.string(),
  text: z.string(),
  register: z.enum(['formal', 'semi-formal', 'casual']),
  context: z.string(),
  example_sentence: z.string(),
  created_at: z.date(),
  updated_at: z.date(),
});

export const LocaleLexiconSchema = z.object({
  expression_density: z.string(),
  rotation_rule: z.string(),
  register_matching: z.boolean(),

  loanwords_policy: z.enum(['native_only', 'mixed', 'english_ok']),
  accepted_loanwords: z.array(z.object({
    word: z.string(),
    context: z.string(),
  })),
  prefer_native: z.array(z.object({
    loanword: z.string(),
    native: z.string(),
    when: z.string(),
  })),

  connectors: z.record(z.record(z.string())),

  unique_concepts: z.array(z.object({
    expression: z.string(),
    meaning: z.string(),
    when: z.string(),
  })),
  common_idioms: z.array(z.object({
    idiom: z.string(),
    meaning: z.string(),
    context: z.string(),
  })),

  created_at: z.date(),
  updated_at: z.date(),
});

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
