'use client';

/**
 * ProjectCardContent - "Mission Control" Premium Design
 *
 * Project is the ROOT of everything in the ORG realm. It deserves
 * the most impressive card design with all premium effects.
 *
 * Design Concept: A command center dashboard showing project health,
 * metrics, and status with animated effects and rich visual feedback.
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = foundation) → violet #8b5cf6
 * - Border style → solid (defined trait)
 * - Full premium effects: Aurora, GridPattern, BorderBeam, Spotlight
 *
 * Layout:
 * ┌──────────────────────────────────────────────────────────────────────────────┐
 * │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
 * │  ░░  ┌─────────────────────────────────────────────────────────────────┐  ░░  │
 * │  ░░  │  ◆ FOUNDATION    ● org                         [Project Logo]  │  ░░  │
 * │  ░░  └─────────────────────────────────────────────────────────────────┘  ░░  │
 * │  ░░                                                                       ░░  │
 * │  ░░                        ┌──────────────────────┐                       ░░  │
 * │  ░░                        │      🎯              │                       ░░  │
 * │  ░░                        │    QR Code AI        │  ← Hero Name          ░░  │
 * │  ░░                        │    qrcode-ai         │  ← Key                ░░  │
 * │  ░░                        └──────────────────────┘                       ░░  │
 * │  ░░                                                                       ░░  │
 * │  ░░  ┌─────────────────────────────────────────────────────────────────┐  ░░  │
 * │  ░░  │                    MISSION CONTROL                              │  ░░  │
 * │  ░░  ├─────────────────────────────────────────────────────────────────┤  ░░  │
 * │  ░░  │                                                                 │  ░░  │
 * │  ░░  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐    │  ░░  │
 * │  ░░  │  │   📄      │  │   🌍      │  │   🎯      │  │   ⚙️      │    │  ░░  │
 * │  ░░  │  │   24      │  │   12      │  │   156     │  │   4       │    │  ░░  │
 * │  ░░  │  │  pages    │  │  locales  │  │ entities  │  │  blocks   │    │  ░░  │
 * │  ░░  │  └───────────┘  └───────────┘  └───────────┘  └───────────┘    │  ░░  │
 * │  ░░  │                                                                 │  ░░  │
 * │  ░░  │  Progress ███████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░  40%      │  ░░  │
 * │  ░░  │                                                                 │  ░░  │
 * │  ░░  └─────────────────────────────────────────────────────────────────┘  ░░  │
 * │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
 * └──────────────────────────────────────────────────────────────────────────────┘
 */

import { memo, useState, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import Image from 'next/image';
import {
  Briefcase,
  FileText,
  Globe2,
  Target,
  Blocks,
  Sparkles,
  Zap,
  TrendingUp,
} from 'lucide-react';
// gapTokens reserved for future use
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { SPRING_CONFIGS } from '../animationPresets';
import type { NodeLayer, NodeRealm, NodeTrait } from '../taxonomyColors';
import {
  LAYER_COLORS,
  REALM_COLORS,
} from '@/design/colors/generated';
import {
  GridPattern,
  AuroraBackground,
  BorderBeam,
  HolographicOverlay,
  GlowEffect,
  MouseSpotlight,
  // ULTRA premium effects (v0.13.1)
  MatrixRain,
  Meteors,
  LightRays,
  ScanLines,
} from '../effects';
import { MotionTechCorners } from '../../effects';

// NovaNet logo URL
const NOVANET_LOGO_URL = 'https://pbs.twimg.com/profile_images/1788187862883598336/q8u1VSz3_400x400.jpg';

// =============================================================================
// Types
// =============================================================================

export interface ProjectNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  logoUrl?: string;
  description?: string;
  /** KPI metrics */
  metrics?: {
    pages?: { current: number; max: number };
    locales?: { current: number; max: number };
    entities?: { current: number; max: number };
    blocks?: { current: number; max: number };
  };
  /** Project status */
  status?: 'active' | 'draft' | 'archived';
}

/** Optional taxonomy props for full visual encoding (ADR-005) */
export interface ProjectTaxonomyProps {
  layer: NodeLayer;
  realm: NodeRealm;
  trait: NodeTrait;
}

export interface ProjectCardContentProps extends CardContext {
  data: ProjectNodeData;
  performanceConfig?: PerformanceConfig;
  /** Optional taxonomy info for TaxonomyBadge (ADR-005) */
  taxonomy?: ProjectTaxonomyProps;
  /** Show TaxonomyBadge in header instead of simple label (default: false) */
  showTaxonomyBadge?: boolean;
}

