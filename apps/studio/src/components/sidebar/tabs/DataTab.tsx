'use client';

/**
 * DataTab - Node properties and statistics
 *
 * Features:
 * - Stats bar (incoming/outgoing arcs, properties count)
 * - Properties table with type badges
 * - Property coverage progress bar
 */

import { memo, useMemo, useState } from 'react';
import { ArrowDownLeft, ArrowUpRight, Braces } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useCopyFieldFeedback } from '@/hooks';
import { PropertyRow, formatValue } from '@/components/ui/detail-panel';
import { CollapsibleSection } from '@/components/ui/detail-panel';
import { gapTokens } from '@/design/tokens';
import type { GraphNode } from '@/types';
import type { Edge } from '@xyflow/react';

interface DataTabProps {
  node: GraphNode;
  colors: { primary: string; secondary: string };
  relatedEdges: Edge[];
}

/**
 * Stats bar showing arc counts and properties
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
  return (
    <div className="grid grid-cols-3 gap-2 p-4 border-b border-white/[0.06]">
      <div
        className="flex flex-col items-center p-3 rounded-lg"
        style={{ background: `${colors.primary}10` }}
      >
        <div className="flex items-center gap-1 text-white/50 mb-1">
          <ArrowDownLeft className="w-3.5 h-3.5" />
          <span className="text-[10px] uppercase tracking-wider">In</span>
        </div>
        <span className="text-lg font-semibold text-white">{incomingCount}</span>
      </div>
      <div
        className="flex flex-col items-center p-3 rounded-lg"
        style={{ background: `${colors.secondary}10` }}
      >
        <div className="flex items-center gap-1 text-white/50 mb-1">
          <ArrowUpRight className="w-3.5 h-3.5" />
          <span className="text-[10px] uppercase tracking-wider">Out</span>
        </div>
        <span className="text-lg font-semibold text-white">{outgoingCount}</span>
      </div>
      <div className="flex flex-col items-center p-3 rounded-lg bg-white/[0.03]">
        <div className="flex items-center gap-1 text-white/50 mb-1">
          <Braces className="w-3.5 h-3.5" />
          <span className="text-[10px] uppercase tracking-wider">Props</span>
        </div>
        <span className="text-lg font-semibold text-white">{propertiesCount}</span>
      </div>
    </div>
  );
}

/**
 * Property type badge
 */
function TypeBadge({ type }: { type: string }) {
  const colors: Record<string, string> = {
    string: 'text-emerald-400 bg-emerald-500/10',
    number: 'text-blue-400 bg-blue-500/10',
    boolean: 'text-amber-400 bg-amber-500/10',
    object: 'text-purple-400 bg-purple-500/10',
    array: 'text-pink-400 bg-pink-500/10',
    null: 'text-gray-400 bg-gray-500/10',
    undefined: 'text-gray-400 bg-gray-500/10',
  };

  return (
    <span className={cn(
      'px-1.5 py-0.5 rounded text-[10px] font-mono uppercase',
      colors[type] || 'text-gray-400 bg-gray-500/10'
    )}>
      {type}
    </span>
  );
}

/**
 * Property coverage progress bar
 */
function PropertyCoverage({
  filled,
  total,
}: {
  filled: number;
  total: number;
}) {
  const percentage = total > 0 ? (filled / total) * 100 : 0;

  return (
    <div className="p-4 border-t border-white/[0.06]">
      <div className="flex items-center justify-between text-xs mb-2">
        <span className="text-white/40">Property Coverage</span>
        <span className="text-white/60 font-mono">
          {filled}/{total} ({percentage.toFixed(0)}%)
        </span>
      </div>
      <div className="h-1.5 bg-white/[0.06] rounded-full overflow-hidden">
        <div
          className="h-full bg-emerald-500 rounded-full transition-all duration-300"
          style={{ width: `${percentage}%` }}
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

  // Build properties list from node
  const coreProps = useMemo(() => {
    const props = [
      { key: 'id', value: node.id, type: 'string' },
      { key: 'type', value: node.type, type: 'string' },
      { key: 'key', value: node.key, type: 'string' },
      { key: 'displayName', value: node.displayName, type: 'string' },
    ];
    if (node.description) props.push({ key: 'description', value: node.description, type: 'string' });
    if (node.llmContext) props.push({ key: 'llmContext', value: node.llmContext, type: 'string' });
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
      <PropertyCoverage filled={filledProps} total={totalProps} />
    </div>
  );
});

export default DataTab;
