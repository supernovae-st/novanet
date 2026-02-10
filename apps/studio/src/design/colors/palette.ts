/**
 * Unified Color Palette System — v11.7.0
 *
 * Single source of truth for all color operations in NovaNet Studio.
 * All colors derive from generated.ts (auto-generated from taxonomy.yaml).
 *
 * Features:
 * - Primary colors from taxonomy.yaml
 * - Computed variations (secondary, tertiary, glow)
 * - Gradient pairs for SVG rendering
 * - Stroke styles for arcs
 * - Consistent across graph (2D/3D) and sidebar
 *
 * @see generated.ts — Auto-generated color tokens
 * @see packages/core/models/taxonomy.yaml — Source of truth
 */

import {
  LAYER_COLORS,
  REALM_COLORS,
  TRAIT_COLORS,
  ARC_FAMILY_COLORS,
  type LayerKey,
  type RealmKey,
  type TraitKey,
  type ArcFamilyKey,
} from './generated';

// =============================================================================
// COLOR TYPES
// =============================================================================

export interface GradientColors {
  primary: string;
  secondary: string;
}

export interface ColorPalette {
  primary: string;
  secondary: string;
  tertiary: string;
  glow: string;
}

export interface StrokeStyle {
  style: 'solid' | 'dashed' | 'dotted';
  width: number;
  dashArray?: string;
}

// =============================================================================
// COLOR UTILITIES
// =============================================================================

/**
 * Lighten a hex color by a percentage
 */
function lightenColor(hex: string, percent: number): string {
  const num = parseInt(hex.replace('#', ''), 16);
  const r = Math.min(255, Math.floor((num >> 16) + (255 - (num >> 16)) * percent));
  const g = Math.min(255, Math.floor(((num >> 8) & 0x00ff) + (255 - ((num >> 8) & 0x00ff)) * percent));
  const b = Math.min(255, Math.floor((num & 0x0000ff) + (255 - (num & 0x0000ff)) * percent));
  return `#${((r << 16) | (g << 8) | b).toString(16).padStart(6, '0')}`;
}

/**
 * Add alpha to a hex color
 */
function withAlpha(hex: string, alpha: number): string {
  const alphaHex = Math.round(alpha * 255).toString(16).padStart(2, '0');
  return `${hex}${alphaHex}`;
}

/**
 * Create a full color palette from a primary color
 */
function createPalette(primary: string): ColorPalette {
  return {
    primary,
    secondary: lightenColor(primary, 0.2),
    tertiary: lightenColor(primary, 0.4),
    glow: withAlpha(primary, 0.4),
  };
}

/**
 * Create gradient colors from a primary color
 */
function createGradient(primary: string): GradientColors {
  return {
    primary,
    secondary: lightenColor(primary, 0.25),
  };
}

// =============================================================================
// LAYER PALETTES (from taxonomy.yaml via generated.ts)
// =============================================================================

export const LAYER_PALETTES: Record<LayerKey, ColorPalette> = {
  config: createPalette(LAYER_COLORS.config.color),
  locale: createPalette(LAYER_COLORS.locale.color),
  geography: createPalette(LAYER_COLORS.geography.color),
  knowledge: createPalette(LAYER_COLORS.knowledge.color),
  semantic: createPalette(LAYER_COLORS.semantic.color),
  foundation: createPalette(LAYER_COLORS.foundation.color),
  structure: createPalette(LAYER_COLORS.structure.color),
  instruction: createPalette(LAYER_COLORS.instruction.color),
  output: createPalette(LAYER_COLORS.output.color),
};

export const LAYER_GRADIENTS: Record<LayerKey, GradientColors> = {
  config: createGradient(LAYER_COLORS.config.color),
  locale: createGradient(LAYER_COLORS.locale.color),
  geography: createGradient(LAYER_COLORS.geography.color),
  knowledge: createGradient(LAYER_COLORS.knowledge.color),
  semantic: createGradient(LAYER_COLORS.semantic.color),
  foundation: createGradient(LAYER_COLORS.foundation.color),
  structure: createGradient(LAYER_COLORS.structure.color),
  instruction: createGradient(LAYER_COLORS.instruction.color),
  output: createGradient(LAYER_COLORS.output.color),
};

// =============================================================================
// REALM PALETTES (from taxonomy.yaml via generated.ts)
// =============================================================================

export const REALM_PALETTES: Record<RealmKey, ColorPalette> = {
  shared: createPalette(REALM_COLORS.shared.color),
  org: createPalette(REALM_COLORS.org.color),
};

// =============================================================================
// TRAIT PALETTES (from taxonomy.yaml via generated.ts)
// =============================================================================

export const TRAIT_PALETTES: Record<TraitKey, ColorPalette> = {
  invariant: createPalette(TRAIT_COLORS.invariant.color),
  localized: createPalette(TRAIT_COLORS.localized.color),
  knowledge: createPalette(TRAIT_COLORS.knowledge.color),
  generated: createPalette(TRAIT_COLORS.generated.color),
  aggregated: createPalette(TRAIT_COLORS.aggregated.color),
};

