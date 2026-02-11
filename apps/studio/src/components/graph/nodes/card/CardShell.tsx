'use client';

/**
 * CardShell - Unified wrapper component for all node cards
 *
 * Handles all common card functionality:
 * - Gradient borders with selection/hover states
 * - SelectionPulseRing effect
 * - GlassmorphismEffects
 * - BlueprintOverlay for meta mode
 * - NodeHandles (connection points)
 * - Interaction state management
 *
 * Uses render props pattern for maximum flexibility in content.
 *
 * @example
 * ```tsx
 * <CardShell
 *   colors={{ primary: '#8b5cf6', secondary: '#6366f1' }}
 *   selected={selected}
 *   width={240}
 *   renderContent={({ colors, selected, isHovered }) => (
 *     <MyCustomContent colors={colors} />
 *   )}
 * />
 * ```
 */

import { memo, useMemo, type ReactNode } from 'react';
import { cn } from '@/lib/utils';
import { useNodeInteractions } from '@/hooks';
import { SelectionPulseRing, GlassmorphismEffects, NodeHandles } from '../effects';
import { BlueprintOverlay } from '../BlueprintOverlay';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import { glassClasses } from '@/design/tokens';

// =============================================================================
// Types
// =============================================================================

export interface CardColors {
  primary: string;
  secondary: string;
}

export interface CardContext {
  /** Primary and secondary colors */
  colors: CardColors;
  /** Whether the node is selected */
  selected: boolean;
  /** Whether the node is being hovered */
  isHovered: boolean;
  /** Card width in pixels */
  width: number;
}

export interface CardShellProps {
  // Required
  /** Primary and secondary colors for gradients and effects */
  colors: CardColors;
  /** Whether the node is currently selected */
  selected: boolean;
  /** Render function that receives card context and returns content */
  renderContent: (context: CardContext) => ReactNode;

  // Sizing
  /** Card width in pixels (default: 200) */
  width?: number;
  /** Minimum card height in pixels (default: auto) */
  minHeight?: number;

  // Feature toggles (all default to true)
  /** Show selection pulse ring effect (default: true) */
  showPulseRing?: boolean;
  /** Show node connection handles (default: true) */
  showHandles?: boolean;
  /** Show glassmorphism effects when selected (default: true) */
  showGlassmorphism?: boolean;
  /** Show blueprint overlay in meta mode (default: true when isMetaMode) */
  showBlueprintOverlay?: boolean;

  // State
  /** Whether the node is dimmed (focus mode) */
  isDimmed?: boolean;
  /** Whether the node is hover-dimmed */
  isHoverDimmed?: boolean;
  /** Whether in meta/schema mode */
  isMetaMode?: boolean;

  // Customization
  /** Border radius in pixels (default: 16) */
  borderRadius?: number;
  /** Additional CSS classes */
  className?: string;
  /** Accessibility label */
  ariaLabel?: string;
}

// =============================================================================
// Component
// =============================================================================

export const CardShell = memo(function CardShell({
  // Required
  colors,
  selected,
  renderContent,

  // Sizing
  width = 200,
  minHeight,

  // Features (defaults = true)
  showPulseRing = true,
  showHandles = true,
  showGlassmorphism = true,
  showBlueprintOverlay = true,

  // State
  isDimmed = false,
  isHoverDimmed = false,
  isMetaMode = false,

  // Customization
  borderRadius = 16,
  className,
  ariaLabel,
}: CardShellProps) {
  // Interaction state management
  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
    containerClassName,
    containerStyle,
  } = useNodeInteractions({ selected, isDimmed, isHoverDimmed });

  // Memoize gradient border style
  const gradientBorderStyle = useMemo(() => ({
    background: selected
      ? NODE_DESIGN.gradients.borderSelected(colors.primary, colors.secondary)
      : isHovered
        ? NODE_DESIGN.gradients.borderHover(colors.primary, colors.secondary)
        : NODE_DESIGN.gradients.borderDefault(colors.primary, colors.secondary),
    boxShadow: selected
      ? NODE_DESIGN.shadows.glowSelected(colors.primary)
      : isHovered
        ? NODE_DESIGN.shadows.glowHover(colors.primary)
        : NODE_DESIGN.shadows.glow(colors.primary),
  }), [colors.primary, colors.secondary, selected, isHovered]);

  // Inner card radius is slightly smaller
  const innerRadius = borderRadius - 4;
  const innerSelectedRadius = borderRadius - 6;

  // Context for render props
  const context: CardContext = useMemo(() => ({
    colors,
    selected,
    isHovered,
    width,
  }), [colors, selected, isHovered, width]);

  return (
    <div
      className={cn(containerClassName, className)}
      style={{
        ...containerStyle,
        ...(isMetaMode && !selected && { opacity: 0.6, filter: 'saturate(0.7)' }),
      }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      aria-label={ariaLabel}
    >
      {/* Selection pulse ring effect */}
      {selected && showPulseRing && (
        <SelectionPulseRing color={colors.primary} borderRadius={borderRadius} />
      )}

      {/* Gradient border wrapper */}
      <div
        className={cn(
          'relative transition-all duration-300',
          selected && 'animate-gradient-rotate',
          isHovered && !selected && 'animate-glow-pulse'
        )}
        style={{
          borderRadius,
          padding: selected ? NODE_DESIGN.border.selected : NODE_DESIGN.border.default,
          ...gradientBorderStyle,
        }}
      >
        {/* Inner card */}
        <div
          className={cn(
            'relative overflow-hidden transition-all duration-500 ease-out',
            selected && showGlassmorphism && glassClasses.medium,
            selected && 'animate-float'
          )}
          style={{
            width,
            minHeight,
            borderRadius: selected ? innerSelectedRadius : innerRadius,
            backgroundColor: selected ? NODE_DESIGN.selectedBg : NODE_BG.default,
            border: selected ? `${NODE_DESIGN.border.innerSelected}px solid ${colors.primary}` : 'none',
            boxShadow: selected ? NODE_DESIGN.shadows.skeuomorphic(colors.primary) : undefined,
          }}
        >
          {/* Glassmorphism effects */}
          {selected && showGlassmorphism && (
            <GlassmorphismEffects borderRadius={selected ? innerSelectedRadius : innerRadius} />
          )}

          {/* Blueprint overlay for meta mode */}
          {isMetaMode && showBlueprintOverlay && (
            <BlueprintOverlay
              color={colors.primary}
              selected={selected}
              borderRadius={selected ? innerSelectedRadius : innerRadius}
            />
          )}

          {/* Node handles */}
          {showHandles && (
            <NodeHandles color={colors.primary} selected={selected} layout="vertical" />
          )}

          {/* Content via render props */}
          <div className="relative">
            {renderContent(context)}
          </div>
        </div>
      </div>
    </div>
  );
});
