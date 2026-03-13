# Phase 3: MCP Prompts + novanet_generate Tool Design

**Version**: 0.3.0
**Date**: 2026-02-12
**Status**: Approved (via brainstorming session)

## Overview

Phase 3 implements the full RLM-on-KG (Recursive Language Model on Knowledge Graph) pipeline for content generation, adding:
- `novanet_generate` composite tool
- 6 MCP Prompts with Full RAG evidence assembly
- Context Anchor support for cross-page references

## Architecture Decisions

### Decision 1: Generate Mode (Hybrid Block + Page)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  GENERATE MODE                                                                ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Option C: Both block and page with mode selection                            ║
║                                                                               ║
║  novanet_generate {                                                           ║
║    focus_key: "homepage",                                                     ║
║    locale: "fr-FR",                                                           ║
║    mode: "block" | "page",   ← Mode selector                                  ║
║    token_budget: 50000,                                                       ║
║    include_examples: true,                                                    ║
║    spreading_depth: 2                                                         ║
║  }                                                                            ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

**Rationale**: Single tool interface, mode parameter determines traversal scope.

### Decision 2: MCP Prompts with Full RAG

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  MCP PROMPTS: FULL RAG WITH EVIDENCE ASSEMBLY                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Prompt template includes:                                                    ║
║  ├── Tool orchestration steps (which tools to call)                           ║
║  ├── Evidence assembly pattern (how to structure context)                     ║
║  ├── Schema context (what nodes/arcs exist)                                   ║
║  └── Output format specification                                              ║
║                                                                               ║
║  Agent workflow:                                                              ║
║  1. Agent receives prompt template with {{variables}}                         ║
║  2. Prompt guides agent to call novanet_traverse, novanet_assemble            ║
║  3. Agent assembles evidence packets from tool results                        ║
║  4. Agent generates content following prompt structure                        ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

**Rationale**: Full RAG enables adaptive retrieval and proper evidence attribution.

### Decision 3: Hybrid Architecture

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  HYBRID ARCHITECTURE                                                          ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Atomic Tools (agent-called):                                                 ║
║  ├── novanet_search      → Find entities by keyword                           ║
║  ├── novanet_traverse    → Hop-by-hop graph exploration                       ║
║  ├── novanet_assemble    → Token-budgeted context assembly                    ║
║  └── novanet_atoms       → Knowledge atoms retrieval                          ║
║                                                                               ║
║  Composite Tool (for simple use cases):                                       ║
║  └── novanet_generate    → All-in-one generation context                      ║
║                                                                               ║
║  Agent can choose:                                                            ║
║  ├── Simple: Call novanet_generate directly                                   ║
║  └── Advanced: Orchestrate atomic tools for custom retrieval                  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

**Rationale**: Flexibility for both simple and complex agent patterns.

### Decision 4: Structured Output Format

```rust
pub struct GenerateResult {
    /// Assembled prompt ready for LLM
    pub prompt: String,

    /// Evidence summary (packets used)
    pub evidence_summary: Vec<EvidenceSummary>,

    /// Locale context (voice, culture, formatting)
    pub locale_context: LocaleContext,

    /// Token usage breakdown
    pub token_usage: TokenUsage,

    /// Metadata (timing, sources)
    pub metadata: GenerateMetadata,
}
```

### Decision 5: Extended 6 MCP Prompts

| Prompt | Purpose | Parameters |
|--------|---------|------------|
| `cypher_query` | Schema-aware Cypher generation | intent, constraints |
| `cypher_explain` | Explain query results in context | query, results |
| `block_generation` | Single block generation context | block_key, locale |
| `page_generation` | Full page orchestration | page_key, locale |
| `entity_analysis` | Entity deep dive | entity_key, locale |
| `locale_briefing` | Locale voice/culture summary | locale_key |

---

## Page Composition Architecture

