/**
 * Vector Search Service (v7.8.0)
 *
 * Provides semantic search over Concept, ConceptL10n, and Page embeddings
 * using Neo4j's HNSW vector indexes.
 */

import type { Driver } from 'neo4j-driver';
import OpenAI from 'openai';

// =============================================================================
// Types
// =============================================================================

export interface VectorMatch {
  key: string;
  score: number;
  nodeType: string;
  properties: Record<string, unknown>;
}

export interface VectorSearchOptions {
  /** Maximum number of results (default: 10) */
  limit?: number;
  /** Minimum similarity threshold 0-1 (default: 0.7) */
  threshold?: number;
  /** Filter by locale (for ConceptL10n) */
  locale?: string;
}

// =============================================================================
// Service
// =============================================================================

export class VectorSearchService {
  private openai: OpenAI;

  constructor(private driver: Driver) {
    this.openai = new OpenAI();
  }

  /**
   * Generate an embedding for a query string.
   */
  async embedQuery(text: string): Promise<number[]> {
    const response = await this.openai.embeddings.create({
      model: 'text-embedding-3-small',
      input: text,
      dimensions: 1536,
    });
    return response.data[0].embedding;
  }

  /**
   * Search Concept nodes by semantic similarity.
   *
   * @param queryOrEmbedding - Search query string or pre-computed embedding
   * @param options - Search options
   * @returns Ranked list of matching Concepts
   */
  async searchConcepts(
    queryOrEmbedding: string | number[],
    options: VectorSearchOptions = {}
  ): Promise<VectorMatch[]> {
    const { limit = 10, threshold = 0.7 } = options;
    const embedding =
      typeof queryOrEmbedding === 'string'
        ? await this.embedQuery(queryOrEmbedding)
        : queryOrEmbedding;

    const session = this.driver.session();
    try {
      const result = await session.run(
        `
        CALL db.index.vector.queryNodes('concept_embedding', $limit, $embedding)
        YIELD node AS c, score
        WHERE score >= $threshold
        RETURN c.key AS key,
               score,
               'Concept' AS nodeType,
               c { .key, .display_name, .description, .llm_context, .priority, .freshness } AS properties
        ORDER BY score DESC
      `,
        { limit, threshold, embedding }
      );

      return result.records.map((r) => ({
        key: r.get('key'),
        score: r.get('score'),
        nodeType: r.get('nodeType'),
        properties: r.get('properties'),
      }));
    } finally {
      await session.close();
    }
  }

  /**
   * Search ConceptL10n nodes by semantic similarity.
   * Filters by locale to ensure results match the target language.
   *
   * @param queryOrEmbedding - Search query string or pre-computed embedding
   * @param locale - Target locale (e.g., "fr-FR")
   * @param options - Search options
   * @returns Ranked list of matching ConceptL10n nodes
   */
  async searchConceptL10n(
    queryOrEmbedding: string | number[],
    locale: string,
    options: VectorSearchOptions = {}
  ): Promise<VectorMatch[]> {
    const { limit = 10, threshold = 0.7 } = options;
    const embedding =
      typeof queryOrEmbedding === 'string'
        ? await this.embedQuery(queryOrEmbedding)
        : queryOrEmbedding;

    const session = this.driver.session();
    try {
      // Query more than needed since we filter by locale after
      const result = await session.run(
        `
        CALL db.index.vector.queryNodes('concept_l10n_embedding', $queryLimit, $embedding)
        YIELD node AS cl, score
        MATCH (cl)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        MATCH (cl)-[:L10N_OF]->(c:Concept)
        WHERE score >= $threshold
        RETURN c.key + '-' + l.key AS key,
               score,
               'ConceptL10n' AS nodeType,
               cl { .title, .definition, .summary, .purpose, .priority, concept_key: c.key, locale_key: l.key } AS properties
        ORDER BY score DESC
        LIMIT $limit
      `,
        { queryLimit: limit * 2, limit, threshold, embedding, locale }
      );

      return result.records.map((r) => ({
        key: r.get('key'),
        score: r.get('score'),
        nodeType: r.get('nodeType'),
        properties: r.get('properties'),
      }));
    } finally {
      await session.close();
    }
  }

  /**
   * Search Page nodes by semantic similarity.
   *
   * @param queryOrEmbedding - Search query string or pre-computed embedding
   * @param options - Search options
   * @returns Ranked list of matching Pages
   */
  async searchPages(
    queryOrEmbedding: string | number[],
    options: VectorSearchOptions = {}
  ): Promise<VectorMatch[]> {
    const { limit = 10, threshold = 0.7 } = options;
    const embedding =
      typeof queryOrEmbedding === 'string'
        ? await this.embedQuery(queryOrEmbedding)
        : queryOrEmbedding;

    const session = this.driver.session();
    try {
      const result = await session.run(
        `
        CALL db.index.vector.queryNodes('page_embedding', $limit, $embedding)
        YIELD node AS p, score
        WHERE score >= $threshold
        RETURN p.key AS key,
               score,
               'Page' AS nodeType,
               p { .key, .display_name, .description, .llm_context, .priority } AS properties
        ORDER BY score DESC
      `,
        { limit, threshold, embedding }
      );

      return result.records.map((r) => ({
        key: r.get('key'),
        score: r.get('score'),
        nodeType: r.get('nodeType'),
        properties: r.get('properties'),
      }));
    } finally {
      await session.close();
    }
  }

  /**
   * Combined search across Concepts, ConceptL10n (if locale provided), and Pages.
   * When locale is provided, includes localized concept content in results.
   *
   * @param query - Search query string
   * @param options - Search options (include locale for ConceptL10n)
   * @returns Ranked list of matching nodes from all applicable types
   */
  async searchAll(
    query: string,
    options: VectorSearchOptions = {}
  ): Promise<VectorMatch[]> {
    const embedding = await this.embedQuery(query);
    const { locale } = options;

    // Build parallel search promises
    const searches: Promise<VectorMatch[]>[] = [
      this.searchConcepts(embedding, options),
      this.searchPages(embedding, options),
    ];

    // Include ConceptL10n if locale is specified
    if (locale) {
      searches.push(this.searchConceptL10n(embedding, locale, options));
    }

    const results = await Promise.all(searches);

    // Flatten, merge and sort by score
    const all = results.flat().sort((a, b) => b.score - a.score);

    // Apply limit to combined results
    const limit = options.limit ?? 10;
    return all.slice(0, limit);
  }
}
