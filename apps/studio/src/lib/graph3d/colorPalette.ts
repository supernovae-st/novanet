/**
 * Color Palette for 3D Graph Visualization
 *
 * Re-exports colors from the unified palette system.
 * All colors derive from taxonomy.yaml via generated.ts.
 *
 * @see @/design/colors/palette.ts — Unified color system
 * @see @/design/colors/generated.ts — Auto-generated from taxonomy.yaml
 */

import type { Layer, Realm } from '@novanet/core/types';
import {
  LAYER_HEX,
  REALM_HEX,
  ARC_FAMILY_HEX,
} from '@/design/colors/palette';

// Re-export unified colors for 3D rendering
export const LAYER_COLORS: Record<Layer, string> = LAYER_HEX;
export const REALM_COLORS: Record<Realm, string> = REALM_HEX;
export const ARC_FAMILY_COLORS = ARC_FAMILY_HEX;

// Helper to convert hex to THREE.js color number
export function hexToNumber(hex: string): number {
  return parseInt(hex.replace('#', ''), 16);
}

// Get layer color with fallback
export function getLayerColor(layer?: Layer): string {
  return layer ? LAYER_COLORS[layer] : '#6366f1';
}

// Get realm color with fallback
export function getRealmColor(realm?: Realm): string {
  return realm ? REALM_COLORS[realm] : '#6366f1';
}
