//! `novanet doc generate` and `novanet doc list` commands.
//!
//! v0.12.5: The old ViewDef-based generation was removed.
//! - doc list: Now uses views.yaml (simplified format)
//! - doc generate: Deprecated (the old views/ directory was removed)

use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Doc Generate (DEPRECATED)
// ─────────────────────────────────────────────────────────────────────────────

/// Result of generating a single view document.
#[derive(Debug)]
pub struct DocGenerateResult {
    pub view_id: String,
    pub view_name: String,
    pub output_path: String,
    pub bytes: usize,
    pub duration_ms: u128,
}

/// Generate Mermaid documentation for views.
///
/// **DEPRECATED in v0.12.5**: The old views/ directory was removed.
/// The new views.yaml uses Cypher queries instead of include rules,
/// so the Mermaid diagram generation is no longer supported.
///
/// Use `novanet schema generate` for the complete-graph.md diagram instead.
pub fn doc_generate(
    _root: &Path,
    _view_id: Option<&str>,
    _dry_run: bool,
) -> crate::Result<Vec<DocGenerateResult>> {
    Err(crate::NovaNetError::Validation(
        "doc generate is deprecated in v0.12.5. \
         The old views/ directory was removed. \
         Use 'novanet schema generate' for the complete-graph.md diagram."
            .to_string(),
    ))
}

// ─────────────────────────────────────────────────────────────────────────────
// Doc List
// ─────────────────────────────────────────────────────────────────────────────

/// Entry for display in `novanet doc generate --list`.
pub struct DocListEntry {
    pub id: String,
    pub category: String,
    pub description: String,
}

/// List available views from views.yaml.
///
/// v0.12.5: Now uses the simplified views.yaml format.
pub fn doc_list(root: &Path) -> crate::Result<Vec<DocListEntry>> {
    let views_file = crate::parsers::views::load_simple_views(root)?;
    let entries = views_file
        .views
        .into_iter()
        .map(|v| DocListEntry {
            id: v.id,
            category: v.category,
            description: v.description,
        })
        .collect();
    Ok(entries)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_root() -> Option<std::path::PathBuf> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());
        let root = root?;
        if !root.join("pnpm-workspace.yaml").exists() {
            return None;
        }
        Some(root.to_path_buf())
    }

    #[test]
    fn doc_generate_returns_deprecation_error() {
        let Some(root) = test_root() else { return };

        let result = doc_generate(&root, None, true);
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        assert!(err.contains("deprecated"));
    }

    #[test]
    fn doc_list_returns_views() {
        let Some(root) = test_root() else { return };

        let entries = doc_list(&root).expect("should list views");

        // v0.13.0: 12 views in views.yaml (added entity-truth)
        assert_eq!(
            entries.len(),
            12,
            "expected 12 views, got {}",
            entries.len()
        );

        // Every entry should have non-empty fields
        for e in &entries {
            assert!(!e.id.is_empty(), "empty id");
            assert!(!e.category.is_empty(), "empty category for {}", e.id);
            assert!(!e.description.is_empty(), "empty description for {}", e.id);
        }
    }
}
