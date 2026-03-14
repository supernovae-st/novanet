'use client';

/**
 * LayerCardWrapper - Semantic layer-based card styling
 *
 * Wraps CardShell with layer-specific visual encoding (ADR-005):
 * - Layer = fill gradient (background)
 * - Realm = border color
 *
 * This component provides the visual "skin" based on node taxonomy,
 * while CardShell handles interactions and effects.
 *
 * @example
 * ```tsx
 * <LayerCardWrapper
 *   layer="semantic"
 *   realm="org"
 *   selected={selected}
 *   renderContent={({ taxonomy }) => (
 *     <TaxonomyBadge {...taxonomy} />
 *     <MyContent />
 *   )}
 * />
 * ```
 */

import { memo, useMemo, type ReactNode } from 'react';
import { CardShell, type CardContext } from './CardShell';
import { LAYER_TOKENS } from '@/design/tokens/layerTokens';
import type { NodeLayer, NodeRealm } from './taxonomyColors';
import {
  getLayerCardClasses,
  getLayerIconGlowClass,
} from './variants/layerCardVariants';

// =============================================================================
// Types
// =============================================================================

export interface TaxonomyInfo {
  layer: NodeLayer;
  realm: NodeRealm;
  className?: string;
}

export interface LayerCardContext extends CardContext {
  /** Taxonomy info for TaxonomyBadge */
  taxonomy: TaxonomyInfo;
  /** Layer-specific icon glow class */
  iconGlowClass: string;
  /** Layer gradient class */
  gradientClass: string;
}

export interface LayerCardWrapperProps {
  // Required taxonomy
  /** Node layer (determines fill gradient) */
  layer: NodeLayer;
  /** Node realm (determines border color) */
  realm: NodeRealm;
  /** Node class name for icons and badges */
  className?: string;

  // State
  /** Whether the node is selected */
  selected: boolean;

  // Content
  /** Render function with extended context including taxonomy */
  renderContent: (context: LayerCardContext) => ReactNode;

  // Optional overrides
  /** Card width in pixels (default: 240) */
  width?: number;
  /** Minimum height in pixels */
  minHeight?: number;
  /** Additional CSS classes */
  wrapperClassName?: string;
  /** Schema mode styling */
  isSchemaMode?: boolean;
  /** Dimmed state */
  isDimmed?: boolean;
}

// =============================================================================
// Component
// =============================================================================

export const LayerCardWrapper = memo(function LayerCardWrapper({
  // Taxonomy
  layer,
  realm,
  className: nodeClassName,

  // State
  selected,

  // Content
  renderContent,

  // Options
  width = 240,
  minHeight,
  wrapperClassName,
  isSchemaMode = false,
  isDimmed = false,
}: LayerCardWrapperProps) {
  // Get layer token for colors
  const layerToken = LAYER_TOKENS[layer];

  // Derive colors from layer token
  const colors = useMemo(
    () => ({
      primary: layerToken.hex,
      secondary: adjustBrightness(layerToken.hex, 20),
    }),
    [layerToken.hex]
  );

  // Icon glow class based on layer
  const iconGlowClass = getLayerIconGlowClass(layer);

  // Layer card classes for inner styling
  const _layerCardClasses = useMemo(
    () =>
      getLayerCardClasses({
        layer,
        realm,
        selected,
        className: wrapperClassName,
      }),
    [layer, realm, selected, wrapperClassName]
  );

  // Taxonomy context for child components
  const taxonomyInfo: TaxonomyInfo = useMemo(
    () => ({
      layer,
      realm,
      className: nodeClassName,
    }),
    [layer, realm, nodeClassName]
  );

  return (
    <CardShell
      colors={colors}
      selected={selected}
      width={width}
      minHeight={minHeight}
      isDimmed={isDimmed}
      isSchemaMode={isSchemaMode}
      className={wrapperClassName}
      renderContent={(cardContext) => {
        // Extend context with taxonomy info
        const extendedContext: LayerCardContext = {
          ...cardContext,
          taxonomy: taxonomyInfo,
          iconGlowClass,
          gradientClass: layerToken.gradient,
        };

        return renderContent(extendedContext);
      }}
    />
  );
});

// =============================================================================
// Utilities
// =============================================================================

/**
 * Adjust hex color brightness
 * @param hex - Hex color string
 * @param percent - Brightness adjustment (-100 to 100)
 */
function adjustBrightness(hex: string, percent: number): string {
  const num = parseInt(hex.replace('#', ''), 16);
  const amt = Math.round(2.55 * percent);
  const R = (num >> 16) + amt;
  const G = ((num >> 8) & 0x00ff) + amt;
  const B = (num & 0x0000ff) + amt;

  return `#${(
    0x1000000 +
    (R < 255 ? (R < 1 ? 0 : R) : 255) * 0x10000 +
    (G < 255 ? (G < 1 ? 0 : G) : 255) * 0x100 +
    (B < 255 ? (B < 1 ? 0 : B) : 255)
  )
    .toString(16)
    .slice(1)}`;
}
