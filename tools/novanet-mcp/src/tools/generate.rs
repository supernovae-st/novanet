//! novanet_generate tool
//!
//! Complete generation context assembly for block or page content.
//! Orchestrates traverse, assemble, and atoms for AI agents.
//! Implements full RLM-on-KG pipeline with context anchors.

use crate::error::Result;
use crate::server::State;
use crate::tools::assemble::{
    self, AssembleParams, AssemblyStrategy, EvidencePacket, LocaleContext,
};
use crate::tools::atoms::{self, AtomsParams};
use crate::tools::traverse::{self, TraversalDirection, TraverseParams};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Generation mode
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum GenerateMode {
    /// Single block generation (entities, knowledge atoms)
    #[default]
    Block,
    /// Full page orchestration (structure, all blocks, cross-references)
    Page,
}

/// Parameters for novanet_generate tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct GenerateParams {
    /// Focus node key (block or page key)
    pub focus_key: String,
    /// Target locale (BCP-47)
    pub locale: String,
    /// Generation mode
    #[serde(default)]
    pub mode: GenerateMode,
    /// Maximum token budget for assembled context
    #[serde(default)]
    pub token_budget: Option<usize>,
    /// Include example content from similar blocks/pages
    #[serde(default)]
    pub include_examples: Option<bool>,
    /// Spreading activation depth (1-3)
    #[serde(default)]
    pub spreading_depth: Option<usize>,
}

/// Evidence summary in generation result
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct EvidenceSummary {
    /// Source node key
    pub source_key: String,
    /// Evidence type (entity, knowledge, structure)
    pub evidence_type: String,
    /// Relevance score (0.0 - 1.0)
    pub relevance: f64,
    /// Token count
    pub tokens: usize,
}

/// Context anchor for cross-page references
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ContextAnchor {
    /// Referenced page key
    pub page_key: String,
    /// Suggested anchor text (locale-specific)
    pub anchor_text: String,
    /// URL slug for the referenced page
    pub slug: String,
    /// Context hint for LLM (when to use this anchor)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_hint: Option<String>,
}

/// Token usage breakdown
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct TokenUsage {
    /// Tokens used for structure context
    pub structure: usize,
    /// Tokens used for entity definitions
    pub entities: usize,
    /// Tokens used for knowledge atoms
    pub knowledge: usize,
    /// Tokens used for locale context
    pub locale: usize,
    /// Total tokens used
    pub total: usize,
    /// Budget remaining
    pub budget_remaining: usize,
}

