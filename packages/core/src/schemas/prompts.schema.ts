/**
 * @fileoverview NovaNet Instruction Node Schemas
 * @module @novanet/core/schemas/prompts
 * @version 0.20.0
 *
 * Zod validation schemas for Instruction nodes in the NovaNet knowledge graph.
 * Instructions are AI directives that guide content generation at the block level.
 *
 * **Instruction Node Types:**
 * - `BlockInstruction`: Specific instructions for individual block generation
 *
 * v0.12.4: PageInstruction removed per ADR-028 - page instructions are now
 * composed from BlockInstructions at generation time.
 *
 * v0.20.0: Standard properties migrated (description+llm_context -> node_class+content+triggers+provenance)
 *
 * @example
 * ```typescript
 * import { BlockInstructionSchema } from '@novanet/core/schemas/prompts';
 *
 * // Validate a block instruction
 * const blockInstruction = BlockInstructionSchema.parse({
 *   display_name: 'Homepage Hero',
 *   node_class: 'BlockInstruction',
 *   content: 'Generates the hero section content',
 *   triggers: ['instruction', 'hero', 'homepage'],
 *   provenance: 'seed',
 *   instruction: 'Generate an engaging hero section...',
 *   version: '1.0.0',
 *   active: true,
 *   created_at: new Date(),
 *   updated_at: new Date(),
 * });
 * ```
 *
 * @see ADR-007 — Generation, Not Translation
 * @see ADR-028 — Page-Entity Architecture
 * @see packages/core/models/node-classes/org/instruction/ — Instruction YAML definitions
 */

import { z } from 'zod';

// =============================================================================
// BASE INSTRUCTION SCHEMA (v0.20.0 standard properties)
// =============================================================================

/**
 * Base schema shared by all instruction types.
 *
 * Contains the common properties required by BlockInstruction.
 * v0.20.0: Standard properties migrated to node_class + content + triggers + provenance.
 */
const InstructionBaseSchema = z.object({
  /** Human-readable name displayed in Studio UI */
  display_name: z.string().min(1)
    .describe('Human-readable name for the instruction'),

  /** Node class identifier (PascalCase for DATA nodes) */
  node_class: z.string().min(1)
    .describe('Node class name (e.g., "BlockInstruction")'),

  /** Plain language description of what this instruction is and how it works */
  content: z.string().min(1)
    .describe('Plain language WHAT+HOW description'),

  /** English routing keywords for search and spreading activation (max 10) */
  triggers: z.array(z.string()).max(10)
    .describe('English routing keywords for search'),

  /** Data origin: seed, nika, or mcp */
  provenance: z.string().min(1)
    .describe('Data origin (seed/nika/mcp)'),

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
// BLOCKINSTRUCTION SCHEMA (v0.12.4: PageInstruction removed per ADR-028)
// =============================================================================

/**
 * Schema for BlockInstruction nodes.
 *
 * BlockInstructions provide specific generation instructions for individual blocks.
 * They are linked via `HAS_INSTRUCTION` arc from Block nodes.
 *
 * **Graph Position:**
 * ```
 * Block ─[:HAS_INSTRUCTION]→ BlockInstruction ─[:GENERATED]→ BlockNative
 * ```
 *
 * @example
 * ```typescript
 * const validated = BlockInstructionSchema.parse({
 *   display_name: 'FAQ Block Instruction',
 *   node_class: 'BlockInstruction',
 *   content: 'Generates FAQ accordion content',
 *   triggers: ['instruction', 'faq', 'questions'],
 *   provenance: 'seed',
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

// v0.19.1: BlockRulesSchema removed — rules merged into BlockType.rules property
