'use client';

/**
 * Instruction Layer Helper Components
 *
 * Shared components for BlockInstruction, BlockType, BlockRules, PromptArtifact cards.
 * Layer color: Yellow #eab308
 *
 * Components:
 * - CategoryBadge: Shows block category (header/body/footer/sidebar)
 * - DirectiveBadge: Shows directive type (GENERATE/FIXED/TRANSLATE)
 * - ReferenceCounter: Shows @entity: and @link: counts
 * - VersionBadge: Shows version with active state
 * - TokenCounter: Shows token count with progress bar
 * - ContentPreview: Syntax-highlighted instruction preview
 * - OrderBadge: Shows block order number
 * - InclusionFlags: Shows include_concepts/voice/culture
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { DURATIONS } from '../../animationPresets';

// =============================================================================
// CategoryBadge - Shows block category
// =============================================================================

export interface CategoryBadgeProps {
  category: 'header' | 'body' | 'footer' | 'sidebar';
  color?: string;
}

const categoryConfig = {
  header: { icon: '▲', label: 'HEADER' },
  body: { icon: '■', label: 'BODY' },
  footer: { icon: '▼', label: 'FOOTER' },
  sidebar: { icon: '◀', label: 'SIDEBAR' },
};

export const CategoryBadge = memo(function CategoryBadge({
  category,
  color = '#eab308',
}: CategoryBadgeProps) {
  const config = categoryConfig[category];

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-mono font-bold"
      style={{
        color: color,
        backgroundColor: `${color}15`,
        border: `1px solid ${color}40`,
      }}
    >
      <span>{config.icon}</span>
      <span>{config.label}</span>
    </span>
  );
});

// =============================================================================
// DirectiveBadge - Shows directive type
// =============================================================================

export interface DirectiveBadgeProps {
  directive: 'GENERATE' | 'FIXED' | 'TRANSLATE';
  animate?: boolean;
}

const directiveConfig = {
  GENERATE: { icon: '⚡', color: '#22c55e', description: 'LLM generates' },
  FIXED: { icon: '🔒', color: '#6b7280', description: 'Invariant value' },
  TRANSLATE: { icon: '🌐', color: '#0ea5e9', description: 'Native per locale' },
};

export const DirectiveBadge = memo(function DirectiveBadge({
  directive,
  animate = true,
}: DirectiveBadgeProps) {
  const config = directiveConfig[directive];
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
      <span>{directive}</span>
    </Badge>
  );
});

// =============================================================================
// ReferenceCounter - Shows @entity: and @link: counts
// =============================================================================

export interface ReferenceCounterProps {
  entityRefs?: number;
  linkRefs?: number;
  color?: string;
}

export const ReferenceCounter = memo(function ReferenceCounter({
  entityRefs = 0,
  linkRefs = 0,
  color = '#eab308',
}: ReferenceCounterProps) {
  return (
    <div
      className="flex items-center gap-2 px-2 py-1 rounded text-[9px] font-mono"
      style={{
        backgroundColor: `${color}08`,
        border: `1px solid ${color}20`,
      }}
    >
      <span className="text-white/50">@entity:</span>
      <span style={{ color }}>{entityRefs}</span>
      <span className="text-white/30">|</span>
      <span className="text-white/50">@link:</span>
      <span style={{ color }}>{linkRefs}</span>
    </div>
  );
});

// =============================================================================
// VersionBadge - Shows version with active state
// =============================================================================

export interface VersionBadgeProps {
  version: number;
  isActive?: boolean;
  color?: string;
}

export const VersionBadge = memo(function VersionBadge({
  version,
  isActive = true,
  color = '#eab308',
}: VersionBadgeProps) {
  const activeColor = isActive ? '#22c55e' : '#6b7280';

  return (
    <span
      className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono"
      style={{
        color: activeColor,
        backgroundColor: `${activeColor}15`,
        border: `1px solid ${activeColor}30`,
      }}
    >
      {isActive && <span className="w-1.5 h-1.5 rounded-full bg-current" />}
      <span>v{version}</span>
    </span>
  );
});

// =============================================================================
// TokenCounter - Shows token count with progress bar
// =============================================================================

export interface TokenCounterProps {
  tokenCount: number;
  maxTokens?: number;
  color?: string;
}

export const TokenCounter = memo(function TokenCounter({
  tokenCount,
  maxTokens = 4096,
  color = '#eab308',
}: TokenCounterProps) {
  const percentage = Math.min((tokenCount / maxTokens) * 100, 100);
  const isWarning = percentage > 80;
  const isCritical = percentage > 95;
  const barColor = isCritical ? '#ef4444' : isWarning ? '#f97316' : color;

  return (
    <div className="space-y-1">
      <div className="flex items-center justify-between text-[8px] font-mono">
        <span className="text-white/50">tokens:</span>
        <span style={{ color: barColor }}>
          {tokenCount.toLocaleString()} / {maxTokens.toLocaleString()}
        </span>
      </div>
      <div
        className="h-1 rounded-full overflow-hidden"
        style={{ backgroundColor: `${color}20` }}
      >
        <div
          className="h-full rounded-full transition-all duration-300"
          style={{
            width: `${percentage}%`,
            backgroundColor: barColor,
          }}
        />
      </div>
    </div>
  );
});

// =============================================================================
// ContentPreview - Syntax-highlighted instruction preview (safe rendering)
// =============================================================================

export interface ContentPreviewProps {
  content: string;
  maxLines?: number;
  color?: string;
}

// Highlight syntax safely using React elements
const highlightLine = (line: string, color: string, index: number): React.ReactNode => {
  const parts: React.ReactNode[] = [];
  let lastIndex = 0;
  let keyIndex = 0;

  // Patterns to highlight
  const patterns = [
    { regex: /\[(GENERATE|FIXED|TRANSLATE)\]/g, type: 'directive' },
    { regex: /@entity:[a-z0-9-]+/g, type: 'entity' },
    { regex: /@link:[a-z0-9-]+/g, type: 'link' },
  ];

  // Find all matches with their positions
  const matches: { start: number; end: number; text: string; type: string }[] = [];

  for (const { regex, type } of patterns) {
    let match;
    while ((match = regex.exec(line)) !== null) {
      matches.push({
        start: match.index,
        end: match.index + match[0].length,
        text: match[0],
        type,
      });
    }
  }

  // Sort by position
  matches.sort((a, b) => a.start - b.start);

  // Build parts
  for (const match of matches) {
    // Add text before match
    if (match.start > lastIndex) {
      parts.push(
        <span key={`text-${index}-${keyIndex++}`} className="text-white/70">
          {line.slice(lastIndex, match.start)}
        </span>
      );
    }

    // Add highlighted match
    let matchColor = color;
    if (match.type === 'directive') {
      if (match.text.includes('GENERATE')) matchColor = '#22c55e';
      else if (match.text.includes('FIXED')) matchColor = '#6b7280';
      else if (match.text.includes('TRANSLATE')) matchColor = '#0ea5e9';
    } else if (match.type === 'entity') {
      matchColor = '#f97316';
    } else if (match.type === 'link') {
      matchColor = '#8b5cf6';
    }

    parts.push(
      <span key={`match-${index}-${keyIndex++}`} style={{ color: matchColor }} className="font-bold">
        {match.text}
      </span>
    );

    lastIndex = match.end;
  }

  // Add remaining text
  if (lastIndex < line.length) {
    parts.push(
      <span key={`tail-${index}-${keyIndex++}`} className="text-white/70">
        {line.slice(lastIndex)}
      </span>
    );
  }

  return parts.length > 0 ? parts : <span className="text-white/70">{line}</span>;
};

export const ContentPreview = memo(function ContentPreview({
  content,
  maxLines = 4,
  color = '#eab308',
}: ContentPreviewProps) {
  const lines = content.split('\n').slice(0, maxLines);
  const hasMore = content.split('\n').length > maxLines;

  return (
    <div
      className="p-2 rounded text-[9px] font-mono space-y-0.5 overflow-hidden"
      style={{
        backgroundColor: `${color}08`,
        border: `1px solid ${color}20`,
      }}
    >
      {lines.map((line, i) => (
        <div key={i} className="truncate">
          {highlightLine(line, color, i)}
        </div>
      ))}
      {hasMore && (
        <div className="text-white/40 italic">... +{content.split('\n').length - maxLines} more lines</div>
      )}
    </div>
  );
});

// =============================================================================
// OrderBadge - Shows block order number
// =============================================================================

export interface OrderBadgeProps {
  order: number;
  color?: string;
}

export const OrderBadge = memo(function OrderBadge({
  order,
  color = '#eab308',
}: OrderBadgeProps) {
  return (
    <span
      className="inline-flex items-center justify-center w-5 h-5 rounded-full text-[10px] font-bold font-mono"
      style={{
        color: color,
        backgroundColor: `${color}20`,
        border: `1px solid ${color}40`,
      }}
    >
      {order}
    </span>
  );
});

// =============================================================================
// InclusionFlags - Shows include_concepts/voice/culture
// =============================================================================

export interface InclusionFlagsProps {
  includeConcepts?: boolean;
  includeVoice?: boolean;
  includeCulture?: boolean;
  color?: string;
}

const flagConfig = {
  concepts: { icon: '💡', label: 'concepts' },
  voice: { icon: '🗣️', label: 'voice' },
  culture: { icon: '🌍', label: 'culture' },
};

export const InclusionFlags = memo(function InclusionFlags({
  includeConcepts = false,
  includeVoice = false,
  includeCulture = false,
  color = '#eab308',
}: InclusionFlagsProps) {
  const flags = [
    { key: 'concepts', enabled: includeConcepts, ...flagConfig.concepts },
    { key: 'voice', enabled: includeVoice, ...flagConfig.voice },
    { key: 'culture', enabled: includeCulture, ...flagConfig.culture },
  ];

  const enabledCount = flags.filter((f) => f.enabled).length;
  if (enabledCount === 0) return null;

  return (
    <div className="flex items-center gap-1 flex-wrap">
      {flags
        .filter((f) => f.enabled)
        .map((flag) => (
          <span
            key={flag.key}
            className="inline-flex items-center gap-0.5 px-1 py-0.5 rounded text-[8px]"
            style={{
              color: color,
              backgroundColor: `${color}15`,
            }}
          >
            <span>{flag.icon}</span>
            <span className="font-mono">{flag.label}</span>
          </span>
        ))}
    </div>
  );
});

// =============================================================================
// SchemaPropertyList - Shows JSON schema properties
// =============================================================================

export interface SchemaPropertyListProps {
  properties: Record<string, { type: string; required?: boolean }>;
  maxDisplay?: number;
  color?: string;
}

export const SchemaPropertyList = memo(function SchemaPropertyList({
  properties,
  maxDisplay = 4,
  color = '#eab308',
}: SchemaPropertyListProps) {
  const entries = Object.entries(properties);
  const displayEntries = entries.slice(0, maxDisplay);
  const hasMore = entries.length > maxDisplay;

  return (
    <div className="space-y-0.5">
      <span className="text-[9px] text-white/50 font-mono">schema:</span>
      <div
        className="p-1.5 rounded text-[8px] font-mono space-y-0.5"
        style={{
          backgroundColor: `${color}08`,
          border: `1px solid ${color}15`,
        }}
      >
        {displayEntries.map(([key, value]) => (
          <div key={key} className="flex items-center gap-1">
            {value.required && (
              <span style={{ color: '#ef4444' }}>*</span>
            )}
            <span className="text-white/70">{key}:</span>
            <span style={{ color }}>{value.type}</span>
          </div>
        ))}
        {hasMore && (
          <div className="text-white/40 italic">+{entries.length - maxDisplay} more</div>
        )}
      </div>
    </div>
  );
});

// =============================================================================
// CompilationStatus - Shows prompt compilation status
// =============================================================================

export interface CompilationStatusProps {
  status: 'pending' | 'compiled' | 'error';
  compiledAt?: string;
  color?: string;
}

const statusConfig = {
  pending: { icon: '⏳', label: 'Pending', color: '#6b7280' },
  compiled: { icon: '✓', label: 'Compiled', color: '#22c55e' },
  error: { icon: '✗', label: 'Error', color: '#ef4444' },
};

export const CompilationStatus = memo(function CompilationStatus({
  status,
  compiledAt,
  color = '#eab308',
}: CompilationStatusProps) {
  const config = statusConfig[status];

  return (
    <div className="flex items-center gap-2">
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
      {compiledAt && (
        <span className="text-[8px] text-white/40 font-mono">{compiledAt}</span>
      )}
    </div>
  );
});
