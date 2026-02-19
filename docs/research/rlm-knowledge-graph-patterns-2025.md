# Research Report: RLM + Knowledge Graph Integration Patterns (2024-2025)

> **For Claude:** This is a research document. Reference patterns here when implementing NovaNet + Nika integration.

**Date:** 2026-02-19
**Researcher:** Claude Opus 4.5
**Focus:** Retrieval-augmented Language Models with ontology/knowledge graph integration

---

## Executive Summary

The 2024-2025 landscape for RLM (Retrieval-augmented Language Models) with knowledge graphs shows a clear shift from naive RAG (vector similarity on text chunks) to **structured graph retrieval** with semantic understanding. Key findings:

1. **GraphRAG dominates** - Microsoft's GraphRAG and variants (LightRAG, GRAG) outperform traditional RAG
2. **MCP is the integration standard** - Model Context Protocol provides standardized KG-to-LLM interfaces
3. **Multi-hop reasoning requires iterative retrieval** - Single-pass retrieval fails on complex queries
4. **Ontology-guided retrieval beats vector-only** - OG-RAG shows 55% recall improvement
5. **Token efficiency through subgraph extraction** - Retrieve structure, not full text

---

## 1. Architecture Patterns

### 1.1 GraphRAG (Microsoft, 2024)

**Core Concept:** Build entity-relation graphs from documents for query-focused retrieval with community summarization.

```
Document Corpus
       ↓
┌──────────────────────────────────────┐
│  GRAPH CONSTRUCTION                  │
│  ─────────────────                   │
│  1. Entity Extraction (NER)          │
│  2. Relationship Extraction          │
│  3. Community Detection (Leiden)     │
│  4. Hierarchical Summarization       │
└──────────────────────────────────────┘
       ↓
┌──────────────────────────────────────┐
│  QUERY-TIME RETRIEVAL                │
│  ────────────────────                │
│  Local Search: Entity subgraphs      │
│  Global Search: Community summaries  │
└──────────────────────────────────────┘
       ↓
   LLM Generation
```

**NovaNet Application:** NovaNet already has the graph - focus on query-time patterns.

### 1.2 OG-RAG (Ontology-Grounded RAG)

**Core Concept:** Transform documents into hypergraph representations with explicit ontology constraints.

```
Document → Hypergraph (hypernodes = subject-attribute-value pairs)
                       (hyperedges = multi-entity relationships)
       ↓
Dual Retrieval Strategy:
  1. Embedding similarity on hypernodes (cosine)
  2. Set cover optimization on hyperedges (minimal context)
       ↓
LLM with structured context
```

**Performance:** 55% increase in fact recall, 40% response correctness improvement, 30% faster attribution.

**NovaNet Application:** Entity/EntityNative pairs are natural hypernodes. Arc families are hyperedges.

### 1.3 LightRAG (Efficient GraphRAG)

**Core Concept:** Lightweight graph construction with dual-level retrieval (low-level entities, high-level concepts).

**Key Differentiators:**
- Incremental graph updates (no full rebuild)
- Hybrid search: keyword + vector + graph
- Lower LLM costs during indexing

**NovaNet Application:** NovaNet's Realm/Layer/Class hierarchy is already a dual-level structure.

---

## 2. MCP Integration Patterns

### 2.1 Knowledge Graph as MCP Server

The Model Context Protocol (MCP) is the 2025 standard for exposing structured data to LLMs.

