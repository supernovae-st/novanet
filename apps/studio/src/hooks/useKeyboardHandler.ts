'use client';

/**
 * useKeyboardHandler - Unified keyboard event handling
 *
 * Provides a declarative way to handle keyboard shortcuts with:
 * - Automatic input element filtering
 * - Modifier key support (Shift, Ctrl, Meta, Alt)
 * - Enabled/disabled state
 * - Automatic cleanup
 *
 * @example
 * useKeyboardHandler([
 *   { key: 'Tab', handler: handleTab, preventDefault: true },
 *   { key: 'Enter', handler: handleEnter, preventDefault: true },
 *   { key: 'Escape', handler: handleEscape },
 *   { key: 'Tab', modifiers: { shift: true }, handler: handleShiftTab },
 * ], { enabled: isActive, skipInputs: true });
 */

import { useEffect, useRef, useCallback } from 'react';

export interface KeyHandler {
  /** The key to listen for (e.g., 'Tab', 'Enter', 'Escape', 'a', '1') */
  key: string;
  /** Handler function called when key is pressed */
  handler: (event: KeyboardEvent) => void;
  /** Whether to call preventDefault() (default: false) */
  preventDefault?: boolean;
  /** Whether to call stopPropagation() (default: false) */
  stopPropagation?: boolean;
  /** Required modifier keys */
  modifiers?: {
    shift?: boolean;
    ctrl?: boolean;
    meta?: boolean;
    alt?: boolean;
  };
}

export interface UseKeyboardHandlerOptions {
  /** Whether keyboard handling is enabled (default: true) */
  enabled?: boolean;
  /** Skip handling when focus is in input/textarea/contenteditable (default: true) */
  skipInputs?: boolean;
  /** Use capture phase instead of bubble (default: false) */
  capture?: boolean;
}

/**
 * Check if the event target is an input element
 */
function isInputElement(target: EventTarget | null): boolean {
  if (!target || !(target instanceof HTMLElement)) return false;
  return (
    target.tagName === 'INPUT' ||
    target.tagName === 'TEXTAREA' ||
    target.isContentEditable
  );
}

/**
 * Check if all required modifiers match
 */
function modifiersMatch(
  event: KeyboardEvent,
  modifiers?: KeyHandler['modifiers']
): boolean {
  if (!modifiers) return true;

  if (modifiers.shift !== undefined && event.shiftKey !== modifiers.shift) {
    return false;
  }
  if (modifiers.ctrl !== undefined && event.ctrlKey !== modifiers.ctrl) {
    return false;
  }
  if (modifiers.meta !== undefined && event.metaKey !== modifiers.meta) {
    return false;
  }
  if (modifiers.alt !== undefined && event.altKey !== modifiers.alt) {
    return false;
  }

  return true;
}

/**
 * Hook for handling keyboard events declaratively
 */
export function useKeyboardHandler(
  handlers: KeyHandler[],
  options: UseKeyboardHandlerOptions = {}
): void {
  const { enabled = true, skipInputs = true, capture = false } = options;

  // Store handlers in ref to avoid re-registering listeners
  const handlersRef = useRef(handlers);
  handlersRef.current = handlers;

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      // Skip if in input element
      if (skipInputs && isInputElement(event.target)) {
        return;
      }

      // Find matching handler
      for (const handler of handlersRef.current) {
        // Check key match (case-insensitive for single letters)
        const keyMatches =
          event.key === handler.key ||
          event.key.toLowerCase() === handler.key.toLowerCase();

        if (!keyMatches) continue;

        // Check modifiers
        if (!modifiersMatch(event, handler.modifiers)) continue;

        // Handle the event
        if (handler.preventDefault) {
          event.preventDefault();
        }
        if (handler.stopPropagation) {
          event.stopPropagation();
        }

        handler.handler(event);
        return; // Only handle first match
      }
    },
    [skipInputs]
  );

  useEffect(() => {
    if (!enabled) return;

    window.addEventListener('keydown', handleKeyDown, capture);
    return () => window.removeEventListener('keydown', handleKeyDown, capture);
  }, [enabled, handleKeyDown, capture]);
}

/**
 * Convenience hook for single key handler
 */
export function useKeyPress(
  key: string,
  handler: (event: KeyboardEvent) => void,
  options: UseKeyboardHandlerOptions & {
    preventDefault?: boolean;
    modifiers?: KeyHandler['modifiers'];
  } = {}
): void {
  const { preventDefault = false, modifiers, ...hookOptions } = options;

  useKeyboardHandler(
    [{ key, handler, preventDefault, modifiers }],
    hookOptions
  );
}
