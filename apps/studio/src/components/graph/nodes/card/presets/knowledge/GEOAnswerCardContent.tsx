'use client';

/**
 * GEOAnswerCardContent - "AI Response Snapshot" design for GEOAnswer nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> dotted
 * - Shows answer preview, brand mention status, competitors, quality
 *
 * Premium Effects (v0.13.1):
 * - GridPattern: AI response matrix background
 * - BorderBeam: Animated border with DOTTED style
 * - MotionTechCorners: AI response corners
 * - GlowEffect: Green (brand mentioned), Amber (competitors), Gray (absent)
 * - LightRays: Radiating answer effect on selection
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ ◎ GEO ANSWER           ● cited      │  <- AI icon + quality badge
 * │ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │  <- Dotted separator (retrieved)
 * │ geo-answer:qr-code-gen@claude-3      │
 * │ ┌────────────────────────────────┐   │
 * │ │ "Pour créer un QR code, vous    │   │  <- Answer preview
 * │ │ pouvez utiliser QRCode AI..."   │   │
 * │ │ ─────────────────────────────  │   │
 * │ │ ✓ Brand mentioned   Claude     │   │  <- Brand + Platform
 * │ │ ─────────────────────────────  │   │
 * │ │ Competitors: QR Monkey, Canva   │   │  <- Competitors list
 * │ │ ─────────────────────────────  │   │
 * │ │ Position: 2nd  |  Retrieved 2h  │   │  <- Position + Timestamp
 * │ └────────────────────────────────┘   │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { GlowEffect, GridPattern, BorderBeam, LightRays } from '../../effects';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { MotionTechCorners } from '../../../effects';

// =============================================================================
// Types
// =============================================================================

export type AnswerQuality = 'cited' | 'mentioned' | 'absent';

export type AIPlatform =
  | 'claude'
  | 'openai'
  | 'perplexity'
  | 'gemini'
  | 'copilot'
  | 'llama'
  | 'mistral';

export interface GEOAnswerNodeData {
  id: string;
  type: 'GEOAnswer';
  key: string;
  displayName: string;
  /** Description */
  description?: string;
  /** Reference to GEOQuery key */
  query_key: string;
  /** The AI's answer text */
  answer_text: string;
  /** Does the answer mention our brand? */
  mentions_brand: boolean;
  /** List of competitor names mentioned in the answer */
  mentions_competitors?: string[];
  /** Position of our brand in the answer (1-based, null if absent) */
  position_in_answer?: number | null;
  /** Quality classification of the answer */
  answer_quality: AnswerQuality;
  /** When the answer was retrieved */
  retrieved_at: string;
  /** Which AI platform generated this answer */
  platform: AIPlatform;
  /** Additional context or notes */
  context?: string;
}

