'use client';

/**
 * AudienceTraitCardContent - "Persona Trait" design for AudienceTrait nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> double
 * - Shows audience trait, category, relevance
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ 👥 AUDIENCE TRAIT     demographic    │ <- Icon + category
 * │ ═════════════════════════════════   │
 * │ tech-savvy-millennial                │
 * │ ┌────────────────────────────────┐  │
 * │ │ "Early adopter of technology"  │  │ <- Trait description
 * │ │ ─────────────────────────────  │  │
 * │ │ importance: HIGH               │  │ <- Importance level
 * │ └────────────────────────────────┘  │
 * │ ◉ knowledge                         │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { UserCircle } from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import type { NodeLayer, NodeRealm } from '../../taxonomyColors';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { GlowEffect } from '../../effects';

// =============================================================================
// Types
// =============================================================================

export type TraitCategory =
  | 'demographic'
  | 'psychographic'
  | 'behavioral'
  | 'professional'
  | 'lifestyle'
  | 'preference'
  | 'pain_point'
  | 'goal';

export type ImportanceLevel = 'low' | 'medium' | 'high' | 'critical';

export interface AudienceTraitNodeData {
  id: string;
  type: 'AudienceTrait';
  key: string;
  displayName: string;
  /** The trait description */
  trait: string;
  /** Category of trait */
  category: TraitCategory;
  /** Importance for content targeting */
  importance: ImportanceLevel;
  /** Description of the trait */
  description?: string;
  /** Content implications */
  implications?: string[];
  /** Tone adjustments for this trait */
  tone_adjustments?: string[];
  /** Topics of interest */
  interests?: string[];
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface AudienceTraitTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
}

export interface AudienceTraitCardContentProps extends CardContext {
  data: AudienceTraitNodeData;
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: AudienceTraitTaxonomyProps;
  /** Show TaxonomyBadge in header (default: true) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Constants
// =============================================================================

const CATEGORY_LABELS: Record<TraitCategory, string> = {
  demographic: 'Demographic',
  psychographic: 'Psychographic',
  behavioral: 'Behavioral',
  professional: 'Professional',
  lifestyle: 'Lifestyle',
  preference: 'Preference',
  pain_point: 'Pain Point',
  goal: 'Goal',
};

const CATEGORY_ICONS: Record<TraitCategory, string> = {
  demographic: '📊',
  psychographic: '🧠',
  behavioral: '🎯',
  professional: '💼',
  lifestyle: '🏃',
  preference: '❤️',
  pain_point: '😤',
  goal: '🎯',
};

const IMPORTANCE_CONFIG: Record<ImportanceLevel, { color: string; bars: number }> = {
  low: { color: '#6b7280', bars: 1 },
  medium: { color: '#f59e0b', bars: 2 },
  high: { color: '#22c55e', bars: 3 },
  critical: { color: '#8b5cf6', bars: 4 },
};

// =============================================================================
// Component
// =============================================================================

export const AudienceTraitCardContent = memo(function AudienceTraitCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = true,
}: AudienceTraitCardContentProps) {
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;

  const importanceConfig = IMPORTANCE_CONFIG[data.importance];
  const categoryIcon = CATEGORY_ICONS[data.category];

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
            className="AudienceTrait"
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
            <UserCircle
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
              AUDIENCE TRAIT
            </span>
          </div>

          {/* Category badge */}
          <span
            className="px-1.5 py-0.5 rounded text-[8px] font-semibold flex items-center gap-1"
            style={{
              background: `${colors.primary}20`,
              color: colors.primary,
            }}
          >
            <span>{categoryIcon}</span>
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

      {/* Trait key */}
      <h3 className="text-sm font-mono text-white/70 mb-2 truncate">
        {data.key}
      </h3>

      {/* Trait value */}
      <div
        className="p-2 rounded-lg mb-2"
        style={{
          backgroundColor: `${colors.primary}10`,
          border: `1px solid ${colors.primary}30`,
        }}
      >
        <p
          className="text-sm font-medium text-white"
          style={glowStyle}
        >
          {data.trait}
        </p>
      </div>

      {/* Importance indicator */}
      <div className="flex items-center gap-2 mb-2">
        <span className="text-[9px] text-white/50 uppercase">Importance:</span>
        <div className="flex items-center gap-0.5">
          {[1, 2, 3, 4].map((bar) => (
            <div
              key={bar}
              className={cn(
                'w-2 h-3 rounded-sm transition-all duration-200',
                bar <= importanceConfig.bars && selected && 'animate-pulse'
              )}
              style={{
                background: bar <= importanceConfig.bars
                  ? importanceConfig.color
                  : `${colors.primary}20`,
                boxShadow: bar <= importanceConfig.bars
                  ? `0 0 4px ${importanceConfig.color}50`
                  : 'none',
              }}
            />
          ))}
        </div>
        <span
          className="text-[9px] font-semibold"
          style={{ color: importanceConfig.color }}
        >
          {data.importance.toUpperCase()}
        </span>
      </div>

      {/* Interests */}
      {data.interests && data.interests.length > 0 && (
        <div className="mb-2">
          <p className="text-[8px] text-white/40 mb-1">Interests:</p>
          <div className={cn('flex flex-wrap', gapTokens.compact)}>
            {data.interests.slice(0, 4).map((interest) => (
              <span
                key={interest}
                className="px-1 py-0.5 rounded text-[8px]"
                style={{
                  background: `${colors.primary}15`,
                  color: colors.primary,
                }}
              >
                {interest}
              </span>
            ))}
            {data.interests.length > 4 && (
              <span className="text-[8px] text-white/40">+{data.interests.length - 4}</span>
            )}
          </div>
        </div>
      )}

      {/* Tone adjustments */}
      {data.tone_adjustments && data.tone_adjustments.length > 0 && (
        <div className="mb-2">
          <p className="text-[8px] text-white/40 mb-1">Tone adjustments:</p>
          <div className={cn('flex flex-wrap', gapTokens.compact)}>
            {data.tone_adjustments.slice(0, 3).map((tone) => (
              <span
                key={tone}
                className="px-1 py-0.5 rounded text-[8px]"
                style={{
                  background: '#22c55e15',
                  color: '#22c55e',
                }}
              >
                {tone}
              </span>
            ))}
          </div>
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
