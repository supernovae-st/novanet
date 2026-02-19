# Plan D: Nika TUI + NovaNet Full Verification

> Verify Nika TUI works with REAL data (no mocks) and demonstrate with comprehensive scenarios.

**Status:** NOT STARTED
**Priority:** High (validates MVP 7 completeness)
**Prerequisites:** Nika v0.4 complete, NovaNet MCP running, Neo4j seeded

---

## Objective

Verify that:
1. Nika TUI is 100% functional (not a mockup)
2. Connects to REAL NovaNet MCP (not MockMcpClient)
3. Displays REAL data from Neo4j
4. Handles all workflow patterns (simple → complex)
5. Agent loop works with real LLM calls

---

## Part 1: Technical Verification

### 1.1 Compilation Check

```bash
cd nika-dev/tools/nika

# Build with TUI feature
cargo build --features tui

# Run TUI
cargo run -- tui examples/simple.yaml
```

**Expected:** Compiles without errors, TUI launches.

### 1.2 No Mocks in Production Code

Search for mock/stub patterns:

```bash
# Find mocks
grep -r "mock\|stub\|fake\|test_data\|MockClient" src/

# Find TODOs/unimplemented
grep -r "TODO\|FIXME\|unimplemented!\|todo!" src/tui/

# Find hardcoded data
grep -r "Lorem\|example\.com\|test@" src/
```

**Verify:**
- [ ] `McpClient` connects to real NovaNet MCP (no `MockMcpClient`)
- [ ] `RigAgentLoop` calls real Claude API (no fake responses)
- [ ] `EventLog` comes from real workflow execution
- [ ] No hardcoded test data in production paths

### 1.3 TUI Architecture

List TUI files and their responsibilities:

```bash
ls -la src/tui/
```

**Expected structure:**
```
src/tui/
├── mod.rs           # Module exports
├── app.rs           # State machine (Running, Paused, Finished)
├── ui.rs            # 4-panel renderer
├── panels/          # Individual panel components
│   ├── workflow.rs  # Left: workflow tree
│   ├── details.rs   # Center: task details
│   ├── events.rs    # Right: event log
│   └── status.rs    # Bottom: status bar
└── events.rs        # Keyboard/event handling
```

### 1.4 Show TUI Layout (ASCII)

Draw what the TUI should look like with REAL data:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NIKA TUI v0.4                                              [?] Help [q] Quit│
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─ Workflow Tree ───────┬─ Task Details ──────────────┬─ Event Log ──────┐ │
│  │                       │                             │                  │ │
│  │ ▼ generate-page       │ Task: get_entity           │ 10:42:01.123     │ │
│  │   ├─ ✓ get_entity     │ Status: Completed ✓        │ WorkflowStarted  │ │
│  │   ├─ ✓ get_seo        │ Duration: 1.23s            │                  │ │
│  │   ├─ ⟳ generate_hero  │                            │ 10:42:01.456     │ │
│  │   ├─ ◯ generate_faq   │ Input:                     │ TaskStarted      │ │
│  │   └─ ◯ combine_page   │   entity_key: "qr-code"    │ get_entity       │ │
│  │                       │   locale: "fr-FR"          │                  │ │
│  │                       │                            │ 10:42:02.789     │ │
│  │ Legend:               │ Output:                    │ McpToolCalled    │ │
│  │ ✓ Completed           │   display_name: "Code QR"  │ novanet_describe │ │
│  │ ⟳ Running             │   description: "Un code..."│                  │ │
│  │ ◯ Pending             │   denomination_forms:      │ 10:42:03.012     │ │
│  │ ✗ Failed              │     text: "code QR"        │ McpToolResponded │ │
│  │                       │     title: "Code QR"       │ 234ms            │ │
│  │                       │     abbrev: "QR"           │                  │ │
│  │                       │                            │ 10:42:03.456     │ │
│  │                       │ Tokens: 0 in / 0 out       │ TaskCompleted    │ │
│  │                       │ (no LLM for invoke)        │ get_entity       │ │
│  │                       │                            │                  │ │
│  ├───────────────────────┴────────────────────────────┴──────────────────┤ │
│  │ Status: Running | Tasks: 2/5 | Events: 6 | Elapsed: 2.34s | ▸ Play   │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
│  Keybindings: [↑↓] Navigate  [Enter] Expand  [Space] Pause  [r] Restart    │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Part 2: Scenario Categories

### Level 1: Simple Workflows (UC1-UC5)

#### UC1: Hello World (Single Infer)

```yaml
# examples/uc01-hello-world.yaml
workflow: hello-world
tasks:
  - id: greet
    infer: "Say hello in French"
```

**DAG:**
```
greet
```

**Expected TUI:**
```
┌─ Workflow ─────┬─ Details ─────────────────┬─ Events ─────────────┐
│ ✓ greet        │ Task: greet               │ WorkflowStarted      │
│                │ Status: Completed         │ TaskStarted: greet   │
│                │ Duration: 0.8s            │ LlmRequestSent       │
│                │                           │ LlmResponseReceived  │
│                │ Output: "Bonjour !"       │ TaskCompleted: greet │
│                │ Tokens: 12 in / 8 out     │ WorkflowCompleted    │
├────────────────┴───────────────────────────┴──────────────────────┤
│ Status: Completed | Tasks: 1/1 | Tokens: 20 | 0.8s               │
└───────────────────────────────────────────────────────────────────┘
```

---

#### UC2: Fetch Then Infer (Linear)

```yaml
# examples/uc02-fetch-infer.yaml
workflow: fetch-then-infer
tasks:
  - id: get_data
    fetch: https://api.github.com/repos/anthropics/claude-code

  - id: summarize
    needs: [get_data]
    infer: "Summarize this repo info: $get_data"
```

**DAG:**
```
get_data ──→ summarize
```

---

#### UC3: Invoke NovaNet Describe

```yaml
# examples/uc03-invoke-describe.yaml
workflow: describe-entity
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: describe
    invoke: novanet_describe
    params:
      entity_key: "qr-code"
      locale: "fr-FR"
```

**Expected:** Real data from Neo4j (not mock).

---

#### UC4: Invoke NovaNet Generate (ADR-033)

```yaml
# examples/uc04-invoke-generate.yaml
workflow: generate-forms
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: generate
    invoke: novanet_generate
    params:
      entity_key: "qr-code"
      locale: "fr-FR"
      forms: ["text", "title", "abbrev", "url"]

  - id: use_forms
    needs: [generate]
    infer: |
      Create a sentence using these denomination forms:
      $generate.denomination_forms
```

**Verify:** `denomination_forms` returned per ADR-033.

---

#### UC5: Traverse Graph

```yaml
# examples/uc05-traverse.yaml
workflow: traverse-graph
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_natives
    invoke: novanet_traverse
    params:
      start: "entity:qr-code"
      arc: "HAS_NATIVE"
      depth: 1

  - id: get_related
    invoke: novanet_traverse
    params:
      start: "entity:qr-code"
      arc: "RELATES_TO"
      depth: 2

  - id: summarize
    needs: [get_natives, get_related]
    infer: |
      Summarize the entity graph:
      Natives: $get_natives
      Related: $get_related
```

---

### Level 2: Parallel Execution (UC6-UC7)

#### UC6: Parallel Locales (for_each)

```yaml
# examples/uc06-parallel-locales.yaml
workflow: parallel-locales
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: define_locales
    exec: |
      echo '["fr-FR", "en-US", "de-DE", "es-ES", "ja-JP"]'

  - id: generate_all
    needs: [define_locales]
    for_each:
      items: $define_locales
      as: locale
      concurrency: 3
    invoke: novanet_generate
    params:
      entity_key: "qr-code"
      locale: $locale
      forms: ["text", "title"]

  - id: report
    needs: [generate_all]
    infer: |
      Create a comparison table of denomination forms across locales:
      $generate_all
```

**DAG:**
```
define_locales ──→ generate_all (for_each, concurrency: 3) ──→ report
                   ├── fr-FR ─┐
                   ├── en-US ─┼── [parallel]
                   ├── de-DE ─┤
                   ├── es-ES ─┤
                   └── ja-JP ─┘
```

