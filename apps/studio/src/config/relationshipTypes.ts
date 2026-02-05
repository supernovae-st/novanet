// =============================================================================
// RELATIONSHIP TYPE CONFIGURATION (v8.1.0)
// =============================================================================
// Visual configuration for all 50 NovaNet relationship types
// Mirrors nodeTypes.ts pattern for consistency

import { RelationType } from '@novanet/core/schemas/relations.schema';

// =============================================================================
// RELATIONSHIP CATEGORIES (8 categories, 50 relations)
// =============================================================================

/**
 * Relationship category type (8 semantic categories)
 * Used for grouping relations in UI and filtering
 */
export type RelationshipCategory =
  | 'ownership'      // Project owns entities
  | 'localization'   // Locale assignment
  | 'knowledge'      // Locale knowledge graph
  | 'semantic'       // Concept usage
  | 'structure'      // Hierarchical structure
  | 'generation'     // Content generation
  | 'optimization'   // SEO/GEO targeting
  | 'navigation';    // Page linking & history

/**
 * Relationship categories with their types
 */
export const RELATIONSHIP_CATEGORIES: Record<RelationshipCategory, RelationType[]> = {
  // Ownership: Project/parent owns child entities (5 relations)
  // v10.3: HAS_CONCEPT removed — Entity in global realm, use USES_ENTITY
  ownership: [
    'HAS_PAGE',
    'HAS_BRAND_IDENTITY',
    'HAS_BLOCK',
    'HAS_PROMPT',
    'HAS_RULES',
  ],
  // Localization: Locale assignment (6 relations)
  localization: [
    'HAS_L10N',
    'FOR_LOCALE',
    'SUPPORTS_LOCALE',
    'DEFAULT_LOCALE',
    'L10N_OF',
    'HAS_LOCALIZED_CONTENT',
  ],
  // Knowledge: Locale knowledge graph (14 relations)
  knowledge: [
    'HAS_IDENTITY',
    'HAS_VOICE',
    'HAS_CULTURE',
    'HAS_MARKET',
    'HAS_LEXICON',
    'HAS_EXPRESSION',
    'HAS_RULES_ADAPTATION',
    'HAS_RULES_FORMATTING',
    'HAS_RULES_SLUG',
    'HAS_CULTURE_REFERENCES',
    'HAS_REFERENCE',
    'HAS_METAPHOR',
    'HAS_PATTERN',
    'HAS_CONSTRAINT',
  ],
  // Semantic: Entity usage (3 relations)
  semantic: [
    'USES_ENTITY',
    'SEMANTIC_LINK',
    'USED_BY',
  ],
  // Structure: Hierarchical structure (4 relations)
  structure: [
    'OF_TYPE',
    'SUBTOPIC_OF',
    'BELONGS_TO_PROJECT_L10N',
    'BLOCK_OF',
  ],
  // Generation: Content generation (6 relations)
  generation: [
    'HAS_OUTPUT',
    'ASSEMBLES',
    'GENERATED',
    'OUTPUT_OF',
    'GENERATED_FROM',
    'INFLUENCED_BY',
  ],
  // Optimization: SEO/GEO targeting (7 relations)
  optimization: [
    'HAS_SEO_TARGET',
    'HAS_GEO_TARGET',
    'TARGETS_SEO',
    'TARGETS_GEO',
    'SEO_MINES',
    'GEO_MINES',
    'HAS_METRICS',
  ],
  // Navigation: Page linking & history (4 relations)
  navigation: [
    'LINKS_TO',
    'FALLBACK_TO',
    'VARIANT_OF',
    'PREVIOUS_VERSION',
  ],
};

// =============================================================================
// RELATIONSHIP TYPE VISUAL CONFIG
// =============================================================================

/**
 * Relationship type visual configuration
 * Used for rendering in React Flow and sidebars
 */
export interface RelationshipTypeConfig {
  type: RelationType;
  label: string;
  color: string;
  category: RelationshipCategory;
}

/**
 * All relationship type configurations (49 relations)
 * v10.3: HAS_CONCEPT removed — Entity in global realm, use USES_ENTITY
 */
