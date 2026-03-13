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

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use tracing::instrument;

// ═══════════════════════════════════════════════════════════════════════════════
// ENUMS
// ═══════════════════════════════════════════════════════════════════════════════

/// Context assembly mode
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ContextMode {
    /// Full page orchestration (structure, all blocks, cross-references)
    Page,
    /// Single block generation (entities, knowledge atoms)
    #[default]
    Block,
    /// Retrieve locale knowledge atoms (Term, Expression, Pattern, etc.)
    Knowledge,
    /// Low-level context assembly with token budget management
    Assemble,
}

/// Assembly strategy (for assemble mode)
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AssemblyStrategy {
    /// Breadth-first traversal from focus node
    #[default]
    Breadth,
    /// Depth-first traversal following ownership arcs
    Depth,
    /// Prioritize by relevance score
    Relevance,
    /// Custom traversal order via arc families
    Custom,
}

/// Type of knowledge atom to retrieve (for knowledge mode)
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AtomType {
    /// Technical terms with definitions
    Term,
    /// Idiomatic expressions
    Expression,
    /// Text patterns/templates
    Pattern,
    /// Cultural references
    CultureRef,
    /// Cultural taboos to avoid
    Taboo,
    /// Audience characteristics
    AudienceTrait,
    /// All atom types
    #[default]
    All,
}

// ═══════════════════════════════════════════════════════════════════════════════
// PARAMS
// ═══════════════════════════════════════════════════════════════════════════════

/// Parameters for novanet_context tool
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
pub struct ContextParams {
    /// Focus node key (block or page key) — required for page/block/assemble modes
    pub focus_key: Option<String>,
    /// Target locale (BCP-47) — required for all modes
    pub locale: String,
    /// Context mode (page, block, knowledge, assemble)
    #[serde(default)]
    pub mode: ContextMode,
    /// Maximum token budget for assembled context
    #[serde(default)]
    pub token_budget: Option<usize>,

    // ─── Page/Block mode params ─────────────────────────────────────────────

    /// Include example content from similar blocks/pages
    #[serde(default)]
    pub include_examples: Option<bool>,
    /// Spreading activation depth (1-3)
    #[serde(default)]
    pub spreading_depth: Option<usize>,
    /// Block type for task-specific spreading activation (CTA, FAQ, HERO, etc.)
    #[serde(default)]
    pub block_type: Option<String>,

    // ─── Knowledge mode params ──────────────────────────────────────────────

    /// Atom type to retrieve (knowledge mode)
    #[serde(default)]
    pub atom_type: Option<AtomType>,
    /// Filter by domain (e.g., "technical", "legal", "marketing")
    #[serde(default)]
    pub domain: Option<String>,
    /// Filter by register (e.g., "formal", "casual")
    #[serde(default)]
    pub register: Option<String>,
    /// Search query to filter atoms
    #[serde(default)]
    pub query: Option<String>,
    /// Include container metadata (TermSet, ExpressionSet, etc.)
    #[serde(default)]
    pub include_containers: Option<bool>,

    // ─── Assemble mode params ───────────────────────────────────────────────

    /// Assembly strategy (assemble mode)
    #[serde(default)]
    pub strategy: Option<AssemblyStrategy>,
    /// Include entity definitions
    #[serde(default)]
    pub include_entities: Option<bool>,
    /// Include locale knowledge (Terms, Expressions)
    #[serde(default)]
    pub include_knowledge: Option<bool>,
    /// Include page/block structure
    #[serde(default)]
    pub include_structure: Option<bool>,
    /// Arc families to follow during assembly
    #[serde(default)]
    pub arc_families: Option<Vec<String>>,
    /// Maximum traversal depth (default: 3)
    #[serde(default)]
    pub max_depth: Option<usize>,

    // ─── Shared limit param ─────────────────────────────────────────────────

