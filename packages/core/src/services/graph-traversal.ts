/**
 * Graph Traversal Service (v7.8.0)
 *
 * Implements spreading activation over the NovaNet concept graph.
 * Uses SEMANTIC_LINK temperature values for activation propagation.
 */

import type { Driver } from 'neo4j-driver';
import {
  loadSpreadingActivationConfig,
  getTaskModifier,
  calculateBoostedTemperature,
  matchesPriorityFilter,
  type SpreadingActivationConfig,
  type TaskModifier,
} from '../config/spreading-activation.js';
import type { TaskType, ActivatedConcept, Priority } from '../types/task-types.js';

// =============================================================================
// Types
// =============================================================================

export interface SpreadingActivationOptions {
  /** Task type for modifier selection (default: 'DEFAULT') */
  taskType?: TaskType;
  /** Override specific parameters */
  overrides?: Partial<TaskModifier>;
  /** Target locale for ConceptL10n retrieval */
  locale?: string;
}

export interface SpreadingActivationResult {
  /** Activated concepts sorted by activation score */
  concepts: ActivatedConcept[];
  /** Execution metadata */
  metadata: {
    /** Total execution time (ms) */
    durationMs: number;
    /** Number of seed concepts */
    seedCount: number;
    /** Total concepts activated (before threshold) */
    totalActivated: number;
    /** Concepts above output threshold */
    outputCount: number;
    /** Parameters used */
    parameters: {
      decayFactor: number;
      retentionFactor: number;
      propagationSteps: number;
      activationThreshold: number;
      outputThreshold: number;
    };
  };
}

// =============================================================================
// Service
// =============================================================================

export class GraphTraversalService {
  private config: SpreadingActivationConfig;

  constructor(private driver: Driver) {
    this.config = loadSpreadingActivationConfig();
  }

