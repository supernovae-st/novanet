import type { FilterPreset, NodeType } from '@/types';
import { ALL_NODE_TYPES, CORE_TYPES, LOCALE_TYPES } from './nodeTypes';

// Concept-related types (Concept + ConceptL10n)
const CONCEPT_TYPES: NodeType[] = ['Concept', 'ConceptL10n'];

/**
 * Built-in filter presets (v7.2.3)
 */
export const FILTER_PRESETS: FilterPreset[] = [
  // 1 - Project Overview
  {
    id: 'overview',
    name: 'Project Overview',
    description: 'Project, Pages, Blocks, and Concepts',
    icon: 'LayoutDashboard',
    shortcut: '1',
    nodeTypes: CORE_TYPES,
    locale: null,
  },

  // 2 - Full Graph
  {
    id: 'full',
    name: 'Full Graph',
    description: 'All node types (may be slow)',
    icon: 'Globe',
    shortcut: '2',
    nodeTypes: ALL_NODE_TYPES,
    locale: null,
  },

  // 3 - Core + Concepts
  {
    id: 'core-concepts',
    name: 'Core + Concepts',
    description: 'Project structure with concepts',
    icon: 'Layers',
    shortcut: '3',
    nodeTypes: [...CORE_TYPES, 'ConceptL10n'] as NodeType[],
    locale: null,
  },

  // 4 - All Locales
  {
    id: 'all-locales',
    name: 'All Locales',
    description: 'View all locale nodes',
    icon: 'Map',
    shortcut: '4',
    nodeTypes: ['Locale'] as NodeType[],
    locale: null,
  },

  // 5 - Concepts Graph
  {
    id: 'concepts',
    name: 'Concepts',
    description: 'Concept nodes and their L10n',
    icon: 'Lightbulb',
    shortcut: '5',
    nodeTypes: CONCEPT_TYPES,
    locale: null,
  },

  // 6 - Current Locale
  {
    id: 'current-locale',
    name: 'Current Locale',
    description: 'Selected locale with all knowledge',
    icon: 'Target',
    shortcut: '6',
    nodeTypes: LOCALE_TYPES,
    locale: 'CURRENT', // Special value - uses selectedLocale
  },

  // 7 - Locale + Expressions
  {
    id: 'locale-expressions',
    name: 'Locale + Expressions',
    description: 'Current locale with all expressions',
    icon: 'MessageSquare',
    shortcut: '7',
    nodeTypes: LOCALE_TYPES, // Expression is in LOCALE_TYPES
    locale: 'CURRENT',
  },

  // 8 - Locale Knowledge
  {
    id: 'locale-knowledge',
    name: 'Locale Knowledge',
    description: 'Locale knowledge nodes (Identity, Voice, Culture, etc.)',
    icon: 'Network',
    shortcut: '8',
    nodeTypes: LOCALE_TYPES.filter((t: NodeType) => t !== 'Expression'),
    locale: null,
  },

  // 9 - Expressions Only
  {
    id: 'expressions',
    name: 'Expressions',
    description: 'All expressions',
    icon: 'Quote',
    shortcut: '9',
    nodeTypes: ['Expression', 'LocaleLexicon'] as NodeType[],
    locale: null,
  },

  // 0 - Clear / Reset
  {
    id: 'clear',
    name: 'Clear Filters',
    description: 'Reset to default view',
    icon: 'X',
    shortcut: '0',
    nodeTypes: CORE_TYPES,
    locale: null,
  },
];

/**
 * Default preset
 */
export const DEFAULT_PRESET = FILTER_PRESETS[0]; // overview
