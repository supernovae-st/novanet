'use client';

/**
 * EntityCategoryCardContent - "Classification Nexus" Premium Design
 *
 * EntityCategory is the CLASSIFICATION SYSTEM for Entity nodes (ADR-017).
 * Each category (Thing, Content Type, Feature, Tool, etc.) classifies entities
 * via the [:BELONGS_TO] arc.
 *
 * Visual Design: LEFT-ALIGNED layout for better readability
 * - GridPattern background for "system/matrix" feel
 * - AuroraBackground + BorderBeam + HolographicOverlay premium effects
 * - Icon on left with pulsing halo effect
 * - Name + question + description aligned left
 *
 * Data format from Neo4j:
 * - key: "THING", "CONTENT_TYPE", "FEATURE" (uppercase, underscore-separated)
 * - display_name: "Thing", "Content Type", "Feature"
 * - description: "Core products and objects..."
 * - question: "WHAT?", "WHERE?", "WHO?", "HOW?"
 * - sort_order: 1, 2, 3...
 *
 * Visual Encoding (ADR-005):
 * - Fill → Layer (config = steel/gray)
 * - Border → Realm (shared = teal cyan)
 * - Effects → Premium (GridPattern + Aurora + BorderBeam)
 *
 * Layout (v0.13.1 - LEFT ALIGNED, 420x320):
 * ┌──────────────────────────────────────────────────────────────────────────┐
 * │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
 * │  ░░  ◇ CONFIG                                          ● shared  ░░░░  │
 * │  ░░                                                               ░░░░  │
 * │  ░░  ┌───────────┐                                               ░░░░  │
 * │  ░░  │    📦     │   Thing                                       ░░░░  │
 * │  ░░  │  (icon)   │   "WHAT?"                                     ░░░░  │
 * │  ░░  └───────────┘   Core products and objects...                ░░░░  │
 * │  ░░                  (up to 3 lines description)                 ░░░░  │
 * │  ░░  ────────────────────────────────────────────────────────    ░░░░  │
 * │  ░░  ◆ ENTITY_CATEGORY                         classifies →      ░░░░  │
 * │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
 * └──────────────────────────────────────────────────────────────────────────┘
 */

