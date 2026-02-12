---
name: novanet-mcp
description: NovaNet MCP Server patterns - RLM-on-KG, agent integration, knowledge graph queries. Use when working on MCP tools, resources, prompts, or AI agent features.
disable-model-invocation: false
user-invocable: true
allowed-tools: Bash, Read, Grep, Glob
argument-hint: [status|test|debug|build]
---

# NovaNet MCP Server

MCP server exposing NovaNet knowledge graph to AI agents via RLM-on-KG patterns.

## Quick Reference

```
tools/novanet-mcp/
├── src/
│   ├── main.rs          # Entry point (stdio transport)
│   ├── lib.rs           # Public exports
│   ├── error.rs         # Error types (thiserror + MCP)
│   ├── server/          # MCP server implementation
│   │   ├── config.rs    # Environment configuration
│   │   ├── state.rs     # Shared state (Arc<StateInner>)
│   │   └── handler.rs   # Tool routing (rmcp macros)
│   ├── neo4j/
│   │   └── pool.rs      # Connection pool + query execution
│   ├── cache/           # Query cache (moka)
│   ├── tokens/          # Token counting (tiktoken)
│   ├── rlm/             # RLM-on-KG structures (Phase 2/3)
│   └── tools/           # MCP tool implementations
│       ├── query.rs     # novanet_query
│       └── describe.rs  # novanet_describe
└── tests/integration/   # Integration tests
```

## Commands

Based on `$ARGUMENTS`, execute:

### `status` (default)

Check server health and compilation:

```bash
cd tools/novanet-mcp && cargo check && echo "Tests:" && cargo test --no-run 2>&1 | tail -3
```

### `test`

Run test suite:

```bash
cd tools/novanet-mcp && cargo test -- --nocapture
```

### `debug`

Start with debug logging:

```bash
cd tools/novanet-mcp && RUST_LOG=novanet_mcp=debug cargo run
```

### `build`

Build release binary:

```bash
cd tools/novanet-mcp && cargo build --release && ls -la target/release/novanet-mcp
```

## Tools Reference

### novanet_query

Execute read-only Cypher queries.

**Parameters:**
| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `cypher` | String | Yes | - | Cypher query (read-only only) |
| `params` | Map | No | {} | Query parameters |
| `limit` | usize | No | 100 | Max results |
| `timeout_ms` | u64 | No | 30000 | Query timeout |

**Blocked Keywords:** CREATE, DELETE, MERGE, SET, REMOVE, DROP, DETACH

**Example:**
```json
{
  "cypher": "MATCH (e:Entity {key: $key}) RETURN e",
  "params": { "key": "qr-code" },
  "limit": 10
}
```

### novanet_describe

Bootstrap agent understanding of schema.

**Targets:**
| Target | Required Params | Returns |
|--------|-----------------|---------|
| `schema` | - | Realms, layers, kinds, arc families |
| `entity` | `entity_key` | Entity details with relations |
| `category` | `category_key` (opt) | Category members |
| `relations` | - | All ArcKind definitions |
| `locales` | - | Available locales |
| `stats` | - | Graph statistics |

**Example:**
```json
{ "describe": "schema" }
```

## Code Patterns

### Tool Implementation (rmcp)

```rust
use rmcp::{tool, tool_router, tool_handler, ServerHandler};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content};

#[tool_router]
impl MyHandler {
    #[tool(
        name = "my_tool",
        description = "Tool description for LLM"
    )]
    async fn my_tool(
        &self,
        params: Parameters<MyParams>,
    ) -> Result<CallToolResult, McpError> {
        // Implementation
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

#[tool_handler]
impl ServerHandler for MyHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Server description".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
```

### Error Handling

```rust
// Good: LLM-friendly, actionable error
Error::invalid_cypher(
    "Query contains DELETE. Only read operations (MATCH, RETURN) are allowed."
)

// Good: Contextual error
Error::Query {
    query: cypher.to_string(),
    source: neo4j_error,
}

// Bad: Generic, unhelpful
Error::Internal("Something went wrong".to_string())
```

### Read-Only Validation

```rust
fn validate_read_only(cypher: &str) -> Result<()> {
    let upper = cypher.to_uppercase();
    let forbidden = ["CREATE", "DELETE", "MERGE", "SET", "REMOVE", "DROP", "DETACH"];

    for keyword in forbidden {
        if upper.contains(&format!(" {} ", keyword))
            || upper.starts_with(&format!("{} ", keyword))
        {
            return Err(Error::write_not_allowed(keyword));
        }
    }
    Ok(())
}
```

### Token Counting

```rust
// Hybrid strategy: estimate first, exact when needed
let counter = TokenCounter::new();

// Fast check (96% accurate)
if counter.within_budget(text, budget) {
    return Ok(text);
}

// Truncate if over budget
let (truncated, actual) = counter.truncate_to_budget(text, budget);
```

### Query Caching

```rust
// Generate cache key
let cache_key = QueryCache::cache_key(&cypher, &params);

// Check cache
if let Some(cached) = state.cache().get(&cache_key).await {
    return Ok(cached);
}

// Execute and cache
let result = execute_query(&cypher, &params).await?;
state.cache().insert(cache_key, result.clone()).await;
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `NOVANET_MCP_NEO4J_URI` | bolt://localhost:7687 | Neo4j Bolt URI |
| `NOVANET_MCP_NEO4J_USER` | neo4j | Neo4j username |
| `NOVANET_MCP_NEO4J_PASSWORD` | (required) | Neo4j password |
| `NOVANET_MCP_NEO4J_POOL_SIZE` | 16 | Connection pool size |
| `NOVANET_MCP_CACHE_MAX_ENTRIES` | 10000 | Max cache entries |
| `NOVANET_MCP_CACHE_TTL_SECS` | 300 | Cache TTL |
| `NOVANET_MCP_DEFAULT_TOKEN_BUDGET` | 100000 | Default token budget |
| `NOVANET_MCP_MAX_HOPS` | 5 | Max traversal hops |

## Debugging

```bash
# Full debug output
RUST_LOG=novanet_mcp=debug cargo run

# MCP protocol tracing
RUST_LOG=rmcp=trace cargo run

# Neo4j query logging
RUST_LOG=novanet_mcp::neo4j=debug cargo run
```

## Common Issues

**Neo4j Connection Failed:**
- Check Neo4j running: `docker ps | grep neo4j`
- Check `NOVANET_MCP_NEO4J_PASSWORD` is set
- Check URI: `bolt://localhost:7687`

**Write Operation Not Allowed:**
- Server is read-only by design
- Use MATCH, not CREATE/DELETE/MERGE

**Token Budget Exceeded:**
- Add LIMIT clause to query
- Increase `NOVANET_MCP_DEFAULT_TOKEN_BUDGET`

## Related

- `/tools/novanet/` - Rust CLI + TUI (sister binary)
- `/packages/core/models/` - YAML schema (source of truth)
- `/.claude/rules/novanet-terminology.md` - Domain vocabulary
