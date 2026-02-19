# Plan: Rig-Core Ecosystem Integration Strategy

**Date:** 2026-02-19
**Status:** Research Complete, Ready for Implementation
**Priority:** Medium (future enhancements after MVP 6)
**Estimated Effort:** Multiple sprints
**Last Updated:** 2026-02-19 (added 0xPlaygrounds research + Context7 + Perplexity findings)

---

## Executive Summary

Research shows **rig-core v0.31+** already covers most multi-provider needs. The ecosystem has complementary crates for specialized use cases. This plan outlines how to maximize rig-core and selectively integrate other crates.

**Key Discovery:** 0xPlaygrounds (rig creators) has 62 repositories including:
- **kg-node** — Knowledge graph indexer with MCP server (Neo4j backend)
- **rig-onchain-kit** — Multi-tenant AI agents for blockchain
- **rig-agent-state-machine-example** — State machine pattern for agents
- **rig-rag-system-example** — Complete RAG implementation

---

## Research Findings

### rig-core v0.31+ Capabilities

| Feature | Coverage | Status |
|---------|----------|--------|
| Multi-provider | 20+ providers | ✅ Already available |
| Vector stores | 10+ stores | ✅ Available |
| Tool calling | Tool/ToolDyn traits | ✅ We use NikaMcpTool |
| RAG | dynamic_context | ✅ Available |
| Streaming | StreamResponse | ✅ Available |
| Embeddings | Embedding model trait | ✅ Available |

**Providers (20+):**
- Cloud: OpenAI, Anthropic, Gemini, Cohere, AWS Bedrock, Azure, Groq
- Local: Ollama, vLLM, llama.cpp
- Specialized: Deepgram (audio), Voyage (embeddings)

**Vector Stores (10+):**
- MongoDB, Qdrant, LanceDB, Neo4j, Pinecone, Milvus, Chroma, PostgreSQL (pgvector), SQLite

### Complementary Crates

| Crate | Purpose | Rig Overlap | Recommendation |
|-------|---------|-------------|----------------|
| **graphrag-core** | GraphRAG with PageRank | Partial (RAG) | ⭐ Integrate for graph-enhanced retrieval |
| **ai-agents** | YAML-defined agents + memory | Partial (agents) | 🔄 Evaluate for Nika v0.4 agent improvements |
| **oxirs-graphrag** | Graph indexing | High (RAG) | ⏳ Monitor, possibly superseded |
| **rllm** | Local model inference | Covered by Ollama | ❌ Skip, use rig+Ollama |
| **candle** | Tensor ops | Low-level | ❌ Skip, too low-level |
| **mistral-rs** | Mistral inference | Covered by Ollama | ❌ Skip |

### 0xPlaygrounds Ecosystem (rig creators)

**62 repositories** — Key ones for Nika/NovaNet:

| Repo | Stars | Purpose | Relevance |
|------|-------|---------|-----------|
| **rig** | 6k | Main LLM framework | ⭐⭐⭐ Core dependency |
| **kg-node** | 10 | Knowledge graph + MCP server (Neo4j) | ⭐⭐⭐ Study for NovaNet MCP patterns |
| **rig-onchain-kit** | 61 | Multi-tenant AI agents | ⭐⭐ SignerContext pattern |
| **rig-rag-system-example** | 12 | Complete RAG with vector store | ⭐⭐ Reference implementation |
| **rig-agent-state-machine-example** | 5 | Agent state machine | ⭐⭐ Pattern for agent: loop |
| **posthog-rs** | 0 | Analytics SDK | ⭐ Observability pattern |

### MCP SDK Options (Rust)

| Crate | Version | Features | Recommendation |
|-------|---------|----------|----------------|
| **rmcp** (official) | 0.16+ | Full MCP 2025-11-25, async-first, `#[tool_router]` macro | ⭐⭐⭐ Primary |
| **tower-mcp** | new | Tower middleware, HTTP/WebSocket/stdio | ⭐⭐ For HTTP transport |
| **rust-mcp-sdk** | 0.1 | Alternative SDK | ⭐ Backup option |

### Memory & State Crates

| Crate | Purpose | Use Case |
|-------|---------|----------|
| **OpenViking** | Context database for AI agents | Conversation history, state |
| **SurrealDB** | Multi-model DB with Rust SDK | Persistent agent memory |
| **ZeroClaw** | Lightweight agent runtime | Pluggable memory backends |

### Graph Libraries

| Crate | Snippets | Use Case |
|-------|----------|----------|
| **petgraph** | 11,578 | In-memory graph ops (BFS, DFS, PageRank) |
| **Memgraph** | 8,435 | Neo4j-compatible streaming graph DB |
| **KnowGraph** | 296 | MCP server for code graph analysis |

---

---

## Code Examples from Research

### rig-core: RAG Agent with Vector Store

