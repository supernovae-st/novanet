/**
 * Node Clustering for Large Graphs
 *
 * When node count exceeds a threshold, automatically groups nodes by layer
 * into cluster nodes. This reduces visual complexity and improves performance.
 *
 * Performance Impact:
 * - Reduces node count from 1000+ to ~10-20 cluster nodes
 * - Each cluster can be expanded on click to show contents
 * - Edges to clustered nodes become edges to the cluster
 */

import type { GraphNode, GraphEdge, NodeType } from '@/types';
import { NODE_LAYERS, type Layer } from '@novanet/core/graph';
import { nodeTypeConfigs } from '@/config/nodeTypes';

/** Threshold above which clustering is activated */
export const CLUSTERING_THRESHOLD = 300;

/** Minimum nodes in a layer to create a cluster (don't cluster tiny groups) */
export const MIN_CLUSTER_SIZE = 5;

/** A cluster node representing multiple nodes of the same layer */
export interface ClusterNode extends Omit<GraphNode, 'type'> {
  type: 'cluster';
  data: {
    layer: Layer;
    nodeCount: number;
    nodeTypes: string[];
    containedNodeIds: string[];
    displayName: string;
    color: string;
  };
}

/** Result of clustering operation */
export interface ClusterResult {
  /** Nodes after clustering (mix of regular nodes and cluster nodes) */
  nodes: (GraphNode | ClusterNode)[];
  /** Edges with clustered node references updated */
  edges: GraphEdge[];
  /** Whether clustering was applied */
  isClustered: boolean;
  /** Map from original node ID to cluster ID (for expansion) */
  nodeToClusterMap: Map<string, string>;
}

/**
 * Cluster nodes by layer when count exceeds threshold.
 * Nodes with high connection counts are kept visible (hub nodes).
 *
 * @param nodes - All graph nodes
 * @param edges - All graph edges
 * @param hubCount - Number of hub nodes to keep visible per layer (default: 3)
 * @returns Clustered result with reduced node count
 */
export function clusterNodesByLayer(
  nodes: GraphNode[],
  edges: GraphEdge[],
  hubCount = 3
): ClusterResult {
  // No clustering needed if below threshold
  if (nodes.length < CLUSTERING_THRESHOLD) {
    return {
      nodes,
      edges,
      isClustered: false,
      nodeToClusterMap: new Map(),
    };
  }

  // Compute connection counts for hub detection
  const connectionCounts = new Map<string, number>();
  for (const edge of edges) {
    connectionCounts.set(edge.source, (connectionCounts.get(edge.source) ?? 0) + 1);
    connectionCounts.set(edge.target, (connectionCounts.get(edge.target) ?? 0) + 1);
  }

  // Group nodes by layer
  const nodesByLayer = new Map<Layer, GraphNode[]>();
  for (const node of nodes) {
    const layer = NODE_LAYERS[node.type as NodeType];
    if (!layer) continue;

    if (!nodesByLayer.has(layer)) {
      nodesByLayer.set(layer, []);
    }
    nodesByLayer.get(layer)!.push(node);
  }

  // Create clusters for large layers, keep small layers unclustered
  const resultNodes: (GraphNode | ClusterNode)[] = [];
  const nodeToClusterMap = new Map<string, string>();

  for (const [layer, layerNodes] of nodesByLayer) {
    if (layerNodes.length < MIN_CLUSTER_SIZE) {
      // Layer too small to cluster - keep all nodes visible
      resultNodes.push(...layerNodes);
      continue;
    }

    // Sort by connection count to find hubs
    const sortedByConnections = [...layerNodes].sort((a, b) => {
      const countA = connectionCounts.get(a.id) ?? 0;
      const countB = connectionCounts.get(b.id) ?? 0;
      return countB - countA;
    });

    // Keep top N hub nodes visible
    const hubNodes = sortedByConnections.slice(0, hubCount);
    const clusteredNodes = sortedByConnections.slice(hubCount);

    // Add hub nodes directly
    resultNodes.push(...hubNodes);

    // Create cluster for remaining nodes
    if (clusteredNodes.length > 0) {
      const clusterId = `cluster-${layer}`;
      const nodeTypes = [...new Set(clusteredNodes.map((n) => n.type))];

      // Get layer color from first node's config
      const firstConfig = nodeTypeConfigs[clusteredNodes[0].type];
      const color = firstConfig?.color ?? '#666';

      const cluster: ClusterNode = {
        id: clusterId,
        key: clusterId,
        type: 'cluster',
        displayName: `${layer} (${clusteredNodes.length})`,
        data: {
          layer,
          nodeCount: clusteredNodes.length,
          nodeTypes,
          containedNodeIds: clusteredNodes.map((n) => n.id),
          displayName: `${layer} (${clusteredNodes.length})`,
          color,
        },
      };

      resultNodes.push(cluster);

      // Map clustered nodes to cluster ID
      for (const node of clusteredNodes) {
        nodeToClusterMap.set(node.id, clusterId);
      }
    }
  }

  // Update edges to point to clusters where needed
  const clusteredEdges = edges.map((edge) => {
    const newSource = nodeToClusterMap.get(edge.source) ?? edge.source;
    const newTarget = nodeToClusterMap.get(edge.target) ?? edge.target;

    // Skip self-loops created by clustering
    if (newSource === newTarget && edge.source !== edge.target) {
      return null;
    }

    return {
      ...edge,
      source: newSource,
      target: newTarget,
    };
  }).filter((e): e is GraphEdge => e !== null);

  // Deduplicate edges (multiple edges between same clusters become one)
  const edgeSet = new Map<string, GraphEdge>();
  for (const edge of clusteredEdges) {
    const key = `${edge.source}->${edge.target}:${edge.type}`;
    if (!edgeSet.has(key)) {
      edgeSet.set(key, edge);
    }
  }

  return {
    nodes: resultNodes,
    edges: Array.from(edgeSet.values()),
    isClustered: true,
    nodeToClusterMap,
  };
}

/**
 * Check if a node is a cluster node
 */
export function isClusterNode(node: GraphNode | ClusterNode): node is ClusterNode {
  return node.type === 'cluster';
}
