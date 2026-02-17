'use client';

/**
 * StructuralCardContent - "Passport" horizontal layout for structural nodes
 *
 * Used by: Page, Entity, Block, PageStructure, BlockType, ContentSlot, etc.
 *
 * Magic UI "Passport" Design (v0.13.1):
 * - Icon zone LEFT (60px) with radial glow background
 * - Glowing vertical separator
 * - Content RIGHT with type label + layer badge, hero display name, property chips
 *
 * Enhanced with optional TaxonomyBadge support for full visual encoding (ADR-005):
 * - Layer → Fill color
 * - Realm → Border color
 * - Trait → Border style + animation
 *
 * Layout:
 * ┌─────────────────────────────────────────┐
 * │  ▣   │  ◉ TYPE           [LAYER] ●     │
 * │      │                                  │
 * │ glow │     Display Name                 │
 * │ zone │                                  │
 * │      │  [chip1] [chip2] [chip3]         │
 * │      │  ─────────────────────────────── │
 * │      │  key: entity-slug        🇫🇷 fr-FR│
 * └──────┴──────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion } from 'motion/react';
import { cn } from '@/lib/utils';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { gapTokens } from '@/design/tokens';
import { localeToFlag } from '@/lib/localeUtils';
import type { CardContext } from '../CardShell';
import type { Layer } from '@novanet/core/types';
import type { NodeLayer, NodeRealm, NodeTrait } from '../taxonomyColors';
import { TaxonomyBadge } from '../TaxonomyBadge';
import { BorderBeam, GlowEffect } from '../effects';
import { SPRING_CONFIGS } from '../animationPresets';

// =============================================================================
// Types
// =============================================================================

export interface StructuralNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  /** BCP-47 locale code for locale-specific nodes */
  locale?: string;
  /** Optional property chips to display (max 3) */
  chips?: string[];
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface TaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
  trait: NodeTrait;
}

export interface StructuralCardContentProps extends CardContext {
  data: StructuralNodeData;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: TaxonomyProps;
  /** Show TaxonomyBadge instead of simple header (default: false) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Component
// =============================================================================

export const StructuralCardContent = memo(function StructuralCardContent({
  data,
  colors,
  selected,
  isHovered,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = false,
}: StructuralCardContentProps) {
  // Performance flags
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;

  // Type-safe config lookup with fallback
  const config = (NODE_TYPE_CONFIG as Record<string, { label: string; layer: string }>)[data.type]
    || { label: data.type, layer: 'foundation' };

  // Use TaxonomyBadge if taxonomy props provided and showTaxonomyBadge is true
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;

  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Animation variants for card
  const cardVariants = {
    idle: { scale: 1, y: 0 },
    hover: { scale: 1.02, y: -2, transition: SPRING_CONFIGS.gentle },
    selected: { scale: 1.03, y: -3, transition: SPRING_CONFIGS.smooth },
  };

  // v0.13.1 UX: DRAMATIC background gradient (40-50% opacity)
  const backgroundStyle = useMemo(
    () => ({
      background: `
        linear-gradient(135deg,
          ${colors.primary}45 0%,
          ${colors.primary}28 25%,
          rgba(18,18,28,0.90) 50%,
          ${colors.secondary}22 75%,
          ${colors.primary}38 100%
        )
      `,
    }),
    [colors.primary, colors.secondary]
  );

  // v0.13.1 UX: DRAMATIC card shadow with multi-layer glow
  const cardShadowStyle = useMemo(
    () => ({
      boxShadow: selected
        ? `
            0 0 25px ${colors.primary}50,
            0 0 50px ${colors.primary}25,
            0 8px 32px -8px ${colors.secondary}30,
            inset 0 1px 0 rgba(255,255,255,0.1)
          `
        : isHovered
          ? `
              0 12px 32px -8px rgba(0,0,0,0.5),
              0 0 20px ${colors.primary}35,
              0 0 40px ${colors.primary}15,
              inset 0 1px 0 rgba(255,255,255,0.08)
            `
          : `
              0 4px 16px -4px rgba(0,0,0,0.4),
              0 0 12px ${colors.primary}20,
              inset 0 1px 0 rgba(255,255,255,0.05)
            `,
    }),
    [colors.primary, colors.secondary, selected, isHovered]
  );

  // Icon style with DRAMATIC glow (v0.13.1)
  const iconStyle = useMemo(
    () => ({
      color: colors.primary,
      filter: `drop-shadow(0 0 ${selected ? '14px' : '8px'} ${colors.primary})`,
    }),
    [colors.primary, selected]
  );

  // Icon zone radial glow - BRIGHTER (v0.13.1 Passport style)
  const iconZoneStyle = useMemo(
    () => ({
      background: `radial-gradient(circle at center, ${colors.primary}35 0%, ${colors.primary}15 40%, transparent 70%)`,
    }),
    [colors.primary]
  );

  // Separator glow style - BRIGHTER (v0.13.1)
  const separatorStyle = useMemo(
    () => ({
      background: `linear-gradient(180deg, transparent 0%, ${colors.primary}60 50%, transparent 100%)`,
      boxShadow: `0 0 12px ${colors.primary}40`,
    }),
    [colors.primary]
  );

  // Property chips to display (max 3)
  const displayChips = useMemo(() => {
    const chips = data.chips || [];
    return chips.slice(0, 3);
  }, [data.chips]);

  // Wrapper component
  const CardWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <CardWrapper
      className="relative flex h-full rounded-xl overflow-hidden"
      style={{
        ...backgroundStyle,
        ...cardShadowStyle,
        border: `2px solid ${colors.primary}${selected ? '80' : '40'}`,
        minHeight: 100,
      }}
      {...(animationsEnabled && {
        variants: cardVariants,
        initial: 'idle',
        animate: animationState,
      })}
    >
      {/* Premium glow effect */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={colors.primary}
          intensity={selected ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Rotating border beam (v0.13.1 premium effect) */}
      {showPremiumEffects && animationsEnabled && (selected || isHovered) && (
        <BorderBeam
          color={colors.primary}
          secondaryColor={colors.secondary}
          borderRadius={12}
          thickness={2}
          duration={selected ? 4 : 7}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          beamLength={0.2}
        />
      )}

