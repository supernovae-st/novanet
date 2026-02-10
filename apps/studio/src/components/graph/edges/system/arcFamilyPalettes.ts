/**
 * Arc Family Palettes - v9.5
 *
 * Colors derived from taxonomy.yaml arc_families definitions.
 * Each arc family has a distinct visual identity.
 *
 * @see packages/core/models/taxonomy.yaml
 */

import type { ColorPalette } from './types';

// =============================================================================
// Arc Family Type (from taxonomy.yaml)
// =============================================================================

/**
 * Arc families from taxonomy.yaml
 * These are the primary classification for relationship styling.
 */
export type ArcFamily =
  | 'ownership'      // Structural parent→child containment
  | 'localization'   // Invariant↔locale-specific bridges
  | 'semantic'       // Concept connections + spreading activation
  | 'generation'     // LLM pipeline: prompts → outputs
  | 'mining';        // SEO/GEO targeting and metrics

// =============================================================================
// Arc Family Colors (from taxonomy.yaml)
// =============================================================================

/**
 * Primary colors from taxonomy.yaml arc_families
 */
export const ARC_FAMILY_COLORS: Record<ArcFamily, string> = {
  ownership: '#3b82f6',      // Blue
  localization: '#22c55e',   // Green
  semantic: '#f97316',       // Orange
  generation: '#8b5cf6',     // Purple
  mining: '#ec4899',         // Pink
} as const;

/**
 * Full color palettes per arc family
 * Based on taxonomy.yaml colors with harmonized secondary/tertiary
 */
export const ARC_FAMILY_PALETTES: Record<ArcFamily, ColorPalette> = {
  ownership: {
    primary: '#3b82f6',      // Blue 500
    secondary: '#60a5fa',    // Blue 400
    tertiary: '#93c5fd',     // Blue 300
    glow: '#3b82f6',
  },
  localization: {
    primary: '#22c55e',      // Green 500
    secondary: '#4ade80',    // Green 400
    tertiary: '#86efac',     // Green 300
    glow: '#22c55e',
  },
  semantic: {
    primary: '#f97316',      // Orange 500
    secondary: '#fb923c',    // Orange 400
    tertiary: '#fdba74',     // Orange 300
    glow: '#f97316',
  },
  generation: {
    primary: '#8b5cf6',      // Violet 500
    secondary: '#a78bfa',    // Violet 400
    tertiary: '#c4b5fd',     // Violet 300
    glow: '#8b5cf6',
  },
  mining: {
    primary: '#ec4899',      // Pink 500
    secondary: '#f472b6',    // Pink 400
    tertiary: '#f9a8d4',     // Pink 300
    glow: '#ec4899',
  },
} as const;

// =============================================================================
// Stroke Styles (from taxonomy.yaml)
// =============================================================================

/**
 * Stroke styles from taxonomy.yaml arc_families
 */
export const ARC_FAMILY_STROKES: Record<ArcFamily, {
  style: 'solid' | 'dashed' | 'dotted';
  width: number;
}> = {
  ownership: { style: 'solid', width: 2 },
  localization: { style: 'dashed', width: 2 },
  semantic: { style: 'dotted', width: 2 },
  generation: { style: 'solid', width: 3 },
  mining: { style: 'dashed', width: 1 },
} as const;

// =============================================================================
// Arc Family Effects Mapping (v11.6.1)
// =============================================================================

/**
 * Primary effect primitive for each arc family
 * These determine the unique visual identity per family.
 *
 * @see docs/plans/2026-02-10-arc-animation-system-v2-design.md
 */
export type ArcFamilyEffect =
  | 'energyPulse'    // ownership: power flows to children
  | 'dnaHelix'       // localization: content DNA adapts
  | 'zigzag'         // semantic: neural sparks (existing primitive)
  | 'matrixCode'     // generation: AI processing data
  | 'radarSweep';    // mining: scanning for intel

/**
 * Map arc families to their signature effect primitives
 */
export const ARC_FAMILY_EFFECTS: Record<ArcFamily, ArcFamilyEffect> = {
  ownership: 'energyPulse',
  localization: 'dnaHelix',
  semantic: 'zigzag',
  generation: 'matrixCode',
  mining: 'radarSweep',
} as const;

/**
 * Get the signature effect for an arc family
 */
export function getArcFamilyEffect(family: ArcFamily): ArcFamilyEffect {
  return ARC_FAMILY_EFFECTS[family];
}

/**
 * Get the signature effect for a relation type
 */
