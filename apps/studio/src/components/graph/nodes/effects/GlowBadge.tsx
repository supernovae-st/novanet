'use client';

/**
 * GlowBadge - Reusable badge with gradient glow effect
 *
 * Used across node components for consistent premium styling:
 * - Type badges (REALM, LAYER, ENTITY, etc.)
 * - Status indicators
 * - Category labels
 *
 * Features:
 * - Gradient background with configurable color
 * - Glow shadow effect
 * - Optional icon support
 * - Size variants (sm, md, lg)
 */

import { memo, type ReactNode } from 'react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';

export interface GlowBadgeProps {
  /** Badge label text */
  label: string;
  /** Primary color for gradient and glow (hex string) */
  color: string;
  /** Optional icon to display before label */
  icon?: ReactNode;
  /** Size variant */
  size?: 'sm' | 'md' | 'lg';
  /** Additional className */
  className?: string;
}

/**
 * Size configurations for badge variants
 */
const SIZE_CONFIG = {
  sm: {
    text: 'text-[9px]',
    padding: 'px-1.5 py-0.5',
    gap: gapTokens.compact,
    radius: 'rounded',
    iconSize: 8,
    glowIntensity: 8,
  },
  md: {
    text: 'text-[10px]',
    padding: 'px-2 py-1',
    gap: gapTokens.default,
    radius: 'rounded-md',
    iconSize: 10,
    glowIntensity: 10,
  },
  lg: {
    text: 'text-[11px]',
    padding: 'px-2.5 py-1.5',
    gap: gapTokens.default,
    radius: 'rounded-md',
    iconSize: 12,
    glowIntensity: 12,
  },
} as const;

/**
 * GlowBadge Component
 *
 * @example
 * ```tsx
 * <GlowBadge label="REALM" color="#2aa198" icon={<RealmIcon />} />
 * <GlowBadge label="LAYER" color="#0ea5e9" size="lg" />
 * ```
 */
export const GlowBadge = memo(function GlowBadge({
  label,
  color,
  icon,
  size = 'md',
  className,
}: GlowBadgeProps) {
  const config = SIZE_CONFIG[size];

  return (
    <span
      className={cn(
        'flex items-center font-bold uppercase tracking-wide',
        config.text,
        config.padding,
        config.radius,
        config.gap,
        className
      )}
      style={{
        background: `linear-gradient(135deg, ${color}40, ${color}20)`,
        color: color,
        border: `1px solid ${color}60`,
        boxShadow: `0 0 ${config.glowIntensity}px ${color}30, inset 0 1px 0 ${color}20`,
      }}
    >
      {icon}
      {label}
    </span>
  );
});
