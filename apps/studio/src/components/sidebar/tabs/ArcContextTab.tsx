'use client';

/**
 * ArcContextTab - Arc context with mini-graph visualization
 *
 * Features:
 * - Mini React Flow graph showing source → arc → target
 * - Highlighted arc with animated glow
 * - Clickable nodes to navigate
 * - Relation type label on edge
 *
 * v11.7 — Enhanced arc experience
 */

import { memo, useMemo, useCallback } from 'react';
import {
  ReactFlow,
  Background,
  Node,
  Edge,
  ConnectionLineType,
  Position,
} from '@xyflow/react';
import { motion } from 'motion/react';
import { Maximize2, Network } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useUIStore } from '@/stores/uiStore';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { gapTokens } from '@/design/tokens';
import type { GraphEdge, GraphNode } from '@/types';

import '@xyflow/react/dist/style.css';

interface ArcContextTabProps {
  arc: GraphEdge;
  sourceNode: GraphNode | null;
  targetNode: GraphNode | null;
}

/**
 * Simple node component for mini-graph
 */
const MiniNode = memo(function MiniNode({ data }: { data: { label: string; type: string; color: string } }) {
  return (
    <div
      className="px-3 py-2 rounded-lg border-2 text-xs font-medium min-w-[80px] text-center truncate"
      style={{
        background: `${data.color}20`,
        borderColor: `${data.color}60`,
        color: data.color,
      }}
    >
      <div className="truncate">{data.label}</div>
      <div className="text-[9px] opacity-60 truncate">{data.type}</div>
    </div>
  );
});

const nodeTypes = {
  mini: MiniNode,
};

/**
 * Build nodes and edges for the mini-graph
 */
function buildMiniGraph(
  arc: GraphEdge,
  sourceNode: GraphNode | null,
  targetNode: GraphNode | null,
  arcColors: { primary: string; glow: string }
): { nodes: Node[]; edges: Edge[] } {
  const sourceConfig = sourceNode ? NODE_TYPE_CONFIG[sourceNode.type] : null;
  const targetConfig = targetNode ? NODE_TYPE_CONFIG[targetNode.type] : null;

  const nodes: Node[] = [
    {
      id: arc.source,
      type: 'mini',
      position: { x: 50, y: 80 },
      data: {
        label: sourceNode?.displayName || 'Source',
        type: sourceNode?.type || 'Unknown',
        color: sourceConfig?.color || '#6366f1',
      },
      sourcePosition: Position.Right,
      targetPosition: Position.Left,
    },
    {
      id: arc.target,
      type: 'mini',
      position: { x: 250, y: 80 },
      data: {
        label: targetNode?.displayName || 'Target',
        type: targetNode?.type || 'Unknown',
        color: targetConfig?.color || '#6366f1',
      },
      sourcePosition: Position.Right,
      targetPosition: Position.Left,
    },
  ];

  const arcType = arc.type || 'RELATES_TO';

  const edges: Edge[] = [
    {
      id: arc.id,
      source: arc.source,
      target: arc.target,
      type: 'default',
      label: arcType,
      labelStyle: {
        fill: arcColors.primary,
        fontSize: 10,
        fontWeight: 600,
      },
      labelBgStyle: {
        fill: 'rgba(0, 0, 0, 0.8)',
        fillOpacity: 0.9,
      },
      labelBgPadding: [4, 6] as [number, number],
      labelBgBorderRadius: 4,
      animated: true,
      style: {
        stroke: arcColors.primary,
        strokeWidth: 2,
        filter: `drop-shadow(0 0 6px ${arcColors.glow})`,
      },
      markerEnd: {
        type: 'arrowclosed' as const,
        color: arcColors.primary,
        width: 20,
        height: 20,
      },
    },
  ];

  return { nodes, edges };
}

export const ArcContextTab = memo(function ArcContextTab({
  arc,
  sourceNode,
  targetNode,
}: ArcContextTabProps) {
  const setSelectedNode = useUIStore((state) => state.setSelectedNode);

  // Get arc colors
  const arcType = arc.type || 'UNKNOWN';
  const arcColors = useMemo(() => {
    // Simple color mapping based on arc type
    if (arcType.includes('HAS_') || arcType.includes('CONTAINS')) {
      return { primary: '#3b82f6', glow: '#60a5fa' };
    }
    if (arcType.includes('FOR_LOCALE') || arcType.includes('CONTENT')) {
      return { primary: '#10b981', glow: '#34d399' };
    }
    if (arcType.includes('OUTPUT') || arcType.includes('GENERATES')) {
      return { primary: '#f97316', glow: '#fb923c' };
    }
    return { primary: '#6366f1', glow: '#818cf8' };
  }, [arcType]);

  // Build graph data
  const { nodes, edges } = useMemo(
    () => buildMiniGraph(arc, sourceNode, targetNode, arcColors),
    [arc, sourceNode, targetNode, arcColors]
  );

  // Handle node click
  const handleNodeClick = useCallback(
    (_event: React.MouseEvent, node: Node) => {
      setSelectedNode(node.id);
    },
    [setSelectedNode]
  );

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className={cn('flex items-center justify-between px-4 py-3 border-b border-white/[0.06]', gapTokens.default)}>
        <div className={cn('flex items-center text-xs text-white/50', gapTokens.default)}>
          <Network className="w-4 h-4" />
          <span>Arc Context</span>
        </div>
        <button
          className="p-1.5 rounded hover:bg-white/10 text-white/40 hover:text-white/60 transition-colors"
          title="Expand view"
        >
          <Maximize2 className="w-3.5 h-3.5" />
        </button>
      </div>

      {/* Mini graph */}
      <motion.div
        className="flex-1 min-h-[200px]"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.3 }}
      >
        <ReactFlow
          nodes={nodes}
          edges={edges}
          nodeTypes={nodeTypes}
          onNodeClick={handleNodeClick}
          connectionLineType={ConnectionLineType.SmoothStep}
          fitView
          fitViewOptions={{
            padding: 0.3,
            maxZoom: 1.5,
          }}
          panOnDrag={false}
          zoomOnScroll={false}
          zoomOnPinch={false}
          zoomOnDoubleClick={false}
          nodesDraggable={false}
          nodesConnectable={false}
          elementsSelectable={false}
          proOptions={{ hideAttribution: true }}
        >
          <Background color="#1e293b" gap={16} size={1} />
        </ReactFlow>
      </motion.div>

      {/* Legend */}
      <div className="px-4 py-3 border-t border-white/[0.06] bg-black/20">
        <div className="flex items-center justify-center gap-6 text-[10px] text-white/40">
          <div className="flex items-center gap-1.5">
            <div
              className="w-2 h-2 rounded-full"
              style={{ background: NODE_TYPE_CONFIG[sourceNode?.type || 'Project']?.color || '#6366f1' }}
            />
            <span>Source</span>
          </div>
          <div className="flex items-center gap-1.5">
            <div
              className="w-4 h-0.5 rounded-full"
              style={{ background: arcColors.primary }}
            />
            <span>{arcType}</span>
          </div>
          <div className="flex items-center gap-1.5">
            <div
              className="w-2 h-2 rounded-full"
              style={{ background: NODE_TYPE_CONFIG[targetNode?.type || 'Project']?.color || '#6366f1' }}
            />
            <span>Target</span>
          </div>
        </div>
        <p className="text-center text-[10px] text-white/25 mt-2">
          Click nodes to navigate
        </p>
      </div>
    </div>
  );
});

export default ArcContextTab;
