'use client';

/**
 * DataTab - Premium node properties and statistics
 *
 * Features:
 * - Premium stats bar with animated counters
 * - Property table with type badges and glow effects
 * - Property coverage progress bar with gradient
 * - Collapsible sections with smooth animations
 *
 * Design System:
 * - Uses glass.surface hierarchy
 * - Color-coded stat cards
 * - Premium progress bar with gradient
 */

import { memo, useMemo, useState } from 'react';
import { motion } from 'motion/react';
import { ArrowDownLeft, ArrowUpRight, Braces, LayoutGrid } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useCopyFieldFeedback } from '@/hooks';
import { PropertyRow, formatValue } from '@/components/ui/detail-panel';
import { CollapsibleSection } from '@/components/ui/detail-panel';
import { gapTokens, glass } from '@/design/tokens';
import type { GraphNode } from '@/types';
import type { Edge } from '@xyflow/react';

interface DataTabProps {
  node: GraphNode;
  colors: { primary: string; secondary: string };
  relatedEdges: Edge[];
}

/**
 * Premium stats bar with animated counters and glass cards
 */
function StatsBar({
  incomingCount,
  outgoingCount,
  propertiesCount,
  colors,
}: {
  incomingCount: number;
  outgoingCount: number;
  propertiesCount: number;
  colors: { primary: string; secondary: string };
}) {
  const stats = [
    {
      label: 'Incoming',
      value: incomingCount,
      icon: ArrowDownLeft,
      color: colors.primary,
    },
    {
      label: 'Outgoing',
      value: outgoingCount,
      icon: ArrowUpRight,
      color: colors.secondary,
    },
    {
      label: 'Properties',
      value: propertiesCount,
      icon: Braces,
      color: '#a3a3a3',
    },
  ];

  return (
    <div
      className="grid grid-cols-3 gap-2 p-3"
      style={{
        background: `linear-gradient(180deg, ${glass.surface[1]}, ${glass.surface[0]})`,
        borderBottom: `1px solid ${glass.border.subtle}`,
      }}
    >
      {stats.map((stat, i) => {
        const Icon = stat.icon;
        return (
          <motion.div
            key={stat.label}
            initial={{ opacity: 0, y: 8 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: i * 0.05 }}
            className="relative flex flex-col items-center p-3 rounded-xl overflow-hidden"
            style={{
              background: `linear-gradient(135deg, ${stat.color}15, ${stat.color}05)`,
              border: `1px solid ${stat.color}20`,
            }}
          >
            {/* Subtle glow effect */}
            <div
              className="absolute inset-0 opacity-0 hover:opacity-100 transition-opacity duration-300"
              style={{
                background: `radial-gradient(circle at center, ${stat.color}10, transparent 70%)`,
              }}
            />

            {/* Icon and label */}
            <div className="flex items-center gap-1.5 mb-1.5 relative z-10">
              <Icon
                className="w-3.5 h-3.5"
                style={{ color: `${stat.color}99` }}
              />
              <span
                className="text-[10px] font-medium uppercase tracking-wider"
                style={{ color: `${stat.color}99` }}
              >
                {stat.label.slice(0, 2)}
              </span>
            </div>

            {/* Animated value */}
            <motion.span
              className="text-xl font-bold text-white relative z-10"
              initial={{ scale: 0.8 }}
              animate={{ scale: 1 }}
              transition={{ type: 'spring', stiffness: 300, delay: i * 0.05 + 0.1 }}
            >
              {stat.value}
            </motion.span>
          </motion.div>
        );
      })}
    </div>
  );
}

/**
 * Premium property type badge with glow
 */
function TypeBadge({ type }: { type: string }) {
  const config: Record<string, { color: string; bgColor: string }> = {
    string: { color: '#10b981', bgColor: 'rgba(16, 185, 129, 0.15)' },
    number: { color: '#3b82f6', bgColor: 'rgba(59, 130, 246, 0.15)' },
    boolean: { color: '#f59e0b', bgColor: 'rgba(245, 158, 11, 0.15)' },
    object: { color: '#a855f7', bgColor: 'rgba(168, 85, 247, 0.15)' },
    array: { color: '#ec4899', bgColor: 'rgba(236, 72, 153, 0.15)' },
    null: { color: '#6b7280', bgColor: 'rgba(107, 114, 128, 0.15)' },
    undefined: { color: '#6b7280', bgColor: 'rgba(107, 114, 128, 0.15)' },
  };

  const { color, bgColor } = config[type] || config.undefined;

  return (
    <span
      className="px-1.5 py-0.5 rounded text-[9px] font-mono uppercase tracking-wide"
      style={{
        color,
        background: bgColor,
        boxShadow: `0 0 8px ${color}20`,
      }}
    >
      {type}
    </span>
  );
}

/**
 * Premium property coverage progress bar with gradient
 */
