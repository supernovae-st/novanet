//! novanet_generate tool
//!
//! Complete generation context assembly for block or page content.
//! Orchestrates traverse, assemble, and atoms for AI agents.
//! Implements full RLM-on-KG pipeline with context anchors.
//!
//! ADR-033: Returns denomination_forms for prescriptive canonical entity references.

use crate::error::Result;
use crate::server::State;
use crate::tools::assemble::{
    self, AssembleParams, AssemblyStrategy, EvidencePacket, LocaleContext,
};
use crate::tools::atoms::{self, AtomsParams};
use crate::tools::traverse::{self, TraversalDirection, TraverseParams};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::instrument;

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
    /// Block type for task-specific spreading activation (CTA, FAQ, HERO, etc.)
    /// Phase 5.1: Enables task-specific activation thresholds and semantic boosts
    /// If not provided, will be auto-detected from Block's OF_TYPE relationship
    #[serde(default)]
    pub block_type: Option<String>,
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

/// Context build log for debugging and transparency (DX-11)
///
/// Shows step-by-step what was discovered during context assembly.
/// Useful for debugging generation issues and understanding token usage.
#[derive(Debug, Clone, Default, Serialize, JsonSchema)]
pub struct ContextBuildLog {
    /// Phase 1: Structure discovery log
    pub structure_phase: Vec<String>,
    /// Phase 2: Entity assembly log
    pub entities_phase: Vec<String>,
    /// Phase 3: Knowledge atoms log
    pub atoms_phase: Vec<String>,
    /// Phase 4: Context anchors log
    pub anchors_phase: Vec<String>,
    /// Phase 5: Token budgeting decisions
    pub token_decisions: Vec<String>,
}

/// Denomination forms for entity references (ADR-033)
///
/// Prescriptive canonical forms that the LLM MUST use when referring to entities.
/// The LLM must NOT invent, paraphrase, or use unlisted variations.
#[derive(Debug, Clone, Default, Serialize, JsonSchema)]
pub struct DenominationForm {
    /// Prose/body content form (e.g., "código qr")
    pub text: String,
    /// Heading/title form (e.g., "Código QR")
    pub title: String,
    /// Abbreviated form after first mention (e.g., "qr")
    pub abbrev: String,
    /// URL-safe slug form (e.g., "crear-codigo-qr"), post-SEO pipeline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Tech/brand hybrid for native_script locales (e.g., "QR码" for zh-CN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed: Option<String>,
    /// International reference form for native_script locales (e.g., "QR Code" in ja-JP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
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
    /// Denomination forms keyed by entity_key (ADR-033)
    ///
    /// Prescriptive canonical forms for LLM entity references.
    /// The LLM MUST use ONLY these forms - no invention, no paraphrase.
    pub denomination_forms: HashMap<String, DenominationForm>,
    /// Token usage breakdown
    pub token_usage: TokenUsage,
    /// Generation metadata
    pub metadata: GenerateMetadata,
    /// Context build log for debugging (DX-11)
    ///
    /// Step-by-step log of what was discovered during context assembly.
    pub context_build_log: ContextBuildLog,
}

