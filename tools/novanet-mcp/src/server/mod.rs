//! Server module
//!
//! Contains the MCP server implementation using rmcp.

mod handler;

pub mod config;
pub mod state;

pub use config::Config;
pub use handler::NovaNetHandler;
pub use state::{StatsSnapshot, State};

use crate::error::Result;
use rmcp::ServiceExt;
use rmcp::transport::stdio;

/// NovaNet MCP Server
pub struct Server {
    state: State,
}

impl Server {
    /// Create a new server with the given configuration
    ///
    /// Initializes the Neo4j connection pool, cache, and warms the cache
    /// with frequently-used schema queries.
    pub async fn new(config: Config) -> Result<Self> {
        let state = State::new(config).await?;

        // Warm cache with frequently-used schema queries (Phase 1 optimization)
        if let Err(e) = state.warm_cache().await {
            tracing::warn!(error = %e, "Cache warming failed, continuing with cold cache");
        }

        Ok(Self { state })
    }

    /// Run the server with stdio transport
    pub async fn run(self) -> Result<()> {
        let handler = NovaNetHandler::new(self.state);

        // Create stdio transport and run the MCP server
        let server = handler
            .serve(stdio())
            .await
            .map_err(|e| crate::error::Error::Mcp(e.to_string()))?;

        // Wait for completion
        server
            .waiting()
            .await
            .map_err(|e| crate::error::Error::Mcp(e.to_string()))?;

        Ok(())
    }
}
