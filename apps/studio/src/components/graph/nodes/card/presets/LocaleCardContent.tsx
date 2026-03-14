'use client';

/**
 * LocaleCardContent - "Passport Élégant" design for Locale nodes
 *
 * Features:
 * - Large flag in dedicated left zone (75px) with radial glow + pulse
 * - BCP-47 code as hero element (28px mono bold, premium letter-spacing)
 * - Vertical glowing separator
 * - Display name with region chip
 * - Layer badge with prominent glow effect
 *
 * Layout:
 * ┌──────────────────────────────────────────────┐
 * │  🇫🇷   │  🌐 LOCALE                       ● │
 * │       │                                     │
 * │ glow  │     fr-FR                           │
 * │ zone  │                                     │
 * │ pulse │  French  [France]                   │
 * │       │  ────────────────────────────────── │
 * │       │  Europe • Western Europe  ●config   │
 * └───────┴─────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion } from 'motion/react';
import { cn } from '@/lib/utils';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { localeToFlag } from '@/lib/localeUtils';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import type { Layer } from '@novanet/core/types';
import type { NodeLayer, NodeRealm } from '../taxonomyColors';
import { TaxonomyBadge } from '../TaxonomyBadge';
import { BorderBeam, GlowEffect } from '../effects';
import { SPRING_CONFIGS } from '../animationPresets';

// =============================================================================
// Types
// =============================================================================

export interface LocaleNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  /** Region/geographic context (e.g., "Europe • Western Europe") */
  region?: string;
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface LocaleTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
}