// =============================================================================
// ARC FAMILY PALETTES (from taxonomy.yaml via generated.ts)
// =============================================================================

export type ArcFamily = ArcFamilyKey;

export const ARC_PALETTES: Record<ArcFamilyKey, ColorPalette> = {
  ownership: createPalette(ARC_FAMILY_COLORS.ownership.color),
  localization: createPalette(ARC_FAMILY_COLORS.localization.color),
  semantic: createPalette(ARC_FAMILY_COLORS.semantic.color),
  generation: createPalette(ARC_FAMILY_COLORS.generation.color),
  mining: createPalette(ARC_FAMILY_COLORS.mining.color),
};

export const ARC_GRADIENTS: Record<ArcFamilyKey, GradientColors> = {
  ownership: createGradient(ARC_FAMILY_COLORS.ownership.color),
  localization: createGradient(ARC_FAMILY_COLORS.localization.color),
  semantic: createGradient(ARC_FAMILY_COLORS.semantic.color),
  generation: createGradient(ARC_FAMILY_COLORS.generation.color),
  mining: createGradient(ARC_FAMILY_COLORS.mining.color),
};

/**
 * Stroke styles per arc family (from taxonomy.yaml visual encoding)
 */
export const ARC_STROKES: Record<ArcFamilyKey, StrokeStyle> = {
  ownership: { style: 'solid', width: 2 },
  localization: { style: 'dashed', width: 2, dashArray: '6 3' },
  semantic: { style: 'dotted', width: 2, dashArray: '2 2' },
  generation: { style: 'solid', width: 3 },
  mining: { style: 'dashed', width: 1, dashArray: '4 2' },
};

// =============================================================================
// ARC FAMILY DETECTION (unified logic)
// =============================================================================

/**
 * Explicit relation type to arc family mapping
 */
const RELATION_TO_FAMILY: Record<string, ArcFamilyKey> = {
  // Ownership family
  HAS_PAGE: 'ownership',
  HAS_BLOCK: 'ownership',
  HAS_BRAND_IDENTITY: 'ownership',
  HAS_ENTITY: 'ownership',
  HAS_PROMPT: 'ownership',
  HAS_RULES: 'ownership',
  HAS_IDENTITY: 'ownership',
  HAS_VOICE: 'ownership',
  HAS_CULTURE: 'ownership',
  HAS_MARKET: 'ownership',
  HAS_LEXICON: 'ownership',
  OF_TYPE: 'ownership',
  OF_KIND: 'ownership',
  SUPPORTS_LOCALE: 'ownership',
  DEFAULT_LOCALE: 'ownership',
  IN_REALM: 'ownership',
  IN_LAYER: 'ownership',
  IN_FAMILY: 'ownership',
  FROM_KIND: 'ownership',
  TO_KIND: 'ownership',
  CONTAINS_TERM: 'ownership',
  CONTAINS_EXPRESSION: 'ownership',
  CONTAINS_PATTERN: 'ownership',
  CONTAINS_CULTURE_REF: 'ownership',
  CONTAINS_TABOO: 'ownership',
  CONTAINS_AUDIENCE_TRAIT: 'ownership',
  BELONGS_TO: 'ownership',

  // Localization family
  HAS_CONTENT: 'localization',
  FOR_LOCALE: 'localization',
  PRIMARY_LOCALE: 'localization',
  BELONGS_TO_PROJECT_CONTENT: 'localization',

  // Semantic family
  USES_ENTITY: 'semantic',
  SEMANTIC_LINK: 'semantic',
  RELATED_TO: 'semantic',
  INFLUENCED_BY: 'semantic',

  // Generation family
  HAS_GENERATED: 'generation',
  GENERATED_BY: 'generation',
  PREVIOUS_VERSION: 'generation',
  BASED_ON: 'generation',
  ASSEMBLED_FROM: 'generation',
  USES_MODEL: 'generation',
  USES_PROVIDER: 'generation',
  FALLBACK_TO: 'generation',

  // Mining family
  HAS_SEO_TARGET: 'mining',
  HAS_GEO_TARGET: 'mining',
  HAS_METRICS: 'mining',
  PRODUCED_BY: 'mining',
  TARGETS: 'mining',
  HAS_VARIATION: 'mining',
  HAS_SNAPSHOT: 'mining',
  HAS_CITATION: 'mining',
  HAS_REFORMULATION: 'mining',
};

/**
 * Pattern-based family detection (fallback)
 */
const FAMILY_PATTERNS: Array<{ pattern: RegExp | string; family: ArcFamilyKey }> = [
  // Localization (check before ownership HAS_*)
  { pattern: 'CONTENT', family: 'localization' },
  { pattern: /LOCALE/, family: 'localization' },

  // Semantic
  { pattern: /ENTITY/, family: 'semantic' },
  { pattern: 'SEMANTIC', family: 'semantic' },
  { pattern: 'USES_', family: 'semantic' },

  // Generation
  { pattern: /GENERATED/, family: 'generation' },
  { pattern: /VERSION/, family: 'generation' },
  { pattern: 'MODEL', family: 'generation' },

  // Mining
  { pattern: /SEO/, family: 'mining' },
  { pattern: /GEO/, family: 'mining' },
  { pattern: 'METRICS', family: 'mining' },
  { pattern: /MINING/, family: 'mining' },

  // Ownership (most HAS_* and CONTAINS)
  { pattern: /^HAS_/, family: 'ownership' },
  { pattern: 'CONTAINS', family: 'ownership' },
];

