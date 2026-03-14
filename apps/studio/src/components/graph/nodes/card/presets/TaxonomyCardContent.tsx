'use client';

/**
 * TaxonomyCardContent - Premium design for TAXONOMY level nodes (v0.13.1)
 *
 * Handles 3 taxonomy variants:
 * - Realm: shared, org (2)
 * - Layer: 10 layers across 2 realms
 * - ArcFamily: ownership, localization, semantic, generation, mining (5)
 *
 * Visual Encoding (3-Level Architecture):
 * - TAXONOMY nodes get the most prominent visual treatment
 * - Heavy border (4px), floating shadow, triple glow, always-animated
 * - Banner badge at top: "TAXONOMY"
 * - Large icons (24px), xl typography
 *
 * Layout:
 * ┌──────────────────────────────────────────────────────────────┐
 * │              ╔════════════════════╗                          │
 * │              ║     TAXONOMY       ║ ← Banner badge            │
 * │              ╚════════════════════╝                          │
 * │                                                              │
 * │   ┌─────────────────────────────────────────────────────┐   │
 * │   │                                                     │   │
 * │   │             ◈  REALM                                │   │
 * │   │                                                     │   │
 * │   │         "shared"                                    │   │
 * │   │                                                     │   │
 * │   │   ┌─────────────────────────────────────────────┐  │   │
 * │   │   │  40 NodeClasses  │  4 Layers               │  │   │
 * │   │   └─────────────────────────────────────────────┘  │   │
 * │   │                                                     │   │
 * │   │   Universal knowledge graph. Read-only. 4 layers.   │   │
 * │   │                                                     │   │
 * │   └─────────────────────────────────────────────────────┘   │
 * │                                                              │
 * └──────────────────────────────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import {
  Globe,
  Layers,
  GitBranch,
  type LucideIcon,
} from 'lucide-react';
import type { CardContext } from '../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { SPRING_CONFIGS } from '../animationPresets';
import {
  REALM_COLORS,
  LAYER_COLORS,
  ARC_FAMILY_COLORS,
  type RealmKey,
  type LayerKey,
  type ArcFamilyKey,
} from '@/design/colors';
import { MotionTechCorners } from '../../effects';
import {
  AuroraBackground,
  BorderBeam,
  HolographicOverlay,
} from '../effects';

// =============================================================================
// Types
// =============================================================================

export type TaxonomyVariant = 'realm' | 'layer' | 'arcFamily';

export interface TaxonomyNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  /** Taxonomy variant */
  variant: TaxonomyVariant;
  /** Color for this taxonomy node */
  color?: string;
  /** Count of child nodes/arcs */
  count?: number;
  /** Description */
  description?: string;
  // Variant-specific data
  /** For Layer: which realm */
  realmKey?: RealmKey;
  /** For Realm */
  layerCount?: number;
  /** Node class count */
  nodeClassCount?: number;
  /** Arc class count */
  arcClassCount?: number;
}

