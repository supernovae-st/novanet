'use client';

/**
 * CopyButton - Standardized copy button with feedback
 *
 * Features:
 * - Visual feedback on copy (check icon, color change)
 * - Multiple size variants
 * - Accessible with proper ARIA labels
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';
import { ACTION_ICONS, STATUS_ICONS } from '@/config/iconSystem';

const CopyIcon = ACTION_ICONS.copy;
const CheckIcon = STATUS_ICONS.success;

export interface CopyButtonProps {
  onCopy: () => void;
  isCopied?: boolean;
  label?: string;
  size?: 'sm' | 'default' | 'lg';
  variant?: 'ghost' | 'filled';
  className?: string;
}

const sizeClasses = {
  sm: 'p-1.5',
  default: 'p-2',
  lg: 'p-2.5',
};

const iconSizes = {
  sm: 'w-3 h-3',
  default: 'w-3.5 h-3.5',
  lg: 'w-4 h-4',
};

export const CopyButton = memo(function CopyButton({
  onCopy,
  isCopied = false,
  label = 'Copy to clipboard',
  size = 'default',
  variant = 'ghost',
  className,
}: CopyButtonProps) {
  const baseClasses = cn(
    'rounded-lg transition-all duration-200',
    sizeClasses[size]
  );

  const variantClasses = isCopied
    ? 'bg-emerald-500/20 text-emerald-400 scale-110'
    : variant === 'ghost'
      ? 'hover:bg-white/10 text-white/40 hover:text-white/60'
      : 'bg-white/10 hover:bg-white/20 text-white/50 hover:text-white/70';

  return (
    <button
      onClick={onCopy}
      aria-label={label}
      className={cn(baseClasses, variantClasses, className)}
    >
      {isCopied ? (
        <CheckIcon className={iconSizes[size]} />
      ) : (
        <CopyIcon className={iconSizes[size]} />
      )}
    </button>
  );
});
