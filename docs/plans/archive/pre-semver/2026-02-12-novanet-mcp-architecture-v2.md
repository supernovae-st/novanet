# NovaNet MCP Server Architecture v2

**Date**: 2026-02-12
**Version**: v2.1 (Post-Deep-Exploration)
**Status**: Ready for Phase 1 Implementation

---

## Executive Summary

After extensive research across 10+ parallel exploration agents including deep codebase analysis, this document synthesizes findings into a production-ready architecture. The architecture implements **RLM-on-KG** (Recursive Language Model on Knowledge Graph) patterns for native content generation across 200+ locales.

### Key Statistics Discovered (Actual Neo4j Queries)

```
NodeKind count:   60   (40 shared + 21 org)
ArcKind count:    114  (5 families)
Locale count:     200  (BCP-47 codes)
Entity count:     281  (semantic layer)
Expression count: 17,036  <- Rich locale knowledge!
```

---

## Research Synthesis

### Key Findings from 10 Agents

| Agent | Key Discovery |
|-------|---------------|
| **Claude Code Docs** | 7 production guides created, tower-mcp preferred over rmcp |
| **rmcp Context7** | Official SDK with `#[tool_router]` macro, 12,916 code snippets |
| **Rust Architect** | tower-mcp + deadpool + moka + tiktoken-rs stack |
| **RLM Patterns** | RLM-on-KG: 4-6x more evidence vs RAG, hop-by-hop traversal |
| **Knowledge Graph Crates** | sara-core, oxirag, shodh-memory (32μs 3-hop traversal) |
| **MCP Best Practices** | IBM ContextForge gateway, resource/tool/prompt trinity |
| **RLM-on-KG Research** | WordLift benchmark: +5.88 citation coverage vs RAG |
| **Self-Describing Graphs** | NovaNet's Kind/ArcKind pattern is industry-leading |
| **neo4rs Patterns** | Arc<Graph> + deadpool pooling, streaming queries |
| **Token Counting** | tiktoken-rs (exact) + tokenx-rs (fast estimate) hybrid |
| **Agentic Patterns** | ReAct + Orchestrator-Subagent + Memory architecture |

---

## Critical Discoveries from Deep Exploration

### 1. Composite Key Pattern

All localized content uses composite keys for uniqueness across locales:

```
entity:{entity_key}@{locale_key}
page:{page_key}@{locale_key}
block:{block_key}@{locale_key}

Examples:
  entity:qr-code-generator@fr-FR
  page:homepage@de-DE
  block:hero-section@ja-JP
```

**Query pattern**: `STARTS WITH 'entity:qr-code-generator@'` finds all locales for an entity.

### 2. llm_context Property Format

All entities have structured `llm_context` for semantic search:

```
USE: [when to use this entity]
TRIGGERS: [keywords that should activate this entity]
NOT: [disambiguation - when NOT to use]
```

**Semantic search query**:
```cypher
WITH $user_keywords AS keywords
MATCH (e:Entity)
WHERE e.llm_context IS NOT NULL
  AND any(kw IN keywords WHERE toLower(e.llm_context) CONTAINS toLower(kw))
WITH e, [kw IN keywords WHERE toLower(e.llm_context) CONTAINS toLower(kw)] AS matches
RETURN e.key, e.display_name, size(matches) AS relevance_score
ORDER BY relevance_score DESC
```

### 3. Spreading Activation Relationships

Key relationships for RLM traversal with `strength` property (0.0-1.0):

```cypher
MATCH (e:Entity)-[r:SIMILAR_TO|INCLUDES|ENABLES|REQUIRES]-(related:Entity)
WHERE r.strength >= 0.8
RETURN e.key, type(r), related.key, r.strength
ORDER BY r.strength DESC
```

### 4. Token Budget per Trait (from taxonomy.yaml)

```yaml
kind_retrieval_defaults:
  invariant:
    traversal_depth: 2
    context_budget: 500
    token_estimate: 100
  localized:
    traversal_depth: 1
    context_budget: 1000
    token_estimate: 250
  knowledge:
    traversal_depth: 3
    context_budget: 300
    token_estimate: 50
```

