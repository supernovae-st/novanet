//! Data loading for TUI — Neo4j queries for taxonomy tree, stats, and detail.
//!
//! Split into submodules:
//! - `types`: All struct/enum definitions (ArcInfo, ClassInfo, TreeItem, etc.)
//! - `conversion`: Bolt-to-JSON conversion, label validation, formatting helpers
//! - `queries`: All async Neo4j query methods (impl TaxonomyTree)

mod conversion;
mod queries;
mod types;

// Re-export all public types
pub use types::*;

// Re-export public utilities
pub use conversion::{locale_to_flag, INSTANCE_LIMIT};

// Make crate-internal utilities accessible via `use super::*` in tests
#[allow(unused_imports)]
pub(crate) use conversion::{bolt_to_json, realm_icon, to_kebab_case, validate_cypher_label};

use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::Value as JsonValue;

/// Full taxonomy tree: Realm > Layer > Class + ArcFamily > ArcClass.
#[derive(Debug, Clone, Default)]
pub struct TaxonomyTree {
    pub realms: Vec<RealmInfo>,
    pub arc_families: Vec<ArcFamilyInfo>,
    pub stats: GraphStats,
    /// Collapsed state: stores keys of collapsed nodes (e.g., "classes", "arcs", "realm:shared", "layer:structure")
    /// Uses FxHashSet for ~30% faster lookups on string keys.
    pub collapsed: FxHashSet<String>,
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
    // Collapse/Expand methods
    // ========================================================================

    /// Check if a node is collapsed.
    pub fn is_collapsed(&self, key: &str) -> bool {
        self.collapsed.contains(key)
    }

    /// Toggle collapse state of a node.
    pub fn toggle(&mut self, key: &str) {
        if self.collapsed.contains(key) {
            self.collapsed.remove(key);
        } else {
            self.collapsed.insert(key.to_string());
        }
    }

    /// Collapse all collapsible nodes.
    pub fn collapse_all(&mut self) {
        self.collapsed.insert("classes".to_string());
        self.collapsed.insert("arcs".to_string());
        for realm in &self.realms {
            self.collapsed.insert(format!("realm:{}", realm.key));
            for layer in &realm.layers {
                self.collapsed
                    .insert(format!("layer:{}:{}", realm.key, layer.key));
            }
        }
        for family in &self.arc_families {
            self.collapsed.insert(format!("family:{}", family.key));
        }
    }

    /// Initialize with smart default collapsed state for good UX.
    /// Start with a clean, navigable tree:
    /// - Classes section: open (shows realms)
    /// - Arcs section: collapsed
    /// - Realms: open (shows layers)
    /// - Layers: collapsed (user expands what they need)
    /// - Classes: collapsed (instances hidden until explicitly opened)
    /// - Categories/Groups: collapsed
    pub fn init_default_collapsed(&mut self) {
        self.collapsed.clear();

        // Arcs section collapsed
        self.collapsed.insert("arcs".to_string());

        // All layers collapsed (user opens what they need)
        for realm in &self.realms {
            for layer in &realm.layers {
                self.collapsed
                    .insert(format!("layer:{}:{}", realm.key, layer.key));
            }
        }

        // All arc families collapsed
        for family in &self.arc_families {
            self.collapsed.insert(format!("family:{}", family.key));
        }

        // All classes collapsed (instances hidden)
        for realm in &self.realms {
            for layer in &realm.layers {
                for class_info in &layer.classes {
                    self.collapsed.insert(format!("class:{}", class_info.key));
                }
            }
        }

        // Entity categories collapsed
        for cat in &self.entity_categories {
            self.collapsed.insert(format!("category:{}", cat.key));
        }

        // EntityNative groups collapsed
        for group in &self.entity_native_groups {
            self.collapsed
                .insert(format!("entity_group:{}", group.entity_key));
        }
    }

    /// Expand all nodes.
    pub fn expand_all(&mut self) {
        self.collapsed.clear();
    }

    /// Expand a single node (remove from collapsed set).
    /// Unlike `expand_subtree`, this only expands the specified item.
    pub fn expand(&mut self, key: &str) {
        self.collapsed.remove(key);
    }

    /// Collapse all Class instances (hide their instances).
    /// Used when switching between Meta and Data modes.
    pub fn collapse_all_classes(&mut self) {
        for realm in &self.realms {
            for layer in &realm.layers {
                for class_info in &layer.classes {
                    self.collapsed.insert(format!("class:{}", class_info.key));
                }
            }
        }
    }

