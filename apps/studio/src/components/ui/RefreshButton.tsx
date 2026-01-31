'use client';

/**
 * RefreshButton - Reusable refresh button with loading state and hover text
 *
 * Uses NovaNet Icon Design System.
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';
import { ACTION_ICONS, ICON_SIZES } from '@/config/iconSystem';
import { gapTokens } from '@/design/tokens';

interface RefreshButtonProps {
  /** Click handler */
  onClick: () => void;
  /** Loading state (shows spinner) */
  isLoading?: boolean;
  /** Additional CSS classes */
  className?: string;
}

const RefreshIcon = ACTION_ICONS.refresh;

export const RefreshButton = memo(function RefreshButton({
  onClick,
  isLoading = false,
  className,
}: RefreshButtonProps) {
  return (
    <button
      onClick={onClick}
      disabled={isLoading}
      className={cn(
        'group flex items-center p-1.5 rounded-lg transition-colors',
        gapTokens.compact,
        'text-white/40 hover:text-white/70 hover:bg-white/10',
        'disabled:opacity-50',
        className
      )}
      title="Refresh data"
    >
      <RefreshIcon className={cn(ICON_SIZES.md, isLoading && 'animate-spin')} />
      <span className="max-w-0 overflow-hidden group-hover:max-w-[60px] transition-all duration-200 text-xs font-medium">
        Refresh
      </span>
    </button>
  );
});
