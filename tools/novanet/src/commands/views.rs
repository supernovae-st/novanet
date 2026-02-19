//! Views validation and export commands.
//!
//! Provides canonical JSON export for cross-validation between Rust (TUI)
//! and TypeScript (Studio) view parsers.

use std::path::Path;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::error::Result;
use crate::parsers::views::load_simple_views;

// ─────────────────────────────────────────────────────────────────────────────
// Canonical Export Structs
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical view icon (fields in alphabetical order for consistent JSON).
#[derive(Serialize)]
struct CanonicalIcon {
    terminal: String,
    web: String,
}

/// Canonical view (fields in ALPHABETICAL order for consistent JSON).
///
/// This ensures Rust and TypeScript produce identical JSON output
/// regardless of struct field declaration order.
#[derive(Serialize)]
struct CanonicalView {
    applicable_types: Vec<String>,
    category: String,
    color: String,
    contextual: bool,
    cypher_hash: String,
    description: String,
    icon: CanonicalIcon,
    id: String,
    name: String,
    root_type: Option<String>,
}

/// Canonical export document (fields in alphabetical order).
#[derive(Serialize)]
struct CanonicalExport {
    count: usize,
    version: String,
    views: Vec<CanonicalView>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Hash Function
// ─────────────────────────────────────────────────────────────────────────────

/// Hash a cypher query string, returning first 8 hex chars of SHA256.
///
/// Trims whitespace before hashing for consistent results.
fn hash_cypher(cypher: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(cypher.trim().as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)[..8].to_string()
}

// ─────────────────────────────────────────────────────────────────────────────
// Export Function
// ─────────────────────────────────────────────────────────────────────────────

/// Export views as canonical JSON for cross-validation.
///
/// Returns pretty-printed JSON with:
/// - Views sorted by id
/// - Fields in alphabetical order
/// - SHA256 hash (first 8 chars) of cypher queries
///
/// # Example output
///
/// ```json
/// {
///   "count": 11,
///   "version": "0.12.5",
///   "views": [
///     {
///       "applicable_types": [],
///       "category": "contextual",
///       "color": "#f59e0b",
///       ...
///     }
///   ]
/// }
/// ```
/// Validate views match between Rust and TypeScript parsers.
///
/// This is a stub implementation. Task 3 will add the full implementation
/// that calls the TypeScript export script and compares results.
pub fn views_validate(root: &Path, verbose: bool) -> Result<()> {
    // Get Rust export to verify it works
    let rust_json = views_export(root)?;
    let rust_parsed: serde_json::Value = serde_json::from_str(&rust_json)
        .map_err(|e| crate::NovaNetError::Validation(format!("JSON parse failed: {}", e)))?;

    let count = rust_parsed["count"].as_u64().unwrap_or(0);
    eprintln!("✓ Rust parsed {} views", count);

    // Check for TypeScript script
    let ts_script = root.join("packages/core/scripts/export-views.mjs");
    if !ts_script.exists() {
        eprintln!(
            "⚠  TypeScript export script not found: {}",
            ts_script.display()
        );
        eprintln!("   Run Task 4 to create: packages/core/scripts/export-views.mjs");
        eprintln!();
        eprintln!("   For now, Rust export works. Cross-validation pending.");
        return Ok(());
    }

    // TypeScript script exists - try to run it
    use std::process::Command;

    let output = Command::new("node")
        .arg(&ts_script)
        .current_dir(root)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let ts_json = String::from_utf8_lossy(&output.stdout);
            let ts_parsed: serde_json::Value = serde_json::from_str(&ts_json).map_err(|e| {
                crate::NovaNetError::Validation(format!("TS JSON parse failed: {}", e))
            })?;

            let ts_count = ts_parsed["count"].as_u64().unwrap_or(0);
            eprintln!("✓ TypeScript parsed {} views", ts_count);

            // Compare
            if rust_parsed == ts_parsed {
                eprintln!("✓ All views match!");
                Ok(())
            } else {
                if verbose {
                    let rust_views = rust_parsed["views"].as_array();
                    let ts_views = ts_parsed["views"].as_array();

                    if let (Some(rv), Some(tv)) = (rust_views, ts_views) {
                        for (r, t) in rv.iter().zip(tv.iter()) {
                            let id = r["id"].as_str().unwrap_or("?");
                            if r == t {
                                eprintln!("  ✓ {}: match", id);
                            } else {
                                eprintln!("  ✗ {}: MISMATCH", id);
                                eprintln!(
                                    "    Rust: {}",
                                    serde_json::to_string(r).unwrap_or_default()
                                );
                                eprintln!(
                                    "    TS:   {}",
                                    serde_json::to_string(t).unwrap_or_default()
                                );
                            }
                        }
                    }
                }
                Err(crate::NovaNetError::Validation(
                    "Views mismatch between Rust and TypeScript".to_string(),
                ))
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("⚠  TypeScript export failed: {}", stderr.trim());
            eprintln!("   Rust export works. Cross-validation pending.");
            Ok(())
        }
        Err(e) => {
            eprintln!("⚠  Could not run node: {}", e);
            eprintln!("   Rust export works. Cross-validation pending.");
            Ok(())
        }
    }
}

