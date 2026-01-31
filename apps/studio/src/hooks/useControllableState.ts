import { useState, useCallback, useRef, useEffect } from 'react';

/**
 * useControllableState - Hybrid controlled/uncontrolled state pattern
 *
 * Follows React's input pattern:
 * - If controlledValue is provided → controlled mode (parent manages state)
 * - If controlledValue is undefined → uncontrolled mode (internal state)
 *
 * @example
 * // Uncontrolled (internal state)
 * const [isOpen, setIsOpen] = useControllableState(undefined, false);
 *
 * // Controlled (parent manages)
 * const [isOpen, setIsOpen] = useControllableState(props.isOpen, false, props.onOpenChange);
 */
export function useControllableState<T>(
  controlledValue: T | undefined,
  defaultValue: T,
  onChange?: (value: T) => void
): [T, (value: T | ((prev: T) => T)) => void] {
  // Track if we're in controlled mode
  const isControlled = controlledValue !== undefined;
  const isControlledRef = useRef(isControlled);

  // Track controlled/uncontrolled mode (switching is unsupported)
  useEffect(() => {
    isControlledRef.current = isControlled;
  }, [isControlled]);

  // Internal state for uncontrolled mode
  const [internalValue, setInternalValue] = useState<T>(defaultValue);

  // Current value depends on mode
  const value = isControlled ? controlledValue : internalValue;

  // Setter that handles both modes
  const setValue = useCallback(
    (nextValue: T | ((prev: T) => T)) => {
      const resolvedValue =
        typeof nextValue === 'function'
          ? (nextValue as (prev: T) => T)(value as T)
          : nextValue;

      // In uncontrolled mode, update internal state
      if (!isControlled) {
        setInternalValue(resolvedValue);
      }

      // Always call onChange if provided
      onChange?.(resolvedValue);
    },
    [isControlled, onChange, value]
  );

  return [value as T, setValue];
}