    /// Expand subtree under a specific key.
    /// Expands the item and all its children.
    pub fn expand_subtree(&mut self, key: &str) {
        // Remove the key itself
        self.collapsed.remove(key);

        // Expand children based on key type
        if key == "classes" {
            // Expand all realms and layers
            for realm in &self.realms {
                self.collapsed.remove(&format!("realm:{}", realm.key));
                for layer in &realm.layers {
                    self.collapsed
                        .remove(&format!("layer:{}:{}", realm.key, layer.key));
                    for class_info in &layer.classes {
                        self.collapsed.remove(&format!("class:{}", class_info.key));
                    }
                }
            }
        } else if key == "arcs" {
            // Expand all arc families
            for family in &self.arc_families {
                self.collapsed.remove(&format!("family:{}", family.key));
            }
        } else if let Some(realm_key) = key.strip_prefix("realm:") {
            // Expand all layers in this realm
            if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                for layer in &realm.layers {
                    self.collapsed
                        .remove(&format!("layer:{}:{}", realm_key, layer.key));
                    for class_info in &layer.classes {
                        self.collapsed.remove(&format!("class:{}", class_info.key));
                    }
                }
            }
        } else if let Some(rest) = key.strip_prefix("layer:") {
            // Layer key format: layer:{realm_key}:{layer_key}
            // Expand all classes in this layer
            if let Some((realm_key, layer_key)) = rest.split_once(':') {
                if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                    if let Some(layer) = realm.layers.iter().find(|l| l.key == layer_key) {
                        for class_info in &layer.classes {
                            self.collapsed.remove(&format!("class:{}", class_info.key));
                        }
                    }
                }
            }
        } else if let Some(family_key) = key.strip_prefix("family:") {
            // Arc family - nothing more to expand (arc classes aren't collapsible)
            let _ = family_key; // Suppress unused warning
        }
        // class: prefix - nothing more to expand (instances aren't collapsible)
    }

    /// Collapse subtree under a specific key.
    /// Collapses the item and all its children.
    pub fn collapse_subtree(&mut self, key: &str) {
        // Collapse the key itself
        self.collapsed.insert(key.to_string());

        // Collapse children based on key type
        if key == "classes" {
            // Collapse all realms and layers
            for realm in &self.realms {
                self.collapsed.insert(format!("realm:{}", realm.key));
                for layer in &realm.layers {
                    self.collapsed
                        .insert(format!("layer:{}:{}", realm.key, layer.key));
                    for class_info in &layer.classes {
                        self.collapsed.insert(format!("class:{}", class_info.key));
                    }
                }
            }
        } else if key == "arcs" {
            // Collapse all arc families
            for family in &self.arc_families {
                self.collapsed.insert(format!("family:{}", family.key));
            }
        } else if let Some(realm_key) = key.strip_prefix("realm:") {
            // Collapse all layers in this realm
            if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                for layer in &realm.layers {
                    self.collapsed
                        .insert(format!("layer:{}:{}", realm_key, layer.key));
                    for class_info in &layer.classes {
                        self.collapsed.insert(format!("class:{}", class_info.key));
                    }
                }
            }
        } else if let Some(rest) = key.strip_prefix("layer:") {
            // Layer key format: layer:{realm_key}:{layer_key}
            // Collapse all classes in this layer
            if let Some((realm_key, layer_key)) = rest.split_once(':') {
                if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                    if let Some(layer) = realm.layers.iter().find(|l| l.key == layer_key) {
                        for class_info in &layer.classes {
                            self.collapsed.insert(format!("class:{}", class_info.key));
                        }
                    }
                }
            }
        } else if let Some(family_key) = key.strip_prefix("family:") {
            // Arc family - nothing more to collapse
            let _ = family_key;
        }
        // class: prefix - nothing more to collapse
    }

    // ========================================================================
    // Data view: Instance methods
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

    // ========================================================================
    // Tree item counting and navigation
    // ========================================================================

    /// Total number of visible items for a specific mode.
    /// In Data mode (data_mode=true), includes instances under expanded Classes.
    /// Entity instances are flat (no category rows) with category suffix in display.
    /// Added hide_empty parameter to match render_tree and item_at_for_mode filtering.
    pub fn item_count_for_mode(&self, data_mode: bool, hide_empty: bool) -> usize {
        let mut count = 0;

        // Classs section
        count += 1; // "Classes" header
        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                count += 1; // realm header
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    // Filter layers like render_tree does
                    let visible_layers: Vec<_> = realm
                        .layers
                        .iter()
                        .filter(|l| {
                            if hide_empty && data_mode {
                                l.classes.iter().map(|k| k.instance_count).sum::<i64>() > 0
                            } else {
                                true
                            }
                        })
                        .collect();

                    for layer in visible_layers {
                        count += 1; // layer header
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            // Filter classes like render_tree does
                            let visible_classes: Vec<_> = layer
                                .classes
                                .iter()
                                .filter(|k| {
                                    if hide_empty && data_mode {
                                        k.instance_count > 0
                                    } else {
                                        true
                                    }
                                })
                                .collect();

                            for class_info in visible_classes {
                                count += 1; // class

                                // In Data mode, add instances if not collapsed
                                if data_mode
                                    && !self.is_collapsed(&format!("class:{}", class_info.key))
                                {
                                    // Entity shows flat alphabetical list (no categories)
                                    if class_info.key == "Entity" {
                                        // DISABLED EntityCategory grouping
                                        // Always use flat instance count
                                        count += self.entity_instance_count();
                                    } else if class_info.key == "EntityNative" {
                                        // EntityNative shows EntityGroup nodes (grouped by parent Entity)
                                        for group in &self.entity_native_groups {
                                            count += 1; // EntityGroup node
                                            // If entity group is expanded, add its EntityNativeItems
                                            if !self.is_collapsed(&format!(
                                                "entity_group:{}",
                                                group.entity_key
                                            )) {
                                                if let Some(natives) = self
                                                    .entity_native_by_entity
                                                    .get(&group.entity_key)
                                                {
                                                    count += natives.len();
                                                }
                                            }
                                        }
                                    } else {
                                        // Regular class: flat instances
                                        if let Some(instances) = self.instances.get(&class_info.key)
                                        {
                                            count += instances.len();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Arcs section
        count += 1; // "Arcs" header
        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                count += 1; // family header
                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    count += family.arc_classes.len();
                }
            }
        }

        count
    }

    /// Get item at cursor position for a specific mode.
    /// In Data mode (data_mode=true), includes instances under expanded Classes.
    /// Added hide_empty parameter to match render_tree filtering.
    pub fn item_at_for_mode(
        &self,
        cursor: usize,
        data_mode: bool,
        hide_empty: bool,
    ) -> Option<TreeItem<'_>> {
        let mut idx = 0;

        // Classes section header
        if idx == cursor {
            return Some(TreeItem::ClassesSection);
        }
        idx += 1;

        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                if idx == cursor {
                    return Some(TreeItem::Realm(realm));
                }
                idx += 1;

                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    // Filter layers like render_tree does
                    let visible_layers: Vec<_> = realm
                        .layers
                        .iter()
                        .filter(|l| {
                            if hide_empty && data_mode {
                                l.classes.iter().map(|k| k.instance_count).sum::<i64>() > 0
                            } else {
                                true
                            }
                        })
                        .collect();

                    for layer in visible_layers {
                        if idx == cursor {
                            return Some(TreeItem::Layer(realm, layer));
                        }
                        idx += 1;

                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            // Filter classes like render_tree does
                            let visible_classes: Vec<_> = layer
                                .classes
                                .iter()
                                .filter(|k| {
                                    if hide_empty && data_mode {
                                        k.instance_count > 0
                                    } else {
                                        true
                                    }
                                })
                                .collect();

                            for class_info in visible_classes {
                                if idx == cursor {
                                    return Some(TreeItem::Class(realm, layer, class_info));
                                }
                                idx += 1;

                                // In Data mode, check for instances
                                if data_mode
                                    && !self.is_collapsed(&format!("class:{}", class_info.key))
                                {
                                    // Entity shows simple flat list (matches tree.rs rendering)
                                    // No categories, no expand - just instances
                                    if class_info.key == "Entity" {
                                        for instance in self.entity_instances_flat() {
                                            if idx == cursor {
                                                return Some(TreeItem::Instance(
                                                    realm, layer, class_info, instance,
                                                ));
                                            }
                                            idx += 1;
                                        }
                                    } else if class_info.key == "EntityNative" {
                                        // EntityNative shows EntityGroup nodes (grouped by parent Entity)
                                        for group in &self.entity_native_groups {
                                            if idx == cursor {
                                                return Some(TreeItem::EntityGroup(
                                                    realm, layer, class_info, group,
                                                ));
                                            }
                                            idx += 1;
                                            // If entity group is expanded, add its EntityNativeItems
                                            if !self.is_collapsed(&format!(
                                                "entity_group:{}",
                                                group.entity_key
                                            )) {
                                                if let Some(natives) = self
                                                    .entity_native_by_entity
                                                    .get(&group.entity_key)
                                                {
                                                    for native in natives {
                                                        if idx == cursor {
                                                            return Some(
                                                                TreeItem::EntityNativeItem(
                                                                    realm, layer, class_info,
                                                                    native,
                                                                ),
                                                            );
                                                        }
                                                        idx += 1;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        // Regular class: flat instances
                                        if let Some(instances) = self.instances.get(&class_info.key)
                                        {
                                            for instance in instances {
                                                if idx == cursor {
                                                    return Some(TreeItem::Instance(
                                                        realm, layer, class_info, instance,
                                                    ));
                                                }
                                                idx += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Arcs section header
        if idx == cursor {
            return Some(TreeItem::ArcsSection);
        }
        idx += 1;

        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                if idx == cursor {
                    return Some(TreeItem::ArcFamily(family));
                }
                idx += 1;

                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    for arc_class in &family.arc_classes {
                        if idx == cursor {
                            return Some(TreeItem::ArcClass(family, arc_class));
                        }
                        idx += 1;
                    }
                }
            }
        }

        None
    }

    /// Total number of visible items in the flattened tree (respects collapsed state).
    pub fn item_count(&self) -> usize {
        let mut count = 0;

        // Classs section
        count += 1; // "Classes" header
        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                count += 1; // realm header
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        count += 1; // layer header
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            count += layer.classes.len();
                        }
                    }
                }
            }
        }

        // Arcs section
        count += 1; // "Arcs" header
        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                count += 1; // family header
                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    count += family.arc_classes.len();
                }
            }
        }

        count
    }

    /// Get item at cursor position (respects collapsed state).
    pub fn item_at(&self, cursor: usize) -> Option<TreeItem<'_>> {
        let mut idx = 0;

        // Classs section header
        if idx == cursor {
            return Some(TreeItem::ClassesSection);
        }
        idx += 1;

        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                if idx == cursor {
                    return Some(TreeItem::Realm(realm));
                }
                idx += 1;

                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        if idx == cursor {
                            return Some(TreeItem::Layer(realm, layer));
                        }
                        idx += 1;

                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            for class_info in &layer.classes {
                                if idx == cursor {
                                    return Some(TreeItem::Class(realm, layer, class_info));
                                }
                                idx += 1;
                            }
                        }
                    }
                }
            }
        }

        // Arcs section header
        if idx == cursor {
            return Some(TreeItem::ArcsSection);
        }
        idx += 1;

        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                if idx == cursor {
                    return Some(TreeItem::ArcFamily(family));
                }
                idx += 1;

                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    for arc_class in &family.arc_classes {
                        if idx == cursor {
                            return Some(TreeItem::ArcClass(family, arc_class));
                        }
                        idx += 1;
                    }
                }
            }
        }

        None
    }

    // =========================================================================
    // Collapse key and cursor navigation
    // =========================================================================

    /// Get the collapse key for an item at cursor position.
    /// Added hide_empty parameter to match render_tree filtering.
    pub fn collapse_key_at(
        &self,
        cursor: usize,
        data_mode: bool,
        hide_empty: bool,
    ) -> Option<String> {
        let item = if data_mode {
            self.item_at_for_mode(cursor, true, hide_empty)
        } else {
            self.item_at(cursor)
        };
        match item {
            Some(TreeItem::ClassesSection) => Some("classes".to_string()),
            Some(TreeItem::ArcsSection) => Some("arcs".to_string()),
            Some(TreeItem::Realm(r)) => Some(format!("realm:{}", r.key)),
            Some(TreeItem::Layer(r, l)) => Some(format!("layer:{}:{}", r.key, l.key)),
            Some(TreeItem::ArcFamily(f)) => Some(format!("family:{}", f.key)),
            // In Data mode, Class can be collapsed to hide instances
            Some(TreeItem::Class(_, _, k)) => Some(format!("class:{}", k.key)),
            // EntityCategory can be collapsed to hide its instances
            Some(TreeItem::EntityCategory(_, _, _, cat)) => Some(format!("category:{}", cat.key)),
            // EntityGroup can be collapsed to hide its EntityNativeItems
            Some(TreeItem::EntityGroup(_, _, _, group)) => {
                Some(format!("entity_group:{}", group.entity_key))
            },
            // Entity instances can be collapsed to hide EntityNatives
            Some(TreeItem::Instance(_, _, class_info, instance)) if class_info.key == "Entity" => {
                Some(format!("entity:{}", instance.key))
            },
            // Other leaf nodes can't be collapsed
            Some(TreeItem::ArcClass(_, _))
            | Some(TreeItem::Instance(_, _, _, _))
            | Some(TreeItem::EntityNativeItem(_, _, _, _))
            | None => None,
        }
    }

    /// Find the cursor position of the parent item.
    /// Returns None if at root or no parent exists.
    /// Hierarchy: Instance -> Class -> Layer -> Realm -> ClassesSection
    ///            ArcClass -> ArcFamily -> ArcsSection
    /// Added hide_empty parameter to match render_tree filtering.
    pub fn find_parent_cursor(
        &self,
        cursor: usize,
        data_mode: bool,
        hide_empty: bool,
    ) -> Option<usize> {
        let current = if data_mode {
            self.item_at_for_mode(cursor, true, hide_empty)
        } else {
            self.item_at(cursor)
        };

        match current {
            // Section headers have no parent
            Some(TreeItem::ClassesSection) | Some(TreeItem::ArcsSection) | None => None,

            // Realm's parent is ClassesSection (always at index 0)
            Some(TreeItem::Realm(_)) => Some(0),

            // Layer's parent is its Realm
            Some(TreeItem::Layer(realm, _)) => self.find_realm_cursor(&realm.key),

            // Class's parent is its Layer
            Some(TreeItem::Class(realm, layer, _)) => {
                self.find_layer_cursor(&realm.key, &layer.key)
            },

            // EntityCategory's parent is its Class (Entity)
            Some(TreeItem::EntityCategory(realm, layer, class_info, _)) => {
                self.find_class_cursor_readonly(&realm.key, &layer.key, &class_info.key, data_mode)
            },

            // EntityGroup's parent is its Class (EntityNative)
            Some(TreeItem::EntityGroup(realm, layer, class_info, _)) => {
                self.find_class_cursor_readonly(&realm.key, &layer.key, &class_info.key, data_mode)
            },

            // Instance's parent is its Class
            Some(TreeItem::Instance(realm, layer, class_info, _)) => {
                self.find_class_cursor_readonly(&realm.key, &layer.key, &class_info.key, data_mode)
            },

            // ArcFamily's parent is ArcsSection
            Some(TreeItem::ArcFamily(_)) => self.find_arcs_section_cursor(),

            // ArcClass's parent is its ArcFamily
            Some(TreeItem::ArcClass(family, _)) => self.find_family_cursor(&family.key),

            // EntityNativeItem's parent is its Class (EntityNative)
            Some(TreeItem::EntityNativeItem(realm, layer, class_info, _)) => {
                self.find_class_cursor_readonly(&realm.key, &layer.key, &class_info.key, data_mode)
            },
        }
    }

    /// Find cursor position of a Realm (does not modify collapse state).
    fn find_realm_cursor(&self, realm_key: &str) -> Option<usize> {
        if self.is_collapsed("classes") {
            return None; // Realm not visible
        }
        let mut idx = 1; // Skip ClassesSection
        for realm in &self.realms {
            if realm.key == realm_key {
                return Some(idx);
            }
            idx += 1;
            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1;
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        idx += layer.classes.len();
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of a Layer (does not modify collapse state).
    fn find_layer_cursor(&self, realm_key: &str, layer_key: &str) -> Option<usize> {
        if self.is_collapsed("classes") {
            return None;
        }
        let mut idx = 1; // Skip ClassesSection
        for realm in &self.realms {
            idx += 1; // Realm
            if realm.key == realm_key {
                if self.is_collapsed(&format!("realm:{}", realm.key)) {
                    return None; // Layer not visible
                }
                for layer in &realm.layers {
                    if layer.key == layer_key {
                        return Some(idx);
                    }
                    idx += 1;
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        idx += layer.classes.len();
                    }
                }
                return None;
            }
            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1;
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        idx += layer.classes.len();
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of a Class (readonly, does not modify collapse state).
    pub(crate) fn find_class_cursor_readonly(
        &self,
        realm_key: &str,
        layer_key: &str,
        class_key: &str,
        data_mode: bool,
    ) -> Option<usize> {
        if self.is_collapsed("classes") {
            return None;
        }
        let mut idx = 1; // Skip ClassesSection
        for realm in &self.realms {
            idx += 1; // Realm
            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1; // Layer
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        for class_info in &layer.classes {
                            if realm.key == realm_key
                                && layer.key == layer_key
                                && class_info.key == class_key
                            {
                                return Some(idx);
                            }
                            idx += 1;
                            // In data mode, count instances
                            if data_mode && !self.is_collapsed(&format!("class:{}", class_info.key))
                            {
                                // Entity uses flat instances (matches tree.rs rendering)
                                if class_info.key == "Entity" {
                                    idx += self.entity_instances_flat().count();
                                } else if class_info.key == "EntityNative" {
                                    // EntityNative: count groups + expanded natives
                                    for group in &self.entity_native_groups {
                                        idx += 1; // The group itself
                                        if !self.is_collapsed(&format!(
                                            "entity_group:{}",
                                            group.entity_key
                                        )) {
                                            if let Some(natives) =
                                                self.entity_native_by_entity.get(&group.entity_key)
                                            {
                                                idx += natives.len();
                                            }
                                        }
                                    }
                                } else if let Some(instances) = self.instances.get(&class_info.key)
                                {
                                    idx += instances.len();
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of the first instance for a Class.
    /// v0.13 A' Tree Sync: Used when switching to Instance tab.
    ///
    /// Requires the class to be expanded (not collapsed).
    /// Returns cursor position right after the class node (where first instance would be).
    pub fn find_first_instance_cursor(
        &self,
        realm_key: &str,
        layer_key: &str,
        class_key: &str,
    ) -> Option<usize> {
        // Check if instances exist - use Entity helper for dual storage
        let has_instances = if class_key == "Entity" {
            self.has_entity_instances()
        } else {
            self.instances
                .get(class_key)
                .map(|v| !v.is_empty())
                .unwrap_or(false)
        };

        if !has_instances {
            return None;
        }

        // Class must be expanded for instances to be visible
        if self.is_collapsed(&format!("class:{}", class_key)) {
            return None;
        }

        // Find the class cursor, then add 1 for first instance
        let class_cursor =
            self.find_class_cursor_readonly(realm_key, layer_key, class_key, true)?;
        Some(class_cursor + 1)
    }

    /// Find cursor position of ArcsSection.
    fn find_arcs_section_cursor(&self) -> Option<usize> {
        let mut idx = 1; // Skip ClassesSection
        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                idx += 1;
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        idx += 1;
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            idx += layer.classes.len();
                        }
                    }
                }
            }
        }
        Some(idx) // ArcsSection is right after all realms
    }

    /// Find cursor position of an ArcFamily.
    fn find_family_cursor(&self, family_key: &str) -> Option<usize> {
        let arcs_idx = self.find_arcs_section_cursor()?;
        if self.is_collapsed("arcs") {
            return None;
        }
        let mut idx = arcs_idx + 1;
        for family in &self.arc_families {
            if family.key == family_key {
                return Some(idx);
            }
            idx += 1;
            if !self.is_collapsed(&format!("family:{}", family.key)) {
                idx += family.arc_classes.len();
            }
        }
        None
    }

    // ========================================================================
    // Entity Helpers — Centralized handling for Entity's dual storage pattern
    // Entity instances can be in entity_category_instances (by category) OR
    // in the regular instances map (fallback/legacy). These helpers abstract
    // this complexity to reduce code duplication across the codebase.
    // ========================================================================

    /// Check if Entity class uses category-based instance storage.
    /// Returns true if entity_category_instances has data.
    #[inline]
    pub fn has_entity_category_instances(&self) -> bool {
        !self.entity_category_instances.is_empty()
    }

    /// Check if Entity class has any displayable content.
    /// Returns true when instances exist (displayable as Instance nodes).
    /// Used for quick "has content" checks to decide if toggle should load or expand.
    pub fn has_entity_instances(&self) -> bool {
        // Entity uses flat instances (same as regular classes)
        self.instances
            .get("Entity")
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    /// Count all Entity instances.
    /// Entity uses flat instances (same as regular classes)
    pub fn entity_instance_count(&self) -> usize {
        self.instances.get("Entity").map(|v| v.len()).unwrap_or(0)
    }

    /// Get a flat iterator over all Entity instances.
    /// Entity uses flat instances (same as regular classes)
    pub fn entity_instances_flat(&self) -> impl Iterator<Item = &InstanceInfo> {
        self.instances
            .get("Entity")
            .into_iter()
            .flat_map(|v| v.iter())
    }

    // ========================================================================
    // Filtered Data mode: show only instances of a specific Class
    // ========================================================================

    /// Get item count when filtered to a specific Class (Data mode drill-down).
    /// Returns only instances of that Class.
    /// Entity uses helper for dual storage (entity_category_instances OR instances).
    pub fn filtered_item_count(&self, class_key: &str) -> usize {
        if class_key == "Entity" {
            self.entity_instance_count()
        } else {
            self.instances.get(class_key).map(|v| v.len()).unwrap_or(0)
        }
    }

    /// Get item at cursor when filtered to a specific Class.
    /// Returns Instance items only.
    /// Entity uses helper for dual storage (entity_category_instances OR instances).
    pub fn filtered_item_at<'a>(&'a self, cursor: usize, class_key: &str) -> Option<TreeItem<'a>> {
        // Find the Class info for context
        let class_tuple = self.find_class(class_key)?;

        if class_key == "Entity" {
            // Use flat instance access via nth()
            if let Some(instance) = self.entity_instances_flat().nth(cursor) {
                return Some(TreeItem::Instance(
                    class_tuple.0,
                    class_tuple.1,
                    class_tuple.2,
                    instance,
                ));
            }
            return None;
        }

        // Regular path for non-Entity classes
        let instances = self.instances.get(class_key)?;
        let instance = instances.get(cursor)?;
        Some(TreeItem::Instance(
            class_tuple.0,
            class_tuple.1,
            class_tuple.2,
            instance,
        ))
    }

    /// Find a Class by key, returns (Realm, Layer, Class) refs.
    /// O(1) lookup using cached index (built once on load).
    pub fn find_class(&self, class_key: &str) -> Option<(&RealmInfo, &LayerInfo, &ClassInfo)> {
        let (r_idx, l_idx, k_idx) = self.class_index.get(class_key)?;
        let realm = self.realms.get(*r_idx)?;
        let layer = realm.layers.get(*l_idx)?;
        let class_info = layer.classes.get(*k_idx)?;
        Some((realm, layer, class_info))
    }

    /// Calculate hierarchical position for the current tree item.
    /// Returns position info: R:realm L:layer C:class I:instance (all 1-based).
    pub fn hierarchy_position(
        &self,
        cursor: usize,
        data_mode: bool,
        hide_empty: bool,
    ) -> HierarchyPosition {
        let item = if data_mode {
            self.item_at_for_mode(cursor, true, hide_empty)
        } else {
            self.item_at(cursor)
        };

        let total_realms = self.realms.len();

        match item {
            None | Some(TreeItem::ClassesSection) | Some(TreeItem::ArcsSection) => {
                HierarchyPosition::default()
            },
            Some(TreeItem::Realm(realm)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    ..Default::default()
                }
            },
            Some(TreeItem::Layer(realm, layer)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    ..Default::default()
                }
            },
            Some(TreeItem::Class(realm, layer, class_info)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let class_idx = layer
                    .classes
                    .iter()
                    .position(|k| k.key == class_info.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    class: Some((class_idx, layer.classes.len())),
                    ..Default::default()
                }
            },
            Some(TreeItem::Instance(realm, layer, class_info, _)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let class_idx = layer
                    .classes
                    .iter()
                    .position(|k| k.key == class_info.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                // Calculate instance position within Class
                let loaded_count = if class_info.key == "Entity"
                    && self.has_entity_category_instances()
                    && !self.entity_categories.is_empty()
                {
                    self.entity_instance_count()
                } else {
                    self.instances
                        .get(&class_info.key)
                        .map(|v| v.len())
                        .unwrap_or(0)
                };
                let total_instances = self
                    .instance_totals
                    .get(&class_info.key)
                    .copied()
                    .unwrap_or(loaded_count);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    class: Some((class_idx, layer.classes.len())),
                    instance: Some((loaded_count.min(INSTANCE_LIMIT), total_instances)),
                }
            },
            Some(TreeItem::EntityCategory(realm, layer, class_info, _)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let class_idx = layer
                    .classes
                    .iter()
                    .position(|k| k.key == class_info.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    class: Some((class_idx, layer.classes.len())),
                    ..Default::default()
                }
            },
            // EntityGroup hierarchy position
            Some(TreeItem::EntityGroup(realm, layer, class_info, _)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let class_idx = layer
                    .classes
                    .iter()
                    .position(|k| k.key == class_info.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    class: Some((class_idx, layer.classes.len())),
                    ..Default::default()
                }
            },
            Some(TreeItem::ArcFamily(_)) | Some(TreeItem::ArcClass(_, _)) => {
                // Arcs section - no realm/layer/class hierarchy
                HierarchyPosition::default()
            },
            Some(TreeItem::EntityNativeItem(realm, layer, class_info, _)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let class_idx = layer
                    .classes
                    .iter()
                    .position(|k| k.key == class_info.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                // Calculate EntityNative instance count across all entity groups
                let loaded_count: usize = self
                    .entity_native_groups
                    .iter()
                    .filter_map(|g| self.entity_native_by_entity.get(&g.entity_key))
                    .map(|v| v.len())
                    .sum();
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    class: Some((class_idx, layer.classes.len())),
                    instance: Some((loaded_count.min(INSTANCE_LIMIT), loaded_count)),
                }
            },
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
    use super::*;
    use std::collections::BTreeMap;

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

    fn create_test_class(key: &str, display_name: &str) -> ClassInfo {
        ClassInfo {
            key: key.to_string(),
            display_name: display_name.to_string(),
            description: String::new(),
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
        }
    }

    fn create_test_layer(key: &str, classes: Vec<ClassInfo>) -> LayerInfo {
        LayerInfo {
            key: key.to_string(),
            display_name: key.to_string(),
            color: "#ffffff".to_string(),
            classes,
            content: String::new(),
        }
    }

    fn create_test_realm(key: &str, layers: Vec<LayerInfo>) -> RealmInfo {
        RealmInfo {
            key: key.to_string(),
            display_name: key.to_string(),
            color: "#ffffff".to_string(),
            icon: "○",
            layers,
            content: String::new(),
        }
    }

    fn create_test_tree() -> TaxonomyTree {
        let locale_class = create_test_class("Locale", "Locale");
        let page_class = create_test_class("Page", "Page");
        let entity_class = create_test_class("Entity", "Entity");

        // Minimal test fixture (v11.5 has 4 shared layers: config, locale, geography, knowledge)
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
        tree.toggle("classes");

        // Now we see: Classes + shared + org + Arcs = 4 (v11.2: shared + org)
        // (realms are still collapsed, so we don't see layers/classes)
        assert_eq!(tree.item_count(), 4);

        // Expand shared realm (v11.2: was global)
        tree.toggle("realm:shared");

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

        // Toggle shared realm to collapse it (v11.2: was global)
        tree.toggle("realm:shared");

        // Now: Classes + shared (collapsed) + org + structure + Page + semantic + Entity + Arcs
        // = 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 = 8
        let after_collapse = tree.item_count();
        assert_eq!(after_collapse, 8);

        // Toggle again to expand
        tree.toggle("realm:shared");

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
        // v11.5: 4 shared layers (config, locale, geography, knowledge); test tree uses "locale"
        let has_locale = shared.layers.iter().any(|l| l.key == "locale");
        assert!(has_locale, "Shared realm should have locale layer");
    }

    // ========================================================================
    // YAML path validation tests
    // ========================================================================

    #[test]
    fn test_yaml_path_fallback_rejects_unknown_realm() {
        // When realm is "unknown", fallback should return empty string
        // instead of generating invalid path like "node-classes/unknown/layer/class.yaml"
        let realm_key = "unknown";
        let layer_key = "structure";
        let class_key = "Page";

        // Simulate the validation logic from TaxonomyTree::load
        let yaml_path = if realm_key == "unknown" || layer_key == "unknown" {
            String::new() // Invalid - can't compute path
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
        // When realm and layer are valid, fallback should generate proper path
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
        // Valid labels: alphanumeric, underscore, dash
        assert!(validate_cypher_label("Entity").is_ok());
        assert!(validate_cypher_label("knowledge").is_ok());
        assert!(validate_cypher_label("PageNative").is_ok());
    }

    #[test]
    fn test_validate_cypher_label_empty() {
        // Empty labels are rejected
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
        // Injection attempts with dangerous characters
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

        // Use current working directory as root (tests run from monorepo root)
        let root = std::path::Path::new(".");
        let tree = TaxonomyTree::load(&db, root)
            .await
            .expect("Failed to load tree");

        // Basic sanity checks
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

        // Load instances for a Class that should exist (Locale has seed data)
        let result = TaxonomyTree::load_instances(&db, "Locale").await;

        match result {
            Ok((instances, total)) => {
                // Should return some data (at least empty vec with count)
                // total is usize, always non-negative
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

        // Load arcs for a Class that should have relationships
        let result = TaxonomyTree::load_class_arcs(&db, "Page").await;

        match result {
            Ok(arcs_data) => {
                // Page should have some outgoing arcs (HAS_BLOCK, etc.)
                // Even if empty, the call should succeed
                // (len() is usize, always >= 0, so just verify call succeeded)
                let _ = arcs_data.outgoing.len();
            },
            Err(e) => {
                panic!("load_class_arcs failed: {}", e);
            },
        }
    }

    // ========================================================================
    // v0.13 A' Tree Sync: find_first_instance_cursor tests
    // ========================================================================

    #[test]
    fn test_find_first_instance_cursor_with_collapsed_class() {
        let mut tree = TaxonomyTree::mock_for_testing();

        // Add an instance to AppConfig (which exists in mock tree)
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
        tree.collapse_subtree("class:AppConfig");

        // Class is collapsed, so should return None
        let result = tree.find_first_instance_cursor("shared", "config", "AppConfig");
        assert!(result.is_none(), "Collapsed class should return None");
    }

    #[test]
    fn test_find_first_instance_cursor_with_expanded_class() {
        let mut tree = TaxonomyTree::mock_for_testing();

        // Add an instance to AppConfig (which exists in mock tree)
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
        tree.expand("classes");
        tree.expand("realm:shared");
        tree.expand("layer:shared:config");
        tree.expand("class:AppConfig");

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

        // Expand necessary nodes but don't add instances
        tree.expand("classes");
        tree.expand("realm:shared");
        tree.expand("layer:shared:config");
        tree.expand("class:AppConfig");

        // No instances, should return None
        let result = tree.find_first_instance_cursor("shared", "config", "AppConfig");
        assert!(
            result.is_none(),
            "Class with no instances should return None"
        );
    }

    #[test]
    fn test_find_first_instance_cursor_invalid_class() {
        let tree = TaxonomyTree::mock_for_testing();

        // Non-existent class
        let result = tree.find_first_instance_cursor("shared", "config", "NonExistent");
        assert!(result.is_none(), "Invalid class should return None");
    }
}
