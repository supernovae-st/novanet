// novanet-core/src/types/prompts.ts
// Prompt types v7.2.0 - AI Instructions with versioning
//
// v7.2.0: Separation of Structure vs Prompts vs Output
// - PagePrompt: Orchestrator instructions (extracted from Page.instructions)
// - BlockPrompt: Sub-agent instructions (extracted from Block.instructions)
// - BlockRules: Template rules (extracted from BlockType.rules)

import type { Priority, Freshness } from './locale-knowledge.js';

// ═══════════════════════════════════════════════════════════════════════════════
// COMMON PROMPT PROPERTIES
// ═══════════════════════════════════════════════════════════════════════════════

interface PromptBase {
  // Standard properties (v7.1.0 - no key, linked via :HAS_PROMPT/:HAS_RULES)
  display_name: string;      // "Pricing Hero Prompt v1.0"
  icon: string;              // "📝"
  description: string;       // "Instructions for pricing hero generation"
  llm_context: string;       // "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."

  // Context management (v7.1.0)
  priority: Priority;
  freshness: Freshness;

  // Versioning (v7.2.0)
  version: string;           // "1.0", "1.1", "2.0"
  active: boolean;           // true = current version

  created_at: Date;
  updated_at: Date;
}

// ═══════════════════════════════════════════════════════════════════════════════
// PAGEPROMPT - Orchestrator instructions (Category: PROMPTS 🔵)
// ═══════════════════════════════════════════════════════════════════════════════

export interface PagePrompt extends PromptBase {
  // PagePrompt-specific
  prompt: string;            // "[GENERATE] Create conversion-focused pricing page..."
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCKPROMPT - Sub-agent instructions (Category: PROMPTS 🔵)
// ═══════════════════════════════════════════════════════════════════════════════

export interface BlockPrompt extends PromptBase {
  // BlockPrompt-specific
  prompt: string;            // "[GENERATE] Hero highlighting @tier-pro benefits"
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCKRULES - Template generation rules (Category: PROMPTS 🔵)
// ═══════════════════════════════════════════════════════════════════════════════

export interface BlockRules extends PromptBase {
  // BlockRules-specific
  rules: string;             // "Title: action verb. Subtitle: value prop. CTA: urgency."
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

export type PromptNode = PagePrompt | BlockPrompt | BlockRules;
