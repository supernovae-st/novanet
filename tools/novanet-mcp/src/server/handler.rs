//! MCP Server Handler
//!
//! Implements rmcp::ServerHandler for NovaNet MCP tools using macro-based routing.
//! Phase 3: Adds novanet_generate tool and 6 MCP prompts.

use crate::prompts::{self, PromptDefinition, PromptMessage as InternalPromptMessage};
use crate::server::State;
use crate::tools::{
    AssembleParams, AtomsParams, AuditParams, BatchParams, CacheInvalidateParams, CacheStatsParams,
    CheckParams, DescribeParams, GenerateParams, IntrospectParams, QueryParams, SearchParams,
    TraverseParams, WriteParams,
};
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content, ErrorCode, GetPromptRequestParams, GetPromptResult, ListPromptsResult,
    PaginatedRequestParams, Prompt, PromptArgument, PromptMessage, PromptMessageRole,
    ServerCapabilities, ServerInfo,
};
use rmcp::service::{RequestContext, RoleServer};
use rmcp::{ErrorData as McpError, ServerHandler, tool, tool_handler, tool_router};
use std::borrow::Cow;

/// NovaNet MCP Handler with tool routing
#[derive(Clone)]
pub struct NovaNetHandler {
    state: State,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl NovaNetHandler {
    /// Create a new handler with the given state
    pub fn new(state: State) -> Self {
        Self {
            state,
            tool_router: Self::tool_router(),
        }
    }

    /// Execute a read-only Cypher query against the NovaNet knowledge graph.
    ///
    /// Returns rows as JSON array with token estimate and execution metadata.
    /// All queries are validated for read-only operations (no CREATE, DELETE, MERGE, SET).
    #[tool(
        name = "novanet_query",
        description = "⚠️ DEBUG/ANALYTICS ONLY - Execute raw Cypher query. Returns rows as JSON. 🚫 DO NOT USE for: finding nodes (use novanet_search), exploring relationships (use novanet_traverse), schema info (use novanet_introspect), content generation (use novanet_generate). ✅ USE ONLY for: custom aggregations, complex analytics, debugging. Most workflows should NEVER need this tool."
    )]
    async fn novanet_query(
        &self,
        params: Parameters<QueryParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::query::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Get self-description of the NovaNet knowledge graph.
    ///
    /// Supports describing: schema overview, specific entities, entity categories,
    /// available relations (ArcKinds), locales, and graph statistics.
    #[tool(
        name = "novanet_describe",
        description = "🚀 START HERE - Bootstrap your understanding of NovaNet. Targets: 'schema' (overview of realms/layers/classes), 'entity' (specific entity details), 'category' (entity category members), 'relations' (arc families), 'locales' (available locales), 'stats' (graph statistics). Use this first to understand what's in the graph before using other tools."
    )]
    async fn novanet_describe(
        &self,
        params: Parameters<DescribeParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::describe::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Search the NovaNet knowledge graph using fulltext and property search.
    ///
    /// Supports fulltext, property-based, and hybrid search modes.
    /// Filter by node kinds, realm, and layer.
    #[tool(
        name = "novanet_search",
        description = "🔍 FIND NODES - Search the knowledge graph by text or properties. Modes: 'fulltext' (Neo4j fulltext index), 'property' (exact/partial match), 'hybrid' (both). Filter by: kinds (Entity, Page, Block...), realm (shared/org), layer. Returns matches with relevance scores. Use this to find nodes, then novanet_traverse to explore their relationships."
    )]
    async fn novanet_search(
        &self,
        params: Parameters<SearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::search::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Traverse the NovaNet knowledge graph from a starting node.
    ///
    /// Configurable depth, direction, and arc filtering.
    /// Implements RLM-on-KG hop-by-hop pattern.
    #[tool(
        name = "novanet_traverse",
        description = "🧭 EXPLORE RELATIONSHIPS - Traverse graph from a starting node. Configure: max_depth (1-5), direction (outgoing/incoming/both), arc_families (ownership/semantic/localization/generation/mining), target_kinds. Returns connected nodes with paths. Use after novanet_search to explore an entity's relationships."
    )]
    async fn novanet_traverse(
        &self,
        params: Parameters<TraverseParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::traverse::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Assemble context for LLM generation from the knowledge graph.
    ///
    /// Gathers entity definitions, locale knowledge, and structural context
    /// with token budget management and evidence packet compression.
    #[tool(
        name = "novanet_assemble",
        description = "🔧 ADVANCED - Low-level context assembly with token budget management. Gathers entities, locale knowledge, and structure. Strategies: breadth (default), depth, relevance, custom. ⚠️ Most users should use novanet_generate instead, which orchestrates assemble + atoms automatically. Use assemble only for custom context pipelines."
    )]
    async fn novanet_assemble(
        &self,
        params: Parameters<AssembleParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::assemble::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Retrieve knowledge atoms for a specific locale.
    ///
    /// Returns Terms, Expressions, Patterns, CultureRefs, Taboos, and AudienceTraits.
    /// Enables selective LLM context loading.
    #[tool(
        name = "novanet_atoms",
        description = "🔤 LOCALE KNOWLEDGE - Retrieve knowledge atoms for a specific locale. Types: term (technical vocabulary), expression (idioms), pattern (templates), cultureref (cultural references), taboo (things to avoid), audiencetrait (audience characteristics), or 'all'. Filter by domain/register. Use for locale-specific content generation or exploring cultural nuances."
    )]
    async fn novanet_atoms(
        &self,
        params: Parameters<AtomsParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::atoms::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Assemble complete generation context for block or page content.
    ///
    /// Orchestrates traverse, assemble, and atoms tools for AI agents.
    /// Implements full RLM-on-KG pipeline with context anchors.
    #[tool(
        name = "novanet_generate",
        description = "⭐ PRIMARY CONTENT TOOL - Assemble complete generation context for block or page content. Modes: 'block' (single block with entities + knowledge) or 'page' (full page with all blocks + cross-references). Automatically orchestrates traverse/assemble/atoms. Returns: prompt, evidence_summary, locale_context, context_anchors, denomination_forms. This is THE tool for Nika content generation workflows."
    )]
    async fn novanet_generate(
        &self,
        params: Parameters<GenerateParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::generate::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Introspect the NovaNet schema: query NodeClasses and ArcClasses.
    ///
    /// Enables agents to understand the knowledge graph schema for dynamic
    /// query generation and task decomposition.
    /// MVP 8 Phase 3: 8th MCP tool for schema introspection.
    #[tool(
        name = "novanet_introspect",
        description = "📚 SCHEMA INFO - Query NodeClasses and ArcClasses with their relationships. Targets: 'classes' (list all, filter by realm/layer), 'class' (specific class with arcs), 'arcs' (list all, filter by family), 'arc' (specific arc). 💡 REQUIRED before novanet_write: use introspect(target='class', name='EntityNative', include_arcs=true) to discover required_properties and valid arc connections."
    )]
    async fn novanet_introspect(
        &self,
        params: Parameters<IntrospectParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::introspect::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Execute multiple NovaNet tools in a single request.
    ///
    /// Supports sequential and parallel execution modes with fail-fast behavior.
    /// Task A1: 9th MCP tool for bulk operations.
    #[tool(
        name = "novanet_batch",
        description = "📦 BULK OPERATIONS - Execute multiple MCP tools in a single request. Set parallel=true for concurrent execution (faster), fail_fast=true to stop on first error. Use cases: batch context assembly, parallel search, bulk schema introspection. Each operation has an id for result mapping. Returns aggregated results with timing."
    )]
    async fn novanet_batch(
        &self,
        params: Parameters<BatchParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::batch::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Get cache statistics including hit rate, entry count, and memory usage.
    ///
    /// Task A3: 10th MCP tool for cache monitoring.
    #[tool(
        name = "novanet_cache_stats",
        description = "🔧 OPS/DEBUG - Get cache statistics: entries count, hit rate percentage, hits/misses, memory usage, TTL settings. Use for monitoring cache performance and debugging. Low hit rate may indicate need for cache tuning."
    )]
    async fn novanet_cache_stats(
        &self,
        params: Parameters<CacheStatsParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::cache_stats::get_stats(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Invalidate cache entries. Use pattern for selective invalidation or all=true for full clear.
    ///
    /// Task A3: 11th MCP tool for cache management.
    #[tool(
        name = "novanet_cache_invalidate",
        description = "🔧 OPS/DEBUG - Invalidate cache entries. Use all=true to clear entire cache (after schema changes), or pattern for selective invalidation. ⚠️ Note: Pattern-based invalidation is NOT YET IMPLEMENTED - use all=true for now. Automatically called after novanet_write operations."
    )]
    async fn novanet_cache_invalidate(
        &self,
        params: Parameters<CacheInvalidateParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::cache_stats::invalidate(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Write data to Neo4j with schema validation.
    ///
    /// Operations: upsert_node, create_arc, update_props.
    /// Validates against schema, checks trait permissions, uses MERGE for idempotency.
    /// Task A4: 12th MCP tool for intelligent data writes.
    #[tool(
        name = "novanet_write",
        description = "Write data to NovaNet with schema validation. ⚠️ ALWAYS call novanet_check FIRST to validate and discover required properties. Operations: upsert_node, create_arc, update_props. Enforces trait permissions (only authored/imported/generated/retrieved traits writable). Auto-creates FOR_LOCALE and HAS_NATIVE arcs for *Native classes."
    )]
    async fn novanet_write(
        &self,
        params: Parameters<WriteParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::write::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Validate a write operation without executing it.
    ///
    /// Pre-write validation with Cypher preview and ontology-driven suggestions.
    /// Returns issues, schema context, and actionable hints.
    /// v0.17.0: 13th MCP tool for pre-write validation.
    #[tool(
        name = "novanet_check",
        description = "⚠️ REQUIRED: Call this BEFORE novanet_write to validate operations. Returns: valid (bool), errors[], warnings[], suggestions[], cypher_preview, and schema_context with required_properties. Use the same parameters as novanet_write. If valid=false, fix issues before calling novanet_write."
    )]
    async fn novanet_check(
        &self,
        params: Parameters<CheckParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::checker::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Audit the knowledge graph for quality issues.
    ///
    /// Post-write quality audit with CSR (Constraint Satisfaction Rate) metrics.
    /// Checks coverage, orphans, integrity, and freshness.
    /// v0.17.0: 14th MCP tool for quality audit.
    #[tool(
        name = "novanet_audit",
        description = "📊 QUALITY AUDIT - Post-write quality checks with CSR (Constraint Satisfaction Rate) metrics. Targets: coverage (missing *Native for locales), orphans (missing FOR_LOCALE/HAS_NATIVE arcs), integrity (broken refs), freshness (stale >30 days), or 'all'. Returns issues by severity + recommendations. CSR ≥0.95 = healthy, 0.85-0.95 = warning, <0.85 = critical."
    )]
    async fn novanet_audit(
        &self,
        params: Parameters<AuditParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::auditor::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

/// Implement ServerHandler for NovaNetHandler
#[tool_handler]
impl ServerHandler for NovaNetHandler {
    fn get_info(&self) -> ServerInfo {
        // rmcp 1.x: Use builder pattern instead of struct literals
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
        )
        .with_instructions(
            "NovaNet MCP Server v0.17.0 - Knowledge Graph for AI Agents. \
             14 tools available. TOOL SELECTION: \
             🔍 novanet_search (find nodes) → 🧭 novanet_traverse (explore relationships) → \
             ⭐ novanet_generate (content context). \
             For writes: 📚 novanet_introspect (schema) → ✅ novanet_check (validate) → ✍️ novanet_write. \
             📊 novanet_audit for quality. ⚠️ novanet_query is LAST RESORT for custom analytics only. \
             6 prompts available.",
        )
    }

    /// List all available prompts
    #[allow(clippy::manual_async_fn)] // Required by ServerHandler trait signature
    fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListPromptsResult, McpError>> + Send + '_ {
        async move {
            let prompt_defs = prompts::list_prompts();
            let prompts: Vec<Prompt> = prompt_defs
                .into_iter()
                .map(convert_prompt_definition)
                .collect();

            Ok(ListPromptsResult::with_all_items(prompts))
        }
    }

    /// Get a specific prompt by name
    #[allow(clippy::manual_async_fn)] // Required by ServerHandler trait signature
    fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<GetPromptResult, McpError>> + Send + '_ {
        async move {
            let args = request.arguments.unwrap_or_default();

            // rmcp 1.x: Use builder pattern instead of struct literals
            let rendered = prompts::render_prompt(&request.name, &args)
                .ok_or_else(|| McpError::resource_not_found(format!("Prompt not found: {}", request.name), None))?;

            // rmcp 1.x: Use builder pattern instead of struct literals
            Ok(GetPromptResult::new(
                rendered
                    .messages
                    .into_iter()
                    .map(convert_prompt_message)
                    .collect(),
            )
            .with_description(rendered.description))
        }
    }
}

