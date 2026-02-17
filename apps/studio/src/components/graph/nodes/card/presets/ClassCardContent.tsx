'use client';

/**
 * ClassCardContent - "Passport Élégant" SCHEMA Level Design for Class nodes (v0.13.1)
 *
 * 3-Level Architecture - SCHEMA Level (M1):
 * - Horizontal flex layout: Icon zone | Separator | Content zone
 * - Elevated visual treatment: 2px border, multi-layer glow
 * - Typography: lg title, rounded-xl corners
 *
 * Visual Encoding (ADR-005):
 * - Fill gradient → Layer (semantic=blue, foundation=purple, etc.) - 40-50% opacity
 * - Border color → Realm (shared=cyan, org=sky)
 * - Border style → Trait (solid=defined, dashed=authored, dotted=imported/generated)
 *
 * Layout (Passport Élégant):
 * ┌────────────────────────────────────────────────┐
 * │       │  NodeClass                   ◉ shared  │  ← Type + Realm badge
 * │  🏛️   │                                        │
 * │ radial│       SEMANTIC                         │  ← Layer as HERO (big text)
 * │  glow │                                        │
 * │       │  ◉ defined  ⊞ 4 props                  │  ← Trait chip + Property count
 * │       │  ──────────────────────────────────    │
 * │       │  Entity                                │  ← Class name (key, monospace)
 * └───────┴────────────────────────────────────────┘
 *     60px  2px           Content zone
 *
 * ArcClass variant:
 * ┌────────────────────────────────────────────────┐
 * │       │  ArcClass                   ◉ owner    │  ← Type + Family badge
 * │  →    │                                        │
 * │ radial│       ownership                        │  ← Family as HERO
 * │  glow │                                        │
 * │       │  Page → Entity                         │  ← Source → Target
 * │       │  ──────────────────────────────────    │
 * │       │  HAS_ENTITY                            │  ← Arc name (key, monospace)
 * └───────┴────────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { ArrowRight } from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import type { Layer } from '@novanet/core/types';
import type { CardContext } from '../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { SPRING_CONFIGS } from '../animationPresets';
import {
  REALM_COLORS,
  LAYER_COLORS,
  TRAIT_COLORS,
  ARC_FAMILY_COLORS,
  type RealmKey,
  type LayerKey,
  type TraitKey,
  type ArcFamilyKey,
} from '@/design/colors/generated';
import type { NodeLayer, NodeRealm, NodeTrait } from '../taxonomyColors';
import { TaxonomyBadge } from '../TaxonomyBadge';
import { LEVEL_VISUALS } from '../variants/levelVariants';
import { GlowEffect, BorderBeam } from '../effects';
import { LLMContextBadge } from '../LLMContextBadge';

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
  /** LLM context string (USE/TRIGGERS/NOT/RELATES) */
  llmContext?: string;
  /** Description from YAML */
  description?: string;
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
// Helper Components - Compact Chips (Passport Elegant pattern)
// =============================================================================

interface ChipProps {
  children: React.ReactNode;
  color: string;
  variant?: 'default' | 'accent';
  icon?: React.ReactNode;
}

const Chip = memo(function Chip({ children, color, variant = 'default', icon }: ChipProps) {
  if (!children) return null;

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-medium whitespace-nowrap"
      style={{
        background: variant === 'accent' ? `${color}25` : `${color}12`,
        color: variant === 'accent' ? color : 'rgba(255,255,255,0.8)',
        border: variant === 'accent' ? `1px solid ${color}40` : undefined,
      }}
    >
      {icon}
      {children}
    </span>
  );
});

// =============================================================================
// Main Component - "Passport Élégant" Horizontal Layout
// =============================================================================

