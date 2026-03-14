//! System health checks for NovaNet.
//!
//! Verifies:
//! - YAML validity (taxonomy, node-classes, arc-classes)
//! - Neo4j connection (optional)
//! - Schema sync status
//! - Models directory structure

use std::path::Path;

use crate::db::Db;

/// Health check status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Ok,
    Warning,
    Error,
}

/// Individual health check result.
#[derive(Debug)]
pub struct HealthCheck {
    pub name: &'static str,
    pub status: HealthStatus,
    pub message: String,
}

impl HealthCheck {
    fn ok(name: &'static str, message: impl Into<String>) -> Self {
        Self {
            name,
            status: HealthStatus::Ok,
            message: message.into(),
        }
    }

    fn warning(name: &'static str, message: impl Into<String>) -> Self {
        Self {
            name,
            status: HealthStatus::Warning,
            message: message.into(),
        }
    }

    fn error(name: &'static str, message: impl Into<String>) -> Self {
        Self {
            name,
            status: HealthStatus::Error,
            message: message.into(),
        }
    }

    fn icon(&self) -> &'static str {
        match self.status {
            HealthStatus::Ok => "✓",
            HealthStatus::Warning => "⚠",
            HealthStatus::Error => "✗",
        }
    }
}

/// Run all health checks and print results.
pub async fn run_doctor(
    root: &Path,
    db: Option<&Db>,
    verbose: bool,
    fix: bool,
) -> crate::Result<Vec<HealthCheck>> {
    let mut checks = Vec::new();

    eprintln!("Running NovaNet health checks...\n");

    // 1. Check models directory exists
    checks.push(check_models_directory(root));

    // 2. Check YAML validity
    checks.push(check_yaml_validity(root, verbose));

    // 3. Check Neo4j connection
    if let Some(db) = db {
        checks.push(check_neo4j_connection(db).await);
    } else {
        checks.push(HealthCheck::warning(
            "Neo4j Connection",
            "Skipped (use without --skip-db to test connection)",
        ));
    }

    // 4. Check schema sync (with optional auto-fix)
    let schema_check = check_schema_sync(root, verbose);
    let needs_fix = fix && schema_check.status != HealthStatus::Ok;
    checks.push(schema_check);

    // 5. Auto-fix schema sync issues if --fix was passed
    if needs_fix {
        eprintln!();
        eprintln!("Attempting to fix schema sync issues...");
        match run_schema_generate(root) {
            Ok(()) => {
                eprintln!("  \x1b[32m✓\x1b[0m Schema regenerated successfully");
                // Re-check schema sync after fix
                let recheck = check_schema_sync(root, verbose);
                if recheck.status == HealthStatus::Ok {
                    eprintln!("  \x1b[32m✓\x1b[0m Schema now in sync");
                } else {
                    eprintln!("  \x1b[33m⚠\x1b[0m Schema still has issues after regeneration");
                }
            },
            Err(e) => {
                eprintln!("  \x1b[31m✗\x1b[0m Failed to regenerate schema: {}", e);
            },
        }
    }

    // Print results
    for check in &checks {
        let color = match check.status {
            HealthStatus::Ok => "\x1b[32m",      // Green
            HealthStatus::Warning => "\x1b[33m", // Yellow
            HealthStatus::Error => "\x1b[31m",   // Red
        };
        eprintln!(
            "  {}{} {}\x1b[0m — {}",
            color,
            check.icon(),
            check.name,
            check.message
        );
    }

    // Summary
    let ok_count = checks
        .iter()
        .filter(|c| c.status == HealthStatus::Ok)
        .count();
    let warn_count = checks
        .iter()
        .filter(|c| c.status == HealthStatus::Warning)
        .count();
    let err_count = checks
        .iter()
        .filter(|c| c.status == HealthStatus::Error)
        .count();

    eprintln!();
    if err_count > 0 {
        eprintln!(
            "\x1b[31m✗ {} error(s)\x1b[0m, {} warning(s), {} ok",
            err_count, warn_count, ok_count
        );
    } else if warn_count > 0 {
        eprintln!(
            "\x1b[33m⚠ {} warning(s)\x1b[0m, {} ok",
            warn_count, ok_count
        );
    } else {
        eprintln!("\x1b[32m✓ All {} checks passed\x1b[0m", ok_count);
    }

    Ok(checks)
}

