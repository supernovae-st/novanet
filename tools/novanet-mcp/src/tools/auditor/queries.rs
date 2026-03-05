//! Audit query implementations for novanet_audit
//!
//! Provides audit queries for coverage, orphans, integrity, and freshness checks.

use super::types::{AuditIssue, AuditParams, AuditResult, AuditScope, AuditTarget, OntologyInsights};
use crate::error::Result;
use crate::metrics::ConstraintSatisfactionRate;
use crate::server::State;
use std::time::Instant;

/// Execute novanet_audit - main entry point
pub async fn execute(state: &State, params: AuditParams) -> Result<AuditResult> {
    let start = Instant::now();
    let limit = params.limit.unwrap_or(100);

    let (issues, nodes_checked, arcs_checked) = match &params.target {
        AuditTarget::Coverage => {
            let issues = audit_coverage(state, &params.scope, limit).await?;
            let nodes = count_entities(state).await.unwrap_or(0);
            (issues, nodes, 0)
        }
        AuditTarget::Orphans => {
            let issues = audit_orphans(state, &params.scope, limit).await?;
            let nodes = count_native_nodes(state).await.unwrap_or(0);
            (issues, nodes, 0)
        }
        AuditTarget::Integrity => {
            let issues = audit_integrity(state, &params.scope, limit).await?;
            let arcs = count_arcs(state).await.unwrap_or(0);
            (issues, 0, arcs)
        }
        AuditTarget::Freshness => {
            let issues = audit_freshness(state, &params.scope, limit).await?;
            let nodes = count_generated_nodes(state).await.unwrap_or(0);
            (issues, nodes, 0)
        }
        AuditTarget::All => {
            let mut all_issues = Vec::new();

            // Run all audits with proportional limits
            let per_audit_limit = limit / 4;
            all_issues.extend(audit_coverage(state, &params.scope, per_audit_limit).await?);
            all_issues.extend(audit_orphans(state, &params.scope, per_audit_limit).await?);
            all_issues.extend(audit_integrity(state, &params.scope, per_audit_limit).await?);
            all_issues.extend(audit_freshness(state, &params.scope, per_audit_limit).await?);

            let nodes = count_all_nodes(state).await.unwrap_or(0);
            let arcs = count_arcs(state).await.unwrap_or(0);

            (all_issues, nodes, arcs)
        }
    };

    // Calculate CSR
    let total_checked = nodes_checked.saturating_add(arcs_checked);
    let violated = issues.len() as u32;
    let satisfied = total_checked.saturating_sub(violated);
    let csr = ConstraintSatisfactionRate::new(satisfied, violated);

    // Build recommendations based on issues
    let recommendations = generate_recommendations(&issues);

    // Build insights
    let insights = build_insights(&issues, &csr);

    let result = AuditResult::new(params.target.to_string())
        .with_issues(issues)
        .with_csr(csr)
        .with_nodes_checked(nodes_checked)
        .with_arcs_checked(arcs_checked)
        .with_insights(insights)
        .with_recommendations(recommendations)
        .with_execution_time(start.elapsed().as_millis() as u64);

    Ok(result)
}

/// Audit coverage - find Entities without EntityNative for target locales
pub async fn audit_coverage(
    state: &State,
    scope: &Option<AuditScope>,
    limit: u32,
) -> Result<Vec<AuditIssue>> {
    let mut issues = Vec::new();

    // Build locale filter
    let locale_clause = scope
        .as_ref()
        .and_then(|s| s.locale.as_ref())
        .map(|l| format!("AND l.key = '{}'", sanitize_string(l)))
        .unwrap_or_default();

    // Query: Find Entities that should have EntityNative but don't
    let cypher = format!(
        r#"
        MATCH (e:Entity)
        WHERE NOT EXISTS {{
            MATCH (e)-[:HAS_NATIVE]->(:EntityNative)
        }}
        {}
        RETURN e.key AS entity_key
        LIMIT $limit
        "#,
        locale_clause
    );

    let mut params = serde_json::Map::new();
    params.insert("limit".to_string(), serde_json::json!(limit));

    let rows = state.pool().execute_query(&cypher, Some(params)).await?;

    for row in rows {
        if let Some(entity_key) = row["entity_key"].as_str() {
            issues.push(
                AuditIssue::warning("coverage", format!("Entity '{}' has no EntityNative", entity_key))
                    .with_node_key(entity_key)
                    .with_arc_class("HAS_NATIVE"),
            );
        }
    }

    Ok(issues)
}

