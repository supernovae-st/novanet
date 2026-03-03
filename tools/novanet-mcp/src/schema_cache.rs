//! Schema metadata cache for write validation
//!
//! Caches NodeClass and ArcClass metadata to validate writes without
//! repeated Neo4j queries. TTL-based invalidation.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

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

/// Schema cache with TTL
pub struct SchemaCache {
    classes: RwLock<HashMap<String, (ClassMetadata, Instant)>>,
    arcs: RwLock<HashMap<String, (ArcClassMetadata, Instant)>>,
    ttl: Duration,
}

impl SchemaCache {
    /// Create new schema cache with given TTL
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            classes: RwLock::new(HashMap::new()),
            arcs: RwLock::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    /// Get class metadata if cached and not expired
    pub fn get_class(&self, name: &str) -> Option<ClassMetadata> {
        let cache = self.classes.read();
        cache.get(name).and_then(|(meta, inserted)| {
            if inserted.elapsed() < self.ttl {
                Some(meta.clone())
            } else {
                None
            }
        })
    }

    /// Insert class metadata into cache
    pub fn insert_class(&self, name: String, meta: ClassMetadata) {
        let mut cache = self.classes.write();
        cache.insert(name, (meta, Instant::now()));
    }

    /// Get arc class metadata if cached and not expired
    pub fn get_arc(&self, name: &str) -> Option<ArcClassMetadata> {
        let cache = self.arcs.read();
        cache.get(name).and_then(|(meta, inserted)| {
            if inserted.elapsed() < self.ttl {
                Some(meta.clone())
            } else {
                None
            }
        })
    }

    /// Insert arc class metadata into cache
    pub fn insert_arc(&self, name: String, meta: ArcClassMetadata) {
        let mut cache = self.arcs.write();
        cache.insert(name, (meta, Instant::now()));
    }

    /// Invalidate all cached entries
    pub fn invalidate_all(&self) {
        self.classes.write().clear();
        self.arcs.write().clear();
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
        assert!(cache.get_class("Test").is_none());
    }
}
