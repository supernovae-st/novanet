# MCP Server Quick Start Template

**Date**: 2026-02-12
**Purpose**: Get a working NovaNet MCP server running in 30 minutes
**Level**: Intermediate

---

## 1. Project Setup (5 minutes)

```bash
# Create new Rust crate
cargo new --bin tools/novanet-mcp

# Navigate to project
cd tools/novanet-mcp

# Update Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "novanet-mcp"
version = "11.8.0"
edition = "2024"

[[bin]]
name = "novanet-mcp-server"
path = "src/main.rs"

[dependencies]
rmcp = { version = "0.8", features = ["tokio"] }
neo4rs = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "0.8"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
anyhow = "1"
uuid = { version = "1", features = ["v4", "serde"] }
EOF
```

---

## 2. Server Implementation (10 minutes)

**File**: `src/main.rs`

```rust
use std::sync::Arc;
use rmcp::{
    tool, tool_router, ErrorData as McpError, model::*,
    transport::StdioServerTransport,
    handler::server::ServerHandler,
};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// === PARAMETER STRUCTS ===

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct QueryParams {
    /// Cypher query to execute
    pub cypher: String,

    /// Query parameters
    #[serde(default)]
    pub params: serde_json::Value,

    /// Timeout in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u32,
}

fn default_timeout() -> u32 {
    30000
}

// === SERVER STRUCT ===

#[derive(Clone)]
pub struct NovaNetServer {
    pub graph: Arc<neo4rs::Graph>,
    pub tool_router: ToolRouter<Self>,
}

// === TOOL IMPLEMENTATIONS ===

#[tool_router]
impl NovaNetServer {
    pub fn new(graph: Arc<neo4rs::Graph>) -> Self {
        Self {
            graph,
            tool_router: Self::tool_router(),
        }
    }

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
                let elapsed = start.elapsed().as_millis() as u64;
                tracing::info!(
                    %query_id,
                    duration_ms = elapsed,
                    result_count = result.len(),
                    "Query succeeded"
                );

                Ok(CallToolResult::success(vec![
                    Content::text(serde_json::to_string(&result)?)
                ]))
            }
            Ok(Err(e)) => {
                let elapsed = start.elapsed().as_millis() as u64;
                tracing::warn!(
                    %query_id,
                    error = %e,
                    duration_ms = elapsed,
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
            // Convert row to JSON (simplified)
            rows.push(serde_json::to_value(row)?);
        }

        Ok(rows)
    }
}

// === MAIN ENTRY POINT ===

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

    tracing::info!("Connecting to Neo4j at {}", uri);

    // Create Neo4j connection
    let graph = Arc::new(
        neo4rs::Graph::new(&uri, &user, &password)
            .await?
    );

    tracing::info!("Connected to Neo4j");

    // Create server
    let server = NovaNetServer::new(graph.clone());

    // Setup MCP capabilities
    let server_info = ServerInfo {
        name: "NovaNet MCP Server".into(),
        version: "11.8.0".into(),
        description: Some("Knowledge graph integration for Claude".into()),
        ..Default::default()
    };

    let capabilities = ServerCapabilities {
        logging: Some(Logging {}),
        resources: Some(ResourceOptions {
            subscribe: false,
            listChanged: false,
        }),
        tools: Some(ToolOptions {
            listChanged: false,
        }),
        ..Default::default()
    };

    tracing::info!("Starting MCP server");

    // Run server on stdio transport
    let transport = StdioServerTransport::new();
    ServerHandler::new(server)
        .run(transport)
        .await?;

    Ok(())
}
```

---

## 3. Test It Locally (10 minutes)

```bash
# 1. Start Neo4j
cd path/to/novanet-hq
pnpm infra:up
pnpm infra:seed

# 2. Build the server
cd tools/novanet-mcp
cargo build --release

# 3. Run with environment variables
NEO4J_URI="bolt://localhost:7687" \
NEO4J_USER="neo4j" \
NEO4J_PASSWORD="novanetpassword" \
RUST_LOG="info" \
./target/release/novanet-mcp-server

# 4. You should see:
# timestamp=... level=INFO message="Connecting to Neo4j..."
# timestamp=... level=INFO message="Connected to Neo4j"
# timestamp=... level=INFO message="Starting MCP server"
```

---

## 4. Configure Claude Code (5 minutes)

**File**: `.claude/mcp.json` (create if doesn't exist)

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
        "RUST_LOG": "info"
      }
    }
  }
}
```

---

## 5. Test with Claude Code

```bash
# In Claude Code terminal:
# Claude should auto-discover the "query" tool

# Try this in Claude:
/set-context
> I want to run a Cypher query to find all node kinds