**TUI Snapshot:**
```
┌─ Workflow ────────────┬─ for_each Progress ────────────┬─ Events ────────┐
│ ✓ define_locales      │ Task: generate_all             │ ForEachStarted  │
│ ⟳ generate_all        │ Status: 3/5 complete           │ ItemStarted:0   │
│   ├─ ✓ [0] fr-FR      │                                │ ItemStarted:1   │
│   ├─ ✓ [1] en-US      │ ████████████░░░░░░░░ 60%      │ ItemStarted:2   │
│   ├─ ✓ [2] de-DE      │                                │ ItemComplete:0  │
│   ├─ ⟳ [3] es-ES      │ Concurrency: 2/3 active        │ ItemComplete:1  │
│   └─ ⟳ [4] ja-JP      │ • es-ES (invoking...)          │ ItemComplete:2  │
│ ◯ report              │ • ja-JP (invoking...)          │ ItemStarted:3   │
│                       │                                │ ItemStarted:4   │
│                       │ Results so far:                │                 │
│                       │ fr-FR: "code QR"               │                 │
│                       │ en-US: "QR code"               │                 │
│                       │ de-DE: "QR-Code"               │                 │
├───────────────────────┴────────────────────────────────┴─────────────────┤
│ Progress: 60% | Items: 3/5 | Concurrency: 2/3 | Elapsed: 4.5s           │
└──────────────────────────────────────────────────────────────────────────┘
```

---

#### UC7: Fan-Out Fan-In

```yaml
# examples/uc07-fan-out-fan-in.yaml
workflow: fan-out-fan-in
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: search_entities
    invoke: novanet_search
    params:
      query: "realm:org trait:defined"
      limit: 5

  - id: enrich_each
    needs: [search_entities]
    for_each:
      items: $search_entities.results
      as: entity
      concurrency: 5
    invoke: novanet_describe
    params:
      entity_key: $entity.key
      locale: "fr-FR"

  - id: aggregate
    needs: [enrich_each]
    infer: |
      Create a summary document combining all entity descriptions:
      $enrich_each

      Format as:
      1. Overview
      2. Entity list with key details
      3. Relationships between them
```

**DAG:**
```
search_entities ──→ enrich_each (fan-out) ──→ aggregate (fan-in)
                    ├── entity[0] ─┐
                    ├── entity[1] ─┤
                    ├── entity[2] ─┼────→ aggregate
                    ├── entity[3] ─┤
                    └── entity[4] ─┘
```

---

### Level 3: Agent Loop (UC8-UC11)

#### UC8: Agent with MCP Tools

```yaml
# examples/uc08-agent-tools.yaml
workflow: agent-researcher
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: research
    agent:
      prompt: |
        You are a research agent. Investigate the "qr-code" entity in NovaNet.

        Your mission:
        1. First, describe the entity to understand it
        2. Find what locales have native content
        3. Discover related entities
        4. Summarize your findings

        Use tools step by step. Think before each action.
      tools:
        - novanet_describe
        - novanet_traverse
        - novanet_search
      max_turns: 5
```

**TUI for Agent:**
```
┌─ Workflow ─────────────┬─ Agent Loop ───────────────────┬─ Events ───────┐
│ ⟳ research             │ Agent: research                │ AgentStarted   │
│   ├─ Turn 1 ✓          │ Turn: 3/5                      │ Turn 1 started │
│   │  └─ novanet_desc   │ Model: claude-sonnet-4         │ ToolCall: desc │
│   ├─ Turn 2 ✓          │                                │ ToolResult     │
│   │  └─ novanet_trav   │ Current thinking:              │ Turn 1 done    │
│   └─ Turn 3 ⟳          │ "Now I should find related     │ Turn 2 started │
│      └─ novanet_search │  entities to complete my       │ ToolCall: trav │
│                        │  research..."                  │ ToolResult     │
│                        │                                │ Turn 2 done    │
│                        │ Last tool call:                │ Turn 3 started │
│                        │ novanet_search                 │ ToolCall: srch │
│                        │ { query: "RELATES_TO qr-code" }│                │
│                        │                                │                │
│                        │ Tokens: 2,345 in / 456 out     │                │
├────────────────────────┴────────────────────────────────┴────────────────┤
│ Agent: Turn 3/5 | Tools: 3 calls | Tokens: 2,801 | Cost: ~$0.03 | 12.3s │
└──────────────────────────────────────────────────────────────────────────┘
```

---

#### UC9: Agent with Context from Previous Task

```yaml
# examples/uc09-agent-context.yaml
workflow: agent-with-context
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_context
    invoke: novanet_assemble
    params:
      entity_key: "qr-code-restaurant"
      locale: "fr-FR"
      include: ["atoms", "seo", "related"]

  - id: write_page
    needs: [get_context]
    agent:
      prompt: |
        You are a content writer. Write a landing page for QR codes in restaurants.

        Context from knowledge graph:
        $get_context

        Requirements:
        - Use the denomination_forms for consistent naming
        - Include SEO keywords naturally
        - Mention related use cases
        - Write in French

        Structure:
        1. Hero section (H1 + intro)
        2. Benefits (3-5 points)
        3. How it works
        4. CTA
      tools:
        - novanet_search  # For additional research if needed
      max_turns: 4
```

---

#### UC10: Diamond DAG

```yaml
# examples/uc10-diamond-dag.yaml
workflow: diamond-dag
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: A_fetch
    invoke: novanet_describe
    params:
      entity_key: "qr-code"
      locale: "en-US"

  - id: B_features
    needs: [A_fetch]
    infer: |
      Extract key features from this entity:
      $A_fetch

      Return as JSON array.

  - id: C_translate
    needs: [A_fetch]
    invoke: novanet_generate
    params:
      entity_key: "qr-code"
      locale: "fr-FR"
      forms: ["text", "title"]

  - id: D_combine
    needs: [B_features, C_translate]
    infer: |
      Create a bilingual feature comparison:

      English source: $A_fetch
      Features extracted: $B_features
      French translation: $C_translate

      Format as side-by-side table.
```

**DAG:**
```
        ┌──→ B_features ──┐
A_fetch ┤                 ├──→ D_combine
        └──→ C_translate ─┘
```

---

#### UC11: Multi-Branch with Agent

```yaml
# examples/uc11-multi-branch.yaml
workflow: multi-branch-agent
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: A_search
    invoke: novanet_search
    params:
      query: "trait:defined realm:org"
      limit: 10

  - id: B_filter
    needs: [A_search]
    exec: |
      echo '$A_search' | jq '.results | map(select(.layer == "semantic"))'

  - id: C_describe_each
    needs: [B_filter]
    for_each:
      items: $B_filter
      as: entity
      concurrency: 3
    invoke: novanet_describe
    params:
      entity_key: $entity.key
      locale: "fr-FR"

  - id: D_get_seo
    needs: [A_search]
    invoke: novanet_atoms
    params:
      type: "seo"
      locale: "fr-FR"

  - id: E_synthesize
    needs: [C_describe_each, D_get_seo]
    agent:
      prompt: |
        Create SEO-optimized descriptions for each entity.

        Entities: $C_describe_each
        SEO keywords: $D_get_seo

        For each entity:
        1. Write meta title (60 chars max)
        2. Write meta description (160 chars max)
        3. Suggest 3 related keywords
      max_turns: 3
```

**DAG:**
```
A_search ──→ B_filter ──→ C_describe_each ──┐
    │                                        ├──→ E_synthesize
    └──→ D_get_seo ─────────────────────────┘
```

---

### Level 4: Content Generation Pipeline (UC12-UC15)

#### UC12: Full Page Generation

