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
  overview: {
    id: 'overview',
    label: 'Overview',
    icon: Crosshair,
    color: '#34d399', // emerald-400
    description: 'Filter by realm and visibility',
  },
  generation: {
    id: 'generation',
    label: 'Generation',
    icon: Sparkles,
    color: '#fbbf24', // amber-400
    description: 'Content generation workflows',
  },
  knowledge: {
    id: 'knowledge',
    label: 'Knowledge',
    icon: BookOpen,
    color: '#a78bfa', // violet-400
    description: 'Knowledge graph exploration',
  },
  project: {
    id: 'project',
    label: 'Project',
    icon: FolderKanban,
    color: '#60a5fa', // blue-400
    description: 'Project-specific views',
  },
  mining: {
    id: 'mining',
    label: 'Mining',
    icon: Pickaxe,
    color: '#fb7185', // rose-400
    description: 'SEO/GEO mining operations',
  },
};

// Ordered list for display
export const CATEGORY_ORDER: ViewCategory[] = [
  'overview',
  'generation',
  'knowledge',
  'project',
  'mining',
];

// Display names map (for API routes)
export const CATEGORY_NAMES: Record<ViewCategory, string> = Object.fromEntries(
  Object.entries(VIEW_CATEGORIES).map(([id, config]) => [id, config.label])
) as Record<ViewCategory, string>;
