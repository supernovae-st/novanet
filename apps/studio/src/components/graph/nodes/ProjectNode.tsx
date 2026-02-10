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
 * Uses shared design system components from effects/ directory.
 */

import { memo, useMemo, useState } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import Image from 'next/image';
import { Briefcase } from 'lucide-react';
import type { BaseNodeData } from './BaseNodeWrapper';
import { BlueprintOverlay } from './BlueprintOverlay';
import { getStructuralColors } from '@/design/nodeColors';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import { useNodeInteractions } from '@/hooks';
import { SelectionPulseRing, GlassmorphismEffects, NodeHandles } from './effects';
import { glassClasses, gapTokens } from '@/design/tokens';

// NovaNet logo URL
const NOVANET_LOGO_URL = 'https://pbs.twimg.com/profile_images/1788187862883598336/q8u1VSz3_400x400.jpg';

// Premium violet theme - using unified design system (v11.6)
const projectColors = getStructuralColors('Project');
const PRIMARY = projectColors.primary;
const SECONDARY = projectColors.secondary;

export type ProjectNodeType = Node<BaseNodeData>;

/**
 * ProjectNode - Premium Gradient Edge Design
 */
export const ProjectNode = memo(function ProjectNode(props: NodeProps<ProjectNodeType>) {
  const { data, selected = false } = props;
  const isDimmed = data.dimmed === true;
  const isHoverDimmed = data.hoverDimmed === true;
  const isMetaMode = data.isMetaMode === true;
  const [imageError, setImageError] = useState(false);

  // Shared interaction state management
  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
    containerClassName,
    containerStyle,
  } = useNodeInteractions({ selected, isDimmed, isHoverDimmed });

  // Memoize gradient border style (ProjectNode uses slightly stronger glow)
  const gradientBorderStyle = useMemo(() => ({
    background: selected
      ? NODE_DESIGN.gradients.borderSelected(PRIMARY, SECONDARY)
      : isHovered
        ? NODE_DESIGN.gradients.borderHover(PRIMARY, SECONDARY)
        : NODE_DESIGN.gradients.borderDefault(PRIMARY, SECONDARY),
    boxShadow: selected
      ? `0 0 45px 10px ${PRIMARY}70, 0 0 90px 18px ${PRIMARY}40, 0 0 130px 26px ${PRIMARY}20`
      : isHovered
        ? `0 0 35px 7px ${PRIMARY}55, 0 0 70px 14px ${PRIMARY}30`
        : `0 0 25px 5px ${PRIMARY}45, 0 0 50px 10px ${PRIMARY}22`,
  }), [selected, isHovered]);

  return (
    <div
      className={containerClassName}
      style={{
        ...containerStyle,
        ...(isMetaMode && !selected && { opacity: 0.6, filter: 'saturate(0.7)' }),
      }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
    >
      {/* Selection pulse ring effect - WOW animation */}
      {selected && (
        <SelectionPulseRing color={PRIMARY} borderRadius={16} />
      )}

      {/* Gradient border wrapper - 2px (3px when selected) */}
      <div
        className={cn(
          'relative rounded-2xl transition-all duration-300',
          selected && 'animate-gradient-rotate',
          isHovered && !selected && 'animate-glow-pulse'
        )}
        style={{
          padding: selected ? NODE_DESIGN.border.selected : NODE_DESIGN.border.default,
          ...gradientBorderStyle,
        }}
      >
        {/* Inner card - Glassmorphism + Skeuomorphism when selected */}
        <div
          className={cn(
            'relative w-[280px] overflow-hidden transition-all duration-500 ease-out',
            selected && glassClasses.medium,
            selected && 'animate-float'
          )}
          style={{
            borderRadius: selected ? NODE_DESIGN.radius.inner : NODE_DESIGN.radius.outer,
            backgroundColor: selected ? NODE_DESIGN.selectedBg : NODE_BG.default,
            border: selected ? `${NODE_DESIGN.border.innerSelected}px solid ${PRIMARY}` : 'none',
            boxShadow: selected ? NODE_DESIGN.shadows.skeuomorphic(PRIMARY) : undefined,
          }}
        >
          {/* Glassmorphism effects (bevel, reflection, shimmer) */}
          {selected && (
            <GlassmorphismEffects borderRadius={NODE_DESIGN.radius.inner} />
          )}

          {/* Blueprint overlay for meta mode */}
          {isMetaMode && (
            <BlueprintOverlay
              color={PRIMARY}
              selected={selected}
              borderRadius={selected ? NODE_DESIGN.radius.inner : NODE_DESIGN.radius.outer}
            />
          )}

          {/* Grid background pattern (Project-specific decoration) */}
          <div
            className={cn(
              'absolute inset-0 pointer-events-none',
              selected ? 'opacity-[0.05]' : 'opacity-[0.03]'
            )}
            style={{
              backgroundImage: `
                linear-gradient(rgba(139, 92, 246, 0.5) 1px, transparent 1px),
                linear-gradient(90deg, rgba(139, 92, 246, 0.5) 1px, transparent 1px)
              `,
              backgroundSize: '20px 20px',
            }}
          />

          {/* Handles - vertical layout (top/bottom) */}
          <NodeHandles color={PRIMARY} selected={selected} layout="vertical" />

          {/* Content */}
          <div className="relative p-5">
            {/* Row 1: Logo + Badge */}
            <div className="flex items-center justify-between mb-4">
              {/* Logo */}
              <div
                className={cn(
                  'w-14 h-14 rounded-xl overflow-hidden ring-2 transition duration-200',
                  selected ? 'ring-white/40' : 'ring-white/20'
                )}
                style={{
                  boxShadow: selected ? `0 0 15px ${PRIMARY}40` : undefined,
                }}
              >
                {!imageError ? (
                  <Image
                    src={NOVANET_LOGO_URL}
                    alt="Project"
                    width={56}
                    height={56}
                    className="object-cover w-full h-full"
                    unoptimized
                    onError={() => setImageError(true)}
                  />
                ) : (
                  <div
                    className="w-full h-full flex items-center justify-center"
                    style={{ background: `${PRIMARY}20` }}
                  >
                    <Briefcase size={28} style={{ color: PRIMARY }} />
                  </div>
                )}
              </div>

              {/* Type Badge */}
              <div
                className={cn('flex items-center px-3 py-1.5 rounded-full border', gapTokens.default)}
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
            <h2 className="text-xl font-extrabold text-white truncate leading-tight">
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
        </div>
      </div>
    </div>
  );
});
