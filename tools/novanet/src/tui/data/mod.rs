//! Data loading for TUI — Neo4j queries for taxonomy tree, stats, and detail.
//!
//! Split into submodules:
//! - `types`: All struct/enum definitions (ArcInfo, ClassInfo, TreeItem, etc.)
//! - `conversion`: Bolt-to-JSON conversion, label validation, formatting helpers
//! - `queries`: Main load() entry point + taxonomy tree assembly helpers
//! - `queries_details`: Schema detail queries (stats, class arcs, arc/realm/layer details)
//! - `queries_instances`: Generic instance loading (full, fast, background arcs)
//! - `queries_entities`: Entity category hierarchy (categories, by-category, natives)
//! - `tree_state`: Collapse/expand state management
//! - `cursor_finders`: Cursor position finders (realm, layer, class, family)
//! - `navigation`: Item counting, cursor lookup, parent finding, hierarchy position
//! - `entity_helpers`: Centralized Entity dual-storage helpers

mod conversion;
mod cursor_finders;
mod entity_helpers;
mod navigation;
mod queries;
mod queries_details;
mod queries_entities;
mod queries_instances;
mod tree_state;
mod types;

// Re-export all public types
pub use types::*;

// Re-export public utilities
pub use conversion::locale_to_flag;

use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::Value as JsonValue;

/// Full taxonomy tree: Realm > Layer > Class + ArcFamily > ArcClass.
#[derive(Debug, Clone, Default)]
pub struct TaxonomyTree {
    pub realms: Vec<RealmInfo>,
    pub arc_families: Vec<ArcFamilyInfo>,
    pub stats: GraphStats,
    /// Collapsed state: stores which tree nodes are collapsed.
    /// Uses FxHashSet for ~30% faster lookups.
    pub collapsed: FxHashSet<CollapseKey>,
    /// Instances loaded for Data view, keyed by Class key.
    /// Only populated when in Data mode and a Class is selected.
    /// Uses FxHashMap for ~30% faster lookups (no ordering needed).
    pub instances: FxHashMap<String, Vec<InstanceInfo>>,
    /// Total instance counts in Neo4j (may be > loaded instances due to INSTANCE_LIMIT).
    /// Used to show "3/300 of 847" when results are truncated.
    /// Uses FxHashMap for ~30% faster lookups (no ordering needed).
    pub instance_totals: FxHashMap<String, usize>,
    /// Cache: class_key -> (realm_idx, layer_idx, kind_idx) for O(1) lookups.
    /// Built once on load, never mutated (tree structure is immutable).
    pub(crate) class_index: FxHashMap<String, (usize, usize, usize)>,
    /// Entity categories for Data mode grouping.
    /// Loaded on-demand when viewing Entity instances by category.
    pub entity_categories: Vec<EntityCategory>,
    /// Entity instances grouped by category (key = category key like "THING", "ACTION").
    /// Loaded on-demand when Entity categories are expanded.
    /// Uses FxHashMap for ~30% faster lookups (no ordering needed).
    pub entity_category_instances: FxHashMap<String, Vec<InstanceInfo>>,
    /// EntityNative groups by parent Entity (sorted by entity key).
    /// Loaded on-demand when viewing EntityNative class.
    pub entity_native_groups: Vec<EntityNativeGroup>,
    /// EntityNative instances grouped by parent entity (key = entity key like "qr-code").
    /// Loaded on-demand when entity groups are expanded.
    pub entity_native_by_entity: FxHashMap<String, Vec<EntityNativeInfo>>,
}

impl TaxonomyTree {
    // ========================================================================
    // Instance data methods
    // ========================================================================

