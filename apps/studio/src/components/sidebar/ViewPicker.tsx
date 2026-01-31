'use client';

/**
 * ViewPicker - View selector with full-screen modal
 *
 * Features:
 * - Same trigger design as ContextPicker (pill with click to open)
 * - Full-screen modal like ProjectPicker
 * - Views organized by category
 * - Keyboard navigation and accessibility
 */

import { useState, useCallback, useMemo, useEffect, useRef, memo, useDeferredValue } from 'react';
import { createPortal } from 'react-dom';
import { motion } from 'motion/react';
import {
  LayoutGrid,
  ChevronDown,
  Check,
  Search,
  X,
  Layers,
  Sparkles,
  Brain,
  FolderOpen,
  TrendingUp,
  type LucideIcon,
} from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { useViewStore } from '@/stores/viewStore';
import { glassClasses, modalClasses, iconSizes, gapTokens } from '@/design/tokens';
import { Kbd } from '@/components/ui';
import {
  useBodyScrollLock,
  useOutsideClick,
  useModalAutoFocus,
  useGridNavigation,
  useTimeoutFn,
  useFocusTrap,
} from '@/hooks';
import { TRANSITION_DURATION_MS } from '@/config/constants';
import type { ViewRegistryEntry } from '@novanet/core/filters';

// Constants
const GRID_COLUMNS = 3;

// Category colors and icons
const CATEGORY_CONFIG: Record<string, { color: string; label: string; icon: LucideIcon }> = {
  scope: { color: '#a78bfa', label: 'Scope', icon: Layers },
  generation: { color: '#34d399', label: 'Generation', icon: Sparkles },
  knowledge: { color: '#60a5fa', label: 'Knowledge', icon: Brain },
  project: { color: '#fbbf24', label: 'Project', icon: FolderOpen },
  mining: { color: '#f472b6', label: 'Mining', icon: TrendingUp },
};

interface ViewPickerProps {
  className?: string;
}

// View Card component
const ViewCard = memo(function ViewCard({
  view,
  isSelected,
  isFocused,
  onSelect,
}: {
  view: ViewRegistryEntry;
  isSelected: boolean;
  isFocused: boolean;
  onSelect: () => void;
}) {
  const config = CATEGORY_CONFIG[view.category] || { color: '#a78bfa', label: 'View', icon: Layers };
  const IconComponent = config.icon;

  return (
    <button
      onClick={onSelect}
      role="option"
      aria-selected={isSelected}
      aria-label={view.description}
      className={cn(
        // Layout: generous padding, comfortable gap
        'flex flex-col items-start p-5 rounded-2xl text-left gap-3',
        'border transition-all duration-150 relative',
        'min-h-[130px]',
        'hover:scale-[1.02] active:scale-[0.98]',
        isSelected
          ? 'bg-violet-500/15 border-violet-500/40 text-white'
          : isFocused
            ? 'bg-white/[0.06] border-white/20 text-white'
            : 'bg-[#111118] border-white/[0.08] hover:bg-[#16161f] hover:border-white/15 text-white/90 hover:text-white'
      )}
    >
      {/* Selection indicator */}
      {isSelected && (
        <div className="absolute top-3 right-3 w-6 h-6 rounded-full bg-violet-500 flex items-center justify-center">
          <Check className="w-3.5 h-3.5 text-white" strokeWidth={3} />
        </div>
      )}

      {/* Category badge */}
      <span
        className="inline-flex items-center gap-1.5 text-[11px] font-semibold uppercase tracking-wide px-2.5 py-1 rounded-lg"
        style={{
          backgroundColor: `${config.color}20`,
          color: config.color,
        }}
      >
        <IconComponent className="w-3.5 h-3.5" />
        {config.label}
      </span>

      {/* Description */}
      <span className="text-[15px] font-medium leading-snug line-clamp-2">
        {view.description}
      </span>

      {/* ID */}
      <span className={cn(
        'text-xs font-mono mt-auto',
        isSelected ? 'text-violet-400' : 'text-white/40'
      )}>
        {view.id}
      </span>
    </button>
  );
});

