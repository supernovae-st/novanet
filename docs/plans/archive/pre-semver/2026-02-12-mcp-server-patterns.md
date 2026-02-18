# MCP Server Reference Patterns

**Date**: 2026-02-12
**Audience**: Rust developers building NovaNet MCP integrations
**Level**: Intermediate (assumes MCP basics understood)

---

## Pattern 1: Tool Router with Parameter Validation

The most common pattern for MCP servers is the `#[tool_router]` macro combined with strongly-typed parameter structs.

### Basic Structure

```rust
use rmcp::{tool, tool_router, ErrorData as McpError, model::*};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Define parameter struct with JSON Schema
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct QueryParams {
    /// Cypher query to execute
    pub cypher: String,

    /// Query parameters as key-value pairs
    #[serde(default)]
    pub params: serde_json::Value,

    /// Maximum execution time in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u32,

    /// Whether to explain the query plan
    #[serde(default)]
    pub explain: bool,
}

fn default_timeout() -> u32 {
    30000
}

// Server struct holds shared state
#[derive(Clone)]
pub struct NovaNetServer {
    graph: Arc<neo4rs::Graph>,
    cache: Arc<std::sync::Mutex<lru::LruCache<String, Vec<u8>>>>,
    tool_router: ToolRouter<Self>,
}

// Macro generates tool list and router
#[tool_router]
impl NovaNetServer {
    pub fn new(graph: Arc<neo4rs::Graph>) -> Self {
        Self {
            graph,
            cache: Arc::new(std::sync::Mutex::new(lru::LruCache::new(100))),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Execute a Cypher query")]
    async fn query(
        &self,
        params: Parameters<QueryParams>,
    ) -> Result<CallToolResult, McpError> {
        // Parse parameters
        let query_str = params.cypher.clone();
        let timeout = std::time::Duration::from_millis(params.timeout_ms as u64);

        // Execute with timeout
        match tokio::time::timeout(
            timeout,
            self.execute_query(&query_str, &params.params),
        )
        .await
        {
            Ok(Ok(result)) => {
                Ok(CallToolResult::success(vec![
                    Content::text(serde_json::to_string(&result)?)
                ]))
            }
            Ok(Err(e)) => {
                Err(McpError::new(-32603, format!("Query error: {}", e)))
            }
            Err(_) => {
                Err(McpError::new(-32603, "Query timeout"))
            }
        }
    }

    async fn execute_query(
        &self,
        cypher: &str,
        params: &serde_json::Value,
    ) -> anyhow::Result<Vec<std::collections::HashMap<String, String>>> {
        let mut query = self.graph.execute(neo4rs::query(cypher));

        // Bind parameters dynamically
        if let Some(obj) = params.as_object() {
            for (key, value) in obj {
                query = query.param(key, value);
            }
        }

        let mut result = query.await?;
        let mut rows = Vec::new();

        while let Some(row) = result.next().await? {
            let mut map = std::collections::HashMap::new();
            // Parse row into map
            rows.push(map);
        }

        Ok(rows)
    }
}
```

### Key Points

1. **Parameters Generic**: `Parameters<T>` automatically deserializes and validates JSON against `T`'s schema
2. **Error Type**: `Result<CallToolResult, McpError>` — use `McpError::new(code, message)`
3. **Tool Macro**: `#[tool(description = "...")]` on methods generates tool definitions
4. **Description Required**: Every parameter struct field MUST have a doc comment for LLM context

---

## Pattern 2: Resource Listing with Pagination

Resources are discovered via `resources/list` with cursor-based pagination.

