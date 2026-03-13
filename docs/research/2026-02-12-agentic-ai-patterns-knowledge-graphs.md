# Research Report: Agentic AI Patterns for Knowledge Graph Interaction

**Date**: 2026-02-12
**Researcher**: Claude Opus 4.5
**Focus**: Patterns applicable to NovaNet MCP for native content generation

---

## Summary

This report analyzes the latest agentic AI patterns (2024-2025) for knowledge graph interaction, with specific focus on patterns applicable to NovaNet's architecture. NovaNet uses a Neo4j knowledge graph with 59 node types and 159 arc types to orchestrate native content generation across 200+ locales. The key finding is that NovaNet's existing architecture aligns well with cutting-edge patterns, particularly the **orchestrator-subagent pattern**, **Graph RAG**, and **MCP-based tool integration**.

---

## Key Findings

### 1. ReAct Pattern (Reason + Act)

**What it is**: ReAct integrates iterative reasoning with action execution in a thought-action-observation loop. The agent reasons about the current state, takes an action (often a tool call), observes the result, and repeats.

**2024-2025 Implementations**:
- LangGraph extends ReAct for graph-based workflows with "plan-and-execute" cycles
- Graph learning extracts relationships from KGs to inform ReAct's reasoning steps
- Agents query KGs for state updates before tool calls

**Applicability to NovaNet**:

```
NovaNet ReAct Loop for Block Generation:

THOUGHT: "I need to generate hero-pricing block for fr-FR locale"
    |
ACTION: Query KG for block context
    |
    v
CYPHER: MATCH (b:Block {key: "hero-pricing"})
        MATCH (b)-[:USES_ENTITY]->(e:Entity)-[:HAS_CONTENT]->(ec:EntityContent)
              -[:FOR_LOCALE]->(l:Locale {key: "fr-FR"})
        MATCH (l)-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)
        RETURN b, e, ec, collect(t) AS terms
    |
OBSERVATION: Block uses Entity "qr-code-generator", locale has 50 relevant terms
    |
THOUGHT: "I have context. Generate natively in French."
    |
ACTION: Generate content using EntityContent + Terms
    |
OBSERVATION: Generated HTML with native French expressions
    |
THOUGHT: "Content complete. Store in BlockGenerated."
    |
ACTION: Create BlockGenerated node with FOR_LOCALE arc
```

**Recommendation**: Implement ReAct loop in NovaNet agent with MCP tools for:
- `read_block_context` - Fetch block + entity + locale knowledge
- `generate_native_content` - LLM generation with context
- `store_generated` - Write BlockGenerated to Neo4j

---

### 2. Tool-Using Agents with Knowledge Graphs

**What it is**: Agents use KGs as structured tools via standardized APIs (MCP, function calling). The agent discovers available tools, selects based on reasoning, and invokes them.

**2024-2025 Implementations**:
- **Anthropic MCP**: Open-source protocol for tool discovery and invocation
- **Neo4j GenAI**: Multi-agent systems with graph query tools
- **Graphiti Framework**: Temporal KGs as agent state stores

**NovaNet's Current MCP Setup**:

```json
{
  "mcpServers": {
    "neo4j": {
      "command": "uvx",
      "args": ["mcp-neo4j-cypher"],
      "env": {
        "NEO4J_URI": "neo4j://localhost:7687",
        "NEO4J_USERNAME": "neo4j",
        "NEO4J_PASSWORD": "novanetpassword"
      }
    }
  }
}
```

**Recommended Tool Architecture**:

| Tool | Type | Purpose |
|------|------|---------|
| `get_block_generation_context` | Resource | Fetch Block + Entity + LocaleKnowledge for generation |
| `get_locale_knowledge` | Resource | Fetch Terms, Expressions, Patterns for a locale |
| `get_entity_content` | Resource | Fetch EntityContent for specific locale |
| `write_generated_block` | Tool | Store BlockGenerated with relationships |
| `write_generation_job` | Tool | Create/update GenerationJob status |
| `get_seo_keywords` | Resource | Fetch SEOKeyword targets for content optimization |

**MCP Server Design for NovaNet**:

```typescript
// novanet-mcp-server.ts
const server = new MCPServer({
  name: "novanet",
  version: "1.0.0",

  resources: {
    "block-context": {
      uri: "novanet://block/{blockKey}/context/{locale}",
      description: "Full generation context for a block in a locale",
      mimeType: "application/json"
    },
    "locale-knowledge": {
      uri: "novanet://locale/{locale}/knowledge",
      description: "All knowledge atoms for a locale",
      mimeType: "application/json"
    }
  },

  tools: {
    "generate-block": {
      description: "Generate native content for a block",
      inputSchema: {
        blockKey: "string",
        locale: "string",
        model: "string"
      }
    }
  },

  prompts: {
    "native-generation": {
      description: "Template for native content generation",
      arguments: ["blockType", "locale", "entityContext"]
    }
  }
});
```

