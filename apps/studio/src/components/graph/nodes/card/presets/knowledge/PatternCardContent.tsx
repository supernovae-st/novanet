'use client';

/**
 * PatternCardContent - "Template Pattern" design for Pattern nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> double
 * - Shows pattern template, placeholders, and usage
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ 🔣 PATTERN            cta-button    │ <- Pattern icon + domain
 * │ ═════════════════════════════════   │
 * │ click-here-pattern                  │
 * │ ┌────────────────────────────────┐  │
 * │ │ "Click {{action}} to {{goal}}"│  │ <- Template with placeholders
 * │ │ ─────────────────────────────  │  │
 * │ │ {{action}} {{goal}}            │  │ <- Placeholders
 * │ └────────────────────────────────┘  │
 * │ ◉ knowledge                         │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo, Fragment } from 'react';
import { cn } from '@/lib/utils';
import { Regex } from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import type { NodeLayer, NodeRealm } from '../../taxonomyColors';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { DomainBadge } from './KnowledgeHelpers';
import { GlowEffect } from '../../effects';

// =============================================================================
// Types
// =============================================================================

export interface PatternNodeData {
  id: string;
  type: 'Pattern';
  key: string;
  displayName: string;
  /** The pattern template with {{placeholders}} */
  template: string;
  /** Description of the pattern */
  description?: string;
  /** Domain of usage */
  domain?: 'pricing' | 'features' | 'technical' | 'marketing' | 'support' | 'legal' | 'general';
  /** Extracted placeholder names */
  placeholders?: string[];
  /** Example filled-in pattern */
  example?: string;
  /** Usage context */
  use_in?: string[];
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface PatternTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
}

export interface PatternCardContentProps extends CardContext {
  data: PatternNodeData;
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: PatternTaxonomyProps;
  /** Show TaxonomyBadge in header (default: true) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Helper: Extract placeholders from template
// =============================================================================

function extractPlaceholders(template: string): string[] {
  const matches = template.match(/\{\{(\w+)\}\}/g);
  if (!matches) return [];
  return [...new Set(matches.map(m => m.replace(/\{\{|\}\}/g, '')))];
}

// =============================================================================
// Helper: Render template with highlighted placeholders
// =============================================================================

interface HighlightedTemplateProps {
  template: string;
  color: string;
}

const HighlightedTemplate = memo(function HighlightedTemplate({
  template,
  color,
}: HighlightedTemplateProps) {
  const parts = useMemo(() => {
    // Split by placeholder pattern while keeping the delimiters
    const regex = /(\{\{\w+\}\})/g;
    return template.split(regex);
  }, [template]);

  return (
    <>
      {parts.map((part, i) => {
        if (/^\{\{\w+\}\}$/.test(part)) {
          return (
            <span key={i} style={{ color, fontWeight: 'bold' }}>
              {part}
            </span>
          );
        }
        return <Fragment key={i}>{part}</Fragment>;
      })}
    </>
  );
});

// =============================================================================
// Component
// =============================================================================

export const PatternCardContent = memo(function PatternCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = true,
}: PatternCardContentProps) {
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

  // Extract placeholders
  const placeholders = useMemo(
    () => data.placeholders || extractPlaceholders(data.template),
    [data.placeholders, data.template]
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
            className="Pattern"
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
            <Regex
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
              PATTERN
            </span>
          </div>

          {data.domain && <DomainBadge domain={data.domain} />}
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

      {/* Pattern key */}
      <h3
        className={cn(
          'text-sm font-mono text-white/70 mb-2 truncate',
          'transition-all duration-200'
        )}
      >
        {data.key}
      </h3>

      {/* Template with highlighted placeholders */}
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
          &quot;<HighlightedTemplate template={data.template} color={colors.primary} />&quot;
        </p>
      </div>

      {/* Placeholders */}
      {placeholders.length > 0 && (
        <div className={cn('flex flex-wrap mb-2', gapTokens.compact)}>
          {placeholders.map((ph) => (
            <span
              key={ph}
              className="px-1.5 py-0.5 rounded text-[9px] font-mono"
              style={{
                background: `${colors.primary}20`,
                color: colors.primary,
                border: `1px solid ${colors.primary}40`,
              }}
            >
              {'{{' + ph + '}}'}
            </span>
          ))}
        </div>
      )}

      {/* Example */}
      {data.example && (
        <div className="mb-2">
          <span className="text-[8px] text-white/40 uppercase">Example:</span>
          <p className="text-[10px] text-white/60 italic">&quot;{data.example}&quot;</p>
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