```rust
use rig::{
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
    providers::openai,
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStore},
};

// Initialize and build embeddings
let openai = openai::Client::from_env();
let embedding_model = openai.embedding_model(openai::TEXT_EMBEDDING_ADA_002);

let mut vector_store = InMemoryVectorStore::default();
let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
    .simple_document("doc0", "Definition of *flurbo*: A green alien")
    .build().await?;

vector_store.add_documents(embeddings).await?;
let index = vector_store.index(embedding_model);

// Build agent with dynamic context (RAG)
let agent = openai.agent(openai::GPT_4O)
    .preamble("You are a dictionary assistant.")
    .dynamic_context(1, index)  // ← RAG magic
    .build();

let response = agent.prompt("What is a flurbo?").await?;
```

### rmcp: MCP Client (what Nika uses)

```rust
use rmcp::{model::CallToolRequestParam, service::ServiceExt, transport::TokioChildProcess};
use tokio::process::Command;

let service = ().serve(TokioChildProcess::new(
    Command::new("novanet-mcp")
)?).await?;

// List and call tools
let tools = service.list_tools(Default::default()).await?;
let result = service.call_tool(CallToolRequestParam {
    name: "novanet_generate".into(),
    arguments: serde_json::json!({
        "focus_key": "qr-code",
        "locale": "fr-FR"
    }).as_object().cloned(),
}).await?;
```

### rmcp: MCP Server with `#[tool_router]`

```rust
use rmcp::{tool, tool_router, handler::server::tool::ToolRouter};

#[derive(Clone)]
pub struct NovaNetTools {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl NovaNetTools {
    #[tool(description = "Generate native content for an entity")]
    async fn novanet_generate(
        &self,
        params: Parameters<GenerateRequest>
    ) -> Result<Json<GenerateResponse>, String> {
        // Implementation...
    }
}
```

### petgraph: Graph Traversal

```rust
use petgraph::Graph;
use petgraph::visit::{Bfs, Dfs};

let mut graph = Graph::<String, ()>::new();
let a = graph.add_node("Entity".to_string());
let b = graph.add_node("EntityNative".to_string());
graph.add_edge(a, b, ());

// BFS traversal
let mut bfs = Bfs::new(&graph, a);
while let Some(nx) = bfs.next(&graph) {
    println!("Visited: {}", graph[nx]);
}
```

### rig-onchain-kit: SignerContext Pattern

```rust
use rig_onchain_kit::signer::SignerContext;

// Thread-local signer for multi-tenant agents
SignerContext::with_signer(Arc::new(signer), async {
    let agent = create_agent();
    let response = agent.prompt("Execute task").await?;
}).await;
```

---

## Strategic Integration Plan

### Phase 1: Maximize Rig-Core (Current Sprint)

**Goal:** Use everything rig provides before adding external crates.

#### 1.1 Complete RigProvider Integration

Current status: NikaMcpTool wrapper works with ToolDyn.

```rust
// Already implemented in provider/rig.rs
pub struct NikaMcpTool {
    def: NikaMcpToolDef,
    client: Arc<McpClient>,
}

impl ToolDyn for NikaMcpTool {
    async fn call(&self, args: String) -> Result<String, ToolError>;
}
```

Next steps:
- [ ] Add all 20+ providers to RigProvider enum
- [ ] Create provider factory with resilience wrappers
- [ ] Test with Ollama for local models

#### 1.2 Enable Dynamic Context (RAG)

Use rig's `dynamic_context` for contextual prompts:

```rust
use rig::agent::Agent;
use rig::context::DynamicContext;

let context = DynamicContext::new()
    .with_tool(mcp_tools)  // NovaNet MCP tools
    .with_retriever(vector_retriever);  // For RAG

let agent = client
    .agent("claude-3-opus")
    .dynamic_context(context)
    .build();
```

#### 1.3 Add Vector Store Support

Rig supports Neo4j as vector store — perfect for NovaNet:

```rust
use rig::vector_store::neo4j::Neo4jVectorStore;

let store = Neo4jVectorStore::new(
    "bolt://localhost:7687",
    "neo4j",
    password,
    "EntityNative",  // Node label
    "embedding",     // Property
).await?;
```

---

### Phase 2: GraphRAG Enhancement (Sprint +1)

**Goal:** Add graph-enhanced retrieval using graphrag-core patterns.

#### 2.1 Why GraphRAG?

NovaNet's knowledge graph has rich relationships. Standard RAG ignores these. GraphRAG:

1. **Entity extraction:** Find entities in queries
2. **Graph expansion:** Traverse relationships for context
3. **PageRank scoring:** Prioritize important nodes
4. **Hybrid retrieval:** Combine vector + graph results

#### 2.2 Integration Approach

Don't replace rig — enhance it:

```rust
// Pseudo-code for hybrid retriever
struct GraphEnhancedRetriever {
    vector_store: RigVectorStore,
    graph_client: McpClient,  // NovaNet MCP
}

impl Retriever for GraphEnhancedRetriever {
    async fn retrieve(&self, query: &str) -> Vec<Document> {
        // 1. Extract entities from query
        let entities = self.extract_entities(query).await?;

        // 2. Expand via graph (NovaNet MCP)
        let graph_context = self.graph_client
            .call_tool("novanet_traverse", json!({
                "start": entities,
                "depth": 2,
                "arc_types": ["HAS_NATIVE", "RELATES_TO"]
            }))
            .await?;

        // 3. Vector search for semantic match
        let vector_results = self.vector_store
            .search(query, 10)
            .await?;

        // 4. Combine and rank
        self.hybrid_rank(graph_context, vector_results)
    }
}
```

