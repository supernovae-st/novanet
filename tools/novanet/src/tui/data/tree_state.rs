//! Collapse/expand state management for the unified tree.
//!
//! All methods operate on the `TaxonomyTree.collapsed` FxHashSet,
//! tracking which tree nodes are collapsed using string keys like
//! `"realm:shared"`, `"layer:shared:config"`, `"class:Entity"`.

use super::TaxonomyTree;

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
}
