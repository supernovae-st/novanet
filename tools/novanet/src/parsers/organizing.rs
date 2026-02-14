//! Organizing principles types (realms, layers, traits, arc families).
//!
//! v0.12.5: Data now comes from individual YAML files (realms/, layers/, traits/, arc-families/)
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
#[derive(Debug, Deserialize)]
pub struct OrganizingDoc {
    pub version: String,
    pub realms: Vec<RealmDef>,
    pub traits: Vec<TraitDef>,
    pub arc_families: Vec<ArcFamilyDef>,
}

/// Realm definition with nested layers.
#[derive(Debug, Deserialize)]
pub struct RealmDef {
    pub key: String,
    pub display_name: String,
    pub emoji: String,
    pub color: String,
    pub llm_context: String,
    pub layers: Vec<LayerDef>,
}

/// Layer definition (nested under its realm).
#[derive(Debug, Deserialize)]
pub struct LayerDef {
    pub key: String,
    pub display_name: String,
    pub emoji: String,
    pub color: String,
    pub llm_context: String,
}

/// Trait (locale behavior) definition.
#[derive(Debug, Deserialize)]
pub struct TraitDef {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub llm_context: String,
}

/// Arc family definition.
#[derive(Debug, Deserialize)]
pub struct ArcFamilyDef {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub arrow_style: String,
    pub llm_context: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader (via individual YAML files conversion)
// ─────────────────────────────────────────────────────────────────────────────

/// Load organizing principles from individual YAML files (with backwards-compatible format).
///
/// v0.12.5: This function now loads from individual YAML files (realms/, layers/, traits/,
/// arc-families/) via `load_taxonomy_from_files()` and converts to `OrganizingDoc` format.
/// The return type remains `OrganizingDoc` for backwards compatibility with existing generators.
pub fn load_organizing(root: &Path) -> crate::Result<OrganizingDoc> {
    // v0.12.5: Load from individual YAML files and convert to OrganizingDoc format
    let taxonomy = crate::parsers::taxonomy::load_taxonomy_from_files(root)?;
    let doc = taxonomy.to_organizing_doc();

    // Fail-fast validation (same as before)
    if doc.realms.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "taxonomy.yaml has no realms".to_string(),
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
    if doc.traits.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "taxonomy.yaml has no traits".to_string(),
        ));
    }
    if doc.arc_families.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "taxonomy.yaml has no arc_families".to_string(),
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
        let yaml = r##"
version: "9.0.0"
realms:
  - key: shared
    display_name: Shared
    emoji: "🌍"
    color: "#2aa198"
    llm_context: "Shared context."
    layers:
      - key: config
        display_name: Configuration
        emoji: "⚙️"
        color: "#64748b"
        llm_context: "Config layer."
traits:
  - key: defined
    display_name: Defined
    color: "#3b82f6"
    llm_context: "Defined nodes (v11.8: was invariant)."
arc_families:
  - key: ownership
    display_name: Ownership
    color: "#3b82f6"
    arrow_style: "-->"
    llm_context: "Ownership arcs."
"##;
        let doc: OrganizingDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.version, "9.0.0");
        assert_eq!(doc.realms.len(), 1);
        assert_eq!(doc.realms[0].key, "shared");
        assert_eq!(doc.realms[0].layers.len(), 1);
        assert_eq!(doc.realms[0].layers[0].key, "config");
        assert_eq!(doc.traits.len(), 1);
        assert_eq!(doc.traits[0].key, "defined"); // v11.8: was invariant
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

        // v11.5: Locale moved to shared/config
        assert_eq!(doc.version, "0.12.0");
        assert_eq!(doc.realms.len(), 2); // v11.2: 2 realms (shared, org)
        assert_eq!(doc.traits.len(), 5); // v11.2: split derived → generated + aggregated
        assert_eq!(doc.arc_families.len(), 5);

        let total_layers: usize = doc.realms.iter().map(|r| r.layers.len()).sum();
        assert_eq!(total_layers, 10); // v11.4: 4 shared + 6 org layers
    }
}