    /// Maximum number of results (atoms in knowledge mode, default: 50)
    #[serde(default)]
    pub limit: Option<usize>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// RESULT TYPES — SHARED
// ═══════════════════════════════════════════════════════════════════════════════

/// Evidence packet — compressed content unit (~200 bytes)
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct EvidencePacket {
    /// Source node key
    pub source_key: String,
    /// Source node kind
    pub source_kind: String,
    /// Evidence type (definition, knowledge, structure, etc.)
    pub evidence_type: String,
    /// Graph distance from focus node
    pub distance: usize,
    /// Relevance score (0.0-1.0)
    pub relevance: f64,
    /// Compressed content
    pub content: String,
    /// Token count for this packet
    pub tokens: usize,
}

/// Locale context information
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct LocaleContext {
    /// Locale key (e.g., "fr-FR")
    pub locale_key: String,
    /// Language name
    pub language: String,
    /// Region name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Voice guidelines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    /// Formatting rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<String>,
}

/// Focus node information
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct FocusNode {
    /// Node key
    pub key: String,
    /// Node kind (label)
    pub kind: String,
    /// Display name
    pub name: String,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// RESULT TYPES — PAGE/BLOCK MODE
// ═══════════════════════════════════════════════════════════════════════════════

/// Evidence summary (compressed from full evidence packets)
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct EvidenceSummary {
    /// Source key
    pub source_key: String,
    /// Evidence type
    pub evidence_type: String,
    /// Relevance score
    pub relevance: f64,
    /// Token count
    pub tokens: usize,
}

/// Context anchor for cross-page links
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ContextAnchor {
    /// Referenced page key
    pub page_key: String,
    /// Anchor display text
    pub anchor_text: String,
    /// URL slug
    pub slug: String,
    /// Usage hint for LLM
    pub context_hint: String,
}

/// Token usage breakdown
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct TokenUsage {
    /// Tokens for structure
    pub structure: usize,
    /// Tokens for entities
    pub entities: usize,
    /// Tokens for knowledge atoms
    pub knowledge: usize,
    /// Tokens for locale context
    pub locale: usize,
    /// Total tokens used
    pub total: usize,
    /// Remaining budget
    pub budget_remaining: usize,
}

/// Generation metadata
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct GenerateMetadata {
    /// Number of blocks discovered (page mode)
    pub blocks_discovered: usize,
    /// Number of entities loaded
    pub entities_loaded: usize,
    /// Number of atoms loaded
    pub atoms_loaded: usize,
    /// Execution time
    pub execution_time_ms: u64,
}

/// Context build log for debugging (DX-11)
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ContextBuildLog {
    /// Phase 1: Structure discovery
    pub structure_phase: String,
    /// Phase 2: Entity assembly
    pub entities_phase: String,
    /// Phase 3: Knowledge atoms
    pub atoms_phase: String,
    /// Phase 4: Context anchors
    pub anchors_phase: String,
    /// Phase 5: Token decisions
    pub token_decisions: String,
}

/// Denomination form for entity references (ADR-033)
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct DenominationForm {
    /// Prose form (body content)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Heading form (H1, H2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Abbreviated form (after first mention)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbrev: Option<String>,
    /// URL-safe slug form
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Mixed-case form
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed: Option<String>,
    /// Base form (root)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// RESULT TYPES — KNOWLEDGE MODE
// ═══════════════════════════════════════════════════════════════════════════════

/// A knowledge atom
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Atom {
    /// Atom key
    pub key: String,
    /// Atom type (Term, Expression, Pattern, etc.)
    pub atom_type: String,
    /// Primary value/content
    pub value: String,
    /// Domain (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Register (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register: Option<String>,
    /// Additional properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Container key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_key: Option<String>,
}

/// Container set information
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AtomContainer {
    /// Container key
    pub key: String,
    /// Container type (TermSet, ExpressionSet, etc.)
    pub container_type: String,
    /// Domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Atom count in this container
    pub atom_count: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
// UNIFIED RESULT
// ═══════════════════════════════════════════════════════════════════════════════

/// Result from novanet_context tool (unified across all modes)
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ContextResult {
    /// Mode that was executed
    pub mode: String,

    // ─── Page/Block mode fields ─────────────────────────────────────────────

    /// Generated prompt text (page/block modes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Evidence summary (page/block modes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_summary: Option<Vec<EvidenceSummary>>,
    /// Locale context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_context: Option<LocaleContext>,
    /// Context anchors for cross-page links (page/block modes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_anchors: Option<Vec<ContextAnchor>>,
    /// Denomination forms (page/block modes, ADR-033)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denomination_forms: Option<HashMap<String, DenominationForm>>,
    /// Token usage breakdown (page/block modes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
    /// Generation metadata (page/block modes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GenerateMetadata>,
    /// Context build log for debugging (page/block modes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_build_log: Option<ContextBuildLog>,

    // ─── Knowledge mode fields ──────────────────────────────────────────────

    /// Retrieved atoms (knowledge mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atoms: Option<Vec<Atom>>,
    /// Container information (knowledge mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<AtomContainer>>,
    /// Total atoms matching query (knowledge mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<usize>,

    // ─── Assemble mode fields ───────────────────────────────────────────────

    /// Focus node info (assemble mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<FocusNode>,
    /// Evidence packets (assemble mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<EvidencePacket>>,
    /// Total tokens used (assemble mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<usize>,
    /// Budget remaining (assemble mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_remaining: Option<usize>,
    /// Nodes visited during assembly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes_visited: Option<usize>,
    /// Whether context was truncated to fit budget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,

    // ─── Shared fields ──────────────────────────────────────────────────────

    /// Token estimate for the result
    pub token_estimate: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXECUTE — DISPATCHER
// ═══════════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE MODE — Full page orchestration
// ═══════════════════════════════════════════════════════════════════════════════

async fn execute_page(state: &State, params: ContextParams) -> Result<ContextResult> {
    let start = std::time::Instant::now();
    let focus_key = params.focus_key.as_deref().unwrap_or("unknown");
    let token_budget = params.token_budget.unwrap_or(state.config().default_token_budget);
    let spreading_depth = params.spreading_depth.unwrap_or(2);

    let mut build_log = ContextBuildLog {
        structure_phase: String::new(),
        entities_phase: String::new(),
        atoms_phase: String::new(),
        anchors_phase: String::new(),
        token_decisions: String::new(),
    };

    // Phase 1: Discover page structure via walk
    let structure = get_structure(state, focus_key, spreading_depth).await?;
    let _ = write!(
        build_log.structure_phase,
        "Discovered {} blocks for page '{}'",
        structure.len(),
        focus_key
    );

    // Phases 2-4 in parallel: assemble entities, get atoms, get anchors
    let (entities, atoms_evidence, anchors) = tokio::join!(
        assemble_entities_internal(state, focus_key, &params.locale, &params.block_type),
        assemble_knowledge_internal(state, &params.locale, &params.block_type),
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

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK MODE — Single block generation
// ═══════════════════════════════════════════════════════════════════════════════

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
        assemble_entities_internal(state, focus_key, &params.locale, &params.block_type),
        assemble_knowledge_internal(state, &params.locale, &params.block_type),
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

// ═══════════════════════════════════════════════════════════════════════════════
// KNOWLEDGE MODE — Locale knowledge atoms
// ═══════════════════════════════════════════════════════════════════════════════

/// Configuration for fetching a specific atom type
struct AtomConfig {
    locale_arc: &'static str,
    container_label: &'static str,
    contains_arc: &'static str,
    atom_label: &'static str,
    atom_type_name: &'static str,
    value_property: &'static str,
    filter_field: Option<&'static str>,
    extra_properties: &'static [(&'static str, &'static str)],
    search_fields: &'static [&'static str],
}

const TERM_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_TERMS",
    container_label: "TermSet",
    contains_arc: "CONTAINS_TERM",
    atom_label: "Term",
    atom_type_name: "Term",
    value_property: "value",
    filter_field: Some("domain"),
    extra_properties: &[("definition", "definition")],
    search_fields: &["key", "value"],
};

const EXPRESSION_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_EXPRESSIONS",
    container_label: "ExpressionSet",
    contains_arc: "CONTAINS_EXPRESSION",
    atom_label: "Expression",
    atom_type_name: "Expression",
    value_property: "value",
    filter_field: Some("register"),
    extra_properties: &[("context", "context")],
    search_fields: &["key", "value"],
};

const PATTERN_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_PATTERNS",
    container_label: "PatternSet",
    contains_arc: "CONTAINS_PATTERN",
    atom_label: "Pattern",
    atom_type_name: "Pattern",
    value_property: "template",
    filter_field: None,
    extra_properties: &[("purpose", "purpose")],
    search_fields: &["key", "template"],
};

const CULTURE_REF_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_CULTURE",
    container_label: "CultureSet",
    contains_arc: "CONTAINS_CULTURE_REF",
    atom_label: "CultureRef",
    atom_type_name: "CultureRef",
    value_property: "reference",
    filter_field: None,
    extra_properties: &[
        ("context", "context"),
        ("appropriateness", "appropriateness"),
    ],
    search_fields: &["key", "reference"],
};

const TABOO_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_TABOOS",
    container_label: "TabooSet",
    contains_arc: "CONTAINS_TABOO",
    atom_label: "Taboo",
    atom_type_name: "Taboo",
    value_property: "description",
    filter_field: None,
    extra_properties: &[("severity", "severity"), ("category", "category")],
    search_fields: &["key", "description"],
};

const AUDIENCE_TRAIT_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_AUDIENCE",
    container_label: "AudienceSet",
    contains_arc: "CONTAINS_AUDIENCE_TRAIT",
    atom_label: "AudienceTrait",
    atom_type_name: "AudienceTrait",
    value_property: "trait",
    filter_field: None,
    extra_properties: &[("demographic", "demographic"), ("behavior", "behavior")],
    search_fields: &["key", "trait"],
};

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
            all_atoms.extend(fetch_atoms(state, &params, &TERM_CONFIG, per_type_limit).await?);
            all_atoms
                .extend(fetch_atoms(state, &params, &EXPRESSION_CONFIG, per_type_limit).await?);
            all_atoms
                .extend(fetch_atoms(state, &params, &PATTERN_CONFIG, per_type_limit).await?);
            all_atoms
                .extend(fetch_atoms(state, &params, &CULTURE_REF_CONFIG, per_type_limit).await?);
            all_atoms
                .extend(fetch_atoms(state, &params, &TABOO_CONFIG, per_type_limit).await?);
            all_atoms.extend(
                fetch_atoms(state, &params, &AUDIENCE_TRAIT_CONFIG, per_type_limit).await?,
            );
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

