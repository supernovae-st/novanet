//! `novanet knowledge generate` command.
//!
//! Generates Neo4j seed files from ATH knowledge data.
//! Currently supports:
//! - technical tier: 2-rules-slug → SlugRule + Slugification

use std::fs;
use std::path::Path;
use std::time::Instant;

use tracing::instrument;

use crate::generators::slugification::SlugificationGenerator;
use crate::Result;

/// Tier of knowledge to generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeTier {
    /// Technical rules: slugification, formatting, adaptation
    Technical,
    /// Voice and style: tone, formality, register
    Voice,
    /// Culture: references, taboos, metaphors
    Culture,
    /// Market: audience, distribution
    Market,
    /// All tiers
    All,
}

impl KnowledgeTier {
    /// Parse tier from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "technical" => Some(Self::Technical),
            "voice" => Some(Self::Voice),
            "culture" => Some(Self::Culture),
            "market" => Some(Self::Market),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

/// Result of a knowledge generation run.
pub struct KnowledgeGenerateResult {
    pub tier: String,
    pub output_path: String,
    pub bytes: usize,
    pub duration_ms: u128,
    pub node_count: usize,
}

/// Generate knowledge seed files.
///
/// # Arguments
///
/// * `root` - Monorepo root path
/// * `tier` - Which tier to generate (technical, voice, culture, market, all)
/// * `ath_path` - Optional custom ATH data path
/// * `dry_run` - If true, generate but don't write files
#[instrument(skip(root))]
pub fn knowledge_generate(
    root: &Path,
    tier: KnowledgeTier,
    ath_path: Option<&str>,
    dry_run: bool,
) -> Result<Vec<KnowledgeGenerateResult>> {
    let mut results = Vec::new();

    // Default ATH path
    let default_ath =
        "/Users/thibaut/Projects/traduction_ai/ath-know-l10n/outputs/localization-data";
    let ath = ath_path.unwrap_or(default_ath);

    match tier {
        KnowledgeTier::Technical | KnowledgeTier::All => {
            // Generate slugification
            let result = generate_slugification(root, ath, dry_run)?;
            results.push(result);
        }
        KnowledgeTier::Voice => {
            eprintln!("  Voice tier not yet implemented");
        }
        KnowledgeTier::Culture => {
            eprintln!("  Culture tier not yet implemented");
        }
        KnowledgeTier::Market => {
            eprintln!("  Market tier not yet implemented");
        }
    }

    Ok(results)
}

/// Generate slugification seed file.
fn generate_slugification(root: &Path, ath_path: &str, dry_run: bool) -> Result<KnowledgeGenerateResult> {
    let start = Instant::now();

    // Generate Cypher content
    let generator = SlugificationGenerator::with_ath_path(ath_path);
    let content = generator.generate()?;

    // Count nodes (rough estimate from MERGE statements)
    let node_count = content.matches("MERGE (").count();

    // Output path
    let output_path = root.join("packages/db/seed/22-slugification.cypher");
    let rel_path = "packages/db/seed/22-slugification.cypher";

    // Write file (unless dry run)
    if !dry_run {
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&output_path, &content)?;
    }

    let duration = start.elapsed();

    Ok(KnowledgeGenerateResult {
        tier: "technical/slugification".to_string(),
        output_path: rel_path.to_string(),
        bytes: content.len(),
        duration_ms: duration.as_millis(),
        node_count,
    })
}

/// List available knowledge tiers.
pub fn knowledge_list() -> Vec<KnowledgeTierInfo> {
    vec![
        KnowledgeTierInfo {
            tier: "technical".to_string(),
            description: "Technical rules: slugification, formatting, adaptation".to_string(),
            sources: vec![
                "2-rules-slug/*.md".to_string(),
                "2-rules-formatting/*.md (pending)".to_string(),
                "2-rules-adaptation/*.md (pending)".to_string(),
            ],
            status: "partial".to_string(),
        },
        KnowledgeTierInfo {
            tier: "voice".to_string(),
            description: "Voice and style: tone, formality, register".to_string(),
            sources: vec![
                "3-voice-tone/*.md (pending)".to_string(),
                "3-voice-formality/*.md (pending)".to_string(),
            ],
            status: "pending".to_string(),
        },
        KnowledgeTierInfo {
            tier: "culture".to_string(),
            description: "Culture: references, taboos, metaphors".to_string(),
            sources: vec![
                "4-culture-references/*.md (pending)".to_string(),
                "4-culture-taboos/*.md (pending)".to_string(),
            ],
            status: "pending".to_string(),
        },
        KnowledgeTierInfo {
            tier: "market".to_string(),
            description: "Market: audience, distribution".to_string(),
            sources: vec!["5-market/*.md (pending)".to_string()],
            status: "pending".to_string(),
        },
    ]
}

/// Information about a knowledge tier.
pub struct KnowledgeTierInfo {
    pub tier: String,
    pub description: String,
    pub sources: Vec<String>,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_tier_from_str() {
        assert_eq!(KnowledgeTier::from_str("technical"), Some(KnowledgeTier::Technical));
        assert_eq!(KnowledgeTier::from_str("TECHNICAL"), Some(KnowledgeTier::Technical));
        assert_eq!(KnowledgeTier::from_str("voice"), Some(KnowledgeTier::Voice));
        assert_eq!(KnowledgeTier::from_str("all"), Some(KnowledgeTier::All));
        assert_eq!(KnowledgeTier::from_str("invalid"), None);
    }

    #[test]
    fn test_knowledge_list() {
        let tiers = knowledge_list();
        assert_eq!(tiers.len(), 4);
        assert_eq!(tiers[0].tier, "technical");
        assert_eq!(tiers[0].status, "partial");
    }
}
