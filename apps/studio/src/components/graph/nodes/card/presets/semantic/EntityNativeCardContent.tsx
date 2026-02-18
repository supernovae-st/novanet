'use client';

/**
 * EntityNativeCardContent - Locale-specific content card for EntityNative nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = semantic) → orange #f97316
 * - Border style → dashed (authored trait)
 * - SUBTLE effects only (parent Entity has premium effects)
 * - Locale badge with locale-specific accent color
 *
 * Design Philosophy (v0.13.1):
 * - CHILD nodes are smaller and more subtle than PARENT (Entity)
 * - Same orange color scheme as Entity parent for visual coherence
 * - Simple dashed border (no heavy effects like BorderBeam)
 * - Locale badge INLINE (not a giant header) with flag + BCP47
 * - Diamond OUTLINE ◇ (not filled) to distinguish from Entity's ◆◆◆
 *
 * Layout:
 * ┌────────────────────────────────────────┐
 * │ ⊕ SEMANTIC │ ORG  ● ─ ─ ─ AUTHORED    │  ← TaxonomyBadge (left)
 * ├────────────────────────────────────────┤
 * │ 🇫🇷 fr-FR                              │  ← Locale badge (prominent)
 * │ entity:wifi-qr-generator@fr-FR        │  ← Composite key (monospace)
 * │                                        │
 * │ ◇  WiFi QR Générateur                 │  ← Diamond OUTLINE + Display name
 * │                                        │
 * │ ┌──────────────────────────────────┐  │
 * │ │ Générateur de QR Codes WiFi...   │  │  ← Description
 * │ │                                  │  │
 * │ │ ✍️ Human  │  ● Published  │ v1   │  │  ← Curation + Status + Version
 * │ └──────────────────────────────────┘  │
 * └────────────────────────────────────────┘
 *    ↑ Simple dashed border (authored trait)
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import {
  CurationBadge,
  StatusBadge,
  BenefitsList,
  getLocaleAccentColor,
  getLocaleFlag,
  toNumber,
} from './SemanticHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { Diamond, ArrowUpRight } from 'lucide-react';

// =============================================================================
// Types
// =============================================================================

export interface EntityNativeNodeData {
  id: string;
  type: 'EntityNative';
  key: string; // entity:{entity_key}@{locale}
  displayName: string;
  /** Parent entity key (can be extracted from composite key) */
  entity_key?: string;
  /** Locale key (from YAML schema property) */
  locale_key?: string;
  /** BCP-47 locale code (from Graph2D extraction) */
  locale?: string;
  /** Localized description */
  description?: string;
  /** Content curation status */
  curation_status?: 'human_authored' | 'machine_translated' | 'ai_generated' | 'ai_generated_reviewed';
  /** Publication status */
  status?: 'draft' | 'reviewed' | 'published';
  /** Localized definition */
  definition?: string;
  /** Value propositions */
  benefits?: string[];
  /** Usage examples */
  usage_examples?: string[];
  /** Content version */
  version?: number;
  /** Audience segment */
  audience_segment?: string;
  /** Cultural notes */
  cultural_notes?: string;
}

