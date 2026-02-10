//! Data quality audit for TUI.
//!
//! Calculates coverage statistics across all Kinds and their instances,
//! enabling the Audit mode dashboard that shows data quality at a glance.

use crate::db::Db;

/// Audit statistics for a single Kind.
#[derive(Debug, Clone)]
#[allow(dead_code)] // WIP: Audit mode implementation
pub struct KindAuditStats {
    /// Kind identifier (e.g., "Culture", "Locale")
    pub kind_key: String,
    /// Display name for the Kind
    pub display_name: String,
    /// Number of instances
    pub instance_count: usize,
    /// Total properties in schema
    pub total_properties: usize,
    /// Number of instances with all required fields filled
    pub complete_instances: usize,
    /// Number of instances with missing required fields
    pub incomplete_instances: usize,
    /// Average coverage percentage (0-100)
    pub coverage_percent: u8,
    /// Total missing required fields across all instances
    pub total_missing_required: usize,
}

#[allow(dead_code)] // WIP: Audit mode implementation
impl KindAuditStats {
    /// Check if this Kind has perfect data quality.
    pub fn is_complete(&self) -> bool {
        self.incomplete_instances == 0
    }

    /// Get issue count for display.
    pub fn issue_count(&self) -> usize {
        self.incomplete_instances
    }
}

/// Shared audit statistics summary.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)] // WIP: Audit mode implementation
pub struct SharedAuditStats {
    /// Stats per Kind
    pub kinds: Vec<KindAuditStats>,
    /// Total instances across all Kinds
    pub total_instances: usize,
    /// Total Kinds with issues
    pub kinds_with_issues: usize,
    /// Shared coverage percentage
    pub global_coverage: u8,
    /// Total missing required fields
    pub total_issues: usize,
}

#[allow(dead_code)] // WIP: Audit mode implementation
impl SharedAuditStats {
    /// Calculate global stats from Kind stats.
    pub fn from_kinds(kinds: Vec<KindAuditStats>) -> Self {
        let total_instances: usize = kinds.iter().map(|k| k.instance_count).sum();
        let kinds_with_issues = kinds.iter().filter(|k| !k.is_complete()).count();
        let total_issues: usize = kinds.iter().map(|k| k.total_missing_required).sum();

        // Calculate weighted average coverage
        let total_weighted: usize = kinds
            .iter()
            .map(|k| k.coverage_percent as usize * k.instance_count)
            .sum();
        let global_coverage = if total_instances > 0 {
            (total_weighted / total_instances) as u8
        } else {
            100
        };

        Self {
            kinds,
            total_instances,
            kinds_with_issues,
            global_coverage,
            total_issues,
        }
    }
}

/// Load audit statistics from Neo4j.
/// This queries all Kinds and their instances to calculate coverage.
#[allow(dead_code)] // WIP: Audit mode implementation
pub async fn load_audit_stats(db: &Db) -> crate::Result<SharedAuditStats> {
    // Query all Kinds with their required properties and instance counts
    let cypher = r#"
MATCH (k:Kind:Meta)
OPTIONAL MATCH (n)-[:OF_KIND]->(k)
WHERE NOT n:Meta
WITH k, count(n) AS instance_count
RETURN k.label AS kind_key,
       coalesce(k.display_name, k.label) AS display_name,
       coalesce(k.required_properties, []) AS required_props,
       coalesce(k.properties, []) AS all_props,
       instance_count
ORDER BY instance_count DESC
"#;

    let rows = db.execute(cypher).await?;
    let mut kinds = Vec::with_capacity(rows.len());

    for row in rows {
        let kind_key: String = row.get("kind_key").unwrap_or_default();
        let display_name: String = row.get("display_name").unwrap_or_default();
        let required_props: Vec<String> = row.get("required_props").unwrap_or_default();
        let all_props: Vec<String> = row.get("all_props").unwrap_or_default();
        let instance_count: i64 = row.get("instance_count").unwrap_or(0);

        if kind_key.is_empty() {
            continue;
        }

        // For now, calculate a rough estimate based on instance count and required props
        // A more accurate calculation would query each instance's properties
        // but that would be very expensive for large datasets
        let total_properties = required_props.len() + all_props.len();

        // Assume 85% average coverage for instances (placeholder - real calculation needs instance query)
        let coverage_percent = if instance_count > 0 { 85 } else { 100 };

        // Estimate incomplete instances (placeholder)
        let incomplete_instances = if instance_count > 0 && !required_props.is_empty() {
            (instance_count as f64 * 0.15).ceil() as usize // ~15% incomplete estimate
        } else {
            0
        };

        let complete_instances = (instance_count as usize).saturating_sub(incomplete_instances);
        let total_missing_required = incomplete_instances * required_props.len().max(1);

        kinds.push(KindAuditStats {
            kind_key,
            display_name,
            instance_count: instance_count as usize,
            total_properties,
            complete_instances,
            incomplete_instances,
            coverage_percent,
            total_missing_required,
        });
    }

    Ok(SharedAuditStats::from_kinds(kinds))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_audit_stats_is_complete() {
        let complete = KindAuditStats {
            kind_key: "Locale".to_string(),
            display_name: "Locale".to_string(),
            instance_count: 200,
            total_properties: 10,
            complete_instances: 200,
            incomplete_instances: 0,
            coverage_percent: 100,
            total_missing_required: 0,
        };

        let incomplete = KindAuditStats {
            kind_key: "Culture".to_string(),
            display_name: "Culture".to_string(),
            instance_count: 200,
            total_properties: 15,
            complete_instances: 180,
            incomplete_instances: 20,
            coverage_percent: 90,
            total_missing_required: 40,
        };

        assert!(complete.is_complete());
        assert!(!incomplete.is_complete());
        assert_eq!(incomplete.issue_count(), 20);
    }

    #[test]
    fn test_global_stats_from_kinds() {
        let kinds = vec![
            KindAuditStats {
                kind_key: "Locale".to_string(),
                display_name: "Locale".to_string(),
                instance_count: 100,
                total_properties: 10,
                complete_instances: 100,
                incomplete_instances: 0,
                coverage_percent: 100,
                total_missing_required: 0,
            },
            KindAuditStats {
                kind_key: "Culture".to_string(),
                display_name: "Culture".to_string(),
                instance_count: 100,
                total_properties: 15,
                complete_instances: 80,
                incomplete_instances: 20,
                coverage_percent: 80,
                total_missing_required: 40,
            },
        ];

        let global = SharedAuditStats::from_kinds(kinds);

        assert_eq!(global.total_instances, 200);
        assert_eq!(global.kinds_with_issues, 1);
        assert_eq!(global.total_issues, 40);
        // Weighted average: (100*100 + 80*100) / 200 = 90
        assert_eq!(global.global_coverage, 90);
    }
}
