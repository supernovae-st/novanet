'use client';

/**
 * KnowledgeHelpers - Shared helper components for Knowledge Layer cards
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> double
 *
 * Helper components for Term, Expression, and SEOKeyword cards.
 */

import { memo, useMemo } from 'react';
import { motion } from 'motion/react';
// cn reserved for future use
// import { cn } from '@/lib/utils';
import { DURATIONS } from '../../animationPresets';

// =============================================================================
// Domain Badge - Shows vocabulary domain for Terms
// =============================================================================

const domainColors: Record<string, string> = {
  pricing: '#22c55e',      // green
  features: '#3b82f6',     // blue
  technical: '#6366f1',    // indigo
  marketing: '#f59e0b',    // amber
  support: '#8b5cf6',      // purple
  legal: '#64748b',        // slate
  general: '#94a3b8',      // neutral
};

export interface DomainBadgeProps {
  domain: string;
  animated?: boolean;
}

export const DomainBadge = memo(function DomainBadge({
  domain,
  animated = false,
}: DomainBadgeProps) {
  const color = domainColors[domain] || domainColors.general;

  const BadgeWrapper = animated ? motion.span : 'span';

  return (
    <BadgeWrapper
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono font-bold uppercase tracking-wider"
      style={{
        backgroundColor: `${color}20`,
        color: color,
        border: `1px solid ${color}40`,
      }}
      {...(animated && {
        whileHover: { scale: 1.05 },
        transition: { duration: DURATIONS.fast },
      })}
    >
      {domain}
    </BadgeWrapper>
  );
});

// =============================================================================
// Register Badge - Shows language register (formal/neutral/casual/technical)
// =============================================================================

const registerIcons: Record<string, string> = {
  formal: '▲',
  neutral: '●',
  casual: '▼',
  technical: '◆',
};

export interface RegisterBadgeProps {
  register: string;
  color: string;
}

export const RegisterBadge = memo(function RegisterBadge({
  register,
  color,
}: RegisterBadgeProps) {
  const icon = registerIcons[register] || '●';

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        backgroundColor: `${color}15`,
        color: `${color}cc`,
      }}
    >
      <span style={{ color }}>{icon}</span>
      {register}
    </span>
  );
});

// =============================================================================
// Tone Badge - Shows expression tone
// =============================================================================

const toneIcons: Record<string, string> = {
  formal: '◼',
  warm: '♥',
  casual: '☺',
  energetic: '⚡',
  empathetic: '♡',
  authoritative: '◆',
  friendly: '☀',
};

const toneColors: Record<string, string> = {
  formal: '#64748b',
  warm: '#f97316',
  casual: '#22c55e',
  energetic: '#eab308',
  empathetic: '#ec4899',
  authoritative: '#6366f1',
  friendly: '#3b82f6',
};

export interface ToneBadgeProps {
  tone: string;
  animated?: boolean;
}

export const ToneBadge = memo(function ToneBadge({
  tone,
  animated = false,
}: ToneBadgeProps) {
  const icon = toneIcons[tone] || '●';
  const color = toneColors[tone] || '#8b5cf6';

  const BadgeWrapper = animated ? motion.span : 'span';

  return (
    <BadgeWrapper
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-medium"
      style={{
        backgroundColor: `${color}20`,
        color: color,
      }}
      {...(animated && {
        whileHover: { scale: 1.05 },
        transition: { duration: DURATIONS.fast },
      })}
    >
      {icon} {tone}
    </BadgeWrapper>
  );
});

// =============================================================================
// Formality Indicator - Visual scale showing formality level
// =============================================================================

const formalityLevels: Record<string, number> = {
  very_formal: 5,
  formal: 4,
  neutral: 3,
  casual: 2,
  very_casual: 1,
};

export interface FormalityIndicatorProps {
  formality: string;
  color: string;
}

export const FormalityIndicator = memo(function FormalityIndicator({
  formality,
  color,
}: FormalityIndicatorProps) {
  const level = formalityLevels[formality] || 3;

  return (
    <div className="flex items-center gap-1">
      <span className="text-[8px] text-white/50">formality:</span>
      <div className="flex gap-0.5">
        {[1, 2, 3, 4, 5].map((i) => (
          <div
            key={i}
            className="w-1.5 h-2 rounded-sm"
            style={{
              backgroundColor: i <= level ? color : `${color}30`,
            }}
          />
        ))}
      </div>
    </div>
  );
});

// =============================================================================
// Use Case Badge - Shows expression use case
// =============================================================================

