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
import { cn } from '@/lib/utils';
import { DURATIONS } from '../../animationPresets';

// =============================================================================
// PillarBadge - Shows if entity is a pillar
// =============================================================================

export interface PillarBadgeProps {
  isPillar: boolean;
  color?: string;
}

export const PillarBadge = memo(function PillarBadge({
  isPillar,
  color = '#f97316',
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

const curationConfig = {
  human_authored: {
    label: 'Human',
    icon: '✍️',
    color: '#22c55e',
  },
  machine_translated: {
    label: 'MT',
    icon: '🤖',
    color: '#eab308',
  },
  ai_generated: {
    label: 'AI',
    icon: '🧠',
    color: '#a855f7',
  },
  ai_generated_reviewed: {
    label: 'AI+Rev',
    icon: '✅',
    color: '#06b6d4',
  },
};

export const CurationBadge = memo(function CurationBadge({
  status,
  animate = true,
}: CurationBadgeProps) {
  const config = curationConfig[status];
  const Badge = animate ? motion.span : 'span';

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
      <span>{config.icon}</span>
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

// Simple locale to flag mapping
const localeFlags: Record<string, string> = {
  'en-US': '🇺🇸',
  'en-GB': '🇬🇧',
  'fr-FR': '🇫🇷',
  'de-DE': '🇩🇪',
  'es-ES': '🇪🇸',
  'es-MX': '🇲🇽',
  'pt-BR': '🇧🇷',
  'ja-JP': '🇯🇵',
  'ko-KR': '🇰🇷',
  'zh-CN': '🇨🇳',
  'it-IT': '🇮🇹',
  'nl-NL': '🇳🇱',
  'ar-SA': '🇸🇦',
  'ru-RU': '🇷🇺',
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
            <span style={{ color }}>✓</span>
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
          <span className="text-white/60">
            <span style={{ color }}>✓</span> {benefitsCount}
          </span>
        )}
        {examplesCount > 0 && (
          <span className="text-white/60">
            <span style={{ color }}>📝</span> {examplesCount}
          </span>
        )}
      </div>
      <span className="text-white/40">v{version}</span>
    </div>
  );
});