/// Convert internal PromptDefinition to MCP Prompt
/// rmcp 1.x: Use builder pattern instead of struct literals
fn convert_prompt_definition(def: PromptDefinition) -> Prompt {
    Prompt::new(
        def.name,
        Some(def.description),
        Some(
            def.arguments
                .into_iter()
                .map(|arg| {
                    PromptArgument::new(arg.name)
                        .with_description(arg.description)
                        .with_required(arg.required)
                })
                .collect(),
        ),
    )
}

/// Convert internal PromptMessage to MCP PromptMessage
fn convert_prompt_message(msg: InternalPromptMessage) -> PromptMessage {
    let role = match msg.role.as_str() {
        "assistant" => PromptMessageRole::Assistant,
        _ => PromptMessageRole::User, // MCP doesn't have system role, use user
    };

    PromptMessage::new_text(role, msg.content)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ══════════════════════════════════════════════════════════════
    // HELPER FUNCTION TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_convert_prompt_definition_maps_all_fields() {
        let def = crate::prompts::PromptDefinition {
            name: "test_prompt".to_string(),
            description: "A test prompt".to_string(),
            arguments: vec![
                crate::prompts::PromptArgument {
                    name: "arg1".to_string(),
                    description: "First argument".to_string(),
                    required: true,
                },
                crate::prompts::PromptArgument {
                    name: "arg2".to_string(),
                    description: "Second argument".to_string(),
                    required: false,
                },
            ],
        };

        let prompt = convert_prompt_definition(def);

        assert_eq!(prompt.name, "test_prompt");
        assert_eq!(prompt.description, Some("A test prompt".to_string()));
        assert!(prompt.title.is_none());
        assert!(prompt.icons.is_none());
        assert!(prompt.meta.is_none());

        let args = prompt.arguments.expect("Should have arguments");
        assert_eq!(args.len(), 2);

        assert_eq!(args[0].name, "arg1");
        assert_eq!(args[0].description, Some("First argument".to_string()));
        assert_eq!(args[0].required, Some(true));

        assert_eq!(args[1].name, "arg2");
        assert_eq!(args[1].description, Some("Second argument".to_string()));
        assert_eq!(args[1].required, Some(false));
    }

    #[test]
    fn test_convert_prompt_definition_empty_arguments() {
        let def = crate::prompts::PromptDefinition {
            name: "no_args".to_string(),
            description: "No arguments".to_string(),
            arguments: vec![],
        };

        let prompt = convert_prompt_definition(def);

        assert_eq!(prompt.name, "no_args");
        let args = prompt.arguments.expect("Should have empty vec");
        assert!(args.is_empty());
    }

    #[test]
    fn test_convert_prompt_message_user_role() {
        let msg = InternalPromptMessage {
            role: "user".to_string(),
            content: "Hello, world!".to_string(),
        };

        let prompt_msg = convert_prompt_message(msg);

        // PromptMessage stores content in Content enum, check via debug
        assert!(format!("{:?}", prompt_msg).contains("User"));
    }

    #[test]
    fn test_convert_prompt_message_assistant_role() {
        let msg = InternalPromptMessage {
            role: "assistant".to_string(),
            content: "I am here to help.".to_string(),
        };

        let prompt_msg = convert_prompt_message(msg);

        assert!(format!("{:?}", prompt_msg).contains("Assistant"));
    }

    #[test]
    fn test_convert_prompt_message_system_maps_to_user() {
        // MCP doesn't have system role, should map to user
        let msg = InternalPromptMessage {
            role: "system".to_string(),
            content: "System instructions".to_string(),
        };

        let prompt_msg = convert_prompt_message(msg);

        // System should be converted to User (per comment in code)
        assert!(format!("{:?}", prompt_msg).contains("User"));
    }

    #[test]
    fn test_convert_prompt_message_unknown_role_maps_to_user() {
        let msg = InternalPromptMessage {
            role: "unknown".to_string(),
            content: "Some content".to_string(),
        };

        let prompt_msg = convert_prompt_message(msg);

        // Unknown roles default to User
        assert!(format!("{:?}", prompt_msg).contains("User"));
    }

    // ══════════════════════════════════════════════════════════════
    // PROMPT LIST TESTS (via prompts module)
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_list_prompts_returns_6_prompts() {
        let prompts = prompts::list_prompts();
        assert_eq!(prompts.len(), 6, "Should return exactly 6 prompts");
    }

    #[test]
    fn test_list_prompts_has_expected_names() {
        let prompts = prompts::list_prompts();
        let names: Vec<&str> = prompts.iter().map(|p| p.name.as_str()).collect();

        assert!(names.contains(&"cypher_query"), "Missing cypher_query");
        assert!(names.contains(&"cypher_explain"), "Missing cypher_explain");
        assert!(
            names.contains(&"block_generation"),
            "Missing block_generation"
        );
        assert!(
            names.contains(&"page_generation"),
            "Missing page_generation"
        );
        assert!(
            names.contains(&"entity_analysis"),
            "Missing entity_analysis"
        );
        assert!(
            names.contains(&"locale_briefing"),
            "Missing locale_briefing"
        );
    }

    #[test]
    fn test_all_prompts_have_descriptions() {
        let prompts = prompts::list_prompts();

        for prompt in prompts {
            assert!(
                !prompt.description.is_empty(),
                "Prompt {} should have a description",
                prompt.name
            );
        }
    }

    // ══════════════════════════════════════════════════════════════
    // GET PROMPT TESTS (via prompts module)
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_get_prompt_cypher_query_renders_with_intent() {
        let mut args = serde_json::Map::new();
        args.insert(
            "intent".to_string(),
            serde_json::Value::String("Find all entities".to_string()),
        );

        let result = prompts::render_prompt("cypher_query", &args);

        assert!(result.is_some(), "Should render cypher_query");
        let rendered = result.unwrap();
        assert!(!rendered.description.is_empty());
        assert!(!rendered.messages.is_empty());
    }

    #[test]
    fn test_get_prompt_not_found_returns_none() {
        let args = serde_json::Map::new();
        let result = prompts::render_prompt("nonexistent_prompt", &args);
        assert!(result.is_none(), "Should return None for unknown prompt");
    }

    #[test]
    fn test_get_prompt_locale_briefing_renders() {
        let mut args = serde_json::Map::new();
        args.insert(
            "locale_key".to_string(),
            serde_json::Value::String("fr-FR".to_string()),
        );

        let result = prompts::render_prompt("locale_briefing", &args);

        assert!(result.is_some(), "Should render locale_briefing");
    }

    // ══════════════════════════════════════════════════════════════
    // ERROR CODE MAPPING TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_tool_error_code_is_minus_32000() {
        // Verify the error code constant used for tool errors
        // rmcp 1.x: Use new() constructor instead of struct literal
        let error = McpError::new(ErrorCode(-32000), "Tool error", None);
        assert_eq!(error.code, ErrorCode(-32000));
    }

    #[test]
    fn test_serialization_error_code_is_minus_32603() {
        // Verify the error code constant used for serialization errors
        // rmcp 1.x: Use internal_error() instead of struct literal
        let error = McpError::internal_error("Serialization error", None);
        assert_eq!(error.code, ErrorCode::INTERNAL_ERROR);
    }

    #[test]
    fn test_prompt_not_found_error_code_is_minus_32001() {
        // Verify the error code constant used for prompt not found
        // rmcp 1.x: Use resource_not_found() instead of struct literal
        let error = McpError::resource_not_found("Prompt not found", None);
        assert_eq!(error.code, ErrorCode::RESOURCE_NOT_FOUND);
    }
}
