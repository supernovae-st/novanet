'use client';

/**
 * Output Layer Helper Components
 *
 * Shared components for PageNative, BlockNative, OutputArtifact cards.
 * Layer color: Green #22c55e
 *
 * Components:
 * - GeneratedBadge: Shows generated trait indicator
 * - StatusBadge: Shows publication status (draft/published/archived)
 * - LocaleBadge: Shows locale with flag
 * - VersionHistory: Shows version chain
 * - AssemblyInfo: Shows assembly/generation timestamp and version
 * - ContentPreview: Shows JSON content preview
 * - BundleStats: Shows page/block counts for OutputArtifact
 * - ChecksumBadge: Shows integrity checksum
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { DURATIONS } from '../../animationPresets';

// =============================================================================
// GeneratedBadge - Shows generated trait indicator
// =============================================================================

export interface GeneratedBadgeProps {
  animate?: boolean;
  color?: string;
}

export const GeneratedBadge = memo(function GeneratedBadge({
  animate = true,
  color = '#22c55e',
}: GeneratedBadgeProps) {
  const Badge = animate ? motion.span : 'span';

  return (
    <Badge
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono font-bold"
      style={{
        color: color,
        backgroundColor: `${color}15`,
        border: `1px dotted ${color}60`,
      }}
      {...(animate && {
        animate: {
          boxShadow: [
            `0 0 0px ${color}00`,
            `0 0 8px ${color}40`,
            `0 0 0px ${color}00`,
          ],
        },
        transition: {
          duration: 2,
          repeat: Infinity,
          ease: 'easeInOut',
        },
      })}
    >
      <span>✦</span>
      <span>GENERATED</span>
    </Badge>
  );
});

// =============================================================================
// OutputStatusBadge - Shows publication status
// =============================================================================

export interface OutputStatusBadgeProps {
  status: 'draft' | 'approved' | 'published' | 'archived' | 'validated';
}

const outputStatusConfig = {
  draft: { label: 'Draft', color: '#6b7280', icon: '○' },
  approved: { label: 'Approved', color: '#eab308', icon: '◎' },
  validated: { label: 'Validated', color: '#0ea5e9', icon: '◉' },
  published: { label: 'Published', color: '#22c55e', icon: '●' },
  archived: { label: 'Archived', color: '#6b7280', icon: '◌' },
};

export const OutputStatusBadge = memo(function OutputStatusBadge({
  status,
}: OutputStatusBadgeProps) {
  const config = outputStatusConfig[status];

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        color: config.color,
        backgroundColor: `${config.color}15`,
        border: `1px solid ${config.color}30`,
      }}
    >
      <span>{config.icon}</span>
      <span>{config.label}</span>
    </span>
  );
});

// =============================================================================
// OutputLocaleBadge - Shows locale with flag
// =============================================================================

export interface OutputLocaleBadgeProps {
  locale: string;
  color?: string;
}

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

export const OutputLocaleBadge = memo(function OutputLocaleBadge({
  locale,
  color = '#22c55e',
}: OutputLocaleBadgeProps) {
  const flag = localeFlags[locale] || '🌐';

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-mono font-bold"
      style={{
        color: color,
        backgroundColor: `${color}20`,
        border: `1px solid ${color}40`,
      }}
    >
      <span>{flag}</span>
      <span>{locale}</span>
    </span>
  );
});

// =============================================================================
// VersionHistory - Shows version with history indicator
// =============================================================================

export interface VersionHistoryProps {
  version: number;
  hasPreviousVersion?: boolean;
  color?: string;
}

export const VersionHistory = memo(function VersionHistory({
  version,
  hasPreviousVersion = false,
  color = '#22c55e',
}: VersionHistoryProps) {
  return (
    <div className="flex items-center gap-1">
      <span
        className="px-1.5 py-0.5 rounded text-[9px] font-mono font-bold"
        style={{
          color: color,
          backgroundColor: `${color}15`,
          border: `1px solid ${color}30`,
        }}
      >
        v{version}
      </span>
      {hasPreviousVersion && (
        <span className="text-[8px] text-white/40 font-mono">← prev</span>
      )}
    </div>
  );
});

// =============================================================================
// AssemblyInfo - Shows assembly/generation info
// =============================================================================

export interface AssemblyInfoProps {
  timestamp?: string;
  version?: string;
  color?: string;
  label?: string;
}

export const AssemblyInfo = memo(function AssemblyInfo({
  timestamp,
  version,
  color = '#22c55e',
  label = 'assembled',
}: AssemblyInfoProps) {
  const formattedTime = timestamp
    ? new Date(timestamp).toLocaleString('en-US', {
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
      })
    : null;

  return (
    <div
      className="flex items-center justify-between text-[8px] font-mono px-2 py-1 rounded"
      style={{
        backgroundColor: `${color}08`,
        border: `1px solid ${color}15`,
      }}
    >
      <div className="flex items-center gap-2">
        <span className="text-white/50">{label}:</span>
        {formattedTime && <span className="text-white/70">{formattedTime}</span>}
      </div>
      {version && (
        <span className="text-white/40">assembler v{version}</span>
      )}
    </div>
  );
});

// =============================================================================
// ContentPreview - Shows JSON content preview
// =============================================================================

export interface ContentPreviewProps {
  content: Record<string, unknown>;
  maxKeys?: number;
  color?: string;
}

export const ContentPreview = memo(function ContentPreview({
  content,
  maxKeys = 3,
  color = '#22c55e',
}: ContentPreviewProps) {
  const keys = Object.keys(content);
  const displayKeys = keys.slice(0, maxKeys);
  const hasMore = keys.length > maxKeys;

  const formatValue = (value: unknown): string => {
    if (typeof value === 'string') {
      return value.length > 20 ? `"${value.slice(0, 20)}..."` : `"${value}"`;
    }
    if (typeof value === 'object' && value !== null) {
      return Array.isArray(value) ? `[${value.length}]` : '{...}';
    }
    return String(value);
  };

  return (
    <div
      className="p-2 rounded space-y-0.5 text-[9px] font-mono"
      style={{
        backgroundColor: `${color}08`,
        border: `1px dotted ${color}30`,
      }}
    >
      {displayKeys.map((key) => (
        <div key={key} className="flex items-center gap-1 truncate">
          <span style={{ color }}>{key}:</span>
          <span className="text-white/70">{formatValue(content[key])}</span>
        </div>
      ))}
      {hasMore && (
        <div className="text-white/40 italic">+{keys.length - maxKeys} more fields</div>
      )}
    </div>
  );
});

// =============================================================================
// BundleStats - Shows page/block counts for OutputArtifact
// =============================================================================

export interface BundleStatsProps {
  pagesIncluded: number;
  blocksIncluded: number;
  totalSizeBytes?: number;
  color?: string;
}

export const BundleStats = memo(function BundleStats({
  pagesIncluded,
  blocksIncluded,
  totalSizeBytes,
  color = '#22c55e',
}: BundleStatsProps) {
  const formatSize = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  };

  return (
    <div
      className="grid grid-cols-3 gap-2 p-2 rounded text-center"
      style={{
        backgroundColor: `${color}08`,
        border: `1px solid ${color}20`,
      }}
    >
      <div>
        <div className="text-lg font-bold" style={{ color }}>
          {pagesIncluded}
        </div>
        <div className="text-[8px] text-white/50 font-mono">pages</div>
      </div>
      <div>
        <div className="text-lg font-bold" style={{ color }}>
          {blocksIncluded}
        </div>
        <div className="text-[8px] text-white/50 font-mono">blocks</div>
      </div>
      {typeof totalSizeBytes === 'number' && (
        <div>
          <div className="text-sm font-bold text-white/80">
            {formatSize(totalSizeBytes)}
          </div>
          <div className="text-[8px] text-white/50 font-mono">size</div>
        </div>
      )}
    </div>
  );
});

// =============================================================================
// ChecksumBadge - Shows integrity checksum
// =============================================================================

export interface ChecksumBadgeProps {
  checksum?: string;
  color?: string;
}

export const ChecksumBadge = memo(function ChecksumBadge({
  checksum,
  color = '#22c55e',
}: ChecksumBadgeProps) {
  if (!checksum) return null;

  // Extract algorithm and show first 8 chars of hash
  const parts = checksum.split(':');
  const algorithm = parts.length > 1 ? parts[0] : 'hash';
  const hash = parts.length > 1 ? parts[1] : checksum;
  const shortHash = hash.slice(0, 8);

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        color: color,
        backgroundColor: `${color}10`,
        border: `1px solid ${color}20`,
      }}
    >
      <span className="text-white/50">{algorithm}:</span>
      <span>{shortHash}...</span>
    </span>
  );
});

// =============================================================================
// AnchorSlugBadge - Shows block anchor slug
// =============================================================================

export interface AnchorSlugBadgeProps {
  slug: string;
  color?: string;
}

export const AnchorSlugBadge = memo(function AnchorSlugBadge({
  slug,
  color = '#22c55e',
}: AnchorSlugBadgeProps) {
  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        color: color,
        backgroundColor: `${color}15`,
      }}
    >
      <span className="text-white/50">#</span>
      <span>{slug}</span>
    </span>
  );
});
