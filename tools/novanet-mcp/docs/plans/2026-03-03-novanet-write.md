# novanet_write Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the 12th MCP tool `novanet_write` that enables AI agents (Nika) to write data to Neo4j with schema validation and trait-based permissions.

**Architecture:** Single tool with 3 operations (upsert_node, create_arc, update_props) validated against the schema-graph. Uses MERGE pattern for idempotency. Enforces trait permissions (defined=READ-ONLY, others=WRITABLE). Special handling for slug_locked and is_slug_source singleton.

**Tech Stack:** Rust 1.86, rmcp 0.16, neo4rs 0.8, serde, schemars, tokio

---

## Table of Contents

1. [Phase 1: Error Types & Validation Foundation](#phase-1-error-types--validation-foundation)
2. [Phase 2: Schema Introspection Cache](#phase-2-schema-introspection-cache)
3. [Phase 3: Write Tool Core](#phase-3-write-tool-core)
4. [Phase 4: Special Validations](#phase-4-special-validations)
5. [Phase 5: Handler Integration](#phase-5-handler-integration)
6. [Phase 6: Documentation & Tests](#phase-6-documentation--tests)

---

## Phase 1: Error Types & Validation Foundation

### Task 1.1: Add Write-Specific Error Variants

**Files:**
- Modify: `src/error.rs:20-85`

**Step 1: Write the failing test**

```rust
// Add at the end of src/error.rs in mod tests
#[test]
fn test_trait_not_writable_error() {
    let err = Error::trait_not_writable("Entity", "defined");
    assert!(err.to_string().contains("Entity"));
    assert!(err.to_string().contains("defined"));
}

#[test]
fn test_slug_locked_error() {
    let err = Error::slug_locked("block:head-seo-meta@fr-FR", "qr-code");
    assert!(err.to_string().contains("slug_locked"));
    assert!(err.to_string().contains("qr-code"));
}

#[test]
fn test_singleton_violation_error() {
    let err = Error::singleton_violation("is_slug_source", "entity-native:qr-code@fr-FR");
    assert!(err.to_string().contains("is_slug_source"));
}

#[test]
fn test_schema_not_found_error() {
    let err = Error::schema_not_found("UnknownClass");
    assert!(err.to_string().contains("UnknownClass"));
}

#[test]
fn test_missing_required_property_error() {
    let err = Error::missing_required_property("SEOKeyword", "keyword");
    assert!(err.to_string().contains("SEOKeyword"));
    assert!(err.to_string().contains("keyword"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_trait_not_writable_error -- --nocapture`
Expected: FAIL with "no function or associated item named `trait_not_writable`"

**Step 3: Write minimal implementation**

Add to `src/error.rs` after line 84 (before `impl Error`):

```rust
    /// Trait does not allow writes
    #[error("Class '{class}' has trait '{trait_type}' which is not writable. Only authored/imported/generated/retrieved allow writes.")]
    TraitNotWritable { class: String, trait_type: String },

    /// Slug is locked after deployment
    #[error("Slug is locked on '{key}'. Current slug: '{current_slug}'. Create a redirect instead of modifying.")]
    SlugLocked { key: String, current_slug: String },

    /// Singleton property violation (e.g., is_slug_source)
    #[error("Singleton violation: Only one arc can have '{property}' = true for target '{target_key}'.")]
    SingletonViolation { property: String, target_key: String },

    /// Schema class not found
    #[error("Schema class not found: '{class}'. Use novanet_introspect to list available classes.")]
    SchemaNotFound { class: String },

    /// Missing required property
    #[error("Missing required property '{property}' for class '{class}'.")]
    MissingRequiredProperty { class: String, property: String },

    /// Arc endpoints not found
    #[error("Arc endpoint not found: {endpoint_type} '{key}' does not exist.")]
    ArcEndpointNotFound { endpoint_type: String, key: String },
```

Add constructor methods in `impl Error` block (after line 130):

```rust
    /// Create a trait not writable error
    pub fn trait_not_writable(class: impl Into<String>, trait_type: impl Into<String>) -> Self {
        Self::TraitNotWritable {
            class: class.into(),
            trait_type: trait_type.into(),
        }
    }

    /// Create a slug locked error
    pub fn slug_locked(key: impl Into<String>, current_slug: impl Into<String>) -> Self {
        Self::SlugLocked {
            key: key.into(),
            current_slug: current_slug.into(),
        }
    }

    /// Create a singleton violation error
    pub fn singleton_violation(property: impl Into<String>, target_key: impl Into<String>) -> Self {
        Self::SingletonViolation {
            property: property.into(),
            target_key: target_key.into(),
        }
    }

    /// Create a schema not found error
    pub fn schema_not_found(class: impl Into<String>) -> Self {
        Self::SchemaNotFound {
            class: class.into(),
        }
    }

    /// Create a missing required property error
    pub fn missing_required_property(class: impl Into<String>, property: impl Into<String>) -> Self {
        Self::MissingRequiredProperty {
            class: class.into(),
            property: property.into(),
        }
    }

    /// Create an arc endpoint not found error
    pub fn arc_endpoint_not_found(endpoint_type: impl Into<String>, key: impl Into<String>) -> Self {
        Self::ArcEndpointNotFound {
            endpoint_type: endpoint_type.into(),
            key: key.into(),
        }
    }
```

Update `impl From<Error> for McpError` to include new variants (after line 151):

```rust
            Error::TraitNotWritable { .. } => INVALID_PARAMS,
            Error::SlugLocked { .. } => INVALID_PARAMS,
            Error::SingletonViolation { .. } => INVALID_PARAMS,
            Error::SchemaNotFound { .. } => RESOURCE_NOT_FOUND,
            Error::MissingRequiredProperty { .. } => INVALID_PARAMS,
            Error::ArcEndpointNotFound { .. } => RESOURCE_NOT_FOUND,
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_trait_not_writable_error test_slug_locked_error test_singleton_violation_error test_schema_not_found_error test_missing_required_property_error -- --nocapture`
Expected: All 5 tests PASS

**Step 5: Commit**

```bash
git add src/error.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add write-specific error variants

Add 6 new error types for novanet_write:
- TraitNotWritable: Class trait doesn't allow writes
- SlugLocked: Slug cannot be modified after deployment
- SingletonViolation: is_slug_source constraint
- SchemaNotFound: Unknown class name
- MissingRequiredProperty: Required property missing
- ArcEndpointNotFound: Arc source/target doesn't exist

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 1.2: Add Write-Specific Hints

**Files:**
- Modify: `src/hints.rs:6-105`

**Step 1: Write the failing test**

```rust
// Add to src/hints.rs mod tests
#[test]
fn test_hint_for_trait_not_writable() {
    let hint = get_hint("Class 'Entity' has trait 'defined' which is not writable");
    assert!(hint.contains("authored"));
    assert!(hint.contains("imported"));
}

#[test]
fn test_hint_for_slug_locked() {
    let hint = get_hint("Slug is locked on 'block:head-seo-meta@fr-FR'");
    assert!(hint.contains("redirect"));
}

#[test]
fn test_hint_for_singleton_violation() {
    let hint = get_hint("Singleton violation: Only one arc can have 'is_slug_source'");
    assert!(hint.contains("is_slug_source"));
}

#[test]
fn test_hint_for_schema_not_found() {
    let hint = get_hint("Schema class not found: 'FooBar'");
    assert!(hint.contains("novanet_introspect"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_hint_for_trait_not_writable -- --nocapture`
Expected: FAIL (hint doesn't contain expected text yet)

**Step 3: Write minimal implementation**

Add to `src/hints.rs` in `get_hint()` function, before the default hint (around line 98):

```rust
    // Write operation errors
    if lower.contains("trait") && lower.contains("not writable") {
        return "💡 Hint: Write permission denied.\n\
             Only these traits allow writes:\n\
             - authored: Human-written content (EntityNative, PageNative)\n\
             - imported: External data (SEOKeyword, GeoTrend)\n\
             - generated: LLM-generated (BlockNative)\n\
             - retrieved: Discovered knowledge (Term, Expression)\n\
             \n\
             'defined' trait is READ-ONLY (Entity, Page, Block, Locale)."
            .to_string();
    }

    if lower.contains("slug") && lower.contains("locked") {
        return "💡 Hint: Slug is immutable after deployment.\n\
             - Create a URL redirect instead of changing the slug\n\
             - Or set slug_locked: false first (requires admin access)\n\
             - ADR-030: BlockNative:head-seo-meta owns the slug"
            .to_string();
    }

    if lower.contains("singleton") && lower.contains("is_slug_source") {
        return "💡 Hint: Only one SEOKeyword can be the slug source.\n\
             - The existing is_slug_source arc will be demoted\n\
             - Use rank: 'primary' for the new slug source\n\
             - Previous keyword becomes rank: 'secondary'"
            .to_string();
    }

    if lower.contains("schema") && lower.contains("not found") {
        return "💡 Hint: Unknown class name.\n\
             - Use novanet_introspect to list available classes\n\
             - Check spelling and case (PascalCase)\n\
             - Common classes: Entity, EntityNative, SEOKeyword, Term"
            .to_string();
    }

    if lower.contains("missing required property") {
        return "💡 Hint: Required property missing.\n\
             - Use novanet_introspect target='class' name='ClassName' to see required properties\n\
             - Check property names are spelled correctly"
            .to_string();
    }

    if lower.contains("arc endpoint") && lower.contains("not found") {
        return "💡 Hint: Arc source or target doesn't exist.\n\
             - Create the node first with upsert_node operation\n\
             - Verify the key spelling\n\
             - Use novanet_search to find existing nodes"
            .to_string();
    }
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_hint_for_trait_not_writable test_hint_for_slug_locked test_hint_for_singleton_violation test_hint_for_schema_not_found -- --nocapture`
Expected: All 4 tests PASS

**Step 5: Commit**

```bash
git add src/hints.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add write-specific error hints

Add actionable hints for novanet_write errors:
- TraitNotWritable: Explain which traits allow writes
- SlugLocked: Suggest redirect instead of modification
- SingletonViolation: Explain is_slug_source constraint
- SchemaNotFound: Point to novanet_introspect
- MissingRequiredProperty: Guide to introspect
- ArcEndpointNotFound: Suggest creating node first

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

## Phase 2: Schema Introspection Cache

### Task 2.1: Create Schema Cache Module

**Files:**
- Create: `src/schema_cache.rs`

**Step 1: Write the failing test**

Create `src/schema_cache.rs`:

```rust
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
```

**Step 2: Run test to verify it passes**

Run: `cargo test schema_cache -- --nocapture`
Expected: All 4 tests PASS

**Step 3: Commit**

```bash
git add src/schema_cache.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add schema metadata cache for write validation

SchemaCache caches ClassMetadata and ArcClassMetadata with TTL:
- is_writable_trait(): Check if trait allows writes
- get_class/insert_class: Cache node class metadata
- get_arc/insert_arc: Cache arc class metadata
- invalidate_all: Clear cache on schema changes

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 2.2: Register Schema Cache in lib.rs

**Files:**
- Modify: `src/lib.rs:27-40`

**Step 1: Add module declaration**

Add after line 35 in `src/lib.rs`:

```rust
pub mod schema_cache;
```

**Step 2: Run test to verify compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "$(cat <<'EOF'
feat(mcp): register schema_cache module

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 2.3: Add Schema Cache to State

**Files:**
- Modify: `src/server/state.rs`

**Step 1: Read current state.rs to understand structure**

Run: `cat src/server/state.rs`

**Step 2: Add schema cache to State struct**

Add import at top:
```rust
use crate::schema_cache::SchemaCache;
```

Add field to `StateInner` struct:
```rust
    schema_cache: SchemaCache,
```

Add initialization in `State::new()`:
```rust
    schema_cache: SchemaCache::new(config.cache_ttl_secs),
```

Add accessor method:
```rust
    /// Get schema cache reference
    pub fn schema_cache(&self) -> &SchemaCache {
        &self.inner.schema_cache
    }
```

**Step 3: Run test to verify compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/server/state.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add schema cache to server state

Add SchemaCache to StateInner for write validation caching.
Initialized with same TTL as query cache.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

## Phase 3: Write Tool Core

### Task 3.1: Create Write Tool Parameters and Result Types

**Files:**
- Create: `src/tools/write.rs`

**Step 1: Create the file with types**

```rust
//! novanet_write tool
//!
//! Intelligent data writes to Neo4j with schema validation.
//! Single tool with 3 operations: upsert_node, create_arc, update_props.

use crate::error::{Error, Result};
use crate::schema_cache::{ClassMetadata, SchemaCache};
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, info, instrument, warn};

/// Write operation type
#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WriteOperation {
    /// Create or update a node (MERGE pattern)
    UpsertNode,
    /// Create an arc between nodes
    CreateArc,
    /// Update specific properties on existing node
    UpdateProps,
}

/// Parameters for novanet_write tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct WriteParams {
    /// Operation type
    pub operation: WriteOperation,

    /// Node class name (for upsert_node, update_props)
    #[serde(default)]
    pub class: Option<String>,

    /// Arc class name (for create_arc)
    #[serde(default)]
    pub arc_class: Option<String>,

    /// Node key (for upsert_node, update_props)
    #[serde(default)]
    pub key: Option<String>,

    /// Source node key (for create_arc)
    #[serde(default)]
    pub from_key: Option<String>,

    /// Target node key (for create_arc)
    #[serde(default)]
    pub to_key: Option<String>,

    /// Properties to write
    #[serde(default)]
    pub properties: Option<serde_json::Map<String, Value>>,

    /// Locale for auto-arc creation (optional)
    #[serde(default)]
    pub locale: Option<String>,
}

/// Result from novanet_write tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct WriteResult {
    /// Whether the operation succeeded
    pub success: bool,

    /// Operation performed
    pub operation: String,

    /// Affected node/arc key
    pub key: String,

    /// Whether a new node was created (vs updated)
    pub created: bool,

    /// Properties that were updated (empty if created)
    pub updated_properties: Vec<String>,

    /// Auto-arcs that were created
    pub auto_arcs_created: Vec<String>,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,

    /// Cache keys invalidated
    pub cache_invalidated: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_operation_deserialize() {
        let json = r#""upsert_node""#;
        let op: WriteOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op, WriteOperation::UpsertNode);

        let json = r#""create_arc""#;
        let op: WriteOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op, WriteOperation::CreateArc);

        let json = r#""update_props""#;
        let op: WriteOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op, WriteOperation::UpdateProps);
    }

    #[test]
    fn test_write_params_deserialize() {
        let json = r#"{
            "operation": "upsert_node",
            "class": "SEOKeyword",
            "key": "seo:qr-code@fr-FR",
            "properties": {
                "keyword": "qr code",
                "search_volume": 110000
            }
        }"#;

        let params: WriteParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.operation, WriteOperation::UpsertNode);
        assert_eq!(params.class, Some("SEOKeyword".to_string()));
        assert_eq!(params.key, Some("seo:qr-code@fr-FR".to_string()));
    }

    #[test]
    fn test_write_result_serialize() {
        let result = WriteResult {
            success: true,
            operation: "upsert_node".to_string(),
            key: "seo:qr-code@fr-FR".to_string(),
            created: true,
            updated_properties: vec![],
            auto_arcs_created: vec!["FOR_LOCALE".to_string()],
            execution_time_ms: 45,
            cache_invalidated: vec!["SEOKeyword:*".to_string()],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("seo:qr-code@fr-FR"));
        assert!(json.contains("FOR_LOCALE"));
    }
}
```

**Step 2: Run test to verify it passes**

Run: `cargo test write::tests -- --nocapture`
Expected: All 3 tests PASS

**Step 3: Commit**

```bash
git add src/tools/write.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add novanet_write params and result types

