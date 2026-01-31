'use client';

/**
 * Pill - Unified floating container for stats, controls, and action groups
 *
 * Features:
 * - Glassmorphism with hover effects
 * - Subtle shadow and border animations
 * - Glow effect for active states
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';
import { glassClasses, gapTokens, opacity } from '@/design/tokens';

interface PillProps {
  children: React.ReactNode;
  className?: string;
  /** Size variant affecting padding and height */
  size?: 'sm' | 'md' | 'lg';
  /** Glass intensity for backdrop effect */
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
  glass = 'medium',
  position = null,
  glow = false,
  glowColor = 'emerald',
}: PillProps) {
  // Glass effect variants mapped to design tokens
  const glassStyles = {
    light: cn(glassClasses.subtle, `hover:bg-[hsl(240,8%,5%)] hover:border-${opacity.border.medium}`),
    medium: cn(glassClasses.light, `hover:bg-[hsl(240,6%,8%)] hover:border-${opacity.border.strong}`),
    heavy: cn(glassClasses.medium, `hover:bg-[hsl(240,5%,12%)] hover:border-${opacity.border.heavy}`),
  };

  const sizeStyles = {
    sm: cn('px-3 py-2', gapTokens.default),
    md: cn('px-4 h-14', gapTokens.default),
    lg: cn('px-5 h-16', gapTokens.large),
  };

  // Position styles for absolute positioning (bottom controls only)
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
        // Base styles
        'flex items-center rounded-2xl border',
        // Shadow and depth
        'shadow-xl shadow-black/40',
        // Smooth transitions
        'transition-all duration-300 ease-out',
        // Glass effect
        glassStyles[glass],
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
