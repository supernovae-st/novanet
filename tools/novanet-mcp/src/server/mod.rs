//! Server module
//!
//! Contains the MCP server implementation using rmcp.

mod config;
mod handler;
mod state;

pub use config::Config;
pub use handler::NovaNetHandler;
pub use state::State;

use crate::error::Result;
use rmcp::ServiceExt;
use rmcp::transport::stdio;

/// NovaNet MCP Server
pub struct Server {
    state: State,
}

impl Server {
    /// Create a new server with the given configuration
    pub async fn new(config: Config) -> Result<Self> {
        let state = State::new(config).await?;
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
