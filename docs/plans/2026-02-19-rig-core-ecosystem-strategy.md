# Plan: Rig-Core Ecosystem Integration Strategy

**Date:** 2026-02-19
**Status:** Research Complete, Ready for Implementation
**Priority:** Medium (future enhancements after MVP 6)
**Estimated Effort:** Multiple sprints
**Last Updated:** 2026-02-19 (COMPLETE: 0xPlaygrounds + kg-node + MCP libraries + GraphRAG techniques + 25+ crates)

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

### MCP Schema Libraries (2026-02-19 Research)

**Deep dive into rust-mcp-schema and mcp-attr for Nika/NovaNet:**

#### rust-mcp-schema (v0.9.5)

| Aspect | Details |
|--------|---------|
| **Purpose** | Type-safe MCP protocol implementation |
| **Downloads** | 241K (high adoption) |
| **Protocol versions** | 5: 2025-11-25, 2025-06-18, 2025-03-26, 2024-11-05, draft |
| **Key feature** | Auto-generated from official MCP spec via TypeScript→Rust transpilation |

```rust
// rust-mcp-schema: Type-safe message handling
use rust_mcp_schema::{ClientMessage, ClientRequest, schema_utils};

pub fn handle_message(payload: &str) -> Result<(), RpcError> {
    let message = ClientMessage::from_str(payload)?;
    if let ClientMessage::Request(message_object) = message {
        if let ClientRequest::InitializeRequest(req) = message_object.request {
            handle_initialize_request(req);
        }
    }
    Ok(())
}
```

**Nika use case:** Could provide stronger type safety for MCP client responses, but rmcp already handles this via serde.

#### mcp-attr (v0.0.7)

| Aspect | Details |
|--------|---------|
| **Purpose** | Declarative MCP server building with attribute macros |
| **Downloads** | 6K |
| **Protocol versions** | 2025-03-26, 2024-11-05 |
| **Key feature** | `#[mcp_server]`, `#[tool]`, `#[prompt]`, `#[resource]` macros |

```rust
// mcp-attr: Declarative server definition
use mcp_attr::server::{mcp_server, McpServer, serve_stdio};

struct NovaNetServer {
    neo4j: Graph,
    embedding: Arc<TextEmbedding>,
}

#[mcp_server]
impl McpServer for NovaNetServer {
    /// Generate native content for an entity (AI tool description)
    #[tool]
    async fn novanet_generate(
        &self,
        focus_key: String,
        locale: String,
    ) -> Result<GenerateResponse> {
        // Implementation...
    }

    #[resource("novanet://entities/{key}")]
    async fn get_entity(&self, key: String) -> Result<String> {
        // Implementation...
    }

    #[prompt]
    async fn entity_prompt(&self) -> Result<&str> {
        Ok("Generate content for the given entity...")
    }
}
```

**NovaNet use case:** Cleaner than raw rmcp for tool definitions. Docstrings become AI tool descriptions automatically.

#### MCP Library Comparison

| Feature | rmcp (current) | mcp-attr | rust-mcp-schema |
|---------|----------------|----------|-----------------|
| **Server building** | `#[tool_router]` | `#[mcp_server]` | Manual |
| **Client building** | Yes (Service/Transport) | No | Message types only |
| **Type safety** | serde + schemars | serde + auto | Auto-generated types |
| **Resource support** | Yes | `#[resource]` | Message types only |
| **Prompt support** | Yes | `#[prompt]` | Message types only |
| **Maturity** | Official SDK | Early (0.0.7) | Mature (0.9.5) |

#### Recommendations

| Project | Current | Recommendation | Rationale |
|---------|---------|----------------|-----------|
| **Nika (MCP Client)** | rmcp | ⭐ Stay with rmcp | Working well, RmcpClientAdapter pattern is clean |
| **NovaNet (MCP Server)** | rmcp | 🔄 Evaluate mcp-attr for v0.15 | Cleaner tool definitions, but wait for maturity |