fn check_models_directory(root: &Path) -> HealthCheck {
    let models = crate::config::models_dir(root);
    let node_classes = models.join("node-classes");
    let arc_classes = models.join("arc-classes");
    let taxonomy = models.join("taxonomy.yaml");

    let mut missing = Vec::new();
    if !node_classes.exists() {
        missing.push("node-classes/");
    }
    if !arc_classes.exists() {
        missing.push("arc-classes/");
    }
    if !taxonomy.exists() {
        missing.push("taxonomy.yaml");
    }

    if missing.is_empty() {
        HealthCheck::ok("Models Directory", format!("Found at {}", models.display()))
    } else {
        HealthCheck::error(
            "Models Directory",
            format!("Missing: {}", missing.join(", ")),
        )
    }
}

fn check_yaml_validity(root: &Path, verbose: bool) -> HealthCheck {
    // Load from individual YAML files
    match crate::parsers::taxonomy::load_taxonomy_from_files(root) {
        Ok(taxonomy) => {
            let total_layers: usize = taxonomy.node_realms.iter().map(|r| r.layers.len()).sum();
            // v0.17.3 (ADR-036): traits removed, provenance is per-instance
            let msg = if verbose {
                format!(
                    "Taxonomy valid ({} realms, {} layers)",
                    taxonomy.node_realms.len(),
                    total_layers
                )
            } else {
                "All YAML files valid".into()
            };
            HealthCheck::ok("YAML Validity", msg)
        },
        Err(e) => HealthCheck::error("YAML Validity", format!("Parse error: {}", e)),
    }
}

async fn check_neo4j_connection(db: &Db) -> HealthCheck {
    match db.execute("RETURN 1 AS ok").await {
        Ok(_) => HealthCheck::ok("Neo4j Connection", "Connected successfully"),
        Err(e) => HealthCheck::error("Neo4j Connection", format!("Failed: {}", e)),
    }
}

fn check_schema_sync(root: &Path, verbose: bool) -> HealthCheck {
    match crate::commands::schema::schema_validate(root) {
        Ok(issues) => {
            let errors: Vec<_> = issues
                .iter()
                .filter(|i| i.severity == crate::commands::schema::Severity::Error)
                .collect();
            let warnings: Vec<_> = issues
                .iter()
                .filter(|i| i.severity == crate::commands::schema::Severity::Warning)
                .collect();

            if errors.is_empty() && warnings.is_empty() {
                HealthCheck::ok("Schema Sync", "All artifacts in sync with YAML")
            } else if errors.is_empty() {
                let msg = if verbose {
                    format!("{} warning(s) found", warnings.len())
                } else {
                    "Warnings found (run with --verbose)".into()
                };
                HealthCheck::warning("Schema Sync", msg)
            } else {
                let msg = if verbose {
                    format!("{} error(s), {} warning(s)", errors.len(), warnings.len())
                } else {
                    format!("{} error(s) found", errors.len())
                };
                HealthCheck::error("Schema Sync", msg)
            }
        },
        Err(e) => HealthCheck::error("Schema Sync", format!("Validation failed: {}", e)),
    }
}

/// Run schema generate to fix sync issues.
fn run_schema_generate(root: &Path) -> crate::Result<()> {
    crate::commands::schema::schema_generate(root, false)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_icons() {
        assert_eq!(HealthCheck::ok("test", "msg").icon(), "✓");
        assert_eq!(HealthCheck::warning("test", "msg").icon(), "⚠");
        assert_eq!(HealthCheck::error("test", "msg").icon(), "✗");
    }
}
