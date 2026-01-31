/**
 * Schema Layouts - Index & Dispatcher
 *
 * Routes layout requests to specialized layout algorithms based on direction.
 * Each layout is optimized for different visualization needs.
 */

export * from './types';
export { applyTreemapLayout } from './treemap';
export { applySwimlaneLayout } from './swimlanes';
export { applyStackedLayout } from './stacked';
export { applyTargetLayout } from './target';
export { applyForceClusterLayout } from './forceClusters';

import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { LayoutDirection, SchemaLayoutResult } from './types';
import { applyTreemapLayout } from './treemap';
import { applySwimlaneLayout } from './swimlanes';
import { applyStackedLayout } from './stacked';
import { applyTargetLayout } from './target';
import { applyForceClusterLayout } from './forceClusters';

/**
 * Apply schema layout based on direction
 *
 * Layout Mapping:
 * - dagre → Treemap (DEFAULT) - Rectangles proportional to node count
 * - LR    → Swimlanes - Horizontal bands per scope
 * - TB    → Stacked - Vertical stacked scopes
 * - radial → Target - Concentric rings by scope
 * - force → Force Clusters - Physics-based clustering
 *
 * @param hierarchy - Schema hierarchy data
 * @param direction - Layout direction from toolbar
 */
export function applySchemaLayoutByDirection(
  hierarchy: HierarchicalSchemaData,
  direction: LayoutDirection
): SchemaLayoutResult {
  switch (direction) {
    case 'dagre':
      return applyTreemapLayout(hierarchy);
    case 'LR':
      return applySwimlaneLayout(hierarchy);
    case 'TB':
      return applyStackedLayout(hierarchy);
    case 'radial':
      return applyTargetLayout(hierarchy);
    case 'force':
      return applyForceClusterLayout(hierarchy);
    default:
      return applyTreemapLayout(hierarchy);
  }
}
