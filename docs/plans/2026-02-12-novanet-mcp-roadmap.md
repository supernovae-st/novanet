# NovaNet MCP Server Implementation Roadmap

**Date**: 2026-02-12
**Target**: Integrate NovaNet knowledge graph with Claude Code via MCP
**Phases**: 3 phases (v11.8, v11.9, v12.0)

---

## Overview

NovaNet MCP Server will expose the knowledge graph to Claude Code through three phases:

```
Phase 1 (v11.8):  Core query tool + schema resources
Phase 2 (v11.9):  Query builder + prompt templates + caching
Phase 3 (v12.0):  Subscriptions + audit trails + performance
```

---

## Phase 1: v11.8 Core Implementation

### Goals

- Basic MCP server running on stdio transport
- Single query tool with Neo4j backend
- Resource discovery for NodeKind/ArcKind/Views
- Error handling + basic logging

### Deliverables

#### 1.1 Project Setup

**New crate**: `tools/novanet-mcp/`

```bash
cargo new --bin tools/novanet-mcp
```

**Cargo.toml:**

```toml
[package]
name = "novanet-mcp"
version = "11.8.0"
edition = "2024"

[[bin]]
name = "novanet-mcp-server"
path = "src/main.rs"

[dependencies]
# MCP
rmcp = { version = "0.8", features = ["tokio"] }

# Database
neo4rs = "0.8"

# Async
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

#### 1.2 Server Struct & Initialization

**File**: `src/server.rs`

```rust
use std::sync::Arc;
use rmcp::{tool_router, ErrorData as McpError, model::*};

#[derive(Clone)]
pub struct NovaNetServer {
    pub graph: Arc<neo4rs::Graph>,
    pub tool_router: ToolRouter<Self>,
}

impl NovaNetServer {
    pub async fn new(uri: &str, user: &str, password: &str) -> anyhow::Result<Self> {
        let graph = Arc::new(
            neo4rs::Graph::new(uri, user, password)
                .await?
        );

        Ok(Self {
            graph,
            tool_router: Self::tool_router(),
        })
    }

    pub fn server_info(&self) -> ServerInfo {
        ServerInfo {
            name: "NovaNet MCP Server".into(),
            version: "11.8.0".into(),
            description: Some("Knowledge graph integration for Claude".into()),
            ..Default::default()
        }
    }

    pub fn capabilities(&self) -> ServerCapabilities {
        ServerCapabilities {
            logging: Some(Logging {}),
            resources: Some(ResourceOptions {
                subscribe: false,
                listChanged: false,
            }),
            tools: Some(ToolOptions {
                listChanged: false,
            }),
            ..Default::default()
        }
    }
}
```

#### 1.3 Query Tool (Primary Tool)

**File**: `src/handlers/tools.rs`

```rust
use rmcp::{tool, tool_router, ErrorData as McpError, model::*};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct QueryParams {
    /// Cypher query to execute (e.g., "MATCH (n:Kind) RETURN n LIMIT 10")
    pub cypher: String,

    /// Query parameter bindings
    #[serde(default)]
    pub params: serde_json::Value,

    /// Maximum execution time (milliseconds)
    #[serde(default = "default_timeout")]
    pub timeout_ms: u32,
}

fn default_timeout() -> u32 {
    30000
}

#[tool_router]
impl NovaNetServer {
    #[tool(description = "Execute Cypher query against NovaNet graph")]
    pub async fn query(
        &self,
        params: Parameters<QueryParams>,
    ) -> Result<CallToolResult, McpError> {
        let query_id = uuid::Uuid::new_v4();
        tracing::debug!(%query_id, cypher = %params.cypher, "Query started");

        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_millis(params.timeout_ms as u64);

        match tokio::time::timeout(
            timeout,
            self.execute_query_internal(&params),
        )
        .await
        {
            Ok(Ok(result)) => {
                let elapsed = start.elapsed();
                tracing::info!(
                    %query_id,
                    duration_ms = elapsed.as_millis() as u64,
                    result_count = result.len(),
                    "Query succeeded"
                );
                Ok(CallToolResult::success(vec![
                    Content::text(serde_json::to_string(&result)?)
                ]))
            }
            Ok(Err(e)) => {
                let elapsed = start.elapsed();
                tracing::warn!(
                    %query_id,
                    error = %e,
                    duration_ms = elapsed.as_millis() as u64,
                    "Query failed"
                );
                Err(McpError::new(-32603, format!("Query error: {}", e)))
            }
            Err(_) => {
                tracing::warn!(
                    %query_id,
                    timeout_ms = params.timeout_ms,
                    "Query timeout"
                );
                Err(McpError::new(-32603, "Query timeout"))
            }
        }
    }

