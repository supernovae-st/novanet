'use client';

/**
 * ViewCard - Individual view card with keyboard shortcut badge
 *
 * Displays a single YAML view definition from the registry.
 * Part of the ViewSelector grid layout.
 */

import { memo } from 'react';
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
  description,
  shortcut,
  icon: Icon,
  isActive,
  onClick,
}: ViewCardProps) {
  // Extract first word for compact display
  const displayName = name.split(' ')[0] || name;

  return (
    <button
      onClick={onClick}
      aria-pressed={isActive}
      aria-label={description ? `${name}: ${description}` : name}
      className={cn(
        'flex flex-col items-center gap-1.5 px-2 py-2.5 rounded-lg text-center',
        'transition-all duration-200',
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50',
        isActive
          ? 'bg-white/[0.1] border border-white/[0.15] text-white'
          : 'bg-white/[0.03] border border-transparent text-white/60 hover:bg-white/[0.06] hover:text-white/80'
      )}
    >
      {/* Icon */}
      <Icon
        className={cn(
          'w-5 h-5 transition-transform duration-200',
          isActive && 'scale-110'
        )}
        strokeWidth={2}
      />

      {/* Display name (compact) */}
      <span className="text-[10px] font-medium leading-tight truncate w-full">
        {displayName}
      </span>

      {/* Keyboard shortcut badge */}
      {shortcut && (
        <kbd className="text-[9px] px-1.5 py-0.5 bg-white/[0.08] rounded text-white/40 font-mono">
          {shortcut}
        </kbd>
      )}
    </button>
  );
});
