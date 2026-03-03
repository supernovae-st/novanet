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
    /// Note: Pattern-based invalidation is NOT YET SUPPORTED.
    /// If provided, an error will be returned. Use `all: true` instead.
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
/// Supports full cache clear only. Pattern-based invalidation is not supported.
pub async fn invalidate(
    state: &State,
    params: CacheInvalidateParams,
) -> Result<CacheInvalidateResult> {
    use crate::error::Error;

    let cache = state.cache();
    let before = cache.entry_count();

    // Validate parameters: cannot specify both 'all' and 'pattern'
    if params.all && params.pattern.is_some() {
        return Err(Error::InvalidParams(
            "Cannot specify both 'all' and 'pattern'".into(),
        ));
    }

    // Pattern-based invalidation is not supported by moka
    if let Some(ref pattern) = params.pattern {
        return Err(Error::NotImplemented(format!(
            "Pattern-based invalidation ('{}') is not yet supported. Use all=true instead.",
            pattern
        )));
    }

    if params.all {
        cache.invalidate_all().await;
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

    #[test]
    fn test_cache_invalidate_params_both_all_and_pattern() {
        // It's valid to deserialize params with both, but invalidate() will reject them
        let params: CacheInvalidateParams =
            serde_json::from_str(r#"{"all": true, "pattern": "entity:*"}"#).unwrap();
        assert!(params.all);
        assert_eq!(params.pattern, Some("entity:*".to_string()));
    }

    #[test]
    fn test_cache_invalidate_params_neither() {
        // Valid params with neither all nor pattern (no-op)
        let params: CacheInvalidateParams = serde_json::from_str(r#"{}"#).unwrap();
        assert!(!params.all);
        assert!(params.pattern.is_none());
    }
}
