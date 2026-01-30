/**
 * YAML View Category Configuration
 *
 * Centralized configuration for view categories.
 * Used by both API routes and UI components.
 */

import {
  Crosshair,
  Sparkles,
  BookOpen,
  FolderKanban,
  Pickaxe,
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
  scope: {
    id: 'scope',
    label: 'Scope Layers',
    icon: Crosshair,
    color: 'text-emerald-400',
    description: 'Filter by scope and visibility',
  },
  generation: {
    id: 'generation',
    label: 'Generation',
    icon: Sparkles,
    color: 'text-amber-400',
    description: 'Content generation workflows',
  },
  knowledge: {
    id: 'knowledge',
    label: 'Knowledge',
    icon: BookOpen,
    color: 'text-violet-400',
    description: 'Knowledge graph exploration',
  },
  project: {
    id: 'project',
    label: 'Project',
    icon: FolderKanban,
    color: 'text-blue-400',
    description: 'Project-specific views',
  },
  mining: {
    id: 'mining',
    label: 'Mining',
    icon: Pickaxe,
    color: 'text-rose-400',
    description: 'SEO/GEO mining operations',
  },
};

// Ordered list for display
export const CATEGORY_ORDER: ViewCategory[] = [
  'scope',
  'generation',
  'knowledge',
  'project',
  'mining',
];

// Helper to get config by ID
export function getCategoryConfig(id: ViewCategory): CategoryConfig {
  return VIEW_CATEGORIES[id];
}

// Display names map (for API routes)
export const CATEGORY_NAMES: Record<ViewCategory, string> = Object.fromEntries(
  Object.entries(VIEW_CATEGORIES).map(([id, config]) => [id, config.label])
) as Record<ViewCategory, string>;
