/**
 * Schema Mode Components Index
 *
 * Custom node types for Schema Mode visualization.
 * These components render the hierarchical grouped layout:
 * - ScopeGroupNode: Top-level container (Project/Global/Shared)
 * - SubcategoryGroupNode: Nested container (foundation/structure/etc.)
 * - SchemaNode: Individual node type card
 *
 * @see docs/plans/2026-01-30-schema-mode-v2.md Task 2.2
 */

export { ScopeGroupNode, type ScopeGroupData, type ScopeGroupNodeType } from './ScopeGroupNode';
export { SubcategoryGroupNode, type SubcategoryGroupData, type SubcategoryGroupNodeType } from './SubcategoryGroupNode';
export { SchemaNode, type SchemaNodeData, type SchemaNodeType } from './SchemaNode';
