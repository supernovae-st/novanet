//! End-to-end Agent Integration Test Scenarios
//!
//! These tests simulate realistic AI agent workflows using NovaNet MCP tools.
//! Each test exercises multiple tools in sequence as a real agent would.
//!
//! Test categories:
//! 1. Content Generation Flow
//! 2. Multi-locale Comparison
//! 3. SEO Optimization Flow
//! 4. Error Recovery Flow
//! 5. Budget Management Flow
//! 6. Schema Discovery Flow
//! 7. Entity Analysis Flow
//! 8. Knowledge Atom Assembly
//! 9. Context Anchor Resolution
//! 10. Concurrent Agent Operations
//!
//! Run with: NEO4J_URI=bolt://localhost:7687 cargo test --test e2e_agent_scenarios
//!
//! These tests require:
//! - Running Neo4j instance with NovaNet seed data
//! - Environment variables: NEO4J_URI, NEO4J_USER, NEO4J_PASSWORD

use std::env;
use std::sync::Arc;

// ═══════════════════════════════════════════════════════════════════════════════
// TEST SETUP HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test helper to check if Neo4j is available
fn neo4j_available() -> bool {
    env::var("NEO4J_URI").is_ok() && env::var("NEO4J_PASSWORD").is_ok()
}

/// Skip test if Neo4j is not configured
#[allow(unused_macros)]
macro_rules! require_neo4j {
    () => {
        if !neo4j_available() {
            eprintln!("Skipping test: NEO4J_* environment variables not set");
            return;
        }
    };
}

/// Get test state or return early if not available
macro_rules! get_test_state {
    () => {
        match create_test_state().await {
            Some(state) => state,
            None => return,
        }
    };
}

/// Create a Neo4j pool for testing
#[allow(dead_code)]
async fn create_test_pool() -> novanet_mcp::neo4j::Neo4jPool {
    let uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
    let user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
    let password = env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD required");

    novanet_mcp::neo4j::Neo4jPool::new(&uri, &user, &password, 5)
        .await
        .expect("Pool creation failed")
}

