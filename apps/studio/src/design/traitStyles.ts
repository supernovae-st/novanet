/**
 * Trait Styles — ADR-014 Visual Encoding
 *
 * Maps each of the 5 Traits to a distinct border style for graph nodes.
 * Uses border-style (not color) to be colorblind-safe.
 *
 * Visual channel: Border style → Trait (HOW locale behavior)
 */

import type { CSSProperties } from 'react';
import type { Trait } from '@novanet/core/types';

export interface TraitStyleTokens {
  /** CSS border-style value */
  borderStyle: 'solid' | 'dashed' | 'double' | 'dotted' | 'none';
  /** CSS border-width value */
  borderWidth: string;
  /** Tailwind border classes (combines style + width) */
  className: string;
  /** Human-readable label */
  label: string;
}

/**
 * 5 Trait border styles
 *
 * | Trait      | Border   | Meaning                          |
 * |-----------|----------|----------------------------------|
 * | invariant  | solid 2px  | Stable, doesn't change per locale |
 * | localized  | dashed 2px | Generated natively per locale    |
 * | knowledge  | double 3px | Rich locale knowledge            |
 * | derived    | dotted 2px | Computed/aggregated data         |
 * | job        | solid 1px  | Background processing tasks      |
 */
export const TRAIT_STYLES: Record<Trait, TraitStyleTokens> = {
  invariant: {
    borderStyle: 'solid',
    borderWidth: '2px',
    className: 'border-2 border-solid',
    label: 'Invariant',
  },
  localized: {
    borderStyle: 'dashed',
    borderWidth: '2px',
    className: 'border-2 border-dashed',
    label: 'Localized',
  },
  knowledge: {
    borderStyle: 'double',
    borderWidth: '3px',
    className: 'border-[3px] border-double',
    label: 'Knowledge',
  },
  derived: {
    borderStyle: 'dotted',
    borderWidth: '2px',
    className: 'border-2 border-dotted',
    label: 'Derived',
  },
  job: {
    borderStyle: 'solid',
    borderWidth: '1px',
    className: 'border border-solid',
    label: 'Job',
  },
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
  const style = TRAIT_STYLES[trait];
  return {
    borderStyle: style.borderStyle,
    borderWidth: style.borderWidth,
  };
}
