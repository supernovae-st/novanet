/**
 * Schema Layouts - Index & Dispatcher
 *
 * Routes layout requests to specialized layout algorithms based on direction.
 * Each layout is optimized for different visualization needs.
 *
 * v9.5: Default layout changed to Hierarchical (pure graph nodes, no containers)
 */

export * from './types';
export { applyHierarchicalLayout } from './hierarchical';


import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { LayoutDirection, SchemaLayoutResult } from './types';
import { applyHierarchicalLayout } from './hierarchical';

/**
 * Apply schema layout based on direction
 *
 * Layout Mapping (v9.5):
 * - dagre → Hierarchical (DEFAULT) - Pure graph nodes with Dagre LR layout
 * - LR    → Hierarchical - Same as dagre (Realm → Layer → Class)
 * - TB    → Hierarchical - Same layout (may add TB variant later)
 * - radial → Hierarchical - Same layout (may add radial variant later)
 * - force → Hierarchical - Same layout (may add force variant later)
 *
 * Note: v9.5 replaces container-based layouts with pure graph visualization.
 * All nodes (Realm, Layer, Class) are regular nodes connected by edges.
 *
 * @param hierarchy - Schema hierarchy data
 * @param direction - Layout direction from toolbar
 */
export function applySchemaLayoutByDirection(
  hierarchy: HierarchicalSchemaData,
  _direction: LayoutDirection
): SchemaLayoutResult {
  // v9.5: Use hierarchical layout for all directions
  // This removes container nodes and shows pure graph with edges
  return applyHierarchicalLayout(hierarchy);
}
