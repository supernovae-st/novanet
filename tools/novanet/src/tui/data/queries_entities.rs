//! Entity category hierarchy queries for the TUI.
//!
//! Loads EntityCategory nodes, Entity instances by category,
//! and EntityNative groups by parent Entity.

use crate::db::Db;
use rustc_hash::FxHashMap;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

use super::conversion::*;
use super::types::*;
use super::TaxonomyTree;

impl TaxonomyTree {
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
        // Use OPTIONAL MATCH to handle categories with 0 entities
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
            let props = parse_bolt_props(&row);
            let outgoing_arcs = parse_outgoing_arcs(&row);
            let incoming_arcs = parse_incoming_arcs(&row);
            let relationship_power = relationship_power_from_arcs(&outgoing_arcs);
            let entity_slug = extract_entity_slug(&props);

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
        // Use APOC to parse denomination_forms JSON string at query time
        // Also load all properties for INSTANCE panel display
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
            // Include all properties for INSTANCE panel display
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
