//! Configuration: monorepo root discovery and derived paths.

use std::path::{Path, PathBuf};

/// Resolve the monorepo root directory.
/// Priority: 1) --root flag  2) NOVANET_ROOT env  3) walk up to pnpm-workspace.yaml
pub fn resolve_root(explicit: Option<&Path>) -> crate::Result<PathBuf> {
    if let Some(root) = explicit {
        return Ok(root.to_path_buf());
    }

    if let Ok(env_root) = std::env::var("NOVANET_ROOT") {
        let path = PathBuf::from(env_root);
        if path.join("pnpm-workspace.yaml").exists() {
            return Ok(path);
        }
    }

    // Walk up from current directory to find pnpm-workspace.yaml
    let mut dir = std::env::current_dir().map_err(crate::NovaNetError::Io)?;
    loop {
        if dir.join("pnpm-workspace.yaml").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            return Err(crate::NovaNetError::Validation(
                "Could not find monorepo root (no pnpm-workspace.yaml in parent directories). \
                 Use --root or set NOVANET_ROOT."
                    .to_string(),
            ));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Derived paths from the monorepo root
// ─────────────────────────────────────────────────────────────────────────────

#[must_use]
pub fn models_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models")
}

#[must_use]
pub fn node_classes_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/node-classes")
}

/// Path to taxonomy.yaml (v9.5 - realms, layers, traits, arc families)
#[must_use]
pub fn taxonomy_path(root: &Path) -> PathBuf {
    root.join("packages/core/models/taxonomy.yaml")
}

/// Directory containing arc-classes YAML files
#[must_use]
pub fn arc_classes_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/arc-classes")
}

/// Directory containing realm YAML files (v0.12.5)
#[must_use]
pub fn realms_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/realms")
}

/// Directory containing layer YAML files (v0.12.5)
#[must_use]
pub fn layers_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/layers")
}

/// Directory containing trait YAML files (v0.12.5)
#[must_use]
pub fn traits_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/traits")
}

/// Directory containing arc-family YAML files (v0.12.5)
#[must_use]
pub fn arc_families_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/arc-families")
}

#[must_use]
pub fn seed_dir(root: &Path) -> PathBuf {
    root.join("packages/db/seed")
}

#[must_use]
pub fn core_src_dir(root: &Path) -> PathBuf {
    root.join("packages/core/src")
}

// NOTE: views_dir was removed in v0.12.5.
// The old views/ directory was replaced by a single views.yaml file.
// Use models_dir(root).join("views.yaml") for the new views file.

#[must_use]
pub fn docs_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/docs")
}

#[must_use]
pub fn migrations_dir(root: &Path) -> PathBuf {
    root.join("packages/db/migrations")
}

/// Resolve the ATH data path for knowledge generators.
/// Priority: 1) explicit path  2) NOVANET_ATH_PATH env var  3) error
pub fn resolve_ath_path(explicit: Option<&str>) -> crate::Result<PathBuf> {
    if let Some(path) = explicit {
        return Ok(PathBuf::from(path));
    }

    if let Ok(env_path) = std::env::var("NOVANET_ATH_PATH") {
        return Ok(PathBuf::from(env_path));
    }

    Err(crate::NovaNetError::Validation(
        "ATH data path not set. Use --ath-path or set NOVANET_ATH_PATH environment variable."
            .to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn models_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            models_dir(root),
            PathBuf::from("/fake/root/packages/core/models")
        );
    }

    #[test]
    fn node_classes_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            node_classes_dir(root),
            PathBuf::from("/fake/root/packages/core/models/node-classes")
        );
    }

    #[test]
    fn taxonomy_path_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            taxonomy_path(root),
            PathBuf::from("/fake/root/packages/core/models/taxonomy.yaml")
        );
    }

    #[test]
    fn arc_classes_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            arc_classes_dir(root),
            PathBuf::from("/fake/root/packages/core/models/arc-classes")
        );
    }

    #[test]
    fn seed_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(seed_dir(root), PathBuf::from("/fake/root/packages/db/seed"));
    }

    #[test]
    fn core_src_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            core_src_dir(root),
            PathBuf::from("/fake/root/packages/core/src")
        );
    }

    // NOTE: views_dir test removed in v0.12.5 (views/ replaced by views.yaml)

    #[test]
    fn docs_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            docs_dir(root),
            PathBuf::from("/fake/root/packages/core/models/docs")
        );
    }

    #[test]
    fn migrations_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            migrations_dir(root),
            PathBuf::from("/fake/root/packages/db/migrations")
        );
    }

    #[test]
    fn resolve_root_explicit_path() {
        let path = Path::new("/explicit/path");
        let result = resolve_root(Some(path)).unwrap();
        assert_eq!(result, PathBuf::from("/explicit/path"));
    }

    #[test]
    fn resolve_root_from_cwd() {
        // Running from within the monorepo should find pnpm-workspace.yaml
        let result = resolve_root(None);
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!(root.join("pnpm-workspace.yaml").exists());
    }
}