### 5. Reusable Code from tools/novanet/

#### Database Layer (src/db.rs)
```rust
pub struct Db {
    graph: Arc<Graph>,  // neo4rs connection pool
}

impl Db {
    pub async fn connect(uri: &str, user: &str, password: &str) -> Result<Self> {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(password)
            .max_connections(16)
            .build()?;
        let graph = Graph::connect(config).await?;
        Ok(Self { graph: Arc::new(graph) })
    }
}
```

#### Error Handling (src/error.rs)
```rust
#[derive(Error, Debug)]
pub enum NovaNetError {
    #[error("Connection failed to {uri}: {source}")]
    Connection { uri: String, source: neo4rs::Error },
    #[error("Query execution failed: {query}\n{source}")]
    Query { query: String, source: neo4rs::Error },
    #[error("Entity not found: {key}")]
    NotFound { key: String },
    #[error("Token budget exceeded: {used}/{budget}")]
    TokenBudgetExceeded { used: usize, budget: usize },
}
```

#### Cypher Query Builder (src/cypher.rs)
```rust
pub struct CypherStatement {
    pub cypher: String,
    pub params: Vec<(String, ParamValue)>,
}

impl CypherStatement {
    pub fn new(cypher: impl Into<String>) -> Self {
        Self { cypher: cypher.into(), params: Vec::new() }
    }

    pub fn param(mut self, name: impl Into<String>, value: impl Into<ParamValue>) -> Self {
        self.params.push((name.into(), value.into()));
        self
    }
}
```

---

## Implementation Gaps & Solutions

### Gap 1: rmcp Version

**Issue**: Research docs show 0.1 and 0.8 inconsistently
**Action**: Verify on crates.io before implementation
**Current**: rmcp = "0.1" with features = ["server", "macros", "transport-io"]

### Gap 2: neo4rs Row to JSON

**Issue**: `neo4rs::Row` doesn't implement `Serialize`
**Solution**: Manual column extraction

```rust
fn row_to_json(row: neo4rs::Row, columns: &[&str]) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for col in columns {
        if let Ok(val) = row.get::<neo4rs::BoltType>(col) {
            map.insert(col.to_string(), bolt_to_json(val));
        }
    }
    serde_json::Value::Object(map)
}
```

### Gap 3: Locale-Aware Token Estimation

**Issue**: Simple 1:4 char:token ratio wrong for CJK/Arabic
**Solution**: Script-based multiplier

```rust
pub fn estimate_tokens(text: &str) -> usize {
    let total_chars = text.chars().count();
    let cjk_count = text.chars().filter(|c| is_cjk(*c)).count();

    if cjk_count > total_chars / 2 {
        (total_chars as f64 * 1.5) as usize  // CJK adjustment
    } else {
        text.len() / 4  // Default
    }
}
```

### Gap 4: Write Operation Validation

**Issue**: Simple keyword check can be bypassed
**Solution**: Regex word-boundary matching

```rust
fn validate_read_only(cypher: &str) -> Result<(), Error> {
    let normalized = cypher.to_uppercase();
    let forbidden = [r"\bCREATE\b", r"\bDELETE\b", r"\bMERGE\b",
                     r"\bSET\b", r"\bREMOVE\b", r"\bDROP\b"];
    for pattern in forbidden {
        if regex::Regex::new(pattern).unwrap().is_match(&normalized) {
            return Err(Error::WriteNotAllowed(pattern.to_string()));
        }
    }
    Ok(())
}
```

---

## Architecture Decision: RLM-on-KG

### Why RLM-on-KG over GraphRAG?

| Aspect | RLM-on-KG | GraphRAG | NovaNet Fit |
|--------|-----------|----------|-------------|
| **Graph Source** | Pre-curated KG | LLM-extracted | ✅ NovaNet is pre-curated |
| **Query Process** | Multi-hop traversal | Community summaries | ✅ Hop-by-hop fits our schema |
| **Context Size** | ~200 bytes/hop | Large summaries | ✅ Token-efficient |
| **Citation** | URI-level provenance | Document-level | ✅ Entity-level citations |

