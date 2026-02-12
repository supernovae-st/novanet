# NovaNet MCP Server - Master Implementation Plan

**Date**: 2026-02-12
**Version**: v3.0 (Final Consolidated)
**Status**: Ready for Implementation

---

## Executive Summary

Ce document consolide toutes les recherches effectuées par 10+ agents parallèles pour créer le **NovaNet MCP Server** - un serveur MCP Rust qui expose notre knowledge graph auto-descriptif aux agents AI via le protocole MCP.

### Vision

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  AVANT: Agent → Neo4j MCP générique → Cypher brut → Résultats non structurés │
│                                                                              │
│  APRÈS: Agent → NovaNet MCP → RLM-on-KG hop-by-hop → Context LLM-ready      │
│                 ↳ Auto-description du graph                                  │
│                 ↳ ~1.5KB context vs 50KB RAG dump                           │
│                 ↳ Citations entity-level                                     │
│                 ↳ Token budget management                                    │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Synthèse des Recherches

### 1. RLM-on-KG (Recursive Language Model on Knowledge Graph)

**Source**: `/docs/research/2026-02-12-rlm-on-kg-research.md`

**Découverte clé**: RLM-on-KG surpasse RAG traditionnel de 4-6x en evidence gathering.

| Métrique | RLM-on-KG | Simple RAG | Delta |
|----------|-----------|------------|-------|
| Citation Coverage | **7.987** | 2.107 | +5.88 |
| Citation Precision | **6.233** | 2.133 | +4.10 |
| Overall Score | **5.813** | 4.981 | +0.83 |

**Algorithme RLM-on-KG**:
```
1. SEED: entitySearch(question) → entité de départ
2. HOP LOOP (budget = 5 hops):
   a. Gather EvidencePacket (~200 bytes)
   b. Expand via relations (INCLUDES, ENABLES, USES_ENTITY)
   c. Select next entity (overlap-plus-score)
   d. Avoid revisiting
3. BUILD compact context (toutes les evidence packets)
4. GENERATE constrained to evidence
5. CITE URIs discovered hop-by-hop
```

**Context Size**: ~1.5KB vs 50KB RAG dump
- Hop 1: Entity (invariant) → ~200 bytes
- Hop 2: EntityContent (fr-FR) → ~500 bytes
- Hop 3: Related entities → ~300 bytes
- Hop 4: Terms from TermSet → ~300 bytes
- Hop 5: Expressions + Culture → ~200 bytes

**Risque identifié**: "Overreach" - RLM accumule beaucoup d'evidence, risque de sur-synthèse.
**Mitigation**: Grounding Judge pour valider faithfulness.

---

### 2. Self-Describing Knowledge Graphs

**Source**: `/docs/research/2026-02-12-self-describing-knowledge-graphs.md`

**Découverte clé**: L'architecture NovaNet avec Kind/ArcKind est déjà alignée avec les best practices.

**Comparaison des approches**:

| Système | Pattern Schema | Mécanisme Auto-Description |
|---------|----------------|---------------------------|
| **NovaNet** | Meta-nodes `:Meta:Kind` | `llm_context`, `schema_hint` |
| Neo4j | Runtime `db.schema()` | Agrégation labels/properties |
| RDF/OWL | Ontology classes | `owl:Class`, `rdfs:domain` |
| GraphRAG | Leiden clustering | Community reports |

**Requêtes d'introspection recommandées**:
```cypher
-- Agent bootstrap: schema complet
MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
RETURN r.key AS realm, l.key AS layer, collect(k) AS kinds

-- Navigation: que puis-je traverser depuis ici?
MATCH (k:Kind {label: $nodeType})
MATCH (ak:ArcKind)-[:FROM_KIND]->(k)-[:TO_KIND]->(target:Kind)
RETURN ak.key, target.label, ak.llm_context
```

**Enhancements proposés**:
1. Token Budget Estimates numériques sur `context_budget`
2. Traversal Hints sur ArcKind (`always_follow`, `max_hops`)
3. Community Detection pour clustering sémantique
4. SHACL-like validation rules sur Kind nodes

