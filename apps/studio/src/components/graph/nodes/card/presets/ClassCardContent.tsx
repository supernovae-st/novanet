'use client';

/**
 * ClassCardContent - Clear Schema Design for Class nodes (v0.12.5)
 *
 * Visual Encoding (ADR-005):
 * - Fill gradient → Layer (semantic=blue, foundation=purple, etc.)
 * - Border color → Realm (shared=cyan, org=sky)
 * - Border style → Trait (solid=defined, dashed=authored, dotted=imported/generated)
 *
 * Design principles:
 * - Hero class name at top with Layer icon
 * - Clear vertical stack: REALM / LAYER / TRAIT badges
 * - Minimal effects for clarity
 * - ArcClass shows source→target
 *
 * Layout:
 * ┌──────────────────────────────────────────┐
 * │  🏛️ NodeClass                         ● │  ← Layer icon + type
 * │  ════════════════════════════════════════│  ← Separator
 * │                                          │
 * │  Entity                                  │  ← Class name (hero)
 * │                                          │
 * │  ┌────────┐ ┌────────┐ ┌────────────┐   │
 * │  │  ORG   │ │SEMANTIC│ │  DEFINED   │   │  ← Stacked badges
 * │  └────────┘ └────────┘ └────────────┘   │
 * │                                          │
 * │  4 properties                            │  ← Property count
 * └──────────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import type { Layer } from '@novanet/core/types';
import type { CardContext } from '../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { SPRING_CONFIGS, DURATIONS } from '../animationPresets';
import {
  REALM_COLORS,
  LAYER_COLORS,
  TRAIT_COLORS,
  type RealmKey,
  type LayerKey,
  type TraitKey,
} from '@/design/colors/generated';
import type { NodeLayer, NodeRealm, NodeTrait } from '../taxonomyColors';
import { TaxonomyBadge } from '../TaxonomyBadge';
import { MotionTechCorners } from '../../effects';

// =============================================================================
// Types
// =============================================================================

export interface ClassNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  /** Realm: shared | org */
  realm?: string;
  /** Layer: config, locale, semantic, etc. */
  layer?: string;
  /** Trait: defined, authored, imported, generated, retrieved */
  trait?: string;
  /** Property count */
  propCount?: number;
  /** For ArcClass: source node type */
  source?: string;
  /** For ArcClass: target node type */
  target?: string;
  /** Arc family for ArcClass */
  family?: string;
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface ClassTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
  trait: NodeTrait;
}

export interface ClassCardContentProps extends CardContext {
  data: ClassNodeData;
  /** Performance configuration for conditional effect rendering */
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: ClassTaxonomyProps;
  /** Show TaxonomyBadge in header instead of simple label (default: false) */
  showTaxonomyBadge?: boolean;
}


// =============================================================================
// Animation Variants
// =============================================================================

const cardVariants: Variants = {
  idle: {
    scale: 1,
    y: 0,
  },
  hover: {
    scale: 1.02,
    y: -2,
    transition: SPRING_CONFIGS.gentle,
  },
  selected: {
    scale: 1.03,
    y: -3,
    transition: SPRING_CONFIGS.smooth,
  },
};

// =============================================================================
// Helper: Trait to border style (ADR-005)
// =============================================================================

const TRAIT_BORDER_STYLES: Record<string, string> = {
  defined: 'solid',
  authored: 'dashed',
  imported: 'double',
  generated: 'dotted',
  retrieved: 'dotted',
};

// =============================================================================
// Main Component
// =============================================================================