### RLM-on-KG Flow for NovaNet

```
Question: "Génère une landing page menu QR code pour restaurant"
    │
    ▼
SEED SEARCH ─────────────────────────────────────────────────────┐
    │ novanet_search(triggers: ["menu", "restaurant", "qr-code"]) │
    │ Returns: [digital-menu, restaurant, qr-code-menu]           │
    ▼                                                             │
HOP 1: Entity (invariant) ───────────────────────────────────────┤
    │ ~200 bytes: key, display_name, llm_context                 │
    ▼                                                             │
HOP 2: EntityContent (fr-FR) ────────────────────────────────────┤ RECURSE
    │ ~500 bytes: localized description, definition              │ 5 hops
    ▼                                                             │
HOP 3: Related via INCLUDES/ENABLES ─────────────────────────────┤
    │ ~300 bytes: menu-item, allergen-info, price-display        │
    ▼                                                             │
HOP 4: Terms from TermSet ───────────────────────────────────────┤
    │ ~300 bytes: "carte", "menu digital", "QR code"             │
    ▼                                                             │
HOP 5: Expressions + CultureRef ─────────────────────────────────┘
    │ ~200 bytes: idioms, cultural context
    ▼
TOTAL: ~1.5KB (vs 50KB traditional RAG dump)
    │
    ▼
SYNTHESIS + GENERATION ──────────────────────────────────────────
    │ LLM generates native fr-FR content with citations
    ▼
OUTPUT with Entity URI provenance
```

---

## Technology Stack (Final)

### Crate Structure: Standalone (NOT Workspace)

> **Decision**: `tools/novanet-mcp/` is a **standalone crate**, not part of the `tools/novanet/` workspace.
> This allows independent versioning and frequent updates to the MCP server without affecting the CLI.

### Core Dependencies (Context7 Verified - 2026-02-12)

```toml
[package]
name = "novanet-mcp"
version = "0.1.0"
edition = "2024"
rust-version = "1.86"

[dependencies]
# MCP Protocol - Official Anthropic SDK
# v0.15 adds OAuth support, improved macros: #[tool_router], #[tool], #[tool_handler]
rmcp = { version = "0.15", features = ["server", "transport-io"] }

# Async Runtime
tokio = { version = "1.43", features = ["full"] }
futures = "0.3"

# Neo4j Driver
# v0.8 supports direct serde deserialization: let person: Person = row.get("p")?;
neo4rs = "0.8"

# Connection Pooling
deadpool = { version = "0.12", features = ["managed"] }

# Caching
# v0.12.13 provides async cache with TTL, concurrent access patterns
moka = { version = "0.12", features = ["future"] }

# Token Counting (Hybrid Strategy)
# v0.9.1 with singleton pattern: cl100k_base_singleton() for single BPE instance
tiktoken-rs = "0.9"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "0.8"  # JSON Schema generation for MCP tool params

# Error Handling
thiserror = "2"
anyhow = "1"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Configuration
dotenvy = "0.15"

# Regex for Cypher validation
regex = "1"

# ═══════════════════════════════════════════════════════════════════════════════
# PERFORMANCE OPTIMIZATION CRATES (Context7 Verified)
# ═══════════════════════════════════════════════════════════════════════════════

# Stack allocation for small collections (avoid heap for <8 elements)
smallvec = "1.15"

# 1.5-5x faster Mutex/RwLock than std
parking_lot = "0.12"

# Concurrent HashMap for cache/state sharing
dashmap = "6.1"

# Zero-copy byte buffers for efficient serialization
bytes = "1.11"

[dev-dependencies]
tokio-test = "0.4"
pretty_assertions = "1"
insta = { version = "1", features = ["yaml"] }

# Optional: mimalloc for production (30% faster allocations)
# [target.'cfg(not(target_env = "msvc"))'.dependencies]
# mimalloc = { version = "0.1", default-features = false }
```

### rmcp v0.15 Macro Patterns (Context7)

