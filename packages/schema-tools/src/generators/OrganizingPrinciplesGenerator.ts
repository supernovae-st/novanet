// packages/schema-tools/src/generators/OrganizingPrinciplesGenerator.ts
// Generates Cypher seed for organizing principles from YAML
// v1.0.0 - Source of truth: organizing-principles.yaml

import * as fs from 'fs/promises';
import * as path from 'path';
import { parse as parseYaml } from 'yaml';
import { filePathToNodeName } from '../utils/filePathToNodeName.js';

// =============================================================================
// TYPES
// =============================================================================

interface SubcategoryDef {
  key: string;
  display_name: string;
  emoji: string;
  llm_context: string;
}

interface ScopeDef {
  key: string;
  display_name: string;
  emoji: string;
  color: string;
  llm_context: string;
  subcategories: SubcategoryDef[];
}

interface OrganizingPrinciplesYaml {
  version: string;
  scopes: ScopeDef[];
}

export interface OrganizingPrinciplesGeneratorConfig {
  /** Path to organizing-principles.yaml */
  organizingPrinciplesPath: string;
  /** Path to models/nodes/ for scanning node types */
  modelsDir: string;
}

// =============================================================================
// GENERATOR
// =============================================================================

/**
 * Generates Cypher seed file for organizing principles.
 *
 * Creates:
 * - Scope nodes (3)
 * - Subcategory nodes (9)
 * - NodeTypeMeta nodes (35)
 * - HAS_SUBCATEGORY relationships
 * - DEFINES_TYPE relationships
 */
export class OrganizingPrinciplesGenerator {
  /**
   * Generate Cypher seed content
   */
  static async generate(config: OrganizingPrinciplesGeneratorConfig): Promise<string> {
    // Load organizing-principles.yaml
    const yamlContent = await fs.readFile(config.organizingPrinciplesPath, 'utf-8');
    const data = parseYaml(yamlContent) as OrganizingPrinciplesYaml;

    // Scan folder structure for node types per subcategory
    const nodeTypesBySubcategory = await this.scanNodeTypes(config.modelsDir);

    // Generate Cypher
    return this.generateCypher(data, nodeTypesBySubcategory);
  }

  /**
   * Scan models/nodes/ to get node types per subcategory
   */
  private static async scanNodeTypes(
    modelsDir: string
  ): Promise<Map<string, string[]>> {
    const result = new Map<string, string[]>();

    const scopeFolders = await fs.readdir(modelsDir, { withFileTypes: true });
    for (const scopeFolder of scopeFolders) {
      if (!scopeFolder.isDirectory()) continue;

      const scopePath = path.join(modelsDir, scopeFolder.name);
      const subcategoryFolders = await fs.readdir(scopePath, { withFileTypes: true });

      for (const subcategoryFolder of subcategoryFolders) {
        if (!subcategoryFolder.isDirectory()) continue;

        const subcategory = subcategoryFolder.name;
        const subcategoryPath = path.join(scopePath, subcategoryFolder.name);
        const yamlFiles = await fs.readdir(subcategoryPath);

        const nodeTypes: string[] = [];
        for (const filename of yamlFiles) {
          if (!filename.endsWith('.yaml')) continue;
          nodeTypes.push(filePathToNodeName(filename));
        }

        if (nodeTypes.length > 0) {
          result.set(subcategory, nodeTypes.sort());
        }
      }
    }

    return result;
  }