/// Audit orphans - find *Native nodes without required FOR_LOCALE arc
pub async fn audit_orphans(
    state: &State,
    scope: &Option<AuditScope>,
    limit: u32,
) -> Result<Vec<AuditIssue>> {
    let mut issues = Vec::new();

    // Build class filter
    let class_filter = scope
        .as_ref()
        .and_then(|s| s.class.as_ref())
        .map(|c| format!("AND labels(n)[0] = '{}'", sanitize_string(c)))
        .unwrap_or_default();

    // Query: Find *Native nodes without FOR_LOCALE arc
    let cypher = format!(
        r#"
        MATCH (n)
        WHERE n.key CONTAINS '@'
        AND NOT EXISTS {{
            MATCH (n)-[:FOR_LOCALE]->(:Locale)
        }}
        {}
        RETURN n.key AS node_key, labels(n)[0] AS node_class
        LIMIT $limit
        "#,
        class_filter
    );

    let mut params = serde_json::Map::new();
    params.insert("limit".to_string(), serde_json::json!(limit));

    let rows = state.pool().execute_query(&cypher, Some(params.clone())).await?;

    for row in rows {
        if let (Some(node_key), Some(node_class)) =
            (row["node_key"].as_str(), row["node_class"].as_str())
        {
            issues.push(
                AuditIssue::critical(
                    "orphan",
                    format!("{}:'{}' missing FOR_LOCALE arc", node_class, node_key),
                )
                .with_node_key(node_key)
                .with_arc_class("FOR_LOCALE"),
            );
        }
    }

    // Also check for *Native without parent HAS_NATIVE arc
    let cypher2 = r#"
        MATCH (n)
        WHERE (n:EntityNative OR n:PageNative OR n:BlockNative)
        AND NOT EXISTS {
            MATCH ()-[:HAS_NATIVE]->(n)
        }
        RETURN n.key AS node_key, labels(n)[0] AS node_class
        LIMIT $limit
    "#;

    let rows2 = state.pool().execute_query(cypher2, Some(params.clone())).await?;

    for row in rows2 {
        if let (Some(node_key), Some(node_class)) =
            (row["node_key"].as_str(), row["node_class"].as_str())
        {
            issues.push(
                AuditIssue::critical(
                    "orphan",
                    format!("{}:'{}' not connected via HAS_NATIVE", node_class, node_key),
                )
                .with_node_key(node_key)
                .with_arc_class("HAS_NATIVE"),
            );
        }
    }

    Ok(issues)
}

/// Audit integrity - find dangling arcs or broken references
pub async fn audit_integrity(
    state: &State,
    _scope: &Option<AuditScope>,
    limit: u32,
) -> Result<Vec<AuditIssue>> {
    let mut issues = Vec::new();

    // Query: Find nodes with key containing @ but no matching base entity
    let cypher = r#"
        MATCH (n:EntityNative)
        WHERE n.key CONTAINS '@'
        WITH n, split(n.key, '@')[0] AS base_key
        WHERE NOT EXISTS {
            MATCH (e:Entity {key: base_key})
        }
        RETURN n.key AS native_key, base_key
        LIMIT $limit
    "#;

    let mut params = serde_json::Map::new();
    params.insert("limit".to_string(), serde_json::json!(limit));

    let rows = state.pool().execute_query(cypher, Some(params.clone())).await?;

    for row in rows {
        if let (Some(native_key), Some(base_key)) =
            (row["native_key"].as_str(), row["base_key"].as_str())
        {
            issues.push(
                AuditIssue::critical(
                    "integrity",
                    format!(
                        "EntityNative '{}' references non-existent Entity '{}'",
                        native_key, base_key
                    ),
                )
                .with_node_key(native_key)
                .with_details(serde_json::json!({
                    "expected_entity": base_key
                })),
            );
        }
    }

    // Query: Find FOR_LOCALE arcs pointing to non-existent Locale
    let cypher2 = r#"
        MATCH (n)-[r:FOR_LOCALE]->(l)
        WHERE NOT l:Locale
        RETURN n.key AS node_key, type(r) AS arc_type
        LIMIT $limit
    "#;

    let rows2 = state.pool().execute_query(cypher2, Some(params)).await?;

    for row in rows2 {
        if let Some(node_key) = row["node_key"].as_str() {
            issues.push(
                AuditIssue::critical(
                    "integrity",
                    format!("Node '{}' has FOR_LOCALE to non-Locale node", node_key),
                )
                .with_node_key(node_key)
                .with_arc_class("FOR_LOCALE"),
            );
        }
    }

    Ok(issues)
}

