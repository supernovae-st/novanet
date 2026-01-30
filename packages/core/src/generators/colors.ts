// src/generators/colors.ts
// Unified color palette for NovaNet graph visualizations
// v8.1.0
//
// SINGLE SOURCE OF TRUTH for all Mermaid diagram colors.
// Used by both complete-graph (locale_behavior) and view diagrams (layers).
// v8.1.0: Added EdgeCategory for semantic edge styling.

// =============================================================================
// TAILWIND COLOR VALUES
// Using consistent Tailwind colors across all visualizations
// =============================================================================

const TAILWIND = {
  // Blues (stroke uses 700 for better contrast)
  blue500: '#3b82f6',
  blue700: '#1d4ed8',

  // Greens
  green500: '#22c55e',
  green600: '#16a34a',

  // Violets (for locale knowledge)
  violet500: '#8b5cf6',
  violet600: '#7c3aed',

  // Oranges (for semantic/concepts)
  orange500: '#f97316',
  orange600: '#ea580c',

  // Grays
  gray400: '#9ca3af',
  gray500: '#6b7280',
  gray600: '#4b5563',

  // Reds (for errors/warnings)
  red500: '#ef4444',
  red600: '#dc2626',

  // Cyans (for config/locale)
  cyan500: '#06b6d4',
  cyan600: '#0891b2',

  // Pinks (for targeting/SEO/GEO)
  pink500: '#ec4899',
  pink600: '#db2777',
} as const;

// =============================================================================
// LOCALE BEHAVIOR COLORS (for complete-graph)
// Each locale_behavior maps to a semantic color
// =============================================================================

export type LocaleBehavior = 'invariant' | 'localized' | 'localeKnowledge' | 'derived' | 'job';

export interface ColorDef {
  fill: string;
  stroke: string;
}

/**
 * Colors for locale_behavior in complete-graph.
 * These represent the semantic meaning of each node type.
 */
export const BEHAVIOR_COLORS: Record<LocaleBehavior, ColorDef> = {
  invariant: { fill: TAILWIND.blue500, stroke: TAILWIND.blue700 },
  localized: { fill: TAILWIND.green500, stroke: TAILWIND.green600 },
  localeKnowledge: { fill: TAILWIND.violet500, stroke: TAILWIND.violet600 },
  derived: { fill: TAILWIND.gray400, stroke: TAILWIND.gray500 },
  job: { fill: TAILWIND.gray500, stroke: TAILWIND.gray600 },
};

/**
 * Mermaid classDef strings for locale_behavior.
 */
export const BEHAVIOR_STYLE: Record<LocaleBehavior, string> = {
  invariant: `fill:${BEHAVIOR_COLORS.invariant.fill},stroke:${BEHAVIOR_COLORS.invariant.stroke},color:#fff`,
  localized: `fill:${BEHAVIOR_COLORS.localized.fill},stroke:${BEHAVIOR_COLORS.localized.stroke},color:#fff`,
  localeKnowledge: `fill:${BEHAVIOR_COLORS.localeKnowledge.fill},stroke:${BEHAVIOR_COLORS.localeKnowledge.stroke},color:#fff`,
  derived: `fill:${BEHAVIOR_COLORS.derived.fill},stroke:${BEHAVIOR_COLORS.derived.stroke},color:#fff`,
  job: `fill:${BEHAVIOR_COLORS.job.fill},stroke:${BEHAVIOR_COLORS.job.stroke},color:#fff`,
};

// =============================================================================
// LAYER COLORS (for view diagrams)
// Generic color names for documentation layers
// ALIGNED with behavior colors for visual coherence
// =============================================================================

export type LayerColor = 'blue' | 'green' | 'orange' | 'purple' | 'red' | 'gray' | 'cyan';

/**
 * Colors for documentation layers in view diagrams.
 * Aligned with behavior colors:
 *   - blue → same as invariant (structure nodes)
 *   - green → same as localized (output nodes)
 *   - purple → same as localeKnowledge (knowledge nodes)
 *   - gray → same as job/derived (metrics nodes)
 *   - orange → semantic/concept nodes (unique to views)
 *   - cyan → config/locale nodes (unique to views)
 *   - red → error/warning (reserved)
 */
