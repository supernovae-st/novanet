'use client';

/**
 * TaxonomyBadge - Visual encoding component for taxonomy classification
 *
 * Displays Layer, Realm, and Class visual encoding prominently.
 * Follows ADR-005 Visual Encoding:
 * - Layer → Fill color (primary identification)
 * - Realm → Border color badge
 * - Class → Icon with glow
 *
 * Usage:
 * ```tsx
 * <TaxonomyBadge
 *   layer="knowledge"
 *   realm="shared"
 *   className="Term"
 * />
 * ```
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import {
  type NodeLayer,
  type NodeRealm,
  LAYER_COLORS,
  LAYER_DISPLAY_NAMES,
  REALM_COLORS,
  getClassIcon,
  hexToRgba,
} from './taxonomyColors';
import { DURATIONS } from './animationPresets';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface TaxonomyBadgeProps {
  /** Node layer (determines fill color) */
  layer: NodeLayer;
  /** Node realm (determines border color) */
  realm: NodeRealm;
  /** Node class name (determines icon) */
  className: string;
  /** Whether the node is selected */
  selected?: boolean;
  /** Whether the node is hovered */
  isHovered?: boolean;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
  /** Size variant */
  size?: 'sm' | 'md' | 'lg';
  /** Show layer name label */
  showLayerLabel?: boolean;
}

// =============================================================================
// Animation Variants
// =============================================================================

const iconVariants: Variants = {
  idle: { scale: 1, rotate: 0 },
  hover: {
    scale: 1.15,
    rotate: [0, -5, 5, 0],
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    scale: 1.2,
    filter: 'drop-shadow(0 0 8px currentColor)',
    transition: { duration: DURATIONS.fast },
  },
};

