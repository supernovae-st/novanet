'use client';

/**
 * KeyboardKey - Styled keyboard shortcut display
 *
 * Design: Linear-dark style for <kbd> elements
 * Extracted from CommandPalette, KeyboardShortcuts, AiChat, LocalePicker
 */

import { memo, type ReactNode } from 'react';
import { cn } from '@/lib/utils';

export interface KeyboardKeyProps {
  /** Key label or children */
  children: ReactNode;
  /** Size variant */
  size?: 'sm' | 'md' | 'lg';
  /** Additional className */
  className?: string;
}

const sizeStyles = {
  sm: 'px-1 py-0.5 text-[10px] min-w-[18px]',
  md: 'px-1.5 py-0.5 text-[11px] min-w-[22px]',
  lg: 'px-2 py-1 text-xs min-w-[26px]',
};

/**
 * Keyboard key display component
 *
 * @example
 * <KeyboardKey>⌘</KeyboardKey>
 * <KeyboardKey>K</KeyboardKey>
 *
 * // With size
 * <KeyboardKey size="sm">Esc</KeyboardKey>
 */
export const KeyboardKey = memo(function KeyboardKey({
  children,
  size = 'md',
  className,
}: KeyboardKeyProps) {
  return (
    <kbd
      className={cn(
        'inline-flex items-center justify-center',
        'font-mono font-medium',
        'bg-white/[0.08] border border-white/[0.12]',
        'rounded text-white/50',
        'shadow-[0_1px_0_0_rgba(255,255,255,0.06)]',
        sizeStyles[size],
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
