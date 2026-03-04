//! User configuration management for NovaNet CLI.
//!
//! Manages ~/.novanet/config.toml with Neo4j credentials and preferences.
//! This is distinct from the monorepo config (packages/core/models/).

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// User configuration stored in ~/.novanet/config.toml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserConfig {
    /// Neo4j connection settings
    #[serde(default)]
    pub neo4j: Neo4jConfig,

    /// CLI preferences
    #[serde(default)]
    pub cli: CliConfig,
}

/// Neo4j connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neo4jConfig {
    /// Neo4j Bolt URI (default: bolt://localhost:7687)
    #[serde(default = "default_uri")]
    pub uri: String,

    /// Neo4j username (default: neo4j)
    #[serde(default = "default_user")]
    pub user: String,

    /// Neo4j password (stored in config, not env var)
    #[serde(default)]
    pub password: Option<String>,
}

impl Default for Neo4jConfig {
    fn default() -> Self {
        Self {
            uri: default_uri(),
            user: default_user(),
            password: None,
        }
    }
}

fn default_uri() -> String {
    "bolt://localhost:7687".to_string()
}

fn default_user() -> String {
    "neo4j".to_string()
}

/// CLI preferences
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CliConfig {
    /// Default output format (table, json, cypher)
    #[serde(default)]
    pub default_format: Option<String>,

    /// Show verbose output by default
    #[serde(default)]
    pub verbose: bool,

    /// NovaNet monorepo root path (alternative to NOVANET_ROOT env var)
    #[serde(default)]
    pub root: Option<String>,
}

impl UserConfig {
    /// Get the config directory path (~/.novanet/)
    pub fn config_dir() -> PathBuf {
        dirs::home_dir()
            .map(|h| h.join(".novanet"))
            .unwrap_or_else(|| PathBuf::from(".novanet"))
    }

    /// Get the config file path (~/.novanet/config.toml)
    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    /// Check if config file exists
    pub fn exists() -> bool {
        Self::config_path().exists()
    }

    /// Load config from file, or return default if not found
    pub fn load() -> crate::Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path).map_err(crate::NovaNetError::Io)?;

        toml::from_str(&content).map_err(|e| {
            crate::NovaNetError::Validation(format!("Failed to parse config file: {}", e))
        })
    }

    /// Save config to file
    pub fn save(&self) -> crate::Result<()> {
        let dir = Self::config_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir).map_err(crate::NovaNetError::Io)?;
        }

        let path = Self::config_path();
        let content = toml::to_string_pretty(self).map_err(|e| {
            crate::NovaNetError::Validation(format!("Failed to serialize config: {}", e))
        })?;

        let mut file = fs::File::create(&path).map_err(crate::NovaNetError::Io)?;
        file.write_all(content.as_bytes())
            .map_err(crate::NovaNetError::Io)?;

        Ok(())
    }

    /// Create config directory if it doesn't exist
    pub fn ensure_dir() -> crate::Result<PathBuf> {
        let dir = Self::config_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir).map_err(crate::NovaNetError::Io)?;
        }
        Ok(dir)
    }

    /// Check if this is a first run (no config exists)
    pub fn is_first_run() -> bool {
        !Self::config_path().exists()
    }

    /// Get Neo4j password from config or environment
    pub fn get_neo4j_password(&self) -> Option<String> {
        // Priority: 1) env var  2) config file
        std::env::var("NEO4J_PASSWORD")
            .ok()
            .or_else(|| self.neo4j.password.clone())
    }

    /// Get Neo4j URI from config or environment
    pub fn get_neo4j_uri(&self) -> String {
        std::env::var("NEO4J_URI").unwrap_or_else(|_| self.neo4j.uri.clone())
    }

    /// Get Neo4j user from config or environment
    pub fn get_neo4j_user(&self) -> String {
        std::env::var("NEO4J_USER").unwrap_or_else(|_| self.neo4j.user.clone())
    }
}

/// Configuration summary for display
pub struct ConfigSummary {
    pub path: PathBuf,
    pub exists: bool,
    pub neo4j_configured: bool,
}

impl UserConfig {
    /// Get a summary of the current config state
    pub fn summary() -> ConfigSummary {
        let path = Self::config_path();
        let exists = path.exists();
        let neo4j_configured = if exists {
            Self::load()
                .map(|c| c.neo4j.password.is_some())
                .unwrap_or(false)
        } else {
            false
        };

        ConfigSummary {
            path,
            exists,
            neo4j_configured,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UserConfig::default();
        assert_eq!(config.neo4j.uri, "bolt://localhost:7687");
        assert_eq!(config.neo4j.user, "neo4j");
        assert!(config.neo4j.password.is_none());
    }

    #[test]
    fn test_config_path() {
        let path = UserConfig::config_path();
        assert!(path.to_string_lossy().contains(".novanet"));
        assert!(path.to_string_lossy().ends_with("config.toml"));
    }

    #[test]
    fn test_serialize_deserialize() {
        let config = UserConfig {
            neo4j: Neo4jConfig {
                uri: "bolt://localhost:7687".to_string(),
                user: "neo4j".to_string(),
                password: Some("secret".to_string()),
            },
            cli: CliConfig {
                default_format: Some("json".to_string()),
                verbose: true,
                root: Some("/path/to/novanet".to_string()),
            },
        };

        let toml = toml::to_string_pretty(&config).unwrap();
        let parsed: UserConfig = toml::from_str(&toml).unwrap();

        assert_eq!(parsed.neo4j.uri, config.neo4j.uri);
        assert_eq!(parsed.neo4j.password, config.neo4j.password);
        assert_eq!(parsed.cli.verbose, config.cli.verbose);
        assert_eq!(parsed.cli.root, config.cli.root);
    }
}
