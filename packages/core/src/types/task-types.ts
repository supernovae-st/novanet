/**
 * Task Types for NovaNet Generation (v7.8.0)
 *
 * Defines task types that influence spreading activation behavior
 * and context retrieval for content generation.
 */

// =============================================================================
// Task Type Enum
// =============================================================================

/**
 * Content generation task types.
 * Each type has specific spreading activation parameters.
 */
export type TaskType = 'CTA' | 'FAQ' | 'HERO' | 'PRICING' | 'TESTIMONIAL' | 'DEFAULT';

/**
 * All available task types.
 */
export const TASK_TYPES: readonly TaskType[] = [
  'CTA',
  'FAQ',
  'HERO',
  'PRICING',
  'TESTIMONIAL',
  'DEFAULT',
] as const;

// =============================================================================
// Semantic Boost Types
// =============================================================================

/**
 * Semantic link types that can receive boost modifiers.
 */
export type SemanticType =
  | 'urgency'
  | 'value'
  | 'action'
  | 'comparison'
  | 'explanation'
  | 'social_proof'
  | 'feature'
  | 'benefit'
  | 'pricing'
  | 'default';

/**
 * Boost multipliers for semantic link types.
 * Values > 1.0 increase activation, < 1.0 decrease.
 */
export type SemanticBoosts = Partial<Record<SemanticType, number>>;

// =============================================================================
// Priority Types
// =============================================================================

/**
 * Concept priority levels for filtering.
 */
export type Priority = 'critical' | 'high' | 'medium' | 'low';

/**
 * All priority levels in order.
 */
export const PRIORITIES: readonly Priority[] = ['critical', 'high', 'medium', 'low'] as const;

// =============================================================================
// Task Modifier Types
// =============================================================================

/**
 * Task-specific parameters that modify spreading activation behavior.
 */
export interface TaskModifier {
  /** Minimum activation required to include a concept */
  activation_threshold?: number;
  /** Number of propagation steps */
  propagation_steps?: number;
  /** Semantic type boost multipliers */
  semantic_boosts?: SemanticBoosts;
  /** Filter concepts by priority */
  priority_filter?: Priority[];
}

/**
 * Static task modifiers (for use when config is not available).
 * These are the default values - prefer loading from spreading-activation.yaml.
 */
export const TASK_MODIFIERS: Record<TaskType, TaskModifier> = {
  CTA: {
    activation_threshold: 0.25,
    propagation_steps: 2,
    semantic_boosts: {
      urgency: 1.3,
      value: 1.2,
      action: 1.15,
    },
    priority_filter: ['critical', 'high'],
  },
  FAQ: {
    activation_threshold: 0.15,
    propagation_steps: 3,
    semantic_boosts: {
      explanation: 1.3,
      comparison: 1.2,
    },
  },
  HERO: {
    activation_threshold: 0.2,
    propagation_steps: 2,
    semantic_boosts: {
      value: 1.4,
      benefit: 1.3,
      feature: 1.2,
    },
    priority_filter: ['critical', 'high'],
  },
  PRICING: {
    activation_threshold: 0.2,
    propagation_steps: 2,
    semantic_boosts: {
      pricing: 1.5,
      value: 1.3,
      comparison: 1.2,
    },
  },
  TESTIMONIAL: {
    activation_threshold: 0.2,
    propagation_steps: 2,
    semantic_boosts: {
      social_proof: 1.4,
      benefit: 1.2,
    },
  },
  DEFAULT: {
    activation_threshold: 0.3,
    propagation_steps: 2,
    semantic_boosts: {},
  },
};

// =============================================================================
// Generation Context Types
// =============================================================================

/**
 * Context retrieved for a generation task.
 */
export interface GenerationContext {
  /** Task type being generated */
  taskType: TaskType;
  /** Target locale */
  locale: string;
  /** Activated concepts with scores */
  concepts: ActivatedConcept[];
  /** Locale knowledge context */
  localeContext: LocaleContext;
}

/**
 * A concept activated through spreading activation or vector search.
 */
export interface ActivatedConcept {
  /** Concept key */
  key: string;
  /** Activation score (0-1) */
  activation: number;
  /** Source of activation */
  source: 'seed' | 'spread' | 'vector' | 'hybrid';
  /** Concept properties */
  properties: {
    display_name?: string;
    description?: string;
    llm_context?: string;
    priority?: Priority;
  };
  /** Localized content if available */
  l10n?: {
    title: string;
    definition?: string;
    summary?: string;
    purpose?: string;
  };
}

/**
 * Locale-specific context for generation.
 */
export interface LocaleContext {
  /** Locale key (e.g., "fr-FR") */
  locale: string;
  /** Voice characteristics */
  voice?: {
    formality_score: number;
    directness_score: number;
    emotional_range: number;
  };
  /** Cultural norms */
  culture?: {
    cultural_dimensions: Record<string, number>;
    taboo_topics: string[];
  };
  /** Available expressions by semantic field */
  expressions?: Map<string, string[]>;
}

// =============================================================================
// Retrieval Request Types
// =============================================================================

/**
 * Request for hybrid context retrieval.
 */
export interface RetrievalRequest {
  /** Task type for modifier selection */
  taskType: TaskType;
  /** Target locale */
  locale: string;
  /** Seed concepts to start spreading activation */
  seedConcepts: string[];
  /** Optional query for vector search */
  query?: string;
  /** Override default parameters */
  overrides?: Partial<TaskModifier>;
}

/**
 * Result from hybrid retrieval.
 */
export interface RetrievalResult {
  /** All activated concepts */
  concepts: ActivatedConcept[];
  /** Retrieval metadata */
  metadata: {
    /** Time taken for retrieval (ms) */
    durationMs: number;
    /** Number of concepts from spreading activation */
    spreadCount: number;
    /** Number of concepts from vector search */
    vectorCount: number;
    /** Number of concepts from both (merged) */
    hybridCount: number;
    /** Task modifier used */
    taskModifier: TaskModifier;
  };
}