    /// Set instances for a Class (used in Data mode).
    /// Also stores the total count for "X of Y" display.
    /// Calculates missing_required_count for each instance based on Class schema.
    pub fn set_instances(
        &mut self,
        class_key: &str,
        mut instances: Vec<InstanceInfo>,
        total: usize,
    ) {
        // Get schema info from Class
        let (required_props, all_props) = self
            .find_class(class_key)
            .map(|(_, _, class_info)| {
                (
                    class_info.required_properties.clone(),
                    class_info.properties.clone(),
                )
            })
            .unwrap_or_default();

        let total_props = all_props.len();

        // Calculate stats for each instance
        for instance in &mut instances {
            // Missing required count
            let missing = required_props
                .iter()
                .filter(|prop| {
                    // Property is missing if not present or is null/empty
                    match instance.properties.get(*prop) {
                        None => true,
                        Some(JsonValue::Null) => true,
                        Some(JsonValue::String(s)) => s.is_empty(),
                        Some(_) => false,
                    }
                })
                .count();
            instance.missing_required_count = missing;

            // Filled properties count (non-null, non-empty)
            let filled = instance
                .properties
                .values()
                .filter(|v| {
                    !matches!(v, JsonValue::Null)
                        && !matches!(v, JsonValue::String(s) if s.is_empty())
                })
                .count();
            instance.filled_properties = filled;
            instance.total_properties = total_props;
        }

        self.instances.insert(class_key.to_string(), instances);
        self.instance_totals.insert(class_key.to_string(), total);
    }

    /// Get instances for a Class.
    pub fn get_instances(&self, class_key: &str) -> Option<&Vec<InstanceInfo>> {
        self.instances.get(class_key)
    }

    /// Get total instance count for a Class (may be > loaded instances).
    pub fn get_instance_total(&self, class_key: &str) -> Option<usize> {
        self.instance_totals.get(class_key).copied()
    }

    /// Update arcs for instances after progressive loading.
    /// Called AFTER `set_instances` with arc data from `load_instance_arcs`.
    pub fn update_instance_arcs(
        &mut self,
        class_key: &str,
        arcs: FxHashMap<String, (Vec<InstanceArc>, Vec<InstanceArc>)>,
    ) {
        if let Some(instances) = self.instances.get_mut(class_key) {
            for instance in instances.iter_mut() {
                if let Some((outgoing, incoming)) = arcs.get(&instance.key) {
                    instance.outgoing_arcs = outgoing.clone();
                    instance.incoming_arcs = incoming.clone();
                    instance.arcs_loading = false;
                }
            }
        }
    }
}

// =============================================================================
// Test-only TaxonomyTree construction
// =============================================================================