  /**
   * Perform spreading activation from seed concepts.
   *
   * Algorithm:
   * 1. Initialize seed concepts with activation = 1.0
   * 2. For T steps:
   *    - For each active concept:
   *      - Propagate to neighbors via SEMANTIC_LINK
   *      - New activation = parent_activation * temperature * (1 - ρ)
   *      - Apply fan-out penalty if many neighbors
   *    - Apply retention: activation *= δ
   *    - Filter by activation_threshold
   * 3. Return concepts above output_threshold
   *
   * @param seedConceptKeys - Starting concept keys
   * @param options - Spreading activation options
   * @returns Activated concepts with scores
   */
  async spreadingActivation(
    seedConceptKeys: string[],
    options: SpreadingActivationOptions = {}
  ): Promise<SpreadingActivationResult> {
    const startTime = Date.now();
    const { taskType = 'DEFAULT', overrides = {}, locale } = options;

    // Get task modifier and merge with overrides
    const taskModifier = getTaskModifier(taskType);
    const params = {
      decayFactor: this.config.decay_factor,
      retentionFactor: this.config.retention_factor,
      propagationSteps: overrides.propagation_steps ?? taskModifier.propagation_steps ?? this.config.propagation_steps,
      activationThreshold: overrides.activation_threshold ?? taskModifier.activation_threshold ?? this.config.activation_threshold,
      outputThreshold: this.config.output_threshold,
      maxFanOut: this.config.max_fan_out,
      fanPenalty: this.config.fan_penalty,
    };

    const session = this.driver.session();
    try {
      // Initialize activation map
      const activations = new Map<string, number>();
      const conceptData = new Map<string, Record<string, unknown>>();

      // Step 1: Load seed concepts
      const seedResult = await session.run(
        `
        MATCH (c:Concept)
        WHERE c.key IN $keys
        RETURN c.key AS key, c { .key, .display_name, .description, .llm_context, .priority } AS props
        `,
        { keys: seedConceptKeys }
      );

      for (const record of seedResult.records) {
        const key = record.get('key');
        activations.set(key, this.config.initial_activation);
        conceptData.set(key, record.get('props'));
      }

      // Step 2: Propagate for T steps
      for (let step = 0; step < params.propagationSteps; step++) {
        // Get current active concepts (above threshold)
        const activeKeys = [...activations.entries()]
          .filter(([, activation]) => activation >= params.activationThreshold)
          .map(([key]) => key);

        if (activeKeys.length === 0) break;

        // Propagate to neighbors
        const propagateResult = await session.run(
          `
          MATCH (c:Concept)-[r:SEMANTIC_LINK]->(neighbor:Concept)
          WHERE c.key IN $activeKeys
          WITH neighbor, c.key AS sourceKey, r.temperature AS baseTemp, r.type AS linkType,
               count { (c)-[:SEMANTIC_LINK]->() } AS fanOut
          RETURN neighbor.key AS key,
                 sourceKey,
                 baseTemp,
                 linkType,
                 fanOut,
                 neighbor { .key, .display_name, .description, .llm_context, .priority } AS props
          `,
          { activeKeys }
        );

        for (const record of propagateResult.records) {
          const neighborKey = record.get('key');
          const sourceKey = record.get('sourceKey');
          const baseTemp = record.get('baseTemp') as number;
          const linkType = record.get('linkType') as string;
          const fanOut = (record.get('fanOut') as { low: number }).low || 1;

          // Calculate boosted temperature based on task
          const boostedTemp = calculateBoostedTemperature(baseTemp, linkType, taskType);

          // Calculate new activation with decay and fan penalty
          const sourceActivation = activations.get(sourceKey) || 0;
          const fanPenalty = fanOut > params.maxFanOut
            ? 1 - (params.fanPenalty * (fanOut - params.maxFanOut) / fanOut)
            : 1;
          const newActivation = sourceActivation * boostedTemp * (1 - params.decayFactor) * fanPenalty;

          // Update activation (keep maximum)
          const currentActivation = activations.get(neighborKey) || 0;
          if (newActivation > currentActivation) {
            activations.set(neighborKey, newActivation);
            if (!conceptData.has(neighborKey)) {
              conceptData.set(neighborKey, record.get('props'));
            }
          }
        }

        // Apply retention factor to all activations
        for (const [key, activation] of activations) {
          activations.set(key, activation * params.retentionFactor);
        }
      }

      // Step 3: Filter by output threshold and priority
      const priorityFilter = taskModifier.priority_filter;
      const concepts: ActivatedConcept[] = [];

      for (const [key, activation] of activations) {
        if (activation < params.outputThreshold) continue;

        const props = conceptData.get(key);
        if (!props) continue;

        // Apply priority filter using numeric comparison
        const priority = props.priority as string;
        if (!matchesPriorityFilter(priority, priorityFilter)) continue;

        const concept: ActivatedConcept = {
          key,
          activation,
          source: seedConceptKeys.includes(key) ? 'seed' : 'spread',
          properties: {
            display_name: props.display_name as string,
            description: props.description as string,
            llm_context: props.llm_context as string,
            priority: props.priority as Priority,
          },
        };

        // Load ConceptL10n if locale specified
        if (locale) {
          const l10nResult = await session.run(
            `
            MATCH (c:Concept {key: $key})-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
            RETURN cl { .title, .definition, .summary, .purpose } AS l10n
            `,
            { key, locale }
          );
          if (l10nResult.records.length > 0) {
            concept.l10n = l10nResult.records[0].get('l10n');
          }
        }

        concepts.push(concept);
      }

      // Sort by activation descending
      concepts.sort((a, b) => b.activation - a.activation);

      return {
        concepts,
        metadata: {
          durationMs: Date.now() - startTime,
          seedCount: seedConceptKeys.length,
          totalActivated: activations.size,
          outputCount: concepts.length,
          parameters: {
            decayFactor: params.decayFactor,
            retentionFactor: params.retentionFactor,
            propagationSteps: params.propagationSteps,
            activationThreshold: params.activationThreshold,
            outputThreshold: params.outputThreshold,
          },
        },
      };
    } finally {
      await session.close();
    }
  }

