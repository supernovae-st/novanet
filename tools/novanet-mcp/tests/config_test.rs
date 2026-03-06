//! Tests for server configuration module
//!
//! Tests default values, environment variable parsing, and validation.

use novanet_mcp::server::config::Config;
use std::time::Duration;

// =============================================================================
// Default Value Tests
// =============================================================================

#[test]
fn test_default_neo4j_uri() {
    let config = Config::default();
    assert_eq!(config.neo4j_uri, "bolt://localhost:7687");
}

#[test]
fn test_default_neo4j_user() {
    let config = Config::default();
    assert_eq!(config.neo4j_user, "neo4j");
}

#[test]
fn test_default_neo4j_password() {
    let config = Config::default();
    assert_eq!(config.neo4j_password, "novanetpassword");
}

#[test]
fn test_default_pool_size() {
    let config = Config::default();
    assert_eq!(config.pool_size, 16);
}

#[test]
fn test_default_cache_max_entries() {
    let config = Config::default();
    assert_eq!(config.cache_max_entries, 10000);
}

#[test]
fn test_default_cache_ttl() {
    let config = Config::default();
    assert_eq!(config.cache_ttl, Duration::from_secs(300)); // 5 minutes
}

#[test]
fn test_default_token_budget() {
    let config = Config::default();
    assert_eq!(config.default_token_budget, 100_000);
}

#[test]
fn test_default_max_hops() {
    let config = Config::default();
    assert_eq!(config.max_hops, 5);
}

#[test]
fn test_default_evidence_packet_size() {
    let config = Config::default();
    assert_eq!(config.evidence_packet_size, 200);
}

// =============================================================================
// Config Clone Tests
// =============================================================================

#[test]
fn test_config_clone() {
    let config = Config {
        neo4j_uri: "bolt://custom:7688".to_string(),
        pool_size: 32,
        ..Config::default()
    };

    let cloned = config.clone();

    assert_eq!(cloned.neo4j_uri, "bolt://custom:7688");
    assert_eq!(cloned.pool_size, 32);
}

#[test]
fn test_config_debug() {
    let config = Config::default();
    let debug = format!("{:?}", config);

    assert!(debug.contains("Config"));
    assert!(debug.contains("neo4j_uri"));
    assert!(debug.contains("pool_size"));
}

// =============================================================================
// Environment Variable Tests
// =============================================================================

mod env_tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    // Use a mutex to ensure env var tests don't interfere with each other
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_env_vars<F, R>(vars: &[(&str, &str)], f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = ENV_LOCK.lock().unwrap();

        // Save original values and set new ones
        // SAFETY: We use a mutex to ensure only one test modifies env vars at a time
        let originals: Vec<_> = vars
            .iter()
            .map(|(k, v)| {
                let original = env::var(k).ok();
                unsafe { env::set_var(k, v) };
                (*k, original)
            })
            .collect();

        let result = f();

        // Restore original values
        // SAFETY: We use a mutex to ensure only one test modifies env vars at a time
        for (key, original) in originals {
            match original {
                Some(v) => unsafe { env::set_var(key, v) },
                None => unsafe { env::remove_var(key) },
            }
        }

        result
    }

    fn with_env_removed<F, R>(keys: &[&str], f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = ENV_LOCK.lock().unwrap();

        // Save original values and remove
        // SAFETY: We use a mutex to ensure only one test modifies env vars at a time
        let originals: Vec<_> = keys
            .iter()
            .map(|k| {
                let original = env::var(k).ok();
                unsafe { env::remove_var(k) };
                (*k, original)
            })
            .collect();

        let result = f();

        // Restore original values
        // SAFETY: We use a mutex to ensure only one test modifies env vars at a time
        for (key, original) in originals {
            if let Some(v) = original {
                unsafe { env::set_var(key, v) };
            }
        }

        result
    }

    #[test]
    fn test_from_env_requires_password() {
        with_env_removed(&["NOVANET_MCP_NEO4J_PASSWORD"], || {
            let result = Config::from_env();
            assert!(result.is_err());
            let err = result.unwrap_err().to_string();
            assert!(err.contains("NEO4J_PASSWORD"));
        });
    }

    #[test]
    fn test_from_env_with_password() {
        with_env_vars(&[("NOVANET_MCP_NEO4J_PASSWORD", "testpass")], || {
            let result = Config::from_env();
            assert!(result.is_ok());
            let config = result.unwrap();
            assert_eq!(config.neo4j_password, "testpass");
        });
    }

    #[test]
    fn test_from_env_custom_uri() {
        with_env_vars(
            &[
                ("NOVANET_MCP_NEO4J_PASSWORD", "pass"),
                ("NOVANET_MCP_NEO4J_URI", "bolt://custom:7688"),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.neo4j_uri, "bolt://custom:7688");
            },
        );
    }

    #[test]
    fn test_from_env_custom_user() {
        with_env_vars(
            &[
                ("NOVANET_MCP_NEO4J_PASSWORD", "pass"),
                ("NOVANET_MCP_NEO4J_USER", "admin"),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.neo4j_user, "admin");
            },
        );
    }

    #[test]
    fn test_from_env_custom_pool_size() {
        with_env_vars(
            &[
                ("NOVANET_MCP_NEO4J_PASSWORD", "pass"),
                ("NOVANET_MCP_NEO4J_POOL_SIZE", "32"),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.pool_size, 32);
            },
        );
    }

    #[test]
    fn test_from_env_invalid_pool_size_uses_default() {
        with_env_vars(
            &[
                ("NOVANET_MCP_NEO4J_PASSWORD", "pass"),
                ("NOVANET_MCP_NEO4J_POOL_SIZE", "not_a_number"),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.pool_size, 16); // Default
            },
        );
    }

    #[test]
    fn test_from_env_custom_cache_settings() {
        with_env_vars(
            &[
                ("NOVANET_MCP_NEO4J_PASSWORD", "pass"),
                ("NOVANET_MCP_CACHE_MAX_ENTRIES", "5000"),
                ("NOVANET_MCP_CACHE_TTL_SECS", "600"),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.cache_max_entries, 5000);
                assert_eq!(config.cache_ttl, Duration::from_secs(600));
            },
        );
    }

    #[test]
    fn test_from_env_custom_token_budget() {
        with_env_vars(
            &[
                ("NOVANET_MCP_NEO4J_PASSWORD", "pass"),
                ("NOVANET_MCP_DEFAULT_TOKEN_BUDGET", "50000"),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.default_token_budget, 50_000);
            },
        );
    }

    #[test]
    fn test_from_env_custom_max_hops() {
        with_env_vars(
            &[
                ("NOVANET_MCP_NEO4J_PASSWORD", "pass"),
                ("NOVANET_MCP_MAX_HOPS", "10"),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.max_hops, 10);
            },
        );
    }

    #[test]
    fn test_from_env_custom_evidence_packet_size() {
        with_env_vars(
            &[
                ("NOVANET_MCP_NEO4J_PASSWORD", "pass"),
                ("NOVANET_MCP_EVIDENCE_PACKET_SIZE", "500"),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.evidence_packet_size, 500);
            },
        );
    }
}