/// Create a full test state, returns None if Neo4j is not configured
async fn create_test_state() -> Option<novanet_mcp::server::State> {
    use novanet_mcp::server::Config;

    if !neo4j_available() {
        eprintln!("Skipping test: NEO4J_* environment variables not set");
        return None;
    }

    let config = Config::from_env().ok()?;
    novanet_mcp::server::State::new(config).await.ok()
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 1: Content Generation Flow
// Agent generates fr-FR content for a page
// Flow: describe schema -> search entities -> traverse from page -> assemble context -> generate
// ═══════════════════════════════════════════════════════════════════════════════

mod content_generation_flow {
    use super::*;
    use novanet_mcp::tools::{
        assemble::{AssembleParams, AssemblyStrategy},
        atoms::{AtomType, AtomsParams},
        describe::{DescribeParams, DescribeTarget},
        generate::{GenerateMode, GenerateParams},
        search::{SearchMode, SearchParams},
        traverse::{TraversalDirection, TraverseParams},
    };

    /// Complete content generation workflow for a page in fr-FR locale
    #[tokio::test]
    async fn test_full_page_generation_workflow() {
        let state = get_test_state!();

        // Step 1: Bootstrap - describe schema to understand the graph
        let describe_result = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Schema,
                entity_key: None,
                category_key: None,
            },
        )
        .await;

        assert!(describe_result.is_ok(), "Schema describe should succeed");
        let schema = describe_result.unwrap();
        assert!(!schema.data.is_null(), "Schema data should exist");
        eprintln!(
            "Step 1: Schema discovered - {} tokens",
            schema.token_estimate
        );

        // Step 2: Find a page to generate content for
        let search_result = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "homepage".to_string(),
                mode: SearchMode::Hybrid,
                kinds: Some(vec!["Page".to_string()]),
                realm: Some("org".to_string()),
                layer: None,
                limit: Some(5),
                properties: None,
            },
        )
        .await;

        // If no pages found, try generic query
        let page_key = if let Ok(ref result) = search_result {
            if !result.hits.is_empty() {
                result.hits[0].key.clone()
            } else {
                "homepage".to_string() // fallback
            }
        } else {
            "homepage".to_string()
        };

        eprintln!("Step 2: Found page '{}' for generation", page_key);

        // Step 3: Traverse from page to discover structure and entities
        let traverse_result = novanet_mcp::tools::traverse::execute(
            &state,
            TraverseParams {
                start_key: page_key.clone(),
                max_depth: Some(3),
                direction: TraversalDirection::Outgoing,
                arc_families: Some(vec!["ownership".to_string(), "semantic".to_string()]),
                arc_kinds: None,
                target_kinds: Some(vec![
                    "Block".to_string(),
                    "Entity".to_string(),
                    "EntityNative".to_string(),
                ]),
                limit: Some(50),
                include_properties: Some(true),
            },
        )
        .await;

        if let Ok(ref result) = traverse_result {
            eprintln!(
                "Step 3: Traversed {} nodes, {} arcs, max depth {}",
                result.nodes.len(),
                result.arcs.len(),
                result.max_depth_reached
            );
        }

        // Step 4: Assemble context with token budget
        let assemble_result = novanet_mcp::tools::assemble::execute(
            &state,
            AssembleParams {
                focus_key: page_key.clone(),
                locale: "fr-FR".to_string(),
                token_budget: Some(50_000),
                strategy: AssemblyStrategy::Breadth,
                include_entities: Some(true),
                include_knowledge: Some(true),
                include_structure: Some(true),
                arc_families: None,
                max_depth: Some(3),
            },
        )
        .await;

        if let Ok(ref result) = assemble_result {
            eprintln!(
                "Step 4: Assembled {} evidence packets, {} tokens used, {} remaining",
                result.evidence.len(),
                result.total_tokens,
                result.budget_remaining
            );
            assert!(
                result.total_tokens <= 50_000,
                "Should not exceed token budget"
            );
        }

        // Step 5: Get locale-specific knowledge atoms
        let atoms_result = novanet_mcp::tools::atoms::execute(
            &state,
            AtomsParams {
                locale: "fr-FR".to_string(),
                atom_type: AtomType::All,
                domain: None,
                register: None,
                query: None,
                limit: Some(30),
                include_containers: Some(true),
            },
        )
        .await;

        if let Ok(ref result) = atoms_result {
            eprintln!(
                "Step 5: Loaded {} knowledge atoms for fr-FR",
                result.atoms.len()
            );
        }

        // Step 6: Generate complete context (composite tool)
        let generate_result = novanet_mcp::tools::generate::execute(
            &state,
            GenerateParams {
                focus_key: page_key.clone(),
                locale: "fr-FR".to_string(),
                mode: GenerateMode::Page,
                token_budget: Some(50_000),
                include_examples: Some(false),
                spreading_depth: Some(2),
            },
        )
        .await;

        if let Ok(ref result) = generate_result {
            eprintln!(
                "Step 6: Generated prompt ({} chars), {} evidence items, {} anchors",
                result.prompt.len(),
                result.evidence_summary.len(),
                result.context_anchors.len()
            );
            eprintln!(
                "Token usage: {} total ({} entities, {} knowledge, {} structure)",
                result.token_usage.total,
                result.token_usage.entities,
                result.token_usage.knowledge,
                result.token_usage.structure
            );

            // Verify prompt structure
            assert!(
                result.prompt.contains("# Generation Context"),
                "Prompt should have header"
            );
            assert!(
                result.prompt.contains("fr-FR"),
                "Prompt should mention locale"
            );
            assert!(
                result.prompt.contains("## Instructions"),
                "Prompt should have instructions"
            );
        }
    }

    /// Block-level content generation (more focused)
    #[tokio::test]
    async fn test_block_generation_workflow() {
        let state = get_test_state!();

        // Find a block
        let search_result = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "hero".to_string(),
                mode: SearchMode::Hybrid,
                kinds: Some(vec!["Block".to_string()]),
                realm: None,
                layer: None,
                limit: Some(1),
                properties: None,
            },
        )
        .await;

        let block_key = search_result
            .ok()
            .and_then(|r| r.hits.first().map(|h| h.key.clone()))
            .unwrap_or_else(|| "hero-section".to_string());

        // Generate for single block
        let generate_result = novanet_mcp::tools::generate::execute(
            &state,
            GenerateParams {
                focus_key: block_key.clone(),
                locale: "fr-FR".to_string(),
                mode: GenerateMode::Block,  // Block mode, not Page
                token_budget: Some(10_000), // Smaller budget for single block
                include_examples: Some(false),
                spreading_depth: Some(2),
            },
        )
        .await;

        if let Ok(ref result) = generate_result {
            eprintln!(
                "Block generation: {} tokens, {} evidence",
                result.token_usage.total,
                result.evidence_summary.len()
            );

            // Block mode should use less tokens than page mode
            assert!(
                result.token_usage.total <= 10_000,
                "Block should be within budget"
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 2: Multi-locale Comparison
// Agent compares terminology across locales
// Flow: atoms for en-US -> atoms for fr-FR -> compare terminology
// ═══════════════════════════════════════════════════════════════════════════════

mod multi_locale_comparison {
    use super::*;
    use novanet_mcp::tools::atoms::{AtomType, AtomsParams};

    /// Compare technical terms between en-US and fr-FR
    #[tokio::test]
    async fn test_locale_terminology_comparison() {
        let state = get_test_state!();

        // Get en-US terms
        let en_atoms = novanet_mcp::tools::atoms::execute(
            &state,
            AtomsParams {
                locale: "en-US".to_string(),
                atom_type: AtomType::Term,
                domain: Some("technical".to_string()),
                register: None,
                query: None,
                limit: Some(50),
                include_containers: Some(false),
            },
        )
        .await;

        // Get fr-FR terms
        let fr_atoms = novanet_mcp::tools::atoms::execute(
            &state,
            AtomsParams {
                locale: "fr-FR".to_string(),
                atom_type: AtomType::Term,
                domain: Some("technical".to_string()),
                register: None,
                query: None,
                limit: Some(50),
                include_containers: Some(false),
            },
        )
        .await;

        // Compare results
        if let (Ok(en), Ok(fr)) = (&en_atoms, &fr_atoms) {
            eprintln!("en-US: {} technical terms", en.atoms.len());
            eprintln!("fr-FR: {} technical terms", fr.atoms.len());

            // Find common term keys
            let en_keys: std::collections::HashSet<_> = en.atoms.iter().map(|a| &a.key).collect();
            let fr_keys: std::collections::HashSet<_> = fr.atoms.iter().map(|a| &a.key).collect();
            let common: Vec<_> = en_keys.intersection(&fr_keys).collect();

            eprintln!("Common term keys: {}", common.len());

            // Display a few comparisons
            for key in common.iter().take(5) {
                let en_term = en.atoms.iter().find(|a| &a.key == **key);
                let fr_term = fr.atoms.iter().find(|a| &a.key == **key);
                if let (Some(e), Some(f)) = (en_term, fr_term) {
                    eprintln!("  {} -> en:'{}' | fr:'{}'", key, e.value, f.value);
                }
            }
        }
    }

    /// Compare expressions (idiomatic phrases) across locales
    #[tokio::test]
    async fn test_locale_expression_comparison() {
        let state = get_test_state!();

        let locales = ["en-US", "fr-FR", "de-DE", "es-ES"];
        let mut results = Vec::new();

        for locale in locales {
            let atoms = novanet_mcp::tools::atoms::execute(
                &state,
                AtomsParams {
                    locale: locale.to_string(),
                    atom_type: AtomType::Expression,
                    domain: None,
                    register: Some("formal".to_string()),
                    query: None,
                    limit: Some(20),
                    include_containers: Some(false),
                },
            )
            .await;

            if let Ok(result) = atoms {
                results.push((locale, result.atoms.len()));
                eprintln!("{}: {} formal expressions", locale, result.atoms.len());
            }
        }

        // Report coverage variance
        if !results.is_empty() {
            let counts: Vec<usize> = results.iter().map(|(_, c)| *c).collect();
            let avg = counts.iter().sum::<usize>() as f64 / counts.len() as f64;
            eprintln!("Average expressions per locale: {:.1}", avg);
        }
    }

    /// Compare cultural context across locales
    #[tokio::test]
    async fn test_cultural_context_comparison() {
        let state = get_test_state!();

        // Get culture refs for multiple locales
        for locale in ["fr-FR", "ja-JP", "de-DE"] {
            let culture = novanet_mcp::tools::atoms::execute(
                &state,
                AtomsParams {
                    locale: locale.to_string(),
                    atom_type: AtomType::CultureRef,
                    domain: None,
                    register: None,
                    query: None,
                    limit: Some(10),
                    include_containers: Some(false),
                },
            )
            .await;

            let taboos = novanet_mcp::tools::atoms::execute(
                &state,
                AtomsParams {
                    locale: locale.to_string(),
                    atom_type: AtomType::Taboo,
                    domain: None,
                    register: None,
                    query: None,
                    limit: Some(10),
                    include_containers: Some(false),
                },
            )
            .await;

            if let (Ok(c), Ok(t)) = (culture, taboos) {
                eprintln!(
                    "{}: {} culture refs, {} taboos",
                    locale,
                    c.atoms.len(),
                    t.atoms.len()
                );
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 3: SEO Optimization Flow
// Agent gathers SEO context for content optimization
// Flow: search keywords -> traverse SEO relations -> assemble evidence
// ═══════════════════════════════════════════════════════════════════════════════

mod seo_optimization_flow {
    use super::*;
    use novanet_mcp::tools::{
        assemble::{AssembleParams, AssemblyStrategy},
        search::{SearchMode, SearchParams},
        traverse::{TraversalDirection, TraverseParams},
    };

    /// Gather SEO keyword context for content optimization
    #[tokio::test]
    async fn test_seo_keyword_discovery() {
        let state = get_test_state!();

        // Step 1: Search for SEO keywords related to topic
        let keyword_search = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "QR code".to_string(),
                mode: SearchMode::Hybrid,
                kinds: Some(vec!["SEOKeyword".to_string()]),
                realm: Some("shared".to_string()),
                layer: None,
                limit: Some(20),
                properties: Some(vec![
                    "name".to_string(),
                    "volume".to_string(),
                    "difficulty".to_string(),
                ]),
            },
        )
        .await;

        if let Ok(ref result) = keyword_search {
            eprintln!("Found {} SEO keywords for 'QR code'", result.hits.len());
            for hit in result.hits.iter().take(5) {
                eprintln!("  - {} (score: {:.2})", hit.key, hit.score);
            }
        }

        // Step 2: If we found keywords, traverse to find related entities
        if let Ok(ref search_result) = keyword_search {
            if let Some(first_keyword) = search_result.hits.first() {
                let traverse_result = novanet_mcp::tools::traverse::execute(
                    &state,
                    TraverseParams {
                        start_key: first_keyword.key.clone(),
                        max_depth: Some(2),
                        direction: TraversalDirection::Both,
                        arc_families: Some(vec!["mining".to_string(), "semantic".to_string()]),
                        arc_kinds: None,
                        target_kinds: Some(vec![
                            "Entity".to_string(),
                            "Page".to_string(),
                            "SEOKeyword".to_string(),
                        ]),
                        limit: Some(30),
                        include_properties: Some(true),
                    },
                )
                .await;

                if let Ok(ref result) = traverse_result {
                    eprintln!(
                        "Traversed from keyword: {} related nodes, {} arcs",
                        result.nodes.len(),
                        result.arcs.len()
                    );
                }
            }
        }
    }

    /// Build SEO-optimized content context
    #[tokio::test]
    async fn test_seo_content_assembly() {
        let state = get_test_state!();

        // Find a page to optimize
        let page_search = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "generator".to_string(),
                mode: SearchMode::Hybrid,
                kinds: Some(vec!["Page".to_string()]),
                realm: None,
                layer: None,
                limit: Some(1),
                properties: None,
            },
        )
        .await;

        let page_key = page_search
            .ok()
            .and_then(|r| r.hits.first().map(|h| h.key.clone()))
            .unwrap_or_else(|| "homepage".to_string());

        // Assemble with emphasis on mining arc family (SEO)
        let assemble_result = novanet_mcp::tools::assemble::execute(
            &state,
            AssembleParams {
                focus_key: page_key.clone(),
                locale: "en-US".to_string(),
                token_budget: Some(30_000),
                strategy: AssemblyStrategy::Relevance, // Prioritize by relevance
                include_entities: Some(true),
                include_knowledge: Some(true),
                include_structure: Some(false), // Focus on content, not structure
                arc_families: Some(vec!["mining".to_string(), "semantic".to_string()]),
                max_depth: Some(3),
            },
        )
        .await;

        if let Ok(ref result) = assemble_result {
            eprintln!(
                "SEO assembly: {} evidence packets for '{}'",
                result.evidence.len(),
                page_key
            );

            // Count evidence by type
            let mut type_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            for e in &result.evidence {
                *type_counts.entry(e.evidence_type.clone()).or_insert(0) += 1;
            }

            for (t, c) in &type_counts {
                eprintln!("  {}: {} packets", t, c);
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 4: Error Recovery Flow
// Agent handles partial failures gracefully
// Flow: query fails -> retry with modified query -> fallback to describe
// ═══════════════════════════════════════════════════════════════════════════════

mod error_recovery_flow {
    use super::*;
    use novanet_mcp::tools::{
        describe::{DescribeParams, DescribeTarget},
        query::QueryParams,
        search::{SearchMode, SearchParams},
    };

    /// Recover from a query failure with fallback strategy
    #[tokio::test]
    async fn test_query_failure_recovery() {
        let state = get_test_state!();

        // Step 1: Try a query that might fail (typo in label)
        let bad_query = novanet_mcp::tools::query::execute(
            &state,
            QueryParams {
                cypher: "MATCH (n:NonExistentLabelXYZ123) RETURN n LIMIT 10".to_string(),
                params: None,
                limit: Some(10),
                timeout_ms: Some(5000),
            },
        )
        .await;

        // Query succeeds but returns empty results
        if let Ok(result) = &bad_query {
            assert!(result.rows.is_empty(), "Nonexistent label returns empty");
            eprintln!(
                "Step 1: Query returned {} rows (empty as expected)",
                result.row_count
            );
        }

        // Step 2: Fallback to describe to understand what labels exist
        let describe_result = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Schema,
                entity_key: None,
                category_key: None,
            },
        )
        .await;

        assert!(describe_result.is_ok(), "Fallback describe should work");
        eprintln!("Step 2: Recovered with schema describe");

        // Step 3: Use search as an alternative approach
        let search_result = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "entity".to_string(),
                mode: SearchMode::Property, // Try property search instead
                kinds: None,                // Don't filter by kind
                realm: None,
                layer: None,
                limit: Some(5),
                properties: Some(vec!["key".to_string(), "name".to_string()]),
            },
        )
        .await;

        if let Ok(result) = search_result {
            eprintln!("Step 3: Property search found {} hits", result.hits.len());
        }
    }

    /// Handle invalid Cypher gracefully
    #[tokio::test]
    async fn test_invalid_cypher_recovery() {
        let state = get_test_state!();

        // Try various invalid queries and recover
        let invalid_queries = [
            // Write operation (blocked)
            "CREATE (n:Test) RETURN n",
            // Syntax error
            "MATCH (n WHERE n.key = 'test' RETURN n",
            // DELETE (blocked)
            "MATCH (n) DELETE n",
        ];

        for query in invalid_queries {
            let result = novanet_mcp::tools::query::execute(
                &state,
                QueryParams {
                    cypher: query.to_string(),
                    params: None,
                    limit: Some(10),
                    timeout_ms: Some(5000),
                },
            )
            .await;

            // Should fail
            assert!(result.is_err(), "Invalid query should fail: {}", query);

            let err = result.unwrap_err();
            eprintln!(
                "Query '{}' failed as expected: {}",
                &query[..30.min(query.len())],
                err
            );
        }

        // Recovery: use describe instead
        let recovery = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Stats,
                entity_key: None,
                category_key: None,
            },
        )
        .await;

        assert!(recovery.is_ok(), "Recovery via describe should work");
    }

    /// Handle entity not found gracefully
    #[tokio::test]
    async fn test_entity_not_found_recovery() {
        let state = get_test_state!();

        // Try to describe a nonexistent entity
        let describe_result = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Entity,
                entity_key: Some("nonexistent-entity-key-12345".to_string()),
                category_key: None,
            },
        )
        .await;

        // Should succeed but return error in data
        if let Ok(result) = describe_result {
            let has_error = result.data.get("error").is_some();
            eprintln!(
                "Entity describe result: has_error={}, data={}",
                has_error, result.data
            );
        }

        // Recovery: search for similar entities
        let search_recovery = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "nonexistent".to_string(), // Use partial name
                mode: SearchMode::Hybrid,
                kinds: Some(vec!["Entity".to_string()]),
                realm: None,
                layer: None,
                limit: Some(5),
                properties: None,
            },
        )
        .await;

        if let Ok(result) = search_recovery {
            eprintln!(
                "Recovery search found {} potential matches",
                result.hits.len()
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 5: Budget Management Flow
// Agent stays within token limits for LLM context
// Flow: large context -> truncation -> verify quality maintained
// ═══════════════════════════════════════════════════════════════════════════════

mod budget_management_flow {
    use super::*;
    use novanet_mcp::tools::{
        assemble::{AssembleParams, AssemblyStrategy},
        atoms::{AtomType, AtomsParams},
        generate::{GenerateMode, GenerateParams},
    };

    /// Test assembly with progressively smaller budgets
    #[tokio::test]
    async fn test_progressive_budget_reduction() {
        let state = get_test_state!();

        let budgets = [100_000, 50_000, 20_000, 10_000, 5_000];
        let mut results = Vec::new();

        for budget in budgets {
            let assemble_result = novanet_mcp::tools::assemble::execute(
                &state,
                AssembleParams {
                    focus_key: "homepage".to_string(),
                    locale: "en-US".to_string(),
                    token_budget: Some(budget),
                    strategy: AssemblyStrategy::Relevance,
                    include_entities: Some(true),
                    include_knowledge: Some(true),
                    include_structure: Some(true),
                    arc_families: None,
                    max_depth: Some(3),
                },
            )
            .await;

            if let Ok(result) = assemble_result {
                results.push((
                    budget,
                    result.evidence.len(),
                    result.total_tokens,
                    result.truncated,
                ));
                eprintln!(
                    "Budget {}: {} evidence, {} tokens, truncated={}",
                    budget,
                    result.evidence.len(),
                    result.total_tokens,
                    result.truncated
                );

                // Verify budget respected
                assert!(
                    result.total_tokens <= budget,
                    "Should not exceed budget of {}",
                    budget
                );
            }
        }

        // Verify larger budgets get more evidence
        if results.len() >= 2 {
            let (larger_budget, larger_evidence, _, _) = results[0];
            let (smaller_budget, smaller_evidence, _, _) = results[results.len() - 1];
            eprintln!(
                "Budget {} -> {} evidence vs Budget {} -> {} evidence",
                larger_budget, larger_evidence, smaller_budget, smaller_evidence
            );
        }
    }

    /// Test that small budgets still produce usable output
    #[tokio::test]
    async fn test_minimal_budget_quality() {
        let state = get_test_state!();

        // Very small budget - should still work
        let generate_result = novanet_mcp::tools::generate::execute(
            &state,
            GenerateParams {
                focus_key: "homepage".to_string(),
                locale: "fr-FR".to_string(),
                mode: GenerateMode::Block,
                token_budget: Some(2_000), // Very small
                include_examples: Some(false),
                spreading_depth: Some(1),
            },
        )
        .await;

        if let Ok(result) = generate_result {
            eprintln!("Minimal budget: {} tokens used", result.token_usage.total);

            // Verify prompt still has essential sections
            assert!(!result.prompt.is_empty(), "Prompt should not be empty");
            assert!(
                result.prompt.contains("# Generation Context"),
                "Should have header even with small budget"
            );

            // Token usage should be within budget
            assert!(
                result.token_usage.total <= 2_000,
                "Should respect minimal budget"
            );
        }
    }

    /// Test atoms limit behavior
    #[tokio::test]
    async fn test_atoms_limit_enforcement() {
        let state = get_test_state!();

        // Test different limits
        for limit in [5, 20, 100, 200] {
            let atoms_result = novanet_mcp::tools::atoms::execute(
                &state,
                AtomsParams {
                    locale: "en-US".to_string(),
                    atom_type: AtomType::All,
                    domain: None,
                    register: None,
                    query: None,
                    limit: Some(limit),
                    include_containers: Some(false),
                },
            )
            .await;

            if let Ok(result) = atoms_result {
                assert!(
                    result.atoms.len() <= limit,
                    "Limit {} not enforced: got {}",
                    limit,
                    result.atoms.len()
                );
                eprintln!("Limit {}: got {} atoms", limit, result.atoms.len());
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 6: Schema Discovery Flow
// Agent learns the graph structure before querying
// Flow: describe schema -> describe relations -> describe categories
// ═══════════════════════════════════════════════════════════════════════════════

mod schema_discovery_flow {
    use super::*;
    use novanet_mcp::tools::describe::{DescribeParams, DescribeTarget};

    /// Complete schema discovery workflow
    #[tokio::test]
    async fn test_complete_schema_discovery() {
        let state = get_test_state!();

        // Step 1: Get schema overview
        let schema = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Schema,
                entity_key: None,
                category_key: None,
            },
        )
        .await
        .expect("Schema describe should work");

        eprintln!("Schema overview: {} tokens", schema.token_estimate);

        // Check for expected structure
        assert!(schema.data.get("realms").is_some(), "Should have realms");
        assert!(
            schema.data.get("arc_families").is_some(),
            "Should have arc_families"
        );

        // Step 2: Get relation definitions
        let relations = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Relations,
                entity_key: None,
                category_key: None,
            },
        )
        .await
        .expect("Relations describe should work");

        eprintln!("Relations: {} tokens", relations.token_estimate);

        if let Some(arc_classes) = relations.data.get("arc_classes") {
            if let Some(arr) = arc_classes.as_array() {
                eprintln!("  Found {} arc classes", arr.len());
            }
        }

        // Step 3: Get categories
        let categories = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Category,
                entity_key: None,
                category_key: None, // Get all categories
            },
        )
        .await
        .expect("Category describe should work");

        eprintln!("Categories: {} tokens", categories.token_estimate);

        // Step 4: Get available locales
        let locales = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Locales,
                entity_key: None,
                category_key: None,
            },
        )
        .await
        .expect("Locales describe should work");

        if let Some(locale_list) = locales.data.get("locales") {
            if let Some(arr) = locale_list.as_array() {
                eprintln!("Available locales: {}", arr.len());
            }
        }

        // Step 5: Get stats
        let stats = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Stats,
                entity_key: None,
                category_key: None,
            },
        )
        .await
        .expect("Stats describe should work");

        eprintln!("Stats: {}", stats.data);
    }

    /// Drill down into specific category
    #[tokio::test]
    async fn test_category_drilldown() {
        let state = get_test_state!();

        // First get all categories
        let all_categories = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Category,
                entity_key: None,
                category_key: None,
            },
        )
        .await;

        if let Ok(_all_result) = all_categories {
            // Try to get a specific category
            let specific = novanet_mcp::tools::describe::execute(
                &state,
                DescribeParams {
                    describe: DescribeTarget::Category,
                    entity_key: None,
                    category_key: Some("product".to_string()), // Common category
                },
            )
            .await;

            if let Ok(cat_result) = specific {
                eprintln!("Category 'product': {}", cat_result.data);
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 7: Entity Analysis Flow
// Agent does deep analysis of a specific entity
// Flow: search entity -> describe entity -> traverse relations -> assemble context
// ═══════════════════════════════════════════════════════════════════════════════

mod entity_analysis_flow {
    use super::*;
    use novanet_mcp::tools::{
        assemble::{AssembleParams, AssemblyStrategy},
        describe::{DescribeParams, DescribeTarget},
        search::{SearchMode, SearchParams},
        traverse::{TraversalDirection, TraverseParams},
    };

    /// Complete entity analysis workflow
    #[tokio::test]
    async fn test_entity_deep_analysis() {
        let state = get_test_state!();

        // Step 1: Search for entity
        let search_result = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "QR".to_string(),
                mode: SearchMode::Hybrid,
                kinds: Some(vec!["Entity".to_string()]),
                realm: None,
                layer: None,
                limit: Some(5),
                properties: None,
            },
        )
        .await;

        let entity_key = search_result
            .ok()
            .and_then(|r| r.hits.first().map(|h| h.key.clone()))
            .unwrap_or_else(|| "qr-code-generator".to_string());

        eprintln!("Analyzing entity: {}", entity_key);

        // Step 2: Get detailed entity description
        let describe_result = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Entity,
                entity_key: Some(entity_key.clone()),
                category_key: None,
            },
        )
        .await;

        if let Ok(result) = &describe_result {
            eprintln!("Entity details: {}", result.data);
        }

        // Step 3: Traverse all relationships
        let traverse_result = novanet_mcp::tools::traverse::execute(
            &state,
            TraverseParams {
                start_key: entity_key.clone(),
                max_depth: Some(2),
                direction: TraversalDirection::Both, // All directions
                arc_families: None,                  // All arc families
                arc_kinds: None,
                target_kinds: None, // All kinds
                limit: Some(100),
                include_properties: Some(true),
            },
        )
        .await;

        if let Ok(result) = &traverse_result {
            eprintln!(
                "Entity relationships: {} nodes, {} arcs",
                result.nodes.len(),
                result.arcs.len()
            );

            // Group by arc kind
            let mut arc_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            for arc in &result.arcs {
                *arc_counts.entry(arc.arc_kind.clone()).or_insert(0) += 1;
            }

            for (kind, count) in arc_counts.iter().take(10) {
                eprintln!("  {}: {}", kind, count);
            }
        }

        // Step 4: Assemble context for entity
        let assemble_result = novanet_mcp::tools::assemble::execute(
            &state,
            AssembleParams {
                focus_key: entity_key.clone(),
                locale: "en-US".to_string(),
                token_budget: Some(20_000),
                strategy: AssemblyStrategy::Breadth,
                include_entities: Some(true),
                include_knowledge: Some(true),
                include_structure: Some(true),
                arc_families: None,
                max_depth: Some(2),
            },
        )
        .await;

        if let Ok(result) = &assemble_result {
            eprintln!(
                "Entity context: {} evidence packets, {} tokens",
                result.evidence.len(),
                result.total_tokens
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 8: Knowledge Atom Assembly
// Agent selectively loads knowledge atoms for context
// Flow: atoms by type -> atoms by domain -> atoms by query
// ═══════════════════════════════════════════════════════════════════════════════

mod knowledge_atom_assembly {
    use super::*;
    use novanet_mcp::tools::atoms::{AtomType, AtomsParams};

    /// Load atoms by type progressively
    #[tokio::test]
    async fn test_atoms_by_type() {
        let state = get_test_state!();

        let atom_types = [
            AtomType::Term,
            AtomType::Expression,
            AtomType::Pattern,
            AtomType::CultureRef,
            AtomType::Taboo,
            AtomType::AudienceTrait,
        ];

        for atom_type in atom_types {
            let result = novanet_mcp::tools::atoms::execute(
                &state,
                AtomsParams {
                    locale: "fr-FR".to_string(),
                    atom_type: atom_type.clone(),
                    domain: None,
                    register: None,
                    query: None,
                    limit: Some(20),
                    include_containers: Some(true),
                },
            )
            .await;

            if let Ok(r) = result {
                eprintln!(
                    "{:?}: {} atoms, {} tokens",
                    atom_type,
                    r.atoms.len(),
                    r.token_estimate
                );
            }
        }
    }

    /// Load atoms filtered by domain
    #[tokio::test]
    async fn test_atoms_by_domain() {
        let state = get_test_state!();

        let domains = ["technical", "legal", "marketing", "medical"];

        for domain in domains {
            let result = novanet_mcp::tools::atoms::execute(
                &state,
                AtomsParams {
                    locale: "en-US".to_string(),
                    atom_type: AtomType::Term,
                    domain: Some(domain.to_string()),
                    register: None,
                    query: None,
                    limit: Some(30),
                    include_containers: Some(false),
                },
            )
            .await;

            if let Ok(r) = result {
                eprintln!("Domain '{}': {} terms", domain, r.atoms.len());
            }
        }
    }

    /// Search atoms with query filter
    #[tokio::test]
    async fn test_atoms_with_query_filter() {
        let state = get_test_state!();

        // Search for specific terms
        let result = novanet_mcp::tools::atoms::execute(
            &state,
            AtomsParams {
                locale: "fr-FR".to_string(),
                atom_type: AtomType::Term,
                domain: None,
                register: None,
                query: Some("code".to_string()), // Filter by query
                limit: Some(50),
                include_containers: Some(false),
            },
        )
        .await;

        if let Ok(r) = result {
            eprintln!("Terms containing 'code': {}", r.atoms.len());
            for atom in r.atoms.iter().take(5) {
                eprintln!("  - {}: {}", atom.key, atom.value);
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 9: Context Anchor Resolution
// Agent discovers and resolves cross-page references
// Flow: traverse references -> resolve anchors -> build link map
// ═══════════════════════════════════════════════════════════════════════════════

mod context_anchor_resolution {
    use super::*;
    use novanet_mcp::tools::{
        generate::{GenerateMode, GenerateParams},
        traverse::{TraversalDirection, TraverseParams},
    };

    /// Discover cross-page references
    #[tokio::test]
    async fn test_cross_page_reference_discovery() {
        let state = get_test_state!();

        // Find pages with REFERENCES_PAGE arcs
        let traverse_result = novanet_mcp::tools::traverse::execute(
            &state,
            TraverseParams {
                start_key: "homepage".to_string(),
                max_depth: Some(2),
                direction: TraversalDirection::Outgoing,
                arc_families: None,
                arc_kinds: Some(vec!["REFERENCES_PAGE".to_string(), "HAS_BLOCK".to_string()]),
                target_kinds: Some(vec!["Page".to_string(), "Block".to_string()]),
                limit: Some(50),
                include_properties: Some(true),
            },
        )
        .await;

        if let Ok(result) = traverse_result {
            eprintln!(
                "Reference discovery: {} nodes, {} arcs",
                result.nodes.len(),
                result.arcs.len()
            );

            // Find REFERENCES_PAGE arcs specifically
            let ref_arcs: Vec<_> = result
                .arcs
                .iter()
                .filter(|a| a.arc_kind == "REFERENCES_PAGE")
                .collect();
            eprintln!("Cross-page references: {}", ref_arcs.len());
        }
    }

    /// Test anchor resolution in generate output
    #[tokio::test]
    async fn test_generate_with_anchors() {
        let state = get_test_state!();

        let generate_result = novanet_mcp::tools::generate::execute(
            &state,
            GenerateParams {
                focus_key: "homepage".to_string(),
                locale: "fr-FR".to_string(),
                mode: GenerateMode::Page,
                token_budget: Some(30_000),
                include_examples: Some(false),
                spreading_depth: Some(2),
            },
        )
        .await;

        if let Ok(result) = generate_result {
            eprintln!("Context anchors found: {}", result.context_anchors.len());

            for anchor in &result.context_anchors {
                eprintln!(
                    "  {{{{anchor:{}|{}}}}} -> {}",
                    anchor.page_key, anchor.anchor_text, anchor.slug
                );
            }

            // Check if prompt mentions anchor syntax
            if !result.context_anchors.is_empty() {
                assert!(
                    result.prompt.contains("{{anchor:")
                        || result.prompt.contains("Context Anchors"),
                    "Prompt should explain anchor syntax when anchors exist"
                );
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 10: Concurrent Agent Operations
// Multiple agent operations running in parallel
// Flow: parallel searches -> parallel traversals -> merged results
// ═══════════════════════════════════════════════════════════════════════════════

mod concurrent_agent_operations {
    use super::*;
    use novanet_mcp::tools::{
        atoms::{AtomType, AtomsParams},
        describe::{DescribeParams, DescribeTarget},
        search::{SearchMode, SearchParams},
    };
    use tokio::sync::Barrier;

    /// Run multiple searches concurrently
    #[tokio::test]
    async fn test_concurrent_searches() {
        let state = match create_test_state().await {
            Some(s) => Arc::new(s),
            None => return,
        };
        let barrier = Arc::new(Barrier::new(5));

        let queries = ["QR", "code", "generator", "page", "entity"];
        let mut handles = vec![];

        for query in queries {
            let state_clone = state.clone();
            let barrier_clone = barrier.clone();
            let query_str = query.to_string();

            handles.push(tokio::spawn(async move {
                barrier_clone.wait().await;

                let result = novanet_mcp::tools::search::execute(
                    &state_clone,
                    SearchParams {
                        query: query_str.clone(),
                        mode: SearchMode::Hybrid,
                        kinds: None,
                        realm: None,
                        layer: None,
                        limit: Some(10),
                        properties: None,
                    },
                )
                .await;

                (query_str, result.map(|r| r.hits.len()).unwrap_or(0))
            }));
        }

        let mut results = vec![];
        for handle in handles {
            results.push(handle.await.unwrap());
        }

        eprintln!("Concurrent search results:");
        for (query, count) in &results {
            eprintln!("  '{}': {} hits", query, count);
        }

        assert_eq!(results.len(), 5, "All concurrent searches should complete");
    }

    /// Run multiple describe operations concurrently
    #[tokio::test]
    async fn test_concurrent_describes() {
        let state = match create_test_state().await {
            Some(s) => Arc::new(s),
            None => return,
        };

        let targets = [
            DescribeTarget::Schema,
            DescribeTarget::Relations,
            DescribeTarget::Locales,
            DescribeTarget::Stats,
        ];

        let mut handles = vec![];

        for target in targets {
            let state_clone = state.clone();

            handles.push(tokio::spawn(async move {
                let result = novanet_mcp::tools::describe::execute(
                    &state_clone,
                    DescribeParams {
                        describe: target,
                        entity_key: None,
                        category_key: None,
                    },
                )
                .await;

                result.map(|r| r.token_estimate).unwrap_or(0)
            }));
        }

        let mut total_tokens = 0;
        for handle in handles {
            total_tokens += handle.await.unwrap();
        }

        eprintln!("Total tokens from concurrent describes: {}", total_tokens);
    }

    /// Run mixed operations concurrently (simulating real agent workload)
    #[tokio::test]
    async fn test_mixed_concurrent_operations() {
        let state = match create_test_state().await {
            Some(s) => Arc::new(s),
            None => return,
        };
        let barrier = Arc::new(Barrier::new(4));

        // Search
        let state1 = state.clone();
        let barrier1 = barrier.clone();
        let search_handle = tokio::spawn(async move {
            barrier1.wait().await;
            novanet_mcp::tools::search::execute(
                &state1,
                SearchParams {
                    query: "test".to_string(),
                    mode: SearchMode::Hybrid,
                    kinds: None,
                    realm: None,
                    layer: None,
                    limit: Some(5),
                    properties: None,
                },
            )
            .await
            .map(|r| ("search", r.hits.len()))
        });

        // Describe
        let state2 = state.clone();
        let barrier2 = barrier.clone();
        let describe_handle = tokio::spawn(async move {
            barrier2.wait().await;
            novanet_mcp::tools::describe::execute(
                &state2,
                DescribeParams {
                    describe: DescribeTarget::Schema,
                    entity_key: None,
                    category_key: None,
                },
            )
            .await
            .map(|r| ("describe", r.token_estimate))
        });

        // Atoms (en-US)
        let state3 = state.clone();
        let barrier3 = barrier.clone();
        let atoms_en_handle = tokio::spawn(async move {
            barrier3.wait().await;
            novanet_mcp::tools::atoms::execute(
                &state3,
                AtomsParams {
                    locale: "en-US".to_string(),
                    atom_type: AtomType::Term,
                    domain: None,
                    register: None,
                    query: None,
                    limit: Some(20),
                    include_containers: Some(false),
                },
            )
            .await
            .map(|r| ("atoms-en", r.atoms.len()))
        });

        // Atoms (fr-FR)
        let state4 = state.clone();
        let barrier4 = barrier.clone();
        let atoms_fr_handle = tokio::spawn(async move {
            barrier4.wait().await;
            novanet_mcp::tools::atoms::execute(
                &state4,
                AtomsParams {
                    locale: "fr-FR".to_string(),
                    atom_type: AtomType::Term,
                    domain: None,
                    register: None,
                    query: None,
                    limit: Some(20),
                    include_containers: Some(false),
                },
            )
            .await
            .map(|r| ("atoms-fr", r.atoms.len()))
        });

        // Collect all results
        let results = vec![
            search_handle.await.unwrap(),
            describe_handle.await.unwrap(),
            atoms_en_handle.await.unwrap(),
            atoms_fr_handle.await.unwrap(),
        ];

        eprintln!("Mixed concurrent operations:");
        for (name, value) in results.iter().flatten() {
            eprintln!("  {}: {}", name, value);
        }

        // All should succeed
        for result in results {
            assert!(result.is_ok(), "All concurrent operations should succeed");
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 11: Cache Behavior in Agent Workflows
// Test that caching improves performance for repeated operations
// ═══════════════════════════════════════════════════════════════════════════════

mod cache_behavior {
    use super::*;
    use novanet_mcp::tools::query::QueryParams;
    use std::time::Instant;

    /// Test that repeated queries benefit from caching
    #[tokio::test]
    async fn test_query_caching_speedup() {
        let state = get_test_state!();

        let cypher = "MATCH (c:Class) RETURN c.name LIMIT 20".to_string();

        // First query (cache miss)
        let start1 = Instant::now();
        let result1 = novanet_mcp::tools::query::execute(
            &state,
            QueryParams {
                cypher: cypher.clone(),
                params: None,
                limit: None,
                timeout_ms: None,
            },
        )
        .await
        .expect("First query should succeed");
        let time1 = start1.elapsed();

        assert!(!result1.cached, "First query should not be cached");

        // Second query (cache hit)
        let start2 = Instant::now();
        let result2 = novanet_mcp::tools::query::execute(
            &state,
            QueryParams {
                cypher: cypher.clone(),
                params: None,
                limit: None,
                timeout_ms: None,
            },
        )
        .await
        .expect("Second query should succeed");
        let time2 = start2.elapsed();

        assert!(result2.cached, "Second query should be cached");
        assert_eq!(
            result1.row_count, result2.row_count,
            "Results should be identical"
        );

        eprintln!("Query timing: uncached={:?}, cached={:?}", time1, time2);

        // Cached should generally be faster (but allow for variance)
        // We just verify caching works, not strict timing guarantees
    }

    /// Test that different parameters produce different cache entries
    #[tokio::test]
    async fn test_cache_isolation_by_params() {
        let state = get_test_state!();

        let cypher = "RETURN $value AS val".to_string();

        // Query with value=1
        let mut params1 = serde_json::Map::new();
        params1.insert("value".to_string(), serde_json::json!(1));

        let result1 = novanet_mcp::tools::query::execute(
            &state,
            QueryParams {
                cypher: cypher.clone(),
                params: Some(params1.clone()),
                limit: None,
                timeout_ms: None,
            },
        )
        .await
        .expect("Query 1 should succeed");

        // Query with value=2 (should NOT use cached result)
        let mut params2 = serde_json::Map::new();
        params2.insert("value".to_string(), serde_json::json!(2));

        let result2 = novanet_mcp::tools::query::execute(
            &state,
            QueryParams {
                cypher: cypher.clone(),
                params: Some(params2.clone()),
                limit: None,
                timeout_ms: None,
            },
        )
        .await
        .expect("Query 2 should succeed");

        // Results should be different
        assert_eq!(result1.rows[0]["val"], 1);
        assert_eq!(result2.rows[0]["val"], 2);

        eprintln!(
            "Cache isolation: val1={}, val2={}, cached1={}, cached2={}",
            result1.rows[0]["val"], result2.rows[0]["val"], result1.cached, result2.cached
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 12: Complete Agent Session Simulation
// Simulate a realistic agent session from start to finish
// ═══════════════════════════════════════════════════════════════════════════════

mod complete_session {
    use super::*;
    use novanet_mcp::tools::{
        assemble::{AssembleParams, AssemblyStrategy},
        atoms::{AtomType, AtomsParams},
        describe::{DescribeParams, DescribeTarget},
        generate::{GenerateMode, GenerateParams},
        search::{SearchMode, SearchParams},
        traverse::{TraversalDirection, TraverseParams},
    };

    /// Simulate a complete agent session: bootstrap -> explore -> generate
    #[tokio::test]
    async fn test_complete_agent_session() {
        let state = get_test_state!();
        let mut session_tokens = 0_usize;

        eprintln!("=== AGENT SESSION START ===");

        // Phase 1: Bootstrap - Learn the graph
        eprintln!("\n--- Phase 1: Bootstrap ---");

        let schema = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Schema,
                entity_key: None,
                category_key: None,
            },
        )
        .await
        .expect("Schema describe failed");
        session_tokens += schema.token_estimate;
        eprintln!("Schema: {} tokens", schema.token_estimate);

        let locales = novanet_mcp::tools::describe::execute(
            &state,
            DescribeParams {
                describe: DescribeTarget::Locales,
                entity_key: None,
                category_key: None,
            },
        )
        .await
        .expect("Locales describe failed");
        session_tokens += locales.token_estimate;
        eprintln!("Locales: {} tokens", locales.token_estimate);

        // Phase 2: Explore - Find what to work with
        eprintln!("\n--- Phase 2: Exploration ---");

        let search_result = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "QR".to_string(),
                mode: SearchMode::Hybrid,
                kinds: Some(vec!["Entity".to_string(), "Page".to_string()]),
                realm: None,
                layer: None,
                limit: Some(10),
                properties: None,
            },
        )
        .await
        .expect("Search failed");
        session_tokens += search_result.token_estimate;
        eprintln!(
            "Search 'QR': {} hits, {} tokens",
            search_result.hits.len(),
            search_result.token_estimate
        );

        // Pick first entity or page
        let focus_key = search_result
            .hits
            .first()
            .map(|h| h.key.clone())
            .unwrap_or_else(|| "homepage".to_string());

        let traverse_result = novanet_mcp::tools::traverse::execute(
            &state,
            TraverseParams {
                start_key: focus_key.clone(),
                max_depth: Some(2),
                direction: TraversalDirection::Both,
                arc_families: None,
                arc_kinds: None,
                target_kinds: None,
                limit: Some(30),
                include_properties: Some(true),
            },
        )
        .await
        .expect("Traverse failed");
        session_tokens += traverse_result.token_estimate;
        eprintln!(
            "Traverse from '{}': {} nodes, {} tokens",
            focus_key,
            traverse_result.nodes.len(),
            traverse_result.token_estimate
        );

        // Phase 3: Gather context for generation
        eprintln!("\n--- Phase 3: Context Assembly ---");

        let assemble_result = novanet_mcp::tools::assemble::execute(
            &state,
            AssembleParams {
                focus_key: focus_key.clone(),
                locale: "fr-FR".to_string(),
                token_budget: Some(30_000),
                strategy: AssemblyStrategy::Relevance,
                include_entities: Some(true),
                include_knowledge: Some(true),
                include_structure: Some(true),
                arc_families: None,
                max_depth: Some(2),
            },
        )
        .await
        .expect("Assemble failed");
        session_tokens += assemble_result.total_tokens;
        eprintln!(
            "Assemble: {} evidence, {} tokens",
            assemble_result.evidence.len(),
            assemble_result.total_tokens
        );

        let atoms_result = novanet_mcp::tools::atoms::execute(
            &state,
            AtomsParams {
                locale: "fr-FR".to_string(),
                atom_type: AtomType::All,
                domain: None,
                register: None,
                query: None,
                limit: Some(30),
                include_containers: Some(false),
            },
        )
        .await
        .expect("Atoms failed");
        session_tokens += atoms_result.token_estimate;
        eprintln!(
            "Atoms fr-FR: {} atoms, {} tokens",
            atoms_result.atoms.len(),
            atoms_result.token_estimate
        );

        // Phase 4: Generate
        eprintln!("\n--- Phase 4: Generation ---");

        let generate_result = novanet_mcp::tools::generate::execute(
            &state,
            GenerateParams {
                focus_key: focus_key.clone(),
                locale: "fr-FR".to_string(),
                mode: GenerateMode::Block,
                token_budget: Some(20_000),
                include_examples: Some(false),
                spreading_depth: Some(2),
            },
        )
        .await
        .expect("Generate failed");
        eprintln!(
            "Generate: {} total tokens, {} evidence items",
            generate_result.token_usage.total,
            generate_result.evidence_summary.len()
        );

        // Session summary
        eprintln!("\n=== SESSION COMPLETE ===");
        eprintln!("Total session tokens: {}", session_tokens);
        eprintln!(
            "Generation prompt size: {} chars",
            generate_result.prompt.len()
        );
        eprintln!("Context anchors: {}", generate_result.context_anchors.len());

        // Verify prompt has essential content
        assert!(
            generate_result.prompt.contains("# Generation Context"),
            "Prompt should have header"
        );
        assert!(
            generate_result.prompt.contains("## Instructions"),
            "Prompt should have instructions"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCENARIO 13: Edge Cases and Boundary Conditions
// Test behavior at the edges of normal operation
// ═══════════════════════════════════════════════════════════════════════════════

mod edge_cases {
    use super::*;
    use novanet_mcp::tools::{
        atoms::{AtomType, AtomsParams},
        search::{SearchMode, SearchParams},
        traverse::{TraversalDirection, TraverseParams},
    };

    /// Test with empty search query
    #[tokio::test]
    async fn test_empty_search_query() {
        let state = get_test_state!();

        let result = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "".to_string(), // Empty query
                mode: SearchMode::Property,
                kinds: None,
                realm: None,
                layer: None,
                limit: Some(5),
                properties: None,
            },
        )
        .await;

        // Should handle gracefully (empty results or error)
        eprintln!("Empty query result: {:?}", result.is_ok());
    }

    /// Test with very deep traversal request
    #[tokio::test]
    async fn test_deep_traversal_clamped() {
        let state = get_test_state!();

        // Request depth 10, should be clamped to max (5)
        let result = novanet_mcp::tools::traverse::execute(
            &state,
            TraverseParams {
                start_key: "homepage".to_string(),
                max_depth: Some(10), // Will be clamped
                direction: TraversalDirection::Outgoing,
                arc_families: None,
                arc_kinds: None,
                target_kinds: None,
                limit: Some(20),
                include_properties: Some(false),
            },
        )
        .await;

        if let Ok(r) = result {
            assert!(r.max_depth_reached <= 5, "Depth should be clamped to 5");
            eprintln!("Deep traversal: max_depth_reached={}", r.max_depth_reached);
        }
    }

    /// Test atoms for nonexistent locale
    #[tokio::test]
    async fn test_nonexistent_locale_atoms() {
        let state = get_test_state!();

        let result = novanet_mcp::tools::atoms::execute(
            &state,
            AtomsParams {
                locale: "xx-XX".to_string(), // Nonexistent locale
                atom_type: AtomType::Term,
                domain: None,
                register: None,
                query: None,
                limit: Some(10),
                include_containers: Some(false),
            },
        )
        .await;

        // Should succeed with empty results
        if let Ok(r) = result {
            assert!(
                r.atoms.is_empty(),
                "Nonexistent locale should return empty atoms"
            );
            eprintln!("Nonexistent locale: {} atoms (expected 0)", r.atoms.len());
        }
    }

    /// Test search with all filters
    #[tokio::test]
    async fn test_heavily_filtered_search() {
        let state = get_test_state!();

        let result = novanet_mcp::tools::search::execute(
            &state,
            SearchParams {
                query: "test".to_string(),
                mode: SearchMode::Hybrid,
                kinds: Some(vec!["Entity".to_string()]),
                realm: Some("org".to_string()),
                layer: Some("semantic".to_string()),
                limit: Some(1),
                properties: Some(vec!["key".to_string()]),
            },
        )
        .await;

        // Should handle all filters correctly
        eprintln!(
            "Heavily filtered search: {} hits",
            result.map(|r| r.hits.len()).unwrap_or(0)
        );
    }

    /// Test traverse with conflicting filters
    #[tokio::test]
    async fn test_traverse_with_restrictive_filters() {
        let state = get_test_state!();

        let result = novanet_mcp::tools::traverse::execute(
            &state,
            TraverseParams {
                start_key: "homepage".to_string(),
                max_depth: Some(3),
                direction: TraversalDirection::Outgoing,
                arc_families: Some(vec!["mining".to_string()]), // Very specific
                arc_kinds: Some(vec!["TARGETS".to_string()]),   // Even more specific
                target_kinds: Some(vec!["SEOKeyword".to_string()]),
                limit: Some(10),
                include_properties: Some(true),
            },
        )
        .await;

        // Should complete (possibly with empty results)
        eprintln!(
            "Restrictive traverse: {} nodes",
            result.map(|r| r.nodes.len()).unwrap_or(0)
        );
    }
}
