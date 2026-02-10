/**
 * Color Palette for 3D Graph Visualization
 *
 * Colors sourced from taxonomy.yaml and visual-encoding.yaml
 * Used by Graph3D for node and arc coloring.
 */

import type { Layer, Realm } from '@novanet/core/types';

// Layer colors (fill) - from taxonomy.yaml node_layers[].color
export const LAYER_COLORS: Record<Layer, string> = {
  config: '#64748b',     // slate-500
  locale: '#64748b',     // slate-500
  geography: '#10b981',  // emerald-500
  knowledge: '#8b5cf6',  // violet-500
  foundation: '#3b82f6', // blue-500
  structure: '#06b6d4',  // cyan-500
  semantic: '#f97316',   // orange-500
  instruction: '#eab308', // yellow-500
  output: '#22c55e',     // green-500
};

// Realm colors (outline/border) - from taxonomy.yaml node_realms[].color
export const REALM_COLORS: Record<Realm, string> = {
  shared: '#2aa198',     // solarized cyan
  org: '#6c71c4',        // solarized purple
};

// Trait colors - for material/effect identification
export const TRAIT_COLORS = {
  invariant: '#3b82f6',  // blue-500
  localized: '#22c55e',  // green-500
  knowledge: '#8b5cf6',  // violet-500
  generated: '#b58900',  // solarized yellow
  aggregated: '#6c71c4', // solarized purple
};

// Arc family colors - for edge/link coloring and particles
export const ARC_FAMILY_COLORS = {
  ownership: '#3b82f6',    // blue
  localization: '#22c55e', // green
  semantic: '#f97316',     // orange
  generation: '#8b5cf6',   // purple
  mining: '#ec4899',       // pink
};

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