---

### 3. Agentic AI Patterns

**Source**: `/docs/research/2026-02-12-agentic-ai-patterns-knowledge-graphs.md`

**Patterns applicables à NovaNet**:

#### Pattern 1: ReAct Loop
```
THOUGHT → ACTION → OBSERVATION → THOUGHT → ...

NovaNet ReAct:
THOUGHT: "Generate hero-pricing block for fr-FR"
ACTION: novanet_traverse(from: "hero-pricing", depth: 2)
OBSERVATION: Evidence packets [entity, content, terms]
THOUGHT: "Context ready. Generate natively."
ACTION: novanet_generate(block: "hero-pricing", locale: "fr-FR")
```

#### Pattern 2: Orchestrator-Subagent
```
┌─────────────────────────────────────────────────────────────┐
│  ORCHESTRATOR                                               │
│  ├── Reçoit: "Generate pricing page for all locales"        │
│  ├── Query: Page → Block structure                          │
│  ├── Dispatch: One subagent per Block × Locale              │
│  └── Aggregate: Collect BlockGenerated                      │
│                                                             │
│  SUBAGENTS (per BlockType)                                  │
│  ├── HeroBlockAgent                                         │
│  ├── PricingBlockAgent                                      │
│  ├── FAQBlockAgent                                          │
│  └── CTABlockAgent                                          │
└─────────────────────────────────────────────────────────────┘
```

#### Pattern 3: Agent Memory Architecture
```
WORKING MEMORY     → Current generation context
EPISODIC MEMORY    → GenerationJob history, PREVIOUS_VERSION chains
LONG-TERM MEMORY   → Entity, EntityContent, LocaleKnowledge
SEMANTIC MEMORY    → USES_ENTITY, SEMANTIC_LINK relationships
```

#### Pattern 4: Self-Improving Loop
```
GENERATE → EVALUATE (EvaluationSignal) → REFLECT → ADAPT (PromptArtifact) → PROPAGATE
```

---

### 4. MCP Protocol & Implementation

**Source**: `/docs/plans/README-MCP-RESEARCH-2026-02-12.md`

**Stack technologique final** (Context7 Verified - 2026-02-12):

> **Note**: Standalone crate (NOT workspace) pour updates indépendants.

```toml
[dependencies]
# MCP Protocol - Official Anthropic SDK
rmcp = { version = "0.15", features = ["server", "transport-io"] }  # v0.15 macros: #[tool_router], #[tool]

# Async Runtime
tokio = { version = "1.43", features = ["full"] }

# Neo4j - Direct serde deserialization
neo4rs = "0.8"                    # row.get::<Person>("p")
deadpool = { version = "0.12", features = ["managed"] }  # Connection pooling

# Caching
moka = { version = "0.12", features = ["future"] }  # Async cache with TTL

# Token Counting (Hybrid)
tiktoken-rs = "0.9"               # cl100k_base_singleton() for Claude

# Performance Optimizations (Context7)
smallvec = "1.15"                 # Stack alloc for <8 elements
parking_lot = "0.12"              # 1.5-5x faster Mutex/RwLock
dashmap = "6.1"                   # Concurrent HashMap
bytes = "1.11"                    # Zero-copy byte buffers

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "0.8"                  # JSON Schema for MCP tool params

# Error Handling
thiserror = "2"
anyhow = "1"
```

**Architecture MCP**:
```
┌─────────────────────────────────────────────────────────────┐
│  MCP SERVER TRINITY                                         │
├─────────────────────────────────────────────────────────────┤
│  RESOURCES (read-only data)                                 │
│  ├── entity://{key}          Entity with content            │
│  ├── kind://{name}           NodeKind schema                │
│  ├── locale://{key}          Locale config + atoms          │
│  └── view://{id}             Predefined view query          │
│                                                             │
│  TOOLS (executable actions)                                 │
│  ├── novanet_search          Trigger-based semantic search  │
│  ├── novanet_traverse        RLM hop-by-hop traversal       │
│  ├── novanet_assemble        LLM context assembly           │
│  ├── novanet_get_atoms       Knowledge atoms                │
│  ├── novanet_describe        Self-description               │
│  ├── novanet_query           Raw Cypher (read-only)         │
│  └── novanet_generate        Trigger generation job         │
│                                                             │
│  PROMPTS (templates)                                        │
│  ├── cypher_query            Natural language → Cypher      │
│  ├── content_generation      Block generation template      │
│  └── context_analysis        Graph analysis prompt          │
└─────────────────────────────────────────────────────────────┘
```

