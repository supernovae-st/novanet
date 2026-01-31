'use client';

/**
 * FilterSection - Collapsible section header for filter groups
 *
 * Design: Unified with FilterTree.Section for consistency
 * - Chevron expand/collapse (right-aligned)
 * - Tri-state checkbox for bulk select (uses TriStateCheckbox component)
 * - Icon + label + count with glow effects
 * - Animated expand/collapse
 */

import {
  memo,
  useCallback,
  useRef,
  type ReactNode,
  type KeyboardEvent,
} from 'react';
import { ChevronDown } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes, gapTokens, glowEffects } from '@/design/tokens';
import { useControllableState } from '@/hooks/useControllableState';
import { TriStateCheckbox, type CheckboxState } from './TriStateCheckbox';

// Re-export for backwards compatibility
export type { CheckboxState };

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

  // Check if any items are selected for glow effect
  const hasSelection = checkboxState !== 'none';

  return (
    <div className={cn('space-y-3', className)}>
      {/* Section Header - Unified: checkbox | icon | label | count | chevron (right) */}
      <div
        className={cn(
          'flex items-center w-full py-1 px-1 group rounded-lg',
          'transition-all duration-300',
          'hover:bg-white/[0.03]',
          gapTokens.comfortable
        )}
        style={{
          // Subtle glow when items are selected - uses design token
          boxShadow: hasSelection ? glowEffects.section(accentColor) : undefined,
        }}
      >
        {/* Tri-state Checkbox - separate from expand button */}
        <TriStateCheckbox
          state={checkboxState}
          onClick={onCheckboxClick}
          color={accentColor}
          label={`Select all ${label}`}
        />

        {/* Expand/Collapse area - icon | label | count | chevron */}
        <button
          ref={headerRef}
          onClick={handleToggle}
          onKeyDown={handleKeyDown}
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
              color: accentColor,
              filter: hasSelection ? `drop-shadow(0 0 4px ${accentColor}60)` : undefined,
            }}
          >
            {icon}
          </span>

          {/* Label */}
          <span
            className={cn(
              'text-[11px] uppercase tracking-wider font-semibold',
              'transition-all duration-300'
            )}
            style={{
              color: accentColor,
              textShadow: hasSelection ? `0 0 8px ${accentColor}40` : undefined,
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
              ({count})
            </span>
          )}

          {/* Chevron - RIGHT aligned */}
          <ChevronDown
            className={cn(
              'ml-auto',
              iconSizes.md,
              'text-white/40',
              'transition-all duration-200',
              'group-hover:text-white/50',
              !isExpanded && '-rotate-90'
            )}
          />
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
        <div className={cn('flex flex-col py-2.5', gapTokens.comfortable)}>{children}</div>
      </div>
    </div>
  );
});
