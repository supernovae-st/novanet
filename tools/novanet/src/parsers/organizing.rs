//! Organizing principles types (realms, layers, arc families).
//!
//! Data comes from individual YAML files (realms/, layers/, arc-families/)
//! via `load_taxonomy_from_files()`. Legacy `taxonomy.yaml` is still used for arc_scopes and terminal config.
//!
//! Used by:
//! - `generators/organizing.rs` → Cypher seed
//! - `generators/hierarchy.rs` → TypeScript hierarchy tree

use serde::Deserialize;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// OrganizingDoc — taxonomy data for generators
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level document (converted from TaxonomyDoc).
/// v0.17.3 (ADR-036): traits removed, provenance is per-instance.
#[derive(Debug, Deserialize)]
pub struct OrganizingDoc {
    pub version: String,
    pub realms: Vec<RealmDef>,
    // v0.17.3 (ADR-036): traits field removed, provenance is per-instance
    pub arc_families: Vec<ArcFamilyDef>,
}

/// Realm definition with nested layers.
#[derive(Debug, Deserialize)]
pub struct RealmDef {
    pub key: String,
    pub display_name: String,
    pub content: String,
    pub emoji: String,
    pub color: String,
    pub triggers: Vec<String>,
    pub layers: Vec<LayerDef>,
}

/// Layer definition (nested under its realm).
#[derive(Debug, Deserialize)]
pub struct LayerDef {
    pub key: String,
    pub display_name: String,
    pub content: String,
    pub emoji: String,
    pub color: String,
    pub triggers: Vec<String>,
}

// v0.17.3 (ADR-036): TraitDef removed, provenance is per-instance

/// Arc family definition.
#[derive(Debug, Deserialize)]
pub struct ArcFamilyDef {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub arrow_style: String,
    pub triggers: Vec<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader (via individual YAML files conversion)
// ─────────────────────────────────────────────────────────────────────────────

/// Load organizing principles from individual YAML files.
///
/// Loads from individual YAML files (realms/, layers/, arc-families/)
/// via `load_taxonomy_from_files()` and converts to `OrganizingDoc` format.
pub fn load_organizing(root: &Path) -> crate::Result<OrganizingDoc> {
    let taxonomy = crate::parsers::taxonomy::load_taxonomy_from_files(root)?;
    let doc = taxonomy.to_organizing_doc();

    // Fail-fast validation (same as before)
    if doc.realms.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "No realms defined in taxonomy".to_string(),
        ));
    }
    for realm in &doc.realms {
        if realm.layers.is_empty() {
            return Err(crate::NovaNetError::Validation(format!(
                "realm '{}' has no layers",
                realm.key
            )));
        }
    }
    // v0.17.3 (ADR-036): traits validation removed, provenance is per-instance
    if doc.arc_families.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "No arc_families defined in taxonomy".to_string(),
        ));
    }

    Ok(doc)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_organizing_yaml() {
        // v0.17.3 (ADR-036): traits section removed from YAML parsing
        let yaml = r##"
version: "9.0.0"
realms:
  - key: shared
    display_name: Shared
    content: "Universal knowledge (READ-ONLY)."
    emoji: "🌍"
    color: "#2aa198"
    triggers: ["shared", "global", "universal"]
    layers:
      - key: config
        display_name: Configuration
        content: "System configuration and locale definitions."
        emoji: "⚙️"
        color: "#64748b"
        triggers: ["config", "settings"]
arc_families:
  - key: ownership
    display_name: Ownership
    color: "#3b82f6"
    arrow_style: "-->"
    triggers: ["parent", "child", "owns"]
"##;
        let doc: OrganizingDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.version, "9.0.0");
        assert_eq!(doc.realms.len(), 1);
        assert_eq!(doc.realms[0].key, "shared");
        assert_eq!(doc.realms[0].layers.len(), 1);
        assert_eq!(doc.realms[0].layers[0].key, "config");
        // v0.17.3 (ADR-036): traits assertions removed
        assert_eq!(doc.arc_families.len(), 1);
        assert_eq!(doc.arc_families[0].arrow_style, "-->");
    }

    #[test]
    fn load_organizing_integration() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let doc = load_organizing(root).expect("should load from taxonomy.yaml");

        assert_eq!(doc.version, "0.13.0");
        assert_eq!(doc.realms.len(), 2); // 2 realms (shared, org)
        assert_eq!(doc.arc_families.len(), 6); // 6 families including schema

        let total_layers: usize = doc.realms.iter().map(|r| r.layers.len()).sum();
        assert_eq!(total_layers, 10); // v11.4: 4 shared + 6 org layers
    }
}
