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
import { sidebarTokens as st, iconSizes, gapTokens, glowEffects, filterTreeClasses as ftc } from '@/design/tokens';
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
          className={cn(st.tree.container, className)}
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
  checkboxState?: CheckboxState;
  /** Callback when checkbox is clicked */
  onCheckboxClick?: () => void;
  /** Show checkbox (default: true) - set to false for mutually exclusive items like Views */
  showCheckbox?: boolean;
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
  checkboxState = 'none',
  onCheckboxClick,
  showCheckbox = true,
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

  // Check if any items are selected for glow effect
  const hasSelection = showCheckbox && checkboxState !== 'none';

  return (
    <div className={cn(st.section.container, className)} role="treeitem" aria-expanded={isExpanded} aria-labelledby={`section-label-${id}`}>
      {/* Section Header - Unified design: checkbox | icon | label | count | chevron (right) */}
      <div
        className={cn(
          st.section.header,
          'group'
        )}
        style={{
          // Subtle glow when items are selected - uses design token
          boxShadow: hasSelection ? glowEffects.section(color) : undefined,
        }}
      >
        {/* Tri-state Checkbox - optional, hidden for mutually exclusive items like Views */}
        {/* When hidden, add spacer to maintain horizontal alignment across all tabs */}
        {showCheckbox && onCheckboxClick ? (
          <TriStateCheckbox
            state={checkboxState}
            onClick={onCheckboxClick}
            color={color}
            disabled={disabled}
            label={`Select all ${label}`}
          />
        ) : (
          <div className={st.checkbox.spacer} aria-hidden="true" />
        )}

        {/* Expand/Collapse area - icon | label | count | chevron */}
        <button
          ref={headerRef}
          onClick={handleToggle}
          onKeyDown={handleKeyDown}
          tabIndex={tabIndex}
          aria-expanded={isExpanded}
          aria-controls={`filter-section-${id}`}
          aria-label={`${isExpanded ? 'Collapse' : 'Expand'} ${label}`}
          className={cn(
            'flex items-center flex-1 min-w-0',
            'active:scale-[0.99]',
            'transition-transform duration-150',
            gapTokens.comfortable
          )}
        >
          {/* Icon with glow effect */}
          <span
            className="flex-shrink-0 transition-all duration-300 group-hover:scale-110"
            style={{
              color,
              filter: hasSelection ? `drop-shadow(0 0 4px ${color}60)` : undefined,
            }}
          >
            {icon}
          </span>

          {/* Label */}
          <span
            id={`section-label-${id}`}
            className={cn(
              'text-[11px] uppercase tracking-wider font-semibold',
              'transition-all duration-300'
            )}
            style={{
              color,
              textShadow: hasSelection ? `0 0 8px ${color}40` : undefined,
            }}
          >
            {label}
          </span>

          {/* Count with glow */}
          {count !== undefined && (
            <span
              className={cn(
                'text-[10px] tabular-nums',
                'transition-colors duration-300',
                hasSelection ? 'text-white/50' : 'text-white/40'
              )}
            >
              ({formatCount(count)})
            </span>
          )}

          {/* Chevron - RIGHT aligned */}
          <ChevronDownIcon
            className={cn(
              st.chevron.base,
              'group-hover:text-white/50',
              !isExpanded && st.chevron.collapsed
            )}
          />
        </button>
      </div>

      {/* Section Content */}
      <div
        id={`filter-section-${id}`}
        className={cn(
          st.section.content,
          isExpanded ? st.section.contentExpanded : st.section.contentCollapsed
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
  /** Show checkbox (default: true) - set to false for Views */
  showCheckbox?: boolean;
  /** Keyboard shortcut to display (e.g., "1", "2") */
  shortcut?: string;
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
  showCheckbox = true,
  shortcut,
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
      role={showCheckbox ? 'checkbox' : 'button'}
      aria-checked={showCheckbox ? isSelected : undefined}
      aria-pressed={!showCheckbox ? isSelected : undefined}
      aria-label={`${label}${count !== undefined ? ` (${count})` : ''}${shortcut ? ` (${shortcut})` : ''}`}
      data-selected={isSelected}
      className={cn(
        st.row.base,
        isSelected && st.row.selected,
        disabled && st.row.disabled,
        className
      )}
      style={{
        // Colored row background - always tinted with item color
        backgroundColor: isSelected ? `${color}18` : `${color}0a`,
        // Ring color when selected
        borderColor: isSelected ? `${color}30` : undefined,
        // Glow effect when selected - uses design token
        boxShadow: isSelected ? glowEffects.row(color) : undefined,
      }}
    >
      {/* Checkbox - matches TriStateCheckbox styling with glow */}
      {/* When hidden, add spacer to maintain horizontal alignment across all tabs */}
      {showCheckbox ? (
        <div
          className={cn(
            st.checkbox.base,
            isSelected ? st.checkbox.checked : st.checkbox.unchecked
          )}
          style={{
            backgroundColor: isSelected ? `${color}25` : 'transparent',
            borderColor: isSelected ? color : 'rgb(255 255 255 / 0.2)',
            boxShadow: isSelected ? glowEffects.checkbox(color) : undefined,
          }}
        >
          {isSelected && <CheckIcon className={iconSizes.xs} style={{ color }} />}
        </div>
      ) : (
        <div className={st.checkbox.spacer} aria-hidden="true" />
      )}

      {/* Icon box - uses sidebarTokens.iconBox.base (32x32px) */}
      <div
        className={st.iconBox.base}
        style={{
          // Always show colored tint, brighter when selected
          backgroundColor: isSelected ? `${color}25` : `${color}15`,
          boxShadow: isSelected ? glowEffects.iconBox(color) : undefined,
        }}
      >
        <span
          className="transition-transform duration-200 group-hover:scale-110"
          style={{ color }}
        >
          {icon}
        </span>
      </div>

      {/* Label */}
      <span
        className={cn(
          st.label.base,
          isSelected ? st.label.selected : st.label.unselected
        )}
      >
        {label}
      </span>

      {/* Progress bar (if enabled) */}
      {showProgressBars && count !== undefined && (
        <div className={st.progressBar.container}>
          <ProgressBar value={count} max={maxCount} color={color} />
        </div>
      )}

      {/* Count badge */}
      {count !== undefined && (
        <span
          className={cn(
            st.badge.count,
            isSelected ? st.badge.countSelected : st.badge.countUnselected
          )}
        >
          {formatCount(count, true)}
        </span>
      )}

      {/* Keyboard shortcut badge */}
      {shortcut && (
        <span
          className={cn(
            st.badge.shortcut
          )}
          style={{
            backgroundColor: isSelected ? `${color}30` : `${color}15`,
            color: isSelected ? 'rgba(255,255,255,0.9)' : 'rgba(255,255,255,0.5)',
          }}
        >
          {shortcut}
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
    <div className={cn(st.header.container, className)}>
      <div className={st.header.title}>
        <TriStateCheckbox
          state={checkboxState}
          onClick={onCheckboxClick}
          color={color}
          disabled={disabled}
          label={`Select all ${title}`}
        />
        <span className="flex-shrink-0 text-white/50">{icon}</span>
        <span className="text-xs font-semibold text-white/70">{title}</span>
        <span className={st.section.count}>
          ({totalCount.toLocaleString()})
        </span>
        {selectedCount !== undefined && selectedCount > 0 && (
          <span className={st.header.selectionBadge}>{selectedCount} selected</span>
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