/// Generation metadata
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct GenerateMetadata {
    /// Number of blocks discovered (page mode)
    pub blocks_discovered: usize,
    /// Number of entities loaded
    pub entities_loaded: usize,
    /// Number of knowledge atoms loaded
    pub atoms_loaded: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Result from novanet_generate tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct GenerateResult {
    /// Assembled prompt ready for LLM
    pub prompt: String,
    /// Evidence summary (packets used)
    pub evidence_summary: Vec<EvidenceSummary>,
    /// Locale context (voice, culture, formatting)
    pub locale_context: LocaleContext,
    /// Context anchors for cross-page linking
    pub context_anchors: Vec<ContextAnchor>,
    /// Token usage breakdown
    pub token_usage: TokenUsage,
    /// Generation metadata
    pub metadata: GenerateMetadata,
}

/// Execute the novanet_generate tool
pub async fn execute(state: &State, params: GenerateParams) -> Result<GenerateResult> {
    let start = std::time::Instant::now();

    let token_budget = params.token_budget.unwrap_or(50_000);
    let spreading_depth = params.spreading_depth.unwrap_or(2).min(3);
    let _include_examples = params.include_examples.unwrap_or(false);

    // Phase 1: Get focus node and structure
    let structure_result =
        get_structure(state, &params.focus_key, &params.mode, spreading_depth).await?;

    // Phase 2: Assemble semantic context (entities)
    let assemble_params = AssembleParams {
        focus_key: params.focus_key.clone(),
        locale: params.locale.clone(),
        token_budget: Some(token_budget / 2), // Half budget for entities
        strategy: AssemblyStrategy::Breadth,
        include_entities: Some(true),
        include_knowledge: Some(false), // We'll get knowledge separately
        include_structure: Some(matches!(params.mode, GenerateMode::Page)),
        arc_families: None,
        max_depth: Some(spreading_depth),
    };
    let assemble_result = assemble::execute(state, assemble_params).await?;

    // Phase 3: Get knowledge atoms
    let atoms_params = AtomsParams {
        locale: params.locale.clone(),
        atom_type: crate::tools::atoms::AtomType::All,
        domain: None, // Get all domains
        register: None,
        query: None,
        limit: Some(50),
        include_containers: Some(false),
    };
    let atoms_result = atoms::execute(state, atoms_params).await?;

    // Phase 4: Get context anchors (cross-page references)
    let context_anchors = get_context_anchors(state, &params.focus_key, &params.locale).await?;

    // Phase 5: Calculate token usage
    let structure_tokens = structure_result.token_estimate;
    let entity_tokens = assemble_result.total_tokens;
    let knowledge_tokens = atoms_result.token_estimate;
    let locale_tokens = estimate_locale_tokens(&assemble_result.locale_context);
    let total_tokens = structure_tokens + entity_tokens + knowledge_tokens + locale_tokens;

    let token_usage = TokenUsage {
        structure: structure_tokens,
        entities: entity_tokens,
        knowledge: knowledge_tokens,
        locale: locale_tokens,
        total: total_tokens,
        budget_remaining: token_budget.saturating_sub(total_tokens),
    };

    // Phase 6: Build evidence summary
    let mut evidence_summary: Vec<EvidenceSummary> = assemble_result
        .evidence
        .iter()
        .map(|e| EvidenceSummary {
            source_key: e.source_key.clone(),
            evidence_type: e.evidence_type.clone(),
            relevance: e.relevance,
            tokens: e.tokens,
        })
        .collect();

    // Add atoms to evidence
    for atom in &atoms_result.atoms {
        evidence_summary.push(EvidenceSummary {
            source_key: atom.key.clone(),
            evidence_type: format!("knowledge:{}", atom.atom_type.to_lowercase()),
            relevance: 0.7, // Default relevance for atoms
            tokens: atom.value.len().div_ceil(4),
        });
    }

    // Sort by relevance
    evidence_summary.sort_by(|a, b| {
        b.relevance
            .partial_cmp(&a.relevance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Phase 7: Build the prompt
    let prompt = build_prompt(
        &params,
        &structure_result,
        &assemble_result.evidence,
        &atoms_result.atoms,
        &assemble_result.locale_context,
        &context_anchors,
    );

    let metadata = GenerateMetadata {
        blocks_discovered: structure_result.blocks_count,
        entities_loaded: assemble_result.nodes_visited,
        atoms_loaded: atoms_result.atoms.len(),
        execution_time_ms: start.elapsed().as_millis() as u64,
    };

    Ok(GenerateResult {
        prompt,
        evidence_summary,
        locale_context: assemble_result.locale_context,
        context_anchors,
        token_usage,
        metadata,
    })
}

/// Structure discovery result
struct StructureResult {
    /// Focus node info
    focus_kind: String,
    #[allow(dead_code)] // Reserved for future prompt enhancement
    focus_name: Option<String>,
    /// Block keys (for page mode)
    block_keys: Vec<String>,
    /// Block count
    blocks_count: usize,
    /// Token estimate
    token_estimate: usize,
}

/// Get structure (blocks for page, single for block)
async fn get_structure(
    state: &State,
    focus_key: &str,
    mode: &GenerateMode,
    depth: usize,
) -> Result<StructureResult> {
    match mode {
        GenerateMode::Block => {
            // Just get the block info
            let query = r#"
                MATCH (b:Block {key: $key})
                OPTIONAL MATCH (b)-[:OF_TYPE]->(bt:BlockType)
                RETURN b.key AS key, labels(b)[0] AS kind, b.name AS name, bt.name AS block_type
            "#;

            let mut params = serde_json::Map::new();
            params.insert("key".to_string(), serde_json::json!(focus_key));

            let rows = state.pool().execute_query(query, Some(params)).await?;

            if let Some(row) = rows.first() {
                Ok(StructureResult {
                    focus_kind: row["kind"].as_str().unwrap_or("Block").to_string(),
                    focus_name: row["name"].as_str().map(|s| s.to_string()),
                    block_keys: vec![focus_key.to_string()],
                    blocks_count: 1,
                    token_estimate: 100, // Minimal structure for single block
                })
            } else {
                Err(crate::error::Error::not_found(focus_key))
            }
        }
        GenerateMode::Page => {
            // Get page with all blocks
            let traverse_params = TraverseParams {
                start_key: focus_key.to_string(),
                max_depth: Some(depth),
                direction: TraversalDirection::Outgoing,
                arc_families: Some(vec!["ownership".to_string()]),
                arc_kinds: Some(vec!["HAS_BLOCK".to_string()]),
                target_kinds: Some(vec!["Block".to_string()]),
                limit: Some(50),
                include_properties: Some(true),
            };

            let traverse_result = traverse::execute(state, traverse_params).await?;

            let block_keys: Vec<String> = traverse_result
                .nodes
                .iter()
                .filter(|n| n.kind == "Block")
                .map(|n| n.key.clone())
                .collect();

            Ok(StructureResult {
                focus_kind: traverse_result.start.kind,
                focus_name: traverse_result
                    .start
                    .properties
                    .as_ref()
                    .and_then(|p| p["name"].as_str().map(|s| s.to_string())),
                blocks_count: block_keys.len(),
                block_keys,
                token_estimate: traverse_result.token_estimate,
            })
        }
    }
}

/// Get context anchors for cross-page references
async fn get_context_anchors(
    state: &State,
    focus_key: &str,
    locale: &str,
) -> Result<Vec<ContextAnchor>> {
    let query = r#"
        MATCH (focus {key: $key})
        OPTIONAL MATCH (focus)-[:REFERENCES_PAGE]->(p:Page)
        OPTIONAL MATCH (p)-[:HAS_GENERATED]->(pg:PageGenerated)
        WHERE pg.locale = $locale OR pg IS NULL
        WITH p, pg
        WHERE p IS NOT NULL
        RETURN p.key AS page_key,
               COALESCE(pg.title, p.name, p.key) AS anchor_text,
               COALESCE(pg.slug, p.slug, '/' + p.key) AS slug
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    let anchors: Vec<ContextAnchor> = rows
        .iter()
        .filter_map(|row| {
            let page_key = row["page_key"].as_str()?;
            Some(ContextAnchor {
                page_key: page_key.to_string(),
                anchor_text: row["anchor_text"].as_str().unwrap_or(page_key).to_string(),
                slug: row["slug"].as_str().unwrap_or("/").to_string(),
                context_hint: None,
            })
        })
        .collect();

    Ok(anchors)
}

/// Estimate tokens for locale context
fn estimate_locale_tokens(locale: &LocaleContext) -> usize {
    let mut tokens = 50; // Base for locale key, language, region
    if locale.voice.is_some() {
        tokens += 30;
    }
    if locale.formatting.is_some() {
        tokens += 50;
    }
    tokens
}

/// Build the generation prompt
fn build_prompt(
    params: &GenerateParams,
    structure: &StructureResult,
    evidence: &[EvidencePacket],
    atoms: &[crate::tools::atoms::Atom],
    locale: &LocaleContext,
    anchors: &[ContextAnchor],
) -> String {
    let mut prompt = String::with_capacity(8000);

    // Header
    prompt.push_str(&format!(
        "# Generation Context for {} ({})\n\n",
        params.focus_key, params.locale
    ));

    // Mode info
    prompt.push_str(&format!(
        "**Mode**: {} | **Focus**: {} ({})\n\n",
        match params.mode {
            GenerateMode::Block => "Block",
            GenerateMode::Page => "Page",
        },
        params.focus_key,
        structure.focus_kind
    ));

    // Locale section
    prompt.push_str("## Locale Context\n\n");
    prompt.push_str(&format!(
        "- **Locale**: {} ({})\n",
        locale.locale_key, locale.language
    ));
    if let Some(region) = &locale.region {
        prompt.push_str(&format!("- **Region**: {}\n", region));
    }
    if let Some(voice) = &locale.voice {
        prompt.push_str(&format!("- **Voice**: {}\n", voice));
    }
    prompt.push('\n');

    // Structure section (page mode)
    if matches!(params.mode, GenerateMode::Page) && !structure.block_keys.is_empty() {
        prompt.push_str("## Page Structure\n\n");
        prompt.push_str(&format!(
            "Blocks to generate ({}):\n",
            structure.blocks_count
        ));
        for (i, key) in structure.block_keys.iter().enumerate() {
            prompt.push_str(&format!("{}. `{}`\n", i + 1, key));
        }
        prompt.push('\n');
    }

    // Entity definitions
    if !evidence.is_empty() {
        prompt.push_str("## Entity Definitions\n\n");
        for packet in evidence.iter().filter(|e| e.evidence_type == "definition") {
            prompt.push_str(&format!(
                "### {} (relevance: {:.0}%)\n{}\n\n",
                packet.source_key,
                packet.relevance * 100.0,
                packet.content
            ));
        }
    }

    // Knowledge atoms
    if !atoms.is_empty() {
        prompt.push_str("## Knowledge Atoms\n\n");

        // Group by type
        let terms: Vec<_> = atoms.iter().filter(|a| a.atom_type == "Term").collect();
        let expressions: Vec<_> = atoms
            .iter()
            .filter(|a| a.atom_type == "Expression")
            .collect();

        if !terms.is_empty() {
            prompt.push_str("### Terms\n");
            for term in terms.iter().take(20) {
                prompt.push_str(&format!("- **{}**: {}\n", term.key, term.value));
            }
            prompt.push('\n');
        }

        if !expressions.is_empty() {
            prompt.push_str("### Expressions\n");
            for expr in expressions.iter().take(20) {
                prompt.push_str(&format!("- {}\n", expr.value));
            }
            prompt.push('\n');
        }
    }

    // Context anchors
    if !anchors.is_empty() {
        prompt.push_str("## Context Anchors (Cross-Page Links)\n\n");
        prompt.push_str(
            "Use `{{anchor:page_key|display text}}` syntax to create internal links:\n\n",
        );
        for anchor in anchors {
            prompt.push_str(&format!(
                "- `{{{{anchor:{}|{}}}}}` → {}\n",
                anchor.page_key, anchor.anchor_text, anchor.slug
            ));
        }
        prompt.push('\n');
    }

    // Generation instructions
    prompt.push_str("## Instructions\n\n");
    prompt.push_str("1. Generate content **natively** in the target locale (NOT translation)\n");
    prompt.push_str("2. Use the entity definitions for accurate terminology\n");
    prompt.push_str("3. Apply the knowledge atoms for locale-specific vocabulary\n");
    prompt.push_str("4. Follow the voice guidelines for tone and formality\n");
    if !anchors.is_empty() {
        prompt.push_str("5. Use context anchors to create natural internal links\n");
    }

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mode_default() {
        let mode: GenerateMode = Default::default();
        assert!(matches!(mode, GenerateMode::Block));
    }

    #[test]
    fn test_estimate_locale_tokens() {
        let locale = LocaleContext {
            locale_key: "fr-FR".to_string(),
            language: "French".to_string(),
            region: Some("France".to_string()),
            voice: Some("Professional".to_string()),
            formatting: None,
        };
        let tokens = estimate_locale_tokens(&locale);
        assert!(tokens >= 80);
    }

    #[test]
    fn test_build_prompt_contains_sections() {
        let params = GenerateParams {
            focus_key: "test-block".to_string(),
            locale: "fr-FR".to_string(),
            mode: GenerateMode::Block,
            token_budget: None,
            include_examples: None,
            spreading_depth: None,
        };

        let structure = StructureResult {
            focus_kind: "Block".to_string(),
            focus_name: Some("Test Block".to_string()),
            block_keys: vec!["test-block".to_string()],
            blocks_count: 1,
            token_estimate: 100,
        };

        let locale = LocaleContext {
            locale_key: "fr-FR".to_string(),
            language: "French".to_string(),
            region: Some("France".to_string()),
            voice: Some("Professional".to_string()),
            formatting: None,
        };

        let prompt = build_prompt(&params, &structure, &[], &[], &locale, &[]);

        assert!(prompt.contains("# Generation Context"));
        assert!(prompt.contains("fr-FR"));
        assert!(prompt.contains("## Locale Context"));
        assert!(prompt.contains("## Instructions"));
    }
}
