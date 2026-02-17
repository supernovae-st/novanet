'use client';

/**
 * CardShell - Unified wrapper component for all node cards
 *
 * Handles all common card functionality:
 * - Gradient borders with selection/hover states
 * - SelectionPulseRing effect (performance-aware)
 * - GlassmorphismEffects (performance-aware)
 * - BlueprintOverlay for schema mode
 * - NodeHandles (connection points)
 * - Interaction state management
 * - Performance tier integration
 *
 * Uses render props pattern for maximum flexibility in content.
 *
 * @example
 * ```tsx
 * <CardShell
 *   colors={{ primary: '#8b5cf6', secondary: '#6366f1' }}
 *   selected={selected}
 *   width={240}
 *   renderContent={({ colors, selected, isHovered, performanceTier }) => (
 *     <MyCustomContent colors={colors} tier={performanceTier} />
 *   )}
 * />
 * ```
 */

import { memo, useMemo, useContext, type ReactNode } from 'react';
import { cn } from '@/lib/utils';
import { useNodeInteractions } from '@/hooks';
import { SelectionPulseRing, GlassmorphismEffects, NodeHandles } from '../effects';
import { BlueprintOverlay } from '../BlueprintOverlay';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import { glassClasses } from '@/design/tokens';
import {
  usePerformance,
  type PerformanceTier,
  type PerformanceConfig,
  TIER_CONFIGS,
} from '@/contexts/PerformanceContext';
import { type NodeTrait, TRAIT_BORDERS } from './taxonomyColors';

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
  /** Current performance tier (ULTRA/HIGH/MEDIUM/LOW/MINIMAL) - defaults to HIGH */
  performanceTier?: PerformanceTier;
  /** Performance configuration with enabled effects - defaults to HIGH config */
  performanceConfig?: PerformanceConfig;
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

  // Taxonomy visual encoding (ADR-005)
  /** Node trait for border style (solid/dashed/dotted/double) */
  trait?: NodeTrait;

  // Feature toggles (all default to true)
  /** Show selection pulse ring effect (default: true) */
  showPulseRing?: boolean;
  /** Show node connection handles (default: true) */
  showHandles?: boolean;
  /** Show glassmorphism effects when selected (default: true) */
  showGlassmorphism?: boolean;
  /** Show blueprint overlay in schema mode (default: true when isSchemaMode) */
  showBlueprintOverlay?: boolean;

  // State
  /** Whether the node is dimmed (focus mode) */
  isDimmed?: boolean;
  /** Whether the node is hover-dimmed */
  isHoverDimmed?: boolean;
  /** Whether in schema mode */
  isSchemaMode?: boolean;

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

// Default performance config for when PerformanceProvider is not available
const DEFAULT_PERFORMANCE: { tier: PerformanceTier; config: PerformanceConfig } = {
  tier: 'HIGH',
  config: TIER_CONFIGS.HIGH,
};

// Safe hook wrapper that returns default if provider is missing
function useSafePerformance(): { tier: PerformanceTier; config: PerformanceConfig } {
  try {
    return usePerformance();
  } catch {
    return DEFAULT_PERFORMANCE;
  }
}

export const CardShell = memo(function CardShell({
  // Required
  colors,
  selected,
  renderContent,

  // Sizing
  width = 200,
  minHeight,

  // Taxonomy (ADR-005)
  trait,

  // Features (defaults = true)
  showPulseRing = true,
  showHandles = true,
  showGlassmorphism = true,
  showBlueprintOverlay = true,

  // State
  isDimmed = false,
  isHoverDimmed = false,
  isSchemaMode = false,

  // Customization
  borderRadius = 16,
  className,
  ariaLabel,
}: CardShellProps) {
  // Performance context (with fallback)
  const { tier: performanceTier, config: performanceConfig } = useSafePerformance();

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

  // Performance-aware effect toggles
  const effectsEnabled = useMemo(() => ({
    pulseRing: showPulseRing && performanceConfig.effects.outerGlow,
    glassmorphism: showGlassmorphism && performanceConfig.effects.glassmorphism,
    animations: performanceConfig.animation.enabled,
  }), [showPulseRing, showGlassmorphism, performanceConfig]);

  // Memoize gradient border style
  const gradientBorderStyle = useMemo(() => ({
    background: selected
      ? NODE_DESIGN.gradients.borderSelected(colors.primary, colors.secondary)
      : isHovered
        ? NODE_DESIGN.gradients.borderHover(colors.primary, colors.secondary)
        : NODE_DESIGN.gradients.borderDefault(colors.primary, colors.secondary),
    boxShadow: effectsEnabled.animations
      ? (selected
          ? NODE_DESIGN.shadows.glowSelected(colors.primary)
          : isHovered
            ? NODE_DESIGN.shadows.glowHover(colors.primary)
            : NODE_DESIGN.shadows.glow(colors.primary))
      : undefined,
  }), [colors.primary, colors.secondary, selected, isHovered, effectsEnabled.animations]);

  // Inner card radius is slightly smaller
  const innerRadius = borderRadius - 4;
  const innerSelectedRadius = borderRadius - 6;

  // Trait border style (ADR-005: border style encodes data origin)
  const traitBorderStyle = useMemo(() => {
    if (!trait) return {};
    const traitInfo = TRAIT_BORDERS[trait];
    return {
      borderStyle: traitInfo.style,
      borderWidth: traitInfo.width,
      borderColor: selected
        ? colors.primary
        : `${colors.primary}80`, // 50% opacity when not selected
    };
  }, [trait, colors.primary, selected]);

  // Context for render props (now includes performance info)
  const context: CardContext = useMemo(() => ({
    colors,
    selected,
    isHovered,
    width,
    performanceTier,
    performanceConfig,
  }), [colors, selected, isHovered, width, performanceTier, performanceConfig]);

  return (
    <div
      className={cn(containerClassName, className)}
      style={{
        ...containerStyle,
        ...(isSchemaMode && !selected && { opacity: 0.6, filter: 'saturate(0.7)' }),
      }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      aria-label={ariaLabel}
    >
      {/* Selection pulse ring effect (performance-aware) */}
      {selected && effectsEnabled.pulseRing && (
        <SelectionPulseRing color={colors.primary} borderRadius={borderRadius} />
      )}

      {/* Gradient border wrapper */}
      <div
        className={cn(
          'relative transition-all duration-300',
          effectsEnabled.animations && selected && 'animate-gradient-rotate',
          effectsEnabled.animations && isHovered && !selected && 'animate-glow-pulse'
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
            selected && effectsEnabled.glassmorphism && glassClasses.medium,
            effectsEnabled.animations && selected && 'animate-float'
          )}
          style={{
            width,
            minHeight,
            borderRadius: selected ? innerSelectedRadius : innerRadius,
            backgroundColor: selected ? NODE_DESIGN.selectedBg : NODE_BG.default,
            // Apply trait border when not selected, primary border when selected
            ...(selected
              ? {
                  border: `${NODE_DESIGN.border.innerSelected}px solid ${colors.primary}`,
                  boxShadow: NODE_DESIGN.shadows.skeuomorphic(colors.primary),
                }
              : trait
                ? traitBorderStyle
                : {}),
          }}
        >
          {/* Glassmorphism effects (performance-aware) */}
          {selected && effectsEnabled.glassmorphism && (
            <GlassmorphismEffects borderRadius={selected ? innerSelectedRadius : innerRadius} />
          )}

          {/* Blueprint overlay for schema mode */}
          {isSchemaMode && showBlueprintOverlay && (
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
