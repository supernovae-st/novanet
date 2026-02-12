//! MCP Server Handler
//!
//! Implements rmcp::ServerHandler for NovaNet MCP tools using macro-based routing.

use crate::server::State;
use crate::tools::{
    AssembleParams, AtomsParams, DescribeParams, QueryParams, SearchParams, TraverseParams,
};
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content, ErrorCode, ServerCapabilities, ServerInfo};
use rmcp::{tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler};
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
}

/// Implement ServerHandler for NovaNetHandler
#[tool_handler]
impl ServerHandler for NovaNetHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "NovaNet MCP Server - Knowledge Graph for AI Agents. \
                 Use novanet_describe to bootstrap your understanding of the schema, \
                 then novanet_query to explore specific data."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
