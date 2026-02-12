//! NovaNet MCP Server - Entry Point
//!
//! Starts the MCP server with stdio transport for Claude Code integration.

use anyhow::Result;
use novanet_mcp::{Config, Server};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("novanet_mcp=info")),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    tracing::info!("NovaNet MCP Server starting...");

    // Load configuration from environment
    let config = Config::from_env()?;
    tracing::info!(
        neo4j_uri = %config.neo4j_uri,
        pool_size = config.pool_size,
        "Configuration loaded"
    );

    // Create and run server
    let server = Server::new(config).await?;
    tracing::info!("Server initialized, starting MCP transport...");

    Ok(server.run().await?)
}
