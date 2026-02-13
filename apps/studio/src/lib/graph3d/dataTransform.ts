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
  // Pre-computed connection count (avoids O(n×m) lookup in render)
  connectionCount: number;
  // Position (set by force simulation)
  x?: number;
  y?: number;
  z?: number;
  // Velocity (used by simulation)
  vx?: number;
  vy?: number;
  vz?: number;
}

/**
 * Link endpoint can be string (before D3 simulation) or object (after simulation).
 * D3 force simulation mutates source/target from string to { id, x, y, z, ... }.
 */
export type LinkEndpoint = string | { id?: string } | undefined | null;

// Link format for react-force-graph-3d
export interface ForceGraphLink {
  id: string;
  source: string | LinkEndpoint; // String initially, object after D3 simulation
  target: string | LinkEndpoint; // String initially, object after D3 simulation
  type: string;
  // Additional properties
  curvature?: number;
  color?: string;
}

/**
 * Safely extract node ID from a link endpoint.
 * D3 force simulation mutates source/target from string to object with id property.
 *
 * @param endpoint - Link source or target (string | { id?: string } | undefined | null)
 * @returns Node ID string, or empty string if invalid
 */
export function getNodeIdFromLinkEndpoint(endpoint: LinkEndpoint): string {
  if (!endpoint) return '';
  if (typeof endpoint === 'string') return endpoint;
  if (typeof endpoint === 'object' && 'id' in endpoint) {
    return endpoint.id ?? '';
  }
  return '';
}

// Complete graph data format
export interface ForceGraphData {
  nodes: ForceGraphNode[];
  links: ForceGraphLink[];
}

/**
 * Transform GraphNode to ForceGraphNode (without connectionCount - added later)
 * Initializes x/y/z to prevent "undefined" errors in Three.js DragControls
 */
function transformNode(node: GraphNode): Omit<ForceGraphNode, 'connectionCount'> {
  const meta = KIND_META[node.type];

  // Generate deterministic initial position based on node id hash
  // This prevents "Cannot read properties of undefined (reading 'x')" in DragControls
  const hash = node.id.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
  const spread = 100;

  return {
    id: node.id,
    name: node.displayName,
    type: node.type,
    layer: meta?.layer || 'semantic',
    realm: meta?.realm || 'org',
    trait: meta?.trait || 'defined',
    description: node.description,
    val: getNodeSize(meta?.layer),
    // Initialize positions to prevent DragControls crash
    x: (hash % spread) - spread / 2,
    y: ((hash * 7) % spread) - spread / 2,
    z: ((hash * 13) % spread) - spread / 2,
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
 * Pre-compute connection counts for all nodes in O(n + m)
 * This avoids the O(n × m) filter in renderNode that was causing 10-20s freezes
 */
function computeConnectionCounts(edges: GraphEdge[]): Map<string, number> {
  const counts = new Map<string, number>();

  for (const edge of edges) {
    counts.set(edge.source, (counts.get(edge.source) ?? 0) + 1);
    counts.set(edge.target, (counts.get(edge.target) ?? 0) + 1);
  }

  return counts;
}

/**
 * Transform complete graph data for 3D visualization
 */
export function transformGraphData(
  nodes: GraphNode[],
  edges: GraphEdge[]
): ForceGraphData {
  // Pre-compute connection counts O(m) instead of O(n × m) per-node lookup
  const connectionCounts = computeConnectionCounts(edges);

  return {
    nodes: nodes.map((node) => ({
      ...transformNode(node),
      connectionCount: connectionCounts.get(node.id) ?? 0,
    })),
    links: edges.map(transformEdge),
  };
}

/**
 * Filter graph data to remove orphan nodes/links
 */
export function filterValidData(data: ForceGraphData): ForceGraphData {
  const nodeIds = new Set(data.nodes.map((n) => n.id));

  // Filter links to only include those with valid source/target
  const validLinks = data.links.filter((link) => {
    const sourceId = getNodeIdFromLinkEndpoint(link.source as LinkEndpoint);
    const targetId = getNodeIdFromLinkEndpoint(link.target as LinkEndpoint);
    return sourceId !== '' && targetId !== '' && nodeIds.has(sourceId) && nodeIds.has(targetId);
  });

  // Get connected node IDs
  const connectedNodeIds = new Set<string>();
  validLinks.forEach((link) => {
    const sourceId = getNodeIdFromLinkEndpoint(link.source as LinkEndpoint);
    const targetId = getNodeIdFromLinkEndpoint(link.target as LinkEndpoint);
    if (sourceId) connectedNodeIds.add(sourceId);
    if (targetId) connectedNodeIds.add(targetId);
  });

  // Keep all nodes (including orphans for now - they'll be positioned separately)
  return {
    nodes: data.nodes,
    links: validLinks,
  };
}
