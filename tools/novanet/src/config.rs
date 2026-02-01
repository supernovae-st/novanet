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

pub fn nodes_dir(root: &Path) -> PathBuf {
    root.join("packages/core/models/nodes")
}

pub fn relations_path(root: &Path) -> PathBuf {
    root.join("packages/core/models/relations.yaml")
}

pub fn organizing_principles_path(root: &Path) -> PathBuf {
    root.join("packages/core/models/organizing-principles.yaml")
}

pub fn seed_dir(root: &Path) -> PathBuf {
    root.join("packages/db/seed")
}

pub fn core_src_dir(root: &Path) -> PathBuf {
    root.join("packages/core/src")
}
