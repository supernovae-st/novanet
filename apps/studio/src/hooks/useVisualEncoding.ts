'use client';

/**
 * useVisualEncoding - Hook for accessing visual encoding rules
 *
 * Provides access to the visual encoding system defined in visual-encoding.yaml:
 * - Channel mapping (which visual property encodes which facet)
 * - Node/arc states (opacity, scale, shadow, etc.)
 * - Trait border styles (solid, dashed, dotted, double)
 * - Scope stroke styles (intra/cross realm)
 * - Cardinality arrow heads
 * - Kind icons (Lucide icon names)
 * - Animation presets
 * - Accessibility settings
 *
 * This is a static hook - no API fetch needed since data is compiled from YAML.
 *
 * @example
 * const {
 *   getNodeState,
 *   getArcState,
 *   getTraitBorder,
 *   getKindIcon,
 *   getAnimation,
 * } = useVisualEncoding();
 *
 * // Get node state style
 * const focusedStyle = getNodeState('focused');
 *
 * // Get trait border for a node
 * const border = getTraitBorder('localized');
 *
 * // Get icon for a Kind
 * const icon = getKindIcon('Page'); // 'file-text'
 */

import { useMemo } from 'react';
import {
  NODE_CHANNEL_MAPPING,
  ARC_CHANNEL_MAPPING,
  NODE_STATES,
  ARC_STATES,
  TRAIT_BORDERS,
  SCOPE_STROKES,
  CARDINALITY_ARROWS,
  KIND_ICONS,
  ANIMATIONS,
  ACCESSIBILITY,
  type NodeStateKey,
  type NodeStateStyle,
  type ArcStateKey,
  type ArcStateStyle,
  type TraitKey,
  type TraitBorderStyle,
  type ScopeKey,
  type ScopeStrokeStyle,
  type CardinalityKey,
  type CardinalityArrowStyle,
  type AnimationKey,
  type AnimationPreset,
} from '@novanet/core/graph';

// =============================================================================
// Types
// =============================================================================

export interface UseVisualEncodingResult {
  // Channel mapping
  nodeChannelMapping: typeof NODE_CHANNEL_MAPPING;
  arcChannelMapping: typeof ARC_CHANNEL_MAPPING;

  // State lookups
  getNodeState: (key: NodeStateKey) => NodeStateStyle;
  getArcState: (key: ArcStateKey) => ArcStateStyle;

  // Trait border lookups
  getTraitBorder: (trait: TraitKey) => TraitBorderStyle;
  getTraitBorderCSS: (trait: TraitKey) => React.CSSProperties;

  // Scope stroke lookups
  getScopeStroke: (scope: ScopeKey) => ScopeStrokeStyle;

  // Cardinality arrow lookups
  getCardinalityArrow: (cardinality: CardinalityKey) => CardinalityArrowStyle;

  // Kind icon lookups
  getKindIcon: (kind: string) => string;

  // Animation lookups
  getAnimation: (key: AnimationKey) => AnimationPreset;

  // Accessibility settings
  accessibility: typeof ACCESSIBILITY;

  // Raw data (for advanced use cases)
  nodeStates: typeof NODE_STATES;
  arcStates: typeof ARC_STATES;
  traitBorders: typeof TRAIT_BORDERS;
  scopeStrokes: typeof SCOPE_STROKES;
  cardinalityArrows: typeof CARDINALITY_ARROWS;
  kindIcons: typeof KIND_ICONS;
  animations: typeof ANIMATIONS;
}

// =============================================================================
// Hook
// =============================================================================

export function useVisualEncoding(): UseVisualEncodingResult {
  return useMemo(() => ({
    // Channel mapping
    nodeChannelMapping: NODE_CHANNEL_MAPPING,
    arcChannelMapping: ARC_CHANNEL_MAPPING,

    // State lookups
    getNodeState: (key: NodeStateKey): NodeStateStyle => NODE_STATES[key],
    getArcState: (key: ArcStateKey): ArcStateStyle => ARC_STATES[key],

    // Trait border lookups
    getTraitBorder: (trait: TraitKey): TraitBorderStyle => TRAIT_BORDERS[trait],
    getTraitBorderCSS: (trait: TraitKey): React.CSSProperties => {
      const border = TRAIT_BORDERS[trait];
      return {
        borderStyle: border.cssStyle as React.CSSProperties['borderStyle'],
        borderWidth: border.cssWidth,
        ...(border.cssCornerRadius && { borderRadius: border.cssCornerRadius }),
      };
    },

    // Scope stroke lookups
    getScopeStroke: (scope: ScopeKey): ScopeStrokeStyle => SCOPE_STROKES[scope],

    // Cardinality arrow lookups
    getCardinalityArrow: (cardinality: CardinalityKey): CardinalityArrowStyle =>
      CARDINALITY_ARROWS[cardinality],

    // Kind icon lookups
    getKindIcon: (kind: string): string => KIND_ICONS[kind] ?? 'circle',

    // Animation lookups
    getAnimation: (key: AnimationKey): AnimationPreset => ANIMATIONS[key],

    // Accessibility settings
    accessibility: ACCESSIBILITY,

    // Raw data
    nodeStates: NODE_STATES,
    arcStates: ARC_STATES,
    traitBorders: TRAIT_BORDERS,
    scopeStrokes: SCOPE_STROKES,
    cardinalityArrows: CARDINALITY_ARROWS,
    kindIcons: KIND_ICONS,
    animations: ANIMATIONS,
  }), []);
}

// =============================================================================
// Static helpers (for non-component code)
// =============================================================================

/**
 * Get node state style directly (no hook needed)
 */
export function getNodeStateStyle(key: NodeStateKey): NodeStateStyle {
  return NODE_STATES[key];
}

/**
 * Get arc state style directly (no hook needed)
 */
export function getArcStateStyle(key: ArcStateKey): ArcStateStyle {
  return ARC_STATES[key];
}

/**
 * Get trait border style directly (no hook needed)
 */
export function getTraitBorderStyle(trait: TraitKey): TraitBorderStyle {
  return TRAIT_BORDERS[trait];
}

/**
 * Get kind icon directly (no hook needed)
 */
export function getKindIconName(kind: string): string {
  return KIND_ICONS[kind] ?? 'circle';
}

/**
 * Get animation preset directly (no hook needed)
 */
export function getAnimationPreset(key: AnimationKey): AnimationPreset {
  return ANIMATIONS[key];
}

// =============================================================================
// Re-exports for convenience
// =============================================================================

export type {
  NodeStateKey,
  NodeStateStyle,
  ArcStateKey,
  ArcStateStyle,
  TraitKey,
  TraitBorderStyle,
  ScopeKey,
  ScopeStrokeStyle,
  CardinalityKey,
  CardinalityArrowStyle,
  AnimationKey,
  AnimationPreset,
};

export {
  NODE_CHANNEL_MAPPING,
  ARC_CHANNEL_MAPPING,
  NODE_STATES,
  ARC_STATES,
  TRAIT_BORDERS,
  SCOPE_STROKES,
  CARDINALITY_ARROWS,
  KIND_ICONS,
  ANIMATIONS,
  ACCESSIBILITY,
};
