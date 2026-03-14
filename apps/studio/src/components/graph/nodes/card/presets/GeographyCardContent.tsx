'use client';

/**
 * GeographyCardContent - "Geographic Compass" design for Geography layer nodes
 *
 * Handles: Continent, GeoRegion, GeoSubRegion, Country, IncomeGroup, LendingCategory
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = geography) → emerald #10b981
 * - Shows hierarchy context (continent > region > country)
 *
 * Layout:
 * ┌─────────────────────────────────────────┐
 * │ 🌍 CONTINENT                         ●  │
 * │ ════════════════════════════════════    │
 * │                                         │
 * │     EU                                  │ ← Code hero
 * │                                         │
 * │ Europe                                  │
 * │ ─────────────────────────────────────── │
 * │ M49: 150 │ 4 regions │ ◉ geography     │
 * └─────────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { Globe, Flag, MapPin, Map, Landmark, Banknote, Building2, type LucideIcon } from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../CardShell';
import type { NodeLayer, NodeRealm } from '../taxonomyColors';
import { TaxonomyBadge } from '../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export type GeographyNodeType =
  | 'Continent'
  | 'GeoRegion'
  | 'GeoSubRegion'
  | 'Country'
  | 'IncomeGroup'
  | 'LendingCategory'
  | 'EconomicRegion';

export interface GeographyNodeData {
  id: string;
  type: GeographyNodeType;
  key: string;
  displayName: string;
  /** UN M49 numeric code */
  m49Code?: number;
  /** ISO alpha-3 code (for Country) */
  alpha3?: string;
  /** Parent region name */
  region?: string;
  /** Sub-region name */
  subRegion?: string;
  /** Parent continent code */
  continent?: string;
  /** Number of child regions (for Continent) */
  regionCount?: number;
  /** Number of locales (for Country) */
  localeCount?: number;
  /** Has cultural style JSON */
  hasCulturalStyle?: boolean;
  /** Has visual prompt JSON */
  hasVisualPrompt?: boolean;
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface GeographyTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
}

