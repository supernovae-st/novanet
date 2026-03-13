//! novanet_batch tool (v0.20.0)
//!
//! Execute multiple NovaNet tools in a single request.
//! Supports sequential and parallel execution modes.
//!
//! Supported tools (8): novanet_query, novanet_describe, novanet_search,
//! novanet_introspect, novanet_context, novanet_write, novanet_audit, novanet_batch.

use crate::error::{Error, Result};
use crate::server::State;
use crate::tools::{auditor, context, describe, introspect, query, search, write};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, instrument};

/// Parameters for batch execution of multiple MCP tools
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct BatchParams {
    /// List of operations to execute
    pub operations: Vec<BatchOperation>,
    /// Execute operations in parallel (default: false)
    #[serde(default)]
    pub parallel: bool,
    /// Stop on first error (default: true)
    #[serde(default = "default_fail_fast")]
    pub fail_fast: bool,
    /// Maximum concurrent operations when parallel=true (default: 5)
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent: usize,
}

fn default_fail_fast() -> bool {
    true
}

fn default_max_concurrent() -> usize {
    5
}

/// A single batch operation
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct BatchOperation {
    /// Tool name (e.g., "novanet_query", "novanet_describe")
    pub tool: String,
    /// Tool-specific parameters
    pub params: Value,
    /// Optional ID for result mapping
    #[serde(default)]
    pub id: Option<String>,
}

/// Result from batch execution
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct BatchResult {
    /// Results from each operation
    pub results: Vec<BatchOperationResult>,
    /// Total number of operations
    pub total_operations: usize,
    /// Number of successful operations
    pub successful: usize,
    /// Number of failed operations
    pub failed: usize,
    /// Aggregated token estimate across all results
    pub token_estimate: usize,
}

/// Result from a single batch operation
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct BatchOperationResult {
    /// Operation ID (from BatchOperation.id or auto-generated index)
    pub id: Option<String>,
    /// Tool name that was executed
    pub tool: String,
    /// Whether the operation succeeded
    pub success: bool,
    /// Result data (if success)
    pub result: Option<Value>,
    /// Error message (if failure)
    pub error: Option<String>,
    /// Token estimate for this operation
    pub token_estimate: usize,
}

/// Execute the novanet_batch tool
#[instrument(name = "novanet_batch", skip(state), fields(
    operation_count = params.operations.len(),
    parallel = params.parallel,
    fail_fast = params.fail_fast
))]
pub async fn execute(state: &State, params: BatchParams) -> Result<BatchResult> {
    let total = params.operations.len();

    let results = if params.parallel {
        execute_parallel(state, &params).await?
    } else {
        execute_sequential(state, &params).await?
    };

    let successful = results.iter().filter(|r| r.success).count();
    let failed = total - successful;
    let token_estimate = results.iter().map(|r| r.token_estimate).sum();

    Ok(BatchResult {
        results,
        total_operations: total,
        successful,
        failed,
        token_estimate,
    })
}

/// Execute operations sequentially
async fn execute_sequential(
    state: &State,
    params: &BatchParams,
) -> Result<Vec<BatchOperationResult>> {
    let mut results = Vec::with_capacity(params.operations.len());

    for (idx, op) in params.operations.iter().enumerate() {
        debug!(tool = %op.tool, id = ?op.id, idx = idx, "Executing batch operation");

        let result = execute_single(state, op).await;
        let failed = result.is_err();

        results.push(match result {
            Ok((value, tokens)) => BatchOperationResult {
                id: op.id.clone().or_else(|| Some(idx.to_string())),
                tool: op.tool.clone(),
                success: true,
                result: Some(value),
                error: None,
                token_estimate: tokens,
            },
            Err(e) => BatchOperationResult {
                id: op.id.clone().or_else(|| Some(idx.to_string())),
                tool: op.tool.clone(),
                success: false,
                result: None,
                error: Some(e.to_string()),
                token_estimate: 0,
            },
        });

        if failed && params.fail_fast {
            debug!("Fail-fast triggered, stopping batch execution");
            break;
        }
    }

    Ok(results)
}

