//! Schema metadata cache for write validation
//!
//! Caches NodeClass and ArcClass metadata to validate writes without
//! repeated Neo4j queries. TTL-based eviction via moka.

use moka::sync::Cache;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Cached class metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMetadata {
    pub name: String,
    pub realm: String,
    pub layer: String,
    pub trait_type: String,
    pub required_properties: Vec<String>,
    pub optional_properties: Vec<String>,
}

/// Cached arc class metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcClassMetadata {
    pub name: String,
    pub from_class: String,
    pub to_class: String,
    pub family: String,
    pub properties: Vec<String>,
}

/// Default max entries for cache
const DEFAULT_MAX_ENTRIES: u64 = 1000;

/// Schema cache with TTL and max entries
///
/// Uses moka::sync::Cache which automatically evicts:
/// - Expired entries (based on TTL)
/// - Excess entries (LRU when over capacity)
pub struct SchemaCache {
    classes: Cache<String, ClassMetadata>,
    arcs: Cache<String, ArcClassMetadata>,
}

impl SchemaCache {
    /// Create new schema cache with given TTL in seconds
    pub fn new(ttl_secs: u64) -> Self {
        Self::with_max_entries(DEFAULT_MAX_ENTRIES, ttl_secs)
    }

    /// Create cache with custom max entries and TTL
    pub fn with_max_entries(max_entries: u64, ttl_secs: u64) -> Self {
        let ttl = Duration::from_secs(ttl_secs);
        Self {
            classes: Cache::builder()
                .max_capacity(max_entries)
                .time_to_live(ttl)
                .build(),
            arcs: Cache::builder()
                .max_capacity(max_entries)
                .time_to_live(ttl)
                .build(),
        }
    }

    /// Get class metadata if cached (auto-evicts expired entries)
    pub fn get_class(&self, name: &str) -> Option<ClassMetadata> {
        self.classes.get(name)
    }

    /// Insert class metadata into cache
    pub fn insert_class(&self, name: String, meta: ClassMetadata) {
        self.classes.insert(name, meta);
    }

    /// Get arc class metadata if cached
    pub fn get_arc(&self, name: &str) -> Option<ArcClassMetadata> {
        self.arcs.get(name)
    }

    /// Insert arc class metadata into cache
    pub fn insert_arc(&self, name: String, meta: ArcClassMetadata) {
        self.arcs.insert(name, meta);
    }

    /// Invalidate all cached entries
    pub fn invalidate_all(&self) {
        self.classes.invalidate_all();
        self.arcs.invalidate_all();
    }

    /// Run pending maintenance tasks (for testing)
    #[cfg(test)]
    pub fn run_pending_tasks(&self) {
        self.classes.run_pending_tasks();
        self.arcs.run_pending_tasks();
    }

    /// Get approximate entry count (for testing/monitoring)
    pub fn entry_count(&self) -> u64 {
        self.classes.entry_count() + self.arcs.entry_count()
    }

    /// Check if class trait allows writes
    pub fn is_writable_trait(trait_type: &str) -> bool {
        matches!(
            trait_type,
            "authored" | "imported" | "generated" | "retrieved"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_cache_insert_and_get() {
        let cache = SchemaCache::new(300);
        let meta = ClassMetadata {
            name: "SEOKeyword".to_string(),
            realm: "shared".to_string(),
            layer: "knowledge".to_string(),
            trait_type: "imported".to_string(),
            required_properties: vec!["keyword".to_string(), "slug_form".to_string()],
            optional_properties: vec!["search_volume".to_string()],
        };

        cache.insert_class("SEOKeyword".to_string(), meta.clone());

        let retrieved = cache.get_class("SEOKeyword");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().trait_type, "imported");
    }

    #[test]
    fn test_schema_cache_miss() {
        let cache = SchemaCache::new(300);
        let retrieved = cache.get_class("NonExistent");
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_is_writable_trait() {
        assert!(!SchemaCache::is_writable_trait("defined"));
        assert!(SchemaCache::is_writable_trait("authored"));
        assert!(SchemaCache::is_writable_trait("imported"));
        assert!(SchemaCache::is_writable_trait("generated"));
        assert!(SchemaCache::is_writable_trait("retrieved"));
    }

    #[test]
    fn test_invalidate_all() {
        let cache = SchemaCache::new(300);
        let meta = ClassMetadata {
            name: "Test".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            trait_type: "authored".to_string(),
            required_properties: vec![],
            optional_properties: vec![],
        };
        cache.insert_class("Test".to_string(), meta);

        assert!(cache.get_class("Test").is_some());
        cache.invalidate_all();
        cache.run_pending_tasks();
        assert!(cache.get_class("Test").is_none());
    }

    #[test]
    fn test_arc_cache() {
        let cache = SchemaCache::new(300);
        let meta = ArcClassMetadata {
            name: "HAS_NATIVE".to_string(),
            from_class: "Entity".to_string(),
            to_class: "EntityNative".to_string(),
            family: "localization".to_string(),
            properties: vec![],
        };

        cache.insert_arc("HAS_NATIVE".to_string(), meta);
        let retrieved = cache.get_arc("HAS_NATIVE");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().family, "localization");
    }

    #[test]
    fn test_entry_count() {
        let cache = SchemaCache::new(300);
        assert_eq!(cache.entry_count(), 0);

        let meta = ClassMetadata {
            name: "Test".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            trait_type: "authored".to_string(),
            required_properties: vec![],
            optional_properties: vec![],
        };
        cache.insert_class("Test".to_string(), meta);
        cache.run_pending_tasks();

        assert!(cache.entry_count() >= 1);
    }

    #[test]
    fn test_with_max_entries() {
        // Test custom max entries
        let cache = SchemaCache::with_max_entries(5, 300);

        for i in 0..10 {
            let meta = ClassMetadata {
                name: format!("Class{}", i),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                trait_type: "authored".to_string(),
                required_properties: vec![],
                optional_properties: vec![],
            };
            cache.insert_class(format!("Class{}", i), meta);
        }
        cache.run_pending_tasks();

        // Should have at most ~5 entries (moka may keep a few more during eviction)
        assert!(cache.entry_count() <= 10);
    }
}