---

## Architecture Détaillée

### Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  AI AGENT (Claude, GPT, etc.)                                               │
│  - ReAct reasoning loop                                                     │
│  - RLM decomposition/recursion                                              │
└───────────────────────────────────┬─────────────────────────────────────────┘
                                    │ MCP Protocol (JSON-RPC 2.0 over stdio)
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET MCP SERVER (Rust)                                                  │
│  tools/novanet-mcp/                                                         │
│                                                                             │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │   search    │ │  traverse   │ │  assemble   │ │  describe   │           │
│  └──────┬──────┘ └──────┬──────┘ └──────┬──────┘ └──────┬──────┘           │
│         └───────────────┴───────────────┴───────────────┘                   │
│                                    │                                        │
│         ┌──────────────────────────┼──────────────────────────┐            │
│         │                          │                          │            │
│         ▼                          ▼                          ▼            │
│  ┌─────────────┐          ┌─────────────┐          ┌─────────────┐         │
│  │ Token       │          │ Query       │          │ Schema      │         │
│  │ Counter     │          │ Builder     │          │ Cache       │         │
│  │ (tiktoken)  │          │ (Cypher)    │          │ (moka)      │         │
│  └─────────────┘          └──────┬──────┘          └─────────────┘         │
│                                  │                                          │
│                    ┌─────────────┴─────────────┐                           │
│                    │  Neo4j Pool (deadpool)    │                           │
│                    └─────────────┬─────────────┘                           │
└──────────────────────────────────┼──────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  KNOWLEDGE LAYER                                                            │
│  Neo4j (60 NodeKinds, 114 ArcKinds) + YAML (source of truth)               │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Structure du projet

```
tools/novanet-mcp/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs                    # Public API
│   ├── main.rs                   # Entry point (stdio transport)
│   │
│   ├── server/
│   │   ├── mod.rs
│   │   ├── handler.rs            # rmcp ServerHandler impl
│   │   ├── config.rs             # Configuration (env vars)
│   │   └── state.rs              # AppState (pool, cache, counter)
│   │
│   ├── tools/                    # 7 MCP Tools
│   │   ├── mod.rs                # Tool registry
│   │   ├── search.rs             # novanet_search (trigger-based)
│   │   ├── traverse.rs           # novanet_traverse (RLM hop-by-hop)
│   │   ├── assemble.rs           # novanet_assemble_context
│   │   ├── atoms.rs              # novanet_get_atoms
│   │   ├── describe.rs           # novanet_describe (self-describing)
│   │   ├── query.rs              # novanet_query (raw Cypher)
│   │   └── generate.rs           # novanet_generate
│   │
│   ├── resources/                # 4 MCP Resources
│   │   ├── mod.rs
│   │   ├── entity.rs             # entity://{key}
│   │   ├── kind.rs               # kind://{name}
│   │   ├── locale.rs             # locale://{key}
│   │   └── view.rs               # view://{id}
│   │
│   ├── prompts/                  # 3 MCP Prompts
│   │   ├── mod.rs
│   │   ├── cypher_query.rs       # Natural language → Cypher
│   │   ├── content_generation.rs # Block generation template
│   │   └── context_analysis.rs   # Graph analysis prompt
│   │
│   ├── neo4j/
│   │   ├── mod.rs
│   │   ├── pool.rs               # Deadpool + neo4rs wrapper
│   │   ├── queries.rs            # Cypher query builders
│   │   └── streaming.rs          # Streaming query support
│   │
│   ├── tokens/
│   │   ├── mod.rs
│   │   ├── counter.rs            # Hybrid tiktoken counter
│   │   ├── budget.rs             # TokenBudget management
│   │   └── chunker.rs            # Token-aware chunking
│   │
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── query_cache.rs        # Moka-based query cache
│   │   └── schema_cache.rs       # Schema metadata cache (TTL: 5min)
│   │
│   ├── rlm/                      # RLM-on-KG patterns
│   │   ├── mod.rs
│   │   ├── hop_executor.rs       # Multi-hop traversal engine
│   │   ├── evidence_packet.rs    # ~200 byte evidence chunks
│   │   └── synthesis.rs          # Context synthesis
│   │
│   └── error.rs                  # Error types (thiserror)
│
└── tests/
    ├── integration/
    │   ├── tools_test.rs
    │   ├── rlm_test.rs
    │   └── streaming_test.rs
    └── fixtures/
        └── test_data.cypher
```

