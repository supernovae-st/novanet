// NovaNet Prompts Migration v8.2.0
//
// Aligned with YAML v7.11.0 (icon, priority, freshness removed from schema)
//
// Extracts AI instructions from existing nodes into dedicated Prompt nodes:
//   - Page.instructions -> PagePrompt
//   - Block.instructions -> BlockPrompt
//   - BlockType.rules -> BlockRules
//
// v8.2.0 STANDARD PROPERTIES (Prompt nodes):
//   display_name, description, llm_context,
//   prompt/rules (content), version, active, created_at, updated_at
//
// VERSIONING:
//   version: "1.0" (semver)
//   active: true (only one active per parent node)
//
// NOTE: Original properties (instructions, rules) are KEPT for backward compatibility.
//       Remove in v7.3.0 after confirming migration success.

// =============================================================================
// PAGEPROMPT - Orchestrator Instructions (Category: PROMPTS)
// =============================================================================
//
// Extracts Page.instructions into PagePrompt nodes.
// Creates :HAS_PROMPT relationship from Page to PagePrompt.
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     -> display_name
//   2. DOCUMENTATION      -> description, llm_context
//   3. PROMPT-SPECIFIC    -> prompt, version, active
//   4. TIMESTAMPS         -> created_at, updated_at
//
// -----------------------------------------------------------------------------

MATCH (p:Page)
WHERE p.instructions IS NOT NULL
CREATE (p)-[:HAS_PROMPT]->(pp:PagePrompt {
  // 1. IDENTIFICATION
  display_name: p.display_name + " Prompt v1.0",
  // 2. DOCUMENTATION
  description: "Orchestration instructions for " + p.display_name,
  llm_context: "USE: page generation orchestration. TRIGGERS: " + p.key + ", page prompt. NOT: individual block prompts.",
  // 3. PROMPT-SPECIFIC
  prompt: p.instructions,
  version: "1.0",
  active: true,
  // 4. TIMESTAMPS
  created_at: datetime(),
  updated_at: datetime()
});

// =============================================================================
// BLOCKPROMPT - Sub-Agent Instructions (Category: PROMPTS)
// =============================================================================
//
// Extracts Block.instructions into BlockPrompt nodes.
// Creates :HAS_PROMPT relationship from Block to BlockPrompt.
//
// PROPERTY ORDER:
//   1. IDENTIFICATION     -> display_name
//   2. DOCUMENTATION      -> description, llm_context
//   3. PROMPT-SPECIFIC    -> prompt, version, active
//   4. TIMESTAMPS         -> created_at, updated_at
//
// -----------------------------------------------------------------------------

MATCH (b:Block)
WHERE b.instructions IS NOT NULL
CREATE (b)-[:HAS_PROMPT]->(bp:BlockPrompt {
  // 1. IDENTIFICATION
  display_name: b.display_name + " Prompt v1.0",
  // 2. DOCUMENTATION
  description: "Generation instructions for " + b.display_name,
  llm_context: "USE: block content generation. TRIGGERS: " + b.key + ", block prompt. NOT: other blocks or page orchestration.",
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
//   2. DOCUMENTATION      -> description, llm_context
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
  description: "Generation rules for " + bt.display_name,
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
// BACKWARD COMPATIBILITY NOTE
// =============================================================================
//
// DO NOT remove original properties yet:
//   - Page.instructions
//   - Block.instructions
//   - BlockType.rules
//
// Keep for backward compatibility until v7.3.0 confirms migration success.
// Removal will be done via separate migration script in v7.3.0.
//
// =============================================================================
