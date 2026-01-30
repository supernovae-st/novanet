'use client';

/**
 * GlowingBorder - Nika-style glowing animated border
 *
 * Features:
 * - Multi-layer glow effects
 * - Animated rotating beam
 * - Category-based color theming
 * - Selection, hover, and active states
 * - Breathing pulse animation for selected nodes
 * - State variants: error, running, pending, success, partial (Nika-inspired)
 *
 * Uses NovaNet Icon Design System for consistent colors.
 */

import { useMemo, memo } from 'react';
import { cn } from '@/lib/utils';
import { ICON_COLORS } from '@/config/iconSystem';

// Convert hex to rgba for proper opacity support
function hexToRgba(hex: string, alpha: number): string {
  // Handle shorthand hex
  const fullHex = hex.length === 4
    ? `#${hex[1]}${hex[1]}${hex[2]}${hex[2]}${hex[3]}${hex[3]}`
    : hex;
  const r = parseInt(fullHex.slice(1, 3), 16);
  const g = parseInt(fullHex.slice(3, 5), 16);
  const b = parseInt(fullHex.slice(5, 7), 16);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

// State-specific colors (Nika-inspired) - using design system
export type GlowingBorderState = 'idle' | 'running' | 'pending' | 'error' | 'success' | 'partial';

const STATE_COLORS: Record<GlowingBorderState, string> = {
  idle: '',                           // Use provided color
  running: ICON_COLORS.system.primary,    // novanet blue
  pending: ICON_COLORS.ai.light,          // amber
  error: ICON_COLORS.error.primary,       // red
  success: ICON_COLORS.node.primary,      // emerald
  partial: '#f97316',                     // orange-500 (not in design system yet)
};

export interface GlowingBorderProps {
  children: React.ReactNode;
  className?: string;
  style?: React.CSSProperties;
  /** Primary glow color (hex) */
  color: string;
  /** Secondary glow color (hex) - defaults to primary */
  colorSecondary?: string;
  /** Whether this element is selected */
  isSelected?: boolean;
  /** Whether this element is hovered */
  isHovered?: boolean;
  /** Whether to show animated beam */
  animated?: boolean;
  /** Border radius in pixels */
  borderRadius?: number;
  /** State for contextual styling (Nika-inspired) */
  state?: GlowingBorderState;
}

export const GlowingBorder = memo(function GlowingBorder({
  children,
  className,
  style,
  color,
  colorSecondary,
  isSelected = false,
  isHovered = false,
  animated = false,
  borderRadius = 12,
  state = 'idle',
}: GlowingBorderProps) {
  // Determine effective color based on state
  const stateColor = STATE_COLORS[state];
  const { primary, secondary } = useMemo(() => ({
    primary: stateColor || color,
    secondary: stateColor || colorSecondary || color,
  }), [color, colorSecondary, stateColor]);

  // State determines animation behavior
  const isActiveState = state === 'running' || state === 'pending';
  const shouldAnimate = animated || isSelected || isHovered || isActiveState;

  // State-specific animation class
  const stateAnimationClass = useMemo(() => {
    switch (state) {
      case 'running': return 'animate-pulse-fast';
      case 'pending': return 'animate-pulse-slow';
      case 'error': return 'animate-shake';
      case 'success': return 'animate-success-flash';
      default: return '';
    }
  }, [state]);

  return (
    <div
      className={cn(
        'relative',
        isSelected && !stateAnimationClass && 'animate-breathing',
        stateAnimationClass,
        className
      )}
      style={{
        transform: 'translateZ(0)', // GPU acceleration
        borderRadius: `${borderRadius}px`,
        ...style,
      }}
    >
      {/* GLOW LAYER - AGGRESSIVE visibility for dark backgrounds */}
      <div
        className={cn(
          'absolute inset-0 pointer-events-none transition-[box-shadow,opacity] duration-300',
          shouldAnimate && 'animate-glow-pulse'
        )}
        style={{
          borderRadius: `${borderRadius}px`,
          willChange: 'box-shadow',
          boxShadow: isSelected
            // Selected: INTENSE glow - clearly visible
            ? `0 0 0 2px ${primary},
               0 0 25px 5px ${hexToRgba(primary, 0.6)},
               0 0 50px 10px ${hexToRgba(primary, 0.35)},
               0 0 100px 20px ${hexToRgba(primary, 0.15)}`
            : isHovered
              // Hovered: strong glow
              ? `0 0 0 1px ${hexToRgba(primary, 0.6)},
                 0 0 20px 4px ${hexToRgba(primary, 0.4)},
                 0 0 40px 8px ${hexToRgba(primary, 0.2)}`
              // Default: visible ambient glow - NOT subtle
              : `0 0 0 1px ${hexToRgba(primary, 0.4)},
                 0 0 15px 2px ${hexToRgba(primary, 0.25)},
                 0 0 30px 5px ${hexToRgba(primary, 0.1)}`,
        }}
      />

      {/* ANIMATED BEAM - Rotating gradient border effect */}
      {shouldAnimate && (
        <div
          className="absolute pointer-events-none"
          style={{
            inset: '-2px',
            borderRadius: `${Math.max(borderRadius - 2, 4)}px`,
            willChange: 'transform',
            // Mask: ring shape - transparent inside and outside, opaque in border area
            WebkitMaskImage: 'radial-gradient(ellipse, transparent calc(100% - 8px), black calc(100% - 6px), black calc(100% - 2px), transparent 100%)',
            maskImage: 'radial-gradient(ellipse, transparent calc(100% - 8px), black calc(100% - 6px), black calc(100% - 2px), transparent 100%)',
          }}
        >
          <div
            className={cn(
              'absolute',
              isSelected ? 'animate-spin-slow' : 'animate-shine-beam'
            )}
            style={{
              inset: '-250%',
              background: `conic-gradient(
                from 0deg,
                transparent 0deg,
                ${hexToRgba(primary, 0.2)} 30deg,
                ${hexToRgba(primary, 0.5)} 50deg,
                ${primary} 70deg,
                ${secondary} 90deg,
                ${primary} 110deg,
                ${hexToRgba(primary, 0.5)} 130deg,
                ${hexToRgba(primary, 0.2)} 150deg,
                transparent 180deg
              )`,
              willChange: 'transform',
            }}
          />
        </div>
      )}

      {/* SOLID BORDER - VISIBLE edge with category color */}
      <div
        className="absolute inset-0 pointer-events-none transition-[outline,box-shadow] duration-300 z-10"
        style={{
          borderRadius: `${borderRadius}px`,
          outline: shouldAnimate
            ? `3px solid ${primary}`
            : `2.5px solid ${hexToRgba(primary, 0.85)}`,
          outlineOffset: '-1px',
          boxShadow: `inset 0 1px 0 ${hexToRgba(primary, shouldAnimate ? 0.25 : 0.15)}`,
        }}
      />

      {/* INNER GLOW - Glass effect highlight */}
      <div
        className="absolute inset-0 pointer-events-none transition-[box-shadow] duration-300"
        style={{
          borderRadius: `${borderRadius}px`,
          boxShadow: shouldAnimate
            ? `inset 0 1px 1px rgba(255,255,255,0.25), inset 0 -1px 1px rgba(0,0,0,0.2)`
            : `inset 0 1px 1px rgba(255,255,255,0.1), inset 0 -1px 1px rgba(0,0,0,0.1)`,
        }}
      />

      {/* CONTENT */}
      <div
        className="relative overflow-hidden"
        style={{
          borderRadius: `${borderRadius}px`,
        }}
      >
        {children}
      </div>
    </div>
  );
});