  /**
   * Generate Cypher content
   */
  private static generateCypher(
    data: OrganizingPrinciplesYaml,
    nodeTypesBySubcategory: Map<string, string[]>
  ): string {
    const lines: string[] = [];
    const timestamp = new Date().toISOString().split('T')[0];

    // Header
    lines.push('// Organizing Principles Seed v8.3.0');
    lines.push('// AUTO-GENERATED from organizing-principles.yaml');
    lines.push(`// Generated: ${timestamp}`);
    lines.push('// Run: pnpm schema:generate');
    lines.push('//');
    lines.push('// Creates: Scope, Subcategory, NodeTypeMeta nodes');
    lines.push('// Uses MERGE for idempotent execution');
    lines.push('');

    // Create Scopes
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('// SCOPES (3)');
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('');

    for (const scope of data.scopes) {
      const llmContext = this.escapeCypher(scope.llm_context.trim());
      lines.push(`MERGE (s_${scope.key}:Scope {key: '${scope.key}'})`);
      lines.push('ON CREATE SET');
      lines.push(`  s_${scope.key}.display_name = '${scope.display_name}',`);
      lines.push(`  s_${scope.key}.emoji = '${scope.emoji}',`);
      lines.push(`  s_${scope.key}.color = '${scope.color}',`);
      lines.push(`  s_${scope.key}.llm_context = '${llmContext}',`);
      lines.push(`  s_${scope.key}.created_at = datetime()`);
      lines.push('ON MATCH SET');
      lines.push(`  s_${scope.key}.display_name = '${scope.display_name}',`);
      lines.push(`  s_${scope.key}.emoji = '${scope.emoji}',`);
      lines.push(`  s_${scope.key}.color = '${scope.color}',`);
      lines.push(`  s_${scope.key}.llm_context = '${llmContext}',`);
      lines.push(`  s_${scope.key}.updated_at = datetime();`);
      lines.push('');
    }

    // Create Subcategories
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('// SUBCATEGORIES (9)');
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('');

    for (const scope of data.scopes) {
      lines.push(`// ${scope.display_name} subcategories`);
      for (const sub of scope.subcategories) {
        const llmContext = this.escapeCypher(sub.llm_context.trim());
        lines.push(`MERGE (sub_${sub.key}:Subcategory {key: '${sub.key}'})`);
        lines.push('ON CREATE SET');
        lines.push(`  sub_${sub.key}.display_name = '${sub.display_name}',`);
        lines.push(`  sub_${sub.key}.emoji = '${sub.emoji}',`);
        lines.push(`  sub_${sub.key}.llm_context = '${llmContext}',`);
        lines.push(`  sub_${sub.key}.created_at = datetime()`);
        lines.push('ON MATCH SET');
        lines.push(`  sub_${sub.key}.display_name = '${sub.display_name}',`);
        lines.push(`  sub_${sub.key}.emoji = '${sub.emoji}',`);
        lines.push(`  sub_${sub.key}.llm_context = '${llmContext}',`);
        lines.push(`  sub_${sub.key}.updated_at = datetime();`);
        lines.push('');
        lines.push(`MERGE (s_${scope.key})-[:HAS_SUBCATEGORY]->(sub_${sub.key});`);
        lines.push('');
      }
    }

    // Create NodeTypeMeta
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('// NODE TYPE META (35)');
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('');

    for (const scope of data.scopes) {
      for (const sub of scope.subcategories) {
        const nodeTypes = nodeTypesBySubcategory.get(sub.key) || [];
        if (nodeTypes.length === 0) continue;

        lines.push(`// ${sub.display_name} (${nodeTypes.length} types)`);
        for (const nodeType of nodeTypes) {
          const varName = `t_${nodeType.toLowerCase().replace(/l10n/g, 'l10n')}`;
          lines.push(`MERGE (${varName}:NodeTypeMeta {label: '${nodeType}'})`);
          lines.push('ON CREATE SET');
          lines.push(`  ${varName}.display_name = '${nodeType}',`);
          lines.push(`  ${varName}.yaml_path = 'models/nodes/${scope.key}/${sub.key}/${this.toKebabCase(nodeType)}.yaml',`);
          lines.push(`  ${varName}.created_at = datetime()`);
          lines.push('ON MATCH SET');
          lines.push(`  ${varName}.updated_at = datetime();`);
          lines.push('');
          lines.push(`MERGE (sub_${sub.key})-[:DEFINES_TYPE]->(${varName});`);
          lines.push('');
        }
      }
    }

    return lines.join('\n');
  }

  /**
   * Escape string for Cypher
   */
  private static escapeCypher(str: string): string {
    return str
      .replace(/\\/g, '\\\\')
      .replace(/'/g, "\\'")
      .replace(/\n/g, ' ')
      .replace(/\s+/g, ' ');
  }

  /**
   * Convert PascalCase to kebab-case
   */
  private static toKebabCase(str: string): string {
    return str
      .replace(/([a-z])([A-Z])/g, '$1-$2')
      .replace(/([A-Z])([A-Z][a-z])/g, '$1-$2')
      .toLowerCase();
  }
}
