'use client';

/**
 * LocaleSettingsCardContent - "Passport Élégant" design for Locale Settings nodes
 *
 * v0.13.1 UX REDESIGN: Magic UI inspired clean composition
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = locale) -> indigo #6366f1
 * - Border style -> double (imported trait)
 * - Shows locale settings type, key properties, and indicators
 *
 * Types handled (6 nodes):
 * - Culture: calendar, seasons, business norms, communication style
 * - Style: tone, formality, directness, warmth, humor
 * - Formatting: dates, numbers, currency, time, phone, address
 * - Adaptation: FACTS vs ILLUSTRATIONS, technical term handling
 * - Slugification: URL slug rules, transliteration, stop words
 * - Market: demographics, digital maturity, e-commerce landscape
 *
 * Layout (Passport Style):
 * ┌────────────────────────────────────────────────┐
 * │  🌍  │  CULTURE                      ◉ locale  │
 * │      │                                         │
 * │ glow │       northern                          │
 * │ zone │                                         │
 * │      │  Monday • Gregorian • 15 holidays       │
 * │      │  ─────────────────────────────────────  │
 * │      │  fr-FR                                  │
 * └──────┴─────────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion } from 'motion/react';
import { cn } from '@/lib/utils';
import {
  Globe,
  Drama,
  Hash,
  Droplet,
  Link,
  BarChart2,
  type LucideIcon,
} from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import type { NodeLayer, NodeRealm, NodeTrait } from '../../taxonomyColors';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { GlowEffect, BorderBeam } from '../../effects';
import { SPRING_CONFIGS } from '../../animationPresets';

// =============================================================================
// Types
// =============================================================================

export type LocaleSettingsType =
  | 'Culture'
  | 'Style'
  | 'Formatting'
  | 'Adaptation'
  | 'Slugification'
  | 'Market';

/** Base data shared by all locale settings nodes */
interface BaseLocaleSettingsData {
  id: string;
  type: LocaleSettingsType;
  key: string;
  displayName: string;
  description?: string;
}

/** Culture node data */
export interface CultureNodeData extends BaseLocaleSettingsData {
  type: 'Culture';
  hemisphere?: 'northern' | 'southern';
  work_week_start?: string;
  calendar_system?: string;
  business_hours?: string;
  holidays_count?: number;
}

/** Style node data */
export interface StyleNodeData extends BaseLocaleSettingsData {
  type: 'Style';
  formality_score?: number;
  default_formality?: 'formal' | 'casual' | 'mixed';
  default_pronoun?: string;
  directness_level?: 'direct' | 'indirect' | 'mixed';
  humor_style?: string;
}

/** Formatting node data */
export interface FormattingNodeData extends BaseLocaleSettingsData {
  type: 'Formatting';
  decimal_separator?: string;
  thousands_separator?: string;
  date_format?: string;
  time_format?: string;
  currency_code?: string;
  currency_symbol?: string;
}

/** Adaptation node data */
export interface AdaptationNodeData extends BaseLocaleSettingsData {
  type: 'Adaptation';
  idiom_preference?: 'translate' | 'localize' | 'native';
  metaphor_handling?: 'literal' | 'adapt' | 'native';
  technical_terms?: 'keep_english' | 'translate' | 'mixed';
  facts_strategy?: string;
  illustrations_strategy?: string;
}

/** Slugification node data */
export interface SlugificationNodeData extends BaseLocaleSettingsData {
  type: 'Slugification';
  slug_rule?: 'latin_strip' | 'latin_preserve' | 'native_script' | 'latin_transform' | 'transliterate';
  allow_diacritics?: boolean;
  stopwords_count?: number;
  transliteration_enabled?: boolean;
  max_length?: number;
}

/** Market node data */
export interface MarketNodeData extends BaseLocaleSettingsData {
  type: 'Market';
  ecommerce_maturity?: 'emerging' | 'growing' | 'mature' | 'advanced';
  digital_adoption?: number;
  popular_payment_methods?: string[];
  avg_internet_penetration?: number;
  mobile_first?: boolean;
}

export type LocaleSettingsNodeData =
  | CultureNodeData
  | StyleNodeData
  | FormattingNodeData
  | AdaptationNodeData
  | SlugificationNodeData
  | MarketNodeData;

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface LocaleSettingsTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
  trait: NodeTrait;
}

