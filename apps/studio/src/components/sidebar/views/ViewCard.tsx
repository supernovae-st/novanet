'use client';

/**
 * ViewCard - Individual view card with horizontal layout
 *
 * Design: Icon left | Label center | Shortcut right
 * Features: Frosted glass, hover animations, click pulse, SVG transforms
 */

import { memo, useState, useCallback } from 'react';
import { type LucideIcon } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes, gapTokens } from '@/design/tokens';

interface ViewCardProps {
  id: string;
  name: string;
  description?: string;
  shortcut?: string;
  icon: LucideIcon;
  isActive: boolean;
  onClick: () => void;
}

export const ViewCard = memo(function ViewCard({
  name,
  shortcut,
  icon: Icon,
  isActive,
  onClick,
}: ViewCardProps) {
  // Extract first word for compact display
  const displayName = name.split(' ')[0] || name;

  // Click animation state
  const [isClicking, setIsClicking] = useState(false);

  const handleClick = useCallback(() => {
    setIsClicking(true);
    onClick();
    // Reset after animation
    setTimeout(() => setIsClicking(false), 400);
  }, [onClick]);

  return (
    <button
      onClick={handleClick}
      aria-pressed={isActive}
      aria-label={name}
      className={cn(
        // Layout - matches FilterCard (48px height for WCAG)
        'group relative flex items-center',
        'h-12 w-full rounded-xl',
        gapTokens.spacious, 'px-3.5',
        // Frosted Glass base - matches FilterCard
        'backdrop-blur-sm',
        'ring-1 ring-inset',
        // Transitions
        'transition-all duration-200',
        // Focus - matches FilterCard
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/60 focus-visible:ring-offset-1 focus-visible:ring-offset-black/50',
        // Click animation
        isClicking && 'scale-[0.97]',
        // States - matches FilterCard selected/unselected
        isActive
          ? [
              'bg-white/[0.08]',
              'ring-white/[0.12]',
              'shadow-lg shadow-black/20',
              'text-white',
            ]
          : [
              'bg-white/[0.03]',
              'ring-white/[0.06]',
              'shadow-sm shadow-black/10',
              'text-white/70',
              'hover:bg-white/[0.06]',
              'hover:ring-white/[0.10]',
              'hover:text-white/90',
              'active:scale-[0.97]',
            ]
      )}
    >
      {/* Icon - matches FilterCard icon container */}
      <div
        className={cn(
          'flex-shrink-0 flex items-center justify-center',
          'w-8 h-8 rounded-lg',
          'transition-all duration-200',
          isActive
            ? 'bg-white/[0.10]'
            : 'bg-white/[0.05] group-hover:bg-white/[0.08]',
          isClicking && 'bg-white/[0.15]'
        )}
      >
        <Icon
          className={cn(
            iconSizes.md,
            'transition-all duration-200',
            isClicking && 'scale-110',
            isActive
              ? 'text-novanet-400'
              : 'text-white/60 group-hover:text-white/80'
          )}
          strokeWidth={1.75}
        />
      </div>

      {/* Label - matches FilterCard label */}
      <span
        className={cn(
          'flex-1 text-left truncate',
          'text-[13px] font-medium leading-tight',
          'transition-colors duration-200',
          isActive ? 'text-white' : 'text-white/70 group-hover:text-white/90'
        )}
      >
        {displayName}
      </span>

      {/* Keyboard shortcut - pill style like FilterCard count */}
      {shortcut && (
        <span
          className={cn(
            'flex-shrink-0',
            'min-w-[32px] px-2.5 py-1 rounded-full',
            'text-[11px] font-semibold text-center tabular-nums',
            'transition-all duration-200',
            isActive
              ? 'bg-white/[0.12] text-white/90'
              : 'bg-white/[0.05] text-white/50 group-hover:bg-white/[0.08] group-hover:text-white/70'
          )}
        >
          {shortcut}
        </span>
      )}
    </button>
  );
});
