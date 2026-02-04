//! Context assembly engine executes traversal plans.
//!
//! v10: BFS traversal with budget tracking, temperature-based semantic arc
//! filtering, depth limiting per Kind rules, and full provenance logging.

use crate::error::Result;
use neo4rs::{Graph, query};
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

use super::meta::MetaGraphReader;
use super::planner::{PlanStep, TraversalPlanner};
use super::types::*;

/// Context assembly engine.
pub struct ContextEngine {
    graph: Arc<Graph>,
    meta: MetaGraphReader,
}

impl ContextEngine {
    /// Create a new engine.
    pub async fn new(graph: Arc<Graph>) -> Result<Self> {
        let meta = MetaGraphReader::new(graph.clone()).await?;
        Ok(Self { graph, meta })
    }

    /// Assemble context window for a block + locale.
    pub async fn assemble(&self, request: &ContextRequest) -> Result<ContextWindow> {
        let mut window = ContextWindow {
            nodes: Vec::new(),
            edges: Vec::new(),
            tokens_used: 0,
            token_budget: request.token_budget,
            traversal_log: Vec::new(),
        };

        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(String, String, u8)> = VecDeque::new(); // (key, kind, depth)

        // Start with the block
        let block = self.fetch_node(&request.block_key).await?;
        if let Some(node) = block {
            window.tokens_used += node.token_estimate;
            visited.insert(node.key.clone());
            queue.push_back((node.key.clone(), node.kind.clone(), 0));
            window.nodes.push(node);
        }

        // BFS traversal
        while let Some((current_key, current_kind, depth)) = queue.pop_front() {
            // Get traversal plan for this kind
            let planner = TraversalPlanner::new(&self.meta);
            let plan = planner.plan_for_kind(&current_kind, request);

            for step in plan {
                self.process_step(
                    &step,
                    &current_key,
                    depth,
                    request,
                    &mut window,
                    &mut visited,
                    &mut queue,
                )
                .await?;
            }
        }

        Ok(window)
    }

    /// Process a single plan step.
    #[allow(clippy::too_many_arguments)]
    async fn process_step(
        &self,
        step: &PlanStep,
        current_key: &str,
        depth: u8,
        request: &ContextRequest,
        window: &mut ContextWindow,
        visited: &mut HashSet<String>,
        queue: &mut VecDeque<(String, String, u8)>,
    ) -> Result<()> {
        // Check depth
        if depth >= step.max_depth {
            window.traversal_log.push(TraversalStep {
                depth,
                from_key: current_key.to_string(),
                arc_kind: step.arc_kind.clone(),
                to_key: "?".into(),
                decision: TraversalDecision::DepthExceeded,
                reason: format!("depth {} >= max {}", depth, step.max_depth),
            });
            return Ok(());
        }

        // Check temperature threshold
        if let Some(threshold) = step.temperature_threshold {
            if request.temperature < threshold {
                window.traversal_log.push(TraversalStep {
                    depth,
                    from_key: current_key.to_string(),
                    arc_kind: step.arc_kind.clone(),
                    to_key: "?".into(),
                    decision: TraversalDecision::ThresholdNotMet,
                    reason: format!("temp {} < threshold {}", request.temperature, threshold),
                });
                return Ok(());
            }
        }

        // Fetch connected nodes
        let neighbors = self
            .fetch_neighbors(current_key, &step.arc_kind, &request.locale_key)
            .await?;

        for (neighbor, edge) in neighbors {
            if visited.contains(&neighbor.key) {
                continue;
            }

            // Check budget
            if window.tokens_used + neighbor.token_estimate > request.token_budget {
                window.traversal_log.push(TraversalStep {
                    depth: depth + 1,
                    from_key: current_key.to_string(),
                    arc_kind: step.arc_kind.clone(),
                    to_key: neighbor.key.clone(),
                    decision: TraversalDecision::BudgetExceeded,
                    reason: format!(
                        "tokens {} + {} > budget {}",
                        window.tokens_used, neighbor.token_estimate, request.token_budget
                    ),
                });
                continue;
            }

            // Include node
            window.traversal_log.push(TraversalStep {
                depth: depth + 1,
                from_key: current_key.to_string(),
                arc_kind: step.arc_kind.clone(),
                to_key: neighbor.key.clone(),
                decision: TraversalDecision::Include,
                reason: "within budget and depth".into(),
            });

            window.tokens_used += neighbor.token_estimate;
            visited.insert(neighbor.key.clone());
            queue.push_back((neighbor.key.clone(), neighbor.kind.clone(), depth + 1));
            window.nodes.push(neighbor);
            window.edges.push(edge);
        }

        Ok(())
    }

