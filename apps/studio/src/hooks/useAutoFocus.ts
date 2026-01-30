'use client';

/**
 * useAutoFocus - Auto-focus an element when condition is true
 *
 * Simpler alternative to useModalAutoFocus when you don't need reset callback.
 * Focuses element after a delay to allow DOM to settle.
 */

import { useEffect, type RefObject } from 'react';
import { FOCUS_DELAY_MS } from '@/config/constants';

/**
 * Auto-focus an element when condition becomes true
 *
 * @param ref - Reference to the element to focus
 * @param condition - When true, focus the element
 * @param delay - Delay before focusing (default: FOCUS_DELAY_MS)
 *
 * @example
 * const inputRef = useRef<HTMLInputElement>(null);
 * useAutoFocus(inputRef, isOpen);
 */
export function useAutoFocus<T extends HTMLElement>(
  ref: RefObject<T | null>,
  condition: boolean,
  delay: number = FOCUS_DELAY_MS
): void {
  useEffect(() => {
    if (!condition) return;

    const timer = setTimeout(() => {
      ref.current?.focus();
    }, delay);

    return () => clearTimeout(timer);
  }, [ref, condition, delay]);
}
