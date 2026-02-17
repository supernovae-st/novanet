/**
 * Trait Tokens - Visual encoding for the 5 NovaNet traits (Data Origin)
 *
 * Source of truth: visual-encoding.yaml (ADR-024)
 * Trait = Data Origin (WHERE does data come from?)
 *
 * Visual encoding:
 * - Border style: solid/dashed/dotted/double
 * - Animation mode: pulse/breathe/colorShift/flowHorizontal/rotate
 *
 * Usage:
 * ```tsx
 * import { TRAIT_TOKENS, getTraitAnimation } from '@/design/tokens/traitTokens';
 *
 * const animation = getTraitAnimation('generated');
 * // -> 'flowHorizontal'
 * ```
 */

import type { NodeTrait } from '@/components/graph/nodes/card/taxonomyColors';
import type { GlowMode } from '@/components/graph/nodes/card/effects/TraitGlow';

// =============================================================================
// Trait Token Definition
// =============================================================================

export interface TraitToken {
  /** Trait identifier */
  key: NodeTrait;
  /** Display name */
  name: string;
  /** Border style CSS value */
  borderStyle: 'solid' | 'dashed' | 'dotted' | 'double';
  /** Border width in pixels */
  borderWidth: number;
  /** Tailwind border class */
  borderClass: string;
  /** Animation mode for TraitGlow */
  animation: GlowMode;
  /** Animation description */
  animationDesc: string;
  /** Unicode character for terminal */
  unicode: string;
  /** Who creates this data */
  creator: string;
  /** Description */
  description: string;
}

export const TRAIT_TOKENS: Record<NodeTrait, TraitToken> = {
  defined: {
    key: 'defined',
    name: 'Defined',
    borderStyle: 'solid',
    borderWidth: 2,
    borderClass: 'border-2 border-solid',
    animation: 'pulse',
    animationDesc: 'Stable pulse (solid, unchanging foundation)',
    unicode: '─',
    creator: 'Human, ONCE',
    description: 'Human-created structure/template',
  },
  authored: {
    key: 'authored',
    name: 'Authored',
    borderStyle: 'dashed',
    borderWidth: 2,
    borderClass: 'border-2 border-dashed',
    animation: 'breathe',
    animationDesc: 'Organic breathing (human touch)',
    unicode: '┄',
    creator: 'Human, PER locale',
    description: 'Human-written content per locale',
  },
  imported: {
    key: 'imported',
    name: 'Imported',
    borderStyle: 'dotted',
    borderWidth: 2,
    borderClass: 'border-2 border-dotted',
    animation: 'colorShift',
    animationDesc: 'Color shift (external data flowing in)',
    unicode: '┈',
    creator: 'External sources',
    description: 'External data brought in',
  },
  generated: {
    key: 'generated',
    name: 'Generated',
    borderStyle: 'double',
    borderWidth: 3,
    borderClass: 'border-[3px] [border-style:double]',
    animation: 'flowHorizontal',
    animationDesc: 'Horizontal flow (LLM generation stream)',
    unicode: '═',
    creator: 'Our LLM',
    description: 'OUR LLM produces this output',
  },
  retrieved: {
    key: 'retrieved',
    name: 'Retrieved',
    borderStyle: 'dotted',
    borderWidth: 3,
    borderClass: 'border-[3px] border-dotted',
    animation: 'rotate',
    animationDesc: 'Rotation (fetching, loading)',
    unicode: '┅',
    creator: 'External APIs',
    description: 'Fetched from EXTERNAL APIs',
  },
};

// =============================================================================
// Helper Functions
// =============================================================================

/**
 * Get animation mode for a trait
 */
export function getTraitAnimation(trait: NodeTrait): GlowMode {
  return TRAIT_TOKENS[trait].animation;
}

/**
 * Get border class for a trait
 */
export function getTraitBorderClass(trait: NodeTrait): string {
  return TRAIT_TOKENS[trait].borderClass;
}

/**
 * Get creator description for a trait
 */
export function getTraitCreator(trait: NodeTrait): string {
  return TRAIT_TOKENS[trait].creator;
}

/**
 * Get all trait keys
 */
export function getAllTraits(): NodeTrait[] {
  return Object.keys(TRAIT_TOKENS) as NodeTrait[];
}

/**
 * Get CSS border style value
 */
export function getTraitBorderStyle(trait: NodeTrait): string {
  const token = TRAIT_TOKENS[trait];
  return `${token.borderWidth}px ${token.borderStyle}`;
}

// =============================================================================
// Animation Timing Presets (for TraitGlow integration)
// =============================================================================

export const TRAIT_ANIMATION_TIMING = {
  pulse: {
    duration: 1.5,
    ease: 'easeInOut',
  },
  breathe: {
    duration: 2.5,
    ease: 'easeInOut',
  },
  colorShift: {
    duration: 3,
    ease: 'easeInOut',
  },
  flowHorizontal: {
    duration: 3,
    ease: 'linear',
  },
  rotate: {
    duration: 4,
    ease: 'linear',
  },
  static: {
    duration: 0,
    ease: 'linear',
  },
} as const;
