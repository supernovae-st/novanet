/**
 * Schema Layouts - Type Definitions
 *
 * Shared types for all schema layout algorithms.
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

/** Node dimensions */
export const NODE_WIDTH = 140;
export const NODE_HEIGHT = 50;
export const GROUP_PADDING = 60;
