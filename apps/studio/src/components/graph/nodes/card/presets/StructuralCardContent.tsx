'use client';

/**
 * StructuralCardContent - Content preset for structural nodes
 *
 * Used by: Page, Entity, Block, PageStructure, BlockType, ContentSlot, etc.
 *
 * Enhanced with optional TaxonomyBadge support for full visual encoding (ADR-005):
 * - Layer → Fill color
 * - Realm → Border color
 * - Trait → Border style + animation
 *
 * Layout:
 * ┌─────────────────────────────┐
 * │ [TaxonomyBadge] or simple  │
 * │ Display Name               │
 * │ key (if different)         │
 * │ [Layer Badge] [Locale]     │
 * └─────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { gapTokens } from '@/design/tokens';
import { localeToFlag } from '@/lib/localeUtils';
import type { CardContext } from '../CardShell';
import type { Layer } from '@novanet/core/types';
import type { NodeLayer, NodeRealm, NodeTrait } from '../taxonomyColors';
import { TaxonomyBadge } from '../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface StructuralNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  /** BCP-47 locale code for locale-specific nodes */
  locale?: string;
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface TaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
  trait: NodeTrait;
}

export interface StructuralCardContentProps extends CardContext {
  data: StructuralNodeData;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: TaxonomyProps;
  /** Show TaxonomyBadge instead of simple header (default: false) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Component
// =============================================================================

export const StructuralCardContent = memo(function StructuralCardContent({
  data,
  colors,
  selected,
  isHovered,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = false,
}: StructuralCardContentProps) {
  // Type-safe config lookup with fallback
  const config = (NODE_TYPE_CONFIG as Record<string, { label: string; layer: string }>)[data.type]
    || { label: data.type, layer: 'foundation' };

  // Memoize icon style
  const iconStyle = useMemo(() => ({
    color: colors.primary,
    filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}80)`,
  }), [colors.primary, selected]);

  // Use TaxonomyBadge if taxonomy props provided and showTaxonomyBadge is true
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;

  return (
    <div className="px-3 py-2.5">
      {/* Header: TaxonomyBadge or simple Icon + Type Label */}
      {useTaxonomyBadge ? (
        <div className="mb-2">
          <TaxonomyBadge
            layer={taxonomy.layer}
            realm={taxonomy.realm}
            trait={taxonomy.trait}
            className={data.type}
            selected={selected}
            isHovered={isHovered}
            performanceConfig={performanceConfig}
            size="sm"
            showLayerLabel={true}
            showTraitIndicator={true}
          />
        </div>
      ) : (
        <div className="flex items-center justify-between mb-1.5">
          <div className={cn('flex items-center', gapTokens.default)}>
            <LayerIcon
              layer={config.layer as Layer}
              size={20}
              strokeWidth={2}
              className={cn(
                'transition-transform duration-200',
                (selected || isHovered) && 'scale-110'
              )}
              style={iconStyle}
            />
            <span
              className="text-[10px] font-bold uppercase tracking-wider"
              style={{ color: colors.primary }}
            >
              {config.label}
            </span>
          </div>

          {/* Status dot */}
          <div
            className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
            style={{
              background: colors.primary,
              boxShadow: `0 0 6px ${colors.primary}`,
            }}
          />
        </div>
      )}

      {/* Display Name */}
      <h3 className="text-sm font-bold text-white truncate">
        {data.displayName}
      </h3>

      {/* Key (if different from displayName) */}
      {data.key !== data.displayName && (
        <p
          className="text-[10px] font-mono truncate mt-0.5"
          style={{ color: `${colors.primary}70` }}
        >
          {data.key}
        </p>
      )}

      {/* Badges row: Layer + Locale */}
      <div className={cn('mt-2 flex items-center flex-wrap', gapTokens.compact)}>
        {/* Layer badge */}
        <div
          className={cn(
            'inline-flex items-center px-1.5 py-0.5 rounded-full',
            'text-[8px] font-semibold uppercase tracking-wider border',
            gapTokens.compact
          )}
          style={{
            background: `${colors.primary}15`,
            borderColor: `${colors.primary}35`,
            color: colors.primary,
          }}
        >
          <span
            className={cn('w-1 h-1 rounded-full', selected && 'animate-pulse')}
            style={{
              background: colors.primary,
              boxShadow: `0 0 4px ${colors.primary}`,
            }}
          />
          {config.layer}
        </div>

        {/* Locale tag badge - only shown for locale-specific nodes */}
        {data.locale && (
          <div
            className="inline-flex items-center px-1.5 py-0.5 rounded-full text-[8px] font-medium border gap-1"
            style={{
              background: 'rgba(255, 255, 255, 0.08)',
              borderColor: 'rgba(255, 255, 255, 0.15)',
            }}
          >
            <span className="text-xs leading-none">{localeToFlag(data.locale)}</span>
            <span className="text-white/70 font-mono">{data.locale}</span>
          </div>
        )}
      </div>
    </div>
  );
});