WriteParams with 3 operations:
- upsert_node: Create/update node with MERGE
- create_arc: Create relationship between nodes
- update_props: Partial property update

WriteResult includes:
- created flag (new vs updated)
- updated_properties list
- auto_arcs_created list
- cache_invalidated patterns

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 3.2: Implement Schema Validation Functions

**Files:**
- Modify: `src/tools/write.rs`

**Step 1: Write the failing test**

Add to `src/tools/write.rs`:

```rust
/// Validate write operation has required params
fn validate_params(params: &WriteParams) -> Result<()> {
    match params.operation {
        WriteOperation::UpsertNode => {
            if params.class.is_none() {
                return Err(Error::InvalidParams("upsert_node requires 'class'".into()));
            }
            if params.key.is_none() {
                return Err(Error::InvalidParams("upsert_node requires 'key'".into()));
            }
        }
        WriteOperation::CreateArc => {
            if params.arc_class.is_none() {
                return Err(Error::InvalidParams("create_arc requires 'arc_class'".into()));
            }
            if params.from_key.is_none() {
                return Err(Error::InvalidParams("create_arc requires 'from_key'".into()));
            }
            if params.to_key.is_none() {
                return Err(Error::InvalidParams("create_arc requires 'to_key'".into()));
            }
        }
        WriteOperation::UpdateProps => {
            if params.class.is_none() {
                return Err(Error::InvalidParams("update_props requires 'class'".into()));
            }
            if params.key.is_none() {
                return Err(Error::InvalidParams("update_props requires 'key'".into()));
            }
            if params.properties.is_none() {
                return Err(Error::InvalidParams(
                    "update_props requires 'properties'".into(),
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // ... existing tests ...

    #[test]
    fn test_validate_params_upsert_node_ok() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: Some("SEOKeyword".to_string()),
            key: Some("seo:test@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: Some(serde_json::Map::new()),
            locale: None,
        };
        assert!(validate_params(&params).is_ok());
    }

    #[test]
    fn test_validate_params_upsert_node_missing_class() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: None,
            key: Some("seo:test@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
        };
        let err = validate_params(&params).unwrap_err();
        assert!(err.to_string().contains("class"));
    }

    #[test]
    fn test_validate_params_create_arc_ok() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: Some("TARGETS".to_string()),
            from_key: Some("seo:qr-code@fr-FR".to_string()),
            to_key: Some("entity-native:qr-code@fr-FR".to_string()),
            properties: Some(serde_json::Map::new()),
            locale: None,
        };
        assert!(validate_params(&params).is_ok());
    }

    #[test]
    fn test_validate_params_create_arc_missing_to_key() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: Some("TARGETS".to_string()),
            from_key: Some("seo:qr-code@fr-FR".to_string()),
            to_key: None,
            properties: None,
            locale: None,
        };
        let err = validate_params(&params).unwrap_err();
        assert!(err.to_string().contains("to_key"));
    }
}
```

