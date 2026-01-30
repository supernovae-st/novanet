'use client';

/**
 * LocaleKnowledgeNode - Circular nodes for locale knowledge types
 *
 * Category: locale (knowledge nodes within locale category)
 * Types: LocaleIdentity, LocaleVoice, LocaleCulture, LocaleMarket, LocaleLexicon, Expression
 * Features:
 * - Circular gradient ring design (2px)
 * - Solid/hollow handles for direction indication
 * - Animated ring on selection
 * - Enhanced hover effects (scale, glow)
 * - Press feedback (scale down on mousedown)
 * - Hover info displayed in centralized bottom pill (via uiStore.hoveredNodeId)
 */

import { memo, useState, useCallback, useMemo } from 'react';
import { Handle, Position, type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { getLocaleKnowledgeColors } from '@/config/categoryColors';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import type { BaseNodeData } from './BaseNodeWrapper';
import { NODE_BG } from '@/config/constants';

export type LocaleKnowledgeNodeType = Node<BaseNodeData>;

/**
 * Get size based on connection count and type
 */
function getCircleSize(type: string, connectionCount?: number): number {
  const count = connectionCount || 0;
  const baseSize: Record<string, number> = {
    LocaleIdentity: 70,
    LocaleVoice: 65,
    LocaleCulture: 60,
    LocaleMarket: 65,
    LocaleLexicon: 55,
    Expression: 50,
  };
  const base = baseSize[type] || 55;
  if (count > 10) return base + 20;
  if (count > 5) return base + 12;
  if (count > 2) return base + 6;
  return base;
}

/**
 * LocaleKnowledgeNode - Circular Gradient Ring Design
 */
export const LocaleKnowledgeNode = memo(function LocaleKnowledgeNode(props: NodeProps<LocaleKnowledgeNodeType>) {
  const { data, selected } = props;
  const [isHovered, setIsHovered] = useState(false);
  const [isPressed, setIsPressed] = useState(false);
  const config = NODE_TYPE_CONFIG[data.type] || NODE_TYPE_CONFIG.Expression;
  const colors = getLocaleKnowledgeColors(data.type);
  const size = getCircleSize(data.type, data.connectionCount);
  const isDimmed = data.dimmed === true;
  const isHoverDimmed = data.hoverDimmed === true;

  const handleMouseEnter = useCallback(() => {
    setIsHovered(true);
  }, []);

  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);
    setIsPressed(false);
  }, []);

  // Memoize gradient ring style to prevent re-renders
  const gradientRingStyle = useMemo(() => ({
    background: selected
      ? `linear-gradient(135deg, ${colors.primary}, ${colors.secondary}, ${colors.primary})`
      : isHovered
        ? `linear-gradient(135deg, ${colors.primary}, ${colors.secondary})`
        : `linear-gradient(135deg, ${colors.primary}, ${colors.secondary}90)`,
    boxShadow: selected
      ? `0 0 35px 8px ${colors.primary}70, 0 0 70px 14px ${colors.primary}40, 0 0 100px 20px ${colors.primary}20`
      : isHovered
        ? `0 0 25px 5px ${colors.primary}50, 0 0 50px 10px ${colors.primary}25`
        : `0 0 18px 4px ${colors.primary}45, 0 0 35px 7px ${colors.primary}20`,
  }), [colors.primary, colors.secondary, selected, isHovered]);

  // Memoize icon style to prevent re-renders
  const iconStyle = useMemo(() => ({
    color: colors.primary,
    filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}90)`,
  }), [colors.primary, selected]);

  return (
    <div
      className={cn(
        'group relative node-pressable',
        // Full dimming (focus mode)
        isDimmed && 'opacity-15 scale-75 grayscale pointer-events-none',
        // Lighter dimming (hover highlight mode)
        isHoverDimmed && !isDimmed && 'hover-dimmed',
        // Enhanced hover effect
        isHovered && !isDimmed && !isHoverDimmed && !selected && 'scale-103',
        // Press feedback
        isPressed && !isDimmed && 'scale-[0.96]',
        // Selection already has its own scale
        selected && 'scale-110'
      )}
      style={{
        transition: 'transform 200ms ease-out, opacity 200ms ease-out',
      }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={() => setIsPressed(true)}
      onMouseUp={() => setIsPressed(false)}
    >
      {/* Gradient ring wrapper - 2px */}
      <div
        className={cn(
          'relative p-[2px] rounded-full transition-all duration-300',
          selected && 'animate-spin-slow',
          isHovered && !selected && 'animate-glow-pulse'
        )}
        style={gradientRingStyle}
      >
        {/* Inner circle */}
        <div
          className="flex flex-col items-center justify-center rounded-full"
          style={{
            width: size,
            height: size,
            backgroundColor: selected ? NODE_BG.selected : NODE_BG.default,
          }}
        >
          {/* Target Handle - SOLID (incoming) */}
          <Handle
            type="target"
            position={Position.Top}
            className="!w-2.5 !h-2.5 !rounded-full !border-2 !-top-1 transition-all duration-200"
            style={{
              backgroundColor: colors.primary,
              borderColor: colors.primary,
              boxShadow: selected ? `0 0 6px ${colors.primary}` : undefined,
            }}
          />

          {/* Icon - SVG from Lucide */}
          <CategoryIcon
            category={config.category}
            size={20}
            strokeWidth={2}
            className={cn(
              'transition-transform duration-200',
              (isHovered || selected) && 'scale-110'
            )}
            style={iconStyle}
          />

          {/* Label */}
          <span
            className="text-[9px] font-semibold text-center truncate max-w-[85%] mt-1"
            style={{
              color: selected ? 'white' : colors.primary,
              textShadow: selected ? `0 0 6px ${colors.primary}` : 'none',
            }}
          >
            {data.displayName.length > 10
              ? data.displayName.slice(0, 8) + '...'
              : data.displayName}
          </span>

          {/* Source Handle - HOLLOW (outgoing) */}
          <Handle
            type="source"
            position={Position.Bottom}
            className="!w-2.5 !h-2.5 !rounded-full !border-2 !-bottom-1 transition-all duration-200"
            style={{
              backgroundColor: 'transparent',
              borderColor: colors.primary,
              boxShadow: selected ? `0 0 6px ${colors.primary}` : undefined,
            }}
          />
        </div>
      </div>

    </div>
  );
});
