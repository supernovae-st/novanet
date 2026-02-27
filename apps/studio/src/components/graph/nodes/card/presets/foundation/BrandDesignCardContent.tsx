'use client';

/**
 * BrandDesignCardContent - "Design System" card for BrandDesign nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = foundation) → violet #8b5cf6
 * - Border style → solid (defined trait)
 * - Shows color swatches and typography preview
 *
 * Layout:
 * ┌────────────────────────────┐
 * │ 🎨 DESIGN                  │
 * │ ═══════════════════════    │
 * │ QRCode Design System       │
 * │ ┌────────────────────────┐ │
 * │ │ ●Primary   #6366f1     │ │ ← Color swatches
 * │ │ ●Secondary #8b5cf6     │ │
 * │ │ ●Accent    #06b6d4     │ │
 * │ ├────────────────────────┤ │
 * │ │ Aa Inter 16/24         │ │ ← Typography preview
 * │ │ Aa JetBrains Mono      │ │
 * │ └────────────────────────┘ │
 * └────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
// cn reserved for future use
// import { cn } from '@/lib/utils';
// gapTokens reserved for future use
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
// DURATIONS reserved for future use
// import { DURATIONS } from '../../animationPresets';
import { ColorSwatch, TypographyPreview } from './FoundationHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface BrandDesignNodeData {
  id: string;
  type: 'BrandDesign';
  key: string;
  displayName: string;
  /** Design philosophy / style mood */
  style_mood?: string[];
  style_keywords?: string[];
  /** Token colors */
  tokens?: {
    primitives?: {
      colors?: Record<string, string>;
    };
    semantic?: {
      colors?: {
        primary?: string;
        secondary?: string;
        accent?: string;
        success?: string;
        warning?: string;
        error?: string;
      };
    };
  };
  /** Typography settings */
  typography?: {
    heading?: { family?: string; size?: number; lineHeight?: number };
    body?: { family?: string; size?: number; lineHeight?: number };
    mono?: { family?: string; size?: number; lineHeight?: number };
  };
}

export interface BrandDesignCardContentProps extends CardContext {
  data: BrandDesignNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const swatchContainerVariants: Variants = {
  idle: { opacity: 0.9 },
  hover: { opacity: 1 },
  selected: {
    opacity: 1,
    transition: { staggerChildren: 0.05 },
  },
};

const swatchVariants: Variants = {
  idle: { x: 0 },
  selected: {
    x: [0, 2, 0],
    transition: { duration: 0.3 },
  },
};

// =============================================================================
// Component
// =============================================================================

export const BrandDesignCardContent = memo(function BrandDesignCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: BrandDesignCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Extract color swatches from tokens
  const colorSwatches = useMemo(() => {
    const semantic = data.tokens?.semantic?.colors;
    const swatches: Array<{ label: string; color: string }> = [];

    if (semantic?.primary) swatches.push({ label: 'Primary', color: semantic.primary });
    if (semantic?.secondary) swatches.push({ label: 'Secondary', color: semantic.secondary });
    if (semantic?.accent) swatches.push({ label: 'Accent', color: semantic.accent });
    if (semantic?.success) swatches.push({ label: 'Success', color: semantic.success });
    if (semantic?.warning) swatches.push({ label: 'Warning', color: semantic.warning });
    if (semantic?.error) swatches.push({ label: 'Error', color: semantic.error });

    return swatches.slice(0, 4); // Max 4 swatches
  }, [data.tokens]);

  // Typography entries
  const typographyEntries = useMemo(() => {
    const entries: Array<{ family: string; size: number; lineHeight: number }> = [];

    if (data.typography?.heading?.family) {
      entries.push({
        family: data.typography.heading.family,
        size: data.typography.heading.size || 24,
        lineHeight: data.typography.heading.lineHeight || 32,
      });
    }
    if (data.typography?.body?.family) {
      entries.push({
        family: data.typography.body.family,
        size: data.typography.body.size || 16,
        lineHeight: data.typography.body.lineHeight || 24,
      });
    }
    if (data.typography?.mono?.family) {
      entries.push({
        family: data.typography.mono.family,
        size: data.typography.mono.size || 14,
        lineHeight: data.typography.mono.lineHeight || 20,
      });
    }

    return entries;
  }, [data.typography]);

  // Glow style
  const glowStyle = useMemo(() => ({
    textShadow: selected
      ? `0 0 12px ${colors.primary}60`
      : isHovered
        ? `0 0 8px ${colors.primary}40`
        : 'none',
  }), [colors.primary, selected, isHovered]);

  const SwatchContainer = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (foundation) + Realm (org) + Trait (defined) + Class (BrandDesign) */}
      <div className="mb-3">
        <TaxonomyBadge
          layer="foundation"
          realm="org"
          trait="defined"
          className="BrandDesign"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
      </div>

      {/* Design system name */}
      <h3
        className="text-lg font-bold text-white mb-3"
        style={glowStyle}
      >
        {data.displayName}
      </h3>

      {/* Color swatches section */}
      {colorSwatches.length > 0 && (
        <SwatchContainer
          className="p-2 rounded-lg mb-2"
          style={{
            background: `${colors.primary}08`,
            border: `1px solid ${colors.primary}20`,
          }}
          {...(animationsEnabled && {
            variants: swatchContainerVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          <div className="space-y-1.5">
            {colorSwatches.map((swatch, idx) => (
              animationsEnabled ? (
                <motion.div key={idx} variants={swatchVariants}>
                  <ColorSwatch
                    color={swatch.color}
                    label={swatch.label}
                    animate={animationsEnabled}
                  />
                </motion.div>
              ) : (
                <ColorSwatch
                  key={idx}
                  color={swatch.color}
                  label={swatch.label}
                  animate={false}
                />
              )
            ))}
          </div>
        </SwatchContainer>
      )}

      {/* Typography section */}
      {typographyEntries.length > 0 && (
        <div
          className="p-2 rounded-lg"
          style={{
            background: `${colors.secondary}08`,
            border: `1px solid ${colors.secondary}20`,
          }}
        >
          <div className="space-y-1.5">
            {typographyEntries.map((entry, idx) => (
              <TypographyPreview
                key={idx}
                fontFamily={entry.family}
                fontSize={entry.size}
                lineHeight={entry.lineHeight}
              />
            ))}
          </div>
        </div>
      )}

      {/* Style keywords */}
      {data.style_keywords && data.style_keywords.length > 0 && (
        <div className="mt-2 flex flex-wrap gap-1">
          {data.style_keywords.slice(0, 3).map((keyword, idx) => (
            <span
              key={idx}
              className="px-1.5 py-0.5 rounded text-[8px] font-mono"
              style={{
                color: colors.primary,
                backgroundColor: `${colors.primary}15`,
              }}
            >
              {keyword}
            </span>
          ))}
        </div>
      )}
    </div>
  );
});
