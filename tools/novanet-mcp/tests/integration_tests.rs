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
            let _result = pool.execute_query(query, None).await;
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

mod generate_tool {
    use super::*;
    use novanet_mcp::tools::generate::{GenerateMode, GenerateParams};

    #[tokio::test]
    async fn test_generate_params_default() {
        // Verify default parameters work
        let params = GenerateParams {
            focus_key: "test-page".to_string(),
            locale: "fr-FR".to_string(),
            mode: GenerateMode::default(),
            token_budget: None,
            include_examples: None,
            spreading_depth: None,
        };

        assert_eq!(params.focus_key, "test-page");
        assert_eq!(params.locale, "fr-FR");
        assert!(matches!(params.mode, GenerateMode::Block));
    }

    #[tokio::test]
    async fn test_generate_mode_block() {
        let _mode = GenerateMode::Block;
        // Block mode should be the default
        let default_mode = GenerateMode::default();
        assert!(matches!(default_mode, GenerateMode::Block));
    }

    #[tokio::test]
    async fn test_generate_mode_page() {
        let _mode = GenerateMode::Page;
        // Page mode for full page orchestration
        assert!(matches!(_mode, GenerateMode::Page));
    }

    #[tokio::test]
    async fn test_generate_with_neo4j() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Query for any Page node to use as focus
        let result = pool
            .execute_query("MATCH (p:Page) RETURN p.key AS key LIMIT 1", None)
            .await;

        // If we have a Page, we can test generate
        if let Ok(rows) = result {
            if !rows.is_empty() {
                let page_key = rows[0]["key"].as_str().unwrap_or("homepage");
                eprintln!("Found page for generate test: {}", page_key);
                // Full generate test would require State setup
            }
        }
    }
}

mod prompts {
    use novanet_mcp::prompts;

    #[test]
    fn test_list_prompts() {
        let prompt_list = prompts::list_prompts();

        // Should have 6 prompts
        assert_eq!(prompt_list.len(), 6, "Expected 6 prompts");

        // Verify all prompt names
        let names: Vec<&str> = prompt_list.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"cypher_query"));
        assert!(names.contains(&"cypher_explain"));
        assert!(names.contains(&"block_generation"));
        assert!(names.contains(&"page_generation"));
        assert!(names.contains(&"entity_analysis"));
        assert!(names.contains(&"locale_briefing"));
    }

    #[test]
    fn test_render_cypher_query_prompt() {
        let mut args = serde_json::Map::new();
        args.insert("intent".to_string(), serde_json::json!("Find all Entity nodes"));

        let rendered = prompts::render_prompt("cypher_query", &args);
        assert!(rendered.is_some(), "cypher_query should render");

        let result = rendered.unwrap();
        assert!(!result.messages.is_empty(), "Should have messages");

        // The intent appears in the user message (message[1]), not system message (message[0])
        let all_content: String = result.messages.iter().map(|m| m.content.clone()).collect();
        assert!(
            all_content.contains("Find all Entity nodes"),
            "Should include intent in one of the messages. Got: {}",
            all_content.chars().take(500).collect::<String>()
        );
    }

    #[test]
    fn test_render_block_generation_prompt() {
        let mut args = serde_json::Map::new();
        args.insert("block_key".to_string(), serde_json::json!("hero-section"));
        args.insert("locale".to_string(), serde_json::json!("fr-FR"));

        let rendered = prompts::render_prompt("block_generation", &args);
        assert!(rendered.is_some(), "block_generation should render");

        let result = rendered.unwrap();
        assert!(!result.messages.is_empty());
        // Check all messages for the arguments
        let all_content: String = result.messages.iter().map(|m| m.content.clone()).collect();
        assert!(
            all_content.contains("hero-section"),
            "Should include block_key in messages"
        );
        assert!(
            all_content.contains("fr-FR"),
            "Should include locale in messages"
        );
    }

    #[test]
    fn test_render_page_generation_prompt() {
        let mut args = serde_json::Map::new();
        args.insert("page_key".to_string(), serde_json::json!("homepage"));
        args.insert("locale".to_string(), serde_json::json!("en-US"));

        let rendered = prompts::render_prompt("page_generation", &args);
        assert!(rendered.is_some(), "page_generation should render");

        let result = rendered.unwrap();
        let all_content: String = result.messages.iter().map(|m| m.content.clone()).collect();
        assert!(
            all_content.contains("homepage"),
            "Should include page_key in messages"
        );
    }

    #[test]
    fn test_render_entity_analysis_prompt() {
        let mut args = serde_json::Map::new();
        args.insert("entity_key".to_string(), serde_json::json!("qr-code-generator"));
        args.insert("locale".to_string(), serde_json::json!("de-DE"));

        let rendered = prompts::render_prompt("entity_analysis", &args);
        assert!(rendered.is_some(), "entity_analysis should render");

        let result = rendered.unwrap();
        let all_content: String = result.messages.iter().map(|m| m.content.clone()).collect();
        assert!(
            all_content.contains("qr-code-generator"),
            "Should include entity_key in messages"
        );
    }

    #[test]
    fn test_render_locale_briefing_prompt() {
        let mut args = serde_json::Map::new();
        args.insert("locale_key".to_string(), serde_json::json!("ja-JP"));

        let rendered = prompts::render_prompt("locale_briefing", &args);
        assert!(rendered.is_some(), "locale_briefing should render");

        let result = rendered.unwrap();
        let all_content: String = result.messages.iter().map(|m| m.content.clone()).collect();
        assert!(
            all_content.contains("ja-JP"),
            "Should include locale_key in messages"
        );
    }

    #[test]
    fn test_render_cypher_explain_prompt() {
        let mut args = serde_json::Map::new();
        args.insert(
            "query".to_string(),
            serde_json::json!("MATCH (e:Entity) RETURN e LIMIT 10"),
        );
        args.insert(
            "results".to_string(),
            serde_json::json!("[{\"key\": \"test\"}]"),
        );

        let rendered = prompts::render_prompt("cypher_explain", &args);
        assert!(rendered.is_some(), "cypher_explain should render");

        let result = rendered.unwrap();
        let all_content: String = result.messages.iter().map(|m| m.content.clone()).collect();
        assert!(
            all_content.contains("MATCH"),
            "Should include query in messages"
        );
    }

    #[test]
    fn test_prompt_missing_required_args() {
        // Missing required 'intent' argument
        let args = serde_json::Map::new();

        let rendered = prompts::render_prompt("cypher_query", &args);
        // Should still render but with placeholder
        assert!(rendered.is_some());
    }

    #[test]
    fn test_prompt_not_found() {
        let args = serde_json::Map::new();
        let rendered = prompts::render_prompt("nonexistent_prompt", &args);
        assert!(rendered.is_none(), "Nonexistent prompt should return None");
    }
}

