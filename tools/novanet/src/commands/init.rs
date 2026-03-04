//! Initialize NovaNet CLI configuration.
//!
//! Interactive setup for first-time users:
//! - Creates ~/.novanet/ directory
//! - Configures monorepo root path
//! - Configures Neo4j connection
//! - Tests connection
//! - Saves configuration

use crate::user_config::{CliConfig, Neo4jConfig, UserConfig};
use std::io::{self, Write};

/// Result of the init command
#[derive(Debug)]
pub struct InitResult {
    pub config_path: std::path::PathBuf,
    pub root_configured: bool,
    pub neo4j_configured: bool,
    pub connection_tested: bool,
}

/// Run the init command
pub async fn run_init(
    non_interactive: bool,
    root_path: Option<&str>,
    neo4j_uri: Option<&str>,
    neo4j_user: Option<&str>,
    neo4j_password: Option<&str>,
    force: bool,
) -> crate::Result<InitResult> {
    let config_path = UserConfig::config_path();

    // Check if config already exists
    if UserConfig::exists() && !force {
        eprintln!("Configuration already exists at {}", config_path.display());
        eprintln!("Use --force to overwrite existing configuration.");
        return Err(crate::NovaNetError::Validation(
            "Configuration already exists. Use --force to overwrite.".to_string(),
        ));
    }

    eprintln!();
    eprintln!("Welcome to NovaNet CLI Setup");
    eprintln!("=============================");
    eprintln!();

    let config = if non_interactive {
        // Non-interactive mode: use provided values or defaults
        create_config_from_args(root_path, neo4j_uri, neo4j_user, neo4j_password)
    } else {
        // Interactive mode: prompt for values
        create_config_interactive(root_path, neo4j_uri, neo4j_user, neo4j_password)?
    };

    // Test connection if password is provided
    let connection_tested = if config.neo4j.password.is_some() {
        eprintln!();
        eprintln!("Testing Neo4j connection...");
        match test_neo4j_connection(&config).await {
            Ok(()) => {
                eprintln!("  Connected successfully");
                true
            }
            Err(e) => {
                eprintln!("  Warning: Connection failed: {}", e);
                eprintln!("  Configuration will be saved anyway.");
                false
            }
        }
    } else {
        eprintln!();
        eprintln!("Note: No password provided. Set NEO4J_PASSWORD env var or run init again.");
        false
    };

    // Save configuration
    config.save()?;
    eprintln!();
    eprintln!("Configuration saved to {}", config_path.display());

    // Print next steps
    eprintln!();
    eprintln!("Next steps:");
    eprintln!("  1. Run 'novanet doctor' to verify setup");
    eprintln!("  2. Run 'novanet schema validate' to check YAML schema");
    eprintln!("  3. Run 'novanet db seed' to populate Neo4j");
    eprintln!();

    Ok(InitResult {
        config_path,
        root_configured: config.cli.root.is_some(),
        neo4j_configured: config.neo4j.password.is_some(),
        connection_tested,
    })
}

fn create_config_from_args(
    root: Option<&str>,
    uri: Option<&str>,
    user: Option<&str>,
    password: Option<&str>,
) -> UserConfig {
    UserConfig {
        neo4j: Neo4jConfig {
            uri: uri.unwrap_or("bolt://localhost:7687").to_string(),
            user: user.unwrap_or("neo4j").to_string(),
            password: password.map(String::from),
        },
        cli: CliConfig {
            root: root.map(String::from),
            ..Default::default()
        },
    }
}

