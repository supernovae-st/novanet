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
 * - WCAG 2.1 AA accessible
 *
 * Used by:
 * - SchemaFilterPanel (Schema Browser)
 * - NodeLabelsSection (Data View)
 */

import {
  createContext,
  useContext,
  useState,
  memo,
  type ReactNode,
} from 'react';
import { cn } from '@/lib/utils';
import { filterTreeClasses as ftc } from '@/design/tokens';
import { TriStateCheckbox, type CheckboxState } from './TriStateCheckbox';
import { ProgressBar } from './ProgressBar';
import { NAV_ICONS, STATUS_ICONS } from '@/config/iconSystem';

const ChevronDownIcon = NAV_ICONS.chevronDown;
const CheckIcon = STATUS_ICONS.success;

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
  /** Additional class names */
  className?: string;
}

function FilterTreeRoot({
  children,
  showProgressBars = false,
  maxCount = 100,
  disabled = false,
  className,
}: FilterTreeRootProps) {
  return (
    <FilterTreeContext.Provider value={{ showProgressBars, maxCount, disabled }}>
      <div className={cn(ftc.container, className)} role="tree">
        {children}
      </div>
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
  /** Default expanded state */
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
  defaultExpanded = true,
  className,
}: FilterTreeSectionProps) {
  const [isExpanded, setIsExpanded] = useState(defaultExpanded);
  const { disabled } = useContext(FilterTreeContext);

  return (
    <div className={cn('mb-1', className)} role="treeitem" aria-expanded={isExpanded}>
      {/* Section Header */}
      <div className={ftc.sectionHeader}>
        {/* Expand/Collapse */}
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          aria-expanded={isExpanded}
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

        {/* Icon + Label (clickable to expand/collapse) */}
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="flex items-center gap-2 flex-1 min-w-0"
        >
          <span className="flex-shrink-0">{icon}</span>
          <span className={ftc.sectionLabel} style={{ color }}>
            {label}
          </span>
          {count !== undefined && (
            <span className={ftc.countMuted}>({count.toLocaleString()})</span>
          )}
        </button>
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

  return (
    <button
      onClick={onToggle}
      disabled={disabled}
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
          {count > 999 ? `${(count / 1000).toFixed(1)}k` : count}
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
