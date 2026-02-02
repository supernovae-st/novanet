/**
 * Schema Layouts - Index & Dispatcher
 *
 * Routes layout requests to specialized layout algorithms based on direction.
 * Each layout is optimized for different visualization needs.
 */

export * from './types';
export { applySwimlaneLayout } from './swimlanes';
export { applyStackedLayout } from './stacked';
export { applyTargetLayout } from './target';
export { applyForceClusterLayout } from './forceClusters';
export { applyElkLayeredLayout, applyElkLayeredLayoutSync } from './elkLayered';


import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { LayoutDirection, SchemaLayoutResult } from './types';
import { applySwimlaneLayout } from './swimlanes';
import { applyStackedLayout } from './stacked';
import { applyTargetLayout } from './target';
import { applyForceClusterLayout } from './forceClusters';
import { applyElkLayeredLayoutSync } from './elkLayered';

/**
 * Apply schema layout based on direction
 *
 * Layout Mapping:
 * - dagre → ELK Layered (DEFAULT) - Sugiyama algorithm with edge crossing minimization
 * - LR    → Swimlanes - Horizontal bands per scope
 * - TB    → Stacked - Vertical stacked scopes
 * - radial → Target - Concentric rings by scope
 * - force → Force Clusters - Physics-based clustering
 *
 * Note: ELK Layered analyzes graph structure to minimize edge crossings,
 * producing much cleaner layouts than simple grid/treemap approaches.
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
      // Use ELK-based edge-aware layout for crossing minimization
      return applyElkLayeredLayoutSync(hierarchy);
    case 'LR':
      return applySwimlaneLayout(hierarchy);
    case 'TB':
      return applyStackedLayout(hierarchy);
    case 'radial':
      return applyTargetLayout(hierarchy);
    case 'force':
      return applyForceClusterLayout(hierarchy);
    default:
      return applyElkLayeredLayoutSync(hierarchy);
  }
}
