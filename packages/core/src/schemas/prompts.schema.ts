/**
 * @fileoverview NovaNet Instruction Node Schemas
 * @module @novanet/core/schemas/prompts
 * @version 0.12.0
 *
 * Zod validation schemas for Instruction nodes in the NovaNet knowledge graph.
 * Instructions are AI directives that guide content generation at the page and block level.
 *
 * **Instruction Node Types:**
 * - `PageInstruction`: High-level instructions for entire page generation
 * - `BlockInstruction`: Specific instructions for individual block generation
 * - `BlockRules`: Constraints and guidelines for block types
 *
 * **LLM Context Format:**
 * All instructions use a standardized `llm_context` format for efficient context loading:
 * ```
 * "USE: [when to use]. TRIGGERS: [relevant keywords]. NOT: [disambiguation]."
 * ```
 *
 * @example
 * ```typescript
 * import { PageInstructionSchema, BlockInstructionSchema, BlockRulesSchema } from '@novanet/core/schemas/prompts';
 *
 * // Validate a page instruction
 * const pageInstruction = PageInstructionSchema.parse({
 *   display_name: 'Homepage Hero',
 *   description: 'Generates the hero section content',
 *   llm_context: 'USE: homepage hero generation. TRIGGERS: hero, banner, headline. NOT: product pages.',
 *   instruction: 'Generate an engaging hero section...',
 *   version: '1.0.0',
 *   active: true,
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 *
 * @see ADR-007 — Generation, Not Translation
 * @see packages/core/models/node-kinds/org/instruction/ — Instruction YAML definitions
 */

import { z } from 'zod';

// =============================================================================
// BASE INSTRUCTION SCHEMA (v11.8.0 - no icon/priority/freshness)
// =============================================================================

/**
 * Base schema shared by all instruction types.
 *
 * Contains the common properties required by PageInstruction, BlockInstruction, and BlockRules.
 * Note: icon, priority, and freshness were removed in v8.2.0 to align with YAML v7.11.0.
 */
const InstructionBaseSchema = z.object({
  /** Human-readable name displayed in Studio UI */
  display_name: z.string().min(1)
    .describe('Human-readable name for the instruction'),

  /** Detailed description of what this instruction does and when to use it */
  description: z.string().min(1)
    .describe('Detailed description of instruction purpose'),

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

  /** Semantic version for instruction versioning (allows rollback and A/B testing) */
  version: z.string().regex(/^\d+\.\d+(\.\d+)?$/, 'version must be semver format')
    .describe('Semantic version (X.Y or X.Y.Z format)'),

  /** Whether this instruction version is currently active for generation */
  active: z.boolean()
    .describe('Whether this instruction is active for generation'),

  /** Timestamp when the instruction was created */
  created_at: z.date()
    .describe('Creation timestamp'),

  /** Timestamp when the instruction was last updated */
  updated_at: z.date()
    .describe('Last update timestamp'),
});

// =============================================================================
// PAGEINSTRUCTION SCHEMA
// =============================================================================

/**
 * Schema for PageInstruction nodes.
 *
 * PageInstructions provide high-level generation instructions for entire pages.
 * They are linked via `HAS_INSTRUCTION` arc from Page nodes.
 *
 * **Graph Position:**
 * ```
 * Page ─[:HAS_INSTRUCTION]→ PageInstruction ─[:GENERATED]→ PageGenerated
 * ```
 *
 * @example
 * ```typescript
 * const validated = PageInstructionSchema.parse({
 *   display_name: 'Product Page Instruction',
 *   description: 'Generates product pages with SEO optimization',
 *   llm_context: 'USE: product pages. TRIGGERS: product, catalog, buy. NOT: blog posts.',
 *   instruction: 'Generate a product page that highlights key features...',
 *   version: '2.1.0',
 *   active: true,
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 */
export const PageInstructionSchema = InstructionBaseSchema.extend({
  /** The actual instruction text sent to the LLM for page generation */
  instruction: z.string().min(1, 'instruction cannot be empty')
    .describe('Page generation instruction text for LLM'),
});

export type PageInstruction = z.infer<typeof PageInstructionSchema>;

// =============================================================================
// BLOCKINSTRUCTION SCHEMA
// =============================================================================

/**
 * Schema for BlockInstruction nodes.
 *
 * BlockInstructions provide specific generation instructions for individual blocks.
 * They are linked via `HAS_INSTRUCTION` arc from Block nodes.
 *
 * **Graph Position:**
 * ```
 * Block ─[:HAS_INSTRUCTION]→ BlockInstruction ─[:GENERATED]→ BlockGenerated
 * ```
 *
 * @example
 * ```typescript
 * const validated = BlockInstructionSchema.parse({
 *   display_name: 'FAQ Block Instruction',
 *   description: 'Generates FAQ accordion content',
 *   llm_context: 'USE: FAQ sections. TRIGGERS: questions, faq, help. NOT: contact forms.',
 *   instruction: 'Generate 5-7 frequently asked questions about...',
 *   version: '1.2.0',
 *   active: true,
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 */
export const BlockInstructionSchema = InstructionBaseSchema.extend({
  /** The actual instruction text sent to the LLM for block generation */
  instruction: z.string().min(1, 'instruction cannot be empty')
    .describe('Block generation instruction text for LLM'),
});

export type BlockInstruction = z.infer<typeof BlockInstructionSchema>;

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
export const BlockRulesSchema = InstructionBaseSchema.extend({
  /** Rules and constraints text for block type compliance */
  rules: z.string().min(1, 'rules cannot be empty')
    .describe('Constraints and guidelines for block type'),
});

export type BlockRules = z.infer<typeof BlockRulesSchema>;
