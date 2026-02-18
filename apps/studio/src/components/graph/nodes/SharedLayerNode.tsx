'use client';

/**
 * SharedLayerNode - Premium card design for shared realm nodes (v0.13.1)
 *
 * Handles nodes from 4 shared realm layers:
 * - config: Locale, EntityCategory
 * - locale: Culture, Style, Formatting, etc.
 * - geography: Region, Country, Continent, etc.
 * - knowledge: Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait
 * Plus containers: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
 *
 * v0.13.1 Enhancements:
 * - Layer-based gradient fills with smooth color encoding
 * - Realm-specific border styling (teal for shared realm)
 * - Trait indicator visualization (border style per ADR-005)
 * - Premium hover/select states with glass morphism
 * - Passport card design language alignment
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { getSharedKnowledgeColors } from '@/design/nodeColors';
import type { BaseNodeData } from './BaseNodeWrapper';
import { CardShell, getCardContentComponent, type NodeTrait } from './card';

export type SharedLayerNodeType = Node<BaseNodeData>;

// =============================================================================
// Layer Mapping
// =============================================================================

type SharedLayer = 'config' | 'locale' | 'geography' | 'knowledge';

/**
 * Map node type to its layer in the shared realm
 */
function getLayerForType(type: string): SharedLayer {
  switch (type) {
    // Config layer
    case 'Locale':
    case 'EntityCategory':
    case 'SEOKeywordFormat':
      return 'config';
    // Locale layer
    case 'Culture':
    case 'Style':
    case 'Formatting':
    case 'Adaptation':
    case 'Slugification':
    case 'Market':
      return 'locale';
    // Geography layer
    case 'Continent':
    case 'GeoRegion':
    case 'GeoSubRegion':
    case 'Country':
    case 'Region':
      return 'geography';
    // Knowledge layer (default)
    default:
      return 'knowledge';
  }
}

/**
 * Map node type to its trait (data origin per ADR-024)
 */
function getTraitForType(type: string): NodeTrait {
  switch (type) {
    // Defined: human-created once
    case 'Locale':
    case 'EntityCategory':
    case 'SEOKeywordFormat':
    case 'Continent':
    case 'GeoRegion':
    case 'GeoSubRegion':
    case 'Country':
    case 'Region':
    case 'TermSet':
    case 'ExpressionSet':
    case 'PatternSet':
    case 'CultureSet':
    case 'TabooSet':
    case 'AudienceSet':
    case 'CategorySet':
      return 'defined';
    // Imported: external data brought in
    case 'Term':
    case 'Expression':
    case 'Pattern':
    case 'CultureRef':
    case 'Taboo':
    case 'AudienceTrait':
    case 'SEOKeyword':
    case 'GEOQuery':
      return 'imported';
    // Retrieved: fetched from external APIs
    case 'GEOAnswer':
    case 'SEOKeywordMetrics':
    case 'GEOMetrics':
      return 'retrieved';
    // Locale settings: defined
    case 'Culture':
    case 'Style':
    case 'Formatting':
    case 'Adaptation':
    case 'Slugification':
    case 'Market':
      return 'defined';
    default:
      return 'defined';
  }
}

// =============================================================================
// Card Sizing
// =============================================================================

/**
 * Get card width based on node type
 * Refined sizing for premium visual balance
 */
function getCardWidth(type: string): number {
  switch (type) {
    // Containers (larger with more presence)
    case 'TermSet':
    case 'ExpressionSet':
    case 'PatternSet':
    case 'CultureSet':
    case 'TabooSet':
    case 'AudienceSet':
    case 'CategorySet':
      return 180;
    // EntityCategory (large "Classification Nexus" design - v0.13.1)
    case 'EntityCategory':
      return 420;
    // Config nodes (important, slightly larger)
    case 'Locale':
    case 'SEOKeywordFormat':
      return 175;
    // Geography nodes (prominent)
    case 'Continent':
    case 'GeoRegion':
    case 'GeoSubRegion':
    case 'Country':
    case 'Region':
      return 175;
    // Locale settings
    case 'Culture':
    case 'Style':
    case 'Formatting':
    case 'Adaptation':
    case 'Slugification':
    case 'Market':
      return 170;
    // Knowledge atoms (compact but readable)
    case 'Term':
    case 'Expression':
    case 'Pattern':
    case 'CultureRef':
    case 'Taboo':
    case 'AudienceTrait':
      return 160;
    // SEO/GEO nodes
    case 'SEOKeyword':
    case 'GEOQuery':
    case 'GEOAnswer':
    case 'SEOKeywordMetrics':
    case 'GEOMetrics':
      return 165;
    default:
      return 165;
  }
}

