// packages/core/src/config/nodeIcons.ts
// Presentation layer configuration for node icons
// Moved from database properties per YAML v7.11.0

import type { NodeType } from '../types/nodes.js';

/**
 * Node icons for UI rendering.
 * YAML v7.11.0: icon removed from standard properties (UI concern).
 * Icons are now in presentation layer, not data model.
 */
export const NODE_ICONS: Record<NodeType, string> = {
  // PROJECT SCOPE (3)
  Project: '📦',
  BrandIdentity: '🎨',
  ProjectL10n: '🌐',

  // CONTENT (6)
  Concept: '💡',
  ConceptL10n: '💬',
  Page: '📄',
  PageType: '📐',
  Block: '🧱',
  BlockType: '📋',

  // LOCALE (15)
  Locale: '🌍',
  LocaleIdentity: '🆔',
  LocaleVoice: '🎭',
  LocaleCulture: '🏛️',
  LocaleCultureReferences: '🎭',
  LocaleMarket: '📈',
  LocaleLexicon: '📚',
  LocaleRulesAdaptation: '🔄',
  LocaleRulesFormatting: '📝',
  LocaleRulesSlug: '🔗',
  Expression: '💭',
  Reference: '📍',
  Metaphor: '🎨',
  Pattern: '🔣',
  Constraint: '⚠️',

  // GENERATION (5)
  PagePrompt: '📝',
  BlockPrompt: '📝',
  BlockRules: '📏',
  PageL10n: '📃',
  BlockL10n: '📝',

  // SEO (3)
  SEOKeywordL10n: '🔍',
  SEOKeywordMetrics: '📊',
  SEOMiningRun: '⚙️',

  // GEO (3)
  GEOSeedL10n: '🤖',
  GEOSeedMetrics: '📊',
  GEOMiningRun: '⚙️',
};

/**
 * Get icon for a node type with fallback.
 */
export function getNodeIcon(type: NodeType): string {
  return NODE_ICONS[type] ?? '❓';
}
