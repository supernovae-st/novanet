// src/parsers/RelationsParser.ts
// Parser for models/relations.yaml - extracts relation edges for Mermaid generation
// v8.0.0

import * as fs from 'fs/promises';
import * as yaml from 'yaml';

// =============================================================================
// TYPES
// =============================================================================

/**
 * Represents a single directed edge in the graph
 * Expanded from array notation (e.g., [A, B] -> C becomes A->C and B->C)
 */
export interface RelationEdge {
  relation: string;
  from: string;
  to: string;
  props?: string[];
  description?: string;
}

/**
 * Raw relation definition from YAML (before expansion)
 */
interface RawRelation {
  from: string;
  to: string;
  props?: string[];
  description?: string;
  inverse_of?: string;
}

/**
 * Top-level YAML structure
 */
interface RelationsYaml {
  relations?: Record<string, RawRelation>;
}

// =============================================================================
// PARSER
// =============================================================================

/**
 * Parses models/relations.yaml and extracts relation edges for Mermaid generation
 *
 * Features:
 * - Expands array notation: `from: "[A, B]"` -> separate edges for A and B
 * - Filters out inverse relations (those with `inverse_of` property)
 * - Preserves props and description for documentation
 */
export class RelationsParser {
  /**
   * Load and parse relations from a YAML file
   */
  static async loadFromFile(filePath: string): Promise<RelationEdge[]> {
    if (!filePath) {
      throw new Error('RelationsParser: File path cannot be empty');
    }

    try {
      const content = await fs.readFile(filePath, 'utf-8');
      return this.parseYaml(content);
    } catch (error) {
      throw new Error(
        `RelationsParser: Failed to load ${filePath}: ${error instanceof Error ? error.message : 'Unknown error'}`
      );
    }
  }

  /**
   * Parse YAML content string into relation edges
   */
  static parseYaml(content: string): RelationEdge[] {
    let parsed: RelationsYaml;

    try {
      parsed = yaml.parse(content) as RelationsYaml;
    } catch (error) {
      console.warn('RelationsParser: Failed to parse YAML:', error instanceof Error ? error.message : error);
      return [];
    }

    if (!parsed?.relations || typeof parsed.relations !== 'object') {
      return [];
    }

    const edges: RelationEdge[] = [];

    for (const [relationName, rawRelation] of Object.entries(parsed.relations)) {
      // Skip inverse relations
      if (rawRelation.inverse_of) {
        continue;
      }

      // Validate required fields
      if (!rawRelation.from || !rawRelation.to) {
        continue;
      }

      // Expand array notation
      const fromTypes = this.parseArrayNotation(rawRelation.from);
      const toTypes = this.parseArrayNotation(rawRelation.to);

      // Create edge for each combination
      for (const from of fromTypes) {
        for (const to of toTypes) {
          const edge: RelationEdge = {
            relation: relationName,
            from,
            to,
          };

          // Add optional fields if present
          if (rawRelation.props && rawRelation.props.length > 0) {
            edge.props = rawRelation.props;
          }
          if (rawRelation.description) {
            edge.description = rawRelation.description;
          }

          edges.push(edge);
        }
      }
    }

    return edges;
  }

  /**
   * Parse array notation in from/to fields
   *
   * Examples:
   * - "Project" -> ["Project"]
   * - "[Concept, Project]" -> ["Concept", "Project"]
   * - "*" -> ["*"]
   */
  private static parseArrayNotation(value: string): string[] {
    if (!value || typeof value !== 'string') {
      return [];
    }

    // Handle array notation: "[A, B, C]"
    if (value.startsWith('[') && value.endsWith(']')) {
      const inner = value.slice(1, -1);
      return inner.split(',').map(s => s.trim()).filter(s => s.length > 0);
    }

    // Single value (including wildcard "*")
    return [value];
  }
}
