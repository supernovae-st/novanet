'use client';

/**
 * AsciiPreview - Renders ASCII art preview for context views
 *
 * Supports 3 styles:
 * - Tree: Hierarchical structure with ├─ └─ │
 * - Flow: Relational arrows with ══► ──► ╦ ╠
 * - Compact: Progress bars and status indicators
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import type { AsciiStyle, ViewId } from '@/config/viewTypes';
import type { ViewStats } from '@/hooks/useContextViews';

// =============================================================================
// TYPES
// =============================================================================

interface AsciiPreviewProps {
  viewId: ViewId;
  style: AsciiStyle;
  stats: ViewStats;
  nodeKey: string;
  className?: string;
}

interface AsciiLine {
  text: string;
  indent: number;
  color?: string;
}

// =============================================================================
// ASCII SYMBOLS
// =============================================================================

const SYM = {
  // Tree
  branch: '├─',
  lastBranch: '└─',
  vertical: '│ ',
  space: '  ',

  // Flow
  arrow: '──►',
  doubleArrow: '══►',
  splitStart: '──┬──►',
  splitMid: '  ├──►',
  splitEnd: '  └──►',

  // Compact
  full: '█',
  half: '▓',
  quarter: '▒',
  empty: '░',
  check: '✓',
  pending: '○',
} as const;

// =============================================================================
// TREE STYLE RENDERER
// =============================================================================

function renderTreeStyle(
  nodeKey: string,
  stats: ViewStats,
  viewId: ViewId
): AsciiLine[] {
  const lines: AsciiLine[] = [];
  const { nodesByType, arcsByType } = stats;

  // Root node
  lines.push({ text: nodeKey, indent: 0 });

  // Get arc types for this view
  const arcEntries = Object.entries(arcsByType);
  const nodeEntries = Object.entries(nodesByType);

  if (viewId === 'composition') {
    // Page → Block hierarchy
    const blockCount = nodesByType['Block'] || 0;
    if (blockCount > 0) {
      lines.push({ text: `${SYM.branch}HAS_BLOCK`, indent: 1, color: 'text-blue-400' });
      const displayCount = Math.min(blockCount, 3);
      for (let i = 0; i < displayCount; i++) {
        const isLast = i === displayCount - 1 && blockCount <= 3;
        const sym = isLast ? SYM.lastBranch : SYM.branch;
        lines.push({ text: `${sym}Block (${i + 1})`, indent: 2 });
      }
      if (blockCount > 3) {
        lines.push({ text: `${SYM.lastBranch}+${blockCount - 3} more`, indent: 2, color: 'text-white/50' });
      }
    }
  } else if (viewId === 'knowledge') {
    // Locale → Sets → Atoms
    const setTypes = ['TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet'];
    const atomTypes = ['Term', 'Expression', 'Pattern', 'CultureRef'];

    setTypes.forEach((setType, idx) => {
      const count = nodesByType[setType] || 0;
      if (count > 0) {
        const isLast = idx === setTypes.length - 1;
        const sym = isLast ? SYM.lastBranch : SYM.branch;
        lines.push({ text: `${sym}${setType} (${count})`, indent: 1 });

        // Show atom count
        const atomType = atomTypes[idx];
        const atomCount = nodesByType[atomType] || 0;
        if (atomCount > 0) {
          lines.push({ text: `${SYM.lastBranch}${atomType} (${atomCount})`, indent: 2, color: 'text-green-400' });
        }
      }
    });
  } else if (viewId === 'geographic') {
    // Geographic hierarchy
    const geoTypes = ['Continent', 'GeoRegion', 'GeoSubRegion'];
    geoTypes.forEach((type, idx) => {
      const count = nodesByType[type] || 0;
      if (count > 0) {
        const isLast = idx === geoTypes.length - 1;
        const sym = isLast ? SYM.lastBranch : SYM.branch;
        lines.push({ text: `${sym}${type} (${count})`, indent: idx + 1 });
      }
    });
  } else if (viewId === 'project') {
    // Project → Pages, Entities
    const pageCount = nodesByType['Page'] || 0;
    const entityCount = nodesByType['Entity'] || 0;

    if (pageCount > 0) {
      lines.push({ text: `${SYM.branch}Pages (${pageCount})`, indent: 1, color: 'text-blue-400' });
    }
    if (entityCount > 0) {
      const sym = pageCount > 0 ? SYM.lastBranch : SYM.branch;
      lines.push({ text: `${sym}Entities (${entityCount})`, indent: 1, color: 'text-amber-400' });
    }
  } else {
    // Generic tree for other views
    nodeEntries.slice(0, 4).forEach(([type, count], idx) => {
      const isLast = idx === Math.min(nodeEntries.length, 4) - 1;
      const sym = isLast ? SYM.lastBranch : SYM.branch;
      lines.push({ text: `${sym}${type} (${count})`, indent: 1 });
    });
  }

  return lines;
}

// =============================================================================
// FLOW STYLE RENDERER
// =============================================================================

function renderFlowStyle(
  nodeKey: string,
  stats: ViewStats,
  viewId: ViewId
): AsciiLine[] {
  const lines: AsciiLine[] = [];
  const { nodesByType, arcsByType } = stats;

  // Root node
  lines.push({ text: nodeKey, indent: 0 });

  const arcEntries = Object.entries(arcsByType);
  const totalArcs = arcEntries.length;

  if (totalArcs === 0) {
    lines.push({ text: `${SYM.arrow} (no connections)`, indent: 1, color: 'text-white/30' });
    return lines;
  }

  if (viewId === 'entities') {
    // Entity connections
    const entityCount = nodesByType['Entity'] || 0;
    const seoCount = nodesByType['SEOKeyword'] || 0;
    const catCount = nodesByType['EntityCategory'] || 0;

    if (entityCount > 0) {
      lines.push({ text: `${SYM.splitStart} Entity (${entityCount})`, indent: 1, color: 'text-amber-400' });
    }
    if (seoCount > 0) {
      const sym = catCount > 0 ? SYM.splitMid : SYM.splitEnd;
      lines.push({ text: `${sym} SEOKey (${seoCount})`, indent: 1, color: 'text-violet-400' });
    }
    if (catCount > 0) {
      lines.push({ text: `${SYM.splitEnd} Category (${catCount})`, indent: 1, color: 'text-slate-400' });
    }
  } else if (viewId === 'seo-intel') {
    // SEO connections
    const keywordCount = nodesByType['SEOKeyword'] || 0;
    const clusterCount = nodesByType['SEOKeywordSet'] || 0;
    const metricsCount = nodesByType['SEOKeywordMetrics'] || 0;

    if (keywordCount > 0) {
      lines.push({ text: `${SYM.splitStart} Keyword (${keywordCount})`, indent: 1, color: 'text-violet-400' });
      if (clusterCount > 0) {
        lines.push({ text: `${SYM.vertical}  └─► Cluster (${clusterCount})`, indent: 1, color: 'text-violet-300' });
      }
    }
    if (metricsCount > 0) {
      lines.push({ text: `${SYM.splitEnd} Metrics (${metricsCount})`, indent: 1, color: 'text-violet-200' });
    }
  } else if (viewId === 'geo-intel') {
    // GEO connections
    const queryCount = nodesByType['GEOQuery'] || 0;
    const answerCount = nodesByType['GEOAnswer'] || 0;

    if (queryCount > 0) {
      lines.push({ text: `${SYM.splitStart} GEOQuery (${queryCount})`, indent: 1, color: 'text-purple-400' });
      if (answerCount > 0) {
        lines.push({ text: `${SYM.splitEnd}   └─► Answer (${answerCount})`, indent: 1, color: 'text-purple-300' });
      }
    }
  } else if (viewId === 'generation') {
    // Generation pipeline
    const generatedCount = (nodesByType['PageNative'] || 0) + (nodesByType['BlockNative'] || 0);
    if (generatedCount > 0) {
      lines.push({ text: `${SYM.doubleArrow} Generated (${generatedCount})`, indent: 1, color: 'text-pink-400' });
    }
  } else {
    // Generic flow
    arcEntries.slice(0, 3).forEach(([type, count], idx) => {
      const isFirst = idx === 0;
      const isLast = idx === Math.min(arcEntries.length, 3) - 1;
      const sym = isFirst ? SYM.splitStart : isLast ? SYM.splitEnd : SYM.splitMid;
      lines.push({ text: `${sym} ${type} (${count})`, indent: 1 });
    });
  }

  return lines;
}

// =============================================================================
// COMPACT STYLE RENDERER
// =============================================================================

function renderCompactStyle(
  nodeKey: string,
  stats: ViewStats,
  viewId: ViewId
): AsciiLine[] {
  const lines: AsciiLine[] = [];
  const { completion, nodesByType } = stats;

  // Root node
  lines.push({ text: nodeKey, indent: 0 });

  if (viewId === 'locales' || viewId === 'content') {
    // Locale coverage with progress bars
    const locales = [
      { code: 'fr', pct: 100 },
      { code: 'en', pct: 80 },
      { code: 'de', pct: 60 },
      { code: 'ja', pct: 40 },
    ];

    locales.forEach((loc, idx) => {
      const barWidth = 10;
      const filled = Math.round((loc.pct / 100) * barWidth);
      const bar = SYM.full.repeat(filled) + SYM.empty.repeat(barWidth - filled);
      const status = loc.pct === 100 ? SYM.check : SYM.pending;
      const sym = idx === locales.length - 1 ? SYM.lastBranch : SYM.branch;
      const color = loc.pct === 100 ? 'text-green-400' : loc.pct >= 60 ? 'text-amber-400' : 'text-red-400';
      lines.push({
        text: `${sym}${loc.code}: ${bar} ${loc.pct}%`,
        indent: 1,
        color,
      });
    });

    // Summary
    if (completion !== undefined) {
      lines.push({ text: `${completion}% complete`, indent: 0, color: 'text-white/50' });
    }
  } else if (viewId === 'metrics') {
    // Metrics with chart-like display
    const metrics = [
      { label: 'Volume', value: 1200, max: 2000 },
      { label: 'Difficulty', value: 45, max: 100 },
      { label: 'Potential', value: 78, max: 100 },
    ];

    metrics.forEach((m, idx) => {
      const barWidth = 8;
      const filled = Math.round((m.value / m.max) * barWidth);
      const bar = SYM.full.repeat(filled) + SYM.empty.repeat(barWidth - filled);
      const sym = idx === metrics.length - 1 ? SYM.lastBranch : SYM.branch;
      lines.push({
        text: `${sym}${m.label}: ${bar}`,
        indent: 1,
      });
    });
  } else {
    // Generic compact
    const total = Object.values(nodesByType).reduce((a, b) => a + b, 0);
    lines.push({ text: `${total} nodes connected`, indent: 1 });
  }

  return lines;
}

// =============================================================================
// MAIN COMPONENT
// =============================================================================

export const AsciiPreview = memo(function AsciiPreview({
  viewId,
  style,
  stats,
  nodeKey,
  className,
}: AsciiPreviewProps) {
  const lines = useMemo(() => {
    // Truncate nodeKey for display
    const displayKey = nodeKey.length > 16 ? nodeKey.slice(0, 14) + '…' : nodeKey;

    switch (style) {
      case 'tree':
        return renderTreeStyle(displayKey, stats, viewId);
      case 'flow':
        return renderFlowStyle(displayKey, stats, viewId);
      case 'compact':
        return renderCompactStyle(displayKey, stats, viewId);
      default:
        return [];
    }
  }, [viewId, style, stats, nodeKey]);

  return (
    <div className={cn('font-mono text-[10px] leading-tight', className)}>
      {lines.map((line, idx) => (
        <div
          key={idx}
          className={cn('whitespace-pre', line.color || 'text-white/70')}
          style={{ paddingLeft: `${line.indent * 8}px` }}
        >
          {line.text}
        </div>
      ))}
    </div>
  );
});

export default AsciiPreview;
