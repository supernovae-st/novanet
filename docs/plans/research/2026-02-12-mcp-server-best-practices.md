# MCP Server Best Practices for NovaNet Knowledge Graph

**Date**: 2026-02-12
**Source**: Claude Code official documentation + Model Context Protocol specification
**Target**: Production-grade Rust MCP server for NovaNet graph integration

---

## Executive Summary

Building a production-grade MCP server for NovaNet requires:

1. **Protocol Foundation**: JSON-RPC 2.0 with standardized capability negotiation
2. **Resource Pattern**: URI-based resource discovery with pagination and content reading
3. **Tool Design**: Strongly-typed input schemas with Rust macros for safety
4. **Async Architecture**: Tokio-based concurrency with Arc/Mutex for state management
5. **Error Handling**: Graceful degradation with comprehensive logging
6. **Capability Declaration**: Explicit feature support for clients to detect capabilities

---

## Part 1: MCP Protocol Foundation

### JSON-RPC 2.0 Baseline

MCP uses JSON-RPC 2.0 for all request/response communication:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "resources/list",
  "params": {
    "cursor": "optional-pagination-cursor"
  }
}
```

**Key Requirements:**
- All requests MUST include `jsonrpc: "2.0"` and `id` (for matching responses)
- Methods use dot notation: `resources/list`, `tools/call`, `prompts/get`
- Params are method-specific; optional params use cursor-based pagination

### Initialization & Capability Negotiation

The lifecycle begins with client → server initialization:

```json
// CLIENT SENDS
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "roots": { "listChanged": true },
      "sampling": {},
      "tasks": { "requests": { "sampling": { "createMessage": {} } } }
    },
    "clientInfo": {
      "name": "Claude Code",
      "version": "1.0.0"
    }
  }
}

// SERVER RESPONDS
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "logging": {},
      "resources": {
        "subscribe": true,
        "listChanged": true
      },
      "tools": {
        "listChanged": true
      },
      "prompts": {
        "listChanged": true
      }
    },
    "serverInfo": {
      "name": "NovaNet MCP Server",
      "version": "11.7.0",
      "description": "Knowledge graph integration server"
    }
  }
}
```

**Capability Negotiation Rules:**

| Capability | Purpose | Sub-Features |
|-----------|---------|--------------|
| `logging` | Send log messages to client | (empty object) |
| `resources` | Expose graph data as resources | `subscribe`: individual item changes, `listChanged`: list updates |
| `tools` | Expose Cypher/query tools | `listChanged`: tool list changes |
| `prompts` | Expose prompt templates | `listChanged`: prompt list changes |

**For NovaNet Graph Server:** Declare all four capabilities to maximize integration:

```rust
// Rust pseudocode
capabilities: ServerCapabilities {
    logging: Some(Logging {}),
    resources: Some(ResourceOptions {
        subscribe: true,
        listChanged: true,
    }),
    tools: Some(ToolOptions {
        listChanged: true,
    }),
    prompts: Some(PromptOptions {
        listChanged: true,
    }),
}
```

---

## Part 2: Resource Exposure Pattern

### Resource Model (URI-Based)

Resources are the primary way to expose graph data. Each resource has:

```json
{
  "uri": "neo4j://novanet/node/kind:page/home",
  "name": "home",
  "title": "Home Page Kind",
  "description": "Page node type definition with 15 properties",
  "mimeType": "application/json",
  "icons": [
    {
      "src": "https://novanet.example.com/icons/page.svg",
      "mimeType": "image/svg+xml",
      "sizes": ["48x48"]
    }
  ]
}
```

**URI Scheme for NovaNet:**

```
neo4j://novanet/{resource_type}/{identifier}

Examples:
  neo4j://novanet/node/kind/page                    # NodeKind definition
  neo4j://novanet/node/realm/org                    # Realm definition
  neo4j://novanet/arc/kind/has-page                 # ArcKind definition
  neo4j://novanet/query/composition?nodeKey=home    # Query result
  neo4j://novanet/data/entity:qrcode@fr-FR          # Entity content instance
```

### List Resources (Discovery)

```json
// REQUEST
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "resources/list",
  "params": {
    "cursor": null
  }
}

