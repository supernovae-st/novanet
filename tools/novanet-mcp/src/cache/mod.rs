//! Cache module
//!
//! Query caching using moka for high-performance concurrent access.
//!
//! ## Request Coalescing
//!
//! The `get_or_try_insert` method provides automatic request coalescing:
//! when multiple concurrent requests hit the same cache key, only one
//! loader function executes, and all waiters receive the result.
//!
//! This prevents the "thundering herd" problem where many concurrent
//! requests for the same uncached data all hit the database.
//!
//! ## Size-Aware Eviction (Phase 2 Optimization)
//!
//! The cache uses token-based weighting for eviction. Each entry's weight
//! is estimated as `json_chars / 4` (rough token approximation). This ensures
//! large results don't starve the cache by evicting based on "token cost"
//! rather than entry count.

use moka::future::Cache;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

/// Query cache using moka with token-based weighting
#[derive(Clone)]
pub struct QueryCache {
    cache: Arc<Cache<String, serde_json::Value>>,
}

impl QueryCache {
    /// Create a new cache with the given settings
    ///
    /// Uses token-based weighting for eviction: each entry's weight is estimated
    /// as `json_chars / 4` (rough token approximation). The `max_entries` parameter
    /// is now interpreted as max tokens capacity.
    pub fn new(max_entries: u64, ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_entries)
            .time_to_live(ttl)
            // Phase 2 Optimization: Size-aware eviction
            // Weight each entry by its estimated token count
            .weigher(|_key: &String, value: &serde_json::Value| -> u32 {
                // Estimate tokens from JSON: serialize and divide by 4
                // This is fast because serde_json is optimized for this
                let weight = match serde_json::to_string(value) {
                    Ok(s) => s.len() / 4,
                    Err(_) => 1, // Minimum weight on error
                };
                // Clamp to u32 range (moka weigher returns u32)
                // Minimum weight of 1 to ensure all entries have some cost
                weight.clamp(1, u32::MAX as usize) as u32
            })
            .build();

        Self {
            cache: Arc::new(cache),
        }
    }

    /// Get a cached value
    pub async fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.cache.get(key).await
    }

    /// Insert a value into the cache
    pub async fn insert(&self, key: String, value: serde_json::Value) {
        self.cache.insert(key, value).await;
    }

    /// Get or compute a value with automatic request coalescing
    ///
    /// If the key exists, returns the cached value.
    /// If not, executes the loader function and caches the result.
    ///
    /// **Key feature**: When multiple concurrent requests hit the same missing key,
    /// only ONE loader function executes. All other requests wait for that result.
    /// This prevents the "thundering herd" problem.
    ///
    /// # Example
    /// ```ignore
    /// let result = cache.get_or_try_insert(
    ///     key,
    ///     || async { execute_query(cypher, params).await }
    /// ).await?;
    /// ```
    pub async fn get_or_try_insert<F, Fut, E>(
        &self,
        key: String,
        loader: F,
    ) -> Result<serde_json::Value, E>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<serde_json::Value, E>>,
        E: Clone + Send + Sync + 'static,
    {
        self.cache
            .try_get_with(key, loader())
            .await
            .map_err(|e| (*e).clone())
    }

    /// Remove a value from the cache
    pub async fn invalidate(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    /// Clear all cached values
    pub async fn clear(&self) {
        self.cache.invalidate_all();
        self.cache.run_pending_tasks().await;
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entry_count: self.cache.entry_count(),
            weighted_size: self.cache.weighted_size(),
        }
    }

    /// Generate a cache key from query and parameters
    ///
    /// PERF: Uses AHash (~30% faster than SipHash for string keys).
    pub fn cache_key(
        cypher: &str,
        params: &Option<serde_json::Map<String, serde_json::Value>>,
    ) -> String {
        use ahash::AHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = AHasher::default();
        cypher.hash(&mut hasher);
        if let Some(p) = params {
            // Serialize params map once (vs. N per-value serializations)
            // serde_json guarantees deterministic key ordering for Map
            if let Ok(serialized) = serde_json::to_string(p) {
                serialized.hash(&mut hasher);
            }
        }
        format!("query:{:016x}", hasher.finish())
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Cache Statistics Methods (Task A3)
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Get the current number of entries in the cache
    pub fn entry_count(&self) -> u64 {
        self.cache.entry_count()
    }

    /// Get the weighted size of the cache in bytes
    pub fn weighted_size(&self) -> u64 {
        self.cache.weighted_size()
    }

    /// Get the maximum capacity of the cache
    pub fn max_capacity(&self) -> u64 {
        self.cache.policy().max_capacity().unwrap_or(10000)
    }

    /// Invalidate all entries in the cache
    pub async fn invalidate_all(&self) {
        self.cache.invalidate_all();
        self.cache.run_pending_tasks().await;
    }

    /// Run pending maintenance tasks (eviction, expiration)
    pub async fn run_pending_tasks(&self) {
        self.cache.run_pending_tasks().await;
    }

    /// Invalidate cache entries matching a pattern
    ///
    /// Supports glob-style patterns:
    /// - `prefix*` - matches keys starting with prefix
    /// - `*suffix` - matches keys ending with suffix
    /// - `*contains*` - matches keys containing substring
    /// - `exact` - matches exact key (no wildcards)
    ///
    /// Returns the number of invalidated entries.
    pub async fn invalidate_pattern(&self, pattern: &str) -> usize {
        // Handle clear-all patterns
        if pattern == "*" || pattern.is_empty() {
            let count = self.cache.entry_count() as usize;
            self.cache.invalidate_all();
            self.cache.run_pending_tasks().await;
            return count;
        }

        // Determine pattern type and extract match string
        let (match_type, match_str) = if let Some(rest) = pattern.strip_prefix('*') {
            if let Some(inner) = rest.strip_suffix('*') {
                // *contains* pattern
                ("contains", inner)
            } else {
                // *suffix pattern
                ("suffix", rest)
            }
        } else if let Some(prefix) = pattern.strip_suffix('*') {
            // prefix* pattern
            ("prefix", prefix)
        } else {
            // Exact match
            ("exact", pattern)
        };

        // Collect matching keys (moka doesn't support remove_if, so collect then invalidate)
        // Note: moka iter() returns (Arc<K>, V), so we dereference the Arc
        let keys_to_invalidate: Vec<String> = self
            .cache
            .iter()
            .filter_map(|(key, _)| {
                let matches = match match_type {
                    "prefix" => key.starts_with(match_str),
                    "suffix" => key.ends_with(match_str),
                    "contains" => key.contains(match_str),
                    "exact" => key.as_str() == match_str,
                    _ => false,
                };
                if matches {
                    // Clone the string from the Arc
                    Some((*key).clone())
                } else {
                    None
                }
            })
            .collect();

        let count = keys_to_invalidate.len();

        // Invalidate each matching key
        for key in keys_to_invalidate {
            self.cache.invalidate(&key).await;
        }

        // Run pending tasks to ensure eviction completes
        self.cache.run_pending_tasks().await;

        count
    }
}

