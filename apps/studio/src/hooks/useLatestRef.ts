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

import { useRef, useEffect, type MutableRefObject } from 'react';

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