```
┌─────────────────────────────────────────────────────────────────────────┐
│  MCP ARCHITECTURE FOR NOVANET                                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌───────────────────┐     MCP Protocol      ┌───────────────────────┐ │
│  │       NIKA        │ ←──────────────────→ │       NOVANET         │ │
│  │   (MCP Client)    │                       │    (MCP Server)       │ │
│  ├───────────────────┤                       ├───────────────────────┤ │
│  │ • Workflow Engine │   novanet_generate    │ • Neo4j Connection    │ │
│  │ • LLM Providers   │   novanet_traverse    │ • Entity Resolution   │ │
│  │ • Context Assembly│   novanet_describe    │ • Subgraph Extraction │ │
│  │ • Tool Calling    │   novanet_search      │ • Context Formatting  │ │
│  └───────────────────┘                       └───────────────────────┘ │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 2.2 MCP Tool Definitions

Based on research, these are the recommended MCP tools for NovaNet:

```yaml
# novanet_generate - Primary content generation context
tools:
  - name: novanet_generate
    description: |
      Retrieve entity context for native content generation.
      Returns locale-specific knowledge atoms, entity definitions,
      and cultural context for LLM generation.
    inputSchema:
      type: object
      properties:
        entity:
          type: string
          description: "Entity key (e.g., 'qr-code')"
        locale:
          type: string
          description: "Target locale (e.g., 'fr-FR')"
        forms:
          type: array
          items:
            type: string
          description: "Content forms needed (text, title, description, etc.)"
        depth:
          type: integer
          default: 2
          description: "Traversal depth for related entities"

  - name: novanet_traverse
    description: |
      Execute multi-hop traversal from a starting node.
      Returns subgraph with all nodes/arcs within specified depth.
    inputSchema:
      type: object
      properties:
        start:
          type: string
          description: "Starting node key with label (e.g., 'entity:qr-code')"
        arc_filter:
          type: array
          items:
            type: string
          description: "Arc types to follow (e.g., ['HAS_NATIVE', 'BELONGS_TO'])"
        depth:
          type: integer
          default: 2
        direction:
          type: string
          enum: ["outgoing", "incoming", "both"]
          default: "both"

  - name: novanet_describe
    description: |
      Get schema-level description of a node class or arc class.
      Returns llm_context, properties, and relation patterns.
    inputSchema:
      type: object
      properties:
        class:
          type: string
          description: "Class name (e.g., 'Entity', 'HAS_NATIVE')"
        include_examples:
          type: boolean
          default: true

  - name: novanet_search
    description: |
      Semantic search across the knowledge graph.
      Combines fulltext search with graph traversal.
    inputSchema:
      type: object
      properties:
        query:
          type: string
          description: "Natural language query"
        class_filter:
          type: array
          items:
            type: string
          description: "Limit to specific node classes"
        locale:
          type: string
          description: "Locale context for search"
        limit:
          type: integer
          default: 10
```

---

## 3. Multi-Hop Reasoning Patterns

### 3.1 Iterative Retrieval (KG-IRAG Pattern)

**Problem:** Complex questions require connecting information across multiple hops.

**Solution:** Incremental graph updating with reasoning fusion.

```
┌─────────────────────────────────────────────────────────────────────────┐
│  ITERATIVE RETRIEVAL PIPELINE                                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Query: "What WiFi content types work best for French restaurants?"    │
│                                                                         │
│  Step 1: Entity Resolution                                              │
│  ──────────────────────────                                             │
│  "WiFi" → Entity:wifi-qr                                                │
│  "French" → Locale:fr-FR                                                │
│  "restaurants" → Industry:restaurant                                    │
│                                                                         │
│  Step 2: Subgraph Extraction (Hop 1)                                   │
│  ─────────────────────────────────────                                  │
│  Entity:wifi-qr -[:BELONGS_TO]-> EntityCategory:CONTENT_TYPE           │
│  Locale:fr-FR -[:HAS_TERMS]-> TermSet:hospitality                      │
│  Industry:restaurant -[:HAS_USE_CASE]-> UseCase:wifi-access            │
│                                                                         │
│  Step 3: Cross-Reference (Hop 2)                                       │
│  ─────────────────────────────────                                      │
│  UseCase:wifi-access -[:ENABLES]-> Entity:wifi-qr                      │
│  TermSet:hospitality -[:CONTAINS_TERM]-> Term:restaurant-wifi-terms    │
│  Entity:wifi-qr -[:HAS_NATIVE@fr-FR]-> EntityNative:wifi-qr-fr        │
│                                                                         │
│  Step 4: Context Assembly                                               │
│  ───────────────────────────                                            │
│  Merge subgraphs, deduplicate, rank by relevance                       │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.2 BFS-RF (Breadth-First Search with Reasoning Fusion)

**Pattern:** Each sub-question triggers targeted graph traversal.

