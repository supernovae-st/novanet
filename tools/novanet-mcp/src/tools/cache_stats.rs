//! Cache statistics and invalidation tools
//!
//! MCP tools for monitoring and managing the query cache.
//! Task A3: Add Cache Statistics and Manual Invalidation

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for cache stats retrieval
#[derive(Debug, Clone, Default, Deserialize, Serialize, JsonSchema)]
pub struct CacheStatsParams {
    /// Include detailed entry list (default: false)
    /// Note: Currently not implemented, reserved for future use
    #[serde(default)]
    pub detailed: bool,
}

/// Cache statistics response
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct CacheStats {
    /// Number of cache hits since server start
    pub hits: u64,
    /// Number of cache misses since server start
    pub misses: u64,
    /// Current number of entries in cache
    pub entries: u64,
    /// Hit rate as a decimal (0.0 to 1.0)
    pub hit_rate: f64,
    /// Approximate memory usage in bytes
    pub size_bytes: u64,
    /// Maximum number of entries configured
    pub max_entries: u64,
    /// Time-to-live in seconds
    pub ttl_seconds: u64,
}

/// Parameters for cache invalidation
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CacheInvalidateParams {
    /// Invalidate specific key pattern (optional)
    /// Note: Pattern matching invalidates all entries containing this substring
    #[serde(default)]
    pub pattern: Option<String>,
    /// Invalidate all entries (default: false)
    #[serde(default)]
    pub all: bool,
}

/// Cache invalidation result
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct CacheInvalidateResult {
    /// Number of entries invalidated
    pub invalidated: u64,
    /// Number of entries remaining after invalidation
    pub remaining: u64,
}

/// Get cache statistics
///
/// Returns current cache metrics including hit rate, entry count, and memory usage.
pub async fn get_stats(state: &State, _params: CacheStatsParams) -> Result<CacheStats> {
    let cache = state.cache();
    let server_stats = state.stats();

    let hits = server_stats.cache_hits;
    let misses = server_stats.cache_misses;
    let total = hits + misses;

    Ok(CacheStats {
        hits,
        misses,
        entries: cache.entry_count(),
        hit_rate: if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        },
        size_bytes: cache.weighted_size(),
        max_entries: cache.max_capacity(),
        ttl_seconds: state.config().cache_ttl.as_secs(),
    })
}

/// Invalidate cache entries
///
/// Supports full cache clear or pattern-based invalidation.
pub async fn invalidate(state: &State, params: CacheInvalidateParams) -> Result<CacheInvalidateResult> {
    let cache = state.cache();
    let before = cache.entry_count();

    if params.all {
        cache.invalidate_all().await;
    } else if let Some(pattern) = params.pattern {
        cache.invalidate_matching(&pattern).await;
    }

    // Force sync to get accurate count
    cache.run_pending_tasks().await;
    let after = cache.entry_count();

    Ok(CacheInvalidateResult {
        invalidated: before.saturating_sub(after),
        remaining: after,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_stats_struct() {
        let stats = CacheStats {
            hits: 100,
            misses: 50,
            entries: 25,
            hit_rate: 0.67,
            size_bytes: 1024,
            max_entries: 10000,
            ttl_seconds: 300,
        };
        assert_eq!(stats.hit_rate, 0.67);
        assert_eq!(stats.entries, 25);
    }

    #[test]
    fn test_cache_stats_params_default() {
        let params: CacheStatsParams = serde_json::from_str("{}").unwrap();
        assert!(!params.detailed);
    }

    #[test]
    fn test_cache_invalidate_params_all() {
        let params: CacheInvalidateParams = serde_json::from_str(r#"{"all": true}"#).unwrap();
        assert!(params.all);
        assert!(params.pattern.is_none());
    }

    #[test]
    fn test_cache_invalidate_params_pattern() {
        let params: CacheInvalidateParams =
            serde_json::from_str(r#"{"pattern": "entity:*"}"#).unwrap();
        assert!(!params.all);
        assert_eq!(params.pattern, Some("entity:*".to_string()));
    }

    #[test]
    fn test_cache_stats_serialization() {
        let stats = CacheStats {
            hits: 100,
            misses: 50,
            entries: 25,
            hit_rate: 0.6666666666666666,
            size_bytes: 1024,
            max_entries: 10000,
            ttl_seconds: 300,
        };
        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("\"hits\":100"));
        assert!(json.contains("\"misses\":50"));
    }

    #[test]
    fn test_cache_invalidate_result_serialization() {
        let result = CacheInvalidateResult {
            invalidated: 10,
            remaining: 15,
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"invalidated\":10"));
        assert!(json.contains("\"remaining\":15"));
    }
}
