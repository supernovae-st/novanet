/**
 * Schema Layout Dispatcher
 *
 * Routes layout requests to specialized layout algorithms based on direction.
 * Each layout is optimized for different visualization needs:
 * - Treemap (dagre): Default, proportional rectangles
 * - Swimlanes (LR): Horizontal bands per scope
 * - Stacked (TB): Vertical stacked scopes
 * - Target (radial): Concentric rings
 * - Force (force): Physics-based clustering
 */

import type { HierarchicalSchemaData } from '@novanet/core/graph';
import { applySchemaLayoutByDirection } from './schemaLayouts';
import type { SchemaLayoutResult, LayoutDirection } from './schemaLayouts/types';

/**
 * Apply schema layout based on direction.
 *
 * @param hierarchy - Hierarchical schema data from @novanet/core
 * @param layoutDirection - UI layout direction
 * @returns React Flow nodes and edges
 */
export async function applySchemaLayout(
  hierarchy: HierarchicalSchemaData,
  layoutDirection: LayoutDirection = 'dagre'
): Promise<SchemaLayoutResult> {
  // Delegate to the new layout system
  return applySchemaLayoutByDirection(hierarchy, layoutDirection);
}

// Re-export types for backward compatibility
export type { SchemaLayoutResult };
