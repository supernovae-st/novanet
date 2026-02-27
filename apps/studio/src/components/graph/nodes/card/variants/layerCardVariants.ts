/**
 * Layer Card Variants - Type-safe styling for NovaNet card components
 *
 * Inspired by CVA (class-variance-authority) pattern.
 * Uses existing clsx + tailwind-merge without additional dependencies.
 *
 * Visual Encoding (ADR-005):
 * - Layer = fill gradient (background)
 * - Realm = border color
 * - Trait = border style + animation
 */

import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { NodeLayer, NodeRealm, NodeTrait } from '../taxonomyColors';

// =============================================================================
// Utility Functions
// =============================================================================

/** Merge Tailwind classes with conflict resolution */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// =============================================================================
// Layer Gradient Variants
// =============================================================================

export const LAYER_GRADIENTS: Record<NodeLayer, string> = {
  // SHARED realm (4 layers)
  config: 'bg-gradient-to-br from-slate-900/80 to-slate-800/60',
  locale: 'bg-gradient-to-br from-violet-900/80 to-violet-800/60',
  geography: 'bg-gradient-to-br from-emerald-900/80 to-emerald-800/60',
  knowledge: 'bg-gradient-to-br from-purple-900/80 to-purple-800/60',
  // ORG realm (6 layers)
  foundation: 'bg-gradient-to-br from-blue-900/80 to-blue-800/60',
  structure: 'bg-gradient-to-br from-cyan-900/80 to-cyan-800/60',
  semantic: 'bg-gradient-to-br from-orange-900/80 to-orange-800/60',
  instruction: 'bg-gradient-to-br from-yellow-900/80 to-yellow-800/60',
  output: 'bg-gradient-to-br from-green-900/80 to-green-800/60',
};

// =============================================================================
// Realm Border Color Variants
// =============================================================================

export const REALM_BORDERS: Record<NodeRealm, string> = {
  shared: 'border-teal-500/60',
  org: 'border-sky-500/60',
};

// =============================================================================
// Trait Border Style Variants (Tailwind classes)
// =============================================================================

export const TRAIT_BORDER_CLASSES: Record<NodeTrait, string> = {
  defined: 'border-2 border-solid',
  authored: 'border-2 border-dashed',
  imported: 'border-2 border-dotted',
  generated: 'border-[3px] [border-style:double]',
  retrieved: 'border-[3px] border-dotted',
};

// =============================================================================
// Combined Card Variant Function
// =============================================================================

export interface LayerCardVariantProps {
  layer: NodeLayer;
  realm: NodeRealm;
  trait: NodeTrait;
  selected?: boolean;
  hovered?: boolean;
  className?: string;
}

/**
 * Get combined classes for a layer card based on taxonomy
 *
 * @example
 * ```tsx
 * <div className={getLayerCardClasses({
 *   layer: 'semantic',
 *   realm: 'org',
 *   trait: 'defined',
 *   selected: isSelected
 * })}>
 *   <CardContent />
 * </div>
 * ```
 */
export function getLayerCardClasses({
  layer,
  realm,
  trait,
  selected = false,
  hovered = false,
  className,
}: LayerCardVariantProps): string {
  return cn(
    // Base styles
    'relative overflow-hidden rounded-xl transition-all duration-200',
    // Layer gradient
    LAYER_GRADIENTS[layer],
    // Realm border color
    REALM_BORDERS[realm],
    // Trait border style
    TRAIT_BORDER_CLASSES[trait],
    // Interactive states
    selected && 'ring-2 ring-white/30 ring-offset-2 ring-offset-transparent',
    hovered && 'shadow-lg shadow-white/5',
    // Custom overrides
    className
  );
}

// =============================================================================
// Individual Variant Getters
// =============================================================================

/** Get gradient class for a layer */
export function getLayerGradientClass(layer: NodeLayer): string {
  return LAYER_GRADIENTS[layer];
}

/** Get border color class for a realm */
export function getRealmBorderClass(realm: NodeRealm): string {
  return REALM_BORDERS[realm];
}

