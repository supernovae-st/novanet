//! `novanet seed` command — YAML-first data persistence.
//!
//! Generates Cypher seed files from YAML source of truth.
//! Data source: `private-data/data/` (symlinked to packages/core/data)
//!
//! # Architecture
//!
//! ```text
//! private-data/data/*.yaml  →  novanet seed generate  →  seed/*.cypher  →  Neo4j
//!      ↑                                                                      │
//!      └──────────────  novanet seed import  ◄────────────────────────────────┘
//! ```
//!
//! # Security Features
//!
//! - File locking with `fs2::FileExt` for concurrent access
//! - Atomic writes (write to temp, then rename)
//! - Secrets detection (reject files with API keys, passwords)
//! - Path canonicalization to prevent traversal attacks

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Instant;

use fs2::FileExt;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};

use crate::Result;
use crate::error::NovaNetError;

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Data directory relative to monorepo root (used in error messages).
#[allow(dead_code)]
const DATA_DIR: &str = "private-data/data";

/// Output directory for generated Cypher files.
const OUTPUT_DIR: &str = "packages/db/seed/generated";

/// Lock timeout in milliseconds (5 seconds default).
const LOCK_TIMEOUT_MS: u64 = 5000;

/// Patterns that indicate secrets (reject files containing these).
const SECRET_PATTERNS: &[&str] = &[
    r"(?i)(api[_-]?key|apikey)\s*[:=]",
    r"(?i)(password|passwd|pwd)\s*[:=]",
    r"(?i)(secret|token)\s*[:=]",
    r"sk-[a-zA-Z0-9]{20,}",                         // OpenAI
    r"sk-ant-[a-zA-Z0-9-]{20,}",                    // Anthropic
    r"ghp_[a-zA-Z0-9]{36}",                         // GitHub PAT
    r"gho_[a-zA-Z0-9]{36}",                         // GitHub OAuth
    r"xoxb-[a-zA-Z0-9-]{24,}",                      // Slack Bot Token
    r"xoxp-[a-zA-Z0-9-]{24,}",                      // Slack User Token
    r"AKIA[0-9A-Z]{16}",                            // AWS Access Key ID
    r"AIza[0-9A-Za-z_-]{35}",                       // Google API Key
    r"-----BEGIN (RSA |EC |DSA )?PRIVATE KEY-----", // Private keys
    r"Bearer\s+[a-zA-Z0-9_-]{20,}",                 // Bearer tokens
];

/// Maximum YAML file size (1 MB) — prevents DoS via large files.
const MAX_YAML_FILE_SIZE: u64 = 1024 * 1024;

/// Validate a Neo4j node class name (PascalCase, no injection).
/// Returns error if the class name could cause Cypher injection.
pub fn validate_class_name(class: &str) -> Result<()> {
    // Must be PascalCase: starts with uppercase, alphanumeric only
    let re = Regex::new(r"^[A-Z][A-Za-z0-9]*$").expect("valid regex");
    if !re.is_match(class) {
        return Err(NovaNetError::Validation(format!(
            "Invalid class name '{}': must be PascalCase (e.g., Entity, EntityNative)",
            class
        )));
    }
    Ok(())
}

/// Validate and sanitize a timestamp string.
/// Returns a safe timestamp string or generates a new one if invalid.
pub fn validate_timestamp(ts: Option<&str>) -> String {
    // RFC 3339 timestamp pattern (strict validation)
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})$")
        .expect("valid regex");

    match ts {
        Some(timestamp) if re.is_match(timestamp) => timestamp.to_string(),
        _ => chrono::Utc::now().to_rfc3339(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// YAML TYPES — Data Manifest
// ═══════════════════════════════════════════════════════════════════════════════

/// Root data manifest (`_index.yaml`).
#[derive(Debug, Deserialize, Serialize)]
pub struct DataManifest {
    pub version: String,
    pub schema_version: String,
    #[serde(default)]
    pub projects: Vec<ProjectDef>,
    #[serde(default)]
    pub sources: SourcesConfig,
    #[serde(default)]
    pub generation: GenerationConfig,
}

/// Project definition within manifest.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectDef {
    pub key: String,
    pub display_name: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// Sources configuration.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SourcesConfig {
    #[serde(default)]
    pub shared: Vec<SourcePath>,
    #[serde(default)]
    pub org: Vec<SourcePath>,
}

/// Source path configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct SourcePath {
    pub path: String,
    pub class: String,
}