fn create_config_interactive(
    default_root: Option<&str>,
    default_uri: Option<&str>,
    default_user: Option<&str>,
    default_password: Option<&str>,
) -> crate::Result<UserConfig> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // NovaNet root path
    eprintln!("NovaNet Monorepo Root");
    eprintln!("---------------------");
    let root = if let Some(root) = default_root {
        Some(root.to_string())
    } else {
        // Try to detect current directory or suggest default
        let cwd = std::env::current_dir().ok();
        let suggested = cwd
            .as_ref()
            .filter(|d| d.join("pnpm-workspace.yaml").exists())
            .map(|d| d.to_string_lossy().to_string());

        if let Some(ref suggested_path) = suggested {
            eprint!("Monorepo root [{}]: ", suggested_path);
        } else {
            eprint!("Monorepo root (absolute path, or leave empty to skip): ");
        }
        stdout.flush().map_err(crate::NovaNetError::Io)?;

        let input = read_line(&stdin)?;
        if input.is_empty() {
            suggested
        } else {
            Some(input)
        }
    };

    // Validate root if provided
    if let Some(ref root_path) = root {
        let path = std::path::Path::new(root_path);
        if !path.join("pnpm-workspace.yaml").exists() {
            eprintln!(
                "  Warning: {} does not contain pnpm-workspace.yaml",
                root_path
            );
        } else {
            eprintln!("  Root path validated");
        }
    }

    eprintln!();
    eprintln!("Neo4j Configuration");
    eprintln!("-------------------");

    // Neo4j URI
    let uri_default = default_uri.unwrap_or("bolt://localhost:7687");
    eprint!("Neo4j URI [{}]: ", uri_default);
    stdout.flush().map_err(crate::NovaNetError::Io)?;
    let uri = read_line_or_default(&stdin, uri_default)?;

    // Neo4j User
    let user_default = default_user.unwrap_or("neo4j");
    eprint!("Neo4j User [{}]: ", user_default);
    stdout.flush().map_err(crate::NovaNetError::Io)?;
    let user = read_line_or_default(&stdin, user_default)?;

    // Neo4j Password
    let password = if let Some(pwd) = default_password {
        pwd.to_string()
    } else {
        eprint!("Neo4j Password (leave empty to skip): ");
        stdout.flush().map_err(crate::NovaNetError::Io)?;
        read_password()?
    };

    let password = if password.is_empty() {
        None
    } else {
        Some(password)
    };

    Ok(UserConfig {
        neo4j: Neo4jConfig {
            uri,
            user,
            password,
        },
        cli: CliConfig {
            root,
            ..Default::default()
        },
    })
}

fn read_line(stdin: &io::Stdin) -> crate::Result<String> {
    let mut input = String::new();
    stdin
        .read_line(&mut input)
        .map_err(crate::NovaNetError::Io)?;
    Ok(input.trim().to_string())
}

fn read_line_or_default(stdin: &io::Stdin, default: &str) -> crate::Result<String> {
    let mut input = String::new();
    stdin
        .read_line(&mut input)
        .map_err(crate::NovaNetError::Io)?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

fn read_password() -> crate::Result<String> {
    // For now, just read from stdin. In the future, could use rpassword crate.
    let stdin = io::stdin();
    let mut input = String::new();
    stdin
        .read_line(&mut input)
        .map_err(crate::NovaNetError::Io)?;
    Ok(input.trim().to_string())
}

async fn test_neo4j_connection(config: &UserConfig) -> crate::Result<()> {
    let password = config
        .neo4j
        .password
        .as_ref()
        .ok_or_else(|| crate::NovaNetError::Validation("No password provided".to_string()))?;

    let db = crate::db::Db::connect(&config.neo4j.uri, &config.neo4j.user, password).await?;

    db.execute("RETURN 1 AS ok").await?;

    Ok(())
}

/// Show current configuration status
pub fn show_config_status() {
    let summary = UserConfig::summary();

    eprintln!("NovaNet Configuration Status");
    eprintln!("=============================");
    eprintln!();
    eprintln!("  Config path: {}", summary.path.display());
    eprintln!(
        "  Config exists: {}",
        if summary.exists { "Yes" } else { "No" }
    );
    eprintln!(
        "  Neo4j configured: {}",
        if summary.neo4j_configured {
            "Yes"
        } else {
            "No"
        }
    );

    if let Ok(config) = UserConfig::load() {
        eprintln!();
        eprintln!("Current settings:");
        eprintln!(
            "  Monorepo root: {}",
            config
                .cli
                .root
                .as_deref()
                .unwrap_or("(not set - will auto-detect)")
        );
        eprintln!("  Neo4j URI: {}", config.neo4j.uri);
        eprintln!("  Neo4j User: {}", config.neo4j.user);
        eprintln!(
            "  Neo4j Password: {}",
            if config.neo4j.password.is_some() {
                "****"
            } else {
                "(not set)"
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_config_from_args_defaults() {
        let config = create_config_from_args(None, None, None, None);
        assert_eq!(config.neo4j.uri, "bolt://localhost:7687");
        assert_eq!(config.neo4j.user, "neo4j");
        assert!(config.neo4j.password.is_none());
        assert!(config.cli.root.is_none());
    }

    #[test]
    fn test_create_config_from_args_custom() {
        let config = create_config_from_args(
            Some("/path/to/novanet"),
            Some("bolt://custom:7687"),
            Some("admin"),
            Some("secret"),
        );
        assert_eq!(config.cli.root, Some("/path/to/novanet".to_string()));
        assert_eq!(config.neo4j.uri, "bolt://custom:7687");
        assert_eq!(config.neo4j.user, "admin");
        assert_eq!(config.neo4j.password, Some("secret".to_string()));
    }
}