const useCaseIcons: Record<string, string> = {
  greeting: '👋',
  closing: '✓',
  apology: '◇',
  thanks: '★',
  request: '→',
  confirmation: '✓',
  warning: '⚠',
  celebration: '✦',
};

export interface UseCaseBadgeProps {
  useCase: string;
  color: string;
}

export const UseCaseBadge = memo(function UseCaseBadge({
  useCase,
  color,
}: UseCaseBadgeProps) {
  // Use text symbols instead of emoji
  const icon = useCaseIcons[useCase] || '●';

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        backgroundColor: `${color}15`,
        color: color,
      }}
    >
      <span>{icon}</span>
      {useCase.replace('_', ' ')}
    </span>
  );
});

// =============================================================================
// Volume Display - Shows search volume with visual scale
// =============================================================================

export interface VolumeDisplayProps {
  volume: number;
  color: string;
  animated?: boolean;
}

export const VolumeDisplay = memo(function VolumeDisplay({
  volume,
  color,
  animated = false,
}: VolumeDisplayProps) {
  const formattedVolume = useMemo(() => {
    if (volume >= 1000000) return `${(volume / 1000000).toFixed(1)}M`;
    if (volume >= 1000) return `${(volume / 1000).toFixed(1)}K`;
    return volume.toString();
  }, [volume]);

  // Scale: 0 = 0, 100K = max
  const scale = Math.min(Math.log10(Math.max(volume, 1)) / 5, 1);

  const VolumeWrapper = animated ? motion.div : 'div';

  return (
    <VolumeWrapper
      className="flex flex-col gap-1"
      {...(animated && {
        initial: { opacity: 0.8 },
        whileHover: { opacity: 1 },
      })}
    >
      <div className="flex items-center justify-between">
        <span className="text-[8px] text-white/50">volume</span>
        <span className="text-xs font-bold" style={{ color }}>
          {formattedVolume}
        </span>
      </div>
      <div className="h-1 rounded-full bg-white/10 overflow-hidden">
        <div
          className="h-full rounded-full"
          style={{
            width: `${scale * 100}%`,
            backgroundColor: color,
          }}
        />
      </div>
    </VolumeWrapper>
  );
});

// =============================================================================
// Difficulty Badge - Shows SEO difficulty score (0-100)
// =============================================================================

export interface DifficultyBadgeProps {
  difficulty: number;
  color: string;
}

export const DifficultyBadge = memo(function DifficultyBadge({
  difficulty,
  color: _color,
}: DifficultyBadgeProps) {
  // Color coding: green (easy) -> yellow (medium) -> red (hard)
  const diffColor =
    difficulty <= 30
      ? '#22c55e'
      : difficulty <= 60
        ? '#eab308'
        : '#ef4444';

  return (
    <div className="flex items-center gap-1.5">
      <span className="text-[8px] text-white/50">difficulty:</span>
      <span
        className="text-xs font-bold px-1 py-0.5 rounded"
        style={{
          backgroundColor: `${diffColor}20`,
          color: diffColor,
        }}
      >
        {difficulty}
      </span>
    </div>
  );
});

// =============================================================================
// Traffic Potential Display
// =============================================================================

export interface TrafficPotentialProps {
  trafficPotential: number;
  color: string;
}

export const TrafficPotential = memo(function TrafficPotential({
  trafficPotential,
  color,
}: TrafficPotentialProps) {
  const formatted = useMemo(() => {
    if (trafficPotential >= 1000000)
      return `${(trafficPotential / 1000000).toFixed(1)}M`;
    if (trafficPotential >= 1000)
      return `${(trafficPotential / 1000).toFixed(1)}K`;
    return trafficPotential.toString();
  }, [trafficPotential]);

  return (
    <div className="flex items-center gap-1.5">
      <span className="text-[8px] text-white/50">traffic potential:</span>
      <span className="text-xs font-bold" style={{ color }}>
        {formatted}
      </span>
    </div>
  );
});

// =============================================================================
// Intent Badge - Shows search intent
// =============================================================================

const intentIcons: Record<string, string> = {
  transactional: '◆',
  informational: '◇',
  navigational: '→',
  commercial: '★',
};

const intentColors: Record<string, string> = {
  transactional: '#22c55e',
  informational: '#3b82f6',
  navigational: '#f59e0b',
  commercial: '#8b5cf6',
};

export interface IntentBadgeProps {
  intent: string;
}

