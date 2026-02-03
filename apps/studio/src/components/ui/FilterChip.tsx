'use client';

/**
 * FilterChip - Unified interactive chip for filtering graph nodes
 *
 * Used in stats bar and sidebar for Realm/Layer/Trait/ArcFamily filtering.
 * Supports click to toggle filter and shift+click to highlight focus.
 *
 * Visual states:
 * - DEFAULT (checked): Full opacity, filled checkbox
 * - FILTERED (unchecked): 30% opacity, empty checkbox
 * - FOCUSED: Glow effect, dot indicator
 * - HOVER: Subtle background
 */

import { memo, useCallback, type MouseEvent, type KeyboardEvent } from 'react';
import { Check } from 'lucide-react';

export interface FilterChipProps {
  /** Full text label, never truncated */
  label: string;
  /** Optional count badge (right-aligned, tabular-nums) */
  count?: number;
  /** Hex color from YAML (border + checkbox fill) */
  color: string;
  /** true = visible in graph */
  checked: boolean;
  /** true = highlight mode active (dims all others) */
  focused?: boolean;
  /** Click handler - toggle filter */
  onToggle: () => void;
  /** Shift+Click handler - highlight focus */
  onFocus?: () => void;
  /** Optional icon to show before the checkbox */
  icon?: React.ReactNode;
  /** Additional className */
  className?: string;
}

export const FilterChip = memo(function FilterChip({
  label,
  count,
  color,
  checked,
  focused = false,
  onToggle,
  onFocus,
  icon,
  className = '',
}: FilterChipProps) {
  // Handle click with shift detection
  const handleClick = useCallback(
    (e: MouseEvent<HTMLButtonElement>) => {
      if (e.shiftKey && onFocus) {
        e.preventDefault();
        onFocus();
      } else {
        onToggle();
      }
    },
    [onToggle, onFocus]
  );

  // Handle keyboard navigation
  const handleKeyDown = useCallback(
    (e: KeyboardEvent<HTMLButtonElement>) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        if (e.shiftKey && onFocus) {
          onFocus();
        } else {
          onToggle();
        }
      }
    },
    [onToggle, onFocus]
  );

  // Determine visual state
  const isActive = checked || focused;
  const opacity = isActive ? 'opacity-100' : 'opacity-40';

  return (
    <button
      type="button"
      role="checkbox"
      aria-checked={checked}
      aria-label={`${label}${count !== undefined ? ` (${count})` : ''}${focused ? ', focused' : ''}`}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      className={`
        group relative flex items-center gap-2 px-2.5 py-1.5
        rounded-lg transition-all duration-150 cursor-pointer
        select-none outline-none
        hover:bg-white/[0.05]
        focus-visible:ring-2 focus-visible:ring-offset-1 focus-visible:ring-offset-black
        ${focused ? 'ring-1' : ''}
        ${opacity}
        ${className}
      `.trim()}
      style={{
        // Dynamic focus ring color
        ['--tw-ring-color' as string]: focused ? color : undefined,
        // Subtle glow when focused
        boxShadow: focused ? `0 0 12px ${color}40` : undefined,
      }}
    >
      {/* Checkbox */}
      <span
        className={`
          relative flex-shrink-0 w-3.5 h-3.5 rounded
          flex items-center justify-center
          border transition-all duration-150
          ${checked ? 'border-transparent' : 'border-current'}
        `}
        style={{
          backgroundColor: checked ? color : 'transparent',
          borderColor: checked ? 'transparent' : `${color}60`,
        }}
        aria-hidden="true"
      >
        {checked && (
          <Check className="w-2.5 h-2.5 text-white" strokeWidth={3} />
        )}
        {/* Focus dot indicator */}
        {focused && !checked && (
          <span
            className="absolute w-1.5 h-1.5 rounded-full"
            style={{ backgroundColor: color }}
          />
        )}
      </span>

      {/* Optional icon */}
      {icon && (
        <span
          className="flex-shrink-0 transition-colors duration-150"
          style={{ color: isActive ? color : undefined }}
        >
          {icon}
        </span>
      )}

      {/* Label - never truncated, can wrap */}
      <span
        className={`
          text-sm font-medium transition-colors duration-150 text-left
          ${isActive ? 'text-white/90' : 'text-white/60'}
        `}
      >
        {label}
      </span>

      {/* Count badge */}
      {count !== undefined && (
        <span
          className={`
            ml-auto text-[10px] tabular-nums flex-shrink-0
            transition-colors duration-150
            ${isActive ? 'text-white/50' : 'text-white/30'}
          `}
        >
          {count}
        </span>
      )}
    </button>
  );
});

/**
 * FilterChipGroup - Container for a group of FilterChips
 *
 * Provides proper spacing and wrapping behavior.
 */
export interface FilterChipGroupProps {
  /** Section title */
  title?: string;
  /** Children FilterChips */
  children: React.ReactNode;
  /** Additional className */
  className?: string;
}

export const FilterChipGroup = memo(function FilterChipGroup({
  title,
  children,
  className = '',
}: FilterChipGroupProps) {
  return (
    <div className={`flex flex-col gap-1 ${className}`}>
      {title && (
        <span className="text-[10px] font-semibold uppercase tracking-wider text-white/40 px-2.5 py-1">
          {title}
        </span>
      )}
      <div className="flex flex-wrap gap-1">
        {children}
      </div>
    </div>
  );
});