```yaml
# examples/uc12-full-page.yaml
workflow: generate-landing-page
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_entity
    invoke: novanet_describe
    params:
      entity_key: "qr-code-dynamic"
      locale: "fr-FR"

  - id: get_seo
    invoke: novanet_atoms
    params:
      type: "seo"
      entity_key: "qr-code-dynamic"
      locale: "fr-FR"

  - id: get_related
    invoke: novanet_traverse
    params:
      start: "entity:qr-code-dynamic"
      arc: "RELATES_TO"
      depth: 2

  - id: generate_hero
    needs: [get_entity, get_seo]
    agent:
      prompt: |
        Generate a hero section for a landing page.

        Entity: $get_entity
        SEO keywords: $get_seo

        Requirements:
        - H1 using denomination_forms.title
        - Compelling subtitle (1 sentence)
        - Primary CTA button text
        - Include primary keyword in first paragraph
      max_turns: 2

  - id: generate_benefits
    needs: [get_entity, get_related]
    agent:
      prompt: |
        Generate a benefits section (5 benefits).

        Entity: $get_entity
        Related entities for inspiration: $get_related

        Each benefit:
        - Icon suggestion (emoji)
        - Title (3-5 words)
        - Description (2 sentences)
      max_turns: 2

  - id: generate_how_it_works
    needs: [get_entity]
    agent:
      prompt: |
        Generate a "How it works" section.

        Entity: $get_entity

        3 steps:
        - Step number
        - Title
        - Description
        - (Optional) illustration suggestion
      max_turns: 2

  - id: generate_faq
    needs: [get_seo]
    agent:
      prompt: |
        Generate FAQ section from SEO questions.

        SEO data (includes questions): $get_seo

        Answer each question in 2-3 sentences.
        Format as schema.org FAQPage compatible.
      max_turns: 3

  - id: assemble_page
    needs: [generate_hero, generate_benefits, generate_how_it_works, generate_faq]
    infer: |
      Assemble the complete landing page in markdown:

      Hero: $generate_hero
      Benefits: $generate_benefits
      How it works: $generate_how_it_works
      FAQ: $generate_faq

      Add:
      - Proper heading hierarchy (H1, H2, H3)
      - Section separators
      - Final CTA before footer
```

---

#### UC13: Multi-Locale Content Sync

```yaml
# examples/uc13-multi-locale-sync.yaml
workflow: sync-all-locales
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_source
    invoke: novanet_describe
    params:
      entity_key: "qr-code"
      locale: "en-US"

  - id: get_enabled_locales
    invoke: novanet_query
    params:
      cypher: |
        MATCH (l:Locale {enabled: true})
        WHERE l.key <> 'en-US'
        RETURN l.key as locale, l.display_name as name

  - id: adapt_each_locale
    needs: [get_source, get_enabled_locales]
    for_each:
      items: $get_enabled_locales.results
      as: target_locale
      concurrency: 2
    agent:
      prompt: |
        Adapt this content for locale: $target_locale.name ($target_locale.locale)

        Source content (en-US):
        $get_source

        Adaptation rules:
        - Translate naturally (not word-for-word)
        - Keep brand terms untranslated: "QR", "NovaNet"
        - Adapt cultural references
        - Use local measurement units
        - Match local writing conventions
      tools:
        - novanet_atoms  # Get locale-specific terminology
      max_turns: 3

  - id: validate_all
    needs: [adapt_each_locale]
    agent:
      prompt: |
        Quality check all adaptations:
        $adapt_each_locale

        Check:
        - No missing translations (empty fields)
        - Brand consistency across locales
        - No machine translation artifacts
        - Appropriate length (not too short/long vs source)

        Return: { valid: bool, issues: [] }
      max_turns: 2
```

---

#### UC14: SEO Gap Analysis

```yaml
# examples/uc14-seo-gap.yaml
workflow: seo-gap-analysis
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: our_keywords
    invoke: novanet_query
    params:
      cypher: |
        MATCH (e:Entity {key: 'qr-code'})<-[:TARGETS]-(kw:SEOKeyword)
        RETURN kw.keyword, kw.volume, kw.difficulty, kw.position
        ORDER BY kw.volume DESC

  - id: all_keywords
    invoke: novanet_query
    params:
      cypher: |
        MATCH (kw:SEOKeyword)
        WHERE NOT (kw)-[:TARGETS]->(:Entity {key: 'qr-code'})
        AND kw.volume > 100
        RETURN kw.keyword, kw.volume, kw.difficulty
        ORDER BY kw.volume DESC
        LIMIT 100

  - id: analyze_gaps
    needs: [our_keywords, all_keywords]
    agent:
      prompt: |
        Analyze SEO keyword gaps:

        Keywords we target: $our_keywords
        Keywords we're missing: $all_keywords

        Identify:
        1. Quick wins: Low difficulty, high volume, relevant
        2. Strategic targets: High volume, worth the effort
        3. Long-tail opportunities: Low competition niches

        For each category, list top 5 keywords with rationale.
      max_turns: 3

  - id: generate_content_plan
    needs: [analyze_gaps]
    agent:
      prompt: |
        Create content plan to target gap keywords:
        $analyze_gaps

        For each keyword group:
        - Suggest page type (landing, blog, feature, comparison)
        - Outline content structure
        - Estimate word count
        - Suggest internal links to existing content
      max_turns: 3
```

---

#### UC15: GEO Question Answering Pipeline

```yaml
# examples/uc15-geo-qa.yaml
workflow: geo-qa-pipeline
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_unanswered
    invoke: novanet_query
    params:
      cypher: |
        MATCH (q:GEOQuery)-[:ABOUT]->(e:Entity)
        WHERE q.answered = false OR q.answered IS NULL
        RETURN q.key, q.question, q.source, q.locale, e.key as entity_key
        LIMIT 10

  - id: answer_each
    needs: [get_unanswered]
    for_each:
      items: $get_unanswered.results
      as: question
      concurrency: 2
    tasks:
      - id: get_context
        invoke: novanet_assemble
        params:
          entity_key: $question.entity_key
          locale: $question.locale
          include: ["atoms", "facts"]

      - id: generate_answer
        needs: [get_context]
        agent:
          prompt: |
            Answer this GEO/PAA question:

            Question: $question.question
            Source: $question.source

            Context from knowledge graph:
            $get_context

            Requirements:
            - Be concise (2-3 sentences for featured snippet)
            - Include entity name naturally
            - Cite facts from context
            - Aim for position 0 (featured snippet)
          max_turns: 2

  - id: format_output
    needs: [answer_each]
    infer: |
      Format all Q&A pairs for import:
      $answer_each

      Output as JSON array:
      [{ question_key, answer, confidence }]
```

---

### Level 5: Knowledge Graph Maintenance (UC16-UC19)

#### UC16: Relationship Discovery

```yaml
# examples/uc16-relationship-discovery.yaml
workflow: discover-relationships
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: find_orphans
    invoke: novanet_query
    params:
      cypher: |
        MATCH (e:Entity)
        WHERE NOT (e)-[:RELATES_TO|PART_OF|DEPENDS_ON]->()
        AND NOT (e)<-[:RELATES_TO|PART_OF|DEPENDS_ON]-()
        AND e.realm = 'org'
        RETURN e.key, e.display_name, e.description
        LIMIT 20

  - id: analyze_each
    needs: [find_orphans]
    for_each:
      items: $find_orphans.results
      as: orphan
      concurrency: 2
    agent:
      prompt: |
        Analyze this orphan entity and suggest relationships:

        Entity: $orphan

        Search the graph for potential connections:
        1. What entities might this RELATE_TO? (similar domain)
        2. Is it PART_OF something larger? (hierarchy)
        3. Does it DEPEND_ON other entities? (prerequisites)
        4. What entities might use this? (reverse deps)

        Return: [{ arc_type, target_key, confidence, reason }]
      tools:
        - novanet_search
        - novanet_traverse
      max_turns: 4

  - id: generate_cypher
    needs: [analyze_each]
    infer: |
      Generate Cypher CREATE statements for proposed relationships:
      $analyze_each

      Format:
      // Relationship: orphan -[ARC]-> target (reason)
      MATCH (a:Entity {key: 'orphan_key'}), (b:Entity {key: 'target_key'})
      CREATE (a)-[:ARC_TYPE {confidence: 0.8, source: 'ai_discovery'}]->(b);
```

---

#### UC17: Content Freshness Audit

