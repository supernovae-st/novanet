// src/schemas/view.schema.ts
// Zod validation schemas for YAML view definitions
import { z } from 'zod';
import { NODE_TYPES } from '../types/nodes.js';

// Cast to tuple for Zod enum
const NodeTypeEnum = NODE_TYPES as unknown as [string, ...string[]];

// ============================================================================
// FILTER CRITERIA SCHEMA
// ============================================================================

const FilterCriteriaSchema = z.object({
  nodeTypes: z.array(z.enum(NodeTypeEnum)).optional(),
  locale: z.string().optional(),
  localeFamily: z.string().optional(),
  active: z.boolean().optional(),
  categories: z.array(z.enum(['shared', 'org'])).optional(),
  maxDepth: z.number().int().positive().optional(),
}).strict().optional();

// ============================================================================
// INCLUDE RULE SCHEMA (recursive)
// ============================================================================

const RelationDirectionSchema = z.enum(['outgoing', 'incoming', 'both']);

// Base schema without recursion (direction is optional with default in types)
const BaseIncludeRuleSchema = z.object({
  relation: z.string().min(1),
  direction: RelationDirectionSchema.optional().default('outgoing'),
  depth: z.number().int().positive().optional(),
  targetTypes: z.array(z.enum(NodeTypeEnum)).optional(),
  filters: FilterCriteriaSchema,
});

// Recursive schema using z.lazy() - use unknown type to avoid circular type issues
export const IncludeRuleSchema: z.ZodType<unknown> = BaseIncludeRuleSchema.extend({
  include: z.lazy(() => z.array(IncludeRuleSchema)).optional(),
});

// ============================================================================
// VIEW DEFINITION SCHEMA
// ============================================================================

export const ViewDefinitionSchema = z.object({
  id: z.string().min(1).regex(/^[a-z0-9-]+$/, 'View ID must be lowercase alphanumeric with hyphens'),
  name: z.string().min(1),
  description: z.string(),
  // Version can be "X.Y" or "X.Y.Z" format
  version: z.string().regex(/^\d+\.\d+(\.\d+)?$/, 'Version must be X.Y or X.Y.Z format'),

  root: z.object({
    type: z.enum(NodeTypeEnum),
    key: z.string().optional(),
  }),

  include: z.array(IncludeRuleSchema),
  filters: FilterCriteriaSchema,
});

export type ViewDefinitionParsed = z.infer<typeof ViewDefinitionSchema>;

// ============================================================================
// VIEW REGISTRY SCHEMA
// ============================================================================

export const ViewCategorySchema = z.enum(['overview', 'generation', 'knowledge', 'project', 'mining']);

export const ViewRegistryEntrySchema = z.object({
  id: z.string().min(1).regex(/^[a-z0-9-]+$/),
  file: z.string().endsWith('.yaml'),
  description: z.string(),
  category: ViewCategorySchema,
});

export const ViewRegistrySchema = z.object({
  version: z.string(),
  description: z.string().optional(),
  views: z.array(ViewRegistryEntrySchema),
});

export type ViewRegistryParsed = z.infer<typeof ViewRegistrySchema>;

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/**
 * Validates a parsed YAML view definition.
 * @throws ZodError if validation fails
 */
export function validateViewDefinition(data: unknown): ViewDefinitionParsed {
  return ViewDefinitionSchema.parse(data);
}

/**
 * Validates a parsed YAML view registry.
 * @throws ZodError if validation fails
 */
export function validateViewRegistry(data: unknown): ViewRegistryParsed {
  return ViewRegistrySchema.parse(data);
}
