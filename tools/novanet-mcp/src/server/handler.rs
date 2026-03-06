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
        description = "Execute a read-only Cypher query against the NovaNet knowledge graph. Returns rows as JSON with token estimate. ⚠️ LAST RESORT: Use specialized tools first (novanet_search for finding nodes, novanet_traverse for relationships, novanet_introspect for schema). Only use novanet_query for custom analytics or aggregations."
    )]
    async fn novanet_query(
        &self,
        params: Parameters<QueryParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::query::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Get self-description of the NovaNet knowledge graph.
    ///
    /// Supports describing: schema overview, specific entities, entity categories,
    /// available relations (ArcKinds), locales, and graph statistics.
    #[tool(
        name = "novanet_describe",
        description = "Get self-description of the NovaNet knowledge graph schema, statistics, or specific entities for agent bootstrap."
    )]
    async fn novanet_describe(
        &self,
        params: Parameters<DescribeParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::describe::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Search the NovaNet knowledge graph using fulltext and property search.
    ///
    /// Supports fulltext, property-based, and hybrid search modes.
    /// Filter by node kinds, realm, and layer.
    #[tool(
        name = "novanet_search",
        description = "Search the NovaNet knowledge graph using fulltext or property search. Filter by kinds, realm, layer."
    )]
    async fn novanet_search(
        &self,
        params: Parameters<SearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::search::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Traverse the NovaNet knowledge graph from a starting node.
    ///
    /// Configurable depth, direction, and arc filtering.
    /// Implements RLM-on-KG hop-by-hop pattern.
    #[tool(
        name = "novanet_traverse",
        description = "Traverse the knowledge graph from a starting node with configurable depth, direction, and arc filters."
    )]
    async fn novanet_traverse(
        &self,
        params: Parameters<TraverseParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::traverse::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Assemble context for LLM generation from the knowledge graph.
    ///
    /// Gathers entity definitions, locale knowledge, and structural context
    /// with token budget management and evidence packet compression.
    #[tool(
        name = "novanet_assemble",
        description = "Assemble context for LLM generation with token budget management. Gathers entities, locale knowledge, and structure."
    )]
    async fn novanet_assemble(
        &self,
        params: Parameters<AssembleParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::assemble::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Retrieve knowledge atoms for a specific locale.
    ///
    /// Returns Terms, Expressions, Patterns, CultureRefs, Taboos, and AudienceTraits.
    /// Enables selective LLM context loading.
    #[tool(
        name = "novanet_atoms",
        description = "Retrieve knowledge atoms (Terms, Expressions, Patterns, etc.) for a locale. Enables selective LLM context."
    )]
    async fn novanet_atoms(
        &self,
        params: Parameters<AtomsParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::atoms::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Assemble complete generation context for block or page content.
    ///
    /// Orchestrates traverse, assemble, and atoms tools for AI agents.
    /// Implements full RLM-on-KG pipeline with context anchors.
    #[tool(
        name = "novanet_generate",
        description = "Assemble complete generation context for block or page content. Orchestrates traverse/assemble/atoms with context anchors."
    )]
    async fn novanet_generate(
        &self,
        params: Parameters<GenerateParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::generate::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Introspect the NovaNet schema: query NodeClasses and ArcClasses.
    ///
    /// Enables agents to understand the knowledge graph schema for dynamic
    /// query generation and task decomposition.
    /// MVP 8 Phase 3: 8th MCP tool for schema introspection.
    #[tool(
        name = "novanet_introspect",
        description = "Introspect NovaNet schema: query NodeClasses, ArcClasses, and their relationships. Filter by realm, layer, or arc family. 💡 Use this to discover required properties, trait permissions, and valid arc connections BEFORE calling novanet_write. Example: introspect(target='class', name='EntityNative', include_arcs=true) returns required_properties and valid arcs."
    )]
    async fn novanet_introspect(
        &self,
        params: Parameters<IntrospectParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::introspect::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Execute multiple NovaNet tools in a single request.
    ///
    /// Supports sequential and parallel execution modes with fail-fast behavior.
    /// Task A1: 9th MCP tool for bulk operations.
    #[tool(
        name = "novanet_batch",
        description = "Execute multiple NovaNet tools in a single request. Supports parallel execution and fail-fast behavior."
    )]
    async fn novanet_batch(
        &self,
        params: Parameters<BatchParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::batch::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Get cache statistics including hit rate, entry count, and memory usage.
    ///
    /// Task A3: 10th MCP tool for cache monitoring.
    #[tool(
        name = "novanet_cache_stats",
        description = "Get cache statistics including hit rate, entry count, and memory usage."
    )]
    async fn novanet_cache_stats(
        &self,
        params: Parameters<CacheStatsParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::cache_stats::get_stats(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Invalidate cache entries. Use pattern for selective invalidation or all=true for full clear.
    ///
    /// Task A3: 11th MCP tool for cache management.
    #[tool(
        name = "novanet_cache_invalidate",
        description = "Invalidate cache entries. Use pattern for selective invalidation or all=true for full clear."
    )]
    async fn novanet_cache_invalidate(
        &self,
        params: Parameters<CacheInvalidateParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::cache_stats::invalidate(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

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
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.with_hint()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

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
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Audit the knowledge graph for quality issues.
    ///
    /// Post-write quality audit with CSR (Constraint Satisfaction Rate) metrics.
    /// Checks coverage, orphans, integrity, and freshness.
    /// v0.17.0: 14th MCP tool for quality audit.
    #[tool(
        name = "novanet_audit",
        description = "Audit knowledge graph quality. Checks: coverage (missing natives), orphans (missing arcs), integrity (broken refs), freshness (stale data). Returns CSR metrics and recommendations."
    )]
    async fn novanet_audit(
        &self,
        params: Parameters<AuditParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = crate::tools::auditor::execute(&self.state, params.0)
            .await
            .map_err(|e| McpError {
                code: ErrorCode(-32000),
                message: Cow::Owned(e.to_string()),
                data: None,
            })?;

        let json = serde_json::to_string_pretty(&result).map_err(|e| McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(format!("Serialization error: {}", e)),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

/// Implement ServerHandler for NovaNetHandler
#[tool_handler]
impl ServerHandler for NovaNetHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "NovaNet MCP Server v0.17.0 - Knowledge Graph for AI Agents. \
                 14 tools: novanet_search (find nodes), novanet_traverse (relationships), \
                 novanet_introspect (schema), novanet_generate (context), novanet_atoms (locale). \
                 Writes: ALWAYS call novanet_check BEFORE novanet_write. \
                 novanet_query is LAST RESORT for custom analytics only. 6 prompts available."
                    .into(),
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

            let rendered =
                prompts::render_prompt(&request.name, &args).ok_or_else(|| McpError {
                    code: ErrorCode(-32001),
                    message: Cow::Owned(format!("Prompt not found: {}", request.name)),
                    data: None,
                })?;

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
fn convert_prompt_definition(def: PromptDefinition) -> Prompt {
    Prompt {
        name: def.name,
        title: None,
        description: Some(def.description),
        arguments: Some(
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
        icons: None,
        meta: None,
    }
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
        let error = McpError {
            code: ErrorCode(-32000),
            message: std::borrow::Cow::Borrowed("Tool error"),
            data: None,
        };

        assert_eq!(error.code, ErrorCode(-32000));
    }

    #[test]
    fn test_serialization_error_code_is_minus_32603() {
        // Verify the error code constant used for serialization errors
        let error = McpError {
            code: ErrorCode(-32603),
            message: std::borrow::Cow::Borrowed("Serialization error"),
            data: None,
        };

        assert_eq!(error.code, ErrorCode(-32603));
    }

    #[test]
    fn test_prompt_not_found_error_code_is_minus_32001() {
        // Verify the error code constant used for prompt not found
        let error = McpError {
            code: ErrorCode(-32001),
            message: std::borrow::Cow::Borrowed("Prompt not found"),
            data: None,
        };

        assert_eq!(error.code, ErrorCode(-32001));
    }
}
