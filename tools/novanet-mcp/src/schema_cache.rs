//! Schema metadata cache for write validation
//!
//! Caches NodeClass and ArcClass metadata to validate writes without
//! repeated Neo4j queries. TTL-based eviction via moka.
//!
//! v0.17.0: Enhanced with ontology-driven fields for novanet_check/audit.
//! Research: G-SPEC (KG validation = 68% safety), MMKG-RDS (CSR metric).

use moka::sync::Cache;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Context budget levels for token estimation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ContextBudget {
    /// ~500 tokens
    Small,
    /// ~2000 tokens
    #[default]
    Medium,
    /// ~5000 tokens
    Large,
}

impl From<&str> for ContextBudget {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "small" => Self::Small,
            "large" => Self::Large,
            _ => Self::Medium,
        }
    }
}

/// Arc cardinality for validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArcCardinality {
    /// Exactly one target (e.g., FOR_LOCALE)
    OneToOne,
    /// One source, many targets (e.g., HAS_NATIVE)
    #[default]
    OneToMany,
    /// Many sources, many targets (e.g., USES_ENTITY)
    ManyToMany,
}

impl From<&str> for ArcCardinality {
    fn from(s: &str) -> Self {
        match s.to_lowercase().replace(':', "_").as_str() {
            "one_to_one" | "1_1" | "1:1" => Self::OneToOne,
            "many_to_many" | "n_m" | "n:m" => Self::ManyToMany,
            _ => Self::OneToMany,
        }
    }
}

/// Arc scope (intra-realm or cross-realm)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArcScope {
    /// Within same realm (shared→shared or org→org)
    #[default]
    IntraRealm,
    /// Across realms (shared↔org)
    CrossRealm,
}

impl From<&str> for ArcScope {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "cross_realm" | "crossrealm" | "cross" => Self::CrossRealm,
            _ => Self::IntraRealm,
        }
    }
}

/// Cached class metadata with ontology-driven fields
///
/// Enhanced in v0.17.0 for neuro-symbolic validation:
/// - `content`: Human-readable purpose (v0.20.0: plain WHAT+HOW)
/// - `triggers`: Keyword triggers for search boosting (v0.20.0: replaces llm_context)
/// - `schema_hint`: Agent guidance for usage
/// - `context_budget`: Token estimation hint
///
/// v0.20.0: Removed `trait_type` — traits deprecated per ADR-024 (v0.19.0).
/// Provenance is now per-instance, not per-class.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClassMetadata {
    /// Class name (e.g., "EntityNative")
    pub name: String,
    /// Realm: "shared" or "org"
    pub realm: String,
    /// Layer within realm (e.g., "semantic", "knowledge")
    pub layer: String,
    /// Properties that MUST be present for valid writes
    pub required_properties: Vec<String>,
    /// Properties that MAY be present
    pub optional_properties: Vec<String>,
    // === Ontology-driven fields (v0.17.0, updated v0.20.0) ===
    /// Human-readable content: what this class IS and how it works
    #[serde(default)]
    pub content: Option<String>,
    /// Keyword triggers for search boosting (max 10, lowercase, English)
    #[serde(default)]
    pub triggers: Option<Vec<String>>,
    /// Agent guidance for when to use this class
    #[serde(default)]
    pub schema_hint: Option<String>,
    /// Token estimation: small (~500), medium (~2000), large (~5000)
    #[serde(default)]
    pub context_budget: ContextBudget,
    /// Visibility: internal, public, restricted
    #[serde(default)]
    pub visibility: Option<String>,
}

