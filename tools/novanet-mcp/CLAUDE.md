# NovaNet MCP Server

MCP (Model Context Protocol) server exposing the NovaNet knowledge graph to AI agents.

**Version**: 0.3.0 | **Rust**: 1.86 | **Edition**: 2024 | **rmcp**: 0.15

---

## Overview

NovaNet MCP implements **RLM-on-KG** (Recursive Language Model on Knowledge Graph) patterns for efficient context assembly. AI agents can query the NovaNet knowledge graph for content generation, SEO analysis, and locale-aware operations.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET MCP ARCHITECTURE                                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Claude Code ──► stdio ──► NovaNet MCP Server ──► Neo4j (bolt://7687)      │
│                    │              │                                         │
│                    │              ├── novanet_query      (Cypher execution) │
│                    │              ├── novanet_describe   (Schema bootstrap) │
│                    │              ├── novanet_search     (Fulltext search)  │
│                    │              ├── novanet_traverse   (Graph traversal)  │
│                    │              ├── novanet_assemble   (Context assembly) │
│                    │              ├── novanet_atoms      (Knowledge atoms)  │
│                    │              └── novanet_generate   (RLM-on-KG context)│
│                    │                                                        │
│               MCP Protocol                                                  │
│               (JSON-RPC 2.0)                                                │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 1 (Complete)                                                         │
│  ├── Tools: novanet_query, novanet_describe                                 │
│  ├── State: Neo4j pool, Query cache, Token counter                          │
│  └── Error: Typed errors with MCP JSON-RPC mapping                          │
│                                                                             │
│  PHASE 2 (Complete)                                                         │
│  ├── Resources: entity://, kind://, locale://, view://                      │
│  └── Tools: search, traverse, assemble, atoms                               │
│                                                                             │
│  PHASE 3 (In Progress)                                                      │
│  ├── Tool: novanet_generate (block/page mode, context anchors)              │
│  ├── Prompts: cypher_query, cypher_explain, block_generation,               │
│  │            page_generation, entity_analysis, locale_briefing             │
│  └── Context Anchors: Cross-page references via REFERENCES_PAGE arc         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Quick Start

### Environment Variables

```bash
# Required
export NOVANET_MCP_NEO4J_PASSWORD="novanetpassword"

# Optional (with defaults)
export NOVANET_MCP_NEO4J_URI="bolt://localhost:7687"
export NOVANET_MCP_NEO4J_USER="neo4j"
export NOVANET_MCP_NEO4J_POOL_SIZE="16"
export NOVANET_MCP_CACHE_MAX_ENTRIES="10000"
export NOVANET_MCP_CACHE_TTL_SECS="300"
export NOVANET_MCP_DEFAULT_TOKEN_BUDGET="100000"
export NOVANET_MCP_MAX_HOPS="5"
export NOVANET_MCP_EVIDENCE_PACKET_SIZE="200"
```

### Build and Run

```bash
# Build
cargo build --release

# Run (for local testing with stdin/stdout)
cargo run

# Run with debug logging
RUST_LOG=novanet_mcp=debug cargo run
```

### Claude Code Integration

Add to `.claude/settings.json`:

```json
{
  "mcpServers": {
    "novanet": {
      "command": "/path/to/novanet-mcp/target/release/novanet-mcp",
      "env": {
        "NOVANET_MCP_NEO4J_PASSWORD": "novanetpassword"
      }
    }
  }
}
```

---

## Tools

### `novanet_query`

Execute read-only Cypher queries against Neo4j.

**Parameters:**
```json
{
  "cypher": "MATCH (n:Entity) RETURN n.key LIMIT 10",
  "params": { "key": "value" },
  "limit": 100,
  "timeout_ms": 30000
}
```

**Returns:**
```json
{
  "rows": [...],
  "row_count": 10,
  "token_estimate": 250,
  "cached": false,
  "execution_time_ms": 45
}
```

**Security:**
- Only read-only queries allowed (MATCH, RETURN, WITH, WHERE, ORDER BY, LIMIT)
- Blocked keywords: CREATE, DELETE, MERGE, SET, REMOVE, DROP, DETACH
- Automatic LIMIT injection if not present

### `novanet_describe`

Bootstrap agent understanding of the knowledge graph.

**Targets:**
| Target | Description |
|--------|-------------|
| `schema` | Full schema overview (realms, layers, kinds, arc families) |
| `entity` | Specific entity details (requires `entity_key`) |
| `category` | EntityCategory members (optional `category_key`) |
| `relations` | All ArcKind definitions |
| `locales` | Available locales |
| `stats` | Graph statistics |

**Example:**
```json
{
  "describe": "schema"
}
```

### `novanet_search`

Search the knowledge graph using fulltext or property search.

**Parameters:**
```json
{
  "query": "QR code",
  "mode": "hybrid",
  "kinds": ["Entity", "Page"],
  "realm": "org",
  "limit": 20
}
```

**Modes:**
- `fulltext` - Neo4j fulltext indexes
- `property` - Property-based search with partial matching
- `hybrid` - Fulltext first, property fallback

**Returns:**
```json
{
  "hits": [
    {
      "key": "qr-code-generator",
      "kind": "Entity",
      "score": 0.95,
      "matches": [{"property": "name", "value": "QR Code Generator"}],
      "properties": {...}
    }
  ],
  "total_hits": 15,
  "mode": "hybrid",
  "token_estimate": 500,
  "execution_time_ms": 12
}
```

### `novanet_traverse`

Traverse the graph from a starting node with configurable depth and filters.

**Parameters:**
```json
{
  "start_key": "homepage",
  "max_depth": 3,
  "direction": "outgoing",
  "arc_families": ["ownership", "semantic"],
  "target_kinds": ["Entity", "Block"],
  "limit": 50,
  "include_properties": true
}
```

**Direction:**
- `outgoing` - Follow outgoing arcs only
- `incoming` - Follow incoming arcs only
- `both` - Follow both directions

**Returns:**
```json
{
  "start": { "key": "homepage", "kind": "Page", "depth": 0, "path": [] },
  "nodes": [...],
  "arcs": [...],
  "max_depth_reached": 2,
  "limited": false,
  "token_estimate": 1200,
  "execution_time_ms": 45
}
```

### `novanet_assemble`

Assemble context for LLM generation with token budget management.

**Parameters:**
```json
{
  "focus_key": "homepage",
  "locale": "fr-FR",
  "token_budget": 50000,
  "strategy": "breadth",
  "include_entities": true,
  "include_knowledge": true,
  "include_structure": true,
  "max_depth": 3
}
```

**Strategies:**
- `breadth` - Breadth-first traversal (default)
- `depth` - Depth-first following ownership arcs
- `relevance` - Prioritize by relevance score
- `custom` - Custom traversal via arc families

**Returns:**
```json
{
  "focus": { "key": "homepage", "kind": "Page", "name": "Homepage" },
  "evidence": [
    {
      "source_key": "qr-code-generator",
      "source_kind": "Entity",
      "evidence_type": "definition",
      "distance": 1,
      "relevance": 0.9,
      "content": "QR Code Generator: Create custom QR codes...",
      "tokens": 45
    }
  ],
  "locale_context": {
    "locale_key": "fr-FR",
    "language": "French",
    "region": "France",
    "voice": "Professional, friendly"
  },
  "total_tokens": 12500,
  "budget_remaining": 37500,
  "nodes_visited": 25,
  "truncated": false,
  "execution_time_ms": 120
}
```

### `novanet_atoms`

Retrieve knowledge atoms for a specific locale.

**Parameters:**
```json
{
  "locale": "fr-FR",
  "atom_type": "term",
  "domain": "technical",
  "query": "QR",
  "limit": 50,
  "include_containers": true
}
```

**Atom Types:**
- `term` - Technical terms with definitions
- `expression` - Idiomatic expressions
- `pattern` - Text patterns/templates
- `cultureref` - Cultural references
- `taboo` - Cultural taboos to avoid
- `audiencetrait` - Audience characteristics
- `all` - All atom types

**Returns:**
```json
{
  "locale": "fr-FR",
  "atoms": [
    {
      "key": "qr-code",
      "atom_type": "Term",
      "value": "code QR",
      "domain": "technical",
      "properties": { "definition": "Code-barres bidimensionnel" },
      "container_key": "tech-terms-fr"
    }
  ],
  "containers": [
    {
      "key": "tech-terms-fr",
      "container_type": "TermSet",
      "domain": "technical",
      "atom_count": 150
    }
  ],
  "total_count": 25,
  "token_estimate": 800,
  "execution_time_ms": 35
}
```

### `novanet_generate`

Assemble complete generation context for block or page content. This is the composite tool that orchestrates traverse, assemble, and atoms for AI agents.

**Parameters:**
```json
{
  "focus_key": "homepage",
  "locale": "fr-FR",
  "mode": "page",
  "token_budget": 50000,
  "include_examples": true,
  "spreading_depth": 2
}
```

**Modes:**
- `block` - Single block generation (entities, knowledge atoms)
- `page` - Full page orchestration (structure, all blocks, cross-references)

**Returns:**
```json
{
  "prompt": "# Generation Context for homepage (fr-FR)...",
  "evidence_summary": [
    {
      "source_key": "qr-code",
      "evidence_type": "entity",
      "relevance": 0.95,
      "tokens": 120
    }
  ],
  "locale_context": {
    "locale_key": "fr-FR",
    "language": "French",
    "voice": "professionnel, accessible",
    "formality": "vous"
  },
  "context_anchors": [
    {
      "page_key": "pricing",
      "anchor_text": "page de tarifs",
      "slug": "/fr/tarifs",
      "context_hint": "Link when mentioning pricing or plans"
    }
  ],
  "token_usage": {
    "structure": 500,
    "entities": 3200,
    "knowledge": 1800,
    "locale": 400,
    "total": 5900,
    "budget_remaining": 44100
  },
  "metadata": {
    "blocks_discovered": 5,
    "entities_loaded": 12,
    "execution_time_ms": 250
  }
}
```

**Context Anchors:**

Context Anchors enable cross-page references in generated content. When a Block references another Page via the `REFERENCES_PAGE` arc, the anchor metadata is included for creating internal links.

Anchor syntax in generated content: `{{anchor:page_key|display text}}`

Example: `"Découvrez notre {{anchor:pricing|page de tarifs}} pour..."` resolves to a proper link.

---

## Resources

NovaNet MCP provides read-only access to knowledge graph data via URI-based resources.

### `entity://{key}`

Fetch an entity with its localized content and relationships.

```
entity://qr-code-generator
```

**Returns:**
```json
{
  "key": "qr-code-generator",
  "name": "QR Code Generator",
  "definition": "Application for creating custom QR codes",
  "category": "product",
  "content": [
    { "locale": "fr-FR", "name": "Générateur de codes QR", "description": "..." },
    { "locale": "en-US", "name": "QR Code Generator", "description": "..." }
  ],
  "related": [
    { "key": "qr-code", "name": "QR Code", "relationship": "USES_ENTITY", "direction": "outgoing" }
  ]
}
```

### `kind://{name}`

Fetch a NodeKind definition from the meta-graph.

```
kind://Entity
```

**Returns:**
```json
{
  "name": "Entity",
  "display_name": "Entity",
  "realm": "org",
  "layer": "semantic",
  "trait_type": "invariant",
  "description": "Core semantic entity",
  "llm_context": "Entities represent invariant concepts...",
  "properties": [...],
  "outgoing_arcs": ["HAS_CONTENT", "USES_ENTITY"],
  "incoming_arcs": ["BELONGS_TO"],
  "instance_count": 150
}
```

### `locale://{key}`

Fetch locale configuration and knowledge summary.

```
locale://fr-FR
```

**Returns:**
```json
{
  "key": "fr-FR",
  "language": "French",
  "region": "France",
  "script": "Latin",
  "direction": "ltr",
  "knowledge_summary": {
    "term_count": 500,
    "expression_count": 200,
    "pattern_count": 50,
    "culture_ref_count": 30,
    "taboo_count": 15,
    "audience_trait_count": 25
  }
}
```

### `view://{id}`

Fetch a saved view/query definition.

```
view://composition
```

**Returns:**
```json
{
  "id": "composition",
  "name": "Page Composition",
  "description": "Page/Block composition hierarchy",
  "category": "contextual",
  "cypher": "MATCH (root {key: $nodeKey})...",
  "parameters": [
    { "name": "nodeKey", "param_type": "string", "required": true }
  ]
}
```

---

## MCP Prompts

NovaNet MCP provides 6 prompt templates for AI agent workflows. Prompts guide agents through tool orchestration for common tasks.

### `cypher_query`

Generate schema-aware Cypher queries from natural language.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `intent` | string | Yes | Natural language description of what to query |
| `constraints` | string | No | Additional constraints (realm, layer, limit) |

**Use case:** Agent needs to query the graph but doesn't know the schema.

### `cypher_explain`

Explain query results in business context.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `query` | string | Yes | The Cypher query that was executed |
| `results` | string | Yes | JSON array of query results |

**Use case:** Agent ran a query and needs to interpret results for the user.

### `block_generation`

Generate context for a single block's content.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `block_key` | string | Yes | The block's key |
| `locale` | string | Yes | Target locale (BCP-47) |
| `token_budget` | number | No | Max tokens (default: 10000) |

**Use case:** Agent generating content for a specific Block.

**Orchestration:**
1. Call `novanet_traverse` to get block structure + entities
2. Call `novanet_atoms` to get locale-specific knowledge
3. Assemble evidence packets by relevance
4. Return context with anchor syntax for cross-page links

### `page_generation`

Orchestrate full page generation across all blocks.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `page_key` | string | Yes | The page's key |
| `locale` | string | Yes | Target locale (BCP-47) |
| `token_budget` | number | No | Max tokens (default: 50000) |

**Use case:** Agent generating an entire page with multiple blocks.

**Orchestration:**
1. Discover page structure via `novanet_traverse`
2. For each block, invoke `block_generation` prompt
3. Assemble page-level metadata (title, meta_description)
4. Include context anchors for internal linking

### `entity_analysis`

Deep analysis of an entity with usage context.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `entity_key` | string | Yes | The entity's key |
| `locale` | string | Yes | Analysis locale |

**Use case:** Agent needs comprehensive understanding of an entity.

**Returns:**
- Entity definition summary
- Locale-specific adaptations (EntityContent)
- Pages/Blocks using this entity
- Related entities (semantic connections)

### `locale_briefing`

Locale voice and culture summary for content generation.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `locale_key` | string | Yes | Locale key (BCP-47) |

**Use case:** Agent needs to understand locale voice before generating content.

**Returns:**
- Voice guidelines (formality, tone, vocabulary)
- Cultural context (references, taboos)
- Technical formats (date, number, currency)
- Example phrases demonstrating the voice

---

## Module Architecture

```
src/
├── main.rs              # Entry point (stdio transport, tracing init)
├── lib.rs               # Public exports
├── error.rs             # Error types (thiserror + MCP mapping)
├── server/
│   ├── mod.rs           # Server struct (State + run loop)
│   ├── config.rs        # Environment configuration
│   ├── state.rs         # Shared state (Arc<StateInner>)
│   └── handler.rs       # MCP handler (rmcp macros)
├── neo4j/
│   ├── mod.rs
│   └── pool.rs          # Connection pool + query execution
├── cache/
│   └── mod.rs           # Query cache (moka)
├── tokens/
│   ├── mod.rs
│   └── counter.rs       # Token counting (tiktoken-rs)
├── rlm/
│   └── mod.rs           # RLM-on-KG structures
├── tools/
│   ├── mod.rs
│   ├── query.rs         # novanet_query implementation
│   ├── describe.rs      # novanet_describe implementation
│   ├── search.rs        # novanet_search implementation
│   ├── traverse.rs      # novanet_traverse implementation
│   ├── assemble.rs      # novanet_assemble implementation
│   └── atoms.rs         # novanet_atoms implementation
├── resources/
│   └── mod.rs           # MCP resources (entity://, kind://, locale://, view://)
└── prompts/
    └── mod.rs           # MCP prompts (Phase 3)
```

---

## Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `rmcp` | 0.15 | MCP protocol SDK (Anthropic official) |
| `neo4rs` | 0.8 | Neo4j driver with serde support |
| `tokio` | 1.43 | Async runtime |
| `moka` | 0.12 | High-performance concurrent cache |
| `tiktoken-rs` | 0.9 | Token counting (cl100k_base) |
| `parking_lot` | 0.12 | Fast Mutex/RwLock |
| `dashmap` | 6.1 | Concurrent HashMap |
| `schemars` | 1 | JSON Schema generation (rmcp requirement) |

---

## Error Handling

Errors map to MCP JSON-RPC 2.0 codes:

| Error Type | JSON-RPC Code | When |
|------------|---------------|------|
| `NotFound` | -32001 | Entity/resource not found |
| `InvalidCypher` | -32602 | Cypher validation failed |
| `WriteNotAllowed` | -32602 | Write operation attempted |
| `TokenBudgetExceeded` | -32602 | Token limit exceeded |
| `Connection` | -32603 | Neo4j connection failed |
| `Query` | -32603 | Query execution failed |
| `Internal` | -32603 | Unexpected error |

**Pattern for LLM-friendly errors:**

```rust
// Good: Actionable error message
Error::invalid_cypher("Query contains DELETE. Use read-only queries only.")

// Bad: Technical error
Error::Internal("deadpool timeout".to_string())
```

---

## Token Counting Strategy

The server uses a **hybrid token counting strategy**:

1. **Estimate (fast)**: `chars / 4` for English, `chars * 1.5` for CJK
2. **Exact (slow)**: tiktoken cl100k_base BPE encoding
3. **Smart budget check**: Estimate first, exact only when within 10% margin

```rust
// Fast path: clearly within budget
if estimate < budget * 90% { return true; }

// Fast path: clearly over budget
if estimate > budget * 110% { return false; }

// Slow path: exact count needed
counter.count(text) <= budget
```

---

## Caching

Query results are cached using moka with:
- **Max entries**: 10,000 (configurable)
- **TTL**: 5 minutes (configurable)
- **Key**: Hash of (cypher + params)

Cache is automatically invalidated on TTL expiry. Manual invalidation available via `cache.invalidate(key)`.

---

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_validate_read_only

# Integration tests (requires Neo4j)
cargo test --test integration
```

**Current test count:** 29 unit tests + 24 integration tests

---

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=novanet_mcp=debug cargo run
```

### Log Levels

| Level | Content |
|-------|---------|
| `error` | Connection failures, query errors |
| `warn` | Cache misses, high token usage |
| `info` | Server start, query execution |
| `debug` | Full query/response payloads |
| `trace` | Internal state changes |

### Common Issues

**Neo4j Connection Failed:**
```
Error: Neo4j connection failed to bolt://localhost:7687
```
- Check Neo4j is running: `docker ps | grep neo4j`
- Check credentials: `NOVANET_MCP_NEO4J_PASSWORD`
- Check URI: `NOVANET_MCP_NEO4J_URI`

**Write Operation Not Allowed:**
```
Error: Write operations not allowed: CREATE
```
- The server only allows read operations
- Remove CREATE/DELETE/MERGE/SET from query

**Token Budget Exceeded:**
```
Error: Token budget exceeded: 150000/100000
```
- Reduce query result size with LIMIT
- Increase budget: `NOVANET_MCP_DEFAULT_TOKEN_BUDGET`

---

## Integration with NovaNet

This MCP server reads from the same Neo4j instance as NovaNet Studio. Schema:

- **60 NodeKinds** across 2 realms (shared: 39, org: 21)
- **114 ArcKinds** in 5 families (ownership, localization, semantic, generation, mining)
- **200+ Locales** for multi-locale content generation

Key queries for agent bootstrap:

```cypher
-- Get schema overview
MATCH (k:Kind)
WITH k.realm AS realm, k.layer AS layer, collect(k.name) AS kinds
RETURN realm, layer, kinds ORDER BY realm, layer

-- Get entity with context
MATCH (e:Entity {key: $key})
OPTIONAL MATCH (e)-[:BELONGS_TO]->(c:EntityCategory)
OPTIONAL MATCH (e)-[:HAS_CONTENT]->(ec:EntityContent)
RETURN e, c.category_key, collect(DISTINCT ec.locale) AS locales

-- Get locale knowledge atoms
MATCH (l:Locale {key: $locale})-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)
WHERE t.domain = $domain
RETURN t.key, t.value LIMIT 50
```

---

## Roadmap

### Phase 1 (Complete)
- [x] Basic tools: novanet_query, novanet_describe
- [x] Query caching (moka)
- [x] Token counting (tiktoken)
- [x] Read-only Cypher validation
- [x] Error mapping to MCP

### Phase 2 (Complete)
- [x] MCP Resources: entity://, kind://, locale://, view://
- [x] Tools: search, traverse, assemble, atoms
- [x] RLM traversal with hop-by-hop evidence packets

### Phase 3 (In Progress)
- [x] Design: 6 MCP Prompts with Full RAG pattern
- [x] Design: Hybrid architecture (atomic tools + novanet_generate composite)
- [x] Design: Context Anchor pattern for cross-page references
- [x] Views: 4 generation views added to _registry.yaml
- [ ] Tool: novanet_generate (block/page mode)
- [ ] Prompts: cypher_query, cypher_explain, block_generation, page_generation, entity_analysis, locale_briefing
- [ ] Context Anchors: REFERENCES_PAGE arc + anchor resolution
- [ ] Evidence packet compression (~200 bytes target)

**Design document**: `docs/plans/2026-02-12-phase3-generate-prompts-design.md`

---

## Related Documentation

| File | Purpose |
|------|---------|
| `/CLAUDE.md` | NovaNet monorepo overview |
| `/packages/core/models/` | YAML schema definitions (source of truth) |
| `/packages/core/models/views/_registry.yaml` | View definitions (19 views, 4 generation) |
| `/packages/db/seed/` | Neo4j seed scripts |
| `/tools/novanet/` | Rust CLI + TUI (sister binary) |
| `/.claude/rules/novanet-terminology.md` | Domain vocabulary |
| `/.claude/rules/novanet-decisions.md` | Architecture decisions (ADRs) |
| `docs/plans/2026-02-12-phase3-generate-prompts-design.md` | Phase 3 design document |
