'use client';

/**
 * LocaleCardContent - "Passport Élégant" design for Locale nodes
 *
 * Features:
 * - Large flag in dedicated left zone with radial glow
 * - BCP-47 code as hero element (28px mono bold)
 * - Vertical glowing separator
 * - Display name and region context
 * - Layer badge with pulse effect
 *
 * Layout:
 * ┌─────────────────────────────────────────┐
 * │  🇫🇷  │  🌐 LOCALE                    ● │
 * │      │                                  │
 * │ glow │     fr-FR                        │
 * │ zone │                                  │
 * │      │  French (France)                 │
 * │      │  ─────────────────────────────── │
 * │      │  Europe • Western Europe ●config │
 * └──────┴──────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { localeToFlag } from '@/lib/localeUtils';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../CardShell';
import type { Layer } from '@novanet/core/types';
import type { NodeLayer, NodeRealm, NodeTrait } from '../taxonomyColors';
import { TaxonomyBadge } from '../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface LocaleNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  /** Region/geographic context (e.g., "Europe • Western Europe") */
  region?: string;
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface LocaleTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
  trait: NodeTrait;
}

export interface LocaleCardContentProps extends CardContext {
  data: LocaleNodeData;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: LocaleTaxonomyProps;
  /** Show TaxonomyBadge instead of simple header (default: false) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Helper: Extract language name from displayName
// =============================================================================

/**
 * Extract display parts from locale display name
 * "French (France)" → { language: "French", region: "France" }
 */
function parseDisplayName(displayName: string): { language: string; region?: string } {
  const match = displayName.match(/^(.+?)\s*\((.+)\)$/);
  if (match) {
    return { language: match[1].trim(), region: match[2].trim() };
  }
  return { language: displayName };
}

// =============================================================================
// Component
// =============================================================================

export const LocaleCardContent = memo(function LocaleCardContent({
  data,
  colors,
  selected,
  isHovered,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = false,
}: LocaleCardContentProps) {
  // Use TaxonomyBadge if taxonomy props provided and showTaxonomyBadge is true
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;
  // Extract BCP-47 code from key (e.g., "Locale:fr-FR" → "fr-FR")
  const bcp47 = useMemo(() => {
    const parts = data.key.split(':');
    return parts.length > 1 ? parts[1] : data.key;
  }, [data.key]);

  // Get flag emoji from BCP-47
  const flag = useMemo(() => localeToFlag(bcp47), [bcp47]);

  // Parse display name for structured display
  const { language, region } = useMemo(
    () => parseDisplayName(data.displayName),
    [data.displayName]
  );

  // Icon style with glow
  const iconStyle = useMemo(
    () => ({
      color: colors.primary,
      filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}80)`,
    }),
    [colors.primary, selected]
  );

  // Flag zone gradient (radial glow effect)
  const flagZoneStyle = useMemo(
    () => ({
      background: `radial-gradient(circle at center, ${colors.primary}20 0%, transparent 70%)`,
    }),
    [colors.primary]
  );

  // Separator glow style
  const separatorStyle = useMemo(
    () => ({
      background: `linear-gradient(180deg, transparent 0%, ${colors.primary}40 50%, transparent 100%)`,
      boxShadow: `0 0 8px ${colors.primary}30`,
    }),
    [colors.primary]
  );

  return (
    <div className="flex h-full">
      {/* Flag Zone (left) */}
      <div
        className="flex items-center justify-center shrink-0"
        style={{
          width: 60,
          ...flagZoneStyle,
        }}
      >
        <span
          className={cn(
            'text-4xl transition-transform duration-200',
            (selected || isHovered) && 'scale-110'
          )}
          style={{
            filter: selected ? `drop-shadow(0 0 12px ${colors.primary}60)` : undefined,
          }}
        >
          {flag}
        </span>
      </div>

      {/* Vertical Separator */}
      <div
        className="w-[1px] shrink-0 self-stretch my-2"
        style={separatorStyle}
      />

      {/* Content Zone (right) */}
      <div className="flex-1 px-3 py-2.5 min-w-0">
        {/* Header: TaxonomyBadge or Icon + Type Label + Status Dot */}
        {useTaxonomyBadge ? (
          <div className="mb-1">
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
          <div className="flex items-center justify-between mb-1">
            <div className={cn('flex items-center', gapTokens.default)}>
              <LayerIcon
                layer={'locale' as Layer}
                size={18}
                strokeWidth={2}
                className={cn(
                  'transition-transform duration-200',
                  (selected || isHovered) && 'scale-110'
                )}
                style={iconStyle}
              />
              <span
                className="text-[9px] font-bold uppercase tracking-wider"
                style={{ color: colors.primary }}
              >
                Locale
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

        {/* BCP-47 Hero */}
        <h3
          className={cn(
            'text-2xl font-bold font-mono tracking-wide text-white',
            'transition-all duration-200',
            (selected || isHovered) && 'tracking-wider'
          )}
          style={{
            textShadow: selected ? `0 0 20px ${colors.primary}60` : undefined,
          }}
        >
          {bcp47}
        </h3>

        {/* Display Name */}
        <p className="text-xs text-white/70 mt-0.5 truncate">
          {language}
          {region && <span className="text-white/40"> ({region})</span>}
        </p>

        {/* Divider */}
        <div
          className="h-px mt-2 mb-1.5"
          style={{
            background: `linear-gradient(90deg, ${colors.primary}30, transparent)`,
          }}
        />

        {/* Footer: Region + Layer Badge */}
        <div className={cn('flex items-center justify-between', gapTokens.compact)}>
          {/* Region context */}
          {data.region && (
            <span className="text-[9px] text-white/40 truncate flex-1">
              {data.region}
            </span>
          )}

          {/* Layer badge */}
          <div
            className={cn(
              'inline-flex items-center px-1.5 py-0.5 rounded-full shrink-0',
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
            config
          </div>
        </div>
      </div>
    </div>
  );
});
