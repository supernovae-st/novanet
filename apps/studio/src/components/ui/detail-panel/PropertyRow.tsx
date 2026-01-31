'use client';

/**
 * PropertyRow - Reusable key-value property display
 *
 * Features:
 * - Responsive layout (vertical for long values)
 * - Copy button with feedback
 * - Formatted value display (boolean badges, numbers, etc.)
 */

import { memo, type ReactNode } from 'react';
import { cn } from '@/lib/utils';
import { ACTION_ICONS, STATUS_ICONS, ICON_SIZES } from '@/config/iconSystem';
import { gapTokens } from '@/design/tokens';

const CopyIcon = ACTION_ICONS.copy;
const CheckIcon = STATUS_ICONS.success;

export interface PropertyRowProps {
  label: string;
  value: unknown;
  onCopy?: () => void;
  isCopied?: boolean;
  showCopyButton?: boolean;
  className?: string;
}

export const PropertyRow = memo(function PropertyRow({
  label,
  value,
  onCopy,
  isCopied = false,
  showCopyButton = true,
  className,
}: PropertyRowProps) {
  const displayValue = formatValue(value);
  const isLongValue = typeof value === 'string' && value.length > 60;

  return (
    <div
      className={cn(
        'group flex py-2.5 px-3 -mx-3 rounded-lg hover:bg-white/6 transition-colors',
        gapTokens.large,
        isLongValue && ['flex-col', gapTokens.compact],
        className
      )}
    >
      <span className="text-xs text-white/60 shrink-0 min-w-[100px] font-mono">
        {label}
      </span>
      <div className={cn('flex-1 flex items-start min-w-0', gapTokens.default)}>
        <span
          className={cn(
            'text-sm text-white/95 flex-1',
            isLongValue ? 'break-words' : 'truncate'
          )}
        >
          {displayValue}
        </span>
        {showCopyButton && onCopy && (
          <button
            onClick={onCopy}
            aria-label={isCopied ? `${label} copied` : `Copy ${label}`}
            className={cn(
              'p-2 rounded opacity-50 group-hover:opacity-100 transition shrink-0 -mr-1',
              isCopied
                ? 'text-emerald-400 opacity-100'
                : 'text-white/60 hover:text-white/90'
            )}
          >
            {isCopied ? (
              <CheckIcon className={ICON_SIZES.xs} />
            ) : (
              <CopyIcon className={ICON_SIZES.xs} />
            )}
          </button>
        )}
      </div>
    </div>
  );
});

// =============================================================================
// Value Formatting
// =============================================================================

export function formatValue(value: unknown): ReactNode {
  if (value === null || value === undefined) {
    return <span className="text-white/40 italic">null</span>;
  }
  if (typeof value === 'boolean') {
    return (
      <span
        className={cn(
          'px-2 py-0.5 rounded text-xs font-medium',
          value
            ? 'bg-emerald-500/20 text-emerald-400'
            : 'bg-red-500/20 text-red-400'
        )}
      >
        {value ? 'true' : 'false'}
      </span>
    );
  }
  if (typeof value === 'number') {
    return (
      <span className="text-primary font-mono">{value.toLocaleString()}</span>
    );
  }
  if (Array.isArray(value)) {
    if (value.length === 0)
      return <span className="text-white/45 italic">[]</span>;
    return (
      <span className="text-white/65 font-mono">[{value.length} items]</span>
    );
  }
  if (typeof value === 'object') {
    return <span className="text-white/65 font-mono">{'{...}'}</span>;
  }
  return String(value);
}

/**
 * Simple string formatting for inline displays
 */
export function formatValueString(value: unknown, maxLength = 50): string {
  if (value === null || value === undefined) {
    return 'null';
  }
  if (typeof value === 'boolean') {
    return value ? 'true' : 'false';
  }
  if (typeof value === 'number') {
    return value.toLocaleString();
  }
  if (Array.isArray(value)) {
    return `[${value.length} items]`;
  }
  if (typeof value === 'object') {
    return '{object}';
  }
  const str = String(value);
  return str.length > maxLength ? str.slice(0, maxLength) + '...' : str;
}
