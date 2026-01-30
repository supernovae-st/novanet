'use client';

/**
 * useOutsideClick - Handle clicks outside a referenced element
 *
 * Calls the provided callback when a click occurs outside the referenced element.
 * Useful for closing modals, dropdowns, and popovers.
 *
 * Uses useLatestRef pattern to avoid re-registering the event listener
 * when the callback changes, preventing memory leaks and stale closures.
 */

import { useEffect, type RefObject } from 'react';
import { useLatestRef } from './useLatestRef';

/**
 * Handle clicks outside a referenced element
 *
 * @param ref - Reference to the element to detect outside clicks
 * @param callback - Function to call when clicking outside
 * @param enabled - Whether the hook is active (default: true)
 *
 * @example
 * const containerRef = useRef<HTMLDivElement>(null);
 * useOutsideClick(containerRef, onClose, isOpen);
 */
export function useOutsideClick<T extends HTMLElement>(
  ref: RefObject<T | null>,
  callback: () => void,
  enabled: boolean = true
): void {
  // Use ref pattern to avoid re-registering listener when callback changes
  const callbackRef = useLatestRef(callback);

  useEffect(() => {
    if (!enabled) return;

    const handleClick = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        callbackRef.current();
      }
    };

    document.addEventListener('mousedown', handleClick);
    return () => document.removeEventListener('mousedown', handleClick);
  }, [ref, enabled, callbackRef]);
}
