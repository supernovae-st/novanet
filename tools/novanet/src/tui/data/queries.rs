//! Neo4j query methods for TUI data loading.
//!
//! Split into sibling submodules by domain:
//! - `queries_details`: Schema detail queries (stats, class arcs, arc/realm/layer details)
//! - `queries_instances`: Generic instance loading (full, fast, background arcs)
//! - `queries_entities`: Entity category hierarchy (categories, by-category, natives)
//!
//! This hub keeps the main `load()` entry point and its helpers.

use crate::db::{Db, RowExt};
use crate::parsers::taxonomy::{TaxonomyDoc, load_taxonomy_from_files};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeMap;
use std::path::Path;
use tokio::join;

use super::conversion::*;
use super::types::*;
use super::TaxonomyTree;

impl TaxonomyTree {
    /// Load taxonomy tree from Neo4j, enriched with content from individual YAML files.
    pub async fn load(db: &Db, root: &Path) -> crate::Result<Self> {
        // Load from individual YAML files for content enrichment
        let taxonomy = load_taxonomy_from_files(root).ok();

        // Build lookup maps for realm/layer content
        let (realm_content, layer_content, arc_family_content) =
            Self::build_content_maps(&taxonomy);

        // Query all Classes with their realm, layer, and instance count
        // Note: Class uses 'label' property as identifier, not 'key'
        // Count by label match instead of OF_CLASS (which only exists for Locale)
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
            // knowledge_tier (optional, only for knowledge-layer nodes)
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
}