**Action items:**
- [ ] Keep rmcp as primary SDK for both projects
- [ ] Monitor mcp-attr for v0.1.0 release (consider for NovaNet refactor)
- [ ] rust-mcp-schema not needed (rmcp covers type safety)

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

### GraphRAG & Context Retrieval Techniques (2026-02-19 Research)

**Research for Nika/NovaNet knowledge graph context assembly:**

#### Core GraphRAG Patterns

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  GRAPHRAG RETRIEVAL PIPELINE                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. SEMANTIC SEARCH (Vector)                                                │
│     └─ Query → Embedding → Vector similarity → Top-K candidates             │
│                                                                             │
│  2. GRAPH EXPANSION (Traversal)                                             │
│     └─ Candidates → Multi-hop neighbors → Subgraph extraction              │
│                                                                             │
│  3. RANKING (PageRank/Relevance)                                            │
│     └─ Subgraph nodes → Score by centrality + query relevance              │
│                                                                             │
│  4. CONTEXT ASSEMBLY (Token-aware)                                          │
│     └─ Ranked nodes → Select within token budget → Structured prompt       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Two-Stage Retrieval Pattern

**Stage 1: Semantic Search (Vector)**
- Dense retrieval using embeddings (fastembed, rig embedding models)
- Returns top-K semantically similar nodes
- Fast initial filtering

**Stage 2: Graph Traversal (Structural)**
- Expand from seed nodes via relationships
- Multi-hop reasoning (2-3 hops typical)
- Capture interconnected knowledge

```rust
// Pseudo-code: Two-stage GraphRAG retrieval
pub struct GraphRAGRetriever {
    vector_index: VectorIndex,  // Neo4j vector index or external
    graph: Neo4jClient,
}

impl GraphRAGRetriever {
    async fn retrieve(&self, query: &str, budget: usize) -> Vec<Evidence> {
        // Stage 1: Vector search
        let embedding = self.embed(query).await?;
        let seeds = self.vector_index.search(embedding, 10).await?;

        // Stage 2: Graph expansion
        let subgraph = self.graph.traverse(
            seeds.iter().map(|n| n.id).collect(),
            TraverseParams {
                max_depth: 2,
                arc_families: vec!["semantic", "localization"],
                direction: Direction::Both,
            }
        ).await?;

        // Stage 3: Rank by PageRank + query relevance
        let ranked = self.rank_nodes(subgraph, query);

        // Stage 4: Token-aware selection
        self.select_within_budget(ranked, budget)
    }
}
```

#### Ontology-Grounded RAG (OG-RAG)

Research shows **ontology-grounded retrieval** outperforms naive RAG:

| Technique | Description | NovaNet Applicability |
|-----------|-------------|----------------------|
| **Hypergraph retrieval** | Group related facts into hyperedges | ⭐⭐⭐ Knowledge atoms (TermSet→Terms) |
| **Spreading activation** | Propagate relevance through graph | ⭐⭐⭐ `novanet_traverse` depth parameter |
| **SPARQL/Cypher queries** | Structured retrieval from ontology | ⭐⭐⭐ NovaNet's Query-First architecture |
| **Entity linking** | Map query terms to graph entities | ⭐⭐ `novanet_search` hybrid mode |

#### NovaNet-Specific Patterns

**Current implementation (novanet_generate):**
```
Entity → HAS_NATIVE → EntityNative (locale content)
       → BELONGS_TO → EntityCategory (classification)
       → RELATES_TO → Entity (semantic links)

Locale → HAS_TERMS → TermSet → CONTAINS_TERM → Term (knowledge atoms)
       → HAS_EXPRESSIONS → ExpressionSet → ...
```

**Recommended enhancements:**

1. **Vector index on EntityNative.llm_context**
   ```cypher
   CREATE VECTOR INDEX entity_native_embedding IF NOT EXISTS
   FOR (n:EntityNative) ON (n.embedding)
   OPTIONS {indexConfig: {`vector.dimensions`: 384, `vector.similarity_function`: 'cosine'}}
   ```

