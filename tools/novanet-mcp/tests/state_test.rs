//! Tests for server state module
//!
//! Tests ServerStats, State accessors, and concurrent access patterns.

use novanet_mcp::server::ServerStats;

// =============================================================================
// ServerStats Tests
// =============================================================================

#[test]
fn test_server_stats_default() {
    let stats = ServerStats::default();
    assert_eq!(stats.queries_executed, 0);
    assert_eq!(stats.cache_hits, 0);
    assert_eq!(stats.cache_misses, 0);
}

#[test]
fn test_server_stats_clone() {
    let mut stats = ServerStats::default();
    stats.queries_executed = 42;
    stats.cache_hits = 10;
    stats.cache_misses = 5;

    let cloned = stats.clone();
    assert_eq!(cloned.queries_executed, 42);
    assert_eq!(cloned.cache_hits, 10);
    assert_eq!(cloned.cache_misses, 5);
}

#[test]
fn test_server_stats_debug() {
    let stats = ServerStats::default();
    let debug = format!("{:?}", stats);
    assert!(debug.contains("ServerStats"));
    assert!(debug.contains("queries_executed"));
}

// =============================================================================
// State Integration Tests (require Neo4j connection)
// =============================================================================

// Note: Full State tests require Neo4j connection.
// Run with: cargo test --test state_test --features test-neo4j

#[cfg(feature = "test-neo4j")]
mod state_integration {
    use novanet_mcp::server::{Config, State};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_state_record_query_increments() {
        let config = Config::default();
        let state = State::new(config).await.unwrap();

        // Initial stats
        assert_eq!(state.stats().queries_executed, 0);

        // Record queries
        state.record_query();
        assert_eq!(state.stats().queries_executed, 1);

        state.record_query();
        state.record_query();
        assert_eq!(state.stats().queries_executed, 3);
    }

    #[tokio::test]
    async fn test_state_record_cache_hit_increments() {
        let config = Config::default();
        let state = State::new(config).await.unwrap();

        assert_eq!(state.stats().cache_hits, 0);

        state.record_cache_hit();
        assert_eq!(state.stats().cache_hits, 1);

        state.record_cache_hit();
        state.record_cache_hit();
        assert_eq!(state.stats().cache_hits, 3);
    }

    #[tokio::test]
    async fn test_state_record_cache_miss_increments() {
        let config = Config::default();
        let state = State::new(config).await.unwrap();

        assert_eq!(state.stats().cache_misses, 0);

        state.record_cache_miss();
        assert_eq!(state.stats().cache_misses, 1);

        state.record_cache_miss();
        assert_eq!(state.stats().cache_misses, 2);
    }

    #[tokio::test]
    async fn test_state_config_accessor() {
        let config = Config {
            pool_size: 8,
            max_hops: 10,
            ..Config::default()
        };
        let state = State::new(config).await.unwrap();

        assert_eq!(state.config().pool_size, 8);
        assert_eq!(state.config().max_hops, 10);
    }

    #[tokio::test]
    async fn test_state_clone_shares_stats() {
        let config = Config::default();
        let state1 = State::new(config).await.unwrap();
        let state2 = state1.clone();

        state1.record_query();
        // Both should see the same stats (Arc-shared)
        assert_eq!(state2.stats().queries_executed, 1);
    }

    #[tokio::test]
    async fn test_state_concurrent_stats_access() {
        use tokio::task::JoinSet;

        let config = Config::default();
        let state = Arc::new(State::new(config).await.unwrap());

        let mut set = JoinSet::new();

        // Spawn 100 tasks that each record a query
        for _ in 0..100 {
            let s = Arc::clone(&state);
            set.spawn(async move {
                s.record_query();
            });
        }

        // Wait for all tasks
        while let Some(result) = set.join_next().await {
            result.unwrap();
        }

        // All 100 should be recorded
        assert_eq!(state.stats().queries_executed, 100);
    }
}

// =============================================================================
// Unit tests without Neo4j
// =============================================================================

#[test]
fn test_stats_can_be_created_and_modified() {
    // Verify the struct fields are accessible
    let mut stats = ServerStats {
        queries_executed: 0,
        cache_hits: 0,
        cache_misses: 0,
    };

    stats.queries_executed = 100;
    stats.cache_hits = 50;
    stats.cache_misses = 25;

    assert_eq!(stats.queries_executed, 100);
    assert_eq!(stats.cache_hits, 50);
    assert_eq!(stats.cache_misses, 25);
}

#[test]
fn test_stats_ratio_calculation() {
    let stats = ServerStats {
        queries_executed: 100,
        cache_hits: 75,
        cache_misses: 25,
    };

    let hit_rate = stats.cache_hits as f64 / (stats.cache_hits + stats.cache_misses) as f64;
    assert!((hit_rate - 0.75).abs() < 0.001);
}

#[test]
fn test_stats_with_zero_cache_operations() {
    let stats = ServerStats {
        queries_executed: 10,
        cache_hits: 0,
        cache_misses: 0,
    };

    // Should handle zero division gracefully in client code
    let total_cache_ops = stats.cache_hits + stats.cache_misses;
    assert_eq!(total_cache_ops, 0);
}