```rust
#[tool_router]
impl NovaNetServer {
    #[tool(description = "List all available node kinds")]
    async fn list_node_kinds(
        &self,
        params: Parameters<ListParams>,
    ) -> Result<CallToolResult, McpError> {
        const PAGE_SIZE: usize = 50;

        // Parse cursor (format: "page:N")
        let page = params.cursor
            .as_ref()
            .and_then(|c| c.strip_prefix("page:"))
            .and_then(|p| p.parse::<usize>().ok())
            .unwrap_or(0);

        // Fetch all kinds (could be optimized with Neo4j SKIP/LIMIT)
        let kinds = self.fetch_all_node_kinds().await?;

        // Paginate
        let start = page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(kinds.len());
        let page_items = &kinds[start..end];

        // Build response
        let resources: Vec<Resource> = page_items
            .iter()
            .map(|k| Resource {
                uri: format!("neo4j://novanet/kind/node/{}", k.name),
                name: k.name.clone(),
                title: Some(k.display_name.clone()),
                description: Some(k.description.clone()),
                mime_type: "application/json".into(),
                icons: None,
            })
            .collect();

        let next_cursor = if end < kinds.len() {
            Some(format!("page:{}", page + 1))
        } else {
            None
        };

        Ok(CallToolResult::success(vec![
            Content::json(serde_json::json!({
                "resources": resources,
                "nextCursor": next_cursor,
            }))
        ]))
    }

    async fn fetch_all_node_kinds(&self) -> anyhow::Result<Vec<NodeKindMetadata>> {
        let mut result = self.graph
            .execute(neo4rs::query("MATCH (k:Kind) RETURN k ORDER BY k.name"))
            .await?;

        let mut kinds = Vec::new();
        while let Some(row) = result.next().await? {
            let kind: NodeKindMetadata = row.get("k")?;
            kinds.push(kind);
        }

        Ok(kinds)
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ListParams {
    /// Pagination cursor from previous response
    #[serde(default)]
    pub cursor: Option<String>,
}
```

**For NovaNet**, implement paging for:
- `list_node_kinds`: All 60 NodeKind definitions
- `list_arc_kinds`: All 114 ArcKind definitions
- `list_instances`: All data instances for a given Kind (could be 100K+)

---

## Pattern 3: Resource Reading with Different MIME Types

Resources can return different formats based on client request.

```rust
#[tool_router]
impl NovaNetServer {
    #[tool(description = "Get detailed definition of a node kind")]
    async fn read_node_kind(
        &self,
        params: Parameters<ReadKindParams>,
    ) -> Result<CallToolResult, McpError> {
        let kind_name = &params.kind_name;
        let format = params.format.as_deref().unwrap_or("json");

        // Fetch from Neo4j
        let kind = self.get_node_kind(kind_name).await
            .map_err(|e| McpError::new(-32603, e.to_string()))?;

        // Format response
        let content = match format {
            "json" => {
                // Full structured representation
                Content::json(serde_json::to_value(&kind)?)
            }
            "yaml" => {
                // YAML from source file
                let yaml = self.load_kind_yaml(kind_name).await?;
                Content::text(yaml)
            }
            "typescript" => {
                // Generated TypeScript type
                let ts = self.generate_typescript_type(&kind).await?;
                Content::text(format!("```typescript\n{}\n```", ts))
            }
            "mermaid" => {
                // Mermaid diagram
                let diagram = self.generate_mermaid_diagram(&kind).await?;
                Content::text(format!("```mermaid\n{}\n```", diagram))
            }
            _ => {
                return Err(McpError::new(
                    -32602,
                    format!("Unsupported format: {}", format),
                ));
            }
        };

        Ok(CallToolResult::success(vec![content]))
    }

    async fn get_node_kind(&self, name: &str) -> anyhow::Result<NodeKindDefinition> {
        let mut result = self.graph
            .execute(neo4rs::query("MATCH (k:Kind {name: $name}) RETURN k")
                .param("name", name))
            .await?;

        result.next()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Kind not found: {}", name))
            .and_then(|row| Ok(row.get("k")?))
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ReadKindParams {
    /// Name of the node kind
    pub kind_name: String,

    /// Response format: json, yaml, typescript, mermaid
    #[serde(default)]
    pub format: Option<String>,
}
```

---

## Pattern 4: Streaming Results

For large result sets, return results in chunks to avoid memory issues.