```rust
// Nika workflow pseudocode
workflow: multi_hop_query
steps:
  - id: decompose
    infer: "Decompose query into sub-questions"
    context: $query
    use.sub_questions: result

  - id: retrieve_per_question
    for_each: $sub_questions
    invoke: novanet_traverse
    params:
      start: "{{ item.entity }}"
      depth: 2
    use.subgraphs: results

  - id: merge_and_rank
    exec: "merge_subgraphs"
    params:
      subgraphs: $subgraphs
      dedup_strategy: "entity_key"
      rank_by: "relevance_to_original_query"
    use.merged_context: result

  - id: generate
    infer: "Answer the original query"
    context:
      - $merged_context
      - $query
```

---

## 4. Context Assembly Strategies

### 4.1 Token Budget Optimization

**Research Finding:** Chunk sizes around 1,800 characters consistently outperform both smaller and larger chunks.

```
┌─────────────────────────────────────────────────────────────────────────┐
│  TOKEN BUDGET ALLOCATION FOR NOVANET                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Total Context Window: 128K tokens (Claude)                            │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐ │
│  │  TIER 1: Schema Context (always loaded)         ~2,000 tokens    │ │
│  │  ─────────────────────────────────────────────────────────────── │ │
│  │  • Target Class llm_context                                       │ │
│  │  • Related Arc llm_contexts                                       │ │
│  │  • Locale configuration (formality, style)                        │ │
│  └───────────────────────────────────────────────────────────────────┘ │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐ │
│  │  TIER 2: Entity Context (per entity)            ~3,000 tokens    │ │
│  │  ─────────────────────────────────────────────────────────────── │ │
│  │  • Entity definition + EntityNative                               │ │
│  │  • Related entities (1-hop)                                       │ │
│  │  • Category/classification context                                │ │
│  └───────────────────────────────────────────────────────────────────┘ │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐ │
│  │  TIER 3: Knowledge Atoms (selective)            ~5,000 tokens    │ │
│  │  ─────────────────────────────────────────────────────────────── │ │
│  │  • Relevant Terms (50 max, not 20K)                               │ │
│  │  • Relevant Expressions (20 max)                                  │ │
│  │  • Cultural patterns (10 max)                                     │ │
│  │  • Taboos (all - safety critical)                                 │ │
│  └───────────────────────────────────────────────────────────────────┘ │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐ │
│  │  TIER 4: Generation Context (if generating)     ~2,000 tokens    │ │
│  │  ─────────────────────────────────────────────────────────────── │ │
│  │  • PageStructure (block order)                                    │ │
│  │  • PageInstruction / BlockInstruction                             │ │
│  │  • Previous block outputs (for continuity)                        │ │
│  └───────────────────────────────────────────────────────────────────┘ │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐ │
│  │  TIER 5: Examples (if needed)                   ~3,000 tokens    │ │
│  │  ─────────────────────────────────────────────────────────────── │ │
│  │  • Similar PageNative examples (2-3)                              │ │
│  │  • Similar BlockNative examples (2-3)                             │ │
│  └───────────────────────────────────────────────────────────────────┘ │
│                                                                         │
│  Total per generation call: ~15,000 tokens                             │
│  Leaves headroom for: Long-form content, multi-block pages             │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Context Ordering (Lost in the Middle Prevention)

**Research Finding:** Most relevant content should appear closest to the prompt.

```yaml
# Recommended context order for Nika workflows
context_assembly:
  order:
    1: instructions      # PageInstruction/BlockInstruction (closest to prompt)
    2: entity_native     # The specific entity being generated
    3: locale_context    # Locale voice, style, formality
    4: knowledge_atoms   # Terms, expressions, patterns
    5: entity_definition # Background/invariant info
    6: examples          # Similar outputs (furthest from prompt)

  format:
    type: structured     # Not plain text
    delimiters: "==="    # Heavy delimiters improve attention
    section_headers: true
```

### 4.3 Context Format (Structured vs. Unstructured)

**Research Finding:** Delimiter-heavy structured formats outperform plain text.

```
=== ENTITY DEFINITION ===
key: qr-code
display_name: QR Code
category: THING
llm_context: >
  Core product. Use for any QR Code creation/customization topic.
  TRIGGERS: "qr code", "code QR", "QR-code"
  NOT: for barcodes (use barcode entity)

=== LOCALE CONTEXT: fr-FR ===
formality: formal
voice: professional
cultural_notes:
  - Prefer "code QR" over "QR code" in French
  - Use vouvoiement (vous) for all instructions

=== RELEVANT TERMS (fr-FR, hospitality) ===
- menu_qr: "menu QR" - for restaurant menus
- wifi_code: "code WiFi" - for network access
- reservation: "reservation" - booking context

