//! Server configuration
//!
//! Configuration loaded from environment variables.

use crate::error::{Error, Result};
use std::time::Duration;

/// Server configuration
#[derive(Debug, Clone)]
pub struct Config {
    // Neo4j settings
    pub neo4j_uri: String,
    pub neo4j_user: String,
    pub neo4j_password: String,
    pub pool_size: usize,

    // Cache settings
    pub cache_max_entries: u64,
    pub cache_ttl: Duration,

    // Token settings
    pub default_token_budget: usize,
    pub max_hops: u8,
    pub evidence_packet_size: usize,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            // Neo4j
            neo4j_uri: std::env::var("NOVANET_MCP_NEO4J_URI")
                .unwrap_or_else(|_| "bolt://localhost:7687".to_string()),
            neo4j_user: std::env::var("NOVANET_MCP_NEO4J_USER")
                .unwrap_or_else(|_| "neo4j".to_string()),
            neo4j_password: std::env::var("NOVANET_MCP_NEO4J_PASSWORD")
                .map_err(|_| Error::Config("NOVANET_MCP_NEO4J_PASSWORD not set".to_string()))?,
            pool_size: std::env::var("NOVANET_MCP_NEO4J_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(16),

            // Cache
            cache_max_entries: std::env::var("NOVANET_MCP_CACHE_MAX_ENTRIES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10000),
            cache_ttl: Duration::from_secs(
                std::env::var("NOVANET_MCP_CACHE_TTL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300),
            ),

            // Token settings
            default_token_budget: std::env::var("NOVANET_MCP_DEFAULT_TOKEN_BUDGET")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100_000),
            max_hops: std::env::var("NOVANET_MCP_MAX_HOPS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            evidence_packet_size: std::env::var("NOVANET_MCP_EVIDENCE_PACKET_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(200),
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            neo4j_uri: "bolt://localhost:7687".to_string(),
            neo4j_user: "neo4j".to_string(),
            neo4j_password: "novanetpassword".to_string(),
            pool_size: 16,
            cache_max_entries: 10000,
            cache_ttl: Duration::from_secs(300),
            default_token_budget: 100_000,
            max_hops: 5,
            evidence_packet_size: 200,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.pool_size, 16);
        assert_eq!(config.max_hops, 5);
        assert_eq!(config.evidence_packet_size, 200);
    }
}