```rust
#[tool_router]
impl NovaNetServer {
    #[tool(description = "Stream large query results")]
    async fn query_stream(
        &self,
        params: Parameters<StreamQueryParams>,
    ) -> Result<CallToolResult, McpError> {
        let cypher = params.cypher.clone();
        let chunk_size = params.chunk_size.unwrap_or(1000);

        let mut result = self.graph
            .execute(neo4rs::query(&cypher))
            .await
            .map_err(|e| McpError::new(-32603, e.to_string()))?;

        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();

        while let Some(row) = result.next().await
            .map_err(|e| McpError::new(-32603, e.to_string()))?
        {
            current_chunk.push(row);

            if current_chunk.len() >= chunk_size {
                chunks.push(Content::text(serde_json::to_string(&current_chunk)?));
                current_chunk.clear();
            }
        }

        // Final chunk
        if !current_chunk.is_empty() {
            chunks.push(Content::text(serde_json::to_string(&current_chunk)?));
        }

        Ok(CallToolResult::success(chunks))
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct StreamQueryParams {
    pub cypher: String,
    /// Results per chunk
    #[serde(default)]
    pub chunk_size: Option<usize>,
}
```

---

## Pattern 5: Caching Expensive Computations

Use `Arc<Mutex>` or `Arc<RwLock>` for thread-safe cache:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CachedNovaNetServer {
    graph: Arc<neo4rs::Graph>,

    // Cache: Kind name → Schema definition (read-heavy)
    schemas: Arc<RwLock<HashMap<String, NodeKindDefinition>>>,

    // Cache: Layer name → Kinds in that layer (read-heavy)
    layer_kinds: Arc<RwLock<HashMap<String, Vec<String>>>>,

    tool_router: ToolRouter<Self>,
}

impl CachedNovaNetServer {
    pub async fn get_node_kind_cached(
        &self,
        kind_name: &str,
    ) -> anyhow::Result<NodeKindDefinition> {
        // Try cache first (read lock, doesn't block other readers)
        {
            let schemas = self.schemas.read().await;
            if let Some(schema) = schemas.get(kind_name) {
                return Ok(schema.clone());
            }
        }

        // Cache miss: fetch from database
        let schema = self.get_node_kind_uncached(kind_name).await?;

        // Update cache (exclusive write lock)
        {
            let mut schemas = self.schemas.write().await;
            schemas.insert(kind_name.to_string(), schema.clone());
        }

        Ok(schema)
    }

    async fn get_node_kind_uncached(&self, name: &str) -> anyhow::Result<NodeKindDefinition> {
        // Actual database query
        todo!()
    }

    // Add cache invalidation
    pub async fn invalidate_kind_cache(&self, kind_name: &str) {
        let mut schemas = self.schemas.write().await;
        schemas.remove(kind_name);
    }

    pub async fn invalidate_all_caches(&self) {
        let mut schemas = self.schemas.write().await;
        schemas.clear();

        let mut kinds = self.layer_kinds.write().await;
        kinds.clear();
    }
}
```

**Cache Invalidation Strategies:**

1. **Time-based**: Auto-expire after 1 hour
2. **Event-based**: Invalidate when schema/data changes
3. **Manual**: Expose tool to clear cache
4. **Watch pattern**: Use Neo4j changefeeds if available

---

## Pattern 6: Error Recovery and Resilience

Handle database failures gracefully:

```rust
use std::time::Duration;

#[derive(Clone)]
pub struct ResilientNovaNetServer {
    graph: Arc<neo4rs::Graph>,
    retry_policy: RetryPolicy,
    tool_router: ToolRouter<Self>,
}

#[derive(Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl RetryPolicy {
    fn calculate_backoff(&self, attempt: u32) -> Duration {
        let delay = self.base_delay_ms * 2_u64.pow(attempt);
        let delay = delay.min(self.max_delay_ms);
        Duration::from_millis(delay)
    }
}