export const relationshipTypeConfigs: Record<RelationType, RelationshipTypeConfig> = {
  // ==========================================================================
  // OWNERSHIP CATEGORY (5 relations) - violet
  // ==========================================================================
  HAS_PAGE: {
    type: 'HAS_PAGE',
    label: 'Has Page',
    color: '#8b5cf6',
    category: 'ownership',
  },
  HAS_BRAND_IDENTITY: {
    type: 'HAS_BRAND_IDENTITY',
    label: 'Has Brand Identity',
    color: '#a78bfa',
    category: 'ownership',
  },
  HAS_BLOCK: {
    type: 'HAS_BLOCK',
    label: 'Has Block',
    color: '#7c3aed',
    category: 'ownership',
  },
  HAS_PROMPT: {
    type: 'HAS_PROMPT',
    label: 'Has Prompt',
    color: '#6d28d9',
    category: 'ownership',
  },
  HAS_RULES: {
    type: 'HAS_RULES',
    label: 'Has Rules',
    color: '#5b21b6',
    category: 'ownership',
  },

  // ==========================================================================
  // LOCALIZATION CATEGORY (6 relations) - emerald
  // ==========================================================================
  HAS_L10N: {
    type: 'HAS_L10N',
    label: 'Has L10n',
    color: '#10b981',
    category: 'localization',
  },
  FOR_LOCALE: {
    type: 'FOR_LOCALE',
    label: 'For Locale',
    color: '#34d399',
    category: 'localization',
  },
  SUPPORTS_LOCALE: {
    type: 'SUPPORTS_LOCALE',
    label: 'Supports Locale',
    color: '#6ee7b7',
    category: 'localization',
  },
  DEFAULT_LOCALE: {
    type: 'DEFAULT_LOCALE',
    label: 'Default Locale',
    color: '#059669',
    category: 'localization',
  },
  L10N_OF: {
    type: 'L10N_OF',
    label: 'L10n Of',
    color: '#047857',
    category: 'localization',
  },
  HAS_LOCALIZED_CONTENT: {
    type: 'HAS_LOCALIZED_CONTENT',
    label: 'Has Localized Content',
    color: '#065f46',
    category: 'localization',
  },

  // ==========================================================================
  // KNOWLEDGE CATEGORY (14 relations) - teal
  // ==========================================================================
  HAS_IDENTITY: {
    type: 'HAS_IDENTITY',
    label: 'Has Identity',
    color: '#14b8a6',
    category: 'knowledge',
  },
  HAS_VOICE: {
    type: 'HAS_VOICE',
    label: 'Has Voice',
    color: '#2dd4bf',
    category: 'knowledge',
  },
  HAS_CULTURE: {
    type: 'HAS_CULTURE',
    label: 'Has Culture',
    color: '#5eead4',
    category: 'knowledge',
  },
  HAS_MARKET: {
    type: 'HAS_MARKET',
    label: 'Has Market',
    color: '#99f6e4',
    category: 'knowledge',
  },
  HAS_LEXICON: {
    type: 'HAS_LEXICON',
    label: 'Has Lexicon',
    color: '#0d9488',
    category: 'knowledge',
  },
  HAS_EXPRESSION: {
    type: 'HAS_EXPRESSION',
    label: 'Has Expression',
    color: '#0f766e',
    category: 'knowledge',
  },
  HAS_RULES_ADAPTATION: {
    type: 'HAS_RULES_ADAPTATION',
    label: 'Has Adaptation Rules',
    color: '#115e59',
    category: 'knowledge',
  },
  HAS_RULES_FORMATTING: {
    type: 'HAS_RULES_FORMATTING',
    label: 'Has Formatting Rules',
    color: '#134e4a',
    category: 'knowledge',
  },
  HAS_RULES_SLUG: {
    type: 'HAS_RULES_SLUG',
    label: 'Has Slug Rules',
    color: '#042f2e',
    category: 'knowledge',
  },
  HAS_CULTURE_REFERENCES: {
    type: 'HAS_CULTURE_REFERENCES',
    label: 'Has Culture References',
    color: '#ccfbf1',
    category: 'knowledge',
  },
  HAS_REFERENCE: {
    type: 'HAS_REFERENCE',
    label: 'Has Reference',
    color: '#5eead4',
    category: 'knowledge',
  },
  HAS_METAPHOR: {
    type: 'HAS_METAPHOR',
    label: 'Has Metaphor',
    color: '#2dd4bf',
    category: 'knowledge',
  },
  HAS_PATTERN: {
    type: 'HAS_PATTERN',
    label: 'Has Pattern',
    color: '#14b8a6',
    category: 'knowledge',
  },
  HAS_CONSTRAINT: {
    type: 'HAS_CONSTRAINT',
    label: 'Has Constraint',
    color: '#0d9488',
    category: 'knowledge',
  },

  // ==========================================================================
  // SEMANTIC CATEGORY (3 relations) - amber
  // ==========================================================================
  USES_ENTITY: {
    type: 'USES_ENTITY',
    label: 'Uses Entity',
    color: '#f59e0b',
    category: 'semantic',
  },
  SEMANTIC_LINK: {
    type: 'SEMANTIC_LINK',
    label: 'Semantic Link',
    color: '#fbbf24',
    category: 'semantic',
  },
  USED_BY: {
    type: 'USED_BY',
    label: 'Used By',
    color: '#d97706',
    category: 'semantic',
  },

  // ==========================================================================
  // STRUCTURE CATEGORY (4 relations) - blue
  // ==========================================================================
  OF_TYPE: {
    type: 'OF_TYPE',
    label: 'Of Type',
    color: '#3b82f6',
    category: 'structure',
  },
  SUBTOPIC_OF: {
    type: 'SUBTOPIC_OF',
    label: 'Subtopic Of',
    color: '#60a5fa',
    category: 'structure',
  },
  BELONGS_TO_PROJECT_L10N: {
    type: 'BELONGS_TO_PROJECT_L10N',
    label: 'Belongs To Project L10n',
    color: '#2563eb',
    category: 'structure',
  },
  BLOCK_OF: {
    type: 'BLOCK_OF',
    label: 'Block Of',
    color: '#1d4ed8',
    category: 'structure',
  },

  // ==========================================================================
  // GENERATION CATEGORY (6 relations) - orange
  // ==========================================================================
  HAS_OUTPUT: {
    type: 'HAS_OUTPUT',
    label: 'Has Output',
    color: '#f97316',
    category: 'generation',
  },
  ASSEMBLES: {
    type: 'ASSEMBLES',
    label: 'Assembles',
    color: '#fb923c',
    category: 'generation',
  },
  GENERATED: {
    type: 'GENERATED',
    label: 'Generated',
    color: '#fdba74',
    category: 'generation',
  },
  OUTPUT_OF: {
    type: 'OUTPUT_OF',
    label: 'Output Of',
    color: '#ea580c',
    category: 'generation',
  },
  GENERATED_FROM: {
    type: 'GENERATED_FROM',
    label: 'Generated From',
    color: '#c2410c',
    category: 'generation',
  },
  INFLUENCED_BY: {
    type: 'INFLUENCED_BY',
    label: 'Influenced By',
    color: '#9a3412',
    category: 'generation',
  },

  // ==========================================================================
  // OPTIMIZATION CATEGORY (7 relations) - red
  // ==========================================================================
  HAS_SEO_TARGET: {
    type: 'HAS_SEO_TARGET',
    label: 'Has SEO Target',
    color: '#ef4444',
    category: 'optimization',
  },
  HAS_GEO_TARGET: {
    type: 'HAS_GEO_TARGET',
    label: 'Has GEO Target',
    color: '#f87171',
    category: 'optimization',
  },
  TARGETS_SEO: {
    type: 'TARGETS_SEO',
    label: 'Targets SEO',
    color: '#dc2626',
    category: 'optimization',
  },
  TARGETS_GEO: {
    type: 'TARGETS_GEO',
    label: 'Targets GEO',
    color: '#b91c1c',
    category: 'optimization',
  },
  SEO_MINES: {
    type: 'SEO_MINES',
    label: 'SEO Mines',
    color: '#991b1b',
    category: 'optimization',
  },
  GEO_MINES: {
    type: 'GEO_MINES',
    label: 'GEO Mines',
    color: '#7f1d1d',
    category: 'optimization',
  },
  HAS_METRICS: {
    type: 'HAS_METRICS',
    label: 'Has Metrics',
    color: '#fca5a5',
    category: 'optimization',
  },

  // ==========================================================================
  // NAVIGATION CATEGORY (4 relations) - indigo
  // ==========================================================================
  LINKS_TO: {
    type: 'LINKS_TO',
    label: 'Links To',
    color: '#6366f1',
    category: 'navigation',
  },
  FALLBACK_TO: {
    type: 'FALLBACK_TO',
    label: 'Fallback To',
    color: '#818cf8',
    category: 'navigation',
  },
  VARIANT_OF: {
    type: 'VARIANT_OF',
    label: 'Variant Of',
    color: '#a5b4fc',
    category: 'navigation',
  },
  PREVIOUS_VERSION: {
    type: 'PREVIOUS_VERSION',
    label: 'Previous Version',
    color: '#4f46e5',
    category: 'navigation',
  },
};

