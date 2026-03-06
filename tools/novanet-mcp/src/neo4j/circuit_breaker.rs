//! Circuit Breaker Pattern
//!
//! Prevents cascading failures when Neo4j is overloaded or unavailable.
//!
//! ## Phase 3 Performance Optimization
//!
//! The circuit breaker has three states:
//! - **Closed**: Normal operation, requests pass through
//! - **Open**: Failures exceeded threshold, requests fail fast
//! - **Half-Open**: After reset timeout, allow one test request
//!
//! ## Usage
//!
//! ```ignore
//! let breaker = CircuitBreaker::new(5, Duration::from_secs(30));
//!
//! if breaker.is_open() {
//!     return Err(Error::CircuitOpen);
//! }
//!
//! match execute_query().await {
//!     Ok(result) => {
//!         breaker.record_success();
//!         Ok(result)
//!     }
//!     Err(e) => {
//!         breaker.record_failure();
//!         Err(e)
//!     }
//! }
//! ```

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Circuit breaker for Neo4j connection resilience
pub struct CircuitBreaker {
    /// Consecutive failure count
    failure_count: AtomicU32,
    /// Timestamp of last failure (milliseconds since epoch)
    last_failure_ms: AtomicU64,
    /// Threshold before circuit opens
    threshold: u32,
    /// Duration before attempting reset
    reset_timeout: Duration,
    /// Start time for calculating elapsed time
    start_time: Instant,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    ///
    /// # Arguments
    /// * `threshold` - Number of consecutive failures before opening circuit
    /// * `reset_timeout` - Duration to wait before attempting half-open state
    pub fn new(threshold: u32, reset_timeout: Duration) -> Self {
        Self {
            failure_count: AtomicU32::new(0),
            last_failure_ms: AtomicU64::new(0),
            threshold,
            reset_timeout,
            start_time: Instant::now(),
        }
    }

    /// Check if the circuit is open (requests should fail fast)
    ///
    /// Returns `true` if the circuit is open and requests should be rejected.
    /// The circuit is open when consecutive failures exceed threshold AND
    /// the reset timeout hasn't elapsed.
    pub fn is_open(&self) -> bool {
        let failures = self.failure_count.load(Ordering::Relaxed);

        // Below threshold, circuit is closed
        if failures < self.threshold {
            return false;
        }

        // Check if reset timeout has elapsed (half-open state)
        let last_failure_ms = self.last_failure_ms.load(Ordering::Relaxed);
        let current_ms = self.start_time.elapsed().as_millis() as u64;
        let elapsed = Duration::from_millis(current_ms.saturating_sub(last_failure_ms));

        // If enough time has passed, allow one request (half-open)
        elapsed < self.reset_timeout
    }

    /// Record a successful request
    ///
    /// Resets the failure counter, closing the circuit.
    pub fn record_success(&self) {
        self.failure_count.store(0, Ordering::Relaxed);
    }

    /// Record a failed request
    ///
    /// Increments the failure counter and updates the last failure timestamp.
    pub fn record_failure(&self) {
        self.failure_count.fetch_add(1, Ordering::Relaxed);
        let current_ms = self.start_time.elapsed().as_millis() as u64;
        self.last_failure_ms.store(current_ms, Ordering::Relaxed);
    }

    /// Get the current failure count
    pub fn failure_count(&self) -> u32 {
        self.failure_count.load(Ordering::Relaxed)
    }

    /// Check if the circuit is in half-open state
    ///
    /// Half-open means threshold was exceeded but reset timeout has elapsed,
    /// allowing one test request.
    pub fn is_half_open(&self) -> bool {
        let failures = self.failure_count.load(Ordering::Relaxed);
        if failures < self.threshold {
            return false;
        }

        let last_failure_ms = self.last_failure_ms.load(Ordering::Relaxed);
        let current_ms = self.start_time.elapsed().as_millis() as u64;
        let elapsed = Duration::from_millis(current_ms.saturating_sub(last_failure_ms));

        elapsed >= self.reset_timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_circuit_breaker_starts_closed() {
        let breaker = CircuitBreaker::new(3, Duration::from_millis(100));
        assert!(!breaker.is_open());
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn test_circuit_breaker_opens_after_threshold() {
        let breaker = CircuitBreaker::new(3, Duration::from_secs(30));

        // Record failures below threshold
        breaker.record_failure();
        assert!(!breaker.is_open());
        breaker.record_failure();
        assert!(!breaker.is_open());

        // Third failure should open the circuit
        breaker.record_failure();
        assert!(breaker.is_open());
        assert_eq!(breaker.failure_count(), 3);
    }

    #[test]
    fn test_circuit_breaker_resets_on_success() {
        let breaker = CircuitBreaker::new(2, Duration::from_secs(30));

        // Open the circuit
        breaker.record_failure();
        breaker.record_failure();
        assert!(breaker.is_open());

        // Success should close it
        breaker.record_success();
        assert!(!breaker.is_open());
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn test_circuit_breaker_half_open_after_timeout() {
        let breaker = CircuitBreaker::new(2, Duration::from_millis(50));

        // Open the circuit
        breaker.record_failure();
        breaker.record_failure();
        assert!(breaker.is_open());

        // Wait for reset timeout
        sleep(Duration::from_millis(60));

        // Should now be half-open (allows requests)
        assert!(!breaker.is_open());
        assert!(breaker.is_half_open());
    }

    #[test]
    fn test_circuit_breaker_remains_open_within_timeout() {
        let breaker = CircuitBreaker::new(2, Duration::from_millis(100));

        // Open the circuit
        breaker.record_failure();
        breaker.record_failure();
        assert!(breaker.is_open());

        // Within timeout, should remain open
        sleep(Duration::from_millis(20));
        assert!(breaker.is_open());
    }

    #[test]
    fn test_circuit_breaker_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let breaker = Arc::new(CircuitBreaker::new(100, Duration::from_secs(30)));
        let mut handles = vec![];

        // Spawn threads to record failures concurrently
        for _ in 0..10 {
            let b = Arc::clone(&breaker);
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    b.record_failure();
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // Should have recorded ~100 failures (atomic increment)
        assert_eq!(breaker.failure_count(), 100);
        assert!(breaker.is_open());
    }
}