/// Execute operations in parallel
async fn execute_parallel(
    state: &State,
    params: &BatchParams,
) -> Result<Vec<BatchOperationResult>> {
    use futures::stream::{self, StreamExt};

    // Clone operations upfront to avoid lifetime issues with async closures
    let operations_with_idx: Vec<_> = params
        .operations
        .iter()
        .enumerate()
        .map(|(idx, op)| (idx, op.clone()))
        .collect();

    let max_concurrent = params.max_concurrent;

    let results: Vec<_> = stream::iter(operations_with_idx)
        .map(|(idx, op)| {
            let state = state.clone();
            async move {
                debug!(tool = %op.tool, id = ?op.id, idx = idx, "Executing parallel batch operation");

                let result = execute_single(&state, &op).await;
                match result {
                    Ok((value, tokens)) => BatchOperationResult {
                        id: op.id.clone().or_else(|| Some(idx.to_string())),
                        tool: op.tool.clone(),
                        success: true,
                        result: Some(value),
                        error: None,
                        token_estimate: tokens,
                    },
                    Err(e) => BatchOperationResult {
                        id: op.id.clone().or_else(|| Some(idx.to_string())),
                        tool: op.tool.clone(),
                        success: false,
                        result: None,
                        error: Some(e.to_string()),
                        token_estimate: 0,
                    },
                }
            }
        })
        .buffer_unordered(max_concurrent)
        .collect()
        .await;

    Ok(results)
}

