//! Parse organizing-principles.yaml (realms, layers, traits, edge families).
//!
//! Shared between:
//! - `generators/organizing.rs` → Cypher seed
//! - `generators/hierarchy.rs` → TypeScript hierarchy tree

use serde::Deserialize;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs (organizing-principles.yaml)
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level document.
#[derive(Debug, Deserialize)]
pub struct OrganizingDoc {
    pub version: String,
    pub realms: Vec<RealmDef>,
    pub traits: Vec<TraitDef>,
    pub edge_families: Vec<EdgeFamilyDef>,
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

/// Edge family definition.
#[derive(Debug, Deserialize)]
pub struct EdgeFamilyDef {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub arrow_style: String,
    pub llm_context: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

/// Load and validate organizing-principles.yaml.
pub fn load_organizing(root: &Path) -> crate::Result<OrganizingDoc> {
    let path = crate::config::organizing_principles_path(root);

    if !path.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "organizing-principles.yaml not found: {}",
            path.display()
        )));
    }

    let content = std::fs::read_to_string(&path)?;
    let doc: OrganizingDoc =
        serde_yml::from_str(&content).map_err(|e| crate::NovaNetError::Schema {
            path: path.display().to_string(),
            source: e,
        })?;

    // Fail-fast validation
    if doc.realms.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "organizing-principles.yaml has no realms".to_string(),
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
            "organizing-principles.yaml has no traits".to_string(),
        ));
    }
    if doc.edge_families.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "organizing-principles.yaml has no edge_families".to_string(),
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
edge_families:
  - key: ownership
    display_name: Ownership
    color: "#3b82f6"
    arrow_style: "-->"
    llm_context: "Ownership edges."
"##;
        let doc: OrganizingDoc = serde_yml::from_str(yaml).unwrap();
        assert_eq!(doc.version, "9.0.0");
        assert_eq!(doc.realms.len(), 1);
        assert_eq!(doc.realms[0].key, "global");
        assert_eq!(doc.realms[0].layers.len(), 1);
        assert_eq!(doc.realms[0].layers[0].key, "config");
        assert_eq!(doc.traits.len(), 1);
        assert_eq!(doc.traits[0].key, "invariant");
        assert_eq!(doc.edge_families.len(), 1);
        assert_eq!(doc.edge_families[0].arrow_style, "-->");
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

        let doc = load_organizing(root).expect("should load organizing-principles.yaml");

        assert_eq!(doc.version, "9.0.0");
        assert_eq!(doc.realms.len(), 3);
        assert_eq!(doc.traits.len(), 5);
        assert_eq!(doc.edge_families.len(), 5);

        let total_layers: usize = doc.realms.iter().map(|r| r.layers.len()).sum();
        assert_eq!(total_layers, 9);
    }
}