### 4-Layer Model

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  PAGE COMPOSITION: 4-LAYER ARCHITECTURE                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  LAYER 1: INVARIANT (Structure)                                             │
│  ════════════════════════════════                                           │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                    │
│  │    Page     │────▶│    Block    │────▶│    Block    │                    │
│  │  homepage   │     │  hero-cta   │     │  features   │                    │
│  │ (invariant) │     │ (invariant) │     │ (invariant) │                    │
│  └─────────────┘     └─────────────┘     └─────────────┘                    │
│        │ HAS_BLOCK          │ USES_ENTITY     │ USES_ENTITY                 │
│        ▼                    ▼                 ▼                             │
│                                                                             │
│  LAYER 2: SEMANTIC (Meaning)                                                │
│  ═══════════════════════════                                                │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                    │
│  │   Entity    │     │   Entity    │     │   Entity    │                    │
│  │  qr-code    │     │  scanning   │     │  customize  │                    │
│  │ (invariant) │     │ (invariant) │     │ (invariant) │                    │
│  └─────────────┘     └─────────────┘     └─────────────┘                    │
│        │ HAS_CONTENT        │ HAS_CONTENT     │ HAS_CONTENT                 │
│        ▼                    ▼                 ▼                             │
│                                                                             │
│  LAYER 3: KNOWLEDGE (Locale-Native)                                         │
│  ══════════════════════════════════                                         │
│  ┌─────────────────────────────────────────────────────────────────┐        │
│  │  Locale:fr-FR                                                   │        │
│  │  ├── TermSet:technical → Term:qr-code = "code QR"               │        │
│  │  ├── ExpressionSet:marketing → Expression:cta = "Créez..."      │        │
│  │  ├── CultureSet → CultureRef:formality = "vous" (formal)        │        │
│  │  └── Style:voice = "professionnel, accessible"                  │        │
│  └─────────────────────────────────────────────────────────────────┘        │
│        │ generates                                                          │
│        ▼                                                                    │
│                                                                             │
│  LAYER 4: OUTPUT (Generated)                                                │
│  ═══════════════════════════                                                │
│  ┌─────────────────────┐     ┌─────────────────────┐                        │
│  │   PageGenerated     │     │   BlockGenerated    │                        │
│  │ page:homepage@fr-FR │     │ block:hero-cta@fr-FR│                        │
│  │   (generated)       │     │    (generated)      │                        │
│  │                     │     │                     │                        │
│  │ title: "Générateur" │     │ headline: "Créez"   │                        │
│  │ meta_desc: "..."    │     │ body: "..."         │                        │
│  └─────────────────────┘     └─────────────────────┘                        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Composite Key Pattern

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  COMPOSITE KEY PATTERN                                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Format: {kind}:{invariant_key}@{locale}                                    │
│                                                                             │
│  Examples:                                                                  │
│  ├── page:homepage@fr-FR         → PageGenerated for French homepage        │
│  ├── page:homepage@de-DE         → PageGenerated for German homepage        │
│  ├── block:hero-cta@fr-FR        → BlockGenerated for French hero           │
│  ├── entity:qr-code@fr-FR        → EntityContent for French QR entity       │
│  └── entity:qr-code@ja-JP        → EntityContent for Japanese QR entity     │
│                                                                             │
│  Benefits:                                                                  │
│  ├── Globally unique across all locales                                     │
│  ├── Parseable: split on @ to get invariant_key + locale                    │
│  ├── Query-friendly: STARTS WITH 'page:homepage@' finds all locales         │
│  └── Self-documenting: key reveals parent and locale at a glance            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Generation Flow (6 Phases)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  GENERATION FLOW: 6 PHASES                                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │ PHASE 1: STRUCTURE DISCOVERY                                          │  │
│  │ ══════════════════════════════                                        │  │
│  │ Query: (Page {key: $pageKey})-[:HAS_BLOCK*1..3]->(Block)              │  │
│  │ Result: Block tree with depth + order                                 │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │ PHASE 2: SEMANTIC CONTEXT                                             │  │
│  │ ═════════════════════════                                             │  │
│  │ Query: (Block)-[:USES_ENTITY]->(Entity)-[:HAS_CONTENT]->              │  │
│  │        (EntityContent {locale: $locale})                              │  │
│  │ Result: Entity definitions + locale-specific content                  │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │ PHASE 3: KNOWLEDGE ATOMS                                              │  │
│  │ ════════════════════════                                              │  │
│  │ Query: (Locale)-[:HAS_TERMS]->(TermSet)-[:CONTAINS_TERM]->(Term)      │  │
│  │        WHERE Term.domain IN $domains                                  │  │
│  │ Result: Domain-specific terms, expressions, patterns                  │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │ PHASE 4: LOCALE CONTEXT                                               │  │
│  │ ═══════════════════════                                               │  │
│  │ Query: (Locale)-[:HAS_CULTURE]->(Culture)                             │  │
│  │        (Locale)-[:HAS_STYLE]->(Style)                                 │  │
│  │ Result: Voice guidelines, formality, cultural preferences             │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │ PHASE 5: EVIDENCE ASSEMBLY                                            │  │
│  │ ══════════════════════════                                            │  │
│  │ Algorithm: Sort by relevance, fit into token_budget                   │  │
│  │ Result: Prioritized evidence packets (~200 bytes each)                │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │ PHASE 6: PROMPT ASSEMBLY                                              │  │
│  │ ════════════════════════                                              │  │
│  │ Output: Structured prompt with locale context + evidence              │  │
│  │         Ready for LLM generation                                      │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Context Anchor Pattern

