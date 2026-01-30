'use client';

/**
 * IconButton - Unified icon button with variants
 *
 * Consolidates 3+ button patterns into single component:
 * - ghost: Default transparent button
 * - success: Green/emerald action button
 * - danger: Red destructive button
 * - primary: NovaNet brand color button
 */

import { memo } from 'react';
import { LucideIcon, Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';

interface IconButtonProps {
  /** Lucide icon component */
  icon: LucideIcon;
  /** Click handler */
  onClick: () => void;
  /** Visual variant */
  variant?: 'ghost' | 'success' | 'danger' | 'primary';
  /** Size affecting padding */
  size?: 'sm' | 'md' | 'lg';
  /** Active state (shows different style/icon) */
  active?: boolean;
  /** Icon to show when active */
  activeIcon?: LucideIcon;
  /** Loading state (shows spinner) */
  loading?: boolean;
  /** Disabled state */
  disabled?: boolean;
  /** Tooltip text */
  title?: string;
  /** Additional CSS classes */
  className?: string;
}

export const IconButton = memo(function IconButton({
  icon: Icon,
  onClick,
  variant = 'ghost',
  size = 'lg',
  active = false,
  activeIcon: ActiveIcon,
  loading = false,
  disabled = false,
  title,
  className,
}: IconButtonProps) {
  const sizeStyles = {
    sm: 'p-1.5',
    md: 'p-2',
    lg: 'p-2.5',
  };

  const getVariantStyles = () => {
    switch (variant) {
      case 'ghost':
        return active
          ? 'text-emerald-400 bg-emerald-500/10 border-emerald-500/30'
          : 'text-white/50 hover:text-white/70 hover:bg-white/5 border-white/10';

      case 'success':
        if (loading) {
          return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30 animate-pulse';
        }
        if (disabled) {
          return 'bg-white/5 text-white/40 border-white/10';
        }
        return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30 hover:bg-emerald-500/30 hover:scale-105';

      case 'danger':
        if (disabled) {
          return 'text-white/40 border-transparent';
        }
        return 'text-white/50 hover:text-red-400 hover:bg-red-500/10 border-white/10 hover:border-red-500/30';

      case 'primary':
        if (disabled) {
          return 'bg-white/5 text-white/40 border-white/10';
        }
        return 'bg-novanet-500/20 text-novanet-400 border-novanet-500/30 hover:bg-novanet-500/30';

      default:
        return '';
    }
  };

  const DisplayIcon = loading ? Loader2 : active && ActiveIcon ? ActiveIcon : Icon;

  return (
    <button
      onClick={onClick}
      disabled={disabled || loading}
      title={title}
      className={cn(
        'rounded-lg transition-all border',
        sizeStyles[size],
        getVariantStyles(),
        'disabled:cursor-not-allowed',
        className
      )}
    >
      <DisplayIcon className={cn('w-4 h-4', loading && 'animate-spin')} />
    </button>
  );
});
