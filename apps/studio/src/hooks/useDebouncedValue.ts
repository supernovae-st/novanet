import { useState, useEffect } from 'react';

/**
 * Debounces a value by the specified delay.
 * Returns the debounced value that only updates after the delay has passed.
 *
 * @param value - The value to debounce
 * @param delay - Delay in milliseconds (default: 150ms)
 * @returns The debounced value
 *
 * @example
 * const [query, setQuery] = useState('');
 * const debouncedQuery = useDebouncedValue(query, 150);
 * // debouncedQuery updates 150ms after query stops changing
 */
export function useDebouncedValue<T>(value: T, delay = 150): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => clearTimeout(timeoutId);
  }, [value, delay]);

  return debouncedValue;
}
