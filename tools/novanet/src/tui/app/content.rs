//! Content loading and tree item data extraction.
//!
//! Handles YAML file loading with caching, determines what the content panel
//! should display, and bridges tree navigation to content presentation.

use std::fs;
use std::path::Path;

use super::state::{ContentPanelMode, TreeItemData};
use super::App;

use super::super::data::TreeItem;

impl App {
    /// Load YAML content for the current cursor position.
    /// Uses mode-aware item lookup to handle filtered Data mode correctly.
    pub fn load_yaml_for_current(&mut self) {
        // Increment generation to invalidate any in-flight async loads
        self.navigation_generation = self.navigation_generation.wrapping_add(1);

        // Reset scroll positions when changing items
        self.yaml.scroll = 0;
        self.props_scroll = 0;
        self.arcs_scroll = 0;
        // Reset property focus when changing tree items
        self.focused_property_idx = 0;
        self.expanded_property = false;

        // Clear Neo4j data AND pending loads when moving away
        // (prevents race condition where pending load completes after navigation)
        self.details.class_arcs = None;
        self.details.arc_class = None;
        self.details.realm = None;
        self.details.layer = None;
        self.pending.arcs = None;
        self.pending.arc_class = None;
        self.pending.realm = None;
        self.pending.layer = None;
        self.pending.instance = None;

        // Clear Class validation state (only populated for Class items)
        self.schema_overlay.validated_class_properties = None;
        self.schema_overlay.validation_stats = None;

        // Get current item using mode-aware method (handles filtered Data mode)
        // This is the same logic as current_item() but we extract data to avoid borrow issues
        let current = self.get_current_tree_item_data();

        // Content panel mode is determined by tree selection (no toggle)
        // Handle based on item type
        match current {
            TreeItemData::Class {
                yaml_path,
                key,
                properties,
            } => {
                self.load_yaml_cached(&yaml_path);
                self.pending.arcs = Some(key);
                // Load Class validation (Neo4j vs YAML)
                self.load_validated_class_properties(&properties);
            },
            TreeItemData::ArcClass { yaml_path, key } => {
                self.load_yaml_cached(&yaml_path);
                self.pending.arc_class = Some(key);
            },
            TreeItemData::Realm { key } => {
                let path = format!("packages/core/models/realms/{}.yaml", key);
                self.load_yaml_cached(&path);
                self.pending.realm = Some(key);
            },
            TreeItemData::Layer { key } => {
                let path = format!("packages/core/models/layers/{}.yaml", key);
                self.load_yaml_cached(&path);
                self.pending.layer = Some(key);
            },
            TreeItemData::ArcFamily { key } => {
                let path = format!("packages/core/models/arc-families/{}.yaml", key);
                self.load_yaml_cached(&path);
            },
            TreeItemData::Section => {
                // Show _index.yaml (complete schema overview) instead of taxonomy.yaml
                self.load_yaml_cached("packages/core/models/_index.yaml");
            },
            TreeItemData::Instance {
                class_yaml_path,
                class_properties,
                ..
            } => {
                // Load the Class's YAML to show Instance schema (standard_properties)
                if !class_yaml_path.is_empty() {
                    self.load_yaml_cached(&class_yaml_path);
                    // Load validated properties with types (same as Class view)
                    self.load_validated_class_properties(&class_properties);
                } else {
                    self.yaml.path.clear();
                    self.yaml.content.clear();
                    self.yaml.line_count = 0;
                }
            },
            TreeItemData::None => {
                self.yaml.path.clear();
                self.yaml.content.clear();
                self.yaml.line_count = 0;
            },
        }
    }

