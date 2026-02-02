//! Dashboard statistics: live Neo4j metrics for the mission control panel.
//!
//! Shows node counts by Realm, edge counts by EdgeFamily, and total graph
//! stats. Data is fetched asynchronously and rendered as bar charts and
//! sparklines.

use crate::db::Db;

/// Aggregated graph statistics for the dashboard panel.
#[derive(Debug, Clone, Default)]
pub struct DashboardStats {
    /// Node counts by Realm (key, display_name, count).
    pub realm_counts: Vec<RealmCount>,
    /// Edge counts by EdgeFamily (key, count).
    pub family_counts: Vec<FamilyCount>,
    /// Total number of data nodes (non-meta).
    pub total_nodes: u64,
    /// Total number of relationships.
    pub total_edges: u64,
    /// Number of Kind meta-nodes.
    pub kind_count: u64,
    /// Number of EdgeKind meta-nodes.
    pub edge_kind_count: u64,
}

/// Node count for a single Realm.
#[derive(Debug, Clone)]
pub struct RealmCount {
    pub key: String,
    pub display_name: String,
    pub count: u64,
}

/// Edge count for a single EdgeFamily.
#[derive(Debug, Clone)]
pub struct FamilyCount {
    pub key: String,
    pub count: u64,
}

impl DashboardStats {
    /// Human-readable summary line (e.g., "142 nodes, 287 edges, 35 Kinds").
    pub fn summary(&self) -> String {
        format!(
            "{} nodes, {} edges, {} Kinds, {} EdgeKinds",
            self.total_nodes, self.total_edges, self.kind_count, self.edge_kind_count
        )
    }

    /// Maximum count in realm_counts (for bar chart scaling).
    pub fn max_realm_count(&self) -> u64 {
        self.realm_counts.iter().map(|r| r.count).max().unwrap_or(1)
    }

    /// Maximum count in family_counts (for bar chart scaling).
    pub fn max_family_count(&self) -> u64 {
        self.family_counts
            .iter()
            .map(|f| f.count)
            .max()
            .unwrap_or(1)
    }
}

/// Fetch dashboard statistics from Neo4j.
///
/// Runs multiple lightweight queries in sequence. Non-fatal: returns
/// partial stats if some queries fail.
pub async fn fetch_stats(db: &Db) -> crate::Result<DashboardStats> {
    let mut stats = DashboardStats::default();

    // 1. Node counts by Realm
    let realm_query = "\
MATCH (k:Kind)-[:IN_REALM]->(r:Realm)
WITH r.key AS realm_key, r.display_name AS display_name, count(k) AS kind_count
RETURN realm_key, display_name, kind_count
ORDER BY realm_key";
    if let Ok(rows) = db.execute(realm_query).await {
        for row in &rows {
            let key: String = row.get("realm_key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();
            let count: i64 = row.get("kind_count").unwrap_or(0);
            stats.realm_counts.push(RealmCount {
                key,
                display_name,
                count: count as u64,
            });
        }
    }

    // 2. Edge counts by EdgeFamily
    let family_query = "\
MATCH (ek:EdgeKind)-[:IN_FAMILY]->(ef:EdgeFamily)
WITH ef.key AS family_key, count(ek) AS edge_count
RETURN family_key, edge_count
ORDER BY family_key";
    if let Ok(rows) = db.execute(family_query).await {
        for row in &rows {
            let key: String = row.get("family_key").unwrap_or_default();
            let count: i64 = row.get("edge_count").unwrap_or(0);
            stats.family_counts.push(FamilyCount {
                key,
                count: count as u64,
            });
        }
    }

    // 3. Total counts (meta-graph summary)
    let totals_query = "\
OPTIONAL MATCH (k:Kind)
WITH count(k) AS kinds
OPTIONAL MATCH (ek:EdgeKind)
WITH kinds, count(ek) AS edge_kinds
RETURN kinds, edge_kinds";
    if let Ok(rows) = db.execute(totals_query).await {
        if let Some(row) = rows.first() {
            stats.kind_count = row.get::<i64>("kinds").unwrap_or(0) as u64;
            stats.edge_kind_count = row.get::<i64>("edge_kinds").unwrap_or(0) as u64;
        }
    }

    // 4. Total data nodes (non-meta)
    let data_query = "MATCH (n) WHERE NOT n:Meta RETURN count(n) AS total";
    if let Ok(rows) = db.execute(data_query).await {
        if let Some(row) = rows.first() {
            stats.total_nodes = row.get::<i64>("total").unwrap_or(0) as u64;
        }
    }

    // 5. Total relationships
    let edge_query = "MATCH ()-[r]->() RETURN count(r) AS total";
    if let Ok(rows) = db.execute(edge_query).await {
        if let Some(row) = rows.first() {
            stats.total_edges = row.get::<i64>("total").unwrap_or(0) as u64;
        }
    }

    Ok(stats)
}

/// Format a horizontal bar for a count value (for terminal rendering).
///
/// Returns a bar like "████████░░░░" proportional to value/max.
pub fn bar(value: u64, max: u64, width: usize) -> String {
    if max == 0 {
        return "░".repeat(width);
    }
    let filled = ((value as f64 / max as f64) * width as f64).round() as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bar_full() {
        assert_eq!(bar(10, 10, 10), "██████████");
    }

    #[test]
    fn bar_half() {
        assert_eq!(bar(5, 10, 10), "█████░░░░░");
    }

    #[test]
    fn bar_empty() {
        assert_eq!(bar(0, 10, 10), "░░░░░░░░░░");
    }

    #[test]
    fn bar_zero_max() {
        assert_eq!(bar(0, 0, 8), "░░░░░░░░");
    }

    #[test]
    fn summary_format() {
        let stats = DashboardStats {
            total_nodes: 142,
            total_edges: 287,
            kind_count: 35,
            edge_kind_count: 47,
            ..Default::default()
        };
        assert_eq!(
            stats.summary(),
            "142 nodes, 287 edges, 35 Kinds, 47 EdgeKinds"
        );
    }

    #[test]
    fn max_realm_count_empty() {
        let stats = DashboardStats::default();
        assert_eq!(stats.max_realm_count(), 1);
    }

    #[test]
    fn max_realm_count_values() {
        let stats = DashboardStats {
            realm_counts: vec![
                RealmCount {
                    key: "global".into(),
                    display_name: "Global".into(),
                    count: 15,
                },
                RealmCount {
                    key: "project".into(),
                    display_name: "Project".into(),
                    count: 20,
                },
            ],
            ..Default::default()
        };
        assert_eq!(stats.max_realm_count(), 20);
    }

    #[test]
    fn max_family_count_values() {
        let stats = DashboardStats {
            family_counts: vec![
                FamilyCount {
                    key: "structural".into(),
                    count: 10,
                },
                FamilyCount {
                    key: "semantic".into(),
                    count: 25,
                },
            ],
            ..Default::default()
        };
        assert_eq!(stats.max_family_count(), 25);
    }
}
