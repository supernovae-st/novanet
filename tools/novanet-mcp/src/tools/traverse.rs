//! novanet_traverse tool
//!
//! Graph traversal with configurable depth, direction, and arc filtering.
//! Implements RLM-on-KG hop-by-hop pattern for efficient context gathering.

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Traversal direction
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TraversalDirection {
    /// Follow outgoing arcs only
    Outgoing,
    /// Follow incoming arcs only
    Incoming,
    /// Follow both directions
    #[default]
    Both,
}

/// Parameters for novanet_traverse tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct TraverseParams {
    /// Starting node key
    pub start_key: String,
    /// Maximum traversal depth (1-5, default: 2)
    #[serde(default)]
    pub max_depth: Option<usize>,
    /// Traversal direction
    #[serde(default)]
    pub direction: TraversalDirection,
    /// Filter by arc families (ownership, localization, semantic, generation, mining)
    #[serde(default)]
    pub arc_families: Option<Vec<String>>,
    /// Filter by specific arc kinds
    #[serde(default)]
    pub arc_kinds: Option<Vec<String>>,
    /// Filter by target node kinds
    #[serde(default)]
    pub target_kinds: Option<Vec<String>>,
    /// Maximum nodes to return (default: 50)
    #[serde(default)]
    pub limit: Option<usize>,
    /// Include node properties in results
    #[serde(default)]
    pub include_properties: Option<bool>,
}

