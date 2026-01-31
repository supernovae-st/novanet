'use client';

/**
 * FilterTree - Unified hierarchical filter component
 *
 * Features:
 * - Compound component pattern (FilterTree.Root, Section, Row)
 * - Tri-state checkboxes for hierarchical selection
 * - Collapsible sections with chevron
 * - Category-colored icons
 * - Optional progress bars (for data counts)
 * - WCAG 2.1 AA accessible with roving tabindex
 * - Controlled/uncontrolled expand state pattern
 *
 * Used by:
 * - SchemaFilterPanel (Schema Browser)
 * - NodeLabelsSection (Data View)
 */

import {
  createContext,
  useContext,
  useMemo,
  useCallback,
  useRef,
  memo,
  type ReactNode,
  type KeyboardEvent,
} from 'react';
import { cn } from '@/lib/utils';
import { filterTreeClasses as ftc } from '@/design/tokens';
import { TriStateCheckbox, type CheckboxState } from './TriStateCheckbox';
import { ProgressBar } from './ProgressBar';
import { NAV_ICONS, STATUS_ICONS } from '@/config/iconSystem';
import { useControllableState } from '@/hooks/useControllableState';
import {
  useRovingTabindexRoot,
  useRovingTabindexItem,
  useRovingKeyboardHandler,
  RovingTabindexProvider,
} from '@/hooks/useRovingTabindex';

const ChevronDownIcon = NAV_ICONS.chevronDown;
const CheckIcon = STATUS_ICONS.success;

// =============================================================================
// Utilities
// =============================================================================

/**
 * Format count for display - uses compact notation for large numbers
 * @param count - The count to format
 * @param compact - Whether to use compact notation (1.2k) for large numbers
 */
function formatCount(count: number, compact = false): string {
  if (compact && count > 999) {
    return `${(count / 1000).toFixed(1)}k`;
  }
  return count.toLocaleString();
}

// =============================================================================
// Context
// =============================================================================

interface FilterTreeContextValue {
  /** Whether to show progress bars */
  showProgressBars: boolean;
  /** Maximum value for progress bars */
  maxCount: number;
  /** Whether items are disabled */
  disabled: boolean;
}

const FilterTreeContext = createContext<FilterTreeContextValue>({
  showProgressBars: false,
  maxCount: 100,
  disabled: false,
});

// =============================================================================
// Root Component
// =============================================================================

export interface FilterTreeRootProps {
  children: ReactNode;
  /** Show progress bars on rows */
  showProgressBars?: boolean;
  /** Maximum count for progress bars */
  maxCount?: number;
  /** Disable all interactions */
  disabled?: boolean;
  /** Enable keyboard navigation (default: true) */
  enableKeyboardNav?: boolean;
  /** Additional class names */
  className?: string;
}

function FilterTreeRoot({
  children,
  showProgressBars = false,
  maxCount = 100,
  disabled = false,
  enableKeyboardNav = true,
  className,
}: FilterTreeRootProps) {
  // Initialize roving tabindex for keyboard navigation
  const rovingContext = useRovingTabindexRoot();
  const keyboardHandler = useRovingKeyboardHandler(enableKeyboardNav ? rovingContext : null);

  // Memoize context value to prevent unnecessary re-renders
  const contextValue = useMemo<FilterTreeContextValue>(
    () => ({
      showProgressBars,
      maxCount,
      disabled,
    }),
    [showProgressBars, maxCount, disabled]
  );

  const handleKeyDown = useCallback(
    (e: KeyboardEvent<HTMLDivElement>) => {
      if (enableKeyboardNav) {
        keyboardHandler(e);
      }
    },
    [enableKeyboardNav, keyboardHandler]
  );

  return (
    <FilterTreeContext.Provider value={contextValue}>
      <RovingTabindexProvider value={enableKeyboardNav ? rovingContext : null}>
        <div
          className={cn(ftc.container, className)}
          role="tree"
          onKeyDown={handleKeyDown}
        >
          {children}
        </div>
      </RovingTabindexProvider>
    </FilterTreeContext.Provider>
  );
}

// =============================================================================
// Section Component (Category Level)
// =============================================================================