/**
 * Get arc family for a relation type
 *
 * Resolution order:
 * 1. Exact match in RELATION_TO_FAMILY
 * 2. Pattern matching
 * 3. Default to 'ownership'
 */
export function getArcFamily(relationType: string): ArcFamilyKey {
  // 1. Exact match
  if (relationType in RELATION_TO_FAMILY) {
    return RELATION_TO_FAMILY[relationType];
  }

  // 2. Pattern matching
  for (const { pattern, family } of FAMILY_PATTERNS) {
    if (typeof pattern === 'string') {
      if (relationType.includes(pattern)) {
        return family;
      }
    } else if (pattern.test(relationType)) {
      return family;
    }
  }

  // 3. Default
  return 'ownership';
}

// =============================================================================
// UNIFIED GETTERS
// =============================================================================

/**
 * Get full palette for a layer
 */
export function getLayerPalette(layer: LayerKey | string): ColorPalette {
  return LAYER_PALETTES[layer as LayerKey] ?? LAYER_PALETTES.foundation;
}

/**
 * Get gradient colors for a layer
 */
export function getLayerGradient(layer: LayerKey | string): GradientColors {
  return LAYER_GRADIENTS[layer as LayerKey] ?? LAYER_GRADIENTS.foundation;
}

/**
 * Get full palette for a realm
 */
export function getRealmPalette(realm: RealmKey | string): ColorPalette {
  return REALM_PALETTES[realm as RealmKey] ?? REALM_PALETTES.org;
}

/**
 * Get full palette for a trait
 */
export function getTraitPalette(trait: TraitKey | string): ColorPalette {
  return TRAIT_PALETTES[trait as TraitKey] ?? TRAIT_PALETTES.invariant;
}

/**
 * Get full palette for an arc type (by relation type string)
 */
export function getArcPalette(relationType: string): ColorPalette {
  const family = getArcFamily(relationType);
  return ARC_PALETTES[family];
}

/**
 * Get gradient colors for an arc type
 */
export function getArcGradient(relationType: string): GradientColors {
  const family = getArcFamily(relationType);
  return ARC_GRADIENTS[family];
}

/**
 * Get stroke style for an arc type
 */
export function getArcStroke(relationType: string): StrokeStyle {
  const family = getArcFamily(relationType);
  return ARC_STROKES[family];
}

/**
 * Get palette for an arc family directly
 */
export function getArcFamilyPalette(family: ArcFamilyKey): ColorPalette {
  return ARC_PALETTES[family];
}

// =============================================================================
// RAW HEX COLOR GETTERS (for Three.js / Canvas)
// =============================================================================

/**
 * Get raw hex colors for layers (for 3D rendering)
 */
export const LAYER_HEX: Record<LayerKey, string> = {
  config: LAYER_COLORS.config.color,
  locale: LAYER_COLORS.locale.color,
  geography: LAYER_COLORS.geography.color,
  knowledge: LAYER_COLORS.knowledge.color,
  semantic: LAYER_COLORS.semantic.color,
  foundation: LAYER_COLORS.foundation.color,
  structure: LAYER_COLORS.structure.color,
  instruction: LAYER_COLORS.instruction.color,
  output: LAYER_COLORS.output.color,
};

/**
 * Get raw hex colors for realms (for 3D rendering)
 */
export const REALM_HEX: Record<RealmKey, string> = {
  shared: REALM_COLORS.shared.color,
  org: REALM_COLORS.org.color,
};

/**
 * Get raw hex colors for traits (for 3D rendering)
 */
export const TRAIT_HEX: Record<TraitKey, string> = {
  invariant: TRAIT_COLORS.invariant.color,
  localized: TRAIT_COLORS.localized.color,
  knowledge: TRAIT_COLORS.knowledge.color,
  generated: TRAIT_COLORS.generated.color,
  aggregated: TRAIT_COLORS.aggregated.color,
};

/**
 * Get raw hex colors for arc families (for 3D rendering)
 */
export const ARC_FAMILY_HEX: Record<ArcFamilyKey, string> = {
  ownership: ARC_FAMILY_COLORS.ownership.color,
  localization: ARC_FAMILY_COLORS.localization.color,
  semantic: ARC_FAMILY_COLORS.semantic.color,
  generation: ARC_FAMILY_COLORS.generation.color,
  mining: ARC_FAMILY_COLORS.mining.color,
};

// =============================================================================
// RE-EXPORTS for convenience
// =============================================================================

export {
  LAYER_COLORS,
  REALM_COLORS,
  TRAIT_COLORS,
  ARC_FAMILY_COLORS,
  type LayerKey,
  type RealmKey,
  type TraitKey,
  type ArcFamilyKey,
} from './generated';
