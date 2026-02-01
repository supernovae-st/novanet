// packages/core/src/graph/hierarchy.ts
// SCOPE_HIERARCHY configuration for schema visualization
// v1.0.0

import type { Scope } from '../types/nodes.js';
import type { Subcategory, ScopeDefinition, SubcategoryMeta } from './types.js';
import { getNodeTypesBySubcategory } from './subcategories.js';

// =============================================================================
// SCOPE_HIERARCHY - Complete scope hierarchy definition
// TODO(v9): Rename SCOPE_HIERARCHY -> REALM_HIERARCHY
// TODO(v9): Properties: scope->realm, subcategories->layers
// =============================================================================

/**
 * Complete scope hierarchy definition.
 * This is the single source of truth for the ontology structure.
 *
 * 3 Scopes:
 * - Project: Project-specific content and structure (5 subcategories, 14 nodes)
 * - Global: Shared across all projects - Locale knowledge (2 subcategories, 15 nodes)
 * - Shared: Shared across projects - SEO/GEO data (2 subcategories, 6 nodes)
 */
export const SCOPE_HIERARCHY: Record<Scope, ScopeDefinition> = {
  Project: {
    scope: 'Project',
    label: 'PROJECT',
    icon: '📦',
    description: 'Project-specific content and structure',
    subcategories: {
      foundation: {
        label: 'Foundation',
        description: 'Core project identity and brand',
        icon: '🏛️',
        nodeTypes: getNodeTypesBySubcategory('foundation'),
      },
      structure: {
        label: 'Structure',
        description: 'Page and block organization',
        icon: '🧱',
        nodeTypes: getNodeTypesBySubcategory('structure'),
      },
      semantic: {
        label: 'Semantic',
        description: 'Concepts and meaning',
        icon: '💡',
        nodeTypes: getNodeTypesBySubcategory('semantic'),
      },
      instruction: {
        label: 'Instruction',
        description: 'Prompts and rules for generation',
        icon: '📝',
        nodeTypes: getNodeTypesBySubcategory('instruction'),
      },
      output: {
        label: 'Output',
        description: 'Generated localized content',
        icon: '📄',
        nodeTypes: getNodeTypesBySubcategory('output'),
      },
    } as Record<Subcategory, SubcategoryMeta>,
  },
  Global: {
    scope: 'Global',
    label: 'GLOBAL',
    icon: '🌍',
    description: 'Shared across all projects (Locale knowledge)',
    subcategories: {
      config: {
        label: 'Configuration',
        description: 'Locale configuration',
        icon: '⚙️',
        nodeTypes: getNodeTypesBySubcategory('config'),
      },
      knowledge: {
        label: 'Knowledge',
        description: 'Locale-specific cultural/linguistic knowledge',
        icon: '🧠',
        nodeTypes: getNodeTypesBySubcategory('knowledge'),
      },
    } as Record<Subcategory, SubcategoryMeta>,
  },
  Shared: {
    scope: 'Shared',
    label: 'SHARED',
    icon: '🎯',
    description: 'Shared across projects (SEO/GEO data)',
    subcategories: {
      seo: {
        label: 'SEO',
        description: 'Search engine optimization data',
        icon: '🔍',
        nodeTypes: getNodeTypesBySubcategory('seo'),
      },
      geo: {
        label: 'GEO',
        description: 'Generative engine optimization data',
        icon: '🤖',
        nodeTypes: getNodeTypesBySubcategory('geo'),
      },
    } as Record<Subcategory, SubcategoryMeta>,
  },
};

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/**
 * Get scope definition by scope
 * @param scope - The scope to look up
 * @returns The scope definition
 */
export function getScopeDefinition(scope: Scope): ScopeDefinition {
  return SCOPE_HIERARCHY[scope];
}

/**
 * Get subcategory metadata
 * @param scope - The scope to look up
 * @param subcategory - The subcategory to look up
 * @returns The subcategory metadata, or undefined if not found
 */
export function getSubcategoryMeta(
  scope: Scope,
  subcategory: Subcategory
): SubcategoryMeta | undefined {
  return SCOPE_HIERARCHY[scope]?.subcategories[subcategory];
}

/**
 * Get all subcategories for a scope
 * @param scope - The scope to look up
 * @returns Array of subcategory names
 */
export function getSubcategoriesForScope(scope: Scope): Subcategory[] {
  return Object.keys(SCOPE_HIERARCHY[scope].subcategories) as Subcategory[];
}
