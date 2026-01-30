// NovaNet Core - Main Entry Point
// Graph-native content generation system v8.0.0

// =============================================================================
// TYPES
// =============================================================================

// Export all types from index (which re-exports locale-knowledge types)
export * from './types/index.js';
// Note: locale-knowledge types are re-exported via types/index.ts to avoid conflicts

// =============================================================================
// DATABASE
// =============================================================================

export * from './db/index.js';

// =============================================================================
// PARSERS
// =============================================================================

export * from './parsers/index.js';

// =============================================================================
// FILTERS
// =============================================================================

/**
 * Filter system for composable Cypher query generation.
 *
 * @example
 * ```typescript
 * import { NovaNetFilter, CypherGenerator, ViewLoader } from 'novanet-core';
 *
 * // Fluent API
 * const filter = NovaNetFilter.create()
 *   .fromPage('page-pricing')
 *   .includeBlocks()
 *   .includeConcepts({ spreading: true })
 *   .forLocale('fr-FR');
 *
 * const { query, params } = CypherGenerator.generate(filter);
 *
 * // YAML-based views
 * const view = await ViewLoader.loadView('page-generation-context', viewsDir);
 * const filterFromView = ViewLoader.toFilter(view, { key: 'page-pricing', locale: 'fr-FR' });
 * ```
 */
export * from './filters/index.js';

// =============================================================================
// CONFIG (v8.0.0)
// =============================================================================

/**
 * Configuration for spreading activation and task-specific modifiers.
 * Use: `import { config } from 'novanet-core'`
 */
export * as config from './config/index.js';

// =============================================================================
// SCHEMAS (v8.0.0)
// =============================================================================

/**
 * Zod validation schemas for Neo4j relations and locale knowledge.
 * Use: `import { schemas } from 'novanet-core'`
 */
export * as schemas from './schemas/index.js';

// =============================================================================
// SERVICES (v8.0.0)
// =============================================================================

/**
 * Core services for Hybrid OntologyRAG:
 * - VectorSearchService: Semantic search using embeddings
 * - GraphTraversalService: Spreading activation over concept graph
 * - HybridRetriever: Combined vector + graph retrieval
 * Use: `import { services } from 'novanet-core'`
 */
export * as services from './services/index.js';

// =============================================================================
// GENERATORS (v8.0.0)
// =============================================================================

/**
 * Documentation generators for Unified View System:
 * - ViewParser: Parse and validate view YAML with docs section
 * - MarkdownGenerator: Generate MD documentation from views
 * - CypherExporter: Extract Cypher queries from views
 * Use: `import { generators } from 'novanet-core'`
 */
export * as generators from './generators/index.js';

// =============================================================================
// GRAPH (v8.2.0)
// =============================================================================

/**
 * Graph module for schema visualization:
 * - generateSchemaGraph(): Flat 35 nodes + ~89 edges
 * - getSchemaHierarchy(): Grouped by scope/subcategory
 * - SCOPE_HIERARCHY: Scope definitions with subcategories
 * - NODE_SUBCATEGORIES: NodeType to subcategory mapping
 * Use: `import { graph } from 'novanet-core'` or `import { generateSchemaGraph } from 'novanet-core'`
 */
export * as graph from './graph/index.js';
export { generateSchemaGraph, getSchemaHierarchy } from './graph/index.js';