---

### 3. Multi-Step Reasoning over Graphs (Graph RAG)

**What it is**: Graph RAG combines knowledge graph traversal with LLM generation. The KG provides structured facts; the LLM provides fluency.

**Core Pattern**:
1. LLM converts query to graph query (Cypher/SPARQL)
2. Graph returns relevant subgraph
3. Subgraph populates LLM context
4. LLM generates grounded response

**2024-2025 Implementations**:
- **Microsoft GraphRAG**: LLM builds KG, clusters for global summaries
- **LlamaIndex KG**: Query augmentation before vector search
- **Neo4j GenAI**: Multi-graph queries with validation

**NovaNet Graph RAG Architecture**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET GRAPH RAG FOR CONTENT GENERATION                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. QUERY UNDERSTANDING                                                     │
│     User: "Generate pricing page for French market"                         │
│     LLM: Extract: page_key=pricing, locale=fr-FR                           │
│                                                                             │
│  2. GRAPH TRAVERSAL (Cypher)                                               │
│     MATCH (p:Page {key: "pricing"})-[:HAS_BLOCK]->(b:Block)                │
│     MATCH (b)-[:USES_ENTITY]->(e:Entity)                                   │
│     MATCH (e)-[:HAS_CONTENT]->(ec:EntityContent)-[:FOR_LOCALE]->           │
│           (l:Locale {key: "fr-FR"})                                        │
│     MATCH (l)-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)        │
│     WHERE t.semantic_field IN ['pricing', 'value', 'urgency']              │
│     RETURN p, collect(b) AS blocks, collect(e) AS entities,                │
│            collect(ec) AS content, collect(t) AS terms                     │
│                                                                             │
│  3. CONTEXT ASSEMBLY                                                        │
│     - Page structure (invariant)                                           │
│     - Block instructions (invariant)                                       │
│     - Entity definitions (invariant)                                       │
│     - EntityContent (fr-FR localized)                                      │
│     - Terms, Expressions, Patterns (fr-FR knowledge)                       │
│     - Style settings (fr-FR)                                               │
│                                                                             │
│  4. NATIVE GENERATION                                                       │
│     LLM generates in French using ONLY French context                      │
│     No translation - pure native generation                                │
│                                                                             │
│  5. STORAGE                                                                 │
│     CREATE (bg:BlockGenerated)-[:FOR_LOCALE]->(l:Locale {key: "fr-FR"})   │
│     CREATE (b)-[:HAS_GENERATED]->(bg)                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Key Insight**: NovaNet's architecture already implements Graph RAG principles:
- **KG supplies truth**: Entity, Block, Locale structures
- **LLM supplies fluency**: Native generation in target locale
- **No translation**: Content generated natively per locale

---

### 4. Agent Memory Architectures

**What it is**: Graph-based memory systems maintain agent state across sessions using nodes (entities/events) and edges (relationships/temporal links).

**Memory Types**:

| Type | Graph Implementation | NovaNet Equivalent |
|------|---------------------|-------------------|
| **Working Memory** | Active context window | Current generation context |
| **Episodic Memory** | Timestamped event nodes | GenerationJob, PREVIOUS_VERSION chains |
| **Long-term Memory** | Knowledge graph structure | Entity, EntityContent, LocaleKnowledge |
| **Semantic Memory** | Entity relationships | SEMANTIC_LINK, USES_ENTITY arcs |

**2024-2025 Implementations**:
- **Graphiti/Zep**: Temporal knowledge graphs with 94.8% retrieval accuracy
- **MemGPT**: Virtual memory with working/archive distinction
- **Neo4j GenAI**: Hybrid graph + vector search

