'use client';

/**
 * SegmentedTabs - iOS-style segmented control for tab switching
 *
 * Design rules applied:
 * - Hit targets ≥24px desktop, ≥44px mobile
 * - Tabular nums for consistent number alignment
 * - Visible focus rings
 * - Generous spacing
 */

import { memo, useCallback, useId } from 'react';
import { cn } from '@/lib/utils';

export interface SegmentedTab {
  id: string;
  label: string;
  count?: number;
}

export interface SegmentedTabsProps {
  tabs: SegmentedTab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
  className?: string;
}

export const SegmentedTabs = memo(function SegmentedTabs({
  tabs,
  activeTab,
  onTabChange,
  className,
}: SegmentedTabsProps) {
  const groupId = useId();

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent, currentIndex: number) => {
      let newIndex = currentIndex;

      if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
        e.preventDefault();
        newIndex = currentIndex > 0 ? currentIndex - 1 : tabs.length - 1;
      } else if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
        e.preventDefault();
        newIndex = currentIndex < tabs.length - 1 ? currentIndex + 1 : 0;
      } else if (e.key === 'Home') {
        e.preventDefault();
        newIndex = 0;
      } else if (e.key === 'End') {
        e.preventDefault();
        newIndex = tabs.length - 1;
      }

      if (newIndex !== currentIndex) {
        onTabChange(tabs[newIndex].id);
      }
    },
    [tabs, onTabChange]
  );

  return (
    <div
      role="tablist"
      aria-label="Content tabs"
      className={cn(
        'flex rounded-lg bg-white/[0.03] p-0.5',
        className
      )}
    >
      {tabs.map((tab, index) => {
        const isActive = tab.id === activeTab;
        const tabId = `${groupId}-tab-${tab.id}`;

        return (
          <button
            key={tab.id}
            id={tabId}
            role="tab"
            aria-selected={isActive}
            aria-controls={`${groupId}-panel-${tab.id}`}
            tabIndex={isActive ? 0 : -1}
            onClick={() => onTabChange(tab.id)}
            onKeyDown={(e) => handleKeyDown(e, index)}
            className={cn(
              // Layout
              'flex-1 flex items-center justify-center gap-1.5',
              'min-h-[32px] rounded-md',
              // Typography
              'text-xs font-medium',
              // Transitions
              'transition duration-150',
              // Focus
              'focus:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/60 focus-visible:ring-offset-1 focus-visible:ring-offset-black/50',
              // States
              isActive
                ? 'bg-white/[0.08] text-white/90 shadow-sm'
                : 'text-white/40 hover:text-white/60 hover:bg-white/[0.04]'
            )}
          >
            <span>{tab.label}</span>
            {tab.count !== undefined && (
              <span
                className={cn(
                  'tabular-nums text-[10px]',
                  isActive ? 'text-white/50' : 'text-white/30'
                )}
              >
                {tab.count.toLocaleString()}
              </span>
            )}
          </button>
        );
      })}
    </div>
  );
});

export default SegmentedTabs;