```yaml
# examples/uc17-freshness-audit.yaml
workflow: freshness-audit
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: find_stale
    invoke: novanet_query
    params:
      cypher: |
        MATCH (n:EntityNative)
        WHERE n.updated_at < datetime() - duration('P30D')
        RETURN n.entity_key, n.locale_key, n.updated_at, n.display_name
        ORDER BY n.updated_at ASC
        LIMIT 30

  - id: check_each
    needs: [find_stale]
    for_each:
      items: $find_stale.results
      as: stale
      concurrency: 2
    tasks:
      - id: get_current
        invoke: novanet_describe
        params:
          entity_key: $stale.entity_key
          locale: $stale.locale_key

      - id: verify_accuracy
        needs: [get_current]
        agent:
          prompt: |
            Verify if this content is still accurate:

            Content: $get_current
            Last updated: $stale.updated_at

            Check:
            1. Are facts still correct?
            2. Are there new developments in this domain?
            3. Is terminology still current?
            4. Are there broken references?

            Return: {
              needs_update: bool,
              priority: 1-5,
              issues: [],
              suggested_changes: []
            }
          max_turns: 2

  - id: prioritize
    needs: [check_each]
    infer: |
      Create prioritized update list:
      $check_each

      Group by priority:
      - P1 (Critical): Factually incorrect
      - P2 (High): Significantly outdated
      - P3 (Medium): Minor updates needed
      - P4 (Low): Nice to have
      - P5 (Skip): Still accurate
```

---

#### UC18: Category Page Generation

```yaml
# examples/uc18-category-page.yaml
workflow: generate-category-page
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_category
    invoke: novanet_describe
    params:
      entity_key: "qr-code-types"
      locale: "fr-FR"

  - id: get_children
    invoke: novanet_traverse
    params:
      start: "entity:qr-code-types"
      arc: "HAS_CHILD"

  - id: describe_children
    needs: [get_children]
    for_each:
      items: $get_children.results
      as: child
      concurrency: 5
    invoke: novanet_describe
    params:
      entity_key: $child.key
      locale: "fr-FR"

  - id: get_features
    invoke: novanet_query
    params:
      cypher: |
        MATCH (parent:Entity {key: 'qr-code-types'})-[:HAS_CHILD]->(child)
        OPTIONAL MATCH (child)-[:HAS_FEATURE]->(f:Feature)
        RETURN child.key, child.display_name, collect(f.name) as features

  - id: generate_comparison
    needs: [describe_children, get_features]
    agent:
      prompt: |
        Create a comparison table for QR code types:

        Children: $describe_children
        Features: $get_features

        Table format:
        | Type | Best For | Key Feature | Price Range |

        Also add:
        - Winner badges (Best for X)
        - Recommendation for each use case
      max_turns: 2

  - id: assemble_category_page
    needs: [get_category, describe_children, generate_comparison]
    agent:
      prompt: |
        Generate complete category page:

        Category: $get_category
        Children: $describe_children
        Comparison: $generate_comparison

        Structure:
        1. H1: Category title
        2. Intro: What are QR code types?
        3. Comparison table
        4. Individual sections for each type
        5. Decision helper: "Which is right for you?"
        6. CTA
      max_turns: 3
```

---

#### UC19: Internal Linking

```yaml
# examples/uc19-internal-linking.yaml
workflow: internal-linking
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_page_content
    invoke: novanet_describe
    params:
      entity_key: "qr-code-dynamic"
      locale: "fr-FR"

  - id: get_linkable_entities
    invoke: novanet_query
    params:
      cypher: |
        MATCH (e:Entity)-[:HAS_NATIVE]->(n:EntityNative)-[:FOR_LOCALE]->(:Locale {key: 'fr-FR'})
        WHERE e.key <> 'qr-code-dynamic'
        AND (e)<-[:REPRESENTS]-(:Page)
        RETURN e.key, n.display_name, n.description

  - id: find_opportunities
    needs: [get_page_content, get_linkable_entities]
    agent:
      prompt: |
        Find internal linking opportunities:

        Current page content: $get_page_content
        Available entities to link: $get_linkable_entities

        For each potential link:
        1. Identify exact anchor text in current content
        2. Match to target entity
        3. Explain SEO value

        Return: [{
          anchor_text: "exact text from content",
          target_entity: "entity-key",
          target_url: "/entity-key",
          seo_value: "explanation"
        }]

        Rules:
        - Max 5-7 internal links per page
        - No duplicate targets
        - Natural placement only
      max_turns: 3

  - id: apply_links
    needs: [find_opportunities, get_page_content]
    infer: |
      Rewrite content with internal links added:

      Original: $get_page_content
      Links to add: $find_opportunities

      Format links as markdown: [anchor text](/target-url)
```

---

### Level 6: Quality Assurance (UC20-UC21)

#### UC20: Cross-Locale Consistency Check

```yaml
# examples/uc20-consistency-check.yaml
workflow: consistency-check
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_all_locales
    invoke: novanet_query
    params:
      cypher: |
        MATCH (e:Entity {key: 'qr-code'})-[:HAS_NATIVE]->(n:EntityNative)-[:FOR_LOCALE]->(l:Locale)
        RETURN l.key as locale,
               n.display_name,
               n.description,
               size(n.key_features) as feature_count,
               n.updated_at
        ORDER BY l.key

  - id: check_consistency
    needs: [get_all_locales]
    agent:
      prompt: |
        Check content consistency across locales:
        $get_all_locales

        Verify:
        1. Same number of key_features in each locale
        2. No critical information missing
        3. Brand terms consistent (not translated)
        4. Similar content length (±30%)
        5. All have recent updates

        Report issues with severity:
        - CRITICAL: Missing content, wrong facts
        - WARNING: Inconsistent structure
        - INFO: Minor differences

        Return: {
          consistent: bool,
          issues: [{ locale, severity, description }]
        }
      max_turns: 3

  - id: generate_fixes
    needs: [check_consistency]
    infer: |
      Generate fix recommendations:
      $check_consistency

      For each issue:
      - What's wrong
      - Which locale(s)
      - Suggested fix
      - Cypher to update (if applicable)
```

---

#### UC21: Terminology Audit

```yaml
# examples/uc21-terminology-audit.yaml
workflow: terminology-audit
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_terminology
    invoke: novanet_atoms
    params:
      type: "terms"
      locale: "fr-FR"
      domain: "qr-code"

  - id: get_taboos
    invoke: novanet_atoms
    params:
      type: "taboos"
      locale: "fr-FR"

  - id: get_all_content
    invoke: novanet_query
    params:
      cypher: |
        MATCH (n:EntityNative)-[:FOR_LOCALE]->(:Locale {key: 'fr-FR'})
        RETURN n.entity_key, n.display_name, n.description

  - id: audit_usage
    needs: [get_terminology, get_taboos, get_all_content]
    agent:
      prompt: |
        Audit terminology usage in all content:

        Approved terms: $get_terminology
        Forbidden terms (taboos): $get_taboos
        Content to check: $get_all_content

        Find:
        1. Incorrect term variants used
        2. Inconsistent capitalization
        3. Forbidden terms (taboos) used
        4. Missing approved terms where expected

        Return: [{
          entity_key,
          issue_type: "wrong_term" | "taboo" | "inconsistent" | "missing",
          found: "what was found",
          expected: "what should be used",
          context: "surrounding text"
        }]
      max_turns: 4

  - id: generate_corrections
    needs: [audit_usage]
    infer: |
      Generate correction commands:
      $audit_usage

      For each issue, output sed-style replacement:
      s/wrong term/correct term/g
```

---

### Level 7: Analytics & Insights (UC22-UC23)

#### UC22: Content Performance Correlation

