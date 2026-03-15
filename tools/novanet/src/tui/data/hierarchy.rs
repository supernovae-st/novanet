//! Hierarchy position calculation for tree status display.
//!
//! Computes R:realm L:layer C:class I:instance coordinates for any tree item,
//! used by the status bar to show compact position info like "R:1/2 L:2/4 C:3/7".

use super::conversion::INSTANCE_LIMIT;
use super::types::*;
use super::TaxonomyTree;

impl TaxonomyTree {
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

        match item {
            None | Some(TreeItem::ClassesSection) | Some(TreeItem::ArcsSection) => {
                HierarchyPosition::default()
            }
            Some(TreeItem::Realm(realm)) => self.realm_position(realm),
            Some(TreeItem::Layer(realm, layer)) => self.layer_position(realm, layer),
            Some(TreeItem::Class(realm, layer, class_info)) => {
                self.class_position(realm, layer, class_info)
            }
            Some(TreeItem::Instance(realm, layer, class_info, _)) => {
                self.instance_position(realm, layer, class_info)
            }
            Some(TreeItem::EntityCategory(realm, layer, class_info, _)) => {
                self.class_position(realm, layer, class_info)
            }
            Some(TreeItem::EntityGroup(realm, layer, class_info, _)) => {
                self.class_position(realm, layer, class_info)
            }
            Some(TreeItem::ArcFamily(_)) | Some(TreeItem::ArcClass(_, _)) => {
                HierarchyPosition::default()
            }
            Some(TreeItem::EntityNativeItem(realm, layer, class_info, _)) => {
                self.entity_native_item_position(realm, layer, class_info)
            }
        }
    }

    // =========================================================================
    // Position helpers (private)
    // =========================================================================

    /// Position for a Realm node: R:x/total.
    fn realm_position(&self, realm: &RealmInfo) -> HierarchyPosition {
        let realm_idx = self
            .realms
            .iter()
            .position(|r| r.key == realm.key)
            .map(|i| i + 1)
            .unwrap_or(1);
        HierarchyPosition {
            realm: Some((realm_idx, self.realms.len())),
            ..Default::default()
        }
    }

    /// Position for a Layer node: R:x/total L:y/total.
    fn layer_position(&self, realm: &RealmInfo, layer: &LayerInfo) -> HierarchyPosition {
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
            realm: Some((realm_idx, self.realms.len())),
            layer: Some((layer_idx, realm.layers.len())),
            ..Default::default()
        }
    }

    /// Position for a Class node (also used for EntityCategory/EntityGroup):
    /// R:x/total L:y/total C:z/total.
    fn class_position(
        &self,
        realm: &RealmInfo,
        layer: &LayerInfo,
        class_info: &ClassInfo,
    ) -> HierarchyPosition {
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
            realm: Some((realm_idx, self.realms.len())),
            layer: Some((layer_idx, realm.layers.len())),
            class: Some((class_idx, layer.classes.len())),
            ..Default::default()
        }
    }

    /// Position for an Instance node: R:x L:y C:z I:loaded/total.
    fn instance_position(
        &self,
        realm: &RealmInfo,
        layer: &LayerInfo,
        class_info: &ClassInfo,
    ) -> HierarchyPosition {
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
            realm: Some((realm_idx, self.realms.len())),
            layer: Some((layer_idx, realm.layers.len())),
            class: Some((class_idx, layer.classes.len())),
            instance: Some((loaded_count.min(INSTANCE_LIMIT), total_instances)),
        }
    }

    /// Position for an EntityNativeItem: R:x L:y C:z I:loaded/total.
    fn entity_native_item_position(
        &self,
        realm: &RealmInfo,
        layer: &LayerInfo,
        class_info: &ClassInfo,
    ) -> HierarchyPosition {
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
            realm: Some((realm_idx, self.realms.len())),
            layer: Some((layer_idx, realm.layers.len())),
            class: Some((class_idx, layer.classes.len())),
            instance: Some((loaded_count.min(INSTANCE_LIMIT), loaded_count)),
        }
    }
}