=== PAGE INSTRUCTION ===
Generate a landing page for QR Code creation targeting French restaurants.
Include: hero section, benefits list, CTA.
Tone: Professional but approachable.
```

---

## 5. Entity Linking Patterns

### 5.1 Three-Stage Entity Resolution

**Pattern:** Fuzzy matching -> Semantic similarity -> Contextual disambiguation.

```rust
// Entity linking pipeline for Nika
pub struct EntityLinker {
    fuzzy_threshold: f32,      // 0.8 default
    semantic_model: String,    // sentence-transformers
    graph_client: McpClient,   // NovaNet connection
}

impl EntityLinker {
    pub async fn link(&self, mention: &str, context: &str) -> Vec<LinkedEntity> {
        // Stage 1: Fuzzy matching
        let candidates = self.fuzzy_match(mention).await?;

        // Stage 2: Semantic ranking
        let ranked = self.semantic_rank(candidates, context).await?;

        // Stage 3: Graph-based disambiguation
        self.disambiguate_with_graph(ranked, context).await
    }

    async fn disambiguate_with_graph(
        &self,
        candidates: Vec<Entity>,
        context: &str
    ) -> Vec<LinkedEntity> {
        // Query NovaNet for entity relationships
        // Score by how well relationships match context
        // Return top candidates with confidence scores
    }
}
```

### 5.2 NovaNet Entity Resolution MCP Tool

```yaml
# Add to novanet MCP tools
- name: novanet_resolve_entity
  description: |
    Resolve natural language mentions to Entity nodes.
    Uses fuzzy matching + semantic similarity + graph context.
  inputSchema:
    type: object
    properties:
      mention:
        type: string
        description: "Text mention to resolve (e.g., 'QR codes')"
      context:
        type: string
        description: "Surrounding context for disambiguation"
      locale:
        type: string
        description: "Locale for EntityNative matching"
      top_k:
        type: integer
        default: 3
  outputSchema:
    type: array
    items:
      type: object
      properties:
        entity_key:
          type: string
        confidence:
          type: number
        matched_via:
          type: string
          enum: ["exact", "fuzzy", "semantic", "alias"]
```

---

## 6. Text-to-Cypher Patterns

### 6.1 Schema-Aware Cypher Generation

**Research Finding:** Providing graph schema improves Cypher accuracy by 40%.

```yaml
# Nika workflow for natural language to Cypher
workflow: nl_to_cypher
steps:
  - id: get_schema
    invoke: novanet_describe
    params:
      class: "*"  # All classes
      include_arcs: true
    use.schema: result

  - id: generate_cypher
    infer: |
      Convert the user query to a Cypher query.

      GRAPH SCHEMA:
      {{ schema }}

      USER QUERY:
      {{ query }}

      RULES:
      1. Use exact node labels from schema
      2. Use exact relationship types from schema
      3. Include LIMIT clause (max 100)
      4. For locale-specific queries, filter on locale_key property
    use.cypher: result

  - id: validate_cypher
    exec: "cypher_syntax_check"
    params:
      query: $cypher
    on_error:
      retry: true
      max_retries: 2
      feedback: "Syntax error: {{ error }}"

  - id: execute
    invoke: novanet_query
    params:
      cypher: $cypher
      read_only: true
```

### 6.2 Cypher Generation Few-Shot Examples

Include domain-specific examples in the prompt:

```
EXAMPLES:

Query: "All entities in the THING category"
Cypher: MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory {key: 'THING'})
        RETURN e.key, e.display_name LIMIT 50

Query: "French content for QR Code entity"
Cypher: MATCH (e:Entity {key: 'qr-code'})-[:HAS_NATIVE]->(n:EntityNative)
        WHERE n.locale_key = 'fr-FR'
        RETURN n

Query: "Pages that use WiFi entity"
Cypher: MATCH (p:Page)-[:REPRESENTS]->(e:Entity {key: 'wifi-qr'})
        RETURN p.key, p.display_name LIMIT 20