// =============================================================================
// Animation Variants
// =============================================================================

const cardVariants: Variants = {
  idle: {
    scale: 1,
    rotateX: 0,
    rotateY: 0,
  },
  hover: {
    scale: 1.01,
    transition: SPRING_CONFIGS.gentle,
  },
  selected: {
    scale: 1.02,
    transition: SPRING_CONFIGS.smooth,
  },
};

const logoVariants: Variants = {
  idle: { scale: 1, rotate: 0 },
  hover: {
    scale: 1.1,
    rotate: 5,
    transition: SPRING_CONFIGS.bouncy,
  },
  selected: {
    scale: 1.15,
    rotate: -5,
    transition: {
      scale: SPRING_CONFIGS.smooth,
      rotate: {
        duration: 2,
        repeat: Infinity,
        repeatType: 'reverse',
        ease: 'easeInOut',
      },
    },
  },
};

const statVariants: Variants = {
  idle: { scale: 1, y: 0 },
  hover: {
    scale: 1.05,
    y: -2,
    transition: SPRING_CONFIGS.gentle,
  },
  selected: {
    scale: 1.1,
    y: -4,
    transition: SPRING_CONFIGS.smooth,
  },
};

const pulseVariants: Variants = {
  animate: {
    scale: [1, 1.2, 1],
    opacity: [0.8, 1, 0.8],
    transition: {
      duration: 2,
      repeat: Infinity,
      ease: 'easeInOut',
    },
  },
};

// =============================================================================
// Stat Card Component
// =============================================================================

interface StatCardProps {
  icon: React.ReactNode;
  value: number;
  label: string;
  color: string;
  animationsEnabled: boolean;
  animationState: string;
  delay?: number;
}

const StatCard = memo(function StatCard({
  icon,
  value,
  label,
  color,
  animationsEnabled,
  animationState,
  delay = 0,
}: StatCardProps) {
  const Wrapper = animationsEnabled ? motion.div : 'div';

  return (
    <Wrapper
      className="flex flex-col items-center p-3 rounded-xl"
      style={{
        background: `linear-gradient(135deg, ${color}15 0%, ${color}08 100%)`,
        border: `1px solid ${color}25`,
        boxShadow: animationState === 'selected' ? `0 0 20px ${color}30` : undefined,
      }}
      {...(animationsEnabled && {
        variants: statVariants,
        initial: 'idle',
        animate: animationState,
        transition: { delay: delay * 0.1 },
      })}
    >
      <div
        className="mb-1"
        style={{
          color,
          filter: `drop-shadow(0 0 8px ${color})`,
        }}
      >
        {icon}
      </div>
      <span
        className="text-2xl font-bold"
        style={{
          color,
          textShadow: `0 0 20px ${color}60`,
        }}
      >
        {value}
      </span>
      <span className="text-[10px] uppercase tracking-wider text-white/50 font-medium">
        {label}
      </span>
    </Wrapper>
  );
});

// =============================================================================
// Progress Bar Component
// =============================================================================

interface ProgressBarProps {
  current: number;
  max: number;
  color: string;
  label: string;
  animationsEnabled: boolean;
}

const ProgressBar = memo(function ProgressBar({
  current,
  max,
  color,
  label,
  animationsEnabled,
}: ProgressBarProps) {
  const percentage = Math.round((current / max) * 100);
  const BarFill = animationsEnabled ? motion.div : 'div';

  return (
    <div className="w-full">
      <div className="flex justify-between items-center mb-1.5">
        <span className="text-xs font-medium text-white/60">{label}</span>
        <span
          className="text-xs font-bold"
          style={{ color }}
        >
          {current}/{max} ({percentage}%)
        </span>
      </div>
      <div
        className="h-2 rounded-full overflow-hidden"
        style={{ background: `${color}15` }}
      >
        <BarFill
          className="h-full rounded-full"
          style={{
            background: `linear-gradient(90deg, ${color}80 0%, ${color} 100%)`,
            boxShadow: `0 0 10px ${color}60`,
          }}
          {...(animationsEnabled && {
            initial: { width: 0 },
            animate: { width: `${percentage}%` },
            transition: { duration: 1, ease: 'easeOut', delay: 0.3 },
          })}
          {...(!animationsEnabled && {
            style: {
              width: `${percentage}%`,
              background: `linear-gradient(90deg, ${color}80 0%, ${color} 100%)`,
              boxShadow: `0 0 10px ${color}60`,
            },
          })}
        />
      </div>
    </div>
  );
});

