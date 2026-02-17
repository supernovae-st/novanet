'use client';

/**
 * Foundation Layer Helper Components
 *
 * Shared components for Foundation Layer cards (Project, Brand, BrandDesign, PromptStyle)
 * - KPIGauge: Progress bar with label
 * - ColorSwatch: Color chip with label
 * - TypographyPreview: Font preview
 * - TraitBadge: Voice/tone badge pills
 */

import { memo } from 'react';
import { motion } from 'motion/react';

// =============================================================================
// KPIGauge - Progress indicator for metrics
// =============================================================================

export interface KPIGaugeProps {
  current: number;
  max: number;
  label: string;
  color: string;
  /** Optional: animate on mount (default: true) */
  animate?: boolean;
}

export const KPIGauge = memo(function KPIGauge({
  current,
  max,
  label,
  color,
  animate = true,
}: KPIGaugeProps) {
  const percentage = max > 0 ? (current / max) * 100 : 0;

  return (
    <div className="flex flex-col gap-1">
      <div className="flex justify-between text-[9px]">
        <span className="text-white/60">{label}</span>
        <span style={{ color }}>{current}/{max}</span>
      </div>
      <div className="h-1.5 rounded-full bg-white/10 overflow-hidden">
        {animate ? (
          <motion.div
            className="h-full rounded-full"
            style={{ backgroundColor: color }}
            initial={{ width: 0 }}
            animate={{ width: `${percentage}%` }}
            transition={{ duration: 0.5, ease: 'easeOut' }}
          />
        ) : (
          <div
            className="h-full rounded-full"
            style={{ backgroundColor: color, width: `${percentage}%` }}
          />
        )}
      </div>
    </div>
  );
});

// =============================================================================
// ColorSwatch - Color chip with label and hex code
// =============================================================================

export interface ColorSwatchProps {
  color: string;
  label: string;
  /** Optional: show hex code (default: true) */
  showHex?: boolean;
  /** Optional: animate on hover (default: true) */
  animate?: boolean;
}

export const ColorSwatch = memo(function ColorSwatch({
  color,
  label,
  showHex = true,
  animate = true,
}: ColorSwatchProps) {
  const SwatchElement = animate ? motion.div : 'div';

  return (
    <div className="flex items-center gap-2">
      <SwatchElement
        className="w-4 h-4 rounded-full border border-white/20"
        style={{ backgroundColor: color }}
        {...(animate && {
          whileHover: { scale: 1.2, boxShadow: `0 0 12px ${color}` },
        })}
      />
      <span className="text-xs text-white/70">{label}</span>
      {showHex && (
        <span className="text-[9px] font-mono text-white/40">{color}</span>
      )}
    </div>
  );
});

// =============================================================================
// TypographyPreview - Font sample with metadata
// =============================================================================

export interface TypographyPreviewProps {
  fontFamily: string;
  fontSize?: number;
  lineHeight?: number;
  /** Optional: preview text (default: "Aa") */
  sample?: string;
}

export const TypographyPreview = memo(function TypographyPreview({
  fontFamily,
  fontSize = 16,
  lineHeight = 24,
  sample = 'Aa',
}: TypographyPreviewProps) {
  return (
    <div className="flex items-baseline gap-2 px-2 py-1 rounded bg-white/5">
      <span
        className="text-lg text-white/80"
        style={{ fontFamily }}
      >
        {sample}
      </span>
      <span className="text-[9px] font-mono text-white/40">
        {fontFamily} {fontSize}/{lineHeight}
      </span>
    </div>
  );
});

// =============================================================================
// TraitBadge - Small pill badge for voice/tone traits
// =============================================================================

export interface TraitBadgeProps {
  label: string;
  color: string;
  /** Optional: filled vs outlined style (default: false = outlined) */
  filled?: boolean;
}

export const TraitBadge = memo(function TraitBadge({
  label,
  color,
  filled = false,
}: TraitBadgeProps) {
  return (
    <span
      className="px-1.5 py-0.5 rounded text-[9px] font-medium"
      style={{
        color: filled ? '#000' : color,
        backgroundColor: filled ? color : `${color}20`,
        border: filled ? 'none' : `1px solid ${color}40`,
      }}
    >
      {label}
    </span>
  );
});

// =============================================================================
// SectionLabel - Section header with icon
// =============================================================================

export interface SectionLabelProps {
  icon: string;
  label: string;
  sublabel?: string;
  color: string;
}

export const SectionLabel = memo(function SectionLabel({
  icon,
  label,
  sublabel,
  color,
}: SectionLabelProps) {
  return (
    <div className="flex items-center gap-2">
      <span style={{ color }}>{icon}</span>
      <span className="text-[10px] font-bold uppercase text-white/60">{label}</span>
      {sublabel && (
        <span className="text-[9px] text-white/40">{sublabel}</span>
      )}
    </div>
  );
});

// =============================================================================
// PlatformBadge - AI platform indicator (MJ, DALL-E, SD, Sora)
// =============================================================================

export interface PlatformBadgeProps {
  platform: 'midjourney' | 'dalle' | 'stable-diffusion' | 'sora' | string;
  active?: boolean;
}

const PLATFORM_LABELS: Record<string, { short: string; color: string }> = {
  'midjourney': { short: 'MJ', color: '#5865f2' },
  'dalle': { short: 'D-E', color: '#00a67e' },
  'stable-diffusion': { short: 'SD', color: '#ff6b6b' },
  'sora': { short: 'Sora', color: '#0ea5e9' },
};

export const PlatformBadge = memo(function PlatformBadge({
  platform,
  active = true,
}: PlatformBadgeProps) {
  const config = PLATFORM_LABELS[platform] || { short: platform.slice(0, 2).toUpperCase(), color: '#6c71c4' };

  return (
    <span
      className="px-1.5 py-0.5 rounded text-[8px] font-bold uppercase"
      style={{
        color: active ? config.color : '#666',
        backgroundColor: active ? `${config.color}20` : '#333',
        border: `1px solid ${active ? `${config.color}40` : '#444'}`,
        opacity: active ? 1 : 0.5,
      }}
    >
      {config.short}
    </span>
  );
});
