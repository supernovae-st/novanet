//! MCP Server Handler
//!
//! Implements rmcp::ServerHandler for NovaNet MCP tools using macro-based routing.
//! Phase 3: Adds novanet_generate tool and 6 MCP prompts.

use crate::prompts::{self, PromptDefinition, PromptMessage as InternalPromptMessage};
use crate::server::State;
use crate::tools::{
    AssembleParams, AtomsParams, DescribeParams, GenerateParams, QueryParams, SearchParams,
    TraverseParams,
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
        description = "Execute a read-only Cypher query against the NovaNet knowledge graph. Returns rows as JSON with token estimate."
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
}

/// Implement ServerHandler for NovaNetHandler
#[tool_handler]
impl ServerHandler for NovaNetHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "NovaNet MCP Server v0.3.0 - Knowledge Graph for AI Agents. \
                 Use novanet_describe to bootstrap your understanding of the schema, \
                 novanet_query to explore specific data, and novanet_generate for \
                 content generation context assembly. 6 prompts available for \
                 guided workflows."
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
