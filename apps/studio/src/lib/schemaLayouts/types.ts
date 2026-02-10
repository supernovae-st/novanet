/**
 * Schema Layouts - Unified Spacing System
 *
 * Single source of truth for ALL layout algorithms.
 * Every layout file MUST import spacing from here - no local overrides.
 *
 * SPACING SYSTEM: Golden Ratio (φ = 1.618)
 * 3-level hierarchy: Node → Layer → Realm
 * Each level scales by φ² (≈2.618) for dramatic visual separation.
 *
 * ┌─────────────────────────────────────────────────────────┐
 * │ Level           │ Gap    │ Padding │ Header │ Derived  │
 * ├─────────────────┼────────┼─────────┼────────┼──────────┤
 * │ Node (leaf)     │  80px  │   -     │   -    │ BASE     │
 * │ Layer           │ 210px  │  80px   │  50px  │ ×φ²      │
 * │ Realm           │ 550px  │ 160px   │  70px  │ ×φ⁴      │
 * │ Canvas margin   │   -    │ 100px   │   -    │          │
 * └─────────────────┴────────┴─────────┴────────┴──────────┘
 */

import type { Node, Edge } from '@xyflow/react';
import type { Realm } from '@novanet/core/types';

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

/** Schema layout type */
export type SchemaLayoutType =
  | 'swimlanes'
  | 'stacked'
  | 'elkLayered'
  | 'target'
  | 'forceClusters';

/** Realm visual config */
export interface RealmConfig {
  realm: Realm;
  color: string;
  order: number;
}

export const REALM_CONFIGS: RealmConfig[] = [
  { realm: 'org', color: '#6c71c4', order: 0 },  // Solarized violet (from taxonomy.yaml)
  { realm: 'shared', color: '#2aa198', order: 1 },  // Solarized cyan (from taxonomy.yaml)
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

/** Gap between nodes in a grid (inside layer containers) */
export const NODE_GAP = BASE_UNIT;                           // 80px

/** Edge spacing for ELK edge routing */
export const EDGE_NODE_GAP = Math.round(BASE_UNIT * PHI);   // 129px
export const EDGE_EDGE_GAP = BASE_UNIT;                      // 80px

// =============================================================================
// LEVEL 2: Layer spacing (φ² scale up from node level)
// =============================================================================

/** Gap between layer containers */
export const LAYER_GAP = Math.round(BASE_UNIT * PHI_SQ);   // 209px → 210px

/** Padding inside layer containers */
export const LAYER_PADDING = BASE_UNIT;                     // 80px

/** Height reserved for layer label */
export const LAYER_HEADER = Math.round(BASE_UNIT * 0.625);  // 50px

// =============================================================================
// LEVEL 3: Realm spacing (φ⁴ scale up from node level)
// =============================================================================

/** Gap between realm containers */
export const REALM_GAP = Math.round(BASE_UNIT * PHI_QUAD);  // 548px → 550px

/** Padding inside realm containers */
export const REALM_PADDING = Math.round(BASE_UNIT * PHI_SQ); // 209px → 210px

/** Height reserved for realm label */
export const REALM_HEADER = Math.round(BASE_UNIT * 0.875);   // 70px

// =============================================================================
// Canvas dimensions
// =============================================================================

export const CANVAS_WIDTH = 20000;
export const CANVAS_HEIGHT = 16000;
export const CANVAS_MARGIN = Math.round(BASE_UNIT * 1.25);   // 100px

// =============================================================================
// Layout grid config
// =============================================================================

/** Max nodes per row in layer grids */
export const MAX_NODES_PER_ROW = 5;

/** Max layers per row in realm containers */
export const MAX_LAYERS_PER_ROW = 4;

/** Max canvas width before wrapping realms to next row */
export const MAX_ROW_WIDTH = 8000;
