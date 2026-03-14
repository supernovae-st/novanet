//! Neo4j module
//!
//! Connection pooling and query execution for Neo4j.
//!
//! ## Phase 3 Performance Optimization
//!
//! Added circuit breaker pattern for resilience under load.

mod circuit_breaker;
pub(crate) mod cypher_guard;
mod pool;

pub use circuit_breaker::CircuitBreaker;
pub use pool::Neo4jPool;
