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
import { glassClasses, panelClasses } from '@/design/tokens';
import { LabelFilter } from './LabelFilter';

export interface FilterPanelProps {
  className?: string;
}

export const FilterPanel = memo(function FilterPanel({ className }: FilterPanelProps) {
  return (
    <div
      className={cn(panelClasses.container, glassClasses.medium, className)}
      data-testid="filter-panel"
    >
      {/* Content - Label Filter only */}
      <div className={panelClasses.body}>
        <LabelFilter />
      </div>
    </div>
  );
});