/// Execute the novanet_generate tool
#[instrument(name = "novanet_generate", skip(state), fields(focus = %params.focus_key, locale = %params.locale, mode = ?params.mode))]
pub async fn execute(state: &State, params: GenerateParams) -> Result<GenerateResult> {
    let start = std::time::Instant::now();

    let token_budget = params.token_budget.unwrap_or(50_000);
    let spreading_depth = params.spreading_depth.unwrap_or(2).min(3);
    // Note: include_examples param exists for future enhancement (example injection)
    let _ = params.include_examples;

    // Initialize context build log (DX-11)
    let mut build_log = ContextBuildLog::default();

    // Phase 1: Get focus node and structure
    let structure_result =
        get_structure(state, &params.focus_key, &params.mode, spreading_depth).await?;

    // Log Phase 1
    build_log.structure_phase.push(format!(
        "Focus: {} ({})",
        params.focus_key, structure_result.focus_kind
    ));
    build_log.structure_phase.push(format!(
        "Mode: {:?}, Spreading depth: {}",
        params.mode, spreading_depth
    ));
    if structure_result.blocks_count > 0 {
        build_log.structure_phase.push(format!(
            "Discovered {} block(s): [{}]",
            structure_result.blocks_count,
            structure_result.block_keys.join(", ")
        ));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Phases 2, 3, 4 run in parallel (Phase 1 Performance Optimization)
    // ═══════════════════════════════════════════════════════════════════════════════

    // Phase 5.1: Resolve block_type with priority: params > structure_result
    // This enables task-specific spreading activation thresholds (CTA: 0.25, FAQ: 0.40, etc.)
    let block_type = params
        .block_type
        .clone()
        .or(structure_result.block_type.clone());

    // Prepare Phase 2 params: Assemble semantic context (entities)
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
        block_type, // Phase 5.1: Task-specific spreading activation
    };

    // Prepare Phase 3 params: Get knowledge atoms
    let atoms_params = AtomsParams {
        locale: params.locale.clone(),
        atom_type: crate::tools::atoms::AtomType::All,
        domain: None, // Get all domains
        register: None,
        query: None,
        limit: Some(50),
        include_containers: Some(false),
    };

    // Execute Phases 2, 3, 4 in parallel using tokio::join!
    let (assemble_result, atoms_result, context_anchors) = tokio::join!(
        assemble::execute(state, assemble_params),
        atoms::execute(state, atoms_params),
        get_context_anchors(state, &params.focus_key, &params.locale)
    );

    // Unwrap results (propagate errors)
    let assemble_result = assemble_result?;
    let atoms_result = atoms_result?;
    let context_anchors = context_anchors?;

    // Log Phase 2
    build_log.entities_phase.push(format!(
        "Visited {} nodes, collected {} evidence packets",
        assemble_result.nodes_visited,
        assemble_result.evidence.len()
    ));
    build_log.entities_phase.push(format!(
        "Entity tokens: {} (budget: {})",
        assemble_result.total_tokens,
        token_budget / 2
    ));
    if assemble_result.truncated {
        build_log
            .entities_phase
            .push("WARNING: Evidence was truncated due to token budget".to_string());
    }

    // Log Phase 3
    build_log.atoms_phase.push(format!(
        "Locale: {}, retrieved {} atoms",
        params.locale,
        atoms_result.atoms.len()
    ));
    build_log.atoms_phase.push(format!(
        "Atom tokens: {} (from {} total available)",
        atoms_result.token_estimate, atoms_result.total_count
    ));

    // Log Phase 4
    build_log.anchors_phase.push(format!(
        "Found {} context anchor(s) for cross-page linking",
        context_anchors.len()
    ));
    for anchor in &context_anchors {
        build_log
            .anchors_phase
            .push(format!("  → {} ({})", anchor.page_key, anchor.slug));
    }

    // Phase 4b: Get denomination forms (ADR-033)
    let entity_keys: Vec<String> = assemble_result
        .evidence
        .iter()
        .filter(|e| e.source_kind == "Entity" || e.source_kind == "EntityNative")
        .map(|e| {
            // For EntityNative, extract the base entity key (before @locale)
            if e.source_key.contains('@') {
                e.source_key
                    .split('@')
                    .next()
                    .unwrap_or(&e.source_key)
                    .to_string()
            } else {
                e.source_key.clone()
            }
        })
        .collect();
    let denomination_forms = fetch_denomination_forms(state, &entity_keys, &params.locale).await?;

    // Log denomination forms
    if !denomination_forms.is_empty() {
        build_log.entities_phase.push(format!(
            "ADR-033: Loaded {} denomination form(s)",
            denomination_forms.len()
        ));
    }

    // Phase 5: Calculate token usage
    let structure_tokens = structure_result.token_estimate;
    let entity_tokens = assemble_result.total_tokens;
    let knowledge_tokens = atoms_result.token_estimate;
    let locale_tokens = estimate_locale_tokens(&assemble_result.locale_context);
    let total_tokens = structure_tokens + entity_tokens + knowledge_tokens + locale_tokens;

    // Log Phase 5
    build_log.token_decisions.push(format!(
        "Token breakdown: structure={}, entities={}, knowledge={}, locale={}",
        structure_tokens, entity_tokens, knowledge_tokens, locale_tokens
    ));
    build_log.token_decisions.push(format!(
        "Total: {}/{} tokens ({:.1}% of budget)",
        total_tokens,
        token_budget,
        (total_tokens as f64 / token_budget as f64) * 100.0
    ));
    if total_tokens > token_budget {
        build_log.token_decisions.push(format!(
            "WARNING: Over budget by {} tokens",
            total_tokens - token_budget
        ));
    }

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
        denomination_forms,
        token_usage,
        metadata,
        context_build_log: build_log,
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
    /// Block type for task-specific spreading activation (CTA, FAQ, HERO, etc.)
    /// Phase 5.1: Auto-detected from Block's OF_TYPE relationship
    block_type: Option<String>,
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
                // Phase 5.1: Extract block_type for task-specific spreading activation
                let block_type = row["block_type"].as_str().map(|s| s.to_string());

                Ok(StructureResult {
                    focus_kind: row["kind"].as_str().unwrap_or("Block").to_string(),
                    focus_name: row["name"].as_str().map(|s| s.to_string()),
                    block_keys: vec![focus_key.to_string()],
                    blocks_count: 1,
                    token_estimate: 100, // Minimal structure for single block
                    block_type,
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
                // Phase 5.1: Page mode doesn't have a single block_type
                // Individual blocks will be processed with their own types
                block_type: None,
            })
        }
    }
}

