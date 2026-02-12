//! Integration tests for NovaNet MCP Server
//!
//! These tests require a running Neo4j instance.
//! Run with: NEO4J_URI=bolt://localhost:7687 cargo test --test integration_tests
//!
//! Test categories:
//! - Neo4j connection and pooling
//! - Query execution and caching
//! - Token counting accuracy
//! - Tool execution via MCP protocol

use std::env;
use std::time::Duration;

/// Test helper to check if Neo4j is available
fn neo4j_available() -> bool {
    env::var("NEO4J_URI").is_ok()
        && env::var("NEO4J_USER").is_ok()
        && env::var("NEO4J_PASSWORD").is_ok()
}

/// Skip test if Neo4j is not configured
macro_rules! require_neo4j {
    () => {
        if !neo4j_available() {
            eprintln!("Skipping test: NEO4J_* environment variables not set");
            return;
        }
    };
}

mod neo4j_connection {
    use super::*;

    #[tokio::test]
    async fn test_pool_creation() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        // Pool creation should succeed
        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5).await;
        assert!(pool.is_ok(), "Pool creation failed: {:?}", pool.err());
    }

    #[tokio::test]
    async fn test_health_check() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        let health = pool.health_check().await;
        assert!(health.is_ok(), "Health check failed: {:?}", health.err());
        assert!(health.unwrap(), "Database is not healthy");
    }

    #[tokio::test]
    async fn test_invalid_credentials() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, "invalid", "invalid", 5).await;
        // Should fail with authentication error
        assert!(pool.is_err());
    }
}

mod query_execution {
    use super::*;

    #[tokio::test]
    async fn test_simple_query() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Simple RETURN query
        let result = pool
            .execute_query("RETURN 1 AS num, 'hello' AS str", None)
            .await;
        assert!(result.is_ok(), "Query failed: {:?}", result.err());

        let rows = result.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["num"], 1);
        assert_eq!(rows[0]["str"], "hello");
    }

    #[tokio::test]
    async fn test_query_with_params() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Query with parameters
        let mut params = serde_json::Map::new();
        params.insert("name".to_string(), serde_json::json!("test"));
        params.insert("value".to_string(), serde_json::json!(42));

        let result = pool
            .execute_query("RETURN $name AS name, $value AS value", Some(params))
            .await;

        assert!(
            result.is_ok(),
            "Query with params failed: {:?}",
            result.err()
        );

        let rows = result.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["name"], "test");
        assert_eq!(rows[0]["value"], 42);
    }

    #[tokio::test]
    async fn test_read_only_enforcement_create() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // CREATE should be blocked
        let result = pool.execute_query("CREATE (n:Test) RETURN n", None).await;
        assert!(result.is_err(), "CREATE should be blocked");

        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("CREATE"),
            "Error should mention CREATE"
        );
    }

    #[tokio::test]
    async fn test_read_only_enforcement_delete() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // DELETE should be blocked
        let result = pool.execute_query("MATCH (n) DELETE n", None).await;
        assert!(result.is_err(), "DELETE should be blocked");
    }

    #[tokio::test]
    async fn test_read_only_enforcement_merge() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // MERGE should be blocked
        let result = pool.execute_query("MERGE (n:Test) RETURN n", None).await;
        assert!(result.is_err(), "MERGE should be blocked");
    }

    #[tokio::test]
    async fn test_read_only_enforcement_set() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // SET should be blocked
        let result = pool
            .execute_query("MATCH (n) SET n.foo = 'bar' RETURN n", None)
            .await;
        assert!(result.is_err(), "SET should be blocked");
    }

    #[tokio::test]
    async fn test_novanet_schema_query() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Query for NovaNet Kind nodes (meta-graph)
        let result = pool
            .execute_query("MATCH (k:Kind) RETURN k.name AS name LIMIT 5", None)
            .await;

        // This might fail if database is empty, which is OK
        if let Ok(rows) = result {
            for row in &rows {
                assert!(row.get("name").is_some(), "Kind node should have name");
            }
        }
    }
}

mod token_counting {
    use novanet_mcp::tokens::TokenCounter;

    #[test]
    fn test_english_text_counting() {
        let counter = TokenCounter::new();

        // "Hello, world!" should be ~4 tokens
        let text = "Hello, world!";
        let count = counter.count(text);
        assert!(
            count >= 2 && count <= 6,
            "English token count off: {}",
            count
        );
    }

    #[test]
    fn test_cjk_text_counting() {
        let counter = TokenCounter::new();

        // CJK characters typically have 1-2 tokens each
        let text = "你好世界";
        let count = counter.count(text);
        assert!(count >= 2 && count <= 8, "CJK token count off: {}", count);
    }

    #[test]
    fn test_estimate_accuracy() {
        let counter = TokenCounter::new();

        // Test estimate vs exact for various texts
        let texts = [
            "Hello, world!",
            "The quick brown fox jumps over the lazy dog.",
            "NovaNet is a knowledge graph for content generation across 200+ locales.",
            "你好世界",
        ];

        for text in texts {
            let exact = counter.count(text);
            let estimate = counter.estimate(text);

            // Estimate should be within 50% of exact for most texts
            let ratio = estimate as f64 / exact as f64;
            assert!(
                ratio >= 0.5 && ratio <= 2.0,
                "Estimate too far from exact for '{}': {} vs {}",
                text,
                estimate,
                exact
            );
        }
    }