    /// Fetch a single node by key.
    async fn fetch_node(&self, key: &str) -> Result<Option<ContextNode>> {
        let q = query(
            r#"
            MATCH (n {key: $key})
            OPTIONAL MATCH (n)-[:OF_KIND]->(k:Kind:Meta)-[:HAS_TRAIT]->(t:Trait:Meta)
            OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer:Meta)-[:IN_REALM]->(r:Realm:Meta)
            RETURN n, labels(n)[0] AS kind,
                   r.key AS realm, l.key AS layer, t.key AS trait_type,
                   coalesce(k.token_estimate, 100) AS token_estimate
        "#,
        )
        .param("key", key);

        let mut result = self.graph.execute(q).await?;
        if let Some(row) = result.next().await? {
            // Extract common properties from node
            let _node: neo4rs::Node = row.get("n")?;
            Ok(Some(ContextNode {
                key: key.to_string(),
                kind: row.get("kind").unwrap_or_default(),
                realm: row.get("realm").unwrap_or_default(),
                layer: row.get("layer").unwrap_or_default(),
                trait_type: row.get("trait_type").unwrap_or_default(),
                properties: serde_json::json!({"key": key}), // Simplified - full properties via separate query
                token_estimate: row.get::<i64>("token_estimate").unwrap_or(100) as u32,
                depth: 0,
            }))
        } else {
            Ok(None)
        }
    }

    /// Fetch neighbors via a specific arc type.
    async fn fetch_neighbors(
        &self,
        from_key: &str,
        arc_kind: &str,
        locale_key: &str,
    ) -> Result<Vec<(ContextNode, ContextEdge)>> {
        let q = query(&format!(
            r#"
            MATCH (from {{key: $from_key}})-[r:{arc_kind}]->(to)
            WHERE NOT to:Meta
            OPTIONAL MATCH (to)-[:OF_KIND]->(k:Kind:Meta)-[:HAS_TRAIT]->(t:Trait:Meta)
            OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer:Meta)-[:IN_REALM]->(realm:Realm:Meta)
            OPTIONAL MATCH (r)-[:OF_ARC_KIND]->(ak:ArcKind:Meta)-[:IN_FAMILY]->(af:ArcFamily:Meta)
            RETURN to, labels(to)[0] AS kind,
                   realm.key AS realm, l.key AS layer, t.key AS trait_type,
                   coalesce(k.token_estimate, 100) AS token_estimate,
                   r, type(r) AS rel_type, af.key AS family
        "#,
            arc_kind = arc_kind
        ))
        .param("from_key", from_key)
        .param("locale_key", locale_key);

        let mut result = self.graph.execute(q).await?;
        let mut neighbors = Vec::new();

        while let Some(row) = result.next().await? {
            let to_node: neo4rs::Node = row.get("to")?;
            let _rel: neo4rs::Relation = row.get("r")?;
            let to_key: String = to_node.get::<String>("key").unwrap_or_default();

            let node = ContextNode {
                key: to_key.clone(),
                kind: row.get("kind").unwrap_or_default(),
                realm: row.get("realm").unwrap_or_default(),
                layer: row.get("layer").unwrap_or_default(),
                trait_type: row.get("trait_type").unwrap_or_default(),
                properties: serde_json::json!({"key": &to_key}), // Simplified
                token_estimate: row.get::<i64>("token_estimate").unwrap_or(100) as u32,
                depth: 0, // Will be set by caller
            };

            let edge = ContextEdge {
                from_key: from_key.to_string(),
                to_key: node.key.clone(),
                arc_kind: row.get::<String>("rel_type").unwrap_or_default(),
                family: row.get::<String>("family").unwrap_or_default(),
                properties: serde_json::json!({}), // Simplified
            };

            neighbors.push((node, edge));
        }

        Ok(neighbors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_window_default() {
        let window = ContextWindow {
            nodes: vec![],
            edges: vec![],
            tokens_used: 0,
            token_budget: 1000,
            traversal_log: vec![],
        };
        assert_eq!(window.token_budget, 1000);
        assert!(window.nodes.is_empty());
    }

    #[test]
    fn test_traversal_decision_serialization() {
        let decision = TraversalDecision::Include;
        let json = serde_json::to_string(&decision).unwrap();
        assert_eq!(json, "\"include\"");
    }
}