export interface FilterTreeSectionProps {
  /** Section ID */
  id: string;
  /** Section label */
  label: string;
  /** Section icon (emoji or ReactNode) */
  icon: ReactNode;
  /** Accent color (hex) */
  color: string;
  /** Checkbox state for the section */
  checkboxState: CheckboxState;
  /** Callback when checkbox is clicked */
  onCheckboxClick: () => void;
  /** Total count for the section */
  count?: number;
  /** Children (rows) */
  children: ReactNode;
  /** Controlled expanded state */
  isExpanded?: boolean;
  /** Callback when expanded state changes */
  onExpandedChange?: (expanded: boolean) => void;
  /** Default expanded state (uncontrolled mode) */
  defaultExpanded?: boolean;
  /** Additional class names */
  className?: string;
}

const FilterTreeSection = memo(function FilterTreeSection({
  id,
  label,
  icon,
  color,
  checkboxState,
  onCheckboxClick,
  count,
  children,
  isExpanded: controlledExpanded,
  onExpandedChange,
  defaultExpanded = true,
  className,
}: FilterTreeSectionProps) {
  // Controlled/uncontrolled expand state
  const [isExpanded, setIsExpanded] = useControllableState(
    controlledExpanded,
    defaultExpanded,
    onExpandedChange
  );

  const { disabled } = useContext(FilterTreeContext);

  // Ref for roving tabindex on the section header
  const headerRef = useRef<HTMLButtonElement>(null);
  const { tabIndex } = useRovingTabindexItem(`section-${id}`, headerRef);

  const handleToggle = useCallback(() => {
    setIsExpanded((prev) => !prev);
  }, [setIsExpanded]);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent<HTMLButtonElement>) => {
      switch (e.key) {
        case 'ArrowLeft':
          if (isExpanded) {
            e.preventDefault();
            e.stopPropagation();
            setIsExpanded(false);
          }
          break;
        case 'ArrowRight':
          if (!isExpanded) {
            e.preventDefault();
            e.stopPropagation();
            setIsExpanded(true);
          }
          break;
        case 'Enter':
        case ' ':
          e.preventDefault();
          e.stopPropagation();
          handleToggle();
          break;
      }
    },
    [isExpanded, setIsExpanded, handleToggle]
  );

  return (
    <div className={cn('mb-1', className)} role="group" aria-labelledby={`section-label-${id}`}>
      {/* Section Header */}
      <div className={ftc.sectionHeader}>
        {/* Expand/Collapse Button - Main focusable element */}
        <button
          ref={headerRef}
          onClick={handleToggle}
          onKeyDown={handleKeyDown}
          tabIndex={tabIndex}
          aria-expanded={isExpanded}
          aria-controls={`filter-section-${id}`}
          aria-label={`${isExpanded ? 'Collapse' : 'Expand'} ${label}`}
          className="p-0.5 -m-0.5 rounded transition-colors hover:bg-white/[0.06] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50"
        >
          <ChevronDownIcon
            className={cn(ftc.chevron, !isExpanded && ftc.chevronCollapsed)}
          />
        </button>

        {/* Tri-state Checkbox */}
        <TriStateCheckbox
          state={checkboxState}
          onClick={onCheckboxClick}
          color={color}
          disabled={disabled}
          label={`Select all ${label}`}
        />

        {/* Icon + Label (clickable to expand/collapse, not focusable) */}
        <div
          id={`section-label-${id}`}
          onClick={handleToggle}
          className="flex items-center gap-2 flex-1 min-w-0 cursor-pointer"
        >
          <span className="flex-shrink-0">{icon}</span>
          <span className={ftc.sectionLabel} style={{ color }}>
            {label}
          </span>
          {count !== undefined && (
            <span className={ftc.countMuted}>({formatCount(count)})</span>
          )}
        </div>
      </div>

      {/* Section Content */}
      <div
        id={`filter-section-${id}`}
        className={cn(
          ftc.sectionContent,
          isExpanded ? ftc.sectionContentExpanded : ftc.sectionContentCollapsed
        )}
        role="group"
        aria-label={`${label} items`}
      >
        {children}
      </div>
    </div>
  );
});

// =============================================================================
// Row Component (Item Level)
// =============================================================================

export interface FilterTreeRowProps {
  /** Item ID */
  id: string;
  /** Item label */
  label: string;
  /** Item icon (emoji or ReactNode) */
  icon: ReactNode;
  /** Accent color (hex) */
  color: string;
  /** Whether the item is selected/checked */
  isSelected: boolean;
  /** Callback when row is clicked */
  onToggle: () => void;
  /** Count for the item */
  count?: number;
  /** Additional class names */
  className?: string;
}