export interface LocaleSettingsCardContentProps extends CardContext {
  data: LocaleSettingsNodeData;
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: LocaleSettingsTaxonomyProps;
  /** Show TaxonomyBadge in header (default: true) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Constants
// =============================================================================

const TYPE_ICONS: Record<LocaleSettingsType, LucideIcon> = {
  Culture: Globe,
  Style: Drama,
  Formatting: Hash,
  Adaptation: Droplet,
  Slugification: Link,
  Market: BarChart2,
};

const TYPE_LABELS: Record<LocaleSettingsType, string> = {
  Culture: 'Culture',
  Style: 'Style',
  Formatting: 'Formatting',
  Adaptation: 'Adaptation',
  Slugification: 'Slugification',
  Market: 'Market',
};

const TYPE_DESCRIPTIONS: Record<LocaleSettingsType, string> = {
  Culture: 'Cultural context & norms',
  Style: 'Communication style',
  Formatting: 'Technical formatting',
  Adaptation: 'Content adaptation',
  Slugification: 'URL slug rules',
  Market: 'Market intelligence',
};

// =============================================================================
// Helper Components - Compact Chips
// =============================================================================

interface ChipProps {
  children: React.ReactNode;
  color: string;
  variant?: 'default' | 'accent';
}

const Chip = memo(function Chip({ children, color, variant = 'default' }: ChipProps) {
  if (!children) return null;

  return (
    <span
      className="inline-flex items-center px-1.5 py-0.5 rounded text-[9px] font-medium whitespace-nowrap"
      style={{
        background: variant === 'accent' ? `${color}25` : `${color}12`,
        color: variant === 'accent' ? color : 'rgba(255,255,255,0.8)',
        border: variant === 'accent' ? `1px solid ${color}40` : undefined,
      }}
    >
      {children}
    </span>
  );
});

interface MiniGaugeProps {
  value: number;
  label: string;
  color: string;
}

const MiniGauge = memo(function MiniGauge({ value, label, color }: MiniGaugeProps) {
  return (
    <div className="flex-1 min-w-0">
      <div className="flex items-center justify-between text-[8px] mb-0.5">
        <span className="text-white/40 truncate">{label}</span>
        <span style={{ color }} className="font-bold">{value}%</span>
      </div>
      <div className="h-1 rounded-full bg-white/10 overflow-hidden">
        <div
          className="h-full rounded-full"
          style={{
            width: `${Math.min(100, value)}%`,
            background: `linear-gradient(90deg, ${color}50, ${color})`,
          }}
        />
      </div>
    </div>
  );
});

// =============================================================================
// Type-specific Hero + Chips extractors
// =============================================================================

interface TypeContent {
  heroValue: string;  // ALWAYS has a value (with fallback)
  chips: string[];    // ALWAYS has at least one chip (with fallback)
  gauge?: { value: number; label: string };
  description?: string;
}

const SLUG_RULE_LABELS: Record<string, string> = {
  latin_strip: 'Strip Diacritics',
  latin_preserve: 'Preserve',
  native_script: 'Native Script',
  latin_transform: 'Transform',
  transliterate: 'Transliterate',
};

/**
 * Extract type-specific content with GUARANTEED fallbacks
 * Never returns empty - always shows meaningful content
 */
function extractTypeContent(data: LocaleSettingsNodeData): TypeContent {
  // Base fallback - ALWAYS works
  const baseFallback: TypeContent = {
    heroValue: data.displayName || TYPE_LABELS[data.type],
    chips: [TYPE_LABELS[data.type]],
    description: data.description || TYPE_DESCRIPTIONS[data.type],
  };

  // Helper to filter undefined and ensure at least one chip
  const filterChips = (chips: (string | undefined)[]): string[] => {
    const filtered = chips.filter((c): c is string => Boolean(c));
    return filtered.length > 0 ? filtered : baseFallback.chips;
  };

  switch (data.type) {
    case 'Culture': {
      const d = data as CultureNodeData;
      const heroValue = d.hemisphere || d.calendar_system || d.work_week_start;
      return {
        heroValue: heroValue || baseFallback.heroValue,
        chips: filterChips([
          d.work_week_start ? `Week: ${d.work_week_start}` : undefined,
          d.calendar_system,
          d.holidays_count !== undefined ? `${d.holidays_count} holidays` : undefined,
        ]),
        description: data.description,
      };
    }
    case 'Style': {
      const d = data as StyleNodeData;
      const heroValue = d.default_formality || d.directness_level;
      return {
        heroValue: heroValue || baseFallback.heroValue,
        chips: filterChips([d.default_pronoun, d.directness_level, d.humor_style]),
        gauge: d.formality_score !== undefined ? { value: d.formality_score, label: 'Formality' } : undefined,
        description: data.description,
      };
    }
    case 'Formatting': {
      const d = data as FormattingNodeData;
      const separators = [d.decimal_separator, d.thousands_separator].filter(Boolean).join('/');
      const heroValue = d.currency_code ? `${d.currency_symbol || ''}${d.currency_code}` : d.date_format;
      return {
        heroValue: heroValue || baseFallback.heroValue,
        chips: filterChips([
          separators || undefined,
          d.date_format,
          d.time_format,
        ]),
        description: data.description,
      };
    }
    case 'Adaptation': {
      const d = data as AdaptationNodeData;
      const heroValue = d.idiom_preference || d.metaphor_handling;
      return {
        heroValue: heroValue || baseFallback.heroValue,
        chips: filterChips([d.metaphor_handling, d.technical_terms, d.facts_strategy]),
        description: data.description,
      };
    }
    case 'Slugification': {
      const d = data as SlugificationNodeData;
      const heroValue = d.slug_rule ? SLUG_RULE_LABELS[d.slug_rule] : undefined;
      return {
        heroValue: heroValue || baseFallback.heroValue,
        chips: filterChips([
          d.allow_diacritics ? 'Diacritics ✓' : d.allow_diacritics === false ? 'No Diacritics' : undefined,
          d.max_length !== undefined ? `Max ${d.max_length}` : undefined,
          d.transliteration_enabled ? 'Translit ✓' : undefined,
        ]),
        description: data.description,
      };
    }
    case 'Market': {
      const d = data as MarketNodeData;
      const heroValue = d.ecommerce_maturity;
      return {
        heroValue: heroValue || baseFallback.heroValue,
        chips: filterChips([
          d.mobile_first ? 'Mobile First' : undefined,
          ...(d.popular_payment_methods?.slice(0, 2) || []),
        ]),
        gauge: d.digital_adoption !== undefined ? { value: d.digital_adoption, label: 'Digital' } : undefined,
        description: data.description,
      };
    }
    default:
      return baseFallback;
  }
}

// =============================================================================
// Main Component
// =============================================================================

export const LocaleSettingsCardContent = memo(function LocaleSettingsCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = false, // Default false for cleaner layout
}: LocaleSettingsCardContentProps) {
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;
  const IconComponent = TYPE_ICONS[data.type];
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;

  // Extract type-specific content
  const typeContent = useMemo(() => extractTypeContent(data), [data]);

  // Animation state
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Card variants - SUBTLE, NO SCALE to prevent layout shift
  const cardVariants = {
    idle: { y: 0 },
    hover: { y: -1, transition: SPRING_CONFIGS.gentle },
    selected: { y: -2, transition: SPRING_CONFIGS.smooth },
  };

  // v0.13.1 UX: DRAMATIC background gradient
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

  // v0.13.1 UX: Multi-layer card shadow
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

  // Icon glow style
  const iconStyle = useMemo(
    () => ({
      color: colors.primary,
      filter: `drop-shadow(0 0 ${selected ? '14px' : '8px'} ${colors.primary})`,
    }),
    [colors.primary, selected]
  );

  // Icon zone radial glow
  const iconZoneStyle = useMemo(
    () => ({
      background: `radial-gradient(circle at center, ${colors.primary}35 0%, ${colors.primary}15 40%, transparent 70%)`,
    }),
    [colors.primary]
  );

  // Separator glow
  const separatorStyle = useMemo(
    () => ({
      background: `linear-gradient(180deg, transparent 0%, ${colors.primary}60 50%, transparent 100%)`,
      boxShadow: `0 0 12px ${colors.primary}40`,
    }),
    [colors.primary]
  );

  // Wrapper
  const CardWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <CardWrapper
      className="relative flex h-full rounded-xl overflow-hidden"
      style={{
        ...backgroundStyle,
        ...cardShadowStyle,
        border: `2px double ${colors.primary}${selected ? '80' : '40'}`,
        minHeight: 100,
      }}
      {...(animationsEnabled && {
        variants: cardVariants,
        initial: 'idle',
        animate: animationState,
      })}
    >
      {/* Glow effect */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={colors.primary}
          intensity={selected ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Border beam */}
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

      {/* ICON ZONE (left) */}
      <div
        className="relative flex items-center justify-center shrink-0 z-10"
        style={{ width: 60, ...iconZoneStyle }}
      >
        <IconComponent
          size={28}
          strokeWidth={1.8}
          className={cn(
            'transition-transform duration-200',
            (selected || isHovered) && 'scale-110'
          )}
          style={iconStyle}
        />
      </div>

      {/* VERTICAL SEPARATOR */}
      <div className="w-[2px] shrink-0 self-stretch my-2 z-10" style={separatorStyle} />

      {/* CONTENT ZONE (right) */}
      <div className="relative flex-1 px-3 py-2.5 min-w-0 z-10 flex flex-col">
        {/* HEADER: Type label + Layer badge */}
        <div className="flex items-center justify-between mb-1.5">
          {useTaxonomyBadge ? (
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
          ) : (
            <span
              className="text-[10px] font-bold uppercase tracking-wider"
              style={{ color: colors.primary }}
            >
              {TYPE_LABELS[data.type]}
            </span>
          )}

          {/* Layer badge */}
          <div
            className={cn(
              'inline-flex items-center gap-1 px-1.5 py-0.5 rounded-full shrink-0',
              'text-[8px] font-bold uppercase tracking-wider'
            )}
            style={{
              background: `linear-gradient(135deg, ${colors.primary}30 0%, ${colors.primary}15 100%)`,
              border: `1px solid ${colors.primary}50`,
              color: colors.primary,
              boxShadow: `0 0 8px ${colors.primary}20`,
            }}
          >
            <span
              className={cn('w-1 h-1 rounded-full', selected && 'animate-pulse')}
              style={{ background: colors.primary, boxShadow: `0 0 4px ${colors.primary}` }}
            />
            locale
          </div>
        </div>

        {/* HERO VALUE - Type-specific primary value - FIXED SIZE */}
        {typeContent.heroValue && (
          <h3
            className="text-xl font-bold text-white tracking-wide capitalize whitespace-nowrap"
            style={{
              textShadow: selected
                ? `0 0 24px ${colors.primary}80, 0 0 48px ${colors.primary}40`
                : isHovered
                  ? `0 0 18px ${colors.primary}60, 0 0 36px ${colors.primary}25`
                  : `0 0 10px ${colors.primary}30`,
            }}
          >
            {typeContent.heroValue}
          </h3>
        )}

        {/* GAUGE (if present) */}
        {typeContent.gauge && (
          <div className="mt-1.5 mb-1">
            <MiniGauge
              value={typeContent.gauge.value}
              label={typeContent.gauge.label}
              color={colors.primary}
            />
          </div>
        )}

        {/* CHIPS - Type-specific properties */}
        {typeContent.chips.some(Boolean) && (
          <div className={cn('flex flex-wrap mt-1.5', gapTokens.compact)}>
            {typeContent.chips
              .filter((c): c is string => Boolean(c))
              .slice(0, 3)
              .map((chip, i) => (
                <Chip key={i} color={colors.primary} variant={i === 0 ? 'accent' : 'default'}>
                  {chip}
                </Chip>
              ))}
          </div>
        )}

        {/* SPACER */}
        <div className="flex-1 min-h-1" />

        {/* DIVIDER */}
        <div
          className="h-px mt-2 mb-1.5"
          style={{
            background: `linear-gradient(90deg, ${colors.primary}50, transparent)`,
            boxShadow: `0 0 6px ${colors.primary}20`,
          }}
        />

        {/* FOOTER: Key */}
        <div className="text-xs text-white/60 truncate font-mono">
          {data.key}
        </div>
      </div>
    </CardWrapper>
  );
});
