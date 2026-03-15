//! Instance loading queries for the TUI.
//!
//! Generic instance CRUD: load_instances (full), load_instances_fast (keys only),
//! and load_instance_arcs (background arc loading).

use crate::db::{Db, RowExt};
use rustc_hash::FxHashMap;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

use super::conversion::*;
use super::types::*;
use super::TaxonomyTree;

impl TaxonomyTree {
    /// Load instances of a Class from Neo4j for Data view.
    /// Returns (instances, total_count) - instances are limited to INSTANCE_LIMIT, total is the real count.
    ///
    /// Performance: Uses a two-pass query strategy for large datasets:
    /// 1. Fast index scan to get first INSTANCE_LIMIT keys + total count
    /// 2. Detailed query with arcs only for those keys
    ///
    /// This avoids scanning all nodes (e.g., 9100 SEOKeyword) for arc collection.
    pub async fn load_instances(
        db: &Db,
        class_label: &str,
    ) -> crate::Result<(Vec<InstanceInfo>, usize)> {
        // Security: Validate label before interpolation into Cypher
        validate_cypher_label(class_label)?;

        // Pass 1: Get total count AND first N keys in a single fast query (index-based)
        let keys_cypher = format!(
            r#"
MATCH (n:{class_label})
WHERE NOT n:Schema
WITH count(n) AS total
MATCH (n:{class_label})
WHERE NOT n:Schema
WITH total, n.key AS key
ORDER BY key
LIMIT {limit}
RETURN collect(key) AS keys, total
"#,
            class_label = class_label,
            limit = INSTANCE_LIMIT
        );
        let keys_rows = db.execute(&keys_cypher).await?;
        let (keys, total_count): (Vec<String>, usize) = keys_rows
            .first()
            .map(|r| {
                let keys: Vec<String> = r.get("keys").unwrap_or_default();
                let total: i64 = r.get("total").unwrap_or(0);
                (keys, total as usize)
            })
            .unwrap_or_default();

        // Early return if no instances
        if keys.is_empty() {
            return Ok((Vec::new(), total_count));
        }

        // Pass 2: Get properties and arcs only for the selected keys
        // This is much faster than scanning all nodes for arc collection
        let cypher = format!(
            r#"
UNWIND $keys AS k
MATCH (n:{class_label} {{key: k}})
OPTIONAL MATCH (n)-[out]->(target)
WHERE NOT target:Schema
WITH n, k, collect(DISTINCT {{
    arc_type: type(out),
    target_key: coalesce(target.key, target.label, id(target)),
    target_class: head(labels(target)),
    target_display_name: target.display_name,
    target_slug: null
}}) AS outgoing
OPTIONAL MATCH (source)-[inc]->(n)
WHERE NOT source:Schema
WITH n, k, outgoing, collect(DISTINCT {{
    arc_type: type(inc),
    source_key: coalesce(source.key, source.label, id(source)),
    source_class: head(labels(source)),
    source_display_name: source.display_name,
    source_slug: null
}}) AS incoming
RETURN
    coalesce(n.key, n.label, toString(id(n))) AS key,
    coalesce(n.display_name, n.key, n.label) AS display_name,
    properties(n) AS props,
    outgoing,
    incoming
ORDER BY key
"#,
            class_label = class_label
        );

        // Execute with parameterized keys (safe from injection)
        let rows = db.execute_with_params(&cypher, [("keys", keys)]).await?;
        let mut instances = Vec::with_capacity(rows.len());

        for row in rows {
            let key = row.str("key");
            let display_name = row.str("display_name");

            // Parse properties as BTreeMap with proper JSON values
            let props: BTreeMap<String, JsonValue> = row
                .get::<neo4rs::BoltMap>("props")
                .map(|m| {
                    m.value
                        .iter()
                        .map(|(k, v)| (k.value.clone(), bolt_to_json(v)))
                        .collect()
                })
                .unwrap_or_default();

            // Parse outgoing arcs
            let outgoing_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("outgoing")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("target_key").unwrap_or_default(),
                        target_class: m.get("target_class").unwrap_or_default(),
                        exists: true,
                        target_display_name: m.get("target_display_name").ok(),
                        target_slug: m.get("target_slug").ok(),
                    })
                })
                .collect();

            // Parse incoming arcs
            let incoming_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("incoming")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("source_key").unwrap_or_default(),
                        target_class: m.get("source_class").unwrap_or_default(),
                        exists: true,
                        target_display_name: m.get("source_display_name").ok(),
                        target_slug: m.get("source_slug").ok(),
                    })
                })
                .collect();

            // Calculate relationship_power from HAS_NATIVE arc count
            let native_count = outgoing_arcs
                .iter()
                .filter(|a| a.arc_type == "HAS_NATIVE")
                .count();
            let max_natives = 10; // Expected max locales
            let relationship_power = ((native_count * 100) / max_natives).min(100) as u8;

            // Extract entity_slug from denomination_forms
            let entity_slug = props
                .get("denomination_forms")
                .and_then(|df| df.as_array())
                .and_then(|arr| {
                    arr.iter()
                        .find(|form| form.get("type").and_then(|t| t.as_str()) == Some("url"))
                        .and_then(|form| form.get("value").and_then(|v| v.as_str()))
                        .map(|s| s.to_string())
                });

            instances.push(InstanceInfo {
                key,
                display_name,
                class_key: class_label.to_string(),
                properties: props,
                outgoing_arcs,
                incoming_arcs,
                arcs_loading: false,       // Arcs already loaded in full query
                missing_required_count: 0, // Calculated later in set_instances
                filled_properties: 0,      // Calculated later in set_instances
                total_properties: 0,       // Calculated later in set_instances
                entity_slug,
                relationship_power,
            });
        }

        Ok((instances, total_count))
    }

    /// Load instances FAST - only keys + display_name, NO arc queries.
    /// This returns in ~50ms instead of ~3s for large datasets.
    /// Arcs should be loaded separately via `load_instance_arcs()`.
    pub async fn load_instances_fast(
        db: &Db,
        class_label: &str,
    ) -> crate::Result<(Vec<InstanceInfo>, usize)> {
        // Security: Validate label before interpolation into Cypher
        validate_cypher_label(class_label)?;

        // Single fast query: get keys + display_name + basic props (no arc traversal)
        let cypher = format!(
            r#"
MATCH (n:{class_label})
WHERE NOT n:Schema
WITH count(n) AS total
MATCH (n:{class_label})
WHERE NOT n:Schema
WITH total, n
ORDER BY n.key
LIMIT {limit}
RETURN
    total,
    coalesce(n.key, n.label, toString(id(n))) AS key,
    coalesce(n.display_name, n.key, n.label) AS display_name,
    properties(n) AS props
"#,
            class_label = class_label,
            limit = INSTANCE_LIMIT
        );

        let rows = db.execute(&cypher).await?;
        let mut instances = Vec::with_capacity(rows.len());
        let mut total_count = 0usize;

        for row in rows {
            // Get total from first row
            if total_count == 0 {
                total_count = row.get::<i64>("total").unwrap_or(0) as usize;
            }

            let key: String = row.get("key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();

            // Parse properties
            let props: BTreeMap<String, JsonValue> = row
                .get::<neo4rs::BoltMap>("props")
                .map(|m| {
                    m.value
                        .iter()
                        .map(|(k, v)| (k.value.clone(), bolt_to_json(v)))
                        .collect()
                })
                .unwrap_or_default();

            // Extract entity_slug from denomination_forms
            let entity_slug = props
                .get("denomination_forms")
                .and_then(|df| df.as_array())
                .and_then(|arr| {
                    arr.iter()
                        .find(|form| form.get("type").and_then(|t| t.as_str()) == Some("url"))
                        .and_then(|form| form.get("value").and_then(|v| v.as_str()))
                        .map(|s| s.to_string())
                });

            instances.push(InstanceInfo {
                key,
                display_name,
                class_key: class_label.to_string(),
                properties: props,
                outgoing_arcs: Vec::new(), // Empty - will be loaded separately
                incoming_arcs: Vec::new(), // Empty - will be loaded separately
                arcs_loading: true,        // Mark as loading
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
                entity_slug,
                relationship_power: 0, // Will be calculated when arcs are loaded
            });
        }

        Ok((instances, total_count))
    }

    /// Load arcs for a batch of instance keys.
    /// Called AFTER `load_instances_fast()` to populate arc data in background.
    pub async fn load_instance_arcs(
        db: &Db,
        class_label: &str,
        keys: Vec<String>,
    ) -> crate::Result<FxHashMap<String, (Vec<InstanceArc>, Vec<InstanceArc>)>> {
        if keys.is_empty() {
            return Ok(FxHashMap::default());
        }

        // Security: Validate label
        validate_cypher_label(class_label)?;

        let cypher = format!(
            r#"
UNWIND $keys AS k
MATCH (n:{class_label} {{key: k}})
OPTIONAL MATCH (n)-[out]->(target)
WHERE NOT target:Schema
WITH n, k, collect(DISTINCT {{
    arc_type: type(out),
    target_key: coalesce(target.key, target.label, id(target)),
    target_class: head(labels(target)),
    target_display_name: target.display_name,
    target_slug: null
}}) AS outgoing
OPTIONAL MATCH (source)-[inc]->(n)
WHERE NOT source:Schema
WITH n, k, outgoing, collect(DISTINCT {{
    arc_type: type(inc),
    source_key: coalesce(source.key, source.label, id(source)),
    source_class: head(labels(source)),
    source_display_name: source.display_name,
    source_slug: null
}}) AS incoming
RETURN k AS key, outgoing, incoming
"#,
            class_label = class_label
        );

        let rows = db.execute_with_params(&cypher, [("keys", keys)]).await?;
        let mut result = FxHashMap::default();

        for row in rows {
            let key: String = row.get("key").unwrap_or_default();

            // Parse outgoing arcs
            let outgoing_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("outgoing")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("target_key").unwrap_or_default(),
                        target_class: m.get("target_class").unwrap_or_default(),
                        exists: true,
                        target_display_name: m.get("target_display_name").ok(),
                        target_slug: m.get("target_slug").ok(),
                    })
                })
                .collect();

            // Parse incoming arcs
            let incoming_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("incoming")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("source_key").unwrap_or_default(),
                        target_class: m.get("source_class").unwrap_or_default(),
                        exists: true,
                        target_display_name: m.get("source_display_name").ok(),
                        target_slug: m.get("source_slug").ok(),
                    })
                })
                .collect();

            result.insert(key, (outgoing_arcs, incoming_arcs));
        }

        Ok(result)
    }
}
