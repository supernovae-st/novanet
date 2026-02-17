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
 * - Animated gradient ring on selection with rotation
 * - Premium hover effects (scale, shadow, backdrop blur)
 * - Enhanced connection port visibility on hover
 * - Press feedback (scale down on mousedown)
 * - Hover info displayed in centralized bottom pill (via uiStore.hoveredNodeId)
 *
 * v11.7 Premium enhancements:
 * - Animated conic gradient selection ring
 * - Backdrop blur on hover for depth
 * - Enhanced handle glow and positioning
 * - Smoother spring-based transitions
 */

import { memo, useMemo } from 'react';
import { Handle, Position, type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { useNodeInteractions } from '@/hooks';
import { localeToFlag } from '@/lib/localeUtils';
import { getNodeConfig } from './nodes/NodeConfig';
import {
  getGradientBorderStyle,
  getIconStyle,
  getIconContainerStyle,
  getBadgeStyle,
  getCategoryBadgeStyle,
  getStatusDotStyle,
  getInnerCardStyle,
  DISPLAY_NAME_CONTAINER_STYLE,
} from './nodes/NodeStyles';
import type { NodeType } from '@/types';

// Premium easing curves
const SPRING_EASING = 'cubic-bezier(0.34, 1.56, 0.64, 1)';
const SMOOTH_EASING = 'cubic-bezier(0.25, 0.46, 0.45, 0.94)';

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
 * Premium handle style with enhanced visibility
 */
function getPremiumHandleStyle(
  primaryColor: string,
  isSelected: boolean,
  isHovered: boolean,
  isSolid: boolean
): React.CSSProperties {
  const glowIntensity = isSelected ? 1 : isHovered ? 0.7 : 0.3;
  const size = isHovered || isSelected ? 14 : 12;

  return {
    width: size,
    height: size,
    backgroundColor: isSolid ? primaryColor : 'rgba(24, 24, 31, 0.95)',
    borderColor: primaryColor,
    borderWidth: isSolid ? 0 : 2,
    boxShadow: `
      0 0 ${8 * glowIntensity}px ${primaryColor}${isSelected ? 'cc' : isHovered ? '99' : '66'},
      0 0 ${16 * glowIntensity}px ${primaryColor}${isSelected ? '66' : isHovered ? '44' : '22'}
    `,
    transition: `all 200ms ${SMOOTH_EASING}`,
  };
}

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

  // Premium wrapper style with enhanced hover effects
  const wrapperStyle = useMemo<React.CSSProperties>(() => ({
    ...containerStyle,
    // Enhanced shadow on hover
    boxShadow: isHovered && !isDimmed
      ? `0 20px 40px -12px rgba(0, 0, 0, 0.4), 0 0 60px ${colors.primary}25`
      : containerStyle.boxShadow,
  }), [containerStyle, isHovered, isDimmed, colors.primary]);

  // Selection ring style with animated gradient
  const selectionRingStyle = useMemo<React.CSSProperties>(() => {
    if (!selected) return {};

    return {
      position: 'absolute' as const,
      inset: -4,
      borderRadius: 18,
      background: `conic-gradient(from var(--ring-angle, 0deg), ${colors.primary}, ${colors.secondary}, ${colors.primary})`,
      opacity: 0.8,
      filter: 'blur(2px)',
      zIndex: -1,
    };
  }, [selected, colors.primary, colors.secondary]);

  // Backdrop blur layer for hover depth
  const backdropStyle = useMemo<React.CSSProperties>(() => ({
    position: 'absolute' as const,
    inset: -8,
    borderRadius: 20,
    background: isHovered && !isDimmed
      ? `radial-gradient(ellipse at center, ${colors.primary}08 0%, transparent 70%)`
      : 'transparent',
    backdropFilter: isHovered && !isDimmed ? 'blur(8px)' : 'none',
    opacity: isHovered ? 1 : 0,
    transition: `all 300ms ${SMOOTH_EASING}`,
    pointerEvents: 'none' as const,
    zIndex: -2,
  }), [isHovered, isDimmed, colors.primary]);

  return (
    <div
      className={cn(containerClassName, 'turbo-node-wrapper')}
      style={wrapperStyle}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
    >
      {/* Backdrop blur layer for depth on hover */}
      <div style={backdropStyle} aria-hidden="true" />

      {/* Animated selection ring */}
      {selected && (
        <div
          className="animate-ring-rotate"
          style={selectionRingStyle}
          aria-hidden="true"
        />
      )}

      {/* Gradient border wrapper */}
      <div
        className={cn(
          'relative p-0.5 rounded-[14px]',
          'transition-all duration-300',
          isHovered && !selected && 'animate-glow-pulse'
        )}
        style={{
          ...gradientBorderStyle,
          transition: `all 250ms ${SPRING_EASING}`,
        }}
      >
        {/* Inner card */}
        <div
          className={cn(
            'relative px-4 py-3 rounded-[12px]',
            'min-w-[200px] max-w-[280px]',
            'transition-all duration-200'
          )}
          style={{
            ...getInnerCardStyle(!!selected),
            // Subtle inner glow on hover
            boxShadow: isHovered && !isDimmed
              ? `inset 0 1px 0 rgba(255,255,255,0.08), inset 0 0 20px ${colors.primary}08`
              : 'inset 0 1px 0 rgba(255,255,255,0.04)',
          }}
        >
          {/* Target Handle - SOLID (incoming) - Enhanced positioning */}
          <Handle
            type="target"
            position={Position.Top}
            className={cn(
              '!rounded-full !border-0',
              '!-top-2',
              'transition-all duration-200'
            )}
            style={getPremiumHandleStyle(colors.primary, !!selected, isHovered, true)}
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

          {/* Source Handle - HOLLOW (outgoing) - Enhanced positioning */}
          <Handle
            type="source"
            position={Position.Bottom}
            className={cn(
              '!rounded-full',
              '!-bottom-2',
              'transition-all duration-200'
            )}
            style={getPremiumHandleStyle(colors.primary, !!selected, isHovered, false)}
          />
        </div>
      </div>
    </div>
  );
});