**NovaNet Memory Architecture**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET AGENT MEMORY                                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  WORKING MEMORY (per-generation context)                                   │
│  ├── Block being generated                                                 │
│  ├── Current locale (fr-FR)                                                │
│  ├── Relevant Terms, Expressions, Patterns                                 │
│  └── Entity definitions + EntityContent                                    │
│                                                                             │
│  EPISODIC MEMORY (generation history)                                      │
│  ├── GenerationJob nodes (status, timestamps)                              │
│  ├── PREVIOUS_VERSION chains on BlockGenerated                             │
│  ├── EvaluationSignal feedback loops                                       │
│  └── OutputArtifact versioning                                             │
│                                                                             │
│  LONG-TERM MEMORY (persistent knowledge)                                   │
│  ├── SHARED realm: Locale, LocaleKnowledge, Geography                     │
│  ├── ORG realm: Project, Page, Block, Entity structures                   │
│  └── Immutable once created (append-only updates)                         │
│                                                                             │
│  SEMANTIC MEMORY (relationships)                                           │
│  ├── USES_ENTITY: Block → Entity                                          │
│  ├── SEMANTIC_LINK: Entity ↔ Entity                                       │
│  ├── CONTAINS_*: Container → Atom relationships                           │
│  └── HAS_CONTENT / HAS_GENERATED ownership chains                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Recommendation**: Add temporal tracking to NovaNet:
1. **Generation Sessions**: Track agent sessions with start/end timestamps
2. **Feedback Loops**: Use EvaluationSignal to improve future generations
3. **Version Chains**: Maintain PREVIOUS_VERSION for content evolution

---

### 5. Self-Improving Agents

**What it is**: Agents that learn from feedback, refine prompts, and optimize reasoning without manual intervention.

**2024-2025 Patterns**:
- **Reflexion**: Verbal self-critique with heuristic feedback
- **Self-Optimizing Prompts**: Dynamic prompt refinement based on outcomes
- **Multi-Agent Correction**: Agents monitor and fix each other
- **Constitutional AI**: Self-governance within defined boundaries

**Self-Improvement Loop for NovaNet**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET SELF-IMPROVEMENT CYCLE                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. GENERATE                                                               │
│     Agent generates BlockGenerated for fr-FR                               │
│                                                                             │
│  2. EVALUATE                                                               │
│     EvaluationSignal captures:                                             │
│     - Quality score (0-100)                                                │
│     - Locale authenticity (native expressions used?)                       │
│     - SEO alignment (keywords present?)                                    │
│     - Brand consistency (BrandIdentity rules followed?)                    │
│                                                                             │
│  3. REFLECT                                                                │
│     Agent analyzes low-scoring outputs:                                    │
│     "French content scored 65/100 - missing idiomatic expressions"        │
│     "SEO keywords underutilized in H1"                                     │
│                                                                             │
│  4. ADAPT                                                                  │
│     Update PromptArtifact with learnings:                                  │
│     - Add instruction: "Use at least 3 Terms from TermSet"                │
│     - Add instruction: "Include primary SEOKeyword in first paragraph"    │
│                                                                             │
│  5. PROPAGATE                                                              │
│     Share improvements across locales:                                     │
│     - Pattern works for fr-FR → Apply to fr-CA, fr-BE                     │
│     - Store successful patterns in PatternSet                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Implementation via Graph**:

```cypher
// Track evaluation feedback
CREATE (es:EvaluationSignal {
  key: "eval:block-hero-pricing@fr-FR:2026-02-12",
  quality_score: 65,
  locale_authenticity: 0.7,
  seo_alignment: 0.5,
  feedback: "Missing idiomatic expressions. SEO keywords underutilized.",
  created_at: datetime()
})
CREATE (bg)-[:HAS_EVALUATION]->(es)

// Update prompt artifact with learnings
MATCH (pa:PromptArtifact {key: "prompt:hero-pricing"})
SET pa.instructions = pa.instructions + "
[LEARNED] Use at least 3 Terms from locale TermSet.
[LEARNED] Include primary SEOKeyword in first paragraph."
SET pa.updated_at = datetime()
```

---

### 6. Orchestrator-Subagent Pattern

**What it is**: A central orchestrator dispatches tasks to specialized subagents, each responsible for a specific function.

**NovaNet's Existing Architecture**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET ORCHESTRATOR-SUBAGENT ARCHITECTURE                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ORCHESTRATOR                                                              │
│  ├── Receives: "Generate pricing page for all supported locales"           │
│  ├── Queries: Page → Block structure                                       │
│  ├── Dispatches: One subagent per Block × Locale                          │
│  └── Aggregates: Collects BlockGenerated, creates PageGenerated           │
│                                                                             │
│  SUBAGENTS (specialized by BlockType)                                      │
│  ├── HeroBlockAgent: Generates hero sections with brand voice             │
│  ├── PricingBlockAgent: Handles pricing tables with locale formatting     │
│  ├── FAQBlockAgent: Generates Q&A using Entity definitions                │
│  ├── CTABlockAgent: Creates calls-to-action with urgency Terms            │
│  └── TestimonialBlockAgent: Adapts social proof for cultural context      │
│                                                                             │
│  WORKFLOW                                                                   │
│  1. Orchestrator queries: MATCH (p:Page)-[:HAS_BLOCK]->(b:Block)          │
│  2. For each Block, identify BlockType                                     │
│  3. Dispatch to appropriate subagent with context                         │
│  4. Subagent queries locale knowledge, generates natively                 │
│  5. Subagent stores BlockGenerated                                         │
│  6. Orchestrator assembles PageGenerated from all BlockGenerated          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**MCP Tool Design for Orchestrator**:

