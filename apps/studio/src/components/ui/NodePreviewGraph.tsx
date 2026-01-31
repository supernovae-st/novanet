/**
 * NodePreviewGraph - Mini graph visualization for node context
 *
 * Shows a selected node with its immediate connections in a compact
 * preview format. Perfect for info panels and tooltips.
 *
 * Features:
 * - Central node with category color
 * - Connected nodes arranged radially
 * - Animated edges with glow effect
 * - Hover states for interaction
 * - Responsive sizing
 */

'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';
import { getNodeConfig } from '@/components/graph/nodes/NodeConfig';
import type { NodeType } from '@/types';

export interface PreviewNode {
  id: string;
  type: NodeType;
  displayName: string;
  isCenter?: boolean;
}

export interface PreviewEdge {
  source: string;
  target: string;
  type?: string;
}

export interface NodePreviewGraphProps {
  /** The central/selected node */
  centerNode: PreviewNode;
  /** Connected nodes to display */
  connectedNodes: PreviewNode[];
  /** Edges between nodes */
  edges: PreviewEdge[];
  /** Width of the preview (default: 200) */
  width?: number;
  /** Height of the preview (default: 160) */
  height?: number;
  /** Optional className */
  className?: string;
  /** Callback when a node is clicked */
  onNodeClick?: (nodeId: string) => void;
}

// Constants for layout
const NODE_RADIUS = 20;
const CENTER_NODE_RADIUS = 28;
const ORBIT_RADIUS_RATIO = 0.35;

/**
 * Calculate radial positions for connected nodes
 */
function calculateNodePositions(
  centerX: number,
  centerY: number,
  nodeCount: number,
  orbitRadius: number
): Array<{ x: number; y: number }> {
  const positions: Array<{ x: number; y: number }> = [];
  const startAngle = -Math.PI / 2; // Start from top

  for (let i = 0; i < nodeCount; i++) {
    const angle = startAngle + (2 * Math.PI * i) / nodeCount;
    positions.push({
      x: centerX + orbitRadius * Math.cos(angle),
      y: centerY + orbitRadius * Math.sin(angle),
    });
  }

  return positions;
}