2. **Spreading activation via arc weights**
   ```yaml
   # arc-classes with retrieval_weight
   HAS_NATIVE:
     retrieval_weight: 1.0  # Always include
   RELATES_TO:
     retrieval_weight: 0.7  # Contextually relevant
   BELONGS_TO:
     retrieval_weight: 0.3  # Classification context
   ```

3. **Context build log (v0.14.0)**
   - NovaNet now returns `context_build_log` showing retrieval decisions
   - Enables debugging and tuning of traversal parameters

#### Multi-Hop Reasoning Patterns

| Hops | Use Case | Example Query |
|------|----------|---------------|
| 1-hop | Direct relations | "Get EntityNative for qr-code in fr-FR" |
| 2-hop | Contextual expansion | "Get Terms used by Blocks on this Page" |
| 3-hop | Deep reasoning | "Find related Entities via shared Categories" |

**NovaNet traversal example:**
```yaml
# Nika workflow: 2-hop context assembly
- invoke: novanet_traverse
  params:
    start_key: "qr-code"
    max_depth: 2
    arc_families: ["semantic", "localization"]
    target_kinds: ["EntityNative", "Term", "Expression"]
```

#### Implementation Priority for NovaNet

| Enhancement | Priority | Sprint | Impact |
|-------------|----------|--------|--------|
| Vector index on EntityNative | 🔴 High | v0.15 | Semantic search capability |
| Spreading activation weights | 🟡 Medium | v0.16 | Better context ranking |
| Subgraph extraction API | 🟡 Medium | v0.16 | LLM-ready context bundles |
| PageRank scoring | 🟢 Low | v0.17 | Entity importance ranking |

#### Research Sources

- **Microsoft GraphRAG**: Two-stage retrieval, community summarization
- **OG-RAG (Ontology-Grounded)**: Hypergraph retrieval, spreading activation
- **kg-node (0xPlaygrounds)**: fastembed + Neo4j prefiltered search pattern
- **Neo4j GenAI**: Native vector index + graph traversal integration

### Extended Crate Research (2026-02-19)

#### Structured Output & Schema

| Crate | Score | Snippets | Description |
|-------|-------|----------|-------------|
| **rstructor** | 88.2 | 92 | Structured LLM output via derive macros, JSON Schema generation, multi-provider |

```rust
use rstructor::{Instructor, LLMClient, OpenAIClient};

#[derive(Instructor, Serialize, Deserialize)]
#[llm(description = "Movie information")]
struct Movie {
    #[llm(description = "Title of the movie")]
    title: String,
    #[llm(description = "Year released", example = 2010)]
    release_year: u16,
}

let client = OpenAIClient::from_env()?;
let movie: Movie = client.materialize("Tell me about Inception").await?;
```

#### Agent Frameworks

| Crate | Score | Snippets | Description |
|-------|-------|----------|-------------|
| **strands-agents** | 82.9 | 6,749 | Model-driven agent SDK with MCP integration, multi-agent |

```python
# strands-agents MCP integration
from strands.tools.mcp import MCPClient
from strands import Agent

agent = Agent(tools=[mcp_client])
response = agent("What is AWS Lambda?")
```

#### LLM API Clients

| Crate | Score | Snippets | Description |
|-------|-------|----------|-------------|
| **async-openai** | 72.1 | 91 | Unofficial OpenAI Rust client, streaming, function calling |
| **ollama** (API) | 89.3 | 1,464 | Local LLM inference via Ollama API |

```rust
// async-openai function calling
use async_openai::{types::*, Client};

let request = CreateChatCompletionRequestArgs::default()
    .model("gpt-4o-mini")
    .messages([ChatCompletionRequestUserMessageArgs::default()
        .content("What's the weather?")
        .build()?])
    .functions([ChatCompletionFunctionsArgs::default()
        .name("get_weather")
        .description("Get current weather")
        .parameters(json!({"type": "object", "properties": {...}}))
        .build()?])
    .function_call("auto")
    .build()?;
```

#### Vector Databases

| Crate | Score | Snippets | Description |
|-------|-------|----------|-------------|
| **qdrant-client** | 87.7 | 40 | Official Qdrant Rust client, gRPC |
| **lancedb** | 90.1 | 585 | Embedded vector DB, multimodal |

