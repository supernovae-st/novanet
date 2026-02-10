'use client';

/**
 * ContextViewFooter - Horizontal scrolling container for context view cards
 *
 * Features:
 * - Horizontal scroll with fade edges
 * - "Context Views" label with count
 * - "More Views" button opens full modal
 * - Scroll indicators on edges
 */

import { memo, useRef, useState, useCallback, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { ChevronRight, Layers } from 'lucide-react';
import { cn } from '@/lib/utils';
import { ActionCard } from './ActionCard';
import type { ContextView } from '@/hooks/useContextViews';

// =============================================================================
// TYPES
// =============================================================================

interface ContextViewFooterProps {
  views: ContextView[];
  nodeKey: string;
  activeViewId?: string | null;
  onViewSelect: (viewId: string) => void;
  onMoreViews?: () => void;
  className?: string;
}

// =============================================================================
// COMPONENT
// =============================================================================

export const ContextViewFooter = memo(function ContextViewFooter({
  views,
  nodeKey,
  activeViewId,
  onViewSelect,
  onMoreViews,
  className,
}: ContextViewFooterProps) {
  const scrollRef = useRef<HTMLDivElement>(null);
  const [canScrollLeft, setCanScrollLeft] = useState(false);
  const [canScrollRight, setCanScrollRight] = useState(false);

  // Check scroll state
  const updateScrollState = useCallback(() => {
    const el = scrollRef.current;
    if (!el) return;

    setCanScrollLeft(el.scrollLeft > 0);
    setCanScrollRight(el.scrollLeft < el.scrollWidth - el.clientWidth - 1);
  }, []);

  // Update scroll state on mount and resize
  useEffect(() => {
    updateScrollState();

    const el = scrollRef.current;
    if (el) {
      el.addEventListener('scroll', updateScrollState, { passive: true });
      window.addEventListener('resize', updateScrollState);

      return () => {
        el.removeEventListener('scroll', updateScrollState);
        window.removeEventListener('resize', updateScrollState);
      };
    }
  }, [updateScrollState, views]);

  // Scroll handlers
  const scrollLeft = useCallback(() => {
    scrollRef.current?.scrollBy({ left: -180, behavior: 'smooth' });
  }, []);

  const scrollRight = useCallback(() => {
    scrollRef.current?.scrollBy({ left: 180, behavior: 'smooth' });
  }, []);

  if (views.length === 0) {
    return null;
  }

  return (
    <div className={cn('border-t border-white/[0.06] bg-black/20', className)}>
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-2">
        <div className="flex items-center gap-2">
          <Layers className="w-3.5 h-3.5 text-white/40" />
          <span className="text-xs font-medium text-white/60">
            Context Views
          </span>
          <span className="text-[10px] text-white/30 bg-white/5 px-1.5 py-0.5 rounded">
            {views.length}
          </span>
        </div>

        {onMoreViews && (
          <button
            onClick={onMoreViews}
            className={cn(
              'flex items-center gap-1 text-xs text-white/40',
              'hover:text-white/60 transition-colors',
              'focus:outline-none focus-visible:ring-1 focus-visible:ring-white/30 rounded'
            )}
          >
            <span>More Views</span>
            <ChevronRight className="w-3.5 h-3.5" />
          </button>
        )}
      </div>

      {/* Scrollable cards container */}
      <div className="relative">
        {/* Left scroll indicator */}
        <AnimatePresence>
          {canScrollLeft && (
            <motion.button
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              onClick={scrollLeft}
              className={cn(
                'absolute left-0 top-0 bottom-0 z-10 w-8',
                'bg-gradient-to-r from-black/80 to-transparent',
                'flex items-center justify-start pl-1',
                'text-white/50 hover:text-white/80 transition-colors'
              )}
              aria-label="Scroll left"
            >
              <ChevronRight className="w-4 h-4 rotate-180" />
            </motion.button>
          )}
        </AnimatePresence>

        {/* Right scroll indicator */}
        <AnimatePresence>
          {canScrollRight && (
            <motion.button
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              onClick={scrollRight}
              className={cn(
                'absolute right-0 top-0 bottom-0 z-10 w-8',
                'bg-gradient-to-l from-black/80 to-transparent',
                'flex items-center justify-end pr-1',
                'text-white/50 hover:text-white/80 transition-colors'
              )}
              aria-label="Scroll right"
            >
              <ChevronRight className="w-4 h-4" />
            </motion.button>
          )}
        </AnimatePresence>

        {/* Cards scroll container */}
        <div
          ref={scrollRef}
          className={cn(
            'flex gap-3 px-3 pb-3 pt-1',
            'overflow-x-auto scrollbar-none',
            'scroll-smooth snap-x snap-mandatory'
          )}
          style={{
            // Hide scrollbar
            scrollbarWidth: 'none',
            msOverflowStyle: 'none',
          }}
        >
          {views.map((view) => (
            <div key={view.id} className="snap-start">
              <ActionCard
                view={view}
                nodeKey={nodeKey}
                isActive={activeViewId === view.id}
                onClick={onViewSelect}
              />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
});

export default ContextViewFooter;