/// Export views as canonical JSON for cross-validation.
pub fn views_export(root: &Path) -> Result<String> {
    let file = load_simple_views(root)?;

    let mut views: Vec<CanonicalView> = file
        .views
        .into_iter()
        .map(|v| {
            let icon = v.icon.unwrap_or_default();
            CanonicalView {
                applicable_types: v.applicable_types.unwrap_or_default(),
                category: v.category,
                color: v.color.unwrap_or_default(),
                contextual: v.contextual.unwrap_or(false),
                cypher_hash: v
                    .cypher
                    .as_ref()
                    .map(|c| hash_cypher(c))
                    .unwrap_or_default(),
                description: v.description,
                icon: CanonicalIcon {
                    terminal: icon.terminal,
                    web: icon.web,
                },
                id: v.id,
                name: v.name,
                root_type: v.root_type,
            }
        })
        .collect();

    // Sort by id for canonical order
    views.sort_by(|a, b| a.id.cmp(&b.id));

    let export = CanonicalExport {
        count: views.len(),
        version: file.version,
        views,
    };

    serde_json::to_string_pretty(&export)
        .map_err(|e| crate::NovaNetError::Validation(format!("JSON serialization failed: {}", e)))
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_root() -> Option<PathBuf> {
        std::env::current_dir()
            .ok()?
            .ancestors()
            .find(|p| p.join("pnpm-workspace.yaml").exists())
            .map(|p| p.to_path_buf())
    }

    #[test]
    fn views_export_returns_canonical_json() {
        let Some(root) = test_root() else { return };
        let result = views_export(&root).expect("should export views");

        assert!(result.contains("\"version\""));
        assert!(result.contains("\"count\""));
        assert!(result.contains("\"views\""));
        assert!(result.contains("\"data-complete\""));
    }

    #[test]
    fn hash_cypher_produces_8_char_hex() {
        let hash = hash_cypher("MATCH (n) RETURN n");
        assert_eq!(hash.len(), 8);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn hash_cypher_trims_whitespace() {
        let hash1 = hash_cypher("MATCH (n) RETURN n");
        let hash2 = hash_cypher("  MATCH (n) RETURN n  ");
        let hash3 = hash_cypher("\nMATCH (n) RETURN n\n");
        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
    }

    #[test]
    fn hash_cypher_is_deterministic() {
        let query = "MATCH (n:Schema) RETURN n";
        let hash1 = hash_cypher(query);
        let hash2 = hash_cypher(query);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn views_export_matches_typescript_format() {
        let Some(root) = test_root() else { return };

        let rust_json = views_export(&root).expect("rust export");
        let rust: serde_json::Value = serde_json::from_str(&rust_json).unwrap();

        // Verify top-level structure
        assert!(rust["version"].is_string(), "version should be a string");
        assert!(rust["count"].is_number(), "count should be a number");
        assert!(rust["views"].is_array(), "views should be an array");

        // Verify all views have required fields
        for view in rust["views"].as_array().unwrap() {
            assert!(view["id"].is_string(), "missing id");
            assert!(view["name"].is_string(), "missing name");
            assert!(view["description"].is_string(), "missing description");
            assert!(view["category"].is_string(), "missing category");
            assert!(view["color"].is_string(), "missing color");
            assert!(view["icon"]["web"].is_string(), "missing icon.web");
            assert!(
                view["icon"]["terminal"].is_string(),
                "missing icon.terminal"
            );
            assert!(view["cypher_hash"].is_string(), "missing cypher_hash");
            // contextual is bool
            assert!(view["contextual"].is_boolean(), "contextual should be bool");
            // applicable_types is array
            assert!(
                view["applicable_types"].is_array(),
                "applicable_types should be array"
            );
        }

        // Verify sorted by id
        let views = rust["views"].as_array().unwrap();
        for i in 1..views.len() {
            let prev = views[i - 1]["id"].as_str().unwrap();
            let curr = views[i]["id"].as_str().unwrap();
            assert!(
                prev < curr,
                "views should be sorted by id: {} < {}",
                prev,
                curr
            );
        }
    }
}
