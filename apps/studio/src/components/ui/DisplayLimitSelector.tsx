'use client';

import { memo, useCallback } from 'react';
import { ChevronDown } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useFilterStore } from '@/stores/filterStore';
import { useQueryStore } from '@/stores/queryStore';
import { injectFilters } from '@/lib/cypher/injectFilters';
import { DISPLAY_LIMIT_OPTIONS } from '@/config/constants';
import { iconSizes } from '@/design/tokens';

interface DisplayLimitSelectorProps {
  className?: string;
}

/**
 * Compact dropdown to select the display limit for graph visualization.
 * v12.1: Query-First - changing limit re-executes current view with new LIMIT
 */
export const DisplayLimitSelector = memo(function DisplayLimitSelector({
  className,
}: DisplayLimitSelectorProps) {
  const displayLimit = useFilterStore((state) => state.displayLimit);
  const setDisplayLimit = useFilterStore((state) => state.setDisplayLimit);
  const currentQuery = useQueryStore((state) => state.currentQuery);
  const executeQuery = useQueryStore((state) => state.executeQuery);

  // v12.1: Query-First - re-execute current query with new limit
  const handleLimitChange = useCallback((newLimit: number) => {
    setDisplayLimit(newLimit);
    // Re-execute current query with new limit injected
    if (currentQuery) {
      const modifiedQuery = injectFilters(currentQuery, { displayLimit: newLimit });
      executeQuery(modifiedQuery);
    }
  }, [setDisplayLimit, currentQuery, executeQuery]);

  return (
    <div className={cn('relative inline-flex items-center', className)}>
      <select
        value={displayLimit}
        onChange={(e) => handleLimitChange(Number(e.target.value))}
        className={cn(
          'appearance-none bg-transparent text-white/60 hover:text-white/80',
          'text-xs font-mono cursor-pointer',
          'pl-2 pr-6 py-1 rounded',
          'hover:bg-white/5 transition-colors',
          'focus:outline-none focus:ring-1 focus:ring-accent-blue/50',
          'border border-transparent hover:border-white/10'
        )}
        title="Display limit (max nodes)"
      >
        {DISPLAY_LIMIT_OPTIONS.map((limit) => (
          <option key={limit} value={limit} className="bg-[#0d0d12] text-white">
            max {limit}
          </option>
        ))}
      </select>
      <ChevronDown
        className={cn(iconSizes.xs, 'absolute right-1 pointer-events-none text-white/40')}
      />
    </div>
  );
});