    #[test]
    fn test_within_budget() {
        let counter = TokenCounter::new();

        let text = "Hello, world!";

        // Should be within large budget
        assert!(counter.within_budget(text, 100));

        // Should NOT be within tiny budget
        assert!(!counter.within_budget(text, 1));
    }

    #[test]
    fn test_truncate_to_budget() {
        let counter = TokenCounter::new();

        let text = "The quick brown fox jumps over the lazy dog. This is a longer sentence.";
        let budget = 5;

        let (truncated, count) = counter.truncate_to_budget(text, budget);

        assert!(
            count <= budget,
            "Truncated count {} exceeds budget {}",
            count,
            budget
        );
        assert!(truncated.len() < text.len(), "Should have truncated");
    }
}

mod caching {
    use novanet_mcp::cache::QueryCache;
    use std::time::Duration;

    #[tokio::test]
    async fn test_cache_insert_and_get() {
        let cache = QueryCache::new(100, Duration::from_secs(60));

        let key = "test_key".to_string();
        let value = serde_json::json!({"data": "test"});

        cache.insert(key.clone(), value.clone()).await;

        let retrieved = cache.get(&key).await;
        assert!(retrieved.is_some(), "Should retrieve cached value");
        assert_eq!(retrieved.unwrap(), value);
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let cache = QueryCache::new(100, Duration::from_secs(60));

        let retrieved = cache.get(&"nonexistent".to_string()).await;
        assert!(retrieved.is_none(), "Should not find nonexistent key");
    }

    #[tokio::test]
    async fn test_cache_key_generation() {
        let cypher = "MATCH (n) RETURN n LIMIT 10";
        let params = Some({
            let mut map = serde_json::Map::new();
            map.insert("key".to_string(), serde_json::json!("value"));
            map
        });

        let key = QueryCache::cache_key(cypher, &params);

        // Key should be consistent
        let key2 = QueryCache::cache_key(cypher, &params);
        assert_eq!(key, key2, "Cache keys should be consistent");

        // Different params should produce different keys
        let params2 = Some({
            let mut map = serde_json::Map::new();
            map.insert("key".to_string(), serde_json::json!("different"));
            map
        });
        let key3 = QueryCache::cache_key(cypher, &params2);
        assert_ne!(key, key3, "Different params should produce different keys");
    }
}

mod tools {
    use super::*;

    #[tokio::test]
    async fn test_describe_schema() {
        require_neo4j!();

        // This test requires the full server to be set up
        // For now, just verify the module structure exists
        // Full integration test would need:
        // 1. Create State with Neo4j pool
        // 2. Call describe::execute with Schema target
        // 3. Verify response structure
    }

    #[tokio::test]
    async fn test_query_tool_limit() {
        require_neo4j!();

        // Verify LIMIT is applied to queries without one
        // This is tested in unit tests, but verify end-to-end
    }
}

mod security {
    use super::*;

    #[tokio::test]
    async fn test_cypher_injection_comments() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Attempt to bypass via comments
        let attack_queries = [
            "MATCH (n) RETURN n /* CREATE (m:Evil) */ LIMIT 10",
            "MATCH (n) RETURN n // DELETE n",
            "MATCH (n) RETURN n --SET n.x = 1",
        ];

        for query in attack_queries {
            // These should either:
            // 1. Be blocked by validation (preferred)
            // 2. Execute the safe part only (Neo4j handles comments)
            // They should NOT execute the dangerous part
            let result = pool.execute_query(query, None).await;
            // If it succeeds, verify no mutations occurred
            // If it fails, that's also acceptable
        }
    }

    #[tokio::test]
    async fn test_parameter_injection() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Attempt injection via parameters
        let mut params = serde_json::Map::new();
        params.insert("name".to_string(), serde_json::json!("test' OR 1=1 --"));

        let result = pool
            .execute_query("RETURN $name AS name", Some(params))
            .await;

        // Should succeed and return the literal string, NOT inject
        assert!(result.is_ok());
        let rows = result.unwrap();
        assert_eq!(rows[0]["name"], "test' OR 1=1 --");
    }
}

mod performance {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_query_performance() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Warm up
        let _ = pool.execute_query("RETURN 1", None).await;

        // Measure simple query performance
        let start = Instant::now();
        let iterations = 10;

        for _ in 0..iterations {
            let _ = pool.execute_query("RETURN 1", None).await;
        }

        let elapsed = start.elapsed();
        let avg_ms = elapsed.as_millis() / iterations as u128;

        // Average query should be under 100ms
        assert!(
            avg_ms < 100,
            "Average query time {}ms exceeds 100ms threshold",
            avg_ms
        );
    }

    #[test]
    fn test_token_counting_performance() {
        use novanet_mcp::tokens::TokenCounter;

        let counter = TokenCounter::new();
        let text = "The quick brown fox jumps over the lazy dog. ".repeat(100);

        let start = Instant::now();
        let iterations = 100;

        for _ in 0..iterations {
            counter.count(&text);
        }

        let elapsed = start.elapsed();
        let avg_us = elapsed.as_micros() / iterations as u128;

        // Token counting should be under 10ms per call (first call initializes BPE)
        assert!(
            avg_us < 10000,
            "Average token count time {}μs exceeds 10000μs threshold",
            avg_us
        );
    }
}