Context Anchors enable cross-page references in generated content, creating internal links.

### Concept

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CONTEXT ANCHOR: CROSS-PAGE REFERENCES                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  When generating Block content, the LLM may reference other Pages.          │
│  Context Anchors provide the metadata needed to create proper links.        │
│                                                                             │
│  Example:                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │ Block: features-overview                                            │    │
│  │                                                                     │    │
│  │ Generated text (fr-FR):                                             │    │
│  │ "Découvrez notre {{anchor:pricing|page de tarifs}} pour..."         │    │
│  │                       ↑                                             │    │
│  │                       Context Anchor                                │    │
│  │                                                                     │    │
│  │ Resolved output:                                                    │    │
│  │ "Découvrez notre <a href="/fr/tarifs">page de tarifs</a> pour..."   │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│  Arc: (Block)-[:REFERENCES_PAGE]->(Page)                                    │
│  Properties: anchor_text, anchor_context                                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Implementation

```rust
/// Context anchor for cross-page references
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ContextAnchor {
    /// Referenced page key
    pub page_key: String,
    /// Suggested anchor text (locale-specific)
    pub anchor_text: String,
    /// URL slug for the referenced page
    pub slug: String,
    /// Context hint for LLM (when to use this anchor)
    pub context_hint: Option<String>,
}

/// Added to GenerateResult
pub struct GenerateResult {
    // ... existing fields ...

    /// Available context anchors for cross-page linking
    pub context_anchors: Vec<ContextAnchor>,
}
```

### Query for Context Anchors

```cypher
// Find pages referenced by this block
MATCH (b:Block {key: $blockKey})-[:REFERENCES_PAGE]->(p:Page)
OPTIONAL MATCH (p)-[:HAS_GENERATED]->(pg:PageGenerated {locale: $locale})
RETURN p.key AS page_key,
       COALESCE(pg.title, p.name) AS anchor_text,
       COALESCE(pg.slug, p.slug) AS slug
```

---

## MCP Prompt Specifications

### 1. cypher_query

```yaml
name: cypher_query
description: Generate schema-aware Cypher queries
arguments:
  - name: intent
    description: Natural language description of what to query
    required: true
  - name: constraints
    description: Additional constraints (realm, layer, limit)
    required: false
template: |
  You are a NovaNet Cypher expert. Generate a query for: {{intent}}

  ## Schema Context
  - Realms: shared (40 nodes), org (21 nodes)
  - Layers: config, locale, geography, knowledge, foundation, structure, semantic, instruction, output
  - Key arcs: HAS_BLOCK, USES_ENTITY, HAS_CONTENT, HAS_GENERATED

  ## Composite Key Pattern
  Generated nodes use: {kind}:{key}@{locale}
  Example: page:homepage@fr-FR, entity:qr-code@ja-JP

  {{#if constraints}}
  ## Constraints
  {{constraints}}
  {{/if}}

  Return valid Cypher with parameters using $paramName syntax.
```

### 2. cypher_explain

```yaml
name: cypher_explain
description: Explain query results in business context
arguments:
  - name: query
    description: The Cypher query that was executed
    required: true
  - name: results
    description: JSON array of query results
    required: true
template: |
  Explain these NovaNet query results in business terms.

  ## Query
  {{query}}

  ## Results
  {{results}}

  Provide:
  1. What the query found
  2. Business meaning of the results
  3. Suggested next queries or actions
```

### 3. block_generation

```yaml
name: block_generation
description: Generate context for a single block
arguments:
  - name: block_key
    description: The block's key
    required: true
  - name: locale
    description: Target locale (BCP-47)
    required: true
  - name: token_budget
    description: Maximum tokens for context
    required: false
    default: 10000
template: |
  Generate content for block "{{block_key}}" in locale "{{locale}}".

  ## Instructions
  1. Call novanet_traverse with:
     - start_key: "{{block_key}}"
     - max_depth: 2
     - arc_families: ["semantic", "localization"]

  2. Call novanet_atoms with:
     - locale: "{{locale}}"
     - domains: (extract from entity categories)

  3. Assemble evidence packets by relevance

  ## Output Format
  Generate the block content following the locale's voice guidelines.
  Use {{anchor:page_key|text}} syntax for cross-page links.
```

### 4. page_generation

