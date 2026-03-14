'use client';

/**
 * KnowledgeSetCardContent - "Container Collection" design for Knowledge Set nodes
 *
 * Handles: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> solid (defined trait - containers are invariant)
 * - Shows domain, item count, and grouping info
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ 📦 TERM SET              pricing     │ <- Container icon + domain
 * │ ═════════════════════════════════    │
 * │ technical-terms                      │ <- Set key
 * │ ┌────────────────────────────────┐   │
 * │ │ ████████████░░░░░░ 125 items   │   │ <- Item count gauge
 * │ │ ─────────────────────────────  │   │
 * │ │ vocabulary for technical docs  │   │ <- Description
 * │ └────────────────────────────────┘   │
 * │ ◉ knowledge                          │ <- Layer badge
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import {
  BookOpen,
  MessageSquare,
  Layers,
  Palette,
  AlertTriangle,
  Users,
  type LucideIcon,
} from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import type { NodeLayer, NodeRealm } from '../../taxonomyColors';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { DomainBadge } from './KnowledgeHelpers';

// =============================================================================
// Types
// =============================================================================

export type KnowledgeSetType =
  | 'TermSet'
  | 'ExpressionSet'
  | 'PatternSet'
  | 'CultureSet'
  | 'TabooSet'
  | 'AudienceSet';

export interface KnowledgeSetNodeData {
  id: string;
  type: KnowledgeSetType;
  key: string;
  displayName: string;
  /** Vocabulary domain */
  domain?: 'pricing' | 'features' | 'technical' | 'marketing' | 'support' | 'legal' | 'general';
  /** Description of the set */
  description?: string;
  /** Number of items in the set */
  itemCount?: number;
  /** Register level (for TermSet/ExpressionSet) */
  register?: 'formal' | 'neutral' | 'casual' | 'technical';
  /** Target audience (for AudienceSet) */
  audience?: string;
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface KnowledgeSetTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
}

export interface KnowledgeSetCardContentProps extends CardContext {
  data: KnowledgeSetNodeData;
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: KnowledgeSetTaxonomyProps;
  /** Show TaxonomyBadge in header (default: true) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Icon & Label Mapping
// =============================================================================

const TYPE_ICONS: Record<KnowledgeSetType, LucideIcon> = {
  TermSet: BookOpen,
  ExpressionSet: MessageSquare,
  PatternSet: Layers,
  CultureSet: Palette,
  TabooSet: AlertTriangle,
  AudienceSet: Users,
};

const TYPE_LABELS: Record<KnowledgeSetType, string> = {
  TermSet: 'TERM SET',
  ExpressionSet: 'EXPRESSION SET',
  PatternSet: 'PATTERN SET',
  CultureSet: 'CULTURE SET',
  TabooSet: 'TABOO SET',
  AudienceSet: 'AUDIENCE SET',
};

const TYPE_ITEM_LABELS: Record<KnowledgeSetType, string> = {
  TermSet: 'terms',
  ExpressionSet: 'expressions',
  PatternSet: 'patterns',
  CultureSet: 'culture refs',
  TabooSet: 'taboos',
  AudienceSet: 'traits',
};

// =============================================================================
// Component
// =============================================================================

export const KnowledgeSetCardContent = memo(function KnowledgeSetCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = true,
}: KnowledgeSetCardContentProps) {
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;

  // Get icon and labels
  const IconComponent = TYPE_ICONS[data.type] || BookOpen;
  const typeLabel = TYPE_LABELS[data.type] || data.type;
  const itemLabel = TYPE_ITEM_LABELS[data.type] || 'items';

  // Icon style with glow
  const iconStyle = useMemo(
    () => ({
      color: colors.primary,
      filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}80)`,
    }),
    [colors.primary, selected]
  );

  // Glow style for set name
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

  // Calculate fill percentage for gauge
  const fillPercentage = useMemo(() => {
    if (!data.itemCount) return 0;
    // Normalize to 100 for visual representation
    const maxItems = 200; // Visual max
    return Math.min((data.itemCount / maxItems) * 100, 100);
  }, [data.itemCount]);

  return (
    <div className="relative px-4 py-4">
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

          {/* Domain badge if available */}
          {data.domain && (
            <DomainBadge domain={data.domain} />
          )}
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

      {/* Set key/name */}
      <h3
        className={cn(
          'text-lg font-bold text-white mb-2',
          'transition-all duration-200'
        )}
        style={glowStyle}
      >
        {data.displayName}
      </h3>

      {/* Item count gauge */}
      {data.itemCount !== undefined && (
        <div
          className="p-2 rounded-lg mb-2"
          style={{
            backgroundColor: `${colors.primary}10`,
            border: `1px solid ${colors.primary}30`,
          }}
        >
          <div className="flex items-center justify-between mb-1">
            <span className="text-[9px] text-white/50 uppercase">{itemLabel}</span>
            <span className="text-xs font-mono text-white/80">{data.itemCount}</span>
          </div>
          <div
            className="h-1.5 rounded-full overflow-hidden"
            style={{ backgroundColor: `${colors.primary}20` }}
          >
            <div
              className={cn(
                'h-full rounded-full transition-all duration-500',
                selected && 'animate-pulse'
              )}
              style={{
                width: `${fillPercentage}%`,
                background: `linear-gradient(90deg, ${colors.primary}, ${colors.primary}80)`,
                boxShadow: `0 0 6px ${colors.primary}60`,
              }}
            />
          </div>
        </div>
      )}

      {/* Description */}
      {data.description && (
        <p className="text-[10px] text-white/60 line-clamp-2 mb-2">
          {data.description}
        </p>
      )}

      {/* Register badge */}
      {data.register && (
        <div
          className="inline-flex items-center px-1.5 py-0.5 rounded text-[8px] font-semibold mb-2"
          style={{
            background: `${colors.primary}20`,
            color: colors.primary,
          }}
        >
          {data.register === 'formal' && '▲'}
          {data.register === 'neutral' && '●'}
          {data.register === 'casual' && '▼'}
          {data.register === 'technical' && '◆'}
          <span className="ml-1">{data.register}</span>
        </div>
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