export const LAYER_COLORS: Record<LayerColor, ColorDef> = {
  blue: { fill: TAILWIND.blue500, stroke: TAILWIND.blue700 },      // = invariant
  green: { fill: TAILWIND.green500, stroke: TAILWIND.green600 },   // = localized
  purple: { fill: TAILWIND.violet500, stroke: TAILWIND.violet600 }, // = localeKnowledge
  gray: { fill: TAILWIND.gray500, stroke: TAILWIND.gray600 },      // = job
  orange: { fill: TAILWIND.orange500, stroke: TAILWIND.orange600 }, // semantic
  cyan: { fill: TAILWIND.cyan500, stroke: TAILWIND.cyan600 },       // config
  red: { fill: TAILWIND.red500, stroke: TAILWIND.red600 },          // error
};

/**
 * Mermaid classDef strings for layer colors.
 */
export const LAYER_STYLE: Record<LayerColor, string> = Object.fromEntries(
  Object.entries(LAYER_COLORS).map(([key, value]) => [
    key,
    `fill:${value.fill},stroke:${value.stroke},color:#fff`,
  ])
) as Record<LayerColor, string>;

// =============================================================================
// EMOJI CONSTANTS (for node labels)
// =============================================================================

/**
 * Emoji for each locale_behavior type.
 */
export const BEHAVIOR_EMOJI: Record<LocaleBehavior, string> = {
  invariant: '\u{1F535}',       // Blue circle
  localized: '\u{1F7E2}',       // Green circle
  localeKnowledge: '\u{1F7E3}', // Purple circle
  derived: '\u{26AA}',          // White circle
  job: '\u{2699}\uFE0F',        // Gear
};

/**
 * Emoji for each scope level.
 */
export const SCOPE_EMOJI: Record<'Global' | 'Shared' | 'Project', string> = {
  Global: '\u{1F30D}',  // Earth globe
  Shared: '\u{1F3AF}',  // Target
  Project: '\u{1F4E6}', // Package
};

// =============================================================================
// COLOR MAPPING UTILITIES
// =============================================================================

/**
 * Map locale_behavior to equivalent layer color.
 * Useful when you need to show behavior-colored nodes in a layer context.
 */
export const BEHAVIOR_TO_LAYER: Record<LocaleBehavior, LayerColor> = {
  invariant: 'blue',
  localized: 'green',
  localeKnowledge: 'purple',
  derived: 'gray',
  job: 'gray',
};

/**
 * Map layer color to most likely locale_behavior.
 * Note: This is approximate - multiple behaviors may map to same layer.
 */
export const LAYER_TO_BEHAVIOR: Record<LayerColor, LocaleBehavior> = {
  blue: 'invariant',
  green: 'localized',
  purple: 'localeKnowledge',
  gray: 'job',
  orange: 'invariant', // semantic nodes are typically invariant
  cyan: 'invariant',   // config nodes are typically invariant
  red: 'derived',      // errors/warnings
};

// =============================================================================
// EDGE CATEGORIES (v8.1.0 - Semantic edge styling)
// =============================================================================

/**
 * Semantic categories for edges/relations.
 * Each category has distinct Mermaid arrow style and color.
 */
export type EdgeCategory =
  | 'ownership'      // HAS_PAGE, HAS_BLOCK, HAS_CONCEPT - structural containment
  | 'localization'   // FOR_LOCALE, HAS_L10N, HAS_OUTPUT - locale binding
  | 'generation'     // GENERATED, ASSEMBLES, HAS_PROMPT - AI output
  | 'semantic'       // SEMANTIC_LINK, USES_CONCEPT - conceptual relationships
  | 'targeting'      // TARGETS_SEO, HAS_SEO_TARGET - SEO/GEO targeting
  | 'inverse'        // L10N_OF, OUTPUT_OF, BLOCK_OF - reverse traversal
  | 'hierarchy';     // OF_TYPE, HAS_RULES, SUBTOPIC_OF - type/rule hierarchy

/**
 * Mermaid arrow syntax for each edge category.
 * Format: `A ${arrow}|label| B`
 *
 * Produces visually distinct edge styles:
 * - `-->` solid arrow (ownership, hierarchy)
 * - `-.->` dashed arrow (localization, semantic)
 * - `==>` thick arrow (generation)
 * - `--o` circle end (targeting)
 */
