//! MCP Server Handler (v0.20.0)
//!
//! 8 tools after The Great Cleanup (was 14):
//!   novanet_query, novanet_describe, novanet_search, novanet_introspect,
//!   novanet_context, novanet_write, novanet_audit, novanet_batch
//!
//! Removed tools:
//!   - novanet_traverse → merged into novanet_search (mode=walk)
//!   - novanet_assemble, novanet_atoms, novanet_generate → merged into novanet_context
//!   - novanet_cache_stats, novanet_cache_invalidate → deleted (D7)
//!   - novanet_check → absorbed into novanet_write (dry_run param, D6)

use crate::prompts::{self, PromptDefinition, PromptMessage as InternalPromptMessage};
use crate::server::State;
use crate::tools::{
    AuditParams, BatchParams, ContextParams, DescribeParams, IntrospectParams, QueryParams,
    SearchParams, WriteParams,
};
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content, GetPromptRequestParams, GetPromptResult, ListPromptsResult,
    PaginatedRequestParams, Prompt, PromptArgument, PromptMessage, PromptMessageRole,
    ServerCapabilities, ServerInfo,
};
use rmcp::service::{RequestContext, RoleServer};
use rmcp::{ErrorData as McpError, ServerHandler, tool, tool_handler, tool_router};

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
        description = "⚠️ DEBUG/ANALYTICS ONLY - Execute raw Cypher query. Returns rows as JSON. 🚫 DO NOT USE for: finding nodes (use novanet_search), exploring relationships (use novanet_search mode=walk), schema info (use novanet_introspect), content generation (use novanet_context). ✅ USE ONLY for: custom aggregations, complex analytics, debugging. Most workflows should NEVER need this tool."
    )]
    async fn novanet_query(
        &self,
        params: Parameters<QueryParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::query::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Get self-description of the NovaNet knowledge graph.
    ///
    /// Supports describing: schema overview, specific entities, entity categories,
    /// available relations (ArcClasses), locales, and graph statistics.
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

        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Search the NovaNet knowledge graph.
    ///
    /// 5 modes: fulltext, property, hybrid, walk (graph traversal), triggers.
    /// Walk mode replaces the former novanet_traverse tool.
    #[tool(
        name = "novanet_search",
        description = "🔍 FIND & EXPLORE - Search the knowledge graph. Modes: 'fulltext' (Neo4j fulltext index), 'property' (exact/partial match), 'hybrid' (both), 'walk' (graph traversal from a start node - replaces novanet_traverse), 'triggers' (match by triggers[] array). Filter by: kinds, realm, layer. Walk mode supports: start_key, max_depth, direction (outgoing/incoming/both), arc_families, target_kinds."
    )]
    async fn novanet_search(
        &self,
        params: Parameters<SearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::search::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Introspect the NovaNet schema: query NodeClasses and ArcClasses.
    ///
    /// Enables agents to understand the knowledge graph schema for dynamic
    /// query generation and task decomposition.
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

        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Assemble context for LLM content generation.
    ///
    /// Unified context tool with 4 modes: page, block, knowledge, assemble.
    /// Replaces novanet_generate + novanet_assemble + novanet_atoms.
    #[tool(
        name = "novanet_context",
        description = "⭐ CONTEXT ASSEMBLY - Unified tool for LLM content generation context. 4 modes: 'page' (full page orchestration with all blocks + cross-references), 'block' (single block with entities + knowledge), 'knowledge' (locale atoms: term/expression/pattern/cultureref/taboo/audiencetrait), 'assemble' (low-level assembly with strategy: breadth/depth/relevance/custom). Replaces novanet_generate + novanet_assemble + novanet_atoms. Returns prompt, evidence, denomination_forms (ADR-033), context_anchors."
    )]
    async fn novanet_context(
        &self,
        params: Parameters<ContextParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::context::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Write data to Neo4j with schema validation.
    ///
    /// Operations: upsert_node, create_arc, update_props.
    /// Validates against schema, checks trait permissions, uses MERGE for idempotency.
    /// Use dry_run=true to validate without executing (replaces novanet_check).
    #[tool(
        name = "novanet_write",
        description = "✍️ WRITE DATA - Write to NovaNet with schema validation. Operations: upsert_node, create_arc, update_props. Use dry_run=true to validate without executing (replaces novanet_check). Returns Cypher preview, validation issues, and suggestions when dry_run=true. Auto-creates FOR_LOCALE and HAS_NATIVE arcs for *Native classes."
    )]
    async fn novanet_write(
        &self,
        params: Parameters<WriteParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::write::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Audit the knowledge graph for quality issues.
    ///
    /// Post-write quality audit with CSR (Constraint Satisfaction Rate) metrics.
    /// Checks coverage, orphans, integrity, and freshness.
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

        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Execute multiple NovaNet tools in a single request.
    ///
    /// Supports sequential and parallel execution modes with fail-fast behavior.
    #[tool(
        name = "novanet_batch",
        description = "📦 BULK OPERATIONS - Execute multiple MCP tools in a single request. Set parallel=true for concurrent execution (faster), fail_fast=true to stop on first error. Supports all 8 tools: query, describe, search, introspect, context, write, audit. Each operation has an id for result mapping. Returns aggregated results with timing."
    )]
    async fn novanet_batch(
        &self,
        params: Parameters<BatchParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::batch::execute(&self.state, params.0)
            .await
            .map_err(McpError::from)?;

        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

/// Implement ServerHandler for NovaNetHandler
#[tool_handler]
impl ServerHandler for NovaNetHandler {
    fn get_info(&self) -> ServerInfo {
        // rmcp 0.16: Use struct literals with ..Default::default()
        ServerInfo {
            instructions: Some(
                "NovaNet MCP Server v0.20.0 - Knowledge Graph for AI Agents. \
                 8 tools available (was 14, after The Great Cleanup). TOOL SELECTION: \
                 🔍 novanet_search (find nodes + walk relationships) → \
                 ⭐ novanet_context (content generation context). \
                 For writes: 📚 novanet_introspect (schema) → ✍️ novanet_write (dry_run=true to validate). \
                 📊 novanet_audit for quality. ⚠️ novanet_query is LAST RESORT for custom analytics only. \
                 6 prompts available.".into(),
            ),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
            ..Default::default()
        }
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
            let rendered = prompts::render_prompt(&request.name, &args).ok_or_else(|| {
                McpError::resource_not_found(format!("Prompt not found: {}", request.name), None)
            })?;

            // rmcp 0.16: Use struct literals
            Ok(GetPromptResult {
                description: Some(rendered.description),
                messages: rendered
                    .messages
                    .into_iter()
                    .map(convert_prompt_message)
                    .collect(),
            })
        }
    }
}

/// Convert internal PromptDefinition to MCP Prompt
/// rmcp 0.16: Use struct literals for PromptArgument (no builder pattern)
fn convert_prompt_definition(def: PromptDefinition) -> Prompt {
    Prompt::new(
        def.name,
        Some(def.description),
        Some(
            def.arguments
                .into_iter()
                .map(|arg| PromptArgument {
                    name: arg.name,
                    title: None,
                    description: Some(arg.description),
                    required: Some(arg.required),
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
    use rmcp::model::ErrorCode;

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
