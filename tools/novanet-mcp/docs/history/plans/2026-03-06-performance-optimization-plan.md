# NovaNet MCP Server Performance Optimization Plan

**Version**: 0.17.0 → 0.18.0
**Created**: 2026-03-06
**Target Improvements**: 60% latency reduction, 200% throughput increase, 50% memory reduction

---

## Executive Summary

This plan outlines 3 phases of performance optimizations for the NovaNet MCP server, based on research from Perplexity, Context7, and Claude Code documentation.

| Phase | Focus | Timeline | Impact |
|-------|-------|----------|--------|
| **Phase 1** | Quick Wins | 1-2 days | 30% latency reduction |
| **Phase 2** | Medium Optimizations | 3-5 days | 50% throughput increase |
| **Phase 3** | Advanced Optimizations | 1-2 weeks | Full target achievement |

---

## Phase 1: Quick Wins (1-2 days)

### 1.1 Token Counting with spawn_blocking

**Problem**: `tiktoken-rs` BPE encoding blocks the async runtime.

**Current Code** (`src/tokens/counter.rs:85-95`):
```rust
pub fn count(&self, text: &str) -> usize {
    self.bpe.encode_with_special_tokens(text).len()
}
```

**Solution**: Add `count_async()` using `spawn_blocking`.

**File**: `src/tokens/counter.rs`

**Changes**:
```rust
use tokio::task::spawn_blocking;

impl TokenCounter {
    /// Async token counting - moves BPE encoding to blocking thread pool
    pub async fn count_async(&self, text: String) -> usize {
        let bpe = self.bpe.clone();
        spawn_blocking(move || bpe.encode_with_special_tokens(&text).len())
            .await
            .unwrap_or_else(|_| text.len() / 4)
    }

    /// Batch async counting for multiple texts
    pub async fn count_batch_async(&self, texts: Vec<String>) -> Vec<usize> {
        let bpe = self.bpe.clone();
        spawn_blocking(move || {
            texts.iter()
                .map(|text| bpe.encode_with_special_tokens(text).len())
                .collect()
        })
        .await
        .unwrap_or_else(|_| texts.iter().map(|t| t.len() / 4).collect())
    }
}
```

**Verification**:
```bash
cargo test test_token_counting_performance -- --nocapture
# Should pass with <20ms threshold
```

---

### 1.2 Request Coalescing with moka try_get_with

**Problem**: Concurrent identical queries cause duplicate Neo4j work.

**Current Code** (`src/cache/mod.rs:45-55`):
```rust
pub async fn get(&self, key: &str) -> Option<serde_json::Value> {
    self.cache.get(key).await
}

pub async fn insert(&self, key: String, value: serde_json::Value) {
    self.cache.insert(key, value).await;
}
```

**Solution**: Use `try_get_with` for automatic request coalescing.

**File**: `src/cache/mod.rs`

**Changes**:
```rust
use std::sync::Arc;

impl QueryCache {
    /// Get or compute with automatic request coalescing
    /// Multiple concurrent requests for the same key only execute the loader once
    pub async fn get_or_try_insert<F, Fut>(
        &self,
        key: String,
        loader: F,
    ) -> Result<serde_json::Value, crate::error::Error>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<serde_json::Value, crate::error::Error>>,
    {
        self.cache
            .try_get_with(key, async {
                loader().await
            })
            .await
            .map_err(|e| crate::error::Error::Internal(e.to_string()))
    }
}
```

**Usage in tools** (`src/tools/query.rs`):
```rust
// Before:
if let Some(cached) = cache.get(&key).await {
    return Ok(cached);
}
let result = execute_query(...).await?;
cache.insert(key, result.clone()).await;

// After:
let result = cache.get_or_try_insert(key, || async {
    execute_query(...).await
}).await?;
```

**Verification**:
```bash
# Run concurrent requests test
cargo test test_request_coalescing -- --nocapture
```

---

### 1.3 Cache Warming on Startup

**Problem**: Cold cache causes slow first requests.

**Solution**: Pre-warm schema cache on server startup.

**File**: `src/server/state.rs`