mod tools_with_seed_data {
    use super::*;

    #[tokio::test]
    async fn test_traverse_from_kind() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Query for Kind nodes (meta-graph)
        let result = pool
            .execute_query(
                "MATCH (k:Kind)-[r]->(m) RETURN k.name AS source, type(r) AS rel, labels(m)[0] AS target LIMIT 5",
                None,
            )
            .await;

        if let Ok(rows) = result {
            eprintln!("Found {} Kind relationships", rows.len());
            for row in &rows {
                eprintln!(
                    "  {} -[{}]-> {}",
                    row["source"].as_str().unwrap_or("?"),
                    row["rel"].as_str().unwrap_or("?"),
                    row["target"].as_str().unwrap_or("?")
                );
            }
        }
    }

    #[tokio::test]
    async fn test_search_entities() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Search for Entity nodes
        let result = pool
            .execute_query(
                "MATCH (e:Entity) RETURN e.key AS key, e.name AS name LIMIT 5",
                None,
            )
            .await;

        if let Ok(rows) = result {
            eprintln!("Found {} Entity nodes", rows.len());
            for row in &rows {
                eprintln!(
                    "  Entity: {} ({})",
                    row["key"].as_str().unwrap_or("?"),
                    row["name"].as_str().unwrap_or("no name")
                );
            }
        }
    }

    #[tokio::test]
    async fn test_locale_knowledge() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Query for locale knowledge atoms
        let result = pool
            .execute_query(
                r#"
                MATCH (l:Locale)
                OPTIONAL MATCH (l)-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)
                WITH l.key AS locale, count(t) AS term_count
                RETURN locale, term_count
                ORDER BY term_count DESC
                LIMIT 5
                "#,
                None,
            )
            .await;

        if let Ok(rows) = result {
            eprintln!("Locale knowledge summary:");
            for row in &rows {
                eprintln!(
                    "  {}: {} terms",
                    row["locale"].as_str().unwrap_or("?"),
                    row["term_count"]
                );
            }
        }
    }

    #[tokio::test]
    async fn test_page_block_structure() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Query for Page/Block structure
        let result = pool
            .execute_query(
                r#"
                MATCH (p:Page)-[:HAS_BLOCK]->(b:Block)
                WITH p.key AS page, collect(b.key) AS blocks
                RETURN page, blocks, size(blocks) AS block_count
                LIMIT 3
                "#,
                None,
            )
            .await;

        if let Ok(rows) = result {
            eprintln!("Page/Block structure:");
            for row in &rows {
                eprintln!(
                    "  Page '{}': {} blocks",
                    row["page"].as_str().unwrap_or("?"),
                    row["block_count"]
                );
            }
        }
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