/**
 * Get enhanced colors with layer-specific gradients
 * Provides richer color palette for premium visuals
 */
function getEnhancedColors(type: string, layer: SharedLayer): { primary: string; secondary: string } {
  const baseColors = getSharedKnowledgeColors(type);

  // Enhance secondary color based on layer for gradient depth
  const layerEnhancements: Record<SharedLayer, { saturation: number; lightness: number }> = {
    config: { saturation: 0.9, lightness: 1.15 },     // Slate tones - subtle
    locale: { saturation: 0.95, lightness: 1.12 },    // Slate with warmth
    geography: { saturation: 1.1, lightness: 1.18 },  // Emerald - vibrant
    knowledge: { saturation: 1.05, lightness: 1.2 },  // Violet - rich
  };

  const enhancement = layerEnhancements[layer];

  // Calculate enhanced secondary color
  const secondary = adjustColorForGradient(
    baseColors.secondary,
    enhancement.saturation,
    enhancement.lightness
  );

  return {
    primary: baseColors.primary,
    secondary,
  };
}

/**
 * Adjust color for gradient effect
 */
function adjustColorForGradient(hex: string, satMod: number, lightMod: number): string {
  // Parse hex to RGB
  const num = parseInt(hex.replace('#', ''), 16);
  let r = (num >> 16) & 0xff;
  let g = (num >> 8) & 0xff;
  let b = num & 0xff;

  // Apply lightness modification
  r = Math.min(255, Math.round(r * lightMod));
  g = Math.min(255, Math.round(g * lightMod));
  b = Math.min(255, Math.round(b * lightMod));

  // Return as hex
  return `#${((r << 16) | (g << 8) | b).toString(16).padStart(6, '0')}`;
}

// =============================================================================
// Component
// =============================================================================

/**
 * SharedLayerNode - Premium card with full visual encoding
 *
 * Features:
 * - Layer-based gradient fills with smooth transitions
 * - Shared realm border styling (teal accent)
 * - Trait indicator via border style (ADR-005)
 * - Enhanced hover/select states with glass morphism effects
 * - Specialized card content routing per node type
 */
export const SharedLayerNode = memo(function SharedLayerNode(props: NodeProps<SharedLayerNodeType>) {
  const { data, selected = false } = props;

  // Determine layer and trait for visual encoding
  const layer = useMemo(() => getLayerForType(data.type), [data.type]);
  const trait = useMemo(() => getTraitForType(data.type), [data.type]);

  // Enhanced colors with layer-specific gradients
  const colors = useMemo(
    () => getEnhancedColors(data.type, layer),
    [data.type, layer]
  );

  const width = getCardWidth(data.type);

  // Get the specialized card content component for this node type
  const CardContent = useMemo(() => getCardContentComponent(data.type), [data.type]);

  // Prepare data for card content
  // BaseNodeData extends Record<string, unknown>, so all Neo4j properties
  // are available directly on data.
  const contentData = useMemo(() => ({
    ...data,
    id: data.id,
    type: data.type,
    key: data.key,
    displayName: data.displayName,
    locale: data.locale,
  }), [data]);

  return (
    <CardShell
      colors={colors}
      selected={selected}
      width={width}
      trait={trait}
      isDimmed={data.dimmed === true}
      isHoverDimmed={data.hoverDimmed === true}
      isSchemaMode={data.isSchemaMode === true}
      ariaLabel={`${data.type} node: ${data.displayName}`}
      renderContent={(ctx) => (
        <CardContent data={contentData} {...ctx} />
      )}
    />
  );
});