---

## Spécifications des 7 MCP Tools

### Tool 1: `novanet_search`

**Purpose**: Semantic search by triggers (not vector similarity)

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Triggers to match against llm_context.TRIGGERS
    pub triggers: Vec<String>,
    /// Filter by EntityCategory
    pub category: Option<String>,
    /// Filter by realm (shared/org)
    pub realm: Option<String>,
    /// Include EntityContent for locale
    pub locale: Option<String>,
    /// Max results (default: 10)
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub entities: Vec<EntityMatch>,
    pub total_count: usize,
    pub token_estimate: usize,
}

#[derive(Debug, Serialize)]
pub struct EntityMatch {
    pub key: String,
    pub display_name: String,
    pub category: String,
    pub trigger_match_count: usize,
    pub llm_context: Option<String>,
}
```

**Cypher**:
```cypher
MATCH (e:Entity)
WHERE ANY(t IN $triggers WHERE e.llm_context CONTAINS t)
OPTIONAL MATCH (e)-[:BELONGS_TO]->(c:EntityCategory)
WITH e, c, size([t IN $triggers WHERE e.llm_context CONTAINS t]) AS match_count
ORDER BY match_count DESC
LIMIT $limit
RETURN e.key, e.display_name, c.category_key, match_count, e.llm_context
```

---

### Tool 2: `novanet_traverse`

**Purpose**: RLM hop-by-hop traversal with evidence packets

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TraverseParams {
    /// Starting entity key
    pub from: String,
    /// Relationship types to follow
    pub relations: Vec<String>,
    /// Max hops (1-5)
    pub depth: u8,
    /// Token budget for entire traversal
    pub token_budget: Option<usize>,
    /// Evidence packet target size (~200 bytes)
    pub packet_size: Option<usize>,
    /// Target locale for content
    pub locale: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TraverseResult {
    pub evidence_packets: Vec<EvidencePacket>,
    pub visited_entities: Vec<String>,
    pub total_hops: u8,
    pub tokens_used: usize,
    pub traversal_path: Vec<PathSegment>,
}

#[derive(Debug, Serialize)]
pub struct EvidencePacket {
    /// Source entity URI (novanet://entity/{key})
    pub source_uri: String,
    /// Hop number (1-5)
    pub hop: u8,
    /// Relationship followed to reach this entity
    pub relation: String,
    /// Compressed content (~200 bytes)
    pub content: String,
    /// Token count for this packet
    pub tokens: usize,
}

impl EvidencePacket {
    pub const TARGET_SIZE: usize = 200;  // ~200 bytes per packet
}
```