// =============================================================================
// Config Construction Tests
// =============================================================================

#[test]
fn test_config_struct_construction() {
    let config = Config {
        neo4j_uri: "bolt://test:7687".to_string(),
        neo4j_user: "testuser".to_string(),
        neo4j_password: "testpass".to_string(),
        pool_size: 8,
        // Phase 3 fields
        connection_timeout: Duration::from_secs(5),
        fetch_size: 500,
        circuit_breaker_threshold: 5,
        circuit_breaker_reset_timeout: Duration::from_secs(30),
        // Original fields
        cache_max_entries: 5000,
        cache_ttl: Duration::from_secs(60),
        default_token_budget: 50_000,
        max_hops: 3,
        evidence_packet_size: 100,
    };

    assert_eq!(config.neo4j_uri, "bolt://test:7687");
    assert_eq!(config.neo4j_user, "testuser");
    assert_eq!(config.neo4j_password, "testpass");
    assert_eq!(config.pool_size, 8);
    assert_eq!(config.cache_max_entries, 5000);
    assert_eq!(config.cache_ttl, Duration::from_secs(60));
    assert_eq!(config.default_token_budget, 50_000);
    assert_eq!(config.max_hops, 3);
    assert_eq!(config.evidence_packet_size, 100);
    // Phase 3 assertions
    assert_eq!(config.connection_timeout, Duration::from_secs(5));
    assert_eq!(config.fetch_size, 500);
    assert_eq!(config.circuit_breaker_threshold, 5);
    assert_eq!(config.circuit_breaker_reset_timeout, Duration::from_secs(30));
}

#[test]
fn test_config_partial_override() {
    let config = Config {
        pool_size: 64,
        max_hops: 10,
        ..Config::default()
    };

    // Overridden values
    assert_eq!(config.pool_size, 64);
    assert_eq!(config.max_hops, 10);

    // Default values preserved
    assert_eq!(config.neo4j_uri, "bolt://localhost:7687");
    assert_eq!(config.cache_max_entries, 10000);
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_config_with_zero_pool_size() {
    let config = Config {
        pool_size: 0,
        ..Config::default()
    };

    assert_eq!(config.pool_size, 0);
}

#[test]
fn test_config_with_large_cache() {
    let config = Config {
        cache_max_entries: u64::MAX,
        ..Config::default()
    };

    assert_eq!(config.cache_max_entries, u64::MAX);
}

#[test]
fn test_config_with_zero_ttl() {
    let config = Config {
        cache_ttl: Duration::ZERO,
        ..Config::default()
    };

    assert_eq!(config.cache_ttl, Duration::ZERO);
}

#[test]
fn test_config_with_empty_uri() {
    let config = Config {
        neo4j_uri: String::new(),
        ..Config::default()
    };

    assert!(config.neo4j_uri.is_empty());
}
