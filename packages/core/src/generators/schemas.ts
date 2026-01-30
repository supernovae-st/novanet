// src/generators/schemas.ts
// Zod validation schemas for Unified View System
// v8.1.0: NodeTypeSchema derived from single source of truth

import { z } from 'zod';
import { NODE_TYPES } from '../types/nodes.js';

// =============================================================================
// NODE TYPE ENUM (derived from types/nodes.ts - single source of truth)
// =============================================================================

const NodeTypeSchema = z.enum(NODE_TYPES);

// =============================================================================
// DOC LAYER SCHEMA
// =============================================================================

export const DocLayerSchema = z.object({
  name: z.string().min(1),
  nodes: z.array(NodeTypeSchema).min(1),
  color: z.enum(['blue', 'green', 'orange', 'purple', 'red', 'gray', 'cyan']).optional(),
  description: z.string().optional(),
});

// =============================================================================
// CYPHER EXAMPLE SCHEMA
// =============================================================================

export const CypherExampleSchema = z.object({
  name: z.string().min(1),
  description: z.string().optional(),
  query: z.string().min(1),
  params: z.record(z.unknown()).optional(),
});

// =============================================================================
// VIEW DOCS SCHEMA
// =============================================================================

export const DocCategorySchema = z.enum([
  'overview',
  'generation',
  'localization',
  'semantic',
  'mining',
]);

export const ViewDocsSchema = z.object({
  title: z.string().min(1),
  category: DocCategorySchema,
  description: z.string().min(1),
  layers: z.array(DocLayerSchema).min(1),
  examples: z.array(CypherExampleSchema).optional(),
  notes: z.array(z.string()).optional(),
  mermaid: z.string().optional(),  // Custom Mermaid diagram (overrides auto-generated)
});

// =============================================================================
// INCLUDE RULE SCHEMA
// =============================================================================

const FilterCriteriaSchema = z.object({
  nodeTypes: z.array(NodeTypeSchema).optional(),
  locale: z.string().optional(),
  localeFamily: z.string().optional(),
  priority: z.array(z.enum(['critical', 'high', 'medium', 'low'])).optional(),
  freshness: z.array(z.enum(['current', 'recent', 'stale', 'outdated'])).optional(),
  active: z.boolean().optional(),
  searchQuery: z.string().optional(),
  maxDepth: z.number().int().positive().optional(),
}).strict();

const IncludeRuleSchema: z.ZodType<unknown> = z.lazy(() =>
  z.object({
    relation: z.string().min(1),
    direction: z.enum(['outgoing', 'incoming', 'both']),
    depth: z.number().int().positive().optional(),
    targetTypes: z.array(NodeTypeSchema).optional(),
    filters: FilterCriteriaSchema.optional(),
    include: z.array(IncludeRuleSchema).optional(),
  })
);

// =============================================================================
// EXTENDED VIEW DEFINITION SCHEMA
// =============================================================================

export const ExtendedViewDefinitionSchema = z.object({
  id: z.string().regex(/^[a-z][a-z0-9-]*$/, 'View ID must be kebab-case'),
  name: z.string().min(1),
  description: z.string().min(1),
  version: z.string().regex(/^\d+\.\d+(\.\d+)?$/, 'Version must be semver-like'),

  root: z.object({
    type: NodeTypeSchema,
    key: z.string().optional(),
  }),

  include: z.array(IncludeRuleSchema),
  filters: FilterCriteriaSchema.optional(),

  // NEW: Documentation section
  docs: ViewDocsSchema.optional(),
});

// =============================================================================
// VIEW REGISTRY SCHEMA
// =============================================================================

export const ExtendedViewRegistrySchema = z.object({
  version: z.string(),
  generated_at: z.string().optional(),
  categories: z.record(DocCategorySchema, z.array(z.string())).optional(),
  views: z.array(z.object({
    id: z.string(),
    file: z.string(),
    description: z.string(),
    category: DocCategorySchema.optional(),
  })),
});

// Type exports are in types.ts to avoid duplication
// Use: import type { DocLayer, ViewDocs, ... } from './types.js'
