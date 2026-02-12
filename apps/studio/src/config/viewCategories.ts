/**
 * YAML View Category Configuration (v11.6.1)
 *
 * Centralized configuration for view categories.
 * Used by both API routes and UI components.
 *
 * Categories:
 * - meta: Schema exploration (Realm, Layer, Kind, ArcKind)
 * - data: Instance exploration by realm/layer/purpose
 * - overlay: Meta + Data combined for debugging
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

export const VIEW_CATEGORIES: Record<ViewCategory, CategoryConfig> = {
  meta: {
    id: 'meta',
    label: 'Meta',
    icon: Database,
    color: '#8b5cf6', // violet-500
    description: 'Schema exploration (Realm, Layer, Kind, ArcKind)',
  },
  data: {
    id: 'data',
    label: 'Data',
    icon: Boxes,
    color: '#6366f1', // indigo-500
    description: 'Instance exploration by realm, layer, or purpose',
  },
  overlay: {
    id: 'overlay',
    label: 'Overlay',
    icon: Layers,
    color: '#f97316', // orange-500
    description: 'Meta + Data combined for debugging',
  },
  contextual: {
    id: 'contextual',
    label: 'Contextual',
    icon: Eye,
    color: '#94a3b8', // slate-400
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
