// src/generators/SubcategoryGenerator.ts
// Generates subcategories.ts from folder structure in models/nodes/
// v1.0.0 - Source of truth: YAML folder structure

import * as fs from 'fs/promises';
import * as path from 'path';
import { filePathToNodeName } from '../utils/filePathToNodeName.js';

// =============================================================================
// TYPES
// =============================================================================

/**
 * Scope classification (defined locally to avoid circular dependency)
 */
type Scope = 'Global' | 'Shared' | 'Project';

/**
 * Configuration for SubcategoryGenerator
 */
export interface SubcategoryGeneratorConfig {
  /** Path to models/nodes/ directory */
  modelsDir: string;
  /** Path to output file (src/graph/subcategories.ts) */
  outputPath?: string;
}

/**
 * Mapping of a node type to its scope and subcategory
 */
interface SubcategoryMapping {
  nodeType: string;
  scope: Scope;
  subcategory: string;
}

// =============================================================================
// CONSTANTS
// =============================================================================

/**
 * Map folder name to Scope
 */
const FOLDER_TO_SCOPE: Record<string, Scope> = {
  global: 'Global',
  shared: 'Shared',
  project: 'Project',
};

/**
 * Scope order for output
 */
const SCOPE_ORDER: Scope[] = ['Project', 'Global', 'Shared'];

// =============================================================================
// GENERATOR
// =============================================================================

/**
 * Generates subcategories.ts from YAML folder structure.
 *
 * Features:
 * - Scans models/nodes/{scope}/{subcategory}/*.yaml
 * - Extracts scope and subcategory from folder paths
 * - Converts filenames to NodeType (kebab-case to PascalCase)
 * - Generates TypeScript with proper formatting
 *
 * @example
 * ```typescript
 * const content = await SubcategoryGenerator.generate({
 *   modelsDir: 'models/nodes',
 * });
 * ```
 */
export class SubcategoryGenerator {
  /**
   * Generate subcategories.ts content from folder structure
   */
  static async generate(config: SubcategoryGeneratorConfig): Promise<string> {
    // Validate config
    if (!config.modelsDir) {
      throw new Error('SubcategoryGenerator: modelsDir cannot be empty');
    }

    // Check if directory exists
    try {
      await fs.access(config.modelsDir);
    } catch {
      throw new Error(`SubcategoryGenerator: modelsDir does not exist: ${config.modelsDir}`);
    }

    // Scan folder structure
    const mappings = await this.scanFolderStructure(config.modelsDir);

    // Group by scope and subcategory
    const grouped = this.groupMappings(mappings);

    // Generate TypeScript content
    return this.generateTypeScript(grouped);
  }

  /**
   * Generate and write to file
   */
  static async writeToFile(config: SubcategoryGeneratorConfig): Promise<void> {
    if (!config.outputPath) {
      throw new Error('SubcategoryGenerator: outputPath is required for writeToFile');
    }

    const content = await this.generate(config);
    await fs.writeFile(config.outputPath, content, 'utf-8');
  }

  /**
   * Scan folder structure to extract mappings
   * models/nodes/{scope}/{subcategory}/*.yaml
   */
  private static async scanFolderStructure(modelsDir: string): Promise<SubcategoryMapping[]> {
    const mappings: SubcategoryMapping[] = [];

    // Read scope folders (global, shared, project)
    const scopeFolders = await fs.readdir(modelsDir, { withFileTypes: true });

    for (const scopeFolder of scopeFolders) {
      if (!scopeFolder.isDirectory()) continue;

      const scopeName = scopeFolder.name.toLowerCase();
      const scope = FOLDER_TO_SCOPE[scopeName];

      if (!scope) {
        console.warn(`SubcategoryGenerator: Unknown scope folder: ${scopeName}`);
        continue;
      }

      const scopePath = path.join(modelsDir, scopeFolder.name);
      const subcategoryFolders = await fs.readdir(scopePath, { withFileTypes: true });

      for (const subcategoryFolder of subcategoryFolders) {
        if (!subcategoryFolder.isDirectory()) continue;

        const subcategory = subcategoryFolder.name;
        const subcategoryPath = path.join(scopePath, subcategoryFolder.name);
        const yamlFiles = await fs.readdir(subcategoryPath);

        for (const filename of yamlFiles) {
          if (!filename.endsWith('.yaml')) continue;

          const nodeType = filePathToNodeName(filename);
          mappings.push({
            nodeType,
            scope,
            subcategory,
          });
        }
      }
    }

    return mappings;
  }

