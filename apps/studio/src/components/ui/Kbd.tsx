'use client';

/**
 * Kbd - Keyboard shortcut badge component (Skeuomorphic)
 *
 * Used in modal footers to display keyboard hints.
 * Dark skeuomorphic design with 3D depth.
 *
 * For more control, use KeyboardKey component instead.
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
        // Layout
        'inline-flex items-center justify-center',
        'px-1.5 py-0.5 min-w-[20px] h-5',
        'font-mono text-[10px] font-medium tracking-tight',
        // Skeuomorphic dark styling
        // opacity.bg.heavy (0.10) + opacity.bg.light (0.04)
        'bg-gradient-to-b from-white/[0.10] to-white/[0.04]',
        // opacity.border.medium (0.12) + opacity.border.light (0.08)
        'border border-white/[0.12] border-b-white/[0.08]',
        // opacity.text.muted (0.60)
        'rounded text-white/60',
        // 3D depth shadows
        'shadow-[0_1px_2px_rgba(0,0,0,0.3),inset_0_1px_0_rgba(255,255,255,0.06)]',
        className
      )}
    >
      {children}
    </kbd>
  );
});
