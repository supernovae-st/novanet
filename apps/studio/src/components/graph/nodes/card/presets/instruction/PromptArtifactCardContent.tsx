'use client';

/**
 * PromptArtifactCardContent - "Compiled Prompt" design for PromptArtifact nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = instruction) → yellow #eab308
 * - Border style → dotted
 * - Shows compiled prompt, token count, status
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ ⎔ ARTIFACT       ✓ Compiled     │  ← Hexagon icon + status badge
 * │ ══════════════════════════════   │
 * │ homepage:hero:prompt@fr-FR       │
 * │ ┌────────────────────────────┐   │
 * │ │ You are generating...      │   │  ← Prompt preview
 * │ │ Context: @entity:qr-gen... │   │
 * │ │ ... +12 more lines         │   │
 * │ ├────────────────────────────┤   │
 * │ │ tokens: 2,450 / 4,096      │   │  ← Token counter
 * │ │ ████████████░░░░░░ 60%     │   │  ← Progress bar
 * │ └────────────────────────────┘   │
 * └──────────────────────────────────┘
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
import { TokenCounter, ContentPreview, CompilationStatus } from './InstructionHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface PromptArtifactNodeData {
  id: string;
  type: 'PromptArtifact';
  key: string; // {page_key}:{block_key}:prompt@{locale}
  displayName: string;
  /** Locale key */
  locale_key?: string;
  /** Compiled prompt content */
  compiled_prompt?: string;
  /** Token count */
  token_count?: number;
  /** Max tokens for this model */
  max_tokens?: number;
  /** Compilation status */
  status?: 'pending' | 'compiled' | 'error';
  /** Compiled at timestamp */
  compiled_at?: string;
  /** Source instruction key */
  source_instruction?: string;
  /** Entities included */
  included_entities?: string[];
  /** Model target */
  target_model?: string;
}

export interface PromptArtifactCardContentProps extends CardContext {
  data: PromptArtifactNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const _hexVariants: Variants = {
  idle: { rotate: 0, scale: 1 },
  hover: {
    rotate: 30,
    scale: 1.1,
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    rotate: 30,
    scale: 1.15,
    filter: 'drop-shadow(0 0 8px currentColor)',
  },
};

const contentVariants: Variants = {
  idle: { opacity: 0.85 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Component
// =============================================================================

export const PromptArtifactCardContent = memo(function PromptArtifactCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: PromptArtifactCardContentProps) {
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

  const _HexIcon = animationsEnabled ? motion.span : 'span';
  const ContentWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (instruction) + Realm (org) + Class (PromptArtifact) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="instruction"
          realm="org"
          className="PromptArtifact"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        <CompilationStatus
          status={data.status || 'pending'}
          compiledAt={data.compiled_at}
          color={colors.primary}
        />
      </div>

      {/* Artifact key */}
      <h3 className="text-xs font-mono text-white/60 mb-1 truncate">{data.key}</h3>

      {/* Display name */}
      <h4 className="text-base font-bold text-white mb-3" style={glowStyle}>
        {data.displayName}
      </h4>

      {/* Content section */}
      <ContentWrapper
        className="space-y-3"
        {...(animationsEnabled && {
          variants: contentVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Locale badge */}
        {data.locale_key && (
          <div className="flex items-center gap-1 text-[9px]">
            <span className="text-white/50">locale:</span>
            <span
              className="px-1.5 py-0.5 rounded font-mono font-bold"
              style={{
                color: colors.primary,
                backgroundColor: `${colors.primary}15`,
              }}
            >
              {data.locale_key}
            </span>
          </div>
        )}

        {/* Prompt preview */}
        {data.compiled_prompt && (
          <ContentPreview
            content={data.compiled_prompt}
            maxLines={4}
            color={colors.primary}
          />
        )}

        {/* Token counter */}
        {typeof data.token_count === 'number' && (
          <TokenCounter
            tokenCount={data.token_count}
            maxTokens={data.max_tokens || 4096}
            color={colors.primary}
          />
        )}

        {/* Included entities */}
        {data.included_entities && data.included_entities.length > 0 && (
          <div className="flex items-center gap-1 flex-wrap text-[8px]">
            <span className="text-white/50">entities:</span>
            {data.included_entities.slice(0, 3).map((entity) => (
              <span
                key={entity}
                className="px-1 py-0.5 rounded font-mono"
                style={{
                  color: '#f97316',
                  backgroundColor: 'rgba(249, 115, 22, 0.15)',
                }}
              >
                @{entity}
              </span>
            ))}
            {data.included_entities.length > 3 && (
              <span className="text-white/40">+{data.included_entities.length - 3}</span>
            )}
          </div>
        )}

        {/* Target model */}
        {data.target_model && (
          <div className="flex items-center gap-1 text-[9px]">
            <span className="text-white/50">model:</span>
            <span className="text-white/70 font-mono">{data.target_model}</span>
          </div>
        )}
      </ContentWrapper>
    </div>
  );
});