**Changes**:
```rust
impl State {
    /// Warm cache with frequently-used schema queries
    pub async fn warm_cache(&self) -> Result<(), crate::error::Error> {
        use tracing::info;

        let queries = vec![
            // Schema introspection (most common)
            "MATCH (c:Class) RETURN c.name, c.realm, c.layer, c.trait",
            // Arc classes
            "MATCH (a:ArcClass) RETURN a.name, a.family, a.scope",
            // Locale list
            "MATCH (l:Locale) RETURN l.key, l.language, l.region LIMIT 50",
        ];

        for cypher in queries {
            let key = format!("warm:{}", cypher);
            let pool = self.pool();
            let result = pool.execute_query(cypher, None).await?;
            self.cache().insert(key, serde_json::to_value(&result)?).await;
        }

        info!(queries = queries.len(), "Cache warmed");
        Ok(())
    }
}
```

**File**: `src/main.rs` (add after State creation):
```rust
// Warm cache on startup
if let Err(e) = state.warm_cache().await {
    warn!("Cache warming failed: {}", e);
}
```

**Verification**:
```bash
# Check logs for cache warming
RUST_LOG=info cargo run 2>&1 | grep "Cache warmed"
```

---

## Phase 2: Medium Optimizations (3-5 days)

### 2.1 Parallel Evidence Collection in novanet_generate

**Problem**: Evidence collection is sequential.

**Current Code** (`src/tools/generate.rs`):
```rust
let entities = self.collect_entities(&focus_key).await?;
let atoms = self.collect_atoms(&locale).await?;
let structure = self.collect_structure(&focus_key).await?;
```

**Solution**: Use `tokio::join!` for parallel collection.

**File**: `src/tools/generate.rs`

**Changes**:
```rust
use tokio::join;

// Parallel evidence collection
let (entities_result, atoms_result, structure_result) = join!(
    self.collect_entities(&focus_key),
    self.collect_atoms(&locale),
    self.collect_structure(&focus_key)
);

let entities = entities_result?;
let atoms = atoms_result?;
let structure = structure_result?;
```

**Expected Impact**: 3x faster context assembly for `novanet_generate`.

---

### 2.2 Size-Aware Cache Eviction

**Problem**: Cache evicts by entry count, not token size.

**Current Code** (`src/cache/mod.rs`):
```rust
let cache = Cache::builder()
    .max_capacity(config.cache_max_entries as u64)
    .time_to_live(Duration::from_secs(config.cache_ttl_secs))
    .build();
```

**Solution**: Use weigher function for token-based capacity.

**File**: `src/cache/mod.rs`

**Changes**:
```rust
use moka::future::Cache;

pub struct QueryCache {
    cache: Cache<String, CacheEntry>,
}

#[derive(Clone)]
pub struct CacheEntry {
    pub value: serde_json::Value,
    pub token_count: usize,
}

impl QueryCache {
    pub fn new(max_tokens: u64, ttl_secs: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_tokens)
            .weigher(|_key: &String, entry: &CacheEntry| -> u32 {
                entry.token_count.min(u32::MAX as usize) as u32
            })
            .time_to_live(Duration::from_secs(ttl_secs))
            .build();

        Self { cache }
    }

    pub async fn insert(&self, key: String, value: serde_json::Value, token_count: usize) {
        self.cache.insert(key, CacheEntry { value, token_count }).await;
    }
}
```

**Config Update** (`src/server/config.rs`):
```rust
// Change from entry count to token capacity
pub cache_max_tokens: u64,  // Default: 1_000_000 tokens
```

---

### 2.3 Query Projection Optimization

**Problem**: Queries return full nodes when only some properties are needed.

**Solution**: Project only needed properties in Cypher.

**File**: `src/tools/search.rs`

**Before**:
```cypher
MATCH (n:Entity)
WHERE n.name CONTAINS $query
RETURN n
```

**After**:
```cypher
MATCH (n:Entity)
WHERE n.name CONTAINS $query
RETURN n.key AS key, n.name AS name, n.description AS description
```

**Impact**: Reduces data transfer by 40-60% for search queries.

---

## Phase 3: Advanced Optimizations (1-2 weeks)

### 3.1 Streaming Results with Early Termination

**Problem**: Large queries buffer all results before returning.

**Current Code** (`src/neo4j/pool.rs:120-135`):
```rust
let mut rows = Vec::new();
while let Some(row) = result.next().await? {
    rows.push(json_row);
}
Ok(rows)
```

**Solution**: Implement streaming with token budget termination.

**File**: `src/neo4j/pool.rs`

