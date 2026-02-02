/**
 * Layer Colors — ADR-014 Visual Encoding
 *
 * Maps each of the 9 Layers to a distinct fill color for graph nodes.
 * Uses the Solarized accent palette (colorblind-safe, tested on dark backgrounds).
 *
 * Visual channel: Fill color → Layer (WHAT functional classification)
 */

import type { Layer } from '@novanet/core/types';

export interface LayerColorTokens {
  /** Raw hex color */
  color: string;
  /** Tailwind bg class at 20% opacity */
  bg: string;
  /** Tailwind text class */
  text: string;
  /** Tailwind border class at 30% opacity */
  border: string;
  /** Tailwind bg class for badges/pills */
  bgSolid: string;
}

/**
 * 9 Layer colors — Solarized accent palette
 *
 * | Layer       | Color    | Solarized Name |
 * |-------------|----------|----------------|
 * | config      | #2aa198  | cyan           |
 * | knowledge   | #268bd2  | blue           |
 * | foundation  | #6c71c4  | violet         |
 * | structure   | #859900  | green          |
 * | semantic    | #b58900  | yellow         |
 * | instruction | #d33682  | magenta        |
 * | output      | #dc322f  | red            |
 * | seo         | #cb4b16  | orange         |
 * | geo         | #93a1a1  | base1 (gray)   |
 */
export const LAYER_COLORS: Record<Layer, LayerColorTokens> = {
  config: {
    color: '#2aa198',
    bg: 'bg-[#2aa198]/20',
    text: 'text-[#2aa198]',
    border: 'border-[#2aa198]/30',
    bgSolid: 'bg-[#2aa198]',
  },
  knowledge: {
    color: '#268bd2',
    bg: 'bg-[#268bd2]/20',
    text: 'text-[#268bd2]',
    border: 'border-[#268bd2]/30',
    bgSolid: 'bg-[#268bd2]',
  },
  foundation: {
    color: '#6c71c4',
    bg: 'bg-[#6c71c4]/20',
    text: 'text-[#6c71c4]',
    border: 'border-[#6c71c4]/30',
    bgSolid: 'bg-[#6c71c4]',
  },
  structure: {
    color: '#859900',
    bg: 'bg-[#859900]/20',
    text: 'text-[#859900]',
    border: 'border-[#859900]/30',
    bgSolid: 'bg-[#859900]',
  },
  semantic: {
    color: '#b58900',
    bg: 'bg-[#b58900]/20',
    text: 'text-[#b58900]',
    border: 'border-[#b58900]/30',
    bgSolid: 'bg-[#b58900]',
  },
  instruction: {
    color: '#d33682',
    bg: 'bg-[#d33682]/20',
    text: 'text-[#d33682]',
    border: 'border-[#d33682]/30',
    bgSolid: 'bg-[#d33682]',
  },
  output: {
    color: '#dc322f',
    bg: 'bg-[#dc322f]/20',
    text: 'text-[#dc322f]',
    border: 'border-[#dc322f]/30',
    bgSolid: 'bg-[#dc322f]',
  },
  seo: {
    color: '#cb4b16',
    bg: 'bg-[#cb4b16]/20',
    text: 'text-[#cb4b16]',
    border: 'border-[#cb4b16]/30',
    bgSolid: 'bg-[#cb4b16]',
  },
  geo: {
    color: '#93a1a1',
    bg: 'bg-[#93a1a1]/20',
    text: 'text-[#93a1a1]',
    border: 'border-[#93a1a1]/30',
    bgSolid: 'bg-[#93a1a1]',
  },
} as const;

/**
 * Get the color tokens for a Layer
 */
export function getLayerColor(layer: Layer): LayerColorTokens {
  return LAYER_COLORS[layer];
}

/**
 * Get the raw hex color for a Layer
 */
export function getLayerHex(layer: Layer): string {
  return LAYER_COLORS[layer].color;
}