// =============================================================================
// Component
// =============================================================================

export const ProjectCardContent = memo(function ProjectCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
  taxonomy: _taxonomy,
}: ProjectCardContentProps) {
  const [imageError, setImageError] = useState(false);
  const logoUrl = data.logoUrl || NOVANET_LOGO_URL;

  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;
  const showTechCorners = performanceConfig?.effects?.techCorners ?? true;

  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Colors
  const primaryColor = colors.primary;
  const layerColor = LAYER_COLORS.foundation?.color ?? '#8b5cf6';
  const realmColor = REALM_COLORS.org?.color ?? '#0ea5e9';

  // Background style
  const backgroundStyle = useMemo(() => ({
    background: `linear-gradient(145deg,
      rgba(15, 15, 20, 0.98) 0%,
      rgba(20, 15, 30, 0.95) 50%,
      rgba(15, 20, 25, 0.98) 100%)`,
  }), []);

  // Shadow style
  const shadowStyle = useMemo(() => ({
    boxShadow: selected
      ? `0 0 40px ${primaryColor}50, 0 0 80px ${primaryColor}25, inset 0 1px 0 ${primaryColor}20`
      : isHovered
        ? `0 0 30px ${primaryColor}30, inset 0 1px 0 ${primaryColor}10`
        : `0 8px 32px rgba(0,0,0,0.4), inset 0 1px 0 rgba(255,255,255,0.05)`,
  }), [primaryColor, selected, isHovered]);

  // Logo container style
  const logoContainerStyle = useMemo(() => ({
    background: `linear-gradient(135deg, ${primaryColor}20 0%, ${primaryColor}10 100%)`,
    border: `2px solid ${primaryColor}40`,
    boxShadow: selected
      ? `0 0 30px ${primaryColor}60, 0 0 60px ${primaryColor}30`
      : isHovered
        ? `0 0 20px ${primaryColor}40`
        : `0 0 15px ${primaryColor}20`,
  }), [primaryColor, selected, isHovered]);

  // Grid squares for pattern
  const gridSquares = useMemo((): [number, number][] => {
    return [
      [2, 1], [5, 2], [8, 1], [11, 2],
      [1, 4], [4, 5], [7, 4], [10, 5], [13, 4],
      [3, 7], [6, 8], [9, 7], [12, 8],
    ];
  }, []);

  // Calculate total progress
  const totalProgress = useMemo(() => {
    if (!data.metrics) return null;
    let total = 0;
    let count = 0;
    if (data.metrics.pages) {
      total += data.metrics.pages.current / data.metrics.pages.max;
      count++;
    }
    if (data.metrics.locales) {
      total += data.metrics.locales.current / data.metrics.locales.max;
      count++;
    }
    if (data.metrics.entities) {
      total += data.metrics.entities.current / data.metrics.entities.max;
      count++;
    }
    return count > 0 ? Math.round((total / count) * 100) : null;
  }, [data.metrics]);

  const CardWrapper = animationsEnabled ? motion.div : 'div';
  const LogoWrapper = animationsEnabled ? motion.div : 'div';
  const PulseIndicator = animationsEnabled ? motion.div : 'div';

  return (
    <CardWrapper
      className="relative p-6 min-h-[340px] w-full rounded-2xl overflow-hidden flex flex-col"
      style={{
        ...backgroundStyle,
        ...shadowStyle,
      }}
      {...(animationsEnabled && {
        variants: cardVariants,
        initial: 'idle',
        animate: animationState,
      })}
    >
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* PREMIUM EFFECTS LAYER                                                   */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}

      {/* Grid pattern background */}
      <GridPattern
        width={32}
        height={32}
        color={primaryColor}
        opacity={selected ? 0.2 : isHovered ? 0.15 : 0.1}
        squares={gridSquares}
        squareColor={primaryColor}
        flicker={animationsEnabled}
        selected={selected}
        isHovered={isHovered}
        performanceConfig={performanceConfig}
        className="rounded-2xl"
      />

      {/* Aurora background */}
      {showPremiumEffects && animationsEnabled && (
        <AuroraBackground
          primaryColor={primaryColor}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          intensity={selected ? 'intense' : isHovered ? 'medium' : 'subtle'}
          borderRadius={16}
        />
      )}

      {/* Mouse spotlight effect */}
      {showPremiumEffects && animationsEnabled && (
        <MouseSpotlight
          color={primaryColor}
          size={300}
          intensity={selected ? 'intense' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Border beam */}
      {showPremiumEffects && animationsEnabled && (selected || isHovered) && (
        <BorderBeam
          color={primaryColor}
          secondaryColor={realmColor}
          borderRadius={16}
          thickness={selected ? 3 : 2}
          duration={selected ? 4 : 6}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          beamLength={0.2}
        />
      )}

      {/* Holographic overlay */}
      {showPremiumEffects && animationsEnabled && selected && (
        <HolographicOverlay
          baseColor={primaryColor}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          intensity="medium"
          borderRadius={16}
        />
      )}

      {/* Outer glow */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={primaryColor}
          intensity={selected ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Tech corners */}
      {showTechCorners && (
        <MotionTechCorners
          color={primaryColor}
          selected={selected}
          isHovered={isHovered}
          size={16}
          performanceConfig={performanceConfig}
        />
      )}

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* ULTRA PREMIUM EFFECTS (v0.13.1 - Maximum WOW)                            */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}

      {/* Matrix rain - digital code rain effect */}
      {showPremiumEffects && animationsEnabled && selected && (
        <MatrixRain
          color="#22c55e"
          columns={10}
          opacity={0.4}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-2xl"
        />
      )}

      {/* Meteors - shooting stars */}
      {showPremiumEffects && animationsEnabled && (selected || isHovered) && (
        <Meteors
          count={selected ? 10 : 6}
          color={primaryColor}
          angle={215}
          minDuration={1.5}
          maxDuration={3}
          meteorWidth={80}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-2xl"
        />
      )}

      {/* Light rays - volumetric beams */}
      {showPremiumEffects && animationsEnabled && selected && (
        <LightRays
          count={5}
          color={primaryColor}
          blur={25}
          opacity={0.25}
          origin="top"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-2xl"
        />
      )}

      {/* Scan lines - CRT hologram effect */}
      {showPremiumEffects && (
        <ScanLines
          spacing={3}
          thickness={1}
          color="rgba(255, 255, 255, 0.02)"
          opacity={selected ? 1 : 0.6}
          showScanBeam={animationsEnabled && selected}
          scanBeamColor="rgba(139, 92, 246, 0.15)"
          flicker={animationsEnabled}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-2xl"
        />
      )}

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* HEADER: Layer + Realm badges + Logo                                     */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div className="relative z-10 flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          {/* Layer badge */}
          <div
            className={cn(
              'inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg',
              'text-[11px] font-bold uppercase tracking-wider'
            )}
            style={{
              background: `linear-gradient(135deg, ${layerColor}25 0%, ${layerColor}15 100%)`,
              color: layerColor,
              border: `1px solid ${layerColor}40`,
              boxShadow: `0 0 15px ${layerColor}20`,
            }}
          >
            <Sparkles size={12} />
            foundation
          </div>

          {/* Realm badge */}
          <div
            className={cn(
              'inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full',
              'text-[11px] font-bold uppercase tracking-wider'
            )}
            style={{
              background: `${realmColor}15`,
              color: realmColor,
              border: `1px solid ${realmColor}35`,
            }}
          >
            <PulseIndicator
              className="w-2 h-2 rounded-full"
              style={{ background: realmColor }}
              {...(animationsEnabled && {
                variants: pulseVariants,
                animate: 'animate',
              })}
            />
            org
          </div>
        </div>

        {/* Project Logo */}
        <LogoWrapper
          className="w-20 h-20 rounded-2xl overflow-hidden"
          style={logoContainerStyle}
          {...(animationsEnabled && {
            variants: logoVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {!imageError ? (
            <Image
              src={logoUrl}
              alt={data.displayName}
              width={80}
              height={80}
              className="object-cover w-full h-full"
              unoptimized
              onError={() => setImageError(true)}
            />
          ) : (
            <div
              className="w-full h-full flex items-center justify-center"
              style={{ background: `${primaryColor}20` }}
            >
              <Briefcase size={36} style={{ color: primaryColor }} />
            </div>
          )}
        </LogoWrapper>
      </div>

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* HERO: Project Name + Key                                                */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div className="relative z-10 text-center mb-5">
        <h2
          className="text-3xl font-bold text-white mb-2"
          style={{
            textShadow: selected
              ? `0 0 40px ${primaryColor}80, 0 0 80px ${primaryColor}40`
              : isHovered
                ? `0 0 30px ${primaryColor}50`
                : `0 0 15px ${primaryColor}30`,
          }}
        >
          {data.displayName}
        </h2>
        <p
          className="font-mono text-sm"
          style={{
            color: `${primaryColor}80`,
            textShadow: `0 0 10px ${primaryColor}30`,
          }}
        >
          {data.key}
        </p>
      </div>

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* MISSION CONTROL: Stats Dashboard                                        */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div
        className="relative z-10 p-4 rounded-xl flex-1"
        style={{
          background: `linear-gradient(180deg, ${primaryColor}08 0%, ${primaryColor}03 100%)`,
          border: `1px solid ${primaryColor}20`,
        }}
      >
        {/* Section Header */}
        <div className="flex items-center justify-center gap-2 mb-4">
          <Zap size={14} style={{ color: primaryColor }} />
          <span
            className="text-[10px] font-bold uppercase tracking-[0.2em]"
            style={{ color: primaryColor }}
          >
            Mission Control
          </span>
          <Zap size={14} style={{ color: primaryColor }} />
        </div>

        {/* Stats Grid */}
        <div className="grid grid-cols-4 gap-3 mb-4">
          <StatCard
            icon={<FileText size={18} />}
            value={data.metrics?.pages?.current ?? 0}
            label="pages"
            color="#8b5cf6"
            animationsEnabled={animationsEnabled}
            animationState={animationState}
            delay={0}
          />
          <StatCard
            icon={<Globe2 size={18} />}
            value={data.metrics?.locales?.current ?? 0}
            label="locales"
            color="#06b6d4"
            animationsEnabled={animationsEnabled}
            animationState={animationState}
            delay={1}
          />
          <StatCard
            icon={<Target size={18} />}
            value={data.metrics?.entities?.current ?? 0}
            label="entities"
            color="#22c55e"
            animationsEnabled={animationsEnabled}
            animationState={animationState}
            delay={2}
          />
          <StatCard
            icon={<Blocks size={18} />}
            value={data.metrics?.blocks?.current ?? 0}
            label="blocks"
            color="#f59e0b"
            animationsEnabled={animationsEnabled}
            animationState={animationState}
            delay={3}
          />
        </div>

        {/* Progress Bar */}
        {totalProgress !== null && (
          <ProgressBar
            current={totalProgress}
            max={100}
            color={primaryColor}
            label="Overall Progress"
            animationsEnabled={animationsEnabled}
          />
        )}

        {/* Status indicator when no metrics */}
        {!data.metrics && (
          <div className="flex items-center justify-center gap-2 py-4">
            <TrendingUp size={16} style={{ color: primaryColor }} />
            <span className="text-sm text-white/50">
              Ready for generation
            </span>
          </div>
        )}
      </div>

      {/* ═══════════════════════════════════════════════════════════════════════ */}
      {/* FOOTER: Status Badge                                                    */}
      {/* ═══════════════════════════════════════════════════════════════════════ */}
      <div className="relative z-10 flex justify-center mt-4">
        <div
          className="inline-flex items-center gap-2 px-4 py-2 rounded-full"
          style={{
            background: data.status === 'active'
              ? 'rgba(34, 197, 94, 0.15)'
              : data.status === 'draft'
                ? 'rgba(251, 191, 36, 0.15)'
                : 'rgba(107, 114, 128, 0.15)',
            border: `1px solid ${
              data.status === 'active'
                ? 'rgba(34, 197, 94, 0.4)'
                : data.status === 'draft'
                  ? 'rgba(251, 191, 36, 0.4)'
                  : 'rgba(107, 114, 128, 0.4)'
            }`,
          }}
        >
          <span
            className="w-2 h-2 rounded-full animate-pulse"
            style={{
              background: data.status === 'active'
                ? '#22c55e'
                : data.status === 'draft'
                  ? '#fbbf24'
                  : '#6b7280',
              boxShadow: `0 0 8px ${
                data.status === 'active'
                  ? '#22c55e'
                  : data.status === 'draft'
                    ? '#fbbf24'
                    : '#6b7280'
              }`,
            }}
          />
          <span
            className="text-xs font-bold uppercase tracking-wider"
            style={{
              color: data.status === 'active'
                ? '#22c55e'
                : data.status === 'draft'
                  ? '#fbbf24'
                  : '#6b7280',
            }}
          >
            {data.status || 'active'}
          </span>
        </div>
      </div>
    </CardWrapper>
  );
});
