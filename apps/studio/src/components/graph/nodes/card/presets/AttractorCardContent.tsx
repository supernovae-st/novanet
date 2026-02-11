'use client';

/**
 * AttractorCardContent - Content preset for attractor nodes
 *
 * Used by: RealmAttractorNode, LayerAttractorNode
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ [Large Icon]     [GlowBadge]    │
 * │   with glow      [GlowBadge]    │
 * │                                  │
 * │ Label                            │
 * │ X types · Y loaded               │
 * └──────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { RealmIcon, LayerIcon } from '@/components/ui/CategoryIcon';
import { GlowBadge } from '../../effects';
import type { CardContext } from '../CardShell';
import type { Realm, Layer } from '@novanet/core/types';

// =============================================================================
// Types
// =============================================================================

export interface AttractorNodeData {
  key: string;
  label: string;
  emoji?: string;
  typeCount?: number;
  loadedCount?: number;
  realmKey?: string; // For layer attractors
}

export interface AttractorCardContentProps extends CardContext {
  data: AttractorNodeData;
  variant: 'realm' | 'layer';
}

// =============================================================================
// Component
// =============================================================================

export const AttractorCardContent = memo(function AttractorCardContent({
  data,
  colors,
  selected,
  isHovered,
  variant,
}: AttractorCardContentProps) {
  const { key, label, typeCount = 0, loadedCount = 0 } = data;

  const badgeLabel = variant === 'realm' ? 'REALM' : 'LAYER';
  const iconContainerSize = variant === 'realm' ? 'w-14 h-14' : 'w-11 h-11';

  // Icon container style with gradient glow
  const iconContainerStyle = useMemo(() => ({
    background: `
      radial-gradient(ellipse at 30% 20%, ${colors.primary}50 0%, transparent 50%),
      radial-gradient(ellipse at 70% 80%, ${colors.primary}30 0%, transparent 50%),
      linear-gradient(135deg, ${colors.primary}35, ${colors.primary}15, ${colors.primary}25)
    `,
    border: `1.5px solid ${colors.primary}50`,
    boxShadow: isHovered
      ? `0 0 ${variant === 'realm' ? '30px' : '25px'} ${colors.primary}50, 0 0 ${variant === 'realm' ? '50px' : '40px'} ${colors.primary}25, inset 0 0 ${variant === 'realm' ? '20px' : '15px'} ${colors.primary}20`
      : `0 0 ${variant === 'realm' ? '25px' : '20px'} ${colors.primary}35, inset 0 0 ${variant === 'realm' ? '15px' : '12px'} ${colors.primary}15`,
  }), [colors.primary, isHovered, variant]);

  // Render icon based on variant to avoid TypeScript spread issues
  const renderMainIcon = () => {
    const iconStyle = { color: colors.primary };
    if (variant === 'realm') {
      return <RealmIcon realm={key as Realm} size={28} strokeWidth={1.5} style={iconStyle} />;
    }
    return <LayerIcon layer={key as Layer} size={22} strokeWidth={1.5} style={iconStyle} />;
  };

  const renderBadgeIcon = () => {
    const iconStyle = { color: colors.primary };
    if (variant === 'realm') {
      return <RealmIcon realm={key as Realm} size={12} strokeWidth={2} style={iconStyle} />;
    }
    return <LayerIcon layer={key as Layer} size={10} strokeWidth={2} style={iconStyle} />;
  };

  return (
    <div className={variant === 'realm' ? 'px-6 py-5' : 'px-4 py-4'}>
      {/* Top row: Icon left, Badges right */}
      <div className={cn('flex justify-between items-start', variant === 'realm' ? 'mb-4' : 'mb-3')}>
        {/* Large icon with gradient glow */}
        <div
          className={cn(
            'flex items-center justify-center rounded-xl transition-all duration-300',
            iconContainerSize,
            isHovered && 'animate-icon-glow'
          )}
          style={iconContainerStyle}
        >
          {renderMainIcon()}
        </div>

        {/* Stacked badges on right */}
        <div className={cn('flex flex-col items-end', variant === 'realm' ? 'gap-2' : 'gap-1.5')}>
          <GlowBadge
            label={badgeLabel}
            icon={renderBadgeIcon()}
            color={colors.primary}
            size={variant === 'realm' ? 'lg' : 'md'}
          />
          <GlowBadge
            label={key.toUpperCase().replace(/-/g, ' ')}
            color={colors.primary}
            size={variant === 'realm' ? 'lg' : 'md'}
          />
        </div>
      </div>

      {/* Title */}
      <h3 className={cn(
        'font-bold text-white truncate',
        variant === 'realm' ? 'text-lg mb-1' : 'text-base mb-0.5'
      )}>
        {label}
      </h3>

      {/* Subtitle - dual count */}
      <p
        className={cn('font-semibold truncate', variant === 'realm' ? 'text-sm' : 'text-xs')}
        style={{ color: colors.primary }}
      >
        {typeCount} types &middot; {loadedCount} loaded
      </p>
    </div>
  );
});