export function getRelationEffect(relationType: string): ArcFamilyEffect {
  const family = getArcFamily(relationType);
  return ARC_FAMILY_EFFECTS[family];
}

// =============================================================================
// Relation Type to Arc Family Mapping
// =============================================================================

/**
 * Map relation types to their arc families (from relations.yaml)
 * This is used when we know the exact relation type.
 */
export const RELATION_TO_FAMILY: Record<string, ArcFamily> = {
  // Ownership family (v10.3: HAS_CONCEPT removed)
  HAS_PAGE: 'ownership',
  HAS_BRAND_IDENTITY: 'ownership',
  SUPPORTS_LOCALE: 'ownership',
  DEFAULT_LOCALE: 'ownership',
  HAS_BLOCK: 'ownership',
  OF_TYPE: 'ownership',
  HAS_PROMPT: 'ownership',
  HAS_RULES: 'ownership',
  HAS_IDENTITY: 'ownership',
  HAS_VOICE: 'ownership',
  HAS_CULTURE: 'ownership',
  HAS_CULTURE_REFERENCES: 'ownership',
  HAS_MARKET: 'ownership',
  HAS_LEXICON: 'ownership',
  HAS_RULES_ADAPTATION: 'ownership',
  HAS_RULES_FORMATTING: 'ownership',
  HAS_RULES_SLUG: 'ownership',
  HAS_EXPRESSION: 'ownership',
  HAS_REFERENCE: 'ownership',
  HAS_METAPHOR: 'ownership',
  HAS_CONSTRAINT: 'ownership',
  HAS_PATTERN: 'ownership',
  OF_KIND: 'ownership',
  HAS_LAYER: 'ownership',
  HAS_KIND: 'ownership',
  HAS_TRAIT: 'ownership',
  HAS_ARC_KIND: 'ownership',

  // Localization family
  HAS_L10N: 'localization',
  HAS_OUTPUT: 'localization',
  FOR_LOCALE: 'localization',
  PRIMARY_LOCALE: 'localization',

  // Semantic family
  USES_ENTITY: 'semantic',
  SEMANTIC_LINK: 'semantic',
  RELATED_TO: 'semantic',

  // Generation family
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

  // Meta-graph arcs
  IN_REALM: 'ownership',
  IN_LAYER: 'ownership',
  IN_FAMILY: 'ownership',
  FROM_KIND: 'ownership',
  TO_KIND: 'ownership',
} as const;

/**
 * Pattern-based family detection for unknown relation types
 */
const FAMILY_PATTERNS: Array<{ pattern: RegExp | string; family: ArcFamily }> = [
  // Localization patterns
  { pattern: 'L10N', family: 'localization' },
  { pattern: /LOCALE/, family: 'localization' },
  { pattern: 'FOR_LOCALE', family: 'localization' },

  // Semantic patterns
  { pattern: /ENTITY/, family: 'semantic' },
  { pattern: 'SEMANTIC', family: 'semantic' },

  // Generation patterns
  { pattern: /GENERATED/, family: 'generation' },
  { pattern: /VERSION/, family: 'generation' },
  { pattern: 'MODEL', family: 'generation' },
  { pattern: 'PROVIDER', family: 'generation' },

  // Mining patterns
  { pattern: /SEO/, family: 'mining' },
  { pattern: /GEO/, family: 'mining' },
  { pattern: 'METRICS', family: 'mining' },
  { pattern: /MINING/, family: 'mining' },

  // Ownership patterns (most HAS_* relations)
  { pattern: /^HAS_/, family: 'ownership' },
  { pattern: 'CONTAINS', family: 'ownership' },
  { pattern: 'OF_TYPE', family: 'ownership' },
];

/**
 * Get the arc family for a relation type
 *
 * Resolution order:
 * 1. Exact match in RELATION_TO_FAMILY
 * 2. Pattern matching
 * 3. Default to 'ownership'
 */
export function getArcFamily(relationType: string): ArcFamily {
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
    } else {
      if (pattern.test(relationType)) {
        return family;
      }
    }
  }

  // 3. Default
  return 'ownership';
}

/**
 * Get palette for a relation type based on its arc family
 */
export function getArcFamilyPalette(relationType: string): ColorPalette {
  const family = getArcFamily(relationType);
  return ARC_FAMILY_PALETTES[family];
}

/**
 * Get stroke style for a relation type based on its arc family
 */
export function getArcFamilyStroke(relationType: string): { style: 'solid' | 'dashed' | 'dotted'; width: number } {
  const family = getArcFamily(relationType);
  return ARC_FAMILY_STROKES[family];
}
