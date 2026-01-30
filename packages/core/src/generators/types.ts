// src/generators/types.ts
// Unified View System types for documentation generation
// v8.0.0

import type { NodeType, NodeCategory, ViewDefinition } from '../filters/types.js';
import { LAYER_COLORS as _LAYER_COLORS } from './colors.js';

// =============================================================================
// DOCUMENTATION LAYER (for Mermaid diagrams)
// =============================================================================

export interface DocLayer {
  name: string;
  nodes: NodeType[];
  color?: 'blue' | 'green' | 'orange' | 'purple' | 'red' | 'gray' | 'cyan';
  description?: string;
}

// =============================================================================
// CYPHER EXAMPLE (for documentation)
// =============================================================================

export interface CypherExample {
  name: string;
  description?: string;
  query: string;
  params?: Record<string, unknown>;
}

// =============================================================================
// DOCS SECTION (extends ViewDefinition)
// =============================================================================

export type DocCategory = 'overview' | 'generation' | 'localization' | 'semantic' | 'mining';

export interface ViewDocs {
  title: string;
  category: DocCategory;
  description: string;
  layers: DocLayer[];
  examples?: CypherExample[];
  notes?: string[];
  mermaid?: string;  // Custom Mermaid diagram (overrides auto-generated)
}

// =============================================================================
// EXTENDED VIEW DEFINITION (with docs)
// =============================================================================

export interface ExtendedViewDefinition extends ViewDefinition {
  docs?: ViewDocs;
}

// =============================================================================
// GENERATOR OUTPUT TYPES
// =============================================================================

export interface GeneratedMarkdown {
  viewId: string;
  content: string;
  generatedAt: Date;
}

export interface GeneratedCypher {
  viewId: string;
  queries: CypherExample[];
  combined: string;
}

// =============================================================================
// VIEW REGISTRY (extended with docs metadata)
// =============================================================================

export interface ExtendedViewRegistry {
  version: string;
  generated_at?: string;
  categories: Record<DocCategory, string[]>;  // category -> view ids
  views: Array<{
    id: string;
    file: string;
    description: string;
    category?: DocCategory;
  }>;
}

// =============================================================================
// GENERATOR OPTIONS
// =============================================================================

export interface MarkdownGeneratorOptions {
  includeTimestamp?: boolean;
  includeMermaid?: boolean;
  includeCypher?: boolean;
  includeNodeTables?: boolean;
  baseUrl?: string;  // For cross-references
  // MermaidGenerator integration
  relationsPath?: string;  // path to relations.yaml (for useFullGraphMermaid)
  indexPath?: string;      // path to _index.yaml (for useFullGraphMermaid)
  useFullGraphMermaid?: boolean;  // use MermaidGenerator for complete diagram
}

export interface CypherExporterOptions {
  includeParams?: boolean;
  includeComments?: boolean;
  format?: 'single' | 'separate';  // single file or per-query
}

// =============================================================================
// MERMAID THEME COLORS - Re-exported from colors.ts (single source of truth)
// =============================================================================

/**
 * Layer colors with just fill value (for simple use cases).
 * For full fill+stroke, import LAYER_COLORS from colors.ts directly.
 */
export const LAYER_COLORS: Record<NonNullable<DocLayer['color']>, string> = {
  blue: _LAYER_COLORS.blue.fill,
  green: _LAYER_COLORS.green.fill,
  orange: _LAYER_COLORS.orange.fill,
  purple: _LAYER_COLORS.purple.fill,
  red: _LAYER_COLORS.red.fill,
  gray: _LAYER_COLORS.gray.fill,
  cyan: _LAYER_COLORS.cyan.fill,
};

// =============================================================================
// NODE CATEGORY DISPLAY INFO
// =============================================================================

export const CATEGORY_DISPLAY: Record<NodeCategory, { label: string; emoji: string }> = {
  project: { label: 'Project', emoji: '🏢' },
  content: { label: 'Content', emoji: '📄' },
  locale: { label: 'Locale', emoji: '🌍' },
  generation: { label: 'Generation', emoji: '⚡' },
  seo: { label: 'SEO', emoji: '🔍' },
  geo: { label: 'GEO', emoji: '📍' },
};
