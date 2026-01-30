// src/filters/index.ts
/**
 * NovaNet Filter System v7.2.1
 *
 * Provides composable filter/view system for generating Cypher queries.
 *
 * Components:
 * - NovaNetFilter: Fluent API for building filter criteria
 * - CypherGenerator: Converts filters to executable Cypher queries
 * - ViewLoader: Loads YAML view definitions
 *
 * @module filters
 */

// Export all types
export * from './types.js';

// Export main classes
export { NovaNetFilter, type FilterState } from './NovaNetFilter.js';
export { CypherGenerator } from './CypherGenerator.js';
export { ViewLoader } from './ViewLoader.js';

// Re-export commonly used types for convenient access
export type {
  FilterCriteria,
  ViewDefinition,
  IncludeRule,
  CypherQuery,
  NodeCategory,
  NodeType,
  RelationDirection,
  ViewRegistry,
} from './types.js';