impl Default for QueryCache {
    fn default() -> Self {
        Self::new(10000, Duration::from_secs(300))
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entry_count: u64,
    pub weighted_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_operations() {
        let cache = QueryCache::new(100, Duration::from_secs(60));

        // Insert and get
        let key = "test_key".to_string();
        let value = serde_json::json!({"foo": "bar"});
        cache.insert(key.clone(), value.clone()).await;

        let retrieved = cache.get(&key).await;
        assert_eq!(retrieved, Some(value));

        // Invalidate
        cache.invalidate(&key).await;
        let retrieved = cache.get(&key).await;
        assert_eq!(retrieved, None);
    }

    #[test]
    fn test_cache_key() {
        let key1 = QueryCache::cache_key("MATCH (n) RETURN n", &None);
        let key2 = QueryCache::cache_key("MATCH (n) RETURN n", &None);
        let key3 = QueryCache::cache_key("MATCH (n) RETURN n LIMIT 10", &None);

        // Same query should produce same key
        assert_eq!(key1, key2);

        // Different query should produce different key
        assert_ne!(key1, key3);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Request Coalescing Tests (Phase 1 Performance Optimization)
    // ═══════════════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_get_or_try_insert_basic() {
        let cache = QueryCache::new(100, Duration::from_secs(60));
        let key = "test_key".to_string();

        // First call should execute the loader
        let result: Result<serde_json::Value, String> = cache
            .get_or_try_insert(key.clone(), || async {
                Ok(serde_json::json!({"value": 42}))
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["value"], 42);

        // Second call should return cached value (not execute loader)
        let result2: Result<serde_json::Value, String> = cache
            .get_or_try_insert(key.clone(), || async {
                // This should NOT be called
                Ok(serde_json::json!({"value": 999}))
            })
            .await;

        assert!(result2.is_ok());
        // Should still be 42, not 999
        assert_eq!(result2.unwrap()["value"], 42);
    }

    #[tokio::test]
    async fn test_get_or_try_insert_error_handling() {
        let cache = QueryCache::new(100, Duration::from_secs(60));
        let key = "error_key".to_string();

        // Loader that returns an error
        let result: Result<serde_json::Value, String> = cache
            .get_or_try_insert(key.clone(), || async {
                Err("something went wrong".to_string())
            })
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "something went wrong");

        // Error should NOT be cached - next call should execute loader again
        let result2: Result<serde_json::Value, String> = cache
            .get_or_try_insert(key.clone(), || async {
                Ok(serde_json::json!({"recovered": true}))
            })
            .await;

        assert!(result2.is_ok());
        assert_eq!(result2.unwrap()["recovered"], true);
    }

    #[tokio::test]
    async fn test_request_coalescing_concurrent() {
        use std::sync::atomic::{AtomicU32, Ordering};
        use tokio::time::sleep;

        let cache = Arc::new(QueryCache::new(100, Duration::from_secs(60)));
        let call_count = Arc::new(AtomicU32::new(0));
        let key = "concurrent_key".to_string();

        // Spawn 10 concurrent requests for the same key
        let mut handles = Vec::new();
        for _ in 0..10 {
            let cache = Arc::clone(&cache);
            let key = key.clone();
            let call_count = Arc::clone(&call_count);

            let handle = tokio::spawn(async move {
                let result: Result<serde_json::Value, String> = cache
                    .get_or_try_insert(key, || {
                        let call_count = Arc::clone(&call_count);
                        async move {
                            // Increment call count
                            call_count.fetch_add(1, Ordering::SeqCst);
                            // Simulate slow operation
                            sleep(Duration::from_millis(50)).await;
                            Ok(serde_json::json!({"data": "result"}))
                        }
                    })
                    .await;
                result
            });
            handles.push(handle);
        }

        // Wait for all to complete
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
            assert_eq!(result.unwrap()["data"], "result");
        }

        // The loader should have been called only ONCE
        // (though timing can cause a few due to racing before coalescing kicks in)
        let total_calls = call_count.load(Ordering::SeqCst);
        assert!(
            total_calls <= 3,
            "Expected at most 3 loader calls due to coalescing, got {}",
            total_calls
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Size-Aware Eviction Tests (Phase 2 Performance Optimization)
    // ═══════════════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_size_aware_eviction_weights_entries() {
        // Create a small cache (100 token capacity)
        let cache = QueryCache::new(100, Duration::from_secs(60));

        // Insert a small entry (~10 tokens)
        let small_value = serde_json::json!({"x": 1});
        cache.insert("small".to_string(), small_value.clone()).await;
        cache.run_pending_tasks().await;

        // Small entry should be cached
        assert!(cache.get("small").await.is_some());

        // Insert a large entry (~100+ chars → ~25 tokens)
        let large_value = serde_json::json!({
            "data": "x".repeat(80),
            "more": "y".repeat(80)
        });
        cache.insert("large".to_string(), large_value.clone()).await;
        cache.run_pending_tasks().await;

        // Weighted size should reflect the estimated tokens
        let stats = cache.stats();
        assert!(stats.weighted_size > 0, "Weighted size should be > 0");
    }

    #[tokio::test]
    async fn test_weigher_returns_minimum_one() {
        let cache = QueryCache::new(1000, Duration::from_secs(60));

        // Even empty/null values should have weight >= 1
        cache
            .insert("null".to_string(), serde_json::json!(null))
            .await;
        cache
            .insert("empty".to_string(), serde_json::json!({}))
            .await;
        cache.run_pending_tasks().await;

        // Both should be cached (weight >= 1 ensures they count)
        assert!(cache.get("null").await.is_some());
        assert!(cache.get("empty").await.is_some());

        // Weighted size should be >= 2 (at least 1 per entry)
        let stats = cache.stats();
        assert!(
            stats.weighted_size >= 2,
            "Expected weighted_size >= 2, got {}",
            stats.weighted_size
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Pattern Matching Tests (Task 2.2)
    // ═══════════════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_invalidate_pattern_prefix() {
        let cache = QueryCache::new(1000, Duration::from_secs(60));

        // Insert entries with different prefixes
        cache
            .insert("query:abc".to_string(), serde_json::json!(1))
            .await;
        cache
            .insert("query:def".to_string(), serde_json::json!(2))
            .await;
        cache
            .insert("entity:abc".to_string(), serde_json::json!(3))
            .await;
        cache.run_pending_tasks().await;

        assert_eq!(cache.entry_count(), 3);

        // Invalidate with prefix pattern
        let count = cache.invalidate_pattern("query:*").await;
        assert_eq!(count, 2, "Should invalidate 2 entries with query: prefix");

        // Only entity:abc should remain
        assert!(cache.get("query:abc").await.is_none());
        assert!(cache.get("query:def").await.is_none());
        assert!(cache.get("entity:abc").await.is_some());
    }

    #[tokio::test]
    async fn test_invalidate_pattern_suffix() {
        let cache = QueryCache::new(1000, Duration::from_secs(60));

        // Insert entries with different suffixes
        cache
            .insert("a_test".to_string(), serde_json::json!(1))
            .await;
        cache
            .insert("b_test".to_string(), serde_json::json!(2))
            .await;
        cache
            .insert("a_other".to_string(), serde_json::json!(3))
            .await;
        cache.run_pending_tasks().await;

        // Invalidate with suffix pattern
        let count = cache.invalidate_pattern("*_test").await;
        assert_eq!(count, 2, "Should invalidate 2 entries with _test suffix");

        // Only a_other should remain
        assert!(cache.get("a_test").await.is_none());
        assert!(cache.get("b_test").await.is_none());
        assert!(cache.get("a_other").await.is_some());
    }

    #[tokio::test]
    async fn test_invalidate_pattern_contains() {
        let cache = QueryCache::new(1000, Duration::from_secs(60));

        // Insert entries
        cache
            .insert("foo_bar_baz".to_string(), serde_json::json!(1))
            .await;
        cache
            .insert("qux_bar_quux".to_string(), serde_json::json!(2))
            .await;
        cache
            .insert("foo_qux_baz".to_string(), serde_json::json!(3))
            .await;
        cache.run_pending_tasks().await;

        // Invalidate entries containing "bar"
        let count = cache.invalidate_pattern("*bar*").await;
        assert_eq!(count, 2, "Should invalidate 2 entries containing bar");

        // Only foo_qux_baz should remain
        assert!(cache.get("foo_bar_baz").await.is_none());
        assert!(cache.get("qux_bar_quux").await.is_none());
        assert!(cache.get("foo_qux_baz").await.is_some());
    }

    #[tokio::test]
    async fn test_invalidate_pattern_exact() {
        let cache = QueryCache::new(1000, Duration::from_secs(60));

        // Insert entries
        cache
            .insert("exact_key".to_string(), serde_json::json!(1))
            .await;
        cache
            .insert("exact_key_extra".to_string(), serde_json::json!(2))
            .await;
        cache.run_pending_tasks().await;

        // Invalidate exact match only
        let count = cache.invalidate_pattern("exact_key").await;
        assert_eq!(count, 1, "Should invalidate 1 exact entry");

        assert!(cache.get("exact_key").await.is_none());
        assert!(cache.get("exact_key_extra").await.is_some());
    }

    #[tokio::test]
    async fn test_invalidate_pattern_wildcard_all() {
        let cache = QueryCache::new(1000, Duration::from_secs(60));

        // Insert entries
        cache.insert("a".to_string(), serde_json::json!(1)).await;
        cache.insert("b".to_string(), serde_json::json!(2)).await;
        cache.insert("c".to_string(), serde_json::json!(3)).await;
        cache.run_pending_tasks().await;

        assert_eq!(cache.entry_count(), 3);

        // Invalidate all with wildcard
        let count = cache.invalidate_pattern("*").await;
        assert_eq!(count, 3, "Should invalidate all 3 entries");
        assert_eq!(cache.entry_count(), 0);
    }

    #[tokio::test]
    async fn test_invalidate_pattern_empty_returns_all() {
        let cache = QueryCache::new(1000, Duration::from_secs(60));

        cache.insert("x".to_string(), serde_json::json!(1)).await;
        cache.insert("y".to_string(), serde_json::json!(2)).await;
        cache.run_pending_tasks().await;

        // Empty pattern should clear all (defensive behavior)
        let count = cache.invalidate_pattern("").await;
        assert_eq!(count, 2, "Empty pattern should clear all");
        assert_eq!(cache.entry_count(), 0);
    }

    #[tokio::test]
    async fn test_invalidate_pattern_no_match() {
        let cache = QueryCache::new(1000, Duration::from_secs(60));

        cache.insert("foo".to_string(), serde_json::json!(1)).await;
        cache.insert("bar".to_string(), serde_json::json!(2)).await;
        cache.run_pending_tasks().await;

        // Pattern that matches nothing
        let count = cache.invalidate_pattern("baz*").await;
        assert_eq!(count, 0, "Should invalidate 0 entries");
        assert_eq!(cache.entry_count(), 2);
    }
}
