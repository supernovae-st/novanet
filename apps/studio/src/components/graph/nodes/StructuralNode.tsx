'use client';

/**
 * StructuralNode - Gradient Edge design for structural nodes
 *
 * Categories: project, content, locale, generation
 * Features:
 * - 2px gradient border (category colors)
 * - Solid/hollow handles for direction indication
 * - Animated border on selection
 * - Enhanced hover effects (scale, glow)
 * - Press feedback (scale down on mousedown)
 * - Hover info displayed in centralized bottom pill (via uiStore.hoveredNodeId)
 */

import { memo, useState, useCallback, useMemo } from 'react';
import { Handle, Position, type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { getStructuralColors } from '@/config/categoryColors';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import type { BaseNodeData } from './BaseNodeWrapper';
import { NODE_BG } from '@/config/constants';

export type StructuralNodeType = Node<BaseNodeData>;

/**
 * Get card width based on node type
 */
function getCardWidth(type: string): number {
  switch (type) {
    case 'Page': return 210;
    case 'Concept': return 195;
    case 'Block': return 175;
    case 'BlockType': return 165;
    case 'Locale': return 200;
    default: return 180;
  }
}

/**
 * StructuralNode - Gradient Edge Design
 */
export const StructuralNode = memo(function StructuralNode(props: NodeProps<StructuralNodeType>) {
  const { data, selected } = props;
  const [isHovered, setIsHovered] = useState(false);
  const [isPressed, setIsPressed] = useState(false);
  const config = NODE_TYPE_CONFIG[data.type] || NODE_TYPE_CONFIG.Project;
  const colors = getStructuralColors(data.type);
  const width = getCardWidth(data.type);
  const isDimmed = data.dimmed === true;
  const isHoverDimmed = data.hoverDimmed === true;

  const handleMouseEnter = useCallback(() => {
    setIsHovered(true);
  }, []);

  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);
    setIsPressed(false);
  }, []);

  // Memoize gradient border style to prevent re-renders
  const gradientBorderStyle = useMemo(() => ({
    background: selected
      ? `linear-gradient(135deg, ${colors.primary}, ${colors.secondary}, ${colors.primary})`
      : isHovered
        ? `linear-gradient(135deg, ${colors.primary}, ${colors.secondary})`
        : `linear-gradient(135deg, ${colors.primary}, ${colors.secondary}90)`,
    boxShadow: selected
      ? `0 0 40px 8px ${colors.primary}70, 0 0 80px 16px ${colors.primary}40, 0 0 120px 24px ${colors.primary}20`
      : isHovered
        ? `0 0 30px 6px ${colors.primary}50, 0 0 60px 12px ${colors.primary}25`
        : `0 0 20px 4px ${colors.primary}40, 0 0 40px 8px ${colors.primary}20`,
  }), [colors.primary, colors.secondary, selected, isHovered]);

  // Memoize icon style to prevent re-renders
  const iconStyle = useMemo(() => ({
    color: colors.primary,
    filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}80)`,
  }), [colors.primary, selected]);

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
      style={{
        transition: 'transform 200ms ease-out, opacity 200ms ease-out',
      }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={() => setIsPressed(true)}
      onMouseUp={() => setIsPressed(false)}
    >
      {/* Gradient border wrapper - 2px */}
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
          className="relative rounded-[12px] overflow-hidden transition-colors duration-300"
          style={{
            width,
            backgroundColor: selected ? NODE_BG.selected : NODE_BG.default,
          }}
        >
          {/* Target Handle - SOLID (incoming) */}
          <Handle
            type="target"
            position={Position.Top}
            className="!w-3 !h-3 !rounded-full !border-2 !-top-1.5 transition-all duration-200"
            style={{
              backgroundColor: colors.primary,
              borderColor: colors.primary,
              boxShadow: selected ? `0 0 8px ${colors.primary}` : undefined,
            }}
          />

          {/* Content */}
          <div className="relative px-4 py-3">
            {/* Header: Icon + Badge */}
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <CategoryIcon
                  category={config.category}
                  size={20}
                  strokeWidth={2}
                  className={cn(
                    'transition-transform duration-200',
                    (selected || isHovered) && 'scale-110'
                  )}
                  style={iconStyle}
                />
                <span
                  className="text-[10px] font-bold uppercase tracking-wider"
                  style={{ color: colors.primary }}
                >
                  {config.label}
                </span>
              </div>

              {/* Status dot */}
              <div
                className={cn('w-2.5 h-2.5 rounded-full', selected && 'animate-pulse')}
                style={{
                  background: colors.primary,
                  boxShadow: `0 0 8px ${colors.primary}`,
                }}
              />
            </div>

            {/* Display Name */}
            <h3
              className="text-base font-bold text-white truncate"
            >
              {data.displayName}
            </h3>

            {/* Key */}
            {data.key !== data.displayName && (
              <p
                className="text-[11px] font-mono truncate mt-0.5"
                style={{ color: `${colors.primary}70` }}
              >
                {data.key}
              </p>
            )}

            {/* Category badge */}
            <div
              className="mt-2.5 inline-flex items-center gap-1.5 px-2 py-1 rounded-full text-[9px] font-semibold uppercase tracking-wider border"
              style={{
                background: `${colors.primary}15`,
                borderColor: `${colors.primary}35`,
                color: colors.primary,
              }}
            >
              <span
                className={cn('w-1.5 h-1.5 rounded-full', selected && 'animate-pulse')}
                style={{
                  background: colors.primary,
                  boxShadow: `0 0 6px ${colors.primary}`,
                }}
              />
              {config.category}
            </div>
          </div>

          {/* Source Handle - HOLLOW (outgoing) */}
          <Handle
            type="source"
            position={Position.Bottom}
            className="!w-3 !h-3 !rounded-full !border-2 !-bottom-1.5 transition-all duration-200"
            style={{
              backgroundColor: 'transparent',
              borderColor: colors.primary,
              boxShadow: selected ? `0 0 8px ${colors.primary}` : undefined,
            }}
          />
        </div>
      </div>

    </div>
  );
});
