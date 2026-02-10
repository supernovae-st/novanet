/**
 * @fileoverview NovaNet View Definition Schemas
 * @module @novanet/core/schemas/view
 * @version 11.6.0
 *
 * Zod validation schemas for YAML view definitions used to describe graph traversal patterns.
 * Views define how to traverse the NovaNet knowledge graph from a root node, following
 * specific relationships to build context for LLM generation.
 *
 * **Architecture Overview:**
 * ```
 * View YAML → ViewDefinitionSchema → Validated View → Cypher Generator → LLM Context
 * ```
 *
 * @example
 * ```typescript
 * import { validateViewDefinition, validateViewRegistry } from '@novanet/core/schemas/view';
 *
 * // Validate a view definition
 * const view = yaml.parse(fs.readFileSync('page-generation.yaml'));
 * const validated = validateViewDefinition(view);
 *
 * // Validate view registry
 * const registry = yaml.parse(fs.readFileSync('_index.yaml'));
 * const validatedRegistry = validateViewRegistry(registry);
 * ```
 *
 * @see ADR-003 — YAML-First Architecture
 * @see packages/core/models/views/ — View YAML definitions
 */

import { z } from 'zod';
import { NODE_TYPES } from '../types/nodes.js';

// Cast to tuple for Zod enum compatibility
const NodeTypeEnum = NODE_TYPES as unknown as [string, ...string[]];

// ============================================================================
// FILTER CRITERIA SCHEMA
// ============================================================================

/**
 * Filter criteria for narrowing graph traversal.
 *
 * Filters can be applied at any level of the view hierarchy to restrict
 * which nodes are included in the traversal result.
 *
 * @example
 * ```yaml
 * filters:
 *   nodeTypes: [Entity, EntityContent]
 *   locale: "fr-FR"
 *   active: true
 *   categories: [org]
 *   maxDepth: 3
 * ```
 */
const FilterCriteriaSchema = z.object({
  /** Restrict to specific node types (from 60 NovaNet types) */
  nodeTypes: z.array(z.enum(NodeTypeEnum))
    .describe('Restrict traversal to specific node types')
    .optional(),
  /** BCP-47 locale code for locale-specific content (e.g., "fr-FR", "ja-JP") */
  locale: z.string()
    .describe('BCP-47 locale code for filtering locale-specific nodes')
    .optional(),
  /** Locale family prefix for regional variants (e.g., "en" matches en-US, en-GB) */
  localeFamily: z.string()
    .describe('Locale family prefix for matching regional variants')
    .optional(),
  /** Filter by active status (true = only active, false = only inactive) */
  active: z.boolean()
    .describe('Filter nodes by active status')
    .optional(),
  /** Realm categories to include: "shared" (universal) or "org" (organization-specific) */
  categories: z.array(z.enum(['shared', 'org']))
    .describe('Filter by realm: shared (universal) or org (organization-specific)')
    .optional(),
  /** Maximum traversal depth from the include rule level */
  maxDepth: z.number().int().positive()
    .describe('Maximum depth for recursive traversal from this point')
    .optional(),
}).strict().optional();

// ============================================================================
// INCLUDE RULE SCHEMA (recursive)
// ============================================================================

/**
 * Relation traversal direction for include rules.
 *
 * - `outgoing`: Follow arcs from source → target (default)
 * - `incoming`: Follow arcs from target ← source
 * - `both`: Follow arcs in either direction
 */
const RelationDirectionSchema = z.enum(['outgoing', 'incoming', 'both'])
  .describe('Direction to traverse the relationship');

/**
 * Base include rule without recursion.
 * Defines how to traverse a single relationship type.
 */
const BaseIncludeRuleSchema = z.object({
  /** Neo4j relationship type to traverse (e.g., "HAS_PAGE", "USES_ENTITY") */
  relation: z.string().min(1)
    .describe('Neo4j relationship type (arc kind) to traverse'),
  /** Direction to traverse: outgoing (default), incoming, or both */
  direction: RelationDirectionSchema.optional().default('outgoing'),
  /** Maximum depth for recursive traversal of this relationship */
  depth: z.number().int().positive()
    .describe('Maximum recursive depth for this relationship')
    .optional(),
  /** Restrict target nodes to specific types */
  targetTypes: z.array(z.enum(NodeTypeEnum))
    .describe('Filter target nodes to specific types')
    .optional(),
  /** Additional filter criteria for this traversal level */
  filters: FilterCriteriaSchema,
});

/**
 * Recursive include rule that can contain nested include rules.
 *
 * @example
 * ```yaml
 * include:
 *   - relation: HAS_PAGE
 *     direction: outgoing
 *     include:
 *       - relation: HAS_BLOCK
 *         direction: outgoing
 *       - relation: USES_ENTITY
 *         direction: outgoing
 * ```
 */
export const IncludeRuleSchema: z.ZodType<unknown> = BaseIncludeRuleSchema.extend({
  /** Nested include rules for deeper traversal */
  include: z.lazy(() => z.array(IncludeRuleSchema))
    .describe('Nested include rules for recursive traversal')
    .optional(),
});

