'use client';

/**
 * TriStateCheckbox - Checkbox with three states: none, partial, all
 *
 * Features:
 * - Smooth animations
 * - Color customization
 * - Accessible with ARIA attributes
 * - Design system icons
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';
import { STATUS_ICONS } from '@/config/iconSystem';

// Design system icons
const CheckIcon = STATUS_ICONS.success;

// We need Minus icon which isn't in the design system semantic categories
// It's a specific UI element, so we import it directly
import { Minus } from 'lucide-react';

export type CheckboxState = 'none' | 'partial' | 'all';

export interface TriStateCheckboxProps {
  /** Current state of the checkbox */
  state: CheckboxState;
  /** Click handler */
  onClick: () => void;
  /** Color for the checkbox (hex) */
  color: string;
  /** Whether the checkbox is disabled */
  disabled?: boolean;
  /** Accessible label */
  label?: string;
  /** Additional class names */
  className?: string;
}

export const TriStateCheckbox = memo(function TriStateCheckbox({
  state,
  onClick,
  color,
  disabled,
  label,
  className,
}: TriStateCheckboxProps) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      role="checkbox"
      aria-checked={state === 'all' ? 'true' : state === 'partial' ? 'mixed' : 'false'}
      aria-label={label || 'Toggle selection'}
      className={cn(
        'w-4 h-4 rounded border-2 flex items-center justify-center transition-all duration-200',
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50',
        disabled && 'opacity-50 cursor-not-allowed',
        className
      )}
      style={{
        borderColor: state === 'none' ? 'rgba(255,255,255,0.2)' : color,
        backgroundColor: state === 'none' ? 'transparent' : `${color}20`,
      }}
    >
      {state === 'all' && <CheckIcon className="w-3 h-3" style={{ color }} />}
      {state === 'partial' && <Minus className="w-3 h-3" style={{ color }} />}
    </button>
  );
});