/** Get border style class for a trait */
export function getTraitBorderClass(trait: NodeTrait): string {
  return TRAIT_BORDER_CLASSES[trait];
}

// =============================================================================
// Badge Variants
// =============================================================================

export const LAYER_BADGE_CLASSES: Record<NodeLayer, string> = {
  config: 'bg-slate-500/20 text-slate-300',
  locale: 'bg-violet-500/20 text-violet-300',
  geography: 'bg-emerald-500/20 text-emerald-300',
  knowledge: 'bg-purple-500/20 text-purple-300',
  foundation: 'bg-blue-500/20 text-blue-300',
  structure: 'bg-cyan-500/20 text-cyan-300',
  semantic: 'bg-orange-500/20 text-orange-300',
  instruction: 'bg-yellow-500/20 text-yellow-300',
  output: 'bg-green-500/20 text-green-300',
};

export const REALM_BADGE_CLASSES: Record<NodeRealm, string> = {
  shared: 'bg-teal-500/20 text-teal-300',
  org: 'bg-sky-500/20 text-sky-300',
};

export const TRAIT_BADGE_CLASSES: Record<NodeTrait, string> = {
  defined: 'bg-blue-500/20 text-blue-300',
  authored: 'bg-green-500/20 text-green-300',
  imported: 'bg-violet-500/20 text-violet-300',
  generated: 'bg-yellow-500/20 text-yellow-300',
  retrieved: 'bg-purple-500/20 text-purple-300',
};

/** Get badge class for a layer */
export function getLayerBadgeClass(layer: NodeLayer): string {
  return LAYER_BADGE_CLASSES[layer];
}

/** Get badge class for a realm */
export function getRealmBadgeClass(realm: NodeRealm): string {
  return REALM_BADGE_CLASSES[realm];
}

/** Get badge class for a trait */
export function getTraitBadgeClass(trait: NodeTrait): string {
  return TRAIT_BADGE_CLASSES[trait];
}

// =============================================================================
// Compound Badge Component Classes
// =============================================================================

export interface TaxonomyBadgeVariants {
  layer: NodeLayer;
  realm: NodeRealm;
  trait: NodeTrait;
  size?: 'sm' | 'md' | 'lg';
}

const BADGE_SIZES = {
  sm: 'text-[10px] px-1.5 py-0.5 gap-1',
  md: 'text-xs px-2 py-1 gap-1.5',
  lg: 'text-sm px-2.5 py-1.5 gap-2',
};

/**
 * Get classes for a taxonomy badge component
 */
export function getTaxonomyBadgeClasses({
  layer,
  realm: _realm,
  trait: _trait,
  size = 'md',
}: TaxonomyBadgeVariants): string {
  return cn(
    // Base
    'inline-flex items-center rounded-full font-medium',
    // Size
    BADGE_SIZES[size],
    // Layer background (primary identification)
    LAYER_BADGE_CLASSES[layer]
  );
}

// =============================================================================
// Icon Glow Variants
// =============================================================================

export const LAYER_ICON_GLOWS: Record<NodeLayer, string> = {
  config: 'drop-shadow-[0_0_6px_rgb(100_116_139/0.6)]',
  locale: 'drop-shadow-[0_0_6px_rgb(139_92_246/0.6)]',
  geography: 'drop-shadow-[0_0_6px_rgb(16_185_129/0.6)]',
  knowledge: 'drop-shadow-[0_0_6px_rgb(168_85_247/0.6)]',
  foundation: 'drop-shadow-[0_0_6px_rgb(59_130_246/0.6)]',
  structure: 'drop-shadow-[0_0_6px_rgb(6_182_212/0.6)]',
  semantic: 'drop-shadow-[0_0_6px_rgb(249_115_22/0.6)]',
  instruction: 'drop-shadow-[0_0_6px_rgb(234_179_8/0.6)]',
  output: 'drop-shadow-[0_0_6px_rgb(34_197_94/0.6)]',
};

/** Get icon glow class for a layer */
export function getLayerIconGlowClass(layer: NodeLayer): string {
  return LAYER_ICON_GLOWS[layer];
}
