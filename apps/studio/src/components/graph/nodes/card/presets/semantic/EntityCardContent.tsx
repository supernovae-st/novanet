'use client';

/**
 * EntityCardContent - Premium "Semantic Diamond" design for Entity nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = semantic) → orange #f97316
 * - Border style → solid
 * - Shows pillar status, schema.org type, semantic links
 *
 * Design Philosophy (v0.13.1):
 * - PARENT nodes (Entity) are BIGGER, more prominent than NATIVE children
 * - Premium effects: Neon glow, Mouse spotlight, Border beam shine
 * - Full description (no truncation, line wrapping)
 * - Triple diamond ◆◆◆ icon with solid border
 * - Natives preview with locale flags
 *
 * Layout:
 * ┌─────────────────────────────────────────────────┐
 * │ ⊕ SEMANTIC │ ORG  ■ DEFINED   ★ PILLAR        │  ← TaxonomyBadge + PillarBadge
 * ├─────────────────────────────────────────────────┤
 * │ ┌─────────┐                                     │
 * │ │  ◆◆◆    │  qr-code-generator                 │  ← Triple diamond + key
 * │ │ ENTITY  │  QR Code Generator                 │  ← Display name (big)
 * │ └─────────┘                                     │
 * │                                                 │
 * │ Generate QR codes for WiFi network sharing.     │  ← Full description
 * │ Supports WPA2, WPA3, and open networks.         │     (no truncation!)
 * │                                                 │
 * │ ┌─────────────────────────────────────────────┐ │
 * │ │ schema:SoftwareApplication                  │ │  ← Schema.org type
 * │ │ category: tool                              │ │  ← Category
 * │ │ → 5  │  ← 12                                │ │  ← Semantic links
 * │ └─────────────────────────────────────────────┘ │
 * │                                                 │
 * │ 🇫🇷 🇺🇸 🇩🇪 +5 natives                          │  ← Locale preview
 * └─────────────────────────────────────────────────┘
 *    ↑ Neon glow + Border beam shine effect
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import { PillarBadge, SchemaOrgBadge, SemanticLinkCounter, toNumber, getLocaleFlag } from './SemanticHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { BorderBeam, GridPattern } from '../../effects';
import { Diamond, Layers, Globe } from 'lucide-react';

// =============================================================================
// Types
// =============================================================================

export interface EntityNodeData {
  id: string;
  type: 'Entity';
  key: string;
  displayName: string;
  /** Entity description */
  description?: string;
  /** Is this a pillar entity? */
  is_pillar?: boolean;
  /** Schema.org type for SEO */
  schema_org_type?: string;
  /** Entity category key */
  category_key?: string;
  /** Semantic link counts */
  semanticLinks?: {
    incoming?: number;
    outgoing?: number;
  };
  /** Content locales count */
  localeCount?: number;
  /** Number of EntityNative children */
  nativeCount?: number;
  /** Preview of native locales (first N locale codes) */
  nativeLocales?: string[];
}