```rust
// qdrant-client search with filters
use qdrant_client::{Qdrant, qdrant::*};

let search_result = client.search_points(
    SearchPointsBuilder::new("collection", vec![0.9, 0.1, 0.1], 10)
        .filter(Filter::all([
            Condition::matches("category", "tutorial".to_string()),
            Condition::range("views", Range { gte: Some(1000.0), ..Default::default() }),
        ]))
        .with_payload(true),
).await?;
```

#### Text Processing

| Crate | Score | Snippets | Description |
|-------|-------|----------|-------------|
| **tokenizers** | 80.6 | 208 | HuggingFace BPE tokenizers, fast |
| **text-splitter** | — | 174 | Semantic text chunking for RAG |
| **tiktoken** | 94.9 | 34 | OpenAI's BPE tokenizer |

```rust
// text-splitter for RAG chunking
use text_splitter::{MarkdownSplitter, CodeSplitter};

let splitter = MarkdownSplitter::new(1000);  // max chars
let chunks = splitter.chunks("# Header\n\nDocument text...");

// Code-aware splitting with tree-sitter
let code_splitter = CodeSplitter::new(tree_sitter_rust::LANGUAGE, 1000)?;
let chunks = code_splitter.chunks("fn main() { ... }");
```

#### Embeddings

| Crate | Score | Snippets | Description |
|-------|-------|----------|-------------|
| **fastembed** | 60.9 | 6 | Local ONNX embeddings, fast inference |
| **safetensors** | — | 68 | Safe model weight storage |

```rust
// fastembed for local embeddings (used by kg-node)
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

let model = TextEmbedding::try_new(
    InitOptions::new(EmbeddingModel::AllMiniLML6V2)
        .with_show_download_progress(true)
)?;

let embeddings = model.embed(vec!["Hello, World!"], None)?;
println!("Dimension: {}", embeddings[0].len());  // 384
```

---

### kg-node Deep Dive (0xPlaygrounds)

**Architecture discovered from source code analysis:**

```
kg-node/
├── mcp-server/src/
│   ├── main.rs         # SSE server + MCP handler
│   ├── lib.rs          # Tool implementations
│   └── input_types.rs  # Request schemas
├── grc20-core/         # Entity/Relation abstractions
├── api/                # REST API
└── sink/               # Data ingestion
```

**Key implementation patterns from kg-node main.rs:**

```rust
use rmcp::{tool, tool_router, RoleServer, ServerHandler};
use fastembed::{TextEmbedding, EmbeddingModel};
use neo4rs::Graph;

#[derive(Clone)]
pub struct KnowledgeGraph {
    neo4j: neo4rs::Graph,
    embedding_model: Arc<TextEmbedding>,
}

#[tool(tool_box)]
impl KnowledgeGraph {
    #[tool(description = include_str!("../resources/search_entity_description.md"))]
    async fn search_entity(
        &self,
        #[tool(param)]
        #[schemars(description = "Search query")]
        search_traversal_filter: SearchTraversalInputFilter,
    ) -> Result<CallToolResult, McpError> {
        // 1. Generate embedding from query
        let embedding = self.embedding_model
            .embed(vec![&search_traversal_filter.query], None)?;

        // 2. Pre-filtered semantic search
        let results = entity::prefiltered_search::<EntityNode>(&self.neo4j, embedding)
            .filter(filter)
            .limit(10)
            .send()
            .await?;

        Ok(CallToolResult::success(vec![Content::json(results)?]))
    }

    #[tool(description = "Get entity info by ID")]
    async fn get_entity_info(&self, id: String) -> Result<CallToolResult, McpError> {
        let attributes = triple::find_many(&self.neo4j)
            .entity_id(prop_filter::value(&id))
            .send().await?;

        let relations = relation::find_many::<RelationEdge<EntityNode>>(&self.neo4j)
            .filter(relation::RelationFilter::default().from_(EntityFilter::default().id(prop_filter::value(&id))))
            .send().await?;

        Ok(CallToolResult::success(vec![Content::json(json!({
            "id": id,
            "attributes": attributes,
            "relations": relations,
        }))?]))
    }
}

#[tool(tool_box)]
impl ServerHandler for KnowledgeGraph {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
            instructions: Some(include_str!("../resources/instructions.md").to_string()),
        }
    }
}
```

