//! novanet_context tool (v0.20.0)
//!
//! Unified context assembly tool for LLM generation workflows.
//! Merges novanet_generate + novanet_assemble + novanet_atoms into one tool.
//!
//! Modes (D5):
//!   - `page`      — Full page orchestration (structure, all blocks, cross-references)
//!   - `block`     — Single block generation (entities, knowledge atoms)
//!   - `knowledge` — Retrieve locale knowledge atoms (Term, Expression, Pattern, etc.)
//!   - `assemble`  — Low-level context assembly with token budget management
//!
//! v0.20.0: Created from merge of generate.rs + assemble.rs + atoms.rs (The Great Cleanup)

mod helpers;
pub mod types;

pub use types::{
    AssemblyStrategy, AtomType, ContextMode, ContextParams, ContextResult, DenominationForm,
    EvidencePacket,
};

use crate::error::Result;
use crate::server::State;
use std::collections::HashMap;
use std::fmt::Write;
use tracing::instrument;

use helpers::*;
use types::*;

// =============================================================================
// EXECUTE — DISPATCHER
// =============================================================================

/// Execute the novanet_context tool
#[instrument(name = "novanet_context", skip(state), fields(mode = ?params.mode, locale = %params.locale))]
pub async fn execute(state: &State, params: ContextParams) -> Result<ContextResult> {
    match params.mode {
        ContextMode::Page => execute_page(state, params).await,
        ContextMode::Block => execute_block(state, params).await,
        ContextMode::Knowledge => execute_knowledge(state, params).await,
        ContextMode::Assemble => execute_assemble(state, params).await,
    }
}

// =============================================================================
// PAGE MODE — Full page orchestration
// =============================================================================

