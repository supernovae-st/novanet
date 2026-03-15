//! Neo4j query methods for TUI data loading.
//!
//! All async methods that fetch data from the database for the taxonomy tree.

use crate::db::{Db, RowExt};
use crate::parsers::taxonomy::{TaxonomyDoc, load_taxonomy_from_files};
use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::path::Path;
use tokio::join;

use super::conversion::*;
use super::types::*;
use super::TaxonomyTree;

impl TaxonomyTree {
    /// Load taxonomy tree from Neo4j, enriched with content from individual YAML files.
    pub async fn load(db: &Db, root: &Path) -> crate::Result<Self> {
        // v0.12.5: Load from individual YAML files for content enrichment
        let taxonomy = load_taxonomy_from_files(root).ok();

        // Build lookup maps for realm/layer content
        let (realm_content, layer_content, arc_family_content) =
            Self::build_content_maps(&taxonomy);

        // Query all Classes with their realm, layer, and instance count
        // Note: Class uses 'label' property as identifier, not 'key'
        // v0.16.4: Count by label match instead of OF_CLASS (which only exists for Locale)
        let cypher = r#"
MATCH (k:Class:Schema)
OPTIONAL MATCH (k)-[:IN_REALM]->(r:Realm)
OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer)
OPTIONAL MATCH (n) WHERE labels(n)[0] = k.label AND NOT n:Schema
WITH k, r, l, count(n) AS instances
RETURN
    k.label AS class_key,
    coalesce(k.display_name, k.label) AS class_display,
    coalesce(k.content, '') AS class_desc,
    coalesce(k.icon, '') AS class_icon,
    coalesce(r.key, 'unknown') AS realm_key,
    coalesce(r.display_name, r.key, 'Unknown') AS realm_display,
    coalesce(r.color, '#ffffff') AS realm_color,
    coalesce(l.key, 'unknown') AS layer_key,
    coalesce(l.display_name, l.key, 'Unknown') AS layer_display,
    coalesce(l.color, '#ffffff') AS layer_color,
    instances,
    coalesce(k.yaml_path, '') AS yaml_path,
    coalesce(k.properties, []) AS properties,
    coalesce(k.required_properties, []) AS required_properties,
    coalesce(k.schema_hint, '') AS schema_hint,
    coalesce(k.context_budget, '') AS context_budget,
    k.knowledge_tier AS knowledge_tier
ORDER BY realm_key, layer_key, class_key
"#;

        let rows = db.execute(cypher).await?;

        // Group into tree structure: realm_key -> (realm_display, realm_color, layer_key -> (layer_display, layer_color, classes))
        #[allow(clippy::type_complexity)]
        let mut realm_map: BTreeMap<
            String,
            (
                String,
                String,
                BTreeMap<String, (String, String, Vec<ClassInfo>)>,
            ),
        > = BTreeMap::new();

        for row in rows {
            // Extract fields using RowExt for ergonomic defaults
            let class_key = row.str("class_key");
            let class_display = row.str("class_display");
            let class_desc = row.str("class_desc");
            let class_icon = row.str("class_icon");
            let realm_key = row.str("realm_key");
            let realm_display = row.str("realm_display");
            let realm_color = row.str("realm_color");
            let layer_key = row.str("layer_key");
            let layer_display = row.str("layer_display");
            let layer_color = row.str("layer_color");
            let instances = row.int("instances");

            // Get YAML path from Neo4j (with fallback to computed path)
            let yaml_path_raw = row.str("yaml_path");
            let yaml_path = if !yaml_path_raw.is_empty() {
                // Neo4j stores relative path like "node-classes/org/structure/block.yaml"
                // We need to prefix with "packages/core/models/"
                format!("packages/core/models/{}", yaml_path_raw)
            } else if realm_key == "unknown" || layer_key == "unknown" {
                // Missing realm/layer relationship - can't compute valid path
                // Return empty to signal "file not found" in UI (better than invalid path)
                String::new()
            } else {
                // Fallback: compute path from realm/layer
                format!(
                    "packages/core/models/node-classes/{}/{}/{}.yaml",
                    realm_key,
                    layer_key,
                    to_kebab_case(&class_key)
                )
            };

            // Get schema properties from Neo4j
            let properties = row.vec_str("properties");
            let required_properties = row.vec_str("required_properties");
            let schema_hint = row.str("schema_hint");
            let context_budget = row.str("context_budget");
            // v10: knowledge_tier (optional, only for knowledge-layer nodes)
            let knowledge_tier = row.opt_str("knowledge_tier");

            let class_info = ClassInfo {
                key: class_key,
                display_name: class_display,
                description: class_desc,
                icon: class_icon,
                instance_count: instances,
                arcs: Vec::new(), // Loaded separately
                yaml_path,
                properties,
                required_properties,
                schema_hint,
                context_budget,
                knowledge_tier,
                // Health stats (not loaded yet, requires separate query)
                health_percent: None,
                issues_count: None,
            };

            realm_map
                .entry(realm_key)
                .or_insert_with(|| (realm_display, realm_color, BTreeMap::new()))
                .2
                .entry(layer_key)
                .or_insert_with(|| (layer_display, layer_color, Vec::new()))
                .2
                .push(class_info);
        }

