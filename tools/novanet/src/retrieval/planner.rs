//! Traversal planner builds execution plan from meta-graph rules.
//!
//! v10: Determines which arcs to traverse and in what order based on
//! ArcFamily default_traversal and ArcKind temperature_threshold.

use super::meta::MetaGraphReader;
use super::types::{ContextRequest, TraversalMode};

/// A step in the traversal plan.
#[derive(Debug, Clone)]
pub struct PlanStep {
    pub arc_kind: String,
    pub family: String,
    pub target_kind: String,
    pub priority: TraversalPriority,
    pub max_depth: u8,
    pub temperature_threshold: Option<f32>,
}

/// Priority determines traversal order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraversalPriority {
    High = 0,   // Eager arcs (ownership, localization)
    Medium = 1, // Lazy arcs with high threshold
    Low = 2,    // Lazy arcs with low threshold
}

/// Builds traversal plans from meta-graph rules.
pub struct TraversalPlanner<'a> {
    meta: &'a MetaGraphReader,
}

impl<'a> TraversalPlanner<'a> {
    pub fn new(meta: &'a MetaGraphReader) -> Self {
        Self { meta }
    }

    /// Build traversal plan for a given Kind.
    pub fn plan_for_kind(&self, kind: &str, request: &ContextRequest) -> Vec<PlanStep> {
        let mut steps = Vec::new();

        // Get Kind rules for max depth
        let kind_rules = self.meta.get_kind_rules(kind);
        let max_depth = request
            .max_depth
            .unwrap_or_else(|| kind_rules.map(|r| r.traversal_depth).unwrap_or(2));

        // Find all arcs FROM this kind
        // (In real implementation, query meta-graph for FROM_KIND relationships)
        // For now, use common patterns
        let arc_patterns = self.get_outgoing_arcs_for_kind(kind);

        for (arc_kind, family, target) in arc_patterns {
            let family_rules = self.meta.get_family_rules(&family);
            let arc_rules = self.meta.get_arc_rules(&arc_kind);

            let priority = match family_rules.map(|r| r.default_traversal) {
                Some(TraversalMode::Eager) => TraversalPriority::High,
                Some(TraversalMode::Skip) => continue, // Skip this arc
                _ => {
                    // Lazy: priority based on temperature threshold
                    match arc_rules.and_then(|r| r.temperature_threshold) {
                        Some(t) if t <= 0.3 => TraversalPriority::Medium,
                        _ => TraversalPriority::Low,
                    }
                }
            };

            steps.push(PlanStep {
                arc_kind: arc_kind.clone(),
                family,
                target_kind: target,
                priority,
                max_depth,
                temperature_threshold: arc_rules.and_then(|r| r.temperature_threshold),
            });
        }

        // Sort by priority (eager first)
        steps.sort_by_key(|s| s.priority);
        steps
    }

    /// Get outgoing arcs for a kind (hardcoded patterns for common kinds).
    fn get_outgoing_arcs_for_kind(&self, kind: &str) -> Vec<(String, String, String)> {
        // This would ideally query the meta-graph, but for initial implementation
        // we use known patterns
        match kind {
            "Block" => vec![
                ("USES_CONCEPT".into(), "semantic".into(), "Concept".into()),
                ("OF_TYPE".into(), "ownership".into(), "BlockType".into()),
                (
                    "HAS_PROMPT".into(),
                    "ownership".into(),
                    "BlockPrompt".into(),
                ),
                ("HAS_RULES".into(), "ownership".into(), "BlockRules".into()),
            ],
            "Concept" => vec![
                (
                    "HAS_L10N".into(),
                    "localization".into(),
                    "ConceptL10n".into(),
                ),
                ("SEMANTIC_LINK".into(), "semantic".into(), "Concept".into()),
            ],
            "ConceptL10n" => vec![("FOR_LOCALE".into(), "localization".into(), "Locale".into())],
            "Locale" => vec![
                (
                    "HAS_VOICE".into(),
                    "localization".into(),
                    "LocaleVoice".into(),
                ),
                (
                    "HAS_CULTURE".into(),
                    "localization".into(),
                    "LocaleCulture".into(),
                ),
                (
                    "HAS_LEXICON".into(),
                    "localization".into(),
                    "LocaleLexicon".into(),
                ),
            ],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(TraversalPriority::High < TraversalPriority::Medium);
        assert!(TraversalPriority::Medium < TraversalPriority::Low);
    }

    #[test]
    fn test_priority_sort() {
        let mut priorities = vec![
            TraversalPriority::Low,
            TraversalPriority::High,
            TraversalPriority::Medium,
        ];
        priorities.sort();
        assert_eq!(
            priorities,
            vec![
                TraversalPriority::High,
                TraversalPriority::Medium,
                TraversalPriority::Low
            ]
        );
    }
}