async fn execute_page(state: &State, params: ContextParams) -> Result<ContextResult> {
    let start = std::time::Instant::now();
    let focus_key = params.focus_key.as_deref().unwrap_or("unknown");
    let token_budget = params.token_budget.unwrap_or(state.config().default_token_budget);

    let mut build_log = ContextBuildLog {
        structure_phase: String::new(),
        entities_phase: String::new(),
        atoms_phase: String::new(),
        anchors_phase: String::new(),
        token_decisions: String::new(),
    };

    // Phase 1: Discover page structure via walk
    let structure = get_structure(state, focus_key).await?;
    let _ = write!(
        build_log.structure_phase,
        "Discovered {} blocks for page '{}'",
        structure.len(),
        focus_key
    );

    // Phases 2-4 in parallel: assemble entities, get atoms, get anchors
    let (entities, atoms_evidence, anchors) = tokio::join!(
        assemble_entities_internal(state, focus_key, &params.locale),
        assemble_knowledge_internal(state, &params.locale),
        get_context_anchors(state, focus_key, &params.locale),
    );

    let entities = entities?;
    let atoms_evidence = atoms_evidence?;
    let anchors = anchors?;

    let _ = write!(
        build_log.entities_phase,
        "Loaded {} entity evidence packets",
        entities.len()
    );
    let _ = write!(
        build_log.atoms_phase,
        "Loaded {} knowledge atoms",
        atoms_evidence.len()
    );
    let _ = write!(
        build_log.anchors_phase,
        "Found {} context anchors",
        anchors.len()
    );

    // Phase 4b: Get denomination forms (ADR-033)
    let denomination_forms = fetch_denomination_forms(state, focus_key, &params.locale).await?;

    // Phase 5: Calculate token usage
    let locale_context = get_locale_context(state, &params.locale).await?;
    let locale_tokens = estimate_locale_tokens(&locale_context);
    let structure_tokens = structure.iter().map(|p| p.tokens).sum::<usize>();
    let entity_tokens = entities.iter().map(|p| p.tokens).sum::<usize>();
    let knowledge_tokens = atoms_evidence.iter().map(|p| p.tokens).sum::<usize>();
    let total = locale_tokens + structure_tokens + entity_tokens + knowledge_tokens;

    let _ = write!(
        build_log.token_decisions,
        "Budget: {}, Used: {} (structure: {}, entities: {}, knowledge: {}, locale: {})",
        token_budget, total, structure_tokens, entity_tokens, knowledge_tokens, locale_tokens
    );

    let token_usage = TokenUsage {
        structure: structure_tokens,
        entities: entity_tokens,
        knowledge: knowledge_tokens,
        locale: locale_tokens,
        total,
        budget_remaining: token_budget.saturating_sub(total),
    };

    // Phase 6: Build evidence summary
    let mut all_evidence = Vec::new();
    all_evidence.extend(structure.iter().map(|p| EvidenceSummary {
        source_key: p.source_key.clone(),
        evidence_type: p.evidence_type.clone(),
        relevance: p.relevance,
        tokens: p.tokens,
    }));
    all_evidence.extend(entities.iter().map(|p| EvidenceSummary {
        source_key: p.source_key.clone(),
        evidence_type: p.evidence_type.clone(),
        relevance: p.relevance,
        tokens: p.tokens,
    }));
    all_evidence.extend(atoms_evidence.iter().map(|p| EvidenceSummary {
        source_key: p.source_key.clone(),
        evidence_type: p.evidence_type.clone(),
        relevance: p.relevance,
        tokens: p.tokens,
    }));

    // Phase 7: Build prompt
    let prompt = build_prompt(
        focus_key,
        &params.locale,
        "page",
        &locale_context,
        &entities,
        &atoms_evidence,
        &anchors,
        &denomination_forms,
    );

    let metadata = GenerateMetadata {
        blocks_discovered: structure.len(),
        entities_loaded: entities.len(),
        atoms_loaded: atoms_evidence.len(),
        execution_time_ms: start.elapsed().as_millis() as u64,
    };

    let json_string = serde_json::to_string(&all_evidence).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(ContextResult {
        mode: "page".to_string(),
        prompt: Some(prompt),
        evidence_summary: Some(all_evidence),
        locale_context: Some(locale_context),
        context_anchors: Some(anchors),
        denomination_forms: if denomination_forms.is_empty() {
            None
        } else {
            Some(denomination_forms)
        },
        token_usage: Some(token_usage),
        metadata: Some(metadata),
        context_build_log: Some(build_log),
        atoms: None,
        containers: None,
        total_count: None,
        focus: None,
        evidence: None,
        total_tokens: None,
        budget_remaining: None,
        nodes_visited: None,
        truncated: None,
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

// =============================================================================
// BLOCK MODE — Single block generation
// =============================================================================

async fn execute_block(state: &State, params: ContextParams) -> Result<ContextResult> {
    let start = std::time::Instant::now();
    let focus_key = params.focus_key.as_deref().unwrap_or("unknown");
    let token_budget = params.token_budget.unwrap_or(state.config().default_token_budget);

    let mut build_log = ContextBuildLog {
        structure_phase: String::new(),
        entities_phase: String::new(),
        atoms_phase: String::new(),
        anchors_phase: String::new(),
        token_decisions: String::new(),
    };

    let _ = write!(
        build_log.structure_phase,
        "Block mode: focusing on single block '{}'",
        focus_key
    );

    // Parallel: entities + atoms + locale context
    let (entities, atoms_evidence, locale_context) = tokio::join!(
        assemble_entities_internal(state, focus_key, &params.locale),
        assemble_knowledge_internal(state, &params.locale),
        get_locale_context(state, &params.locale),
    );

    let entities = entities?;
    let atoms_evidence = atoms_evidence?;
    let locale_context = locale_context?;

    let _ = write!(
        build_log.entities_phase,
        "Loaded {} entity evidence packets",
        entities.len()
    );
    let _ = write!(
        build_log.atoms_phase,
        "Loaded {} knowledge atoms",
        atoms_evidence.len()
    );

    // Get denomination forms
    let denomination_forms = fetch_denomination_forms(state, focus_key, &params.locale).await?;

    // Calculate token usage
    let locale_tokens = estimate_locale_tokens(&locale_context);
    let entity_tokens = entities.iter().map(|p| p.tokens).sum::<usize>();
    let knowledge_tokens = atoms_evidence.iter().map(|p| p.tokens).sum::<usize>();
    let total = locale_tokens + entity_tokens + knowledge_tokens;

    let _ = write!(
        build_log.token_decisions,
        "Budget: {}, Used: {} (entities: {}, knowledge: {}, locale: {})",
        token_budget, total, entity_tokens, knowledge_tokens, locale_tokens
    );

    let token_usage = TokenUsage {
        structure: 0,
        entities: entity_tokens,
        knowledge: knowledge_tokens,
        locale: locale_tokens,
        total,
        budget_remaining: token_budget.saturating_sub(total),
    };

    // Build evidence summary
    let mut all_evidence = Vec::new();
    all_evidence.extend(entities.iter().map(|p| EvidenceSummary {
        source_key: p.source_key.clone(),
        evidence_type: p.evidence_type.clone(),
        relevance: p.relevance,
        tokens: p.tokens,
    }));
    all_evidence.extend(atoms_evidence.iter().map(|p| EvidenceSummary {
        source_key: p.source_key.clone(),
        evidence_type: p.evidence_type.clone(),
        relevance: p.relevance,
        tokens: p.tokens,
    }));

    // Build prompt
    let anchors = Vec::new();
    let prompt = build_prompt(
        focus_key,
        &params.locale,
        "block",
        &locale_context,
        &entities,
        &atoms_evidence,
        &anchors,
        &denomination_forms,
    );

    let metadata = GenerateMetadata {
        blocks_discovered: 1,
        entities_loaded: entities.len(),
        atoms_loaded: atoms_evidence.len(),
        execution_time_ms: start.elapsed().as_millis() as u64,
    };

    let json_string = serde_json::to_string(&all_evidence).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(ContextResult {
        mode: "block".to_string(),
        prompt: Some(prompt),
        evidence_summary: Some(all_evidence),
        locale_context: Some(locale_context),
        context_anchors: None,
        denomination_forms: if denomination_forms.is_empty() {
            None
        } else {
            Some(denomination_forms)
        },
        token_usage: Some(token_usage),
        metadata: Some(metadata),
        context_build_log: Some(build_log),
        atoms: None,
        containers: None,
        total_count: None,
        focus: None,
        evidence: None,
        total_tokens: None,
        budget_remaining: None,
        nodes_visited: None,
        truncated: None,
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

// =============================================================================
// KNOWLEDGE MODE — Locale knowledge atoms
// =============================================================================

async fn execute_knowledge(state: &State, params: ContextParams) -> Result<ContextResult> {
    let start = std::time::Instant::now();
    let limit = params.limit.unwrap_or(50).min(200);
    let include_containers = params.include_containers.unwrap_or(false);
    let atom_type = params.atom_type.clone().unwrap_or_default();

    let mut all_atoms = Vec::new();

    match atom_type {
        AtomType::Term => {
            all_atoms.extend(fetch_atoms(state, &params, &TERM_CONFIG, limit).await?);
        }
        AtomType::Expression => {
            all_atoms.extend(fetch_atoms(state, &params, &EXPRESSION_CONFIG, limit).await?);
        }
        AtomType::Pattern => {
            all_atoms.extend(fetch_atoms(state, &params, &PATTERN_CONFIG, limit).await?);
        }
        AtomType::CultureRef => {
            all_atoms.extend(fetch_atoms(state, &params, &CULTURE_REF_CONFIG, limit).await?);
        }
        AtomType::Taboo => {
            all_atoms.extend(fetch_atoms(state, &params, &TABOO_CONFIG, limit).await?);
        }
        AtomType::AudienceTrait => {
            all_atoms
                .extend(fetch_atoms(state, &params, &AUDIENCE_TRAIT_CONFIG, limit).await?);
        }
        AtomType::All => {
            let per_type_limit = (limit / 6).max(5);
            let (terms, expressions, patterns, culture_refs, taboos, audience_traits) =
                tokio::try_join!(
                    fetch_atoms(state, &params, &TERM_CONFIG, per_type_limit),
                    fetch_atoms(state, &params, &EXPRESSION_CONFIG, per_type_limit),
                    fetch_atoms(state, &params, &PATTERN_CONFIG, per_type_limit),
                    fetch_atoms(state, &params, &CULTURE_REF_CONFIG, per_type_limit),
                    fetch_atoms(state, &params, &TABOO_CONFIG, per_type_limit),
                    fetch_atoms(state, &params, &AUDIENCE_TRAIT_CONFIG, per_type_limit),
                )?;
            all_atoms.extend(terms);
            all_atoms.extend(expressions);
            all_atoms.extend(patterns);
            all_atoms.extend(culture_refs);
            all_atoms.extend(taboos);
            all_atoms.extend(audience_traits);
        }
    }

    let containers = if include_containers {
        Some(get_containers(state, &params.locale).await?)
    } else {
        None
    };

    let total_count = all_atoms.len();
    let json_string = serde_json::to_string(&all_atoms).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(ContextResult {
        mode: "knowledge".to_string(),
        prompt: None,
        evidence_summary: None,
        locale_context: None,
        context_anchors: None,
        denomination_forms: None,
        token_usage: None,
        metadata: None,
        context_build_log: None,
        atoms: Some(all_atoms),
        containers,
        total_count: Some(total_count),
        focus: None,
        evidence: None,
        total_tokens: None,
        budget_remaining: None,
        nodes_visited: None,
        truncated: None,
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

// =============================================================================
// ASSEMBLE MODE — Low-level context assembly
// =============================================================================

async fn execute_assemble(state: &State, params: ContextParams) -> Result<ContextResult> {
    let start = std::time::Instant::now();
    let focus_key = params.focus_key.as_deref().unwrap_or("unknown");
    let token_budget = params.token_budget.unwrap_or(state.config().default_token_budget);

    let include_entities = params.include_entities.unwrap_or(true);
    let include_knowledge = params.include_knowledge.unwrap_or(true);
    let include_structure = params.include_structure.unwrap_or(true);

    // If arc_families is specified, it overrides include_* flags
    let has_arc_filter = params.arc_families.is_some();

    // Parallel: locale context + evidence assembly
    let (locale_result, entity_evidence, knowledge_evidence, structure_evidence) = tokio::join!(
        get_locale_context(state, &params.locale),
        async {
            if include_entities || has_arc_filter {
                assemble_entities_for_focus(state, focus_key, &params.locale).await
            } else {
                Ok(Vec::new())
            }
        },
        async {
            if include_knowledge || has_arc_filter {
                assemble_knowledge_for_focus(state, &params.locale).await
            } else {
                Ok(Vec::new())
            }
        },
        async {
            if include_structure || has_arc_filter {
                assemble_structure_for_focus(state, focus_key).await
            } else {
                Ok(Vec::new())
            }
        },
    );

    let locale_context = locale_result?;
    let mut entity_evidence = entity_evidence?;
    let mut knowledge_evidence = knowledge_evidence?;
    let mut structure_evidence = structure_evidence?;

    // Apply spreading activation relevance from config
    let spreading_config = state.spreading_config();
    let task_key = params
        .block_type
        .as_deref()
        .unwrap_or("DEFAULT")
        .to_uppercase();
    let default_modifier = crate::activation::TaskModifier {
        activation_threshold: Some(0.30),
        propagation_steps: Some(2),
        semantic_boosts: HashMap::new(),
        priority_filter: vec![
            "critical".to_string(),
            "high".to_string(),
            "medium".to_string(),
        ],
    };
    let modifier = spreading_config
        .task_modifiers
        .get(&task_key)
        .or_else(|| spreading_config.task_modifiers.get("DEFAULT"))
        .unwrap_or(&default_modifier);
    let threshold = modifier.activation_threshold.unwrap_or(0.30);

    // Filter by activation threshold
    entity_evidence.retain(|p| p.relevance >= threshold);
    knowledge_evidence.retain(|p| p.relevance >= threshold);
    structure_evidence.retain(|p| p.relevance >= threshold);

    // Combine all evidence
    let mut all_evidence = Vec::new();
    all_evidence.extend(entity_evidence);
    all_evidence.extend(knowledge_evidence);
    all_evidence.extend(structure_evidence);

    // Sort by relevance descending
    all_evidence.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal));

    // Truncate to token budget
    let mut total_tokens = 0;
    let mut truncated = false;
    all_evidence.retain(|p| {
        if total_tokens + p.tokens <= token_budget {
            total_tokens += p.tokens;
            true
        } else {
            truncated = true;
            false
        }
    });

    let nodes_visited = all_evidence.len();

    // Get focus node info
    let focus = get_focus_node(state, focus_key).await?;

    let json_string = serde_json::to_string(&all_evidence).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(ContextResult {
        mode: "assemble".to_string(),
        prompt: None,
        evidence_summary: None,
        locale_context: Some(locale_context),
        context_anchors: None,
        denomination_forms: None,
        token_usage: None,
        metadata: None,
        context_build_log: None,
        atoms: None,
        containers: None,
        total_count: None,
        focus: Some(focus),
        evidence: Some(all_evidence),
        total_tokens: Some(total_tokens),
        budget_remaining: Some(token_budget.saturating_sub(total_tokens)),
        nodes_visited: Some(nodes_visited),
        truncated: Some(truncated),
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- ContextMode tests ---------------------------------------------------

    #[test]
    fn test_context_mode_default_is_block() {
        let mode: ContextMode = Default::default();
        assert!(matches!(mode, ContextMode::Block));
    }

    #[test]
    fn test_context_mode_deserialize_page() {
        let json = r#""page""#;
        let mode: ContextMode = serde_json::from_str(json).unwrap();
        assert!(matches!(mode, ContextMode::Page));
    }

    #[test]
    fn test_context_mode_deserialize_knowledge() {
        let json = r#""knowledge""#;
        let mode: ContextMode = serde_json::from_str(json).unwrap();
        assert!(matches!(mode, ContextMode::Knowledge));
    }

    #[test]
    fn test_context_mode_deserialize_assemble() {
        let json = r#""assemble""#;
        let mode: ContextMode = serde_json::from_str(json).unwrap();
        assert!(matches!(mode, ContextMode::Assemble));
    }

    // --- AssemblyStrategy tests ----------------------------------------------

    #[test]
    fn test_assembly_strategy_default_is_breadth() {
        let strategy: AssemblyStrategy = Default::default();
        assert!(matches!(strategy, AssemblyStrategy::Breadth));
    }

    // --- AtomType tests ------------------------------------------------------

    #[test]
    fn test_atom_type_default_is_all() {
        let atom_type: AtomType = Default::default();
        assert!(matches!(atom_type, AtomType::All));
    }

    #[test]
    fn test_atom_type_deserialize_term() {
        let json = r#""term""#;
        let atom_type: AtomType = serde_json::from_str(json).unwrap();
        assert!(matches!(atom_type, AtomType::Term));
    }

    #[test]
    fn test_atom_type_deserialize_expression() {
        let json = r#""expression""#;
        let atom_type: AtomType = serde_json::from_str(json).unwrap();
        assert!(matches!(atom_type, AtomType::Expression));
    }

    // --- ContextParams tests -------------------------------------------------

    #[test]
    fn test_context_params_minimal() {
        let json = r#"{"locale": "fr-FR"}"#;
        let params: ContextParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.locale, "fr-FR");
        assert!(matches!(params.mode, ContextMode::Block));
        assert!(params.focus_key.is_none());
    }

    #[test]
    fn test_context_params_page_mode() {
        let json = r#"{
            "focus_key": "homepage",
            "locale": "fr-FR",
            "mode": "page",
            "token_budget": 50000,
            "spreading_depth": 2
        }"#;
        let params: ContextParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.focus_key, Some("homepage".to_string()));
        assert!(matches!(params.mode, ContextMode::Page));
        assert_eq!(params.token_budget, Some(50000));
        assert_eq!(params.spreading_depth, Some(2));
    }

    #[test]
    fn test_context_params_knowledge_mode() {
        let json = r#"{
            "locale": "es-MX",
            "mode": "knowledge",
            "atom_type": "expression",
            "domain": "technical",
            "register": "formal",
            "limit": 100
        }"#;
        let params: ContextParams = serde_json::from_str(json).unwrap();
        assert!(matches!(params.mode, ContextMode::Knowledge));
        assert!(matches!(params.atom_type, Some(AtomType::Expression)));
        assert_eq!(params.domain, Some("technical".to_string()));
        assert_eq!(params.register, Some("formal".to_string()));
        assert_eq!(params.limit, Some(100));
    }

    #[test]
    fn test_context_params_assemble_mode() {
        let json = r#"{
            "focus_key": "homepage",
            "locale": "fr-FR",
            "mode": "assemble",
            "strategy": "relevance",
            "include_entities": true,
            "include_knowledge": false,
            "max_depth": 4
        }"#;
        let params: ContextParams = serde_json::from_str(json).unwrap();
        assert!(matches!(params.mode, ContextMode::Assemble));
        assert!(matches!(params.strategy, Some(AssemblyStrategy::Relevance)));
        assert_eq!(params.include_entities, Some(true));
        assert_eq!(params.include_knowledge, Some(false));
        assert_eq!(params.max_depth, Some(4));
    }

    // --- DenominationForm tests ----------------------------------------------

    #[test]
    fn test_denomination_form_serialize_minimal() {
        let form = DenominationForm {
            text: Some("code QR".to_string()),
            title: None,
            abbrev: None,
            url: None,
            mixed: None,
            base: None,
        };
        let json = serde_json::to_value(&form).unwrap();
        assert_eq!(json["text"], "code QR");
        assert!(json.get("title").is_none());
    }

    #[test]
    fn test_denomination_form_serialize_full() {
        let form = DenominationForm {
            text: Some("code QR".to_string()),
            title: Some("Code QR".to_string()),
            abbrev: Some("QR".to_string()),
            url: Some("code-qr".to_string()),
            mixed: None,
            base: None,
        };
        let json = serde_json::to_value(&form).unwrap();
        assert_eq!(json["text"], "code QR");
        assert_eq!(json["title"], "Code QR");
        assert_eq!(json["abbrev"], "QR");
        assert_eq!(json["url"], "code-qr");
    }

    // --- EvidencePacket tests ------------------------------------------------

    #[test]
    fn test_evidence_packet_serialize() {
        let packet = EvidencePacket {
            source_key: "qr-code".to_string(),
            source_kind: "Entity".to_string(),
            evidence_type: "definition".to_string(),
            distance: 1,
            relevance: 0.95,
            content: "QR code generator".to_string(),
            tokens: 10,
        };
        let json = serde_json::to_value(&packet).unwrap();
        assert_eq!(json["source_key"], "qr-code");
        assert_eq!(json["relevance"], 0.95);
    }

    // --- Atom tests ----------------------------------------------------------

    #[test]
    fn test_atom_serialize_minimal() {
        let atom = Atom {
            key: "qr-code".to_string(),
            atom_type: "Term".to_string(),
            value: "code QR".to_string(),
            domain: None,
            register: None,
            properties: None,
            container_key: None,
        };
        let json = serde_json::to_value(&atom).unwrap();
        assert_eq!(json["key"], "qr-code");
        assert_eq!(json["atom_type"], "Term");
        assert!(json.get("domain").is_none());
    }

    // --- Prompt building tests -----------------------------------------------

    #[test]
    fn test_build_prompt_includes_locale() {
        let locale_ctx = LocaleContext {
            locale_key: "fr-FR".to_string(),
            language: "French".to_string(),
            region: Some("France".to_string()),
            voice: Some("Professional".to_string()),
            formatting: None,
        };
        let prompt = build_prompt(
            "homepage",
            "fr-FR",
            "page",
            &locale_ctx,
            &[],
            &[],
            &[],
            &HashMap::new(),
        );
        assert!(prompt.contains("French"));
        assert!(prompt.contains("France"));
        assert!(prompt.contains("Professional"));
        assert!(prompt.contains("homepage"));
    }

    #[test]
    fn test_build_prompt_includes_denomination_forms() {
        let locale_ctx = LocaleContext {
            locale_key: "fr-FR".to_string(),
            language: "French".to_string(),
            region: None,
            voice: None,
            formatting: None,
        };
        let mut forms = HashMap::new();
        forms.insert(
            "qr-code".to_string(),
            DenominationForm {
                text: Some("code QR".to_string()),
                title: Some("Code QR".to_string()),
                abbrev: None,
                url: None,
                mixed: None,
                base: None,
            },
        );
        let prompt = build_prompt(
            "homepage",
            "fr-FR",
            "block",
            &locale_ctx,
            &[],
            &[],
            &[],
            &forms,
        );
        assert!(prompt.contains("ADR-033"));
        assert!(prompt.contains("code QR"));
        assert!(prompt.contains("Code QR"));
    }

    #[test]
    fn test_estimate_locale_tokens() {
        let ctx = LocaleContext {
            locale_key: "fr-FR".to_string(),
            language: "French".to_string(),
            region: Some("France".to_string()),
            voice: None,
            formatting: None,
        };
        let tokens = estimate_locale_tokens(&ctx);
        assert!(tokens > 0);
        assert!(tokens < 100); // Sanity check
    }

    // --- AtomConfig tests ----------------------------------------------------

    #[test]
    fn test_term_config_values() {
        assert_eq!(TERM_CONFIG.locale_arc, "HAS_TERMS");
        assert_eq!(TERM_CONFIG.container_label, "TermSet");
        assert_eq!(TERM_CONFIG.atom_label, "Term");
        assert_eq!(TERM_CONFIG.filter_field, Some("domain"));
    }

    #[test]
    fn test_expression_config_values() {
        assert_eq!(EXPRESSION_CONFIG.locale_arc, "HAS_EXPRESSIONS");
        assert_eq!(EXPRESSION_CONFIG.filter_field, Some("register"));
    }

    #[test]
    fn test_pattern_config_has_no_filter() {
        assert_eq!(PATTERN_CONFIG.filter_field, None);
        assert_eq!(PATTERN_CONFIG.value_property, "template");
    }

    // --- ContextResult tests -------------------------------------------------

    #[test]
    fn test_context_result_knowledge_mode_serialize() {
        let result = ContextResult {
            mode: "knowledge".to_string(),
            prompt: None,
            evidence_summary: None,
            locale_context: None,
            context_anchors: None,
            denomination_forms: None,
            token_usage: None,
            metadata: None,
            context_build_log: None,
            atoms: Some(vec![Atom {
                key: "test".to_string(),
                atom_type: "Term".to_string(),
                value: "test value".to_string(),
                domain: None,
                register: None,
                properties: None,
                container_key: None,
            }]),
            containers: None,
            total_count: Some(1),
            focus: None,
            evidence: None,
            total_tokens: None,
            budget_remaining: None,
            nodes_visited: None,
            truncated: None,
            token_estimate: 50,
            execution_time_ms: 25,
        };
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["mode"], "knowledge");
        assert_eq!(json["atoms"].as_array().unwrap().len(), 1);
        assert!(json.get("prompt").is_none()); // Skipped when None
    }
}