export interface EntityNativeCardContentProps extends CardContext {
  data: EntityNativeNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const contentVariants: Variants = {
  idle: { opacity: 0.9 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

const iconVariants: Variants = {
  idle: { scale: 1, rotate: 0 },
  hover: { scale: 1.1, rotate: 45, transition: { duration: DURATIONS.normal } },
  selected: { scale: 1.15, rotate: 45, filter: 'drop-shadow(0 0 8px currentColor)' },
};

const localeBadgeVariants: Variants = {
  idle: { scale: 1 },
  hover: { scale: 1.05 },
  selected: { scale: 1.1, boxShadow: '0 0 20px currentColor' },
};

// =============================================================================
// Component
// =============================================================================

export const EntityNativeCardContent = memo(function EntityNativeCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: EntityNativeCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Locale code (Graph2D provides data.locale, YAML schema provides data.locale_key)
  // Also try to extract from composite key pattern: entity:{key}@{locale}
  const extractedLocale = data.key?.includes('@') ? data.key.split('@')[1] : '';
  const localeCode = data.locale || data.locale_key || extractedLocale || '';

  // Extract entity_key from composite key if not provided directly
  const extractedEntityKey = data.key?.includes(':')
    ? data.key.split(':')[1]?.split('@')[0]
    : '';
  const entityKey = data.entity_key || extractedEntityKey || '';

  // Locale accent color for badge background
  const localeAccent = useMemo(
    () => getLocaleAccentColor(localeCode),
    [localeCode]
  );

  // Get flag emoji for locale
  const localeFlag = useMemo(
    () => getLocaleFlag(localeCode),
    [localeCode]
  );

  // Glow style for display name
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

  const IconWrapper = animationsEnabled ? motion.div : 'div';
  const ContentWrapper = animationsEnabled ? motion.div : 'div';
  const LocaleBadgeWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="relative px-5 py-5">
      {/* === DASHED BORDER EFFECT (ADR-005: authored = dashed) === */}
      {/* Simple dashed border instead of heavy BorderBeam for authored trait */}
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          borderRadius: 12,
          border: `2px dashed ${colors.primary}${selected ? '80' : isHovered ? '60' : '40'}`,
          boxShadow: selected
            ? `0 0 20px ${colors.primary}25, inset 0 0 20px ${colors.primary}08`
            : isHovered
              ? `0 0 12px ${colors.primary}15`
              : 'none',
          transition: 'all 0.3s ease-out',
        }}
      />

      {/* Row 1: Taxonomy badge + NATIVE badge */}
      <div className="flex items-center justify-between mb-4">
        <TaxonomyBadge
          layer="semantic"
          realm="org"
          trait="authored"
          className="EntityNative"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        {/* NATIVE badge (top right) */}
        <div
          className="flex items-center gap-1.5 px-2 py-1 rounded-md"
          style={{
            background: `${colors.primary}20`,
            border: `1px solid ${colors.primary}50`,
          }}
        >
          <Diamond size={12} style={{ color: colors.primary }} strokeWidth={2} />
          <span className="text-[10px] font-bold" style={{ color: colors.primary }}>
            NATIVE
          </span>
        </div>
      </div>

      {/* Row 2: Locale badge (prominent) - CENTERED */}
      {localeCode && (
        <LocaleBadgeWrapper
          className="flex items-center justify-center gap-2 mb-4 px-3 py-2 rounded-lg"
          style={{
            background: `linear-gradient(135deg, ${localeAccent}20, ${localeAccent}08)`,
            border: `1.5px solid ${localeAccent}40`,
            boxShadow: selected ? `0 0 20px ${localeAccent}30` : 'none',
          }}
          {...(animationsEnabled && {
            variants: localeBadgeVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {/* Flag emoji */}
          <span className="text-2xl">{localeFlag}</span>
          {/* BCP47 code */}
          <span
            className="text-lg font-bold font-mono"
            style={{ color: localeAccent }}
          >
            {localeCode}
          </span>
        </LocaleBadgeWrapper>
      )}

      {/* Row 3: Composite key in styled block */}
      <div
        className="flex items-center justify-center mb-4 px-4 py-2 rounded-lg"
        style={{
          background: `linear-gradient(135deg, ${colors.primary}15, ${colors.primary}05)`,
          border: `1.5px dashed ${colors.primary}30`,
        }}
      >
        <span
          className="text-sm font-mono font-bold"
          style={{ color: colors.primary }}
        >
          {data.key}
        </span>
      </div>

      {/* Row 4: Icon + DisplayName (horizontal layout) */}
      <div className="flex items-center gap-4 mb-4">
        {/* Diamond OUTLINE icon — dashed border (authored trait), NO fill */}
        <IconWrapper
          className="flex-shrink-0 w-12 h-12 rounded-lg flex items-center justify-center"
          style={{
            background: `linear-gradient(135deg, ${colors.primary}15, ${colors.primary}05)`,
            border: `2px dashed ${colors.primary}45`,
            boxShadow: selected ? `0 0 15px ${colors.primary}30` : 'none',
            transition: 'box-shadow 0.2s ease-out',
          }}
          {...(animationsEnabled && {
            variants: iconVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {/* Diamond outline ◇ (NOT filled, unlike Entity's ◆) */}
          <Diamond
            size={24}
            style={{ color: colors.primary }}
            strokeWidth={2}
          />
        </IconWrapper>

        {/* Display name (localized) */}
        <h4 className="text-xl font-bold text-white leading-tight" style={glowStyle}>
          {data.displayName}
        </h4>
      </div>

      {/* Parent reference (subtle link to Entity) */}
      {entityKey && (
        <div
          className="flex items-center justify-center gap-2 mb-4 px-2 py-1.5 rounded-md"
          style={{
            background: `${colors.primary}08`,
            border: `1px dashed ${colors.primary}20`,
          }}
        >
          <Diamond size={10} style={{ color: colors.primary }} className="opacity-60" />
          <span className="text-[10px] font-mono text-white/50">
            parent: <span style={{ color: colors.primary }}>{entityKey}</span>
          </span>
          <ArrowUpRight size={10} className="opacity-40" style={{ color: colors.primary }} />
        </div>
      )}

      {/* Content section */}
      <ContentWrapper
        className="p-3 rounded-lg space-y-2"
        style={{
          background: `${colors.primary}06`,
          border: `1px dashed ${colors.primary}20`,
        }}
        {...(animationsEnabled && {
          variants: contentVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Description (truncated) */}
        {data.description && (
          <p className="text-[11px] text-white/70 line-clamp-2">{data.description}</p>
        )}

        {/* Curation + Status + Version row */}
        <div className="flex items-center gap-2 flex-wrap pt-1">
          {data.curation_status && (
            <CurationBadge status={data.curation_status} animate={animationsEnabled} />
          )}
          {data.status && <StatusBadge status={data.status} />}
          <span className="text-[9px] text-white/30 font-mono ml-auto">
            v{toNumber(data.version) || 1}
          </span>
        </div>

        {/* Benefits list (if any) */}
        {data.benefits && data.benefits.length > 0 && (
          <BenefitsList
            benefits={data.benefits}
            maxDisplay={2}
            color={colors.primary}
            animate={animationsEnabled}
          />
        )}
      </ContentWrapper>
    </div>
  );
});
