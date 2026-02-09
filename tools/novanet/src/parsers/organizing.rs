//! Organizing principles types (realms, layers, traits, arc families).
//!
//! Data comes from `taxonomy.yaml` and is exposed via `OrganizingDoc`.
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
// Loader (via taxonomy.yaml conversion)
// ─────────────────────────────────────────────────────────────────────────────

/// Load organizing principles from taxonomy.yaml (with backwards-compatible format).
///
/// This function loads `taxonomy.yaml` and converts it to `OrganizingDoc` format.
/// The underlying data source is now `taxonomy.yaml`, but the return type remains
/// `OrganizingDoc` for backwards compatibility with existing generators.
pub fn load_organizing(root: &Path) -> crate::Result<OrganizingDoc> {
    // Load from taxonomy.yaml and convert to OrganizingDoc format
    let taxonomy = crate::parsers::taxonomy::load_taxonomy(root)?;
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
  - key: global
    display_name: Global
    emoji: "🌍"
    color: "#2aa198"
    llm_context: "Global context."
    layers:
      - key: config
        display_name: Configuration
        emoji: "⚙️"
        color: "#64748b"
        llm_context: "Config layer."
traits:
  - key: invariant
    display_name: Invariant
    color: "#3b82f6"
    llm_context: "Invariant nodes."
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
        assert_eq!(doc.realms[0].key, "global");
        assert_eq!(doc.realms[0].layers.len(), 1);
        assert_eq!(doc.realms[0].layers[0].key, "config");
        assert_eq!(doc.traits.len(), 1);
        assert_eq!(doc.traits[0].key, "invariant");
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

        // Version now comes from taxonomy.yaml (10.6.0)
        assert_eq!(doc.version, "11.0.0");
        assert_eq!(doc.realms.len(), 2); // v10.6: 2 realms (global, tenant)
        assert_eq!(doc.traits.len(), 4); // v11.1: removed job trait
        assert_eq!(doc.arc_families.len(), 5);

        let total_layers: usize = doc.realms.iter().map(|r| r.layers.len()).sum();
        assert_eq!(total_layers, 9); // v10.6: 3 global + 6 tenant layers
    }
}