export function NodePreviewGraph({
  centerNode,
  connectedNodes,
  edges,
  width = 200,
  height = 160,
  className,
  onNodeClick,
}: NodePreviewGraphProps) {
  const [hoveredNodeId, setHoveredNodeId] = React.useState<string | null>(null);

  const centerX = width / 2;
  const centerY = height / 2;
  const orbitRadius = Math.min(width, height) * ORBIT_RADIUS_RATIO;

  // Calculate positions for connected nodes (guard against empty array)
  const nodePositions = connectedNodes.length > 0
    ? calculateNodePositions(centerX, centerY, connectedNodes.length, orbitRadius)
    : [];

  // Build position map for edge drawing
  const positionMap = new Map<string, { x: number; y: number }>();
  positionMap.set(centerNode.id, { x: centerX, y: centerY });
  connectedNodes.forEach((node, index) => {
    positionMap.set(node.id, nodePositions[index]);
  });

  // Get colors for center node
  const centerConfig = getNodeConfig(centerNode.type);

  return (
    <div className={cn('relative', className)}>
      <svg
        width={width}
        height={height}
        viewBox={`0 0 ${width} ${height}`}
        className="overflow-visible"
      >
        {/* Definitions for gradients and filters */}
        <defs>
          {/* Glow filter for edges */}
          <filter id="edge-glow" x="-50%" y="-50%" width="200%" height="200%">
            <feGaussianBlur stdDeviation="2" result="blur" />
            <feMerge>
              <feMergeNode in="blur" />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>

          {/* Glow filter for nodes */}
          <filter id="node-glow" x="-100%" y="-100%" width="300%" height="300%">
            <feGaussianBlur stdDeviation="4" result="blur" />
            <feMerge>
              <feMergeNode in="blur" />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>

          {/* Gradient for center node */}
          <radialGradient id={`center-gradient-${centerNode.id}`}>
            <stop offset="0%" stopColor={centerConfig.colors.primary} stopOpacity="0.9" />
            <stop offset="100%" stopColor={centerConfig.colors.primary} stopOpacity="0.6" />
          </radialGradient>
        </defs>

        {/* Draw edges */}
        <g className="edges">
          {edges.map((edge, index) => {
            const sourcePos = positionMap.get(edge.source);
            const targetPos = positionMap.get(edge.target);

            if (!sourcePos || !targetPos) return null;

            const isHovered =
              hoveredNodeId === edge.source || hoveredNodeId === edge.target;

            return (
              <line
                key={`edge-${index}`}
                x1={sourcePos.x}
                y1={sourcePos.y}
                x2={targetPos.x}
                y2={targetPos.y}
                stroke={isHovered ? centerConfig.colors.primary : 'rgba(255,255,255,0.2)'}
                strokeWidth={isHovered ? 2 : 1}
                strokeLinecap="round"
                filter={isHovered ? 'url(#edge-glow)' : undefined}
                className="transition-all duration-200"
              />
            );
          })}
        </g>

        {/* Draw connected nodes */}
        <g className="connected-nodes">
          {connectedNodes.map((node, index) => {
            const pos = nodePositions[index];
            const nodeConfig = getNodeConfig(node.type);
            const isHovered = hoveredNodeId === node.id;

            return (
              <g
                key={node.id}
                transform={`translate(${pos.x}, ${pos.y})`}
                className="cursor-pointer"
                onMouseEnter={() => setHoveredNodeId(node.id)}
                onMouseLeave={() => setHoveredNodeId(null)}
                onClick={() => onNodeClick?.(node.id)}
              >
                {/* Node circle */}
                <circle
                  r={isHovered ? NODE_RADIUS + 2 : NODE_RADIUS}
                  fill="rgba(0,0,0,0.6)"
                  stroke={nodeConfig.colors.primary}
                  strokeWidth={isHovered ? 2 : 1}
                  filter={isHovered ? 'url(#node-glow)' : undefined}
                  className="transition-all duration-150"
                />
                {/* Node type initial */}
                <text
                  textAnchor="middle"
                  dominantBaseline="central"
                  fill={nodeConfig.colors.primary}
                  fontSize={10}
                  fontWeight={500}
                  className="select-none pointer-events-none"
                >
                  {node.type.charAt(0)}
                </text>
              </g>
            );
          })}
        </g>

        {/* Draw center node */}
        <g
          transform={`translate(${centerX}, ${centerY})`}
          className="cursor-pointer"
          onMouseEnter={() => setHoveredNodeId(centerNode.id)}
          onMouseLeave={() => setHoveredNodeId(null)}
          onClick={() => onNodeClick?.(centerNode.id)}
        >
          {/* Outer glow ring */}
          <circle
            r={CENTER_NODE_RADIUS + 6}
            fill="none"
            stroke={centerConfig.colors.primary}
            strokeWidth={1}
            strokeOpacity={0.3}
            className="animate-pulse"
          />

          {/* Main circle */}
          <circle
            r={hoveredNodeId === centerNode.id ? CENTER_NODE_RADIUS + 2 : CENTER_NODE_RADIUS}
            fill={`url(#center-gradient-${centerNode.id})`}
            stroke={centerConfig.colors.primary}
            strokeWidth={2}
            filter="url(#node-glow)"
            className="transition-all duration-150"
          />

          {/* Node type initial */}
          <text
            textAnchor="middle"
            dominantBaseline="central"
            fill="white"
            fontSize={14}
            fontWeight={600}
            className="select-none pointer-events-none"
          >
            {centerNode.type.charAt(0)}
          </text>
        </g>
      </svg>

      {/* Node count badge */}
      <div className="absolute bottom-1 right-1 px-1.5 py-0.5 bg-white/10 rounded text-[10px] text-white/50">
        {connectedNodes.length} connections
      </div>
    </div>
  );
}

export default NodePreviewGraph;
