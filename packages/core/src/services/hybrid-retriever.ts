/**
 * Hybrid Retriever Service (v7.8.0)
 *
 * Combines vector search + graph traversal (spreading activation) for
 * context-aware semantic retrieval. Implements the OntologyRAG pattern.
 *
 * Pipeline:
 * 1. Vector search → top-K semantic matches
 * 2. Graph traversal → spreading activation from seeds
 * 3. Merge & rank → deduplicate, apply schema constraints
 * 4. Return ranked context for LLM generation
 */

import type { Driver } from 'neo4j-driver';
import { VectorSearchService, type VectorSearchOptions } from './vector-search.js';
import { GraphTraversalService, type SpreadingActivationOptions, type SpreadingActivationResult } from './graph-traversal.js';
import type { TaskType, Priority } from '../types/task-types.js';

// =============================================================================
// Types
// =============================================================================

export interface HybridRetrievalOptions {
  /** Task type for spreading activation modifiers */
  taskType?: TaskType;
  /** Target locale for ConceptL10n retrieval */
  locale?: string;
  /** Vector search options */
  vector?: {
    /** Number of vector search results (default: 10) */
    limit?: number;
    /** Minimum similarity threshold (default: 0.7) */
    threshold?: number;
  };
  /** Graph traversal options */
  graph?: {
    /** Maximum propagation steps (default: from task modifier) */
    propagationSteps?: number;
    /** Activation threshold (default: from task modifier) */
    activationThreshold?: number;
  };
  /** Merge strategy */
  merge?: {
    /** Weight for vector scores (0-1, default: 0.6) */
    vectorWeight?: number;
    /** Weight for graph activation (0-1, default: 0.4) */
    graphWeight?: number;
    /** Maximum results after merge (default: 20) */
    maxResults?: number;
    /** Minimum combined score (default: 0.3) */
    minScore?: number;
  };
}

export interface HybridMatch {
  /** Concept key */
  key: string;
  /** Combined score (vectorScore * weight + graphScore * weight) */
  score: number;
  /** Source of match */
  source: 'vector' | 'graph' | 'hybrid';
  /** Vector similarity score (if from vector search) */
  vectorScore?: number;
  /** Graph activation score (if from spreading activation) */
  graphScore?: number;
  /** Concept properties */
  properties: {
    display_name?: string;
    description?: string;
    llm_context?: string;
    priority?: Priority;
  };
  /** Localized content (if locale specified) */
  l10n?: {
    title: string;
    definition?: string;
    summary?: string;
    purpose?: string;
  };
}

export interface HybridRetrievalResult {
  /** Ranked matches */
  matches: HybridMatch[];
  /** Execution metadata */
  metadata: {
    /** Total execution time (ms) */
    durationMs: number;
    /** Vector search stats */
    vector: {
      count: number;
      durationMs: number;
    };
    /** Graph traversal stats */
    graph: {
      seedCount: number;
      totalActivated: number;
      outputCount: number;
      durationMs: number;
    };
    /** Merge stats */
    merge: {
      beforeDedupe: number;
      afterDedupe: number;
      finalCount: number;
    };
    /** Parameters used */
    parameters: {
      taskType: TaskType;
      locale?: string;
      vectorWeight: number;
      graphWeight: number;
    };
  };
}

// =============================================================================
// Service
// =============================================================================

export class HybridRetriever {
  private vectorService: VectorSearchService;
  private graphService: GraphTraversalService;

  constructor(private driver: Driver) {
    this.vectorService = new VectorSearchService(driver);
    this.graphService = new GraphTraversalService(driver);
  }

