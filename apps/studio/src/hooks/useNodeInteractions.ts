'use client';

/**
 * useNodeInteractions - Shared hover/press state management for nodes
 *
 * Provides consistent interaction state (hover, press) and mouse event handlers
 * for all node components. Eliminates duplicated state and callback logic.
 *
 * v11.3 Premium interactions:
 * - Levitation effect with translateY on hover/selected
 * - Enhanced shadows for depth perception
 * - Spring-like easing (cubic-bezier) for premium feel
 * - Unified across all node types
 *
 * Used by: StructuralNode, LocaleKnowledgeNode, SchemaNode, ProjectNode, SchemaBadgeNode
 */

import { useState, useCallback, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { NODE_DESIGN } from '@/config/constants';

// Premium easing curve (spring-like feel)
const PREMIUM_EASING = 'cubic-bezier(0.25, 0.46, 0.45, 0.94)';

// Levitation constants
const LEVITATION = {
  // How high the node floats (translateY in px, negative = up)
  hover: -6,
  selected: -10,
  pressed: -2,
  dimmed: 0,
  // Scale values
  scale: {
    hover: 1.02,
    selected: 1.04,
    pressed: 0.98,
    pressedSelected: 1.02,
    dimmed: 0.9,
    dimmedCircular: 0.75,
    selectedCircular: 1.08,
    hoverCircular: 1.04,
    pressedCircular: 0.96,
  },
  // Shadow depth (box-shadow spread)
  shadow: {
    base: '0 4px 12px rgba(0, 0, 0, 0.15)',
    hover: '0 12px 28px rgba(0, 0, 0, 0.2), 0 4px 8px rgba(0, 0, 0, 0.1)',
    selected: '0 16px 40px rgba(0, 0, 0, 0.25), 0 6px 12px rgba(0, 0, 0, 0.15)',
    pressed: '0 6px 16px rgba(0, 0, 0, 0.18)',
  },
  // Timing
  timing: 200,
} as const;

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
  /** Inline style for container transitions with levitation */
  containerStyle: React.CSSProperties;
}

/**
 * Hook for managing node hover/press interactions with premium levitation effects
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

  // Compute transform with levitation (translateY + scale)
  const transform = useMemo(() => {
    if (isDimmed) {
      const scale = isCircular ? LEVITATION.scale.dimmedCircular : LEVITATION.scale.dimmed;
      return `translateY(0) scale(${scale})`;
    }

    if (isPressed) {
      if (selected) {
        // Pressed while selected: slight sink from floating position
        const scale = isCircular ? LEVITATION.scale.selectedCircular * 0.98 : LEVITATION.scale.pressedSelected;
        return `translateY(${LEVITATION.pressed}px) scale(${scale})`;
      }
      // Pressed: sink down
      const scale = isCircular ? LEVITATION.scale.pressedCircular : LEVITATION.scale.pressed;
      return `translateY(${LEVITATION.pressed}px) scale(${scale})`;
    }

    if (selected) {
      // Selected: high levitation
      const scale = isCircular ? LEVITATION.scale.selectedCircular : LEVITATION.scale.selected;
      return `translateY(${LEVITATION.selected}px) scale(${scale})`;
    }

    if (isHovered && !isHoverDimmed) {
      // Hover: moderate levitation
      const scale = isCircular ? LEVITATION.scale.hoverCircular : LEVITATION.scale.hover;
      return `translateY(${LEVITATION.hover}px) scale(${scale})`;
    }

    // Default: grounded
    return 'translateY(0) scale(1)';
  }, [isDimmed, isPressed, selected, isHovered, isHoverDimmed, isCircular]);

  // Compute shadow based on state
  const boxShadow = useMemo(() => {
    if (isDimmed) return 'none';
    if (isPressed) return LEVITATION.shadow.pressed;
    if (selected) return LEVITATION.shadow.selected;
    if (isHovered && !isHoverDimmed) return LEVITATION.shadow.hover;
    return LEVITATION.shadow.base;
  }, [isDimmed, isPressed, selected, isHovered, isHoverDimmed]);

  // Compute container className based on state
  const containerClassName = useMemo(() => {
    return cn(
      'group relative',
      // Full dimming (focus mode) - grayscale via className
      isDimmed && 'grayscale pointer-events-none',
      // Lighter dimming (hover highlight mode)
      isHoverDimmed && !isDimmed && 'hover-dimmed'
    );
  }, [isDimmed, isHoverDimmed]);

  // Container style with premium levitation effects
  const containerStyle = useMemo<React.CSSProperties>(() => ({
    transform,
    boxShadow,
    opacity: isDimmed ? NODE_DESIGN.opacity.dimmed : 1,
    transition: `transform ${LEVITATION.timing}ms ${PREMIUM_EASING}, box-shadow ${LEVITATION.timing}ms ${PREMIUM_EASING}, opacity ${LEVITATION.timing}ms ease-out`,
    willChange: 'transform, box-shadow',
  }), [transform, boxShadow, isDimmed]);

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