```typescript
// Orchestrator tools
const orchestratorTools = {
  "plan-generation": {
    description: "Plan generation workflow for a page across locales",
    execute: async ({ pageKey, locales }) => {
      // Query page structure
      const structure = await neo4j.read(`
        MATCH (p:Page {key: $pageKey})-[:HAS_BLOCK]->(b:Block)
        MATCH (b)-[:OF_TYPE]->(bt:BlockType)
        RETURN b.key AS blockKey, bt.key AS blockType
        ORDER BY b.position
      `, { pageKey });

      // Create generation plan
      const tasks = [];
      for (const locale of locales) {
        for (const block of structure) {
          tasks.push({
            blockKey: block.blockKey,
            blockType: block.blockType,
            locale: locale,
            agent: `${block.blockType}-agent`
          });
        }
      }
      return { tasks, totalCount: tasks.length };
    }
  },

  "dispatch-block-generation": {
    description: "Dispatch a block generation task to appropriate subagent",
    execute: async ({ blockKey, locale, blockType }) => {
      // Fetch full context
      const context = await getBlockContext(blockKey, locale);
      // Invoke subagent via MCP
      return await mcpClient.invoke(`${blockType}-agent`, {
        tool: "generate-native",
        args: { context, locale }
      });
    }
  }
};
```

---

### 7. Multilingual Native Generation (Not Translation)

**Key Research Finding**: Effective multilingual AI content generation treats language as a **dimension of graph structure**, not a property to translate.

**NovaNet's Approach Validated by Research**:

| Pattern | Traditional Translation | NovaNet Native Generation |
|---------|------------------------|---------------------------|
| Data Flow | Source → Translate → Target | Entity (invariant) → Generate → EntityContent |
| Graph Structure | Single node + translated properties | Separate Content nodes per locale |
| Cultural Adaptation | Post-hoc localization | Built into generation context |
| Knowledge | Same knowledge, translated | Locale-specific Terms, Expressions, Patterns |

**Graph Architecture for Native Generation**:

```
Entity (invariant)
├── key: "qr-code-generator"
├── description: "Technical definition in English"
└── llm_context: "USE: when referencing the core product..."

EntityContent (fr-FR)
├── key: "entity:qr-code-generator@fr-FR"
├── title: "Generateur de QR Code"
├── description: "Description native en francais..."
└── -[:FOR_LOCALE]-> Locale {key: "fr-FR"}

EntityContent (ja-JP)
├── key: "entity:qr-code-generator@ja-JP"
├── title: "QRコードジェネレーター"
├── description: "日本語のネイティブな説明..."
└── -[:FOR_LOCALE]-> Locale {key: "ja-JP"}
```

**Context Assembly for Native Generation**:

```cypher
// Fetch ONLY target locale context - no source language involved
MATCH (b:Block {key: $blockKey})
MATCH (b)-[:USES_ENTITY]->(e:Entity)-[:HAS_CONTENT]->(ec:EntityContent)
      -[:FOR_LOCALE]->(l:Locale {key: $locale})
MATCH (l)-[:HAS_STYLE]->(s:Style)
MATCH (l)-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)
WHERE t.semantic_field IN $relevantFields
MATCH (l)-[:HAS_EXPRESSIONS]->(es:ExpressionSet)
      -[:CONTAINS_EXPRESSION]->(expr:Expression)
WHERE expr.register = s.formality_level

RETURN {
  block: b,
  entity: e,
  entityContent: ec,
  style: s,
  terms: collect(DISTINCT t),
  expressions: collect(DISTINCT expr)
} AS generationContext
```

---

## Recommended Architecture for NovaNet Agent

### Phase 1: MCP Server Implementation

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET MCP SERVER                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  RESOURCES (read-only data)                                                │
│  ├── novanet://block/{key}/context/{locale}                               │
│  ├── novanet://page/{key}/structure                                       │
│  ├── novanet://locale/{key}/knowledge                                     │
│  ├── novanet://entity/{key}/content/{locale}                              │
│  └── novanet://seo/keywords/{locale}                                      │
│                                                                             │
│  TOOLS (executable actions)                                                │
│  ├── create-generation-job                                                │
│  ├── generate-block-native                                                │
│  ├── store-block-generated                                                │
│  ├── evaluate-content                                                     │
│  └── update-prompt-artifact                                               │
│                                                                             │
│  PROMPTS (templates)                                                       │
│  ├── native-generation (per BlockType)                                    │
│  ├── quality-evaluation                                                   │
│  └── self-reflection                                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Phase 2: ReAct Agent Loop

