'use client';

/**
 * Semantic Layer Helper Components
 *
 * Shared components for Entity and EntityNative cards.
 * Layer color: Orange #f97316
 *
 * Components:
 * - PillarBadge: Shows pillar entity status
 * - SchemaOrgBadge: Shows schema.org type
 * - CurationBadge: Shows content curation status
 * - LocaleBadge: Shows locale with flag emoji
 * - SemanticLinkCounter: Shows semantic relationship counts
 * - BenefitsList: Expandable benefits preview
 */

import { memo, useState } from 'react';
import { motion, type Variants } from 'motion/react';
// import { cn } from '@/lib/utils';
import { DURATIONS } from '../../animationPresets';
import { Pencil, Bot, Brain, CheckCircle, FileText, Check } from 'lucide-react';

// =============================================================================
// Neo4j Integer Helper
// =============================================================================

/**
 * Convert Neo4j Integer {low, high} to JavaScript number
 * Neo4j uses 64-bit integers which are returned as objects
 *
 * WARNING: For integers > 2^31-1, only the low 32 bits are used.
 * If high !== 0, we log a warning (data may be incorrect).
 */
export function toNumber(value: unknown): number {
  if (value === undefined || value === null) return 0;
  if (typeof value === 'number') return value;
  // Neo4j Integer type: {low: number, high: number}
  if (typeof value === 'object' && 'low' in value && 'high' in value) {
    const neo4jInt = value as { low: number; high: number };
    if (neo4jInt.high !== 0) {
      console.warn(
        `[toNumber] Neo4j integer has non-zero high bits (${neo4jInt.high}), value may be truncated`
      );
    }
    return neo4jInt.low;
  }
  return Number(value) || 0;
}

// =============================================================================
// PillarBadge - Shows if entity is a pillar
// =============================================================================

export interface PillarBadgeProps {
  isPillar: boolean;
  color?: string;
}

export const PillarBadge = memo(function PillarBadge({
  isPillar,
  color: _color = '#f97316',
}: PillarBadgeProps) {
  if (!isPillar) return null;

  return (
    <span
      className="px-1.5 py-0.5 rounded text-[9px] font-mono font-bold"
      style={{
        color: '#fbbf24',
        backgroundColor: 'rgba(251, 191, 36, 0.15)',
        border: '1px solid rgba(251, 191, 36, 0.4)',
      }}
    >
      ★ PILLAR
    </span>
  );
});

// =============================================================================
// SchemaOrgBadge - Shows schema.org type
// =============================================================================

export interface SchemaOrgBadgeProps {
  type: string;
  color?: string;
}

export const SchemaOrgBadge = memo(function SchemaOrgBadge({
  type,
  color = '#f97316',
}: SchemaOrgBadgeProps) {
  return (
    <span
      className="px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        color: color,
        backgroundColor: `${color}15`,
        border: `1px solid ${color}30`,
      }}
    >
      schema:{type}
    </span>
  );
});

// =============================================================================
// CurationBadge - Shows content curation status
// =============================================================================

export interface CurationBadgeProps {
  status: 'human_authored' | 'machine_translated' | 'ai_generated' | 'ai_generated_reviewed';
  animate?: boolean;
}

// ADR-013: Use Lucide icons, not emoji
const curationConfig = {
  human_authored: {
    label: 'Human',
    Icon: Pencil,
    color: '#22c55e',
  },
  machine_translated: {
    label: 'MT',
    Icon: Bot,
    color: '#eab308',
  },
  ai_generated: {
    label: 'AI',
    Icon: Brain,
    color: '#a855f7',
  },
  ai_generated_reviewed: {
    label: 'AI+Rev',
    Icon: CheckCircle,
    color: '#06b6d4',
  },
};

export const CurationBadge = memo(function CurationBadge({
  status,
  animate = true,
}: CurationBadgeProps) {
  const config = curationConfig[status];
  const Badge = animate ? motion.span : 'span';
  const { Icon } = config;

  return (
    <Badge
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        color: config.color,
        backgroundColor: `${config.color}15`,
        border: `1px solid ${config.color}30`,
      }}
      {...(animate && {
        whileHover: { scale: 1.05 },
        transition: { duration: DURATIONS.fast },
      })}
    >
      <Icon size={10} />
      <span>{config.label}</span>
    </Badge>
  );
});

