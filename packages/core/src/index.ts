// NovaNet Core - Main Entry Point
// Graph-native content generation system v10.4.0

// =============================================================================
// TYPES
// =============================================================================

export * from './types/index.js';

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
 *   .includeEntities({ spreading: true })
 *   .forLocale('fr-FR');
 *
 * const { query, params } = CypherGenerator.generate(filter);
 *
 * // View-based filters (generated constants)
 * const view = await ViewLoader.loadView('page-generation-context');
 * const filterFromView = ViewLoader.toFilter(view, { key: 'page-pricing', locale: 'fr-FR' });
 * ```
 */
export * from './filters/index.js';

// =============================================================================
// SCHEMAS
// =============================================================================

/**
 * Zod validation schemas for Neo4j relations and locale knowledge.
 * Use: `import { schemas } from 'novanet-core'`
 */
export * as schemas from './schemas/index.js';

// =============================================================================
// GRAPH
// =============================================================================

/**
 * Graph module for schema visualization:
 * - generateSchemaGraph(): Flat 43 nodes + ~77 edges
 * - getSchemaHierarchy(): Grouped by realm/layer
 * - REALM_HIERARCHY: Realm definitions with layers
 * - NODE_LAYERS: NodeType to layer mapping
 * Use: `import { graph } from 'novanet-core'` or `import { generateSchemaGraph } from 'novanet-core'`
 */
export * as graph from './graph/index.js';
export { generateSchemaGraph, getSchemaHierarchy } from './graph/index.js';