/// Fetch denomination forms for entities (ADR-033)
///
/// Queries EntityNative nodes to get prescriptive canonical forms for LLM references.
async fn fetch_denomination_forms(
    state: &State,
    entity_keys: &[String],
    locale: &str,
) -> Result<HashMap<String, DenominationForm>> {
    if entity_keys.is_empty() {
        return Ok(HashMap::new());
    }

    // Build composite keys for EntityNative nodes: entity:{key}@{locale}
    let native_keys: Vec<String> = entity_keys
        .iter()
        .map(|k| {
            // Handle keys that already have "entity:" prefix
            let base_key = k.strip_prefix("entity:").unwrap_or(k);
            format!("entity:{}@{}", base_key, locale)
        })
        .collect();

    let query = r#"
        UNWIND $keys AS native_key
        MATCH (en:EntityNative {key: native_key})
        RETURN en.key AS key, en.denomination_forms AS forms
    "#;

    let mut params = serde_json::Map::new();
    params.insert("keys".to_string(), serde_json::json!(native_keys));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    let mut result = HashMap::new();
    for row in rows {
        let key = match row["key"].as_str() {
            Some(k) => k,
            None => continue,
        };

        // Extract entity key from composite key (entity:qr-code@fr-FR -> qr-code)
        let entity_key = key
            .strip_prefix("entity:")
            .and_then(|s| s.split('@').next())
            .unwrap_or(key)
            .to_string();

        // Parse denomination_forms array
        let forms = match row["forms"].as_array() {
            Some(arr) => arr,
            None => continue,
        };

        // Extract each form type
        let mut text = String::new();
        let mut title = String::new();
        let mut abbrev = String::new();
        let mut url: Option<String> = None;
        let mut mixed: Option<String> = None;
        let mut base: Option<String> = None;

        for form in forms {
            let form_type = form["type"].as_str().unwrap_or("");
            let value = form["value"].as_str().unwrap_or("").to_string();

            match form_type {
                "text" => text = value,
                "title" => title = value,
                "abbrev" => abbrev = value,
                "url" => url = Some(value),
                "mixed" => mixed = Some(value),
                "base" => base = Some(value),
                _ => {}
            }
        }

        // Only include if we have the required fields
        if !text.is_empty() && !title.is_empty() && !abbrev.is_empty() {
            result.insert(
                entity_key,
                DenominationForm {
                    text,
                    title,
                    abbrev,
                    url,
                    mixed,
                    base,
                },
            );
        }
    }

    Ok(result)
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
        OPTIONAL MATCH (p)-[:HAS_NATIVE]->(pn:PageNative)
        WHERE pn.locale = $locale OR pn IS NULL
        WITH p, pn
        WHERE p IS NOT NULL
        RETURN p.key AS page_key,
               COALESCE(pn.title, p.name, p.key) AS anchor_text,
               COALESCE(pn.slug, p.slug, '/' + p.key) AS slug
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
            block_type: None,
        };

        let structure = StructureResult {
            focus_kind: "Block".to_string(),
            focus_name: Some("Test Block".to_string()),
            block_keys: vec!["test-block".to_string()],
            blocks_count: 1,
            token_estimate: 100,
            block_type: Some("HERO".to_string()), // Phase 5.1 test
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

    // =========================================================================
    // DenominationForm Tests (ADR-033)
    // =========================================================================

    #[test]
    fn test_denomination_form_required_fields() {
        let form = DenominationForm {
            text: "código qr".to_string(),
            title: "Código QR".to_string(),
            abbrev: "qr".to_string(),
            url: None,
            mixed: None,
            base: None,
        };

        assert_eq!(form.text, "código qr");
        assert_eq!(form.title, "Código QR");
        assert_eq!(form.abbrev, "qr");
        assert!(form.url.is_none());
        assert!(form.mixed.is_none());
        assert!(form.base.is_none());
    }

    #[test]
    fn test_denomination_form_with_optional_fields() {
        let form = DenominationForm {
            text: "QRコード".to_string(),
            title: "QRコード作成".to_string(),
            abbrev: "QR".to_string(),
            url: Some("qr-code-sakusei".to_string()),
            mixed: Some("QR码".to_string()),
            base: Some("QR Code".to_string()),
        };

        assert_eq!(form.text, "QRコード");
        assert_eq!(form.title, "QRコード作成");
        assert_eq!(form.abbrev, "QR");
        assert_eq!(form.url, Some("qr-code-sakusei".to_string()));
        assert_eq!(form.mixed, Some("QR码".to_string()));
        assert_eq!(form.base, Some("QR Code".to_string()));
    }

    #[test]
    fn test_composite_key_building() {
        // Test the logic used in fetch_denomination_forms for building composite keys
        let entity_keys = ["qr-code".to_string(), "wifi".to_string()];
        let locale = "fr-FR";

        let native_keys: Vec<String> = entity_keys
            .iter()
            .map(|k| {
                let base_key = k.strip_prefix("entity:").unwrap_or(k);
                format!("entity:{}@{}", base_key, locale)
            })
            .collect();

        assert_eq!(native_keys.len(), 2);
        assert_eq!(native_keys[0], "entity:qr-code@fr-FR");
        assert_eq!(native_keys[1], "entity:wifi@fr-FR");
    }

    #[test]
    fn test_composite_key_with_prefix_handling() {
        // Keys that already have "entity:" prefix should not duplicate it
        let entity_keys = ["entity:qr-code".to_string(), "wifi".to_string()];
        let locale = "es-MX";

        let native_keys: Vec<String> = entity_keys
            .iter()
            .map(|k| {
                let base_key = k.strip_prefix("entity:").unwrap_or(k);
                format!("entity:{}@{}", base_key, locale)
            })
            .collect();

        assert_eq!(native_keys[0], "entity:qr-code@es-MX");
        assert_eq!(native_keys[1], "entity:wifi@es-MX");
    }

    #[test]
    fn test_entity_key_extraction_from_composite() {
        // Test the logic used in fetch_denomination_forms for extracting entity key
        let composite_key = "entity:qr-code@fr-FR";

        let entity_key = composite_key
            .strip_prefix("entity:")
            .and_then(|s| s.split('@').next())
            .unwrap_or(composite_key)
            .to_string();

        assert_eq!(entity_key, "qr-code");
    }

    #[test]
    fn test_entity_key_extraction_various_formats() {
        let test_cases = vec![
            ("entity:qr-code@fr-FR", "qr-code"),
            ("entity:wifi-qr@es-MX", "wifi-qr"),
            ("entity:test-entity@ja-JP", "test-entity"),
        ];

        for (composite, expected) in test_cases {
            let entity_key = composite
                .strip_prefix("entity:")
                .and_then(|s| s.split('@').next())
                .unwrap_or(composite)
                .to_string();

            assert_eq!(
                entity_key, expected,
                "Failed for composite key: {}",
                composite
            );
        }
    }

    #[test]
    fn test_denomination_form_default() {
        let form: DenominationForm = Default::default();

        assert!(form.text.is_empty());
        assert!(form.title.is_empty());
        assert!(form.abbrev.is_empty());
        assert!(form.url.is_none());
        assert!(form.mixed.is_none());
        assert!(form.base.is_none());
    }
}
