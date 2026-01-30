'use client';

/**
 * useEscapeKey - Handle Escape key press to close modals/panels
 *
 * Adds and removes event listener for Escape key.
 * Only triggers when enabled is true.
 *
 * Uses useLatestRef to avoid re-registering the listener when callback changes.
 */

import { useEffect } from 'react';
import { useLatestRef } from './useLatestRef';

/**
 * Call a callback when Escape key is pressed
 *
 * @param callback - Function to call on Escape
 * @param enabled - Whether the listener is active (default: true)
 *
 * @example
 * useEscapeKey(onClose, isOpen);
 */
export function useEscapeKey(
  callback: () => void,
  enabled: boolean = true
): void {
  // Use ref to avoid re-registering listener when callback changes
  const callbackRef = useLatestRef(callback);

  useEffect(() => {
    if (!enabled) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        callbackRef.current();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [enabled, callbackRef]);
}