// RESPONSE
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "resources": [
      {
        "uri": "neo4j://novanet/node/kind/page",
        "name": "Page",
        "title": "Page Kind",
        "description": "Content page node type",
        "mimeType": "application/json"
      },
      {
        "uri": "neo4j://novanet/node/kind/block",
        "name": "Block",
        "title": "Block Kind",
        "description": "Content block node type",
        "mimeType": "application/json"
      }
    ],
    "nextCursor": "page:2"  // For pagination
  }
}
```

**Pagination Implementation:**
- Return max 50 resources per page
- Include `nextCursor` if more results exist
- Client resubmits with `cursor` to fetch next page

### Read Resource (Content)

```json
// REQUEST
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "resources/read",
  "params": {
    "uri": "neo4j://novanet/node/kind/page"
  }
}

// RESPONSE
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "contents": [
      {
        "uri": "neo4j://novanet/node/kind/page",
        "mimeType": "application/json",
        "text": "{\"name\": \"Page\", \"realm\": \"org\", \"layer\": \"structure\", ...}"
      }
    ]
  }
}
```

**For NovaNet:** Return YAML or JSON with:
- Node/Arc kind definitions
- Property schemas
- Relationship cardinality
- Generated code artifacts (TypeScript, Cypher, Mermaid)

### Resource Subscriptions (Optional)

```json
// REQUEST - Subscribe to resource changes
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "resources/subscribe",
  "params": {
    "uri": "neo4j://novanet/node/kind/page"
  }
}

// SERVER SENDS (notification, not response)
{
  "jsonrpc": "2.0",
  "method": "notifications/resources/changed",
  "params": {
    "uri": "neo4j://novanet/node/kind/page"
  }
}
```

---

## Part 3: Tool Definition & Invocation

### Tool Schema Pattern

Tools use strongly-typed JSON schemas for input validation:

```json
// TOOL DEFINITION
{
  "name": "query_graph",
  "description": "Execute a Cypher query against the NovaNet graph",
  "inputSchema": {
    "type": "object",
    "properties": {
      "cypher": {
        "type": "string",
        "description": "Cypher query to execute"
      },
      "params": {
        "type": "object",
        "description": "Query parameters",
        "additionalProperties": true
      },
      "timeout_ms": {
        "type": "integer",
        "description": "Query timeout in milliseconds",
        "default": 30000
      }
    },
    "required": ["cypher"]
  }
}
```

**Best Practices:**
1. Use `description` for every property (LLM context)
2. Provide `default` values where sensible
3. Mark `required` fields explicitly
4. Use `enum` for constrained options
5. Include examples in description for complex schemas

### Tool Invocation

```json
// REQUEST
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "query_graph",
    "arguments": {
      "cypher": "MATCH (k:Kind) WHERE k.layer = $layer RETURN k.name, k.trait",
      "params": {
        "layer": "semantic"
      }
    }
  }
}

// RESPONSE
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "[{\"name\": \"Entity\", \"trait\": \"invariant\"}, ...]"
      }
    ],
    "isError": false
  }
}
```

### Rust Implementation with rmcp SDK

**Tool Router Pattern** (recommended):

```rust
use std::sync::Arc;
use rmcp::{
    ErrorData as McpError, model::*, tool, tool_router,
    handler::server::tool::ToolRouter,
};
use tokio::sync::Mutex;
use neo4rs::Graph;

#[derive(Clone)]
pub struct NovaNetGraphServer {
    graph: Arc<Graph>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl NovaNetGraphServer {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self {
            graph,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        name = "query_graph",
        description = "Execute Cypher query against NovaNet graph"
    )]
    async fn query_graph(
        &self,
        params: Parameters<QueryGraphParams>,
    ) -> Result<CallToolResult, McpError> {
        // Execute query
        let query = params.cypher.clone();
        let result = self.graph
            .execute(query::query(&params.cypher)
                .param("params", &params.params))
            .await
            .map_err(|e| McpError::new(-32603, e.to_string()))?;

        // Format results
        Ok(CallToolResult::success(vec![
            Content::text(serde_json::to_string(&result)?)
        ]))
    }

    #[tool(description = "List all NodeKind definitions")]
    async fn list_node_kinds(&self) -> Result<CallToolResult, McpError> {
        let mut result = self.graph
            .execute(query::query("MATCH (k:Kind) RETURN k"))
            .await
            .map_err(|e| McpError::new(-32603, e.to_string()))?;

        let kinds: Vec<_> = result
            .into_iter()
            .map(|row| /* parse to NodeKind */)
            .collect();

        Ok(CallToolResult::success(vec![
            Content::text(serde_json::to_string(&kinds)?)
        ]))
    }