export interface GEOAnswerCardContentProps extends CardContext {
  data: GEOAnswerNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const contentVariants: Variants = {
  idle: { opacity: 0.85 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

const qualityBadgeVariants: Variants = {
  idle: { scale: 1 },
  hover: { scale: 1.05 },
  selected: { scale: 1.1, transition: { type: 'spring', stiffness: 300 } },
};

// =============================================================================
// Helper Components
// =============================================================================

/** Quality badge colors and styling */
const QUALITY_CONFIG: Record<AnswerQuality, { color: string; label: string; icon: string }> = {
  cited: { color: '#22c55e', label: 'CITED', icon: '◆' },
  mentioned: { color: '#f59e0b', label: 'MENTIONED', icon: '◇' },
  absent: { color: '#6b7280', label: 'ABSENT', icon: '○' },
};

/** Platform display info */
const PLATFORM_CONFIG: Record<AIPlatform, { label: string; color: string }> = {
  claude: { label: 'Claude', color: '#d4a574' },
  openai: { label: 'ChatGPT', color: '#10a37f' },
  perplexity: { label: 'Perplexity', color: '#20b8cd' },
  gemini: { label: 'Gemini', color: '#4285f4' },
  copilot: { label: 'Copilot', color: '#0078d4' },
  llama: { label: 'Llama', color: '#7c3aed' },
  mistral: { label: 'Mistral', color: '#ff7000' },
};

interface QualityBadgeProps {
  quality: AnswerQuality;
  animated: boolean;
  animationState: 'idle' | 'hover' | 'selected';
  selected: boolean;
}

const QualityBadge = memo(function QualityBadge({
  quality,
  animated,
  animationState,
  selected,
}: QualityBadgeProps) {
  const config = QUALITY_CONFIG[quality];
  const Wrapper = animated ? motion.span : 'span';

  return (
    <Wrapper
      className="px-2 py-0.5 rounded-full text-[9px] font-bold uppercase tracking-wider flex items-center gap-1"
      style={{
        backgroundColor: `${config.color}20`,
        color: config.color,
        border: `1px solid ${config.color}40`,
        boxShadow: selected ? `0 0 12px ${config.color}40` : 'none',
      }}
      {...(animated && {
        variants: qualityBadgeVariants,
        initial: 'idle',
        animate: animationState,
      })}
    >
      <span>{config.icon}</span>
      <span>{config.label}</span>
    </Wrapper>
  );
});

interface PlatformBadgeProps {
  platform: AIPlatform;
  selected: boolean;
}

const PlatformBadge = memo(function PlatformBadge({ platform, selected }: PlatformBadgeProps) {
  const config = PLATFORM_CONFIG[platform];

  return (
    <span
      className="px-1.5 py-0.5 rounded text-[8px] font-medium"
      style={{
        backgroundColor: `${config.color}20`,
        color: config.color,
        border: `1px solid ${config.color}40`,
        boxShadow: selected ? `0 0 8px ${config.color}30` : 'none',
      }}
    >
      {config.label}
    </span>
  );
});

interface BrandMentionIndicatorProps {
  mentionsBrand: boolean;
  position?: number | null;
  selected: boolean;
  primaryColor: string;
}

const BrandMentionIndicator = memo(function BrandMentionIndicator({
  mentionsBrand,
  position,
  selected,
  primaryColor,
}: BrandMentionIndicatorProps) {
  const color = mentionsBrand ? '#22c55e' : '#ef4444';
  const icon = mentionsBrand ? '✓' : '✗';
  const label = mentionsBrand ? 'Brand mentioned' : 'Brand absent';

  return (
    <div className="flex items-center justify-between">
      <div
        className="flex items-center gap-1.5 text-[10px] font-medium"
        style={{
          color,
          textShadow: selected ? `0 0 8px ${color}60` : 'none',
        }}
      >
        <span className="text-xs">{icon}</span>
        <span>{label}</span>
      </div>

      {mentionsBrand && position && (
        <span
          className="text-[9px] px-1.5 py-0.5 rounded"
          style={{
            backgroundColor: `${primaryColor}15`,
            color: `${primaryColor}cc`,
          }}
        >
          Position: {position === 1 ? '1st' : position === 2 ? '2nd' : position === 3 ? '3rd' : `${position}th`}
        </span>
      )}
    </div>
  );
});

interface CompetitorsListProps {
  competitors: string[];
  selected: boolean;
}

const CompetitorsList = memo(function CompetitorsList({
  competitors,
  selected,
}: CompetitorsListProps) {
  if (competitors.length === 0) return null;

  const amberColor = '#f59e0b';

  return (
    <div className="space-y-1">
      <span
        className="text-[8px] text-white/50 uppercase tracking-wider"
        style={{ textShadow: selected ? `0 0 4px ${amberColor}30` : 'none' }}
      >
        Competitors mentioned:
      </span>
      <div className="flex flex-wrap gap-1">
        {competitors.slice(0, 5).map((competitor, i) => (
          <span
            key={i}
            className="px-1.5 py-0.5 rounded text-[8px]"
            style={{
              backgroundColor: `${amberColor}15`,
              color: `${amberColor}cc`,
              border: `1px solid ${amberColor}30`,
            }}
          >
            {competitor}
          </span>
        ))}
        {competitors.length > 5 && (
          <span className="text-[8px] text-white/40">+{competitors.length - 5} more</span>
        )}
      </div>
    </div>
  );
});

/** Format relative time from ISO date string */
function formatRelativeTime(isoDate: string): string {
  try {
    const date = new Date(isoDate);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    return date.toLocaleDateString();
  } catch {
    return 'Unknown';
  }
}

/** Truncate answer text with ellipsis */
function truncateAnswer(text: string, maxLength: number = 100): string {
  if (text.length <= maxLength) return text;
  return text.slice(0, maxLength).trim() + '...';
}

// =============================================================================
// Component
// =============================================================================

export const GEOAnswerCardContent = memo(function GEOAnswerCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: GEOAnswerCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Performance tier checks
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showTechCorners = performanceConfig?.effects?.techCorners ?? true;

  // Determine glow color based on answer quality and brand mention
  const glowColor = useMemo(() => {
    if (data.mentions_brand) return '#22c55e'; // Green for brand mentioned
    if (data.mentions_competitors && data.mentions_competitors.length > 0) return '#f59e0b'; // Amber for competitors
    return '#6b7280'; // Gray for absent/no mentions
  }, [data.mentions_brand, data.mentions_competitors]);

  // Enhanced glow style
  const glowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 16px ${glowColor}80, 0 0 32px ${glowColor}40`
        : isHovered
          ? `0 0 12px ${glowColor}60`
          : 'none',
    }),
    [glowColor, selected, isHovered]
  );

  const ContentWrapper = animationsEnabled ? motion.div : 'div';

  // Grid pattern squares for AI response matrix effect
  const gridSquares = useMemo((): [number, number][] => {
    // Create a pattern suggesting AI/neural network
    return [
      [1, 1], [3, 1], [5, 2], [7, 1],
      [2, 3], [4, 2], [6, 3], [8, 2],
      [1, 4], [3, 5], [5, 4], [7, 5],
      [2, 6], [4, 5], [6, 6], [8, 4],
    ];
  }, []);

  return (
    <div className="relative px-4 py-4">
      {/* Layer 0: GridPattern - AI response matrix background (MEDIUM+ tier) */}
      {showPremiumEffects && (
        <GridPattern
          width={14}
          height={14}
          color={colors.primary}
          opacity={selected ? 0.25 : isHovered ? 0.18 : 0.12}
          squares={gridSquares}
          squareColor={glowColor}
          flicker={animationsEnabled}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-lg"
        />
      )}

      {/* Layer 1: BorderBeam - Animated DOTTED border (MEDIUM+ tier) */}
      {showPremiumEffects && (selected || isHovered) && (
        <BorderBeam
          color={glowColor}
          secondaryColor={colors.primary}
          borderRadius={12}
          thickness={2}
          duration={selected ? 4 : 6}
          beamLength={selected ? 0.18 : 0.12}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Layer 2: GlowEffect - Color based on brand/competitor status (MEDIUM+ tier) */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={glowColor}
          intensity={selected ? 'ultra' : isHovered ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Layer 3: LightRays - Radiating answer effect (HIGH+ tier, selected only) */}
      {showPremiumEffects && selected && (
        <LightRays
          count={5}
          color={glowColor}
          blur={25}
          opacity={0.35}
          speed={0.8}
          length={1.0}
          origin="center"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-lg"
        />
      )}

      {/* Layer 4: MotionTechCorners - AI response corners (MEDIUM+ tier) */}
      {showTechCorners && (selected || isHovered) && (
        <MotionTechCorners
          color={glowColor}
          selected={selected}
          isHovered={isHovered}
          size={12}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Content Layer */}
      <div className="relative z-10">
        {/* Taxonomy Badge: Layer (knowledge) + Realm (shared) + Class (GEOAnswer) */}
        <div className="mb-3">
          <TaxonomyBadge
            layer="knowledge"
            realm="shared"
            className="GEOAnswer"
            selected={selected}
            isHovered={isHovered}
            performanceConfig={performanceConfig}
            size="sm"
          />
        </div>

        {/* Header row: Quality badge + Platform badge */}
        <div className="flex items-center justify-between mb-2">
          <QualityBadge
            quality={data.answer_quality}
            animated={animationsEnabled}
            animationState={animationState}
            selected={selected}
          />
          <PlatformBadge platform={data.platform} selected={selected} />
        </div>

        {/* Key with subtle styling */}
        <h3 className="text-xs font-mono text-white/60 mb-2 truncate">{data.key}</h3>

        {/* Answer preview with enhanced glow */}
        <ContentWrapper
          className="space-y-3"
          {...(animationsEnabled && {
            variants: contentVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {/* Answer text preview */}
          <div
            className="p-2 rounded-md text-[11px] text-white/90 leading-relaxed"
            style={{
              backgroundColor: `${colors.primary}10`,
              border: `1px dotted ${colors.primary}30`,
              ...glowStyle,
            }}
          >
            &quot;{truncateAnswer(data.answer_text)}&quot;
          </div>

          {/* Brand mention indicator */}
          <BrandMentionIndicator
            mentionsBrand={data.mentions_brand}
            position={data.position_in_answer}
            selected={selected}
            primaryColor={colors.primary}
          />

          {/* Competitors list (if any) */}
          {data.mentions_competitors && data.mentions_competitors.length > 0 && (
            <CompetitorsList competitors={data.mentions_competitors} selected={selected} />
          )}

          {/* Query reference + Retrieved timestamp */}
          <div className="flex items-center justify-between text-[8px] pt-1 border-t border-white/10">
            <span
              className="text-white/40 truncate max-w-[60%]"
              title={data.query_key}
            >
              Query: {data.query_key}
            </span>
            <span
              className="text-white/50"
              style={{
                textShadow: selected ? `0 0 4px ${colors.primary}20` : 'none',
              }}
            >
              Retrieved {formatRelativeTime(data.retrieved_at)}
            </span>
          </div>
        </ContentWrapper>
      </div>
    </div>
  );
});