    /// Extract current tree item data using mode-aware lookup.
    /// Handles filtered Data mode correctly (same logic as current_item()).
    pub(super) fn get_current_tree_item_data(&self) -> TreeItemData {
        // In filtered Data mode, always return Instance (that's all we show)
        if self.is_graph_mode() && self.data_filter_class.is_some() {
            if let Some(class_key) = &self.data_filter_class {
                if let Some(TreeItem::Instance(realm, layer, class_info, instance)) =
                    self.tree.filtered_item_at(self.tree_cursor, class_key)
                {
                    return TreeItemData::Instance {
                        instance_key: instance.key.clone(),
                        class_name: class_info.key.clone(),
                        realm: realm.key.clone(),
                        layer: layer.key.clone(),
                        class_yaml_path: class_info.yaml_path.clone(),
                        class_properties: class_info.properties.clone(),
                        properties: instance.properties.clone(),
                    };
                }
            }
            return TreeItemData::None;
        }

        // Use mode-aware item lookup
        // Pass hide_empty to match render_tree filtering
        let item = if self.is_graph_mode() {
            self.tree
                .item_at_for_mode(self.tree_cursor, true, self.hide_empty)
        } else {
            self.tree.item_at(self.tree_cursor)
        };

        match item {
            Some(TreeItem::Class(_, _, class_info)) => TreeItemData::Class {
                yaml_path: class_info.yaml_path.clone(),
                key: class_info.key.clone(),
                properties: class_info.properties.clone(),
            },
            Some(TreeItem::ArcClass(family, arc)) => {
                let arc_file = arc.key.to_lowercase().replace('_', "-");
                TreeItemData::ArcClass {
                    yaml_path: format!(
                        "packages/core/models/arc-classes/{}/{}.yaml",
                        family.key, arc_file
                    ),
                    key: arc.key.clone(),
                }
            },
            Some(TreeItem::Realm(realm)) => TreeItemData::Realm {
                key: realm.key.clone(),
            },
            Some(TreeItem::Layer(_, layer)) => TreeItemData::Layer {
                key: layer.key.clone(),
            },
            Some(TreeItem::ArcFamily(family)) => TreeItemData::ArcFamily {
                key: family.key.clone(),
            },
            Some(TreeItem::ClassesSection) | Some(TreeItem::ArcsSection) => TreeItemData::Section,
            Some(TreeItem::Instance(realm, layer, class_info, instance)) => {
                TreeItemData::Instance {
                    instance_key: instance.key.clone(),
                    class_name: class_info.key.clone(),
                    realm: realm.key.clone(),
                    layer: layer.key.clone(),
                    class_yaml_path: class_info.yaml_path.clone(),
                    class_properties: class_info.properties.clone(),
                    properties: instance.properties.clone(),
                }
            },
            // EntityCategory is a grouper (THING, ACTION, etc.) - show as Section
            Some(TreeItem::EntityCategory(_, _, _, _)) => TreeItemData::Section,
            // EntityGroup shows parent Entity as INSTANCE panel
            // Look up the Entity instance by key to show its properties
            Some(TreeItem::EntityGroup(_, _, _, group)) => {
                // Find Entity class info
                if let Some((entity_realm, entity_layer, entity_class_info)) =
                    self.tree.find_class("Entity")
                {
                    // Look up the Entity instance with matching key
                    if let Some(instances) = self.tree.instances.get("Entity") {
                        if let Some(entity_instance) =
                            instances.iter().find(|i| i.key == group.entity_key)
                        {
                            return TreeItemData::Instance {
                                instance_key: entity_instance.key.clone(),
                                class_name: entity_class_info.key.clone(),
                                realm: entity_realm.key.clone(),
                                layer: entity_layer.key.clone(),
                                class_yaml_path: entity_class_info.yaml_path.clone(),
                                class_properties: entity_class_info.properties.clone(),
                                properties: entity_instance.properties.clone(),
                            };
                        }
                    }
                }
                // Fallback: show helpful message if Entity lookup fails
                TreeItemData::None
            },
            // EntityNativeItem shows as Instance (same data structure)
            // Now includes full properties for INSTANCE panel display
            Some(TreeItem::EntityNativeItem(realm, layer, class_info, native)) => {
                TreeItemData::Instance {
                    instance_key: native.key.clone(),
                    class_name: class_info.key.clone(),
                    realm: realm.key.clone(),
                    layer: layer.key.clone(),
                    class_yaml_path: class_info.yaml_path.clone(),
                    class_properties: class_info.properties.clone(),
                    properties: native.properties.clone(),
                }
            },
            None => TreeItemData::None,
        }
    }

