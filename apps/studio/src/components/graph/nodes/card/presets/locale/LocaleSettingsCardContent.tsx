'use client';

/**
 * LocaleSettingsCardContent - Unified card for Locale Settings nodes
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
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ 🌍 CULTURE            northern       │ <- Type icon + key property
 * │ ═════════════════════════════════   │ <- Double line (imported)
 * │ fr-FR                                │
 * │ ┌────────────────────────────────┐  │
 * │ │ ◉ Hemisphere: Northern         │  │ <- Type-specific details
 * │ │ ◉ Work Week: Monday            │  │
 * │ └────────────────────────────────┘  │
 * │ ◉ locale                            │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
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
import { GlowEffect } from '../../effects';

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
// Helper Components
// =============================================================================

interface PropertyRowProps {
  label: string;
  value: string | number | boolean | undefined;
  color: string;
}

const PropertyRow = memo(function PropertyRow({ label, value, color }: PropertyRowProps) {
  if (value === undefined || value === null) return null;

  const displayValue = typeof value === 'boolean'
    ? (value ? 'Yes' : 'No')
    : String(value);

  return (
    <div className="flex items-center justify-between text-[10px]">
      <span className="text-white/50">{label}:</span>
      <span style={{ color }} className="font-medium">
        {displayValue}
      </span>
    </div>
  );
});

interface FormalityGaugeProps {
  score: number;
  color: string;
}

const FormalityGauge = memo(function FormalityGauge({ score, color }: FormalityGaugeProps) {
  const percentage = Math.min(100, Math.max(0, score));

  return (
    <div className="mb-2">
      <div className="flex items-center justify-between text-[9px] mb-1">
        <span className="text-white/50">Formality</span>
        <span style={{ color }} className="font-semibold">{score}%</span>
      </div>
      <div className="h-1.5 rounded-full bg-white/10 overflow-hidden">
        <div
          className="h-full rounded-full transition-all duration-300"
          style={{
            width: `${percentage}%`,
            background: `linear-gradient(90deg, ${color}40, ${color})`,
          }}
        />
      </div>
    </div>
  );
});

interface SlugRuleBadgeProps {
  rule: string;
  color: string;
}

const SlugRuleBadge = memo(function SlugRuleBadge({ rule, color }: SlugRuleBadgeProps) {
  const RULE_LABELS: Record<string, string> = {
    latin_strip: 'Strip Diacritics',
    latin_preserve: 'Preserve Diacritics',
    native_script: 'Native Script',
    latin_transform: 'Transform (ü→ue)',
    transliterate: 'Transliterate',
  };

  return (
    <span
      className="px-2 py-0.5 rounded text-[9px] font-semibold"
      style={{
        background: `${color}20`,
        color: color,
        border: `1px solid ${color}40`,
      }}
    >
      {RULE_LABELS[rule] || rule}
    </span>
  );
});

interface MaturityBadgeProps {
  maturity: string;
  color: string;
}

const MaturityBadge = memo(function MaturityBadge({ maturity, color }: MaturityBadgeProps) {
  const MATURITY_COLORS: Record<string, string> = {
    emerging: '#f59e0b',
    growing: '#22c55e',
    mature: '#3b82f6',
    advanced: '#8b5cf6',
  };

  const maturityColor = MATURITY_COLORS[maturity] || color;

  return (
    <span
      className="px-2 py-0.5 rounded text-[9px] font-semibold"
      style={{
        background: `${maturityColor}20`,
        color: maturityColor,
      }}
    >
      {maturity.charAt(0).toUpperCase() + maturity.slice(1)}
    </span>
  );
});

// =============================================================================
// Type-specific content renderers
// =============================================================================

const renderCultureContent = (data: CultureNodeData, colors: { primary: string }) => (
  <>
    <PropertyRow label="Hemisphere" value={data.hemisphere} color={colors.primary} />
    <PropertyRow label="Work Week" value={data.work_week_start} color={colors.primary} />
    <PropertyRow label="Calendar" value={data.calendar_system} color={colors.primary} />
    {data.holidays_count !== undefined && (
      <PropertyRow label="Holidays" value={`${data.holidays_count} holidays`} color={colors.primary} />
    )}
  </>
);

const renderStyleContent = (data: StyleNodeData, colors: { primary: string }) => (
  <>
    {data.formality_score !== undefined && (
      <FormalityGauge score={data.formality_score} color={colors.primary} />
    )}
    <PropertyRow label="Formality" value={data.default_formality} color={colors.primary} />
    <PropertyRow label="Pronoun" value={data.default_pronoun} color={colors.primary} />
    <PropertyRow label="Directness" value={data.directness_level} color={colors.primary} />
    {data.humor_style && (
      <PropertyRow label="Humor" value={data.humor_style} color={colors.primary} />
    )}
  </>
);

const renderFormattingContent = (data: FormattingNodeData, colors: { primary: string }) => (
  <>
    <div className="grid grid-cols-2 gap-2 mb-2">
      {data.decimal_separator && (
        <div className="text-center p-1.5 rounded" style={{ background: `${colors.primary}10` }}>
          <span className="text-[8px] text-white/40 block">Decimal</span>
          <span className="text-sm font-mono" style={{ color: colors.primary }}>
            {data.decimal_separator}
          </span>
        </div>
      )}
      {data.thousands_separator && (
        <div className="text-center p-1.5 rounded" style={{ background: `${colors.primary}10` }}>
          <span className="text-[8px] text-white/40 block">Thousands</span>
          <span className="text-sm font-mono" style={{ color: colors.primary }}>
            {data.thousands_separator || 'space'}
          </span>
        </div>
      )}
    </div>
    <PropertyRow label="Date" value={data.date_format} color={colors.primary} />
    <PropertyRow label="Time" value={data.time_format} color={colors.primary} />
    {data.currency_code && (
      <PropertyRow
        label="Currency"
        value={`${data.currency_symbol || ''} ${data.currency_code}`}
        color={colors.primary}
      />
    )}
  </>
);

const renderAdaptationContent = (data: AdaptationNodeData, colors: { primary: string }) => (
  <>
    <PropertyRow label="Idioms" value={data.idiom_preference} color={colors.primary} />
    <PropertyRow label="Metaphors" value={data.metaphor_handling} color={colors.primary} />
    <PropertyRow label="Tech Terms" value={data.technical_terms} color={colors.primary} />
    {data.facts_strategy && (
      <div className="mt-2 p-2 rounded text-[9px]" style={{ background: `${colors.primary}10` }}>
        <span className="text-white/50">FACTS:</span>{' '}
        <span className="text-white/80">{data.facts_strategy}</span>
      </div>
    )}
  </>
);

const renderSlugificationContent = (data: SlugificationNodeData, colors: { primary: string }) => (
  <>
    {data.slug_rule && (
      <div className="mb-2">
        <SlugRuleBadge rule={data.slug_rule} color={colors.primary} />
      </div>
    )}
    <PropertyRow label="Diacritics" value={data.allow_diacritics} color={colors.primary} />
    <PropertyRow label="Stopwords" value={data.stopwords_count} color={colors.primary} />
    <PropertyRow label="Max Length" value={data.max_length} color={colors.primary} />
    <PropertyRow label="Transliterate" value={data.transliteration_enabled} color={colors.primary} />
  </>
);

const renderMarketContent = (data: MarketNodeData, colors: { primary: string }) => (
  <>
    {data.ecommerce_maturity && (
      <div className="mb-2">
        <MaturityBadge maturity={data.ecommerce_maturity} color={colors.primary} />
      </div>
    )}
    {data.digital_adoption !== undefined && (
      <div className="mb-2">
        <div className="flex items-center justify-between text-[9px] mb-1">
          <span className="text-white/50">Digital Adoption</span>
          <span style={{ color: colors.primary }} className="font-semibold">
            {data.digital_adoption}%
          </span>
        </div>
        <div className="h-1.5 rounded-full bg-white/10 overflow-hidden">
          <div
            className="h-full rounded-full"
            style={{
              width: `${data.digital_adoption}%`,
              background: `linear-gradient(90deg, ${colors.primary}40, ${colors.primary})`,
            }}
          />
        </div>
      </div>
    )}
    <PropertyRow label="Mobile First" value={data.mobile_first} color={colors.primary} />
    {data.popular_payment_methods && data.popular_payment_methods.length > 0 && (
      <div className="mt-2">
        <span className="text-[8px] text-white/40 block mb-1">Payment Methods:</span>
        <div className={cn('flex flex-wrap', gapTokens.compact)}>
          {data.popular_payment_methods.slice(0, 3).map((method) => (
            <span
              key={method}
              className="px-1 py-0.5 rounded text-[8px]"
              style={{ background: `${colors.primary}15`, color: colors.primary }}
            >
              {method}
            </span>
          ))}
        </div>
      </div>
    )}
  </>
);

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
  showTaxonomyBadge = true,
}: LocaleSettingsCardContentProps) {
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;
  const IconComponent = TYPE_ICONS[data.type];

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

  const showGlow = performanceConfig?.effects?.outerGlow ?? true;

  // Render type-specific content
  const renderContent = () => {
    switch (data.type) {
      case 'Culture':
        return renderCultureContent(data as CultureNodeData, colors);
      case 'Style':
        return renderStyleContent(data as StyleNodeData, colors);
      case 'Formatting':
        return renderFormattingContent(data as FormattingNodeData, colors);
      case 'Adaptation':
        return renderAdaptationContent(data as AdaptationNodeData, colors);
      case 'Slugification':
        return renderSlugificationContent(data as SlugificationNodeData, colors);
      case 'Market':
        return renderMarketContent(data as MarketNodeData, colors);
      default:
        return null;
    }
  };

  return (
    <div className="relative px-4 py-4">
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

      {/* Header: TaxonomyBadge or Icon + Type Label */}
      {useTaxonomyBadge ? (
        <div className="mb-2">
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
        <div className="flex items-center justify-between mb-2">
          <div className={cn('flex items-center', gapTokens.default)}>
            <IconComponent
              size={18}
              strokeWidth={2}
              className={cn(
                'transition-transform duration-200',
                (selected || isHovered) && 'scale-110'
              )}
              style={{
                color: colors.primary,
                filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}80)`,
              }}
            />
            <span
              className="text-[9px] font-bold uppercase tracking-widest font-mono"
              style={{ color: colors.primary }}
            >
              {TYPE_LABELS[data.type]}
            </span>
          </div>

          {/* Type description badge */}
          <span
            className="px-1.5 py-0.5 rounded text-[8px] font-medium"
            style={{
              background: `${colors.primary}15`,
              color: `${colors.primary}cc`,
            }}
          >
            {TYPE_DESCRIPTIONS[data.type]}
          </span>
        </div>
      )}

      {/* Double line separator (imported trait) */}
      <div className="mb-3">
        <div
          className="h-[2px] mb-[2px]"
          style={{
            background: `linear-gradient(90deg, ${colors.primary}60, ${colors.primary}20, transparent)`,
          }}
        />
        <div
          className="h-[1px]"
          style={{
            background: `linear-gradient(90deg, ${colors.primary}40, transparent)`,
          }}
        />
      </div>

      {/* Key */}
      <h3
        className="text-sm font-semibold text-white mb-2 truncate"
        style={glowStyle}
      >
        {data.key}
      </h3>

      {/* Display name */}
      <p className="text-xs text-white/70 mb-3 truncate">{data.displayName}</p>

      {/* Type-specific content box */}
      <div
        className="p-2 rounded-lg mb-2"
        style={{
          backgroundColor: `${colors.primary}08`,
          border: `1px solid ${colors.primary}20`,
        }}
      >
        {renderContent()}
      </div>

      {/* Description */}
      {data.description && (
        <p className="text-[10px] text-white/50 line-clamp-2 mb-2">
          {data.description}
        </p>
      )}

      {/* Divider */}
      <div
        className="h-px my-2"
        style={{
          background: `linear-gradient(90deg, transparent, ${colors.primary}30, transparent)`,
        }}
      />

      {/* Layer badge */}
      <div className="flex justify-center">
        <div
          className={cn(
            'inline-flex items-center px-1.5 py-0.5 rounded-full',
            'text-[8px] font-semibold uppercase tracking-wider border',
            gapTokens.compact
          )}
          style={{
            background: `${colors.primary}15`,
            borderColor: `${colors.primary}35`,
            color: colors.primary,
          }}
        >
          <span
            className={cn('w-1 h-1 rounded-full', selected && 'animate-pulse')}
            style={{
              background: colors.primary,
              boxShadow: `0 0 4px ${colors.primary}`,
            }}
          />
          locale
        </div>
      </div>
    </div>
  );
});