/// Audit freshness - find stale generated content or metrics
pub async fn audit_freshness(
    state: &State,
    scope: &Option<AuditScope>,
    limit: u32,
) -> Result<Vec<AuditIssue>> {
    let mut issues = Vec::new();

    // Build class filter
    let class_filter = scope
        .as_ref()
        .and_then(|s| s.class.as_ref())
        .map(|c| format!("AND labels(n)[0] = '{}'", sanitize_string(c)))
        .unwrap_or_default();

    // Query: Find nodes with stale updated_at (older than 30 days)
    // Note: Using 30 days as threshold for generated/retrieved content
    let cypher = format!(
        r#"
        MATCH (n)
        WHERE (n:BlockNative OR n:PageNative OR n:SEOKeywordMetrics)
        AND n.updated_at IS NOT NULL
        AND n.updated_at < timestamp() - 30 * 24 * 60 * 60 * 1000
        {}
        RETURN n.key AS node_key, labels(n)[0] AS node_class,
               datetime({{epochMillis: n.updated_at}}) AS last_updated
        LIMIT $limit
        "#,
        class_filter
    );

    let mut params = serde_json::Map::new();
    params.insert("limit".to_string(), serde_json::json!(limit));

    let rows = state.pool().execute_query(&cypher, Some(params)).await?;

    for row in rows {
        if let (Some(node_key), Some(node_class)) =
            (row["node_key"].as_str(), row["node_class"].as_str())
        {
            let last_updated = row["last_updated"].as_str().unwrap_or("unknown");
            issues.push(
                AuditIssue::info(
                    "freshness",
                    format!(
                        "{}:'{}' last updated {} (>30 days ago)",
                        node_class, node_key, last_updated
                    ),
                )
                .with_node_key(node_key)
                .with_details(serde_json::json!({
                    "last_updated": last_updated,
                    "threshold_days": 30
                })),
            );
        }
    }

    Ok(issues)
}

// ══════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ══════════════════════════════════════════════════════════════

