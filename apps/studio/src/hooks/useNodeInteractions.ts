'use client';

/**
 * useNodeInteractions - Shared hover/press state management for nodes
 *
 * Provides consistent interaction state (hover, press) and mouse event handlers
 * for all node components. Eliminates duplicated state and callback logic.
 *
 * Used by: StructuralNode, LocaleKnowledgeNode, SchemaNode, ProjectNode
 */

import { useState, useCallback, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { NODE_DESIGN } from '@/config/constants';

export interface UseNodeInteractionsOptions {
  /** Whether node is selected */
  selected?: boolean;
  /** Whether node is dimmed (focus mode) */
  isDimmed?: boolean;
  /** Whether node is hover-dimmed (lighter dimming) */
  isHoverDimmed?: boolean;
  /** Whether this is a circular node (LocaleKnowledge) */
  isCircular?: boolean;
}

export interface NodeInteractionsResult {
  /** Whether mouse is hovering over node */
  isHovered: boolean;
  /** Whether mouse is pressed on node */
  isPressed: boolean;
  /** Handler for mouseenter event */
  handleMouseEnter: () => void;
  /** Handler for mouseleave event */
  handleMouseLeave: () => void;
  /** Handler for mousedown event */
  handleMouseDown: () => void;
  /** Handler for mouseup event */
  handleMouseUp: () => void;
  /** Combined className for container based on state */
  containerClassName: string;
  /** Inline style for container transitions */
  containerStyle: React.CSSProperties;
}

/**
 * Hook for managing node hover/press interactions
 */
export function useNodeInteractions({
  selected = false,
  isDimmed = false,
  isHoverDimmed = false,
  isCircular = false,
}: UseNodeInteractionsOptions = {}): NodeInteractionsResult {
  const [isHovered, setIsHovered] = useState(false);
  const [isPressed, setIsPressed] = useState(false);

  // Event handlers
  const handleMouseEnter = useCallback(() => {
    setIsHovered(true);
  }, []);

  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);
    setIsPressed(false);
  }, []);

  const handleMouseDown = useCallback(() => {
    setIsPressed(true);
  }, []);

  const handleMouseUp = useCallback(() => {
    setIsPressed(false);
  }, []);

  // Compute scale class based on state priority
  // Single scale value prevents class conflicts (e.g., scale-[0.98] + scale-105)
  const scaleClass = useMemo(() => {
    if (isDimmed) {
      return isCircular ? 'scale-75' : 'scale-90';
    }
    if (isPressed) {
      // Press feedback: reduce current scale by ~2%
      if (selected) {
        // 1.05 * 0.98 ≈ 1.03 - maintains press feedback on selected nodes
        return isCircular ? 'scale-[1.08]' : 'scale-[1.03]';
      }
      return isCircular ? 'scale-[0.96]' : 'scale-[0.98]';
    }
    if (selected) {
      return isCircular ? 'scale-110' : 'scale-105';
    }
    if (isHovered && !isHoverDimmed) {
      return 'scale-103';
    }
    return '';
  }, [isDimmed, isPressed, selected, isHovered, isHoverDimmed, isCircular]);

  // Compute container className based on state
  const containerClassName = useMemo(() => {
    return cn(
      'group relative node-pressable',
      // Full dimming (focus mode) - grayscale via className, opacity via style
      isDimmed && 'grayscale pointer-events-none',
      // Lighter dimming (hover highlight mode)
      isHoverDimmed && !isDimmed && 'hover-dimmed',
      // Single scale class (computed above to prevent conflicts)
      scaleClass
    );
  }, [isDimmed, isHoverDimmed, scaleClass]);

  // Container style for transitions (opacity moved here for dynamic values)
  const containerStyle = useMemo<React.CSSProperties>(() => ({
    transition: `transform ${NODE_DESIGN.timing.transform}ms ease-out, opacity ${NODE_DESIGN.timing.transform}ms ease-out`,
    opacity: isDimmed ? NODE_DESIGN.opacity.dimmed : 1,
  }), [isDimmed]);

  return {
    isHovered,
    isPressed,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
    containerClassName,
    containerStyle,
  };
}
