//! novanet_search tool (v0.20.0)
//!
//! Unified search across the NovaNet knowledge graph.
//! Modes: find (fulltext/property/hybrid), walk (graph traversal), triggers (trigger-based).
//!
//! v0.20.0: Absorbed novanet_traverse (mode=walk) per D4.

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use tracing::instrument;

// ─── Enums ───────────────────────────────────────────────────────────────────

/// Search mode
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SearchMode {
    /// Fulltext search using Neo4j fulltext indexes
    Fulltext,
    /// Property-based search with exact or partial matching
    Property,
    /// Combined fulltext + property search
    #[default]
    Hybrid,
    /// Graph traversal from a starting node (was novanet_traverse)
    Walk,
    /// Search by trigger terms (v0.20.0)
    Triggers,
}

/// Traversal direction (for walk mode)
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WalkDirection {
    /// Follow outgoing arcs only
    Outgoing,
    /// Follow incoming arcs only
    Incoming,
    /// Follow both directions
    #[default]
    Both,
}

// ─── Params ──────────────────────────────────────────────────────────────────

/// Parameters for novanet_search tool
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Search query string (required for fulltext/property/hybrid/triggers modes)
    #[serde(default)]
    pub query: Option<String>,
    /// Search mode (fulltext, property, hybrid, walk, triggers)
    #[serde(default)]
    pub mode: SearchMode,
    /// Filter by node kinds (e.g., ["Entity", "Page", "Block"])
    #[serde(default)]
    pub kinds: Option<Vec<String>>,
    /// Filter by realm (shared, org)
    #[serde(default)]
    pub realm: Option<String>,
    /// Filter by layer
    #[serde(default)]
    pub layer: Option<String>,
    /// Maximum number of results (default: 20)
    #[serde(default)]
    pub limit: Option<usize>,
    /// Properties to search in (for property mode)
    #[serde(default)]
    pub properties: Option<Vec<String>>,

    // ─── Walk mode params (was novanet_traverse) ─────────────────────────

    /// Starting node key (required for walk mode)
    #[serde(default)]
    pub start_key: Option<String>,
    /// Maximum traversal depth (1-5, default: 2, walk mode only)
    #[serde(default)]
    pub max_depth: Option<usize>,
    /// Traversal direction (walk mode only)
    #[serde(default)]
    pub direction: Option<WalkDirection>,
    /// Filter by arc families (ownership, localization, semantic, generation, mining)
    #[serde(default)]
    pub arc_families: Option<Vec<String>>,
    /// Filter by specific arc kinds
    #[serde(default)]
    pub arc_kinds: Option<Vec<String>>,
    /// Filter by target node kinds (walk mode only)
    #[serde(default)]
    pub target_kinds: Option<Vec<String>>,
    /// Include node properties in results (walk mode, default: true)
    #[serde(default)]
    pub include_properties: Option<bool>,
}

// ─── Result Types ────────────────────────────────────────────────────────────

/// A single search hit (find modes)
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SearchHit {
    /// Node key
    pub key: String,
    /// Node kind (label)
    pub kind: String,
    /// Match score (0.0 - 1.0)
    pub score: f64,
    /// Matched properties with highlights
    pub matches: Vec<PropertyMatch>,
    /// Node properties (subset)
    pub properties: serde_json::Value,
}

/// A matched property
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct PropertyMatch {
    /// Property name
    pub property: String,
    /// Matched value (with highlight markers)
    pub value: String,
}

