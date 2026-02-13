/**
 * Schema Mode Components Index
 *
 * Custom node types for Schema Mode visualization.
 *
 * v9.5 Hierarchical Layout (pure graph):
 * - SchemaBadgeNode: Compact badge for Realm & Layer (like relation badges)
 * - SchemaNode: Individual node type card (Class)
 *
 * @see docs/plans/2026-01-30-schema-mode-v2.md Task 2.2
 */

export { SchemaNode, type SchemaNodeData, type SchemaNodeType } from './SchemaNode';
export { SchemaBadgeNode, type SchemaBadgeNodeData, type SchemaBadgeNodeType } from './SchemaBadgeNode';
