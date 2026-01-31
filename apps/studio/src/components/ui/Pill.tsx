'use client';

/**
 * Pill - Unified floating container for stats, controls, and action groups
 *
 * Solid dark design (no glass/blur):
 * - Opaque dark background for maximum contrast
 * - Inner ring highlight for depth perception
 * - Deep shadow for floating effect
 * - Hover: border brightens
 * - Glow variants for active states
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';

interface PillProps {
  children: React.ReactNode;
  className?: string;
  /** Size variant affecting padding and height */
  size?: 'sm' | 'md' | 'lg';
  /** @deprecated Kept for API compat. */
  glass?: 'light' | 'medium' | 'heavy';
  /** Absolute positioning preset */
  position?: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | 'bottom-center' | null;
  /** Glow effect (e.g., when executing) */
  glow?: boolean;
  /** Custom glow color */
  glowColor?: 'emerald' | 'novanet' | 'red';
}

export const Pill = memo(function Pill({
  children,
  className,
  size = 'md',
  position = null,
  glow = false,
  glowColor = 'emerald',
}: PillProps) {
  const sizeStyles = {
    sm: cn('px-3 py-2', gapTokens.default),
    md: cn('px-4 h-14', gapTokens.default),
    lg: cn('px-5 h-16', gapTokens.large),
  };

  const positionStyles = position
    ? {
        'top-left': 'absolute top-4 left-4 z-30',
        'top-right': 'absolute top-4 right-4 z-30',
        'bottom-left': 'absolute bottom-4 left-4 z-20',
        'bottom-right': 'absolute bottom-4 right-4 z-20',
        'bottom-center': 'absolute bottom-4 left-1/2 -translate-x-1/2 z-20',
      }[position]
    : '';

  const glowStyles = glow
    ? {
        emerald: 'border-emerald-500/40 shadow-[0_0_20px_rgba(16,185,129,0.15)] ring-1 ring-emerald-500/20',
        novanet: 'border-novanet-500/40 shadow-[0_0_20px_rgba(139,92,246,0.15)] ring-1 ring-novanet-500/20',
        red: 'border-red-500/40 shadow-[0_0_20px_rgba(239,68,68,0.15)] ring-1 ring-red-500/20',
      }[glowColor]
    : '';

  return (
    <div
      className={cn(
        // Layout
        'flex items-center rounded-2xl',
        // Solid dark - opaque, no blur
        'bg-[#0a0a0f]',
        'border border-white/[0.10]',
        // Deep shadow for float
        'shadow-2xl shadow-black/60',
        // Inner highlight for depth
        'ring-1 ring-white/[0.03] ring-inset',
        // Hover: border brightens
        'hover:border-white/[0.18]',
        // Transitions
        'transition duration-300 ease-out',
        // Size
        sizeStyles[size],
        // Position
        positionStyles,
        // Glow override
        glowStyles,
        className
      )}
    >
      {children}
    </div>
  );
});
