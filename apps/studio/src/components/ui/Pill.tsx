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

import { memo, useEffect, useState } from 'react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';

// Matrix characters for shimmer overlay
const MATRIX_CHARS = 'アイウエオカキクケコサシスセソタチツテト0123456789';

/** Matrix rain columns for glow state - intensified */
const MatrixShimmer = memo(function MatrixShimmer({ color }: { color: 'emerald' | 'novanet' | 'red' }) {
  const [columns, setColumns] = useState<string[]>([]);

  useEffect(() => {
    const interval = setInterval(() => {
      setColumns(
        Array.from({ length: 16 }, () =>
          Array.from({ length: 3 }, () =>
            MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]
          ).join('')
        )
      );
    }, 70);
    return () => clearInterval(interval);
  }, []);

  const textColor = {
    emerald: 'text-emerald-400',
    novanet: 'text-novanet-400',
    red: 'text-red-400',
  }[color];

  const glowShadow = {
    emerald: '0 0 6px rgba(52,211,153,0.5)',
    novanet: '0 0 6px rgba(139,92,246,0.5)',
    red: '0 0 6px rgba(239,68,68,0.5)',
  }[color];

  return (
    <div className="absolute inset-0 overflow-hidden pointer-events-none opacity-30 rounded-2xl" aria-hidden="true">
      <div className="flex justify-around h-full">
        {columns.map((col, i) => (
          <span
            key={i}
            className={cn('font-mono text-[8px] animate-pulse', textColor)}
            style={{
              animationDelay: `${i * 60}ms`,
              writingMode: 'vertical-rl',
              textOrientation: 'upright',
              textShadow: glowShadow,
            }}
          >
            {col}
          </span>
        ))}
      </div>
    </div>
  );
});

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
    md: cn('px-4 min-h-14', gapTokens.default),
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
        emerald: 'border-emerald-400/50 shadow-[0_0_40px_rgba(16,185,129,0.25),0_0_80px_rgba(16,185,129,0.1)] ring-1 ring-emerald-400/30',
        novanet: 'border-novanet-400/50 shadow-[0_0_40px_rgba(139,92,246,0.25),0_0_80px_rgba(139,92,246,0.1)] ring-1 ring-novanet-400/30',
        red: 'border-red-400/50 shadow-[0_0_40px_rgba(239,68,68,0.25),0_0_80px_rgba(239,68,68,0.1)] ring-1 ring-red-400/30',
      }[glowColor]
    : '';

  const shimmerGradient = glow
    ? {
        emerald: 'via-emerald-400/15',
        novanet: 'via-novanet-400/15',
        red: 'via-red-400/15',
      }[glowColor]
    : '';

  return (
    <div
      className={cn(
        // Layout
        'relative flex items-center rounded-2xl',
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
      {/* Matrix effects when glowing */}
      {glow && (
        <>
          <MatrixShimmer color={glowColor} />
          <div className="absolute inset-0 rounded-2xl overflow-hidden pointer-events-none" aria-hidden="true">
            <div className={cn(
              'absolute inset-0 bg-gradient-to-r from-transparent to-transparent animate-[shimmer_1.5s_infinite]',
              shimmerGradient
            )} />
          </div>
        </>
      )}
      {children}
    </div>
  );
});