/// A node in walk traversal results
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct WalkNode {
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

/// An arc in walk traversal results
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct WalkArc {
    /// Source node key
    pub source: String,
    /// Target node key
    pub target: String,
    /// Arc kind (relationship type)
    pub arc_kind: String,
    /// Arc family
    pub family: String,
}

/// Walk-specific result data
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct WalkData {
    /// Start node
    pub start: WalkNode,
    /// Discovered nodes
    pub nodes: Vec<WalkNode>,
    /// Discovered arcs
    pub arcs: Vec<WalkArc>,
    /// Maximum depth reached
    pub max_depth_reached: usize,
    /// Whether the limit was hit
    pub limited: bool,
}

/// Result from novanet_search tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SearchResult {
    /// Search hits (for find modes: fulltext, property, hybrid, triggers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits: Option<Vec<SearchHit>>,
    /// Walk traversal data (for walk mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walk: Option<WalkData>,
    /// Total hits found (may be > returned if limited)
    pub total_hits: usize,
    /// Search mode used
    pub mode: String,
    /// Token estimate for the result
    pub token_estimate: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// ─── Execute ─────────────────────────────────────────────────────────────────

/// Execute the novanet_search tool
#[instrument(name = "novanet_search", skip(state), fields(mode = ?params.mode))]
pub async fn execute(state: &State, params: SearchParams) -> Result<SearchResult> {
    let start = std::time::Instant::now();

    match params.mode {
        SearchMode::Walk => execute_walk(state, params, start).await,
        SearchMode::Triggers => execute_triggers(state, params, start).await,
        _ => execute_find(state, params, start).await,
    }
}

/// Execute find modes (fulltext, property, hybrid)
async fn execute_find(
    state: &State,
    params: SearchParams,
    start: std::time::Instant,
) -> Result<SearchResult> {
    let limit = params.limit.unwrap_or(20).min(100);
    let query = params.query.clone().unwrap_or_default();
    if query.is_empty() {
        return Err(crate::error::Error::InvalidParams(
            "query is required for fulltext/property/hybrid modes".to_string(),
        ));
    }

    let (hits, mode_str) = match params.mode {
        SearchMode::Fulltext => (fulltext_search(state, &params, &query, limit).await?, "fulltext"),
        SearchMode::Property => (property_search(state, &params, &query, limit).await?, "property"),
        _ => (hybrid_search(state, &params, &query, limit).await?, "hybrid"),
    };

    let total_hits = hits.len();
    let json_string = serde_json::to_string(&hits).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(SearchResult {
        hits: Some(hits),
        walk: None,
        total_hits,
        mode: mode_str.to_string(),
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Execute walk mode (was novanet_traverse)
async fn execute_walk(
    state: &State,
    params: SearchParams,
    start: std::time::Instant,
) -> Result<SearchResult> {
    let start_key = params.start_key.clone().ok_or_else(|| {
        crate::error::Error::InvalidParams("start_key is required for walk mode".to_string())
    })?;

    let max_depth = params
        .max_depth
        .unwrap_or(2)
        .min(state.config().max_hops as usize);
    let limit = params.limit.unwrap_or(50).min(200);
    let include_props = params.include_properties.unwrap_or(true);
    let direction = params.direction.clone().unwrap_or_default();

    // Build arc filter
    let arc_filter = build_arc_filter(&params.arc_families, &params.arc_kinds);

    // Build target kind filter
    let target_filter = build_target_filter(&params.target_kinds);

    // Get the start node
    let start_query = r#"
        MATCH (n {key: $key})
        RETURN n.key AS key, labels(n)[0] AS kind, properties(n) AS props
    "#;

    let mut query_params = serde_json::Map::new();
    query_params.insert("key".to_string(), serde_json::json!(start_key));

    let start_rows = state
        .pool()
        .execute_query(start_query, Some(query_params.clone()))
        .await?;

    let start_node = start_rows
        .first()
        .ok_or_else(|| crate::error::Error::not_found(&start_key))?;

    let start_result = WalkNode {
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

    // Build APOC traversal query
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

    // Try APOC, fall back to simple traversal
    let traverse_result = state
        .pool()
        .execute_query(&traverse_query, Some(query_params.clone()))
        .await;

    let (nodes, arcs, max_depth_reached) = match traverse_result {
        Ok(rows) => process_apoc_results(&rows, include_props),
        Err(_) => {
            simple_traverse(
                state,
                &direction,
                &params.target_kinds,
                &query_params,
                max_depth,
                limit,
                include_props,
            )
            .await?
        }
    };

    let limited = nodes.len() >= limit;
    let total_hits = nodes.len();

    let json_string = serde_json::to_string(&nodes).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(SearchResult {
        hits: None,
        walk: Some(WalkData {
            start: start_result,
            nodes,
            arcs,
            max_depth_reached,
            limited,
        }),
        total_hits,
        mode: "walk".to_string(),
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Execute triggers mode — search by trigger terms on nodes
async fn execute_triggers(
    state: &State,
    params: SearchParams,
    start: std::time::Instant,
) -> Result<SearchResult> {
    let query = params.query.clone().unwrap_or_default();
    if query.is_empty() {
        return Err(crate::error::Error::InvalidParams(
            "query is required for triggers mode".to_string(),
        ));
    }
    let limit = params.limit.unwrap_or(20).min(100);

    // Split query into individual trigger terms
    let terms: Vec<&str> = query
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if terms.is_empty() {
        return Ok(SearchResult {
            hits: Some(vec![]),
            walk: None,
            total_hits: 0,
            mode: "triggers".to_string(),
            token_estimate: 0,
            execution_time_ms: start.elapsed().as_millis() as u64,
        });
    }

    let kind_filter = build_kind_filter(&params.kinds, "n");
    let (realm_filter, realm_value) = build_realm_filter(&params.realm, "n", "realm_filter");

    // Search for nodes whose triggers[] array overlaps with query terms
    let cypher = format!(
        r#"
        MATCH (n)
        WHERE n.triggers IS NOT NULL
        AND ANY(t IN n.triggers WHERE toLower(t) IN $terms)
        {kind_filter}
        {realm_filter}
        WITH n,
             size([t IN n.triggers WHERE toLower(t) IN $terms]) AS overlap,
             toFloat(size([t IN n.triggers WHERE toLower(t) IN $terms])) /
               toFloat(size(n.triggers)) AS score
        RETURN n.key AS key,
               labels(n)[0] AS kind,
               score,
               n.name AS name,
               n.content AS content,
               properties(n) AS props
        ORDER BY overlap DESC, score DESC
        LIMIT {limit}
        "#,
        kind_filter = kind_filter,
        realm_filter = realm_filter,
        limit = limit
    );

    let lower_terms: Vec<String> = terms.iter().map(|t| t.to_lowercase()).collect();
    let mut query_params = serde_json::Map::new();
    query_params.insert("terms".to_string(), serde_json::json!(lower_terms));
    if let Some(realm) = realm_value {
        query_params.insert("realm_filter".to_string(), serde_json::json!(realm));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    let hits: Vec<SearchHit> = rows
        .into_iter()
        .map(|row| SearchHit {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            kind: row["kind"].as_str().unwrap_or_default().to_string(),
            score: row["score"].as_f64().unwrap_or(0.0),
            matches: extract_trigger_matches(&row, &lower_terms),
            properties: row["props"].clone(),
        })
        .collect();

    let total_hits = hits.len();
    let json_string = serde_json::to_string(&hits).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(SearchResult {
        hits: Some(hits),
        walk: None,
        total_hits,
        mode: "triggers".to_string(),
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

// ─── Find Implementations ───────────────────────────────────────────────────

/// Fulltext search using Neo4j fulltext indexes
async fn fulltext_search(
    state: &State,
    params: &SearchParams,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchHit>> {
    let kind_filter = build_kind_filter(&params.kinds, "node");
    let (realm_filter, realm_value) = build_realm_filter(&params.realm, "node", "realm_filter");

    let cypher = format!(
        r#"
        CALL db.index.fulltext.queryNodes('novanet_fulltext', $query)
        YIELD node, score
        WHERE score > 0.1
        {kind_filter}
        {realm_filter}
        RETURN node.key AS key,
               labels(node)[0] AS kind,
               score,
               node.name AS name,
               node.content AS content,
               properties(node) AS props
        ORDER BY score DESC
        LIMIT {limit}
        "#,
        kind_filter = kind_filter,
        realm_filter = realm_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("query".to_string(), serde_json::json!(query));
    if let Some(realm) = realm_value {
        query_params.insert("realm_filter".to_string(), serde_json::json!(realm));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| SearchHit {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            kind: row["kind"].as_str().unwrap_or_default().to_string(),
            score: row["score"].as_f64().unwrap_or(0.0),
            matches: extract_matches(&row, query),
            properties: row["props"].clone(),
        })
        .collect())
}

/// Property-based search
async fn property_search(
    state: &State,
    params: &SearchParams,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchHit>> {
    let properties = params.properties.clone().unwrap_or_else(|| {
        vec![
            "key".to_string(),
            "name".to_string(),
            "content".to_string(),
        ]
    });

    let kind_filter = build_kind_filter(&params.kinds, "n");
    let (realm_filter, realm_value) = build_realm_filter(&params.realm, "n", "realm_filter");

    // SECURITY: Validate property names to prevent Cypher injection
    let safe_properties: Vec<&String> = properties
        .iter()
        .filter(|p| is_valid_label(p))
        .collect();

    if safe_properties.is_empty() {
        return Ok(vec![]);
    }

    let mut conditions = String::with_capacity(safe_properties.len() * 60);
    for (i, p) in safe_properties.iter().enumerate() {
        if i > 0 {
            conditions.push_str(" OR ");
        }
        let _ = write!(
            conditions,
            "toLower(toString(n.{})) CONTAINS toLower($query)",
            p
        );
    }

    let cypher = format!(
        r#"
        MATCH (n)
        WHERE ({conditions})
        {kind_filter}
        {realm_filter}
        WITH n,
             CASE
                WHEN toLower(n.key) = toLower($query) THEN 1.0
                WHEN toLower(n.key) CONTAINS toLower($query) THEN 0.8
                WHEN toLower(n.name) CONTAINS toLower($query) THEN 0.6
                ELSE 0.4
             END AS score
        RETURN n.key AS key,
               labels(n)[0] AS kind,
               score,
               n.name AS name,
               n.content AS content,
               properties(n) AS props
        ORDER BY score DESC
        LIMIT {limit}
        "#,
        conditions = conditions,
        kind_filter = kind_filter,
        realm_filter = realm_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("query".to_string(), serde_json::json!(query));
    if let Some(realm) = realm_value {
        query_params.insert("realm_filter".to_string(), serde_json::json!(realm));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| SearchHit {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            kind: row["kind"].as_str().unwrap_or_default().to_string(),
            score: row["score"].as_f64().unwrap_or(0.0),
            matches: extract_matches(&row, query),
            properties: row["props"].clone(),
        })
        .collect())
}

/// Hybrid search combining fulltext and property search
async fn hybrid_search(
    state: &State,
    params: &SearchParams,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchHit>> {
    let fulltext_hits = fulltext_search(state, params, query, limit).await?;

    if !fulltext_hits.is_empty() {
        return Ok(fulltext_hits);
    }

    property_search(state, params, query, limit).await
}

// ─── Walk Implementations ───────────────────────────────────────────────────

/// Process APOC path expansion results
fn process_apoc_results(
    rows: &[serde_json::Value],
    include_props: bool,
) -> (Vec<WalkNode>, Vec<WalkArc>, usize) {
    let mut nodes = Vec::new();
    let mut arcs = Vec::new();
    let mut max_depth: usize = 0;

    for row in rows {
        let depth = row["depth"].as_u64().unwrap_or(0) as usize;
        max_depth = max_depth.max(depth);

        nodes.push(WalkNode {
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

        if let Some(rels) = row["rels"].as_array() {
            for rel in rels {
                arcs.push(WalkArc {
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

/// Simple traversal without APOC (fallback)
async fn simple_traverse(
    state: &State,
    direction: &WalkDirection,
    target_kinds: &Option<Vec<String>>,
    query_params: &serde_json::Map<String, serde_json::Value>,
    max_depth: usize,
    limit: usize,
    include_props: bool,
) -> Result<(Vec<WalkNode>, Vec<WalkArc>, usize)> {
    let direction_pattern = match direction {
        WalkDirection::Outgoing => format!("-[r]->*1..{}", max_depth),
        WalkDirection::Incoming => format!("<-[r]-*1..{}", max_depth),
        WalkDirection::Both => format!("-[r]-*1..{}", max_depth),
    };

    let target_filter = build_target_filter(target_kinds);

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

        nodes.push(WalkNode {
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

    // Arc extraction not available in non-APOC fallback
    let arcs = Vec::new();

    Ok((nodes, arcs, max_depth_reached))
}

// ─── Shared Helpers ─────────────────────────────────────────────────────────

/// Validate that a label/property name only contains safe characters.
/// SECURITY: Prevents label injection attacks by only allowing alphanumeric + underscore.
fn is_valid_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= 100
        && label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Build kind filter clause
/// SECURITY: Labels are validated to prevent injection attacks.
fn build_kind_filter(kinds: &Option<Vec<String>>, var: &str) -> String {
    match kinds {
        Some(k) if !k.is_empty() => {
            let valid_labels: Vec<String> = k
                .iter()
                .filter(|l| is_valid_label(l))
                .map(|l| format!("{}:{}", var, l))
                .collect();
            if valid_labels.is_empty() {
                String::new()
            } else {
                format!("AND ({})", valid_labels.join(" OR "))
            }
        }
        _ => String::new(),
    }
}

/// Build realm filter clause using parameterized query
fn build_realm_filter(
    realm: &Option<String>,
    var: &str,
    param_name: &str,
) -> (String, Option<String>) {
    match realm {
        Some(r) => (
            format!("AND {}.realm = ${}", var, param_name),
            Some(r.clone()),
        ),
        None => (String::new(), None),
    }
}

/// Build arc filter for APOC path expansion (walk mode)
fn build_arc_filter(families: &Option<Vec<String>>, kinds: &Option<Vec<String>>) -> String {
    let mut filters = Vec::new();

    if let Some(kinds) = kinds {
        if !kinds.is_empty() {
            filters.extend(kinds.iter().cloned());
        }
    }

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

/// Build target kind filter for walk mode
/// SECURITY: Labels are validated to prevent injection attacks.
fn build_target_filter(kinds: &Option<Vec<String>>) -> String {
    match kinds {
        Some(k) if !k.is_empty() => {
            let valid_labels: Vec<String> = k
                .iter()
                .filter(|l| is_valid_label(l))
                .map(|l| format!("endNode:{}", l))
                .collect();
            if valid_labels.is_empty() {
                String::new()
            } else {
                format!("WHERE ({})", valid_labels.join(" OR "))
            }
        }
        _ => String::new(),
    }
}

/// Extract property matches from a result row
fn extract_matches(row: &serde_json::Value, query: &str) -> Vec<PropertyMatch> {
    let mut matches = Vec::new();
    let query_lower = query.to_lowercase();

    for prop in ["key", "name", "content"] {
        if let Some(value) = row.get(prop).and_then(|v| v.as_str()) {
            if value.to_lowercase().contains(&query_lower) {
                matches.push(PropertyMatch {
                    property: prop.to_string(),
                    value: value.to_string(),
                });
            }
        }
    }

    matches
}

/// Extract trigger matches from a result row
fn extract_trigger_matches(row: &serde_json::Value, terms: &[String]) -> Vec<PropertyMatch> {
    let mut matches = Vec::new();

    if let Some(props) = row.get("props") {
        if let Some(triggers) = props.get("triggers").and_then(|t| t.as_array()) {
            for trigger in triggers {
                if let Some(t) = trigger.as_str() {
                    if terms.contains(&t.to_lowercase()) {
                        matches.push(PropertyMatch {
                            property: "triggers".to_string(),
                            value: t.to_string(),
                        });
                    }
                }
            }
        }
    }

    matches
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_label() {
        assert!(is_valid_label("Entity"));
        assert!(is_valid_label("Page"));
        assert!(is_valid_label("Entity_Native"));
        assert!(is_valid_label("Test123"));

        assert!(!is_valid_label(""));
        assert!(!is_valid_label("Entity:Foo"));
        assert!(!is_valid_label("Entity OR n"));
        assert!(!is_valid_label("Entity})"));
        assert!(!is_valid_label("'; DROP DATABASE"));
        assert!(!is_valid_label("Entity\n"));
        assert!(!is_valid_label(&"A".repeat(101)));
    }

    #[test]
    fn test_build_kind_filter() {
        assert_eq!(build_kind_filter(&None, "n"), "");
        assert_eq!(build_kind_filter(&Some(vec![]), "n"), "");
        assert_eq!(
            build_kind_filter(&Some(vec!["Entity".to_string()]), "n"),
            "AND (n:Entity)"
        );
        assert_eq!(
            build_kind_filter(&Some(vec!["Entity".to_string(), "Page".to_string()]), "n"),
            "AND (n:Entity OR n:Page)"
        );
        assert_eq!(
            build_kind_filter(&Some(vec!["Entity".to_string()]), "node"),
            "AND (node:Entity)"
        );
    }

    #[test]
    fn test_build_kind_filter_rejects_injection() {
        assert_eq!(
            build_kind_filter(&Some(vec!["Entity:Foo".to_string()]), "n"),
            ""
        );
        assert_eq!(
            build_kind_filter(
                &Some(vec!["Entity".to_string(), "'; DROP DATABASE".to_string()]),
                "n"
            ),
            "AND (n:Entity)"
        );
        assert_eq!(
            build_kind_filter(&Some(vec!["Entity OR 1=1".to_string()]), "n"),
            ""
        );
    }

    #[test]
    fn test_build_realm_filter() {
        assert_eq!(
            build_realm_filter(&None, "n", "realm"),
            (String::new(), None)
        );
        assert_eq!(
            build_realm_filter(&Some("shared".to_string()), "n", "realm"),
            (
                "AND n.realm = $realm".to_string(),
                Some("shared".to_string())
            )
        );
        assert_eq!(
            build_realm_filter(&Some("shared".to_string()), "node", "r"),
            (
                "AND node.realm = $r".to_string(),
                Some("shared".to_string())
            )
        );
    }

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

    #[test]
    fn test_build_target_filter_rejects_injection() {
        assert_eq!(
            build_target_filter(&Some(vec!["Entity:Foo".to_string()])),
            ""
        );
        assert_eq!(
            build_target_filter(&Some(vec![
                "Entity".to_string(),
                "'; DROP DATABASE".to_string()
            ])),
            "WHERE (endNode:Entity)"
        );
    }

    #[test]
    fn test_extract_matches() {
        let row = serde_json::json!({
            "key": "test-entity",
            "name": "Test Entity Name",
            "content": "A test entity for testing"
        });

        let matches = extract_matches(&row, "test");
        assert_eq!(matches.len(), 3);
        assert!(matches.iter().any(|m| m.property == "key"));
        assert!(matches.iter().any(|m| m.property == "name"));
        assert!(matches.iter().any(|m| m.property == "content"));
    }

    #[test]
    fn test_extract_trigger_matches() {
        let row = serde_json::json!({
            "props": {
                "triggers": ["qr", "barcode", "scan"]
            }
        });

        let terms = vec!["qr".to_string(), "scan".to_string()];
        let matches = extract_trigger_matches(&row, &terms);
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().any(|m| m.value == "qr"));
        assert!(matches.iter().any(|m| m.value == "scan"));
    }

    #[test]
    fn test_extract_trigger_matches_empty() {
        let row = serde_json::json!({
            "props": {
                "triggers": ["qr", "barcode"]
            }
        });

        let terms = vec!["zebra".to_string()];
        let matches = extract_trigger_matches(&row, &terms);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_search_mode_default() {
        let mode = SearchMode::default();
        assert!(matches!(mode, SearchMode::Hybrid));
    }

    #[test]
    fn test_walk_direction_default() {
        let dir = WalkDirection::default();
        assert!(matches!(dir, WalkDirection::Both));
    }

    #[test]
    fn test_property_search_rejects_injection() {
        // Property names like "key}) RETURN n //--" must be rejected
        assert!(!is_valid_label("key}) RETURN n"));
        assert!(!is_valid_label("name; DROP"));
        assert!(!is_valid_label("key}) DETACH DELETE n"));
        assert!(!is_valid_label("a.b"));
        // Valid property names
        assert!(is_valid_label("key"));
        assert!(is_valid_label("display_name"));
        assert!(is_valid_label("content"));
    }
}