// ═══════════════════════════════════════════════════════════════════════════════
// ASSEMBLE MODE — Low-level context assembly
// ═══════════════════════════════════════════════════════════════════════════════

async fn execute_assemble(state: &State, params: ContextParams) -> Result<ContextResult> {
    let start = std::time::Instant::now();
    let focus_key = params.focus_key.as_deref().unwrap_or("unknown");
    let token_budget = params.token_budget.unwrap_or(state.config().default_token_budget);
    let max_depth = params.max_depth.unwrap_or(3).min(5);

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
                assemble_entities_for_focus(state, focus_key, &params.locale, max_depth).await
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
                assemble_structure_for_focus(state, focus_key, max_depth).await
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
    let default_modifier = crate::context::TaskModifier {
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

// ═══════════════════════════════════════════════════════════════════════════════
// INTERNAL HELPERS — Page/Block structure discovery
// ═══════════════════════════════════════════════════════════════════════════════

/// Discover page structure by walking ownership arcs
async fn get_structure(
    state: &State,
    focus_key: &str,
    _depth: usize,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (p {key: $key})-[:HAS_BLOCK]->(b)
        OPTIONAL MATCH (b)-[:OF_TYPE]->(bt)
        RETURN b.key AS key, labels(b)[0] AS kind, b.display_name AS name,
               b.content AS content, bt.key AS block_type
        ORDER BY b.position
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let content = row["content"]
                .as_str()
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>();
            let tokens = content.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: row["kind"].as_str().unwrap_or("Block").to_string(),
                evidence_type: "structure".to_string(),
                distance: 1,
                relevance: 1.0 - (i as f64 * 0.05).min(0.5),
                content,
                tokens,
            }
        })
        .collect())
}

