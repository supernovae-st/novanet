'use client';

/**
 * KeyboardKey - Skeuomorphic keyboard shortcut display
 *
 * Design: Dark skeuomorphic style with depth, shadows, and subtle gradients
 * Inspired by macOS keyboard keys with glass morphism accents
 *
 * Features:
 * - 3D depth with inner/outer shadows
 * - Subtle top highlight for light reflection
 * - Hover/active states with press feedback
 * - Three sizes: sm, md, lg
 */

import { memo, type ReactNode } from 'react';
import { cn } from '@/lib/utils';

export interface KeyboardKeyProps {
  /** Key label or children */
  children: ReactNode;
  /** Size variant */
  size?: 'sm' | 'md' | 'lg';
  /** Interactive mode - adds hover/active states */
  interactive?: boolean;
  /** Additional className */
  className?: string;
}

const sizeStyles = {
  sm: 'px-1.5 py-0.5 text-[10px] min-w-[20px] h-5 rounded',
  md: 'px-2 py-0.5 text-[11px] min-w-[24px] h-6 rounded-md',
  lg: 'px-2.5 py-1 text-xs min-w-[28px] h-7 rounded-md',
};

/**
 * Keyboard key display component - Skeuomorphic dark design
 *
 * @example
 * <KeyboardKey>⌘</KeyboardKey>
 * <KeyboardKey>K</KeyboardKey>
 *
 * // With size
 * <KeyboardKey size="sm">Esc</KeyboardKey>
 *
 * // Interactive (for clickable shortcuts)
 * <KeyboardKey interactive>⇧H</KeyboardKey>
 */
export const KeyboardKey = memo(function KeyboardKey({
  children,
  size = 'md',
  interactive = false,
  className,
}: KeyboardKeyProps) {
  return (
    <kbd
      className={cn(
        // Layout
        'inline-flex items-center justify-center',
        'font-mono font-medium tracking-tight',
        // Skeuomorphic dark styling
        // opacity.bg.heavy (0.10) + opacity.bg.light (0.04)
        'bg-gradient-to-b from-white/[0.10] to-white/[0.04]',
        // opacity.border.medium (0.12) + opacity.border.light (0.08)
        'border border-white/[0.12] border-b-white/[0.08]',
        // opacity.text.secondary (0.70)
        'text-white/70',
        // 3D depth shadows
        'shadow-[0_1px_2px_rgba(0,0,0,0.4),0_0_0_1px_rgba(0,0,0,0.2),inset_0_1px_0_rgba(255,255,255,0.08)]',
        // Size
        sizeStyles[size],
        // Interactive states
        interactive && [
          'cursor-pointer',
          'transition-all duration-100',
          // opacity.bg.intense (0.15) + opacity.bg.medium (0.06)
          'hover:bg-gradient-to-b hover:from-white/[0.15] hover:to-white/[0.06]',
          // opacity.text.strong (0.90) + opacity.border.strong (0.15)
          'hover:text-white/90 hover:border-white/[0.15]',
          // opacity.bg.medium (0.06) + opacity.bg.heavy (0.10)
          'active:bg-gradient-to-b active:from-white/[0.06] active:to-white/[0.10]',
          'active:shadow-[0_0_1px_rgba(0,0,0,0.4),inset_0_1px_2px_rgba(0,0,0,0.2)]',
          'active:translate-y-[1px]',
        ],
        className
      )}
    >
      {children}
    </kbd>
  );
});

/**
 * Keyboard shortcut group (e.g., ⌘ + K)
 */
export interface KeyboardShortcutProps {
  /** Array of keys to display */
  keys: string[];
  /** Size variant */
  size?: 'sm' | 'md' | 'lg';
  /** Additional className for container */
  className?: string;
}

export const KeyboardShortcut = memo(function KeyboardShortcut({
  keys,
  size = 'md',
  className,
}: KeyboardShortcutProps) {
  return (
    <span className={cn('inline-flex items-center gap-1', className)}>
      {keys.map((key, index) => (
        <KeyboardKey key={index} size={size}>
          {key}
        </KeyboardKey>
      ))}
    </span>
  );
});