```yaml
# examples/uc22-performance-analysis.yaml
workflow: performance-analysis
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_top_pages
    invoke: novanet_query
    params:
      cypher: |
        MATCH (p:Page)-[:HAS_METRICS]->(m:PageMetrics)
        WHERE m.period = 'last_30_days'
        WITH p, m
        ORDER BY m.pageviews DESC
        LIMIT 20
        RETURN p.key, p.entity_key, m.pageviews, m.avg_time_on_page, m.bounce_rate

  - id: get_bottom_pages
    invoke: novanet_query
    params:
      cypher: |
        MATCH (p:Page)-[:HAS_METRICS]->(m:PageMetrics)
        WHERE m.period = 'last_30_days'
        AND m.pageviews > 10
        WITH p, m
        ORDER BY m.bounce_rate DESC
        LIMIT 20
        RETURN p.key, p.entity_key, m.pageviews, m.avg_time_on_page, m.bounce_rate

  - id: enrich_with_graph
    needs: [get_top_pages, get_bottom_pages]
    for_each:
      items: [...$get_top_pages.results, ...$get_bottom_pages.results]
      as: page
      concurrency: 5
    invoke: novanet_describe
    params:
      entity_key: $page.entity_key
      locale: "en-US"

  - id: analyze_patterns
    needs: [get_top_pages, get_bottom_pages, enrich_with_graph]
    agent:
      prompt: |
        Analyze content patterns correlating with performance:

        Top performers: $get_top_pages
        Bottom performers: $get_bottom_pages
        Content details: $enrich_with_graph

        Investigate:
        1. What entity types perform best?
        2. Content length vs engagement
        3. Number of related entities vs bounce rate
        4. Freshness (updated_at) vs traffic

        Hypothesize causation and suggest A/B tests.
      tools:
        - novanet_traverse  # Check relationships
      max_turns: 5
```

---

#### UC23: Graph Health Check

```yaml
# examples/uc23-graph-health.yaml
workflow: graph-health-check
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: check_orphans
    invoke: novanet_query
    params:
      cypher: |
        MATCH (n)
        WHERE NOT (n)--()
        RETURN labels(n)[0] as type, count(n) as orphan_count

  - id: check_missing_natives
    invoke: novanet_query
    params:
      cypher: |
        MATCH (e:Entity {realm: 'org'})
        WHERE NOT (e)-[:HAS_NATIVE]->()
        RETURN e.key, e.display_name

  - id: check_locale_coverage
    invoke: novanet_query
    params:
      cypher: |
        MATCH (l:Locale {enabled: true})
        WITH collect(l.key) as required
        MATCH (e:Entity {realm: 'org'})-[:HAS_NATIVE]->(n:EntityNative)-[:FOR_LOCALE]->(el:Locale)
        WITH e, required, collect(el.key) as has
        WHERE size(required) > size(has)
        RETURN e.key, has, required, size(required) - size(has) as missing_count

  - id: check_broken_refs
    invoke: novanet_query
    params:
      cypher: |
        MATCH (a)-[r]->(b)
        WHERE NOT (b:Entity OR b:Page OR b:Block OR b:Locale OR b:EntityNative)
        RETURN type(r) as arc, labels(a)[0] as from, labels(b)[0] as to, count(*) as count

  - id: check_duplicates
    invoke: novanet_query
    params:
      cypher: |
        MATCH (e1:Entity), (e2:Entity)
        WHERE e1.display_name = e2.display_name
        AND id(e1) < id(e2)
        RETURN e1.key as key1, e2.key as key2, e1.display_name as name

  - id: generate_report
    needs: [check_orphans, check_missing_natives, check_locale_coverage, check_broken_refs, check_duplicates]
    agent:
      prompt: |
        Generate graph health report:

        Orphan nodes: $check_orphans
        Missing natives: $check_missing_natives
        Locale coverage gaps: $check_locale_coverage
        Broken references: $check_broken_refs
        Potential duplicates: $check_duplicates

        For each category:
        - Severity (CRITICAL/WARNING/INFO)
        - Count affected
        - Fix recommendation
        - Cypher to remediate

        Overall health score: X/100
      max_turns: 3
```

---

### Level 8: Advanced Agent Patterns (UC24-UC26)

#### UC24: Research Agent (Multi-Turn Discovery)

```yaml
# examples/uc24-research-agent.yaml
workflow: research-agent
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: research
    agent:
      prompt: |
        You are a research agent investigating QR code market opportunities.

        Your mission:
        1. Explore what entities exist in the knowledge graph
        2. Identify gaps in coverage (missing use cases, industries)
        3. Find entities with poor locale coverage
        4. Suggest new entities to add
        5. Prioritize by business value

        Approach:
        - Start broad (search all entities)
        - Then deep dive into interesting areas
        - Keep notes of findings
        - Build a coherent recommendation

        Think step by step. Use tools iteratively.
      tools:
        - novanet_search
        - novanet_describe
        - novanet_traverse
        - novanet_query
      max_turns: 10

  - id: format_report
    needs: [research]
    infer: |
      Format research findings as executive report:
      $research

      Structure:
      1. Executive Summary (3 bullet points)
      2. Current State Analysis
      3. Gap Identification
      4. Recommendations (prioritized by ROI)
      5. Next Steps
```

---

#### UC25: Multi-Agent Pipeline

```yaml
# examples/uc25-multi-agent.yaml
workflow: multi-agent-content
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: researcher
    agent:
      prompt: |
        You are a research agent. Gather all information about "qr-code-restaurant".

        Collect:
        - Entity description
        - Related entities
        - SEO keywords
        - Competitor positioning

        Output a comprehensive research brief for the writer.
      tools:
        - novanet_describe
        - novanet_traverse
        - novanet_atoms
        - novanet_search
      max_turns: 5

  - id: writer
    needs: [researcher]
    agent:
      prompt: |
        You are a content writer. Based on the research brief, write a landing page.

        Research: $researcher

        Write:
        - Hero section (H1 + intro paragraph)
        - 5 key benefits
        - How it works (3 steps)
        - Social proof placeholder
        - CTA

        Style: Professional but friendly, French language.
      max_turns: 4

  - id: editor
    needs: [writer]
    agent:
      prompt: |
        You are a senior editor. Review and improve the content.

        Draft: $writer

        Check:
        - Grammar and spelling
        - Brand voice consistency
        - Flow and readability
        - CTA effectiveness

        Return edited version with [CHANGE: reason] annotations.
      max_turns: 3

  - id: seo_specialist
    needs: [editor, researcher]
    agent:
      prompt: |
        You are an SEO specialist. Optimize the content.

        Content: $editor
        Research (includes keywords): $researcher

        Optimize:
        - H1 contains primary keyword
        - Meta title (60 chars)
        - Meta description (160 chars)
        - Keyword density check
        - Internal linking suggestions

        Return final optimized version + metadata.
      max_turns: 2
```

---

#### UC26: Self-Correcting Agent

```yaml
# examples/uc26-self-correcting.yaml
workflow: self-correcting
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: draft_v1
    agent:
      prompt: |
        Generate a product description for "qr-code-dynamic".
        Use the knowledge graph for accurate information.
      tools:
        - novanet_describe
        - novanet_atoms
      max_turns: 3

  - id: critique
    needs: [draft_v1]
    agent:
      prompt: |
        Critically evaluate this content:
        $draft_v1

        Check against source data:
        1. Factual accuracy (verify each claim)
        2. Completeness (any missing key points?)
        3. Tone (matches brand voice?)
        4. SEO (keywords present?)

        Be harsh. List every improvement needed.
      tools:
        - novanet_describe  # Verify facts
      max_turns: 3

  - id: draft_v2
    needs: [draft_v1, critique]
    agent:
      prompt: |
        Improve the content based on critique:

        Original: $draft_v1
        Critique: $critique

        Address EVERY point in the critique.
        Mark each fix with [FIXED: critique point].
      tools:
        - novanet_describe
        - novanet_atoms
      max_turns: 3

  - id: final_check
    needs: [draft_v2, critique]
    infer: |
      Verify all critique points addressed:

      Critique points: $critique
      Final version: $draft_v2

      Return: {
        all_addressed: bool,
        remaining_issues: [],
        quality_score: 1-10
      }
```

---

### Level 9: Batch Operations (UC27-UC28)

#### UC27: Mass Page Generation

```yaml
# examples/uc27-mass-generation.yaml
workflow: generate-all-pages
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: find_entities_without_pages
    invoke: novanet_query
    params:
      cypher: |
        MATCH (e:Entity {realm: 'org', trait: 'defined'})
        WHERE NOT (e)<-[:REPRESENTS]-(:Page)
        RETURN e.key, e.display_name

  - id: generate_each
    needs: [find_entities_without_pages]
    for_each:
      items: $find_entities_without_pages.results
      as: entity
      concurrency: 2
    tasks:
      - id: get_context
        invoke: novanet_assemble
        params:
          entity_key: $entity.key
          locale: "en-US"
          include: ["atoms", "seo", "related"]

      - id: generate_page
        needs: [get_context]
        agent:
          prompt: |
            Generate landing page for: $entity.display_name
            Context: $get_context

            Include: hero, benefits, how-it-works, faq, cta
          max_turns: 3

  - id: summary
    needs: [generate_each]
    infer: |
      Summarize batch generation:
      $generate_each

      Report:
      - Total generated
      - Any failures
      - Total tokens used
      - Estimated cost
```