    async fn execute_query_internal(
        &self,
        params: &Parameters<QueryParams>,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut query = neo4rs::query(&params.cypher);

        // Bind parameters
        if let Some(obj) = params.params.as_object() {
            for (key, value) in obj {
                query = query.param(key.as_str(), value);
            }
        }

        let mut result = self.graph.execute(query).await?;
        let mut rows = Vec::new();

        while let Some(row) = result.next().await? {
            // Convert Neo4j row to JSON
            let row_json = self.row_to_json(&row)?;
            rows.push(row_json);
        }

        Ok(rows)
    }

    fn row_to_json(&self, row: &neo4rs::Row) -> anyhow::Result<serde_json::Value> {
        // Convert Neo4j row to JSON representation
        // This depends on query structure - simplified version:
        Ok(serde_json::to_value(row)?)
    }
}
```

#### 1.4 Resource Discovery

**File**: `src/handlers/resources.rs`

```rust
use rmcp::{tool, tool_router, ErrorData as McpError, model::*};

#[tool_router]
impl NovaNetServer {
    #[tool(description = "List all available resources")]
    pub async fn resources_list(
        &self,
        params: Parameters<ResourceListParams>,
    ) -> Result<CallToolResult, McpError> {
        const PAGE_SIZE: usize = 50;

        let page = params.cursor
            .as_ref()
            .and_then(|c| c.strip_prefix("page:"))
            .and_then(|p| p.parse::<usize>().ok())
            .unwrap_or(0);

        // Fetch all node kinds
        let kinds = self.fetch_all_node_kinds().await
            .map_err(|e| McpError::new(-32603, e.to_string()))?;

        let start = page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(kinds.len());
        let page_items = &kinds[start..end];

        let resources: Vec<_> = page_items
            .iter()
            .map(|k| serde_json::json!({
                "uri": format!("neo4j://novanet/kind/node/{}", k["name"]),
                "name": k["name"],
                "title": k.get("display_name").unwrap_or(&serde_json::json!(k["name"])),
                "description": k.get("description"),
                "mimeType": "application/json",
            }))
            .collect();

        let next_cursor = if end < kinds.len() {
            Some(format!("page:{}", page + 1))
        } else {
            None
        };

        let response = serde_json::json!({
            "resources": resources,
            "nextCursor": next_cursor,
        });

        Ok(CallToolResult::success(vec![
            Content::text(response.to_string())
        ]))
    }

    #[tool(description = "Read resource contents")]
    pub async fn resources_read(
        &self,
        params: Parameters<ResourceReadParams>,
    ) -> Result<CallToolResult, McpError> {
        let uri = &params.uri;

        // Parse URI: neo4j://novanet/kind/node/{name}
        let kind_name = uri.split('/').last()
            .ok_or_else(|| McpError::new(-32602, "Invalid URI"))?;

        let kind = self.fetch_node_kind(kind_name).await
            .map_err(|e| McpError::new(-32603, e.to_string()))?;

        Ok(CallToolResult::success(vec![
            Content::text(serde_json::to_string(&kind)?)
        ]))
    }

    async fn fetch_all_node_kinds(&self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut result = self.graph
            .execute(neo4rs::query("MATCH (k:Kind) RETURN k ORDER BY k.name"))
            .await?;

        let mut kinds = Vec::new();
        while let Some(row) = result.next().await? {
            let kind = row.get("k")?;
            kinds.push(kind);
        }

        Ok(kinds)
    }

    async fn fetch_node_kind(&self, name: &str) -> anyhow::Result<serde_json::Value> {
        let mut result = self.graph
            .execute(neo4rs::query("MATCH (k:Kind {name: $name}) RETURN k")
                .param("name", name))
            .await?;

        result.next()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Kind not found"))
            .map(|row| row.get("k"))
            .and_then(|r| r)
    }
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ResourceListParams {
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ResourceReadParams {
    /// Resource URI (e.g., "neo4j://novanet/kind/node/Page")
    pub uri: String,
}
```

#### 1.5 Main Entry Point

**File**: `src/main.rs`

```rust
use rmcp::transport::StdioServerTransport;
use rmcp::handler::server::ServerHandler;
use std::sync::Arc;

mod server;
mod handlers {
    pub mod tools;
    pub mod resources;
}

use server::NovaNetServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or("info".into())
        )
        .with_writer(std::io::stderr)
        .init();

    // Load configuration from environment
    let uri = std::env::var("NEO4J_URI").unwrap_or("bolt://localhost:7687".into());
    let user = std::env::var("NEO4J_USER").unwrap_or("neo4j".into());
    let password = std::env::var("NEO4J_PASSWORD").unwrap_or("password".into());

    // Create server
    let server = NovaNetServer::new(&uri, &user, &password).await?;
    tracing::info!("NovaNet MCP Server initialized");