  /**
   * Perform hybrid retrieval combining vector search and graph traversal.
   *
   * @param query - Natural language query
   * @param options - Retrieval options
   * @returns Ranked context matches
   */
  async retrieve(
    query: string,
    options: HybridRetrievalOptions = {}
  ): Promise<HybridRetrievalResult> {
    const startTime = Date.now();

    const {
      taskType = 'DEFAULT',
      locale,
      vector = {},
      graph = {},
      merge = {},
    } = options;

    const vectorWeight = merge.vectorWeight ?? 0.6;
    const graphWeight = merge.graphWeight ?? 0.4;
    const maxResults = merge.maxResults ?? 20;
    const minScore = merge.minScore ?? 0.3;

    // Step 1: Vector search
    const vectorStart = Date.now();
    const vectorOptions: VectorSearchOptions = {
      limit: vector.limit ?? 10,
      threshold: vector.threshold ?? 0.7,
      locale,
    };
    const vectorResults = await this.vectorService.searchConcepts(query, vectorOptions);
    const vectorDuration = Date.now() - vectorStart;

    // Step 2: Spreading activation from vector matches
    const graphStart = Date.now();
    const seedKeys = vectorResults.map(r => r.key);
    const graphOptions: SpreadingActivationOptions = {
      taskType,
      locale,
      overrides: {
        propagation_steps: graph.propagationSteps,
        activation_threshold: graph.activationThreshold,
      },
    };

    let graphResult: SpreadingActivationResult;
    if (seedKeys.length > 0) {
      graphResult = await this.graphService.spreadingActivation(seedKeys, graphOptions);
    } else {
      graphResult = {
        concepts: [],
        metadata: {
          durationMs: 0,
          seedCount: 0,
          totalActivated: 0,
          outputCount: 0,
          parameters: {
            decayFactor: 0,
            retentionFactor: 0,
            propagationSteps: 0,
            activationThreshold: 0,
            outputThreshold: 0,
          },
        },
      };
    }
    const graphDuration = Date.now() - graphStart;

    // Step 3: Merge results
    const mergedMap = new Map<string, HybridMatch>();

    // Add vector results
    for (const vr of vectorResults) {
      mergedMap.set(vr.key, {
        key: vr.key,
        score: vr.score * vectorWeight,
        source: 'vector',
        vectorScore: vr.score,
        properties: {
          display_name: vr.properties.display_name as string,
          description: vr.properties.description as string,
          llm_context: vr.properties.llm_context as string,
          priority: vr.properties.priority as Priority,
        },
      });
    }

    // Add/merge graph results
    for (const gr of graphResult.concepts) {
      const existing = mergedMap.get(gr.key);
      if (existing) {
        // Hybrid: combine scores
        existing.score = (existing.vectorScore ?? 0) * vectorWeight + gr.activation * graphWeight;
        existing.graphScore = gr.activation;
        existing.source = 'hybrid';
        existing.l10n = gr.l10n;
      } else {
        // Graph-only result
        mergedMap.set(gr.key, {
          key: gr.key,
          score: gr.activation * graphWeight,
          source: 'graph',
          graphScore: gr.activation,
          properties: gr.properties,
          l10n: gr.l10n,
        });
      }
    }

    // Step 4: Filter, sort, limit
    const beforeDedupe = vectorResults.length + graphResult.concepts.length;
    const afterDedupe = mergedMap.size;

    let matches = Array.from(mergedMap.values())
      .filter(m => m.score >= minScore)
      .sort((a, b) => b.score - a.score)
      .slice(0, maxResults);

    // Normalize scores to 0-1 range
    if (matches.length > 0) {
      const maxScore = matches[0].score;
      if (maxScore > 0) {
        matches = matches.map(m => ({
          ...m,
          score: m.score / maxScore,
        }));
      }
    }

    return {
      matches,
      metadata: {
        durationMs: Date.now() - startTime,
        vector: {
          count: vectorResults.length,
          durationMs: vectorDuration,
        },
        graph: {
          seedCount: graphResult.metadata.seedCount,
          totalActivated: graphResult.metadata.totalActivated,
          outputCount: graphResult.metadata.outputCount,
          durationMs: graphDuration,
        },
        merge: {
          beforeDedupe,
          afterDedupe,
          finalCount: matches.length,
        },
        parameters: {
          taskType,
          locale,
          vectorWeight,
          graphWeight,
        },
      },
    };
  }