---

#### UC28: Locale Expansion

```yaml
# examples/uc28-locale-expansion.yaml
workflow: expand-to-locale
mcp:
  novanet:
    command: ["cargo", "run", "--manifest-path", "../../novanet-dev/tools/novanet-mcp/Cargo.toml"]
tasks:
  - id: get_source_entities
    invoke: novanet_query
    params:
      cypher: |
        MATCH (e:Entity {realm: 'org'})-[:HAS_NATIVE]->(n:EntityNative)-[:FOR_LOCALE]->(:Locale {key: 'en-US'})
        WHERE NOT (e)-[:HAS_NATIVE]->(:EntityNative)-[:FOR_LOCALE]->(:Locale {key: 'ja-JP'})
        RETURN e.key, n.display_name, n.description
        LIMIT 50

  - id: get_source_content
    needs: [get_source_entities]
    for_each:
      items: $get_source_entities.results
      as: entity
      concurrency: 5
    invoke: novanet_describe
    params:
      entity_key: $entity.key
      locale: "en-US"

  - id: get_target_atoms
    invoke: novanet_atoms
    params:
      type: "terms"
      locale: "ja-JP"

  - id: adapt_each
    needs: [get_source_content, get_target_atoms]
    for_each:
      items: $get_source_content
      as: content
      concurrency: 2
    agent:
      prompt: |
        Adapt this content to Japanese (ja-JP):

        Source: $content
        Japanese terminology guide: $get_target_atoms

        Cultural adaptation:
        - Use formal/polite register (です/ます)
        - Metric units
        - Japanese business conventions
        - Local examples where possible

        Keep brand terms: "QR", company names
      max_turns: 3

  - id: validate_batch
    needs: [adapt_each]
    agent:
      prompt: |
        Quality check batch Japanese adaptations:
        $adapt_each

        Check:
        - Consistent register (all formal)
        - No machine translation artifacts
        - Brand terms not translated
        - Natural Japanese phrasing

        Flag any items needing human review.
      max_turns: 2
```

---

## Part 3: Neo4j Query Library

### Discovery Queries

```cypher
-- Q1: Schema visualization
CALL db.schema.visualization()

-- Q2: Node class distribution
MATCH (n)
RETURN labels(n)[0] as class, count(n) as count
ORDER BY count DESC

-- Q3: Arc type distribution
MATCH ()-[r]->()
RETURN type(r) as arc, count(r) as count
ORDER BY count DESC

-- Q4: Most connected nodes (hubs)
MATCH (n)-[r]-()
WITH n, count(r) as connections
ORDER BY connections DESC
LIMIT 20
RETURN labels(n)[0] as type, n.key, connections

-- Q5: Graph density by realm
MATCH (n)
WHERE n.realm IS NOT NULL
WITH n.realm as realm, count(n) as nodes
MATCH (a)-[r]->(b)
WHERE a.realm = realm
WITH realm, nodes, count(r) as arcs
RETURN realm, nodes, arcs, toFloat(arcs)/nodes as density
```

### Traversal Queries

```cypher
-- Q6: Entity subgraph (depth 3)
MATCH path = (e:Entity {key: 'qr-code'})-[*1..3]-(connected)
RETURN path

-- Q7: Shortest path between entities
MATCH path = shortestPath(
  (a:Entity {key: 'qr-code'})-[*]-(b:Entity {key: 'qr-code-restaurant'})
)
RETURN path, length(path) as hops

-- Q8: All paths between entities (max 4 hops)
MATCH path = (a:Entity {key: 'qr-code'})-[*1..4]-(b:Entity {key: 'qr-code-wifi'})
RETURN path, [r in relationships(path) | type(r)] as arc_types

-- Q9: Entity lineage (ancestors)
MATCH path = (e:Entity {key: 'qr-code-static'})-[:PART_OF*1..5]->(ancestor)
RETURN path

-- Q10: Entity descendants
MATCH path = (e:Entity {key: 'qr-code-types'})-[:HAS_CHILD*1..3]->(descendant)
RETURN path
```

### Analysis Queries

```cypher
-- Q11: Locale coverage matrix
MATCH (e:Entity {realm: 'org'})
OPTIONAL MATCH (e)-[:HAS_NATIVE]->(n:EntityNative)-[:FOR_LOCALE]->(l:Locale)
WITH e, collect(l.key) as locales
RETURN e.key,
       size(locales) as locale_count,
       locales
ORDER BY locale_count ASC

-- Q12: Content freshness distribution
MATCH (n:EntityNative)
WHERE n.updated_at IS NOT NULL
WITH n,
     CASE
       WHEN n.updated_at > datetime() - duration('P7D') THEN 'week'
       WHEN n.updated_at > datetime() - duration('P30D') THEN 'month'
       WHEN n.updated_at > datetime() - duration('P90D') THEN 'quarter'
       ELSE 'stale'
     END as freshness
RETURN freshness, count(n) as count

-- Q13: Arc balance (in/out ratio)
MATCH (e:Entity)
OPTIONAL MATCH (e)-[out]->()
OPTIONAL MATCH (e)<-[in]-()
WITH e, count(DISTINCT out) as out_count, count(DISTINCT in) as in_count
WHERE out_count + in_count > 0
RETURN e.key,
       out_count,
       in_count,
       toFloat(out_count) / (out_count + in_count) as out_ratio
ORDER BY out_ratio DESC

-- Q14: Entities by trait
MATCH (e:Entity)
RETURN e.trait, e.layer, count(e) as count
ORDER BY count DESC

-- Q15: Layer connectivity
MATCH (a)-[r]->(b)
WHERE a.layer IS NOT NULL AND b.layer IS NOT NULL
RETURN a.layer as from_layer, b.layer as to_layer, type(r) as arc, count(r) as count
ORDER BY count DESC
```

### Maintenance Queries

```cypher
-- Q16: Orphan nodes (no connections)
MATCH (n)
WHERE NOT (n)--()
RETURN labels(n)[0] as type, n.key, n.display_name

-- Q17: Entities without native content
MATCH (e:Entity {realm: 'org'})
WHERE NOT (e)-[:HAS_NATIVE]->()
RETURN e.key, e.display_name, e.created_at

-- Q18: Duplicate display names
MATCH (e1:Entity), (e2:Entity)
WHERE e1.display_name = e2.display_name
AND id(e1) < id(e2)
RETURN e1.key, e2.key, e1.display_name

-- Q19: Missing inverse arcs (per ADR-026)
MATCH (a)-[r:HAS_NATIVE]->(b)
WHERE NOT (b)-[:NATIVE_OF]->(a)
RETURN a.key, b.key as missing_inverse_from

-- Q20: Nodes with null required properties
MATCH (e:Entity)
WHERE e.display_name IS NULL OR e.description IS NULL
RETURN e.key,
       e.display_name IS NULL as missing_name,
       e.description IS NULL as missing_desc
```

### SEO/GEO Queries

```cypher
-- Q21: Keywords by entity
MATCH (e:Entity {key: 'qr-code'})<-[:TARGETS]-(kw:SEOKeyword)
RETURN kw.keyword, kw.volume, kw.difficulty
ORDER BY kw.volume DESC

-- Q22: High opportunity keywords (high volume, low difficulty)
MATCH (kw:SEOKeyword)
WHERE kw.volume > 1000 AND kw.difficulty < 30
RETURN kw.keyword, kw.volume, kw.difficulty,
       toFloat(kw.volume) / (kw.difficulty + 1) as opportunity_score
ORDER BY opportunity_score DESC
LIMIT 20

-- Q23: Unanswered GEO questions
MATCH (q:GEOQuery)-[:ABOUT]->(e:Entity)
WHERE q.answered = false OR q.answered IS NULL
RETURN q.question, e.key, q.source, q.locale
ORDER BY q.created_at DESC

-- Q24: Keyword gaps (competitors rank, we don't)
MATCH (c:Competitor)-[:RANKS_FOR]->(kw:SEOKeyword)
WHERE NOT (kw)-[:TARGETS]->(:Entity)
RETURN kw.keyword, kw.volume, collect(c.name) as competitors
ORDER BY kw.volume DESC
LIMIT 30
```

