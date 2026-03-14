'use client';

/**
 * CategoryIcon - Memory-efficient SVG icon system using Lucide
 * Source: packages/core/models/visual-encoding.yaml
 *
 * Features:
 * - Tree-shakeable (only imports used icons)
 * - No memory leaks (pure React components)
 * - One icon per layer/realm (+ legacy trait icons)
 * - Consistent styling with glow effects
 *
 * Layers (v11.7 - 10 layers across 2 realms):
 * SHARED (4 layers):
 * - config: Settings, locale: Globe, geography: Map, knowledge: BookOpen
 * ORG (6 layers):
 * - foundation: Package, structure: LayoutGrid, semantic: Lightbulb
 * - instruction: FileText, output: Sparkles
 *
 * Traits — DEPRECATED in v0.19.0 (ADR-024):
 * TraitIcon component retained for backward compatibility.
 * - defined: Lock, authored: Pen, imported: Download
 * - generated: Sparkles, retrieved: CloudDownload
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
  Globe,
  Building2,
  Lock,
  Pen,           // v11.8: authored trait (was: localized → Globe)
  Download,      // v11.8: imported trait (was: knowledge → Brain)
  CloudDownload, // v11.8: retrieved trait (was: aggregated → Calculator)
  Map,
  type LucideProps,
} from 'lucide-react';
import type { Layer, Realm } from '@novanet/core/types';

// =============================================================================
// Icon Map - Statically defined for tree-shaking
// =============================================================================

/**
 * Layer to Lucide icon component mapping (v11.7 - 10 layers)
 * Source: packages/core/models/visual-encoding.yaml
 * Each layer has ONE representative icon
 */
const LAYER_ICONS: Record<Layer, React.ComponentType<LucideProps>> = {
  // Shared realm (4) — v11.5
  config: Settings,       // ⚙ settings
  locale: Globe,          // ⊕ globe
  geography: Map,         // ⊙ map
  knowledge: BookOpen,    // ◈ book-open
  // Org realm (6) — v11.5
  foundation: Package,    // ▣ landmark (using Package as Landmark not imported)
  structure: LayoutGrid,  // ▤ layout
  semantic: Lightbulb,    // ◆ lightbulb
  instruction: FileText,  // ▧ file-text
  output: Sparkles,       // ● check-circle (using Sparkles for visual distinction)
};

/**
 * Realm to Lucide icon component mapping (v11.2 - 2 realms)
 */
const REALM_ICONS: Record<Realm, React.ComponentType<LucideProps>> = {
  shared: Globe,
  org: Building2,
};

/**
 * Trait to Lucide icon component mapping (v11.8 - 5 traits)
 * v11.8: Renamed per ADR-024 Data Origin semantics
 * Source: packages/core/models/visual-encoding.yaml
 */
export type Trait = 'defined' | 'authored' | 'imported' | 'generated' | 'retrieved';

const TRAIT_ICONS: Record<Trait, React.ComponentType<LucideProps>> = {
  defined: Lock,      // ■ lock - Structurally fixed definitions (was: invariant)
  authored: Pen,      // □ pen - Human-authored content (was: localized)
  imported: Download, // ◊ download - External authoritative data (was: knowledge)
  generated: Sparkles, // ★ sparkles - LLM-generated output
  retrieved: CloudDownload, // ▪ cloud-download - Computed/aggregated (was: aggregated)
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
 * @deprecated Traits removed from schema in v0.19.0 (ADR-024).
 * Retained for backward compatibility in existing UI surfaces.
 *
 * @example
 * <TraitIcon trait="defined" size={24} className="text-blue-500" />
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
    return <Lock style={computedStyle} {...props} />;
  }

  return <IconComponent style={computedStyle} {...props} />;
});

// =============================================================================
// Utility - Get icon component directly (for advanced use)
// =============================================================================

/**
 * Export icon map for type checking
 */
export type LayerIconMap = typeof LAYER_ICONS;
