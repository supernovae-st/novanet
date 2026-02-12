//! NovaNet MCP Server
//!
//! MCP (Model Context Protocol) server that exposes the NovaNet knowledge graph
//! to AI agents. Implements RLM-on-KG (Recursive Language Model on Knowledge Graph)
//! patterns for efficient context assembly.
//!
//! # Architecture
//!
//! - 7 Tools: search, traverse, assemble, atoms, describe, query, generate
//! - 4 Resources: entity://, kind://, locale://, view://
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

pub mod cache;
pub mod error;
pub mod neo4j;
pub mod prompts;
pub mod resources;
pub mod rlm;
pub mod server;
pub mod tokens;
pub mod tools;

// Re-export main types
pub use error::{Error, Result};
pub use server::{Config, Server, State};
