'use client';

/**
 * useNodeSelection - React Flow selection state synchronization
 *
 * Uses React Flow's useOnSelectionChange hook to sync selection state
 * with our uiStore. Handles node selection, aside panel opening, and
 * detail panel tab reset.
 *
 * Features:
 * - Memoized onChange handler (required by React Flow)
 * - Auto-opens panel when node is selected
 * - Resets detail panel tab to 'overview' on new selection
 * - Syncs with uiStore for cross-component access
 */

import { useCallback } from 'react';
import { useOnSelectionChange, type OnSelectionChangeParams } from '@xyflow/react';
import { useUIStore, selectSelectedNodeId, selectNodeSelectionActions } from '@/stores/uiStore';

export interface UseNodeSelectionResult {
  /** Currently selected node ID (null if none) */
  selectedNodeId: string | null;
  /** Set selected node programmatically */
  setSelectedNode: (id: string | null) => void;
  /** Clear all selection */
  clearSelection: () => void;
}

/**
 * Hook for managing node selection with React Flow integration
 *
 * @example
 * ```tsx
 * function GraphCanvas() {
 *   const { selectedNodeId } = useNodeSelection();
 *   // Selection is automatically synced with React Flow
 * }
 * ```
 */
export function useNodeSelection(): UseNodeSelectionResult {
  // Single primitive selector (no shallow needed for primitives)
  const selectedNodeId = useUIStore(selectSelectedNodeId);
  // Combined actions selector (stable reference - actions never change)
  const { setSelectedNode, setDetailPanelTab, clearSelection } = useUIStore(selectNodeSelectionActions);

  // IMPORTANT: onChange MUST be memoized for useOnSelectionChange
  // See: https://reactflow.dev/api-reference/hooks/use-on-selection-change
  const onChange = useCallback(
    ({ nodes }: OnSelectionChangeParams) => {
      // Get first selected node (single selection mode)
      const selected = nodes[0]?.id ?? null;

      // Update store with selected node
      setSelectedNode(selected);

      // Reset to overview tab on new selection
      if (selected) {
        setDetailPanelTab('overview');
      }
    },
    [setSelectedNode, setDetailPanelTab]
  );

  // Subscribe to React Flow selection changes
  useOnSelectionChange({ onChange });

  return {
    selectedNodeId,
    setSelectedNode,
    clearSelection,
  };
}

export default useNodeSelection;