        // Convert to RealmInfo vec with content from realms/*.yaml
        let realms: Vec<RealmInfo> = realm_map
            .into_iter()
            .map(|(realm_key, (realm_display, realm_color, layers_map))| {
                let layers: Vec<LayerInfo> = layers_map
                    .into_iter()
                    .map(|(layer_key, (layer_display, layer_color, classes))| {
                        // Look up content from layers/*.yaml
                        let content_val = layer_content
                            .get(&layer_key)
                            .cloned()
                            .unwrap_or_default();
                        LayerInfo {
                            key: layer_key,
                            display_name: layer_display,
                            color: layer_color,
                            classes,
                            content: content_val,
                        }
                    })
                    .collect();

                // Look up realm content from realms/*.yaml
                let realm_content_val = realm_content
                    .get(&realm_key)
                    .cloned()
                    .unwrap_or_default();
                RealmInfo {
                    icon: realm_icon(&realm_key),
                    key: realm_key.clone(),
                    display_name: realm_display,
                    color: realm_color,
                    layers,
                    content: realm_content_val,
                }
            })
            .collect();

        // Load stats, arcs, and families in parallel (~3x faster startup)
        let (stats_result, arcs_result, families_result) = join!(
            Self::load_stats(db),
            Self::fetch_arcs(db),
            Self::fetch_arc_families(db)
        );

        let stats = stats_result?;
        let arc_map = arcs_result.unwrap_or_default();
        // Enrich arc_families with content from arc-families/*.yaml
        let arc_families = Self::enrich_arc_families_with_content(
            families_result.unwrap_or_default(),
            &arc_family_content,
        );

        // Apply arcs to classes
        let realms = Self::apply_arcs_to_realms(realms, arc_map);

        // Build class_index for O(1) lookups (replaces O(n*m*k) find_class)
        let mut class_index = FxHashMap::default();
        for (r_idx, realm) in realms.iter().enumerate() {
            for (l_idx, layer) in realm.layers.iter().enumerate() {
                for (k_idx, class_info) in layer.classes.iter().enumerate() {
                    class_index.insert(class_info.key.clone(), (r_idx, l_idx, k_idx));
                }
            }
        }