/// Sanitize string for use in Cypher (basic injection prevention)
fn sanitize_string(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

/// Count total entities
async fn count_entities(state: &State) -> Result<u32> {
    let cypher = "MATCH (e:Entity) RETURN count(e) AS count";
    let rows = state.pool().execute_query(cypher, None).await?;
    Ok(rows
        .first()
        .and_then(|r| r["count"].as_u64())
        .unwrap_or(0) as u32)
}

/// Count *Native nodes
async fn count_native_nodes(state: &State) -> Result<u32> {
    let cypher = r#"
        MATCH (n)
        WHERE n:EntityNative OR n:PageNative OR n:BlockNative
        RETURN count(n) AS count
    "#;
    let rows = state.pool().execute_query(cypher, None).await?;
    Ok(rows
        .first()
        .and_then(|r| r["count"].as_u64())
        .unwrap_or(0) as u32)
}

/// Count generated nodes
async fn count_generated_nodes(state: &State) -> Result<u32> {
    let cypher = r#"
        MATCH (n)
        WHERE n:BlockNative OR n:PageNative OR n:SEOKeywordMetrics
        RETURN count(n) AS count
    "#;
    let rows = state.pool().execute_query(cypher, None).await?;
    Ok(rows
        .first()
        .and_then(|r| r["count"].as_u64())
        .unwrap_or(0) as u32)
}

/// Count all nodes
async fn count_all_nodes(state: &State) -> Result<u32> {
    let cypher = "MATCH (n) RETURN count(n) AS count";
    let rows = state.pool().execute_query(cypher, None).await?;
    Ok(rows
        .first()
        .and_then(|r| r["count"].as_u64())
        .unwrap_or(0) as u32)
}

/// Count all arcs
async fn count_arcs(state: &State) -> Result<u32> {
    let cypher = "MATCH ()-[r]->() RETURN count(r) AS count";
    let rows = state.pool().execute_query(cypher, None).await?;
    Ok(rows
        .first()
        .and_then(|r| r["count"].as_u64())
        .unwrap_or(0) as u32)
}

/// Generate recommendations based on issues
fn generate_recommendations(issues: &[AuditIssue]) -> Vec<String> {
    let mut recs = Vec::new();

    // Count issues by category
    let coverage_count = issues.iter().filter(|i| i.category == "coverage").count();
    let orphan_count = issues.iter().filter(|i| i.category == "orphan").count();
    let integrity_count = issues.iter().filter(|i| i.category == "integrity").count();
    let freshness_count = issues.iter().filter(|i| i.category == "freshness").count();

    if coverage_count > 0 {
        recs.push(format!(
            "Run EntityNative generation workflow for {} missing locale combinations",
            coverage_count
        ));
    }

    if orphan_count > 0 {
        recs.push(format!(
            "Create missing FOR_LOCALE arcs for {} orphaned *Native nodes",
            orphan_count
        ));
    }

    if integrity_count > 0 {
        recs.push(format!(
            "Fix {} integrity issues - check for missing parent nodes",
            integrity_count
        ));
    }

    if freshness_count > 0 {
        recs.push(format!(
            "Consider regenerating {} stale nodes (>30 days old)",
            freshness_count
        ));
    }

    recs
}

/// Build ontology insights from issues and CSR
fn build_insights(issues: &[AuditIssue], csr: &ConstraintSatisfactionRate) -> OntologyInsights {
    // Find most violated constraint (arc_class)
    let mut constraint_counts: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
    for issue in issues {
        if let Some(arc) = &issue.arc_class {
            *constraint_counts.entry(arc.as_str()).or_insert(0) += 1;
        }
    }
    let most_violated = constraint_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(arc, _)| arc.to_string());

    // Determine health based on CSR
    let severity = csr.severity();
    let attention = match severity {
        crate::metrics::CsrSeverity::Warning | crate::metrics::CsrSeverity::Critical => {
            Some(format!("CSR {:.1}% needs attention", csr.rate * 100.0))
        }
        _ => None,
    };

    OntologyInsights {
        most_violated_constraint: most_violated,
        healthiest_layer: if csr.rate >= 0.95 {
            Some(format!("Overall CSR {:.1}%", csr.rate * 100.0))
        } else {
            None
        },
        attention_needed: attention,
        traversal_gaps: Vec::new(), // Would require more complex analysis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_string() {
        assert_eq!(sanitize_string("valid-key_123"), "valid-key_123");
        assert_eq!(sanitize_string("invalid';DROP"), "invalidDROP");
        assert_eq!(sanitize_string("test@locale"), "testlocale");
    }

    #[test]
    fn test_generate_recommendations_coverage() {
        let issues = vec![
            AuditIssue::warning("coverage", "Test 1"),
            AuditIssue::warning("coverage", "Test 2"),
        ];
        let recs = generate_recommendations(&issues);
        assert!(recs.iter().any(|r| r.contains("2 missing locale")));
    }

    #[test]
    fn test_generate_recommendations_orphans() {
        let issues = vec![AuditIssue::critical("orphan", "Test")];
        let recs = generate_recommendations(&issues);
        assert!(recs.iter().any(|r| r.contains("FOR_LOCALE")));
    }

    #[test]
    fn test_generate_recommendations_empty() {
        let issues: Vec<AuditIssue> = vec![];
        let recs = generate_recommendations(&issues);
        assert!(recs.is_empty());
    }

    #[test]
    fn test_build_insights_most_violated() {
        let issues = vec![
            AuditIssue::critical("orphan", "Test 1").with_arc_class("FOR_LOCALE"),
            AuditIssue::critical("orphan", "Test 2").with_arc_class("FOR_LOCALE"),
            AuditIssue::warning("coverage", "Test 3").with_arc_class("HAS_NATIVE"),
        ];
        let csr = ConstraintSatisfactionRate::new(97, 3);
        let insights = build_insights(&issues, &csr);

        assert_eq!(
            insights.most_violated_constraint,
            Some("FOR_LOCALE".to_string())
        );
    }

    #[test]
    fn test_build_insights_healthy() {
        let issues: Vec<AuditIssue> = vec![];
        let csr = ConstraintSatisfactionRate::new(100, 0);
        let insights = build_insights(&issues, &csr);

        assert!(insights.healthiest_layer.is_some());
        assert!(insights.attention_needed.is_none());
    }
}
