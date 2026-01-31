/**
 * Force Simulation Utilities (v8.0.0)
 *
 * D3-force physics simulation for dynamic graph layout.
 * Uses velocity Verlet numerical integration for smooth animations.
 *
 * Optimized for knowledge graph visualization with:
 * - Strong node repulsion for clear separation
 * - Category-based clustering
 * - Viewport-aware scaling
 * - Type-aware collision detection for accurate spacing
 * - Spacing presets (compact, normal, spacious)
 * - Default: SPACIOUS (+100% spacing)
 */

import {
  forceSimulation,
  forceLink,
  forceManyBody,
  forceCenter,
  forceCollide,
  forceX,
  forceY,
  type Simulation,
  type SimulationNodeDatum,
  type SimulationLinkDatum,
} from 'd3-force';
import type { Node, Edge } from '@xyflow/react';
import { getNodeDimensions } from './layout';

// ============================================================================
// SPACING PRESETS - TDD-defined values for consistent layout
// ============================================================================

export type SpacingPreset = 'compact' | 'normal' | 'spacious';

export interface SpacingPresetValues {
  chargeStrength: number;
  linkDistance: number;
  collisionRadius: number;
}

/**
 * Spacing presets for graph layout
 * - compact: Dense layout for overview mode
 * - normal: Balanced layout (previous default)
 * - spacious: Generous spacing for detailed exploration (+100%)
 */
export const SPACING_PRESETS: Record<SpacingPreset, SpacingPresetValues> = {
  compact: {
    chargeStrength: -800,
    linkDistance: 200,
    collisionRadius: 2.0,
  },
  normal: {
    chargeStrength: -1200,
    linkDistance: 320,
    collisionRadius: 2.5,
  },
  spacious: {
    chargeStrength: -2400,  // 2x normal
    linkDistance: 640,      // 2x normal
    collisionRadius: 4.0,   // ~1.6x normal
  },
};

/** Default spacing preset - SPACIOUS for better readability */
export const DEFAULT_SPACING_PRESET: SpacingPreset = 'spacious';

export interface ForceNode extends SimulationNodeDatum {
  id: string;
  x: number;
  y: number;
  fx?: number | null;
  fy?: number | null;
  category?: string;
  nodeType?: string;
}

export interface ForceLink extends SimulationLinkDatum<ForceNode> {
  source: string | ForceNode;
  target: string | ForceNode;
}

export interface ForceOptions {
  /** Spacing preset - overrides chargeStrength, linkDistance, collisionRadius */
  preset?: SpacingPreset;
  /** Strength of node repulsion (-100 to -2400, more negative = stronger) */
  chargeStrength?: number;
  /** Link distance (100-640) */
  linkDistance?: number;
  /** Collision radius multiplier (1.5-4.0) */
  collisionRadius?: number;
  /** Center force strength (0-0.1) */
  centerStrength?: number;
  /** Enable category clustering */
  clusterByCategory?: boolean;
  /** Alpha decay rate (0.01-0.05) - lower = longer simulation */
  alphaDecay?: number;
  /** Velocity decay (0.1-0.6) */
  velocityDecay?: number;
  /** Viewport width for scaling */
  viewportWidth?: number;
  /** Viewport height for scaling */
  viewportHeight?: number;
  /** Node count for adaptive scaling */
  nodeCount?: number;
}

// Use spacious preset values as defaults for better readability
const spaciousPreset = SPACING_PRESETS[DEFAULT_SPACING_PRESET];

const DEFAULT_OPTIONS: Required<ForceOptions> = {
  preset: DEFAULT_SPACING_PRESET,
  chargeStrength: spaciousPreset.chargeStrength, // -2400 (2x stronger repulsion)
  linkDistance: spaciousPreset.linkDistance,     // 640 (2x link distance)
  collisionRadius: spaciousPreset.collisionRadius, // 4.0 (~1.6x collision radius)
  centerStrength: 0.015,  // Slightly reduced center pull for spacious layout
  clusterByCategory: true,
  alphaDecay: 0.012,     // Slower decay for better convergence
  velocityDecay: 0.25,   // Lower velocity decay for more movement
  viewportWidth: 1920,
  viewportHeight: 1080,
  nodeCount: 50,
};

// Category cluster positions - spread across a larger area (v7.2.3)
// Increased spacing for better visual separation
const getCategoryCenter = (category: string, scale: number = 1): { x: number; y: number } => {
  const positions: Record<string, { x: number; y: number }> = {
    project: { x: -800 * scale, y: 0 },         // Left center
    content: { x: -400 * scale, y: -500 * scale }, // Upper left
    locale: { x: 0, y: 600 * scale },           // Bottom center
    generation: { x: 400 * scale, y: -500 * scale }, // Upper right
    seo: { x: 700 * scale, y: 300 * scale },    // Right lower
    geo: { x: -700 * scale, y: 300 * scale },   // Left lower
    analytics: { x: 800 * scale, y: 0 },        // Right center
  };
  return positions[category] || { x: 0, y: 0 };
};

/**
 * Calculate adaptive parameters based on node count
 * Optimized for better spacing and visual clarity
 */
function getAdaptiveParams(nodeCount: number, viewportWidth: number, viewportHeight: number) {
  // Scale factor based on node count - more nodes = more space needed
  const countFactor = Math.sqrt(nodeCount / 15); // Lower divisor = more spread

  // Scale factor based on viewport
  const viewportArea = viewportWidth * viewportHeight;
  const baseArea = 1920 * 1080;
  const viewportFactor = Math.sqrt(viewportArea / baseArea);

  return {
    // Much stronger repulsion for more nodes
    chargeStrength: -900 * Math.max(1, countFactor * 1.2),
    // Larger link distance for more nodes
    linkDistance: 280 * Math.max(1, countFactor * 0.9),
    // Generous collision radius
    collisionMultiplier: 2.0 + (countFactor * 0.4),
    // Cluster spread scales with viewport and count
    clusterScale: viewportFactor * Math.max(1.2, countFactor * 0.8),
  };
}

