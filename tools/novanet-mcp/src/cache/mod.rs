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
    pub fn cache_key(
        cypher: &str,
        params: &Option<serde_json::Map<String, serde_json::Value>>,
    ) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        cypher.hash(&mut hasher);
        if let Some(p) = params {
            for (k, v) in p {
                k.hash(&mut hasher);
                v.to_string().hash(&mut hasher);
            }
        }
        format!("query:{:016x}", hasher.finish())
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
