// src/generators/MermaidGenerator.ts
// Generates Mermaid flowchart from relations.yaml and _index.yaml
// v8.1.0: Added semantic edge styling with EdgeCategory

import * as fs from 'fs/promises';
import * as path from 'path';
import * as yaml from 'yaml';
import { RelationsParser, type RelationEdge } from '../parsers/RelationsParser.js';
import { filePathToNodeName } from '../utils/filePathToNodeName.js';
import {
  BEHAVIOR_STYLE,
  BEHAVIOR_EMOJI,
  SCOPE_EMOJI,
  EDGE_ARROWS,
  EDGE_TO_CATEGORY,
  EDGE_COLORS,
  type LocaleBehavior,
  type EdgeCategory,
} from '../config/colors.js';

// =============================================================================
// TYPES
// =============================================================================

/**
 * Configuration for MermaidGenerator
 */
export interface MermaidGeneratorConfig {
  /** Path to models/ directory (contains relations.yaml and _index.yaml) */
  modelsDir: string;
  /** Optional output path for writeToFile */
  outputPath?: string;
}

/**
 * Scope classification for node grouping
 */
type Scope = 'Global' | 'Shared' | 'Project';

/**
 * Category within a scope (e.g., 'knowledge', 'foundation', 'seo')
 */
type Category = string;

/**
 * Node metadata including scope and category
 */
interface NodeMetadata {
  scope: Scope;
  category: Category;
}

/**
 * Scope subcategory with file paths
 */
interface ScopeSubcategory {
  [key: string]: string[] | undefined;
}

/**
 * Index YAML structure (partial, relevant fields only)
 */
interface IndexYaml {
  files?: {
    global?: ScopeSubcategory;
    shared?: ScopeSubcategory;
    project?: ScopeSubcategory;
  };
  nodes_by_locale_behavior?: {
    invariant?: string[];
    localized?: string[];
    localeKnowledge?: string[];
    derived?: string[];
    job?: string[];
  };
}

// =============================================================================
// CONSTANTS (from colors.ts, with additional display mappings)
// =============================================================================

/**
 * Category display names (PascalCase)
 */
const CATEGORY_DISPLAY: Record<string, string> = {
  config: 'Config',
  knowledge: 'Knowledge',
  seo: 'SEO',
  geo: 'GEO',
  foundation: 'Foundation',
  structure: 'Structure',
  semantic: 'Semantic',
  instruction: 'Instruction',
  output: 'Output',
};

// =============================================================================
// GENERATOR
// =============================================================================

/**
 * Generates Mermaid flowchart diagrams from NovaNet graph schema
 *
 * Features:
 * - Reads relations from RelationsParser (67 edges after expansion)
 * - Groups nodes by scope (Global, Shared, Project) from _index.yaml
 * - Styles nodes by locale behavior (invariant, localized, localeKnowledge, derived, job)
 * - Generates complete flowchart TB format
 */
export class MermaidGenerator {
  /**
   * Generate Mermaid flowchart from config files
   */
  static async generate(config: MermaidGeneratorConfig): Promise<string> {
    // Validate config
    if (!config.modelsDir) {
      throw new Error('MermaidGenerator: modelsDir cannot be empty');
    }

    const relationsPath = path.join(config.modelsDir, 'relations.yaml');
    const indexPath = path.join(config.modelsDir, '_index.yaml');

    // Load relations
    let edges: RelationEdge[];
    try {
      edges = await RelationsParser.loadFromFile(relationsPath);
    } catch (error) {
      throw new Error(
        `MermaidGenerator: Failed to load relations from ${relationsPath}: ${error instanceof Error ? error.message : 'Unknown error'}`
      );
    }

    // Load index
    let indexData: IndexYaml;
    try {
      const content = await fs.readFile(indexPath, 'utf-8');
      indexData = yaml.parse(content) as IndexYaml;
    } catch (error) {
      throw new Error(
        `MermaidGenerator: Failed to load index from ${indexPath}: ${error instanceof Error ? error.message : 'Unknown error'}`
      );
    }

    // Build node maps
    const nodeMetadata = buildNodeMetadataMap(indexData);
    const nodeToBehavior = buildNodeToBehaviorMap(indexData);

    // Collect all unique nodes from edges (excluding wildcards)
    const allNodes = collectUniqueNodes(edges);

    // Generate Mermaid output
    return generateMermaid(edges, allNodes, nodeMetadata, nodeToBehavior);
  }