/**
 * Create force simulation from React Flow nodes/edges
 */
export function createForceSimulation<N extends Node, E extends Edge>(
  nodes: N[],
  edges: E[],
  options: ForceOptions = {}
): Simulation<ForceNode, ForceLink> {
  // Resolve preset values first, then apply explicit overrides
  const presetKey = options.preset ?? DEFAULT_SPACING_PRESET;
  const presetValues = SPACING_PRESETS[presetKey];

  const opts = {
    ...DEFAULT_OPTIONS,
    // Apply preset values
    chargeStrength: presetValues.chargeStrength,
    linkDistance: presetValues.linkDistance,
    collisionRadius: presetValues.collisionRadius,
    // Allow explicit overrides
    ...options,
  };

  const nodeCount = nodes.length;

  // Get adaptive parameters (for cluster scaling)
  const adaptive = getAdaptiveParams(
    nodeCount,
    opts.viewportWidth,
    opts.viewportHeight
  );

  // Use preset/override values (no fallback to adaptive for main params)
  const chargeStrength = opts.chargeStrength;
  const linkDistance = opts.linkDistance;
  const collisionMultiplier = opts.collisionRadius;
  const clusterScale = adaptive.clusterScale;

  // Convert to force nodes
  const forceNodes: ForceNode[] = nodes.map((n) => ({
    id: n.id,
    x: n.position.x,
    y: n.position.y,
    category: (n.data as { category?: string })?.category,
    nodeType: (n.data as { type?: string })?.type,
  }));

  // Convert to force links
  const forceLinks: ForceLink[] = edges.map((e) => ({
    source: e.source,
    target: e.target,
  }));

  // Create simulation
  const simulation = forceSimulation<ForceNode>(forceNodes)
    .alphaDecay(opts.alphaDecay)
    .velocityDecay(opts.velocityDecay)
    .alphaMin(0.001); // Run longer for better convergence

  // Link force - keeps connected nodes at optimal distance
  simulation.force(
    'link',
    forceLink<ForceNode, ForceLink>(forceLinks)
      .id((d) => d.id)
      .distance(linkDistance)
      .strength(0.4) // Moderate link strength
  );

  // Charge force - type-aware repulsion (larger nodes repel more strongly)
  simulation.force(
    'charge',
    forceManyBody<ForceNode>()
      .strength((d) => {
        // Larger nodes should have stronger repulsion
        const dims = getNodeDimensions(d.nodeType || 'default');
        const sizeFactor = Math.max(dims.width, dims.height) / 200; // Normalize to default size
        return chargeStrength * sizeFactor;
      })
      .distanceMin(60)  // Increased minimum distance for charge
      .distanceMax(1000) // Increased maximum influence distance
      .theta(0.9)       // Barnes-Hut approximation threshold
  );

  // Center force - gentle pull toward center
  simulation.force(
    'center',
    forceCenter<ForceNode>(0, 0).strength(opts.centerStrength)
  );

  // Collision force - type-aware collision radius for accurate spacing
  // Uses actual node dimensions from layout.ts for proper separation
  simulation.force(
    'collision',
    forceCollide<ForceNode>()
      .radius((d) => {
        // Get dimensions for this node type
        const dims = getNodeDimensions(d.nodeType || 'default');
        // Use the larger dimension (width or height) as base radius
        // For circular nodes (LocaleKnowledge), width and height are equal
        const baseRadius = Math.max(dims.width, dims.height) / 2;
        // Apply multiplier for breathing room between nodes
        return baseRadius * collisionMultiplier;
      })
      .strength(0.9) // Strong collision response
      .iterations(3)  // Multiple iterations for better results
  );

  // Category clustering force (optional)
  if (opts.clusterByCategory) {
    simulation.force(
      'x',
      forceX<ForceNode>()
        .x((d) => getCategoryCenter(d.category || 'analytics', clusterScale).x)
        .strength(0.12) // Moderate clustering strength
    );
    simulation.force(
      'y',
      forceY<ForceNode>()
        .y((d) => getCategoryCenter(d.category || 'analytics', clusterScale).y)
        .strength(0.12)
    );
  }

  return simulation;
}

/**
 * Run simulation until stable and return final positions
 */
export function runSimulationSync(
  simulation: Simulation<ForceNode, ForceLink>,
  maxIterations: number = 500 // More iterations for better results
): Map<string, { x: number; y: number }> {
  // Run simulation to completion
  simulation.stop();

  for (let i = 0; i < maxIterations; i++) {
    simulation.tick();
    // Stop early if simulation has cooled down
    if (simulation.alpha() < 0.005) break;
  }

  // Extract final positions
  const positions = new Map<string, { x: number; y: number }>();
  for (const node of simulation.nodes()) {
    positions.set(node.id, { x: node.x || 0, y: node.y || 0 });
  }

  return positions;
}

/**
 * Apply force positions to React Flow nodes
 */
export function applyForcePositions<N extends Node>(
  nodes: N[],
  positions: Map<string, { x: number; y: number }>
): N[] {
  return nodes.map((node) => {
    const pos = positions.get(node.id);
    if (pos) {
      return {
        ...node,
        position: { x: pos.x, y: pos.y },
      };
    }
    return node;
  });
}

export { getCategoryCenter };
