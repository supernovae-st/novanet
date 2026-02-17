'use client';

/**
 * ProjectCardContent - "Project HQ" premium design for Project nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = foundation) → violet #8b5cf6
 * - Border style → solid (defined trait)
 * - Shows KPI gauges for pages/locales completion
 *
 * Layout:
 * ┌────────────────────────────┐
 * │ 📁 PROJECT                 │
 * │ ═══════════════════════    │
 * │ QRCode AI                  │
 * │ ┌────────────────────────┐ │
 * │ │ ┌──┬──┬──┬──┬──┐       │ │
 * │ │ │██│██│░░│░░│░░│ 40%   │ │ ← KPI gauge
 * │ │ └──┴──┴──┴──┴──┘       │ │
 * │ │ pages 24/60 │ loc 5/200│ │
 * │ └────────────────────────┘ │
 * └────────────────────────────┘
 */

import { memo, useState, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import Image from 'next/image';
import { Briefcase } from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { SPRING_CONFIGS, DURATIONS } from '../animationPresets';
import { KPIGauge } from './foundation/FoundationHelpers';
import type { NodeLayer, NodeRealm, NodeTrait } from '../taxonomyColors';
import { TaxonomyBadge } from '../TaxonomyBadge';

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
  /** KPI metrics */
  metrics?: {
    pages?: { current: number; max: number };
    locales?: { current: number; max: number };
    entities?: { current: number; max: number };
  };
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

const logoVariants: Variants = {
  idle: { scale: 1 },
  hover: {
    scale: 1.05,
    transition: SPRING_CONFIGS.gentle,
  },
  selected: {
    scale: 1.08,
    transition: SPRING_CONFIGS.smooth,
  },
};

const badgeVariants: Variants = {
  idle: { opacity: 0.8 },
  hover: { opacity: 1 },
  selected: {
    opacity: 1,
    scale: [1, 1.05, 1],
    transition: { duration: DURATIONS.normal },
  },
};

// =============================================================================
// Component
// =============================================================================

export const ProjectCardContent = memo(function ProjectCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
  taxonomy,
  showTaxonomyBadge = false,
}: ProjectCardContentProps) {
  // Use TaxonomyBadge if taxonomy props provided and showTaxonomyBadge is true
  const useTaxonomyBadge = showTaxonomyBadge && taxonomy;

  const [imageError, setImageError] = useState(false);
  const logoUrl = data.logoUrl || NOVANET_LOGO_URL;

  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Memoize ring style
  const ringStyle = useMemo(() => ({
    boxShadow: selected ? `0 0 15px ${colors.primary}40` : undefined,
  }), [colors.primary, selected]);

  // Glow style
  const glowStyle = useMemo(() => ({
    textShadow: selected
      ? `0 0 12px ${colors.primary}60`
      : isHovered
        ? `0 0 8px ${colors.primary}40`
        : 'none',
  }), [colors.primary, selected, isHovered]);

  // Has any metrics
  const hasMetrics = data.metrics && (
    data.metrics.pages || data.metrics.locales || data.metrics.entities
  );

  const LogoWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Grid background pattern (Project-specific decoration) */}
      <div
        className={cn(
          'absolute inset-0 pointer-events-none',
          selected ? 'opacity-[0.05]' : 'opacity-[0.03]'
        )}
        style={{
          backgroundImage: `
            linear-gradient(rgba(139, 92, 246, 0.5) 1px, transparent 1px),
            linear-gradient(90deg, rgba(139, 92, 246, 0.5) 1px, transparent 1px)
          `,
          backgroundSize: '20px 20px',
        }}
      />

      {/* Header: TaxonomyBadge or Icon + PROJECT label */}
      {useTaxonomyBadge ? (
        <div className={cn('relative flex items-center justify-between mb-2', gapTokens.default)}>
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

          {/* Logo thumbnail */}
          <LogoWrapper
            className={cn(
              'w-8 h-8 rounded-lg overflow-hidden ring-1',
              selected ? 'ring-white/40' : 'ring-white/20'
            )}
            style={ringStyle}
            {...(animationsEnabled && {
              variants: logoVariants,
              initial: 'idle',
              animate: animationState,
            })}
          >
            {!imageError ? (
              <Image
                src={logoUrl}
                alt="Project"
                width={32}
                height={32}
                className="object-cover w-full h-full"
                unoptimized
                onError={() => setImageError(true)}
              />
            ) : (
              <div
                className="w-full h-full flex items-center justify-center"
                style={{ background: `${colors.primary}20` }}
              >
                <Briefcase size={16} style={{ color: colors.primary }} />
              </div>
            )}
          </LogoWrapper>
        </div>
      ) : (
        <div className={cn('relative flex items-center justify-between mb-2', gapTokens.default)}>
          <div className={cn('flex items-center', gapTokens.default)}>
            <span
              className="text-base"
              style={{
                color: colors.primary,
                filter: `drop-shadow(0 0 4px ${colors.primary})`,
              }}
            >
              📁
            </span>
            <span
              className="text-[9px] font-bold uppercase tracking-widest font-mono"
              style={{ color: colors.primary }}
            >
              PROJECT
            </span>
          </div>

          {/* Logo thumbnail */}
          <LogoWrapper
            className={cn(
              'w-8 h-8 rounded-lg overflow-hidden ring-1',
              selected ? 'ring-white/40' : 'ring-white/20'
            )}
            style={ringStyle}
            {...(animationsEnabled && {
              variants: logoVariants,
              initial: 'idle',
              animate: animationState,
            })}
          >
            {!imageError ? (
              <Image
                src={logoUrl}
                alt="Project"
                width={32}
                height={32}
                className="object-cover w-full h-full"
                unoptimized
                onError={() => setImageError(true)}
              />
            ) : (
              <div
                className="w-full h-full flex items-center justify-center"
                style={{ background: `${colors.primary}20` }}
              >
                <Briefcase size={16} style={{ color: colors.primary }} />
              </div>
            )}
          </LogoWrapper>
        </div>
      )}

      {/* Double line separator */}
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

      {/* Project name */}
      <h3
        className="relative text-lg font-bold text-white mb-3"
        style={glowStyle}
      >
        {data.displayName}
      </h3>

      {/* KPI Metrics section */}
      {hasMetrics && (
        <div
          className="p-2 rounded-lg"
          style={{
            background: `${colors.primary}08`,
            border: `1px solid ${colors.primary}20`,
          }}
        >
          <div className="space-y-2">
            {data.metrics?.pages && (
              <KPIGauge
                current={data.metrics.pages.current}
                max={data.metrics.pages.max}
                label="pages"
                color={colors.primary}
                animate={animationsEnabled}
              />
            )}
            {data.metrics?.locales && (
              <KPIGauge
                current={data.metrics.locales.current}
                max={data.metrics.locales.max}
                label="locales"
                color={colors.secondary}
                animate={animationsEnabled}
              />
            )}
            {data.metrics?.entities && (
              <KPIGauge
                current={data.metrics.entities.current}
                max={data.metrics.entities.max}
                label="entities"
                color="#22c55e"
                animate={animationsEnabled}
              />
            )}
          </div>
        </div>
      )}

      {/* Key (if different from display name) */}
      {data.key && data.key !== data.displayName && !hasMetrics && (
        <p
          className="relative font-mono text-xs mt-1 truncate"
          style={{ color: `${colors.primary}70` }}
        >
          {data.key}
        </p>
      )}
    </div>
  );
});