    #[tool(description = "Get schema for a specific node kind")]
    async fn get_node_kind_schema(
        &self,
        params: Parameters<NodeKindParams>,
    ) -> Result<CallToolResult, McpError> {
        let kind_name = &params.kind_name;
        let schema = self.graph
            .execute(query::query(
                "MATCH (k:Kind {name: $name}) RETURN k"
            ).param("name", kind_name))
            .await
            .map_err(|e| McpError::new(-32603, e.to_string()))?;

        Ok(CallToolResult::success(vec![
            Content::text(serde_json::to_string(&schema)?)
        ]))
    }
}

// Parameter structs (strongly typed)
#[derive(Serialize, Deserialize, JsonSchema)]
struct QueryGraphParams {
    cypher: String,
    #[serde(default)]
    params: serde_json::Value,
    #[serde(default = "default_timeout")]
    timeout_ms: u32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct NodeKindParams {
    kind_name: String,
}

fn default_timeout() -> u32 {
    30000
}
```

**Macro Breakdown:**
- `#[tool_router]` on impl block: auto-generates tool list and router
- `#[tool(name = "...", description = "...")]` on methods: declares tool with metadata
- `Parameters<T>` generic: auto-deserializes JSON input to `T` with validation
- `CallToolResult` return type: wraps success/error responses

**Benefits:**
- Type safety: input validation at compile time
- Reflection: tool list auto-generated from methods
- Error handling: automatic JSON-RPC error formatting
- Documentation: tool descriptions extracted to capabilities

---

## Part 4: Prompt Templates

Prompts expose reusable LLM context templates:

```json
// PROMPT DEFINITION
{
  "name": "analyze_entity_schema",
  "description": "Analyze entity schema structure and suggest improvements",
  "arguments": [
    {
      "name": "entity_kind",
      "description": "Name of the EntityKind to analyze",
      "required": true
    },
    {
      "name": "analysis_depth",
      "description": "depth: basic, detailed, or comprehensive",
      "required": false
    }
  ]
}
```

```json
// PROMPT RETRIEVAL REQUEST
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "prompts/get",
  "params": {
    "name": "analyze_entity_schema",
    "arguments": {
      "entity_kind": "EntityContent",
      "analysis_depth": "detailed"
    }
  }
}

// RESPONSE
{
  "jsonrpc": "2.0",
  "id": 5,
  "result": {
    "messages": [
      {
        "role": "user",
        "content": {
          "type": "text",
          "text": "Analyze the EntityContent node kind structure:\n\n[YAML schema]\n\nProvide detailed analysis of:\n1. Property design\n2. Relationship patterns\n3. Data consistency risks\n4. Performance implications"
        }
      }
    ]
  }
}
```

**For NovaNet Prompts:**

1. **schema-review**: Request LLM review of node/arc definitions
2. **data-audit**: Prompt for data quality analysis
3. **query-assist**: Help generate Cypher queries
4. **arc-validation**: Validate arc cardinality and scope
5. **layer-coherence**: Check layer-specific naming conventions

---

## Part 5: Error Handling & Logging

### Error Response Format (JSON-RPC 2.0)

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32603,
    "message": "Internal server error",
    "data": {
      "context": "Query timeout after 30s",
      "neo4j_error": "TransientError: Connection pool exhausted"
    }
  }
}
```

**Standard Error Codes:**

| Code | Meaning | Example |
|------|---------|---------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid request | Missing jsonrpc field |
| -32601 | Method not found | Unknown method name |
| -32602 | Invalid params | Schema validation failed |
| -32603 | Internal error | Database connection failed |
| -32000 to -32099 | Server errors | Custom MCP errors |

### Logging Capability

```json
{
  "capabilities": {
    "logging": {}
  }
}
```

**Send Log Messages:**

```json
// SERVER SENDS (notification)
{
  "jsonrpc": "2.0",
  "method": "notifications/message",
  "params": {
    "level": "info",  // debug, info, notice, warning, error, critical
    "logger": "novanet.graph",
    "timestamp": "2026-02-12T10:30:00Z",
    "data": {
      "query_id": "abc123",
      "duration_ms": 145,
      "nodes_returned": 42
    }
  }
}
```

**Rust Implementation:**

```rust
use tracing::{info, warn, error};

