'use client';

/**
 * GEOQueryCardContent - "AI Search Query" design for GEOQuery nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> double (imported trait)
 * - Shows query, platform, locale, category, difficulty, context
 *
 * Premium Effects (v0.13.1):
 * - GridPattern: Neural network / AI matrix background
 * - BorderBeam: Animated border respecting double style (imported trait)
 * - AuroraBackground: Subtle "AI thinking" aurora effect
 * - MotionTechCorners: AI/tech corners
 * - Enhanced GlowEffect: AI-themed intensity
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ ✦ GEO QUERY      ◇ perplexity       │  <- Sparkles icon + platform badge
 * │ ═════════════════════════════════   │  <- Double separator (imported)
 * │ geo-qr-code-generator-fr-FR          │
 * │ ┌────────────────────────────────┐   │
 * │ │ "comment créer un qr code"     │   │  <- Query text (monospace)
 * │ │ ─────────────────────────────  │   │
 * │ │ ◇ perplexity  🌍 fr-FR         │   │  <- Platform + Locale badges
 * │ │ ─────────────────────────────  │   │
 * │ │ category: how-to               │   │  <- Category badge
 * │ │ difficulty: [███░░] medium     │   │  <- Difficulty indicator
 * │ │ ─────────────────────────────  │   │
 * │ │ context: "QR code generator"   │   │  <- Context preview
 * │ └────────────────────────────────┘   │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { GlowEffect, GridPattern, BorderBeam, AuroraBackground } from '../../effects';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { MotionTechCorners } from '../../../effects';

// =============================================================================
// Types
// =============================================================================

/** AI platform types for GEO queries */
export type GEOPlatform = 'perplexity' | 'chatgpt' | 'claude' | 'gemini';

/** Query category types */
export type GEOCategory =
  | 'how-to'
  | 'comparison'
  | 'definition'
  | 'recommendation'
  | 'troubleshooting'
  | 'best-practices'
  | 'review'
  | 'general';

/** Difficulty levels */
export type GEODifficulty = 'easy' | 'medium' | 'hard' | 'expert';

export interface GEOQueryNodeData {
  id: string;
  type: 'GEOQuery';
  key: string;
  displayName: string;
  /** The AI search query string */
  query: string;
  /** Description */
  description?: string;
  /** AI platform targeted */
  platform: GEOPlatform;
  /** Target locale (BCP-47) */
  locale: string;
  /** Query category */
  category?: GEOCategory;
  /** Estimated answer complexity */
  difficulty?: GEODifficulty;
  /** Context provided to the AI */
  context?: string;
  /** Expected response type */
  response_type?: 'text' | 'list' | 'structured' | 'mixed';
  /** Is this a follow-up query */
  is_followup?: boolean;
  /** Parent query key (if follow-up) */
  parent_query_key?: string;
  /** Keywords extracted from query */
  keywords?: string[];
  /** Created timestamp */
  created_at?: string;
  /** Last tested timestamp */
  last_tested_at?: string;
}

export interface GEOQueryCardContentProps extends CardContext {
  data: GEOQueryNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Constants
// =============================================================================

/** Platform display configuration */
const PLATFORM_CONFIG: Record<GEOPlatform, { icon: string; color: string; label: string }> = {
  perplexity: { icon: '◈', color: '#22c55e', label: 'Perplexity' },
  chatgpt: { icon: '◆', color: '#10b981', label: 'ChatGPT' },
  claude: { icon: '◇', color: '#f97316', label: 'Claude' },
  gemini: { icon: '★', color: '#3b82f6', label: 'Gemini' },
};

/** Category display configuration */
const CATEGORY_CONFIG: Record<GEOCategory, { icon: string; color: string }> = {
  'how-to': { icon: '?', color: '#3b82f6' },
  comparison: { icon: '⇔', color: '#8b5cf6' },
  definition: { icon: '≡', color: '#6366f1' },
  recommendation: { icon: '★', color: '#22c55e' },
  troubleshooting: { icon: '⚠', color: '#f59e0b' },
  'best-practices': { icon: '✓', color: '#10b981' },
  review: { icon: '◉', color: '#ec4899' },
  general: { icon: '●', color: '#94a3b8' },
};

/** Difficulty level configuration */
const DIFFICULTY_CONFIG: Record<GEODifficulty, { level: number; color: string }> = {
  easy: { level: 1, color: '#22c55e' },
  medium: { level: 2, color: '#f59e0b' },
  hard: { level: 3, color: '#f97316' },
  expert: { level: 4, color: '#ef4444' },
};

// =============================================================================
// Animation Variants
// =============================================================================

const queryVariants: Variants = {
  idle: { opacity: 0.9 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Helper Components
// =============================================================================

/** Platform badge with icon and color */
const PlatformBadge = memo(function PlatformBadge({
  platform,
  selected = false,
}: {
  platform: GEOPlatform;
  selected?: boolean;
}) {
  const config = PLATFORM_CONFIG[platform];

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-bold uppercase tracking-wider"
      style={{
        backgroundColor: `${config.color}20`,
        color: config.color,
        border: `1px solid ${config.color}40`,
        boxShadow: selected ? `0 0 8px ${config.color}30` : 'none',
      }}
    >
      {config.icon} {config.label}
    </span>
  );
});

/** Locale badge */
const LocaleBadge = memo(function LocaleBadge({
  locale,
  color,
}: {
  locale: string;
  color: string;
}) {
  // Extract language and region
  const [lang, region] = locale.split('-');
  const displayLocale = region ? `${lang.toUpperCase()}-${region}` : lang.toUpperCase();

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        backgroundColor: `${color}15`,
        color: `${color}cc`,
        border: `1px solid ${color}30`,
      }}
    >
      ⊕ {displayLocale}
    </span>
  );
});