export interface TaxonomyCardContentProps extends CardContext {
  data: TaxonomyNodeData;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Constants
// =============================================================================

const VARIANT_ICONS: Record<TaxonomyVariant, LucideIcon> = {
  realm: Globe,
  layer: Layers,
  arcFamily: GitBranch,
};

const VARIANT_LABELS: Record<TaxonomyVariant, string> = {
  realm: 'REALM',
  layer: 'LAYER',
  arcFamily: 'ARC FAMILY',
};

// =============================================================================
// Animation Variants (TAXONOMY = always animated)
// =============================================================================

const cardVariants: Variants = {
  idle: {
    scale: 1,
    y: 0,
  },
  hover: {
    scale: 1.04,
    y: -4,
    transition: SPRING_CONFIGS.bouncy,
  },
  selected: {
    scale: 1.06,
    y: -6,
    transition: SPRING_CONFIGS.smooth,
  },
};

// Subtle breathing animation for taxonomy nodes
const glowVariants: Variants = {
  idle: {
    boxShadow: [
      '0 0 20px rgba(139, 92, 246, 0.15)',
      '0 0 40px rgba(139, 92, 246, 0.25)',
      '0 0 20px rgba(139, 92, 246, 0.15)',
    ],
    transition: {
      duration: 3,
      repeat: Infinity,
      ease: 'easeInOut',
    },
  },
};

// =============================================================================
// Helper Functions
// =============================================================================

function getVariantColor(data: TaxonomyNodeData): string {
  if (data.color) return data.color;

  switch (data.variant) {
    case 'realm':
      return REALM_COLORS[data.key as RealmKey]?.color ?? '#8b5cf6';
    case 'layer':
      return LAYER_COLORS[data.key as LayerKey]?.color ?? '#6366f1';
    case 'arcFamily':
      return ARC_FAMILY_COLORS[data.key as ArcFamilyKey]?.color ?? '#f59e0b';
    default:
      return '#8b5cf6';
  }
}

function _formatCount(count: number | undefined, singular: string, plural: string): string {
  if (count === undefined) return '';
  return `${count} ${count === 1 ? singular : plural}`;
}

// =============================================================================
// Main Component
// =============================================================================

export const TaxonomyCardContent = memo(function TaxonomyCardContent({
  data,
  colors: _colors,
  selected,
  isHovered,
  performanceConfig,
}: TaxonomyCardContentProps) {
  // Performance flags
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const showTechCorners = performanceConfig?.effects?.techCorners ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;

  const variantColor = useMemo(() => getVariantColor(data), [data]);
  const Icon = VARIANT_ICONS[data.variant];
  const variantLabel = VARIANT_LABELS[data.variant];
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Stats based on variant
  const stats = useMemo(() => {
    switch (data.variant) {
      case 'realm':
        return [
          { label: 'Layers', value: data.layerCount ?? 0 },
          { label: 'NodeClasses', value: data.nodeClassCount ?? 0 },
        ];
      case 'layer':
        return [
          { label: 'NodeClasses', value: data.nodeClassCount ?? 0 },
          { label: 'Realm', value: data.realmKey ?? '—' },
        ];
      case 'arcFamily':
        return [
          { label: 'ArcClasses', value: data.arcClassCount ?? 0 },
        ];
      default:
        return [];
    }
  }, [data]);

  // Wrapper component based on animation state
  const CardWrapper = animationsEnabled ? motion.div : 'div';

  // Background style with prominent gradient
  const backgroundStyle = useMemo(() => ({
    background: `
      linear-gradient(135deg, ${variantColor}20 0%, rgba(0,0,0,0.95) 40%, ${variantColor}10 100%),
      radial-gradient(ellipse at top right, ${variantColor}15 0%, transparent 50%)
    `,
    borderWidth: 4,
    borderStyle: 'solid',
    borderColor: selected ? variantColor : `${variantColor}60`,
  }), [variantColor, selected]);

  return (
    <CardWrapper
      className="relative p-5 min-h-[180px] rounded-2xl overflow-hidden"
      style={backgroundStyle}
      {...(animationsEnabled && {
        variants: cardVariants,
        initial: 'idle',
        animate: animationState,
      })}
    >
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* PREMIUM EFFECTS LAYER (TAXONOMY = Maximum Wow)                          */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}

      {/* Aurora northern lights background effect */}
      {showPremiumEffects && animationsEnabled && (
        <AuroraBackground
          primaryColor={variantColor}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          intensity={selected ? 'intense' : isHovered ? 'medium' : 'subtle'}
          borderRadius={16}
        />
      )}

      {/* Rotating border beam spotlight */}
      {showPremiumEffects && animationsEnabled && (
        <BorderBeam
          color={variantColor}
          borderRadius={16}
          thickness={3}
          duration={selected ? 4 : 8}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          beamLength={0.2}
        />
      )}

      {/* Holographic prismatic overlay */}
      {showPremiumEffects && animationsEnabled && (
        <HolographicOverlay
          baseColor={variantColor}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          intensity={selected ? 'intense' : 'medium'}
          borderRadius={16}
        />
      )}

      {/* Taxonomy banner badge */}
      <div
        className="absolute -top-0.5 left-1/2 -translate-x-1/2 px-4 py-1.5 rounded-b-lg z-20"
        style={{
          background: `linear-gradient(135deg, ${variantColor} 0%, ${variantColor}cc 100%)`,
          boxShadow: `0 4px 12px ${variantColor}40`,
        }}
      >
        <span className="text-[10px] font-bold text-white tracking-[0.2em]">
          TAXONOMY
        </span>
      </div>

      {/* Triple glow rings (TAXONOMY signature effect) - enhanced with pulse */}
      {animationsEnabled && (
        <motion.div
          className="absolute inset-0 pointer-events-none"
          style={{
            borderRadius: 16,
            boxShadow: `
              0 0 30px ${variantColor}25,
              0 0 60px ${variantColor}18,
              0 0 90px ${variantColor}12,
              inset 0 0 30px ${variantColor}08
            `,
          }}
          variants={glowVariants}
          animate="idle"
        />
      )}

      {/* Tech corners with level-appropriate size */}
      {showTechCorners && (
        <MotionTechCorners
          color={variantColor}
          selected={selected}
          isHovered={isHovered}
          size={14}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Content */}
      <div className="relative z-10 mt-6">
        {/* Variant icon + label */}
        <div className={cn('flex items-center justify-center mb-4', gapTokens.default)}>
          <div
            className="p-3 rounded-xl"
            style={{
              background: `linear-gradient(135deg, ${variantColor}30 0%, ${variantColor}15 100%)`,
              border: `2px solid ${variantColor}40`,
              boxShadow: `0 0 20px ${variantColor}30`,
            }}
          >
            <Icon
              size={28}
              className="transition-transform duration-300"
              style={{
                color: variantColor,
                filter: `drop-shadow(0 0 8px ${variantColor}80)`,
                transform: (selected || isHovered) ? 'scale(1.1)' : 'scale(1)',
              }}
            />
          </div>
        </div>

        {/* Variant type label */}
        <div className="text-center mb-2">
          <span
            className="text-xs font-semibold uppercase tracking-widest"
            style={{ color: variantColor }}
          >
            {variantLabel}
          </span>
        </div>

        {/* Display name (hero) */}
        <h2
          className="text-2xl font-bold text-center text-white mb-3"
          style={{
            textShadow: selected
              ? `0 0 20px ${variantColor}60`
              : isHovered
                ? `0 0 12px ${variantColor}40`
                : 'none',
          }}
        >
          {data.displayName}
        </h2>

        {/* Stats grid */}
        {stats.length > 0 && (
          <div
            className={cn(
              'flex items-center justify-center flex-wrap mb-3',
              gapTokens.default
            )}
          >
            {stats.map((stat, idx) => (
              <div
                key={idx}
                className="px-3 py-1.5 rounded-lg text-center"
                style={{
                  background: `${variantColor}15`,
                  border: `1px solid ${variantColor}30`,
                }}
              >
                <div className="text-lg font-bold text-white">
                  {typeof stat.value === 'number' ? stat.value : stat.value}
                </div>
                <div className="text-[10px] text-white/60 uppercase tracking-wide">
                  {stat.label}
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Description */}
        {data.description && (
          <p className="text-sm text-white/60 text-center line-clamp-2 px-2">
            {data.description}
          </p>
        )}
      </div>
    </CardWrapper>
  );
});
