'use client';

/**
 * CultureRefCardContent - "Cultural Reference" design for CultureRef nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> double
 * - Shows cultural reference, category, sensitivity level
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ 🎭 CULTURE REF        holiday        │ <- Culture icon + category
 * │ ═════════════════════════════════   │
 * │ christmas-greeting                   │
 * │ ┌────────────────────────────────┐  │
 * │ │ "Merry Christmas"              │  │ <- The reference
 * │ │ ─────────────────────────────  │  │
 * │ │ ⚠ medium sensitivity           │  │ <- Sensitivity level
 * │ └────────────────────────────────┘  │
 * │ ◉ knowledge                         │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { Theater } from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import type { NodeLayer, NodeRealm } from '../../taxonomyColors';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { GlowEffect } from '../../effects';

// =============================================================================
// Types
// =============================================================================

export type CultureCategory =
  | 'holiday'
  | 'tradition'
  | 'religion'
  | 'humor'
  | 'idiom'
  | 'gesture'
  | 'food'
  | 'custom'
  | 'general';

export type SensitivityLevel = 'low' | 'medium' | 'high' | 'critical';

export interface CultureRefNodeData {
  id: string;
  type: 'CultureRef';
  key: string;
  displayName: string;
  /** The cultural reference text/phrase */
  reference: string;
  /** Category of cultural reference */
  category: CultureCategory;
  /** Sensitivity level for localization */
  sensitivity: SensitivityLevel;
  /** Description of the cultural context */
  description?: string;
  /** Regions where this applies */
  regions?: string[];
  /** Alternative references for other cultures */
  alternatives?: string[];
  /** When to avoid using this reference */
  avoid_when?: string[];
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface CultureRefTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
}

export interface CultureRefCardContentProps extends CardContext {
  data: CultureRefNodeData;
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: CultureRefTaxonomyProps;
  /** Show TaxonomyBadge in header (default: true) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Constants
// =============================================================================

const CATEGORY_LABELS: Record<CultureCategory, string> = {
  holiday: 'Holiday',
  tradition: 'Tradition',
  religion: 'Religion',
  humor: 'Humor',
  idiom: 'Idiom',
  gesture: 'Gesture',
  food: 'Food',
  custom: 'Custom',
  general: 'General',
};

const SENSITIVITY_COLORS: Record<SensitivityLevel, string> = {
  low: '#22c55e',
  medium: '#f59e0b',
  high: '#ef4444',
  critical: '#dc2626',
};

const SENSITIVITY_ICONS: Record<SensitivityLevel, string> = {
  low: '○',
  medium: '◐',
  high: '●',
  critical: '◉',
};

// =============================================================================
// Component
// =============================================================================

export const CultureRefCardContent = memo(function CultureRefCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = true,
}: CultureRefCardContentProps) {
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;

  // Glow style
  const glowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 12px ${colors.primary}60`
        : isHovered
          ? `0 0 8px ${colors.primary}40`
          : 'none',
    }),
    [colors.primary, selected, isHovered]
  );

  const sensitivityColor = SENSITIVITY_COLORS[data.sensitivity];
  const sensitivityIcon = SENSITIVITY_ICONS[data.sensitivity];
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;

  return (
    <div className="relative px-4 py-4">
      {/* Premium glow effect */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={colors.primary}
          intensity={selected ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Header: TaxonomyBadge or Icon + Type Label */}
      {useTaxonomyBadge ? (
        <div className="mb-2">
          <TaxonomyBadge
            layer={taxonomy.layer}
            realm={taxonomy.realm}
            className="CultureRef"
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
            <Theater
              size={18}
              strokeWidth={2}
              className={cn(
                'transition-transform duration-200',
                (selected || isHovered) && 'scale-110'
              )}
              style={{
                color: colors.primary,
                filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}80)`,
              }}
            />
            <span
              className="text-[9px] font-bold uppercase tracking-widest font-mono"
              style={{ color: colors.primary }}
            >
              CULTURE REF
            </span>
          </div>

          {/* Category badge */}
          <span
            className="px-1.5 py-0.5 rounded text-[8px] font-semibold"
            style={{
              background: `${colors.primary}20`,
              color: colors.primary,
            }}
          >
            {CATEGORY_LABELS[data.category]}
          </span>
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

      {/* Reference key */}
      <h3 className="text-sm font-mono text-white/70 mb-2 truncate">
        {data.key}
      </h3>

      {/* Reference value */}
      <div
        className="p-2 rounded-lg mb-2"
        style={{
          backgroundColor: `${colors.primary}10`,
          border: `1px solid ${colors.primary}30`,
        }}
      >
        <p
          className="text-sm font-medium text-white italic"
          style={glowStyle}
        >
          &quot;{data.reference}&quot;
        </p>
      </div>

      {/* Sensitivity level */}
      <div
        className="flex items-center gap-2 px-2 py-1 rounded mb-2"
        style={{
          background: `${sensitivityColor}15`,
          border: `1px solid ${sensitivityColor}30`,
        }}
      >
        <span style={{ color: sensitivityColor }}>{sensitivityIcon}</span>
        <span className="text-[9px] font-semibold" style={{ color: sensitivityColor }}>
          {data.sensitivity.toUpperCase()} SENSITIVITY
        </span>
      </div>

      {/* Regions */}
      {data.regions && data.regions.length > 0 && (
        <div className={cn('flex flex-wrap mb-2', gapTokens.compact)}>
          {data.regions.slice(0, 4).map((region) => (
            <span
              key={region}
              className="px-1 py-0.5 rounded text-[8px] font-mono"
              style={{
                background: `${colors.primary}15`,
                color: colors.primary,
              }}
            >
              {region}
            </span>
          ))}
          {data.regions.length > 4 && (
            <span className="text-[8px] text-white/40">+{data.regions.length - 4}</span>
          )}
        </div>
      )}

      {/* Description */}
      {data.description && (
        <p className="text-[10px] text-white/60 line-clamp-2 mb-2">
          {data.description}
        </p>
      )}

      {/* Divider */}
      <div
        className="h-px my-2"
        style={{
          background: `linear-gradient(90deg, transparent, ${colors.primary}30, transparent)`,
        }}
      />

      {/* Layer badge */}
      <div className="flex justify-center">
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
          knowledge
        </div>
      </div>
    </div>
  );
});
