'use client';

/**
 * CollapsibleSection - Unified collapsible section for sidebar panels
 *
 * CANONICAL COMPONENT - Use this for collapsible sections in detail panels:
 * - NodeDetailsPanel (properties, relationships)
 * - EdgeDetailsPanel (edge details)
 *
 * NOTE: For filter lists (Views, Nodes, Rels, Schema tabs), use FilterTree
 * instead which provides more specialized features (tri-state checkboxes,
 * progress bars, keyboard navigation).
 *
 * Features:
 * - Icon + Label + Count on the left
 * - Chevron on the RIGHT (unified design)
 * - Optional tri-state checkbox (showCheckbox prop)
 * - Smooth expand/collapse animation (max-h transition)
 * - Keyboard navigation (Arrow keys, Enter, Space)
 * - WCAG AA accessible (4.5:1 contrast)
 *
 * Uses useControllableState for controlled/uncontrolled pattern.
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
import { iconSizes, gapTokens, defaultColors } from '@/design/tokens';
import { useControllableState } from '@/hooks/useControllableState';
import { TriStateCheckbox, type CheckboxState } from './TriStateCheckbox';

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
  color = defaultColors.neutral,
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
  // Use controllable state hook for cleaner controlled/uncontrolled pattern
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
      switch (e.key) {
        case 'ArrowLeft':
          if (isExpanded) {
            e.preventDefault();
            setIsExpanded(false);
          }
          break;
        case 'ArrowRight':
          if (!isExpanded) {
            e.preventDefault();
            setIsExpanded(true);
          }
          break;
        case 'Enter':
        case ' ':
          e.preventDefault();
          handleToggle();
          break;
      }
    },
    [isExpanded, setIsExpanded, handleToggle]
  );

  return (
    <div className={cn('space-y-3', className)}>
      {/* Section Header - Linear style: flat, minimal */}
      <div
        className={cn(
          'flex items-center w-full py-1.5 px-2 group rounded-md',
          gapTokens.default,
          'transition-colors duration-150',
          'hover:bg-white/[0.03]'
        )}
      >
        {/* Tri-state Checkbox (optional) - uses shared component for consistency */}
        {showCheckbox && onCheckboxClick && (
          <TriStateCheckbox
            state={checkboxState}
            onClick={onCheckboxClick}
            color={color}
            label={`Select all ${label}`}
          />
        )}

        {/* Expand/Collapse button */}
        <button
          ref={headerRef}
          onClick={handleToggle}
          onKeyDown={handleKeyDown}
          aria-expanded={isExpanded}
          aria-controls={`collapsible-section-${id}`}
          aria-label={`${isExpanded ? 'Collapse' : 'Expand'} ${label}`}
          className={cn(
            'flex items-center flex-1 min-w-0',
            'active:scale-[0.99]',
            'transition-transform duration-150',
            gapTokens.comfortable
          )}
        >
          {/* Icon - clean, colored */}
          <span
            className="flex-shrink-0 transition-colors duration-150"
            style={{ color }}
          >
            {icon}
          </span>

          {/* Label - clean, colored */}
          <span
            className="text-[11px] uppercase tracking-wider font-semibold transition-colors duration-150"
            style={{ color }}
          >
            {label}
          </span>

          {/* Count */}
          {count !== undefined && (
            <span className="text-[10px] tabular-nums text-white/40">
              ({count})
            </span>
          )}

          {/* Chevron - RIGHT aligned */}
          <ChevronDown
            className={cn(
              'ml-auto',
              iconSizes.md,
              'text-white/40',
              'transition duration-200',
              'group-hover:text-white/50',
              !isExpanded && '-rotate-90'
            )}
          />
        </button>
      </div>

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
