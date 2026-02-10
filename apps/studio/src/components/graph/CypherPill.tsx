'use client';

/**
 * CypherPill - Persistent query status bar
 *
 * Always visible at the top of the graph canvas, showing the
 * current Cypher query that drives the view.
 *
 * Features:
 * - Shows truncated query with full query on hover
 * - Displays stats: nodes, arcs, execution time
 * - Copy to clipboard
 * - Re-run query button
 * - Loading state with spinner
 * - Error state with retry
 *
 * States:
 * - IDLE: Query complete, showing stats
 * - LOADING: Query in progress, spinner active
 * - ERROR: Query failed, showing retry button
 */

import { memo, useState, useCallback } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Copy, Play, RefreshCw, AlertCircle, Loader2, Zap } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';

export type CypherPillState = 'idle' | 'loading' | 'error';

interface CypherPillProps {
  /** Current Cypher query */
  query: string;
  /** Current state */
  state: CypherPillState;
  /** Number of nodes returned */
  nodeCount?: number;
  /** Number of arcs/relationships returned */
  arcCount?: number;
  /** Execution time in milliseconds */
  executionTime?: number;
  /** Error message (when state is 'error') */
  errorMessage?: string;
  /** Callback when run button is clicked */
  onRun?: () => void;
  /** Callback when retry button is clicked */
  onRetry?: () => void;
  /** Additional className */
  className?: string;
}

/**
 * Truncate query for display (max 60 chars)
 */
function truncateQuery(query: string, maxLength = 60): string {
  const singleLine = query.replace(/\s+/g, ' ').trim();
  if (singleLine.length <= maxLength) return singleLine;
  return singleLine.slice(0, maxLength - 3) + '...';
}

/**
 * Format execution time
 */
function formatTime(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  return `${(ms / 1000).toFixed(1)}s`;
}

export const CypherPill = memo(function CypherPill({
  query,
  state,
  nodeCount,
  arcCount,
  executionTime,
  errorMessage,
  onRun,
  onRetry,
  className,
}: CypherPillProps) {
  const [isHovered, setIsHovered] = useState(false);
  const { copied, copy } = useCopyFeedback();

  const handleCopy = useCallback(() => {
    copy(query);
  }, [copy, query]);

  const truncatedQuery = truncateQuery(query);
  const isError = state === 'error';
  const isLoading = state === 'loading';

  return (
    <motion.div
      className={cn(
        'sticky top-0 z-[100] mx-auto max-w-4xl',
        'px-4 py-2',
        className
      )}
      initial={{ y: -20, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ type: 'spring', bounce: 0.2, duration: 0.4 }}
    >
      <div
        className={cn(
          'flex items-center gap-3 px-4 py-2 rounded-lg',
          'bg-black/80 backdrop-blur-md',
          'border transition-colors duration-200',
          isError
            ? 'border-red-500/50 bg-red-950/20'
            : isLoading
              ? 'border-emerald-500/50 animate-pulse'
              : 'border-white/10 hover:border-white/20'
        )}
        onMouseEnter={() => setIsHovered(true)}
        onMouseLeave={() => setIsHovered(false)}
      >
        {/* Icon */}
        <div className="flex-shrink-0">
          {isError ? (
            <AlertCircle className="w-4 h-4 text-red-400" />
          ) : isLoading ? (
            <Loader2 className="w-4 h-4 text-emerald-400 animate-spin" />
          ) : (
            <Zap className="w-4 h-4 text-emerald-400" />
          )}
        </div>

        {/* Query text */}
        <div className="flex-1 min-w-0">
          <code
            className={cn(
              'font-mono text-sm truncate block',
              isError ? 'text-red-300' : 'text-white/80'
            )}
            title={query}
          >
            {isError ? errorMessage || 'Query failed' : truncatedQuery}
          </code>
        </div>

        {/* Stats (only when idle) */}
        <AnimatePresence mode="wait">
          {state === 'idle' && (
            <motion.div
              className="flex items-center gap-2 text-xs text-white/50"
              initial={{ opacity: 0, x: 10 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -10 }}
            >
              {nodeCount !== undefined && (
                <span className="px-2 py-0.5 bg-white/5 rounded">
                  {nodeCount}
                </span>
              )}
              {arcCount !== undefined && (
                <span className="px-2 py-0.5 bg-white/5 rounded">
                  {arcCount}
                </span>
              )}
              {executionTime !== undefined && (
                <span className="px-2 py-0.5 bg-white/5 rounded">
                  {formatTime(executionTime)}
                </span>
              )}
            </motion.div>
          )}
        </AnimatePresence>

        {/* Actions */}
        <div className="flex items-center gap-1">
          {/* Copy button */}
          <button
            onClick={handleCopy}
            className={cn(
              'p-1.5 rounded transition-colors',
              'hover:bg-white/10',
              copied && 'text-emerald-400'
            )}
            title={copied ? 'Copied!' : 'Copy query'}
            aria-label="Copy query to clipboard"
          >
            <Copy className="w-3.5 h-3.5" />
          </button>

          {/* Run/Retry button */}
          {isError ? (
            <button
              onClick={onRetry}
              className="p-1.5 rounded hover:bg-white/10 text-red-400 hover:text-red-300"
              title="Retry query"
              aria-label="Retry query"
            >
              <RefreshCw className="w-3.5 h-3.5" />
            </button>
          ) : (
            <button
              onClick={onRun}
              disabled={isLoading}
              className={cn(
                'p-1.5 rounded transition-colors',
                'hover:bg-white/10',
                isLoading && 'opacity-50 cursor-not-allowed'
              )}
              title="Run query"
              aria-label="Run query"
            >
              <Play className="w-3.5 h-3.5" />
            </button>
          )}
        </div>
      </div>

      {/* Expanded query tooltip on hover */}
      <AnimatePresence>
        {isHovered && query.length > 60 && (
          <motion.div
            className={cn(
              'absolute left-4 right-4 mt-2 p-3 rounded-lg',
              'bg-black/95 backdrop-blur-md border border-white/10',
              'z-[101]'
            )}
            initial={{ opacity: 0, y: -5 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -5 }}
            transition={{ duration: 0.15 }}
          >
            <pre className="font-mono text-xs text-white/80 whitespace-pre-wrap break-all">
              {query}
            </pre>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
});

export default CypherPill;
