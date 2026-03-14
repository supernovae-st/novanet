//! Type definitions for novanet_context tool
//!
//! Enums, params, result types, and atom configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// ENUMS
// =============================================================================

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

// =============================================================================
// PARAMS
// =============================================================================

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

    // --- Page/Block mode params ------------------------------------------------

    /// Include example content from similar blocks/pages
    #[serde(default)]
    pub include_examples: Option<bool>,
    /// Spreading activation depth (1-3)
    #[serde(default)]
    pub spreading_depth: Option<usize>,
    /// Block type for task-specific spreading activation (CTA, FAQ, HERO, etc.)
    #[serde(default)]
    pub block_type: Option<String>,

    // --- Knowledge mode params -------------------------------------------------

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

    // --- Assemble mode params --------------------------------------------------

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

    // --- Shared limit param ----------------------------------------------------

    /// Maximum number of results (atoms in knowledge mode, default: 50)
    #[serde(default)]
    pub limit: Option<usize>,
}

// =============================================================================
// RESULT TYPES — SHARED
// =============================================================================

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

// =============================================================================
// RESULT TYPES — PAGE/BLOCK MODE
// =============================================================================

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

// =============================================================================
// RESULT TYPES — KNOWLEDGE MODE
// =============================================================================

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

// =============================================================================
// UNIFIED RESULT
// =============================================================================

/// Result from novanet_context tool (unified across all modes)
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ContextResult {
    /// Mode that was executed
    pub mode: String,

    // --- Page/Block mode fields ------------------------------------------------

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

    // --- Knowledge mode fields -------------------------------------------------

    /// Retrieved atoms (knowledge mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atoms: Option<Vec<Atom>>,
    /// Container information (knowledge mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<AtomContainer>>,
    /// Total atoms matching query (knowledge mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<usize>,

    // --- Assemble mode fields --------------------------------------------------

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

    // --- Shared fields ---------------------------------------------------------

    /// Token estimate for the result
    pub token_estimate: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// =============================================================================
// ATOM CONFIG — Knowledge mode query configuration
// =============================================================================

/// Configuration for fetching a specific atom type
pub(crate) struct AtomConfig {
    pub locale_arc: &'static str,
    pub container_label: &'static str,
    pub contains_arc: &'static str,
    pub atom_label: &'static str,
    pub atom_type_name: &'static str,
    pub value_property: &'static str,
    pub filter_field: Option<&'static str>,
    pub extra_properties: &'static [(&'static str, &'static str)],
    pub search_fields: &'static [&'static str],
}

pub(crate) const TERM_CONFIG: AtomConfig = AtomConfig {
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

pub(crate) const EXPRESSION_CONFIG: AtomConfig = AtomConfig {
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

pub(crate) const PATTERN_CONFIG: AtomConfig = AtomConfig {
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

pub(crate) const CULTURE_REF_CONFIG: AtomConfig = AtomConfig {
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

pub(crate) const TABOO_CONFIG: AtomConfig = AtomConfig {
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

pub(crate) const AUDIENCE_TRAIT_CONFIG: AtomConfig = AtomConfig {
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
