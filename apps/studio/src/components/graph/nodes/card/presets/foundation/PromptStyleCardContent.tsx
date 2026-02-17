'use client';

/**
 * PromptStyleCardContent - "AI Prompt Preset" card for PromptStyle nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = foundation) → violet #8b5cf6
 * - Border style → solid (defined trait)
 * - Shows AI platform badges and style attributes
 *
 * Layout:
 * ┌────────────────────────────────┐
 * │ ✨ PROMPT STYLE                │
 * │ ══════════════════════════    │
 * │ hero-illustration             │
 * │ ┌──────────────────────────┐  │
 * │ │ 🖼️ cinematic             │  │
 * │ │ 🎭 confident, minimal    │  │
 * │ │ 🌍 EA (Eastern Asia)     │  │
 * │ ├──────────────────────────┤  │
 * │ │ platforms: MJ, DALL-E    │  │
 * │ └──────────────────────────┘  │
 * └────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import { PlatformBadge, SectionLabel } from './FoundationHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface PromptStyleNodeData {
  id: string;
  type: 'PromptStyle';
  key: string;
  displayName: string;
  /** Visual style */
  style?: string;
  /** Subject description */
  subject?: string;
  /** Environment/setting */
  environment?: string;
  /** Lighting style */
  lighting?: string;
  /** Color palette keywords */
  color_palette?: string[];
  /** Composition style */
  composition?: string;
  /** Mood/atmosphere */
  mood?: string[];
  /** Quality settings */
  quality?: string[];
  /** Target platforms */
  platforms?: string[];
  /** Inspired by region */
  inspired_by_region?: string;
  /** For specific locale */
  for_locale?: string;
}

export interface PromptStyleCardContentProps extends CardContext {
  data: PromptStyleNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const attributeVariants: Variants = {
  idle: { opacity: 0.8 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

const platformContainerVariants: Variants = {
  idle: { opacity: 0.7 },
  hover: { opacity: 1 },
  selected: {
    opacity: 1,
    transition: { staggerChildren: 0.05 },
  },
};

const platformVariants: Variants = {
  idle: { scale: 1 },
  selected: {
    scale: [1, 1.15, 1],
    transition: { duration: 0.25 },
  },
};

// =============================================================================
// Component
// =============================================================================

export const PromptStyleCardContent = memo(function PromptStyleCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: PromptStyleCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Build attributes list
  const attributes = useMemo(() => {
    const attrs: Array<{ icon: string; label: string; value: string }> = [];

    if (data.style) {
      attrs.push({ icon: '🖼️', label: 'style', value: data.style });
    }
    if (data.mood && data.mood.length > 0) {
      attrs.push({ icon: '🎭', label: 'mood', value: data.mood.slice(0, 2).join(', ') });
    }
    if (data.inspired_by_region) {
      attrs.push({ icon: '🌍', label: 'region', value: data.inspired_by_region });
    }
    if (data.lighting) {
      attrs.push({ icon: '💡', label: 'lighting', value: data.lighting });
    }
    if (data.composition) {
      attrs.push({ icon: '📐', label: 'composition', value: data.composition });
    }

    return attrs.slice(0, 4); // Max 4 attributes
  }, [data]);

  // Glow style
  const glowStyle = useMemo(() => ({
    textShadow: selected
      ? `0 0 12px ${colors.primary}60`
      : isHovered
        ? `0 0 8px ${colors.primary}40`
        : 'none',
  }), [colors.primary, selected, isHovered]);

  const AttributeContainer = animationsEnabled ? motion.div : 'div';
  const PlatformContainer = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (foundation) + Realm (org) + Trait (defined) + Class (PromptStyle) */}
      <div className="mb-3">
        <TaxonomyBadge
          layer="foundation"
          realm="org"
          trait="defined"
          className="PromptStyle"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
      </div>

      {/* Style name */}
      <h3
        className="text-lg font-bold text-white mb-3"
        style={glowStyle}
      >
        {data.displayName}
      </h3>

      {/* Attributes section */}
      {attributes.length > 0 && (
        <AttributeContainer
          className="p-2 rounded-lg mb-2"
          style={{
            background: `${colors.primary}08`,
            border: `1px solid ${colors.primary}20`,
          }}
          {...(animationsEnabled && {
            variants: attributeVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          <div className="space-y-1.5">
            {attributes.map((attr, idx) => (
              <div key={idx} className="flex items-center gap-2 text-[10px]">
                <span>{attr.icon}</span>
                <span className="text-white/50 font-mono">{attr.label}:</span>
                <span className="text-white/80">{attr.value}</span>
              </div>
            ))}
          </div>
        </AttributeContainer>
      )}

      {/* Platforms section */}
      {data.platforms && data.platforms.length > 0 && (
        <PlatformContainer
          className="p-2 rounded-lg"
          style={{
            background: `${colors.secondary}08`,
            border: `1px solid ${colors.secondary}20`,
          }}
          {...(animationsEnabled && {
            variants: platformContainerVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          <div className="flex items-center gap-1.5 flex-wrap">
            <span className="text-[9px] text-white/50 font-mono mr-1">platforms:</span>
            {data.platforms.map((platform, idx) => (
              animationsEnabled ? (
                <motion.span key={idx} variants={platformVariants}>
                  <PlatformBadge platform={platform} active={true} />
                </motion.span>
              ) : (
                <PlatformBadge key={idx} platform={platform} active={true} />
              )
            ))}
          </div>
        </PlatformContainer>
      )}

      {/* Quality badges */}
      {data.quality && data.quality.length > 0 && (
        <div className="mt-2 flex flex-wrap gap-1">
          {data.quality.slice(0, 3).map((q, idx) => (
            <span
              key={idx}
              className="px-1.5 py-0.5 rounded text-[8px] font-mono"
              style={{
                color: colors.primary,
                backgroundColor: `${colors.primary}15`,
              }}
            >
              {q}
            </span>
          ))}
        </div>
      )}
    </div>
  );
});
