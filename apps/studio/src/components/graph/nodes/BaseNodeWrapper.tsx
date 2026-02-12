'use client';

/**
 * BaseNodeWrapper - Foundation component for all custom node types
 *
 * Features:
 * - GlowingBorder integration for Nika-style effects
 * - Size variants (xs through xl)
 * - Focus mode support (dimmed state)
 * - Consistent handles with glow effects
 */

import { memo, useState } from 'react';
import { Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { glassClasses } from '@/design/tokens';
import { GlowingBorder } from '@/components/ui/GlowingBorder';
import { SelectionHalo } from '../SelectionHalo';
import { EdgeConnectionHalo, type EdgeConnectionRole } from '../EdgeConnectionHalo';
import {
  useUIStore,
  selectHoveredNodeId,
  selectHoveredEdgeId,
  selectSelectedNodeId,
  selectHoveredConnectedNodeIds,
  selectSelectedEdgeData,
} from '@/stores/uiStore';
import type { NodeType } from '@/types';

export interface BaseNodeData extends Record<string, unknown> {
  id: string;
  type: NodeType;
  key: string;
  displayName: string;
  icon?: string;
  description?: string;
  category?: string;
  /** Connection count for size scaling */
  connectionCount?: number;
  /** Whether this node is fully dimmed (focus mode - 15% opacity) */
  dimmed?: boolean;
  /** Whether this node is lightly dimmed (hover mode - 25% opacity) */
  hoverDimmed?: boolean;
  /** Whether this node is in meta mode (blueprint styling) */
  isMetaMode?: boolean;
}

export interface BaseNodeWrapperProps {
  data: BaseNodeData;
  selected?: boolean;
  /** Primary color for glow */
  color: string;
  /** Secondary color for gradient */
  colorSecondary?: string;
  /** Node size variant */
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
  /** Custom shape class */
  shapeClass?: string;
  children: React.ReactNode;
}

const SIZE_CLASSES = {
  xs: 'min-w-[100px] max-w-[140px]',
  sm: 'min-w-[120px] max-w-[180px]',
  md: 'min-w-[160px] max-w-[220px]',
  lg: 'min-w-[200px] max-w-[280px]',
  xl: 'min-w-[240px] max-w-[320px]',
};

export const BaseNodeWrapper = memo(function BaseNodeWrapper({
  data,
  selected = false,
  color,
  colorSecondary,
  size = 'md',
  shapeClass = 'rounded-xl',
  children,
}: BaseNodeWrapperProps) {
  const [isHovered, setIsHovered] = useState(false);

  // ==========================================================================
  // DIRECT STORE SUBSCRIPTION - Bypass React Flow's broken data updates
  // ==========================================================================
  // React Flow's internal memoization blocks data prop updates for nodes.
  // We subscribe directly to Zustand store to get fresh hover state.
  const hoveredNodeId = useUIStore(selectHoveredNodeId);
  const hoveredEdgeId = useUIStore(selectHoveredEdgeId);
  const selectedNodeId = useUIStore(selectSelectedNodeId);
  const hoveredConnectedNodeIds = useUIStore(selectHoveredConnectedNodeIds);
  const selectedEdgeData = useUIStore(selectSelectedEdgeData);

  // Compute edge connection role for EdgeConnectionHalo
  const edgeConnectionRole: EdgeConnectionRole =
    selectedEdgeData?.source === data.id
      ? 'source'
      : selectedEdgeData?.target === data.id
        ? 'target'
        : null;

  // Focus mode dimming (when a node is selected, non-connected nodes dim to 15%)
  // Still comes from data prop as focus mode works correctly
  const isDimmed = data.dimmed === true;

  // Compute hover dimming locally (bypass React Flow's stale data)
  // Edge hover: only edge endpoints visible (stored in hoveredConnectedNodeIds)
  // Node hover: only hovered node + 1-hop connections visible
  // Priority: selectedNode (focus mode) > edgeHover > nodeHover
  const hasHover = hoveredEdgeId !== null || hoveredNodeId !== null;
  const isThisNodeHighlighted =
    data.id === hoveredNodeId || hoveredConnectedNodeIds.has(data.id);
  const isHoverDimmed = !selectedNodeId && hasHover && !isThisNodeHighlighted;

  // Combined dimming state - focus mode takes priority
  const effectiveDimmed = isDimmed || isHoverDimmed;
  // Opacity: focus = 15%, hover = 25%, normal = 100%
  const opacity = isDimmed ? 0.15 : isHoverDimmed ? 0.25 : 1;

  return (
    <div
      className={cn(
        'relative transition duration-200 overflow-visible',
        effectiveDimmed && 'scale-95 pointer-events-none',
        isDimmed && 'grayscale'
      )}
      style={{ opacity, transition: 'opacity 0.15s ease-out, transform 0.2s ease-out' }}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      {/* Edge Connection Halo - OUTSIDE GlowingBorder to avoid overflow clipping */}
      <EdgeConnectionHalo
        role={edgeConnectionRole}
        color={color}
        className={shapeClass}
      />

      <GlowingBorder
        color={color}
        colorSecondary={colorSecondary || color}
        isSelected={selected}
        isHovered={isHovered && !isDimmed}
        animated={selected || isHovered}
        borderRadius={shapeClass.includes('full') ? 9999 : 14}
      >
        <div
          className={cn(
            'relative transition duration-300',
            glassClasses.modal,
            SIZE_CLASSES[size],
            shapeClass
          )}
        >
          {/* Selection Halo - Pulsing ring for selected nodes */}
          <SelectionHalo
            isSelected={selected}
            color={color}
            className={shapeClass}
          />

          {/* Target Handle (invisible - required for React Flow) */}
          <Handle
            type="target"
            position={Position.Top}
            className="!w-0 !h-0 !min-w-0 !min-h-0 !border-0 !bg-transparent !opacity-0"
          />

          {/* Content */}
          <div className="px-4 py-3">
            {children}
          </div>

          {/* Source Handle (invisible - required for React Flow) */}
          <Handle
            type="source"
            position={Position.Bottom}
            className="!w-0 !h-0 !min-w-0 !min-h-0 !border-0 !bg-transparent !opacity-0"
          />
        </div>
      </GlowingBorder>
    </div>
  );
});