---

## Part 4: Socratic Questions

Answer these by exploring the system:

### Architecture Questions

```
Q1: Why separate NovaNet (brain) and Nika (body)?
    - What would break if they were one system?
    - What does MCP enable that direct integration wouldn't?

Q2: Is the MCP protocol the right choice?
    - Compare: REST, GraphQL, gRPC
    - What advantages for agentic workflows?
    - What are the latency implications?

Q3: Why exactly 5 verbs (infer, exec, fetch, invoke, agent)?
    - What use cases are NOT covered?
    - Could some be merged?
    - What's missing for MVP 8?

Q4: Is the DAG execution model sufficient?
    - Can it handle cycles? Should it?
    - What about conditional branching?
    - How does for_each interact with DAG?
```

### Data Model Questions

```
Q5: 61 node classes, 182 arc classes — is this right?
    - Are there redundancies?
    - What's the cognitive load for developers?
    - How does this compare to Schema.org?

Q6: The *Native pattern (EntityNative, PageNative) — optimal?
    - Compare with: separate tables, JSON blobs, property arrays
    - What's the query performance impact?
    - How does it scale to 50 locales?

Q7: denomination_forms (ADR-033) — sufficient?
    - text, title, abbrev, url, mixed, base — missing any?
    - How do LLMs actually use these?
    - What about pluralization?

Q8: Trait as "data origin" (ADR-024) — clear enough?
    - defined vs authored — when is it ambiguous?
    - What about user-generated content?
    - What about AI-assisted human content?
```

### Workflow Questions

```
Q9: for_each with concurrency — what could go wrong?
    - Rate limiting (LLM, MCP)?
    - Partial failures — how handled?
    - Result ordering — guaranteed?

Q10: Agent loop — safeguards against infinite loops?
     - max_turns is the only check?
     - What about token budget?
     - What about time budget?

Q11: Context assembly ($var references) — type safe?
     - What happens with missing variables?
     - What about deeply nested access?
     - JSON vs string interpolation?

Q12: Error propagation in DAG — clear enough?
     - If B fails, does C run (when C needs A only)?
     - Can we retry individual tasks?
     - What about rollback?
```

### Product Questions (QR-Code AI)

```
Q13: 279 entities for QR-Code AI — comprehensive?
     - What domains are missing?
     - How to systematically discover gaps?
     - What's the entity creation workflow?

Q14: Content quality assurance — automated?
     - What metrics to track?
     - Human-in-the-loop where?
     - Feedback loop from users?

Q15: Multi-tenant support — needed?
     - Can multiple orgs share the graph?
     - What's the isolation model?
     - How does this affect MCP?
```

### Evolution Questions

```
Q16: MVP 8 "nested agents" — necessary?
     - What can't be done today?
     - What's the recursion risk?
     - How to debug nested agent chains?

Q17: "Lazy context loading" — worth it?
     - Measure current context loading cost
     - What's the implementation complexity?
     - Is this premature optimization?

Q18: After MVP 8, what's next?
     - Brainstorm 5 killer features
     - Prioritize by impact/effort
     - What would change the architecture?
```

---

## Part 5: Expected TUI Snapshots

For each scenario category, show a realistic TUI state.