#[tool_router]
impl ResilientNovaNetServer {
    #[tool(description = "Execute query with automatic retries")]
    async fn query_resilient(
        &self,
        params: Parameters<QueryParams>,
    ) -> Result<CallToolResult, McpError> {
        for attempt in 0..self.retry_policy.max_retries {
            match self.execute_query_attempt(&params.cypher, &params.params).await {
                Ok(result) => {
                    tracing::info!(attempt, "Query succeeded");
                    return Ok(CallToolResult::success(vec![
                        Content::text(serde_json::to_string(&result)?)
                    ]));
                }
                Err(e) if is_transient_error(&e) && attempt < self.retry_policy.max_retries - 1 => {
                    let backoff = self.retry_policy.calculate_backoff(attempt);
                    tracing::warn!(attempt, ?backoff, %e, "Transient error, retrying");
                    tokio::time::sleep(backoff).await;
                    continue;
                }
                Err(e) => {
                    tracing::error!(%e, "Query failed (non-transient or max retries)");
                    return Err(McpError::new(-32603, format!("Query error: {}", e)));
                }
            }
        }

        Err(McpError::new(-32603, "Max retries exceeded"))
    }

    async fn execute_query_attempt(
        &self,
        cypher: &str,
        params: &serde_json::Value,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        // Actual query execution
        todo!()
    }
}

fn is_transient_error(e: &anyhow::Error) -> bool {
    let msg = e.to_string().to_lowercase();
    msg.contains("timeout") ||
    msg.contains("connection") ||
    msg.contains("pool") ||
    msg.contains("temporarily")
}
```

---

## Pattern 7: Structured Logging

Use `tracing` crate for structured, filterable logs:

```rust
use tracing::{info, warn, error, debug, span, Level, Instrument};

#[tool_router]
impl NovaNetServer {
    #[tool(description = "Query with structured logging")]
    async fn query_logged(
        &self,
        params: Parameters<QueryParams>,
    ) -> Result<CallToolResult, McpError> {
        let query_id = uuid::Uuid::new_v4();
        let span = span!(Level::INFO, "query_execution", %query_id);

        async {
            debug!(query = %params.cypher, "Query started");

            let start = std::time::Instant::now();
            match self.execute_query(&params.cypher, &params.params).await {
                Ok(result) => {
                    let elapsed = start.elapsed();
                    info!(
                        duration_ms = elapsed.as_millis() as u64,
                        result_count = result.len(),
                        "Query completed successfully"
                    );
                    Ok(CallToolResult::success(vec![
                        Content::text(serde_json::to_string(&result)?)
                    ]))
                }
                Err(e) => {
                    let elapsed = start.elapsed();
                    error!(
                        error = %e,
                        duration_ms = elapsed.as_millis() as u64,
                        "Query failed"
                    );
                    Err(McpError::new(-32603, format!("Query error: {}", e)))
                }
            }
        }
        .instrument(span)
        .await
    }
}

// Initialize logging at startup
pub fn init_logging() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env()
            .add_directive("novanet_mcp=info".parse().unwrap())
            .add_directive("rmcp=debug".parse().unwrap()))
        .with(fmt::layer()
            .with_writer(std::io::stderr)
            .json()
            .with_target(true)
            .with_thread_ids(true))
        .init();
}
```

**Usage:**
```bash
RUST_LOG=novanet_mcp=debug cargo run -- --server
# Output: {"level":"INFO","message":"Query completed","query_id":"..."}
```

---

## Pattern 8: Protocol Compliance Testing

Test that your server properly implements MCP:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::model::*;

    #[tokio::test]
    async fn test_initialize_response() {
        let server = NovaNetServer::new(Arc::new(/* mock graph */));

        // Verify capabilities are declared
        let capabilities = server.get_capabilities();
        assert!(capabilities.logging.is_some());
        assert!(capabilities.resources.is_some());
        assert!(capabilities.tools.is_some());
    }

    #[tokio::test]
    async fn test_error_code_format() {
        let server = NovaNetServer::new(Arc::new(/* mock graph */));

        // Test that errors use correct JSON-RPC error codes
        let result = server.query(Parameters::new(QueryParams {
            cypher: "INVALID CYPHER".into(),
            params: serde_json::json!({}),
            timeout_ms: 5000,
            explain: false,
        })).await;

        match result {
            Err(e) => {
                assert_eq!(e.code, -32603);  // Internal error
                assert!(!e.message.is_empty());
            }
            _ => panic!("Expected error"),
        }
    }

    #[tokio::test]
    async fn test_pagination() {
        let server = NovaNetServer::new(Arc::new(/* mock graph */));

        // Test first page
        let page1 = server.list_node_kinds(Parameters::new(ListParams {
            cursor: None,
        })).await.unwrap();

        // Should have next cursor if more items
        // ...

        // Test second page
        let page2 = server.list_node_kinds(Parameters::new(ListParams {
            cursor: Some("page:1".into()),
        })).await.unwrap();

        // No overlap between pages
        // ...
    }
}
```