# Claude should suggest using the "query" tool with:
# - cypher: "MATCH (k:Kind) RETURN k.name, k.realm, k.layer LIMIT 10"
# - params: {}
# - timeout_ms: 30000
```

---

## 6. Quick Improvements (Optional)

### Add Resource Discovery

```rust
#[tool_router]
impl NovaNetServer {
    #[tool(description = "List all node kinds")]
    pub async fn list_kinds(
        &self,
        params: Parameters<ListParams>,
    ) -> Result<CallToolResult, McpError> {
        let kinds = self.query(Parameters::new(QueryParams {
            cypher: "MATCH (k:Kind) RETURN k ORDER BY k.name".into(),
            params: serde_json::json!({}),
            timeout_ms: 30000,
            explain: false,
        })).await?;

        Ok(kinds)
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ListParams {}
```

### Add Error Type

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NovaNetError {
    #[error("Query timeout")]
    Timeout,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}
```

### Add Retry Logic

```rust
async fn execute_with_retry(
    &self,
    cypher: &str,
    max_retries: u32,
) -> anyhow::Result<Vec<serde_json::Value>> {
    for attempt in 0..max_retries {
        match self.execute_query_internal_simple(cypher).await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < max_retries - 1 => {
                let backoff = std::time::Duration::from_millis(100 * 2_u64.pow(attempt));
                tracing::warn!(%e, ?backoff, "Retrying query");
                tokio::time::sleep(backoff).await;
            }
            Err(e) => return Err(e),
        }
    }
    Err(anyhow::anyhow!("Max retries exceeded"))
}
```

---

## 7. File Structure

After following this guide, your project should look like:

```
tools/novanet-mcp/
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs                # Everything in one file for now
└── target/
    └── release/
        └── novanet-mcp-server
```

---

## 8. Troubleshooting

### "Connection refused" Error
```bash
# Make sure Neo4j is running
pnpm infra:up

# Check if port 7687 is open
lsof -i :7687
```

### "Invalid cypher" Error in Claude
```bash
# Test query in Neo4j Browser first
# http://localhost:7474

# Make sure Cypher syntax is valid
MATCH (k:Kind) RETURN k LIMIT 10
```

### "Tool not discovered" in Claude Code
```bash
# Check .claude/mcp.json exists and is valid
cat .claude/mcp.json | jq

# Restart Claude Code after changing mcp.json

# Check server is actually running:
ps aux | grep novanet-mcp
```

---

## 9. Next Steps

After getting this working:

1. **Extract to modules**: Move server.rs, handlers/, etc.
2. **Add resource discovery**: Implement `resources/list` and `resources/read`
3. **Add caching**: Use Arc<RwLock<LruCache>> for schemas
4. **Add tests**: Unit tests for each tool
5. **Add prompts**: Implement prompt templates
6. **Optimize performance**: Connection pooling, query optimization

---

## 10. Complete Working Example

Here's a minimal complete working server (copy-paste ready):

**File**: `src/main.rs`

```rust
use std::sync::Arc;
use rmcp::{
    tool, tool_router, ErrorData as McpError, model::*,
    transport::StdioServerTransport,
    handler::server::ServerHandler,
};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct QueryParams {
    pub cypher: String,
    #[serde(default)]
    pub params: serde_json::Value,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u32,
}

fn default_timeout() -> u32 { 30000 }

#[derive(Clone)]
pub struct NovaNetServer {
    pub graph: Arc<neo4rs::Graph>,
    pub tool_router: ToolRouter<Self>,
}

#[tool_router]
impl NovaNetServer {
    pub fn new(graph: Arc<neo4rs::Graph>) -> Self {
        Self { graph, tool_router: Self::tool_router() }
    }

    #[tool(description = "Execute Cypher query")]
    pub async fn query(
        &self,
        params: Parameters<QueryParams>,
    ) -> Result<CallToolResult, McpError> {
        let timeout = std::time::Duration::from_millis(params.timeout_ms as u64);
        match tokio::time::timeout(timeout, async {
            let mut q = neo4rs::query(&params.cypher);
            if let Some(obj) = params.params.as_object() {
                for (k, v) in obj {
                    q = q.param(k, v);
                }
            }
            let mut result = self.graph.execute(q).await.map_err(|e| format!("{}", e))?;
            let mut rows = Vec::new();
            while let Some(row) = result.next().await.map_err(|e| format!("{}", e))? {
                rows.push(serde_json::to_value(row).map_err(|e| format!("{}", e))?);
            }
            Ok::<_, String>(rows)
        }).await {
            Ok(Ok(r)) => Ok(CallToolResult::success(vec![Content::text(serde_json::to_string(&r)?)])),
            Ok(Err(e)) => Err(McpError::new(-32603, e)),
            Err(_) => Err(McpError::new(-32603, "Timeout")),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let graph = Arc::new(
        neo4rs::Graph::new(
            &std::env::var("NEO4J_URI").unwrap_or("bolt://localhost:7687".into()),
            &std::env::var("NEO4J_USER").unwrap_or("neo4j".into()),
            &std::env::var("NEO4J_PASSWORD").unwrap_or("password".into()),
        )
        .await?
    );

    let server = NovaNetServer::new(graph);
    let transport = StdioServerTransport::new();
    ServerHandler::new(server).run(transport).await?;
    Ok(())
}
```

---

## References

- Official MCP Spec: https://modelcontextprotocol.io
- rmcp Rust SDK: https://docs.rs/rmcp/
- Neo4j Rust Driver: https://docs.rs/neo4rs/
- Full Best Practices: See `2026-02-12-mcp-server-best-practices.md`
- Implementation Patterns: See `2026-02-12-mcp-server-patterns.md`
- Complete Roadmap: See `2026-02-12-novanet-mcp-roadmap.md`

