'use client';

/**
 * FilterPanel - Label filter panel for granular type filtering
 *
 * Features:
 * - LabelFilter for granular type filtering
 * - Glassmorphism design matching existing sidebar panels
 *
 * Note: AI Search and ViewSelector have been moved to DatabaseInfoPanel
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';
import { LabelFilter } from './LabelFilter';

export interface FilterPanelProps {
  className?: string;
}

export const FilterPanel = memo(function FilterPanel({ className }: FilterPanelProps) {
  return (
    <div
      className={cn(
        'h-full flex flex-col bg-gradient-to-b from-black/60 to-black/40 backdrop-blur-xl',
        className
      )}
      data-testid="filter-panel"
    >
      {/* Content - Label Filter only */}
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        <div className="p-3">
          <LabelFilter />
        </div>
      </div>
    </div>
  );
});
