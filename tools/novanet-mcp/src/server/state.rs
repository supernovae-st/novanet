//! Server state
//!
//! Shared application state with lock-free atomic counters for stats.
//!
//! ## Cache Warming
//!
//! On startup, the server pre-warms the cache with frequently-used schema queries
//! to avoid cold-cache latency on first requests. See `warm_cache()` method.
//!
//! ## Phase 3 Performance Optimization
//!
//! Added circuit breaker pattern for Neo4j resilience under load.
//!
//! ## Phase 2.2 Spreading Activation
//!
//! Added SpreadingConfig for context assembly with exponential decay
//! and task-specific modifiers.

use crate::cache::QueryCache;
use crate::activation::SpreadingConfig;
use crate::error::Result;
use crate::neo4j::{CircuitBreaker, Neo4jPool};
use crate::schema_cache::SchemaCache;
use crate::server::Config;
use crate::tokens::TokenCounter;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
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
    /// Spreading activation config (Phase 2.2)
    pub spreading_config: SpreadingConfig,
    /// Server statistics (lock-free atomic counters)
    pub stats: ServerStats,
}

/// Server statistics (lock-free atomic counters)
#[derive(Debug, Default)]
pub struct ServerStats {
    pub queries_executed: AtomicU64,
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
}

/// Snapshot of server statistics at a point in time
#[derive(Debug, Clone)]
pub struct StatsSnapshot {
    pub queries_executed: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl State {
    /// Create new state with the given configuration
    pub async fn new(config: Config) -> Result<Self> {
        // Create Neo4j pool with retry config
        let pool = Neo4jPool::with_retry_config(
            &config.neo4j_uri,
            &config.neo4j_user,
            &config.neo4j_password,
            config.pool_size,
            config.fetch_size,
            config.max_retries,
            config.retry_base_delay,
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

        // Load spreading config (Phase 2.2)
        let spreading_config = Self::load_spreading_config(&config);

        Ok(Self {
            inner: Arc::new(StateInner {
                config,
                pool,
                cache,
                counter,
                schema_cache,
                circuit_breaker,
                spreading_config,
                stats: ServerStats::default(),
            }),
        })
    }

    /// Load spreading activation config from YAML or use defaults
    ///
    /// If `NOVANET_MCP_SPREADING_CONFIG_PATH` is set, attempts to load from YAML.
    /// Falls back to hardcoded defaults if file not found or parse error.
    fn load_spreading_config(config: &Config) -> SpreadingConfig {
        if let Some(ref path) = config.spreading_config_path {
            match SpreadingConfig::load_from_yaml(path) {
                Ok(cfg) => {
                    info!(
                        path = %path.display(),
                        decay_factor = cfg.default.decay_factor,
                        "Loaded spreading config from YAML"
                    );
                    return cfg;
                }
                Err(e) => {
                    warn!(
                        path = %path.display(),
                        error = %e,
                        "Failed to load spreading config, using defaults"
                    );
                }
            }
        }

        let default = SpreadingConfig::default();
        info!(
            decay_factor = default.default.decay_factor,
            activation_threshold = default.default.activation_threshold,
            "Using default spreading config"
        );
        default
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

    /// Get spreading activation config (Phase 2.2)
    pub fn spreading_config(&self) -> &SpreadingConfig {
        &self.inner.spreading_config
    }

    /// Increment query count
    pub fn record_query(&self) {
        self.inner.stats.queries_executed.fetch_add(1, Ordering::Relaxed);
    }

    /// Record cache hit
    pub fn record_cache_hit(&self) {
        self.inner.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Record cache miss
    pub fn record_cache_miss(&self) {
        self.inner.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current statistics snapshot
    pub fn stats(&self) -> StatsSnapshot {
        StatsSnapshot {
            queries_executed: self.inner.stats.queries_executed.load(Ordering::Relaxed),
            cache_hits: self.inner.stats.cache_hits.load(Ordering::Relaxed),
            cache_misses: self.inner.stats.cache_misses.load(Ordering::Relaxed),
        }
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
    /// - NodeClasses (realm, layer)
    /// - ArcClasses (family, scope)
    /// - Locales (language, region)
    pub async fn warm_cache(&self) -> Result<()> {
        let queries = [
            // Schema introspection - most common queries
            (
                "warm:classes",
                "MATCH (c:Schema:Class) RETURN c.name AS name, c.realm AS realm, c.layer AS layer LIMIT 100",
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