    // Setup MCP transport
    let transport = StdioServerTransport::new();

    // Run server
    ServerHandler::new(server.clone())
        .run(transport)
        .await?;

    Ok(())
}
```

#### 1.6 Error Handling Module

**File**: `src/errors.rs`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NovaNetMcpError {
    #[error("Neo4j error: {0}")]
    Neo4jError(#[from] neo4rs::Error),

    #[error("Query timeout")]
    QueryTimeout,

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<NovaNetMcpError> for rmcp::ErrorData {
    fn from(e: NovaNetMcpError) -> Self {
        let (code, message) = match e {
            NovaNetMcpError::Neo4jError(e) => (-32603, format!("Database error: {}", e)),
            NovaNetMcpError::QueryTimeout => (-32603, "Query timeout".into()),
            NovaNetMcpError::InvalidQuery(msg) => (-32602, format!("Invalid query: {}", msg)),
            NovaNetMcpError::ResourceNotFound(msg) => (-32603, format!("Resource not found: {}", msg)),
            NovaNetMcpError::Internal(msg) => (-32603, format!("Internal error: {}", msg)),
        };

        rmcp::ErrorData::new(code, message)
    }
}
```

#### 1.7 Tests

**File**: `tests/protocol_tests.rs`

```rust
use novanet_mcp::NovaNetServer;

#[tokio::test]
async fn test_server_initialization() {
    let server = NovaNetServer::new(
        "bolt://localhost:7687",
        "neo4j",
        "password",
    ).await;

    assert!(server.is_ok());
}

#[tokio::test]
async fn test_query_tool_exists() {
    let server = NovaNetServer::new(
        "bolt://localhost:7687",
        "neo4j",
        "password",
    ).await.unwrap();

    let capabilities = server.capabilities();
    assert!(capabilities.tools.is_some());
}

#[tokio::test]
async fn test_resource_discovery() {
    let server = NovaNetServer::new(
        "bolt://localhost:7687",
        "neo4j",
        "password",
    ).await.unwrap();

    // Test resources_list tool
    // ...
}
```

### Timeline

- Week 1: Setup project, server struct, main entry point
- Week 2: Query tool implementation + error handling
- Week 3: Resource discovery implementation
- Week 4: Testing + integration with Claude Code
- End: v11.8 release

---

## Phase 2: v11.9 Advanced Features

### Goals

- Query builder for safe Cypher generation
- Prompt templates for LLM context
- Schema caching for performance
- Tool list changes notification

### Key Additions

1. **Query Builder Tool**
   - Input: semantic query description
   - Output: Safe Cypher + explanation

2. **Prompt Templates**
   - Analyze entity schema
   - Review node relationships
   - Suggest improvements

3. **Caching Layer**
   - LRU cache for node kinds
   - TTL-based invalidation
   - Manual cache clear tool

4. **Performance**
   - Query result batching
   - Parallel resource loading
   - Connection pool tuning

---

## Phase 3: v12.0 Production Features

### Goals

- Resource subscriptions
- Audit trail logging
- Full schema import/export
- Advanced query optimization

### Key Additions

1. **Resource Subscriptions**
   - Subscribe to schema changes
   - Notify on data updates
   - Live graph visualization support

2. **Audit Trails**
   - Log all queries with context
   - Track LLM usage
   - Performance metrics

3. **Schema Tools**
   - Export full schema as JSON/YAML
   - Import schema changes
   - Schema diff/merge

---

## Configuration for Claude Code

**File**: `.claude/mcp.json`

```json
{
  "mcpServers": {
    "novanet": {
      "command": "cargo",
      "args": [
        "run",
        "--release",
        "--manifest-path=tools/novanet-mcp/Cargo.toml",
        "--"
      ],
      "env": {
        "NEO4J_URI": "bolt://localhost:7687",
        "NEO4J_USER": "neo4j",
        "NEO4J_PASSWORD": "novanetpassword",
        "RUST_LOG": "novanet_mcp=info"
      }
    }
  }
}
```

---

## Success Criteria

### v11.8
- [ ] Server starts without errors
- [ ] Query tool works with sample Cypher
- [ ] Resource discovery returns NodeKinds
- [ ] Error handling for invalid queries
- [ ] Logging to stderr

### v11.9
- [ ] Query builder generates safe Cypher
- [ ] Prompt templates provide useful context
- [ ] Cache improves performance by 50%+
- [ ] Tool list updates detected

### v12.0
- [ ] Subscriptions send notifications
- [ ] Audit logs track all operations
- [ ] Schema import/export works
- [ ] 1000+ concurrent queries handled

---

## References

- MCP Spec: https://modelcontextprotocol.io/specification/
- rmcp SDK: https://docs.rs/rmcp/
- Neo4j Rust: https://neo4j.com/developer/rust/

