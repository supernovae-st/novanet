'use client';

/**
 * GraphEmptyState - Context-aware empty state for graph visualization
 *
 * Shows diagnostic information about why the graph is empty:
 * - Which view was executed
 * - What root type was expected
 * - Suggestions for troubleshooting
 */

import { memo, useState, useEffect } from 'react';
import {
  Database,
  AlertTriangle,
  Search,
  Filter,
  RefreshCw,
  FileQuestion,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import { useViewStore } from '@/stores/viewStore';
import { useQueryStore } from '@/stores/queryStore';
import { gapTokens } from '@/design/tokens';

// =============================================================================
// TYPES
// =============================================================================

interface GraphEmptyStateProps {
  className?: string;
}

// =============================================================================
// COMPONENT
// =============================================================================

export const GraphEmptyState = memo(function GraphEmptyState({
  className,
}: GraphEmptyStateProps) {
  // Hydration fix: only show dynamic content after mount
  const [hasMounted, setHasMounted] = useState(false);
  useEffect(() => {
    setHasMounted(true);
  }, []);

  const activeViewId = useViewStore((state) => state.activeViewId);
  const isCustomQuery = useViewStore((state) => state.isCustomQuery);
  const error = useQueryStore((state) => state.error);
  const result = useQueryStore((state) => state.result);
  const isExecuting = useQueryStore((state) => state.isExecuting);

  // Get active view info
  const getActiveView = useViewStore((state) => state.getActiveView);
  const activeView = getActiveView();

  // Determine the diagnostic message
  const getDiagnosticInfo = () => {
    // Error state
    if (error) {
      return {
        icon: AlertTriangle,
        title: 'Query Error',
        description: error,
        accentClass: 'accent-red',
        suggestions: [
          'Check if Neo4j is running (pnpm infra:up)',
          'Verify database credentials in .env.local',
          'Check the query syntax in the console',
        ],
      };
    }

    // Custom query returned empty
    if (isCustomQuery) {
      return {
        icon: Search,
        title: 'Query returned no results',
        description:
          'The custom Cypher query executed successfully but found no matching data.',
        accentClass: 'accent-amber',
        suggestions: [
          'Check if the query matches existing data',
          'Try a broader query with fewer constraints',
          'Use MATCH (n) RETURN n LIMIT 10 to verify data exists',
        ],
      };
    }

    // View returned empty
    if (activeView) {
      return {
        icon: FileQuestion,
        title: 'View returned no results',
        description: `View "${activeViewId}" executed but found no matching data. ${activeView.description || 'Check if the expected nodes and relationships exist.'}`,
        accentClass: 'accent-purple',
        suggestions: [
          'Run pnpm infra:seed to populate sample data',
          'Try the "Complete Graph" view to see all available data',
          'Switch to META mode to view the schema structure',
          'Check Neo4j Browser directly to verify data exists',
        ],
      };
    }

    // Generic empty state
    return {
      icon: Database,
      title: 'No data to display',
      description:
        result && result.totalNodes === 0
          ? 'The query returned 0 nodes. The database may be empty or the view criteria do not match any data.'
          : 'Select a view or execute a query to visualize data.',
      accentClass: 'accent-blue',
      suggestions: [
        'Ensure Neo4j is running: pnpm infra:up',
        'Seed the database: pnpm infra:seed',
        'Select a different view from the picker',
      ],
    };
  };

  const info = getDiagnosticInfo();
  const Icon = info.icon;

  // SSR/Hydration: show stable loading state until mounted
  // This prevents hydration mismatch from Zustand store state differences
  if (!hasMounted || isExecuting) {
    return (
      <div className={cn('flex items-center justify-center h-full', className)}>
        <div className="flex flex-col items-center gap-4">
          <RefreshCw className="w-8 h-8 text-accent-blue animate-spin" />
          <p className="text-sm text-white/70">Executing query...</p>
        </div>
      </div>
    );
  }

  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center h-full',
        gapTokens.large,
        className
      )}
    >
      {/* Icon */}
      <div
        className={cn(
          'flex items-center justify-center w-16 h-16 rounded-2xl border',
          `bg-${info.accentClass}/15`,
          `border-${info.accentClass}/30`
        )}
      >
        <Icon className={cn('w-8 h-8', `text-${info.accentClass}`)} />
      </div>

      {/* Title & Description */}
      <div className="text-center max-w-md">
        <p className="font-medium text-base text-white/90">{info.title}</p>
        <p className="text-xs text-white/50 mt-2">{info.description}</p>
      </div>

      {/* View Context */}
      {activeViewId && !isCustomQuery && (
        <div className="flex items-center gap-2 text-xs text-white/40">
          <Filter className="w-3 h-3" />
          <span>
            View: <code className="text-white/60">{activeViewId}</code>
          </span>
        </div>
      )}

      {/* Suggestions */}
      <div className="mt-4 p-4 rounded-xl bg-white/[0.03] border border-white/[0.06] max-w-md">
        <p className="text-xs font-medium text-white/60 mb-2">
          Troubleshooting suggestions:
        </p>
        <ul className="space-y-1.5">
          {info.suggestions.map((suggestion, i) => (
            <li
              key={i}
              className="text-xs text-white/40 flex items-start gap-2"
            >
              <span className="text-white/20 select-none">•</span>
              <span className="font-mono text-[10px]">{suggestion}</span>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
});