#[cfg(test)]
impl TaxonomyTree {
    /// Create a minimal mock tree for unit tests.
    ///
    /// Structure:
    /// - shared (1 layer)
    ///   - config (1 class)
    ///     - AppConfig
    /// - org (1 layer)
    ///   - foundation (1 class)
    ///     - Entity
    ///
    /// Empty arc_families and default stats.
    pub fn mock_for_testing() -> Self {
        let app_config = ClassInfo {
            key: "AppConfig".to_string(),
            display_name: "App Config".to_string(),
            description: "Application configuration".to_string(),
            icon: String::new(),
            instance_count: 0,
            arcs: Vec::new(),
            yaml_path: String::new(),
            properties: Vec::new(),
            required_properties: Vec::new(),
            schema_hint: String::new(),
            context_budget: String::new(),
            knowledge_tier: None,
            health_percent: None,
            issues_count: None,
        };

        let entity = ClassInfo {
            key: "Entity".to_string(),
            display_name: "Entity".to_string(),
            description: "Foundation entity".to_string(),
            icon: String::new(),
            instance_count: 0,
            arcs: Vec::new(),
            yaml_path: String::new(),
            properties: Vec::new(),
            required_properties: Vec::new(),
            schema_hint: String::new(),
            context_budget: String::new(),
            knowledge_tier: None,
            health_percent: None,
            issues_count: None,
        };

        let config_layer = LayerInfo {
            key: "config".to_string(),
            display_name: "Config".to_string(),
            color: "#6c71c4".to_string(),
            classes: vec![app_config],
            content: String::new(),
        };

        let foundation_layer = LayerInfo {
            key: "foundation".to_string(),
            display_name: "Foundation".to_string(),
            color: "#268bd2".to_string(),
            classes: vec![entity],
            content: String::new(),
        };

        let shared_realm = RealmInfo {
            key: "shared".to_string(),
            display_name: "Shared".to_string(),
            color: "#2aa198".to_string(),
            icon: "◉",
            layers: vec![config_layer],
            content: String::new(),
        };

        let org_realm = RealmInfo {
            key: "org".to_string(),
            display_name: "Org".to_string(),
            color: "#d33682".to_string(),
            icon: "◎",
            layers: vec![foundation_layer],
            content: String::new(),
        };

        let realms = vec![shared_realm, org_realm];

        // Build class_index for O(1) lookups
        let mut class_index = FxHashMap::default();
        for (r_idx, realm) in realms.iter().enumerate() {
            for (l_idx, layer) in realm.layers.iter().enumerate() {
                for (k_idx, class_info) in layer.classes.iter().enumerate() {
                    class_index.insert(class_info.key.clone(), (r_idx, l_idx, k_idx));
                }
            }
        }

        Self {
            realms,
            arc_families: Vec::new(),
            stats: GraphStats::default(),
            collapsed: FxHashSet::default(),
            instances: FxHashMap::default(),
            instance_totals: FxHashMap::default(),
            class_index,
            entity_categories: Vec::new(),
            entity_category_instances: FxHashMap::default(),
            entity_native_groups: Vec::new(),
            entity_native_by_entity: FxHashMap::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::conversion::{to_kebab_case, validate_cypher_label};
    use super::*;
    use crate::tui::testing::{create_test_class, create_test_layer, create_test_realm};
    use std::collections::BTreeMap;
    use types::CollapseKey;

    // ========================================================================
    // TaxonomyTree::mock_for_testing() tests
    // ========================================================================

    #[test]
    fn test_mock_tree_has_realms() {
        let tree = TaxonomyTree::mock_for_testing();

        assert_eq!(tree.realms.len(), 2, "mock should have 2 realms");
        assert_eq!(tree.realms[0].key, "shared");
        assert_eq!(tree.realms[1].key, "org");
    }

    #[test]
    fn test_mock_tree_shared_structure() {
        let tree = TaxonomyTree::mock_for_testing();
        let shared = &tree.realms[0];

        assert_eq!(shared.layers.len(), 1, "shared should have 1 layer");
        assert_eq!(shared.layers[0].key, "config");
        assert_eq!(
            shared.layers[0].classes.len(),
            1,
            "config should have 1 class"
        );
        assert_eq!(shared.layers[0].classes[0].key, "AppConfig");
    }

    #[test]
    fn test_mock_tree_org_structure() {
        let tree = TaxonomyTree::mock_for_testing();
        let org = &tree.realms[1];

        assert_eq!(org.layers.len(), 1, "org should have 1 layer");
        assert_eq!(org.layers[0].key, "foundation");
        assert_eq!(
            org.layers[0].classes.len(),
            1,
            "foundation should have 1 class"
        );
        assert_eq!(org.layers[0].classes[0].key, "Entity");
    }

    #[test]
    fn test_mock_tree_class_index() {
        let tree = TaxonomyTree::mock_for_testing();

        // Verify class_index has correct mappings
        assert_eq!(tree.class_index.get("AppConfig"), Some(&(0, 0, 0)));
        assert_eq!(tree.class_index.get("Entity"), Some(&(1, 0, 0)));
    }

    // ========================================================================
    // Helper functions for creating test data
    // ========================================================================

    fn create_test_tree() -> TaxonomyTree {
        let locale_class = create_test_class("Locale");
        let page_class = create_test_class("Page");
        let entity_class = create_test_class("Entity");

        // Minimal test fixture (4 shared layers: config, locale, geography, knowledge)
        let locale_layer = create_test_layer("locale", vec![locale_class]);
        let structure = create_test_layer("structure", vec![page_class]);
        let semantic = create_test_layer("semantic", vec![entity_class]);

        let shared_realm = create_test_realm("shared", vec![locale_layer]);
        let org_realm = create_test_realm("org", vec![structure, semantic]);

        let realms = vec![shared_realm, org_realm];

        // Build class_index (mirrors load() behavior)
        let mut class_index = FxHashMap::default();
        for (r_idx, realm) in realms.iter().enumerate() {
            for (l_idx, layer) in realm.layers.iter().enumerate() {
                for (k_idx, class_info) in layer.classes.iter().enumerate() {
                    class_index.insert(class_info.key.clone(), (r_idx, l_idx, k_idx));
                }
            }
        }

        TaxonomyTree {
            realms,
            arc_families: Vec::new(),
            stats: GraphStats::default(),
            collapsed: FxHashSet::default(),
            instances: FxHashMap::default(),
            instance_totals: FxHashMap::default(),
            class_index,
            entity_categories: Vec::new(),
            entity_category_instances: FxHashMap::default(),
            entity_native_groups: Vec::new(),
            entity_native_by_entity: FxHashMap::default(),
        }
    }

    // ========================================================================
    // Instance data structure tests
    // ========================================================================

    #[test]
    fn test_instance_info_creation() {
        let instance = InstanceInfo {
            key: "fr-FR".to_string(),
            display_name: "Français (France)".to_string(),
            class_key: "Locale".to_string(),
            properties: BTreeMap::from([
                ("language".to_string(), JsonValue::String("fr".to_string())),
                ("region".to_string(), JsonValue::String("FR".to_string())),
            ]),
            outgoing_arcs: vec![],
            incoming_arcs: vec![],
            arcs_loading: false,
            missing_required_count: 0,
            filled_properties: 0,
            total_properties: 0,
            entity_slug: None,
            relationship_power: 0,
        };

        assert_eq!(instance.key, "fr-FR");
        assert_eq!(instance.class_key, "Locale");
        assert_eq!(
            instance.properties.get("language"),
            Some(&JsonValue::String("fr".to_string()))
        );
    }

    // ========================================================================
    // Tree with instances tests (Data view)
    // ========================================================================

    #[test]
    fn test_tree_item_count_schema_mode() {
        let tree = create_test_tree();
        // In Schema mode: 1 (Classes) + 1 (shared) + 1 (locale) + 1 (Locale)
        //              + 1 (org) + 1 (structure) + 1 (Page) + 1 (semantic) + 1 (Entity)
        //              + 1 (Arcs)
        // Total: 10
        assert_eq!(tree.item_count(), 10);
    }

    #[test]
    fn test_item_count_collapsed() {
        let mut tree = create_test_tree();

        // Collapse everything
        tree.collapse_all();

        // When all collapsed: 1 (Classes header) + 1 (Arcs header) = 2
        assert_eq!(tree.item_count(), 2);
    }

    #[test]
    fn test_toggle_expands_realm() {
        let mut tree = create_test_tree();

        // Start with everything collapsed
        tree.collapse_all();
        let collapsed_count = tree.item_count();
        assert_eq!(collapsed_count, 2); // Just Classes + Arcs headers

        // Expand Classes section
        tree.toggle(&CollapseKey::Classes);

        // Now we see: Classes + shared + org + Arcs = 4
        // (realms are still collapsed, so we don't see layers/classes)
        assert_eq!(tree.item_count(), 4);

        // Expand shared realm
        tree.toggle(&CollapseKey::Realm("shared".to_string()));

        // Now we see: Classes + shared + locale + org + Arcs = 5
        // Note: collapse_all() also collapsed the layer, so we don't see Locale yet
        assert_eq!(tree.item_count(), 5);
    }

    #[test]
    fn test_toggle_twice_collapses() {
        let mut tree = create_test_tree();

        // Get initial count (everything expanded)
        let initial_count = tree.item_count();
        assert_eq!(initial_count, 10);

        // Toggle shared realm to collapse it
        tree.toggle(&CollapseKey::Realm("shared".to_string()));

        // Now: Classes + shared (collapsed) + org + structure + Page + semantic + Entity + Arcs
        // = 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 = 8
        let after_collapse = tree.item_count();
        assert_eq!(after_collapse, 8);

        // Toggle again to expand
        tree.toggle(&CollapseKey::Realm("shared".to_string()));

        // Should return to original count
        assert_eq!(tree.item_count(), initial_count);
    }

    // ========================================================================
    // Tree structure navigation tests
    // ========================================================================

    #[test]
    fn test_mock_tree_has_two_realms() {
        let tree = create_test_tree();
        assert_eq!(tree.realms.len(), 2, "Tree should have exactly 2 realms");
    }

    #[test]
    fn test_mock_tree_shared_realm() {
        let tree = create_test_tree();
        let shared = tree.realms.iter().find(|r| r.key == "shared");
        assert!(shared.is_some(), "Tree should have a shared realm");
    }

    #[test]
    fn test_mock_tree_org_realm() {
        let tree = create_test_tree();
        let org = tree.realms.iter().find(|r| r.key == "org");
        assert!(org.is_some(), "Tree should have an org realm");
    }

    #[test]
    fn test_mock_tree_shared_has_locale_layer() {
        let tree = create_test_tree();
        let shared = tree
            .realms
            .iter()
            .find(|r| r.key == "shared")
            .expect("Shared realm should exist");
        let has_locale = shared.layers.iter().any(|l| l.key == "locale");
        assert!(has_locale, "Shared realm should have locale layer");
    }

    // ========================================================================
    // YAML path validation tests
    // ========================================================================

    #[test]
    fn test_yaml_path_fallback_rejects_unknown_realm() {
        let realm_key = "unknown";
        let layer_key = "structure";
        let class_key = "Page";

        let yaml_path = if realm_key == "unknown" || layer_key == "unknown" {
            String::new()
        } else {
            format!(
                "packages/core/models/node-classes/{}/{}/{}.yaml",
                realm_key,
                layer_key,
                to_kebab_case(class_key)
            )
        };

        assert!(
            yaml_path.is_empty(),
            "Should return empty for unknown realm"
        );
    }

    #[test]
    fn test_yaml_path_fallback_accepts_valid_realm_layer() {
        let realm_key = "org";
        let layer_key = "structure";
        let class_key = "Page";

        let yaml_path = if realm_key == "unknown" || layer_key == "unknown" {
            String::new()
        } else {
            format!(
                "packages/core/models/node-classes/{}/{}/{}.yaml",
                realm_key,
                layer_key,
                to_kebab_case(class_key)
            )
        };

        assert_eq!(
            yaml_path,
            "packages/core/models/node-classes/org/structure/page.yaml"
        );
    }

    // ========================================================================
    // Cypher label validation tests (SQL/Cypher injection prevention)
    // ========================================================================

    #[test]
    fn test_validate_cypher_label_valid() {
        assert!(validate_cypher_label("Entity").is_ok());
        assert!(validate_cypher_label("knowledge").is_ok());
        assert!(validate_cypher_label("PageNative").is_ok());
    }

    #[test]
    fn test_validate_cypher_label_empty() {
        let result = validate_cypher_label("");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Empty label"),
            "Error should mention empty label: {}",
            err_msg
        );
    }

    #[test]
    fn test_validate_cypher_label_invalid_chars() {
        let injection_attempts = [
            "Entity;DROP", // SQL/Cypher injection attempt
            "Page'",       // Quote injection
            "Node\"",      // Double quote injection
            "Entity{",     // Cypher clause injection
            "Kind}",       // Cypher clause end
            "Node:Label",  // Additional label injection
            "A()",         // Function call injection
            "A[0]",        // Index access injection
        ];

        for label in &injection_attempts {
            let result = validate_cypher_label(label);
            assert!(
                result.is_err(),
                "Label '{}' should be rejected as invalid",
                label
            );
            let err_msg = result.unwrap_err().to_string();
            assert!(
                err_msg.contains("Invalid characters"),
                "Error for '{}' should mention invalid characters: {}",
                label,
                err_msg
            );
        }
    }

    // ========================================================================
    // Neo4j Integration Tests (require running Neo4j)
    // Run with: cargo test -- --ignored
    // ========================================================================

    #[tokio::test]
    #[ignore] // Requires running Neo4j
    async fn test_taxonomy_tree_load_integration() {
        let db = crate::db::Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword")
            .await
            .expect("Failed to connect to Neo4j");

        let root = std::path::Path::new(".");
        let tree = TaxonomyTree::load(&db, root)
            .await
            .expect("Failed to load tree");

        assert!(!tree.realms.is_empty(), "Should load realms from Neo4j");
        assert!(
            tree.realms.iter().any(|r| r.key == "shared"),
            "Should have shared realm"
        );
        assert!(
            tree.realms.iter().any(|r| r.key == "org"),
            "Should have org realm"
        );
    }

    #[tokio::test]
    #[ignore] // Requires running Neo4j
    async fn test_load_instances_integration() {
        let db = crate::db::Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword")
            .await
            .expect("Failed to connect to Neo4j");

        let result = TaxonomyTree::load_instances(&db, "Locale").await;

        match result {
            Ok((instances, total)) => {
                assert_eq!(instances.len(), total, "Instance count should match");
            },
            Err(e) => {
                panic!("load_instances failed: {}", e);
            },
        }
    }

    #[tokio::test]
    #[ignore] // Requires running Neo4j
    async fn test_load_class_arcs_integration() {
        let db = crate::db::Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword")
            .await
            .expect("Failed to connect to Neo4j");

        let result = TaxonomyTree::load_class_arcs(&db, "Page").await;

        match result {
            Ok(arcs_data) => {
                let _ = arcs_data.outgoing.len();
            },
            Err(e) => {
                panic!("load_class_arcs failed: {}", e);
            },
        }
    }

    // ========================================================================
    // find_first_instance_cursor tests
    // ========================================================================

    #[test]
    fn test_find_first_instance_cursor_with_collapsed_class() {
        let mut tree = TaxonomyTree::mock_for_testing();

        tree.instances.insert(
            "AppConfig".to_string(),
            vec![InstanceInfo {
                key: "instance1".to_string(),
                display_name: "Instance 1".to_string(),
                class_key: "AppConfig".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
                entity_slug: None,
                relationship_power: 0,
            }],
        );

        // Explicitly collapse the class (default is expanded)
        tree.collapse_subtree(&CollapseKey::Class("AppConfig".to_string()));

        // Class is collapsed, so should return None
        let result = tree.find_first_instance_cursor("shared", "config", "AppConfig");
        assert!(result.is_none(), "Collapsed class should return None");
    }

    #[test]
    fn test_find_first_instance_cursor_with_expanded_class() {
        let mut tree = TaxonomyTree::mock_for_testing();

        tree.instances.insert(
            "AppConfig".to_string(),
            vec![InstanceInfo {
                key: "instance1".to_string(),
                display_name: "Instance 1".to_string(),
                class_key: "AppConfig".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
                entity_slug: None,
                relationship_power: 0,
            }],
        );

        // Expand necessary nodes
        tree.expand(&CollapseKey::Classes);
        tree.expand(&CollapseKey::Realm("shared".to_string()));
        tree.expand(&CollapseKey::Layer {
            realm: "shared".to_string(),
            layer: "config".to_string(),
        });
        tree.expand(&CollapseKey::Class("AppConfig".to_string()));

        // Now should find the first instance
        let result = tree.find_first_instance_cursor("shared", "config", "AppConfig");
        assert!(
            result.is_some(),
            "Expanded class with instances should return Some"
        );

        // The cursor should be after the class node
        let class_cursor = tree.find_class_cursor_readonly("shared", "config", "AppConfig", true);
        assert!(class_cursor.is_some());
        assert_eq!(result.unwrap(), class_cursor.unwrap() + 1);
    }

    #[test]
    fn test_find_first_instance_cursor_no_instances() {
        let mut tree = TaxonomyTree::mock_for_testing();

        tree.expand(&CollapseKey::Classes);
        tree.expand(&CollapseKey::Realm("shared".to_string()));
        tree.expand(&CollapseKey::Layer {
            realm: "shared".to_string(),
            layer: "config".to_string(),
        });
        tree.expand(&CollapseKey::Class("AppConfig".to_string()));

        let result = tree.find_first_instance_cursor("shared", "config", "AppConfig");
        assert!(
            result.is_none(),
            "Class with no instances should return None"
        );
    }

    #[test]
    fn test_find_first_instance_cursor_invalid_class() {
        let tree = TaxonomyTree::mock_for_testing();

        let result = tree.find_first_instance_cursor("shared", "config", "NonExistent");
        assert!(result.is_none(), "Invalid class should return None");
    }
}