    /// Determine the content panel mode based on current tree selection.
    ///
    /// Returns a `ContentPanelMode` indicating what the center panel should show:
    /// - `Schema`: YAML definition for Class/ArcClass nodes
    /// - `InstanceInfo`: Info message pointing to PROPERTIES for instances
    /// - `SectionInfo`: Section overview for Section headers
    /// - `Empty`: No content available
    pub fn content_panel_mode(&self) -> ContentPanelMode {
        match self.get_current_tree_item_data() {
            TreeItemData::Class { yaml_path, key, .. } => ContentPanelMode::Schema {
                path: yaml_path,
                name: key,
            },
            TreeItemData::ArcClass { yaml_path, key } => ContentPanelMode::Schema {
                path: yaml_path,
                name: key,
            },
            TreeItemData::Instance {
                instance_key,
                class_name,
                realm,
                layer,
                properties,
                ..
            } => ContentPanelMode::InstanceInfo {
                instance_key,
                class_name,
                realm,
                layer,
                properties,
            },
            TreeItemData::Realm { key } => ContentPanelMode::Schema {
                path: format!("packages/core/models/realms/{}.yaml", key),
                name: format!("Realm: {}", key),
            },
            TreeItemData::Layer { key } => ContentPanelMode::Schema {
                path: format!("packages/core/models/layers/{}.yaml", key),
                name: format!("Layer: {}", key),
            },
            TreeItemData::ArcFamily { key } => ContentPanelMode::Schema {
                path: format!("packages/core/models/arc-families/{}.yaml", key),
                name: format!("Arc Family: {}", key),
            },
            TreeItemData::Section => ContentPanelMode::SectionInfo {
                name: "Section".to_string(),
                description: "Navigate to view node or arc classes.".to_string(),
            },
            TreeItemData::None => ContentPanelMode::Empty,
        }
    }

    /// Load YAML content with caching (avoids re-reading files on every navigation).
    fn load_yaml_cached(&mut self, relative_path: &str) {
        self.yaml.path = relative_path.to_string();

        // Check cache first
        if let Some(cached) = self.yaml_cache.get(relative_path) {
            self.yaml.content = cached.clone();
            self.yaml.line_count = self.yaml.content.lines().count();
            return;
        }

        // Load from disk
        let full_path = Path::new(&self.root_path).join(relative_path);
        let content = fs::read_to_string(&full_path)
            .unwrap_or_else(|_| format!("# File not found: {}", full_path.display()));

        // Update cache
        self.yaml_cache
            .insert(relative_path.to_string(), content.clone());

        // Update display
        self.yaml.content = content;
        self.yaml.line_count = self.yaml.content.lines().count();
    }

    /// Load matched properties for the current instance (schema + values).
    /// Called after loading instance data to prepare schema overlay.
    pub fn load_matched_properties(
        &mut self,
        instance_props: &std::collections::BTreeMap<String, serde_json::Value>,
    ) {
        use super::super::schema::{CoverageStats, load_schema_properties, match_properties};

        // Only in Data mode with schema overlay enabled
        if !self.is_graph_mode() || !self.schema_overlay.enabled {
            self.schema_overlay.matched_properties = None;
            self.schema_overlay.coverage_stats = None;
            return;
        }

        // Need the Class's YAML path to load schema
        if self.yaml.path.is_empty() {
            self.schema_overlay.matched_properties = None;
            self.schema_overlay.coverage_stats = None;
            return;
        }

        // Load schema from YAML
        let schema = load_schema_properties(&self.root_path, &self.yaml.path);
        if schema.is_empty() {
            self.schema_overlay.matched_properties = None;
            self.schema_overlay.coverage_stats = None;
            return;
        }

        // Match properties
        let matched = match_properties(&schema, instance_props);
        let stats = CoverageStats::from_matched(&matched);

        self.schema_overlay.matched_properties = Some(matched);
        self.schema_overlay.coverage_stats = Some(stats);
    }

    /// Load validated properties for the current Class (compares Neo4j vs YAML).
    /// Called when selecting a Class in Meta mode to show validation status.
    /// Uses cached YAML content to avoid redundant file I/O.
    pub fn load_validated_class_properties(&mut self, class_properties: &[String]) {
        use super::super::schema::{ValidationStats, parse_schema_properties, validate_class_properties};

        // Need the Class's YAML path to load schema
        if self.yaml.path.is_empty() {
            return; // State already cleared in load_yaml_for_current()
        }

        // Use cached YAML content (already loaded by load_yaml_cached)
        let yaml_content = match self.yaml_cache.get(&self.yaml.path) {
            Some(content) => content,
            None => {
                tracing::warn!(path = %self.yaml.path, "YAML not in cache for Class validation");
                return;
            },
        };

        // Parse schema from cached YAML content
        let schema = parse_schema_properties(yaml_content);
        if schema.is_empty() {
            tracing::debug!(path = %self.yaml.path, "No schema properties found in YAML");
            return;
        }

        // Validate: compare YAML schema against Neo4j properties
        let validated = validate_class_properties(&schema, class_properties);
        let stats = ValidationStats::from_validated(&validated);

        self.schema_overlay.validated_class_properties = Some(validated);
        self.schema_overlay.validation_stats = Some(stats);
    }
}