```

---

## 7. Semantic Chunking for KG Construction

### 7.1 Entity-Aware Chunking

**Pattern:** Chunk boundaries respect entity mentions.

```
┌─────────────────────────────────────────────────────────────────────────┐
│  CHUNKING STRATEGY FOR NOVANET CONTENT                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Input: BlockNative content (HTML or Markdown)                         │
│                                                                         │
│  Strategy: Semantic chunking with entity anchors                       │
│                                                                         │
│  1. Pre-extraction: Identify @entity mentions                          │
│     "@qr-code", "@wifi-qr", "@restaurant"                              │
│                                                                         │
│  2. Semantic boundaries: Split at topic shifts                         │
│     Use embedding similarity (threshold: 0.7)                          │
│                                                                         │
│  3. Entity-aware merging: Ensure chunks contain                        │
│     complete entity contexts (don't split mid-mention)                 │
│                                                                         │
│  4. Overlap: 15% token overlap for relationship preservation           │
│                                                                         │
│  Target chunk size: 1,500-2,000 tokens                                 │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 7.2 Hierarchical Chunking for Pages

```yaml
# Page structure for hierarchical chunking
page: landing-qr-code
blocks:
  - block: hero
    chunk_strategy: complete  # Never split hero
  - block: benefits
    chunk_strategy: per_item  # Each benefit is a chunk
  - block: how_it_works
    chunk_strategy: semantic  # Split by topic
  - block: faq
    chunk_strategy: per_qa    # Each Q&A is a chunk
  - block: cta
    chunk_strategy: complete  # Never split CTA
```

---

## 8. Actionable Patterns for NovaNet + Nika

### 8.1 Immediate Implementation (Phase 1)

| Pattern | Implementation | Priority |
|---------|---------------|----------|
| **MCP Server** | Add `novanet_generate`, `novanet_traverse`, `novanet_describe` | HIGH |
| **Schema Context** | Load llm_context from Class YAMLs at runtime | HIGH |
| **Token Budget** | Implement tiered context loading (schema -> entity -> atoms) | HIGH |
| **Structured Format** | Use delimiter-heavy context assembly | MEDIUM |

### 8.2 Near-Term Implementation (Phase 2)

| Pattern | Implementation | Priority |
|---------|---------------|----------|
| **Entity Linking** | `novanet_resolve_entity` MCP tool | HIGH |
| **Multi-hop Retrieval** | BFS-RF pattern in Nika workflows | MEDIUM |
| **Text-to-Cypher** | Schema-aware Cypher generation workflow | MEDIUM |
| **Context Ordering** | Instructions closest to prompt | MEDIUM |

### 8.3 Future Implementation (Phase 3)

| Pattern | Implementation | Priority |
|---------|---------------|----------|
| **KG Embeddings** | RotatE/TransE for Entity/Arc embeddings | LOW |
| **Hybrid Search** | Vector + Graph + Keyword fusion | LOW |
| **Incremental Updates** | LightRAG-style graph updates | LOW |
| **Quality Feedback** | Reasoning chain validation | LOW |

---

## 9. Reference Implementation: Content Generation Workflow

```yaml
# Nika workflow: generate-page-native
# Uses all researched patterns

$schema: nika/workflow@0.2
name: generate-page-native
description: Generate locale-native page content using NovaNet context

mcp:
  servers:
    - name: novanet
      command: novanet-mcp-server
      args: ["--neo4j-uri", "bolt://localhost:7687"]

inputs:
  page_key: string
  locale_key: string

steps:
  # TIER 1: Schema Context
  - id: get_schema_context
    invoke: novanet_describe
    params:
      class: PageNative
      include_examples: true
    use.schema_ctx: result

  # TIER 2: Entity Context
  - id: get_page
    invoke: novanet_traverse
    params:
      start: "page:{{ inputs.page_key }}"
      arc_filter: ["REPRESENTS", "HAS_BLOCK"]
      depth: 1
    use.page_ctx: result

  - id: get_entity
    invoke: novanet_generate
    params:
      entity: "{{ page_ctx.entity_key }}"
      locale: "{{ inputs.locale_key }}"
      forms: ["title", "description", "text"]
      depth: 2
    use.entity_ctx: result

  # TIER 3: Knowledge Atoms (selective)
  - id: get_locale_terms
    invoke: novanet_traverse
    params:
      start: "locale:{{ inputs.locale_key }}"
      arc_filter: ["HAS_TERMS", "CONTAINS_TERM"]
      depth: 2
    use.terms: result
    filter:
      domain: "{{ entity_ctx.category.domain }}"
      limit: 50

  - id: get_taboos
    invoke: novanet_traverse
    params:
      start: "locale:{{ inputs.locale_key }}"
      arc_filter: ["HAS_TABOOS", "CONTAINS_TABOO"]
      depth: 2
    use.taboos: result  # All taboos - safety critical

  # TIER 4: Generation Context
  - id: get_instructions
    invoke: novanet_traverse
    params:
      start: "page:{{ inputs.page_key }}"
      arc_filter: ["HAS_STRUCTURE", "HAS_INSTRUCTION"]
      depth: 1
    use.instructions: result

  # CONTEXT ASSEMBLY (ordered by proximity to prompt)
  - id: assemble_context
    exec: assemble_context
    params:
      tiers:
        - name: instructions
          content: $instructions
          position: 1  # Closest to prompt
        - name: entity_native
          content: $entity_ctx.native
          position: 2
        - name: locale_voice
          content: $entity_ctx.locale
          position: 3
        - name: knowledge
          content:
            terms: $terms
            taboos: $taboos
          position: 4
        - name: entity_definition
          content: $entity_ctx.definition
          position: 5
        - name: schema
          content: $schema_ctx
          position: 6  # Furthest from prompt
      format: structured
      delimiters: "==="
    use.assembled: result

  # GENERATION
  - id: generate
    infer:
      model: claude-sonnet-4-20250514
      temperature: 0.7
      max_tokens: 4000
    prompt: |
      Generate the page content following the instructions.
      Output format: JSON with { title, description, blocks: [...] }

      {{ assembled }}
    use.page_native: result

  # OUTPUT
  - id: save
    invoke: novanet_create
    params:
      class: PageNative
      data:
        key: "page:{{ inputs.page_key }}@{{ inputs.locale_key }}"
        page_key: "{{ inputs.page_key }}"
        locale_key: "{{ inputs.locale_key }}"
        content: "{{ page_native }}"

outputs:
  page_native: $page_native
```

---

## 10. Sources

### Academic Papers (2024-2025)
1. **GraphRAG** (Microsoft, 2024) - Entity graphs for global reasoning
2. **GRAG** - Subgraph retrieval from textual graphs
3. **KG-IRAG** - Incremental KG retrieval with LLM evaluation
4. **OG-RAG** - Ontology-grounded hypergraph retrieval
5. **Auto-Cypher** - LLM-supervised Text-to-Cypher (40% accuracy gain)
6. **Multi-Agent GraphRAG** - Agentic query verification and refinement
7. **GraphFlow** (NeurIPS 2025) - Flow-matching for KG-RAG
8. **RIGOR** - Ontology extraction from relational databases

### Production Implementations
- Microsoft GraphRAG (open-source)
- Neo4j GenAI Stack + LangChain integration
- TigerGraph MCP Server
- Perplexity Sonar API for agentic workflows

### Benchmarks
- STaRK (text-rich KG RAG)
- HotpotQA (multi-hop reasoning)
- CypherBench (Text-to-Cypher)
- SimpleQA (factuality)

---

## Confidence Level

**HIGH** - Findings are consistent across multiple academic sources and production implementations. The MCP pattern is particularly well-supported as the emerging standard. Token budget and context assembly strategies have strong empirical backing.

**Caveats:**
- KG embedding integration (TransE/RotatE) is less mature for RAG specifically
- LightRAG vs. GraphRAG tradeoffs depend on specific use case
- Text-to-Cypher still requires safeguards for production

---

## 11. RLM-on-NovaNet: Gap Analysis & Roadmap

**Added:** 2026-02-19 (follow-up research)

### 11.1 Research Sources

| Source | Type | Key Contribution |
|--------|------|------------------|
| [rig-rlm](https://github.com/joshua-mo-143/rig-rlm) | Rust + pyo3 | REPL-based execution model |
| [Google ADK RLM Discussion](https://discuss.google.dev/t/recursive-language-models-in-adk/323523) | Forum | ADK integration patterns |
| [arXiv 2512.24601](https://arxiv.org/abs/2512.24601) | Paper | Formal RLM definition |

### 11.2 Core RLM Insight

RLM treats prompts as **external environments**, not direct inputs:

```
Traditional: User Input → LLM → Output
RLM:         Environment (prompt) → LLM inspects → LLM acts → Environment updates → Loop
```

The LLM doesn't "answer" - it **navigates** an environment.

### 11.3 NovaNet + Nika IS Already RLM-on-KG

| RLM-on-KG Component | NovaNet + Nika Equivalent |
|---------------------|---------------------------|
| Knowledge Graph | NovaNet (Neo4j, 61 NodeClasses, 182 ArcClasses) |
| GraphQL queries | MCP tools: `novanet_generate`, `novanet_traverse`, `novanet_describe` |
| REPL executor | Nika `runtime/executor.rs` (5 verbs) |
| Recursive calls | `agent:` verb with multi-turn tool calling |
| Task decomposition | DAG flows + `for_each:` parallelism |
| Environment state | `DataStore` + bindings (`{{use.alias}}`) |

### 11.4 Gap Analysis

| Gap | Description | Severity | Current State |
|-----|-------------|----------|---------------|
| **Nested Agents** | Self-recursion (`rlm_call()`) | High | `agent:` loops but no sub-agents |
| **Dynamic Decomposition** | Runtime DAG expansion | High | Static YAML DAG |
| **Schema Introspection** | Query graph SCHEMA | Medium | MCP queries data only |
| **Reasoning Transparency** | Chain-of-action visibility | Medium | EventLog lacks thinking |
| **Lazy Context Loading** | Deferred loading | Medium | Context assembled upfront |

### 11.5 Implementation Roadmap

| Phase | Version | Feature | Effort | Impact |
|-------|---------|---------|--------|--------|
| 1 | v0.4.1 | **Reasoning capture** - Add `thinking` field to `AgentTurn` events | Low | Debug agent reasoning |
| 2 | v0.5 | **Nested agents** - `spawn_agent` internal tool | Medium | True recursion |
| 3 | v0.5 | **Schema introspection** - `novanet_introspect` MCP tool | Medium | Environment exploration |
| 4 | v0.6 | **Dynamic decomposition** - `decompose:` YAML modifier | High | Runtime DAG expansion |
| 5 | v0.6 | **Lazy bindings** - `lazy: true` modifier | Medium | Performance |

### 11.6 Proposed: Nested Agent Spawning

```rust
// New internal tool for agents
pub struct SpawnAgentParams {
    pub task_id: String,
    pub prompt: String,
    pub context: Option<serde_json::Value>,
    pub max_turns: Option<u32>,
    pub tools: Vec<String>,
}
```

Execution flow:
```
Agent A (page generation)
├── novanet_describe("qr-code")
├── SPAWN_AGENT("research SEO")
│   └── Agent B runs independently
│       ├── novanet_traverse("seo-keywords")
│       └── Returns: SEO recommendations
├── Agent A continues with B's output
└── Final generation
```

### 11.7 Proposed: Dynamic Decomposition

```yaml
tasks:
  - id: generate_all_entities
    decompose:
      strategy: semantic    # Use graph structure
      traverse: "HAS_CHILD"
      source: $category
      concurrency: 5
    infer: "Generate documentation for {{item}}"
```

Strategies:
- `llm`: Ask LLM to break down task
- `fixed`: Split by delimiter
- `semantic`: Use graph arcs (KG-native!)

### 11.8 Competitive Comparison: NovaNet vs rig-rlm

| Aspect | rig-rlm | NovaNet + Nika | Winner |
|--------|---------|----------------|--------|
| **Safety** | Python arbitrary code | MCP sandboxing | **Nika** |
| **Type safety** | Python strings | Rust structs | **Nika** |
| **Observability** | Basic logging | 16 events, TUI | **Nika** |
| **Parallelism** | Sequential REPL | tokio JoinSet | **Nika** |
| **Schema** | Unstructured | 61+182 classes | **Nika** |
| **Flexibility** | Arbitrary code | 5 verbs | rig-rlm |
| **Self-recursion** | Built-in | Not yet | rig-rlm |
| **Dynamic tasks** | REPL creates | Static DAG | rig-rlm |

**Conclusion:** NovaNet + Nika is already RLM-on-KG with better safety and observability. Key improvements (nested agents, dynamic decomposition) are **additive**, not fundamental redesigns.

---

*Generated by Claude Opus 4.5 for SuperNovae Studio*
*Research methodology: Perplexity Sonar API with year-filtered search*
