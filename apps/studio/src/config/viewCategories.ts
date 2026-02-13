/**
 * YAML View Category Configuration (v11.8)
 *
 * Centralized configuration for view categories.
 * Used by both API routes and UI components.
 *
 * Categories:
 * - meta: Schema exploration (Realm, Layer, Class, ArcClass)
 * - data: Instance exploration by realm/layer/purpose
 * - overlay: Schema + Data combined for debugging
 * - contextual: Node-centered subgraphs
 */

import {
  Database,
  Boxes,
  Layers,
  Eye,
  type LucideIcon,
} from 'lucide-react';
import type { ViewCategory } from '@novanet/core/filters';

// ============================================================================
// CATEGORY CONFIGURATION
// ============================================================================

export interface CategoryConfig {
  id: ViewCategory;
  label: string;
  icon: LucideIcon;
  color: string;
  description?: string;
}

// Color rationale (UI categories, not taxonomy-derived):
// - meta: violet (#8b5cf6) - matches knowledge layer color (schema = knowledge)
// - data: indigo (#6366f1) - distinct from meta, professional data color
// - overlay: orange (#f97316) - attention/debug color, matches semantic layer
// - contextual: slate (#94a3b8) - neutral, secondary importance
export const VIEW_CATEGORIES: Record<ViewCategory, CategoryConfig> = {
  meta: {
    id: 'meta',
    label: 'Schema',
    icon: Database,
    color: '#8b5cf6',  // violet-500 (matches LAYER_COLORS.knowledge)
    description: 'Schema exploration (Realm, Layer, Class, ArcClass)',
  },
  data: {
    id: 'data',
    label: 'Data',
    icon: Boxes,
    color: '#6366f1',  // indigo-500 (distinct UI color for instances)
    description: 'Instance exploration by realm, layer, or purpose',
  },
  overlay: {
    id: 'overlay',
    label: 'Overlay',
    icon: Layers,
    color: '#f97316',  // orange-500 (matches LAYER_COLORS.semantic - debugging)
    description: 'Meta + Data combined for debugging',
  },
  contextual: {
    id: 'contextual',
    label: 'Contextual',
    icon: Eye,
    color: '#94a3b8',  // slate-400 (neutral, secondary)
    description: 'Node-centered subgraphs',
  },
};

// Ordered list for display
export const CATEGORY_ORDER: ViewCategory[] = [
  'meta',
  'data',
  'overlay',
  'contextual',
];

// Display names map (for API routes)
export const CATEGORY_NAMES: Record<ViewCategory, string> = Object.fromEntries(
  Object.entries(VIEW_CATEGORIES).map(([id, config]) => [id, config.label])
) as Record<ViewCategory, string>;
