'use client';

/**
 * LayerIcon - Memory-efficient SVG icon system using Lucide (v10.5.0)
 *
 * Features:
 * - Tree-shakeable (only imports used icons)
 * - No memory leaks (pure React components)
 * - One icon per layer (9 layers in v10.5)
 * - Consistent styling with glow effects
 *
 * Layers (v11.0 - 9 layers across 2 realms):
 * GLOBAL (2 layers):
 * - config: Settings (locale configuration)
 * - locale-knowledge: BookOpen (knowledge atoms)
 * TENANT (7 layers):
 * - config: Settings (organization)
 * - foundation: Package (project structure)
 * - structure: Layout (pages, blocks)
 * - semantic: Lightbulb (entities)
 * - instruction: FileText (prompts, rules)
 * - seo: Search (SEO optimization - moved to tenant in v11.0)
 * - output: Sparkles (generated content)
 */

import { memo, useMemo } from 'react';
import {
  Package,
  LayoutGrid,
  Lightbulb,
  FileText,
  Sparkles,
  Settings,
  BookOpen,
  Search,
  Globe,
  Building2,
  Square,
  SquareDashed,
  Gem,
  Zap,
  Layers,
  type LucideProps,
} from 'lucide-react';
import type { Layer, Realm } from '@novanet/core/types';

// =============================================================================
// Icon Map - Statically defined for tree-shaking
// =============================================================================

/**
 * Layer to Lucide icon component mapping (v11.4 - 10 layers)
 * Each layer has ONE representative icon
 */
const LAYER_ICONS: Record<Layer, React.ComponentType<LucideProps>> = {
  // Shared realm (4) — v11.4: includes config
  config: Settings,
  locale: Settings,
  geography: BookOpen,
  knowledge: BookOpen,  // v11.4: includes SEO/GEO nodes
  // Org realm (6) — v11.4: seo/geo removed
  foundation: Package,
  structure: LayoutGrid,
  semantic: Lightbulb,
  instruction: FileText,
  output: Sparkles,
};

/**
 * Realm to Lucide icon component mapping (v11.2 - 2 realms)
 */
const REALM_ICONS: Record<Realm, React.ComponentType<LucideProps>> = {
  shared: Globe,
  org: Building2,
};

/**
 * Trait to Lucide icon component mapping (v11.2 - 5 traits)
 */
export type Trait = 'invariant' | 'localized' | 'knowledge' | 'generated' | 'aggregated';

const TRAIT_ICONS: Record<Trait, React.ComponentType<LucideProps>> = {
  invariant: Square,
  localized: SquareDashed,
  knowledge: Gem,
  generated: Zap,
  aggregated: Layers,
};

// =============================================================================
// Component Props
// =============================================================================

export interface LayerIconProps extends Omit<LucideProps, 'ref'> {
  /** The layer to display icon for */
  layer: Layer;
  /** Optional glow effect with layer color */
  glow?: boolean;
  /** Glow color (defaults to currentColor) */
  glowColor?: string;
}

export interface RealmIconProps extends Omit<LucideProps, 'ref'> {
  /** The realm to display icon for */
  realm: Realm;
  /** Optional glow effect */
  glow?: boolean;
  /** Glow color (defaults to currentColor) */
  glowColor?: string;
}

export interface TraitIconProps extends Omit<LucideProps, 'ref'> {
  /** The trait to display icon for */
  trait: Trait;
  /** Optional glow effect */
  glow?: boolean;
  /** Glow color (defaults to currentColor) */
  glowColor?: string;
}

// =============================================================================
// Component
// =============================================================================

/**
 * LayerIcon - Renders the appropriate Lucide icon for a layer
 *
 * @example
 * <LayerIcon layer="semantic" size={24} className="text-amber-500" />
 * <LayerIcon layer="config" glow glowColor="#10b981" />
 */
export const LayerIcon = memo(function LayerIcon({
  layer,
  glow = false,
  glowColor,
  style,
  ...props
}: LayerIconProps) {
  const IconComponent = LAYER_ICONS[layer];

  // Memoize glow style to prevent re-renders
  const computedStyle = useMemo(() => {
    if (glow && glowColor) {
      return {
        filter: `drop-shadow(0 0 6px ${glowColor}80) drop-shadow(0 0 12px ${glowColor}40)`,
        ...style,
      };
    }
    return style;
  }, [glow, glowColor, style]);

  if (!IconComponent) {
    // Fallback for unknown layer
    return <Package style={computedStyle} {...props} />;
  }

  return <IconComponent style={computedStyle} {...props} />;
});

/**
 * RealmIcon - Renders the appropriate Lucide icon for a realm
 *
 * @example
 * <RealmIcon realm="shared" size={24} className="text-teal-500" />
 * <RealmIcon realm="org" glow glowColor="#6c71c4" />
 */
export const RealmIcon = memo(function RealmIcon({
  realm,
  glow = false,
  glowColor,
  style,
  ...props
}: RealmIconProps) {
  const IconComponent = REALM_ICONS[realm];

  const computedStyle = useMemo(() => {
    if (glow && glowColor) {
      return {
        filter: `drop-shadow(0 0 6px ${glowColor}80) drop-shadow(0 0 12px ${glowColor}40)`,
        ...style,
      };
    }
    return style;
  }, [glow, glowColor, style]);

  if (!IconComponent) {
    return <Globe style={computedStyle} {...props} />;
  }

  return <IconComponent style={computedStyle} {...props} />;
});

/**
 * TraitIcon - Renders the appropriate Lucide icon for a trait
 *
 * @example
 * <TraitIcon trait="invariant" size={24} className="text-blue-500" />
 * <TraitIcon trait="generated" glow glowColor="#b58900" />
 */
export const TraitIcon = memo(function TraitIcon({
  trait,
  glow = false,
  glowColor,
  style,
  ...props
}: TraitIconProps) {
  const IconComponent = TRAIT_ICONS[trait];

  const computedStyle = useMemo(() => {
    if (glow && glowColor) {
      return {
        filter: `drop-shadow(0 0 6px ${glowColor}80) drop-shadow(0 0 12px ${glowColor}40)`,
        ...style,
      };
    }
    return style;
  }, [glow, glowColor, style]);

  if (!IconComponent) {
    return <Square style={computedStyle} {...props} />;
  }

  return <IconComponent style={computedStyle} {...props} />;
});

// =============================================================================
// Utility - Get icon component directly (for advanced use)
// =============================================================================

/**
 * Get the Lucide icon component for a layer
 * Useful when you need the component reference, not a rendered element
 */
export function getLayerIconComponent(layer: Layer) {
  return LAYER_ICONS[layer] || Package;
}

/**
 * Export icon map for type checking
 */
export type LayerIconMap = typeof LAYER_ICONS;
