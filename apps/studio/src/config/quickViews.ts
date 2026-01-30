/**
 * Quick Views Configuration
 *
 * Pre-defined filter presets for common NovaNet workflows.
 * Each quick view selects a specific set of node types.
 *
 * @see Task 10 - Neo4j Browser-like interactions implementation plan
 */

import type { NodeType } from '@/types';

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
 * NovaNet-specific quick views (v7.2.5 aligned)
 *
 * Categories covered:
 * - Content Pipeline: Page/Block content flow
 * - Locale Knowledge: All locale-related nodes
 * - SEO/GEO Targeting: Search optimization nodes
 * - Project Structure: Business definition nodes
 * - Generation Pipeline: AI prompts and outputs
 * - Full Graph: All node types
 */
export const QUICK_VIEWS: QuickView[] = [
  {
    id: 'content-pipeline',
    name: 'Content Pipeline',
    description: 'Page, Block, and output flow',
    icon: '📄',
    nodeTypes: ['Page', 'Block', 'BlockType', 'PageOutput', 'BlockOutput'],
  },
  {
    id: 'locale-knowledge',
    name: 'Locale Knowledge',
    description: 'Identity, Voice, Culture, Market, Lexicon',
    icon: '🌍',
    nodeTypes: [
      'Locale',
      'LocaleIdentity',
      'LocaleVoice',
      'LocaleCulture',
      'LocaleMarket',
      'LocaleLexicon',
      'Expression',
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
      'SEOKeyword',
      'SEOVariation',
      'GEOSeed',
      'GEOReformulation',
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
    description: 'Prompts, Rules, and Outputs',
    icon: '🤖',
    nodeTypes: ['PagePrompt', 'BlockPrompt', 'BlockRules', 'PageOutput', 'BlockOutput'],
  },
  {
    id: 'semantic-content',
    name: 'Semantic Content',
    description: 'Concepts, Pages, and Blocks',
    icon: '💡',
    nodeTypes: ['Concept', 'ConceptL10n', 'Page', 'Block', 'BlockType'],
  },
];

/**
 * Get a quick view by ID
 */
export function getQuickViewById(id: string): QuickView | undefined {
  return QUICK_VIEWS.find((view) => view.id === id);
}
