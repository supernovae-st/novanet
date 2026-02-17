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

import { memo } from 'react';
import { Handle, Position, type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { iconSizes, gapTokens } from '@/design/tokens';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { useNodeInteractions } from '@/hooks';
import { localeToFlag } from '@/lib/localeUtils';
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
  /** BCP-47 locale code for locale-specific nodes (*Native, Knowledge atoms, Locale layer) */
  locale?: string;
}

export type TurboNodeType = Node<TurboNodeData>;

/**
 * TurboNode Component - Gradient Edge Design
 */
export const TurboNode = memo(function TurboNode(props: NodeProps<TurboNodeType>) {
  const { data, selected } = props;

  // O(1) lookups via pre-computed tables
  const typeConfig = NODE_TYPE_CONFIG[data.type] || NODE_TYPE_CONFIG.Project;
  const nodeConfig = getNodeConfig(data.type);
  const { colors } = nodeConfig;

  const isDimmed = data.dimmed === true;
  const isHoverDimmed = data.hoverDimmed === true;

  // Shared interaction state management (same as SchemaNode)
  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
    containerClassName,
    containerStyle,
  } = useNodeInteractions({ selected: !!selected, isDimmed, isHoverDimmed });

  // Use memoized style factories - returns cached references for identical inputs
  const gradientBorderStyle = getGradientBorderStyle(colors.primary, colors.secondary, !!selected, isHovered);
  const iconStyle = getIconStyle(typeConfig.color, !!selected);
  const iconContainerStyle = getIconContainerStyle(colors.primary, colors.secondary);

  return (
    <div
      className={containerClassName}
      style={containerStyle}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
    >
      {/* Gradient border wrapper */}
      <div
        className={cn(
          'relative p-0.5 rounded-[14px] transition-all duration-300',
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
            className={cn(iconSizes.xs, '!rounded-full !border-2 !-top-1.5 transition duration-200')}
            style={getHandleStyle(colors.primary, !!selected, true)}
          />

          {/* Header: Icon + Type label */}
          <div className="flex items-center justify-between mb-3">
            <div className={cn('flex items-center', gapTokens.spacious)}>
              {/* Icon - SVG from Lucide (memory-efficient) */}
              <div
                className={cn(
                  'flex items-center justify-center w-10 h-10 rounded-lg transition-transform duration-200',
                  (isHovered || selected) && 'scale-105'
                )}
                style={iconContainerStyle}
              >
                <LayerIcon
                  layer={typeConfig.layer}
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

          {/* Badges row: Category + Locale */}
          <div className={cn('flex items-center flex-wrap', gapTokens.default)}>
            {/* Category badge */}
            <div
              className={cn('inline-flex items-center px-3 py-1.5 rounded-full text-[10px] font-bold uppercase tracking-wider border', gapTokens.default)}
              style={getCategoryBadgeStyle(colors.primary)}
            >
              <span
                className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
                style={getStatusDotStyle(colors.primary)}
              />
              {typeConfig.layer}
            </div>

            {/* Locale tag badge - only shown for locale-specific nodes */}
            {data.locale && (
              <div
                className="inline-flex items-center px-2.5 py-1.5 rounded-full text-[10px] font-medium border gap-1.5"
                style={{
                  background: 'rgba(255, 255, 255, 0.08)',
                  borderColor: 'rgba(255, 255, 255, 0.15)',
                }}
              >
                <span className="text-base leading-none">{localeToFlag(data.locale)}</span>
                <span className="text-white/70 font-mono">{data.locale}</span>
              </div>
            )}
          </div>

          {/* Source Handle - HOLLOW (outgoing) */}
          <Handle
            type="source"
            position={Position.Bottom}
            className={cn(iconSizes.xs, '!rounded-full !border-2 !-bottom-1.5 transition duration-200')}
            style={getHandleStyle(colors.primary, !!selected, false)}
          />
        </div>
      </div>

    </div>
  );
});
