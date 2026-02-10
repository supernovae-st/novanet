'use client';

/**
 * RealmAttractorNode - Unified card design for Realm attractor nodes
 *
 * v11.6 Design - matches MetaBadgeNode/SchemaNode:
 * - Card (280px width, 140px min height)
 * - Large icon top-left with glow
 * - Stacked badges on right
 * - Glow pulse + gradient borders
 * - Shows realm info + typeCount / loadedCount
 *
 * Used in magnetic layout mode as gravitational center for child nodes.
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { useNodeInteractions } from '@/hooks';
import { RealmIcon } from '@/components/ui/CategoryIcon';
import { BlueprintOverlay } from './BlueprintOverlay';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import type { Realm } from '@novanet/core/types';
import { REALM_COLORS } from '@/design/colors/generated';

// =============================================================================
// Types
// =============================================================================

export interface RealmAttractorData extends Record<string, unknown> {
  key: string;
  label: string;
  emoji: string;
  color: string;
  typeCount: number;
  loadedCount: number;
}

export type RealmAttractorNodeType = Node<RealmAttractorData, 'realmAttractor'>;

// =============================================================================
// GlowBadge - Badge with glow + gradient effect (matches SchemaNode)
// =============================================================================

const GlowBadge = memo(function GlowBadge({
  label,
  icon,
  color,
}: {
  label: string;
  icon?: React.ReactNode;
  color: string;
}) {
  return (
    <span
      className="flex items-center gap-1.5 text-[11px] font-bold uppercase tracking-wide px-2.5 py-1.5 rounded-md"
      style={{
        background: `linear-gradient(135deg, ${color}40, ${color}20)`,
        color: color,
        border: `1px solid ${color}60`,
        boxShadow: `0 0 12px ${color}35, inset 0 1px 0 ${color}25`,
      }}
    >
      {icon}
      {label}
    </span>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const RealmAttractorNode = memo(function RealmAttractorNode({
  data,
  selected = false,
}: NodeProps<RealmAttractorNodeType>) {
  const { key, label, typeCount = 0, loadedCount = 0 } = data;

  // Get design system color from generated taxonomy (fallback to data.color for safety)
  const realmKey = key as Realm;
  const primaryColor = REALM_COLORS[realmKey]?.color || data.color || '#2aa198';

  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
    containerClassName,
    containerStyle,
  } = useNodeInteractions({ selected });

  const gradientBorderStyle = useMemo(() => ({
    background: selected
      ? NODE_DESIGN.gradients.borderSelected(primaryColor, primaryColor)
      : isHovered
        ? NODE_DESIGN.gradients.borderHover(primaryColor, primaryColor)
        : NODE_DESIGN.gradients.borderDefault(primaryColor, primaryColor),
    boxShadow: selected
      ? NODE_DESIGN.shadows.glowSelected(primaryColor)
      : isHovered
        ? NODE_DESIGN.shadows.glowHover(primaryColor)
        : NODE_DESIGN.shadows.glow(primaryColor),
  }), [primaryColor, selected, isHovered]);

  const traitBorderStyle = useMemo(() => ({
    borderStyle: 'solid' as const,
    borderWidth: '2px',
    borderColor: `${primaryColor}60`,
  }), [primaryColor]);

  return (
    <div
      className={containerClassName}
      style={{ ...containerStyle, width: 280 }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      aria-label={`${label} realm: ${typeCount} types, ${loadedCount} loaded`}
    >
      {/* Hidden handles for edges */}
      <Handle type="target" position={Position.Top} className="!opacity-0 !w-1 !h-1" />
      <Handle type="source" position={Position.Bottom} className="!opacity-0 !w-1 !h-1" />

      {/* Gradient border wrapper */}
      <div
        className={cn(
          'relative transition-all duration-300',
          selected && 'animate-gradient-rotate',
          isHovered && !selected && 'animate-glow-pulse'
        )}
        style={{
          borderRadius: NODE_DESIGN.radius.outer,
          padding: selected ? NODE_DESIGN.border.selected : NODE_DESIGN.border.default,
          ...gradientBorderStyle,
        }}
      >
        {/* Inner card - premium sizing with breathing room */}
        <div
          className={cn(
            'relative overflow-hidden transition-all duration-300',
            isHovered && !selected && 'animate-shimmer-sweep'
          )}
          style={{
            minHeight: 140,
            borderRadius: selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner,
            backgroundColor: selected ? NODE_DESIGN.selectedBg : NODE_BG.default,
            border: selected ? `${NODE_DESIGN.border.innerSelected}px solid ${primaryColor}` : undefined,
            borderStyle: selected ? undefined : traitBorderStyle.borderStyle,
            borderWidth: selected ? undefined : traitBorderStyle.borderWidth,
            borderColor: selected ? undefined : traitBorderStyle.borderColor,
            boxShadow: selected ? NODE_DESIGN.shadows.skeuomorphic(primaryColor) : undefined,
            // CSS variable for animation color
            '--pulse-color': `${primaryColor}60`,
            '--glow-color': primaryColor,
            '--scan-color': `${primaryColor}80`,
          } as React.CSSProperties}
        >
          <BlueprintOverlay
            color={primaryColor}
            selected={selected}
            borderRadius={selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner}
            showBadge={false}
          />

          {/* Content - more breathing room */}
          <div className="relative px-6 py-5">
            {/* Top row: Large icon left, Badges right */}
            <div className="flex justify-between items-start mb-4">
              {/* Large icon with premium gradient glow + animation */}
              <div
                className={cn(
                  'flex items-center justify-center w-14 h-14 rounded-xl transition-all duration-300',
                  isHovered && 'animate-icon-glow'
                )}
                style={{
                  background: `
                    radial-gradient(ellipse at 30% 20%, ${primaryColor}50 0%, transparent 50%),
                    radial-gradient(ellipse at 70% 80%, ${primaryColor}30 0%, transparent 50%),
                    linear-gradient(135deg, ${primaryColor}35, ${primaryColor}15, ${primaryColor}25)
                  `,
                  border: `1.5px solid ${primaryColor}50`,
                  boxShadow: isHovered
                    ? `0 0 30px ${primaryColor}50, 0 0 50px ${primaryColor}25, inset 0 0 20px ${primaryColor}20`
                    : `0 0 25px ${primaryColor}35, inset 0 0 15px ${primaryColor}15`,
                  '--glow-color': primaryColor,
                } as React.CSSProperties}
              >
                <RealmIcon
                  realm={realmKey}
                  size={28}
                  strokeWidth={1.5}
                  style={{ color: primaryColor }}
                />
              </div>

              {/* Stacked badges on right */}
              <div className="flex flex-col gap-2 items-end">
                <GlowBadge
                  label="REALM"
                  icon={<RealmIcon realm={realmKey} size={12} strokeWidth={2} style={{ color: primaryColor }} />}
                  color={primaryColor}
                />
                <GlowBadge
                  label={realmKey.toUpperCase()}
                  color={primaryColor}
                />
              </div>
            </div>

            {/* Title - larger */}
            <h3 className="text-lg font-bold text-white truncate mb-1">
              {label}
            </h3>

            {/* Subtitle - dual count with separator */}
            <p
              className="text-sm font-semibold truncate"
              style={{ color: primaryColor }}
            >
              {typeCount} types &middot; {loadedCount} loaded
            </p>
          </div>
        </div>
      </div>
    </div>
  );
});
