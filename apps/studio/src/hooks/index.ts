/**
 * Hooks barrel export
 */

export { useFilteredGraph, type FilteredGraphResult } from './useFilteredGraph';
export { useGraphData, type UseGraphDataReturn, type FetchOptions } from './useGraphData';
export { useUrlSync, UrlSyncComponent } from './useUrlSync';
export { useFocusMode, type FocusModeState } from './useFocusMode';
export { useHoverHighlight, type HoverHighlightState } from './useHoverHighlight';
export { useNodeExpansion, type UseNodeExpansionReturn, type ExpansionResult } from './useNodeExpansion';
export { useCopyFeedback, useCopyFieldFeedback, type UseCopyFeedbackReturn, type UseCopyFieldFeedbackReturn } from './useCopyFeedback';
export { useDatabaseSchema, type UseDatabaseSchemaReturn, type SchemaData, type NodeLabel, type RelationType } from './useDatabaseSchema';
export { useTriStateSelection, calculateCheckboxState, type UseTriStateSelectionReturn } from './useTriStateSelection';

// Modal utilities
export { useBodyScrollLock } from './useBodyScrollLock';
export { useFocusTrap } from './useFocusTrap';
export { useOutsideClick } from './useOutsideClick';
export { useModalAutoFocus } from './useModalAutoFocus';
export { useGridNavigation, type UseGridNavigationReturn } from './useGridNavigation';
export { useEscapeKey } from './useEscapeKey';
export { useModal, type UseModalOptions, type UseModalReturn } from './useModal';

// UI utilities
export { useTimeout, useTimeoutFn } from './useTimeout';
export { useAutoFocus } from './useAutoFocus';
export { useCollapsibleSections, type UseCollapsibleSectionsReturn } from './useCollapsibleSections';
export { useDebouncedValue } from './useDebouncedValue';

// Keyboard utilities
export { useKeyboardHandler, type KeyHandler, type UseKeyboardHandlerOptions } from './useKeyboardHandler';

// Animation utilities
export {
  useAnimationShortcut,
  ANIMATION_MODE_LABELS,
  getAnimationModeToast,
} from './useAnimationShortcut';

// Ref utilities
export { useLatestRef } from './useLatestRef';

// Viewport utilities
export {
  useViewportInsets,
  calculateViewportInsets,
  LAYOUT_CONSTANTS,
  type ViewportInsets,
  type FitViewConfig,
  type UIState,
} from './useViewportInsets';

export {
  useCenterOnNode,
  calculateCenterOffset,
  type CenterOffset,
  type CenterOnNodeOptions,
} from './useCenterOnNode';

export {
  useSmartFitView,
  type SmartFitViewOptions,
  type UseSmartFitViewReturn,
} from './useSmartFitView';

// Drag interaction utilities
export {
  useMagneticForce,
  type MagneticForceOptions,
  type UseMagneticForceReturn,
} from './useMagneticForce';

export {
  useGroupDrag,
  type GroupDragOptions,
  type UseGroupDragReturn,
  type NodePosition,
  type EdgeConnection,
} from './useGroupDrag';

export {
  useContainerConstraint,
  type ContainerConstraintOptions,
  type UseContainerConstraintReturn,
  type ContainerBounds,
} from './useContainerConstraint';

// Node interaction utilities
export {
  useNodeInteractions,
  type UseNodeInteractionsOptions,
  type NodeInteractionsResult,
} from './useNodeInteractions';

// Controlled/uncontrolled state pattern
export { useControllableState } from './useControllableState';

// Roving tabindex for keyboard navigation
export {
  useRovingTabindexRoot,
  useRovingTabindexItem,
  useRovingKeyboardHandler,
  RovingTabindexProvider,
  type RovingTabindexContextValue,
} from './useRovingTabindex';
