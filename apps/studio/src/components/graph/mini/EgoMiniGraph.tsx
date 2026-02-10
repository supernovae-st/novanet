'use client';

/**
 * EgoMiniGraph - Compact React Flow graph for sidebar
 *
 * Features:
 * - Shows selected node + direct neighbors
 * - Simplified nodes (circles with layer colors)
 * - Auto-fit layout
 * - Click to navigate in main canvas
 */

import { memo, useMemo, useCallback } from 'react';
import {
  ReactFlow,
  Background,
  useNodesState,
  useEdgesState,
  type Node,
  type Edge,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { MiniNode, type MiniNodeData } from './MiniNode';
import { MiniEdge, type MiniEdgeData } from './MiniEdge';
import type { GraphNode } from '@/types';
import type { Edge as StudioEdge } from '@xyflow/react';
import { KIND_META } from '@novanet/core/types';

// Node types for React Flow
const nodeTypes = {
  mini: MiniNode,
};

// Edge types for React Flow
const edgeTypes = {
  mini: MiniEdge,
};

interface EgoMiniGraphProps {
  centerNode: GraphNode;
  relatedEdges: StudioEdge[];
  relatedNodes: GraphNode[];
  onNodeClick?: (nodeId: string) => void;
}

/**
 * Layout nodes in a radial pattern around center
 */
function createRadialLayout(
  centerNode: GraphNode,
  relatedNodes: GraphNode[],
  relatedEdges: StudioEdge[]
): { nodes: Node<MiniNodeData>[]; edges: Edge<MiniEdgeData>[] } {
  const centerX = 120;
  const centerY = 80;
  const radius = 60;

  // Get meta info for center node
  const centerMeta = KIND_META[centerNode.type];

  // Center node
  const nodes: Node<MiniNodeData>[] = [
    {
      id: centerNode.id,
      type: 'mini',
      position: { x: centerX, y: centerY },
      data: {
        label: centerNode.displayName,
        layer: centerMeta?.layer,
        realm: centerMeta?.realm,
        isCenter: true,
      },
    },
  ];

  // Related nodes in radial layout
  const angleStep = (2 * Math.PI) / Math.max(relatedNodes.length, 1);

  relatedNodes.forEach((node, index) => {
    const angle = angleStep * index - Math.PI / 2; // Start from top
    const x = centerX + radius * Math.cos(angle);
    const y = centerY + radius * Math.sin(angle);

    const meta = KIND_META[node.type];

    nodes.push({
      id: node.id,
      type: 'mini',
      position: { x, y },
      data: {
        label: node.displayName,
        layer: meta?.layer,
        realm: meta?.realm,
        isCenter: false,
      },
    });
  });

  // Edges
  const edges: Edge<MiniEdgeData>[] = relatedEdges.map((edge) => {
    // Determine arc family from edge type
    let family: MiniEdgeData['family'] = undefined;
    const edgeType = edge.type || '';

    if (edgeType.startsWith('HAS_') || edgeType.includes('BELONGS')) {
      family = 'ownership';
    } else if (edgeType.includes('CONTENT') || edgeType.includes('GENERATED')) {
      family = 'localization';
    } else if (edgeType.includes('USES') || edgeType.includes('LINKS')) {
      family = 'semantic';
    } else if (edgeType.includes('GENERATION') || edgeType.includes('PROMPT')) {
      family = 'generation';
    } else if (edgeType.includes('SEO') || edgeType.includes('GEO')) {
      family = 'mining';
    }

    return {
      id: edge.id,
      source: edge.source,
      target: edge.target,
      type: 'mini',
      data: { family },
    };
  });

  return { nodes, edges };
}

export const EgoMiniGraph = memo(function EgoMiniGraph({
  centerNode,
  relatedEdges,
  relatedNodes,
  onNodeClick,
}: EgoMiniGraphProps) {
  // Create layout
  const { nodes: initialNodes, edges: initialEdges } = useMemo(
    () => createRadialLayout(centerNode, relatedNodes, relatedEdges),
    [centerNode, relatedNodes, relatedEdges]
  );

  const [nodes] = useNodesState(initialNodes);
  const [edges] = useEdgesState(initialEdges);

  // Handle node click
  const handleNodeClick = useCallback(
    (_: React.MouseEvent, node: Node) => {
      if (node.id !== centerNode.id && onNodeClick) {
        onNodeClick(node.id);
      }
    },
    [centerNode.id, onNodeClick]
  );

  // Empty state
  if (relatedNodes.length === 0) {
    return (
      <div className="flex items-center justify-center h-40 text-white/30 text-xs">
        No connected nodes
      </div>
    );
  }

  return (
    <div className="h-40 w-full rounded-lg overflow-hidden bg-black/20">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        onNodeClick={handleNodeClick}
        fitView
        fitViewOptions={{ padding: 0.3 }}
        panOnDrag={false}
        zoomOnScroll={false}
        zoomOnPinch={false}
        zoomOnDoubleClick={false}
        nodesDraggable={false}
        nodesConnectable={false}
        elementsSelectable={false}
        proOptions={{ hideAttribution: true }}
        style={{ background: 'transparent' }}
      >
        <Background color="rgba(255,255,255,0.03)" gap={16} />
      </ReactFlow>
    </div>
  );
});

export default EgoMiniGraph;