/// Generation settings.
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationConfig {
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_idempotent")]
    pub idempotent: bool,
}

fn default_output_dir() -> String {
    OUTPUT_DIR.to_string()
}

fn default_format() -> String {
    "cypher".to_string()
}

fn default_idempotent() -> bool {
    true
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            output_dir: default_output_dir(),
            format: default_format(),
            idempotent: default_idempotent(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// YAML TYPES — Entity Data Files
// ═══════════════════════════════════════════════════════════════════════════════

/// Entity data file.
#[derive(Debug, Deserialize)]
pub struct EntityDataFile {
    pub version: String,
    #[serde(default)]
    pub project: Option<String>,
    pub class: String,
    #[serde(default)]
    pub entities: Vec<EntityData>,
}

/// Individual entity definition.
#[derive(Debug, Deserialize)]
pub struct EntityData {
    pub key: String,
    pub display_name: String,
    pub description: String,
    #[serde(rename = "type", default)]
    pub entity_type: Option<String>,
    #[serde(default)]
    pub is_pillar: bool,
    #[serde(default)]
    pub schema_org_type: Option<String>,
    #[serde(default)]
    pub llm_context: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// YAML TYPES — EntityNative Data Files
// ═══════════════════════════════════════════════════════════════════════════════

/// EntityNative data file (one per locale).
#[derive(Debug, Deserialize)]
pub struct EntityNativeDataFile {
    pub version: String,
    pub locale: String,
    pub class: String,
    #[serde(default)]
    pub parent_class: Option<String>,
    #[serde(default)]
    pub natives: Vec<EntityNativeData>,
}

/// Individual EntityNative definition.
#[derive(Debug, Deserialize)]
pub struct EntityNativeData {
    pub entity_key: String,
    pub key: String,
    pub display_name: String,
    pub description: String,
    #[serde(default)]
    pub denomination_forms: Vec<DenominationForm>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Denomination form (ADR-033).
#[derive(Debug, Deserialize, Serialize)]
pub struct DenominationForm {
    #[serde(rename = "type")]
    pub form_type: String,
    pub value: String,
    #[serde(default = "default_priority")]
    pub priority: u8,
}

fn default_priority() -> u8 {
    1
}

// ═══════════════════════════════════════════════════════════════════════════════
// SECURITY UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════

/// Check if content contains secrets.
pub fn detect_secrets(content: &str) -> Option<String> {
    for pattern in SECRET_PATTERNS {
        if let Ok(re) = Regex::new(pattern) {
            if re.is_match(content) {
                return Some(format!("Secret detected matching pattern: {}", pattern));
            }
        }
    }
    None
}

/// Execute a function with file lock.
pub fn with_file_lock<F, T>(path: &Path, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    // Create or open lock file
    let lock_path = path.with_extension("lock");
    let lock_file = File::create(&lock_path)?;

    // Try to acquire exclusive lock with timeout
    let timeout = std::time::Duration::from_millis(LOCK_TIMEOUT_MS);
    let start = Instant::now();

    loop {
        match lock_file.try_lock_exclusive() {
            Ok(_) => break,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                if start.elapsed() > timeout {
                    return Err(NovaNetError::Io(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        format!("Failed to acquire lock on {:?} after {:?}", path, timeout),
                    )));
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            },
            Err(e) => return Err(NovaNetError::Io(e)),
        }
    }

    // Execute function with lock held
    let result = f();

    // Lock is automatically released on drop of lock_file
    drop(lock_file);
    let _ = fs::remove_file(&lock_path);

    result
}

/// Write content atomically (write to temp, then rename).
pub fn atomic_write(path: &Path, content: &str) -> Result<()> {
    let temp = path.with_extension("tmp");
    fs::write(&temp, content)?;
    fs::rename(&temp, path)?;
    Ok(())
}

/// Validate path is within allowed directory.
pub fn validate_path(path: &Path, allowed_root: &Path) -> Result<PathBuf> {
    let canonical = path.canonicalize().map_err(|e| {
        NovaNetError::Io(std::io::Error::new(
            e.kind(),
            format!("Cannot canonicalize path: {:?}", path),
        ))
    })?;

    let root_canonical = allowed_root.canonicalize().map_err(|e| {
        NovaNetError::Io(std::io::Error::new(
            e.kind(),
            format!("Cannot canonicalize root: {:?}", allowed_root),
        ))
    })?;

    if canonical.starts_with(&root_canonical) {
        Ok(canonical)
    } else {
        Err(NovaNetError::Validation(format!(
            "Path {:?} is outside allowed directory {:?}",
            path, allowed_root
        )))
    }
}

/// Safely read a YAML file with path validation and size limits.
///
/// Security features:
/// - Validates path is within allowed_root (prevents traversal)
/// - Checks file size is under MAX_YAML_FILE_SIZE (prevents DoS)
/// - Detects secrets in content (prevents credential leaks)
pub fn safe_read_yaml(file_path: &Path, allowed_root: &Path) -> Result<String> {
    // 1. Validate path is within allowed directory
    let validated_path = validate_path(file_path, allowed_root)?;

    // 2. Check file size before reading
    let metadata = fs::metadata(&validated_path)?;
    if metadata.len() > MAX_YAML_FILE_SIZE {
        return Err(NovaNetError::Validation(format!(
            "File {:?} exceeds maximum size ({} bytes > {} bytes)",
            file_path,
            metadata.len(),
            MAX_YAML_FILE_SIZE
        )));
    }

    // 3. Read file content
    let mut content = String::new();
    File::open(&validated_path)?.read_to_string(&mut content)?;

    // 4. Check for secrets
    if let Some(reason) = detect_secrets(&content) {
        return Err(NovaNetError::Validation(format!(
            "File {:?} contains secrets: {}",
            file_path, reason
        )));
    }

    Ok(content)
}

// ═══════════════════════════════════════════════════════════════════════════════
// CYPHER GENERATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Escape a string for Cypher.
fn cypher_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Generate Cypher MERGE statement for an Entity.
pub fn generate_entity_cypher(entity: &EntityData) -> String {
    let mut props = vec![
        format!("key: '{}'", cypher_escape(&entity.key)),
        format!("display_name: '{}'", cypher_escape(&entity.display_name)),
        format!("description: '{}'", cypher_escape(&entity.description)),
    ];

    if let Some(ref t) = entity.entity_type {
        props.push(format!("type: '{}'", cypher_escape(t)));
    }
    if entity.is_pillar {
        props.push("is_pillar: true".to_string());
    }
    if let Some(ref s) = entity.schema_org_type {
        props.push(format!("schema_org_type: '{}'", cypher_escape(s)));
    }
    if let Some(ref c) = entity.llm_context {
        props.push(format!("llm_context: '{}'", cypher_escape(c)));
    }

    // Timestamps (with validation to prevent injection)
    let now = chrono::Utc::now().to_rfc3339();
    let created = validate_timestamp(entity.created_at.as_deref());
    props.push(format!("created_at: datetime('{}')", created));
    props.push(format!("updated_at: datetime('{}')", now));

    format!(
        "MERGE (e:Entity {{key: '{}'}})\nON CREATE SET e += {{{}}}\nON MATCH SET e.updated_at = datetime('{}');",
        cypher_escape(&entity.key),
        props.join(", "),
        now
    )
}

/// Generate Cypher MERGE statement for an EntityNative.
pub fn generate_entity_native_cypher(native: &EntityNativeData, locale: &str) -> String {
    let mut props = vec![
        format!("key: '{}'", cypher_escape(&native.key)),
        format!("entity_key: '{}'", cypher_escape(&native.entity_key)),
        format!("locale: '{}'", cypher_escape(locale)),
        format!("display_name: '{}'", cypher_escape(&native.display_name)),
        format!("description: '{}'", cypher_escape(&native.description)),
    ];

    // Denomination forms as JSON
    if !native.denomination_forms.is_empty() {
        let forms_json = serde_json::to_string(&native.denomination_forms).unwrap_or_default();
        props.push(format!(
            "denomination_forms: '{}'",
            cypher_escape(&forms_json)
        ));
    }

    // Timestamps (with validation to prevent injection)
    let now = chrono::Utc::now().to_rfc3339();
    let created = validate_timestamp(native.created_at.as_deref());
    props.push(format!("created_at: datetime('{}')", created));
    props.push(format!("updated_at: datetime('{}')", now));

    // MERGE node + create arcs
    let mut cypher = format!(
        "MERGE (n:EntityNative {{key: '{}'}})\nON CREATE SET n += {{{}}}\nON MATCH SET n.updated_at = datetime('{}');\n",
        cypher_escape(&native.key),
        props.join(", "),
        now
    );

    // HAS_NATIVE arc (Entity → EntityNative)
    cypher.push_str(&format!(
        "MATCH (e:Entity {{key: '{}'}}), (n:EntityNative {{key: '{}'}})\nMERGE (e)-[:HAS_NATIVE]->(n);\n",
        cypher_escape(&native.entity_key),
        cypher_escape(&native.key)
    ));

    // FOR_LOCALE arc (EntityNative → Locale)
    cypher.push_str(&format!(
        "MATCH (n:EntityNative {{key: '{}'}}), (l:Locale {{key: '{}'}})\nMERGE (n)-[:FOR_LOCALE]->(l);",
        cypher_escape(&native.key),
        cypher_escape(locale)
    ));

    cypher
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMMANDS
// ═══════════════════════════════════════════════════════════════════════════════

/// Find data directory from root.
fn find_data_dir(root: &Path) -> Result<PathBuf> {
    // Try private-data/data/ at current level (novanet as submodule)
    let private_data = root.join("private-data/data");
    if private_data.exists() {
        return Ok(private_data);
    }

    // Try parent level (supernovae-agi monorepo structure)
    if let Some(parent) = root.parent() {
        let parent_private_data = parent.join("private-data/data");
        if parent_private_data.exists() {
            return Ok(parent_private_data);
        }
    }

    // Try packages/core/data (symlink within novanet)
    let pkg_data = root.join("packages/core/data");
    if pkg_data.exists() {
        return Ok(pkg_data);
    }

    Err(NovaNetError::Validation(format!(
        "Data directory not found. Expected: {:?} or parent/private-data/data",
        private_data
    )))
}

/// Generate Cypher from YAML data files.
#[instrument(skip_all)]
pub fn generate(root: Option<PathBuf>, class_filter: Option<String>, dry_run: bool) -> Result<()> {
    let root = root.ok_or_else(|| NovaNetError::Validation("No root path provided".to_string()))?;
    let data_dir = find_data_dir(&root)?;
    let output_dir = root.join(OUTPUT_DIR);

    info!("📂 Data directory: {:?}", data_dir);
    info!("📤 Output directory: {:?}", output_dir);

    if !dry_run {
        fs::create_dir_all(&output_dir)?;
    }

    let start = Instant::now();
    let mut generated_count = 0;
    let mut cypher_buffer: HashMap<String, Vec<String>> = HashMap::new();

    // Process EntityNative files
    let natives_dir = data_dir.join("natives");
    if natives_dir.exists() {
        for project_entry in fs::read_dir(&natives_dir)? {
            let project_entry = project_entry?;
            let project_path = project_entry.path();
            if !project_path.is_dir() {
                continue;
            }

            let entity_natives_dir = project_path.join("entity-natives");
            if entity_natives_dir.exists() {
                for locale_file in fs::read_dir(&entity_natives_dir)? {
                    let locale_file = locale_file?;
                    let file_path = locale_file.path();
                    if file_path
                        .extension()
                        .is_some_and(|e| e == "yaml" || e == "yml")
                    {
                        // SECURITY: Use safe_read_yaml with path validation + size limits
                        let content = match safe_read_yaml(&file_path, &data_dir) {
                            Ok(c) => c,
                            Err(e) => {
                                warn!("⚠️ Skipping {:?}: {}", file_path, e);
                                continue;
                            },
                        };

                        // Parse YAML
                        let data: EntityNativeDataFile =
                            serde_yaml::from_str(&content).map_err(|e| NovaNetError::Schema {
                                path: file_path.display().to_string(),
                                source: e,
                            })?;

                        // Apply class filter
                        if let Some(ref filter) = class_filter {
                            if data.class != *filter {
                                continue;
                            }
                        }

                        // Generate Cypher
                        let buffer = cypher_buffer
                            .entry("030-entity-natives".to_string())
                            .or_default();
                        for native in &data.natives {
                            buffer.push(generate_entity_native_cypher(native, &data.locale));
                            generated_count += 1;
                        }

                        info!(
                            "✓ Processed {:?} ({} natives)",
                            file_path.file_name().unwrap_or_default(),
                            data.natives.len()
                        );
                    }
                }
            }
        }
    }

    // Process Entity files (in entities/ directory)
    let entities_dir = data_dir.join("entities");
    if entities_dir.exists() {
        for project_entry in fs::read_dir(&entities_dir)? {
            let project_entry = project_entry?;
            let project_path = project_entry.path();
            if !project_path.is_dir() {
                continue;
            }

            for phase_file in fs::read_dir(&project_path)? {
                let phase_file = phase_file?;
                let file_path = phase_file.path();
                if file_path
                    .extension()
                    .is_some_and(|e| e == "yaml" || e == "yml")
                {
                    // SECURITY: Use safe_read_yaml with path validation + size limits
                    let content = match safe_read_yaml(&file_path, &data_dir) {
                        Ok(c) => c,
                        Err(e) => {
                            warn!("⚠️ Skipping {:?}: {}", file_path, e);
                            continue;
                        },
                    };

                    // Parse YAML (try as EntityDataFile)
                    if let Ok(data) = serde_yaml::from_str::<EntityDataFile>(&content) {
                        // Apply class filter
                        if let Some(ref filter) = class_filter {
                            if data.class != *filter {
                                continue;
                            }
                        }

                        // Generate Cypher
                        let buffer = cypher_buffer.entry("020-entities".to_string()).or_default();
                        for entity in &data.entities {
                            buffer.push(generate_entity_cypher(entity));
                            generated_count += 1;
                        }

                        info!(
                            "✓ Processed {:?} ({} entities)",
                            file_path.file_name().unwrap_or_default(),
                            data.entities.len()
                        );
                    }
                }
            }
        }
    }

    // Write output files
    if dry_run {
        println!("\n📋 Dry run — would generate:");
        for (name, statements) in &cypher_buffer {
            println!("  - {}.cypher ({} statements)", name, statements.len());
        }
    } else {
        for (name, statements) in &cypher_buffer {
            let output_path = output_dir.join(format!("{}.cypher", name));
            let content = format!(
                "// Generated by novanet seed generate\n// DO NOT EDIT — regenerate with: novanet seed generate\n\n{}",
                statements.join("\n\n")
            );
            atomic_write(&output_path, &content)?;
            println!(
                "✓ Wrote {:?} ({} statements)",
                output_path,
                statements.len()
            );
        }
    }

    let elapsed = start.elapsed();
    println!(
        "\n✅ Generated {} statements in {:?}",
        generated_count, elapsed
    );

    Ok(())
}

/// Validate YAML data files against schema.
#[instrument(skip_all)]
pub fn validate(root: Option<PathBuf>, _fix: bool) -> Result<()> {
    let root = root.ok_or_else(|| NovaNetError::Validation("No root path provided".to_string()))?;
    let data_dir = find_data_dir(&root)?;

    info!("🔍 Validating data directory: {:?}", data_dir);

    let start = Instant::now();
    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut file_count = 0;

    // Validate manifest
    let manifest_path = data_dir.join("_index.yaml");
    if manifest_path.exists() {
        match safe_read_yaml(&manifest_path, &data_dir) {
            Ok(content) => match serde_yaml::from_str::<DataManifest>(&content) {
                Ok(_manifest) => info!("✓ Manifest valid"),
                Err(e) => errors.push(format!("_index.yaml: {}", e)),
            },
            Err(e) => errors.push(format!("_index.yaml: {}", e)),
        }
        file_count += 1;
    } else {
        warnings.push("_index.yaml not found".to_string());
    }

    // Validate EntityNative files
    let natives_dir = data_dir.join("natives");
    if natives_dir.exists() {
        for project_entry in fs::read_dir(&natives_dir)? {
            let project_entry = project_entry?;
            let project_path = project_entry.path();
            if !project_path.is_dir() {
                continue;
            }

            let entity_natives_dir = project_path.join("entity-natives");
            if entity_natives_dir.exists() {
                for locale_file in fs::read_dir(&entity_natives_dir)? {
                    let locale_file = locale_file?;
                    let file_path = locale_file.path();
                    if file_path
                        .extension()
                        .is_some_and(|e| e == "yaml" || e == "yml")
                    {
                        file_count += 1;

                        // SECURITY: Use safe_read_yaml with path validation + size limits
                        let content = match safe_read_yaml(&file_path, &data_dir) {
                            Ok(c) => c,
                            Err(e) => {
                                errors.push(format!("{:?}: {}", file_path, e));
                                continue;
                            },
                        };

                        // Parse and validate
                        match serde_yaml::from_str::<EntityNativeDataFile>(&content) {
                            Ok(data) => {
                                // Validate each native
                                for native in &data.natives {
                                    // Key format validation (ADR-036)
                                    if !native.key.contains('@') {
                                        errors.push(format!(
                                            "{:?}: key '{}' missing @ separator",
                                            file_path, native.key
                                        ));
                                    }

                                    // Denomination forms validation
                                    if native.denomination_forms.is_empty() {
                                        warnings.push(format!(
                                            "{:?}: native '{}' has no denomination_forms",
                                            file_path, native.key
                                        ));
                                    }
                                }
                            },
                            Err(e) => errors.push(format!("{:?}: {}", file_path, e)),
                        }
                    }
                }
            }
        }
    }

    let elapsed = start.elapsed();

    // Print results
    println!("\n📊 Validation Results");
    println!("   Files checked: {}", file_count);
    println!("   Errors: {}", errors.len());
    println!("   Warnings: {}", warnings.len());
    println!("   Time: {:?}", elapsed);

    if !errors.is_empty() {
        println!("\n❌ Errors:");
        for e in &errors {
            println!("   • {}", e);
        }
    }

    if !warnings.is_empty() {
        println!("\n⚠️ Warnings:");
        for w in &warnings {
            println!("   • {}", w);
        }
    }

    if errors.is_empty() {
        println!("\n✅ Validation passed");
        Ok(())
    } else {
        Err(NovaNetError::Validation(format!(
            "{} validation errors found",
            errors.len()
        )))
    }
}

/// Compare YAML data with Neo4j database.
#[instrument(skip_all)]
pub async fn diff(
    root: Option<PathBuf>,
    pool: &neo4rs::Graph,
    class_filter: Option<String>,
) -> Result<()> {
    let root = root.ok_or_else(|| NovaNetError::Validation("No root path provided".to_string()))?;
    let data_dir = find_data_dir(&root)?;

    info!("🔍 Comparing YAML with Neo4j");

    // Collect YAML keys
    let mut yaml_keys: HashMap<String, Vec<String>> = HashMap::new();

    // Read EntityNative files
    let natives_dir = data_dir.join("natives");
    if natives_dir.exists() {
        for project_entry in fs::read_dir(&natives_dir)? {
            let project_entry = project_entry?;
            let project_path = project_entry.path();
            if !project_path.is_dir() {
                continue;
            }

            let entity_natives_dir = project_path.join("entity-natives");
            if entity_natives_dir.exists() {
                for locale_file in fs::read_dir(&entity_natives_dir)? {
                    let locale_file = locale_file?;
                    let file_path = locale_file.path();
                    if file_path
                        .extension()
                        .is_some_and(|e| e == "yaml" || e == "yml")
                    {
                        // SECURITY: Use safe_read_yaml with path validation + size limits
                        let content = match safe_read_yaml(&file_path, &data_dir) {
                            Ok(c) => c,
                            Err(e) => {
                                warn!("⚠️ Skipping {:?}: {}", file_path, e);
                                continue;
                            },
                        };
                        if let Ok(data) = serde_yaml::from_str::<EntityNativeDataFile>(&content) {
                            // SECURITY: Validate class name before storing
                            if validate_class_name(&data.class).is_err() {
                                warn!("⚠️ Skipping {:?}: invalid class name", file_path);
                                continue;
                            }
                            let keys = yaml_keys.entry(data.class.clone()).or_default();
                            for native in &data.natives {
                                keys.push(native.key.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    // Query Neo4j for each class
    let mut in_yaml_not_neo4j: Vec<String> = Vec::new();
    let mut in_neo4j_not_yaml: Vec<String> = Vec::new();

    for (class, keys) in &yaml_keys {
        if let Some(ref filter) = class_filter {
            if class != filter {
                continue;
            }
        }

        // SECURITY: Validate class name before using in Cypher query
        validate_class_name(class)?;

        // Query Neo4j (class name is now validated as safe PascalCase)
        let query = format!("MATCH (n:{}) RETURN n.key AS key", class);
        let mut result =
            pool.execute(neo4rs::query(&query))
                .await
                .map_err(|e| NovaNetError::Query {
                    query: query.clone(),
                    source: e,
                })?;

        let mut neo4j_keys: Vec<String> = Vec::new();
        while let Some(row) = result.next().await.map_err(|e| NovaNetError::Query {
            query: query.clone(),
            source: e,
        })? {
            if let Ok(key) = row.get::<String>("key") {
                neo4j_keys.push(key);
            }
        }

        // Compare
        for key in keys {
            if !neo4j_keys.contains(key) {
                in_yaml_not_neo4j.push(format!("+ {} ({})", key, class));
            }
        }

        for key in &neo4j_keys {
            if !keys.contains(key) {
                in_neo4j_not_yaml.push(format!("- {} ({})", key, class));
            }
        }
    }

    // Print diff
    println!("\n📊 YAML ↔ Neo4j Diff");

    if in_yaml_not_neo4j.is_empty() && in_neo4j_not_yaml.is_empty() {
        println!("   ✅ No differences found");
    } else {
        if !in_yaml_not_neo4j.is_empty() {
            println!("\n   In YAML, not in Neo4j:");
            for item in &in_yaml_not_neo4j {
                println!("   {}", item);
            }
        }

        if !in_neo4j_not_yaml.is_empty() {
            println!("\n   In Neo4j, not in YAML:");
            for item in &in_neo4j_not_yaml {
                println!("   {}", item);
            }
        }
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_secrets_openai_key() {
        let content = "api_key: sk-1234567890abcdefghijklmnop";
        assert!(detect_secrets(content).is_some());
    }

    #[test]
    fn test_detect_secrets_anthropic_key() {
        let content = "key: sk-ant-abc123-def456-ghi789-jkl012";
        assert!(detect_secrets(content).is_some());
    }

    #[test]
    fn test_detect_secrets_password() {
        let content = "password: mysecret123";
        assert!(detect_secrets(content).is_some());
    }

    #[test]
    fn test_detect_secrets_github_pat() {
        let content = "token: ghp_abcdefghijklmnopqrstuvwxyz1234567890";
        assert!(detect_secrets(content).is_some());
    }

    #[test]
    fn test_detect_secrets_clean() {
        let content = r#"
            key: qr-code@fr-FR
            display_name: Code QR
            description: Un code QR est un code-barres bidimensionnel
        "#;
        assert!(detect_secrets(content).is_none());
    }

    #[test]
    fn test_cypher_escape() {
        assert_eq!(cypher_escape("it's"), "it\\'s");
        assert_eq!(cypher_escape("line1\nline2"), "line1\\nline2");
        assert_eq!(cypher_escape("back\\slash"), "back\\\\slash");
    }

    #[test]
    fn test_generate_entity_cypher() {
        let entity = EntityData {
            key: "qr-code".to_string(),
            display_name: "QR Code".to_string(),
            description: "Quick Response code".to_string(),
            entity_type: Some("THING".to_string()),
            is_pillar: true,
            schema_org_type: None,
            llm_context: None,
            created_at: None,
            updated_at: None,
        };

        let cypher = generate_entity_cypher(&entity);
        assert!(cypher.contains("MERGE (e:Entity {key: 'qr-code'})"));
        assert!(cypher.contains("is_pillar: true"));
        assert!(cypher.contains("type: 'THING'"));
    }

    #[test]
    fn test_generate_entity_native_cypher() {
        let native = EntityNativeData {
            entity_key: "qr-code".to_string(),
            key: "qr-code@fr-FR".to_string(),
            display_name: "Code QR".to_string(),
            description: "Un code QR".to_string(),
            denomination_forms: vec![DenominationForm {
                form_type: "text".to_string(),
                value: "code QR".to_string(),
                priority: 1,
            }],
            created_at: None,
            updated_at: None,
        };

        let cypher = generate_entity_native_cypher(&native, "fr-FR");
        assert!(cypher.contains("MERGE (n:EntityNative {key: 'qr-code@fr-FR'})"));
        assert!(cypher.contains("[:HAS_NATIVE]"));
        assert!(cypher.contains("[:FOR_LOCALE]"));
    }

    #[test]
    fn test_data_manifest_deserialize() {
        let yaml = r#"
            version: "1.0"
            schema_version: "0.17.2"
            projects:
              - key: qrcode-ai
                display_name: QR Code AI
            generation:
              output_dir: seed/generated
              format: cypher
              idempotent: true
        "#;

        let manifest: DataManifest = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(manifest.version, "1.0");
        assert_eq!(manifest.schema_version, "0.17.2");
        assert_eq!(manifest.projects.len(), 1);
        assert!(manifest.generation.idempotent);
    }

    #[test]
    fn test_entity_native_file_deserialize() {
        let yaml = r#"
            version: "1.0"
            locale: fr-FR
            class: EntityNative
            parent_class: Entity
            natives:
              - entity_key: qr-code
                key: qr-code@fr-FR
                display_name: Code QR
                description: Un code QR
                denomination_forms:
                  - type: text
                    value: code QR
                    priority: 1
        "#;

        let data: EntityNativeDataFile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(data.locale, "fr-FR");
        assert_eq!(data.class, "EntityNative");
        assert_eq!(data.natives.len(), 1);
        assert_eq!(data.natives[0].key, "qr-code@fr-FR");
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // SECURITY TESTS
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_validate_class_name_valid() {
        assert!(validate_class_name("Entity").is_ok());
        assert!(validate_class_name("EntityNative").is_ok());
        assert!(validate_class_name("Page").is_ok());
        assert!(validate_class_name("SEOKeyword").is_ok());
    }

    #[test]
    fn test_validate_class_name_injection() {
        // Injection attempts should fail
        assert!(validate_class_name("Entity}DETACH DELETE n//").is_err());
        assert!(validate_class_name("Entity)-[:HACK]->(:Victim").is_err());
        assert!(validate_class_name("Entity`DROP DATABASE`").is_err());
        assert!(validate_class_name("Entity' OR '1'='1").is_err());
    }

    #[test]
    fn test_validate_class_name_lowercase() {
        // Must start with uppercase
        assert!(validate_class_name("entity").is_err());
        assert!(validate_class_name("entityNative").is_err());
    }

    #[test]
    fn test_validate_class_name_special_chars() {
        // No special characters allowed
        assert!(validate_class_name("Entity_Native").is_err());
        assert!(validate_class_name("Entity-Native").is_err());
        assert!(validate_class_name("Entity.Native").is_err());
    }

    #[test]
    fn test_validate_timestamp_valid() {
        let ts = validate_timestamp(Some("2026-03-08T12:00:00Z"));
        assert_eq!(ts, "2026-03-08T12:00:00Z");

        let ts_offset = validate_timestamp(Some("2026-03-08T12:00:00+02:00"));
        assert_eq!(ts_offset, "2026-03-08T12:00:00+02:00");

        let ts_ms = validate_timestamp(Some("2026-03-08T12:00:00.123Z"));
        assert_eq!(ts_ms, "2026-03-08T12:00:00.123Z");
    }

    #[test]
    fn test_validate_timestamp_invalid() {
        // Invalid timestamps should return current time (not crash)
        let ts = validate_timestamp(Some("invalid"));
        assert!(ts.contains("T")); // Should be a valid RFC 3339

        // Injection attempt
        let ts_inject = validate_timestamp(Some("2026-03-08')); DROP DATABASE;//"));
        assert!(!ts_inject.contains("DROP"));
    }

    #[test]
    fn test_validate_timestamp_none() {
        let ts = validate_timestamp(None);
        assert!(ts.contains("T")); // Should be a valid RFC 3339
    }

    #[test]
    fn test_detect_secrets_slack() {
        let content = "token: xoxb-12345678901234567890";
        assert!(detect_secrets(content).is_some());
    }

    #[test]
    fn test_detect_secrets_aws() {
        let content = "AWS_ACCESS_KEY_ID: AKIA1234567890123456";
        assert!(detect_secrets(content).is_some());
    }

    #[test]
    fn test_detect_secrets_private_key() {
        let content = "-----BEGIN RSA PRIVATE KEY-----\nMIIEow...";
        assert!(detect_secrets(content).is_some());
    }

    #[test]
    fn test_detect_secrets_bearer() {
        let content = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        assert!(detect_secrets(content).is_some());
    }
}