// Modal component
const ViewPickerModal = memo(function ViewPickerModal({
  isOpen,
  onClose,
  views,
  activeViewId,
  onSelect,
}: {
  isOpen: boolean;
  onClose: () => void;
  views: ViewRegistryEntry[];
  activeViewId: string | null;
  onSelect: (viewId: string) => void;
}) {
  const [searchInput, setSearchInput] = useState('');
  const search = useDeferredValue(searchInput);
  const [mounted, setMounted] = useState(false);

  const searchRef = useRef<HTMLInputElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const gridRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setMounted(true);
  }, []);

  // Delayed close for visual feedback
  const [delayedClose] = useTimeoutFn(onClose, TRANSITION_DURATION_MS);

  const filteredViews = useMemo(() => {
    if (!search.trim()) return views;
    const query = search.toLowerCase();
    return views.filter(
      (v) =>
        v.description.toLowerCase().includes(query) ||
        v.id.toLowerCase().includes(query) ||
        v.category.toLowerCase().includes(query)
    );
  }, [views, search]);

  const handleSelect = useCallback(
    (viewId: string) => {
      onSelect(viewId);
      delayedClose();
    },
    [onSelect, delayedClose]
  );

  // Grid navigation hook
  const { focusedIndex, handleKeyDown, resetFocus } = useGridNavigation({
    columns: GRID_COLUMNS,
    totalItems: filteredViews.length,
    gridRef,
    onSelect: (index) => {
      if (index >= 0 && index < filteredViews.length) {
        handleSelect(filteredViews[index].id);
      }
    },
    onEscape: onClose,
    enabled: isOpen,
  });

  // Modal utilities
  useBodyScrollLock(isOpen);
  useOutsideClick(containerRef, onClose, isOpen);
  useFocusTrap(containerRef, isOpen);
  useModalAutoFocus(searchRef, isOpen, {
    delay: 50,
    onReset: () => {
      setSearchInput('');
      resetFocus();
    },
  });

  if (!mounted || !isOpen) return null;

  const content = (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-6 animate-in fade-in duration-200"
      role="presentation"
    >
      {/* Backdrop */}
      <div className={modalClasses.backdrop} aria-hidden="true" />

      {/* Modal */}
      <div
        ref={containerRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="view-picker-title"
        onKeyDown={handleKeyDown}
        className={cn(
          'relative w-full max-w-4xl max-h-[80vh] overflow-hidden flex flex-col rounded-2xl',
          glassClasses.modal,
          'animate-in zoom-in-95 slide-in-from-bottom-4 duration-300'
        )}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-white/[0.06]">
          <div className="flex items-center gap-4">
            <div className="w-10 h-10 rounded-xl bg-violet-500/20 border border-violet-500/30 flex items-center justify-center">
              <LayoutGrid className="w-5 h-5 text-violet-400" />
            </div>
            <div>
              <h2 id="view-picker-title" className="text-base font-semibold text-white">
                Select View
              </h2>
              <p className="text-sm text-white/50 mt-0.5">Choose a schema view to display</p>
            </div>
          </div>
          <button
            onClick={onClose}
            aria-label="Close"
            className="p-2 rounded-lg hover:bg-white/10 transition-colors text-white/50 hover:text-white"
          >
            <X className={iconSizes.xl} />
          </button>
        </div>

        {/* Search */}
        <div className="flex items-center gap-4 px-6 py-4 border-b border-white/[0.06]">
          <Search className="w-5 h-5 text-white/40 shrink-0" />
          <input
            ref={searchRef}
            type="text"
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            placeholder="Search views..."
            aria-label="Search views"
            className="flex-1 bg-transparent text-white placeholder-white/40 text-sm outline-none border-none ring-0 focus:outline-none focus:ring-0"
            autoComplete="off"
            spellCheck={false}
          />
          {searchInput && (
            <button
              onClick={() => setSearchInput('')}
              aria-label="Clear search"
              className="p-1.5 hover:bg-white/10 rounded-lg transition-colors text-white/40 hover:text-white/60"
            >
              <X className={iconSizes.md} />
            </button>
          )}
        </div>

        {/* Grid */}
        <div
          className="flex-1 overflow-y-auto p-6"
          role="listbox"
          aria-label="Available views"
        >
          <div
            ref={gridRef}
            className="grid grid-cols-3 gap-5"
          >
            {filteredViews.map((view, index) => (
              <ViewCard
                key={view.id}
                view={view}
                isSelected={activeViewId === view.id}
                isFocused={focusedIndex === index}
                onSelect={() => handleSelect(view.id)}
              />
            ))}
          </div>

          {/* No results */}
          {filteredViews.length === 0 && (
            <div className="text-center py-12 text-white/40">
              <Search className={`${iconSizes['2xl']} mx-auto mb-3 opacity-30`} />
              <p className="text-sm font-medium">No views found</p>
              <p className="text-xs opacity-60 mt-1">Try a different search term</p>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="px-6 py-3 border-t border-white/[0.06] flex items-center justify-between text-xs text-white/40">
          <span>{filteredViews.length} views</span>
          <div className={cn('flex items-center', gapTokens.large)}>
            <span className={cn('flex items-center', gapTokens.compact)}>
              <Kbd>↑↓←→</Kbd>
              <span>Navigate</span>
            </span>
            <span className={cn('flex items-center', gapTokens.compact)}>
              <Kbd>↵</Kbd>
              <span>Select</span>
            </span>
            <span className={cn('flex items-center', gapTokens.compact)}>
              <Kbd>Esc</Kbd>
              <span>Close</span>
            </span>
          </div>
        </div>
      </div>
    </div>
  );

  return createPortal(content, document.body);
});

export const ViewPicker = memo(function ViewPicker({ className }: ViewPickerProps) {
  const [isOpen, setOpen] = useState(false);

  const { categories, activeViewId, selectView, getActiveView } = useViewStore(
    useShallow((s) => ({
      categories: s.categories,
      activeViewId: s.activeViewId,
      selectView: s.selectView,
      getActiveView: s.getActiveView,
    }))
  );

  // Flatten categories to get all views
  const views = useMemo(() => categories.flatMap((cat) => cat.views), [categories]);
  const activeView = getActiveView();

  const handleOpen = useCallback(() => {
    setOpen(true);
  }, []);

  const handleClose = useCallback(() => {
    setOpen(false);
  }, []);

  const handleSelect = useCallback(
    (viewId: string) => {
      selectView(viewId);
    },
    [selectView]
  );

  return (
    <>
      {/* Trigger button */}
      <motion.button
        whileTap={{ scale: 0.97 }}
        onClick={handleOpen}
        className={cn(
          'flex items-center gap-3 px-3 py-2 rounded-xl',
          'transition-all duration-150',
          'hover:bg-white/8 active:bg-white/10',
          className
        )}
      >
        <LayoutGrid className="w-5 h-5 shrink-0 text-violet-400" />
        <div className="flex flex-col items-start gap-0.5">
          <span className="text-sm font-medium text-white/90 truncate max-w-[160px]">
            {activeView?.description ?? 'Select View'}
          </span>
          <span className="text-[11px] leading-tight text-white/50">
            {views.length} views available
          </span>
        </div>
        <ChevronDown className="w-4 h-4 ml-1 text-white/50" />
      </motion.button>

      {/* Modal */}
      <ViewPickerModal
        isOpen={isOpen}
        onClose={handleClose}
        views={views}
        activeViewId={activeViewId}
        onSelect={handleSelect}
      />
    </>
  );
});

export default ViewPicker;
