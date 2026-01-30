'use client';

/**
 * useLatestRef - Always-current ref without triggering re-renders
 *
 * Use this when you need to access the latest value in callbacks
 * without adding it to dependency arrays (which would cause re-registration).
 *
 * Common use cases:
 * - Event handlers that need current state
 * - Callbacks passed to external libraries
 * - Avoiding stale closures in useEffect
 *
 * @example
 * const countRef = useLatestRef(count);
 *
 * useEffect(() => {
 *   const handler = () => {
 *     // Always has latest count, even without count in deps
 *     console.log(countRef.current);
 *   };
 *   window.addEventListener('click', handler);
 *   return () => window.removeEventListener('click', handler);
 * }, []); // No need to add count to deps
 */

import { useRef, useEffect, useCallback, type MutableRefObject } from 'react';

/**
 * Returns a ref that always contains the latest value
 */
export function useLatestRef<T>(value: T): MutableRefObject<T> {
  const ref = useRef(value);

  useEffect(() => {
    ref.current = value;
  }, [value]);

  return ref;
}

/**
 * Returns a callback that always calls the latest version of the function
 * Useful for event handlers that shouldn't trigger effect re-runs
 *
 * Uses useCallback with empty deps for stable identity, while useLatestRef
 * ensures the callback always invokes the most recent function version.
 */
export function useLatestCallback<T extends (...args: unknown[]) => unknown>(
  callback: T
): T {
  const ref = useLatestRef(callback);

  // useCallback with empty deps provides stable identity
  // ref.current always has the latest callback
  return useCallback(
    ((...args: Parameters<T>) => ref.current(...args)) as T,
    // eslint-disable-next-line react-hooks/exhaustive-deps -- ref is stable
    []
  );
}
