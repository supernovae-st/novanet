/**
 * Arc Family Effects - v11.7.0
 *
 * Signature animation effects per arc family.
 * Colors and family detection now delegated to unified palette system.
 *
 * @see packages/core/models/visual-encoding.yaml — arc_animation_effects section
 * @see FloatingEdge.tsx — InlineEdgeEffects implementation
 * @see @/design/colors/palette.ts — Unified color palette (colors, families)
 *
 * Animation Effects per Family:
 * - ownership:     ⚡ Energy Pulse (blue) — power flows to children
 * - localization:  🧬 DNA Helix (green) — content DNA adapts
 * - semantic:      🔗 Neural Sparks (orange) — meaning connections
 * - generation:    💻 Matrix Code (purple) — AI processing
 * - mining:        📡 Radar Sweep (pink) — data discovery
 */

// Import from unified palette system
import {
  type ArcFamilyKey,
  type ColorPalette,
  getArcFamily as getArcFamilyFromPalette,
  getArcPalette,
  getArcFamilyPalette as getArcFamilyPaletteByKey,
  ARC_PALETTES,
  ARC_STROKES,
  ARC_FAMILY_HEX,
} from '@/design/colors/palette';

// Re-export types and functions from unified palette
export type ArcFamily = ArcFamilyKey;
export {
  type ArcFamilyKey,
  ARC_PALETTES,
  ARC_STROKES,
  ARC_FAMILY_HEX,
};

// Re-export getArcFamily (same signature)
export const getArcFamily = getArcFamilyFromPalette;

/**
 * Get palette for a relation type (backwards compatible)
 *
 * v11.7.0: This wraps getArcPalette from unified palette for backwards compatibility.
 * Old signature: getArcFamilyPalette(relationType: string) -> ColorPalette
 */
export function getArcFamilyPalette(relationType: string): ColorPalette {
  return getArcPalette(relationType);
}

// Also re-export for direct family access
export { getArcPalette, getArcFamilyPaletteByKey };

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
  | 'radarSweep'     // mining: scanning for intel
  | 'schemaFlow';    // schema: structured definition flow

/**
 * Map arc families to their signature effect primitives
 */
export const ARC_FAMILY_EFFECTS: Record<ArcFamilyKey, ArcFamilyEffect> = {
  ownership: 'energyPulse',
  localization: 'dnaHelix',
  semantic: 'zigzag',
  generation: 'matrixCode',
  mining: 'radarSweep',
  schema: 'schemaFlow',
} as const;

/**
 * Get the signature effect for an arc family
 */
export function getArcFamilyEffect(family: ArcFamilyKey): ArcFamilyEffect {
  return ARC_FAMILY_EFFECTS[family];
}

/**
 * Get the signature effect for a relation type
 */
export function getRelationEffect(relationType: string): ArcFamilyEffect {
  const family = getArcFamily(relationType);
  return ARC_FAMILY_EFFECTS[family];
}