async fn query_graph(&self, params: Parameters<QueryGraphParams>) -> Result<CallToolResult, McpError> {
    let start = std::time::Instant::now();

    match self.graph.execute(query).await {
        Ok(result) => {
            let elapsed = start.elapsed().as_millis();
            info!(
                query_id = %uuid::Uuid::new_v4(),
                duration_ms = elapsed,
                node_count = result.len(),
                "Graph query executed successfully"
            );
            Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string(&result)?
            )]))
        }
        Err(e) => {
            error!(
                query = %params.cypher,
                error = %e,
                "Graph query failed"
            );
            Err(McpError::new(-32603, format!("Query failed: {}", e)))
        }
    }
}
```

---

## Part 6: Rust MCP Server Architecture

### Project Structure

```
tools/novanet-mcp/
├── Cargo.toml                    # rmcp + neo4rs + tokio
├── src/
│   ├── main.rs                   # Entry point, stdio/HTTP transport
│   ├── server.rs                 # NovaNetGraphServer struct
│   ├── handlers/
│   │   ├── tools.rs              # Tool implementations
│   │   ├── resources.rs          # Resource exposure
│   │   ├── prompts.rs            # Prompt templates
│   │   └── logging.rs            # Log forwarding
│   ├── graph/
│   │   ├── client.rs             # Neo4j connection
│   │   ├── schema.rs             # YAML schema loading
│   │   └── queries.rs            # Cypher templates
│   └── config.rs                 # Configuration
└── tests/
    ├── tool_tests.rs
    ├── resource_tests.rs
    └── protocol_tests.rs
```

### Cargo.toml Dependencies

```toml
[package]
name = "novanet-mcp"
version = "11.7.0"
edition = "2024"

[dependencies]
# MCP
rmcp = { version = "0.8", features = ["tokio"] }

# Database
neo4rs = { version = "0.8" }

