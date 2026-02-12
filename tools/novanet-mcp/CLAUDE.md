# NovaNet MCP Server

MCP (Model Context Protocol) server exposing the NovaNet knowledge graph to AI agents.

**Version**: 0.1.0 | **Rust**: 1.86 | **Edition**: 2024 | **rmcp**: 0.15

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
│                    │              └── novanet_describe   (Schema bootstrap) │
│                    │                                                        │
│               MCP Protocol                                                  │
│               (JSON-RPC 2.0)                                                │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 1 (Current)                                                          │
│  ├── Tools: novanet_query, novanet_describe                                 │
│  ├── State: Neo4j pool, Query cache, Token counter                          │
│  └── Error: Typed errors with MCP JSON-RPC mapping                          │
│                                                                             │
│  PHASE 2 (Planned)                                                          │
│  ├── Resources: entity://, kind://, locale://, view://                      │
│  └── Tools: search, traverse, assemble, atoms                               │
│                                                                             │
│  PHASE 3 (Planned)                                                          │
│  ├── Prompts: cypher_query, content_generation, context_analysis            │
│  └── Tools: generate (RLM-on-KG evidence assembly)                          │
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

**Returns:**
```json
{
  "target": "schema",
  "data": {
    "schema_version": "11.7.0",
    "realms": { "shared": {...}, "org": {...} },
    "arc_families": [...],
    "statistics": {...},
    "traversal_hints": {...}
  },
  "token_estimate": 1500
}
```

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
│   └── mod.rs           # RLM-on-KG structures (Phase 2/3)
├── tools/
│   ├── mod.rs
│   ├── query.rs         # novanet_query implementation
│   └── describe.rs      # novanet_describe implementation
├── resources/
│   └── mod.rs           # MCP resources (Phase 2)
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

**Current test count:** 15 unit tests

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

### Phase 1 (Current)
- [x] Basic tools: novanet_query, novanet_describe
- [x] Query caching (moka)
- [x] Token counting (tiktoken)
- [x] Read-only Cypher validation
- [x] Error mapping to MCP

### Phase 2
- [ ] MCP Resources: entity://, kind://, locale://, view://
- [ ] Tools: search, traverse, assemble, atoms
- [ ] RLM traversal with hop-by-hop evidence packets

### Phase 3
- [ ] MCP Prompts: cypher_query, content_generation, context_analysis
- [ ] Tool: generate (full RLM-on-KG pipeline)
- [ ] Evidence packet compression (~200 bytes)
- [ ] Token budget enforcement

---

## Related Documentation

| File | Purpose |
|------|---------|
| `/CLAUDE.md` | NovaNet monorepo overview |
| `/packages/core/models/` | YAML schema definitions (source of truth) |
| `/packages/db/seed/` | Neo4j seed scripts |
| `/tools/novanet/` | Rust CLI + TUI (sister binary) |
| `/.claude/rules/novanet-terminology.md` | Domain vocabulary |
| `/.claude/rules/novanet-decisions.md` | Architecture decisions (ADRs) |
