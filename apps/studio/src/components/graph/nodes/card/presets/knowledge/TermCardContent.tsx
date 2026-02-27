'use client';

/**
 * TermCardContent - "Vocabulary Entry" design for Term nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> double (imported trait)
 * - Shows vocabulary term, domain, register, part of speech
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ ✎ TERM            pricing           │  <- Edit icon + domain badge
 * │ ═════════════════════════════════   │  <- Double separator (imported)
 * │ subscription_monthly                 │
 * │ ┌────────────────────────────────┐   │
 * │ │ "abonnement mensuel"           │   │  <- The actual term value
 * │ │ ─────────────────────────────  │   │
 * │ │ ▲ formal    noun               │   │  <- Register + part of speech
 * │ │ ─────────────────────────────  │   │
 * │ │ synonyms: formule mensuelle... │   │  <- Synonyms
 * │ └────────────────────────────────┘   │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
// import { cn } from '@/lib/utils';
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import {
  DomainBadge,
  RegisterBadge,
  PartOfSpeechBadge,
  SynonymsList,
} from './KnowledgeHelpers';
import { GlowEffect } from '../../effects';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface TermNodeData {
  id: string;
  type: 'Term';
  key: string;
  displayName: string;
  /** The actual term/phrase value */
  value: string;
  /** Description */
  description?: string;
  /** Vocabulary domain */
  domain: 'pricing' | 'features' | 'technical' | 'marketing' | 'support' | 'legal' | 'general';
  /** Language register */
  register?: 'formal' | 'neutral' | 'casual' | 'technical';
  /** Grammatical category */
  part_of_speech?: 'noun' | 'verb' | 'adjective' | 'adverb' | 'phrase';
  /** Alternative terms */
  synonyms?: string[];
  /** Contexts to avoid */
  avoid_in_context?: string[];
  /** Terms this should replace */
  prefer_over?: string[];
}

export interface TermCardContentProps extends CardContext {
  data: TermNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const _editVariants: Variants = {
  idle: { rotate: 0 },
  hover: {
    rotate: [-5, 5, 0],
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    rotate: 0,
    filter: 'drop-shadow(0 0 6px currentColor)',
  },
};

const valueVariants: Variants = {
  idle: { opacity: 0.9, y: 0 },
  hover: { opacity: 1, y: -1 },
  selected: { opacity: 1, y: 0 },
};

// =============================================================================
// Component
// =============================================================================

export const TermCardContent = memo(function TermCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: TermCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Glow style for the term value
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

  const _EditIcon = animationsEnabled ? motion.span : 'span';
  const ValueWrapper = animationsEnabled ? motion.div : 'div';

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

      {/* Taxonomy Badge: Layer (knowledge) + Realm (shared) + Trait (imported) + Class (Term) */}
      <div className="mb-3">
        <TaxonomyBadge
          layer="knowledge"
          realm="shared"
          trait="imported"
          className="Term"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
      </div>

      {/* Domain badge */}
      <div className="flex justify-end mb-2">
        <DomainBadge domain={data.domain} animated={animationsEnabled} />
      </div>

      {/* Term key */}
      <h3 className="text-xs font-mono text-white/60 mb-2 truncate">{data.key}</h3>

      {/* Term value (the actual phrase) */}
      <ValueWrapper
        className="mb-3 p-2 rounded-lg"
        style={{
          backgroundColor: `${colors.primary}10`,
          border: `1px solid ${colors.primary}30`,
        }}
        {...(animationsEnabled && {
          variants: valueVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        <p
          className="text-sm font-medium text-white italic"
          style={glowStyle}
        >
          &quot;{data.value}&quot;
        </p>
      </ValueWrapper>

      {/* Register + Part of Speech */}
      <div className="flex items-center gap-2 mb-2">
        {data.register && (
          <RegisterBadge register={data.register} color={colors.primary} />
        )}
        {data.part_of_speech && (
          <PartOfSpeechBadge partOfSpeech={data.part_of_speech} color={colors.primary} />
        )}
      </div>

      {/* Description */}
      {data.description && (
        <p className="text-[10px] text-white/60 line-clamp-2 mb-2 italic">
          {data.description}
        </p>
      )}

      {/* Synonyms */}
      {data.synonyms && data.synonyms.length > 0 && (
        <SynonymsList synonyms={data.synonyms} color={colors.primary} maxShow={3} />
      )}

      {/* Avoid/Prefer indicators */}
      {data.prefer_over && data.prefer_over.length > 0 && (
        <div className="mt-2 flex items-center gap-1 text-[8px]">
          <span className="text-white/50">replaces:</span>
          {data.prefer_over.slice(0, 2).map((term, i) => (
            <span
              key={i}
              className="px-1 py-0.5 rounded line-through"
              style={{ color: '#ef4444', backgroundColor: '#ef444420' }}
            >
              {term}
            </span>
          ))}
        </div>
      )}
    </div>
  );
});
