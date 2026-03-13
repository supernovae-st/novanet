// =============================================================================
// RELATIONSHIP TYPE CONFIGURATION (v0.12.4)
// =============================================================================
// Visual configuration for all 53 NovaNet relationship types
// Mirrors nodeTypes.ts pattern for consistency

import { RelationType } from '@novanet/core/schemas/relations.schema';

// =============================================================================
// RELATIONSHIP CATEGORIES (8 categories, 53 relations)
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
  // Ownership: Project/parent owns child entities (8 relations)
  // v0.12.4: HAS_BRAND_IDENTITY → HAS_BRAND + Brand Architecture arcs
  ownership: [
    'HAS_PAGE',
    'HAS_BRAND',
    'HAS_DESIGN',
    'HAS_PRINCIPLES',
    'HAS_PROMPT_STYLE',
    'HAS_BLOCK',
    'HAS_INSTRUCTION',
  ],
  // Localization: Locale assignment (5 relations)
  localization: [
    'HAS_NATIVE',
    'NATIVE_OF',
    'FOR_LOCALE',
    'SUPPORTS_LOCALE',
    'DEFAULT_LOCALE',
  ],
  // Knowledge: Locale knowledge graph (13 relations — v0.18.0: HAS_MARKET removed)
  knowledge: [
    'HAS_IDENTITY',
    'HAS_VOICE',
    'HAS_CULTURE',
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
  // Semantic: Entity usage (6 relations — v11.1: +BELONGS_TO, v0.12.4: +REFERENCES, +POPULAR_IN. v0.16: -HAS_KEYWORD per brainstorm decision)
  semantic: [
    'USES_ENTITY',
    'SEMANTIC_LINK',
    'USED_BY',
    'BELONGS_TO',
    'REFERENCES',
    'POPULAR_IN',
  ],
  // Structure: Hierarchical structure (4 relations)
  // v0.12.4: HAS_STRUCTURE removed per ADR-028
  structure: [
    'OF_TYPE',
    'SUBTOPIC_OF',
    'BELONGS_TO_PROJECT_CONTENT',
    'BLOCK_OF',
  ],
  // Generation: Content generation (6 relations)
  // v10.9.0: OUTPUT_OF → NATIVE_OF (ADR-014)
  generation: [
    'HAS_NATIVE',
    'ASSEMBLES',
    'GENERATED',
    'NATIVE_OF',
    'GENERATED_FROM',
    'INFLUENCED_BY',
  ],
  // Optimization: SEO/GEO targeting (5 relations)
  optimization: [
    'HAS_SEO_TARGET',
    'HAS_GEO_TARGET',
    'TARGETS_SEO',
    'TARGETS_GEO',
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
 * All relationship type configurations (52 relations)
 * v10.3: HAS_CONCEPT removed — Entity in shared realm, use USES_ENTITY
 * v11.1: BELONGS_TO added — Entity → EntityCategory semantic classification
 * v0.12.4: REFERENCES, POPULAR_IN added (ADR-028)
 * v0.16: HAS_KEYWORD removed — use TARGETS on EntityNative instead
 */
export const relationshipTypeConfigs: Record<RelationType, RelationshipTypeConfig> = {
  // ==========================================================================
  // OWNERSHIP CATEGORY (8 relations) - blue (from taxonomy.yaml)
  // ==========================================================================
  HAS_PAGE: {
    type: 'HAS_PAGE',
    label: 'Has Page',
    color: '#3b82f6',  // blue-500
    category: 'ownership',
  },
  // v0.12.4: HAS_BRAND_IDENTITY → HAS_BRAND + Brand Architecture arcs (ADR-028)
  HAS_BRAND: {
    type: 'HAS_BRAND',
    label: 'Has Brand',
    color: '#60a5fa',  // blue-400
    category: 'ownership',
  },
  HAS_DESIGN: {
    type: 'HAS_DESIGN',
    label: 'Has Design',
    color: '#93c5fd',  // blue-300
    category: 'ownership',
  },
  HAS_PRINCIPLES: {
    type: 'HAS_PRINCIPLES',
    label: 'Has Principles',
    color: '#bfdbfe',  // blue-200
    category: 'ownership',
  },
  HAS_PROMPT_STYLE: {
    type: 'HAS_PROMPT_STYLE',
    label: 'Has Prompt Style',
    color: '#dbeafe',  // blue-100
    category: 'ownership',
  },
  HAS_BLOCK: {
    type: 'HAS_BLOCK',
    label: 'Has Block',
    color: '#2563eb',  // blue-600
    category: 'ownership',
  },
  HAS_INSTRUCTION: {
    type: 'HAS_INSTRUCTION',
    label: 'Has Instruction',
    color: '#1d4ed8',  // blue-700
    category: 'ownership',
  },
  // ==========================================================================
  // LOCALIZATION CATEGORY (5 relations) - green (from taxonomy.yaml)
  // ==========================================================================
  HAS_NATIVE: {
    type: 'HAS_NATIVE',
    label: 'Has Native',  // v0.13.0: unified arc for all *Native nodes
    color: '#22c55e',  // green-500
    category: 'localization',
  },
  FOR_LOCALE: {
    type: 'FOR_LOCALE',
    label: 'For Locale',
    color: '#4ade80',  // green-400
    category: 'localization',
  },
  SUPPORTS_LOCALE: {
    type: 'SUPPORTS_LOCALE',
    label: 'Supports Locale',
    color: '#86efac',  // green-300
    category: 'localization',
  },
  DEFAULT_LOCALE: {
    type: 'DEFAULT_LOCALE',
    label: 'Default Locale',
    color: '#16a34a',  // green-600
    category: 'localization',
  },
  NATIVE_OF: {
    type: 'NATIVE_OF',
    label: 'Native Of',  // v0.13.0: unified inverse arc for all *Native nodes
    color: '#15803d',  // green-700
    category: 'localization',
  },

  // ==========================================================================
  // KNOWLEDGE CATEGORY (14 relations) - violet (matches LAYER_COLORS.knowledge)
  // ==========================================================================
  HAS_IDENTITY: {
    type: 'HAS_IDENTITY',
    label: 'Has Identity',
    color: '#8b5cf6',  // violet-500
    category: 'knowledge',
  },
  HAS_VOICE: {
    type: 'HAS_VOICE',
    label: 'Has Voice',
    color: '#a78bfa',  // violet-400
    category: 'knowledge',
  },
  HAS_CULTURE: {
    type: 'HAS_CULTURE',
    label: 'Has Culture',
    color: '#c4b5fd',  // violet-300
    category: 'knowledge',
  },
  // v0.18.0: HAS_MARKET removed (market data from external APIs)
  HAS_LEXICON: {
    type: 'HAS_LEXICON',
    label: 'Has Lexicon',
    color: '#7c3aed',  // violet-600
    category: 'knowledge',
  },
  HAS_EXPRESSION: {
    type: 'HAS_EXPRESSION',
    label: 'Has Expression',
    color: '#6d28d9',  // violet-700
    category: 'knowledge',
  },
  HAS_RULES_ADAPTATION: {
    type: 'HAS_RULES_ADAPTATION',
    label: 'Has Adaptation Rules',
    color: '#5b21b6',  // violet-800
    category: 'knowledge',
  },
  HAS_RULES_FORMATTING: {
    type: 'HAS_RULES_FORMATTING',
    label: 'Has Formatting Rules',
    color: '#4c1d95',  // violet-900
    category: 'knowledge',
  },
  HAS_RULES_SLUG: {
    type: 'HAS_RULES_SLUG',
    label: 'Has Slug Rules',
    color: '#2e1065',  // violet-950
    category: 'knowledge',
  },
  HAS_CULTURE_REFERENCES: {
    type: 'HAS_CULTURE_REFERENCES',
    label: 'Has Culture References',
    color: '#ede9fe',  // violet-100
    category: 'knowledge',
  },
  HAS_REFERENCE: {
    type: 'HAS_REFERENCE',
    label: 'Has Reference',
    color: '#c4b5fd',  // violet-300
    category: 'knowledge',
  },
  HAS_METAPHOR: {
    type: 'HAS_METAPHOR',
    label: 'Has Metaphor',
    color: '#a78bfa',  // violet-400
    category: 'knowledge',
  },
  HAS_PATTERN: {
    type: 'HAS_PATTERN',
    label: 'Has Pattern',
    color: '#8b5cf6',  // violet-500
    category: 'knowledge',
  },
  HAS_CONSTRAINT: {
    type: 'HAS_CONSTRAINT',
    label: 'Has Constraint',
    color: '#7c3aed',  // violet-600
    category: 'knowledge',
  },

  // ==========================================================================
  // SEMANTIC CATEGORY (7 relations) - orange (from taxonomy.yaml)
  // ==========================================================================
  USES_ENTITY: {
    type: 'USES_ENTITY',
    label: 'Uses Entity',
    color: '#f97316',  // orange-500
    category: 'semantic',
  },
  SEMANTIC_LINK: {
    type: 'SEMANTIC_LINK',
    label: 'Semantic Link',
    color: '#fb923c',  // orange-400
    category: 'semantic',
  },
  USED_BY: {
    type: 'USED_BY',
    label: 'Used By',
    color: '#ea580c',  // orange-600
    category: 'semantic',
  },
  BELONGS_TO: {
    type: 'BELONGS_TO',
    label: 'Belongs To',
    color: '#c2410c',  // orange-700
    category: 'semantic',
  },
  // v0.12.4: REFERENCES added per ADR-028. v0.16: HAS_KEYWORD removed (use TARGETS on EntityNative instead)
  REFERENCES: {
    type: 'REFERENCES',
    label: 'References',
    color: '#ea580c',  // orange-600
    category: 'semantic',
  },
  // v0.12.4: POPULAR_IN added per ADR-028
  POPULAR_IN: {
    type: 'POPULAR_IN',
    label: 'Popular In',
    color: '#fdba74',  // orange-300
    category: 'semantic',
  },

  // ==========================================================================
  // STRUCTURE CATEGORY (4 relations) - cyan (matches LAYER_COLORS.structure)
  // ==========================================================================
  OF_TYPE: {
    type: 'OF_TYPE',
    label: 'Of Type',
    color: '#06b6d4',  // cyan-500
    category: 'structure',
  },
  // v0.12.4: HAS_STRUCTURE removed per ADR-028
  SUBTOPIC_OF: {
    type: 'SUBTOPIC_OF',
    label: 'Subtopic Of',
    color: '#22d3ee',  // cyan-400
    category: 'structure',
  },
  BELONGS_TO_PROJECT_CONTENT: {
    type: 'BELONGS_TO_PROJECT_CONTENT',
    label: 'Belongs To Project Native',
    color: '#0891b2',  // cyan-600
    category: 'structure',
  },
  BLOCK_OF: {
    type: 'BLOCK_OF',
    label: 'Block Of',
    color: '#0e7490',  // cyan-700
    category: 'structure',
  },

  // ==========================================================================
  // GENERATION CATEGORY (5 relations) - violet (from taxonomy.yaml)
  // v0.13.0: HAS_NATIVE/NATIVE_OF moved to LOCALIZATION (unified arc)
  // ==========================================================================
  ASSEMBLES: {
    type: 'ASSEMBLES',
    label: 'Assembles',
    color: '#a78bfa',  // violet-400
    category: 'generation',
  },
  GENERATED: {
    type: 'GENERATED',
    label: 'Generated',
    color: '#c4b5fd',  // violet-300
    category: 'generation',
  },
  // v0.13.0: NATIVE_OF moved to LOCALIZATION CATEGORY
  GENERATED_FROM: {
    type: 'GENERATED_FROM',
    label: 'Generated From',
    color: '#6d28d9',  // violet-700
    category: 'generation',
  },
  INFLUENCED_BY: {
    type: 'INFLUENCED_BY',
    label: 'Influenced By',
    color: '#5b21b6',  // violet-800
    category: 'generation',
  },

  // ==========================================================================
  // OPTIMIZATION CATEGORY (5 relations) - pink (from taxonomy.yaml mining)
  // ==========================================================================
  HAS_SEO_TARGET: {
    type: 'HAS_SEO_TARGET',
    label: 'Has SEO Target',
    color: '#ec4899',  // pink-500
    category: 'optimization',
  },
  HAS_GEO_TARGET: {
    type: 'HAS_GEO_TARGET',
    label: 'Has GEO Target',
    color: '#f472b6',  // pink-400
    category: 'optimization',
  },
  TARGETS_SEO: {
    type: 'TARGETS_SEO',
    label: 'Targets SEO',
    color: '#db2777',  // pink-600
    category: 'optimization',
  },
  TARGETS_GEO: {
    type: 'TARGETS_GEO',
    label: 'Targets GEO',
    color: '#be185d',  // pink-700
    category: 'optimization',
  },
  HAS_METRICS: {
    type: 'HAS_METRICS',
    label: 'Has Metrics',
    color: '#fbcfe8',  // pink-200
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
  // Colors from taxonomy.yaml arc_families via ARC_FAMILY_COLORS (generated.ts)
  {
    id: 'ownership',
    label: 'Ownership',
    icon: 'package',
    color: '#3b82f6',  // blue-500 (from taxonomy.yaml arc_families.ownership)
    colorLight: '#60a5fa',  // blue-400
    relationTypes: getRelationTypesByCategory('ownership'),
  },
  {
    id: 'localization',
    label: 'Localization',
    icon: 'globe',
    color: '#22c55e',  // green-500 (from taxonomy.yaml arc_families.localization)
    colorLight: '#4ade80',  // green-400
    relationTypes: getRelationTypesByCategory('localization'),
  },
  {
    id: 'knowledge',
    label: 'Knowledge',
    icon: 'book-open',
    color: '#8b5cf6',  // violet-500 (matches LAYER_COLORS.knowledge)
    colorLight: '#a78bfa',  // violet-400
    relationTypes: getRelationTypesByCategory('knowledge'),
  },
  {
    id: 'semantic',
    label: 'Semantic',
    icon: 'lightbulb',
    color: '#f97316',  // orange-500 (from taxonomy.yaml arc_families.semantic)
    colorLight: '#fb923c',  // orange-400
    relationTypes: getRelationTypesByCategory('semantic'),
  },
  {
    id: 'structure',
    label: 'Structure',
    icon: 'layout-grid',
    color: '#06b6d4',  // cyan-500 (matches LAYER_COLORS.structure)
    colorLight: '#22d3ee',  // cyan-400
    relationTypes: getRelationTypesByCategory('structure'),
  },
  {
    id: 'generation',
    label: 'Generation',
    icon: 'bot',
    color: '#8b5cf6',  // violet-500 (from taxonomy.yaml arc_families.generation)
    colorLight: '#a78bfa',  // violet-400
    relationTypes: getRelationTypesByCategory('generation'),
  },
  {
    id: 'optimization',
    label: 'Optimization',
    icon: 'target',
    color: '#ec4899',  // pink-500 (from taxonomy.yaml arc_families.mining)
    colorLight: '#f472b6',  // pink-400
    relationTypes: getRelationTypesByCategory('optimization'),
  },
  {
    id: 'navigation',
    label: 'Navigation',
    icon: 'link',
    color: '#6366f1',  // indigo-500 (UI-only category, no arc family match)
    colorLight: '#818cf8',  // indigo-400
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

