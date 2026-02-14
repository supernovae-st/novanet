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
  Database,
  Boxes,
  Layers,
  Eye,
  Sparkles,
  type LucideIcon,
} from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { toast } from '@/lib/toast';
import { useViewStore } from '@/stores/viewStore';
import { useUIStore, selectSelectedNodeId } from '@/stores/uiStore';
import { useGraphStore } from '@/stores/graphStore';
import { pickerClasses, iconSizes, gapTokens, getCardStagger } from '@/design/tokens';
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

// Category colors and icons (v0.12.5 unified view system)
const CATEGORY_CONFIG: Record<string, { color: string; label: string; icon: LucideIcon }> = {
  meta: { color: '#8b5cf6', label: 'Meta', icon: Database },
  data: { color: '#6366f1', label: 'Data', icon: Boxes },
  overlay: { color: '#f97316', label: 'Overlay', icon: Layers },
  contextual: { color: '#94a3b8', label: 'Contextual', icon: Eye },
  generation: { color: '#ec4899', label: 'Generation', icon: Sparkles },
};

interface ViewPickerProps {
  className?: string;
}

// View Card component - uses pickerClasses state tokens + stagger animation
const ViewCard = memo(function ViewCard({
  view,
  isSelected,
  isFocused,
  onSelect,
  index,
}: {
  view: ViewRegistryEntry;
  isSelected: boolean;
  isFocused: boolean;
  onSelect: (e: React.MouseEvent) => void;
  index: number;
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
        // Custom layout (left-aligned info cards, larger than standard picker cards)
        'flex flex-col items-start p-5 rounded-2xl text-left gap-3',
        'border transition-all duration-150 relative',
        'min-h-[130px]',
        'hover:scale-[1.02] active:scale-[0.98]',
        isSelected
          ? 'bg-violet-500/15 border-violet-500/40 text-white'
          : isFocused
            ? pickerClasses.cardFocused
            : pickerClasses.cardIdle,
        pickerClasses.cardAnimation,
        getCardStagger(index),
      )}
    >
      {/* Selection indicator */}
      {isSelected && (
        <div className={cn('absolute top-3 right-3', iconSizes.xl, 'rounded-full bg-violet-500 flex items-center justify-center')}>
          <Check className={cn(iconSizes.sm, 'text-white')} strokeWidth={3} />
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
        <IconComponent className={iconSizes.sm} />
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
  onExecute,
}: {
  isOpen: boolean;
  onClose: () => void;
  views: ViewRegistryEntry[];
  activeViewId: string | null;
  /** Called when Ctrl+click - just select view without executing */
  onSelect: (viewId: string) => void;
  /** Called on regular click - select AND execute view */
  onExecute: (viewId: string) => void;
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

  /**
   * Handle view selection.
   * - Regular click: execute view (auto-execute)
   * - Ctrl/Cmd+click: just select view (load query without executing)
   */
  const handleSelect = useCallback(
    (viewId: string, ctrlPressed = false) => {
      if (ctrlPressed) {
        // Ctrl+click: just select, don't execute
        onSelect(viewId);
      } else {
        // Regular click: execute (auto-execute behavior)
        onExecute(viewId);
      }
      delayedClose();
    },
    [onSelect, onExecute, delayedClose]
  );

  // Grid navigation hook
  const { focusedIndex, handleKeyDown, resetFocus } = useGridNavigation({
    columns: GRID_COLUMNS,
    totalItems: filteredViews.length,
    gridRef,
    onSelect: (index: number, e?: React.KeyboardEvent) => {
      if (index >= 0 && index < filteredViews.length) {
        const ctrlPressed = e?.ctrlKey || e?.metaKey || false;
        handleSelect(filteredViews[index].id, ctrlPressed);
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
    <div className={pickerClasses.container} role="presentation">
      {/* Backdrop - Raycast blur ramp */}
      <div className={pickerClasses.backdrop} aria-hidden="true" />

      {/* Modal shell */}
      <div
        ref={containerRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="view-picker-title"
        onKeyDown={handleKeyDown}
        className={cn(pickerClasses.shell, pickerClasses.sizeLarge, 'h-[80vh]')}
      >
        {/* Header */}
        <div className={pickerClasses.header}>
          <div className={cn('flex items-center', gapTokens.spacious)}>
            <div className={cn(pickerClasses.headerIconBox, 'bg-violet-500/20 border-violet-500/30')}>
              <LayoutGrid className={cn(iconSizes.xl, 'text-violet-400')} />
            </div>
            <div>
              <h2 id="view-picker-title" className={pickerClasses.headerTitle}>
                Select View
              </h2>
              <p className={pickerClasses.headerSubtitle}>Choose a view to display</p>
            </div>
          </div>
          <button onClick={onClose} aria-label="Close" className={pickerClasses.closeButton}>
            <X className={iconSizes.xl} />
          </button>
        </div>

        {/* Search */}
        <div className={pickerClasses.searchBar}>
          <Search className={cn(iconSizes.xl, 'text-white/40 shrink-0')} />
          <input
            ref={searchRef}
            type="text"
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            placeholder="Search views\u2026"
            aria-label="Search views"
            aria-describedby="view-picker-hint"
            className={pickerClasses.searchInput}
            autoComplete="off"
            spellCheck={false}
          />
          <span id="view-picker-hint" className="sr-only">Type to filter saved views</span>
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
          className={pickerClasses.grid}
          role="listbox"
          aria-label="Available views"
        >
          <div
            ref={gridRef}
            className={cn('grid grid-cols-3', gapTokens.xl)}
          >
            {filteredViews.map((view, index) => (
              <ViewCard
                key={view.id}
                view={view}
                isSelected={activeViewId === view.id}
                isFocused={focusedIndex === index}
                onSelect={(e) => handleSelect(view.id, e.ctrlKey || e.metaKey)}
                index={index}
              />
            ))}
          </div>

          {/* No results */}
          {filteredViews.length === 0 && (
            <div className={pickerClasses.emptyState}>
              <Search className={cn(iconSizes.xl, 'mx-auto mb-3 opacity-30')} />
              <p className="text-sm font-medium">No views found</p>
              <p className="text-xs opacity-60 mt-1">Try a different search term</p>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className={pickerClasses.footer}>
          <div className={pickerClasses.footerContent}>
            <span>{filteredViews.length} views</span>
            <div className={cn('flex items-center', gapTokens.large)}>
              <span className={cn('flex items-center', gapTokens.compact)}>
                <Kbd>↑↓←→</Kbd>
                <span>Navigate</span>
              </span>
              <span className={cn('flex items-center', gapTokens.compact)}>
                <Kbd>↵</Kbd>
                <span>Execute</span>
              </span>
              <span className={cn('flex items-center', gapTokens.compact)}>
                <Kbd>⌘</Kbd><span>+</span><Kbd>Click</Kbd>
                <span>Load only</span>
              </span>
              <span className={cn('flex items-center', gapTokens.compact)}>
                <Kbd>Esc</Kbd>
                <span>Close</span>
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );

  return createPortal(content, document.body);
});

export const ViewPicker = memo(function ViewPicker({ className }: ViewPickerProps) {
  const [isOpen, setOpen] = useState(false);

  const { categories, activeViewId, selectView, executeView, getActiveView, loadRegistry } = useViewStore(
    useShallow((s) => ({
      categories: s.categories,
      activeViewId: s.activeViewId,
      selectView: s.selectView,
      executeView: s.executeView,
      getActiveView: s.getActiveView,
      loadRegistry: s.loadRegistry,
    }))
  );

  // Get selected node for contextual view filtering
  const selectedNodeId = useUIStore(selectSelectedNodeId);
  const getNodeById = useGraphStore((s) => s.getNodeById);
  const selectedNode = selectedNodeId ? getNodeById(selectedNodeId) : null;

  // Load registry on mount
  useEffect(() => {
    loadRegistry();
  }, [loadRegistry]);

  // Flatten categories and filter by selected node type
  // v11.6.1: Show all non-contextual views + contextual views applicable to selected node
  const views = useMemo(() => {
    const allViews = categories.flatMap((cat) => cat.views);
    const selectedNodeType = selectedNode?.type;

    return allViews.filter((view) => {
      // Non-contextual views are always visible
      if (!view.contextual) return true;

      // Contextual views: show if applicable to selected node type
      if (!selectedNodeType) return true; // No selection: show all contextual

      // If applicable_types is empty or undefined, it applies to all types
      const applicableTypes = view.applicable_types ?? [];
      if (applicableTypes.length === 0) return true;

      // Check if selected node type is in applicable_types
      return applicableTypes.includes(selectedNodeType);
    });
  }, [categories, selectedNode]);
  const activeView = getActiveView();

  const handleOpen = useCallback(() => {
    setOpen(true);
  }, []);

  const handleClose = useCallback(() => {
    setOpen(false);
  }, []);

  // Ctrl+click: just select without executing
  const handleSelect = useCallback(
    (viewId: string) => {
      selectView(viewId);
    },
    [selectView]
  );

  // Regular click: select AND execute
  // v0.12.5: Pass selectedNode.key for contextual/generation views
  const handleExecute = useCallback(
    (viewId: string) => {
      const view = views.find((v) => v.id === viewId);
      const isContextual = view?.contextual || view?.category === 'generation';

      // For contextual views, require a selected node
      if (isContextual) {
        if (!selectedNode) {
          // v0.12.5: Show warning toast instead of silently failing
          toast.warning(
            'Select a node first',
            'This view requires a selected node to show its context.'
          );
          return;
        }
        executeView(viewId, { key: selectedNode.key });
      } else {
        executeView(viewId);
      }
    },
    [executeView, views, selectedNode]
  );

  return (
    <>
      {/* Trigger button */}
      <motion.button
        whileTap={{ scale: 0.97 }}
        onClick={handleOpen}
        className={cn(
          'flex items-center px-3 py-2 rounded-xl',
          gapTokens.spacious,
          'transition-colors duration-150',
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
        onExecute={handleExecute}
      />
    </>
  );
});

export default ViewPicker;
