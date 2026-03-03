//! Cache module
//!
//! Query caching using moka for high-performance concurrent access.

use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

/// Query cache using moka
#[derive(Clone)]
pub struct QueryCache {
    cache: Arc<Cache<String, serde_json::Value>>,
}

impl QueryCache {
    /// Create a new cache with the given settings
    pub fn new(max_entries: u64, ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_entries)
            .time_to_live(ttl)
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

    /// Invalidate cache entries matching a pattern (simple: clears all for now)
    ///
    /// TODO: Implement proper prefix-based pattern matching when needed.
    /// For now, this is a simple implementation that clears all entries.
    pub async fn invalidate_pattern(&self, _pattern: &str) {
        // Simple implementation: invalidate all
        // A more sophisticated implementation would iterate and match prefixes
        self.cache.invalidate_all();
        self.cache.run_pending_tasks().await;
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
}
