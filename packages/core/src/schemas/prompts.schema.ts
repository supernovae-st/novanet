// novanet-core/src/schemas/prompts.schema.ts
// Zod schemas for Prompt nodes v7.2.0

import { z } from 'zod';
import { PrioritySchema, FreshnessSchema } from './locale-knowledge.schema.js';

// =============================================================================
// BASE PROMPT SCHEMA
// =============================================================================

const PromptBaseSchema = z.object({
  // Standard properties (no key - linked via relations)
  display_name: z.string().min(1),
  icon: z.string().min(1),
  description: z.string().min(1),
  llm_context: z.string().regex(
    /^USE:.*\. TRIGGERS:.*\. NOT:.*\.$/,
    'llm_context must follow format: "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."'
  ),

  // Context management
  priority: PrioritySchema,
  freshness: FreshnessSchema,

  // Versioning (v7.2.0)
  version: z.string().regex(/^\d+\.\d+(\.\d+)?$/, 'version must be semver format'),
  active: z.boolean(),

  // Timestamps
  created_at: z.date(),
  updated_at: z.date(),
});

// =============================================================================
// PAGEPROMPT SCHEMA
// =============================================================================

export const PagePromptSchema = PromptBaseSchema.extend({
  prompt: z.string().min(1, 'prompt cannot be empty'),
});

// =============================================================================
// BLOCKPROMPT SCHEMA
// =============================================================================

export const BlockPromptSchema = PromptBaseSchema.extend({
  prompt: z.string().min(1, 'prompt cannot be empty'),
});

// =============================================================================
// BLOCKRULES SCHEMA
// =============================================================================

export const BlockRulesSchema = PromptBaseSchema.extend({
  rules: z.string().min(1, 'rules cannot be empty'),
});