export interface LocaleCardContentProps extends CardContext {
  data: LocaleNodeData;
  /** Performance configuration for conditional effect rendering */
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: LocaleTaxonomyProps;
  /** Show TaxonomyBadge instead of simple header (default: false) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Helper: Extract language name from displayName
// =============================================================================

/**
 * Extract display parts from locale display name
 * "French (France)" → { language: "French", region: "France" }
 */
function parseDisplayName(displayName: string): { language: string; region?: string } {
  const match = displayName.match(/^(.+?)\s*\((.+)\)$/);
  if (match) {
    return { language: match[1].trim(), region: match[2].trim() };
  }
  return { language: displayName };
}

// =============================================================================
// Component
// =============================================================================

export const LocaleCardContent = memo(function LocaleCardContent({
  data,
  colors,
  selected,
  isHovered,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = false,
}: LocaleCardContentProps) {
  // Performance flags
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;

  // Use TaxonomyBadge if taxonomy props provided and showTaxonomyBadge is true
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;
  // Extract BCP-47 code from key (e.g., "Locale:fr-FR" → "fr-FR")
  const bcp47 = useMemo(() => {
    const parts = data.key.split(':');
    return parts.length > 1 ? parts[1] : data.key;
  }, [data.key]);

  // Get flag emoji from BCP-47
  const flag = useMemo(() => localeToFlag(bcp47), [bcp47]);

  // Parse display name for structured display
  const { language, region } = useMemo(
    () => parseDisplayName(data.displayName),
    [data.displayName]
  );

  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Animation variants for card - SUBTLE, NO SCALE to prevent layout shift
  const cardVariants = {
    idle: { y: 0 },
    hover: { y: -1, transition: SPRING_CONFIGS.gentle },
    selected: { y: -2, transition: SPRING_CONFIGS.smooth },
  };

  // v0.13.1 UX: DRAMATIC background gradient (40-50% opacity for clear impact)
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

  // Flag zone gradient - BRIGHTER radial glow with pulse-ready layers (v0.13.1)
  const flagZoneStyle = useMemo(
    () => ({
      background: `radial-gradient(circle at center, ${colors.primary}40 0%, ${colors.primary}20 35%, ${colors.primary}08 60%, transparent 80%)`,
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

      {/* Flag Zone (left) - FIXED 75px, no layout-shifting animations */}
      <div
        className="relative flex items-center justify-center shrink-0 z-10"
        style={{
          width: 75,
          ...flagZoneStyle,
        }}
      >
        <span
          className="text-5xl"
          style={{
            filter: selected
              ? `drop-shadow(0 0 20px ${colors.primary}90)`
              : isHovered
                ? `drop-shadow(0 0 14px ${colors.primary}70)`
                : `drop-shadow(0 0 8px ${colors.primary}40)`,
          }}
        >
          {flag}
        </span>
      </div>

      {/* Vertical Separator - GLOWING (v0.13.1) */}
      <div
        className="w-[2px] shrink-0 self-stretch my-2 z-10"
        style={separatorStyle}
      />

      {/* Content Zone (right) */}
      <div className="relative flex-1 px-3 py-2.5 min-w-0 z-10">
        {/* Header: TaxonomyBadge or Icon + Type Label + Status Dot */}
        {useTaxonomyBadge ? (
          <div className="mb-1">
            <TaxonomyBadge
              layer={taxonomy.layer}
              realm={taxonomy.realm}
              className={data.type}
              selected={selected}
              isHovered={isHovered}
              performanceConfig={performanceConfig}
              size="sm"
              showLayerLabel={true}
            />
          </div>
        ) : (
          <div className="flex items-center justify-between mb-1">
            <div className={cn('flex items-center', gapTokens.default)}>
              <LayerIcon
                layer={'locale' as Layer}
                size={18}
                strokeWidth={2}
                className={cn(
                  'transition-transform duration-200',
                  (selected || isHovered) && 'scale-110'
                )}
                style={iconStyle}
              />
              <span
                className="text-[10px] font-bold uppercase tracking-wider"
                style={{ color: colors.primary }}
              >
                Locale
              </span>
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

        {/* BCP-47 Hero - FIXED SIZE, NO LETTER-SPACING ANIMATION (v0.13.1 fix) */}
        <h3
          className="text-2xl font-bold font-mono text-white whitespace-nowrap"
          style={{
            letterSpacing: '0.05em',
            textShadow: selected
              ? `0 0 28px ${colors.primary}90, 0 0 56px ${colors.primary}50, 0 2px 4px rgba(0,0,0,0.5)`
              : isHovered
                ? `0 0 20px ${colors.primary}70, 0 0 40px ${colors.primary}30, 0 2px 4px rgba(0,0,0,0.4)`
                : `0 0 12px ${colors.primary}40, 0 2px 4px rgba(0,0,0,0.3)`,
          }}
        >
          {bcp47}
        </h3>

        {/* Display Name with Region Chip */}
        <div className="flex items-center gap-2 mt-1">
          <span className="text-xs text-white/80 font-medium truncate">
            {language}
          </span>
          {region && (
            <span
              className="inline-flex items-center px-1.5 py-0.5 rounded text-[9px] font-medium shrink-0"
              style={{
                background: `${colors.primary}20`,
                color: `${colors.primary}`,
                border: `1px solid ${colors.primary}35`,
              }}
            >
              {region}
            </span>
          )}
        </div>

        {/* Divider - GLOWING (v0.13.1) */}
        <div
          className="h-px mt-2 mb-1.5"
          style={{
            background: `linear-gradient(90deg, ${colors.primary}50, transparent)`,
            boxShadow: `0 0 6px ${colors.primary}20`,
          }}
        />

        {/* Footer: Region + Layer Badge - FIXED, no animations */}
        <div className={cn('flex items-center justify-between', gapTokens.compact)}>
          {/* Region context */}
          {data.region && (
            <span className="text-[9px] text-white/50 truncate flex-1">
              {data.region}
            </span>
          )}

          {/* Layer badge - FIXED pill, no jarring animations */}
          <div
            className={cn(
              'inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full shrink-0',
              'text-[9px] font-bold uppercase tracking-widest'
            )}
            style={{
              background: `linear-gradient(135deg, ${colors.primary}35 0%, ${colors.primary}18 100%)`,
              border: `1.5px solid ${colors.primary}60`,
              color: colors.primary,
              boxShadow: selected
                ? `0 0 20px ${colors.primary}60, inset 0 1px 0 rgba(255,255,255,0.15)`
                : `0 0 12px ${colors.primary}30, inset 0 1px 0 rgba(255,255,255,0.1)`,
            }}
          >
            <span
              className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
              style={{
                background: colors.primary,
                boxShadow: `0 0 8px ${colors.primary}`,
              }}
            />
            config
          </div>
        </div>
      </div>
    </CardWrapper>
  );
});