        Ok(Self {
            realms,
            arc_families,
            stats,
            collapsed: FxHashSet::default(),
            instances: FxHashMap::default(),
            instance_totals: FxHashMap::default(),
            class_index,
            entity_categories: Vec::new(), // Loaded on-demand via load_entity_categories
            entity_category_instances: FxHashMap::default(), // Loaded on-demand when category expanded
            locale_groups: Vec::new(), // Loaded on-demand via load_entity_natives_by_locale
            entity_native_by_locale: FxHashMap::default(), // Loaded on-demand when locale expanded
            entity_native_groups: Vec::new(), // Loaded on-demand via load_entity_natives_by_entity
            entity_native_by_entity: FxHashMap::default(), // Loaded on-demand when entity group expanded
        })
    }

    /// Build lookup maps for content from individual YAML files.
    /// Returns (realm_content, layer_content, arc_family_content).
    fn build_content_maps(
        taxonomy: &Option<TaxonomyDoc>,
    ) -> (
        FxHashMap<String, String>,
        FxHashMap<String, String>,
        FxHashMap<String, String>,
    ) {
        let mut realm_map = FxHashMap::default();
        let mut layer_map = FxHashMap::default();
        let mut arc_family_map = FxHashMap::default();

        if let Some(tax) = taxonomy {
            // Extract realm content
            for realm in &tax.node_realms {
                realm_map.insert(realm.key.clone(), realm.content.clone());
                // Extract layer content (nested under realm)
                for layer in &realm.layers {
                    layer_map.insert(layer.key.clone(), layer.content.clone());
                }
            }

            // Extract arc_family content from triggers (taxonomy only has triggers, not content)
            for family in &tax.arc_families {
                arc_family_map.insert(family.key.clone(), family.triggers.join(", "));
            }
        }

        (realm_map, layer_map, arc_family_map)
    }

    /// Enrich arc_families with content from arc-families/*.yaml lookup map.
    fn enrich_arc_families_with_content(
        mut families: Vec<ArcFamilyInfo>,
        content_map: &FxHashMap<String, String>,
    ) -> Vec<ArcFamilyInfo> {
        for family in &mut families {
            if let Some(content_val) = content_map.get(&family.key) {
                family.content = content_val.clone();
            }
        }
        families
    }

    /// Apply arc map to realm/layer/class tree.
    fn apply_arcs_to_realms(
        mut realms: Vec<RealmInfo>,
        mut arc_map: BTreeMap<String, Vec<ArcInfo>>,
    ) -> Vec<RealmInfo> {
        for realm in &mut realms {
            for layer in &mut realm.layers {
                for class_info in &mut layer.classes {
                    if let Some(arcs) = arc_map.remove(&class_info.key) {
                        class_info.arcs = arcs;
                    }
                }
            }
        }
        realms
    }

    /// Fetch arcs as a map (for parallel loading).
    async fn fetch_arcs(db: &Db) -> crate::Result<BTreeMap<String, Vec<ArcInfo>>> {
        let cypher = r#"
MATCH (ak:ArcClass:Schema)-[:FROM_CLASS]->(fromClass:Class:Schema)
MATCH (ak)-[:TO_CLASS]->(toClass:Class:Schema)
RETURN fromClass.label AS class_key, ak.key AS arc_type, 'outgoing' AS direction, toClass.label AS target_class
ORDER BY fromClass.label, ak.key

UNION

MATCH (ak:ArcClass:Schema)-[:FROM_CLASS]->(fromClass:Class:Schema)
MATCH (ak)-[:TO_CLASS]->(toClass:Class:Schema)
RETURN toClass.label AS class_key, ak.key AS arc_type, 'incoming' AS direction, fromClass.label AS target_class
ORDER BY toClass.label, ak.key
"#;

        let rows = db.execute(cypher).await?;
        let mut arc_map: BTreeMap<String, Vec<ArcInfo>> = BTreeMap::new();

        for row in rows {
            let class_key = row.str("class_key");
            let arc_type = row.str("arc_type");
            let direction_str = row.str("direction");
            let target_class = row.str("target_class");

            if class_key.is_empty() || arc_type.is_empty() {
                continue;
            }

            let direction = if direction_str == "incoming" {
                ArcDirection::Incoming
            } else {
                ArcDirection::Outgoing
            };

            arc_map.entry(class_key).or_default().push(ArcInfo {
                arc_type,
                direction,
                target_class,
            });
        }

        Ok(arc_map)
    }

    /// Fetch arc families (for parallel loading).
    async fn fetch_arc_families(db: &Db) -> crate::Result<Vec<ArcFamilyInfo>> {
        let cypher = r#"
MATCH (ak:ArcClass:Schema)-[:IN_FAMILY]->(af:ArcFamily:Schema)
MATCH (ak)-[:FROM_CLASS]->(fromClass:Class:Schema)
MATCH (ak)-[:TO_CLASS]->(toClass:Class:Schema)
RETURN
    af.key AS family_key,
    coalesce(af.display_name, af.key) AS family_display,
    ak.key AS arc_key,
    coalesce(ak.display_name, ak.key) AS arc_display,
    coalesce(ak.cardinality, '') AS cardinality,
    coalesce(ak.content, '') AS arc_desc,
    fromClass.label AS from_class,
    toClass.label AS to_class
ORDER BY family_key, arc_key
"#;

        let rows = db.execute(cypher).await?;
        let mut family_map: BTreeMap<String, (String, Vec<ArcClassInfo>)> = BTreeMap::new();

        for row in rows {
            let family_key = row.str("family_key");
            let family_display = row.str("family_display");
            let arc_key = row.str("arc_key");
            let arc_display = row.str("arc_display");
            let cardinality = row.str("cardinality");
            let arc_desc = row.str("arc_desc");
            let from_class = row.str("from_class");
            let to_class = row.str("to_class");

            if family_key.is_empty() || arc_key.is_empty() {
                continue;
            }

            let arc_class = ArcClassInfo {
                key: arc_key,
                display_name: arc_display,
                from_class,
                to_class,
                cardinality,
                description: arc_desc,
            };

            family_map
                .entry(family_key)
                .or_insert_with(|| (family_display, Vec::new()))
                .1
                .push(arc_class);
        }

        Ok(family_map
            .into_iter()
            .map(|(key, (display_name, arc_classes))| ArcFamilyInfo {
                key,
                display_name,
                arc_classes,
                content: String::new(), // Enriched later from individual YAML files
            })
            .collect())
    }

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

    /// Load graph statistics.
    async fn load_stats(db: &Db) -> crate::Result<GraphStats> {
        let cypher = r#"
MATCH (n) WHERE NOT n:Schema
WITH count(n) AS nodes
MATCH ()-[r]->() WHERE NOT startNode(r):Schema AND NOT endNode(r):Schema
WITH nodes, count(r) AS arcs
MATCH (k:Class:Schema)
WITH nodes, arcs, count(k) AS classes
MATCH (ak:ArcClass:Schema)
RETURN nodes, arcs, classes, count(ak) AS arc_classes
"#;

        let rows = db.execute(cypher).await?;
        if let Some(row) = rows.into_iter().next() {
            Ok(GraphStats {
                node_count: row.get("nodes").unwrap_or(0),
                arc_count: row.get("arcs").unwrap_or(0),
                class_count: row.get("classes").unwrap_or(0),
                arc_class_count: row.get("arc_classes").unwrap_or(0),
            })
        } else {
            Ok(GraphStats::default())
        }
    }

    /// Load arc relationships for a Class from Neo4j.
    /// Returns incoming and outgoing arcs with their families.
    pub async fn load_class_arcs(db: &Db, class_label: &str) -> crate::Result<ClassArcsData> {
        let cypher = r#"
MATCH (c:Class {label: $classLabel})
OPTIONAL MATCH (c)-[:IN_LAYER]->(l:Layer)
OPTIONAL MATCH (l)<-[:HAS_LAYER]-(r:Realm)
OPTIONAL MATCH (c)<-[:TO_CLASS]-(inArc:ArcClass)-[:FROM_CLASS]->(fromClass:Class)
OPTIONAL MATCH (inArc)-[:IN_FAMILY]->(inFamily:ArcFamily)
OPTIONAL MATCH (c)<-[:FROM_CLASS]-(outArc:ArcClass)-[:TO_CLASS]->(toClass:Class)
OPTIONAL MATCH (outArc)-[:IN_FAMILY]->(outFamily:ArcFamily)
WITH c, r, l,
     collect(DISTINCT CASE WHEN inArc IS NOT NULL
         THEN {arc: inArc.key, from: fromClass.label, family: inFamily.key} END) as incoming,
     collect(DISTINCT CASE WHEN outArc IS NOT NULL
         THEN {arc: outArc.key, to: toClass.label, family: outFamily.key} END) as outgoing
RETURN c.label as class,
       r.key as realm,
       l.key as layer,
       [x IN incoming WHERE x IS NOT NULL] as incoming,
       [x IN outgoing WHERE x IS NOT NULL] as outgoing
LIMIT 1
"#;

        let rows = db
            .execute_with_params(cypher, [("classLabel", class_label)])
            .await?;

        if let Some(row) = rows.into_iter().next() {
            let class: String = row.get("class").unwrap_or_default();
            let realm: String = row.get("realm").unwrap_or_default();
            let layer: String = row.get("layer").unwrap_or_default();

            // Parse incoming arcs
            let incoming_raw: Vec<neo4rs::BoltMap> = row
                .get::<Vec<neo4rs::BoltMap>>("incoming")
                .unwrap_or_default();
            let incoming: Vec<Neo4jArc> = incoming_raw
                .into_iter()
                .filter_map(|m| {
                    let arc_key = m.get::<String>("arc").ok()?;
                    let other_class = m.get::<String>("from").ok()?;
                    let family = m.get::<String>("family").ok().unwrap_or_default();
                    Some(Neo4jArc {
                        arc_key,
                        other_class,
                        family,
                    })
                })
                .collect();

            // Parse outgoing arcs
            let outgoing_raw: Vec<neo4rs::BoltMap> = row
                .get::<Vec<neo4rs::BoltMap>>("outgoing")
                .unwrap_or_default();
            let outgoing: Vec<Neo4jArc> = outgoing_raw
                .into_iter()
                .filter_map(|m| {
                    let arc_key = m.get::<String>("arc").ok()?;
                    let other_class = m.get::<String>("to").ok()?;
                    let family = m.get::<String>("family").ok().unwrap_or_default();
                    Some(Neo4jArc {
                        arc_key,
                        other_class,
                        family,
                    })
                })
                .collect();

            Ok(ClassArcsData {
                class_label: class,
                realm,
                layer,
                incoming,
                outgoing,
            })
        } else {
            Ok(ClassArcsData::default())
        }
    }

    /// Load ArcClass details from Neo4j (endpoints, family, cardinality).
    pub async fn load_arc_class_details(db: &Db, arc_key: &str) -> crate::Result<ArcClassDetails> {
        let cypher = r#"
MATCH (ac:ArcClass {key: $arcKey})
OPTIONAL MATCH (ac)-[:IN_FAMILY]->(af:ArcFamily)
OPTIONAL MATCH (ac)-[:FROM_CLASS]->(fromClass:Class)
OPTIONAL MATCH (fromClass)-[:IN_LAYER]->(fromLayer:Layer)
OPTIONAL MATCH (fromLayer)<-[:HAS_LAYER]-(fromRealm:Realm)
OPTIONAL MATCH (ac)-[:TO_CLASS]->(toClass:Class)
OPTIONAL MATCH (toClass)-[:IN_LAYER]->(toLayer:Layer)
OPTIONAL MATCH (toLayer)<-[:HAS_LAYER]-(toRealm:Realm)
RETURN coalesce(ac.display_name, ac.key) as display_name,
       coalesce(ac.content, '') as description,
       coalesce(ac.cardinality, '') as cardinality,
       coalesce(ac.cypher_pattern, '') as cypher_pattern,
       coalesce(af.key, '') as family,
       fromClass.label as from_class,
       coalesce(fromRealm.key, '') as from_realm,
       coalesce(fromLayer.key, '') as from_layer,
       toClass.label as to_class,
       coalesce(toRealm.key, '') as to_realm,
       coalesce(toLayer.key, '') as to_layer
LIMIT 1
"#;

        let rows = db
            .execute_with_params(cypher, [("arcKey", arc_key)])
            .await?;

        if let Some(row) = rows.into_iter().next() {
            let display_name: String = row.get("display_name").unwrap_or_default();
            let description: String = row.get("description").unwrap_or_default();
            let cardinality: String = row.get("cardinality").unwrap_or_default();
            let cypher_pattern: String = row.get("cypher_pattern").unwrap_or_default();
            let family: String = row.get("family").unwrap_or_default();

            let from_class: Option<String> = row.get("from_class").ok();
            let from_realm: String = row.get("from_realm").unwrap_or_default();
            let from_layer: String = row.get("from_layer").unwrap_or_default();

            let to_class: Option<String> = row.get("to_class").ok();
            let to_realm: String = row.get("to_realm").unwrap_or_default();
            let to_layer: String = row.get("to_layer").unwrap_or_default();

            let from_endpoint = from_class.map(|class_label| ArcEndpoint {
                class_label,
                realm: from_realm,
                layer: from_layer,
            });

            let to_endpoint = to_class.map(|class_label| ArcEndpoint {
                class_label,
                realm: to_realm,
                layer: to_layer,
            });

            Ok(ArcClassDetails {
                display_name,
                description,
                family,
                cardinality,
                cypher_pattern,
                from_endpoint,
                to_endpoint,
            })
        } else {
            Ok(ArcClassDetails::default())
        }
    }

    /// Load Realm details from Neo4j (layers with class counts, total stats).
    pub async fn load_realm_details(db: &Db, realm_key: &str) -> crate::Result<RealmDetails> {
        // Query 1: Get realm info and totals
        let cypher_realm = r#"
MATCH (r:Realm {key: $realmKey})
OPTIONAL MATCH (r)-[:HAS_LAYER]->(l:Layer)<-[:IN_LAYER]-(c:Class)
OPTIONAL MATCH (c)<-[:OF_CLASS]-(n)
RETURN r.key as realm_key,
       coalesce(r.display_name, r.key) as display_name,
       coalesce(r.content, '') as description,
       count(DISTINCT c) as total_classes,
       count(DISTINCT n) as total_instances
"#;

        // Query 2: Get layers with their class counts (separate rows)
        let cypher_layers = r#"
MATCH (r:Realm {key: $realmKey})-[:HAS_LAYER]->(l:Layer)
OPTIONAL MATCH (l)<-[:IN_LAYER]-(c:Class)
WITH l, count(DISTINCT c) as class_count
ORDER BY l.order
RETURN l.key as layer_key,
       coalesce(l.display_name, l.key) as layer_display,
       class_count
"#;

        // Execute both queries in parallel using tokio::join!
        let (realm_result, layers_result) = tokio::join!(
            db.execute_with_params(cypher_realm, [("realmKey", realm_key)]),
            db.execute_with_params(cypher_layers, [("realmKey", realm_key)]),
        );

        let realm_rows = realm_result?;
        let layer_rows = layers_result?;

        if let Some(row) = realm_rows.into_iter().next() {
            let key: String = row.get("realm_key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();
            let description: String = row.get("description").unwrap_or_default();
            let total_classes: i64 = row.get("total_classes").unwrap_or(0);
            let total_instances: i64 = row.get("total_instances").unwrap_or(0);

            let layers: Vec<LayerStats> = layer_rows
                .into_iter()
                .map(|lr| LayerStats {
                    key: lr.get("layer_key").unwrap_or_default(),
                    display_name: lr.get("layer_display").unwrap_or_default(),
                    class_count: lr.get::<i64>("class_count").unwrap_or(0) as usize,
                })
                .collect();

            Ok(RealmDetails {
                key,
                display_name,
                description,
                layers,
                total_classes: total_classes as usize,
                total_instances: total_instances as usize,
            })
        } else {
            Ok(RealmDetails::default())
        }
    }

    /// Load Layer details from Neo4j (classes, stats).
    /// Simplified - returns classes with instance counts.
    pub async fn load_layer_details(db: &Db, layer_key: &str) -> crate::Result<LayerDetails> {
        let cypher = r#"
MATCH (l:Layer {key: $layerKey})
OPTIONAL MATCH (r:Realm)-[:HAS_LAYER]->(l)
OPTIONAL MATCH (l)<-[:IN_LAYER]-(c:Class)
OPTIONAL MATCH (n) WHERE labels(n)[0] = c.label AND NOT n:Schema
WITH l, r, c, count(DISTINCT n) as inst_count
ORDER BY c.label
WITH l, r, collect(coalesce(c.display_name, c.label)) as class_names, count(c) as total_classes, sum(inst_count) as total_instances
RETURN l.key as layer_key,
       coalesce(l.display_name, l.key) as display_name,
       coalesce(l.content, '') as description,
       coalesce(r.key, '') as realm,
       class_names,
       total_classes,
       total_instances
"#;

        let rows = db
            .execute_with_params(cypher, [("layerKey", layer_key)])
            .await?;

        if let Some(row) = rows.into_iter().next() {
            let key: String = row.get("layer_key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();
            let description: String = row.get("description").unwrap_or_default();
            let realm: String = row.get("realm").unwrap_or_default();
            let total_classes: i64 = row.get("total_classes").unwrap_or(0);
            let total_instances: i64 = row.get("total_instances").unwrap_or(0);

            let class_names: Vec<String> = row.get("class_names").unwrap_or_default();

            Ok(LayerDetails {
                key,
                display_name,
                description,
                realm,
                class_names,
                total_classes: total_classes as usize,
                total_instances: total_instances as usize,
            })
        } else {
            Ok(LayerDetails::default())
        }
    }

    // ========================================================================
    // Entity Category Hierarchy (Data mode)
    // ========================================================================

    /// Load all EntityCategory nodes from Neo4j with instance counts.
    /// Returns categories sorted by sort_order for display in Data mode.
    pub async fn load_entity_categories(db: &Db) -> crate::Result<Vec<EntityCategory>> {
        let cypher = r#"
MATCH (c:EntityCategory)
OPTIONAL MATCH (e:Entity)-[:BELONGS_TO]->(c)
WITH c, count(e) AS instance_count
RETURN c.key AS key,
       coalesce(c.display_name, c.key) AS display_name,
       coalesce(c.sort_order, 0) AS sort_order,
       coalesce(c.question, '') AS question,
       coalesce(c.content, '') AS content,
       instance_count
ORDER BY c.sort_order, c.key
"#;

        let rows = db.execute(cypher).await?;
        let mut categories = Vec::with_capacity(rows.len());

        for row in rows {
            categories.push(EntityCategory {
                key: row.get("key").unwrap_or_default(),
                display_name: row.get("display_name").unwrap_or_default(),
                sort_order: row.get("sort_order").unwrap_or(0),
                question: row.get("question").unwrap_or_default(),
                content: row.get("content").unwrap_or_default(),
                instance_count: row.get("instance_count").unwrap_or(0),
            });
        }

        Ok(categories)
    }

    /// Load Entity instances that belong to a specific EntityCategory.
    /// Returns (instances, total_count) for pagination display.
    ///
    /// Uses the BELONGS_TO arc: Entity -[:BELONGS_TO]-> EntityCategory
    /// This enables Data mode to show Entity instances grouped by category.
    pub async fn load_entities_by_category(
        db: &Db,
        category_key: &str,
    ) -> crate::Result<(Vec<InstanceInfo>, i64)> {
        // v0.17.3: Use OPTIONAL MATCH to handle categories with 0 entities
        // Query returns empty result set if category has no entities (handled in Rust)
        let cypher = r#"
MATCH (c:EntityCategory {key: $category})
OPTIONAL MATCH (e:Entity)-[:BELONGS_TO]->(c)
WITH c, e WHERE e IS NOT NULL
ORDER BY e.display_name, e.key
LIMIT 1000
WITH collect(e) AS entities
WITH entities, size(entities) AS total
UNWIND entities AS e
OPTIONAL MATCH (e)-[out]->(target)
WHERE NOT target:Schema
WITH total, e, collect(DISTINCT {
    arc_type: type(out),
    target_key: coalesce(target.key, target.label, toString(id(target))),
    target_class: head(labels(target)),
    target_display_name: target.display_name,
    target_slug: null
}) AS outgoing
OPTIONAL MATCH (source)-[inc]->(e)
WHERE NOT source:Schema
WITH total, e, outgoing, collect(DISTINCT {
    arc_type: type(inc),
    source_key: coalesce(source.key, source.label, toString(id(source))),
    source_class: head(labels(source)),
    source_display_name: source.display_name,
    source_slug: null
}) AS incoming
RETURN total,
       coalesce(e.key, toString(id(e))) AS key,
       coalesce(e.display_name, e.key) AS display_name,
       properties(e) AS props,
       outgoing,
       incoming
"#;

        let rows = db
            .execute_with_params(cypher, [("category", category_key)])
            .await?;

        // Get total count from first row (all rows have same total)
        let total_count: i64 = rows.first().and_then(|r| r.get("total").ok()).unwrap_or(0);

        let mut instances = Vec::with_capacity(rows.len());

        for row in rows {
            let key: String = row.get("key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();

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
                class_key: "Entity".to_string(),
                properties: props,
                outgoing_arcs,
                incoming_arcs,
                arcs_loading: false,
                missing_required_count: 0, // Calculated later if needed
                filled_properties: 0,      // Calculated later if needed
                total_properties: 0,       // Calculated later if needed
                entity_slug,
                relationship_power,
            });
        }

        Ok((instances, total_count))
    }

    /// Load all EntityNative instances grouped by locale.
    /// Returns locale groups sorted by locale code, with natives sorted A-Z by entity_display_name.
    pub async fn load_entity_natives_by_locale(
        db: &Db,
    ) -> crate::Result<(Vec<LocaleGroup>, FxHashMap<String, Vec<EntityNativeInfo>>)> {
        // v0.17.3: Use APOC to parse denomination_forms JSON string at query time
        let cypher = r#"
MATCH (en:EntityNative)
OPTIONAL MATCH (en)-[:FOR_LOCALE]->(l:Locale)
OPTIONAL MATCH (e:Entity)-[:HAS_NATIVE]->(en)
WITH coalesce(l.key, 'unknown') AS locale_code,
     coalesce(l.display_name, l.key, 'Unknown Locale') AS locale_name,
     en, e
ORDER BY coalesce(e.display_name, e.key, en.key)
WITH locale_code, locale_name,
     collect({
         key: en.key,
         display_name: coalesce(en.display_name, en.key),
         entity_key: coalesce(e.key, ''),
         entity_display_name: coalesce(e.display_name, e.key, ''),
         slug: CASE
           WHEN en.denomination_forms IS NOT NULL
           THEN [form IN apoc.convert.fromJsonList(en.denomination_forms) WHERE form.type = 'url' | form.value][0]
           ELSE null
         END
     }) AS natives
RETURN locale_code, locale_name, natives, size(natives) AS count
ORDER BY locale_code
"#;

        let rows = db.execute(cypher).await?;
        let mut locale_groups = Vec::with_capacity(rows.len());
        let mut natives_by_locale: FxHashMap<String, Vec<EntityNativeInfo>> = FxHashMap::default();

        for row in rows {
            let locale_code: String = row.get("locale_code").unwrap_or_default();
            let locale_name: String = row.get("locale_name").unwrap_or_default();
            let count: i64 = row.get("count").unwrap_or(0);

            // Convert locale code to flag emoji
            let flag = locale_to_flag(&locale_code);

            locale_groups.push(LocaleGroup {
                locale_code: locale_code.clone(),
                locale_name,
                flag,
                instance_count: count,
            });

            // Parse natives
            // Note: This legacy function doesn't load full properties (use load_entity_natives_by_entity)
            let natives: Vec<EntityNativeInfo> = row
                .get::<Vec<neo4rs::BoltMap>>("natives")
                .unwrap_or_default()
                .into_iter()
                .map(|m| {
                    let slug: Option<String> = m.get("slug").ok();
                    // Power based on completeness: 100 if has slug, 50 otherwise
                    let relationship_power = if slug.is_some() { 100 } else { 50 };
                    EntityNativeInfo {
                        key: m.get("key").unwrap_or_default(),
                        display_name: m.get("display_name").unwrap_or_default(),
                        entity_key: m.get("entity_key").unwrap_or_default(),
                        entity_display_name: m.get("entity_display_name").unwrap_or_default(),
                        locale_code: locale_code.clone(),
                        slug,
                        relationship_power,
                        properties: BTreeMap::new(), // Legacy: no full properties in this query
                    }
                })
                .collect();

            natives_by_locale.insert(locale_code.clone(), natives);
        }

        Ok((locale_groups, natives_by_locale))
    }

    /// Load EntityNatives grouped by parent Entity.
    /// Returns groups (for tree nodes) and a map of entity_key -> natives.
    /// Each native includes locale_code and relationship_power for display.
    pub async fn load_entity_natives_by_entity(
        db: &Db,
    ) -> crate::Result<(
        Vec<EntityNativeGroup>,
        FxHashMap<String, Vec<EntityNativeInfo>>,
    )> {
        // Query EntityNatives grouped by parent Entity
        // v0.17.3: Use APOC to parse denomination_forms JSON string at query time
        // v0.17.3: Also load all properties for INSTANCE panel display
        let cypher = r#"
MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
OPTIONAL MATCH (en)-[:FOR_LOCALE]->(l:Locale)
WITH e, en, l
ORDER BY coalesce(l.key, 'zzz'), coalesce(en.display_name, en.key)
WITH e.key AS entity_key,
     coalesce(e.display_name, e.key) AS entity_display_name,
     collect({
         key: en.key,
         display_name: coalesce(en.display_name, en.key),
         locale_code: coalesce(l.key, en.locale_key, 'unknown'),
         slug: CASE
           WHEN en.denomination_forms IS NOT NULL
           THEN [form IN apoc.convert.fromJsonList(en.denomination_forms) WHERE form.type = 'url' | form.value][0]
           ELSE null
         END,
         props: properties(en)
     }) AS natives
RETURN entity_key, entity_display_name, natives, size(natives) AS count
ORDER BY entity_key
"#;

        let rows = db.execute(cypher).await?;
        let mut entity_groups = Vec::with_capacity(rows.len());
        let mut natives_by_entity: FxHashMap<String, Vec<EntityNativeInfo>> = FxHashMap::default();

        for row in rows {
            let entity_key: String = row.get("entity_key").unwrap_or_default();
            let entity_display_name: String = row.get("entity_display_name").unwrap_or_default();
            let count: i64 = row.get("count").unwrap_or(0);

            // Power based on native count: full if 5+, proportional otherwise
            let relationship_power = ((count.min(5) as f32 / 5.0) * 100.0) as u8;

            entity_groups.push(EntityNativeGroup {
                entity_key: entity_key.clone(),
                entity_display_name: entity_display_name.clone(),
                native_count: count as usize,
                relationship_power,
            });

            // Parse natives with slug from denomination_forms and full properties
            // v0.17.3: Include all properties for INSTANCE panel display
            let natives: Vec<EntityNativeInfo> = row
                .get::<Vec<neo4rs::BoltMap>>("natives")
                .unwrap_or_default()
                .into_iter()
                .map(|m| {
                    let slug: Option<String> = m.get("slug").ok();

                    // Extract nested properties map from the "props" field
                    let properties: BTreeMap<String, JsonValue> = m
                        .get::<neo4rs::BoltMap>("props")
                        .map(|props_map| {
                            props_map
                                .value
                                .iter()
                                .map(|(k, v)| (k.value.clone(), bolt_to_json(v)))
                                .collect()
                        })
                        .unwrap_or_default();

                    EntityNativeInfo {
                        key: m.get("key").unwrap_or_default(),
                        display_name: m.get("display_name").unwrap_or_default(),
                        entity_key: entity_key.clone(),
                        entity_display_name: entity_display_name.clone(),
                        locale_code: m.get("locale_code").unwrap_or_default(),
                        slug,
                        relationship_power: 80, // Power for EntityNative item
                        properties,
                    }
                })
                .collect();

            natives_by_entity.insert(entity_key.clone(), natives);
        }

        Ok((entity_groups, natives_by_entity))
    }
}