/** Category badge with icon */
const CategoryBadge = memo(function CategoryBadge({
  category,
}: {
  category: GEOCategory;
}) {
  const config = CATEGORY_CONFIG[category];

  return (
    <div className="flex items-center gap-1.5">
      <span className="text-[8px] text-white/50">category:</span>
      <span
        className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-medium"
        style={{
          backgroundColor: `${config.color}20`,
          color: config.color,
        }}
      >
        {config.icon} {category.replace('-', ' ')}
      </span>
    </div>
  );
});

/** Difficulty indicator with visual scale */
const DifficultyIndicator = memo(function DifficultyIndicator({
  difficulty,
}: {
  difficulty: GEODifficulty;
}) {
  const config = DIFFICULTY_CONFIG[difficulty];

  return (
    <div className="flex items-center gap-1.5">
      <span className="text-[8px] text-white/50">difficulty:</span>
      <div className="flex gap-0.5">
        {[1, 2, 3, 4].map((i) => (
          <div
            key={i}
            className="w-2 h-2 rounded-sm"
            style={{
              backgroundColor: i <= config.level ? config.color : `${config.color}30`,
            }}
          />
        ))}
      </div>
      <span
        className="text-[8px] font-bold"
        style={{ color: config.color }}
      >
        {difficulty}
      </span>
    </div>
  );
});

/** Context preview (truncated) */
const ContextPreview = memo(function ContextPreview({
  context,
  color,
}: {
  context: string;
  color: string;
}) {
  const truncated = context.length > 50 ? `${context.slice(0, 47)}...` : context;

  return (
    <div className="flex flex-col gap-0.5">
      <span className="text-[8px] text-white/50">context:</span>
      <span
        className="text-[9px] font-mono px-1.5 py-0.5 rounded"
        style={{
          backgroundColor: `${color}10`,
          color: `${color}99`,
        }}
      >
        &quot;{truncated}&quot;
      </span>
    </div>
  );
});

/** Keywords list */
const KeywordsList = memo(function KeywordsList({
  keywords,
  color,
  maxShow = 4,
}: {
  keywords: string[];
  color: string;
  maxShow?: number;
}) {
  if (!keywords || keywords.length === 0) return null;

  const displayKeywords = keywords.slice(0, maxShow);
  const remaining = keywords.length - maxShow;

  return (
    <div className="flex flex-wrap gap-1 items-center">
      <span className="text-[8px] text-white/50">keywords:</span>
      {displayKeywords.map((keyword, i) => (
        <span
          key={i}
          className="px-1 py-0.5 rounded text-[7px] font-mono"
          style={{
            backgroundColor: `${color}15`,
            color: `${color}aa`,
          }}
        >
          {keyword}
        </span>
      ))}
      {remaining > 0 && (
        <span className="text-[7px] text-white/40">+{remaining}</span>
      )}
    </div>
  );
});

// =============================================================================
// Component
// =============================================================================