const FilterTreeRow = memo(function FilterTreeRow({
  id,
  label,
  icon,
  color,
  isSelected,
  onToggle,
  count,
  className,
}: FilterTreeRowProps) {
  const { showProgressBars, maxCount, disabled } = useContext(FilterTreeContext);

  // Ref for roving tabindex
  const rowRef = useRef<HTMLButtonElement>(null);
  const { tabIndex } = useRovingTabindexItem(`row-${id}`, rowRef);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent<HTMLButtonElement>) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        e.stopPropagation();
        onToggle();
      }
    },
    [onToggle]
  );

  return (
    <button
      ref={rowRef}
      onClick={onToggle}
      onKeyDown={handleKeyDown}
      disabled={disabled}
      tabIndex={tabIndex}
      role="checkbox"
      aria-checked={isSelected}
      aria-label={`${label}${count !== undefined ? ` (${count})` : ''}`}
      data-selected={isSelected}
      className={cn(
        ftc.row,
        isSelected && ftc.rowSelected,
        disabled && ftc.rowDisabled,
        className
      )}
    >
      {/* Checkbox */}
      <div
        className={cn(
          ftc.checkbox,
          isSelected ? ftc.checkboxChecked : ftc.checkboxUnchecked
        )}
        style={{
          backgroundColor: isSelected ? `${color}30` : 'transparent',
          borderColor: isSelected ? color : undefined,
        }}
      >
        {isSelected && <CheckIcon className="w-2.5 h-2.5" style={{ color }} />}
      </div>

      {/* Icon */}
      <span className="flex-shrink-0" style={{ color }}>
        {icon}
      </span>

      {/* Label */}
      <span
        className={cn(
          ftc.label,
          isSelected ? ftc.labelSelected : ftc.labelUnselected
        )}
      >
        {label}
      </span>

      {/* Progress bar (if enabled) */}
      {showProgressBars && count !== undefined && (
        <div className={ftc.progressBar}>
          <ProgressBar value={count} max={maxCount} color={color} />
        </div>
      )}

      {/* Count */}
      {count !== undefined && (
        <span
          className={cn(
            ftc.count,
            isSelected ? ftc.countSelected : ftc.countUnselected,
            'w-7 text-right font-mono'
          )}
        >
          {formatCount(count, true)}
        </span>
      )}
    </button>
  );
});

// =============================================================================
// Header Component (with total and execute button)
// =============================================================================

export interface FilterTreeHeaderProps {
  /** Header icon */
  icon: ReactNode;
  /** Header title */
  title: string;
  /** Total count */
  totalCount: number;
  /** Checkbox state */
  checkboxState: CheckboxState;
  /** Callback when checkbox is clicked */
  onCheckboxClick: () => void;
  /** Checkbox color */
  color: string;
  /** Number of selected items */
  selectedCount?: number;
  /** Execute button (optional) */
  executeButton?: ReactNode;
  /** Additional class names */
  className?: string;
}

const FilterTreeHeader = memo(function FilterTreeHeader({
  icon,
  title,
  totalCount,
  checkboxState,
  onCheckboxClick,
  color,
  selectedCount,
  executeButton,
  className,
}: FilterTreeHeaderProps) {
  const { disabled } = useContext(FilterTreeContext);

  return (
    <div className={cn(ftc.header, className)}>
      <div className={ftc.headerTitle}>
        <TriStateCheckbox
          state={checkboxState}
          onClick={onCheckboxClick}
          color={color}
          disabled={disabled}
          label={`Select all ${title}`}
        />
        <span className="flex-shrink-0 text-white/50">{icon}</span>
        <span className="text-xs font-semibold text-white/70">{title}</span>
        <span className="text-[10px] text-white/40">
          ({totalCount.toLocaleString()})
        </span>
        {selectedCount !== undefined && selectedCount > 0 && (
          <span className={ftc.headerBadge}>{selectedCount} selected</span>
        )}
      </div>
      {executeButton}
    </div>
  );
});

// =============================================================================
// Export Compound Component
// =============================================================================

export const FilterTree = {
  Root: FilterTreeRoot,
  Section: FilterTreeSection,
  Row: FilterTreeRow,
  Header: FilterTreeHeader,
};