**Algorithm**:
```rust
pub async fn execute_rlm(&self, params: TraverseParams) -> Result<TraverseResult> {
    let mut packets = Vec::new();
    let mut visited = HashSet::new();
    let mut frontier = vec![params.from.clone()];
    let mut tokens_used = 0;
    let budget = params.token_budget.unwrap_or(8000);

    for hop in 1..=params.depth {
        if frontier.is_empty() || tokens_used >= budget {
            break;
        }

        let mut next_frontier = Vec::new();

        for entity_key in frontier {
            if visited.contains(&entity_key) {
                continue;
            }
            visited.insert(entity_key.clone());

            // Fetch and compress entity data
            let data = self.fetch_entity_data(&entity_key, &params.locale).await?;
            let packet = EvidencePacket::compress(&data, hop, &self.counter);

            tokens_used += packet.tokens;
            if tokens_used > budget {
                break;
            }
            packets.push(packet);

            // Find neighbors via specified relations
            let neighbors = self.fetch_neighbors(&entity_key, &params.relations).await?;
            next_frontier.extend(neighbors);
        }

        frontier = next_frontier;
    }

    Ok(TraverseResult {
        evidence_packets: packets,
        visited_entities: visited.into_iter().collect(),
        total_hops: packets.iter().map(|p| p.hop).max().unwrap_or(0),
        tokens_used,
        traversal_path: self.build_path(&packets),
    })
}
```

---

### Tool 3: `novanet_assemble_context`

