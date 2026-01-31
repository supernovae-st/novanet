'use client';

/**
 * IconButton - Unified icon button with variants
 *
 * Consolidates 3+ button patterns into single component:
 * - ghost: Default transparent button
 * - success: Green/emerald action button (with Matrix glow when loading)
 * - danger: Red destructive button
 * - primary: NovaNet brand color button
 */

import { memo } from 'react';
import { LucideIcon, Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes } from '@/design/tokens';

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
          // Matrix-style glow when executing
          return [
            'bg-emerald-500/30 text-emerald-300 border-emerald-400/50',
            'shadow-[0_0_20px_rgba(52,211,153,0.4),inset_0_0_10px_rgba(52,211,153,0.2)]',
            'animate-pulse',
          ];
        }
        if (disabled) {
          return 'bg-white/5 text-white/40 border-white/10';
        }
        return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30 hover:bg-emerald-500/30 hover:scale-105 active:scale-95';

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
        'relative rounded-lg transition duration-200 border',
        sizeStyles[size],
        getVariantStyles(),
        'disabled:cursor-not-allowed',
        className
      )}
    >
      {/* Matrix glow ring when loading success */}
      {loading && variant === 'success' && (
        <span
          className="absolute inset-0 rounded-lg border-2 border-emerald-400/60 animate-ping"
          style={{ animationDuration: '1s' }}
        />
      )}
      <DisplayIcon
        className={cn(
          iconSizes.md,
          'relative z-10',
          loading && 'animate-spin',
          loading && variant === 'success' && 'drop-shadow-[0_0_8px_rgba(52,211,153,0.8)]'
        )}
      />
    </button>
  );
});
