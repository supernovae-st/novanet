/**
 * NovaNet Services (v7.8.0)
 *
 * Core services for Hybrid OntologyRAG:
 * - VectorSearchService: Semantic search using embeddings
 * - GraphTraversalService: Spreading activation over concept graph
 * - HybridRetriever: Combined vector + graph retrieval
 */

export {
  VectorSearchService,
  type VectorMatch,
  type VectorSearchOptions,
} from './vector-search.js';

export {
  GraphTraversalService,
  type SpreadingActivationOptions,
  type SpreadingActivationResult,
} from './graph-traversal.js';

export {
  HybridRetriever,
  type HybridRetrievalOptions,
  type HybridRetrievalResult,
  type HybridMatch,
} from './hybrid-retriever.js';