export const ClassCardContent = memo(function ClassCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = false,
}: ClassCardContentProps) {
  // Performance flags
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;

  // SCHEMA level visual config
  const levelVisuals = LEVEL_VISUALS.schema;

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
  const familyColor = isArcClass && data.family
    ? ARC_FAMILY_COLORS[data.family as ArcFamilyKey]?.color ?? '#94a3b8'
    : layerColor;
  const borderStyle = TRAIT_BORDER_STYLES[data.trait ?? 'defined'] ?? 'solid';

  // Primary color for effects (layer for NodeClass, family for ArcClass)
  const primaryColor = isArcClass ? familyColor : layerColor;

  // Hero text: Layer name for NodeClass, Family name for ArcClass
  const heroText = isArcClass ? (data.family ?? 'arc') : (data.layer ?? 'class');

  // v0.13.1 UX: DRAMATIC background gradient (40-50% opacity for clear visibility)
  const backgroundStyle = useMemo(
    () => ({
      background: `
        linear-gradient(135deg,
          ${primaryColor}45 0%,
          ${primaryColor}28 25%,
          rgba(18,18,28,0.90) 50%,
          ${realmColor}22 75%,
          ${primaryColor}38 100%
        )
      `,
    }),
    [primaryColor, realmColor]
  );

  // v0.13.1 UX: Multi-layer card shadow
  const cardShadowStyle = useMemo(
    () => ({
      boxShadow: selected
        ? `
            0 0 25px ${primaryColor}50,
            0 0 50px ${primaryColor}25,
            0 8px 32px -8px ${realmColor}30,
            inset 0 1px 0 rgba(255,255,255,0.1)
          `
        : isHovered
          ? `
              0 12px 32px -8px rgba(0,0,0,0.5),
              0 0 20px ${primaryColor}35,
              0 0 40px ${primaryColor}15,
              inset 0 1px 0 rgba(255,255,255,0.08)
            `
          : `
              0 4px 16px -4px rgba(0,0,0,0.4),
              0 0 12px ${primaryColor}20,
              inset 0 1px 0 rgba(255,255,255,0.05)
            `,
    }),
    [primaryColor, realmColor, selected, isHovered]
  );

  // Icon zone radial glow
  const iconZoneStyle = useMemo(
    () => ({
      background: `radial-gradient(circle at center, ${primaryColor}40 0%, ${primaryColor}18 45%, transparent 75%)`,
    }),
    [primaryColor]
  );

  // Separator glow
  const separatorStyle = useMemo(
    () => ({
      background: `linear-gradient(180deg, transparent 0%, ${primaryColor}65 50%, transparent 100%)`,
      boxShadow: `0 0 14px ${primaryColor}45`,
    }),
    [primaryColor]
  );

  // Icon style with glow
  const iconStyle = useMemo(
    () => ({
      color: primaryColor,
      filter: `drop-shadow(0 0 ${selected ? '16px' : '10px'} ${primaryColor})`,
    }),
    [primaryColor, selected]
  );

  // Border width based on trait style
  const borderWidth = borderStyle === 'double' ? 3 : levelVisuals.borderWidth;

  // Wrapper
  const CardWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <CardWrapper
      className="relative flex h-full rounded-xl overflow-hidden"
      style={{
        ...backgroundStyle,
        ...cardShadowStyle,
        borderWidth,
        borderStyle,
        borderColor: selected ? realmColor : `${realmColor}${selected ? '80' : '50'}`,
        minHeight: 110,
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
          color={primaryColor}
          intensity={selected ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Rotating border beam (v0.13.1 premium effect) */}
      {showPremiumEffects && animationsEnabled && (selected || isHovered) && (
        <BorderBeam
          color={primaryColor}
          secondaryColor={realmColor}
          borderRadius={12}
          thickness={2}
          duration={selected ? 5 : 8}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          beamLength={0.15}
        />
      )}

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* ICON ZONE (left, 60px) with radial glow                                 */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div
        className="relative flex items-center justify-center shrink-0 z-10"
        style={{ width: 60, ...iconZoneStyle }}
      >
        {isArcClass ? (
          // ArcClass: Arrow icon
          <ArrowRight
            size={30}
            strokeWidth={2}
            className={cn(
              'transition-transform duration-200',
              (selected || isHovered) && 'scale-110'
            )}
            style={iconStyle}
          />
        ) : (
          // NodeClass: Layer icon
          <LayerIcon
            layer={(data.layer || 'semantic') as Layer}
            size={30}
            strokeWidth={1.8}
            className={cn(
              'transition-transform duration-200',
              (selected || isHovered) && 'scale-110'
            )}
            style={iconStyle}
          />
        )}
      </div>

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* VERTICAL SEPARATOR (2px, glowing)                                       */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div className="w-[2px] shrink-0 self-stretch my-2 z-10" style={separatorStyle} />

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* CONTENT ZONE (right)                                                    */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div className="relative flex-1 px-3 py-2.5 min-w-0 z-10 flex flex-col">
        {/* HEADER: Type label + Realm/Family badge (no overlap) */}
        <div className="flex items-center justify-between mb-1.5">
          <span
            className="text-[10px] font-bold uppercase tracking-wider"
            style={{ color: primaryColor }}
          >
            {isArcClass ? 'ArcClass' : 'NodeClass'}
          </span>

          {/* Realm badge (NodeClass) or Family badge (ArcClass) */}
          <div
            className={cn(
              'inline-flex items-center gap-1 px-1.5 py-0.5 rounded-full shrink-0',
              'text-[8px] font-bold uppercase tracking-wider'
            )}
            style={{
              background: `linear-gradient(135deg, ${realmColor}30 0%, ${realmColor}15 100%)`,
              border: `1px solid ${realmColor}50`,
              color: realmColor,
              boxShadow: `0 0 8px ${realmColor}20`,
            }}
          >
            <span
              className={cn('w-1 h-1 rounded-full', selected && 'animate-pulse')}
              style={{ background: realmColor, boxShadow: `0 0 4px ${realmColor}` }}
            />
            {data.realm ?? 'org'}
          </div>
        </div>

        {/* HERO: Layer name (NodeClass) or Family name (ArcClass) as BIG text with glow */}
        <h3
          className={cn(
            'text-xl font-bold text-white uppercase tracking-wide',
            'transition-all duration-200',
            (selected || isHovered) && 'tracking-wider'
          )}
          style={{
            textShadow: selected
              ? `0 0 24px ${primaryColor}80, 0 0 48px ${primaryColor}40`
              : isHovered
                ? `0 0 18px ${primaryColor}60, 0 0 36px ${primaryColor}25`
                : `0 0 10px ${primaryColor}30`,
          }}
        >
          {heroText}
        </h3>

        {/* CHIPS ROW: Trait chip + Property count (NodeClass) OR Source→Target (ArcClass) */}
        <div className={cn('flex flex-wrap items-center mt-1.5', gapTokens.compact)}>
          {isArcClass ? (
            // ArcClass: Source → Target
            <>
              {data.source && (
                <Chip color={primaryColor} variant="accent">
                  {data.source}
                </Chip>
              )}
              <span className="text-[10px]" style={{ color: primaryColor }}>→</span>
              {data.target && (
                <Chip color={primaryColor} variant="accent">
                  {data.target}
                </Chip>
              )}
            </>
          ) : (
            // NodeClass: Trait chip + Property count chip
            <>
              {data.trait && (
                <Chip
                  color={traitColor}
                  variant="accent"
                  icon={
                    <span
                      className="w-1.5 h-1.5 rounded-full"
                      style={{ background: traitColor, boxShadow: `0 0 4px ${traitColor}` }}
                    />
                  }
                >
                  {data.trait}
                </Chip>
              )}
              {data.propCount !== undefined && (
                <Chip color={primaryColor}>
                  {data.propCount} props
                </Chip>
              )}
            </>
          )}
        </div>

        {/* SPACER */}
        <div className="flex-1 min-h-1" />

        {/* DIVIDER */}
        <div
          className="h-px mt-2 mb-1.5"
          style={{
            background: `linear-gradient(90deg, ${primaryColor}50, transparent)`,
            boxShadow: `0 0 6px ${primaryColor}20`,
          }}
        />

        {/* FOOTER: Class name (key) with monospace font */}
        <div className="text-xs text-white/70 truncate font-mono font-medium">
          {className}
        </div>

        {/* LLM Context Badge (ADR-027) - Only show if present and space allows */}
        {data.llmContext && (
          <div className="mt-1.5">
            <LLMContextBadge
              llmContext={data.llmContext}
              color={primaryColor}
              selected={selected}
              isHovered={isHovered}
              mode="compact"
              expandable={true}
            />
          </div>
        )}
      </div>
    </CardWrapper>
  );
});
