/**
 * Schema Mode Components Index
 *
 * Custom node types for Schema Mode visualization.
 *
 * v9.5 Hierarchical Layout (pure graph):
 * - MetaBadgeNode: Compact badge for Realm & Layer (like relation badges)
 * - SchemaNode: Individual node type card (Kind)
 *
 * Legacy (container-based):
 * - RealmGroupNode: Top-level container (Project/Global/Shared)
 * - LayerGroupNode: Nested container (foundation/structure/etc.)
 *
 * @see docs/plans/2026-01-30-schema-mode-v2.md Task 2.2
 */

export { RealmGroupNode, type RealmGroupData, type RealmGroupNodeType } from './RealmGroupNode';
export { LayerGroupNode, type LayerGroupData, type LayerGroupNodeType } from './LayerGroupNode';
export { SchemaNode, type SchemaNodeData, type SchemaNodeType } from './SchemaNode';
export { MetaBadgeNode, type MetaBadgeNodeData, type MetaBadgeNodeType } from './MetaBadgeNode';
