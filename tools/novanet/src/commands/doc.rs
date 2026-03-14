//! `novanet doc list` command.
//!
//! Lists available views from views.yaml.
//! Note: `doc generate` was removed in v0.20.0 (deprecated since v0.12.5).
//! Use `novanet schema generate` for diagram generation.

use std::path::Path;

/// Entry for display in `novanet doc generate --list`.
pub struct DocListEntry {
    pub id: String,
    pub category: String,
    pub description: String,
}

/// List available views from views.yaml.
///
/// Now uses the simplified views.yaml format.
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
    fn doc_list_returns_views() {
        let Some(root) = test_root() else { return };

        let entries = doc_list(&root).expect("should list views");

        // 13 views in views.yaml (added denomination-forms-context, ADR-033)
        assert_eq!(
            entries.len(),
            13,
            "expected 13 views, got {}",
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