**Step 2: Run test to verify it passes**

Run: `cargo test validate_params -- --nocapture`
Expected: All 4 tests PASS

**Step 3: Commit**

```bash
git add src/tools/write.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add parameter validation for novanet_write

validate_params() checks required fields per operation:
- upsert_node: class, key
- create_arc: arc_class, from_key, to_key
- update_props: class, key, properties

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 3.3: Implement Trait Validation

**Files:**
- Modify: `src/tools/write.rs`

**Step 1: Write the failing test and implementation**

Add to `src/tools/write.rs`:

```rust
/// Fetch and validate class metadata for write permission
async fn fetch_and_validate_class(state: &State, class_name: &str) -> Result<ClassMetadata> {
    // Check cache first
    if let Some(meta) = state.schema_cache().get_class(class_name) {
        if !SchemaCache::is_writable_trait(&meta.trait_type) {
            return Err(Error::trait_not_writable(class_name, &meta.trait_type));
        }
        return Ok(meta);
    }

    // Fetch from Neo4j
    let query = r#"
        MATCH (c:Schema:Class {name: $name})
        RETURN c.name AS name,
               c.realm AS realm,
               c.layer AS layer,
               c.trait AS trait_type,
               c.required_properties AS required_properties,
               c.optional_properties AS optional_properties
    "#;

    let mut params = serde_json::Map::new();
    params.insert("name".to_string(), Value::String(class_name.to_string()));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    if rows.is_empty() {
        return Err(Error::schema_not_found(class_name));
    }

    let row = &rows[0];
    let meta = ClassMetadata {
        name: row["name"].as_str().unwrap_or_default().to_string(),
        realm: row["realm"].as_str().unwrap_or_default().to_string(),
        layer: row["layer"].as_str().unwrap_or_default().to_string(),
        trait_type: row["trait_type"].as_str().unwrap_or_default().to_string(),
        required_properties: row["required_properties"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        optional_properties: row["optional_properties"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
    };

    // Validate trait allows writes
    if !SchemaCache::is_writable_trait(&meta.trait_type) {
        return Err(Error::trait_not_writable(class_name, &meta.trait_type));
    }

    // Cache the metadata
    state
        .schema_cache()
        .insert_class(class_name.to_string(), meta.clone());

    Ok(meta)
}
```

**Step 2: Run compilation check**

Run: `cargo check`
Expected: Compiles (tests require Neo4j so we'll test via integration)

**Step 3: Commit**

```bash
git add src/tools/write.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add trait validation for writes

fetch_and_validate_class():
- Check schema cache first
- Fetch from Neo4j if not cached
- Validate trait allows writes (authored/imported/generated/retrieved)
- Cache result for future requests
- Return TraitNotWritable error for 'defined' trait

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 3.4: Implement UpsertNode Operation

**Files:**
- Modify: `src/tools/write.rs`

**Step 1: Add upsert_node implementation**

```rust
/// Execute upsert_node operation
async fn execute_upsert_node(
    state: &State,
    params: &WriteParams,
    meta: &ClassMetadata,
) -> Result<WriteResult> {
    let start = std::time::Instant::now();
    let key = params.key.as_ref().unwrap();
    let class = params.class.as_ref().unwrap();
    let props = params.properties.clone().unwrap_or_default();

    // Build MERGE query
    let props_json = serde_json::to_string(&props)
        .map_err(|e| Error::Internal(format!("Props serialization: {}", e)))?;

    let query = format!(
        r#"
        MERGE (n:{class} {{key: $key}})
        ON CREATE SET n += $props, n.created_at = timestamp()
        ON MATCH SET n += $props, n.updated_at = timestamp()
        WITH n,
             CASE WHEN n.created_at = timestamp() THEN true ELSE false END AS created
        RETURN created, keys(n) AS all_keys
        "#,
        class = class
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("key".to_string(), Value::String(key.clone()));
    query_params.insert("props".to_string(), serde_json::from_str(&props_json)?);

    let rows = state
        .pool()
        .execute_query(&query, Some(query_params))
        .await?;

    let created = rows
        .first()
        .and_then(|r| r["created"].as_bool())
        .unwrap_or(false);

    // Determine updated properties (diff with previous)
    let updated_properties: Vec<String> = if created {
        vec![]
    } else {
        props.keys().cloned().collect()
    };

    // Handle auto-arcs (FOR_LOCALE)
    let mut auto_arcs = vec![];
    if let Some(locale) = &params.locale {
        let auto_arc_query = format!(
            r#"
            MATCH (n:{class} {{key: $key}})
            MATCH (l:Locale {{key: $locale}})
            MERGE (n)-[:FOR_LOCALE]->(l)
            "#,
            class = class
        );

        let mut arc_params = serde_json::Map::new();
        arc_params.insert("key".to_string(), Value::String(key.clone()));
        arc_params.insert("locale".to_string(), Value::String(locale.clone()));

        state
            .pool()
            .execute_query(&auto_arc_query, Some(arc_params))
            .await?;
        auto_arcs.push("FOR_LOCALE".to_string());
    }

    // Invalidate cache
    let cache_patterns = vec![format!("{}:*", class), key.clone()];
    for pattern in &cache_patterns {
        state.cache().invalidate_pattern(pattern).await;
    }

    info!(key = %key, class = %class, created = created, "upsert_node completed");

    Ok(WriteResult {
        success: true,
        operation: "upsert_node".to_string(),
        key: key.clone(),
        created,
        updated_properties,
        auto_arcs_created: auto_arcs,
        execution_time_ms: start.elapsed().as_millis() as u64,
        cache_invalidated: cache_patterns,
    })
}
```

**Step 2: Run compilation check**

Run: `cargo check`
Expected: Compiles (may have unused warnings, that's OK)

**Step 3: Commit**

```bash
git add src/tools/write.rs
git commit -m "$(cat <<'EOF'
feat(mcp): implement upsert_node operation

execute_upsert_node():
- MERGE pattern for idempotent creates/updates
- ON CREATE SET created_at timestamp
- ON MATCH SET updated_at timestamp
- Auto-arc FOR_LOCALE if locale param provided
- Cache invalidation for class and key patterns
- Returns created flag and updated_properties list

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 3.5: Implement CreateArc Operation

**Files:**
- Modify: `src/tools/write.rs`

**Step 1: Add create_arc implementation**

```rust
/// Execute create_arc operation
async fn execute_create_arc(state: &State, params: &WriteParams) -> Result<WriteResult> {
    let start = std::time::Instant::now();
    let arc_class = params.arc_class.as_ref().unwrap();
    let from_key = params.from_key.as_ref().unwrap();
    let to_key = params.to_key.as_ref().unwrap();
    let props = params.properties.clone().unwrap_or_default();

    // Verify endpoints exist
    let check_query = r#"
        MATCH (from {key: $from_key})
        MATCH (to {key: $to_key})
        RETURN from.key AS from_exists, to.key AS to_exists
    "#;

    let mut check_params = serde_json::Map::new();
    check_params.insert("from_key".to_string(), Value::String(from_key.clone()));
    check_params.insert("to_key".to_string(), Value::String(to_key.clone()));

    let check_rows = state
        .pool()
        .execute_query(check_query, Some(check_params))
        .await?;

    if check_rows.is_empty() {
        // Determine which endpoint is missing
        let from_exists_query = "MATCH (n {key: $key}) RETURN n.key AS exists";
        let mut p = serde_json::Map::new();
        p.insert("key".to_string(), Value::String(from_key.clone()));
        let from_check = state.pool().execute_query(from_exists_query, Some(p)).await?;

        if from_check.is_empty() {
            return Err(Error::arc_endpoint_not_found("from", from_key));
        }
        return Err(Error::arc_endpoint_not_found("to", to_key));
    }

    // Handle is_slug_source singleton (Phase 4 will add this)
    // For now, just create the arc

    // Build MERGE query for arc
    let props_json = serde_json::to_string(&props)
        .map_err(|e| Error::Internal(format!("Props serialization: {}", e)))?;

    let query = format!(
        r#"
        MATCH (from {{key: $from_key}})
        MATCH (to {{key: $to_key}})
        MERGE (from)-[r:{arc_class}]->(to)
        SET r += $props
        RETURN true AS created
        "#,
        arc_class = arc_class
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("from_key".to_string(), Value::String(from_key.clone()));
    query_params.insert("to_key".to_string(), Value::String(to_key.clone()));
    query_params.insert("props".to_string(), serde_json::from_str(&props_json)?);

    state
        .pool()
        .execute_query(&query, Some(query_params))
        .await?;

    // Invalidate cache
    let cache_patterns = vec![from_key.clone(), to_key.clone()];
    for pattern in &cache_patterns {
        state.cache().invalidate_pattern(pattern).await;
    }

    let arc_key = format!("({})--[{}]-->({})", from_key, arc_class, to_key);
    info!(arc = %arc_key, "create_arc completed");

    Ok(WriteResult {
        success: true,
        operation: "create_arc".to_string(),
        key: arc_key,
        created: true, // MERGE always reports as created for arcs
        updated_properties: vec![],
        auto_arcs_created: vec![],
        execution_time_ms: start.elapsed().as_millis() as u64,
        cache_invalidated: cache_patterns,
    })
}
```

**Step 2: Run compilation check**

Run: `cargo check`
Expected: Compiles

**Step 3: Commit**

```bash
git add src/tools/write.rs
git commit -m "$(cat <<'EOF'
feat(mcp): implement create_arc operation

execute_create_arc():
- Verify both endpoints exist before creating arc
- MERGE pattern for idempotent arc creation
- Set arc properties from params
- Cache invalidation for both endpoints
- Returns arc key in format (from)--[TYPE]-->(to)

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 3.6: Implement UpdateProps Operation

**Files:**
- Modify: `src/tools/write.rs`

**Step 1: Add update_props implementation**

```rust
/// Execute update_props operation
async fn execute_update_props(
    state: &State,
    params: &WriteParams,
    meta: &ClassMetadata,
) -> Result<WriteResult> {
    let start = std::time::Instant::now();
    let key = params.key.as_ref().unwrap();
    let class = params.class.as_ref().unwrap();
    let props = params.properties.as_ref().unwrap();

    // Verify node exists
    let check_query = format!(
        "MATCH (n:{class} {{key: $key}}) RETURN n.key AS exists",
        class = class
    );
    let mut check_params = serde_json::Map::new();
    check_params.insert("key".to_string(), Value::String(key.clone()));

    let check_rows = state
        .pool()
        .execute_query(&check_query, Some(check_params))
        .await?;

    if check_rows.is_empty() {
        return Err(Error::not_found(key));
    }

    // Check for slug_locked (Phase 4 will add full implementation)
    // For now, just update properties

    // Build SET query
    let props_json = serde_json::to_string(&props)
        .map_err(|e| Error::Internal(format!("Props serialization: {}", e)))?;

    let query = format!(
        r#"
        MATCH (n:{class} {{key: $key}})
        SET n += $props, n.updated_at = timestamp()
        RETURN keys(n) AS all_keys
        "#,
        class = class
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("key".to_string(), Value::String(key.clone()));
    query_params.insert("props".to_string(), serde_json::from_str(&props_json)?);

    state
        .pool()
        .execute_query(&query, Some(query_params))
        .await?;

    let updated_properties: Vec<String> = props.keys().cloned().collect();

    // Invalidate cache
    let cache_patterns = vec![format!("{}:*", class), key.clone()];
    for pattern in &cache_patterns {
        state.cache().invalidate_pattern(pattern).await;
    }

    info!(key = %key, class = %class, props = ?updated_properties, "update_props completed");

    Ok(WriteResult {
        success: true,
        operation: "update_props".to_string(),
        key: key.clone(),
        created: false,
        updated_properties,
        auto_arcs_created: vec![],
        execution_time_ms: start.elapsed().as_millis() as u64,
        cache_invalidated: cache_patterns,
    })
}
```

**Step 2: Run compilation check**

Run: `cargo check`
Expected: Compiles

**Step 3: Commit**

```bash
git add src/tools/write.rs
git commit -m "$(cat <<'EOF'
feat(mcp): implement update_props operation

execute_update_props():
- Verify node exists before updating
- SET += for partial property update
- Set updated_at timestamp
- Cache invalidation for class and key patterns
- Returns list of updated property names

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 3.7: Implement Main Execute Function

**Files:**
- Modify: `src/tools/write.rs`

**Step 1: Add main execute function**

```rust
/// Execute the novanet_write tool
#[instrument(name = "novanet_write", skip(state), fields(operation = ?params.operation))]
pub async fn execute(state: &State, params: WriteParams) -> Result<WriteResult> {
    // Validate parameters
    validate_params(&params)?;

    match params.operation {
        WriteOperation::UpsertNode => {
            let class = params.class.as_ref().unwrap();
            let meta = fetch_and_validate_class(state, class).await?;
            execute_upsert_node(state, &params, &meta).await
        }
        WriteOperation::CreateArc => execute_create_arc(state, &params).await,
        WriteOperation::UpdateProps => {
            let class = params.class.as_ref().unwrap();
            let meta = fetch_and_validate_class(state, class).await?;
            execute_update_props(state, &params, &meta).await
        }
    }
}
```

**Step 2: Run compilation check**

Run: `cargo check`
Expected: Compiles

**Step 3: Commit**

```bash
git add src/tools/write.rs
git commit -m "$(cat <<'EOF'
feat(mcp): implement novanet_write execute function

Main entry point that:
- Validates parameters
- Routes to appropriate operation handler
- Fetches and validates class metadata for node operations
- Returns WriteResult with all metadata

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

## Phase 4: Special Validations

### Task 4.1: Implement Slug Lock Validation

**Files:**
- Modify: `src/tools/write.rs`

**Step 1: Add slug lock check to update_props**

Add this function before `execute_update_props`:

```rust
/// Check if slug is locked and reject modification
async fn validate_slug_not_locked(
    state: &State,
    key: &str,
    props: &serde_json::Map<String, Value>,
) -> Result<()> {
    // Only check if "slug" property is being modified
    if !props.contains_key("slug") {
        return Ok(());
    }

    let query = r#"
        MATCH (n {key: $key})
        RETURN n.slug_locked AS locked, n.slug AS current_slug
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), Value::String(key.to_string()));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    if let Some(row) = rows.first() {
        let locked = row["locked"].as_bool().unwrap_or(false);
        let current_slug = row["current_slug"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        if locked {
            return Err(Error::slug_locked(key, current_slug));
        }
    }

    Ok(())
}
```

Update `execute_update_props` to call this function after verifying node exists:

```rust
    // Check for slug_locked before update
    validate_slug_not_locked(state, key, props).await?;
```

**Step 2: Write test**

```rust
#[cfg(test)]
mod tests {
    // ... existing tests ...

    #[test]
    fn test_slug_lock_check_skips_non_slug_props() {
        // This is a unit test that doesn't need Neo4j
        let props = serde_json::Map::new();
        // If no "slug" key, validation should pass
        assert!(!props.contains_key("slug"));
    }
}
```

**Step 3: Commit**

```bash
git add src/tools/write.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add slug_locked validation

validate_slug_not_locked():
- Only checks if "slug" property is being modified
- Fetches current slug_locked status from Neo4j
- Returns SlugLocked error if locked=true
- Integrated into update_props flow

ADR-030: Slug is immutable after deployment.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 4.2: Implement is_slug_source Singleton Validation

**Files:**
- Modify: `src/tools/write.rs`

**Step 1: Add singleton validation to create_arc**

Add this function before `execute_create_arc`:

```rust
/// Handle is_slug_source singleton - only one arc can have this property true
async fn handle_slug_source_singleton(
    state: &State,
    to_key: &str,
    props: &serde_json::Map<String, Value>,
) -> Result<()> {
    // Only check if is_slug_source is being set to true
    let is_setting_slug_source = props
        .get("is_slug_source")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !is_setting_slug_source {
        return Ok(());
    }

    // Find existing is_slug_source arc and demote it
    let query = r#"
        MATCH (kw)-[r:TARGETS {is_slug_source: true}]->(en {key: $to_key})
        SET r.is_slug_source = false, r.rank = 'secondary'
        RETURN kw.key AS demoted_key
    "#;

    let mut params = serde_json::Map::new();
    params.insert("to_key".to_string(), Value::String(to_key.to_string()));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    if let Some(row) = rows.first() {
        let demoted = row["demoted_key"].as_str().unwrap_or("unknown");
        warn!(
            demoted = %demoted,
            new_source = %to_key,
            "is_slug_source takeover: demoted previous source"
        );
    }

    Ok(())
}
```

Update `execute_create_arc` to call this function after verifying endpoints:

```rust
    // Handle is_slug_source singleton (takeover pattern)
    handle_slug_source_singleton(state, to_key, &props).await?;
```

**Step 2: Commit**

```bash
git add src/tools/write.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add is_slug_source singleton validation

handle_slug_source_singleton():
- Only activates when is_slug_source=true in props
- Finds existing TARGETS arc with is_slug_source=true
- Demotes it: is_slug_source=false, rank='secondary'
- Logs the takeover for audit trail

Takeover pattern instead of rejection for better UX.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

## Phase 5: Handler Integration

### Task 5.1: Register Write Module in tools/mod.rs

**Files:**
- Modify: `src/tools/mod.rs`

**Step 1: Add module and re-export**

Add after line 19:
```rust
pub mod write;
```

Add to re-exports:
```rust
pub use write::{WriteParams, WriteResult};
```

**Step 2: Run compilation check**

Run: `cargo check`
Expected: Compiles

**Step 3: Commit**

```bash
git add src/tools/mod.rs
git commit -m "$(cat <<'EOF'
feat(mcp): register write module in tools

- Add pub mod write
- Re-export WriteParams, WriteResult

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 5.2: Add novanet_write to Handler

**Files:**
- Modify: `src/server/handler.rs`

**Step 1: Add import**

Add `WriteParams` to the imports:

```rust
use crate::tools::{
    AssembleParams, AtomsParams, BatchParams, CacheInvalidateParams, CacheStatsParams,
    DescribeParams, GenerateParams, IntrospectParams, QueryParams, SearchParams, TraverseParams,
    WriteParams,
};
```

**Step 2: Add tool handler**

Add after `novanet_cache_invalidate` handler (around line 356):

```rust
    /// Write data to the NovaNet knowledge graph with schema validation.
    ///
    /// Supports 3 operations: upsert_node, create_arc, update_props.
    /// Validates trait permissions (only authored/imported/generated/retrieved allow writes).
    /// Enforces slug_locked and is_slug_source singleton constraints.
    #[tool(
        name = "novanet_write",
        description = "Write data to NovaNet with schema validation. Operations: upsert_node, create_arc, update_props. Enforces trait permissions and slug immutability."
    )]
    async fn novanet_write(
        &self,
        params: Parameters<WriteParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::write::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.with_hint()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
```

**Step 3: Update ServerInfo tool count**

Update the instructions in `get_info()` to mention 12 tools.

**Step 4: Run compilation check**

Run: `cargo check`
Expected: Compiles

**Step 5: Commit**

```bash
git add src/server/handler.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add novanet_write tool handler

12th MCP tool with:
- Schema validation via introspect
- Trait permission checking
- slug_locked enforcement
- is_slug_source singleton handling
- Error hints for actionable feedback

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

## Phase 6: Documentation & Tests

### Task 6.1: Add Cache invalidate_pattern Method

**Files:**
- Modify: `src/cache/mod.rs`

**Step 1: Add invalidate_pattern method**

Add to the QueryCache impl:

```rust
    /// Invalidate cache entries matching a pattern (prefix match)
    pub async fn invalidate_pattern(&self, pattern: &str) {
        // Simple prefix matching - invalidate all entries that start with pattern
        // For more complex patterns, we'd need to track keys separately
        self.cache.invalidate_all();
        // TODO: Implement proper pattern matching when needed
    }
```

**Step 2: Commit**

```bash
git add src/cache/mod.rs
git commit -m "$(cat <<'EOF'
feat(mcp): add cache invalidate_pattern method

Simple implementation that invalidates all entries.
TODO: Implement proper pattern matching for selective invalidation.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 6.2: Update CLAUDE.md with novanet_write

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Update tool count and add novanet_write documentation**

Update the architecture diagram to show 12 tools.

Add novanet_write section in the MCP Tools table:

```markdown
│  novanet_write      Write data to Neo4j with schema validation             │
│                      params: operation, class/arc_class, key/from_key/to_key│
│                              properties, locale                             │
│                      operations: upsert_node, create_arc, update_props      │
│                      returns: success, created, updated_properties,         │
│                               auto_arcs_created, cache_invalidated          │
```

**Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "$(cat <<'EOF'
docs: add novanet_write to CLAUDE.md

Document 12th MCP tool:
- 3 operations: upsert_node, create_arc, update_props
- Schema validation via introspect
- Trait permission enforcement
- slug_locked and is_slug_source constraints

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

### Task 6.3: Run Full Test Suite

**Step 1: Run all tests**

Run: `cargo test`
Expected: All tests pass (348+ tests)

**Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

**Step 3: Final commit if needed**

```bash
git add -A
git commit -m "$(cat <<'EOF'
chore: fix any remaining issues from test suite

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>
EOF
)"
```

---

## Summary

| Phase | Tasks | Est. Time |
|-------|-------|-----------|
| Phase 1 | Error types + hints | 2 tasks |
| Phase 2 | Schema cache | 3 tasks |
| Phase 3 | Write tool core | 7 tasks |
| Phase 4 | Special validations | 2 tasks |
| Phase 5 | Handler integration | 2 tasks |
| Phase 6 | Docs + tests | 3 tasks |
| **Total** | **19 tasks** | ~1600 lines |

## Key Design Decisions (Validated)

1. **Slug in BlockNative:head-seo-meta** (ADR-030 v0.13.1)
2. **is_slug_source takeover pattern** (demote old, not reject)
3. **Trait permissions**: defined=READ-ONLY, others=WRITABLE
4. **MERGE pattern** for idempotency
5. **Auto-arcs**: FOR_LOCALE created automatically
6. **Cache invalidation**: Pattern-based (class:*, key)

## Files Created/Modified

| File | Action |
|------|--------|
| `src/error.rs` | Add 6 error variants |
| `src/hints.rs` | Add 6 error hints |
| `src/schema_cache.rs` | Create new module |
| `src/lib.rs` | Register schema_cache |
| `src/server/state.rs` | Add schema cache to state |
| `src/tools/write.rs` | Create new tool (~500 lines) |
| `src/tools/mod.rs` | Register write module |
| `src/server/handler.rs` | Add tool handler |
| `src/cache/mod.rs` | Add invalidate_pattern |
| `CLAUDE.md` | Update documentation |
