//! Server state
//!
//! Shared application state using parking_lot for faster locking.
//!
//! ## Cache Warming
//!
//! On startup, the server pre-warms the cache with frequently-used schema queries
//! to avoid cold-cache latency on first requests. See `warm_cache()` method.
//!
//! ## Phase 3 Performance Optimization
//!
//! Added circuit breaker pattern for Neo4j resilience under load.

use crate::cache::QueryCache;
use crate::error::Result;
use crate::neo4j::{CircuitBreaker, Neo4jPool};
use crate::schema_cache::SchemaCache;
use crate::server::Config;
use crate::tokens::TokenCounter;
use parking_lot::RwLock;
use std::sync::Arc;
use tracing::{info, warn};

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
    pub schema_cache: SchemaCache,
    /// Circuit breaker for Neo4j resilience (Phase 3)
    pub circuit_breaker: CircuitBreaker,
    /// Server statistics
    pub stats: RwLock<ServerStats>,
}

/// Server statistics
#[derive(Debug, Default, Clone)]
pub struct ServerStats {
    pub queries_executed: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl State {
    /// Create new state with the given configuration
    pub async fn new(config: Config) -> Result<Self> {
        // Create Neo4j pool with custom fetch_size (Phase 3)
        let pool = Neo4jPool::with_fetch_size(
            &config.neo4j_uri,
            &config.neo4j_user,
            &config.neo4j_password,
            config.pool_size,
            config.fetch_size,
        )
        .await?;

        // Create cache
        let cache = QueryCache::new(config.cache_max_entries, config.cache_ttl);

        // Create token counter
        let counter = TokenCounter::new();

        // Create schema cache
        let schema_cache = SchemaCache::new(config.cache_ttl.as_secs());

        // Create circuit breaker (Phase 3)
        let circuit_breaker = CircuitBreaker::new(
            config.circuit_breaker_threshold,
            config.circuit_breaker_reset_timeout,
        );

        Ok(Self {
            inner: Arc::new(StateInner {
                config,
                pool,
                cache,
                counter,
                schema_cache,
                circuit_breaker,
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

    /// Get schema cache reference
    pub fn schema_cache(&self) -> &SchemaCache {
        &self.inner.schema_cache
    }

    /// Get circuit breaker reference (Phase 3)
    pub fn circuit_breaker(&self) -> &CircuitBreaker {
        &self.inner.circuit_breaker
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

    // ═══════════════════════════════════════════════════════════════════════════════
    // Cache Warming (Phase 1 Performance Optimization)
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Warm the cache with frequently-used schema queries
    ///
    /// Pre-loads schema metadata to avoid cold-cache latency on first requests.
    /// Called automatically during server startup.
    ///
    /// **Queries warmed:**
    /// - NodeClasses (realm, layer, trait)
    /// - ArcClasses (family, scope)
    /// - Locales (language, region)
    pub async fn warm_cache(&self) -> Result<()> {
        let queries = [
            // Schema introspection - most common queries
            (
                "warm:classes",
                "MATCH (c:Schema:Class) RETURN c.name AS name, c.realm AS realm, c.layer AS layer, c.trait_type AS trait LIMIT 100",
            ),
            // Arc classes
            (
                "warm:arcs",
                "MATCH (a:Schema:ArcClass) RETURN a.name AS name, a.family AS family, a.scope AS scope LIMIT 200",
            ),
            // Locales
            (
                "warm:locales",
                "MATCH (l:Locale) RETURN l.key AS key, l.language AS language, l.region AS region LIMIT 50",
            ),
        ];

        let mut warmed = 0;
        let mut failed = 0;

        for (key, cypher) in queries {
            match self.pool().execute_query(cypher, None).await {
                Ok(rows) => {
                    let value = serde_json::to_value(&rows).unwrap_or_default();
                    self.cache().insert(key.to_string(), value).await;
                    warmed += 1;
                }
                Err(e) => {
                    warn!(query = key, error = %e, "Cache warming query failed");
                    failed += 1;
                }
            }
        }

        if warmed > 0 {
            info!(warmed = warmed, failed = failed, "Cache warmed");
        }

        Ok(())
    }
}
