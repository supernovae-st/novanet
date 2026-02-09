/**
 * Graph Layout Utilities (v7.2.7)
 *
 * Uses dagre for automatic graph layout with intelligent positioning.
 * Supports hierarchical layouts for knowledge graphs.
 *
 * Features:
 * - Variable node sizes based on type for accurate spacing
 * - Increased margins for better visual hierarchy
 * - Type-aware collision detection
 */

import dagre from '@dagrejs/dagre';
import type { Node, Edge } from '@xyflow/react';

export interface LayoutOptions {
  /** Layout direction: TB (top-bottom), BT, LR (left-right), RL */
  direction?: 'TB' | 'BT' | 'LR' | 'RL';
  /** Default node width for layout calculation */
  nodeWidth?: number;
  /** Default node height for layout calculation */
  nodeHeight?: number;
  /** Horizontal spacing between ranks (levels) */
  ranksep?: number;
  /** Vertical spacing between nodes in same rank */
  nodesep?: number;
  /** Layout algorithm */
  ranker?: 'network-simplex' | 'tight-tree' | 'longest-path';
  /** Use variable node sizes based on type */
  useVariableSizes?: boolean;
}

/**
 * AGGRESSIVE SPACING for clear edge visibility
 */
const DEFAULT_OPTIONS: Required<LayoutOptions> = {
  direction: 'TB',
  nodeWidth: 200,
  nodeHeight: 100,
  ranksep: 500,  // HUGE vertical spacing between ranks (was ~291)
  nodesep: 350,  // HUGE horizontal spacing between nodes (was ~194)
  ranker: 'network-simplex',
  useVariableSizes: true,
};

/**
 * Node dimensions based on type (matches actual component sizes)
 * Includes padding for visual breathing room
 */
const NODE_DIMENSIONS: Record<string, { width: number; height: number }> = {
  // Project node - largest, most prominent
  Project: { width: 320, height: 180 },

  // Structural nodes - medium sized cards
  Page: { width: 240, height: 140 },
  Locale: { width: 230, height: 130 },
  Entity: { width: 225, height: 130 }, // v10.3: was Concept
  Block: { width: 205, height: 120 },
  BlockType: { width: 195, height: 115 },
  BrandIdentity: { width: 210, height: 120 },
  ProjectContent: { width: 210, height: 120 },

  // v10 knowledge nodes - tiered model
  // Technical tier
  Formatting: { width: 100, height: 100 },
  Slugification: { width: 105, height: 105 },
  Adaptation: { width: 100, height: 100 },
  // Style tier
  Style: { width: 95, height: 95 },
  // Semantic tier
  TermSet: { width: 90, height: 90 },
  ExpressionSet: { width: 110, height: 110 },
  PatternSet: { width: 100, height: 100 },
  CultureSet: { width: 100, height: 100 },
  TabooSet: { width: 90, height: 90 },
  AudienceSet: { width: 105, height: 105 },

  // Generation nodes
  PagePrompt: { width: 200, height: 110 },
  BlockPrompt: { width: 190, height: 105 },
  BlockRules: { width: 190, height: 105 },
  PageOutput: { width: 200, height: 110 },
  BlockOutput: { width: 190, height: 105 },

  // SEO/GEO nodes
  SEOKeyword: { width: 180, height: 100 },
  SEOVariation: { width: 170, height: 95 },
  GEOSeed: { width: 180, height: 100 },
  GEOReformulation: { width: 180, height: 100 },

  // Default fallback
  default: { width: 200, height: 100 },
};

/**
 * Get dimensions for a specific node type
 */
export function getNodeDimensions(nodeType: string): { width: number; height: number } {
  return NODE_DIMENSIONS[nodeType] || NODE_DIMENSIONS.default;
}

/**
 * Apply dagre layout to nodes and edges
 * Returns new node array with updated positions
 *
 * Now supports variable node sizes for accurate layout
 */
export function applyDagreLayout<T extends Node>(
  nodes: T[],
  edges: Edge[],
  options: LayoutOptions = {}
): T[] {
  const opts = { ...DEFAULT_OPTIONS, ...options };

  // Create a new dagre graph
  const g = new dagre.graphlib.Graph();
  g.setDefaultEdgeLabel(() => ({}));

  // Set graph options with AGGRESSIVE margins for edge visibility
  g.setGraph({
    rankdir: opts.direction,
    ranksep: opts.ranksep,
    nodesep: opts.nodesep,
    ranker: opts.ranker,
    marginx: 200,  // Generous margins
    marginy: 200,  // Generous margins
  });

  // Build a map of node dimensions for position calculation
  const nodeSizes = new Map<string, { width: number; height: number }>();

  // Add nodes to the graph with variable sizes
  nodes.forEach((node) => {
    let width = opts.nodeWidth;
    let height = opts.nodeHeight;

    if (opts.useVariableSizes && node.data && typeof node.data === 'object' && 'type' in node.data) {
      const nodeType = node.data.type as string;
      const dims = getNodeDimensions(nodeType);
      width = dims.width;
      height = dims.height;
    }

    nodeSizes.set(node.id, { width, height });
    g.setNode(node.id, { width, height });
  });

  // Add edges to the graph
  edges.forEach((edge) => {
    g.setEdge(edge.source, edge.target);
  });

  // Run the layout algorithm
  dagre.layout(g);

  // Apply the calculated positions to nodes
  return nodes.map((node) => {
    const nodeWithPosition = g.node(node.id);
    const size = nodeSizes.get(node.id) || { width: opts.nodeWidth, height: opts.nodeHeight };

    // dagre returns center position, we need top-left
    return {
      ...node,
      position: {
        x: nodeWithPosition.x - size.width / 2,
        y: nodeWithPosition.y - size.height / 2,
      },
    };
  });
}