/// Assemble entity evidence for page/block modes
async fn assemble_entities_internal(
    state: &State,
    focus_key: &str,
    locale: &str,
    _block_type: &Option<String>,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (n {key: $key})-[:USES_ENTITY|HAS_ENTITY*1..2]->(e:Entity)
        OPTIONAL MATCH (e)-[:HAS_NATIVE]->(en:EntityNative)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        RETURN DISTINCT e.key AS key, e.display_name AS name, e.content AS content,
               en.denomination_forms AS forms
        LIMIT 20
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let content = row["content"]
                .as_str()
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>();
            let tokens = content.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: "Entity".to_string(),
                evidence_type: "entity".to_string(),
                distance: 1,
                relevance: 0.95 - (i as f64 * 0.05).min(0.4),
                content,
                tokens,
            }
        })
        .collect())
}

/// Assemble knowledge atoms as evidence for page/block modes
async fn assemble_knowledge_internal(
    state: &State,
    locale: &str,
    _block_type: &Option<String>,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (l:Locale {key: $locale})-[:HAS_EXPRESSIONS]->(es:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e:Expression)
        RETURN e.key AS key, e.value AS value
        LIMIT 20
    "#;

    let mut params = serde_json::Map::new();
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let value = row["value"].as_str().unwrap_or_default().to_string();
            let tokens = value.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: "Expression".to_string(),
                evidence_type: "knowledge".to_string(),
                distance: 2,
                relevance: 0.7 - (i as f64 * 0.02).min(0.3),
                content: value,
                tokens,
            }
        })
        .collect())
}