```python
class NovaNetAgent:
    def __init__(self, mcp_client):
        self.mcp = mcp_client
        self.memory = AgentMemory()

    async def generate_page(self, page_key: str, locale: str):
        # THOUGHT: Plan the generation
        structure = await self.mcp.get_resource(
            f"novanet://page/{page_key}/structure"
        )

        for block in structure.blocks:
            # THOUGHT: Prepare block context
            context = await self.mcp.get_resource(
                f"novanet://block/{block.key}/context/{locale}"
            )

            # ACTION: Generate native content
            generated = await self.mcp.invoke_tool(
                "generate-block-native",
                {
                    "block_key": block.key,
                    "locale": locale,
                    "context": context
                }
            )

            # OBSERVATION: Evaluate quality
            evaluation = await self.mcp.invoke_tool(
                "evaluate-content",
                {"content": generated, "locale": locale}
            )

            # THOUGHT: Self-reflect on low scores
            if evaluation.score < 80:
                reflection = await self.reflect(generated, evaluation)
                await self.mcp.invoke_tool(
                    "update-prompt-artifact",
                    {"learnings": reflection}
                )

            # ACTION: Store result
            await self.mcp.invoke_tool(
                "store-block-generated",
                {"block_key": block.key, "locale": locale, "content": generated}
            )
```

### Phase 3: Multi-Agent Orchestration

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET MULTI-AGENT SYSTEM                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ORCHESTRATOR AGENT                                                        │
│  ├── Plans generation workflow                                             │
│  ├── Dispatches to block agents                                            │
│  ├── Monitors progress via GenerationJob                                   │
│  └── Aggregates results into PageGenerated                                 │
│                                                                             │
│  BLOCK AGENTS (per BlockType)                                              │
│  ├── HeroAgent: Brand voice, emotional hooks                               │
│  ├── PricingAgent: Locale-specific number formatting                       │
│  ├── FAQAgent: Entity-based Q&A generation                                 │
│  ├── CTAAgent: Urgency terms, action verbs                                │
│  └── FeatureAgent: Technical accuracy with accessibility                  │
│                                                                             │
│  QUALITY AGENT                                                             │
│  ├── Evaluates all generated content                                       │
│  ├── Checks locale authenticity                                           │
│  ├── Validates SEO keyword usage                                          │
│  └── Provides EvaluationSignal feedback                                   │
│                                                                             │
│  LEARNING AGENT                                                            │
│  ├── Analyzes EvaluationSignal patterns                                   │
│  ├── Updates PromptArtifact with learnings                                │
│  ├── Identifies successful patterns                                       │
│  └── Propagates improvements across locales                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Sources

1. **Graphs Meet AI Agents** (arXiv 2506.18019v1, 2025) - Taxonomy of graph-agent integration
2. **Neo4j Graphiti Framework** (2024) - Temporal knowledge graphs for agent state
3. **LangGraph/AutoGen** (2024-2025) - Multi-agent workflow patterns
4. **Microsoft GraphRAG** (2024) - LLM + KG for retrieval-augmented generation
5. **Anthropic MCP** (2024) - Model Context Protocol for tool-using agents
6. **Zep/Graphiti** (2024) - Agent memory with 94.8% retrieval accuracy
7. **McKinsey AI Survey** (2025) - 23% organizations scaling agentic systems

---

## Methodology

- **Tools used**: Perplexity AI (web search), local file analysis
- **Sources analyzed**: 7 primary sources, 20+ referenced papers/implementations
- **Time period**: 2024-2025 implementations and research

---

## Confidence Level

**High** - NovaNet's architecture aligns exceptionally well with state-of-the-art patterns:
- Graph structure matches Graph RAG best practices
- Native generation (not translation) matches multilingual research findings
- MCP integration ready for tool-using agents
- Orchestrator-subagent pattern already documented in architecture

---

## Further Research Suggestions

1. **Temporal Knowledge Graphs**: Add timestamps to all relationships for Graphiti-style memory
2. **Self-Improvement Metrics**: Define evaluation criteria for EvaluationSignal
3. **Multi-Agent Communication**: Research LangGraph patterns for agent-to-agent coordination
4. **Prompt Optimization**: Investigate DSPy for automated prompt tuning
5. **Vector Embeddings**: Add Neo4j vector indexes for hybrid search on EntityContent
