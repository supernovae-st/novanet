'use client';

/**
 * FilterCard - Premium filter item with glassmorphism design
 *
 * Design: Unified sidebar design with FilterTree components
 * - Frosted glass background
 * - Checkbox with accent color
 * - Icon with subtle background
 * - Count badge
 *
 * Variants:
 * - default: Standard filter item
 * - compact: Smaller height for dense lists
 */

import { memo, useCallback, useState } from 'react';
import { Check } from 'lucide-react';
import { cn } from '@/lib/utils';
import { glassClasses, gapTokens, iconSizes } from '@/design/tokens';

export interface FilterCardProps {
  /** Unique identifier */
  id: string;
  /** Display label */
  label: string;
  /** Icon (emoji or ReactNode) */
  icon: React.ReactNode;
  /** Accent color for checkbox and hover states */
  accentColor?: string;
  /** Whether the item is selected/checked */
  isSelected: boolean;
  /** Callback when toggled */
  onToggle: () => void;
  /** Optional count badge */
  count?: number;
  /** Optional keyboard shortcut badge */
  shortcut?: string;
  /** Compact variant (smaller height) */
  compact?: boolean;
  /** Disabled state */
  disabled?: boolean;
  /** Additional class names */
  className?: string;
}

export const FilterCard = memo(function FilterCard({
  label,
  icon,
  accentColor = '#8b5cf6',
  isSelected,
  onToggle,
  count,
  compact = false,
  disabled = false,
  className,
}: FilterCardProps) {
  // Click animation state
  const [isClicking, setIsClicking] = useState(false);

  const handleClick = useCallback(() => {
    if (disabled) return;
    setIsClicking(true);
    onToggle();
    setTimeout(() => setIsClicking(false), 300);
  }, [onToggle, disabled]);

  return (
    <button
      onClick={handleClick}
      disabled={disabled}
      aria-pressed={isSelected}
      aria-label={`${isSelected ? 'Deselect' : 'Select'} ${label}`}
      className={cn(
        // Layout - minimum 48px for accessibility (WCAG tap targets)
        'group relative flex items-center w-full',
        compact ? cn('h-12 px-3.5', gapTokens.spacious) : 'h-14 gap-3.5 px-4',
        'rounded-xl',
        // Frosted Glass base - light variant for filter cards
        glassClasses.light,
        'ring-inset',
        // Transitions
        'transition-all duration-200',
        // Focus
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/60 focus-visible:ring-offset-1 focus-visible:ring-offset-black/50',
        // Click animation
        isClicking && 'scale-[0.97]',
        // States
        isSelected
          ? [
              'bg-white/[0.08]',
              'ring-white/[0.12]',
              'shadow-lg shadow-black/20',
            ]
          : [
              'bg-white/[0.03]',
              'ring-white/[0.06]',
              'shadow-sm shadow-black/10',
              'hover:bg-white/[0.06]',
              'hover:ring-white/[0.10]',
            ],
        // Disabled
        disabled && 'opacity-50 cursor-not-allowed',
        className
      )}
    >
      {/* Checkbox - 20px for better visibility */}
      <div
        className={cn(
          'flex-shrink-0 flex items-center justify-center',
          'w-5 h-5 rounded-md',
          'border-[1.5px] transition-all duration-200',
          isSelected
            ? 'border-transparent shadow-sm'
            : 'border-white/25 bg-white/[0.04]'
        )}
        style={{
          backgroundColor: isSelected ? `${accentColor}30` : undefined,
          borderColor: isSelected ? accentColor : undefined,
        }}
      >
        {isSelected && (
          <Check
            className={cn(iconSizes.xs, 'transition-transform duration-200')}
            style={{ color: accentColor }}
            strokeWidth={2.5}
          />
        )}
      </div>

      {/* Icon with subtle background - 32px square */}
      <div
        className={cn(
          'flex-shrink-0 flex items-center justify-center',
          'w-8 h-8 rounded-lg',
          'transition-all duration-200',
          isSelected
            ? 'bg-white/[0.10]'
            : 'bg-white/[0.05] group-hover:bg-white/[0.08]',
          isClicking && 'bg-white/[0.15]'
        )}
      >
        <span className={cn('text-base transition-transform duration-200', isClicking && 'scale-110')}>
          {icon}
        </span>
      </div>

      {/* Label */}
      <span
        className={cn(
          'flex-1 text-left truncate',
          'text-[13px] font-medium leading-tight',
          'transition-colors duration-200',
          isSelected
            ? 'text-white'
            : 'text-white/70 group-hover:text-white/90'
        )}
      >
        {label}
      </span>

      {/* Count badge - pill style */}
      {count !== undefined && (
        <span
          className={cn(
            'flex-shrink-0',
            'min-w-[32px] px-2.5 py-1 rounded-full',
            'text-[11px] font-semibold text-center tabular-nums',
            'transition-all duration-200',
            isSelected
              ? 'bg-white/[0.12] text-white/90'
              : 'bg-white/[0.05] text-white/50 group-hover:bg-white/[0.08] group-hover:text-white/70'
          )}
        >
          {count}
        </span>
      )}
    </button>
  );
});
