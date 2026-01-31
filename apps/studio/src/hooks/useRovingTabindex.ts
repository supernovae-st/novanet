import {
  createContext,
  useContext,
  useCallback,
  useMemo,
  useEffect,
  useRef,
  useState,
  type RefObject,
  type KeyboardEvent,
} from 'react';

/**
 * Roving Tabindex Context
 *
 * Implements WAI-ARIA TreeView keyboard navigation pattern:
 * - Only one item is tabbable at a time (tabindex=0)
 * - Arrow keys move focus between items
 * - Home/End jump to first/last item
 */

export interface RovingTabindexContextValue {
  /** Currently focused item ID */
  focusedId: string | null;
  /** Set focused item */
  setFocusedId: (id: string | null) => void;
  /** Register an item for navigation */
  registerItem: (id: string, element: HTMLElement) => void;
  /** Unregister an item */
  unregisterItem: (id: string) => void;
  /** Get ordered list of item IDs */
  getItemIds: () => string[];
  /** Focus next item */
  focusNext: () => void;
  /** Focus previous item */
  focusPrev: () => void;
  /** Focus first item */
  focusFirst: () => void;
  /** Focus last item */
  focusLast: () => void;
}

const RovingTabindexContext = createContext<RovingTabindexContextValue | null>(null);

export const RovingTabindexProvider = RovingTabindexContext.Provider;

/**
 * Hook to create roving tabindex container
 *
 * @returns Context value to pass to RovingTabindexProvider
 */
export function useRovingTabindexRoot(): RovingTabindexContextValue {
  // Map of id -> element for focus management (stable ref)
  const itemsRef = useRef<Map<string, HTMLElement>>(new Map());
  // Ordered list of IDs (stable ref, updated on register/unregister)
  const orderedIdsRef = useRef<string[]>([]);
  // Currently focused ID - useState for reactivity
  const [focusedId, setFocusedIdState] = useState<string | null>(null);

  // Update ordered IDs based on DOM position
  const updateOrderedIds = useCallback(() => {
    const entries = Array.from(itemsRef.current.entries());
    if (entries.length === 0) {
      orderedIdsRef.current = [];
      return;
    }
    entries.sort(([, a], [, b]) => {
      const position = a.compareDocumentPosition(b);
      if (position & Node.DOCUMENT_POSITION_FOLLOWING) return -1;
      if (position & Node.DOCUMENT_POSITION_PRECEDING) return 1;
      return 0;
    });
    orderedIdsRef.current = entries.map(([id]) => id);
  }, []);

  const registerItem = useCallback((id: string, element: HTMLElement) => {
    itemsRef.current.set(id, element);
    updateOrderedIds();
  }, [updateOrderedIds]);

  const unregisterItem = useCallback((id: string) => {
    itemsRef.current.delete(id);
    updateOrderedIds();
    // If focused item was removed, clear focus
    setFocusedIdState((current) => (current === id ? null : current));
  }, [updateOrderedIds]);

  const getItemIds = useCallback(() => orderedIdsRef.current, []);

  // Set focused ID and move DOM focus
  const setFocusedId = useCallback((id: string | null) => {
    setFocusedIdState(id);
    if (id) {
      const element = itemsRef.current.get(id);
      element?.focus();
    }
  }, []);

  const focusNext = useCallback(() => {
    const ids = orderedIdsRef.current;
    if (ids.length === 0) return;

    setFocusedIdState((current) => {
      const currentIndex = current ? ids.indexOf(current) : -1;
      const nextIndex = currentIndex < ids.length - 1 ? currentIndex + 1 : 0;
      const nextId = ids[nextIndex];
      // Move DOM focus
      const element = itemsRef.current.get(nextId);
      element?.focus();
      return nextId;
    });
  }, []);

  const focusPrev = useCallback(() => {
    const ids = orderedIdsRef.current;
    if (ids.length === 0) return;

    setFocusedIdState((current) => {
      const currentIndex = current ? ids.indexOf(current) : ids.length;
      const prevIndex = currentIndex > 0 ? currentIndex - 1 : ids.length - 1;
      const prevId = ids[prevIndex];
      // Move DOM focus
      const element = itemsRef.current.get(prevId);
      element?.focus();
      return prevId;
    });
  }, []);

  const focusFirst = useCallback(() => {
    const ids = orderedIdsRef.current;
    if (ids.length > 0) {
      const firstId = ids[0];
      setFocusedIdState(firstId);
      const element = itemsRef.current.get(firstId);
      element?.focus();
    }
  }, []);

  const focusLast = useCallback(() => {
    const ids = orderedIdsRef.current;
    if (ids.length > 0) {
      const lastId = ids[ids.length - 1];
      setFocusedIdState(lastId);
      const element = itemsRef.current.get(lastId);
      element?.focus();
    }
  }, []);

  // Memoize context value to prevent unnecessary consumer re-renders
  // Only recreates when focusedId changes (other functions are stable)
  return useMemo(
    () => ({
      focusedId,
      setFocusedId,
      registerItem,
      unregisterItem,
      getItemIds,
      focusNext,
      focusPrev,
      focusFirst,
      focusLast,
    }),
    [
      focusedId,
      setFocusedId,
      registerItem,
      unregisterItem,
      getItemIds,
      focusNext,
      focusPrev,
      focusFirst,
      focusLast,
    ]
  );
}

/**
 * Hook for individual roving tabindex items
 *
 * @param id - Unique identifier for this item
 * @param elementRef - Ref to the focusable element
 * @returns { tabIndex, isFocused } - Props to spread on the element
 */
export function useRovingTabindexItem<T extends HTMLElement>(
  id: string,
  elementRef: RefObject<T | null>
): { tabIndex: number; isFocused: boolean } {
  const context = useContext(RovingTabindexContext);

  // Register/unregister on mount/unmount
  useEffect(() => {
    if (!context) return;
    const element = elementRef.current;
    if (element) {
      context.registerItem(id, element);
      return () => context.unregisterItem(id);
    }
  }, [id, elementRef, context]);

  if (!context) {
    // Fallback when not in roving context - all items tabbable
    return { tabIndex: 0, isFocused: false };
  }

  const { focusedId, getItemIds } = context;
  const isFocused = focusedId === id;

  // First item is tabbable if nothing focused, otherwise only focused item
  const ids = getItemIds();
  const isFirstItem = ids.length > 0 && ids[0] === id;
  const tabIndex = isFocused || (focusedId === null && isFirstItem) ? 0 : -1;

  return { tabIndex, isFocused };
}

/**
 * Keyboard handler for roving tabindex container
 *
 * @param context - Roving tabindex context
 * @returns onKeyDown handler
 */
export function useRovingKeyboardHandler(
  context: RovingTabindexContextValue | null
): (e: KeyboardEvent) => void {
  return useCallback(
    (e: KeyboardEvent) => {
      if (!context) return;

      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault();
          context.focusNext();
          break;
        case 'ArrowUp':
          e.preventDefault();
          context.focusPrev();
          break;
        case 'Home':
          e.preventDefault();
          context.focusFirst();
          break;
        case 'End':
          e.preventDefault();
          context.focusLast();
          break;
      }
    },
    [context]
  );
}
