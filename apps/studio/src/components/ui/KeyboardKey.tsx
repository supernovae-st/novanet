'use client';

/**
 * KeyboardKey - Premium skeuomorphic keyboard key
 *
 * Hyper-realistic dark keyboard key with multi-layer depth:
 * - Raised key cap with concave surface gradient
 * - 4-layer shadow stack (ambient + drop + ring + inner highlight)
 * - Bright top-edge catchlight (light reflecting off key lip)
 * - Dark bottom bevel (key sits in a well)
 * - Active state: key physically presses down (translate + shadow inversion)
 * - Three sizes: sm, md, lg
 */

import { memo, type ReactNode } from 'react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';

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
  sm: 'px-1.5 py-[3px] text-[10px] min-w-[20px] h-5 rounded-[4px]',
  md: 'px-2 py-1 text-[11px] min-w-[24px] h-6 rounded-[5px]',
  lg: 'px-2.5 py-1 text-xs min-w-[28px] h-7 rounded-[6px]',
};

// Multi-layer shadow for realistic key depth (CSS values, applied via style prop):
// 1. Ambient shadow (soft spread underneath)
// 2. Drop shadow (hard edge, key is raised)
// 3. Dark ring (key well / bezel)
// 4. Inner top highlight (catchlight from above)
// 5. Inner bottom shadow (concave surface)
const KEY_SHADOW = [
  '0 2px 4px rgba(0,0,0,0.5)',
  '0 1px 1px rgba(0,0,0,0.6)',
  '0 0 0 1px rgba(0,0,0,0.35)',
  'inset 0 1px 0 rgba(255,255,255,0.12)',
  'inset 0 -1px 0 rgba(0,0,0,0.15)',
].join(', ');

/**
 * Keyboard key display component - Premium skeuomorphic dark design
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
      style={{ boxShadow: KEY_SHADOW }}
      className={cn(
        // Layout
        'inline-flex items-center justify-center',
        'font-mono font-semibold tracking-tight leading-none',
        // Key cap surface: concave gradient (lighter top → darker bottom)
        'bg-gradient-to-b from-white/[0.14] via-white/[0.07] to-white/[0.03]',
        // Border: bright top lip, darker sides, dark bottom bevel
        'border border-t-white/[0.18] border-x-white/[0.10] border-b-white/[0.05]',
        // Text: crisp, slightly brighter than surroundings
        'text-white/75',
        // Size
        sizeStyles[size],
        // Interactive states
        interactive && [
          'cursor-pointer',
          'transition-all duration-75 ease-out',
          // Hover: lift up, brighter surface + catchlight
          'hover:from-white/[0.18] hover:via-white/[0.09] hover:to-white/[0.05]',
          'hover:text-white/90',
          'hover:border-t-white/[0.22]',
          'hover:-translate-y-[0.5px]',
          // Active: press down into well (gradient inverts)
          'active:from-white/[0.04] active:via-white/[0.06] active:to-white/[0.08]',
          'active:translate-y-[1px]',
          'active:border-t-white/[0.08]',
          'active:text-white/60',
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
    <span className={cn('inline-flex items-center', gapTokens.tight, className)}>
      {keys.map((key, index) => (
        <KeyboardKey key={index} size={size}>
          {key}
        </KeyboardKey>
      ))}
    </span>
  );
});
