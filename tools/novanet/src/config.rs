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

/// Derived paths from the monorepo root
pub fn models_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models")
}

pub fn node_kinds_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/node-kinds")
}

pub fn relations_path(root: &Path) -> PathBuf {
    root.join("packages/core/models/relations.yaml")
}

/// Path to taxonomy.yaml (v9.5 - realms, layers, traits, arc families)
pub fn taxonomy_path(root: &Path) -> PathBuf {
    root.join("packages/core/models/taxonomy.yaml")
}

/// Directory containing arc-kinds YAML files
pub fn arc_kinds_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/arc-kinds")
}

pub fn seed_dir(root: &Path) -> PathBuf {
    root.join("packages/db/seed")
}

pub fn core_src_dir(root: &Path) -> PathBuf {
    root.join("packages/core/src")
}

pub fn views_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/views")
}

pub fn docs_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/docs")
}

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
    fn node_kinds_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            node_kinds_dir(root),
            PathBuf::from("/fake/root/packages/core/models/node-kinds")
        );
    }

    #[test]
    fn relations_path_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            relations_path(root),
            PathBuf::from("/fake/root/packages/core/models/relations.yaml")
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
    fn arc_kinds_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            arc_kinds_dir(root),
            PathBuf::from("/fake/root/packages/core/models/arc-kinds")
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

    #[test]
    fn views_dir_joins_correctly() {
        let root = Path::new("/fake/root");
        assert_eq!(
            views_dir(root),
            PathBuf::from("/fake/root/packages/core/models/views")
        );
    }

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