  /**
   * Retrieve context for a specific block generation task.
   * Optimized for content generation with locale context.
   *
   * @param blockKey - Block key
   * @param locale - Target locale
   * @param taskType - Block task type (CTA, FAQ, HERO, etc.)
   * @returns Context for LLM generation
   */
  async retrieveForBlock(
    blockKey: string,
    locale: string,
    taskType: TaskType
  ): Promise<{
    concepts: HybridMatch[];
    localeContext: Awaited<ReturnType<GraphTraversalService['loadLocaleContext']>>;
  }> {
    const session = this.driver.session();

    try {
      // Get concepts used by this block
      const blockResult = await session.run(
        `
        MATCH (b:Block {key: $blockKey})-[:USES_CONCEPT]->(c:Concept)
        RETURN c.key AS key, c.llm_context AS context
        `,
        { blockKey }
      );

      const conceptKeys = blockResult.records.map(r => r.get('key') as string);
      const conceptContext = blockResult.records.map(r => r.get('context') as string).join(' ');

      // Hybrid retrieval using block concepts as query
      const retrievalResult = await this.retrieve(conceptContext, {
        taskType,
        locale,
        vector: { limit: 5, threshold: 0.6 },
        graph: { propagationSteps: 2 },
        merge: { maxResults: 15 },
      });

      // Boost concepts directly used by block
      for (const match of retrievalResult.matches) {
        if (conceptKeys.includes(match.key)) {
          match.score = Math.min(1.0, match.score * 1.5);
        }
      }

      // Re-sort after boost
      retrievalResult.matches.sort((a, b) => b.score - a.score);

      // Load locale context
      const localeContext = await this.graphService.loadLocaleContext(locale);

      return {
        concepts: retrievalResult.matches,
        localeContext,
      };
    } finally {
      await session.close();
    }
  }

  /**
   * Retrieve context for page orchestration.
   * Returns high-level concepts for coordinating block generation.
   *
   * @param pageKey - Page key
   * @param locale - Target locale
   * @returns Context for orchestrator
   */
  async retrieveForPage(
    pageKey: string,
    locale: string
  ): Promise<{
    concepts: HybridMatch[];
    blocks: Array<{ key: string; type: string; position: number }>;
  }> {
    const session = this.driver.session();

    try {
      // Get page with blocks
      const pageResult = await session.run(
        `
        MATCH (p:Page {key: $pageKey})
        OPTIONAL MATCH (p)-[:USES_CONCEPT]->(c:Concept)
        OPTIONAL MATCH (p)-[r:HAS_BLOCK]->(b:Block)-[:OF_TYPE]->(bt:BlockType)
        RETURN p.llm_context AS context,
               collect(DISTINCT c.key) AS conceptKeys,
               collect(DISTINCT {key: b.key, type: bt.key, position: r.position}) AS blocks
        `,
        { pageKey }
      );

      if (pageResult.records.length === 0) {
        return { concepts: [], blocks: [] };
      }

      const record = pageResult.records[0];
      const context = record.get('context') as string || '';
      const conceptKeys = record.get('conceptKeys') as string[];
      const blocks = (record.get('blocks') as Array<{ key: string; type: string; position: { low: number } }>)
        .filter(b => b.key)
        .map(b => ({
          key: b.key,
          type: b.type,
          position: b.position?.low ?? 0,
        }))
        .sort((a, b) => a.position - b.position);

      // Hybrid retrieval for page context
      const retrievalResult = await this.retrieve(context, {
        taskType: 'DEFAULT',
        locale,
        vector: { limit: 10, threshold: 0.6 },
        merge: { maxResults: 20 },
      });

      // Boost page-level concepts
      for (const match of retrievalResult.matches) {
        if (conceptKeys.includes(match.key)) {
          match.score = Math.min(1.0, match.score * 1.3);
        }
      }

      retrievalResult.matches.sort((a, b) => b.score - a.score);

      return {
        concepts: retrievalResult.matches,
        blocks,
      };
    } finally {
      await session.close();
    }
  }
}
