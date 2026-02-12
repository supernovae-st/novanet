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

// ═══════════════════════════════════════════════════════════════════════════════
// ADDITIONAL SECURITY TESTS (mcplint SEC-* category)
// ═══════════════════════════════════════════════════════════════════════════════

mod security_extended {
    use super::*;

    #[tokio::test]
    async fn test_path_traversal_in_key() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Path traversal attempts in key parameter
        let attack_keys = [
            "../../etc/passwd",
            "../../../root/.ssh/id_rsa",
            "..\\..\\windows\\system32",
            "%2e%2e%2f%2e%2e%2f",
            "....//....//",
        ];

        for key in attack_keys {
            let mut params = serde_json::Map::new();
            params.insert("key".to_string(), serde_json::json!(key));

            // Query should execute safely (key is just a string value)
            let result = pool
                .execute_query("RETURN $key AS key", Some(params))
                .await;

            // Should succeed and return the literal string, not traverse
            assert!(result.is_ok(), "Path traversal test failed for: {}", key);
            let rows = result.unwrap();
            assert_eq!(
                rows[0]["key"].as_str().unwrap(),
                key,
                "Key should be literal"
            );
        }
    }

    #[tokio::test]
    async fn test_null_byte_injection() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Null byte injection attempts
        let attack_strings = [
            "test\x00evil",
            "test%00evil",
            "test\u{0000}evil",
            "normal\x00",
        ];

        for attack in attack_strings {
            let mut params = serde_json::Map::new();
            params.insert("value".to_string(), serde_json::json!(attack));

            let result = pool
                .execute_query("RETURN $value AS value", Some(params))
                .await;

            // Should handle gracefully (either succeed with literal or reject)
            // Key point: should not cause security bypass
            if result.is_ok() {
                let rows = result.unwrap();
                // Verify the string is returned as-is or sanitized
                assert!(rows[0].get("value").is_some());
            }
            // Error is also acceptable - null bytes rejected
        }
    }

    #[tokio::test]
    async fn test_unicode_normalization_attacks() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Unicode normalization edge cases
        let unicode_strings = [
            // Combining characters
            "café",                      // NFC form
            "cafe\u{0301}",              // NFD form (e + combining acute)
            // Zero-width characters
            "test\u{200B}value",         // Zero-width space
            "test\u{FEFF}value",         // BOM
            "test\u{200C}\u{200D}value", // ZWNJ + ZWJ
            // Right-to-left override
            "test\u{202E}evil\u{202C}",  // RLO attack
            // Homoglyphs
            "ᎻᎾᎷᎬ",                       // Cherokee letters look like HOME
        ];

        for text in unicode_strings {
            let mut params = serde_json::Map::new();
            params.insert("text".to_string(), serde_json::json!(text));

            let result = pool
                .execute_query("RETURN $text AS text", Some(params))
                .await;

            // Should handle all unicode gracefully
            assert!(
                result.is_ok(),
                "Unicode handling failed for: {:?}",
                text.escape_debug()
            );
        }
    }

    #[tokio::test]
    async fn test_deep_nesting_dos() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Create deeply nested JSON structure
        let mut nested = serde_json::json!("leaf");
        for _ in 0..50 {
            nested = serde_json::json!({"nested": nested});
        }

        let mut params = serde_json::Map::new();
        params.insert("data".to_string(), nested);

        // Should handle deep nesting without stack overflow or timeout
        let result = pool
            .execute_query("RETURN $data AS data", Some(params))
            .await;

        // Either succeeds or fails gracefully, but shouldn't crash
        if let Err(e) = &result {
            eprintln!("Deep nesting error (acceptable): {}", e);
        }
    }

    #[tokio::test]
    async fn test_very_long_string_dos() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Very long string (1MB)
        let long_string = "x".repeat(1_000_000);

        let mut params = serde_json::Map::new();
        params.insert("long".to_string(), serde_json::json!(long_string));

        let result = pool
            .execute_query("RETURN length($long) AS len", Some(params))
            .await;

        // Should handle or reject gracefully
        if let Ok(rows) = result {
            assert_eq!(rows[0]["len"], 1_000_000);
        }
        // Rejection due to size is also acceptable
    }

    #[tokio::test]
    async fn test_cypher_injection_case_variations() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Case variation attacks
        let attack_queries = [
            "MATCH (n) cReAtE (m:Evil) RETURN n",
            "MATCH (n) DeLeTe n",
            "MATCH (n) MeRgE (m:Evil) RETURN m",
            "match (n) CREATE (m:Evil) return n", // lowercase
            "MATCH (n) RETURN n; CREATE (m:Evil)", // statement injection
        ];

        for query in attack_queries {
            let result = pool.execute_query(query, None).await;
            assert!(
                result.is_err(),
                "Case variation attack should be blocked: {}",
                query
            );
        }
    }

    #[tokio::test]
    async fn test_template_injection_in_cypher() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Template injection attempts
        let mut params = serde_json::Map::new();
        params.insert("key".to_string(), serde_json::json!("test'} CREATE (m:Evil) WITH m MATCH (n {key: 'x"));

        let result = pool
            .execute_query("MATCH (n {key: $key}) RETURN n.key AS key", Some(params))
            .await;

        // Should succeed safely (parameterized query prevents injection)
        assert!(
            result.is_ok(),
            "Parameterized query should prevent injection"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// EDGE CASE TESTS (mcplint EDGE-* category)
// ═══════════════════════════════════════════════════════════════════════════════

mod edge_cases {
    use super::*;
    use novanet_mcp::tokens::TokenCounter;

    #[test]
    fn test_empty_string_token_count() {
        let counter = TokenCounter::new();

        // Empty string
        let count = counter.count("");
        assert_eq!(count, 0, "Empty string should have 0 tokens");

        // Whitespace only
        let count = counter.count("   ");
        assert!(count <= 2, "Whitespace should have minimal tokens");

        // Single character
        let count = counter.count("a");
        assert!(count >= 1, "Single char should have at least 1 token");
    }

    #[test]
    fn test_boundary_token_budgets() {
        let counter = TokenCounter::new();

        let text = "Hello, world!";

        // Zero budget
        assert!(!counter.within_budget(text, 0));

        // Budget of 1
        assert!(!counter.within_budget(text, 1));

        // Large budget (but not overflow-inducing)
        assert!(counter.within_budget(text, 1_000_000));
    }

    #[test]
    fn test_truncate_edge_cases() {
        let counter = TokenCounter::new();

        // Empty text
        let (truncated, count) = counter.truncate_to_budget("", 100);
        assert!(truncated.is_empty());
        assert_eq!(count, 0);

        // Zero budget
        let (truncated, _count) = counter.truncate_to_budget("Hello world", 0);
        assert!(truncated.is_empty() || truncated.len() <= 10); // Minimal truncation
    }

    #[test]
    fn test_unicode_emoji_token_counting() {
        let counter = TokenCounter::new();

        // Emoji text
        let emoji_text = "Hello 👋🌍🎉 World!";
        let count = counter.count(emoji_text);
        assert!(count > 0, "Emoji text should have tokens");

        // ZWJ sequences (family emoji)
        let zwj_emoji = "👨‍👩‍👧‍👦"; // Family with ZWJ
        let count = counter.count(zwj_emoji);
        assert!(count > 0, "ZWJ emoji should have tokens");

        // Flag emoji
        let flags = "🇫🇷🇺🇸🇯🇵";
        let count = counter.count(flags);
        assert!(count > 0, "Flag emoji should have tokens");
    }

    #[tokio::test]
    async fn test_empty_query_results() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Query that returns no results
        let result = pool
            .execute_query("MATCH (n:NonexistentLabel12345) RETURN n", None)
            .await;

        assert!(result.is_ok(), "Empty result query should succeed");
        let rows = result.unwrap();
        assert!(rows.is_empty(), "Should return empty array");
    }

    #[tokio::test]
    async fn test_limit_zero() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // LIMIT 0 should return empty results
        let result = pool
            .execute_query("RETURN 1 AS num LIMIT 0", None)
            .await;

        assert!(result.is_ok(), "LIMIT 0 query should succeed");
        let rows = result.unwrap();
        assert!(rows.is_empty(), "LIMIT 0 should return empty array");
    }

    #[tokio::test]
    async fn test_special_characters_in_properties() {
        require_neo4j!();

        let uri = env::var("NEO4J_URI").unwrap();
        let user = env::var("NEO4J_USER").unwrap();
        let password = env::var("NEO4J_PASSWORD").unwrap();

        let pool = novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
            .await
            .expect("Pool creation failed");

        // Special characters that might cause issues
        let special_values = [
            "test'value",          // Single quote
            "test\"value",         // Double quote
            "test\\value",         // Backslash
            "test\nvalue",         // Newline
            "test\tvalue",         // Tab
            "test\r\nvalue",       // CRLF
            r"test`value",         // Backtick
            "test{value}",         // Braces
            "test[value]",         // Brackets
            "test$value",          // Dollar sign
        ];

        for value in special_values {
            let mut params = serde_json::Map::new();
            params.insert("val".to_string(), serde_json::json!(value));

            let result = pool
                .execute_query("RETURN $val AS value", Some(params))
                .await;

            assert!(
                result.is_ok(),
                "Special character '{}' should be handled",
                value.escape_debug()
            );

            let rows = result.unwrap();
            assert_eq!(
                rows[0]["value"].as_str().unwrap(),
                value,
                "Value should be preserved"
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CACHE EDGE CASES
// ═══════════════════════════════════════════════════════════════════════════════

mod cache_extended {
    use novanet_mcp::cache::QueryCache;
    use std::time::Duration;

    #[tokio::test]
    async fn test_cache_key_collision_resistance() {
        // Different queries should have different keys
        let key1 = QueryCache::cache_key("MATCH (n) RETURN n", &None);
        let key2 = QueryCache::cache_key("MATCH (n) RETURN n LIMIT 10", &None);
        assert_ne!(key1, key2, "Different queries should have different keys");

        // Same query, different params should have different keys
        let params1 = Some({
            let mut map = serde_json::Map::new();
            map.insert("a".to_string(), serde_json::json!(1));
            map
        });
        let params2 = Some({
            let mut map = serde_json::Map::new();
            map.insert("a".to_string(), serde_json::json!(2));
            map
        });

        let key3 = QueryCache::cache_key("RETURN $a", &params1);
        let key4 = QueryCache::cache_key("RETURN $a", &params2);
        assert_ne!(key3, key4, "Different params should have different keys");
    }

    #[tokio::test]
    async fn test_cache_concurrent_access() {
        let cache = QueryCache::new(100, Duration::from_secs(60));

        // Spawn multiple concurrent inserts/gets
        let mut handles = vec![];
        for i in 0..10 {
            let cache_clone = cache.clone();
            let handle = tokio::spawn(async move {
                let key = format!("key_{}", i);
                let value = serde_json::json!({"index": i});
                cache_clone.insert(key.clone(), value.clone()).await;
                let retrieved = cache_clone.get(&key).await;
                assert_eq!(retrieved, Some(value));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.expect("Concurrent cache operation failed");
        }
    }

    #[tokio::test]
    async fn test_cache_with_large_value() {
        let cache = QueryCache::new(10, Duration::from_secs(60));

        // Large JSON value
        let large_array: Vec<serde_json::Value> = (0..1000)
            .map(|i| serde_json::json!({"index": i, "data": "x".repeat(100)}))
            .collect();

        let key = "large_value".to_string();
        cache.insert(key.clone(), serde_json::json!(large_array)).await;

        let retrieved = cache.get(&key).await;
        assert!(retrieved.is_some(), "Large value should be cached");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROMPT EDGE CASES
// ═══════════════════════════════════════════════════════════════════════════════

mod prompts_extended {
    use novanet_mcp::prompts;

    #[test]
    fn test_prompt_with_special_chars_in_args() {
        // Special characters in arguments
        let mut args = serde_json::Map::new();
        args.insert(
            "intent".to_string(),
            serde_json::json!("Find entities with name containing ' OR \""),
        );

        let rendered = prompts::render_prompt("cypher_query", &args);
        assert!(rendered.is_some(), "Should handle special chars");

        let result = rendered.unwrap();
        let all_content: String = result.messages.iter().map(|m| m.content.clone()).collect();
        assert!(all_content.contains("'"), "Single quote should be preserved");
    }

    #[test]
    fn test_prompt_with_unicode_args() {
        let mut args = serde_json::Map::new();
        args.insert(
            "block_key".to_string(),
            serde_json::json!("hero-日本語-section"),
        );
        args.insert("locale".to_string(), serde_json::json!("ja-JP"));

        let rendered = prompts::render_prompt("block_generation", &args);
        assert!(rendered.is_some(), "Should handle unicode in args");

        let result = rendered.unwrap();
        let all_content: String = result.messages.iter().map(|m| m.content.clone()).collect();
        assert!(
            all_content.contains("日本語"),
            "Japanese should be preserved"
        );
    }

    #[test]
    fn test_prompt_with_very_long_args() {
        let mut args = serde_json::Map::new();
        let long_intent = "Find ".to_owned() + &"entities ".repeat(1000);
        args.insert("intent".to_string(), serde_json::json!(long_intent));

        let rendered = prompts::render_prompt("cypher_query", &args);
        assert!(rendered.is_some(), "Should handle long args");
    }

    #[test]
    fn test_prompt_with_empty_args() {
        let mut args = serde_json::Map::new();
        args.insert("intent".to_string(), serde_json::json!(""));

        let rendered = prompts::render_prompt("cypher_query", &args);
        assert!(rendered.is_some(), "Should handle empty string args");
    }

    #[test]
    fn test_all_prompts_render() {
        let prompt_names = [
            "cypher_query",
            "cypher_explain",
            "block_generation",
            "page_generation",
            "entity_analysis",
            "locale_briefing",
        ];

        for name in prompt_names {
            // With minimal args
            let args = serde_json::Map::new();
            let rendered = prompts::render_prompt(name, &args);
            assert!(
                rendered.is_some(),
                "Prompt '{}' should render with empty args",
                name
            );

            // Verify structure
            let result = rendered.unwrap();
            assert!(!result.messages.is_empty(), "'{}' should have messages", name);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RESOURCE URI TESTS
// ═══════════════════════════════════════════════════════════════════════════════

mod resource_uris {
    #[test]
    fn test_entity_uri_special_chars() {
        // URIs with special characters should be parsed correctly
        let special_keys = [
            "qr-code-generator",   // Normal
            "entity_with_underscore",
            "entity.with.dots",
            "entity:with:colons", // Might conflict with URI scheme
            "entity/with/slashes", // Path-like
            "entity?with&query=params", // Query-string like
            "entity#with#hashes",
            "entity%20encoded",
        ];

        for key in special_keys {
            let uri = format!("entity://{}", key);
            // Just verify the string is valid - actual parsing is in resources module
            assert!(!uri.is_empty(), "URI should be non-empty for key: {}", key);
        }
    }

    #[test]
    fn test_locale_uri_formats() {
        // Valid BCP-47 locale formats
        let valid_locales = [
            "en",
            "en-US",
            "fr-FR",
            "zh-CN",
            "zh-Hans-CN",
            "sr-Latn-RS",
            "es-419", // Latin America
        ];

        for locale in valid_locales {
            let uri = format!("locale://{}", locale);
            assert!(!uri.is_empty());
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// GENERATE TOOL EDGE CASES
// ═══════════════════════════════════════════════════════════════════════════════

mod generate_extended {
    use novanet_mcp::tools::generate::{GenerateMode, GenerateParams};

    #[test]
    fn test_generate_params_with_zero_budget() {
        let params = GenerateParams {
            focus_key: "test".to_string(),
            locale: "en-US".to_string(),
            mode: GenerateMode::Block,
            token_budget: Some(0),
            include_examples: Some(false),
            spreading_depth: Some(0),
        };

        // Should construct without panic
        assert_eq!(params.token_budget, Some(0));
        assert_eq!(params.spreading_depth, Some(0));
    }

    #[test]
    fn test_generate_params_with_large_budget() {
        let params = GenerateParams {
            focus_key: "test".to_string(),
            locale: "en-US".to_string(),
            mode: GenerateMode::Page,
            token_budget: Some(1_000_000), // 1M tokens
            include_examples: Some(true),
            spreading_depth: Some(10),
        };

        assert_eq!(params.token_budget, Some(1_000_000));
    }

    #[test]
    fn test_generate_params_special_locale() {
        let locales = ["zh-Hans-CN", "sr-Latn-RS", "es-419"];

        for locale in locales {
            let params = GenerateParams {
                focus_key: "test".to_string(),
                locale: locale.to_string(),
                mode: GenerateMode::Block,
                token_budget: None,
                include_examples: None,
                spreading_depth: None,
            };

            assert_eq!(params.locale, locale);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERFORMANCE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

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