```rust
use rmcp::{tool_router, tool, tool_handler, ServerHandler};

#[tool_router]
impl MyHandler {
    #[tool(
        name = "novanet_query",
        description = "Execute read-only Cypher query"
    )]
    async fn query(&self, params: QueryParams) -> Result<QueryResult, Error> {
        // Implementation
    }
}

#[tool_handler]
impl ServerHandler for MyHandler {
    // Auto-implemented by macro
}
```

### neo4rs v0.8 Serde Integration (Context7)

```rust
#[derive(Debug, Deserialize)]
struct Entity {
    key: String,
    display_name: String,
    llm_context: Option<String>,
}

// Direct deserialization from Row
let entity: Entity = row.get("e")?;
```

### SDK Choice: rmcp vs tower-mcp

| Criteria | rmcp | tower-mcp |
|----------|------|-----------|
| Official | ✅ Anthropic | ❌ Community |
| Maturity | 0.8.0, 39/39 conformance | 0.6.0, emerging |
| Macro Support | `#[tool_router]` | Native Tower |
| Learning Curve | Lower | Higher (Tower knowledge) |
| Middleware | Custom | Native Tower layers |

**Decision**: Start with **rmcp** (official, simpler), migrate to tower-mcp if middleware needs grow.

---

## Project Structure (Final)

```
tools/novanet-mcp/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # Public API
│   ├── main.rs                   # Entry point
│   │
│   ├── server/
│   │   ├── mod.rs
│   │   ├── handler.rs            # rmcp ServerHandler impl
│   │   ├── config.rs             # Configuration
│   │   └── state.rs              # Shared state (AppState)
│   │
│   ├── tools/                    # MCP Tools (7 tools)
│   │   ├── mod.rs                # Tool registry
│   │   ├── search.rs             # novanet_search (trigger-based)
│   │   ├── traverse.rs           # novanet_traverse (hop-by-hop)
│   │   ├── assemble.rs           # novanet_assemble_context (LLM-ready)
│   │   ├── atoms.rs              # novanet_get_atoms (Terms, Expressions)
│   │   ├── describe.rs           # novanet_describe (self-describing)
│   │   ├── query.rs              # novanet_query (raw Cypher)
│   │   └── generate.rs           # novanet_generate (trigger generation)
│   │
│   ├── resources/                # MCP Resources (4 resources)
│   │   ├── mod.rs
│   │   ├── entity.rs             # entity://{key}
│   │   ├── kind.rs               # kind://{name}
│   │   ├── locale.rs             # locale://{key}
│   │   └── view.rs               # view://{id}
│   │
│   ├── prompts/                  # MCP Prompts (3 prompts)
│   │   ├── mod.rs
│   │   ├── cypher_query.rs       # Natural language → Cypher
│   │   ├── content_generation.rs # Block generation prompt
│   │   └── context_analysis.rs   # Graph analysis prompt
│   │
│   ├── neo4j/
│   │   ├── mod.rs
│   │   ├── pool.rs               # Deadpool + neo4rs
│   │   ├── queries.rs            # Cypher builders
│   │   └── streaming.rs          # Streaming query support
│   │
│   ├── tokens/
│   │   ├── mod.rs
│   │   ├── counter.rs            # Hybrid tiktoken + tokenx
│   │   ├── budget.rs             # TokenBudget management
│   │   └── chunker.rs            # Token-aware chunking
│   │
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── query_cache.rs        # Moka-based query cache
│   │   └── schema_cache.rs       # Schema metadata cache
│   │
│   ├── rlm/                      # RLM-on-KG patterns
│   │   ├── mod.rs
│   │   ├── hop_executor.rs       # Multi-hop traversal
│   │   ├── evidence_packet.rs    # ~200 byte evidence chunks
│   │   └── synthesis.rs          # Context synthesis
│   │
│   └── error.rs                  # Error types
│
└── tests/
    ├── integration/
    │   ├── tools_test.rs
    │   ├── rlm_test.rs
    │   └── streaming_test.rs
    └── fixtures/
```

---

## MCP Tools Specification (7 Tools)

