// src/generators/ViewParser.ts
// Parse and validate view YAML files with docs section
// v8.0.0

import * as fs from 'fs/promises';
import * as path from 'path';
import * as yaml from 'yaml';
import { ExtendedViewDefinitionSchema } from './schemas.js';
import type { ExtendedViewDefinition, ViewDocs } from './types.js';

/**
 * Parser for view YAML files with docs section support.
 *
 * @example
 * ```typescript
 * // Parse a single file
 * const view = await ViewParser.parseFile('models/views/page-generation.yaml');
 *
 * // Parse YAML string
 * const view = ViewParser.parseString(yamlContent);
 *
 * // Load all views
 * const views = await ViewParser.loadAllViews('models/views');
 * ```
 */
export class ViewParser {
  /**
   * Parse a view YAML file.
   */
  static async parseFile(filePath: string): Promise<ExtendedViewDefinition> {
    const content = await fs.readFile(filePath, 'utf-8');
    return this.parseString(content);
  }

  /**
   * Parse a YAML string into a view definition.
   * @throws {Error} if YAML is invalid or doesn't match schema
   */
  static parseString(yamlContent: string): ExtendedViewDefinition {
    const parsed = yaml.parse(yamlContent);
    return this.validate(parsed);
  }

  /**
   * Validate a parsed object against the schema.
   * @throws {Error} if validation fails
   */
  static validate(data: unknown): ExtendedViewDefinition {
    const result = ExtendedViewDefinitionSchema.safeParse(data);

    if (!result.success) {
      const errors = result.error.errors
        .map(e => `${e.path.join('.')}: ${e.message}`)
        .join('\n');
      throw new Error(`Invalid view definition:\n${errors}`);
    }

    return result.data as ExtendedViewDefinition;
  }

  /**
   * Load all view files from a directory.
   * Skips files starting with underscore (like _registry.yaml).
   */
  static async loadAllViews(dirPath: string): Promise<ExtendedViewDefinition[]> {
    const files = await fs.readdir(dirPath);
    const viewFiles = files.filter(f =>
      f.endsWith('.yaml') && !f.startsWith('_')
    );

    const views: ExtendedViewDefinition[] = [];

    for (const file of viewFiles) {
      try {
        const view = await this.parseFile(path.join(dirPath, file));
        views.push(view);
      } catch (error) {
        console.warn(`Failed to parse ${file}:`, error);
      }
    }

    return views;
  }

  /**
   * Check if a view has a docs section.
   */
  static hasDocs(view: ExtendedViewDefinition): view is ExtendedViewDefinition & { docs: ViewDocs } {
    return view.docs !== undefined &&
      view.docs.title !== undefined &&
      view.docs.layers !== undefined &&
      view.docs.layers.length > 0;
  }

  /**
   * Get views that have docs sections.
   */
  static async loadViewsWithDocs(dirPath: string): Promise<ExtendedViewDefinition[]> {
    const allViews = await this.loadAllViews(dirPath);
    return allViews.filter(this.hasDocs);
  }

  /**
   * Get views grouped by category.
   */
  static groupByCategory(views: ExtendedViewDefinition[]): Map<string, ExtendedViewDefinition[]> {
    const groups = new Map<string, ExtendedViewDefinition[]>();

    for (const view of views) {
      const category = view.docs?.category ?? 'other';
      const existing = groups.get(category) ?? [];
      groups.set(category, [...existing, view]);
    }

    return groups;
  }
}
