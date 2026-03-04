# Claude Code Skill Creation Guide for Nika Workflows

Complete guide to creating, testing, and deploying Claude Code skills for Nika workflow expert assistance.

**Date:** 2026-03-04
**Target:** Nika Skills for YAML workflow generation and validation
**Status:** Definitive reference from supernovae-agi monorepo

---

## Table of Contents

1. [Skill File Structure](#skill-file-structure)
2. [Frontmatter Format](#frontmatter-format)
3. [Content Patterns](#content-patterns)
4. [Nika-Specific Patterns](#nika-specific-patterns)
5. [Testing & Validation](#testing--validation)
6. [Best Practices](#best-practices)
7. [Examples](#examples)
8. [Integration with CLAUDE.md](#integration-with-claudemd)

---

## Skill File Structure

### Location and Naming

```
.claude/skills/
├── my-skill/
│   ├── SKILL.md           ← Main skill file (required)
│   ├── examples/          ← Example outputs (optional)
│   │   ├── example-1.md
│   │   └── example-2.md
│   ├── templates/         ← Reusable templates (optional)
│   │   ├── template-1.md
│   │   └── template-2.md
│   └── assets/            ← Diagrams, images (optional)
│       └── architecture.txt
```

**Naming Convention:**
- Skill directory: `kebab-case` (e.g., `nika-workflow-expert`)
- Skill file: Always `SKILL.md` (exact case)
- No other files required in directory (supports team skills in git)

### Minimal Skill

```
.claude/skills/my-skill/
└── SKILL.md  ← Single file, checked into git
```

---

## Frontmatter Format

### Required Fields

```yaml
---
name: Skill Name                 # Display name
description: What this does and when to use it
---
```

### Optional Fields

```yaml
---
name: Skill Name
description: Detailed description with specific use cases
allowed-tools: Read, Grep, Glob  # Restrict to these tools only
disabled: false                   # Can be disabled temporarily
author: Your Name                 # Optional
tags: [yaml, workflow, validation]  # Optional categorization
---
```

### Description Best Practices

**❌ Bad (too generic):**
```yaml
description: Helps with Nika workflows
```

**✅ Good (specific with triggers):**
```yaml
description: Generate valid Nika YAML workflows from requirements. Use when creating new workflows, optimizing existing workflows, or debugging workflow syntax errors.
```

**Why:** Claude autonomously chooses skills based on relevance. Specific descriptions with trigger keywords (like "when creating", "when debugging") help Claude recognize when to use the skill.

---

## Content Patterns

### Header Structure

```markdown
# Skill Name

One-sentence summary of what this skill does.

## When to Use This Skill

- Creating new [type of thing]
- Debugging [specific problem]
- Optimizing [aspect]
- Validating [component]

## How to Use This Skill

Step-by-step instructions for Claude to follow.

## Examples

Real examples showing before/after or correct usage.

## Validation Checklist

Criteria Claude should verify before claiming success.

## See Also

Links to related skills, documentation, or ADRs.
```

### Detailed Example: Content Moderation Pattern

```markdown
# Nika Workflow Validator

Validates YAML workflows against Nika schema standards and identifies common errors before execution.

## When to Use This Skill

- Creating new workflows (validate as you build)
- Debugging failing workflows (find syntax/logic errors)
- Refactoring existing workflows (ensure compatibility)
- Code reviewing workflow files (verify best practices)

## Prerequisites

- You have read `NIKA.md` in project root
- You understand 5 verbs: `infer:`, `exec:`, `fetch:`, `invoke:`, `agent:`
- You know workflow structure: `workflow: name`, `steps:`, `tasks:`

## Step-by-Step Instructions

### Step 1: Read the Workflow File

```bash
grep -A 50 "^workflow:" <filename>.yaml
```

Identify:
- Workflow name and version
- Number of steps/tasks
- Verbs used (infer/exec/fetch/invoke/agent)
- Dependencies between steps

### Step 2: Check Verb Syntax

For each step, verify:

- **infer:** Has `context:` field with valid reference?
- **exec:** Command is properly quoted and contains no unescaped pipes?
- **fetch:** URL is absolute and uses `use.ctx:` for output?
- **invoke:** References valid MCP tool with correct parameters?
- **agent:** Has valid `agent:` reference and `workflow:` field?

### Step 3: Validate Parameter References

- All `$variable` references exist in earlier steps
- All `use.ctx:` assignments are used in later steps
- No circular dependencies (step A → B → A)

### Step 4: Check Error Handling

- Are `on_error:` handlers defined for critical steps?
- Do fallback steps exist for network operations?
- Are budget/timeout constraints reasonable?

### Step 5: Report Findings

Format: YAML block with errors/warnings.

## Examples

### Example 1: Valid Workflow (Correct)

```yaml
workflow: extract-and-summarize
version: 1.0
steps:
  - id: fetch_page
    fetch:
      url: $page_url
      timeout_ms: 5000
    use.ctx: page_content

  - id: extract_text
    infer: "Extract main text from page"
    context: $page_content
    use.ctx: extracted_text

  - id: summarize
    invoke: novanet_generate
    params:
      entity: "summary"
      locale: "en-US"
    use.ctx: summary
```

**Validation Result: ✅ PASS**
- All verbs valid
- All variables referenced exist
- No circular dependencies
- Error handling: None needed (simple pipeline)

### Example 2: Invalid Workflow (Errors)

```yaml
workflow: broken-workflow
steps:
  - fetch:
      url: $missing_url  # ❌ $missing_url not defined

  - exec: "echo $extracted"  # ❌ $extracted from step 1? No use.ctx defined

  - invoke: novanet_describe  # ❌ Missing params
    params:
      target: "entity"
```

**Validation Result: ❌ FAIL**
- Error 1: `$missing_url` not defined anywhere
- Error 2: Step 1 (fetch) has no `use.ctx:` but step 2 references output
- Error 3: Step 3 (invoke) missing required `describe` parameter

## Validation Checklist

Before reporting success, verify:
- [ ] All variables (`$name`) are defined in earlier steps
- [ ] All `use.ctx:` assignments are actually used
- [ ] No circular dependencies (graph is acyclic)
- [ ] All `invoke:` steps reference real MCP tools
- [ ] Command strings in `exec:` are properly quoted
- [ ] No undefined verbs (only infer/exec/fetch/invoke/agent)
- [ ] Workflow has a name and at least one step

## See Also

- `NIKA.md` - Nika project overview
- Nika v0.13 schema: `.nika/config.toml`
- MCP tools reference: NovaNet MCP documentation
```

---

## Nika-Specific Patterns

### Pattern 1: Workflow Validator Skill

**Purpose:** Validate YAML workflows before execution

```yaml
---
name: nika-workflow-validator
description: Validate YAML workflows against Nika schema standards. Use when creating new workflows, debugging syntax errors, or refactoring workflow files.
allowed-tools: Read, Grep, Glob
---

# Nika Workflow Validator

## When to Use

- Creating new `.nika/workflows/` files
- Debugging `nika run` errors
- Code reviewing workflow YAML
- Migrating workflows between versions

## How to Use

1. Read the workflow file using Read tool
2. Check against Nika schema (5 verbs: infer/exec/fetch/invoke/agent)
3. Validate parameter references
4. Report errors with line numbers

## Validation Rules

[Rules from Nika schema...]
```

### Pattern 2: Workflow Generator Skill

**Purpose:** Generate workflows from requirements

```yaml
---
name: nika-workflow-generator
description: Generate valid Nika YAML workflows from natural language requirements. Use when starting a new workflow, creating workflow examples, or refactoring workflows.
---

# Nika Workflow Generator

## When to Use

- Creating new automation workflows
- Demonstrating workflow patterns
- Generating boilerplate workflows

## How to Use

1. Understand user requirements
2. Map to Nika verbs (infer/exec/fetch/invoke/agent)
3. Generate YAML with proper indentation
4. Validate with nika-workflow-validator skill

## Patterns

[Common workflow patterns: data pipeline, agent loop, notification, etc.]
```

### Pattern 3: MCP Integration Helper

**Purpose:** Help integrate MCP tools into workflows

```yaml
---
name: nika-mcp-integration-helper
description: Help integrate MCP tools (novanet_*, etc.) into Nika workflows. Use when invoking MCP tools, assembling context, or debugging MCP errors.
---

# Nika MCP Integration Helper

## When to Use

- Calling `novanet_generate`, `novanet_query`, etc.
- Assembling context for LLM tasks
- Debugging MCP tool invocation errors

## How to Use

1. Identify which MCP tool to use
2. Check tool schema and parameters
3. Generate correct invoke step
4. Validate with validator skill
```

---

## Testing & Validation

### Manual Testing Pattern

**Before submitting a skill, test it with varied prompts:**

```bash
# Test 1: Explicit invocation
"Use /nika-workflow-validator to check my workflow"

# Test 2: Implicit trigger (should activate automatically)
"I'm creating a new Nika workflow, help me generate it"

# Test 3: Edge cases
"Validate this broken workflow: [invalid YAML]"

# Test 4: Integration
"Generate a workflow AND validate it before I run it"
```

### Verification Checklist

- [ ] Skill activates on relevant keywords
- [ ] Instructions are clear and step-by-step
- [ ] Examples show correct and incorrect usage
- [ ] Error messages are actionable
- [ ] No contradictions with project rules in CLAUDE.md
- [ ] Tool restrictions in `allowed-tools:` are appropriate
- [ ] Works with git pull (no external dependencies)

### Testing with Subagents

For complex skills, test with dedicated agents:

```
Pattern: spn-powers:testing-skills-with-subagents

1. Run baseline (without skill)
2. Implement skill
3. Test with skill (RED-GREEN-REFACTOR)
4. Validate success criteria
```

---

## Best Practices

### ✅ DO

1. **Specific Descriptions** - Include trigger keywords and use cases
2. **Clear Prerequisites** - List what readers should know first
3. **Step-by-Step** - Give Claude explicit steps to follow
4. **Real Examples** - Show correct and incorrect examples
5. **Validation Checklist** - List criteria before claiming success
6. **Team Git** - Commit skills to git for team sharing
7. **Focused Scope** - One skill = one capability
8. **Tool Restrictions** - Use `allowed-tools:` for safety

### ❌ DON'T

1. **Generic Descriptions** - "Helps with workflows" is too vague
2. **Assumed Knowledge** - Don't assume readers know Nika details
3. **Rambling Instructions** - Make steps explicit and numbered
4. **Made-Up Examples** - Use real patterns from project
5. **Tool Overload** - Don't require all tools
6. **Overlapping Skills** - Avoid duplicate functionality
7. **Vague Validation** - Be specific about success criteria

### Overlap Avoidance

**Example: Two skills doing the same thing**

❌ **Bad**
```yaml
---
name: workflow-validator
description: Validates workflows
---

---
name: nika-workflow-checker
description: Checks workflows
---
```

✅ **Good**
```yaml
---
name: nika-workflow-validator
description: Validate YAML workflows against Nika schema
---

---
name: nika-workflow-optimizer
description: Optimize workflows for performance and clarity
---
```

---

## Examples

### Complete Skill: Nika Workflow Validator

**File:** `.claude/skills/nika-workflow-validator/SKILL.md`

```yaml
---
name: nika-workflow-validator
description: Validate YAML workflows against Nika schema (5 verbs, parameters, references). Use when creating workflows, debugging errors, or code reviewing YAML.
allowed-tools: Read, Grep, Glob
---

# Nika Workflow Validator

Validates Nika YAML workflows for correctness and identifies common errors.

## When to Use This Skill

- **Creating new workflows** - Validate as you write
- **Debugging errors** - Find syntax and logic issues
- **Code reviewing** - Check workflow files before merging
- **Migrating workflows** - Ensure compatibility after refactoring

## Prerequisites

- Read `.nika/README.md` or `NIKA.md` in project root
- Understand 5 verbs: `infer:`, `exec:`, `fetch:`, `invoke:`, `agent:`
- Know workflow structure: `workflow: name`, `steps:`, `tasks:`

## Step-by-Step Validation

### Step 1: Read Workflow File

Use Read tool to get the complete workflow YAML.

### Step 2: Extract Metadata

- Workflow name
- Version (if specified)
- Number of steps
- Verbs used in each step

### Step 3: Validate Each Verb

**For `infer:` steps:**
- Must have `context:` field
- Context must reference valid variable (from earlier step's `use.ctx:`)

**For `exec:` steps:**
- Command must be quoted
- Must not have unescaped special characters
- Use `use.ctx:` if output is needed later

**For `fetch:` steps:**
- URL must be absolute (http/https)
- Must have `timeout_ms` for network operations
- Use `use.ctx:` to capture response

**For `invoke:` steps:**
- Tool name must be valid MCP tool (novanet_*, etc.)
- Must have `params:` matching tool schema
- Use `use.ctx:` for output

**For `agent:` steps:**
- Must reference valid agent (from `.nika/agents/`)
- Must have `workflow:` field
- Can have `context:` for input

### Step 4: Validate References

- All `$variable` references exist in earlier steps
- All `$variable` have corresponding `use.ctx:` assignment
- No undefined variables

### Step 5: Detect Common Errors

```
❌ Missing variable definition: $var referenced but never assigned
❌ Unused assignment: use.ctx: $var but $var never used
❌ Circular dependency: Step A depends on B depends on A
❌ Invalid verb: Step uses unknown verb (not in: infer/exec/fetch/invoke/agent)
❌ Missing context: infer: without context: field
❌ Quoted parameters: Params not properly escaped in YAML
```

### Step 6: Report Results

Format report as YAML block:

```yaml
workflow: <name>
status: PASS  # or FAIL
errors: []
warnings: []
summary: "Workflow is valid and ready to execute"
```

## Examples

### Example 1: Valid Workflow

```yaml
workflow: page-generator
version: 1.0
steps:
  - id: load_context
    invoke: novanet_generate
    params:
      entity: "qr-code"
      locale: "fr-FR"
    use.ctx: entity_context

  - id: generate_page
    infer: "Generate landing page HTML"
    context: $entity_context
    use.ctx: page_html

  - id: save_output
    exec: "mkdir -p output && echo '$page_html' > output/page.html"
```

**Validation Report:**
```yaml
workflow: page-generator
status: PASS
errors: []
warnings: []
summary: "All 3 steps valid. Variables properly chained."
```

### Example 2: Invalid Workflow

```yaml
workflow: broken
steps:
  - fetch: $url         # ❌ $url undefined
    use.ctx: response

  - infer: "Process"    # ❌ No context:
    use.ctx: result

  - exec: "echo $foo"   # ❌ $foo undefined
```

**Validation Report:**
```yaml
workflow: broken
status: FAIL
errors:
  - line: 2
    error: "Variable $url not defined"
  - line: 5
    error: "infer: step missing context: field"
  - line: 9
    error: "Variable $foo referenced but not defined"
warnings: []
summary: "Fix 3 errors before running"
```

## Validation Checklist

Before reporting success:
- [ ] All variables defined before use
- [ ] All use.ctx assignments are used
- [ ] No circular dependencies (can be executed top-to-bottom)
- [ ] All verbs are valid (infer/exec/fetch/invoke/agent)
- [ ] invoke: steps have matching MCP tools
- [ ] infer: steps have context:
- [ ] YAML is valid (proper indentation, no syntax errors)

## See Also

- [Nika Project README](../../../NIKA.md)
- [Nika Workflow Schema](../../../.nika/README.md)
- [NovaNet MCP Tools](../../../novanet/CLAUDE.md)
- Related skill: `nika-workflow-generator`
```

### Complete Skill: Nika Workflow Generator

**File:** `.claude/skills/nika-workflow-generator/SKILL.md`

```yaml
---
name: nika-workflow-generator
description: Generate valid Nika YAML workflows from natural language requirements. Use when starting new workflows, creating examples, or prototyping automation.
---

# Nika Workflow Generator

Generate well-structured YAML workflows that follow Nika conventions.

## When to Use This Skill

- **Starting new workflows** - Generate boilerplate from requirements
- **Creating examples** - Show workflow patterns for documentation
- **Prototyping** - Quickly build workflow proof-of-concept

## Prerequisites

- Understand Nika 5 verbs: `infer:`, `exec:`, `fetch:`, `invoke:`, `agent:`
- Know workflow structure and conventions
- Have `.nika/config.toml` available for context

## How to Generate Workflows

### Step 1: Understand Requirements

Ask clarifying questions:
- What is the workflow trying to accomplish?
- What inputs does it need?
- What outputs should it produce?
- Which MCP tools or agents are involved?

### Step 2: Map to Nika Verbs

- **Data fetching** → `fetch:` (web URLs, APIs)
- **LLM tasks** → `infer:` (generation, analysis, reasoning)
- **Execution** → `exec:` (run commands, shell operations)
- **Tool calls** → `invoke:` (MCP tools: novanet_*, etc.)
- **Agent loops** → `agent:` (hand off to agents)

### Step 3: Generate YAML

Structure:
```yaml
workflow: descriptive-name
version: 1.0
steps:
  - id: step_id_1
    [verb]: definition...
    use.ctx: output_name

  - id: step_id_2
    [verb]: ...
```

### Step 4: Validate

Use `nika-workflow-validator` skill to check.

## Common Patterns

### Pattern 1: Data Pipeline

```yaml
workflow: data-extraction-pipeline
steps:
  - id: fetch_data
    fetch:
      url: "https://api.example.com/data"
    use.ctx: raw_data

  - id: parse_data
    infer: "Extract structured data from raw response"
    context: $raw_data
    use.ctx: parsed_data

  - id: validate
    infer: "Validate data matches schema"
    context: $parsed_data
    use.ctx: validation_result
```

### Pattern 2: Generation with Context

```yaml
workflow: generate-content
steps:
  - id: load_context
    invoke: novanet_generate
    params:
      entity: "main-entity"
      locale: "en-US"
    use.ctx: entity_data

  - id: create_content
    infer: "Generate marketing copy"
    context: $entity_data
    use.ctx: content
```

### Pattern 3: Agent Loop

```yaml
workflow: agent-research
steps:
  - id: research
    agent:
      agent: researcher
      workflow: search-and-summarize
      context: "Topic: $topic"
    use.ctx: research_results

  - id: compile
    infer: "Compile findings"
    context: $research_results
    use.ctx: final_report
```

## Generation Checklist

Before submitting generated workflow:
- [ ] Workflow has descriptive name
- [ ] All steps have unique `id:`
- [ ] Variables chained correctly
- [ ] Uses MCP tools appropriately
- [ ] Validated with `nika-workflow-validator`
- [ ] Ready to run: `nika run workflow.yaml`

## See Also

- Related skill: `nika-workflow-validator`
- [Nika Verbs Reference](../../../.nika/README.md#verbs)
```

---

## Integration with CLAUDE.md

### Add Skills Section to CLAUDE.md

```markdown
## 🎯 Nika Skills

These skills help write and validate Nika YAML workflows.

| Skill | When | Purpose |
|-------|------|---------|
| `nika-workflow-validator` | Creating/debugging workflows | Validate YAML against schema |
| `nika-workflow-generator` | Starting new workflows | Generate workflow boilerplate |
| `nika-mcp-helper` | Calling MCP tools | Help integrate tools correctly |

**Usage:** Skills activate automatically. Mention "workflow" in conversation.

**See:** `.claude/skills/` directory
```

### Add to settings.json (Optional)

```json
{
  "skills": {
    "enabled": true,
    "paths": [".claude/skills/"],
    "autodiscover": true
  }
}
```

---

## Reference: Skill Frontmatter Spec

| Field | Type | Required | Example |
|-------|------|----------|---------|
| `name` | string | ✅ | `nika-workflow-validator` |
| `description` | string | ✅ | `Validate YAML workflows...` |
| `allowed-tools` | string (comma-sep) | ❌ | `Read, Grep, Glob` |
| `disabled` | boolean | ❌ | `false` |
| `author` | string | ❌ | `Your Name` |
| `tags` | array | ❌ | `[yaml, nika, validation]` |

---

## Quick Reference: Skill Checklist

Before submitting a skill:

**Structure**
- [ ] `.claude/skills/skill-name/SKILL.md` exists
- [ ] Frontmatter has `name:` and `description:`
- [ ] Description includes trigger keywords and use cases
- [ ] Markdown is well-formatted and readable

**Content**
- [ ] "When to Use" section lists specific scenarios
- [ ] Prerequisites documented
- [ ] Step-by-step instructions are clear and numbered
- [ ] At least 2 examples (correct and incorrect)
- [ ] Validation checklist provided

**Quality**
- [ ] No contradictions with `CLAUDE.md`
- [ ] No overlap with existing skills
- [ ] Tool restrictions (`allowed-tools:`) are appropriate
- [ ] Works offline (no external dependencies)
- [ ] Tested with varied prompts

**Integration**
- [ ] Committed to git
- [ ] Listed in `.claude/SKILLS.md`
- [ ] References added to relevant CLAUDE.md sections

---

## Additional Resources

- **Official Documentation:** [Claude Code Skills Guide](https://docs.anthropic.com/en/docs/agents-and-tools/agent-skills/overview)
- **Project Skills:** `.claude/skills/` directory in supernovae-agi
- **Testing:** `spn-powers:testing-skills-with-subagents` skill
- **Nika Schema:** `.nika/config.toml`, `NIKA.md`

---

**Version:** 1.0
**Last Updated:** 2026-03-04
**Applies to:** Nika v0.13+, Claude Code 2026
