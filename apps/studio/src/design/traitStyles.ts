/**
 * Trait Styles — ADR-014 Visual Encoding
 *
 * Maps each of the 5 Traits to a distinct border style for graph nodes.
 * Uses border-style (not color) to be colorblind-safe.
 *
 * Visual channel: Border style → Trait (HOW locale behavior)
 *
 * v9.5: Now reads from generated visual-encoding.ts (source: visual-encoding.yaml)
 */

import type { CSSProperties } from 'react';
import type { Trait } from '@novanet/core/types';
import { TRAIT_BORDERS, type TraitKey } from '@novanet/core/graph';

export interface TraitStyleTokens {
  /** CSS border-style value */
  borderStyle: 'solid' | 'dashed' | 'double' | 'dotted' | 'none';
  /** CSS border-width value */
  borderWidth: string;
  /** Tailwind border classes (combines style + width) */
  className: string;
  /** Human-readable label */
  label: string;
  /** Unicode character for TUI */
  unicodeChar: string;
}

/**
 * Convert visual-encoding TraitBorderStyle to TraitStyleTokens
 */
function toTraitStyleTokens(trait: Trait): TraitStyleTokens {
  const border = TRAIT_BORDERS[trait as TraitKey];

  // Map CSS style to Tailwind class
  const styleClass =
    border.cssStyle === 'solid' ? 'border-solid' :
    border.cssStyle === 'dashed' ? 'border-dashed' :
    border.cssStyle === 'dotted' ? 'border-dotted' :
    border.cssStyle === 'double' ? 'border-double' :
    '';

  // Map CSS width to Tailwind class
  const widthClass =
    border.cssWidth === '1px' ? 'border' :
    border.cssWidth === '2px' ? 'border-2' :
    border.cssWidth === '3px' ? 'border-[3px]' :
    'border-2';

  return {
    borderStyle: border.cssStyle as TraitStyleTokens['borderStyle'],
    borderWidth: border.cssWidth,
    className: `${widthClass} ${styleClass}`,
    label: trait.charAt(0).toUpperCase() + trait.slice(1),
    unicodeChar: border.unicodeChar,
  };
}

/**
 * 5 Trait border styles (generated from visual-encoding.yaml)
 *
 * | Trait      | Border   | Meaning                          |
 * |-----------|----------|----------------------------------|
 * | invariant  | solid 2px  | Stable, doesn't change per locale |
 * | localized  | dashed 2px | Generated natively per locale    |
 * | knowledge  | dotted 2px | Locale knowledge reference data  |
 * | derived    | double 3px | Computed/aggregated data         |
 * | job        | solid 1px  | Background processing tasks      |
 */
export const TRAIT_STYLES: Record<Trait, TraitStyleTokens> = {
  invariant: toTraitStyleTokens('invariant'),
  localized: toTraitStyleTokens('localized'),
  knowledge: toTraitStyleTokens('knowledge'),
  derived: toTraitStyleTokens('derived'),
  job: toTraitStyleTokens('job'),
} as const;

/**
 * Get the style tokens for a Trait
 */
export function getTraitStyle(trait: Trait): TraitStyleTokens {
  return TRAIT_STYLES[trait];
}

/**
 * Get inline CSS border styles for a Trait (for React Flow nodes)
 */
export function getTraitBorderCSS(trait: Trait): CSSProperties {
  const border = TRAIT_BORDERS[trait as TraitKey];
  return {
    borderStyle: border.cssStyle as CSSProperties['borderStyle'],
    borderWidth: border.cssWidth,
    ...(border.cssCornerRadius && { borderRadius: border.cssCornerRadius }),
  };
}

/**
 * Get Unicode character for a Trait (for TUI rendering)
 */
export function getTraitUnicode(trait: Trait): string {
  return TRAIT_BORDERS[trait as TraitKey].unicodeChar;
}
