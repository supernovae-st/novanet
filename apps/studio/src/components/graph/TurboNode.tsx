'use client';

/**
 * TurboNode - Premium node with gradient edge design
 *
 * Performance optimizations:
 * - Uses memoized style factories to prevent object allocations
 * - Uses NodeConfig lookup table for O(1) color access
 * - Static styles extracted to module-level constants
 *
 * Features:
 * - 2px gradient border (category colors)
 * - Solid/hollow handles for direction indication
 * - Animated border on selection
 * - Enhanced hover effects (scale, glow)
 * - Press feedback (scale down on mousedown)
 * - Hover info displayed in centralized bottom pill (via uiStore.hoveredNodeId)
 */

import { memo, useState, useCallback } from 'react';
import { Handle, Position, type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { iconSizes } from '@/design/tokens';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import { getNodeConfig } from './nodes/NodeConfig';
import {
  getGradientBorderStyle,
  getIconStyle,
  getIconContainerStyle,
  getHandleStyle,
  getBadgeStyle,
  getCategoryBadgeStyle,
  getStatusDotStyle,
  getInnerCardStyle,
  ROOT_TRANSITION_STYLE,
  DISPLAY_NAME_CONTAINER_STYLE,
} from './nodes/NodeStyles';
import type { NodeType } from '@/types';

/**
 * Data structure for TurboNode
 */
export interface TurboNodeData extends Record<string, unknown> {
  id: string;
  type: NodeType;
  key: string;
  displayName: string;
  icon?: string;
  description?: string;
  category?: string;
  /** Full dimming (focus mode - 15% opacity) */
  dimmed?: boolean;
  /** Lighter dimming (hover mode - 25% opacity) */
  hoverDimmed?: boolean;
}

export type TurboNodeType = Node<TurboNodeData>;

/**
 * TurboNode Component - Gradient Edge Design
 */
export const TurboNode = memo(function TurboNode(props: NodeProps<TurboNodeType>) {
  const { data, selected } = props;
  const [isHovered, setIsHovered] = useState(false);
  const [isPressed, setIsPressed] = useState(false);

  // O(1) lookups via pre-computed tables
  const typeConfig = NODE_TYPE_CONFIG[data.type] || NODE_TYPE_CONFIG.Project;
  const nodeConfig = getNodeConfig(data.type);
  const { colors } = nodeConfig;

  const isDimmed = data.dimmed === true;
  const isHoverDimmed = data.hoverDimmed === true;

  const handleMouseEnter = useCallback(() => {
    setIsHovered(true);
  }, []);

  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);
    setIsPressed(false);
  }, []);

  // Use memoized style factories - returns cached references for identical inputs
  const gradientBorderStyle = getGradientBorderStyle(colors.primary, colors.secondary, !!selected, isHovered);
  const iconStyle = getIconStyle(typeConfig.color, !!selected);
  const iconContainerStyle = getIconContainerStyle(colors.primary, colors.secondary);

  return (
    <div
      className={cn(
        'group relative node-pressable',
        // Full dimming (focus mode)
        isDimmed && 'opacity-15 scale-90 grayscale pointer-events-none',
        // Lighter dimming (hover highlight mode)
        isHoverDimmed && !isDimmed && 'hover-dimmed',
        // Enhanced hover effect
        isHovered && !isDimmed && !isHoverDimmed && !selected && 'scale-103',
        // Press feedback
        isPressed && !isDimmed && 'scale-[0.98]',
        // Selection already has its own scale
        selected && 'scale-105'
      )}
      style={ROOT_TRANSITION_STYLE}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={() => setIsPressed(true)}
      onMouseUp={() => setIsPressed(false)}
    >
      {/* Gradient border wrapper */}
      <div
        className={cn(
          'relative p-[2px] rounded-[14px] transition-all duration-300',
          selected && 'animate-gradient-rotate',
          isHovered && !selected && 'animate-glow-pulse'
        )}
        style={gradientBorderStyle}
      >
        {/* Inner card */}
        <div
          className={cn(
            'relative px-4 py-3 rounded-[12px] transition-colors duration-300',
            'min-w-[200px] max-w-[280px]'
          )}
          style={getInnerCardStyle(!!selected)}
        >
          {/* Target Handle - SOLID (incoming) */}
          <Handle
            type="target"
            position={Position.Top}
            className={cn(iconSizes.xs, '!rounded-full !border-2 !-top-1.5 transition-all duration-200')}
            style={getHandleStyle(colors.primary, !!selected, true)}
          />

          {/* Header: Icon + Type label */}
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-3">
              {/* Icon - SVG from Lucide (memory-efficient) */}
              <div
                className={cn(
                  'flex items-center justify-center w-10 h-10 rounded-lg transition-transform duration-200',
                  (isHovered || selected) && 'scale-105'
                )}
                style={iconContainerStyle}
              >
                <CategoryIcon
                  category={typeConfig.category}
                  size={24}
                  strokeWidth={2}
                  style={iconStyle}
                />
              </div>

              {/* Type label */}
              <div
                className="px-2.5 py-1 rounded-md border"
                style={getBadgeStyle(colors.primary)}
              >
                <span
                  className="text-[11px] font-bold uppercase tracking-wider"
                  style={{ color: typeConfig.color }}
                >
                  {typeConfig.label}
                </span>
              </div>
            </div>

            {/* Status dot */}
            <div
              className={cn(
                'w-2.5 h-2.5 rounded-full',
                selected && 'animate-pulse'
              )}
              style={getStatusDotStyle(colors.primary)}
            />
          </div>

          {/* Display Name */}
          <div
            className="px-3 py-2 rounded-lg mb-2 border"
            style={DISPLAY_NAME_CONTAINER_STYLE}
          >
            <div
              className="text-[15px] font-bold text-white truncate"
            >
              {data.displayName}
            </div>
            {data.key !== data.displayName && (
              <div className="text-[10px] text-white/40 font-mono truncate mt-1">
                {data.key}
              </div>
            )}
          </div>

          {/* Category badge */}
          <div
            className="inline-flex items-center gap-2 px-3 py-1.5 rounded-full text-[10px] font-bold uppercase tracking-wider border"
            style={getCategoryBadgeStyle(colors.primary)}
          >
            <span
              className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
              style={getStatusDotStyle(colors.primary)}
            />
            {typeConfig.category}
          </div>

          {/* Source Handle - HOLLOW (outgoing) */}
          <Handle
            type="source"
            position={Position.Bottom}
            className={cn(iconSizes.xs, '!rounded-full !border-2 !-bottom-1.5 transition-all duration-200')}
            style={getHandleStyle(colors.primary, !!selected, false)}
          />
        </div>
      </div>

    </div>
  );
});