  /**
   * Group mappings by scope and subcategory
   */
  private static groupMappings(
    mappings: SubcategoryMapping[]
  ): Map<Scope, Map<string, string[]>> {
    const result = new Map<Scope, Map<string, string[]>>();

    // Initialize scope maps
    for (const scope of SCOPE_ORDER) {
      result.set(scope, new Map<string, string[]>());
    }

    // Group mappings
    for (const mapping of mappings) {
      const scopeMap = result.get(mapping.scope)!;
      if (!scopeMap.has(mapping.subcategory)) {
        scopeMap.set(mapping.subcategory, []);
      }
      scopeMap.get(mapping.subcategory)!.push(mapping.nodeType);
    }

    // Sort node types within each subcategory
    for (const scopeMap of result.values()) {
      for (const [subcategory, nodeTypes] of scopeMap) {
        scopeMap.set(subcategory, nodeTypes.sort());
      }
    }

    return result;
  }

  /**
   * Generate TypeScript file content
   */
  private static generateTypeScript(
    grouped: Map<Scope, Map<string, string[]>>
  ): string {
    const lines: string[] = [];
    const timestamp = new Date().toISOString().split('T')[0];

    // Header
    lines.push('// packages/core/src/graph/subcategories.ts');
    lines.push('// NODE_SUBCATEGORIES mapping all 35 node types to their subcategories');
    lines.push('// AUTO-GENERATED from models/nodes/ folder structure');
    lines.push(`// Generated: ${timestamp}`);
    lines.push('// Run: pnpm schema:generate');
    lines.push('');
    lines.push("import type { NodeType } from '../types/nodes.js';");
    lines.push("import type { Subcategory } from './types.js';");
    lines.push('');
    lines.push('// =============================================================================');
    lines.push('// NODE_SUBCATEGORIES - Maps each NodeType to its subcategory');
    lines.push('// =============================================================================');
    lines.push('');
    lines.push('/**');
    lines.push(' * Maps each NodeType to its subcategory within its scope.');
    lines.push(' * AUTO-GENERATED from models/nodes/ folder structure.');
    lines.push(' *');
    lines.push(' * Subcategories by scope:');
    lines.push(' * - Project: foundation, structure, semantic, instruction, output');
    lines.push(' * - Global: config, knowledge');
    lines.push(' * - Shared: seo, geo');
    lines.push(' */');
    lines.push('export const NODE_SUBCATEGORIES: Record<NodeType, Subcategory> = {');

    // Generate entries by scope
    let totalNodes = 0;
    for (const scope of SCOPE_ORDER) {
      const scopeMap = grouped.get(scope)!;
      const scopeNodeCount = Array.from(scopeMap.values()).reduce(
        (sum, nodes) => sum + nodes.length,
        0
      );

      if (scopeNodeCount === 0) continue;

      totalNodes += scopeNodeCount;

      lines.push('  // ═══════════════════════════════════════════════════════════════════════════');
      lines.push(`  // ${scope.toUpperCase()} SCOPE (${scopeNodeCount} nodes)`);
      lines.push('  // ═══════════════════════════════════════════════════════════════════════════');
      lines.push('');

      // Sort subcategories alphabetically
      const sortedSubcategories = Array.from(scopeMap.keys()).sort();

      for (const subcategory of sortedSubcategories) {
        const nodeTypes = scopeMap.get(subcategory)!;
        if (nodeTypes.length === 0) continue;

        lines.push(`  // ${subcategory} (${nodeTypes.length} node${nodeTypes.length > 1 ? 's' : ''}) - matches models/nodes/${scope.toLowerCase()}/${subcategory}/`);

        for (const nodeType of nodeTypes) {
          lines.push(`  ${nodeType}: '${subcategory}',`);
        }

        lines.push('');
      }
    }

    lines.push('};');
    lines.push('');

    // Helper functions
    lines.push('// =============================================================================');
    lines.push('// HELPER FUNCTIONS');
    lines.push('// =============================================================================');
    lines.push('');
    lines.push('/**');
    lines.push(' * Get the subcategory for a node type');
    lines.push(' * @param nodeType - The node type to look up');
    lines.push(' * @returns The subcategory for the node type');
    lines.push(' */');
    lines.push('export function getSubcategory(nodeType: NodeType): Subcategory {');
    lines.push('  return NODE_SUBCATEGORIES[nodeType];');
    lines.push('}');
    lines.push('');
    lines.push('/**');
    lines.push(' * Get all node types in a subcategory');
    lines.push(' * @param subcategory - The subcategory to look up');
    lines.push(' * @returns Array of node types belonging to the subcategory');
    lines.push(' */');
    lines.push('export function getNodeTypesBySubcategory(subcategory: Subcategory): NodeType[] {');
    lines.push('  return (Object.entries(NODE_SUBCATEGORIES) as [NodeType, Subcategory][])');
    lines.push('    .filter(([, subcat]) => subcat === subcategory)');
    lines.push('    .map(([nodeType]) => nodeType);');
    lines.push('}');
    lines.push('');

    return lines.join('\n');
  }
}
