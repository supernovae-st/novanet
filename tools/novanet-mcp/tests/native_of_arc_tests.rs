//! Integration tests for NATIVE_OF arc integrity (ADR-029)
//!
//! NATIVE_OF is the inverse arc of HAS_NATIVE:
//! - (Entity)-[:HAS_NATIVE]->(EntityNative)
//! - (EntityNative)-[:NATIVE_OF]->(Entity)
//!
//! These tests verify that all *Native nodes have the required NATIVE_OF arc
//! back to their parent entity, enabling bidirectional traversal.
//!
//! Run with: NEO4J_PASSWORD=novanetpassword cargo test --test native_of_arc_tests
//!
//! Phase 1.1 of Spreading Activation v2 plan.

use std::env;

/// Test helper to check if Neo4j is available
fn neo4j_available() -> bool {
    env::var("NEO4J_URI").is_ok() || env::var("NEO4J_PASSWORD").is_ok()
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

/// Create Neo4j pool for testing
async fn create_test_pool() -> novanet_mcp::neo4j::Neo4jPool {
    let uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
    let user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
    let password = env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD required");

    novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
        .await
        .expect("Pool creation failed")
}

// ═══════════════════════════════════════════════════════════════════════════════
// NATIVE_OF ARC INTEGRITY TESTS
// Verify that all *Native nodes have NATIVE_OF arcs back to their parent
// ═══════════════════════════════════════════════════════════════════════════════

mod native_of_integrity {
    use super::*;

    /// Test that ALL EntityNative nodes have a NATIVE_OF arc back to Entity
    ///
    /// This test should FAIL if any EntityNative is orphaned (missing NATIVE_OF arc).
    /// The Cypher query finds EntityNatives WITHOUT the required inverse arc.
    #[tokio::test]
    async fn test_entity_native_has_native_of_arc() {
        require_neo4j!();
        let pool = create_test_pool().await;

        // Query: Find EntityNatives that are MISSING the NATIVE_OF arc
        // Expected: count should be 0 (all EntityNatives have NATIVE_OF)
        let result = pool
            .execute_query(
                r#"
                MATCH (en:EntityNative)
                WHERE NOT (en)-[:NATIVE_OF]->(:Entity)
                RETURN count(en) AS orphan_count
                "#,
                None,
            )
            .await
            .expect("Query should execute");

        let orphan_count = result[0]["orphan_count"].as_i64().unwrap_or(-1);

        // This assertion should FAIL initially if there are orphaned EntityNatives
        assert_eq!(
            orphan_count, 0,
            "Found {} EntityNative nodes WITHOUT NATIVE_OF arc to Entity. \
             All EntityNative nodes MUST have (EntityNative)-[:NATIVE_OF]->(Entity) arc.",
            orphan_count
        );
    }

    /// Test that ALL PageNative nodes have a NATIVE_OF arc back to Page
    #[tokio::test]
    async fn test_page_native_has_native_of_arc() {
        require_neo4j!();
        let pool = create_test_pool().await;

        let result = pool
            .execute_query(
                r#"
                MATCH (pn:PageNative)
                WHERE NOT (pn)-[:NATIVE_OF]->(:Page)
                RETURN count(pn) AS orphan_count
                "#,
                None,
            )
            .await
            .expect("Query should execute");

        let orphan_count = result[0]["orphan_count"].as_i64().unwrap_or(-1);

        assert_eq!(
            orphan_count, 0,
            "Found {} PageNative nodes WITHOUT NATIVE_OF arc to Page. \
             All PageNative nodes MUST have (PageNative)-[:NATIVE_OF]->(Page) arc.",
            orphan_count
        );
    }

    /// Test that ALL BlockNative nodes have a NATIVE_OF arc back to Block
    #[tokio::test]
    async fn test_block_native_has_native_of_arc() {
        require_neo4j!();
        let pool = create_test_pool().await;

        let result = pool
            .execute_query(
                r#"
                MATCH (bn:BlockNative)
                WHERE NOT (bn)-[:NATIVE_OF]->(:Block)
                RETURN count(bn) AS orphan_count
                "#,
                None,
            )
            .await
            .expect("Query should execute");

        let orphan_count = result[0]["orphan_count"].as_i64().unwrap_or(-1);

        assert_eq!(
            orphan_count, 0,
            "Found {} BlockNative nodes WITHOUT NATIVE_OF arc to Block. \
             All BlockNative nodes MUST have (BlockNative)-[:NATIVE_OF]->(Block) arc.",
            orphan_count
        );
    }

    /// Test that ALL ProjectNative nodes have a NATIVE_OF arc back to Project
    #[tokio::test]
    async fn test_project_native_has_native_of_arc() {
        require_neo4j!();
        let pool = create_test_pool().await;

        let result = pool
            .execute_query(
                r#"
                MATCH (pn:ProjectNative)
                WHERE NOT (pn)-[:NATIVE_OF]->(:Project)
                RETURN count(pn) AS orphan_count
                "#,
                None,
            )
            .await
            .expect("Query should execute");

        let orphan_count = result[0]["orphan_count"].as_i64().unwrap_or(-1);

        assert_eq!(
            orphan_count, 0,
            "Found {} ProjectNative nodes WITHOUT NATIVE_OF arc to Project. \
             All ProjectNative nodes MUST have (ProjectNative)-[:NATIVE_OF]->(Project) arc.",
            orphan_count
        );
    }

    /// Comprehensive test: ALL *Native nodes should have NATIVE_OF arcs
    ///
    /// This is a catch-all test that checks all Native pattern classes.
    #[tokio::test]
    async fn test_all_native_nodes_have_native_of_arc() {
        require_neo4j!();
        let pool = create_test_pool().await;

        // Query all *Native nodes that are missing NATIVE_OF arcs
        let result = pool
            .execute_query(
                r#"
                MATCH (n)
                WHERE any(label IN labels(n) WHERE label ENDS WITH 'Native')
                  AND NOT (n)-[:NATIVE_OF]->()
                RETURN labels(n) AS labels, n.key AS key, count(*) AS count
                ORDER BY count DESC
                LIMIT 20
                "#,
                None,
            )
            .await
            .expect("Query should execute");

        if !result.is_empty() {
            eprintln!("=== ORPHANED *Native NODES (missing NATIVE_OF) ===");
            for row in &result {
                eprintln!("  {:?} - key: {:?}", row.get("labels"), row.get("key"));
            }
        }

        assert!(
            result.is_empty(),
            "Found {} *Native node types WITHOUT NATIVE_OF arcs. \
             Per ADR-029, all *Native nodes MUST have a NATIVE_OF arc to their parent.",
            result.len()
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BIDIRECTIONAL TRAVERSAL TESTS
// Verify we can traverse Entity <-> EntityNative in both directions
// ═══════════════════════════════════════════════════════════════════════════════

mod bidirectional_traversal {
    use super::*;

    /// Test traversal from Entity to EntityNative via HAS_NATIVE
    /// AND back via NATIVE_OF
    ///
    /// This verifies the bidirectional relationship is intact.
    #[tokio::test]
    async fn test_traverse_entity_to_native_bidirectional() {
        require_neo4j!();
        let pool = create_test_pool().await;

        // Step 1: Find an Entity with at least one EntityNative
        let entities_with_native = pool
            .execute_query(
                r#"
                MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
                RETURN e.key AS entity_key, en.key AS native_key
                LIMIT 5
                "#,
                None,
            )
            .await
            .expect("Query should execute");

        if entities_with_native.is_empty() {
            eprintln!("No Entity->EntityNative relationships found, skipping test");
            return;
        }

        eprintln!(
            "Found {} Entity->EntityNative pairs to test",
            entities_with_native.len()
        );

        // Step 2: For each pair, verify bidirectional traversal works
        for row in &entities_with_native {
            let entity_key = row["entity_key"].as_str().unwrap_or("");
            let native_key = row["native_key"].as_str().unwrap_or("");

            // Forward: Entity -> EntityNative via HAS_NATIVE
            let forward = pool
                .execute_query(
                    r#"
                    MATCH (e:Entity {key: $entity_key})-[:HAS_NATIVE]->(en:EntityNative {key: $native_key})
                    RETURN count(*) AS found
                    "#,
                    Some({
                        let mut params = serde_json::Map::new();
                        params.insert("entity_key".to_string(), serde_json::json!(entity_key));
                        params.insert("native_key".to_string(), serde_json::json!(native_key));
                        params
                    }),
                )
                .await
                .expect("Forward traversal query should execute");

            let forward_found = forward[0]["found"].as_i64().unwrap_or(0);
            assert_eq!(
                forward_found, 1,
                "HAS_NATIVE arc from {} to {} should exist",
                entity_key, native_key
            );

            // Backward: EntityNative -> Entity via NATIVE_OF
            let backward = pool
                .execute_query(
                    r#"
                    MATCH (en:EntityNative {key: $native_key})-[:NATIVE_OF]->(e:Entity {key: $entity_key})
                    RETURN count(*) AS found
                    "#,
                    Some({
                        let mut params = serde_json::Map::new();
                        params.insert("entity_key".to_string(), serde_json::json!(entity_key));
                        params.insert("native_key".to_string(), serde_json::json!(native_key));
                        params
                    }),
                )
                .await
                .expect("Backward traversal query should execute");

            let backward_found = backward[0]["found"].as_i64().unwrap_or(0);
            assert_eq!(
                backward_found, 1,
                "NATIVE_OF arc from {} to {} MUST exist (ADR-029 inverse arc)",
                native_key, entity_key
            );

            eprintln!(
                "  [OK] {} <--HAS_NATIVE/NATIVE_OF--> {}",
                entity_key, native_key
            );
        }
    }

    /// Test that novanet_traverse can follow NATIVE_OF arcs in incoming direction
    ///
    /// This simulates what the spreading activation algorithm needs to do.
    #[tokio::test]
    async fn test_traverse_incoming_native_of_arcs() {
        require_neo4j!();
        let pool = create_test_pool().await;

        // Find an EntityNative that should have NATIVE_OF arc
        let natives = pool
            .execute_query(
                r#"
                MATCH (en:EntityNative)
                RETURN en.key AS key
                LIMIT 1
                "#,
                None,
            )
            .await
            .expect("Query should execute");

        if natives.is_empty() {
            eprintln!("No EntityNative nodes found, skipping test");
            return;
        }

        let native_key = natives[0]["key"].as_str().unwrap_or("");

        // Query: From EntityNative, traverse OUTGOING NATIVE_OF to get Entity
        let traversal = pool
            .execute_query(
                r#"
                MATCH (en:EntityNative {key: $key})-[r:NATIVE_OF]->(e:Entity)
                RETURN e.key AS entity_key, type(r) AS arc_type
                "#,
                Some({
                    let mut params = serde_json::Map::new();
                    params.insert("key".to_string(), serde_json::json!(native_key));
                    params
                }),
            )
            .await
            .expect("Traversal query should execute");

        assert!(
            !traversal.is_empty(),
            "EntityNative '{}' should have NATIVE_OF arc to Entity. \
             This is required for spreading activation to work bidirectionally.",
            native_key
        );

        eprintln!(
            "EntityNative '{}' has NATIVE_OF -> Entity '{}'",
            native_key,
            traversal[0]["entity_key"].as_str().unwrap_or("?")
        );
    }

    /// Test round-trip: Entity -> HAS_NATIVE -> EntityNative -> NATIVE_OF -> same Entity
    #[tokio::test]
    async fn test_round_trip_entity_native_entity() {
        require_neo4j!();
        let pool = create_test_pool().await;

        // Find entities that have natives
        let result = pool
            .execute_query(
                r#"
                MATCH (e1:Entity)-[:HAS_NATIVE]->(en:EntityNative)-[:NATIVE_OF]->(e2:Entity)
                WHERE e1 = e2
                RETURN e1.key AS entity_key, en.key AS native_key,
                       e1.key = e2.key AS same_entity
                LIMIT 10
                "#,
                None,
            )
            .await
            .expect("Query should execute");

        if result.is_empty() {
            // This test FAILS if no round-trip pairs exist (missing NATIVE_OF)
            let has_native_count = pool
                .execute_query(
                    "MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative) RETURN count(*) AS c",
                    None,
                )
                .await
                .expect("Count query should work");

            let count = has_native_count[0]["c"].as_i64().unwrap_or(0);

            if count > 0 {
                panic!(
                    "Found {} Entity->EntityNative pairs via HAS_NATIVE, \
                     but ZERO complete round-trips via HAS_NATIVE->NATIVE_OF. \
                     This means NATIVE_OF arcs are MISSING!",
                    count
                );
            } else {
                eprintln!("No Entity->EntityNative relationships in database, skipping test");
                return;
            }
        }

        eprintln!("=== ROUND-TRIP VERIFICATION ===");
        for row in &result {
            let same = row["same_entity"].as_bool().unwrap_or(false);
            assert!(
                same,
                "Round-trip should return to same Entity. Entity: {:?}, Native: {:?}",
                row.get("entity_key"),
                row.get("native_key")
            );
            eprintln!(
                "  [OK] {} -> {} -> {} (round-trip verified)",
                row["entity_key"].as_str().unwrap_or("?"),
                row["native_key"].as_str().unwrap_or("?"),
                row["entity_key"].as_str().unwrap_or("?")
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONSISTENCY CHECK: HAS_NATIVE COUNT == NATIVE_OF COUNT
// ═══════════════════════════════════════════════════════════════════════════════

mod consistency_checks {
    use super::*;

    /// Verify that HAS_NATIVE and NATIVE_OF arc counts match
    ///
    /// If they don't match, some NATIVE_OF arcs are missing.
    #[tokio::test]
    async fn test_has_native_count_equals_native_of_count() {
        require_neo4j!();
        let pool = create_test_pool().await;

        // Count HAS_NATIVE arcs
        let has_native = pool
            .execute_query("MATCH ()-[r:HAS_NATIVE]->() RETURN count(r) AS count", None)
            .await
            .expect("Query should execute");

        // Count NATIVE_OF arcs
        let native_of = pool
            .execute_query("MATCH ()-[r:NATIVE_OF]->() RETURN count(r) AS count", None)
            .await
            .expect("Query should execute");

        let has_native_count = has_native[0]["count"].as_i64().unwrap_or(0);
        let native_of_count = native_of[0]["count"].as_i64().unwrap_or(0);

        eprintln!("HAS_NATIVE arcs: {}", has_native_count);
        eprintln!("NATIVE_OF arcs:  {}", native_of_count);

        assert_eq!(
            has_native_count,
            native_of_count,
            "HAS_NATIVE ({}) and NATIVE_OF ({}) arc counts MUST match. \
             Difference of {} indicates missing inverse arcs.",
            has_native_count,
            native_of_count,
            (has_native_count - native_of_count).abs()
        );
    }

    /// List specific *Native nodes missing NATIVE_OF for debugging
    #[tokio::test]
    async fn test_list_orphaned_natives_for_debugging() {
        require_neo4j!();
        let pool = create_test_pool().await;

        let result = pool
            .execute_query(
                r#"
                MATCH (n)
                WHERE any(label IN labels(n) WHERE label ENDS WITH 'Native')
                  AND NOT (n)-[:NATIVE_OF]->()
                WITH labels(n) AS nodeLabels, n.key AS nodeKey, n.locale AS locale
                RETURN nodeLabels, nodeKey, locale
                ORDER BY nodeLabels, nodeKey
                LIMIT 50
                "#,
                None,
            )
            .await
            .expect("Query should execute");

        if !result.is_empty() {
            eprintln!("\n=== ORPHANED *Native NODES (for fixing) ===");
            eprintln!("These nodes need NATIVE_OF arcs created:\n");
            for row in &result {
                eprintln!(
                    "  - {:?} | key: {:?} | locale: {:?}",
                    row.get("nodeLabels"),
                    row.get("nodeKey"),
                    row.get("locale")
                );
            }
            eprintln!("\nTotal: {} orphaned nodes\n", result.len());
        }

        // This assertion ensures the test fails if there are orphaned nodes
        assert!(
            result.is_empty(),
            "Found {} orphaned *Native nodes missing NATIVE_OF arcs. See output above.",
            result.len()
        );
    }
}