### Simple Workflow (UC1-UC5)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NIKA v0.4.0 | UC3: Invoke Describe                          [q]uit [?]help │
├──────────────────────┬──────────────────────────────┬───────────────────────┤
│ Workflow             │ Task: describe               │ Events                │
│ ──────────           │ ────────────                 │ ────────              │
│                      │                              │                       │
│ ✓ describe           │ Status: ✓ Completed          │ 10:42:01.001          │
│                      │ Duration: 1.23s              │ WorkflowStarted       │
│                      │                              │                       │
│                      │ Invoke: novanet_describe     │ 10:42:01.005          │
│                      │                              │ TaskStarted           │
│                      │ Input:                       │ id=describe           │
│                      │   entity_key: "qr-code"      │                       │
│                      │   locale: "fr-FR"            │ 10:42:01.010          │
│                      │                              │ McpToolCalled         │
│                      │ Output:                      │ tool=novanet_describe │
│                      │   key: "qr-code"             │                       │
│                      │   display_name: "Code QR"    │ 10:42:02.234          │
│                      │   description: "Un code..."  │ McpToolResponded      │
│                      │   denomination_forms:        │ duration=1224ms       │
│                      │     text: "code QR"          │                       │
│                      │     title: "Code QR"         │ 10:42:02.240          │
│                      │     abbrev: "QR"             │ TaskCompleted         │
│                      │                              │                       │
│                      │ Tokens: 0 in / 0 out         │ 10:42:02.242          │
│                      │ (invoke = no LLM)            │ WorkflowCompleted     │
│                      │                              │                       │
├──────────────────────┴──────────────────────────────┴───────────────────────┤
│ ✓ Completed | Tasks: 1/1 | MCP: 1 call | Duration: 1.24s                    │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Parallel for_each (UC6-UC7)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NIKA v0.4.0 | UC6: Parallel Locales                         [q]uit [?]help │
├──────────────────────┬──────────────────────────────┬───────────────────────┤
│ Workflow             │ for_each: generate_all       │ Events                │
│ ──────────           │ ────────────────────         │ ────────              │
│                      │                              │                       │
│ ✓ define_locales     │ Progress: 3/5 (60%)          │ 10:42:05.100          │
│ ⟳ generate_all       │ ████████████░░░░░░░░         │ ForEachStarted        │
│   ├─ ✓ [0] fr-FR     │                              │ items=5, conc=3       │
│   ├─ ✓ [1] en-US     │ Concurrency: 2/3 active      │                       │
│   ├─ ✓ [2] de-DE     │                              │ 10:42:05.102          │
│   ├─ ⟳ [3] es-ES     │ Active:                      │ ItemStarted [0]       │
│   └─ ⟳ [4] ja-JP     │ • es-ES (novanet_generate)   │ 10:42:05.103          │
│ ◯ report             │ • ja-JP (novanet_generate)   │ ItemStarted [1]       │
│                      │                              │ 10:42:05.104          │
│                      │ Completed results:           │ ItemStarted [2]       │
│                      │ fr-FR: "code QR"             │                       │
│                      │ en-US: "QR code"             │ 10:42:06.234          │
│                      │ de-DE: "QR-Code"             │ ItemCompleted [0]     │
│                      │                              │ 10:42:06.567          │
│                      │ ETA: ~3 seconds              │ ItemCompleted [1]     │
│                      │                              │ 10:42:06.890          │
│                      │                              │ ItemCompleted [2]     │
│                      │                              │ 10:42:07.001          │
│                      │                              │ ItemStarted [3]       │
│                      │                              │ ItemStarted [4]       │
├──────────────────────┴──────────────────────────────┴───────────────────────┤
│ ⟳ Running | Progress: 60% | Items: 3/5 | Concurrency: 2/3 | Elapsed: 4.5s  │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Agent Loop (UC8-UC9)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NIKA v0.4.0 | UC8: Agent Researcher                         [q]uit [?]help │
├──────────────────────┬──────────────────────────────┬───────────────────────┤
│ Workflow             │ Agent: research              │ Events                │
│ ──────────           │ ─────────────                │ ────────              │
│                      │                              │                       │
│ ⟳ research           │ Turn: 3/5                    │ 10:42:01.000          │
│   ├─ Turn 1 ✓        │ Model: claude-sonnet-4       │ AgentStarted          │
│   │  └─ describe ✓   │                              │                       │
│   ├─ Turn 2 ✓        │ ┌─ Thinking ────────────┐    │ 10:42:01.500          │
│   │  └─ traverse ✓   │ │ I've described the    │    │ Turn 1 started        │
│   └─ Turn 3 ⟳        │ │ entity. Now I should  │    │                       │
│      └─ search ⟳     │ │ find related entities │    │ 10:42:02.100          │
│                      │ │ to complete my        │    │ ToolCall: describe    │
│                      │ │ research...           │    │                       │
│                      │ └──────────────────────┘    │ 10:42:03.200          │
│                      │                              │ ToolResult            │
│                      │ Tool call:                   │ (success, 234ms)      │
│                      │ novanet_search               │                       │
│                      │ {                            │ 10:42:03.500          │
│                      │   query: "RELATES_TO..."     │ Turn 2 started        │
│                      │ }                            │                       │
│                      │                              │ 10:42:04.100          │
│                      │ Tokens:                      │ ToolCall: traverse    │
│                      │   Input:  2,345              │                       │
│                      │   Output: 456                │ 10:42:05.300          │
│                      │   Total:  2,801              │ Turn 3 started        │
│                      │                              │                       │
│                      │ Cost estimate: ~$0.03        │ 10:42:05.500          │
│                      │                              │ ToolCall: search ⟳    │
├──────────────────────┴──────────────────────────────┴───────────────────────┤
│ ⟳ Agent Turn 3/5 | Tools: 3 calls | Tokens: 2,801 | Cost: ~$0.03 | 12.3s   │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Complex DAG (UC10-UC11)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NIKA v0.4.0 | UC10: Diamond DAG                             [q]uit [?]help │
├──────────────────────┬──────────────────────────────┬───────────────────────┤
│ Workflow (DAG)       │ Task: D_combine              │ Events                │
│ ────────────         │ ────────────                 │ ────────              │
│                      │                              │                       │
│ ✓ A_fetch            │ Status: ⟳ Running            │ 10:42:01.000          │
│ ├─→ ✓ B_features     │ Needs: B_features ✓          │ TaskStarted: A        │
│ │                    │        C_translate ✓         │                       │
│ └─→ ✓ C_translate    │                              │ 10:42:02.000          │
│      │               │ Input context:               │ TaskCompleted: A      │
│      └───┬───────────│ • B_features: [...]          │                       │
│          ↓           │ • C_translate: {...}         │ 10:42:02.010          │
│     ⟳ D_combine      │                              │ TaskStarted: B (||)   │
│                      │ LLM Request:                 │ TaskStarted: C (||)   │
│                      │ "Create bilingual..."        │                       │
│ ┌─ DAG Visual ─────┐ │                              │ 10:42:03.500          │
│ │     ┌─→ B ──┐    │ │                              │ TaskCompleted: B      │
│ │  A ─┤       ├─→ D│ │                              │                       │
│ │     └─→ C ──┘    │ │                              │ 10:42:04.200          │
│ └──────────────────┘ │                              │ TaskCompleted: C      │
│                      │ Tokens: 567 in / 234 out     │                       │
│                      │                              │ 10:42:04.210          │
│                      │                              │ TaskStarted: D        │
│                      │                              │ (deps satisfied)      │
├──────────────────────┴──────────────────────────────┴───────────────────────┤
│ ⟳ Running | Tasks: 3/4 | Parallelism: 2 used | Tokens: 801 | 3.2s          │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Batch Operation (UC27)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NIKA v0.4.0 | UC27: Mass Generation                         [q]uit [?]help │
├──────────────────────┬──────────────────────────────┬───────────────────────┤
│ Workflow             │ Batch Progress               │ Events (recent)       │
│ ──────────           │ ──────────────               │ ───────────────       │
│                      │                              │                       │
│ ✓ find_entities      │ for_each: generate_each      │ 10:42:15.000          │
│   └─ 47 found        │ ████████░░░░░░░░░░ 36%       │ ItemCompleted [15]    │
│ ⟳ generate_each      │                              │                       │
│   ├─ ✓ [0-15] done   │ Progress: 17/47              │ 10:42:15.500          │
│   ├─ ⟳ [16] qr-vcard │ Concurrency: 2/2 active      │ ItemStarted [16]      │
│   ├─ ⟳ [17] qr-menu  │                              │                       │
│   └─ ◯ [18-46] queue │ Active items:                │ 10:42:15.600          │
│ ◯ summary            │ • qr-vcard (agent turn 2/3)  │ ItemStarted [17]      │
│                      │ • qr-menu (get_context)      │                       │
│                      │                              │ 10:42:16.000          │
│ Stats:               │ Completed: 17                │ AgentTurn [16] 2/3    │
│ • Succeeded: 16      │ Failed: 1 (qr-test)          │                       │
│ • Failed: 1          │ Queued: 29                   │ 10:42:16.500          │
│ • Rate: 2.1/min      │                              │ McpToolCalled [17]    │
│                      │ Tokens used: 45,678          │                       │
│                      │ Cost estimate: $0.52         │ 10:42:17.000          │
│                      │                              │ McpToolResponded [17] │
│                      │ ETA: ~12 minutes             │                       │
│                      │                              │                       │
├──────────────────────┴──────────────────────────────┴───────────────────────┤
│ ⟳ Batch 36% | Items: 17/47 | Failed: 1 | Tokens: 45.6k | ETA: 12m | 8m 23s │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Part 6: Final Checklist

### Infrastructure
- [ ] TUI compiles: `cargo build --features tui`
- [ ] TUI launches: `cargo run -- tui examples/uc01-hello-world.yaml`
- [ ] No mocks in production code
- [ ] MCP client connects to real NovaNet
- [ ] Neo4j accessible and seeded

### Simple Scenarios (UC1-UC5)
- [ ] UC1: Hello World ✓
- [ ] UC2: Fetch + Infer ✓
- [ ] UC3: Invoke describe ✓
- [ ] UC4: Invoke generate (denomination_forms) ✓
- [ ] UC5: Traverse graph ✓

### Parallel Scenarios (UC6-UC7)
- [ ] UC6: for_each locales ✓
- [ ] UC7: Fan-out fan-in ✓

### Agent Scenarios (UC8-UC11)
- [ ] UC8: Agent with tools ✓
- [ ] UC9: Agent with context ✓
- [ ] UC10: Diamond DAG ✓
- [ ] UC11: Multi-branch agent ✓

### Content Pipeline (UC12-UC15)
- [ ] UC12: Full page generation ✓
- [ ] UC13: Multi-locale sync ✓
- [ ] UC14: SEO gap analysis ✓
- [ ] UC15: GEO QA pipeline ✓

### Graph Maintenance (UC16-UC19)
- [ ] UC16: Relationship discovery ✓
- [ ] UC17: Freshness audit ✓
- [ ] UC18: Category page ✓
- [ ] UC19: Internal linking ✓

### QA (UC20-UC21)
- [ ] UC20: Consistency check ✓
- [ ] UC21: Terminology audit ✓

### Analytics (UC22-UC23)
- [ ] UC22: Performance analysis ✓
- [ ] UC23: Graph health check ✓

### Advanced Agents (UC24-UC26)
- [ ] UC24: Research agent ✓
- [ ] UC25: Multi-agent pipeline ✓
- [ ] UC26: Self-correcting agent ✓

### Batch Operations (UC27-UC28)
- [ ] UC27: Mass page generation ✓
- [ ] UC28: Locale expansion ✓

### Neo4j Queries
- [ ] Q1-Q5: Discovery queries work
- [ ] Q6-Q10: Traversal queries work
- [ ] Q11-Q15: Analysis queries work
- [ ] Q16-Q20: Maintenance queries work
- [ ] Q21-Q24: SEO/GEO queries work

### Socratic Questions
- [ ] Q1-Q4: Architecture questions answered
- [ ] Q5-Q8: Data model questions answered
- [ ] Q9-Q12: Workflow questions answered
- [ ] Q13-Q15: Product questions answered
- [ ] Q16-Q18: Evolution questions answered

### TUI Verification
- [ ] All panels render correctly
- [ ] Real-time updates work
- [ ] Keyboard navigation works
- [ ] Status bar shows accurate info
- [ ] Agent turns visible with thinking

---

## Deliverables

1. **Status Report:** ✅ Functional / ⚠️ Partial / ❌ Non-functional
2. **Data Connection:** ✅ Real / ❌ Mock
3. **TUI Snapshots:** ASCII for each category
4. **Limitations:** Documented if any
5. **Recommendations:** What to fix/improve
