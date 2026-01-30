'use client';

/**
 * Kbd - Keyboard shortcut badge component
 *
 * Used in modal footers to display keyboard hints.
 * Matches glass design system styling.
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';

interface KbdProps {
  children: React.ReactNode;
  className?: string;
}

export const Kbd = memo(function Kbd({ children, className }: KbdProps) {
  return (
    <kbd
      className={cn(
        'px-1.5 py-0.5 bg-white/10 rounded text-white/50 font-mono text-xs',
        className
      )}
    >
      {children}
    </kbd>
  );
});
