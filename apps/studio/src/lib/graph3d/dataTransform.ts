/**
 * Data Transform for 3D Graph Visualization
 *
 * Converts NovaNet graph data to react-force-graph-3d format.
 */

import type { GraphNode, GraphEdge } from '@/types';
import type { Layer, Realm } from '@novanet/core/types';
import { KIND_META } from '@novanet/core/types';

// Node format for react-force-graph-3d
export interface ForceGraphNode {
  id: string;
  name: string;
  type: string;
  layer: Layer;
  realm: Realm;
  trait: string;
  description?: string;
  // Force graph properties
  val?: number; // Node size
  color?: string;
  // Position (set by force simulation)
  x?: number;
  y?: number;
  z?: number;
  // Velocity (used by simulation)
  vx?: number;
  vy?: number;
  vz?: number;
}

// Link format for react-force-graph-3d
export interface ForceGraphLink {
  id: string;
  source: string;
  target: string;
  type: string;
  // Additional properties
  curvature?: number;
  color?: string;
}

// Complete graph data format
export interface ForceGraphData {
  nodes: ForceGraphNode[];
  links: ForceGraphLink[];
}

/**
 * Transform GraphNode to ForceGraphNode
 */
function transformNode(node: GraphNode): ForceGraphNode {
  const meta = KIND_META[node.type];

  return {
    id: node.id,
    name: node.displayName,
    type: node.type,
    layer: meta?.layer || 'semantic',
    realm: meta?.realm || 'org',
    trait: meta?.trait || 'invariant',
    description: node.description,
    val: getNodeSize(meta?.layer),
  };
}

/**
 * Get node size based on layer importance
 */
function getNodeSize(layer?: Layer): number {
  const sizeMap: Record<Layer, number> = {
    foundation: 8,   // Largest (project, brand)
    structure: 6,    // Large (page, block)
    semantic: 5,     // Medium (entity, content)
    instruction: 4,  // Medium-small (prompts)
    output: 4,       // Medium-small (generated)
    config: 3,       // Small (configuration)
    locale: 5,       // Medium (locale settings)
    geography: 4,    // Medium-small
    knowledge: 3,    // Small (atoms)
  };

  return layer ? sizeMap[layer] : 4;
}

/**
 * Transform GraphEdge to ForceGraphLink
 */
function transformEdge(edge: GraphEdge): ForceGraphLink {
  return {
    id: edge.id,
    source: edge.source,
    target: edge.target,
    type: edge.type,
  };
}

/**
 * Transform complete graph data for 3D visualization
 */
export function transformGraphData(
  nodes: GraphNode[],
  edges: GraphEdge[]
): ForceGraphData {
  return {
    nodes: nodes.map(transformNode),
    links: edges.map(transformEdge),
  };
}

/**
 * Filter graph data to remove orphan nodes/links
 */
export function filterValidData(data: ForceGraphData): ForceGraphData {
  const nodeIds = new Set(data.nodes.map((n) => n.id));

  // Filter links to only include those with valid source/target
  const validLinks = data.links.filter(
    (link) => nodeIds.has(link.source as string) && nodeIds.has(link.target as string)
  );

  // Get connected node IDs
  const connectedNodeIds = new Set<string>();
  validLinks.forEach((link) => {
    connectedNodeIds.add(link.source as string);
    connectedNodeIds.add(link.target as string);
  });

  // Keep all nodes (including orphans for now - they'll be positioned separately)
  return {
    nodes: data.nodes,
    links: validLinks,
  };
}
