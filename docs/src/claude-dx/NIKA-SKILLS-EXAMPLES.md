# Nika Skills - Ready-to-Use Examples

Copy-paste ready skill files for Nika workflow expertise.

**Created:** 2026-03-04
**Format:** SKILL.md frontmatter + markdown
**Installation:** Copy to `.claude/skills/<skill-name>/SKILL.md`

---

## Table of Contents

1. [Skill 1: Nika Workflow Validator](#skill-1-nika-workflow-validator)
2. [Skill 2: Nika Workflow Generator](#skill-2-nika-workflow-generator)
3. [Skill 3: Nika MCP Integration Helper](#skill-3-nika-mcp-integration-helper)
4. [Skill 4: Nika Agent & Skill Definer](#skill-4-nika-agent--skill-definer)
5. [Testing All Skills](#testing-all-skills)

---

## Skill 1: Nika Workflow Validator

**Directory:** `.claude/skills/nika-workflow-validator/`

**File:** `SKILL.md`

```yaml
---
name: nika-workflow-validator
description: Validate YAML workflows against Nika schema. Use when creating new workflows, debugging execution errors, or code reviewing workflow files.
allowed-tools: Read, Grep, Glob
---

# Nika Workflow Validator

Validates Nika YAML workflows for correctness before execution.

## When to Use This Skill

- **Creating new workflows** - Validate syntax and structure
- **Debugging errors** - Identify invalid steps or references
- **Code reviewing** - Check workflow files before merging
- **Refactoring** - Ensure compatibility after changes

## Nika Workflow Basics

**5 Verbs** (valid actions in workflows):
- `infer:` - LLM task (reasoning, generation, analysis)
- `exec:` - Shell command execution
- `fetch:` - HTTP GET request
- `invoke:` - Call MCP tool (novanet_*, etc.)
- `agent:` - Delegate to agent

**Workflow Structure:**
```yaml
workflow: descriptive-name      # Required: unique identifier
version: 1.0                    # Optional: semantic version
steps:                          # Required: list of steps
  - id: step_identifier         # Required: unique within workflow
    [verb]: ...                 # Required: exactly one verb
    use.ctx: output_variable    # Optional: capture output
    on_error: fallback-step     # Optional: error handling
```

**Variable References:**
- `$variable` - Reference from earlier step's `use.ctx:` output
- All references must be defined before use
- Output captured in `use.ctx:` becomes available as `$name`

## Validation Rules

### Rule 1: Valid Verbs

Each step must use exactly one verb from:
- `infer:` (string: description or prompt)
- `exec:` (string: shell command)
- `fetch:` (object: { url, timeout_ms })
- `invoke:` (string: tool name), with `params:` (object)
- `agent:` (object: { agent, workflow, context })

**Error Example:**
```yaml
- id: bad_step
  invalid-verb: "This is not a valid verb"  # ❌ Not in [infer/exec/fetch/invoke/agent]
```

### Rule 2: Variable Definitions

Every `$variable` reference must be defined in an earlier step's `use.ctx:`.

```yaml
steps:
  - id: step_1
    exec: "echo hello"
    use.ctx: greeting           # ✅ Defines $greeting

  - id: step_2
    infer: "Process: $greeting" # ✅ Uses $greeting (defined above)
    use.ctx: result

  - id: step_3
    infer: "Use: $undefined"    # ❌ $undefined never defined
```

### Rule 3: No Circular Dependencies

Workflow must be acyclic (can execute top-to-bottom).

```yaml
# ❌ Bad: circular dependency
steps:
  - id: a
    infer: "Process $b"
    use.ctx: a_result

  - id: b
    infer: "Process $a_result"  # This creates: a→b→a
```

### Rule 4: Context for infer:

All `infer:` steps must have `context:` field with valid reference.

```yaml
- id: infer_step
  infer: "Process this data"
  context: $data              # ✅ Required
  use.ctx: result
```

### Rule 5: Parameters for invoke:

All `invoke:` steps must have `params:` matching tool schema.

```yaml
- id: invoke_step
  invoke: novanet_generate    # ✅ Real MCP tool
  params:                      # ✅ Required
    entity: "my-entity"
    locale: "en-US"
  use.ctx: result
```

## Validation Steps

### Step 1: Read Workflow File

```bash
# Use Read tool
```

Extract:
- Workflow name and version
- List of all step IDs
- Verbs used (infer/exec/fetch/invoke/agent)
- Variables (use.ctx: assignments and $references)

### Step 2: Check Verbs

For each step, verify:
1. Verb is one of: infer, exec, fetch, invoke, agent
2. Verb has appropriate value (string or object)
3. No multiple verbs in same step

### Step 3: Check Variables

1. Collect all `use.ctx:` assignments (left side)
2. Find all `$variable` references (right side)
3. Verify each reference exists in earlier step
4. Check context: fields in infer: steps

### Step 4: Check Tool References

For `invoke:` steps:
- Tool must be valid: `novanet_query`, `novanet_generate`, `novanet_search`, etc.
- `params:` must match tool schema

Valid NovaNet tools:
- `novanet_query` - Cypher queries
- `novanet_generate` - Full context assembly + generation
- `novanet_describe` - Schema introspection
- `novanet_search` - Fulltext search
- `novanet_traverse` - Graph traversal
- `novanet_atoms` - Knowledge atoms (terms, expressions, etc.)
- `novanet_assemble` - Token-aware context assembly
- `novanet_introspect` - Schema metadata
- `novanet_batch` - Bulk operations
- `novanet_cache_stats` - Cache monitoring
- `novanet_cache_invalidate` - Cache clearing

### Step 5: Report Results

Format:

```yaml
workflow: <name>
status: PASS          # or FAIL
error_count: 0        # Number of errors found
errors:               # Detailed list
  - line: 5
    type: "undefined_variable"
    message: "Variable $url not defined"
warnings:             # Non-blocking issues
  - "Consider adding error handler for fetch step"
summary: "Workflow is valid and ready to execute"
```

## Common Errors & Fixes

### Error 1: Undefined Variable

```yaml
❌ WRONG:
steps:
  - id: process
    infer: "Analyze $data"      # $data not defined
    context: {}
```

```yaml
✅ FIX:
steps:
  - id: fetch_data
    fetch:
      url: "https://example.com/api"
    use.ctx: data               # Defines $data

  - id: process
    infer: "Analyze $data"      # Now $data exists
    context: $data
```

### Error 2: Missing Context in infer:

```yaml
❌ WRONG:
steps:
  - id: analyze
    infer: "Do something"       # Missing context:
    use.ctx: result
```

```yaml
✅ FIX:
steps:
  - id: fetch
    fetch: ...
    use.ctx: input

  - id: analyze
    infer: "Do something"
    context: $input             # Added context:
    use.ctx: result
```

### Error 3: Invalid Verb

```yaml
❌ WRONG:
steps:
  - id: bad
    process: "This is not a verb"  # Not in [infer/exec/fetch/invoke/agent]
```

```yaml
✅ FIX:
steps:
  - id: good
    infer: "This is the right verb"
    context: $some_context
```

### Error 4: Missing Parameters for invoke:

```yaml
❌ WRONG:
steps:
  - id: call_tool
    invoke: novanet_generate    # Missing params:
    use.ctx: result
```

```yaml
✅ FIX:
steps:
  - id: call_tool
    invoke: novanet_generate
    params:                      # Added params
      entity: "my-entity"
      locale: "en-US"
    use.ctx: result
```

## Examples

### Example 1: Valid Simple Workflow

```yaml
workflow: hello-world
version: 1.0
steps:
  - id: greet
    exec: "echo 'Hello, Nika!'"
    use.ctx: greeting

  - id: process
    infer: "What should I do with this greeting?"
    context: $greeting
    use.ctx: action
```

**Validation Result:**
```
✅ PASS
- 2 steps, both valid
- $greeting properly chained
- No circular dependencies
- Ready to execute: nika run hello-world.yaml
```

### Example 2: Complex Valid Workflow

```yaml
workflow: content-generation
version: 1.0
steps:
  - id: load_entity
    invoke: novanet_generate
    params:
      entity: "qr-code"
      locale: "fr-FR"
    use.ctx: entity_context

  - id: generate_headline
    infer: "Create a catchy headline for this entity"
    context: $entity_context
    use.ctx: headline

  - id: generate_body
    infer: "Create marketing copy body text"
    context: $entity_context
    use.ctx: body

  - id: combine
    infer: "Combine headline and body into a complete landing page"
    context:
      headline: $headline
      body: $body
    use.ctx: final_page

  - id: save_output
    exec: "mkdir -p output && echo '$final_page' > output/page.html"
```

**Validation Result:**
```
✅ PASS
- 5 steps, all valid
- Variables properly chained: entity_context → headline/body → final_page
- All invoke: steps have valid tool (novanet_generate)
- All infer: steps have context:
- No circular dependencies
- Ready: nika run content-generation.yaml
```

### Example 3: Invalid Workflow with Errors

```yaml
workflow: broken-example
steps:
  - id: fetch_data
    fetch:
      url: $base_url             # ❌ $base_url undefined

  - id: analyze
    infer: "Process this"        # ❌ Missing context:
    use.ctx: analysis

  - id: generate
    invoke: novanet_unknown      # ❌ Invalid tool
    params:
      data: $analysis
```

**Validation Result:**
```
❌ FAIL - 3 errors found:

1. Line 4: Undefined variable
   - $base_url referenced but never defined
   - Fix: Define earlier step with use.ctx: base_url

2. Line 8: Missing required field
   - infer: step missing context: field
   - Fix: Add context: $some_value

3. Line 12: Invalid tool
   - novanet_unknown not a valid MCP tool
   - Valid: novanet_query, novanet_generate, novanet_search, etc.
```

## Validation Checklist

Before reporting success:
- [ ] All steps have valid verbs (one of: infer/exec/fetch/invoke/agent)
- [ ] All variables ($name) defined before use
- [ ] All infer: steps have context:
- [ ] All invoke: steps have params:
- [ ] No circular dependencies
- [ ] Tool names are valid (novanet_*, nika:*)
- [ ] YAML syntax is valid (proper indentation)

## See Also

- **Nika Project:** `.nika/README.md`
- **Related Skills:** `nika-workflow-generator`, `nika-mcp-helper`
- **MCP Tools:** NovaNet MCP documentation
```

---

## Skill 2: Nika Workflow Generator

**Directory:** `.claude/skills/nika-workflow-generator/`

**File:** `SKILL.md`

```yaml
---
name: nika-workflow-generator
description: Generate valid Nika YAML workflows from natural language requirements. Use when starting new workflows, creating workflow examples, or building workflow templates.
---

# Nika Workflow Generator

Generate well-structured YAML workflows that follow Nika conventions.

## When to Use This Skill

- **Starting new workflows** - Generate boilerplate from requirements
- **Creating examples** - Show patterns for documentation
- **Prototyping** - Build proof-of-concept quickly
- **Documenting** - Create reference workflows for patterns

## Before You Start

- Read `.nika/README.md` in your Nika project
- Understand 5 verbs: infer, exec, fetch, invoke, agent
- Know what MCP tools are available in your environment
- Have clarity on workflow inputs/outputs

## How to Generate

### Step 1: Clarify Requirements

Ask user:
1. **Goal** - What should this workflow accomplish?
2. **Inputs** - What data does it need? Variables?
3. **Steps** - What are the main steps? In order?
4. **Outputs** - What should it produce?
5. **Tools** - Which MCP tools or agents?

Example conversation:
```
User: "I want to generate content"
You: "Great! What entity? Which locale? Should we fetch context from NovaNet?"
```

### Step 2: Design Step-by-Step

Map requirements to steps:

```
Requirement: "Fetch data from API"
→ Use: fetch: verb
  Input: API URL
  Output: $api_response

Requirement: "Generate content from data"
→ Use: invoke: novanet_generate verb
  Input: entity name, locale
  Output: $content

Requirement: "Save to file"
→ Use: exec: verb
  Input: filename, content
  Output: None
```

### Step 3: Verify Variable Flow

Trace variables through steps:

```
Step 1 (fetch)
  Output: $api_response
  ↓
Step 2 (invoke)
  Input: params (uses $api_response)
  Output: $content
  ↓
Step 3 (exec)
  Input: $content
  Output: None (final step)
```

### Step 4: Generate YAML

Structure:
```yaml
workflow: <descriptive-name>
version: 1.0
steps:
  - id: step_1
    [verb]: ...
    use.ctx: output_1

  - id: step_2
    [verb]: ...
    context: $output_1
    use.ctx: output_2

  # ... more steps
```

**Naming Convention:**
- Workflow: `kebab-case` (e.g., `content-generator`)
- Step IDs: `snake_case` (e.g., `load_entity`)
- Variables: `snake_case` (e.g., `$entity_data`)

### Step 5: Validate

After generating, validate with `nika-workflow-validator` skill.

## Common Patterns

### Pattern 1: Data Pipeline

**Requirement:** Fetch → Process → Save

```yaml
workflow: data-pipeline
steps:
  - id: fetch_data
    fetch:
      url: $api_url
      timeout_ms: 5000
    use.ctx: raw_data

  - id: process
    infer: "Extract and clean the data"
    context: $raw_data
    use.ctx: processed_data

  - id: save
    exec: "mkdir -p data && echo '$processed_data' > data/output.json"
```

### Pattern 2: Context Assembly + Generation

**Requirement:** Load context from NovaNet → Generate content

```yaml
workflow: content-generator
steps:
  - id: load_context
    invoke: novanet_generate
    params:
      entity: $entity_name      # From workflow input
      locale: $locale           # From workflow input
    use.ctx: entity_context

  - id: generate
    infer: "Create marketing copy"
    context: $entity_context
    use.ctx: content

  - id: output
    exec: "echo '$content'"
```

### Pattern 3: Agent Delegation

**Requirement:** Research topic → Compile results

```yaml
workflow: agent-research
steps:
  - id: research
    agent:
      agent: researcher-agent
      workflow: search-and-analyze
      context: "Topic: $topic"
    use.ctx: research_findings

  - id: compile
    infer: "Write executive summary"
    context: $research_findings
    use.ctx: summary

  - id: report
    exec: "echo '$summary' > report.md"
```

### Pattern 4: Decision Tree

**Requirement:** Fetch → Analyze → Choose action

```yaml
workflow: decision-tree
steps:
  - id: fetch_data
    fetch:
      url: $data_url
    use.ctx: data

  - id: analyze
    infer: "Analyze this data and decide: should we proceed?"
    context: $data
    use.ctx: decision

  - id: handle_yes
    exec: "echo 'Proceeding...'"
    # (in real workflow, you'd use on_error or conditional logic)
```

### Pattern 5: Multi-Step Generation

**Requirement:** Generate multiple outputs from same context

```yaml
workflow: multi-output-generator
steps:
  - id: load_context
    invoke: novanet_generate
    params:
      entity: "product"
      locale: "en-US"
    use.ctx: product_context

  - id: headline
    infer: "Create a catchy product headline"
    context: $product_context
    use.ctx: product_headline

  - id: description
    infer: "Create a detailed product description"
    context: $product_context
    use.ctx: product_description

  - id: cta
    infer: "Create a call-to-action"
    context: $product_context
    use.ctx: product_cta

  - id: bundle
    infer: "Combine into a complete product card"
    context:
      headline: $product_headline
      description: $product_description
      cta: $product_cta
    use.ctx: product_card
```

## Generation Checklist

Before presenting generated workflow:
- [ ] Workflow name is descriptive and kebab-case
- [ ] Each step has unique id (snake_case)
- [ ] Variables flow logically (step 1 → step 2 → ...)
- [ ] All steps use valid verbs (infer/exec/fetch/invoke/agent)
- [ ] All infer: steps have context:
- [ ] All invoke: steps have params: and valid tool name
- [ ] YAML syntax is valid
- [ ] Ready to validate with nika-workflow-validator

## Tips & Patterns

### Tip 1: Clear Step IDs

Use verb_subject format:
- ✅ `fetch_data`, `invoke_generate`, `infer_analysis`, `exec_save`
- ❌ `step1`, `do_thing`, `s1`

### Tip 2: Meaningful Variable Names

```yaml
# ❌ Unclear
use.ctx: data
use.ctx: result

# ✅ Clear
use.ctx: api_response
use.ctx: entity_context
use.ctx: generated_content
```

### Tip 3: Error Handling (Future Enhancement)

```yaml
# Plan for future error handling:
- id: fetch_data
  fetch:
    url: $api_url
  use.ctx: data
  on_error: use_fallback_data  # Future feature
```

### Tip 4: Timeout for Network Operations

```yaml
- id: fetch
  fetch:
    url: $url
    timeout_ms: 5000           # Always set timeout
  use.ctx: response
```

## Examples

### Example 1: Simple CLI Command

**User Request:** "Create a workflow that counts files"

```yaml
workflow: count-files
version: 1.0
steps:
  - id: list_files
    exec: "find . -type f -name '*.md' | wc -l"
    use.ctx: count

  - id: report
    infer: "This is interesting. Comment on this number: $count files"
    context: $count
    use.ctx: comment
```

### Example 2: Data Processing

**User Request:** "Fetch weather data and summarize it"

```yaml
workflow: weather-summary
version: 1.0
steps:
  - id: fetch_weather
    fetch:
      url: "https://api.weather.example.com/current?city=$city"
      timeout_ms: 10000
    use.ctx: weather_data

  - id: parse
    infer: "Extract temperature, condition, and wind speed"
    context: $weather_data
    use.ctx: weather_summary

  - id: friendly_summary
    infer: "Create a brief, friendly weather summary"
    context: $weather_summary
    use.ctx: user_friendly_text

  - id: output
    exec: "echo '$user_friendly_text'"
```

### Example 3: Content Generation with NovaNet

**User Request:** "Generate product page content in French"

```yaml
workflow: product-page-generator
version: 1.0
steps:
  - id: load_product
    invoke: novanet_generate
    params:
      entity: "smartwatch"
      locale: "fr-FR"
      forms: ["text", "title", "abbrev"]
    use.ctx: product_context

  - id: write_headline
    infer: "Create an engaging headline (French)"
    context: $product_context
    use.ctx: headline

  - id: write_body
    infer: "Write detailed product description (French)"
    context: $product_context
    use.ctx: body

  - id: write_cta
    infer: "Write call-to-action button text (French)"
    context: $product_context
    use.ctx: cta_text

  - id: assemble_page
    infer: "Create complete HTML page structure"
    context:
      headline: $headline
      body: $body
      cta: $cta_text
    use.ctx: html_page

  - id: save_file
    exec: "mkdir -p output && echo '$html_page' > output/smartwatch-fr.html"
```

## See Also

- **Related Skills:** `nika-workflow-validator`, `nika-mcp-helper`
- **Nika Docs:** `.nika/README.md`
- **Workflow Schema:** `.nika/config.toml`
- **MCP Tools:** NovaNet reference
```

---

## Skill 3: Nika MCP Integration Helper

**Directory:** `.claude/skills/nika-mcp-integration-helper/`

**File:** `SKILL.md`

```yaml
---
name: nika-mcp-integration-helper
description: Help integrate MCP tools (novanet_*, etc.) into Nika workflows. Use when calling MCP tools, assembling context, or debugging tool invocation errors.
---

# Nika MCP Integration Helper

Help developers correctly integrate Model Context Protocol (MCP) tools into Nika workflows.

## When to Use This Skill

- **Calling MCP tools** - Know which tool to use and how
- **Assembling context** - Build proper parameters for tools
- **Debugging errors** - Fix tool invocation problems
- **Learning tools** - Understand what each tool does

## MCP Tools Overview

### NovaNet MCP Tools (11 tools)

**Query Tools (Read-only):**
- `novanet_query` - Execute Cypher queries
- `novanet_search` - Fulltext/property search
- `novanet_traverse` - Graph traversal with filters
- `novanet_introspect` - Schema introspection

**Generation Tools:**
- `novanet_generate` - Full RLM-on-KG context assembly
- `novanet_assemble` - Token-aware context assembly
- `novanet_describe` - Schema descriptions

**Context Tools:**
- `novanet_atoms` - Knowledge atoms (terms, expressions, patterns)

**Batch & Cache:**
- `novanet_batch` - Bulk operations
- `novanet_cache_stats` - Cache monitoring
- `novanet_cache_invalidate` - Clear cache

### Built-in Nika Tools

- `nika:run` - Run shell commands
- `nika:sleep` - Wait N milliseconds
- `nika:log` - Write to log
- `nika:emit` - Send events
- `nika:assert` - Validate conditions
- `nika:prompt` - Ask user for input

## NovaNet Tool Reference

### Tool 1: novanet_generate

**Purpose:** Full context assembly for LLM generation

**When to Use:**
- Generating content in multiple locales
- Need complete entity context + locale knowledge
- Want token-aware context assembly

**Parameters:**
```yaml
invoke: novanet_generate
params:
  focus_key: "entity-slug"           # Required: entity ID
  locale: "en-US"                    # Required: target locale
  mode: "block"                       # Optional: block|page (default: page)
  token_budget: 8000                 # Optional: max tokens
  spreading_depth: 2                 # Optional: context depth
```

**Output:** Complete context for generation
- `prompt` - Assembled prompt for LLM
- `evidence_summary` - What was included
- `locale_context` - Locale-specific knowledge
- `denomination_forms` - Canonical entity names
- `context_build_log` - Debug info

**Example:**
```yaml
steps:
  - id: load_context
    invoke: novanet_generate
    params:
      focus_key: "qr-code"
      locale: "fr-FR"
      mode: "page"
      token_budget: 6000
    use.ctx: generation_context
```

### Tool 2: novanet_query

**Purpose:** Execute arbitrary Cypher queries

**When to Use:**
- Custom graph queries
- Specific data lookups
- Complex graph patterns

**Parameters:**
```yaml
invoke: novanet_query
params:
  cypher: "MATCH (n:Entity) RETURN n.name LIMIT 5"
  params: {}                    # Cypher parameters
  limit: 100                    # Max results
  timeout_ms: 5000              # Query timeout
```

**Example:**
```yaml
- id: query_entities
  invoke: novanet_query
  params:
    cypher: "MATCH (e:Entity) WHERE e.realm = 'org' RETURN e.key"
    limit: 50
  use.ctx: entity_list
```

### Tool 3: novanet_search

**Purpose:** Fulltext or property search

**When to Use:**
- Finding entities by name
- Searching across properties
- Hybrid search (text + metadata)

**Parameters:**
```yaml
invoke: novanet_search
params:
  query: "qr code"               # Search string
  mode: "hybrid"                 # fulltext|property|hybrid
  kinds: ["Entity"]              # Optional: filter by kind
  limit: 20                       # Max results
```

**Example:**
```yaml
- id: search
  invoke: novanet_search
  params:
    query: "qr"
    mode: "fulltext"
    kinds: ["Entity"]
  use.ctx: search_results
```

### Tool 4: novanet_traverse

**Purpose:** Graph traversal with configurable depth

**When to Use:**
- Walking relationships from entity
- Finding connected entities
- Exploring graph patterns

**Parameters:**
```yaml
invoke: novanet_traverse
params:
  start_key: "my-entity"         # Starting node
  max_depth: 2                   # How far to traverse
  direction: "outgoing"          # outgoing|incoming|both
  arc_families: ["ownership"]    # Optional: filter relationships
```

**Example:**
```yaml
- id: traverse
  invoke: novanet_traverse
  params:
    start_key: "qr-code"
    max_depth: 1
    direction: "outgoing"
    arc_families: ["semantic"]
  use.ctx: related_entities
```

### Tool 5: novanet_atoms

**Purpose:** Get knowledge atoms (terms, expressions, patterns)

**When to Use:**
- Need locale-specific terminology
- Building cultural knowledge
- Semantic context

**Parameters:**
```yaml
invoke: novanet_atoms
params:
  locale: "fr-FR"                # Required
  atom_type: "term"              # term|expression|pattern|cultureref|all
  domain: "general"              # Optional: domain filter
```

**Example:**
```yaml
- id: get_terms
  invoke: novanet_atoms
  params:
    locale: "es-MX"
    atom_type: "term"
  use.ctx: spanish_terms
```

### Tool 6: novanet_describe

**Purpose:** Get schema descriptions and metadata

**When to Use:**
- Learning schema structure
- Bootstrap agent context
- Understand relationships

**Parameters:**
```yaml
invoke: novanet_describe
params:
  describe: "schema"             # schema|entity|category|relations
  entity_key: "optional"         # For entity descriptions
```

**Example:**
```yaml
- id: describe_schema
  invoke: novanet_describe
  params:
    describe: "schema"
  use.ctx: schema_info
```

### Tool 7: novanet_introspect

**Purpose:** Query schema metadata

**When to Use:**
- Getting class/arc definitions
- Introspection queries
- Meta-schema operations

**Parameters:**
```yaml
invoke: novanet_introspect
params:
  target: "classes"              # classes|class|arcs|arc
  realm: "org"                   # Optional: filter by realm
```

**Example:**
```yaml
- id: introspect
  invoke: novanet_introspect
  params:
    target: "classes"
    realm: "shared"
  use.ctx: class_definitions
```

### Tool 8: novanet_assemble

**Purpose:** Token-aware context assembly

**When to Use:**
- Limited token budget
- Selective context building
- Optimized generation

**Parameters:**
```yaml
invoke: novanet_assemble
params:
  focus_key: "entity"            # Central entity
  locale: "en-US"
  token_budget: 4000             # Tight budget
  strategy: "breadth"            # breadth|depth|relevance|custom
```

**Example:**
```yaml
- id: assemble_lite
  invoke: novanet_assemble
  params:
    focus_key: "qr-code"
    locale: "en-US"
    token_budget: 3000
    strategy: "relevance"
  use.ctx: optimized_context
```

### Tool 9: novanet_batch

**Purpose:** Execute multiple operations in parallel

**When to Use:**
- Multiple independent queries
- Bulk operations
- Parallel context assembly

**Parameters:**
```yaml
invoke: novanet_batch
params:
  operations:                    # Array of operations
    - tool: "novanet_query"
      params: { ... }
    - tool: "novanet_search"
      params: { ... }
  max_concurrent: 3              # Parallel limit
  fail_fast: false               # Stop on error?
```

**Example:**
```yaml
- id: batch_queries
  invoke: novanet_batch
  params:
    operations:
      - tool: "novanet_query"
        params: { cypher: "MATCH (e:Entity) RETURN count(e)" }
      - tool: "novanet_search"
        params: { query: "qr", mode: "fulltext" }
    max_concurrent: 2
  use.ctx: batch_results
```

### Tool 10 & 11: Cache Management

**novanet_cache_stats** - Get cache statistics
```yaml
invoke: novanet_cache_stats
params: {}                       # No parameters
```

**novanet_cache_invalidate** - Clear cache
```yaml
invoke: novanet_cache_invalidate
params:
  clear_all: true                # Or pattern: "query:*"
```

## Decision Tree: Which Tool to Use?

```
Question 1: What are you doing?
├─ Reading data
│  ├─ Custom query? → novanet_query (Cypher)
│  ├─ Search? → novanet_search (fulltext/property)
│  ├─ Related entities? → novanet_traverse (graph walk)
│  ├─ Schema info? → novanet_describe (introspection)
│  ├─ Knowledge atoms? → novanet_atoms (terms/expressions)
│  └─ Multiple queries? → novanet_batch (parallel)
│
├─ Generating content
│  ├─ Full context? → novanet_generate (complete assembly)
│  └─ Limited tokens? → novanet_assemble (optimized)
│
└─ Managing cache
   ├─ See stats? → novanet_cache_stats
   └─ Clear cache? → novanet_cache_invalidate
```

## Common Patterns

### Pattern 1: Generate with Full Context

```yaml
workflow: full-generation
steps:
  - id: load_context
    invoke: novanet_generate
    params:
      focus_key: $entity
      locale: $locale
      mode: "page"
      token_budget: 8000
    use.ctx: context

  - id: generate
    infer: "Create content using this context"
    context: $context
    use.ctx: content
```

### Pattern 2: Search + Generate

```yaml
workflow: search-and-generate
steps:
  - id: search
    invoke: novanet_search
    params:
      query: $search_query
      mode: "fulltext"
    use.ctx: search_results

  - id: process_results
    infer: "Pick best result and prepare for generation"
    context: $search_results
    use.ctx: selected_entity

  - id: generate
    invoke: novanet_generate
    params:
      focus_key: $selected_entity
      locale: $locale
    use.ctx: final_content
```

### Pattern 3: Batch Queries

```yaml
workflow: parallel-queries
steps:
  - id: parallel_lookups
    invoke: novanet_batch
    params:
      operations:
        - tool: "novanet_query"
          params: { cypher: "MATCH (e:Entity) RETURN count(e)" }
        - tool: "novanet_describe"
          params: { describe: "schema" }
      max_concurrent: 2
    use.ctx: results
```

## Debugging Common Errors

### Error 1: Missing Required Parameters

**Error Message:** "Missing required parameter: focus_key"

**Fix:**
```yaml
# ❌ Wrong
invoke: novanet_generate
params:
  locale: "en-US"

# ✅ Correct
invoke: novanet_generate
params:
  focus_key: "my-entity"         # Added required param
  locale: "en-US"
```

### Error 2: Invalid Tool Name

**Error Message:** "Unknown tool: novanet_xyz"

**Fix:**
- Check spelling: `novanet_generate` (not `novanet_gen`)
- Use only: query, search, traverse, generate, describe, atoms, assemble, introspect, batch, cache_stats, cache_invalidate

### Error 3: Invalid Locale

**Error Message:** "Locale en-UK not found"

**Fix:**
- Use standard locales: `en-US`, `fr-FR`, `es-MX`, `de-DE`, `ja-JP`, etc.
- Check available locales in Neo4j: Locale nodes in database

### Error 4: Context Reference Error

**Error Message:** "$entity not defined"

**Fix:**
```yaml
# ❌ Wrong
steps:
  - id: search
    invoke: novanet_search
    params:
      query: $entity             # $entity not defined yet

# ✅ Correct
steps:
  - id: search
    fetch: $entity_url
    use.ctx: entity

  - id: generate
    invoke: novanet_generate
    params:
      focus_key: $entity         # Now $entity exists
```

## See Also

- **Nika Workflow Validator** - Check your workflows
- **Nika Workflow Generator** - Generate workflows
- **NovaNet Docs** - Full tool reference
```

---

## Skill 4: Nika Agent & Skill Definer

**Directory:** `.claude/skills/nika-agent-skill-definer/`

**File:** `SKILL.md`

```yaml
---
name: nika-agent-skill-definer
description: Define Nika agents and skills in YAML. Use when creating agents (.agent.yaml), skills (.skill.yaml), or structuring .nika/ directories.
---

# Nika Agent & Skill Definer

Create valid agent and skill definitions for Nika.

## When to Use This Skill

- **Creating agents** - Define `.agent.yaml` files
- **Creating skills** - Define `.skill.yaml` or `SKILL.yaml` files
- **Structuring projects** - Set up `.nika/` directory
- **Learning formats** - Understand Nika YAML structures

## Agent Definition Format

### Basic Agent Structure

**File:** `.nika/agents/my-agent.agent.yaml`

```yaml
agent:
  id: my-agent                      # Unique identifier
  schema: 1.0                       # Schema version
  soul:
    role: "Content Writer"          # What does it do?
    mission: "Write engaging content"  # Why?
    personality: "Professional and creative"
    values:
      - accuracy
      - clarity
      - accessibility

  rules:
    must:
      - "Always cite sources"
      - "Use simple language"
    never:
      - "Plagiarize"
      - "Make up facts"

  workflow:
    mcp:
      - novanet_generate
      - novanet_search
    max_turns: 10                   # Conversation limit
    depth_limit: 3                  # Context depth
    stop_conditions:
      - "User says done"
      - "Generated 5 pages"

  handoffs:
    - to: reviewer-agent
      when: "Content review needed"
      context: "generated_content"
```

### Agent Properties

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `id` | string | ✅ | kebab-case identifier |
| `schema` | number | ✅ | 1.0 currently |
| `soul.role` | string | ✅ | What the agent does |
| `soul.mission` | string | ✅ | Why it exists |
| `soul.personality` | string | ❌ | Agent personality |
| `soul.values` | array | ❌ | Core values list |
| `rules.must` | array | ❌ | Required behaviors |
| `rules.never` | array | ❌ | Forbidden behaviors |
| `workflow.mcp` | array | ❌ | Available MCP tools |
| `workflow.max_turns` | number | ❌ | Conversation limit |
| `handoffs` | array | ❌ | Agent handoff definitions |

### Example Agents

**Agent 1: Content Writer**
```yaml
agent:
  id: content-writer
  schema: 1.0
  soul:
    role: "Content Creator"
    mission: "Generate engaging marketing content"
    personality: "Professional, creative, brand-aware"
    values:
      - clarity
      - engagement
      - brand_consistency

  rules:
    must:
      - "Write in active voice"
      - "Use customer-centric language"
      - "Include calls-to-action"
    never:
      - "Use jargon without explanation"
      - "Write more than 500 words per section"

  workflow:
    mcp:
      - novanet_generate
      - novanet_search
    max_turns: 8
```

**Agent 2: Code Reviewer**
```yaml
agent:
  id: code-reviewer
  schema: 1.0
  soul:
    role: "Code Quality Guardian"
    mission: "Review code for quality, security, and maintainability"
    values:
      - correctness
      - readability
      - security

  rules:
    must:
      - "Check for security vulnerabilities"
      - "Verify error handling exists"
      - "Confirm tests exist"
    never:
      - "Skip edge case analysis"
      - "Approve code with clippy warnings"

  workflow:
    mcp: []
    max_turns: 5
```

## Skill Definition Formats

### Format 1: File-Based Skill (.skill.yaml)

**File:** `.nika/skills/my-skill.skill.yaml`

```yaml
skill:
  id: my-skill                       # Unique identifier
  schema: 1.0                        # Schema version
  name: "My Skill"                   # Display name
  description: "What this skill does"
  params:                            # Input parameters
    - name: input_text
      description: "Text to process"
      required: true
      default: ""
    - name: format
      description: "Output format"
      required: false
      default: "markdown"

  workflow:                          # Inline workflow
    steps:
      - id: process
        infer: "Process this: $input_text"
        context: $input_text
        use.ctx: result

  output:                            # Output format
    format: "markdown"               # json|yaml|markdown|text
    template: "# Result\n\n$result"  # Handlebars template
```

### Format 2: Directory-Based Skill (SKILL.yaml)

**File:** `.nika/skills/my-skill/SKILL.yaml`

```yaml
skill:
  id: my-skill
  schema: 1.0
  name: "My Skill"
  description: "What this skill does"

  params:
    - name: input
      description: "Input data"
      required: true

  workflow:
    path: "./workflow.yaml"          # Path to workflow file

  output:
    format: "markdown"
    template: "./template.md.hbs"    # Path to template
```

**Directory Structure:**
```
.nika/skills/my-skill/
├── SKILL.yaml                       # Skill definition
├── workflow.yaml                    # Workflow (if using path)
├── template.md.hbs                  # Output template
└── examples/                        # Optional examples
    └── example-1.md
```

### Skill Properties

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `id` | string | ✅ | kebab-case identifier |
| `schema` | number | ✅ | 1.0 currently |
| `name` | string | ✅ | Display name |
| `description` | string | ✅ | What it does |
| `params` | array | ❌ | Input parameters |
| `workflow` | object | ✅ | Inline or path |
| `output.format` | string | ✅ | json/yaml/markdown/text |
| `output.template` | string | ✅ | Template or path |

### Example Skills

**Skill 1: Summarizer (File-Based)**

```yaml
skill:
  id: summarizer
  schema: 1.0
  name: "Summarizer"
  description: "Summarize long text into concise points"

  params:
    - name: text
      description: "Text to summarize"
      required: true
    - name: max_points
      description: "Maximum bullet points"
      required: false
      default: "5"

  workflow:
    steps:
      - id: analyze
        infer: "Extract key points from this text (max $max_points points)"
        context: $text
        use.ctx: points

  output:
    format: "markdown"
    template: |
      # Summary

      Key points:
      $points
```

**Skill 2: Generator (Directory-Based)**

**File:** `.nika/skills/generator/SKILL.yaml`
```yaml
skill:
  id: generator
  schema: 1.0
  name: "Content Generator"
  description: "Generate content using NovaNet context"

  params:
    - name: entity
      description: "Entity to generate for"
      required: true
    - name: locale
      description: "Target locale"
      required: true

  workflow:
    path: "./workflow.yaml"

  output:
    format: "markdown"
    template: "./template.md.hbs"
```

**File:** `.nika/skills/generator/workflow.yaml`
```yaml
workflow: generate-content
steps:
  - id: load_context
    invoke: novanet_generate
    params:
      focus_key: $entity
      locale: $locale
    use.ctx: context

  - id: generate
    infer: "Create content"
    context: $context
    use.ctx: content
```

**File:** `.nika/skills/generator/template.md.hbs`
```
# Generated Content

For: {{entity}} ({{locale}})

## Content

{{{content}}}
```

## .nika/ Directory Structure (Complete)

```
.nika/
├── config.toml                   # Main configuration
├── user.yaml                     # User profile
├── memory.yaml                   # Memory config
├── policies.yaml                 # Security policies
├── agents/                       # Agent definitions
│   ├── researcher.agent.yaml     # Research agent
│   ├── writer.agent.yaml         # Writing agent
│   └── reviewer.agent.yaml       # Review agent
├── skills/                       # Skill definitions
│   ├── summarize.skill.yaml      # File-based skill
│   └── generator/                # Directory-based skill
│       ├── SKILL.yaml
│       ├── workflow.yaml
│       └── template.md.hbs
├── workflows/                    # Workflow definitions
│   ├── research.yaml
│   └── write.yaml
├── context/                      # Shared context files
│   └── project-context.md
├── memory/                       # Persistent memory
│   └── session-memory.json
├── proposed/                     # Proposed changes
│   └── 2026-02-26-feature.diff
├── sessions/                     # TUI session state
│   └── session-123.json
├── traces/                       # Execution traces
│   └── trace-456.ndjson
└── cache/                        # Temporary cache
    └── mcp-schemas.json
```

## Creation Workflow

### Create an Agent

**Step 1:** Create file
```bash
mkdir -p .nika/agents
touch .nika/agents/my-agent.agent.yaml
```

**Step 2:** Write YAML (use example above)

**Step 3:** Validate
```bash
# Manual check: nika can load agents
nika config show --agents
```

### Create a File-Based Skill

**Step 1:** Create file
```bash
touch .nika/skills/my-skill.skill.yaml
```

**Step 2:** Write YAML
- Add skill metadata
- Define params
- Write inline workflow
- Set output format

**Step 3:** Test
```bash
nika skill invoke my-skill --input "test data"
```

### Create a Directory-Based Skill

**Step 1:** Create directory
```bash
mkdir -p .nika/skills/my-skill
touch .nika/skills/my-skill/SKILL.yaml
```

**Step 2:** Create workflow
```bash
touch .nika/skills/my-skill/workflow.yaml
```

**Step 3:** Create output template
```bash
touch .nika/skills/my-skill/template.md.hbs
```

**Step 4:** Test
```bash
nika skill invoke my-skill --entity qr-code --locale en-US
```

## Validation Checklist

### For Agents

- [ ] ID is kebab-case (my-agent, not my_agent)
- [ ] schema: 1.0
- [ ] soul has role and mission
- [ ] rules have must or never (or both)
- [ ] workflow has mcp tools list (can be empty)
- [ ] YAML syntax is valid

### For Skills

- [ ] ID is kebab-case
- [ ] schema: 1.0
- [ ] name is human-readable
- [ ] description explains purpose
- [ ] params are well-documented
- [ ] workflow is valid YAML
- [ ] output format is one of: json/yaml/markdown/text
- [ ] For directory skills: all files exist

## See Also

- **Nika Project:** `.nika/README.md`
- **Related Skills:** `nika-workflow-validator`, `nika-workflow-generator`
- **Nika Docs:** Agent and skill documentation
```

---

## Testing All Skills

### Test Setup

Before using skills in production, test each one:

```bash
# Test 1: Explicit invocation
"Check my workflow with /nika-workflow-validator"

# Test 2: Implicit activation
"I'm creating a Nika workflow that calls novanet_generate"

# Test 3: Integration
"Generate a workflow that fetches data, then generates content, then saves the result"

# Test 4: Error handling
"Validate this broken workflow: [invalid YAML]"
```

### Expected Behavior

**Skill 1: Workflow Validator**
- Activates on keywords: "validate", "check", "broken", "error"
- Provides step-by-step validation
- Reports errors with line numbers
- Success: ✅ or Failure: ❌

**Skill 2: Workflow Generator**
- Activates on keywords: "generate", "create", "new workflow", "template"
- Asks clarifying questions
- Generates valid YAML
- Ready for validator

**Skill 3: MCP Helper**
- Activates on keywords: "MCP tool", "invoke", "novanet_*"
- Explains which tool to use
- Provides correct parameters
- Shows working examples

**Skill 4: Agent Definer**
- Activates on keywords: "agent", ".agent.yaml", "skill.yaml"
- Shows correct YAML structures
- Validates definitions
- Provides examples

### Verification Checklist

- [ ] All skills activate on relevant keywords
- [ ] Instructions are clear and actionable
- [ ] Examples are correct and executable
- [ ] No contradictions with project CLAUDE.md
- [ ] Ready for team git commit

---

**Version:** 1.0
**Created:** 2026-03-04
**Status:** Ready for production use