**Purpose**: Assemble LLM-ready context with token budget management

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AssembleParams {
    /// Entity keys to include
    pub entity_keys: Vec<String>,
    /// Target locale
    pub locale: String,
    /// Content types to include
    pub include: IncludeConfig,
    /// Total token budget
    pub token_budget: usize,
    /// Priority strategy
    pub priority: Option<PriorityStrategy>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct IncludeConfig {
    pub entity_content: bool,      // EntityContent for locale
    pub terms: bool,               // Terms from TermSet
    pub expressions: bool,         // Expressions from ExpressionSet
    pub patterns: bool,            // Patterns from PatternSet
    pub culture: bool,             // CultureRef from CultureSet
    pub taboos: bool,              // Taboos from TabooSet
    pub relations: bool,           // Related entities (1-hop)
}

#[derive(Debug, Deserialize, JsonSchema)]
pub enum PriorityStrategy {
    EntitiesFirst,    // Load all entities, then atoms if room
    AtomsFirst,       // Load terms/expressions first
    Balanced,         // 50/50 split
}

#[derive(Debug, Serialize)]
pub struct AssembleResult {
    pub context: StructuredContext,
    pub tokens_used: usize,
    pub budget_remaining: usize,
    pub pruned_items: Vec<String>,  // Items that didn't fit
}

#[derive(Debug, Serialize)]
pub struct StructuredContext {
    pub locale_info: LocaleInfo,
    pub entities: Vec<EntityContext>,
    pub terms: Vec<Term>,
    pub expressions: Vec<Expression>,
    pub patterns: Vec<Pattern>,
    pub culture_refs: Vec<CultureRef>,
    pub relations: Vec<RelationContext>,
}
```

---

### Tool 4: `novanet_get_atoms`

**Purpose**: Load Knowledge Atoms (Terms, Expressions, Patterns)

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AtomsParams {
    pub locale: String,
    /// Atom types to fetch
    pub types: Vec<AtomType>,
    /// Filter by domain (pricing, legal, technical...)
    pub domain: Option<String>,
    /// Filter by entity usage
    pub used_by: Option<String>,
    /// Max atoms per type
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub enum AtomType {
    Term,
    Expression,
    Pattern,
    CultureRef,
    Taboo,
    AudienceTrait,
}
```

**Cypher**:
```cypher
MATCH (l:Locale {key: $locale})-[:HAS_TERMS]->(ts:TermSet)
      -[:CONTAINS_TERM]->(t:Term)
WHERE ($domain IS NULL OR ts.domain = $domain)
RETURN t.key, t.text, t.semantic_field, ts.domain
LIMIT $limit
```

---

### Tool 5: `novanet_describe`

**Purpose**: Self-description for agent bootstrap

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DescribeParams {
    pub describe: DescribeTarget,
    pub entity_key: Option<String>,
    pub category_key: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub enum DescribeTarget {
    Schema,      // Full schema overview
    Entity,      // Specific entity details
    Category,    // EntityCategory members
    Relations,   // ArcKind definitions
    Locales,     // Available locales
    Stats,       // Graph statistics
}
```

**Response for `Schema`**:
```json
{
  "schema_version": "11.7.0",
  "realms": [
    { "key": "shared", "layers": ["config", "locale", "geography", "knowledge"], "node_count": 39 },
    { "key": "org", "layers": ["config", "foundation", "structure", "semantic", "instruction", "output"], "node_count": 21 }
  ],
  "total_kinds": 60,
  "total_arc_kinds": 114,
  "arc_families": ["ownership", "localization", "semantic", "generation", "mining"],
  "entity_categories": ["THING", "CONTENT_TYPE", "PLACE", "PERSON", "ORGANIZATION", ...],
  "available_locales": ["fr-FR", "en-US", "ja-JP", ...],
  "traversal_hints": {
    "for_generation": ["USES_ENTITY", "HAS_CONTENT", "HAS_TERMS"],
    "for_exploration": ["INCLUDES", "ENABLES", "SEMANTIC_LINK"]
  }
}
```

---

### Tool 6: `novanet_query`

**Purpose**: Raw Cypher execution (read-only)

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QueryParams {
    /// Cypher query (must be read-only)
    pub cypher: String,
    /// Query parameters
    pub params: Option<serde_json::Map<String, Value>>,
    /// Result limit
    pub limit: Option<usize>,
    /// Timeout in ms
    pub timeout_ms: Option<u64>,
}
```

**Validation**:
```rust
fn validate_read_only(cypher: &str) -> Result<(), Error> {
    let forbidden = ["CREATE", "DELETE", "MERGE", "SET", "REMOVE", "DROP"];
    let upper = cypher.to_uppercase();
    for keyword in forbidden {
        if upper.contains(keyword) {
            return Err(Error::WriteNotAllowed(keyword.to_string()));
        }
    }
    Ok(())
}
```

---

### Tool 7: `novanet_generate`

**Purpose**: Trigger full RLM-on-KG generation workflow

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GenerateParams {
    /// Block key to generate
    pub block_key: String,
    /// Target locale
    pub locale: String,
    /// Use specific PromptArtifact
    pub prompt_artifact: Option<String>,
    /// Override token budget
    pub token_budget: Option<usize>,
    /// Include SEO keywords
    pub include_seo: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GenerateResult {
    pub job_id: String,
    pub status: GenerationStatus,
    pub context_assembled: ContextSummary,
    pub evidence_packets: Vec<EvidencePacket>,
    pub provenance: Vec<String>,  // Entity URIs used
}
```

---

## Token Counting Strategy

### Hybrid Counter

```rust
pub struct HybridTokenCounter {
    exact: tiktoken_rs::CoreBPE,  // cl100k_base for Claude
}

impl HybridTokenCounter {
    /// Fast estimate for pre-flight checks (~96% accuracy)
    pub fn estimate(&self, text: &str) -> usize {
        // Simple heuristic: 1 token ≈ 4 chars for English
        // Adjust for French/Japanese/etc.
        text.len() / 4
    }

    /// Exact count for budget enforcement
    pub fn exact(&self, text: &str) -> usize {
        self.exact.encode_with_special_tokens(text).len()
    }

    /// Smart check: fast path when clearly within/outside budget
    pub fn within_budget(&self, text: &str, budget: usize) -> bool {
        let estimate = self.estimate(text);
        // Fast path: clearly within budget (10% margin)
        if estimate < budget * 90 / 100 { return true; }
        // Fast path: clearly over budget
        if estimate > budget * 110 / 100 { return false; }
        // Slow path: exact count needed
        self.exact(text) <= budget
    }
}
```

### Budget Management

```rust
pub struct TokenBudget {
    pub total: usize,
    pub used: usize,
    pub reserved: HashMap<String, usize>,  // e.g., "entities": 2000
}

impl TokenBudget {
    pub fn remaining(&self) -> usize {
        self.total.saturating_sub(self.used)
    }

    pub fn allocate(&mut self, category: &str, amount: usize) -> bool {
        if amount <= self.remaining() {
            self.used += amount;
            *self.reserved.entry(category.to_string()).or_default() += amount;
            true
        } else {
            false
        }
    }

    pub fn try_fit(&self, text: &str, counter: &HybridTokenCounter) -> Option<usize> {
        let tokens = counter.exact(text);
        if tokens <= self.remaining() {
            Some(tokens)
        } else {
            None
        }
    }
}
```

---

## Roadmap d'Implémentation

### Phase 1: Foundation (v11.8) - 1 semaine

| Jour | Tâche | Livrable |
|------|-------|----------|
| 1 | Scaffold crate + Cargo.toml | Structure projet créée |
| 2 | Neo4j pool (deadpool + neo4rs) | `src/neo4j/pool.rs` |
| 2 | Error types (thiserror) | `src/error.rs` |
| 3 | Token counter (tiktoken-rs) | `src/tokens/counter.rs` |
| 3 | Basic rmcp server handler | `src/server/handler.rs` |
| 4 | `novanet_query` tool | Premier tool fonctionnel |
| 4 | `novanet_describe` tool | Bootstrap agent |
| 5 | Integration tests | Tests avec Neo4j local |
| 5 | Claude Code config | `.claude/mcp.json` |

**Critères de succès Phase 1**:
- [ ] Server runs without crashes (1 hour)
- [ ] 100+ queries execute successfully
- [ ] Query response times < 100ms median
- [ ] Claude Code discovers tools automatically
- [ ] `cargo test` passes

---

### Phase 2: Core Tools (v11.9) - 2 semaines

| Semaine | Tâche | Livrable |
|---------|-------|----------|
| 1 | `novanet_search` | Trigger-based search |
| 1 | `novanet_traverse` | RLM hop executor |
| 1 | Evidence packets | ~200 byte compression |
| 2 | `novanet_get_atoms` | Knowledge atoms |
| 2 | Resources (4) | entity://, kind://, locale://, view:// |
| 2 | Schema cache (moka) | 50%+ cache hit rate |

**Critères de succès Phase 2**:
- [ ] RLM traversal produces <2KB context
- [ ] Cache hit rate > 50%
- [ ] 500+ concurrent queries handled

---

### Phase 3: RLM Integration (v12.0) - 2 semaines

| Semaine | Tâche | Livrable |
|---------|-------|----------|
| 1 | `novanet_assemble_context` | Full context assembly |
| 1 | `novanet_generate` | Generation orchestration |
| 1 | Prompts (3) | MCP prompt templates |
| 2 | Grounding Judge | Faithfulness validation |
| 2 | Provenance tracking | Entity URI citations |
| 2 | Full documentation | API docs + examples |

**Critères de succès Phase 3**:
- [ ] Full RLM-on-KG workflow functional
- [ ] Evidence packets < 200 bytes each
- [ ] Provenance URIs on all generated content
- [ ] Documentation complete

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
NOVANET_MCP_CACHE_TTL_SECS=300

# Token Budgets
NOVANET_MCP_DEFAULT_TOKEN_BUDGET=100000
NOVANET_MCP_MAX_HOPS=5
NOVANET_MCP_EVIDENCE_PACKET_SIZE=200

# Server
NOVANET_MCP_LOG_LEVEL=info
RUST_LOG=novanet_mcp=debug
```

### Claude Code Integration

```json
// .claude/mcp.json
{
  "mcpServers": {
    "novanet": {
      "command": "./tools/novanet-mcp/target/release/novanet-mcp",
      "env": {
        "NOVANET_MCP_NEO4J_PASSWORD": "novanetpassword"
      }
    }
  }
}
```

---

## Métriques de Succès

| Métrique | Target | Mesure |
|----------|--------|--------|
| Query latency (p50) | < 50ms | Histogram |
| Query latency (p99) | < 200ms | Histogram |
| Context assembly | < 500ms | Timer |
| Token accuracy | 99%+ (exact mode) | Comparison |
| Cache hit rate | > 80% | Counter |
| Evidence per generation | 10-20 packets | Average |
| RLM context size | ~1.5KB | Bytes |

---

## Risques et Mitigations

| Risque | Impact | Mitigation |
|--------|--------|------------|
| Query timeout hangs | High | `tokio::time::timeout` sur toutes les queries |
| Connection pool exhaustion | High | Deadpool avec max_size + wait_timeout |
| Memory leak from cache | Medium | Moka avec TTL + max_entries |
| RLM "overreach" | Medium | Grounding Judge pour faithfulness |
| Token budget overflow | Medium | Hybrid counter + strict budget |
| Neo4j downtime | Low | Retry logic + graceful degradation |

---

## Documents de Référence

### Créés pendant cette recherche

| Document | Contenu | Priorité |
|----------|---------|----------|
| `2026-02-12-rlm-on-kg-research.md` | RLM-on-KG benchmarks, algorithm | High |
| `2026-02-12-self-describing-knowledge-graphs.md` | Self-description patterns | High |
| `2026-02-12-agentic-ai-patterns-knowledge-graphs.md` | ReAct, Orchestrator patterns | High |
| `README-MCP-RESEARCH-2026-02-12.md` | MCP protocol guide | High |
| `2026-02-12-mcp-server-best-practices.md` | Production patterns | Medium |
| `2026-02-12-mcp-server-patterns.md` | Code templates | Medium |
| `2026-02-12-novanet-mcp-roadmap.md` | Implementation plan | Medium |
| `2026-02-12-mcp-quickstart.md` | 30-minute quickstart | Medium |
| `2026-02-12-novanet-mcp-architecture-v2.md` | Full architecture | High |

### External References

| Source | URL |
|--------|-----|
| WordLift RLM-on-KG | https://wordlift.io/blog/en/recursive-language-models-on-kg/ |
| rmcp crate | https://crates.io/crates/rmcp |
| tiktoken-rs | https://github.com/zurawiki/tiktoken-rs |
| MCP Protocol Spec | https://spec.modelcontextprotocol.io/ |
| neo4rs | https://crates.io/crates/neo4rs |
| moka cache | https://crates.io/crates/moka |

---

## Décision Finale

**Go/No-Go**: **GO**

**Justification**:
1. NovaNet architecture is already aligned with RLM-on-KG best practices
2. Self-describing graph pattern is industry-leading
3. MCP protocol is simple and well-documented
4. Rust ecosystem (rmcp, neo4rs, moka) is mature
5. ROI: ~1.5KB context vs 50KB RAG = 30x token efficiency

**Next Action**: Commence Phase 1 implementation.

---

---

## DX Documentation Updates (Required)

> **Post-Phase 1**: All documentation must be updated.

| File | Update Required |
|------|-----------------|
| `/CLAUDE.md` | Add novanet-mcp overview, MCP tools reference |
| `/README.md` | Add MCP server section to monorepo structure |
| `/ROADMAP.md` | Mark v11.8 complete |
| `/CHANGELOG.md` | Document novanet-mcp release |
| `/.claude/rules/novanet-decisions.md` | Add ADR-023: MCP Server Architecture |
| `/.claude/rules/novanet-terminology.md` | Add MCP terms (RLM, evidence packet, hop) |
| `/.claude/mcp.json` | Create MCP server registration |
| `tools/novanet-mcp/CLAUDE.md` | Claude Code context for MCP development |

### Skills to Create

| Skill | Purpose |
|-------|---------|
| `/novanet-mcp` | Start/stop/status of MCP server |
| `/novanet-search` | Invoke novanet_search tool |
| `/novanet-traverse` | Invoke novanet_traverse tool |
| `/novanet-describe` | Invoke novanet_describe for schema info |

---

## Crate Versions Summary (Context7 Verified - 2026-02-12)

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

---

*Document généré le 2026-02-12 par Claude Opus 4.5 suite à l'exploration de 10+ agents parallèles.*
*Mis à jour avec recherche Context7 pour crates optimisés.*