// =============================================================================
// LocaleBadge - Shows locale with flag
// =============================================================================

export interface LocaleBadgeProps {
  locale: string;
  color?: string;
}

// Locale to flag emoji mapping (comprehensive BCP-47 coverage)
const localeFlags: Record<string, string> = {
  // English
  'en-US': '🇺🇸',
  'en-GB': '🇬🇧',
  'en-AU': '🇦🇺',
  'en-CA': '🇨🇦',
  'en-NZ': '🇳🇿',
  'en-IE': '🇮🇪',
  // French
  'fr-FR': '🇫🇷',
  'fr-CA': '🇨🇦',
  'fr-BE': '🇧🇪',
  'fr-CH': '🇨🇭',
  // German
  'de-DE': '🇩🇪',
  'de-AT': '🇦🇹',
  'de-CH': '🇨🇭',
  // Spanish
  'es-ES': '🇪🇸',
  'es-MX': '🇲🇽',
  'es-AR': '🇦🇷',
  'es-CO': '🇨🇴',
  'es-CL': '🇨🇱',
  // Portuguese
  'pt-BR': '🇧🇷',
  'pt-PT': '🇵🇹',
  // Asian
  'ja-JP': '🇯🇵',
  'ko-KR': '🇰🇷',
  'zh-CN': '🇨🇳',
  'zh-TW': '🇹🇼',
  'zh-HK': '🇭🇰',
  'vi-VN': '🇻🇳',
  'th-TH': '🇹🇭',
  'id-ID': '🇮🇩',
  'ms-MY': '🇲🇾',
  // European
  'it-IT': '🇮🇹',
  'nl-NL': '🇳🇱',
  'nl-BE': '🇧🇪',
  'pl-PL': '🇵🇱',
  'ru-RU': '🇷🇺',
  'uk-UA': '🇺🇦',
  'cs-CZ': '🇨🇿',
  'sk-SK': '🇸🇰',
  'hu-HU': '🇭🇺',
  'ro-RO': '🇷🇴',
  'el-GR': '🇬🇷',
  'bg-BG': '🇧🇬',
  'hr-HR': '🇭🇷',
  'sr-RS': '🇷🇸',
  'sl-SI': '🇸🇮',
  // Nordic
  'sv-SE': '🇸🇪',
  'da-DK': '🇩🇰',
  'nb-NO': '🇳🇴',
  'fi-FI': '🇫🇮',
  'is-IS': '🇮🇸',
  // Middle Eastern
  'ar-SA': '🇸🇦',
  'ar-AE': '🇦🇪',
  'ar-EG': '🇪🇬',
  'he-IL': '🇮🇱',
  'tr-TR': '🇹🇷',
  'fa-IR': '🇮🇷',
  // Indian subcontinent
  'hi-IN': '🇮🇳',
  'bn-BD': '🇧🇩',
  'ta-IN': '🇮🇳',
  'te-IN': '🇮🇳',
};

// Locale-specific accent colors for flag ribbons
// Each language family has a distinct primary color for easy visual differentiation
const localeColors: Record<string, string> = {
  // French family - Bleu France (distinct bright blue)
  'fr-FR': '#2563eb', // Bleu France
  'fr-CA': '#1d4ed8', // Bleu Québec (slightly darker)
  'fr-BE': '#3b82f6', // Bleu Belgique
  // English family - Red/Crimson
  'en-US': '#dc2626', // American red
  'en-GB': '#b91c1c', // British darker red
  'en-AU': '#fbbf24', // Australian gold
  // Spanish family - Warm yellow/gold
  'es-ES': '#ca8a04', // Spanish gold
  'es-MX': '#16a34a', // Mexican green
  'es-AR': '#0ea5e9', // Argentine sky blue
  // German family - Gray/Steel
  'de-DE': '#6b7280', // German steel gray
  'de-AT': '#dc2626', // Austrian red
  // Portuguese - Green
  'pt-BR': '#22c55e', // Brazilian green
  'pt-PT': '#16a34a', // Portuguese darker green
  // Asian languages - distinct per language
  'ja-JP': '#dc2626', // Japanese red sun
  'ko-KR': '#3b82f6', // Korean blue
  'zh-CN': '#dc2626', // Chinese red
  'zh-TW': '#22c55e', // Taiwanese green
  // Other European
  'it-IT': '#16a34a', // Italian green
  'nl-NL': '#f97316', // Dutch orange
  'pl-PL': '#dc2626', // Polish red
  'ru-RU': '#3b82f6', // Russian blue
  'uk-UA': '#fbbf24', // Ukrainian yellow
  // Middle Eastern
  'ar-SA': '#16a34a', // Saudi green
  'he-IL': '#3b82f6', // Israeli blue
  'tr-TR': '#dc2626', // Turkish red
};

