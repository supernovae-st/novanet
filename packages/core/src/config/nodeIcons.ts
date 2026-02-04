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
  // PROJECT — foundation (3)
  Project: '📦',
  BrandIdentity: '🎨',
  ProjectL10n: '🌐',

  // PROJECT — structure (3)
  Page: '📄',
  Block: '🧱',
  ContentSlot: '🔲',

  // PROJECT — semantic (6)
  Concept: '💡',
  ConceptL10n: '💬',
  SearchIntent: '🔎',
  TopicCluster: '🗂️',
  AudiencePersona: '👤',
  ChannelSurface: '📡',

  // PROJECT — instruction (6)
  PageType: '📐',
  BlockType: '📋',
  PagePrompt: '📝',
  BlockPrompt: '📝',
  BlockRules: '📏',
  PromptArtifact: '📋',

  // PROJECT — output (5)
  PageL10n: '📃',
  BlockL10n: '📝',
  GenerationJob: '🚀',
  OutputArtifact: '📦',
  EvaluationSignal: '📊',

  // GLOBAL — config (1)
  Locale: '🌍',

  // GLOBAL — knowledge (14)
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

  // SHARED — seo (3)
  SEOKeywordL10n: '🔍',
  SEOKeywordMetrics: '📊',
  SEOMiningRun: '⚙️',

  // SHARED — geo (5)
  Thing: '📍',
  ThingL10n: '🌐',
  GEOSeedL10n: '🤖',
  GEOSeedMetrics: '📊',
  GEOMiningRun: '⚙️',
};