---

## Pattern 9: Tool Composition

Complex queries built from simpler tools:

```rust
#[tool_router]
impl NovaNetServer {
    #[tool(description = "Analyze entity relationships")]
    async fn analyze_entity(
        &self,
        params: Parameters<AnalyzeEntityParams>,
    ) -> Result<CallToolResult, McpError> {
        // Use composition: fetch kind → fetch instances → analyze
        let kind = self.query(Parameters::new(QueryParams {
            cypher: format!(
                "MATCH (k:Kind {{name: '{}'}}) RETURN k",
                params.entity_name
            ),
            params: serde_json::json!({}),
            timeout_ms: 5000,
            explain: false,
        })).await?;

        let instances = self.query(Parameters::new(QueryParams {
            cypher: format!(
                "MATCH (e:{}) RETURN e LIMIT 100",
                params.entity_name
            ),
            params: serde_json::json!({}),
            timeout_ms: 5000,
            explain: false,
        })).await?;

        let analysis = self.perform_analysis(&kind, &instances).await?;

        Ok(CallToolResult::success(vec![
            Content::text(serde_json::to_string(&analysis)?)
        ]))
    }

    async fn perform_analysis(
        &self,
        kind: &CallToolResult,
        instances: &CallToolResult,
    ) -> anyhow::Result<serde_json::Value> {
        // Analysis logic
        todo!()
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AnalyzeEntityParams {
    pub entity_name: String,
}
```

---

## Pattern 10: Configuration Management

Load configuration from environment and files:

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub neo4j: Neo4jConfig,
    pub logging: LoggingConfig,
    pub performance: PerformanceConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Neo4jConfig {
    pub uri: String,
    pub user: String,
    pub password: String,
    pub connection_pool_size: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PerformanceConfig {
    pub query_timeout_ms: u32,
    pub cache_size: usize,
    pub cache_ttl_secs: u64,
}

impl ServerConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            neo4j: Neo4jConfig {
                uri: std::env::var("NEO4J_URI").unwrap_or("bolt://localhost:7687".into()),
                user: std::env::var("NEO4J_USER").unwrap_or("neo4j".into()),
                password: std::env::var("NEO4J_PASSWORD").unwrap_or("password".into()),
                connection_pool_size: std::env::var("NEO4J_POOL_SIZE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10),
            },
            logging: LoggingConfig {
                level: std::env::var("RUST_LOG").unwrap_or("info".into()),
                format: std::env::var("LOG_FORMAT").unwrap_or("json".into()),
            },
            performance: PerformanceConfig {
                query_timeout_ms: std::env::var("QUERY_TIMEOUT_MS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30000),
                cache_size: std::env::var("CACHE_SIZE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1000),
                cache_ttl_secs: std::env::var("CACHE_TTL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600),
            },
        })
    }

    pub fn from_file(path: PathBuf) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServerConfig::from_env()?;
    // Use config...
    Ok(())
}
```

---

## Summary of Patterns

| Pattern | Use Case | Key Benefit |
|---------|----------|------------|
| Tool Router | All tools | Type safety + auto-registration |
| Resource Listing | Browse schema/data | Client discovery |
| Pagination | Large result sets | Memory efficient |
| Format Negotiation | Different output types | Flexibility |
| Streaming | Huge queries | Non-blocking |
| Caching | Expensive computations | Performance |
| Retry Logic | Transient failures | Resilience |
| Structured Logging | Observability | Easy debugging |
| Composition | Complex queries | Code reuse |
| Configuration | Environment setup | Deployment flexibility |