# Async runtime
tokio = { version = "1", features = ["full"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "0.8"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Error handling
thiserror = "1"
anyhow = "1"

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
```

### Initialization with Capability Declaration

```rust
use rmcp::{
    handler::server::ServerHandler,
    model::*,
    transport::StdioServerTransport,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("novanet_mcp=debug,rmcp=info")
        .with_writer(std::io::stderr)
        .init();

    // Connect to Neo4j
    let uri = std::env::var("NEO4J_URI").unwrap_or("bolt://localhost:7687".into());
    let user = std::env::var("NEO4J_USER").unwrap_or("neo4j".into());
    let pass = std::env::var("NEO4J_PASSWORD").unwrap_or("password".into());

    let graph = Arc::new(
        neo4rs::Graph::new(&uri, &user, &pass)
            .await?
    );

    // Create server
    let server = NovaNetGraphServer::new(graph);

    // Setup MCP transport (stdin/stdout by default)
    let transport = StdioServerTransport::new();

    // Initialize with capabilities
    let server_info = ServerInfo {
        name: "NovaNet MCP Server".into(),
        version: "11.7.0".into(),
        description: Some("Knowledge graph integration for Claude".into()),
        ..Default::default()
    };

    let capabilities = ServerCapabilities {
        logging: Some(Logging {}),
        resources: Some(ResourceOptions {
            subscribe: true,
            listChanged: true,
        }),
        tools: Some(ToolOptions {
            listChanged: true,
        }),
        prompts: Some(PromptOptions {
            listChanged: true,
        }),
    };

    // Run server
    ServerHandler::new(server)
        .with_capabilities(capabilities)
        .with_info(server_info)
        .run(transport)
        .await?;

    Ok(())
}
```

### Thread-Safe State Management

```rust
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use std::collections::HashMap;

#[derive(Clone)]
pub struct NovaNetGraphServer {
    // Connection pool
    graph: Arc<neo4rs::Graph>,

    // Cached schemas (RwLock for multiple readers)
    schemas: Arc<RwLock<HashMap<String, NodeKindSchema>>>,

    // Active queries (Mutex for exclusive access)
    active_queries: Arc<Mutex<HashMap<String, QueryState>>>,

    // Tool router (from macro)
    tool_router: ToolRouter<Self>,
}

pub struct QueryState {
    pub query_id: String,
    pub started_at: std::time::Instant,
    pub cypher: String,
}

impl NovaNetGraphServer {
    pub fn new(graph: Arc<neo4rs::Graph>) -> Self {
        Self {
            graph,
            schemas: Arc::new(RwLock::new(HashMap::new())),
            active_queries: Arc::new(Mutex::new(HashMap::new())),
            tool_router: Self::tool_router(),
        }
    }

    async fn get_schema(&self, kind_name: &str) -> anyhow::Result<NodeKindSchema> {
        // Check cache first (read lock)
        {
            let schemas = self.schemas.read().await;
            if let Some(schema) = schemas.get(kind_name) {
                return Ok(schema.clone());
            }
        }

        // Fetch from database if not cached
        let schema = self.fetch_schema_from_db(kind_name).await?;

        // Update cache (write lock)
        {
            let mut schemas = self.schemas.write().await;
            schemas.insert(kind_name.to_string(), schema.clone());
        }

        Ok(schema)
    }
}
```

**Best Practices:**
- `Arc<T>`: Share ownership across async tasks
- `RwLock<T>`: Multiple concurrent readers, exclusive writer
- `Mutex<T>`: Exclusive access for mutable state
- Never hold locks across `.await` boundaries (causes deadlocks)

---

## Part 7: Production Checklist

### Before Deployment

- [ ] **Protocol Compliance**
  - [ ] Implements JSON-RPC 2.0 correctly
  - [ ] Declares all supported capabilities
  - [ ] Handles capability negotiation errors
  - [ ] Supports protocol version negotiation

- [ ] **Resource Implementation**
  - [ ] Resource URIs follow consistent scheme
  - [ ] Pagination works for 1000+ resources
  - [ ] Resource descriptions are helpful for LLMs
  - [ ] Supports both text and structured MIME types

- [ ] **Tool Implementation**
  - [ ] All tool input schemas are JSON Schema compliant
  - [ ] Tools have detailed descriptions
  - [ ] Timeout handling prevents hanging
  - [ ] Error responses include context

- [ ] **Error Handling**
  - [ ] All errors use correct JSON-RPC codes
  - [ ] Errors include actionable messages
  - [ ] Database connection failures gracefully degrade
  - [ ] Query timeouts are handled

- [ ] **Logging & Observability**
  - [ ] Structured logging to stderr
  - [ ] Log levels appropriate (debug/info/warn/error)
  - [ ] Query performance metrics logged
  - [ ] Error context captured (stack traces in debug)

- [ ] **Testing**
  - [ ] Unit tests for each tool
  - [ ] Integration tests with real Neo4j
  - [ ] Protocol compliance tests (can parse all responses)
  - [ ] Performance tests (100+ concurrent queries)

- [ ] **Documentation**
  - [ ] README with setup instructions
  - [ ] Tool descriptions match capabilities
  - [ ] Resource URI scheme documented
  - [ ] Example prompts and queries provided

---

## Part 8: Integration with Claude Code

### Configuration in `.claude/mcp.json`

```json
{
  "mcpServers": {
    "novanet": {
      "command": "cargo",
      "args": ["run", "--release", "--"],
      "env": {
        "NEO4J_URI": "bolt://localhost:7687",
        "NEO4J_USER": "neo4j",
        "NEO4J_PASSWORD": "novanetpassword",
        "RUST_LOG": "novanet_mcp=info,rmcp=debug"
      }
    }
  }
}
```

### Usage from Claude Code

```bash
# Claude Code will auto-discover tools:
# - query_graph
# - list_node_kinds
# - get_node_kind_schema
# - search_graph
# - validate_schema

# Access resources:
# - neo4j://novanet/node/kind/*
# - neo4j://novanet/arc/kind/*
# - neo4j://novanet/query/*

# Use prompts:
# - analyze_entity_schema
# - schema_review
# - data_audit
```

---

## Part 9: Key Takeaways

### For NovaNet MCP Server v1

1. **Keep it simple**: Focus on query tool + resource discovery first
2. **Strong typing**: Use Rust type system + JSON Schema for safety
3. **Async all the way**: Tokio for handling concurrent Claude requests
4. **Graceful errors**: Always return helpful error context
5. **Observable**: Log all queries, timeouts, errors
6. **Testable**: Unit test each tool independently

### Phased Implementation

**Phase 1** (v11.7):
- Basic server with query_graph tool
- Resource listing for NodeKind/ArcKind
- Error handling + logging

**Phase 2** (v11.8):
- Prompt templates for schema analysis
- Cached schema retrieval
- Performance optimization

**Phase 3** (v11.9):
- Advanced query builder tool
- Resource subscriptions for change notifications
- Audit trail logging

---

## References

- **MCP Specification**: https://modelcontextprotocol.io/specification/2025-11-25/
- **Rust rmcp SDK**: https://docs.rs/rmcp/latest/rmcp/
- **JSON-RPC 2.0**: https://www.jsonrpc.org/specification
- **Neo4j Rust Driver**: https://neo4j.com/developer/rust/
- **Tokio Async Runtime**: https://tokio.rs/
- **Tracing Instrumentation**: https://docs.rs/tracing/latest/tracing/

