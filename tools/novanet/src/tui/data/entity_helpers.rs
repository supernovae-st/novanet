//! Centralized Entity helpers for dual storage pattern.
//!
//! Entity instances can be in `entity_category_instances` (by category) OR
//! in the regular `instances` map (fallback/legacy). These helpers abstract
//! this complexity to reduce code duplication across the codebase.

use super::types::InstanceInfo;
use super::TaxonomyTree;

impl TaxonomyTree {
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
}
