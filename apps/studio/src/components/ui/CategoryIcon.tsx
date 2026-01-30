'use client';

/**
 * CategoryIcon - Memory-efficient SVG icon system using Lucide (v8.1.0)
 *
 * Features:
 * - Tree-shakeable (only imports used icons)
 * - No memory leaks (pure React components)
 * - One icon per category (6 categories in v8.1.0)
 * - Consistent styling with glow effects
 *
 * Categories:
 * - project: Package/folder structure
 * - content: Semantic content (lightbulb)
 * - locale: Global/languages
 * - generation: AI/magic (sparkles)
 * - seo: Search optimization
 * - geo: Target/geolocation
 */

import { memo, useMemo } from 'react';
import {
  Package,
  Lightbulb,
  Globe,
  Sparkles,
  Search,
  Target,
  type LucideProps,
} from 'lucide-react';
import type { NodeCategory } from '@/config/nodeTypes';

// =============================================================================
// Icon Map - Statically defined for tree-shaking
// =============================================================================

/**
 * Category to Lucide icon component mapping (v8.1.0 - 6 categories)
 * Each category has ONE representative icon
 */
const CATEGORY_ICONS: Record<NodeCategory, React.ComponentType<LucideProps>> = {
  project: Package,
  content: Lightbulb,
  locale: Globe,
  generation: Sparkles,
  seo: Search,
  geo: Target,
};

// =============================================================================
// Component Props
// =============================================================================

export interface CategoryIconProps extends Omit<LucideProps, 'ref'> {
  /** The node category to display icon for */
  category: NodeCategory;
  /** Optional glow effect with category color */
  glow?: boolean;
  /** Glow color (defaults to currentColor) */
  glowColor?: string;
}

// =============================================================================
// Component
// =============================================================================

/**
 * CategoryIcon - Renders the appropriate Lucide icon for a node category
 *
 * @example
 * <CategoryIcon category="content" size={24} className="text-amber-500" />
 * <CategoryIcon category="locale" glow glowColor="#10b981" />
 */
export const CategoryIcon = memo(function CategoryIcon({
  category,
  glow = false,
  glowColor,
  style,
  ...props
}: CategoryIconProps) {
  const IconComponent = CATEGORY_ICONS[category];

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
    // Fallback for unknown category
    return <Package style={computedStyle} {...props} />;
  }

  return <IconComponent style={computedStyle} {...props} />;
});

// =============================================================================
// Utility - Get icon component directly (for advanced use)
// =============================================================================

/**
 * Get the Lucide icon component for a category
 * Useful when you need the component reference, not a rendered element
 */
export function getCategoryIconComponent(category: NodeCategory) {
  return CATEGORY_ICONS[category] || Package;
}

/**
 * Export icon map for type checking
 */
export type CategoryIconMap = typeof CATEGORY_ICONS;
