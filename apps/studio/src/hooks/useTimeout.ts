'use client';

/**
 * useTimeout - Declarative timeout hook with auto-cleanup
 *
 * Handles setTimeout with automatic cleanup on unmount or dependency changes.
 * Pass null as delay to pause/cancel the timeout.
 */

import { useEffect, useRef, useCallback } from 'react';

/**
 * Execute a callback after a delay with auto-cleanup
 *
 * @param callback - Function to call after delay
 * @param delay - Delay in ms, or null to disable
 *
 * @example
 * // Auto-hide feedback after 2 seconds
 * useTimeout(() => setShowFeedback(false), showFeedback ? 2000 : null);
 */
export function useTimeout(callback: () => void, delay: number | null): void {
  const savedCallback = useRef(callback);

  // Remember the latest callback
  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  // Set up the timeout
  useEffect(() => {
    if (delay === null) return;

    const id = setTimeout(() => savedCallback.current(), delay);
    return () => clearTimeout(id);
  }, [delay]);
}

/**
 * Get a function to trigger a timeout imperatively
 *
 * @returns [triggerTimeout, cancelTimeout]
 *
 * @example
 * const [showFeedback, triggerFeedback, cancelFeedback] = useTimeoutFn(
 *   () => setShowFeedback(false),
 *   2000
 * );
 */
export function useTimeoutFn(
  callback: () => void,
  delay: number
): [() => void, () => void] {
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);
  const savedCallback = useRef(callback);

  // Remember the latest callback
  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  const trigger = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }
    timeoutRef.current = setTimeout(() => savedCallback.current(), delay);
  }, [delay]);

  const cancel = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
  }, []);

  return [trigger, cancel];
}