export const ClassCardContent = memo(function ClassCardContent({
  data,
  colors,
  selected,
  isHovered,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = false,
}: ClassCardContentProps) {
  // Performance flags
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const showTechCorners = performanceConfig?.effects?.techCorners ?? true;

  // Determine if this is a NodeClass or ArcClass
  const isArcClass = data.type === 'ArcClass' || data.family !== undefined;

  // Extract class name from key (e.g., "NodeClass:Entity" → "Entity")
  const className = useMemo(() => {
    const parts = data.key.split(':');
    return parts.length > 1 ? parts[1] : data.displayName;
  }, [data.key, data.displayName]);

  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Get colors for each taxonomy axis
  const realmColor = REALM_COLORS[data.realm as RealmKey]?.color ?? colors.secondary;
  const layerColor = LAYER_COLORS[data.layer as LayerKey]?.color ?? colors.primary;
  const traitColor = TRAIT_COLORS[data.trait as TraitKey]?.color ?? '#94a3b8';
  const borderStyle = TRAIT_BORDER_STYLES[data.trait ?? 'defined'] ?? 'solid';

  // Background gradient based on layer
  const backgroundStyle = useMemo(
    () => ({
      background: `linear-gradient(135deg, ${layerColor}15 0%, rgba(0,0,0,0.9) 50%, ${layerColor}08 100%)`,
    }),
    [layerColor]
  );

  // Border based on realm + trait style (ADR-005)
  const borderWidth = borderStyle === 'double' ? 3 : 2;
  const cardStyle = useMemo(
    () => ({
      ...backgroundStyle,
      borderWidth,
      borderStyle,
      borderColor: selected ? realmColor : `${realmColor}80`,
      boxShadow: selected
        ? `0 0 20px ${layerColor}40, 0 0 40px ${layerColor}20`
        : isHovered
          ? `0 0 12px ${layerColor}30`
          : `0 0 4px ${layerColor}10`,
    }),
    [backgroundStyle, borderWidth, borderStyle, realmColor, layerColor, selected, isHovered]
  );

  // Icon style
  const iconStyle = useMemo(
    () => ({
      color: layerColor,
      filter: `drop-shadow(0 0 ${selected ? '8px' : '4px'} ${layerColor}80)`,
    }),
    [layerColor, selected]
  );

  // Name glow style
  const nameGlowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 16px ${layerColor}60`
        : isHovered
          ? `0 0 8px ${layerColor}40`
          : 'none',
    }),
    [layerColor, selected, isHovered]
  );

  // Wrapper component
  const CardWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <CardWrapper
      className="relative p-3 min-h-[130px] rounded-lg overflow-hidden"
      style={cardStyle}
      {...(animationsEnabled && {
        variants: cardVariants,
        initial: 'idle',
        animate: animationState,
      })}
    >
      {/* Subtle tech corners */}
      {showTechCorners && (
        <MotionTechCorners
          color={layerColor}
          selected={selected}
          isHovered={isHovered}
          size={10}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Content */}
      <div className="relative z-10">
        {/* Header: Layer Icon + Type */}
        <div className={cn('flex items-center justify-between mb-2', gapTokens.default)}>
          <div className={cn('flex items-center', gapTokens.default)}>
            <LayerIcon
              layer={(data.layer || 'semantic') as Layer}
              size={18}
              strokeWidth={2}
              className={cn(
                'transition-transform duration-200',
                (selected || isHovered) && 'scale-110'
              )}
              style={iconStyle}
            />
            <span
              className="text-[10px] font-semibold uppercase tracking-wide"
              style={{ color: layerColor }}
            >
              {isArcClass ? 'ArcClass' : 'NodeClass'}
            </span>
          </div>

          {/* Status dot with realm color */}
          <div
            className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
            style={{
              background: realmColor,
              boxShadow: `0 0 6px ${realmColor}`,
            }}
          />
        </div>

        {/* Separator line */}
        <div
          className="h-px mb-3"
          style={{
            background: `linear-gradient(90deg, ${layerColor}50, ${layerColor}20, transparent)`,
          }}
        />

        {/* Class name - hero element */}
        <h3
          className={cn(
            'text-xl font-bold font-mono text-white mb-3',
            'transition-all duration-200'
          )}
          style={nameGlowStyle}
        >
          {className}
        </h3>

        {/* 3 taxonomy badges - clear vertical hierarchy */}
        <div className={cn('flex flex-wrap items-center mb-2', gapTokens.default)}>
          {/* Realm badge */}
          {data.realm && (
            <div
              className="inline-flex items-center px-2 py-0.5 rounded text-[10px] font-semibold uppercase tracking-wide"
              style={{
                background: `${realmColor}25`,
                color: realmColor,
                border: `1px solid ${realmColor}40`,
              }}
            >
              {data.realm}
            </div>
          )}

          {/* Layer badge */}
          {data.layer && (
            <div
              className="inline-flex items-center px-2 py-0.5 rounded text-[10px] font-semibold uppercase tracking-wide"
              style={{
                background: `${layerColor}25`,
                color: layerColor,
                border: `1px solid ${layerColor}40`,
              }}
            >
              {data.layer}
            </div>
          )}

          {/* Trait badge (for NodeClass) or Family badge (for ArcClass) */}
          {isArcClass ? (
            data.family && (
              <div
                className="inline-flex items-center px-2 py-0.5 rounded text-[10px] font-semibold uppercase tracking-wide"
                style={{
                  background: 'rgba(148, 163, 184, 0.2)',
                  color: '#94a3b8',
                  border: '1px solid rgba(148, 163, 184, 0.3)',
                }}
              >
                {data.family}
              </div>
            )
          ) : (
            data.trait && (
              <div
                className="inline-flex items-center px-2 py-0.5 rounded text-[10px] font-semibold uppercase tracking-wide"
                style={{
                  background: `${traitColor}25`,
                  color: traitColor,
                  border: `1px solid ${traitColor}40`,
                }}
              >
                {data.trait}
              </div>
            )
          )}
        </div>

        {/* Property count */}
        {data.propCount !== undefined && (
          <div className="text-[10px] text-white/50 font-mono">
            {data.propCount} {data.propCount === 1 ? 'property' : 'properties'}
          </div>
        )}

        {/* ArcClass: source → target */}
        {isArcClass && (data.source || data.target) && (
          <div
            className="mt-2 pt-2 flex items-center justify-center text-[11px] font-mono border-t"
            style={{ borderColor: `${layerColor}20` }}
          >
            <span className="text-white/60">{data.source || '?'}</span>
            <span className="mx-2" style={{ color: layerColor }}>→</span>
            <span className="text-white/60">{data.target || '?'}</span>
          </div>
        )}
      </div>
    </CardWrapper>
  );
});
