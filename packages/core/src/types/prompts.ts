// novanet-core/src/types/prompts.ts
// Instruction types v0.19.1 - AI Instructions with versioning
//
// BlockInstruction: Sub-agent instructions for block generation
//
// v0.12.4: PageInstruction removed per ADR-028
// v0.19.1: BlockRules removed — rules merged into BlockType.rules property

// ═══════════════════════════════════════════════════════════════════════════════
// COMMON INSTRUCTION PROPERTIES
// ═══════════════════════════════════════════════════════════════════════════════

interface InstructionBase {
  // Standard properties (v0.20.0 - no key, linked via :HAS_INSTRUCTION)
  display_name: string;      // "Pricing Hero Instruction v1.0"
  node_class: string;        // "BlockInstruction"
  content: string;           // "Instructions for pricing hero generation"
  triggers: string[];        // ["instruction", "hero", "pricing"]
  provenance: string;        // "seed"

  // Versioning (v7.2.0)
  version: string;           // "1.0", "1.1", "2.0"
  active: boolean;           // true = current version

  created_at: Date;
  updated_at: Date;
}

// v0.12.4: PageInstruction removed (ADR-028)
// Page-level instructions are now composed from BlockInstructions at generation time

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCKINSTRUCTION - Sub-agent instructions (Category: INSTRUCTION)
// ═══════════════════════════════════════════════════════════════════════════════

export interface BlockInstruction extends InstructionBase {
  // BlockInstruction-specific — Markdown with @ references
  instruction: string;       // "[GENERATE] Hero highlighting @tier-pro benefits"
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

export type InstructionNode = BlockInstruction;
