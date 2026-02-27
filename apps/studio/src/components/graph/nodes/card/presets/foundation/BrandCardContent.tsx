'use client';

/**
 * BrandCardContent - "Brand Atlas" design for Brand nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = foundation) → violet #8b5cf6
 * - Border style → solid (defined trait)
 * - Shows Brand soul/pitch/voice triad with trait badges
 *
 * Layout:
 * ┌────────────────────────────────┐
 * │ ◉ BRAND              ●active  │
 * │ ══════════════════════════    │
 * │ QRCode AI Brand               │
 * │ ┌──────────────────────────┐  │
 * │ │ SOUL     purpose/vision  │  │
 * │ │ PITCH    what + whom     │  │
 * │ │ VOICE    tone + formal   │  │
 * │ └──────────────────────────┘  │
 * │ ┌──────────────────────────┐  │
 * │ │ ● confidence  ● friendly │  │
 * │ │ ● professional           │  │
 * │ └──────────────────────────┘  │
 * └────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
// cn reserved for future use
// import { cn } from '@/lib/utils';
// gapTokens reserved for future use
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
// SPRING_CONFIGS reserved for future use
import { DURATIONS } from '../../animationPresets';
import { TraitBadge, SectionLabel } from './FoundationHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface BrandNodeData {
  id: string;
  type: 'Brand';
  key: string;
  displayName: string;
  /** Brand soul - purpose and vision */
  soul?: {
    purpose?: string;
    mission?: string;
    vision?: string;
  };
  /** Brand pitch - positioning */
  pitch?: {
    what?: string;
    for_whom?: string;
    how?: string;
    elevator_pitch?: string;
  };
  /** Brand voice - communication style */
  voice?: {
    tone?: string[];
    formality?: 'casual' | 'professional' | 'formal';
    humor?: 'none' | 'subtle' | 'playful';
    values?: string[];
  };
}

export interface BrandCardContentProps extends CardContext {
  data: BrandNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const sectionVariants: Variants = {
  idle: { opacity: 0.8, y: 0 },
  hover: { opacity: 1, y: -1, transition: { duration: DURATIONS.fast } },
  selected: { opacity: 1, y: 0 },
};

const badgeContainerVariants: Variants = {
  idle: { opacity: 0.7 },
  hover: { opacity: 1 },
  selected: {
    opacity: 1,
    transition: { staggerChildren: 0.05 },
  },
};

const badgeVariants: Variants = {
  idle: { scale: 1 },
  selected: {
    scale: [1, 1.1, 1],
    transition: { duration: 0.3 },
  },
};

// =============================================================================
// Component
// =============================================================================

export const BrandCardContent = memo(function BrandCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: BrandCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Extract voice traits for badges
  const voiceTraits = useMemo(() => {
    const traits: string[] = [];
    if (data.voice?.tone) traits.push(...data.voice.tone);
    if (data.voice?.values) traits.push(...data.voice.values);
    return traits.slice(0, 4); // Max 4 badges
  }, [data.voice]);

  // Glow style
  const glowStyle = useMemo(() => ({
    textShadow: selected
      ? `0 0 12px ${colors.primary}60`
      : isHovered
        ? `0 0 8px ${colors.primary}40`
        : 'none',
  }), [colors.primary, selected, isHovered]);

  const SectionWrapper = animationsEnabled ? motion.div : 'div';
  const BadgeContainer = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (foundation) + Realm (org) + Trait (defined) + Class (Brand) */}
      <div className="mb-3">
        <TaxonomyBadge
          layer="foundation"
          realm="org"
          trait="defined"
          className="Brand"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
      </div>

      {/* Brand name */}
      <h3
        className="text-lg font-bold text-white mb-3"
        style={glowStyle}
      >
        {data.displayName}
      </h3>

      {/* Soul / Pitch / Voice triad */}
      <SectionWrapper
        className="p-2 rounded-lg mb-3"
        style={{
          background: `${colors.primary}08`,
          border: `1px solid ${colors.primary}20`,
        }}
        {...(animationsEnabled && {
          variants: sectionVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        <div className="space-y-1.5">
          <div className="flex items-center justify-between">
            <SectionLabel icon="❤️" label="SOUL" sublabel="purpose/vision" color={colors.primary} />
          </div>
          <div className="flex items-center justify-between">
            <SectionLabel icon="🎯" label="PITCH" sublabel="what + whom" color={colors.primary} />
          </div>
          <div className="flex items-center justify-between">
            <SectionLabel icon="🗣️" label="VOICE" sublabel="tone + formal" color={colors.primary} />
          </div>
        </div>
      </SectionWrapper>

      {/* Voice traits as badges */}
      {voiceTraits.length > 0 && (
        <BadgeContainer
          className="p-2 rounded-lg"
          style={{
            background: `${colors.secondary}08`,
            border: `1px solid ${colors.secondary}20`,
          }}
          {...(animationsEnabled && {
            variants: badgeContainerVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          <div className="flex flex-wrap gap-1.5">
            {voiceTraits.map((trait, idx) => (
              animationsEnabled ? (
                <motion.span key={idx} variants={badgeVariants}>
                  <TraitBadge label={trait} color={colors.primary} />
                </motion.span>
              ) : (
                <TraitBadge key={idx} label={trait} color={colors.primary} />
              )
            ))}
          </div>
        </BadgeContainer>
      )}

      {/* Formality indicator */}
      {data.voice?.formality && (
        <div className="mt-2 flex items-center gap-2 text-[9px]">
          <span className="text-white/40">formality:</span>
          <span
            className="px-1.5 py-0.5 rounded"
            style={{
              color: colors.primary,
              backgroundColor: `${colors.primary}15`,
            }}
          >
            {data.voice.formality}
          </span>
        </div>
      )}
    </div>
  );
});