export interface EntityCardContentProps extends CardContext {
  data: EntityNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const cardVariants: Variants = {
  idle: { scale: 1 },
  hover: { scale: 1.02, transition: { duration: DURATIONS.normal } },
  selected: { scale: 1.03, transition: { duration: DURATIONS.normal } },
};

const tripleIconVariants: Variants = {
  idle: { scale: 1, rotate: 0 },
  hover: {
    scale: 1.08,
    rotate: 5,
    transition: { duration: DURATIONS.normal, ease: 'easeOut' },
  },
  selected: {
    scale: 1.12,
    rotate: 10,
    transition: { duration: DURATIONS.normal },
  },
};

const infoVariants: Variants = {
  idle: { opacity: 0.9 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

const nativesPreviewVariants: Variants = {
  idle: { opacity: 0.7 },
  hover: { opacity: 0.9 },
  selected: { opacity: 1 },
};

// =============================================================================
// Component
// =============================================================================

export const EntityCardContent = memo(function EntityCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: EntityCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Glow style for title
  const titleGlowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 20px ${colors.primary}80, 0 0 40px ${colors.primary}40`
        : isHovered
          ? `0 0 12px ${colors.primary}50`
          : 'none',
    }),
    [colors.primary, selected, isHovered]
  );

  const CardWrapper = animationsEnabled ? motion.div : 'div';
  const IconWrapper = animationsEnabled ? motion.div : 'div';
  const InfoWrapper = animationsEnabled ? motion.div : 'div';
  const NativesWrapper = animationsEnabled ? motion.div : 'div';

  // Convert Neo4j integers to JS numbers
  const nativeCount = toNumber(data.nativeCount);
  const localeCount = toNumber(data.localeCount);
  const hasNatives = nativeCount > 0;

  // Show premium effects only on MEDIUM+ tier
  const performanceTier = performanceConfig?.tier ?? 'MEDIUM';
  const showPremiumEffects = performanceTier !== 'LOW' && performanceTier !== 'MINIMAL';

  return (
    <CardWrapper
      className="relative px-5 py-5"
      {...(animationsEnabled && {
        variants: cardVariants,
        initial: 'idle',
        animate: animationState,
      })}
    >
      {/* === GRID PATTERN BACKGROUND (static for performance) === */}
      {showPremiumEffects && (
        <GridPattern
          color={colors.primary}
          opacity={selected ? 0.2 : isHovered ? 0.15 : 0.1}
          width={24}
          height={24}
          flicker={false}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-xl"
        />
      )}

      {/* === BORDER BEAM SHINE EFFECT (only on selected for perf) === */}
      {showPremiumEffects && animationsEnabled && selected && (
        <BorderBeam
          color={colors.primary}
          borderRadius={16}
          thickness={3}
          duration={4}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          beamLength={0.2}
        />
      )}

      {/* Row 1: Taxonomy + ENTITY badge + Pillar */}
      <div className="flex items-center justify-between mb-4">
        <TaxonomyBadge
          layer="semantic"
          realm="org"
          className="Entity"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        <div className="flex items-center gap-2">
          {/* ENTITY badge (top right) */}
          <div
            className="flex items-center gap-1.5 px-2 py-1 rounded-md"
            style={{
              background: `${colors.primary}20`,
              border: `1px solid ${colors.primary}50`,
            }}
          >
            <Diamond size={12} style={{ color: colors.primary }} fill={colors.primary} />
            <span className="text-[10px] font-bold" style={{ color: colors.primary }}>
              ENTITY
            </span>
          </div>
          <PillarBadge isPillar={data.is_pillar || false} color={colors.primary} />
        </div>
      </div>

      {/* Row 2: INVARIANT badge with globe - CENTERED */}
      <div
        className="flex items-center justify-center gap-2 mb-4 px-3 py-2 rounded-lg"
        style={{
          background: `linear-gradient(135deg, ${colors.primary}20, ${colors.primary}08)`,
          border: `1.5px solid ${colors.primary}40`,
          boxShadow: selected ? `0 0 20px ${colors.primary}30` : 'none',
        }}
      >
        <Globe size={20} style={{ color: colors.primary }} />
        <span
          className="text-lg font-bold font-mono"
          style={{ color: colors.primary }}
        >
          INVARIANT
        </span>
      </div>

      {/* Row 3: Key in styled block */}
      <div
        className="flex items-center justify-center mb-4 px-4 py-2 rounded-lg"
        style={{
          background: `linear-gradient(135deg, ${colors.primary}15, ${colors.primary}05)`,
          border: `1.5px solid ${colors.primary}30`,
        }}
      >
        <span
          className="text-sm font-mono font-bold"
          style={{ color: colors.primary }}
        >
          {data.key}
        </span>
      </div>

      {/* Row 4: Icon + DisplayName (horizontal layout) */}
      <div className="flex items-center gap-4 mb-4">
        {/* Single filled Diamond icon — SOLID border */}
        <IconWrapper
          className="flex-shrink-0 w-12 h-12 rounded-lg flex items-center justify-center"
          style={{
            background: `linear-gradient(135deg, ${colors.primary}30, ${colors.primary}10)`,
            border: `3px solid ${colors.primary}70`,
            boxShadow: selected ? `0 0 20px ${colors.primary}40` : 'none',
            transition: 'box-shadow 0.2s ease-out',
          }}
          {...(animationsEnabled && {
            variants: tripleIconVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {/* Single filled diamond ◆ */}
          <Diamond size={24} style={{ color: colors.primary }} fill={colors.primary} />
        </IconWrapper>

        {/* displayName */}
        <h4
          className="text-xl font-bold text-white leading-tight"
          style={titleGlowStyle}
        >
          {data.displayName}
        </h4>
      </div>

      {/* Row 5: Description in styled block */}
      {data.description && (
        <div
          className="p-3 rounded-lg mb-4"
          style={{
            background: `${colors.primary}06`,
            border: `1px solid ${colors.primary}20`,
          }}
        >
          <p
            className="text-sm text-white/70 leading-relaxed"
            style={{
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-word',
            }}
          >
            {data.description}
          </p>
        </div>
      )}

      {/* Row 4: Info section — metadata badges (only if there's content) */}
      {(data.schema_org_type || data.category_key || data.semanticLinks || localeCount > 0) && (
        <InfoWrapper
          className="p-3 rounded-lg space-y-2"
          style={{
            background: `${colors.primary}08`,
            border: `1.5px solid ${colors.primary}30`,
          }}
          {...(animationsEnabled && {
            variants: infoVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {/* Schema.org type */}
          {data.schema_org_type && (
            <div className="flex items-center gap-2">
              <SchemaOrgBadge type={data.schema_org_type} color={colors.primary} />
            </div>
          )}

          {/* Category if present */}
          {data.category_key && (
            <div className="flex items-center gap-1.5 text-[10px]">
              <span className="text-white/50">category:</span>
              <span
                className="px-2 py-0.5 rounded font-mono font-medium"
                style={{
                  color: colors.primary,
                  backgroundColor: `${colors.primary}20`,
                  border: `1px solid ${colors.primary}30`,
                }}
              >
                {data.category_key}
              </span>
            </div>
          )}

          {/* Semantic link counts */}
          {data.semanticLinks && (
            <SemanticLinkCounter
              incoming={toNumber(data.semanticLinks.incoming)}
              outgoing={toNumber(data.semanticLinks.outgoing)}
              color={colors.primary}
            />
          )}

          {/* Locale count */}
          {localeCount > 0 && (
            <div className="flex items-center gap-1.5 text-[10px]">
              <Globe size={12} className="text-white/50" />
              <span className="text-white/70 font-medium">{localeCount} locales</span>
            </div>
          )}
        </InfoWrapper>
      )}

      {/* Row 6: Natives list with ALL flags */}
      {hasNatives && (
        <NativesWrapper
          className="mt-4 p-3 rounded-lg"
          style={{
            background: `linear-gradient(135deg, #22c55e12, #22c55e05)`,
            border: `1.5px dashed #22c55e40`,
          }}
          {...(animationsEnabled && {
            variants: nativesPreviewVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {/* Header */}
          <div className="flex items-center gap-2 mb-2">
            <Layers size={14} className="text-green-500" />
            <span className="text-xs font-bold text-green-400">
              {nativeCount} NATIVE{nativeCount > 1 ? 'S' : ''}
            </span>
          </div>

          {/* All locale flags in a wrap */}
          <div className="flex flex-wrap items-center gap-1">
            {(data.nativeLocales || []).map((locale) => (
              <span
                key={locale}
                className="text-lg px-1 py-0.5 rounded hover:bg-green-500/20 transition-colors cursor-pointer"
                title={locale}
              >
                {getLocaleFlag(locale)}
              </span>
            ))}
            {/* Show count if no locales data but has count */}
            {(!data.nativeLocales || data.nativeLocales.length === 0) && nativeCount > 0 && (
              <span className="text-xs font-mono text-green-400/70">
                ({nativeCount} locales)
              </span>
            )}
          </div>
        </NativesWrapper>
      )}
    </CardWrapper>
  );
});
