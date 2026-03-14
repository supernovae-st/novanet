//! NovaNet MCP Server
//!
//! MCP (Model Context Protocol) server that exposes the NovaNet knowledge graph
//! to AI agents. Implements RLM-on-KG (Recursive Language Model on Knowledge Graph)
//! patterns for efficient context assembly.
//!
//! # Architecture (v0.20.0 — The Great Cleanup)
//!
//! - 8 Tools: query, describe, search, introspect, context, write, audit, batch
//! - 4 Resources: entity://, class://, locale://, view://
//! - 6 Prompts: cypher_query, cypher_explain, block_generation, page_generation,
//!   entity_analysis, locale_briefing
//!
//! # Example
//!
//! ```rust,ignore
//! use novanet_mcp::{Server, Config};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config::from_env()?;
//!     let server = Server::new(config).await?;
//!     server.run().await
//! }
//! ```

// Public modules (re-exported or used by integration tests):
pub mod cache;
pub mod error;
pub mod neo4j;
pub mod prompts;
pub mod resources;
pub mod server;
pub mod tokens;
pub mod tools;

// Internal modules — pub(crate) only:
pub(crate) mod activation;
pub(crate) mod hints;
pub(crate) mod metrics;
pub(crate) mod schema_cache;
pub(crate) mod validation;

// Re-export main types
pub use error::{Error, Result};
pub use server::{Config, Server, State};
