'use client';

/**
 * useTriStateSelection - Hook for managing tri-state checkbox selection
 *
 * Features:
 * - Tracks selection state (none/partial/all)
 * - Toggle individual items
 * - Toggle all items in a group
 * - Calculate checkbox state from selection
 */

import { useState, useCallback } from 'react';
import type { CheckboxState } from '@/components/ui/TriStateCheckbox';

export interface UseTriStateSelectionReturn<T extends string> {
  /** Currently selected items */
  selectedItems: Set<T>;
  /** Toggle a single item */
  toggle: (item: T) => void;
  /** Set specific items (replaces current selection) */
  setSelected: (items: T[]) => void;
  /** Clear all selections */
  clearAll: () => void;
  /** Select all from a list */
  selectAll: (items: T[]) => void;
  /** Get checkbox state for a group of items */
  getCheckboxState: (items: T[]) => CheckboxState;
  /** Check if an item is selected */
  isSelected: (item: T) => boolean;
}

/**
 * Hook for managing tri-state selection
 *
 * @example
 * const { selectedItems, toggle, getCheckboxState } = useTriStateSelection<NodeType>();
 * const state = getCheckboxState(category.nodeTypes); // 'none' | 'partial' | 'all'
 */
export function useTriStateSelection<T extends string>(
  initialSelection: T[] = []
): UseTriStateSelectionReturn<T> {
  const [selectedItems, setSelectedItems] = useState<Set<T>>(
    () => new Set(initialSelection)
  );

  // Toggle single item
  const toggle = useCallback((item: T) => {
    setSelectedItems((prev) => {
      const next = new Set(prev);
      if (next.has(item)) {
        next.delete(item);
      } else {
        next.add(item);
      }
      return next;
    });
  }, []);

  // Set specific items
  const setSelected = useCallback((items: T[]) => {
    setSelectedItems(new Set(items));
  }, []);

  // Clear all
  const clearAll = useCallback(() => {
    setSelectedItems(new Set());
  }, []);

  // Select all from list
  const selectAll = useCallback((items: T[]) => {
    setSelectedItems((prev) => {
      const next = new Set(prev);
      items.forEach((item) => next.add(item));
      return next;
    });
  }, []);

  // Get checkbox state for a group
  const getCheckboxState = useCallback(
    (items: T[]): CheckboxState => {
      const selectedCount = items.filter((item) => selectedItems.has(item)).length;
      if (selectedCount === 0) return 'none';
      if (selectedCount === items.length) return 'all';
      return 'partial';
    },
    [selectedItems]
  );

  // Check if item is selected
  const isSelected = useCallback(
    (item: T): boolean => selectedItems.has(item),
    [selectedItems]
  );

  return {
    selectedItems,
    toggle,
    setSelected,
    clearAll,
    selectAll,
    getCheckboxState,
    isSelected,
  };
}

/**
 * Calculate checkbox state from a set of items
 * (Standalone function for use outside the hook)
 */
export function calculateCheckboxState<T>(
  allItems: T[],
  selectedItems: Set<T>
): CheckboxState {
  const selectedCount = allItems.filter((item) => selectedItems.has(item)).length;
  if (selectedCount === 0) return 'none';
  if (selectedCount === allItems.length) return 'all';
  return 'partial';
}