```yaml
name: page_generation
description: Orchestrate full page generation
arguments:
  - name: page_key
    description: The page's key
    required: true
  - name: locale
    description: Target locale (BCP-47)
    required: true
  - name: token_budget
    description: Maximum tokens for context
    required: false
    default: 50000
template: |
  Generate full page "{{page_key}}" in locale "{{locale}}".

  ## Orchestration Steps

  ### Step 1: Page Structure
  Call novanet_traverse:
  - start_key: "{{page_key}}"
  - arc_kinds: ["HAS_BLOCK"]
  - max_depth: 3

  ### Step 2: For Each Block
  Invoke block_generation prompt with:
  - block_key: (from step 1)
  - locale: "{{locale}}"
  - token_budget: {{token_budget}} / block_count

  ### Step 3: Page Metadata
  Assemble:
  - title (from PageGenerated or generate)
  - meta_description
  - og_image_prompt

  ## Context Anchors
  Include available anchors for internal linking:
  {{#each context_anchors}}
  - {{page_key}}: "{{anchor_text}}" → {{slug}}
  {{/each}}
```

### 5. entity_analysis

```yaml
name: entity_analysis
description: Deep analysis of an entity
arguments:
  - name: entity_key
    description: The entity's key
    required: true
  - name: locale
    description: Analysis locale
    required: true
template: |
  Analyze entity "{{entity_key}}" for locale "{{locale}}".

  ## Analysis Steps

  1. Fetch entity definition:
     novanet_search { query: "{{entity_key}}", kinds: ["Entity"] }

  2. Fetch locale content:
     novanet_traverse {
       start_key: "{{entity_key}}",
       arc_kinds: ["HAS_CONTENT"],
       target_kinds: ["EntityContent"]
     }

  3. Find usage context:
     novanet_traverse {
       start_key: "{{entity_key}}",
       direction: "incoming",
       arc_kinds: ["USES_ENTITY"]
     }

  ## Output
  Provide:
  - Entity definition summary
  - Locale-specific adaptations
  - Pages/Blocks using this entity
  - Related entities (semantic connections)
```

### 6. locale_briefing

```yaml
name: locale_briefing
description: Locale voice and culture summary
arguments:
  - name: locale_key
    description: The locale key (BCP-47)
    required: true
template: |
  Provide a briefing for locale "{{locale_key}}".

  ## Retrieval
  Call novanet_traverse:
  - start_key: "{{locale_key}}"
  - max_depth: 2
  - arc_families: ["localization", "ownership"]

  ## Briefing Structure

  ### Voice Guidelines
  - Formality level
  - Tone descriptors
  - Vocabulary preferences

  ### Cultural Context
  - Key cultural references
  - Taboos to avoid
  - Local expressions

  ### Technical
  - Date/time formats
  - Number formats
  - Currency display

  ### Content Examples
  Include 3-5 example phrases demonstrating the voice.
```

---

## Implementation Plan

### Phase 3A: novanet_generate Tool

```
tools/novanet-mcp/src/tools/generate.rs
├── GenerateParams { focus_key, locale, mode, token_budget, ... }
├── GenerateResult { prompt, evidence_summary, locale_context, ... }
├── execute() → orchestrates traverse + assemble + atoms
├── build_block_context() → single block generation
├── build_page_context() → full page orchestration
└── resolve_context_anchors() → cross-page links
```

### Phase 3B: MCP Prompts

```
tools/novanet-mcp/src/prompts/
├── mod.rs (prompt registry)
├── cypher_query.rs
├── cypher_explain.rs
├── block_generation.rs
├── page_generation.rs
├── entity_analysis.rs
└── locale_briefing.rs
```

### Phase 3C: Server Integration

```
tools/novanet-mcp/src/server/handler.rs
├── handle_prompts_list() → return 6 prompts
├── handle_prompts_get() → return specific prompt template
└── handle_tools_call() → add novanet_generate handler
```

---

## Testing Strategy

### Unit Tests

- `generate.rs`: Evidence assembly, token budgeting, context anchor resolution
- Prompts: Template rendering with mock arguments

### Integration Tests

- End-to-end generation flow with Neo4j
- MCP protocol compliance for prompts
- Cross-page anchor resolution

### Property Tests

- Token budget never exceeded
- Evidence packets maintain ~200 byte target
- Context anchors always resolve to valid pages

---

## Future Evolution

This design supports future enhancements:

1. **Streaming Generation**: Evidence packets can stream as they're discovered
2. **Caching**: Cache evidence packets by (entity_key, locale) for reuse
3. **Feedback Loop**: Store generation quality scores for relevance tuning
4. **Multi-Agent**: Page orchestrator can dispatch to specialized block agents
5. **A/B Testing**: Generate multiple variants with different evidence selection

---

## References

- ADR-007: Generation, Not Translation
- ADR-014: Naming Convention (Content/Generated suffixes)
- ADR-021: Query-First Architecture
- `spreading-activation.yaml`: Temperature weighting configuration
- `block-generation.yaml`: Existing view query pattern
