'use client';

/**
 * ViewScrollBar - Horizontal scrollable view selector
 *
 * v12: Views are the single source of truth for navigation.
 * Replaces META/DATA mode toggle with scrollable view pills.
 *
 * Features:
 * - Left/right scroll buttons
 * - Custom Query indicator when custom Cypher is active
 * - View pills with active state styling
 * - Keyboard shortcuts (1-9 for quick access)
 */

import { memo, useRef, useCallback, useEffect } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { ChevronLeft, ChevronRight, Zap, Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes } from '@/design/tokens';
import { useViewStore } from '@/stores/viewStore';
import type { ViewRegistryEntry } from '@novanet/core/filters';

// Keyboard shortcut mapping (1-9 keys map to first 9 views)
const SHORTCUT_KEYS = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

interface ViewScrollBarProps {
  className?: string;
}

export const ViewScrollBar = memo(function ViewScrollBar({
  className,
}: ViewScrollBarProps) {
  const scrollRef = useRef<HTMLDivElement>(null);

  const {
    categories,
    activeViewId,
    isCustomQuery,
    isExecuting,
    executeView,
  } = useViewStore(
    useShallow((state) => ({
      categories: state.categories,
      activeViewId: state.activeViewId,
      isCustomQuery: state.isCustomQuery,
      isExecuting: state.isExecuting,
      executeView: state.executeView,
    }))
  );

  // Flatten views for scroll bar
  const allViews = categories.flatMap((cat) => cat.views);

  // Scroll handler
  const scroll = useCallback((direction: 'left' | 'right') => {
    if (!scrollRef.current) return;
    const amount = 200;
    scrollRef.current.scrollBy({
      left: direction === 'left' ? -amount : amount,
      behavior: 'smooth',
    });
  }, []);

  // Handle view selection
  const handleViewClick = useCallback(
    (view: ViewRegistryEntry) => {
      executeView(view.id);
    },
    [executeView]
  );

  // Keyboard shortcuts (1-9 for first 9 views)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ignore if typing in an input
      if (
        e.target instanceof HTMLInputElement ||
        e.target instanceof HTMLTextAreaElement
      ) {
        return;
      }

      const index = SHORTCUT_KEYS.indexOf(e.key);
      if (index !== -1 && index < allViews.length) {
        e.preventDefault();
        handleViewClick(allViews[index]);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [allViews, handleViewClick]);

  // Get shortcut key for a view (if in first 9)
  const getShortcut = (viewId: string): string | undefined => {
    const index = allViews.findIndex((v) => v.id === viewId);
    return index !== -1 && index < SHORTCUT_KEYS.length
      ? SHORTCUT_KEYS[index]
      : undefined;
  };

  // Empty state
  if (allViews.length === 0) {
    return null;
  }

  return (
    <div
      className={cn(
        'flex items-center gap-2 px-3 py-2 bg-[#0d0d12]/95 border-b border-white/[0.06]',
        className
      )}
    >
      {/* Scroll Left */}
      <button
        onClick={() => scroll('left')}
        className={cn(
          'p-1.5 rounded-md transition-colors',
          'text-white/40 hover:text-white/70 hover:bg-white/[0.06]'
        )}
        aria-label="Scroll views left"
      >
        <ChevronLeft className={iconSizes.sm} />
      </button>

      {/* Scrollable Views */}
      <div
        ref={scrollRef}
        className="flex-1 flex items-center gap-1.5 overflow-x-auto scrollbar-hide"
        style={{ scrollbarWidth: 'none', msOverflowStyle: 'none' }}
      >
        {/* Custom Query indicator */}
        {isCustomQuery && (
          <div
            className={cn(
              'flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium',
              'bg-amber-500/15 text-amber-400 border border-amber-500/25',
              'flex-shrink-0'
            )}
          >
            <Zap className="w-3 h-3" />
            <span>Custom Query</span>
          </div>
        )}

        {/* View Pills */}
        {allViews.map((view) => {
          const isActive = activeViewId === view.id && !isCustomQuery;
          const shortcut = getShortcut(view.id);

          return (
            <button
              key={view.id}
              onClick={() => handleViewClick(view)}
              disabled={isExecuting}
              className={cn(
                'flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium',
                'whitespace-nowrap transition-all duration-200',
                'flex-shrink-0',
                isActive
                  ? 'bg-violet-500/15 text-violet-300 border border-violet-500/30'
                  : 'text-white/50 hover:text-white/80 hover:bg-white/[0.06] border border-transparent',
                isExecuting && 'opacity-50 cursor-not-allowed'
              )}
              title={view.description}
            >
              {/* Loading indicator for active view being executed */}
              {isActive && isExecuting && (
                <Loader2 className="w-3 h-3 animate-spin" />
              )}

              {/* View name - use first word of description or id */}
              <span>{view.description?.split(' ')[0] || view.id}</span>

              {/* Keyboard shortcut badge */}
              {shortcut && (
                <kbd
                  className={cn(
                    'ml-0.5 px-1 py-0.5 text-[9px] font-mono rounded',
                    isActive
                      ? 'bg-violet-500/25 text-violet-300'
                      : 'bg-white/[0.06] text-white/30'
                  )}
                >
                  {shortcut}
                </kbd>
              )}
            </button>
          );
        })}
      </div>

      {/* Scroll Right */}
      <button
        onClick={() => scroll('right')}
        className={cn(
          'p-1.5 rounded-md transition-colors',
          'text-white/40 hover:text-white/70 hover:bg-white/[0.06]'
        )}
        aria-label="Scroll views right"
      >
        <ChevronRight className={iconSizes.sm} />
      </button>
    </div>
  );
});

export default ViewScrollBar;
