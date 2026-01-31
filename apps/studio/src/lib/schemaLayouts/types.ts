/**
 * Schema Layouts - Unified Spacing System
 *
 * Single source of truth for ALL layout algorithms.
 * Every layout file MUST import spacing from here - no local overrides.
 *
 * SPACING SYSTEM: Golden Ratio (φ = 1.618)
 * 3-level hierarchy: Node → Subcategory → Scope
 * Each level scales by φ² (≈2.618) for dramatic visual separation.
 *
 * ┌─────────────────────────────────────────────────────────┐
 * │ Level           │ Gap    │ Padding │ Header │ Derived  │
 * ├─────────────────┼────────┼─────────┼────────┼──────────┤
 * │ Node (leaf)     │  80px  │   -     │   -    │ BASE     │
 * │ Subcategory     │ 210px  │  80px   │  50px  │ ×φ²      │
 * │ Scope           │ 550px  │ 160px   │  70px  │ ×φ⁴      │
 * │ Canvas margin   │   -    │ 100px   │   -    │          │
 * └─────────────────┴────────┴─────────┴────────┴──────────┘
 */

import type { Node, Edge } from '@xyflow/react';
import type { Scope } from '@novanet/core/types';

// =============================================================================
// Types
// =============================================================================

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
// GOLDEN RATIO UNIFIED SPACING SYSTEM
// =============================================================================

/** Golden Ratio - foundation for visual harmony */
export const PHI = 1.618;
export const PHI_SQ = PHI * PHI;      // ≈2.618 - scale factor between levels
export const PHI_QUAD = PHI_SQ * PHI_SQ; // ≈6.854 - 2 levels up

// =============================================================================
// Node Dimensions (leaf level)
// =============================================================================

export const NODE_WIDTH = 160;
export const NODE_HEIGHT = 60;

// =============================================================================
// BASE_UNIT: Everything derives from this
// =============================================================================

/**
 * Base spacing unit = node width / 2 = 80px
 * All spacing is a clean multiple or φ-scale of this.
 */
export const BASE_UNIT = Math.round(NODE_WIDTH / 2); // 80px

// =============================================================================
// LEVEL 1: Node spacing (gaps between leaf nodes)
// =============================================================================

/** Gap between nodes in a grid (inside subcategory containers) */
export const NODE_GAP = BASE_UNIT;                           // 80px

/** Edge spacing for ELK edge routing */
export const EDGE_NODE_GAP = Math.round(BASE_UNIT * PHI);   // 129px
export const EDGE_EDGE_GAP = BASE_UNIT;                      // 80px

// =============================================================================
// LEVEL 2: Subcategory spacing (φ² scale up from node level)
// =============================================================================

/** Gap between subcategory containers */
export const SUBCAT_GAP = Math.round(BASE_UNIT * PHI_SQ);   // 209px → 210px

/** Padding inside subcategory containers */
export const SUBCAT_PADDING = BASE_UNIT;                     // 80px

/** Height reserved for subcategory label */
export const SUBCAT_HEADER = Math.round(BASE_UNIT * 0.625);  // 50px

// =============================================================================
// LEVEL 3: Scope spacing (φ⁴ scale up from node level)
// =============================================================================

/** Gap between scope containers */
export const SCOPE_GAP = Math.round(BASE_UNIT * PHI_QUAD);  // 548px → 550px

/** Padding inside scope containers */
export const SCOPE_PADDING = Math.round(BASE_UNIT * PHI_SQ); // 209px → 210px

/** Height reserved for scope label */
export const SCOPE_HEADER = Math.round(BASE_UNIT * 0.875);   // 70px

// =============================================================================
// LEGACY ALIASES (for backward compatibility during migration)
// =============================================================================

/** @deprecated Use SUBCAT_PADDING */
export const INNER_PADDING = SUBCAT_PADDING;

/** @deprecated Use SCOPE_PADDING */
export const GROUP_PADDING = SCOPE_PADDING;

// =============================================================================
// Canvas dimensions
// =============================================================================

export const CANVAS_WIDTH = 20000;
export const CANVAS_HEIGHT = 16000;
export const CANVAS_MARGIN = Math.round(BASE_UNIT * 1.25);   // 100px

// =============================================================================
// Layout grid config
// =============================================================================

/** Max nodes per row in subcategory grids */
export const MAX_NODES_PER_ROW = 5;

/** Max subcategories per row in scope containers */
export const MAX_SUBCATS_PER_ROW = 4;

/** Max canvas width before wrapping scopes to next row */
export const MAX_ROW_WIDTH = 8000;
