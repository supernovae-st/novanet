'use client';

/**
 * FilterSection - Collapsible section header for filter groups
 *
 * Design: Matches ViewCategorySection for consistency
 * - Chevron expand/collapse
 * - Tri-state checkbox for bulk select
 * - Icon + label + count
 * - Animated expand/collapse
 */

import {
  memo,
  useCallback,
  useRef,
  type ReactNode,
  type KeyboardEvent,
} from 'react';
import { ChevronDown, Check, Minus } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes } from '@/design/tokens';
import { useControllableState } from '@/hooks/useControllableState';

export type CheckboxState = 'all' | 'partial' | 'none';

export interface FilterSectionProps {
  /** Section identifier */
  id: string;
  /** Section label */
  label: string;
  /** Icon (emoji or ReactNode) */
  icon: ReactNode;
  /** Accent color for the section */
  accentColor?: string;
  /** Total count */
  count?: number;
  /** Checkbox state */
  checkboxState: CheckboxState;
  /** Callback when checkbox is clicked */
  onCheckboxClick: () => void;
  /** Children (filter items) */
  children: ReactNode;
  /** Controlled expanded state */
  isExpanded?: boolean;
  /** Callback when expanded changes */
  onExpandedChange?: (expanded: boolean) => void;
  /** Default expanded (uncontrolled) */
  defaultExpanded?: boolean;
  /** Additional class names */
  className?: string;
}

export const FilterSection = memo(function FilterSection({
  id,
  label,
  icon,
  accentColor = '#8b5cf6',
  count,
  checkboxState,
  onCheckboxClick,
  children,
  isExpanded: controlledExpanded,
  onExpandedChange,
  defaultExpanded = true,
  className,
}: FilterSectionProps) {
  const [isExpanded, setIsExpanded] = useControllableState(
    controlledExpanded,
    defaultExpanded,
    onExpandedChange
  );

  const headerRef = useRef<HTMLButtonElement>(null);

  const handleToggle = useCallback(() => {
    setIsExpanded((prev) => !prev);
  }, [setIsExpanded]);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent<HTMLButtonElement>) => {
      if (e.key === 'ArrowLeft' && isExpanded) {
        e.preventDefault();
        setIsExpanded(false);
      } else if (e.key === 'ArrowRight' && !isExpanded) {
        e.preventDefault();
        setIsExpanded(true);
      } else if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        handleToggle();
      }
    },
    [isExpanded, setIsExpanded, handleToggle]
  );

  return (
    <div className={cn('space-y-3', className)}>
      {/* Section Header */}
      <div className="flex items-center gap-2.5 py-1">
        {/* Expand/Collapse Button */}
        <button
          ref={headerRef}
          onClick={handleToggle}
          onKeyDown={handleKeyDown}
          aria-expanded={isExpanded}
          aria-controls={`filter-section-${id}`}
          aria-label={`${isExpanded ? 'Collapse' : 'Expand'} ${label}`}
          className={cn(
            'p-1 -m-1 rounded-lg',
            'transition-colors duration-200',
            'hover:bg-white/[0.06]',
            'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50'
          )}
        >
          <ChevronDown
            className={cn(
              iconSizes.md,
              'text-white/40',
              'transition-transform duration-200',
              !isExpanded && '-rotate-90'
            )}
          />
        </button>

        {/* Tri-state Checkbox */}
        <button
          onClick={onCheckboxClick}
          aria-label={`Select all ${label}`}
          className={cn(
            'flex-shrink-0 flex items-center justify-center',
            'w-5 h-5 rounded-md',
            'border-[1.5px] transition-all duration-200',
            'hover:scale-105',
            'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50',
            checkboxState === 'none'
              ? 'border-white/25 bg-transparent'
              : 'border-transparent'
          )}
          style={{
            backgroundColor: checkboxState !== 'none' ? `${accentColor}25` : undefined,
            borderColor: checkboxState !== 'none' ? accentColor : undefined,
          }}
        >
          {checkboxState === 'all' && (
            <Check className={iconSizes.xs} style={{ color: accentColor }} strokeWidth={2.5} />
          )}
          {checkboxState === 'partial' && (
            <Minus className={iconSizes.xs} style={{ color: accentColor }} strokeWidth={2.5} />
          )}
        </button>

        {/* Icon + Label (clickable to expand) */}
        <button
          onClick={handleToggle}
          className="flex items-center gap-2 flex-1 min-w-0"
        >
          <span className="flex-shrink-0 text-base">{icon}</span>
          <span
            className="text-[11px] font-bold uppercase tracking-wider"
            style={{ color: accentColor }}
          >
            {label}
          </span>
          {count !== undefined && (
            <span className="text-[10px] tabular-nums text-white/30">({count})</span>
          )}
        </button>
      </div>

      {/* Section Content */}
      <div
        id={`filter-section-${id}`}
        className={cn(
          'ml-6 pl-4 border-l-2 border-white/[0.08]',
          'overflow-hidden transition-all duration-300',
          isExpanded ? 'max-h-[800px] opacity-100' : 'max-h-0 opacity-0'
        )}
        role="group"
        aria-label={`${label} items`}
      >
        <div className="flex flex-col gap-2.5 py-2.5">{children}</div>
      </div>
    </div>
  );
});
