/**
 * @fileoverview NovaNet Prompt Node Schemas
 * @module @novanet/core/schemas/prompts
 * @version 11.6.0
 *
 * Zod validation schemas for Prompt nodes in the NovaNet knowledge graph.
 * Prompts are AI instructions that guide content generation at the page and block level.
 *
 * **Prompt Node Types:**
 * - `PagePrompt`: High-level instructions for entire page generation
 * - `BlockPrompt`: Specific instructions for individual block generation
 * - `BlockRules`: Constraints and guidelines for block types
 *
 * **LLM Context Format:**
 * All prompts use a standardized `llm_context` format for efficient context loading:
 * ```
 * "USE: [when to use]. TRIGGERS: [relevant keywords]. NOT: [disambiguation]."
 * ```
 *
 * @example
 * ```typescript
 * import { PagePromptSchema, BlockPromptSchema, BlockRulesSchema } from '@novanet/core/schemas/prompts';
 *
 * // Validate a page prompt
 * const pagePrompt = PagePromptSchema.parse({
 *   display_name: 'Homepage Hero',
 *   description: 'Generates the hero section content',
 *   llm_context: 'USE: homepage hero generation. TRIGGERS: hero, banner, headline. NOT: product pages.',
 *   prompt: 'Generate an engaging hero section...',
 *   version: '1.0.0',
 *   active: true,
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 *
 * @see ADR-007 — Generation, Not Translation
 * @see packages/core/models/node-kinds/org/instruction/ — Prompt YAML definitions
 */

import { z } from 'zod';

// =============================================================================
// BASE PROMPT SCHEMA (v8.2.0 - no icon/priority/freshness)
// =============================================================================

/**
 * Base schema shared by all prompt types.
 *
 * Contains the common properties required by PagePrompt, BlockPrompt, and BlockRules.
 * Note: icon, priority, and freshness were removed in v8.2.0 to align with YAML v7.11.0.
 */
const PromptBaseSchema = z.object({
  /** Human-readable name displayed in Studio UI */
  display_name: z.string().min(1)
    .describe('Human-readable name for the prompt'),

  /** Detailed description of what this prompt does and when to use it */
  description: z.string().min(1)
    .describe('Detailed description of prompt purpose'),

  /**
   * Structured context for LLM retrieval.
   * Must follow format: "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."
   *
   * @example "USE: homepage hero generation. TRIGGERS: hero, banner, headline. NOT: product pages."
   */
  llm_context: z.string().regex(
    /^USE:.*\. TRIGGERS:.*\. NOT:.*\.$/,
    'llm_context must follow format: "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."'
  ).describe('Structured LLM context in USE/TRIGGERS/NOT format'),

  // REMOVED v8.2.0: icon, priority, freshness (YAML v7.11.0 alignment)

  /** Semantic version for prompt versioning (allows rollback and A/B testing) */
  version: z.string().regex(/^\d+\.\d+(\.\d+)?$/, 'version must be semver format')
    .describe('Semantic version (X.Y or X.Y.Z format)'),

  /** Whether this prompt version is currently active for generation */
  active: z.boolean()
    .describe('Whether this prompt is active for generation'),

  /** Timestamp when the prompt was created */
  created_at: z.date()
    .describe('Creation timestamp'),

  /** Timestamp when the prompt was last updated */
  updated_at: z.date()
    .describe('Last update timestamp'),
});

// =============================================================================
// PAGEPROMPT SCHEMA
// =============================================================================

/**
 * Schema for PagePrompt nodes.
 *
 * PagePrompts provide high-level generation instructions for entire pages.
 * They are linked via `HAS_PROMPT` arc from Page nodes.
 *
 * **Graph Position:**
 * ```
 * Page ─[:HAS_PROMPT]→ PagePrompt ─[:GENERATED]→ PageGenerated
 * ```
 *
 * @example
 * ```typescript
 * const validated = PagePromptSchema.parse({
 *   display_name: 'Product Page Prompt',
 *   description: 'Generates product pages with SEO optimization',
 *   llm_context: 'USE: product pages. TRIGGERS: product, catalog, buy. NOT: blog posts.',
 *   prompt: 'Generate a product page that highlights key features...',
 *   version: '2.1.0',
 *   active: true,
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 */
export const PagePromptSchema = PromptBaseSchema.extend({
  /** The actual prompt text sent to the LLM for page generation */
  prompt: z.string().min(1, 'prompt cannot be empty')
    .describe('Page generation prompt text for LLM'),
});

export type PagePrompt = z.infer<typeof PagePromptSchema>;

// =============================================================================
// BLOCKPROMPT SCHEMA
// =============================================================================

/**
 * Schema for BlockPrompt nodes.
 *
 * BlockPrompts provide specific generation instructions for individual blocks.
 * They are linked via `HAS_PROMPT` arc from Block nodes.
 *
 * **Graph Position:**
 * ```
 * Block ─[:HAS_PROMPT]→ BlockPrompt ─[:GENERATED]→ BlockGenerated
 * ```
 *
 * @example
 * ```typescript
 * const validated = BlockPromptSchema.parse({
 *   display_name: 'FAQ Block Prompt',
 *   description: 'Generates FAQ accordion content',
 *   llm_context: 'USE: FAQ sections. TRIGGERS: questions, faq, help. NOT: contact forms.',
 *   prompt: 'Generate 5-7 frequently asked questions about...',
 *   version: '1.2.0',
 *   active: true,
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 */
export const BlockPromptSchema = PromptBaseSchema.extend({
  /** The actual prompt text sent to the LLM for block generation */
  prompt: z.string().min(1, 'prompt cannot be empty')
    .describe('Block generation prompt text for LLM'),
});

export type BlockPrompt = z.infer<typeof BlockPromptSchema>;

// =============================================================================
// BLOCKRULES SCHEMA
// =============================================================================

/**
 * Schema for BlockRules nodes.
 *
 * BlockRules define constraints and guidelines for block types, ensuring
 * consistent output across all blocks of a given type.
 *
 * **Graph Position:**
 * ```
 * BlockType ─[:HAS_RULES]→ BlockRules
 * ```
 *
 * @example
 * ```typescript
 * const validated = BlockRulesSchema.parse({
 *   display_name: 'Hero Block Rules',
 *   description: 'Constraints for hero block generation',
 *   llm_context: 'USE: hero blocks. TRIGGERS: hero, banner, cta. NOT: body content.',
 *   rules: 'Headlines must be under 60 characters. Include exactly one CTA button...',
 *   version: '1.0.0',
 *   active: true,
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 */
export const BlockRulesSchema = PromptBaseSchema.extend({
  /** Rules and constraints text for block type compliance */
  rules: z.string().min(1, 'rules cannot be empty')
    .describe('Constraints and guidelines for block type'),
});

export type BlockRules = z.infer<typeof BlockRulesSchema>;
