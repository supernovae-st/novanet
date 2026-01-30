'use client';

/**
 * useModalAutoFocus - Auto-focus an element when modal opens
 *
 * Focuses a referenced element after a short delay when the modal opens.
 * Also calls a reset callback to initialize modal state.
 */

import { useEffect, type RefObject } from 'react';
import { useLatestRef } from './useLatestRef';

interface UseModalAutoFocusOptions {
  /** Delay before focusing (ms) */
  delay?: number;
  /** Callback to reset modal state on open */
  onReset?: () => void;
}

/**
 * Auto-focus an element when modal opens
 *
 * @param ref - Reference to the element to focus
 * @param isOpen - Whether the modal is open
 * @param options - Configuration options
 *
 * @example
 * const searchRef = useRef<HTMLInputElement>(null);
 * useModalAutoFocus(searchRef, isOpen, {
 *   delay: 50,
 *   onReset: () => {
 *     setSearch('');
 *     setFocusedIndex(0);
 *   }
 * });
 */
export function useModalAutoFocus<T extends HTMLElement>(
  ref: RefObject<T | null>,
  isOpen: boolean,
  options: UseModalAutoFocusOptions = {}
): void {
  const { delay = 50, onReset } = options;

  // Use ref to avoid stale closure - callback always gets latest version
  const onResetRef = useLatestRef(onReset);

  useEffect(() => {
    if (!isOpen) return;

    const timer = setTimeout(() => ref.current?.focus(), delay);
    // Use ref.current to always get latest callback
    onResetRef.current?.();

    return () => clearTimeout(timer);
  }, [isOpen, delay, ref, onResetRef]);
}