export const LocaleBadge = memo(function LocaleBadge({
  locale,
  color = '#f97316',
}: LocaleBadgeProps) {
  const flag = localeFlags[locale] || '🌐';

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-mono"
      style={{
        color: color,
        backgroundColor: `${color}20`,
        border: `1px solid ${color}40`,
      }}
    >
      <span>{flag}</span>
      <span className="font-bold">{locale}</span>
    </span>
  );
});

// =============================================================================
// LocaleFlagRibbon - Diagonal corner ribbon with flag (Flag Badge Prominent)
// =============================================================================

export interface LocaleFlagRibbonProps {
  locale: string;
  animate?: boolean;
}

const ribbonVariants: Variants = {
  idle: { scale: 1, opacity: 0.95 },
  hover: { scale: 1.02, opacity: 1 },
  selected: {
    scale: 1.05,
    opacity: 1,
    boxShadow: '0 0 12px rgba(255, 255, 255, 0.3)',
  },
};

export const LocaleFlagRibbon = memo(function LocaleFlagRibbon({
  locale,
  animate = true,
}: LocaleFlagRibbonProps) {
  // Guard against undefined locale
  if (!locale) return null;

  const flag = localeFlags[locale] || '🌐';
  const accentColor = localeColors[locale] || '#6366f1';
  const localeCode = locale.split('-')[0]?.toUpperCase() || '??';
  const Ribbon = animate ? motion.div : 'div';

  return (
    <div className="absolute top-0 left-0 overflow-hidden w-24 h-24 pointer-events-none z-10">
      <Ribbon
        className="absolute top-5 -left-8 transform -rotate-45 w-32 text-center py-1.5 shadow-lg"
        style={{
          background: `linear-gradient(135deg, ${accentColor}, ${accentColor}dd)`,
          boxShadow: `0 2px 8px ${accentColor}60`,
        }}
        {...(animate && {
          variants: ribbonVariants,
          initial: 'idle',
          whileHover: 'hover',
        })}
      >
        <span className="text-lg">{flag}</span>
        <span className="ml-1 text-[10px] font-bold text-white/90 drop-shadow-sm">
          {localeCode}
        </span>
      </Ribbon>
    </div>
  );
});

// Helper to get flag for a locale
export function getLocaleFlag(locale: string): string {
  return localeFlags[locale] || '🌐';
}

// Helper to get accent color for a locale
export function getLocaleAccentColor(locale: string): string {
  return localeColors[locale] || '#6366f1';
}

// =============================================================================
// SemanticLinkCounter - Shows semantic relationship counts
// =============================================================================

export interface SemanticLinkCounterProps {
  incoming?: number;
  outgoing?: number;
  color?: string;
}

export const SemanticLinkCounter = memo(function SemanticLinkCounter({
  incoming = 0,
  outgoing = 0,
  color = '#f97316',
}: SemanticLinkCounterProps) {
  return (
    <div
      className="flex items-center gap-2 px-2 py-1 rounded text-[9px] font-mono"
      style={{
        backgroundColor: `${color}08`,
        border: `1px solid ${color}20`,
      }}
    >
      <span className="text-white/50">→</span>
      <span style={{ color }}>{outgoing}</span>
      <span className="text-white/30">|</span>
      <span className="text-white/50">←</span>
      <span style={{ color }}>{incoming}</span>
    </div>
  );
});

