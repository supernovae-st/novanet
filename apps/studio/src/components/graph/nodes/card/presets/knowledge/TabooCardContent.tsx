'use client';

/**
 * TabooCardContent - "Warning Zone" design for Taboo nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> double
 * - Shows taboo topic, severity, affected regions
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ ⚠️ TABOO              political      │ <- Warning icon + category
 * │ ═════════════════════════════════   │
 * │ tiananmen-reference                  │
 * │ ┌────────────────────────────────┐  │
 * │ │ ⛔ CRITICAL - Never use        │  │ <- Severity indicator
 * │ │ ─────────────────────────────  │  │
 * │ │ CN, HK, TW                     │  │ <- Affected regions
 * │ └────────────────────────────────┘  │
 * │ ◉ knowledge                         │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { ShieldAlert } from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import type { NodeLayer, NodeRealm } from '../../taxonomyColors';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export type TabooCategory =
  | 'political'
  | 'religious'
  | 'cultural'
  | 'historical'
  | 'sexual'
  | 'violent'
  | 'discriminatory'
  | 'other';

export type TabooSeverity = 'avoid' | 'sensitive' | 'critical' | 'illegal';

export interface TabooNodeData {
  id: string;
  type: 'Taboo';
  key: string;
  displayName: string;
  /** The taboo topic/phrase */
  topic: string;
  /** Category of taboo */
  category: TabooCategory;
  /** Severity level */
  severity: TabooSeverity;
  /** Description of why this is taboo */
  description?: string;
  /** Regions where this applies */
  regions?: string[];
  /** What to use instead */
  alternatives?: string[];
  /** Legal implications if any */
  legal_note?: string;
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface TabooTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
}

