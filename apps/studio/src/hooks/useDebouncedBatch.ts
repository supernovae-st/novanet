import { useRef, useCallback, useEffect } from 'react';

/**
 * Return type for useDebouncedBatch hook
 */
export interface UseDebouncedBatchReturn<T> {
  /** Add an item to the batch */
  add: (item: T) => void;
  /** Flush the batch immediately, calling the callback with all pending items */
  flush: () => void;
}

/**
 * Batches multiple rapid updates into a single callback invocation.
 * Useful for performance optimization when multiple state updates
 * happen in quick succession (e.g., rapid node selections, filter changes).
 *
 * The batch is automatically flushed after the specified delay since
 * the last item was added. Each new item resets the timer.
 *
 * @param callback - Function to call with all batched items
 * @param delay - Delay in ms before flushing (default: 16ms = one frame at 60fps)
 * @returns Object with `add` and `flush` functions
 *
 * @example
 * const { add, flush } = useDebouncedBatch<string>((nodeIds) => {
 *   console.log('Selected nodes:', nodeIds);
 * }, 50);
 *
 * // Rapid selections only trigger one callback
 * add('node-1');
 * add('node-2');
 * add('node-3');
 * // After 50ms: logs ['node-1', 'node-2', 'node-3']
 *
 * // Force immediate flush
 * add('node-4');
 * flush(); // Immediately logs ['node-4']
 */
export function useDebouncedBatch<T>(
  callback: (items: T[]) => void,
  delay: number = 16 // One frame at 60fps
): UseDebouncedBatchReturn<T> {
  const batchRef = useRef<T[]>([]);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const callbackRef = useRef(callback);

  // Keep callback ref up to date to always call the latest callback
  useEffect(() => {
    callbackRef.current = callback;
  }, [callback]);

  const add = useCallback((item: T) => {
    batchRef.current.push(item);

    // Reset timer on each addition
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    timeoutRef.current = setTimeout(() => {
      if (batchRef.current.length > 0) {
        callbackRef.current([...batchRef.current]);
        batchRef.current = [];
      }
      timeoutRef.current = null;
    }, delay);
  }, [delay]);

  const flush = useCallback(() => {
    // Clear pending timeout
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }

    // Immediately call callback if batch has items
    if (batchRef.current.length > 0) {
      callbackRef.current([...batchRef.current]);
      batchRef.current = [];
    }
  }, []);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  return { add, flush };
}
