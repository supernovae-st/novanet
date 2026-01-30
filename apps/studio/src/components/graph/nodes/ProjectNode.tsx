'use client';

/**
 * ProjectNode - Premium card for Project nodes
 *
 * Features:
 * - 2px gradient border (violet theme from design system)
 * - Solid/hollow handles for direction indication
 * - Logo + bold typography
 * - Animated border on selection
 * - Enhanced hover effects (scale, glow)
 * - Press feedback (scale down on mousedown)
 * - Hover info displayed in centralized bottom pill (via uiStore.hoveredNodeId)
 *
 * Uses NovaNet Icon Design System for consistent colors.
 */

import { memo, useState, useCallback } from 'react';
import { Handle, Position, type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import Image from 'next/image';
import type { BaseNodeData } from './BaseNodeWrapper';
import { ICON_COLORS } from '@/config/iconSystem';
import { NODE_BG } from '@/config/constants';

// NovaNet logo URL
const NOVANET_LOGO_URL = 'https://pbs.twimg.com/profile_images/1788187862883598336/q8u1VSz3_400x400.jpg';

// Premium violet theme - using design system (relationship = violet)
const PRIMARY = ICON_COLORS.relationship.primary;
const SECONDARY = ICON_COLORS.relationship.light;

export type ProjectNodeType = Node<BaseNodeData>;

/**
 * ProjectNode - Premium Gradient Edge Design
 */
export const ProjectNode = memo(function ProjectNode(props: NodeProps<ProjectNodeType>) {
  const { data, selected } = props;
  const [isHovered, setIsHovered] = useState(false);
  const [isPressed, setIsPressed] = useState(false);
  const isDimmed = data.dimmed === true;
  const isHoverDimmed = data.hoverDimmed === true;

  const handleMouseEnter = useCallback(() => {
    setIsHovered(true);
  }, []);

  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);
    setIsPressed(false);
  }, []);

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
          'relative p-[2px] rounded-2xl transition-all duration-300',
          selected && 'animate-gradient-rotate',
          isHovered && !selected && 'animate-glow-pulse'
        )}
        style={{
          background: selected
            ? `linear-gradient(135deg, ${PRIMARY}, ${SECONDARY}, ${PRIMARY})`
            : isHovered
              ? `linear-gradient(135deg, ${PRIMARY}, ${SECONDARY})`
              : `linear-gradient(135deg, ${PRIMARY}, ${SECONDARY}90)`,
          boxShadow: selected
            ? `0 0 45px 10px ${PRIMARY}70, 0 0 90px 18px ${PRIMARY}40, 0 0 130px 26px ${PRIMARY}20`
            : isHovered
              ? `0 0 35px 7px ${PRIMARY}55, 0 0 70px 14px ${PRIMARY}30`
              : `0 0 25px 5px ${PRIMARY}45, 0 0 50px 10px ${PRIMARY}22`,
        }}
      >
        {/* Inner card */}
        <div
          className="relative w-[280px] rounded-[14px] overflow-hidden"
          style={{ backgroundColor: selected ? NODE_BG.selected : NODE_BG.default }}
        >
          {/* Grid background pattern */}
          <div
            className="absolute inset-0 opacity-[0.03]"
            style={{
              backgroundImage: `
                linear-gradient(rgba(139, 92, 246, 0.5) 1px, transparent 1px),
                linear-gradient(90deg, rgba(139, 92, 246, 0.5) 1px, transparent 1px)
              `,
              backgroundSize: '20px 20px',
            }}
          />

          {/* Target Handle - SOLID (incoming) */}
          <Handle
            type="target"
            position={Position.Top}
            className="!w-3 !h-3 !rounded-full !border-2 !-top-1.5 transition-all duration-200"
            style={{
              backgroundColor: PRIMARY,
              borderColor: PRIMARY,
              boxShadow: selected ? `0 0 8px ${PRIMARY}` : undefined,
            }}
          />

          {/* Content */}
          <div className="relative p-5">
            {/* Row 1: Logo + Badge */}
            <div className="flex items-center justify-between mb-4">
              {/* Logo */}
              <div
                className={cn(
                  'w-14 h-14 rounded-xl overflow-hidden ring-2 transition-all duration-200',
                  selected ? 'ring-white/40' : 'ring-white/20'
                )}
                style={{
                  boxShadow: selected ? `0 0 15px ${PRIMARY}40` : undefined,
                }}
              >
                <Image
                  src={NOVANET_LOGO_URL}
                  alt="Project"
                  width={56}
                  height={56}
                  className="object-cover w-full h-full"
                  unoptimized
                />
              </div>

              {/* Type Badge */}
              <div
                className="flex items-center gap-2 px-3 py-1.5 rounded-full border"
                style={{
                  background: `${PRIMARY}15`,
                  borderColor: `${PRIMARY}40`,
                }}
              >
                <span
                  className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
                  style={{
                    background: PRIMARY,
                    boxShadow: `0 0 8px ${PRIMARY}`,
                  }}
                />
                <span
                  className="text-xs font-bold uppercase tracking-wider"
                  style={{ color: SECONDARY }}
                >
                  Project
                </span>
              </div>
            </div>

            {/* Row 2: Name + Key */}
            <h2
              className="text-xl font-extrabold text-white truncate leading-tight"
            >
              {data.displayName}
            </h2>

            {data.key && data.key !== data.displayName && (
              <p
                className="font-mono text-sm mt-1 truncate"
                style={{ color: `${PRIMARY}70` }}
              >
                {data.key}
              </p>
            )}
          </div>

          {/* Source Handle - HOLLOW (outgoing) */}
          <Handle
            type="source"
            position={Position.Bottom}
            className="!w-3 !h-3 !rounded-full !border-2 !-bottom-1.5 transition-all duration-200"
            style={{
              backgroundColor: 'transparent',
              borderColor: PRIMARY,
              boxShadow: selected ? `0 0 8px ${PRIMARY}` : undefined,
            }}
          />
        </div>
      </div>

    </div>
  );
});