// =============================================================================
// VISUAL CATEGORY CONFIG (for FilterTree)
// =============================================================================

export interface RelationshipCategoryConfig {
  id: RelationshipCategory;
  label: string;
  icon: string;
  color: string;
  colorLight: string;
  relationTypes: RelationType[];
}

/**
 * Get all relation types by category
 */
export function getRelationTypesByCategory(category: RelationshipCategory): RelationType[] {
  return RELATIONSHIP_CATEGORIES[category];
}

/**
 * All visual categories with their configuration (ordered for display)
 */
export const RELATIONSHIP_VISUAL_CATEGORIES: RelationshipCategoryConfig[] = [
  {
    id: 'ownership',
    label: 'Ownership',
    icon: '📦',
    color: '#8b5cf6',
    colorLight: '#a78bfa',
    relationTypes: getRelationTypesByCategory('ownership'),
  },
  {
    id: 'localization',
    label: 'Localization',
    icon: '🌍',
    color: '#10b981',
    colorLight: '#34d399',
    relationTypes: getRelationTypesByCategory('localization'),
  },
  {
    id: 'knowledge',
    label: 'Knowledge',
    icon: '📚',
    color: '#14b8a6',
    colorLight: '#2dd4bf',
    relationTypes: getRelationTypesByCategory('knowledge'),
  },
  {
    id: 'semantic',
    label: 'Semantic',
    icon: '💡',
    color: '#f59e0b',
    colorLight: '#fbbf24',
    relationTypes: getRelationTypesByCategory('semantic'),
  },
  {
    id: 'structure',
    label: 'Structure',
    icon: '🏗️',
    color: '#3b82f6',
    colorLight: '#60a5fa',
    relationTypes: getRelationTypesByCategory('structure'),
  },
  {
    id: 'generation',
    label: 'Generation',
    icon: '🤖',
    color: '#f97316',
    colorLight: '#fb923c',
    relationTypes: getRelationTypesByCategory('generation'),
  },
  {
    id: 'optimization',
    label: 'Optimization',
    icon: '🎯',
    color: '#ef4444',
    colorLight: '#f87171',
    relationTypes: getRelationTypesByCategory('optimization'),
  },
  {
    id: 'navigation',
    label: 'Navigation',
    icon: '🔗',
    color: '#6366f1',
    colorLight: '#818cf8',
    relationTypes: getRelationTypesByCategory('navigation'),
  },
];

/**
 * Get color for a relationship type (replaces relationshipColors.ts)
 */
export function getRelationshipColor(type: string): string {
  const config = relationshipTypeConfigs[type as RelationType];
  return config?.color || '#6b7280';
}

/**
 * Get category for a relationship type
 */
export function getRelationshipCategory(type: string): RelationshipCategoryConfig | undefined {
  const config = relationshipTypeConfigs[type as RelationType];
  if (!config) return undefined;
  return RELATIONSHIP_VISUAL_CATEGORIES.find((c) => c.id === config.category);
}

/**
 * Alias for backwards compatibility with relationshipColors.ts
 */
export const RELATIONSHIP_TYPE_CONFIG = relationshipTypeConfigs;
