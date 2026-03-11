// NovaNet Instructions Migration v0.12.0
//
// v0.12.0 ADR-025: PagePrompt → PageInstruction, BlockPrompt → BlockInstruction
//
// Extracts AI instructions from existing nodes into dedicated Instruction nodes:
//   - Page.instructions -> PageInstruction
//   - Block.instructions -> BlockInstruction
//   - BlockType.rules -> BlockRules
//
// v0.19.0 STANDARD PROPERTIES (Instruction nodes):
//   display_name, content, llm_context,
//   prompt/rules (content), version, active, created_at, updated_at
//
// VERSIONING:
//   version: "1.0" (semver)
//   active: true (only one active per parent node)
//
// NOTE: Instruction extraction creates dedicated nodes (PageInstruction, BlockInstruction, BlockRules).
//       v0.13.0: Migration complete. Properties moved to instruction nodes.

// =============================================================================
// PAGEINSTRUCTION - Orchestrator Instructions (Category: INSTRUCTIONS)
// =============================================================================
//
// Extracts Page.instructions into PageInstruction nodes.
// Creates :HAS_INSTRUCTION relationship from Page to PageInstruction.
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     -> display_name
//   2. DOCUMENTATION      -> content, llm_context
//   3. PROMPT-SPECIFIC    -> prompt, version, active
//   4. TIMESTAMPS         -> created_at, updated_at
//
// -----------------------------------------------------------------------------

MATCH (p:Page)
WHERE p.instructions IS NOT NULL
CREATE (p)-[:HAS_INSTRUCTION]->(pi:PageInstruction {
  // 1. IDENTIFICATION
  display_name: p.display_name + " Instruction v1.0",
  // 2. DOCUMENTATION
  content: "Orchestration instructions for " + p.display_name,
  llm_context: "USE: page generation orchestration. TRIGGERS: " + p.key + ", page instruction. NOT: individual block instructions.",
  // 3. PROMPT-SPECIFIC
  prompt: p.instructions,
  version: "1.0",
  active: true,
  // 4. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// BLOCKINSTRUCTION - Sub-Agent Instructions (Category: INSTRUCTIONS)
// =============================================================================
//
// Extracts Block.instructions into BlockInstruction nodes.
// Creates :HAS_INSTRUCTION relationship from Block to BlockInstruction.
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     -> display_name
//   2. DOCUMENTATION      -> content, llm_context
//   3. PROMPT-SPECIFIC    -> prompt, version, active
//   4. TIMESTAMPS         -> created_at, updated_at
//
// -----------------------------------------------------------------------------

MATCH (b:Block)
WHERE b.instructions IS NOT NULL
CREATE (b)-[:HAS_INSTRUCTION]->(bi:BlockInstruction {
  // 1. IDENTIFICATION
  display_name: b.display_name + " Instruction v1.0",
  // 2. DOCUMENTATION
  content: "Generation instructions for " + b.display_name,
  llm_context: "USE: block content generation. TRIGGERS: " + b.key + ", block instruction. NOT: other blocks or page orchestration.",
  // 3. PROMPT-SPECIFIC
  prompt: b.instructions,
  version: "1.0",
  active: true,
  // 4. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// BLOCKRULES - Template Generation Rules (Category: PROMPTS)
// =============================================================================
//
// Extracts BlockType.rules into BlockRules nodes.
// Creates :HAS_RULES relationship from BlockType to BlockRules.
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     -> display_name
//   2. DOCUMENTATION      -> content, llm_context
//   3. RULES-SPECIFIC     -> rules, version, active
//   4. TIMESTAMPS         -> created_at, updated_at
//
// -----------------------------------------------------------------------------

MATCH (bt:BlockType)
WHERE bt.rules IS NOT NULL
CREATE (bt)-[:HAS_RULES]->(br:BlockRules {
  // 1. IDENTIFICATION
  display_name: bt.display_name + " Rules v1.0",
  // 2. DOCUMENTATION
  content: "Generation rules for " + bt.display_name,
  llm_context: "USE: block type rule validation. TRIGGERS: " + bt.key + ", block rules, template rules. NOT: specific block prompts.",
  // 3. RULES-SPECIFIC
  rules: bt.rules,
  version: "1.0",
  active: true,
  // 4. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// END OF FILE
// =============================================================================