  /**
   * Find related concepts using bidirectional traversal.
   * Uses inverse relationships for efficient traversal.
   *
   * @param conceptKey - Starting concept key
   * @param maxDepth - Maximum traversal depth (default: 2)
   * @returns Related concepts with path info
   */
  async findRelatedConcepts(
    conceptKey: string,
    maxDepth: number = 2
  ): Promise<{ key: string; depth: number; path: string[] }[]> {
    const session = this.driver.session();
    try {
      const result = await session.run(
        `
        MATCH path = (c:Concept {key: $key})-[:SEMANTIC_LINK*1..${maxDepth}]-(related:Concept)
        WHERE related.key <> $key
        WITH related, length(path) AS depth, [n IN nodes(path) | n.key] AS pathKeys
        RETURN DISTINCT related.key AS key, min(depth) AS depth, collect(pathKeys)[0] AS path
        ORDER BY depth, key
        `,
        { key: conceptKey }
      );

      return result.records.map((r) => ({
        key: r.get('key'),
        depth: (r.get('depth') as { low: number }).low,
        path: r.get('path'),
      }));
    } finally {
      await session.close();
    }
  }

  /**
   * Load locale context for content generation.
   *
   * @param locale - Locale key (e.g., "fr-FR")
   * @returns Locale context with voice, culture, and expressions
   */
  async loadLocaleContext(locale: string): Promise<{
    locale: string;
    voice?: Record<string, number>;
    culture?: Record<string, unknown>;
    expressions: Map<string, string[]>;
  }> {
    const session = this.driver.session();
    try {
      // Load voice
      const voiceResult = await session.run(
        `
        MATCH (l:Locale {key: $locale})-[:HAS_VOICE]->(v:LocaleVoice)
        RETURN v { .formality_score, .directness_score, .emotional_range } AS voice
        `,
        { locale }
      );
      const voice = voiceResult.records.length > 0
        ? voiceResult.records[0].get('voice')
        : undefined;

      // Load culture
      const cultureResult = await session.run(
        `
        MATCH (l:Locale {key: $locale})-[:HAS_CULTURE]->(c:LocaleCulture)
        RETURN c { .cultural_dimensions, .taboo_topics } AS culture
        `,
        { locale }
      );
      const culture = cultureResult.records.length > 0
        ? cultureResult.records[0].get('culture')
        : undefined;

      // Load expressions
      const expressionsResult = await session.run(
        `
        MATCH (l:Locale {key: $locale})-[:HAS_LEXICON]->(lex:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
        RETURN e.semantic_field AS field, collect(e.text) AS texts
        `,
        { locale }
      );

      const expressions = new Map<string, string[]>();
      for (const record of expressionsResult.records) {
        expressions.set(record.get('field'), record.get('texts'));
      }

      return { locale, voice, culture, expressions };
    } finally {
      await session.close();
    }
  }

  /**
   * Get concept hierarchy (for context understanding).
   *
   * @param conceptKey - Concept key
   * @returns Parent and child concepts
   */
  async getConceptHierarchy(conceptKey: string): Promise<{
    parents: string[];
    children: string[];
  }> {
    const session = this.driver.session();
    try {
      const result = await session.run(
        `
        MATCH (c:Concept {key: $key})
        OPTIONAL MATCH (c)-[:SEMANTIC_LINK {type: 'type_of'}]->(parent:Concept)
        OPTIONAL MATCH (c)<-[:SEMANTIC_LINK {type: 'type_of'}]-(child:Concept)
        RETURN collect(DISTINCT parent.key) AS parents, collect(DISTINCT child.key) AS children
        `,
        { key: conceptKey }
      );

      if (result.records.length === 0) {
        return { parents: [], children: [] };
      }

      return {
        parents: result.records[0].get('parents').filter(Boolean),
        children: result.records[0].get('children').filter(Boolean),
      };
    } finally {
      await session.close();
    }
  }
}