**Changes**:
```rust
use tokio::sync::mpsc;

pub struct StreamingResult {
    pub rows: Vec<serde_json::Value>,
    pub terminated_early: bool,
    pub total_tokens: usize,
}

impl Neo4jPool {
    /// Execute query with streaming and token budget
    pub async fn execute_streaming(
        &self,
        query: &str,
        params: Option<serde_json::Map<String, serde_json::Value>>,
        token_budget: usize,
        counter: &TokenCounter,
    ) -> Result<StreamingResult> {
        let conn = self.pool.get().await?;
        let mut result = conn.execute(query.into(), params_to_bolt(params)).await?;

        let mut rows = Vec::new();
        let mut total_tokens = 0;
        let mut terminated_early = false;

        while let Some(row) = result.next().await? {
            let json_row = row_to_json(row)?;
            let row_tokens = counter.estimate(&serde_json::to_string(&json_row)?);

            if total_tokens + row_tokens > token_budget {
                terminated_early = true;
                break;
            }

            total_tokens += row_tokens;
            rows.push(json_row);
        }

        Ok(StreamingResult {
            rows,
            terminated_early,
            total_tokens,
        })
    }
}
```

---

### 3.2 Connection Pool Tuning

**Problem**: Default pool settings may not be optimal.

**Solution**: Tune based on workload characteristics.

**File**: `src/neo4j/pool.rs`

**Changes**:
```rust
pub fn new(config: &Config) -> Result<Self> {
    let pool = Pool::builder()
        .max_size(config.neo4j_pool_size)
        .min_idle(Some(2))  // Keep 2 connections warm
        .connection_timeout(Duration::from_secs(5))
        .idle_timeout(Some(Duration::from_secs(60)))
        .max_lifetime(Some(Duration::from_secs(1800)))  // 30 min
        .build(manager)?;

    Ok(Self { pool })
}
```

---

### 3.3 Circuit Breaker Pattern

**Problem**: Cascading failures when Neo4j is overloaded.

**Solution**: Implement circuit breaker with exponential backoff.

**File**: `src/neo4j/circuit_breaker.rs` (new file)

```rust
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, Instant};

pub struct CircuitBreaker {
    failure_count: AtomicU32,
    last_failure: AtomicU64,
    threshold: u32,
    reset_timeout: Duration,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, reset_timeout: Duration) -> Self {
        Self {
            failure_count: AtomicU32::new(0),
            last_failure: AtomicU64::new(0),
            threshold,
            reset_timeout,
        }
    }

    pub fn is_open(&self) -> bool {
        let failures = self.failure_count.load(Ordering::Relaxed);
        if failures < self.threshold {
            return false;
        }

        let last = self.last_failure.load(Ordering::Relaxed);
        let elapsed = Instant::now().duration_since(
            Instant::now() - Duration::from_millis(last)
        );
        elapsed < self.reset_timeout
    }

    pub fn record_success(&self) {
        self.failure_count.store(0, Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.failure_count.fetch_add(1, Ordering::Relaxed);
        self.last_failure.store(
            Instant::now().elapsed().as_millis() as u64,
            Ordering::Relaxed
        );
    }
}
```

---

## Verification Checklist

### Phase 1
- [ ] `test_token_counting_performance` passes (<20ms)
- [ ] `test_request_coalescing` shows single Neo4j call for concurrent requests
- [ ] Startup logs show "Cache warmed" message
- [ ] No regressions in existing tests

### Phase 2
- [ ] `novanet_generate` benchmark shows 3x improvement
- [ ] Cache memory usage is bounded by token count
- [ ] Search queries return projected properties only

### Phase 3
- [ ] Streaming terminates early on token budget
- [ ] Connection pool handles burst traffic
- [ ] Circuit breaker trips on consecutive failures

---

## Metrics to Track

| Metric | Current | Phase 1 | Phase 2 | Phase 3 |
|--------|---------|---------|---------|---------|
| p50 latency | ~100ms | ~70ms | ~50ms | ~40ms |
| p99 latency | ~500ms | ~350ms | ~200ms | ~150ms |
| Throughput | 100 req/s | 150 req/s | 250 req/s | 300 req/s |
| Memory/1K entries | 50MB | 50MB | 35MB | 25MB |

---

## Rollback Plan

Each phase can be rolled back independently:

1. **Phase 1**: Revert to sync token counting, remove coalescing
2. **Phase 2**: Revert to entry-count cache, remove parallelization
3. **Phase 3**: Remove streaming, circuit breaker

Feature flags can be added for gradual rollout:

```rust
// In config.rs
pub use_async_tokens: bool,
pub use_request_coalescing: bool,
pub use_streaming_results: bool,
```
