'use client';

/**
 * ProgressBar - Animated progress bar with glow effect
 *
 * Features:
 * - Smooth width transitions
 * - Color customization
 * - Glow effect when value > 0
 * - Proportional display based on max value
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';

export interface ProgressBarProps {
  /** Current value */
  value: number;
  /** Maximum value (for percentage calculation) */
  max: number;
  /** Color for the progress bar (hex) */
  color: string;
  /** Height variant */
  size?: 'sm' | 'default' | 'lg';
  /** Whether to show glow effect */
  showGlow?: boolean;
  /** Additional class names */
  className?: string;
}

const sizeClasses = {
  sm: 'h-1',
  default: 'h-1.5',
  lg: 'h-2',
};

export const ProgressBar = memo(function ProgressBar({
  value,
  max,
  color,
  size = 'default',
  showGlow = true,
  className,
}: ProgressBarProps) {
  const percentage = max > 0 ? Math.min((value / max) * 100, 100) : 0;

  return (
    <div
      role="progressbar"
      aria-valuenow={value}
      aria-valuemin={0}
      aria-valuemax={max}
      aria-label={`${Math.round(percentage)}% complete`}
      className={cn(
        'flex-1 bg-white/[0.06] rounded-full overflow-hidden',
        sizeClasses[size],
        className
      )}
    >
      <div
        className="h-full rounded-full transition-all duration-500 ease-out"
        style={{
          width: `${percentage}%`,
          backgroundColor: color,
          boxShadow: showGlow && percentage > 0 ? `0 0 8px ${color}60` : 'none',
        }}
      />
    </div>
  );
});