/// Cached arc class metadata with cardinality and scope
///
/// Enhanced in v0.17.0 for arc validation:
/// - `cardinality`: 1:1, 1:N, N:M validation
/// - `scope`: intra_realm vs cross_realm
/// - `inverse_name`: For bidirectional consistency audit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArcClassMetadata {
    /// Arc class name (e.g., "HAS_NATIVE")
    pub name: String,
    /// Source class (e.g., "Entity")
    pub from_class: String,
    /// Target class (e.g., "EntityNative")
    pub to_class: String,
    /// Arc family: ownership, localization, semantic, generation, mining
    pub family: String,
    /// Properties on the arc itself
    pub properties: Vec<String>,
    // === Ontology-driven fields (v0.17.0, updated v0.20.0) ===
    /// Cardinality: one_to_one, one_to_many, many_to_many
    #[serde(default)]
    pub cardinality: ArcCardinality,
    /// Scope: intra_realm or cross_realm
    #[serde(default)]
    pub scope: ArcScope,
    /// Keyword triggers for search boosting (max 10, lowercase, English)
    #[serde(default)]
    pub triggers: Option<Vec<String>>,
    /// Human-readable content: what this arc IS and how it works
    #[serde(default)]
    pub content: Option<String>,
    /// Inverse arc name (e.g., "NATIVE_OF" for "HAS_NATIVE")
    #[serde(default)]
    pub inverse_name: Option<String>,
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

            required_properties: vec!["keyword".to_string(), "slug_form".to_string()],
            optional_properties: vec!["search_volume".to_string()],
            ..Default::default()
        };

        cache.insert_class("SEOKeyword".to_string(), meta.clone());

        let retrieved = cache.get_class("SEOKeyword");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "SEOKeyword");
    }

    #[test]
    fn test_schema_cache_miss() {
        let cache = SchemaCache::new(300);
        let retrieved = cache.get_class("NonExistent");
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_invalidate_all() {
        let cache = SchemaCache::new(300);
        let meta = ClassMetadata {
            name: "Test".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),

            required_properties: vec![],
            optional_properties: vec![],
            ..Default::default()
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
            ..Default::default()
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

            required_properties: vec![],
            optional_properties: vec![],
            ..Default::default()
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
    
                required_properties: vec![],
                optional_properties: vec![],
                ..Default::default()
            };
            cache.insert_class(format!("Class{}", i), meta);
        }
        cache.run_pending_tasks();

        // Should have at most ~5 entries (moka may keep a few more during eviction)
        assert!(cache.entry_count() <= 10);
    }

    #[test]
    fn test_ontology_fields() {
        // Test that ontology-driven fields are preserved through cache
        let cache = SchemaCache::new(300);
        let meta = ClassMetadata {
            name: "EntityNative".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            required_properties: vec!["key".to_string()],
            optional_properties: vec![],
            content: Some("LLM-generated locale-native content".to_string()),
            triggers: Some(vec!["localized".to_string(), "entity".to_string(), "native".to_string()]),
            schema_hint: Some("Load via HAS_NATIVE from Entity".to_string()),
            context_budget: ContextBudget::Medium,
            visibility: Some("public".to_string()),
        };

        cache.insert_class("EntityNative".to_string(), meta);

        let retrieved = cache.get_class("EntityNative").unwrap();
        assert_eq!(
            retrieved.content,
            Some("LLM-generated locale-native content".to_string())
        );
        assert_eq!(retrieved.triggers.as_ref().unwrap().len(), 3);
        assert_eq!(retrieved.context_budget, ContextBudget::Medium);
    }

    #[test]
    fn test_arc_ontology_fields() {
        // Test that arc ontology-driven fields are preserved through cache
        let cache = SchemaCache::new(300);
        let meta = ArcClassMetadata {
            name: "HAS_NATIVE".to_string(),
            from_class: "Entity".to_string(),
            to_class: "EntityNative".to_string(),
            family: "localization".to_string(),
            properties: vec![],
            cardinality: ArcCardinality::OneToMany,
            scope: ArcScope::IntraRealm,
            triggers: Some(vec!["locale".to_string(), "content".to_string(), "native".to_string()]),
            content: Some("Links entity to its native content".to_string()),
            inverse_name: Some("NATIVE_OF".to_string()),
        };

        cache.insert_arc("HAS_NATIVE".to_string(), meta);

        let retrieved = cache.get_arc("HAS_NATIVE").unwrap();
        assert_eq!(retrieved.cardinality, ArcCardinality::OneToMany);
        assert_eq!(retrieved.scope, ArcScope::IntraRealm);
        assert_eq!(retrieved.inverse_name, Some("NATIVE_OF".to_string()));
    }
}
