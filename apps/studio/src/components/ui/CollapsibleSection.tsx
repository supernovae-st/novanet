'use client';

/**
 * CollapsibleSection - Unified collapsible section for sidebar filters
 *
 * Design System Component - Use this for ALL collapsible sections:
 * - Views tab (ViewCategorySection)
 * - Nodes tab (FilterTree.Section)
 * - Schema tab (FilterSection)
 *
 * Features:
 * - Icon + Label + Count on the left
 * - Chevron on the RIGHT (unified design)
 * - Optional tri-state checkbox
 * - Smooth expand/collapse animation
 * - Keyboard navigation (Arrow keys, Enter, Space)
 * - WCAG accessible
 */

import {
  memo,
  useState,
  useCallback,
  useRef,
  type ReactNode,
  type KeyboardEvent,
} from 'react';
import { ChevronDown, Check, Minus } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes, gapTokens } from '@/design/tokens';
import type { CheckboxState } from './FilterSection';

export type { CheckboxState };

export interface CollapsibleSectionProps {
  /** Section identifier */
  id: string;
  /** Section label */
  label: string;
  /** Icon (LucideIcon component or emoji) */
  icon: ReactNode;
  /** Accent color (hex or tailwind class) */
  color?: string;
  /** Item count to display */
  count?: number;
  /** Children content */
  children: ReactNode;
  /** Default expanded state */
  defaultExpanded?: boolean;
  /** Controlled expanded state */
  isExpanded?: boolean;
  /** Callback when expanded changes */
  onExpandedChange?: (expanded: boolean) => void;
  /** Show tri-state checkbox */
  showCheckbox?: boolean;
  /** Checkbox state (requires showCheckbox) */
  checkboxState?: CheckboxState;
  /** Callback when checkbox is clicked */
  onCheckboxClick?: () => void;
  /** Show indent line on content */
  showIndentLine?: boolean;
  /** Additional class names */
  className?: string;
}

// =============================================================================
// Component
// =============================================================================

export const CollapsibleSection = memo(function CollapsibleSection({
  id,
  label,
  icon,
  color = '#8b8b8b',
  count,
  children,
  defaultExpanded = true,
  isExpanded: controlledExpanded,
  onExpandedChange,
  showCheckbox = false,
  checkboxState = 'none',
  onCheckboxClick,
  showIndentLine = false,
  className,
}: CollapsibleSectionProps) {
  // Use controlled or uncontrolled state
  const [internalExpanded, setInternalExpanded] = useState(defaultExpanded);
  const isExpanded = controlledExpanded !== undefined ? controlledExpanded : internalExpanded;

  const headerRef = useRef<HTMLButtonElement>(null);

  const setExpanded = useCallback(
    (value: boolean | ((prev: boolean) => boolean)) => {
      const newValue = typeof value === 'function' ? value(isExpanded) : value;
      if (controlledExpanded === undefined) {
        setInternalExpanded(newValue);
      }
      onExpandedChange?.(newValue);
    },
    [isExpanded, controlledExpanded, onExpandedChange]
  );

  const handleToggle = useCallback(() => {
    setExpanded((prev) => !prev);
  }, [setExpanded]);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent<HTMLButtonElement>) => {
      switch (e.key) {
        case 'ArrowLeft':
          if (isExpanded) {
            e.preventDefault();
            setExpanded(false);
          }
          break;
        case 'ArrowRight':
          if (!isExpanded) {
            e.preventDefault();
            setExpanded(true);
          }
          break;
        case 'Enter':
        case ' ':
          e.preventDefault();
          handleToggle();
          break;
      }
    },
    [isExpanded, setExpanded, handleToggle]
  );

  const handleCheckboxClick = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onCheckboxClick?.();
    },
    [onCheckboxClick]
  );

  return (
    <div className={cn('space-y-3', className)}>
      {/* Section Header */}
      <button
        ref={headerRef}
        onClick={handleToggle}
        onKeyDown={handleKeyDown}
        aria-expanded={isExpanded}
        aria-controls={`collapsible-section-${id}`}
        aria-label={`${isExpanded ? 'Collapse' : 'Expand'} ${label}`}
        className={cn('flex items-center w-full group py-0.5', gapTokens.comfortable)}
      >
        {/* Tri-state Checkbox (optional) */}
        {showCheckbox && (
          <div
            onClick={handleCheckboxClick}
            role="checkbox"
            aria-checked={checkboxState === 'all' ? true : checkboxState === 'partial' ? 'mixed' : false}
            aria-label={`Select all ${label}`}
            className={cn(
              'flex-shrink-0 flex items-center justify-center',
              'w-5 h-5 rounded-md',
              'border-[1.5px] transition-all duration-200',
              'hover:scale-105 cursor-pointer',
              checkboxState === 'none'
                ? 'border-white/25 bg-transparent'
                : 'border-transparent'
            )}
            style={{
              backgroundColor: checkboxState !== 'none' ? `${color}25` : undefined,
              borderColor: checkboxState !== 'none' ? color : undefined,
            }}
          >
            {checkboxState === 'all' && (
              <Check className={iconSizes.xs} style={{ color }} strokeWidth={2.5} />
            )}
            {checkboxState === 'partial' && (
              <Minus className={iconSizes.xs} style={{ color }} strokeWidth={2.5} />
            )}
          </div>
        )}

        {/* Icon */}
        <span className="flex-shrink-0" style={{ color }}>
          {icon}
        </span>

        {/* Label */}
        <span
          className="text-[11px] uppercase tracking-wider font-semibold text-white/45"
          style={{ color }}
        >
          {label}
        </span>

        {/* Count */}
        {count !== undefined && (
          <span className="text-[10px] tabular-nums text-white/25">({count})</span>
        )}

        {/* Chevron - RIGHT aligned */}
        <ChevronDown
          className={cn(
            'ml-auto',
            iconSizes.md,
            'text-white/30',
            'transition-transform duration-200',
            'group-hover:text-white/50',
            !isExpanded && '-rotate-90'
          )}
        />
      </button>

      {/* Collapsible Content */}
      <div
        id={`collapsible-section-${id}`}
        className={cn(
          'overflow-hidden transition-all duration-300',
          isExpanded ? 'max-h-[800px] opacity-100' : 'max-h-0 opacity-0',
          showIndentLine && 'ml-6 pl-4 border-l-2 border-white/[0.08]'
        )}
        role="group"
        aria-label={`${label} items`}
      >
        <div className={cn('flex flex-col py-1', gapTokens.comfortable)}>{children}</div>
      </div>
    </div>
  );
});