  /**
   * Generate complete Markdown document with Mermaid diagram
   * Use this for file output and validation
   */
  static async generateMarkdown(config: MermaidGeneratorConfig): Promise<string> {
    const mermaidCode = await this.generate(config);
    return this.wrapInMarkdown(mermaidCode);
  }

  /**
   * Generate and write to file as proper Markdown document
   */
  static async writeToFile(config: MermaidGeneratorConfig): Promise<void> {
    if (!config.outputPath) {
      throw new Error('MermaidGenerator: outputPath is required for writeToFile');
    }

    const mermaidCode = await this.generate(config);
    const markdown = this.wrapInMarkdown(mermaidCode);
    await fs.writeFile(config.outputPath, markdown, 'utf-8');
  }

  /**
   * Wrap Mermaid code in proper Markdown document format
   */
  private static wrapInMarkdown(mermaidCode: string): string {
    const today = new Date().toISOString().split('T')[0];

    return `# NovaNet Complete Graph

> Auto-generated from YAML models. Do not edit manually.
> Last updated: ${today}

## Overview

This diagram shows the complete NovaNet graph schema with all 35 node types and their relationships.

### Legend

| Color | Locale Behavior | Description |
|-------|-----------------|-------------|
| 🔵 Blue | Invariant | Nodes that don't change between locales |
| 🟢 Green | Localized | Nodes with locale-specific content |
| 🟣 Purple | LocaleKnowledge | Cultural/linguistic knowledge per locale |
| ⚪ Gray | Derived | Computed/aggregated data |
| ⚙️ Gray | Job | Background processing tasks |

### Scopes

- **🌍 GLOBAL** - Locale configuration and knowledge (shared across all projects)
- **🎯 SHARED** - SEO/GEO optimization data (shared across projects)
- **📦 PROJECT** - Project-specific content structure and generation

## Graph Diagram

\`\`\`mermaid
${mermaidCode}
\`\`\`

## Edge Categories

| Arrow | Category | Description |
|-------|----------|-------------|
| \`-->\` | Ownership | Parent-child relationships |
| \`--->\` | Semantic | Meaning and concept links |
| \`-.->\` | Localization | Locale-specific content |
| \`==>\` | Generation | LLM generation flow |
| \`--o\` | Mining | SEO/GEO data extraction |

---

*Generated by @novanet/schema-tools MermaidGenerator*
`;
  }
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/**
 * Extract nodes from a scope with their category
 */
function extractNodesWithCategory(
  subcategory: ScopeSubcategory | undefined,
  scope: Scope
): Array<{ node: string; metadata: NodeMetadata }> {
  if (!subcategory) return [];

  const results: Array<{ node: string; metadata: NodeMetadata }> = [];
  for (const [category, paths] of Object.entries(subcategory)) {
    if (paths) {
      for (const path of paths) {
        results.push({
          node: filePathToNodeName(path),
          metadata: { scope, category },
        });
      }
    }
  }
  return results;
}

/**
 * Build mapping of node name to metadata (scope + category)
 * DYNAMIC: Extracts from file paths and category structure in _index.yaml
 */
function buildNodeMetadataMap(index: IndexYaml): Map<string, NodeMetadata> {
  const map = new Map<string, NodeMetadata>();

  // Global scope
  for (const { node, metadata } of extractNodesWithCategory(index.files?.global, 'Global')) {
    map.set(node, metadata);
  }

  // Shared scope
  for (const { node, metadata } of extractNodesWithCategory(index.files?.shared, 'Shared')) {
    map.set(node, metadata);
  }

  // Project scope
  for (const { node, metadata } of extractNodesWithCategory(index.files?.project, 'Project')) {
    map.set(node, metadata);
  }

  return map;
}

/**
 * Build mapping of node name to locale behavior from nodes_by_locale_behavior
 */
function buildNodeToBehaviorMap(index: IndexYaml): Map<string, LocaleBehavior> {
  const map = new Map<string, LocaleBehavior>();

  const behaviors = index.nodes_by_locale_behavior;
  if (!behaviors) {
    return map;
  }

  const behaviorKeys: LocaleBehavior[] = ['invariant', 'localized', 'localeKnowledge', 'derived', 'job'];
  for (const behavior of behaviorKeys) {
    const nodes = behaviors[behavior];
    if (nodes) {
      for (const node of nodes) {
        map.set(node, behavior);
      }
    }
  }

  return map;
}

/**
 * Collect all unique node names from edges, excluding wildcards
 */
function collectUniqueNodes(edges: RelationEdge[]): Set<string> {
  const nodes = new Set<string>();
  for (const edge of edges) {
    if (edge.from !== '*') {
      nodes.add(edge.from);
    }
    if (edge.to !== '*') {
      nodes.add(edge.to);
    }
  }
  return nodes;
}

/**
 * Generate complete Mermaid flowchart output with nested subgraphs
 */
function generateMermaid(
  edges: RelationEdge[],
  allNodes: Set<string>,
  nodeMetadata: Map<string, NodeMetadata>,
  nodeToBehavior: Map<string, LocaleBehavior>
): string {
  const lines: string[] = [];
  const warnings: string[] = [];

  // Check for unmapped nodes
  for (const node of allNodes) {
    if (!nodeMetadata.has(node)) {
      warnings.push(`Node "${node}" not found in _index.yaml files structure`);
    }
    if (!nodeToBehavior.has(node)) {
      warnings.push(`Node "${node}" not found in nodes_by_locale_behavior`);
    }
  }

  // Log warnings to console (but don't fail)
  for (const warning of warnings) {
    console.warn(`MermaidGenerator: ${warning}`);
  }

  // Header with statistics
  const edgeCount = edges.filter(e => e.from !== '*' && e.to !== '*').length;
  lines.push('flowchart TB');
  lines.push(`  %% NovaNet Graph v8.1.0`);
  lines.push(`  %% Generated: ${allNodes.size} nodes, ${edgeCount} edges`);
  lines.push(`  %% Source: relations.yaml + _index.yaml (with semantic edge styling)`);
  lines.push('');

  // Class definitions (from unified colors.ts)
  lines.push('  %% Locale behavior styling');
  for (const [behavior, style] of Object.entries(BEHAVIOR_STYLE)) {
    lines.push(`  classDef ${behavior} ${style}`);
  }
  lines.push('');

  // Group nodes by scope and category
  const nodesByScopeAndCategory = groupNodesByScopeAndCategory(allNodes, nodeMetadata);

  // Render nested subgraphs: Scope > Category > Nodes
  // Use _LAYER suffix to avoid collision with node names (e.g., "Project" node vs "Project" subgraph)
  const scopeOrder: Scope[] = ['Global', 'Shared', 'Project'];
  for (const scope of scopeOrder) {
    const categoriesMap = nodesByScopeAndCategory.get(scope);
    if (!categoriesMap || categoriesMap.size === 0) continue;

    const scopeEmoji = SCOPE_EMOJI[scope];
    const scopeId = `${scope.toUpperCase()}_LAYER`;
    lines.push(`  subgraph ${scopeId}["${scopeEmoji} ${scope.toUpperCase()}"]`);
    lines.push('    direction TB');

    // Sort categories for deterministic output
    const sortedCategories = [...categoriesMap.keys()].sort();
    for (const category of sortedCategories) {
      const categoryNodes = categoriesMap.get(category) || [];
      if (categoryNodes.length === 0) continue;

      const categoryDisplay = CATEGORY_DISPLAY[category] || category;
      const categoryId = `${scope.toUpperCase()}_${category}`;
      lines.push(`    subgraph ${categoryId}["${categoryDisplay}"]`);

      // Sort nodes and render with emoji labels
      const sortedNodes = [...categoryNodes].sort();
      for (const node of sortedNodes) {
        const behavior = nodeToBehavior.get(node) || 'invariant';
        const emoji = BEHAVIOR_EMOJI[behavior];
        lines.push(`      ${node}["${emoji} ${node}"]`);
      }
      lines.push('    end');
    }

    lines.push('  end');
    lines.push('');
  }

  // Render edges with semantic styling (excluding wildcards)
  lines.push('  %% Relationships (styled by edge category)');
  const filteredEdges = edges.filter(e => e.from !== '*' && e.to !== '*');
  // Sort edges for deterministic output
  const sortedEdges = [...filteredEdges].sort((a, b) => {
    const keyA = `${a.from}-${a.relation}-${a.to}`;
    const keyB = `${b.from}-${b.relation}-${b.to}`;
    return keyA.localeCompare(keyB);
  });

  // Track edge indices for linkStyle coloring
  const edgeStyles: Array<{ index: number; category: EdgeCategory }> = [];

  for (let i = 0; i < sortedEdges.length; i++) {
    const edge = sortedEdges[i];
    const category: EdgeCategory = EDGE_TO_CATEGORY[edge.relation] ?? 'ownership';
    const arrow = EDGE_ARROWS[category];
    lines.push(`  ${edge.from} ${arrow}|${edge.relation}| ${edge.to}`);
    edgeStyles.push({ index: i, category });
  }
  lines.push('');

  // Apply link styles for edge coloring (grouped by category)
  lines.push('  %% Edge colors by category');
  const stylesByCategory = new Map<EdgeCategory, number[]>();
  for (const { index, category } of edgeStyles) {
    if (!stylesByCategory.has(category)) {
      stylesByCategory.set(category, []);
    }
    stylesByCategory.get(category)!.push(index);
  }

  // Sort categories for deterministic output
  const sortedCategories = [...stylesByCategory.keys()].sort();
  for (const category of sortedCategories) {
    const indices = stylesByCategory.get(category)!;
    const color = EDGE_COLORS[category];
    // Mermaid linkStyle: linkStyle 0,1,2 stroke:#color,stroke-width:2px
    lines.push(`  linkStyle ${indices.join(',')} stroke:${color},stroke-width:2px`);
  }
  lines.push('');

  // Class assignments (sorted alphabetically for deterministic output)
  lines.push('  %% Class assignments');
  const sortedAllNodes = [...allNodes].sort();
  for (const node of sortedAllNodes) {
    const behavior = nodeToBehavior.get(node) || 'invariant';
    lines.push(`  class ${node} ${behavior}`);
  }

  return lines.join('\n');
}

/**
 * Group nodes by scope and category for nested subgraphs
 */
function groupNodesByScopeAndCategory(
  allNodes: Set<string>,
  nodeMetadata: Map<string, NodeMetadata>
): Map<Scope, Map<Category, string[]>> {
  const result = new Map<Scope, Map<Category, string[]>>();

  // Initialize scope maps
  const scopes: Scope[] = ['Global', 'Shared', 'Project'];
  for (const scope of scopes) {
    result.set(scope, new Map<Category, string[]>());
  }

  // Group nodes
  for (const node of allNodes) {
    const meta = nodeMetadata.get(node);
    if (meta) {
      const categoryMap = result.get(meta.scope)!;
      if (!categoryMap.has(meta.category)) {
        categoryMap.set(meta.category, []);
      }
      categoryMap.get(meta.category)!.push(node);
    } else {
      // Fallback: put unmapped nodes in Project/unknown
      const projectMap = result.get('Project')!;
      if (!projectMap.has('unknown')) {
        projectMap.set('unknown', []);
      }
      projectMap.get('unknown')!.push(node);
    }
  }

  return result;
}
