'use client';

/**
 * ModalHeader - Unified modal header component
 *
 * Design: Linear-dark style with icon + title + close button
 * Extracted from KeyboardShortcuts, CypherEditModal, QuerySidebar
 */

import { memo, type ReactNode } from 'react';
import { X } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes, gapTokens } from '@/design/tokens';

export interface ModalHeaderProps {
  /** Title text */
  title: string;
  /** Optional subtitle/description */
  subtitle?: string;
  /** Icon element (component instance, not component type) */
  icon?: ReactNode;
  /** Callback when close button is clicked */
  onClose: () => void;
  /** Additional className for the container */
  className?: string;
  /** Whether to show border bottom (default: true) */
  showBorder?: boolean;
}

export const ModalHeader = memo(function ModalHeader({
  title,
  subtitle,
  icon,
  onClose,
  className,
  showBorder = true,
}: ModalHeaderProps) {
  return (
    <div
      className={cn(
        'flex items-center justify-between px-5 py-4',
        showBorder && 'border-b border-white/10', // opacity.border.light (10%)
        className
      )}
    >
      <div className={cn('flex items-center', gapTokens.spacious)}>
        {icon && (
          // opacity.bg.medium (6%) + opacity.border.light (10%) + opacity.text.muted (60%)
          <div className="w-9 h-9 rounded-xl bg-white/[0.06] border border-white/10 flex items-center justify-center text-white/60">
            {icon}
          </div>
        )}
        <div>
          <h2 className="text-base font-semibold text-white">{title}</h2>
          {subtitle && (
            <p className="text-xs text-white/40 mt-0.5">{subtitle}</p>
          )}
        </div>
      </div>

      <button
        onClick={onClose}
        aria-label="Close"
        className={cn(
          'p-2 rounded-lg transition-colors duration-150',
          'text-white/40 hover:text-white/80',
          'hover:bg-white/[0.06] active:bg-white/[0.08]', // opacity.bg.medium (0.06) + opacity.bg.strong (0.08)
          'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/20'
        )}
      >
        <X className={iconSizes.xl} />
      </button>
    </div>
  );
});