export const EDGE_ARROWS: Record<EdgeCategory, string> = {
  ownership:     '-->',    // solid - structural containment
  localization:  '-.->',   // dashed - locale binding
  generation:    '==>',    // thick - AI output
  semantic:      '-.->',   // dashed - conceptual (same as localization)
  targeting:     '--o',    // circle end - SEO/GEO targeting
  inverse:       '-->',    // solid - reverse traversal (semantically inverse, visually same)
  hierarchy:     '-->',    // solid - type/rule hierarchy (same as ownership)
};

/**
 * Colors for each edge category.
 * Used with Mermaid linkStyle for colored edges.
 */
export const EDGE_COLORS: Record<EdgeCategory, string> = {
  ownership:    TAILWIND.blue500,    // blue - structure
  localization: TAILWIND.green500,   // green - locale binding
  generation:   TAILWIND.violet500,  // purple - AI output
  semantic:     TAILWIND.orange500,  // orange - concepts
  targeting:    TAILWIND.pink500,    // pink - SEO/GEO
  inverse:      TAILWIND.gray500,    // gray - reverse traversal
  hierarchy:    TAILWIND.cyan500,    // cyan - types/rules
};

/**
 * Map each relation type to its semantic category.
 * All 50 relations from relations.schema.ts are mapped.
 */
export const EDGE_TO_CATEGORY: Record<string, EdgeCategory> = {
  // Ownership (structural containment) - 19 relations
  HAS_CONCEPT: 'ownership',
  HAS_PAGE: 'ownership',
  HAS_BRAND_IDENTITY: 'ownership',
  HAS_BLOCK: 'ownership',
  HAS_IDENTITY: 'ownership',
  HAS_VOICE: 'ownership',
  HAS_CULTURE: 'ownership',
  HAS_MARKET: 'ownership',
  HAS_LEXICON: 'ownership',
  HAS_EXPRESSION: 'ownership',
  HAS_CULTURE_REFERENCES: 'ownership',
  HAS_REFERENCE: 'ownership',
  HAS_METAPHOR: 'ownership',
  HAS_PATTERN: 'ownership',
  HAS_CONSTRAINT: 'ownership',
  HAS_RULES_ADAPTATION: 'ownership',
  HAS_RULES_FORMATTING: 'ownership',
  HAS_RULES_SLUG: 'ownership',
  HAS_METRICS: 'ownership',

  // Localization (locale binding) - 8 relations
  FOR_LOCALE: 'localization',
  HAS_L10N: 'localization',
  HAS_OUTPUT: 'localization',
  SUPPORTS_LOCALE: 'localization',
  DEFAULT_LOCALE: 'localization',
  FALLBACK_TO: 'localization',
  VARIANT_OF: 'localization',
  HAS_LOCALIZED_CONTENT: 'localization',

  // Generation (AI output) - 6 relations
  GENERATED: 'generation',
  ASSEMBLES: 'generation',
  HAS_PROMPT: 'generation',
  GENERATED_FROM: 'generation',
  INFLUENCED_BY: 'generation',
  PREVIOUS_VERSION: 'generation',

  // Semantic (concepts) - 3 relations
  SEMANTIC_LINK: 'semantic',
  USES_CONCEPT: 'semantic',
  LINKS_TO: 'semantic',

  // Targeting (SEO/GEO) - 6 relations
  TARGETS_SEO: 'targeting',
  TARGETS_GEO: 'targeting',
  HAS_SEO_TARGET: 'targeting',
  HAS_GEO_TARGET: 'targeting',
  SEO_MINES: 'targeting',
  GEO_MINES: 'targeting',

  // Inverse (reverse traversal) - 5 relations
  L10N_OF: 'inverse',
  OUTPUT_OF: 'inverse',
  BLOCK_OF: 'inverse',
  USED_BY: 'inverse',
  BELONGS_TO_PROJECT_L10N: 'inverse',

  // Hierarchy (types/rules) - 3 relations
  OF_TYPE: 'hierarchy',
  HAS_RULES: 'hierarchy',
  SUBTOPIC_OF: 'hierarchy',
};
