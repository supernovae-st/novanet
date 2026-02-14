/**
 * Node & Arc Gradient Colors — v11.7.0
 *
 * SVG gradient pairs (primary/secondary) for React Flow node rendering.
 * All colors now derive from the unified palette system (taxonomy.yaml).
 *
 * @see @/design/colors/palette.ts — Unified color system
 * @see @/design/colors/generated.ts — Auto-generated from taxonomy.yaml
 */

import type { NodeType } from '@/types';
import type { Layer } from '@novanet/core/types';
import {
  getLayerPalette,
  getArcPalette,
  DEFAULT_LAYER_GRADIENT,
  LAYER_GRADIENTS,
  type ColorPalette,
  type GradientColors,
} from '@/design/colors/palette';

// Re-export GradientColors type for backwards compatibility
export type { GradientColors } from '@/design/colors/palette';

// =============================================================================
// Unified Palette Conversion
// =============================================================================

function paletteToGradient(palette: ColorPalette): GradientColors {
  return {
    primary: palette.primary,
    secondary: palette.secondary,
  };
}

/**
 * Get gradient colors for a layer
 * Uses unified palette from taxonomy.yaml
 */
export function getLayerGradientColors(layer: Layer | undefined): GradientColors {
  if (!layer) return DEFAULT_LAYER_GRADIENT;
  return paletteToGradient(getLayerPalette(layer));
}

// =============================================================================
// Structural Node Colors (type-specific)
// =============================================================================

// Type-specific colors that extend layer colors
// These provide more granular styling for specific node types
// v0.12.4: BrandIdentity → Brand + BrandDesign + BrandPrinciples + PromptStyle (ADR-028)
const STRUCTURAL_LAYER_MAP: Record<string, Layer> = {
  Project: 'foundation',
  Page: 'structure',
  Block: 'structure',
  BlockType: 'instruction',
  Entity: 'semantic',
  Locale: 'config',
  Brand: 'foundation',
  BrandDesign: 'foundation',
  BrandPrinciples: 'foundation',
  PromptStyle: 'foundation',
  ProjectContent: 'foundation',
};

/**
 * Get gradient colors for structural node types
 * Maps node types to their layer colors from the unified palette
 */
export function getStructuralColors(type: string): GradientColors {
  const layer = STRUCTURAL_LAYER_MAP[type];
  if (layer) {
    return paletteToGradient(getLayerPalette(layer));
  }
  return DEFAULT_LAYER_GRADIENT;
}

// =============================================================================
// Shared Realm Knowledge Colors (v11.3: locale + knowledge layers)
// =============================================================================

// Knowledge-related types in shared realm (locale + knowledge layers)
const SHARED_KNOWLEDGE_TYPES = new Set([
  'Formatting', 'Slugification', 'Adaptation', 'Style',
  'TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet',
  'Term', 'Expression', 'Pattern', 'CultureRef', 'Taboo', 'AudienceTrait',
]);

// Knowledge layer gradient from unified palette
const KNOWLEDGE_LAYER_GRADIENT: GradientColors = LAYER_GRADIENTS.knowledge;

/**
 * Get gradient colors for shared realm knowledge types
 * All shared knowledge types use the 'knowledge' layer color
 */
export function getSharedKnowledgeColors(type: string): GradientColors {
  if (SHARED_KNOWLEDGE_TYPES.has(type)) {
    return paletteToGradient(getLayerPalette('knowledge'));
  }
  return KNOWLEDGE_LAYER_GRADIENT;
}

// =============================================================================
// Node Type Colors (unified lookup for any node type)
// =============================================================================

/**
 * Get gradient colors for any node type
 * Tries structural colors first, then shared knowledge, then defaults
 */
export function getNodeTypeColors(type: NodeType | string): GradientColors {
  // Check structural colors first
  if (type in STRUCTURAL_LAYER_MAP) {
    return getStructuralColors(type);
  }
  // Check shared realm knowledge colors
  if (SHARED_KNOWLEDGE_TYPES.has(type)) {
    return getSharedKnowledgeColors(type);
  }
  // Default to foundation layer
  return paletteToGradient(getLayerPalette('foundation'));
}

// =============================================================================
// Arc Colors (for arc details panel)
// =============================================================================

/**
 * Get gradient colors for arc types (relationships)
 * Uses unified arc family detection from palette.ts
 */
export function getRelationColors(type: string): GradientColors {
  return paletteToGradient(getArcPalette(type));
}

// =============================================================================
// Exports for constants (backwards compatibility)
// =============================================================================

export const COLORS = {
  DEFAULT: DEFAULT_LAYER_GRADIENT,
} as const;