### 1. `novanet_search` — Semantic Trigger Search

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Triggers to match against llm_context.TRIGGERS
    triggers: Vec<String>,
    /// Filter by EntityCategory
    category: Option<String>,
    /// Filter by realm
    realm: Option<String>,
    /// Include EntityContent for locale
    locale: Option<String>,
    /// Max results
    limit: Option<usize>,
}

// Returns entities ranked by trigger match count
// Cypher: WHERE ANY(t IN $triggers WHERE e.llm_context CONTAINS t)
```

### 2. `novanet_traverse` — RLM Hop-by-Hop Traversal

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TraverseParams {
    /// Starting entity key
    from: String,
    /// Relationship types to follow
    relations: Vec<String>,
    /// Max hops (1-5)
    depth: u8,
    /// Token budget for traversal
    token_budget: Option<usize>,
    /// Evidence packet size (~200 bytes)
    packet_size: Option<usize>,
}

// Returns evidence packets per hop with provenance
// Implements RLM-on-KG hop loop
```

### 3. `novanet_assemble_context` — LLM Context Assembly

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AssembleParams {
    /// Entity keys to include
    entity_keys: Vec<String>,
    /// Target locale
    locale: String,
    /// Content types to include
    include: IncludeConfig,
    /// Token budget
    token_budget: usize,
    /// Priority strategy
    priority: Option<String>, // "entities_first" | "atoms_first" | "balanced"
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct IncludeConfig {
    entity_content: bool,
    terms: bool,
    expressions: bool,
    patterns: bool,
    culture: bool,
    taboos: bool,
    relations: bool,
}

// Returns structured context optimized for LLM generation
// Implements priority-based selection with graceful degradation
```

### 4. `novanet_get_atoms` — Knowledge Atoms

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AtomsParams {
    locale: String,
    types: Vec<String>, // ["Term", "Expression", "Pattern", ...]
    domain: Option<String>,
    used_by: Option<String>, // entity key
    limit: Option<usize>,
}

// Returns locale-specific knowledge atoms
// Cypher: MATCH (l:Locale {key: $locale})-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)
```

### 5. `novanet_describe` — Self-Description

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DescribeParams {
    describe: String, // "schema" | "entity" | "category" | "relations" | "locales" | "stats"
    entity_key: Option<String>,
    category_key: Option<String>,
}

// Agent bootstrap: discover graph structure
// Returns schema, statistics, traversal hints
```

### 6. `novanet_query` — Raw Cypher (Read-Only)

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QueryParams {
    cypher: String,
    params: Option<serde_json::Map<String, Value>>,
    limit: Option<usize>,
}

// Direct Cypher access for advanced queries
// Validates read-only (no CREATE/DELETE/MERGE/SET)
```

### 7. `novanet_generate` — Trigger Generation Job

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GenerateParams {
    /// Block key to generate
    block_key: String,
    /// Target locale
    locale: String,
    /// Use specific PromptArtifact
    prompt_artifact: Option<String>,
    /// Override token budget
    token_budget: Option<usize>,
}

// Orchestrates full RLM-on-KG generation flow
// Returns generated BlockGenerated content with provenance
```

---

## MCP Resources (4 Resources)

| URI Pattern | Description | Example |
|-------------|-------------|---------|
| `entity://{key}` | Entity with content | `entity://qr-code` |
| `kind://{name}` | NodeKind schema | `kind://Entity` |
| `locale://{key}` | Locale config | `locale://fr-FR` |
| `view://{id}` | Predefined view query | `view://composition` |

---

## MCP Prompts (3 Prompts)

### 1. `cypher_query`
Natural language → Cypher translation with schema context.

### 2. `content_generation`
Block generation prompt template with locale atoms injection.

### 3. `context_analysis`
Graph structure analysis for understanding entity relationships.

---

## Token Counting Strategy (Hybrid)

```rust
pub struct HybridTokenCounter {
    exact: tiktoken_rs::CoreBPE,  // cl100k_base
}

impl HybridTokenCounter {
    /// Fast estimate for pre-flight checks (96% accuracy)
    pub fn estimate(&self, text: &str) -> usize {
        tokenx_rs::estimate_token_count(text)
    }

    /// Exact count for budget enforcement
    pub fn exact(&self, text: &str) -> usize {
        self.exact.encode_with_special_tokens(text).len()
    }

    /// Smart check: fast path when clearly within/outside budget
    pub fn within_budget(&self, text: &str, budget: usize) -> bool {
        let estimate = self.estimate(text);
        if estimate < budget * 90 / 100 { return true; }
        if estimate > budget * 110 / 100 { return false; }
        self.exact(text) <= budget
    }
}
```

---

## RLM-on-KG Implementation

### Evidence Packet Structure

```rust
/// ~200 byte evidence packet (per RLM-on-KG spec)
#[derive(Debug, Serialize)]
pub struct EvidencePacket {
    /// Source entity URI
    pub source_uri: String,
    /// Hop number (1-5)
    pub hop: u8,
    /// Relationship followed
    pub relation: String,
    /// Compressed content
    pub content: String,
    /// Token count
    pub tokens: usize,
}

impl EvidencePacket {
    pub const TARGET_SIZE: usize = 200;

    pub fn compress(content: &str, counter: &impl TokenCounter) -> Self {
        let tokens = counter.count(content);
        let compressed = if tokens > Self::TARGET_SIZE / 4 {
            counter.truncate_to_budget(content, Self::TARGET_SIZE / 4)
        } else {
            content.to_string()
        };
        // ...
    }
}
```

### Hop Executor

```rust
pub struct HopExecutor {
    pool: Arc<Neo4jPool>,
    counter: Arc<HybridTokenCounter>,
    max_hops: u8,
    token_budget: usize,
}

impl HopExecutor {
    pub async fn execute_rlm(
        &self,
        seed_entities: Vec<String>,
        relations: Vec<String>,
    ) -> Result<Vec<EvidencePacket>, Error> {
        let mut packets = Vec::new();
        let mut visited = HashSet::new();
        let mut frontier = seed_entities;
        let mut tokens_used = 0;

        for hop in 1..=self.max_hops {
            if frontier.is_empty() || tokens_used >= self.token_budget {
                break;
            }

            let mut next_frontier = Vec::new();

            for entity_key in frontier {
                if visited.contains(&entity_key) {
                    continue;
                }
                visited.insert(entity_key.clone());

                // Fetch entity data
                let data = self.fetch_entity(&entity_key).await?;
                let packet = EvidencePacket::compress(&data, &self.counter);

                tokens_used += packet.tokens;
                if tokens_used > self.token_budget {
                    break;
                }

                packets.push(packet);

                // Find next hop targets
                let neighbors = self.fetch_neighbors(&entity_key, &relations).await?;
                next_frontier.extend(neighbors);
            }

            frontier = next_frontier;
        }

        Ok(packets)
    }
}
```

---

## Agent Integration Patterns

### ReAct Loop with NovaNet MCP

```
Agent: "I need to generate a landing page for restaurant QR menus"

THOUGHT 1: I should search for relevant entities in NovaNet
ACTION 1: novanet_search(triggers: ["menu", "restaurant", "qr-code", "landing-page"])
OBSERVATION 1: Found [digital-menu, restaurant, qr-code-menu, link-in-bio]

THOUGHT 2: I should traverse from digital-menu to understand its structure
ACTION 2: novanet_traverse(from: "digital-menu", relations: ["INCLUDES", "ENABLES"], depth: 2)
OBSERVATION 2: Evidence packets: [menu-item, menu-category, allergen-info, price-display]

THOUGHT 3: I need locale-specific terms and expressions for fr-FR
ACTION 3: novanet_get_atoms(locale: "fr-FR", types: ["Term", "Expression"], domain: "restaurant")
OBSERVATION 3: Terms: [carte, menu digital, QR code], Expressions: [Scannez pour découvrir]

THOUGHT 4: Now I can assemble the full context for generation
ACTION 4: novanet_assemble_context(entity_keys: [...], locale: "fr-FR", token_budget: 6000)
OBSERVATION 4: Structured context with entities, atoms, relations

THOUGHT 5: Ready to generate native French content
ACTION 5: Generate content using assembled context
OUTPUT: Native fr-FR landing page with entity URI citations
```

### Orchestrator-Subagent Pattern

```
┌─────────────────────────────────────────────────────────────────┐
│  ORCHESTRATOR AGENT                                             │
│                                                                 │
│  1. Receive generation request (Page x Locale)                  │
│  2. novanet_describe("schema") → understand available entities  │
│  3. For each Block in Page:                                     │
│     └─ Dispatch to SUBAGENT(BlockType, Locale)                 │
│  4. Collect results, validate coherence                         │
│  5. Store via novanet_generate()                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────────────────────────────┐
│  SUBAGENT (per BlockType x Locale)                              │
│                                                                 │
│  1. novanet_traverse() → gather block-specific context          │
│  2. novanet_get_atoms() → load locale knowledge                 │
│  3. novanet_assemble_context() → build LLM input                │
│  4. Generate native content                                     │
│  5. Return with provenance                                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Configuration

### Environment Variables

```bash
# Neo4j
NOVANET_MCP_NEO4J_URI=bolt://localhost:7687
NOVANET_MCP_NEO4J_USER=neo4j
NOVANET_MCP_NEO4J_PASSWORD=novanetpassword
NOVANET_MCP_NEO4J_POOL_SIZE=16

# Cache
NOVANET_MCP_CACHE_MAX_ENTRIES=10000
NOVANET_MCP_CACHE_TTL=5m

# Token Budgets
NOVANET_MCP_DEFAULT_TOKEN_BUDGET=100000
NOVANET_MCP_MAX_HOPS=5
NOVANET_MCP_EVIDENCE_PACKET_SIZE=200

# Server
NOVANET_MCP_LOG_LEVEL=info
NOVANET_MCP_METRICS_PORT=9090
```

### Claude Code Integration

```json
// ~/.claude/settings.json
{
  "mcpServers": {
    "novanet": {
      "command": "/path/to/novanet-mcp",
      "env": {
        "NOVANET_MCP_NEO4J_PASSWORD": "novanetpassword"
      }
    }
  }
}
```

---

## Implementation Roadmap

### Phase 1: Foundation (v11.8) — 2 weeks

| Task | Description |
|------|-------------|
| Scaffold crate | Create `tools/novanet-mcp/` with structure |
| Neo4j pool | Implement deadpool + neo4rs wrapper |
| Token counter | Hybrid tiktoken + tokenx implementation |
| Basic server | rmcp stdio transport |
| First tool | `novanet_query` (raw Cypher) |
| Testing | Integration tests with testcontainers |

### Phase 2: Core Tools (v11.9) — 2 weeks

| Task | Description |
|------|-------------|
| `novanet_search` | Trigger-based search |
| `novanet_traverse` | RLM hop executor |
| `novanet_describe` | Self-description |
| `novanet_get_atoms` | Knowledge atoms |
| Resources | entity://, kind://, locale:// |
| Caching | Moka query cache |

### Phase 3: RLM Integration (v12.0) — 2 weeks

| Task | Description |
|------|-------------|
| `novanet_assemble_context` | Full context assembly |
| `novanet_generate` | Generation orchestration |
| Prompts | 3 MCP prompts |
| Evidence packets | ~200 byte compression |
| Provenance | Entity URI citations |
| Documentation | Full API docs |

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Query latency (p50) | < 50ms |
| Query latency (p99) | < 200ms |
| Context assembly | < 500ms |
| Token accuracy | 99%+ (exact mode) |
| Cache hit rate | > 80% |
| Evidence per generation | 10-20 packets |

---

## References

### Research Documents Created

1. `/docs/plans/2026-02-12-novanet-mcp-architecture.md` — Initial plan
2. `/docs/research/2026-02-12-rlm-on-kg-research.md` — RLM-on-KG findings
3. `/docs/research/2026-02-12-self-describing-knowledge-graphs.md` — Self-describing patterns
4. `/docs/research/2026-02-12-agentic-ai-patterns-knowledge-graphs.md` — Agentic patterns
5. `/docs/plans/README-MCP-RESEARCH-2026-02-12.md` — Claude Code MCP guide

### External References

- [WordLift RLM-on-KG](https://wordlift.io/blog/en/recursive-language-models-on-kg/)
- [rmcp crate](https://crates.io/crates/rmcp)
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs)
- [IBM ContextForge MCP Gateway](https://ibm.github.io/mcp-context-forge/)
- [Microsoft GraphRAG](https://github.com/microsoft/graphrag)

---

## DX Documentation Updates (Required)

> **IMPORTANT**: Upon completion of Phase 1, all documentation must be updated to reflect the new MCP server.

### Files to Update

| File | Update Required |
|------|-----------------|
| `/CLAUDE.md` | Add novanet-mcp overview, MCP tools reference |
| `/README.md` | Add MCP server section to monorepo structure |
| `/ROADMAP.md` | Mark v11.8 complete, add v12.0 RLM phase |
| `/CHANGELOG.md` | Document novanet-mcp release |
| `/.claude/rules/novanet-decisions.md` | Add ADR-023: MCP Server Architecture |
| `/.claude/rules/novanet-terminology.md` | Add MCP terms (RLM, evidence packet, hop) |

### Claude Code DX Files

| File | Update Required |
|------|-----------------|
| `/.claude/settings.json` | Add novanet MCP server config |
| `/.claude/mcp.json` | Create MCP server registration |
| `/.claude/plugins/` | Optional: novanet-mcp skill plugin |

### Skills to Create/Update

| Skill | Purpose |
|-------|---------|
| `/novanet-mcp` | Start/stop/status of MCP server |
| `/novanet-search` | Invoke novanet_search tool |
| `/novanet-traverse` | Invoke novanet_traverse tool |
| `/novanet-describe` | Invoke novanet_describe for schema info |

### tools/novanet-mcp/ Documentation

| File | Content |
|------|---------|
| `README.md` | Installation, usage, configuration |
| `CLAUDE.md` | Claude Code context for development |
| `docs/TOOLS.md` | Detailed tool specifications |
| `docs/RLM.md` | RLM-on-KG patterns explanation |

### Example Claude Code Config

```json
// /.claude/mcp.json (create this file)
{
  "mcpServers": {
    "novanet": {
      "command": "./tools/novanet-mcp/target/release/novanet-mcp",
      "args": [],
      "env": {
        "NOVANET_MCP_NEO4J_URI": "bolt://localhost:7687",
        "NOVANET_MCP_NEO4J_USER": "neo4j",
        "NOVANET_MCP_NEO4J_PASSWORD": "novanetpassword"
      }
    }
  }
}
```

### Example Skill Definition

```yaml
# /.claude/skills/novanet-mcp.yaml
name: novanet-mcp
description: NovaNet MCP Server management
commands:
  - name: start
    description: Start the MCP server
    run: |
      cd tools/novanet-mcp && cargo run --release
  - name: build
    description: Build the MCP server
    run: |
      cd tools/novanet-mcp && cargo build --release
  - name: test
    description: Run MCP server tests
    run: |
      cd tools/novanet-mcp && cargo test
```

---

## Crate Versions Summary (Context7 Verified)

| Crate | Version | Key Feature |
|-------|---------|-------------|
| **rmcp** | 0.15.0 | `#[tool_router]`, `#[tool]`, `#[tool_handler]` macros |
| **neo4rs** | 0.8.0 | Direct serde: `row.get::<Person>("p")` |
| **tiktoken-rs** | 0.9.1 | Singleton: `cl100k_base_singleton()` |
| **moka** | 0.12.13 | `Cache::builder().time_to_live().build()` |
| **smallvec** | 1.15.1 | Stack alloc for <8 elements |
| **parking_lot** | 0.12.5 | 1.5-5x faster Mutex/RwLock |
| **dashmap** | 6.1.0 | Concurrent HashMap |
| **bytes** | 1.11.1 | Zero-copy byte buffers |
| **deadpool** | 0.12.0 | Async connection pooling |
| **thiserror** | 2.0 | Error derive macros |
| **schemars** | 0.8 | JSON Schema for MCP tool params |