/// Get context anchors (cross-page references)
async fn get_context_anchors(
    state: &State,
    focus_key: &str,
    locale: &str,
) -> Result<Vec<ContextAnchor>> {
    let cypher = r#"
        MATCH (b {key: $key})-[:REFERENCES_PAGE]->(p:Page)
        OPTIONAL MATCH (p)-[:HAS_NATIVE]->(pn:PageNative)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        RETURN p.key AS page_key, p.display_name AS name,
               pn.slug AS slug, pn.meta_description AS hint
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .map(|row| ContextAnchor {
            page_key: row["page_key"].as_str().unwrap_or_default().to_string(),
            anchor_text: row["name"].as_str().unwrap_or_default().to_string(),
            slug: row["slug"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            context_hint: row["hint"]
                .as_str()
                .unwrap_or("Link when relevant")
                .to_string(),
        })
        .collect())
}

/// Fetch denomination forms (ADR-033)
async fn fetch_denomination_forms(
    state: &State,
    focus_key: &str,
    locale: &str,
) -> Result<HashMap<String, DenominationForm>> {
    let cypher = r#"
        MATCH (n {key: $key})-[:USES_ENTITY|HAS_ENTITY*1..2]->(e:Entity)
        MATCH (e)-[:HAS_NATIVE]->(en:EntityNative)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        WHERE en.denomination_forms IS NOT NULL
        RETURN e.key AS entity_key, en.denomination_forms AS forms
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    let mut forms_map = HashMap::new();
    for row in rows {
        let entity_key = row["entity_key"].as_str().unwrap_or_default().to_string();
        if let Some(forms_val) = row.get("forms") {
            // Parse denomination_forms from JSON array
            if let Some(forms_arr) = forms_val.as_array() {
                let mut form = DenominationForm {
                    text: None,
                    title: None,
                    abbrev: None,
                    url: None,
                    mixed: None,
                    base: None,
                };
                for f in forms_arr {
                    let form_type = f["type"].as_str().unwrap_or_default();
                    let value = f["value"].as_str().unwrap_or_default().to_string();
                    match form_type {
                        "text" => form.text = Some(value),
                        "title" => form.title = Some(value),
                        "abbrev" => form.abbrev = Some(value),
                        "url" => form.url = Some(value),
                        "mixed" => form.mixed = Some(value),
                        "base" => form.base = Some(value),
                        _ => {}
                    }
                }
                forms_map.insert(entity_key, form);
            }
        }
    }

    Ok(forms_map)
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTERNAL HELPERS — Assemble mode
// ═══════════════════════════════════════════════════════════════════════════════

/// Get focus node info
async fn get_focus_node(state: &State, key: &str) -> Result<FocusNode> {
    let cypher = r#"
        MATCH (n {key: $key})
        RETURN n.key AS key, labels(n)[0] AS kind, n.display_name AS name,
               n.content AS description
        LIMIT 1
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(key));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .first()
        .map(|row| FocusNode {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            kind: row["kind"].as_str().unwrap_or("Unknown").to_string(),
            name: row["name"].as_str().unwrap_or_default().to_string(),
            description: row["description"].as_str().map(|s| s.to_string()),
        })
        .unwrap_or(FocusNode {
            key: key.to_string(),
            kind: "Unknown".to_string(),
            name: key.to_string(),
            description: None,
        }))
}

/// Assemble entity evidence packets (for assemble mode)
async fn assemble_entities_for_focus(
    state: &State,
    focus_key: &str,
    locale: &str,
    _max_depth: usize,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (n {key: $key})-[:USES_ENTITY|HAS_ENTITY|HAS_BLOCK*1..3]->(e:Entity)
        OPTIONAL MATCH (e)-[:HAS_NATIVE]->(en:EntityNative)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        RETURN DISTINCT e.key AS key, e.content AS content, en.content AS native_content
        LIMIT 30
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let content = row["native_content"]
                .as_str()
                .or_else(|| row["content"].as_str())
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>();
            let tokens = content.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: "Entity".to_string(),
                evidence_type: "definition".to_string(),
                distance: 1 + i.min(3),
                relevance: 0.9 - (i as f64 * 0.03).min(0.5),
                content,
                tokens,
            }
        })
        .collect())
}

