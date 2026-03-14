//! Server configuration
//!
//! Configuration loaded from environment variables.
//!
//! ## Phase 3 Performance Optimization
//!
//! Added connection pool tuning and circuit breaker settings for improved
//! resilience under load.
//!
//! ## Phase 2.2 Spreading Activation
//!
//! Added optional spreading-activation.yaml path for context assembly.

use crate::error::{Error, Result};
use std::path::PathBuf;
use std::time::Duration;

/// Parse an environment variable with a default value.
/// Pattern: read env var -> parse to T -> fallback to default.
fn env_or<T: std::str::FromStr>(var: &str, default: T) -> T {
    std::env::var(var)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct Config {
    // Neo4j settings
    pub neo4j_uri: String,
    pub neo4j_user: String,
    pub neo4j_password: String,
    pub pool_size: usize,

    // Connection pool tuning (Phase 3)
    pub fetch_size: usize,

    // Retry settings
    pub max_retries: u32,
    pub retry_base_delay: Duration,

    // Circuit breaker settings (Phase 3)
    pub circuit_breaker_threshold: u32,
    pub circuit_breaker_reset_timeout: Duration,

    // Cache settings
    pub cache_max_entries: u64,
    pub cache_ttl: Duration,

    // Token settings
    pub default_token_budget: usize,
    pub max_hops: u8,
    pub evidence_packet_size: usize,

    // Spreading activation config (Phase 2.2)
    /// Path to spreading-activation.yaml (optional, defaults to hardcoded values)
    pub spreading_config_path: Option<PathBuf>,
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
            pool_size: env_or("NOVANET_MCP_NEO4J_POOL_SIZE", 16),

            // Connection pool tuning (Phase 3)
            fetch_size: env_or("NOVANET_MCP_FETCH_SIZE", 500),

            // Retry settings
            max_retries: env_or("NOVANET_MCP_MAX_RETRIES", 3),
            retry_base_delay: Duration::from_millis(
                env_or("NOVANET_MCP_RETRY_BASE_DELAY_MS", 100),
            ),

            // Circuit breaker settings (Phase 3)
            circuit_breaker_threshold: env_or("NOVANET_MCP_CIRCUIT_BREAKER_THRESHOLD", 5),
            circuit_breaker_reset_timeout: Duration::from_secs(
                env_or("NOVANET_MCP_CIRCUIT_BREAKER_RESET_SECS", 30),
            ),

            // Cache
            cache_max_entries: env_or("NOVANET_MCP_CACHE_MAX_ENTRIES", 10000),
            cache_ttl: Duration::from_secs(
                env_or("NOVANET_MCP_CACHE_TTL_SECS", 300),
            ),

            // Token settings
            default_token_budget: env_or("NOVANET_MCP_DEFAULT_TOKEN_BUDGET", 100_000),
            max_hops: env_or("NOVANET_MCP_MAX_HOPS", 5),
            evidence_packet_size: env_or("NOVANET_MCP_EVIDENCE_PACKET_SIZE", 200),

            // Spreading activation config (Phase 2.2)
            spreading_config_path: std::env::var("NOVANET_MCP_SPREADING_CONFIG_PATH")
                .ok()
                .map(PathBuf::from),
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
            // Connection pool tuning (Phase 3)
            fetch_size: 500,
            // Retry settings
            max_retries: 3,
            retry_base_delay: Duration::from_millis(100),
            // Circuit breaker (Phase 3)
            circuit_breaker_threshold: 5,
            circuit_breaker_reset_timeout: Duration::from_secs(30),
            // Cache
            cache_max_entries: 10000,
            cache_ttl: Duration::from_secs(300),
            // Token settings
            default_token_budget: 100_000,
            max_hops: 5,
            evidence_packet_size: 200,
            // Spreading activation (Phase 2.2)
            spreading_config_path: None,
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
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_base_delay, Duration::from_millis(100));
    }
}
