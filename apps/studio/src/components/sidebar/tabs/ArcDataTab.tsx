'use client';

/**
 * ArcDataTab - Arc properties and raw JSON view
 *
 * Features:
 * - Properties table with type badges
 * - Toggle between table and raw JSON
 * - Copy functionality
 *
 * v11.7 — Enhanced arc experience
 */

import { memo, useMemo, useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { TableIcon, Braces, ArrowDownLeft, ArrowUpRight } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { PropertyRow, JsonView } from '@/components/ui/detail-panel';
import { gapTokens } from '@/design/tokens';
import type { GraphEdge } from '@/types';

interface ArcDataTabProps {
  arc: GraphEdge;
}

type ViewMode = 'properties' | 'json';

/**
 * View mode toggle button
 */
function ViewModeButton({
  label,
  icon: Icon,
  isActive,
  onClick,
}: {
  label: string;
  icon: React.ComponentType<{ className?: string }>;
  isActive: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        'flex items-center px-3 py-1.5 rounded-lg text-xs font-medium transition-all',
        gapTokens.tight,
        isActive
          ? 'bg-white/10 text-white'
          : 'text-white/50 hover:text-white/70 hover:bg-white/5'
      )}
    >
      <Icon className="w-3.5 h-3.5" />
      {label}
    </button>
  );
}

/**
 * Stats bar showing direction
 */
function StatsBar({
  sourceId,
  targetId,
  propertiesCount,
}: {
  sourceId: string;
  targetId: string;
  propertiesCount: number;
}) {
  return (
    <div className="grid grid-cols-3 gap-2 p-4 border-b border-white/[0.06]">
      <div className="flex flex-col items-center p-3 rounded-lg bg-blue-500/10">
        <div className="flex items-center gap-1 text-white/50 mb-1">
          <ArrowUpRight className="w-3.5 h-3.5" />
          <span className="text-[10px] uppercase tracking-wider">From</span>
        </div>
        <span className="text-xs font-mono text-white/70 truncate max-w-full">
          {sourceId.slice(0, 8)}...
        </span>
      </div>
      <div className="flex flex-col items-center p-3 rounded-lg bg-orange-500/10">
        <div className="flex items-center gap-1 text-white/50 mb-1">
          <ArrowDownLeft className="w-3.5 h-3.5" />
          <span className="text-[10px] uppercase tracking-wider">To</span>
        </div>
        <span className="text-xs font-mono text-white/70 truncate max-w-full">
          {targetId.slice(0, 8)}...
        </span>
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
    <span
      className={cn(
        'px-1.5 py-0.5 rounded text-[10px] font-mono uppercase',
        colors[type] || 'text-gray-400 bg-gray-500/10'
      )}
    >
      {type}
    </span>
  );
}

export const ArcDataTab = memo(function ArcDataTab({ arc }: ArcDataTabProps) {
  const [viewMode, setViewMode] = useState<ViewMode>('properties');
  const { copied, copy } = useCopyFeedback();

  // Build core properties
  const coreProps = useMemo(() => {
    const props = [
      { key: 'id', value: arc.id, type: 'string' },
      { key: 'type', value: arc.type, type: 'string' },
      { key: 'source', value: arc.source, type: 'string' },
      { key: 'target', value: arc.target, type: 'string' },
    ];
    return props;
  }, [arc]);

  // Extract data properties
  const dataProps = useMemo(() => {
    if (!arc.data) return [];
    return Object.entries(arc.data).map(([key, value]) => ({
      key,
      value,
      type: Array.isArray(value) ? 'array' : typeof value,
    }));
  }, [arc.data]);

  const totalProps = coreProps.length + dataProps.length;

  // Full arc object for JSON view
  const arcObject = useMemo(
    () => ({
      id: arc.id,
      type: arc.type,
      source: arc.source,
      target: arc.target,
      data: arc.data || {},
    }),
    [arc]
  );

  return (
    <div className="flex flex-col h-full">
      {/* Stats bar */}
      <StatsBar
        sourceId={arc.source}
        targetId={arc.target}
        propertiesCount={totalProps}
      />

      {/* View mode toggle */}
      <div className={cn('flex items-center p-2 border-b border-white/[0.06]', gapTokens.tight)}>
        <ViewModeButton
          label="Properties"
          icon={TableIcon}
          isActive={viewMode === 'properties'}
          onClick={() => setViewMode('properties')}
        />
        <ViewModeButton
          label="JSON"
          icon={Braces}
          isActive={viewMode === 'json'}
          onClick={() => setViewMode('json')}
        />
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        <AnimatePresence mode="wait">
          <motion.div
            key={viewMode}
            initial={{ opacity: 0, y: 4 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -4 }}
            transition={{ duration: 0.1 }}
            className="p-4"
          >
            {viewMode === 'properties' ? (
              <div className="space-y-4">
                {/* Core properties */}
                <div>
                  <h4 className="text-xs font-medium text-white/40 mb-2">Core Properties</h4>
                  <div className="rounded-lg border border-white/[0.06] overflow-hidden">
                    {coreProps.map(({ key, value, type }) => (
                      <div
                        key={key}
                        className="flex items-center gap-2 px-3 py-2 hover:bg-white/[0.02] border-b border-white/[0.04] last:border-b-0"
                      >
                        <div className="flex-1 min-w-0">
                          <div className={cn('flex items-center', gapTokens.tight)}>
                            <span className="text-xs font-mono text-white/40">{key}</span>
                            <TypeBadge type={type} />
                          </div>
                          <div className="text-sm text-white/80 truncate mt-0.5 font-mono">
                            {String(value)}
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Data properties */}
                {dataProps.length > 0 && (
                  <div>
                    <h4 className="text-xs font-medium text-white/40 mb-2">
                      Data Properties ({dataProps.length})
                    </h4>
                    <div className="rounded-lg border border-white/[0.06] overflow-hidden">
                      {dataProps.map(({ key, value }) => (
                        <PropertyRow
                          key={key}
                          label={key}
                          value={value}
                          onCopy={() => copy(JSON.stringify(value))}
                          isCopied={copied}
                        />
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ) : (
              <JsonView
                data={arcObject}
                onCopy={() => copy(JSON.stringify(arcObject, null, 2))}
                isCopied={copied}
              />
            )}
          </motion.div>
        </AnimatePresence>
      </div>
    </div>
  );
});

export default ArcDataTab;
