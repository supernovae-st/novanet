//! Tree navigation: item counting, cursor lookup, parent finding, and hierarchy position.
//!
//! Handles the mapping between a flat cursor index and the hierarchical tree structure.
//! All methods respect collapsed state and data/schema mode filtering.

use super::conversion::INSTANCE_LIMIT;
use super::types::*;
use super::TaxonomyTree;

impl TaxonomyTree {
    // ========================================================================
    // Tree item counting and navigation
    // ========================================================================

    /// Total number of visible items for a specific mode.
    /// In Data mode (data_mode=true), includes instances under expanded Classes.
    /// Entity instances are flat (no category rows) with category suffix in display.
    /// Added hide_empty parameter to match render_tree and item_at_for_mode filtering.
    pub fn item_count_for_mode(&self, data_mode: bool, hide_empty: bool) -> usize {
        let mut count = 0;

        // Classes section
        count += 1; // "Classes" header
        if !self.is_collapsed(&CollapseKey::Classes) {
            for realm in &self.realms {
                count += 1; // realm header
                if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
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
                        if !self.is_collapsed(&CollapseKey::Layer { realm: realm.key.clone(), layer: layer.key.clone() }) {
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
                                    && !self.is_collapsed(&CollapseKey::Class(class_info.key.clone()))
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
                                            if !self.is_collapsed(&CollapseKey::EntityGroup(group.entity_key.clone())) {
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
        if !self.is_collapsed(&CollapseKey::Arcs) {
            for family in &self.arc_families {
                count += 1; // family header
                if !self.is_collapsed(&CollapseKey::Family(family.key.clone())) {
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

        if !self.is_collapsed(&CollapseKey::Classes) {
            for realm in &self.realms {
                if idx == cursor {
                    return Some(TreeItem::Realm(realm));
                }
                idx += 1;

                if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
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

                        if !self.is_collapsed(&CollapseKey::Layer { realm: realm.key.clone(), layer: layer.key.clone() }) {
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
                                    && !self.is_collapsed(&CollapseKey::Class(class_info.key.clone()))
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
                                            if !self.is_collapsed(&CollapseKey::EntityGroup(group.entity_key.clone())) {
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

        if !self.is_collapsed(&CollapseKey::Arcs) {
            for family in &self.arc_families {
                if idx == cursor {
                    return Some(TreeItem::ArcFamily(family));
                }
                idx += 1;

                if !self.is_collapsed(&CollapseKey::Family(family.key.clone())) {
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

        // Classes section
        count += 1; // "Classes" header
        if !self.is_collapsed(&CollapseKey::Classes) {
            for realm in &self.realms {
                count += 1; // realm header
                if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                    for layer in &realm.layers {
                        count += 1; // layer header
                        if !self.is_collapsed(&CollapseKey::Layer { realm: realm.key.clone(), layer: layer.key.clone() }) {
                            count += layer.classes.len();
                        }
                    }
                }
            }
        }

        // Arcs section
        count += 1; // "Arcs" header
        if !self.is_collapsed(&CollapseKey::Arcs) {
            for family in &self.arc_families {
                count += 1; // family header
                if !self.is_collapsed(&CollapseKey::Family(family.key.clone())) {
                    count += family.arc_classes.len();
                }
            }
        }

        count
    }

    /// Get item at cursor position (respects collapsed state).
    pub fn item_at(&self, cursor: usize) -> Option<TreeItem<'_>> {
        let mut idx = 0;

        // Classes section header
        if idx == cursor {
            return Some(TreeItem::ClassesSection);
        }
        idx += 1;

        if !self.is_collapsed(&CollapseKey::Classes) {
            for realm in &self.realms {
                if idx == cursor {
                    return Some(TreeItem::Realm(realm));
                }
                idx += 1;

                if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                    for layer in &realm.layers {
                        if idx == cursor {
                            return Some(TreeItem::Layer(realm, layer));
                        }
                        idx += 1;

                        if !self.is_collapsed(&CollapseKey::Layer { realm: realm.key.clone(), layer: layer.key.clone() }) {
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

        if !self.is_collapsed(&CollapseKey::Arcs) {
            for family in &self.arc_families {
                if idx == cursor {
                    return Some(TreeItem::ArcFamily(family));
                }
                idx += 1;

                if !self.is_collapsed(&CollapseKey::Family(family.key.clone())) {
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
    ) -> Option<CollapseKey> {
        let item = if data_mode {
            self.item_at_for_mode(cursor, true, hide_empty)
        } else {
            self.item_at(cursor)
        };
        match item {
            Some(TreeItem::ClassesSection) => Some(CollapseKey::Classes),
            Some(TreeItem::ArcsSection) => Some(CollapseKey::Arcs),
            Some(TreeItem::Realm(r)) => Some(CollapseKey::Realm(r.key.clone())),
            Some(TreeItem::Layer(r, l)) => Some(CollapseKey::Layer { realm: r.key.clone(), layer: l.key.clone() }),
            Some(TreeItem::ArcFamily(f)) => Some(CollapseKey::Family(f.key.clone())),
            // In Data mode, Class can be collapsed to hide instances
            Some(TreeItem::Class(_, _, k)) => Some(CollapseKey::Class(k.key.clone())),
            // EntityCategory can be collapsed to hide its instances
            Some(TreeItem::EntityCategory(_, _, _, cat)) => Some(CollapseKey::Category(cat.key.clone())),
            // EntityGroup can be collapsed to hide its EntityNativeItems
            Some(TreeItem::EntityGroup(_, _, _, group)) => {
                Some(CollapseKey::EntityGroup(group.entity_key.clone()))
            },
            // Entity instances can be collapsed to hide EntityNatives
            Some(TreeItem::Instance(_, _, class_info, _instance)) if class_info.key == "Entity" => {
                None // Entity instances don't have a collapse variant
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
        if self.is_collapsed(&CollapseKey::Classes) {
            return None; // Realm not visible
        }
        let mut idx = 1; // Skip ClassesSection
        for realm in &self.realms {
            if realm.key == realm_key {
                return Some(idx);
            }
            idx += 1;
            if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                for layer in &realm.layers {
                    idx += 1;
                    if !self.is_collapsed(&CollapseKey::Layer { realm: realm.key.clone(), layer: layer.key.clone() }) {
                        idx += layer.classes.len();
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of a Layer (does not modify collapse state).
    fn find_layer_cursor(&self, realm_key: &str, layer_key: &str) -> Option<usize> {
        if self.is_collapsed(&CollapseKey::Classes) {
            return None;
        }
        let mut idx = 1; // Skip ClassesSection
        for realm in &self.realms {
            idx += 1; // Realm
            if realm.key == realm_key {
                if self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                    return None; // Layer not visible
                }
                for layer in &realm.layers {
                    if layer.key == layer_key {
                        return Some(idx);
                    }
                    idx += 1;
                    if !self.is_collapsed(&CollapseKey::Layer { realm: realm.key.clone(), layer: layer.key.clone() }) {
                        idx += layer.classes.len();
                    }
                }
                return None;
            }
            if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                for layer in &realm.layers {
                    idx += 1;
                    if !self.is_collapsed(&CollapseKey::Layer { realm: realm.key.clone(), layer: layer.key.clone() }) {
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
        if self.is_collapsed(&CollapseKey::Classes) {
            return None;
        }
        let mut idx = 1; // Skip ClassesSection
        for realm in &self.realms {
            idx += 1; // Realm
            if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                for layer in &realm.layers {
                    idx += 1; // Layer
                    if !self.is_collapsed(&CollapseKey::Layer { realm: realm.key.clone(), layer: layer.key.clone() }) {
                        for class_info in &layer.classes {
                            if realm.key == realm_key
                                && layer.key == layer_key
                                && class_info.key == class_key
                            {
                                return Some(idx);
                            }
                            idx += 1;
                            // In data mode, count instances
                            if data_mode && !self.is_collapsed(&CollapseKey::Class(class_info.key.clone()))
                            {
                                // Entity uses flat instances (matches tree.rs rendering)
                                if class_info.key == "Entity" {
                                    idx += self.entity_instances_flat().count();
                                } else if class_info.key == "EntityNative" {
                                    // EntityNative: count groups + expanded natives
                                    for group in &self.entity_native_groups {
                                        idx += 1; // The group itself
                                        if !self.is_collapsed(&CollapseKey::EntityGroup(group.entity_key.clone())) {
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
    /// Used when expanding a Class node to select its first instance.
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
        if self.is_collapsed(&CollapseKey::Class(class_key.to_string())) {
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
        if !self.is_collapsed(&CollapseKey::Classes) {
            for realm in &self.realms {
                idx += 1;
                if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                    for layer in &realm.layers {
                        idx += 1;
                        if !self.is_collapsed(&CollapseKey::Layer { realm: realm.key.clone(), layer: layer.key.clone() }) {
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
        if self.is_collapsed(&CollapseKey::Arcs) {
            return None;
        }
        let mut idx = arcs_idx + 1;
        for family in &self.arc_families {
            if family.key == family_key {
                return Some(idx);
            }
            idx += 1;
            if !self.is_collapsed(&CollapseKey::Family(family.key.clone())) {
                idx += family.arc_classes.len();
            }
        }
        None
    }

    // ========================================================================
    // Filtered Data mode: show only instances of a specific Class
    // ========================================================================

    /// Get item count when filtered to a specific Class (Data mode drill-down).
    /// Returns only instances of that Class.
    /// Entity uses helper for dual storage.
    pub fn filtered_item_count(&self, class_key: &str) -> usize {
        if class_key == "Entity" {
            self.entity_instance_count()
        } else {
            self.instances.get(class_key).map(|v| v.len()).unwrap_or(0)
        }
    }

    /// Get item at cursor when filtered to a specific Class.
    /// Returns Instance items only.
    /// Entity uses helper for dual storage.
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
