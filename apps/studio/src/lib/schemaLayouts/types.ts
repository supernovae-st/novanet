/**
 * Schema Layouts - Type Definitions
 *
 * Shared types for all schema layout algorithms.
 *
 * SPACING SYSTEM: Golden Ratio (φ = 1.618)
 * Each hierarchy level has φ× more spacing than the level below.
 * This creates visual harmony with mathematically pleasing proportions.
 */

import type { Node, Edge } from '@xyflow/react';
import type { Scope } from '@novanet/core/types';

/** Layout result with React Flow nodes and edges */
export interface SchemaLayoutResult {
  nodes: Node[];
  edges: Edge[];
}

/** Layout direction from UI store */
export type LayoutDirection = 'LR' | 'TB' | 'dagre' | 'radial' | 'force';

/** Scope visual config */
export interface ScopeConfig {
  scope: Scope;
  color: string;
  order: number;
}

export const SCOPE_CONFIGS: ScopeConfig[] = [
  { scope: 'Project', color: '#8b5cf6', order: 0 },
  { scope: 'Global', color: '#10b981', order: 1 },
  { scope: 'Shared', color: '#f59e0b', order: 2 },
];

// =============================================================================
// GOLDEN RATIO SPACING SYSTEM
// =============================================================================

/** Golden Ratio - foundation for visual harmony */
export const PHI = 1.618;

/** Node dimensions */
export const NODE_WIDTH = 160;      // Slightly larger nodes for better visibility
export const NODE_HEIGHT = 60;

/** Base unit for spacing calculations */
export const BASE_UNIT = NODE_WIDTH;

/**
 * AGGRESSIVE SPACING - Nodes need LOTS of room for edges to be visible
 * Each level multiplied by φ² (≈2.618) for dramatic separation
 */
export const NODE_GAP = 280;        // HUGE gap between nodes (was 112) - edges need space!
export const SUBCAT_GAP = 500;      // Major gap between subcategories (was 181)
export const SCOPE_GAP = 800;       // Massive gap between scopes (was 293)

/**
 * Container padding (inside groups) - VERY generous
 * Nodes need breathing room from container edges
 */
export const INNER_PADDING = 180;   // Big padding inside subcategories (was 70)
export const GROUP_PADDING = 250;   // Big padding inside scope groups (was 87)

/**
 * Canvas dimensions - MUCH larger to accommodate spacing
 */
export const CANVAS_WIDTH = 10000;  // 10k wide (was 4000)
export const CANVAS_HEIGHT = 8000;  // 8k tall (was 3200)
export const CANVAS_MARGIN = 100;   // Generous outer margin