**kg-node MCP tools (discovered):**
- `search_types` — Search for type definitions
- `search_relation_types` — Search relation types
- `search_entity` — Semantic entity search with traversal filters
- `search_entity_using_ids` — Search using known IDs
- `get_entity_info` — Get entity details + relations
- `get_relations_between_entities` — Find paths between entities

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

## Cargo.toml Recommendations

### Nika (Workflow Engine / MCP Client)

```toml
[dependencies]
# ═══════════════════════════════════════════════════════════════════════════════
# TIER 1: CORE (already using or must have)
# ═══════════════════════════════════════════════════════════════════════════════
rig-core = "0.31"                  # LLM orchestration, 20+ providers
rmcp = "0.16"                      # MCP client/server SDK
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# ═══════════════════════════════════════════════════════════════════════════════
# TIER 2: HIGH VALUE (add for v0.4+)
# ═══════════════════════════════════════════════════════════════════════════════
# Text Processing for RAG
text-splitter = { version = "0.16", features = ["markdown", "code"] }
tokenizers = "0.20"                # HuggingFace BPE tokenizers

# Structured Output
rstructor = "0.2"                  # Derive macro for LLM structured output

# Vector Search (choose one based on deployment)
qdrant-client = "1.12"             # Production: managed Qdrant
# lancedb = "0.15"                 # Embedded: local vector DB

# ═══════════════════════════════════════════════════════════════════════════════
# TIER 3: NICE TO HAVE (evaluate for v0.5+)
# ═══════════════════════════════════════════════════════════════════════════════
# Local Embeddings (for offline/privacy)
fastembed = "5"                    # ONNX embeddings, no API calls

# Direct API Clients (if rig doesn't cover)
async-openai = "0.25"              # OpenAI streaming + function calling
# ollama-rs = "0.2"                # Local Ollama (rig covers this)

# Graph Processing (in-memory)
petgraph = "0.6"                   # BFS/DFS/PageRank for local graph ops

[dev-dependencies]
insta = { version = "1.41", features = ["yaml"] }
proptest = "1.5"
```

### NovaNet (Knowledge Graph / MCP Server)

```toml
[dependencies]
# ═══════════════════════════════════════════════════════════════════════════════
# TIER 1: CORE (must have for MCP server)
# ═══════════════════════════════════════════════════════════════════════════════
rmcp = "0.16"                      # MCP server SDK with #[tool] macro
neo4rs = "0.8"                     # Async Neo4j driver
tokio = { version = "1", features = ["full"] }
axum = "0.7"                       # HTTP/SSE server
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "0.8"                   # JSON Schema for tool params

# ═══════════════════════════════════════════════════════════════════════════════
# TIER 2: HIGH VALUE (add for semantic search)
# ═══════════════════════════════════════════════════════════════════════════════
# Embeddings (following kg-node pattern)
fastembed = "5"                    # Local ONNX embeddings
# rig-core = "0.31"                # If using rig for embeddings

# Text Processing
text-splitter = { version = "0.16", features = ["markdown"] }

# ═══════════════════════════════════════════════════════════════════════════════
# TIER 3: NICE TO HAVE
# ═══════════════════════════════════════════════════════════════════════════════
petgraph = "0.6"                   # In-memory graph for caching/analysis
safetensors = "0.4"                # If loading local models
```

### Crate Selection Matrix