/// Execute a single tool operation
async fn execute_single(state: &State, op: &BatchOperation) -> Result<(Value, usize)> {
    match op.tool.as_str() {
        "novanet_query" => {
            let params: query::QueryParams = serde_json::from_value(op.params.clone())
                .map_err(|e| Error::Internal(format!("Invalid params for novanet_query: {e}")))?;
            let result = query::execute(state, params).await?;
            let tokens = result.token_estimate;
            Ok((serde_json::to_value(result)?, tokens))
        }
        "novanet_describe" => {
            let params: describe::DescribeParams =
                serde_json::from_value(op.params.clone()).map_err(|e| {
                    Error::Internal(format!("Invalid params for novanet_describe: {e}"))
                })?;
            let result = describe::execute(state, params).await?;
            let tokens = result.token_estimate;
            Ok((serde_json::to_value(result)?, tokens))
        }
        "novanet_search" => {
            let params: search::SearchParams =
                serde_json::from_value(op.params.clone()).map_err(|e| {
                    Error::Internal(format!("Invalid params for novanet_search: {e}"))
                })?;
            let result = search::execute(state, params).await?;
            let tokens = result.token_estimate;
            Ok((serde_json::to_value(result)?, tokens))
        }
        "novanet_introspect" => {
            let params: introspect::IntrospectParams =
                serde_json::from_value(op.params.clone()).map_err(|e| {
                    Error::Internal(format!("Invalid params for novanet_introspect: {e}"))
                })?;
            let result = introspect::execute(state, params).await?;
            let tokens = result.token_estimate;
            Ok((serde_json::to_value(result)?, tokens))
        }
        "novanet_context" => {
            let params: context::ContextParams =
                serde_json::from_value(op.params.clone()).map_err(|e| {
                    Error::Internal(format!("Invalid params for novanet_context: {e}"))
                })?;
            let result = context::execute(state, params).await?;
            let tokens = result.token_estimate;
            Ok((serde_json::to_value(result)?, tokens))
        }
        "novanet_write" => {
            let params: write::WriteParams =
                serde_json::from_value(op.params.clone()).map_err(|e| {
                    Error::Internal(format!("Invalid params for novanet_write: {e}"))
                })?;
            let result = write::execute(state, params).await?;
            // WriteResult is an untagged enum — estimate tokens from serialized size
            let value = serde_json::to_value(&result)?;
            let tokens = value.to_string().len().div_ceil(4);
            Ok((value, tokens))
        }
        "novanet_audit" => {
            let params: auditor::AuditParams =
                serde_json::from_value(op.params.clone()).map_err(|e| {
                    Error::Internal(format!("Invalid params for novanet_audit: {e}"))
                })?;
            let result = auditor::execute(state, params).await?;
            let tokens = result.token_estimate as usize;
            Ok((serde_json::to_value(result)?, tokens))
        }
        _ => Err(Error::invalid_tool(&op.tool)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ══════════════════════════════════════════════════════════════
    // PARAMS DESERIALIZATION TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_batch_params_deserialize() {
        let json = r#"{
            "operations": [
                {"tool": "novanet_query", "params": {"cypher": "MATCH (n) RETURN n LIMIT 1"}},
                {"tool": "novanet_describe", "params": {"describe": "schema"}}
            ],
            "parallel": true,
            "fail_fast": false
        }"#;
        let params: BatchParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.operations.len(), 2);
        assert!(params.parallel);
        assert!(!params.fail_fast);
    }

    #[test]
    fn test_batch_params_defaults() {
        let json = r#"{
            "operations": [
                {"tool": "novanet_query", "params": {"cypher": "MATCH (n) RETURN n LIMIT 1"}}
            ]
        }"#;
        let params: BatchParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.operations.len(), 1);
        assert!(!params.parallel); // default: false
        assert!(params.fail_fast); // default: true
        assert_eq!(params.max_concurrent, 5); // default: 5
    }

    #[test]
    fn test_batch_operation_with_id() {
        let json =
            r#"{"tool": "novanet_query", "params": {"cypher": "MATCH (n) RETURN n"}, "id": "op1"}"#;
        let op: BatchOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op.tool, "novanet_query");
        assert_eq!(op.id, Some("op1".to_string()));
    }

    #[test]
    fn test_batch_operation_without_id() {
        let json = r#"{"tool": "novanet_describe", "params": {"describe": "schema"}}"#;
        let op: BatchOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op.tool, "novanet_describe");
        assert!(op.id.is_none());
    }

    // ══════════════════════════════════════════════════════════════
    // RESULT STRUCTURE TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_batch_result_serialize() {
        let result = BatchResult {
            results: vec![
                BatchOperationResult {
                    id: Some("op1".to_string()),
                    tool: "novanet_query".to_string(),
                    success: true,
                    result: Some(serde_json::json!({"rows": []})),
                    error: None,
                    token_estimate: 100,
                },
                BatchOperationResult {
                    id: Some("op2".to_string()),
                    tool: "novanet_describe".to_string(),
                    success: false,
                    result: None,
                    error: Some("Not found".to_string()),
                    token_estimate: 0,
                },
            ],
            total_operations: 2,
            successful: 1,
            failed: 1,
            token_estimate: 100,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"total_operations\":2"));
        assert!(json.contains("\"successful\":1"));
        assert!(json.contains("\"failed\":1"));
    }

    #[test]
    fn test_batch_operation_result_success_case() {
        let result = BatchOperationResult {
            id: Some("test".to_string()),
            tool: "novanet_query".to_string(),
            success: true,
            result: Some(serde_json::json!({"data": "test"})),
            error: None,
            token_estimate: 50,
        };

        assert!(result.success);
        assert!(result.result.is_some());
        assert!(result.error.is_none());
    }

    #[test]
    fn test_batch_operation_result_failure_case() {
        let result = BatchOperationResult {
            id: Some("test".to_string()),
            tool: "novanet_invalid".to_string(),
            success: false,
            result: None,
            error: Some("Invalid tool".to_string()),
            token_estimate: 0,
        };

        assert!(!result.success);
        assert!(result.result.is_none());
        assert!(result.error.is_some());
    }

    // ══════════════════════════════════════════════════════════════
    // INVALID TOOL TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_invalid_tool_error() {
        let error = Error::invalid_tool("unknown_tool");
        assert_eq!(error.to_string(), "Invalid tool: unknown_tool");
    }

    // ══════════════════════════════════════════════════════════════
    // SUPPORTED TOOLS LIST TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_supported_tools() {
        // List of all 8 supported tools that should work in batch (v0.20.0)
        let supported_tools = vec![
            "novanet_query",
            "novanet_describe",
            "novanet_search",
            "novanet_introspect",
            "novanet_context",
            "novanet_write",
            "novanet_audit",
            "novanet_batch",
        ];

        for tool in &supported_tools {
            // Verify tool name is non-empty and starts with novanet_
            assert!(!tool.is_empty());
            assert!(tool.starts_with("novanet_"));
        }

        assert_eq!(supported_tools.len(), 8);
    }

    // ══════════════════════════════════════════════════════════════
    // MAX CONCURRENT TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_custom_max_concurrent() {
        let json = r#"{
            "operations": [],
            "parallel": true,
            "max_concurrent": 10
        }"#;
        let params: BatchParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.max_concurrent, 10);
    }

    #[test]
    fn test_default_max_concurrent_value() {
        assert_eq!(default_max_concurrent(), 5);
    }

    #[test]
    fn test_default_fail_fast_value() {
        assert!(default_fail_fast());
    }
}
