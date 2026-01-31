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
        // Layout - horizontal with icon left, kbd right
        'group relative flex items-center',
        'h-[52px] w-full rounded-xl',
        'gap-3 px-4',
        // Frosted Glass base - backdrop-blur-md matches glass.blur.md (12px)
        'backdrop-blur-md',
        'ring-1 ring-inset',
        // Transitions
        'transition-all duration-200',
        // Focus - visible ring
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/60 focus-visible:ring-offset-1 focus-visible:ring-offset-black/50',
        // Click animation
        isClicking && 'scale-[0.97] ring-novanet-500/40',
        // States with glassmorphism
        isActive
          ? [
              'bg-white/[0.1]',
              'ring-white/[0.15]',
              'shadow-lg shadow-black/25',
              'shadow-[inset_0_1px_0_rgba(255,255,255,0.1)]',
              'text-white',
            ]
          : [
              'bg-white/[0.04]',
              'ring-white/[0.06]',
              'shadow-md shadow-black/10',
              'text-white/60',
              'hover:bg-white/[0.08]',
              'hover:ring-white/[0.12]',
              'hover:text-white/90',
              'hover:shadow-lg',
              'active:scale-[0.97]',
            ]
      )}
    >
      {/* Icon - left with hover/click animation */}
      <div
        className={cn(
          'flex-shrink-0 flex items-center justify-center',
          'w-8 h-8 rounded-lg',
          'transition-all duration-300',
          isActive
            ? 'bg-white/[0.1]'
            : 'bg-white/[0.04] group-hover:bg-white/[0.08]',
          // Click pulse effect
          isClicking && 'bg-novanet-500/20'
        )}
      >
        <Icon
          className={cn(
            'w-4 h-4',
            'transition-all duration-300',
            // SVG animation on hover
            'group-hover:scale-110',
            // Click animation - rotate + scale
            isClicking && 'scale-125 rotate-12 text-novanet-400',
            isActive
              ? 'opacity-100 text-novanet-400'
              : 'opacity-60 group-hover:opacity-100'
          )}
          strokeWidth={1.75}
        />
      </div>

      {/* Label - center flex-grow */}
      <span
        className={cn(
          'flex-1 text-left',
          'text-[12px] font-medium leading-tight truncate',
          'transition-all duration-200'
        )}
      >
        {displayName}
      </span>

      {/* Keyboard shortcut - right */}
      {shortcut && (
        <kbd
          className={cn(
            'flex-shrink-0',
            'text-[10px] min-w-[24px] px-2 py-1 rounded-md',
            'font-mono text-center tabular-nums',
            'transition-all duration-200',
            isActive
              ? 'bg-white/[0.12] text-white/70 ring-1 ring-inset ring-white/[0.1]'
              : 'bg-white/[0.05] text-white/35 group-hover:bg-white/[0.08] group-hover:text-white/50'
          )}
        >
          {shortcut}
        </kbd>
      )}
    </button>
  );
});