| Need | Crate | Why |
|------|-------|-----|
| Multi-provider LLM | `rig-core` | 20+ providers, maintained, ToolDyn trait |
| MCP protocol | `rmcp` | Official SDK, `#[tool]` macro, async |
| Neo4j access | `neo4rs` | Async, production-ready (used by kg-node) |
| Local embeddings | `fastembed` | ONNX, no API, used by kg-node |
| Text chunking | `text-splitter` | Semantic boundaries, markdown/code aware |
| Tokenization | `tokenizers` | HuggingFace, BPE, fast |
| Structured output | `rstructor` | Derive macro, JSON Schema, validation |
| Vector search | `qdrant-client` | Production-ready, filtered search |
| Embedded vectors | `lancedb` | Zero-config, embedded, multimodal |
| OpenAI direct | `async-openai` | If rig doesn't cover specific features |
| Graph ops | `petgraph` | In-memory BFS/DFS/PageRank |

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

### Primary Sources
- **rig-core docs**: https://docs.rs/rig-core
- **rmcp docs**: https://docs.rs/rmcp
- **0xPlaygrounds/kg-node**: https://github.com/0xPlaygrounds/kg-node (MCP + Neo4j reference)

### Crate Documentation
| Crate | Docs | Repo |
|-------|------|------|
| rig-core | https://docs.rs/rig-core | https://github.com/0xPlaygrounds/rig |
| rmcp | https://docs.rs/rmcp | https://github.com/anthropics/rmcp |
| fastembed | https://docs.rs/fastembed | https://github.com/anush008/fastembed-rs |
| text-splitter | https://docs.rs/text-splitter | https://github.com/benbrandt/text-splitter |
| qdrant-client | https://docs.rs/qdrant-client | https://github.com/qdrant/rust-client |
| async-openai | https://docs.rs/async-openai | https://github.com/64bit/async-openai |
| tokenizers | https://docs.rs/tokenizers | https://github.com/huggingface/tokenizers |
| petgraph | https://docs.rs/petgraph | https://github.com/petgraph/petgraph |

### Project Files
- NovaNet MCP: `novanet-dev/tools/novanet-mcp/`
- Nika provider: `nika-dev/tools/nika/src/provider/rig.rs`

### Research Tools Used
- **Context7**: Library documentation search (28,000+ snippets accessed)
- **Firecrawl**: GitHub repository scraping
- **Perplexity**: Web search for crate discovery

---

## Appendix: Research Summary

### Searches Performed
1. **0xPlaygrounds exploration** — 62 repositories analyzed
2. **Perplexity searches** (6):
   - LLM orchestration crates
   - Knowledge graph RAG
   - MCP protocol implementations
   - AI agent memory
   - Rig alternatives
   - GraphRAG context retrieval Neo4j techniques
3. **Context7 queries** (14+):
   - rig-core, rmcp, petgraph, async-openai, qdrant-client
   - lancedb, tokenizers, text-splitter, fastembed, safetensors
   - rstructor, strands-agents
   - modelcontextprotocol/rust-sdk (rmcp patterns)
4. **Firecrawl scrapes** (2):
   - crates.io/crates/rust-mcp-schema — Type-safe MCP protocol (v0.9.5, 241K downloads)
   - crates.io/crates/mcp-attr — Declarative MCP server (v0.0.7, 6K downloads)
5. **Source code analysis**:
   - kg-node/mcp-server/src/main.rs — Complete MCP server pattern
   - novanet-mcp/src/main.rs — Current NovaNet MCP implementation
   - nika/src/mcp/client.rs — Nika MCP client pattern
   - nika/src/mcp/rmcp_adapter.rs — rmcp SDK wrapper

### Key Discoveries
1. **kg-node** provides production MCP server pattern with rmcp + neo4rs + fastembed
2. **rstructor** enables structured LLM output via derive macros
3. **text-splitter** offers semantic chunking for RAG pipelines
4. **fastembed** provides local ONNX embeddings (no API calls)
5. **rig-core** already covers 20+ providers and 10+ vector stores
6. **mcp-attr** (v0.0.7) offers cleaner server definitions than raw rmcp — monitor for v0.1.0
7. **rust-mcp-schema** (v0.9.5) provides type-safe protocol types — rmcp already covers this
8. **GraphRAG** two-stage retrieval pattern aligns perfectly with NovaNet's architecture
9. **OG-RAG** ontology-grounded retrieval validates NovaNet's knowledge atoms design
