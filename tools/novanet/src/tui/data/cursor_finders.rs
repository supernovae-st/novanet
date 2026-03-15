//! Cursor-finding helpers: locate tree items by key without modifying state.
//!
//! These methods scan the flattened tree to find cursor positions for specific
//! Realm, Layer, Class, ArcFamily, or ArcsSection nodes. Used by `find_parent_cursor()`
//! and `find_first_instance_cursor()` to navigate the tree hierarchy.

use super::types::CollapseKey;
use super::TaxonomyTree;

impl TaxonomyTree {
    /// Find cursor position of a Realm (does not modify collapse state).
    pub(super) fn find_realm_cursor(&self, realm_key: &str) -> Option<usize> {
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
                    if !self.is_collapsed(&CollapseKey::Layer {
                        realm: realm.key.clone(),
                        layer: layer.key.clone(),
                    }) {
                        idx += layer.classes.len();
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of a Layer (does not modify collapse state).
    pub(super) fn find_layer_cursor(&self, realm_key: &str, layer_key: &str) -> Option<usize> {
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
                    if !self.is_collapsed(&CollapseKey::Layer {
                        realm: realm.key.clone(),
                        layer: layer.key.clone(),
                    }) {
                        idx += layer.classes.len();
                    }
                }
                return None;
            }
            if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                for layer in &realm.layers {
                    idx += 1;
                    if !self.is_collapsed(&CollapseKey::Layer {
                        realm: realm.key.clone(),
                        layer: layer.key.clone(),
                    }) {
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
                    if !self.is_collapsed(&CollapseKey::Layer {
                        realm: realm.key.clone(),
                        layer: layer.key.clone(),
                    }) {
                        for class_info in &layer.classes {
                            if realm.key == realm_key
                                && layer.key == layer_key
                                && class_info.key == class_key
                            {
                                return Some(idx);
                            }
                            idx += 1;
                            // In data mode, count instances
                            if data_mode
                                && !self
                                    .is_collapsed(&CollapseKey::Class(class_info.key.clone()))
                            {
                                // Entity uses flat instances (matches tree.rs rendering)
                                if class_info.key == "Entity" {
                                    idx += self.entity_instances_flat().count();
                                } else if class_info.key == "EntityNative" {
                                    // EntityNative: count groups + expanded natives
                                    for group in &self.entity_native_groups {
                                        idx += 1; // The group itself
                                        if !self.is_collapsed(&CollapseKey::EntityGroup(
                                            group.entity_key.clone(),
                                        )) {
                                            if let Some(natives) = self
                                                .entity_native_by_entity
                                                .get(&group.entity_key)
                                            {
                                                idx += natives.len();
                                            }
                                        }
                                    }
                                } else if let Some(instances) =
                                    self.instances.get(&class_info.key)
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
    #[cfg(test)]
    pub(super) fn find_first_instance_cursor(
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
    pub(super) fn find_arcs_section_cursor(&self) -> Option<usize> {
        let mut idx = 1; // Skip ClassesSection
        if !self.is_collapsed(&CollapseKey::Classes) {
            for realm in &self.realms {
                idx += 1;
                if !self.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                    for layer in &realm.layers {
                        idx += 1;
                        if !self.is_collapsed(&CollapseKey::Layer {
                            realm: realm.key.clone(),
                            layer: layer.key.clone(),
                        }) {
                            idx += layer.classes.len();
                        }
                    }
                }
            }
        }
        Some(idx) // ArcsSection is right after all realms
    }

    /// Find cursor position of an ArcFamily.
    pub(super) fn find_family_cursor(&self, family_key: &str) -> Option<usize> {
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
}