function PropertyCoverage({
  filled,
  total,
  colors,
}: {
  filled: number;
  total: number;
  colors: { primary: string; secondary: string };
}) {
  const percentage = total > 0 ? (filled / total) * 100 : 0;

  return (
    <div
      className="p-4"
      style={{
        background: `linear-gradient(180deg, ${glass.surface[1]}, ${glass.surface[0]})`,
        borderTop: `1px solid ${glass.border.subtle}`,
      }}
    >
      <div className="flex items-center justify-between text-xs mb-2.5">
        <div className="flex items-center gap-2">
          <LayoutGrid className="w-3.5 h-3.5 text-white/40" />
          <span className="text-white/50 font-medium">Coverage</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-white/70 font-mono tabular-nums">
            {filled}/{total}
          </span>
          <span
            className="px-1.5 py-0.5 rounded text-[10px] font-bold"
            style={{
              background: percentage >= 80
                ? 'rgba(16, 185, 129, 0.2)'
                : percentage >= 50
                  ? 'rgba(245, 158, 11, 0.2)'
                  : 'rgba(239, 68, 68, 0.2)',
              color: percentage >= 80
                ? '#10b981'
                : percentage >= 50
                  ? '#f59e0b'
                  : '#ef4444',
            }}
          >
            {percentage.toFixed(0)}%
          </span>
        </div>
      </div>

      {/* Progress bar with gradient */}
      <div
        className="h-2 rounded-full overflow-hidden"
        style={{ background: glass.surface[2] }}
      >
        <motion.div
          className="h-full rounded-full"
          initial={{ width: 0 }}
          animate={{ width: `${percentage}%` }}
          transition={{ duration: 0.5, ease: 'easeOut' }}
          style={{
            background: `linear-gradient(90deg, ${colors.primary}, ${colors.secondary})`,
            boxShadow: `0 0 12px ${colors.primary}40`,
          }}
        />
      </div>
    </div>
  );
}

export const DataTab = memo(function DataTab({
  node,
  colors,
  relatedEdges,
}: DataTabProps) {
  const { copiedField, copyField } = useCopyFieldFeedback();
  const [expandedSections, setExpandedSections] = useState<Set<string>>(
    new Set(['core', 'data'])
  );

  const toggleSection = (section: string) => {
    setExpandedSections((prev) => {
      const next = new Set(prev);
      if (next.has(section)) {
        next.delete(section);
      } else {
        next.add(section);
      }
      return next;
    });
  };

  // Count incoming/outgoing arcs
  const { incomingCount, outgoingCount } = useMemo(() => {
    let incoming = 0;
    let outgoing = 0;
    for (const edge of relatedEdges) {
      if (edge.target === node.id) incoming++;
      if (edge.source === node.id) outgoing++;
    }
    return { incomingCount: incoming, outgoingCount: outgoing };
  }, [node.id, relatedEdges]);

  // Build properties list from node (v0.20.0: content, triggers, nodeClass, provenance)
  const coreProps = useMemo(() => {
    const props: { key: string; value: unknown; type: string }[] = [
      { key: 'id', value: node.id, type: 'string' },
      { key: 'type', value: node.type, type: 'string' },
      { key: 'key', value: node.key, type: 'string' },
      { key: 'displayName', value: node.displayName, type: 'string' },
    ];
    if (node.nodeClass) props.push({ key: 'nodeClass', value: node.nodeClass, type: 'string' });
    if (node.content) props.push({ key: 'content', value: node.content, type: 'string' });
    if (node.triggers && node.triggers.length > 0) props.push({ key: 'triggers', value: node.triggers.join(', '), type: 'array' });
    if (node.provenance) props.push({ key: 'provenance', value: node.provenance, type: 'string' });
    return props;
  }, [node]);

  // Extract data properties
  const dataProps = useMemo(() => {
    if (!node.data) return [];
    return Object.entries(node.data).map(([key, value]) => ({
      key,
      value,
      type: Array.isArray(value) ? 'array' : typeof value,
    }));
  }, [node.data]);

  // Calculate coverage
  const totalProps = coreProps.length + dataProps.length;
  const filledProps = coreProps.filter(p => p.value !== null && p.value !== undefined).length +
    dataProps.filter(p => p.value !== null && p.value !== undefined).length;

  return (
    <div className="flex flex-col h-full">
      {/* Stats bar */}
      <StatsBar
        incomingCount={incomingCount}
        outgoingCount={outgoingCount}
        propertiesCount={totalProps}
        colors={colors}
      />

      {/* Properties sections */}
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        {/* Core properties */}
        <CollapsibleSection
          title="Core Properties"
          count={coreProps.length}
          isExpanded={expandedSections.has('core')}
          onToggle={() => toggleSection('core')}
        >
          <div className="space-y-0">
            {coreProps.map(({ key, value, type }) => (
              <div key={key} className="flex items-start gap-2 px-4 py-2 hover:bg-white/[0.02]">
                <div className="flex-1 min-w-0">
                  <div className={cn('flex items-center', gapTokens.tight)}>
                    <span className="text-xs font-mono text-white/40">{key}</span>
                    <TypeBadge type={type} />
                  </div>
                  <div className="text-sm text-white/80 truncate mt-0.5">
                    {formatValue(value)}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </CollapsibleSection>

        {/* Data properties */}
        {dataProps.length > 0 && (
          <CollapsibleSection
            title="Data Properties"
            count={dataProps.length}
            isExpanded={expandedSections.has('data')}
            onToggle={() => toggleSection('data')}
          >
            <div className="space-y-0">
              {dataProps.map(({ key, value }) => (
                <PropertyRow
                  key={key}
                  label={key}
                  value={value}
                  onCopy={() => copyField(JSON.stringify(value), `data.${key}`)}
                  isCopied={copiedField === `data.${key}`}
                />
              ))}
            </div>
          </CollapsibleSection>
        )}
      </div>

      {/* Coverage bar */}
      <PropertyCoverage filled={filledProps} total={totalProps} colors={colors} />
    </div>
  );
});

export default DataTab;