/// Assemble knowledge evidence packets (for assemble mode)
async fn assemble_knowledge_for_focus(
    state: &State,
    locale: &str,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (l:Locale {key: $locale})-[:HAS_EXPRESSIONS]->(es)-[:CONTAINS_EXPRESSION]->(e:Expression)
        RETURN e.key AS key, e.value AS value
        LIMIT 15
        UNION ALL
        MATCH (l:Locale {key: $locale})-[:HAS_PATTERNS]->(ps)-[:CONTAINS_PATTERN]->(p:Pattern)
        RETURN p.key AS key, p.template AS value
        LIMIT 10
    "#;

    let mut params = serde_json::Map::new();
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let value = row["value"].as_str().unwrap_or_default().to_string();
            let tokens = value.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: "Knowledge".to_string(),
                evidence_type: "knowledge".to_string(),
                distance: 2,
                relevance: 0.6 - (i as f64 * 0.01).min(0.3),
                content: value,
                tokens,
            }
        })
        .collect())
}

/// Assemble structure evidence packets (for assemble mode)
async fn assemble_structure_for_focus(
    state: &State,
    focus_key: &str,
    _max_depth: usize,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (n {key: $key})-[:HAS_BLOCK|HAS_PAGE*1..2]->(child)
        RETURN child.key AS key, labels(child)[0] AS kind, child.content AS content
        LIMIT 20
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let content = row["content"]
                .as_str()
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>();
            let tokens = content.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: row["kind"].as_str().unwrap_or("Unknown").to_string(),
                evidence_type: "structure".to_string(),
                distance: 1,
                relevance: 0.8 - (i as f64 * 0.04).min(0.4),
                content,
                tokens,
            }
        })
        .collect())
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTERNAL HELPERS — Knowledge mode atom fetching
// ═══════════════════════════════════════════════════════════════════════════════

/// Generic function to fetch atoms of any type using configuration
async fn fetch_atoms(
    state: &State,
    params: &ContextParams,
    config: &AtomConfig,
    limit: usize,
) -> Result<Vec<Atom>> {
    // Build filter for domain/register (if applicable) using parameterized queries
    let (field_filter, field_param_name, field_param_value): (String, Option<&str>, Option<&str>) =
        match config.filter_field {
            Some("domain") => match &params.domain {
                Some(d) => (
                    "AND a.domain = $domain_filter".to_string(),
                    Some("domain_filter"),
                    Some(d.as_str()),
                ),
                None => (String::new(), None, None),
            },
            Some("register") => match &params.register {
                Some(r) => (
                    "AND a.register = $register_filter".to_string(),
                    Some("register_filter"),
                    Some(r.as_str()),
                ),
                None => (String::new(), None, None),
            },
            _ => (String::new(), None, None),
        };

    // Build search query filter
    let query_filter = params
        .query
        .as_ref()
        .map(|_| {
            let fields: Vec<String> = config
                .search_fields
                .iter()
                .map(|f| format!("toLower(a.{}) CONTAINS toLower($query)", f))
                .collect();
            format!("AND ({})", fields.join(" OR "))
        })
        .unwrap_or_default();

    // Build extra properties return clause
    let extra_props: Vec<String> = config
        .extra_properties
        .iter()
        .map(|(prop, alias)| format!("a.{} AS {}", prop, alias))
        .collect();
    let extra_props_clause = if extra_props.is_empty() {
        String::new()
    } else {
        format!(", {}", extra_props.join(", "))
    };

    let cypher = format!(
        r#"
        MATCH (l:Locale {{key: $locale}})-[:{locale_arc}]->(c:{container})-[:{contains_arc}]->(a:{atom})
        WHERE true {field_filter} {query_filter}
        RETURN a.key AS key, a.{value_prop} AS value, c.key AS container_key{extra_props}
        LIMIT {limit}
        "#,
        locale_arc = config.locale_arc,
        container = config.container_label,
        contains_arc = config.contains_arc,
        atom = config.atom_label,
        value_prop = config.value_property,
        field_filter = field_filter,
        query_filter = query_filter,
        extra_props = extra_props_clause,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("locale".to_string(), serde_json::json!(params.locale));
    if let Some(q) = &params.query {
        query_params.insert("query".to_string(), serde_json::json!(q));
    }
    if let (Some(param_name), Some(param_value)) = (field_param_name, field_param_value) {
        query_params.insert(param_name.to_string(), serde_json::json!(param_value));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let properties = if config.extra_properties.is_empty() {
                None
            } else {
                let mut props = serde_json::Map::new();
                for (_, alias) in config.extra_properties {
                    if let Some(val) = row.get(*alias) {
                        if !val.is_null() {
                            props.insert(alias.to_string(), val.clone());
                        }
                    }
                }
                if props.is_empty() {
                    None
                } else {
                    Some(serde_json::Value::Object(props))
                }
            };

            Atom {
                key: row["key"].as_str().unwrap_or_default().to_string(),
                atom_type: config.atom_type_name.to_string(),
                value: row["value"].as_str().unwrap_or_default().to_string(),
                domain: if config.filter_field == Some("domain") {
                    row.get("domain")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                },
                register: if config.filter_field == Some("register") {
                    row.get("register")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                },
                properties,
                container_key: row["container_key"].as_str().map(|s| s.to_string()),
            }
        })
        .collect())
}

