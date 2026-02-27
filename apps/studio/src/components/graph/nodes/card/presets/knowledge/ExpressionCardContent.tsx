'use client';

/**
 * ExpressionCardContent - "Phrase Template" design for Expression nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> double (imported trait)
 * - Shows expression text, tone, formality, use case
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ ◇ EXPRESSION      ★ closing         │  <- Diamond icon + use case badge
 * │ ═════════════════════════════════   │  <- Double separator (imported)
 * │ closing_formal_fr                    │
 * │ ┌────────────────────────────────┐   │
 * │ │ "Veuillez agréer l'expression  │   │  <- The phrase text
 * │ │  de mes sentiments distingués" │   │
 * │ │ ─────────────────────────────  │   │
 * │ │ ◼ formal   [▓▓▓▓▓] formality   │   │  <- Tone + formality scale
 * │ │ ─────────────────────────────  │   │
 * │ │ channels: email, letter        │   │  <- Channel badges
 * │ └────────────────────────────────┘   │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
// cn reserved for future use
// import { cn } from '@/lib/utils';
// gapTokens reserved for future use
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import {
  ToneBadge,
  FormalityIndicator,
  UseCaseBadge,
  ChannelBadges,
} from './KnowledgeHelpers';
import { GlowEffect } from '../../effects';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface ExpressionNodeData {
  id: string;
  type: 'Expression';
  key: string;
  displayName: string;
  /** The actual expression/phrase */
  text: string;
  /** Description */
  description?: string;
  /** Emotional tone */
  tone: 'formal' | 'warm' | 'casual' | 'energetic' | 'empathetic' | 'authoritative' | 'friendly';
  /** Formality level */
  formality: 'very_formal' | 'formal' | 'neutral' | 'casual' | 'very_casual';
  /** Primary use case */
  use_case: 'greeting' | 'closing' | 'apology' | 'thanks' | 'request' | 'confirmation' | 'warning' | 'celebration';
  /** Appropriate channels */
  channel?: string[];
  /** Target audience */
  audience?: string[];
  /** Alternative expressions */
  alternatives?: string[];
}

export interface ExpressionCardContentProps extends CardContext {
  data: ExpressionNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const _diamondVariants: Variants = {
  idle: { scale: 1, rotate: 0 },
  hover: {
    scale: 1.1,
    rotate: 45,
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    scale: 1.15,
    rotate: 45,
    filter: 'drop-shadow(0 0 8px currentColor)',
  },
};

const textVariants: Variants = {
  idle: { opacity: 0.9 },
  hover: {
    opacity: 1,
    transition: { duration: DURATIONS.fast },
  },
  selected: { opacity: 1 },
};

// =============================================================================
// Component
// =============================================================================

export const ExpressionCardContent = memo(function ExpressionCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: ExpressionCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

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

  const _DiamondIcon = animationsEnabled ? motion.span : 'span';
  const TextWrapper = animationsEnabled ? motion.div : 'div';

  // Check if glow should be shown (MEDIUM+ tier)
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;

  return (
    <div className="relative px-4 py-4">
      {/* Premium glow effect (MEDIUM+ tier) */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={colors.primary}
          intensity={selected ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Taxonomy Badge: Layer (knowledge) + Realm (shared) + Trait (imported) + Class (Expression) */}
      <div className="mb-3">
        <TaxonomyBadge
          layer="knowledge"
          realm="shared"
          trait="imported"
          className="Expression"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
      </div>

      {/* Use case badge */}
      <div className="flex justify-end mb-2">
        <UseCaseBadge useCase={data.use_case} color={colors.primary} />
      </div>

      {/* Expression key */}
      <h3 className="text-xs font-mono text-white/60 mb-2 truncate">{data.key}</h3>

      {/* Expression text (the actual phrase) */}
      <TextWrapper
        className="mb-3 p-3 rounded-lg"
        style={{
          backgroundColor: `${colors.primary}10`,
          border: `1px solid ${colors.primary}30`,
        }}
        {...(animationsEnabled && {
          variants: textVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        <p
          className="text-sm font-medium text-white italic leading-relaxed"
          style={glowStyle}
        >
          &quot;{data.text}&quot;
        </p>
      </TextWrapper>

      {/* Tone + Formality */}
      <div className="flex items-center justify-between mb-3">
        <ToneBadge tone={data.tone} animated={animationsEnabled} />
        <FormalityIndicator formality={data.formality} color={colors.primary} />
      </div>

      {/* Description */}
      {data.description && (
        <p className="text-[10px] text-white/60 line-clamp-2 mb-2">
          {data.description}
        </p>
      )}

      {/* Channels */}
      {data.channel && data.channel.length > 0 && (
        <div className="mb-2">
          <ChannelBadges channels={data.channel} color={colors.primary} />
        </div>
      )}

      {/* Audience */}
      {data.audience && data.audience.length > 0 && (
        <div className="flex items-center gap-1 text-[8px]">
          <span className="text-white/50">audience:</span>
          {data.audience.slice(0, 3).map((seg, i) => (
            <span
              key={i}
              className="px-1 py-0.5 rounded"
              style={{
                backgroundColor: `${colors.primary}15`,
                color: `${colors.primary}cc`,
              }}
            >
              {seg}
            </span>
          ))}
        </div>
      )}

      {/* Alternatives count */}
      {data.alternatives && data.alternatives.length > 0 && (
        <div className="mt-2 flex items-center gap-1 text-[8px]">
          <span className="text-white/50">alternatives:</span>
          <span style={{ color: colors.primary }}>{data.alternatives.length}</span>
        </div>
      )}
    </div>
  );
});
