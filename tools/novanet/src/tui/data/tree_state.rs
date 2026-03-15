//! Collapse/expand state management for the unified tree.
//!
//! All methods operate on the `TaxonomyTree.collapsed` FxHashSet<CollapseKey>,
//! tracking which tree nodes are collapsed using type-safe enum keys.

use super::types::CollapseKey;
use super::TaxonomyTree;

impl TaxonomyTree {
    // ========================================================================
    // Collapse/Expand methods
    // ========================================================================

    /// Check if a node is collapsed.
    pub fn is_collapsed(&self, key: &CollapseKey) -> bool {
        self.collapsed.contains(key)
    }

    /// Toggle collapse state of a node.
    pub fn toggle(&mut self, key: &CollapseKey) {
        if self.collapsed.contains(key) {
            self.collapsed.remove(key);
        } else {
            self.collapsed.insert(key.clone());
        }
    }

    /// Collapse all collapsible nodes.
    pub fn collapse_all(&mut self) {
        self.collapsed.insert(CollapseKey::Classes);
        self.collapsed.insert(CollapseKey::Arcs);
        for realm in &self.realms {
            self.collapsed
                .insert(CollapseKey::Realm(realm.key.clone()));
            for layer in &realm.layers {
                self.collapsed.insert(CollapseKey::Layer {
                    realm: realm.key.clone(),
                    layer: layer.key.clone(),
                });
            }
        }
        for family in &self.arc_families {
            self.collapsed
                .insert(CollapseKey::Family(family.key.clone()));
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
        self.collapsed.insert(CollapseKey::Arcs);

        // All layers collapsed (user opens what they need)
        for realm in &self.realms {
            for layer in &realm.layers {
                self.collapsed.insert(CollapseKey::Layer {
                    realm: realm.key.clone(),
                    layer: layer.key.clone(),
                });
            }
        }

        // All arc families collapsed
        for family in &self.arc_families {
            self.collapsed
                .insert(CollapseKey::Family(family.key.clone()));
        }

        // All classes collapsed (instances hidden)
        for realm in &self.realms {
            for layer in &realm.layers {
                for class_info in &layer.classes {
                    self.collapsed
                        .insert(CollapseKey::Class(class_info.key.clone()));
                }
            }
        }

        // Entity categories collapsed
        for cat in &self.entity_categories {
            self.collapsed
                .insert(CollapseKey::Category(cat.key.clone()));
        }

        // EntityNative groups collapsed
        for group in &self.entity_native_groups {
            self.collapsed
                .insert(CollapseKey::EntityGroup(group.entity_key.clone()));
        }
    }

    /// Expand all nodes.
    pub fn expand_all(&mut self) {
        self.collapsed.clear();
    }

    /// Expand a single node (remove from collapsed set).
    /// Unlike `expand_subtree`, this only expands the specified item.
    pub fn expand(&mut self, key: &CollapseKey) {
        self.collapsed.remove(key);
    }

    /// Collapse all Class instances (hide their instances).
    /// Used when switching between Meta and Data modes.
    pub fn collapse_all_classes(&mut self) {
        for realm in &self.realms {
            for layer in &realm.layers {
                for class_info in &layer.classes {
                    self.collapsed
                        .insert(CollapseKey::Class(class_info.key.clone()));
                }
            }
        }
    }

    /// Expand subtree under a specific key.
    /// Expands the item and all its children.
    pub fn expand_subtree(&mut self, key: &CollapseKey) {
        // Remove the key itself
        self.collapsed.remove(key);

        // Expand children based on key type
        match key {
            CollapseKey::Classes => {
                for realm in &self.realms {
                    self.collapsed.remove(&CollapseKey::Realm(realm.key.clone()));
                    for layer in &realm.layers {
                        self.collapsed.remove(&CollapseKey::Layer {
                            realm: realm.key.clone(),
                            layer: layer.key.clone(),
                        });
                        for class_info in &layer.classes {
                            self.collapsed
                                .remove(&CollapseKey::Class(class_info.key.clone()));
                        }
                    }
                }
            }
            CollapseKey::Arcs => {
                for family in &self.arc_families {
                    self.collapsed
                        .remove(&CollapseKey::Family(family.key.clone()));
                }
            }
            CollapseKey::Realm(realm_key) => {
                if let Some(realm) = self.realms.iter().find(|r| &r.key == realm_key) {
                    for layer in &realm.layers {
                        self.collapsed.remove(&CollapseKey::Layer {
                            realm: realm_key.clone(),
                            layer: layer.key.clone(),
                        });
                        for class_info in &layer.classes {
                            self.collapsed
                                .remove(&CollapseKey::Class(class_info.key.clone()));
                        }
                    }
                }
            }
            CollapseKey::Layer { realm, layer } => {
                if let Some(r) = self.realms.iter().find(|r| &r.key == realm) {
                    if let Some(l) = r.layers.iter().find(|l| &l.key == layer) {
                        for class_info in &l.classes {
                            self.collapsed
                                .remove(&CollapseKey::Class(class_info.key.clone()));
                        }
                    }
                }
            }
            // Family, Class, Category, EntityGroup: no children to expand
            CollapseKey::Family(_)
            | CollapseKey::Class(_)
            | CollapseKey::Category(_)
            | CollapseKey::EntityGroup(_) => {}
        }
    }

    /// Collapse subtree under a specific key.
    /// Collapses the item and all its children.
    pub fn collapse_subtree(&mut self, key: &CollapseKey) {
        // Collapse the key itself
        self.collapsed.insert(key.clone());

        // Collapse children based on key type
        match key {
            CollapseKey::Classes => {
                for realm in &self.realms {
                    self.collapsed
                        .insert(CollapseKey::Realm(realm.key.clone()));
                    for layer in &realm.layers {
                        self.collapsed.insert(CollapseKey::Layer {
                            realm: realm.key.clone(),
                            layer: layer.key.clone(),
                        });
                        for class_info in &layer.classes {
                            self.collapsed
                                .insert(CollapseKey::Class(class_info.key.clone()));
                        }
                    }
                }
            }
            CollapseKey::Arcs => {
                for family in &self.arc_families {
                    self.collapsed
                        .insert(CollapseKey::Family(family.key.clone()));
                }
            }
            CollapseKey::Realm(realm_key) => {
                if let Some(realm) = self.realms.iter().find(|r| &r.key == realm_key) {
                    for layer in &realm.layers {
                        self.collapsed.insert(CollapseKey::Layer {
                            realm: realm_key.clone(),
                            layer: layer.key.clone(),
                        });
                        for class_info in &layer.classes {
                            self.collapsed
                                .insert(CollapseKey::Class(class_info.key.clone()));
                        }
                    }
                }
            }
            CollapseKey::Layer { realm, layer } => {
                if let Some(r) = self.realms.iter().find(|r| &r.key == realm) {
                    if let Some(l) = r.layers.iter().find(|l| &l.key == layer) {
                        for class_info in &l.classes {
                            self.collapsed
                                .insert(CollapseKey::Class(class_info.key.clone()));
                        }
                    }
                }
            }
            // Family, Class, Category, EntityGroup: no children to collapse
            CollapseKey::Family(_)
            | CollapseKey::Class(_)
            | CollapseKey::Category(_)
            | CollapseKey::EntityGroup(_) => {}
        }
    }
}
