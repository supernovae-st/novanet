/**
 * Taxonomy Visual Encoding Utilities
 *
 * Maps Layer and Realm to their visual properties.
 * Source of truth: taxonomy.yaml, visual-encoding.yaml, layer/*.yaml
 *
 * Visual Channels (ADR-005):
 * - Fill color → Layer
 * - Border color → Realm (at 60% opacity)
 * - Icon → Class
 */

// =============================================================================
// Layer Colors (10 layers: 4 shared + 6 org)
// =============================================================================

export type NodeLayer =
  | 'config'
  | 'locale'
  | 'geography'
  | 'knowledge'
  | 'foundation'
  | 'structure'
  | 'semantic'
  | 'instruction'
  | 'output';

export const LAYER_COLORS: Record<NodeLayer, string> = {
  // SHARED realm (4 layers)
  config: '#64748b',     // slate-500 (gray)
  locale: '#64748b',     // slate-500 (gray)
  geography: '#10b981',  // emerald-500
  knowledge: '#8b5cf6',  // violet-500 (purple)
  // ORG realm (6 layers)
  foundation: '#3b82f6', // blue-500
  structure: '#06b6d4',  // cyan-500
  semantic: '#f97316',   // orange-500
  instruction: '#eab308', // yellow-500
  output: '#22c55e',     // green-500
};

export const LAYER_DISPLAY_NAMES: Record<NodeLayer, string> = {
  config: 'Config',
  locale: 'Locale',
  geography: 'Geography',
  knowledge: 'Knowledge',
  foundation: 'Foundation',
  structure: 'Structure',
  semantic: 'Semantic',
  instruction: 'Instruction',
  output: 'Output',
};

// =============================================================================
// Realm Colors (2 realms)
// =============================================================================

export type NodeRealm = 'shared' | 'org';

export const REALM_COLORS: Record<NodeRealm, string> = {
  shared: '#2aa198', // Solarized cyan
  org: '#6c71c4',    // Solarized violet
};

export const REALM_DISPLAY_NAMES: Record<NodeRealm, string> = {
  shared: 'Shared',
  org: 'Org',
};

// =============================================================================
// Class Icons (Lucide icon names)
// =============================================================================

export const CLASS_ICONS: Record<string, string> = {
  // Config
  Locale: 'globe',
  EntityCategory: 'tag',
  OrgConfig: 'settings',
  // Knowledge
  Formatting: 'type',
  Slugification: 'link',
  Adaptation: 'sliders',
  Style: 'palette',
  TermSet: 'book-open',
  ExpressionSet: 'message-circle',
  PatternSet: 'puzzle',
  CultureSet: 'heart',
  TabooSet: 'alert-triangle',
  AudienceSet: 'users',
  Term: 'type',
  Expression: 'message-circle',
  Pattern: 'puzzle',
  CultureRef: 'heart',
  Taboo: 'alert-triangle',
  AudienceTrait: 'user',
  SEOKeyword: 'search',
  SEOKeywordMetrics: 'bar-chart-2',
  SEOCluster: 'network',
  GEOQuery: 'map-pin',
  GEOAnswer: 'message-square',
  // Semantic
  Entity: 'lightbulb',
  EntityNative: 'globe',
  AudiencePersona: 'user',
  ChannelSurface: 'monitor',
  // Foundation
  Project: 'folder',
  Brand: 'palette',
  BrandDesign: 'brush',
  BrandPrinciples: 'heart-handshake',
  PromptStyle: 'sparkles',
  ProjectNative: 'languages',
  // Structure
  Page: 'file-text',
  Block: 'square',
  ContentSlot: 'inbox',
  // Instruction
  BlockInstruction: 'terminal',
  BlockType: 'component',
  PromptArtifact: 'file-code',
  // Output
  PageNative: 'file-check',
  BlockNative: 'check-square',
  OutputArtifact: 'package',
};

// =============================================================================
// Helper Functions
// =============================================================================

/**
 * Get layer color with optional opacity
 */
export function getLayerColor(layer: NodeLayer, opacity = 1): string {
  return hexToRgba(LAYER_COLORS[layer], opacity);
}

/**
 * Get realm color with optional opacity (default 60% per ADR-005)
 */
export function getRealmColor(realm: NodeRealm, opacity = 0.6): string {
  return hexToRgba(REALM_COLORS[realm], opacity);
}

/**
 * Convert hex color to rgba
 */
export function hexToRgba(hex: string, alpha: number): string {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

/**
 * Get Lucide icon name for a class
 */
export function getClassIcon(className: string): string {
  return CLASS_ICONS[className] || 'box'; // Default to 'box' if not found
}

// =============================================================================
// Taxonomy Context Type (for components)
// =============================================================================

export interface TaxonomyContext {
  layer: NodeLayer;
  realm: NodeRealm;
  className: string;
}

/**
 * Get all visual encoding properties for a node
 */
export function getTaxonomyColors(ctx: TaxonomyContext) {
  return {
    // Layer = fill color
    fillColor: LAYER_COLORS[ctx.layer],
    layerName: LAYER_DISPLAY_NAMES[ctx.layer],
    // Realm = border color (60% opacity)
    realmColor: getRealmColor(ctx.realm),
    realmName: REALM_DISPLAY_NAMES[ctx.realm],
    // Class = icon
    classIcon: getClassIcon(ctx.className),
  };
}