const badgeVariants: Variants = {
  idle: { opacity: 0.8 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Lucide Icon Component (Simplified inline SVG)
// =============================================================================

// Using inline SVGs for commonly used icons to avoid external dependency
const ICON_PATHS: Record<string, string> = {
  // Knowledge layer
  type: 'M4 7V4h16v3M9 20h6M12 4v16',
  'message-circle': 'M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z',
  search: 'M21 21l-6-6m2-5a7 7 0 1 1-14 0 7 7 0 0 1 14 0z',
  'book-open': 'M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2zm20 0h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z',
  // Semantic layer
  lightbulb: 'M9 18h6M10 22h4M15.09 14c.18-.98.65-1.74 1.41-2.5A4.65 4.65 0 0 0 18 8 6 6 0 0 0 6 8c0 1 .23 2.23 1.5 3.5A4.61 4.61 0 0 1 8.91 14',
  globe: 'M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm0 0v20M2 12h20M12 2a15 15 0 0 1 4 10 15 15 0 0 1-4 10M12 2a15 15 0 0 0-4 10 15 15 0 0 0 4 10',
  // Structure layer
  'file-text': 'M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7l-5-5zM14 2v5h5M16 13H8M16 17H8M10 9H8',
  square: 'M3 3h18v18H3z',
  // Foundation layer
  folder: 'M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z',
  palette: 'M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z',
  sparkles: 'M12 3l1.88 3.75L18 8.06l-3 2.88.71 4.06L12 13l-3.71 2L9 10.94 6 8.06l4.12-1.31L12 3z',
  // Instruction layer
  terminal: 'M4 17l6-6-6-6M12 19h8',
  component: 'M5.5 8.5L9 12l-3.5 3.5L2 12l3.5-3.5zM12 2l3.5 3.5L12 9 8.5 5.5 12 2zm6.5 6.5L22 12l-3.5 3.5L15 12l3.5-3.5zM12 15l3.5 3.5L12 22l-3.5-3.5L12 15z',
  // Output layer
  'file-check': 'M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2zM14 2v6h6M9 15l2 2 4-4',
  'check-square': 'M9 11l3 3 8-8M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11',
  package: 'M16.5 9.4l-9-5.19M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16zM3.27 6.96L12 12.01l8.73-5.05M12 22.08V12',
  // Default
  box: 'M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z',
};

function IconSVG({
  name,
  size = 20,
  color = 'currentColor',
  className,
}: {
  name: string;
  size?: number;
  color?: string;
  className?: string;
}) {
  const path = ICON_PATHS[name] || ICON_PATHS.box;

  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke={color}
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className={className}
    >
      <path d={path} />
    </svg>
  );
}

// =============================================================================
// Component
// =============================================================================

export const TaxonomyBadge = memo(function TaxonomyBadge({
  layer,
  realm,
  className: nodeClassName,
  selected = false,
  isHovered = false,
  performanceConfig,
  size = 'md',
  showLayerLabel = true,
}: TaxonomyBadgeProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Colors from taxonomy
  const layerColor = LAYER_COLORS[layer];
  const realmColor = REALM_COLORS[realm];
  const layerName = LAYER_DISPLAY_NAMES[layer];
  const iconName = getClassIcon(nodeClassName);

  // Size variants
  const sizes = {
    sm: { icon: 16, badge: 'text-[8px] px-1.5 py-0.5', gap: 'gap-1.5' },
    md: { icon: 20, badge: 'text-[9px] px-2 py-0.5', gap: 'gap-2' },
    lg: { icon: 24, badge: 'text-[10px] px-2.5 py-1', gap: 'gap-2.5' },
  };

  const currentSize = sizes[size];

  // Glow effect for icon
  const iconGlow = useMemo(
    () => ({
      filter: selected
        ? `drop-shadow(0 0 8px ${layerColor}) drop-shadow(0 0 12px ${layerColor})`
        : isHovered
          ? `drop-shadow(0 0 6px ${layerColor})`
          : `drop-shadow(0 0 3px ${layerColor})`,
    }),
    [layerColor, selected, isHovered]
  );

  const IconWrapper = animationsEnabled ? motion.div : 'div';
  const BadgeWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className={cn('flex flex-col', currentSize.gap)}>
      {/* Top row: Icon + Layer badge + Realm indicator */}
      <div className={cn('flex items-center justify-between', currentSize.gap)}>
        {/* Class Icon with Layer color */}
        <div className="flex items-center gap-2">
          <IconWrapper
            className="relative"
            style={iconGlow}
            {...(animationsEnabled && {
              variants: iconVariants,
              initial: 'idle',
              animate: animationState,
            })}
          >
            <IconSVG
              name={iconName}
              size={currentSize.icon}
              color={layerColor}
            />
          </IconWrapper>

          {/* Layer badge with fill color */}
          {showLayerLabel && (
            <BadgeWrapper
              className={cn(
                'rounded font-bold uppercase tracking-wider font-mono',
                currentSize.badge
              )}
              style={{
                backgroundColor: hexToRgba(layerColor, 0.2),
                color: layerColor,
                border: `1px solid ${hexToRgba(layerColor, 0.4)}`,
              }}
              {...(animationsEnabled && {
                variants: badgeVariants,
                initial: 'idle',
                animate: animationState,
              })}
            >
              {layerName}
            </BadgeWrapper>
          )}
        </div>

        {/* Realm indicator */}
        <div
          className={cn(
            'rounded-full font-mono uppercase tracking-wider',
            currentSize.badge
          )}
          style={{
            backgroundColor: hexToRgba(realmColor, 0.15),
            color: hexToRgba(realmColor, 0.9),
            border: `1px solid ${hexToRgba(realmColor, 0.3)}`,
          }}
        >
          {realm}
        </div>
      </div>

    </div>
  );
});

// =============================================================================
// Compact Badge Variant (for tight spaces)
// =============================================================================

export interface TaxonomyBadgeCompactProps {
  layer: NodeLayer;
  className: string;
  color?: string;
}

export const TaxonomyBadgeCompact = memo(function TaxonomyBadgeCompact({
  layer,
  className: nodeClassName,
  color,
}: TaxonomyBadgeCompactProps) {
  const layerColor = color || LAYER_COLORS[layer];
  const iconName = getClassIcon(nodeClassName);

  return (
    <div className="flex items-center gap-1.5">
      <IconSVG name={iconName} size={14} color={layerColor} />
      <span
        className="text-[9px] font-bold uppercase tracking-widest font-mono"
        style={{ color: layerColor }}
      >
        {nodeClassName}
      </span>
    </div>
  );
});