      {/* Icon Zone (left) - 60px with radial glow */}
      <div
        className="relative flex items-center justify-center shrink-0 z-10"
        style={{
          width: 60,
          ...iconZoneStyle,
        }}
      >
        <LayerIcon
          layer={config.layer as Layer}
          size={28}
          strokeWidth={2}
          className={cn(
            'transition-transform duration-200',
            (selected || isHovered) && 'scale-110'
          )}
          style={{
            ...iconStyle,
            filter: selected
              ? `drop-shadow(0 0 16px ${colors.primary}80)`
              : isHovered
                ? `drop-shadow(0 0 10px ${colors.primary}60)`
                : `drop-shadow(0 0 6px ${colors.primary}30)`,
          }}
        />
      </div>

      {/* Vertical Separator - GLOWING (v0.13.1) */}
      <div
        className="w-[2px] shrink-0 self-stretch my-2 z-10"
        style={separatorStyle}
      />

      {/* Content Zone (right) */}
      <div className="relative flex-1 px-3 py-2.5 min-w-0 z-10">
        {/* Header: TaxonomyBadge OR Type Label + Layer Badge + Status Dot */}
        {useTaxonomyBadge ? (
          <div className="mb-1">
            <TaxonomyBadge
              layer={taxonomy.layer}
              realm={taxonomy.realm}
              trait={taxonomy.trait}
              className={data.type}
              selected={selected}
              isHovered={isHovered}
              performanceConfig={performanceConfig}
              size="sm"
              showLayerLabel={true}
              showTraitIndicator={true}
            />
          </div>
        ) : (
          <div className="flex items-center justify-between mb-1">
            <div className={cn('flex items-center', gapTokens.default)}>
              {/* Type label */}
              <span
                className="text-[10px] font-bold uppercase tracking-wider"
                style={{ color: colors.primary }}
              >
                {config.label}
              </span>

              {/* Layer badge - pill style */}
              <div
                className={cn(
                  'inline-flex items-center gap-1 px-1.5 py-0.5 rounded-full',
                  'text-[8px] font-bold uppercase tracking-wider'
                )}
                style={{
                  background: `linear-gradient(135deg, ${colors.primary}30 0%, ${colors.primary}15 100%)`,
                  border: `1px solid ${colors.primary}50`,
                  color: colors.primary,
                  boxShadow: `0 0 8px ${colors.primary}20`,
                }}
              >
                {config.layer}
              </div>
            </div>

            {/* Status dot - LARGER with glow (v0.13.1) */}
            <div
              className={cn('w-2.5 h-2.5 rounded-full', selected && 'animate-pulse')}
              style={{
                background: colors.primary,
                boxShadow: `0 0 10px ${colors.primary}`,
              }}
            />
          </div>
        )}

        {/* Display Name - HERO text with dramatic glow (v0.13.1) */}
        <h3
          className={cn(
            'text-sm font-bold text-white truncate',
            'transition-all duration-200',
            (selected || isHovered) && 'tracking-wide'
          )}
          style={{
            textShadow: selected
              ? `0 0 20px ${colors.primary}70, 0 0 40px ${colors.primary}35`
              : isHovered
                ? `0 0 14px ${colors.primary}50, 0 0 28px ${colors.primary}20`
                : `0 0 8px ${colors.primary}25`,
          }}
        >
          {data.displayName}
        </h3>

        {/* Property chips row (max 3) */}
        {displayChips.length > 0 && (
          <div className={cn('flex items-center flex-wrap mt-1.5', gapTokens.compact)}>
            {displayChips.map((chip, index) => (
              <div
                key={index}
                className="inline-flex items-center px-1.5 py-0.5 rounded text-[8px] font-medium"
                style={{
                  background: `${colors.primary}20`,
                  border: `1px solid ${colors.primary}35`,
                  color: `${colors.primary}dd`,
                }}
              >
                {chip}
              </div>
            ))}
          </div>
        )}

        {/* Divider - GLOWING (v0.13.1) */}
        <div
          className="h-px mt-2 mb-1.5"
          style={{
            background: `linear-gradient(90deg, ${colors.primary}50, transparent)`,
            boxShadow: `0 0 6px ${colors.primary}20`,
          }}
        />

        {/* Footer: Key (monospace) + Locale */}
        <div className={cn('flex items-center justify-between', gapTokens.compact)}>
          {/* Key in monospace - truncated */}
          <span
            className="text-[9px] font-mono truncate flex-1"
            style={{ color: `${colors.primary}90` }}
          >
            {data.key}
          </span>

          {/* Locale badge - only shown for locale-specific nodes */}
          {data.locale && (
            <div
              className="inline-flex items-center px-1.5 py-0.5 rounded-full text-[8px] font-medium gap-1 shrink-0"
              style={{
                background: 'rgba(255, 255, 255, 0.08)',
                border: '1px solid rgba(255, 255, 255, 0.15)',
                boxShadow: `0 0 8px ${colors.primary}15`,
              }}
            >
              <span className="text-xs leading-none">{localeToFlag(data.locale)}</span>
              <span className="text-white/70 font-mono">{data.locale}</span>
            </div>
          )}
        </div>
      </div>
    </CardWrapper>
  );
});