export interface TabooCardContentProps extends CardContext {
  data: TabooNodeData;
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: TabooTaxonomyProps;
  /** Show TaxonomyBadge in header (default: true) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Constants
// =============================================================================

const CATEGORY_LABELS: Record<TabooCategory, string> = {
  political: 'Political',
  religious: 'Religious',
  cultural: 'Cultural',
  historical: 'Historical',
  sexual: 'Sexual',
  violent: 'Violent',
  discriminatory: 'Discriminatory',
  other: 'Other',
};

const SEVERITY_CONFIG: Record<TabooSeverity, { color: string; label: string; icon: string }> = {
  avoid: { color: '#f59e0b', label: 'Avoid if possible', icon: '⚠' },
  sensitive: { color: '#f97316', label: 'Handle with care', icon: '⚡' },
  critical: { color: '#ef4444', label: 'Never use', icon: '⛔' },
  illegal: { color: '#dc2626', label: 'ILLEGAL', icon: '🚫' },
};

// =============================================================================
// Component
// =============================================================================

export const TabooCardContent = memo(function TabooCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = true,
}: TabooCardContentProps) {
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;

  const severityConfig = SEVERITY_CONFIG[data.severity];

  // Warning glow style
  const warningGlowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 12px ${severityConfig.color}60`
        : isHovered
          ? `0 0 8px ${severityConfig.color}40`
          : 'none',
    }),
    [severityConfig.color, selected, isHovered]
  );

  return (
    <div className="relative px-4 py-4">
      {/* Warning pattern background */}
      <div
        className="absolute inset-0 pointer-events-none opacity-[0.03]"
        style={{
          backgroundImage: `repeating-linear-gradient(
            45deg,
            ${severityConfig.color} 0px,
            ${severityConfig.color} 10px,
            transparent 10px,
            transparent 20px
          )`,
        }}
      />

      {/* Header: TaxonomyBadge or Icon + Type Label */}
      {useTaxonomyBadge ? (
        <div className="relative mb-2">
          <TaxonomyBadge
            layer={taxonomy.layer}
            realm={taxonomy.realm}
            className="Taboo"
            selected={selected}
            isHovered={isHovered}
            performanceConfig={performanceConfig}
            size="sm"
            showLayerLabel={true}
          />
        </div>
      ) : (
        <div className="relative flex items-center justify-between mb-2">
          <div className={cn('flex items-center', gapTokens.default)}>
            <ShieldAlert
              size={18}
              strokeWidth={2}
              className={cn(
                'transition-transform duration-200',
                (selected || isHovered) && 'scale-110'
              )}
              style={{
                color: severityConfig.color,
                filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${severityConfig.color}80)`,
              }}
            />
            <span
              className="text-[9px] font-bold uppercase tracking-widest font-mono"
              style={{ color: severityConfig.color }}
            >
              TABOO
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

      {/* Double line separator - warning color */}
      <div className="relative mb-3">
        <div
          className="h-[2px] mb-[2px]"
          style={{
            background: `linear-gradient(90deg, ${severityConfig.color}60, ${severityConfig.color}20, transparent)`,
          }}
        />
        <div
          className="h-[1px]"
          style={{
            background: `linear-gradient(90deg, ${severityConfig.color}40, transparent)`,
          }}
        />
      </div>

      {/* Taboo key */}
      <h3 className="relative text-sm font-mono text-white/70 mb-2 truncate">
        {data.key}
      </h3>

      {/* Severity indicator */}
      <div
        className="relative flex items-center gap-2 px-3 py-2 rounded-lg mb-2"
        style={{
          background: `${severityConfig.color}15`,
          border: `2px solid ${severityConfig.color}50`,
        }}
      >
        <span className="text-lg">{severityConfig.icon}</span>
        <div>
          <p
            className="text-sm font-bold"
            style={{ color: severityConfig.color, ...warningGlowStyle }}
          >
            {data.severity.toUpperCase()}
          </p>
          <p className="text-[9px] text-white/60">{severityConfig.label}</p>
        </div>
      </div>

      {/* Topic (what to avoid) */}
      <div
        className="relative p-2 rounded mb-2"
        style={{
          backgroundColor: `${colors.primary}10`,
          border: `1px solid ${colors.primary}30`,
        }}
      >
        <p className="text-[9px] text-white/50 uppercase mb-1">Topic to avoid:</p>
        <p className="text-xs text-white/80 line-through">{data.topic}</p>
      </div>

      {/* Affected regions */}
      {data.regions && data.regions.length > 0 && (
        <div className="relative mb-2">
          <p className="text-[8px] text-white/40 mb-1">Affected regions:</p>
          <div className={cn('flex flex-wrap', gapTokens.compact)}>
            {data.regions.slice(0, 5).map((region) => (
              <span
                key={region}
                className="px-1 py-0.5 rounded text-[8px] font-mono"
                style={{
                  background: `${severityConfig.color}20`,
                  color: severityConfig.color,
                }}
              >
                {region}
              </span>
            ))}
            {data.regions.length > 5 && (
              <span className="text-[8px] text-white/40">+{data.regions.length - 5}</span>
            )}
          </div>
        </div>
      )}

      {/* Alternatives */}
      {data.alternatives && data.alternatives.length > 0 && (
        <div className="relative mb-2">
          <p className="text-[8px] text-green-400/70 mb-1">Use instead:</p>
          <div className={cn('flex flex-wrap', gapTokens.compact)}>
            {data.alternatives.slice(0, 3).map((alt) => (
              <span
                key={alt}
                className="px-1 py-0.5 rounded text-[8px]"
                style={{
                  background: '#22c55e20',
                  color: '#22c55e',
                }}
              >
                {alt}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Description */}
      {data.description && (
        <p className="relative text-[10px] text-white/60 line-clamp-2 mb-2">
          {data.description}
        </p>
      )}

      {/* Legal note */}
      {data.legal_note && (
        <p
          className="relative text-[9px] px-2 py-1 rounded"
          style={{
            background: `${severityConfig.color}10`,
            color: severityConfig.color,
          }}
        >
          Legal: {data.legal_note}
        </p>
      )}

      {/* Divider */}
      <div
        className="relative h-px my-2"
        style={{
          background: `linear-gradient(90deg, transparent, ${colors.primary}30, transparent)`,
        }}
      />

      {/* Layer badge */}
      <div className="relative flex justify-center">
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
