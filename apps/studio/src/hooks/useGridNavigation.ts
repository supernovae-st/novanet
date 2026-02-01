'use client';

/**
 * useGridNavigation - Keyboard navigation for grid layouts
 *
 * Provides arrow key navigation for grids with configurable columns.
 * Handles focus scrolling and selection.
 */

import { useState, useCallback, useEffect, type RefObject } from 'react';

interface UseGridNavigationOptions<T extends HTMLElement = HTMLElement> {
  /** Number of columns in the grid */
  columns: number;
  /** Total number of items in the grid */
  totalItems: number;
  /** Reference to the grid container for scroll management */
  gridRef: RefObject<T | null>;
  /** Callback when an item is selected via Enter key */
  onSelect: (index: number) => void;
  /** Callback when Escape is pressed */
  onEscape: () => void;
  /** Whether navigation is enabled */
  enabled?: boolean;
}

export interface UseGridNavigationReturn {
  /** Currently focused item index */
  focusedIndex: number;
  /** Reset focus to first item */
  resetFocus: () => void;
  /** Keyboard event handler to attach to container */
  handleKeyDown: (e: React.KeyboardEvent) => void;
}

/**
 * Keyboard navigation for grid layouts
 *
 * @example
 * const { focusedIndex, handleKeyDown, resetFocus } = useGridNavigation({
 *   columns: 4,
 *   totalItems: filteredItems.length + 1,
 *   gridRef,
 *   onSelect: (index) => {
 *     if (index === 0) handleSelectAll();
 *     else handleSelectItem(index - 1);
 *   },
 *   onEscape: onClose,
 *   enabled: isOpen,
 * });
 */
export function useGridNavigation<T extends HTMLElement = HTMLElement>({
  columns,
  totalItems,
  gridRef,
  onSelect,
  onEscape,
  enabled = true,
}: UseGridNavigationOptions<T>): UseGridNavigationReturn {
  const [focusedIndex, setFocusedIndex] = useState(0);

  // Reset focus when disabled (modal closed)
  const resetFocus = useCallback(() => {
    setFocusedIndex(0);
  }, []);

  // Handle keyboard navigation
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (!enabled) return;

      switch (e.key) {
        case 'Escape':
          e.preventDefault();
          onEscape();
          break;
        case 'ArrowDown':
          e.preventDefault();
          setFocusedIndex((i) => Math.min(i + columns, totalItems - 1));
          break;
        case 'ArrowUp':
          e.preventDefault();
          setFocusedIndex((i) => Math.max(i - columns, 0));
          break;
        case 'ArrowRight':
          e.preventDefault();
          setFocusedIndex((i) => Math.min(i + 1, totalItems - 1));
          break;
        case 'ArrowLeft':
          e.preventDefault();
          setFocusedIndex((i) => Math.max(i - 1, 0));
          break;
        case 'Enter':
          e.preventDefault();
          onSelect(focusedIndex);
          break;
      }
    },
    [enabled, columns, totalItems, focusedIndex, onSelect, onEscape]
  );

  // Clamp focusedIndex when totalItems decreases (e.g., after filtering)
  // Prevents invalid state where focus points to non-existent item
  useEffect(() => {
    if (totalItems > 0 && focusedIndex >= totalItems) {
      setFocusedIndex(totalItems - 1);
    }
  }, [totalItems, focusedIndex]);

  // Scroll focused element into view
  useEffect(() => {
    if (!enabled || focusedIndex < 0 || !gridRef.current) return;

    // Defensive bound check (edge case: effect runs before clamp effect)
    if (focusedIndex >= gridRef.current.children.length) return;

    const focusedElement = gridRef.current.children[focusedIndex] as HTMLElement;
    if (focusedElement) {
      focusedElement.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
    }
  }, [enabled, focusedIndex, gridRef]);

  return {
    focusedIndex,
    resetFocus,
    handleKeyDown,
  };
}
