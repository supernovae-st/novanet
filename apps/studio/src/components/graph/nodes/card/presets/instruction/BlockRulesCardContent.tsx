'use client';

/**
 * BlockRulesCardContent - "Rules Engine" design for BlockRules nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = instruction) → yellow #eab308
 * - Border style → solid (defined trait)
 * - Shows validation rules, constraints, version
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ ⚖ RULES          v3 ●          │  ← Scale icon + version badge
 * │ ══════════════════════════════   │
 * │ hero:rules                       │
 * │ ┌────────────────────────────┐   │
 * │ │ Hero Block Rules           │   │  ← Display name
 * │ │ ─────────────────────────  │   │
 * │ │ constraints:               │   │
 * │ │ • max_length: 120          │   │  ← Rules preview
 * │ │ • required_fields: 3       │   │
 * │ │ • allowed_tags: h1,h2,p    │   │
 * │ │ +2 more                    │   │
 * │ └────────────────────────────┘   │
 * └──────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import { VersionBadge } from './InstructionHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface BlockRulesNodeData {
  id: string;
  type: 'BlockRules';
  key: string;
  displayName: string;
  /** Version number */
  version?: number;
  /** Active flag */
  is_active?: boolean;
  /** Description */
  description?: string;
  /** Validation rules (key-value pairs) */
  rules?: Record<string, string | number | boolean | string[]>;
  /** Number of violations */
  violations?: number;
}

export interface BlockRulesCardContentProps extends CardContext {
  data: BlockRulesNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const scaleVariants: Variants = {
  idle: { rotate: 0 },
  hover: {
    rotate: [-3, 3, -3, 0],
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    scale: 1.1,
    filter: 'drop-shadow(0 0 6px currentColor)',
  },
};

const rulesVariants: Variants = {
  idle: { opacity: 0.85 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Helper: Format rule value for display
// =============================================================================

const formatRuleValue = (value: string | number | boolean | string[]): string => {
  if (Array.isArray(value)) {
    return value.slice(0, 3).join(', ') + (value.length > 3 ? '...' : '');
  }
  if (typeof value === 'boolean') {
    return value ? 'yes' : 'no';
  }
  return String(value);
};

// =============================================================================
// Component
// =============================================================================

export const BlockRulesCardContent = memo(function BlockRulesCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: BlockRulesCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Glow style
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

  const ScaleIcon = animationsEnabled ? motion.span : 'span';
  const RulesWrapper = animationsEnabled ? motion.div : 'div';

  // Get rules entries
  const rulesEntries = data.rules ? Object.entries(data.rules) : [];
  const displayRules = rulesEntries.slice(0, 4);
  const hasMoreRules = rulesEntries.length > 4;

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (instruction) + Realm (org) + Trait (defined) + Class (BlockRules) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="instruction"
          realm="org"
          trait="defined"
          className="BlockRules"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        <VersionBadge
          version={data.version || 1}
          isActive={data.is_active !== false}
          color={colors.primary}
        />
      </div>

      {/* Rules key */}
      <h3 className="text-xs font-mono text-white/60 mb-1 truncate">{data.key}</h3>

      {/* Display name */}
      <h4 className="text-base font-bold text-white mb-2" style={glowStyle}>
        {data.displayName}
      </h4>

      {/* Description */}
      {data.description && (
        <p className="text-[10px] text-white/60 line-clamp-2 italic mb-3">
          {data.description}
        </p>
      )}

      {/* Rules section */}
      <RulesWrapper
        className="space-y-2"
        {...(animationsEnabled && {
          variants: rulesVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {displayRules.length > 0 && (
          <div
            className="p-2 rounded space-y-1"
            style={{
              backgroundColor: `${colors.primary}08`,
              border: `1px solid ${colors.primary}20`,
            }}
          >
            <span className="text-[9px] text-white/50 font-mono">constraints:</span>
            {displayRules.map(([key, value]) => (
              <div key={key} className="flex items-center gap-1 text-[9px] font-mono">
                <span style={{ color: colors.primary }}>•</span>
                <span className="text-white/70">{key}:</span>
                <span className="text-white/90 truncate">{formatRuleValue(value)}</span>
              </div>
            ))}
            {hasMoreRules && (
              <div className="text-[8px] text-white/40 italic">
                +{rulesEntries.length - 4} more
              </div>
            )}
          </div>
        )}

        {/* Violations count */}
        {typeof data.violations === 'number' && data.violations > 0 && (
          <div className="flex items-center gap-1 text-[9px]">
            <span
              className="px-1.5 py-0.5 rounded font-mono font-bold"
              style={{
                color: '#ef4444',
                backgroundColor: 'rgba(239, 68, 68, 0.15)',
                border: '1px solid rgba(239, 68, 68, 0.3)',
              }}
            >
              ⚠ {data.violations} violation{data.violations > 1 ? 's' : ''}
            </span>
          </div>
        )}
      </RulesWrapper>
    </div>
  );
});