/// Get container information for a locale
async fn get_containers(state: &State, locale: &str) -> Result<Vec<AtomContainer>> {
    let cypher = r#"
        MATCH (l:Locale {key: $locale})
        OPTIONAL MATCH (l)-[:HAS_TERMS]->(ts:TermSet)
        OPTIONAL MATCH (ts)-[:CONTAINS_TERM]->(t:Term)
        WITH l, ts, count(t) AS term_count
        OPTIONAL MATCH (l)-[:HAS_EXPRESSIONS]->(es:ExpressionSet)
        OPTIONAL MATCH (es)-[:CONTAINS_EXPRESSION]->(e:Expression)
        WITH l, ts, term_count, es, count(e) AS expr_count
        OPTIONAL MATCH (l)-[:HAS_PATTERNS]->(ps:PatternSet)
        OPTIONAL MATCH (ps)-[:CONTAINS_PATTERN]->(p:Pattern)
        WITH l, ts, term_count, es, expr_count, ps, count(p) AS pattern_count
        OPTIONAL MATCH (l)-[:HAS_CULTURE]->(cs:CultureSet)
        OPTIONAL MATCH (cs)-[:CONTAINS_CULTURE_REF]->(c:CultureRef)
        WITH l, ts, term_count, es, expr_count, ps, pattern_count, cs, count(c) AS culture_count
        OPTIONAL MATCH (l)-[:HAS_TABOOS]->(tbs:TabooSet)
        OPTIONAL MATCH (tbs)-[:CONTAINS_TABOO]->(tb:Taboo)
        WITH l, ts, term_count, es, expr_count, ps, pattern_count, cs, culture_count, tbs, count(tb) AS taboo_count
        OPTIONAL MATCH (l)-[:HAS_AUDIENCE]->(aus:AudienceSet)
        OPTIONAL MATCH (aus)-[:CONTAINS_AUDIENCE_TRAIT]->(au:AudienceTrait)
        RETURN
            ts.key AS term_set_key, ts.domain AS term_set_domain, term_count,
            es.key AS expr_set_key, es.register AS expr_set_register, expr_count,
            ps.key AS pattern_set_key, pattern_count,
            cs.key AS culture_set_key, culture_count,
            tbs.key AS taboo_set_key, taboo_count,
            aus.key AS audience_set_key, count(au) AS audience_count
    "#;

    let mut params = serde_json::Map::new();
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    let mut containers = Vec::new();

    if let Some(row) = rows.first() {
        if let Some(key) = row["term_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "TermSet".to_string(),
                domain: row["term_set_domain"].as_str().map(|s| s.to_string()),
                atom_count: row["term_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["expr_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "ExpressionSet".to_string(),
                domain: row["expr_set_register"].as_str().map(|s| s.to_string()),
                atom_count: row["expr_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["pattern_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "PatternSet".to_string(),
                domain: None,
                atom_count: row["pattern_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["culture_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "CultureSet".to_string(),
                domain: None,
                atom_count: row["culture_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["taboo_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "TabooSet".to_string(),
                domain: None,
                atom_count: row["taboo_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["audience_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "AudienceSet".to_string(),
                domain: None,
                atom_count: row["audience_count"].as_u64().unwrap_or(0) as usize,
            });
        }
    }

    Ok(containers)
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTERNAL HELPERS — Shared
// ═══════════════════════════════════════════════════════════════════════════════

/// Get locale context information
async fn get_locale_context(state: &State, locale: &str) -> Result<LocaleContext> {
    let cypher = r#"
        MATCH (l:Locale {key: $locale})
        RETURN l.key AS key, l.language AS language, l.region AS region,
               l.voice AS voice, l.formatting AS formatting
    "#;

    let mut params = serde_json::Map::new();
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .first()
        .map(|row| LocaleContext {
            locale_key: row["key"].as_str().unwrap_or(locale).to_string(),
            language: row["language"].as_str().unwrap_or("Unknown").to_string(),
            region: row["region"].as_str().map(|s| s.to_string()),
            voice: row["voice"].as_str().map(|s| s.to_string()),
            formatting: row["formatting"].as_str().map(|s| s.to_string()),
        })
        .unwrap_or(LocaleContext {
            locale_key: locale.to_string(),
            language: "Unknown".to_string(),
            region: None,
            voice: None,
            formatting: None,
        }))
}

/// Estimate token count for locale context
fn estimate_locale_tokens(ctx: &LocaleContext) -> usize {
    let mut total = ctx.locale_key.len() + ctx.language.len();
    if let Some(ref r) = ctx.region {
        total += r.len();
    }
    if let Some(ref v) = ctx.voice {
        total += v.len();
    }
    if let Some(ref f) = ctx.formatting {
        total += f.len();
    }
    total.div_ceil(4)
}

/// Build generation prompt from assembled context
#[allow(clippy::too_many_arguments)]
fn build_prompt(
    focus_key: &str,
    locale: &str,
    mode: &str,
    locale_ctx: &LocaleContext,
    entities: &[EvidencePacket],
    atoms: &[EvidencePacket],
    anchors: &[ContextAnchor],
    denomination_forms: &HashMap<String, DenominationForm>,
) -> String {
    let mut prompt = String::with_capacity(4096);

    let _ = writeln!(prompt, "# Generation Context for {} ({})", focus_key, locale);
    let _ = writeln!(prompt, "## Mode: {}\n", mode);

    // Locale
    let _ = writeln!(prompt, "## Locale: {}", locale_ctx.language);
    if let Some(ref region) = locale_ctx.region {
        let _ = writeln!(prompt, "Region: {}", region);
    }
    if let Some(ref voice) = locale_ctx.voice {
        let _ = writeln!(prompt, "Voice: {}", voice);
    }
    let _ = writeln!(prompt);

    // Denomination forms (ADR-033)
    if !denomination_forms.is_empty() {
        let _ = writeln!(prompt, "## Entity Reference Forms (ADR-033)");
        let _ = writeln!(prompt, "Use ONLY these forms when referencing entities:\n");
        for (entity_key, form) in denomination_forms {
            let _ = writeln!(prompt, "### {}", entity_key);
            if let Some(ref text) = form.text {
                let _ = writeln!(prompt, "- text (body): {}", text);
            }
            if let Some(ref title) = form.title {
                let _ = writeln!(prompt, "- title (headings): {}", title);
            }
            if let Some(ref abbrev) = form.abbrev {
                let _ = writeln!(prompt, "- abbrev (after first): {}", abbrev);
            }
            if let Some(ref url) = form.url {
                let _ = writeln!(prompt, "- url (slugs): {}", url);
            }
            let _ = writeln!(prompt);
        }
    }

    // Entities
    if !entities.is_empty() {
        let _ = writeln!(prompt, "## Entities ({} loaded)\n", entities.len());
        for e in entities {
            let _ = writeln!(prompt, "- **{}**: {}", e.source_key, e.content);
        }
        let _ = writeln!(prompt);
    }

    // Knowledge atoms
    if !atoms.is_empty() {
        let _ = writeln!(prompt, "## Knowledge Atoms ({} loaded)\n", atoms.len());
        for a in atoms {
            let _ = writeln!(prompt, "- {}: {}", a.source_key, a.content);
        }
        let _ = writeln!(prompt);
    }

    // Anchors
    if !anchors.is_empty() {
        let _ = writeln!(prompt, "## Cross-Page Links\n");
        let _ = writeln!(
            prompt,
            "Use {{{{anchor:key|text}}}} syntax for internal links:\n"
        );
        for a in anchors {
            let _ = writeln!(
                prompt,
                "- {{{{anchor:{}|{}}}}} → {} ({})",
                a.page_key, a.anchor_text, a.slug, a.context_hint
            );
        }
    }

    prompt
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ─── ContextMode tests ──────────────────────────────────────────────────

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

    // ─── AssemblyStrategy tests ─────────────────────────────────────────────

    #[test]
    fn test_assembly_strategy_default_is_breadth() {
        let strategy: AssemblyStrategy = Default::default();
        assert!(matches!(strategy, AssemblyStrategy::Breadth));
    }

    // ─── AtomType tests ─────────────────────────────────────────────────────

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

    // ─── ContextParams tests ────────────────────────────────────────────────

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

    // ─── DenominationForm tests ─────────────────────────────────────────────

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

    // ─── EvidencePacket tests ───────────────────────────────────────────────

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

    // ─── Atom tests ─────────────────────────────────────────────────────────

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

    // ─── Prompt building tests ──────────────────────────────────────────────

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

    // ─── AtomConfig tests ───────────────────────────────────────────────────

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

    // ─── ContextResult tests ────────────────────────────────────────────────

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