/// A node in the traversal result
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct TraversalNode {
    /// Node key
    pub key: String,
    /// Node kind (label)
    pub kind: String,
    /// Depth from start node (0 = start node)
    pub depth: usize,
    /// Path from start node (arc kinds)
    pub path: Vec<String>,
    /// Node properties (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// An arc in the traversal result
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct TraversalArc {
    /// Source node key
    pub source: String,
    /// Target node key
    pub target: String,
    /// Arc kind (relationship type)
    pub arc_kind: String,
    /// Arc family
    pub family: String,
}

/// Result from novanet_traverse tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct TraverseResult {
    /// Start node
    pub start: TraversalNode,
    /// Discovered nodes
    pub nodes: Vec<TraversalNode>,
    /// Discovered arcs
    pub arcs: Vec<TraversalArc>,
    /// Maximum depth reached
    pub max_depth_reached: usize,
    /// Whether the limit was hit
    pub limited: bool,
    /// Token estimate for the result
    pub token_estimate: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Execute the novanet_traverse tool
pub async fn execute(state: &State, params: TraverseParams) -> Result<TraverseResult> {
    let start = std::time::Instant::now();

    let max_depth = params.max_depth.unwrap_or(2).min(state.config().max_hops as usize);
    let limit = params.limit.unwrap_or(50).min(200);
    let include_props = params.include_properties.unwrap_or(true);

    // Build arc filter
    let arc_filter = build_arc_filter(&params.arc_families, &params.arc_kinds);

    // Build target kind filter
    let target_filter = build_target_filter(&params.target_kinds);

    // First, get the start node
    let start_query = r#"
        MATCH (n {key: $key})
        RETURN n.key AS key, labels(n)[0] AS kind, properties(n) AS props
    "#;

    let mut query_params = serde_json::Map::new();
    query_params.insert("key".to_string(), serde_json::json!(params.start_key));

    let start_rows = state
        .pool()
        .execute_query(start_query, Some(query_params.clone()))
        .await?;

    let start_node = start_rows
        .first()
        .ok_or_else(|| crate::error::Error::not_found(&params.start_key))?;

    let start_result = TraversalNode {
        key: start_node["key"].as_str().unwrap_or_default().to_string(),
        kind: start_node["kind"].as_str().unwrap_or_default().to_string(),
        depth: 0,
        path: vec![],
        properties: if include_props {
            Some(start_node["props"].clone())
        } else {
            None
        },
    };

    // Traverse the graph
    let traverse_query = format!(
        r#"
        MATCH (start {{key: $key}})
        CALL apoc.path.expandConfig(start, {{
            relationshipFilter: "{arc_filter}",
            minLevel: 1,
            maxLevel: {max_depth},
            uniqueness: "NODE_GLOBAL",
            limit: {limit}
        }})
        YIELD path
        WITH path, last(nodes(path)) AS endNode, length(path) AS depth
        {target_filter}
        UNWIND relationships(path) AS rel
        WITH path, endNode, depth,
             collect(DISTINCT {{
                 source: startNode(rel).key,
                 target: endNode(rel).key,
                 arc_kind: type(rel),
                 family: COALESCE(rel.family, 'unknown')
             }}) AS rels
        RETURN endNode.key AS key,
               labels(endNode)[0] AS kind,
               depth,
               [r IN relationships(path) | type(r)] AS path_arcs,
               properties(endNode) AS props,
               rels
        ORDER BY depth, key
        LIMIT {limit}
        "#,
        arc_filter = if arc_filter.is_empty() {
            ">".to_string()
        } else {
            arc_filter
        },
        max_depth = max_depth,
        target_filter = target_filter,
        limit = limit
    );

    // Try APOC path expansion, fall back to simple traversal if APOC not available
    let traverse_result = state
        .pool()
        .execute_query(&traverse_query, Some(query_params.clone()))
        .await;

    let (nodes, arcs, max_depth_reached) = match traverse_result {
        Ok(rows) => process_apoc_results(&rows, include_props),
        Err(_) => {
            // Fallback to simple traversal without APOC
            simple_traverse(
                state,
                &params,
                &query_params,
                max_depth,
                limit,
                include_props,
            )
            .await?
        }
    };

    let limited = nodes.len() >= limit;

    let json_string = serde_json::to_string(&nodes).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(TraverseResult {
        start: start_result,
        nodes,
        arcs,
        max_depth_reached,
        limited,
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Process APOC path expansion results
fn process_apoc_results(
    rows: &[serde_json::Value],
    include_props: bool,
) -> (Vec<TraversalNode>, Vec<TraversalArc>, usize) {
    let mut nodes = Vec::new();
    let mut arcs = Vec::new();
    let mut max_depth: usize = 0;

    for row in rows {
        let depth = row["depth"].as_u64().unwrap_or(0) as usize;
        max_depth = max_depth.max(depth);

        nodes.push(TraversalNode {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            kind: row["kind"].as_str().unwrap_or_default().to_string(),
            depth,
            path: row["path_arcs"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            properties: if include_props {
                Some(row["props"].clone())
            } else {
                None
            },
        });

        // Extract arcs from rels array
        if let Some(rels) = row["rels"].as_array() {
            for rel in rels {
                arcs.push(TraversalArc {
                    source: rel["source"].as_str().unwrap_or_default().to_string(),
                    target: rel["target"].as_str().unwrap_or_default().to_string(),
                    arc_kind: rel["arc_kind"].as_str().unwrap_or_default().to_string(),
                    family: rel["family"].as_str().unwrap_or("unknown").to_string(),
                });
            }
        }
    }

    // Deduplicate arcs
    arcs.sort_by(|a, b| {
        (&a.source, &a.target, &a.arc_kind).cmp(&(&b.source, &b.target, &b.arc_kind))
    });
    arcs.dedup_by(|a, b| a.source == b.source && a.target == b.target && a.arc_kind == b.arc_kind);

    (nodes, arcs, max_depth)
}

/// Simple traversal without APOC
async fn simple_traverse(
    state: &State,
    params: &TraverseParams,
    query_params: &serde_json::Map<String, serde_json::Value>,
    max_depth: usize,
    limit: usize,
    include_props: bool,
) -> Result<(Vec<TraversalNode>, Vec<TraversalArc>, usize)> {
    let direction = match params.direction {
        TraversalDirection::Outgoing => "-[r]->",
        TraversalDirection::Incoming => "<-[r]-",
        TraversalDirection::Both => "-[r]-",
    };

    let target_filter = build_target_filter(&params.target_kinds);
    let direction_pattern = format!("{}*1..{}", direction, max_depth);

    let cypher = format!(
        r#"
        MATCH (start {{key: $key}})
        MATCH path = (start){direction_pattern}(end)
        WHERE length(path) <= {max_depth}
        {target_filter}
        WITH end, length(path) AS depth, [r IN relationships(path) | type(r)] AS path_arcs
        RETURN DISTINCT end.key AS key,
               labels(end)[0] AS kind,
               depth,
               path_arcs,
               properties(end) AS props
        ORDER BY depth, key
        LIMIT {limit}
        "#,
        direction_pattern = direction_pattern,
        max_depth = max_depth,
        target_filter = target_filter,
        limit = limit
    );

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params.clone()))
        .await?;

    let mut nodes = Vec::new();
    let mut max_depth_reached: usize = 0;

    for row in rows {
        let depth = row["depth"].as_u64().unwrap_or(0) as usize;
        max_depth_reached = max_depth_reached.max(depth);

        nodes.push(TraversalNode {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            kind: row["kind"].as_str().unwrap_or_default().to_string(),
            depth,
            path: row["path_arcs"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            properties: if include_props {
                Some(row["props"].clone())
            } else {
                None
            },
        });
    }

    // Arc extraction not implemented in non-APOC fallback (requires path tracking)
    let arcs = Vec::new();

    Ok((nodes, arcs, max_depth_reached))
}

/// Build arc filter for APOC path expansion
fn build_arc_filter(families: &Option<Vec<String>>, kinds: &Option<Vec<String>>) -> String {
    let mut filters = Vec::new();

    if let Some(kinds) = kinds {
        if !kinds.is_empty() {
            filters.extend(kinds.iter().cloned());
        }
    }

    // Map families to arc kind patterns (simplified)
    if let Some(families) = families {
        for family in families {
            match family.as_str() {
                "ownership" => filters.push("HAS_*|BELONGS_TO*|CONTAINS_*".to_string()),
                "localization" => filters.push("HAS_NATIVE|FOR_LOCALE".to_string()),
                "semantic" => filters.push("USES_*|REFERENCES|SIMILAR_TO".to_string()),
                "generation" => filters.push("GENERATES|DERIVED_FROM".to_string()),
                "mining" => filters.push("TARGETS|RANKS_FOR".to_string()),
                _ => {}
            }
        }
    }

    if filters.is_empty() {
        String::new()
    } else {
        filters.join("|")
    }
}

/// Build target kind filter
fn build_target_filter(kinds: &Option<Vec<String>>) -> String {
    match kinds {
        Some(k) if !k.is_empty() => {
            let labels: Vec<String> = k.iter().map(|l| format!("endNode:{}", l)).collect();
            format!("WHERE ({})", labels.join(" OR "))
        }
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_arc_filter() {
        assert_eq!(build_arc_filter(&None, &None), "");
        assert_eq!(
            build_arc_filter(&None, &Some(vec!["HAS_PAGE".to_string()])),
            "HAS_PAGE"
        );
    }

    #[test]
    fn test_build_target_filter() {
        assert_eq!(build_target_filter(&None), "");
        assert_eq!(
            build_target_filter(&Some(vec!["Entity".to_string()])),
            "WHERE (endNode:Entity)"
        );
    }
}
