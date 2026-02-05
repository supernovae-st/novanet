//! Traversal planner builds execution plan from meta-graph rules.
//!
//! v10.4: Determines which arcs to traverse and in what order based on
//! ArcFamily default_traversal and ArcKind temperature_threshold.
//!
//! Uses v10 tiered knowledge model:
//! - Technical tier: Formatting, Slugification, Adaptation
//! - Style tier: Style
//! - Semantic tier: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
//!
//! Entity-Centric Architecture (v10.3): Entity/EntityL10n are the primary semantic types.

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
    /// v10.4: Entity-Centric Architecture + tiered knowledge model
    fn get_outgoing_arcs_for_kind(&self, kind: &str) -> Vec<(String, String, String)> {
        // This would ideally query the meta-graph, but for initial implementation
        // we use known patterns
        match kind {
            "Block" => vec![
                ("USES_ENTITY".into(), "semantic".into(), "Entity".into()),
                ("OF_TYPE".into(), "ownership".into(), "BlockType".into()),
                ("HAS_PROMPT".into(), "ownership".into(), "BlockPrompt".into()),
                ("HAS_RULES".into(), "ownership".into(), "BlockRules".into()),
            ],
            // v10.3 Entity-Centric: Entity replaces Concept
            "Entity" => vec![
                ("HAS_L10N".into(), "localization".into(), "EntityL10n".into()),
                ("SEMANTIC_LINK".into(), "semantic".into(), "Entity".into()),
            ],
            "EntityL10n" => vec![("FOR_LOCALE".into(), "localization".into(), "Locale".into())],
            // v10 tiered knowledge model
            "Locale" => vec![
                // Technical tier
                ("HAS_FORMATTING".into(), "localization".into(), "Formatting".into()),
                ("HAS_SLUGIFICATION".into(), "localization".into(), "Slugification".into()),
                ("HAS_ADAPTATION".into(), "localization".into(), "Adaptation".into()),
                // Style tier
                ("HAS_STYLE".into(), "localization".into(), "Style".into()),
                // Semantic tier (sets)
                ("HAS_TERMS".into(), "localization".into(), "TermSet".into()),
                ("HAS_EXPRESSIONS".into(), "localization".into(), "ExpressionSet".into()),
                ("HAS_PATTERNS".into(), "localization".into(), "PatternSet".into()),
                ("HAS_CULTURE".into(), "localization".into(), "CultureSet".into()),
                ("HAS_TABOOS".into(), "localization".into(), "TabooSet".into()),
                ("HAS_AUDIENCE".into(), "localization".into(), "AudienceSet".into()),
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