import React, { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import {
  Box,
  FileType,
  Sparkles,
  Wrench,
  Newspaper,
  Users,
  Building2,
  Briefcase,
  Target,
  Lightbulb,
  HelpCircle,
  Tag,
  Zap,
  MousePointer,
  type LucideIcon,
} from 'lucide-react';
// gapTokens reserved for future use
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { SPRING_CONFIGS } from '../../animationPresets';
import {
  LAYER_COLORS,
  REALM_COLORS,
  type LayerKey,
  type RealmKey,
} from '@/design/colors/generated';
import { MotionTechCorners } from '../../../effects';
import {
  GridPattern,
  AuroraBackground,
  BorderBeam,
  HolographicOverlay,
  GlowEffect,
} from '../../effects';

// =============================================================================
// Types
// =============================================================================

export interface EntityCategoryNodeData {
  id: string;
  type: string;
  /** Key in UPPERCASE_FORMAT: "THING", "CONTENT_TYPE", etc. */
  key: string;
  /** Human-readable name: "Thing", "Content Type" */
  displayName: string;
  /** Description of what this category contains */
  description?: string;
  /** Semantic question: "WHAT?", "WHERE?", "WHO?", "HOW?" */
  question?: string;
  /** Sort order for display */
  sortOrder?: number;
  /** Number of entities in this category */
  entityCount?: number;
}

export interface EntityCategoryTaxonomyProps {
  layer: 'config';
  realm: 'shared';
  trait: 'defined';
}

export interface EntityCategoryCardContentProps extends CardContext {
  data: EntityCategoryNodeData;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info */
  taxonomy?: EntityCategoryTaxonomyProps;
}

// =============================================================================
// Category Icons Mapping (Lucide icons based on actual keys)
// =============================================================================

const CATEGORY_ICONS: Record<string, LucideIcon> = {
  THING: Box,
  CONTENT_TYPE: FileType,
  FEATURE: Sparkles,
  TOOL: Wrench,
  MEDIUM: Newspaper,
  AUDIENCE: Users,
  INDUSTRY: Building2,
  USE_CASE: Briefcase,
  GOAL: Target,
  CONCEPT: Lightbulb,
  QUESTION: HelpCircle,
  ACTION: Zap,
  INTERACTION: MousePointer,
};

function getCategoryIcon(key: string): LucideIcon {
  return CATEGORY_ICONS[key.toUpperCase()] ?? Tag;
}

// Render category icon using createElement (avoids react-hooks/static-components rule)
function renderCategoryIcon(
  key: string,
  props: React.ComponentProps<LucideIcon>
): React.ReactElement {
  const IconComponent = getCategoryIcon(key);
  return React.createElement(IconComponent, props);
}

// =============================================================================
// Animation Variants
// =============================================================================

const cardVariants: Variants = {
  idle: { scale: 1, y: 0 },
  hover: { scale: 1.03, y: -4, transition: SPRING_CONFIGS.bouncy },
  selected: { scale: 1.05, y: -6, transition: SPRING_CONFIGS.smooth },
};

const iconVariants: Variants = {
  idle: {
    scale: 1,
    rotate: 0,
  },
  hover: {
    scale: 1.1,
    rotate: [0, -5, 5, 0],
    transition: { duration: 0.5 },
  },
  selected: {
    scale: 1.15,
    transition: SPRING_CONFIGS.bouncy,
  },
};

// =============================================================================
// Component - "Classification Nexus" Centered Layout
// =============================================================================

export const EntityCategoryCardContent = memo(function EntityCategoryCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: EntityCategoryCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showTechCorners = performanceConfig?.effects?.techCorners ?? true;
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;

  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Colors: config layer (gray/steel), shared realm (teal)
  const layerColor = LAYER_COLORS['config' as LayerKey]?.color ?? '#94a3b8';
  const realmColor = REALM_COLORS['shared' as RealmKey]?.color ?? '#2aa198';
  const primaryColor = colors.primary || realmColor;

  // Background style with deep gradient
  const backgroundStyle = useMemo(
    () => ({
      background: `
        linear-gradient(145deg,
          ${primaryColor}15 0%,
          rgba(12,12,18,0.97) 35%,
          rgba(8,8,14,0.99) 65%,
          ${realmColor}10 100%
        )
      `,
      borderWidth: 3,
      borderStyle: 'solid',
      borderColor: selected ? primaryColor : `${primaryColor}45`,
    }),
    [primaryColor, realmColor, selected]
  );

  // Shadow style
  const shadowStyle = useMemo(
    () => ({
      boxShadow: selected
        ? `
            0 0 40px ${primaryColor}35,
            0 0 80px ${primaryColor}18,
            0 20px 60px -20px rgba(0,0,0,0.7),
            inset 0 1px 0 rgba(255,255,255,0.1)
          `
        : isHovered
          ? `
              0 0 30px ${primaryColor}25,
              0 15px 50px -15px rgba(0,0,0,0.6),
              inset 0 1px 0 rgba(255,255,255,0.08)
            `
          : `
              0 0 20px ${primaryColor}15,
              0 10px 40px -10px rgba(0,0,0,0.5),
              inset 0 1px 0 rgba(255,255,255,0.05)
            `,
    }),
    [primaryColor, selected, isHovered]
  );

  // Icon container style
  const iconContainerStyle = useMemo(
    () => ({
      background: `
        radial-gradient(circle at center, ${primaryColor}30 0%, ${primaryColor}10 50%, transparent 70%),
        linear-gradient(135deg, ${primaryColor}20 0%, transparent 50%)
      `,
      border: `2px solid ${primaryColor}40`,
      boxShadow: selected
        ? `0 0 40px ${primaryColor}50, inset 0 0 20px ${primaryColor}20`
        : isHovered
          ? `0 0 30px ${primaryColor}35, inset 0 0 15px ${primaryColor}15`
          : `0 0 20px ${primaryColor}25`,
    }),
    [primaryColor, selected, isHovered]
  );

  // Grid pattern squares based on category
  const gridSquares = useMemo((): [number, number][] => {
    // Different pattern for each category to make them unique
    const hash = data.key.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
    const baseSquares: [number, number][] = [
      [(hash % 5) + 1, (hash % 4) + 1],
      [(hash % 6) + 2, (hash % 5) + 2],
      [(hash % 4) + 4, (hash % 3) + 1],
      [(hash % 5) + 1, (hash % 4) + 4],
      [(hash % 3) + 6, (hash % 5) + 3],
      [(hash % 4) + 3, (hash % 6) + 5],
    ];
    return baseSquares;
  }, [data.key]);

  const CardWrapper = animationsEnabled ? motion.div : 'div';
  const IconWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <CardWrapper
      className="relative p-6 min-h-[280px] w-full rounded-2xl overflow-hidden flex flex-col"
      style={{
        ...backgroundStyle,
        ...shadowStyle,
      }}
      {...(animationsEnabled && {
        variants: cardVariants,
        initial: 'idle',
        animate: animationState,
      })}
    >
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* PREMIUM EFFECTS LAYER                                                   */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}

      {/* Grid pattern background - "Matrix" feel */}
      <GridPattern
        width={24}
        height={24}
        color={primaryColor}
        opacity={selected ? 0.25 : isHovered ? 0.18 : 0.12}
        squares={gridSquares}
        squareColor={primaryColor}
        flicker={animationsEnabled}
        selected={selected}
        isHovered={isHovered}
        performanceConfig={performanceConfig}
        className="rounded-2xl"
      />

      {/* Aurora background */}
      {showPremiumEffects && animationsEnabled && (
        <AuroraBackground
          primaryColor={primaryColor}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          intensity={selected ? 'medium' : 'subtle'}
          borderRadius={16}
        />
      )}

      {/* Border beam */}
      {showPremiumEffects && animationsEnabled && (selected || isHovered) && (
        <BorderBeam
          color={primaryColor}
          secondaryColor={realmColor}
          borderRadius={16}
          thickness={2}
          duration={selected ? 5 : 8}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          beamLength={0.15}
        />
      )}

      {/* Holographic overlay */}
      {showPremiumEffects && animationsEnabled && selected && (
        <HolographicOverlay
          baseColor={primaryColor}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          intensity="subtle"
          borderRadius={16}
        />
      )}

      {/* Outer glow */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={primaryColor}
          intensity={selected ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Tech corners */}
      {showTechCorners && (
        <MotionTechCorners
          color={primaryColor}
          selected={selected}
          isHovered={isHovered}
          size={12}
          performanceConfig={performanceConfig}
        />
      )}

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* HEADER: Layer + Realm badges                                            */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div className="relative z-10 flex items-center justify-between mb-4">
        <div
          className={cn(
            'inline-flex items-center gap-1.5 px-2 py-1 rounded-md',
            'text-[10px] font-bold uppercase tracking-wider'
          )}
          style={{
            background: `${layerColor}20`,
            color: layerColor,
            border: `1px solid ${layerColor}35`,
          }}
        >
          <span
            className="w-2 h-2 rounded-sm"
            style={{ background: layerColor, boxShadow: `0 0 6px ${layerColor}` }}
          />
          config
        </div>

        <div
          className={cn(
            'inline-flex items-center gap-1.5 px-2 py-1 rounded-full',
            'text-[10px] font-bold uppercase tracking-wider'
          )}
          style={{
            background: `${realmColor}18`,
            color: realmColor,
            border: `1px solid ${realmColor}35`,
          }}
        >
          <span
            className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
            style={{ background: realmColor, boxShadow: `0 0 6px ${realmColor}` }}
          />
          shared
        </div>
      </div>

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* MAIN CONTENT: Left-aligned icon + text layout                          */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div className="relative z-10 flex items-start gap-4 mb-4">
        {/* Icon with pulsing halo */}
        <IconWrapper
          className="relative p-4 rounded-xl flex-shrink-0"
          style={iconContainerStyle}
          {...(animationsEnabled && {
            variants: iconVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {/* Pulsing halo behind icon */}
          {animationsEnabled && (selected || isHovered) && (
            <motion.div
              className="absolute inset-0 rounded-xl"
              style={{
                background: `radial-gradient(circle, ${primaryColor}40 0%, transparent 70%)`,
              }}
              animate={{
                scale: [1, 1.2, 1],
                opacity: [0.5, 0.8, 0.5],
              }}
              transition={{
                duration: 2,
                repeat: Infinity,
                ease: 'easeInOut',
              }}
            />
          )}
          {renderCategoryIcon(data.key, {
            size: 40,
            strokeWidth: 1.5,
            style: {
              color: primaryColor,
              filter: selected
                ? `drop-shadow(0 0 20px ${primaryColor}) drop-shadow(0 0 40px ${primaryColor}80)`
                : isHovered
                  ? `drop-shadow(0 0 15px ${primaryColor})`
                  : `drop-shadow(0 0 10px ${primaryColor}80)`,
            },
          })}
        </IconWrapper>

        {/* Text content aligned left */}
        <div className="flex-1 min-w-0">
          {/* Category name */}
          <h2
            className="text-xl font-bold text-white mb-1"
            style={{
              textShadow: selected
                ? `0 0 30px ${primaryColor}70, 0 0 60px ${primaryColor}40`
                : isHovered
                  ? `0 0 20px ${primaryColor}50`
                  : `0 0 10px ${primaryColor}30`,
            }}
          >
            {data.displayName}
          </h2>

          {/* Question badge inline */}
          {data.question && (
            <span
              className="inline-block px-2.5 py-0.5 rounded-full text-xs font-bold mb-2"
              style={{
                background: `linear-gradient(135deg, ${primaryColor}25 0%, ${primaryColor}15 100%)`,
                color: primaryColor,
                border: `1px solid ${primaryColor}40`,
              }}
            >
              {data.question}
            </span>
          )}

          {/* Description */}
          {data.description && (
            <p className="text-sm text-white/55 line-clamp-3">
              {data.description}
            </p>
          )}
        </div>
      </div>

      {/* Spacer */}
      <div className="flex-1" />

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* DIVIDER                                                                 */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div
        className="relative z-10 h-px mb-3"
        style={{
          background: `linear-gradient(90deg, transparent, ${primaryColor}50, transparent)`,
          boxShadow: `0 0 10px ${primaryColor}20`,
        }}
      />

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* FOOTER: Type badge + classifies indicator                               */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div className="relative z-10 flex items-center justify-between">
        <div
          className={cn(
            'inline-flex items-center gap-2 px-3 py-1.5 rounded-lg',
            'text-[10px] font-bold uppercase tracking-wider'
          )}
          style={{
            background: `${primaryColor}15`,
            border: `1px solid ${primaryColor}30`,
            color: primaryColor,
          }}
        >
          <span
            className="w-2 h-2 rounded-sm rotate-45"
            style={{ background: primaryColor, boxShadow: `0 0 6px ${primaryColor}` }}
          />
          entity_category
        </div>

        <span
          className="text-sm font-medium"
          style={{ color: `${primaryColor}` }}
        >
          classifies →
        </span>
      </div>
    </CardWrapper>
  );
});
