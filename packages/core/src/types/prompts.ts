// novanet-core/src/types/prompts.ts
// Instruction types v0.12.4 - AI Instructions with versioning
//
// BlockInstruction: Sub-agent instructions for block generation
// BlockRules: Template rules (extracted from BlockType.rules)
//
// v0.12.4: PageInstruction removed per ADR-028 - instructions composed from BlockInstructions

// ═══════════════════════════════════════════════════════════════════════════════
// COMMON INSTRUCTION PROPERTIES
// ═══════════════════════════════════════════════════════════════════════════════

interface InstructionBase {
  // Standard properties (v11.8.0 - no key, linked via :HAS_INSTRUCTION/:HAS_RULES)
  display_name: string;      // "Pricing Hero Instruction v1.0"
  description: string;       // "Instructions for pricing hero generation"
  llm_context: string;       // "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."

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
// BLOCKRULES - Template generation rules (Category: INSTRUCTION)
// ═══════════════════════════════════════════════════════════════════════════════

export interface BlockRules extends InstructionBase {
  // BlockRules-specific
  rules: string;             // "Title: action verb. Subtitle: value prop. CTA: urgency."
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

// v0.12.4: PageInstruction removed from union
export type InstructionNode = BlockInstruction | BlockRules;
