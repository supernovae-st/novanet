//! Server state
//!
//! Shared application state using parking_lot for faster locking.

use crate::cache::QueryCache;
use crate::error::Result;
use crate::neo4j::Neo4jPool;
use crate::server::Config;
use crate::tokens::TokenCounter;
use parking_lot::RwLock;
use std::sync::Arc;

/// Shared application state
#[derive(Clone)]
pub struct State {
    inner: Arc<StateInner>,
}

struct StateInner {
    pub config: Config,
    pub pool: Neo4jPool,
    pub cache: QueryCache,
    pub counter: TokenCounter,
    /// Server statistics
    pub stats: RwLock<ServerStats>,
}

/// Server statistics
#[derive(Debug, Default)]
pub struct ServerStats {
    pub queries_executed: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub total_tokens_counted: u64,
}

impl State {
    /// Create new state with the given configuration
    pub async fn new(config: Config) -> Result<Self> {
        // Create Neo4j pool
        let pool = Neo4jPool::new(
            &config.neo4j_uri,
            &config.neo4j_user,
            &config.neo4j_password,
            config.pool_size,
        )
        .await?;

        // Create cache
        let cache = QueryCache::new(config.cache_max_entries, config.cache_ttl);

        // Create token counter
        let counter = TokenCounter::new();

        Ok(Self {
            inner: Arc::new(StateInner {
                config,
                pool,
                cache,
                counter,
                stats: RwLock::new(ServerStats::default()),
            }),
        })
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    /// Get the Neo4j pool
    pub fn pool(&self) -> &Neo4jPool {
        &self.inner.pool
    }

    /// Get the query cache
    pub fn cache(&self) -> &QueryCache {
        &self.inner.cache
    }

    /// Get the token counter
    pub fn counter(&self) -> &TokenCounter {
        &self.inner.counter
    }

    /// Increment query count
    pub fn record_query(&self) {
        self.inner.stats.write().queries_executed += 1;
    }

    /// Record cache hit
    pub fn record_cache_hit(&self) {
        self.inner.stats.write().cache_hits += 1;
    }

    /// Record cache miss
    pub fn record_cache_miss(&self) {
        self.inner.stats.write().cache_misses += 1;
    }

    /// Get current statistics
    pub fn stats(&self) -> ServerStats {
        self.inner.stats.read().clone()
    }
}

impl Clone for ServerStats {
    fn clone(&self) -> Self {
        Self {
            queries_executed: self.queries_executed,
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            total_tokens_counted: self.total_tokens_counted,
        }
    }
}