// =============================================================================
// StatusBadge - Shows publication status (draft/reviewed/published)
// =============================================================================

export interface StatusBadgeProps {
  status: 'draft' | 'reviewed' | 'published';
}

const statusConfig = {
  draft: {
    label: 'Draft',
    color: '#6b7280',
    bgColor: 'rgba(107, 114, 128, 0.15)',
  },
  reviewed: {
    label: 'Reviewed',
    color: '#eab308',
    bgColor: 'rgba(234, 179, 8, 0.15)',
  },
  published: {
    label: 'Published',
    color: '#22c55e',
    bgColor: 'rgba(34, 197, 94, 0.15)',
  },
};

export const StatusBadge = memo(function StatusBadge({ status }: StatusBadgeProps) {
  const config = statusConfig[status];

  return (
    <span
      className="px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        color: config.color,
        backgroundColor: config.bgColor,
        border: `1px solid ${config.color}40`,
      }}
    >
      {config.label}
    </span>
  );
});

// =============================================================================
// BenefitsList - Expandable benefits preview
// =============================================================================

export interface BenefitsListProps {
  benefits: string[];
  maxDisplay?: number;
  color?: string;
  animate?: boolean;
}

const benefitVariants: Variants = {
  hidden: { opacity: 0, x: -10 },
  visible: (i: number) => ({
    opacity: 1,
    x: 0,
    transition: { delay: i * 0.05, duration: DURATIONS.fast },
  }),
};

export const BenefitsList = memo(function BenefitsList({
  benefits,
  maxDisplay = 3,
  color = '#f97316',
  animate = true,
}: BenefitsListProps) {
  const [expanded, setExpanded] = useState(false);
  const displayBenefits = expanded ? benefits : benefits.slice(0, maxDisplay);
  const hasMore = benefits.length > maxDisplay;

  const ListItem = animate ? motion.li : 'li';

  return (
    <div className="space-y-1">
      <span className="text-[9px] text-white/50 font-mono">benefits:</span>
      <ul className="space-y-0.5">
        {displayBenefits.map((benefit, i) => (
          <ListItem
            key={i}
            className="flex items-start gap-1 text-[9px]"
            {...(animate && {
              custom: i,
              variants: benefitVariants,
              initial: 'hidden',
              animate: 'visible',
            })}
          >
            <Check size={9} style={{ color }} className="flex-shrink-0 mt-0.5" />
            <span className="text-white/70 line-clamp-1">{benefit}</span>
          </ListItem>
        ))}
      </ul>
      {hasMore && (
        <button
          onClick={() => setExpanded(!expanded)}
          className="text-[8px] font-mono hover:underline"
          style={{ color }}
        >
          {expanded ? '▲ show less' : `▼ +${benefits.length - maxDisplay} more`}
        </button>
      )}
    </div>
  );
});

// =============================================================================
// ContentStats - Shows content statistics
// =============================================================================

export interface ContentStatsProps {
  benefitsCount?: number;
  examplesCount?: number;
  version?: number;
  color?: string;
}

export const ContentStats = memo(function ContentStats({
  benefitsCount = 0,
  examplesCount = 0,
  version = 1,
  color = '#f97316',
}: ContentStatsProps) {
  return (
    <div
      className="flex items-center justify-between text-[9px] font-mono px-2 py-1 rounded"
      style={{
        backgroundColor: `${color}08`,
        border: `1px solid ${color}15`,
      }}
    >
      <div className="flex items-center gap-3">
        {benefitsCount > 0 && (
          <span className="flex items-center gap-1 text-white/60">
            <Check size={10} style={{ color }} />
            {benefitsCount}
          </span>
        )}
        {examplesCount > 0 && (
          <span className="flex items-center gap-1 text-white/60">
            <FileText size={10} style={{ color }} />
            {examplesCount}
          </span>
        )}
      </div>
      <span className="text-white/40">v{version}</span>
    </div>
  );
});