export const GEOQueryCardContent = memo(function GEOQueryCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: GEOQueryCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Performance tier checks
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showTechCorners = performanceConfig?.effects?.techCorners ?? true;

  // Enhanced glow style for AI/neural feel
  const glowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 16px ${colors.primary}80, 0 0 32px ${colors.primary}40`
        : isHovered
          ? `0 0 12px ${colors.primary}60`
          : 'none',
    }),
    [colors.primary, selected, isHovered]
  );

  const QueryWrapper = animationsEnabled ? motion.div : 'div';

  // Grid pattern squares for neural network / AI matrix effect
  const gridSquares = useMemo((): [number, number][] => {
    // Create a pattern that suggests AI/neural connections
    return [
      [1, 1], [3, 1], [5, 2], [7, 1],
      [2, 3], [4, 2], [6, 4], [8, 2],
      [1, 5], [3, 4], [5, 5], [7, 3],
      [2, 6], [4, 6], [6, 6], [8, 5],
    ];
  }, []);

  return (
    <div className="relative px-4 py-4">
      {/* Layer 0: GridPattern - Neural network / AI matrix background (MEDIUM+ tier) */}
      {showPremiumEffects && (
        <GridPattern
          width={14}
          height={14}
          color={colors.primary}
          opacity={selected ? 0.22 : isHovered ? 0.15 : 0.10}
          squares={gridSquares}
          squareColor={colors.primary}
          flicker={animationsEnabled}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-lg"
        />
      )}

      {/* Layer 1: AuroraBackground - Subtle "AI thinking" aurora (HIGH+ tier) */}
      {showPremiumEffects && (selected || isHovered) && (
        <AuroraBackground
          primaryColor={colors.primary}
          secondaryColor={colors.secondary ?? colors.primary}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          intensity="subtle"
          borderRadius={12}
        />
      )}

      {/* Layer 2: BorderBeam - Animated border respecting double style (MEDIUM+ tier) */}
      {showPremiumEffects && (selected || isHovered) && (
        <BorderBeam
          color={colors.primary}
          secondaryColor={colors.secondary ?? colors.primary}
          borderRadius={12}
          thickness={3}
          duration={selected ? 4 : 6}
          beamLength={selected ? 0.2 : 0.15}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Layer 3: Enhanced GlowEffect (MEDIUM+ tier) */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={colors.primary}
          intensity={selected ? 'ultra' : isHovered ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Layer 4: MotionTechCorners - AI/tech corners (MEDIUM+ tier) */}
      {showTechCorners && (selected || isHovered) && (
        <MotionTechCorners
          color={colors.primary}
          selected={selected}
          isHovered={isHovered}
          size={14}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Content Layer */}
      <div className="relative z-10">
        {/* Taxonomy Badge: Layer (knowledge) + Realm (shared) + Trait (imported) + Class (GEOQuery) */}
        <div className="mb-3">
          <TaxonomyBadge
            layer="knowledge"
            realm="shared"
            trait="imported"
            className="GEOQuery"
            selected={selected}
            isHovered={isHovered}
            performanceConfig={performanceConfig}
            size="sm"
          />
        </div>

        {/* Platform badge */}
        <div className="flex justify-end mb-2">
          <PlatformBadge platform={data.platform} selected={selected} />
        </div>

        {/* Query key with subtle AI styling */}
        <h3 className="text-xs font-mono text-white/60 mb-1 truncate">{data.key}</h3>

        {/* Query text with enhanced glow - monospace for "code" feel */}
        <QueryWrapper
          className="mb-3"
          {...(animationsEnabled && {
            variants: queryVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          <div
            className="text-sm font-mono font-bold text-white px-2 py-1.5 rounded"
            style={{
              ...glowStyle,
              backgroundColor: `${colors.primary}10`,
              border: `1px double ${colors.primary}40`,
            }}
          >
            &quot;{data.query}&quot;
          </div>
        </QueryWrapper>

        {/* Info section */}
        <div className="space-y-2.5">
          {/* Platform + Locale row */}
          <div className="flex items-center gap-2">
            <LocaleBadge locale={data.locale} color={colors.primary} />
            {data.is_followup && (
              <span
                className="px-1 py-0.5 rounded text-[7px] font-mono"
                style={{
                  backgroundColor: `${colors.primary}15`,
                  color: `${colors.primary}99`,
                }}
              >
                ↳ follow-up
              </span>
            )}
          </div>

          {/* Category badge */}
          {data.category && <CategoryBadge category={data.category} />}

          {/* Difficulty indicator */}
          {data.difficulty && <DifficultyIndicator difficulty={data.difficulty} />}

          {/* Context preview */}
          {data.context && (
            <ContextPreview context={data.context} color={colors.primary} />
          )}

          {/* Keywords */}
          {data.keywords && data.keywords.length > 0 && (
            <KeywordsList keywords={data.keywords} color={colors.primary} />
          )}

          {/* Response type badge */}
          {data.response_type && (
            <div className="flex items-center gap-1.5 text-[8px]">
              <span className="text-white/50">response:</span>
              <span
                className="px-1.5 py-0.5 rounded font-medium"
                style={{
                  backgroundColor: `${colors.primary}15`,
                  color: `${colors.primary}cc`,
                }}
              >
                {data.response_type}
              </span>
            </div>
          )}

          {/* Last tested timestamp */}
          {data.last_tested_at && (
            <div className="text-[7px] text-white/40">
              last tested: {new Date(data.last_tested_at).toLocaleDateString()}
            </div>
          )}
        </div>
      </div>
    </div>
  );
});