export const IntentBadge = memo(function IntentBadge({
  intent,
}: IntentBadgeProps) {
  const icon = intentIcons[intent] || '●';
  const color = intentColors[intent] || '#8b5cf6';

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-medium uppercase tracking-wider"
      style={{
        backgroundColor: `${color}20`,
        color: color,
      }}
    >
      {icon} {intent}
    </span>
  );
});

// =============================================================================
// SERP Features Display
// =============================================================================

const serpFeatureIcons: Record<string, string> = {
  featured_snippet: '◆',
  ai_overview: '◈',
  people_also_ask: '?',
  images: '▣',
  videos: '▶',
  knowledge_panel: '◫',
  local_pack: '◉',
  shopping: '◇',
};

export interface SerpFeaturesProps {
  features: string[];
  color: string;
}

export const SerpFeatures = memo(function SerpFeatures({
  features,
  color,
}: SerpFeaturesProps) {
  if (!features || features.length === 0) return null;

  return (
    <div className="flex flex-wrap gap-1">
      {features.map((feature) => {
        const icon = serpFeatureIcons[feature] || '●';
        return (
          <span
            key={feature}
            className="px-1 py-0.5 rounded text-[7px] font-mono"
            style={{
              backgroundColor: `${color}15`,
              color: `${color}cc`,
            }}
            title={feature.replace('_', ' ')}
          >
            {icon}
          </span>
        );
      })}
    </div>
  );
});

// =============================================================================
// Trend Badge
// =============================================================================

const trendIcons: Record<string, string> = {
  rising: '↑',
  stable: '→',
  declining: '↓',
};

const trendColors: Record<string, string> = {
  rising: '#22c55e',
  stable: '#f59e0b',
  declining: '#ef4444',
};

export interface TrendBadgeProps {
  trend: string;
}

export const TrendBadge = memo(function TrendBadge({ trend }: TrendBadgeProps) {
  const icon = trendIcons[trend] || '→';
  const color = trendColors[trend] || '#f59e0b';

  return (
    <span
      className="inline-flex items-center gap-0.5 px-1.5 py-0.5 rounded text-[8px] font-bold"
      style={{
        backgroundColor: `${color}20`,
        color: color,
      }}
    >
      {icon} {trend}
    </span>
  );
});

// =============================================================================
// Synonyms List
// =============================================================================

export interface SynonymsListProps {
  synonyms: string[];
  color: string;
  maxShow?: number;
}

export const SynonymsList = memo(function SynonymsList({
  synonyms,
  color,
  maxShow = 3,
}: SynonymsListProps) {
  if (!synonyms || synonyms.length === 0) return null;

  const displaySynonyms = synonyms.slice(0, maxShow);
  const remaining = synonyms.length - maxShow;

  return (
    <div className="flex flex-wrap gap-1 items-center">
      <span className="text-[8px] text-white/50">synonyms:</span>
      {displaySynonyms.map((synonym, i) => (
        <span
          key={i}
          className="px-1 py-0.5 rounded text-[8px] font-mono"
          style={{
            backgroundColor: `${color}15`,
            color: `${color}cc`,
          }}
        >
          {synonym}
        </span>
      ))}
      {remaining > 0 && (
        <span className="text-[8px] text-white/40">+{remaining}</span>
      )}
    </div>
  );
});

// =============================================================================
// Channel Badges
// =============================================================================

export interface ChannelBadgesProps {
  channels: string[];
  color: string;
}

export const ChannelBadges = memo(function ChannelBadges({
  channels,
  color,
}: ChannelBadgesProps) {
  if (!channels || channels.length === 0) return null;

  return (
    <div className="flex flex-wrap gap-1 items-center">
      <span className="text-[8px] text-white/50">channels:</span>
      {channels.map((channel, i) => (
        <span
          key={i}
          className="px-1 py-0.5 rounded text-[7px] font-mono uppercase"
          style={{
            backgroundColor: `${color}10`,
            color: `${color}aa`,
            border: `1px solid ${color}30`,
          }}
        >
          {channel.replace('_', ' ')}
        </span>
      ))}
    </div>
  );
});

// =============================================================================
// Part of Speech Badge
// =============================================================================

export interface PartOfSpeechBadgeProps {
  partOfSpeech: string;
  color: string;
}

export const PartOfSpeechBadge = memo(function PartOfSpeechBadge({
  partOfSpeech,
  color,
}: PartOfSpeechBadgeProps) {
  return (
    <span
      className="px-1 py-0.5 rounded text-[7px] font-mono italic"
      style={{
        backgroundColor: `${color}10`,
        color: `${color}99`,
      }}
    >
      {partOfSpeech}
    </span>
  );
});
