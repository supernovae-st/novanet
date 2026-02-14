/**
 * YAML View Category Configuration (v0.12.5)
 *
 * Centralized configuration for view categories.
 * Used by both API routes and UI components.
 *
 * Categories (v0.12.5):
 * - schema: Schema exploration (Classes, ArcClasses)
 * - data: Instance exploration (Project, Locales, Geography)
 * - generation: AI agent context assembly
 * - contextual: Node-centered subgraphs
 */

import {
  Database,
  Boxes,
  Eye,
  Sparkles,
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
// - schema: violet (#8b5cf6) - matches knowledge layer color
// - data: indigo (#6366f1) - professional data color
// - generation: pink (#ec4899) - AI/LLM theme
// - contextual: slate (#94a3b8) - neutral, secondary importance
export const VIEW_CATEGORIES: Record<ViewCategory, CategoryConfig> = {
  schema: {
    id: 'schema',
    label: 'Schema',
    icon: Database,
    color: '#8b5cf6',  // violet-500 (matches LAYER_COLORS.knowledge)
    description: 'Schema exploration (Classes, ArcClasses)',
  },
  data: {
    id: 'data',
    label: 'Data',
    icon: Boxes,
    color: '#6366f1',  // indigo-500 (distinct UI color for instances)
    description: 'Instance exploration (Project, Locales, Geography)',
  },
  generation: {
    id: 'generation',
    label: 'Generation',
    icon: Sparkles,
    color: '#ec4899',  // pink-500 (AI/LLM theme)
    description: 'AI agent context assembly',
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
  'schema',
  'data',
  'generation',
  'contextual',
];

// Display names map (for API routes)
export const CATEGORY_NAMES: Record<ViewCategory, string> = Object.fromEntries(
  Object.entries(VIEW_CATEGORIES).map(([id, config]) => [id, config.label])
) as Record<ViewCategory, string>;
