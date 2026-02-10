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
  type ColorPalette,
} from '@/design/colors/palette';

// =============================================================================
// Color Types
// =============================================================================

export interface GradientColors {
  primary: string;
  secondary: string;
}

// =============================================================================
// Unified Palette Conversion
// =============================================================================

function paletteToGradient(palette: ColorPalette): GradientColors {
  return {
    primary: palette.primary,
    secondary: palette.secondary,
  };
}

// =============================================================================
// Layer Gradient Colors (used by TurboNode)
// =============================================================================

const DEFAULT_LAYER_COLORS: GradientColors = { primary: '#6366f1', secondary: '#8b5cf6' };

/**
 * Get gradient colors for a layer
 * Uses unified palette from taxonomy.yaml
 */
export function getLayerGradientColors(layer: Layer | undefined): GradientColors {
  if (!layer) return DEFAULT_LAYER_COLORS;
  return paletteToGradient(getLayerPalette(layer));
}

// =============================================================================
// Structural Node Colors (type-specific)
// =============================================================================

// Type-specific colors that extend layer colors
// These provide more granular styling for specific node types
const STRUCTURAL_LAYER_MAP: Record<string, Layer> = {
  Project: 'foundation',
  Page: 'structure',
  Block: 'structure',
  BlockType: 'instruction',
  Entity: 'semantic',
  Locale: 'config',
  BrandIdentity: 'foundation',
  ProjectContent: 'foundation',
};

const DEFAULT_STRUCTURAL_COLORS: GradientColors = { primary: '#6366f1', secondary: '#8b5cf6' };

/**
 * Get gradient colors for structural node types
 * Maps node types to their layer colors from the unified palette
 */
export function getStructuralColors(type: string): GradientColors {
  const layer = STRUCTURAL_LAYER_MAP[type];
  if (layer) {
    return paletteToGradient(getLayerPalette(layer));
  }
  return DEFAULT_STRUCTURAL_COLORS;
}

// =============================================================================
// Locale Knowledge Colors
// =============================================================================

// Locale knowledge types map to 'knowledge' layer
const LOCALE_KNOWLEDGE_TYPES = new Set([
  'Formatting', 'Slugification', 'Adaptation', 'Style',
  'TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet',
  'Term', 'Expression', 'Pattern', 'CultureRef', 'Taboo', 'AudienceTrait',
]);

const DEFAULT_LOCALE_KNOWLEDGE_COLORS: GradientColors = { primary: '#10b981', secondary: '#059669' };

/**
 * Get gradient colors for locale knowledge node types
 * All locale knowledge types use the 'knowledge' layer color
 */
export function getLocaleKnowledgeColors(type: string): GradientColors {
  if (LOCALE_KNOWLEDGE_TYPES.has(type)) {
    return paletteToGradient(getLayerPalette('knowledge'));
  }
  return DEFAULT_LOCALE_KNOWLEDGE_COLORS;
}

// =============================================================================
// Node Type Colors (unified lookup for any node type)
// =============================================================================

/**
 * Get gradient colors for any node type
 * Tries structural colors first, then locale knowledge, then defaults
 */
export function getNodeTypeColors(type: NodeType | string): GradientColors {
  // Check structural colors first
  if (type in STRUCTURAL_LAYER_MAP) {
    return getStructuralColors(type);
  }
  // Check locale knowledge colors
  if (LOCALE_KNOWLEDGE_TYPES.has(type)) {
    return getLocaleKnowledgeColors(type);
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
  DEFAULT: DEFAULT_LAYER_COLORS,
} as const;
