'use client';

/**
 * Query Result Views - Table and Raw JSON displays
 *
 * Linear-inspired design with:
 * - Premium glass morphism surfaces
 * - Vibrant accent colors
 * - Clear visual hierarchy
 */

import { memo, useCallback, useMemo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { Table2, Code, Copy, Check, AlertCircle, FileJson, Clock } from 'lucide-react';
import { GRAPH_ICONS, ICON_COLORS } from '@/config/iconSystem';
import { cn } from '@/lib/utils';
import { useQueryStore } from '@/stores/queryStore';
import { useCopyFeedback } from '@/hooks';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import { EmptyState, LoadingState } from '@/components/ui/EmptyState';
import { ACCENT_COLORS } from '@/config/constants';
import { iconSizes, gapTokens, paddingTokens, badgeClasses } from '@/design/tokens';

/**
 * TableView - Linear-style data table with rich formatting
 */
export const TableView = memo(function TableView() {
  const { result, isExecuting, error } = useQueryStore(
    useShallow((state) => ({
      result: state.result,
      isExecuting: state.isExecuting,
      error: state.error,
    }))
  );

  // Get all unique property keys from nodes (memoized for large datasets)
  // Must be before early returns to satisfy rules-of-hooks
  const columns = useMemo(() => {
    if (!result?.nodes?.length) return [];
    const allKeys = new Set<string>();
    result.nodes.forEach((node) => {
      allKeys.add('type');
      allKeys.add('displayName');
      allKeys.add('key');
      allKeys.add('id');
      if (node.data) {
        Object.keys(node.data).forEach((k) => allKeys.add(k));
      }
    });
    return Array.from(allKeys);
  }, [result?.nodes]);

  // Loading state
  if (isExecuting) {
    return (
      <div className="h-full" style={{ backgroundColor: ACCENT_COLORS.base }}>
        <LoadingState
          title="Loading table data"
          description="Fetching results from Neo4j"
          accentColor="accent-blue"
        />
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <div className="h-full" style={{ backgroundColor: ACCENT_COLORS.base }}>
        <EmptyState
          icon={AlertCircle}
          title="Query Error"
          description={error}
          variant="error"
        />
      </div>
    );
  }

  // Empty state
  if (!result?.nodes?.length) {
    return (
      <div className="h-full" style={{ backgroundColor: ACCENT_COLORS.base }}>
        <EmptyState
          icon={Table2}
          title="No data to display"
          description="Execute a query to see results in table format"
        />
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[hsl(240,8%,4%)]">
      {/* Table header bar */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-white/10 bg-[hsl(240,8%,6%)]">
        <div className={cn('flex items-center', gapTokens.spacious)}>
          <div className="w-8 h-8 rounded-lg bg-accent-blue/15 flex items-center justify-center border border-accent-blue/30">
            <Table2 className={cn(iconSizes.md, 'text-accent-blue')} />
          </div>
          <div>
            <span className="text-sm font-medium text-white/90">Table View</span>
            <span className="text-xs text-white/40 ml-3">
              {result.totalNodes.toLocaleString()} nodes
            </span>
          </div>
        </div>
        {result.duration > 0 && (
          <div className={cn('flex items-center rounded-lg bg-white/5 text-xs text-white/50', gapTokens.compact, paddingTokens.compact)}>
            <Clock className={iconSizes.xs} />
            {result.duration}ms
          </div>
        )}
      </div>

      {/* Table content */}
      <div className="overflow-auto flex-1">
        <table className="w-full text-xs">
          <thead className="sticky top-0 z-10">
            <tr className="bg-[hsl(240,8%,8%)]">
              {columns.map((col, idx) => (
                <th
                  key={col}
                  className={cn(
                    'px-4 py-3 text-left font-semibold text-white/80 border-b border-white/12',
                    'uppercase text-[10px] tracking-wider',
                    idx === 0 && 'pl-5'
                  )}
                >
                  {col}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {result.nodes.map((node, rowIdx) => {
              const config = NODE_TYPE_CONFIG[node.type] || NODE_TYPE_CONFIG.Project;

              return (
                <tr
                  key={node.id}
                  className={cn(
                    'transition-colors group cursor-pointer',
                    'hover:bg-accent-blue/8',
                    rowIdx % 2 === 0 ? 'bg-[hsl(240,8%,5%)]' : 'bg-[hsl(240,8%,4%)]'
                  )}
                >
                  {columns.map((col, colIdx) => {
                    let value: unknown;
                    if (col === 'id') value = node.id;
                    else if (col === 'type') value = node.type;
                    else if (col === 'key') value = node.key;
                    else if (col === 'displayName') value = node.displayName;
                    else value = node.data?.[col];

                    const displayValue = typeof value === 'object' ? JSON.stringify(value) : String(value ?? '');
                    const isType = col === 'type';
                    const isDisplayName = col === 'displayName';
                    const isKey = col === 'key';

                    return (
                      <td
                        key={col}
                        className={cn(
                          'px-4 py-3 border-b border-white/6 max-w-[300px] truncate',
                          'transition-colors',
                          colIdx === 0 && 'pl-5',
                          isDisplayName ? 'text-white/90 font-medium' : 'text-white/60 group-hover:text-white/80'
                        )}
                        title={typeof value === 'string' ? value : JSON.stringify(value)}
                      >
                        {isType ? (
                          <span
                            className={cn('inline-flex items-center px-2 py-1 rounded-md text-[10px] font-semibold border', gapTokens.compact)}
                            style={{
                              backgroundColor: `${config.color}20`,
                              borderColor: `${config.color}40`,
                              color: config.color,
                            }}
                          >
                            <CategoryIcon
                              category={config.category}
                              size={12}
                              strokeWidth={2}
                              style={{ color: config.color }}
                            />
                            {displayValue}
                          </span>
                        ) : isKey ? (
                          <span className="font-mono text-accent-blue/80">{displayValue}</span>
                        ) : (
                          displayValue
                        )}
                      </td>
                    );
                  })}
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between px-5 py-3 bg-[hsl(240,8%,6%)] border-t border-white/10">
        <div className={cn('flex items-center text-xs text-white/50', gapTokens.default)}>
          <GRAPH_ICONS.node className={cn(iconSizes.sm, ICON_COLORS.node.muted)} />
          <span>Showing <span className="text-white/80 font-medium">{result.nodes.length}</span> of {result.totalNodes} nodes</span>
        </div>
        <div className="text-xs text-white/40">
          {result.totalEdges} relationships
        </div>
      </div>
    </div>
  );
});

/**
 * JsonSyntaxHighlight - Renders JSON with Linear-style syntax colors
 */
const JsonSyntaxHighlight = memo(function JsonSyntaxHighlight({ data }: { data: unknown }) {
  const renderValue = useCallback((value: unknown, depth: number): React.ReactNode => {
    if (value === null) {
      return <span className="text-white/40">null</span>;
    }
    if (typeof value === 'boolean') {
      return <span className="text-accent-blue">{String(value)}</span>;
    }
    if (typeof value === 'number') {
      return <span className="text-accent-orange">{value}</span>;
    }
    if (typeof value === 'string') {
      return <span className="text-accent-green">&quot;{value}&quot;</span>;
    }
    if (Array.isArray(value)) {
      if (value.length === 0) return <span className="text-white/60">[]</span>;
      const indent = '  '.repeat(depth);
      const innerIndent = '  '.repeat(depth + 1);
      return (
        <>
          <span className="text-white/60">[</span>
          {'\n'}
          {value.map((item, idx) => (
            <span key={idx}>
              {innerIndent}
              {renderValue(item, depth + 1)}
              {idx < value.length - 1 ? ',' : ''}
              {'\n'}
            </span>
          ))}
          {indent}<span className="text-white/60">]</span>
        </>
      );
    }
    if (typeof value === 'object') {
      const entries = Object.entries(value);
      if (entries.length === 0) return <span className="text-white/60">{'{}'}</span>;
      const indent = '  '.repeat(depth);
      const innerIndent = '  '.repeat(depth + 1);
      return (
        <>
          <span className="text-white/60">{'{'}</span>
          {'\n'}
          {entries.map(([key, val], idx) => (
            <span key={key}>
              {innerIndent}
              <span className="text-accent-purple">&quot;{key}&quot;</span>
              <span className="text-white/60">: </span>
              {renderValue(val, depth + 1)}
              {idx < entries.length - 1 ? ',' : ''}
              {'\n'}
            </span>
          ))}
          {indent}<span className="text-white/60">{'}'}</span>
        </>
      );
    }
    return <span className="text-white/60">{String(value)}</span>;
  }, []);

  return (
    <pre className="text-xs font-mono leading-relaxed">
      {renderValue(data, 0)}
    </pre>
  );
});

/**
 * RawView - Linear-style JSON viewer with syntax highlighting
 */
export const RawView = memo(function RawView() {
  const { result, isExecuting, error } = useQueryStore(
    useShallow((state) => ({
      result: state.result,
      isExecuting: state.isExecuting,
      error: state.error,
    }))
  );
  const { copied, copy } = useCopyFeedback();

  const handleCopy = useCallback(async () => {
    if (!result) return;
    await copy(JSON.stringify(result, null, 2));
  }, [result, copy]);

  // Loading state
  if (isExecuting) {
    return (
      <div className="h-full" style={{ backgroundColor: ACCENT_COLORS.base }}>
        <LoadingState
          title="Loading raw data"
          description="Fetching JSON response from Neo4j"
          accentColor="accent-purple"
        />
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <div className="h-full" style={{ backgroundColor: ACCENT_COLORS.base }}>
        <EmptyState
          icon={AlertCircle}
          title="Query Error"
          description={error}
          variant="error"
        />
      </div>
    );
  }

  // Empty state
  if (!result) {
    return (
      <div className="h-full" style={{ backgroundColor: ACCENT_COLORS.base }}>
        <EmptyState
          icon={Code}
          title="No data to display"
          description="Execute a query to see the raw JSON response"
        />
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[hsl(240,8%,4%)]">
      {/* Header bar */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-white/10 bg-[hsl(240,8%,6%)]">
        <div className={cn('flex items-center', gapTokens.spacious)}>
          <div className="w-8 h-8 rounded-lg bg-accent-purple/15 flex items-center justify-center border border-accent-purple/30">
            <FileJson className={cn(iconSizes.md, 'text-accent-purple')} />
          </div>
          <div>
            <span className="text-sm font-medium text-white/90">Raw JSON</span>
            <span className="text-xs text-white/40 ml-3">
              {result.totalNodes} nodes, {result.totalEdges} relationships
            </span>
          </div>
        </div>
        <div className={cn('flex items-center', gapTokens.default)}>
          {result.duration > 0 && (
            <div className={cn('flex items-center rounded-lg bg-white/5 text-xs text-white/50', gapTokens.compact, paddingTokens.compact)}>
              <Clock className={iconSizes.xs} />
              {result.duration}ms
            </div>
          )}
          <button
            onClick={handleCopy}
            className={cn(
              'flex items-center rounded-lg text-xs font-medium transition-all duration-150 border',
              gapTokens.compact, paddingTokens.standard,
              copied
                ? 'text-accent-green bg-accent-green/15 border-accent-green/30'
                : 'text-white/60 hover:text-white/90 bg-white/5 hover:bg-white/10 border-white/10 hover:border-white/20'
            )}
          >
            {copied ? <Check className={iconSizes.sm} /> : <Copy className={iconSizes.sm} />}
            <span>{copied ? 'Copied!' : 'Copy JSON'}</span>
          </button>
        </div>
      </div>

      {/* JSON content with syntax highlighting */}
      <div className="overflow-auto flex-1 p-4">
        <JsonSyntaxHighlight data={result} />
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between px-5 py-3 bg-[hsl(240,8%,6%)] border-t border-white/10">
        <div className={cn('flex items-center text-xs text-white/50', gapTokens.default)}>
          <Code className={iconSizes.sm} />
          <span>JSON format</span>
        </div>
        <div className="text-xs text-white/40">
          {(JSON.stringify(result).length / 1024).toFixed(1)} KB
        </div>
      </div>
    </div>
  );
});