export interface GeographyCardContentProps extends CardContext {
  data: GeographyNodeData;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: GeographyTaxonomyProps;
  /** Show TaxonomyBadge instead of simple header (default: false) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Icon Mapping
// =============================================================================

const TYPE_ICONS: Record<GeographyNodeType, LucideIcon> = {
  Continent: Globe,
  GeoRegion: Map,
  GeoSubRegion: MapPin,
  Country: Flag,
  IncomeGroup: Banknote,
  LendingCategory: Landmark,
  EconomicRegion: Building2,
};

const TYPE_LABELS: Record<GeographyNodeType, string> = {
  Continent: 'CONTINENT',
  GeoRegion: 'REGION',
  GeoSubRegion: 'SUB-REGION',
  Country: 'COUNTRY',
  IncomeGroup: 'INCOME',
  LendingCategory: 'LENDING',
  EconomicRegion: 'ECON REGION',
};

// =============================================================================
// Component
// =============================================================================

export const GeographyCardContent = memo(function GeographyCardContent({
  data,
  colors,
  selected,
  isHovered,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = false,
}: GeographyCardContentProps) {
  // Use TaxonomyBadge if taxonomy props provided and showTaxonomyBadge is true
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;

  // Get icon component for this geography type
  const IconComponent = TYPE_ICONS[data.type] || Globe;
  const typeLabel = TYPE_LABELS[data.type] || data.type.toUpperCase();

  // Icon style with glow
  const iconStyle = useMemo(
    () => ({
      color: colors.primary,
      filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}80)`,
    }),
    [colors.primary, selected]
  );

  // Glow style for code
  const codeGlowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 20px ${colors.primary}60`
        : isHovered
          ? `0 0 12px ${colors.primary}40`
          : `0 0 6px ${colors.primary}20`,
    }),
    [colors.primary, selected, isHovered]
  );

  // Build context line based on type
  const contextLine = useMemo(() => {
    const parts: string[] = [];

    if (data.continent && data.type !== 'Continent') {
      parts.push(data.continent);
    }
    if (data.region && data.type !== 'GeoRegion' && data.type !== 'Continent') {
      parts.push(data.region);
    }
    if (data.subRegion && data.type !== 'GeoSubRegion') {
      parts.push(data.subRegion);
    }

    return parts.join(' › ');
  }, [data]);

  // Build stats based on type
  const stats = useMemo(() => {
    const items: Array<{ label: string; value: string | number }> = [];

    if (data.m49Code) {
      items.push({ label: 'M49', value: data.m49Code });
    }
    if (data.alpha3) {
      items.push({ label: 'ISO', value: data.alpha3 });
    }
    if (data.regionCount !== undefined) {
      items.push({ label: 'regions', value: data.regionCount });
    }
    if (data.localeCount !== undefined) {
      items.push({ label: 'locales', value: data.localeCount });
    }

    return items;
  }, [data]);

  return (
    <div className="px-4 py-3">
      {/* Header: TaxonomyBadge or Icon + Type Label */}
      {useTaxonomyBadge ? (
        <div className="mb-2">
          <TaxonomyBadge
            layer={taxonomy.layer}
            realm={taxonomy.realm}
            className={data.type}
            selected={selected}
            isHovered={isHovered}
            performanceConfig={performanceConfig}
            size="sm"
            showLayerLabel={true}
          />
        </div>
      ) : (
        <div className="flex items-center justify-between mb-2">
          <div className={cn('flex items-center', gapTokens.default)}>
            <IconComponent
              size={18}
              strokeWidth={2}
              className={cn(
                'transition-transform duration-200',
                (selected || isHovered) && 'scale-110'
              )}
              style={iconStyle}
            />
            <span
              className="text-[9px] font-bold uppercase tracking-widest font-mono"
              style={{ color: colors.primary }}
            >
              {typeLabel}
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

      {/* Double line separator */}
      <div className="mb-3">
        <div
          className="h-[2px] mb-[2px]"
          style={{
            background: `linear-gradient(90deg, ${colors.primary}60, ${colors.primary}20, transparent)`,
          }}
        />
        <div
          className="h-[1px]"
          style={{
            background: `linear-gradient(90deg, ${colors.primary}40, transparent)`,
          }}
        />
      </div>

      {/* Code Hero */}
      <h3
        className={cn(
          'text-2xl font-bold font-mono tracking-wide text-white text-center',
          'transition-all duration-200',
          (selected || isHovered) && 'tracking-wider'
        )}
        style={codeGlowStyle}
      >
        {data.key}
      </h3>

      {/* Display Name */}
      <p className="text-sm text-white/80 text-center mt-1 truncate">
        {data.displayName}
      </p>

      {/* Context line (hierarchy path) */}
      {contextLine && (
        <p className="text-[10px] text-white/40 text-center mt-1 truncate">
          {contextLine}
        </p>
      )}

      {/* Divider */}
      <div
        className="h-px mt-3 mb-2"
        style={{
          background: `linear-gradient(90deg, transparent, ${colors.primary}30, transparent)`,
        }}
      />

      {/* Stats row */}
      <div className={cn('flex items-center justify-center flex-wrap', gapTokens.default)}>
        {stats.map((stat, index) => (
          <div
            key={stat.label}
            className={cn(
              'flex items-center text-[9px]',
              index > 0 && 'border-l border-white/10 pl-2'
            )}
          >
            <span className="text-white/40 mr-1">{stat.label}:</span>
            <span className="text-white/70 font-mono">{stat.value}</span>
          </div>
        ))}

        {/* AI capabilities indicators */}
        {data.hasCulturalStyle && (
          <div
            className="flex items-center px-1.5 py-0.5 rounded text-[8px]"
            style={{
              background: `${colors.primary}20`,
              color: colors.primary,
            }}
            title="Has cultural style"
          >
            🎨
          </div>
        )}
        {data.hasVisualPrompt && (
          <div
            className="flex items-center px-1.5 py-0.5 rounded text-[8px]"
            style={{
              background: `${colors.primary}20`,
              color: colors.primary,
            }}
            title="Has visual prompt"
          >
            🖼️
          </div>
        )}
      </div>

      {/* Layer badge */}
      <div className="flex justify-center mt-2">
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
          geography
        </div>
      </div>
    </div>
  );
});