#### 2.3 graphrag-core Features to Use

| Feature | Use Case |
|---------|----------|
| Entity extraction | Identify concepts in user queries |
| Graph construction | Already have (NovaNet) |
| PageRank scoring | Rank entity importance for context window |
| Hybrid retrieval | Combine vector + graph for better RAG |

---

### Phase 3: Memory & Persistence (Sprint +2)

**Goal:** Add conversation memory and state persistence using ai-agents patterns.

#### 3.1 Memory Architecture

ai-agents provides `CompactingMemory` — summarizes long conversations:

```rust
// Concept from ai-agents
pub struct CompactingMemory {
    short_term: Vec<Message>,      // Recent messages
    compressed: String,            // Summary of older messages
    token_budget: usize,           // Max tokens before compacting
}
```

Apply to Nika's agent: loop:

```rust
// In runtime/agent_loop.rs
pub struct AgentLoop {
    // Existing
    goal: String,
    tools: Vec<Box<dyn ToolDyn>>,
    provider: Arc<dyn LlmProvider>,

    // Add memory
    memory: CompactingMemory,
    state: AgentState,  // State machine for reasoning
}
```

#### 3.2 State Machine Reasoning

ai-agents uses state machines for agent control:

```
┌─────────┐    tool_call    ┌──────────────┐    success    ┌──────────┐
│ Thinking│ ─────────────► │ ToolExecuting│ ────────────► │ Thinking │
└─────────┘                 └──────────────┘               └──────────┘
     │                            │
     │ final_answer               │ error
     ▼                            ▼
┌─────────┐                 ┌─────────────┐
│ Complete│                 │ Error/Retry │
└─────────┘                 └─────────────┘
```

#### 3.3 Persistence Options

| Storage | Use Case |
|---------|----------|
| SQLite | Local development, single-user |
| Redis | Production, multi-instance |
| Neo4j | Graph-native (NovaNet integration) |

---

## Implementation Priorities

### Must Have (v0.3)

- [x] NikaMcpTool implementing ToolDyn
- [ ] RigProvider with resilience wrappers
- [ ] Multi-provider support (at least Claude + OpenAI + Ollama)

### Should Have (v0.4)

- [ ] Vector store integration (Neo4j via rig)
- [ ] Dynamic context with NovaNet retrieval
- [ ] Basic memory for agent: loops

### Nice to Have (v0.5+)

- [ ] GraphRAG hybrid retrieval
- [ ] CompactingMemory
- [ ] State machine agent control
- [ ] Multi-model fallback chains

---

## Cargo.toml Additions

```toml
# Phase 1: Already have
rig-core = "0.31"

# Phase 2: GraphRAG (when ready)
graphrag-core = "0.1"  # If stable

# Phase 3: Memory patterns
# Implement in-house, don't depend on ai-agents
# (ai-agents is framework, we want patterns)
```

---

## Code Organization

```
tools/nika/src/
├── provider/
│   ├── mod.rs
│   ├── rig.rs          # ← Current: RigProvider, NikaMcpTool
│   ├── factory.rs      # NEW: Provider factory with fallback
│   └── multi.rs        # NEW: Multi-provider orchestration
├── retrieval/          # NEW module for RAG
│   ├── mod.rs
│   ├── vector.rs       # Rig vector store wrapper
│   ├── graph.rs        # NovaNet graph retrieval
│   └── hybrid.rs       # Combined retrieval
├── memory/             # NEW module for agent memory
│   ├── mod.rs
│   ├── compacting.rs   # CompactingMemory implementation
│   └── storage.rs      # SQLite/Redis backends
└── runtime/
    └── agent_loop.rs   # Enhanced with memory + state
```

---

## Decision Matrix

| Question | Answer |
|----------|--------|
| Use rig or custom multi-provider? | **Rig** — 20+ providers, maintained |
| Use rig RAG or graphrag-core? | **Rig + custom graph enhancement** |
| Use ai-agents or custom agents? | **Custom** — implement patterns, not framework |
| Add vector store now? | **No** — focus on MCP first, add later |
| Add memory now? | **No** — basic agent loop sufficient for MVP |

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Provider coverage | 5+ providers | Count in RigProvider |
| Tool compatibility | 100% MCP tools | Integration tests pass |
| RAG quality | TBD | Human eval of responses |
| Memory efficiency | <50% token reduction | Compacting ratio |

---

## References

- rig-core docs: https://docs.rs/rig-core
- graphrag-core: https://crates.io/crates/graphrag-core
- ai-agents: https://lib.rs/crates/ai-agents
- NovaNet MCP: `novanet-dev/tools/novanet-mcp/`
- Nika provider: `nika-dev/tools/nika/src/provider/rig.rs`