// ============================================================================
// VIEW DEFINITION SCHEMA
// ============================================================================

/**
 * Complete view definition schema.
 *
 * A view defines a graph traversal pattern starting from a root node type,
 * following specified relationships to build LLM context for generation.
 *
 * @example
 * ```yaml
 * id: page-generation
 * name: Page Generation View
 * description: Context for generating page content
 * version: "11.6.0"
 *
 * root:
 *   type: Project
 *   key: qrcode-ai
 *
 * include:
 *   - relation: HAS_PAGE
 *     direction: outgoing
 *     include:
 *       - relation: HAS_BLOCK
 *         direction: outgoing
 * ```
 */
export const ViewDefinitionSchema = z.object({
  /** Unique identifier for the view (lowercase, alphanumeric, hyphens) */
  id: z.string().min(1).regex(/^[a-z0-9-]+$/, 'View ID must be lowercase alphanumeric with hyphens')
    .describe('Unique view identifier (kebab-case)'),
  /** Human-readable view name */
  name: z.string().min(1)
    .describe('Human-readable view name'),
  /** Detailed description of the view purpose */
  description: z.string()
    .describe('Detailed description of view purpose and usage'),
  /** Semantic version (X.Y or X.Y.Z format) */
  version: z.string().regex(/^\d+\.\d+(\.\d+)?$/, 'Version must be X.Y or X.Y.Z format')
    .describe('Semantic version matching NovaNet schema version'),

  /** Root node specification for traversal starting point */
  root: z.object({
    /** Node type to start traversal from (one of 60 NovaNet types) */
    type: z.enum(NodeTypeEnum)
      .describe('Root node type to start traversal from'),
    /** Optional specific node key (if omitted, matches all nodes of type) */
    key: z.string()
      .describe('Specific node key to start from (optional)')
      .optional(),
  }).describe('Root node specification for traversal'),

  /** Relationship traversal rules defining the graph pattern */
  include: z.array(IncludeRuleSchema)
    .describe('Array of include rules defining relationship traversal'),
  /** Global filters applied to entire view */
  filters: FilterCriteriaSchema,
});

export type ViewDefinitionParsed = z.infer<typeof ViewDefinitionSchema>;

// ============================================================================
// VIEW REGISTRY SCHEMA
// ============================================================================

/**
 * View category for organization.
 *
 * - `overview`: High-level views of the entire graph
 * - `generation`: Views for LLM content generation context
 * - `knowledge`: Views for locale knowledge loading
 * - `project`: Views for project-specific data
 * - `mining`: Views for SEO/GEO mining operations
 */
export const ViewCategorySchema = z.enum(['overview', 'generation', 'knowledge', 'project', 'mining'])
  .describe('View category for organization and filtering');

/**
 * Single entry in the view registry.
 */
export const ViewRegistryEntrySchema = z.object({
  /** View ID matching the view definition */
  id: z.string().min(1).regex(/^[a-z0-9-]+$/)
    .describe('View ID (must match view definition)'),
  /** Relative path to the view YAML file */
  file: z.string().endsWith('.yaml')
    .describe('Relative path to view YAML file'),
  /** Brief description for the registry */
  description: z.string()
    .describe('Brief description of view purpose'),
  /** Category for organizing views */
  category: ViewCategorySchema,
});

/**
 * View registry schema (models/views/_index.yaml).
 *
 * The registry maintains a list of all available views with their
 * metadata, used by the Rust CLI for view discovery and validation.
 */
export const ViewRegistrySchema = z.object({
  /** Registry format version */
  version: z.string()
    .describe('Registry format version'),
  /** Optional description of the registry */
  description: z.string()
    .describe('Registry description')
    .optional(),
  /** Array of registered views */
  views: z.array(ViewRegistryEntrySchema)
    .describe('List of registered views'),
});

export type ViewRegistryParsed = z.infer<typeof ViewRegistrySchema>;

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/**
 * Validates a parsed YAML view definition.
 *
 * @param data - Raw parsed YAML data
 * @returns Validated and typed view definition
 * @throws {ZodError} If validation fails with detailed error messages
 *
 * @example
 * ```typescript
 * try {
 *   const validated = validateViewDefinition(yamlData);
 *   console.log(`View ${validated.id} is valid`);
 * } catch (error) {
 *   if (error instanceof z.ZodError) {
 *     console.error('Validation errors:', error.issues);
 *   }
 * }
 * ```
 */
export function validateViewDefinition(data: unknown): ViewDefinitionParsed {
  return ViewDefinitionSchema.parse(data);
}

/**
 * Validates a parsed YAML view registry.
 *
 * @param data - Raw parsed YAML data
 * @returns Validated and typed view registry
 * @throws {ZodError} If validation fails with detailed error messages
 *
 * @example
 * ```typescript
 * const registry = validateViewRegistry(yamlData);
 * console.log(`Registry contains ${registry.views.length} views`);
 * ```
 */
export function validateViewRegistry(data: unknown): ViewRegistryParsed {
  return ViewRegistrySchema.parse(data);
}
