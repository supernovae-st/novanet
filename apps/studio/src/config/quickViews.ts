/**
 * Quick Views Configuration (v8.1.0)
 *
 * Pre-defined filter presets for common NovaNet workflows.
 * Each quick view selects a specific set of node types.
 *
 * @see Task 10 - Neo4j Browser-like interactions implementation plan
 */

import type { NodeType } from '@novanet/core/types';

/**
 * Quick view definition
 */
export interface QuickView {
  id: string;
  name: string;
  description: string;
  icon: string;
  nodeTypes: NodeType[];
}

/**
 * NovaNet-specific quick views (v8.1.0 aligned)
 *
 * Categories covered:
 * - Content Pipeline: Page/Block content flow
 * - Locale Knowledge: All 15 locale-related nodes
 * - SEO/GEO Targeting: Search optimization nodes
 * - Project Structure: Business definition nodes
 * - Generation Pipeline: AI prompts and L10n outputs
 * - Full Graph: All node types
 */
export const QUICK_VIEWS: QuickView[] = [
  {
    id: 'content-pipeline',
    name: 'Content Pipeline',
    description: 'Page, Block, and L10n output flow',
    icon: '📄',
    nodeTypes: ['Page', 'PageType', 'Block', 'BlockType', 'PageL10n', 'BlockL10n'],
  },
  {
    id: 'locale-knowledge',
    name: 'Locale Knowledge',
    description: 'All 15 locale nodes: Identity, Voice, Culture, Market, Lexicon, Rules',
    icon: '🌍',
    nodeTypes: [
      'Locale',
      'LocaleIdentity',
      'LocaleVoice',
      'LocaleCulture',
      'LocaleCultureReferences',
      'LocaleMarket',
      'LocaleLexicon',
      'LocaleRulesAdaptation',
      'LocaleRulesFormatting',
      'LocaleRulesSlug',
      'Expression',
      'Reference',
      'Metaphor',
      'Pattern',
      'Constraint',
    ],
  },
  {
    id: 'seo-geo-targeting',
    name: 'SEO/GEO Targeting',
    description: 'Concepts, Keywords, and Seeds',
    icon: '🎯',
    nodeTypes: [
      'Concept',
      'ConceptL10n',
      'SEOKeywordL10n',
      'SEOKeywordMetrics',
      'SEOMiningRun',
      'GEOSeedL10n',
      'GEOSeedMetrics',
      'GEOMiningRun',
    ],
  },
  {
    id: 'project-structure',
    name: 'Project Structure',
    description: 'Project, Brand Identity, L10n',
    icon: '🏢',
    nodeTypes: ['Project', 'BrandIdentity', 'ProjectL10n'],
  },
  {
    id: 'generation-pipeline',
    name: 'Generation Pipeline',
    description: 'Prompts, Rules, and L10n outputs',
    icon: '🤖',
    nodeTypes: ['PagePrompt', 'BlockPrompt', 'BlockRules', 'PageL10n', 'BlockL10n'],
  },
  {
    id: 'semantic-content',
    name: 'Semantic Content',
    description: 'Concepts, Pages, and Blocks',
    icon: '💡',
    nodeTypes: ['Concept', 'ConceptL10n', 'Page', 'PageType', 'Block', 'BlockType'],
  },
];

/**
 * Get a quick view by ID
 */
export function getQuickViewById(id: string): QuickView | undefined {
  return QUICK_VIEWS.find((view) => view.id === id);
}
